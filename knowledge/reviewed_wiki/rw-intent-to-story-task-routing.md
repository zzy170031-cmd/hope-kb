# rw-intent-to-story-task-routing

wiki_type: writing_continuity_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Intent to story task routing: Route short user input into story expansion, script rewrite, or storyboard creation before filling existing final storyboard fields.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Classify short input as story expansion, script rewrite, or storyboard creation before planning rows.
- Bind each row to an explicit change, character intent, scene purpose, and field-level output need.
- Use routing to decide whether the next step needs visible action, dialogue or narration, camera planning, duration allocation, or repair.

## Runtime Mapping

- writing_rule_packs: wg-intent-to-story-task-routing
- selected_kb_rules: rule:intent-to-story-task-routing
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, kb_context_summary

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not answer short input as a generic question when a product task is required.
- Do not apply fixed three-act, fifteen-beat, or row-count templates without field evidence.
- Do not invent character relationships, dialogue, or backstory to make the route look complete.
