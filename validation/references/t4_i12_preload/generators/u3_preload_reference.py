#!/usr/bin/env python3
"""T4-I12 preload addendum: the U3 system cases under the variable-spring-hanger preload law.

Supplements validation/references/t4_i12 (frozen, unchanged). Its system cases solve the re-authored
invented demo, whose variable spring hanger support:SH-140 carries an entered cold load (430 N on UZ in
every one of the nine system documents, restored verbatim from the demo fixture in round 02). The
frozen generator modelled the hanger as a bare spring of its rate (42000 N/m); the cold load was not
applied. This generator derives the same quantities under DEL-04-03/REQ-014 and DEL-04-11/REQ-014:

  - the hanger is a spring of its entered rate k on its declared translational DOF, plus its entered
    cold load F_cold as a constant force on the pipe along the positive axis of that DOF at its node,
    in every load case;
  - its published support-on-pipe reaction on that DOF is F_cold - k*u (no case moves its base);
  - the cold load is published separately as the support's applied force (recorded under "preload");
  - installed_load and hot_load are review data and are not applied.

Standard-library Python 3 only. It imports, reads and runs no product code and no product output. It
executes the frozen round-00/01 generator (reference code) in its own namespace, writing that
generator's output only to a temporary directory, to use its frame, connector, document interpreter,
document solver and formatting; it reads the frozen u3_reference_cases.json for the request documents
and the frozen values, and staged/sh140_hanger_leaves.json (the hanger record's mechanical leaves, staged
from the demo fixture at a named commit) to confirm the hanger's leaves. The default check opens no file
outside this package and the frozen package. Every number
is formed from the documents' leaves in exact rationals (pi as the frozen generator's 125-decimal
rational); nothing is copied.

Order of work:
  1. Reproduction: with every cold load set to zero, every derived subtree equals the frozen one
     string for string.
  2. The law: the same subtrees with the cold load applied, checked for global balance of the
     document's applied loads plus the published reactions, stability under a second pi truncation,
     and agreement between the frozen generator's hard-coded demo solve and its document solve.
  3. Every leaf that differs from the frozen value is listed beside it.

Modes (run from this package's directory):
  python3 -I generators/u3_preload_reference.py --check   regenerate into a temporary directory and
                                                          compare byte for byte (changes no file)
  python3 -I generators/u3_preload_reference.py --write   write the outputs (intentional changes only)
  python3 -I generators/u3_preload_reference.py --check-live
                                                          compare the staged hanger leaves with the live demo
                                                          fixture's, naming any that differ (not run by --check)
"""
import argparse
import contextlib
import copy
import hashlib
import io
import json
import sys
import tempfile
from decimal import Decimal
from fractions import Fraction as Fr
from pathlib import Path

sys.dont_write_bytecode = True

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
ROOT = OUT.parents[2]
FROZEN = OUT.parent / "t4_i12"
FROZEN_FILE = FROZEN / "u3_reference_cases.json"
FROZEN_GEN = FROZEN / "generators" / "u3_reference.py"
FIXTURE = ROOT / "fixtures" / "product_preview" / "invented_preview_model.json"  # --check-live only
STAGED = "staged/sh140_hanger_leaves.json"
OUTPUT = "u3_reference_cases_preload.json"
FIXED_FILES = ("README.md", "generators/u3_preload_reference.py", STAGED)
HANGER_ID = "support:SH-140"
AXES = {"UX": ("global_x", 0), "UY": ("global_y", 1), "UZ": ("global_z", 2)}
BASIS = ("stiffness-only result of the frozen package validation/references/t4_i12 (u3_reference_cases.json): the "
         "variable spring hanger support:SH-140 modelled as a bare spring of its entered rate, its cold load not applied")


