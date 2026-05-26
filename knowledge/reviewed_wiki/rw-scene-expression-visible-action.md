# rw-scene-expression-visible-action

wiki_type: scene_expression_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

场景表达与可见行动规则增强: Convert accepted story material into filmable, visible, and storyboard-ready anime scene expression without losing facts, motivation, or causal purpose.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- expand_story
- rewrite_story
- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Expose the scene objective, obstacle, action, reaction, and turning point as visible storyboard material.
- Convert explanation into visible behavior, frame relation, dialogue action, or environmental consequence only when accepted intent is preserved.
- Keep conflict escalation tied to cause, consequence, and an action or emotion transition.

## Runtime Mapping

- writing_rule_packs: wg-scene-expression-visible-action
- selected_kb_rules: rule:scene-expression-visible-action
- runtime_targets: writing_rule_packs, director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not let style overwrite accepted story facts.
- Do not replace visible action with abstract mood labels.
- Do not use dialogue to invent missing motives, relationships, or facts.
