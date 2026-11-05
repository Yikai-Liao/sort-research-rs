# Tessera 实验记录（stable tiled radix sort）

分支 `exp/radix-opt`，从 `feat/block-lsd`（429f6f7）分出。Block LSD 不动，作为基线；候选算法是新模块
`rust_tessera_stable_radix`。这是研究代码，不是合并提案。

> **本文件的组织方式**：先看 §A 结论，再按需查 §B 账本（每个实验一行，编号 §N 与历史一致，代码注释和
> backlog 都按这个编号引用），§C 是方法教训，§D 是空间/验证/复现，§E 是未决事项。
> 压缩前的完整流水账在 git：`git show a80b566:EXPERIMENTS.md`。

## A. 结论

### A.1 最终设计

1. 一层稳定 MSD split，digit 由采样决定宽度（偏斜时每次加宽 2 bit，直到 1/4 桶被占用，上限 12，§49）。
2. 子节点能放进 ~31 KiB tile 就在缓存里收尾：叶子 = 插入排序（≤48 条）/ 一层 MSD（tile 内）/ 平坦计数 LSD，
   由两个计数量决定：还需几遍、采样窗口里有几个非空桶（§53、§58）。
3. 路由规则只有一条：**节点还需要 ≥2 遍就 split**，实现为 varying 位跨度 > RADIX（§41、§51）。
   不含带宽、缓存大小、延迟等机器常量。
4. 常量位处处丢弃（per-node variation）；digit 窗口跟着 varying 位走，不走固定网格（§13）。
5. 有序输入先探测：升序直接返回，降序反转并修复等键段（§9）。
6. 辅助空间 O(√n)：块置换引擎 + 一块常量大小 tile 缓冲（§D.1）。

### A.2 数字（1M，ns/记录，中位数，本机）

完整表格在 `CEILING.md`。要点：

| cell | tessera | ipnsort | 相对基线 block LSD |
| --- | --- | --- | --- |
| u32 random | ~9–11 | ~23 | 持平（u32 在 1M 不 split） |
| u64 random | ~18 | ~25 | 约 2x（§25、§35） |
| u128 random | ~32 | ~45 | 2–4x（§23、§25，随负载变）；1M 领先 ipnsort 1.40x，10M 1.31x（§35） |
| record128 random | 155.6 | 183.9 | 1.18x 领先 ipnsort（§37） |
| u64 ascending | 2.8 | 1.2 | 12.8x（§9） |

输的地方（比较排序更好）：**重复键重的分布**——zipf、few-values、sparse8 的 u128（ipnsort 快 2.4–2.9x，
§39、CEILING.md）。这是 radix 的结构性代价，答案是比较式 partition，不是再调 radix。另有 pipe_organ
（两段长有序，std 12.5 vs 22.3，§20）。

### A.3 为什么说已到天花板

- 本机 VM 1M 时几乎每次访问都 miss cache（37 miss/记录，§47）。split 在这台机器上已在 256 流 scatter 的下限；
  叶子在自己的 pass 下限。
- 后期所有"少几条指令"的改动（§59/§60/§61）指令数下降但墙钟变慢：**本设计在这些规模下，任何增加每记录工作集
  压力的机制都不会赚钱**（见 §C.3）。
- 剩余 backlog 的预期收益都 ≤ 5%，而本机噪声 5–20%，无法分辨（见 `BACKLOG.md`）。

---

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
| 12 / 17 | 测量卫生 | 方法 | 见 §C.1。一次是单个异常轮次污染 min；一次是后台遗留的 criterion 跑了 46 分钟 |
| 13 | 窗口按 varying 位打包 | 保留 | u128 8 narrow bands 37.0 → 31.3；稠密随机不变 |
| 14 | 更宽的 split fan-out（9–11 bit） | 否决 | u64 10M 35.4 → 37.9 → 41.4。2048 路流的页/TLB 压力大于 L1 常驻叶子的收益 |
| 15 | 流式回退函数 `#[cold]` | 保留，§31 又撤 | u32 1M 0.92x → 1.12x；10M 时每个子节点都走它，cold 标注变成谎言，§31 去掉 |
| 16 | 叶子缓冲大于 tile | 回滚 | 10M u64 33.1 → 32.7（噪声内），且 64–512 KiB 数组上缓冲变 O(n) |
| 18 | 预取 scatter 目标行 | 否决 | 慢 25–36%（u128 1M 76.7 → 103.3）。同样是 store 口瓶颈，要少遍数，不是更好的一遍 |
| 19 | merge 叶子 key panic 会复制+丢失记录 | 修正 | `ptr::read` → bitwise copy + Drop 恢复；41 个 panic 点的扫描测试，两半缺一不可 |
| 20 | u128 的 pattern 全是 32 位 key 装 128 位容器 | 修正 | 重做 type-space pattern。`two_near`（1 个 varying bit）u128 11.7 vs ipnsort 7.2：证明"无变化"的全扫描成本 |
| 21 | 叶子宽度逐 tile 不同，路由模型不得跟随 | 保留（模型后废） | u32 10M 因扫宽度而 13.9 → 18.6（路由被带偏）。u128 最优 14 bit、u64 最差 14 bit（32 KiB 游标表 = 整个 L1）|
| 22 | 四份独立 review | 度量 | 发现：叶子遍数随 key 宽而非 tile 记录数 → §23；≤8 MiB 的 cell 是热缓存；record 类型 payload 全零看不出稳定性回归（见 §C.4）|
| 23 | tile 内先做一层 MSD | 保留 | u128 1M 76.8 → 31.9（2.41x），u64 1M 25.4 → 20.2（1.26x）。窗口须先采样（否则 random_local 慢 3x）；表不清零（省 14%）|
| 24 | 路由模型给叶子标了过时的价 | 修正 | 10M u64 34.2 → 27.6；u128 10M 91.9 → 49.1。u32 10M 仍是边界 case |

