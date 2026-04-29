# Source To Wiki Minimal Loop Intake

Intake date: 2026-04-30

Source thread: `(running) HopePrompt KB - source acquisition and knowledge governance`

Original dispatch: `source-to-wiki-loop-dispatch.md`

Repository: `E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb`

Live Git reported by lane:

- branch: `codex/contracts-freeze`
- HEAD: `dd8608d`
- status: clean and aligned with `origin/codex/contracts-freeze`

## Intake Result

Status: accepted for total-control decision.

The lane satisfied the read-only source-to-wiki minimal loop design dispatch:

- confirmed the live Git state is a clean fast-forward successor of the dispatch anchor
- reviewed the core, LLM Wiki governance, source governance, descriptor matrix, freshness activation, OpenSpec, SourceDeltaBatch validator, and Lane 1 synthetic pass fixture materials
- produced a minimal offline loop from source intent registration through reviewed wiki to SourceDeltaBatch handoff
- kept the work read-only
- did not create artifacts, edit docs, fetch sources, store raw source material, alter seed/snapshot/runtime/UI/export, modify validators, create fixtures, stage, commit, or push

## Accepted Design Content

Accepted alignment with the Karpathy LLM Wiki pattern:

- borrow the durable raw/wiki/schema/snapshot separation
- use wiki as a human-reviewable synthesis surface, not a runtime query store
- require explicit review and promotion instead of automatic LLM promotion
- keep raw evidence and provenance in offline governance boundaries
- do not adopt runtime live web fetch, runtime wiki lookup, automatic raw corpus indexing, automatic promotion, or provenance/raw source exposure to runtime payloads

Accepted minimal loop:

1. `source_intent_registration`
2. `quarantine_raw`
3. `quarantine_labeled`
4. `wiki_draft`
5. human or controller review
6. `reviewed_wiki`
7. `SourceDeltaBatch` governance descriptor

Accepted record-shape direction:

- `source_intake` records use stable IDs, controlled source type, normalized hashes, bucketed timestamps, owner/review fields, candidate knowledge types, risk flags, quarantine state, leakage count, and runtime exclusion
- `wiki_draft` records use wiki ID, source IDs, wiki type/title, compact claims, source coverage summary, linked schema targets, open questions, review status, effective confidence, risk flags, leakage count, and runtime exclusion
- `reviewed_wiki` records use reviewed claims summary, source coverage summary, linked schema targets, review status, confidence, controller review ID, reviewed bucket, limitation notes, supersession references, leakage zero, and runtime exclusion
- `SourceDeltaBatch` handoff uses aggregate hashes, counts, summaries, review completeness flags, sanitized locator/runtime exclusion flags, activation request fields, leakage zero, and notes summary references

Accepted review gates:

- quarantine raw to labeled requires normalized digest, risk flags, quality label, and runtime exclusion
- labeled to wiki draft requires bounded extraction, candidate wiki type, linked schema hints, open questions, and zero leakage
- wiki draft to reviewed wiki requires human/controller review, resolved or retained open questions, effective confidence, reviewed claim summary, and zero leakage
- reviewed wiki to SourceDeltaBatch requires accepted or accepted-limited state, normalized hashes, count consistency, runtime exclusion, sanitized locators, and required reviews when activation is requested
- SourceDeltaBatch to activation planning may cross only aggregate `source_delta_batch_hash`, `source_delta_count`, and `source_freshness_digest`

## Core Challenger Notes

- This design is sufficient to open a docs-only artifact contract gate, not an implementation gate.
- It does not prove real source acquisition, review labor, storage layout, wiki artifact schema implementation, cross-artifact binding, canonical digest recomputation, snapshot rebuild, activation switch, or runtime consumption.
- `review_status_summary` and `effective_confidence_summary` remain too loose for activation planning until structured contract and validator hardening gates exist.
- `reviewed_wiki` must not be treated as runtime-ready or seed-ready without a later seed/snapshot/index rebuild and activation verification gate.
- The next docs should make stop conditions and non-runtime boundaries concrete enough that future artifact prototypes cannot drift into raw-source storage or runtime fallback behavior.

## Audit Specialist Notes

- No raw source text, per-source locator values, source register details, matched values, local path details, provider config, request/response body, secrets, or tokens were requested or needed.
- Any future artifact contract must preserve aggregate-only diagnostics and avoid copying raw source or locator data into prompt payloads, logs, UI, telemetry, shadow records, rollback records, or chat-visible summaries.
- SourceDeltaBatch may support governance review and activation planning only through aggregate binding.

## Total-Control Decision

Open a narrow docs-only artifact contract gate.

Approved next gate:

- update docs only
- define `source_intake`, `wiki_draft`, `reviewed_wiki`, and `SourceDeltaBatch` handoff artifact contracts
- define field allowlists, denied fields, lifecycle states, review transitions, aggregate-only diagnostics, and future validator-design matrix
- do not create real artifacts, synthetic artifacts, fixtures, validators, storage, source fetching, seed changes, snapshot changes, runtime changes, UI changes, export changes, staging, commit, or push in the branch lane
