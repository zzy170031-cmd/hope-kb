# QA Hardening v0.2 Validation Telemetry Leakage Packet 2026-04-25

## Scope

This packet defines acceptance requirements for the `hope-kb` v0.2 QA hardening surface. It is implementation-neutral and does not open product-side runtime work in `hope`.

Current v0.2 baseline context:

- seed bundle validation already covers import-map counts, primary keys, v0.2 linkage, coverage summaries, source-register counts, and manifest content hash
- snapshot build already covers explicit `--version v0.2`, SQLite import counts, foreign-key checks, and `quick_check`
- V120/V148 safe-plus absorption keeps the public seed contract at 23 fields and 152 rows, with no `director_style_ref`, no schema expansion, and no runtime/provider hookup

## Acceptance Test Checklist

All v0.2 QA gates should pass before any downstream consumer treats the v0.2 snapshot or retrieval surface as auto-switch eligible.

- Seed validation: `validate-v0-2-seed-bundle.ps1` passes for the current repo root, including record counts, unique primary keys, library/failure/repair linkage, coverage summaries, source-register counts, and manifest hash.
- Snapshot validation: `build-kb-snapshot.py --version v0.2` builds a v0.2 snapshot and passes table counts, foreign-key checks, and SQLite `quick_check`.
- Runtime-consume contract: any consumer-facing packet continues to treat `hope-kb` as a support surface only; product-side runtime implementation belongs to `hope`.
- Structural leakage: generated payloads, logs, telemetry events, shadow records, and rollback records are parsed as structured objects and inspected by field/path, not only searched as raw strings.
- Log and telemetry redaction: no log or telemetry record contains full prompt text, `prompt_body`, source original text, local paths, API keys, raw graph payloads, raw KB rows, the full source register, or quarantine raw original text.
- Local aggregation: telemetry aggregation remains local by default, stores only bounded aggregate counters and sanitized IDs, and has a purge path that leaves local aggregate files empty or removed.
- Cloud sync default: v0.2 has no telemetry opt-in cloud sync by default. Any opt-in cloud sync belongs to a separately reviewed v0.2.1+ contract.
- Shadow phase: shadow runs observe selected IDs, payload size, fallback rate, leakage result, purge result, and no-short-input-expensive-path result before any auto switch.
- Threshold enforcement: any shadow threshold breach blocks auto switch and keeps the deterministic or existing fallback path active.
- Rollback contract: catastrophic triggers force rollback or disablement of the v0.2 retrieval/runtime path while preserving only sanitized incident metadata.
- Calibration deferral: threshold tuning and calibration experiments are deferred to v0.2.1; v0.2 should use conservative fixed gates or controller-approved thresholds.
- Short-input guard: short or trivial inputs do not enter expensive retrieval, graph expansion, or provider paths when a bounded local answer/fallback is sufficient.

## Structural Leakage Tests

Leakage tests must parse each artifact as its native structure where possible: JSON object, telemetry event object, log schema, SQLite row, export record, or shadow-run record. Raw string scans may remain a secondary belt-and-braces check, but they are not sufficient acceptance evidence.

Required structural assertions:

- Denied field names are absent at every nesting level, including `prompt_full_text`, `full_prompt`, `prompt_body`, `source_original_text`, `original_text`, `raw_original_text`, `quarantine_raw_original_text`, `raw_kb_rows`, `raw_graph`, `source_register`, `full_source_register`, `local_path`, `absolute_path`, `api_key`, `secret`, `token`, and provider credential fields.
- Denied field aliases are absent even when embedded in generic containers such as `metadata`, `debug`, `trace`, `extra`, `context`, `attachments`, `payload`, or serialized JSON strings.
- Allowed KB disclosure is bounded to sanitized summary fields such as `snapshot_version`, `snapshot_hash`, `selected_sample_ids`, `selected_kb_rules`, `payload_bytes`, `fallback_reason_code`, and `kb_context_summary`.
- `selected_sample_ids` and `selected_kb_rules` may be recorded for QA traceability, but must not expand into raw rows, source-register entries, source text, prompt bodies, or full graph neighborhoods.
- `kb_context_summary` must be a compressed summary and must not contain full prompt text, full source rows, or raw source-register values.
- Path checks must reject Windows drive paths, home-directory paths, repo-local absolute paths, and temp/cache paths.
- Credential checks must reject direct key values, credential-reference expansion, bearer tokens, provider request headers, and environment-variable dumps.
- Quarantine checks must verify that quarantine records expose only reason code, content hash or opaque ID, count, and state; quarantine raw original text must remain absent.

