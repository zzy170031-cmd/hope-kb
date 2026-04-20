# hope Main-Thread Review Packet for 0d534a4

This review packet is a `hope-kb` delivery note for `hope` main-thread review.

It does not implement `hope` code.
It freezes what the `hope` side should review after the pushed checkpoint `0d534a4`.

## Checkpoint

- branch: `codex/contracts-freeze`
- pushed commit: `0d534a4` (`Harden exporter runtime payload edge cases`)
- manifest path: `seed/v0.1/manifest.json`
- runtime consume contract path: `seed/v0.1/runtime_consume_contracts.json`
- runtime handoff packet: `docs/runtime-consume-integration-packet-v0.1.md`

## Bundle Baseline

- snapshot version: `v0.1`
- snapshot name: `hope-kb-minimum-benchmark-pack`
- bundle hash: `bundle-sha256:5f042c10ada3726bdbd71d5f4cbad2187b3895dd85e1e5195d6938a3a22d20b6`
- key record counts:
  - `committee_handoff_rules = 27`
  - `runtime_consume_contracts = 5`
  - `degraded_input_examples = 26`
  - `classic_case_examples = 28`
  - `export_templates = 17`

## Validation And Snapshot

- validator command: `& 'E:\codex\hope-kb\scripts\validate-seed-bundle.ps1'`
- validator result: passed
- standard snapshot path: `snapshots/hope-kb-v0.1.sqlite3`
- snapshot result:
  - `snapshot_meta.content_hash = bundle-sha256:5f042c10ada3726bdbd71d5f4cbad2187b3895dd85e1e5195d6938a3a22d20b6`
  - `degraded_input_example = 26`
  - `runtime_consume_contract = 5`
  - `committee_handoff_rule = 27`

## Hope-Facing Review Focus

Review the pushed checkpoint against these 5 runtime consume surfaces:

1. `snapshot_bootstrap`
   `hope` should trust `snapshot_meta`, `export_template`, `failure_pattern`, `degraded_input_example`, and `runtime_consume_contract` before bootstrap, and should not accept local sheet alias fallback as a substitute for the Validation sheet contract.
2. `segment_and_cut_projection`
   `hope` should consume `RenderSegments` and `Cuts` only through snapshot columns and validator-aware continuity gates, and should not locally backfill `render_segment_id` when explicit mappings are missing.
3. `handoff_projection`
   `hope` should treat `HandoffZones` as a full payload with pair, director, transition, buffer, and continuity context, and should not silently downgrade empty `buffer_cut_ids` into a safe default.
4. `prompt_package_projection`
   `hope` should trust `PromptPackage` from snapshot contract and avoid local negative/default injection, including the case where `negative_prompt` is intentionally blank for the target model family.
5. `validation_feedback_projection`
   `hope` should preserve `status`, `severity`, `message`, `related_cut_or_segment`, and `is_blocking` instead of inferring blocking semantics from `severity` alone.

## Hope-Facing Boundary

- `hope-kb` owns the knowledge assets, validators, seed bundle, and read-only snapshot contract.
- `hope` owns product-side runtime consumption and any implementation that reads this snapshot.
- this packet is for review only; it does not reopen merge-readiness
- no thread merge is implied by this packet
- do not move `hope` product logic into `hope-kb`

## Review Ask For hope

1. confirm the 5 runtime consume surfaces still match `hope` main-thread expectations after the finer payload edge-case hardening in `0d534a4`
2. confirm no product-side consumer still depends on local sheet alias fallback, local blocking inference, local segment backfill, local handoff defaults, or local negative/default prompt fill
3. confirm product-side hookup should continue to consume the validated snapshot contract rather than recreate contract logic locally
