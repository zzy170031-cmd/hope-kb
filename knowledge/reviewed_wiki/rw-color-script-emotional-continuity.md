# rw-color-script-emotional-continuity

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Color script emotional continuity: Use visible light source, color temperature, material response, air medium, and emotional color progression to support the current beat and adjacent rows.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Bind light direction, color temperature, material response, and air medium to the current beat and scene logic.
- Use color progression to clarify emotional or information change across adjacent rows.
- Route emotional tone through visible light, color temperature, material response, air medium, and rhythm only when accepted body and scene type support it.
- Keep visual_description and prompt_text consistent with the established visual world.

## Runtime Mapping

- director_rule_packs: dg-color-script-emotional-continuity
- selected_kb_rules: rule:color-script-emotional-continuity
- runtime_targets: director_rule_packs, selected_kb_rules, kb_context_summary, negative_constraints

## PWA Fields Served

- scene_profile
- visual_description
- character_action
- camera
- shot_size
- prompt_text
- duration_seconds
- negative_constraints
- kb_context_summary

## Negative Constraints

- Do not stack generic anime color words without a visible source or story function.
- Do not turn emotional tone into an unrelated style switch.
- Do not use real studio, IP, film, or director style names as runtime style switches.
- Do not add unsupported weather, light, or atmosphere to manufacture mood.
