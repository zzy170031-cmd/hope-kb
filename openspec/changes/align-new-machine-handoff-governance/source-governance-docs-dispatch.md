# Source Governance Docs-Only Dispatch

Dispatch date: 2026-04-30

Total-control thread: `(running) HopePrompt KB - v0.2 total-control`

Target branch lane: `(running) HopePrompt KB - source acquisition and knowledge governance`

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate: Source acquisition / SourceDeltaBatch / source quality scoring docs-only update
target thread: （运行）HopePrompt知识库-【来源入库与知识治理】
thread action: reuse
objective:
Update the approved docs-only source governance design into the repository docs, using the accepted Lane 1 design intake. Keep implementation, fixtures, runtime, seed, snapshot, UI, export, staging, commit, and push closed.
repository path: E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD anchor: codex/contracts-freeze / 9391519
live git checks required:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
- openspec/changes/align-new-machine-handoff-governance/source-governance-design-intake.md
- openspec/changes/align-new-machine-handoff-governance/source-governance-dispatch.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
write scope:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
allowed actions:
- small docs-only edits in the write scope
- add source acquisition boundary language
- add or refine source quality scoring rubric language
- add SourceDeltaBatch descriptor design refinements
- add SourceDeltaBatch future fixture matrix design notes
- add sanitized diagnostics and aggregate-only constraints
- run `git diff --check`
- run targeted `rg` checks
prohibited actions:
- no Rust code changes
- no fixture file creation or edits
- no seed changes
- no snapshot changes
- no runtime changes
- no UI changes
- no export changes
- no broad formatting churn
- no staging
- no commit
- no push
- no raw source text, per-source locator values, source register details, matched values, local path details, secrets, tokens, provider config, or request/response bodies
validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- rg -n "SourceDeltaBatch|source quality|aggregate-only|runtime" docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
stop conditions:
- target path is not the repository path above
- live Git is not codex/contracts-freeze / 9391519
- existing tracked files are dirty outside the allowed write scope
- edit requires docs/prompt-knowledge-freshness-activation-contract-v0.2.md or any other file; report first instead of editing
- edit requires Rust code, fixtures, seed, snapshot, runtime, UI, export, staging, commit, or push
expected report format:
single copy-ready text block with required total-control fields, changed files, validation results, risks, and next recommendation
key-node reminder:
Refresh thread label, anchor commit, worktree state, and boundary statement before the next package starts.
```

## 2. OpenSpec Review

```text
openspec dispatch review:
review outcome: approved
gate alignment:
Aligned with Lane 1 accepted design intake and total-control decision to open only a docs-only update gate.
target lane alignment:
Aligned with source acquisition and knowledge governance lane responsibilities.
scope boundary:
Write scope is limited to two docs files. No implementation, fixtures, runtime, seed, snapshot, UI, export, staging, commit, or push.
global rule anchor check:
Aligned with global docs-only rule synchronization and small-block verification discipline.
Core Challenger check:
The dispatch does not claim SourceDeltaBatch validator readiness. It records design and keeps implementation deferred.
Audit Specialist boundary check:
The dispatch permits only whitelisted docs edits and validation. It prohibits cleanup, broad formatting churn, raw value output, and all non-doc surfaces.
safety boundary check:
Aggregate-only and sanitized diagnostics constraints are required. Per-source locator values and raw source details remain prohibited.
validation command check:
Requires startup Git checks, `git diff --check` on the allowed write scope, and targeted `rg` anchors. Cargo is not required for docs-only edits.
report format check:
Requires one copy-ready text block with exact total-control fields.
missing evidence: none for docs-only update
required corrections: none
blocking reason: none
guided next action:
Total-control may send the guided docs-only dispatch below to Lane 1.
```

## 3. OpenSpec-Guided Dispatch

```text
branch-thread instruction:
thread action: reuse
target thread: （运行）HopePrompt知识库-【来源入库与知识治理】
gate: Source acquisition / SourceDeltaBatch / source quality scoring docs-only update

objective:
把已被总控吸收的 Lane 1 设计草案写入文档。只做 docs-only，小块修改，不实现代码，不新增 fixtures。

repository path:
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb

branch / HEAD anchor:
codex/contracts-freeze / 9391519

startup checks:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

read scope:
- openspec/changes/align-new-machine-handoff-governance/source-governance-design-intake.md
- openspec/changes/align-new-machine-handoff-governance/source-governance-dispatch.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md

write scope:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md

allowed actions:
- 小块 docs-only 修改
- 在 source governance 文档中补充 Source acquisition 边界和 source quality scoring rubric
- 在 descriptor validator matrix 文档中补充或细化 SourceDeltaBatch descriptor 字段、未来 fixture matrix 设计、sanitized diagnostics / aggregate-only 规则
- 运行 git diff --check
- 运行 targeted rg 检查

prohibited actions:
- No Rust code changes.
- No fixture file creation or edits.
- No seed changes.
- No snapshot changes.
- No runtime changes.
- No UI changes.
- No export changes.
- No broad formatting churn.
- No staging.
- No commit.
- No push.
- 不得输出 raw source text、per-source locator values、source register details、matched values、本地路径细节、secret、token、provider config、request/response body。

validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- rg -n "SourceDeltaBatch|source quality|aggregate-only|runtime" docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md

stop conditions:
- 如果目标路径不是 E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb，先报告路径冲突。
- 如果 live Git 不是 codex/contracts-freeze / 9391519，先报告冲突。
- 如果允许写入范围之外已有 tracked dirty，先报告，不继续。
- 如果需要修改 docs/prompt-knowledge-freshness-activation-contract-v0.2.md 或其他文件，先报告，不要编辑。
- 如果需要 Rust code、fixtures、seed、snapshot、runtime、UI、export、stage、commit、push，先报告越界。

report format:
必须只用一个 text 代码块回报：

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

key-node reminder:
关键节点提醒：请刷新线程标签、锚点提交、工作树状态和边界说明。
```
