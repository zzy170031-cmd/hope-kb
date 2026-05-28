# rw-scene-driven-style-routing

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Scene driven style routing: Use scene type as the first style prior, then refine with accepted body content and style_profile before projecting style into storyboard fields.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Treat scene type as the first style prior, then narrow style guidance with accepted body content, scene goal, emotional tone, and visual continuity.
- Select only a small style family or production cue set that serves the current scene function.
- Keep scene-driven style routing in summary form so runtime receives field guidance, not an alias table.

## Runtime Mapping

- director_rule_packs: dg-scene-driven-style-routing
- selected_kb_rules: rule:scene-driven-style-routing
- runtime_targets: director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- camera
- shot_size
- prompt_text
- duration_seconds
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not route style from a free-floating alias list when scene type and accepted body conflict with it.
- Do not stack multiple style families to make a row look richer.
- Do not use style routing to bypass person, action, continuity, camera, shot size, or duration constraints.
