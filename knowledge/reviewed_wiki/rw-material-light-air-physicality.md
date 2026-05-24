# rw-material-light-air-physicality

wiki_type: shot_language_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

把材质、光影、空气介质和环境反馈加入分镜提示词，使画面更有物理真实感。

## Source Basis

Manual product-side intake summary and local image batch were used only as summarized evidence. Runtime output must not include source images, raw article text, OCR text, local paths, hashes, or source registers.

## Applies To

- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- 材质描述应落到重量、阻力、湿度、粗糙、磨损、反光或受力反馈。
- 光影与空气介质只在场景逻辑支持时加入，例如侧逆光、轮廓光、雨雾、尘埃、烟气、水汽和湿地反光。
- 环境反馈必须绑定当前人物动作或物体状态，不能凭空扩展世界观。
- 该规则主要改善 visual_description、character_action、prompt_text、negative_constraints 和 kb_context_summary。

## Runtime Mapping

- director_rule_packs: `dg-material-light-air-physicality`
- selected_kb_rules: `rule:material-light-air-physicality`
- negative_constraints: physical detail must stay scene-bound
- summary_fragment: `summary:material-light-air-physicality`

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

- Do not add material, light, weather, smoke, dust, or fire effects unsupported by the current scene.
- Do not replace action with texture lists.
- Do not expose source images or raw article text.
