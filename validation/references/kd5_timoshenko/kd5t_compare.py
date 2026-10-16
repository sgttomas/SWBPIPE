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
- Lame rows form a group per (pipe, location, basis) judged against its own q [own, stricter: K-D5
  does not list them; the D-5 "nominal Lame at the pressure bore" mutant needs them judged].
- conv to force units: s_axial A_b*1e6, s_bending Z_b*1e6, s_torsion J_b/r_o*1e6, membrane A_b,
  with the row's basis section [cc `conv`; per basis: Design §2.1].
- Coupled scales: SF = max(force, moment/L_b), SM = max(moment, L_b*force) over the published ACT
  groups, L_b the node bounding-box diagonal [K-D5 §3.3; cc]. Displacement and rotation groups use
  their own q [cc; K-D5 §4.1].
- SKIP: the solver-mode row [cc]. Maxima are checked by coverage, not as groups [cc].
- Coverage: U = evidence `value_upper_pa` of the same pipe and, on the U6 producer, the same
  `section_basis`; every published station sample of that pipe and basis satisfies (U - |axial|)^2
  >= My^2 + Mz^2 with U - |axial| >= 0, exactly [K-D5 §3.2, §4; cc
  `circular_public_sample_covered`]. The axial sample is the wall membrane stress where the member
  is pressurized [K-D5 §3.2 "the exact-pressure route uses wall axial stress"; own, stricter: cc
  used axial 0 there].

The M3 verdict (a run fails on any clause; the set passes when no run fails)
- C1 [K-D5 §4.3]: 0 judged member, section, support or stress breaches on Passed cases (Lame
  included, see above).
- C2 [K-D5 §4.3]: 0 strict coverage failures, on every run with results.
- C3 [K-D5 §4.3, literally]: on Passed cases every displacement and rotation group is within
  RD(1e-9) of its body's published scale: translation groups against the largest published
  translation (component or magnitude), rotation groups against the largest published rotation.
  The scale coupled through L_b is reported for information only [own].
- C4 [K-D5 §4.3; b4]: 0 typed/captured disagreements (integrity codes or breach counts); and
  identical published rows [own, stricter].
- C5 [own, stricter]: admission: no refusal, DTO or runner error.
- C6 [own, stricter; Design §§2.1, 2.5 for both families]: 0 unmapped product rows, 0 unpublished
  reference rows; on the U6 producer 0 stress rows without a basis and both maxima per pipe; on
  today's producer 0 basis-suffixed rows.
- C7 [own, stricter; Design §§2.2, 2.5 for the headlines]: every maximum binds same-basis evidence;
  on Passed cases the evidence enclosure contains the exact maximum to within CRIT of it; the
  headline(s) equal the maximum of their basis (and the single headline is null on U6).
