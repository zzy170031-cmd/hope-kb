# rw-seedance2-storyboard-field-discipline

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

Seedance2 storyboard field discipline: Turn Seedance-oriented camera, expression, scene, light, and composition guidance into existing PWA storyboard fields without schema drift.

## Source Basis

This entry was created from a confirmed manual intake package. Source candidates were used only as summary evidence. Runtime output must not include source images, OCR text, raw article text, local paths, prompt bodies, source registers, or raw KB rows.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- Choose one primary camera movement and one compatible viewpoint for the current beat before compiling prompt_text.
- Bind shot_size before expression detail: close shots may carry eye, lip, breath, and hand tension; wide shots should carry silhouette, spacing, route, and group motion.
- Build visual_description from scene-supported space, subject placement, material, light direction, color temperature, air medium, composition, and depth cues.
- Translate emotion into visible action such as gaze, breath, posture, hand motion, step, recoil, pause, or blocking relation.
- Use style_profile only after scene type, accepted facts, shot_size, and visible row purpose are already coherent.
- Compile prompt_text from person, camera, shot_size, visual_description, character_action, dialogue_or_narration, duration_seconds, and safe negative constraints only.

## Runtime Mapping

- director_rule_packs: dg-seedance2-storyboard-field-discipline
- selected_kb_rules: rule:seedance2-storyboard-field-discipline
- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules, kb_context_summary, negative_constraints

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

- Do not add anime-only camera vocabulary when the scene style is grounded, documentary, or otherwise not anime-compatible.
- Do not put camera, style, composition, source, or KB terms into person.
- Do not force tiny facial muscle detail into wide, aerial, group, or battlefield-scale shots.
- Do not use light, color, atmosphere, or composition as disconnected decoration.
- Do not let style_profile become a second prompt template or override existing final-field responsibilities.
- Do not expose source or governance metadata in PWA output.
