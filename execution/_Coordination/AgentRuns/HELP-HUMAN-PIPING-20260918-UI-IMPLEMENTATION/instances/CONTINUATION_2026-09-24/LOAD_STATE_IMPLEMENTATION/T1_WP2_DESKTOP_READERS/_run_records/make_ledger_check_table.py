"""Check-for-check table of the invocation-free ledger port (T1 WP2 RETURN).

Lists every check Python source_blocks._validate_source_blocks(doc, None,
context=physics_source) can raise, in source order, with: the desktop line(s)
in loadReferenceSourceEvidence.ts raising the same code, the ledger cases whose
Python outcome is that code, and the mutation status of each desktop site.
Run from projects/chirality-piping:
    python <this> <mutation_results.json>
"""
from __future__ import annotations
import json, re, sys
from pathlib import Path

PY = Path("core/analysis_runs/source_blocks.py").read_text().splitlines()
PS = Path("core/analysis_runs/physics_source.py").read_text().splitlines()
TS = Path("apps/desktop/src/features/results/loadReferenceSourceEvidence.ts").read_text().splitlines()
CASES = json.loads(Path("apps/desktop/src/features/results/loadReferenceSourceLedger.cases.json").read_text())
MUT = {(m["line"]): m for m in json.loads(Path(sys.argv[1]).read_text()) if m["file"].endswith("loadReferenceSourceEvidence.ts") and m["group"] == "L"}
CODE = re.compile(r'"([A-Z][A-Z0-9_]+)"\)')
# Evaluated only under `actual_invocation`, `model`, `actual` or `invocation`
# not None, which the invocation-free route never supplies (checked by hand
# against the Python source; listed so the table shows every Python check).
INVOCATION_ONLY = {"ACTUAL_INVOCATION_SHAPE", "INVOCATION_HASH", "REQUESTED_MODE", "CURRENT_MODEL_SOURCE_COVERAGE", "CURRENT_MODEL",
                   "CURRENT_MODEL_IDS", "REQUESTED_CASE_COVERAGE", "SUPPORT_NODE", "ACTUAL_SUPPORT_LAW", "NODAL_MAGNITUDE_COVERAGE",
                   "NODAL_PRIMARY_COVERAGE", "ACTUAL_EXACT_PROFILE", "ACTUAL_EMPTY_PRESSURE_INVENTORY", "ACTUAL_SOURCE_FAMILY",
                   "ACTUAL_NODAL_LOADS", "ACTUAL_SUPPORT_FAMILY"}


def region(lines, start, stop_prefixes):
    begin = next(i for i, l in enumerate(lines) if l.startswith(start))
    end = next((i for i in range(begin + 1, len(lines)) if any(lines[i].startswith(p) for p in stop_prefixes)), len(lines))
    return begin, end


def codes(lines, begin, end, prefix):
    out = []
    for i in range(begin, end):
        for code in CODE.findall(lines[i]):
            if re.search(r"_require|_unique|_fail|need|raise", lines[i]) or code in INVOCATION_ONLY:
                out.append((code, f"{prefix}:{i + 1}"))
    return out


# Python: the helpers _validate_source_blocks calls live between _require and validate_source_blocks.
b, e = region(PY, "def _require", ["def validate_source_blocks"])
python = codes(PY, b, e, "source_blocks.py")
b, e = region(PS, "def validate_source_case", ["def ", "@"])
python += codes(PS, b, e, "physics_source.py")
seen, rows = set(), []
for code, where in python:
    key = (code, where)
    if key in seen:
        continue
    seen.add(key)
    ts = sorted({next(j + 1 for j in range(i, max(-1, i - 6), -1) if re.search(r"\b(blocks|composite|unique)\(", TS[j]))
                 for i, l in enumerate(TS) if f'"{code}")' in l and any(re.search(r"\b(blocks|composite|unique)\(", TS[j]) for j in range(i, max(-1, i - 6), -1))})
    cases = [c["id"] for c in CASES if c["python"].endswith("_" + code)]
    muts = [f"{line}:{'killed' if MUT[line]['killed'] else 'survived'}" for line in ts if line in MUT]
    note = "invocation/model-only: not evaluated without an invocation; not ported" if code in INVOCATION_ONLY else ""
    rows.append((code, where, ",".join(map(str, ts)) or "-", ",".join(cases) or "-", ",".join(muts) or "-", note))
print("| Python check | Python site | Desktop line(s) | Ledger case(s) reaching it | Mutant(s) | Note |")
print("|---|---|---|---|---|---|")
for r in rows:
    print("| " + " | ".join(r) + " |")
