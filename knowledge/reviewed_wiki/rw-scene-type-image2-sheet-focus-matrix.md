# rw-scene-type-image2-sheet-focus-matrix

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene type Image2 sheet focus matrix: Bind Image2 storyboard sheet export focus to the current scene type, row purpose, and field-bound negative constraints.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- export_result
- validate_result

## Claims Summary

- Choose Image2 focus from scene-specific subject, action, space, composition, material, lighting, and continuity needs.
- Use storyboard sheet export as a still-reference focus layer, not as complete video or provider execution.
- Pair focus guidance with concise negative constraints that block stale body reuse and unsafe provenance.

## Runtime Mapping

- director_rule_packs: dg-scene-type-image2-sheet-focus-matrix
- selected_kb_rules: rule:scene-type-image2-sheet-focus-matrix
- runtime_targets: director_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints, scene_mappings

## PWA Fields Served

- visual_description
- prompt_text
- negative_constraints

## Negative Constraints

- Do not export a generic quality prompt that ignores scene type.
- Do not write generated images, execution material, or raw prompt libraries back into KB runtime fields.