def load_frozen_generator():
    """Execute the frozen round-00/01 generator in a namespace of its own; its output goes to a temporary file."""
    ns = {"__name__": "u3_reference_frozen", "__file__": str(FROZEN_GEN)}
    saved = sys.argv
    with tempfile.TemporaryDirectory() as tmp:
        sys.argv = [str(FROZEN_GEN), str(Path(tmp) / "round01.json")]
        try:
            with contextlib.redirect_stdout(io.StringIO()):
                exec(compile(FROZEN_GEN.read_text(), str(FROZEN_GEN), "exec"), ns)
            code = 0
        except SystemExit as e:
            code = e.code
        finally:
            sys.argv = saved
    assert code == 0, "the frozen generator's own checks must pass"
    return ns


G = load_frozen_generator()
PI = G["PI"]
PI_ALT = Fr(G["PI_DEC"].quantize(Decimal(10) ** -115))
DEMO_CASES = list(G["CASES"])
COMPS = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
CROSS = []


def check(name, ok):
    CROSS.append({"check": name, "agrees": bool(ok)})


def get(doc, pointer):
    o = doc
    for part in pointer.strip("/").split("/"):
        o = o[int(part)] if isinstance(o, list) else o[part]
    return o


# ----------------------------------------------------------------------------- the law on a document
def hanger_of(document):
    """(node, dof, k, F_cold) of support:SH-140, read from the document's leaves."""
    (s,) = [s for s in document["model"]["supports"] if s["id"] == HANGER_ID]
    assert s["family"] == "variable_spring_hanger"
    h = s["hanger"]
    assert h["cold_load"]["unit"] == "N" and h["stiffness"]["value"]["unit"] == "N/m"
    return s["node"], h["stiffness"]["dof"], Fr(repr(h["stiffness"]["value"]["value"])), Fr(repr(h["cold_load"]["value"]))


def preload_primitive(node, dof, fc):
    """The cold load as a constant nodal force along +DOF (fed to the frozen document solver)."""
    return {"id": "preload:" + HANGER_ID, "category": "hanger_cold_load", "target": {"type": "node", "node": node},
            "direction": AXES[dof][0], "magnitude": {"value": fc}, "dimension": "force"}


def solve_law(document, cid, pi_val, scale, keep_span=False, material_variant=None):
    """The frozen document solve with the cold load added to f and to the hanger's published row."""
    node, dof, k, fc = hanger_of(document)
    fc = fc * scale
    model_i = G["interpret"](material_variant if material_variant is not None else document)
    model_i["cases"][cid]["applied"].append((preload_primitive(node, dof, float(fc)), Fr(1)))
    assert Fr(repr(float(fc))) == fc
    res = G["solve_doc"](model_i, cid, pi_val, keep_span=keep_span)
    res["reactions"][HANGER_ID][AXES[dof][1]] += fc
    balance(model_i, cid, res, keep_span)
    return res


