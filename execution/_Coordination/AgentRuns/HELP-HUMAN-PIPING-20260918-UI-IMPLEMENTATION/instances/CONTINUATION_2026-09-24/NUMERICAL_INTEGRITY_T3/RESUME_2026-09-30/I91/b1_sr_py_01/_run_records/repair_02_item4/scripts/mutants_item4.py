"""I91 SR-PY mutants: one textual edit each to a copy of the head tree, then the two retained test files.

Repair 02, item 4 (2843a59a16): each mutant on the B1, repair, N1 and item-4 tests
(-k "b1_ or rv108_n1 or repair02_transport"); N0 and PRE on both files in full. PRE here is the head
tests with repair 02's reader before item 4 (70d4a68bd7).
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
    # Item 4: the transport header check at G2, the metadata check at G7.
    ("T-header-skipped", "transport: the base header check skipped (only the preview-physics metadata check runs)", PY,
     '    from .compatibility import _source_contract\n    try:_source_contract(projected,check_receipt=False)\n',
     '    from .preview_physics_evidence import validate_transport_metadata as _metadata_only\n    try:_metadata_only(projected)\n'),
    ("T-g7-label-kept", "transport: every base-step failure keeps the G7 label (the pre-item-4 gate)", PY,
     'RetainedPrecisionError("G7" if code in ("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID","SOURCE_PREVIEW_PHYSICS_INVALID") else "G2",code)',
     'RetainedPrecisionError("G7",code)'),
    ("T-metadata-at-g2", "transport: the metadata check's code reported at G2 with the header's", PY,
     'code in ("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID","SOURCE_PREVIEW_PHYSICS_INVALID") else "G2"',
     'code in ("SOURCE_PREVIEW_PHYSICS_INVALID",) else "G2"'),
]


FULL = {"N0", "PRE"}


def run_tests(label):
    junit = S / "mutants_item4" / f"{label}.xml"
    env = dict(os.environ, TMPDIR=str(S / "tmp" / "mut"))
    # Each pytest run is its own T3 slot job (WT/tools/t3_slot.sh; ROOT's host rules), with I91's own CLI builds.
    cmd = [os.environ["WT"] + "/tools/t3_slot.sh", os.environ["VENV"] + "/bin/python", "-m", "pytest", "-p", "no:cacheprovider", f"--basetemp={S}/tmp/mut/{label}", "-q",
           f"--junitxml={junit}", "tests/test_retained_precision_contract.py", "tests/test_retained_precision_carriers.py"]
    if label not in FULL:
        cmd += ["-k", "b1_ or rv108_n1 or repair02_transport"]
    proc = subprocess.run(cmd, cwd=ROOT, env=env, capture_output=True, text=True)
    failed, total = [], 0
    for tc in ET.parse(junit).getroot().iter("testcase"):
        total += 1
        if tc.find("failure") is not None or tc.find("error") is not None:
            failed.append(tc.get("name"))
    return proc.returncode, total, failed


def main():
    (S / "mutants_item4").mkdir(exist_ok=True)
    (S / "tmp" / "mut").mkdir(parents=True, exist_ok=True)
    plan = [("N0", "no mutation (head)", None, None, None)] + MUTANTS + [("PRE", "head tests with repair 02's reader before item 4 (70d4a68bd7)", "PRE", None, None)]
    with OUT.open("a") as out:
        for mid, what, rel, old, new in plan:
            if ONLY and mid not in ONLY:
                continue
            saved = {}
            if rel == "PRE":
                saved[PY] = (ROOT / PY).read_bytes()
                (ROOT / PY).write_bytes((S / "r02_head_retained_precision.py").read_bytes())
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
