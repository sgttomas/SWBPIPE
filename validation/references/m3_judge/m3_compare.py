"""The M3 comparison: the K-D5 M3 rules on the canonical evidence view, for both routes.

It reads only the view (m3_types.py): judge_model takes a ModelView, an Expected and RunViews, refuses anything
else (TypeError), and never sees a raw record. It imports no JSON reader and no validator; m3_selftest.py asserts
that from the source and from a fresh interpreter. The view guarantees what used to be checked here: one outcome
per run, unique row and extremum identities, case-bound and validated evidence, contract units, lo <= hi.

The judging and coverage rules are kd5t_compare.py's reconstruction of the M3 harness (kd5_timoshenko/, refutation
rounds 1-4), ported unchanged in meaning; sources are cited as there: [K-D5 §n] is
`execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md`, [Design §n] is
`execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md`, [cc] the surviving harness
compare_current.py, [own, stricter] a rule no source states.

Judging rules
- CRIT = RD(1e-9), the largest binary64 <= 1e-9 [K-D5 §3.3]. Breach <=> sum (v - t)^2 > CRIT^2 sum v^2 [cc].
- FLOOR = 2^-34, GUARD = 2^-988 [K-D5 §3.3]. Judged <=> S >= GUARD and max(q^2, t^2) conv^2 >= FLOOR^2 S^2 [cc].
- Groups [K-D5 §3.2]: per location |N|, hypot(V_y, V_z), |T|, hypot(M_y, M_z); each pipe-wall kind its own group;
  stress rows through the resultant they scale (s_axial, s_bending, s_torsion, membrane); the support force and
  moment vectors and magnitudes; displacement, rotation and displacement-magnitude groups; Lame rows a group per
  (pipe, location, basis) against their own q (C1 when they breach; manager ruling M3T2, F4). The key is (group,
  entity, location, section basis): both D-5 bases on the exact route, PF's single section (basis None) on PF.
- conv to force units: s_axial A_b 1e6, s_bending Z_b 1e6, s_torsion J_b/r_o 1e6, membrane A_b, with the row's
  basis section [cc; Design §2.1].
- Coupled scales SF = max(force, moment/L_b), SM = max(moment, L_b force) over the published action groups, L_b
  the node bounding-box diagonal [K-D5 §3.3; cc]. Displacements and rotations are judged on their own q [cc;
  K-D5 §4.1]; their own-q breaches are reported, not acceptance.
- Reference scales [M3T2, F1]: a force, moment, stress or Lame group is also judged on the scales max(published,
  reference), so a vanishing publication is judged and breaches.
- Overflow [M3T2, F2]: every decision is an exact rational predicate; ratios are display values (+inf beyond
  binary64). A published null (serde's non-finite) is a C1 breach on a Passed case, C6i otherwise.
- C3 scales [K-D5 "The defect", §3.3, §4.1]: S*_t = max(S_t, L_b S_r), S*_r = max(S_r, S_t/L_b), from the body's
  largest published translation and rotation components (mm, rad; L_b in mm). No noise floor (HELP_HUMAN,
  2026-10-12).
- One body per model; a model with more than one body or with components is a harness error (C0).
- The solver-mode row is skipped; the maxima are checked by coverage [cc]. Rows of contract kinds the reference
  does not model (the exact route's derived rows; any other contract kind) are counted, not judged.
- Coverage: U = the evidence `value_upper_pa` of the same case, pipe and section basis (the maximum row's own
  value when there is no evidence); every published station sample of that pipe and basis satisfies
  (U - |axial|)^2 >= My^2 + Mz^2 with U - |axial| >= 0, exactly [K-D5 §3.2, §4; cc]. The axial sample is the wall
  membrane stress where it is published [K-D5 §3.2].

Acceptance, per run (the K-D5 M3 criterion and the soundness of its judging; a run fails on any clause):
- C0: every run judged (no runner error, reference or harness error).
- C1 [K-D5 §4.3]: 0 judged member, section, support or stress (Lame included) breaches on Passed cases, and no
  null published value on a Passed case.
- C2 [K-D5 §3.2, §4, §4.3]: 0 strict coverage failures of the circular maxima, on every run with rows.
- C3 [K-D5 §4.3]: on Passed cases every displacement and rotation group within RD(1e-9) of S*_kind.
- C4 [K-D5 §4.3; b4]: 0 typed/captured disagreements (standing codes or breach counts).
- C6 [M3T2, F3]: on Passed cases, no reference row or maximum unpublished.
- C8 [M3T2, F1]: on Passed cases, no group the reference itself would judge (factor-2 margin on the floor) left
  unjudged, and no body with a nonzero reference and nothing judged.
- C9 [M3T4]: a row identity published more than once. Since refutation round 5 the view refuses every repeated
  identity (whatever its value and standing), so the record is schema-invalid: C0 and an inventory failure.
Design conformance (separate; PF only): C10 [pf-timoshenko.md §2]: an expected refusal not refused by its named
code, or an admitted model refused.
Information (counted, not acceptance): C3lit (C3 against the separate published scales), C4b (typed/captured rows
differ), C5 (refusal), C6i (unmapped published rows; unpublished or null rows when not Passed), C7 (maximum
evidence missing, an enclosure that misses the exact maximum by more than CRIT, a headline that is not the maximum
of its basis), and on PF: C11 (derived rows beyond 1e-9 of their coupled scale, unpublished, or the tau-domain
warning differs), C11tau_b (the known tau_v,max defect on main, Agent 0 option (b)) and C11ops (derived rows
against the design's prescribed binary64 operations).
"""

