# rw-camera-language-grammar

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Camera language grammar: Bind shot function, scale, angle, movement, composition, and adjacent-shot relation to PWA storyboard fields.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Each row should include shot function, shot scale, camera angle, camera movement, and composition logic when useful.
- Prefer precise camera language over broad cinematic quality words.
- Avoid repeating the same framing and movement across all rows.

## Runtime Mapping

- director_rule_packs: dg-camera-language-grammar
- selected_kb_rules: rule:camera-language-grammar
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- camera
- shot_size
- prompt_text
- duration_seconds

## Negative Constraints

- Do not use camera vocabulary as empty decoration.
- Do not expose internal rule ids in prompt_text.
