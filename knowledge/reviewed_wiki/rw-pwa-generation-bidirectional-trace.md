# rw-pwa-generation-bidirectional-trace

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

PWA generation bidirectional trace: Keep writer-director coupling traceable in summary-only KB rules while leaving explicit PWA trace fields for a future implementation gate.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- export_result
- validate_result

## Claims Summary

- Use selected KB rules and context summaries to show why writing or director repair guidance was applied.
- Keep actual trace fields, UI warnings, and stale-body implementation out of this KB apply gate.
- Validate that summary output contains no audit provenance or raw prompt material.

## Runtime Mapping

- validation_rule_packs: vg-pwa-generation-bidirectional-trace
- selected_kb_rules: rule:pwa-generation-bidirectional-trace
- runtime_targets: validation_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- negative_constraints
- prompt_text

## Negative Constraints

- Do not add PWA fields, adapter schema, UI state, or execution material in this gate.
- Do not expose audit provenance, unpublished KB detail, local-only identifiers, fingerprints, credential material, or credential material.
