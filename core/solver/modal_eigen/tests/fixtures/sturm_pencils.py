#!/usr/bin/env python3
"""Formed pencils for the Sturm-count tests (tests/sturm_counts.rs).

Two frozen sources, both read-only:

1. validation/references/f1_modal/continuum/inertia_witnesses.json: the
   inertia-risk witnesses. Their K and M are binary64 inputs, copied here as
   hex, with each shift's exact count below and exact count equal.

2. The frozen class (a) references in validation/references/f1_modal/mass_discrete
publish, per case, the model and exact Sturm counts at stated binary64 shifts,
but not the formed matrices. This script forms K_ff and M_ff for a few of
those cases with the references' own generator (imported read-only; nothing
there is written or changed), rounds each entry once to the nearest binary64,
and copies the published shifts and counts verbatim from eigen_references.json.

Exact zeros: the generator's 80-digit point field leaves residues of about
1e-80 relative where an entry is exactly zero (rotations of inclined
members). An entry at or below 1e-50 of its matrix's largest magnitude is
written as zero, the generator's own flush level (FLUSH = 1e-50).

Usage (from the repository root):
  python3 -I -B core/solver/modal_eigen/tests/fixtures/sturm_pencils.py          # check
  python3 -I -B core/solver/modal_eigen/tests/fixtures/sturm_pencils.py --write  # rewrite

Output format (sturm_pencils.txt), one record per line:
  source <file> <sha256>
  witness <id> <n>             or
  case <id> <n> <n_finite_eigenvalues>
  k <row> <col> <hex>          lower triangle, row >= col, nonzero entries
  m <row> <col> <hex>
  shift <kind> <sigma hex> <exact count below> <exact count equal>
  end
Witness shift kinds are the reference's labels with spaces replaced by '_'.
"""
import hashlib
import json
import sys
from decimal import Decimal
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
REF = ROOT / "validation/references/f1_modal/mass_discrete"
WIT = ROOT / "validation/references/f1_modal/continuum/inertia_witnesses.json"
sys.path.insert(0, str(REF / "generators"))

from model import Model, form_mass, form_stiffness, reduce  # noqa: E402
from numeric import PointField, set_point_precision  # noqa: E402

CASES = ["E-AXIAL-WITNESS", "E-CANT-4", "E-SS-8", "E-FF-4", "E-POINTMASS-GENERAL",
         "E-L-LINE-K3", "E-Z-BEND-K1", "E-RIGID-SCALE-1e6", "E-SPRINGS"]
POINT_PREC = 80
FLUSH = Decimal("1e-50")
OUT = HERE / "sturm_pencils.txt"


def lower(A, tag):
    big = max(abs(x) for row in A for x in row)
    lines = []
    for i, row in enumerate(A):
        for j in range(i + 1):
            x = row[j]
            if abs(x) <= FLUSH * big:
                continue
            lines.append(f"{tag} {i} {j} {float(x).hex()}")
    return lines


def b64(text):
    value = float(text)
    assert repr(value) == text or float(repr(value)) == value
    return value


def witness_lines():
    source = WIT.read_bytes()
    lines = [f"source continuum/inertia_witnesses.json {hashlib.sha256(source).hexdigest()}"]
    for w in json.loads(source)["witnesses"]:
        if w["id"] == "WIT-ROUND2":
            n = w["n"]
            K = [[0.0] * n for _ in range(n)]
            M = [[0.0] * n for _ in range(n)]
            for i in range(n):
                K[i][i] = b64(w["K_diagonal"][i])
                M[i][i] = float.fromhex(w["M_diagonal"][i]["hex"])
            for i in range(n - 1):
                K[i + 1][i] = b64(w["K_offdiagonal"][i])
            assert K[1][0] == float.fromhex(w["K_offdiagonal_exact"][0]["hex"])
            shifts = [(p["probe"], p["shift"]["hex"], p["exact_count_below"], 0) for p in w["probes"]]
        else:
            K = [[b64(x) for x in row] for row in w["K"]]
            M = [[b64(x) for x in row] for row in w["M"]]
            shifts = [(f"{s['shift']['binary64']}", s["shift"]["hex"], s["exact_count_below"],
                       s["exact_count_equal"]) for s in w["shifts"]]
        n = len(K)
        for i in range(n):
            for j in range(i):
                assert K[i][j] == K[j][i] or w["id"] == "WIT-ROUND2"
        lines.append(f"witness {w['id']} {n}")
        for tag, A in (("k", K), ("m", M)):
            for i in range(n):
                for j in range(i + 1):
                    if A[i][j] != 0.0:
                        lines.append(f"{tag} {i} {j} {A[i][j].hex()}")
        for label, hx, below, equal in shifts:
            label = label.replace(" ", "_")
            lines.append(f"shift {label} {float.fromhex(hx).hex()} {below} {equal}")
        lines.append("end")
    return lines


def build():
    source = (REF / "eigen_references.json").read_bytes()
    refs = {c["id"]: c for c in json.loads(source)["cases"]}
    set_point_precision(POINT_PREC)
    P = PointField()
    lines = witness_lines()
    lines.append(f"source mass_discrete/eigen_references.json {hashlib.sha256(source).hexdigest()}")
    for cid in CASES:
        case = refs[cid]
        model = Model(case["model"])
        free = model.free
        K = reduce(form_stiffness(model, P), free)
        M = reduce(form_mass(model, P, "PF")[0], free)
        assert len(free) == case["n_free"]
        lines.append(f"case {cid} {len(free)} {case['n_finite_eigenvalues']}")
        lines += lower(K, "k") + lower(M, "m")
        for s in case["sturm_counts"]:
            assert float.fromhex(s["sigma_hex"]) == s["sigma_rad2_per_s2"]
            lines.append(f"shift {s['kind']} {s['sigma_hex']} {s['count']} 0")
        lines.append("end")
    return "\n".join(lines) + "\n"


def main():
    text = build()
    if "--write" in sys.argv[1:]:
        OUT.write_text(text)
        print(f"wrote {OUT}")
        return 0
    if OUT.read_text() != text:
        print(f"{OUT} differs from a fresh build", file=sys.stderr)
        return 1
    print("sturm_pencils.txt matches a fresh build")
    return 0


if __name__ == "__main__":
    sys.exit(main())
