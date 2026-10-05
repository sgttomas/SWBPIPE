"""RV90 mutant runner: I61's 22 re-run plus RV90's own; one mutant at a time per language lane;
pristine bytes restored and verified after each run."""
import hashlib, json, os, subprocess, sys, time
M = os.environ["RV90_MUT"]; LOG = os.environ["RV90_MUTLOG"]; TGT = os.environ["RV90_MUT_TARGET"]
PYF = "core/analysis_runs/retained_precision.py"; PYT = "tests/test_retained_precision_contract.py"
RSF = "core/reporting/result_export/src/retained_precision.rs"
TSF = "apps/desktop/src/features/results/retainedPrecision.ts"
CORPUS = "fixtures/results/retained_precision_cases.json"
F5PY = '        fail(refs == [d["id"] for d in diags if name in (d.get("affected_refs") or []) and not str(d.get("code")).startswith("RETAINED_PRECISION_")])'
F5TS = "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"
F5RS = '        fail(list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;'
M35 = ('"abandoned": {done(4, [F]), done(6, [F]), done(7, [])}', '"abandoned": {done(4, [F]), done(6, [F]), done(7, []), done(8, [])}')
MUT = [
 ("RV90:PY01_f5_first_case_only", "py", [(PYF, '        fail(refs == [d["id"] for d in diags if isinstance(', '        if i == 0: fail(refs == [d["id"] for d in diags if isinstance(')]),
 ("RV90:PY05_s1_reverted_to_07g_predicate", "py", [(PYF, 'if isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]', 'if name in (d.get("affected_refs") or [])')]),
 ("RV90:RS01_f5_first_case_only", "rs", [(RSF, F5RS, '        if i == 0 {\n' + F5RS + '\n        }')]),
 ("RV90:RS02_f5_prefix_tolerant", "rs", [(RSF, F5RS, '        fail(exact.starts_with(&list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()))?;')]),
 ("RV90:TS01_f5_length_check_dropped", "ts", [(TSF, F5TS, "    fail(a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));")]),
 ("RV90:TS02_f5_first_case_only", "ts", [(TSF, F5TS, "    if (ci === 0)" + F5TS[3:])]),
 ("RV90:TS04_f5_isArray_guard_dropped", "ts", [(TSF, "ds.filter(d => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid)", "ds.filter(d => d.affected_refs?.includes(cid)")]),
]
CORPUS_MUT = []
def run(lang):
    env = dict(os.environ)
    if lang == "py":
        return subprocess.run([os.environ["VENV"] + "/bin/python", "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_schema.py"], cwd=M, capture_output=True, text=True, timeout=1800, env=env)
    if lang == "rs":
        assert subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode == 0, "MEMGUARD NOT RUNNING"
        env.update(CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
        return subprocess.run(["cargo", "test", "--locked", "--offline", "--no-fail-fast", "--target-dir", TGT], cwd=os.path.join(M, "core/reporting/result_export"), capture_output=True, text=True, timeout=2400, env=env)
    if lang == "ts":
        return subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--maxWorkers=2"], cwd=os.path.join(M, "apps/desktop"), capture_output=True, text=True, timeout=1800, env=env)
def failing(lang, out):
    if lang == "py": return sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
    if lang == "rs": return sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
    return [l.strip()[:160] for l in out.splitlines() if l.strip().startswith(("×", "FAIL"))]
def one(mid, langs, edits, results):
    files = sorted({e[0] for e in edits}); pristine = {f: open(os.path.join(M, f), "rb").read() for f in files}
    texts = {f: pristine[f].decode() for f in files}
    for f, old, new in edits:
        assert texts[f].count(old) == 1, (mid, f, texts[f].count(old)); texts[f] = texts[f].replace(old, new)
    for f in files: open(os.path.join(M, f), "w").write(texts[f])
    try:
        for lang in langs:
            t0 = time.time(); proc = run(lang); out = proc.stdout + proc.stderr
            open(os.path.join(LOG, f"{mid.replace(':', '_')}_{lang}.log"), "w").write(out)
            fl = failing(lang, out)
            r = {"id": mid, "lang": lang, "killed": proc.returncode != 0, "compile_or_collect_error": ("error[" in out or "could not compile" in out or "SyntaxError" in out or "Transform failed" in out), "failing_count": len(fl), "failing": fl[:10], "seconds": round(time.time() - t0, 1)}
            results.append(r); print(json.dumps(r), flush=True)
    finally:
        for f in files:
            open(os.path.join(M, f), "wb").write(pristine[f]); assert open(os.path.join(M, f), "rb").read() == pristine[f]
which = sys.argv[1]; results = []
if which == "corpus":
    for mid, edits in CORPUS_MUT: one(mid, ["py", "rs", "ts"], edits, results)
else:
    for mid, lang, edits in MUT:
        if lang in which.split(","): one(mid, [lang], edits, results)
json.dump(results, open(os.path.join(LOG, f"mutants_{which.replace(',', '_')}.json"), "w"), indent=1)
