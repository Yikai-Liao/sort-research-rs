# Tessera 清理：历史筛选账本

基线为 `2ef2415`。本目录保存清理过程的逐轮数据和 manifest。当前最终版本见[上级报告](../README.md)。候选筛选结果用于选择实现。

## 当时的实现取舍

清理删除了实验用 `Policy`、叶子消融入口、环境调参与未采用的实验分支，保留稳定的 MSD、LSD、插入、等值和离群 run 路径。`Digit { shift, width }` 描述处理窗口，递归和共享 block 接口用 `u128` 变化位 mask；`RestoreTile` 用一个状态标识记录集所在缓冲区。digit 分区在 compact 时处理能放进 tile 的桶，等值分区在 compact 后处理两侧。

block 分区在 key panic 后恢复完整记录集，包括没有实现 `Drop` 的类型：这类记录仍可持有独占引用。正常 scatter 只保留输入和输出块游标；发生 panic 时用各桶已写数减去完整输入块的记录数，重建当前块内的恢复偏移。等值分类器读取一次 key，用两个比较计算桶编号；已审查 `Unsigned` 的六种 sealed 基本类型上与原分类等价。`split_first` 用于消费桶边界；稳定逆序的 `reverse` 和离群 run 的 `rotate` 是已有操作。

以下对照的比值均为后一候选 / 前一候选的逐周期中位数之中位数，大于 1 表示候选较慢。下表除 block-history 外均为 5 周期、每周期 3 轮；具体类型、形状和二进制 SHA-256 见各 manifest。原始逐轮数据在同名前缀的 `*-rounds.csv`，汇总 stdout 在本地 `logs/tessera-cleanup/*comparison.log`。这些日志未纳入本目录。

| 对照 | 观察和取舍 | 数据 |
| --- | --- | --- |
| raw → safe 计数内核 | 18 cell 中，u32 random_local 为 1.378，small12 为 1.139，f64_normal 为 1.206；撤回候选，保留 raw 内核。 | [轮次](safe-rounds.csv) · [配置](safe-manifest.json) |
| unrolled → simple scatter | u32 Block random_wide 1.258、zipf13 1.330；Tessera 各周期波动较大。保留展开循环。 | [轮次](loop-rounds.csv) · [配置](loop-manifest.json) |
| outlier on → off | u64 head_high 1.251、tail_low 1.151，两个 cell 五周期均大于 1；保留离群 run 路径。 | [轮次](outlier-rounds.csv) · [配置](outlier-manifest.json) |
| visitor → plain / lean | 对照探索减少 visitor 结构；最终 digit 路径保留 tile 回调，等值路径不挂回调。各周期波动明显。 | [plain 轮次](plain-rounds.csv) · [配置](plain-manifest.json)；[lean 轮次](lean-rounds.csv) · [配置](lean-manifest.json) |
| union → xor / pair → mask | 探索变化位表示；最终使用 mask。u32 Tessera random_sorted99 的 mask/pair 为 1.239，波动较大，最终配对结果见[上级报告](../README.md)。 | [xor 轮次](xor-rounds.csv) · [配置](xor-manifest.json)；[mask 轮次](mask-rounds.csv) · [配置](mask-manifest.json) |
| typed → shift / u32shift | 探索移位操作的表示；当时尝试 u32 shift。局部样本 u32 Block random_wide 的 u32shift/typed 为 1.144；后续确认未稳定重现；最终因公开泛型契约恢复同类型右操作数（见下表）。 | [shift 轮次](shift-rounds.csv) · [配置](shift-manifest.json)；[u32shift 轮次](u32shift-rounds.csv) · [配置](u32shift-manifest.json) |
| 初始 block → 清理中 block | 历史实现与清理中版本的 3 周期、每周期 7 轮对照，用于定位共享 block 的变化；例如 u32 random_wide 1M 为 1.213。最终清理版与 `2ef2415` 的配对见[上级报告](../README.md)。 | [轮次](block-history-rounds.csv) · [配置](block-history-manifest.json) |

各 manifest 的比较方向和执行顺序已按 CSV 的实际 arm 和脚本校正；hashes 是当时的二进制哈希。block-history 的初始源码哈希 `208d8648…` 已与 `git show 429f6f7:src/other/rust_block_lsd/core.rs` 核对一致。`guard-attribution-rounds.csv` 保存另一次归因数据。

### u32 shift 确认

