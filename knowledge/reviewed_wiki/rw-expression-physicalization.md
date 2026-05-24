# rw-expression-physicalization

wiki_type: scene_expression_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

表情表演物理化规则: Convert abstract emotion into visible face, posture, breath, and blocking cues that fit shot scale.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

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

- character_action
- visual_description
- prompt_text

## Negative Constraints

- Do not output abstract mood without visible action.
- Do not force invisible micro-expression cues into wide shots.
