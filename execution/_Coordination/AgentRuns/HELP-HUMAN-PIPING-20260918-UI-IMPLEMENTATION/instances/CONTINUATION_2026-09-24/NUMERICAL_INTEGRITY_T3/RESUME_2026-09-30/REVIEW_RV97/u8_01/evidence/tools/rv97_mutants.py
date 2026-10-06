"""RV97 round 1: mutants against U8-1's three tests (registered build, PP `--lib`, facade tests).
Each mutant is a set of exact-text edits (each anchor must occur exactly once) in the MUT copy.
After every run the pristine bytes are restored and checked by sha256. A compile error is not a kill.
Usage: rv97_mutants.py MUT_P LOG_DIR TARGET_DIR T3_CARGO [only-ids...]"""
import hashlib, json, os, re, subprocess, sys
from pathlib import Path

mut_p, log_dir, target, t3_cargo, *only = sys.argv[1:]
P = Path(mut_p)
PP = P / "core/product_physics"
LIB, TESTS = PP / "src/lib.rs", PP / "src/retained_facade_tests.rs"
DENSE = P / "fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"
FILES = [LIB, TESTS, DENSE]
PRISTINE = {f: hashlib.sha256(f.read_bytes()).hexdigest() for f in FILES}
ORIG = {f: f.read_bytes() for f in FILES}

L = "lib"; T = "tests"; D = "dense"
CAND_ARM = "        Err(refusal) => return notice.publish(refusal.ordinary, W1Fallback::Candidate),\n"
PREP_ARM = "        Err(failure) => return notice.publish(failure.ordinary, W1Fallback::Preparation),\n"
NATIVE_RET = "        return notice.publish(prepared.into_ordinary(), W1Fallback::Native);\n"
TRANSFER = "    (frozen.into_ordinary(), Ok(RetainedSuccessor(successor)))\n"
DROP_NOTICE = "    drop(notice);\n"
EXTRA_RUN = "let _ = run_linear_static_preview_value_with_mode(capture.borrowed_raw().clone(), capture.mode());"
ROWS = '    let rows = successor["results"].as_array().unwrap();\n'
CLASSES = '    let classes_l0 = classes(successor, &json!({"request": raw, "solver_mode": name}));\n'
BODY = '    let body = &successor["retained_precision"]["body"];\n'
BEFORE = '            assert!(hooks::armed_names().is_empty(), "{label} {mode:?}: no fault armed before");\n'
L0_SUPPORT = '    raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id": "rigid:N2", "node": "N2", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": p}));\n'
COMBO = '    raw["model"]["combinations"] = json!([{"id": "combo", "basis": "mechanics", "terms": [{"load_case": "case", "factor": 1.0}]}]);\n'
FIX_S = '        include_str!("../../../fixtures/results/retained_precision_l0_successor_sparse_interactive.json"),\n'
FIX_D = '        include_str!("../../../fixtures/results/retained_precision_l0_successor_dense_scrutiny.json"),\n'

