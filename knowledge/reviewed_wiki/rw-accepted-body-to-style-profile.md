# rw-accepted-body-to-style-profile

wiki_type: writing_continuity_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Accepted body to style profile: Derive a compact style_profile from accepted body facts, scene type, tone, time period, world logic, and visible production need before any storyboard field receives style guidance.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Derive style_profile only from accepted body facts, scene type, tone, time period, world logic, and visible production need.
- Keep style_profile as a compact routing summary that guides downstream fields without adding unaccepted plot, person, IP, studio, creator, or audit facts.
- When accepted body does not support a style direction, keep style_profile neutral and let scene action, camera, continuity, and duration rules carry the row.

## Runtime Mapping

- writing_rule_packs: wg-accepted-body-to-style-profile
- selected_kb_rules: rule:accepted-body-to-style-profile
- runtime_targets: writing_rule_packs, director_rule_packs, validation_rule_packs, scene_mappings, kb_context_summary, negative_constraints

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

- Do not paste style alias catalogs or audit-only provenance into runtime fields.
- Do not let style_profile override accepted facts, character identity, scene type, duration, or row objective.
- Do not use real IP, real creators, studios, or blocked style categories as runtime style switches.
