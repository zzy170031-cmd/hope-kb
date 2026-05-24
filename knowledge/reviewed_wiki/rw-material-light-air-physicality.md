# rw-material-light-air-physicality

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

材质光影空气物理反馈规则: Add scene-bound material feedback, light direction, and air medium cues to visible frame and prompt text.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Add visible material weight, resistance, wetness, roughness, wear, or reflection when tied to action.
- Add light direction, hardness, rim light, side backlight, haze, rain, dust, smoke, steam, sparks, or water reflection only when supported by scene logic.
- Keep physical detail bound to current action.

## Runtime Mapping

- director_rule_packs: dg-material-light-air-physicality
- selected_kb_rules: rule:material-light-air-physicality
- runtime_targets: director_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not add unsupported world facts or weather effects.
- Do not replace action with texture lists.
