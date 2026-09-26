"""Author the invocation-free source-blocks ledger cases for the desktop port.

Each case is a list of concrete JSON-pointer ops applied to the physics-source-1
projection (Python load_reference_source._project(raw, True)) of a committed
joined raw, then the publication and receipt hashes are resealed (unless the
case says not to) so that the targeted ledger check is reached. The expected
code of every case is the one Python's own port target,
source_blocks._validate_source_blocks(doc, None, context=physics_source),
raises on the same bytes. Run from the project root with the repo venv:
    python make_ledger_cases.py OUT.json
All values are invented mutations of invented fixtures.
"""
from __future__ import annotations
import json, sys
from copy import deepcopy
from pathlib import Path

from core.analysis_runs import load_reference_source as lrs, source_blocks as blocks, physics_source as ps

ROOT = Path.cwd()
SOURCES = {name: f"fixtures/product_preview/load_reference_source/{name}.raw.json" for name in [
    "mixed-sparse_interactive", "n05-sparse_interactive", "eigen_motion-dense_scrutiny"]}
MALFORMED = (KeyError, TypeError, IndexError, AttributeError, OverflowError, StopIteration)


def projected(name):
    raw = json.loads((ROOT / SOURCES[name]).read_text())
    lrs.validate_load_reference_source_evidence(raw)
    return lrs._project(raw, True)


BASE = {name: projected(name) for name in SOURCES}


def tokens(path):
    return [t.replace("~1", "/").replace("~0", "~") for t in path[1:].split("/")]


def resolve(doc, path):
    for t in tokens(path):
        doc = doc[int(t)] if isinstance(doc, list) else doc[t]
    return doc


def apply(doc, ops):
    doc = deepcopy(doc)
    for op in ops:
        t = tokens(op["path"])
        parent = doc
        for x in t[:-1]:
            parent = parent[int(x)] if isinstance(parent, list) else parent[x]
        last = t[-1]
        if op["op"] == "set":
            if isinstance(parent, list): parent[int(last)] = deepcopy(op["value"])
            else: parent[last] = deepcopy(op["value"])
        elif op["op"] == "remove":
            if isinstance(parent, list): parent.pop(int(last))
            else: del parent[last]
        elif op["op"] == "remove_where":
            array = parent[int(last)] if isinstance(parent, list) else parent[last]
            array[:] = [item for item in array if not all(item.get(k) == v for k, v in op["match"].items())]
        elif op["op"] == "append":
            (parent[int(last)] if isinstance(parent, list) else parent[last]).append(deepcopy(op["value"]))
        else:
            raise AssertionError(op)
    return doc


def reseal(doc, publication=True):
    body = doc["source_block_recovery"]["body"]
    if publication:
        body["publication_sha256"] = blocks.domain_hash("source_blocks_publication_v1", {k: v for k, v in doc.items() if k != "source_block_recovery"})
    doc["source_block_recovery"]["receipt_sha256"] = blocks.domain_hash("source_blocks_receipt_v1", body)


def run(doc):
    try:
        blocks._validate_source_blocks(doc, None, context=ps)
    except ValueError as error:
        return str(error)
    except MALFORMED:
        return "PHYSICS_SOURCE_MALFORMED_VALUE"
    return "accept"


M = "mixed-sparse_interactive"
N = "n05-sparse_interactive"
E = "eigen_motion-dense_scrutiny"
C0 = "/source_block_recovery/body/cases/0"
C1 = "/source_block_recovery/body/cases/1"
B = "/source_block_recovery/body"


def idx(name, path, pred):
    return next(i for i, item in enumerate(resolve(BASE[name], path)) if pred(item))


def row_index(name, rid):
    return idx(name, "/results", lambda r: r["id"] == rid)


