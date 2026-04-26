# Parallel Execution Appendix

`hope-kb` executes with one KB integration owner plus four working lanes.

## Lane 1

- source register
- source policy
- provenance review

## Lane 2

- scene taxonomy
- director mappings
- alias normalization

## Lane 3

- failure pattern library
- repair template mappings
- prompt template hardening

## Lane 4

- classic case examples
- export templates
- benchmark-facing content thickness

## KB integration owner

- only KB integrator
- main seat in the 1 + 4 five-agent pattern
- only manifest / snapshot release owner
- only manifest / import map maintenance owner
- only validation script and snapshot build owner
- only runtime-consumer handoff artifact owner
- only merge-readiness owner for `hope-kb`
- only boundary-sync owner with `hope`
- no `hope` main-thread implementation work in this thread
- `hope-kb` serves `hope`; it does not absorb `hope` main-thread implementation work

## Governance

Use five-agent governance at milestones, not every small commit:

- KB seed pack freeze
- taxonomy / repair validation green
- snapshot build + import validation green
- merge-readiness review before any explicit thread merge

## Current v0.2 Five-Agent Assignment

For `hope-kb-图谱制作`, use exactly one total-control thread plus four branch
threads:

- Main control: `HopePrompt知识库-【v0.2总控】`
- Lane 1: `HopePrompt知识库-【来源入库与知识治理】`
- Lane 2: `HopePrompt知识库-【图谱Schema与快照合同】`
- Lane 3: `HopePrompt知识库-【路由评测与FutureQA】`
- Lane 4: `HopePrompt知识库-【安全Lint与泄漏防护】`

Control responsibilities:

- own integration decisions
- own docs/live-progress sync
- own commit/push decisions
- whitelist staging before any commit
- keep `hope` mainline, desktop, intake, image/video runtime, GraphRAG,
  hybrid/rerank defaults, runtime network fetch, runtime auto-ingest, and
  runtime LLM summarization out of v0.2 unless a future gate explicitly reopens
  them

Lane responsibilities:

- own only the bounded files and contract surface assigned by total control
- verify live Git state before acting
- avoid broad replanning
- do not commit or push by default
- report back in the core `总控回报` text-code-block format