from __future__ import annotations

import collections
import math
from fractions import Fraction as Q

from m3_types import SECTION_BASES, Expected, ModelView, Row, RunView

_c = 1e-9
CRIT = Q(_c) if Q(_c) <= Q(1, 10 ** 9) else Q(math.nextafter(_c, 0.0))
FLOOR = Q(1, 2 ** 34)
GUARD = Q(1, 2 ** 988)
MPA = Q(10 ** 6)
PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"
SENSITIVE = "NUMERICAL_INTEGRITY_SENSITIVE"
MAXIMUM = "pipe_elastic_normal_stress_maximum_v2"
SOLVER = "linear_solver_mode_basis"
TAU_WARNING = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN"
DERIVED = frozenset({"element_global_end_force_v1", "element_global_end_moment_v1", "element_local_normal_stress_extreme",
                     "element_local_transverse_shear_stress_maximum"})
TAU_OPS_TOL = Q(1, 2 ** 40)

GROUP_OF_KIND = {
    "global_nodal_displacement_x": ("disp", "D"), "global_nodal_displacement_y": ("disp", "D"), "global_nodal_displacement_z": ("disp", "D"),
    "global_nodal_rotation_x": ("rot", "R"), "global_nodal_rotation_y": ("rot", "R"), "global_nodal_rotation_z": ("rot", "R"),
    "displacement_magnitude": ("dispmag", "D"),
    "element_local_axial_force": ("axial", "F"), "element_local_shear_force_y": ("shear", "F"), "element_local_shear_force_z": ("shear", "F"),
    "element_local_torsional_moment": ("torsion", "M"), "element_local_bending_moment_y": ("bending", "M"), "element_local_bending_moment_z": ("bending", "M"),
    "element_local_axial_normal_stress": ("s_axial", "S"), "element_local_bending_normal_stress_y": ("s_bending", "S"),
    "element_local_bending_normal_stress_z": ("s_bending", "S"), "element_local_torsional_shear_stress": ("s_torsion", "S"),
    "pipe_wall_axial_force_v2": ("pipe_wall_axial_force_v2", "F"), "pipe_effective_axial_force_v2": ("pipe_effective_axial_force_v2", "F"),
    "pipe_wall_endpoint_action_v2": ("pipe_wall_endpoint_action_v2", "F"),
    "pipe_axial_membrane_stress_v2": ("pipe_axial_membrane_stress_v2", "S"),
    "pipe_lame_radial_stress_v2": ("lame", "X"), "pipe_lame_hoop_stress_v2": ("lame", "X"),
}
REACT = {"Fx": "reactF", "Fy": "reactF", "Fz": "reactF", "Mx": "reactM", "My": "reactM", "Mz": "reactM"}
SUPPORT_KINDS = {"support_reaction_component_v2", "support_reaction_force_magnitude_v2", "support_reaction_moment_magnitude_v2"}
ACT = {"axial", "shear", "torsion", "bending", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2",
       "reactF", "reactM", "reactFmag", "reactMmag"}
