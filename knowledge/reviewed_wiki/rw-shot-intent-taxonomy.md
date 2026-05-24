# rw-shot-intent-taxonomy

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Define shot-level intent so storyboard rows carry visible production work
instead of generic descriptions.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Shot intent should be selected from scene purpose and director scheduling.
- Supported first-pass intents are establish, reveal, pursue, clash, reaction,
  transition, payoff, spectacle, and detail_insert.
- Each shot intent must correspond to visible row behavior.
- Shot intent must not require raw KB rows or graph traversal at runtime.

## Runtime Mapping

- director_rule_packs: `dg-shot-intent-taxonomy`
- selected_kb_rules: `rule:shot-intent-taxonomy`
- summary_fragment: `summary:shot-intent-visible-function`

## Negative Constraints

- Do not use intent labels as user-visible prompt text by themselves.
- Do not create rows with no visible action or visual function.

