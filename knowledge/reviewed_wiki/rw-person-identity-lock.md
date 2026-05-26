# rw-person-identity-lock

wiki_type: validation_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

人物身份锁规则增强: Keep the person field limited to accepted story characters or explicit role labels while blocking location, action, scene, camera, style, KB, and placeholder terms.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result
- export_result

## Claims Summary

- person must preserve only accepted-body characters or explicit role labels.
- Reject location, action fragment, scene term, camera term, style term, KB term, slash, dash, or empty placeholder in person.
- When the person field is uncertain, leave it empty or repair from accepted facts instead of inventing a new name.

## Runtime Mapping

- validation_rule_packs: vg-field-aware-entity
- selected_kb_rules: rule:field-aware-entity
- runtime_targets: validation_rule_packs, scene_mappings, selected_kb_rules, negative_constraints

## PWA Fields Served

- person
- prompt_text
- visual_description

## Negative Constraints

- Do not invent character names.
- Do not put location, action, camera, visual style, source, or KB terms into person.
- Do not expose governance wording in user exports.
