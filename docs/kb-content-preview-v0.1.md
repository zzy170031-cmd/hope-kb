# hope-kb 内容预览 v0.1

这份预览不是 schema 清单，也不是 seed 清单，而是面向审阅的“当前知识库里已经实际写入了什么”。

当前版本对应：

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- seed pack: `seed/v0.1`

---

## 1. 当前已写入的知识库模块

当前 `hope-kb` 已经实际写入以下内容：

1. 导演内核层
2. 导演规则层
3. 导演公开锚点层
4. 导演场景适配层
5. 场景分类层
6. 场景别名归一层
7. 委员会角色层
8. 委员会模板层
9. handoff 规则层
10. 风格合并规则层
11. 视觉语言术语层
12. 中文摄影术语层
13. 连续性规则层
14. 转场术语层
15. 故事结构模板层
16. 漫剧结构规则层
17. 角色弧线模式层
18. 对白风格规则层
19. Prompt 模板层
20. 导出模板层
21. 经典案例示例层
22. 失败模式库
23. degraded-input 回归样例层
24. 导入映射层

当前 `seed/v0.1/manifest.json` 中的记录数为：

- 7 条 `committee_role_definitions`
- 7 条 `director_profiles`
- 7 条 `director_rules`
- 7 条 `director_reference_sets`
- 7 条 `director_scene_affinity`
- 35 条 `director_cut_samples`
- 12 条 `scene_taxonomy`
- 18 条 `scene_taxonomy_aliases`
- 4 条 `committee_templates`
- 21 条 `committee_handoff_rules`
- 7 条 `committee_style_merge_rules`
- 4 条 `story_structure_templates`
- 5 条 `manga_structure_rules`
- 6 条 `character_arc_patterns`
- 5 条 `dialogue_style_rules`
- 20 条 `visual_language_terms`
- 15 条 `camera_terms`
- 3 条 `continuity_rules`
- 8 条 `transition_vocabulary`
- 18 条 `prompt_templates`
- 17 条 `export_templates`
- 24 条 `classic_case_examples`
- 8 条 `failure_patterns`
- 8 条 `degraded_input_examples`

这一轮补充后，导演样例已经从“每位导演 1 条代表 cut”扩到了“每位导演 5 条代表 cut”，导出模板也已经补齐到正式方案要求的 17 张 sheet，同时知识库还补进了 scene alias 归一、degraded-input 回归样例、4 条 failure/repair 修复案例、bundle 校验脚本和可直接生成 SQLite snapshot 的构建链。

---

## 2. 七位导演的核心内核

### 今石洋之

- 核心摘要：
  - 把角色意志和剧情爆点变成极端透视、高饱和撞色与瞬时爆发的视觉冲击。
- 叙事内核：
  - 蓄力、爆发、再升级，人物动机必须直接而炽热。
- 镜头内核：
  - 推镜、拉镜、斜向冲刺式构图，强调冲击点前后的压缩与释放。
- 表演内核：
  - 姿态、重心、喊声和动作线直接把情绪打出来。
- 节奏内核：
  - 前短后快，高潮段连续重击。

### 荒木哲郎

- 核心摘要：
  - 让危机感通过纵深空间、群像排布和持续压迫性的镜头推进不断累积。
- 叙事内核：
  - 信息推进紧而清楚，危机一步步逼近人物。
- 镜头内核：
  - 纵向通道、压缩长焦、前景遮挡、人群挤压。
- 表演内核：
  - 紧绷感和群体反应比单个夸张动作更重要。
- 节奏内核：
  - 持续向前挤压，越到关键点越短促。

### 朴性厚

- 核心摘要：
  - 把近身动作的受力、轴线和打击点做清楚。
- 叙事内核：
  - 动作链先于对白完成信息表达。
- 镜头内核：
  - 贴身近景和跟镜优先，动作轴线必须清晰。
- 表演内核：
  - 重心、扭转、停顿和反击时机清楚。
- 节奏内核：
  - 快，但每个打击点都要有明确节拍。

### 新海诚

- 核心摘要：
  - 先让景观和天气承载情绪，再把人物的迟疑、回望和心事收进镜头。
