# Tessera 与独立 MSD：安全修复及性能验收

基线为 `2aa312d1cda9359e0b187bf1dde893baf219b564`，其排序源码与上一轮冻结提交一致。本目录的源码哈希、测试和计时独立归档；旧性能矩阵不用于验证新源码。数据见[逐轮计时](performance-rounds.csv.gz)、[汇总](performance-summary.csv)、[配置、源码与二进制哈希](performance-manifest.json)。

复现日志、基线源码、bench 接线补丁、失败候选的源码快照和复现用例在本地证据包 `logs/tessera-safety-20261105-evidence.tar.gz`（23,305,617 B，sha256 `1383397713e09dbe5595ab927b8c657ea8ddb59ccf021a2b74b15b75820d3399`，858 个文件；逐文件哈希见包内 `evidence-files.json`，包哈希见同目录 `-evidence.json`）。

## 安全边界与实现

`Key` 继续是安全 trait。确定性是排序正确性的前提；违反确定性时可以产生错误结果或 panic，但不能造成未初始化读取、越界或资源重复释放。正常确定性 key 的排序稳定性和 panic 后记录保留要求不变。

私有 `RecordType` 只通过 Rust 的真实 `Copy` 能力分派，不使用 `needs_drop` 推断，也不维护一份“可信 key 类型”名单。无 Drop 的独占引用记录仍走非 Copy 保护。生产代码只有一份 counting 内核、一份 MSD 置换循环；`Sorter` 没有新增字段，缓存与否由 `counting_pass::<T, CACHED>` 的 const 参数选择，`pass` 按同一参数单态化。

Copy 路径：`Sorter.scratch` 的前缀一旦被一整趟成功写入就保持有效。第一次遇到更大的 tile 时，首趟使用与直方图同一份 u16 分类做 scatter，把所有输出槽写满，并把 `scratch.len` 记为 tile 长度；之后的趟和 tile 直接复用该前缀。因此 scatter 的 destination 从第一趟起就持有有效 Copy 值，不确定 key 即使重复写入，也不会留下未初始化的记录；写入下标仍限制在整个 destination 分配内。此时错误 key 不保证输出值构成原始排列。正常 key 的记录搬运次数与基线相同：

| 路径 | 基线完整记录搬运 | 修复版 |
| --- | ---: | ---: |
| tile MSD | scatter + copy back，2 遍 | scatter + copy back，2 遍 |
| 奇数 p 趟 tile LSD | p + 1 遍 | p + 1 遍 |
| 偶数 p 趟 tile LSD | p 遍 | p 遍 |

首趟每个 tile 大小额外写一次 u16 分类；分类表优先复用现有 32 KiB cursor 表的未使用后缀，两者都放不下时使用同一个 scratch 分配的尾部，不增加第二次分配或缓存清零。尾部按 u16 对齐，并与有效记录区分开。

非 Copy 路径在 histogram 时缓存每条记录的 u16 分类，scatter 不再调用用户 key，所以两阶段必然使用相同分类。各桶区间互不重叠，每个输出槽恰好初始化一次。

`RestoreTile` 仍只持有 tile、len、source。tile MSD 与 LSD 共用 pass 成功后的来源切换协议，并在 key panic 时恢复当前有效来源；LSD 每趟只判断一次是否需要分类，scatter 循环内没有这个决策。

独立 MSD 的 `PermutationHole` 保管循环中取出的记录，在正常完成或 unwind 时填回原槽，保留最后一次 key 调用对 payload 的修改。非 Copy 游标必须留在目标桶内，避免读取原始空槽；Copy 可读取仍有效的旧副本，但同样在读取前检查整个 input 的边界。私有 digit mask 已保证桶号在 0..256 内，因此删除了冗余的 cursor 数组边界检查，保留必要的记录边界检查，它同时保证非确定 key 下的置换循环终止。

## 回归与验证

未修复源码上，Miri 已分别复现 Tessera 未初始化读取、MSD key panic 后访问已释放的 Box、MSD 非确定 key 越过整个分配边界。原始日志与复现用例（`repro-tessera.log`、`repro-msd.log`、`repro-msd-changing.log`、`repro-tests.rs`）在本地证据包中。

新增回归覆盖公开安全入口、Copy/非 Copy、奇偶 LSD 趟数、tile MSD、正常 key panic 和非确定 key。测试读取实际恢复出的 payload、核对 ID、最后一次 key 的内部修改和逐条析构；无 Drop 的独占引用在恢复后实际写回原槽。额外覆盖单字节非 Copy 记录的奇数长度 scratch 尾部对齐、Copy scratch 复用后只需一次分类，以及 digit 规划的 22 窗口上界。

最终验证固定 CPU 4，nightly `rustc 1.101.0-nightly (a30aa9064 2026-10-08)`：release lib 24 项、五个集成 suite 共 495 项（233 + 3 + 13 + 13 + 233）、ASan 53 项（24 + 3 + 13 + 13）、Miri 24 项（安全入口 13、views 6、owned 1、lib 单测 4）。十一条命令退出码均为 0；命令、耗时和日志哈希见 [validation.json](validation.json)。

