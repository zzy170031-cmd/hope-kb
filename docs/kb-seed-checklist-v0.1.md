# hope-kb 首批知识录入清单 v0.1

这份清单服务于 `Hope` 的 v0.1 最小闭环，不追求百科全书式完整，只追求：

- 能支撑 `45s clip`
- 能支撑 `10min single episode`
- 能支撑 `60min multi-episode`
- 能支撑 `Qwen-compatible` 主路线下的结构化中文输出
- 能支撑 `jimeng / kling` 外部消费验证

## 使用原则

- 先录**规则层**，再录**风格层**
- 先录**benchmark 必需集**，再扩厚
- 所有 token 字段默认中文，不带语言后缀
- 所有记录都要带来源说明或内部归纳说明

---

## 当前 `2026-04-20` gate 补充

- `scene_taxonomy_aliases` 已经进入 seed bundle，用来把接入侧细粒度场景标签归并回 canonical taxonomy。
- `degraded_input_examples` 需要覆盖当前 8 个 baseline `failure_code`，并与 repair template、validator target、repair scope 保持一致。
- `classic_case_examples` 需要同时满足无占位符脏文本、导演/委员会引用合法，以及当前最小分布 `director_classic=3 / benchmark_reference=3 / director_few_shot=5 / committee_handoff=6 / failure_repair=4`。
- `validate-seed-bundle.ps1` 与 `build-kb-snapshot.py` 需要持续为绿，且 `snapshot_meta.content_hash` 必须与 `manifest.content_hash` 对齐。

## A. Day 1 必录

### 1. `director_profiles`
目标：先把 7 位导演的主字段录全。

首批导演：

- 今石洋之
- 荒木哲郎
- 朴性厚
- 新海诚
- 山田尚子
- 汤浅政明
- 今敏

每条记录最少字段：

- `id`
- `name`
- `category`
- `director_role_tags`
- `visual_style_tokens`
- `composition_traits`
- `character_shape_bias`
- `panel_rhythm_bias`
- `color_lighting_mood`
- `emotion_expression_style`
- `negative_prompt_defaults`
- `hard_lock_safe_fields`
- `handoff_compatibility`
- `signature_camera_works`
- `signature_transitions`
- `source_notes`
- `confidence_level`
- `last_reviewed_at`

Day 1 完成标准：

- 7 位导演主字段齐全
- 不要求每人都补满大量样例
- 先保证 schema 可用、字段含义统一

### 2. `director_committees`
目标：录入 v0.1 预设小组。

至少录：

- 动作冒险组
- 日常治愈组
- 悬疑心理组
- 史诗群像组

每条记录最少字段：

- `id`
- `name`
- `summary`
- `chief_director_id`
- `default_roles`
- `usage_notes`

### 3. `committee_role_definitions`
目标：冻结 7 个角色的定义。

必须录：

- `chief`
- `scene`
- `action`
- `emotion`
- `transition`
- `suspense`
- `comedy`

每条记录最少字段：

- `role_code`
- `display_name`
- `responsibility`
- `v0_1_activation_mode`

### 4. `prompt_templates`
目标：先录最小模板集。

v0.1 最少 5 条：

- 梗概 -> Story
- Story -> Screenplay
- Screenplay -> NarrativeScene / Dialogue
- RenderSegment 规划
- Cut / layout / render prompt 生成

每条记录最少字段：

- `id`
- `stage`
- `name`
- `body`
- `required_inputs`
- `expected_output_schema`
- `target_model_family`
- `is_structured_output`

### 5. `visual_language_terms`
目标：先录 20 条最常用术语。

每条记录最少字段：

- `id`
- `name`
- `definition`
- `prompt_token`
- `usage_rule`
- `example_usage`

首批建议优先录入：

- 动感线
- 速度线
- 残影
- 焦点线
- 大格冲击
- 连续小格
- 分格密度
- 留白转场
- 拟声词视觉化
- 对话气泡布局
- 情绪符号
- 压迫构图
- 俯视压制
- 低机位压迫
- 光晕高光
- 雨幕层次
- 逆光轮廓
- 爆点停格
- 静压镜头
- 追逐动势

### 6. `camera_work_vocabulary`
目标：录入 15 条中文标准摄影术语。

每条记录最少字段：

- `id`
- `name`
- `prompt_token`
- `aliases`
- `definition`
- `usage_rule`

首批术语：

- 左横摇
- 右横摇
- 上俯仰
- 下俯仰
- 推镜
- 拉镜
- 固定
- 跟镜
- 手持
- 变焦推近
- 变焦拉远
- 移动
- 升降
- 主观镜头
- 越肩镜头

