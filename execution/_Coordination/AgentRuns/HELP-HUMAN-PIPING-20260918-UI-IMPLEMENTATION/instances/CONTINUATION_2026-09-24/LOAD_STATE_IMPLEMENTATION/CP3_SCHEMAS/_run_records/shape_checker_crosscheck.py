"""Cross-check: the repository's Python closed-shape checker agrees with jsonschema on the new $defs.

Run from WORKING_ROOT:  <python> <this file>
Read-only use of core.analysis_runs.source_blocks._shape (the checker the physics
readers use for transport-shape checks), rooted at load_reference_state.schema.json.
This is run evidence, not a committed test (the checker is private to another lane).
"""
from __future__ import annotations

import copy
import glob
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path.cwd()
sys.path[:0] = [str(ROOT / "tests"), str(ROOT)]
from schema_validation import schema_for_definition, validate_instance  # noqa: E402
from core.analysis_runs.source_blocks import _shape  # noqa: E402

changed = subprocess.run(["git", "diff", "--quiet", "HEAD", "--", "core/analysis_runs/source_blocks.py"], cwd=ROOT).returncode
print("core/analysis_runs/source_blocks.py differs from HEAD:", bool(changed))
S = json.loads((ROOT / "schemas/load_reference_state.schema.json").read_text())


def both(name, instance):
    try:
        validate_instance(schema_for_definition(S, name), instance)
        js = True
    except AssertionError:
        js = False
    return js, _shape(instance, S["$defs"][name], S)


cases = []
for stem in ("connected", "pressure"):
    model = json.loads((ROOT / f"fixtures/product_preview/load_reference/{stem}.request.json").read_text())["model"]
    cases += [(f"{stem} reference", "ReferenceConfiguration", rc, True) for rc in model["reference_configurations"]]
    cases += [(f"{stem} laws", "MaterialExpansionLaws", m["expansion_laws"], True) for m in model["materials"]]
    cases += [(f"{stem} {c['id']}", "AnalysisState", c["analysis_state"], True) for c in model["load_cases"]]
for path in sorted(glob.glob(str(ROOT / "fixtures/product_preview/load_reference/*.raw.json"))):
    evidence = json.loads(Path(path).read_text())["contract_evidence"]
    cases.append((Path(path).stem, "LoadReferenceContractEvidence", evidence, True))
    bad = copy.deepcopy(evidence); bad["load_reference_states"][0]["members"][0]["extra"] = 1
    cases.append((Path(path).stem + " member extra key", "LoadReferenceContractEvidence", bad, False))
    bad = copy.deepcopy(evidence); bad["load_reference_states"][0]["source_recovery"]["status"] = "joined"
    cases.append((Path(path).stem + " status joined", "LoadReferenceContractEvidence", bad, False))
    bad = copy.deepcopy(evidence); del bad["load_reference_states"][0]["solve"]
    cases.append((Path(path).stem + " missing solve", "LoadReferenceContractEvidence", bad, False))
model = json.loads((ROOT / "fixtures/product_preview/load_reference/connected.request.json").read_text())["model"]
state = model["load_cases"][0]["analysis_state"]
bad = copy.deepcopy(state); bad["history"] = {"kind": "continuation"}
cases.append(("history continuation", "AnalysisState", bad, False))
bad = copy.deepcopy(model["reference_configurations"][0]); bad["member_references"][1]["fit"]["strain"] = {"value": 1, "unit": "1"}
cases.append(("fit length_change+strain", "ReferenceConfiguration", bad, False))
physics = json.loads((ROOT / "fixtures/results/physics_connected_mechanics_sparse.json").read_text())["contract_evidence"]
cases.append(("physics-1 evidence", "LoadReferenceContractEvidence", physics, False))

disagreements = 0
for title, name, instance, expected in cases:
    js, shape = both(name, instance)
    ok = js == shape == expected
    disagreements += not ok
    print(f"{'OK ' if ok else 'BAD'} expected={expected!s:5} jsonschema={js!s:5} _shape={shape!s:5}  {title}")
print("CROSSCHECK", "PASS" if disagreements == 0 else f"FAIL ({disagreements})")
sys.exit(1 if disagreements else 0)