NONACT = {"disp", "rot", "dispmag", "rotmag"}
CLAUSES = {
    "C0": "runner, reference or harness error",
    "C1": "judged member/section/support/stress (incl. Lame) breach, or a null value, on a Passed case",
    "C2": "strict coverage failure of a circular maximum",
    "C3": "displacement/rotation group beyond 1e-9 of the body's coupled scale S*_kind (Passed)",
    "C4": "typed/captured disagreement (standing codes or breach counts)",
    "C6": "expected reference row or maximum unpublished (Passed)",
    "C8": "vacuous pass: a group the reference judges is not judged, or nothing judged (Passed)",
    "C9": "a row identity published more than once: refused by the view (schema-invalid, C0)",
}
DESIGN = {"C10": "admission differs from pf-timoshenko.md section 2 (PF only)"}
INFO = {
    "C3lit": "displacement/rotation group beyond 1e-9 of the body's separate published scale (Passed)",
    "C4b": "typed/captured published rows differ",
    "C5": "refused",
    "C6i": "unmapped published rows; unpublished or null rows when not Passed",
    "C7": "maximum evidence missing, exact maximum outside the enclosure, or headline finding",
    "C11": "PF derived row beyond 1e-9 of its coupled scale and not the known tau defect, unpublished, or its tau-domain warning differs (Passed)",
    "C11tau_b": "PF tau_v,max equal to the prescribed binary64 operations but beyond 1e-9: the known defect on main (Agent 0 option (b))",
    "C11ops": "PF sigma+- not bit for bit, or tau_v,max not within 2^-40, of the prescribed operations on the published rows (Passed)",
}


# ----------------------------------------------------------------- exact predicates
def group_judged(q2, t2, conv, s):
    """[K-D5 §3.3; cc] s >= 2^-988 and max(q^2, t^2) conv^2 >= FLOOR^2 s^2."""
    return s >= GUARD and max(q2, t2) * conv ** 2 >= FLOOR ** 2 * s ** 2


def group_breach(e2, q2):
    """[K-D5 §3.3; cc] sum (v - t)^2 > RD(1e-9)^2 sum v^2."""
    return e2 > CRIT ** 2 * q2


def circular_public_sample_covered(upper, axial, my, mz):
    difference = upper - abs(axial)
    return difference >= 0 and difference * difference >= my * my + mz * mz


def qf(x):
    """float(x), or +-inf beyond the binary64 range (display only) [M3T2, F2]."""
    try:
        return float(x)
    except OverflowError:
        return math.inf if x > 0 else -math.inf


def qsqrt(x):
    """sqrt of a nonnegative Fraction rounded down to a relative 2^-64, with no binary64 range limit."""
    if x <= 0:
        return Q(0)
    n, d = x.numerator, x.denominator
    return Q(math.isqrt((n * d) << 128), d << 64)


def qratio(e2, d2):
    """sqrt(e2/d2) for display: 0 when e2 = 0, +inf when d2 = 0 < e2 or beyond the binary64 range."""
    if not e2:
        return 0.0
    if not d2:
        return math.inf
    return math.sqrt(qf(e2 / d2))


def classify(code, message):
    """Standing classes (kd5-eval-final/summarize.py)."""
    if code == PASSED:
        return "passed"
    if code != SENSITIVE:
        return "other:" + code
    for marker, cls in (("S11-G formation-noise guard", "sensitive-load-row"), ("S11-G recovery guard (R-b'): load case", "sensitive-recovery-guard"),
                        ("the member-relative formation check is not available", "sensitive-no-member-bound"),
                        ("reason=formation_check_unavailable", "sensitive-fcu"), ("reason=estimate", "sensitive-kd5-estimate"),
                        ("reciprocal_condition_estimate", "sensitive-gate")):
        if marker in message:
            return cls
    return "sensitive-solve-other"


# ----------------------------------------------------------------- PF derived rows (C11, C11tau_b, C11ops)
def rn_norm2(a: float, b: float) -> float:
    """The correctly rounded binary64 sqrt(a^2 + b^2) of two binary64 values (the shared norm2, E15)."""
    X = Q(a) ** 2 + Q(b) ** 2
    if X == 0:
        return 0.0
    c = math.hypot(a, b)
    if not math.isfinite(c):
        return c
    while True:
        lo, hi = (Q(math.nextafter(c, 0.0)) + Q(c)) / 2, (Q(c) + Q(math.nextafter(c, math.inf))) / 2
        if X < lo * lo:
            c = math.nextafter(c, 0.0)
        elif X > hi * hi:
            c = math.nextafter(c, math.inf)
        else:
            return c


def sigma_ops(sN: float, sy: float, sz: float):
    """u4b-u5b-implementation.md section 2.3: h = norm2(sigma_y, sigma_z); RN(sigma_N + h), RN(sigma_N - h)."""
    h = rn_norm2(sy, sz)
    return sN + h, sN - h


