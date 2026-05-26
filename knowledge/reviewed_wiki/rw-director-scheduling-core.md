# rw-director-scheduling-core

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Director scheduling core: Guide performance focus, blocking, rhythm, visual attention, and scene-to-shot handoff while preserving accepted story facts.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- Bind blocking, gaze, spacing, reaction, and rhythm to scene objective, conflict, relation, or reveal.
- Use director scheduling to turn accepted story material into visible anime shot guidance.
- Keep task, duration, source boundary, and scene purpose aligned before storyboard generation.

## Runtime Mapping

- director_rule_packs: dg-director-scheduling-core
- selected_kb_rules: rule:director-scheduling-core
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- person
- camera
- shot_size
- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not let director hints overwrite story facts.
- Do not introduce new people, places, or world rules.
- Do not expose internal KB evidence in storyboard rows.
