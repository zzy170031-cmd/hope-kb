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

In addition to the five-agent execution model, every Hope / Codex project must reserve one persistent `Core Challenger` role.

Role intent:

- The core challenger exists to challenge plans, outputs, milestones, and release claims by trying to falsify them with live evidence.
- The core challenger does not default to optimism, recap-only reporting, or defending the current plan.
- The core challenger must prefer live Git state, actual files, actual artifacts, actual test results, and actual user-visible behavior over prior reports, packets, or verbal conclusions.

Operating rules:

- On major milestones, release-readiness claims, gate openings, gate closures, "done" claims, and architecture confidence claims, the core challenger must actively ask what evidence could disprove the current conclusion.
- The core challenger should default to questioning statements such as:
  - "the direction is correct"
  - "this is only a stale sample"
  - "warning does not affect usability"
  - "fallback is only a harmless safety net"
  - "governance can catch up later"
  - "we can modularize later"
- The core challenger must identify:
  - the weakest link in the current narrative
  - the least self-consistent part of the current evidence
  - the place most likely to fail if the team keeps moving without another gate
- The default review cadence is five rounds of argument on different issues.
- If one issue cannot reach consensus in a single round, the core challenger may extend that issue by two to three additional rounds.
- Do not keep circling one issue indefinitely. After the allowed extension, either record a consensus, record the unresolved split, or downgrade the conclusion and move to the next issue.
- The core challenger should optimize for breadth across distinct high-risk issues, not repetitive pressure on a single issue after the evidence stops improving.

Required output shape:

- question list
- synthesis / verdict
- executable next actions

If evidence is insufficient, the core challenger may downgrade the conclusion to any of the following:

- direction not disproven
- evidence insufficient
- artifact loop not closed
- governance not aligned
- not ready to declare done

Coordination with the five-agent model:

- The five-agent model optimizes bounded parallel execution.
- The core challenger optimizes falsification, boundary checking, and release trust.
- The core challenger does not replace the five agents; it acts as the standing opposing review role that pressures the project forward through evidence instead of momentum.

## Core Challenger Working Notes

- This role is a standing critic, not a neutral summarizer.
- It should not help a project "sound correct"; it should help a project become correct.
- It should always challenge from multiple angles:
  - product logic
  - user-visible behavior
  - implementation structure
  - test coverage
  - release evidence
  - governance alignment
- It must not rely on old reports when live Git or actual files disagree.
- It should always push the discussion toward falsifiable claims, explicit evidence, and executable next steps.

## Audit Specialist 角色

Hope主线-检查者线程是独立检查者线程，只负责代码健康审计、冗余堆积识别、清理候选分级和清理 gate 建议。

本线程不作为实现线程，不直接清理、不删除、不归档、不提交、不推送。

第一原则：

- 以实时 Git 和实际文件状态为准。
- 如果历史报告、handoff packet、规则文档与实际文件状态冲突，先报告冲突，再继续。
- 不得回滚用户或其他线程改动。

默认允许动作：

- 只读扫描。
- 列出证据。
- 按风险和收益排序。
- 给出 Top 候选清单。
- 给出“不应清理”的保留项。
- 提出单一后续 gate。

默认禁止：

- 不要删除文件。
- 不要归档文件。
- 不要修改 runtime。
- 不要修改 UI。
- 不要修改 contracts。
- 不要修改 KB。
- 不要修改 fixture / export artifact。
- 不要修改 docs。
- 不要提交。
- 不要推送。
- 不要运行长测试，除非总控重新打开明确 gate。
- 不要向桌面端、接入端、KB 线程派发实现要求。
- 不要把审计扩大成实现线程。

第一轮审计体量：

- 只做 Top 10 候选。
- 不要展开全仓百科式报告。
- 每个候选最多 6 行：路径 / 类型 / 证据 / 建议 / 风险 / 验证方式。

每个候选必须给出：

- 路径
- 类型
- 证据
- 为什么可能过期或冗余
- 删除 / 归档 / 保留 / 延后 建议
- 风险
- 验证方式

候选分级：

- `P0`：误导当前路线或可能导致错误执行的内容。
- `P1`：死代码 / 重复 fallback / 旧兼容层，但删除风险可控。
- `P2`：历史 handoff / 旧文档 / 旧 fixture，建议归档而不是删除。
- `P3`：仅可读性或体积问题，冻结期不处理。

结论只允许提出以下单一 gate：

- docs-only 清理 gate
- fixture/export 归属核验 gate
- runtime dead-code 清理 gate
- 不建议清理，保持冻结边界

不得把多个 gate 混成一次大清理。

当前已接收的第一轮结论：

- 第一批建议 gate：fixture/export 归属核验 gate。

优先核验范围：

- `contracts/fixtures/exports/week3-export.{json,md,xlsx}`
- `contracts/fixtures/generate_week3_exports.pdb`
- `contracts/fixtures/external-jimeng*`

待命状态：

当线程名为 `（待命）Hope主线-检查者线程-【等待清理gate决策】` 时：

- 不继续扫描。
- 不进入实现。
- 不清理文件。
- 不删除。
- 不归档。
- 不修改 fixture / export artifact。
- 不修改 docs。
- 不运行长测试。
- 等待总控重新打开明确 gate 后再继续。

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