def tau_cb(t):
    """Section 2.4's c_b in its fixed binary64 sequence; I_b is the exact I rounded once (its order is not prescribed)."""
    nu = t.E / (2.0 * t.G) - 1.0
    ro = t.od / 2.0
    ri = ro - (t.wall - t.mill)
    num = (3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri)
    den = (4.0 * (1.0 + nu)) * float(t.I)
    return num / den


def tau_ops(t, vy: float, vz: float) -> float:
    """Section 2.4: tau = RN(RN(norm2(V_y, V_z) c_b)/1e6), on the station's published shear rows."""
    return (rn_norm2(vy, vz) * tau_cb(t)) / 1e6


def _derived(exp, ec, cs, derived_rows, core_rows, pipes):
    """C11, C11tau_b and C11ops of one Passed case of a PF run."""
    out = {"n_groups": 0, "beyond": [], "tau_b": [], "ops": [], "missing": 0, "unexpected": 0, "worst": 0.0}
    SF, SM = max(cs["_SF"], cs["_SFr"]), max(cs["_SM"], cs["_SMr"])
    refrows = dict(ec.derived)
    operands = {(r.kind, r.entity, r.location): r.value for r in core_rows if r.value is not None}
    pub = {}
    for r in derived_rows:
        if r.key not in refrows or r.value is None:
            out["unexpected"] += 1
            continue
        pub[r.key] = r.value
    out["missing"] += sum(1 for k in refrows if k not in pub)
    groups = collections.defaultdict(list)
    for key, v in pub.items():
        kind, pid, loc, comp, _ = key
        groups[(kind, pid, loc)].append((comp, v, refrows[key]))
    for (kind, pid, loc), vals in groups.items():
        t = pipes[pid].tau
        props = dict(pipes[pid].bases)[None]
        op = lambda k: operands.get((k, pid, loc))
        consistent = None
        if kind == "element_global_end_force_v1":
            scale = SF
        elif kind == "element_global_end_moment_v1":
            scale = SM
        elif kind == "element_local_normal_stress_extreme":
            scale = SM / (props.Z * MPA)
            ops = (op("element_local_axial_normal_stress"), op("element_local_bending_normal_stress_y"), op("element_local_bending_normal_stress_z"))
            if None not in ops:
                sp, sm = sigma_ops(*ops)
                consistent = all(v == (sp if comp == "sigma_plus" else sm) for comp, v, _ in vals)
        else:
            scale = SF * t.tau_c / MPA
            ops = (op("element_local_shear_force_y"), op("element_local_shear_force_z"))
            if None not in ops:
                tt = tau_ops(t, *ops)
                consistent = all(abs(Q(v) - Q(tt)) <= TAU_OPS_TOL * abs(Q(tt)) for _, v, _ in vals)
        e2 = sum((Q(v) - tv) ** 2 for _, v, tv in vals)
        out["n_groups"] += 1
        ratio = qratio(e2, CRIT ** 2 * scale * scale)
        item = [ec.case, kind, pid, loc, ratio]
        if consistent is False:
            out["ops"].append(item)
        if e2 > CRIT ** 2 * scale * scale:
            if kind == "element_local_transverse_shear_stress_maximum" and consistent:
                out["tau_b"].append(item)
            else:
                out["beyond"].append(item)
                out["worst"] = max(out["worst"], ratio)
    return out


# ----------------------------------------------------------------- one run
def _group_of(row):
    if row.kind == "support_reaction_component_v2":
        g = REACT[row.component]
        return (g, row.entity, row.location, None), ("F" if g == "reactF" else "M")
    if row.kind == "support_reaction_force_magnitude_v2":
        return ("reactFmag", row.entity, row.location, None), "F"
    if row.kind == "support_reaction_moment_magnitude_v2":
        return ("reactMmag", row.entity, row.location, None), "M"
    g, fam = GROUP_OF_KIND[row.kind]
    return (g, row.entity, row.location, row.basis), fam


def _group_scale(key, fam, pipes, SF, SM, q2):
    conv, s = Q(1), None
    if key[0] in ("s_axial", "s_bending", "s_torsion", "pipe_axial_membrane_stress_v2"):
        p = pipes[key[1]]
        sec = dict(p.bases)[key[3]]
        if key[0] == "s_axial":
            conv, s = sec.A * MPA, SF
        elif key[0] == "s_bending":
            conv, s = sec.Z * MPA, SM
        elif key[0] == "s_torsion":
            conv, s = sec.J / p.ro * MPA, SM
        else:
            conv, s = sec.A, SF
    elif fam == "F":
        s = SF
    elif fam == "M":
        s = SM
    else:
        s = qsqrt(q2)
    return conv, s


