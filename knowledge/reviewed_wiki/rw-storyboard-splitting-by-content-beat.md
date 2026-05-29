# rw-storyboard-splitting-by-content-beat

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Storyboard splitting by content beat: Split storyboard rows by semantic content beat, shot purpose, readable action phase, information change, or spatial scheduling change instead of fixed row count or fixed seconds.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Treat rows_seed as semantic storyboard candidates, not as a fixed row-count table or duration averaging plan.
- Split only when the content beat, shot purpose, visible action phase, emotional or information change, or spatial scheduling changes enough to need a separate readable row.
- Consider dialogue or narration beats, prop handoff, attention change, and duration density only when they change what the current row must show.
- Allow a complete shot to remain one row when content, action, rhythm, and prompt_text readability are coherent.

## Runtime Mapping

- validation_rule_packs: vg-storyboard-splitting-by-content-beat
- selected_kb_rules: rule:storyboard-splitting-by-content-beat
- runtime_targets: validation_rule_packs, director_rule_packs, scene_mappings, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- camera
- shot_size
- visual_description
- character_action
- duration_seconds
- prompt_text
- negative_constraints

## Negative Constraints

- Do not split by fixed row count, fixed seconds, table slots, or rows_seed averaging.
- Do not hard-split a complete shot just to fill a table or satisfy a target row count.
- Do not create rows with meaningless prompt_text or no visible content change.
