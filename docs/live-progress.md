# hope-kb Live Progress

Update this file for online sync after every 10-20 minute work package.

## Current Focus

- current thread state: `codex/contracts-freeze`, `hope-kb` only, no merge with `hope`
- repo path: `E:\codex\hope-kb-live-20260525`
- current task: productize the 5179 manual intake loop from live search through user confirmation, KB apply, runtime snapshot, and PWA summary-only handoff
- owner / lane: KB integration owner as total control, coordinating four verification lanes and keeping `hope` runtime / desktop work out of scope
- last updated: 2026-05-25 16:52:00 +08:00

## Latest Completed

- 2026-05-25 16:52 +08: 5179 UI confirmation path was clarified. The
  execution chain now has a persistent `用户确认入口` that explains
  `保留` as a screening-only decision, shows selected/confirmable counts,
  disables confirmation until the user checks a recommendation, and switches
  to `确认已完成` after apply. The evidence area now uses a two-column layout
  with compact collapsed cards, and screening/recommendation headers display
  live counts such as `5 条筛选记录` and `已写入 1`.
- 2026-05-25 15:54 +08: PWA adapter contract boundary was refreshed after
  the first real PWA runtime consumer anchor
  `hope-web-pwa@ee350c7`. `docs/pwa-adapter-contract-v0.2.md` now records the
  prototype activation opt-ins (`allowPrototype=true`,
  `allowPartialSceneCatalog=true`), the current effective fallback
  (`hope-web-pwa` built-in `KB_SNAPSHOT` only), artifact freshness handoff
  requirements, the unchanged summary-only downstream surface, and QA
  observability limits. This was docs-only and did not reopen PWA fields,
  runtime stages, raw KB transport, export shape, or Hope mainline merge.
- 2026-05-25 15:36 +08: 5179 dashboard reached a live-search-to-KB smoke
  loop under the current `codex/contracts-freeze` route. The confirmed live
  run is
  `knowledge/intake_runs/intake-20260525073418-seedance2-turbo-search-smoke-camera-action-promp/`.
  It used `HOPE_KB_SEARCH_KEY` from local env only, returned
  `search_status=live_search`, `reason=ok`, and 5 URL-bound summary candidates.
- The live-search run was screened into 5 KB recommendations. The manual
  confirmation gate accepted only `rw-camera-language-grammar`; the package
  then applied successfully with `status=applied_to_kb` and
  `validation_status=passed`. Empty or missing `accepted_reviewed_wiki_ids`
  now fail validation and cannot be applied.
- 5179 product flow now shows the execution chain under the graph: search
  candidates, rule screening / intake recommendation, confirmation package,
  and KB write + validation. Buttons are disabled after successful apply to
  avoid duplicate writes.
- Provider status/test API is live at `/api/provider/status` and
  `/api/provider/test`. The dashboard checks the env file under
  `Desktop\换机用\hope-kb-search.env` first, records only provider/model/host
  and credential state, and never exposes the key.
- Search robustness improved: Qwen web search uses explicit `turbo` strategy
  and disables thinking by default; if live web search fails, the server tries
  a normal Qwen model fallback before using registered fixture candidates, with
  distinct status reasons for each path.
- Verification after live-search apply passed:
  `node scripts/validate-manual-intake-package.js --package
  knowledge\intake_runs\intake-20260525073418-seedance2-turbo-search-smoke-camera-action-promp\intake-package.confirmed.json`,
  `node scripts/build-pwa-kb-adapter-output.js --check`,
  `powershell.exe -ExecutionPolicy Bypass -File
  scripts\validate-kb-runtime-prototype.ps1`, and
  `node scripts\sync-pwa-kb-latest.js --pwa-root
  "C:\Users\Administrator\Documents\New project\hope-web-pwa-inspect"`.
  PWA `src/lib/kbAdapter.test.ts` passed 13 tests.
