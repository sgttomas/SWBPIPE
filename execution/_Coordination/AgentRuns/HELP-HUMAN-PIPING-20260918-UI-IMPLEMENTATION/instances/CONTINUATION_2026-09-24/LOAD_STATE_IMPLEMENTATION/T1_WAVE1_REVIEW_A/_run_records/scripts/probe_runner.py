"""Reviewer probe runner, Python side. Usage: probe_runner.py <WORKING_ROOT> <PROBE_DIR> <OUT>."""
import json, pathlib, sys
root = pathlib.Path(sys.argv[1]).resolve()
sys.path.insert(0, str(root))
from core.analysis_runs.compatibility import _source_contract, numerical_use_standing
from core.analysis_runs.load_reference_source import (
    validate_load_reference_source_evidence, validate_load_reference_source_transport_metadata)


def outcome(call):
    try:
        call()
    except ValueError as e:
        return str(e)
    except Exception as e:  # noqa: BLE001 - record any non-ValueError escape
        return "PY_EXCEPTION:" + type(e).__name__ + ":" + str(e)[:120]
    return "accept"


lines = []
for p in sorted(pathlib.Path(sys.argv[2]).glob("*.json")):
    text = p.read_text()
    try:
        v = json.loads(text)
    except ValueError:
        lines.append({"id": p.name, "dispatch": "JSON_PARSE_REJECTED"})
        continue
    meta = {k: v[k] for k in ["schema_version", "producer", "numerical_quality", "formulation_basis",
                              "contract_evidence", "source_block_recovery", "carrier_evidence"] if isinstance(v, dict) and k in v}
    refs = [c.get("basis_ref") for c in v.get("numerical_quality", {}).get("cases", [])] if isinstance(v, dict) else []
    try:
        standing = numerical_use_standing(v, refs)
    except Exception as e:  # noqa: BLE001
        standing = "PY_EXCEPTION:" + type(e).__name__
    lines.append({"id": p.name,
                  "dispatch": outcome(lambda: _source_contract(v)),
                  "validator": outcome(lambda: validate_load_reference_source_evidence(v)),
                  "transport": outcome(lambda: validate_load_reference_source_transport_metadata(meta)),
                  "standing": standing})
pathlib.Path(sys.argv[3]).write_text("\n".join(json.dumps(l) for l in lines) + "\n")
