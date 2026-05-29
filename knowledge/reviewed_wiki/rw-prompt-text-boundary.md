# rw-prompt-text-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Prompt text boundary: Ensure prompt_text is a clean AI anime generation-facing field compiled from accepted facts and confirmed storyboard rows.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- import_source
- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints.
- Keep prompt_text aligned with accepted story facts and confirmed row fields.
- Use style_profile as a compact field-bound modifier after the row's subject, action, camera, shot size, and visible frame are settled.
- Use compact field-bound anime prompt wording rather than long universal templates.

## Runtime Mapping

- validation_rule_packs: vg-prompt-text-boundary
- selected_kb_rules: rule:prompt-text-boundary
- runtime_targets: director_rule_packs, validation_rule_packs, selected_kb_rules, negative_constraints

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- prompt_text
- negative_constraints
- camera
- shot_size

## Negative Constraints

- Do not expose trace refs, schema IDs, audit-only provenance, unpublished KB detail, local-only identifiers, or internal fingerprints.
- Do not emit empty prompt_text.
- Do not paste style alias catalogs or blocked style labels into prompt_text.
- Do not mix note, status, governance evidence, or repair traces into final fields.
