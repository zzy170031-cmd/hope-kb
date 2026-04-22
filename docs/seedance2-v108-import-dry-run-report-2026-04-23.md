# Seedance2 V108 Import Dry-Run Report 2026-04-23

## Route

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- batch_id: `seedance2-v108-import-dry-run-2026-04-23`
- source commit: `6e5a150c39e3e1c7abba1da08fb1d13bf8e199ee`
- Hope dispatch commit: `9823a6ff23ed6c616466a7156ec8c1d52a9d034e`
- dispatch document:
  `E:\codex\hope\docs\seedance2-v108-import-dry-run-dispatch-2026-04-23.md`
- report type: KB-only import dry-run report

This dry-run is documentation only. It does not import V108 rows, does not
modify product structures, does not change KB seed rows, does not promote
positive few-shot rows, and does not create `reference_control_core`.

## Source Artifacts

| artifact | path | sha256 | source register check |
| --- | --- | --- | --- |
| XLSX primary table | `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版.xlsx` | `2c7f7a0b37f5d7a6f90f18c0080a78fa568fcb4528b1b4e6fbdbfb959cfe9e34` | pass |
| DOCX explanation | `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版说明.docx` | `a2335102154ab88cb5c01517b2b20bbb15efdbdd25f0d32eeb9715d42e8d7ed3` | pass |
| normalized staging | `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json` | `a9ace9098ae60ce942ee5101b39329dc5b9ee70ab2d73e20d058d6db5938c167` | pass |

source_hash_check: `pass`

## Header Check

- primary sheet: `V108融合字段库`
- header row: `1`
- field count: `23`
- header_check: `pass`

Accepted canonical headers:

```text
shot_id
library_status
reserve_reason
sample_type
sequence_id
shot_order
sample_title
style_cluster
scene_category
scene_tag
quality_grade
usable_for_fewshot
technical_profile
scene_performance_core
camera_directing_core
audio_directing_core
continuity_negative_core
reference_bundle
ip_abstraction_note
covered_points
missed_points
teaching_note
prompt_body
```

## Row Count Check

- row_count_check: `pass`
- total data rows: `115`
- official rows: `108`
- reserve rows: `7`
- official_reserve_count_check: `pass`

Sample type distribution:

- `single_shot = 79`
- `sequence_shot = 36`
- official `single_shot = 72`
- official `sequence_shot = 36`
- reserve `single_shot = 7`

## Placeholder Summary

- rows with placeholder-like `待补` text: `102`
- official rows with placeholder-like text: `95`
- reserve rows with placeholder-like text: `7`

Placeholder cells by field:

- `reference_bundle = 102`
- `prompt_body = 102`
- `technical_profile = 41`
- `scene_performance_core = 41`
- `camera_directing_core = 42`
- `audio_directing_core = 67`
- `reserve_reason = 1`

All placeholder cells remain gaps. No placeholder content is guessed, filled, or
promoted.

## Blank Conditional Surface Summary

- blank `reserve_reason` rows: `108`
- blank `sequence_id` rows: `79`
- blank `shot_order` rows: `79`

These are conditional / future-fill surfaces. They are preserved as source
shape, not counted as separate records, and not promoted into current product
import, positive few-shot, or validator evidence.

## Reference Handle Summary

- `reference_handle_normalization_needed = 115`
- `reference_handle_needs_canonical_name = 75`
- rows with non-empty candidate stems in staging: `40`
- unique candidate stems in staging: `37`
- `product_ready_external_reference_handles = 0`
- `reference_control_core_coverage = 0`

Dry-run conclusion: `reference_bundle` remains raw reference evidence only.
There is no accepted canonical object registry, no product-ready
`external_reference_handles`, and no `reference_control_core`.

## Sequence Summary

- sequence rows: `36`
- single-shot rows: `79`
- distinct sequence IDs: `9`
- sequence rows missing `sequence_id` or `shot_order`: `0`
- blank `sequence_id` rows: `79`
- blank `shot_order` rows: `79`