当时的 indexed 候选与 typed-mask 基线以同一 harness 在 CPU 2 对照，每组 5 周期、每周期 7 轮，按 A1/candidate/A2 交替顺序执行；A1 和 A2 使用同一基线二进制。AB 为逐周期 candidate 中位数除以 A1/A2 中位数的平均值，再取五周期中位数；AA 为 A2/A1 同口径中位数。1M u32 结果如下：

| 形状 | Block AB | Block AA | Tessera AB | Tessera AA |
| --- | ---: | ---: | ---: | ---: |
| random_wide | 0.946 | 0.925 | 0.939 | 0.985 |
| random_sorted99 | 0.963 | 0.930 | 1.028 | 1.000 |
| zipf13 | 0.962 | 0.945 | 1.043 | 0.918 |
| sparse8 | 0.941 | 1.025 | 1.031 | 1.154 |

初筛中 Block random_wide 的 14.4% 回退在本次确认中未稳定重现。周期及 A/A 波动较大，这组结果不足以判断小幅差异；当时保留该候选；最终因公开泛型契约恢复同类型右操作数。具体周期范围、二进制和冻结源码哈希见 [汇总](shift-confirm-summary.csv)、[轮次](shift-confirm-rounds.csv)和[配置](shift-confirm-manifest.json)。

### 顺序探测与窗口写法

旧 indexed 候选相对 `2ef2415` 的相同 harness 配对在完成两周期及第三周期部分数据后停止，已由 windows 候选替代；数据及停止状态见 [轮次](indexed-rounds.csv)和[配置](indexed-manifest.json)。其中 record16 all_equal 100k 连续出现 40–58% 的表面回退，随后作 3 周期、每周期 1001 轮的专项对照：indexed 相对 A1/A2 平均基线分别为 1.221、1.126、1.078。整进程的 instructions 基本相同，cycles 高约 3.8–5.8%；PMU 包含克隆和校验。见[逐轮数据](long-scan-rounds.csv)和 [PMU](long-scan-pmu.json)。

新版 `probe_order` 用 `windows(2).step_by(stride)` 表达采样。独立审查核对了第 k 次比较仍为 `(k*stride-1, k*stride)`，`Key` 调用顺序与提前退出一致，未引入新的可达 panic。5 周期、每周期 31 轮的六个局部 cell 使用 baseline、indexed、windows 三个二进制，比较为逐周期中位数之比的中位数：

| 类型 / 形状 / 数量 | windows / indexed | windows / 基线 |
| --- | ---: | ---: |
| record16 all_equal 100k | 1.031 | 1.019 |
| u32 random_sorted99 1M | 0.804 | 1.118 |
| u64 random_sorted99 1M | 0.859 | 0.997 |
| u32 sparse8 100k | 1.011 | 1.130 |
| u32 random_wide 1M | 1.008 | 1.037 |
| u128 random_wide 1M | 1.044 | 1.036 |

采用更短的窗口写法。原始数据和执行顺序见[轮次](windows-rounds.csv)和[配置](windows-manifest.json)。

### eager、cold 与 rank 筛选

前一版 eager 窗口候选将恢复偏移逐记录写入热路径。完整矩阵和补测已归档到 [eager 数据](eager-rounds.csv)、[汇总](eager-summary.csv)、[补测](eager-recheck-summary.csv)及其 manifest；eager 补测相对基线，u32 two_values 在 100k / 1M 为 1.261 / 1.312，p95zero 为 1.199 / 1.187。

cold 候选将偏移重建移到 panic 冷路径；筛选中 Block 两个随机 cell 相对基线为 0.943 / 0.958，等值场景仍回退。借用和引用捕获两种消融未通过筛选，已撤回。见 [cold 汇总](cold-summary.csv)、[轮次](cold-rounds.csv)和[配置](cold-manifest.json)。

rank 在 cold 基础上改写等值分类。12 cell、5 周期、每周期 31 轮的局部对照中，u32 two_values 100k / 1M 的 rank/cold 为 0.746 / 0.745，rank/基线为 0.980 / 1.083；u32 p95zero 对基线为 0.991 / 0.990，zipf13 1M 为 1.019，u128 two_values 100k 为 0.938。Block u32 random 1M、u64 random 1M、u32 ascending 1M 的 rank/基线分别为 1.013、0.962、1.035。u32 sparse8 100k 为 1.112，周期波动较大；完整矩阵和 A/A 用于进一步判断。见 [rank 汇总](rank-summary.csv)、[轮次](rank-rounds.csv)和[配置](rank-manifest.json)。

