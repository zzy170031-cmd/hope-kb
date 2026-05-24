# rw-expression-physicalization

wiki_type: scene_expression_rule
review_status: reviewed
effective_confidence: medium
runtime_eligible: true
leakage_count: 0

## Purpose

把抽象情绪翻译成可见的面部肌肉、身体姿态、呼吸节奏和表演动作。

## Source Basis

Manual product-side intake summary and local image batch were used only as summarized evidence. Runtime output must not include source images, raw article text, OCR text, local paths, hashes, or source registers.

## Applies To

- create_story_task
- generate_storyboard
- repair_storyboard
- validate_result

## Claims Summary

- 情绪词不能单独进入 character_action，应转成镜头可见的表演线索。
- 近景可使用下颌紧绷、咬肌凸起、瞳孔锁定、鼻翼扩张、嘴唇压紧或呼吸停滞等细节。
- 中远景应使用姿态、轮廓、blocking、手部迟疑、肩背收紧和运动停顿表达情绪。
- 表演物理化必须受 shot_size 支持，不能在远景塞入不可见微表情。

## Runtime Mapping

- director_rule_packs: `dg-expression-physicalization`
- selected_kb_rules: `rule:expression-physicalization`
- negative_constraints: no abstract mood only
- summary_fragment: `summary:expression-physicalization`

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

- Do not output abstract emotion without visible action.
- Do not force close-up micro-expression cues into wide shots.
- Do not invent character psychology beyond accepted body.
