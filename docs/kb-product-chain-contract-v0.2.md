# KB Product Chain Contract v0.2

Status: product-chain contract baseline.
Scope: `hope-kb` reviewed wiki and graph governance content compiled into a
summary-only runtime KB snapshot for `hope-web-pwa`.

This contract defines how the knowledge repository supports the browser PWA
without exposing raw governance material. It is the bridge between the rich
`hope-kb` wiki/graph layer and the narrow runtime fields already used by
`hope-web-pwa`.

## Project Requirement Restatement

Hope-KB is a controlled knowledge graph and wiki repository for Hope. Its job
is to continuously collect, review, structure, validate, and publish prompt
knowledge that improves story expansion, story rewriting, storyboard task
creation, storyboard row generation, prompt text packaging, validation, repair,
and export safety.

Hope-KB is a support capability for Hope content production. It must not become
the product center, a standalone knowledge-management app, a graph UI project,
or a user-facing destination. Its success is measured by better Hope outputs:
more coherent story bodies, more usable storyboard rows, cleaner prompt text,
safer validation, and safer exports.

The target product consumer is `hope-web-pwa`. The PWA must not read raw KB
rows, source registers, raw wiki pages, raw graph neighborhoods, prompt bodies,
overlay JSON, local paths, provider configuration, API keys, or other internal
governance evidence. It may consume only a verified runtime KB snapshot whose
fields match the PWA `KbSnapshot` and `SanitizedKbSummary` contracts.

The intended product chain is:

```text
source_intake
-> wiki_draft
-> reviewed_wiki
-> graph node / edge governance
-> wiki-to-runtime mapping
-> runtime-kb-snapshot.json
-> hope-web-pwa public/kb/latest.json
-> buildSanitizedKbSummary()
-> story / storyboard / prompt / export workflow
```

The first release target is not a live runtime graph, GraphRAG, runtime web
lookup, auto-ingest, UI knowledge browser, or user-editable wiki. The first
release target is a deterministic, reviewable, hash-bound, summary-only JSON
snapshot that the PWA can load safely.

The PWA and any future Hope runtime remain the content production surface.
Knowledge artifacts stay behind the scenes unless they are compiled into the
summary-only runtime snapshot.

## Product Support Responsibilities

### `hope-kb`

`hope-kb` owns:

- source intake and quarantine governance
- wiki draft and reviewed wiki content
- graph node and edge semantics
- writing, director, validation, failure, repair, and FutureQA knowledge
- mapping reviewed knowledge into runtime rule packs
- snapshot schema and mapping schema
- validation of coverage, leakage boundaries, freshness, and hash bindings
- last-known-good and fail-closed release discipline

### `hope-web-pwa`

`hope-web-pwa` owns:

- loading a runtime KB snapshot or falling back to a built-in minimal snapshot
- consuming only `KbSnapshot` and `SanitizedKbSummary` shaped data
- preserving summary-only guarantees in prompts, traces, UI, and exports
- producing QA trace signals that can feed future KB failure and repair work
- refusing raw KB rows, raw source text, prompt bodies, source registers,
  local paths, internal hashes, secrets, and internal graph dumps in user output

## Runtime Snapshot Contract

The PWA-facing runtime snapshot is the only product handoff artifact.

Required top-level fields:

```text
snapshot_version
snapshot_hash
schema_version
generated_at_bucket
source_delta_batch_hash
source_freshness_digest
activation_status
freshness_status
scene_types
allowed_durations
writing_rule_packs
director_rule_packs
validation_rule_packs
scene_mappings
duration_profiles
runtime_safety
coverage_matrix
```

The snapshot must compile into PWA-compatible data:

```text
KbSnapshot.sceneTypes
KbSnapshot.allowedDurations
KbSnapshot.writingRulePacks
KbSnapshot.directorRulePacks
KbSnapshot.validationRulePacks
KbSnapshot.sceneMappings
KbSnapshot.durationProfiles
SanitizedKbSummary.selected_sample_ids
SanitizedKbSummary.selected_kb_rules
SanitizedKbSummary.kb_context_summary
SanitizedKbSummary.negative_constraints
SanitizedKbSummary.action_results
```

Runtime safety constants:

```text
raw_kb_rows_included = 0
raw_sample_text_absent = true
source_register_absent = true
overlay_json_absent = true
prompt_body_absent = true
raw_graph_absent = true
local_paths_absent = true
secrets_absent = true
runtime_graph_lookup_used = false
runtime_auto_ingest_used = false
runtime_llm_summarize_used = false
```

`action_results.state=ready` means the KB support surface is ready for the
prototype snapshot. It does not prove that a PWA button, IPC command, provider
call, export engine, or desktop capability is runnable.

Prototype hashes that contain `sample-` are placeholders. A production release
gate must replace them with recomputed content hashes and must fail closed when
hashes do not match the current reviewed wiki, mapping, snapshot, and
sanitized summary fragments.

## Wiki To Runtime Mapping Contract

Every runtime-eligible reviewed wiki entry must be mapped before it can affect
the PWA snapshot.

Minimum mapping fields:

```text
mapping_id
mapping_version
reviewed_wiki_id
reviewed_wiki_hash
wiki_type
runtime_targets
scene_type_ids
kb_actions
rule_group
runtime_rule_pack_ids
selected_kb_rule_ids
summary_fragment_ids
negative_constraint_ids
coverage_status
review_status
effective_confidence
runtime_eligible
leakage_count
```

Allowed `wiki_type` values:

```text
writing_continuity_rule
scene_expression_rule
director_scheduling_rule
shot_language_rule
validation_rule
failure_pattern
repair_mapping
future_qa_candidate
source_policy_note
```

Allowed `runtime_targets`:

```text
writing_rule_packs
director_rule_packs
validation_rule_packs
scene_mappings
duration_profiles
selected_kb_rules
kb_context_summary
negative_constraints
action_results
future_qa_backlog
```

Allowed `kb_actions`:

```text
import_source
expand_story
rewrite_story
accept_story_body
create_story_task
generate_storyboard
repair_storyboard
validate_result
export_result
golden_sample_review
```

## Completeness Guarantees

Completeness is enforced by mapping and validation, not by manual memory.

The runtime snapshot is incomplete unless all of the following are true:

- every `runtime_eligible=true` reviewed wiki entry maps to at least one
  runtime target
- every runtime rule pack traces back to one or more reviewed wiki IDs
- every PWA scene type has writing, director, and validation coverage
- every PWA KB action has at least one supporting rule or an explicit fallback
- every selected KB rule ID resolves to a runtime rule pack or summary fragment
- every failure pattern either maps to a repair mapping or is marked as
  FutureQA-only
- every repair mapping names the failure pattern it addresses
- every duration profile is covered by at least one pacing or density rule
- every summary fragment is sanitized and does not contain raw source text
- every snapshot has a stable `snapshot_hash`
- every release has a last-known-good fallback plan

## Leakage And Boundary Guarantees

The following must never appear in the PWA-facing snapshot, prompts, UI, logs,
telemetry, export files, or QA artifacts:

- raw KB rows
- raw source text
- raw prompt body or full prompt body
- source register dumps or source register paths
- local absolute paths
- raw graph neighborhoods
- overlay JSON
- provider config, request bodies, response bodies, headers, API keys, tokens,
  secrets, credentials, or env var names
- matched sensitive values or snippets

If a validator detects any denied class, the snapshot release fails closed.

## Knowledge Content Plan

The KB still needs new knowledge content. The immediate content program should
be organized into product-facing knowledge domains instead of generic wiki
pages.

### Domain 1: Writing Continuity

Purpose: keep user facts, motivation, causality, timeline, prop state, and
chapter/scene continuity stable during expansion or rewriting.

Needed document families:

- `docs/knowledge/writing-continuity/core-rules.md`
- `docs/knowledge/writing-continuity/failure-patterns.md`
- `docs/knowledge/writing-continuity/repair-mappings.md`
- `docs/knowledge/writing-continuity/scene-type-coverage.md`

Runtime targets:

- `writing_rule_packs`
- `scene_mappings`
- `kb_context_summary`
- `negative_constraints`

### Domain 2: Scene Expression

Purpose: convert story material into filmable scene intent, visible action,
dialogue function, conflict turn, and storyboard-ready structure.

Needed document families:

- `docs/knowledge/scene-expression/core-rules.md`
- `docs/knowledge/scene-expression/scene-purpose-taxonomy.md`
- `docs/knowledge/scene-expression/dialogue-action-rules.md`
- `docs/knowledge/scene-expression/compression-and-turning-points.md`

Runtime targets:

- `writing_rule_packs`
- `director_rule_packs`
- `scene_mappings`
- `action_results`

### Domain 3: Director Scheduling

Purpose: guide performance focus, blocking, rhythm, visual attention, and
scene-to-shot handoff without overriding accepted story facts.

Needed document families:

- `docs/knowledge/director-scheduling/core-rules.md`
- `docs/knowledge/director-scheduling/blocking-patterns.md`
- `docs/knowledge/director-scheduling/rhythm-and-attention.md`
- `docs/knowledge/director-scheduling/continuity-handoff.md`

Runtime targets:

- `director_rule_packs`
- `scene_mappings`
- `kb_context_summary`
- `selected_kb_rules`

### Domain 4: Shot Language And Prompt Packaging

Purpose: produce visible-frame shot rows, camera language, duration-aware shot
counts, and clean `prompt_text`.

Needed document families:

- `docs/knowledge/shot-language/shot-intent-taxonomy.md`
- `docs/knowledge/shot-language/camera-and-scale-rules.md`
- `docs/knowledge/shot-language/duration-density-rules.md`
- `docs/knowledge/shot-language/prompt-text-boundary.md`

Runtime targets:

- `director_rule_packs`
- `duration_profiles`
- `validation_rule_packs`
- `negative_constraints`

#### AI Video Prompt Strengthening Extension

The new AI video storyboard material is admitted only as summary-only director
knowledge. It strengthens PWA prompt generation through existing runtime fields
and does not authorize raw image, OCR, tutorial, graph, or prompt-body exposure.

