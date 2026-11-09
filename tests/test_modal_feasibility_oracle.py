"""DEL-04-09 I3a: the readers against the feasibility oracle (`tests/modal_feasibility_oracle.py`,
Design §5.3.4's dependency table).

Generated tamperings of small committed fixtures, with a fixed seed: an operand nulled
with each reason, flagged below-normal, negated, zeroed or scaled; one or two per case.
For each case the oracle decides, family by family, whether ANY value within the stated
sets satisfies every relation instance. It is supporting evidence, not the governing
check. A reader never fails a family the oracle finds feasible (a false refusal), and
never passes or scopes one the oracle proves infeasible: it fails it, or, where a
flagged value's extended exponent puts the decision below binary64's reach, leaves it
`unresolved`. The other standings must not fail. The producer's own edge output
(`fixtures/results/modal_producer_edges.json`) is the governing false-refusal control:
both readers and the oracle accept every edge.

The cases and the oracle's verdicts are committed as `fixtures/results/modal_oracle_cases.json`.
This test requires the committed file to be the regeneration, and it holds the Python
reader to it. The Rust reader is held to the same file in
`core/reporting/modal_result/tests/corpus.rs`. To regenerate it, set
MODAL_ORACLE_WRITE=1. All values are invented.
"""
from __future__ import annotations

import json
import os
import random
from pathlib import Path

from tests import modal_feasibility_oracle as oracle
from tests.test_modal_evidence import entry_text, load
from tests.verifiers import modal_evidence as me

ROOT = Path(__file__).resolve().parents[1]
CASES = ROOT / "fixtures/results/modal_oracle_cases.json"
FIXTURES = ("component_body", "arc", "state_friction_sliding")
SEED = 20261104
SINGLES, PAIRS = 110, 60
REASONS = ("outside_canonical_profile", "not_finite", "total_mass_zero", "not_factored", "no_neighbour", "not_formed")
OTHER = ("normalization", "counts", "units")


def _targets(doc):
    """(JSON pointer of a value object, the value object) for every operand the oracle models."""
    t, out = doc["tables"], []

    def table(name, fields):
        for i, row in enumerate(t[name]["rows"]):
            for f in fields:
                out.append((f"/tables/{name}/rows/{i}/{f}", row[f]))
    table("mass_matrix", ("value",))
    table("mass_totals", ("total_mass", "free_dof_mass"))
    table("modes", ("generalized_mass", "cluster_error_estimate"))
    table("participation", ("participation_factor", "effective_mass", "effective_mass_fraction", "cumulative_effective_mass_fraction"))
    table("computed_modes", ("backward_error", "condition", "error_estimate"))
    table("clusters", ("error_estimate",))
    table("sturm_candidates", ("shift", "formation_term", "factor_term", "backward_error", "inverse_norm_estimate",
                               "conditioning_product", "separation_ratio", "growth"))
    table("identity", ("relative",))
    out.append(("/orthogonality/max_ratio", doc["orthogonality"]["max_ratio"]))
    return out


def _shape_targets(doc):
    """Free shape components published as an exact 0: flagging one is producer output (a
    nonzero component below the normal range, which m and φᵀp_k take as its published 0)."""
    free = [r["free_index"] is not None for r in doc["tables"]["dofs"]["rows"]]
    ndof = len(free)
    return [f"/tables/shapes/rows/{i}/value" for i, row in enumerate(doc["tables"]["shapes"]["rows"])
            if free[i % ndof] and row["value"]["value"] == 0.0 and not row["value"]["below_normal"]]


