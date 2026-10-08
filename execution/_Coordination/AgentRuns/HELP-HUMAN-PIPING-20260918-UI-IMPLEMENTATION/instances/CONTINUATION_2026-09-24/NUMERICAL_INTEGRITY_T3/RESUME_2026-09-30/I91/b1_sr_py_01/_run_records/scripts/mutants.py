"""I91 SR-PY mutants: one textual edit each to a copy of the head tree, then the two retained test files.

Usage: mutants.py <mutant P root (a copy of the head archive)> <out.jsonl> [ids...]
Each mutant replaces exactly one occurrence of `old` with `new` in one file (asserted), runs
tests/test_retained_precision_contract.py and tests/test_retained_precision_carriers.py with junit
output, records the failing tests, and restores the file byte for byte. "BASE" swaps in the base
reader and compatibility module instead (head tests at base).
"""
import json, os, subprocess, sys, xml.etree.ElementTree as ET
from pathlib import Path

ROOT, OUT = Path(sys.argv[1]), Path(sys.argv[2])
ONLY = set(sys.argv[3:])
S = Path(__file__).parent
PY, CP = "core/analysis_runs/retained_precision.py", "core/analysis_runs/compatibility.py"
STATUS_SET = '{"not_assessed", "checks_passed", "sensitive", "unresolved", "failed"}'