### B.2 清理与简化期（§25–§34）

| § | 主题 | 状态 | 关键数字 / 理由 |
| --- | --- | --- | --- |
| 25 | 删 merge 叶子；5 个入口并成 `Policy`；`descend` 与 tile MSD 共享 `for_each_bucket` | 清理 | merge 在一个 cell 赚 6%、另一个亏 15%，不是路径。**但**共享"边界来源"不行：tile MSD 比较 digit 来找边界慢 48%（19.4 → 28.8），保留游标 |
| 26 | "始终 split" 取代成本模型 | 否决 | 模型在主流 cell 持平或更好；"2x 更好"的 cell 是噪声（静音机器 5 轮后 9.1 vs 9.2）。**教训进 §C.1** |
| 27 | level-by-level 路径改用本模块 planner | 保留，§29 删 `StreamMode` | 与块引擎自己的排序持平，于是路径合并 |
| 28 | 一个 planner 同时给路由定价和执行 | 清理 | 常见形状路由逐节点相同（6/12/18 stream 节点前后一致）；clustered 形状 u64 zipf −8.5% |
| 29 | `tile_cost` 合并进路由 | 回滚 | 路由只知道"造出该子节点的 digit"的占比，远比子节点自己的 top 窗口粗 → 312 KiB 子节点都被当成聚簇，u64 10M 258 → 323 ms（+25%）。路由改按"叶子最好情况（一层）"定价 |
| 30 | 整轮简化的总代价 | 度量 | 18 个常见 cell 中 17 个指令数在 ±1.5% 内，最差 u32 two-values +4.2%。墙钟跨 build 无法分辨 ±5%（9 好 9 坏）|
| 31 | 10M 回归检查 | 修正 | 去掉 stream_node 上的 `#[cold]`。`perf stat` 显示 10M 指令数 0.02% 差 → 墙钟差是频率/布局 |
| 32 | 路由模型去量纲 | 保留 | 原模型对任一常数扰动 2x 就改变 40–60/652 条路由。改成纯遍数算术；33 个 cell 指令全在 3.7% 内 |
| 33 | 新 workload：`mixed_runs`（长度 16–2064、方向随机的 run）| 度量 | u64 1M 18.2 vs ipnsort 24.4（1.34x）；u128 持平；这是 merge 多个预排序源的真实形状 |
| 34 | 哪些常量是机器相关的 | 度量 | 只有 `TILE_BYTES = 64 KiB`（是容量不是速率，在 plateau 上）。其余全是流/记录/遍数的计数，移植无需重调 |

### B.3 审查、修正与"最后 5%"期（§35–§62）

