# rw-shot-intent-taxonomy

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

镜头意图分类规则增强: Define shot-level intent so storyboard rows carry visible anime production work instead of generic descriptions.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Select shot intent from scene purpose and director scheduling before choosing camera or shot size.
- Map intents such as establish, reveal, pursue, clash, reaction, transition, payoff, spectacle, and detail insert to visible row behavior.
- Use reaction and detail shots only when they clarify story pressure, emotion, continuity, or payoff.

## Runtime Mapping

- director_rule_packs: dg-shot-intent-taxonomy
- selected_kb_rules: rule:shot-intent-taxonomy
- runtime_targets: director_rule_packs, selected_kb_rules, kb_context_summary

## PWA Fields Served

- camera
- shot_size
- character_action
- prompt_text

## Negative Constraints

- Do not use intent labels as user-visible prompt text by themselves.
- Do not create rows with no visible action or visual function.
