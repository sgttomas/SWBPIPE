"""Exact-rational K-D5 references under the variable spring preload law (supplements ../kd5_nonlinear).

The law (DEL-04-03/REQ-014, DEL-04-11/REQ-014): a variable spring hanger is a spring of its entered rate k on its
declared translational DOF plus its entered cold load F_cold as a constant force on the structure along the
positive axis of that DOF at its node, in every load case. Its published support force on that DOF is
F_cold - k*(u - g) (g = 0 here: no case moves a base); the case also publishes F_cold as the support's applied force.

This reuses the frozen independent reference ../kd5_nonlinear/reference.py (frame formation, assembly, state
enumeration, exact solve, DEC-067 classification, published rows) and adds only the law: the cold load in the
force vector and in the hanger's published row, and the committed plain-spring record form (family "spring",
restraints == [its stiffness DOF], top-level stiffness), which reference.py does not read. Every number is formed
in exact rationals from the leaves of the request documents. It imports and reads no product code or output.

Outputs (README.md): u1_spring_law_oracle.json for the frozen u1_spring request under the law, with every
quantity that differs from the frozen stiffness-only oracle recorded beside it; the twin request with a plain
spring in place of the hanger and its oracle, asserted here to reproduce the frozen oracle exactly; and a
preloaded request in which the one-way support stays pressed, with its oracle.

  python3 reference_preload.py --check   regenerate into a temporary directory and compare byte for byte (default)
  python3 reference_preload.py --write   replace the outputs here
"""
import argparse
import copy
import hashlib
import json
import sys
import tempfile
from fractions import Fraction as F
from itertools import product
from pathlib import Path

HERE = Path(__file__).resolve().parent
FROZEN = HERE.parent / "kd5_nonlinear"
sys.dont_write_bytecode = True  # import the frozen reference without writing into its directory
sys.path.insert(0, str(FROZEN))
import reference as kd5  # noqa: E402  (the frozen independent reference; not product code)

FROZEN_REQUEST = "u1_spring_request.json"
FROZEN_ORACLE = "u1_spring_oracle.json"
HANGER = "support:SH-C"
STIFFNESS_ONLY_BASIS = ("stiffness-only result of ../kd5_nonlinear/u1_spring_oracle.json: the hanger modelled as a "
                        "bare spring of its entered rate, its cold load not applied")
LAW_BASIS = ("exact rational solve of independently formed Euler-Bernoulli frames (../kd5_nonlinear/reference.py), "
             "every support state enumerated; a variable spring hanger is a spring of its entered rate plus its "
             "entered cold load as a constant force along the positive axis of its DOF, published support force "
             "F_cold - k*u, applied force F_cold")


class LawModel(kd5.Model):
    """The frozen reference's model plus the cold load of each variable spring hanger and the plain spring form."""

    def __init__(self, request):
        super().__init__(request)
        self.cold = []  # (support id, global DOF, F_cold)
        for s in request["model"]["supports"]:
            node = self.nodes.index(s["node"])
            if s.get("family") == "spring" and s.get("stiffness"):
                # Committed plain-spring form: the record's one restraint is its spring axis, not a rigid DOF.
                st = s["stiffness"]
                assert s["restraints"] == [st["dof"]] and not s.get("nonlinear") and not s.get("hanger")
                assert st["value"]["unit"] == "N/m"
                d = 6 * node + kd5.DOFS.index(st["dof"])
                assert self.rigid.pop(s["id"]) == [d]  # reference.py read it as a rigid restraint
                k = F(float(st["value"]["value"]))
                self.springs.append((s["id"], d, k))
                self.K[d][d] += k
            elif s.get("hanger", {}).get("hanger_type") == "variable_spring_hanger":
                h = s["hanger"]
                assert h["stiffness"]["dof"] in ("UX", "UY", "UZ") and h["cold_load"]["unit"] == "N"
                assert s["id"] not in self.rigid and not s.get("nonlinear")
                d = 6 * node + kd5.DOFS.index(h["stiffness"]["dof"])
                self.cold.append((s["id"], d, F(float(h["cold_load"]["value"]))))

    def force(self, case):
        f = super().force(case)
        for _, d, cold in self.cold:
            f[d] += cold  # the cold load acts on the structure along +DOF, in every case
        return f


def exact(x):
    return {"exact": str(x), "value": kd5.bounds(x)}


def one_way_measure(entry, state, u, r):
    """DEC-067 measure of a one-way support: the reaction when active, the lift (gap opening) when inactive."""
    s = 1 if entry["positive"] else -1
    return ("reaction", s * r[entry["dof"]]) if state == "active" else ("lift", s * u[entry["dof"]])


