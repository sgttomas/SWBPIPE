#!/usr/bin/env python3
"""Model family for the sparse active-set loop's memory measurement (DEL-04-07 REQ-008, part 1: evidence only).

Writes product model documents (canonical JSON: sorted keys, no whitespace) and `manifest.json` into an output
directory. `tests/sparse_loop_memory.rs` (the loop-level counting-allocator binary) and the product crate's
`tests/sparse_admission_memory_twin.rs` (the whole-call twin) read the manifest from the directory named by
`SWBPIPE_SPARSE_MEMORY_MODELS`.

Every value is invented for this measurement: no library, supplier, standard or project data. The racks come from
the maintained generator `core/solver/performance_harness/rack/rack_model.py`, unchanged; the other families are
built here. Each manifest entry names the levels it is measured at:

- `loop`: translated by the loop-level test into a `NonlinearFrameSolveInput` and solved by the sparse loop;
- `product`: the product's headless entry in sparse mode, whole call;
- `refusal`: expected to be refused by the memory guard (the loop entry's and the product's).

Usage: python3 generate_models.py OUTPUT_DIR [--quick]
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import random
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[5]
RACK = ROOT / "core" / "solver" / "performance_harness" / "rack" / "rack_model.py"
PROV = "invented_sparse_loop_memory_measurement_not_library_or_project_data"
UNITS = {"angle": "rad", "force": "N", "length": "m", "moment": "N*m", "pressure": "Pa", "stress": "Pa",
         "temperature": "degC"}
E, G, ALPHA = 2.0e11, 8.0e10, 1.2e-5
OD, WALL = 0.27, 0.009
SPAN = 1.5
ALL6 = ["UX", "UY", "UZ", "RX", "RY", "RZ"]


def load_rack():
    spec = importlib.util.spec_from_file_location("rack_model", RACK)
    module = importlib.util.module_from_spec(spec)
    sys.modules["rack_model"] = module
    spec.loader.exec_module(module)
    return module


def canonical(document: dict) -> bytes:
    return json.dumps(document, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")


def q(value, unit):
    return {"value": value, "unit": unit}


def node(nid, x, y, z):
    return {"id": nid, "position": {"x": x, "y": y, "z": z}, "provenance": PROV}


def pipe(pid, a, b, y_ref=(0.0, 0.0, 1.0)):
    return {"id": pid, "from": a, "to": b, "material": "material:SLM",
            "section": {"outside_diameter": q(OD, "m"), "wall_thickness": q(WALL, "m")},
            "y_reference": {"x": y_ref[0], "y": y_ref[1], "z": y_ref[2]}, "provenance": PROV}


def rigid(sid, nid, family, restraints):
    return {"id": sid, "node": nid, "family": family, "restraints": restraints, "provenance": PROV}


def spring(sid, nid, dof, k):
    unit = "N/m" if dof.startswith("U") else "N*m/rad"
    return {"id": sid, "node": nid, "family": "spring", "restraints": [dof],
            "stiffness": {"dof": dof, "value": q(k, unit)}, "provenance": PROV}


def nonlinear(sid, nid, spec):
    return {"id": sid, "node": nid, "family": "nonlinear", "restraints": [], "nonlinear": spec, "provenance": PROV}


def lift_off(state):
    return {"behavior": "lift_off", "contact_when": "positive_reaction", "dof": "UZ", "initial_state": state}


def gap(dof, clearance, state="inactive"):
    return {"behavior": "gap", "closes_when": "positive_displacement", "dof": dof, "gap": q(clearance, "m"),
            "initial_state": state}


def friction(dof, mu, source):
    return {"behavior": "friction", "dof": dof, "friction_coefficient": q(mu, "none"), "initial_state": "sticking",
            "normal_reaction_source": {"dof": "UZ", "support_ref": source}}


def force(lid, nid, direction, value):
    return {"id": lid, "category": "concentrated_force", "target": {"type": "node", "node": nid},
            "direction": direction, "magnitude": q(value, "N"), "dimension": "force", "provenance": PROV}


def weight(lid, pid, value):
    return {"id": lid, "category": "weight", "target": {"type": "element", "pipe": pid}, "direction": "global_z",
            "magnitude": q(value, "N/m"), "dimension": "force_per_length", "provenance": PROV}


def thermal(lid, pid, dt):
    return {"id": lid, "category": "thermal", "target": {"type": "element", "pipe": pid}, "direction": "global_x",
            "magnitude": q(dt, "degC"), "dimension": "temperature_interval", "provenance": PROV}


def document(name, nodes, pipes, supports, cases, combinations=(), components=()):
    return {"model": {
        "schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "analysis_status": {"mechanics": "ready_for_preview_diagnostics",
                            "rule_check": "not_performed_user_rule_inputs_missing",
                            "professional_acceptance": "not_provided"},
        "project": {"id": f"project:slm-{name}", "units": UNITS},
        "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": list(components),
        "materials": [{"id": "material:SLM", "elastic_modulus": q(E, "Pa"), "shear_modulus": q(G, "Pa"),
                       "thermal_expansion_coefficient": q(ALPHA, "1/degC"), "provenance": PROV}],
        "load_cases": cases, "combinations": list(combinations)}, "materials": []}


def case(cid, loads):
    return {"id": cid, "label": cid, "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV}


def chain(count, prefix="N"):
    nodes = [node(f"node:{prefix}{i}", SPAN * i, 0.0, 0.0) for i in range(count)]
    pipes = [pipe(f"pipe:{prefix}{i}", f"node:{prefix}{i - 1}", f"node:{prefix}{i}") for i in range(1, count)]
    return nodes, pipes


# -- families --------------------------------------------------------------------------------------------------------

def renumbered(document: dict, seed: int) -> dict:
    """The same document with its node list permuted by a seeded shuffle (node indices are document order)."""
    out = json.loads(json.dumps(document))
    model = out["model"] if "model" in out else out
    rng = random.Random(seed)
    rng.shuffle(model["nodes"])
    return out


def wrap_rack(document: dict) -> dict:
    """The request the release timing driver sends: the model and an empty materials list."""
    return {"model": document, "materials": []}


def gap_frame(count):
    """Gap-only straight frame: a chain along x anchored at node 0, a UY gap at every third node, a +y tip load."""
    nodes, pipes = chain(count)
    supports = [rigid("support:A0", "node:N0", "anchor", ALL6)]
    for i in range(3, count, 3):
        supports.append(nonlinear(f"support:GP{i}", f"node:N{i}", gap("UY", 0.0005)))
    loads = [force("load:tip", f"node:N{count - 1}", "global_y", 400.0)]
    loads += [force(f"load:y{i}", f"node:N{i}", "global_y", 150.0) for i in range(1, count)]
    return document(f"gap-frame-{count}", nodes, pipes, supports, [case("load:OPE", loads)])


def grid(rows, cols, name, every=5):
    """A planar frame grid (z = 0) of rows x cols nodes, members along x and y (cross-ties), anchored at the four
    corners, with a lift-off UZ rest at every `every`-th node and a downward weight on every member."""
    nid = lambda r, c: f"node:G{r:03d}-{c:03d}"
    nodes = [node(nid(r, c), SPAN * c, SPAN * r, 0.0) for r in range(rows) for c in range(cols)]
    pipes = []
    for r in range(rows):
        for c in range(cols):
            if c + 1 < cols:
                pipes.append(pipe(f"pipe:GX{r:03d}-{c:03d}", nid(r, c), nid(r, c + 1)))
            if r + 1 < rows:
                pipes.append(pipe(f"pipe:GY{r:03d}-{c:03d}", nid(r, c), nid(r + 1, c)))
    corners = {(0, 0), (0, cols - 1), (rows - 1, 0), (rows - 1, cols - 1)}
    supports = [rigid(f"support:A{r}-{c}", nid(r, c), "anchor", ALL6) for r, c in sorted(corners)]
    index = 0
    for r in range(rows):
        for c in range(cols):
            if (r, c) in corners:
                continue
            index += 1
            if index % every == 0:
                supports.append(nonlinear(f"support:LO{r}-{c}", nid(r, c), lift_off("active")))
    loads = [weight(f"load:W-{p['id'][5:]}", p["id"], -1000.0) for p in pipes]
    return document(name, nodes, pipes, supports, [case("load:OPE", loads)])


def every_node_lift_off(count, recovery):
    """Every node a nonlinear support. `recovery`: no rigid UZ restraint anywhere and every rest seeded inactive, so
    the first iterate is singular and the loop takes its one all-active recovery trial; otherwise anchored at both
    ends, rests seeded inactive under alternating loads so that states change between iterates."""
    nodes, pipes = chain(count)
    supports = []
    if recovery:
        supports.append(rigid("support:G0", "node:N0", "anchor", ["UX", "UY", "RX", "RY", "RZ"]))
        supports.append(rigid(f"support:G{count - 1}", f"node:N{count - 1}", "line_stop", ["UY"]))
        rest = range(count)
    else:
        supports.append(rigid("support:A0", "node:N0", "anchor", ALL6))
        supports.append(rigid(f"support:A{count - 1}", f"node:N{count - 1}", "anchor", ALL6))
        rest = range(1, count - 1)
    for i in rest:
        supports.append(nonlinear(f"support:LO{i}", f"node:N{i}", lift_off("inactive")))
    loads = []
    for i in range(count):
        # Mostly downward, with every seventh node pulled up harder than its neighbours push down.
        value = 6000.0 if i % 7 == 3 else -1500.0
        loads.append(force(f"load:z{i}", f"node:N{i}", "global_z", value))
    return document(f"every-node-lift-off-{'recovery' if recovery else 'flip'}-{count}", nodes, pipes, supports,
                    [case("load:OPE", loads)])


def friction_many(count):
    """Derived-normal friction at every node but the anchor: a vertical support UZ (the normal source) and a
    friction UX shoe at each, a chain anchored at node 0 and free at its tip, weight, and an axial force at every
    node above mu times its normal reaction, so that the shoes slide and the derived-normal coupling is formed."""
    nodes, pipes = chain(count)
    supports = [rigid("support:A0", "node:N0", "anchor", ALL6)]
    for i in range(1, count):
        supports.append(rigid(f"support:V{i}", f"node:N{i}", "vertical_support", ["UZ"]))
        supports.append(nonlinear(f"support:FR{i}", f"node:N{i}", friction("UX", 0.3, f"support:V{i}")))
    loads = [weight(f"load:W{i}", p["id"], -1200.0) for i, p in enumerate(pipes)]
    loads += [force(f"load:x{i}", f"node:N{i}", "global_x", 1500.0) for i in range(1, count)]
    return document(f"friction-many-{count}", nodes, pipes, supports, [case("load:OPE", loads)])


def springs_chain(count):
    """Linear springs (UY and RZ at every node) beside lift-off rests at every fourth node and two anchors."""
    nodes, pipes = chain(count)
    supports = [rigid("support:A0", "node:N0", "anchor", ALL6),
                rigid(f"support:A{count - 1}", f"node:N{count - 1}", "anchor", ALL6)]
    for i in range(1, count - 1):
        supports.append(spring(f"support:SY{i}", f"node:N{i}", "UY", 2.0e5))
        supports.append(spring(f"support:SR{i}", f"node:N{i}", "RZ", 5.0e4))
        if i % 4 == 0:
            supports.append(nonlinear(f"support:LO{i}", f"node:N{i}", lift_off("active")))
    loads = [weight(f"load:W{i}", p["id"], -1500.0) for i, p in enumerate(pipes)]
    loads += [force(f"load:y{i}", f"node:N{i}", "global_y", 300.0 if i % 2 else -300.0) for i in range(count)]
    return document(f"springs-{count}", nodes, pipes, supports, [case("load:OPE", loads)])


def skew_with_nonlinear(count):
    """A skew (direction-authored) line stop beside a lift-off rest: the product refuses the combination."""
    nodes, pipes = chain(count)
    supports = [rigid("support:A0", "node:N0", "anchor", ALL6),
                {"id": "support:SK", "node": f"node:N{count // 2}", "family": "line_stop", "restraints": ["UY"],
                 "direction": {"x": 3.0, "y": 4.0, "z": 0.0}, "provenance": PROV},
                nonlinear("support:LO", f"node:N{count - 1}", lift_off("active"))]
    loads = [weight(f"load:W{i}", p["id"], -1000.0) for i, p in enumerate(pipes)]
    return document(f"skew-nonlinear-{count}", nodes, pipes, supports, [case("load:OPE", loads)])


def multicase(rack, rack_document: dict, name: str, bases: list[float], material: dict, gaps: bool = True) -> dict:
    """The pressure-free rack with, at each modulus basis temperature t, a sustained case SUS_t (weight) and an
    operating case OPE_t (weight + thermal), and a held-status (`minuend_final`) subtraction OPE_t - SUS_t (each
    pair shares its basis, as gate G-2 requires). `material` replaces the rack's (the pressure-free rack's material
    has no temperature points, so a modulus basis needs the exact rack's E/nu points), with an explicit shear
    modulus E/2.6 on the material and on each point, as the pressure-free route requires."""
    out = json.loads(json.dumps(rack_document))
    model = out["model"]
    shear = lambda record: q(rack.r6(record["elastic_modulus"]["value"] / 2.6), "GPa")
    material = dict(material, shear_modulus=shear(material),
                    temperature_points=[dict(point, shear_modulus=shear(point)) for point in material["temperature_points"]])
    model["materials"] = [material]
    if not gaps:
        # A gap support leaves a minuend unqualified (strict-gap proof unavailable on the rack), and a held-status
        # solve needs a qualified minuend: without gaps the held solves run.
        model["supports"] = [s for s in model["supports"] if (s.get("nonlinear") or {}).get("behavior") != "gap"]
    (ope,) = model["load_cases"]
    weights = [load for load in ope["primitive_loads"] if load["category"] == "weight"]
    thermals = [load for load in ope["primitive_loads"] if load["category"] == "thermal"]

    def renamed(loads, tag):
        result = []
        for load in loads:
            load = json.loads(json.dumps(load))
            load["id"] = load["id"].replace("RK-OPE", f"RK-{tag}")
            result.append(load)
        return result

    base = {key: value for key, value in ope.items() if key not in ("primitive_loads", "id", "label")}
    cases, combinations = [], []
    for index, temperature in enumerate(bases):
        sus, opr = f"SUS{index}", f"OPE{index}"
        cases.append(dict(base, id=f"load:RK-{sus}", label=f"{sus} sustained: W at {temperature} degC",
                          primitive_loads=renamed(weights, sus), modulus_basis_temperature=q(temperature, "degC")))
        cases.append(dict(base, id=f"load:RK-{opr}", label=f"{opr} operating: W + T at {temperature} degC",
                          primitive_loads=renamed(weights, opr) + renamed(thermals, opr),
                          modulus_basis_temperature=q(temperature, "degC")))
        combinations.append({"id": f"comb:RK-EXP{index}", "label": f"{opr} - {sus} held",
                             "basis": "result_state_subtraction", "minuend_id": f"load:RK-{opr}",
                             "subtrahend_id": f"load:RK-{sus}", "difference_basis": "operating_minus_sustained",
                             "subtrahend_support_state": "minuend_final", "terms": [], "provenance": PROV})
    model["load_cases"] = cases
    model["combinations"] = combinations
    model["project"]["id"] = f"project:slm-{name}"
    return out


def build(quick: bool):
    rack = load_rack()
    entries = []

    def add(name, family, levels, request, how, extra=None):
        entries.append((name, family, levels, request, how, extra or {}))

    def bare(document):
        """The rack generator's own document hash (what `rack_model.py -o` writes)."""
        return {"rack_document_sha256": hashlib.sha256(rack.canonical(document)).hexdigest()}

    sizes = [250, 1000] if quick else [250, 1000, 1500, 1750]
    for profile in ("nonlinear-ex", "nonlinear-pf"):
        for nodes in sizes:
            document, _ = rack.build(profile, nodes)
            add(f"rack-{profile[10:]}-{nodes}", "reference rack", ["loop", "product"], wrap_rack(document),
                f"rack_model.py --profile {profile} --nodes {nodes}, wrapped as {{model, materials: []}}", bare(document))
    for profile in ("nonlinear-ex", "nonlinear-pf"):
        document, _ = rack.build(profile, 1775)
        add(f"rack-{profile[10:]}-1775", "refusal (guard edge)", ["refusal"], wrap_rack(document),
            f"rack_model.py --profile {profile} --nodes 1775 (first refused size)", bare(document))
    document, _ = rack.build("nonlinear-ex", 1000)
    for seed in (1, 2):
        add(f"rack-ex-1000-renum-s{seed}", "renumbered rack", ["loop", "product"],
            renumbered(wrap_rack(document), seed), f"rack-ex-1000 with its node list shuffled by random.Random({seed})")
    if not quick:
        document, _ = rack.build("nonlinear-ex", 1750)
        add("rack-ex-1750-renum-s1", "renumbered rack", ["loop", "product"], renumbered(wrap_rack(document), 1),
            "rack-ex-1750 with its node list shuffled by random.Random(1)")
    grids = [(20, 20), (30, 30)] if quick else [(20, 20), (30, 30), (41, 41)]
    for rows, cols in grids:
        add(f"grid-{rows}x{cols}", "RCM-hostile grid", ["loop", "product"], grid(rows, cols, f"grid-{rows}x{cols}"),
            f"planar frame grid {rows} x {cols}, members along x and y, corner anchors, lift-off at every 5th node")
    add("grid-30x30-renum-s1", "RCM-hostile grid, renumbered", ["loop", "product"],
        renumbered(grid(30, 30, "grid-30x30-renum-s1"), 1), "grid-30x30 with its node list shuffled by random.Random(1)")
    for count in (400, 1000) if not quick else (400,):
        add(f"every-node-lift-off-flip-{count}", "every node nonlinear", ["loop", "product"],
            every_node_lift_off(count, False), f"chain of {count} nodes, anchors at both ends, inactive lift-off rest "
                                              "at every interior node, alternating nodal loads")
        add(f"every-node-lift-off-recovery-{count}", "every node nonlinear (recovery trial)", ["loop", "product"],
            every_node_lift_off(count, True), f"chain of {count} nodes, no rigid UZ (partial anchor at node 0), "
                                              "inactive lift-off rest at every node (singular seed)")
    for count in (500, 1500) if not quick else (500,):
        add(f"friction-many-{count}", "derived-normal friction", ["loop", "product"], friction_many(count),
            f"chain of {count} nodes, vertical support and derived-normal friction UX at every node but the anchor, "
            "axial nodal forces above mu N")
    for count in (42, 43):
        add(f"gap-frame-{count}", "gap-only straight frame", ["loop", "product"], gap_frame(count),
            f"straight chain of {count} nodes ({6 * count} DOFs), anchor at node 0, UY gaps at every third node")
    add("springs-1000", "springs", ["loop", "product"], springs_chain(1000),
        "chain of 1000 nodes, UY and RZ springs at every interior node, lift-off rest at every fourth")
    add("skew-nonlinear-50", "skew restraint with a nonlinear support", ["product"], skew_with_nonlinear(50),
        "chain of 50 nodes with a direction-authored line stop and a lift-off rest")
    pf = {size: rack.build("nonlinear-pf", size)[0] for size in ((250, 1000) if quick else (250, 1000, 1750))}
    for size, document in pf.items():
        add(f"multicase-pf-{size}", "multi-case, held status, several modulus bases", ["product"],
            multicase(rack, wrap_rack(document), f"multicase-pf-{size}", [180.0, 150.0, 250.0], rack.material(True)),
            f"rack-pf-{size} with the exact rack's material (E/nu temperature points); at each of 180, 150 and 250 "
            "degC modulus bases SUSi (W) and OPEi (W + T); three minuend_final subtractions OPEi - SUSi")
    for size in (250, 1000) if quick else (250, 1000, 1750):
        document = pf[size]
        add(f"multicase-pf-nogap-{size}", "multi-case, held status solved, several modulus bases", ["product"],
            multicase(rack, wrap_rack(document), f"multicase-pf-nogap-{size}", [180.0, 150.0, 250.0],
                      rack.material(True), gaps=False),
            f"multicase-pf-{size} with the rack's gap supports removed, so that each minuend is qualified and its "
            "held-status solve runs")
    return entries


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("output", type=Path)
    parser.add_argument("--quick", action="store_true", help="small sizes only (a smoke run)")
    args = parser.parse_args(argv)
    args.output.mkdir(parents=True, exist_ok=True)
    manifest = []
    for name, family, levels, request, how, extra in build(args.quick):
        data = canonical(request)
        path = args.output / f"{name}.json"
        path.write_bytes(data)
        manifest.append({"name": name, "family": family, "levels": levels, "file": path.name,
                         "sha256": hashlib.sha256(data).hexdigest(), "bytes": len(data), "generated_by": how,
                         **extra})
    (args.output / "manifest.json").write_bytes(canonical({"generator": "generate_models.py", "models": manifest}))
    with open(args.output / "SHA256SUMS", "w", encoding="utf-8") as sums:
        for entry in manifest:
            sums.write(f"{entry['sha256']}  {entry['file']}\n")
    print(json.dumps({"models": len(manifest), "output": str(args.output)}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
