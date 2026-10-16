"""M3-T comparison: the product's published rows against the kd5t exact reference, and the M3 verdict.

Reconstructs the judging and coverage logic of the surviving M3 harness,
`pr_kd5_eval_current/compare_current.py` in the T3 scratch tree (sha256 4b21fa99...), whose
imported modules (RV30's `compare.py` and I41's `ref_ext.py`) are lost. Sources are cited in
brackets: [K-D5 §n] is `execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md`,
[Design §n] is `execution/PKG-05/DEL-05-03/Design/section-bases-and-timoshenko.md`, [cc] is
compare_current.py, [b4] is b4-finish/analyze_b4.py, and [own, stricter] marks a rule this
harness adds that no source states. Exact rationals throughout; no product code.

Usage:
  python3 -I -B kd5t_compare.py <set.jsonl> <runs.jsonl> <out.jsonl> [--producer u6|pre-u6]
          [--ref own|twin|control:<controls.jsonl>:<key>] [--workers N]
  python3 -I -B kd5t_compare.py --selftest <set.jsonl>

--producer u6 (the default) is the T4-U6/U8 producer: every stress row must name its basis and
both families must be published [Design §§2.1, 2.5]. --producer pre-u6 is today's producer:
stress rows carry no basis and are judged on the reduced section, which is that producer's single
section (wall - mill tolerance) whenever it admits the document (zero allowance) [Design §1].

Judging rules
- CRIT = RD(1e-9), the largest binary64 <= 1e-9 (0x1.12e0be826d694p-30) [K-D5 §3.3 "exceeds(B, q)
  is today's exact B > RD(1e-9)*q"]. Breach <=> sum (v - t)^2 > CRIT^2 * sum v^2 [cc `breach`].
- FLOOR = 2^-34, GUARD = 2^-988 [K-D5 §3.3]. Judged <=> S >= GUARD and max(q^2, t^2)*conv^2 >=
  FLOOR^2*S^2 [cc `judged`, "actual-truth floor complements published q"].
- Groups [K-D5 §3.2]: per location |N|, hypot(V_y, V_z), |T|, hypot(M_y, M_z); each pipe-wall kind
  is its own group; stress rows are judged through the resultant they scale (s_axial, s_bending,
  s_torsion, membrane); support force vector, moment vector and the two scalar magnitudes;
  displacement, rotation and displacement-magnitude groups. Names are cc's ACT/conv names and the
  analysis scripts' group names. The key is (group, entity, location, section basis).
- Lame rows are stress rows (the pressure-1 `stress` family, Design §2.1) and form a group per
  (pipe, location, basis), judged against its own q: they scale no certified resultant, so no body
  scale applies [own]. K-D5 neither names nor excludes them ("The groups are every published member
  row", K-D5 §3.2; no Lame, hoop or radial row appears in either K-D5 file), so their breaches are C1
  [manager ruling M3T2, F4]. They kill the D-5 "nominal Lame at the pressure bore" and "reduced Lame
  copied from nominal" mutants.
- conv to force units: s_axial A_b*1e6, s_bending Z_b*1e6, s_torsion J_b/r_o*1e6, membrane A_b,
  with the row's basis section [cc `conv`; per basis: Design §2.1].
- Coupled scales: SF = max(force, moment/L_b), SM = max(moment, L_b*force) over the published ACT
  groups, L_b the node bounding-box diagonal [K-D5 §3.3; cc]. Displacement and rotation groups use
  their own q for judging [cc; K-D5 §4.1]; their own-q breaches are reported, not acceptance.
- Reference scales [manager ruling M3T2, F1]: a force, moment, stress or Lame group is also judged
  when it passes the guard and floor on the scales max(published, reference): SF and SM formed from
  the published and from the reference values of the same groups, and max(q, t) for a Lame group.
  So a publication that vanishes (all zero, or scaled below the guard) is judged and breaches.
  Whenever the published scale passes the guard, a group judged on the larger scale is already
  judged on the published one, so this changes nothing there.
- Overflow [manager ruling M3T2, F2]: ratios and magnitudes are binary64 display values formed
  without exceptions (beyond the binary64 range they are +inf). Every decision is an exact rational
  predicate, so a judged group whose ratio overflows is a breach. A published value that is null
  (serde's non-finite) or non-finite is a C1 breach on a Passed case [K-D5 §3.3 "A nonfinite q ...
  fails closed"], and C6i information otherwise.
- Displacement and rotation scales for C3 [K-D5 "The defect", §3.3, §4.1]: S*_t = max(S_t, L_b*S_r)
  for translation groups (components and magnitude) and S*_r = max(S_r, S_t/L_b) for rotation
  groups, with S_t and S_r the largest published translation and rotation components of the body
  (mm and rad, L_b in mm). This is the per-DOF rule's "max(|u_i|, S*_kind)" with K-D5's S*, the
  body's coupled scale over the published rows, as for the force and moment scales above.
- One body per model: per-body scales are not implemented, so a model with more than one connected
  body is a harness error (C0).
- SKIP: the solver-mode row [cc]. Maxima are checked by coverage, not as groups [cc].
- Coverage: U = evidence `value_upper_pa` of the same pipe and, on the U6 producer, the same
  `section_basis`; every published station sample of that pipe and basis satisfies (U - |axial|)^2
  >= My^2 + Mz^2 with U - |axial| >= 0, exactly [K-D5 §3.2, §4; cc
  `circular_public_sample_covered`]. The axial sample is the wall membrane stress where the member
  is pressurized [K-D5 §3.2 "the exact-pressure route uses wall axial stress"; cc used axial 0 there].

The M3 verdict. Acceptance is the K-D5 M3 criterion and the soundness of the judging itself
[Design §4 item 5: "0 judged member, section, support or stress breaches on Passed cases and 0
strict coverage failures"; K-D5 §4.3; manager ruling M3T2: a Passed case on which an expected group
is unpublished, or on which nothing is judged, is a coverage failure]. A run fails the verdict on
any acceptance clause (C0-C4, C6, C8), and the set passes when no run fails.
- C0: every run is judged (no reference or harness error); otherwise the criterion is not evaluated.
- C1 [K-D5 §4.3]: 0 judged member, section, support or stress breaches on Passed cases (pipe-wall
  groups and Lame rows included), and no null or non-finite published value on a Passed case.
- C2 [K-D5 §3.2, §4, §4.3]: 0 strict coverage failures of the circular maxima, on every run with
  results.
- C3 [K-D5 "The defect", §3.2, §4.1, §4.3]: on Passed cases every displacement and rotation group is
  within RD(1e-9) of its body's coupled scale of its kind, S*_t or S*_r. No displacement-noise floor
  applies (HELP_HUMAN, 2026-10-12): in a body whose published translations and rotations are all
  noise, the noise is the scale.
- C4 [K-D5 §4.3; b4]: 0 typed/captured disagreements (integrity codes or breach counts).
- C6 [coverage; manager ruling M3T2, F3]: on Passed cases, a reference row or a maximum (of a
  basis the producer must publish) that the product does not publish: an expected group unpublished.
- C8 [coverage; manager ruling M3T2, F1]: on Passed cases, a vacuous pass: a group that the
  reference itself would judge with a factor-2 margin on the floor (scales formed from the reference
  values) is not judged; a body with a nonzero reference has no judged group; or a Passed run
  publishes no rows (or is refused).

Reported information: clauses this harness adds, which K-D5 does not require (manager ruling: own,
stricter clauses are reported, not acceptance). They are counted per run in `info`.
- C3lit [own, stricter]: C3 against the separate published scales (translations against the largest
  published translation, component or magnitude; rotations against the largest published rotation).
- C4b [own, stricter]: typed and captured published rows differ.
- C5 [own]: refusal, DTO or runner error.
- C6i [own; Design §§2.1, 2.5 for both families]: unmapped or duplicate product rows; on the U6
  producer a stress row without a basis; on today's producer a basis-suffixed row; and, on runs that
  are not Passed, unpublished reference rows or maxima and null or non-finite values.
- C7 [own; Design §§2.2, 2.5 for the headlines]: a maximum without same-basis evidence; on Passed
  cases an evidence enclosure that misses the exact maximum by more than CRIT of it; a headline that
  is not the maximum of its basis (or a single headline that is not null on U6).
"""

