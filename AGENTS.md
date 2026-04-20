# hope-kb Agent Rules

This repo is built through Codex-only parallel execution.

## Core rules

- Use Codex / Codex subagents only.
- This thread owns `hope-kb` work only; do not implement `hope` main-thread tasks here.
- Keep the `hope-kb` thread separate from the Hope RC main thread.
- Treat `docs/kb-freeze-contract.md`, `seed/v0.1/manifest.json`, `seed/v0.1/import_map.json`, and `migrations/0001_init_kb.sql` as the frozen contract surface for v0.1.
- Do not force-feed new KB scope back into `hope` until merge readiness is explicitly re-opened.
- Keep runtime hardening, taxonomy expansion, and repair coverage traceable to seed assets and validation scripts.

## Five-agent lanes

- Main thread: only integrator, only merge-readiness owner, only cross-repo sync owner with `hope`
- Lane 1: source register, source policy, provenance review
- Lane 2: taxonomy, director mappings, alias normalization
- Lane 3: failure patterns, repair mappings, prompt template hardening, degraded-input coverage
- Lane 4: case examples, export templates, validation chain, snapshot build, runtime-consumer handoff artifacts

## Delivery discipline

Every lane handoff must include:

1. change summary
2. seed files or schema touched
3. validation command actually run
4. blockers / merge risks
5. current canonical thread label
6. key-node reminder line when applicable

## Milestone status labels

The KB thread follows the same canonical milestone-label system as the main Hope project thread.

- Use the display format `线程名【状态标签】`.
- Use the shared policy in `E:\codex\ENGINEERING_THREAD_LABEL_POLICY.md`.
- Update the label whenever the KB thread crosses a key node such as: checkpoint accepted, standby entered, bounded feedback received, commit-ready, or environment-blocked.
- The label must reflect the real local KB state, not only the remote Git state.
- Every key-node handoff must end with:
  `关键节点提醒：请立即刷新线程标签、锚点提交、工作树状态和边界说明。`
- Keep one canonical label for the active KB thread and make sure it stays traceable to:
  1. repo path
  2. branch
  3. anchor commit
  4. clean / dirty worktree state
  5. one-line scope boundary

### Current KB canonical name and label

- `Hope-KB-检查点支持【检查点待命】`

## Main references

- `docs/project-thread-startup.md`
- `docs/parallel-execution-appendix.md`
- `docs/live-progress.md`
- `docs/kb-build-playbook-v0.1.md`
- `docs/kb-freeze-contract.md`
- `docs/kb-import-runtime-v0.1.md`
- `E:\codex\ENGINEERING_THREAD_LABEL_POLICY.md`
