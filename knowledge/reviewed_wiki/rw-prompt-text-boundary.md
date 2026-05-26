# rw-prompt-text-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

分镜提示词边界规则增强: Ensure prompt_text is a clean AI anime generation-facing field compiled from accepted facts and confirmed storyboard rows.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- import_source
- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints.
- Keep prompt_text aligned with accepted story facts and confirmed row fields.
- Use compact field-bound anime prompt wording rather than long universal templates.

## Runtime Mapping

- validation_rule_packs: vg-prompt-text-boundary
- selected_kb_rules: rule:prompt-text-boundary
- runtime_targets: director_rule_packs, validation_rule_packs, selected_kb_rules, negative_constraints

## PWA Fields Served

- prompt_text
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not expose trace refs, schema IDs, source registers, raw KB, prompt bodies, local paths, or internal hashes.
- Do not emit empty prompt_text.
- Do not mix note, status, governance evidence, or repair traces into final fields.
