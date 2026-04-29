## Context

The repository has been restored on this machine at `E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb` and is on `codex/contracts-freeze`. The handoff text named `a6b7e6d` as the last known source-machine HEAD, while live Git after clone and fetch resolves to `9391519`, which is four commits ahead and contains `a6b7e6d` in history.

The current project is a Hope-serving Prompt knowledge governance layer. Its first version centers on raw / wiki / schema / snapshot layers and Ingest / Query / Lint / Future QA operations. Runtime consumption remains future, gated, and summary-only.

The global Codex rule anchors are `E:\codex\AGENTS.md` and `E:\codex\hope-kb\docs\codex-usage-core-rules.md` at commit `9391519`. The relevant global additions are the persistent Core Challenger role and the Audit Specialist rule. In this repository, OpenSpec should carry those as gate-level supervisory requirements.

## Goals / Non-Goals

**Goals:**

- Make new-machine startup repeatable through OpenSpec artifacts in this repo.
- Preserve the required one total-control thread plus four branch-lane structure.
- Require every lane to begin with live Git checks before planning claims.
- Keep validation explicit: seed bundle wrapper, descriptor fixture wrapper, and Rust descriptor-validator tests.
- Use OpenSpec as total-control's assistant and supervisor for gate evidence, challenge questions, audit boundaries, validation status, and next-action traceability.
- Require total-control to route branch-lane dispatch drafts through one OpenSpec review round before issuing them by default.
- Require each major gate to record Core Challenger and Audit Specialist checks before total-control declares readiness.
- Capture the next recommended gate as source acquisition / SourceDeltaBatch / source quality scoring docs-only plus validator design.

**Non-Goals:**

- Do not roll the repository back from `9391519` to `a6b7e6d`.
- Do not start runtime implementation or Hope runtime integration.
- Do not add image generation, video generation, runtime network fetch, runtime auto-ingest, default LLM summarize, or default GraphRAG / hybrid / rerank.
- Do not modify validator semantics, seed assets, snapshots, fixtures, exports, or runtime payload contracts as part of this alignment.
- Do not let OpenSpec overrule total-control or branch-lane ownership.
- Do not let OpenSpec perform broad cleanup, implementation, staging, commit, or push by default.
- Do not commit or push by default.

## Decisions

### Treat Live Git As Authoritative

The setup records the mismatch between the supplied handoff anchor and the actual remote branch. It continues from `9391519` because the branch contains the supplied `a6b7e6d` commit and has advanced through additional docs-only rule commits.

Alternative considered: reset or checkout `a6b7e6d`. This was rejected because the handoff explicitly says to trust actual Git state when it differs.

### Encode Governance In OpenSpec

The relay structure is captured as an OpenSpec change rather than only in chat. This gives the next Codex session a discoverable proposal, design, requirements, and tasks tree.

Alternative considered: edit `AGENTS.md`. This was avoided because the repo rules prohibit committing `AGENTS.md` unless explicitly whitelisted.

### Keep This As Alignment-Only

The OpenSpec change is docs/control-plane alignment. It does not touch existing project behavior or validator implementation.

Alternative considered: immediately implement the next SourceDeltaBatch validator gate. This was rejected because the current user request is to align the project first.

### Make OpenSpec Total-Control's Supervisor Ledger

OpenSpec is assigned the assistant/supervisor role for total-control. It must record the current gate, required evidence, validation commands, branch-lane reports, Core Challenger falsification questions, Audit Specialist boundaries, and executable next actions.

Alternative considered: make OpenSpec a separate decision-making thread. This was rejected because total-control remains the decision owner; OpenSpec is the contract and evidence ledger.

### Require OpenSpec Review Before Dispatch

Total-control dispatches should follow this default sequence:

1. Total-control drafts the branch-lane instruction.
2. OpenSpec reviews the draft against current gate scope, global rules, branch responsibilities, prohibited surfaces, required evidence, validation commands, stop conditions, and report format.
3. OpenSpec returns one of three outcomes: `approved`, `revise`, or `blocked`.
4. If approved, OpenSpec provides a guided copy-ready dispatch block for total-control to send.
5. If revise or blocked, total-control updates the draft or records the block before issuing anything to branch lanes.

This review is required for normal branch-lane dispatch. Emergency user overrides may bypass it only when explicitly requested by the user and recorded as an override in the gate packet.

Alternative considered: allow total-control to dispatch first and let OpenSpec audit afterward. This was rejected because the user's requested default is pre-dispatch review.

The operational template is stored in `dispatch-review-packet.md` inside this OpenSpec change. It defines four blocks: total-control dispatch draft, OpenSpec review, OpenSpec-guided dispatch, and branch report intake.

### Map Global Rules Into Gate Checks

The global Core Challenger rule becomes a required "what would disprove this?" checkpoint for major milestones, readiness claims, and gate closure. The Audit Specialist rule becomes a required read-only boundary check that may list evidence and risks but may not implement, clean, delete, stage, commit, or push.

Alternative considered: keep global rules as external memory only. This was rejected because new machines need the rules discoverable in the repo-local OpenSpec workflow.

## Risks / Trade-offs

- [Risk] OpenSpec files are new repository artifacts and may need total-control approval before commit. -> Mitigation: leave worktree dirty and report exact files.
- [Risk] The project path is machine-local. -> Mitigation: use it only as a local handoff coordinate, not as runtime payload content.
- [Risk] The four branch lanes are encoded but not launched as real concurrent Codex threads in this turn. -> Mitigation: the spec requires lane startup checks and copy-ready reports before any next gate proceeds.
- [Risk] OpenSpec could be mistaken for an implementation authority. -> Mitigation: specs state that OpenSpec supervises evidence and gate discipline only; total-control remains the decision owner.
- [Risk] Global rules could be applied too broadly and interrupt active delivery. -> Mitigation: OpenSpec requires docs-only rule synchronization unless total-control opens a specific implementation gate.
- [Risk] Full validation may require Rust toolchain `1.95.0`. -> Mitigation: run the available lightweight OpenSpec/Git checks first and report any validator blocker separately.

## Migration Plan

1. Keep the cloned repository on `codex/contracts-freeze` at live HEAD.
2. Use OpenSpec/Codex skills from `.codex/skills/` for future proposal/apply/archive work.
3. Before implementation, total-control should run the required Git and validator commands.
4. Use OpenSpec to prepare a gate packet with evidence, Core Challenger questions, Audit Specialist boundary notes, validation status, and executable next actions.
5. Have OpenSpec review each total-control branch dispatch draft before issuing it by default.
6. Launch or reuse the four branch lanes only after total-control confirms the OpenSpec-reviewed dispatch.
7. Archive this OpenSpec change only after the handoff protocol and OpenSpec supervisor role are accepted as the canonical startup path.

## Open Questions

- Should the OpenSpec alignment files be committed on `codex/contracts-freeze`, or kept local until the next total-control commit gate?
- Should `AGENTS.md` later be repaired for encoding/readability, or left untouched under the current "do not commit AGENTS.md unless whitelisted" rule?
- Should the next source-governance gate be one OpenSpec change, or split into source acquisition, SourceDeltaBatch, and source quality scoring changes?
