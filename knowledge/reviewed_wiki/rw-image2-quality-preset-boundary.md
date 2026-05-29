# rw-image2-quality-preset-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Image2 quality preset boundary: Treat low, medium, high, and similar quality settings as provider-side presets that cannot replace row-grounded subject, composition, material, lighting, or forbidden-item structure.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- export_result
- repair_storyboard
- validate_result

## Claims Summary

- Use quality presets only as optional export guidance after row-grounded prompt structure is present.
- Keep provider or model preset choices outside current PWA final fields, storyboard headers, and runtime provider binding.
- Repair quality-preset overuse by restoring subject, composition, material, lighting, and negative constraints first.

## Runtime Mapping

- validation_rule_packs: vg-image2-quality-preset-boundary
- selected_kb_rules: rule:image2-quality-preset-boundary
- runtime_targets: validation_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- prompt_text
- negative_constraints

## Negative Constraints

- Do not treat high-quality, HD, 8K, detail, or similar labels as sufficient prompt structure.
- Do not bind runtime output to one provider quality parameter or API.
- Do not let quality presets override accepted facts, row purpose, or field boundaries.
