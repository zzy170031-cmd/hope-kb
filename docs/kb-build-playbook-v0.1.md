# hope-kb 构建手册 v0.1

这份手册回答两件事：

- `hope-kb` 应该怎么构建
- 首批知识内容应该从哪里来

它不替代正式 schema，也不替代最终产品方案；它只负责把 Track C 的知识录入工作变成一条可执行流水线。

---

## 1. 构建目标

v0.1 的 `hope-kb` 不是百科全书，也不是自动抓取仓库。

它的目标只有三个：

1. 支撑 `45s clip / 10min single episode / 60min multi-episode` 三个 benchmark
2. 为 `Qwen-compatible` 主路线提供稳定的中文结构化输入资产
3. 为 `jimeng / kling` 的外部消费提供稳定、可复核的中文提示词语料

---

## 2. 知识库分层

### A. 规则层

回答“这类内容为什么应该这样组织”。

适用表：

- `camera_work_vocabulary`
- `continuity_rules`
- `story_structure_templates`
- `manga_structure_rules`
- `prompt_templates`

主要来源：

- 官方课程页
- 官方教学资料
- 官方模型文档

### B. 风格层

回答“这个导演 / committee / handoff 经验为什么这么归纳”。

适用表：

- `director_profiles`
- `director_scene_affinity`
- `committee_handoff_rules`
- `committee_style_merge_rules`

主要来源：

- 官方作品资料
- 官方访谈
- 团队内部归纳

### C. 样例层

回答“把规则和风格放进实际记录后长什么样”。

适用内容：

- 导演 few-shot
- benchmark 参考 cut
- 导出模板例子

---

## 3. 来源从哪里来

### 第一优先：官方规则来源

优先录入这些来源归纳出来的内容：

- Adobe Animation Storyboarding
- Adobe 12 Principles of Animation
- SVA Storyboarding for Animation
- 阿里云 Qwen Structured Output
- 阿里云 Qwen JSON Mode

这些来源优先服务：

- 术语表
- 结构模板
- 结构化输出纪律

### 第二优先：官方产品来源

优先录入这些来源归纳出来的下游适配信息：

- 即梦官方入口
- Kling 官方指南

这些来源优先服务：

- `target_model_family` 默认优先级
- 中文 PromptRenderer 的消费环境
- benchmark 外部验证策略

### 第三优先：团队归纳来源

只在以下场景使用：

- 导演风格摘要
- role affinity
- handoff 经验
- negative defaults

要求：

- 必须标为 `team_distillation`
- 必须带 `source_notes`
- 必须带 `confidence_level`

---

## 4. 构建步骤

### Step 1：先冻结字段模板

在收资料之前，先确认：

- 这类内容进哪张表
- 每条记录的必填字段是什么
- 中文 token 字段怎么命名

没有字段模板，不进入采集。

### Step 2：做来源登记

每次开始一批录入前，先登记来源到：

- `docs/kb-source-index-v0.1.md`
- `seed/v0.1/source_register.json`

### Step 3：按表提炼内容

要求：

- 一条来源对应一批可字段化的内容
- 不接受“只形成散文总结，不知道写进哪列”

### Step 4：落成 seed

落地到：

- `seed/v0.1/*.json`

规则：

- 使用稳定 `machine_id`
- 中文主字段
- 不追加语言后缀

### Step 5：进入复核

至少检查：

- schema 完整性
- 中文 token 规范
- 来源是否已登记
- benchmark 是否真的会用到

---

## 5. v0.1 首批录入顺序

### Day 1：规则层优先

1. `committee_role_definitions`
2. `prompt_templates`
3. `camera_work_vocabulary`
4. `continuity_rules`
5. `transition_vocabulary`
6. `story_structure_templates`
7. `manga_structure_rules`

### Day 2：风格层最小可用集

8. `director_profiles`
9. `director_scene_affinity`
10. `committee_handoff_rules`
11. `committee_style_merge_rules`

### Day 3：样例层

12. 导演 few-shot
13. benchmark 参考 cut
14. `export_templates`

---

## 6. 完成标准

一批知识内容只有同时满足下面 5 条，才算进入 v0.1 首批 seed：

1. 能映射到已冻结字段
2. 来源已登记
3. 中文 token 规范通过
4. 至少能服务一个 benchmark
5. 能被 Track D / E / G 直接消费

---

## 7. 当前不做的事情

v0.1 不在 `hope-kb` 里优先做：

- 大规模导演百科
- 全量多模型 prompt 方言适配
- 大规模论坛经验收集
- 没有出处的 Prompt 合集清洗
- 自动爬取和自动归纳整站资料

结论：

`hope-kb` v0.1 先做“可执行、可追溯、能支撑 benchmark 的知识底座”，而不是“看起来很全”的资料仓。
