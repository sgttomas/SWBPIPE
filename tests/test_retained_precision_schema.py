"""Schema controls are synthetic shape checks, never numerical qualification."""
from copy import deepcopy
import json
from pathlib import Path
import sys

import jsonschema
import pytest

from checked_json_fixture_cache import checked_json_fixture_cache

if str(Path(__file__).resolve().parent) not in sys.path:
    sys.path.insert(0, str(Path(__file__).resolve().parent))
from retained_precision_corpus import load_corpus  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = json.loads((ROOT / "schemas/retained_precision_mp_v2.schema.json").read_text())
CORPUS = load_corpus()


def validate(value, name=None):
    schema = SCHEMA if name is None else {"$defs":SCHEMA["$defs"], "$ref":"#/$defs/"+name}
    jsonschema.Draft202012Validator(schema).validate(value)


def test_schema_itself_and_full_synthetic_receipt():
    jsonschema.Draft202012Validator.check_schema(SCHEMA)
    for fixture in CORPUS["cases"]:
        validate(fixture["source"]["retained_precision"])
        for row in fixture["source"]["results"]:
            validate(row, "RawRow")


@pytest.mark.parametrize("token", [None, 1, {}, [], True])
def test_direct_method_has_string_shape(token):
    row = deepcopy(CORPUS["cases"][0]["source"]["results"][0])
    row["recovery_method"] = token
    with pytest.raises(jsonschema.ValidationError): validate(row, "RawRow")


@pytest.mark.parametrize("token", ["", "wrong", "contribution_preserving_multiprecision_v1"])
def test_string_value_scope_is_reserved_for_g6(token):
    row = deepcopy(CORPUS["cases"][0]["source"]["results"][0])
    row["recovery_method"] = token
    validate(row, "RawRow")


def test_g5a_discriminator_does_not_collide_with_quantity_payload():
    for kind in ["sanity", "lower"]:
        variant = next(v for v in SCHEMA["$defs"]["G5aError"]["oneOf"] if v["properties"]["kind"].get("const") == kind)
        assert "quantity_kind" in variant["required"]
        assert variant["properties"]["quantity_kind"] == {"enum":[0,1]}
        assert variant["additionalProperties"] is False


def test_all_object_definitions_are_closed():
    def walk(value):
        if isinstance(value, dict):
            if value.get("type") == "object":
                assert value.get("additionalProperties") is False
                assert set(value["required"]) <= value["properties"].keys()
            for child in value.values(): walk(child)
        elif isinstance(value, list):
            for child in value: walk(child)
    walk(SCHEMA)

