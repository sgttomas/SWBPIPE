"""B6 (I83): apply stage A (items 1, 2 and 4) or stage B (item 3, on top of A) to a P tree, from the
reviewed dev versions in DEV. Files touched by one stage only are copied from DEV; the four shared
files (case file, three carrier tests) get stage-specific text. Usage: stage_patch.py <P> <A|B> <DEV P>"""
import shutil, subprocess, sys
from pathlib import Path
P, STAGE, DEV = Path(sys.argv[1]), sys.argv[2], Path(sys.argv[3])
SCRIPTS = Path(__file__).resolve().parent
ONLY = {"A": ["apps/desktop/src/features/results/retainedPrecision.ts", "apps/desktop/src/features/results/retainedPrecision.test.ts",
              "fixtures/results/retained_precision_cases.json", "core/reporting/result_export/tests/retained_precision_contract.rs",
              "tests/test_retained_precision_contract.py"],
        "B": ["core/analysis_runs/retained_precision.py", "core/analysis_runs/compatibility.py"]}
for rel in ONLY[STAGE]:
    shutil.copyfile(DEV / rel, P / rel)

def patch(rel, pairs):
    path = P / rel; t = path.read_text()
    for a, b in pairs:
        assert t.count(a) == 1, (rel, a[:100]); t = t.replace(a, b)
    path.write_text(t)

PY_N3_OLD = '''    assert all(p in cases["scope"] for p in ("G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse",
                                              "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "An invalid enum value in a not_required case's quality is refused at G7 with each language's own code", "Python SOURCE_NUMERICAL_CASE_INVALID"))'''
PY_N3_A = '''    assert all(p in cases["scope"] for p in ("G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse",
                                              "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"))
    # RV94 N-3 (B6; PLAN decision 11): TS aligned, so no language-specific G7 code is declared for an
    # invalid numerical_quality case.
    assert not any(p in cases["scope"] for p in ("An invalid enum value in a not_required case's quality", "SOURCE_NUMERICAL_CASE_INVALID"))'''
PY_N3_B = '''    assert all(p in cases["scope"] for p in ("G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse",
                                              "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Python SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID",
                                              # F-U6b-2 (B6): Python's transport refusals carry the reader's codes, as Rust's.
                                              "Rust and Python the reader's G0 code or their base header code"))
    # RV94 N-3 (B6; PLAN decision 11): TS aligned, so no language-specific G7 code is declared for an
    # invalid numerical_quality case, and Python's transport is no longer a declared refusal.
    assert not any(p in cases["scope"] for p in ("An invalid enum value in a not_required case's quality", "SOURCE_NUMERICAL_CASE_INVALID", "Python F-U6b-2's code"))'''
RS_N3_OLD = '''    assert!(["G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse", "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE", "An invalid enum value in a not_required case's quality is refused at G7 with each language's own code", "Rust SOURCE_NUMERICAL_CASE_INVALID"].iter().all(|p| cases["scope"].as_str().unwrap().contains(p)));'''
RS_N3_A = '''    assert!(["G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse", "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE"].iter().all(|p| cases["scope"].as_str().unwrap().contains(p)));
    // RV94 N-3 (B6; PLAN decision 11): TS aligned to this reader's G7 code, so no
    // language-specific code is declared for an invalid numerical_quality case.
    assert!(!["An invalid enum value in a not_required case's quality", "SOURCE_NUMERICAL_CASE_INVALID"].iter().any(|p| cases["scope"].as_str().unwrap().contains(p)));'''
RS_N3_B = '''    assert!(["G7 parity compares the reader's (gate, code)", "parity there compares only accept against refuse", "no carrier authenticates producer origin", "a blocked envelope is refused at G7 with each language's own base code", "Rust SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE", "Rust and Python the reader's G0 code or their base header code"].iter().all(|p| cases["scope"].as_str().unwrap().contains(p)));
    // RV94 N-3 (B6; PLAN decision 11): TS aligned to this reader's G7 code, so no
    // language-specific code is declared for an invalid numerical_quality case,
    // and Python's transport is no longer a declared refusal (F-U6b-2).
    assert!(!["An invalid enum value in a not_required case's quality", "SOURCE_NUMERICAL_CASE_INVALID", "Python F-U6b-2's code"].iter().any(|p| cases["scope"].as_str().unwrap().contains(p)));'''
TS_N3_OLD = '''    // RV94 N-3 (U7 repair): an invalid enum value in a not_required case's quality, TS's G7 contract check first.
    expect(caseFile.scope).toMatch(/An invalid enum value in a not_required case's quality is refused at G7 with each language's own code[^.]*TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED/);'''
TS_N3_A = '''    // RV94 N-3 (B6; PLAN decision 11): TS's G7 refusal now carries the base readers' header code, so no
    // language-specific code is declared for an invalid numerical_quality case.
    expect(caseFile.scope).not.toMatch(/An invalid enum value in a not_required case's quality|SOURCE_NUMERICAL_CASE_INVALID/);'''
TS_N3_B = TS_N3_A + '''
    // F-U6b-2 (B6): Python's transport refusals carry the reader's codes, as Rust's; TS's header code stays its own.
    expect(caseFile.scope).toMatch(/TS SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED; Rust and Python the reader's G0 code or their base header code/);
    expect(caseFile.scope).not.toMatch(/Python F-U6b-2's code/);'''
CASE = "fixtures/results/retained_precision_carrier_cases.json"
PYC, RSC, TSC = "tests/test_retained_precision_carriers.py", "core/reporting/result_export/tests/retained_precision_carriers.rs", "apps/desktop/src/features/results/retainedPrecisionIntegration.test.tsx"
if STAGE == "A":
    subprocess.run([sys.executable, str(SCRIPTS / "build_case_file.py"), str(P / CASE), "A"], check=True)
    patch(PYC, [(PY_N3_OLD, PY_N3_A)]); patch(RSC, [(RS_N3_OLD, RS_N3_A)]); patch(TSC, [(TS_N3_OLD, TS_N3_A)])
else:
    subprocess.run([sys.executable, str(SCRIPTS / "build_case_file.py"), str(P / CASE), "B"], check=True)
    # Item 3 in the shared carrier tests: the final versions (the caller checks the stage-B diff and
    # compares the result with DEV byte for byte); the scope-pin texts are PY_N3_B, RS_N3_B and TS_N3_B.
    for rel, text in ((PYC, PY_N3_B), (RSC, RS_N3_B), (TSC, TS_N3_B)):
        assert (DEV / rel).read_text().count(text) == 1, rel
        shutil.copyfile(DEV / rel, P / rel)
print("stage", STAGE, "applied")