def _tamper(rng, path, vo):
    """Corpus operations that change one value object; None when the change is not
    representable (a value the checked profile cannot carry)."""
    kind = rng.choice(("reason", "reason", "below", "negate", "zero", "scale", "scale"))
    if kind == "reason":
        return [{"op": "replace", "path": path + "/value", "value": None},
                {"op": "replace", "path": path + "/below_normal", "value": False},
                {"op": "replace", "path": path + "/undefined_reason", "value": rng.choice(REASONS)}]
    if vo["value"] is None:
        return None
    if kind == "below":
        return [{"op": "replace", "path": path + "/value", "value": 0.0},
                {"op": "replace", "path": path + "/below_normal", "value": True}]
    if vo["below_normal"]:
        return None
    if kind == "negate":
        new = -vo["value"]
    elif kind == "zero":
        new = 0.0
    else:
        new = vo["value"] * rng.choice((1.000001, 1.5, 0.5, 1e-20, 1e12, -1.0))
    if new == 0.0 and vo["value"] == 0.0 or abs(new) >= 2.0 ** 53 or new == vo["value"]:
        return None
    if new == 0.0:
        new = 0.0
    return [{"op": "replace", "path": path + "/value", "value": new}]


def generate():
    rng = random.Random(SEED)
    cases = []
    for name in FIXTURES:
        document = f"modal_{name}.json"
        base = load(document)
        targets = _targets(base)
        shapes = _shape_targets(base)
        for path in rng.sample(shapes, min(4, len(shapes))):
            cases.append({"id": f"oracle:{name}:{len(cases)}", "document": document, "request": f"modal_{name}.request.json",
                          "request_operations": [], "reseal": True,
                          "operations": [{"op": "replace", "path": path + "/below_normal", "value": True}]})
        for count, size in ((SINGLES, 1), (PAIRS, 2)):
            made = 0
            while made < count:
                operations = []
                for path, vo in rng.sample(targets, size):
                    ops = _tamper(rng, path, vo)
                    if ops is None:
                        operations = None
                        break
                    operations += ops
                if operations is None:
                    continue
                made += 1
                cases.append({"id": f"oracle:{name}:{len(cases)}", "document": document,
                              "request": f"modal_{name}.request.json", "request_operations": [],
                              "operations": operations, "reseal": True})
    cases += _reproductions() + _coordinated()
    for case in cases:
        text = entry_text(case)
        case["oracle"] = oracle.verdict(json.loads(text))
        # A flagged value's magnitude can lie below binary64's range: there a reader may
        # leave an infeasible family unresolved rather than decide it.
        case["unresolved_allowed"] = '"below_normal": true' in text
    return {"seed": SEED, "fixtures": list(FIXTURES), "cases": cases}


def _large(path):
    return [{"op": "replace", "path": path + "/value", "value": None},
            {"op": "replace", "path": path + "/undefined_reason", "value": "outside_canonical_profile"}]


def _coordinated():
    """Tamperings chosen to stay feasible, so false refusals are tested where the readers
    must scope: two couplings outside the profile in one direction's block (their sum can
    be anything), and a not-finite chain η → e_λ → its cluster's e_λ."""
    out = []
    arc = load("modal_arc.json")
    rows = arc["tables"]["mass_matrix"]["rows"]
    for k in ("ux", "uy", "uz"):
        a = next(i for i, r in enumerate(rows) if r["row_id"] == f"mass:dof:a:{k}|dof:b:{k}")
        b = next(i for i, r in enumerate(rows) if r["row_id"] == f"mass:dof:c:{k}|dof:d:{k}")
        out.append((f"two_large_couplings_along_{k[1]}", "modal_arc.json",
                    _large(f"/tables/mass_matrix/rows/{a}/value") + _large(f"/tables/mass_matrix/rows/{b}/value")))
    for name in ("component_body", "arc"):
        nan = []
        for path in ("/tables/computed_modes/rows/0/backward_error", "/tables/computed_modes/rows/0/error_estimate",
                     "/tables/clusters/rows/0/error_estimate", "/tables/modes/rows/0/cluster_error_estimate"):
            nan += [{"op": "replace", "path": path + "/value", "value": None},
                    {"op": "replace", "path": path + "/undefined_reason", "value": "not_finite"}]
        out.append((f"not_finite_chain_{name}", f"modal_{name}.json", nan))
    return [{"id": f"coordinated:{name}", "document": document, "request": document.replace(".json", ".request.json"),
             "request_operations": [], "operations": ops, "reseal": True} for name, document, ops in out]


