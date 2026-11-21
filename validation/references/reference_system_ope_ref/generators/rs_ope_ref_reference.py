#!/usr/bin/env python3
"""Reference system, the reference-modulus operating case RS-OPE-REF: member states, EQ-1, EQ-2, EQ-3 (DEL-09-01).

Standard-library Python 3 only. This generator imports, reads and runs no product code and reads no fixture model
file and no product output. Its inputs are the reference-system specification's own statements
(`fixtures/reference_system/SPECIFICATION.md` §3.2, §4.1, §5.1, §8 and §8.1), transcribed below. It reads one other
file, a frozen independent reference, for a consistency binding (nothing in it is recomputed or altered here):

  ../reference_system_r1_ce/references.json   EQ-1 and EQ-2 of the fixture with RS-R1 and RS-CE1 (RS-SUS, RS-OPE,
                                               RS-SAM), whose bytes are checked against that package's SHA256SUMS

What it derives, every value an exact rational:

  member state     for each member and each of RS-OPE and RS-OPE-REF: the material point(s) the case's selection
                   consumes, the elastic modulus E, Poisson's ratio nu, the shear modulus G = E / (2 (1 + nu)) and
                   the free-length thermal strain; the analysis-basis reason each RS-OPE-REF member states
  modulus          RS-OPE: piecewise linear between the bracketing points at 210 degC (spec §4.1, §8.1);
                   RS-OPE-REF: the 20 degC point (spec §8.1)
  thermal strain   engineering secant law with constant coefficient alpha and datum T_d (spec §4.1): the free length
                   at T is L_d (1 + alpha (T - T_d)); the strain from installation T_i to operating T_o is
                   (1 + alpha (T_o - T_d)) / (1 + alpha (T_i - T_d)) - 1. With T_i = T_d = 20 degC the denominator is
                   exactly 1, so the strain is alpha (T_o - T_d) under this ratio form and under the plain difference
                   form alike. RS-P180 has no operating temperature and zero eigenstrain (spec §5.1, §8)
  EQ-1, EQ-2       RS-OPE-REF carries the same weight, generated over the same members with the same gravity, as
                   RS-SUS, RS-OPE and RS-SAM; a free-length thermal state applies no external load and every pressure
                   closure transfers to the wall, so its vertical reaction sum and reaction moment about the origin
                   are those of the other cases: the r1_ce package's values, bound here by identity of record
  EQ-3             RS-EXP = RS-OPE-REF - RS-SUS on every published signed component row (an exact identity); it
                   establishes the combination's algebra and nothing about either solve

Kelvin values are degC + 273.15 (an exact decimal offset).

Modes:
  python3 -I rs_ope_ref_reference.py           check: rebuild every output in memory and compare byte for byte
                                                with the committed files, SHA256SUMS included
  python3 -I rs_ope_ref_reference.py --write   write references.json and SHA256SUMS (intentional changes only)
"""
import argparse
import hashlib
import json
import re
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
REFS = OUT.parent
R1CE_DIR = REFS / "reference_system_r1_ce"
R1CE = R1CE_DIR / "references.json"
R1CE_REL = "validation/references/reference_system_r1_ce/references.json"
WRITTEN = ("references.json", "SHA256SUMS")

