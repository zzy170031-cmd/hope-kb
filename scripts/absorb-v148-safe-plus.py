import argparse
import copy
import hashlib
import json
import re
from collections import Counter
from pathlib import Path

import pandas as pd
from docx import Document


REPO_ROOT = Path(__file__).resolve().parents[1]
SEED_ROOT = REPO_ROOT / "seed" / "v0.2"
CURRENT_LIBRARY = SEED_ROOT / "golden_sample_library.json"
CURRENT_FAILURE = SEED_ROOT / "golden_sample_failure_mapping.json"
CURRENT_REPAIR = SEED_ROOT / "golden_sample_repair_mapping.json"
CURRENT_COVERAGE = SEED_ROOT / "golden_sample_field_coverage_rules.json"
CURRENT_SOURCE_REGISTER = SEED_ROOT / "source_register.json"
CURRENT_MANIFEST = SEED_ROOT / "manifest.json"
INPUT_DOCX = Path(r"C:\Users\Administrator\Desktop\claude\V148_最新交付\Seedance2.0黄金样本库V148建设方案_导演风格版.docx")
INPUT_XLSX = Path(r"C:\Users\Administrator\Desktop\claude\V148_最新交付\Seedance2.0黄金样本库V148_24字段导演风格版.xlsx")
OUTPUT_XLSX = Path(r"E:\codex\outputs\黄金样本库v120究极版_V120国产动漫三国SLG专项增强候选池_最终版.xlsx")
DESKTOP_OUTPUT_XLSX = Path(r"C:\Users\Administrator\Desktop\样本存储区\黄金样本库v120究极版_V120国产动漫三国SLG专项增强候选池_最终版.xlsx")
REPORT_PATH = REPO_ROOT / "docs" / "v120-v148-safe-plus-absorption-report-2026-04-24.md"
DATE = "2026-04-24"
SAFE_PLUS_SOURCE_ID = "seedance2_v120_safe_plus_final_xlsx_2026_04_24"
V148_XLSX_SOURCE_ID = "seedance2_v148_director_style_xlsx_2026_04_24"
V148_DOCX_SOURCE_ID = "seedance2_v148_plan_docx_2026_04_24"
SAFE_PLUS_PROVENANCE_ID = "seedance2_v120_v148_safe_plus_seed_package_2026_04_24"
CORE_FIELDS = [
    "technical_profile",
    "scene_performance_core",
    "camera_directing_core",
    "audio_directing_core",
    "continuity_negative_core",
]
FIELD_ORDER = [
    "shot_id",
    "library_status",
    "reserve_reason",
    "sample_type",
    "sequence_id",
    "shot_order",
    "sample_title",
    "style_cluster",
    "scene_category",
    "scene_tag",
    "quality_grade",
    "usable_for_fewshot",
    "technical_profile",
    "scene_performance_core",
    "camera_directing_core",
    "audio_directing_core",
    "continuity_negative_core",
    "reference_bundle",
    "ip_abstraction_note",
    "covered_points",
    "missed_points",
    "teaching_note",
    "prompt_body",
]
PLACEHOLDER_TOKENS = ["待补", "TODO", "TBD", "template"]
PROMPT_BANNED = [
    "Seedance",
    "Qwen",
    "Doubao",
    "三国",
    "王者荣耀",
    "英雄联盟",
    "LOL",
    "迪士尼",
    "宫崎骏",
    "今敏",
    "新海诚",
    "王家卫",
    "诺兰",
]