MUTANTS = [
    # --- the fallback test: wrong cause ---
    ("F1", "wrong cause (production): the Candidate fallback reports Native", [(L, CAND_ARM, CAND_ARM.replace("W1Fallback::Candidate)", "W1Fallback::Native)"))]),
    ("F2", "wrong cause (production): the Preparation fallback reports Candidate", [(L, PREP_ARM, PREP_ARM.replace("W1Fallback::Preparation)", "W1Fallback::Candidate)"))]),
    ("F3", "wrong cause (production): the Native fallback reports Preparation", [(L, NATIVE_RET, NATIVE_RET.replace("W1Fallback::Native)", "W1Fallback::Preparation)"))]),
    ("F4", "wrong cause (input): tiny_spring's stiffness restored to 144", [(T, 'json!(1e-300);', 'json!(144.0);')]),
    ("F5", "wrong cause (input): W-C1's entry runs two-body case A", [(T, '("w_c1_two_body_case_b", u8_two_body_case_b(), W1Fallback::Native),', '("w_c1_two_body_case_b", u8_two_body_case_a(), W1Fallback::Native),')]),
    ("F6", "outside D1 (input): first_load_only gains a combination", [(T, '    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);\n', '    raw["model"]["load_cases"][0]["primitive_loads"] = json!([first]);\n' + COMBO)]),
    # --- two notices ---
    ("F7", "two notices (production, no allocation): the last ordinary diagnostic is overwritten by the notice, then the notice is appended",
        [(L, "        ordinary.diagnostics.push(notice);\n", "        if let Some(last) = ordinary.diagnostics.last_mut() { *last = notice.clone(); }\n        ordinary.diagnostics.push(notice);\n")]),
    ("F8", "two notices, isolated (production reserves and appends two; the test's with_notice oracle also appends two): only the count can catch it",
        [(L, "        ordinary.diagnostics.try_reserve_exact(1).ok()?;\n", "        ordinary.diagnostics.try_reserve_exact(2).ok()?;\n"),
         (L, "        ordinary.diagnostics.push(notice);\n", "        ordinary.diagnostics.push(notice.clone());\n        ordinary.diagnostics.push(notice);\n"),
         (T, 'format!(r#"{head}"diagnostics":[{items}{sep}{}{rest}"#, notice_json(case, detail))', 'format!(r#"{head}"diagnostics":[{items}{sep}{},{}{rest}"#, notice_json(case, detail), notice_json(case, detail))')]),
    # --- plain bytes without the notice ---
    ("F9", "no notice (production): Candidate fallback returns the plain envelope", [(L, CAND_ARM, "        Err(refusal) => { drop(notice); return (refusal.ordinary, Err(W1Fallback::Candidate)) },\n")]),
    ("F10", "no notice (production): Preparation fallback returns the plain envelope", [(L, PREP_ARM, "        Err(failure) => { drop(notice); return (failure.ordinary, Err(W1Fallback::Preparation)) },\n")]),
    ("F11", "no notice (production): Native fallback returns the plain envelope", [(L, NATIVE_RET, "        drop(notice); return (prepared.into_ordinary(), Err(W1Fallback::Native));\n")]),
    # --- counts ---
    ("F12", "a second ordinary run (production) on the Candidate fallback path", [(L, CAND_ARM, "        Err(refusal) => { " + EXTRA_RUN + " return notice.publish(refusal.ordinary, W1Fallback::Candidate) },\n")]),
    # --- hooks ---
    ("F13", "hook armed before the loop (test)", [(T, "    let variants = [\n        (\"first_load_only\"", "    hooks::break_next_staging();\n    let variants = [\n        (\"first_load_only\"")]),
    ("F14", "hook armed after the before-check and left unfired (test)", [(T, BEFORE, BEFORE + "            hooks::break_next_staging();\n")]),
    ("F15", "hook left armed by production (the Candidate path arms staging on the worker; handed back)",
        [(L, CAND_ARM, "        Err(refusal) => { #[cfg(test)] retained_tests_hooks::break_next_staging(); return notice.publish(refusal.ordinary, W1Fallback::Candidate) },\n")]),
    # --- L = 0: the pinned successor ---
    ("L1", "corrupted body-1 row (production): result:disp:N2:ux = 5e-324 in the published successor",
        [(L, TRANSFER, '    #[cfg(test)] if let Some(r) = successor["results"].as_array_mut().and_then(|a| a.iter_mut().find(|r| r["id"] == "result:disp:N2:ux")) { r["value"] = serde_json::json!(5e-324); }\n' + TRANSFER)]),
    ("L2", "B-prime (production): the ordinary envelope beside a successor gains a diagnostic",
        [(L, TRANSFER, "    ({ let mut o = frozen.into_ordinary(); let d = o.diagnostics[0].clone(); o.diagnostics.push(d); o }, Ok(RetainedSuccessor(successor)))\n")]),
    ("L3", "a second ordinary run (production) on the success path", [(L, DROP_NOTICE, "    " + EXTRA_RUN + "\n" + DROP_NOTICE)]),
    ("L4", "outside D1 (input): the L = 0 request gains a combination", [(T, L0_SUPPORT, L0_SUPPORT + COMBO)]),
    ("L5", "W1 falls back (input): N2 left unsupported", [(T, L0_SUPPORT, "")]),
    # --- L = 0 value controls, past the pin (test-side data mutations) ---
    ("V1", "corrupted body-1 row, past the pin: N2:ux = 5e-324 in the rows the value controls read",
        [(T, ROWS, '    let mut rv97 = successor.clone(); { let r = rv97["results"].as_array_mut().unwrap().iter_mut().find(|r| r["id"] == "result:disp:N2:ux").unwrap(); r["value"] = json!(5e-324); }\n    let rows = rv97["results"].as_array().unwrap();\n')]),
    ("V2", "body-1 class, past the pin: N2:ux classified relative-verified",
        [(T, CLASSES, CLASSES + '    let mut classes_l0 = classes_l0; classes_l0.get_mut("result:disp:N2:ux").unwrap().2 = AccuracyClass::RelativeVerified;\n')]),
    ("V3", "body-1 tally, past the pin: one input-derived row reclassified as an exact-zero absolute row",
        [(T, CLASSES, CLASSES + '    let mut classes_l0 = classes_l0; *classes_l0.get_mut("result:disp:N2:ux").unwrap() = (0, Some(0), AccuracyClass::AbsoluteVerified { bound_bits: 0 });\n')]),
    ("V4", "body-1 coverage, past the pin: has_data true",
        [(T, BODY, '    let mut rv97b = successor["retained_precision"]["body"].clone(); rv97b["product_attempts"][0]["proof"]["summary_coverage"][1]["has_data"] = json!(true);\n    let body = &rv97b;\n')]),
    ("V5", "body-0 bit identity, past the pin: result:disp:N1 moved by one ulp",
        [(T, ROWS, '    let mut rv97 = successor.clone(); { let r = rv97["results"].as_array_mut().unwrap().iter_mut().find(|r| r["id"] == "result:disp:N1").unwrap(); let v = r["value"].as_f64().unwrap(); r["value"] = json!(f64::from_bits(v.to_bits() + 1)); }\n    let rows = rv97["results"].as_array().unwrap();\n')]),
    ("V6", "body-0 U5 criterion, past the pin: result:disp:N1's normalized value moved by 3e-9 relative",
        [(T, CLASSES, CLASSES + '    let mut classes_l0 = classes_l0; { let e = classes_l0.get_mut("result:disp:N1").unwrap(); e.0 = (f64::from_bits(e.0) * (1.0 + 3e-9)).to_bits(); }\n')]),
    ("V6c", "CONTROL (expected to survive): the same row moved by 5e-10 relative, inside the criterion",
        [(T, CLASSES, CLASSES + '    let mut classes_l0 = classes_l0; { let e = classes_l0.get_mut("result:disp:N1").unwrap(); e.0 = (f64::from_bits(e.0) * (1.0 + 5e-10)).to_bits(); }\n')]),
    ("V7", "body-0 class claim, past the pin: result:disp:N1 classified input-derived",
        [(T, CLASSES, CLASSES + '    let mut classes_l0 = classes_l0; classes_l0.get_mut("result:disp:N1").unwrap().2 = AccuracyClass::InputDerived;\n')]),
    # --- D-U6-5 ---
    ("D1", "the dense L = 0 fixture gains a trailing newline", [(D, None, b"\n")]),
    ("D2", "the two fixtures swapped in the D-U6-5 test", [(T, FIX_S + FIX_D, FIX_D + FIX_S)]),
]
FILEMAP = {L: LIB, T: TESTS, D: DENSE}