| § | 主题 | 状态 | 关键数字 / 理由 |
| --- | --- | --- | --- |
| 35 | 对 7da8daa 的外部 review | 修正 | (1) merge 叶子 unwind 洞：随代码消失。(2) 说谎的 `Key` 可使 scatter 越界 → scatter 目的地 clamp。(3) "full-width random" 其实只有 32 bit 熵 → SplitMix64 重测，**u128 1M 变成领先 ipnsort 1.40x** |
| 36 | tile 内原地 MSD 层 | 回滚 | 先宣称 −7%，是单次配对的**周期数**。正式测：u64 1M 18.7 → 25.8，u128 34.3 → 42.6。原地环遍历是随机跳链，scatter 是顺序写。同提交留下的"直接走插入的桶不扫 variation"保留（u128 sparse8 −6.7% 指令）|
| 37 | 叶子 digit 宽度按记录数封顶（+2 bit）| 保留 | record128 1M 290 → 155.6（从落后 1.48x 到领先 1.18x）；8 字节记录不受影响 |
| 38 | Miri 证据范围 | 度量 | 当前 HEAD **7/7 Miri，216 s**（测试尺寸按 `cfg!(miri)` 缩小；此前的审计 agent 因测试 1M 元素超时 30 分钟）|
| 39 | 叶子消融（flat-only vs shipped）| 度量 | MSD 层不可去：u128 random 无它 81.4 vs 35.7。u128 sparse8 落后 ipnsort 2.9x |
| 40 | 冷/热遍数比 r=3 | 否决 | u128 sparse8 −53%，但随机 cell 指令 +8–14%。被 §41 取代 |
| 41 | 路由改为"还需 ≥2 遍就 split" | 保留 | u128 sparse8 35.7 → 15.0；u64 10M −2.6%。机制上去掉机器事实 |
| 42 | 天花板表 | 度量 | u128 在 pass 下限的 0.3–0.6x；u32 离下限最远（1.0–3.1x，每遍固定开销主导）。更新版见 `CEILING.md` |
| 43 / 45 | 删 `SPLIT_PASSES`、`flat_hot`、`hot_passes`、`MODEL_LEAF_RADIX` | 清理 | 指令 −0.02%，路由行数 2566 前后相同 |
| 44 | lib 测试目标 3 个提交没编译 | 修正 | "N targets green" 靠数 `test result: ok` 行，看不到编译失败。现按 7 个目标含 lib 分别点名 |
| 46 | split 占 u64 的 29–34%、u128 的 52–55% | 度量 | split 是自身下限的 1.7–2.1x（u64）/ 2.5–2.7x（u128）|
| 47 | RFO 假设 | 度量 | VM 无 LLC 事件，**不可直接验证**。cache-miss 37/记录 → 1M 无缓存常驻可赢。结论后在 §52 收窄为"排除缓存常驻收益，不证明 split 无法靠别的写模式变便宜" |
| 48 | 叶子宽度再扫描 | 否决 | 单次看似 −6%，3 组交错重复符号翻转。无宽度赢 auto 超过 5% |
| 49 | split 在窗口不分散时加宽 2 bit | 保留 | u32 zipf −17.4%、u128 zipf −16.0% 指令；sparse8 +1.6–2.9%；其余 ±0.5% |
| 50 | 把后代聚合折进置换（删 descent 遍） | 否决 | 指令 +7–16%，墙钟 u32 1M 10.7 → 20.3。**决定性先例**：在 scatter 热循环里再加每记录两次 RMW 不值 |
| 51 | 路由改读 varying 跨度，不再调用叶子 planner | 清理 | 4770 个节点 0 个决策差异；指令持平 |
| 52 | 外部 review：两个生成器 bug + skew 家族 | 修正 | `type_top_byte` 在 u32/u64 全是等值键（结论作废）；`sparse8` 只用了高 32 bit（§40 的机制解释错，结果仍在 2% 内）。skew10–99 无悬崖。统计量改为**中位数**，`raw_runs.csv` 存档每一轮 |
| 53 | 叶子 f64 成本模型 → 两个计数 | 清理 | 14/15 cell auto = MSD-only；−23 行。flat-only 在 u128 random 上 82.1 vs 31.2 |
| 54 | 单 digit 终点（idea 01）| **作废** | 两个 build 各自生成随机数据，"+14.3% 指令"是数据差。见 §56 |
| 55 | 永远过不了的加宽采样（宽 ≥10 跳过）| 清理 | `OVERRIDE_SEED=1`，路由 trace 8 cell 逐字节相同，指令 ±0.03%。**暴露 §54 的方法错误** |
| 56 | idea 01 带种子重测 | 保留 | trace 10 cell 完全相同，指令 ±0.6%，删 `stream_node`、`sort_with_digits`。墙钟同数据 ±10%（噪声大于效应）|
| 57 | block 大小扫描（0.5x–4x）| 否决 | 指令全程只跨 1.0–2.1%，无台阶。`RADIX_BLOCK_SCALE` 钩子保留 |
| 58 | 恢复叶子 guard（整数版深度估计）| 修正 | §53 过度简化：u64 random_local 27.10 vs flat 23.46。恢复后 24.30 vs 24.28，指令 −4.2%。**注意**：这是**两组独立配对**，不要合并成一个精确的跨 build 百分比 |
| 59 | 叶子窗口按数据大小定 | 否决 | 指令 −5% ~ −11%，墙钟 +4% ~ +8%（精确版 +43%）：表窄 → 多一层 → 多一遍 tile |
| 60 | rank 压缩叶子 | 否决 | 正确，且在 bands8/zipf/skew99 赢 4–5%；但无门版 random_wide −25%。门后同 cell 两次跑差 10–20 个点（噪声）|
| 61 | N3：下一遍直方图融进平坦 scatter | 否决 | 指令 −1% ~ −7%，墙钟 u32 random **+21.6%**、99% sorted +11%。第二张 16 KiB 表把 31 KiB tile 挤出缓存。**这是第三次"指令降、墙钟升"**（见 §C.3）|
| 62 | `RADIX_BLOCK_SCALE` 读环境变量改 `OnceLock` | 清理 | 不声称性能。每个 split 节点少一把锁 |
| 63 | N6：tile MSD 交替两个缓冲，省拷回 | 关闭（无覆盖）| 未实现，只做了覆盖测量。每条记录当前拷回 `d+(p mod 2)`（d=tile 内 MSD 层数，p=末端 flat 趟数），交替后最少 `(d+p) mod 2`。临时诊断补丁（未入库）在每个终止区间记 (d,p,len)：u64/u128 random 在 2e4/1e5/1e6/1e7 **100% 记录为 d=1,p=0**，可省 0%；local20/mod1000 为 d=0,p=1（或 u128 p=2），可省 0%；zipf/少值大多在 block 层按等值桶结束，到不了 tile。主流形状没有可省的拷回。**之前"random −3～6%"无依据，撤回** |
| 64 | 顺序探测移到 variation 扫描之前（B16/P4）| 保留 | `examples/sort_time.rs`（同一程序，只换库源码）1M 中位数：升序 u64 2.61 → 2.24 ns、u128 6.48 → 3.37、u32 1.70 → 1.49；10M：u64 3.26 → 2.53、u128 6.61 → 3.61。全等输入为 strided 采样多付一次（1M u64 1.00 → 1.15），10M 持平。单次排序指令：u64 升序 −6.8%、u128 −19.5%，random 持平；random 与 sorted99 的 trace 逐字节相同。**注**：`radix_ab` 对 u32 ascending 1M 给出互相矛盾的两个 build 数（1.4 vs 2.0），见 §C.1 |
| 65 | 叶子跳过逐桶遍历：前缀和记最大桶，最大桶 ≤ 插入阈值时整块插入排序（P1②）| 保留 | `radix_ab` 中位数，1M/10M：u64 random 18.3/27.3 → 15.1/23.1 ns、u64 random_wide 18.7/25.9 → 14.8/21.7、u128 random_wide 35.0/51.5 → 26.7/39.7；u128 narrow random 不调用 msd 叶子，不变；zipf13/sparse8/two_values/p95zero 全在噪声内。单次排序指令（1M）：u64 random −14.7%，u32 random（全平坦，max 累加被 DCE）±0.0%，u64 zipf13 +0.6%（fallback 记 max 的代价）。u64 random 1M 的 256 个 tile 全部走 insertion。Miri 7/7 |
| 66 | 加宽循环越过最后一个变化位 | 修正 | 9-bit span、采样集中的节点先取 8 bit，再按 +2 加宽到 10：shift = top - radix 下溢，partition_digit 的 assert!(shift < bits) 触发 panic（不是内存错误）。公开 sort 可达：100 万条落在同一字节内 3 个值的记录。现在 radix = (radix + 2).min(top)；回归测试用 16k 条 0..=511 的三值输入 + 1 KiB tile（Miri 下同形）|
| 67 | 共享值等值切分：采样找众数，<、=、> 三段，= 段直接完成（P2）| 保留 | 1M 中位数（p1 → 本项）：u64 two_values 8.5 → 6.6、p95zero 16.5 → 6.0、u128 two_values 19.4 → 11.5、p95zero 31.2 → 11.2 ns；10M：u64 10.0 → 7.1、15.8 → 6.6，u128 18.0 → 12.0、29.9 → 12.0。zipf13 首层削掉 32%（模式值占比）后回到 radix，约 −5~10%；sparse8/random 不动。**采样教训**：单步长众数采样与 sparse8 的周期 8 混叠，假 25% 众数让 10M +40%；改成 8 段 × 连续 8 键后消失（见 §C.2）。实现：block 引擎新增 `partition_by`（任意 classify 闭包、稳定）；等值段边界由两端内扫得到，同一次扫描顺带算出两个子节点的 variation |
| 68 | 少数离群值：采样发现顶端位的少数派，把一端的短 run 抬出去（P3）| 保留 | 1M 中位数（p4 → 本项）：u64 random_sorted99 21.35 → 16.16、random_sorted95 21.95 → 16.11（1.32–1.36x）；random 15.14 → 14.71、zipf13 17.21 → 16.15、two_values/p95zero/sparse8 不动。对照：同样的输入去掉两个哨兵键后是 13.8–14.8，剩下的 ~2 ns 是抬出前的确认扫描。10M 上这个形状消失（前 12 位已把记录分到 10 个桶），21.67 vs 22.47 持平。机制：`sampled_minority` 判顶端位是否只被 ≤1/8 的样本持有；`outlier_run` 确认它只出现在一端的一小段（≤1024）并顺带算出主体 variation（散落的少数派在第一次不一致处退出）；`lift_outlier` 把 run 旋到该在的一端，主体递归，run 插入排序 |
| 69 | 流式 compact / 双缓冲（P5）| 拒绝 | perf 当前 build 1M（含 harness 校验排序的周期）：memmove 占 u64 random 9.85%、zipf13 11.10%、sorted99 14.46%；“对根节点省一次”最多把这部分砍半 → 5–7%，低于 1.2x 门槛。真正能省的是整棵树在两个缓冲间乒乓（每层 1R+1W 而非 2R+2W），但那要求 MSD 递归接受“数据在 scratch 里”，是引擎重构不是路由改动 |
| 70 | 近似有序：一趟插入排序，按一个 tile 的键副本定价、按测得的移动率给预算（B19/s70）| 保留 | 独立程序（只排序，无 harness）1M u64：相邻对换 1% 8.89 → 1.05 ns/rec；同样 1% 但远距离对换 10.15 → 9.39（试算超预算，回退）；random 13.43 → 12.95。radix_ab 1M 中位数（s68 → s70）：u64 sorted99 15.37 → 1.36 / u64 sorted95 15.86 → 1.20 / u128 18.75 → 2.95 / u32 8.03 → 0.99；10M：u64 21.67 → 1.65、u128 27.30 → 3.04。其余 cell（random/zipf13/sparse8/mixed_runs/p95zero/runs64/bands8）在噪声内。机制：`probe_order` 判无序后，在**一个 tile 的键副本**上试算插入趟的移动数（预算 1 次移动/记录 = 一趟 radix 的写量）；失败只损失一个 tile 的读，数组一字节未动；成功则以 moves×n/tile 为全数组预算跑记录版插入排序。停下时只发生过交换 → 数组仍是原输入的排列，radix 照常接手；插入排序稳定，两路都稳定 |
| 71 | Dovetail 式多众数切分：采样出最多 8 个重键，各自给一个桶，轻键同一趟按 digit 分桶（灵感：DovetailSort 的 heavy keys）| **否决（代码已删除）** | 1M u64 中位数：two_values 5.5 → 13.0、zipf13 17.2 → 51.8、sparse8 7.0 → 15.1、p95zero 6.0 → 12.9 ns；random_wide 14.3 → 14.7（中性）。根因两条：(1) 在 block scatter 每条记录多做 rank/分类，实测每次 3–7 ns，直接抵消退休重复键的收益；(2) zipf 根节点只退休约 58%，剩余 42% 仍要分层，且轻键的 top digit 在区间内不分散，反而多一层。Dovetail 用 O(n) 暂存 + merge 支撑它的分桶/合并，搬到 O(√n) 块引擎只剩分类成本。portable SIMD 8 lane rank 在该形状也比标量 partition_point 慢（100k two_values：31.8 → 14.75 ns），说明逐记录 SIMD 比较没有预加载 lanes 不划算。 |
| 72 | 用 block 引擎返回的桶记录边界，并让子节点 variation 扫描在父节点候选 mask 全部出现过之后提前停止；等值切分同样直接用桶边界 | 保留（默认开启） | A/B（3 轮 × 7 次，1M u64，中位数）：random_wide 13.90 → 12.78、sparse8 8.47 → 7.64、zipf13 16.52 → 15.57 ns/记录；two_values ~持平。record16 26.97 → 25.02、record128 167.29 → 161.67 ns。思想来自 block 置换引擎（IPS²Ra/radsort）本来就持有桶边界：旧代码为了找边界又扫一遍所有 key；现在边界由 partition_digit_with_ends 返回，variation 只在候选位尚未全部出现时继续扫。父节点 split digit 在子节点内恒定，所以候选 mask = 父 varying 去掉 split mask。等值切分的左/右段也由 partition_by 的 ends 直接定位，少一次边界扫描。 |
| 73 | tile 内 MSD 叶子的大桶也用父 variation 的候选 mask 提前停止扫描 | 保留 | 1M u64（3 轮 × 7 次，中位数）：zipf13 16.16 → 15.67、p95zero 6.06 → 5.90 ns/记录；random_wide/two_values 持平。record16 zipf 30.3 → 约 27、record16 random 24.5 → 23.8；record128 random 持平，zipf 约 −2%（噪声内）。机制同 §72：tile 的 MSD digit 在桶内恒定，桶的候选 mask = tile varying 去掉 digit mask；旧代码对每个超过插入阈值的桶调用完整 varying_key_bits。 |
| 74 | 众数切分阈值从 1/4 放宽到 1/3、1/6、1/8（B18） | 否决 | 1M（5 轮中位数，RADIX_MODE_SHARE）：u64 sparse8 8.03 @1/4 → 6.84 @1/6，但 zipf13 15.54 → 21.38；1/8 把 sparse8 推到 19.04。u128/record16 同样出现 sparse 改善、zipf 退化的交叉。没有单一阈值同时赢，保留 1/4。 |
| 75 | 宽记录叶子的间接 tile 排序：key + u16 index 排好后 gather 一次（B09） | 否决 | 1M（7 轮中位数，强制 RADIX_INDIRECT=1）：record128 random 126.63 → 160.82 ns，zipf 170.58 → 170.71（持平）。16B pair 的 6 趟 counting pass 加 gather 仍不如现有 MSD 叶子；记录宽度不是唯一成本，pass 数与随机 gather 把收益吃掉。 |
| 76 | 跨节点复用 block workspace（B06） | 否决（噪声内） | 1M 三周期交替（5 轮中位数）：u64 random 13.02 → 13.13、u128 random 23.11 → 24.13、u128 zipf 23.97 → 24.51；record128 zipf/sparse 约 −2%；u64 sparse 7.36 → 6.54 但一轮 5.26 离群。整体方向不一致，保留每节点分配。 |
| 77 | 等值切分后常数子节点的 all-equal 快路径 | 否决 | 1M 三周期交替（5 轮中位数）：u64 sparse8 6.48 → 5.74、record16 sparse 16.04 → 14.76、record16 p95 9.51 → 8.95；但 u64 zipf13 13.67 → 15.13、record16 zipf 24.97 → 25.71，random/two 持平。低基数但非常数的子节点会被预读一路扫到首个差异，收益与损失交叉，不保留。 |
| 78 | 把 leaf 排序挂到 block compact 的桶完成点：每个 tile 桶物化到 input 后立即 scan_variation + lsd_leaf | 保留 | 1M（5 周期 × 7 次，中位数，RADIX_FUSE_LEAF A/B）：u64 random 12.25 → 11.89、u128 random 23.21 → 21.77、record16 random 21.85 → 19.42；u64 zipf 13.91 → 14.02、u128 zipf 23.99 → 24.03、record16 zipf 24.77 → 25.66；record128 random 122.39 → 123.45、zipf 168.92 → 170.11。随机 cell 稳定获益，zipf 小幅回退，宽记录持平；净收益为正，保留。这是 compact 后的缓存融合，不改变 leaf 内部 pass 结构，也没有处理 E1 的 flat 奇偶性。 |
| 79 | 等值切分的 less/greater 桶也挂到 compact 完成点；equal band 跳过 | 保留 | 1M 三周期 × 5（digit fusion 固定，RADIX_FUSE_EQ A/B）：u64 two_values 5.43 → 4.96、p95zero 5.15 → 4.94；u128 two_values 10.67 → 9.63、zipf13 23.35 → 20.72；u64 zipf13 13.47 → 13.63、record16 two_values 9.04 → 9.23。无 equality 的 cell 差异是噪声；重复键 cell 的净收益更大。 |