- Five-lane closeout notes were integrated: provider fallback states are now
  documented, manual intake validation enforces package/recommendation status
  enums, `apply` output stored in packages is sanitized and truncated as
  `stdout_summary` / `stderr_summary`, and the Seedance2 field-consumption
  matrix now distinguishes KB runtime snapshot fields from PWA adapter
  `sceneMappings`.
- PWA handoff closeout now syncs both tracked targets:
  `public/kb/latest.json` and `dist/kb/latest.json`. Their SHA256 values match
  `samples/pwa-kb-adapter-output.sample.json`:
  `2B1B07002F37A5DF8A536BC0F5B24A5A34A97F72AD26C8153A1A7125785A3B41`.
  Full PWA `npm test` passed with 7 files / 30 tests, and `npm run build`
  passed.

- 2026-05-25 continuation used fresh remote-aligned worktrees instead of the
  stale local `E:\codex\hope-kb` checkout: KB HEAD
  `4ead8078b57c286b71633d198cd1aeafd639aa3e`, PWA HEAD
  `ae62249978c0970522222ce2bf4b317ab470d10b`.
- Seedance2 field-discipline knowledge was promoted through a confirmed manual
  intake package:
  `knowledge/intake_runs/intake-20260525-seedance2-field-discipline/intake-package.confirmed.json`.
  It writes only summary-safe guidance for existing PWA fields: `camera`,
  `shot_size`, `visual_description`, `character_action`, and `prompt_text`.
- Added reviewed wiki `rw-seedance2-storyboard-field-discipline`, mapping
  `map-rw-seedance2-storyboard-field-discipline`, and director rule pack
  `dg-seedance2-storyboard-field-discipline`. The new rule pack is constrained
  to existing adapter surfaces and does not add PWA fields, stages, provider
  paths, export shape, or dashboard runtime role.
- Regenerated `knowledge/runtime_snapshots/latest.candidate.json`,
  `samples/runtime-kb-snapshot.sample.json`,
  `knowledge/mappings/wiki-to-runtime-mapping.v0.2.json`,
  `samples/wiki-to-runtime-mapping.sample.json`, and
  `samples/pwa-kb-adapter-output.sample.json`. Adapter check passed with 21
  scene types and 17 rule packs.
- Synced the regenerated adapter to the PWA latest snapshot files in the fresh
  PWA worktree: `public/kb/latest.json` and `dist/kb/latest.json`. Their SHA256
  matched the KB adapter sample exactly after sync.
- Verification passed: `node scripts/validate-manual-intake-package.js
  --package knowledge\intake_runs\intake-20260525-seedance2-field-discipline\intake-package.confirmed.json`,
  `node scripts/apply-confirmed-intake-package.js --package
  knowledge\intake_runs\intake-20260525-seedance2-field-discipline\intake-package.confirmed.json`,
  `node scripts/build-pwa-kb-adapter-output.js --check`, and
  `powershell.exe -NoProfile -ExecutionPolicy Bypass -File
  scripts\validate-kb-runtime-prototype.ps1`.
- PWA `npm test` and `npm run build` were attempted without installing
  dependencies. They are blocked in the fresh clone because `node_modules` is
  absent: `vitest` and `tsc` are not recognized. This matches the handoff rule
  to avoid direct `npm install` when `E:\coderely\env\coderely-dev.cmd` is not
  present.
- Existing dashboard sample validation blocker was closed in the fresh KB
  worktree: `web/kb-flow-dashboard/source-candidates.sample.json` now carries
  `no_runtime_effect=true`, and
  `rw-seedance-anime-style-and-frame-constraint` now references existing
  candidate `src-nezha-production-chinanews` instead of the missing
  `src-wushan-director-interview-bilibili`. `node
  scripts/validate-manual-intake-samples.js` now passes with 1 run, 6 source
  candidates, and 10 recommendations.

