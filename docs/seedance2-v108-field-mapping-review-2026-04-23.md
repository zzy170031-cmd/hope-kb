# Seedance2 V108 Field Mapping Review 2026-04-23

This note records the KB-only field mapping review for the Seedance2.0 V108
fused golden sample source package.

## Scope

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `818c098`
- latest pushed source receipt commit before this alignment addendum: `e056e7d`
- package type: KB-only raw source receipt, normalized staging, field mapping
  review, and V3 alignment supervision

This package does not modify `E:\codex\hope`, V3 proposal files, desktop,
intake, Qwen, Seedance, or Hope product/runtime code. It does not merge
`hope-kb` into `hope`.

## Canonical Format Decision

The local V108 XLSX / DOCX table format is the latest canonical source format
for this gate.

Historical V3 remote branch content from
`E:\codex\hope / origin/codex/v3-field-overlay-proposal @ 69c645c` is used only
as proposal / freeze-candidate reference. It is not treated as the current field
format truth and is not modified in this gate.

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

V3 alignment report:

- `docs/seedance2-v108-v3-alignment-report-2026-04-23.md`

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

Placeholder and empty-surface scan:

- rows with placeholder-like `待补` text: `102`
- official rows with placeholder-like text: `95`
- reserve rows with placeholder-like text: `7`
- affected placeholder fields:
  - `reference_bundle = 102`
  - `prompt_body = 102`
  - `technical_profile = 41`
  - `scene_performance_core = 41`
  - `camera_directing_core = 42`
  - `audio_directing_core = 67`
  - `reserve_reason = 1`
- empty / conditional field surfaces:
  - `reserve_reason = 108` blank official rows
  - `sequence_id = 79` blank single-shot rows
  - `shot_order = 79` blank single-shot rows

Empty, blank, and placeholder surfaces are preserved and marked as
`future_model_fill_surface` or schema/content gaps. They are not deleted, locally
filled, counted as separate sample records, promoted to positive few-shot, or
used as current validator evidence.

## Field Mapping Review