另有一次"拨乱反正"：§30 和 §36 里原有的 "(Superseded)" 段落（被推翻的旧结论、旧数字）在本次压缩中删除，
对应的最终结论已写入上表。

---

## C. 方法教训（复盘时先读这一节）

### C.1 测量

- **噪声底**：同一份代码同一进程内交错测，cell 之间差 5–6%；跨 build 或负载下 10–20% 都见过。5% 以内不是结果，
  决策只看 ≥1.2x。单个 <10 ms 的 cell 需要静音机器 + 至少 5 轮才有投票资格（§26）；有人用 15 轮把"7 轮下
  u32 random +39% 回归"变成了 −12% 收益（§30）。
- **用指令数（`perf stat`）判定"行为是否变了"，用墙钟判定"是否变快了"，两者必须同时看**。只看一个都出过错：
  §49 墙钟 +16.7% 而指令完全相同；§59/§61 指令降而墙钟升。
- **跨 build 对比必须固定数据**：`OVERRIDE_SEED=1`。§54 因此作出了错误的否决（§55 发现、§56 纠正）。
- 流程：`taskset` 绑核；5–7 轮交错；报**中位数**，min 单列；跑前 `uptime`；后台任务（尤其 criterion）
  先 `ps`。`RADIX_RAW=raw_runs.csv` 存档每一轮（commit、类型、pattern、n、variant、round、秒）。
