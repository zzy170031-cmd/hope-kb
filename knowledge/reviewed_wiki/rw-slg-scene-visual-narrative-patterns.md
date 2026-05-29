# rw-slg-scene-visual-narrative-patterns

wiki_type: scene_routing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

SLG scene visual narrative patterns: Route SLG map, march, city-growth, and battle-report scenes through strategic state, force relation, terrain, data hierarchy, and legible outcome.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- export_result
- validate_result

## Claims Summary

- For SLG scenes, prioritize strategic objective, force relation, route, terrain, resource phase, or outcome hierarchy over character-only drama.
- Use map, formation, city phase, or report interface composition only as summary-level storyboard guidance.
- Keep UI-like report scenes legible without dumping fake text or raw data.

## Runtime Mapping

- director_rule_packs: dg-slg-scene-visual-narrative-patterns
- selected_kb_rules: rule:slg-scene-visual-narrative-patterns
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- visual_description
- camera
- shot_size
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not shrink SLG scenes into solo running, alley pursuit, or ordinary dialogue drama.
- Do not expose raw tables, local assets, audit provenance, or execution material.
