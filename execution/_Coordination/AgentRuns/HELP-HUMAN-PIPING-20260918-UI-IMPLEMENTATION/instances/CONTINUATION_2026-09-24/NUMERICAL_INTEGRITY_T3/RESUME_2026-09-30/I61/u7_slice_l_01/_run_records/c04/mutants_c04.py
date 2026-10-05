#!/usr/bin/env python3
"""I61 U7 slice L: the C04 narrowing mutant (`not_required` dropped from the eligibility case-status
conjunct) in each reader, run on the candidate corpus (07j) and on the U7 head's corpus (07i) for
contrast. One textual edit in the disposable `mut` lane; each language's retained contract suite;
pristine bytes restored and compared after every run. One cargo job at a time.

Usage: mutants_c04.py MUT_P HEAD_FILES_DIR LOG_DIR RUST_TARGET_DIR VENV_PYTHON
For the 07i contrast, the corpus and the two contract tests take the U7 head's bytes (HEAD_FILES_DIR).
"""
import hashlib, json, os, shutil, subprocess, sys, time
M, C07I, LOG, TD, PY = sys.argv[1:6]
SWAPPED = {"fixtures/results/retained_precision_cases.json": "retained_precision_cases.json",
           "tests/test_retained_precision_contract.py": "test_retained_precision_contract.py",
           "core/reporting/result_export/tests/retained_precision_contract.rs": "retained_precision_contract.rs"}
CANDIDATE = {rel: open(os.path.join(M, rel), "rb").read() for rel in SWAPPED}
HEAD = {rel: open(os.path.join(C07I, name), "rb").read() for rel, name in SWAPPED.items()}
MUTANTS = [
    ("C04_py", "py", "core/analysis_runs/retained_precision.py",
     'all(c["status"] in ("selected","not_required") for c in cases)', 'all(c["status"] in ("selected",) for c in cases)'),
    ("C04_rs", "rs", "core/reporting/result_export/src/retained_precision.rs",
     '            .all(|c| matches!(text(&c["status"]), "selected" | "not_required"));', '            .all(|c| matches!(text(&c["status"]), "selected"));'),
    ("C04_ts", "ts", "apps/desktop/src/features/results/retainedPrecision.ts",
     "b.cases.every((c: Obj) => ['selected', 'not_required'].includes(c.status))", "b.cases.every((c: Obj) => ['selected'].includes(c.status))"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", PYTHONDONTWRITEBYTECODE="1")
ENV.pop("RUSTFLAGS", None)
results = []
for corpus_label, files in (("07j", CANDIDATE), ("07i", HEAD)):
    for rel, data in files.items():
        open(os.path.join(M, rel), "wb").write(data)
    for mid, lang, rel, old, new in MUTANTS:
        target = os.path.join(M, rel)
        pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
        text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
        open(target, "w").write(text.replace(old, new))
        t0 = time.time()
        try:
            if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
                sys.exit("MEMGUARD NOT RUNNING")
            if lang == "py":
                cmd, cwd = [PY, "-m", "pytest", "-q", "-p", "no:cacheprovider", "--basetemp", os.path.join(LOG, "tmp_" + mid + corpus_label), "tests/test_retained_precision_contract.py"], M
            elif lang == "rs":
                cmd, cwd = ["cargo", "test", "--locked", "--offline", "--no-fail-fast", "--target-dir", TD, "--test", "retained_precision_contract"], os.path.join(M, "core/reporting/result_export")
            else:
                cmd, cwd = ["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--maxWorkers=1"], os.path.join(M, "apps/desktop")
            proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1800"] + cmd, cwd=cwd, env=ENV, capture_output=True, text=True)
        finally:
            open(target, "wb").write(pristine)
            assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
        out = proc.stdout + proc.stderr
        open(os.path.join(LOG, f"mutant_{mid}_{corpus_label}.log"), "w").write(out)
        if lang == "py":
            failing = sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
        elif lang == "rs":
            failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
        else:
            failing = sorted({l.strip() for l in out.splitlines() if l.strip().startswith("×")})
        compile_error = "error[" in out or "could not compile" in out or "SyntaxError" in out
        r = {"id": mid, "corpus": corpus_label, "killed": proc.returncode != 0, "compile_error": compile_error,
             "failing_count": len(failing), "failing": failing[:8], "seconds": round(time.time() - t0, 1)}
        results.append(r); print(json.dumps(r), flush=True)
for rel, data in CANDIDATE.items():
    open(os.path.join(M, rel), "wb").write(data)
    assert open(os.path.join(M, rel), "rb").read() == data
json.dump(results, open(os.path.join(LOG, "mutants_c04.json"), "w"), indent=1)
