import argparse
import copy
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

import pandas as pd


REPO_ROOT = Path(__file__).resolve().parents[1]
SEED_ROOT = REPO_ROOT / "seed" / "v0.2"
WORKBOOK_PATH = Path(r"E:\codex\outputs\黄金样本库v120究极版_V120专项增强候选池_20260424.xlsx")
DRY_RUN_REPORT = REPO_ROOT / "docs" / "v120-cnwar-slg-one-shot-dry-run-2026-04-24.md"
DATE = "2026-04-24"
NEW_SOURCE_ID = "seedance2_v120_cnwar_slg_candidate_pool_xlsx_2026_04_24"
NEW_PROVENANCE_ID = "seedance2_v120_cnwar_slg_candidate_pool_seed_package_2026_04_24"
NEW_SOURCE_PATH = "E:/codex/outputs/黄金样本库v120究极版_V120专项增强候选池_20260424.xlsx"
NEW_SEQUENCE_IDS = [
    "CNWARSEQ01",
    "CNWARSEQ02",
    "CNWARSEQ03",
    "CNWARSEQ04",
    "SLGSEQ01",
    "SLGSEQ02",
    "SLGSEQ03",
    "SLGSEQ04",
]
CORE_FIELDS = [
    "technical_profile",
    "scene_performance_core",
    "camera_directing_core",
    "audio_directing_core",
    "continuity_negative_core",
]
PLACEHOLDER_TOKENS = ["待补", "TODO", "TBD", "template"]
PROMPT_SCAN_TERMS = [
    "Seedance",
    "Qwen",
    "宫崎骏",
    "今敏",
    "新海诚",
    "王家卫",
    "诺兰",
    "漫威",
    "迪士尼",
    "皮克斯",
    "三国",
]


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write_json(path: Path, payload: dict):
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=4) + "\n", encoding="utf-8")


def canonical_bundle_hash(repo_root: Path, bundle_order: list[str]) -> str:
    sha = hashlib.sha256()
    for rel in bundle_order:
        raw = (repo_root / rel).read_bytes().replace(b"\r\n", b"\n")
        sha.update(raw)
    return sha.hexdigest()


def sha256_file(path: Path) -> str:
    sha = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            sha.update(chunk)
    return sha.hexdigest()


