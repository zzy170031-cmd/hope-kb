# rw-writing-continuity-core

wiki_type: writing_continuity_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Preserve accepted user facts, character motivation, event order, causality,
timeline, prop state, and setup/payoff obligations during story expansion and
rewriting.

## Applies To

- expand_story
- rewrite_story
- accept_story_body
- create_story_task

## Claims Summary

- User facts outrank KB suggestions, scene style, director hints, and prompt
  packaging.
- Accepted body is the fact-lock boundary. Later storyboard tasks, rows, and
  prompt text must trace back to that accepted body rather than older drafts or
  stale candidate text.
- Expansion may add connective tissue, but it must not change accepted people,
  places, goals, events, or constraints.
- Conflict escalation requires a cause, a consequence, and an emotional or
  action transition.
- Timeline, prop state, and unresolved setup/payoff obligations must be carried
  into later story and storyboard stages.

## Runtime Mapping

- writing_rule_packs: `wg-continuity-core`
- scene_mappings: all supported scene types
- selected_kb_rules: `rule:writing-continuity-core`
- summary_fragment: `summary:continuity-first`
- product boundary: `accepted_body` / accepted rewrite snapshot /
  StoryFactFrame-equivalent fact lock

## Negative Constraints

- Do not rewrite accepted facts for stronger style.
- Do not invent unsupported world facts.
- Do not expose raw source, raw KB rows, source registers, or prompt bodies.
