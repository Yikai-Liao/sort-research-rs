"""Print the historical random tables from the preserved JSON medians."""
import json
from refresh import ROOT, SOURCE, WARNING, random_table
benchmarks = json.loads(SOURCE.read_text())["benchmarks"]
print(WARNING)
for temp in ("cold", "hot"):
    print(f"\n### {temp}\n")
    print(random_table(benchmarks, temp))