- live Git anchor checked by control: `52b61c7` (`docs: add v0.2 QA hardening RFC`) on `codex/contracts-freeze`, matching `origin/codex/contracts-freeze`
- `git diff --stat` was empty at takeover; `git status --short --branch` reported only the branch line plus a `.git/index.lock` unlink warning
- existing `.git/index.lock` observed at `2026-04-26 11:38:04 +08:00`; control did not delete or repair it without explicit instruction
- required baseline files read: `AGENTS.md`, `docs/live-progress.md`, the v0.2 QA RFC and four worker packets, `seed/v0.2/manifest.json`, and `scripts/validate-v0-2-seed-bundle.ps1`
- route confirmed: this project serves Hope with Prompt knowledge governance, graph QA, and verified snapshots; it is not an independent product and does not open image/video generation, Hope desktop expansion, runtime GraphRAG, hybrid/rerank default, runtime network fetch, runtime auto-ingest, or runtime LLM summarize
- total-control baseline docs added: `docs/prompt-knowledge-core-v0.2.md`, `docs/llm-wiki-governance-model-v0.2.md`, and `docs/graph-governance-v0.2.md`
- current QA/RFC baseline remains docs-only: `manifest_hash` / `snapshot_hash` / `index_hash`, last-known-good / fail-closed behavior, quarantine isolation, source minimum fields, BM25 score normalization, layered eval stale blocking, ephemeral prompt safety, structural leakage tests, local telemetry purge, shadow thresholds, and rollback triggers
- additional untracked lane-like docs observed after validation: `docs/prompt-knowledge-source-governance-v0.2.md`, `docs/prompt-knowledge-schema-snapshot-contract-v0.2.md`, `docs/prompt-knowledge-router-futureqa-contract-v0.2.md`, and `docs/prompt-knowledge-lint-safety-contract-v0.2.md`; control has not modified, staged, or integrated them pending the required text-block reports
- core collaboration rules from `Codex总控分线程协作规则_2026-04-25` synced into `AGENTS.md`, `docs/project-thread-startup.md`, `docs/parallel-execution-appendix.md`, and this live sync board: reports/instructions use `text` code blocks, branch reports start with `总控回报：`, reports target `总控线程`, Git/file state is the only truth source, branch threads do not commit/push by default, and current v0.2 five-agent lanes are named explicitly
- five-agent strict questioning completed by control on 2026-04-26: all four lanes agree the repo can start only with read-only contract review / text-block reports; no lane recommends implementation, staging, commit, snapshot build, seed edits, validator edits, runtime work, or push until total control resolves untracked document ownership and `.git/index.lock`
- first-stage lane reports received from Lane 1 source governance, Lane 2 schema/snapshot, Lane 4 lint/leakage, and Lane 3 router/FutureQA; all four recommend docs-only integration only, with no implementation or runtime gate
- docs-only integration reports received from Lane 1, Lane 2, Lane 3, and Lane 4. Target documents now contain source field normalization mapping, hash semantic split, runtime artifact classes, QueryResult / RetrievalTrace / FutureQACandidate / EvalArtifact descriptor drafts, lint artifact-class allow-list, denied aliases, telemetry purge descriptor, and rollback descriptor. Total-control verification found `diff --check` clean and v0.2 seed validation passing.
- Lane 2 / Lane 4 terminology alignment received and control-verified: `retrieval_trace_log_telemetry_shadow_rollback` is the canonical non-model-visible artifact class for retrieval trace, log, telemetry, shadow, and rollback records; `retrieval_trace_log_telemetry` is not a separate canonical class.
- stale `.git/index.lock` was removed after explicit approval; no active Git process was observed on the second process check
- docs-only governance package was committed and pushed as `9fcb391` (`docs: add v0.2 prompt knowledge governance contracts`); `AGENTS.md` was intentionally excluded from the commit
- v0.2 control rules in `AGENTS.md` were reviewed, committed, and pushed as `aa1e224` (`docs: sync v0.2 control rules`)
- Freshness / Activation four-lane reports were received and integrated into `docs/prompt-knowledge-freshness-activation-contract-v0.2.md`
- Machine-Checkable Descriptor / Validator Gate opened by total control after `2bd7f0c`; first step is four-lane read-only descriptor breakdown, not validator implementation
- Machine-Checkable Descriptor / Validator four-lane reports were received and integrated into `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`
- Implementation language policy recorded: when code implementation is explicitly opened, validators and durable governance logic are Rust-first; PowerShell remains only thin Windows orchestration or explicit total-control exception
- Offline Descriptor Fixture Validator Implementation Gate is opened as a Rust-first implementation gate after `8f8bd87`; first wave priority is Lane 2 Rust crate/scaffold and Lane 4 structural leakage scanner design/implementation, with Lane 1 and Lane 3 waiting for the scaffold before adding domain fixtures/rules
- Lane 2 Rust scaffold was received and control-verified: `tools/descriptor-validator` exists, exposes CLI help, defines canonical descriptor/freshness/activation/fallback enums, includes six artifact classes, and passes `cargo +1.95.0 test` with 4 tests; `target/` is ignored as a build artifact
- Lane 4 first-wave safety base was received and control-integrated: `tools/descriptor-validator` now validates offline JSON descriptor fixtures with artifact-class allow-listing, recursive denied-field scanning, sanitized diagnostics, `full_kb_rows_included=0`, and `leakage_count=0`; control fixed CLI failure semantics so diagnostics return a non-zero exit code, refreshed help text, added UTF-8 BOM tolerance, and kept `target/` ignored
- Lane 4 second-wave safety rules were received and control-verified: `PurgeDescriptor` zero-residue checks, `RollbackDescriptor` sanitized-only checks, and `RefreshTelemetryRecord` required-field checks are implemented in Rust; validation passes with 23 tests and remains limited to offline JSON fixtures
- Cross-Descriptor Binding Review Gate is opened after `14d391e`; Lane 2 and Lane 3 should report read-only implementation breakdowns for activation/hash/pointer binding and stale auto-switch blocking before total control opens the next Rust implementation slice
- Lane 2 and Lane 3 cross-descriptor read-only reports were received; total control recorded decisions in `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`, including strict empty selected IDs for stale/index-miss first pass, deferred verified fallback exceptions, static-before-cross implementation order, and deferred canonical digest recomputation
- Lane 2 activation/pointer static implementation was received for total-control review: `ActivationDescriptor`, `ActivePointer`, `LastKnownGoodDescriptor`, and `RollbackPointer` single-descriptor rules are implemented without cross fixture binding; hash shape remains fixture-friendly `sha256:<token>` / `bundle-sha256:<token>` until the canonical digest gate
- Lane 3 router/eval/FutureQA static implementation was received for total-control review: runtime/media/expensive flags, auto-switch gating, `EvalArtifact`, `QueryResult`, `RetrievalTrace`, and `FutureQACandidate` single-descriptor rules are implemented without verified fallback exceptions or cross fixture binding
- Descriptor Fixture Gate is opened after `5542509`; total control recorded fixture directory ownership in `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`. Lane 2, Lane 3, and Lane 4 may add JSON fixtures under disjoint pass/fail leaf directories; Lane 1 source-delta fixtures remain deferred until SourceDeltaBatch static rules exist.
- Descriptor Fixture Gate reports were received from Lane 2, Lane 3, and Lane 4. Total control verified the live untracked `tools/descriptor-validator/fixtures/` tree matches the reported lane-owned pass/fail fixture directories and contains no extra unreported fixture leaves.
- Descriptor fixture validation matrix passed under total control: Lane 2 activation/pointer pass `status=passed` with 4 descriptors and fail `status=failed` with 4 descriptors; Lane 3 router/eval/FutureQA pass `status=passed` with 4 descriptors and fail `status=failed` with 6 descriptors; Lane 4 safety/observability pass `status=passed` with 5 descriptors and fail `status=failed` with 6 descriptors.
- Control verification also passed `cargo +1.95.0 test --manifest-path tools/descriptor-validator/Cargo.toml` with 45 tests, `git diff --check`, `scripts/validate-v0-2-seed-bundle.ps1`, fixture trailing-whitespace scan, and a fixture sensitive-pattern scan. The only sensitive-pattern hits were intentional synthetic fail-field names / trigger-code strings or denial text, not real raw KB, paths, source registers, prompts, or secrets.
- Descriptor Fixture Gate was committed and pushed as `f1a63d3` (`tools: add descriptor validator fixtures`); live Git is clean and aligned with `origin/codex/contracts-freeze`.
- Cross-Descriptor Fixture-Set Binding Gate is opened after `f1a63d3` in `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`. This next gate stays offline JSON fixture-only and targets literal `descriptor_id + descriptor_hash` binding for activation/pointer/LKG/rollback descriptors; canonical digest recomputation, verified fallback exceptions, SourceDeltaBatch implementation, runtime artifact reads, snapshot SQLite reads, raw KB reads, and Hope runtime access remain closed.
- Lane 2 cross-descriptor binding implementation was received and control-verified: `DescriptorSet` indexes offline JSON fixture descriptors by `descriptor_id + descriptor_hash`, `ActivePointer` binds to `ActivationDescriptor`, `LastKnownGoodDescriptor` binds to verified activated/fresh activation descriptors, and `RollbackPointer` binds failed activation candidates to verified LKG targets. Validation passed with 50 Rust tests, Lane 2/3/4 pass/fail fixture matrix, v0.2 seed bundle validation, `git diff --check`, Lane 2 fixture trailing-whitespace scan, and Lane 2 fixture sensitive-pattern scan.
- Lane 3 read-only review was received and accepted: cross-descriptor binding does not open verified fallback exceptions, does not relax stale/index-miss empty-selection rules, keeps `auto_switch_allowed=true` limited to fresh/all-gates/controller-approved/runtime-flags-false cases, keeps FutureQACandidate out of eval truth, and does not misapply `cross_descriptor_binding` diagnostics to QueryResult, RetrievalTrace, EvalArtifact, or FutureQACandidate fixtures.
- Lane 4 read-only review was received and accepted: `cross_descriptor_binding` diagnostics still use the sanitized five-field shape only (`descriptor_type`, `descriptor_id`, `field_path`, `denied_class`, `rule_id`), do not emit matched values or raw path/source/prompt/secret content, do not misapply to Lane 4 safety fixtures, and preserve denied-scan / full-KB-row / leakage-count / purge / rollback boundaries.
- Duplicate Descriptor Identity Gate is opened in `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`: duplicate literal `descriptor_id + descriptor_hash` identities must fail closed in offline fixture sets; recursive fixture runner, canonical digest recomputation, SourceDeltaBatch implementation, and verified fallback exceptions remain closed.
- Lane 2 duplicate descriptor identity implementation was received and control-verified: `DescriptorSet` tracks duplicate `descriptor_id + descriptor_hash` occurrences, `duplicate_descriptor_identity` diagnostics mark the second and later duplicate descriptors without emitting descriptor hashes or matched values, and Lane 2 fail fixtures now include `06_duplicate_descriptor_identity.json`. Validation passed with 52 Rust tests, Lane 2/3/4 pass/fail fixture matrix, v0.2 seed bundle validation, `git diff --check`, and duplicate fixture whitespace/sensitive-pattern scans.
- Lane 4 read-only review of `duplicate_descriptor_identity` was received and accepted: diagnostics keep the sanitized five-field shape, do not emit duplicate descriptor hash / matched value / file path / raw path / raw source text / prompt body / source register / secret, and do not misapply to Lane 4 safety/observability fixtures.
- Recursive Fixture Matrix Runner Gate is opened in `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`: the next tool slice should run all existing pass/fail fixture leaves from one explicit fixture root while preserving current validation semantics and sanitized reporting. Canonical digest recomputation, SourceDeltaBatch implementation, verified fallback exceptions, runtime reads, snapshot SQLite reads, raw KB reads, and Hope runtime access remain closed.
- Lane 4 recursive fixture matrix runner implementation was received and control-verified: `descriptor-validator --matrix <fixture-root>` now validates all existing `pass/<leaf>/` and `fail/<leaf>/` directories in one run, treats expected failing leaves as successful matrix cases, keeps the original single-directory mode, and outputs only sanitized leaf-level summary fields. Validation passed with 55 Rust tests, Lane 2/3/4 single-leaf pass/fail checks, matrix `status=passed` across 6 leaves, v0.2 seed bundle validation, `git diff --check`, and a matrix-summary sensitive-pattern check.
- Lane 4 thin PowerShell wrapper was received and control-verified: `scripts/validate-descriptor-fixtures.ps1` only orchestrates the Rust matrix runner, accepts `-RepoRoot` and optional `-FixtureRoot`, keeps `FixtureRoot` inside `RepoRoot`, returns the Rust CLI exit code, and contains no raw KB / source-register / prompt / secret/provider output path. Control verified default matrix execution, outside-root rejection with exit code 2, 55 Rust tests, and `git diff --check`.