def _reproductions():
    """The reviewers' reproductions (third review of f063e5532) as oracle cases."""
    body = "modal_component_body.json"
    doc = load(body)
    rows = doc["tables"]["participation"]["rows"]
    tamper = []
    # 1b: a restrained rotational diagonal mass entry outside the profile, and Γ's sign reversed.
    tamper.append(("restrained_entry_beside_a_reversed_gamma", body, _large("/tables/mass_matrix/rows/5/value") + [
        {"op": "replace", "path": "/tables/participation/rows/2/participation_factor/value",
         "value": -rows[2]["participation_factor"]["value"]}]))
    # 2a: T_x outside the profile, its fractions claiming T_x = 0, the payload withheld.
    ops = [{"op": "withhold", "table": "mass_matrix"}] + _large("/tables/mass_totals/rows/0/total_mass")
    for m in range(len(doc["tables"]["modes"]["rows"])):
        for field in ("effective_mass_fraction", "cumulative_effective_mass_fraction"):
            ops += [{"op": "replace", "path": f"/tables/participation/rows/{3 * m}/{field}/value", "value": None},
                    {"op": "replace", "path": f"/tables/participation/rows/{3 * m}/{field}/undefined_reason", "value": "total_mass_zero"}]
    tamper.append(("large_total_beside_total_mass_zero_claims", body, ops))
    # 2b: T_x outside the profile, fractions m_eff/10²⁰, the last cumulative x fraction 0.75.
    ops = [{"op": "withhold", "table": "mass_matrix"}] + _large("/tables/mass_totals/rows/0/total_mass")
    running = 0.0
    n = len(doc["tables"]["modes"]["rows"])
    for m in range(n):
        fraction = rows[3 * m]["effective_mass"]["value"] / 1e20
        running += fraction
        ops += [{"op": "replace", "path": f"/tables/participation/rows/{3 * m}/effective_mass_fraction/value", "value": fraction},
                {"op": "replace", "path": f"/tables/participation/rows/{3 * m}/cumulative_effective_mass_fraction/value",
                 "value": 0.75 if m == n - 1 else running}]
    tamper.append(("large_total_beside_a_wrong_cumulative_fraction", body, ops))
    # 3: zero factor and formation terms and e_b, ν outside the profile, a positive product.
    c = "/tables/sturm_candidates/rows/0/"
    ops = [{"op": "replace", "path": c + f + "/value", "value": 0.0} for f in ("factor_term", "formation_term", "backward_error")]
    ops += _large(c + "inverse_norm_estimate")
    tamper.append(("zero_backward_error_times_a_large_inverse_norm", body, ops))
    # 4: the payload withheld, mode 1's m below-normal, its Γs outside the profile.
    ops = [{"op": "withhold", "table": "mass_matrix"},
           {"op": "replace", "path": "/tables/modes/rows/0/generalized_mass/value", "value": 0.0},
           {"op": "replace", "path": "/tables/modes/rows/0/generalized_mass/below_normal", "value": True}]
    for k in range(3):
        ops += _large(f"/tables/participation/rows/{k}/participation_factor")
    tamper.append(("below_normal_mass_beside_large_gammas", body, ops))
    # 1a: the committed coupling case: one coupling outside the profile cannot close T_x.
    tamper.append(("one_large_coupling", body, _large("/tables/mass_matrix/rows/8/value")))
    return [{"id": f"reproduction:{name}", "document": document, "request": document.replace(".json", ".request.json"),
             "request_operations": [], "operations": ops, "reseal": True} for name, document, ops in tamper]


