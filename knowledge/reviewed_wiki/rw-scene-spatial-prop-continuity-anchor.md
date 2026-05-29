# rw-scene-spatial-prop-continuity-anchor

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene spatial prop continuity anchor: Keep location, left-right relation, eyeline, prop anchors, background anchors, and event order continuous across adjacent storyboard rows.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Preserve character position, eyeline direction, prop state, background anchor, and event order when splitting rows.
- Use camera movement and shot size to clarify spatial relation instead of resetting location or screen direction.
- Treat a prop handoff, position change, or event-order change as a row-level continuity requirement.

## Runtime Mapping

- director_rule_packs: dg-scene-spatial-prop-continuity-anchor
- selected_kb_rules: rule:scene-spatial-prop-continuity-anchor
- runtime_targets: director_rule_packs, validation_rule_packs, scene_mappings, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- camera
- prompt_text
- negative_constraints
- person

## Negative Constraints

- Do not jump screen direction, swap left-right placement, lose key props, or change location without accepted cause.
- Do not hide spatial continuity repair in final prompt_text as an internal note.
- Do not create new props or locations to solve a continuity gap.
