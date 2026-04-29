## ADDED Requirements

### Requirement: OpenSpec Supervises But Does Not Own Total-Control Decisions
OpenSpec MUST act as total-control's assistant and supervisor ledger for gates, evidence, and validation, while total-control remains the decision owner.

#### Scenario: Total-control opens a gate
- **WHEN** total-control opens a project gate
- **THEN** OpenSpec records the gate name, scope boundary, required evidence, validation commands, expected artifacts, branch-lane responsibilities, and explicit non-goals before implementation begins

#### Scenario: A decision is needed
- **WHEN** branch-lane reports or validation results require a decision
- **THEN** OpenSpec records the options, evidence, risks, and next actions, but does not mark the gate accepted until total-control decides

### Requirement: Total-Control Dispatches Pass OpenSpec Review By Default
Total-control branch-lane dispatch instructions MUST pass one OpenSpec review round before being issued by default.

#### Scenario: Total-control drafts a branch dispatch
- **WHEN** total-control prepares an instruction for a branch lane
- **THEN** the instruction remains a draft until OpenSpec reviews gate scope, target lane, allowed actions, prohibited actions, evidence requirements, validation commands, stop conditions, and report format

#### Scenario: OpenSpec approves a dispatch
- **WHEN** the dispatch draft matches the current gate and required boundaries
- **THEN** OpenSpec records `approved` and provides a guided copy-ready dispatch block for total-control to issue

#### Scenario: OpenSpec requires revision
- **WHEN** the dispatch draft is missing scope, evidence, validation, stop conditions, or report format
- **THEN** OpenSpec records `revise` with required corrections and total-control MUST NOT issue the dispatch until the draft is corrected or explicitly overridden by the user

#### Scenario: OpenSpec blocks a dispatch
- **WHEN** the dispatch draft crosses a prohibited surface, mixes multiple gates, skips live Git checks, bypasses safety boundaries, or asks a read-only lane to implement
- **THEN** OpenSpec records `blocked` with the blocking reason and a safer next action

### Requirement: OpenSpec-Guided Dispatch Uses A Fixed Packet
OpenSpec MUST guide total-control dispatches into a fixed packet that branch lanes can execute and report against.

#### Scenario: Guided dispatch is emitted
- **WHEN** OpenSpec approves a dispatch draft
- **THEN** the guided packet includes thread action, target thread, gate name, objective, repository path, branch/HEAD anchor, read/write scope, allowed actions, prohibited actions, required inputs, validation commands, stop conditions, report format, and key-node reminder

#### Scenario: Dispatch is read-only
- **WHEN** a lane is assigned read-only review
- **THEN** the guided packet explicitly says no file edits, no cleanup, no staging, no commit, no push, and no runtime/UI/seed/fixture/export changes

### Requirement: Branch Reports Are Re-absorbed Through OpenSpec
OpenSpec MUST be used to intake branch-lane reports before total-control opens the next action.

#### Scenario: Branch report arrives
- **WHEN** a branch lane reports back
- **THEN** OpenSpec checks the report against the original dispatch packet, records evidence, unresolved questions, validation status, Core Challenger pressure, Audit Specialist boundaries, and recommended total-control decision

#### Scenario: Branch report changes scope
- **WHEN** a branch report proposes work outside the reviewed dispatch packet
- **THEN** OpenSpec marks scope changed and requires a new or revised dispatch review before the branch proceeds

### Requirement: Core Challenger Check Is Required For Major Claims
OpenSpec MUST require a Core Challenger section before major milestone, readiness, gate opening, gate closure, or "done" claims are accepted.

#### Scenario: Readiness is claimed
- **WHEN** a thread claims a gate is ready, complete, or safe to proceed
- **THEN** OpenSpec records challenger questions, weakest-link analysis, evidence that could disprove the claim, synthesis or verdict, and executable next actions

#### Scenario: Evidence is insufficient
- **WHEN** live Git state, actual files, artifacts, tests, or user-visible behavior do not support the claim
- **THEN** OpenSpec records the conclusion as evidence insufficient, artifact loop not closed, governance not aligned, or not ready to declare done

### Requirement: Audit Specialist Boundary Is Required For Review Work
OpenSpec MUST require Audit Specialist boundaries for audit, cleanup, dead-code, fixture/export ownership, or health-review gates.

#### Scenario: Audit review starts
- **WHEN** an audit or cleanup-related gate starts
- **THEN** OpenSpec records that the audit is read-only unless total-control explicitly opens an implementation gate

#### Scenario: Audit report is produced
- **WHEN** an audit lane reports findings
- **THEN** the report lists evidence, risk, recommendation, and verification method without deleting, archiving, modifying runtime, modifying UI, modifying contracts, modifying KB, modifying fixtures, staging, committing, or pushing

### Requirement: Global Rule Anchors Are Part Of Gate Context
OpenSpec MUST reference the global Codex rule anchors when preparing total-control gate packets for this repository.

#### Scenario: Gate packet is prepared
- **WHEN** OpenSpec prepares or updates a gate packet
- **THEN** it references `E:\codex\AGENTS.md` and `E:\codex\hope-kb\docs\codex-usage-core-rules.md` at commit `9391519`, plus the live repository Git state

#### Scenario: Rule text conflicts with live state
- **WHEN** global rule notes, handoff text, or prior reports conflict with live Git or actual files
- **THEN** OpenSpec records the conflict and treats live Git and actual files as authoritative until total-control decides otherwise

### Requirement: Key-Node Reminder Is Required
OpenSpec MUST require a key-node reminder whenever a gate reaches freeze accepted, package completed, tests green, commit-ready, handoff-ready, environment-blocked, standby entered, or scope changed.

#### Scenario: Key node is reached
- **WHEN** the project crosses a key node
- **THEN** OpenSpec records a reminder to refresh the thread label, milestone anchor, worktree state, and boundary statement before the next package starts

### Requirement: Small-Block And Allowlist Discipline Is Required
OpenSpec MUST keep large or high-risk work split into small verifiable blocks and MUST require allowlist staging before commit.

#### Scenario: A change touches multiple surfaces
- **WHEN** a gate proposes changes across docs, validator, seed, fixtures, runtime, or UI
- **THEN** OpenSpec requires separate scoped tasks or separate gates, with local verification after each block

#### Scenario: Commit readiness is claimed
- **WHEN** total-control declares commit-ready
- **THEN** OpenSpec requires `git status --short --branch`, `git diff --stat`, `git diff --check`, the relevant validators, and allowlist staging instead of broad staging

### Requirement: Rule Synchronization Stays Docs-Only Unless Reopened
OpenSpec MUST keep global-rule synchronization docs-only unless total-control explicitly opens a different implementation gate.

#### Scenario: Global rules are synchronized
- **WHEN** global Codex rules are incorporated into a project gate
- **THEN** OpenSpec records that runtime, UI, KB seed, fixtures, exports, packaging, and install configuration remain untouched unless separately authorized
