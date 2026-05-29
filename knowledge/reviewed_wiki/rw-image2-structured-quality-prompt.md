# rw-image2-structured-quality-prompt

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Image2 structured quality prompt: Keep derived Image2 prompt guidance structured by visible purpose, subject, composition, material, lighting, and concise negative constraints instead of generic quality-word stacking.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- export_result
- repair_storyboard
- validate_result

## Claims Summary

- Use Image2 quality guidance only as summary-only export or repair support after the storyboard row fields are confirmed.
- Structure quality prompts around subject anchor, composition hierarchy, material, lighting, and field-bound negative constraints.
- Prefer compact visible controls over long universal templates, raw user experience text, or quality adjective stacks.

## Runtime Mapping

- validation_rule_packs: vg-image2-structured-quality-prompt
- selected_kb_rules: rule:image2-structured-quality-prompt
- runtime_targets: validation_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- visual_description
- prompt_text
- negative_constraints

## Negative Constraints

- Do not copy fixed prompt templates, user experience text, source evidence, local paths, provider syntax, or raw negative-prompt dictionaries.
- Do not let quality words replace subject, action, camera, shot size, material, lighting, or composition control.
- Do not write generated images, image prompts, or provider results back into storyboard row fields.
