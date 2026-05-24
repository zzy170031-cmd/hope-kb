## 1. Repository Restore

- [x] 1.1 Create `E:\codex-projects\hope-prompt-kb-v0.2-independent`.
- [x] 1.2 Clone `https://github.com/zzy170031-cmd/hope-kb.git` into `hope-kb`.
- [x] 1.3 Check out `codex/contracts-freeze`.
- [x] 1.4 Run `git fetch --all --prune`.
- [x] 1.5 Record live Git state and report the `a6b7e6d` handoff anchor versus `9391519` live HEAD mismatch.

## 2. OpenSpec And Codex Binding

- [x] 2.1 Initialize OpenSpec in the repository.
- [x] 2.2 Configure Codex OpenSpec skills under `.codex/skills/`.
- [x] 2.3 Add project context to `openspec/config.yaml`.
- [x] 2.4 Create the `align-new-machine-handoff-governance` OpenSpec change.

## 3. Handoff Governance Contract

- [x] 3.1 Capture restore, Git truth, and mismatch handling in the proposal and design.
- [x] 3.2 Capture total-control startup checks and required validation commands.
- [x] 3.3 Capture the four branch lanes and their prohibited actions.
- [x] 3.4 Capture required branch-lane report format.
- [x] 3.5 Capture safety boundaries and out-of-scope runtime/product capabilities.
- [x] 3.6 Capture the next recommended gate: source acquisition / SourceDeltaBatch / source quality scoring docs-only plus validator design.

## 4. OpenSpec Supervisor Contract

- [x] 4.1 Define OpenSpec as total-control assistant and supervisor, not the decision owner.
- [x] 4.2 Integrate global rule anchors from `E:\codex\AGENTS.md` and `E:\codex\hope-kb\docs\codex-usage-core-rules.md` at `9391519`.
- [x] 4.3 Require Core Challenger falsification checks for major claims and gate closure.
- [x] 4.4 Require Audit Specialist read-only boundaries for audit and cleanup gates.
- [x] 4.5 Require key-node label reminders, small-block verification, and allowlist staging.
- [x] 4.6 Require rule synchronization to stay docs-only unless total-control opens another gate.

## 5. Dispatch Review Protocol

- [x] 5.1 Require total-control branch-lane dispatch drafts to pass one OpenSpec review round by default.
- [x] 5.2 Define OpenSpec review outcomes: `approved`, `revise`, and `blocked`.
- [x] 5.3 Define fixed OpenSpec-guided dispatch packet fields.
- [x] 5.4 Require read-only dispatches to state no edits, cleanup, staging, commit, push, runtime, UI, seed, fixture, or export changes.
- [x] 5.5 Require branch-lane reports to be re-absorbed through OpenSpec before total-control opens the next action.
- [x] 5.6 Require scope changes to trigger a new or revised OpenSpec dispatch review.
- [x] 5.7 Add `dispatch-review-packet.md` as the reusable draft/review/dispatch/intake template.

## 6. Verification

- [x] 6.1 Run `openspec status --change align-new-machine-handoff-governance --json`.
- [x] 6.2 Run `openspec validate align-new-machine-handoff-governance --strict --no-interactive`.
- [x] 6.3 Run `git status --short --branch`.
- [x] 6.4 Run `git diff --stat`.
- [x] 6.5 Report that no commit or push was performed.

## 7. Branch Report Intake

- [x] 7.1 Intake Lane 1 source governance report.
- [x] 7.2 Intake Lane 2 schema/snapshot report.
- [x] 7.3 Intake Lane 3 router/FutureQA report with target-path evidence caveat.
- [x] 7.4 Intake Lane 4 safety lint report with target-path evidence caveat.
- [x] 7.5 Record total-control synthesis and candidate next gates.

## 8. Source Governance Dispatch

