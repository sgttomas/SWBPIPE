#!/usr/bin/env python3
"""I61 07h mutants: RV90's surviving F5 mutants (N1, N2, S1), S1's Python revert and the cross-reader
analogues, plus the round-1 F5 mutants whose text or killing test changed. One textual edit each in a
disposable tree (Python and Rust: `mut`; TypeScript: `lane`), the language's full suite, killed or
survived, the pristine bytes restored and compared after every run.

Usage: mutants_07h.py MUT_P LANE_P LOG_DIR RUST_TARGET_DIR [ids...]
"""
import hashlib, json, os, subprocess, sys, time
M, L, LOG, TD = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
only = set(sys.argv[5:])
PY = os.path.join(M, "core/analysis_runs/retained_precision.py")
RS = os.path.join(M, "core/reporting/result_export/src/retained_precision.rs")
TS = os.path.join(L, "apps/desktop/src/features/results/retainedPrecision.ts")

PY_F5_HEAD = '        fail(refs == [d["id"] for d in diags if isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]\n'
PY_F5_TAIL = '                      and not str(d.get("code")).startswith("RETAINED_PRECISION_")])\n'
RS_F5 = '        fail(list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>() == exact)?;'
RS_FILTER = '.filter(|d| list(&d["affected_refs"]).contains(cid) && !text(&d["code"]).starts_with("RETAINED_PRECISION_"))'
TS_F5 = "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"
TS_FILTER = "ds.filter(d => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid)"

MUTANTS = [
    # RV90's surviving mutants (N1, N2, S1), verbatim in intent.
    ("RV90:PY01_f5_first_case_only", "py", PY_F5_HEAD, "        if i == 0: " + PY_F5_HEAD[8:]),
    ("RV90:RS01_f5_first_case_only", "rs", RS_F5, "        if i == 0 {\n" + RS_F5 + "\n        }"),
    ("RV90:RS02_f5_prefix_tolerant", "rs", RS_F5, '        fail(exact.starts_with(&list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()))?;'),
    ("RV90:TS01_f5_length_check_dropped", "ts", TS_F5, "    fail(a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));"),
    ("RV90:TS02_f5_first_case_only", "ts", TS_F5, "    if (ci === 0)" + TS_F5[3:]),
    ("RV90:TS04_f5_isArray_guard_dropped", "ts", TS_FILTER, "ds.filter(d => d.affected_refs?.includes(cid)"),
    # S1 in Python: the 07g predicate restored; and the cross-reader analogues of N2 and S1.
    ("I61:PY05_s1_reverted", "py", 'isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]\n', 'name in (d.get("affected_refs") or [])\n'),
    ("I61:PY06_f5_prefix_tolerant", "py", PY_F5_TAIL, PY_F5_TAIL.replace('_")])', '_")][:len(refs)])')),
    ("I61:RS06_s1_string_names_case", "rs", RS_FILTER, '.filter(|d| (list(&d["affected_refs"]).contains(cid) || d["affected_refs"] == *cid) && !text(&d["code"]).starts_with("RETAINED_PRECISION_"))'),
    # Round-1 F5 mutants on the edited Python line, and Rust's F5 removal (N3 sharpened its probe).
    ("I61:PY01_f5_removed", "py", PY_F5_HEAD + PY_F5_TAIL, "        pass\n"),
    ("I61:PY02_f5_retained_included", "py", PY_F5_TAIL, '                      ])\n'),
    ("I61:PY03_f5_order_free", "py", PY_F5_HEAD + PY_F5_TAIL, PY_F5_HEAD.replace("fail(refs == [", "fail(sorted(refs) == sorted([") + PY_F5_TAIL.replace('_")])', '_")]))')),
    ("I61:PY04_f5_case_filter_removed", "py", 'if isinstance(d.get("affected_refs"), list) and name in d["affected_refs"]\n                      and not str(', 'if not str('),
    ("I61:RS01_f5_removed", "rs", RS_F5, "        let _ = exact;"),
]
ENV = dict(os.environ, CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2")
results = []
for mid, lang, old, new in MUTANTS:
    if only and mid not in only:
        continue
    target = {"py": PY, "rs": RS, "ts": TS}[lang]
    pristine = open(target, "rb").read(); psha = hashlib.sha256(pristine).hexdigest()
    text = pristine.decode(); assert text.count(old) == 1, (mid, text.count(old))
    open(target, "w").write(text.replace(old, new))
    t0 = time.time()
    try:
        if lang == "py":
            cmd, cwd = [sys.executable, "-m", "pytest", "-q", "-p", "no:cacheprovider", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_schema.py"], M
        elif lang == "rs":
            if subprocess.run(["pgrep", "-f", "memguard.sh"], capture_output=True).returncode != 0:
                sys.exit("MEMGUARD NOT RUNNING")
            cmd, cwd = ["cargo", "test", "--locked", "--offline", "--no-fail-fast", "--target-dir", TD], os.path.join(M, "core/reporting/result_export")
        else:
            cmd, cwd = ["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--maxWorkers=1"], os.path.join(L, "apps/desktop")
        proc = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1200"] + cmd, cwd=cwd, env=ENV, capture_output=True, text=True)
    finally:
        open(target, "wb").write(pristine)
        assert hashlib.sha256(open(target, "rb").read()).hexdigest() == psha
    out = proc.stdout + proc.stderr
    open(os.path.join(LOG, f"mutant_{mid.replace(':', '_')}.log"), "w").write(out)
    if lang == "py":
        failing = sorted({l.split("::", 1)[1].split(" ")[0] for l in out.splitlines() if l.startswith("FAILED ")})
        error = "SyntaxError" in out or "ImportError" in out or " error" in (out.strip().splitlines() or [""])[-1]
    elif lang == "rs":
        failing = sorted({l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.endswith("FAILED")})
        error = "error[" in out or "could not compile" in out
    else:
        failing = sorted({l.strip() for l in out.splitlines() if l.strip().startswith("×")})
        error = "Transform failed" in out or "SyntaxError" in out
    r = {"id": mid, "lang": lang, "killed": proc.returncode != 0, "compile_or_collect_error": bool(error),
         "failing_count": len(failing), "failing": failing[:12], "seconds": round(time.time() - t0, 1)}
    results.append(r); print(json.dumps(r), flush=True)
json.dump(results, open(os.path.join(LOG, "mutants_07h" + ("_" + "_".join(sorted(x.split(":")[1] for x in only)) if only else "") + ".json"), "w"), indent=1)
