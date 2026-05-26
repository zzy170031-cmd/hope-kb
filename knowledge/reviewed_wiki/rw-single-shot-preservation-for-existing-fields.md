# rw-single-shot-preservation-for-existing-fields

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Single shot preservation for existing fields: Allow one complete, coherent shot to remain one storyboard row using existing final fields when splitting would reduce readability or invent structure.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Preserve a complete shot as one row when the camera, shot size, visual description, action, dialogue or narration, prompt_text, and duration remain readable together.
- Use existing final fields only; a preservation decision must not introduce new PWA columns or internal note fields.
- Split a preserved shot only when a real content beat, shot purpose, action phase, reaction, or spatial relation requires a separate row.

## Runtime Mapping

- validation_rule_packs: vg-single-shot-preservation-for-existing-fields
- selected_kb_rules: rule:single-shot-preservation-for-existing-fields
- runtime_targets: validation_rule_packs, director_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- person
- camera
- shot_size
- visual_description
- character_action
- dialogue_or_narration
- prompt_text
- duration_seconds

## Negative Constraints

- Do not force a coherent shot into multiple rows because of table shape, fixed duration slicing, or rows_seed count.
- Do not duplicate the same action across rows to manufacture row count.
- Do not hide split rationale in note, status, or internal fields as final output.