# ============================================================================ the specification's statements
MEMBERS = [  # §3.2, in order
    "RS-P010", "RS-P020", "RS-P030", "RS-P040", "RS-P050", "RS-P060", "RS-P065", "RS-P070", "RS-P080", "RS-P085",
    "RS-P090", "RS-P100", "RS-P110", "RS-P120", "RS-P130", "RS-P140", "RS-P150", "RS-P160", "RS-P170", "RS-P180",
    "RS-P190", "RS-P200", "RS-P210", "RS-P220", "RS-P225", "RS-P230",
]
REPLACED_SPAN = "RS-P180"  # §5.1: replaced by RS-EJ1; §8: no operating temperature on it
MATERIAL = "material:RS-M1"
POINTS = [  # §4.1: (id, degC, GPa)
    ("point:RS-M1-T020", F(20), F(200)), ("point:RS-M1-T100", F(100), F(195)),
    ("point:RS-M1-T200", F(200), F(189)), ("point:RS-M1-T300", F(300), F(182)),
]
NU = F(3, 10)  # §4.1, repeated on each point
ALPHA = F(12, 10 ** 6)  # §4.1: 12.0 x 10^-6 1/K, constant
LAW = "law:RS-M1-SECANT"  # §4.1: engineering_secant
DATUM_C = F(20)  # §4.1
INSTALLATION_C = F(20)  # §8
OPERATING_C = F(210)  # §8
KELVIN = F("273.15")
# §8.1: the reason every RS-OPE-REF member with an operating temperature states for its modulus.
REASON = "Modulus at 20 degC so that RS-EXP compares two states on one material basis (spec §8.1)"
# §8.1: how each case selects its modulus (the element state's material_selection).
CASE_SELECTION = {
    "load:RS-OPE": ("temperature_interpolation", OPERATING_C),  # every member, RS-P180 included (§10 coverage note)
    "load:RS-OPE-REF": ("exact_point", "point:RS-M1-T020"),
}
EQ_CASES = ("load:RS-SUS", "load:RS-OPE", "load:RS-SAM")  # the r1_ce package's cases

CROSS = []


def cross_check(label, agrees):
    CROSS.append({"check": label, "agrees": bool(agrees)})


def rec(q):
    """An exact rational's record: its text, 40 significant digits and the nearest binary64."""
    q = F(q)
    if q == 0:
        return {"exact": "0", "decimal": "0", "value": 0.0}
    getcontext().prec = 60
    return {"exact": str(q), "decimal": format(Decimal(q.numerator) / Decimal(q.denominator), ".39e"),
            "value": float(q)}


def kelvin(c):
    return c + KELVIN


def stretch(t):
    """The secant law's free-length ratio L(T) / L_d."""
    return 1 + ALPHA * (t - DATUM_C)


# ============================================================================ member states
def select(case):
    """(consumed points, interpolation fraction or None, E in Pa, selection temperature degC)."""
    kind, arg = CASE_SELECTION[case]
    if kind == "exact_point":
        pid, t, e = next(p for p in POINTS if p[0] == arg)
        return [(pid, t, e)], None, e * 10 ** 9, t
    k = next(i for i in range(len(POINTS) - 1) if POINTS[i][1] <= arg <= POINTS[i + 1][1])
    (pa, ta, ea), (pb, tb, eb) = POINTS[k], POINTS[k + 1]
    fraction = (arg - ta) / (tb - ta)
    return [(pa, ta, ea), (pb, tb, eb)], fraction, (ea + fraction * (eb - ea)) * 10 ** 9, arg


def member_state(case, member):
    hot = member != REPLACED_SPAN
    points, fraction, e, t_sel = select(case)
    g = e / (2 * (1 + NU))
    strain = stretch(OPERATING_C) / stretch(INSTALLATION_C) - 1 if hot else F(0)
    row = {
        "member": f"pipe:{member}",
        "operating_temperature_K": str(kelvin(OPERATING_C)) if hot else None,
        "material_selection_kind": CASE_SELECTION[case][0],
        "material_selection_temperature_K": str(kelvin(t_sel)),
        "consumed_material_points": [{"point_id": pid, "temperature_K": str(kelvin(t)), "E_Pa": rec(ep * 10 ** 9),
                                      "nu": rec(NU)} for pid, t, ep in points],
        "interpolation_fraction": rec(fraction) if fraction is not None else None,
        "E_Pa": rec(e),
        "nu": rec(NU),
        "G_Pa": rec(g),
        "thermal_state": "free_length_state" if hot else "unchanged_reference",
        "expansion_law": LAW if hot else None,
        "thermal_strain": rec(strain),
        "analysis_basis_override_reason": REASON if hot and case == "load:RS-OPE-REF" else None,
    }
    return row, e, g, strain


