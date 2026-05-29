# rw-writer-director-bidirectional-routing

wiki_type: scene_routing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Writer director bidirectional routing: Keep writing and director rules coupled: writing supplies filmable beats, and director planning requests repair when visibility, space, or action logic is missing.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- export_result
- validate_result

## Claims Summary

- Writing must provide visible subject, action cause, relationship state, space, and beat turn before director planning.
- Director planning may request rewrite only for filmability gaps, not for free style expansion.
- Keep the loop summary-only until a PWA implementation gate opens explicit trace fields.

## Runtime Mapping

- writing_rule_packs: wg-writer-director-bidirectional-routing
- selected_kb_rules: rule:writer-director-bidirectional-routing
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- camera
- shot_size
- prompt_text

## Negative Constraints

- Do not invent new runtime fields or UI controls in this KB apply gate.
- Do not turn director feedback into hidden prompt text or execution material.