## 性能门槛与取舍

性能按工况验收，稳定回退不由其他输入的收益抵消。矩阵同时包含整数、仓库内 Copy records、自定义 Copy wrapper，以及持有 Box 的非 Copy records（8 B key + Box + 16/128 B padding，记录为 32/144 B）。自定义 wrapper 只转发确定性 key，避免只验证内置类型。

计时使用同一 ELF 中的两条基线臂和两条候选臂，按 A1、B1、A2、B2 轮换，确保两版代码的冷热条件对称；ELF 的组装、基线源码和补丁在本地证据包的 `bench-setup.md`。A1/A2 共用基线函数，B1/B2 共用候选函数。输入、克隆及独立参考排序不计时，分配与正常保护成本计时。每轮计算 mean(B1,B2)/mean(A1,A2)，先取 block 中位数，再取五个 block 的中位数；A/A 为对应的 A2/A1。45 个工况、5 个 block、225 次调用，轮数按规模 11–5001。

A/A 对照中位数 1.0000（0.980–1.040）。同一二进制连跑两遍完整矩阵，逐工况的 |Δ| 中位 0.007、p90 0.021、最大 0.056（record128 zipf13），这是该矩阵的实际复现误差，高于它的单点差异不应当作算法结论。结果按组：

| 组 | 工况数 | 中位 | 范围 |
| --- | ---: | ---: | ---: |
| Tessera 全部 | 24 | 0.972 | 0.852–1.035 |
| Tessera 自定义 Copy wrapper | 10 | 0.968 | 0.860–0.992 |
| 独立 MSD | 8 | 0.998 | 0.993–1.024 |
| 独立 MSD 自定义 Copy wrapper | 3 | 0.992 | 0.989–1.015 |
| 非 Copy owned（Tessera，32/144 B，含 Box） | 4 | 0.974 | 0.941–0.976 |

Copy 路径没有稳定回退：Tessera 的 15 个整数与仓库 record 随机工况为 0.866–1.009，10 个自定义 Copy wrapper 为 0.860–0.992，独立 MSD 随机工况为 0.993–1.009。进入 scatter 的工况中高于 1.01 的只有两个宽记录 cell：record128 zipf13 100k 1.016 与独立 MSD 的 record128 自定义 wrapper 1.015；前者在同一二进制的另一遍完整运行是 0.960，后者是 1.001。非 Copy 记录在 4096 和 100k 上为基线的 0.941–0.976，与 Copy 路径共用同一套遍数和 buffering，只多一次 u16 分类读写、少一次用户 key 调用。

两个不进入 scatter 的预排序形状（u64 ascending / descending 1M）只做一次顺序探测或原地反转：31 轮的退休指令数 A/B 分别为 296.05M/295.84M 和 1034.66M/1034.71M（差 <0.02%），它们的配对比值在不同 build 之间从 0.814 波动到 1.110，反映同一源码两处实例的代码放置，不作为算法结论。

已淘汰的实现不会随源码提交：

- 运行时 `cache: bool` + 未内联的 `pass`：u32 random_wide 1M 稳定 1.055（五个 block 全部大于 1）。两条 scatter 循环被塞进同一个函数，改为 const 泛型并在调用点分支后回到 1.002。
- `largest` 留在前缀和循环：u64 random_wide 4096 1.072、record128 random_wide 4096 1.020（当前分别为 0.971 / 0.972）。tile 小、digit 宽时桶表约为记录数的四倍，把最大值累加放到直方图循环更便宜。
- 每桶结束边界的整趟末尾校验：u64 random 4096 的短筛约 1.461，A/A 约 1.005。
- 对所有泛型记录统一保留分类缓存的整合候选：同进程筛选仍显示小规模自定义 Copy record 的回退。
- 不对称三臂计时与过短的分进程计时保留为筛选资料，不作为最终性能结论。

代码量：生产代码 `+262/−80` 行（`radix_support` 15/1、block core 10/7、MSD 52/12、Tessera engine 185/60），测试 `+140/−1` 行并新增 517 行安全回归。没有按记录类型复制排序实现。代码生成相关的 `MAX_DIGITS` 48→22、两处 `max(1)` 删除、位宽表达替换和 `tile_bytes` 字段别名删除暂不落地；22 是规划窗口的有效上界，但不能仅凭语义等价推断这些清理不会影响性能。本轮保留 `Visit` 别名、准确的注释、饱和字节算术和 `MAX_DIGIT_BUCKETS` 命名。

## 历史报告口径

上一轮 digit 分区已统一在桶物化完成时处理 tile，取消 ≤64 B 记录门控；equality 分区在 compact 后处理两侧。上一轮 1537→1220 NLOC 对应约 20.6%，不代表本轮源码规模。

Probe、unchecked 采样索引、try_fold、共用插入核、compact 回调与事后循环两阶段收尾均保留。模块文档补充每层最多 32 KiB cursor 表的栈预算；确定性 u128 key 的保守递归深度为 22。共享堆表和窄跨度 radix=span 两项候选未并入修复。
