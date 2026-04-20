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

## Main references

- `docs/project-thread-startup.md`
- `docs/parallel-execution-appendix.md`
- `docs/live-progress.md`
- `docs/kb-build-playbook-v0.1.md`
- `docs/kb-freeze-contract.md`
- `docs/kb-import-runtime-v0.1.md`