from __future__ import annotations

import collections
import json
import math
import sys
from decimal import Decimal
from fractions import Fraction as Q

_c = 1e-9
CRIT = Q(_c) if Q(_c) <= Q(1, 10 ** 9) else Q(math.nextafter(_c, 0.0))
assert CRIT == Q(float.fromhex("0x1.12e0be826d694p-30"))
FLOOR = Q(1, 2 ** 34)
GUARD = Q(1, 2 ** 988)
MPA = Q(10 ** 6)
BASES = ("nominal_section", "reduced_section")

SKIP = {"linear_solver_mode_basis", "pipe_elastic_normal_stress_maximum_v2"}
STRESS_KINDS = {"element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
                "element_local_torsional_shear_stress", "pipe_axial_membrane_stress_v2", "pipe_lame_radial_stress_v2",
                "pipe_lame_hoop_stress_v2", "pipe_elastic_normal_stress_maximum_v2"}
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
ACT = {"axial", "shear", "torsion", "bending", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2",
       "reactF", "reactM", "reactFmag", "reactMmag"}
NONACT = {"disp", "rot", "dispmag", "rotmag"}
CLAUSES = {  # acceptance: the K-D5 M3 criterion and the soundness of its judging
    "C0": "reference or harness error",
    "C1": "judged member/section/support/stress (incl. Lame) breach, or a non-finite value, on a Passed case",
    "C2": "strict coverage failure of a circular maximum",
    "C3": "displacement/rotation group beyond 1e-9 of the body's coupled scale S*_kind (Passed)",
    "C4": "typed/captured disagreement (integrity codes or breach counts)",
    "C6": "expected reference row or maximum unpublished (Passed)",
    "C8": "vacuous pass: a group the reference judges is not judged, nothing judged, or no rows (Passed)",
}
INFO = {  # reported information: own, stricter clauses
    "C3lit": "displacement/rotation group beyond 1e-9 of the body's separate published scale (Passed)",
    "C4b": "typed/captured published rows differ",
    "C5": "refused, DTO or runner error",
    "C6i": "unmapped, basis-free or unexpected-basis rows; unpublished or non-finite rows when not Passed",
    "C7": "maximum evidence missing, exact maximum outside the enclosure, or headline finding",
}


def row_basis(row):
    token = (row.get("metadata") or {}).get("basis") or ""
    marker = "; section_basis="
    if marker in token:
        return token.split(marker, 1)[1].split(";")[0].strip()
    return None


def group_of(row, basis):
    kind = row["kind"]
    if kind == "support_reaction_component_v2":
        g = REACT[row["component"]]
        return (g, row["entity_ref"], row["location"], None), ("F" if g == "reactF" else "M")
    if kind == "support_reaction_force_magnitude_v2":
        return ("reactFmag", row["entity_ref"], row["location"], None), "F"
    if kind == "support_reaction_moment_magnitude_v2":
        return ("reactMmag", row["entity_ref"], row["location"], None), "M"
    g, fam = GROUP_OF_KIND[kind]
    return (g, row["entity_ref"], row["location"], basis if kind in STRESS_KINDS else None), fam


def group_judged(q2, t2, conv, s):
    """[K-D5 §3.3; cc] s >= 2^-988 and max(q^2, t^2) conv^2 >= FLOOR^2 s^2."""
    return s >= GUARD and max(q2, t2) * conv ** 2 >= FLOOR ** 2 * s ** 2


def group_breach(e2, q2):
    """[K-D5 §3.3; cc] sum (v - t)^2 > RD(1e-9)^2 sum v^2."""
    return e2 > CRIT ** 2 * q2


def circular_public_sample_covered(upper, axial, my, mz):
    difference = upper - abs(axial)
    return difference >= 0 and difference * difference >= my * my + mz * mz


def fq(text):
    return Q(Decimal(text))


def qf(x):
    """float(x) of a Fraction, or +-inf beyond the binary64 range: an explicit overflow, never an
    exception [manager ruling M3T2, F2]. Display only; no decision uses it."""
    try:
        return float(x)
    except OverflowError:
        return math.inf if x > 0 else -math.inf


def qsqrt(x):
    """sqrt of a nonnegative Fraction, rounded down to a relative 2^-64, with no binary64 range limit."""
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


def finite_value(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def qsqrt_up(x):
    """A Fraction >= sqrt(x) (float sqrt nudged up until its square bounds x)."""
    if x <= 0:
        return Q(0)
    r = Q(math.sqrt(float(x)))
    while r * r < x:
        r = Q(math.nextafter(float(r), math.inf))
    return r


def classify(codes, messages):
    """Standing classes of the harness (kd5-eval-final/summarize.py)."""
    if not codes:
        return "no-integrity"
    code, msg = codes[0], (messages[0] if messages else "")
    if code == "NUMERICAL_INTEGRITY_CHECKS_PASSED":
        return "passed"
    if code != "NUMERICAL_INTEGRITY_SENSITIVE":
        return "other:" + code
    if "S11-G formation-noise guard" in msg:
        return "sensitive-load-row"
    if "S11-G recovery guard (R-b'): load case" in msg:
        return "sensitive-recovery-guard"
    if "the member-relative formation check is not available" in msg:
        return "sensitive-no-member-bound"
    if "reason=formation_check_unavailable" in msg:
        return "sensitive-fcu"
    if "reason=estimate" in msg:
        return "sensitive-kd5-estimate"
    if "reciprocal_condition_estimate" in msg:
        return "sensitive-gate"
    return "sensitive-solve-other"


def extract_items(x):
    if isinstance(x, dict):
        yield x
        for v in x.values():
            yield from extract_items(v)
    elif isinstance(x, list):
        for v in x:
            yield from extract_items(v)


def group_scale(key, fam, sections, SF, SM, q2):
    conv, s = Q(1), None
    sec = sections.get(key[1], {}).get(key[3] or "reduced_section") if key[1] in sections else None
    if key[0] == "s_axial":
        conv, s = fq(sec["A"]) * MPA, SF
    elif key[0] == "s_bending":
        conv, s = fq(sec["Z"]) * MPA, SM
    elif key[0] == "s_torsion":
        conv, s = fq(sec["J"]) / fq(sections[key[1]]["ro"]) * MPA, SM
    elif key[0] == "pipe_axial_membrane_stress_v2":
        conv, s = fq(sec["A"]), SF
    elif fam == "F":
        s = SF
    elif fam == "M":
        s = SM
    else:
        s = qsqrt(q2)
    return conv, s


def coupled(groups, Lb, which):
    force = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "F"] + [Q(0)])
    moment = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "M"] + [Q(0)])
    LbQ = Q(Lb)
    SF = max(force, moment / LbQ) if Lb else force
    SM = max(moment, LbQ * force) if Lb else moment
    return SF, SM


