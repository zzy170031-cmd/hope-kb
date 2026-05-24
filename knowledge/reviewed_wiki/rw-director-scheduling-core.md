# rw-director-scheduling-core

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Guide performance focus, blocking, rhythm, visual attention, and scene-to-shot
handoff while preserving accepted story facts.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Director scheduling inherits accepted story facts and scene expression.
- Storyboard task creation must bind task name, task hash, duration target,
  source fragment, and task queue state to the current accepted body boundary.
- Blocking should clarify objective, conflict, relation, or reveal.
- Rhythm may adjust emphasis, but it must not delete cause and effect.
- Viewer attention should be directed to visible story function, not internal
  rule names or trace evidence.

## Runtime Mapping

- director_rule_packs: `dg-director-scheduling-core`
- scene_mappings: all supported scene types
- selected_kb_rules: `rule:director-scheduling-core`
- summary_fragment: `summary:director-handoff`
- product boundary: `StoryboardTask`, task hash, task queue item, duration
  allocation plan

## Negative Constraints

- Do not let director hints overwrite story facts.
- Do not introduce new people, places, or world rules.
- Do not expose internal KB evidence in storyboard rows.
