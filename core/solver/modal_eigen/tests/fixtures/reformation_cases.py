#!/usr/bin/env python3
"""Operands and frozen expectations for the re-formation tests (tests/reformation.rs).

Sources, all read-only, in validation/references/f1_modal/mass_discrete:

1. mass_references.json: every mass case on both bases (PF and EX). The
   operands are the case's `model`, verbatim. The expectations are the
   published 40-digit decimals of the section quantities, the straight
   element matrices, the half-arc lumps, the component bodies and the
   assembled full M.

2. eigen_references.json: a few PF eigen cases. Their K_ff and M_ff are
   formed by the references' own generator at 80 digits (imported read-only;
   nothing there is written or changed), as sturm_pencils.py does, and the
   reported eigenvalues are copied from the file.

The generator also supplies the component-eligibility predicate and the
stiffness scale of each member (Model.body_of, Model.scale_of), which the
product supplies in I2.

Number formats: a binary64 value is its 16-hex-digit IEEE bit pattern. An
expectation is a double-double pair `hi lo`: hi is the decimal (or the
80-digit value) rounded once to binary64 and lo the remainder rounded once,
so hi + lo is within 2^-106 relative of the reference. hi alone is the
reference rounded once, which is what the eigen tests use as a formed
binary64 matrix. An 80-digit entry at or below 1e-50 of its matrix's largest
magnitude is written as zero (omitted), the generator's own flush level.

Usage (from the repository root):
  python3 -I -B core/solver/modal_eigen/tests/fixtures/reformation_cases.py          # check
  python3 -I -B core/solver/modal_eigen/tests/fixtures/reformation_cases.py --write  # rewrite

Records, one per line:
  source <file> <sha256>
  case <id> <basis> <n_nodes> <a | ->
  node <k> <x> <y> <z>
  straight <k> <i> <j> <y_ref x3> <section> <E> <G> <scale | -> <carries_mass 0|1>
  arc <k> <i> <j> <R> <y_ref x3> <section> <E> <G> <flexibility factor> <carries_mass 0|1>
      <section> = <OD> <t> <c_m> <c_a> <rho_m> <rho_c | -> <t_ins | -> <rho_ins | ->
  body <node> <W> <a> <r x3>
  spring <global dof> <k>
  section <member k> <quantity> <hi> <lo>
  emass <member k> <r> <c> <hi> <lo>         straight 12x12, upper nonzero, local
  lump <member k> <0 = node i | 1 = node j> <r> <c> <hi> <lo>   6x6 nonzero
  cmass <node> <r> <c> <hi> <lo>             component body 6x6 nonzero
  mfull <r> <c> <hi> <lo>                    assembled full M, upper nonzero, global
  free <global dofs ...>
  k <row> <col> <hi> <lo>                    K_ff lower nonzero (eigen cases)
  m <row> <col> <hi> <lo>                    M_ff lower nonzero (eigen cases)
  eig <mode> <hi> <lo>                       reported eigenvalue
  end
"""
import hashlib
import json
import struct
import sys
from decimal import Decimal
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[4]
REF = ROOT / "validation/references/f1_modal/mass_discrete"
sys.path.insert(0, str(REF / "generators"))

from model import DOFS, Model, form_mass, form_stiffness, reduce  # noqa: E402
from numeric import PointField, set_point_precision  # noqa: E402

EIGEN_CASES = ["E-AXIAL-WITNESS", "E-CANT-4", "E-CANT-INCLINED-4", "E-SS-8", "E-FF-4",
               "E-POINTMASS-NODE", "E-POINTMASS-GENERAL", "E-L-LINE-K3", "E-Z-BEND-K2.5",
               "E-RIGID-SCALE-1e6", "E-SPRINGS"]
POINT_PREC = 80
FLUSH = Decimal("1e-50")
OUT = HERE / "reformation_cases.txt"
SECTION_QUANTITIES = ["inner_diameter_metal", "inner_diameter_bore", "metal", "contents",
                      "insulation", "mbar", "jbar", "ibar", "A", "I", "J"]


def bits(value):
    value = float(value)
    return struct.pack(">d", value).hex()


def b64(value):
    """An input number: a binary64 value written exactly."""
    x = float(value)
    assert Fraction(x) == Fraction(str(value)), value
    return bits(x)


def opt(value):
    return "-" if value is None else b64(value)


def dd(value):
    fr = Fraction(Decimal(value)) if isinstance(value, str) else Fraction(value)
    hi = float(fr)
    lo = float(fr - Fraction(hi))
    return f"{bits(hi)} {bits(lo)}"


