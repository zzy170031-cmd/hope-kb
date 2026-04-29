# OpenSpec Dispatch Review Packet

This packet defines the default total-control dispatch flow for HopePrompt KB.

Default sequence:

1. Total-control writes a dispatch draft.
2. OpenSpec reviews the draft once before branch-lane delivery.
3. OpenSpec returns `approved`, `revise`, or `blocked`.
4. If approved, OpenSpec emits a guided dispatch block for total-control to send.
5. Branch-lane reports are re-absorbed through OpenSpec before the next action.

## 1. Total-Control Dispatch Draft

```text
total-control dispatch draft:
gate:
target thread:
thread action: new / reuse / restart / standby / close / archive
objective:
repository path:
branch / HEAD anchor:
live git checks required:
read scope:
write scope:
allowed actions:
prohibited actions:
required source documents:
validation commands:
stop conditions:
expected report format:
key-node reminder:
```

## 2. OpenSpec Review

```text
openspec dispatch review:
review outcome: approved / revise / blocked
gate alignment:
target lane alignment:
scope boundary:
global rule anchor check:
Core Challenger check:
Audit Specialist boundary check:
safety boundary check:
validation command check:
report format check:
missing evidence:
required corrections:
blocking reason:
guided next action:
```

Review rules:

- `approved`: The dispatch can be issued after total-control confirms.
- `revise`: Total-control must correct the draft before issuing it.
- `blocked`: Total-control must not issue the dispatch unless the user explicitly overrides and the override is recorded.

## 3. OpenSpec-Guided Dispatch

```text
branch-thread instruction:
thread action: new / reuse / restart / standby / close / archive
target thread:
gate:
objective:
repository path:
branch / HEAD anchor:
startup checks:
- git status --short --branch
- git log -1 --oneline --decorate
- git diff --stat
read scope:
write scope:
allowed actions:
prohibited actions:
required inputs:
validation commands:
stop conditions:
report format:
key-node reminder:
```

Read-only dispatches must include:

```text
No file edits.
No cleanup.
No staging.
No commit.
No push.
No runtime changes.
No UI changes.
No seed changes.
No fixture/export changes.
No raw KB rows, prompt bodies, full source registers, overlay JSON, raw graph payloads, local paths, secrets, tokens, provider config, or request/response bodies.
```

## 4. Branch Report Intake

```text
openspec branch report intake:
source thread:
original gate:
original dispatch reviewed: yes / no
repository path:
branch / HEAD:
git status:
completed:
unfinished:
modified files:
validation result:
commit / push status:
risks / blockers:
scope changed: yes / no
Core Challenger notes:
Audit Specialist notes:
total-control decision needed:
recommended next action:
```

If `scope changed` is `yes`, OpenSpec must require a new or revised dispatch review before the branch proceeds.