501 轮 two_values 的 perf 采样中 scatter 约占 56% / 60%；cold 的整进程 instructions 较少，cycles 较多。对实际 ELF 的审查发现 cold 的低字节 `CL` 前值与上一条记录的 `bucket.next` 有关联；rank 生成全宽 `xor r14d` 再 `seta`。LLVM-MCA 模型预测方向与实测不同，尚不能将假依赖确认为根因。相关原始文件位于本地 `logs/tessera-cleanup/two-{baseline,cold}.{stat,data}`。

### direction 与顺序确认

当时的 `probe_order` 保留标准库 `windows` 采样；恢复手写采样的 direct 候选未解决降序回退，u32 descending 1M 的 direct/基线为 1.136，已撤回。见 [direct 汇总](direct-summary.csv)、[轮次](direct-rounds.csv)和[配置](direct-manifest.json)。

`confirm_order` 先寻找首个与首键不同的键来确定方向，再用 `once(key).chain(keys)` 的 `is_sorted` 或 `is_sorted_by` 确认该方向，移除了每条记录同时更新两个方向状态。静态审查确认 sealed 基本键类型的调用顺序、次数和提前退出位置与原版相同。采样显示降序差异主要涉及等值段修复；根因未确证。

以相同新 harness 比较 direction、仅更换 harness 的候选及基线，5 周期、每周期 31 轮、10 cell：u32 descending 100k / 1M 的 direction/harness 分别为 0.778 / 0.780，direction/基线为 0.904 / 0.885；u64 all_equal 100k / 1M 的 direction/基线为 0.976 / 1.017，u32 two_values 1M 为 0.971、p95zero 100k 为 0.976；u64 / u128 random 1M 为 0.976 / 0.988，Block u32 / u64 random 1M 为 0.977 / 0.995。周期波动见 [direction 汇总](direction-summary.csv)、[轮次](direction-rounds.csv)和[配置](direction-manifest.json)。baseline 和 candidate 的 benchmark harness 均使用 `expected.chunk_by` 统计，harness SHA-256 为 `c49aec01d8d3c08591d11ea81e0ffce668ad085ad15b0f04be7ac97230467ea0`。最终完整矩阵和 A/A 见[上级报告](../README.md)。

## 预算插入筛选与接口恢复

统一预算插入核后，依次筛选了内联边界、缓存当前键、旋转移动以及有序段扫描。缓存和旋转方案的近序数据出现回退，例如 u32 random_sorted99 100k 的 cached/unified 为 1.336、rotate/cached 为 4.398；对应 [cached](cached-summary.csv) 和 [rotate](rotate-summary.csv)。这些是各自独立的三周期筛选。最终选择窗口扫描有序段、直接交换已比较的逆序对，再反向插入；[run-scan](run-scan-summary.csv) 的三周期局部筛选供判断方向，完整表现以后续冻结版配对为准。[inline](inline-summary.csv)、[unified](unified-summary.csv)及其他同名 manifest 保留其余候选和轮次。

`source-guard` 候选用唯一 source 指针记录完整记录集，其完整矩阵、补测和验证保存在 [完整汇总](source-guard-full-summary.csv)、[补测汇总](source-guard-recheck-summary.csv)和[验证记录](source-guard-validation.json)。这是 `dc331` 冻结版的历史证据。

后续恢复基线 `Unsigned` 的 `Shl<Self>` / `Shr<Self>` 公开约束，内部也使用同类型移位量。`Unsigned` 虽为 sealed trait，旧泛型代码仍依赖其 supertrait 的移位约束；新增的泛型 consumer 检查覆盖显式与推导的右操作数。

## 采样与索引热点审计

odd 步长、等价 range、unchecked 采样与共享 scatter 的实验及模型见[热点审计](index-audit.md)。采样和 scatter 候选已撤回；后续 Probe/try_fold 已冻结，性能与安全验证均已完成。

正式冻结版的完整性能与随机顺序专项已记录在[最终报告](../README.md#最终性能对照)；release、ASan 和 Miri 均已通过。

前一版 typed 冻结的证据： [完整性能汇总](typed-final-summary.csv)、[补测汇总](typed-recheck-summary.csv)及[验证记录](typed-validation.json)。
