"""I91 SR-PY mutants: one textual edit each to a copy of the head tree, then the two retained test files.

Repair 02: each mutant on the B1, repair and N1 tests (-k "b1_ or rv108_n1"); N0 and PRE on both files in full.
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
    # Item 1: G3's receipt references, and the ordinary attempt's basis at G5.
    ("F-mb-index", "G3: a material basis's index unchecked (the new check)", PY,
     '_need(mb["index"]==mi and len(set(', '_need(True and len(set('),
    ("F-mb-unique", "G3: a basis's case list may repeat a case", PY,
     'and len(set(mb["case_indices"]))==len(mb["case_indices"]) and all(', 'and all('),
    ("F-mb-range", "G3: a basis's case list may name a case out of range", PY,
     ' and all(x<len(cases) for x in mb["case_indices"]),gate,"COVERAGE_MISMATCH")', ',gate,"COVERAGE_MISMATCH")'),
    ("F-src-index", "G3: a source's index unchecked (moved from G8)", PY,
     '_need(s["index"]==si and s["owner"]["kind"]=="case"', '_need(s["owner"]["kind"]=="case"'),
    ("F-src-kind", "G3: a source's owner kind unchecked (moved from G8)", PY,
     ' and s["owner"]["kind"]=="case" and ci is not None', ' and ci is not None'),
    ("F-src-range", "G3: a source's owner index unchecked for range", PY,
     ' and 0<=ci<len(cases) and ids[ci]==', ' and ids[ci]=='),
    ("F-src-id", "G3: a source's owner id unchecked against its case (moved from G8)", PY,
     ' and ids[ci]==s["owner"]["case_id"],gate,"COVERAGE_MISMATCH")', ',gate,"COVERAGE_MISMATCH")'),
    ("G5-basis-off", "G5: the ordinary attempt's basis reference unchecked", PY,
     '        fail(basis is not None and i in basis["case_indices"])\n', ''),
    ("G5-basis-resolve", "G5: the basis need only resolve, not list its case", PY,
     'fail(basis is not None and i in basis["case_indices"])', 'fail(basis is not None)'),
    # Item 2: the model scope at G8 INVOCATION.
    ("G-refconf", "G8: reference_configurations unchecked", PY,
     ' and "reference_configurations" not in model, "INVOCATION_MISMATCH")', ', "INVOCATION_MISMATCH")'),
    ("G-pressure-falsy", "G8: pressure_contract checked by falsiness (the pre-repair rule)", PY,
     'model.get("pressure_contract") is None', 'not model.get("pressure_contract")'),
    ("G-combinations-falsy", "G8: combinations checked by falsiness (the pre-repair rule)", PY,
     'model.get("combinations", []) == []', 'not model.get("combinations")'),
    ("G-components-falsy", "G8: components checked by falsiness (the pre-repair rule)", PY,
     'model.get("components", []) == []', 'not model.get("components")'),
    # Item 3: C2's cause table.
    ("C2-off", "G5: C2's cause table removed", PY,
     'if cause is not None and cause.get("kind") != "prepared_product_failure":', 'if False:'),
    ("C2-source-decline", "C2 source_error: the decline and its error unchecked", PY,
     '\n                     and c.get("source_decline") is not None and _same(c["source_decline"]["error"], cause["error"]))', ')'),
    ("C2-receipt-phase", "C2 receipt_failure: phase unchecked", PY,
     'fail(phase == "receipt" and code in RECEIPT_FAILURE_CODES)', 'fail(code in RECEIPT_FAILURE_CODES)'),
    ("C2-receipt-code", "C2 receipt_failure: code unchecked", PY,
     'fail(phase == "receipt" and code in RECEIPT_FAILURE_CODES)', 'fail(phase == "receipt")'),
    ("C2-facade-run", "C2 facade_failure: the selected Run unchecked", PY,
     ' and run is not None and run["kernel_terminal"]["kind"] == "selected"\n                     and _same(cause["owner_ref"]', '\n                     and _same(cause["owner_ref"]'),
    ("C2-facade-owner", "C2 facade_failure: the owner unchecked", PY,
     '\n                     and _same(cause["owner_ref"], {"kind": "case", "index": i}))', ')'),
    ("C2-precondition-set", "C2 unavailable_precondition: TS's set of four codes, not keyed", PY,
     'code == PRECONDITION_CODES.get(cause["precondition"])', 'code in set(PRECONDITION_CODES.values())'),
    ("C2-precondition-run", "C2 unavailable_precondition: a Run allowed", PY,
     'fail(phase in ("routing", "preparation") and run is None and code', 'fail(phase in ("routing", "preparation") and code'),
    ("C2-kernel-code", "C2 kernel reason: code unchecked", PY,
     ' and code == "kernel_" + run["kernel_terminal"]["kind"]\n', '\n'),
    ("C2-kernel-cause", "C2 kernel reason: the cause unchecked against the Run's terminal reason", PY,
     '\n                     and _same(cause, run["kernel_terminal"]["reason"]))', ')'),
    # Item 5 (RV113 S-4) and item 6 (N-2), and repair 01's count (now pinned by D16).
    ("Q17", "not_required: product_attempt_ref null dropped (RV113's Q17)", PY,
     'fail(c["product_attempt_ref"] is None and verdict == "checks_passed")', 'fail(verdict == "checks_passed")'),
    ("D38-capture", "(4b) without its capture-error conjunct", PY,
     'and a["result"]["kind"] == "unavailable" and a["result"]["error"]["kind"] == "capture"',
     'and a["result"]["kind"] == "unavailable"'),
    ("D38-reason", "(4b) without the reason (source_unavailable, preparation)", PY,
     'and (reason.get("code"), reason.get("phase")) == ("source_unavailable", "preparation")', 'and True'),
    ("D38-names-a", "(4b) without the cause naming this attempt", PY,
     'and cause.get("product_attempt_ref") == ai\n', 'and True\n'),
    ("D38-case-status", "(4b) without the case unavailable with prepared_product_failure", PY,
     'and case["status"] == "unavailable" and cause.get("kind") == "prepared_product_failure"', 'and True'),
    ("D38-no-run-no-proof", "(4b) without case.run null and proof null", PY,
     'return (case.get("run") is None and a["proof"] is None\n', 'return (True\n'),
    ("R-c-count", "(c) the basis count unchecked (repair 01)", PY,
     '    need(len(body["material_bases"]) == len(selectors))\n', ''),
]


FULL = {"N0", "PRE"}


def run_tests(label):
    junit = S / "mutants" / f"{label}.xml"
    env = dict(os.environ, TMPDIR=str(S / "tmp" / "mut"))
    # Each pytest run is its own T3 slot job (WT/tools/t3_slot.sh; ROOT's host rules), with I91's own CLI builds.
    cmd = [os.environ["WT"] + "/tools/t3_slot.sh", os.environ["VENV"] + "/bin/python", "-m", "pytest", "-p", "no:cacheprovider", f"--basetemp={S}/tmp/mut/{label}", "-q",
           f"--junitxml={junit}", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_carriers.py"]
    if label not in FULL:
        cmd += ["-k", "b1_ or rv108_n1"]
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
    plan = [("N0", "no mutation (head)", None, None, None)] + MUTANTS + [("PRE", "head tests with SR-PY's repair-01 reader (11cc14e3e6)", "PRE", None, None)]
    with OUT.open("a") as out:
        for mid, what, rel, old, new in plan:
            if ONLY and mid not in ONLY:
                continue
            saved = {}
            if rel == "PRE":
                saved[PY] = (ROOT / PY).read_bytes()
                (ROOT / PY).write_bytes((S / "repair01_retained_precision.py").read_bytes())
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
                   "outcome": ("passes" if not failed else "fails") if mid == "N0" else ("killed" if failed else "SURVIVES")}
            out.write(json.dumps(row) + "\n"); out.flush()
            print(mid, row["outcome"], len(failed), failed[:6], flush=True)


main()
