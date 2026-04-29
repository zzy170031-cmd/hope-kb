## Why

New Codex machines need a reproducible handoff path for the HopePrompt knowledge base. The project already has governance documents and a Rust-first descriptor validator, but the relay structure must be encoded in-repo so future sessions continue from live Git truth instead of chat-only instructions.

## What Changes

- Add an OpenSpec change that captures the new-machine handoff governance contract.
- Bind this repository to OpenSpec and Codex project skills.
- Preserve the required one total-control thread plus four branch-lane structure.
- Define OpenSpec as total-control's assistant and supervisor, not the decision owner.
- Require total-control dispatch instructions to pass one OpenSpec review round before branch-lane delivery by default.
- Integrate the global Codex rule anchors, including Core Challenger and Audit Specialist expectations.
- Record the expected startup checks, validation commands, safety boundaries, and next gate priorities.
- Mark the supplied historical anchor `a6b7e6d` as a handoff reference while treating live Git HEAD as authoritative.

## Capabilities

### New Capabilities

- `new-machine-handoff-governance`: Defines how a fresh Codex environment restores the repository, validates live Git state, recreates total-control plus four branch-lane coordination, and chooses the next gate.
- `openspec-control-supervisor`: Defines how OpenSpec assists total-control by recording gate evidence, reviewing dispatch drafts, guiding branch-lane instructions, recording challenger questions, audit boundaries, validation status, and key-node reminders.

### Modified Capabilities

- None.

## Impact

- Adds OpenSpec project files under `openspec/`.
- Adds Codex OpenSpec skills under `.codex/skills/`.
- References global Codex rules anchored at `E:\codex\AGENTS.md` and `E:\codex\hope-kb\docs\codex-usage-core-rules.md` commit `9391519`.
- Does not change runtime code, seed data, snapshots, fixtures, validator rules, exports, or Hope runtime integration.
