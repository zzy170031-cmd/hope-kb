# Hope-KB to hope-web-pwa Integration Progress

Date: 2026-05-23
Status: Batch 1 prototype complete; product PWA integration not yet released.

## Requirement Restatement

Hope-KB is a controlled reviewed wiki and graph knowledge repository for Hope
content production. It supports better story expansion, rewriting, storyboard
task creation, storyboard row generation, prompt text packaging, validation,
repair, and export safety.

Hope-KB must stay behind the Hope product workflow. It is not the user-facing
product center, not a standalone graph UI, and not a replacement for
`hope-web-pwa`. Its product-facing output is a summary-only runtime snapshot
and adapter output that `hope-web-pwa` can consume through an explicit loader or
compiler boundary.

## Current Proven State

The repository is synchronized with `origin/codex/contracts-freeze` at
`3df5bd6`; `git rev-list --left-right --count HEAD...origin/codex/contracts-freeze`
returned `0 0` after fetch.

The current Batch 1 prototype provides:

- product-chain contract:
  `docs/kb-product-chain-contract-v0.2.md`
- knowledge plan and content inventory:
  `docs/knowledge-plan-v0.2.md`
- approved Batch 1 and deferred Batch 2 candidate list:
  `docs/knowledge-intake-candidate-list-v0.2.md`
- PWA action/type alignment note:
  `docs/product-action-crosswalk-v0.2.md`
- PWA adapter boundary:
  `docs/pwa-adapter-contract-v0.2.md`
- runtime snapshot schema:
  `schemas/runtime-kb-snapshot.schema.json`
- wiki-to-runtime mapping schema:
  `schemas/wiki-to-runtime-mapping.schema.json`
- PWA adapter output schema:
  `schemas/pwa-kb-adapter-output.schema.json`
- runtime snapshot sample:
  `samples/runtime-kb-snapshot.sample.json`
- wiki-to-runtime mapping sample:
  `samples/wiki-to-runtime-mapping.sample.json`
- PWA adapter output sample:
  `samples/pwa-kb-adapter-output.sample.json`
- eight reviewed wiki Batch 1 entries:
  `knowledge/reviewed_wiki/*.md`
- Batch 1 mapping:
  `knowledge/mappings/wiki-to-runtime-mapping.v0.2.json`
- Batch 1 candidate runtime snapshot:
  `knowledge/runtime_snapshots/latest.candidate.json`
- prototype validator:
  `scripts/validate-kb-runtime-prototype.ps1`
- adapter output generator:
  `scripts/build-pwa-kb-adapter-output.js`
- local visual flow dashboard:
  `web/kb-flow-dashboard/index.html`
- local dashboard preview server:
  `scripts/serve-kb-flow-dashboard.js`

## Batch 1 Coverage

Reviewed wiki entries:

1. `rw-writing-continuity-core`
2. `rw-scene-expression-visible-action`
3. `rw-director-scheduling-core`
4. `rw-shot-intent-taxonomy`
5. `rw-duration-density-rules`
6. `rw-prompt-text-boundary`
7. `rw-validation-no-pseudo-success`
8. `rw-validation-no-leakage`

Covered runtime dimensions:

- 8 reviewed wiki pages
- 8 wiki-to-runtime mappings
- 10 rule packs
- 6 allowed durations: `5`, `10`, `15`, `30`, `45`, `60`
- 10 KB actions:
  `import_source`, `expand_story`, `rewrite_story`, `accept_story_body`,
  `create_story_task`, `generate_storyboard`, `repair_storyboard`,
  `validate_result`, `export_result`, `golden_sample_review`

## PWA Adapter Boundary

Batch 1 does not claim that `hope-web-pwa` already consumes the external KB
asset. It proves the KB-side artifact chain and provides a generated adapter
output shaped for the PWA contract:

```text
reviewed_wiki
-> wiki-to-runtime mapping
-> runtime-kb-snapshot candidate
-> PWA adapter output
-> future PWA loader/compiler
-> PWA KbSnapshot/SanitizedKbSummary
-> Hope content output
```

The adapter output intentionally declares:

- `adapter_status = prototype`
- `scene_catalog_status = partial`
- `rule_pack_crosswalk_status = prototype`
- `fail_closed.fallback_required = true`

This prevents the Batch 1 sample from being mistaken for full product catalog
coverage or a production release artifact.

## Validation Evidence

The current verified command set is:

```text
git fetch origin
git rev-list --left-right --count HEAD...origin/codex/contracts-freeze
node scripts/build-pwa-kb-adapter-output.js --check
powershell.exe -ExecutionPolicy Bypass -File scripts/validate-kb-runtime-prototype.ps1
node -e "<dashboard script syntax check>"
Invoke-WebRequest -Uri http://127.0.0.1:5179/ -UseBasicParsing
git diff --check
```

Observed validator result:

```json
{
  "status": "passed",
  "reviewed_wiki_count": 8,
  "mapping_count": 8,
  "rule_pack_count": 10,
  "duration_count": 6,
  "action_count": 10,
  "pwa_adapter_status": "prototype",
  "pwa_adapter_scene_catalog_status": "partial"
}
```

## Not Yet Claimed

The following are not claimed complete:

- production PWA loader implementation
- full PWA scene catalog coverage
- real non-sample hash recomputation and verification
- release-candidate or activated runtime snapshot
- product-side last-known-good/fail-closed tests
- prompt/UI/trace/export leakage tests inside `hope-web-pwa`
- provider, IPC, desktop runtime, export, or packaging readiness

## Next Product Gate

The next gate should be a narrow `hope-web-pwa` integration gate:

1. Add a PWA loader or compiler that accepts the adapter output.
2. Keep built-in `KB_SNAPSHOT` as fallback.
3. Reject partial catalog artifacts unless the product path explicitly allows
   prototype scope.
4. Add type tests for `KbSnapshot` and `SanitizedKbSummary`.
5. Add fail-closed tests for missing, invalid, stale, partial, and unsafe KB
   assets.
6. Add leakage tests covering prompts, UI traces, and exports.
7. Only after that, decide whether to promote Batch 1 beyond prototype.
