# rw-anime-key-pose-silhouette-priority

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Anime key pose silhouette priority: Prioritize visible anime key pose, subject silhouette, action line, and readable frame language over production-process wording.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Describe anime action through key pose, subject silhouette, center of gravity, action line, and readable frame priority.
- Keep the subject silhouette and main action readable before adding light, background, material, or style detail.
- Translate production handoff ideas into visible storyboard and prompt fields instead of exposing production-process terms.

## Runtime Mapping

- director_rule_packs: dg-anime-key-pose-silhouette-priority
- selected_kb_rules: rule:anime-key-pose-silhouette-priority
- runtime_targets: director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- shot_size
- prompt_text

## Negative Constraints

- Do not put production process, workflow labels, internal status, or note text into final storyboard fields.
- Do not let background, lighting, style words, or ensemble noise obscure the main subject silhouette.
- Do not invent characters, props, or animation steps to justify a pose.
