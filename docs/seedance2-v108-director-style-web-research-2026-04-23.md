# Seedance2 V108 Director Style Web Research 2026-04-23

## Route

- repo: `E:\codex\hope-kb`
- branch: `codex/contracts-freeze`
- anchor before this package: `75d8e52`
- package type: KB-only web research and director-style seed expansion evidence

This research supports internal KB style-lane expansion only. It does not
authorize product prompt text, visible UI taxonomy, V108 row import, positive
few-shot promotion, Qwen / Seedance calls, or `reference_control_core`.

## V108 Gaps Being Targeted

The source gap comes from normalized V108 staging:

- `国漫 / 热血打斗 = 11`
- `国漫 / 场域追逐 = 10`
- `国漫 / 群像表演 = 7`
- `原创 / 群像表演 = 8`
- `都市末世 / 工业压迫` appears across subway, overpass, ruins, evacuation,
  alarm, and infrastructure-pressure rows
- `舞台 / 群舞 / 国潮表演` appears across stage lighting, synchronized group
  movement, performer entrance, blackout, music beat, and final-freeze rows

## Search Keywords

- `雾山五行 林魂 分镜 动作 访谈 镜头 调度`
- `Fog Hill of Five Elements Lin Hun action choreography storyboard`
- `哪吒之魔童降世 分镜 特效镜头 制作 饺子`
- `哪吒之魔童闹海 特效镜头 洪流大战 制作 饺子`
- `长安三万里 谢君伟 邹靖 导演 专访 群像 场面`
- `长安三万里 幕后 制作 分镜 群像 追光动画`
- `灵笼 董相博 访谈 末世 科幻 场景 动画 制作`
- `灵笼 第二季 制作 动作捕捉 分镜 末世 场景`
- `天官赐福 太子悦神 惊鸿一瞥 动画 短片 官方`
- `我为歌狂之旋律重启 音乐 动画 舞台 制作 访谈`

## Accepted Candidates

| internal style lane | evidence links | representative works / scenes | V108 gap | why it guides shot language |
| --- | --- | --- | --- | --- |
| `国风武侠 / 仙侠动作调度` | [Fog Hill staff / action design](https://bgm.tv/subject/305515); [Ne Zha behind-the-scenes](https://m.1905.com/m/film/feature/2243707.shtml); [Ne Zha 2 large-scale action / effects report](https://finance.sina.cn/2025-02-07/detail-ineirmwe0752856.d.html?vt=4) | `雾山五行`; `哪吒之魔童降世`; `哪吒之魔童闹海` action-heavy sequences | `国漫 / 热血打斗`, `国漫 / 场域追逐`, bamboo rain, blade/scabbard continuity, roof-eave chase, qinggong verticality | `雾山五行` has public staff evidence for director/storyboard/action-design concentration; `哪吒` production notes expose storyboard volume, effects-shot volume, clear-action requirements, and physicalized combat staging. This supports movement axis, pause-before-burst, weapon continuity, vertical traversal, and readable impact, not only visual art style. |
| `国漫史诗群像 / 战争场面调度` | [Chang An director interview, Cinephilia / Film Art](https://cinephilia.net/84772/); [Beijing Evening News interview](https://news.bjd.com.cn/2023/07/14/10495943.shtml); [Ne Zha 2 crowd / flood-battle effects report](https://finance.sina.cn/2025-02-07/detail-ineirmwe0752856.d.html?vt=4) | `长安三万里`; `哪吒之魔童闹海` crowd / flood-battle reporting | `国漫 / 群像表演`, war oath, army formation, charge staging, crowd rhythm | `长安三万里` interviews describe historical research, storyboarding, location / ritual logic, and a long-form ensemble structure; `哪吒2` reports describe large crowd/flood action and rhythm. This supports formation depth, command hierarchy, army-scale blocking, and crowd-motion rhythm. |
| `都市末世 / 工业压迫调度` | [Ling Cage creator interview via Global Times / Sohu](https://www.sohu.com/a/921231673_121620820); [China Youth Daily / Sina season-2 production report](https://k.sina.com.cn/article_7857201856_1d45362c001902wz2k.html); [China News Hubei season-2 launch report](https://www.hb.chinanews.com.cn/news/2025/0525/417311.html) | `灵笼`; `灵笼 第二季`; lighthouse / ruins / evacuation / future battlefield scenes | urban apocalypse, subway / overpass / ruins, alarm layering, infrastructure pressure | Public interviews and reports frame `灵笼` as post-apocalyptic Chinese sci-fi and describe action capture, facial capture, high shot count, high-difficulty VFX, survival pressure, and future battlefield staging. This guides industrial spatial compression, evacuation blocking, alarm/sound pressure, and danger escalation. |
| `国潮舞台 / 原创表演调度` | [TGCF official Bilibili short announcement](https://www.bilibili.com/opus/1031137468618899457); [TGCF short episode / action guidance note](https://bgm.tv/ep/1418982); [I Am Crazy for Singing 2 Bilibili page](https://www.bilibili.com/bangumi/media/md28223062/); [I Am Crazy for Singing 2 staff / storyboard page](https://bgm.tv/subject/219141); [People / Shanghai music-animation report](https://sh.people.com.cn/n2/2020/1211/c350122-34468870.html) | `天官赐福 全新动画短片：太子悦神·惊鸿一瞥`; `我为歌狂之旋律重启` | `原创 / 群像表演`, stage, group dance, performer entrance, blackout, music beat, final freeze | `太子悦神` supplies ritualized performance and action-guidance evidence; `我为歌狂之旋律重启` supplies music-stage and storyboard / photography staff evidence. Together they support stage axis, spotlight timing, music-synced entrances, group blocking, audience/reaction cuts, and final-freeze staging. |

## Rejected / Not Added

| candidate | reason not added in this gate |
| --- | --- |
| generic `国漫导演` bucket | too broad; does not prove a camera-language capability tied to V108 rows |
| pure art-direction lanes from visually famous works | rejected unless the source also supports shot language, movement, staging, blocking, rhythm, or transition decisions |
| real-person imitation lanes | rejected; this gate only creates internal style channels / scheduling prototypes and does not ask any model to imitate a real director |
| extra 5th / 6th lanes for `美漫` or generic `日漫` | not needed for V108 gap closure; the current 7-director background model already covers broad action, pressure, close combat, emotion, and transition lanes, and this gate is scoped to the accepted Chinese-animation / performance gaps |

## Seed Decision

Add exactly four internal style-lane records:

- `director_08`: `内部风格通道：国风武侠/仙侠动作调度`
- `director_09`: `内部风格通道：国漫史诗群像/战争场面调度`
- `director_10`: `内部风格通道：都市末世/工业压迫调度`
- `director_11`: `内部风格通道：国潮舞台/原创表演调度`

Each added seed record must cite this research memo in `source_notes`.

## Boundary Assertions

- `rows_ready_for_product_import = 0`
- `rows_ready_for_positive_fewshot = 0`
- `product_ready_external_reference_handles = 0`
- `reference_control_core_coverage = 0`
- `v108_rows_imported_into_seed_examples = 0`
- `qwen_calls = 0`
- `seedance_calls = 0`
