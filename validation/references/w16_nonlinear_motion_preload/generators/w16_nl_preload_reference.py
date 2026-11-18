#!/usr/bin/env python3
"""W-16 stage 2 preload addendum: c1_hanger under the variable-spring-hanger preload law.

Supplements validation/references/w16_nonlinear_motion (frozen, unchanged). That
package derived its expectation for requests/c1_hanger.json with the variable
spring hanger as a bare spring of its entered rate; the hanger record carries an
entered cold load that was not applied. This generator derives the same
quantities under the law of DEL-04-03/REQ-014 and DEL-04-11/REQ-014:

  - the hanger is a spring of its entered rate k on its declared translational
    DOF, plus its entered cold load F_cold as a constant force on the pipe along
    the positive axis of that DOF at the support's node, in every load case;
  - its published support force on that DOF is F = F_cold - k*(u - g), with g
    the case's movement of its base (0 where the case does not move it);
  - the cold load itself is published separately as the support's applied force
    (recorded here under "preload", not inside the frozen-format case rows);
  - installed_load and hot_load are review data and are not applied.

Standard-library Python 3 only. It imports, reads and runs no product code and
no product output. It imports the frozen package's generator (reference code)
for its exact Euler-Bernoulli frame, its state enumeration and its published-row
rules, and reads the committed request document and the frozen references.json.
Every number is formed from the leaves of the committed request, in exact
rationals; nothing is copied.

Order of work:
  1. Reproduction: with every cold load set to zero, the case rows equal the
     frozen package's rows for c1_hanger exactly (as JSON values).
  2. The law: the same rows with the cold load applied, checked against the
     closed form u = (k*g + F_cold)/(k + k_m), F = F_cold - k*(u - g), and
     equilibrium with the anchor.
  3. Every leaf that differs from the frozen value is listed beside it.

Modes (run from this package's directory):
  python3 -I generators/w16_nl_preload_reference.py --check   regenerate into a temporary
                                                              directory and compare byte for byte
  python3 -I generators/w16_nl_preload_reference.py --write   write the outputs (intentional changes only)
"""
import argparse
import hashlib
import importlib.util
import json
import sys
import tempfile
from fractions import Fraction as F
from itertools import product
from pathlib import Path

sys.dont_write_bytecode = True  # never leave a __pycache__ in the frozen package

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FROZEN = OUT.parent / "w16_nonlinear_motion"
FROZEN_GEN = FROZEN / "generators" / "w16_nl_reference.py"
REQUEST_NAME = "c1_hanger"
REQUEST_PATH = FROZEN / "requests" / f"{REQUEST_NAME}.json"
FIXED_FILES = ("README.md", "generators/w16_nl_preload_reference.py")
STIFFNESS_ONLY_BASIS = ("stiffness-only result of the frozen package validation/references/w16_nonlinear_motion "
                        "(references.json): the variable spring hanger modelled as a bare spring of its entered rate, "
                        "its cold load not applied")

_spec = importlib.util.spec_from_file_location("w16_nl_reference_frozen", FROZEN_GEN)
ref = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ref)

DOFS = ref.DOFS
bounds, sqrt_bounds = ref.bounds, ref.sqrt_bounds


# ----------------------------------------------------------------------------- the law
def preloads(model, request, scale):
    """{support id: (global dof, F_cold)} for every variable spring hanger, read from the request's leaves."""
    out = {}
    for s in request["model"]["supports"]:
        if s.get("family") != "variable_spring_hanger":
            continue
        h = s["hanger"]
        dof = h["stiffness"]["dof"]
        assert dof in ("UX", "UY", "UZ"), "a hanger acts on a translational DOF"
        assert h["cold_load"]["unit"] == "N"
        d = 6 * model.nodes.index(s["node"]) + DOFS.index(dof)
        out[s["id"]] = (d, scale * F(h["cold_load"]["value"]))
    return out


def published_law(model, combo, u, r, ground, spring_g, pre):
    """The frozen published rows, with each hanger's support row replaced by F_cold - k*(u - g).

    Member groups, nonlinear slots and displacements are the frozen rules applied to the state solved under the
    law. Support rows are formed exactly as the frozen generator forms them (rigid rows K*u - f, spring rows
    -k*(u - g), slot rows), with the hanger's cold load added to its own spring row."""
    groups, signed, displacements = ref.published(model, combo, u, r, ground, spring_g)
    for support_id, nd in model.supports:
        if support_id not in pre:
            continue
        rows = [F(0)] * 6
        for d in model.rigid.get(support_id, []):
            rows[d % 6] = r[d]
        for sid, d, k in model.springs:
            if sid == support_id:
                rows[d % 6] = -k * (u[d] - (spring_g or {}).get(sid, F(0)))
        d, fc = pre[support_id]
        rows[d % 6] += fc
        assert not any(e["id"] == support_id for e in model.nonlinear)
        force_sq = sum(x * x for x in rows[:3])
        moment_sq = sum(x * x for x in rows[3:])
        groups[support_id] = {"node": {
            "support_force": sqrt_bounds(force_sq), "support_moment": sqrt_bounds(moment_sq),
            "support_force_magnitude": sqrt_bounds(force_sq), "support_moment_magnitude": sqrt_bounds(moment_sq)}}
        signed[support_id]["components"] = [bounds(x) for x in rows]
    return groups, signed, displacements