MUTANTS = [
    # R-D38 (4b) and D38's one relaxed check.
    ("D38-restore", "the relaxed D38 check restored: an entered native stage needs a Run (no (4b) branch)", PY,
     '''    elif st["native"] == "failed" and a["run_ref"] is None:
        # R-D38''', '''    elif False:
        # R-D38'''),
    ("D38-capture", "(4b) without its capture-error conjunct", PY,
     'and a["result"]["kind"] == "unavailable" and a["result"]["error"]["kind"] == "capture"',
     'and a["result"]["kind"] == "unavailable"'),
    ("D38-stages", "(4b) without the later stages not_entered", PY,
     'and all(st[k] == "not_entered" for k in STAGE_ORDER[2:])', 'and True'),
    ("D38-prep", "(4b) without preparation completed and native failed", PY,
     'and st["preparation"] == "completed" and st["native"] == "failed"', 'and True'),
    ("D38-reason", "(4b) without the reason (source_unavailable, preparation)", PY,
     'and (reason.get("code"), reason.get("phase")) == ("source_unavailable", "preparation")', 'and True'),
    ("D38-source-eq", "(4b) without a.source_ref == case.source_ref (r01 N-6)", PY,
     'and a["source_ref"] is not None and a["source_ref"] == case.get("source_ref"))', 'and a["source_ref"] is not None)'),
    ("D38-source-nonnull", "(4b) without a.source_ref non-null", PY,
     'and a["source_ref"] is not None and a["source_ref"] == case.get("source_ref"))', 'and a["source_ref"] == case.get("source_ref"))'),
    ("D38-names-a", "(4b) without the cause naming this attempt", PY,
     'and cause.get("product_attempt_ref") == ai\n', 'and True\n'),
    ("D38-case-status", "(4b) without the case unavailable with prepared_product_failure", PY,
     'and case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure"', 'and True'),
    ("D38-no-run-no-proof", "(4b) without case.run null and proof null", PY,
     'return (case.get("run") is None and a["proof"] is None\n', 'return (True\n'),
    ("D38-builds", "the every-Build-referenced check removed ((4b)'s Build conjunct)", PY,
     '    fail(built_refs == set(range(len(body["builds"]))), "WORK_MISMATCH")\n', ''),
    # G5's not_required rule: Rust's three dropped conjuncts restored, one at a time.
    ("G5-report", "not_required also needs initial.kind == report", PY,
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed")',
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed" and o["initial"]["kind"] == "report")'),
    ("G5-outcome", "not_required also needs initial.outcome == checks_passed", PY,
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed")',
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed" and o["initial"].get("outcome") == "checks_passed")'),
    ("G5-w2", "not_required also needs w2.kind == not_triggered", PY,
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed")',
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed" and o["w2"]["kind"] == "not_triggered")'),
    # G8's per-case loop, the requested mode, P1-P4.
    ("G8-selected-only", "the per-case loop over selected cases only (RS's old scope)", PY,
     '''        o = body["ordinary_attempts"][i]
        need(o["requested_mode"] == mode)''',
     '''        o = body["ordinary_attempts"][i]
        if case["status"] != "selected": continue
        need(o["requested_mode"] == mode)'''),
    ("G8-first-only", "the per-case loop over the first case only", PY,
     '    for i, case in enumerate(body["cases"]):\n        o = body["ordinary_attempts"][i]\n',
     '    for i, case in enumerate(body["cases"][:1]):\n        o = body["ordinary_attempts"][i]\n'),
    ("G8-requested", "the requested-mode check removed", PY,
     '        need(o["requested_mode"] == mode)\n', ''),
    ("P1-removed", "P1 removed", PY,
     'need(len(modes) == 1 and mode_code is not None and type(modes[0]["value"]) in (int, float) and modes[0]["value"] == mode_code)',
     'pass'),
    ("P1-count", "P1 without exactly one mode row (any number, the first checked)", PY,
     'need(len(modes) == 1 and mode_code is not None', 'need(len(modes) >= 1 and mode_code is not None'),
    ("P1-code3", "P1 admits mode code 3 in sparse (TS's old rule)", PY,
     'and modes[0]["value"] == mode_code)', 'and modes[0]["value"] in ((1, 3) if mode_code == 1 else (mode_code,)))'),
    ("P2-removed", "P2 removed (at most one parity row)", PY,
     'need(parity <= 1 and (parity == 0 or mode == "dense_scrutiny")', 'need(True and (parity == 0 or mode == "dense_scrutiny")'),
    ("P3-removed", "P3 removed (no parity row in sparse)", PY,
     'need(parity <= 1 and (parity == 0 or mode == "dense_scrutiny")', 'need(parity <= 1 and True'),
    ("P4-removed", "P4 removed (no parity row when W2 published)", PY,
     'and (parity == 0 or o["w2"]["kind"] != "published"))', ')'),
    ("P2-4-old", "P2-P4 replaced by RS's old 'exactly one parity row iff dense'", PY,
     'need(parity <= 1 and (parity == 0 or mode == "dense_scrutiny") and (parity == 0 or o["w2"]["kind"] != "published"))',
     'need(parity == int(mode == "dense_scrutiny"))'),
    # RV108 N1: each guard removed.
    ("N1-status", "N1 guard removed: numerical_quality.status", CP,
     'not isinstance(quality.get("status"), str) or quality.get("status") not in', 'quality.get("status") not in'),
    ("N1-solve", "N1 guard removed: case solve_quality", CP,
     'not isinstance(case.get("solve_quality"), str) or case.get("solve_quality") not in', 'case.get("solve_quality") not in'),
    ("N1-structural", "N1 guard removed: case structural_status", CP,
     'not isinstance(case.get("structural_status"), str) or case.get("structural_status") not in', 'case.get("structural_status") not in'),
    ("N1-fidelity", "N1 guard removed: case model_matrix_fidelity", CP,
     'not isinstance(case.get("model_matrix_fidelity"), str) or case.get("model_matrix_fidelity") not in', 'case.get("model_matrix_fidelity") not in'),
    ("N1-accuracy", "N1 guard removed: case accuracy_evidence", CP,
     'not isinstance(case.get("accuracy_evidence"), str) or case.get("accuracy_evidence") not in', 'case.get("accuracy_evidence") not in'),
    # RV108 N2.
    ("N2-removed", "N2 check removed: the verdict read by key again", PY,
     'verdict = quality[i]["solve_quality"] if "solve_quality" in quality[i] else None', 'verdict = quality[i]["solve_quality"]'),
]


def run_tests(label):
    junit = S / "mutants" / f"{label}.xml"
    env = dict(os.environ, TMPDIR=str(S / "tmp" / "mut"))
    cmd = [os.environ["VENV"] + "/bin/python", "-m", "pytest", "-p", "no:cacheprovider", f"--basetemp={S}/tmp/mut/{label}", "-q",
           f"--junitxml={junit}", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_carriers.py"]
    proc = subprocess.run(cmd, cwd=ROOT, env=env, capture_output=True, text=True)
    failed, total = [], 0
    for tc in ET.parse(junit).getroot().iter("testcase"):
        total += 1
        if tc.find("failure") is not None or tc.find("error") is not None:
            failed.append(tc.get("name"))
    return proc.returncode, total, failed


def main():
    (S / "mutants").mkdir(exist_ok=True)
    (S / "tmp" / "mut").mkdir(parents=True, exist_ok=True)
    plan = [("N0", "no mutation (head)", None, None, None)] + MUTANTS + [("BASE", "head tests with the base reader and compatibility module", "BASE", None, None)]
    with OUT.open("a") as out:
        for mid, what, rel, old, new in plan:
            if ONLY and mid not in ONLY:
                continue
            saved = {}
            if rel == "BASE":
                for r in (PY, CP):
                    saved[r] = (ROOT / r).read_bytes()
                    (ROOT / r).write_bytes((S / "base/projects/chirality-piping" / r).read_bytes())
            elif rel is not None:
                saved[rel] = (ROOT / rel).read_bytes()
                text = saved[rel].decode()
                assert text.count(old) == 1, (mid, text.count(old))
                (ROOT / rel).write_text(text.replace(old, new))
            try:
                rc, total, failed = run_tests(mid)
            finally:
                for r, raw in saved.items():
                    (ROOT / r).write_bytes(raw)
            row = {"id": mid, "mutant": what, "file": rel, "rc": rc, "tests": total, "failed": failed,
                   "outcome": ("passes" if not failed else "fails") if mid in ("N0",) else ("killed" if failed else "SURVIVES")}
            out.write(json.dumps(row) + "\n"); out.flush()
            print(mid, row["outcome"], len(failed), failed[:6], flush=True)


main()