def derive(model, request_bytes, request_name):
    assert all(e["behavior"] == "one_way" for e in model.nonlinear), "these requests carry one-way supports only"
    cases = {}
    for case in model.cases:
        f = model.force(case)
        enumerated, qualifying = [], []
        for combo in product(*[kd5.states_for(e) for e in model.nonlinear]):
            states = {e["id"]: c[0] for e, c in zip(model.nonlinear, combo)}
            solved = kd5.solve_state(model, f, combo)
            if solved is None:
                enumerated.append({"states": states, "standing": "singular"})
                continue
            u, r = solved
            kind = kd5.classify(model, combo, u, r)
            measures = {}
            for e, c in zip(model.nonlinear, combo):
                name, value = one_way_measure(e, c[0], u, r)
                measures[e["id"]] = {"measure": name, **exact(value)}
            enumerated.append({"states": states, "standing": kind or "inadmissible", "margins": measures})
            if kind is None:
                continue
            groups, signed = kd5.published(model, combo, u, r, f)
            record = {"standing": kind, "states": states, "groups": groups, "signed": signed, "margins": measures}
            rows = {}
            for sid, d, cold in model.cold:
                k = next(k for i, dd, k in model.springs if i == sid and dd == d)
                hanger_force = cold - k * u[d]  # F_cold - k*(u - g), g = 0
                comps = [F(0)] * 6
                comps[d % 6] = hanger_force
                signed[sid]["components"] = [kd5.bounds(x) for x in comps]
                mag = kd5.sqrt_bounds(hanger_force ** 2)
                groups[sid]["node"].update(support_force=mag, support_force_magnitude=mag)
                record.setdefault("hanger_force", {})[sid] = {"slot": kd5.DOFS[d % 6], **exact(hanger_force)}
                record.setdefault("applied_force", {})[sid] = {"slot": kd5.DOFS[d % 6], **exact(cold)}
                rows[sid] = (d, hanger_force)
            # Independent closure: the support forces on the structure balance the case's external loads exactly.
            ext = kd5.Model.force(model, case)  # the case's own loads, without the cold load
            for axis in range(3):
                total = sum(ext[i] for i in range(axis, model.n, 6))
                total += sum(r[d] for d in range(axis, model.n, 6)
                             if d in {dd for v in model.rigid.values() for dd in v}
                             or any(e["dof"] == d and states[e["id"]] == "active" for e in model.nonlinear))
                total += sum(-k * u[d] for sid, d, k in model.springs if d % 6 == axis and sid not in rows)
                total += sum(v for d, v in rows.values() if d % 6 == axis)
                assert total == 0, (request_name, case["id"], axis, total)
            record["displacements"] = {node: [kd5.bounds(u[6 * i + j]) for j in range(6)]
                                       for i, node in enumerate(model.nodes)}
            qualifying.append(record)
        cases[case["id"]] = {
            "strict_states": [q for q in qualifying if q["standing"] == "strict"],
            "weak_states": [q["states"] for q in qualifying if q["standing"] == "weak"],
            "enumerated": enumerated,
        }
    return {
        "request": request_name,
        "request_sha256": hashlib.sha256(request_bytes).hexdigest(),
        "basis": LAW_BASIS,
        "normalized_sections": model.sections,
        "elastic_modulus_pa": float(model.E),
        "shear_modulus_pa": float(model.G),
        "cases": cases,
    }


def leaves(obj, path=""):
    """Flatten an oracle record to {quantity name: leaf}; a [lo, hi] binary64 pair is one leaf."""
    if isinstance(obj, list) and len(obj) == 2 and all(isinstance(x, float) for x in obj):
        return {path: obj}
    if isinstance(obj, dict):
        items = obj.items()
    elif isinstance(obj, list):
        names = kd5.DOFS if path.endswith("/components") else range(len(obj))
        items = zip(names, obj)
    else:
        return {path: obj}
    out = {}
    for key, value in items:
        out.update(leaves(value, f"{path}/{key}" if path else str(key)))
    return out


def frozen_part(case):
    """The keys the frozen oracle's case carries, with each state record cut to the frozen record's keys."""
    keys = ("standing", "states", "groups", "signed")
    return {"strict_states": [{k: q[k] for k in keys} for q in case["strict_states"]], "weak_states": case["weak_states"]}