Sequence IDs:

- `JPSEQ01 = 4`
- `JPSEQ02 = 4`
- `JPSEQ03 = 4`
- `CNSEQ01 = 4`
- `CNSEQ02 = 4`
- `CNSEQ03 = 4`
- `USSEQ01 = 4`
- `USSEQ02 = 4`
- `MIXSEQ01 = 4`

Dry-run conclusion: sequence structure is available as raw planning evidence
only. No sequence product table, DTO, validator, or import structure is created.

## Prompt Candidate Summary

- source official `usable_for_fewshot=Yes`: `97`
- `prompt_body_candidate` preserved in normalized staging: `13`
- `prompt_body_candidate` blocked by placeholder text: `102`
- placeholder-clean positive candidates after staging: `13`
- rows ready for positive few-shot: `0`

Dry-run conclusion: prompt candidates remain source evidence only. No row is
promoted into runtime positive few-shot.

## Validator Evidence Summary

Source evidence fields are present for dry-run review:

- `covered_points`: present on `115` rows
- `missed_points`: present on `115` rows
- `teaching_note`: present on `115` rows
- `ip_abstraction_note`: present on `115` rows
- `continuity_negative_core`: present on `115` rows

Current validator status:

- validator evidence contract accepted for runtime: `false`
- current product validator evidence rows imported: `0`
- current failure-code or severity enum changes: `0`

Dry-run conclusion: evidence is raw planning material only. It does not create
validator rows, failure-code enums, severities, repair behavior, or runtime
blocking logic.

## Promotion Summary

Required dry-run assertions:

```text
raw_source_rows_ready_for_future_dry_run = 115
rows_ready_for_product_import = 0
rows_ready_for_positive_fewshot = 0
product_ready_external_reference_handles = 0
reference_control_core_coverage = 0
```

Additional assertions:

```text
v108_rows_imported_into_seed = 0
v108_rows_imported_into_product_structure = 0
positive_fewshot_promotions = 0
qwen_calls = 0
seedance_calls = 0
v3_branch_edits = 0
hope_product_code_changes = 0
desktop_or_intake_changes = 0
rust_dto_validator_exporter_workbook_ipc_changes = 0
```

## Blocked Row Summary

- blocked source rows in dry-run: `115`

Blocker counts, with multiple blockers possible per row:

- `promotion_gate_not_accepted = 115`
- `reference_handle_unresolved = 115`
- `schema_field_missing = 115`
- `v3_alignment_gap = 115`
- `validator_evidence_not_defined = 115`
- `placeholder_present = 102`
- `prompt_body_blocked_by_placeholder = 102`
- `blank_conditional_surface = 79`
- `reserve_row = 7`
- `sequence_identity_missing_for_sequence_shot = 0`
- `source_hash_mismatch = 0`
- `source_row_count_mismatch = 0`

Dry-run conclusion: all 115 rows remain raw source rows only. Blockers do not
delete source rows; they prevent product import, runtime positive few-shot, and
product-ready reference handle use.

## Changed Files

This dry-run package changes one KB documentation file:

```text
docs/seedance2-v108-import-dry-run-report-2026-04-23.md
```

No seed JSON, manifest, import map, migration, product code, V3 document,
desktop, intake, Rust DTO, validator, exporter, workbook, IPC, Qwen, or
Seedance file is changed by this dry-run report.

## Validation

Command:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\validate-seed-bundle.ps1
```

Result:

```text
Seed bundle validation passed.
```

## no_runtime_import_assertion

This dry-run report is not an implementation gate. It does not authorize any
runtime import or product behavior.

No V108 row has been imported into KB seed rows, Hope product structures,
positive few-shot context, validator runtime evidence, repair mapping runtime
logic, external reference handle product data, or `reference_control_core`.