SEQUENCE_KITS = {
    "CNWARSEQ01": {
        "v148_anchor": ["CNWARSEQ01-S1", "CNWARSEQ01-S2", "CNWARSEQ01-S3", "CNWARSEQ01-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责军阵秩序、阵营压迫和旗面层次，辅锚点负责尘土重量、鼓点节拍和队列呼吸。",
        "ip_note": "只抽象中式古代战争与架空军政的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的军阵组织、秒点动作、风向/人数/兵器轨迹和旗面负向清单；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.8s 前排长兵同时抬起，2.4s 右侧骑阵第一次踏地产生赭黄尘带；风向固定为画右向画左，旗面延迟约 0.3s 摆动。",
                "perf": "补强：前景橹盾约 36 面，中层长兵约 60 杆，后层弓手以两列压在雾后；人物只做呼吸、换握和甲片轻碰，不做抢戏动作。",
                "camera": "补强：画面采用横向三分结构，左军 1/3、中轴留白 1/3、右军 1/3；镜头缓降时不越轴，保证阵线宽度和人数密度同时可读。",
                "audio": "补强：0.0-0.5s 全静默，0.8s 金属齐鸣，2.4s 低频蹄声冲击；号角只做远景提示，不压过军阵体量。",
                "negative": "补强：禁止旗帜乱码、兵种混站、现代迷彩图案、塑料甲面高光、扬尘方向与风向相反。",
                "covered": "军阵组织 / 人数密度 / 旗色层级 / 风向锚定 / 秒点动作 / 鼓点同步 / 负向清单 / reference handles / IP抽象",
                "story": "建立国战开场的空间、阵营和压迫关系，让后续攻防有可信起点。",
            },
            "2": {
                "tech": "补强：1.2s 前排盾面同步前倾，2.8s 第二排长兵压上，4.6s 队列整体前挪半身位；镜头速度控制在 0.5m/s 内，保留阵列重量。",
                "perf": "补强：盾阵前沿保持 12 面完整可读，矛尖朝向一致，脚步只出现两次重踏节拍；尘土随前缘推进滚动，不可乱飞。",
                "camera": "补强：镜头沿盾阵法线轻推，盾面占前景约 40%，中景留给长兵压近，后景维持旗面和山口出口，不让画面只剩盔甲贴脸。",
                "audio": "补强：1.2s 盾面摩擦与甲片碰撞对齐，2.8s 长兵落位给一次金属共振，4.6s 重踏给一次低频落点；尾声留 0.3s 紧张静默。",
                "negative": "补强：禁止盾面尺寸跳变、脚步方向漂移、长兵穿模、行军线折返、旗面朝向与风向冲突。",
                "covered": "盾阵压近 / 行进节拍 / 盾矛层次 / 前中后景组织 / 重踏同步 / 风尘一致 / 负向清单 / reserve gate",
                "story": "把军阵从静态展示推进到行动状态，服务命令即将传递的故事节拍。",
            },
            "3": {
                "tech": "补强：0.9s 左翼先展开，1.8s 右翼跟上，3.2s 中轴空隙形成包抄口，4.8s 尘带在两翼后方闭合；两翼速度差控制在 0.2s 内。",
                "perf": "补强：左右两翼各保留至少两层兵种差异，第一层持盾，第二层长兵或骑阵；动作重点是战术变化，不是无意义奔跑。",
                "camera": "补强：镜头先稳住中轴，再让两翼从画面边缘打开，包抄口必须在画面中央可读；不能为了热闹牺牲方向链。",
                "audio": "补强：左右翼展开各给一次鼓点，3.2s 包抄口成形时加入一声号令，4.8s 留 0.25s 静默让战术变化落地。",
                "negative": "补强：禁止两翼人数瞬间翻倍、兵器左右手互换、包抄方向反转、远景旗面消失、尘带遮住关键战术口。",
                "covered": "两翼展开 / 包抄口 / 方向链 / 多层兵种 / 鼓点节点 / 中轴留白 / 负向清单 / 会审抽象",
                "story": "把军阵组织转成可读的战术变化，让观众知道局势正在收紧。",
            },
            "4": {
                "tech": "补强：1.0s 令旗抬高，2.2s 半停顿，3.0s 旗面落下，4.2s 队列整体响应；风向固定为画右向画左，旗面展开延迟约 0.25s。",
                "perf": "补强：主将只做一次抬臂与压旗动作，前排士卒在旗落后同步压步，不能出现各做各的零散表演。",
                "camera": "补强：令旗位于上三分之一，响应队列位于下三分之二；镜头在旗落前微停，旗落后仅给 5-8cm 的呼吸式前推。",
                "audio": "补强：2.2s 前后给 0.2s 静默，3.0s 旗杆落位给一次闷响，4.2s 队列响应给一次整齐甲片声。",
                "negative": "补强：禁止旗面文字现代化、旗杆长度跳变、响应时间参差不齐、鼓点和落旗错位、镜头为炫技突然甩动。",
                "covered": "令旗调度 / 指令停顿 / 队列响应 / 风旗一致 / 画面停顿 / 闷响同步 / 负向清单 / reserve gate",
                "story": "收束建立序列，让战事从排列进入执行，为后续高光或攻城留入口。",
            },
        },
    },
    "CNWARSEQ02": {
        "v148_anchor": ["CNWARSEQ02-S1", "CNWARSEQ02-S2", "CNWARSEQ02-S3", "CNWARSEQ02-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责人物高光和兵器重量，辅锚点负责泥尘冲击、跟拍速度和收势停顿。",
        "ip_note": "只抽象中式古代战争、武将高光与单骑突阵的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的高光出场、突阵轨迹、兵器弧线、收势停顿和战场负向清单；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.7s 马背抬头，1.6s 护肩受侧光打亮，3.2s 兵器角度稳定在胸前 30°；镜头焦段偏中长，压住背景杂讯。",
                "perf": "补强：人物高光来自克制的停顿和视线，不靠夸张姿态；白鬃马只做两次踏步和一次短促喷气，战袍受风向统一向后掀起。",
                "camera": "补强：画面右侧留出 1/3 进攻方向，人物与马头处在黄金分割点，镜头以半步前推建立高光，不做大幅环绕。",
                "audio": "补强：0.0-0.4s 静默，0.7s 马蹄踏地，1.6s 甲片轻响，3.2s 低鼓回收；音效强调人物重量而不是大场面轰炸。",
                "negative": "补强：禁止头发和披风风向相反、人物脸型漂移、战袍金属感过塑、坐骑比例忽大忽小。",
                "covered": "武将高光 / 坐骑节奏 / 视线停顿 / 风向统一 / 焦段压缩 / 低鼓同步 / 负向清单 / reference handles",
                "story": "先让主将站住气场，再把后续突阵动作建立为可信延伸。",
            },
            "2": {
                "tech": "补强：0.6s 马蹄腾空，1.6s 横扫落点，2.8s 穿过敌阵缝隙；兵器弧线控制在约 0.8m，跟拍速度约 4m/s。",
                "perf": "补强：敌兵倒地控制在前排 4 人以内，血雾只做极轻提示；重点是速度、重量和轨迹清晰，不是夸张残酷。",
                "camera": "补强：人物始终在画面右侧 MS 位，敌阵从画左压向画右；突阵方向固定，绝不在中段变向。",
                "audio": "补强：每次扫中有 0.04s 贴帧金属声，2.8s 出阵前给 0.2s 静默；鼓点频率随突进节奏提高。",
                "negative": "补强：禁止兵器长度跳变、敌兵人数漂移、粒子方向与扫刀矛盾、镜头突然掉轴。",
                "covered": "突阵速度 / 兵器弧线 / 敌兵落点 / 跟拍距离 / 打击同步 / 节奏加速 / 负向清单 / reserve gate",
                "story": "把高光人物从亮相推进到真实可感的突阵动作，建立一骑破阵的说服力。",
            },
            "3": {
                "tech": "补强：1.1s 破盾第一击，2.3s 第二段追击，3.4s 兵器回收到肘线内；盾裂方向与人物前冲方向一致。",
                "perf": "补强：近身段落只保留一条主兵器轨迹和一条副反应线，避免满屏乱兵器；前景碎片数量受控，突出主击点。",
                "camera": "补强：构图压低机位让兵器轨迹吃满对角线，镜头只跟一条主动作链，不能为了热闹四处找反应。",
                "audio": "补强：1.1s 盾裂声偏重，2.3s 追击声偏尖，3.4s 收回时削掉环境层，留一次短静默让击点成立。",
                "negative": "补强：禁止盾裂碎片飞向反方向、主兵器穿模、敌兵站位瞬间重排、尘土遮死关键击点。",
                "covered": "破盾轨迹 / 单主动作链 / 低机位压迫 / 盾裂音色 / 碎片方向 / 负向清单 / 会审抽象",
                "story": "把高光从速度推进到命中重量，证明人物不是只会摆姿态。",
            },
            "4": {
                "tech": "补强：0.9s 回拉兵器，2.1s 呼吸停顿，3.0s 升格定住肩线，4.5s 余势回落；披风和长髯只保留一次惯性摆动。",
                "perf": "补强：收势的重点是胜利后的克制，不是继续乱动；背景敌兵退成虚化层，前景只留人物与兵器。",
                "camera": "补强：镜头在收势点轻微回收，人物肩线保持稳定，背景压成浅景；定格前后不能突然切光或切轴。",
                "audio": "补强：2.1s 给一次可感知静默，3.0s 升格时削去鼓点，只保留披风和呼吸；4.5s 再补极轻环境回声。",
                "negative": "补强：禁止升格时衣甲材质变化、人物比例漂移、收势后又多做一套动作、背景兵器乱穿。",
                "covered": "收势停顿 / 升格回收 / 肩线稳定 / 呼吸静默 / 披风惯性 / 负向清单 / reserve gate",
                "story": "把人物高光收束成可记忆的尾态，为下一个序列留下情绪余量。",
            },
        },
    },
    "CNWARSEQ03": {
        "v148_anchor": ["CNWARSEQ03-S1", "CNWARSEQ03-S2", "CNWARSEQ03-S3", "CNWARSEQ03-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责权谋空间、席位秩序和视线压迫，辅锚点负责甩镜节奏、拍案重量和群像反应层次。",
        "ip_note": "只抽象中式古代朝堂权谋与军政决策的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的朝堂权谋结构、席位人数、甩镜反应、拍案重量和器物负向清单；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：景别维持 ELS 到 LS，前 1.2s 给出廊柱、席位和地面纹路，2.6s 才把视线压向沙盘或主位；光源固定自画左上斜入。",
                "perf": "补强：两列席位各至少 6 人，主位与谋臣之间留一条可读行动线；人物只做低声交流、抬眼和袖摆整理，不抢发言者的位阶。",
                "camera": "补强：廊柱做纵向分割线，座席与中轴形成对称秩序；镜头移动极轻，只为把空间权力关系交代清。",
                "audio": "补强：环境声以衣料、脚步、竹简和低声议论为主；开场 0.4s 静默，随后逐层打开空间反射。",
                "negative": "补强：禁止现代会议室质感、席位数量跳变、柱距缩放不一、地面纹样现代化、器物材质塑料感。",
                "covered": "权谋空间 / 席位秩序 / 中轴关系 / 光源方向 / 器物层级 / 低声环境 / 负向清单 / IP抽象",
                "story": "建立军政决策空间，让后续发言、急报和军令都有权力基础。",
            },
            "2": {
                "tech": "补强：0.8s 谋士开口，1.9s 折扇或令牌轻压桌面，3.0s 主位给一次眼神反馈；焦段中景，压住背景喧闹。",
                "perf": "补强：发言者只做一次关键手势，其余靠目光、呼吸和停顿建立压迫；周围陪席保持压低反应，不抢主位。",
                "camera": "补强：镜头锁发言者肩线，桌案和人物视线形成三角构图；发言期间不做无意义推拉，只在重点句后给微停。",
                "audio": "补强：0.8s 开口清晰贴口型，1.9s 桌面轻响，3.0s 环境声回落 20% 让目光反馈成立；留 0.25s 停顿。",
                "negative": "补强：禁止口型与台词节拍错位、袖口长度跳变、桌案器物位置漂移、背景群臣同时乱动。",
                "covered": "发言压迫 / 肩线锁定 / 桌案节拍 / 视线反馈 / 环境回落 / 负向清单 / 会审抽象",
                "story": "把权谋重心从空间转到发言者，让决策冲突真正启动。",
            },
            "3": {
                "tech": "补强：0-0.8s 甩镜完成，1.6s 拍案，1.8s 竹简或茶盏轻抖，2.2s 次级人物起身反驳；群臣数量与席位间距必须稳定。",
                "perf": "补强：前排 3 人做主反应，中后排做延迟反应，形成层次；器物只做一次受力弹动，不可满桌乱飞。",
                "camera": "补强：甩镜方向与上一镜视线方向一致，停下后只锁主反应群；群臣列尽量吃满横幅，避免掉人。",
                "audio": "补强：0.2s 甩镜前静默，1.6s 拍案低频，1.8s 竹简清脆，2.2s 人声层次打开；音乐不上旋律，只让空间声撑住。",
                "negative": "补强：禁止群臣动作全一致、茶盏翻覆过度、竹简材质现代纸感、反应人数忽多忽少。",
                "covered": "群像反应 / 甩镜节奏 / 拍案重量 / 器物弹动 / 席位稳定 / 负向清单 / reserve gate",
                "story": "用群臣哗然把前一镜的话语杀伤力落成可见冲击。",
            },
            "4": {
                "tech": "补强：1.0s 主位定住，2.4s 军令或令牌离桌，3.6s 侍从转身出帐；光源保持单侧落下，突出决断的方向性。",
                "perf": "补强：主位动作只做一次，侍从响应干净利落；群臣反应收回，场面从喧哗转为执行。",
                "camera": "补强：镜头从群像收回到主位与出令路径，保证主位、令牌和出帐方向同屏可读；不搞炫技转场。",
                "audio": "补强：2.4s 令牌落位或离桌给一次闷响，3.6s 脚步出帐清晰，尾声留 0.3s 空气感静默。",
                "negative": "补强：禁止主位服色变化、令牌尺寸漂移、出帐方向与前镜视线反转、背景群臣继续抢戏。",
                "covered": "定论落点 / 令牌路径 / 执行转场 / 单侧光源 / 脚步同步 / 负向清单 / reserve gate",
                "story": "把权谋辩论收束成真正的命令执行，让序列完成闭环。",
            },
        },
    },
    "CNWARSEQ04": {
        "v148_anchor": ["SLGSEQ04-S1", "SLGSEQ04-S2", "SLGSEQ04-S3", "SLGSEQ04-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责攻城几何、梯具压迫和旗面落点，辅锚点负责城墙受击、烟尘节拍和守军反应层次。",
        "ip_note": "只抽象中式古代攻城与守城的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的攻城组织、器械推进、城门受击、守城旗面和攻防负向清单；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.6s 警钟首次撞响，1.8s 城头火盆和旗面同时受风，3.4s 远处梯具与盾墙进入视口；风向固定为画右向画左。",
                "perf": "补强：城头守军至少 8 人分两层站位，前层观察、后层备箭；钟绳只拉一次，避免连续乱晃。",
                "camera": "补强：镜头先给城头警戒线，再把攻城方向压入下方视野；城墙垂直线要稳，不能为了紧张感把城体拍歪。",
                "audio": "补强：0.6s 钟响低频延长，1.8s 火盆和旗面给风噪，3.4s 远处器械摩擦声进入；钟响后留 0.2s 空气感。",
                "negative": "补强：禁止城砖现代感、钟体材质塑料化、守军人数跳变、旗面乱码、警戒方向漂移。",
                "covered": "警戒建立 / 钟响节拍 / 城头层级 / 风旗一致 / 远近威胁 / 负向清单 / 会审抽象",
                "story": "建立守城方已经感知到攻城压力，让后续梯具逼近有先兆。",
            },
            "2": {
                "tech": "补强：1.1s 第一架云梯抬起，2.0s 第二架逼近城墙，3.3s 盾墙与推梯步频同步；镜头保持俯角但不过度压扁深度。",
                "perf": "补强：梯具至少两层人力驱动，前排盾墙护头，后排推梯；动作重点是重、慢、稳，而不是狂奔。",
                "camera": "补强：城墙边线与梯具方向形成斜向冲突，表现压迫；镜头沿城头轻移跟住最危险的一架云梯。",
                "audio": "补强：1.1s 木梯撞击声，2.0s 盾面摩擦，3.3s 集体步频低鼓同步；尾声留 0.25s 压抑静默。",
                "negative": "补强：禁止梯具长度跳变、步频失齐、盾墙缝隙忽大忽小、梯脚与地面浮空。",
                "covered": "云梯推进 / 盾墙护头 / 重量步频 / 城墙斜压 / 撞击同步 / 负向清单 / reserve gate",
                "story": "把攻城威胁从远景预警推进到真实贴近城墙的压迫。",
            },
            "3": {
                "tech": "补强：0.9s 冲车第一次撞门，2.1s 木屑与铁钉反震，3.5s 第二次重击后门体内陷；烟尘只沿门缝和地面扩散。",
                "perf": "补强：前景只跟一架主冲车，旁侧器械和士卒作为辅助层；守军反应集中在门楼与墙垛，不到处乱喊乱跑。",
                "camera": "补强：镜头锁门体中轴，冲车行进线和门板裂纹都要可读；不能把撞点切到画外。",
                "audio": "补强：0.9s、3.5s 两次重击都给厚重闷响，2.1s 反震给尖锐木屑声；撞门间隙削弱鼓点让冲击更重。",
                "negative": "补强：禁止门体材质塑料感、冲车大小漂移、烟尘反向飘、裂纹突然消失、器械和门体碰撞错位。",
                "covered": "城门受击 / 门缝烟尘 / 裂纹追踪 / 冲车主线 / 闷响层次 / 负向清单 / reserve gate",
                "story": "把攻城从逼近推进到真正破防，让守城压力具象化。",
            },
            "4": {
                "tech": "补强：1.0s 守方旗影重新站稳，2.4s 城头队形回填，3.8s 火光和晨光同时压住旗面；镜头尾声只给极轻上扬，不进入 UI。",
                "perf": "补强：重点是守住，不是庆功；旗面、墙垛和守军站位形成稳定三角，队列只做克制回防动作。",
                "camera": "补强：中轴给旗影，左右两侧给回防队列，镜头上扬幅度小于 8°，让“守住”而非“炫酷”成为画面中心。",
                "audio": "补强：1.0s 旗面猎猎，2.4s 回防脚步和甲片声同步，尾声留 0.35s 静默让守住的情绪站住。",
                "negative": "补强：禁止旗帜变色、守军数量飘移、墙体突然恢复无损、灯火与晨光方向冲突、胜利化配乐误导语义。",
                "covered": "守城收势 / 旗影稳定 / 回防层级 / 晨火双光 / 克制静默 / 负向清单 / reserve gate",
                "story": "把攻守序列收束在“城还守得住”的状态，为后续更大规模攻防留接口。",
            },
        },
    },
    "SLGSEQ01": {
        "v148_anchor": ["SLGSEQ01-S1", "SLGSEQ01-S2", "SLGSEQ01-S3", "SLGSEQ01-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责沙盘视口、地形比例和边界可读性，辅锚点负责轨迹节拍、镜头穿入和 UI 呼吸感。",
        "ip_note": "只抽象策略沙盘、势力边界和行军视口的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的沙盘视口、边界点亮、行军轨迹和穿入切换能力；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.8s 先给山河轮廓，1.9s 关隘和河道浮出，3.4s 势力边界以低亮光带显现；视口保持 16:9 内可读比例。",
                "perf": "补强：地图元素必须先有地形、再有势力、最后有行动线；不允许一上来就满屏 UI 图标遮地图。",
                "camera": "补强：镜头从纯俯视缓慢下沉到轻等距视口，地图中心不漂移，山河关系保持整体可读。",
                "audio": "补强：0.0-0.4s 轻风和纸面展开声，1.9s 地形层级打开，3.4s 边界点亮给一声轻 UI 铃音；留 0.25s 静默看全局。",
                "negative": "补强：禁止现代地图纹理、品牌化按钮、边界线漂浮离地、地形比例失真、视口突然旋转。",
                "covered": "沙盘视口 / 地形比例 / 边界点亮 / 下沉镜头 / 轻 UI 声 / 负向清单 / reference handles",
                "story": "建立策略视口的世界关系，让后续关隘聚焦和斥候入画有空间依据。",
            },
            "2": {
                "tech": "补强：1.1s 关隘区域亮起，2.2s 两条支路同时出现，3.5s 视口停在要冲位置；边界线与地形起伏贴合。",
                "perf": "补强：主关隘必须是唯一视觉中心，周围支路只做辅助提示；图标数量控制在 3 个以内，避免广告式堆满。",
                "camera": "补强：镜头从全局压向关隘，但仍保留一条回到全图的空间线索；不把画面切成只剩 UI 图块。",
                "audio": "补强：关隘点亮给短促木鱼式 UI 音，2.2s 支路出现给两次轻击，尾声留 0.2s 空气感。",
                "negative": "补强：禁止关隘轮廓现代化、支路方向突变、图标遮挡地形、视口缩放忽快忽慢。",
                "covered": "关隘聚焦 / 支路读图 / 视口压近 / UI 节拍 / 边界贴地 / 负向清单 / reserve gate",
                "story": "把全局视图收束到真正影响局势的要冲位置，服务后续行动线。",
            },
            "3": {
                "tech": "补强：0.8s 第一条轨迹展开，1.6s 第二条和斥候图标同步出现，2.6s 第三条轨迹补齐；箭头拖尾长度和速度一致。",
                "perf": "补强：轨迹至少区分主线与侦查线，斥候入画只做一次切入，不可满屏乱跑；都城和起点必须可见。",
                "camera": "补强：镜头保持地图中心稳定，围绕主轨迹轻绕拍小于 30°；所有轨迹必须能一眼追到终点。",
                "audio": "补强：每条轨迹各给一声轻鼓和一声推轨摩擦，3.6s 多轨迹共存后留 0.4s 静默。",
                "negative": "补强：禁止轨迹交叉自相矛盾、箭头前后颠倒、起点不在城市、粒子颜色乱套、UI 文字乱码。",
                "covered": "行军轨迹 / 斥候入画 / 起终点可读 / 轨迹节拍 / 轻绕拍 / 负向清单 / reserve gate",
                "story": "把策略视口从静态地图推进成真正可推演的行动网络。",
            },
            "4": {
                "tech": "补强：1.0s 轨迹收束回边界，2.2s 视口轻抬，3.4s 保留一秒全局停顿；边界亮度回落但不消失。",
                "perf": "补强：回边界的重点是让观众记住势力关系，而不是再加新信息；地图元素只做回收不做新增。",
                "camera": "补强：镜头从局部回到中广角视口，确保边界、关隘和主路径同屏；不做炫目的翻转切换。",
                "audio": "补强：边界回亮给一次轻提示，3.4s 停顿时只保留风声与极轻 UI 底噪。",
                "negative": "补强：禁止边界突然断开、地图比例跳变、回收过程卡顿、图标在停顿里乱闪。",
                "covered": "边界回收 / 全局停顿 / 信息回看 / 视口稳定 / UI 底噪 / 负向清单 / reserve gate",
                "story": "把斥候与行军信息重新归回全局，让这一组视口建立完整闭环。",
            },
        },
    },
    "SLGSEQ02": {
        "v148_anchor": ["SLGSEQ01-S3", "SLGSEQ04-S1", "SLGSEQ04-S2", "SLGSEQ04-S3"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责多路线调度、补给节点和联盟合围逻辑，辅锚点负责光轨密度、攻城器械权重和撤退线提示。",
        "ip_note": "只抽象策略行军、补给阻断和联盟合围的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的多路线出征、补给受阻、联盟合围和撤退线提示能力；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.7s 第一条主路线亮起，1.4s 两条支线同步，2.8s 三路汇向同一城池；路线颜色、箭头大小和速度层级分开。",
                "perf": "补强：主路线负责攻击，支线负责补给或包抄；地图上至少留出一条未启用路线，避免像满屏撒线。",
                "camera": "补强：镜头停在主城与三路出征线构成的三角区，不把视口拖出地图中心。",
                "audio": "补强：每条路线起步给一次轻鼓，主线多一层低频；2.8s 汇向同城时给一次集结提示音。",
                "negative": "补强：禁止路线从荒地起步、箭头大小跳变、颜色与势力不匹配、路线互相遮挡看不清。",
                "covered": "多路线出征 / 主支线分工 / 集结城池 / 轨迹层级 / 集结音效 / 负向清单 / reserve gate",
                "story": "把出征从单一路线升级成真正可读的多线推进，为后续阻断和合围埋下逻辑。",
            },
            "2": {
                "tech": "补强：1.0s 补给线变暗，2.1s 中转节点闪红，3.4s 主线速度下降；补给受阻要通过亮度、速度和节点状态同时表现。",
                "perf": "补强：阻断不靠大爆炸，而靠路线停滞、节点闪烁和队列压缩来表达；地图仍需保持清晰可读。",
                "camera": "补强：镜头围绕补给节点轻缩放，既看到主线，也看到被截断的支线；不能只剩一个红点。",
                "audio": "补强：1.0s 光轨衰减给一声低沉 UI 警报，2.1s 节点闪红对齐短促敲击，尾声留 0.25s 静默。",
                "negative": "补强：禁止补给线凭空消失、节点位置漂移、现代告警图标、文字乱码、警报声过度品牌化。",
                "covered": "补给阻断 / 节点警报 / 路线降速 / 局部缩放 / 告警音效 / 负向清单 / reserve gate",
                "story": "让观众清楚看到推进为什么卡住，而不是只知道战线忽然不动。",
            },
            "3": {
                "tech": "补强：0.9s 友军第三路加入，1.8s 合围圈闭合一半，3.1s 攻城器械推进到城外；路线弧度和汇流点必须提前可读。",
                "perf": "补强：联盟合围重点是方向收束和包围口，不是地图上越多线越好；器械只做一条主推进线。",
                "camera": "补强：镜头在合围圈中心略微抬升，既看得见汇流，又看得见目标城外的器械前沿。",
                "audio": "补强：三路合围分别有三次节拍，3.1s 器械推进给一次沉重木轮声；3.6s 留 0.2s 停顿看合围成形。",
                "negative": "补强：禁止合围口闭合方向反转、器械速度漂移、路线遮死目标城、不同军团颜色混乱。",
                "covered": "联盟合围 / 汇流口 / 器械推进 / 方向收束 / 三路节拍 / 负向清单 / reserve gate",
                "story": "把多线推进收束为真正的包围优势，让战局进入压城阶段。",
            },
            "4": {
                "tech": "补强：1.2s 撤退线亮起，2.4s 主线反向收缩，3.8s 保留一条安全撤离通道；撤退线颜色要和主攻线明显区分。",
                "perf": "补强：撤退线不是失败炫技，而是信息保护；只点亮关键退路和补给出口，避免满图都是逃跑箭头。",
                "camera": "补强：镜头回到能同时看到主城、退路和补给点的位置，形成完整战报式信息面板。",
                "audio": "补强：撤退线亮起给一声低频 UI 提示，2.4s 反向收缩给摩擦回卷声；尾声保留 0.35s 低噪静默。",
                "negative": "补强：禁止撤退线与进攻线同色、箭头方向错误、退路穿山越河、界面字标乱码。",
                "covered": "撤退线 / 反向收缩 / 安全通道 / 战报信息面板 / UI 提示 / 负向清单 / reserve gate",
                "story": "把战局从合围推进到风险管理，让观众知道哪些线还能安全退出。",
            },
        },
    },
    "SLGSEQ03": {
        "v148_anchor": ["SLGSEQ02-S1", "SLGSEQ02-S2", "SLGSEQ02-S3", "SLGSEQ02-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责城建演进、光影时间流逝和材质升级，辅锚点负责夯石节拍、资源反馈和昼夜季节变化。",
        "ip_note": "只抽象城建演进、资源反馈和时间流逝的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的开荒立桩、筑墙升级、资源反馈和昼夜季节能力；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.8s 地桩落下，1.9s 基础格网亮起，3.2s 木料和石料堆位成形；地面比例和建筑基址必须匹配。",
                "perf": "补强：开荒不是空地瞬间变城，而是先有桩位、再有基础、最后有材料堆放；工人动作控制在铺桩和搬运两类。",
                "camera": "补强：镜头从轻俯视切到可读的等距城建视口，桩位网格必须一眼可数，避免虚浮。",
                "audio": "补强：0.8s 地桩落位有木击声，1.9s 格网点亮给轻 UI 铃音，3.2s 材料堆位有木轮和石块摩擦。",
                "negative": "补强：禁止现代施工机械、地面网格漂浮、桩位乱歪、建筑基址比例失真、文字标签乱码。",
                "covered": "开荒立桩 / 格网建立 / 材料堆位 / 等距视口 / 工序节拍 / 负向清单 / reserve gate",
                "story": "先把一片空地变成可施工的秩序面，为后续升级建立逻辑。",
            },
            "2": {
                "tech": "补强：1.0s 夯石打点，2.4s 墙体从 1m 升到 5m，3.2s 角楼木构出现；光影从晨到午连续变化。",
                "perf": "补强：墙体升级靠工序推进和 time-lapse 提示，不靠突然弹出成品；民夫数量稳定，动作分层可辨。",
                "camera": "补强：镜头随墙体上升约 2.5m，但保持城墙相对画面稳定；天空光带作为时间标记，不可乱跳。",
                "audio": "补强：夯石为主节拍，木轮和呼号做副层，墙升完前留 0.3s 静默；由纯环境过渡入轻乐器底色。",
                "negative": "补强：禁止城墙现代砖感、民夫人数骤变、角楼形制错乱、日影反向、建筑几何断裂。",
                "covered": "筑墙升级 / time-lapse / 日影变化 / 工序分层 / 夯石节拍 / 负向清单 / reserve gate",
                "story": "让建设进度可视化，把资源投入转成真正看得见的城防成长。",
            },
            "3": {
                "tech": "补强：0.9s 仓库或资源点回亮，1.8s 数值反馈区短暂跳动，3.0s 运输线与仓点形成闭环；UI 仅做可见反馈不遮主体。",
                "perf": "补强：资源反馈要看到生产、运输和入库三步，而不是只看数值弹字；运料车或人力线路必须清楚。",
                "camera": "补强：镜头在资源点和仓库之间做小范围平移，既能看建筑变化，也能看资源入库反馈。",
                "audio": "补强：1.8s UI 反馈给清脆木珠/铜铃声，3.0s 入库时给短促确认音；尾声只保留低噪环境。",
                "negative": "补强：禁止现代数字字体、品牌化按钮、数值遮住主体、资源图标材质塑料感、运输线漂移。",
                "covered": "资源反馈 / 生产入库 / 轻 UI 面板 / 运输闭环 / 清脆确认音 / 负向清单 / reserve gate",
                "story": "把建设成果转成真正的收益反馈，让城建不只是在长墙，而是在形成系统循环。",
            },
            "4": {
                "tech": "补强：1.0s 昼转夕，2.2s 夜灯点亮，3.4s 季节层变化轻落；昼夜变化和建筑状态必须保持同一视口比例。",
                "perf": "补强：时间变化只服务城池成长，不让天气成为主角；灯光、屋顶、地面植被各做一层轻变化即可。",
                "camera": "补强：镜头保持高位稳视，不做大绕拍；让观众看到同一城池在不同时间层的连续变化。",
                "audio": "补强：昼夜切换削弱施工噪声，夜灯点亮给极轻 UI 铃音，季节变化只做环境床变化；尾声留 0.4s 静默看全貌。",
                "negative": "补强：禁止季节跳变过猛、光源方向冲突、灯火现代霓虹化、地表材质忽换、界面文字乱码。",
                "covered": "昼夜季节 / 灯火层次 / 同视口连续性 / 环境床变化 / 全貌停顿 / 负向清单 / reserve gate",
                "story": "让一组城建镜头收束成完整成长时间线，证明这座城在持续演进。",
            },
        },
    },
    "SLGSEQ04": {
        "v148_anchor": ["SLGSEQ04-S1", "SLGSEQ04-S2", "SLGSEQ04-S3", "SLGSEQ04-S4"],
        "meeting": "内部双锚点会审只保留抽象能力：主锚点负责战报 UI、兵损反馈和倒计时结构，辅锚点负责卷轴揭示、数据层次和收束闭环。",
        "ip_note": "只抽象战报 UI、兵损反馈和资源倒计时的镜头语法，不复刻任何外部作品、角色专名、游戏名、商业标识或个人风格。",
        "missed": "已吸收 safe-plus 的卷轴揭示、兵损反馈、倒计时和战报收束能力；当前仍保留 reserve，仅作为能力池候选，不进入 official/Yes。",
        "shots": {
            "1": {
                "tech": "补强：0.8s 卷轴展开，1.9s 战区摘要面板显现，3.1s 指向线落到主战区；卷轴与 UI 层之间过渡要平顺。",
                "perf": "补强：卷轴揭示重点是信息进场顺序，先范围、再重点、最后指令；不要一开场就把所有面板全亮。",
                "camera": "补强：镜头从卷轴边缘推入战区摘要，保证标题区、地图区和摘要区三块都有呼吸空间。",
                "audio": "补强：0.8s 纸面展开声，1.9s 面板点亮给清脆 UI 音，3.1s 指向线落点给一次短促确认音。",
                "negative": "补强：禁止现代办公 UI 风格、卷轴材质太新、面板堆叠遮地图、文字乱码、品牌化图标。",
                "covered": "卷轴揭示 / 面板进场顺序 / 指向线 / UI 呼吸空间 / 提示音 / 负向清单 / reserve gate",
                "story": "建立战报入口，让后续兵损和资源信息有可信的信息框架。",
            },
            "2": {
                "tech": "补强：1.0s 兵损图形开始变化，2.0s 主损失项高亮，3.3s 伤亡结构稳定；图形和图标的变化顺序必须可追踪。",
                "perf": "补强：兵损反馈只突出主损失与补偿关系，不堆满所有数据；图形变化服务阅读，不服务炫酷特效。",
                "camera": "补强：镜头聚焦主损失区，周边面板保留但轻虚化，保证眼睛知道先看哪一块。",
                "audio": "补强：每次关键数值变化给一次干净击音，2.0s 主损失高亮配一声低频确认，尾声留 0.2s 安静阅读时间。",
                "negative": "补强：禁止伤亡图形抽象到看不懂、数字滚动抖动、品牌化图标、现代游戏专名、字形乱码。",
                "covered": "兵损反馈 / 主损失高亮 / 阅读顺序 / 数值击音 / 面板聚焦 / 负向清单 / reserve gate",
                "story": "把结果从“输了多少”整理成观众能立刻读懂的代价结构。",
            },
            "3": {
                "tech": "补强：0.9s 倒计时启动，1.8s 资源条缩减，3.0s 关键补给节点闪烁；时钟、资源条和节点告警必须联动。",
                "perf": "补强：倒计时不是独立 UI，而是和资源压力绑定；画面要同时看到剩余时间、资源状态和警戒对象。",
                "camera": "补强：镜头保持面板中心稳定，关键节点放在右侧阅读区，倒计时位于上方，资源条位于下方，不互相抢位。",
                "audio": "补强：倒计时每秒不连续鸣叫，只在关键阈值给一次提示；1.8s 资源缩减给低沉回卷声。",
                "negative": "补强：禁止时间跳秒不稳、资源条颜色混乱、节点闪烁无节制、现代品牌式 UI 发光、文字乱码。",
                "covered": "资源倒计时 / 面板布局 / 阈值提示 / 资源缩减 / 节点告警 / 负向清单 / reserve gate",
                "story": "把结果压力从静态数字推进成时间驱动的决策紧迫感。",
            },
            "4": {
                "tech": "补强：1.2s 结论栏收束，2.4s 主战区标记稳定，3.8s 画面留出可阅读停顿；所有 UI 模块在结尾回到一个统一层级。",
                "perf": "补强：收束重点是明确结论，不再追加新信息；只保留结论、损失摘要和下一步提示三块核心信息。",
                "camera": "补强：镜头极轻后退，让结论区、主战区和摘要区同屏且不拥挤；结尾稳定不抖动。",
                "audio": "补强：2.4s 结论确认音落下后，环境只留极轻底噪；3.8s 停顿时不再有连续提示声。",
                "negative": "补强：禁止结论栏跳变、图层前后错位、字体混乱、面板遮挡主战区、品牌/IP 泄漏。",
                "covered": "战报收束 / 结论栏 / 模块归位 / 阅读停顿 / 极轻底噪 / 负向清单 / reserve gate",
                "story": "把一组信息镜头收成真正可投放的战报闭环，让观众带着结论离开。",
            },
        },
    },
}


