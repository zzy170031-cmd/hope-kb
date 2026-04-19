# hope-kb 来源与采集规范 v0.1

这份文档定义：

- 从哪里拿知识
- 怎么判断来源是否可靠
- 录入时怎么标注“规则”与“经验”
- v0.1 先用哪些来源

---

## 1. 来源分层

### A. 权威规则层
用于录入：

- storyboard 基础构成
- 动画节奏与视觉原则
- 摄影术语
- 连续性规则
- 结构化输出规范

优先来源：

- 官方课程页
- 官方文档
- 官方教学内容

### B. 风格经验层
用于录入：

- 导演 profile
- committee 模板
- few-shot 样例
- negative defaults
- affinity / handoff 经验

优先来源：

- 官方作品资料
- 官方访谈
- 团队内部整理与复核

规则：

- 风格经验层必须明确标注为“团队归纳”
- 不得伪装成权威规则

---

## 2. v0.1 优先采用的公开来源

### Storyboard / Animation 规则层

- Adobe: animation storyboard 是脚本与成片之间的视觉桥梁  
  [Adobe Animation Storyboarding](https://www.adobe.com/creativecloud/animation/discover/animation-storyboarding.html)

- Adobe: 12 principles of animation 可作为视觉动势、节奏、夸张与重量感的底层参考  
  [Adobe 12 Principles of Animation](https://www.adobe.com/creativecloud/animation/discover/principles-of-animation.html)

- SVA: storyboard 课程明确覆盖 `narrative`, `dramatic beats`, `cinematography`, `blocking`, `performance`, `panels`, `layers`, `camera moves`, `transitions`, `audio`  
  [SVA Storyboarding for Animation](https://qa.sva.edu/academics/continuing-education/animation/courses/storyboarding-for-animation-26-cs-anc-1024-ol)

### 模型与结构化输出规则

- 阿里云 Model Studio: Qwen structured output  
  明确 `response_format`、JSON 关键词要求、thinking mode 不支持 structured output  
  [Qwen Structured Output](https://www.alibabacloud.com/help/en/model-studio/qwen-structured-output)

- 阿里云 Model Studio: JSON mode / JSON schema mode  
  [Qwen JSON Mode](https://www.alibabacloud.com/help/en/model-studio/json-mode)

### 下游工具目标环境

- Kling 官方公开信息：多语言、multi-shot、element reference、连续性能力  
  [Kling AI](https://klingai.com/)

---

## 3. 录入规范

### 所有记录必须至少标注一类来源

- `source_type`
  - `official_rule`
  - `official_product`
  - `team_distillation`
  - `internal_example`

- `source_notes`
  - 来源说明
  - 记录提炼理由

- `confidence_level`
  - `high`
  - `medium`
  - `low`

- `last_reviewed_at`
  - 最近一次人工复核日期

### 规则层记录

必须满足：

- 可复用
- 可泛化
- 不依赖某个导演私人口味

### 风格层记录

必须满足：

- 可以解释为什么这么归纳
- 至少能对应到代表性作品或样例
- 可以随着 v1 继续调整

---

## 4. 不接受的来源

以下内容不进入 v0.1 主知识库：

- 没有出处的 Prompt 合集
- 论坛口口相传的“某导演就是这种风格”
- 纯短视频口播总结
- 没有来源标注的二手表格
- 无法映射到 schema 字段的长篇散文总结

---

## 5. 采集流程

### 第一步：先字段化

在收资料前，先确定：

- 这类知识要进哪张表
- 每条记录的必填字段是什么

### 第二步：再收来源

只收能填进字段的内容。

### 第三步：最后做归纳

把资料整理成：

- 规则层记录
- 风格层记录
- few-shot 样例

### 第四步：进入复核

Track C 录入后，至少经过：

- schema 完整性检查
- 中文 token 规范检查
- benchmark 可用性检查

---

## 6. v0.1 知识库工作原则

- 先支持 benchmark，不先追求完整
- 先支持 `Qwen-compatible` 结构化输出，不先追求多模型最优适配
- 先支持 `jimeng / kling` 外部消费，不先追求所有下游模型
- 先让 Track C 可执行，不让它变成无限资料收集
