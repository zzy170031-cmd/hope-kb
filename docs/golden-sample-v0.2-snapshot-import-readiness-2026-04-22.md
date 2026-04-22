# Golden Sample v0.2 Snapshot Import Readiness 2026-04-22

This note records the KB-only snapshot-import readiness gate for the v0.2
`golden_sample_library` package.

## Scope

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- source checkpoint before this package: `445c452`
- package type: KB-only snapshot/import readiness

This package does not modify `E:\codex\hope`, desktop, intake, Qwen, Seedance,
or the v0.1 seed/snapshot contract.

## Added Snapshot Inputs

- `seed/v0.2/import_map.json`
- `seed/v0.2/manifest.json`
- `migrations/0002_golden_sample_v0_2.sql`
- versioned support in `scripts/build-kb-snapshot.py`

The builder default remains v0.1. v0.2 must be selected explicitly with
`--version v0.2`.

## v0.2 Import Map

The v0.2 import map contains 6 entries:

- `golden_sample_library`
- `golden_sample_field_coverage_rule`
- `golden_sample_failure_mapping`
- `golden_sample_repair_mapping`
- `golden_sample_source`
- `golden_sample_provenance`

## v0.2 Manifest

- snapshot version: `v0.2`
- snapshot name: `hope-kb-golden-sample-library-v0.2`
- content hash:
  `bundle-sha256:b62c565b7c048bb5e0d2623b438eb83e269471283e2b10fcdd7646ef06c83bf2`
- record counts:
  - `golden_sample_library = 40`
  - `golden_sample_field_coverage_rules = 5`
  - `golden_sample_failure_mapping = 40`
  - `golden_sample_repair_mapping = 40`
  - `golden_sample_sources = 3`
  - `golden_sample_provenance_entries = 1`

## Migration Tables

Migration file: `migrations/0002_golden_sample_v0_2.sql`

Tables:

- `snapshot_meta`
- `golden_sample_library`
- `golden_sample_field_coverage_rule`
- `golden_sample_failure_mapping`
- `golden_sample_repair_mapping`
- `golden_sample_source`
- `golden_sample_provenance`

Nested JSON values are stored as JSON text, matching the existing snapshot
builder pattern.

## Builder Commands

v0.1 no-regression validation:

```powershell
powershell -ExecutionPolicy Bypass -File E:\codex\hope-kb\scripts\validate-seed-bundle.ps1
```

v0.1 default snapshot build:

```powershell
python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb
```

v0.2 explicit snapshot build:

```powershell
python E:\codex\hope-kb\scripts\build-kb-snapshot.py --repo-root E:\codex\hope-kb --version v0.2
```

## Results

- v0.1 no-regression validation: passed
- v0.1 snapshot build: passed; fallback output was
  `snapshots/hope-kb-v0.1.rebuilt-14.sqlite3` because the standard v0.1 sqlite
  path could not be overwritten in this Windows environment
- v0.2 snapshot build: passed; builder output was
  `snapshots/hope-kb-v0.2.rebuilt-2.sqlite3`, then the verified DB was copied
  to the standard ignored path `snapshots/hope-kb-v0.2.sqlite3`
- v0.2 SQLite `quick_check`: `ok`
- v0.2 snapshot hash:
  `bundle-sha256:b62c565b7c048bb5e0d2623b438eb83e269471283e2b10fcdd7646ef06c83bf2`

v0.2 row counts in the generated snapshot:

- `golden_sample_library = 40`
- `golden_sample_field_coverage_rule = 5`
- `golden_sample_failure_mapping = 40`
- `golden_sample_repair_mapping = 40`
- `golden_sample_source = 3`
- `golden_sample_provenance = 1`

## Remaining Gaps

- `reference_control_core` remains absent and must not be invented from current
  golden sample rows
- Hope validator implementation remains closed
- Hope repair implementation remains closed
- exporter/debug metadata remains closed
- desktop read-only provenance remains closed
- intake / Qwen retrieval boundary remains closed
- Qwen and Seedance integration remain closed