def load_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


def write_json(path: Path, payload: dict):
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=4) + "\n", encoding="utf-8")


def sha256_file(path: Path) -> str:
    sha = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            sha.update(chunk)
    return sha.hexdigest()


def canonical_bundle_hash(repo_root: Path, bundle_order: list[str]) -> str:
    sha = hashlib.sha256()
    for rel in bundle_order:
        sha.update((repo_root / rel).read_bytes().replace(b"\r\n", b"\n"))
    return sha.hexdigest()


def parse_scene_tags(scene_tag: str) -> list[str]:
    return [part.strip() for part in re.split(r"[，,、/；;]\s*", scene_tag) if part.strip()]


def machine_slug(sample_id: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", sample_id.lower()).strip("_")


def append_unique(base: str, extra: str) -> str:
    base = base.strip()
    extra = extra.strip()
    if not extra:
        return base
    if extra in base:
        return base
    return f"{base}\n{extra}" if base else extra


def normalize_prompt_text(text: str) -> str:
    text = text.replace("三国", "古代军政")
    text = text.replace("游戏名", "外部游戏名称")
    text = text.replace("品牌名", "外部品牌名称")
    text = text.replace("director_style_ref", "内部会审抽象")
    text = text.replace("Seedance", "生成系统")
    return text


def build_prompt_body(source_fields: dict) -> str:
    reference_block = source_fields["reference_bundle"].strip()
    prompt = "\n\n".join(
        [
            "生成一个动漫分镜视频镜头。",
            "【技术参数】\n" + source_fields["technical_profile"].strip(),
            "【画面与表演】\n" + source_fields["scene_performance_core"].strip(),
            "【镜头调度】\n" + source_fields["camera_directing_core"].strip(),
            "【声音设计】\n" + source_fields["audio_directing_core"].strip(),
            "【连续性与禁止项】\n" + source_fields["continuity_negative_core"].strip(),
            "【参考素材】\n" + reference_block,
            "【IP 抽象说明】\n" + source_fields["ip_abstraction_note"].strip(),
        ]
    ).strip()
    return normalize_prompt_text(prompt)


def enhance_candidate_row(record: dict) -> dict:
    enhanced = copy.deepcopy(record)
    source = enhanced["source_fields"]
    sequence_id = source["sequence_id"]
    shot_order = source["shot_order"]
    kit = SEQUENCE_KITS[sequence_id]
    shot = kit["shots"][shot_order]

    source["technical_profile"] = append_unique(source["technical_profile"], shot["tech"])
    source["scene_performance_core"] = append_unique(source["scene_performance_core"], shot["perf"])
    source["camera_directing_core"] = append_unique(source["camera_directing_core"], shot["camera"])
    source["audio_directing_core"] = append_unique(source["audio_directing_core"], shot["audio"])
    source["continuity_negative_core"] = append_unique(source["continuity_negative_core"], shot["negative"])
    source["covered_points"] = shot["covered"]
    source["missed_points"] = kit["missed"]
    source["ip_abstraction_note"] = kit["ip_note"]
    source["teaching_note"] = (
        f"V120 safe-plus reserve 精修。故事功能：{shot['story']}。"
        f"{kit['meeting']}。"
        "本镜继续只作为内部能力池候选，不进入 official / Yes。"
    )
    source["prompt_body"] = build_prompt_body(source)

    enhanced["classification"]["scene_tags"] = parse_scene_tags(source["scene_tag"])
    enhanced["validator_evidence"]["covered_points"] = source["covered_points"]
    enhanced["validator_evidence"]["missed_points"] = source["missed_points"]
    enhanced["validator_evidence"]["teaching_note"] = source["teaching_note"]
    enhanced["negative_sample"]["reserve_reason"] = source["reserve_reason"]
    enhanced["v3_core_coverage"]["source_coverage_statement"] = source["covered_points"]
    enhanced["v3_core_coverage"]["source_missing_statement"] = source["missed_points"]
    return enhanced


def update_failure_record(record: dict, library_record: dict) -> dict:
    updated = copy.deepcopy(record)
    source = library_record["source_fields"]
    updated["validator_evidence"]["covered_points"] = source["covered_points"]
    updated["validator_evidence"]["missed_points"] = source["missed_points"]
    updated["validator_evidence"]["teaching_note"] = source["teaching_note"]
    return updated


def update_repair_record(record: dict, library_record: dict) -> dict:
    updated = copy.deepcopy(record)
    source = library_record["source_fields"]
    updated["planned_repair_inputs"]["missed_points"] = source["missed_points"]
    updated["planned_repair_inputs"]["teaching_note"] = source["teaching_note"]
    updated["planned_repair_inputs"]["style_cluster"] = source["style_cluster"]
    updated["planned_repair_inputs"]["scene_category"] = source["scene_category"]
    updated["planned_repair_inputs"]["sample_type"] = source["sample_type"]
    updated["planned_repair_inputs"]["sequence_id"] = source["sequence_id"]
    updated["planned_repair_inputs"]["library_status"] = source["library_status"]
    updated["planned_repair_inputs"]["reserve_reason"] = source["reserve_reason"]
    updated["planned_repair_inputs"]["usable_for_fewshot"] = source["usable_for_fewshot"]
    return updated


def build_workbook(library_payload: dict):
    rows = [record["source_fields"] for record in library_payload["records"]]
    df = pd.DataFrame(rows, columns=FIELD_ORDER)
    summary = [
        ["版本", "V120 Safe Plus Absorb V148"],
        ["字段数", str(len(FIELD_ORDER))],
        ["总行数", str(len(df))],
        ["正式口径", "23字段 / 152行 / 前120稳定 / 新32 reserve+No"],
        ["增强范围", "CNWARSEQ01-04 + SLGSEQ01-04"],
        ["未吸收", "director_style_ref / official+Yes / GAMESEQ / schema变更 / runtime接入"],
    ]

    OUTPUT_XLSX.parent.mkdir(parents=True, exist_ok=True)
    with pd.ExcelWriter(OUTPUT_XLSX, engine="openpyxl") as writer:
        pd.DataFrame(summary, columns=["主控摘要", "值"]).to_excel(writer, sheet_name="主控摘要", index=False)
        df.to_excel(writer, sheet_name="V120工作簿", index=False)


def recalc_coverage_summary(records: list[dict]) -> dict:
    rows = [record["source_fields"] for record in records]
    summary = {
        "library_status": dict(Counter(row["library_status"] for row in rows)),
        "sample_type": dict(Counter(row["sample_type"] for row in rows)),
        "quality_grade": dict(Counter(row["quality_grade"] for row in rows)),
        "usable_for_fewshot": dict(Counter(row["usable_for_fewshot"] for row in rows)),
        "style_cluster": dict(Counter(row["style_cluster"] for row in rows)),
        "scene_category": dict(Counter(row["scene_category"] for row in rows)),
        "sequence_groups": dict(Counter(row["sequence_id"] for row in rows if row["sequence_id"])),
        "surface_completeness": {},
    }
    for field in CORE_FIELDS:
        summary["surface_completeness"][field] = sum(1 for row in rows if row[field].strip())
    return summary


def validate_state(before_payload: dict, after_payload: dict):
    before_by_sample = {record["sample_id"]: record for record in before_payload["records"]}
    after_by_sample = {record["sample_id"]: record for record in after_payload["records"]}
    new_rows = [record for record in after_payload["records"] if record["sample_id"].startswith("GS120-CAND-")]
    legacy_rows = [record for record in after_payload["records"] if not record["sample_id"].startswith("GS120-CAND-")]

    legacy_unchanged = True
    legacy_changed = []
    for record in legacy_rows:
        before_source = before_by_sample[record["sample_id"]]["source_fields"]
        if before_source != record["source_fields"]:
            legacy_unchanged = False
            legacy_changed.append(record["sample_id"])

    placeholder_hits = []
    banned_prompt_hits = []
    for record in new_rows:
        combined = " ".join(record["source_fields"][field] for field in FIELD_ORDER)
        for token in PLACEHOLDER_TOKENS:
            if token.lower() in combined.lower():
                placeholder_hits.append((record["sample_id"], token))
                break
        prompt_body = record["source_fields"]["prompt_body"]
        hits = [term for term in PROMPT_BANNED if term in prompt_body]
        if hits:
            banned_prompt_hits.append((record["sample_id"], hits))

    return {
        "field_count": len(after_payload["source_field_order"]),
        "row_count": len(after_payload["records"]),
        "director_style_ref_present": "director_style_ref" in after_payload["source_field_order"],
        "legacy_unchanged": legacy_unchanged,
        "legacy_changed": legacy_changed,
        "new_rows_reserve_no": all(
            record["source_fields"]["library_status"] == "reserve"
            and record["source_fields"]["usable_for_fewshot"] == "No"
            for record in new_rows
        ),
        "placeholder_hits": placeholder_hits,
        "banned_prompt_hits": banned_prompt_hits,
        "sequence_counts": dict(Counter(record["source_fields"]["sequence_id"] for record in new_rows)),
    }


def build_report(docx_paragraphs: list[str], v148_frame: pd.DataFrame, validation: dict, workbook_sha: str | None):
    relevant_v148 = v148_frame[v148_frame["sequence_id"].str.startswith(("CNWAR", "SLG"))].copy()
    lines = [
        "# V120 V148 Safe-Plus Absorption Report 2026-04-24",
        "",
        "## Goal",
        "",
        "- absorb V148 camera structure, density, negative-list, and abstract meeting logic into the current 32 reserve candidates only",
        "- keep Hope public contract at `23 fields / 152 rows`",
        "- keep the original 120 rows stable",
        "- do not import `director_style_ref`, real director names, official/Yes promotions, runtime hookups, or schema changes",
        "",
        "## Inputs",
        "",
        f"- plan docx: `{INPUT_DOCX}`",
        f"- style workbook: `{INPUT_XLSX}`",
        f"- current seed baseline: `{CURRENT_LIBRARY}` at commit base `e3140d7`",
        "",
        "## V148 Capability Intake",
        "",
        "- absorbed lens structures: 国战军阵, 武将高光, 朝堂权谋, 沙盘视口, 城建演进, 战报 UI, 多军团攻城",
        "- absorbed density controls: 秒点动作, 构图比例, 风向, 人数规模, 兵器轨迹, 光源方向, 静默点, 声音同步",
        "- absorbed negative controls: 旗帜乱码, 朝代错乱, 盔甲塑料感, 地图现代感, 行军线漂移, UI 乱码, 品牌/IP 泄漏",
        "- absorbed meeting logic only as abstraction inside `teaching_note` and this report; no real names were written into any seed prompt",
        "",
        "## Explicitly Not Absorbed",
        "",
        "- no `director_style_ref` field in any output seed or final 23-field workbook",
        "- no V148 `official / Yes` status promotion into Hope",
        "- no GAMESEQ rows, no row-count change beyond the existing 152",
        "- no schema change to 24 fields, no V152 work, no runtime / Qwen / Doubao / Seedance / reference_control_core opening",
        "",
        "## Sequence Mapping",
        "",
        "- `CNWARSEQ01-03`: absorbed directly from matching V148 war / general / court sequences",
        "- `CNWARSEQ04`: absorbed from V148 siege / multi-legion siege capability while staying inside current Hope war sequence semantics",
        "- `SLGSEQ01`: absorbed from V148 strategic sandbox viewport sequence",
        "- `SLGSEQ02`: absorbed from V148 trajectory + siege routing capability as multi-route / alliance / retreat grammar",
        "- `SLGSEQ03`: absorbed from V148 city-build time-lapse capability",
        "- `SLGSEQ04`: absorbed as battle-report UI closure, using V148 siege-closing information rhythm without importing external names or 24th-field metadata",
        "",
        "## Dry-Run Result",
        "",
        f"- field count remains `23`: `{validation['field_count'] == 23}`",
        f"- row count remains `152`: `{validation['row_count'] == 152}`",
        f"- `director_style_ref` absent: `{not validation['director_style_ref_present']}`",
        f"- original 120 rows unchanged: `{validation['legacy_unchanged']}`",
        f"- new 32 remain `reserve / No`: `{validation['new_rows_reserve_no']}`",
        f"- placeholder residue count: `{len(validation['placeholder_hits'])}`",
        f"- banned prompt hit count: `{len(validation['banned_prompt_hits'])}`",
        f"- eight sequences remain complete: `{json.dumps(validation['sequence_counts'], ensure_ascii=False)}`",
        "",
        "## Output Workbook",
        "",
        f"- output path: `{OUTPUT_XLSX}`",
        f"- workbook sha256: `{workbook_sha or 'pending'}`",
        "- sheet layout: `主控摘要` + `V120工作簿`",
        "- schema: `23 fields`, no `director_style_ref` column",
        "",
        "## V148 Document Notes",
        "",
    ]
    for paragraph in docx_paragraphs[:16]:
        lines.append(f"- {paragraph}")

    lines.extend(
        [
            "",
            "## V148 Relevant Rows",
            "",
        ]
    )
    for sequence_id in sorted(relevant_v148["sequence_id"].unique().tolist()):
        titles = relevant_v148[relevant_v148["sequence_id"] == sequence_id]["sample_title"].tolist()
        lines.append(f"- `{sequence_id}`: " + " | ".join(titles))

    REPORT_PATH.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main():
    parser = argparse.ArgumentParser(description="Absorb V148 safe-plus capability into the existing 32 reserve candidates without schema changes.")
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    library = load_json(CURRENT_LIBRARY)
    failure = load_json(CURRENT_FAILURE)
    repair = load_json(CURRENT_REPAIR)
    coverage = load_json(CURRENT_COVERAGE)
    source_register = load_json(CURRENT_SOURCE_REGISTER)
    manifest = load_json(CURRENT_MANIFEST)
    before_library = copy.deepcopy(library)

    docx_paragraphs = [paragraph.text.strip() for paragraph in Document(INPUT_DOCX).paragraphs if paragraph.text.strip()]
    v148_frame = pd.read_excel(INPUT_XLSX, sheet_name="V148融合字段库24列", dtype=str).fillna("")

    library_by_sample = {record["sample_id"]: record for record in library["records"]}
    failure_by_sample = {record["sample_id"]: record for record in failure["records"]}
    repair_by_sample = {record["sample_id"]: record for record in repair["records"]}

    for sample_id in sorted(sample_id for sample_id in library_by_sample if sample_id.startswith("GS120-CAND-")):
        enhanced = enhance_candidate_row(library_by_sample[sample_id])
        library_by_sample[sample_id] = enhanced
        failure_by_sample[sample_id] = update_failure_record(failure_by_sample[sample_id], enhanced)
        repair_by_sample[sample_id] = update_repair_record(repair_by_sample[sample_id], enhanced)

    library["generated_at"] = DATE
    library["records"] = [library_by_sample[record["sample_id"]] for record in library["records"]]

    failure["generated_at"] = DATE
    failure["records"] = [failure_by_sample[record["sample_id"]] for record in failure["records"]]

    repair["generated_at"] = DATE
    repair["records"] = [repair_by_sample[record["sample_id"]] for record in repair["records"]]

    build_workbook(library)
    workbook_sha = sha256_file(OUTPUT_XLSX)

    library["source"]["primary_workbook"] = OUTPUT_XLSX.as_posix()
    library["source"]["primary_workbook_sha256"] = workbook_sha
    for record in library["records"]:
        if record["sample_id"].startswith("GS120-CAND-"):
            record["provenance"]["source_workbook"] = OUTPUT_XLSX.as_posix()
            record["provenance"]["source_workbook_sha256"] = workbook_sha

    failure["source_context"]["primary_source_id"] = SAFE_PLUS_SOURCE_ID
    repair["source_context"]["primary_source_id"] = SAFE_PLUS_SOURCE_ID
    coverage["generated_at"] = DATE
    coverage["source_context"]["primary_source_id"] = SAFE_PLUS_SOURCE_ID

    summary_after = recalc_coverage_summary(library["records"])
    sample_ids = [record["sample_id"] for record in library["records"]]
    for rule in coverage["records"]:
        rule["row_count"] = len(library["records"])
        rule["source_sample_ids"] = list(sample_ids)
        rule["coverage_summary"] = copy.deepcopy(summary_after)
        rule["coverage_summary"]["surface_completeness"] = {rule["core"]: summary_after["surface_completeness"][rule["core"]]}

    source_register["updated_at"] = DATE
    source_register["sources"].append(
        {
            "source_id": V148_DOCX_SOURCE_ID,
            "source_type": "immutable_raw_docx",
            "label": "V148 safe-plus plan docx",
            "path": INPUT_DOCX.as_posix(),
            "sha256": sha256_file(INPUT_DOCX),
            "applies_to": ["golden_sample_library", "golden_sample_failure_mapping", "golden_sample_repair_mapping"],
            "notes": "Read-only capability plan. Used only for safe-plus ability absorption; director names remain abstract and never enter prompt_body.",
        }
    )
    source_register["sources"].append(
        {
            "source_id": V148_XLSX_SOURCE_ID,
            "source_type": "immutable_raw_xlsx",
            "label": "V148 24-field director-style workbook",
            "path": INPUT_XLSX.as_posix(),
            "sha256": sha256_file(INPUT_XLSX),
            "applies_to": ["golden_sample_library", "golden_sample_failure_mapping", "golden_sample_repair_mapping"],
            "notes": "Capability donor only. `director_style_ref` is not imported into Hope seed fields.",
        }
    )
    source_register["sources"].append(
        {
            "source_id": SAFE_PLUS_SOURCE_ID,
            "source_type": "normalized_safe_plus_xlsx",
            "label": "V120 safe-plus final 23-field workbook",
            "path": OUTPUT_XLSX.as_posix(),
            "sha256": workbook_sha,
            "applies_to": [
                "golden_sample_library",
                "golden_sample_field_coverage_rules",
                "golden_sample_failure_mapping",
                "golden_sample_repair_mapping",
            ],
            "notes": "Canonical 23-field workbook after safe-plus absorption of V148 capability into the current 32 reserve candidates.",
        }
    )
    source_register["provenance_entries"].append(
        {
            "provenance_id": SAFE_PLUS_PROVENANCE_ID,
            "source_ids": [
                SAFE_PLUS_SOURCE_ID,
                V148_DOCX_SOURCE_ID,
                V148_XLSX_SOURCE_ID,
                "seedance2_v120_cnwar_slg_candidate_pool_xlsx_2026_04_24",
                "hope_seedance2_v120_full_kb_ingest_review_2026_04_23",
                "hope_v120_v3_kb_full_ingest_dispatch_2026_04_23",
            ],
            "generated_files": [
                "seed/v0.2/golden_sample_library.json",
                "seed/v0.2/golden_sample_field_coverage_rules.json",
                "seed/v0.2/golden_sample_failure_mapping.json",
                "seed/v0.2/golden_sample_repair_mapping.json",
                "seed/v0.2/source_register.json",
                "seed/v0.2/manifest.json",
                "docs/v120-v148-safe-plus-absorption-report-2026-04-24.md",
            ],
            "preservation_contract": {
                "schema_remains_23_fields": True,
                "row_count_remains_152": True,
                "original_120_rows_unchanged": True,
                "new_32_rows_stay_reserve_no": True,
                "director_style_ref_imported": False,
                "official_yes_promotion_opened": False,
                "runtime_opened": False,
                "qwen_doubao_seedance_opened": False,
                "reference_control_core_created": False,
            },
        }
    )

    manifest["imported_at"] = DATE
    manifest["record_counts"]["golden_sample_library"] = 152
    manifest["record_counts"]["golden_sample_field_coverage_rules"] = 5
    manifest["record_counts"]["golden_sample_failure_mapping"] = 152
    manifest["record_counts"]["golden_sample_repair_mapping"] = 152
    manifest["record_counts"]["golden_sample_sources"] = len(source_register["sources"])
    manifest["record_counts"]["golden_sample_provenance_entries"] = len(source_register["provenance_entries"])

    validation = validate_state(before_library, library)
    build_report(docx_paragraphs, v148_frame, validation, workbook_sha)

    if args.write:
        write_json(CURRENT_LIBRARY, library)
        write_json(CURRENT_FAILURE, failure)
        write_json(CURRENT_REPAIR, repair)
        write_json(CURRENT_COVERAGE, coverage)
        write_json(CURRENT_SOURCE_REGISTER, source_register)
        manifest["content_hash"] = "bundle-sha256:pending"
        write_json(CURRENT_MANIFEST, manifest)
        manifest["content_hash"] = f"bundle-sha256:{canonical_bundle_hash(repo_root, manifest['bundle_order'])}"
        write_json(CURRENT_MANIFEST, manifest)
    else:
        manifest["content_hash"] = manifest["content_hash"]

    print(f"Report: {REPORT_PATH}")
    print(f"Workbook: {OUTPUT_XLSX}")
    print(f"Field count: {validation['field_count']}")
    print(f"Row count: {validation['row_count']}")
    print(f"Legacy unchanged: {validation['legacy_unchanged']}")
    print(f"New rows reserve/no: {validation['new_rows_reserve_no']}")
    print(f"Placeholder hits: {len(validation['placeholder_hits'])}")
    print(f"Banned prompt hits: {len(validation['banned_prompt_hits'])}")
    print(f"Sequence counts: {json.dumps(validation['sequence_counts'], ensure_ascii=False)}")


if __name__ == "__main__":
    main()
