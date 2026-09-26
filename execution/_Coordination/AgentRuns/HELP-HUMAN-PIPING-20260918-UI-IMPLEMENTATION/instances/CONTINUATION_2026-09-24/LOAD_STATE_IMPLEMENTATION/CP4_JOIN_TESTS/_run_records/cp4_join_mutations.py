"""CP4_JOIN_TESTS: the CP3 reviewer's mutants K1-K12, anchors taken verbatim
from REVIEW_CHECKPOINT_3/_run_records/review3_mutations.py (parsed, not
retyped), run against a scratch copy of core/product_physics (never the
checkout).

Each mutant runs two suites:
  prior   = the suites the reviewer ran as "committed" (lib, runtime,
            runtime extension, physics_source_runtime), skipping this TASK's
            module, i.e. what was maintained before CP4;
  join    = this TASK's module alone (lib, source_receipt::load_state_join_tests).
Bytes are restored and sha256-verified after every mutant.
usage: python3 cp4_join_mutations.py <reviewer review3_mutations.py> <scratch core/product_physics> <target dir>
"""
import ast, hashlib, json, os, pathlib, subprocess, sys

source = pathlib.Path(sys.argv[1]).read_text()
M = None
for node in ast.parse(source).body:
    if isinstance(node, ast.Assign) and any(getattr(t, "id", None) == "M" for t in node.targets):
        names = {"R": "src/source_recovery.rs", "L": "src/lib.rs",
                 "C": "src/source_receipt/composite.rs", "S": "src/source_receipt.rs"}
        M = eval(compile(ast.Expression(node.value), "review3_mutations.py", "eval"), {}, names)
assert M is not None and len(M) == 12, "reviewer mutant table"
root = pathlib.Path(sys.argv[2])
env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[3])
base = ["test", "--locked", "--offline", "-j", "2"]
prior = base + ["--lib", "--test", "load_reference_state_runtime", "--test", "load_reference_state_runtime_extension",
                "--test", "physics_source_runtime", "--", "--skip", "load_state_join_tests"]
join = base + ["--lib", "--", "source_receipt::load_state_join_tests"]
only = os.environ.get("ONLY")
results = {}
for name, edits in M.items():
    if only and name.split("_")[0] not in only.split(","):
        continue
    originals = {}
    try:
        for rel, old, new in edits:
            path = root / rel
            originals.setdefault(path, path.read_bytes())
            text = path.read_text()
            assert text.count(old) == 1, (name, rel, text.count(old))
            path.write_text(text.replace(old, new))
        entry = {}
        for label, args in (("prior", prior), ("join", join)):
            run = subprocess.run(["cargo", "+1.97.1", *args], cwd=root, env=env, capture_output=True, text=True)
            out = run.stdout + run.stderr
            entry[label] = {
                "exit": run.returncode,
                "compile_error": "error[" in out or "could not compile" in out,
                "failed_tests": [l for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")],
                "results": [l for l in out.splitlines() if l.startswith("test result:")],
            }
            entry[label]["killed"] = run.returncode != 0 and not entry[label]["compile_error"]
        results[name] = entry
        print(name, "prior_killed=", entry["prior"]["killed"], len(entry["prior"]["failed_tests"]),
              "join_killed=", entry["join"]["killed"], len(entry["join"]["failed_tests"]),
              "compile_error=", entry["prior"]["compile_error"] or entry["join"]["compile_error"], flush=True)
        for t in entry["join"]["failed_tests"]:
            print("   join:", t, flush=True)
    finally:
        for path, original in originals.items():
            path.write_bytes(original)
            assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