- 路由类改动必须 100k / 1M / 10M 三档都查，再用 `perf stat` 确认（§31）。
- **路由决策是否改变，用 `RADIX_TRACE=1` 的逐节点 trace 对比**，比计时确定得多（§28、§51、§55、§56）。
- 周期数增量不是证据（§36）；隔离出一个阶段的 microbench 在该阶段是"链式依赖"而非"带宽"时会把价格估反。
- **最便宜的 cell（<2 ns/记录）不要用 `radix_ab` 的跨 build 中位数判定**：u32 ascending 1M 在两次 build 里稳定地读到 1.4 和 2.0 ns，而两个 build 的指令数相同，`examples/sort_time.rs`（同一段计时程序，只换库源码）也测不出差异；把 harness 的预分配照搬到专用程序里同样复现不了。原因未查明，但这类 cell 改用专用程序 + 指令数判定，并把冲突写进账本（§64）。

### C.2 harness 曾经说谎的地方（影响过结论）

| 位置 | 问题 | 影响 | § |
| --- | --- | --- | --- |
| u128 非 `random_wide` 行 | 32 位 key 装 128 位容器 | 一批 u128 结论重做 | 20 |
| u64/u128 "full-width random" | 实为 32 bit 熵，10M 带 ~11k 重复 | 重测；u128 反而更好 | 35 |
| `type_top_byte` u32/u64 | 全等值键 | 该行结论作废 | 52 |
| `sparse8` | 只用高 32 bit | §40 的机制解释错，结果仍对 | 52 |
| record16/record128 payload | 恒为 0 | 稳定性回归看不见（测试里有 payload identity 补）| 22 |
| ≤8 MiB cell | clone 刚完成，热缓存；10M 因 memcpy 切非时序存储，起点缓存状态不连续 | 1M→10M 的缩放结论要带着这个保留 | 22 |
| 单轮 min | 一个异常轮次污染 | 改中位数 | 12/52 |
| `--filter` 无种子 | 两 build 数据不同 | §54 作废 | 55 |
| `p95zero`（95% 等值 + 随机尾）| 等值前缀让"重排率"为 0，全数组扫描去验证它要一整天趟读 | s70 第二版按 tile 定价（moves=0）后仍整数组试跑，扫过 950k 等值记录 = +0.9 ns/记录。零率必须直接停（数据说"前一个 tile 没有错位"时，去后面找错位就是拿一整趟读赌）| 70 |
| 采样/试算型机制 | 在数组上试跑会改变后续按位置采样的决策 | s70 第一版把 tile 就地插入排序来定价；失败时留下部分有序前缀，zipf13 第二层的众数采样被假 run 骗到，多出一层。改成在**键副本**上试算（移动数只取决于键序列），数组不动 | 70 |
| 一棵有几千个 split 节点的树（zipf13）| 每个 split 节点都付 64 键采样的固定成本 | 固定成本必须按节点大小摊开：采样门槛 = 4 个 tile；否则 4096 个小节点各付一次，1M 上 +10% | 68 |
| `sparse8`（周期 8 的重复）| 单步长众数采样与 8 的周期混叠，12.5% 的值在样本里占 25% | §67 第一版 10M +40%；8×8 连续段采样后消失 | 67 |
| `random_sorted99`（u64/record16/record128）| `narrow_u64` 把升序键整体抬到 2^62 附近，`x=0,1` 两个键落在 2^62 下方 | 顶层变化位跨度 51 → 63：12-bit split 后 2 条落进第一个桶、999998 条落进第二个，第二层才真正开始分。这一整层是生成器的哨兵值买来的，写这个 cell 的结论时要扣掉 | — |
| "7 targets green" | 数 ok 行，看不到编译失败 | lib 目标 3 提交没编译 | 44 |

