# rw-visual-master-consistency

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Visual master consistency: Carry a compact visual master into storyboard rows while preserving accepted facts.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Carry subject identity, world, palette, lighting logic, material feel, era, and key props into every storyboard row.
- Vary framing and action without resetting the visual world.
- Treat visual master as continuity guidance, not as a source of new facts.

## Runtime Mapping

- director_rule_packs: dg-visual-master-consistency
- selected_kb_rules: rule:visual-master-consistency
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- person
- visual_description
- character_action
- prompt_text
- negative_constraints

## Negative Constraints

- Do not overwrite accepted body facts.
- Do not expose source images, OCR text, local paths, or source registers.
