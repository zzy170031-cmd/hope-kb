# rw-pwa-21-scene-three-layer-routing

wiki_type: scene_routing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

PWA 21 scene three layer routing: Route each PWA scene type through scene profile, writing rewrite, and director storyboard planning before final field assembly.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- export_result
- validate_result

## Claims Summary

- Select the scene type before rewriting body text, storyboard rows, or Image2 export guidance.
- Change narrative objective, conflict engine, relationship mode, space, and action chain when the scene type changes.
- Keep the 21 scene matrix as summary-only routing guidance rather than raw examples or execution material.

## Runtime Mapping

- writing_rule_packs: wg-pwa-21-scene-three-layer-routing
- selected_kb_rules: rule:pwa-21-scene-three-layer-routing
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- visual_description
- character_action
- camera
- shot_size
- prompt_text
- negative_constraints

## Negative Constraints

- Do not reuse old body, chase, evidence, rain, alley, or pursuit templates after a scene type switch.
- Do not expose audit provenance, audit-only quality anchors, audit prompts, or local-only identifiers in runtime output.