- 叙事内核：
  - 关键情绪靠环境、时刻和距离感托起来。
- 镜头内核：
  - 远景景观和近景人物交替，天空、窗框、列车都参与叙事。
- 表演内核：
  - 动作克制，真正波动藏在停顿和视线里。
- 节奏内核：
  - 情感高点允许放慢。

### 山田尚子

- 核心摘要：
  - 用低视点、局部动作和安静留白把轻微情绪变化放大成关系。
- 叙事内核：
  - 人物关系变化必须准确，靠细节而不是说明建立。
- 镜头内核：
  - 脚步、手指、桌下动作、停顿后的余韵。
- 表演内核：
  - 微表情和小动作比大段情绪外放更重要。
- 节奏内核：
  - 轻但不松，讲究停顿前后的读感。

### 汤浅政明

- 核心摘要：
  - 让情绪直接侵入空间、形体和时间感。
- 叙事内核：
  - 现实段落和主观感受层可以互相侵染。
- 镜头内核：
  - 镜头组织服务情绪，而不是端正写实。
- 表演内核：
  - 角色形体允许被情绪夸张拉伸。
- 节奏内核：
  - 忽快忽慢，用突然错位带人进入心理层。

### 今敏

- 核心摘要：
  - 把现实与幻觉、记忆与当下的边界做成几乎看不见的缝。
- 叙事内核：
  - 通过结构错位和匹配接切让观众自己意识到断层。
- 镜头内核：
  - 稳定镜头中的镜像、反射、匹配剪接和视点微偏。
- 表演内核：
  - 越正常越让错位刺眼。
- 节奏内核：
  - 前段稳定，转折精准。

---

## 3. 导演运行规则

知识库里已经为 7 位导演分别写入了 `director_rules`，每位导演都有：

- `visual_non_negotiables`
- `camera_rules`
- `dialogue_rules`
- `scene_rules`
- `transition_rules`
- `anti_patterns`

例如：

### 今石洋之规则摘要

- 不可妥协：
  - 高饱和撞色
  - 极端透视
  - 爆发性构图
- 镜头规则：
  - 高潮段优先推镜或拉镜
- 对白规则：
  - 爆发段不靠解释推进
- 反模式：
  - 连续静态闲聊
  - 灰暗写实压住爆发

### 今敏规则摘要

- 不可妥协：
  - 镜像反射
  - 现实错位
  - 匹配剪接
- 镜头规则：
  - 镜头可以稳，但切口必须精准
- 对白规则：
  - 减少直接解释
- 反模式：
  - 平铺直叙解释
  - 无铺垫超现实

---

## 4. 导演公开锚点

不是只写了风格，还给 7 位导演都补了 `director_reference_sets`，用于追溯风格来源。

例如：

- 今石洋之：
  - 《天元突破》《斩服少女》《普罗米亚》
  - 官方锚点：
    - `https://promare-movie.com/`
    - `https://promare-movie.com/about/`

- 山田尚子：
  - 《电影 声之形》《利兹与青鸟》《电影 K-ON!》
  - 官方锚点：
    - `https://www.kyotoanimation.co.jp/works/koeM/`
    - `https://www.kyotoanimation.co.jp/works/liz/`
    - `https://www.kyotoanimation.co.jp/works/k-onMovie/`

- 今敏：
  - 《PERFECT BLUE》《千年女優》《パプリカ》
  - 官方锚点：
    - `https://konstone.s-kon.net/`
    - `https://konstone.s-kon.net/modules/works/index.php?content_id=1`
    - `https://www.madhouse.co.jp/special/mhpf/mhpf_no035.html`

---

## 5. 导演场景适配

当前已经写入 `director_scene_affinity`，而且不再只是粗分值，还补了：

- `detailed_scene_affinity`
- `preferred_scene_types`
- `non_fit_scene_types`
- `primary_role_bias`
- `secondary_role_bias`
- `preferred_handoff_targets`
- `anti_dispatch_warning`
- `dispatch_hint`

适配维度包括：

- 战斗
- 追逐
- 群像
- 悬疑
- 日常
- 抒情
- 心理
- 转场
- 爆发

例如：

