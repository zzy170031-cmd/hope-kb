# rw-style-contamination-guard

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Style contamination guard: Block unsupported style switches, raw alias catalog material, blocked categories, and proper-name style references from runtime fields, prompt text, adapter output, and exports.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Keep style alias normalization and blocked-category review in the audit layer; runtime receives only compact, accepted, field-bound style guidance.
- Reject proper-name style references, real IP, real creators, real studios, incomplete labels, unsupported cultural or religious labels, generalized category labels, and unconstrained media-format labels.
- When style evidence is weak, repair by dropping the style cue or returning to neutral scene-driven guidance.

## Runtime Mapping

- validation_rule_packs: vg-style-contamination-guard
- selected_kb_rules: rule:style-contamination-guard
- runtime_targets: validation_rule_packs, scene_mappings, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- prompt_text
- negative_constraints
- kb_context_summary
- selected_kb_rules

## Negative Constraints

- Do not expose raw alias lists, audit-only provenance, or rejected labels in runtime output.
- Do not use proper names, protected works, studios, creators, or unsupported cultural labels as style switches.
- Do not let a blocked style item survive inside prompt_text, selected_kb_rules, kb_context_summary, or exports.