# ============================================================================ the r1_ce binding
TERM = re.compile(r"\((-?\d+(?:/\d+)?)\)(?:\*pi(?:\^(\d+))?)?(?:\*sqrt\((\d+)\))?")


def parse_exact(text):
    """{(k, n): q} of an `exact` text in the r1_ce package's format (sum of q * pi^k * sqrt(n))."""
    terms = {}
    if text == "0":
        return terms
    for part in text.split(" + "):
        m = TERM.fullmatch(part)
        assert m, part
        key = (0 if "*pi" not in part else int(m.group(2) or 1), int(m.group(3) or 1))
        terms[key] = terms.get(key, F(0)) + F(m.group(1))
    return {k: v for k, v in terms.items() if v != 0}


def add_terms(a, b):
    out = dict(a)
    for k, v in b.items():
        out[k] = out.get(k, F(0)) + v
    return {k: v for k, v in out.items() if v != 0}


def r1ce_binding():
    raw = R1CE.read_bytes()
    sums = dict(reversed(line.split("  ", 1)) for line in (R1CE_DIR / "SHA256SUMS").read_text().splitlines() if line)
    cross_check(f"{R1CE_REL}'s bytes match its own SHA256SUMS", sums.get("references.json") == hashlib.sha256(raw).hexdigest())
    refs = json.loads(raw)
    cross_check(f"{R1CE_REL} states EQ-1 and EQ-2 for exactly RS-SUS, RS-OPE and RS-SAM", tuple(refs["cases"]) == EQ_CASES)
    key = refs["key_values"]
    # Internal consistency of the bound record: W is the sum of its weighted members' and components' weights, and the
    # weight moment about the origin is (-S_y, S_x, 0) of its first moment.
    total = {}
    for row in refs["members"] + refs["components"]:
        if "weight_N" in row:
            total = add_terms(total, parse_exact(row["weight_N"]["exact"]))
    cross_check(f"{R1CE_REL}: W_total is the sum of its member and component weight records", total == parse_exact(key["W_total_N"]["exact"]))
    first = [parse_exact(v["exact"]) for v in key["weight_first_moment_N_m"]]
    moment = [parse_exact(v["exact"]) for v in key["weight_moment_about_origin_N_m"]]
    cross_check(f"{R1CE_REL}: the weight moment about the origin is (-S_y, S_x, 0)",
                moment[0] == {k: -v for k, v in first[1].items()} and moment[1] == first[0] and moment[2] == {})
    reaction = [parse_exact(v["exact"]) for v in key["reaction_moment_about_origin_N_m"]]
    cross_check(f"{R1CE_REL}: the reaction moment about the origin is minus the weight moment",
                all(reaction[i] == {k: -v for k, v in moment[i].items()} for i in range(3)))
    bound = {name: key[name] for name in ("W_total_N", "weight_resultant_N", "weight_first_moment_N_m",
                                          "weight_moment_about_origin_N_m", "reaction_moment_about_origin_N_m")}
    return bound, hashlib.sha256(raw).hexdigest()


