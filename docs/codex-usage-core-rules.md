# Codex 全局协作规则锚点

本文档是 Hope / Codex 协作规则在 Git 中的项目可见锚点。

本机全局源规则仍以 `E:\codex\AGENTS.md` 为准；本文件用于把关键协作规则、线程角色和同步策略放入 Git，方便新线程、检查线程和后续项目仓读取。

## 放置与同步策略

- 全局源规则：`E:\codex\AGENTS.md`
- Git 锚点：`hope-kb/docs/codex-usage-core-rules.md`
- 其他 Hope 仓可在项目内建立同名 docs-only 锚点，引用本文件或同步必要摘要。
- 全局规则变更必须作为 docs-only gate 处理，不得混入 runtime、UI、KB seed、fixture、导出物、打包配置或安装包改动。
- 如果目标仓 dirty 且 dirty 归属未确认，只记录“待同步”，不要为同步规则回滚或清理其他线程改动。
- 如果本文件与实时 Git / 实际文件状态冲突，先信实时 Git 和实际文件状态，再回报冲突。

## 线程动作声明规则

总控下发任何分线程指令时，必须显式声明线程动作。

copy-ready 指令块必须使用：

```text
分线程指令：
线程动作：<新开 / 沿用 / 重启 / 待命 / 结束 / 归档>
目标线程：<目标线程名>
```

动作定义：

- `新开`：没有合适旧线程，或旧上下文不应继承。
- `沿用`：现有线程健康、上下文清晰，继续追加下一步。
- `重启`：旧线程卡顿、过长、状态混乱，或已结束但同一工作面还要继续。
- `待命`：当前不能编辑，只等待上游 gate 或回报。
- `结束`：当前任务完成，不再追加内容。
- `归档`：线程可准备关闭或归档。

使用 `重启` 时，指令必须写清：

- 旧线程名
- 新线程名
- 继承哪些结论
- 哪些旧状态必须忽略
- 必须重新核验 live Git / 实际文件状态

## 小块推进规则

大文件和高风险工作面默认按“小块推进 / 小块验证 / allowlist stage”执行。

- 不允许默认整文件 patch 套入。
- 不允许默认使用机械大替换脚本。
- 每次只改一个功能块，例如结构定义、解析函数、调用点、回归测试、UI 布局、状态文案。
- 每块完成后立即做局部核验，例如 `rg` 锚点、`git diff --check -- <file>`、必要的定向测试。
- 全部小块稳定后，才跑完整验证。
- 提交必须 allowlist stage；禁止 `git add .`、`git add -u`、`git commit -a`。

## Core Challenger 角色

Hope / Codex 项目除五线程执行模型外，保留一个常驻 `Core Challenger` 角色。

角色意图：

- 用实时证据质疑计划、产出、里程碑和“已完成”结论。
- 不默认乐观，不只复盘，不替当前方案辩护。
- 优先信 live Git、实际文件、实际 artifact、实际测试结果和用户可见行为。

必做行为：

- 在重大 gate、发布就绪、任务完成、架构信心判断时，主动问“什么证据能推翻当前结论”。
- 找出当前叙事最薄弱的一环。
- 找出证据中最不自洽的地方。
- 找出如果继续推进最可能失败的点。
- 默认五轮质疑，覆盖不同高风险问题。
- 单个问题未达成共识时，只允许额外延展两到三轮；之后记录共识、记录分歧，或降级结论。

输出形状：

- question list
- synthesis / verdict
- executable next actions

可降级结论：

- direction not disproven
- evidence insufficient
- artifact loop not closed
- governance not aligned
- not ready to declare done

## Audit Specialist 角色

Hope / Codex 项目保留一个独立 `Audit Specialist` / 审计专员角色。

角色意图：

- 专门审计冗余、过期、重复、废弃、误导性的代码、文档、fixture、导出物、fallback、配置和流程说明。
- 从产品视角和工程师视角同时判断健康风险。
- 默认只读审计，不直接清理、不删除、不格式化、不提交、不推送。

审计范围：

- 产品冗余：用户不可见、不可验证、已不服务当前 V0 / packaging 路线的旧入口、旧文案、旧流程。
- 工程冗余：重复实现、废弃 fallback、死代码、过期测试、未使用 helper、历史兼容层。
- 文档冗余：旧路线、旧 handoff、误导性 progress board、与当前 Git 路线不一致的说明。
- 数据冗余：过期 fixture、旧导出样本、重复 seed、脏但未归属的样本文件。
- 健康风险：大型单文件膨胀、职责混杂、不可测试分支、内部控制文本泄漏、契约边界不清。

每个候选必须给出：

- 路径
- 类型
- 证据
- 为什么可能过期或冗余
- 删除 / 归档 / 保留 / 延后 建议
- 风险
- 验证方式

审计结论只能提出 gate，不能自行执行清理：

- docs-only 清理 gate
- fixture/export 归属核验 gate
- runtime dead-code 清理 gate
- 当前不建议清理，保持冻结边界

## 当前总控执行优先级

在 Hope V0 收口期间，规则同步不得打断正在运行的交付线。

优先顺序：

1. 正在运行的 runtime / desktop / UI / QA gate
2. 必要的只读审计或质疑
3. docs-only 规则同步
4. fixture/export 归属核验
5. runtime dead-code 或模块拆分

规则同步完成后，应回报：

- 已同步项目
- 待同步项目
- 跳过原因
- 修改文件
- 验证命令
- commit hash / push 结果
