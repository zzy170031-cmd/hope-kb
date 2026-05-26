# rw-transition-motion-dynamics

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

转场与动势规则增强: Use transitions and motion dynamics to express rhythm, continuity, action pressure, and emotional direction.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Use transitions and motion dynamics to express rhythm and continuity.
- Select fade, dissolve, flash, wipe, push, pull, match cut, or montage by scene function.
- Select horizontal, vertical, diagonal, curved, radial, spiral, or S-curve motion by action pressure and emotional direction.

## Runtime Mapping

- director_rule_packs: dg-transition-motion-dynamics
- selected_kb_rules: rule:transition-motion-dynamics
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules

## PWA Fields Served

- camera
- character_action
- prompt_text

## Negative Constraints

- Do not add decorative transitions unrelated to scene function.
- Do not break action continuity or accepted event order.
