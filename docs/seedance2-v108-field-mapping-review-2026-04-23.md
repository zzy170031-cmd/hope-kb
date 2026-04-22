# Seedance2 V108 Field Mapping Review 2026-04-23

This note records the KB-only field mapping review for the Seedance2.0 V108
fused golden sample source package.

## Scope

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `818c098`
- package type: KB-only raw source receipt, normalized staging, and mapping review

This package does not modify `E:\codex\hope`, V3 proposal files, desktop,
intake, Qwen, Seedance, or Hope product/runtime code. It does not merge
`hope-kb` into `hope`.

## Source Artifacts

Raw artifacts were copied without renaming or content edits:

- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版.xlsx`
  - role: primary data table
  - sha256: `2c7f7a0b37f5d7a6f90f18c0080a78fa568fcb4528b1b4e6fbdbfb959cfe9e34`
- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版说明.docx`
  - role: explanation document
  - sha256: `a2335102154ab88cb5c01517b2b20bbb15efdbdd25f0d32eeb9715d42e8d7ed3`

Derived staging artifact:

- `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`
  - sha256: `a9ace9098ae60ce942ee5101b39329dc5b9ee70ab2d73e20d058d6db5938c167`

## Read-Only XLSX Check

- sheet: `V108融合字段库`
- field count: `23`
- total data rows: `115`
- library status:
  - `official = 108`
  - `reserve = 7`
- official sample types:
  - `single_shot = 72`
  - `sequence_shot = 36`
- official quality distribution:
  - `优 = 42`
  - `好 = 33`
  - `中 = 22`
  - `差 = 11`
- official source few-shot flag:
  - `Yes = 97`
  - `No = 11`

Placeholder scan:

- rows with placeholder-like `待补` text: `102`
- official rows with placeholder-like text: `95`
- reserve rows with placeholder-like text: `7`
- affected fields:
  - `reference_bundle = 102`
  - `prompt_body = 102`
  - `technical_profile = 41`
  - `scene_performance_core = 41`
  - `camera_directing_core = 42`
  - `audio_directing_core = 67`
  - `reserve_reason = 1`

## Field Mapping Review

| V108 source field | Current v0.2 mapping | Review status |
| --- | --- | --- |
| `shot_id` | `sample_id` candidate and preserved in `source_fields` | mappable |
| `library_status` | preserved in `source_fields`; drives official/reserve split | mappable as gate metadata |
| `reserve_reason` | validator / repair / gap evidence only | mappable as evidence, not positive few-shot |
| `sample_type` | preserved in `source_fields`; `sequence_shot` needs sequence semantics | partial; vNext gap |
| `sequence_id` | preserved in `source_fields` | vNext sequence gap |
| `shot_order` | preserved in `source_fields` | vNext sequence gap |
| `sample_title` | existing `source_fields.sample_title` | mappable |
| `style_cluster` | classification/source metadata candidate | partial; no dedicated v0.2 field |
| `scene_category` | scene/genre metadata candidate | partial; no dedicated v0.2 field |
| `scene_tag` | existing scene tag style metadata candidate | mappable |
| `quality_grade` | existing `tier` equivalent candidate | mappable |
| `usable_for_fewshot` | existing few-shot source gate candidate | mappable, still blocked by reserve/negative/placeholder gates |
| `technical_profile` | derived `technical_profile` candidate | schema gap; no top-level v0.2 column |
| `scene_performance_core` | derived `scene_performance_core` candidate | schema gap; no top-level v0.2 column |
| `camera_directing_core` | existing core axis candidate | partial; v0.2 has one core per record, V108 carries multiple fused cores |
| `audio_directing_core` | existing core axis candidate | partial; v0.2 has one core per record, V108 carries multiple fused cores |
| `continuity_negative_core` | negative/continuity validator evidence candidate | partial; needs dedicated continuity negative structure |
| `reference_bundle` | `external_reference_handles` source value and candidate extraction | semantic gap; source is media descriptor / placeholder, not canonical object-name list |
| `ip_abstraction_note` | compliance/source note | mappable as evidence; no product prompt expansion |
| `covered_points` | existing validator evidence | mappable |
| `missed_points` | existing validator evidence | mappable |
| `teaching_note` | existing validator / repair planning evidence | mappable |
| `prompt_body` | `prompt_body_candidate` when placeholder-free | partial; 102 rows blocked by placeholder text |

## external_reference_handles Conclusion

`reference_bundle` is treated only as source input for
`external_reference_handles`, not as image files, URLs, asset IDs, or a completed
`reference_control_core`.

- `reference_handle_needs_canonical_name = 75`
- `reference_handle_normalization_needed = 115`
- 13 rows have placeholder-free `reference_bundle` values that can produce
  provisional handle candidates, but the source values are still media-style
  descriptors such as image/video/audio references.
- No row is promoted to `reference_control_core`.
- No prompt candidate appends appearance, clothing, scene art, file paths, URLs,
  asset IDs, or claims about real reference images.

## prompt_body_candidate Conclusion

- `prompt_body_candidate` preserved from source for `13` placeholder-free rows.
- `prompt_body_candidate` blocked for `102` rows containing `待补`.
- No deterministic fill was attempted for blocked rows.
- Reference append is blocked until canonical external object names are accepted.

## Few-Shot and Evidence Split

- `official` rows may become later positive few-shot candidates only after
  usable/negative/reference/placeholder gates pass.
- `reserve` rows do not enter positive few-shot.
- `reserve`, `negative`, `unusable`, and placeholder-bearing rows remain
  validator / repair / gap evidence.
- Source official few-shot `Yes` rows: `97`
- Placeholder-clean positive few-shot candidates after this staging pass: `13`
- Rows ready for positive few-shot promotion in this package: `0`

## Schema Or Content Gaps

- V108 has 23 fused fields; current v0.2 sample records were designed for the
  earlier 17-field export.
- Current v0.2 has no dedicated top-level columns for:
  - `external_reference_handles`
  - `technical_profile`
  - `scene_performance_core`
  - `prompt_body_candidate`
  - sequence grouping by `sequence_id` / `shot_order`
- Current v0.2 stores one `classification.core` per sample, while V108 carries
  multiple fused core fields per row.
- `reference_bundle` source values are media descriptors or placeholders, not
  canonical external object names.
- 102 rows contain placeholder-like text and must not be guessed into completed
  prompt candidates.

## Decision

This package stages the V108 source package and mapping review only. It does not
directly import the 115 V108 rows into `seed/v0.2/golden_sample_library.json`,
coverage rules, failure mapping, or repair mapping. A later vNext or explicit
main-control import gate should decide the schema extension and promotion rules.

