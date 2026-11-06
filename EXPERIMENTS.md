# Tessera：实现与实验记录

当前实现是 `src/other/rust_tessera_radix/engine.rs`；分支 `exp/radix-opt`。
本文件保留设计、历史判决和复现方法。最新清理的验证与原始轮次在
[results/tessera/cleanup](results/tessera/cleanup/README.md)，采样和索引审计见
[热点审计](results/tessera/cleanup/archive/index-audit.md)；可 Copy／不可 Copy 记录与
不确定 key 的安全修复及性能验收在
[results/tessera/safety-20261105](results/tessera/safety-20261105/README.md)。
Tessera 迁出为独立仓库 [tessera-sort](https://github.com/Yikai-Liao/tessera-sort)，迁移计划见
[results/tessera/standalone-plan.md](results/tessera/standalone-plan.md)。

## A. 当前实现

Probe/try_fold 已冻结，完整性能矩阵、复查和安全验证已完成。

1. 先探测顺序：升序返回；降序反转后修复等键段；其余在一个 tile 的窗口上试算插入。
   窗口覆盖整个数组时直接在记录上花同样的交换预算（不再复制键副本），否则仍在键副本上
   估计再按交换率给整数组预算，失败后继续 radix。预算插入核共用于试算和记录排序：
   先以窗口扫描有序段，对已比较的首个逆序对直接交换，再沿可变前缀反向迭代插入。
2. 大节点用稳定 block 分区：变化位跨度不超过 8 bit 时一趟完成；其余按最高变化位
   取 digit。digit 从节点大小选 3–8 bit，采样不分散时每次加宽 2 bit，最多 12 bit，
   且不越过最高变化位。
3. 至少四个 tile 的节点还检查两种形状：样本排序后用 `chunk_by` 选择最大等值组，占比至少 1/4 的键用 `<、=、>` 分区；
   最高变化位分离的一端短 run 用旋转和插入排序完成。
4. tile 默认 64 KiB，最多 65535 条记录。≤48 条用插入排序；其余在 tile 内 MSD 分区
   与 LSD counting passes 之间选择。LSD 的趟数由变化位跨度与表宽上限决定，再把同样的位
   均衡摊到各趟（连续变化位时不再先填满再收尾）；MSD 一次分桶用自己采样能负担的最宽
   窗口，与该 tile 的 LSD 计划无关，并记一层分桶 + 插入收尾约等于两趟。
   所有 tile 共用 scratch。
5. block 引擎直接给出桶边界，子节点扫描变化位时提前停止。digit 分区中，能放入 tile
   的子桶在 compact 时立即排序；较大的子桶在 compact 后递归。等值分区在 compact 后处理两侧。
6. 宽 key 的变化位全落在同一 64-bit 半区时，根节点借用 `KeyHalf` 视图，保留记录布局。
   递归用 `u128` 变化位 mask；`Digit { shift, width }` 描述各轮处理的位窗口，常量位不参与规划。

算法保持稳定排序、key panic 后恢复原记录排列和 O(√n) 辅助空间。分区的 scratch 随
block 大小增长，tile 缓冲有固定上界。公开适配器提供 `sort` / `sort_keyed` / `SortImpl`；
`sort_by` 按原约定拒绝。测试通过私有 tile 参数覆盖边界。

实验用 `Policy`、叶子消融入口、宽度／融合／block-size 环境开关、tile／插入阈值变体和
两个临时 example 已删除。有效的 MSD、LSD、插入和等值分区路径继续保留。

上一轮修复了 block 分区的 panic 恢复：panic 时根据已写桶记录数和已消费输入块重建块内偏移，未物化时恢复完整记录排列。

本轮修复可 Copy 与不可 Copy 记录在不确定 key 下的内存安全，不实现 Drop 的记录可能持有独占引用，因此与非 Copy 走同一保护。私有 `RecordType` 只按真实 `Copy` 能力分派，不按 `needs_drop` 推断，也不维护可信类型名单。非 Copy 在 histogram 缓存 u16 分类，scatter 不再调用 key；Copy 的 `scratch` 前缀在一次成功趟后保持有效，首趟借分类缓存写满，之后的趟与 tile 直接复用，记录搬运遍数与基线相同。tile 的 `RestoreTile` 用唯一 `source` 指针标识完整记录集，必要时将其恢复到 tile。公开 `Unsigned` 保留同类型移位右操作数约束，内部也使用键类型的移位量。标准库的 `split_first` 用于遍历桶边界；稳定逆序和离群 run 的 `reverse`、`rotate` 是已有算法操作。

### 阅读历史数字

下面 §1–§88 是各实验当时的结论，包含其后被修正或撤回的结果；它们不是当前版本的性能表。
完整旧报告、脑暴计划与 backlog 可用 `git show 2ef2415:CEILING.md`、
`git show 2ef2415:PLAN.md`、`git show 2ef2415:BACKLOG.md` 查阅。更早的流水账在
`git show a80b566:EXPERIMENTS.md`。原始历史轮次迁到
[history.csv](results/tessera/history.csv)，内容未改；只覆盖文件中列出的实验，
后期只有表内摘要的实验没有可恢复的逐轮日志。

机器为 Xeon Gold 6140、6 vCPU 的共享 KVM。历史记录注明 L1d 32 KiB / L2 1 MiB /
每实例 L3 24.8 MiB，nightly 1.101.0。每行数字的类型、形状、规模和参照以该行说明为准。

## B. 判决账本

状态：**保留** 进了代码；**否决** 实现并测过，被数据拒绝；**回滚** 先上线后被推翻；**修正** 修了 bug 或纠正了
先前结论；**清理** 删代码/合并，行为不变；**度量** 测量/诊断，不改代码。

### B.1 设计成形期（§1–§24）

| § | 主题 | 状态 | 关键数字 / 理由 |
| --- | --- | --- | --- |
| 1 | block LSD 的 digit 宽度 | 否决 | 1M u64：r8 43.7 → r12 72.8 ms。宽 digit 的游标表出 L1。split 也因此停在 8 bit |
| 2 | 时间去哪了 | 度量 | tile 内一遍 ≈2.6 ns/记录；split 层 8.9 ns（scatter 5.4 + 物化 ~2 + 边界 ~1.5）。成本模型据此标定 |
| 3 | tile 预算 | 保留 64 KiB | 16–24 KiB 要多一层 split（1.8x 差）；48–64 KiB 平坦最优；≥128 KiB 在 1M 差 3–10% |
| 4 | 把下一遍直方图融进 scatter | 否决 | u32 1M 16.6 → 20.9。scatter 是 store 口瓶颈（llvm-mca：port 4 满载）|
| 5 | 小 tile：插入(≤48) + 归并(≤384) | 保留（归并 §25 删） | 100k u64 25.7 → 21.8 |
| 6 | 32 位游标 + 裸指针 | 保留 | u64 1M 27.5 → 25.4 |
| 7 | 叶子 digit 8 → 11 bit | 保留 | u128 1M 95 → 77.5；12 bit 差 1–2% 回退 |
| 8 | 一次 variation 扫描 | 保留 | 小 n 对基线的 3–8% 回退消失 |
| 9 | 有序探测 | 保留 | u64 升序 46.3 → 2.9；探测不并入 variation 扫描（会让聚簇输入慢 5%，破坏向量化）|
| 10 | split 只在 digit 能分开时才有用 | 保留 | 采样 top digit 占用；u128 zipf 10M 37.2 → 24.2。**注**：此处的带宽模型后被 §32/41 整体替换 |
| 11 | u16 游标（tile ≤ 65535 条） | 保留 | 宽 key 几个 %，叶子栈少 2 KiB |
| 12 / 17 | 测量卫生 | 方法 | 见 §C。一次是单个异常轮次污染 min；一次是后台遗留的 criterion 跑了 46 分钟 |
| 13 | 窗口按 varying 位打包 | 保留 | u128 8 narrow bands 37.0 → 31.3；稠密随机不变 |
| 14 | 更宽的 split fan-out（9–11 bit） | 否决 | u64 10M 35.4 → 37.9 → 41.4。2048 路流的页/TLB 压力大于 L1 常驻叶子的收益 |
| 15 | 流式回退函数 `#[cold]` | 保留，§31 又撤 | u32 1M 0.92x → 1.12x；10M 时每个子节点都走它，cold 标注变成谎言，§31 去掉 |
| 16 | 叶子缓冲大于 tile | 回滚 | 10M u64 33.1 → 32.7（噪声内），且 64–512 KiB 数组上缓冲变 O(n) |
| 18 | 预取 scatter 目标行 | 否决 | 慢 25–36%（u128 1M 76.7 → 103.3）。同样是 store 口瓶颈，要少遍数，不是更好的一遍 |
| 19 | merge 叶子 key panic 会复制+丢失记录 | 修正 | `ptr::read` → bitwise copy + Drop 恢复；41 个 panic 点的扫描测试，两半缺一不可 |
| 20 | u128 的 pattern 全是 32 位 key 装 128 位容器 | 修正 | 重做 type-space pattern。`two_near`（1 个 varying bit）u128 11.7 vs ipnsort 7.2：证明"无变化"的全扫描成本 |
| 21 | 叶子宽度逐 tile 不同，路由模型不得跟随 | 保留（模型后废） | u32 10M 因扫宽度而 13.9 → 18.6（路由被带偏）。u128 最优 14 bit、u64 最差 14 bit（32 KiB 游标表 = 整个 L1）|
| 22 | 四份独立 review | 度量 | 发现：叶子遍数随 key 宽而非 tile 记录数 → §23；≤8 MiB 的 cell 是热缓存；record 类型 payload 全零看不出稳定性回归（见 §C）|
| 23 | tile 内先做一层 MSD | 保留 | u128 1M 76.8 → 31.9（2.41x），u64 1M 25.4 → 20.2（1.26x）。窗口须先采样（否则 random_local 慢 3x）；表不清零（省 14%）|
| 24 | 路由模型给叶子标了过时的价 | 修正 | 10M u64 34.2 → 27.6；u128 10M 91.9 → 49.1。u32 10M 仍是边界 case |

### B.2 清理与简化期（§25–§34）

| § | 主题 | 状态 | 关键数字 / 理由 |
| --- | --- | --- | --- |
| 25 | 删 merge 叶子；5 个入口并成 `Policy`；`descend` 与 tile MSD 共享 `for_each_bucket` | 清理 | merge 在一个 cell 赚 6%、另一个亏 15%，不是路径。**但**共享"边界来源"不行：tile MSD 比较 digit 来找边界慢 48%（19.4 → 28.8），保留游标 |
| 26 | "始终 split" 取代成本模型 | 否决 | 模型在主流 cell 持平或更好；"2x 更好"的 cell 是噪声（静音机器 5 轮后 9.1 vs 9.2）。**教训进 §C** |
| 27 | level-by-level 路径改用本模块 planner | 保留，§29 删 `StreamMode` | 与块引擎自己的排序持平，于是路径合并 |
| 28 | 一个 planner 同时给路由定价和执行 | 清理 | 常见形状路由逐节点相同（6/12/18 stream 节点前后一致）；clustered 形状 u64 zipf −8.5% |
| 29 | `tile_cost` 合并进路由 | 回滚 | 路由只知道"造出该子节点的 digit"的占比，远比子节点自己的 top 窗口粗 → 312 KiB 子节点都被当成聚簇，u64 10M 258 → 323 ms（+25%）。路由改按"叶子最好情况（一层）"定价 |
| 30 | 整轮简化的总代价 | 度量 | 18 个常见 cell 中 17 个指令数在 ±1.5% 内，最差 u32 two-values +4.2%。墙钟跨 build 无法分辨 ±5%（9 好 9 坏）|
| 31 | 10M 回归检查 | 修正 | 去掉 stream_node 上的 `#[cold]`。`perf stat` 显示 10M 指令数 0.02% 差 → 墙钟差是频率/布局 |
| 32 | 路由模型去量纲 | 保留 | 原模型对任一常数扰动 2x 就改变 40–60/652 条路由。改成纯遍数算术；33 个 cell 指令全在 3.7% 内 |
| 33 | 新 workload：`mixed_runs`（长度 16–2064、方向随机的 run）| 度量 | u64 1M 18.2 vs ipnsort 24.4（1.34x）；u128 持平；这是 merge 多个预排序源的真实形状 |
| 34 | 哪些常量是机器相关的 | 度量 | 只有 `TILE_BYTES = 64 KiB`（是容量不是速率，在 plateau 上）。其余全是流/记录/遍数的计数，移植无需重调 |

### B.3 审查、修正与后续实验（§35–§88）

| § | 主题 | 状态 | 关键数字 / 理由 |
| --- | --- | --- | --- |
| 35 | 对 7da8daa 的外部 review | 修正 | (1) merge 叶子 unwind 洞：随代码消失。(2) 说谎的 `Key` 可使 scatter 越界 → scatter 目的地 clamp。(3) "full-width random" 其实只有 32 bit 熵 → SplitMix64 重测，**u128 1M 变成领先 ipnsort 1.40x** |
| 36 | tile 内原地 MSD 层 | 回滚 | 先宣称 −7%，是单次配对的**周期数**。正式测：u64 1M 18.7 → 25.8，u128 34.3 → 42.6。原地环遍历是随机跳链，scatter 是顺序写。同提交留下的"直接走插入的桶不扫 variation"保留（u128 sparse8 −6.7% 指令）|
| 37 | 叶子 digit 宽度按记录数封顶（+2 bit）| 保留 | record128 1M 290 → 155.6（从落后 1.48x 到领先 1.18x）；8 字节记录不受影响 |
| 38 | Miri 证据范围 | 度量 | 当时的 HEAD **7/7 Miri，216 s**（测试尺寸按 `cfg!(miri)` 缩小；此前的审计 agent 因测试 1M 元素超时 30 分钟）|
| 39 | 叶子消融（flat-only vs shipped）| 度量 | MSD 层不可去：u128 random 无它 81.4 vs 35.7。u128 sparse8 落后 ipnsort 2.9x |
| 40 | 冷/热遍数比 r=3 | 否决 | u128 sparse8 −53%，但随机 cell 指令 +8–14%。被 §41 取代 |
| 41 | 路由改为"还需 ≥2 遍就 split" | 保留 | u128 sparse8 35.7 → 15.0；u64 10M −2.6%。机制上去掉机器事实 |
| 42 | 天花板表 | 度量 | u128 在 pass 下限的 0.3–0.6x；u32 离下限最远（1.0–3.1x，每遍固定开销主导）。历史整表见 `2ef2415:CEILING.md` |
| 43 / 45 | 删 `SPLIT_PASSES`、`flat_hot`、`hot_passes`、`MODEL_LEAF_RADIX` | 清理 | 指令 −0.02%，路由行数 2566 前后相同 |
| 44 | lib 测试目标 3 个提交没编译 | 修正 | "N targets green" 靠数 `test result: ok` 行，看不到编译失败。现按 7 个目标含 lib 分别点名 |
| 46 | split 占 u64 的 29–34%、u128 的 52–55% | 度量 | split 是自身下限的 1.7–2.1x（u64）/ 2.5–2.7x（u128）|
| 47 | RFO 假设 | 度量 | VM 无 LLC 事件，**不可直接验证**。cache-miss 37/记录 → 1M 无缓存常驻可赢。结论后在 §52 收窄为"排除缓存常驻收益，不证明 split 无法靠别的写模式变便宜" |
| 48 | 叶子宽度再扫描 | 否决 | 单次看似 −6%，3 组交错重复符号翻转。无宽度赢 auto 超过 5% |
| 49 | split 在窗口不分散时加宽 2 bit | 保留 | u32 zipf −17.4%、u128 zipf −16.0% 指令；sparse8 +1.6–2.9%；其余 ±0.5% |
| 50 | 把后代聚合折进置换（删 descent 遍） | 否决 | 指令 +7–16%，墙钟 u32 1M 10.7 → 20.3。**决定性先例**：在 scatter 热循环里再加每记录两次 RMW 不值 |
| 51 | 路由改读 varying 跨度，不再调用叶子 planner | 清理 | 4770 个节点 0 个决策差异；指令持平 |
| 52 | 外部 review：两个生成器 bug + skew 家族 | 修正 | `type_top_byte` 在 u32/u64 全是等值键（结论作废）；`sparse8` 只用了高 32 bit（§40 的机制解释错，结果仍在 2% 内）。skew10–99 无悬崖。统计量改为**中位数**，历史 `raw_runs.csv` 存档逐轮数据 |
| 53 | 叶子 f64 成本模型 → 两个计数 | 清理 | 14/15 cell auto = MSD-only；−23 行。flat-only 在 u128 random 上 82.1 vs 31.2 |
| 54 | 单 digit 终点（idea 01）| **作废** | 两个 build 各自生成随机数据，"+14.3% 指令"是数据差。见 §56 |
| 55 | 永远过不了的加宽采样（宽 ≥10 跳过）| 清理 | `OVERRIDE_SEED=1`，路由 trace 8 cell 逐字节相同，指令 ±0.03%。**暴露 §54 的方法错误** |
| 56 | idea 01 带种子重测 | 保留 | trace 10 cell 完全相同，指令 ±0.6%，删 `stream_node`、`sort_with_digits`。墙钟同数据 ±10%（噪声大于效应）|
| 57 | block 大小扫描（0.5x–4x）| 否决 | 指令全程只跨 1.0–2.1%，无台阶。当时保留 `RADIX_BLOCK_SCALE`；本轮清理删除 |
| 58 | 恢复叶子 guard（整数版深度估计）| 修正 | §53 过度简化：u64 random_local 27.10 vs flat 23.46。恢复后 24.30 vs 24.28，指令 −4.2%。**注意**：这是**两组独立配对**，不要合并成一个精确的跨 build 百分比 |
| 59 | 叶子窗口按数据大小定 | 否决 | 指令 −5% ~ −11%，墙钟 +4% ~ +8%（精确版 +43%）：表窄 → 多一层 → 多一遍 tile |
| 60 | rank 压缩叶子 | 否决 | 正确，且在 bands8/zipf/skew99 赢 4–5%；但无门版 random_wide −25%。门后同 cell 两次跑差 10–20 个点（噪声）|
| 61 | N3：下一遍直方图融进平坦 scatter | 否决 | 指令 −1% ~ −7%，墙钟 u32 random **+21.6%**、99% sorted +11%。第二张 16 KiB 表把 31 KiB tile 挤出缓存。**这是第三次"指令降、墙钟升"**（见 §C）|
| 62 | `RADIX_BLOCK_SCALE` 读环境变量改 `OnceLock` | 清理 | 不声称性能。每个 split 节点少一把锁 |
| 63 | N6：tile MSD 交替两个缓冲，省拷回 | 关闭（无覆盖）| 未实现，只做了覆盖测量。每条记录当前拷回 `d+(p mod 2)`（d=tile 内 MSD 层数，p=末端 flat 趟数），交替后最少 `(d+p) mod 2`。临时诊断补丁（未入库）在每个终止区间记 (d,p,len)：u64/u128 random 在 2e4/1e5/1e6/1e7 **100% 记录为 d=1,p=0**，可省 0%；local20/mod1000 为 d=0,p=1（或 u128 p=2），可省 0%；zipf/少值大多在 block 层按等值桶结束，到不了 tile。主流形状没有可省的拷回。**之前"random −3～6%"无依据，撤回** |
| 64 | 顺序探测移到 variation 扫描之前（B16/P4）| 保留 | 当时的 `examples/sort_time.rs`（同一程序，只换库源码；现已删除）1M 中位数：升序 u64 2.61 → 2.24 ns、u128 6.48 → 3.37、u32 1.70 → 1.49；10M：u64 3.26 → 2.53、u128 6.61 → 3.61。全等输入为 strided 采样多付一次（1M u64 1.00 → 1.15），10M 持平。单次排序指令：u64 升序 −6.8%、u128 −19.5%，random 持平；random 与 sorted99 的 trace 逐字节相同。**注**：`radix_ab` 对 u32 ascending 1M 给出互相矛盾的两个 build 数（1.4 vs 2.0），见 §C |
| 65 | 叶子跳过逐桶遍历：前缀和记最大桶，最大桶 ≤ 插入阈值时整块插入排序（P1②）| 保留 | `radix_ab` 中位数，1M/10M：u64 random 18.3/27.3 → 15.1/23.1 ns、u64 random_wide 18.7/25.9 → 14.8/21.7、u128 random_wide 35.0/51.5 → 26.7/39.7；u128 narrow random 不调用 msd 叶子，不变；zipf13/sparse8/two_values/p95zero 全在噪声内。单次排序指令（1M）：u64 random −14.7%，u32 random（全平坦，max 累加被 DCE）±0.0%，u64 zipf13 +0.6%（fallback 记 max 的代价）。u64 random 1M 的 256 个 tile 全部走 insertion。Miri 7/7 |
| 66 | 加宽循环越过最后一个变化位 | 修正 | 9-bit span、采样集中的节点先取 8 bit，再按 +2 加宽到 10：shift = top - radix 下溢，partition_digit 的 assert!(shift < bits) 触发 panic（不是内存错误）。公开 sort 可达：100 万条落在同一字节内 3 个值的记录。现在 radix = (radix + 2).min(top)；回归测试用 16k 条 0..=511 的三值输入 + 1 KiB tile（Miri 下同形）|
| 67 | 共享值等值切分：采样找众数，<、=、> 三段，= 段直接完成（P2）| 保留 | 1M 中位数（p1 → 本项）：u64 two_values 8.5 → 6.6、p95zero 16.5 → 6.0、u128 two_values 19.4 → 11.5、p95zero 31.2 → 11.2 ns；10M：u64 10.0 → 7.1、15.8 → 6.6，u128 18.0 → 12.0、29.9 → 12.0。zipf13 首层削掉 32%（模式值占比）后回到 radix，约 −5~10%；sparse8/random 不动。**采样教训**：单步长众数采样与 sparse8 的周期 8 混叠，假 25% 众数让 10M +40%；改成 8 段 × 连续 8 键后消失（见 §C）。实现：block 引擎新增 `partition_by`（任意 classify 闭包、稳定）；当时由两端内扫确定等值段边界和子节点 variation；当前由 block 引擎提供边界 |
| 68 | 少数离群值：采样发现顶端位的少数派，把一端的短 run 抬出去（P3）| 保留 | 1M 中位数（p4 → 本项）：u64 random_sorted99 21.35 → 16.16、random_sorted95 21.95 → 16.11（1.32–1.36x）；random 15.14 → 14.71、zipf13 17.21 → 16.15、two_values/p95zero/sparse8 不动。对照：同样的输入去掉两个哨兵键后是 13.8–14.8，剩下的 ~2 ns 是抬出前的确认扫描。10M 上这个形状消失（前 12 位已把记录分到 10 个桶），21.67 vs 22.47 持平。机制：`sampled_minority` 判顶端位是否只被 ≤1/8 的样本持有；`outlier_run` 确认它只出现在一端的一小段（≤1024）并顺带算出主体 variation（散落的少数派在第一次不一致处退出）；`lift_outlier` 把 run 旋到该在的一端，主体递归，run 插入排序 |
| 69 | 流式 compact / 双缓冲（P5）| 拒绝 | perf 当时的 build 1M（含 harness 校验排序的周期）：memmove 占 u64 random 9.85%、zipf13 11.10%、sorted99 14.46%；“对根节点省一次”最多把这部分砍半 → 5–7%，低于 1.2x 门槛。真正能省的是整棵树在两个缓冲间乒乓（每层 1R+1W 而非 2R+2W），但那要求 MSD 递归接受“数据在 scratch 里”，是引擎重构不是路由改动 |
| 70 | 近似有序：一趟插入排序，按一个 tile 的键副本定价、按测得的移动率给预算（B19/s70）| 保留 | 独立程序（只排序，无 harness）1M u64：相邻对换 1% 8.89 → 1.05 ns/rec；同样 1% 但远距离对换 10.15 → 9.39（试算超预算，回退）；random 13.43 → 12.95。radix_ab 1M 中位数（s68 → s70）：u64 sorted99 15.37 → 1.36 / u64 sorted95 15.86 → 1.20 / u128 18.75 → 2.95 / u32 8.03 → 0.99；10M：u64 21.67 → 1.65、u128 27.30 → 3.04。其余 cell（random/zipf13/sparse8/mixed_runs/p95zero/runs64/bands8）在噪声内。机制：`probe_order` 判无序后，在**一个 tile 的键副本**上试算插入趟的移动数（预算 1 次移动/记录 = 一趟 radix 的写量）；失败只损失一个 tile 的读，数组一字节未动；成功则以 moves×n/tile 为全数组预算跑记录版插入排序。停下时只发生过交换 → 数组仍是原输入的排列，radix 照常接手；插入排序稳定，两路都稳定 |
| 71 | Dovetail 式多众数切分：采样出最多 8 个重键，各自给一个桶，轻键同一趟按 digit 分桶（灵感：DovetailSort 的 heavy keys）| **否决（代码已删除）** | 1M u64 中位数：two_values 5.5 → 13.0、zipf13 17.2 → 51.8、sparse8 7.0 → 15.1、p95zero 6.0 → 12.9 ns；random_wide 14.3 → 14.7（中性）。根因两条：(1) 在 block scatter 每条记录多做 rank/分类，实测每次 3–7 ns，直接抵消退休重复键的收益；(2) zipf 根节点只退休约 58%，剩余 42% 仍要分层，且轻键的 top digit 在区间内不分散，反而多一层。Dovetail 用 O(n) 暂存 + merge 支撑它的分桶/合并，搬到 O(√n) 块引擎只剩分类成本。portable SIMD 8 lane rank 在该形状也比标量 partition_point 慢（100k two_values：31.8 → 14.75 ns），说明逐记录 SIMD 比较没有预加载 lanes 不划算。 |
| 72 | 用 block 引擎返回的桶记录边界，并让子节点 variation 扫描在父节点候选 mask 全部出现过之后提前停止；等值切分同样直接用桶边界 | 保留（默认开启） | A/B（3 轮 × 7 次，1M u64，中位数）：random_wide 13.90 → 12.78、sparse8 8.47 → 7.64、zipf13 16.52 → 15.57 ns/记录；two_values ~持平。record16 26.97 → 25.02、record128 167.29 → 161.67 ns。思想来自 block 置换引擎（IPS²Ra/radsort）本来就持有桶边界：旧代码为了找边界又扫一遍所有 key；现在边界由 partition_digit_with_ends 返回，variation 只在候选位尚未全部出现时继续扫。父节点 split digit 在子节点内恒定，所以候选 mask = 父 varying 去掉 split mask。等值切分的左/右段也由 partition_by 的 ends 直接定位，少一次边界扫描。 |
| 73 | tile 内 MSD 叶子的大桶也用父 variation 的候选 mask 提前停止扫描 | 保留 | 1M u64（3 轮 × 7 次，中位数）：zipf13 16.16 → 15.67、p95zero 6.06 → 5.90 ns/记录；random_wide/two_values 持平。record16 zipf 30.3 → 约 27、record16 random 24.5 → 23.8；record128 random 持平，zipf 约 −2%（噪声内）。机制同 §72：tile 的 MSD digit 在桶内恒定，桶的候选 mask = tile varying 去掉 digit mask；旧代码对每个超过插入阈值的桶调用完整 varying_key_bits。 |
| 74 | 众数切分阈值从 1/4 放宽到 1/3、1/6、1/8（B18） | 否决 | 1M（5 轮中位数，RADIX_MODE_SHARE）：u64 sparse8 8.03 @1/4 → 6.84 @1/6，但 zipf13 15.54 → 21.38；1/8 把 sparse8 推到 19.04。u128/record16 同样出现 sparse 改善、zipf 退化的交叉。没有单一阈值同时赢，保留 1/4。 |
| 75 | 宽记录叶子的间接 tile 排序：key + u16 index 排好后 gather 一次（B09） | 否决 | 1M（7 轮中位数，强制 RADIX_INDIRECT=1）：record128 random 126.63 → 160.82 ns，zipf 170.58 → 170.71（持平）。16B pair 的 6 趟 counting pass 加 gather 仍不如现有 MSD 叶子；记录宽度不是唯一成本，pass 数与随机 gather 把收益吃掉。 |
| 76 | 跨节点复用 block workspace（B06） | 否决（噪声内） | 1M 三周期交替（5 轮中位数）：u64 random 13.02 → 13.13、u128 random 23.11 → 24.13、u128 zipf 23.97 → 24.51；record128 zipf/sparse 约 −2%；u64 sparse 7.36 → 6.54 但一轮 5.26 离群。整体方向不一致，保留每节点分配。 |
| 77 | 等值切分后常数子节点的 all-equal 快路径 | 否决 | 1M 三周期交替（5 轮中位数）：u64 sparse8 6.48 → 5.74、record16 sparse 16.04 → 14.76、record16 p95 9.51 → 8.95；但 u64 zipf13 13.67 → 15.13、record16 zipf 24.97 → 25.71，random/two 持平。低基数但非常数的子节点会被预读一路扫到首个差异，收益与损失交叉，不保留。 |
| 78 | 把 leaf 排序挂到 block compact 的桶完成点：每个 tile 桶物化到 input 后立即 scan_variation + lsd_leaf | 保留 | 1M（5 周期 × 7 次，中位数，RADIX_FUSE_LEAF A/B）：u64 random 12.25 → 11.89、u128 random 23.21 → 21.77、record16 random 21.85 → 19.42；u64 zipf 13.91 → 14.02、u128 zipf 23.99 → 24.03、record16 zipf 24.77 → 25.66；record128 random 122.39 → 123.45、zipf 168.92 → 170.11。随机 cell 稳定获益，zipf 小幅回退，宽记录持平；净收益为正，保留。这是 compact 后的缓存融合，不改变 leaf 内部 pass 结构，也没有处理 E1 的 flat 奇偶性。 §78 的 A/B 是 no-fusion 对 digit-only；当时 equality visitor 尚未加入。 |
| 79 | 等值切分的 less/greater 桶也挂到 compact 完成点；equal band 跳过 | 保留 | 1M 三周期 × 5（digit fusion 固定，RADIX_FUSE_EQ A/B）：u128 zipf13 23.35 → 20.72；u64 zipf13 13.47 → 13.63；u64 p95zero 5.15 → 4.94；u64 two_values 5.43 → 4.96、u128 two_values 10.67 → 9.63。s81 显示 two_values 的 fused-leaf 为 0，这组差异不能归因于提前叶子；保留依据是 zipf 有 37.4% 记录实际提前完成，two_values 数字视为噪声/布局。 |
| 80 | E1 覆盖统计：只统计 MSD→插入路径（RADIX_TRACE，1M u64，rounds=1） | 诊断，未实现 | random_wide 1,000,000/1,000,000 条（100%）走 leaf-msd 且 largest≤insertion；zipf13 2,443/1,000,000（0.2443%）走 leaf-msd，其余由 equality、直接插入或其他叶子完成；sparse8/two_values 为 0。harness 先校验再计时，所以原始 trace 是两次调用；比例已归一化。E1 最小切片只对 random-like 有效；zipf 需要 flat/equality 融合，而 flat 有趟数奇偶性。结合 s78/s79，E1 的默认收益范围比旧预估窄。 |
| 81 | fusion visitor 实际提前完成的记录统计（trace，u64，1M，--trace-once） | 诊断 | random 1,000,000/1M（256 个 tile）；zipf13 373,518/1M（37.35%，3735 个 tile）；two_values 0/1M（回调到达但外侧桶远大于 tile，未进入叶子）。two_values 的 §79 时间差不能归因于该机制；只有 random/zipf 的部分记录真正命中。 |
| 82 | 128 字节记录不挂 fused leaf，落回 compact 后排序（类型策略） | 保留 | 1M（5 周期 × 5，中位数，RADIX_FUSE_LEAF A/B）：record128 random 130.2 vs 131.2 ms、zipf 177.5 vs 179.4、sparse 102.9 vs 103.6、two 64.2 vs 66.0、p95 56.3 vs 56.8；128 B tile 的 visitor 让叶子多一份全 tile 工作集，8–16 B 记录仍保留 4–11% 融合收益。 |
| 83 | 分位数切分 + 等值桶（方案 1；另含链式恒等表证伪） | 否决 | 1M 7 轮：u64 zipf 18.75→20.98（+11.9%）、u128 zipf 35.13→45.53（+29.6%）、u64 p95zero 6.64→8.49（+27.9%）、u128 p95zero 15.27→25.76（+68.7%）；two_values 约 2x 慢；random/sparse 持平（random 无 ≥1/4 众数，未触发）。恒等表 1/2/3 次链式 load 只 +0.9/+0.9/+1.9 ns。分类器 + 重格不均衡吃掉省下的一层；zipf 一级桶仍有 206k 条。 |
| 84 | key 运算宽度跟随 varying 跨度（方案 2） | 保留 | 同一 32-bit 逻辑 key 的 1M 对照：u64 16.60 / record16 26.38 / u128 31.94 ns；u128 同记录宽度仍慢 21%。根节点 Shifted 视图 1M 7 轮 A/B：window32 30.37→25.94（1.17x）、small12 21.26→18.48（1.15x）、f64_normal 36.56→32.69（1.12x）、zipf 28.21→26.82（1.05x）、sparse 18.87→18.18（1.04x）；10M 同向 1.05–1.15x。默认开启；按 varying 跨度触发，不按类型名。 |
| 85 | compact 完成即递归所有子节点（方案 3） | 否决 | 10M 3 周期 × 7：u64 random 20.54→19.41、19.71→18.98、20.20→20.21；u64 zipf 20.76→21.40、21.27→21.26、21.04→20.62；u128 random 34.62→34.79、36.05→33.88、34.84→35.30；u128 zipf 32.14→32.81、32.96→33.18、33.34→34.65（ns/记录）；sparse 持平。符号不一致、改善 <5%，没有门槛级收益。 |
| 86 | scatter 依赖链微基准（方案 4） | 否决（前提不成立） | 1M 同记录/同桶大小，只改桶号顺序：random 9.03、runs8 7.33、runs64 8.12、runs4096 5.41、mixed_runs 5.81 ns/记录；同桶连续更快。寄存器游标缓存：7.94/5.54/5.06/4.61/4.81，未进真实引擎。mixed_runs 的慢不在游标依赖链。 |
| 87 | 叶子分支/CPU 成本（方案 5） | 关闭 | perf stat 1M u64（21 次排序）：random_wide 5.27M branch-misses（0.25/记录）、zipf 6.87M（0.33）、f64_normal 11.36M（0.54）；插入叶子已是最便宜收尾（s65 比逐桶快 14.7%），改它上限低于 1.2x。5c 与 s69/B13 的 E1 变体同族，不重开。 |
| 88 | 宽记录间接叶子 + 原地环应用（方案 6） | 否决 | record128 1M 5 轮：random_wide 138.59→181.12 ms（+30.7%）、zipf 200.40→194.64（−2.9%）、sparse 116.22→119.19（+2.6%）、two 79.61→77.42（−2.8%）。pairs 63 KiB、sort 在 L2，但 506 KiB 的随机置换吃掉第二层 split 的节省。附带 512 KiB tile（按记录字节特化）1M random 约 6–8%，10M 持平，按“不做类型特化”否决。 |
| 89 | Copy 记录在不确定 key 下必须不产生未初始化槽 | 保留 | 方案是把源 tile 先整块拷入 scratch 再 scatter（奇数 p 的拷入代替拷回，偶数 p 加缓存），配对矩阵（45 工况 5 block，A/A 中位 1.0000）里 Copy 路径 0.86–1.01，非 Copy owned16/128 为 0.941–0.976。随后改为 scratch 前缀复用：首个更大 tile 的首趟借分类缓存写满，之后直接复用，少一次每 tile 拷入，遍数与基线相同。 |
| 90 | 分类决策放在 `pass` 内（运行时 bool）还是按 const 泛型单态化 | 保留 const 泛型 | 运行时 bool 版把两条 scatter 循环塞进同一个未内联函数：u32 random_wide 1M 1.055（5/5 block > 1）、u32 4096 约 1.00、record16 64 0.958；`pass::<CACHED>` + 调用点分支后分别为 1.002、0.979、0.843。只改一处决策位置，不用为类型或 key 复制排序实现。 |
| 91 | `largest` 在前缀和循环还是直方图循环累加 | 保留直方图 | 前缀和版 u64 random_wide 4096 1.072、record128 random_wide 4096 1.020、u32 random_wide 1M 1.019–1.037；直方图版 0.971／0.972／1.002。tile 小、digit 宽时桶表约为记录数的四倍，逐桶累加比逐记录更贵。`largest` 只用于“整 tile 插入”判断。 |

| 92 | 变化位连续时把位宽均衡摊到同样趟数 | 保留 | 4K u32 原计划 3 趟 14+14+4 = 32784 桶槽（8 槽/记录），均衡成 11+11+10 后 5120 槽，趟数与散布次数不变；桶槽实测约 1 ns、计数趟 1.6（L1，8 位）到 3.7（冷，12 位）ns/记录。同趟数下更少的表工作只能靠均衡，不能靠加趟（§59/§61 的代价） |
| 93 | MSD 窗口与 LSD 计划解耦 | 保留 | 原先 MSD 直接借用 LSD 的第一个窗口；改为按自身采样取最宽窗口后，32 位键在 4K 的 MSD 收尾从 10.0 降到 6.6 ns/记录 |
| 94 | 一层 MSD 按"分桶 + 收尾"两趟计费 | 保留 | 原来的 3 趟使门槛"趟数 > 3"把 2–3 趟计划全挡在 MSD 外，32 位键 1K–64K 正好落在这段；改正后 u32 1024/4096 达 0.593/0.614（配对 5 block） |
| 95 | 入口试算在窗口覆盖整数组时就地执行 | 保留 | 省一次键副本分配与拷贝：n≤32 为 0.70–0.71、1K–4K 6%–8%；安全测试改为扫描切换点后不再依赖调用次数 |
| 96 | 插入收尾阈值 48 是否该提高 | 关闭 | 上限扫到 96 时 96 条 tile 变成 35 vs 12 ns/记录；24/48 在冷侧难分（32 条仍以插入更快）。48 保留 |
| 97 | 安全测试从"钉 key 调用次数"改为"扫描切换点" | 保留 | 入口试算让调用次数随数据变化，原断言（196/220…）只能靠改数字维持；改为先测该输入的调用总数，再在区间上扫 panic/改答案位置，覆盖点从 3–5 个变 16 个，计划变化不再需要重推 |

| 98 | 桶槽上限按记录数收紧（每记录 8 槽 → 2 槽） | 否决 | 同进程四臂交替（base=8 槽，even=2 槽，3 block 中位数）：u32 4096 1.259、8192 1.168、16384 1.142、1M 1.115、128 1.065；u64 16384 1.434、65536 1.417、1M 1.436（>1 表示收紧更慢）。只有 u64 128/512/2048 变快（0.87–0.97）。说明这一档的差距不在表/数据比，而在**多一趟**的代价：窄窗口省下的表流量补不回多出的整趟扫描 |
| 99 | 16K 与 radsort 差距的成因 | 度量 | 同为 L2 驻留，perf 1M 记录量级：tessera 指令 1.14x、周期 1.49x、L1 载入缺失 2.56x（每记录 2.66 vs 1.04）。缺口是**每条记录碰一个冷桶游标**（1 万级桶 ≈ 数据规模的游标表），不是指令数。可行的方向是两级分桶让游标表常驻 L1，或按桶序而非记录序散布；两者都是结构改动，另起一轮 |

| 100 | 每趟 digit 宽度扫描（同进程、同代码、3 block 中位数） | 关闭 | u32 ns/记录，wide/14/12/10/9/8 位：4096 6.59/6.75/7.49/12.58/14.97/17.82；16384 8.99/9.10/10.76/19.17/21.90/20.87；65536 10.24/10.50/11.18/11.06/20.95/24.10；1M 10.49/10.79/12.20/12.04/12.09/26.26。当前规则（尽量宽）在每个规模都最优；radsort 那种 8 位多趟在本实现里慢 1.3–2.3 倍，说明差距在每趟成本而不是趟数。**方法学**：同规模 A/B 若用两份模块编译同一份代码，内联差异会伪造出 5–20% 的差（§98 的 1.14–1.43 即此类），只有同代码 + 运行期开关的同进程交替可信 |
| 101 | 每趟成本高于 radsort 的来源 | 诊断 | 16K u32：指令 1.14x、周期 1.49x、L1 载入缺失 2.56x。Copy 记录的分类缓存占 2 字节/记录（首趟写、后续趟读），加游标表与 tile+scratch 往返，每趟搬运约为数据的 3 倍；radsort 每趟约 2 倍且无分类缓存。去掉缓存会退回 §89 的安全问题，所以下一步是让一趟携带 (key, payload) 对、减少一次往返，而不是继续调窗口 |

| 102 | 去掉 Copy 分类缓存能省多少（上界测量） | 关闭 | 同进程开关强制 cache=false（键稳定时安全）测其上界：u32 4096 +0.9%、16384 −5.1%、65536 −0.8%、1M +0.4%（负值才是收益）。缓存只在首个 Copy 趟写 2 字节/记录，后续趟本来就重算键，所以"每趟读 32 KiB"不成立。收益 ≤5% 且只在一个规模，不值当改写成"越界守卫 + 插入回落"并重推安全不变量 |
| 103 | 小数组继续收窄表宽 | 关闭 | 128/256/512 上 wide 与 8 位基本持平（11.38/11.40、8.61/8.42、7.45/7.42），7 位以下全面变慢；1024 起 8 位是 12.57 vs wide 7.82。小数组的差距在入口（探测 + 试算 + 一次分配），不在表宽；n≤48 直接走插入，剩余可压缩空间约 15%，绝对量 <15 ns |
| 104 | 可读性重构的零退化检验（基线 `747d9f5`；每个候选单独编译，先逐函数比较指令流再做配对计时，5 周期×60 cell） | 部分采用 | **采用（C1）**：`MAX_RADIX_BITS` 取代复制的 12、`bit_len`、`Sorter::new`、少数派比例改名、`WIDENING_SAMPLE_BITS` 改为由 `DIGIT_SAMPLE_LEN` 推出、去掉恒真的 `BITS <= 128`、模块词汇表；166 个 Tessera/block 函数除断言消息长度立即数外指令流逐条一致，函数起始地址对 32 取模与基线相同；AB 中位数 0.994，最大 1.039。**不得删 `sort_tile` 里的两处 `.max(1)`**：值上冗余，但向优化器保证除数非零，删后多一个零检查分支并重排寄存器，u64 128 的 zipf11／random_d256 慢 8–11%（15 周期复测，AA≈0.99）。断言消息改成带 `{}` 的格式串也会令 `partition_digit_inner` 变大 33 字节，已改回字面量。**未采用**：离群段改存目的端并用 `split_at_mut`、`sort_child` 合并子节点递归（C2）。热函数指令流不变，但 `probe_order`／`sort::<u32>` 起始地址对 32 取模从 0 变 16，u32 随机 4096 慢 10.8%、降序 65536 慢 24%、u64 随机 4096 慢 21%（AA≈1）；与对齐相关，因果未证。把 `partition_tile`／`sort_tile` 的 pass 分派合并为 `run_pass`、`RestoreTile` 持有备用缓冲、把根部预处理抽成 `finish_if_ordered`／`sort_nearly_ordered`：改动了 `sort_tile`、`sort::<T>`，未在对齐受控的条件下单独计时，不采用。后续对照（不采用，仅作测量参考）：同源码仅换编译选项，基线 vs C2 变慢的 cell 数为 无选项 16、`-Zmin-function-alignment=64` 1、`-C llvm-args=-x86-branches-within-32B-boundaries` 3（含 1 个 AA 噪声）；但 64 对齐本身让基线 17/60 cell 变慢（最多 +15.5%）、几个 cell 变快，不是免费优化；u32 random_d256 4096 上 `idq.mite_uops` 占比 7%（无选项）→33%（64 对齐）→6%（JCC 缓解），说明 Skylake-SP（Xeon Gold 6140）的 DSB／JCC erratum 是位置敏感的主因之一，但 JCC 缓解后仍剩 3 格差异，未查清，不可当作已解决。结论：会改机器码的重构无法用逐 cell A/B 证明零退化，且 `-Z` 选项在 Stable 下不可用，故 C2 及 `run_pass`／`finish_if_ordered` 抽取均放弃；只接受指令流逐条一致的改动。**harness 注意**：`radix_ab --batch N>1` 只克隆一次缓冲，第一轮后反复排序已排序数据，不能用于比较，须用 `--batch 1` |
| 105 | Copy 记录也走桶号缓存的成本（独立仓库 API 决定；同一二进制 A1/A2/逐字副本/全缓存副本，5 周期，[记录](results/tessera/cache-default-20261106/README.md)） | 度量 | cached/A：u32 random 16K–1M +3.0–4.4%；u64 1K–4K −2.1–4.0%、random 64K +2.7%；record16 random 16K–1M 最多 +3.5%，skewed 1K–4K −1.9–3.4%；record128 在 ±1.4% 内。放置对照本身最大偏离 4%（u64 random 1M 0.960）。新库 `stable_sort_by_key` 对所有类型存桶号，`stable_sort` 不存。本轮 harness 改为每次计时前在计时外恢复全部缓冲：原 `--batch 1` 把 `clone_from_slice` 计入耗时，与 C 节约定不符，历史 `--batch 1` 比值因此被稀释。 |

## C. 测量与生成器约定

- 固定 `OVERRIDE_SEED=1`，绑同一 CPU，交替 A/B，以 A/A 估计当轮波动。
  比较逐 cell 结果，报告中位数和所有原始轮次；单次 min 不作为收益依据。
- 计时仅包含排序。生成、clone、参考排序和 distinct 统计均在计时外（`radix_ab` 的
  `--batch 1` 实际把恢复输入的 clone 计入耗时，`--batch N>1` 不恢复输入，见 §104、§105）；clone 后的小数组
  可能驻留缓存，不能直接把 1M→10M 的变化解释成算法缩放。
- `random_wide` 在 u64/u128 使用独立 SplitMix64 draw；普通命名分布由 i32 保序映射到
  目标宽度，u128 的低 96 bit 恒定。`type_*`、`small12`、`window32_high` 等直接构造目标位型。
  `f64_*` 是浮点列转换后的整数排序键。
- record16/record128 与 u64 共用键生成器。harness 的零 payload 只检查键顺序；稳定性和
  记录身份由核心测试及 `radix_owned` / `radix_views` 检查。这两个集成测试此前只覆盖 block；
  本轮新增 Tessera 覆盖，并检查 panic 后实际恢复的记录和不实现 Drop 的独占引用。
- `random_sorted99` 是相邻对换；u64 的保序映射会让开头两个键落在主体最高位下方。
  它同时覆盖短离群 run。远距离对换具有不同的插入交换成本。
- 历史上 `type_top_byte` 固定移位导致窄类型全等（§52）、完整位宽随机只有 32-bit 熵
  （§35）、周期采样与 sparse8 混叠（§67），相关结论以修正后的账本行为准。
- 指令计数用于解释工作量，墙钟用于判断耗时；局部 microbench 和静态模型不能代替
  完整排序。历史 VM 事件不足以直接确认 RFO 或物理性能下界。
- 旧的 1.2x 晋级门槛是当时的实验取舍；本轮保行为清理按逐路径回归和当轮噪声判定。

## D. 当前复现入口

```sh
# Cargo.toml 关闭了默认 lib test 目标，因此单独运行。
cargo test --release --features rust_tessera_radix --lib
# 小数组的计时开销用 --batch 摊掉（每轮按 N 次排序计时）。
cargo bench --features rust_tessera_radix --bench radix_ab -- --sizes 8,32,128 --batch 64
cargo test --release --features rust_tessera_radix
cargo +nightly miri test --features rust_tessera_radix --lib

# 只测保留实现；每个变体先与 std 稳定排序核对键顺序。
OVERRIDE_SEED=1 RADIX_RAW=results/tessera/new-run.csv taskset -c 2 \
cargo bench --features rust_tessera_radix --bench radix_ab -- \
  --types u32,u64,u128 --patterns random_wide,zipf13,random_local \
  --sizes 100000,1000000,10000000 --rounds 7 --variants tessera,block_lsd_r8
```

CSV 无表头，列为 `revision,type,pattern,n,variant,round,seconds`。新记录的 revision 是
完整 git describe（含 dirty 标记）；历史文件保留旧的短 SHA。跨 build 对照还需另存源码和
二进制哈希、编译参数、执行顺序，不能仅以运行时仓库的 HEAD 归属已复制的旧二进制。
`RADIX_RAW` 写失败会直接报错。

## E. 后续工作的重开条件

- split 物化与首趟 tile scatter 融合（旧 E1）尚未实现。先证明可融合字节覆盖；§80 显示
  1M u64 random 全部命中 MSD→插入，zipf 仅 0.24%，sparse/two 为零。
- 双缓冲省拷回（§63）在主流形状可省 0%；仅在多层 tile MSD 或 MSD 后奇数趟 LSD
  占比显著时重开。
- 少量不同宽值的整体比较式分区需要独立原型；单主导值已由等值分区处理。
- 其余否决实验的重开需要新的负载或机制证据。当前无排定的性能实验。

## F. 外部整数排序对照（历史对照）

这一节只回答三个比较问题，不改 Tessera 的路由或数据结构：

1. 与成熟的不稳定、低额外空间整数 radix 相比，Tessera 在哪里；
2. 允许稳定的 O(n) 工作区后，性能如何变化；
3. 在完整 `u64/u128` 值域上，SIMD 排序实现处于什么位置。

### F.1 接入对象

| 适配器 | 上游 | 稳定性 | 额外空间 | 适配类型 |
| --- | --- | --- | --- | --- |
| `cpp_ips2ra_unstable` | IPS²Ra `83f6b76` | 不稳定 | 默认 256 桶 × ~2 KiB 块缓冲（~512 KiB 固定）+ 交换块/任务元数据 | `i32/u32/u64`，`Record16/128`（u64 键） |
| `cpp_dovetail_stable` | DovetailSort `f4630a2`，`PARLAY_SEQUENTIAL` | 稳定 | O(n) 记录暂存数组 | `i32/u32/u64`，`Record16/128`（u64 键） |
| `cpp_vqsort` | Highway 1.4.0 `2607d3b` | 不稳定 | 无堆分配；~1.2 KiB 栈 + TLS 随机状态 | `i32/u16/u32/u64/u128`，`u128` 是完整 128 位键 |
| `cpp_intel_avx512` | numpy/x86-simd-sort `6e059b1` | 不稳定 | 上游内部工作区（未计入 Rust 分配器） | `i32/u32/u64` |

`u128` 没有被 IPS²Ra/Dovetail 声明支持：前者采样路径缺 128 位 `clz/ctz`，后者没有单独验证 128 位键。
`Record128` 是 128 字节记录，不是 VQSort 的 `uint128_t`。

所有 C/C++ 适配器由 `build.rs` 统一用 `clang++` 编译；`cpp_std_gcc4_3` 的历史 GCC 4.3 基线不在本轮清单里。
IPS²Ra/IPS⁴o 的 TBB include 加了串行构建守卫（不改变算法）。已有 `cpp_ips4o`、
`rust_radsort` 与 `radsort_keyed` 也已接入 `radix_ab`；它们在 1M 下明显更慢（radsort 63.4、IPS⁴o 40.2
ns/记录），不放进下面的快表。

### F.2 本机小规模对照

5 轮交错，中位数，ns/记录。机器同本文历史记录；共享 VM，比例只看量级。**本节表格只到 1M。**

`u64`，1M：

| 变体 | random | zipf13 | two_values |
| --- | ---: | ---: | ---: |
| `cpp_vqsort` | 12.59 | 4.88 | 1.52 |
| `tessera` | 15.32 | 18.45 | 5.89 |
| `cpp_intel_avx512` | 17.53 | 7.35 | 1.68 |
| `ipnsort` | 27.59 | 10.43 | 4.03 |
| `cpp_ips2ra_unstable` | 29.48 | 26.85 | 24.51 |
| `cpp_dovetail_stable` | 32.48 | 46.95 | 9.17 |

`u128` / 记录，100k：

| 变体 | u128 random | u128 zipf13 | rec16 random | rec16 zipf13 | rec128 random | rec128 zipf13 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `tessera` | 25.57 | 19.12 | 16.04 | 22.22 | 91.60 | 116.17 |
| `cpp_ips2ra_unstable` | — | — | 28.55 | 29.07 | 106.08 | 144.62 |
| `cpp_dovetail_stable` | — | — | 37.45 | 60.02 | 171.03 | 175.73 |
| `cpp_vqsort` | 27.97 | 9.95 | — | — | — | — |
| `ipnsort` | 41.02 | 11.85 | 31.40 | 10.59 | 127.36 | 62.91 |

要点：

- **VQSort 对照**：1M `u64` random 比 Tessera 快约 1.22x，zipf13 快约 3.8x，
  two_values 快约 3.9x。100k `u128` random 上 Tessera 略快（25.6 vs 28.0），zipf13 上 VQSort 仍快 1.9x。
- **IPS²Ra 对照**：本机串行配置在 1M `u64` random 比 Tessera 慢约 1.9x；
  record128 random 也慢 1.16x。这组结果包含其固定桶缓冲和串行分类成本。
- **Dovetail 对照**：O(n) 暂存 + 合并，u64 1M zipf13 为 47.0 vs Tessera 18.5；
  record128 random 为 171.0 vs 91.6。它在 two_values 上比 Tessera 慢，但仍快于比较排序参考。
- **x86-simd-sort 与 VQSort 同属 SIMD quicksort 家族**：random 上 VQSort 领先约 1.39x，
  zipf13 约 1.51x；修正 unsigned 适配后，它的 `u64` 结果可用于完整值域。
- 早先 g++ 编译的 IPS²Ra `record128` 数字已作废；上表统一为 clang++。

### F.3 空间口径

`bench extra_memory` 只统计 Rust 全局分配器。上面两个 C++ 适配器的 `new/malloc` 与栈空间**没有**被它计入，
所以不能把它们的额外空间读成 0。IPS²Ra 的固定桶缓冲、Dovetail 的 n 记录暂存数组按上游结构单独报告；
VQSort 按上游声明为无堆分配。

### F.4 复现

```sh
# 重跑对照，1M 上限；--variants 精确匹配
cargo bench --features rust_block_lsd,rust_tessera_radix,cpp_ips2ra,cpp_dovetail,cpp_vqsort,cpp_intel_avx512 \
    --bench radix_ab -- --types u64 --patterns random,zipf13,two_values \
    --sizes 100000,1000000 --rounds 5 \
    --variants tessera,ips2ra,dovetail,vqsort,intel_avx512,block_lsd_r8,std_stable,ipnsort

# 主 benchmark：BENCH_REGEX 要加 $，否则 ...-10000 也会匹配 ...-10000000
BENCH_REGEX='^(cpp_ips2ra_unstable|cpp_dovetail_stable|cpp_vqsort|cpp_intel_avx512)-hot-u64-random-10000$' \
    cargo bench --features cpp_ips2ra,cpp_dovetail,cpp_vqsort,cpp_intel_avx512 --bench bench
```

验证：`tests/cpp_ips2ra.rs`、`tests/cpp_dovetail.rs` 走标准套件；`tests/cpp_dovetail.rs` 另有
`Record16/Record128` 等键保序测试；`tests/cpp_vqsort.rs` 检查完整 `u128` 键；
`tests/cpp_intel_avx512.rs` 检查 `u32/u64` 的无符号顺序。

能力声明顺带修了三处 harness 假设：`basic` 不再对不支持的 `()` 调用排序；
`observable_is_less_u64` 尊重 `supports_sort_by`；稳定性测试先检查元组类型是否受支持。
