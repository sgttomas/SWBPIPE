"""O4 declared-problem oracle for the retained-source route (standard library only).

DEL-04-08 Design `o4-source-extension.md` section 5, "A declared-problem oracle
for the source route": RF's P1 method made a maintained oracle. It reads no
product code. Its inputs are product outputs only:

- a dump written by the product's test-only entry `o4_source_payload_dump`
  (`{"artifact": "o4-source-declared-problem-dump", ..., "records": [...]}`,
  each record carrying the checked canonical text of the private normalized
  source payload and functional plan, and their SHA-256);
- the published envelope of the same request and solver mode (the headless
  runner's `solve` output, or a bare mechanics envelope);
- the request itself (for the authored section and material, used only by the
  supplementary screens of BOUND.md section 4).

Per selected source case it asserts:
  (a) sha256(text) equals the record's and the receipt's normalized_source_sha256
      and functional_plan_sha256, and the dump's publication_sha256 equals the
      receipt's, so it judges exactly the committed source;
  (b) the declared problem (K = sum of exact congruences T^T K_local T of the
      declared binary64 K_local and T, plus springs; declared loads, eigenloads
      and prescribed motions) solved exactly in Fractions; every published
      projection interval, every station/endpoint action interval and every
      support-norm interval contains the exact value of its quantity, computed
      from the exact solution by statics in this file (not from the plan's
      coefficients); the plan's own lowering, evaluated exactly, agrees;
  (c) every public section stress check recomputes from its action and its own
      basis's parameters, which are the payload's leaves of that basis; each
      basis's endpoint maximum interval contains the exact maximum;
  (d) every declared K_local entry lies within BOUND.md's bound (bound.json) of
      the exact matrix re-formed from the formation inputs;
  (e) per case PASS/FAIL with the worst margins.

Usage:
  python3 -I oracle.py --selftest
  python3 -I oracle.py --dump D.json --envelope E.json --request R.json [--theory eb|timo] [--allowance zero|mill|ca|both]
  python3 -I oracle.py --index runs_index.json [--out report.json]
"""

import argparse
import copy
import hashlib
import json
import math
import os
import struct
import sys
from fractions import Fraction as F

HERE = os.path.dirname(os.path.abspath(__file__))
EPS = F(1, 2 ** 53)
PI_LO = F("3.14159265358979323846264338327950288419")
PI_HI = PI_LO + F(1, 10 ** 38)
TO_M = {"m": F(1), "mm": F(1, 1000), "cm": F(1, 100)}
TO_PA = {"Pa": F(1), "kPa": F(1000), "MPa": F(10 ** 6), "GPa": F(10 ** 9)}
COMPONENTS6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
SUPPORT_COMPONENTS = {"Fx": "UX", "Fy": "UY", "Fz": "UZ", "Mx": "RX", "My": "RY", "Mz": "RZ"}
ACTION_INDEX = {"axial_force": 0, "shear_force_y": 1, "shear_force_z": 2,
                "torsional_moment": 3, "bending_moment_y": 4, "bending_moment_z": 5}
STATION_FRACTION = {"end_i": F(0), "quarter_1": F(1, 4), "midspan": F(1, 2), "quarter_3": F(3, 4), "end_j": F(1)}
UNIT_FACTOR = {"m": F(1), "mm": F(1000), "rad": F(1), "N": F(1), "N*m": F(1)}  # value in unit = SI value * factor
DECLARED_SI = {"U": ("m", "N"), "R": ("rad", "N*m")}  # the declared problem's DOF units (dof_map), required SI


def public_unit(quantity, component):
    """Round 3: the public unit of a functional's quantity, from the public vocabulary, never from the evidence."""
    if quantity == "nodal_translation":
        return "mm"
    if quantity == "nodal_rotation":
        return "rad"
    if quantity in ("member_end_action", "member_station_action"):
        return "N" if ACTION_INDEX.get(component, 9) < 3 else "N*m"
    if quantity == "support_action_component":
        return "N" if component.startswith("F") else "N*m"
    return None


def dups(seq):
    """Duplicate keys of a raw list (checked before any dict is built from it)."""
    seen, out = set(), set()
    for x in seq:
        key = json.dumps(x, sort_keys=True) if isinstance(x, (dict, list)) else x
        (out if key in seen else seen).add(key)
    return sorted(out, key=str)


# ----------------------------------------------------------------------------- numbers

def b2f(bits):
    return struct.unpack(">d", bytes.fromhex(bits))[0]


def b2q(bits):
    x = b2f(bits)
    if not math.isfinite(x):
        raise ValueError(f"non-finite operand {bits}")
    return F(x)


def f2b(x):
    return struct.pack(">d", x).hex()


def qf(x):
    if isinstance(x, bool) or not isinstance(x, (int, float)):
        raise ValueError(f"not a number: {x!r}")
    return F(x)


def rn(x):
    return float(x)  # Fraction -> float is correctly rounded (int / int true division)


def rd(x):
    f = rn(x)
    return f if F(f) <= x else math.nextafter(f, -math.inf)


def ru(x):
    f = rn(x)
    return f if F(f) >= x else math.nextafter(f, math.inf)


def sqrt_up(x):
    """A rational upper bound of sqrt(x), x >= 0, within 2^-260 relative."""
    if x == 0:
        return F(0)
    a, b = x.numerator, x.denominator
    m = 2 ** 300
    return F(math.isqrt(a * b * m * m) + 1, b * m)


def sqrt_down(x):
    if x == 0:
        return F(0)
    a, b = x.numerator, x.denominator
    m = 2 ** 300
    return F(math.isqrt(a * b * m * m), b * m)


def sqrt_in(s2, lo, hi):
    """lo <= sqrt(s2) <= hi exactly, sign-safe (round 2 (e)): a negative hi can never hold a square root."""
    return (lo <= 0 or lo * lo <= s2) and hi >= 0 and s2 <= hi * hi


def max_in(terms, Z, lo, hi):
    """lo <= max_i(a_i + sqrt(s_i)/Z) <= hi exactly, for terms (a_i, s_i) and Z > 0."""
    def at_least(a_, s_, bound_):  # a + sqrt(s)/Z >= bound
        d = (bound_ - a_) * Z
        return d <= 0 or d * d <= s_

    def at_most(a_, s_, bound_):  # a + sqrt(s)/Z <= bound
        d = (bound_ - a_) * Z
        return d >= 0 and s_ <= d * d
    return bool(terms) and any(at_least(a_, s_, lo) for a_, s_ in terms) and all(at_most(a_, s_, hi) for a_, s_ in terms)


def rel_in(x, v, rel):
    """|v - x| <= rel*|x| for exact x (the claimed relative accuracy against the exact value)."""
    return v == x if x == 0 else abs(v - x) <= rel * abs(x)


def rho(k):
    """Upper bound of (1 - eps)^(-k) - 1 for k a nonnegative multiple of 1/2, exact for integers."""
    k = F(k)
    if (2 * k).denominator != 1 or k < 0:
        raise ValueError(f"count {k} is not a nonnegative multiple of 1/2")
    n = int(k)  # floor
    base = (1 / (1 - EPS)) ** n
    if k != n:
        base *= sqrt_up(1 / (1 - EPS))
    return base - 1


def fmt(x):
    if x is None:
        return None
    if isinstance(x, F):
        return float(x) if x == 0 or abs(x) > F(1, 10 ** 300) else ("tiny" if x > 0 else "-tiny")
    return x


# ----------------------------------------------------------------------------- bound.json

def load_bound(path=None):
    with open(path or os.path.join(HERE, "bound.json"), encoding="utf-8") as fh:
        bound = json.load(fh)
    if bound.get("schema") != "o4_declared_problem_bound_v1" or bound["unit_roundoff_log2"] != -53:
        raise SystemExit("oracle: bound.json is not the frozen v1 bound")
    phi = F(bound["phi_count"]["constant"]), F(bound["phi_count"]["per_ell"])
    # Re-derive every count from BOUND.md's formulas; refuse on any disagreement.
    derived = {
        "timoshenko.k_vv": (6 + phi[0] + 1, 3 + phi[1]),
        "timoshenko.k_vtheta": (5 + phi[0] + 1, 2 + phi[1]),
        "timoshenko.k_thth": (4 + 2 * (phi[0] + 1), 1 + 2 * phi[1]),
        "timoshenko.k_ththprime": ((phi[0] + 1) + 4 + (phi[0] + 1), phi[1] + 1 + phi[1]),
        "timoshenko.k_axial": (2, 1), "timoshenko.k_torsion": (2, 1),
        "euler_bernoulli.k_vv": (5, 3), "euler_bernoulli.k_vtheta": (4, 2),
        "euler_bernoulli.k_thth": (3, 1), "euler_bernoulli.k_ththprime": (3, 1),
        "euler_bernoulli.k_axial": (2, 1), "euler_bernoulli.k_torsion": (2, 1),
    }
    for key, (c0, c1) in derived.items():
        theory, role = key.split(".")
        entry = bound["roles"][theory][role]
        if (F(entry["constant"]), F(entry["per_ell"])) != (F(c0), F(c1)):
            raise SystemExit(f"oracle: bound.json count for {key} disagrees with BOUND.md's derivation")
    return bound


# ----------------------------------------------------------------------------- linear algebra

def solve_exact(K, f, free, prescribed, n):
    """Exact solution u of K u = f on the free DOFs with u fixed on the prescribed DOFs."""
    u = [F(0)] * n
    for d, v in prescribed.items():
        u[d] = v
    idx = list(free)
    m = len(idx)
    A = [[K[idx[r]][idx[c]] for c in range(m)] + [f[idx[r]] - sum((K[idx[r]][d] * v for d, v in prescribed.items()), F(0))]
         for r in range(m)]
    for col in range(m):
        piv = next((r for r in range(col, m) if A[r][col] != 0), None)
        if piv is None:
            raise ValueError("declared free-DOF matrix is singular")
        A[col], A[piv] = A[piv], A[col]
        p = A[col][col]
        for r in range(m):
            if r != col and A[r][col] != 0:
                factor = A[r][col] / p
                A[r] = [a - factor * b for a, b in zip(A[r], A[col])]
    for r in range(m):
        u[idx[r]] = A[r][m] / A[r][r]
    return u


def matmul(A, B):
    return [[sum((A[i][k] * B[k][j] for k in range(len(B)) if A[i][k] != 0 and B[k][j] != 0), F(0))
             for j in range(len(B[0]))] for i in range(len(A))]


def transpose(A):
    return [list(r) for r in zip(*A)]


def is_signed_permutation(T):
    for row in T:
        nz = [x for x in row if x != 0]
        if len(nz) != 1 or abs(nz[0]) != 1:
            return False
    return all(sum(1 for r in range(len(T)) if T[r][c] != 0) == 1 for c in range(len(T)))


# ----------------------------------------------------------------------------- the exact reference K_local

def exact_k_local(E, G, A, Iy, Iz, J, L, As):
    """Exact K* and per-entry (role, scale) maps per BOUND.md 1.1 (As None: Euler-Bernoulli)."""
    def coeffs(I):
        phi = F(0) if As is None else 12 * E * I / (G * As * L * L)
        c = {
            "k_vv": 12 * E * I / (L ** 3 * (1 + phi)),
            "k_vtheta": 6 * E * I / (L ** 2 * (1 + phi)),
            "k_thth": (4 + phi) * E * I / (L * (1 + phi)),
            "k_ththprime": (2 - phi) * E * I / (L * (1 + phi)),
        }
        scale_tp = (phi + abs(2 - phi)) * E * I / (L * (1 + phi)) if As is not None else abs(c["k_ththprime"])
        return c, phi, scale_tp
    planes = {"z": coeffs(Iz), "y": coeffs(Iy)}
    common = {"k_axial": E * A / L, "k_torsion": G * J / L}
    return planes, common


def reference_matrix(bound, inputs, theory):
    E, G, A, Iy, Iz, J, L, As = (inputs[k] for k in ("E", "G", "A", "Iy", "Iz", "J", "L", "As"))
    planes, common = exact_k_local(E, G, A, Iy, Iz, J, L, As)
    K = [[F(0)] * 12 for _ in range(12)]
    role = [[None] * 12 for _ in range(12)]
    scale = [[F(0)] * 12 for _ in range(12)]
    phis = {p: planes[p][1] for p in planes}
    for r, c, name, plane, sign in bound["layout"]["upper_triangle"]:
        if name in common:
            v = sign * common[name]
            s = abs(v)
        else:
            coeff, _, scale_tp = planes[plane]
            v = sign * coeff[name]
            s = scale_tp if name == "k_ththprime" else abs(v)
        for (i, j) in ((r, c), (c, r)):
            K[i][j] = v
            role[i][j] = name
            scale[i][j] = s
    return K, role, scale, phis


def role_usage(bound, declared, inputs, theory, ell):
    """Largest |K^ - K*| / (rho_N * S) per role (BOUND.md section 3)."""
    K, role, scale, _ = reference_matrix(bound, inputs, theory)
    roles = bound["roles"]["timoshenko" if theory == "timo" else "euler_bernoulli"]
    out = {}
    for i in range(12):
        for j in range(12):
            if role[i][j] is None:
                continue
            entry = roles[role[i][j]]
            allow = rho(F(entry["constant"]) + F(entry["per_ell"]) * ell) * scale[i][j]
            if allow > 0:
                u = abs(declared[i][j] - K[i][j]) / allow
                out[role[i][j]] = max(out.get(role[i][j], F(0)), u)
    return out


def judge_k_local(bound, declared, inputs, theory, ell):
    """BOUND.md: |K^ - K*| <= rho_N * S per entry. Returns (ok, worst usage, findings)."""
    K, role, scale, phis = reference_matrix(bound, inputs, theory)
    roles = bound["roles"]["timoshenko" if theory == "timo" else "euler_bernoulli"]
    findings = []
    worst = (F(-1), None)
    for i in range(12):
        for j in range(12):
            d = declared[i][j]
            if role[i][j] is None:
                if d != 0:
                    findings.append(f"structural zero ({i},{j}) declared {fmt(d)}")
                continue
            entry = roles[role[i][j]]
            N = F(entry["constant"]) + F(entry["per_ell"]) * ell
            allow = rho(N) * scale[i][j]
            err = abs(d - K[i][j])
            usage = err / allow if allow > 0 else (F(0) if err == 0 else F(10 ** 30))
            if usage > worst[0]:
                worst = (usage, (i, j, role[i][j], int(N) if N.denominator == 1 else float(N)))
            if err > allow:
                findings.append(f"entry ({i},{j}) {role[i][j]}: |declared - exact| = {float(err):.6e} > rho_{float(N)}*S = {float(allow):.6e} (usage {float(usage):.4g}); exact {float(K[i][j]):.17g}, declared {float(d):.17g}")
    return not findings, worst, findings, phis


# ----------------------------------------------------------------------------- payload helpers

def pointer(doc, path):
    cur = doc
    for part in path.split("/")[1:]:
        part = part.replace("~1", "/").replace("~0", "~")
        cur = cur[int(part)] if isinstance(cur, list) else cur[part]
    return cur


def frame_inputs(frame):
    fi = frame["formation_inputs"]
    out = {"E": b2q(fi["young_modulus_bits"]), "G": b2q(fi["shear_modulus_bits"]), "A": b2q(fi["area_bits"]),
           "Iy": b2q(fi["iy_bits"]), "Iz": b2q(fi["iz_bits"]), "J": b2q(fi["torsion_constant_bits"]),
           "As": b2q(fi["shear_area_bits"]) if "shear_area_bits" in fi else None}
    if "length_bits" in fi:
        out["L"] = b2q(fi["length_bits"])
        out["ell"] = F(0)
        out["L_source"] = "declared_length_leaf"
    else:
        out["L"] = None
        out["ell"] = F(7, 2)
        out["L_source"] = "coordinates"
    return out


def basis_leaves(frame, basis):
    fi = frame["formation_inputs"]
    nominal = {"area": fi["area_bits"], "section_modulus": frame["recovery_section"]["section_modulus_bits"],
               "torsion_constant": fi["torsion_constant_bits"], "torsion_radius": frame["recovery_section"]["torsion_radius_bits"]}
    if basis == "nominal_section":
        return nominal
    red = frame.get("reduced_recovery_section")
    if red is None:
        return nominal
    return {"area": red["area_bits"], "section_modulus": red["section_modulus_bits"],
            "torsion_constant": red["torsion_constant_bits"], "torsion_radius": red["torsion_radius_bits"]}


def annulus(D, wall):
    ro = D / 2
    ri = ro - wall
    a2 = ro * ro - ri * ri
    i4 = (ro ** 4 - ri ** 4) / 4
    return {"A": (PI_LO * a2, PI_HI * a2), "I": (PI_LO * i4, PI_HI * i4), "J": (2 * PI_LO * i4, 2 * PI_HI * i4),
            "Z": (PI_LO * i4 / ro, PI_HI * i4 / ro), "ro": ro}


def within_screen(x, enclosure, rel):
    lo, hi = enclosure
    return lo * (1 - rel) <= x <= hi * (1 + rel)


def kappa_hat(D, t, nu):
    """U8 section 3.1's fixed binary64 order (no FMA): Python float arithmetic is IEEE binary64."""
    ro = D / 2.0
    ri = ro - t
    m = ri / ro
    m2 = m * m
    a = 1.0 + m2
    b = 1.0 - m2
    n1 = 1.0 + nu
    c = (8.0 * nu + 14.0) * nu + 7.0
    Dn = c * (b * b) + 48.0 * (m2 * (n1 * n1))
    return (6.0 * (n1 * n1)) * (a * a) / Dn


def request_member(request, element_id):
    model = request["model"]
    if dups([p["id"] for p in model.get("pipe_segments", [])]) or dups([m["id"] for m in model.get("materials", [])]):
        return None  # ambiguous request: the supplementary screens are not judged
    pipe = next((p for p in model["pipe_segments"] if p["id"] == element_id), None)
    if pipe is None:
        return None
    sec = pipe["section"]

    def length(key):
        if key not in sec:
            return 0.0
        v = sec[key]
        q = qf(v["value"]) * TO_M[v["unit"]]
        f = float(q)
        return f if F(f) == q else None
    mat = next((m for m in model["materials"] if m["id"] == pipe["material"]), None)
    nu = mat["poisson_ratio"]["value"] if mat else None
    return {"D": length("outside_diameter"), "t": length("wall_thickness"), "cm": length("mill_tolerance"),
            "ca": length("corrosion_allowance"), "nu": nu}


# ----------------------------------------------------------------------------- the judge

class Case:
    def __init__(self, case_id):
        self.case_id = case_id
        self.checks = {}   # name -> [passed count, failed count]
        self.findings = []
        self.metrics = {}
        self.notes = []
        self.not_judged_counts = {}

    def check(self, name, ok, message=None):
        slot = self.checks.setdefault(name, [0, 0])
        slot[0 if ok else 1] += 1
        if not ok and message:
            self.findings.append(f"[{name}] {message}")
        return ok

    def not_judged(self, name, reason):
        """A supplementary screen whose stated condition is unmet: recorded, never a pass or a failure."""
        self.not_judged_counts[name] = self.not_judged_counts.get(name, 0) + 1
        if self.not_judged_counts[name] == 1:
            self.notes.append(f"[{name}] not judged: {reason}")

    def metric_max(self, name, value, where):
        if value is None:
            return
        cur = self.metrics.get(name)
        if cur is None or value > cur[0]:
            self.metrics[name] = (value, where)

    def metric_min(self, name, value, where):
        if value is None:
            return
        cur = self.metrics.get(name)
        if cur is None or value < cur[0]:
            self.metrics[name] = (value, where)

    @property
    def passed(self):
        return all(f == 0 for _, f in self.checks.values())

    def report(self):
        return {"case_id": self.case_id, "verdict": "PASS" if self.passed else "FAIL",
                "checks": {k: {"passed": v[0], "failed": v[1]} for k, v in sorted(self.checks.items())},
                "metrics": {k: {"value": fmt(v[0]), "where": v[1]} for k, v in sorted(self.metrics.items())},
                "findings": self.findings[:200], "finding_count": len(self.findings), "notes": self.notes,
                "not_judged": dict(sorted(self.not_judged_counts.items()))}