def test_results_successor_branch_is_explicit_and_old_branches_reject_receipt():
    from tests.test_load_reference_schema import _results_document
    from tests.schema_validation import validate_instance
    schema = json.loads((ROOT / "schemas/results.v0.3.schema.yaml").read_text())
    jsonschema.Draft202012Validator.check_schema(schema)
    source = CORPUS["cases"][0]["source"]
    document = _results_document(source, source["producer"]["semantic_contract_id"])
    document["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
    validate_instance(schema, document, instance_label="synthetic retained derivative shape")
    del document["result_envelope"]["retained_precision"]
    with pytest.raises(AssertionError):
        validate_instance(schema, document, instance_label="missing required receipt")
    old = json.loads((ROOT / "fixtures/results/preview_physics_connected_sparse.json").read_text())
    document = _results_document(old, old["producer"]["semantic_contract_id"])
    validate_instance(schema, document, instance_label="historical preview shape")
    document["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
    with pytest.raises(AssertionError):
        validate_instance(schema, document, instance_label="receipt on historical branch")



# ---- U6c (I66): the successor carrier schemas (C1:162; D2 4.9.6; RV78-S1, RV78-N2;
# D-U6-2). Shape checks only: schema validity creates no standing, and the
# stress-neutral branch does not lift the T6 export refusal.

SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-2"
# B7b (a43583c625, "Rebase retained exact evidence onto current pressure tables") re-issued the tables: pressure-1
# left preview-physics-2's reserved successors (36064ef3... and f675693b... before), the preview successor inherits
# the new preview hash, the exact successor (D4 (a)) inherits pressure-2's, and pressure-source-1 is the current
# PRESSURE-SOURCE-1 table (4a1873e7... before).
SUCC_SHA = "c6e3ce0fe8ed60a66d40ef7f22505598d6e8bd1f3656752ca0b50e59a8f55991"
PREVIEW_SHA = "5e3285343565883e961239fc189fb3cb1fd49d45250191340ecc5e8f3df9d74a"
PROFILE = "product_preview_retained_w1a_v2"
CODES = ["retained_precision_absolute_verified", "retained_precision_not_covered"]
# B3b (B3-D §7): the exact successor `<physics-retained>`, whose carrier branches follow
# the preview successor's. Both successors, and only they, carry a receipt.
EXACT = "openpipestress.result_semantics/0.3.0/physics-retained-1"
# T4-U6 (D-5, §2.4): follows pressure-2's per-basis table by its inherited hash (was 69581249...).
EXACT_SHA = "d6d86dfc418978257d1d3370684e879ba9711465ceba1de72729eb48bb12ad3c"
EXACT_PROFILE = "exact_straight_retained_w1a_v2"
SUCCESSORS = (SUCC, EXACT)
# b7-plan's pressure-source join: a tenth run branch (no receipt); its contract
# registry entry is appended after the exact successor's.
PRESSURE_SOURCE = "openpipestress.result_semantics/0.3.0/pressure-source-1"
PRESSURE_SOURCE_SHA = "b5a4b0e585b9ec440d36144860baaf101b628498cfdab9a1de9248c2ed5069e7"
# B7b: the composite source statement is pressure-source-1's (physics-source-1 is retired).
SOURCE_RAW = "pressure_source/physics_source/n05-sparse_interactive.raw.json"
MILESTONES = {
    "sparse_interactive": "cf053309e8048ae013dc5388d1115399ff89cf493eb2e36772500a3c6985a6e5",
    "dense_scrutiny": "f8078ee2c82ba370ab04591af49ed12c48fda5d88a54a75b0725fbdde317d884",
}


def carrier_schema(name):
    return json.loads((ROOT / "schemas" / name).read_text())


def check(schema_name, document, valid, label):
    from tests.schema_validation import validate_instance
    schema = carrier_schema(schema_name)
    if valid:
        validate_instance(schema, document, instance_label=label)
    else:
        with pytest.raises(AssertionError):
            validate_instance(schema, document, instance_label=label)


def milestone(mode):
    """D-U6-5: byte-identical copies of PP's pinned successor files."""
    import hashlib
    raw = (ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == MILESTONES[mode]
    return json.loads(raw)["source"]


def successors():
    """Every current shared-corpus successor and both preview milestones."""
    sources = [(case["id"], case["source"]) for case in CORPUS["cases"]]
    return sources + [(f"milestone_{mode}", milestone(mode)) for mode in sorted(MILESTONES)]


def results_document(source):
    from tests.test_load_reference_schema import _results_document
    document = _results_document(source, source["producer"]["semantic_contract_id"])
    if "retained_precision" in source:
        document["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
    return document


def projected(source):
    """The reader's G7 projection: the same statement under the base identity."""
    base = deepcopy(source)
    del base["retained_precision"]
    exact = source["producer"]["semantic_contract_id"] == EXACT
    base["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/pressure-2" if exact else PREVIEW
    base["formulation_basis"]["profile_id"] = "exact_pressure_v3" if exact else "product_preview_mechanics_v1"
    for row in base["results"]:
        row.pop("recovery_method", None)
    return base


def disclosure(code, index=0):
    checksum = {"algorithm": "sha256", "canonicalization": "openpipestress_jcs_ijson_v1",
                "payload_ref": {"ref_type": "test_carrier", "ref_id": "schema"}, "payload_scope": "raw_source_row", "value": "0" * 64}
    return {"source_row_index": index, "source_result_id": f"result:schema:{index}", "source_kind": "displacement_magnitude",
            "source_value": 0.0, "source_unit": "mm", "source_dimension_present": False, "source_dimension": None,
            "declared_semantic_dimension": "length", "source_physical_semantic_dimension": "length",
            "semantic_category": "physical_quantity", "reason_code": code,
            "object_ref": {"ref_type": "preview_entity", "ref_id": "N0"}, "source_field_path": f"/results/{index}",
            "source_annotation_ref": {"ref_type": "source_annotation", "ref_id": f"source-annotation:{index}"},
            "received_carrier_row_checksum": checksum, "original_producer_row_checksum": deepcopy(checksum),
            "message": f"displacement_magnitude: {code}; schema control"}


@pytest.mark.parametrize("label,source", successors(), ids=lambda x: x if isinstance(x, str) else "")
def test_rv78_n2_y0_every_successor_statement_has_a_valid_derivative_shape(label, source):
    check("results.v0.3.schema.yaml", results_document(source), True, label)


def foreign_source_block_recovery():
    """A shape-valid foreign member: the real pressure-source n05 receipt (RV88 U6c N-1)."""
    raw = json.loads((ROOT / "fixtures/product_preview" / SOURCE_RAW).read_text())
    return deepcopy(raw["source_block_recovery"])


def real_value_row(document):
    """A real derivative value row (the pressure-2 load/reference-state document's first displacement, B7b's twin
    of load-reference-1's), on the document's own basis, so Y11's token is the only change (RV88 U6c N-1)."""
    source = json.loads((ROOT / "fixtures/results/pressure_load_state_connected_sparse.document.json").read_text())
    row = deepcopy(source["result_envelope"]["result_sets"][0]["values"][0])
    row["basis_ref"] = deepcopy(document["result_envelope"]["result_sets"][0]["basis_ref"])
    return row


@pytest.mark.parametrize("label,source", successors(), ids=lambda x: x if isinstance(x, str) else "")
def test_rv78_n2_receipt_and_branch_probes(label, source):
    """RV78's Y1-Y11 probes (YAML_PROBES.json), each a discriminating pair over every
    successor statement: the unmodified document validates, the single change is
    refused. Y4's member and Y11's row are shape-valid, so only the branch refuses."""
    base = results_document(source)
    check("results.v0.3.schema.yaml", base, True, f"Y0 {label} control")
    with_row = deepcopy(base)
    with_row["result_envelope"]["result_sets"][0]["values"] = [real_value_row(base)]
    check("results.v0.3.schema.yaml", with_row, True, f"Y11 {label} control: a real value row")
    edits = {
        "Y1_receipt_policy_v1": lambda e: e["retained_precision"]["body"].__setitem__("policy", "M03-INTEGRITY-MP-v1"),
        "Y2_receipt_unknown_member": lambda e: e["retained_precision"]["body"].__setitem__("unknown", 1),
        "Y3_receipt_empty_body": lambda e: e.__setitem__("retained_precision", {"body": {}, "receipt_sha256": "0" * 64}),
        "Y4_source_block_recovery": lambda e: e.__setitem__("source_block_recovery", foreign_source_block_recovery()),
        "Y5_preview_profile": lambda e: e["formulation_basis"].__setitem__("profile_id", "product_preview_mechanics_v1"),
        "Y6_limitations_changed": lambda e: e["formulation_basis"].__setitem__("limitations", e["formulation_basis"]["limitations"][:-1]),
        "Y7_contract_ref_mismatch": lambda e: e["semantic_contract_ref"].__setitem__("ref_id", PREVIEW),
        "Y8_no_contract_evidence": lambda e: e.pop("contract_evidence"),
        "missing_receipt": lambda e: e.pop("retained_precision"),
        "null_receipt": lambda e: e.__setitem__("retained_precision", None),
    }
    for name, edit in edits.items():
        document = deepcopy(base)
        edit(document["result_envelope"])
        check("results.v0.3.schema.yaml", document, False, f"{name} {label}")
    token = deepcopy(with_row)
    token["result_envelope"]["result_sets"][0]["values"][0]["recovery_method"] = "contribution_preserving_multiprecision_v1"
    check("results.v0.3.schema.yaml", token, False, f"Y11_derivative_value_row_token {label}")
    # Y9 and Y10: the base identity's branch admits the projection, and refuses it
    # only once a receipt is added.
    base_document = results_document(projected(source))
    check("results.v0.3.schema.yaml", base_document, True, f"Y10 {label} projection")
    base_document["result_envelope"]["retained_precision"] = deepcopy(source["retained_precision"])
    check("results.v0.3.schema.yaml", base_document, False, f"Y9 {label} receipt on the base branch")


def test_rv78_n2_y4_member_is_shape_valid_where_it_belongs():
    """Y4's foreign member is a real receipt, accepted on its own identity's
    branch, so its refusal on the successor branch is the branch's."""
    raw = json.loads((ROOT / "fixtures/product_preview" / SOURCE_RAW).read_text())
    document = results_document(raw)
    check("results.v0.3.schema.yaml", document, False, "pressure-source n05 without its receipt")
    document["result_envelope"]["source_block_recovery"] = foreign_source_block_recovery()
    check("results.v0.3.schema.yaml", document, True, "pressure-source n05 with its own receipt")
    historical = json.loads((ROOT / "fixtures/results/preview_physics_connected_sparse.json").read_text())
    check("results.v0.3.schema.yaml", results_document(historical), True, "Y10 historical preview")


def test_d_u6_2_class_codes_are_admitted_only_in_the_successor_branch():
    schema = carrier_schema("results.v0.3.schema.yaml")
    enum = schema["$defs"]["RowDisclosure"]["properties"]["reason_code"]["enum"]
    assert enum[-2:] == CODES and len(set(enum)) == len(enum)
    clause = {"properties": {"row_disclosures": {"items": {"properties": {"reason_code": {"not": {"enum": CODES}}}}}}}
    from tests.schema_validation import envelope_branches
    branches = envelope_branches(schema)  # pressure-2's branch is held in results_pressure_2.v0.3
    for branch in branches:
        sid = branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"]
        if sid in SUCCESSORS:
            assert "allOf" not in branch
        else:
            assert branch["allOf"] == [{"not": {"required": ["retained_precision"]}}, clause], sid
    for sid in SUCCESSORS:
        assert sum(1 for b in branches if b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == sid) == 1
    source = milestone("sparse_interactive")
    for code in CODES + ["diagnostic_evidence_not_physical_quantity"]:
        document = results_document(source)
        document["result_envelope"]["row_disclosures"] = [disclosure(code)]
        check("results.v0.3.schema.yaml", document, True, f"successor {code}")
        base_document = results_document(projected(source))
        base_document["result_envelope"]["row_disclosures"] = [disclosure(code)]
        check("results.v0.3.schema.yaml", base_document, code not in CODES, f"base {code}")
    document = results_document(source)
    document["result_envelope"]["row_disclosures"] = [disclosure("retained_precision_unknown")]
    check("results.v0.3.schema.yaml", document, False, "unknown code")


def foreign(path, member):
    return deepcopy(json.loads((ROOT / "fixtures/product_preview" / path).read_text())[member])


def analysis_record(source):
    from core.analysis_runs.compatibility import build_analysis_run
    with checked_json_fixture_cache():
        return build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:u6c-schema"}, input_manifest_hash="1" * 64)


def as_successor_record(record, receipt):
    """The successor's record shape (D2 4.9.6): its identity, and the complete receipt."""
    record = deepcopy(record)
    run = record["analysis_run"]
    run["reproducibility"]["semantic_contract"] = {"id": SUCC, "sha256": SUCC_SHA}
    for ref in run["result_refs"]:
        ref["semantic_contract"]["id"], ref["semantic_contract"]["sha256"] = SUCC, SUCC_SHA
    run["retained_precision"] = deepcopy(receipt)
    return record


@pytest.mark.parametrize("mode", sorted(MILESTONES))
def test_analysis_run_successor_branch_requires_the_receipt_and_only_it(mode):
    source = milestone(mode)
    base = analysis_record(projected(source))
    check("analysis_run.schema.json", base, True, "base preview record")
    record = as_successor_record(base, source["retained_precision"])
    check("analysis_run.schema.json", record, True, "successor record")
    check("analysis_run.v0.3.schema.json", record, True, "successor record (exact version)")
    refused = {
        "no_receipt": lambda r: r["analysis_run"].pop("retained_precision"),
        "receipt_policy_v1": lambda r: r["analysis_run"]["retained_precision"]["body"].__setitem__("policy", "M03-INTEGRITY-MP-v1"),
        "receipt_unknown_member": lambda r: r["analysis_run"]["retained_precision"].__setitem__("unknown", 1),
        # Shape-valid foreign members (a physics-source receipt, physics-source
        # evidence): only the successor branch's own exclusions refuse them.
        "source_block_recovery": lambda r: r["analysis_run"].__setitem__("source_block_recovery", foreign(SOURCE_RAW, "source_block_recovery")),
        "contract_evidence": lambda r: r["analysis_run"].__setitem__("contract_evidence", foreign(SOURCE_RAW, "contract_evidence")),
        "preview_sha": lambda r: r["analysis_run"]["reproducibility"]["semantic_contract"].__setitem__("sha256", PREVIEW_SHA),
        "row_identity_mixed": lambda r: r["analysis_run"]["result_refs"][0]["semantic_contract"].update(id=PREVIEW, sha256=PREVIEW_SHA),
    }
    for name, edit in refused.items():
        changed = deepcopy(record)
        edit(changed)
        check("analysis_run.schema.json", changed, False, name)
    receipted = deepcopy(base)
    receipted["analysis_run"]["retained_precision"] = deepcopy(source["retained_precision"])
    check("analysis_run.schema.json", receipted, False, "receipt on the preview branch")


def test_every_existing_analysis_run_branch_refuses_a_receipt():
    schema = carrier_schema("analysis_run.v0.3.schema.json")
    run = schema["$defs"]["AnalysisRun"]
    assert run["properties"]["retained_precision"] == {"$ref": "retained_precision_mp_v2.schema.json"}
    ids = []
    for branch in run["oneOf"]:
        sid = branch["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["id"]["const"]
        ids.append(sid)
        refused = {"required": ["retained_precision"]} in branch.get("not", {}).get("anyOf", [])
        assert refused is (sid not in SUCCESSORS), sid
        assert ("retained_precision" in branch.get("required", [])) is (sid in SUCCESSORS), sid
    # T4-U2 added pressure-1; the pressure-source join added pressure-source-1;
    # B1 removed precision-1 and B4/D1 (b) source-blocks-1; B7b the four exact v2 identities.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 5
    contract = schema["$defs"]["SemanticContract"]
    assert {"properties": {"id": {"const": SUCC}, "sha256": {"const": SUCC_SHA}}} in contract["oneOf"]
    # Two receipt-carrying identities without a former `not` now refuse a receipt.
    for path in ("fixtures/product_preview/" + SOURCE_RAW,):
        raw = json.loads((ROOT / path).read_text())
        record = analysis_record(raw)
        check("analysis_run.schema.json", record, True, path)
        record["analysis_run"]["retained_precision"] = deepcopy(CORPUS["cases"][0]["source"]["retained_precision"])
        check("analysis_run.schema.json", record, False, path + " with receipt")


def stress_neutral_package(source):
    from core.handoff.stress_neutral import package as sn
    from tests.test_stress_neutral_physics_source import arguments
    with checked_json_fixture_cache():
        record = analysis_record(source)
        return sn.build_stress_neutral_export_package(source_envelope=source, analysis_record=record, **arguments(source, record))


def as_successor_package(packet, receipt):
    packet = deepcopy(packet)
    packet["producer"]["semantic_contract_id"] = SUCC
    packet["semantic_contract_ref"]["ref_id"] = SUCC
    packet["semantic_contract"] = {"id": SUCC, "sha256": SUCC_SHA}
    packet["formulation_basis"]["profile_id"] = PROFILE
    packet["retained_precision"] = deepcopy(receipt)
    return packet


@pytest.mark.parametrize("mode", sorted(MILESTONES))
def test_stress_neutral_successor_branch_is_transport_shape_only(mode):
    """The branch admits the transported receipt's shape; the packager itself still
    refuses the successor (SN-SOURCE-METHOD-UNSUPPORTED), so T6's refusal stands."""
    from core.handoff.stress_neutral import package as sn
    source = milestone(mode)
    base = stress_neutral_package(projected(source))
    check("stress_neutral_export.schema.json", base, True, "base preview package")
    packet = as_successor_package(base, source["retained_precision"])
    check("stress_neutral_export.schema.json", packet, True, "successor package shape")
    check("stress_neutral_export.v0.3.schema.json", packet, True, "successor package shape (exact version)")
    refused = {
        "no_receipt": lambda p: p.pop("retained_precision"),
        "receipt_policy_v1": lambda p: p["retained_precision"]["body"].__setitem__("policy", "M03-INTEGRITY-MP-v1"),
        "preview_profile": lambda p: p["formulation_basis"].__setitem__("profile_id", "product_preview_mechanics_v1"),
        "preview_sha": lambda p: p["semantic_contract"].__setitem__("sha256", PREVIEW_SHA),
        "contract_ref_mismatch": lambda p: p["semantic_contract_ref"].__setitem__("ref_id", PREVIEW),
        "source_block_recovery": lambda p: p.__setitem__("source_block_recovery", foreign_source_block_recovery()),
        "no_contract_evidence": lambda p: p.pop("contract_evidence"),
        # RV88 U6c N-4 (W06): the successor branch requires the source annotations.
        "no_source_annotations": lambda p: p.pop("source_annotations"),
    }
    for name, edit in refused.items():
        changed = deepcopy(packet)
        edit(changed)
        check("stress_neutral_export.schema.json", changed, False, name)
    receipted = deepcopy(base)
    receipted["retained_precision"] = deepcopy(source["retained_precision"])
    check("stress_neutral_export.schema.json", receipted, False, "receipt on the preview branch")
    with pytest.raises(ValueError, match="SN-SOURCE-METHOD-UNSUPPORTED|SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"):
        stress_neutral_package(source)


def test_every_existing_stress_neutral_branch_refuses_a_receipt():
    schema = carrier_schema("stress_neutral_export.v0.3.schema.json")
    props = schema["properties"]
    assert props["retained_precision"] == {"$ref": "retained_precision_mp_v2.schema.json"}
    for enum in (props["producer"]["properties"]["semantic_contract_id"]["enum"],
                 props["semantic_contract_ref"]["properties"]["ref_id"]["enum"],
                 props["semantic_contract"]["properties"]["id"]["enum"]):
        assert enum[-2:] == list(SUCCESSORS)
    assert props["formulation_basis"]["properties"]["profile_id"]["enum"][-2:] == [PROFILE, EXACT_PROFILE]
    ids = []
    for branch in schema["oneOf"]:
        sid = branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"]
        ids.append(sid)
        assert ({"required": ["retained_precision"]} in branch.get("not", {}).get("anyOf", [])) is (sid not in SUCCESSORS), sid
        assert ("retained_precision" in branch.get("required", [])) is (sid in SUCCESSORS), sid
    # T4-U2 added pressure-1; B1 removed precision-1 and B4/D1 (b) source-blocks-1; B7b the four exact v2
    # identities and added pressure-source-1's branch.
    assert ids.count(SUCC) == 1 and ids.count(EXACT) == 1 and ids.count(PRESSURE_SOURCE) == 1 and len(ids) == 5


# ---- B3b (I101, lane T): the exact successor's carrier branches (B3-D §7; REVISION_01 §6).
# Shape checks only, on a pressure-2 fixture (B7b, D4 (a): the exact successor's G7 projection is
# pressure-2/exact_pressure_v3) re-stated under the exact identity with a shape-valid receipt:
# schema validity creates no standing.

PLAIN = "fixtures/results/pressure_connected_mechanics_sparse.json"


def exact_source():
    """A pressure-2 statement under the exact successor's identity and profile (its limitations are pressure-2's,
    the branch's const), carrying the shared corpus's first receipt (shape only; no reader runs here)."""
    source = json.loads((ROOT / PLAIN).read_text())
    assert source["producer"]["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/pressure-2"
    source = deepcopy(source)
    source["producer"]["semantic_contract_id"] = EXACT
    source["formulation_basis"]["profile_id"] = EXACT_PROFILE
    source["retained_precision"] = deepcopy(CORPUS["cases"][0]["source"]["retained_precision"])
    return source


def test_exact_successor_results_branch():
    schema = carrier_schema("results.v0.3.schema.yaml")
    branch = schema["$defs"]["ResultEnvelope"]["oneOf"][-1]
    assert branch["properties"]["producer"]["properties"]["semantic_contract_id"] == {"const": EXACT}
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": EXACT_PROFILE}
    # D4 (a): the exact successor's evidence is the G7 transport shape over pressure-2's evidence.
    assert branch["properties"]["contract_evidence"] == {"$ref": "#/$defs/ExactRetainedContractEvidence"}
    assert branch["properties"]["result_sets"] == {"items": {"$ref": "#/$defs/ResultSet"}}
    assert branch["properties"]["retained_precision"] == {"$ref": "retained_precision_mp_v2.schema.json"}
    assert branch["required"] == ["contract_evidence", "retained_precision"]
    source = exact_source()
    check("results.v0.3.schema.yaml", results_document(source), True, "exact successor")
    for code in CODES:
        document = results_document(source)
        document["result_envelope"]["row_disclosures"] = [disclosure(code)]
        check("results.v0.3.schema.yaml", document, True, f"exact successor {code}")
    refused = {
        "no_receipt": lambda s: s.pop("retained_precision"),
        "preview_profile": lambda s: s["formulation_basis"].__setitem__("profile_id", PROFILE),
        "physics_profile": lambda s: s["formulation_basis"].__setitem__("profile_id", "exact_straight_pressure_v2"),
        "pressure_profile": lambda s: s["formulation_basis"].__setitem__("profile_id", "exact_pressure_v3"),
        "other_limitations": lambda s: s["formulation_basis"].__setitem__("limitations", ["Straight circular members and linear restraints or springs only."]),
        "preview_evidence": lambda s: s.__setitem__("contract_evidence", deepcopy(CORPUS["cases"][0]["source"]["contract_evidence"])),
        "source_block_recovery": lambda s: s.__setitem__("source_block_recovery", foreign_source_block_recovery()),
    }
    for name, edit in refused.items():
        document = results_document(source)
        edit(document["result_envelope"])
        check("results.v0.3.schema.yaml", document, False, name)


def test_exact_successor_analysis_run_branch():
    plain = json.loads((ROOT / PLAIN).read_text())
    base = analysis_record(plain)
    check("analysis_run.schema.json", base, True, "base pressure-2 record")
    record = deepcopy(base)
    run = record["analysis_run"]
    run["reproducibility"]["semantic_contract"] = {"id": EXACT, "sha256": EXACT_SHA}
    for ref in run["result_refs"]:
        ref["semantic_contract"]["id"], ref["semantic_contract"]["sha256"] = EXACT, EXACT_SHA
    run["retained_precision"] = deepcopy(CORPUS["cases"][0]["source"]["retained_precision"])
    check("analysis_run.v0.3.schema.json", record, True, "exact successor record")
    refused = {
        "no_receipt": lambda r: r["analysis_run"].pop("retained_precision"),
        "preview_successor_sha": lambda r: r["analysis_run"]["reproducibility"]["semantic_contract"].__setitem__("sha256", SUCC_SHA),
        "contract_evidence": lambda r: r["analysis_run"].__setitem__("contract_evidence", foreign(SOURCE_RAW, "contract_evidence")),
        "source_block_recovery": lambda r: r["analysis_run"].__setitem__("source_block_recovery", foreign(SOURCE_RAW, "source_block_recovery")),
        "row_identity_mixed": lambda r: r["analysis_run"]["result_refs"][0]["semantic_contract"].update(id=SUCC, sha256=SUCC_SHA),
    }
    for name, edit in refused.items():
        changed = deepcopy(record)
        edit(changed)
        check("analysis_run.v0.3.schema.json", changed, False, name)
    assert [{"properties": {"id": {"const": EXACT}, "sha256": {"const": EXACT_SHA}}},
            {"properties": {"id": {"const": PRESSURE_SOURCE}, "sha256": {"const": PRESSURE_SOURCE_SHA}}}] == carrier_schema("analysis_run.v0.3.schema.json")["$defs"]["SemanticContract"]["oneOf"][-2:]


def test_exact_successor_stress_neutral_branch():
    plain = json.loads((ROOT / PLAIN).read_text())
    base = stress_neutral_package(plain)
    check("stress_neutral_export.v0.3.schema.json", base, True, "base pressure-2 package")
    packet = deepcopy(base)
    packet["producer"]["semantic_contract_id"] = EXACT
    packet["semantic_contract_ref"]["ref_id"] = EXACT
    packet["semantic_contract"] = {"id": EXACT, "sha256": EXACT_SHA}
    packet["formulation_basis"]["profile_id"] = EXACT_PROFILE
    packet["retained_precision"] = deepcopy(CORPUS["cases"][0]["source"]["retained_precision"])
    check("stress_neutral_export.v0.3.schema.json", packet, True, "exact successor package shape")
    refused = {
        "no_receipt": lambda p: p.pop("retained_precision"),
        "preview_profile": lambda p: p["formulation_basis"].__setitem__("profile_id", PROFILE),
        "preview_successor_sha": lambda p: p["semantic_contract"].__setitem__("sha256", SUCC_SHA),
        "preview_evidence": lambda p: p.__setitem__("contract_evidence", deepcopy(CORPUS["cases"][0]["source"]["contract_evidence"])),
        "source_block_recovery": lambda p: p.__setitem__("source_block_recovery", foreign_source_block_recovery()),
        "no_source_annotations": lambda p: p.pop("source_annotations"),
    }
    for name, edit in refused.items():
        changed = deepcopy(packet)
        edit(changed)
        check("stress_neutral_export.v0.3.schema.json", changed, False, name)
