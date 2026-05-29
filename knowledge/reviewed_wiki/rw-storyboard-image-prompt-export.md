# rw-storyboard-image-prompt-export

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Storyboard image prompt export: Derive an optional anime or manga storyboard reference prompt export from confirmed storyboard rows without adding final PWA fields, changing export headers, or writing the result back into row fields.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- export_result
- validate_result

## Claims Summary

- Build anime, manga, comic, or storyboard reference prompt export guidance only from confirmed row fields and accepted story facts.
- Keep the export separate from final storyboard rows, existing table headers, runtime contracts, and provider API calls.
- Use duration and confirmed row count as layout hints while preserving each row's anime storyboard shot purpose.

## Runtime Mapping

- validation_rule_packs: vg-storyboard-image-prompt-export
- selected_kb_rules: rule:storyboard-image-prompt-export
- runtime_targets: validation_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints, duration_profiles

## PWA Fields Served

- prompt_text
- visual_description
- camera
- shot_size
- negative_constraints
- duration_seconds
- person
- character_action

## Negative Constraints

- Do not create a new final PWA field or alter existing storyboard export headers.
- Do not write generated image prompts back into storyboard row fields.
- Do not copy universal prompt templates, complete case text, audit provenance, local paths, provider credentials, or internal rule identifiers into runtime output.