- [x] 8.1 Prepare total-control dispatch draft for Lane 1 source governance design gate.
- [x] 8.2 Run OpenSpec dispatch review for the Lane 1 draft.
- [x] 8.3 Record approved OpenSpec-guided dispatch in `source-governance-dispatch.md`.

## 9. Source Governance Design Intake And Docs Dispatch

- [x] 9.1 Intake Lane 1 source governance design report.
- [x] 9.2 Record total-control decisions for the next source governance docs-only gate.
- [x] 9.3 Keep SourceDeltaBatch Rust rules, fixture creation, source acquisition implementation, and quality scoring activation deferred.
- [x] 9.4 Prepare and approve OpenSpec-reviewed docs-only dispatch in `source-governance-docs-dispatch.md`.

## 10. Source Governance Docs Intake

- [x] 10.1 Intake Lane 1 docs-only update report.
- [x] 10.2 Verify that modified tracked files are limited to the approved docs write scope.
- [x] 10.3 Record total-control acceptance of the docs-only update in `source-governance-docs-intake.md`.
- [x] 10.4 Keep SourceDeltaBatch Rust implementation, fixture creation, source acquisition implementation, and quality scoring activation deferred until a separate dispatch review.

## 11. SourceDeltaBatch Validator Implementation Dispatch

- [x] 11.1 Prepare total-control dispatch draft for a narrow Rust-first SourceDeltaBatch static validator implementation gate.
- [x] 11.2 Run OpenSpec dispatch review for the implementation draft.
- [x] 11.3 Record approved OpenSpec-guided implementation dispatch in `source-delta-validator-implementation-dispatch.md`.
- [x] 11.4 Keep source acquisition, source fetching, seed, snapshot, runtime, UI, export, wrapper/toolchain repair, canonical digest recomputation, and verified fallback exceptions closed.

## 12. SourceDeltaBatch Validator Implementation Intake

- [x] 12.1 Intake Lane 1 SourceDeltaBatch static validator implementation report.
- [x] 12.2 Verify live Git state and the explicit untracked SourceDeltaBatch module and fixture files.
- [x] 12.3 Verify `git diff --check`, cargo fmt check, cargo test, descriptor fixture matrix, and OpenSpec validation.
- [x] 12.4 Record total-control acceptance and residual hardening notes in `source-delta-validator-implementation-intake.md`.
- [x] 12.5 Keep source acquisition, source storage, source fetching, cross-descriptor binding, seed, snapshot, runtime, UI, export, canonical digest recomputation, verified fallback exceptions, and wrapper/toolchain repair closed.

## 13. Source To Wiki Minimal Loop Dispatch

- [x] 13.1 Prepare total-control dispatch draft for an offline source acquisition to reviewed wiki minimal loop design gate.
- [x] 13.2 Run OpenSpec dispatch review for the read-only design draft.
- [x] 13.3 Record approved OpenSpec-guided dispatch in `source-to-wiki-loop-dispatch.md`.
- [x] 13.4 Keep file edits, source fetching, source storage, seed, snapshot, runtime, UI, export, validator changes, fixtures, staging, commit, and push closed for the target lane.

## 14. Source To Wiki Minimal Loop Intake And Contract Dispatch

- [x] 14.1 Intake Lane 1 source-to-wiki minimal loop design report.
- [x] 14.2 Record accepted loop design, Core Challenger notes, and Audit Specialist boundaries in `source-to-wiki-loop-intake.md`.
- [x] 14.3 Prepare and approve OpenSpec-reviewed docs-only artifact contract dispatch in `source-wiki-artifact-contract-dispatch.md`.
- [x] 14.4 Keep artifact creation, source fetching/storage, seed, snapshot, runtime, UI, export, validator changes, fixtures, staging, commit, and push closed for the target lane.

## 15. KB Product Chain And PWA Snapshot Contract

