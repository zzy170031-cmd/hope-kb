# rw-writing-group-scene-rewrite-profile

wiki_type: writing_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Writing group scene rewrite profile: Rewrite accepted story material into scene-specific objective, conflict, relationship, visible beats, and filmable action.

## Source Basis

This entry was created from a confirmed manual intake package. Audit candidates were used only as summary evidence. Runtime output must not include audit images, audit text, hidden prompt templates, local-only identifiers, audit registries, or unpublished KB detail.

## Applies To

- create_story_task
- generate_storyboard
- validate_result

## Claims Summary

- Rewrite body material around the selected scene objective, obstacle, relationship state, and turning point.
- Expose visible behavior, spatial anchors, and action cause-effect so director rules have filmable material.
- Preserve accepted facts while changing the scene skeleton when the scene type changes.

## Runtime Mapping

- writing_rule_packs: wg-writing-group-scene-rewrite-profile
- selected_kb_rules: rule:writing-group-scene-rewrite-profile
- runtime_targets: writing_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary

## PWA Fields Served

- visual_description
- character_action
- prompt_text

## Negative Constraints

- Do not leave old body structure in place after changing scene type.
- Do not add unaccepted facts or audit material wording to fill a scene profile.
