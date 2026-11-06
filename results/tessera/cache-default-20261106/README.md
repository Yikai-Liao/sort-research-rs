# Copy 记录统一走桶号缓存的成本（2026-11-06）

结论用于 [独立仓库计划](../standalone-plan.md) 5.1 节的 API 决定。

问题：`stable_sort_by_key` 能否对所有记录类型默认使用桶号缓存（每趟在 histogram 时存 u16 桶号，scatter 读取，不再调用 key），而不为 Copy 记录保留不带缓存的快路径。

## 方法

- 源码：`exp/radix-opt` `6c0877f` 的独立 worktree，nightly 1.101，thin LTO，默认 bench profile。
- 四个变体编译进同一个 `radix_ab` 二进制：
  - `tessera`（A1）、`tessera_a2`（A2）：同一函数，测轮间噪声；
  - `tessera_copy`：逐字相同的引擎副本，测代码放置差异（[EXPERIMENTS.md](../../../EXPERIMENTS.md) §104 记录过对齐造成 10–24% 的差异）；
  - `tessera_cached`：引擎副本中 `T::COPY` 全部替换为 `false`，Copy 记录也每趟存取桶号。
- 修正 harness：每次计时前在计时外恢复全部缓冲（原 `--batch N>1` 不恢复输入，`--batch 1` 把复制计入耗时）。补丁见 `harness-and-arms.patch`；两份引擎副本由下列命令生成：
  ```sh
  sed '/^#\[cfg(test)\]$/{N;/mod tests;/d}' engine.rs > engine_copy.rs
  sed -e '/^#\[cfg(test)\]$/{N;/mod tests;/d}' -e 's/T::COPY/false/g' engine.rs > engine_cached.rs
  ```
- `run.sh` 中的路径指向已删除的临时构建目录，复现时改成自己的 worktree 和二进制路径。
- 矩阵：u32、u64、u128、record16、record128 × random、random_d256、zipf13 × 1K、4K、16K、64K、1M；5 个独立进程周期，CPU 2 绑核串行，`run.sh`。
- 统计：每轮 X / mean(A1, A2)，周期内取轮中位数，再取 5 周期中位数（`analyze.py`）。只有超出 A2/A1 与 copy/A 两种对照范围、且至少 4/5 周期同向的结果才算真实差异。

## 结果

完整表格见 `results.md`，原始数据 `raw.tar.zst`。cached/A 小于 1 表示统一缓存更快。

| 类型 | 统一缓存更慢的工况 | 统一缓存更快的工况 |
|---|---|---|
| u32（4 B） | random 16K–1M +3.0–4.4%；1K random_d256 / zipf13 约 +2% | random_d256 1M −3.9%（未超出对照） |
| u64（8 B） | random 64K +2.7% | 1K–4K 的 random、random_d256、zipf13 −2.1–4.0% |
| u128（16 B） | random 1M +2.0%（5/5，未超出对照） | 无 |
| record16（16 B） | random 16K–1M +0.9–3.5%（4–5/5，未超出对照） | 1K–4K 的 random_d256、zipf13 −1.9–3.4% |
| record128（128 B） | 无 | 无，全部在 ±1.4% 内 |

结论：统一缓存的成本在 ±4.5% 以内，方向随类型、分布和规模变化，没有一致的赢家；128 字节记录无差别。最大的回退出现在 u32 随机键 16K 以上，而只排键的场景在新库中走 `stable_sort`，键由库编码，可以不用缓存。8 字节 key + 行号记录（如 `{u32, u32}`）未直接测量，u32 与 u64 键数组只是近似。

放置对照 copy/A 本身最大偏离 4%（u64 random 1M 0.960），1M 规模单个工况 ±4% 以内的差异不能当作算法结论。
