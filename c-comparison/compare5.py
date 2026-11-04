
import subprocess
BASE = "/root/code/sort-research-rs-c-compare"
EXES = {
    "gcc":   (BASE + "/c-compare/driver_dyn", True),
    "clang": (BASE + "/c-compare/driver_dyn_clang", True),
    "rust":  (BASE + "/target/release/rust-compare", False),
}

def sample(key, n, reps=7):
    exe, is_c = EXES[key]
    args = ["taskset", "-c", "2", exe, str(n), str(reps)] + (["policy"] if is_c else ["auto"])
    out = subprocess.run(args, capture_output=True, text=True, check=True).stdout.strip()
    if is_c:
        return float(out.split(",")[4])
    for line in out.splitlines():
        p = line.split(",")
        if p[0] == "auto":
            return float(p[3])
    raise SystemExit("no auto")

sizes = [100_000, 1_000_000]
best = {}
for n in sizes:
    for _ in range(4):
        for k in EXES:
            v = sample(k, n)
            best[k, n] = min(best.get((k, n), 1e18), v)
print(f"{'n':>9} | " + " ".join(f"{k:>8}" for k in EXES) + " | rust/gcc rust/clang")
for n in sizes:
    g, c, r = (best[k, n] for k in ("gcc", "clang", "rust"))
    print(f"{n:>9} | {g:8.2f} {c:8.2f} {r:8.2f} | {r/g:8.3f} {r/c:10.3f}")
