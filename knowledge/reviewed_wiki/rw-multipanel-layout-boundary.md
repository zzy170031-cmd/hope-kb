# rw-multipanel-layout-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Multipanel layout boundary: Derive multi-panel storyboard image layout from confirmed row count, row timing, and shot purpose without forcing fixed grids, padding shots, or captions.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- export_result
- validate_result

## Claims Summary

- Use confirmed row count, duration, and visible shot purpose to suggest panel layout.
- Allow one row to remain one panel when the shot is coherent and readable.
- Split or group panels only when a real beat, action phase, reaction, spatial relation, or duration need supports it.

## Runtime Mapping

- validation_rule_packs: vg-multipanel-layout-boundary
- selected_kb_rules: rule:multipanel-layout-boundary
- runtime_targets: validation_rule_packs, director_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints, duration_profiles

## PWA Fields Served

- visual_description
- camera
- shot_size
- character_action
- duration_seconds
- prompt_text
- negative_constraints

## Negative Constraints

- Do not force preset panel grids or fixed page slots.
- Do not split a coherent shot just to fill a page or make the board look complete.
- Do not add unsupported captions, dialogue, or row text to satisfy a layout.
