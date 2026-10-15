"""T4-U3 (S14): the test-only pressure-1 verifier admits objective connector
records and rows over the shared connector corpus (the Rust reader's
``tests/connector_contract.rs`` reads the same bytes; the Rust reader is the
only product reader), refuses each broken binding by name, and the retained-source
route (pressure-source-1) refuses connector evidence. (The retired v2 readers'
refusal of connector evidence went with them in B7b.)
"""
from copy import deepcopy
import json
from pathlib import Path

import pytest
from core.analysis_runs import rust_reader
from tests.verifiers import physics_source
from tests.verifiers.source_blocks import domain_hash
from tests.verifiers.pressure_1 import (
    CONTRACT_ID as PRESSURE_ID, validate_exact_pressure_evidence, validate_pressure_evidence, verify_source,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_connector_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def first_connector_row(envelope):
    return next(r for r in envelope["results"] if r["kind"].startswith("connector_"))


def u5a_remote_control():
    """Result-reader control only; no new model version is admitted here."""
    envelope = deepcopy(CASES[0][1])
    envelope["contract_evidence"]["connector"][0].update(pressure_model="constant_effective_area_reference_v1", effective_area_m2=0.004,
        pressure_basis="internal_differential_zero_external_v1", pressure_source_reference="invented U5a reader control")
    generalized = [row for row in envelope["results"] if row["kind"] == "connector_generalized_force_v1" and row["metadata"]["component"] == "gt_x"]
    for generalized_row in generalized:
        for kind, component, value in [("connector_pressure_force_v1", "pe_x", 0.0), ("connector_effective_axial_force_v1", "s_x", generalized_row["value"])]:
            row = deepcopy(generalized_row)
            row.update(id=f'{row["id"]}:{component}', kind=kind, value=value)
            row["metadata"]["component"] = component
            envelope["results"].append(row)
    return envelope


def test_u5a_remote_reader_control_and_zero_pressure_tamper():
    import math
    envelope = u5a_remote_control()
    validate_pressure_evidence(envelope)
    forged = deepcopy(envelope)
    next(row for row in forged["results"] if row["kind"] == "connector_pressure_force_v1")["value"] = math.ulp(0.0)
    with pytest.raises(ValueError, match="joint pressure row binding"):
        validate_pressure_evidence(forged)
    for field in ["effective_area_m2", "pressure_basis", "pressure_source_reference"]:
        missing = deepcopy(envelope)
        del missing["contract_evidence"]["connector"][0][field]
        with pytest.raises(ValueError):
            validate_pressure_evidence(missing)


def test_u5a_projection_guard_stays_finite_and_does_not_widen_direct_product():
    import math
    from tests.verifiers.pressure_1 import _joint_projection
    assert _joint_projection(0.0, 0.0, 1e308, 1e308, 0)
    assert not _joint_projection(1e300, 0.0, 1e308, 1e308, 0)
    tiny = math.ulp(0.0)
    assert _joint_projection(0.0, tiny, tiny, 0.0, 1)
    assert not _joint_projection(2 * tiny, 0.0, tiny, 0.0, 1)


def test_u5a_participating_producer_control_and_tampered_edges_and_groups():
    envelope = json.loads((PROJECT / "fixtures/results/pressure_v3_joint_reader_control.json").read_text())["envelope"]
    from tests.schema_validation import schema_for_definition, validate_instance, validate_schema_document
    schema = json.loads((PROJECT / "schemas/results_pressure_1.v0.3.schema.json").read_text())
    validate_schema_document(schema)
    validate_instance(schema_for_definition(schema, "PressureContractEvidence"), envelope["contract_evidence"])
    validate_pressure_evidence(envelope)
    for field, value in [("connector_id", "component:wrong"), ("face_i_pipe_id", "pipe:wrong"), ("effective_area_m2", 0.005), ("p_pa", 1.0), ("area_difference_force_n", [0.0, 0.0]), ("attachment_pair_global_n", [[0.0] * 3, [0.0] * 3])]:
        forged = deepcopy(envelope)
        forged["contract_evidence"]["pressure"][0]["joint_edges"][0][field] = value
        with pytest.raises(ValueError):
            validate_pressure_evidence(forged)
    for field in ["effective_area_bits", "pressure_bits", "direction_component_magnitude"]:
        forged = deepcopy(envelope)
        group = next(group for group in forged["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"] if "effective_area_bits" in group)
        group[field] = 0.5 if field == "direction_component_magnitude" else "0000000000000000"
        with pytest.raises(ValueError):
            validate_pressure_evidence(forged)
    forged = deepcopy(envelope)
    group = next(group for group in forged["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"] if "effective_area_bits" in group)
    group["terms"].append(deepcopy(group["terms"][0]))
    group["coefficient_sum"] *= 2
    group["assembled_force_n"] *= 2
    with pytest.raises(ValueError, match="duplicate joint RHS source"):
        validate_pressure_evidence(forged)


def record(envelope):
    return envelope["contract_evidence"]["connector"][0]


def _push_duplicate_row(envelope):
    row = deepcopy(first_connector_row(envelope))
    row["id"] = "result:connector:duplicate"
    envelope["results"].append(row)


def _push_replaced_span_row(envelope):
    row = deepcopy(next(r for r in envelope["results"] if r["kind"] == "element_local_axial_force"))
    row["id"], row["entity_ref"] = "result:replaced-span", "pipe:P-130"
    envelope["results"].append(row)


def _remove_connector_row(envelope):
    envelope["results"].remove(first_connector_row(envelope))


# (label, mutation, detail): the same mutations as the Rust reader's test.
MUTATIONS = [
    ("record removed", lambda e: e["contract_evidence"].__setitem__("connector", []), "unbound connector row"),
    ("record duplicated", lambda e: e["contract_evidence"]["connector"].append(deepcopy(record(e))), "duplicate connector record"),
    ("record extra key", lambda e: record(e).__setitem__("extra", 1), "connector record shape"),
    ("record hardware tied", lambda e: record(e).__setitem__("hardware", "tied"), "connector record law"),
    ("record q_ref short", lambda e: record(e).__setitem__("q_ref", [0.0, 0.0]), "connector record frame"),
    ("record rotation scale", lambda e: record(e)["work_matrix"].__setitem__("rotation_scale_rad", 2.0), "connector work matrix"),
    ("replaced span published", lambda e: record(e).__setitem__("replaced_pipe_id", "pipe:P-120"), "connector replaced span published"),
    ("row removed", _remove_connector_row, "connector row coverage"),
    ("row unbound", lambda e: first_connector_row(e).__setitem__("entity_ref", "component:C-999"), "unbound connector row"),
    ("row unknown kind", lambda e: first_connector_row(e).__setitem__("kind", "connector_energy_v1"), "unknown connector kind"),
    ("row unit", lambda e: first_connector_row(e).__setitem__("unit", "mm"), "connector row semantics"),
    ("row basis", lambda e: first_connector_row(e)["metadata"].__setitem__("basis", "objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1"), "connector row semantics"),
    ("row sign", lambda e: first_connector_row(e)["metadata"].__setitem__("sign_convention", "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f"), "connector row semantics"),
    ("row duplicated", _push_duplicate_row, "duplicate connector row"),
    ("replaced span row", _push_replaced_span_row, "connector replaced span published"),
]


def test_corpus_shape():
    assert len(CASES) == 2 and {load_state(e) for _, e in CASES} == {True, False}
    assert all(len(e["contract_evidence"]["connector"]) == 1 for _, e in CASES)


@pytest.mark.parametrize("label,envelope", CASES)
def test_verifier_and_dispatch_admit_the_connector_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))
    verify_source(deepcopy(envelope))
    verify_source(deepcopy(envelope), raw=False)


