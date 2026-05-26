# rw-validation-no-leakage

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

无泄漏验证规则增强: Prevent raw KB, source registers, prompt bodies, internal refs, local paths, provider config, keys, tokens, and secrets from leaking into PWA prompts, UI, traces, or exports.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- import_source
- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Keep runtime snapshots and PWA output summary-only.
- Reject raw source material, source registers, prompt bodies, raw graphs, local paths, provider config, credentials, and internal evidence in user-visible fields.
- Keep FutureQA and rejected material out of runtime and adapter downlink.

## Runtime Mapping

- validation_rule_packs: vg-no-leakage
- selected_kb_rules: rule:no-leakage
- runtime_targets: validation_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- prompt_text
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not expose raw KB rows, prompt bodies, source registers, local paths, or secrets.
- Do not expose internal evidence in exported files.
- Do not move FutureQA items into runtime.