- [x] 15.1 Fast-forward `E:\codex\hope-kb` to `origin/codex/contracts-freeze` at `3df5bd6`.
- [x] 15.2 Restate the project requirement as a reviewed wiki / graph KB that publishes summary-only runtime snapshots to `hope-web-pwa`.
- [x] 15.3 Add `docs/kb-product-chain-contract-v0.2.md` with KB/PWA responsibilities, runtime snapshot boundary, mapping completeness rules, leakage boundaries, knowledge content plan, and release gates.
- [x] 15.4 Add `schemas/runtime-kb-snapshot.schema.json` and `schemas/wiki-to-runtime-mapping.schema.json`.
- [x] 15.5 Add `samples/runtime-kb-snapshot.sample.json` and `samples/wiki-to-runtime-mapping.sample.json`.
- [x] 15.6 Add `docs/knowledge-plan-v0.2.md` with the knowledge-domain plan and document inventory.
- [x] 15.7 Run JSON parse checks, light sample validation, targeted anchor search, `git diff --check`, and `git status --short --branch`.

## 16. Batch 1 Reviewed Wiki Prototype And Closed-Loop Repair

- [x] 16.1 Add `docs/knowledge-intake-candidate-list-v0.2.md` with the approved Batch 1 and deferred Batch 2 knowledge entries.
- [x] 16.2 Add `docs/product-action-crosswalk-v0.2.md` to keep KB actions aligned with `hope-web-pwa` and adjacent Hope production artifacts.
- [x] 16.3 Add eight Batch 1 reviewed wiki prototype pages under `knowledge/reviewed_wiki/`.
- [x] 16.4 Add Batch 1 prototype mapping at `knowledge/mappings/wiki-to-runtime-mapping.v0.2.json`.
- [x] 16.5 Add Batch 1 prototype runtime snapshot at `knowledge/runtime_snapshots/latest.candidate.json`.
- [x] 16.6 Align Batch 1 IDs across knowledge plan, candidate list, reviewed wiki pages, mapping, and runtime snapshot.
- [x] 16.7 Add structure-level `action_results`, mapping item version, field-path, duration-target, fallback, and sanitization fields to schemas.
- [x] 16.8 Add `scripts/validate-kb-runtime-prototype.ps1` for duration, action, rule-pack, reviewed-wiki, mapping, and leakage-boundary checks.
- [x] 16.9 Run prototype validation and confirm 8 reviewed wiki pages, 8 mappings, 10 rule packs, 6 durations, and 10 KB actions pass.

## 17. PWA Adapter And Flow Dashboard Prototype

- [x] 17.1 Add `docs/pwa-adapter-contract-v0.2.md` to define the KB snapshot to `hope-web-pwa` adapter boundary.
- [x] 17.2 Add `schemas/pwa-kb-adapter-output.schema.json` for the wrapped PWA adapter output.
- [x] 17.3 Add `samples/pwa-kb-adapter-output.sample.json` with camelCase `KbSnapshot`-shaped data and fail-closed metadata.
- [x] 17.4 Add `scripts/build-pwa-kb-adapter-output.js` so the adapter sample can be regenerated and checked for drift.
- [x] 17.5 Extend `scripts/validate-kb-runtime-prototype.ps1` to validate adapter status, partial scene catalog boundary, camelCase PWA fields, rule-pack references, duration profiles, summary-only boundary, and fail-closed metadata.
- [x] 17.6 Add `web/kb-flow-dashboard/index.html` as a Chinese-first visual flow dashboard showing Hope demand, role knowledge, reviewed wiki, mapping, PWA KB snapshot, adapter flow, and local draft additions.
- [x] 17.7 Add `scripts/serve-kb-flow-dashboard.js` for local dashboard preview at `http://127.0.0.1:5179/`.
- [x] 17.8 Run `node scripts/build-pwa-kb-adapter-output.js --check`, `scripts/validate-kb-runtime-prototype.ps1`, dashboard script syntax check, preview HTTP 200 check, and `git diff --check`.
- [x] 17.9 Add `docs/kb-pwa-integration-progress-2026-05-23.md` as the current progress, evidence, non-claims, and next-gate report.
