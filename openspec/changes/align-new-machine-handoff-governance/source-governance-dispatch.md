# Source Governance Dispatch

Dispatch date: 2026-04-30

Total-control thread: `(running) HopePrompt KB - v0.2 total-control`

Target branch lane: `(running) HopePrompt KB - source acquisition and knowledge governance`

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate: Source acquisition / SourceDeltaBatch / source quality scoring docs-only + validator-design
target thread: （运行）HopePrompt知识库-【来源入库与知识治理】
thread action: reuse
objective:
Ask Lane 1 to produce the next design draft for Source acquisition boundaries, SourceDeltaBatch descriptor fields, SourceDeltaBatch pass/fail fixture design, source quality scoring rubric, denied/sanitized/aggregate-only rules, and the minimum later Rust-first validator implementation gate.
repository path: E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD anchor: codex/contracts-freeze / 9391519
live git checks required:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
- AGENTS.md
- docs/live-progress.md
- docs/prompt-knowledge-core-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- openspec/changes/align-new-machine-handoff-governance/branch-report-intake.md
- openspec/changes/align-new-machine-handoff-governance/dispatch-review-packet.md
- openspec/changes/align-new-machine-handoff-governance/specs/openspec-control-supervisor/spec.md
write scope: none
allowed actions:
- read-only Git checks
- read-only document review
- design-draft synthesis
- propose Source acquisition boundaries
- propose SourceDeltaBatch descriptor fields
- propose SourceDeltaBatch pass/fail fixture matrix
- propose source quality scoring rubric
- propose sanitized diagnostics and aggregate-only rules
- propose the smallest later Rust-first validator implementation gate
prohibited actions:
- no file edits
- no cleanup
- no staging
- no commit
- no push
- no runtime changes
- no UI changes
- no seed changes
- no fixture/export changes
- no runtime network fetch
- no automatic ingest
- no provenance_locator, url_or_path, source_register path, or raw source text in runtime payload/log/UI/telemetry
- no raw KB rows, raw prompt bodies, full source registers, overlay JSON, raw graph payloads, local path details, secrets, tokens, provider config, or request/response bodies
required source documents:
- Lane 1 intake in branch-report-intake.md
- source governance contract
- freshness activation contract
- descriptor validator matrix
- OpenSpec supervisor spec
validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
stop conditions:
- live Git is not codex/contracts-freeze / 9391519
- target path is not E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
- design attempts runtime fetch, automatic ingest, raw source text exposure, or per-source locator runtime exposure
- design requires Rust implementation or fixture creation in this gate
expected report format:
single copy-ready text block with the required total-control fields plus the six requested design sections
key-node reminder:
Refresh thread label, anchor commit, worktree state, and boundary statement before the next package starts.
```

## 2. OpenSpec Review

```text
openspec dispatch review:
review outcome: approved
gate alignment: aligned with branch-report-intake recommendation and the prior handoff priority
target lane alignment: aligned with Lane 1 source acquisition and knowledge governance responsibilities
scope boundary: docs-only + validator-design; no implementation or fixture creation
global rule anchor check: aligned with E:\codex\AGENTS.md and E:\codex\hope-kb\docs\codex-usage-core-rules.md at 9391519
Core Challenger check:
The dispatch does not claim implementation readiness. It asks for design evidence and keeps full SourceDeltaBatch Rust rules, fixtures, source quality activation, runtime fetch, and auto-ingest deferred.
Audit Specialist boundary check:
Read-only. No cleanup, deletion, staging, commit, push, runtime/UI/seed/fixture/export changes, or raw source/raw KB disclosure.
safety boundary check:
Explicitly prohibits provenance locator, url/path, source register path, raw source text, raw KB rows, raw prompt bodies, overlay JSON, raw graph payloads, secrets, tokens, provider config, and request/response body disclosure.
validation command check:
Startup Git checks are required; cargo is not required for this design-only dispatch.
report format check:
Requires one copy-ready text block and the project standard fields, plus six design sections.
missing evidence: none for a design-only dispatch
required corrections: none
blocking reason: none
guided next action:
Total-control may send the OpenSpec-guided dispatch below to Lane 1.
```

## 3. OpenSpec-Guided Dispatch

```text
branch-thread instruction:
thread action: reuse
target thread: （运行）HopePrompt知识库-【来源入库与知识治理】
gate: Source acquisition / SourceDeltaBatch / source quality scoring docs-only + validator-design
objective:
基于已完成的只读审查，推进下一阶段 docs-only + validator-design 草案。请产出可供总控审核的设计回报，不直接改文件。

repository path:
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb

branch / HEAD anchor:
codex/contracts-freeze / 9391519

startup checks:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

read scope:
- AGENTS.md
- docs/live-progress.md
- docs/prompt-knowledge-core-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- openspec/changes/align-new-machine-handoff-governance/branch-report-intake.md
- openspec/changes/align-new-machine-handoff-governance/dispatch-review-packet.md
- openspec/changes/align-new-machine-handoff-governance/specs/openspec-control-supervisor/spec.md
- openspec/changes/align-new-machine-handoff-governance/source-governance-dispatch.md

write scope:
无。本轮只读，不改文件。

allowed actions:
- 只读核验 live Git
- 只读阅读指定文档
- 汇总设计草案
- 列出 Source acquisition 边界
- 设计 SourceDeltaBatch descriptor 字段
- 设计 SourceDeltaBatch pass/fail fixture 矩阵
- 设计 source quality scoring rubric
- 设计 sanitized diagnostics 和 aggregate-only 规则
- 给出后续 Rust-first validator implementation gate 的最小范围

prohibited actions:
- No file edits.
- No cleanup.
- No staging.
- No commit.
- No push.
- No runtime changes.
- No UI changes.
- No seed changes.
- No fixture/export changes.
- 不得让 runtime 联网抓取。
- 不得自动入库。
- 不得输出 provenance_locator、url_or_path、source_register path、raw source text。
- 不得泄漏 raw KB rows、raw prompt_body、full source_register、overlay JSON、raw graph、本地路径细节、API key、token、secret、provider config、request/response body。

required inputs:
- Lane 1 已回报的 source governance intake
- OpenSpec branch-report-intake.md 中 Lane 1 Intake
- 当前 source governance / freshness / descriptor matrix 合同文档

validation commands:
本轮只读设计，可不运行 cargo。
必须运行或复述启动核验：
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

stop conditions:
- 如果 live Git 不是 codex/contracts-freeze / 9391519，先报告冲突，不继续设计。
- 如果目标路径不是 E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb，先报告路径冲突。
- 如果设计会要求 runtime fetch、auto ingest、raw source text、per-source locator 进入 runtime payload/log/UI/telemetry，立即停止并报告越界。
- 如果需要实现 Rust rules 或新增 fixtures，先标为“后续 implementation gate”，本轮不得执行。

report format:
必须只用一个 text 代码块回报，字段如下：

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

本轮必须额外包含：
Source acquisition 边界草案：
SourceDeltaBatch descriptor 字段草案：
SourceDeltaBatch fixture 矩阵草案：
source quality scoring rubric 草案：
sanitized diagnostics / aggregate-only 规则：
后续 Rust-first validator implementation gate 最小范围：

key-node reminder:
关键节点提醒：请刷新线程标签、锚点提交、工作树状态和边界说明。
```
