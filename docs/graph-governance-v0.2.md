# Graph Governance v0.2

Status: graph QA contract baseline, not runtime GraphRAG.

## Purpose

The v0.2 graph is a governance graph for prompt knowledge QA. It helps control
reason about relationships among sources, samples, intents, rules, failure
patterns, repairs, eval artifacts, and snapshots.

It is not a runtime graph retrieval system and does not authorize GraphRAG,
hybrid search, rerank defaults, vector indexes, network lookup, or model-side
expansion.

## Governed Nodes

The first graph QA layer may reason about these node classes:

- source handle
- source review decision
- prompt sample
- scene or shot intent
- KB rule
- failure pattern
- repair mapping
- eval case
- leakage or lint rule
- snapshot descriptor
- runtime selection index descriptor

The graph may contain references and IDs, but consumer-facing outputs must stay
bounded to sanitized summaries and stable identifiers.

## Governed Edges

Useful v0.2 edge types:

- source supports sample
- sample maps to intent
- sample activates rule
- failure maps to repair
- eval case covers intent
- lint rule blocks artifact
- snapshot includes eligible sample
- index exposes runtime selection surface

Edges should be checkable through seed records, schema docs, validation output,
or lane handoff evidence. They should not depend on hidden model judgment at
runtime.

## QA Uses

Graph governance can support:

- source integrity review
- candidate/quarantine isolation checks
- intent coverage review
- failure/repair completeness checks
- per-intent eval coverage
- leakage deny-list path inspection
- snapshot activation readiness
- Future QA issue tracking

Graph governance must not:

- retrieve raw graph neighborhoods into prompts
- expose full raw KB rows
- expose full source register
- expose raw `prompt_body`
- select reserve or negative fixtures as positive few-shot samples
- use graph traversal to bypass BM25 fallback gates
- open runtime LLM summarization

## Runtime Boundary

The runtime boundary stays summary-only. A future Hope consumer may receive
sanitized IDs and summaries after snapshot verification, but it must not receive
raw graph objects or graph traversal dumps.

Allowed runtime-facing fields remain:

- `snapshot_version`
- `snapshot_hash`
- `selected_sample_ids`
- `selected_kb_rules`
- `fallback_reason_code`
- `payload_bytes`
- `kb_context_summary`

## Acceptance Gates

Before graph-derived QA can affect snapshot or router eligibility, control must
have machine-checkable evidence for:

- no candidate or quarantine node is reachable from active runtime selection
- every selected sample has accepted source support or an approved exception
- every configured intent meets the eval sample minimum
- failure/repair edges do not point to missing sample IDs
- leakage lint paths reject raw rows, raw graph, full prompt body, full source
  register, local paths, and credential fields
- graph QA output remains sanitized and does not include raw evidence text

## Deferred Topics

These are v0.2.1+ unless reopened:

- compact graph evidence packs
- reviewer UI for graph QA
- signature chain over graph descriptors
- shadow-only graph-assisted routing
- calibrated graph coverage scoring

Any deferred graph work must preserve the v0.2 rule that Hope consumes verified
snapshots, not live governance internals.