def envelope_of(doc):
    if "payload" in doc and isinstance(doc["payload"], dict) and "mechanics_envelope" in doc["payload"]:
        return doc["payload"]["mechanics_envelope"]
    return doc


def judge(dump, envelope_doc, request, bound, theory=None, allowance=None):
    envelope = envelope_of(envelope_doc)
    raw_ids = [r.get("id") for r in envelope.get("results", [])]
    dup_ids = sorted({i for i in raw_ids if raw_ids.count(i) > 1}, key=str)  # round 2 (c): before any dict
    results = {r["id"]: r for r in envelope.get("results", [])}
    sbr = envelope.get("source_block_recovery")
    out = {"publication_link": None, "cases": [], "unselected": [], "errors": []}
    if dump.get("artifact") != "o4-source-declared-problem-dump":
        out["errors"].append("not an o4-source-declared-problem-dump")
        return out
    if sbr is None:
        out["errors"].append("envelope carries no source_block_recovery receipt")
        return out
    body = sbr["body"]
    out["publication_link"] = dump.get("publication_sha256") == body.get("publication_sha256")
    # round 3: duplicate case identities are rejected on the raw lists, before any dict collapses them
    case_dups = {"receipt cases": dups([c["basis_ref"]["ref_id"] for c in body["cases"]]),
                 "contract-evidence cases": dups([c.get("load_case_id") for c in envelope.get("contract_evidence", {}).get("exact_cases", [])]),
                 "dump records": dups([r["case_id"] for r in dump["records"]])}
    for what, d_ in case_dups.items():
        if d_:
            out["errors"].append(f"duplicate {what}: {d_}")
    receipt_cases = {c["basis_ref"]["ref_id"]: c for c in body["cases"]}
    evidence = {c["load_case_id"]: c for c in envelope.get("contract_evidence", {}).get("exact_cases", [])}
    recorded = {r["case_id"] for r in dump["records"]}
    for cid, c in receipt_cases.items():
        if c["selected_method"] != "retained_source_blocks_exact_v1":
            out["unselected"].append({"case_id": cid, "selected_method": c["selected_method"],
                                      "ordinary_attempt": c.get("ordinary_attempt")})
        elif cid not in recorded:
            out["errors"].append(f"selected case {cid} has no dump record")
    for record in dump["records"]:
        case = Case(record["case_id"])
        case.check("i_unique_cases", not any(case_dups.values()), f"duplicate case identities: { {k: v for k, v in case_dups.items() if v} }")
        try:
            judge_record(case, record, receipt_cases.get(record["case_id"]), evidence.get(record["case_id"]),
                         results, request, bound, theory, allowance, out["publication_link"], dup_ids)
        except Exception as exc:  # a malformed dump is a failure, never a pass
            case.check("oracle_completed", False, f"{type(exc).__name__}: {exc}")
        out["cases"].append(case.report())
    return out


def solve_declared(ns, fp, case=None, src=None):
    """The declared problem solved exactly; every functional's exact value by statics and by the plan."""
    case = case or Case("declared")
    src = src if src is not None else {"free_dofs": list(ns["free_dofs"]), "prescribed_dofs": sorted(p["dof"] for p in ns["prescribed"])}
    # (b) the declared problem ----------------------------------------------------------
    n = len(ns["dof_map"])
    # round 3: every keyed list of the declared problem is unique before a dict is built from it
    for name, keys in (("dof_map", [(d["node_id"], d["component"]) for d in ns["dof_map"]]),
                       ("frames", [f["element_id"] for f in ns["frames"]]),
                       ("springs", [s["spring_id"] for s in ns["springs"]]),
                       ("spring_supports", [s["support_id"] for s in ns["springs"]]),
                       ("spring_records", list(ns["springs"])),  # a byte-identical spring record twice
                       ("prescribed", [p["dof"] for p in ns["prescribed"]]),
                       ("free_dofs", list(ns["free_dofs"])),
                       ("functionals", [f["functional_id"] for f in fp["functions"]]),
                       ("eigen_loads", [e["pipe_id"] for e in (ns.get("load_reference_state") or {}).get("eigen_loads", [])]),
                       ("load_state_members", [m["pipe_id"] for m in (ns.get("load_reference_state") or {}).get("members", [])])):
        d_ = dups(keys)
        case.check(f"i_unique_declared_{name}", not d_, f"duplicate declared {name}: {d_[:5]}")
    case.check("b_declared_units_si", all((d["displacement_unit"], d["action_unit"]) == DECLARED_SI[d["component"][0]] for d in ns["dof_map"]),
               "the declared DOF units are not SI (m or rad, N or N*m)")
    dof_of = {(d["node_id"], d["component"]): i for i, d in enumerate(ns["dof_map"])}
    K = [[F(0)] * n for _ in range(n)]
    frames = {}
    for frame in ns["frames"]:
        Kl = [[b2q(x) for x in row] for row in frame["local_matrix_bits"]]
        T = [[b2q(x) for x in row] for row in frame["transform_bits"]]
        scatter = frame["scatter"]
        G = matmul(transpose(T), matmul(Kl, T))
        frames[frame["element_id"]] = {"frame": frame, "Kl": Kl, "T": T, "G": G, "scatter": scatter,
                                       "perm": is_signed_permutation(T)}
        for i in range(12):
            for j in range(12):
                if G[i][j] != 0:
                    K[scatter[i]][scatter[j]] += G[i][j]
        # declared contributions bind the same matrix (signed permutation: exactly +-K_local)
        if frames[frame["element_id"]]["perm"]:
            declared = {}
            for t in ns["stiffness_terms"]:
                if t["owner"]["kind"] == "frame" and t["owner"]["id"] == frame["element_id"]:
                    declared[(t["row"], t["col"])] = declared.get((t["row"], t["col"]), F(0)) + b2q(t["value_bits"])
            expect = {(scatter[i], scatter[j]): G[i][j] for i in range(12) for j in range(12) if G[i][j] != 0}
            mism = [k for k in set(declared) | set(expect) if declared.get(k, F(0)) != expect.get(k, F(0))]
            case.check("b_stiffness_terms_bind_k_local", not mism, f"{frame['element_id']}: {len(mism)} declared stiffness terms differ from T^T K_local T")
        else:
            case.notes.append(f"{frame['element_id']}: congruence member (not a signed permutation); terms judged through the exact congruence only")
    springs = []
    for s in ns["springs"]:
        k, g = b2q(s["stiffness_bits"]), b2q(s["ground_movement_bits"])
        K[s["dof"]][s["dof"]] += k
        springs.append((s["dof"], k, g, s))
    spring_terms = {}
    for t in ns["stiffness_terms"]:
        if t["owner"]["kind"] == "spring":
            spring_terms[(t["row"], t["col"])] = spring_terms.get((t["row"], t["col"]), F(0)) + b2q(t["value_bits"])
    # round 4: distinct springs may act in parallel on one DOF; their stiffnesses sum exactly
    want_terms = {}
    for d, k, _, _ in springs:
        want_terms[(d, d)] = want_terms.get((d, d), F(0)) + k
    case.check("b_spring_terms", {key: v for key, v in spring_terms.items() if v != 0} == {key: v for key, v in want_terms.items() if v != 0},
               "declared spring stiffness terms differ from the springs' per-DOF sums")
    f = [F(0)] * n
    eig = {}
    for t in ns["force_terms"]:
        v = b2q(t["value_bits"])
        f[t["dof"]] += v
        if t["owner"]["kind"] == "member_eigenstrain":
            eig.setdefault(t["owner"]["id"], {})[t["dof"]] = eig.setdefault(t["owner"]["id"], {}).get(t["dof"], F(0)) + v
    for d, k, g, _ in springs:
        f[d] += k * g
    prescribed = {}
    for p in ns["prescribed"]:
        prescribed[p["dof"]] = b2q(p["value_bits"])
    free = list(ns["free_dofs"])
    case.check("b_partition", sorted(free + list(prescribed)) == list(range(n)) and not set(free) & set(prescribed),
               "free and prescribed DOFs do not partition the DOFs")
    case.check("b_partition_matches_receipt", free == list(src.get("free_dofs", [])) and sorted(prescribed) == sorted(src.get("prescribed_dofs", [])),
               "payload partition differs from the receipt's")
    u = solve_exact(K, f, free, prescribed, n)
    resid = [sum((K[i][j] * u[j] for j in range(n) if K[i][j] != 0), F(0)) - f[i] for i in free]
    case.check("b_exact_solve_residual", all(r == 0 for r in resid), "exact solve leaves a nonzero free-row residual")

    # exact physical quantities -----------------------------------------------------
    def local_end_actions(eid):
        fr = frames[eid]
        ue = [u[d] for d in fr["scatter"]]
        Tu = [sum((fr["T"][i][j] * ue[j] for j in range(12) if fr["T"][i][j] != 0), F(0)) for i in range(12)]
        act = [sum((fr["Kl"][i][j] * Tu[j] for j in range(12) if fr["Kl"][i][j] != 0), F(0)) for i in range(12)]
        feq_g = [eig.get(eid, {}).get(d, F(0)) for d in fr["scatter"]]
        feq_l = [sum((fr["T"][i][j] * feq_g[j] for j in range(12) if fr["T"][i][j] != 0), F(0)) for i in range(12)]
        return [a - q for a, q in zip(act, feq_l)]
    end_actions = {eid: local_end_actions(eid) for eid in frames}
    support_dofs = {}
    for p in ns["prescribed"]:
        support_dofs.setdefault(p["support_id"], set()).add(p["dof"])
    spring_by_id = {s["support_id"]: (d, k, g) for d, k, g, s in springs}
    spring_by_spring_id = {s["spring_id"]: (d, k, g) for d, k, g, s in springs}

    def physics_value(fn):
        q, comp, owner = fn["quantity"], fn["component"], fn["owner"]
        # round 3: the exact value is formed in the quantity's public unit, converted from the declared SI by a
        # factor derived from the two units; the functional's own unit is only compared, never used
        pu = public_unit(q, comp)
        if pu is None:
            raise ValueError(f"no public unit for {q}/{comp}")
        uf = UNIT_FACTOR[pu]
        if q in ("nodal_translation", "nodal_rotation"):
            return u[dof_of[(owner["id"], comp)]] * uf
        if q == "member_end_action":
            a = end_actions[owner["id"]]
            return a[ACTION_INDEX[comp] + (6 if fn["location"] == "end_j" else 0)] * uf
        if q == "member_station_action":
            a = end_actions[owner["id"]][:6]
            L = frame_inputs(frames[owner["id"]]["frame"])["L"]
            x = STATION_FRACTION[fn["location"]] * L
            sec = {"axial_force": -a[0], "shear_force_y": -a[1], "shear_force_z": -a[2], "torsional_moment": -a[3],
                   "bending_moment_y": -a[4] - x * a[2], "bending_moment_z": -a[5] + x * a[1]}
            return sec[comp] * uf
        if q == "support_action_component":
            if owner["kind"] == "spring":
                d, k, g = spring_by_spring_id.get(owner["id"]) or spring_by_id[owner["id"]]
                node_comp = (ns["dof_map"][d]["node_id"], SUPPORT_COMPONENTS[comp])
                if dof_of[node_comp] != d:
                    return F(0)
                return -k * (u[d] - g) * uf
            dofs = support_dofs.get(owner["id"], set())
            spring_dofs = {sd: (k, g) for sd, k, g, s in springs if s["support_id"] == owner["id"]}
            if not dofs and not spring_dofs:
                return F(0)
            node = ns["dof_map"][next(iter(dofs or spring_dofs))]["node_id"]
            d = dof_of[(node, SUPPORT_COMPONENTS[comp])]
            if d in spring_dofs and d not in dofs:
                k, g = spring_dofs[d]
                return -k * (u[d] - g) * uf
            if d not in dofs:
                return F(0)
            r = sum((sum((fr["G"][i][j] * u[fr["scatter"][j]] for j in range(12) if fr["G"][i][j] != 0), F(0))
                     for fr in frames.values() for i in range(12) if fr["scatter"][i] == d), F(0))
            r += sum((k * (u[sd] - g) for sd, k, g, _ in springs if sd == d), F(0))
            return (r - f[d] + sum((k * g for sd, k, g, _ in springs if sd == d), F(0))) * uf
        raise ValueError(f"unknown quantity {q}")

    def plan_value(fn):
        total = F(0)
        for term in fn["displacement_terms"]:
            coeff = F(0)
            for ct in term["coefficient_terms"]:
                prod = F(1)
                for fac in ct["factors"]:
                    prod *= b2q(fac["bits"])
                coeff += prod
            total += coeff * u[term["dof"]]
        for ot in fn["offset_terms"]:
            prod = F(1)
            for fac in ot["factors"]:
                prod *= b2q(fac["bits"])
            total += prod
        return total * b2q(fn["final_unit_factor"]["bits"])

    bad_paths = 0
    for fn in fp["functions"]:
        for term in fn["displacement_terms"]:
            for ct in term["coefficient_terms"]:
                for fac in ct["factors"]:
                    if fac.get("kind") == "source" and pointer(ns, fac["source_path"]) != fac["bits"]:
                        bad_paths += 1
        for ot in fn["offset_terms"]:
            for fac in ot["factors"]:
                if fac.get("kind") == "source" and pointer(ns, fac["source_path"]) != fac["bits"]:
                    bad_paths += 1
    case.check("b_plan_operands_resolve", bad_paths == 0, f"{bad_paths} plan factors do not resolve to their payload bits")

    exact = {}
    by_id = {}
    for idx, fn in enumerate(fp["functions"]):
        case.check("b_functional_unit_is_public_unit", fn["unit"] == public_unit(fn["quantity"], fn["component"]),
                   f"{fn['functional_id']} ({fn['quantity']} {fn['component']}): unit {fn['unit']!r} is not the public {public_unit(fn['quantity'], fn['component'])!r}")
        x = physics_value(fn)
        pv = plan_value(fn)
        exact[fn["functional_id"]] = x
        by_id[fn["functional_id"]] = (idx, fn)
        case.check("b_plan_lowering_equals_statics", pv == x,
                   f"{fn['functional_id']} ({fn['quantity']} {fn['component']} {fn['location']}): plan {float(pv):.17g} != statics {float(x):.17g}")

    return {"frames": frames, "exact": exact, "by_id": by_id, "u": u, "eig": eig}


STRESS_COMPONENTS = ("axial_normal_stress", "bending_normal_stress_y", "bending_normal_stress_z", "torsional_shear_stress")
INTERIOR = ("quarter_1", "midspan", "quarter_3")


def inventory(case, ns, fp, rc, extrema, case_rows, frames):
    """Complete, unique inventories (repair round 1, item 1).

    The expected sets come from the declared problem (its nodes, frames and supports) and the case's published
    row universe; every receipt row, functional, projection and evidence item must be present exactly once,
    and every reference must resolve. Returns lookups used by the value checks."""
    def unique(name, items, what):
        dup = sorted({x for x in items if items.count(x) > 1}, key=str)
        case.check("i_unique_" + name, not dup, f"duplicate {what}: {dup[:5]}")

    fns = fp["functions"]
    unique("functionals", [f["functional_id"] for f in fns], "functional ids")
    nodes = []
    for d in ns["dof_map"]:
        if d["node_id"] not in nodes:
            nodes.append(d["node_id"])
    supports = sorted({p["support_id"] for p in ns["prescribed"]} | {s["support_id"] for s in ns["springs"]})
    springs = sorted(s["spring_id"] for s in ns["springs"])
    expected = {}
    for node in nodes:
        for comp in COMPONENTS6:
            expected[("nodal_translation" if comp[0] == "U" else "nodal_rotation", node, "node", comp)] = True
    for eid in frames:
        for comp in ACTION_INDEX:
            for loc in ("end_i", "end_j"):
                expected[("member_end_action", eid, loc, comp)] = True
            for loc in STATION_FRACTION:
                expected[("member_station_action", eid, loc, comp)] = loc in INTERIOR
    for sup in supports:
        for comp in SUPPORT_COMPONENTS:
            expected[("support_action_component", sup, "node", comp)] = True
    got = {}
    for f in fns:
        key = (f["quantity"], f["owner"]["id"], f["location"], f["component"])
        if f["quantity"] == "support_action_component" and f["owner"]["kind"] == "spring":
            case.check("i_spring_functional_known", f["owner"]["id"] in springs, f"spring functional for unknown spring {f['owner']['id']}")
            continue
        got.setdefault(key, []).append(f)
    missing = [k for k in expected if k not in got]
    extra = [k for k in got if k not in expected]
    dups = [k for k, v in got.items() if len(v) > 1]
    case.check("i_functional_inventory", not missing and not extra and not dups,
               f"functionals missing {missing[:4]}, unexpected {extra[:4]}, duplicated {dups[:4]}")
    for key, published in expected.items():
        for f in got.get(key, []):
            case.check("i_published_functionals", bool(f["result_ids"]) == published,
                       f"{key}: published rows {f['result_ids']} but expected {'published' if published else 'unpublished'}")

    # projections: one per published functional, bound to its row
    proj = rc.get("projections", [])
    unique("projection_ids", [p["projection_id"] for p in proj], "projection ids")
    unique("projection_functionals", [p["functional_id"] for p in proj], "projection functionals")
    by_fid = {p["functional_id"]: p for p in proj}
    published_fns = [f for f in fns if f["result_ids"]]
    lacking = [f["functional_id"] for f in published_fns if f["functional_id"] not in by_fid]
    stray = [p["projection_id"] for p in proj if not any(f["functional_id"] == p["functional_id"] for f in published_fns)]
    case.check("i_projection_inventory", not lacking and not stray and len(proj) == len(published_fns),
               f"{len(proj)} projections for {len(published_fns)} published functionals; lacking {lacking[:4]}, stray {stray[:4]}")

    # the receipt's rows are exactly the case's published rows
    rows = rc.get("rows", [])
    unique("receipt_rows", [r["result_id"] for r in rows], "receipt row ids")
    row_ids = {r["result_id"] for r in rows}
    case.check("i_rows_are_published_universe", row_ids == set(case_rows) and bool(row_ids),
               f"receipt rows {len(row_ids)} vs published rows of the case {len(case_rows)}; "
               f"unpublished {sorted(row_ids - set(case_rows))[:3]}, unreceipted {sorted(set(case_rows) - row_ids)[:3]}")
    stress = {sc["result_id"]: sc for sc in rc.get("section_stress_checks", [])}
    unique("stress_checks", [sc["result_id"] for sc in rc.get("section_stress_checks", [])], "stress check result ids")
    derived = {dc["result_id"]: dc for dc in rc.get("derived_checks", [])}
    unique("derived_checks", [dc["result_id"] for dc in rc.get("derived_checks", [])], "derived check result ids")
    maxima = {mx["result_id"]: mx for mx in extrema}
    unique("maxima", [mx["result_id"] for mx in extrema], "maximum result ids")
    proj_by_id = {p["projection_id"]: p for p in proj}
    used = {"projection": set(), "stress": set(), "maximum": set(), "derived": set()}
    translation_norms = {}
    for r in rows:
        rid, t, recipe = r["result_id"], r["treatment"], r["recipe_id"]
        refs_ok = all(i in case_rows for i in r.get("input_result_ids", []))
        case.check("i_row_inputs_published", refs_ok, f"{rid}: input rows {r.get('input_result_ids')} are not all published")
        if t == "qualified_projection":
            p = proj_by_id.get(r["projection_id"])
            case.check("i_row_evidence", p is not None and p["result_id"] == rid, f"{rid}: projection {r['projection_id']} missing or bound elsewhere")
            used["projection"].add(r["projection_id"])
        elif t == "checked_derived" and recipe == "retained_source_straight_stress_v1":
            case.check("i_row_evidence", rid in stress, f"{rid}: no section stress check")
            used["stress"].add(rid)
        elif t == "checked_derived" and recipe == "retained_source_endpoint_normal_max_v1":
            case.check("i_row_evidence", rid in maxima, f"{rid}: no endpoint maximum evidence")
            used["maximum"].add(rid)
        elif t == "checked_derived" and recipe in ("support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1"):
            case.check("i_row_evidence", rid in derived and derived[rid]["recipe_id"] == recipe, f"{rid}: no {recipe} check")
            used["derived"].add(rid)
        elif t == "checked_derived" and recipe == "translation_norm_scaled_v1":
            node = next((f["owner"]["id"] for f in fns if f["quantity"] == "nodal_translation" and r["input_result_ids"]
                         and r["input_result_ids"][0] in f["result_ids"]), None)
            case.check("i_row_evidence", node is not None and len(r["input_result_ids"]) == 3, f"{rid}: translation norm inputs do not name a node's translations")
            if node is not None:
                translation_norms[rid] = node
        elif t == "derived_not_qualified":
            pass  # U4b's derived rows: inputs must exist (checked above); not judged numerically
        else:
            case.check("i_row_evidence", False, f"{rid}: unexpected treatment/recipe {t}/{recipe}")
    case.check("i_evidence_referenced", used["projection"] == set(proj_by_id) and used["stress"] == set(stress)
               and used["maximum"] == set(maxima) and used["derived"] == set(derived),
               "evidence not referenced by exactly the receipt rows: "
               f"projections {len(used['projection'])}/{len(proj_by_id)}, stress {len(used['stress'])}/{len(stress)}, "
               f"maxima {len(used['maximum'])}/{len(maxima)}, derived {len(used['derived'])}/{len(derived)}")

    # expected cardinalities from the declared problem
    want_stress = {(eid, loc, comp, basis) for eid in frames for loc in STATION_FRACTION for comp in STRESS_COMPONENTS
                   for basis in ("nominal_section", "reduced_section")}
    have_stress = [(sc["pipe_id"], sc["location"], sc["component"],
                    "reduced_section" if sc["result_id"].endswith(":reduced-section") else "nominal_section")
                   for sc in rc.get("section_stress_checks", [])]
    case.check("i_stress_inventory", set(have_stress) == want_stress and len(have_stress) == len(want_stress),
               f"{len(have_stress)} stress checks for {len(want_stress)} expected (5 stations x 4 components x 2 bases per member)")
    want_max = {(eid, b) for eid in frames for b in ("nominal_section", "reduced_section")}
    have_max = [(mx["pipe_id"], mx["section_basis"]) for mx in extrema]
    case.check("i_maximum_inventory", set(have_max) == want_max and len(have_max) == len(want_max),
               f"{len(have_max)} endpoint maxima for {len(want_max)} expected (one per member and basis)")
    want_der = {(sup, kind) for sup in supports for kind in ("support_force_norm_scaled_checked_v1", "support_moment_norm_scaled_checked_v1")}
    have_der = [(dc["support_id"], dc["recipe_id"]) for dc in rc.get("derived_checks", [])]
    case.check("i_support_norm_inventory", set(have_der) == want_der and len(have_der) == len(want_der),
               f"{len(have_der)} support norm checks for {len(want_der)} expected (force and moment per support)")
    case.check("i_translation_norm_inventory", sorted(translation_norms.values()) == sorted(nodes),
               f"translation norm rows for {sorted(translation_norms.values())}, nodes {nodes}")
    src = rc.get("source") or {}
    es = [e["pipe_id"] for e in src.get("endpoint_sections", [])]
    sf = [e["pipe_id"] for e in src.get("section_functionals", [])]
    case.check("i_endpoint_sections_inventory", sorted(es) == sorted(frames) and all(
        sorted(ep["station_fraction"] for ep in e["endpoints"]) == [0, 1] for e in src.get("endpoint_sections", [])),
               f"endpoint sections {es} for members {sorted(frames)}")
    case.check("i_section_functionals_inventory", sorted(sf) == sorted(frames) and all(
        sorted(st["station_fraction"] for st in e["stations"]) == [0, 0.25, 0.5, 0.75, 1] for e in src.get("section_functionals", [])),
               f"section functionals {sf} for members {sorted(frames)}")
    return {"projections": by_fid, "translation_norms": translation_norms}


