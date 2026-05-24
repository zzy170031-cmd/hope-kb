---
reviewed_wiki_id: rw-preproduction-storyboard-guide
wiki_type: shot_language_rule
review_status: reviewed
runtime_eligible: true
leakage_count: 0
source_family: summarized-ai-video-preproduction-guide
---

# Preproduction Storyboard Guide

This reviewed wiki entry summarizes the new storyboard-board material into a
runtime-safe rule. It does not publish source images, raw prompt bodies, OCR
text, tutorial provenance, local paths, or graph payloads.

## Runtime Summary

AI video storyboard generation should behave like a compact director
preproduction guide. Before producing shot rows, the director layer should
anchor:

- project setup: theme, scene type, duration, shot count, style, emotion,
  palette, environment, subject identity, and shot narrative strategy
- character and style reference: front, back, side, close-up, relaxed/action
  states, clothing, accessories, material, and allowable micro-variation
- environment and scene design: dramatic location, spatial route, camera
  positions, scene props, light changes, and movement path
- storyboard rows: numbered shots with shot function, camera type, shot size,
  movement mode, action, emotion, dialogue/narration, sound, and duration
- lighting, mood, and style: light direction, time-of-day transition, texture,
  atmosphere, and color palette
- audio and tone: environment sound, music style, sound texture, and rhythm
- cinematography notes: lens feel, movement philosophy, post-production tone,
  grain, blur, contrast, and overall visual discipline

## Prompt Text Impact

When compiled into PWA prompt_text, this rule must improve only user-facing
shot output:

- camera should carry shot function, angle, movement, and focal feeling
- visual_description should carry visual master, space, material, light, air,
  and composition cues supported by the current task fragment
- character_action should translate emotion into visible performance
- prompt_text should include production-facing guidance for light, sound,
  motion, and post feel without exposing internal knowledge fields
- selected storyboard task duration and script fragment remain the hard
  boundary; never expand back to the full accepted body

## Runtime Mapping

- runtime_targets: director_rule_packs, scene_mappings, selected_kb_rules,
  kb_context_summary, negative_constraints
- runtime_rule_pack_ids: dg-preproduction-storyboard-guide
- selected_kb_rule_ids: director:preproduction-guide
- scene_type_ids: hot_blood_battle, urban_fantasy
- kb_actions: create_story_task, generate_storyboard, repair_storyboard
