#!/usr/bin/env python3
"""Verify declared exact signatures in captured openpipestress-runner outputs.

This verifier does not run product code. Invoke it only after candidate-bound
CLI outputs exist:

  python3 verify_declared_signatures.py EXPECTED_SIGNATURES_V3.json \
    hanger_constant_effort=/path/hanger.json \
    rotational_nonlinear=/path/rotational.json \
    zero_pressure_longitudinal=/path/pressure.json
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(f"FAIL: {message}", file=sys.stderr)


def main(argv: list[str]) -> int:
    if len(argv) < 3:
        fail("expected mapping JSON and one or more case=output.json arguments")
        return 2
    mapping = json.loads(Path(argv[1]).read_text())
    supplied: dict[str, Path] = {}
    for raw in argv[2:]:
        if "=" not in raw:
            fail(f"invalid output binding {raw!r}; expected case=path")
            return 2
        case, path = raw.split("=", 1)
        supplied[case] = Path(path)

    failures = 0
    for case, path in supplied.items():
        declaration = mapping.get("cases", {}).get(case)
        if declaration is None:
            fail(f"undeclared case {case}")
            failures += 1
            continue
        document = json.loads(path.read_text())
        envelope = document.get("mechanics_envelope")
        if not isinstance(envelope, dict):
            fail(f"{case}: /mechanics_envelope is absent or not an object")
            failures += 1
            continue
        mechanics = envelope.get("status", {}).get("mechanics")
        expected_status = declaration.get("expected_mechanics_status")
        if mechanics != expected_status:
            fail(f"{case}: mechanics status {mechanics!r}, expected {expected_status!r}")
            failures += 1
        rows = envelope.get("results")
        if not isinstance(rows, list):
            fail(f"{case}: /mechanics_envelope/results is absent or not an array")
            failures += 1
            continue
        observed = {
            (row.get("kind"), row.get("unit"), (row.get("metadata") or {}).get("component"))
            for row in rows
            if isinstance(row, dict)
        }
        for expected in declaration.get("required_signatures", []):
            signature = (expected["kind"], expected["unit"], expected["component"])
            if signature not in observed:
                fail(f"{case}: missing {expected['signature_id']} {signature!r}")
                failures += 1
            else:
                print(f"PASS: {case}: {expected['signature_id']} {signature!r}")
    missing_cases = set(mapping.get("cases", {})) - set(supplied)
    if missing_cases:
        fail(f"no output supplied for declared cases: {', '.join(sorted(missing_cases))}")
        failures += len(missing_cases)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
