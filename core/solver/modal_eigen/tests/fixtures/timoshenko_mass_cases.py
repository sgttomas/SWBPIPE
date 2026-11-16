#!/usr/bin/env python3
"""Operands and frozen expectations for tests/timoshenko_mass.rs.

Sources, read-only, in validation/references/modal_timoshenko_mass (DEL-04-09 AC-012): the source-blind
derivation's two outputs, timoshenko_consistent_mass.json (the frozen problem statement's five cases) and
timoshenko_consistent_mass_addendum1.json (Addendum 1's three cases). The operands are each case's binary64
bit patterns, verbatim. The expectations are the references' double-double pairs of Φ_y, Φ_z (where the
reference gives a pair; a Φ outside binary64 is given there as an exact rational and is not carried here)
and of the global 12x12 upper triangle; nothing is recomputed here.

Usage (from the repository root):
  python3 -I -B core/solver/modal_eigen/tests/fixtures/timoshenko_mass_cases.py          # check
  python3 -I -B core/solver/modal_eigen/tests/fixtures/timoshenko_mass_cases.py --write  # rewrite

Records, one per line:
  source <file> <sha256>
  case <id>
  point <x_i | x_j | v> <x bits> <y bits> <z bits>
  op <E | G | A_s | I_y | I_z | mbar | ibar | jbar> <bits>
  phi <y | z> <hi> <lo>        only where the reference gives a pair
  g <r> <c> <hi> <lo>          global 12x12, upper nonzero, diagonal included
  end
"""
import hashlib
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
PACKAGE = "validation/references/modal_timoshenko_mass"
SOURCES = (f"{PACKAGE}/timoshenko_consistent_mass.json", f"{PACKAGE}/timoshenko_consistent_mass_addendum1.json")
OUT = HERE / "timoshenko_mass_cases.txt"


def build():
    lines = []
    cases = []
    for source in SOURCES:
        raw = (ROOT / source).read_bytes()
        lines.append(f"source {source} {hashlib.sha256(raw).hexdigest()}")
        cases.extend(json.loads(raw)["cases"])
    for case in cases:
        ops = case["operands"]
        lines.append(f"case {case['id']}")
        for name in ("x_i", "x_j", "v"):
            lines.append(f"point {name} " + " ".join(ops[name]))
        for name in ("E", "G", "A_s", "I_y", "I_z", "mbar", "ibar", "jbar"):
            lines.append(f"op {name} {ops[name]}")
        for axis in ("y", "z"):
            phi = case[f"Phi_{axis}"]
            if "hi" in phi:
                lines.append(f"phi {axis} {phi['hi']} {phi['lo']}")
        for entry in case["global_upper_nonzero"]:
            lines.append(f"g {entry['r']} {entry['c']} {entry['hi']} {entry['lo']}")
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
