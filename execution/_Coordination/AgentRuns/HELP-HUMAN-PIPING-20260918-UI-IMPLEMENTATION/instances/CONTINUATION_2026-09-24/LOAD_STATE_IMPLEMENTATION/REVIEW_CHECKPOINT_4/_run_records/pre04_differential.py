"""Differential pre-0.4 check: run the value-route example over committed
pre-0.4 requests and invented stress variants (extra declared tip loads, the
P12 composite failure), in both modes, and hash stdout/stderr/exit.
usage: r4_pre04_differential.py <examples dir> <out.json>   (cwd = WORKING_ROOT)"""
import copy, hashlib, json, pathlib, subprocess, sys, tempfile
ex = pathlib.Path(sys.argv[1]); out = pathlib.Path(sys.argv[2])
root = pathlib.Path("fixtures/product_preview")
inputs = {}
for d in ["physics_source", "source_blocks", "load_reference", "load_reference_source"]:
    for p in sorted((root / d).glob("*.request.json")):
        inputs[f"{d}/{p.name}"] = json.loads(p.read_text())
def extra(req, n):
    r = copy.deepcopy(req)
    for c in r["model"]["load_cases"][:1]:
        base = c["primitive_loads"][0]
        for i, dir_ in enumerate(["UZ", "UY", "RY"][:n]):
            l = copy.deepcopy(base); l["id"] = f"r4extra:{i}"; l["direction"] = dir_
            l["provenance"] = "invented_cp4_review_probe_not_library_data"
            if dir_.startswith("R"):
                l["category"], l["dimension"], l["magnitude"] = "concentrated_moment", "moment", {"value": 1e-6, "unit": "N*m"}
            else:
                l["category"], l["dimension"], l["magnitude"] = "concentrated_force", "force", {"value": 1e-6, "unit": "N"}
            node = [n["id"] for n in r["model"]["nodes"]][-1]
            l["target"] = {"type": "node", "node": node}
            c["primitive_loads"].append(l)
            if "analysis_state" in c:
                c["analysis_state"]["load_sources"].append({"source_ref": l["id"], "factor": 1.0})
    return r
for name, req in list(inputs.items()):
    if name.startswith(("physics_source/", "source_blocks/")) and req["model"]["load_cases"][0].get("primitive_loads"):
        for n in (1, 2, 3):
            inputs[f"{name}+extra{n}"] = extra(req, n)
p12 = copy.deepcopy(inputs["physics_source/mixed.request.json"])
p12["model"]["load_cases"][1].pop("modulus_basis_ref", None)
inputs["physics_source/mixed.request.json+P12"] = p12
results = {}
with tempfile.TemporaryDirectory() as tmp:
    for name, req in inputs.items():
        path = pathlib.Path(tmp) / "req.json"; path.write_text(json.dumps(req))
        for mode in ("sparse", "dense"):
            run = subprocess.run([ex / "physics_source_connected", mode, path], capture_output=True)
            label = None
            try: label = json.loads(run.stdout)["producer"]["semantic_contract_id"].split("/")[-1]
            except Exception: pass
            results[f"{name}|{mode}"] = {"exit": run.returncode, "label": label,
                "stdout_sha256": hashlib.sha256(run.stdout).hexdigest(), "stderr": run.stderr.decode()[-200:]}
out.write_text(json.dumps(results, indent=1, sort_keys=True))
print(len(results), "runs")
