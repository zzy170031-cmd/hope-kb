# rw-field-level-failure-repair-routing

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Field level failure repair routing: Route storyboard failures to the specific affected field while preserving row facts, shot purpose, duration, and summary-only boundaries.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Locate failure by final field before repairing person, camera, shot_size, visual_description, character_action, dialogue_or_narration, prompt_text, or duration_seconds.
- Repair only the affected field unless adjacent fields must change to preserve row coherence.
- Keep repair output aligned with accepted facts, shot purpose, duration, and summary-only runtime constraints.

## Runtime Mapping

- validation_rule_packs: vg-field-level-failure-repair-routing
- selected_kb_rules: rule:field-level-failure-repair-routing
- runtime_targets: validation_rule_packs, negative_constraints, duration_profiles, kb_context_summary

## PWA Fields Served

- person
- character_action
- visual_description
- prompt_text
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not rewrite every field with a long universal prompt template.
- Do not use style words to overwrite facts or field responsibilities.
- Do not mix status, notes, evidence, source refs, or internal repair traces into final fields.