def apply(edits):
    for kind, old, new in edits:
        f = FILEMAP[kind]
        if old is None:
            f.write_bytes(f.read_bytes() + new)
            continue
        text = f.read_text()
        assert text.count(old) == 1, (kind, old[:80], text.count(old))
        f.write_text(text.replace(old, new))

def restore():
    for f in FILES:
        f.write_bytes(ORIG[f])
        assert hashlib.sha256(f.read_bytes()).hexdigest() == PRISTINE[f], f

def parse(log):
    failed = re.findall(r"^test (\S+) \.\.\. FAILED", log, re.M)
    passed = re.findall(r"^test (\S+) \.\.\. ok", log, re.M)
    panics = {}
    for name, body in re.findall(r"^---- (\S+) stdout ----\n(.*?)(?=^---- |\Z|^failures:\n\n)", log, re.M | re.S):
        m = re.search(r"panicked at [^\n]*\n(.*?)(?:\nnote:|\nstack backtrace|\Z)", body, re.S)
        panics[name] = (m.group(1).strip()[:400] if m else body.strip()[:400])
    compile_error = "error[E" in log or "could not compile" in log
    return failed, passed, panics, compile_error

results = []
env = {k: v for k, v in os.environ.items() if k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS")}
env.update(CARGO_BUILD_JOBS="4", RUST_TEST_THREADS="2", CARGO_TARGET_DIR=target)
for mid, what, edits in MUTANTS:
    if only and mid not in only:
        continue
    restore()
    apply(edits)
    cmd = ["perl", "-e", "alarm shift; exec @ARGV", "2400", t3_cargo, "test", "--locked", "--offline", "--lib", "--no-fail-fast", "retained_facade_tests"]
    p = subprocess.run(cmd, cwd=PP, env=env, capture_output=True, text=True)
    log = p.stdout + p.stderr
    Path(log_dir, f"mutant_{mid}.log").write_text(log)
    restore()
    failed, passed, panics, compile_error = parse(log)
    u8_failed = [t for t in failed if "::u8_" in t]
    entry = {"id": mid, "what": what, "rc": p.returncode, "compile_error": compile_error, "failed": failed, "passed_count": len(passed),
             "killed": bool(failed) and not compile_error, "killed_by_u8": u8_failed, "only_u8": bool(failed) and all("::u8_" in t for t in failed),
             "panics": {t: panics.get(t) for t in failed}}
    results.append(entry)
    print(json.dumps({k: entry[k] for k in ("id", "rc", "compile_error", "killed", "killed_by_u8", "only_u8")}), flush=True)
Path(log_dir, "mutants.json").write_text(json.dumps(results, indent=1) + "\n")
restore()
print("pristine restored and verified")