RELATIVE_LIMIT = F(1e-9)  # the receipt's relative_limit constant, the binary64 1e-9 (schema const)
AXES = {"X": "x", "Y": "y", "Z": "z"}


def public_identity(row):
    """A published row's public identity: what it claims to be, independent of its id and of any evidence."""
    m = row.get("metadata") or {}
    b = m.get("basis") or ""
    basis = b.split("section_basis=", 1)[1].split(";")[0].strip() if "section_basis=" in b else None
    return (row.get("entity_ref"), row.get("kind"), m.get("component"), m.get("location"), m.get("coordinate_system"), row.get("unit"), basis)


def expected_public_rows(ns, frames):
    """Round 2 (a): every public row of a selected case, derived from the declared problem (nodes, members,
    supports), each mapped to the evidence it must carry. Kinds and metadata are the public vocabulary of the
    pressure-source-1 rows."""
    nodes = []
    for d in ns["dof_map"]:
        if d["node_id"] not in nodes:
            nodes.append(d["node_id"])
    supports = sorted({p["support_id"] for p in ns["prescribed"]} | {s["support_id"] for s in ns["springs"]})
    want = {}
    for node in nodes:
        for comp in COMPONENTS6:
            axis = comp[1].lower()
            if comp[0] == "U":
                ident = (node, f"global_nodal_displacement_{axis}", f"nodal_displacement_{axis}", "node", "global", "mm", None)
                want[ident] = ("projection", ("nodal_translation", node, "node", comp))
            else:
                ident = (node, f"global_nodal_rotation_{axis}", f"nodal_rotation_{axis}", "node", "global", "rad", None)
                want[ident] = ("projection", ("nodal_rotation", node, "node", comp))
        want[(node, "displacement_magnitude", None, None, None, "mm", None)] = ("translation_norm", node)
    for m in frames:
        for comp in ACTION_INDEX:
            unit = "N" if ACTION_INDEX[comp] < 3 else "N*m"
            for loc in ("end_i", "end_j"):
                want[(m, f"element_local_{comp}", comp, loc, "element_local", unit, None)] = ("projection", ("member_end_action", m, loc, comp))
            for loc in INTERIOR:
                want[(m, f"element_local_{comp}", comp, loc, "element_local", unit, None)] = ("projection", ("member_station_action", m, loc, comp))
        for loc in STATION_FRACTION:
            for basis in ("nominal_section", "reduced_section"):
                for comp in STRESS_COMPONENTS:
                    want[(m, f"element_local_{comp}", comp, loc, "element_local", "MPa", basis)] = ("stress", m, loc, comp, basis)
                stress_inputs = [(m, f"element_local_{c}", c, loc, "element_local", "MPa", basis)
                                 for c in ("axial_normal_stress", "bending_normal_stress_y", "bending_normal_stress_z")]
                for extreme in ("sigma_plus", "sigma_minus"):
                    want[(m, "element_local_normal_stress_extreme", extreme, loc, "pipe_section", "MPa", basis)] = ("dnq", stress_inputs)
                shear_inputs = [(m, f"element_local_{c}", c, loc, "element_local", "N", None) for c in ("shear_force_y", "shear_force_z")]
                want[(m, "element_local_transverse_shear_stress_maximum", "tau_v_max", loc, "pipe_section", "MPa", basis)] = ("dnq", shear_inputs)
        for basis in ("nominal_section", "reduced_section"):
            want[(m, "pipe_elastic_normal_stress_maximum_v2", "maximum_absolute_normal_stress", "governing_station", "pipe_section", "Pa", basis)] = ("maximum", m, basis)
    for sup in supports:
        for comp in SUPPORT_COMPONENTS:
            want[(sup, "support_reaction_component_v2", comp, "node", "global", "N" if comp[0] == "F" else "N*m", None)] = \
                ("projection", ("support_action_component", sup, "node", comp))
        want[(sup, "support_reaction_force_magnitude_v2", "force_magnitude", "node", "global", "N", None)] = ("support_norm", sup, "force")
        want[(sup, "support_reaction_moment_magnitude_v2", "moment_magnitude", "node", "global", "N*m", None)] = ("support_norm", sup, "moment")
    return want


def public_binding(case, ns, fp, rc, extrema, case_rows, frames, by_id):
    """Round 2 (b): bind every evidence item to the public row it vouches for by that row's full identity.

    Expectations come from the row's identity (entity, kind, component, location, basis, unit), never from the
    evidence's own labels; the evidence's labels must then agree. Returns the node of each translation norm row."""
    want = expected_public_rows(ns, frames)
    ident_of = {rid: public_identity(row) for rid, row in case_rows.items()}
    idents = list(ident_of.values())
    dup = sorted({i for i in idents if idents.count(i) > 1}, key=str)
    missing = [k for k in want if k not in idents]
    extra = [i for i in idents if i not in want]
    case.check("p_public_identity_map", not dup and not missing and not extra,
               f"public rows: duplicated identities {dup[:3]}, missing {missing[:3]}, unexpected {extra[:3]}")
    id_of = {}
    for rid, ident in ident_of.items():
        id_of.setdefault(ident, rid)
    fid_of = {}
    for fid, (_, fn) in by_id.items():
        if not (fn["quantity"] == "support_action_component" and fn["owner"]["kind"] == "spring"):
            fid_of.setdefault((fn["quantity"], fn["owner"]["id"], fn["location"], fn["component"]), fid)
    rows = {r["result_id"]: r for r in rc.get("rows", [])}
    proj = {p["projection_id"]: p for p in rc.get("projections", [])}
    stress = {sc["result_id"]: sc for sc in rc.get("section_stress_checks", [])}
    derived = {dc["result_id"]: dc for dc in rc.get("derived_checks", [])}
    maxima = {mx["result_id"]: mx for mx in extrema}
    stress_action = {"axial_normal_stress": "axial_force", "bending_normal_stress_y": "bending_moment_y",
                     "bending_normal_stress_z": "bending_moment_z", "torsional_shear_stress": "torsional_moment"}
    node_of_norm = {}
    for rid, ident in ident_of.items():
        spec = want.get(ident)
        r = rows.get(rid)
        if spec is None or r is None:
            continue
        kind = spec[0]
        if kind == "projection":
            p = proj.get(r.get("projection_id")) if r["treatment"] == "qualified_projection" else None
            case.check("p_projection_bound_to_row", p is not None and p["functional_id"] == fid_of.get(spec[1]) and p["result_id"] == rid,
                       f"{rid} {ident}: its projection is not the functional {spec[1]}")
        elif kind == "stress":
            _, m, loc, comp, basis = spec
            sc = stress.get(rid) if r["recipe_id"] == "retained_source_straight_stress_v1" else None
            leaves = basis_leaves(frames[m]["frame"], basis)
            ok = sc is not None and sc["functional_id"] == fid_of.get(("member_station_action", m, loc, stress_action[comp])) \
                and sc["pipe_id"] == m and sc["location"] == loc and sc["component"] == comp \
                and sc["result_id"].endswith(":reduced-section" if basis == "reduced_section" else ":nominal-section") \
                and f2b(sc["parameters"]["area_m2"]) == leaves["area"] and f2b(sc["parameters"]["section_modulus_m3"]) == leaves["section_modulus"] \
                and f2b(sc["parameters"]["torsion_constant_m4"]) == leaves["torsion_constant"] \
                and f2b(sc["parameters"]["torsion_radius_m"]) == leaves["torsion_radius"]
            case.check("p_stress_bound_to_row", ok, f"{rid} {ident}: its stress check is not the {stress_action[comp]} action of {m} at {loc} on {basis}")
        elif kind == "maximum":
            _, m, basis = spec
            mx = maxima.get(rid) if r["recipe_id"] == "retained_source_endpoint_normal_max_v1" else None
            leaves = basis_leaves(frames[m]["frame"], basis)
            ok = mx is not None and mx["pipe_id"] == m and mx["section_basis"] == basis \
                and f2b(mx["area_m2"]) == leaves["area"] and f2b(mx["section_modulus_m3"]) == leaves["section_modulus"]
            case.check("p_maximum_bound_to_row", ok, f"{rid} {ident}: its maximum evidence is not {m} on {basis}")
        elif kind == "support_norm":
            _, sup, which = spec
            recipe = f"support_{which}_norm_scaled_checked_v1"
            comps = ("Fx", "Fy", "Fz") if which == "force" else ("Mx", "My", "Mz")
            dc = derived.get(rid) if r["recipe_id"] == recipe else None
            expect_ids = [fid_of.get(("support_action_component", sup, "node", c)) for c in comps]
            ok = dc is not None and dc["support_id"] == sup and dc["recipe_id"] == recipe and list(dc["functional_ids"]) == expect_ids
            case.check("p_support_norm_bound_to_row", ok, f"{rid} {ident}: its norm check is not {sup}'s {which} components")
        elif kind == "translation_norm":
            node = spec[1]
            expect_ids = [id_of.get(i) for i, s_ in want.items() if s_[0] == "projection" and s_[1][0] == "nodal_translation" and s_[1][1] == node]
            got = list(r.get("input_result_ids", []))
            ok = r["recipe_id"] == "translation_norm_scaled_v1" and len(got) == 3 and len(set(got)) == 3 and set(got) == set(expect_ids)
            case.check("p_translation_inputs_are_the_node_translations", ok, f"{rid}: inputs {got} are not {node}'s distinct UX, UY, UZ rows {expect_ids}")
            node_of_norm[rid] = node
        elif kind == "dnq":
            expect_ids = [id_of.get(i) for i in spec[1]]
            got = list(r.get("input_result_ids", []))
            ok = r["treatment"] == "derived_not_qualified" and len(got) == len(set(got)) == len(expect_ids) and set(got) == set(expect_ids)
            case.check("p_derived_row_inputs", ok, f"{rid}: inputs {got} are not {expect_ids}")
    return node_of_norm