The extension maps to the existing director layer:

- visual master consistency: role identity, style reference, palette, light, key
  props, and scene atmosphere remain stable across rows
- camera language grammar: shot function, shot scale, angle, movement, focal
  feeling, and composition logic become explicit
- preproduction storyboard guide: character/style reference, environment route,
  camera map, storyboard rows, lighting, audio tone, and cinematography notes are
  compressed into executable prompt guidance
- material light air physicality: material feedback, light direction, air medium,
  and environmental response make prompt_text filmable
- expression physicalization: abstract emotion becomes visible face, breath,
  posture, hand, distance, or silhouette behavior
- transition and motion dynamics: transition type and movement vector support
  rhythm and continuity without overriding the selected fragment

PWA impact must be observable in `director_group_rule_pack_ids`,
`selected_kb_rules`, `kb_context_summary`, `negative_constraints`,
`visual_description`, `camera`, `character_action`, and compiled
`prompt_text`. It must remain bounded by the selected storyboard task duration
and script fragment.

### Domain 5: Validation, Leakage, And Export Safety

Purpose: prevent pseudo-success, raw KB leakage, internal refs in export,
invalid person fields, stale tasks, empty rows, and unsafe prompt output.

Needed document families:

- `docs/knowledge/validation/no-pseudo-success.md`
- `docs/knowledge/validation/field-aware-entity-rules.md`
- `docs/knowledge/validation/no-leakage-rules.md`
- `docs/knowledge/validation/export-safety-rules.md`

Runtime targets:

- `validation_rule_packs`
- `negative_constraints`
- `action_results`
- `future_qa_backlog`

### Domain 6: PWA Feedback To KB

Purpose: turn PWA QA failures into KB failure patterns, repair mappings, and
FutureQA candidates.

Needed document families:

- `docs/knowledge/pwa-feedback/trace-to-failure-patterns.md`
- `docs/knowledge/pwa-feedback/repair-routing.md`
- `docs/knowledge/pwa-feedback/futureqa-intake.md`
- `docs/knowledge/pwa-feedback/coverage-gap-review.md`

Runtime targets:

- `future_qa_backlog`
- `validation_rule_packs`
- `action_results`
- `kb_context_summary`
- `validation_rule_packs`

## Document Content Inventory

The next implementation waves should create these contract and artifact files:

```text
docs/kb-product-chain-contract-v0.2.md
schemas/runtime-kb-snapshot.schema.json
schemas/wiki-to-runtime-mapping.schema.json
samples/runtime-kb-snapshot.sample.json
samples/wiki-to-runtime-mapping.sample.json
docs/knowledge-plan-v0.2.md
```

The next content waves should create reviewed wiki or draft wiki pages under a
future approved artifact root:

```text
knowledge/wiki_draft/
knowledge/reviewed_wiki/
knowledge/mappings/
knowledge/runtime_snapshots/
```

Those directories are proposed here only. They must not be treated as approved
runtime stores until a later artifact-prototype gate opens.

## PWA Consumption Plan

The PWA integration should happen after the snapshot and mapping contracts are
validated.

Recommended sequence:

1. Keep the current hardcoded `src/lib/kb.ts` snapshot as built-in fallback.
2. Add `public/kb/latest.json` as the first external runtime snapshot target.
3. Add a snapshot loader that validates schema version, safety constants, and
   snapshot hash.
4. If loading fails, keep the built-in fallback and expose only a sanitized
   fallback reason.
5. Keep `buildSanitizedKbSummary()` as the PWA's narrow consumption function.
6. Extend PWA tests to prove external snapshots do not leak raw KB or internal
   references into prompts, UI, traces, or exports.

## Release Gates

A KB snapshot is releasable to PWA only when:

- runtime snapshot schema validation passes
- wiki-to-runtime mapping schema validation passes
- coverage matrix passes
- leakage scan passes
- `snapshot_hash` is computed and stable
- source delta evidence is aggregate-only
- no draft or quarantine material is runtime-eligible
- PWA loader tests pass against the snapshot
- last-known-good fallback is documented

## Non-Goals

This contract does not authorize:

- turning Hope-KB into the main product or replacing the Hope content workflow
- PWA runtime graph traversal
- GraphRAG or hybrid/rerank defaults
- runtime source fetching
- runtime auto-ingest
- runtime LLM summarization of raw wiki/source material
- exposing an Obsidian-like graph UI in the PWA
- writing user session content back into `hope-kb`
- publishing raw wiki or graph files as PWA assets
- modifying seed, snapshot SQLite, migrations, or Hope desktop runtime

## Open Decisions

- whether the first artifact prototype should use `knowledge/` or
  `artifacts/knowledge/` as its root
- whether `reviewed_wiki_id` should be human-readable or opaque
- whether runtime snapshots should be stored in `samples/` first or a dedicated
  `runtime_snapshots/` directory
- whether PWA should vendor the snapshot at build time or fetch it as a static
  public asset
- which validator should own coverage matrix checks: Rust descriptor validator
  extension or a separate snapshot validator
