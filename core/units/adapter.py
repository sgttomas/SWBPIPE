"""Configured subprocess bridge to the existing Rust units authority.

No runtime build, PATH search, shell, conversion formula or fallback catalog.
The caller supplies dimensions/units explicitly; the Rust library selects the
canonical target and performs the same conversion used by the producer.
"""
from __future__ import annotations
from collections.abc import Mapping, Sequence
import json
import math
import os
from pathlib import Path
import subprocess
from typing import Any

ENV_EXECUTABLE = "OPENPIPESTRESS_UNITS_BIN"
PROTOCOL_VERSION = "1.0.0"

def units_executable() -> Path:
    configured = os.environ.get(ENV_EXECUTABLE)
    if not configured or not Path(configured).is_absolute():
        raise RuntimeError("UNITS-AUTHORITY-MISSING: set OPENPIPESTRESS_UNITS_BIN to an explicitly built absolute executable path")
    path = Path(configured)
    if not path.is_file():
        raise RuntimeError(f"UNITS-AUTHORITY-MISSING: {path}")
    return path

def convert_quantities_to_canonical(quantities: Sequence[Mapping[str, Any]]) -> list[dict[str, Any]]:
    rows=[]; ids=set()
    for item in quantities:
        if not isinstance(item, Mapping) or set(item) != {"id", "value", "unit", "dimension"}:
            raise ValueError("UNITS-REQUEST-SHAPE")
        q=dict(item)
        if any(not isinstance(q[k], str) or not q[k] for k in ("id","unit","dimension")) or q["id"] in ids:
            raise ValueError("UNITS-REQUEST-IDENTITY")
        try:
            finite = type(q["value"]) in (int, float) and math.isfinite(q["value"])
        except OverflowError:
            finite = False
        if not finite:
            raise ValueError("UNITS-REQUEST-NONFINITE")
        ids.add(q["id"]);rows.append(q)
    payload=json.dumps({"protocol_version":PROTOCOL_VERSION,"quantities":rows},allow_nan=False,separators=(",",":"))
    result=subprocess.run([str(units_executable())],input=payload,text=True,capture_output=True,check=False,timeout=30)
    try: response=json.loads(result.stdout)
    except (ValueError, TypeError) as error: raise RuntimeError("UNITS-AUTHORITY-RESPONSE-INVALID") from error
    if result.returncode:
        raise ValueError(str(response.get("error", "UNITS-AUTHORITY-REFUSED")))
    if not isinstance(response,dict) or set(response)!={"protocol_version","quantities"} or response["protocol_version"]!=PROTOCOL_VERSION or not isinstance(response["quantities"],list) or len(response["quantities"])!=len(rows):
        raise RuntimeError("UNITS-AUTHORITY-RESPONSE-INVALID")
    for actual, expected in zip(response["quantities"],rows):
        if not isinstance(actual,dict) or set(actual)!={"id","value","unit","dimension"} or actual["id"]!=expected["id"] or actual["dimension"]!=expected["dimension"] or not isinstance(actual["unit"],str) or not actual["unit"] or type(actual["value"]) not in (int,float) or not math.isfinite(actual["value"]):
            raise RuntimeError("UNITS-AUTHORITY-RESPONSE-INVALID")
    return response["quantities"]