Acceptance result: leakage count is exactly zero. Any structural leak is a catastrophic trigger.

## Logging And Telemetry Contract

v0.2 logging and telemetry should be local, minimal, and operational rather than content-bearing.

Allowed telemetry classes:

- validation counters: pass/fail counts, rule IDs, snapshot version, snapshot hash, table counts
- routing counters: selected ID count, selected sample IDs, selected rule IDs, payload byte size, fallback reason code
- shadow counters: candidate path name, existing path name, agreement class, latency bucket, fallback rate bucket, threshold status
- purge counters: files inspected, files emptied, files removed, bytes remaining, purge status
- rollback counters: trigger code, disabled path, previous path, timestamp bucket, sanitized incident ID

Denied telemetry content:

- prompt full text or `prompt_body`
- source original text, normalized raw source text, or quarantine raw original text
- local filesystem paths or user/home/repo paths
- API keys, tokens, provider headers, environment dumps, or credential references expanded to values
- raw graph payloads, raw KB rows, full source register, full import map expansion, or source workbook/document excerpts
- provider request/response bodies when they contain prompt or source text

Local aggregation boundaries:

- Aggregates must be append-safe or replace-safe local files only; no default cloud upload or sync target is allowed in v0.2.
- Aggregate rows must store counters, bounded IDs, hashes, byte counts, reason codes, and status codes only.
- Debug mode must not widen the payload content boundary.
- A purge action must remove aggregate files or truncate them to empty content. Leaving sensitive tombstones, previous values, or non-empty raw event backups fails the purge gate.
- After purge, a fresh aggregation run starts from zeroed counters and cannot reconstruct purged events from sidecar files.

## Shadow Phase Gates

The v0.2 shadow phase observes behavior without switching the product path automatically.

Required observations:

- selected sample IDs and selected KB rule IDs
- final payload byte size and per-step payload size bucket
- fallback rate and fallback reason codes
- leakage test result for payload, logs, telemetry, shadow record, and rollback record
- short-input routing result, including whether expensive retrieval/provider paths were skipped
- purge result for local aggregate files

Default blocking thresholds for v0.2:

- leakage events: must be `0`
- purge residual sensitive files: must be `0`
- payloads containing raw KB rows, full source register, raw graph, or prompt full text: must be `0`
- short-input expensive-path violations: must be `0`
- payload size over the controller-approved byte limit: must be `0`
- fallback rate over the controller-approved threshold: blocks auto switch
- missing selected-ID trace for non-empty retrieval: blocks auto switch

If control has not supplied a calibrated fallback-rate threshold, v0.2 must treat any unexplained fallback spike or sustained fallback regression as blocking. Statistical calibration, per-domain threshold tuning, and automatic threshold relaxation are v0.2.1+ work.

## Rollback And Catastrophic Triggers

Rollback is a downstream contract requirement, not an implementation change in this packet. A future consumer should be able to disable the v0.2 retrieval/runtime path and continue on the previous deterministic or fallback path.

Catastrophic triggers:

- any API key, token, provider header, or credential value appears in payload, logs, telemetry, shadow records, or rollback records
- any prompt full text, `prompt_body`, source original text, raw KB row, raw graph, full source register, local path, or quarantine raw original text appears in an emitted artifact
- purge fails to remove or empty local aggregate files
- manifest hash, snapshot counts, foreign-key checks, or SQLite `quick_check` fail
- shadow threshold breach is ignored and auto switch proceeds
- short or trivial input enters an expensive retrieval/provider path contrary to the short-input guard
- fallback rate exceeds the controller-approved threshold or shows an unexplained spike during shadow

Rollback record requirements:

- record only trigger code, sanitized incident ID, timestamp bucket, disabled path, previous path, and remediation state
- do not record request body, prompt text, source text, raw rows, local paths, provider response body, or credential material
- after rollback, keep v0.2 shadow observations available only as sanitized aggregates

## v0.2.1 Deferrals

The following items are explicitly deferred beyond the v0.2 default contract:

- telemetry opt-in cloud sync
- fallback-rate calibration experiments
- automatic threshold relaxation
- provider-specific request/response telemetry
- richer graph diagnostics that would require raw graph or raw row capture
- schema expansion for director-style metadata, source-register exposure, or prompt-body retention

Any v0.2.1+ reopening should preserve the v0.2 leakage boundary as the default floor.