def judge_record(case, record, rc, ev, results, request, bound, theory, allowance, publication_link, dup_ids=()):
    # (a) commitments ---------------------------------------------------------------
    ns_text, fp_text = record["normalized_source_text"], record["functional_plan_text"]
    ns_hash = hashlib.sha256(ns_text.encode("utf-8")).hexdigest()
    fp_hash = hashlib.sha256(fp_text.encode("utf-8")).hexdigest()
    case.check("a_publication_link", bool(publication_link), "dump publication_sha256 differs from the receipt's")
    case.check("a_record_hashes", ns_hash == record["normalized_source_sha256"] and fp_hash == record["functional_plan_sha256"],
               "a record text does not hash to the record's own sha256")
    if rc is None:
        case.check("a_receipt_case", False, "no receipt case for this record")
        return
    src = rc.get("source") or {}
    case.check("a_receipt_hashes", ns_hash == src.get("normalized_source_sha256") and fp_hash == src.get("functional_plan_sha256"),
               f"text hashes {ns_hash[:12]}/{fp_hash[:12]} differ from the receipt's {str(src.get('normalized_source_sha256'))[:12]}/{str(src.get('functional_plan_sha256'))[:12]}")
    case.check("a_selected", rc["selected_method"] == "retained_source_blocks_exact_v1", "receipt case is not source-selected")
    ns_doc, fp_doc = json.loads(ns_text), json.loads(fp_text)
    case.check("a_domains", ns_doc.get("domain") == "source_blocks_normalized_source_v1" and fp_doc.get("domain") == "source_blocks_functional_plan_v1",
               "unexpected commitment domains")
    ns, fp = ns_doc["payload"], fp_doc["payload"]
    case.check("a_payload_case", ns.get("case_id") == record["case_id"], "payload case_id differs from the record's")
    case.check("a_plan_binds_payload", fp.get("normalized_source_sha256") == ns_hash, "plan's normalized_source_sha256 differs")

    ctx = solve_declared(ns, fp, case, src)
    frames, exact, by_id, eig = ctx["frames"], ctx["exact"], ctx["by_id"], ctx["eig"]

    def contains(name, interval, x, where, value=None):
        lo, hi = qf(interval[0]), qf(interval[1])
        ok = lo <= x <= hi and lo <= hi
        if value is not None:
            ok = ok and lo <= qf(value) <= hi
        case.check(name, ok, f"{where}: interval [{interval[0]!r}, {interval[1]!r}] (value {value!r}) does not contain exact {float(x):.17g}")
        if ok and lo < hi:
            slack = min(x - lo, hi - x)
            if x != 0:
                case.metric_min(name + ":min_endpoint_slack_rel", slack / abs(x), where)
                if value is not None:
                    case.metric_max(name + ":max_value_rel_error", abs(qf(value) - x) / abs(x), where)
        return ok

    # inventories: every published row of the case is accounted for, every expected functional and
    # evidence item exists exactly once, and every reference resolves (repair round 1, item 1)
    cid = record["case_id"]
    case.check("i_unique_published_rows", not dup_ids, f"duplicate published row ids (a dict would keep only one): {list(dup_ids)[:5]}")
    case_rows = {}
    for row in results.values():
        if (row.get("basis_ref") or {}).get("ref_id") == cid:
            case_rows[row["id"]] = row
    ev_extrema = [mx for mx in (ev or {}).get("pipe_stress_extrema", []) if mx.get("basis") == "retained_source_endpoint_normal_max_v1"]
    inv = inventory(case, ns, fp, rc, ev_extrema, case_rows, frames)
    projections = inv["projections"]
    node_of_norm = public_binding(case, ns, fp, rc, ev_extrema, case_rows, frames, by_id)

    def action_contains(name, act, x, where):
        """An evidence action: its interval holds the exact value and its value is within the relative limit."""
        contains(name, act["interval"], x, where, act["value"])
        a, lo, hi = qf(act["value"]), qf(act["interval"][0]), qf(act["interval"][1])
        case.check("b_action_accuracy", lo <= hi and rel_in(x, a, RELATIVE_LIMIT) and hi - lo <= 2 * RELATIVE_LIMIT * abs(a),
                   f"{where}: action {act['value']!r} [{act['interval'][0]!r}, {act['interval'][1]!r}] is not within 1e-9 of exact {float(x):.17g}")

    for p in rc["projections"]:
        fid = p["functional_id"]
        if fid not in exact:
            case.check("b_projection_has_functional", False, f"{p['projection_id']}: functional {fid} not in the plan")
            continue
        x = exact[fid]
        where = f"{p['projection_id']} {p['result_id']}"
        fn_ = by_id[fid][1]
        case.check("b_projection_unit_is_public_unit", p["unit"] == public_unit(fn_["quantity"], fn_["component"]),
                   f"{p['projection_id']}: unit {p['unit']!r} is not the public {public_unit(fn_['quantity'], fn_['component'])!r}")
        case.check("b_projection_result_is_functional_row", p["result_id"] in by_id[fid][1]["result_ids"],
                   f"{where}: result_id is not one of the functional's published rows")
        contains("b_projection_contains_exact", p["interval"], x, where, p["value"])
        case.check("b_projection_value_bits", f2b(p["value"]) == p["value_bits"], f"{where}: value and value_bits disagree")
        if p["basis"] == "exact_zero":
            case.check("b_projection_basis", x == 0 and p["value"] == 0 and qf(p["interval"][0]) == 0 == qf(p["interval"][1]),
                       f"{where}: exact_zero but exact value {float(x):.17g}")
        elif p["basis"] == "exact_identity":
            case.check("b_projection_basis", qf(p["value"]) == x, f"{where}: exact_identity but exact value {float(x):.17g} != {p['value']!r}")
        err = abs(qf(p["value"]) - x)
        ab = qf(p["absolute_error_bound"])
        case.check("b_projection_absolute_bound", err <= ab, f"{where}: |value - exact| = {float(err):.3e} > absolute_error_bound {p['absolute_error_bound']!r}")
        v, rl = qf(p["value"]), qf(p["relative_error_bound"])
        lo_, hi_ = qf(p["interval"][0]), qf(p["interval"][1])
        claims = ab >= 0 and 0 <= rl <= RELATIVE_LIMIT and qf(p["relative_limit"]) == RELATIVE_LIMIT and lo_ <= hi_ \
            and v - ab <= lo_ and hi_ <= v + ab and (ab <= rl * abs(v) * (1 + F(1, 2 ** 50)) if v else ab == 0)
        case.check("b_projection_claims_consistent", claims,
                   f"{where}: bound claims inconsistent (abs {p['absolute_error_bound']!r}, rel {p['relative_error_bound']!r}, interval {p['interval']!r})")
        case.check("b_projection_relative_claim", x == 0 or rel_in(x, v, rl), f"{where}: |value - exact| exceeds the claimed relative bound {p['relative_error_bound']!r}")
        if ab > 0:
            case.metric_max("b_projection_absolute_bound:max_usage", err / ab, where)
        row = case_rows.get(p["result_id"])
        case.check("b_projection_published_row", row is not None and row["value"] == p["value"] and f2b(float(row["value"])) == p["value_bits"]
                   and row.get("unit") == p["unit"],
                   f"{where}: published row {None if row is None else (row['value'], row.get('unit'))!r} differs from the projection value {p['value']!r} {p['unit']}")

    def bound_functionals(where, ids, indices, expect):
        """ids/indices agree with the plan, and each functional is the expected (quantity, owner, location, component)."""
        ok = len(ids) == len(indices) == len(expect)
        for fid, idx, (quantity, owner, location, component) in zip(ids, indices, expect):
            got = by_id.get(fid)
            ok = ok and got is not None and got[0] == idx
            if got is not None:
                fn = got[1]
                ok = ok and fn["quantity"] == quantity and fn["owner"]["id"] == owner and fn["component"] == component \
                    and (location is None or fn["location"] == location)
        case.check("b_evidence_bound_to_expected_functionals", ok, f"{where}: functionals {ids}/{indices} are not {expect}")
        return ok

    srcblk = rc["source"]
    for es in srcblk.get("endpoint_sections", []):
        for ep in es["endpoints"]:
            loc = {0: "end_i", 1: "end_j"}.get(ep["station_fraction"])
            bound_functionals(f"endpoint {es['pipe_id']} {ep['station_fraction']}", ep["functional_ids"], ep["functional_indices"],
                              [("member_station_action", es["pipe_id"], loc, c) for c in ("axial_force", "bending_moment_y", "bending_moment_z")])
            case.check("b_evidence_list_lengths", len(ep["actions"]) == len(ep["functional_ids"]) == 3, f"endpoint {es['pipe_id']}: list lengths")
            for fid, act in zip(ep["functional_ids"], ep["actions"]):
                if fid in exact:
                    action_contains("b_endpoint_action_contains_exact", act, exact[fid], f"endpoint {es['pipe_id']} {ep['station_fraction']} {fid}")
    station_loc = {0: "end_i", 0.25: "quarter_1", 0.5: "midspan", 0.75: "quarter_3", 1: "end_j"}
    for sf in srcblk.get("section_functionals", []):
        for st in sf["stations"]:
            loc = station_loc.get(st["station_fraction"])
            comps = [by_id[f][1]["component"] if f in by_id else None for f in st["functional_ids"]]
            case.check("b_evidence_list_lengths", len(st["actions"]) == len(st["functional_ids"]) == len(st["functional_indices"]) == 6
                       and sorted(c for c in comps if c) == sorted(ACTION_INDEX), f"station {sf['pipe_id']} {st['station_fraction']}: not the six actions")
            bound_functionals(f"station {sf['pipe_id']} {st['station_fraction']}", st["functional_ids"], st["functional_indices"],
                              [("member_station_action", sf["pipe_id"], loc, c) for c in comps])
            for fid, act in zip(st["functional_ids"], st["actions"]):
                if fid in exact:
                    action_contains("b_station_action_contains_exact", act, exact[fid], f"station {sf['pipe_id']} {st['station_fraction']} {fid}")
    for dc in rc.get("derived_checks", []):
        names = ("Fx", "Fy", "Fz") if dc["recipe_id"] == "support_force_norm_scaled_checked_v1" else ("Mx", "My", "Mz")
        unit = "N" if names[0] == "Fx" else "N*m"
        lengths = len(dc["functional_ids"]) == len(dc["functional_indices"]) == len(dc["values"]) == len(dc["intervals"]) == 3
        case.check("b_evidence_list_lengths", lengths, f"{dc['result_id']}: component lists are not three long")
        if not lengths:
            continue
        if not bound_functionals(dc["result_id"], dc["functional_ids"], dc["functional_indices"],
                                 [("support_action_component", dc["support_id"], None, c) for c in names]):
            continue
        comps = []
        for fid, iv, val in zip(dc["functional_ids"], dc["intervals"], dc["values"]):
            contains("b_derived_component_contains_exact", iv, exact[fid], f"{dc['result_id']} {fid}", val)
            proj = projections.get(fid)
            case.check("b_derived_component_is_projection", proj is not None and proj["value"] == val and list(proj["interval"]) == list(iv),
                       f"{dc['result_id']} {fid}: component value {val!r} is not its projection's")
            comps.append(exact[fid])
        s2 = sum((c * c for c in comps), F(0))
        lo, hi = qf(dc["interval"][0]), qf(dc["interval"][1])
        ok = sqrt_in(s2, lo, hi)
        case.check("b_derived_norm_contains_exact", ok, f"{dc['result_id']}: [{dc['interval'][0]!r}, {dc['interval'][1]!r}] does not contain sqrt({float(s2):.17g})")
        # the published value (round 1 item 3, round 2 (e)): sign-safe, nonnegative claims, and both the
        # absolute and the relative claim must hold against the exact norm independently
        v, ab, rl = qf(dc["value"]), qf(dc["absolute_error_bound"]), qf(dc["relative_error_bound"])
        claims = v >= 0 and ab >= 0 and 0 <= lo <= v <= hi and 0 <= rl <= RELATIVE_LIMIT and qf(dc["relative_limit"]) == RELATIVE_LIMIT \
            and v - ab <= lo and hi <= v + ab and (ab <= rl * v * (1 + F(1, 2 ** 50)) if v else ab == 0)
        case.check("b_derived_norm_claims_consistent", claims,
                   f"{dc['result_id']}: claims inconsistent (value {dc['value']!r}, abs {dc['absolute_error_bound']!r}, rel {dc['relative_error_bound']!r}, interval {dc['interval']!r})")
        case.check("b_derived_norm_value", v >= 0 and sqrt_in(s2, v - ab, v + ab),
                   f"{dc['result_id']}: value {dc['value']!r} (bound {dc['absolute_error_bound']!r}) does not hold sqrt({float(s2):.17g})")
        case.check("b_derived_norm_relative_claim", (v == 0) if s2 == 0 else (rl < 1 and sqrt_in(s2, v / (1 + rl), v / (1 - rl))),
                   f"{dc['result_id']}: value {dc['value']!r} is not within the claimed relative bound {dc['relative_error_bound']!r} of sqrt({float(s2):.17g})")
        row = case_rows.get(dc["result_id"])
        case.check("b_derived_norm_published_row", row is not None and row["value"] == dc["value"] and row.get("unit") == unit,
                   f"{dc['result_id']}: published row {None if row is None else (row['value'], row.get('unit'))!r} is not the check's value {dc['value']!r} {unit}")
    for rid, node in node_of_norm.items():
        comps = [exact.get(next((f for f, (_, fn) in by_id.items() if fn["quantity"] == "nodal_translation" and fn["owner"]["id"] == node and fn["component"] == c), None)) for c in ("UX", "UY", "UZ")]
        row = case_rows[rid]
        if None in comps:
            case.check("b_translation_norm", False, f"{rid}: node {node} lacks translation functionals")
            continue
        s2 = sum((c * c for c in comps), F(0))
        v = qf(row["value"])
        tol = F(1, 10 ** 9)
        ok = (v == 0) if s2 == 0 else (v >= 0 and (v * v <= s2 * (1 + tol) ** 2) and (v * v >= s2 * (1 - tol) ** 2))
        case.check("b_translation_norm", ok and row.get("unit") == "mm", f"{rid}: published {row['value']!r} is not |u| = sqrt({float(s2):.17g}) mm within 1e-9")

    # (c) stress checks and maxima ------------------------------------------------------
    member_req = {eid: (request_member(request, eid) if request else None) for eid in frames}
    nominal_checks = {}
    stress_action = {"axial_normal_stress": "axial_force", "bending_normal_stress_y": "bending_moment_y",
                     "bending_normal_stress_z": "bending_moment_z", "torsional_shear_stress": "torsional_moment"}
    for sc in rc.get("section_stress_checks", []):
        rid = sc["result_id"]
        basis = "reduced_section" if rid.endswith(":reduced-section") else ("nominal_section" if rid.endswith(":nominal-section") else None)
        if basis is None:
            case.check("c_stress_basis_named", False, f"{rid}: no basis suffix")
            continue
        fr = frames.get(sc["pipe_id"])
        if fr is None:
            case.check("c_stress_pipe", False, f"{rid}: pipe {sc['pipe_id']} is not a declared frame")
            continue
        leaves = basis_leaves(fr["frame"], basis)
        par = sc["parameters"]
        same = (f2b(par["area_m2"]) == leaves["area"] and f2b(par["section_modulus_m3"]) == leaves["section_modulus"]
                and f2b(par["torsion_constant_m4"]) == leaves["torsion_constant"] and f2b(par["torsion_radius_m"]) == leaves["torsion_radius"]
                and par["pa_per_mpa"] == 1e6)
        case.check("c_stress_parameters_are_basis_leaves", same, f"{rid}: parameters are not the payload's {basis} leaves")
        mr = member_req.get(sc["pipe_id"])
        if mr and None not in (mr["D"], mr["t"], mr["cm"], mr["ca"]):
            wall = mr["t"] if basis == "nominal_section" else (mr["t"] - mr["cm"]) - mr["ca"]
            an = annulus(F(mr["D"]), F(wall))
            rel = F(2) ** bound["supplementary"]["section_leaf_screen_log2_relative"]
            ok = (within_screen(qf(par["area_m2"]), an["A"], rel) and within_screen(qf(par["section_modulus_m3"]), an["Z"], rel)
                  and within_screen(qf(par["torsion_constant_m4"]), an["J"], rel) and qf(par["torsion_radius_m"]) == an["ro"])
            case.check("c_stress_parameters_screen_basis_annulus", ok, f"{rid}: parameters are not the {basis} annulus (wall {wall!r})")
        else:
            case.not_judged("c_stress_parameters_screen_basis_annulus", f"{sc['pipe_id']}: section lengths do not convert to SI binary64 exactly")
        fid = sc["functional_id"]
        x = exact.get(fid)
        comp = sc["component"]
        if comp not in stress_action:
            case.check("c_stress_component", False, f"{rid}: unknown component {comp}")
            continue
        if x is None or not bound_functionals(rid, [fid], [sc["functional_index"]],
                                              [("member_station_action", sc["pipe_id"], sc["location"], stress_action[comp])]):
            case.check("c_stress_bound_to_its_action", False, f"{rid}: functional {fid}/{sc['functional_index']} is not the {stress_action[comp]} station action of {sc['pipe_id']} at {sc['location']}")
            continue
        case.check("c_stress_bound_to_its_action", True)
        action_contains("c_stress_action_contains_exact", sc["action"], x, rid)
        a = qf(sc["action"]["value"])
        if comp == "axial_normal_stress":
            pa, ops = a / qf(par["area_m2"]), 2
            nat = (sc["action"]["value"] / par["area_m2"]) / par["pa_per_mpa"]
        elif comp in ("bending_normal_stress_y", "bending_normal_stress_z"):
            pa, ops = a / qf(par["section_modulus_m3"]), 2
            nat = (sc["action"]["value"] / par["section_modulus_m3"]) / par["pa_per_mpa"]
        else:
            pa, ops = a * qf(par["torsion_radius_m"]) / qf(par["torsion_constant_m4"]), 3
            nat = (sc["action"]["value"] * par["torsion_radius_m"] / par["torsion_constant_m4"]) / par["pa_per_mpa"]
        sigma = pa / qf(par["pa_per_mpa"])
        row = case_rows.get(rid)
        if row is None:
            case.check("c_stress_row_published", False, f"{rid}: no published row")
        else:
            v = qf(row["value"])
            case.check("c_stress_row_unit", row.get("unit") == "MPa", f"{rid}: unit {row.get('unit')}")
            ok = abs(v - sigma) <= rho(ops) * abs(sigma)
            case.check("c_stress_recomputes", ok, f"{rid}: published {row['value']!r} MPa is not within rho_{ops} of {float(sigma):.17g} recomputed from action {sc['action']['value']!r} and the {basis} parameters")
            case.metrics["c_stress_natural_order_bit_mismatches"] = (
                case.metrics.get("c_stress_natural_order_bit_mismatches", (0, None))[0] + (row["value"] != nat), "count")
            sig_exact = (x / qf(par["area_m2"]) if comp == "axial_normal_stress" else
                         x / qf(par["section_modulus_m3"]) if comp.startswith("bending") else
                         x * qf(par["torsion_radius_m"]) / qf(par["torsion_constant_m4"])) / qf(par["pa_per_mpa"])
            if sig_exact != 0:
                case.metric_max("c_stress_row_rel_error_vs_exact_functional", abs(v - sig_exact) / abs(sig_exact), rid)
        key = rid[: -len(":reduced-section")] if basis == "reduced_section" else rid[: -len(":nominal-section")]
        if basis == "nominal_section":
            nominal_checks[key] = sc
        else:
            nom = nominal_checks.get(key)
            if nom is None:
                case.check("c_stress_basis_twin", False, f"{rid}: no nominal twin precedes it")
            else:
                case.check("c_stress_basis_twin", nom["functional_id"] == sc["functional_id"] and nom["functional_index"] == sc["functional_index"]
                           and nom["action"] == sc["action"] and nom["component"] == sc["component"] and nom["location"] == sc["location"],
                           f"{rid}: reduced check does not share functional and action with its nominal twin")
                distinct = frames[sc["pipe_id"]]["frame"].get("reduced_recovery_section") is not None
                case.check("c_stress_bases_distinct_iff_reduced_section", (nom["parameters"] != sc["parameters"]) == distinct,
                           f"{rid}: parameters {'equal' if not distinct else 'differ'} unexpectedly")
                if mr and None not in (mr["t"], mr["cm"], mr["ca"]):
                    nonzero = mr["cm"] != 0 or mr["ca"] != 0
                    walls_differ = ((mr["t"] - mr["cm"]) - mr["ca"]) != mr["t"]
                    case.check("c_reduced_section_present_iff_allowance", distinct == walls_differ,
                               f"{sc['pipe_id']}: reduced_recovery_section present={distinct} but allowance nonzero={nonzero}")
                else:
                    case.not_judged("c_reduced_section_present_iff_allowance", f"{sc['pipe_id']}: wall or allowances do not convert to SI binary64 exactly")
    for mx in ev_extrema:
        fr = frames.get(mx["pipe_id"])
        if fr is None:
            case.check("c_maximum_pipe", False, f"{mx['result_id']}: pipe {mx['pipe_id']} is not a declared frame")
            continue
        leaves = basis_leaves(fr["frame"], mx["section_basis"])
        case.check("c_maximum_parameters_are_basis_leaves", f2b(mx["area_m2"]) == leaves["area"] and f2b(mx["section_modulus_m3"]) == leaves["section_modulus"],
                   f"{mx['result_id']}: A/Z are not the payload's {mx['section_basis']} leaves")
        A_, Z_ = qf(mx["area_m2"]), qf(mx["section_modulus_m3"])
        lo, hi = qf(mx["value_lower_pa"]), qf(mx["value_upper_pa"])
        terms = []
        fractions = sorted(ep["station_fraction"] for ep in mx["endpoints"])
        case.check("b_evidence_list_lengths", fractions == [0, 1], f"{mx['result_id']}: endpoints are not the two ends")
        endpoints_ok = A_ > 0 and Z_ > 0
        for ep in mx["endpoints"]:
            loc = {0: "end_i", 1: "end_j"}.get(ep["station_fraction"])
            if not bound_functionals(f"{mx['result_id']} endpoint {ep['station_fraction']}", ep["functional_ids"], ep["functional_indices"],
                                     [("member_station_action", mx["pipe_id"], loc, c) for c in ("axial_force", "bending_moment_y", "bending_moment_z")]) \
                    or len(ep["actions"]) != 3:
                endpoints_ok = False
                continue
            N, My, Mz = (exact[fid] for fid in ep["functional_ids"])
            for fid, act in zip(ep["functional_ids"], ep["actions"]):
                action_contains("c_maximum_action_contains_exact", act, exact[fid], f"{mx['result_id']} {fid}")
            a_, s2 = abs(N) / A_, My * My + Mz * Mz
            terms.append((a_, s2))
            elo, ehi = qf(ep["interval_pa"][0]), qf(ep["interval_pa"][1])
            ev_ = qf(ep["value_pa"])
            case.check("c_maximum_endpoint_claims", 0 <= elo <= ev_ <= ehi and (ehi - elo <= 2 * RELATIVE_LIMIT * ev_ if ev_ else elo == ehi == 0),
                       f"{mx['result_id']} endpoint {ep['station_fraction']}: value_pa {ep['value_pa']!r} and interval_pa {ep['interval_pa']!r} are not a nonnegative enclosure within 1e-9")
            case.check("c_maximum_endpoint_contains_exact", Z_ > 0 and elo <= qf(ep["value_pa"]) <= ehi and max_in([(a_, s2)], Z_, elo, ehi),
                       f"{mx['result_id']} endpoint {ep['station_fraction']}: interval_pa does not contain the exact endpoint value")
        v, ab, rl = qf(mx["value_pa"]), qf(mx["absolute_error_bound_pa"]), qf(mx["relative_error_bound"])
        claims = v >= 0 and ab >= 0 and 0 <= lo <= v <= hi and 0 <= rl <= RELATIVE_LIMIT and qf(mx["relative_limit"]) == RELATIVE_LIMIT \
            and v - ab <= lo and hi <= v + ab and (ab <= rl * v * (1 + F(1, 2 ** 50)) if v else ab == 0)
        case.check("c_maximum_claims_consistent", claims,
                   f"{mx['result_id']}: claims inconsistent (value {mx['value_pa']!r}, abs {mx['absolute_error_bound_pa']!r}, rel {mx['relative_error_bound']!r}, [{mx['value_lower_pa']!r}, {mx['value_upper_pa']!r}])")
        case.check("c_maximum_contains_exact", endpoints_ok and max_in(terms, Z_, lo, hi) and max_in(terms, Z_, v - ab, v + ab),
                   f"{mx['result_id']}: [{mx['value_lower_pa']!r}, {mx['value_upper_pa']!r}] or value +- bound does not contain the exact maximum")
        case.check("c_maximum_relative_claim", endpoints_ok and rl < 1 and (max_in(terms, Z_, v / (1 + rl), v / (1 - rl)) if v else max_in(terms, Z_, F(0), F(0))),
                   f"{mx['result_id']}: value {mx['value_pa']!r} is not within the claimed relative bound {mx['relative_error_bound']!r} of the exact maximum")
        row = case_rows.get(mx["result_id"])
        if row is None:
            ok = False
        elif row.get("unit") == "Pa":
            ok = row["value"] == mx["value_pa"]
        elif row.get("unit") == "MPa":
            ok = abs(qf(row["value"]) * 10 ** 6 - qf(mx["value_pa"])) <= rho(1) * qf(mx["value_pa"])
        else:
            ok = False
        case.check("c_maximum_row", ok, f"{mx['result_id']}: published row {None if row is None else (row['value'], row.get('unit'))!r} is not value_pa {mx['value_pa']!r}")

    # (d) K_local against BOUND.md ------------------------------------------------------
    for eid, fr in frames.items():
        frame = fr["frame"]
        inputs = frame_inputs(frame)
        frame_theory = "timo" if inputs["As"] is not None else "eb"
        if theory is not None:
            case.check("d_frame_theory_matches_request", frame_theory == theory,
                       f"{eid}: frame is {frame_theory} but the request selects {theory}")
        if inputs["L"] is None:
            coords = frame["formation_inputs"].get("coordinates_bits")
            if coords:
                d2 = sum(((b2q(coords[1][k]) - b2q(coords[0][k])) ** 2 for k in range(3)), F(0))
                inputs["L"] = None
                case.notes.append(f"{eid}: no length leaf; L from coordinates with ell = 7/2")
                Llo, Lhi = sqrt_down(d2), sqrt_up(d2)
                oks = []
                for Lc in (Llo, Lhi):
                    inputs["L"] = Lc
                    oks.append(judge_k_local(bound, fr["Kl"], inputs, frame_theory, F(7, 2)))
                ok = all(o[0] for o in oks)
                case.check("d_k_local_within_bound", ok, "; ".join(sum((o[2] for o in oks), [])[:6]))
                case.metric_max("d_k_local_bound_usage", max(o[1][0] for o in oks), f"{eid} {oks[0][1][1]}")
                continue
            case.check("d_k_local_judged", False, f"{eid}: no length leaf and no coordinates; not judged")
            continue
        ok, worst, findings, phis = judge_k_local(bound, fr["Kl"], inputs, frame_theory, inputs["ell"])
        case.check("d_k_local_within_bound", ok, f"{eid}: " + "; ".join(findings[:6]))
        case.metric_max(f"d_k_local_bound_usage:{frame_theory}", worst[0], f"{eid} {worst[1]}")
        for role_name, usage in role_usage(bound, fr["Kl"], inputs, frame_theory, inputs["ell"]).items():
            case.metric_max(f"d_k_local_bound_usage:{frame_theory}:{role_name}", usage, eid)
        if frame_theory == "timo":
            case.metric_min("phi_min", min(phis.values()), eid)
            case.metric_max("phi_max", max(phis.values()), eid)
        coords = frame["formation_inputs"].get("coordinates_bits")
        if coords:
            d2 = sum(((b2q(coords[1][k]) - b2q(coords[0][k])) ** 2 for k in range(3)), F(0))
            Lq = inputs["L"]
            # declared length leaf against the coordinates' exact length (ell = 7/2 chain)
            ok = Lq * Lq <= d2 * (1 + rho(F(7, 2))) ** 2 and Lq * Lq * (1 + rho(F(7, 2))) ** 2 >= d2
            case.check("s_length_leaf_matches_coordinates", ok, f"{eid}: length leaf {float(Lq)!r} vs coordinates sqrt({float(d2)!r})")
        # supplementary: shear-area leaf, stiffness-section screen, eigenload area
        mr = member_req.get(eid)
        if not (mr and None not in (mr["D"], mr["t"])):
            case.not_judged("s_stiffness_section_is_nominal_annulus", f"{eid}: D or t does not convert to SI binary64 exactly")
        if mr and None not in (mr["D"], mr["t"]):
            an = annulus(F(mr["D"]), F(mr["t"]))
            rel = F(2) ** bound["supplementary"]["section_leaf_screen_log2_relative"]
            ok = (within_screen(inputs["A"], an["A"], rel) and within_screen(inputs["Iy"], an["I"], rel)
                  and within_screen(inputs["Iz"], an["I"], rel) and within_screen(inputs["J"], an["J"], rel))
            case.check("s_stiffness_section_is_nominal_annulus", ok, f"{eid}: A/I/J are not the nominal annulus of D={mr['D']!r}, t={mr['t']!r}")
            nu = None
            lrs = ns.get("load_reference_state") or {}
            for m in lrs.get("members", []):
                if m.get("pipe_id") == eid and "nu_bits" in m:
                    nu = b2f(m["nu_bits"])
            if nu is None and mr["nu"] is not None:
                nu = float(mr["nu"])
            if frame_theory == "timo":
                if nu is None:
                    case.notes.append(f"{eid}: shear-area rule not judged (no nu)")
                else:
                    kh = kappa_hat(mr["D"], mr["t"], nu)
                    expect = kh * b2f(frame["formation_inputs"]["area_bits"])
                    case.check("s_shear_area_is_u8_kappa_hat_times_area", f2b(expect) == frame["formation_inputs"]["shear_area_bits"],
                               f"{eid}: shear_area {b2f(frame['formation_inputs']['shear_area_bits'])!r} != RN(kappa_hat {kh!r} * A) = {expect!r}")
        for el in (ns.get("load_reference_state") or {}).get("eigen_loads", []):
            if el.get("pipe_id") != eid:
                continue
            m = next((m for m in ns["load_reference_state"]["members"] if m.get("pipe_id") == eid), None)
            if m is None:
                case.check("s_eigenload_member", False, f"{eid}: eigenload without its member record")
                continue
            P = b2q(el["axial_load_bits"])
            exact_P = b2q(m["E_bits"]) * inputs["A"] * b2q(el["eigenstrain_bits"])
            case.check("s_eigenload_on_nominal_area", abs(P - exact_P) <= rho(2) * abs(exact_P),
                       f"{eid}: eigenload {float(P)!r} is not E*A_nominal*eps = {float(exact_P)!r} within rho_2")
            # repair round 1, item 4: the global terms are the declared member axis's components of the local
            # pair (-P at end i, +P at end j): T^T f_local, each component one rounded product P*T[0][k]
            f_local = [F(0)] * 12
            f_local[0], f_local[6] = -P, P
            want = {}
            for j in range(12):
                v = sum((fr["T"][i][j] * f_local[i] for i in range(12) if fr["T"][i][j] != 0), F(0))
                want[fr["scatter"][j]] = want.get(fr["scatter"][j], F(0)) + v
            terms = eig.get(eid, {})
            bad = [d for d in set(want) | set(terms)
                   if not (abs(terms.get(d, F(0)) - want.get(d, F(0))) <= rho(1) * abs(want.get(d, F(0))))]
            case.check("s_eigenload_terms", not bad,
                       f"{eid}: eigenload force terms on DOFs {sorted(bad)} are not T^T(-P, +P) of the declared member axis")