- 今石洋之：
  - 战斗 `5`
  - 追逐 `5`
  - 爆发 `5`
- 山田尚子：
  - 日常 `5`
  - 抒情 `4`
  - 心理 `4`
- 今敏：
  - 悬疑 `5`
  - 心理 `5`
  - 转场 `5`

这些字段现在不只是说明“谁适合什么”，还已经开始具备运行时价值：

- 可作为自动分派时的主角色倾向
- 可作为 handoff 时的优先接力对象
- 可作为 rule-based assignment 的反派工警告

这部分已经能直接支撑 rule-based assignment 的第一版，而且已经开始具备“适合什么 / 不适合什么 / 为什么这样派”的调度解释能力。

---

## 6. 场景分类层

当前已经新增 `scene_taxonomy`，把运行时常用的场景类别从散落的字符串约束收成一层正式知识。

首批已写入 12 类：

- 战斗开场
- 战斗高潮
- 追逐推进
- 群像集结
- 爆点揭示
- 关系停顿
- 微表演对白
- 梦境切层
- 悬疑线索
- 片尾收束
- 情绪景观
- 转场空镜

每条都包含：

- `definition`
- `default_duration_band`
- `typical_committee_roles`
- `default_handoff_out`
- `risk_flags`
- `continuity_priority`
- `prompt_focus`

这一层的作用是把 rule-based assignment、RenderSegment 规划和 handoff 规则都锚定到统一 taxonomy 上，减少不同模块各自发明场景名导致的漂移。

---

## 7. 委员会与控制层

### 角色定义

当前已写入 7 个角色：

- `chief`
- `scene`
- `action`
- `emotion`
- `transition`
- `suspense`
- `comedy`

每个角色都写了：

- `display_name`
- `responsibility`
- `v0_1_activation_mode`

### 委员会模板

已写入 4 套：

- 动作冒险组
- 日常治愈组
- 悬疑心理组
- 史诗群像组

每套都包含：

- `chief_director_id`
- `default_roles`
- `使用场景`
- `决策规则`

### Handoff 规则

已写入 12 条，例如：

- `chief -> scene`
- `scene -> action`
- `action -> scene`
- `scene -> emotion`
- `emotion -> transition`
- `transition -> scene`
- `scene -> comedy`
- `comedy -> scene`

每条都包含：

- `transition_type`
- `buffer_guidance`
- `continuity_notes`
- `before_cut_pattern`
- `after_cut_pattern`
- `buffer_cut_pattern`
- `applicable_director_pairs`

### 风格合并规则

已写入 7 条，明确：

- 每个角色的 `precedence_order`
- 哪些字段可覆盖
- 哪些字段绝不能覆盖

这部分已经不再是空口规则，而是正式 seed 内容。

同时，`committee_style_merge_rules` 这一层也已经补上了 `merge_note`、`source_type`、`source_notes`、`confidence_level` 和 `last_reviewed_at`，开始具备可追溯和可复核属性。

---

## 8. 视觉语言与摄影术语

### 视觉语言术语 20 条

每条都已经写入：

- `术语`
- `分类`
- `prompt_token`
- `定义`
- `usage_rule`
- `example_usage`

例如：

- 高反差
- 低饱和
- 逆光轮廓
- 前景遮挡
- 长焦压缩
- 环境反射
- 体积光
- 冷暖对照
- 对角线构图
- 留白
- 运动模糊
- 静帧定格

### 中文摄影术语 15 条

每条都已经写入：

- `prompt_token`
- `aliases`
- `定义`
- `usage_rule`

当前术语包括：

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

---

## 9. 连续性、转场、结构、对白

### 连续性规则

已写入 3 条：

- 人物服装连续
- 空间方位连续
- 时间光线连续

### 转场术语

已写入 8 条：

- 硬切
- 柔化
- 桥接镜头
- 风格过渡
- 白闪
- 余像衔接
- 视线接切
- 运动接切

这一层现在也已经补上来源说明和置信度，不再只是术语列表。

### 故事结构模板

已写入 4 条：

- `45s-3min`
- `3-15min`
- `15-30min`
- `30-45min`

### 漫剧结构规则

已写入 5 条：

