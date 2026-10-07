"""RV108: T6S decision 8 (Python refuses successor packages) at base and head."""
import json, sys
from copy import deepcopy
from pathlib import Path
PROOT = Path(sys.argv[1]); sys.path.insert(0, str(PROOT))
from core.handoff.stress_neutral import package_v0_3 as pk
from core.analysis_runs import compatibility as c
for mode in ("sparse_interactive", "dense_scrutiny"):
    src = json.loads((PROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())["source"]
    pkg = {k: deepcopy(src[k]) for k in ("producer", "numerical_quality", "formulation_basis", "contract_evidence")}
    variants = {"package_view": pkg, "package_view_with_receipt": dict(pkg, retained_precision=deepcopy(src["retained_precision"])),
                "full_successor_as_package": deepcopy(src)}
    for name, p in variants.items():
        for fn_name, fn in (("_transport_contract", lambda p: pk._transport_contract(p)), ("validate_stress_neutral_export_package_v0_3", lambda p: pk.validate_stress_neutral_export_package_v0_3(p)),
                            ("validate_..._v0_3(source_envelope=successor)", lambda p: pk.validate_stress_neutral_export_package_v0_3(p, source_envelope=deepcopy(src)))):
            try: r = fn(deepcopy(p)); out = f"ADMITTED {r!r}"[:120]
            except Exception as e: out = f"{type(e).__name__}: {e}"[:160]
            print(mode, name, fn_name, "->", out)
    # The transport dispatch on the full successor itself (what changed in B6):
    try: print(mode, "_source_contract(successor, check_receipt=False) ->", c._source_contract(deepcopy(src), check_receipt=False)[0])
    except Exception as e: print(mode, "_source_contract(successor, check_receipt=False) ->", type(e).__name__, e)
