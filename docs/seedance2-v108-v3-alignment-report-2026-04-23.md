# Seedance2 V108 V3 Alignment Report 2026-04-23

This report compares the local V108 XLSX canonical source format against the
historical V3 / golden sample / field overlay proposal material and the current
hope-kb v0.2 golden sample seed shape.

## Inputs

Canonical source for this gate:

- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版.xlsx`
- `golden-samples/seedance2-v108/Seedance2.0黄金样本库V108_单表融合字段版说明.docx`
- normalized staging:
  `docs/normalized-staging/seedance2-v108-fused-golden-sample.normalized.json`

Historical reference only:

- repo: `E:\codex\hope`
- remote branch: `origin/codex/v3-field-overlay-proposal`
- reviewed commit: `69c645c07e6e745df7beccb8cb91daf249081ca2`
- key docs:
  - `docs/contracts/export-overlays/v3-field-overlay-proposal-2026-04-22.md`
  - `docs/contracts/export-overlays/golden-sample-v0.2-schema-overlay-proposal-2026-04-22.md`
  - `docs/contracts/export-overlays/golden-sample-v0.2-contract-freeze-candidate-2026-04-22.md`
  - `docs/contracts/export-overlays/seedance-v1-v0.2-export-overlay-proposal-2026-04-22.md`

Current KB v0.2 reference:

- `seed/v0.2/golden_sample_library.json`
- `seed/v0.2/golden_sample_field_coverage_rules.json`
- `seed/v0.2/golden_sample_failure_mapping.json`
- `seed/v0.2/golden_sample_repair_mapping.json`

## Governing Decision

The local V108 XLSX / DOCX table format is the latest canonical source format.
Old V3 material remains proposal / freeze-candidate history only. No old V3
branch files were changed.

If V108 conflicts with old V3 terms, this report marks the item as
`v3_alignment_gap` and leaves the old V3 branch untouched. A V3 docs-only refresh
should only be scheduled after main control accepts this KB update.

## V108 Field Set

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

## Alignment Table

| V108 column | meaning | old V3 field if any | current KB v0.2 field if any | mapping status | notes |
| --- | --- | --- | --- | --- | --- |
| `shot_id` | Source row / shot identifier. | `shot_id`; golden-sample `sample_id` | `sample_id`; `source_fields.sample_id` | rename | Preserve source value. Rename only in a future import layer if v0.2/vNext accepts it. |
| `library_status` | Official/reserve governance status. | none exact | none in v0.2 seed rows | new | V108 canonical source status. `reserve` is not positive few-shot. |
| `reserve_reason` | Reserve-only exclusion rationale. | `repair_hint`, `negative_boundary_marker` analogues | validator / repair evidence candidate only | new | Blank official rows are conditional metadata, not sample content. |
| `sample_type` | `single_shot` or `sequence_shot`. | `structure_mode`; conflicts with old golden sample `sample_type` | none | rename | `v3_alignment_gap`: old V3 `sample_type` meant sample category, not shot structure. |
| `sequence_id` | Sequence grouping key. | `shot_beats_link`, `sequence_no`, `shot_beats` | none | future_model_fill | Blank for single-shot rows; future sequence surface only. |
| `shot_order` | Order inside sequence. | `beat_order`, `sequence_no`, `shot_beats` | none | future_model_fill | Blank for single-shot rows; not a current record count. |
| `sample_title` | Human-readable title. | `sample_title` | `source_fields.sample_title` | exact | Directly mappable. |
| `style_cluster` | Broad style cluster. | `style_profile_id`, `retrieval_tags` | no dedicated field | new | Internal retrieval taxonomy candidate only. |
| `scene_category` | Broad scene family. | `kb_scene_type`, `shot_function_code`, `retrieval_tags` | no dedicated field; closest `classification.genre_tags` / `scene_tags` | rename | Needs future scene taxonomy review. |
| `scene_tag` | Scene tag. | `scene_tag`, `retrieval_tags` | `source_fields.scene_tag`; `classification.scene_tags` | exact | Existing v0.2 can preserve analogous value. |
| `quality_grade` | Quality tier. | `tier` | `source_fields.tier`; `classification.tier` | rename | Preserve `优/好/中/差`. |
| `usable_for_fewshot` | Few-shot source gate. | `usable_for_fewshot`; `fewshot.eligible` | `source_fields.usable_for_fewshot`; `fewshot.eligible` | exact | Still subject to reserve/negative/placeholder gates. |
| `technical_profile` | Duration, shot size, transition, lens/frame-rate, movement. | `target_clip_duration_sec`, `shot_type`, `transition_note`, `lens_feel`, model capability fields | none dedicated | split | 41 rows include `待补`; future model fill / schema extension needed. |
| `scene_performance_core` | Fused visual scene and motion/performance content. | `visual_scene_core` + `motion_performance_core` | no full-content field | merge | `v3_alignment_gap`: V108 merges old V3 axes. |
| `camera_directing_core` | Camera directing content. | `camera_directing_core` | no full-content field; current KB only has core-axis sample evidence | exact | Old field name matches; KB v0.2 cannot yet store it as top-level prompt content. |
| `audio_directing_core` | Audio directing content. | `audio_directing_core` | no full-content field; current KB only has core-axis sample evidence | exact | Old field name matches; KB v0.2 cannot yet store it as top-level prompt content. |
| `continuity_negative_core` | Continuity locks, prohibitions, empty-word blacklist, risk list. | `continuity_lock_core`, `blocking_failure_codes`, `negative_boundary_marker` | `negative_sample`, `validator_evidence`, failure mapping analogues | split | `v3_alignment_gap`: V108 combines continuity and negative constraints. |
| `reference_bundle` | Source reference descriptor to normalize into external object handles. | `reference_control_core`, `asset_registry`, `image_ref_assets`, `video_ref_assets`, `audio_ref_assets` | none; normalized staging has `external_reference_handles_candidate` | rename | `v3_alignment_gap`: canonical target is `external_reference_handles`, not asset binding or completed `reference_control_core`. |
| `ip_abstraction_note` | IP abstraction / compliance note. | `rights_or_ip_risk_flag`, `copyright_risk_flag`, `negative_boundary_marker` | none dedicated | new | Evidence only; not prompt expansion. |
| `covered_points` | Covered evidence. | `covered_points`, `covered_core_fields`, `covered_atomic_fields` | `validator_evidence.covered_points`; coverage rules | exact | Future validator evidence. |
| `missed_points` | Missing evidence. | `missed_points`, `missing_items` | `validator_evidence.missed_points`; failure / repair mapping evidence | exact | Future repair / validator evidence. |
| `teaching_note` | Human teaching rationale. | `teaching_note`, `repair_hint` | `source_fields.teaching_note`; `planned_repair_inputs.teaching_note` | exact | Future repair rationale only. |
| `prompt_body` | Source prompt text / candidate body. | `sample_text`; compiled prompt fields are adapter outputs, not source truth | none dedicated | rename | 13 placeholder-free rows preserved as candidates; 102 rows blocked by `待补`. |

## V3 Alignment Gaps

- `sample_type` conflict: V108 uses shot structure semantics; old V3 golden
  sample proposal used positive/negative/repair sample categories.
- `scene_performance_core` merge: V108 fuses old `visual_scene_core` and
  `motion_performance_core`.
- `continuity_negative_core` split need: V108 combines continuity locks with
  negative constraints, while old V3 separated continuity, blockers, and
  negative boundary concepts.
- `reference_bundle` reinterpretation: V108 should become
  `external_reference_handles`, while old V3 spoke in terms of
  `reference_control_core`, asset registry, and image/video/audio ref assets.
- `prompt_body` reinterpretation: V108 prompt text is source sample text /
  candidate text, not `compiled_prompt_seedance` or adapter output truth.

No V3 branch change is made for these gaps.

## Future Model Fill Surfaces

V108 contains no empty worksheet. It does contain blank or placeholder field
surfaces that must be preserved:

| surface | count | handling |
| --- | ---: | --- |
| blank `reserve_reason` on official rows | 108 | conditional metadata; not evidence and not a sample count |
| blank `sequence_id` on single-shot rows | 79 | `future_model_fill_surface`; not a sequence record count |
| blank `shot_order` on single-shot rows | 79 | `future_model_fill_surface`; not a sequence record count |
| `reference_bundle` containing `待补` | 102 | gap report; no asset/path/reference claim |
| `prompt_body` containing `待补` | 102 | gap report; block `prompt_body_candidate` |
| `technical_profile` containing `待补` | 41 | `future_model_fill_surface`; no local fill |
| `scene_performance_core` containing `待补` | 41 | `future_model_fill_surface`; no local fill |
| `camera_directing_core` containing `待补` | 42 | `future_model_fill_surface`; no local fill |
| `audio_directing_core` containing `待补` | 67 | `future_model_fill_surface`; no local fill |

These surfaces do not enter current positive few-shot or current validator
evidence. They are template/future-fill structure only until a later explicit
main-control gate.

## Current KB v0.2 Impact

Current v0.2 row counts remain unchanged:

- `golden_sample_library = 40`
- `golden_sample_field_coverage_rules = 5`
- `golden_sample_failure_mapping = 40`
- `golden_sample_repair_mapping = 40`

The V108 package is not imported into these records. It is staged as raw source,
normalized candidate material, gap evidence, and V3 alignment evidence.

## Recommendation

Keep V108 accepted as KB-side raw source and normalized staging. After main
control accepts the KB update, schedule a separate V3 docs-only refresh to
replace historical field assumptions with the V108 canonical format and the
`external_reference_handles` interpretation.

