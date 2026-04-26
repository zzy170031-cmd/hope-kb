# hope-kb Agent Rules

This repo is built through Codex-only parallel execution.

## Core collaboration rules

- Follow `Codex总控分线程协作规则_2026-04-25` for total-control dispatches,
  branch-thread reports, naming, Git truth, and safety boundaries.
- Total-control task dispatches, total-control reviews, and branch-thread
  reports must use copy-ready `text` code blocks.
- Branch-thread reports to control must start with `总控回报：` and must set
  `目标线程：总控线程`.
- Git state is the only source of truth. Every branch thread must check
  `git status --short --branch`, `git log -1 --oneline --decorate`, and
  `git diff --stat` before making planning claims.
- If a handoff packet, chat instruction, or local file state conflicts with
  current Git/file truth, report the conflict first and do not silently
  normalize it.
- Do not revert user or other-thread changes unless total control explicitly
  asks.
- Do not commit unknown dirty files, build side effects, `AGENTS.md`, or
  snapshot SQLite files unless total control explicitly whitelists them.
- Branch threads do not commit or push by default. The main control thread is
  the default staging, commit, and push owner.

## Core rules

- Use Codex / Codex subagents only.
- This thread owns `hope-kb` work only; do not implement `hope` main-thread tasks here.
- Keep the `hope-kb` thread separate from the Hope RC main thread.
- Treat `docs/kb-freeze-contract.md`, `seed/v0.1/manifest.json`, `seed/v0.1/import_map.json`, and `migrations/0001_init_kb.sql` as the frozen contract surface for v0.1.
- Do not force-feed new KB scope back into `hope` until merge readiness is explicitly re-opened.
- Keep runtime hardening, taxonomy expansion, and repair coverage traceable to seed assets and validation scripts.

## Implementation language policy

- When a gate explicitly opens code implementation, prefer Rust first for
  validators, descriptor checkers, CLI tools, and durable governance logic.
- Use PowerShell only for thin Windows orchestration, existing script
  compatibility, or explicit total-control exceptions.
- Docs-only gates remain docs-only. Do not introduce Rust or any other code
  until total control explicitly opens an implementation gate.

## Five-agent lanes

- Main thread: only integrator, only merge-readiness owner, only cross-repo sync owner with `hope`
- Lane 1: source register, source policy, provenance review
- Lane 2: taxonomy, director mappings, alias normalization
- Lane 3: failure patterns, repair mappings, prompt template hardening, degraded-input coverage
- Lane 4: case examples, export templates, validation chain, snapshot build, runtime-consumer handoff artifacts

### Current v0.2 Prompt knowledge lanes

Use exactly one total-control thread plus four branch lanes for the current
`hope-kb-图谱制作` project:

- Main control: `HopePrompt知识库-【v0.2总控】`
- Lane 1: `HopePrompt知识库-【来源入库与知识治理】`
- Lane 2: `HopePrompt知识库-【图谱Schema与快照合同】`
- Lane 3: `HopePrompt知识库-【路由评测与FutureQA】`
- Lane 4: `HopePrompt知识库-【安全Lint与泄漏防护】`

The current project still serves Hope. It is not an independent product, does
not generate images or videos, does not connect image/video generation runtime,
does not turn Hope desktop into a knowledge-management product, and does not
make GraphRAG, hybrid/rerank, runtime network fetch, runtime auto-ingest, or
runtime LLM summarization the v0.2 default.

## Delivery discipline

Every lane handoff must include:

1. `来源线程`
2. `仓库路径`
3. `branch / HEAD`
4. `git status`
5. `已完成`
6. `未完成`
7. `修改文件`
8. `验证结果`
9. `是否提交 / push`
10. `风险 / 阻塞`
11. `需要总控决策的问题`
12. `下一步建议`

The report must also state whether seed files, schema, snapshots, validators,
or only docs were touched. Include the validation command actually run and any
blockers or merge risks.

Use this exact report shape:

```text
总控回报：
目标线程：总控线程
来源线程：（运行）HopePrompt知识库-【目标动作】
仓库路径：
E:\codex-projects\hope-prompt-kb-v0.2-independent\hope-kb
branch / HEAD：
...
git status：
...
已完成：
...
未完成：
...
修改文件：
...
验证结果：
...
是否提交 / push：
...
风险 / 阻塞：
...
需要总控决策的问题：
...
下一步建议：
...
```

## Fixed safety boundaries

- Do not write, display, or log API keys, tokens, provider headers, or secrets.
- Do not expose raw `prompt_body`, full raw KB rows, full `source_register`,
  overlay JSON, raw graph payloads, local paths, or quarantine raw original
  text.
- Runtime-facing KB context must stay summary-only:
  `kb_context_summary`, `selected_sample_ids`, `selected_kb_rules`, and a
  sanitized retrieval-trace summary.
- `full_kb_rows_included` must remain `0` for runtime prompt-routing payloads.

## Milestone status labels

The KB thread follows the same canonical milestone-label system as the main Hope project thread.

- Use the display format `线程名【状态标签】`.
- Use the shared policy in `E:\codex\ENGINEERING_THREAD_LABEL_POLICY.md`.
- Update the label whenever the KB thread crosses a key node such as: checkpoint accepted, standby entered, bounded feedback received, commit-ready, or environment-blocked.
- The label must reflect the real local KB state, not only the remote Git state.
- Every key-node handoff must end with:
  `关键节点提醒：请立即刷新线程标签、锚点提交、工作树状态和边界说明。`
- Keep one canonical label for the active KB thread and make sure it stays traceable to:
  1. repo path
  2. branch
  3. anchor commit
  4. clean / dirty worktree state
  5. one-line scope boundary

### Current KB canonical name and label

- `Hope-KB-检查点支持【检查点待命】`

## Main references

- `docs/project-thread-startup.md`
- `docs/parallel-execution-appendix.md`
- `docs/live-progress.md`
- `docs/kb-build-playbook-v0.1.md`
- `docs/kb-freeze-contract.md`
- `docs/kb-import-runtime-v0.1.md`
- `E:\codex\ENGINEERING_THREAD_LABEL_POLICY.md`
