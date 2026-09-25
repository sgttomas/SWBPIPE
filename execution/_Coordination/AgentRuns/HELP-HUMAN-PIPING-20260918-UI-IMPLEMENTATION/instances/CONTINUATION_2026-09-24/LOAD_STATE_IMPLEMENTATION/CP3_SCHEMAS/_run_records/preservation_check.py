"""Path-preservation check for the three edited v0.3 carrier schemas.

Run from WORKING_ROOT:  <python> <this file>
Baseline = `git show HEAD:<path>` (HEAD bytes equal the recorded pre-edit sha256).
Every JSON pointer of the baseline must exist in the edited file with an equal
value, except lists whose last key is `enum`, `oneOf` or `anyOf`: those may only
be extended by appending (baseline list is a strict prefix). Object key sets
must be unchanged. Prints every extended pointer with its appended members.
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path.cwd()
PRE_EDIT = {
    "schemas/results.v0.3.schema.yaml": "a527f083dabc9e64ab7103e16c51103e85b6a67f5cef1427d1e8e908d7f5cc9f",
    "schemas/analysis_run.v0.3.schema.json": "f81f1c8e04ba3ecce5da51d72f303d41f6d710b26ea1a77f9163cfe94cae6abe",
    "schemas/stress_neutral_export.v0.3.schema.json": "af3d229289c72349f2daa5b7826bd7f0be4924d6b854c6358d480c89d1d991dd",
}
EXTENSIBLE = {"enum", "oneOf", "anyOf"}


def escape(key):
    return str(key).replace("~", "~0").replace("/", "~1")


def compare(old, new, pointer, failures, extended, count):
    count[0] += 1
    if type(old) is not type(new):
        failures.append(f"{pointer}: type {type(old).__name__} -> {type(new).__name__}")
        return
    if isinstance(old, dict):
        if set(old) != set(new):
            failures.append(f"{pointer}: keys added {sorted(set(new) - set(old))} removed {sorted(set(old) - set(new))}")
        for key in old:
            if key in new:
                compare(old[key], new[key], f"{pointer}/{escape(key)}", failures, extended, count)
    elif isinstance(old, list):
        last = pointer.rsplit("/", 1)[-1]
        if len(new) != len(old):
            if last in EXTENSIBLE and len(new) > len(old):
                extended.append((pointer, new[len(old):]))
            else:
                failures.append(f"{pointer}: length {len(old)} -> {len(new)}")
                return
        for index, item in enumerate(old):
            compare(item, new[index], f"{pointer}/{index}", failures, extended, count)
    elif old != new:
        failures.append(f"{pointer}: {old!r} -> {new!r}")


status = 0
for name, expected in PRE_EDIT.items():
    baseline = subprocess.run(["git", "show", f"HEAD:projects/chirality-piping/{name}"], cwd=ROOT,
                              check=True, capture_output=True).stdout
    assert hashlib.sha256(baseline).hexdigest() == expected, f"{name}: HEAD differs from the recorded pre-edit bytes"
    edited = (ROOT / name).read_bytes()
    old, new = json.loads(baseline), json.loads(edited)
    failures, extended, count = [], [], [0]
    compare(old, new, "", failures, extended, count)
    print(f"== {name}")
    print(f"   pre-edit sha256  {expected}")
    print(f"   post-edit sha256 {hashlib.sha256(edited).hexdigest()}")
    print(f"   baseline pointers compared: {count[0]}; failures: {len(failures)}; extended lists: {len(extended)}")
    for pointer, added in extended:
        summary = [item if isinstance(item, str) else ("<branch/ref " + json.dumps(item)[:90] + "...>") for item in added]
        print(f"   EXTENDED {pointer} += {summary}")
    for failure in failures:
        print(f"   FAIL {failure}")
    status |= bool(failures)
print("PRESERVATION", "FAIL" if status else "PASS")
sys.exit(status)
