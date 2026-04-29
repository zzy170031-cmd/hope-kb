# Source To Wiki Minimal Loop Dispatch

Dispatch date: 2026-04-30

Total-control thread: `(running) HopePrompt KB - v0.2 total-control`

Target branch lane: `(running) HopePrompt KB - source acquisition and knowledge governance`

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate: offline source acquisition to reviewed wiki minimal loop design
target thread: (running) HopePrompt KB - source acquisition and knowledge governance
thread action: reuse
objective:
Design the smallest real-but-offline governance loop that connects controlled source acquisition, wiki draft, reviewed wiki, and SourceDeltaBatch descriptor evidence. The design should align the Karpathy LLM Wiki pattern with HopePrompt boundaries without implementing storage, fetching, runtime, seed, snapshot, UI, export, or validator changes.
repository path: E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD anchor: codex/contracts-freeze / fe46da5
live git checks required:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
- docs/prompt-knowledge-core-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
- openspec/config.yaml
- openspec/changes/align-new-machine-handoff-governance/source-delta-validator-implementation-intake.md
- tools/descriptor-validator/src/rules/source_delta_batch.rs
- tools/descriptor-validator/fixtures/pass/lane1_source_delta_batch/*.json
write scope:
- none
allowed actions:
- read-only analysis
- map Karpathy LLM Wiki ideas to HopePrompt-specific governance objects
- define the minimal source intake record shape
- define the minimal wiki draft and reviewed wiki record shape
- define review state transitions and stop conditions
- define how the reviewed output can produce a SourceDeltaBatch descriptor without leaking raw source text or locator values
- identify which future file locations or artifact directories should be proposed later
- identify the next implementation/design gate options
prohibited actions:
- no file edits
- no source fetching
- no source storage
- no source register reads
- no raw source text collection
- no wiki artifact creation
- no seed changes
- no snapshot changes
- no runtime changes or reads
- no UI changes
- no export changes
- no validator code changes
- no fixture creation or edits
- no OpenSpec edits
- no staging
- no commit
- no push
- no raw KB rows, raw prompt bodies, full source registers, overlay JSON, raw graph payloads, local path details, secrets, tokens, provider config, or request/response bodies
required source documents:
- Karpathy LLM Wiki pattern as external inspiration only
- HopePrompt v0.2 core/source/wiki/schema/snapshot contracts
- SourceDeltaBatch static validator intake and fixtures
validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- optional rg anchors for `raw`, `wiki`, `SourceDeltaBatch`, `reviewed_wiki`, `runtime`
stop conditions:
- target path is not the repository path above
- live Git is not codex/contracts-freeze / fe46da5 or successor clean fast-forward state
- tracked dirty files are present before analysis
- design requires implementation, file edits, raw source collection, source fetching, source storage, seed/snapshot/runtime/UI/export work, validator changes, or fixtures
- design would allow raw source text, locator values, source register paths, or per-source evidence into runtime payload/log/UI/telemetry
expected report format:
single copy-ready text block with required total-control fields plus:
- Karpathy pattern alignment:
- Minimal offline loop:
- Record shapes:
- Review gates:
- SourceDeltaBatch handoff:
- Runtime boundary:
- Next gate recommendation:
key-node reminder:
Refresh thread label, anchor commit, worktree state, and boundary statement before the next package starts.
```

## 2. OpenSpec Review

```text
openspec dispatch review:
review outcome: approved
gate alignment:
Aligned with the Core Challenger conclusion that the direction is not disproven but the artifact loop is not closed.
target lane alignment:
Aligned with Lane 1 because the question is source acquisition, source review, wiki governance, and SourceDeltaBatch handoff.
scope boundary:
Read-only design gate. No repository edits, source fetching, raw source storage, seed/snapshot/runtime/UI/export work, validator changes, or fixture work.
global rule anchor check:
Aligned with small-block verification, live Git truth, no user-change reverts, and total-control dispatch review discipline.
Core Challenger check:
The dispatch asks for falsifiable loop design and explicitly forbids claiming runtime or activation readiness.
Audit Specialist boundary check:
The dispatch prohibits raw values, source locators, local path details, matched values, credentials, provider config, and request/response bodies.
safety boundary check:
Keeps runtime consumption limited to verified activated snapshot products and aggregate source-delta evidence.
validation command check:
Requires startup Git checks and optional `rg` anchors only; no cargo or implementation validation is required for read-only design.
report format check:
Requires one copy-ready text block with total-control fields and loop-specific sections.
missing evidence: none for read-only design
required corrections: none
blocking reason: none
guided next action:
Total-control may send the guided read-only design dispatch below to Lane 1.
```

## 3. OpenSpec-Guided Dispatch

```text
分线线程指令：
线程动作：沿用
目标线程：（运行）HopePrompt知识库-【来源入库与知识治理】
gate：offline source acquisition to reviewed wiki minimal loop design

目标：
设计一个最小的、真实但离线的治理闭环，把 controlled source acquisition、wiki draft、reviewed wiki、SourceDeltaBatch descriptor evidence 串起来。这个 gate 只做只读设计，不改文件、不实现。

仓库路径：
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb

branch / HEAD anchor：
codex/contracts-freeze / fe46da5

启动检查：
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

读取范围：
- docs/prompt-knowledge-core-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
- openspec/config.yaml
- openspec/changes/align-new-machine-handoff-governance/source-delta-validator-implementation-intake.md
- tools/descriptor-validator/src/rules/source_delta_batch.rs
- tools/descriptor-validator/fixtures/pass/lane1_source_delta_batch/*.json

写入范围：
无。本轮只读，不改文件。

允许动作：
- 只读分析
- 对齐 Karpathy LLM Wiki 模式与 HopePrompt 边界
- 设计最小 source intake record
- 设计最小 wiki_draft / reviewed_wiki record
- 设计 review state transitions 和 stop conditions
- 设计 reviewed output 如何生成 SourceDeltaBatch descriptor evidence
- 指出未来可能的 artifact 目录或文件位置，但本轮不创建
- 给出下一 gate 建议

禁止动作：
- 不改文件
- 不抓取 source
- 不建立 source storage
- 不读取或导出 raw source text
- 不创建 wiki artifact
- 不改 seed
- 不改 snapshot
- 不碰 Hope runtime
- 不改 UI
- 不改 export
- 不改 validator code
- 不新增或修改 fixtures
- 不改 OpenSpec
- 不 stage
- 不 commit
- 不 push
- 不泄漏 raw KB rows、raw prompt body、full source register、overlay JSON、raw graph、本地路径细节、secret、token、provider config、request/response body

必须回答：
- Karpathy pattern alignment：我们借鉴什么，不借鉴什么
- Minimal offline loop：source acquisition -> wiki draft -> reviewed wiki -> SourceDeltaBatch descriptor 的最小闭环
- Record shapes：每层最小字段
- Review gates：人工 review、confidence、quarantine、reject、accepted_limited 的规则
- SourceDeltaBatch handoff：哪些字段能进入 descriptor，哪些只能保留在治理层
- Runtime boundary：为什么仍不能进入 Hope runtime
- Next gate recommendation：下一步应该 docs-only、artifact prototype、fixture、validator hardening，还是继续只读

验证命令：
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- 可选：rg -n "raw|wiki|SourceDeltaBatch|reviewed_wiki|runtime" 指定文档和 validator 文件

停止条件：
- 目标路径不是 E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
- live Git 不是 codex/contracts-freeze / fe46da5 或其干净 fast-forward 后继
- 分析前存在 tracked dirty
- 设计需要实现、改文件、抓取 source、保存 raw source、读 source register、seed/snapshot/runtime/UI/export、validator/fixture 改动
- 设计会允许 raw source text、locator values、source register path、per-source evidence 进入 runtime payload/log/UI/telemetry

回报格式：
必须只用一个 text 代码块：

目标线程：总控线程
来源线程：（运行）HopePrompt知识库-【来源入库与知识治理】
仓库路径：
branch / HEAD：
git status：
已完成：
未完成：
修改文件：
验证结果：
是否提交 / push：
风险 / 阻塞：
需要总控决策的问题：
下一步建议：

Karpathy pattern alignment：
Minimal offline loop：
Record shapes：
Review gates：
SourceDeltaBatch handoff：
Runtime boundary：
Next gate recommendation：

关键节点提醒：
请刷新线程标签、锚点提交、工作树状态和边界说明。
```
