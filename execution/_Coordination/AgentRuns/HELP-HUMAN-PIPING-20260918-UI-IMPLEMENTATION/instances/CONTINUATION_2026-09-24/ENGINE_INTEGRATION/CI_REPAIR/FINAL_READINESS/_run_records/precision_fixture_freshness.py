"""Compare fixtures/product_preview/precision_fixture_generation.json source hashes
with the working tree. Read-only. Run from projects/chirality-piping."""
import hashlib, json, pathlib
d = json.load(open("fixtures/product_preview/precision_fixture_generation.json"))
diff = [p for p, h in d["source_input_files"].items()
        if not pathlib.Path(p).exists() or hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest() != h]
print("recorded source inputs:", len(d["source_input_files"]), "differing now:", len(diff))
for p in diff: print("  ", p)
for k in ("recipe", "generator", "input_model"):
    p = d[k]["path"]; print(k, p, "matches" if hashlib.sha256(open(p, "rb").read()).hexdigest() == d[k]["sha256"] else "DIFFERS")
print("pressure_runtime.rs in record:", any("pressure_runtime" in p for p in d["source_input_files"]))
for f in ("fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json",):
    r = json.load(open(f)); print(f, r["producer"], r["status"]["mechanics"])