def bodies(request):
    """Connected bodies of the model by its pipe segments (one is supported, see the docstring)."""
    model = request["model"]
    parent = {n["id"]: n["id"] for n in model["nodes"]}

    def find(a):
        while parent[a] != a:
            parent[a] = parent[parent[a]]
            a = parent[a]
        return a

    for p in model.get("pipe_segments") or []:
        parent[find(p["from"])] = find(p["to"])
    if model.get("components"):
        raise ValueError("components are not modelled by this harness")
    return len({find(a) for a in parent})


def judge_run(gen, ref, run, producer="u6"):
    rec = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"], "producer": producer}
    if bodies(gen["request"]) != 1:
        raise ValueError("a model with more than one body: per-body scales are not implemented")
    fails = collections.Counter()
    info = collections.Counter()
    rec["verdict_failures"] = fails
    rec["info"] = info
    ds = run.get("diagnostics") or []
    rec["blocking"] = [d["code"] for d in ds if d.get("severity") == "blocking"]
    rec["integrity"] = [d["code"] for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")]
    msgs = [d.get("message", "") for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")]
    rec["cls"] = classify(rec["integrity"], msgs)
    rec["msg_tail"] = msgs[0][-400:] if msgs and rec["cls"] != "passed" else None
    rec["notices"] = sum(1 for d in ds if d["code"] == "STRESS_FREE_WITHIN_NUMERICAL_PRECISION")
    m = None
    for msg in msgs:
        i = msg.find("reciprocal_condition_estimate: ")
        if i >= 0:
            m = msg[i + 31:].split(",")[0]
            break
    rec["rcond"] = m
    passed = rec["cls"] == "passed"
    if "error" in run:
        rec["error"] = run["error"]
        rec["verdict_failures"], rec["info"] = {}, {"C5": 1}
        return rec
    if rec["blocking"] or not run.get("results"):
        # a Passed run that publishes nothing (or is refused) judges nothing: a vacuous pass (C8)
        rec["verdict_failures"], rec["info"] = ({"C8": 1} if passed else {}), {"C5": 1}
        return rec
    rows_by_case = collections.defaultdict(list)
    for row in run["results"]:
        rows_by_case[row.get("case")].append(row)
    suffixed = any(row_basis(r) for r in run["results"])
    rec["basis_suffixed_rows"] = suffixed
    points = [[float(Decimal(c)) for c in p] for _, p in ref["nodes"]]
    delta = [max(p[a] for p in points) - min(p[a] for p in points) for a in range(3)]
    Lb = ((delta[0] ** 2 + delta[1] ** 2) + delta[2] ** 2) ** 0.5
    sections = ref["sections"]
    rec["cases"] = {}
    all_groups = []
    for cid, cref in ref["cases"].items():
        refrows = {(r[0], r[1], r[2], r[3], r[4]): fq(r[5]) for r in cref["rows"]}
        groups, skips, unmapped, used = {}, collections.Counter(), [], set()
        basis_free, unexpected_basis, nonfinite = [], [], []
        station = collections.defaultdict(dict)
        maxrows = []
        for row in rows_by_case.get(cid, []):
            kind = row["kind"]
            basis = row_basis(row)
            if not finite_value(row.get("value")):
                # null (serde's non-finite) or non-finite: C1 on a Passed case, never an exception (F2)
                nonfinite.append([kind, row["entity_ref"], row.get("location"), row.get("component"), basis])
                continue
            if kind in STRESS_KINDS:
                if producer == "u6" and basis is None:
                    basis_free.append([kind, row["entity_ref"], row.get("location")])
                if producer == "pre-u6" and basis is not None:
                    unexpected_basis.append([kind, row["entity_ref"], row.get("location"), basis])
            if kind == "pipe_elastic_normal_stress_maximum_v2":
                maxrows.append((row, basis))
                continue
            if kind in SKIP:
                skips[kind] += 1
                continue
            lookup_basis = (basis or "reduced_section") if kind in STRESS_KINDS else None
            key = (kind, row["entity_ref"], row["location"], row["component"], lookup_basis)
            if key not in refrows or key in used or (kind not in GROUP_OF_KIND and not kind.startswith("support_reaction")):
                unmapped.append(list(key))
                continue
            used.add(key)
            gkey, fam = group_of(row, lookup_basis)
            g = groups.setdefault(gkey, {"fam": fam, "pub": [], "ref": [], "kinds": []})
            g["pub"].append(Q(row["value"]))
            g["ref"].append(refrows[key])
            g["kinds"].append(kind)
            if kind in ("element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"):
                station[(row["entity_ref"], lookup_basis, row["location"])][kind] = Q(row["value"]) * MPA
            elif kind == "pipe_axial_membrane_stress_v2":
                station[(row["entity_ref"], lookup_basis, row["location"])][kind] = Q(row["value"])
        need_nominal = producer == "u6"
        missing = [list(k) for k in refrows if k not in used and not (k[4] == "nominal_section" and not need_nominal)]
        SF, SM = coupled(groups, Lb, "pub")
        SFr, SMr = coupled(groups, Lb, "ref")
        # F1: the scales max(published, reference), so that a vanishing publication is judged
        SFm, SMm = max(SF, SFr), max(SM, SMr)
        # C3 scales [K-D5]: S*_t = max(S_t, L_b S_r), S*_r = max(S_r, S_t / L_b) from the published
        # translation and rotation components (mm, rad; L_b in mm)
        sT = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] == "disp"] + [Q(0)])
        sR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] == "rot"] + [Q(0)])
        LbMM = Q(Lb) * 1000
        body_kd5 = {"D": max(sT, LbMM * sR) if Lb else sT, "R": max(sR, sT / LbMM) if Lb else sR}
        # C3lit (information): the separate published scales (translation including magnitude rows)
        bD = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "D"] + [Q(0)])
        bR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "R"] + [Q(0)])
        body_literal = {"D": bD, "R": bR}
        # C8 reference scales of the own-q families (the largest reference value of the family in the case)
        fam_ref = collections.defaultdict(lambda: Q(0))
        for k, g in groups.items():
            if g["fam"] in ("D", "R", "X"):
                fam_ref[g["fam"]] = max(fam_ref[g["fam"]], max(abs(x) for x in g["ref"]))
        stats = []
        unjudged_by_ref = []
        ref_nonzero = False
        for key, g in groups.items():
            q2 = sum(x * x for x in g["pub"])
            t2 = sum(x * x for x in g["ref"])
            e2 = sum((a - b) ** 2 for a, b in zip(g["pub"], g["ref"]))
            conv, s = group_scale(key, g["fam"], sections, SF, SM, q2)
            judged = group_judged(q2, t2, conv, s)
            judged_ref_only = False
            if not judged and g["fam"] not in ("D", "R"):
                # F1: judged on the scales max(published, reference) (max(q, t) for a Lame group)
                _, sm = group_scale(key, g["fam"], sections, SFm, SMm, max(q2, t2))
                judged = judged_ref_only = group_judged(q2, t2, conv, sm)
            breach = group_breach(e2, q2)
            ratio = qratio(e2, CRIT ** 2 * q2)
            st = {"group": key[0], "entity": key[1], "loc": key[2], "basis": key[3], "n": len(g["pub"]), "q": math.sqrt(qf(q2)),
                  "err": math.sqrt(qf(e2)), "ratio": ratio, "judged": judged, "breach": breach, "judged_ref_only": judged_ref_only}
            if g["fam"] in body_kd5:
                for name, scales in (("kd5", body_kd5), ("literal", body_literal)):
                    sc = scales[g["fam"]]
                    st["beyond_" + name] = e2 > CRIT ** 2 * sc * sc
                    st["ratio_" + name] = qratio(e2, CRIT ** 2 * sc * sc)
            # C8: would the reference itself judge this group (scales from reference values, floor margin 2)?
            if t2 > 0:
                ref_nonzero = True
                if g["fam"] in ("D", "R", "X"):
                    sr = fam_ref[g["fam"]]  # own-q families: noise-level truths (below 2 FLOOR of the family) are not required
                else:
                    _, sr = group_scale(key, g["fam"], sections, SFr, SMr, t2)
                if sr >= GUARD and t2 * conv ** 2 >= 4 * FLOOR ** 2 * sr ** 2 and not judged:
                    unjudged_by_ref.append([key[0], key[1], key[2], key[3]])
            stats.append(st)
        # coverage of the circular maxima
        evidence = {}
        for item in extract_items(run.get("contract_evidence")):
            if "value_upper_pa" in item and "pipe_id" in item:
                evidence[(item["pipe_id"], item.get("section_basis"))] = item
        maxima_ref = {(p, b): fq(v) for p, b, v in cref["maxima"]}
        mchecks = []
        seen_max = set()
        for row, basis in maxrows:
            pid = row["entity_ref"]
            lb = basis or "reduced_section"
            seen_max.add((pid, lb))
            ev = evidence.get((pid, basis))  # same basis only; no cross-basis fallback
            upper_value = ev["value_upper_pa"] if ev else row["value"]
            finite_upper = finite_value(upper_value)  # a null or non-finite upper covers nothing (C2), no exception
            upper = Q(upper_value) if finite_upper else Q(0)
            lower = Q(ev["value_lower_pa"]) if ev and finite_value(ev.get("value_lower_pa")) else None
            comps = []
            for (p, b, loc), v in station.items():
                if p != pid or b != lb:
                    continue
                axial = v.get("pipe_axial_membrane_stress_v2", v.get("element_local_axial_normal_stress", Q(0)))
                comps.append((axial, v.get("element_local_bending_normal_stress_y", Q(0)), v.get("element_local_bending_normal_stress_z", Q(0))))
            covered = finite_upper and all(circular_public_sample_covered(upper, *c) for c in comps)
            truth = maxima_ref.get((pid, lb))
            ext = None
            if truth is not None and truth > 0:
                gap = max(Q(0), truth - upper, (lower - truth) if lower is not None else Q(0))
                ext = qf(gap / (CRIT * truth)) if finite_upper else math.inf
            mchecks.append({"entity": pid, "basis": basis, "value": row["value"], "upper": qf(upper) if finite_upper else None, "n_samples": len(comps),
                            "covers_published_stations": covered, "evidence": ev is not None, "ref_outside_enclosure_over_crit": ext})
        wanted_max = {(p, b) for p, b in maxima_ref if producer == "u6" or b == "reduced_section"}
        missing_max = sorted(wanted_max - seen_max)
        summ = run.get("summary") or {}
        head = []
        if maxrows:
            if producer == "u6":
                if summ.get("max_open_formula_stress") is not None:
                    head.append("max_open_formula_stress is not null on the U6 producer")
                for b in BASES:
                    vals = [r["value"] for r, rb in maxrows if rb == b]
                    h = summ.get("max_open_formula_stress_" + b)
                    if not vals:
                        head.append(f"no {b} maximum rows")
                    elif not isinstance(h, dict) or h.get("value") != max(vals):
                        head.append(f"{b} headline {h and h.get('value')} != max {max(vals)}")
            else:
                h = summ.get("max_open_formula_stress")
                vals = [r["value"] for r, _ in maxrows]
                if not isinstance(h, dict) or h.get("value") != max(vals):
                    head.append(f"headline {h and h.get('value')} != max {max(vals)}")
        n_judged = sum(g["judged"] for g in stats)
        rec["cases"][cid] = {"headline_findings": head, "n_groups": len(stats), "n_judged": n_judged, "skips": dict(skips),
                             "unmapped": unmapped[:20], "n_unmapped": len(unmapped), "missing": missing[:20], "n_missing": len(missing),
                             "basis_free": basis_free[:10], "n_basis_free": len(basis_free), "unexpected_basis": unexpected_basis[:10],
                             "n_unexpected_basis": len(unexpected_basis), "missing_maxima": [list(x) for x in missing_max],
                             "unjudged_by_ref": unjudged_by_ref[:10], "n_unjudged_by_ref": len(unjudged_by_ref),
                             "nonfinite": nonfinite[:10], "n_nonfinite": len(nonfinite),
                             "n_judged_ref_only": sum(g["judged_ref_only"] for g in stats),
                             "max_checks": mchecks, "SF": qf(SF), "SM": qf(SM), "SF_ref": qf(SFr), "SM_ref": qf(SMr),
                             "body_D": qf(bD), "body_R": qf(bR),
                             "S_t": qf(sT), "S_r": qf(sR), "Sstar_t": qf(body_kd5["D"]), "Sstar_r": qf(body_kd5["R"])}
        # acceptance coverage (C6, C8) and information clauses for this case
        cs = rec["cases"][cid]
        unpublished = cs["n_missing"] + len(missing_max)
        info["C6i"] += cs["n_unmapped"] + cs["n_basis_free"] + cs["n_unexpected_basis"]
        info["C7"] += sum(1 for c in mchecks if not c["evidence"]) + len(head)
        if passed:
            fails["C1"] += cs["n_nonfinite"]
            fails["C6"] += unpublished
            fails["C8"] += len(unjudged_by_ref) + (1 if ref_nonzero and n_judged == 0 else 0)
            info["C7"] += sum(1 for c in mchecks if (c["ref_outside_enclosure_over_crit"] or 0) > 1)
        else:
            info["C6i"] += unpublished + cs["n_nonfinite"]
        for g in stats:
            g["case"] = cid
        all_groups += stats
    # published rows of a case the reference does not have are unmapped (information)
    stray = sum(len(rs) for c, rs in rows_by_case.items() if c not in ref["cases"])
    rec["n_stray_case_rows"] = stray
    info["C6i"] += stray
    judged = [g for g in all_groups if g["judged"]]
    rec["_groups"] = all_groups
    rec["n_groups"] = len(all_groups)
    rec["n_judged"] = len(judged)
    rec["n_judged_ref_only"] = sum(g["judged_ref_only"] for g in all_groups)
    rec["judged_by_group"] = dict(collections.Counter(g["group"] for g in judged))
    rec["breaches"] = [g for g in judged if g["breach"] and g["group"] not in NONACT]  # C1: Lame groups included (F4)
    rec["lame_breaches"] = [g for g in rec["breaches"] if g["group"] == "lame"]  # the Lame part of C1, for reporting
    rec["n_nonfinite"] = sum(cs["n_nonfinite"] for cs in rec["cases"].values())
    rec["disp_breaches"] = [g for g in judged if g["breach"] and g["group"] in NONACT]
    rec["disp_beyond_kd5"] = [g for g in all_groups if g["group"] in NONACT and g.get("beyond_kd5")]
    rec["disp_beyond_literal"] = [g for g in all_groups if g["group"] in NONACT and g.get("beyond_literal")]
    worst = {}
    for g in judged:
        if g["group"] not in worst or g["ratio"] > worst[g["group"]]["ratio"]:
            worst[g["group"]] = {k: g[k] for k in ("ratio", "entity", "loc", "basis", "q", "err", "case")}
    rec["worst_by_group"] = worst
    wb = [g for g in all_groups if g["group"] in NONACT]
    rec["worst_disp_kd5"] = max((g["ratio_kd5"] for g in wb), default=0.0)
    rec["worst_disp_literal"] = max((g["ratio_literal"] for g in wb), default=0.0)
    rec["max_fail"] = [c for cs in rec["cases"].values() for c in cs["max_checks"] if not c["covers_published_stations"]]
    rec["n_max"] = sum(len(cs["max_checks"]) for cs in rec["cases"].values())
    rec["n_unmapped"] = sum(cs["n_unmapped"] for cs in rec["cases"].values())
    rec["n_missing"] = sum(cs["n_missing"] for cs in rec["cases"].values())
    fails["C2"] += len(rec["max_fail"])
    if passed:
        fails["C1"] += len(rec["breaches"])
        fails["C3"] += len(rec["disp_beyond_kd5"])
        info["C3lit"] += len(rec["disp_beyond_literal"])
    rec["verdict_failures"] = {k: v for k, v in fails.items() if v}
    rec["info"] = {k: v for k, v in info.items() if v}
    return rec