| V108 column | meaning | old V3 field if any | current KB v0.2 field if any | mapping status | notes |
| --- | --- | --- | --- | --- | --- |
| `shot_id` | Source row / shot identifier for V108 samples. | `shot_id`; golden-sample proposal `sample_id` | `sample_id`; `source_fields.sample_id` | rename | V108 uses `shot_id`; current KB v0.2 golden sample rows use `sample_id`. Preserve source value and map only after explicit import gate. |
| `library_status` | Official/reserve library split. | none exact; do not confuse with old V3 `sample_type` | none in current seed rows; staged in normalized artifact | new | `official/reserve` is V108 source governance metadata. `reserve` rows are excluded from positive few-shot. |
| `reserve_reason` | Reason a reserve row stays out of official positive promotion. | `repair_hint`, `negative_boundary_marker`, and validator notes are only historical analogues | validator / repair evidence candidate only | new | Blank for 108 official rows. Blank area is conditional metadata, not missing sample content. |
| `sample_type` | V108 structure type: `single_shot` or `sequence_shot`. | `structure_mode`; conflicts with old golden-sample `sample_type=positive/negative/repair_before/repair_after` | none dedicated | rename | `v3_alignment_gap`: V108 `sample_type` does not mean old V3 golden-sample `sample_type`. |
| `sequence_id` | Sequence grouping key for `sequence_shot` rows. | `shot_beats_link`, `sequence_no`, `shot_beats` | none | future_model_fill | Blank for 79 single-shot rows; preserve as future sequence surface and do not count blank cells as records. |
| `shot_order` | Per-sequence order for sequence shots. | `beat_order`, `shot_beats`, `sequence_no` | none | future_model_fill | Blank for 79 single-shot rows; preserve for future model fill / sequence validator design. |
| `sample_title` | Human-readable sample title. | `sample_title` | `source_fields.sample_title` | exact | Safe source title / retrieval label. |
| `style_cluster` | Broad style cluster such as basic migration, anime, guoman, US comic, original. | `style_profile_id` / `retrieval_tags` historical analogues | no dedicated field; possible retrieval tag only | new | Do not emit as imitation language. Needs future retrieval taxonomy review. |
| `scene_category` | Broad scene category / scenario family. | `kb_scene_type`, `shot_function_code`, `retrieval_tags` | no dedicated field; closest current tags are `classification.genre_tags` / `scene_tags` | rename | Candidate scene taxonomy input; not imported into current v0.2 rows. |
| `scene_tag` | Source scene tag. | `scene_tag`, `retrieval_tags` | `source_fields.scene_tag`; `classification.scene_tags` | exact | Existing v0.2 supports scene tags from the earlier 17-field export. |
| `quality_grade` | Quality tier label. | `tier` | `source_fields.tier`; `classification.tier` | rename | Preserve source labels `优/好/中/差`. |
| `usable_for_fewshot` | Source few-shot eligibility flag. | `usable_for_fewshot`; `fewshot.eligible` | `source_fields.usable_for_fewshot`; `fewshot.eligible` | exact | Still gated by official/reserve, negative, placeholder, and reference-handle checks. |
| `technical_profile` | Duration, shot size, transition, lens / aperture / frame-rate, movement profile. | split across `target_clip_duration_sec`, `shot_type`, `transition_note`, `lens_feel`, model-capability fields | none dedicated | split | Contains `待补` in 41 rows. `future_model_fill_surface` until deterministic values exist. |
| `scene_performance_core` | Fused environment, subject, action, micro-expression, light/material, and beat content. | merge of `visual_scene_core` and `motion_performance_core` | no full-content field; current KB stores `core` axis and evidence text | merge | V108 intentionally fuses old V3 visual and motion/performance material. Requires vNext schema decision. |
| `camera_directing_core` | Camera purpose, composition, motion path, axis rule, and relation to adjacent shots. | `camera_directing_core` | no full-content field; current KB stores core-axis samples only | exact | Old V3 field name matches, but current KB v0.2 does not have a top-level long-text content field. |
| `audio_directing_core` | Sound layers, sync points, silence, music/SFX, audio-picture relation. | `audio_directing_core` | no full-content field; current KB stores core-axis samples only | exact | Old V3 field name matches, but current KB v0.2 stores evidence, not fused prompt body content. |
| `continuity_negative_core` | Continuity locks plus prohibitions, empty-word blacklist, and failure risks. | `continuity_lock_core`, `blocking_failure_codes`, `negative_boundary_marker` | `negative_sample`, `validator_evidence`, failure mapping analogues only | split | `v3_alignment_gap`: V108 combines continuity lock and negative constraints; old V3 had separate reference/continuity/validator concepts. |
| `reference_bundle` | Source reference descriptor that must normalize to external reference handle candidates. | historical `reference_control_core`, `asset_registry`, `image_ref_assets`, `video_ref_assets`, `audio_ref_assets` | none dedicated; normalized staging derives `external_reference_handles_candidate` | rename | `v3_alignment_gap`: V108 canonical interpretation is `external_reference_handles` as object-name list, not asset binding or completed `reference_control_core`. |
| `ip_abstraction_note` | IP abstraction / compliance note. | `rights_or_ip_risk_flag`, `copyright_risk_flag`, `negative_boundary_marker` | none dedicated | new | Compliance evidence only; no Hope product prompt expansion. |
| `covered_points` | Source coverage evidence. | `covered_points`, `covered_core_fields`, `covered_atomic_fields` | `validator_evidence.covered_points`; coverage rules | exact | First-class validator evidence when imported through a future gate. |
| `missed_points` | Source missing-point evidence. | `missed_points`, `missing_items`, expected failure evidence | `validator_evidence.missed_points`; failure / repair mapping evidence | exact | Used only as validator / repair / gap evidence. |
| `teaching_note` | Human explanation of why the sample is useful, weak, or risky. | `teaching_note`, `repair_hint` | `source_fields.teaching_note`; `planned_repair_inputs.teaching_note` | exact | Can support future repair rationale, not current product behavior. |
| `prompt_body` | Source prompt text / prompt candidate body. | `sample_text`; historical compiled prompt fields are not canonical truth | none dedicated in current KB v0.2 seed rows | rename | `prompt_body_candidate` is preserved only for 13 placeholder-free rows; 102 rows with `待补` are blocked and recorded as gaps. |

## V3 Alignment Gaps

The following conflicts are intentionally not fixed in the old V3 branch:

- `sample_type`: V108 means `single_shot/sequence_shot`; old golden-sample V3
  proposal used `sample_type` for positive/negative/repair categories.
- `reference_bundle`: V108 normalizes toward `external_reference_handles`;
  old V3 discussed `reference_control_core` and asset registry / ref-assets.
- `continuity_negative_core`: V108 fuses continuity locks and negative
  constraints; old V3 separated `continuity_lock_core`, validator blockers,
  and negative boundary concepts.
- `scene_performance_core`: V108 fuses the old visual and motion/performance
  axes into one source field.
- `prompt_body`: V108 treats source prompt body as sample text /
  `prompt_body_candidate`; old compiled Seedance prompt fields are adapter
  outputs and must not become source truth.

Each item above is marked `v3_alignment_gap` and should wait for main-control
acceptance before any V3 docs-only refresh is scheduled.

## external_reference_handles Conclusion

`reference_bundle` is treated only as source input for
`external_reference_handles`, not as image files, URLs, asset IDs, real material
bindings, or a completed `reference_control_core`.

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
- Empty / conditional surfaces are preserved as `future_model_fill_surface` and
  do not alter the source row count.

## Decision

This package stages the V108 source package, mapping review, and V3 alignment
report only. It does not directly import the 115 V108 rows into
`seed/v0.2/golden_sample_library.json`, coverage rules, failure mapping, or
repair mapping. A later vNext or explicit main-control import gate should decide
the schema extension and promotion rules.

