# Prompt Knowledge Core v0.2

Status: total-control baseline, pending four lane reports.
Repo: `E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb`.
Branch anchor: `codex/contracts-freeze` at `52b61c7`.

## Purpose

This project remains a Hope support project. Its first version builds prompt
knowledge governance and graph QA for storyboard prompt optimization, then
hands Hope only verified, read-only snapshot contracts.

It is not an independent product, not a Hope desktop knowledge-management
surface, and not an image or video generation runtime.

## First Version Boundary

v0.2 covers prompt knowledge governance only:

- maintain reviewed prompt knowledge assets for Hope
- define the LLM Wiki `raw / wiki / schema / snapshot` layers
- define the `Ingest / Query / Lint / Future QA` operations
- keep QA contracts aligned with snapshot/source integrity, router/eval gates,
  ephemeral prompt safety, and validation/telemetry/leakage controls
- keep Hope runtime consumption summary-only and snapshot-bound

v0.2 does not:

- generate images
- generate videos
- connect image or video generation runtime
- runtime-fetch from the network
- runtime-auto-ingest user or external material
- default to GraphRAG, hybrid search, rerank, or runtime LLM summarization
- expose raw KB rows, raw `prompt_body`, full source register, overlay JSON, or
  secrets

## Knowledge Layers

### raw

The raw layer is intake evidence and reviewed source material before promotion.
It is not runtime-selectable. It may contain source handles, hashes, review
notes, and quarantine/candidate states, but it must not leak raw prompt bodies
or full source content to Hope runtime payloads.

### wiki

The wiki layer is curated human-readable knowledge. It captures why a prompt
sample, rule, repair mapping, or scene-intent pattern matters. It is useful for
review and handoff, but it is not itself the runtime contract.

### schema

The schema layer is the machine-readable contract layer. It defines required
fields, IDs, relationships, validation gates, source minimum fields, router/eval
metrics, leakage deny-lists, and snapshot activation descriptors.

### snapshot

The snapshot layer is the only future Hope consumption target. A snapshot is
eligible only when its manifest, snapshot artifact, runtime selection index,
counts, source eligibility, quarantine exclusion, and leakage gates are
verified.

## Operation Model

### Ingest

Ingest accepts reviewed material into the raw and wiki governance path. It must
record source identity, content hash or equivalent digest, review status,
effective confidence, and quarantine/candidate state when applicable.

Ingest does not automatically promote material into runtime selection.

### Query

Query means governed lookup against verified knowledge surfaces. v0.2 baseline
query is BM25-style local candidate pooling with explicit intent gating,
normalized score fields, deterministic fallback reasons, and summary-only
outputs.

Query does not emit full KB rows or perform runtime graph expansion.

### Lint

Lint checks structural safety before promotion or consumption. The minimum lint
surface includes source-field completeness, manifest/snapshot/index binding,
candidate and quarantine exclusion, router fallback determinism, eval freshness,
and leakage denial by structured field/path inspection.

### Future QA

Future QA records candidate improvements without changing v0.2 defaults. This
includes shadow-only hybrid/rerank exploration, compact evidence packs,
signature/provenance upgrades, maintenance UI ideas, and threshold calibration.
Those are v0.2.1+ topics unless control explicitly opens a new gate.

## Hope Consumption Contract

Hope runtime may later consume only verified snapshot outputs:

- `snapshot_version`
- `snapshot_hash`
- `selected_sample_ids`
- `selected_kb_rules`
- `fallback_reason_code`
- `payload_bytes`
- `kb_context_summary`

`full_kb_rows_included` must remain `0` for runtime prompt-routing payloads.

## Open Integration State

This document is a total-control baseline. It should be updated only after the
four branch threads report back in the required text-block format and control
decides whether their outputs are ready for unified staging.
