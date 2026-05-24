# rw-transition-motion-dynamics

wiki_type: director_scheduling_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

为镜头之间的节奏、转场、动势和情绪递进提供导演规则。

## Source Basis

Manual product-side intake summary and local image batch were used only as summarized evidence. Runtime output must not include source images, raw article text, OCR text, local paths, hashes, or source registers.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard

## Claims Summary

- 转场和动势应服务镜头功能、动作压力和情绪方向。
- 可选转场包括淡入淡出、叠化、闪白/闪黑、划像、推拉、匹配剪辑和蒙太奇。
- 可选动势包括水平、垂直、对角线、曲线、放射状、螺旋和 S 型动势。
- 动势规则应帮助 create_story_task 与 generate_storyboard 形成连续镜头，而不是替代剧情。

## Runtime Mapping

- director_rule_packs: `dg-transition-motion-dynamics`
- scene_mappings: all 21 PWA scene types
- selected_kb_rules: `rule:transition-motion-dynamics`
- summary_fragment: `summary:transition-motion-dynamics`

## PWA Fields Served

- visual_description
- character_action
- camera
- shot_size
- prompt_text
- scene_profile
- negative_constraints
- kb_context_summary
- selected_kb_rules

## Negative Constraints

- Do not add decorative transitions unrelated to scene function.
- Do not break action continuity or accepted event order.
- Do not let transitions replace visible action.
