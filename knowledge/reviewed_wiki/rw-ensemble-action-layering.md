# rw-ensemble-action-layering

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Ensemble action layering: Layer anime group shots into primary action, secondary reaction, background motion, and spatial depth without inventing entities.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

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
- camera
- shot_size
- prompt_text
- scene_profile
- duration_seconds
- negative_constraints

## Negative Constraints

- Do not invent headcount, faction, named characters, teams, or unsupported crowd facts.
- Do not place new entities into the person field.
- Do not use group action layering as a generic spectacle filler.
