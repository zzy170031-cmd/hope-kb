# rw-chinese-heritage-style-scene-routing

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Chinese heritage style scene routing: Allow Chinese heritage visual cues only when the accepted scene, era, object logic, environment, or user intent supports them.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Use Chinese heritage visual cues only when accepted scene facts, era, location, object logic, or user intent support them.
- Bind heritage cues to visible material, line rhythm, spatial composition, prop logic, costume logic, environment, or light rather than treating them as a global style switch.
- Keep cultural cues subordinate to scene objective, character action, continuity, camera, shot size, and prompt boundary.

## Runtime Mapping

- director_rule_packs: dg-chinese-heritage-style-scene-routing
- selected_kb_rules: rule:chinese-heritage-style-scene-routing
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

- Do not add heritage motifs as generic decoration or stereotype.
- Do not use cultural or religious labels without accepted scene support.
- Do not replace accepted setting, props, costume, or character facts with a style family.
