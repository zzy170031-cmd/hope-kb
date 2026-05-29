# rw-scene-type-writer-director-interlock

wiki_type: scene_routing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene type writer director interlock: Bind each scene type to the writing information director planning needs and to the director feedback writing must answer.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- export_result
- validate_result

## Claims Summary

- For each scene type, define what writing must expose and what director planning must check before row export.
- Use director feedback to request missing route, rank, prop, space, relation, action, or outcome information.
- Keep scene-type interlock compact in KB summaries and selected rules.

## Runtime Mapping

- writing_rule_packs: wg-scene-type-writer-director-interlock
- selected_kb_rules: rule:scene-type-writer-director-interlock
- runtime_targets: writing_rule_packs, director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- visual_description
- character_action
- camera
- shot_size
- prompt_text
- negative_constraints

## Negative Constraints

- Do not allow a scene type label to stand in for plot, space, action, or relationship logic.
- Do not expose raw matrix rows, source text, or prompt libraries.
