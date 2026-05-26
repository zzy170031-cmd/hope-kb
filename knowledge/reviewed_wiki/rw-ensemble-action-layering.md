# rw-ensemble-action-layering

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

群像动作分层候选: Layer anime group shots into primary action, secondary reaction, background motion, and spatial depth without inventing entities.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Use only accepted characters, explicit role labels, or authorized group labels when describing ensemble action.
- Separate primary action, secondary reaction, background group motion, and spatial layer when a group shot needs readability.
- Bind group motion to shot size, camera purpose, and scene objective.

## Runtime Mapping

- director_rule_packs: dg-ensemble-action-layering
- selected_kb_rules: rule:ensemble-action-layering
- runtime_targets: director_rule_packs, validation_rule_packs, scene_mappings, selected_kb_rules, negative_constraints, kb_context_summary

## PWA Fields Served

- person
- visual_description
- character_action
- shot_size
- prompt_text

## Negative Constraints

- Do not invent headcount, faction, named characters, teams, or unsupported crowd facts.
- Do not place new entities into the person field.
- Do not use group action layering as a generic spectacle filler.