mixed = BASE[M]
case0 = mixed["source_block_recovery"]["body"]["cases"][0]
outward = idx(M, f"{C0}/projections", lambda p: p["basis"] == "outward_interval")
outward_p = case0["projections"][outward]
exact_zero = idx(M, f"{C0}/projections", lambda p: p["basis"] == "exact_zero")
exact_zero_p = case0["projections"][exact_zero]
identity = idx(M, f"{C0}/projections", lambda p: p["basis"] == "exact_identity")
translation = idx(M, f"{C0}/rows", lambda r: r["recipe_id"] == "translation_norm_scaled_v1")
translation_r = case0["rows"][translation]
proj_row = idx(M, f"{C0}/rows", lambda r: r["treatment"] == "qualified_projection")
proj_row_r = case0["rows"][proj_row]
stress_row = idx(M, f"{C0}/rows", lambda r: r["recipe_id"] == "retained_source_straight_stress_v1")
first_result = mixed["results"][0]["id"]
ordinary_row = idx(M, f"{C1}/rows", lambda r: r["treatment"] == "ordinary_physics_checked")
disp_row = row_index(M, translation_r["result_id"])
anchor = idx(M, f"{C0}/supports", lambda s: s["support_id"] == "anchor")
zero_component = idx(M, f"{C0}/supports/{anchor}/components", lambda x: x["action_terms"][0]["kind"] == "structural_zero")
ideal_component = idx(M, f"{C0}/supports/{anchor}/components", lambda x: x["action_terms"][0]["kind"] == "ideal_constraint")
spring = idx(M, f"{C0}/supports", lambda s: s["support_id"] == "independent-spring")
spring_component = idx(M, f"{C0}/supports/{spring}/components", lambda x: x["action_terms"][0]["kind"] == "ground_spring")
diag_ids = [d["id"] for d in mixed["diagnostics"]]
rows0 = case0["rows"]
free_proj_row = idx(M, f"{C0}/rows", lambda r: r["result_id"] == "result:disp:independent-root:rx")
input_row = idx(M, f"{C0}/rows", lambda r: r["result_id"] == translation_r["input_result_ids"][0])
force_norm = idx(M, f"{C0}/rows", lambda r: r["recipe_id"] == "support_force_norm_scaled_checked_v1")
force_norm_r = rows0[force_norm]
sup_input_row = idx(M, f"{C0}/rows", lambda r: r["result_id"] == force_norm_r["input_result_ids"][0])
tip_translation = idx(M, f"{C0}/rows", lambda r: r["recipe_id"] == "translation_norm_scaled_v1" and r["result_id"] != translation_r["result_id"])
end_i_force = row_index(M, "result:force:independent-member:axial")
spring_row = row_index(M, case0["supports"][spring]["components"][spring_component]["result_id"])
stress_r = rows0[stress_row]
stress_result = next(r for r in mixed["results"] if r["id"] == stress_r["result_id"])
stress_check = next(c for c in case0["section_stress_checks"] if c["result_id"] == stress_r["result_id"])
max_row = idx(M, f"{C0}/rows", lambda r: r["recipe_id"] == "retained_source_endpoint_normal_max_v1")
max_result = next(r for r in mixed["results"] if r["id"] == rows0[max_row]["result_id"])
max_extremum = next(e for e in mixed["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"] if e["result_id"] == max_result["id"])
force_result = next(r for r in mixed["results"] if r["id"] == force_norm_r["result_id"])
force_check = next(c for c in case0["derived_checks"] if c["result_id"] == force_norm_r["result_id"])
support_projection = next(p for p in case0["projections"] if p["quantity"] == "support_action_component")
support_result = next(r for r in mixed["results"] if r["id"] == support_projection["result_id"])
ordinary_only_results = [r for r in mixed["results"] if (r.get("basis_ref") or {}).get("ref_id") != "case"]
q0 = "/numerical_quality/cases/0"
second_ordinary = [i for i, r in enumerate(mixed["source_block_recovery"]["body"]["cases"][1]["rows"]) if r["treatment"] == "ordinary_physics_checked"][1]
second_ordinary_r = mixed["source_block_recovery"]["body"]["cases"][1]["rows"][second_ordinary]
eigen_case0 = BASE[E]["source_block_recovery"]["body"]["cases"][0]
eigen_stress = [r["result_id"] for r in eigen_case0["rows"] if r["recipe_id"] == "retained_source_straight_stress_v1"]
eigen_norm_input = next(j for r in eigen_case0["rows"] if r["recipe_id"] == "translation_norm_scaled_v1" for rid in r["input_result_ids"]
                        for j, p in enumerate(eigen_case0["projections"])
                        if p["projection_id"] == next(x for x in eigen_case0["rows"] if x["result_id"] == rid)["projection_id"] and p["basis"] == "outward_interval")

