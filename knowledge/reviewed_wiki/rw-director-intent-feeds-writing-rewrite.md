# rw-director-intent-feeds-writing-rewrite

wiki_type: writing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Director intent feeds writing rewrite: Use director-side filmability gaps to request targeted body rewrite without opening new PWA fields.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- validate_result

## Claims Summary

- Request rewrite when subject, action chain, spatial anchor, gaze relation, or emotional externalization is missing.
- Keep repair requests bounded to accepted facts and current scene type.
- Return the repaired writing beat to storyboard planning instead of generating a second raw prompt.

## Runtime Mapping

- writing_rule_packs: wg-director-intent-feeds-writing-rewrite
- selected_kb_rules: rule:director-intent-feeds-writing-rewrite
- runtime_targets: writing_rule_packs, director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not treat director intent as a free style switch.
- Do not add real director, studio, or IP names to runtime style controls.