### C.3 设计层面的稳定结论

1. **本设计在这些规模下，增加每记录工作集压力的机制不会赚钱，不论省多少指令**。证据：§4（融合直方图）、
   §18（预取）、§50（融合聚合）、§59（窄表）、§60（rank 压缩）、§61（融合直方图 v2）、§14（宽 fan-out）。
   这七条都是同一个现象。
2. 反过来，**少一遍内存遍历**才是有效杠杆：§23（tile 内 MSD 层）、§41（还需 ≥2 遍就 split）、§13（窗口打包）。
3. **不要把"路由"与"叶子"的价格混为一谈**：§21、§29 各因此出过 25% 回归。
4. **形状识别要用代价单位，不能只数"看起来乱不乱"**：§70 的判据是插入趟的移动数（在键副本上试算），所以"1% 相邻对换"赢 8.5x 而"1% 远距离对换"一步不赢——同一个下降计数、同一个逆序数，代价差两个数量级。
4. **机制只留一个**：在噪声内的路径删掉（§25 merge、§29 StreamMode、§53 成本模型），即使它曾经测得更快。
5. 简化后要在**非主流 cell** 上也复核（§58：§53 只查了 6 个主流 cell，漏了 random_local）。
6. 每个决策读的应是**计数**而不是速率（§32、§41、§51），这样移植到别的核不用重调。