def balance(model_i, cid, res, keep_span):
    """Global balance under the law: the document's own applied loads plus every published reaction = 0.

    Formed independently of the solve: a uniform load w on a member is its resultant w*L at the member's
    midpoint; thermal strain is self-equilibrated; the cold load is not an applied load (it is inside the
    hanger's published row)."""
    Ft, Mt = [Fr(0)] * 3, [Fr(0)] * 3
    axis = {"global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}
    for ld, fac in model_i["cases"][cid]["applied"]:
        if ld["category"] == "hanger_cold_load":
            continue
        val = Fr(repr(ld["magnitude"]["value"])) * fac
        if ld["target"]["type"] == "element":
            p = model_i["pipes"][ld["target"]["pipe"]]
            a, b = model_i["nodes"][p["from"]], model_i["nodes"][p["to"]]
            L = G["sqrt_fr"](G["dot"](G["vsub"](b, a), G["vsub"](b, a)))
            force, at, moment = [Fr(0)] * 3, [(x + y) / 2 for x, y in zip(a, b)], [Fr(0)] * 3
            force[axis[ld["direction"]]] = val * L
        else:
            at = model_i["nodes"][ld["target"]["node"]]
            force, moment = [Fr(0)] * 3, [Fr(0)] * 3
            j = axis[ld["direction"]]
            (force if j < 3 else moment)[j % 3] = val
        Ft = G["vadd"](Ft, force)
        Mt = G["vadd"](Mt, G["vadd"](G["cross"](at, force), moment))
    nodes_of = {s[0]: s[1] for s in model_i["sups"]}
    for sid, rv in res["reactions"].items():
        Ft = G["vadd"](Ft, rv[0:3])
        Mt = G["vadd"](Mt, G["vadd"](G["cross"](model_i["nodes"][nodes_of[sid]], rv[0:3]), rv[3:6]))
    check(f"{cid}{' (P-130 retained)' if keep_span else ''}: applied loads plus published reactions balance exactly "
          "(force and moment about the origin)", Ft == [0] * 3 and Mt == [0] * 3)


def demo_law(cid, pi_val, variant, fc, dof):
    """The frozen generator's hard-coded demo solve (connector, raw-difference or parallel) with the cold load.

    Its loads table gains the cold load as a nodal force at the hanger's node; the published hanger row gains
    F_cold. Its balance sums (f plus the -k u row) equal the law's (applied loads plus F_cold - k u)."""
    saved = G["CASES"]
    G["CASES"] = copy.deepcopy(saved)
    vec = [Fr(0)] * 3
    vec[AXES[dof][1]] = fc
    G["CASES"][cid]["nodal"].append((G["SPRING"][1], vec))
    try:
        res = G["solve_demo"](cid, pi_val, variant)
    finally:
        G["CASES"] = saved
    res["reactions"][HANGER_ID][AXES[dof][1]] += fc
    return res


def discriminating(res, other, Fs, Ms):
    """The frozen rule: components whose wrong values differ by more than 1e-6 of the family floor."""
    return [f"{sid}.{COMPS[k]}" for sid in res["reactions"] for k in range(6)
            if abs(res["reactions"][sid][k] - other["reactions"][sid][k]) > Fr(1, 10 ** 6) * (Fs if k < 3 else Ms)]


# ----------------------------------------------------------------------------- the derived subtrees
def derive(frozen, scale):
    """Every derived subtree of the affected cases, at cold-load scale 0 (reproduction) or 1 (the law)."""
    fmt = G["sysfmt"]
    sys_block = G["sys_block"]
    c = frozen["cases"]
    d001 = get(frozen, "/cases/U3-SYS-DEMO-CONNECTOR-001/inputs/document_v3_0.3.0")
    d002 = get(frozen, "/cases/U3-SYS-DEMO-CONNECTOR-002-LR1/inputs/document_v3_0.4.0")
    dzero = get(frozen, "/cases/U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL/boundary_zero_strain/document_v3_0.4.0")
    dmat = get(frozen, "/cases/U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL/inputs/document_v3_0.4.0")
    dstored = get(frozen, "/cases/U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL/inputs/document_v3_0.4.0")
    node, dof, k, fc0 = hanger_of(d001)
    fc = fc0 * scale
    out = {"U3-SYS-DEMO-CONNECTOR-001": {"expected": {}, "wrong_result_discriminators": {}},
           "U3-SYS-DEMO-CONNECTOR-002-LR1": {"expected": {}},
           "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL": {"boundary_zero_strain": {"expected_values": {}}},
           "U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL": {"wrong_result_discriminators": {}},
           "U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL": {"expected": {}}}
    tag = "F_cold = 0" if scale == 0 else "the law"
    for cid in DEMO_CASES:
        # 001: the 0.3.0 document; the hard-coded demo solve must agree with it string for string.
        res = solve_law(d001, cid, PI, scale)
        block = sys_block(res)
        alt = sys_block(solve_law(d001, cid, PI_ALT, scale))
        check(f"{tag} 001 {cid}: every published string is stable under pi truncated at 115 vs 125 decimals", alt == block)
        demo = demo_law(cid, PI, "connector", fc, dof)
        check(f"{tag} 001 {cid}: the hard-coded demo solve equals the document solve (displacements, reactions, connector)",
              all(demo["d"][i] == res["d"][i] for i in range(len(res["d"]))) and demo["reactions"] == res["reactions"]
              and demo["g"] == res["g"])
        check(f"{tag} 001 {cid}: the demo solve balances exactly", demo["Ftot"] == [0] * 3 and demo["Mtot"] == [0] * 3)
        out["U3-SYS-DEMO-CONNECTOR-001"]["expected"][cid] = block
        Fs = max(max(abs(x) for x in rv[0:3]) for rv in res["reactions"].values())
        Ms = max(max(abs(x) for x in rv[3:6]) for rv in res["reactions"].values())
        raw = demo_law(cid, PI, "raw", fc, dof)
        par = demo_law(cid, PI, "parallel", fc, dof)
        xi, xj = G["NODES"]["node:N-130"], G["NODES"]["node:N-140"]
        frozen_disc = c["U3-SYS-DEMO-CONNECTOR-001"]["wrong_result_discriminators"][cid]
        out["U3-SYS-DEMO-CONNECTOR-001"]["wrong_result_discriminators"][cid] = {
            "use": frozen_disc["use"],
            "raw_difference_element_solve": {
                "what": frozen_disc["raw_difference_element_solve"]["what"],
                "reactions": {sid: [fmt(x, 20) for x in rv] for sid, rv in raw["reactions"].items()},
                "global_moment_imbalance_about_origin_N_m": [fmt(x, 20) for x in raw["Mtot"]],
                "global_force_imbalance_N": [fmt(x, 20) for x in raw["Ftot"]],
                "element_couple_r_x_Fj_N_m": [fmt(x, 20) for x in G["cross"](G["vsub"](xj, xi), raw["fc"][6:9])],
                "discriminating_components": discriminating(res, raw, Fs, Ms)},
            "retained_span_in_parallel_solve": {
                "what": frozen_disc["retained_span_in_parallel_solve"]["what"],
                "reactions": {sid: [fmt(x, 20) for x in rv] for sid, rv in par["reactions"].items()},
                "connector_g_local": [fmt(x, 20) for x in par["g"]],
                "discriminating_components": discriminating(res, par, Fs, Ms)}}
        # 002-LR1, the zero-strain boundary and the stored-unapplied control: their own 0.4.0 documents.
        for case_id, document, path in (("U3-SYS-DEMO-CONNECTOR-002-LR1", d002, ("expected",)),
                                        ("U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL", dzero, ("boundary_zero_strain", "expected_values")),
                                        ("U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL", dstored, ("expected",))):
            check(f"{tag} {case_id} {cid}: admitted (no replaced-span refusal)",
                  G["replaced_span_refusal"](G["interpret"](document), cid) is None)
            b = sys_block(solve_law(document, cid, PI, scale))
            target = out[case_id]
            for p in path:
                target = target[p]
            target[cid] = b
            check(f"{tag} {case_id} {cid}: equals 001 string for string (the frozen equality, kept under the law)",
                  b == out["U3-SYS-DEMO-CONNECTOR-001"]["expected"][cid])
        for form, mdoc in (("0.3.0", get(frozen, "/cases/U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL/inputs/document_v3_0.3.0")),
                           ("0.4.0", dmat)):
            check(f"{tag} U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL {form} {cid}: equals 001 string for string (its expected "
                  "equality statement holds)", sys_block(solve_law(mdoc, cid, PI, scale)) == out["U3-SYS-DEMO-CONNECTOR-001"]["expected"][cid])
        # Material control: P-130 kept in parallel, with its own and with the other E/nu (0.4.0 document).
        base_par = solve_law(d002, cid, PI, scale, keep_span=True)
        ctl_par = solve_law(d002, cid, PI, scale, keep_span=True, material_variant=dmat)
        Fs1 = Fr(out["U3-SYS-DEMO-CONNECTOR-001"]["expected"][cid]["zero_scale_floors"]["force_N"])
        Ms1 = Fr(out["U3-SYS-DEMO-CONNECTOR-001"]["expected"][cid]["zero_scale_floors"]["moment_N_m"])
        frozen_ctl = c["U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL"]["wrong_result_discriminators"][cid]
        out["U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL"]["wrong_result_discriminators"][cid] = {
            "what": frozen_ctl["what"],
            "reactions_with_P130_E_2e11_nu_0.3": {sid: [fmt(x, 20) for x in rv] for sid, rv in base_par["reactions"].items()},
            "reactions_with_P130_E_1e11_nu_0.25": {sid: [fmt(x, 20) for x in rv] for sid, rv in ctl_par["reactions"].items()},
            "connector_g_with_P130_E_2e11": [fmt(x, 20) for x in base_par["g"]],
            "connector_g_with_P130_E_1e11": [fmt(x, 20) for x in ctl_par["g"]],
            "moved_reaction_components": discriminating(base_par, ctl_par, Fs1, Ms1),
            "moved_g_components": [j for j in range(6) if abs(base_par["g"][j] - ctl_par["g"][j]) > Fr(1, 10 ** 9) * max(abs(base_par["g"][j]), 1)]}
        if scale:
            # The closed-form part of the law: the hanger row is F_cold - k u at its DOF, nothing else.
            u = res["d"][res["dof"](node)[AXES[dof][1]]]
            row = res["reactions"][HANGER_ID]
            check(f"the law 001 {cid}: the hanger's published row is F_cold - k u on {dof} and zero elsewhere",
                  row[AXES[dof][1]] == fc - k * u and all(row[j] == 0 for j in range(6) if j != AXES[dof][1]))
    return out


# ----------------------------------------------------------------------------- comparison
def leaf_diffs(law, frozen, pointer):
    if isinstance(law, dict):
        assert set(law) == set(frozen), pointer
        return [d for key in law for d in leaf_diffs(law[key], frozen[key], f"{pointer}/{key}")]
    if isinstance(law, list):
        assert len(law) == len(frozen), pointer
        return [d for i, (a, b) in enumerate(zip(law, frozen)) for d in leaf_diffs(a, b, f"{pointer}/{i}")]
    return [] if law == frozen else [(pointer, law, frozen)]


def quantity(pointer):
    p = pointer.strip("/").split("/")
    tail = p[-1]
    if "displacements" in p:
        i = p.index("displacements")
        kind = {"u_m": ["UX", "UY", "UZ"], "theta_rad": ["RX", "RY", "RZ"]}[p[i + 2]][int(tail)]
        return f"displacement {kind} at {p[i + 1]}"
    for key in ("reactions_support_on_pipe_global_Fx_Fy_Fz_Mx_My_Mz", "reactions", "reactions_with_P130_E_2e11_nu_0.3",
                "reactions_with_P130_E_1e11_nu_0.25"):
        if key in p:
            i = p.index(key)
            return f"{p[i - 1] if key == 'reactions' else key} reaction {COMPS[int(tail)]} of {p[i + 1]}"
    if "zero_scale_floors" in p:
        return f"zero-scale floor {tail}"
    if "connector" in p:
        return "connector " + "/".join(p[p.index("connector") + 1:])
    return "/".join(p[-2:])


def mechanical_leaves(support):
    """The leaves of a hanger support record that the derivation rests on (descriptive strings excluded)."""
    h = support["hanger"]
    out = {"id": support["id"], "node": support["node"], "family": support["family"]}
    for key in ("stiffness", "installed_load", "cold_load", "hot_load", "travel_range"):
        stack = [(f"hanger.{key}", h[key])]
        while stack:
            path, v = stack.pop()
            if isinstance(v, dict):
                stack += [(f"{path}.{k}", x) for k, x in v.items()]
            else:
                out[path] = v
    return out


def staged_record():
    return json.loads((OUT / STAGED).read_text())["record"]


def check_live():
    """The staged hanger leaves against the live demo fixture's, leaf by leaf."""
    (live,) = [s for s in json.loads(FIXTURE.read_text())["supports"] if s["id"] == HANGER_ID]
    staged, now = mechanical_leaves(staged_record()), mechanical_leaves(live)
    differ = sorted(k for k in set(staged) | set(now) if staged.get(k) != now.get(k))
    for k in differ:
        print(f"DIFFERS: {HANGER_ID} {k}: staged {staged.get(k)!r}, live {now.get(k)!r}")
    print(f"live fixture {FIXTURE.relative_to(ROOT)}: {len(staged)} staged leaves compared, {len(differ)} differ")
    return 1 if differ else 0


def encode(obj):
    return (json.dumps(obj, indent=1, sort_keys=False) + "\n").encode()


def build():
    CROSS.clear()
    raw = FROZEN_FILE.read_bytes()
    frozen = json.loads(raw)
    # The hanger leaves: every system document carries the same SH-140, whose mechanical leaves equal those
    # staged from the demo fixture (staged/sh140_hanger_leaves.json).
    docs = frozen["round_02"]["b1_hanger_restored"]["documents"]
    records = [get(frozen, p.rsplit("/hanger", 1)[0]) for p in docs]
    check("the nine system documents' SH-140 records have the mechanical leaves staged from the demo fixture",
          len(records) == 9 and all(mechanical_leaves(r) == mechanical_leaves(staged_record()) for r in records))
    node, dof, k, fc = hanger_of(get(frozen, "/cases/U3-SYS-DEMO-CONNECTOR-001/inputs/document_v3_0.3.0"))
    check("the hanger acts on a translational DOF", dof in AXES)

    zero = derive(frozen, Fr(0))
    for cid, sub in zero.items():
        diffs = leaf_diffs(sub, {k_: (frozen["cases"][cid][k_] if k_ != "boundary_zero_strain" else
                                      {"expected_values": frozen["cases"][cid][k_]["expected_values"]}) for k_ in sub},
                           f"/cases/{cid}")
        check(f"reproduction {cid}: with F_cold = 0 every derived string equals the frozen file's", not diffs)
    law = derive(frozen, Fr(1))

    changed, unchanged_subtrees = [], []
    for cid, sub in law.items():
        frozen_sub = {k_: (frozen["cases"][cid][k_] if k_ != "boundary_zero_strain" else
                           {"expected_values": frozen["cases"][cid][k_]["expected_values"]}) for k_ in sub}
        for pointer, a, b in leaf_diffs(sub, frozen_sub, f"/cases/{cid}"):
            changed.append({"pointer": pointer, "quantity": quantity(pointer), "law": a, "stiffness_only": b,
                            "stiffness_only_basis": BASIS, "cold_load_N": str(fc)})
    # The frozen text quoting the raw-difference imbalances (770, 275, 440 N*m) stays true only if they are unchanged.
    for cid in DEMO_CASES:
        pointer = f"U3-SYS-DEMO-CONNECTOR-001/wrong_result_discriminators/{cid}/raw_difference_element_solve/global_moment_imbalance_about_origin_N_m"
        check(f"the law: the raw-difference imbalance of {cid} is unchanged, so the frozen text quoting it "
              "(plan_section5_substitute_round_02, findings) still holds (the cold load acts along the connector axis r)",
              get(law, pointer) == get(frozen["cases"], pointer))
    # No other case carries a hanger: the nine system documents are the only ones with variable_spring_hanger.
    others = [cid for cid, case in frozen["cases"].items() if cid not in law and "variable_spring_hanger" in json.dumps(case)]
    check("no case outside the five derived here carries a variable spring hanger", others == [])
    doc = {
        "schema_version": "t4-i12-u3-reference-1-preload-addendum-1",
        "status": "candidate references under the variable-spring-hanger preload law; derived by their author TASK without "
                  "sight of any product output; not yet independently refuted; not product evidence",
        "supplements": {"file": "validation/references/t4_i12/u3_reference_cases.json (frozen; unchanged)",
                        "sha256": hashlib.sha256(raw).hexdigest()},
        "law": "DEL-04-03/REQ-014 and DEL-04-11/REQ-014: spring of the entered rate on its declared translational DOF plus "
               "the entered cold load as a constant force along +DOF at the support's node in every case; published "
               "support force F_cold - k*(u - g) (g = 0: no case moves the hanger's base); the cold load published "
               "separately as the support's applied force; installed_load and hot_load not applied",
        "conventions": {
            "reactions": "support-on-pipe, global, Fx Fy Fz Mx My Mz (the frozen convention); a variable spring hanger's "
                         "row is F_cold - k u on its DOF",
            "layout": "cases/<case id>/... holds each derived subtree at the same JSON pointer as in the frozen file, "
                      "with the same strings' format (30 significant digits; 20 in the discriminators); the request "
                      "documents are the frozen file's own (see inputs)",
            "criteria": "unchanged: the frozen file's criterion of each case applies"},
        # Each pointer comes from the frozen file's round_02.b1_hanger_restored.documents, which fixes the hanger at
        # index 3 of every document's supports list; removing that suffix leaves the document's own pointer.
        "inputs": {p.replace("/model/supports/3/hanger", ""): {"sha256_of_canonical_json": hashlib.sha256(
            json.dumps(get(frozen, p.replace("/model/supports/3/hanger", "")), sort_keys=True, separators=(",", ":")).encode()
        ).hexdigest()} for p in docs},
        "cases": law,
        "preload": {
            "support": HANGER_ID, "node": node, "dof": dof, "rate_N_per_m": str(k), "cold_load_N": str(fc),
            "applied_force": {"per_case": "in every admitted load case (load:L-100, load:L-200, load:L-300) of every "
                                          "affected case, support:SH-140 publishes the applied force F_cold = "
                              + str(fc) + " N along +" + dof + " at " + node,
                              "value_N": str(fc)},
            "changed": changed,
            "unaffected": {
                "U3-SYS-DEMO-CONNECTOR-001 refusal_variants, U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL expected (a, b, c)":
                    "structural refusals (MODEL_INCOMPLETE, no results); which cases refuse depends only on what the replaced "
                    "span owns, not on the hanger",
                "U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL expected":
                    "an equality statement (identical to 001 / 002-LR1), which still holds under the law (checked)",
                "every other case (U3-J*, U3-REF-*, U3-SIX-*, U3-B-ORACLE, U3-GENERIC-*, U3-W4-*, U3-FRAME-*, U3-OFFSETS, "
                "U3-COUPLED-*, U3-PRELOAD-RELIEF, U3-REVERSAL, U3-FINITE-*, U3-RAW-*, U3-NI-FRICTION-FRAME, U3-KD5-*, "
                "U3-ZERO-LENGTH-*, U3-QX-*)": "no model with a variable spring hanger",
                "round_02.zero_floors": "keyed to FK unit-level arrays only; no system case",
                "text quoting the raw-difference imbalances (U3-SYS-DEMO-CONNECTOR-001 plan_section5_substitute_round_02, "
                "findings[1])": "the imbalances 770, 275 and 440 N*m are unchanged under the law (checked): the cold load "
                                "acts along global Z, which is the connector axis r = N-140 - N-130, so r x F_j does not change"}},
        "crosscheck": list(CROSS),
    }
    return {OUTPUT: encode(doc)}


def with_sums(files):
    sums = []
    for path in sorted(list(files) + list(FIXED_FILES)):  # every file of the package except SHA256SUMS
        data = files[path] if path in files else (OUT / path).read_bytes()
        sums.append(f"{hashlib.sha256(data).hexdigest()}  {path}")
    files["SHA256SUMS"] = ("\n".join(sums) + "\n").encode()
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--check-live", action="store_true")
    args = parser.parse_args()
    if args.check_live:
        return check_live()
    files = with_sums(build())
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", c["check"])
    if bad:
        return 1
    if args.write:
        for path, data in files.items():
            (OUT / path).parent.mkdir(parents=True, exist_ok=True)
            (OUT / path).write_bytes(data)
        print(f"wrote {len(files)} files")
        return 0
    with tempfile.TemporaryDirectory() as tmp:
        for path, data in files.items():
            (Path(tmp) / path).write_bytes(data)
        failures = [p for p in files if not (OUT / p).exists() or (OUT / p).read_bytes() != (Path(tmp) / p).read_bytes()]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files) - set(FIXED_FILES))
    if failures or extra:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
