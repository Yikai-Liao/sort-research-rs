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
| `random_sorted99`（u64/record16/record128）| `narrow_u64` 把升序键整体抬到 2^62 附近，`x=0,1` 两个键落在 2^62 下方 | 顶层变化位跨度 51 → 63：12-bit split 后 2 条落进第一个桶、999998 条落进第二个，第二层才真正开始分。这一整层是生成器的哨兵值买来的，写这个 cell 的结论时要扣掉 | — |
| "7 targets green" | 数 ok 行，看不到编译失败 | lib 目标 3 提交没编译 | 44 |

### C.3 设计层面的稳定结论

1. **本设计在这些规模下，增加每记录工作集压力的机制不会赚钱，不论省多少指令**。证据：§4（融合直方图）、
   §18（预取）、§50（融合聚合）、§59（窄表）、§60（rank 压缩）、§61（融合直方图 v2）、§14（宽 fan-out）。
   这七条都是同一个现象。
2. 反过来，**少一遍内存遍历**才是有效杠杆：§23（tile 内 MSD 层）、§41（还需 ≥2 遍就 split）、§13（窗口打包）。
3. **不要把"路由"与"叶子"的价格混为一谈**：§21、§29 各因此出过 25% 回归。
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
