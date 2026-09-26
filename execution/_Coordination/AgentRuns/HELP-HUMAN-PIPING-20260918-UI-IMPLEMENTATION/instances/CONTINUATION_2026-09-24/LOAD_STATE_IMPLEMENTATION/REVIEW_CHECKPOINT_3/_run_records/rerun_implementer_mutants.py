"""CP3 review: rerun the implementer's 17 mutants (anchors taken verbatim from
LSI/_run_records/session2/cp3_join_mutations.py) on a scratch copy of the FROZEN
candidate bytes, against the committed suites (lib without reviewer probes,
load_reference_state_runtime, load_reference_state_runtime_extension,
physics_source_runtime). Bytes restored and sha256-verified after each mutant.
usage: python3 rerun_implementer_mutants.py <implementer script> <scratch core/product_physics> <target>"""
import hashlib, json, os, pathlib, subprocess, sys
src = open(sys.argv[1]).read()
ns = {}
exec(src[src.index('M = {'):src.index('results = {}')], ns)
root = pathlib.Path(sys.argv[2]); env = dict(os.environ, CARGO_TARGET_DIR=sys.argv[3])
args = ["cargo", "+1.97.1", "test", "--locked", "--offline", "-j", "1", "--lib", "--test", "load_reference_state_runtime",
        "--test", "load_reference_state_runtime_extension", "--test", "physics_source_runtime", "--", "--skip", "review3_"]
results = {}
for name, edits in ns['M'].items():
    originals = {}
    try:
        for rel, old, new in edits:
            path = root / rel
            originals.setdefault(path, path.read_bytes())
            text = path.read_text()
            assert text.count(old) == 1, (name, rel)
            path.write_text(text.replace(old, new))
        run = subprocess.run(args, cwd=root, env=env, capture_output=True, text=True)
        out = run.stdout + run.stderr
        failed = [l.split(' ')[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compile_error = "error[" in out or "could not compile" in out
        results[name] = {"exit": run.returncode, "compile_error": compile_error, "killed": run.returncode != 0 and not compile_error, "failed_tests": failed}
        print(name, "killed=", results[name]["killed"], len(failed), "compile_error=", compile_error, flush=True)
    finally:
        for path, original in originals.items():
            path.write_bytes(original)
            assert hashlib.sha256(path.read_bytes()).hexdigest() == hashlib.sha256(original).hexdigest()
print(json.dumps(results, indent=1))
