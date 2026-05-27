# rw-character-visual-continuity-anchor

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Character visual continuity anchor: Preserve character identity, relationship, appearance, posture, and expression anchors across storyboard rows and prompt_text.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Carry confirmed identity, relationship, appearance, posture, and expression anchors into every row that uses the character.
- Let prompt_text inherit confirmed character anchors without adding new names, IP references, or source handles.
- Repair drift by restoring the accepted character anchor before changing camera, action, or style wording.

## Runtime Mapping

- validation_rule_packs: vg-character-visual-continuity-anchor
- selected_kb_rules: rule:character-visual-continuity-anchor
- runtime_targets: validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- person
- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not invent character names, relationships, costumes, or visual signatures.
- Do not use file names, source paths, real IP, studio names, or source metadata as continuity switches.
- Do not replace accepted person values with generic role labels when a confirmed character anchor exists.
