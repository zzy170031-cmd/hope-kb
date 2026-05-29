# rw-expression-physicalization

wiki_type: scene_expression_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Expression physicalization: Convert abstract emotion into visible face, posture, breath, and blocking cues that fit shot scale.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Convert abstract emotion into visible performance cues.
- Use close-up cues such as jaw tension, cheek muscle, locked gaze, nasal flare, lip pressure, or trembling breath only when shot scale supports them.
- Use posture, silhouette, blocking, and motion for wide shots.

## Runtime Mapping

- director_rule_packs: dg-expression-physicalization
- selected_kb_rules: rule:expression-physicalization
- runtime_targets: director_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- camera
- shot_size
- prompt_text

## Negative Constraints

- Do not output abstract mood without visible action.
- Do not force invisible micro-expression cues into wide shots.