def _coupled(groups, Lb, which):
    force = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "F"] + [Q(0)])
    moment = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "M"] + [Q(0)])
    LbQ = Q(Lb)
    SF = max(force, moment / LbQ) if Lb else force
    SM = max(moment, LbQ * force) if Lb else moment
    return SF, SM


def judge_run(model, exp, run):
    """The judgement of one run on its expected reference: a plain record (an output, never read back)."""
    rec = {"tag": run.tag, "entry": run.entry, "mode": run.mode, "route": run.route}
    if run.error is not None:
        rec.update(runner_error=run.error, verdict_failures={"C0": 1}, info={}, design_failures={"unjudged": 1}, cls="runner-error")
        return rec
    if model.harness_error:
        raise ValueError(model.harness_error)
    fails, info, design = collections.Counter(), collections.Counter(), collections.Counter()
    rec["integrity"] = [s.code for s in run.standings]
    rec["blocking"] = [run.refusal] if run.refusal else []
    rec["codes"] = sorted({s.code for s in run.standings} | set(run.warnings) | set(rec["blocking"]))
    if exp.expect == "refused":
        design["C10"] += int(run.refusal != exp.refusal)
    elif exp.expect == "rows":
        design["C10"] += int(run.refusal is not None)
    rec["expect"] = exp.expect
    if run.refusal is not None:
        rec.update(cls="refused", verdict_failures={}, info={"C5": 1}, design_failures=dict(design), n_judged=0, n_judged_passed=0, passed_cases=0,
                   breaches=[])
        return rec
    first = next((s for s in run.standings if s.code != PASSED), None)
    rec["cls"] = "passed" if first is None else classify(first.code, first.message)
    rec["msg_tail"] = None if first is None else first.message[-400:]
    standing = {s.case: s.code for s in run.standings}
    pipes = {p.pipe: p for p in exp.sections}
    Lb = exp.Lb
    rec["cases"] = {}
    all_groups = []
    derived_all = []
    maxvals = collections.defaultdict(list)
    max_fail = []
    n_other = 0
    for ec in exp.cases:
        cid = ec.case
        passed = standing[cid] == PASSED
        refrows = dict(ec.rows)
        groups, used, unmapped, nonfinite, skips = {}, set(), [], [], 0
        station = collections.defaultdict(dict)
        maxrows, derived_rows, core_rows = [], [], []
        for row in run.rows:
            if row.case != cid:
                continue
            if exp.derived_judged and row.kind in DERIVED:
                derived_rows.append(row)
                continue
            if row.kind not in GROUP_OF_KIND and row.kind not in SUPPORT_KINDS and row.kind not in (MAXIMUM, SOLVER):
                n_other += 1          # a contract kind the reference does not model: counted, not judged
                continue
            core_rows.append(row)
            if row.value is None:
                nonfinite.append(list(row.key))
                continue
            if row.kind == MAXIMUM:
                maxrows.append(row)
                maxvals[row.basis].append(row.value)
                continue
            if row.kind == SOLVER:
                skips += 1
                continue
            if row.key not in refrows:
                unmapped.append(list(row.key))
                continue
            used.add(row.key)
            gkey, fam = _group_of(row)
            g = groups.setdefault(gkey, {"fam": fam, "pub": [], "ref": []})
            g["pub"].append(Q(row.value))
            g["ref"].append(refrows[row.key])
            if row.kind in ("element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"):
                station[(row.entity, row.basis, row.location)][row.kind] = Q(row.value) * MPA
            elif row.kind == "pipe_axial_membrane_stress_v2":
                station[(row.entity, row.basis, row.location)][row.kind] = Q(row.value)
        missing = [list(k) for k in refrows if k not in used]
        SF, SM = _coupled(groups, Lb, "pub")
        SFr, SMr = _coupled(groups, Lb, "ref")
        SFm, SMm = max(SF, SFr), max(SM, SMr)
        sT = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] == "disp"] + [Q(0)])
        sR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] == "rot"] + [Q(0)])
        LbMM = Q(Lb) * 1000
        body_kd5 = {"D": max(sT, LbMM * sR) if Lb else sT, "R": max(sR, sT / LbMM) if Lb else sR}
        bD = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "D"] + [Q(0)])
        bR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "R"] + [Q(0)])
        body_literal = {"D": bD, "R": bR}
        fam_ref = collections.defaultdict(lambda: Q(0))
        for k, g in groups.items():
            if g["fam"] in ("D", "R", "X"):
                fam_ref[g["fam"]] = max(fam_ref[g["fam"]], max(abs(x) for x in g["ref"]))
        stats, unjudged_by_ref, ref_nonzero = [], [], False
        for key, g in groups.items():
            q2 = sum(x * x for x in g["pub"])
            t2 = sum(x * x for x in g["ref"])
            e2 = sum((a - b) ** 2 for a, b in zip(g["pub"], g["ref"]))
            conv, s = _group_scale(key, g["fam"], pipes, SF, SM, q2)
            judged = group_judged(q2, t2, conv, s)
            judged_ref_only = False
            if not judged and g["fam"] not in ("D", "R"):
                _, sm = _group_scale(key, g["fam"], pipes, SFm, SMm, max(q2, t2))
                judged = judged_ref_only = group_judged(q2, t2, conv, sm)
            st = {"case": cid, "passed": passed, "group": key[0], "entity": key[1], "loc": key[2], "basis": key[3], "n": len(g["pub"]),
                  "q": math.sqrt(qf(q2)), "err": math.sqrt(qf(e2)), "ratio": qratio(e2, CRIT ** 2 * q2), "judged": judged,
                  "breach": group_breach(e2, q2), "judged_ref_only": judged_ref_only}
            if g["fam"] in body_kd5:
                for name, scales in (("kd5", body_kd5), ("literal", body_literal)):
                    sc = scales[g["fam"]]
                    st["beyond_" + name] = e2 > CRIT ** 2 * sc * sc
                    st["ratio_" + name] = qratio(e2, CRIT ** 2 * sc * sc)
            if t2 > 0:
                ref_nonzero = True
                if g["fam"] in ("D", "R", "X"):
                    sr = fam_ref[g["fam"]]
                else:
                    _, sr = _group_scale(key, g["fam"], pipes, SFr, SMr, t2)
                if sr >= GUARD and t2 * conv ** 2 >= 4 * FLOOR ** 2 * sr ** 2 and not judged:
                    unjudged_by_ref.append([key[0], key[1], key[2], key[3]])
            stats.append(st)
        # coverage of the circular maxima: evidence of this case, pipe and basis only
        evidence = {(e.pipe, e.basis): e for e in run.extrema if e.case == cid}
        maxima_ref = dict(ec.maxima)
        mchecks, seen_max = [], set()
        for row in maxrows:
            pid, basis = row.entity, row.basis
            seen_max.add((pid, basis))
            ev = evidence.get((pid, basis))
            upper_value = ev.upper if ev is not None else row.value
            finite_upper = upper_value is not None
            upper = Q(upper_value) if finite_upper else Q(0)
            lower = Q(ev.lower) if ev is not None and ev.lower is not None else None
            comps = []
            for (p, b, loc), v in station.items():
                if p != pid or b != basis:
                    continue
                axial = v.get("pipe_axial_membrane_stress_v2", v.get("element_local_axial_normal_stress", Q(0)))
                comps.append((axial, v.get("element_local_bending_normal_stress_y", Q(0)), v.get("element_local_bending_normal_stress_z", Q(0))))
            covered = finite_upper and all(circular_public_sample_covered(upper, *c) for c in comps)
            truth = maxima_ref.get((pid, basis))
            ext = None
            if truth is not None and truth > 0:
                gap = max(Q(0), truth - upper, (lower - truth) if lower is not None else Q(0))
                ext = qf(gap / (CRIT * truth)) if finite_upper else math.inf
            mchecks.append({"case": cid, "entity": pid, "basis": basis, "value": row.value, "upper": qf(upper) if finite_upper else None,
                            "n_samples": len(comps), "covers_published_stations": covered, "evidence": ev is not None,
                            "ref_outside_enclosure_over_crit": ext})
        missing_max = sorted((k for k in maxima_ref if k not in seen_max), key=repr)
        n_judged = sum(g["judged"] for g in stats)
        cs = {"passed": passed, "n_groups": len(stats), "n_judged": n_judged, "skips": skips, "unmapped": unmapped[:20], "n_unmapped": len(unmapped),
              "missing": missing[:20], "n_missing": len(missing), "missing_maxima": [list(x) for x in missing_max],
              "unjudged_by_ref": unjudged_by_ref[:10], "n_unjudged_by_ref": len(unjudged_by_ref), "nonfinite": nonfinite[:10], "n_nonfinite": len(nonfinite),
              "n_judged_ref_only": sum(g["judged_ref_only"] for g in stats), "max_checks": mchecks, "SF": qf(SF), "SM": qf(SM), "SF_ref": qf(SFr),
              "SM_ref": qf(SMr), "body_D": qf(bD), "body_R": qf(bR), "S_t": qf(sT), "S_r": qf(sR), "Sstar_t": qf(body_kd5["D"]),
              "Sstar_r": qf(body_kd5["R"]), "_SF": SF, "_SM": SM, "_SFr": SFr, "_SMr": SMr}
        unpublished = len(missing) + len(missing_max)
        info["C6i"] += len(unmapped)
        info["C7"] += sum(1 for c in mchecks if not c["evidence"])
        if passed:
            fails["C1"] += len(nonfinite)
            fails["C6"] += unpublished
            fails["C8"] += len(unjudged_by_ref) + (1 if ref_nonzero and n_judged == 0 else 0)
            info["C7"] += sum(1 for c in mchecks if (c["ref_outside_enclosure_over_crit"] or 0) > 1)
            if exp.derived_judged:
                d = _derived(exp, ec, cs, derived_rows, core_rows, pipes)
                derived_all.append(d)
        else:
            info["C6i"] += unpublished + len(nonfinite)
        max_fail += [c for c in mchecks if not c["covers_published_stations"]]
        all_groups += stats
        rec["cases"][cid] = cs
    # headlines: run level, over every case's maxima of each basis
    head = []
    if maxvals:
        hs = {h.basis: h for h in run.headlines}
        if exp.single_headline:
            vals = [v for vs in maxvals.values() for v in vs]
            h = hs.get(None)
            if h is None or h.is_null or h.value != max(vals):
                head.append(f"headline {None if h is None else h.value} != max {max(vals)}")
        else:
            h0 = hs.get(None)
            if h0 is None or not h0.is_null:
                head.append("max_open_formula_stress is not null with two section bases")
            for b in SECTION_BASES:
                vals = maxvals.get(b, [])
                h = hs.get(b)
                if not vals:
                    head.append(f"no {b} maximum rows")
                elif h is None or h.is_null or h.value != max(vals):
                    head.append(f"{b} headline {None if h is None else h.value} != max {max(vals)}")
    info["C7"] += len(head)
    rec["headline_findings"] = head
    judged = [g for g in all_groups if g["judged"]]
    rec["n_groups"] = len(all_groups)
    rec["n_judged"] = len(judged)
    rec["n_judged_passed"] = sum(1 for g in judged if g["passed"])
    rec["passed_cases"] = sum(1 for cs in rec["cases"].values() if cs["passed"])
    rec["n_judged_ref_only"] = sum(g["judged_ref_only"] for g in all_groups)
    rec["n_other_rows"] = n_other
    rec["breaches"] = [g for g in judged if g["breach"] and g["group"] not in NONACT]
    rec["lame_breaches"] = [g for g in rec["breaches"] if g["group"] == "lame"]
    rec["disp_breaches"] = [g for g in judged if g["breach"] and g["group"] in NONACT]
    rec["disp_beyond_kd5"] = [g for g in all_groups if g["group"] in NONACT and g.get("beyond_kd5") and g["passed"]]
    rec["disp_beyond_literal"] = [g for g in all_groups if g["group"] in NONACT and g.get("beyond_literal") and g["passed"]]
    worst = {}
    for g in judged:
        if g["group"] not in worst or g["ratio"] > worst[g["group"]]["ratio"]:
            worst[g["group"]] = {k: g[k] for k in ("ratio", "entity", "loc", "basis", "q", "err", "case")}
    rec["worst_by_group"] = worst
    wb = [g for g in all_groups if g["group"] in NONACT and g["passed"]]
    rec["worst_disp_kd5"] = max((g["ratio_kd5"] for g in wb), default=0.0)
    rec["worst_disp_literal"] = max((g["ratio_literal"] for g in wb), default=0.0)
    rec["max_fail"] = max_fail
    rec["n_max"] = sum(len(cs["max_checks"]) for cs in rec["cases"].values())
    rec["n_unmapped"] = sum(cs["n_unmapped"] for cs in rec["cases"].values())
    rec["n_missing"] = sum(cs["n_missing"] for cs in rec["cases"].values())
    fails["C2"] += len(max_fail)
    fails["C1"] += sum(1 for g in rec["breaches"] if g["passed"])
    fails["C3"] += len(rec["disp_beyond_kd5"])
    info["C3lit"] += len(rec["disp_beyond_literal"])
    if derived_all:
        d = {"n_groups": sum(x["n_groups"] for x in derived_all), "missing": sum(x["missing"] for x in derived_all),
             "unexpected": sum(x["unexpected"] for x in derived_all), "worst": max(x["worst"] for x in derived_all)}
        for k in ("beyond", "tau_b", "ops"):
            items = [i for x in derived_all for i in x[k]]
            d["n_" + k], d[k] = len(items), items[:20]
        d["warning_mismatch"] = int((TAU_WARNING in run.warnings) != (TAU_WARNING in exp.warnings))
        info["C11"] += d["n_beyond"] + d["missing"] + d["warning_mismatch"]
        info["C11tau_b"] += d["n_tau_b"]
        info["C11ops"] += d["n_ops"]
        info["C6i"] += d["unexpected"]
        rec["derived"] = d
    for cs in rec["cases"].values():
        for k in ("_SF", "_SM", "_SFr", "_SMr"):
            cs.pop(k)
    rec["verdict_failures"] = {k: v for k, v in fails.items() if v}
    rec["info"] = {k: v for k, v in info.items() if v}
    rec["design_failures"] = {k: v for k, v in design.items() if v}
    rec["_groups"] = all_groups       # every group's statistics; judge_model drops them from the records
    return rec


