import argparse
import json
import sqlite3
from pathlib import Path


def load_json(path: Path):
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def resolve_records(repo_root: Path, entry: dict):
    payload = load_json(repo_root / Path(entry["file"]))
    json_path = entry["json_path"]
    if json_path == "$":
        return payload
    return payload[json_path]


def normalize_value(value):
    if isinstance(value, (dict, list)):
        return json.dumps(value, ensure_ascii=False, separators=(",", ":"))
    if isinstance(value, bool):
        return int(value)
    return value


def insert_record(conn: sqlite3.Connection, table: str, record: dict):
    columns = list(record.keys())
    quoted_columns = ", ".join(f'"{column}"' for column in columns)
    placeholders = ", ".join("?" for _ in columns)
    sql = f'INSERT INTO "{table}" ({quoted_columns}) VALUES ({placeholders})'
    values = [normalize_value(record[column]) for column in columns]
    conn.execute(sql, values)


def validate_import_counts(conn: sqlite3.Connection, manifest: dict, import_map: list):
    for entry in import_map:
        table = entry["table"]
        expected = int(manifest["record_counts"][entry["record_count_key"]])
        actual = conn.execute(f'SELECT COUNT(*) FROM "{table}"').fetchone()[0]
        if actual != expected:
            raise RuntimeError(
                f"Count mismatch for table '{table}': expected {expected}, actual {actual}"
            )


def configure_connection(conn: sqlite3.Connection):
    conn.execute("PRAGMA temp_store = MEMORY")
    conn.execute("PRAGMA journal_mode = MEMORY").fetchone()
    conn.execute("PRAGMA synchronous = NORMAL")
    conn.execute("PRAGMA foreign_keys = ON")


def reserve_output_path(output_path: Path) -> Path:
    if not output_path.exists():
        return output_path

    try:
        output_path.unlink()
        return output_path
    except PermissionError:
        pass

    for idx in range(1, 100):
        fallback = output_path.with_name(f"{output_path.stem}.rebuilt-{idx}{output_path.suffix}")
        if not fallback.exists():
            print(f"Warning: cannot overwrite {output_path}; writing snapshot to {fallback} instead.")
            return fallback
        try:
            fallback.unlink()
            print(f"Warning: cannot overwrite {output_path}; reusing fallback path {fallback}.")
            return fallback
        except PermissionError:
            continue

    raise RuntimeError(f"Unable to reserve snapshot output path near {output_path}")


def main():
    parser = argparse.ArgumentParser(description="Build hope-kb SQLite snapshot from seed bundle.")
    parser.add_argument(
        "--repo-root",
        default=str(Path(__file__).resolve().parents[1]),
        help="Repository root for hope-kb",
    )
    parser.add_argument(
        "--output",
        default="snapshots/hope-kb-v0.1.sqlite3",
        help="Relative output path under repo root",
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    output_path = (repo_root / args.output).resolve()
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path = reserve_output_path(output_path)

    manifest_path = repo_root / "seed" / "v0.1" / "manifest.json"
    import_map_path = repo_root / "seed" / "v0.1" / "import_map.json"
    migration_path = repo_root / "migrations" / "0001_init_kb.sql"

    manifest = load_json(manifest_path)
    import_map = load_json(import_map_path)
    migration_sql = migration_path.read_text(encoding="utf-8")

    conn = sqlite3.connect(output_path)
    try:
        # Some Windows environments reject file-backed rollback journals on this drive.
        # Build with in-memory journaling so snapshot creation stays deterministic.
        configure_connection(conn)
        conn.executescript(migration_sql)

        conn.execute(
            """
            INSERT INTO snapshot_meta
            (snapshot_version, snapshot_name, content_hash, hash_algo, seed_import_format, imported_at, note)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
            (
                manifest["snapshot_version"],
                manifest["snapshot_name"],
                manifest["content_hash"],
                manifest["content_hash_algo"],
                manifest["seed_import_format"],
                manifest["imported_at"],
                "built-by build-kb-snapshot.py",
            ),
        )

        for entry in import_map:
            records = resolve_records(repo_root, entry)
            for record in records:
                insert_record(conn, entry["table"], record)

        validate_import_counts(conn, manifest, import_map)

        fk_errors = conn.execute("PRAGMA foreign_key_check").fetchall()
        if fk_errors:
            raise RuntimeError(f"Foreign key check failed: {fk_errors}")

        quick_check = conn.execute("PRAGMA quick_check").fetchone()[0]
        if quick_check != "ok":
            raise RuntimeError(f"SQLite quick_check failed: {quick_check}")

        conn.commit()
    finally:
        conn.close()

    print(f"Built snapshot: {output_path}")
    print(f"Snapshot version: {manifest['snapshot_version']}")
    print(f"Director cut samples: {manifest['record_counts']['director_cut_samples']}")


if __name__ == "__main__":
    main()