- 3 秒钩子开场
- 情绪点密度
- 悬念结尾
- 角色标签化
- 短台词与气泡约束

### 角色弧线模式

已写入 6 条：

- 受压后觉醒
- 误解到理解
- 逃避到承担
- 冷感到连接
- 秩序到失衡
- 执念到放下

### 对白风格规则

已写入 5 条：

- 短句直给
- 口语留白
- 内压情绪
- 悬疑延后
- 反应式插科

`story_structure_templates` 与 `dialogue_style_rules` 也已经补上来源字段，后续可以继续往“规则层 / 经验层”更清晰地分层。

`0001_init_kb.sql` 也已经按当前 seed 字段重新对齐，不再停留在早期的薄 schema 状态。

---

## 10. Prompt 模板层

当前已经不是 5 条占位模板，而是 16 条真正覆盖链路和修复回合的模板：

1. 梗概 -> Story
2. Story -> Outline / Beats
3. Story -> Screenplay
4. Screenplay -> RenderSegments
5. RenderSegment -> Cuts
6. Cut -> Layout Prompt
7. Layout -> Render Prompt
8. Dialogue Refine
9. Repair Pass
10. Export Summary
11. Action Director Variant
12. Emotion Director Variant
13. Suspense Director Variant
14. Transition Director Variant
15. Repair Hard Locks
16. Repair Continuity

每条模板都已经有：

- `stage`
- `required_inputs`
- `body`
- `expected_output_schema`
- `target_model_family`
- `is_structured_output`

这部分已经能直接给 Track D / E 作为输入资产。

同时它已经不只覆盖主生成链，也开始覆盖：

- 导演化局部变体
- validator 失败后的 repair pass
- hard_locks 修复
- continuity 修复

并且现在 Repair 类模板已经和失败模式库建立了显式映射：

- `Repair Pass`
  - 对应：
    - 风格漂移
    - 角色不一致
    - 连续性断裂
    - RenderSegment 跨场景
    - 导演交接缺口
    - 中文 Prompt 表达失真
    - 导出 contract 漂移
- `Repair Hard Locks`
  - 对应：
    - 风格漂移
    - 角色不一致
    - Hard Locks 丢失
- `Repair Continuity`
  - 对应：
    - 连续性断裂
    - 导演交接缺口

---

## 11. 导出模板层

当前已写入 17 条导出模板，已经覆盖正式方案里的全部 Excel sheet：

- `Project`
- `Episodes`
- `RenderSegments`
- `Committee`
- `DirectorAssignments`
- `HandoffZones`
- `Synopsis`
- `WorldBible`
- `CharacterArcs`
- `StoryOutline`
- `Characters`
- `ShootingLocations`
- `NarrativeScenes`
- `Screenplay`
- `Cuts`
- `PromptPackage`
- `Validation`

每条都包含：

- `sheet_name`
- `显示标题`
- `用途`
- `核心字段`
- `列定义`

也就是说，这一层已经不再是“导出占位”，而是能直接支撑 17-sheet workbook contract 的知识资产。

---

## 12. 经典案例示例

当前已写入 24 条 `classic_case_examples`。

### benchmark 级

- `case_01`：45 秒追击钩子短片
- `case_02`：10 分钟初遇单集
- `case_03`：60 分钟多集群像项目

### 经典段落级

- `case_04`：镜中错位悬疑段
- `case_05`：桌下心动微表演段
- `case_06`：近身反击动作段

### handoff / 导演协作级

- `case_07`：Chief 到 Scene 的稳态接管段
- `case_08`：Scene 到 Action 的动作起手段
- `case_09`：Emotion 到 Transition 的余韵收束段
- `case_13`：Chief 到 Transition 的压场收束段
- `case_14`：Action 到 Emotion 的反冲缓释段

### 导演经典语汇级

- `case_10`：今石式群像集结爆发段
- `case_11`：新海式列车回望段
- `case_12`：今敏式媒介错位揭示段
- `case_15`：汤浅式梦境奔逃切层段
- `case_16`：荒木式压迫推进集结段
- `case_17`：朴式贴身追击断点段

### failure / repair 回归级

