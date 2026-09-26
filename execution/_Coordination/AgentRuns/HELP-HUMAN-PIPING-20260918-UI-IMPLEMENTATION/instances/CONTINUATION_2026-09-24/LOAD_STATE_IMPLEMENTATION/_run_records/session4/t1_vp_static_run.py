"""T1 VP-STATIC development comparison (manager lane, after T1_WAVE1_REVIEW_A).

Writes the reader binding and one run-selection record per solver mode into
<scratch>, then runs tools/validation/qualification_load_reference.py for each
mode against the admitted package validation/qualification/fixtures/load_reference/
MANIFEST.json, using the explicitly built runner executable, a 300 s process limit and the
adapter's maximum 64 MiB output limit (two cases exceed the 8 MiB default). Purpose is
development_comparison. Nothing here edits a fixture, reference or criterion.

usage: python t1_vp_static_run.py <WORKING_ROOT> <runner executable> <scratch dir> <review path>...
Review paths are WORKING_ROOT-relative independent reviews of the reader module.
"""
import hashlib, json, pathlib, subprocess, sys
root = pathlib.Path(sys.argv[1]).resolve()
executable = pathlib.Path(sys.argv[2]).resolve()
scratch = pathlib.Path(sys.argv[3]).resolve()
reviews = sys.argv[4:]
sys.path.insert(0, str(root / "tools/validation"))
import qualification_load_reference as lr
def sha(path): return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()
scratch.mkdir(parents=True, exist_ok=False)
binding = {"format": lr.BINDING_FORMAT, "contract_id": lr.CONTRACT, "entrypoint": lr.ENTRYPOINT,
           "status": "reviewed_candidate",
           "review_basis": [{"path": r, "sha256": sha(root / r)} for r in reviews],
           "files": [{"path": p, "sha256": d} for p, d in lr.DEPENDENCIES.items()]}
for row in binding["files"]:
    assert sha(root / row["path"]) == row["sha256"], row["path"]
(scratch / "reader_binding.json").write_text(json.dumps(binding, indent=2) + "\n")
commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip()
manifest = root / "validation/qualification/fixtures/load_reference/MANIFEST.json"
results = {}
for mode in lr.MODES:
    selection = {"format": lr.RUN_FORMAT, "profile_id": "t1-vp-static-load-reference-1",
                 "purpose": "development_comparison", "transport": lr.TRANSPORT,
                 "runner": {"candidate_commit": commit, "executable_sha256": sha(executable),
                            "solver_mode": mode, "explicit_local_private_intent": True},
                 "case_manifest": {"path": str(manifest), "sha256": sha(manifest)},
                 "reader_binding": {"path": "reader_binding.json", "sha256": sha(scratch / "reader_binding.json")}}
    path = scratch / f"selection-{mode}.json"
    path.write_text(json.dumps(selection, indent=2) + "\n")
    out = scratch / f"out-{mode}"
    proc = subprocess.run([sys.executable, str(root / "tools/validation/qualification_load_reference.py"),
                           "--selection", str(path), "--executable", str(executable),
                           "--source-root", str(root), "--output-dir", str(out), "--timeout-seconds", "300", "--output-limit-bytes", str(64 * 1024 * 1024)],
                          capture_output=True, text=True)
    results[mode] = {"exit": proc.returncode, "stderr_tail": proc.stderr[-2000:], "stdout_tail": proc.stdout[-2000:]}
    ledger = out / "ledger.json"
    if ledger.exists():
        data = json.loads(ledger.read_text())
        results[mode]["outcome"] = data.get("outcome")
        results[mode]["summary"] = data.get("summary")
print(json.dumps({"candidate_commit": commit, "executable_sha256": sha(executable), "results": results}, indent=1))
