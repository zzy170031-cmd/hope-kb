# Source Governance Docs Intake

Intake date: 2026-04-30

Source thread: `(running) HopePrompt KB - source acquisition and knowledge governance`

Original dispatch: `source-governance-docs-dispatch.md`

Repository: `E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb`

Live Git reported by lane:

- branch: `codex/contracts-freeze`
- HEAD: `9391519`
- status: tracked docs modified only in the approved write scope; untracked alignment artifacts `.codex/` and `openspec/`

## Intake Result

Status: accepted for total-control review.

The lane satisfied the approved docs-only dispatch:

- updated `docs/prompt-knowledge-source-governance-v0.2.md`
- updated `docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md`
- kept the update docs-only
- did not change Rust code, fixtures, seed, snapshot, runtime, UI, export, staging, commit, or push
- ran the required Git, diff, and targeted `rg` checks

## Total-Control Verification

Total-control independently verified:

- `git status --short --branch` shows only the two approved tracked docs modified, plus untracked `.codex/` and `openspec/`
- `git diff --stat` shows 2 files changed with 179 insertions
- `git diff --name-status` is limited to the approved docs
- `git diff --check -- docs/prompt-knowledge-source-governance-v0.2.md docs/prompt-knowledge-descriptor-validator-matrix-v0.2.md` passes
- `openspec validate align-new-machine-handoff-governance --strict --no-interactive` passes before this intake update

## Accepted Docs Content

Accepted in the source governance document:

- Source acquisition is bounded to offline governance intake and human-controlled review.
- Runtime fetch, auto-ingest, user-session writes, query-time raw/wiki maintenance, direct seed mutation, snapshot mutation, and runtime selection expansion remain prohibited.
- The source quality scoring rubric is a review aid only.
- Source quality scoring cannot auto-promote source, wiki, seed candidate, eval truth, snapshot, index, active pointer, or runtime selection.
- Quality labels remain categorical for v0.2: `high`, `medium`, `low`, `unknown`, and `rejected`.

Accepted in the descriptor validator matrix document:

- SourceDeltaBatch remains a governance descriptor design.
- SourceDeltaBatch future validation is aggregate-only and uses batch hash, count, and freshness digest rather than runtime locator evidence.
- Future fixture matrix entries are design notes only; no fixture files were created.
- Sanitized diagnostics must avoid matched values, raw source excerpts, locator values, credential-like values, provider details, and request/response bodies.
- Future Rust-first implementation is limited to an offline SourceDeltaBatch static rule module, dispatcher registration, synthetic fixtures, and targeted tests after a separate implementation gate opens.

## Core Challenger Notes

- This update improves docs/design evidence only; it does not make SourceDeltaBatch validation implemented.
- The fixture matrix is not a claim that Lane 1 fixtures exist.
- The quality rubric remains a review aid and does not provide an activation path.
- Future implementation must still prove required-field checks, hash shape checks, count consistency, denied-field behavior, leakage zero, runtime exclusion, and unknown-field fail-closed behavior through code and synthetic fixtures.

## Audit Specialist Notes

- The docs update stayed inside the approved write scope.
- No runtime, UI, seed, snapshot, fixture, export, staging, commit, or push action occurred.
- The next implementation gate must continue to prohibit source fetching, source storage, raw source text, per-source locator values, full source register details, matched values, secrets, provider config, and request/response body output.

## Total-Control Decision

Accept the Lane 1 docs-only update.

Recommended next action:

- prepare an OpenSpec-reviewed dispatch for a narrow Rust-first SourceDeltaBatch static validator implementation gate
- keep source acquisition implementation, source fetching, seed mutation, snapshot rebuild, runtime consumption, UI, export, canonical digest recomputation, verified fallback exceptions, and wrapper/toolchain changes outside that gate
