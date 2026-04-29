# Source Wiki Artifact Contract Dispatch

Dispatch date: 2026-04-30

Total-control thread: `(running) HopePrompt KB - v0.2 total-control`

Target branch lane: `(running) HopePrompt KB - source acquisition and knowledge governance`

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate: source intake and reviewed wiki artifact contract docs-only
target thread: (running) HopePrompt KB - source acquisition and knowledge governance
thread action: reuse
objective:
Write the accepted minimal offline source-to-wiki loop into docs as an artifact contract. Define source_intake, wiki_draft, reviewed_wiki, and SourceDeltaBatch handoff boundaries, field allowlists/denylists, lifecycle states, review transitions, aggregate-only diagnostics, and future validator-design notes. Do not create artifacts or implement storage/runtime/validator work.
repository path: E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD anchor: codex/contracts-freeze / dd8608d
live git checks required:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
- openspec/changes/align-new-machine-handoff-governance/source-to-wiki-loop-intake.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md
write scope:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
allowed actions:
- docs-only edits in the write scope
- add source intake artifact contract
- add wiki draft artifact contract
- add reviewed wiki artifact contract
- add lifecycle and review transition rules
- add SourceDeltaBatch handoff rules from reviewed wiki to aggregate descriptor evidence
- add denied/sanitized/aggregate-only diagnostics rules
- add future validator-design matrix notes for source/wiki artifacts
- run targeted validation commands
prohibited actions:
- no real artifact creation
- no synthetic artifact creation
- no source fetching
- no source storage
- no source register reads
- no raw source text collection
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
- no raw KB rows, raw prompt bodies, full source registers, overlay JSON, raw graph payloads, local path details, secrets, tokens, provider config, request/response bodies, matched values, or per-source locator values
validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- rg -n "source_intake|wiki_draft|reviewed_wiki|SourceDeltaBatch|aggregate-only|runtime_excluded|leakage_count" docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
stop conditions:
- target path is not the repository path above
- live Git is not codex/contracts-freeze / dd8608d or a clean fast-forward successor
- tracked dirty files exist before editing
- edits require files outside the write scope
- design requires implementation, artifact creation, source fetching/storage, seed/snapshot/runtime/UI/export, validator/fixture changes, staging, commit, or push
- docs wording would allow raw source text, locator values, source register paths, or per-source evidence into runtime payload/log/UI/telemetry
expected report format:
single copy-ready text block with required total-control fields, changed files, validation results, risks, decisions needed, and next recommendation
key-node reminder:
Refresh thread label, anchor commit, worktree state, and boundary statement before the next package starts.
```

## 2. OpenSpec Review

```text
openspec dispatch review:
review outcome: approved
gate alignment:
Aligned with the accepted source-to-wiki minimal loop intake and total-control decision to open docs-only artifact contract work.
target lane alignment:
Aligned with Lane 1 source acquisition and knowledge governance responsibilities.
scope boundary:
Docs-only edits in two approved docs. No artifacts, source fetching/storage, seed/snapshot/runtime/UI/export, validator, fixture, OpenSpec, stage, commit, or push actions.
global rule anchor check:
Aligned with live Git truth, small-block edits, OpenSpec supervision, and total-control-only commit/push discipline.
Core Challenger check:
The dispatch records a contract only and does not claim real source acquisition, reviewed wiki implementation, SourceDeltaBatch cross-artifact binding, snapshot activation, or runtime readiness.
Audit Specialist boundary check:
The dispatch prohibits raw values, matched values, locators, source register details, local path details, credentials, provider config, and request/response bodies.
safety boundary check:
Requires runtime exclusion, leakage zero, denied fields, and aggregate-only handoff language.
validation command check:
Requires Git checks, diff check on write scope, and targeted `rg` anchors. Cargo is not required for docs-only edits.
report format check:
Requires one copy-ready text block with total-control fields.
missing evidence: none for docs-only contract update
required corrections: none
blocking reason: none
guided next action:
Total-control may send the guided docs-only dispatch below to Lane 1.
```

## 3. OpenSpec-Guided Dispatch

```text
分线线程指令：
线程动作：沿用
目标线程：（运行）HopePrompt知识库-【来源入库与知识治理】
gate：source intake and reviewed wiki artifact contract docs-only

目标：
把总控已吸收的最小离线 source-to-wiki loop 写入文档合同。只做 docs-only，不创建 artifact、不抓取 source、不实现 storage/runtime/validator。

仓库路径：
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb

branch / HEAD anchor：
codex/contracts-freeze / dd8608d

启动检查：
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

读取范围：
- openspec/changes/align-new-machine-handoff-governance/source-to-wiki-loop-intake.md
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/llm-wiki-governance-model-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- docs/prompt-knowledge-freshness-activation-contract-v0.2.md

写入范围：
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md

允许动作：
- 小块 docs-only 修改
- 增加 source_intake artifact contract
- 增加 wiki_draft artifact contract
- 增加 reviewed_wiki artifact contract
- 增加 lifecycle / review transition rules
- 增加 reviewed_wiki -> SourceDeltaBatch handoff rules
- 增加 denied / sanitized / aggregate-only diagnostics rules
- 增加未来 source/wiki artifact validator-design matrix notes
- 运行指定验证命令

禁止动作：
- 不创建真实 artifact
- 不创建 synthetic artifact
- 不抓取 source
- 不建立 source storage
- 不读取 source register
- 不采集 raw source text
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
- 不泄漏 raw KB rows、raw prompt body、full source register、overlay JSON、raw graph、本地路径细节、secret、token、provider config、request/response body、matched values、per-source locator values

验证命令：
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- rg -n "source_intake|wiki_draft|reviewed_wiki|SourceDeltaBatch|aggregate-only|runtime_excluded|leakage_count" docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md

停止条件：
- 目标路径不是 E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
- live Git 不是 codex/contracts-freeze / dd8608d 或其干净 fast-forward 后继
- 编辑前存在 tracked dirty
- 需要修改写入范围之外的文件
- 需要实现、创建 artifact、抓取/存储 source、seed/snapshot/runtime/UI/export、validator/fixture 改动、stage、commit、push
- 文档措辞会允许 raw source text、locator values、source register path、per-source evidence 进入 runtime payload/log/UI/telemetry

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

关键节点提醒：
请刷新线程标签、锚点提交、工作树状态和边界说明。
```
