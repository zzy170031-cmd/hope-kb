# rw-duration-density-rules

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Connect target durations to shot count, beat density, row duration, and
validation rules.

## Applies To

- create_story_task
- generate_storyboard
- validate_result

## Claims Summary

- Duration planning should preserve semantic beats while matching target
  duration exactly.
- First-pass shot counts are 5s -> 3, 10s -> 4, 15s -> 5, 30s -> 7,
  45s -> 9, and 60s -> 12.
- No row may have zero duration.
- Repeated full-budget rows indicate invalid duration allocation.

## Runtime Mapping

- duration_profiles: all supported durations
- director_rule_packs: `dg-duration-density`
- validation_rule_packs: `vg-duration-integrity`
- selected_kb_rules: `rule:duration-density`

## Negative Constraints

- Do not accept duration sum mismatch.
- Do not treat empty or zero-duration rows as success.

