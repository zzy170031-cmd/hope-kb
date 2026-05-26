# rw-duration-density-rules

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Duration density rules: Connect target durations to shot count, beat density, row duration, and validation rules.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- validate_result

## Claims Summary

- Preserve semantic beats while matching target duration exactly.
- Use duration to control reaction-shot density, action phases, impact pauses, and row information load.
- Keep every row duration positive and aligned with the selected duration profile.

## Runtime Mapping

- director_rule_packs: dg-duration-density
- selected_kb_rules: rule:duration-density
- runtime_targets: duration_profiles, director_rule_packs, validation_rule_packs, selected_kb_rules, negative_constraints

## PWA Fields Served

- character_action
- dialogue_or_narration
- prompt_text
- duration_seconds
- person
- shot_size
- visual_description

## Negative Constraints

- Do not accept duration sum mismatch.
- Do not treat empty or zero-duration rows as success.
- Do not overload a short row with multiple unrelated story objectives.
