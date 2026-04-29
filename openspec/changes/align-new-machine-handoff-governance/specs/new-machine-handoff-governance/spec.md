## ADDED Requirements

### Requirement: Repository Restore Uses Live Git Truth
The handoff process MUST restore `hope-kb` on `codex/contracts-freeze`, verify remote configuration, and treat live Git state as authoritative when it differs from historical handoff text.

#### Scenario: Fresh clone succeeds
- **WHEN** a new machine does not have the target repository
- **THEN** the operator clones `https://github.com/zzy170031-cmd/hope-kb.git`, checks out `codex/contracts-freeze`, fetches the remote, and records `git status --short --branch`, `git log -1 --oneline --decorate`, and `git remote -v`

#### Scenario: Handoff anchor differs from live HEAD
- **WHEN** the supplied handoff commit differs from the current branch HEAD
- **THEN** the operator reports the difference first and continues from live Git state unless total-control explicitly opens a rollback gate

### Requirement: OpenSpec And Codex Binding Exists
The repository MUST contain OpenSpec project structure and Codex OpenSpec skills so future sessions can propose, apply, validate, and archive changes through repo-local artifacts.

#### Scenario: OpenSpec setup is checked
- **WHEN** the repository is prepared on a new machine
- **THEN** `openspec/config.yaml`, `openspec/changes/`, `openspec/specs/`, and `.codex/skills/` exist and `openspec list --specs` runs successfully from the repository root

### Requirement: Total-Control Startup Gate Runs First
The total-control thread MUST run read-only Git checks and read the canonical governance documents before making planning or implementation claims.

#### Scenario: Total-control starts
- **WHEN** the total-control thread begins work
- **THEN** it runs `git status --short --branch`, `git log -1 --oneline --decorate`, and `git diff --stat`, then reads the repo rule and governance documents listed in the handoff

#### Scenario: Total-control validates before next gate
- **WHEN** total-control is ready to open a next gate
- **THEN** it runs the seed bundle wrapper, descriptor fixture wrapper, and Rust descriptor-validator tests, or reports exact blockers before proceeding

### Requirement: Four Branch Lanes Are Recreated
The project governance structure MUST use one total-control thread plus four branch lanes: source governance, schema/snapshot contract, router/FutureQA, and safety lint/leakage protection.

#### Scenario: Source governance lane starts
- **WHEN** the source governance lane starts
- **THEN** it performs read-only Git checks and reviews source acquisition, source review, freshness, source quality, effective confidence, and provenance locator boundaries without enabling runtime network fetch or automatic ingest

#### Scenario: Schema snapshot lane starts
- **WHEN** the schema/snapshot lane starts
- **THEN** it performs read-only Git checks and reviews schema, snapshot, activation descriptor, active pointer, last-known-good, rollback pointer, hash binding, and activation boundaries without touching Hope runtime

#### Scenario: Router FutureQA lane starts
- **WHEN** the router/FutureQA lane starts
- **THEN** it performs read-only Git checks and reviews QueryResult, RetrievalTrace, FutureQACandidate, EvalArtifact, stale blocking, and FutureQA promotion boundaries without treating FutureQACandidate as automatic eval truth

#### Scenario: Safety lint lane starts
- **WHEN** the safety lint/leakage lane starts
- **THEN** it performs read-only Git checks and reviews denied-field scanning, zero leakage, purge descriptors, rollback descriptors, refresh telemetry, and sanitized diagnostics without outputting matched values

### Requirement: Branch Reports Use The Relay Contract
Every branch lane MUST report back to total-control in a copy-ready text block with the required fields from the handoff.

#### Scenario: Branch lane reports
- **WHEN** a branch lane completes its read-only review
- **THEN** its report includes target thread, source thread, repository path, branch/HEAD, git status, completed work, unfinished work, modified files, validation result, commit/push status, risks/blockers, total-control decisions needed, and next recommendation

### Requirement: Safety Boundaries Remain Closed
The handoff process MUST preserve the current safety and product boundaries for HopePrompt KB v0.2.

#### Scenario: Runtime-facing data is discussed
- **WHEN** a future gate mentions runtime-facing KB context
- **THEN** the allowed context remains summary-only and excludes raw KB rows, raw prompt bodies, full source registers, overlay JSON, raw graph payloads, local paths, secrets, tokens, provider config, and request/response bodies

#### Scenario: Out-of-scope capabilities are proposed
- **WHEN** image generation, video generation, runtime network fetch, runtime auto-ingest, default LLM summarize, or default GraphRAG/hybrid/rerank is proposed
- **THEN** the proposal is rejected unless total-control opens a separate explicit exception gate

### Requirement: Next Gate Defaults To Source Governance
After alignment, total-control MUST prefer a docs-only plus validator-design gate for source acquisition, SourceDeltaBatch, and source quality scoring before runtime consumption.

#### Scenario: Alignment is accepted
- **WHEN** the OpenSpec handoff alignment is accepted
- **THEN** the next recommended gate is source acquisition / SourceDeltaBatch / source quality scoring, with runtime consumption deferred until source, snapshot, and activation contracts are stable