- `case_18`：双层 Hard Lock 回正修复段
- `case_19`：Chief/Transition 交接缺口回正段
- `case_20`：导出合同漂移回正段
- `case_21`：中文脏提示清洗回正段
- `case_22`：相邻 Cut 连续性断裂回正段
- `case_23`：跨场景 Segment 回切修复段
- `case_24`：Hard Locks 顺序漂移回正段

每条都带：

- `推荐委员会`
- `推荐导演组合`
- `结构提示`
- `验证价值`
- `source_type`
- `source_notes`

截至 `2026-04-20`，`RenderSegments / HandoffZones / Cuts / PromptPackage / Validation` 这 5 张 hope 主线程支持 sheet 已经把 `narrative_scene_id`、`continuity_notes`、`transition_out`、`negative_prompt`、`validation message` 等字段真实落成可导出列，不再只是停留在 `核心字段` 注释层。

同时，`validate-seed-bundle.ps1` 现在会把 `handoff / export / prompt` 这三类 hope 主线程支持流当成硬门槛，要求 failure pattern、degraded input、failure_repair 案例、repair template 输入/输出 contract 和 export sheet contract 彼此对齐；并且会继续检查 `negative-boundary` marker，确认 repair scope 没有悄悄越层。新增的 consumer-facing guardrail 还会检查 `RenderSegments / HandoffZones / Cuts / PromptPackage / Validation` 是否把关键 traceability 字段真实暴露成列，以及这些 sheet 仍然和 `prompt_04 / 05 / 07 / 15 / 16 / 17 / 18` 的 prompt contract 对齐。

## 13. 失败模式库

当前已写入 8 条系统级失败模式，覆盖：

- 风格漂移
- 角色外观不一致
- 连续性断裂
- RenderSegment 跨场景
- Hard Locks 丢失
- 导演交接缺口
- 中文 Prompt 表达失真
- 导出 contract 漂移

每条都包含：

- `symptom`
- `common_causes`
- `detection_hint`
- `repair_strategy`
- `affected_layers`
- `validator_hint`
- `repair_template_ids`
- `repair_priority`
- `repair_scope`
- `suggested_followup_validator`

这意味着 `hope-kb` 现在不仅描述“正确应该是什么”，也开始描述“错误通常长什么样、该怎么修”。

## 14. 导入映射层

当前已写入 `import_map.json`，用于把 seed 文件正式映射到 SQLite 表。

它已经定义：

- `file`
- `json_path`
- `table`
- `record_count_key`
- `primary_key_field`

同时还新增了：

- `scripts/validate-seed-bundle.ps1`

这条脚本可以直接校验：

- bundle_order 文件是否齐全
- manifest 计数是否与真实 JSON 一致
- machine_id 是否重复
- scene alias、failure->repair 映射、degraded-input 样例、classic case 样例完整性是否成立
- bundle hash 是否一致

---

## 15. 当前还没补满的地方

虽然现在已经不是空壳，而且这一轮已经把最关键的补深层做进去了，但如果要继续往“更厚、更强”补，优先级最高的现在变成：

1. 继续扩 `committee_handoff_rules` 的跨风格样例，尤其补 chief/transition 与 action/emotion 的更多细粒度边界变体
2. 把 `degraded_input_examples` 继续扩到 runtime-consumer / export / prompt rendering 的更细粒度回归样例，而不只是当前 baseline 8 条
3. 给 `classic_case_examples` 再补更细的 support-facing `failure_repair` 案例，尤其是 exporter / consumer payload 漂移
4. 给更多 build/runtime 文档同步 scene alias、degraded-input、consumer-facing export guard 和 snapshot 校验链
5. 在产品仓库里把 `scene_taxonomy`、failure->repair 映射和 validator / repair pass 真正接起来

---

## 16. 审阅结论

如果只问一个问题：

**当前 `hope-kb` 还是不是空壳？**

答案是：

**已经不是。**

它现在已经具备：

- 风格内核
- 运行规则
- 公开锚点
- 术语层
- 模板层
- 结构层
- 委员会控制层
- benchmark / handoff / 导演经典案例层
- failure / repair 回归层

也就是说，它已经从“知识库设计”进入了“真实知识资产”的阶段。
