# Product Action Crosswalk v0.2

Status: product alignment note.
Scope: map Hope-KB action names to observed `hope-web-pwa` and Hope pipeline
terms so KB coverage does not drift away from actual output artifacts.

## Evidence Sources

- `hope-web-pwa` repository: `https://github.com/zzy170031-cmd/hope-web-pwa`
- local inspection clone used during this gate:
  `C:\Users\Administrator\Documents\New project\hope-web-pwa-inspect`
- `hope-web-pwa` type anchors:
  `src/lib/types.ts` defines `KB_ACTIONS`, `KbSnapshot`,
  `SanitizedKbSummary`, `StoryboardTask`, and `StoryboardRow`
- Hope-2-v0 contract anchor:
  `E:\codex\hope-2-v0\contracts\pipeline-stage-io.md`

This crosswalk does not make Hope-2-v0 or desktop shell the runtime target for
this gate. It prevents terminology drift while `hope-web-pwa` remains the first
PWA consumer target.

Hope-KB remains subordinate to the Hope content production workflow. The
crosswalk exists to keep KB terms aligned with actual product artifacts, not to
make KB terminology the product source of truth.

## Action Crosswalk

| KB action | PWA / product surface | Hope-2-v0 adjacent term | KB support boundary |
| --- | --- | --- | --- |
| `import_source` | source text import / source readiness | `SynopsisInput` | summary-only intake readiness; no raw source disclosure |
| `expand_story` | story body expansion | `StoryOutput` / `ScreenplayOutput` adjacent | writing continuity and visible action guidance |
| `rewrite_story` | story body rewrite | accepted rewrite snapshot / `ScreenplayOutput` adjacent | preserve facts while improving expression |
| `accept_story_body` | accepted body lock | accepted snapshot / StoryFactFrame boundary | fact lock before storyboard planning |
| `create_story_task` | `StoryboardTask`, task queue, task hash | `StoryboardPlanOutput` adjacent | convert accepted body into task structure and duration plan |
| `generate_storyboard` | `StoryboardRow[]`, prompt package | `StoryboardPlanOutput` | visible rows, shot intent, prompt boundary |
| `repair_storyboard` | explicit repair after validation failure | validation repair loop | targeted repair without silently rewriting accepted facts |
| `validate_result` | `ValidationResult`, QA trace | `ValidationOutput` | no pseudo-success, field safety, duration, leakage checks |
| `export_result` | `ExportBundle` / user deliverable | `ExportBundle` | confirmed rows only; no internal refs or raw KB |
| `golden_sample_review` | QA / sample review only | eval or review packet adjacent | governance only; no raw sample runtime payload |

## Boundary Notes

- KB coverage proves rule support, not that a product command is currently
  runnable.
- `action_results.state=ready` means KB support ready only. It must not be
  presented as PWA, IPC, provider, export, or desktop runtime readiness.
- Blocked or gated product commands must remain marked as product-side gated.
- KB runtime snapshots may describe how to guide or validate an action, but
  they must not claim UI, IPC, export, provider, or desktop capability exists.
- Action names should remain stable in KB mapping; product-specific aliases
  should be handled through this crosswalk rather than by renaming KB actions
  ad hoc.

## PWA Adapter Boundary

The Batch 1 `runtime-kb-snapshot` is a KB-side candidate artifact. It is not a
drop-in replacement for the current `hope-web-pwa` in-code `KB_SNAPSHOT`.

Before product integration, the bridge must compile or adapt the KB contract
into the PWA contract:

- field shape: KB `snake_case` fields must become PWA `camelCase` fields such
  as `snapshotVersion`, `allowedDurations`, and `writingRulePacks`;
- rule IDs: KB rule pack ids must be mapped to the PWA rule identifiers already
  used by `src/lib/kb.ts`, or the PWA must intentionally adopt the new ids;
- scene types: Batch 1 currently proves only the narrow prototype scene set, not
  the full product scene-type catalog;
- failure mode: if the external KB asset is absent, invalid, stale, or unsafe,
  PWA must keep using last-known-good or built-in safe defaults;
- leakage boundary: adapted output must still expose only
  `SanitizedKbSummary`-style data and must not expose raw wiki, raw graph,
  source register, prompt body, local paths, or secrets.

So the intended product flow is:

```text
reviewed_wiki -> wiki-to-runtime mapping -> runtime-kb-snapshot candidate
-> PWA adapter/compiler -> PWA KbSnapshot/SanitizedKbSummary -> Hope content output
```

This keeps KB subordinate to Hope content generation while making the future
PWA handoff explicit and testable.

## Required Follow-Up

When `hope-web-pwa` changes action names or type fields, update this crosswalk
before changing runtime snapshot schemas.