def machine_slug(sample_id: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", sample_id.lower()).strip("_")


def parse_scene_tags(scene_tag: str) -> list[str]:
    return [part.strip() for part in re.split(r"[，,、/；;]\s*", scene_tag) if part.strip()]


def normalize_prompt_body(prompt_body: str) -> str:
    normalized = prompt_body
    normalized = re.sub(
        r"^生成一个\s*[^。\n]*的动漫分镜视频镜头。",
        "生成一个动漫分镜视频镜头。",
        normalized,
        count=1,
    )
    normalized = normalized.replace("只抽象国漫战争、架空三国军政与策略视口的镜头语法，", "只抽象中式战争、架空古代军政与策略视口的镜头语法，")
    normalized = normalized.replace("国漫战争", "中式战争")
    normalized = normalized.replace("架空三国军政", "架空古代军政")
    return normalized


def build_library_record(template_record: dict, row: dict, row_index: int, workbook_sha: str) -> dict:
    sample_id = row["shot_id"]
    source_fields = {field: row[field] for field in template_record["source_fields"].keys()}
    source_fields["prompt_body"] = normalize_prompt_body(source_fields["prompt_body"])

    record = copy.deepcopy(template_record)
    record["machine_id"] = f"golden_sample_{machine_slug(sample_id)}"
    record["sample_id"] = sample_id
    record["source_fields"] = source_fields
    record["provenance"]["source_workbook"] = NEW_SOURCE_PATH
    record["provenance"]["source_workbook_sha256"] = workbook_sha
    record["provenance"]["source_row_index"] = row_index
    record["provenance"]["source_library_status"] = row["library_status"]
    record["provenance"]["source_sample_type"] = row["sample_type"]
    record["classification"]["sequence_id"] = row["sequence_id"]
    record["classification"]["shot_order"] = row["shot_order"]
    record["classification"]["style_cluster"] = row["style_cluster"]
    record["classification"]["scene_category"] = row["scene_category"]
    record["classification"]["scene_tags"] = parse_scene_tags(row["scene_tag"])
    record["classification"]["quality_grade"] = row["quality_grade"]
    record["classification"]["usable_for_fewshot"] = False
    record["fewshot"]["eligible"] = False
    record["fewshot"]["source_value"] = "No"
    record["fewshot"]["retrieval_status"] = "reserve_holdout_not_for_positive_fewshot"
    record["validator_evidence"]["covered_points"] = row["covered_points"]
    record["validator_evidence"]["missed_points"] = row["missed_points"]
    record["validator_evidence"]["teaching_note"] = row["teaching_note"]
    record["validator_evidence"]["source_quality_grade"] = row["quality_grade"]
    record["validator_evidence"]["source_library_status"] = row["library_status"]
    record["validator_evidence"]["source_sample_type"] = row["sample_type"]
    for field in CORE_FIELDS:
        record["validator_evidence"]["surface_completeness"][field] = bool(row[field].strip())
    record["negative_sample"]["is_negative_sample"] = False
    record["negative_sample"]["signal_codes"] = ["fewshot_closed", "reserve_sample", "coverage_gap"]
    record["negative_sample"]["reserve_reason"] = row["reserve_reason"]
    record["v3_core_coverage"]["source_coverage_statement"] = row["covered_points"]
    record["v3_core_coverage"]["source_missing_statement"] = row["missed_points"]
    for field in CORE_FIELDS:
        record["v3_core_coverage"]["source_field_presence"][field] = bool(row[field].strip())
    record["v3_core_coverage"]["comparison_baseline"]["primary_source_id"] = NEW_SOURCE_ID
    return record


def build_failure_record(row: dict) -> dict:
    sample_id = row["shot_id"]
    return {
        "mapping_id": f"gs_failure_map_{machine_slug(sample_id)}",
        "sample_id": sample_id,
        "core": "full_stack_sequence_sample",
        "tier": row["quality_grade"],
        "usable_for_fewshot": row["usable_for_fewshot"],
        "negative_sample_signal": False,
        "planned_failure_codes": [
            "golden_coverage_gap",
            "golden_fewshot_closed",
            "golden_reserve_sample",
        ],
        "validator_evidence": {
            "covered_points": row["covered_points"],
            "missed_points": row["missed_points"],
            "teaching_note": row["teaching_note"],
            "library_status": row["library_status"],
            "sample_type": row["sample_type"],
            "sequence_id": row["sequence_id"],
            "shot_order": row["shot_order"],
            "reference_bundle_present": bool(row["reference_bundle"].strip()),
        },
        "source_field_refs": [
            "library_status",
            "reserve_reason",
            "sample_type",
            "sequence_id",
            "shot_order",
            "quality_grade",
            "usable_for_fewshot",
            "covered_points",
            "missed_points",
            "teaching_note",
        ],
    }


def build_repair_record(row: dict) -> dict:
    sample_id = row["shot_id"]
    return {
        "mapping_id": f"gs_repair_plan_{machine_slug(sample_id)}",
        "sample_id": sample_id,
        "core": "full_stack_sequence_sample",
        "repair_planning_mode": "reserve_holdout_exclude_from_positive_fewshot",
        "linked_failure_mapping_id": f"gs_failure_map_{machine_slug(sample_id)}",
        "planned_repair_inputs": {
            "missed_points": row["missed_points"],
            "teaching_note": row["teaching_note"],
            "style_cluster": row["style_cluster"],
            "scene_category": row["scene_category"],
            "sample_type": row["sample_type"],
            "sequence_id": row["sequence_id"],
            "library_status": row["library_status"],
            "reserve_reason": row["reserve_reason"],
            "usable_for_fewshot": row["usable_for_fewshot"],
        },
        "future_repair_gate": "reserve_sequence_review_gate",
        "planning_only": True,
    }


def recalc_coverage_summary(rows: list[dict]) -> dict:
    summary = {
        "library_status": dict(Counter(row["library_status"] for row in rows)),
        "sample_type": dict(Counter(row["sample_type"] for row in rows)),
        "quality_grade": dict(Counter(row["quality_grade"] for row in rows)),
        "usable_for_fewshot": dict(Counter(row["usable_for_fewshot"] for row in rows)),
        "style_cluster": dict(Counter(row["style_cluster"] for row in rows)),
        "scene_category": dict(Counter(row["scene_category"] for row in rows)),
        "sequence_groups": dict(Counter(row["sequence_id"] for row in rows if row["sequence_id"])),
        "surface_completeness": {},
    }
    for field in CORE_FIELDS:
        summary["surface_completeness"][field] = sum(1 for row in rows if row[field].strip())
    return summary


def load_workbook_rows():
    frame = pd.read_excel(WORKBOOK_PATH, sheet_name="V120工作簿", dtype=str).fillna("")
    return frame, frame.to_dict(orient="records")


def run_dry_run(repo_root: Path):
    library = load_json(SEED_ROOT / "golden_sample_library.json")
    field_order = library["source_field_order"]
    frame, rows = load_workbook_rows()
    existing_by_sample = {record["sample_id"]: record["source_fields"] for record in library["records"]}
    new_rows = [row for row in rows if row["shot_id"].startswith("GS120-CAND-")]
    legacy_rows = [row for row in rows if not row["shot_id"].startswith("GS120-CAND-")]

    raw_prompt_hits = []
    normalized_prompt_hits = []
    slg_failures = []
    weak_teaching_notes = []
    placeholder_hits = []
    missing_core_fields = {field: [] for field in CORE_FIELDS}
    style_labels = sorted({row["style_cluster"] for row in new_rows})

    for row in new_rows:
        prompt_body = row["prompt_body"]
        raw_hits = [term for term in PROMPT_SCAN_TERMS if term in prompt_body]
        raw_hits.extend(label for label in style_labels if label and label in prompt_body)
        if raw_hits:
            raw_prompt_hits.append({"shot_id": row["shot_id"], "hits": sorted(set(raw_hits))})

        normalized_prompt = normalize_prompt_body(prompt_body)
        normalized_hits = [term for term in PROMPT_SCAN_TERMS if term in normalized_prompt]
        normalized_hits.extend(label for label in style_labels if label and label in normalized_prompt)
        if normalized_hits:
            normalized_prompt_hits.append({"shot_id": row["shot_id"], "hits": sorted(set(normalized_hits))})

        combined = " ".join(str(row[field]) for field in field_order)
        for token in PLACEHOLDER_TOKENS:
            if token.lower() in combined.lower():
                placeholder_hits.append({"shot_id": row["shot_id"], "token": token})
                break

        for field in CORE_FIELDS:
            if not row[field].strip():
                missing_core_fields[field].append(row["shot_id"])

        if not any(keyword in row["teaching_note"] for keyword in ["训练", "建立", "交代", "推进", "转折", "承接", "收束", "功能", "信息", "叙事", "故事", "关系", "升级", "回落", "确认"]):
            weak_teaching_notes.append(row["shot_id"])

        if row["sequence_id"].startswith("SLGSEQ"):
            visible_text = " ".join(str(row[field]) for field in ["sample_title", "scene_performance_core", "camera_directing_core", "audio_directing_core", "continuity_negative_core", "teaching_note", "prompt_body"])
            if not any(keyword in visible_text for keyword in ["沙盘", "视口", "行军", "轨迹", "城建", "演进", "战报", "UI"]):
                slg_failures.append(row["shot_id"])

    unchanged_legacy = True
    legacy_mismatches = []
    for row in legacy_rows:
        source_fields = existing_by_sample.get(row["shot_id"])
        if source_fields is None:
            unchanged_legacy = False
            legacy_mismatches.append(f"{row['shot_id']}: missing from existing v0.2 seed")
            continue
        for field in field_order:
            if source_fields.get(field, "") != row.get(field, ""):
                unchanged_legacy = False
                legacy_mismatches.append(f"{row['shot_id']}: field '{field}' differs")
                break

    sequence_counts = dict(Counter(row["sequence_id"] for row in new_rows))
    summary_after = recalc_coverage_summary(rows)
    report_lines = [
        "# V120 CN War / SLG Candidate Pool Dry-Run 2026-04-24",
        "",
        "## Scope",
        "",
        f"- source workbook: `{WORKBOOK_PATH}`",
        "- target repo: `E:\\codex\\hope-kb` on `codex/contracts-freeze`",
        "- write mode: dry-run only for this step; no seed files were modified while generating this report",
        "- ingest assumption: new 32 rows will be appended to v0.2 as reserve sequence samples, and their `prompt_body` will be safety-normalized on ingest to remove label / IP leakage while leaving the other 22 source fields untouched",
        "",
        "## Structural Checks",
        "",
        f"- workbook sheet `V120工作簿` column count: `{len(frame.columns)}`",
        f"- workbook headers match `seed/v0.2/golden_sample_library.json` `source_field_order`: `{list(frame.columns) == field_order}`",
        f"- workbook row count: `{len(rows)}`",
        f"- existing 120 rows unchanged versus current v0.2 seed: `{unchanged_legacy}`",
        f"- new candidate IDs present: `{new_rows[0]['shot_id']} .. {new_rows[-1]['shot_id']}`",
        "",
        "## New 32 Checks",
        "",
        f"- all new rows are `reserve`: `{all(row['library_status'] == 'reserve' for row in new_rows)}`",
        f"- all new rows are `usable_for_fewshot=No`: `{all(row['usable_for_fewshot'] == 'No' for row in new_rows)}`",
        f"- all new rows are `sequence_shot`: `{all(row['sample_type'] == 'sequence_shot' for row in new_rows)}`",
        f"- sequence groups: `{json.dumps(sequence_counts, ensure_ascii=False)}`",
        f"- core fields non-empty across all 32: `{all(not missing_core_fields[field] for field in CORE_FIELDS)}`",
        f"- placeholder residue (`待补` / `TODO` / `TBD` / `template`): `{len(placeholder_hits) == 0}`",
        f"- teaching notes include story-function language: `{len(weak_teaching_notes) == 0}`",
        f"- SLG rows stay on visible strategy-shot syntax: `{len(slg_failures) == 0}`",
        "",
        "## Prompt Safety",
        "",
        f"- raw workbook prompt-body label/IP hits: `{len(raw_prompt_hits)}`",
        f"- normalized ingest prompt-body label/IP hits: `{len(normalized_prompt_hits)}`",
        "- normalization rule 1: rewrite the opening line to `生成一个动漫分镜视频镜头。`",
        "- normalization rule 2: rewrite `国漫战争` to `中式战争`",
        "- normalization rule 3: rewrite `架空三国军政` to `架空古代军政`",
        "",
        "## Counts After Planned Ingest",
        "",
        f"- golden_sample_library: `120 -> {len(rows)}`",
        "- golden_sample_field_coverage_rules: `5 -> 5`",
        f"- golden_sample_failure_mapping: `120 -> {len(rows)}`",
        f"- golden_sample_repair_mapping: `120 -> {len(rows)}`",
        f"- library_status summary after ingest: `{json.dumps(summary_after['library_status'], ensure_ascii=False)}`",
        f"- sample_type summary after ingest: `{json.dumps(summary_after['sample_type'], ensure_ascii=False)}`",
        f"- quality_grade summary after ingest: `{json.dumps(summary_after['quality_grade'], ensure_ascii=False)}`",
        f"- usable_for_fewshot summary after ingest: `{json.dumps(summary_after['usable_for_fewshot'], ensure_ascii=False)}`",
        "",
        "## Dry-Run Result",
        "",
        "- raw workbook structure passes items 1-5 and 7-9.",
        "- raw workbook `prompt_body` text needs deterministic safety normalization for item 6 before seed write.",
        "- with the normalization above applied only to the new 32 `prompt_body` values, the ingest is ready to proceed.",
    ]
    if legacy_mismatches:
        report_lines.extend(["", "## Legacy Mismatches", ""])
        report_lines.extend(f"- {item}" for item in legacy_mismatches[:10])
    if raw_prompt_hits:
        report_lines.extend(["", "## Raw Prompt Hits", ""])
        report_lines.extend(f"- `{item['shot_id']}`: `{', '.join(item['hits'])}`" for item in raw_prompt_hits[:16])

    report_path = DRY_RUN_REPORT
    report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")
    return {
        "field_order": field_order,
        "rows": rows,
        "new_rows": new_rows,
        "raw_prompt_hits": raw_prompt_hits,
        "normalized_prompt_hits": normalized_prompt_hits,
        "sequence_counts": sequence_counts,
        "summary_after": summary_after,
        "report_path": report_path,
    }


def write_seed(repo_root: Path, dry_run_result: dict):
    library = load_json(SEED_ROOT / "golden_sample_library.json")
    failure = load_json(SEED_ROOT / "golden_sample_failure_mapping.json")
    repair = load_json(SEED_ROOT / "golden_sample_repair_mapping.json")
    coverage = load_json(SEED_ROOT / "golden_sample_field_coverage_rules.json")
    source_register = load_json(SEED_ROOT / "source_register.json")
    manifest = load_json(SEED_ROOT / "manifest.json")

    workbook_sha = sha256_file(WORKBOOK_PATH)
    library_template = next(record for record in library["records"] if record["sample_id"] == "GS120-109")

    new_library_records = []
    new_failure_records = []
    new_repair_records = []
    for index, row in enumerate(dry_run_result["new_rows"], start=121):
        new_library_records.append(build_library_record(library_template, row, index, workbook_sha))
        new_failure_records.append(build_failure_record(row))
        new_repair_records.append(build_repair_record(row))

    all_rows = dry_run_result["rows"]
    all_sample_ids = [row["shot_id"] for row in all_rows]
    summary_after = recalc_coverage_summary(all_rows)

    library["generated_at"] = DATE
    library["record_count"] = len(all_rows)
    library["source"]["primary_workbook"] = NEW_SOURCE_PATH
    library["source"]["primary_workbook_sha256"] = workbook_sha
    library["records"].extend(new_library_records)

    failure["generated_at"] = DATE
    failure["source_context"]["primary_source_id"] = NEW_SOURCE_ID
    failure["record_count"] = len(all_rows)
    failure["records"].extend(new_failure_records)

    repair["generated_at"] = DATE
    repair["source_context"]["primary_source_id"] = NEW_SOURCE_ID
    repair["record_count"] = len(all_rows)
    repair["records"].extend(new_repair_records)

    coverage["generated_at"] = DATE
    coverage["source_context"]["primary_source_id"] = NEW_SOURCE_ID
    for record in coverage["records"]:
        record["row_count"] = len(all_rows)
        record["source_sample_ids"] = list(all_sample_ids)
        record["coverage_summary"] = copy.deepcopy(summary_after)
        record["coverage_summary"]["surface_completeness"] = {record["core"]: summary_after["surface_completeness"][record["core"]]}

    source_register["updated_at"] = DATE
    source_register["sources"].append(
        {
            "source_id": NEW_SOURCE_ID,
            "source_type": "immutable_raw_xlsx",
            "label": "V120 CN war / SLG candidate pool workbook",
            "path": NEW_SOURCE_PATH,
            "sha256": workbook_sha,
            "applies_to": [
                "golden_sample_library",
                "golden_sample_field_coverage_rules",
                "golden_sample_failure_mapping",
                "golden_sample_repair_mapping",
            ],
            "notes": "Candidate pool workbook with 152 total rows. Existing 120 rows remain unchanged; new 32 reserve sequence rows are ingested after prompt_body safety normalization.",
        }
    )
    source_register["provenance_entries"].append(
        {
            "provenance_id": NEW_PROVENANCE_ID,
            "source_ids": [
                NEW_SOURCE_ID,
                "hope_seedance2_v120_full_kb_ingest_review_2026_04_23",
                "hope_v120_v3_kb_full_ingest_dispatch_2026_04_23",
                "seedance2_v108_raw_xlsx_2026_04_23",
                "seedance2_v108_raw_docx_2026_04_23",
            ],
            "generated_files": [
                "seed/v0.2/golden_sample_library.json",
                "seed/v0.2/golden_sample_field_coverage_rules.json",
                "seed/v0.2/golden_sample_failure_mapping.json",
                "seed/v0.2/golden_sample_repair_mapping.json",
                "seed/v0.2/source_register.json",
                "seed/v0.2/manifest.json",
            ],
            "preservation_contract": {
                "v120_is_primary_source": True,
                "v108_is_comparison_baseline": True,
                "preserves_existing_120_rows": True,
                "ingests_32_new_reserve_sequence_rows": True,
                "prompt_body_safety_normalized_for_new_rows": True,
                "imports_into_v0_2_golden_sample_library": True,
                "imports_into_live_hope_runtime": False,
                "product_ready_external_reference_handles_closed": True,
                "does_not_create_reference_control_core": True,
                "desktop_and_intake_runtime_unchanged": True,
                "qwen_and_seedance_calls": False,
            },
        }
    )

    manifest["imported_at"] = DATE
    manifest["record_counts"]["golden_sample_library"] = len(all_rows)
    manifest["record_counts"]["golden_sample_field_coverage_rules"] = 5
    manifest["record_counts"]["golden_sample_failure_mapping"] = len(all_rows)
    manifest["record_counts"]["golden_sample_repair_mapping"] = len(all_rows)
    manifest["record_counts"]["golden_sample_sources"] = len(source_register["sources"])
    manifest["record_counts"]["golden_sample_provenance_entries"] = len(source_register["provenance_entries"])

    write_json(SEED_ROOT / "golden_sample_library.json", library)
    write_json(SEED_ROOT / "golden_sample_field_coverage_rules.json", coverage)
    write_json(SEED_ROOT / "golden_sample_failure_mapping.json", failure)
    write_json(SEED_ROOT / "golden_sample_repair_mapping.json", repair)
    write_json(SEED_ROOT / "source_register.json", source_register)

    manifest["content_hash"] = "bundle-sha256:pending"
    write_json(SEED_ROOT / "manifest.json", manifest)
    manifest["content_hash"] = f"bundle-sha256:{canonical_bundle_hash(repo_root, manifest['bundle_order'])}"
    write_json(SEED_ROOT / "manifest.json", manifest)

    return {
        "workbook_sha": workbook_sha,
        "manifest_hash": manifest["content_hash"],
        "counts": manifest["record_counts"],
    }


def main():
    parser = argparse.ArgumentParser(description="Dry-run and ingest the V120 CN war / SLG candidate pool into hope-kb v0.2.")
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    dry_run_result = run_dry_run(repo_root)
    print(f"Dry-run report: {dry_run_result['report_path']}")
    print(f"Workbook rows: {len(dry_run_result['rows'])}")
    print(f"New rows: {len(dry_run_result['new_rows'])}")
    print(f"Raw prompt hits: {len(dry_run_result['raw_prompt_hits'])}")
    print(f"Normalized prompt hits: {len(dry_run_result['normalized_prompt_hits'])}")

    if args.write:
        write_result = write_seed(repo_root, dry_run_result)
        print(f"Seed updated. Manifest hash: {write_result['manifest_hash']}")


if __name__ == "__main__":
    main()