def row_fingerprint(run):
    return [(r["kind"], r["entity_ref"], r.get("case"), r.get("location"), r.get("component"), (r.get("metadata") or {}).get("basis"), repr(r["value"]))
            for r in run.get("results") or []]


def work(job, producer="u6"):
    gen, ref, runs = job[:3]
    if len(job) > 3:
        producer = job[3]
    out = []
    by = {(r["entry"], r["mode"]): r for r in runs}
    judged = {}
    for run in runs:
        try:
            judged[(run["entry"], run["mode"])] = judge_run(gen, ref, run, producer)
        except Exception:
            import traceback
            judged[(run["entry"], run["mode"])] = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"], "reference_error": traceback.format_exc(),
                                                   "verdict_failures": {"C0": 1}}
    for (entry, mode), rec in judged.items():
        if entry == "captured":
            other, orec = by.get(("typed", mode)), judged.get(("typed", mode))
            if other is not None and orec is not None and "reference_error" not in orec and "reference_error" not in rec:
                run = by[(entry, mode)]
                rec["typed_captured_rows_identical"] = row_fingerprint(other) == row_fingerprint(run)
                a = [d["code"] for d in other.get("diagnostics") or []]
                b = [d["code"] for d in run.get("diagnostics") or []]
                rec["captured_extra_codes"] = sorted(set(b) - set(a))
                rec["typed_extra_codes"] = sorted(set(a) - set(b))
                rec["typed_captured_disagree"] = (orec["integrity"] != rec.get("integrity") or len(orec.get("breaches", [])) != len(rec.get("breaches", [])))
                vf = rec["verdict_failures"]
                vf["C4"] = vf.get("C4", 0) + int(rec["typed_captured_disagree"])
                inf = rec.setdefault("info", {})
                inf["C4b"] = inf.get("C4b", 0) + int(not rec["typed_captured_rows_identical"])
        rec.pop("_groups", None)
        rec["meta"] = {k: gen["meta"].get(k) for k in ("family", "theory", "placement", "mill_and_allowance", "section_variant", "harness_validation", "design", "load_state", "twin")}
        rec["verdict_failures"] = {k: v for k, v in rec["verdict_failures"].items() if v}
        rec["info"] = {k: v for k, v in rec.get("info", {}).items() if v}
        rec["verdict"] = "fail" if rec["verdict_failures"] else "pass"
        out.append(rec)
    return out


