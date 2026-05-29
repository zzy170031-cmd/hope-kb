# rw-pwa21-scene-profile-matrix

wiki_type: scene_routing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

PWA21 scene profile matrix: Summarize the PWA 21 scene families as compact routing profiles for writing, director planning, validation, and Image2 storyboard export focus.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- export_result
- validate_result

## Claims Summary

- Use the scene matrix to choose narrative goal, conflict engine, relationship mode, spatial engine, and action chain.
- Expose only compact profile summaries in runtime, never audit material tables or full prompt libraries.
- Use the matrix to detect when a generated body belongs to the old scene type.

## Runtime Mapping

- writing_rule_packs: wg-pwa21-scene-profile-matrix
- selected_kb_rules: rule:pwa21-scene-profile-matrix
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- visual_description
- character_action
- camera
- shot_size
- prompt_text
- negative_constraints

## Negative Constraints

- Do not copy complete scene tables, golden rows, provider prompt text, or audit wording.
- Do not treat 403 or full16 assets as required reruns for this KB apply.
