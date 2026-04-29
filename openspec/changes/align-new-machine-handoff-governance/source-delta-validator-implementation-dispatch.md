# SourceDeltaBatch Validator Implementation Dispatch

Dispatch date: 2026-04-30

Total-control thread: `(running) HopePrompt KB - v0.2 total-control`

Target branch lane: `(running) HopePrompt KB - source acquisition and knowledge governance`

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate: Rust-first SourceDeltaBatch static validator implementation
target thread: (running) HopePrompt KB - source acquisition and knowledge governance
thread action: reuse
objective:
Implement the minimum offline SourceDeltaBatch descriptor validation gate approved by total-control. Add a static Rust validator rule, register it in the first-wave dispatcher, add synthetic Lane 1 fixture coverage, and run targeted validator checks. Keep source acquisition, source fetching, source storage, seed, snapshot, runtime, UI, export, wrapper/toolchain repair, canonical digest recomputation, and verified fallback exceptions closed.
repository path: E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD anchor: codex/contracts-freeze / 9391519
known pre-existing dirty state:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- untracked .codex/ and openspec/ alignment artifacts
live git checks required:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- openspec/changes/align-new-machine-handoff-governance/source-governance-docs-intake.md
- tools/descriptor-validator/src/lib.rs
- tools/descriptor-validator/src/rules/mod.rs
- tools/descriptor-validator/src/rules/*.rs
- tools/descriptor-validator/fixtures/pass/*/*.json
- tools/descriptor-validator/fixtures/fail/*/*.json
write scope:
- tools/descriptor-validator/src/rules/source_delta_batch.rs
- tools/descriptor-validator/src/rules/mod.rs
- tools/descriptor-validator/fixtures/pass/lane1_source_delta_batch/*.json
- tools/descriptor-validator/fixtures/fail/lane1_source_delta_batch/*.json
allowed actions:
- add SourceDeltaBatch static validator module
- register SourceDeltaBatch in the first-wave rule dispatcher
- reuse existing denied-field scan and diagnostic shape
- validate required fields, artifact_class, freshness/stale enums, hash shapes, count consistency, review/confidence summaries, activation review preconditions, leakage_count=0, runtime exclusion, and unknown-field fail-closed behavior
- add synthetic pass/fail fixtures only in Lane 1 fixture leaves
- add local unit tests inside the new SourceDeltaBatch rule module if useful
- run cargo fmt on descriptor-validator code
- run cargo test for descriptor-validator
- run descriptor-validator matrix on tools/descriptor-validator/fixtures
prohibited actions:
- no source acquisition implementation
- no source fetching, network, model calls, or auto-ingest
- no source storage or source register reads
- no seed changes
- no snapshot rebuild or snapshot reads
- no activation switch
- no Hope runtime changes or reads
- no UI changes
- no export changes
- no telemetry export changes
- no wrapper/toolchain repair
- no canonical digest recomputation
- no verified fallback exception work
- no edits to docs unless total-control sends a revised dispatch
- no broad refactor or formatting churn outside the write scope
- no staging
- no commit
- no push
- no raw source text, per-source locator values, source register details, matched values, local path details, secrets, tokens, provider config, or request/response bodies
validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- tools/descriptor-validator
- cargo fmt --manifest-path tools/descriptor-validator/Cargo.toml --check
- cargo test --manifest-path tools/descriptor-validator/Cargo.toml
- cargo run --manifest-path tools/descriptor-validator/Cargo.toml -- --matrix tools/descriptor-validator/fixtures
stop conditions:
- target path is not the repository path above
- live branch or HEAD does not match codex/contracts-freeze / 9391519
- tracked dirty files exist outside the known pre-existing docs and the approved implementation write scope
- implementation requires docs, scripts, wrapper/toolchain repair, seed, snapshot, runtime, UI, export, source fetching, source storage, or any network/model call
- validator requires reading raw source registers, raw KB rows, raw prompt bodies, source locator values, snapshot SQLite, or Hope runtime state
- diagnostics would need to output matched values or raw/sensitive payloads
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
Aligned with the accepted source governance docs intake and the docs-only decision to keep SourceDeltaBatch implementation deferred until a separate gate.
target lane alignment:
Aligned with Lane 1 source acquisition and knowledge governance because SourceDeltaBatch is a governance descriptor, not a runtime descriptor.
scope boundary:
Write scope is limited to one new rule module, first-wave dispatcher registration, and synthetic Lane 1 fixture files. It does not open source acquisition, runtime, snapshot, UI, export, wrapper/toolchain, or canonical digest work.
global rule anchor check:
Aligned with small-block verification, allowlisted edits, no user-change reverts, and total-control-only commit/push discipline.
Core Challenger check:
The dispatch only asks for static offline descriptor validation. It does not claim source acquisition readiness, true source ingestion, snapshot activation readiness, canonical digest recomputation, or runtime consumption readiness.
Audit Specialist boundary check:
The dispatch prohibits raw values, matched values, local path details, source registers, request/response bodies, provider config, and secret-like output. Diagnostics remain structural only.
safety boundary check:
Uses synthetic fixtures only, aggregate-only hashes/counts/freshness digests, leakage_count=0, runtime exclusion, and existing denied-field scan.
validation command check:
Requires Git checks, diff check, cargo fmt check, cargo test, and descriptor fixture matrix validation. It does not require `cargo +1.95.0` wrapper repair because the fixed-toolchain issue is outside this gate.
report format check:
Requires one copy-ready text block with the existing total-control fields.
missing evidence: none for opening this implementation gate
required corrections: none
blocking reason: none
guided next action:
Total-control may send the guided implementation dispatch below to Lane 1.
```

## 3. OpenSpec-Guided Dispatch

```text
branch-thread instruction:
thread action: reuse
target thread: (running) HopePrompt KB - source acquisition and knowledge governance
gate: Rust-first SourceDeltaBatch static validator implementation

objective:
Implement only the minimum offline SourceDeltaBatch descriptor validation gate approved by total-control. Add the static Rust validator rule, register it in the first-wave dispatcher, add synthetic Lane 1 pass/fail fixtures, and run targeted validator checks.

repository path:
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb

branch / HEAD anchor:
codex/contracts-freeze / 9391519

known pre-existing dirty state:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- untracked .codex/ and openspec/ alignment artifacts

startup checks:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat

read scope:
- docs/prompt-knowledge-source-governance-v0.2.md
- docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md
- openspec/changes/align-new-machine-handoff-governance/source-governance-docs-intake.md
- tools/descriptor-validator/src/lib.rs
- tools/descriptor-validator/src/rules/mod.rs
- tools/descriptor-validator/src/rules/*.rs
- tools/descriptor-validator/fixtures/pass/*/*.json
- tools/descriptor-validator/fixtures/fail/*/*.json

write scope:
- tools/descriptor-validator/src/rules/source_delta_batch.rs
- tools/descriptor-validator/src/rules/mod.rs
- tools/descriptor-validator/fixtures/pass/lane1_source_delta_batch/*.json
- tools/descriptor-validator/fixtures/fail/lane1_source_delta_batch/*.json

allowed actions:
- add SourceDeltaBatch static validator module
- register SourceDeltaBatch in the first-wave rule dispatcher
- reuse existing global denied-field scan
- validate required fields
- validate artifact_class=governance_descriptor
- validate canonical freshness / stale enum usage
- validate hash shapes for descriptor_hash, source_delta_batch_hash, source_freshness_digest, content_hashes, and previous_content_hashes
- validate source_delta_count and accepted/rejected/quarantined/limited count consistency
- validate review_status_summary and effective_confidence_summary presence
- validate activation_requested requires all_required_reviews_present=true and non-empty activation_blocked_reason_codes when blocked
- validate leakage_count=0
- validate all_sources_runtime_excluded=true
- validate unknown fields fail closed
- add synthetic pass/fail fixtures only under lane1_source_delta_batch leaves
- add unit tests in the new module if useful
- run cargo fmt/check/test/matrix validation

prohibited actions:
- no source acquisition implementation
- no source fetching, network, model calls, or auto-ingest
- no source storage or source register reads
- no seed changes
- no snapshot rebuild or snapshot reads
- no activation switch
- no Hope runtime changes or reads
- no UI changes
- no export changes
- no telemetry export changes
- no wrapper/toolchain repair
- no canonical digest recomputation
- no verified fallback exception work
- no docs edits unless total-control sends a revised dispatch
- no broad refactor or formatting churn outside the write scope
- no staging
- no commit
- no push
- no raw source text, per-source locator values, source register details, matched values, local path details, secrets, tokens, provider config, or request/response bodies

required inputs:
- accepted docs-only updates already present in the two source governance docs
- OpenSpec docs intake: openspec/changes/align-new-machine-handoff-governance/source-governance-docs-intake.md

validation commands:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
- git diff --check -- tools/descriptor-validator
- cargo fmt --manifest-path tools/descriptor-validator/Cargo.toml --check
- cargo test --manifest-path tools/descriptor-validator/Cargo.toml
- cargo run --manifest-path tools/descriptor-validator/Cargo.toml -- --matrix tools/descriptor-validator/fixtures

stop conditions:
- If the target path is not E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb, report the path mismatch and stop.
- If live Git is not codex/contracts-freeze / 9391519, report the mismatch and stop.
- If tracked dirty files exist outside the two known docs and the implementation write scope, report first and stop.
- If the work requires docs, scripts, wrapper/toolchain repair, seed, snapshot, runtime, UI, export, source fetching, source storage, or network/model calls, report scope expansion and stop.
- If validation would need raw source registers, raw KB rows, raw prompt bodies, source locator values, snapshot SQLite, or Hope runtime state, report scope expansion and stop.
- If diagnostics would need to output matched values or raw/sensitive payloads, stop and report the safety issue.

report format:
Return exactly one text code block with:

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
