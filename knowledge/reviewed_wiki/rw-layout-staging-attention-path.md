# rw-layout-staging-attention-path

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Layout staging attention path: Arrange subject, action, background, light, composition, and depth so the viewer reads the current row's most important information first.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Set a primary visual focus before adding secondary action, background detail, light, or style wording.
- Use staging, depth, silhouette, contrast, and motion direction to guide attention through the row.
- Keep composition subordinate to the current character action, scene objective, and shot size.

## Runtime Mapping

- director_rule_packs: dg-layout-staging-attention-path
- selected_kb_rules: rule:layout-staging-attention-path
- runtime_targets: director_rule_packs, validation_rule_packs, selected_kb_rules, kb_context_summary

## PWA Fields Served

- shot_size
- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not let multiple subjects, background details, style terms, and action words compete for the same focus.
- Do not use composition language as decorative filler disconnected from the row objective.
- Do not bury the main action behind atmosphere or background description.
