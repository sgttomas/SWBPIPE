#!/usr/bin/env python3
"""Operands and frozen expectations for tests/timoshenko_mass.rs.

Source, read-only: validation/references/modal_timoshenko_mass/references.json (DEL-04-09 AC-012). The
operands are each case's binary64 bit patterns, verbatim. The expectations are the reference's
double-double pairs of Φ_y, Φ_z and the global 12x12 upper triangle; nothing is recomputed here.

Usage (from the repository root):
  python3 -I -B core/solver/modal_eigen/tests/fixtures/timoshenko_mass_cases.py          # check
  python3 -I -B core/solver/modal_eigen/tests/fixtures/timoshenko_mass_cases.py --write  # rewrite

Records, one per line:
  source <file> <sha256>
  case <id>
  point <xi | xj | y_reference> <x bits> <y bits> <z bits>
  op <E | G | A_s | I_y | I_z | mbar | ibar | jbar> <bits>
  phi <y | z> <hi> <lo>
  g <r> <c> <hi> <lo>          global 12x12, upper nonzero
  end
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
SOURCE = "validation/references/modal_timoshenko_mass/references.json"
OUT = HERE / "timoshenko_mass_cases.txt"


def build():
    raw = (ROOT / SOURCE).read_bytes()
    doc = json.loads(raw)
    lines = [f"source {SOURCE} {hashlib.sha256(raw).hexdigest()}"]
    for case in doc["cases"]:
        ops = case["operands"]
        lines.append(f"case {case['id']}")
        for name in ("xi", "xj", "y_reference"):
            lines.append(f"point {name} " + " ".join(ops[name]["bits"]))
        for name in ("E", "G", "A_s", "I_y", "I_z", "mbar", "ibar", "jbar"):
            lines.append(f"op {name} {ops[name]['bits']}")
        for axis in ("y", "z"):
            hi, lo = case[f"phi_{axis}"][1]
            lines.append(f"phi {axis} {hi} {lo}")
        for r, c, _decimal, (hi, lo) in case["global_upper"]:
            lines.append(f"g {r} {c} {hi} {lo}")
        lines.append("end")
    return ("\n".join(lines) + "\n").encode("ascii")


def main():
    data = build()
    if "--write" in sys.argv[1:]:
        OUT.write_bytes(data)
        print(f"wrote {OUT.name}")
        return 0
    if not OUT.is_file() or OUT.read_bytes() != data:
        print(f"{OUT.name} differs from a fresh build", file=sys.stderr)
        return 1
    print(f"{OUT.name} matches a fresh build")
    return 0


if __name__ == "__main__":
    sys.exit(main())