# ----------------------------------------------------------------- self-test
def synthetic_run(gen, suffixed, upper_scale=1.0 + 1e-12, entry="typed"):
    """A run record that publishes the reference rows themselves (rounded to binary64), Passed."""
    ref = gen["ref"]
    rows = []
    evidence = []
    for cid, c in ref["cases"].items():
        for kind, ent, loc, comp, basis, val in c["rows"]:
            if basis == "nominal_section" and not suffixed:
                continue
            token = "recovered_from_open_mechanics_stress_components"
            if basis and suffixed:
                token += "; section_basis=" + basis
            rows.append({"kind": kind, "value": float(Decimal(val)), "entity_ref": ent, "case": cid, "location": loc, "component": comp,
                         "metadata": {"basis": token}})
        for pid, basis, val in c["maxima"]:
            if basis == "nominal_section" and not suffixed:
                continue
            token = "recovered_from_open_mechanics_stress_components" + ("; section_basis=" + basis if suffixed else "")
            rows.append({"kind": "pipe_elastic_normal_stress_maximum_v2", "value": float(Decimal(val)), "entity_ref": pid, "case": cid,
                         "location": "governing_station", "component": "maximum_absolute_normal_stress", "metadata": {"basis": token}})
            e = {"pipe_id": pid, "value_upper_pa": float(Decimal(val)) * upper_scale, "value_lower_pa": float(Decimal(val)) / upper_scale}
            if suffixed:
                e["section_basis"] = basis
            evidence.append(e)
    summary = {}
    mx = [r for r in rows if r["kind"] == "pipe_elastic_normal_stress_maximum_v2"]
    if suffixed:
        summary["max_open_formula_stress"] = None
        for b in BASES:
            v = [r["value"] for r in mx if row_basis(r) == b]
            summary["max_open_formula_stress_" + b] = {"value": max(v), "unit": "Pa"} if v else None
    elif mx:
        summary["max_open_formula_stress"] = {"value": max(r["value"] for r in mx), "unit": "Pa"}
    return {"tag": gen["tag"], "entry": entry, "mode": "SparseInteractive", "results": rows, "summary": summary,
            "diagnostics": [{"code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "synthetic"}],
            "contract_evidence": {"exact_cases": [{"load_case_id": "case:1", "pipe_stress_extrema": evidence}]}}


def _gkey_of(row):
    k, _ = group_of(row, row_basis(row) or ("reduced_section" if row["kind"] in STRESS_KINDS else None))
    return k


def _all_groups(gen, run, producer):
    return judge_run(gen, gen["ref"], run, producer)["_groups"]


def selftest(set_path):
    """Exercise every rule and verdict clause on synthetic runs built from the reference itself."""
    import copy
    gens = [json.loads(l) for l in open(set_path)]
    out = []

    def check(name, cond):
        out.append((name, bool(cond)))
        print(("PASS " if cond else "FAIL ") + name)

    # predicates, exactly at their boundaries
    check("CRIT is RD(1e-9) = 0x1.12e0be826d694p-30, below 1e-9 and within one ulp", CRIT <= Q(1, 10 ** 9) < Q(math.nextafter(float(CRIT), 1.0)))
    q2 = Q(7, 3)
    check("breach is strict: e^2 = CRIT^2 q^2 does not breach", not group_breach(CRIT ** 2 * q2, q2))
    check("breach: e^2 one part in 2^60 above CRIT^2 q^2 breaches", group_breach(CRIT ** 2 * q2 * (1 + Q(1, 2 ** 60)), q2))
    s, conv = Q(5), Q(3)
    t2 = FLOOR ** 2 * s ** 2 / conv ** 2
    check("floor is inclusive: max(q^2,t^2) conv^2 = FLOOR^2 S^2 is judged", group_judged(Q(0), t2, conv, s))
    check("floor: just below FLOOR^2 S^2 is not judged", not group_judged(t2 * (1 - Q(1, 2 ** 60)), Q(0), conv, s))
    check("floor uses max(q^2, t^2): a published zero of a judged truth is judged", group_judged(Q(0), t2 * 4, conv, s))
    check("guard: S = 2^-988 is judged", group_judged(Q(1), Q(1), Q(1), GUARD))
    check("guard: S below 2^-988 is not judged", not group_judged(Q(1), Q(1), Q(1), GUARD * (1 - Q(1, 2 ** 60))))

    def bent(g):
        return any(r[0].startswith("element_local_bending_moment") and abs(float(Decimal(r[5]))) > 1.0 for r in g["ref"]["cases"]["case:1"]["rows"])

    plain = [g for g in gens if g["meta"]["harness_validation"] and g["meta"]["family"] in ("fixed_fixed", "l_line", "multi_span", "cantilever") and bent(g)][:4]
    press = [g for g in gens if g["meta"]["harness_validation"] and g["meta"]["family"] == "pressure" and bent(g)][:2]
    mills = [g for g in gens if g["meta"].get("section_variant", "both" if g["meta"]["mill_and_allowance"] else "zero") == "both"
             and g["meta"]["family"] == "pressure" and bent(g)][:2]
    if not (plain and press and mills):
        raise SystemExit("self-test needs plain, pressure and mill pressure models in the set")
    pinned_kinds = set()
    for g in plain + press + mills:
        for suffixed in (False, True):
            prod = "u6" if suffixed else "pre-u6"
            run = synthetic_run(g, suffixed)
            rec = judge_run(g, g["ref"], run, prod)
            check(f"{g['tag']} {prod}: the reference published as itself passes every clause and raises no information clause",
                  not rec["verdict_failures"] and not rec["info"] and rec["n_judged"] > 0 and rec["n_judged_ref_only"] == 0)
            # CRIT from both sides on the largest judged group of every group kind present
            byg = collections.defaultdict(list)
            for i, r in enumerate(run["results"]):
                if r["kind"] in SKIP:
                    continue
                byg[_gkey_of(r)].append(i)
            judged_set = {(x["group"], x["entity"], x["loc"], x["basis"]) for x in _all_groups(g, run, prod) if x["judged"]}
            kinds_done = set()
            for gk in sorted(byg, key=lambda k: -sum(run["results"][i]["value"] ** 2 for i in byg[k])):
                if gk[0] in kinds_done or gk not in judged_set:
                    continue
                idx = byg[gk]
                qn = math.sqrt(sum(run["results"][i]["value"] ** 2 for i in idx))
                kinds_done.add(gk[0])
                big = max(idx, key=lambda i: abs(run["results"][i]["value"]))
                for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                    r2 = copy.deepcopy(run)
                    v = r2["results"][big]["value"]
                    r2["results"][big]["value"] = v + math.copysign(factor * float(CRIT) * qn, v)
                    rec = judge_run(g, g["ref"], r2, prod)
                    hit = any(b["group"] == gk[0] and b["entity"] == gk[1] and b["loc"] == gk[2] and b["basis"] == gk[3]
                              for b in rec["breaches"] + rec["disp_breaches"] + rec["lame_breaches"])
                    check(f"{g['tag'].split('|')[1]} {prod}: {gk[0]} at {factor:.5f} CRIT {'breaches' if expect else 'does not breach'}", hit == expect)
            pinned_kinds.update(kinds_done)
            name = g["tag"].split("|")[1]
            # C3 pinned from both sides on each kind: a component error of (1 +- 1e-5) CRIT S*_kind
            cs = judge_run(g, g["ref"], run, prod)["cases"]["case:1"]
            for prefix, star in (("global_nodal_displacement", "Sstar_t"), ("global_nodal_rotation", "Sstar_r")):
                idx = [i for i, r in enumerate(run["results"]) if r["kind"].startswith(prefix)]
                i = min(idx, key=lambda i: abs(run["results"][i]["value"]))
                for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                    r2 = copy.deepcopy(run)
                    r2["results"][i]["value"] += factor * float(CRIT) * cs[star]
                    rec = judge_run(g, g["ref"], r2, prod)
                    check(f"{name} {prod}: {prefix} error {factor:.5f} CRIT S*_kind {'fails' if expect else 'passes'} C3",
                          (rec["verdict_failures"].get("C3", 0) > 0) == expect)
            # C3lit (information): rotations or translations wrong by 1e-6
            for prefix in ("global_nodal_rotation", "global_nodal_displacement"):
                r2 = copy.deepcopy(run)
                nonzero = any(r["kind"].startswith(prefix) and r["value"] != 0.0 for r in r2["results"])
                for r in r2["results"]:
                    if r["kind"].startswith(prefix):
                        r["value"] *= 1 + 1e-6
                rec = judge_run(g, g["ref"], r2, prod)
                if nonzero:
                    check(f"{name} {prod}: {prefix} x(1+1e-6) raises C3lit", rec["info"].get("C3lit", 0) > 0)
            # F1: every published row zero; and every force, moment, stress and reaction row zero while the
            # displacements are correct. Judged on the reference scales, so C1 fails (acceptance); no C0
            def is_disp(r):
                return r["kind"].startswith("global_nodal_") or r["kind"] == "displacement_magnitude"

            for keep_disp in (False, True):
                r2 = copy.deepcopy(run)
                for r in r2["results"]:
                    if not (keep_disp and is_disp(r)):
                        r["value"] = 0.0
                for e in r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]:
                    e["value_upper_pa"] = e["value_lower_pa"] = 0.0
                vf = work((g, g["ref"], [r2], prod))[0]["verdict_failures"]
                rec = judge_run(g, g["ref"], r2, prod)
                if keep_disp:
                    check(f"{name} {prod}: F1 zero actions, stresses and reactions with correct displacements fail C1 (reference scales)",
                          "C0" not in vf and vf.get("C1", 0) > 0 and rec["n_judged_ref_only"] > 0 and "C3" not in vf)
                else:
                    check(f"{name} {prod}: an all-zero publication fails C1 (reference scales) and C3",
                          "C0" not in vf and vf.get("C1", 0) > 0 and vf.get("C3", 0) > 0 and rec["n_judged_ref_only"] > 0)
            # F2: every action, stress and reaction row (and the evidence) scaled by 1e-300: no exception; every
            # breach is decided exactly and its ratio overflows to +inf
            r2 = copy.deepcopy(run)
            for r in r2["results"]:
                if not is_disp(r):
                    r["value"] *= 1e-300
            for e in r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]:
                e["value_upper_pa"] *= 1e-300
                e["value_lower_pa"] *= 1e-300
            res = work((g, g["ref"], [r2], prod))[0]
            overflowed = [x for x in judge_run(g, g["ref"], r2, prod)["_groups"] if x["judged"] and x["ratio"] == math.inf]
            check(f"{name} {prod}: F2 action rows x1e-300 fail C1 with +inf ratios and no exception (no C0)",
                  "C0" not in res["verdict_failures"] and res["verdict_failures"].get("C1", 0) > 0
                  and overflowed and all(x["breach"] for x in overflowed))
            # F2: a null (serde's non-finite) and an infinite published value, and a null evidence upper
            r2 = copy.deepcopy(run)
            kinds = [r["kind"] for r in r2["results"]]
            r2["results"][kinds.index("element_local_bending_moment_z")]["value"] = None
            r2["results"][kinds.index("element_local_shear_force_y")]["value"] = math.inf
            vf = work((g, g["ref"], [r2], prod))[0]["verdict_failures"]
            check(f"{name} {prod}: a null and an infinite published value fail C1 and C6, no exception",
                  "C0" not in vf and vf.get("C1", 0) == 2 and vf.get("C6", 0) == 2)
            r2 = copy.deepcopy(run)
            r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["value_upper_pa"] = None
            vf = work((g, g["ref"], [r2], prod))[0]["verdict_failures"]
            check(f"{name} {prod}: a null evidence upper fails C2, no exception", "C0" not in vf and vf.get("C2", 0) > 0)
            # F3: one member's interior station rows dropped: C6 (acceptance)
            r2 = copy.deepcopy(run)
            pid = next(r["entity_ref"] for r in r2["results"] if r.get("location") == "midspan")
            n0 = len(r2["results"])
            r2["results"] = [r for r in r2["results"] if not (r["entity_ref"] == pid and r.get("location") in ("quarter_1", "midspan", "quarter_3"))]
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: F3 one member's station rows dropped ({n0 - len(r2['results'])}) fail C6",
                  n0 > len(r2["results"]) and rec["verdict_failures"].get("C6", 0) == n0 - len(r2["results"]))
            # C8 (acceptance): a Passed run with no rows; one gross support moment that lifts the published scales
            r2 = copy.deepcopy(run)
            r2["results"] = []
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: a Passed run that publishes no rows fails C8", rec["verdict_failures"].get("C8", 0) == 1)
            r2 = copy.deepcopy(run)
            i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "support_reaction_component_v2" and r["component"] in ("Mx", "My", "Mz"))
            r2["results"][i]["value"] = 1e20
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: a gross support moment fails C1 and leaves groups the reference judges unjudged (C8)",
                  rec["verdict_failures"].get("C1", 0) > 0 and rec["verdict_failures"].get("C8", 0) > 0)
            # C2 coverage; C7 headline, evidence and enclosure (information)
            r2 = copy.deepcopy(run)
            r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["value_upper_pa"] *= 0.999
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: an upper bound 0.1% low fails C2 and raises C7", rec["verdict_failures"].get("C2", 0) > 0 and rec["info"].get("C7", 0) > 0)
            r2 = copy.deepcopy(run)
            key = "max_open_formula_stress_reduced_section" if suffixed else "max_open_formula_stress"
            r2["summary"][key] = {"value": r2["summary"][key]["value"] * 0.5, "unit": "Pa"}
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: a headline below its basis maximum raises C7", rec["info"].get("C7", 0) > 0)
            r2 = copy.deepcopy(run)
            r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"] = []
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: missing maximum evidence raises C7", rec["info"].get("C7", 0) > 0)
            # C6i unmapped (information) and C6 unpublished (acceptance)
            r2 = copy.deepcopy(run)
            r2["results"].append({"kind": "element_local_bending_moment_q", "value": 1.0, "entity_ref": "x", "case": "case:1", "location": "midspan", "component": "q", "metadata": {}})
            r2["results"].pop(5)
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: an unknown row raises C6i and a dropped row fails C6", rec["n_unmapped"] == 1 and rec["n_missing"] == 1
                  and rec["info"].get("C6i", 0) == 1 and rec["verdict_failures"].get("C6", 0) == 1)
            # producer mismatch: C6i; the single family judged as U6 also leaves the nominal family unpublished (C6)
            rec = judge_run(g, g["ref"], run, "pre-u6" if suffixed else "u6")
            check(f"{name} {prod}: judged as the other producer raises C6i" + ("" if suffixed else " and fails C6"),
                  rec["info"].get("C6i", 0) > 0 and (suffixed or rec["verdict_failures"].get("C6", 0) > 0))
            # C4 typed/captured; C4b (information)
            cap = copy.deepcopy(run)
            cap["entry"] = "captured"
            i = [r["kind"] for r in cap["results"]].index("element_local_bending_moment_z")
            cap["results"][i]["value"] = math.nextafter(cap["results"][i]["value"], math.inf)
            res = {r["entry"]: r for r in work((g, g["ref"], [run, cap], prod))}
            check(f"{name} {prod}: a captured row one ulp off raises C4b", res["captured"]["info"].get("C4b", 0) == 1)
            cap["diagnostics"] = [{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "severity": "warning", "message": "synthetic"}]
            res = {r["entry"]: r for r in work((g, g["ref"], [run, cap], prod))}
            check(f"{name} {prod}: a captured integrity code that differs fails C4", res["captured"]["verdict_failures"].get("C4", 0) == 1)
            # C5 (information): a refusal fails no acceptance clause
            r2 = copy.deepcopy(run)
            r2["diagnostics"] = [{"code": "EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED", "severity": "blocking", "message": "x"}]
            r2["results"] = []
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{name} {prod}: a refusal raises C5 and fails no acceptance clause", rec["info"].get("C5", 0) == 1 and not rec["verdict_failures"])
    # C3 in a body with an exactly zero rotation field (the FF00 case): rotation noise against S*_r passes C3,
    # and fails the separate-scale C3lit
    zero_rot = [g for g in gens if g["meta"]["harness_validation"]
                and all(Decimal(r[5]) == 0 for r in g["ref"]["cases"]["case:1"]["rows"] if r[0].startswith("global_nodal_rotation"))
                and any(Decimal(r[5]) != 0 for r in g["ref"]["cases"]["case:1"]["rows"] if r[0].startswith("global_nodal_displacement"))][:2]
    if not zero_rot:
        raise SystemExit("self-test needs a model with an exactly zero rotation field")
    for g in zero_rot:
        run = synthetic_run(g, True)
        r2 = copy.deepcopy(run)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "global_nodal_rotation_z")
        r2["results"][i]["value"] = 1.15e-15
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: rotation noise 1.15e-15 in an exactly zero rotation field passes C3 and raises C3lit",
              not rec["verdict_failures"] and rec["info"].get("C3lit", 0) > 0)
    # No displacement-noise floor: a body whose true translations and rotations are all zero
    g = copy.deepcopy(plain[0])
    for r in g["ref"]["cases"]["case:1"]["rows"]:
        if r[0].startswith("global_nodal_") or r[0] == "displacement_magnitude":
            r[5] = "0"
    run = synthetic_run(g, True)
    rec = judge_run(g, g["ref"], run, "u6")
    check(f"{g['tag'].split('|')[1]}: an all-zero displacement field published as exact zeros passes C3", rec["verdict_failures"].get("C3", 0) == 0)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"].startswith("global_nodal_"):
            r["value"] = 1e-15
    rec = judge_run(g, g["ref"], r2, "u6")
    check(f"{g['tag'].split('|')[1]}: an all-zero displacement field published as noise fails C3 (no noise floor)", rec["verdict_failures"].get("C3", 0) > 0)
    # one body per model
    g = copy.deepcopy(plain[0])
    g["request"]["model"]["nodes"].append({"id": "node:island", "position": {"x": 0.0, "y": 0.0, "z": 0.0}})
    res = work((g, g["ref"], [synthetic_run(g, True)], "u6"))
    check("a model with a second body is a harness error (C0)", res[0]["verdict_failures"].get("C0", 0) == 1)
    for g in mills:
        run = synthetic_run(g, True)
        # nominal and reduced bending stresses swapped
        r2 = copy.deepcopy(run)
        sw = {}
        for r in r2["results"]:
            b = row_basis(r)
            if b and r["kind"] == "element_local_bending_normal_stress_z":
                sw[(r["entity_ref"], r["location"], b)] = r
        for (ent, loc, b), r in list(sw.items()):
            if b == "nominal_section":
                o = sw[(ent, loc, "reduced_section")]
                r["value"], o["value"] = o["value"], r["value"]
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: swapping nominal and reduced bending stresses fails C1 (s_bending)", any(b["group"] == "s_bending" for b in rec["breaches"]))
        # the D-5 mutant: nominal Lame hoop evaluated at the pressure bore
        r2 = copy.deepcopy(run)
        p = abs(float(next(Decimal(r[5]) for r in g["ref"]["cases"]["case:1"]["rows"] if r[3] == "lame_inner_radial_stress")))
        for r in r2["results"]:
            if r["kind"] == "pipe_lame_hoop_stress_v2" and row_basis(r) == "nominal_section":
                s = g["ref"]["sections"][r["entity_ref"]]
                outer = 2 * p * float(Decimal(s["pressure_bore"]["Ai"])) / float(Decimal(s["nominal_section"]["A"]))
                r["value"] = outer if r["component"] == "lame_outer_hoop_stress" else outer + p
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: nominal Lame hoop at the pressure bore fails C1 (lame, nominal)",
              any(b["group"] == "lame" and b["basis"] == "nominal_section" for b in rec["breaches"]) and rec["verdict_failures"].get("C1", 0) > 0
              and all(b["group"] == "lame" for b in rec["breaches"]))
        # F4: reduced Lame rows copied from the nominal ones
        r2 = copy.deepcopy(run)
        nominal = {(r["kind"], r["entity_ref"], r["location"], r["component"]): r["value"] for r in r2["results"]
                   if r["kind"].startswith("pipe_lame_") and row_basis(r) == "nominal_section"}
        for r in r2["results"]:
            if r["kind"].startswith("pipe_lame_") and row_basis(r) == "reduced_section":
                r["value"] = nominal[(r["kind"], r["entity_ref"], r["location"], r["component"])]
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: F4 reduced Lame rows copied from nominal fail C1 (lame, reduced)",
              any(b["group"] == "lame" and b["basis"] == "reduced_section" for b in rec["breaches"]) and rec["verdict_failures"].get("C1", 0) > 0)
        # a basis-free stress row, and the suffix-free single family, on the U6 producer
        r2 = copy.deepcopy(run)
        for r in r2["results"]:
            if r["kind"] == "element_local_torsional_shear_stress":
                r["metadata"]["basis"] = "recovered_from_open_mechanics_stress_components"
                break
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: one basis-free stress row on the U6 producer raises C6i", rec["info"].get("C6i", 0) > 0)
        rec = judge_run(g, g["ref"], synthetic_run(g, False), "u6")
        check(f"{g['tag'].split('|')[1]}: a suffix-free single family on the U6 producer raises C6i and fails C6",
              rec["info"].get("C6i", 0) > 0 and rec["verdict_failures"].get("C6", 0) > 0)
        # reduced maximum bound to the nominal evidence
        r2 = copy.deepcopy(run)
        evs = r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]
        for e in evs:
            if e["section_basis"] == "reduced_section":
                e["section_basis"] = "nominal_section_x"
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: reduced maxima without reduced evidence raise C7", rec["info"].get("C7", 0) > 0)
    every = {"disp", "rot", "dispmag", "reactF", "reactM", "reactFmag", "reactMmag", "axial", "shear", "torsion", "bending", "s_axial", "s_bending",
             "s_torsion", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2", "pipe_axial_membrane_stress_v2", "lame"}
    check(f"CRIT pinned from both sides on every group kind (missing: {sorted(every - pinned_kinds)})", every <= pinned_kinds)
    bad = [n for n, ok in out if not ok]
    print(f"compare self-test: {len(out) - len(bad)} of {len(out)} checks pass")
    if bad:
        raise SystemExit(1)


def main():
    args = sys.argv[1:]
    if args and args[0] == "--selftest":
        selftest(args[1])
        return
    variant, workers, producer = "own", 6, "u6"
    for flag in ("--ref", "--workers", "--producer"):
        if flag in args:
            i = args.index(flag)
            val = args[i + 1]
            del args[i:i + 2]
            if flag == "--ref":
                variant = val
            elif flag == "--workers":
                workers = int(val)
            else:
                producer = val
    if producer not in ("u6", "pre-u6"):
        raise SystemExit("--producer u6|pre-u6")
    set_path, runs_path, out_path = args
    gens = {}
    for line in open(set_path):
        x = json.loads(line)
        gens[x["tag"]] = x
    controls = {}
    if variant.startswith("control:"):
        _, cpath, ckey = variant.split(":", 2)
        for line in open(cpath):
            x = json.loads(line)
            controls[x["tag"]] = x[ckey]
    runs = collections.defaultdict(list)
    for line in open(runs_path):
        x = json.loads(line)
        runs[x["tag"]].append(x)
    jobs = []
    for tag, rs in runs.items():
        gen = gens[tag]
        if variant == "own":
            ref = gen["ref"]
        elif variant == "twin":
            ref = gens[gen["meta"]["twin"]]["ref"]
        else:
            if tag not in controls:
                continue
            ref = controls[tag]
        jobs.append((gen, ref, rs, producer))
    import multiprocessing as mp
    with open(out_path, "w") as f:
        if workers <= 1:
            results = map(work, jobs)
        else:
            pool = mp.get_context("fork").Pool(workers)
            results = pool.imap_unordered(work, jobs, chunksize=2)
        for result in results:
            for rec in result:
                rec["ref_variant"] = variant.split(":")[0] + (":" + variant.split(":")[-1] if variant.startswith("control:") else "")
                f.write(json.dumps(rec) + "\n")


if __name__ == "__main__":
    main()