---

## D. 空间、验证、复现

### D.1 空间

- split：`2 * buckets` 块，每块 `default_block_size`，与基线相同的 O(√n) 块 scratch，逐层瞬时。
- 叶子：一块 ≤ `TILE_BYTES`（64 KiB）的缓冲，所有叶子复用。取 `min(tile, len)`，所以 100 条 u64 只用 800 B。
- 分配器可见峰值（`bench extra_memory`）：u64 10k 17.6 KB；u64 1M 839 KB；u64 10M 2.6 MB（80 MiB 数据）；
  u128 10M 3.7 MB，与 block LSD 在 10M 持平。

### D.2 验证

- `cargo test --release --features rust_tessera_radix` 共 **7 个目标**，**必须含 lib 并逐个点名**（§44）：
  lib 10、block_lsd 8、block_lsd_functionality 233、main 233、radix_owned 2、radix_views 6、tessera_functionality 233。
- 随机差分测试：2000 种形状 × 5 种 tile 预算，与 `std` 稳定排序逐记录比较，payload 带输入下标。
- panic 扫描：每个 key 调用点注入 panic，断言记录集合与 drop 日志（§19）。
- Miri：`cargo +nightly miri test --features rust_tessera_radix --lib other::rust_tessera_radix` → 7/7，216 s（§38）。
- ASan：曾运行过，但没有留存日志，不作为证据；需要时重跑。