### 7. `continuity_rules`
目标：先录 3 条核心规则。

必须录：

- 180 度线
- 屏幕方向一致
- 视线匹配

每条记录最少字段：

- `id`
- `name`
- `rule_summary`
- `validator_intent`
- `repair_hint`

---

## B. Day 2 必录

### 8. `director_scene_affinity`
目标：支撑 rule-based director assignment。

每条记录最少字段：

- `director_id`
- `scene_type`
- `affinity_score`
- `reason`

至少覆盖：

- 日常
- 抒情
- 战斗
- 追逐
- 悬疑
- 心理
- 转场
- 群像
- 爆发

### 9. `committee_handoff_rules`
目标：支撑 `project_handoff_zones` 自动生成。

每条记录最少字段：

- `id`
- `from_role`
- `to_role`
- `transition_type`
- `buffer_guidance`
- `continuity_notes`

### 10. `committee_style_merge_rules`
目标：定义 hard / soft 合并顺序。

每条记录最少字段：

- `id`
- `role_code`
- `precedence_order`
- `overridable_fields`
- `non_overridable_fields`

### 11. `transition_vocabulary`
目标：录入首批转场术语。

每条记录最少字段：

- `id`
- `name`
- `prompt_token`
- `definition`
- `usage_rule`

首批建议：

- 硬切
- 柔化
- 桥接镜头
- 风格过渡
- 白闪
- 余像衔接
- 视线接切
- 运动接切

### 12. `story_structure_templates`
目标：让 Writer 层按时长选择结构模板。

至少录：

- `45s–3min`
- `3–15min`
- `15–45min`

每条记录最少字段：

- `id`
- `duration_band`
- `structure_name`
- `beat_count_range`
- `hook_rule`
- `turn_rule`
- `ending_rule`

### 13. `manga_structure_rules`
目标：录入漫剧结构规则，而不是通用电影结构规则。

首批规则：

- 3 秒钩子开场
- 情绪点密度
- 悬念结尾
- 角色标签化
- 短台词 / 气泡约束

---

## C. Day 3 必录

### 14. 导演 few-shot 最小集
目标：先完成 benchmark 所需样例。

规则：

- 每位导演至少 1 条代表 `cut`
- 深度导演优先补到 3 条：
  - 今石洋之
  - 新海诚
  - 今敏

每条样例至少包含：

- `camera_work`
- `layout_prompt`
- `render_prompt`
- `why_it_matches_director`

### 15. `dialogue_style_rules`
目标：补齐对话风格边界。

至少录：

- 简短口语化
- 情绪内压
- 爆发式短句
- 悬疑留白

### 16. `export_templates`
目标：服务 Excel 导出而不是反向主导 schema。

至少录：

- `Project`
- `Episodes`
- `RenderSegments`
- `Cuts`
- `PromptPackage`
- `Validation`

---

## D. 来源优先级

### 规则层优先来源

1. 官方课程 / 官方教学资料
2. 官方模型文档
3. 官方动画制作流程说明

### 风格层优先来源

1. 官方作品介绍 / 官方访谈
2. 团队归纳的导演风格摘要
3. 内部 few-shot 样例

### 不建议作为 v0.1 主来源

- 无来源的二手整理
- 短视频口播总结
- 纯论坛主观印象
- 未标注出处的 Prompt 合集

---

## E. 录入顺序建议

### 先录规则

1. `committee_role_definitions`
2. `prompt_templates`
3. `camera_work_vocabulary`
4. `continuity_rules`
5. `transition_vocabulary`
6. `story_structure_templates`
7. `manga_structure_rules`

### 再录风格

8. `director_profiles`
9. `director_scene_affinity`
10. `committee_handoff_rules`
11. `committee_style_merge_rules`
12. 导演 few-shot

### 最后录导出

13. `export_templates`

---

## F. v0.1 不要提前做的内容

这些内容放到 v1 再扩：

- 大规模导演案例库
- 完整 overlay 策略
- 自动角色表生成知识库
- 多 target model 的全量渲染规则
- 大量反 AI 画面机械感规则
- 大量跨风格 handoff 样例库

---

## G. 完成标准

这份清单完成，不代表 `hope-kb` 全量完成；只代表：

- Track C 可以开工
- Track D / E 有稳定输入
- `45s / 10min / 60min` 三个 benchmark 有知识支撑
- Hope v0.1 不会在 Week 2 因“知识库没定义”而停住
