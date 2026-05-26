# rw-anime-translation-failure-guard

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

动漫化失败防护候选: Prevent live-action film feel, proper-name style switches, IP references, and generic cinematic wording from bypassing anime storyboard field translation.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Translate film-language input into anime subject, camera, shot size, visible frame, character action, and field-bound prompt constraints before runtime use.
- Block live-action film feel, real director or studio references, IP references, and generic cinematic quality words as runtime style switches.
- When a candidate cannot map to fixed PWA final fields, keep it in FutureQA or reject it before adapter downlink.

## Runtime Mapping

- validation_rule_packs: vg-anime-translation-failure-guard
- selected_kb_rules: rule:anime-translation-failure-guard
- runtime_targets: validation_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not use real directors, studios, films, games, IP names, or proper-name style references as runtime style switches.
- Do not let generic film terms replace anime storyboard language.
- Do not move FutureQA items, long universal templates, automatic dialogue routines, or generic anime lexicons into runtime.