CASES = [
    # id, source, ops, reseal publication (True/False/None=no reseal at all), intended check
    ("RAW_FIELDS", M, [{"op": "set", "path": "/invented_extra", "value": 1}], True),
    ("PRODUCER", M, [{"op": "set", "path": "/producer/component_version", "value": "0.3.0"}], True),
    ("FORMULATION", M, [{"op": "set", "path": "/formulation_basis/limitations", "value": []}], True),
    ("RECEIPT_SHAPE", M, [{"op": "set", "path": f"{B}/invented", "value": 1}], True),
    ("RECEIPT_HASH", M, [{"op": "set", "path": "/source_block_recovery/receipt_sha256", "value": "0" * 64}], None),
    ("PUBLICATION_HASH", M, [{"op": "set", "path": f"{B}/publication_sha256", "value": "0" * 64}], False),
    ("ROWS", M, [{"op": "set", "path": "/diagnostics", "value": {}}], True),
    ("EVIDENCE_IDS", M, [{"op": "set", "path": "/diagnostics/0/id", "value": first_result}], True),
    ("VALUE_NONFINITE", M, [{"op": "set", "path": "/results/0/value", "value": "0"}], True),
    ("ROW_FIELDS", M, [{"op": "set", "path": "/results/0/unit", "value": ""}], True),
    ("ROW_METADATA", M, [{"op": "set", "path": f"/results/{row_index(M, proj_row_r['result_id'])}/metadata/invented", "value": "x"}], True),
    ("CASE_IDS", M, [{"op": "set", "path": f"{C1}/basis_ref", "value": case0["basis_ref"]}], True),
    ("ORDINARY_QUALITY", M, [{"op": "set", "path": "/numerical_quality/integrity_policy", "value": "invented"}], True),
    ("ORDINARY_CASE", M, [{"op": "set", "path": f"{q0}/accuracy_evidence", "value": "invented"}], True),
    ("ORDINARY_AGGREGATE", M, [{"op": "set", "path": "/numerical_quality/status", "value": "failed"}], True),
    ("QUALITY_CASE_BINDING", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/quality_case_index", "value": 1}], True),
    ("ORDINARY_MODE", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/requested_mode", "value": "dense_scrutiny"}], True),
    ("ORDINARY_REPORT_REF", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/structural_report_diagnostic_ref", "value": "diagnostic:invented"}], True),
    ("ORDINARY_FAILURE_REF", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/failure", "value": {"stage": "assembly", "diagnostic_ref": "diagnostic:invented"}}], True),
    ("ORDINARY_NOT_ATTEMPTED", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/outcome", "value": "not_attempted"}], True),
    ("ORDINARY_REPORT", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/outcome", "value": "checks_passed"}], True),
    ("ORDINARY_REPORT_KIND", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/structural_report_diagnostic_ref", "value": "diagnostic:numerical-integrity:case:ordinary-pressure"},
                                 {"op": "set", "path": f"{q0}/evidence_refs", "value": ["diagnostic:numerical-integrity:case:ordinary-pressure"]}], True),
    ("ORDINARY_REJECTION", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/outcome", "value": "rejected"}], True),
    ("INVOCATION_WORK_LIMIT", M, [{"op": "set", "path": f"{C0}/work/charged", "value": 64_000_001}], True),
    ("WORK_LEDGER", M, [{"op": "set", "path": f"{C0}/work/charged", "value": 8_000_001}], True),
    ("QUALIFIED_OUTCOME", M, [{"op": "set", "path": f"{C0}/work/reserved_unobserved_failure", "value": 1}], True),
    ("ORDINARY_SELECTION", M, [{"op": "set", "path": f"{C1}/selected_method", "value": "ordinary_dense_structural_v1"}], True),
    ("EXACT_SOURCE_REQUIRED", M, [{"op": "set", "path": f"{C0}/source", "value": None}], True),
    ("FAILED_OUTCOME", M, [{"op": "set", "path": f"{C1}/outcome", "value": "failed"}], True),
    ("FAILURE_BLOCK_ORDER", M, [{"op": "set", "path": f"{C1}/outcome", "value": "failed"}, {"op": "set", "path": f"{C1}/selected_method", "value": None},
                                {"op": "set", "path": f"{C1}/failure", "value": {"code": "unsupported_block", "block_order": 1, "diagnostic_ref": diag_ids[0]}}], True),
    ("FAILURE_CATEGORY", M, [{"op": "set", "path": f"{C1}/outcome", "value": "unsupported"}, {"op": "set", "path": f"{C1}/selected_method", "value": None},
                             {"op": "set", "path": f"{C1}/failure", "value": {"code": "numerical_failure", "block_order": None, "diagnostic_ref": diag_ids[0]}}], True),
    ("SOURCE_COUNTS", M, [{"op": "set", "path": f"{C0}/source/stiffness_term_count", "value": 0}], True),
    ("FUNCTIONAL_COUNT", M, [{"op": "set", "path": f"{C0}/source/functional_count", "value": 1}], True),
    ("SOURCE_PARTITION", M, [{"op": "set", "path": f"{C0}/source/prescribed_dofs", "value": [0, 1, 2, 4]}], True),
    ("BLOCK_PARTITION", M, [{"op": "set", "path": f"{C0}/source/free_blocks", "value": [[3, 9], [6], [7, 11]]}], True),
    ("MEMBER_IDENTITY", M, [{"op": "set", "path": f"{C0}/source/member_ids", "value": ["independent-member", "independent-member"]}], True),
    ("SUPPORT_IDENTITY", M, [{"op": "set", "path": f"{C0}/source/support_ids", "value": ["anchor", "anchor"]}], True),
    ("MEMBER_COVERAGE", M, [{"op": "set", "path": f"{C0}/source/member_ids", "value": []}], True),
    ("COMMITMENT_DOMAIN_SEPARATION", M, [{"op": "set", "path": f"{C0}/source/normalized_source_sha256", "value": case0["source"]["functional_plan_sha256"]}], True),
    ("PROJECTION_IDS", M, [{"op": "set", "path": f"{C0}/projections/1/projection_id", "value": case0["projections"][0]["projection_id"]}], True),
    ("FUNCTIONAL_IDS", M, [{"op": "set", "path": f"{C0}/projections/1/functional_id", "value": case0["projections"][0]["functional_id"]}], True),
    ("PROJECTION_RESULTS", M, [{"op": "set", "path": f"{C0}/projections/1/result_id", "value": case0["projections"][0]["result_id"]}], True),
    ("ROW_IDS", M, [{"op": "append", "path": f"{C0}/rows", "value": case0["rows"][0]}], True),
    ("STRESS_TOTAL_RELATIVE_BOUND", M, [{"op": "set", "path": f"{C0}/projections/{outward}/relative_error_bound", "value": 1e-9}], True),
    ("CASE_ROW_COVERAGE", M, [{"op": "remove", "path": f"{C0}/rows/{proj_row}"}], True),
    ("ROW_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{translation}/input_result_ids", "value": [translation_r["input_result_ids"][0]] * 3}], True),
    ("SAME_CASE_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{translation}/input_result_ids", "value": translation_r["input_result_ids"][:2] + [translation_r["result_id"]]}], True),
    ("PROJECTION_ROW", M, [{"op": "set", "path": f"{C0}/rows/{free_proj_row}/projection_id", "value": "projection:invented"}], True),
    ("PROJECTION_VALUE_BINDING", M, [{"op": "set", "path": f"{C0}/projections/{exact_zero}/value_bits", "value": "8000000000000000"}], True),
    ("PROJECTION_INTERVAL", M, [{"op": "set", "path": f"{C0}/projections/{exact_zero}/interval", "value": [1.0, 2.0]}], True),
    ("PROJECTION_EXACT_BASIS", M, [{"op": "set", "path": f"{C0}/projections/{exact_zero}/absolute_error_bound", "value": 1e-30}], True),
    ("PROJECTION_ENCLOSURE", M, [{"op": "set", "path": f"{C0}/projections/{outward}/absolute_error_bound", "value": 0.0}], True),
    ("PROJECTION_RELATIVE_BOUND", M, [{"op": "set", "path": f"{C0}/projections/{outward}/relative_error_bound", "value": 0.0}], True),
    ("PROJECTION_CRITERION", M, [{"op": "set", "path": f"{C0}/projections/{outward}/relative_limit", "value": 1e-8}], True),
    ("PROJECTION_TYPED_QUANTITY", M, [{"op": "set", "path": f"{C0}/projections/{exact_zero}/quantity", "value": "constraint_reaction"}], True),
    ("DERIVED_ROW", M, [{"op": "set", "path": f"{C0}/rows/{translation}/recipe_id", "value": None}], True),
    ("DERIVED_RECIPE", M, [{"op": "set", "path": f"{C0}/rows/{translation}/recipe_id", "value": "invented_recipe_v1"}], True),
    ("DERIVED_SIGNATURE", M, [{"op": "set", "path": f"/results/{disp_row}/unit", "value": "m"}], True),
    ("TRANSLATION_NORM_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{translation}/input_result_ids", "value": list(reversed(translation_r["input_result_ids"]))}], True),
    ("DERIVED_NORM_VALUE", M, [{"op": "set", "path": f"/results/{disp_row}/value", "value": 1.0}], True),
    ("DERIVED_ENTITY", M, [{"op": "set", "path": f"{C0}/rows/{translation}/input_result_ids", "value": rows0[tip_translation]["input_result_ids"]}], True),
    ("DERIVED_NORM_RANGE", M, [{"op": "set", "path": f"/results/{row_index(M, i)}/value", "value": 1e-310} for i in translation_r["input_result_ids"]], True),
    ("NORM_PROJECTION_INPUT", M, [{"op": "set", "path": f"{C0}/rows/{input_row}/treatment", "value": "checked_derived"},
                                  {"op": "set", "path": f"{C0}/rows/{input_row}/projection_id", "value": None},
                                  {"op": "set", "path": f"{C0}/rows/{input_row}/recipe_id", "value": "translation_norm_scaled_v1"}], True),
    ("UNQUALIFIED_DERIVED_INPUT", M, [{"op": "set", "path": f"{C0}/rows/{sup_input_row}/treatment", "value": "inspection_only"},
                                      {"op": "set", "path": f"{C0}/rows/{sup_input_row}/projection_id", "value": None}], True),
    ("ORDINARY_ROW", M, [{"op": "set", "path": f"{C1}/rows/{ordinary_row}/recipe_id", "value": "translation_norm_scaled_v1"}], True),
    ("INSPECTION_ROW", M, [{"op": "set", "path": f"{C1}/rows/{ordinary_row}/treatment", "value": "inspection_only"},
                           {"op": "set", "path": f"{C1}/rows/{ordinary_row}/recipe_id", "value": "translation_norm_scaled_v1"}], True),
    ("PROJECTION_BIJECTION", M, [{"op": "append", "path": f"{C0}/projections", "value": dict(exact_zero_p, projection_id="projection:invented", functional_id="source-functional:invented", result_id="result:invented")}], True),
    ("SUPPORT_COVERAGE", M, [{"op": "set", "path": f"{C0}/source/support_ids", "value": ["independent-spring", "anchor"]}], True),
    ("SIX_SUPPORT_COMPONENTS", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/0/component", "value": "Fy"}], True),
    ("SUPPORT_PROJECTION", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/0/functional_id", "value": "source-functional:invented"}], True),
    ("SUPPORT_ROW_SEMANTICS", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/support_id", "value": "independent-spring"},
                                  {"op": "set", "path": f"{C0}/supports/{spring}/support_id", "value": "anchor"},
                                  {"op": "set", "path": f"{C0}/source/support_ids", "value": ["independent-spring", "anchor"]}], True),
    ("SUPPORT_ACTION_TERMS", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/{ideal_component}/action_terms", "value": []}], True),
    ("SUPPORT_TERM_DUPLICATE", M, [{"op": "append", "path": f"{C0}/supports/{anchor}/components/{ideal_component}/action_terms", "value": case0["supports"][anchor]["components"][ideal_component]["action_terms"][0]}], True),
    ("SUPPORT_ACTION_OWNER", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/{ideal_component}/action_terms/0/source_id", "value": "independent-spring"}], True),
    ("SUPPORT_ACTION_DOF", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/{zero_component}/action_terms/0/global_dof", "value": case0["supports"][anchor]["components"][zero_component]["action_terms"][0]["global_dof"] + 6}], True),
    ("IDEAL_ATTRIBUTION_AMBIGUOUS", M, [{"op": "set", "path": f"{C0}/supports/{spring}/components/{ideal_component}/action_terms/0/kind", "value": "ideal_constraint"}], True),
    ("STRUCTURAL_ZERO", M, [{"op": "set", "path": f"{C0}/supports/{spring}/components/{spring_component}/action_terms/0/kind", "value": "structural_zero"}], True),
    ("IDEAL_SOURCE_COVERAGE", M, [{"op": "set", "path": f"{C0}/supports/{anchor}/components/{zero_component}/action_terms/0/kind", "value": "ground_spring"},
                                  {"op": "set", "path": f"{C0}/source/prescribed_dofs", "value": [0, 1, 2, 3, 4, 5]},
                                  {"op": "set", "path": f"{C0}/source/free_dofs", "value": [6, 7, 8, 9, 10, 11]},
                                  {"op": "set", "path": f"{C0}/source/free_blocks", "value": [[9], [6], [7, 11], [8, 10]]}], True),
    ("SUPPORT_RESULT_BIJECTION", M, [{"op": "append", "path": "/results", "value": dict(support_result, id="result:invented-ghost", entity_ref="ghost")},
                                     {"op": "append", "path": f"{C0}/projections", "value": dict(support_projection, projection_id="projection:invented-ghost", functional_id="source-functional:invented-ghost", result_id="result:invented-ghost")},
                                     {"op": "append", "path": f"{C0}/rows", "value": {"result_id": "result:invented-ghost", "treatment": "qualified_projection", "projection_id": "projection:invented-ghost", "recipe_id": None, "input_result_ids": []}}], True),
    ("MEMBER_PRIMARY_COVERAGE", M, [{"op": "set", "path": f"/results/{end_i_force}/metadata/location", "value": "end_j"}], True),
    ("MEMBER_STRESS_COVERAGE", M, [{"op": "append", "path": "/results", "value": dict(stress_result, id="result:invented-stress")},
                                   {"op": "append", "path": f"{C0}/rows", "value": dict(stress_r, result_id="result:invented-stress")},
                                   {"op": "append", "path": f"{C0}/section_stress_checks", "value": dict(stress_check, result_id="result:invented-stress")}], True),
    ("MEMBER_SUMMARY_COVERAGE", M, [{"op": "append", "path": "/results", "value": dict(max_result, id="result:invented-maximum")},
                                    {"op": "append", "path": f"{C0}/rows", "value": dict(rows0[max_row], result_id="result:invented-maximum")},
                                    {"op": "append", "path": "/contract_evidence/exact_cases/0/pipe_stress_extrema", "value": dict(max_extremum, result_id="result:invented-maximum")}], True),
    ("SUPPORT_MAGNITUDE_COVERAGE", M, [{"op": "append", "path": "/results", "value": dict(force_result, id="result:invented-force-norm")},
                                       {"op": "append", "path": f"{C0}/rows", "value": dict(force_norm_r, result_id="result:invented-force-norm")},
                                       {"op": "append", "path": f"{C0}/derived_checks", "value": dict(force_check, result_id="result:invented-force-norm")}], True),
    ("SOURCE_FALLBACK_TRIGGER", M, [{"op": "set", "path": f"{C0}/ordinary_attempt/outcome", "value": "checks_passed"},
                                    {"op": "set", "path": f"{C0}/ordinary_attempt/structural_report_diagnostic_ref", "value": "diagnostic:numerical-integrity:case:ordinary-pressure"},
                                    {"op": "set", "path": f"{q0}/evidence_refs", "value": ["diagnostic:numerical-integrity:case:ordinary-pressure"]},
                                    {"op": "set", "path": f"{q0}/solve_quality", "value": "checks_passed"},
                                    {"op": "set", "path": "/numerical_quality/status", "value": "checks_passed"}], True),
    ("DERIVED_CHECK_COVERAGE", M, [{"op": "append", "path": f"{C0}/derived_checks", "value": dict(force_check, result_id="result:invented")}], True),
    ("STRESS_CHECK_COVERAGE", M, [{"op": "append", "path": f"{C0}/section_stress_checks", "value": dict(stress_check, result_id="result:invented")}], True),
    ("INVOCATION_WORK_LEDGER", M, [{"op": "set", "path": f"{B}/invocation_work/publication_charged", "value": 1}], True),
    ("OBSERVATION_IDS", M, [{"op": "set", "path": f"{B}/envelope_observation_result_ids", "value": [first_result, first_result]}], True),
    ("ENVELOPE_ROW_COVERAGE", M, [{"op": "set", "path": f"{B}/envelope_observation_result_ids", "value": [first_result]}], True),
    ("SOURCE_METHOD_RECORD_REQUIRED", M, [{"op": "remove", "path": f"{C0}"}, {"op": "remove", "path": q0},
                                          {"op": "set", "path": "/source_block_recovery/body/cases/0/ordinary_attempt/quality_case_index", "value": 0},
                                          {"op": "remove_where", "path": "/results", "match": {"basis_ref": case0["basis_ref"]}},
                                          {"op": "set", "path": "/numerical_quality/status", "value": "checks_passed"},
                                          {"op": "set", "path": f"{B}/invocation_work/charged", "value": mixed["source_block_recovery"]["body"]["invocation_work"]["publication_charged"]}], True),
    ("SUMMARY_HEADLINE", M, [{"op": "set", "path": "/summary/max_displacement", "value": None}], True),
    ("SUMMARY_RESULT_BINDING", M, [{"op": "set", "path": "/summary/max_displacement/value", "value": 1.0}], True),
    # Added after the first mutation run (T1 WP2): each reaches a check no earlier case reached.
    ("EVIDENCE_IDS#empty-id", M, [{"op": "set", "path": "/diagnostics/0/id", "value": ""}], True),
    ("SOURCE_PRESSURE_RHS", M, [{"op": "set", "path": "/contract_evidence/exact_cases/0/pressure_rhs_assembly/assembled_pressure_rhs_global/0", "value": 1.0}], True),
    ("PHYSICAL_OBSERVATION_ESCAPE", M, [{"op": "append", "path": "/results", "value": dict(mixed["results"][0], id="result:invented-observation", basis_ref=None)},
                                        {"op": "append", "path": f"{B}/envelope_observation_result_ids", "value": "result:invented-observation"}], True),
    # Added after the second mutation run: checks first recorded as unreachable
    # that a concrete input does reach.
    ("SUMMARY_UNSOURCED_HEADLINE", M, [{"op": "remove_where", "path": "/results", "match": {"kind": "displacement_magnitude"}}]
        + [{"op": "remove_where", "path": f"/source_block_recovery/body/cases/{i}/rows", "match": {"result_id": r["id"]}}
           for i in range(len(mixed["source_block_recovery"]["body"]["cases"])) for r in mixed["results"] if r["kind"] == "displacement_magnitude"], True),
    ("ROW_DEPENDENCY_CYCLE", M, [{"op": "set", "path": f"{C1}/rows/{ordinary_row}/input_result_ids", "value": [second_ordinary_r["result_id"]]},
                                 {"op": "set", "path": f"{C1}/rows/{second_ordinary}/input_result_ids", "value": [mixed["source_block_recovery"]["body"]["cases"][1]["rows"][ordinary_row]["result_id"]]}], True),
    ("NORM_TOTAL_RELATIVE_BOUND", E, [{"op": "set", "path": f"{C0}/projections/{eigen_norm_input}/relative_error_bound", "value": 1e-9}]
        + [op for rid in eigen_stress for op in ({"op": "remove_where", "path": f"{C0}/rows", "match": {"result_id": rid}}, {"op": "remove_where", "path": "/results", "match": {"id": rid}})], True),
    ("AGGREGATE_STATUS", M, [{"op": "set", "path": f"{B}/cases", "value": []}, {"op": "set", "path": "/numerical_quality/cases", "value": []},
                             {"op": "set", "path": "/numerical_quality/status", "value": "not_assessed"}, {"op": "set", "path": "/results", "value": []},
                             {"op": "set", "path": f"{B}/invocation_work/charged", "value": mixed["source_block_recovery"]["body"]["invocation_work"]["publication_charged"]}], True),
    # Pre-empted demonstrations (the port keeps the check; no input reaches it):
    # the closed receipt schema, or an earlier check of the same case.
    ("SPRING_ATTRIBUTION_DUPLICATE", M, [{"op": "append", "path": f"{C0}/supports/{spring}/components", "value": case0["supports"][spring]["components"][spring_component]}], True),
    ("SUPPORT_NORM_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{force_norm}/recipe_id", "value": "support_force_norm_scaled_v1"}], True),
    ("STRESS_RECIPE_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{stress_row}/recipe_id", "value": "straight_open_stress_v1"}], True),
    ("SUMMARY_RECIPE_INPUTS", M, [{"op": "set", "path": f"{C0}/rows/{max_row}/recipe_id", "value": "reviewed_stress_summary_v1"}], True),
    ("SECTION_RECIPE_KIND", M, [{"op": "set", "path": f"{C0}/rows/{translation}/recipe_id", "value": "section_property_from_source_v1"}], True),
    ("UNSELECTED_SUPPORT_CERTIFICATE", M, [{"op": "set", "path": f"{C1}/supports", "value": case0["supports"]}], True),
    ("ACCEPT-mixed", M, [], True),
    ("ACCEPT-n05", N, [], True),
    ("ACCEPT-eigen", E, [], True),
]


def main(out):
    records = []
    for cid, name, ops, publication in CASES:
        doc = apply(BASE[name], ops)
        if publication is not None:
            reseal(doc, publication)
        records.append({"id": cid, "source": name, "ops": ops, "reseal_publication": publication, "python": run(doc)})
    Path(out).write_text(json.dumps(records, indent=1) + "\n")
    for r in records:
        mark = "" if r["python"].endswith(r["id"].split("#")[0]) or (r["id"].startswith("ACCEPT") and r["python"] == "accept") else "   <-- differs"
        print(f'{r["id"]:32s} {r["python"]}{mark}')


if __name__ == "__main__":
    main(sys.argv[1])
