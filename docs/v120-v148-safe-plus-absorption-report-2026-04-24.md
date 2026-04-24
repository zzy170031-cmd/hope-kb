# V120 V148 Safe-Plus Absorption Report 2026-04-24

## Goal

- absorb V148 camera structure, density, negative-list, and abstract meeting logic into the current 32 reserve candidates only
- keep Hope public contract at `23 fields / 152 rows`
- keep the original 120 rows stable
- do not import `director_style_ref`, real director names, official/Yes promotions, runtime hookups, or schema changes

## Inputs

- plan docx: `C:\Users\Administrator\Desktop\claude\V148_最新交付\Seedance2.0黄金样本库V148建设方案_导演风格版.docx`
- style workbook: `C:\Users\Administrator\Desktop\claude\V148_最新交付\Seedance2.0黄金样本库V148_24字段导演风格版.xlsx`
- current seed baseline: `E:\codex\hope-kb\seed\v0.2\golden_sample_library.json` at commit base `e3140d7`

## V148 Capability Intake

- absorbed lens structures: 国战军阵, 武将高光, 朝堂权谋, 沙盘视口, 城建演进, 战报 UI, 多军团攻城
- absorbed density controls: 秒点动作, 构图比例, 风向, 人数规模, 兵器轨迹, 光源方向, 静默点, 声音同步
- absorbed negative controls: 旗帜乱码, 朝代错乱, 盔甲塑料感, 地图现代感, 行军线漂移, UI 乱码, 品牌/IP 泄漏
- absorbed meeting logic only as abstraction inside `teaching_note` and this report; no real names were written into any seed prompt

## Explicitly Not Absorbed

- no `director_style_ref` field in any output seed or final 23-field workbook
- no V148 `official / Yes` status promotion into Hope
- no GAMESEQ rows, no row-count change beyond the existing 152
- no schema change to 24 fields, no V152 work, no runtime / Qwen / Doubao / Seedance / reference_control_core opening

## Sequence Mapping

- `CNWARSEQ01-03`: absorbed directly from matching V148 war / general / court sequences
- `CNWARSEQ04`: absorbed from V148 siege / multi-legion siege capability while staying inside current Hope war sequence semantics
- `SLGSEQ01`: absorbed from V148 strategic sandbox viewport sequence
- `SLGSEQ02`: absorbed from V148 trajectory + siege routing capability as multi-route / alliance / retreat grammar
- `SLGSEQ03`: absorbed from V148 city-build time-lapse capability
- `SLGSEQ04`: absorbed as battle-report UI closure, using V148 siege-closing information rhythm without importing external names or 24th-field metadata

## Dry-Run Result

- field count remains `23`: `True`
- row count remains `152`: `True`
- `director_style_ref` absent: `True`
- original 120 rows unchanged: `True`
- new 32 remain `reserve / No`: `True`
- placeholder residue count: `0`
- banned prompt hit count: `0`
- eight sequences remain complete: `{"CNWARSEQ01": 4, "CNWARSEQ02": 4, "CNWARSEQ03": 4, "CNWARSEQ04": 4, "SLGSEQ01": 4, "SLGSEQ02": 4, "SLGSEQ03": 4, "SLGSEQ04": 4}`

## Output Workbook

- output path: `E:\codex\outputs\黄金样本库v120究极版_V120国产动漫三国SLG专项增强候选池_最终版.xlsx`
- desktop mirror path: `C:\Users\Administrator\Desktop\样本存储区\黄金样本库v120究极版_V120国产动漫三国SLG专项增强候选池_最终版.xlsx`
- workbook sha256: `b24fa72898d1ede5ee4eb14d1cd0f64f2bc1027718b05580bd942d06defbb2ed`
- sheet layout: `主控摘要` + `V120工作簿`
- schema: `23 fields`, no `director_style_ref` column

## Validation

- manifest hash: `bundle-sha256:b16bb3719d73c7246eef674838130bc671879263f5889075cfbc4fbe1fff049e`
- `validate-v0-2-seed-bundle.ps1`: passed
- `build-kb-snapshot.py --version v0.2`: passed
- snapshot output: `E:\codex\hope-kb\snapshots\hope-kb-v0.2.rebuilt-5.sqlite3`
- snapshot counts: `golden_sample_library=152`, `golden_sample_field_coverage_rule=5`, `golden_sample_failure_mapping=152`, `golden_sample_repair_mapping=152`