## Five-Agent Start Readiness

- Lane 1 `来源入库与知识治理`: start first as a read-only source/provenance review; check `docs/prompt-knowledge-source-governance-v0.2.md`, `seed/v0.2/source_register.json`, `seed/v0.2/manifest.json`, and the snapshot/source integrity QA packet; do not change `source_register` fields yet
- Lane 2 `图谱Schema与快照合同`: start in parallel as a read-only schema/snapshot contract consistency review; focus on `docs/prompt-knowledge-schema-snapshot-contract-v0.2.md` and cross-check core/source/graph/router/lint contracts
- Lane 3 `路由评测与FutureQA`: start after Lane 2 begins, or in parallel as read-only only; verify that FutureQA candidates do not become eval truth and stale eval blocks auto-switch
- Lane 4 `安全Lint与泄漏防护`: start after Lane 2 starts, or in parallel as read-only only; verify leakage deny-list, allowed summary fields, telemetry purge, rollback trigger, and ephemeral prompt safety boundaries
- total-control decision: first operational wave is `read-only contract review + unified total-control integration plan`, not implementation

## First-Stage Control Decision

- status: all four lane reports are now received
- decision: open a docs-only integration gate, not an implementation gate
- include candidates: `docs/prompt-knowledge-core-v0.2.md`, `docs/llm-wiki-governance-model-v0.2.md`, `docs/graph-governance-v0.2.md`, `docs/prompt-knowledge-source-governance-v0.2.md`, `docs/prompt-knowledge-schema-snapshot-contract-v0.2.md`, `docs/prompt-knowledge-router-futureqa-contract-v0.2.md`, `docs/prompt-knowledge-lint-safety-contract-v0.2.md`
- do not touch: seed JSON, manifests, snapshots, validators, migrations, Hope runtime, desktop UI, image/video generation, GraphRAG, hybrid/rerank defaults, runtime network fetch, runtime auto-ingest, or runtime LLM summarization
- integration issues to resolve before staging:
  - old RFC anchors still mention `E:\codex\hope-kb` and `d5b319c`; current live anchor is this repo at `52b61c7`
  - `manifest_hash` vs `seed_bundle_hash` naming/semantic split is not settled
  - runtime fields must be split into `prompt_payload`, `retrieval_trace_log_telemetry_shadow_rollback`, and `activation_descriptor`
  - source field normalization is deferred; current `source_register.json` remains `path` / `sha256` and lacks review/runtime eligibility fields
  - path and secret aliases need explicit lint coverage, including `path`, `url_or_path`, `source_register[].path`, `secret_ref`, `credential_ref`, `api_key_ref`, and provider config references
  - FutureQA candidates must remain non-truth until reviewed, Git-promoted, rebuilt, and validated
  - resolved: Lane 2 and Lane 4 now use `retrieval_trace_log_telemetry_shadow_rollback` as the canonical non-model-visible artifact class

