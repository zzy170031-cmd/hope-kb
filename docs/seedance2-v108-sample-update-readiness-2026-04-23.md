# Seedance2 V108 Sample Update Readiness 2026-04-23

This note records the KB-only sample update readiness gate for the Seedance2.0
V108 fused golden sample source package.

## Scope

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `818c098`
- package type: immutable raw source receipt plus normalized staging

This package does not modify Hope product code, V3 proposal files, desktop,
intake, Qwen, Seedance, or the v0.1 seed/snapshot contract.

## Added Raw Source Artifacts

- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版.xlsx`
- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版说明.docx`

Both files preserve the original filenames and source hashes. They are treated
as immutable raw source artifacts.

## Added Derived KB Artifacts

- `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`
- `docs/seedance2-v108-field-mapping-review-2026-04-23.md`
- `docs/seedance2-v108-sample-update-readiness-2026-04-23.md`

## XLSX Formal Counts

- total rows: `115`
- official rows: `108`
- reserve rows: `7`
- official `single_shot`: `72`
- official `sequence_shot`: `36`
- official few-shot source flag:
  - `Yes = 97`
  - `No = 11`

## Readiness Decision

The V108 package has entered KB-side sample update readiness as raw source and
normalized staging. It has not been promoted into positive few-shot runtime
material.

Current v0.2 sample row counts remain unchanged in this package:

- `golden_sample_library = 40`
- `golden_sample_field_coverage_rules = 5`
- `golden_sample_failure_mapping = 40`
- `golden_sample_repair_mapping = 40`

The v0.2 source register and provenance are updated to record the V108 raw and
derived source package.

## external_reference_handles Handling

- `reference_bundle` is interpreted as external reference handle source input.
- It is not interpreted as image files, URLs, asset IDs, or real material
  bindings.
- `external_reference_handles_candidate` is derived only in the normalized
  staging layer.
- `reference_handle_needs_canonical_name = 75`
- `reference_handle_normalization_needed = 115`
- `reference_control_core` remains absent and is not invented by this package.

## prompt_body_candidate Handling

- `prompt_body_candidate` preserved from source for `13` placeholder-free rows.
- `prompt_body_candidate` blocked for `102` rows containing `待补`.
- No missing prompt body content was guessed or locally filled.
- No appearance, clothing, scene art, image path, URL, or asset ID was appended.

## Positive Few-Shot / Validator Split

- Official samples may become later positive few-shot candidates only after
  usable/negative/reference/placeholder gates pass.
- Reserve samples are excluded from positive few-shot.
- Reserve, negative, unusable, and placeholder-bearing samples remain validator,
  repair, and gap evidence.
- Source official few-shot `Yes` rows: `97`
- Placeholder-clean positive few-shot candidates after this staging pass: `13`
- Rows promoted to positive few-shot in this package: `0`

## Remaining Gaps

- Current v0.2 has no dedicated top-level `external_reference_handles` field.
- Current v0.2 has no dedicated top-level `technical_profile`,
  `scene_performance_core`, or `prompt_body_candidate` fields.
- Current v0.2 has no sequence grouping contract for `sequence_id` and
  `shot_order`.
- V108 carries multiple fused core fields per row, while current v0.2 records
  one main `classification.core`.
- 102 rows include placeholder-like `待补` text.
- Canonical external object names must be reviewed before prompt reference
  append or later positive few-shot promotion.