### D.3 复现

```sh
# 测试
cargo test --release --features rust_tessera_radix

# 交错锦标赛（本文所有表格的来源）
RADIX_RAW=raw_runs.csv OVERRIDE_SEED=1 taskset -c 2 \
cargo bench --features rust_block_lsd,rust_tessera_radix,rust_msd_radix \
    --bench radix_ab -- --types u32,u64,u128 --patterns random_wide,zipf13 \
    --sizes 100000,1000000,10000000 --rounds 7

# 钩子（出厂运行均不设）
RADIX_TRACE=1            # 每个路由节点一行决策日志
RADIX_AB_LEAVES=1        # 加入 tile / leaf 扫描变体
RADIX_LEAF_WIDTH=n  RADIX_SPLIT_WIDTH=n  RADIX_BLOCK_SCALE=x

# 仓库标准 criterion（与仓库已发布基线可比，不与上面的表可比：u128 行是 i32 放大的窄 key）
BENCH_REGEX='^(rust_(tessera|block_lsd)_stable_radix|rust_ipnsort_unstable|rust_std_stable)-hot-(u32|u64|u128|f64)-(random|random_z1|ascending)-(100000|1000000|10000000)$' \
cargo bench --features rust_block_lsd,rust_tessera_radix,cold_benchmarks,bench_type_u128,bench_type_f64 \
    --bench bench -- --warm-up-time 2 --measurement-time 4 --noplot --save-baseline tessera
```

机器：Xeon Gold 6140（Skylake-SP）2.3 GHz，6 vCPU 共享 VM，L1d 32 KiB / L2 1 MiB / L3 24.8 MiB，nightly 1.101.0。

### D.4 归档数据

只保留 `raw_runs.csv`（每一轮原始计时：commit, 类型, pattern, n, variant, round, 秒）。
扫描日志（`logs/`）已清理，不再入库；本文表格里的数字即为留存记录，需要时用 D.3 的命令重跑。

---

## E. 未决事项

完整清单和判决见 `BACKLOG.md`。这里只留与本文件直接相关的。

- **u32 10M 路由**：旧模型预测"不 split"而 split 快 12%。路由已在 §41 改规则，此问题**未重测**，可能已过时。
- **split 的物化遍（~2 ns/记录）**：计数优先 + 精确边界可去掉 8–13%，代价是块级原地置换（§36 的教训：环遍历慢）。
  即 `BACKLOG.md` 的 E1。
- **u128 与 ipnsort 的差距**：比较排序在寄存器里比 16 字节，不是我们能追的。
- `sort_by` 两个 radix 模块都不支持（显式拒绝）。
- 已关闭：叶子 flat plan 是否值得保留（§39、§53：值得，但只在低基数 tile 上）。

---

## F. 外部整数排序对照（本轮新增）

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

5 轮交错，中位数，ns/记录。机器同 §D.3；共享 VM，比例只看量级。**本节表格只到 1M。**

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

- **VQSort 是完整的 SIMD 上限**：1M `u64` random 比 Tessera 快约 1.22x，zipf13 快约 3.8x，
  two_values 快约 3.9x。100k `u128` random 上 Tessera 略快（25.6 vs 28.0），zipf13 上 VQSort 仍快 1.9x。
- **IPS²Ra 不是“低空间就一定更快”**：本机串行配置在 1M `u64` random 比 Tessera 慢约 1.9x；
  record128 random 也慢 1.16x。它的固定桶缓冲和串行分类成本在 1M 没有摊薄。
- **Dovetail 的稳定性有明确价格**：O(n) 暂存 + 合并，u64 1M zipf13 为 47.0 vs Tessera 18.5；
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
# 只跑本轮对照，1M 上限；--variants 精确匹配，避免 10000 前缀误中 10000000
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