## Branch Reports Status

All branch reports have been received and accepted for docs-only integration.

- `HopePrompt知识库-【来源入库与知识治理】`
- `HopePrompt知识库-【图谱Schema与快照合同】`
- `HopePrompt知识库-【路由评测与FutureQA】`
- `HopePrompt知识库-【安全Lint与泄漏防护】`

Required report fields:

- 总控回报
- 目标线程：总控线程
- 来源线程
- 仓库路径
- branch / HEAD
- git status
- 已完成
- 未完成
- 修改文件
- 验证结果
- 是否提交 / push
- 风险 / 阻塞
- 需要总控决策的问题
- 下一步建议

Copy-ready report template:

```text
总控回报：
目标线程：总控线程
来源线程：（运行）HopePrompt知识库-【目标动作】
仓库路径：
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD：
...
git status：
...
已完成：
...
未完成：
...
修改文件：
...
验证结果：
...
是否提交 / push：
未提交，未 push。
风险 / 阻塞：
...
需要总控决策的问题：
...
下一步建议：
...
```

## Freshness / Activation Gate

- status: opened by total control after `aa1e224`
- goal: define how KB data can be refreshed quickly while keeping runtime consumption limited to verified snapshots
- output target: `docs/prompt-knowledge-freshness-activation-contract-v0.2.md`
- gate type: docs-only contract planning before validator/runtime implementation
- runtime boundary: no runtime network fetch, no runtime auto-ingest, no raw source exposure, no default LLM summarize, no GraphRAG/hybrid/rerank default, and no image/video generation
- freshness model: source deltas enter governance first, then pass validation, then activate through atomic snapshot pointer update
- fallback model: runtime remains on last-known-good snapshot if source ingestion, validation, eval, leakage lint, or activation binding fails
- branch work requested:
  - Lane 1: source delta, freshness metadata, provenance locator, and review-status boundaries
  - Lane 2: activation descriptor, atomic switch, hash binding, last-known-good, and rollback pointer
  - Lane 3: stale index/eval/query blocking, FutureQA freshness status, and fallback reason codes
  - Lane 4: refresh telemetry, leakage restrictions, purge/rollback records, and denied fields
