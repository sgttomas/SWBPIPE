"""M3-T comparison: the product's published rows against the kd5t exact reference.

Reconstructs the judging and coverage logic of the surviving M3 harness,
`scratch/pr_kd5_eval_current/compare_current.py` (sha256 4b21fa99...), whose
imported modules (RV30's `compare.py` and I41's `ref_ext.py`) are lost. The
lost pieces are rebuilt from the K-D5 design
(`execution/PKG-04/DEL-04-01/Design/kd5-member-relative-bound.md`) and from
compare_current.py's own use of them; every rule names its source below. It
is basis-aware (section-bases Design sections 2.1 and 2.5): a stress row whose
`metadata.basis` ends in `; section_basis=nominal_section|reduced_section` is
judged against that basis's section; a stress row without the suffix (the
producer before T4-U6) is judged against the reduced section, which is the
single section wall - mill tolerance of that producer whenever it admits the
document (zero allowance). Exact rationals throughout; no product code.

Usage:
  python3 -I -B kd5t_compare.py <set.jsonl> <runs.jsonl> <out.jsonl> [--ref own|twin|control:<controls.jsonl>:<key>] [--workers N]

Rules and their sources
- CRIT = RD(1e-9), the largest binary64 <= 1e-9 (K-D5 section 3.3: "exceeds(B, q)
  is today's exact B > RD(1e-9)*q"); breach <=> sum (v - t)^2 > CRIT^2 * sum v^2
  over the group's published components v and reference values t
  (compare_current: `breach=e2>C.CRIT**2*q2`).
- FLOOR = 2^-34 and GUARD = 2^-988 (K-D5 section 3.3: fires only if
  S*_g >= 2^-988 and q_hat_g >= 2^-34*S*_g); judged <=> S >= GUARD and
  max(q^2, t^2)*conv^2 >= FLOOR^2*S^2 (compare_current: `judged=...`; its comment
  "Actual-truth floor complements published q").
- Groups (K-D5 section 3.2 "per location, four groups: |N|, hypot(V_y, V_z),
  |T| and hypot(M_y, M_z)"; pipe-wall rows "form their own groups"; stress rows
  "judged through the resultant they scale"; supports "two physical families
  per support: force and moment ... separately judges the published scalar
  magnitude"; displacement and rotation groups "judged against [their] own q",
  section 4.1). Group names are compare_current's (`ACT`, `s_axial`,
  `s_bending`, `s_torsion`, `pipe_axial_membrane_stress_v2*`, and the analysis
  scripts' `disp`, `rot`, `dispmag`, `rotmag`, `reactF`, `reactM`, `reactFmag`,
  `reactMmag`); the key is (group, entity, location, section basis).
- Conversions to force units for the floor: s_axial A_b*1e6, s_bending
  Z_b*1e6 (=I_b/r_o), s_torsion J_b/r_o*1e6, membrane A_b (compare_current's
  `conv`), with A_b, Z_b, J_b of the row's basis.
- Coupled scales (K-D5 section 3.3, "S*_g is the body's coupled scale ... the
  moment scale for bending, torsion and support moment groups, and the force
  scale for axial, shear and support force groups"): force = max |v| over F
  groups in ACT, moment likewise over M groups, L_b the node bounding-box
  diagonal, SF = max(force, moment/L_b), SM = max(moment, L_b*force)
  (compare_current, exactly). Stress groups use SF (axial, membrane) or SM
  (bending, torsion); displacement and rotation groups use their own q.
- SKIP: rows that scale no member action (the solver-mode record) are not
  judged; `pipe_elastic_normal_stress_maximum_v2` is checked by coverage, not as
  a group (compare_current). Lame rows depend only on p and the section; they
  are judged as an extension group `lame` against their own q and reported
  apart from the M3 criterion.
- Coverage (K-D5 section 3.2 "A strict consuming check asserts value_upper_pa
  is at least the exact-unit-converted absolute value of each published
  normal-stress station component of that member"; section 4 "a finite global
  upper U covers the sample exactly when U - |axial| >= 0 and (U - |axial|)^2 >=
  My^2 + Mz^2"; compare_current's `circular_public_sample_covered`). U is the
  evidence `value_upper_pa` of the same pipe (and, from U6, the same
  `section_basis`); the axial sample is the member's published wall membrane
  stress where the member is pressurized (K-D5 section 3.2: "the
  exact-pressure route uses wall axial stress") and its axial normal stress
  otherwise; the bending samples are the same basis's bending stresses. Every
  published station location counts. compare_current read only the
  element-local stresses (axial 0 when absent); this is the stricter form.
- M3 criterion (K-D5 section 4.3; section-bases Design section 4 item 5): 0
  judged member, section, support or stress breaches on Passed cases and 0
  strict coverage failures. Displacement and rotation groups are reported
  separately (K-D5 section 4.1 "comparison artefacts of judging displacement
  groups without a body-scale floor").
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
EXTENSION = {"lame"}


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


def circular_public_sample_covered(upper, axial, my, mz):
    difference = upper - abs(axial)
    return difference >= 0 and difference * difference >= my * my + mz * mz


def fq(text):
    return Q(Decimal(text))


def classify(codes, messages):
    """compare harness standing classes (kd5-eval-final/summarize.py)."""
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


def judge_run(gen, ref, run):
    rec = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"]}
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
    if "error" in run:
        rec["error"] = run["error"]
        return rec
    if rec["blocking"] or not run.get("results"):
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
        regions = set(cref["region_members"])
        groups, skips, unmapped, used = {}, collections.Counter(), [], set()
        station = collections.defaultdict(dict)
        maxrows = []
        for row in rows_by_case.get(cid, []):
            kind = row["kind"]
            basis = row_basis(row)
            if kind == "pipe_elastic_normal_stress_maximum_v2":
                maxrows.append((row, basis))
                continue
            if kind in SKIP:
                skips[kind] += 1
                continue
            lookup_basis = (basis or "reduced_section") if kind in STRESS_KINDS else None
            key = (kind, row["entity_ref"], row["location"], row["component"], lookup_basis)
            if key not in refrows or (kind not in GROUP_OF_KIND and not kind.startswith("support_reaction")):
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
        missing = [list(k) for k in refrows if k not in used and not (k[4] == "nominal_section" and not suffixed)]
        force = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "F"] + [Q(0)])
        moment = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if k[0] in ACT and g["fam"] == "M"] + [Q(0)])
        LbQ = Q(Lb)
        SF = max(force, moment / LbQ) if Lb else force
        SM = max(moment, LbQ * force) if Lb else moment
        # body displacement / rotation scales for reporting the displacement-group artefacts (K-D5 section 4.3:
        # "every one within 1e-9 of its body's published displacement or rotation scale")
        # The translation (mm) and rotation (rad) scales are coupled through L_b as the force and moment scales are.
        bD = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "D"] + [Q(0)])
        bR = max([max(abs(x) for x in g["pub"]) for k, g in groups.items() if g["fam"] == "R"] + [Q(0)])
        body = {"D": max(bD, bR * Q(Lb) * 1000) if Lb else bD, "R": max(bR, bD / (Q(Lb) * 1000)) if Lb else bR}
        stats = []
        for key, g in groups.items():
            q2 = sum(x * x for x in g["pub"])
            t2 = sum(x * x for x in g["ref"])
            e2 = sum((a - b) ** 2 for a, b in zip(g["pub"], g["ref"]))
            conv = Q(1)
            sec = sections.get(key[1], {}).get(key[3] or "reduced_section") if key[1] in sections else None
            if key[0] == "s_axial":
                conv, s = fq(sec["A"]) * MPA, SF
            elif key[0] == "s_bending":
                conv, s = fq(sec["Z"]) * MPA, SM
            elif key[0] == "s_torsion":
                conv, s = fq(sec["J"]) / fq(sections[key[1]]["ro"]) * MPA, SM
            elif key[0].startswith("pipe_axial_membrane_stress_v2"):
                conv, s = fq(sec["A"]), SF
            elif g["fam"] == "F":
                s = SF
            elif g["fam"] == "M":
                s = SM
            else:
                s = Q(math.sqrt(float(q2)))
            judged = s >= GUARD and max(q2, t2) * conv ** 2 >= FLOOR ** 2 * s ** 2
            breach = e2 > CRIT ** 2 * q2
            ratio = math.sqrt(float(e2 / (CRIT ** 2 * q2))) if q2 else (math.inf if e2 else 0.0)
            st = {"group": key[0], "entity": key[1], "loc": key[2], "basis": key[3], "n": len(g["pub"]), "q": math.sqrt(float(q2)),
                  "err": math.sqrt(float(e2)), "ratio": ratio, "judged": judged, "breach": breach}
            if g["fam"] in body and body[g["fam"]] > 0:
                st["ratio_body"] = math.sqrt(float(e2)) / (float(CRIT) * float(body[g["fam"]]))
            stats.append(st)
        # coverage of the circular maxima
        evidence = {}
        for item in extract_items(run.get("contract_evidence")):
            if "value_upper_pa" in item and "pipe_id" in item:
                evidence[(item["pipe_id"], item.get("section_basis"))] = item
        maxima_ref = {(p, b): fq(v) for p, b, v in cref["maxima"]}
        mchecks = []
        for row, basis in maxrows:
            pid = row["entity_ref"]
            lb = basis or "reduced_section"
            # a basis-named maximum binds only the evidence of its own basis; no cross-basis fallback
            ev = evidence.get((pid, basis))
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
        # extension: the headline(s) bind a maximum row of their own basis and are not below any maximum of it
        # (section-bases Design sections 2.2 and 2.5; one load case per model here)
        summ = run.get("summary") or {}
        head = []
        if maxrows:
            if suffixed:
                if summ.get("max_open_formula_stress") is not None:
                    head.append("max_open_formula_stress is not null on a basis-suffixed envelope")
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
        rec["cases"][cid] = {"headline_findings": head, "n_groups": len(stats), "n_judged": sum(g["judged"] for g in stats), "skips": dict(skips),
                             "unmapped": unmapped[:20], "n_unmapped": len(unmapped), "missing": missing[:20], "n_missing": len(missing),
                             "max_checks": mchecks, "SF": float(SF), "SM": float(SM)}
        for g in stats:
            g["case"] = cid
        all_groups += stats
    judged = [g for g in all_groups if g["judged"]]
    rec["n_groups"] = len(all_groups)
    rec["n_judged"] = len(judged)
    rec["judged_by_group"] = dict(collections.Counter(g["group"] for g in judged))
    rec["breaches"] = [g for g in judged if g["breach"] and g["group"] not in NONACT | EXTENSION]
    rec["disp_breaches"] = [g for g in judged if g["breach"] and g["group"] in NONACT]
    rec["ext_breaches"] = [g for g in judged if g["breach"] and g["group"] in EXTENSION]
    worst = {}
    for g in judged:
        if g["group"] not in worst or g["ratio"] > worst[g["group"]]["ratio"]:
            worst[g["group"]] = {k: g[k] for k in ("ratio", "entity", "loc", "basis", "q", "err", "case")}
    rec["worst_by_group"] = worst
    rec["max_fail"] = [c for cs in rec["cases"].values() for c in cs["max_checks"] if not c["covers_published_stations"]]
    rec["n_max"] = sum(len(cs["max_checks"]) for cs in rec["cases"].values())
    rec["n_unmapped"] = sum(cs["n_unmapped"] for cs in rec["cases"].values())
    rec["n_missing"] = sum(cs["n_missing"] for cs in rec["cases"].values())
    return rec


def row_fingerprint(run):
    return [(r["kind"], r["entity_ref"], r.get("case"), r.get("location"), r.get("component"), (r.get("metadata") or {}).get("basis"), repr(r["value"])) for r in run.get("results") or []]


def work(job):
    gen, ref, runs = job
    out = []
    by = {(r["entry"], r["mode"]): r for r in runs}
    for run in runs:
        try:
            rec = judge_run(gen, ref, run)
        except Exception:
            import traceback
            rec = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"], "reference_error": traceback.format_exc()}
        if run["entry"] == "captured":
            other = by.get(("typed", run["mode"]))
            if other is not None:
                rec["typed_captured_rows_identical"] = row_fingerprint(other) == row_fingerprint(run)
                a = [d["code"] for d in other.get("diagnostics") or []]
                b = [d["code"] for d in run.get("diagnostics") or []]
                rec["captured_extra_codes"] = sorted(set(b) - set(a))
                rec["typed_extra_codes"] = sorted(set(a) - set(b))
                try:
                    t = judge_run(gen, ref, other)
                    # the harness's disagreement (b4-finish/analyze_b4.py): integrity codes or breach counts differ
                    rec["typed_captured_disagree"] = (t["integrity"] != rec.get("integrity") or len(t.get("breaches", [])) != len(rec.get("breaches", [])))
                except Exception:
                    rec["typed_captured_disagree"] = None
        rec["meta"] = {k: gen["meta"][k] for k in ("family", "theory", "placement", "mill_and_allowance", "harness_validation", "design", "load_state")}
        out.append(rec)
    return out


def synthetic_run(gen, suffixed, upper_scale=1.0 + 1e-12):
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
    return {"tag": gen["tag"], "entry": "typed", "mode": "SparseInteractive", "results": rows, "summary": summary,
            "diagnostics": [{"code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "synthetic"}],
            "contract_evidence": {"exact_cases": [{"load_case_id": "case:1", "pipe_stress_extrema": evidence}]}}


def selftest(set_path):
    """Exercise the reconstructed rules on synthetic runs built from the reference itself."""
    gens = [json.loads(l) for l in open(set_path)]
    def bent(g):
        return any(r[0].startswith("element_local_bending_moment") and abs(float(Decimal(r[5]))) > 1.0 for r in g["ref"]["cases"]["case:1"]["rows"])

    picks = [g for g in gens if g["meta"]["harness_validation"] and g["meta"]["family"] in ("fixed_fixed", "pressure", "l_line") and bent(g)][:9]
    mills = [g for g in gens if g["meta"]["mill_and_allowance"] and g["meta"]["family"] == "pressure" and bent(g)][:2]
    out = []

    def check(name, cond):
        out.append((name, bool(cond)))
        print(("PASS " if cond else "FAIL ") + name)

    for g in picks + mills:
        for suffixed in (False, True):
            run = synthetic_run(g, suffixed)
            rec = judge_run(g, g["ref"], run)
            check(f"{g['tag']} suffixed={suffixed}: reference published as itself has no breach, coverage failure, unmapped or missing row",
                  not rec["breaches"] and not rec["disp_breaches"] and not rec["max_fail"] and rec["n_unmapped"] == 0 and rec["n_missing"] == 0 and rec["n_judged"] > 0
                  and not rec["cases"]["case:1"]["headline_findings"])
            key = "max_open_formula_stress_reduced_section" if suffixed else "max_open_formula_stress"
            saved = run["summary"][key]
            run["summary"][key] = {"value": saved["value"] * 0.5, "unit": "Pa"}
            rec = judge_run(g, g["ref"], run)
            run["summary"][key] = saved
            check(f"{g['tag']} suffixed={suffixed}: a headline below its basis maximum is found", bool(rec["cases"]["case:1"]["headline_findings"]))
            # pick the largest |value| bending-moment row
            bm = max((r for r in run["results"] if r["kind"] in ("element_local_bending_moment_y", "element_local_bending_moment_z")), key=lambda r: abs(r["value"]))
            for factor, expect in ((3.0, True), (0.5, False)):
                saved = bm["value"]
                bm["value"] = saved * (1 + factor * 1e-9)
                rec = judge_run(g, g["ref"], run)
                bm["value"] = saved
                hit = any(b["group"] == "bending" and b["entity"] == bm["entity_ref"] and b["loc"] == bm["location"] for b in rec["breaches"])
                check(f"{g['tag']} suffixed={suffixed}: bending row x(1+{factor}e-9) {'breaches' if expect else 'does not breach'}", hit == expect)
            # a group below the floor is not judged whatever its error
            rec0 = judge_run(g, g["ref"], run)
            # coverage: an upper bound below the largest station sample fails
            ev = run["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0]
            saved = ev["value_upper_pa"]
            ev["value_upper_pa"] = saved * 0.999
            rec = judge_run(g, g["ref"], run)
            ev["value_upper_pa"] = saved
            check(f"{g['tag']} suffixed={suffixed}: an upper bound 0.1% low fails coverage", len(rec["max_fail"]) >= 1)
            # an unknown row is unmapped; a dropped row is missing
            run["results"].append({"kind": "element_local_bending_moment_q", "value": 1.0, "entity_ref": "x", "case": "case:1", "location": "midspan", "component": "q", "metadata": {}})
            rec = judge_run(g, g["ref"], run)
            run["results"].pop()
            check(f"{g['tag']} suffixed={suffixed}: an unknown row kind is unmapped", rec["n_unmapped"] == 1)
            dropped = run["results"].pop(5)
            rec = judge_run(g, g["ref"], run)
            run["results"].insert(5, dropped)
            check(f"{g['tag']} suffixed={suffixed}: a dropped row is missing", rec["n_missing"] == 1)
            unjudged = [s for s in rec0.get("breaches", [])]
            check(f"{g['tag']} suffixed={suffixed}: baseline judged groups {rec0['n_judged']} > 0", rec0["n_judged"] > 0)
        if g["meta"]["mill_and_allowance"]:
            run = synthetic_run(g, True)
            swap = {}
            for r in run["results"]:
                b = row_basis(r)
                if b and r["kind"] == "element_local_bending_normal_stress_z":
                    swap[(r["entity_ref"], r["location"], b)] = r
            for (ent, loc, b), r in list(swap.items()):
                if b == "nominal_section":
                    o = swap[(ent, loc, "reduced_section")]
                    r["value"], o["value"] = o["value"], r["value"]
            rec = judge_run(g, g["ref"], run)
            check(f"{g['tag']}: swapping the nominal and reduced bending stresses breaches s_bending", any(b["group"] == "s_bending" for b in rec["breaches"]))
            run = synthetic_run(g, False)
            rec = judge_run(g, g["ref"], run)
            check(f"{g['tag']}: suffix-free stress rows are judged on the reduced section (no breach)", not rec["breaches"])
    bad = [n for n, ok in out if not ok]
    print(f"compare self-test: {len(out) - len(bad)} of {len(out)} checks pass")
    if bad:
        raise SystemExit(1)


def main():
    args = sys.argv[1:]
    if args and args[0] == "--selftest":
        selftest(args[1])
        return
    variant, workers = "own", 6
    if "--ref" in args:
        i = args.index("--ref")
        variant = args[i + 1]
        del args[i:i + 2]
    if "--workers" in args:
        i = args.index("--workers")
        workers = int(args[i + 1])
        del args[i:i + 2]
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
        jobs.append((gen, ref, rs))
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
