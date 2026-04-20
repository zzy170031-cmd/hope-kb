# hope-kb

This is the system knowledge repository for Hope.

It holds:

- director profiles
- committee definitions
- prompt templates
- visual language vocabulary
- camera work vocabulary
- continuity rules
- transition vocabulary
- export templates
- failure patterns
- seed import mapping and validation contracts

The app consumes a validated read-only snapshot of this repo.
This KB exists to support Hope main-thread execution and outputs; it does not replace or absorb `hope` main-thread implementation work.

## Primary docs

- `AGENTS.md`
- `docs/project-thread-startup.md`
- `docs/parallel-execution-appendix.md`
- `docs/live-progress.md`
- `docs/kb-overview.md`
- `docs/kb-build-playbook-v0.1.md`
- `docs/kb-freeze-contract.md`
- `docs/kb-seed-checklist-v0.1.md`
- `docs/kb-seed-v0.1.md`
- `docs/kb-source-policy-v0.1.md`
- `docs/kb-source-index-v0.1.md`
- `docs/director-reference-index-v0.1.md`
- `seed/v0.1-seed-manifest.md`
- `seed/v0.1/source_register.json`
- `seed/v0.1/import_map.json`
- `scripts/validate-seed-bundle.ps1`
- `scripts/build-kb-snapshot.py`

## Threading rule

- Keep `hope-kb` on a dedicated KB hardening thread.
- Keep the Hope RC thread separate until merge readiness is explicitly reopened.
- Use `docs/live-progress.md` as the online sync surface when another person needs real-time status.
- Treat `hope-kb` as a support surface for `hope`: knowledge contracts, validation, and snapshot artifacts live here; main-thread feature work stays in `hope`.
