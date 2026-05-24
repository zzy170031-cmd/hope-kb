# rw-prompt-text-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Ensure `prompt_text` is a clean generation-facing field compiled from accepted
facts and confirmed storyboard rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- `prompt_text` should include subject, action, visual frame, camera, duration,
  and negative constraints.
- `prompt_text` must align with `visual_description` and accepted story facts.
- `prompt_text` must be compiled after the confirmed row.
- Internal schema IDs, trace refs, source registers, raw KB, prompt bodies, and
  internal hashes must not appear in user-visible prompt text.

## Runtime Mapping

- director_rule_packs: `dg-prompt-text-boundary`
- validation_rule_packs: `vg-prompt-text-boundary`
- selected_kb_rules: `rule:prompt-text-boundary`
- negative_constraints: `negative:no-internal-prompt-refs`

## Negative Constraints

- Do not expose trace refs, schema IDs, source registers, or internal hashes.
- Do not emit empty prompt text.