# ============================================================================ the derivation
def build():
    CROSS.clear()
    cases = {}
    values = {}
    for case in CASE_SELECTION:
        rows = []
        for member in MEMBERS:
            row, e, g, strain = member_state(case, member)
            rows.append(row)
            values[(case, member)] = (e, g, strain)
        cases[case] = rows
    # Controls on the derivation.
    cross_check("RS-OPE's modulus at 210 degC is 188.3 GPa: 189 + (210 - 200)/(300 - 200) * (182 - 189)",
                values[("load:RS-OPE", "RS-P010")][0] == F(1883, 10) * 10 ** 9)
    cross_check("RS-OPE-REF's modulus is the 20 degC point's 200 GPa on every member",
                all(values[("load:RS-OPE-REF", m)][0] == 200 * 10 ** 9 for m in MEMBERS))
    cross_check("the thermal strain from 20 to 210 degC is 57/25000 = alpha * 190 (the ratio form's denominator is 1)",
                stretch(INSTALLATION_C) == 1 and values[("load:RS-OPE", "RS-P010")][2] == F(57, 25000)
                == ALPHA * (OPERATING_C - DATUM_C))
    cross_check("RS-OPE and RS-OPE-REF have the same thermal strain on every member",
                all(values[("load:RS-OPE", m)][2] == values[("load:RS-OPE-REF", m)][2] for m in MEMBERS))
    cross_check("RS-P180 has zero eigenstrain in both cases",
                values[("load:RS-OPE", REPLACED_SPAN)][2] == 0 == values[("load:RS-OPE-REF", REPLACED_SPAN)][2])
    cross_check("the two cases' moduli differ on every member: ratio 1883/2000",
                all(values[("load:RS-OPE", m)][0] / values[("load:RS-OPE-REF", m)][0] == F(1883, 2000) for m in MEMBERS))
    cross_check("G = E / 2.6 in both cases (nu = 0.3)",
                all(values[k][1] * F(26, 10) == values[k][0] for k in values))
    cross_check("25 members of each case have an operating temperature; RS-P180 has none",
                all(sum(r["operating_temperature_K"] is not None for r in rows) == 25 for rows in cases.values()))
    eq, r1ce_sha = r1ce_binding()
    distinguish = {
        "same_thermal_strain": rec(F(57, 25000)),
        "E_Pa": {"load:RS-OPE": rec(F(1883, 10) * 10 ** 9), "load:RS-OPE-REF": rec(200 * 10 ** 9)},
        "G_Pa": {"load:RS-OPE": rec(F(1883, 10) * 10 ** 9 / F(26, 10)), "load:RS-OPE-REF": rec(200 * 10 ** 9 / F(26, 10))},
        "E_ratio_RS-OPE_to_RS-OPE-REF": rec(F(1883, 2000)),
        "rule": "the two cases carry the same thermal strain and differ in modulus only; a reader of the published load "
                "reference state tells them apart by the consumed point(s), E and G, and by RS-OPE-REF's stated "
                "analysis-basis reason",
    }
    return {"cases": cases, "eq": eq, "r1ce_sha256": r1ce_sha, "distinguish": distinguish}