- integration decision: `freshness_status` uses `fresh`, `stale_source`, `stale_index`, `stale_eval`, `stale_snapshot`, `activation_failed`, `unknown`, and `blocked`
- integration decision: `activation_status` uses `candidate`, `validating`, `verified`, `activated`, `failed`, `rolled_back`, and `superseded`; `no_kb_context` remains a fallback reason, not an activation status
- integration decision: `source_delta` may bind only through aggregate `source_delta_batch_hash`, `source_delta_count`, and `source_freshness_digest`
- integration decision: `rollback_pointer` may point only to a verified last-known-good activation descriptor, not raw snapshot paths

## Next Up

- commit and push `scripts/validate-descriptor-fixtures.ps1` after whitelist staging
- next gate candidate: pause implementation for a validator usage note and total-control readiness checkpoint before opening canonical digest or SourceDeltaBatch work
- keep canonical digest recomputation, SourceDeltaBatch implementation, verified fallback exceptions, runtime reads, snapshot SQLite reads, raw KB reads, and Hope runtime access closed
- keep Lane 1 read-only on future SourceDeltaBatch participation; no source-delta implementation in this gate
- keep `E:\codex\hope` untouched

## Blockers / Risks

- blocker: none for the docs-only integration package after stale `.git/index.lock` removal
- blocker: none for the docs-only Freshness / Activation contract draft
- blocker: none for the docs-only descriptor validator matrix draft
- blocker: none for Lane 4 second-wave safety rules after control verification; next critical path item is cross-descriptor binding and domain fixture coverage
- risk: current package is docs/RFC and governance baseline only; it does not yet implement machine-checkable validator gates for the new QA contracts
- risk: standard ignored `snapshots/hope-kb-v0.2.sqlite3` previously differed from verified `rebuilt-5`; any future runtime activation must bind a chosen snapshot by hash instead of relying on path name
- risk to `hope` separation: none in current scope; keep Hope product-side runtime hookup, desktop, intake, Qwen, Doubao, Seedance, GraphRAG, hybrid/rerank default, and runtime LLM summarize out of this repo/package
- merge-readiness status: closed, not reopened