def build(out):
    frozen_request_bytes = (FROZEN / FROZEN_REQUEST).read_bytes()
    frozen_oracle_bytes = (FROZEN / FROZEN_ORACLE).read_bytes()
    frozen_request = json.loads(frozen_request_bytes)
    frozen = json.loads(frozen_oracle_bytes)
    hanger = next(s for s in frozen_request["model"]["supports"] if s["id"] == HANGER)
    cold_load = hanger["hanger"]["cold_load"]
    written = {}

    def emit(name, obj, sort_keys):
        data = (json.dumps(obj, indent=1, sort_keys=sort_keys) + "\n").encode()
        (out / name).write_bytes(data)
        written[name] = data
        return data

    # 1. The frozen request as it stands, under the law; every changed quantity beside its stiffness-only value.
    law = derive(LawModel(frozen_request), frozen_request_bytes, f"../kd5_nonlinear/{FROZEN_REQUEST}")
    for cid, case in law["cases"].items():
        new, old = leaves(frozen_part(case)), leaves(frozen["cases"][cid])
        case["changed_from_stiffness_only"] = {
            "basis": STIFFNESS_ONLY_BASIS,
            "cold_load": {"support": HANGER, **cold_load},
            "quantities": [{"quantity": q, "value": new.get(q), "stiffness_only_value": old.get(q)}
                           for q in sorted(set(new) | set(old)) if new.get(q) != old.get(q)],
        }
    law["supersedes"] = {"file": f"../kd5_nonlinear/{FROZEN_ORACLE}",
                         "sha256": hashlib.sha256(frozen_oracle_bytes).hexdigest(),
                         "for": "expectations of u1_spring_request.json under the variable spring preload law"}
    emit("u1_spring_law_oracle.json", law, True)

    # 2. The twin: the hanger replaced by a plain spring of the same rate on the same DOF, everything else identical.
    twin = copy.deepcopy(frozen_request)
    supports = twin["model"]["supports"]
    i = next(i for i, s in enumerate(supports) if s["id"] == HANGER)
    st = hanger["hanger"]["stiffness"]
    supports[i] = {"id": hanger["id"], "label": hanger["label"], "node": hanger["node"], "family": "spring",
                   "restraints": [st["dof"]], "stiffness": copy.deepcopy(st), "provenance": hanger["provenance"]}
    assert {**twin, "model": {**twin["model"], "supports": None}} == \
        {**frozen_request, "model": {**frozen_request["model"], "supports": None}}
    assert supports[:i] + supports[i + 1:] == [s for s in frozen_request["model"]["supports"] if s["id"] != HANGER]
    twin_bytes = emit("u1_spring_twin_request.json", twin, False)
    twin_oracle = derive(LawModel(twin), twin_bytes, "u1_spring_twin_request.json")
    for cid in ("load:PRESSED", "load:LIFTED"):
        assert frozen_part(twin_oracle["cases"][cid]) == frozen["cases"][cid], f"twin {cid} differs from the frozen oracle"
    assert set(twin_oracle["cases"]) == set(frozen["cases"])
    twin_oracle["reproduces"] = {"file": f"../kd5_nonlinear/{FROZEN_ORACLE}",
                                 "sha256": hashlib.sha256(frozen_oracle_bytes).hexdigest(),
                                 "asserted": "strict_states (standing, states, groups, signed) and weak_states of "
                                             "load:PRESSED and load:LIFTED equal exactly, by computation"}
    emit("u1_spring_twin_oracle.json", twin_oracle, True)

    # 3. The frozen request's hanger with its cold load beside a one-way support kept pressed, and a lifted case.
    pre = copy.deepcopy(frozen_request)
    pre["model"]["project"].update(id="project:kd5-nl-u1p", name="K-D5 nonlinear U-1 preloaded hanger beside a "
                                   "one-way support")
    template = frozen_request["model"]["load_cases"][0]
    cases = []
    for cid, fz in (("PRESSED-PRELOAD", -1000), ("LIFTED-PRELOAD", -200)):
        c = copy.deepcopy(template)
        c.update(id=f"load:{cid}", label=cid)
        c["primitive_loads"][0].update(id=f"load:{cid}-0")
        c["primitive_loads"][0]["magnitude"]["value"] = fz
        cases.append(c)
    pre["model"]["load_cases"] = cases
    pre_bytes = emit("u1_preload_request.json", pre, False)
    emit("u1_preload_oracle.json", derive(LawModel(pre), pre_bytes, "u1_preload_request.json"), True)

    sums = "".join(f"{hashlib.sha256(data).hexdigest()}  {name}\n" for name, data in sorted(written.items()))
    (out / "SHA256SUMS").write_text(sums)
    written["SHA256SUMS"] = sums.encode()

    # Every expected state is strict and unique; report each case.
    for name in ("u1_spring_law_oracle.json", "u1_spring_twin_oracle.json", "u1_preload_oracle.json"):
        oracle = json.loads(written[name])
        for cid, case in oracle["cases"].items():
            assert len(case["strict_states"]) == 1 and not case["weak_states"], (name, cid, "not one strict state")
            q = case["strict_states"][0]
            print(f"{name} {cid}: " + ", ".join(f"{sid} {st} ({q['margins'][sid]['measure']} "
                                                f"{q['margins'][sid]['exact']})" for sid, st in q["states"].items()))
    return written


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="regenerate into a temporary directory and compare (default)")
    mode.add_argument("--write", action="store_true", help="replace the outputs here")
    args = parser.parse_args()
    if args.write:
        build(HERE)
        return
    with tempfile.TemporaryDirectory() as tmp:
        written = build(Path(tmp))
    for name, data in written.items():
        assert (HERE / name).read_bytes() == data, f"{name} differs from its regeneration"
    print("check: all outputs reproduce byte for byte")


if __name__ == "__main__":
    main()
