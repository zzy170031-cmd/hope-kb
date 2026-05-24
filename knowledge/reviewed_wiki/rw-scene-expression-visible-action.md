# rw-scene-expression-visible-action

wiki_type: scene_expression_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Convert accepted story material into filmable, visible, and
storyboard-ready scene expression without losing facts, motivation, or causal
purpose.

## Applies To

- expand_story
- rewrite_story
- create_story_task
- generate_storyboard

## Claims Summary

- A scene should expose objective, obstacle, action, reaction, and turning
  point.
- Exposition should become visible behavior or dialogue action only when the
  original intent is preserved.
- Decorative scenes with no causal purpose should be blocked or downgraded.
- Scene expression may compress prose, but it must keep accepted facts and
  scene purpose intact.

## Runtime Mapping

- writing_rule_packs: `wg-scene-expression-visible-action`
- director_rule_packs: `dg-director-scheduling-core`
- selected_kb_rules: `rule:scene-expression-visible-action`
- summary_fragment: `summary:visible-action-scene-expression`

## Negative Constraints

- Do not let style overwrite story facts.
- Do not replace visible action with abstract mood labels.
- Do not include raw source or internal graph evidence.
