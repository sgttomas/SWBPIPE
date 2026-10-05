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
 # ---- I61's 22, verbatim edits ----
 ("I61:PY01_f5_removed", "py", [(PYF, F5PY, "        pass")]),
 ("I61:PY02_f5_retained_included", "py", [(PYF, ' and not str(d.get("code")).startswith("RETAINED_PRECISION_")])', '])')]),
 ("I61:PY03_f5_order_free", "py", [(PYF, F5PY, F5PY.replace("fail(refs == [", "fail(sorted(refs) == sorted([").replace('"RETAINED_PRECISION_")])', '"RETAINED_PRECISION_")]))'))]),
 ("I61:PY04_f5_case_filter_removed", "py", [(PYF, 'if name in (d.get("affected_refs") or []) and not str(', 'if not str(')]),
 ("I61:M33_d37_removed", "py", [(PYF, 'fail(tuple(st[k] for k in STAGE_ORDER) in ERROR_STAGE_RECORDS.get(result["error"]["kind"], ()))', 'pass')]),
 ("I61:M34_observable_narrowed", "py", [(PYF, '"observable": {done(8, [F, C]), done(8, [F, F])}', '"observable": {done(8, [F, C])}')]),
 ("I61:M35_abandoned_widened_after_certificate", "py", [(PYF,) + M35]),
 ("I61:M36_proof_drops_unchecked_pair", "py", [(PYF, 'for x, y in ((N, N), (C, C), (C, F), (F, C), (F, F))', 'for x, y in ((C, C), (C, F), (F, C), (F, F))')]),
 ("I61:M37_values_widened", "py", [(PYF, '"values": {done(5, [F])}', '"values": {done(5, [F]), done(6, [])}')]),
 ("I61:M38_numeric_widened", "py", [(PYF, '"numeric": {done(10, [])}', '"numeric": {done(10, []), done(8, [])}')]),
 ("I61:M39_g5a_widened", "py", [(PYF, '"g5a": {done(9, [F])}', '"g5a": {done(9, [F]), done(8, [])}')]),
 ("I61:PY_M50_statement_normalized", "py", [(PYF, '_normalize_integrals(receipt)  # D34, then D32', '_normalize_integrals(snapshot)  # D34, then D32')]),
 ("I61:RS01_f5_removed", "rs", [(RSF, F5RS, '        let _ = exact;')]),
 ("I61:RS02_f5_retained_included", "rs", [(RSF, '.filter(|d| list(&d["affected_refs"]).contains(cid) && !text(&d["code"]).starts_with("RETAINED_PRECISION_"))', '.filter(|d| list(&d["affected_refs"]).contains(cid))')]),
 ("I61:RS03_d37_values_widened", "rs", [(RSF, '        "values" => first_failed == Some("values"),', '        "values" => first_failed == Some("values") || (first_failed.is_none() && is("values", "completed") && is("aliases", "not_entered")),')]),
 ("I61:RS04_d37_abandoned_widened_after_certificate", "rs", [(RSF, '                    && is("certificate", "not_entered"))', '                    && is("certificate", "not_entered"))\n                || (certified && is("observables", "not_entered"))')]),
 ("I61:RS05_m50_statement_normalized", "rs", [(RSF, '    if let Some(r) = owned.get_mut("retained_precision") {\n        normalize(r);\n    }', '    normalize(&mut owned);')]),
 ("I61:TS01_f5_removed", "ts", [(TSF, F5TS, "    void exact;")]),
 ("I61:TS02_f5_order_free", "ts", [(TSF, F5TS, "    fail(a.diagnostic_refs.length === exact.length && a.diagnostic_refs.every((ref: string) => exact.includes(ref)));")]),
 ("I61:TS03_f5_retained_included", "ts", [(TSF, " && !String(d.code).startsWith('RETAINED_PRECISION_')).map(d => d.id);", ").map(d => d.id);")]),
 ("I61:TS04_d37_values_widened", "ts", [(TSF, "  values: ['CCCCCF----'],", "  values: ['CCCCCF----', 'CCCCCC----'],")]),
 ("I61:TS05_d37_abandoned_widened_after_certificate", "ts", [(TSF, "  abandoned: ['CCCCF-----', 'CCCCCCF---', 'CCCCCCC---'],", "  abandoned: ['CCCCF-----', 'CCCCCCF---', 'CCCCCCC---', 'CCCCCCCC--'],")]),
 # ---- RV90's own ----
 ("RV90:PY01_f5_first_case_only", "py", [(PYF, F5PY, "        if i == 0:" + F5PY[7:])]),
 ("RV90:PY02_f5_excludes_selected_code_only", "py", [(PYF, 'and not str(d.get("code")).startswith("RETAINED_PRECISION_")])', 'and str(d.get("code")) != "RETAINED_PRECISION_SELECTED"])')]),
 ("RV90:PY03_d6a_unique_resolve_removed", "py", [(PYF, '        fail(len(set(refs)) == len(refs) and all(x in by_id for x in refs))', '        pass')]),
 ("RV90:PY04_d37_test_expects_reader_table_with_M35", "py", [
     (PYT, 'expected = {kind: {tuple(marks[m] for m in r) for r in records} for kind, records in table["kinds"].items()}', 'expected = rp.ERROR_STAGE_RECORDS'),
     (PYF,) + M35]),
 ("RV90:RS01_f5_first_case_only", "rs", [(RSF, F5RS, '        if i == 0 {\n' + F5RS + '\n        }')]),
 ("RV90:RS02_f5_prefix_tolerant", "rs", [(RSF, F5RS, '        fail(exact.starts_with(&list(&o["diagnostic_refs"]).iter().collect::<Vec<_>>()))?;')]),
 ("RV90:RS03_d37_not_enforced_in_g5_products", "rs", [(RSF, '        pf(error_stages(a))?;', '        let _ = error_stages(a);')]),
 ("RV90:RS04_f5_excludes_selected_code_only", "rs", [(RSF, '&& !text(&d["code"]).starts_with("RETAINED_PRECISION_"))', '&& text(&d["code"]) != "RETAINED_PRECISION_SELECTED")')]),
 ("RV90:TS01_f5_length_check_dropped", "ts", [(TSF, F5TS, "    fail(a.diagnostic_refs.every((ref: string, i: number) => ref === exact[i]));")]),
 ("RV90:TS02_f5_first_case_only", "ts", [(TSF, F5TS, "    if (ci === 0)" + F5TS[3:])]),
 ("RV90:TS03_d37_not_enforced_in_productAttempts", "ts", [(TSF, "    fail(errorStageRecordAgrees(a.result.error.kind, a.stages));", "    void errorStageRecordAgrees(a.result.error.kind, a.stages);")]),
 ("RV90:TS04_f5_isArray_guard_dropped", "ts", [(TSF, "ds.filter(d => Array.isArray(d.affected_refs) && d.affected_refs.includes(cid)", "ds.filter(d => d.affected_refs?.includes(cid)")]),
]
CORPUS_MUT = [
 ("RV90:ALL01_corpus_d37_abandoned_widened", [(CORPUS, '        "CCCCCCF---",\n        "CCCCCCC---"\n      ],\n      "numeric"', '        "CCCCCCF---",\n        "CCCCCCC---",\n        "CCCCCCCC--"\n      ],\n      "numeric"')]),
 ("RV90:ALL02_corpus_d37_abandoned_narrowed", [(CORPUS, '        "CCCCCCF---",\n        "CCCCCCC---"\n      ],\n      "numeric"', '        "CCCCCCF---"\n      ],\n      "numeric"')]),
]
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