def document(result):
    return {
        "document": "swbpipe.reference_system_ope_ref_references",
        "status": "candidate frozen references; derived and cross-checked by the author TASK before the fixture's "
                  "RS-OPE-REF was solved against them, not yet independently refuted",
        "specification": "DEL-09-01 (the member states of RS-OPE and RS-OPE-REF; EQ-1 and EQ-2 for RS-OPE-REF; EQ-3 on "
                         "RS-OPE-REF - RS-SUS)",
        "inputs": "the specification's statements (§3.2, §4.1, §5.1, §8, §8.1), transcribed in the generator; no fixture "
                  "model file and no product output is read",
        "controls_read": [R1CE_REL],
        "units": {"modulus": "Pa", "temperature": "K", "strain": "1", "force": "N", "moment": "N*m"},
        "checks": {
            "member states": "the reference derives the state of every member of RS-OPE and RS-OPE-REF, 26 per case; "
                             "for each member the product publishes, 25 per case (it removes the replaced span "
                             "RS-P180's record before publication, though it still resolves the member), the consumed "
                             "material point(s), E, nu, G, the thermal strain and (RS-OPE-REF) the analysis-basis "
                             "reason equal `member_states`",
            "EQ-1": "in RS-OPE-REF, the sum of the global Z support action components over every support, RS-VS1 and "
                    "RS-CE1 included, equals W_total",
            "EQ-2": "in RS-OPE-REF, the horizontal (X, Y) resultant is zero and the support action moment resultant "
                    "about the origin equals reaction_moment_about_origin",
            "EQ-3": "on every published signed component row, RS-EXP = RS-OPE-REF - RS-SUS; magnitude rows equal the "
                    "norm of the combined components; rows the combination withholds are excluded and listed; the "
                    "identity establishes the combination's algebra and nothing about either solve",
        },
        "criterion": "EQ-1, EQ-2, EQ-3: the r1_ce package's (DEC-024/DEC-026), |observed - expected| <= 1e-9 * "
                     "|expected|, a zero row against 1e-9 * zero_scale; member states: the consumer states its a "
                     "priori rounding bound",
        "crosscheck": list(CROSS),
        "member_states": result["cases"],
        "distinguishing": result["distinguish"],
        "eq1_eq2": {
            "cases": ["load:RS-OPE-REF"],
            "bound_to": R1CE_REL,
            "bound_sha256": result["r1ce_sha256"],
            "grounds": [
                "RS-OPE-REF's W is the self-weight of the same members with the same gravity and the same component "
                "weights as RS-SUS, RS-OPE and RS-SAM (spec §8.1)",
                "a free-length thermal state is an eigenstrain: it applies no external load, so the reaction "
                "resultant is unchanged by temperature, whatever the modulus",
                "every pressure closure transfers to the wall, so each region is self-equilibrated under pressure "
                "(spec §9.2, EQ-2)",
                "RS-CE1's 9000 N is the same support action in every case and adds no weight",
            ],
            "key_values": result["eq"],
        },
        "eq3": {
            "combination": "combination:RS-EXP",
            "label": "RS-EXP expansion: RS-OPE-REF - RS-SUS, the difference of two solved states on the 20 degC modulus",
            "minuend": "load:RS-OPE-REF",
            "subtrahend": "load:RS-SUS",
            "identity": "on every published signed component row (support force and moment components, translation and "
                        "rotation components, element end-force components), RS-EXP = RS-OPE-REF - RS-SUS; magnitude "
                        "rows are recomputed from the combined components; rows the combination withholds, such as "
                        "tie-rod forces, are excluded",
            "both_operands_on_one_material_basis": "every member of RS-SUS and RS-OPE-REF selects the 20 degC point",
            "what_it_establishes": "the combination's algebra: RS-EXP is the row-by-row difference of the two published "
                                   "solved states; it establishes nothing about either solve, and RS-EXP is the "
                                   "difference of two independently solved equilibria, not a solution of the system "
                                   "under the load difference",
        },
    }


def dumps(obj):
    def inline(o):
        return json.dumps(o, ensure_ascii=False, separators=(", ", ": "))

    def emit(o, level):
        pad = " " * level
        if isinstance(o, dict) and ("exact" in o or "check" in o):
            return inline(o)
        if isinstance(o, dict):
            items = [f'{pad} {json.dumps(k)}: {emit(v, level + 1)}' for k, v in o.items()]
            return "{\n" + ",\n".join(items) + "\n" + pad + "}" if items else "{}"
        if isinstance(o, list) and o and all(isinstance(x, dict) for x in o):
            items = [f"{pad} {emit(v, level + 1)}" for v in o]
            return "[\n" + ",\n".join(items) + "\n" + pad + "]"
        return inline(o)
    return emit(obj, 0) + "\n"


def package_files():
    """Every committed file of the package but the two this generator writes."""
    return {str(p.relative_to(OUT)): p.read_bytes() for p in sorted(OUT.rglob("*"))
            if p.is_file() and "__pycache__" not in p.parts and str(p.relative_to(OUT)) not in WRITTEN}


def build_files():
    result = build()
    text = dumps(document(result))
    assert json.loads(text) == json.loads(json.dumps(document(result)))
    files = {"references.json": text.encode(), **package_files()}
    lines = [f"{hashlib.sha256(data).hexdigest()}  {path}" for path, data in sorted(files.items())]
    files["SHA256SUMS"] = ("\n".join(lines) + "\n").encode()
    return files, result


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files, result = build_files()
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("  DISAGREES:", c["check"])
    if args.write:
        if bad:
            print("not written: a cross-check disagrees")
            return 1
        for path in WRITTEN:
            (OUT / path).write_bytes(files[path])
        print(f"wrote {', '.join(WRITTEN)}")
        return 0
    failures = [p for p, d in files.items() if not (OUT / p).is_file() or (OUT / p).read_bytes() != d]
    if failures or bad:
        print("MISMATCH: " + ", ".join(failures))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
