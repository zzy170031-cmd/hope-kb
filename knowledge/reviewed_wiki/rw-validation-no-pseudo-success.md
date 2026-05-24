# rw-validation-no-pseudo-success

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Prevent empty rows, missing prompt text, blocked state, stale tasks, or stale
artifacts from being treated as successful output.

## Applies To

- validate_result
- repair_storyboard
- export_result

## Claims Summary

- Storyboard success requires non-empty rows.
- Each row must have non-empty visual description and prompt text.
- `rows_match=true` cannot pass when `rows=0`.
- Blocked export cannot be marked ready.
- Repair must be an explicit action after validation failure.

## Runtime Mapping

- validation_rule_packs: `vg-no-pseudo-success`
- selected_kb_rules: `rule:no-pseudo-success`
- summary_fragment: `summary:no-pseudo-success`

## Negative Constraints

- Do not mark rows=0 as success.
- Do not export blocked or stale output as ready.