## V148 Document Notes

- Seedance 2.0 黄金样本库 V148 建设方案
- 版本: V148 (24 字段 × 155 行) | 升级焦点: +导演风格维度 + 三国/SLG 新增 40 样本 | 交付日期: 2026-04-24
- 一、升级概览 (V120 → V148)
- 二、新增第 24 字段: director_style_ref
- 定位: 视觉导演风格锚点, 每条样本标注 "主 + 辅" 两位导演 (最多 2 位)。该字段仅作为风格参考与检索入口, 不进 prompt_body。
- 字段规范:
- 格式: "D0X 风格名派 (主) + D0X 风格名派 (辅)"
- 主导演 = 视觉 DNA 主锚点, 辅导演 = 运镜/色彩/节奏补充锚点
- 差质量样本统一标记 "未分配 (差质量样本, 不纳入导演风格维度)"
- 命名全部对齐 DB-06 官方风格名 (例如 "D02 双雄浪漫主义动作派", 非零散简称)
- 核心使用原则 — IP 抽象:
- prompt_body 只出现 visual_dna_keywords (抽象视觉 DNA), 严禁出现真实导演姓名或电影片名
- director_style_ref 字段存导演代码, 团队内部使用; 产出 prompt 时通过 DB-06 查表替换为抽象视觉关键词
- 这层抽象保证 Seedance 不会被风格 "卡版权", 同时保留风格迁移能力
- 三、DB-06 导演风格字典 (14 条)
- 覆盖 华语动作 (7) + 国漫 CG (3) + 国际大师 (2) + 游戏 CG (2) = 14 条。每条 style_code 对应一套可抽取的视觉 DNA 关键词包。

## V148 Relevant Rows

- `CNWARSEQ01`: CNWARSEQ01 三国战场千军万马·高空俯瞰列阵 | CNWARSEQ01 三国战场千军万马·冲锋发起 | CNWARSEQ01 三国战场千军万马·接战肉搏 | CNWARSEQ01 三国战场千军万马·旗帜易主收尾
- `CNWARSEQ02`: CNWARSEQ02 三国将领一骑当千·主将登场 | CNWARSEQ02 三国将领一骑当千·突入敌阵 | CNWARSEQ02 三国将领一骑当千·单挑斩敌将 | CNWARSEQ02 三国将领一骑当千·回眸收势
- `CNWARSEQ03`: CNWARSEQ03 朝堂舌战群儒·廊柱全景 | CNWARSEQ03 朝堂舌战群儒·谋士开腔 | CNWARSEQ03 朝堂舌战群儒·群臣哗然 | CNWARSEQ03 朝堂舌战群儒·主君定论
- `SLGSEQ01`: SLGSEQ01 战略沙盘推演·宇宙级俯冲 | SLGSEQ01 战略沙盘推演·势力边界点亮 | SLGSEQ01 战略沙盘推演·行军轨迹展开 | SLGSEQ01 战略沙盘推演·穿入地面切换
- `SLGSEQ02`: SLGSEQ02 城池建设时间流逝·空地起基 | SLGSEQ02 城池建设时间流逝·墙体升起 | SLGSEQ02 城池建设时间流逝·城楼点灯 | SLGSEQ02 城池建设时间流逝·俯瞰全貌定格
- `SLGSEQ03`: SLGSEQ03 武将召唤光效·黑场涟漪启动 | SLGSEQ03 武将召唤光效·光阵聚合 | SLGSEQ03 武将召唤光效·武将显形 | SLGSEQ03 武将召唤光效·战袍翻飞定格
- `SLGSEQ04`: SLGSEQ04 多军团汇流攻城战·多路行军沙盘 | SLGSEQ04 多军团汇流攻城战·攻城器械推进 | SLGSEQ04 多军团汇流攻城战·城墙破碎 | SLGSEQ04 多军团汇流攻城战·胜旗插顶