# ----------------------------------------------------------------------------- runs

NO_RECEIPT = "envelope carries no source_block_recovery receipt"


def load_required(path, name):
    """The committed required-selection list: (document, mode, load case) triples that must select and pass."""
    with open(path, encoding="utf-8") as fh:
        doc = json.load(fh)
    if doc.get("schema") != "o4_declared_problem_required_selection_v1" or name not in doc.get("sets", {}):
        raise SystemExit(f"oracle: {path} has no required-selection set {name!r}")
    return [tuple(x) for x in doc["sets"][name]]


def judge_batch(index, load, bound, required, echo=True):
    """Judge every (entry, mode). `load(entry, mode)` returns (dump, envelope, request) or raises OSError.

    The batch fails (repair round 1, item 6) when it is vacuous (no entries or no judged case), when any
    required (document, mode, case) triple did not select and pass, on any FAIL case, and on any error other
    than a recorded non-selection (an envelope with no receipt). Errors accumulate (item 7)."""
    reports = []
    for entry in index:
        for mode in ("sparse_interactive", "dense_scrutiny"):
            rep = {"twin": entry["twin"], "family": entry["family"], "name": entry["name"], "variant": entry["variant"],
                   "theory": entry["theory"], "mode": mode, "cases": [], "unselected": [], "errors": []}
            try:
                dump, env, request = load(entry, mode)
            except (OSError, ValueError) as exc:
                rep["errors"].append(f"missing or unreadable dump or envelope: {exc}")
                reports.append(rep)
                continue
            errors = []
            if dump.get("solver_mode") != mode:
                errors.append(f"dump solver_mode {dump.get('solver_mode')} != {mode}")
            result = judge(dump, env, request, bound, entry["theory"], entry["variant"])
            rep.update(result)
            rep["errors"] = errors + result["errors"]
            reports.append(rep)
            if echo:
                print(f"{entry['twin']:58s} {mode:18s} cases={[c['verdict'] for c in rep['cases']]} "
                      f"unselected={[u['case_id'] for u in rep['unselected']]} errors={rep['errors']}", flush=True)
    summary = summarize(reports)
    batch = []
    if dups([e["twin"] for e in index]):
        batch.append(f"duplicate documents in the index: {dups([e['twin'] for e in index])[:5]}")
    if dups([list(t) for t in required]):
        batch.append(f"duplicate required triples: {dups([list(t) for t in required])[:5]}")
    if not index:
        batch.append("vacuous: the index has no entries")
    if summary["cases"] == 0:
        batch.append("vacuous: no case was judged")
    if not required:
        batch.append("vacuous: the required-selection list is empty")
    passed = {(rep["twin"], rep["mode"], c["case_id"]) for rep in reports for c in rep["cases"] if c["verdict"] == "PASS"}
    missing = [r for r in required if tuple(r) not in passed]
    if missing:
        batch.append(f"{len(missing)} required (document, mode, case) did not select and pass: {missing[:5]}")
    hard = [f for f in summary["failures"] if f.get("errors") != [NO_RECEIPT]]
    if hard:
        batch.append(f"{len(hard)} failing cases or errors")
    summary["not_selected_runs"] = sum(1 for f in summary["failures"] if f.get("errors") == [NO_RECEIPT])
    summary["required"] = {"count": len(required), "missing": len(missing)}
    summary["batch_failures"] = batch
    summary["verdict"] = "FAIL" if batch else "PASS"
    return reports, summary


def run_index(index_path, runs_dir, bound, out_path=None, required=()):
    with open(index_path, encoding="utf-8") as fh:
        index = json.load(fh)

    def load(entry, mode):
        stem = entry["twin"].replace("|", "+")
        with open(entry["request"], encoding="utf-8") as fh:
            request = json.load(fh)
        with open(os.path.join(runs_dir, f"{stem}+{mode}.dump.json"), encoding="utf-8") as fh:
            dump = json.load(fh)
        with open(os.path.join(runs_dir, f"{stem}+{mode}.envelope.json"), encoding="utf-8") as fh:
            env = json.load(fh)
        return dump, env, request
    reports, summary = judge_batch(index, load, bound, list(required))
    if out_path:
        with open(out_path, "w", encoding="utf-8") as fh:
            json.dump({"summary": summary, "reports": reports}, fh, ensure_ascii=False, indent=1, default=str)
    print(json.dumps({k: v for k, v in summary.items() if k != "failures"}, ensure_ascii=False, indent=1, default=str))
    return summary


def run_controls(index_path, runs_dir, bound):
    """Real-data controls: each must FAIL, or the oracle is not discriminating on product outputs.

    1. Cross-pairing: each Timoshenko twin's dump judged against its Euler-Bernoulli twin's
       envelope (same request otherwise) must fail the commitment and projection checks.
    2. Reference control: each declared Timoshenko K_local judged against the Euler-Bernoulli
       reference of the same leaves (A_s ignored) must exceed BOUND.md's bound.
    3. Allowance control: each nonzero-allowance twin's dump judged against its zero-allowance
       twin's envelope must fail (the reduced stress section and the hashes differ).
    """
    with open(index_path, encoding="utf-8") as fh:
        index = json.load(fh)

    def load(twin, mode, kind):
        with open(os.path.join(runs_dir, f"{twin.replace('|', '+')}+{mode}.{kind}.json"), encoding="utf-8") as fh:
            return json.load(fh)

    def load_request(entry):
        with open(entry["request"], encoding="utf-8") as fh:
            return json.load(fh)
    out = control_batch(index, load, load_request, bound)
    print(json.dumps({"controls": out}, indent=1))
    return out


def control_batch(index_list, load, load_request, bound):
    if dups([e["twin"] for e in index_list]):
        raise SystemExit(f"oracle: duplicate documents in the control index: {dups([e['twin'] for e in index_list])[:5]}")
    index = {e["twin"]: e for e in index_list}
    counts = {"cross_theory": [0, 0], "eb_reference_on_timo_k": [0, 0], "cross_allowance": [0, 0]}
    failures = {"cross_theory": set(), "cross_allowance": set()}
    for twin, entry in index.items():
        fam, variant, theory = twin.split("|")
        request = load_request(entry)
        for mode in ("sparse_interactive", "dense_scrutiny"):
            dump = load(twin, mode, "dump")
            if theory == "timo":
                env = load(f"{fam}|{variant}|eb", mode, "envelope")
                rep = judge(dump, env, request, bound, theory, variant)
                failed = any(c["verdict"] == "FAIL" for c in rep["cases"])
                counts["cross_theory"][0 if failed else 1] += 1
                failures["cross_theory"].update(k for c in rep["cases"] for k, v in c["checks"].items() if v["failed"])
                for rec in dump["records"]:
                    ns = json.loads(rec["normalized_source_text"])["payload"]
                    for frame in ns["frames"]:
                        inputs = frame_inputs(frame)
                        eb = dict(inputs)
                        eb["As"] = None
                        Kl = [[b2q(x) for x in row] for row in frame["local_matrix_bits"]]
                        ok = judge_k_local(bound, Kl, eb, "eb", inputs["ell"])[0]
                        counts["eb_reference_on_timo_k"][0 if not ok else 1] += 1
            if variant != "zero":
                env = load(f"{fam}|zero|{theory}", mode, "envelope")
                rep = judge(dump, env, request, bound, theory, variant)
                failed = any(c["verdict"] == "FAIL" for c in rep["cases"])
                counts["cross_allowance"][0 if failed else 1] += 1
                failures["cross_allowance"].update(k for c in rep["cases"] for k, v in c["checks"].items() if v["failed"])
    out = {k: {"failed_as_required": v[0], "passed_wrongly": v[1]} for k, v in counts.items()}
    # repair round 1, item 6: a control set with nothing to control is a failure, not a pass
    out["vacuous"] = [k for k, v in counts.items() if v[0] + v[1] == 0]
    out["checks_that_failed"] = {k: sorted(v) for k, v in failures.items()}
    return out


def summarize(reports):
    by = {}
    worst = {}
    fails = []
    unselected = []
    for rep in reports:
        for c in rep.get("cases", []):
            for key in (("family", rep["family"]), ("request", f"{rep['family']}-{rep['name']}"), ("theory", rep["theory"]),
                        ("variant", rep["variant"]), ("mode", rep["mode"]), ("theory+variant", f"{rep['theory']}+{rep['variant']}"),
                        ("all", "all")):
                slot = by.setdefault(f"{key[0]}={key[1]}", {"PASS": 0, "FAIL": 0})
                slot[c["verdict"]] += 1
            if c["verdict"] == "FAIL":
                fails.append({"twin": rep["twin"], "mode": rep["mode"], "case": c["case_id"], "findings": c["findings"][:20]})
            for name, m in c["metrics"].items():
                cur = worst.get(name)
                v = m["value"]
                if not isinstance(v, (int, float)):
                    continue
                better = (v < cur["value"]) if (cur and ("min_" in name or name.endswith("_min"))) else (cur is None or v > cur["value"])
                if cur is None or better:
                    worst[name] = {"value": v, "where": f"{rep['twin']} {rep['mode']} {c['case_id']}: {m['where']}"}
        for u in rep.get("unselected", []):
            unselected.append({"twin": rep["twin"], "mode": rep["mode"], **u})
        if rep.get("errors"):
            fails.append({"twin": rep["twin"], "mode": rep["mode"], "errors": rep["errors"]})
    return {"counts": by, "worst": worst, "failures": fails, "unselected": unselected,
            "runs": len(reports), "cases": sum(len(r.get("cases", [])) for r in reports)}


# ----------------------------------------------------------------------------- self-test