def reader_disagreements(case, verdict):
    """The families where a reader's verdict is not compatible with the oracle's."""
    out = []
    if verdict["closure"]["status"] != "passed":
        return [f"closure {verdict['closure']['status']}: {verdict['closure']['difference']}"]
    for family, truth in case["oracle"].items():
        standing = verdict["relations"][family]
        status = standing["status"]
        if truth == "infeasible":
            agrees = status == "failed" or (status == "unresolved" and case["unresolved_allowed"])
        else:
            agrees = status != "failed"
        if not agrees:
            out.append(f"{family}: oracle {truth}, reader {status} ({standing['difference']})")
    for name in oracle_other():
        if verdict["relations"][name]["status"] == "failed":
            out.append(f"{name}: failed ({verdict['relations'][name]['difference']})")
    return out


def oracle_other():
    return OTHER


def test_the_committed_cases_are_the_oracles_regeneration():
    generated = generate()
    if os.environ.get("MODAL_ORACLE_WRITE") == "1":
        CASES.write_text(json.dumps(generated, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    committed = json.loads(CASES.read_text(encoding="utf-8"))
    assert committed == generated
    assert len(committed["cases"]) >= 500


def test_the_python_reader_agrees_with_the_oracle():
    cases = json.loads(CASES.read_text(encoding="utf-8"))["cases"]
    failures = []
    for case in cases:
        text = entry_text(case)
        # The oracle decides the payload's own feasibility: compared without the request.
        for problem in reader_disagreements(case, me.read_text(text, None)):
            failures.append(f"{case['id']}: {problem}")
        # With the bound request the model-derived mass is held as well: never weaker than the
        # oracle (an infeasible family still fails), though it may fail what the payload admits.
        bound = me.read_text(text, load(case["request"]))
        for family, truth in case["oracle"].items():
            status = bound["relations"][family]["status"]
            if truth == "infeasible" and not (status == "failed" or (status == "unresolved" and case["unresolved_allowed"])):
                failures.append(f"{case['id']}: {family}: oracle infeasible, reader with the request {status}")
    assert not failures, f"{len(failures)} disagreements:\n" + "\n".join(failures[:60])
    infeasible = sum(1 for c in cases if "infeasible" in c["oracle"].values())
    assert 0 < infeasible < len(cases), "the cases reach both verdicts"


EDGES = ROOT / "fixtures/results/modal_producer_edges.json"


def test_the_producers_edges_are_accepted_by_the_python_reader_and_the_oracle():
    entries = json.loads(EDGES.read_text(encoding="utf-8"))["entries"]
    assert len(entries) >= 3
    for entry in entries:
        text = entry_text(entry)
        # Invented values at the payload's arithmetic edges, not the bound model's mass: read
        # without the request (the payload domain).
        verdict = me.read_text(text, None)
        assert verdict["decision"] != "rejected", (entry["id"], verdict)
        assert verdict["validation"]["status"] == "not_fully_validated", entry["id"]
        assert all(v == "feasible" for v in oracle.verdict(json.loads(text)).values()), entry["id"]


def test_the_oracle_accepts_the_producers_generalized_mass_probe():
    """m = 2⁻¹²⁰⁰ from `normalize_mode` (modal_quantities.rs:294), a flagged 0: the oracle's
    exact sum admits it."""
    probe = json.loads(EDGES.read_text(encoding="utf-8"))["relation_probes"][0]
    shape = {dof: vo["value"] for dof, vo in zip(probe["global_dofs"], probe["shape"])}
    (i, j, value), = probe["mass_entries"]
    from fractions import Fraction
    terms = [(Fraction(1) * Fraction(shape[i]) * Fraction(shape[j]),
              {"value": value, "below_normal": False, "undefined_reason": None}, i, j)]
    gm = {"value": probe["generalized_mass"]["value"], "below_normal": probe["generalized_mass"]["below_normal"],
          "undefined_reason": None}
    assert oracle._sum_relation(gm, terms, 1)
