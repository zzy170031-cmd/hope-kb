# hope-kb 来源索引 v0.1

这份索引只列 **v0.1 首批会直接用到** 的外部来源。  
目标不是做全量资料库，而是给 Track C 一个稳定、可追溯、可复核的起点。

---

## A. 规则层来源

### 1. Storyboard / Animation 基础

- Adobe: Animation Storyboarding  
  用途：
  - 支撑 storyboard 是“脚本到成片的视觉桥梁”这一规则层定义
  - 支撑 `Cuts`、`NarrativeScenes`、视觉桥接这类结构字段
  链接：
  [Adobe Animation Storyboarding](https://www.adobe.com/creativecloud/animation/discover/animation-storyboarding.html)

- Adobe: 12 Principles of Animation  
  用途：
  - 支撑动势、节奏、夸张、重量感、anticipation、staging 等视觉语言归纳
  - 支撑 `visual_language_terms` 的规则解释层
  链接：
  [Adobe 12 Principles of Animation](https://www.adobe.com/creativecloud/animation/discover/principles-of-animation.html)

- SVA: Storyboarding for Animation  
  用途：
  - 支撑 storyboard 里 `narrative / dramatic beats / cinematography / blocking / panels / layers / camera moves / transitions / audio`
  - 支撑 `storyboard_cuts` 字段不属于“过度设计”
  链接：
  [SVA Storyboarding for Animation](https://qa.sva.edu/academics/continuing-education/animation/courses/storyboarding-for-animation-26-cs-anc-1024-ol)

### 2. 结构化输出与模型规则

- Alibaba Cloud: Qwen Structured Output  
  用途：
  - 冻结 `Qwen-compatible` 主路线的结构化输出纪律
  - 明确 JSON 关键词、`response_format`、thinking mode 限制
  链接：
  [Qwen Structured Output](https://www.alibabacloud.com/help/en/model-studio/qwen-structured-output)

- Alibaba Cloud: Qwen JSON Mode / JSON Schema Mode  
  用途：
  - 支撑 Writer / Storyboard 落库层必须优先使用结构化 JSON 输出
  链接：
  [Qwen JSON Mode](https://www.alibabacloud.com/help/en/model-studio/json-mode)

---

## B. 下游目标环境来源

### 3. 即梦

- 即梦 AI 官方入口  
  用途：
  - 证明中文提示词是其主创作入口之一
  - 支撑 v0.1 外部验证优先考虑中文原生模型
  链接：
  [即梦 AI](https://jimeng.jianying.com/)

### 4. 可灵

- Kling VIDEO 3.0 用户指南  
  用途：
  - 支撑多镜头、多角色一致性、元素锁定、多语言支持等外部验证方向
  - 支撑 `jimeng / kling` 作为 v0.1 首选下游验证目标
  链接：
  [Kling VIDEO 3.0 Guide](https://app.klingai.com/cn/quickstart/klingai-video-3-model-user-guide)

---

## C. 团队归纳层

这部分不是官方规则，而是我们自己的风格摘要层。  
需要单独标注为：

- `source_type = team_distillation`

首批适用表：

- `director_profiles`
- `director_reference_sets`
- `director_scene_affinity`
- `committee_handoff_rules`
- `committee_style_merge_rules`

录入要求：

- 每条记录都要有 `source_notes`
- 必须说明是从哪些作品或公开访谈中归纳的
- 必须标注 `confidence_level`

---

## D. v0.1 优先使用顺序

1. 先用规则层来源，建立字段和约束
2. 再用下游目标环境来源，确定中文 prompt 与验证目标
3. 最后录入团队归纳层，用于导演风格和 committee 逻辑

这意味着：

- 先录 `continuity_rules / camera_work_vocabulary / story_structure_templates`
- 再录 `director_profiles / director_scene_affinity / handoff rules`

---

## E. 当前不进入来源索引的内容

以下内容不作为 v0.1 首批主来源：

- 无出处 Prompt 合集
- 论坛主观印象帖
- 短视频口播总结
- 无法映射到 schema 字段的长篇主观分析
