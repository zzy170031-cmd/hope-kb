# rw-prompt-load-and-shot-plan-boundary

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Prompt load and shot plan boundary: Keep prompt_text compact, row-bound, and single-shot focused while blocking long prompt overload, multi-shot mixing, and empty prompt rows.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- Compile prompt_text from the current row fields and one main shot objective.
- Use shot plan only as internal row planning support; final prompt_text must stay readable, compact, and bound to the current row.
- Reduce prompt load by choosing the current subject, action, camera, shot size, visible frame, and safe constraints instead of stacking unrelated instructions.

## Runtime Mapping

- validation_rule_packs: vg-prompt-load-and-shot-plan-boundary
- selected_kb_rules: rule:prompt-load-and-shot-plan-boundary
- runtime_targets: validation_rule_packs, director_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- person
- camera
- shot_size
- visual_description
- character_action
- dialogue_or_narration
- prompt_text
- duration_seconds

## Negative Constraints

- Do not pack multiple shot objectives, unrelated beats, or full scene plans into one prompt_text row.
- Do not create empty or meaningless prompt_text rows.
- Do not use long universal prompt templates, raw source text, internal notes, or workflow labels as final prompt_text.
