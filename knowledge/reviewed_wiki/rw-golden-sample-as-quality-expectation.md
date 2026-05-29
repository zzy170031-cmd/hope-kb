# rw-golden-sample-as-quality-expectation

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Golden sample as quality expectation: Use golden samples only as abstract quality expectations, negative controls, and anti-homogeneity anchors, never as raw runtime samples.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- validate_result
- export_result
- generate_storyboard

## Claims Summary

- Convert golden samples into abstract expectations for scene differentiation, density, continuity, and readability.
- Use golden anchors to check whether a scene type changed objective, conflict, relationship, space, and action chain.
- Keep 403 and full16 artifacts as boundary evidence only unless a separate QA gate opens.

## Runtime Mapping

- validation_rule_packs: vg-golden-sample-as-quality-expectation
- selected_kb_rules: rule:golden-sample-as-quality-expectation
- runtime_targets: validation_rule_packs, director_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints, scene_mappings

## PWA Fields Served

- negative_constraints
- prompt_text
- visual_description

## Negative Constraints

- Do not copy audit-only quality anchors text, complete shot tables, hidden prompt templates, audit provenance, or execution material.
- Do not turn real director, real studio, or IP style evidence into runtime style switches.
