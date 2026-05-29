# rw-field-level-failure-repair-routing

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Field level failure repair routing: Route storyboard failures to the specific affected field while preserving row facts, shot purpose, duration, and summary-only boundaries.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

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

- scene_profile
- visual_description
- character_action
- prompt_text
- negative_constraints
- camera
- shot_size

## Negative Constraints

- Do not rewrite every field with a long universal prompt template.
- Do not use style words to overwrite facts or field responsibilities.
- Do not mix status, notes, evidence, audit provenance, or internal repair traces into final fields.
