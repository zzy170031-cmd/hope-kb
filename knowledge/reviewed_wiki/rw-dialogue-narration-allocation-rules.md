# rw-dialogue-narration-allocation-rules

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Dialogue narration allocation rules: Allocate dialogue, narration, action explanation, and duration to the current storyboard row only when supported by text, fact, character intent, or scene purpose.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Bind each dialogue or narration line to the current row's person, action, scene purpose, and duration.
- Use visible action or silence when dialogue evidence is weak.
- Keep narration short and row-bound when it clarifies information that cannot be shown visibly.

## Runtime Mapping

- validation_rule_packs: vg-dialogue-narration-allocation-rules
- selected_kb_rules: rule:dialogue-narration-allocation-rules
- runtime_targets: writing_rule_packs, validation_rule_packs, duration_profiles, kb_context_summary

## PWA Fields Served

- person
- character_action
- dialogue_or_narration
- prompt_text
- duration_seconds

## Negative Constraints

- Do not turn this rule into an automatic dialogue-generation template.
- Do not use narration to replace visible action that the row can show.
- Do not move dialogue across rows or invent exposition to fill duration.
