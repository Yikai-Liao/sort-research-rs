# 采样与索引热点审计

前一版 typed 的 u64 sparse8 100k 五周期补测 AB 为 1.062956、AA 为 1.016；后续七周期、每 arm 1001 轮的专项复查 AB 为 1.01491、AA 为 0.95565。6.3% 的幅度未重现，根因未确证。数据见[五周期补测](typed-recheck-summary.csv)和[专项复查](sparse-confirm-summary.csv)。

奇数步长将周期 8 的混叠转移到周期 5 或 25，用户拒绝该候选。等价 range 保持原采样点和 Key 调用顺序，生成代码仍有部分边界分支。probe 的 unchecked 版本消除了相应分支，但改变了 ELF 布局；15 个随机顺序 block 的同 ELF 双次 A/A 对照中，sparse8 100k 的 unchecked/checked 为 1.0007。descending 1M 为 1.1072，15/15 block 大于 1；整进程 cycles、task-clock、instructions 比值分别为 1.0611、1.0865、0.9999。该差异属于整体 ELF，具体原因未定。见 [odd](odd-stride-summary.csv)、[range](range-probe-summary.csv)、[probe noise](probe-noise-summary.csv)及各自 manifest。

采样处三个、共享 scatter 四个 unchecked 索引各自做了边界证明，净代码行数均不增加；20 cell、3 周期的独立筛选波动较大，未支持晋级。这两组新增 unsafe 已撤回。当时 Probe 单独继续评估；后续 Probe/try_fold 已冻结，性能与安全验证均已完成。边界隔离实验见下文。见[汇总](index-audit-summary.csv)、[轮次](index-audit-rounds.csv)和[配置](index-audit-manifest.json)。

## 实际 ELF 指令片段模型

LLVM-MCA 19 的 Skylake-AVX512 模型从原 window、checked range 与 unchecked 实际 ELF 提取十一段代码，运行 1000 次；下表为每次迭代的模型周期，BRT 为 block reciprocal throughput。完整地址、哈希、输入与命令记录在 [analysis.json](index-mca/analysis.json)。

| 路径 | checked | unchecked | 单位 |
| --- | ---: | ---: | --- |
| probe | 4.288（17 指令、19 uops） | 4.011（15 指令、17 uops） | 每采样对；原 window 为 4.510（18 指令、20 uops） |
| confirm | 1.672 | 1.631 | 每记录；两版 BRT 1.2 |
| reverse | 4.011 | 4.011 | 每 8 记录 |
| singleton 等值段修复 | 5.007 | 5.007 | 每记录 |
| 正常 scatter | 13.023 | 13.023 | 每 4 记录；两版 BRT 11.3 |

scatter 片段少了 spill，但模型吞吐不变。模型不包括真实 CPU 的缓存和代码布局，不能据此解释整体 ELF 的实测差异。

## Probe 隔离与后续候选

以 typed 原始 ELF `fbe2288f…` 为分母，unchecked Probe 的 u64 descending 1M 比值为 1.1305；对 Probe 加 `#[inline(never)]` 隔离函数边界后为 1.0942，两者五个 block 均慢。见[隔离汇总](probe-isolation-summary.csv)和[配置](probe-isolation-manifest.json)。

`chunk_by_mut` 等值段修复候选已撤回：chunk-window 的 record16 descending 1M 为 1.0616，五个 block 均慢；chunk-probe 的 u64 descending 1M 为 1.0905。另一次强制 `confirm_order` 内联试验见 [confirm-inline](confirm-inline-summary.csv)；[chunk 数据](chunk-reverse-summary.csv)保留两种变体。

新 Probe/try_fold 候选维持采样点及 Key 次序，去掉顺序确认中的 `Chain` 等中间状态。6 cell、5 个随机 block 的局部对照中 u64 descending 1M 相对 typed 为 0.9939，相对 fold-window 为 0.8819；u32 descending 1M 分别为 0.8323 / 0.8021。该候选已冻结，完整性能矩阵、release、ASan 与 Miri 已完成；见[筛选汇总](fold-probe-compare-summary.csv)、[轮次](fold-probe-compare-rounds.csv)和[配置](fold-probe-compare-manifest.json)。

## 正式冻结版的指令对应关系

Probe/try_fold 候选在正式冻结前只修改了两处注释。独立 ELF 审计确认最终 `c4196d51…` 与前一 ELF `764f6b42…` 的整个 `.text` 完全一致，Probe 和 confirm 的实际机器指令也与 MCA 输入逐字节一致；见[验证记录](final-probe-mca/analysis.json)。

新候选的 Probe 模型为每采样对 15 指令、17 uops、4.011 周期、BRT 3.0；原 window 和 fold-window 均为 18 指令、20 uops、4.510 周期、BRT 3.5。`try_fold` 消除了 `Chain::is_sorted_by` helper 调用，confirm 扫描进入 root 内联，但模型未显示吞吐收益：fold 两版均为 2.003 周期/记录，原 window 为 1.672、旧 unchecked 为 1.631，均为 7 指令、7 uops、BRT 1.2。模型不能证明内联边界变化是整体运行时间差异的全部原因；[模型输入与结果](fold-probe-mca/analysis.json)供复核。

正式冻结版的长复查与 15 block 随机专项结果见[最终报告](../README.md#最终性能对照)和[随机汇总](sparse-final-randomized-summary.csv)。长复查 AB 为 1.05008，随机专项为 1.01520；根因未确证。
