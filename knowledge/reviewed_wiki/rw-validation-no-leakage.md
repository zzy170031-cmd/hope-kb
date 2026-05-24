# rw-validation-no-leakage

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Prevent raw KB, source registers, prompt bodies, internal refs, local paths,
provider config, API keys, tokens, and secrets from leaking into PWA prompts,
UI, traces, or exports.

## Applies To

- import_source
- validate_result
- export_result
- golden_sample_review

## Claims Summary

- Runtime snapshots and PWA output must remain summary-only.
- `raw_kb_rows_included` must be `0`.
- Raw sample text, source registers, overlay JSON, prompt bodies, raw graphs,
  local paths, provider config, keys, tokens, and secrets must be absent.
- User exports must not include internal hashes, refs, or governance evidence.

## Runtime Mapping

- validation_rule_packs: `vg-no-leakage`
- selected_kb_rules: `rule:no-leakage`
- summary_fragment: `summary:summary-only-boundary`
- negative_constraints: `negative:no-raw-kb-or-internal-refs`

## Negative Constraints

- Do not expose raw KB rows, prompt bodies, source registers, local paths, or
  secrets.
- Do not expose internal evidence in exported files.
