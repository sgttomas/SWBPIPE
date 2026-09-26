"""Path-preservation check for the load-reference-source-1 carrier edits.

Run from WORKING_ROOT:  <python> <this file> [<base git revision, default HEAD>]

For each carrier, every JSON pointer of the base document must keep its value in
the edited document. Allowed differences:
- a list may be extended by appending (its base prefix is unchanged);
- an object may gain keys (reported);
- the single documented wrap: AnalysisRun `properties/contract_evidence`, whose
  base value must equal the first `anyOf` member of the edited value.
Anything else is a failure. Exit status 1 on any failure.
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

BASE = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
CARRIERS = ("results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json",
            "stress_neutral_export.v0.3.schema.json", "analysis_run.schema.json")
WRAPS = {"analysis_run.v0.3.schema.json": {"/$defs/AnalysisRun/properties/contract_evidence"}}


def base_bytes(name):
    prefix = subprocess.run(["git", "rev-parse", "--show-prefix"], capture_output=True, text=True, check=True).stdout.strip()
    return subprocess.run(["git", "show", f"{BASE}:{prefix}schemas/{name}"], capture_output=True, check=True).stdout


def escape(key):
    return str(key).replace("~", "~0").replace("/", "~1")


def compare(old, new, pointer, report, wraps):
    report["pointers"] += 1
    if pointer in wraps:
        if isinstance(new, dict) and set(new) == {"anyOf"} and new["anyOf"] and new["anyOf"][0] == old:
            report["wrapped"].append((pointer, len(new["anyOf"])))
            return
        report["failures"].append(f"{pointer}: wrap does not keep the base value as anyOf[0]")
        return
    if type(old) is not type(new):
        report["failures"].append(f"{pointer}: type changed")
        return
    if isinstance(old, dict):
        for key, value in old.items():
            if key not in new:
                report["failures"].append(f"{pointer}/{escape(key)}: removed")
                continue
            compare(value, new[key], f"{pointer}/{escape(key)}", report, wraps)
        for key in new:
            if key not in old:
                report["added_keys"].append(f"{pointer}/{escape(key)}")
    elif isinstance(old, list):
        if len(new) < len(old):
            report["failures"].append(f"{pointer}: shortened")
            return
        for index, value in enumerate(old):
            compare(value, new[index], f"{pointer}/{index}", report, wraps)
        if len(new) > len(old):
            report["appended"].append((pointer, len(old), len(new)))
    elif old != new:
        report["failures"].append(f"{pointer}: value changed")


failed = False
for name in CARRIERS:
    old = json.loads(base_bytes(name))
    new = json.loads(Path("schemas", name).read_text(encoding="utf-8"))
    report = {"pointers": 0, "failures": [], "appended": [], "added_keys": [], "wrapped": []}
    compare(old, new, "", report, WRAPS.get(name, set()))
    status = "PASS" if not report["failures"] else "FAIL"
    failed |= bool(report["failures"])
    print(f"== {name}: {status}; base pointers compared {report['pointers']}; failures {len(report['failures'])}")
    for pointer, before, after in report["appended"]:
        print(f"   appended {pointer}: {before} -> {after}")
    for pointer in report["added_keys"]:
        print(f"   added key {pointer}")
    for pointer, members in report["wrapped"]:
        print(f"   WRAP (only non-append change) {pointer}: base value kept as anyOf[0] of {members}")
    for failure in report["failures"]:
        print(f"   FAIL {failure}")
print("OVERALL", "FAIL" if failed else "PASS")
sys.exit(1 if failed else 0)
