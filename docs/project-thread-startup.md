# hope-kb Project Thread Startup

Use this prompt when opening the dedicated `hope-kb` implementation thread:

```text
本线程专门用于 hope-kb 并行实施。启用 five-agent-governance 真实五代理模式，并按 Git 当前 handoff 路线执行。当前阶段为 KB completeness + taxonomy / repair runtime hardening。请以主线程作为唯一集成者，按 4 Lane + 1 主线程 的真实五代理规则推进，并保持与 E:\codex\hope 的 RC 主线程分离。把 E:\codex\hope-kb\docs\live-progress.md 作为线上同步面板，每完成一个 10-20 分钟工作包就刷新“当前在做 / 最新已完成 / 下一步 / 阻塞”。
```

The prompt block above is historical context only. Use the canonical startup prompt below for the current `hope-kb` thread.

## Canonical startup prompt

```text
This thread is only for `hope-kb` work. Enable real five-agent governance for the `hope-kb` hardening lanes only. Keep `hope` main-thread implementation out of scope. Current phase: KB completeness plus taxonomy / repair runtime hardening. Run with 1 KB integration owner plus 4 working lanes inside `E:\codex\hope-kb`. This repo serves `hope` with validated knowledge assets, validation gates, and read-only snapshots; it does not take over main-thread implementation work. Do not merge with `hope` unless merge readiness is explicitly reopened. Use `E:\codex\hope-kb\docs\live-progress.md` as the online sync board and update it after each 10-20 minute work package.
```

## Scope guard

- This thread is only for `hope-kb` work.
- Run with 1 KB integration owner plus 4 working lanes inside `hope-kb`; do not implement `hope` main-thread work here.
- Keep `hope` as a parallel boundary only until merge readiness is explicitly reopened.
- Use `docs/live-progress.md` as the live sync surface for the current `hope-kb` thread.
- Treat KB work as support for `hope` main-thread operation and outputs, not as a substitute mainline.

## Thread intent

- one KB project thread per active hardening wave
- one integration owner that controls manifest, import map, and snapshot convergence
- one sync surface for online status updates

## Current v0.2 Prompt Knowledge Startup Addendum

The current project name is `hope-kb-图谱制作`, under
`E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb`.

Use the Hope core collaboration rules from
`Codex总控分线程协作规则_2026-04-25`:

- dispatches, reviews, and branch reports use copy-ready `text` code blocks
- branch reports start with `总控回报：`
- branch reports set `目标线程：总控线程`
- Git/file state is the only source of truth
- branch threads report conflicts before acting
- branch threads do not revert user or other-thread changes
- branch threads do not commit or push by default
- total control is the only default integrator and commit owner

Current five-agent shape:

- Main control: `HopePrompt知识库-【v0.2总控】`
- Lane 1: `HopePrompt知识库-【来源入库与知识治理】`
- Lane 2: `HopePrompt知识库-【图谱Schema与快照合同】`
- Lane 3: `HopePrompt知识库-【路由评测与FutureQA】`
- Lane 4: `HopePrompt知识库-【安全Lint与泄漏防护】`

The first version is Prompt knowledge governance and graph QA for Hope. It
uses the LLM Wiki `raw / wiki / schema / snapshot` layers and the
`Ingest / Query / Lint / Future QA` operations. It does not generate images,
generate videos, connect image/video runtime, runtime-fetch from the network,
runtime-auto-ingest, default to GraphRAG/hybrid/rerank, or default to runtime
LLM summarization.

## Suggested next thread sequence

1. taxonomy / alias hardening thread
2. repair mapping / prompt template hardening thread
3. degraded-input coverage thread
4. snapshot / validation / import handoff thread
5. merge-readiness evaluation thread
