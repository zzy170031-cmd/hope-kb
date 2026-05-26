# rw-dialogue-evidence-lock

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

对白旁白依据锁候选: Keep dialogue and narration grounded in user-provided text, accepted facts, explicit character intent, or the current scene goal.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- rewrite_story
- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Use dialogue or narration only when it is supported by accepted facts, user text, clear character intent, or current scene purpose.
- When evidence is weak, prefer visible action, reaction, silence, or a repair request instead of invented dialogue.
- Keep dialogue and narration aligned with person identity, scene causality, and final-field boundaries.

## Runtime Mapping

- validation_rule_packs: vg-dialogue-evidence-lock
- selected_kb_rules: rule:dialogue-evidence-lock
- runtime_targets: validation_rule_packs, writing_rule_packs, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- prompt_text
- character_action
- visual_description

## Negative Constraints

- Do not treat this rule as an automatic dialogue template.
- Do not invent promises, relationships, motives, exposition, or facts to justify a line.
- Do not leak source evidence or internal governance text into dialogue or prompt_text.