def fingerprint(run):
    return tuple((r.identity, r.unit, r.source, repr(r.value)) for r in run.rows)


def judge_model(model, expected, runs):
    """Every run of one model judged on the expected reference, with the typed/captured pairing (C4, C4b): a list of
    plain records. Arguments must be the view's types; a raw dictionary is refused."""
    if not isinstance(model, ModelView) or not isinstance(expected, Expected) or not isinstance(runs, tuple) \
            or not all(isinstance(r, RunView) and r.tag == model.tag and r.route == model.route for r in runs):
        raise TypeError("judge_model reads only the canonical view (ModelView, Expected, tuple of RunView of that model)")
    ident = collections.Counter(r.identity for r in runs)
    if any(n > 1 for n in ident.values()):
        raise ValueError("judge_model needs unique run identities (the judging pass rejects duplicates before judging)")
    by = {(r.entry, r.mode): r for r in runs}
    judged = {}
    for run in runs:
        try:
            judged[(run.entry, run.mode)] = judge_run(model, expected, run)
        except Exception as e:
            judged[(run.entry, run.mode)] = {"tag": run.tag, "entry": run.entry, "mode": run.mode, "route": run.route, "cls": "reference-error",
                                             "reference_error": f"{type(e).__name__}: {e}", "verdict_failures": {"C0": 1}, "info": {},
                                             "design_failures": {"unjudged": 1}}
    out = []
    for (entry, mode), rec in judged.items():
        if entry == "captured":
            other, orec = by.get(("typed", mode)), judged.get(("typed", mode))
            bad = {"reference_error", "runner_error"}
            if other is not None and orec is not None and not bad & (set(orec) | set(rec)):
                run = by[(entry, mode)]
                rec["typed_captured_rows_identical"] = fingerprint(other) == fingerprint(run)
                rec["captured_extra_codes"] = sorted(set(rec["codes"]) - set(orec["codes"]))
                rec["typed_extra_codes"] = sorted(set(orec["codes"]) - set(rec["codes"]))
                rec["typed_captured_disagree"] = (orec.get("integrity") != rec.get("integrity") or orec.get("blocking") != rec.get("blocking")
                                                  or len(orec.get("breaches", [])) != len(rec.get("breaches", [])))
                rec["verdict_failures"]["C4"] = rec["verdict_failures"].get("C4", 0) + int(rec["typed_captured_disagree"])
                rec["info"]["C4b"] = rec["info"].get("C4b", 0) + int(not rec["typed_captured_rows_identical"])
        rec.pop("_groups", None)
        rec["meta"] = dict(model.meta)
        rec["family"] = model.family
        rec["theory"] = model.theory
        rec["ref_variant"] = expected.variant
        rec["verdict_failures"] = {k: v for k, v in rec["verdict_failures"].items() if v}
        rec["info"] = {k: v for k, v in rec.get("info", {}).items() if v}
        rec["design_failures"] = {k: v for k, v in (rec.get("design_failures") or {}).items() if v}
        rec["verdict"] = "fail" if rec["verdict_failures"] else "pass"
        rec["design_verdict"] = "fail" if rec["design_failures"] else "pass"
        out.append(rec)
    return out
