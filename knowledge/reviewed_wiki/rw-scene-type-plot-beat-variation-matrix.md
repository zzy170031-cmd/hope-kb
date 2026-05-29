# rw-scene-type-plot-beat-variation-matrix

wiki_type: writing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene type plot beat variation matrix: Vary plot beats by scene type so story rows do not collapse into one reused pressure template.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- validate_result

## Claims Summary

- Map each scene type to a distinct objective, obstacle, relationship function, spatial logic, and action chain.
- Use scene-specific beat turns before selecting shot or prompt wording.
- Treat repeated beat skeletons across different scene types as a repair signal.

## Runtime Mapping

- writing_rule_packs: wg-scene-type-plot-beat-variation-matrix
- selected_kb_rules: rule:scene-type-plot-beat-variation-matrix
- runtime_targets: writing_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not only swap mood, lighting, or genre adjectives.
- Do not preserve a prior scene's chase or evidence skeleton when the scene type has changed.
