# rw-director-group-scene-shot-profile

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Director group scene shot profile: Convert scene-specific writing beats into shot size, camera, blocking, action readability, and storyboard sheet focus.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Choose shot size, camera, blocking, and transition logic from the scene objective and visible action.
- Ask for writing repair when subject, space, action chain, relationship state, or emotional externalization is missing.
- Keep storyboard and Image2 export focus tied to the current scene type rather than a generic cinematic template.

## Runtime Mapping

- director_rule_packs: dg-director-group-scene-shot-profile
- selected_kb_rules: rule:director-group-scene-shot-profile
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- camera
- shot_size
- visual_description
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not use director vocabulary as decoration when the writing beat is not filmable.
- Do not turn abstract director methods into real-director, studio, IP, or style switches.
