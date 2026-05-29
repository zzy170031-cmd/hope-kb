# rw-visual-master-board-from-confirmed-rows

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Visual master board from confirmed rows: Summarize a visual master board from confirmed storyboard rows only so image prompt exports inherit character, environment, palette, lighting, material, and symbol continuity without inventing facts.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- export_result
- validate_result

## Claims Summary

- Use confirmed person, visual_description, character_action, camera, shot_size, prompt_text, duration, and safe constraints as the only source for the visual master board.
- Summarize recurring character anchors, environment logic, palette, lighting, material cues, spatial motifs, and visual symbols as guidance, not new facts.
- Keep visual master guidance compact and subordinate to the current row's shot purpose and accepted facts.

## Runtime Mapping

- director_rule_packs: dg-visual-master-board-from-confirmed-rows
- selected_kb_rules: rule:visual-master-board-from-confirmed-rows
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- person
- visual_description
- character_action
- camera
- shot_size
- prompt_text
- duration_seconds
- negative_constraints

## Negative Constraints

- Do not add characters, costumes, props, locations, palette rules, or symbols that are not supported by confirmed rows.
- Do not expose source article wording, complete case text, audit provenance, local paths, provider credentials, or hidden prompt templates.
- Do not turn the visual master board into a replacement for storyboard rows.