- C8 [own, stricter]: on Passed cases, no vacuous pass: every group that the reference itself
  would judge with a factor-2 margin on the floor (scales formed from the reference values) is
  judged, and a body with a nonzero reference has judged groups.
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
CLAUSES = {
    "C0": "reference or harness error",
    "C1": "judged member/section/support/stress breach on a Passed case",
    "C2": "strict coverage failure",
    "C3": "displacement/rotation group beyond 1e-9 of the body's published scale (Passed)",
    "C4": "typed/captured disagreement (integrity codes or breach counts)",
    "C4b": "typed/captured published rows differ",
    "C5": "refused, DTO or runner error",
    "C6": "unmapped, unpublished, basis-free or unexpected-basis rows",
    "C7": "maximum evidence missing, exact maximum outside the enclosure, or headline finding",
    "C8": "vacuous pass: a group the reference judges is not judged",
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
        s = Q(math.sqrt(float(q2)))
    return conv, s


def coupled(groups, Lb, which):
    force = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "F"] + [Q(0)])
    moment = max([max(abs(x) for x in g[which]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "M"] + [Q(0)])
    LbQ = Q(Lb)
    SF = max(force, moment / LbQ) if Lb else force
    SM = max(moment, LbQ * force) if Lb else moment
    return SF, SM


def judge_run(gen, ref, run, producer="u6"):
    rec = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"], "producer": producer}
    fails = collections.Counter()
    rec["verdict_failures"] = fails
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
        rec["verdict_failures"] = {"C5": 1}
        return rec
    if rec["blocking"] or not run.get("results"):
        rec["verdict_failures"] = {"C5": 1}
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
        basis_free, unexpected_basis = [], []
        station = collections.defaultdict(dict)
        maxrows = []
        for row in rows_by_case.get(cid, []):
            kind = row["kind"]
            basis = row_basis(row)
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
        # C3 scales: the body's published translation and rotation scales [K-D5 §4.3], and the coupled form (information)
        bD = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "D"] + [Q(0)])
        bR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "R"] + [Q(0)])
        body = {"D": bD, "R": bR}
        body_coupled = {"D": max(bD, bR * Q(Lb) * 1000) if Lb else bD, "R": max(bR, bD / (Q(Lb) * 1000)) if Lb else bR}
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
            breach = group_breach(e2, q2)
            ratio = math.sqrt(float(e2 / (CRIT ** 2 * q2))) if q2 else (math.inf if e2 else 0.0)
            st = {"group": key[0], "entity": key[1], "loc": key[2], "basis": key[3], "n": len(g["pub"]), "q": math.sqrt(float(q2)),
                  "err": math.sqrt(float(e2)), "ratio": ratio, "judged": judged, "breach": breach}
            if g["fam"] in body:
                sc = body[g["fam"]]
                st["beyond_body"] = e2 > CRIT ** 2 * sc * sc
                st["ratio_body"] = (math.sqrt(float(e2)) / (float(CRIT) * float(sc))) if sc else (math.inf if e2 else 0.0)
                scc = body_coupled[g["fam"]]
                st["ratio_body_coupled"] = (math.sqrt(float(e2)) / (float(CRIT) * float(scc))) if scc else (math.inf if e2 else 0.0)
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
            upper = Q(ev["value_upper_pa"]) if ev else Q(row["value"])
            lower = Q(ev["value_lower_pa"]) if ev and "value_lower_pa" in ev else None
            comps = []
            for (p, b, loc), v in station.items():
                if p != pid or b != lb:
                    continue
                axial = v.get("pipe_axial_membrane_stress_v2", v.get("element_local_axial_normal_stress", Q(0)))
                comps.append((axial, v.get("element_local_bending_normal_stress_y", Q(0)), v.get("element_local_bending_normal_stress_z", Q(0))))
            covered = all(circular_public_sample_covered(upper, *c) for c in comps)
            truth = maxima_ref.get((pid, lb))
            ext = None
            if truth is not None and truth > 0:
                gap = max(Q(0), truth - upper, (lower - truth) if lower is not None else Q(0))
                ext = float(gap / (CRIT * truth))
            mchecks.append({"entity": pid, "basis": basis, "value": row["value"], "upper": float(upper), "n_samples": len(comps),
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
                             "max_checks": mchecks, "SF": float(SF), "SM": float(SM), "body_D": float(bD), "body_R": float(bR)}
        # verdict clauses for this case
        cs = rec["cases"][cid]
        fails["C6"] += cs["n_unmapped"] + cs["n_missing"] + cs["n_basis_free"] + cs["n_unexpected_basis"] + len(missing_max)
        fails["C7"] += sum(1 for c in mchecks if not c["evidence"]) + len(head)
        if passed:
            fails["C7"] += sum(1 for c in mchecks if (c["ref_outside_enclosure_over_crit"] or 0) > 1)
            fails["C8"] += len(unjudged_by_ref) + (1 if ref_nonzero and n_judged == 0 else 0)
        for g in stats:
            g["case"] = cid
        all_groups += stats
    judged = [g for g in all_groups if g["judged"]]
    rec["_groups"] = all_groups
    rec["n_groups"] = len(all_groups)
    rec["n_judged"] = len(judged)
    rec["judged_by_group"] = dict(collections.Counter(g["group"] for g in judged))
    rec["breaches"] = [g for g in judged if g["breach"] and g["group"] not in NONACT]
    rec["disp_breaches"] = [g for g in judged if g["breach"] and g["group"] in NONACT]
    rec["disp_beyond_body"] = [g for g in all_groups if g["group"] in NONACT and g.get("beyond_body")]
    worst = {}
    for g in judged:
        if g["group"] not in worst or g["ratio"] > worst[g["group"]]["ratio"]:
            worst[g["group"]] = {k: g[k] for k in ("ratio", "entity", "loc", "basis", "q", "err", "case")}
    rec["worst_by_group"] = worst
    wb = [g for g in all_groups if g["group"] in NONACT]
    rec["worst_disp_body"] = max((g["ratio_body"] for g in wb), default=0.0)
    rec["worst_disp_body_coupled"] = max((g["ratio_body_coupled"] for g in wb), default=0.0)
    rec["max_fail"] = [c for cs in rec["cases"].values() for c in cs["max_checks"] if not c["covers_published_stations"]]
    rec["n_max"] = sum(len(cs["max_checks"]) for cs in rec["cases"].values())
    rec["n_unmapped"] = sum(cs["n_unmapped"] for cs in rec["cases"].values())
    rec["n_missing"] = sum(cs["n_missing"] for cs in rec["cases"].values())
    fails["C2"] += len(rec["max_fail"])
    if passed:
        fails["C1"] += len(rec["breaches"])
        fails["C3"] += len(rec["disp_beyond_body"])
    rec["verdict_failures"] = {k: v for k, v in fails.items() if v}
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
                vf["C4b"] = vf.get("C4b", 0) + int(not rec["typed_captured_rows_identical"])
        rec.pop("_groups", None)
        rec["meta"] = {k: gen["meta"].get(k) for k in ("family", "theory", "placement", "mill_and_allowance", "section_variant", "harness_validation", "design", "load_state", "twin")}
        rec["verdict_failures"] = {k: v for k, v in rec["verdict_failures"].items() if v}
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
            check(f"{g['tag']} {prod}: the reference published as itself passes every clause", not rec["verdict_failures"] and rec["n_judged"] > 0)
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
                              for b in rec["breaches"] + rec["disp_breaches"])
                    check(f"{g['tag'].split('|')[1]} {prod}: {gk[0]} at {factor:.5f} CRIT {'breaches' if expect else 'does not breach'}", hit == expect)
            pinned_kinds.update(kinds_done)
            # C3: rotations or translations wrong by 1e-6
            for prefix in ("global_nodal_rotation", "global_nodal_displacement"):
                r2 = copy.deepcopy(run)
                nonzero = any(r["kind"].startswith(prefix) and r["value"] != 0.0 for r in r2["results"])
                for r in r2["results"]:
                    if r["kind"].startswith(prefix):
                        r["value"] *= 1 + 1e-6
                rec = judge_run(g, g["ref"], r2, prod)
                if nonzero:
                    check(f"{g['tag'].split('|')[1]} {prod}: {prefix} x(1+1e-6) fails C3", rec["verdict_failures"].get("C3", 0) > 0)
            # C8: every published row zero
            r2 = copy.deepcopy(run)
            for r in r2["results"]:
                r["value"] = 0.0
            for e in r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]:
                e["value_upper_pa"] = e["value_lower_pa"] = 0.0
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: an all-zero publication fails C8", rec["verdict_failures"].get("C8", 0) > 0)
            # C2 coverage, C7 headline, evidence and enclosure
            r2 = copy.deepcopy(run)
            r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]["value_upper_pa"] *= 0.999
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: an upper bound 0.1% low fails C2 and C7", rec["verdict_failures"].get("C2", 0) > 0 and rec["verdict_failures"].get("C7", 0) > 0)
            r2 = copy.deepcopy(run)
            key = "max_open_formula_stress_reduced_section" if suffixed else "max_open_formula_stress"
            r2["summary"][key] = {"value": r2["summary"][key]["value"] * 0.5, "unit": "Pa"}
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: a headline below its basis maximum fails C7", rec["verdict_failures"].get("C7", 0) > 0)
            r2 = copy.deepcopy(run)
            r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"] = []
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: missing maximum evidence fails C7", rec["verdict_failures"].get("C7", 0) > 0)
            # C6 unmapped and missing
            r2 = copy.deepcopy(run)
            r2["results"].append({"kind": "element_local_bending_moment_q", "value": 1.0, "entity_ref": "x", "case": "case:1", "location": "midspan", "component": "q", "metadata": {}})
            r2["results"].pop(5)
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: an unknown row and a dropped row fail C6", rec["n_unmapped"] == 1 and rec["n_missing"] == 1 and rec["verdict_failures"].get("C6", 0) == 2)
            # C6 producer mismatch
            rec = judge_run(g, g["ref"], run, "pre-u6" if suffixed else "u6")
            check(f"{g['tag'].split('|')[1]} {prod}: judged as the other producer fails C6", rec["verdict_failures"].get("C6", 0) > 0)
            # C4, C4b typed/captured
            cap = copy.deepcopy(run)
            cap["entry"] = "captured"
            i = [r["kind"] for r in cap["results"]].index("element_local_bending_moment_z")
            cap["results"][i]["value"] = math.nextafter(cap["results"][i]["value"], math.inf)
            res = {r["entry"]: r for r in work((g, g["ref"], [run, cap], prod))}
            check(f"{g['tag'].split('|')[1]} {prod}: a captured row one ulp off fails C4b", res["captured"]["verdict_failures"].get("C4b", 0) == 1)
            cap["diagnostics"] = [{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "severity": "warning", "message": "synthetic"}]
            res = {r["entry"]: r for r in work((g, g["ref"], [run, cap], prod))}
            check(f"{g['tag'].split('|')[1]} {prod}: a captured integrity code that differs fails C4", res["captured"]["verdict_failures"].get("C4", 0) == 1)
            # C5
            r2 = copy.deepcopy(run)
            r2["diagnostics"] = [{"code": "EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED", "severity": "blocking", "message": "x"}]
            r2["results"] = []
            rec = judge_run(g, g["ref"], r2, prod)
            check(f"{g['tag'].split('|')[1]} {prod}: a refusal fails C5", rec["verdict_failures"].get("C5", 0) == 1)
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
        check(f"{g['tag'].split('|')[1]}: nominal Lame hoop at the pressure bore fails C1 (lame)", any(b["group"] == "lame" and b["basis"] == "nominal_section" for b in rec["breaches"]))
        # a basis-free stress row, and the suffix-free single family, on the U6 producer
        r2 = copy.deepcopy(run)
        for r in r2["results"]:
            if r["kind"] == "element_local_torsional_shear_stress":
                r["metadata"]["basis"] = "recovered_from_open_mechanics_stress_components"
                break
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: one basis-free stress row on the U6 producer fails C6", rec["verdict_failures"].get("C6", 0) > 0)
        rec = judge_run(g, g["ref"], synthetic_run(g, False), "u6")
        check(f"{g['tag'].split('|')[1]}: a suffix-free single family on the U6 producer fails C6", rec["verdict_failures"].get("C6", 0) > 0)
        # reduced maximum bound to the nominal evidence
        r2 = copy.deepcopy(run)
        evs = r2["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]
        for e in evs:
            if e["section_basis"] == "reduced_section":
                e["section_basis"] = "nominal_section_x"
        rec = judge_run(g, g["ref"], r2, "u6")
        check(f"{g['tag'].split('|')[1]}: reduced maxima without reduced evidence fail C7", rec["verdict_failures"].get("C7", 0) > 0)
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