def model_lines(case, basis):
    spec = case["model"]
    model = Model(spec)
    a = spec.get("weight_to_mass_acceleration_m_per_s2")
    lines = [f"case {case['id']} {basis} {len(model.node_ids)} {opt(a)}"]
    for k, n in enumerate(spec["nodes"]):
        lines.append(f"node {k} " + " ".join(b64(v) for v in n["xyz"]))
    index = {}
    for m in spec["members"]:
        if m["kind"] not in ("straight", "arc"):
            continue  # connectors and tie rods: massless, no stiffness in these references
        k = len(index)
        index[m["id"]] = k
        sec = spec["sections"][m["section"]]
        mat = spec["materials"][m["material"]]
        section = " ".join([
            b64(sec["outside_diameter_m"]), b64(sec["wall_thickness_m"]),
            b64(sec["mill_tolerance_m"]), b64(sec["corrosion_allowance_m"]),
            b64(mat["density_kg_per_m3"]), opt(sec.get("contents_density_kg_per_m3")),
            opt(sec.get("insulation_thickness_m")), opt(sec.get("insulation_density_kg_per_m3"))])
        moduli = f"{b64(mat['elastic_modulus_Pa'])} {b64(mat['shear_modulus_Pa'])}"
        ends = f"{model.idx[m['i']]} {model.idx[m['j']]}"
        yref = " ".join(b64(v) for v in m["y_reference"])
        mass = 0 if m["id"] in model.body_of else 1
        if m["kind"] == "straight":
            scale = model.scale_of.get(m["id"])
            scale = "-" if scale is None else bits(scale)
            lines.append(f"straight {k} {ends} {yref} {section} {moduli} {scale} {mass}")
        else:
            assert m["id"] not in model.scale_of
            lines.append(f"arc {k} {ends} {b64(m['radius_m'])} {yref} {section} {moduli} "
                         f"{b64(m['flexibility_factor'])} {mass}")
    for c in spec.get("components", []):
        mem = c.get("maps_member")
        if mem in model.body_of and model.body_of[mem] is c:
            lines.append(f"body {model.idx[c['node']]} {b64(c['weight_N'])} {b64(a)} "
                         + " ".join(b64(v) for v in c["center_of_gravity_m"]))
    for sp in spec.get("springs", []):
        dof = 6 * model.idx[sp["node"]] + DOFS.index(sp["dof"])
        lines.append(f"spring {dof} {b64(sp['stiffness'])}")
    return model, index, lines


def decimal_of(entry):
    return entry["decimal"] if isinstance(entry, dict) else entry


def mass_case_lines(case, basis):
    model, index, lines = model_lines(case, basis)
    ref = case["bases"][basis]
    for m in case["model"]["members"]:
        if m["id"] not in index:
            continue
        q = ref["sections"][m["section"]]
        values = {
            "inner_diameter_metal": q["inner_diameter_metal_m"],
            "inner_diameter_bore": q["inner_diameter_bore_m"],
            "metal": q["mass_per_length_kg_per_m"]["metal"],
            "contents": q["mass_per_length_kg_per_m"]["contents"],
            "insulation": q["mass_per_length_kg_per_m"]["insulation"],
            "mbar": q["mass_per_length_kg_per_m"]["mbar"],
            "jbar": q["polar_rotary_inertia_per_length_kg_m"],
            "ibar": q["bending_rotary_inertia_per_length_kg_m"],
            "A": q["stiffness_section"]["A_m2"],
            "I": q["stiffness_section"]["I_m4"],
            "J": q["stiffness_section"]["J_m4"],
        }
        for name in SECTION_QUANTITIES:
            lines.append(f"section {index[m['id']]} {name} {dd(decimal_of(values[name]))}")
    for e in ref["elements"]:
        if e["kind"] == "straight" and "matrix_12x12_upper_nonzero" in e:
            for entry in e["matrix_12x12_upper_nonzero"]:
                lines.append(f"emass {index[e['id']]} {entry[0]} {entry[1]} {dd(entry[-1])}")
        elif e["kind"] == "arc":
            for side, lump in enumerate(e["lumps"]):
                for r, row in enumerate(lump["matrix_6x6"]):
                    for c, dec in enumerate(row):
                        if Decimal(dec) != 0:
                            lines.append(f"lump {index[e['id']]} {side} {r} {c} {dd(dec)}")
        elif e["kind"] == "component_body":
            node = model.idx[e["node"]]
            for r, row in enumerate(e["matrix_6x6_decimal"]):
                for c, dec in enumerate(row):
                    if Decimal(dec) != 0:
                        lines.append(f"cmass {node} {r} {c} {dd(dec)}")
    for entry in ref["assembled_full_mass_upper_nonzero"]:
        lines.append(f"mfull {entry[0]} {entry[1]} {dd(entry[-1])}")
    lines.append("end")
    return lines


def lower(A, tag):
    big = max(abs(x) for row in A for x in row)
    lines = []
    for i, row in enumerate(A):
        for j in range(i + 1):
            x = row[j]
            if abs(x) <= FLUSH * big:
                continue
            lines.append(f"{tag} {i} {j} {dd(Fraction(x))}")
    return lines


def eigen_case_lines(case, P):
    model, _index, lines = model_lines(case, "PF")
    free = model.free
    assert len(free) == case["n_free"]
    K = reduce(form_stiffness(model, P), free)
    M = reduce(form_mass(model, P, "PF")[0], free)
    lines.append("free " + " ".join(str(d) for d in free))
    lines += lower(K, "k") + lower(M, "m")
    for mode in case["modes"]:
        lines.append(f"eig {mode['mode']} {dd(mode['eigenvalue_rad2_per_s2'])}")
    lines.append("end")
    return lines


def build():
    lines = []
    source = (REF / "mass_references.json").read_bytes()
    lines.append(f"source mass_discrete/mass_references.json {hashlib.sha256(source).hexdigest()}")
    for case in json.loads(source)["cases"]:
        for basis in ("PF", "EX"):
            lines += mass_case_lines(case, basis)
    source = (REF / "eigen_references.json").read_bytes()
    lines.append(f"source mass_discrete/eigen_references.json {hashlib.sha256(source).hexdigest()}")
    refs = {c["id"]: c for c in json.loads(source)["cases"]}
    set_point_precision(POINT_PREC)
    P = PointField()
    for cid in EIGEN_CASES:
        lines += eigen_case_lines(refs[cid], P)
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
    print("reformation_cases.txt matches a fresh build")
    return 0


if __name__ == "__main__":
    sys.exit(main())