def enumerate_case_law(model, case, pre):
    """The frozen enumeration (every nonlinear state, strict and weak) with the cold loads in f."""
    f, base, ground, spring_g = model.loads(case)
    for d, fc in pre.values():
        f[d] += fc  # the cold load: a constant force on the pipe along +DOF, in every case
    qualifying, exact = [], []
    for combo in product(*[ref.states_for(e) for e in model.nonlinear]):
        solved = ref.solve_state(model, f, base, ground, combo)
        if solved is None:
            continue
        u, r = solved
        kind = ref.classify(model, combo, u, r, ground)
        if kind is None:
            continue
        groups, signed, displacements = published_law(model, combo, u, r, ground, spring_g, pre)
        states = {e["id"]: s for e, (s, _, _) in zip(model.nonlinear, combo)}
        qualifying.append({"standing": kind, "states": states, "groups": groups, "signed": signed,
                           "displacements": displacements})
        exact.append((kind, states, u, r, f))
    assert not model.nonlinear, "c1_hanger has no nonlinear support; the state enumeration is the single linear state"
    oracle = {"strict_states": [q for q in qualifying if q["standing"] == "strict"],
              "weak_states": [q["states"] for q in qualifying if q["standing"] == "weak"],
              "base_prescribed": {DOFS[d % 6] + "@" + model.nodes[d // 6]: bounds(v) for d, v in sorted(base.items())},
              "ground_motion": {k: bounds(v) for k, v in sorted(ground.items())}}
    if spring_g:
        oracle["spring_ground"] = {k: bounds(v) for k, v in sorted(spring_g.items())}
    return oracle, exact, spring_g


# ----------------------------------------------------------------------------- closed form
CROSS = []


def check(name, agrees):
    CROSS.append({"check": name, "agrees": bool(agrees)})


def closed_form(model, case_id, exact, spring_g, pre):
    """u = (k g + F_cold)/(k + k_m) on the tip UX; F = F_cold - k(u - g); anchor UX row = -F."""
    (kind, _, u, r, f), = exact
    check(f"c1_hanger {case_id}: one linear state, strict", kind == "strict")
    (sid, d, k), = model.springs
    _, fc = pre[sid]
    (cond,), = ref.condensed(model, [d], set(model.rigid["support:ANCHOR"]))
    k_m = cond - k
    check(f"c1_hanger {case_id}: the condensed tip stiffness is EA/L", k_m == ref.E * ref.A / 4)
    g = spring_g.get(sid, F(0))
    check(f"c1_hanger {case_id}: u = (k g + F_cold)/(k + k_m)", u[d] == (k * g + fc) / (k + k_m))
    force = fc - k * (u[d] - g)
    check(f"c1_hanger {case_id}: F = F_cold - k(u - g) = (F_cold + k g)/2 (k = k_m)", force == (fc + k * g) / 2)
    anchor_ux = r[6 * model.nodes.index("node:A")]
    check(f"c1_hanger {case_id}: equilibrium: anchor UX row + hanger row = 0 (no other applied force)",
          anchor_ux + force == 0)
    return force


# ----------------------------------------------------------------------------- comparison
def leaf_diffs(law, frozen, path=""):
    """Every leaf (a [lo, hi] bound pair or a scalar) where the two trees differ, with its path."""
    if isinstance(law, list) and len(law) == 2 and all(isinstance(x, float) for x in law):
        return [] if law == frozen else [(path, law, frozen)]
    if isinstance(law, dict):
        assert set(law) == set(frozen), path
        out = []
        for key in sorted(law):
            out += leaf_diffs(law[key], frozen[key], f"{path}/{key}")
        return out
    if isinstance(law, list):
        assert len(law) == len(frozen), path
        out = []
        for i, (a, b) in enumerate(zip(law, frozen)):
            out += leaf_diffs(a, b, f"{path}/{i}")
        return out
    return [] if law == frozen else [(path, law, frozen)]


def quantity_name(path):
    parts = path.strip("/").split("/")
    if parts[0] == "strict_states":
        parts = parts[2:]
    if parts[0] == "displacements":
        return f"nodal displacement {DOFS[int(parts[2])]} at {parts[1]}"
    if parts[0] == "signed" and parts[2] == "components":
        return f"signed support row {DOFS[int(parts[3])]} of {parts[1]}"
    if parts[0] == "groups":
        if parts[1].startswith("pipe:"):
            return f"member {parts[3]} force norm at {parts[2]} of {parts[1]}"
        return f"support {parts[3]} of {parts[1]}"
    return "/".join(parts)


def encode(obj):
    return (json.dumps(obj, indent=1, sort_keys=True) + "\n").encode()


def build():
    CROSS.clear()
    data = REQUEST_PATH.read_bytes()
    request = json.loads(data)
    frozen = json.loads((FROZEN / "references.json").read_text())
    frozen_req = frozen["requests"][REQUEST_NAME]
    check("the committed request is the one the frozen package hashed",
          hashlib.sha256(data).hexdigest() == frozen_req["request_sha256"])
    model = ref.Model(request)

    # 1. Reproduction with the cold load at zero.
    zero = preloads(model, request, F(0))
    for c in model.cases:
        oracle, _, _ = enumerate_case_law(model, c, zero)
        same = json.loads(json.dumps(oracle)) == frozen_req["cases"][c["id"]]
        check(f"reproduction {c['id']}: with F_cold = 0 the rows equal the frozen package's rows exactly", same)

    # 2. The law.
    pre = preloads(model, request, F(1))
    cases, supplement = {}, {}
    for c in model.cases:
        oracle, exact, spring_g = enumerate_case_law(model, c, pre)
        force = closed_form(model, c["id"], exact, spring_g, pre)
        oracle = json.loads(json.dumps(oracle))
        cases[c["id"]] = oracle
        diffs = leaf_diffs(oracle, frozen_req["cases"][c["id"]])
        supplement[c["id"]] = {
            "applied_force": {sid: {"dof": DOFS[d % 6], "node": model.nodes[d // 6], "value": bounds(fc)}
                              for sid, (d, fc) in sorted(pre.items())},
            "hanger_support_force": {sid: {"dof": DOFS[d % 6], "value": bounds(force)} for sid, (d, _) in pre.items()},
            "changed": [{"path": p, "quantity": quantity_name(p), "law": a, "stiffness_only": b,
                         "stiffness_only_basis": STIFFNESS_ONLY_BASIS,
                         "cold_load": {sid: bounds(fc) for sid, (_, fc) in sorted(pre.items())}}
                        for p, a, b in diffs],
            "unchanged_leaves": "every other leaf of this case's rows equals the frozen value",
        }
    references = {
        "document": "openpipestress.w16_nonlinear_motion_preload_references",
        "status": "candidate references under the variable-spring-hanger preload law; derived by their author TASK "
                  "without sight of any product output, not yet independently refuted",
        "supplements": "validation/references/w16_nonlinear_motion/references.json (frozen; unchanged), request c1_hanger",
        "law": "DEL-04-03/REQ-014 and DEL-04-11/REQ-014: spring of the entered rate on its declared translational DOF "
               "plus the entered cold load as a constant force along +DOF at the support's node in every case; "
               "published support force F_cold - k*(u - g); the cold load published separately as the support's "
               "applied force; installed_load and hot_load not applied",
        "basis": frozen["basis"] + "; the cold load added to f at the hanger's DOF and to the hanger's published row",
        "units": frozen["units"],
        "normalized_section": frozen["normalized_section"],
        "elastic_modulus_pa": frozen["elastic_modulus_pa"],
        "shear_modulus_pa": frozen["shear_modulus_pa"],
        "criterion": frozen["criterion"],
        "crosscheck": list(CROSS),
        "requests": {REQUEST_NAME: {"part": frozen_req["part"],
                                    "request": f"../w16_nonlinear_motion/requests/{REQUEST_NAME}.json",
                                    "request_sha256": hashlib.sha256(data).hexdigest(), "cases": cases}},
        "preload": {REQUEST_NAME: supplement},
        "unaffected": "every other request of the frozen package: none carries a variable spring hanger "
                      "(c1_hanger is the only request with family variable_spring_hanger)",
    }
    others = [p.name for p in sorted((FROZEN / "requests").glob("*.json")) if p.name != f"{REQUEST_NAME}.json"
              and any(s.get("family") == "variable_spring_hanger" for s in json.loads(p.read_text())["model"]["supports"])]
    check("no other frozen request carries a variable spring hanger", not others)
    references["crosscheck"] = list(CROSS)
    files = {"references.json": encode(references)}
    return files


def with_sums(files):
    sums = []
    for path in sorted(list(files) + list(FIXED_FILES)):
        data = files[path] if path in files else (OUT / path).read_bytes()
        sums.append(f"{hashlib.sha256(data).hexdigest()}  {path}")
    files["SHA256SUMS"] = ("\n".join(sums) + "\n").encode()
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    args = parser.parse_args()
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
            (Path(tmp) / path).parent.mkdir(parents=True, exist_ok=True)
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
