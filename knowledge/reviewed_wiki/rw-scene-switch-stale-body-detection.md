# rw-scene-switch-stale-body-detection

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene switch stale body detection: Detect when a scene switch keeps the previous body skeleton, narrative objective, space, or action chain.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- validate_result
- repair_storyboard

## Claims Summary

- Treat unchanged objective, conflict engine, relationship mode, space, or action chain after a scene switch as stale-body risk.
- Use KB summary rules to flag stale body reuse now; leave exact PWA trace fields and UI behavior to a future implementation gate.
- Repair by returning to the selected scene profile and writing/director interlock.

## Runtime Mapping

- validation_rule_packs: vg-scene-switch-stale-body-detection
- selected_kb_rules: rule:scene-switch-stale-body-detection
- runtime_targets: validation_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints, scene_mappings

## PWA Fields Served

- negative_constraints
- prompt_text

## Negative Constraints

- Do not add runtime trace fields, UI warnings, adapter schema, or PWA code in this KB gate.
- Do not expose prior prompt, audit provenance, or unpublished KB detail while checking stale content.
