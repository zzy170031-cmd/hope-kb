# rw-style-to-field-projection

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Style to field projection: Project a confirmed style_profile into existing PWA fields through visible frame, action, camera, shot size, duration, prompt text, and safe negative constraints.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Project style_profile into visual_description through palette, material, lighting, texture, composition, and space only when visible in the row.
- Project style_profile into character_action, camera, shot_size, and duration only when it improves action readability, rhythm, or attention path.
- Compile prompt_text from the already-set row fields and one compact style modifier, then place unsafe or unsupported style material into negative_constraints.

## Runtime Mapping

- director_rule_packs: dg-style-to-field-projection
- selected_kb_rules: rule:style-to-field-projection
- runtime_targets: director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- camera
- shot_size
- prompt_text
- duration_seconds
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not create new PWA fields for style output.
- Do not place style terms in person or use style to replace accepted action.
- Do not let prompt_text carry a second scene plan, alias list, or unsupported style stack.