def synth(theory="timo", allowance="both", kl_hook=None, as_hook=None, rotate=False, eig_on_x=False, mm=False, split_spring=False):
    """A hand-made, internally consistent dump, envelope and request in the product's shapes.

    One member root -> tip (along global x, or along global y with `rotate`), an anchor at the root on
    UX, UY, UZ, RY, RZ (UY prescribed to 1 mm), a soft RX spring support at the root, six nodal loads at the
    tip and an axial eigenload P. The receipt is complete: every published functional has its projection,
    every published row its receipt row and evidence. `eig_on_x` declares the eigenload terms on global X
    whatever the member axis (consistently through the receipt), `mm` authors the request's section in mm."""
    D, t = 0.2, 0.01
    cm = 0.00125 if allowance in ("mill", "both") else 0.0
    ca = 0.0015 if allowance in ("ca", "both") else 0.0
    nu, E, G, L = 0.25, 2.0e11, 8.0e10, 2.0
    an = annulus(F(D), F(t))
    A, I, J, Z = (rn((an[k][0] + an[k][1]) / 2) for k in ("A", "I", "J", "Z"))
    tr = (t - cm) - ca
    ar = annulus(F(D), F(tr))
    Ar, Ir, Jr, Zr = (rn((ar[k][0] + ar[k][1]) / 2) for k in ("A", "I", "J", "Z"))
    As = kappa_hat(D, t, nu) * A if theory == "timo" else None
    if as_hook and As is not None:
        As = as_hook(As, A)
    bound = load_bound()
    inputs = {"E": F(E), "G": F(G), "A": F(A), "Iy": F(I), "Iz": F(I), "J": F(J), "L": F(L), "As": F(As) if As else None}
    Kx, _, _, _ = reference_matrix(bound, inputs, theory)
    Kl = [[rn(x) for x in row] for row in Kx]
    if kl_hook:
        kl_hook(Kl, inputs, bound)
    R = [[0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]] if rotate else [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    T = [[R[i % 3][j % 3] if i // 3 == j // 3 else 0.0 for j in range(12)] for i in range(12)]
    tip = [0.0, L, 0.0] if rotate else [L, 0.0, 0.0]
    nodes = ["root", "tip"]
    dof_map = [{"node_id": nodes[i // 6], "component": COMPONENTS6[i % 6], "displacement_unit": "m" if i % 6 < 3 else "rad",
                "action_unit": "N" if i % 6 < 3 else "N*m"} for i in range(12)]
    k_spring = 1.0e-4
    spring_ids = ["spring", "spring-2"] if split_spring else ["spring"]
    k_each = k_spring / len(spring_ids)  # exact: 1e-4 / 2 is a binary64 halving
    loads = {6: 0.001, 7: 0.002, 8: -0.003, 9: 1e-8, 10: -0.0005, 11: 0.0006}
    P = 1000.0
    f_local = [0.0] * 12
    f_local[0], f_local[6] = -P, P
    if eig_on_x:
        eig_g = {0: -P, 6: P}
    else:
        eig_g = {}
        for j in range(12):
            v = sum(T[i][j] * f_local[i] for i in range(12))
            if v != 0:
                eig_g[j] = v
    anchor = {0: 0.0, 1: 0.001, 2: 0.0, 4: 0.0, 5: 0.0}
    force_terms = [{"dof": d, "owner": {"id": f"load{d}", "kind": "load"}, "value_bits": f2b(v)} for d, v in loads.items()]
    force_terms += [{"dof": d, "owner": {"id": "member", "kind": "member_eigenstrain"}, "value_bits": f2b(v)} for d, v in sorted(eig_g.items())]
    Gq = matmul(transpose([[F(x) for x in r] for r in T]), matmul([[F(x) for x in r] for r in Kl], [[F(x) for x in r] for r in T]))
    stiffness_terms = [{"row": i, "col": j, "owner": {"id": "member", "kind": "frame"}, "value_bits": f2b(float(Gq[i][j]))}
                       for i in range(12) for j in range(12) if Gq[i][j] != 0]
    for sid in spring_ids:
        stiffness_terms.append({"row": 3, "col": 3, "owner": {"id": sid, "kind": "spring"}, "value_bits": f2b(k_each)})
    fi = {"area_bits": f2b(A), "coordinates_bits": [[f2b(0.0)] * 3, [f2b(x) for x in tip]], "iy_bits": f2b(I), "iz_bits": f2b(I),
          "length_bits": f2b(L), "shear_modulus_bits": f2b(G), "torsion_constant_bits": f2b(J), "young_modulus_bits": f2b(E)}
    if As is not None:
        fi["shear_area_bits"] = f2b(As)
    frame = {"element_id": "member", "formation_inputs": fi, "local_matrix_bits": [[f2b(x) for x in r] for r in Kl],
             "transform_bits": [[f2b(x) for x in r] for r in T], "global_matrix_bits": [[f2b(float(x)) for x in r] for r in Gq],
             "node_i_id": "root", "node_j_id": "tip", "scatter": list(range(12)),
             "recovery_section": {"section_modulus_bits": f2b(Z), "torsion_radius_bits": f2b(D / 2)}}
    if tr != t:
        frame["reduced_recovery_section"] = {"area_bits": f2b(Ar), "section_modulus_bits": f2b(Zr), "torsion_constant_bits": f2b(Jr), "torsion_radius_bits": f2b(D / 2)}
    eps = rn(F(P) / (F(E) * F(A)))
    ns = {"case_id": "case", "dof_map": dof_map, "force_terms": force_terms, "frames": [frame], "free_dofs": [3, 6, 7, 8, 9, 10, 11],
          "prescribed": [{"dof": d, "support_id": "anchor", "value_bits": f2b(v)} for d, v in anchor.items()],
          "springs": [{"dof": 3, "ground_movement_bits": f2b(0.0), "node_id": "root", "spring_id": sid,
                       "stiffness_bits": f2b(k_each), "support_id": sid} for sid in spring_ids],
          "stiffness_terms": stiffness_terms,
          "load_reference_state": {"eigen_loads": [{"axial_load_bits": f2b(rn(F(E) * F(A) * F(eps))), "eigenstrain_bits": f2b(eps), "pipe_id": "member"}],
                                   "members": [{"pipe_id": "member", "E_bits": f2b(E), "nu_bits": f2b(nu)}]}}
    # the eigenload record's P is E*A*eps rounded; the declared force terms carry the same P
    P_rec = rn(F(E) * F(A) * F(eps))
    if P_rec != P:
        scale = P_rec / P
        for ft in ns["force_terms"]:
            if ft["owner"]["kind"] == "member_eigenstrain":
                ft["value_bits"] = f2b(b2f(ft["value_bits"]) * scale)
        eig_g = {d: v * scale for d, v in eig_g.items()}
        P = P_rec
    off_local = [-sum(T[i][j] * eig_g.get(j, 0.0) for j in range(12)) for i in range(12)]  # -(T feq_g), exact here

    functions = []

    def kfac(r, p):
        return {"bits": f2b(Kl[r][p]), "kind": "source", "source_path": f"/frames/0/local_matrix_bits/{r}/{p}"}

    def tfac(p, j):
        return {"bits": f2b(T[p][j]), "kind": "source", "source_path": f"/frames/0/transform_bits/{p}/{j}"}

    def const(x):
        return {"bits": f2b(x), "kind": "method_constant"}

    def row_terms(r, mult):
        """Terms of local end-action row r (K_local T u), each product times the constant factors `mult`."""
        out = []
        for j in range(12):
            cts = [{"factors": [kfac(r, p), tfac(p, j)] + [const(m) for m in mult]}
                   for p in range(12) if Kl[r][p] != 0 and T[p][j] != 0]
            if cts:
                out.append({"dof": j, "coefficient_terms": cts})
        return out, ([{"factors": [const(off_local[r])] + [const(m) for m in mult]}] if off_local[r] != 0 else [])

    def add(fn, published):
        fn.setdefault("offset_terms", [])
        fn.setdefault("final_unit_factor", const(1.0))
        fn["functional_id"] = f"source-functional:case:{len(functions)}"
        fn["result_ids"] = [published] if published else []
        functions.append(fn)
        return fn["functional_id"]
    names = list(ACTION_INDEX)
    end_ids = {}
    for end, base in (("end_i", 0), ("end_j", 6)):
        for k, nm in enumerate(names):
            terms, offs = row_terms(base + k, [])
            end_ids[(end, nm)] = add({"quantity": "member_end_action", "component": nm, "location": end, "unit": "N" if k < 3 else "N*m",
                                      "owner": {"id": "member", "kind": "frame"}, "displacement_terms": terms, "offset_terms": offs,
                                      "sign_convention": "node_on_element_local_end", "coordinate_system": "element_local"},
                                     f"result:force:member:{end}:{nm}")
    station_ids = {}
    for loc, s in STATION_FRACTION.items():
        for k, nm in enumerate(names):
            parts = {0: [(0, [-1.0])], 1: [(1, [-1.0])], 2: [(2, [-1.0])], 3: [(3, [-1.0])],
                     4: [(4, [-1.0]), (2, [-1.0, float(s), L])], 5: [(5, [-1.0]), (1, [float(s), L])]}[k]
            terms, offs = [], []
            for r, mult in parts:
                t_, o_ = row_terms(r, mult)
                terms += t_
                offs += o_
            station_ids[(loc, nm)] = add({"quantity": "member_station_action", "component": nm, "location": loc, "unit": "N" if k < 3 else "N*m",
                                         "owner": {"id": "member", "kind": "frame"}, "displacement_terms": terms, "offset_terms": offs,
                                         "sign_convention": "j_side_section_local", "coordinate_system": "element_local"},
                                        f"result:force:member:{loc}:{nm}" if loc in INTERIOR else None)
    nodal_ids = {}
    for node_index, node in enumerate(nodes):
        for k, comp in enumerate(COMPONENTS6):
            d = node_index * 6 + k
            nodal_ids[(node, comp)] = add({"quantity": "nodal_translation" if k < 3 else "nodal_rotation", "component": comp, "location": "node",
                                          "unit": "mm" if k < 3 else "rad", "owner": {"id": node, "kind": "node"},
                                          "displacement_terms": [{"dof": d, "coefficient_terms": [{"factors": [const(1000.0 if k < 3 else 1.0)]}]}],
                                          "sign_convention": "declared_global_nodal_component", "coordinate_system": "global"},
                                         f"result:disp:{node}:{comp.lower()}")
    f_glob = {d: v for d, v in loads.items()}
    for d, v in eig_g.items():
        f_glob[d] = f_glob.get(d, 0.0) + v
    sup_ids = {}
    for comp, cname in SUPPORT_COMPONENTS.items():
        d = COMPONENTS6.index(cname)
        terms, offs = [], []
        if d in anchor:
            for j in range(12):
                cts = [{"factors": [tfac(kk, d), kfac(kk, ll), tfac(ll, j)]}
                       for kk in range(12) for ll in range(12) if T[kk][d] != 0 and Kl[kk][ll] != 0 and T[ll][j] != 0]
                if cts:
                    terms.append({"dof": j, "coefficient_terms": cts})
            if f_glob.get(d, 0.0) != 0:
                offs = [{"factors": [const(-f_glob[d])]}]
        sup_ids[("anchor", comp)] = add({"quantity": "support_action_component", "component": comp, "location": "node",
                                         "unit": "N" if comp[0] == "F" else "N*m", "owner": {"id": "anchor", "kind": "support"},
                                         "displacement_terms": terms, "offset_terms": offs, "coordinate_system": "global",
                                         "sign_convention": "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node"},
                                        f"result:support:anchor:{comp}")
    for sid in spring_ids:
        for comp in SUPPORT_COMPONENTS:
            terms = [{"dof": 3, "coefficient_terms": [{"factors": [const(-k_each)]}]}] if comp == "Mx" else []
            sup_ids[(sid, comp)] = add({"quantity": "support_action_component", "component": comp, "location": "node",
                                        "unit": "N" if comp[0] == "F" else "N*m", "owner": {"id": sid, "kind": "support"},
                                        "displacement_terms": terms, "coordinate_system": "global",
                                        "sign_convention": "support_on_pipe_positive_global_force_right_hand_couple_at_attachment_node"},
                                       f"result:support:{sid}:{comp}")
        add({"quantity": "support_action_component", "component": "Mx", "location": "node", "unit": "N*m", "owner": {"id": sid, "kind": "spring"},
             "displacement_terms": [{"dof": 3, "coefficient_terms": [{"factors": [const(-k_each)]}]}],
             "sign_convention": "spring_on_structure", "coordinate_system": "global"}, None)

    ns_text = json.dumps({"domain": "source_blocks_normalized_source_v1", "payload": ns}, sort_keys=True, separators=(",", ":"))
    ns_hash = hashlib.sha256(ns_text.encode()).hexdigest()
    fp = {"functions": functions, "normalized_source_sha256": ns_hash, "derived_recipes": []}
    fp_text = json.dumps({"domain": "source_blocks_functional_plan_v1", "payload": fp}, sort_keys=True, separators=(",", ":"))
    fp_hash = hashlib.sha256(fp_text.encode()).hexdigest()
    rec = {"case_id": "case", "normalized_source_sha256": ns_hash, "functional_plan_sha256": fp_hash,
           "normalized_source_text": ns_text, "functional_plan_text": fp_text}
    dump = {"artifact": "o4-source-declared-problem-dump", "schema_version": "1.0.0", "solver_mode": "sparse_interactive",
            "publication_sha256": "ab" * 32, "records": [rec]}
    exact = exact_values(ns, fp)
    basis_ref = {"ref_type": "load_case", "ref_id": "case"}
    projections, results, rows = [], [], []

    def publish(rid, value, unit, entity, kind, component=None, location=None, cs=None, section_basis=None):
        row = {"id": rid, "value": value, "unit": unit, "basis_ref": basis_ref, "entity_ref": entity, "kind": kind}
        if component is not None:
            row["metadata"] = {"component": component, "location": location, "coordinate_system": cs,
                               "basis": "synthetic" + (f"; section_basis={section_basis}" if section_basis else "")}
        results.append(row)

    def claims(v, lo, hi):
        """Bound claims in the product's form: the interval within value +- abs, abs within rel*|value|."""
        ab = ru(max(F(v) - F(lo), F(hi) - F(v), F(0)))
        return ab, (ru(F(ab) / abs(F(v))) if v else 0.0)

    def public_of(fn):
        q, comp = fn["quantity"], fn["component"]
        if q in ("nodal_translation", "nodal_rotation"):
            axis = comp[1].lower()
            kind = f"global_nodal_displacement_{axis}" if q == "nodal_translation" else f"global_nodal_rotation_{axis}"
            return (fn["owner"]["id"], kind, kind.replace("global_", ""), "node", "global")
        if q in ("member_end_action", "member_station_action"):
            return (fn["owner"]["id"], f"element_local_{comp}", comp, fn["location"], "element_local")
        return (fn["owner"]["id"], "support_reaction_component_v2", comp, "node", "global")

    def act(fid):
        x = exact[fid]
        return {"value": rn(x), "interval": [rd(x), ru(x)]}
    for fn in functions:
        if not fn["result_ids"]:
            continue
        x = exact[fn["functional_id"]]
        v = rn(x)
        err = abs(F(v) - x)
        basis = "exact_zero" if x == 0 else ("exact_identity" if err == 0 else "outward_interval")
        pid = "projection:" + fn["functional_id"]
        ab, rl = claims(v, rd(x), ru(x))
        projections.append({"projection_id": pid, "functional_id": fn["functional_id"], "result_id": fn["result_ids"][0],
                            "quantity": fn["quantity"], "value": v, "value_bits": f2b(v), "unit": fn["unit"], "interval": [rd(x), ru(x)],
                            "absolute_error_bound": ab, "relative_error_bound": rl, "relative_limit": 1e-9, "basis": basis})
        ent, kind, comp_, loc_, cs_ = public_of(fn)
        publish(fn["result_ids"][0], v, fn["unit"], ent, kind, comp_, loc_, cs_)
        rows.append({"result_id": fn["result_ids"][0], "treatment": "qualified_projection", "projection_id": pid,
                     "recipe_id": None, "input_result_ids": []})
    checks = []
    comp_map = [("axial_normal_stress", "axial_force", "axial-normal"), ("bending_normal_stress_y", "bending_moment_y", "bending-normal-y"),
                ("bending_normal_stress_z", "bending_moment_z", "bending-normal-z"), ("torsional_shear_stress", "torsional_moment", "torsional-shear")]
    index_of = {fn["functional_id"]: i for i, fn in enumerate(functions)}
    for loc in STATION_FRACTION:
        for comp, nm, slug in comp_map:
            fid = station_ids[(loc, nm)]
            a = act(fid)
            for basis, (aa, zz, jj) in (("nominal-section", (A, Z, J)), ("reduced-section", (Ar, Zr, Jr) if tr != t else (A, Z, J))):
                par = {"area_m2": aa, "section_modulus_m3": zz, "torsion_constant_m4": jj, "torsion_radius_m": D / 2, "pa_per_mpa": 1e6}
                rid = f"result:stress:member:{loc}:{slug}:{basis}"
                checks.append({"result_id": rid, "recipe_id": "retained_source_straight_stress_v1", "pipe_id": "member", "location": loc,
                               "component": comp, "functional_index": index_of[fid], "functional_id": fid, "action": a, "parameters": par})
                v = a["value"]
                val = (v / aa) / 1e6 if comp == "axial_normal_stress" else ((v / zz) / 1e6 if comp.startswith("bending") else (v * (D / 2) / jj) / 1e6)
                publish(rid, val, "MPa", "member", f"element_local_{comp}", comp, loc, "element_local", basis.replace("-", "_"))
                rows.append({"result_id": rid, "treatment": "checked_derived", "projection_id": None,
                             "recipe_id": "retained_source_straight_stress_v1", "input_result_ids": []})
    extrema = []
    for basis, (aa, zz) in (("nominal_section", (A, Z)), ("reduced_section", (Ar, Zr) if tr != t else (A, Z))):
        eps_, vals = [], []
        for loc in ("end_i", "end_j"):
            fids = [station_ids[(loc, "axial_force")], station_ids[(loc, "bending_moment_y")], station_ids[(loc, "bending_moment_z")]]
            N, My, Mz = (exact[f_] for f_ in fids)
            hi_ = abs(N) / F(aa) + sqrt_up(My * My + Mz * Mz) / F(zz)
            lo_ = abs(N) / F(aa) + sqrt_down(My * My + Mz * Mz) / F(zz)
            vals.append((lo_, hi_))
            eps_.append({"station_fraction": 0 if loc == "end_i" else 1, "functional_ids": fids, "functional_indices": [index_of[f_] for f_ in fids],
                         "actions": [act(f_) for f_ in fids], "value_pa": rn(hi_), "interval_pa": [rd(lo_), ru(hi_)]})
        lo_m, hi_m = max(v[0] for v in vals), max(v[1] for v in vals)
        rid = f"result:elastic-maximum:member:{basis}"
        ab, rl = claims(rn(hi_m), rd(lo_m), ru(hi_m))
        extrema.append({"basis": "retained_source_endpoint_normal_max_v1", "pipe_id": "member", "section_basis": basis, "result_id": rid,
                        "area_m2": aa, "section_modulus_m3": zz, "value_pa": rn(hi_m), "value_lower_pa": rd(lo_m), "value_upper_pa": ru(hi_m),
                        "absolute_error_bound_pa": ab, "relative_error_bound": rl, "relative_limit": 1e-9,
                        "endpoints": eps_, "load_case_id": "case"})
        publish(rid, rn(hi_m), "Pa", "member", "pipe_elastic_normal_stress_maximum_v2", "maximum_absolute_normal_stress",
                "governing_station", "pipe_section", basis)
        rows.append({"result_id": rid, "treatment": "checked_derived", "projection_id": None,
                     "recipe_id": "retained_source_endpoint_normal_max_v1", "input_result_ids": []})
    derived = []
    for sup in ["anchor"] + spring_ids:
        for recipe, comps, unit in (("support_force_norm_scaled_checked_v1", ("Fx", "Fy", "Fz"), "N"),
                                    ("support_moment_norm_scaled_checked_v1", ("Mx", "My", "Mz"), "N*m")):
            fids = [sup_ids[(sup, c)] for c in comps]
            s2 = sum((exact[f_] ** 2 for f_ in fids), F(0))
            lo_, hi_ = sqrt_down(s2), sqrt_up(s2)
            v = rn(hi_)
            rid = f"result:support:{sup}:{'force' if unit == 'N' else 'moment'}_magnitude"
            ab, rl = claims(v, rd(lo_), ru(hi_))
            derived.append({"result_id": rid, "recipe_id": recipe, "support_id": sup, "functional_ids": fids,
                            "functional_indices": [index_of[f_] for f_ in fids], "values": [rn(exact[f_]) for f_ in fids],
                            "intervals": [[rd(exact[f_]), ru(exact[f_])] for f_ in fids], "value": v, "interval": [rd(lo_), ru(hi_)],
                            "absolute_error_bound": ab, "relative_error_bound": rl, "relative_limit": 1e-9})
            which = "force" if unit == "N" else "moment"
            publish(rid, v, unit, sup, f"support_reaction_{which}_magnitude_v2", f"{which}_magnitude", "node", "global")
            rows.append({"result_id": rid, "treatment": "checked_derived", "projection_id": None, "recipe_id": recipe,
                         "input_result_ids": [f"result:support:{sup}:{c}" for c in comps]})
    for node in nodes:
        comps = [exact[nodal_ids[(node, c)]] for c in ("UX", "UY", "UZ")]
        rid = f"result:disp:{node}"
        publish(rid, rn(sqrt_up(sum((c * c for c in comps), F(0)))), "mm", node, "displacement_magnitude")
        rows.append({"result_id": rid, "treatment": "checked_derived", "projection_id": None, "recipe_id": "translation_norm_scaled_v1",
                     "input_result_ids": [f"result:disp:{node}:{c}" for c in ("ux", "uy", "uz")]})
    for loc in STATION_FRACTION:
        shear_rows = [f"result:force:member:{loc}:{c}" for c in ("shear_force_y", "shear_force_z")]
        for basis in ("nominal-section", "reduced-section"):
            stress_rows = [f"result:stress:member:{loc}:{slug}:{basis}" for slug in ("axial-normal", "bending-normal-y", "bending-normal-z")]
            for extreme in ("sigma_plus", "sigma_minus"):
                rid = f"result:stress:member:{loc}:{extreme}:{basis}"
                publish(rid, 0.0, "MPa", "member", "element_local_normal_stress_extreme", extreme, loc, "pipe_section", basis.replace("-", "_"))
                rows.append({"result_id": rid, "treatment": "derived_not_qualified", "projection_id": None,
                             "recipe_id": "section_extremes_from_published_stress_rows_v1", "input_result_ids": stress_rows})
            rid = f"result:stress:member:{loc}:tau:{basis}"
            publish(rid, 0.0, "MPa", "member", "element_local_transverse_shear_stress_maximum", "tau_v_max", loc, "pipe_section", basis.replace("-", "_"))
            rows.append({"result_id": rid, "treatment": "derived_not_qualified", "projection_id": None,
                         "recipe_id": "transverse_shear_from_published_shear_rows_v1", "input_result_ids": shear_rows})
    endpoint_sections = [{"pipe_id": "member", "endpoints": [dict((k, v) for k, v in ep.items() if k in ("station_fraction", "functional_ids", "functional_indices", "actions"))
                                                         for ep in extrema[0]["endpoints"]]}]
    section_functionals = [{"pipe_id": "member", "stations": [
        {"station_fraction": float(s) if s not in (0, 1) else int(s), "functional_ids": [station_ids[(loc, nm)] for nm in names],
         "functional_indices": [index_of[station_ids[(loc, nm)]] for nm in names], "actions": [act(station_ids[(loc, nm)]) for nm in names]}
        for loc, s in STATION_FRACTION.items()]}]
    case = {"basis_ref": basis_ref, "selected_method": "retained_source_blocks_exact_v1",
            "source": {"normalized_source_sha256": ns_hash, "functional_plan_sha256": fp_hash, "free_dofs": ns["free_dofs"],
                       "prescribed_dofs": sorted(anchor), "endpoint_sections": endpoint_sections, "section_functionals": section_functionals},
            "projections": projections, "rows": rows, "section_stress_checks": checks, "derived_checks": derived}
    envelope = {"results": results, "source_block_recovery": {"body": {"publication_sha256": "ab" * 32, "cases": [case]}},
                "contract_evidence": {"exact_cases": [{"load_case_id": "case", "pipe_stress_extrema": extrema}]}}
    k_ = 1000.0 if mm else 1.0
    unit = "mm" if mm else "m"
    request = {"model": {"pipe_segments": [{"id": "member", "material": "material", "section": {
        "outside_diameter": {"unit": unit, "value": D * k_}, "wall_thickness": {"unit": unit, "value": t * k_},
        "mill_tolerance": {"unit": unit, "value": cm * k_}, "corrosion_allowance": {"unit": unit, "value": ca * k_}}}],
        "materials": [{"id": "material", "poisson_ratio": {"unit": "1", "value": nu}}]}}
    return dump, envelope, request


def exact_values(ns, fp):
    """The self-test's receipt builder uses the same exact solve and statics the judge uses."""
    return solve_declared(ns, fp)["exact"]


def selftest():
    bound = load_bound()
    outcomes = []

    def record(label, good, detail):
        outcomes.append((label, good))
        print(f"selftest {'ok  ' if good else 'BAD '} {label}: {detail}")

    def expect(label, dump, env, req, theory, want_pass, want_check=None):
        rep = judge(dump, env, req, bound, theory)
        cases = rep["cases"]
        ok_case = bool(cases) and all(c["verdict"] == "PASS" for c in cases) and not rep["errors"]
        failed = sorted({k for c in cases for k, v in c["checks"].items() if v["failed"]})
        good = ok_case if want_pass else (not ok_case and (want_check is None or want_check in failed))
        record(label, good, ("PASS" if ok_case else "FAIL") + ("" if want_pass else f" via {failed}") + (f" errors {rep['errors']}" if rep["errors"] else ""))
        return rep

    for theory in ("timo", "eb"):
        for allowance in ("both", "zero"):
            expect(f"passing synthetic {theory}/{allowance}", *synth(theory, allowance), theory, True)
    expect("passing synthetic, member along global y (rotated eigenload)", *synth("timo", "both", rotate=True), "timo", True)
    rep = expect("passing synthetic, section authored in mm (screens not judged, no error)", *synth("timo", "both", mm=True), "timo", True)
    nj = rep["cases"][0].get("not_judged", {}) if rep["cases"] else {}
    record("mm section: supplementary screens reported not judged", bool(nj.get("c_stress_parameters_screen_basis_annulus")), f"not_judged {nj}")

    def env_case(e):
        return e["source_block_recovery"]["body"]["cases"][0]

    def row_of(e, rid):
        return next(x for x in e["results"] if x["id"] == rid)

    def mutated(fn, theory="timo", allowance="both", rehash=True, **kw):
        d, e, r = synth(theory, allowance, **kw)
        rec = d["records"][0]
        ns = json.loads(rec["normalized_source_text"])
        fp = json.loads(rec["functional_plan_text"])
        fn(ns, fp, e, r, d)
        if rehash:
            rec["normalized_source_text"] = json.dumps(ns, sort_keys=True, separators=(",", ":"))
            h = hashlib.sha256(rec["normalized_source_text"].encode()).hexdigest()
            fp["payload"]["normalized_source_sha256"] = h
            rec["functional_plan_text"] = json.dumps(fp, sort_keys=True, separators=(",", ":"))
            h2 = hashlib.sha256(rec["functional_plan_text"].encode()).hexdigest()
            rec["normalized_source_sha256"], rec["functional_plan_sha256"] = h, h2
            src = env_case(e)["source"]
            src["normalized_source_sha256"], src["functional_plan_sha256"] = h, h2
        return d, e, r

    # (a) commitments
    d, e, r = synth()
    d["records"][0]["normalized_source_text"] = d["records"][0]["normalized_source_text"].replace('"case_id":"case"', '"case_id":"case" ', 1)
    expect("(a) payload text altered after commitment", d, e, r, "timo", False, "a_record_hashes")
    d, e, r = synth()
    env_case(e)["source"]["functional_plan_sha256"] = "00" * 32
    expect("(a) receipt plan hash differs", d, e, r, "timo", False, "a_receipt_hashes")

    # (b) projections and the declared problem
    def shift_projection(ns, fp, e, r, d):
        p = next(p for p in env_case(e)["projections"] if p["basis"] == "outward_interval")
        p["interval"] = [math.nextafter(p["interval"][1], math.inf), math.nextafter(math.nextafter(p["interval"][1], math.inf), math.inf)]
    expect("(b) projection interval excludes the exact value", *mutated(shift_projection, rehash=False), "timo", False, "b_projection_contains_exact")

    def change_load(ns, fp, e, r, d):
        ns["payload"]["force_terms"][1]["value_bits"] = f2b(0.0025)
    expect("(b) declared load changed under the same receipt", *mutated(change_load), "timo", False, "b_projection_contains_exact")

    def flip_plan(ns, fp, e, r, d):
        fn = next(f for f in fp["payload"]["functions"] if f["quantity"] == "member_station_action" and f["component"] == "bending_moment_z" and f["location"] == "midspan")
        for t_ in fn["displacement_terms"]:
            for ct in t_["coefficient_terms"]:
                if len(ct["factors"]) > 3:
                    ct["factors"][2]["bits"] = f2b(-1.0)
    expect("(b) plan lowering differs from statics", *mutated(flip_plan), "timo", False, "b_plan_lowering_equals_statics")

    # repair round 1, item 1 (Codex probes): missing evidence and rows
    def missing_projections(ns, fp, e, r, d):
        rid = env_case(e)["projections"][0]["result_id"]
        env_case(e)["projections"] = []
        row_of(e, rid)["value"] = 1234567.0
    expect("(r1) projections removed and a projected row changed to 1234567", *mutated(missing_projections, rehash=False), "timo", False, "i_projection_inventory")

    def missing_stress(ns, fp, e, r, d):
        rid = env_case(e)["section_stress_checks"][0]["result_id"]
        env_case(e)["section_stress_checks"] = []
        row_of(e, rid)["value"] = 1234567.0
    expect("(r1) stress checks removed and a stress row changed to 1234567", *mutated(missing_stress, rehash=False), "timo", False, "i_stress_inventory")

    def missing_maximum(ns, fp, e, r, d):
        rid = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["result_id"]
        e["contract_evidence"]["exact_cases"] = []
        row_of(e, rid)["value"] = 1234567.0
    expect("(r1) maximum evidence removed and a maximum row changed to 1234567", *mutated(missing_maximum, rehash=False), "timo", False, "i_maximum_inventory")

    def missing_row(ns, fp, e, r, d):
        rid = env_case(e)["projections"][0]["result_id"]
        e["results"] = [x for x in e["results"] if x["id"] != rid]
    expect("(r1) a projected row removed from the publication", *mutated(missing_row, rehash=False), "timo", False, "i_rows_are_published_universe")

    def empty_norm(ns, fp, e, r, d):
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        for key in ("functional_ids", "functional_indices", "intervals", "values"):
            dc[key] = []
        dc["interval"], dc["value"] = [0.0, 0.0], 0.0
        row_of(e, dc["result_id"])["value"] = 0.0
    expect("(r1) empty component lists certify a zero support norm", *mutated(empty_norm, rehash=False), "timo", False, "b_evidence_list_lengths")

    # item 2: a stress check bound to another action
    def bending_z_uses_axial(ns, fp, e, r, d):
        cks = env_case(e)["section_stress_checks"]
        src = cks[0]
        for sc in cks:
            if sc["component"] != "bending_normal_stress_z":
                continue
            for key in ("functional_id", "functional_index", "action"):
                sc[key] = copy.deepcopy(src[key])
            row_of(e, sc["result_id"])["value"] = (sc["action"]["value"] / sc["parameters"]["section_modulus_m3"]) / 1e6
    expect("(r2) bending-z checks bound to the axial-force functional, publishing N/Z", *mutated(bending_z_uses_axial, rehash=False), "timo", False, "c_stress_bound_to_its_action")

    # item 3: the published support-norm value
    def wrong_norm_value(ns, fp, e, r, d):
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        dc["value"] = 1234567.0
        row_of(e, dc["result_id"])["value"] = 1234567.0
    expect("(r3) support norm value and row changed to 1234567", *mutated(wrong_norm_value, rehash=False), "timo", False, "b_derived_norm_value")

    def norm_row_differs(ns, fp, e, r, d):
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        row_of(e, dc["result_id"])["value"] = dc["value"] * 2
    expect("(r3) support norm published row differs from its check", *mutated(norm_row_differs, rehash=False), "timo", False, "b_derived_norm_published_row")

    # item 4: the eigenload terms follow the member axis
    expect("(r4) member along y with the eigenload declared on global X", *synth("timo", "both", rotate=True, eig_on_x=True), "timo", False, "s_eigenload_terms")

    # (c) stress checks and maxima
    def nominal_on_reduced(ns, fp, e, r, d):
        cks = env_case(e)["section_stress_checks"]
        cks[1]["parameters"] = dict(cks[0]["parameters"])
    expect("(c) reduced check carries nominal parameters", *mutated(nominal_on_reduced, rehash=False), "timo", False, "c_stress_parameters_are_basis_leaves")

    def bad_stress(ns, fp, e, r, d):
        sc = next(c for c in env_case(e)["section_stress_checks"] if c["action"]["value"] != 0)
        row = row_of(e, sc["result_id"])
        row["value"] = row["value"] * (1 + 1e-12)
    expect("(c) stress row does not recompute", *mutated(bad_stress, rehash=False), "timo", False, "c_stress_recomputes")

    def low_max(ns, fp, e, r, d):
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][1]
        mx["value_upper_pa"] = mx["value_lower_pa"] = mx["value_pa"] = mx["value_lower_pa"] * (1 - 1e-9)
    expect("(c) maximum interval excludes the exact maximum", *mutated(low_max, rehash=False), "timo", False, "c_maximum_contains_exact")

    def drop_reduced(ns, fp, e, r, d):
        del ns["payload"]["frames"][0]["reduced_recovery_section"]
    expect("(c) reduced section missing under nonzero allowance", *mutated(drop_reduced), "timo", False, "c_stress_parameters_are_basis_leaves")

    # (d) K_local formed wrongly but consistently: only the bound can see it
    def wrong_recipe(Kl, inputs, bnd):
        E, I, L, G, As = inputs["E"], inputs["Iz"], inputs["L"], inputs["G"], inputs["As"]
        phi = 12 * E * I / (G * As * L * L)
        Kl[5][5] = Kl[11][11] = rn((4 + 2 * phi) * E * I / (L * (1 + phi)))
    expect("(d) k_thth formed as (4 + 2 phi), consistently", *synth("timo", "both", kl_hook=wrong_recipe), "timo", False, "d_k_local_within_bound")

    def beyond(Kl, inputs, bnd):
        K, _, _, _ = reference_matrix(bnd, inputs, "timo")
        Kl[1][1] = ru(K[1][1] * (1 + 2 * rho(F(bnd["roles"]["timoshenko"]["k_vv"]["constant"]))))
    expect("(d) k_vv beyond rho_13 (2x the bound), consistently", *synth("timo", "both", kl_hook=beyond), "timo", False, "d_k_local_within_bound")

    def inside(Kl, inputs, bnd):
        K, _, _, _ = reference_matrix(bnd, inputs, "timo")
        Kl[1][1] = rd(K[1][1] * (1 + rho(F(bnd["roles"]["timoshenko"]["k_vv"]["constant"])) / 2))
    expect("(d) k_vv inside rho_13 (half the bound), consistently", *synth("timo", "both", kl_hook=inside), "timo", True)

    def stray(Kl, inputs, bnd):
        Kl[0][1] = 1e-300
    expect("(d) structural zero declared nonzero, consistently", *synth("timo", "both", kl_hook=stray), "timo", False, "d_k_local_within_bound")

    def eb_in_timo(Kl, inputs, bnd):
        eb = dict(inputs, As=None)
        K, _, _, _ = reference_matrix(bnd, eb, "eb")
        for i in range(12):
            for j in range(12):
                Kl[i][j] = rn(K[i][j])
    expect("(d) Euler-Bernoulli matrix on a Timoshenko frame, consistently", *synth("timo", "both", kl_hook=eb_in_timo), "timo", False, "d_k_local_within_bound")

    def metal_area(Kl, inputs, bnd):
        K, _, _, _ = reference_matrix(bnd, dict(inputs, As=inputs["A"]), "timo")
        for i in range(12):
            for j in range(12):
                Kl[i][j] = rn(K[i][j])
    expect("(d) phi formed with the metal area A for A_s, consistently (Codex numerical probe)", *synth("timo", "both", kl_hook=metal_area), "timo", False, "d_k_local_within_bound")

    def swapped_plane_sign(Kl, inputs, bnd):
        Kl[2][4] = -Kl[2][4]
        Kl[4][2] = -Kl[4][2]
    expect("(d) x-z plane coupling sign flipped, consistently", *synth("eb", "zero", kl_hook=swapped_plane_sign), "eb", False, "d_k_local_within_bound")

    expect("(s) shear area from 5/6, consistently", *synth("timo", "both", as_hook=lambda As, A: A * (5.0 / 6.0)), "timo", False, "s_shear_area_is_u8_kappa_hat_times_area")
    expect("(d) frame theory differs from the request", *synth("timo", "zero"), "eb", False, "d_frame_theory_matches_request")

    # repair round 2: Codex's eight evasions and our own adversarial pass (swap, duplicate, relabel,
    # sign-flip, bound-inflate). Each must fail at its intended check.
    def row_id(e, pred):
        return next(x["id"] for x in e["results"] if pred(x))

    def ident(x):
        return public_identity(x)

    def swap_stress_yz(ns, fp, e, r, d):  # Codex E1
        for basis in ("nominal-section", "reduced-section"):
            cks = [c for c in env_case(e)["section_stress_checks"] if c["location"] == "end_i" and c["result_id"].endswith(":" + basis)]
            a = next(c for c in cks if c["component"] == "bending_normal_stress_y")
            z = next(c for c in cks if c["component"] == "bending_normal_stress_z")
            ar, zr = row_of(e, a["result_id"]), row_of(e, z["result_id"])
            ar["value"], zr["value"] = zr["value"], ar["value"]
            a["result_id"], z["result_id"] = z["result_id"], a["result_id"]
    expect("(r2-E1) stress y/z values swapped with their evidence ids", *mutated(swap_stress_yz, rehash=False), "timo", False, "p_stress_bound_to_row")

    def swap_maxima(ns, fp, e, r, d):  # Codex E2
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]
        a, z = mx[0], mx[1]
        ar, zr = row_of(e, a["result_id"]), row_of(e, z["result_id"])
        ar["value"], zr["value"] = zr["value"], ar["value"]
        a["result_id"], z["result_id"] = z["result_id"], a["result_id"]
    expect("(r2-E2) nominal/reduced maximum rows swapped with their evidence ids", *mutated(swap_maxima, rehash=False), "timo", False, "p_maximum_bound_to_row")

    def swap_support_norms(ns, fp, e, r, d):  # Codex E3
        cks = [c for c in env_case(e)["derived_checks"] if c["recipe_id"] == "support_force_norm_scaled_checked_v1"]
        a, z = cks[0], cks[-1]
        ar, zr = row_of(e, a["result_id"]), row_of(e, z["result_id"])
        ar["value"], zr["value"] = zr["value"], ar["value"]
        a["result_id"], z["result_id"] = z["result_id"], a["result_id"]
    expect("(r2-E3) support force-norm rows swapped across supports (anchor's norm published as zero)", *mutated(swap_support_norms, rehash=False), "timo", False, "p_support_norm_bound_to_row")

    def negative_norm(ns, fp, e, r, d):  # Codex E4
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        v = dc["value"]
        dc["interval"], dc["value"] = [-2 * v, 2 * v], -2 * v
        row_of(e, dc["result_id"])["value"] = dc["value"]
    expect("(r2-E4) negative support norm with its small original bound", *mutated(negative_norm, rehash=False), "timo", False, "b_derived_norm_value")

    def inflated_norm(ns, fp, e, r, d):  # Codex E5
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        dc["value"] = dc["absolute_error_bound"] = 1234567.0
        dc["interval"] = [0.0, 1234567.0]
        row_of(e, dc["result_id"])["value"] = 1234567.0
    expect("(r2-E5) support norm 1234567 with an inflated absolute bound and a tiny relative claim", *mutated(inflated_norm, rehash=False), "timo", False, "b_derived_norm_relative_claim")

    def duplicate_row(ns, fp, e, r, d):  # Codex E6
        bad = copy.deepcopy(e["results"][0])
        bad["value"] = 1234567.0
        e["results"].insert(0, bad)
    expect("(r2-E6) a duplicate published row with a wrong value placed first", *mutated(duplicate_row, rehash=False), "timo", False, "i_unique_published_rows")

    def endpoint_twice(ns, fp, e, r, d):  # Codex E7
        es = env_case(e)["source"]["endpoint_sections"][0]
        es["endpoints"] = [copy.deepcopy(es["endpoints"][0]) for _ in range(2)]
    expect("(r2-E7) endpoint section lists end i twice", *mutated(endpoint_twice, rehash=False), "timo", False, "i_endpoint_sections_inventory")

    def translation_same_input(ns, fp, e, r, d):  # Codex E8
        cr = next(x for x in env_case(e)["rows"] if x["recipe_id"] == "translation_norm_scaled_v1")
        cr["input_result_ids"] = [cr["input_result_ids"][0]] * 3
    expect("(r2-E8) translation norm inputs [UX, UX, UX]", *mutated(translation_same_input, rehash=False), "timo", False, "p_translation_inputs_are_the_node_translations")

    def swap_projections(ns, fp, e, r, d):  # A1 swap
        ps = env_case(e)["projections"]
        a = next(p for p in ps if p["result_id"] == "result:force:member:end_i:shear_force_y")
        z = next(p for p in ps if p["result_id"] == "result:force:member:end_i:shear_force_z")
        for key in ("functional_id", "value", "value_bits", "interval", "absolute_error_bound", "relative_error_bound", "basis"):
            a[key], z[key] = z[key], a[key]
        row_of(e, a["result_id"])["value"], row_of(e, z["result_id"])["value"] = a["value"], z["value"]
    expect("(r2-A1) two projections' functionals and values swapped under their rows", *mutated(swap_projections, rehash=False), "timo", False, "p_projection_bound_to_row")

    def relabel_location(ns, fp, e, r, d):  # A2 relabel
        row = row_of(e, "result:stress:member:end_i:bending-normal-y:nominal-section")
        row["metadata"]["location"] = "quarter_1"
    expect("(r2-A2) a stress row relabelled to another station", *mutated(relabel_location, rehash=False), "timo", False, "p_public_identity_map")

    def relabel_unit(ns, fp, e, r, d):  # A19 relabel
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]
        row_of(e, mx["result_id"])["unit"] = "MPa"
    expect("(r2-A3) a maximum row relabelled to MPa", *mutated(relabel_unit, rehash=False), "timo", False, "p_public_identity_map")

    def inflate_projection(ns, fp, e, r, d):  # A4 bound-inflate
        p = next(p for p in env_case(e)["projections"] if p["value"] != 0)
        p["absolute_error_bound"] = 1.0
        p["interval"] = [p["value"] - 1.0, p["value"] + 1.0]
    expect("(r2-A4) projection interval and absolute bound inflated, relative claim kept", *mutated(inflate_projection, rehash=False), "timo", False, "b_projection_claims_consistent")

    def wrong_projection_consistent(ns, fp, e, r, d):  # A5 value off 1e-6 with inflated, self-consistent claims
        p = next(p for p in env_case(e)["projections"] if p["value"] != 0 and p["result_id"].startswith("result:force"))
        v = p["value"] * (1 + 1e-6)
        ab = abs(v) * 2e-6
        p.update({"value": v, "value_bits": f2b(v), "absolute_error_bound": ab, "relative_error_bound": 2e-6, "interval": [v - ab, v + ab]})
        row_of(e, p["result_id"])["value"] = v
    expect("(r2-A5) projection 1e-6 off with self-consistent claims above the 1e-9 limit", *mutated(wrong_projection_consistent, rehash=False), "timo", False, "b_projection_claims_consistent")

    def negative_bound(ns, fp, e, r, d):  # A7 sign-flip of a bound
        p = next(p for p in env_case(e)["projections"] if p["basis"] == "exact_identity" and p["value"] != 0)
        p["absolute_error_bound"] = -1.0
    expect("(r2-A6) a negative projection absolute bound", *mutated(negative_bound, rehash=False), "timo", False, "b_projection_claims_consistent")

    def flip_projection(ns, fp, e, r, d):  # A8 sign-flip of a value
        p = next(p for p in env_case(e)["projections"] if p["value"] != 0)
        p["value"], p["interval"] = -p["value"], [-p["interval"][1], -p["interval"][0]]
        p["value_bits"] = f2b(p["value"])
        row_of(e, p["result_id"])["value"] = p["value"]
    expect("(r2-A7) a projection value, interval and row sign-flipped together", *mutated(flip_projection, rehash=False), "timo", False, "b_projection_contains_exact")

    def wrong_action(ns, fp, e, r, d):  # A9 action off 1e-6 with a widened interval, stress recomputed from it
        for sc in env_case(e)["section_stress_checks"]:
            if sc["component"] == "bending_normal_stress_y" and sc["location"] == "end_i":
                a = sc["action"]["value"] * (1 + 1e-6)
                sc["action"] = {"value": a, "interval": [min(a, sc["action"]["interval"][0]) * 1.01 if a < 0 else min(a, sc["action"]["interval"][0]) * 0.99,
                                                         max(a, sc["action"]["interval"][1]) * 0.99 if a < 0 else max(a, sc["action"]["interval"][1]) * 1.01]}
                row_of(e, sc["result_id"])["value"] = (a / sc["parameters"]["section_modulus_m3"]) / 1e6
    expect("(r2-A8) stress action 1e-6 off inside a widened interval, row recomputed from it", *mutated(wrong_action, rehash=False), "timo", False, "b_action_accuracy")

    def negative_maximum(ns, fp, e, r, d):  # A10 sign-flip / bound-inflate a maximum
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]
        mx["value_pa"] = mx["value_lower_pa"] = -mx["value_pa"]
        row_of(e, mx["result_id"])["value"] = mx["value_pa"]
    expect("(r2-A9) a negative maximum", *mutated(negative_maximum, rehash=False), "timo", False, "c_maximum_claims_consistent")

    def inflated_maximum(ns, fp, e, r, d):
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]
        mx["value_pa"] = mx["value_upper_pa"] = mx["absolute_error_bound_pa"] = 1234567.0
        mx["value_lower_pa"] = 0.0
        row_of(e, mx["result_id"])["value"] = 1234567.0
    expect("(r2-A10) maximum 1234567 with an inflated absolute bound and a tiny relative claim", *mutated(inflated_maximum, rehash=False), "timo", False, "c_maximum_relative_claim")

    def maximum_endpoint_twice(ns, fp, e, r, d):  # A11 duplicate
        mx = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]
        mx["endpoints"] = [copy.deepcopy(mx["endpoints"][0]) for _ in range(2)]
    expect("(r2-A11) maximum evidence lists end i twice", *mutated(maximum_endpoint_twice, rehash=False), "timo", False, "b_evidence_list_lengths")

    def norm_component_copied(ns, fp, e, r, d):  # A12 relabel a component value
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        dc["values"][0] = dc["values"][0] * 2 if dc["values"][0] else 1.0
        dc["intervals"][0] = [min(dc["intervals"][0][0], dc["values"][0]) - 1, max(dc["intervals"][0][1], dc["values"][0]) + 1]
    expect("(r2-A12) a support norm component value not its projection (interval widened)", *mutated(norm_component_copied, rehash=False), "timo", False, "b_derived_component_is_projection")

    def duplicate_receipt_row(ns, fp, e, r, d):  # A13 duplicate
        env_case(e)["rows"].append(copy.deepcopy(env_case(e)["rows"][0]))
    expect("(r2-A13) a duplicated receipt row", *mutated(duplicate_receipt_row, rehash=False), "timo", False, "i_unique_receipt_rows")

    def duplicate_stress_check(ns, fp, e, r, d):  # A14 duplicate, second wrong
        bad = copy.deepcopy(env_case(e)["section_stress_checks"][0])
        bad["action"]["value"] = bad["action"]["value"] * 2 + 1
        env_case(e)["section_stress_checks"].append(bad)
    expect("(r2-A14) a duplicated stress check with a wrong action", *mutated(duplicate_stress_check, rehash=False), "timo", False, "i_unique_stress_checks")

    def dnq_inputs_relabelled(ns, fp, e, r, d):  # A15 relabel inputs
        cr = next(x for x in env_case(e)["rows"] if x["recipe_id"] == "section_extremes_from_published_stress_rows_v1")
        cr["input_result_ids"] = [i.replace("end_i", "end_j") for i in cr["input_result_ids"]]
    expect("(r2-A15) a derived row's inputs pointed at another station", *mutated(dnq_inputs_relabelled, rehash=False), "timo", False, "p_derived_row_inputs")

    def translation_value(ns, fp, e, r, d):  # A16
        rid = next(x["result_id"] for x in env_case(e)["rows"] if x["recipe_id"] == "translation_norm_scaled_v1" and row_of(e, x["result_id"])["value"] != 0)
        row_of(e, rid)["value"] *= 1.001
    expect("(r2-A16) a translation norm value 1e-3 off", *mutated(translation_value, rehash=False), "timo", False, "b_translation_norm")

    def station_twice(ns, fp, e, r, d):  # A17 duplicate
        sf = env_case(e)["source"]["section_functionals"][0]
        sf["stations"][2] = copy.deepcopy(sf["stations"][1])
    expect("(r2-A17) a section-functional station listed twice", *mutated(station_twice, rehash=False), "timo", False, "i_section_functionals_inventory")

    def projection_row_mismatch(ns, fp, e, r, d):  # A18 relabel a projection's row
        ps = env_case(e)["projections"]
        ps[0]["result_id"] = ps[1]["result_id"]
    expect("(r2-A18) a projection's result_id pointed at another row", *mutated(projection_row_mismatch, rehash=False), "timo", False, "p_projection_bound_to_row")

    def member_relabel(ns, fp, e, r, d):  # A19 relabel entity
        row_of(e, "result:force:member:end_i:axial_force")["entity_ref"] = "ghost"
    expect("(r2-A19) a row relabelled to an undeclared member", *mutated(member_relabel, rehash=False), "timo", False, "p_public_identity_map")

    def norm_negative_bound(ns, fp, e, r, d):  # A20 sign-flip a norm bound
        dc = next(c for c in env_case(e)["derived_checks"] if c["value"] != 0)
        dc["absolute_error_bound"] = -dc["absolute_error_bound"] - 1e-30
    expect("(r2-A20) a negative support-norm absolute bound", *mutated(norm_negative_bound, rehash=False), "timo", False, "b_derived_norm_claims_consistent")

    # repair round 3: functional units bound to the public unit; duplicates rejected at every keyed level
    def coherent_metre_lowering(ns, fp, e, r, d):  # Codex P1: tip translations lowered in m, public rows in mm
        for fn in fp["payload"]["functions"]:
            if fn["quantity"] != "nodal_translation" or fn["owner"]["id"] != "tip":
                continue
            fn["unit"] = "m"
            for t_ in fn["displacement_terms"]:
                for ct in t_["coefficient_terms"]:
                    for fac in ct["factors"]:
                        fac["bits"] = f2b(1.0)
            p = next(p for p in env_case(e)["projections"] if p["functional_id"] == fn["functional_id"])
            v = p["value"] / 1000.0
            ab = abs(v) * 1e-12
            p.update({"value": v, "value_bits": f2b(v), "basis": "outward_interval" if v else "exact_zero",
                      "absolute_error_bound": ab, "relative_error_bound": 1e-12 if v else 0.0, "interval": [v - 0.9 * ab, v + 0.9 * ab]})
            row_of(e, p["result_id"])["value"] = v
        row_of(e, "result:disp:tip")["value"] /= 1000.0
    expect("(r3-U1) tip translations lowered coherently in m under mm rows (Codex)", *mutated(coherent_metre_lowering), "timo", False, "b_functional_unit_is_public_unit")

    def force_unit_mm(ns, fp, e, r, d):  # Codex: a force functional labelled mm with a factor 1000
        fn = next(f for f in fp["payload"]["functions"] if f["quantity"] == "member_end_action" and f["component"] == "shear_force_y")
        fn["unit"], fn["final_unit_factor"]["bits"] = "mm", f2b(1000.0)
    expect("(r3-U2) a force functional labelled mm with a factor 1000 (Codex)", *mutated(force_unit_mm), "timo", False, "b_functional_unit_is_public_unit")

    def moment_unit_nmm(ns, fp, e, r, d):  # variant: coherent N*mm lowering of a moment under an N*m row
        fn = next(f for f in fp["payload"]["functions"] if f["quantity"] == "member_end_action" and f["component"] == "bending_moment_z" and f["location"] == "end_j")
        fn["unit"], fn["final_unit_factor"]["bits"] = "N*mm", f2b(1000.0)
    expect("(r3-U3) a moment functional lowered in N*mm under an N*m row", *mutated(moment_unit_nmm), "timo", False, "b_functional_unit_is_public_unit")

    def rotation_unit_deg(ns, fp, e, r, d):  # variant: a rotation functional labelled deg
        fn = next(f for f in fp["payload"]["functions"] if f["quantity"] == "nodal_rotation")
        fn["unit"] = "deg"
    expect("(r3-U4) a rotation functional labelled deg", *mutated(rotation_unit_deg), "timo", False, "b_functional_unit_is_public_unit")

    def projection_unit_m(ns, fp, e, r, d):  # variant: the projection's unit differs from its functional's public unit
        p = next(p for p in env_case(e)["projections"] if p["unit"] == "mm")
        p["unit"] = "m"
    expect("(r3-U5) a translation projection labelled m", *mutated(projection_unit_m, rehash=False), "timo", False, "b_projection_unit_is_public_unit")

    def declared_units_mm(ns, fp, e, r, d):  # variant: the declared problem's DOF units not SI
        for dm in ns["payload"]["dof_map"]:
            if dm["component"].startswith("U"):
                dm["displacement_unit"] = "mm"
    expect("(r3-U6) declared translation DOFs in mm", *mutated(declared_units_mm), "timo", False, "b_declared_units_si")

    def dup_receipt_case(ns, fp, e, r, d):  # Codex P2: an earlier duplicate receipt case with a bad interval
        bad = copy.deepcopy(env_case(e))
        p = next(p for p in bad["projections"] if p["value"] != 0)
        p["interval"] = [1234567.0, 1234567.0]
        e["source_block_recovery"]["body"]["cases"].insert(0, bad)
    expect("(r3-D1) a duplicate receipt case with a bad interval placed first (Codex)", *mutated(dup_receipt_case, rehash=False), "timo", False, "i_unique_cases")

    def dup_evidence_case(ns, fp, e, r, d):  # Codex P2: an earlier duplicate contract-evidence case with a bad maximum
        bad = copy.deepcopy(e["contract_evidence"]["exact_cases"][0])
        bad["pipe_stress_extrema"][0]["value_lower_pa"] = 1234567.0
        e["contract_evidence"]["exact_cases"].insert(0, bad)
    expect("(r3-D2) a duplicate contract-evidence case with a bad maximum placed first (Codex)", *mutated(dup_evidence_case, rehash=False), "timo", False, "i_unique_cases")

    def dup_record(ns, fp, e, r, d):
        d["records"].append(copy.deepcopy(d["records"][0]))
    expect("(r3-D3) a duplicate dump record", *mutated(dup_record, rehash=False), "timo", False, "i_unique_cases")
    for label, fn_, check in (
            ("(r3-D4) a duplicate declared DOF", lambda ns, fp, e, r, d: ns["payload"]["dof_map"].__setitem__(1, copy.deepcopy(ns["payload"]["dof_map"][0])), "i_unique_declared_dof_map"),
            ("(r3-D5) a duplicate declared frame", lambda ns, fp, e, r, d: ns["payload"]["frames"].append(copy.deepcopy(ns["payload"]["frames"][0])), "i_unique_declared_frames"),
            ("(r3-D6) a duplicate declared spring", lambda ns, fp, e, r, d: ns["payload"]["springs"].append(copy.deepcopy(ns["payload"]["springs"][0])), "i_unique_declared_springs"),
            ("(r3-D7) a duplicate declared prescribed DOF", lambda ns, fp, e, r, d: ns["payload"]["prescribed"].append(copy.deepcopy(ns["payload"]["prescribed"][0])), "i_unique_declared_prescribed"),
            ("(r3-D8) a duplicate plan functional id", lambda ns, fp, e, r, d: fp["payload"]["functions"][1].__setitem__("functional_id", fp["payload"]["functions"][0]["functional_id"]), "i_unique_declared_functionals"),
            ("(r3-D9) a duplicate eigenload record", lambda ns, fp, e, r, d: ns["payload"]["load_reference_state"]["eigen_loads"].append(copy.deepcopy(ns["payload"]["load_reference_state"]["eigen_loads"][0])), "i_unique_declared_eigen_loads")):
        expect(label, *mutated(fn_), "timo", False, check)

    def endpoint_huge(ns, fp, e, r, d):  # Codex note: an auxiliary endpoint estimate huge inside a wide interval
        ep = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["endpoints"][0]
        ep["value_pa"], ep["interval_pa"] = 1234567.0, [0.0, 1234567.0]
    expect("(r3-E1) a maximum endpoint estimate 1234567 inside a wide interval", *mutated(endpoint_huge, rehash=False), "timo", False, "c_maximum_endpoint_claims")

    def endpoint_negative(ns, fp, e, r, d):
        ep = e["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["endpoints"][1]
        ep["value_pa"], ep["interval_pa"] = -1.0, [-1.0, ep["interval_pa"][1] + 1.0]
    expect("(r3-E2) a negative maximum endpoint estimate inside a wide interval", *mutated(endpoint_negative, rehash=False), "timo", False, "c_maximum_endpoint_claims")

    # repair round 4: distinct parallel springs on one DOF are valid and equal the combined spring exactly
    split = synth("timo", "both", split_spring=True)
    expect("(r4) two distinct parallel springs (2 x 5e-5) on the root RX DOF pass", *split, "timo", True)
    one = synth("timo", "both")

    def exact_by_identity(dump):
        ns_ = json.loads(dump["records"][0]["normalized_source_text"])["payload"]
        fp_ = json.loads(dump["records"][0]["functional_plan_text"])["payload"]
        ctx_ = solve_declared(ns_, fp_)
        keyed = {}
        for fid, (_, fn) in ctx_["by_id"].items():
            if fn["quantity"] in ("nodal_translation", "nodal_rotation", "member_end_action", "member_station_action") \
                    or (fn["quantity"] == "support_action_component" and fn["owner"]["id"] == "anchor"):
                keyed[(fn["quantity"], fn["owner"]["id"], fn["location"], fn["component"])] = ctx_["exact"][fid]
        return ctx_["u"], keyed
    u1, k1 = exact_by_identity(one[0])
    u2, k2 = exact_by_identity(split[0])
    record("(r4) the parallel-spring declared problem solves to the combined spring's exact answer", u1 == u2 and k1 == k2,
           f"u equal {u1 == u2}; {len(k1)} node, member and anchor functionals equal {k1 == k2}")

    def dup_spring_id(ns, fp, e, r, d):
        ns["payload"]["springs"][1]["spring_id"] = ns["payload"]["springs"][0]["spring_id"]
    expect("(r4) two springs with one spring id", *mutated(dup_spring_id, split_spring=True), "timo", False, "i_unique_declared_springs")

    # repair round 1, items 6 and 7: batch-level vacuity, the required-selection list and mode errors
    good = synth("timo", "both")
    entry = {"twin": "synth|both|timo", "family": "synth", "name": "synth", "variant": "both", "theory": "timo"}

    def loader(dump_mode=None, receipt=True):
        def load(ent, mode):
            d, e, r = copy.deepcopy(good)
            d["solver_mode"] = dump_mode or mode
            if not receipt:
                del e["source_block_recovery"]
                d["records"] = []
            return d, e, r
        return load
    req = [("synth|both|timo", m, "case") for m in ("sparse_interactive", "dense_scrutiny")]
    _, s0 = judge_batch([entry], loader(), bound, req, echo=False)
    record("(r6) batch with its required pairs selected passes", s0["verdict"] == "PASS", f"{s0['verdict']} {s0['batch_failures']}")
    _, s1 = judge_batch([], loader(), bound, req, echo=False)
    record("(r6) empty index fails as vacuous", s1["verdict"] == "FAIL" and any("vacuous" in b for b in s1["batch_failures"]), f"{s1['batch_failures']}")
    _, s2 = judge_batch([entry], loader(receipt=False), bound, req, echo=False)
    record("(r6) a required pair that does not select fails the batch", s2["verdict"] == "FAIL" and any("required" in b for b in s2["batch_failures"]), f"{s2['batch_failures']}")
    _, s3 = judge_batch([entry], loader(), bound, [], echo=False)
    record("(r6) an empty required-selection list fails the batch", s3["verdict"] == "FAIL", f"{s3['batch_failures']}")
    c0 = control_batch([], lambda *a: None, lambda e: None, bound)
    record("(r6) empty control set is vacuous", set(c0["vacuous"]) == {"cross_theory", "eb_reference_on_timo_k", "cross_allowance"}, f"vacuous {c0['vacuous']}")
    _, s5 = judge_batch([entry, dict(entry)], loader(), bound, req, echo=False)
    record("(r3-D10) a duplicate document in the batch index fails the batch", s5["verdict"] == "FAIL" and any("duplicate documents" in b for b in s5["batch_failures"]), f"{s5['batch_failures']}")
    _, s6 = judge_batch([entry], loader(), bound, req + req[:1], echo=False)
    record("(r3-D11) a duplicate required triple fails the batch", s6["verdict"] == "FAIL" and any("duplicate required" in b for b in s6["batch_failures"]), f"{s6['batch_failures']}")
    reps, s4 = judge_batch([entry], loader(dump_mode="dense_scrutiny"), bound, req, echo=False)
    kept = any("solver_mode" in e_ for rp in reps for e_ in rp["errors"])
    record("(r7) a solver-mode mismatch is kept and fails the batch", kept and s4["verdict"] == "FAIL", f"errors {[rp['errors'] for rp in reps]}")

    bad = [o for o in outcomes if not o[1]]
    print(f"selftest: {len(outcomes) - len(bad)}/{len(outcomes)} as expected")
    return not bad


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--dump")
    ap.add_argument("--envelope")
    ap.add_argument("--request")
    ap.add_argument("--theory", choices=["eb", "timo"])
    ap.add_argument("--allowance", choices=["zero", "mill", "ca", "both"])
    ap.add_argument("--index")
    ap.add_argument("--runs")
    ap.add_argument("--out")
    ap.add_argument("--controls", action="store_true", help="with --index: also run the real-data controls")
    ap.add_argument("--required", help="with --index: the committed required-selection list (required_selection.json)")
    ap.add_argument("--required-set", help="with --index: the set name within --required")
    args = ap.parse_args()
    bound = load_bound()
    if args.selftest:
        sys.exit(0 if selftest() else 1)
    if args.index:
        runs = args.runs or os.path.dirname(args.index)
        if not (args.required and args.required_set):
            raise SystemExit("oracle: --index needs --required FILE and --required-set NAME (repair round 1, item 6)")
        required = load_required(args.required, args.required_set)
        summary = run_index(args.index, runs, bound, args.out, required)
        ok = summary["verdict"] == "PASS"
        if args.controls:
            controls = run_controls(args.index, runs, bound)
            ok = ok and not controls["vacuous"] and all(
                v["passed_wrongly"] == 0 for k, v in controls.items() if k not in ("checks_that_failed", "vacuous"))
            if args.out:
                with open(args.out, encoding="utf-8") as fh:
                    doc = json.load(fh)
                doc["controls"] = controls
                with open(args.out, "w", encoding="utf-8") as fh:
                    json.dump(doc, fh, ensure_ascii=False, indent=1, default=str)
        sys.exit(0 if ok else 1)
    if args.dump and args.envelope:
        with open(args.dump, encoding="utf-8") as fh:
            dump = json.load(fh)
        with open(args.envelope, encoding="utf-8") as fh:
            env = json.load(fh)
        request = None
        if args.request:
            with open(args.request, encoding="utf-8") as fh:
                request = json.load(fh)
        rep = judge(dump, env, request, bound, args.theory, args.allowance)
        print(json.dumps(rep, ensure_ascii=False, indent=1, default=str))
        sys.exit(0 if rep["cases"] and all(c["verdict"] == "PASS" for c in rep["cases"]) and not rep["errors"] else 1)
    ap.print_help()
    sys.exit(2)


if __name__ == "__main__":
    main()