@pytest.mark.parametrize("label,envelope", CASES)
@pytest.mark.parametrize("what,mutate,detail", MUTATIONS, ids=[m[0] for m in MUTATIONS])
def test_broken_connector_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}"), str(refused.value)


@pytest.mark.parametrize("family,path,code", [
    # The ordinary family's closed physical shape (`connector: const []`) refuses the record before the evidence
    # check; the load/reference-state pre-pass names it first. Both readers agree (the Rust verdict below).
    ("0.3 family", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", "PHYSICS_SOURCE_PHYSICAL_SHAPE"),
    ("load/reference-state family", "fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json", "CONNECTOR_UNSUPPORTED"),
])
def test_the_source_route_refuses_connector_evidence(family, path, code):
    # B7b (Rust `the_source_route_refuses_connector_evidence`): the retained-source route (pressure-source-1, both
    # source families) declines objective connectors (b7-plan J7/D7). A connector record grafted into a genuine
    # pressure-source-1 statement is refused by name, its receipt resealed so that the evidence check is reached.
    source = json.loads((PROJECT / path).read_text())
    physics_source.validate_physics_source(deepcopy(source))
    source["contract_evidence"]["connector"] = [deepcopy(CASES[0][1]["contract_evidence"]["connector"][0])]
    publication = {key: value for key, value in source.items() if key != "source_block_recovery"}
    recovery = source["source_block_recovery"]
    recovery["body"]["publication_sha256"] = domain_hash("source_blocks_publication_v1", publication)
    recovery["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", recovery["body"])
    with pytest.raises(ValueError) as refused:
        physics_source.validate_physics_source(deepcopy(source))
    assert str(refused.value).endswith(code), f"{family}: {refused.value}"
    # The product path takes the Rust reader's verdict, refused by the same name.
    with pytest.raises(rust_reader.ReaderRefusal) as refused:
        rust_reader.validate("pressure-source", deepcopy(source))
    assert str(refused.value).endswith(code), f"{family}: {refused.value}"
