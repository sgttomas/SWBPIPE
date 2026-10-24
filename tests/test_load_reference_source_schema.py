"""Carrier schema branches for pressure-source-1 on load/reference-state publications (B7b).

Shape only. B7b retired the joined identity load-reference-source-1 (`resolved_straight_load_state_source_v1`, policy
LOAD-REFERENCE-SOURCE-1) with `2.0.0/exact_straight_pressure_v2`. A load/reference-state model whose case publishes
the selected retained-source response now publishes pressure-source-1 (`3.0.0/exact_pressure_v3`, policy
PRESSURE-SOURCE-1) with `contract_evidence.load_reference_states`: physics_source_recovery.schema.json's
pressure_source_resolved_evidence beside its ordinary evidence, under the one PRESSURE-SOURCE-1 receipt (the schema
root). Receipt hashes, case binding, case order and the "at least one selected case" rule are executable reader
validation and are not asserted here.

Positive instances:
- the ten current producer raws (verbatim evidence and receipt);
- a minimal results 0.3 scaffold around their verbatim transport metadata;
- the actual AnalysisRun and stress-neutral carriers (fixtures/results/pressure_source_load_state_*) the product's
  Python route builds, and the actual result documents beside them.

Each carrier matches the pressure-source-1 branch only; the retired identities have no branch.
"""
from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
import json
from pathlib import Path
from urllib.parse import urljoin

import pytest

from schema_validation import envelope_branches, validate_schema_document

PROJECT = Path(__file__).resolve().parents[1]
SCHEMAS = PROJECT / "schemas"
JOINED = PROJECT / "fixtures/product_preview/pressure_source/load_reference_source"
COMPOSITE = PROJECT / "fixtures/product_preview/pressure_source/physics_source"
LOAD_STATE = PROJECT / "fixtures/product_preview/pressure/load_reference"
RESULTS_FIXTURES = PROJECT / "fixtures/results"

RESULTS = "results.v0.3.schema.yaml"
RUN = "analysis_run.v0.3.schema.json"
RUN_ROOT = "analysis_run.schema.json"
PACKAGE = "stress_neutral_export.v0.3.schema.json"
PSR = "physics_source_recovery.schema.json"
LRSCHEMA = "load_reference_state.schema.json"

PRS_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
# T4-U6 (D-5, DEL-05-03 section-bases-and-timoshenko.md §2.4): the per-basis stress signatures (88).
PRS_SHA = "b5a4b0e585b9ec440d36144860baaf101b628498cfdab9a1de9248c2ed5069e7"
PRS_POLICY = "PRESSURE-SOURCE-1"
PROFILE = "exact_pressure_v3"
PRESSURE_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
PRESSURE_SHA = "9183ef9b83f879620df45c03994f71808cc9c04c4a093c86b0bedfca2f96e59f"
PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
SUCCESSOR_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
EXACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
RETAINED = "retained_source_blocks_exact_v1"
# The retired v2 identities, profiles and policies (each refused by name by the readers).
LR_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
LRS_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
PS_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
LR_SHA = "e41657242c65fe063b0cc3a82248191eb998dff9c80afdae03c8a7a419aa7f09"
LRS_SHA = "28f454334352a089bd05e4773a756806b707113b732df986eb89eb7e800d5924"
PS_SHA = "0309b16e2ccb4ec40eaa0c3e7d339ed0d41693ee85dc4fc736d1656cf2702f53"
LRS_PROFILE = "resolved_straight_load_state_source_v1"
PS_PROFILE = "exact_straight_pressure_v2"
CARRIER_ORDER = [PRS_ID, PREVIEW_ID, PRESSURE_ID, SUCCESSOR_ID, EXACT_ID]
BRANCH_OF = {contract_id: index for index, contract_id in enumerate(CARRIER_ORDER)}
EXPORT_BRANCH_OF = BRANCH_OF
RESOLVED = {"$ref": f"{PSR}#/$defs/pressure_source_physical_evidence"}
CARRIER_STEMS = [f"{stem}_{mode}" for stem in ("eigen_motion", "fields", "mixed", "n05", "n06") for mode in ("sparse", "dense")]


# ---------------------------------------------------------------- strict loading and a shared registry
def _reject_constant(token):
    raise ValueError(f"non-JSON constant {token}")


def _unique_object(pairs):
    keys = [key for key, _ in pairs]
    assert len(keys) == len(set(keys)), f"duplicate keys {keys}"
    return dict(pairs)


def strict_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"), parse_constant=_reject_constant,
                      object_pairs_hook=_unique_object)


@lru_cache(maxsize=None)
def _schema_text(name):
    return (SCHEMAS / name).read_text(encoding="utf-8")


def schema(name):
    return json.loads(_schema_text(name))


@lru_cache(maxsize=None)
def _registry():
    """The same local resources as tests/schema_validation.py, built once (no network retriever)."""
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012
    resources = {}
    for path in SCHEMAS.glob("*.schema.*"):
        try:
            candidate = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            continue
        resource = Resource.from_contents(candidate, default_specification=DRAFT202012)
        urls = {path.resolve().as_uri(), urljoin("https://openpipestress.org/schemas/", path.name), path.name}
        if "$id" in candidate:
            urls.add(candidate["$id"])
        resources.update({url: resource for url in urls})
    return Registry().with_resources(resources.items())


@lru_cache(maxsize=None)
def _checked(text):
    from jsonschema import Draft202012Validator
    document = json.loads(text)
    validate_schema_document(document)
    return Draft202012Validator(document, registry=_registry())


def errors(schema_document, instance):
    """A failure witness, or none: callers assert validity, not an error census.

    Every carrier branch is still checked. A rejected branch can stop at its
    first constraint failure instead of traversing and formatting the rest of
    an already-rejected large receipt.
    """
    validator = _checked(json.dumps(schema_document, sort_keys=True))
    error = next(validator.iter_errors(instance), None)
    return [] if error is None else [f"{list(error.absolute_path)}: {error.message}"[:300]]


def valid(schema_document, instance, label="instance"):
    found = errors(schema_document, instance)
    assert not found, f"{label} failed JSON Schema validation:\n" + "\n".join(found[:10])


def invalid(schema_document, instance, label="instance"):
    assert errors(schema_document, instance), f"{label} was accepted"


def definition(name, root=RESULTS):
    """One definition as a root of its own. The receipt schema's `$id` is not a URL base, so its definitions are
    rooted without it: their sibling references then resolve by file name, as the readers' resources do."""
    document = schema(root)
    rooted = {"$schema": document["$schema"], "$defs": document["$defs"], "$ref": f"#/$defs/{name}"}
    if root != PSR:
        rooted["$id"] = document["$id"]
    return rooted


# ---------------------------------------------------------------- branch mapping
def _branches(name):
    document = schema(name)
    if name == RESULTS:
        return document, document["$defs"]["ResultEnvelope"]["oneOf"], lambda instance: instance["result_envelope"]
    if name == RUN:
        return document, document["$defs"]["AnalysisRun"]["oneOf"], lambda instance: instance["analysis_run"]
    return document, document["oneOf"], lambda instance: instance


def matching_branches(name, instance):
    """Indices of the carrier oneOf branches the instance satisfies, each checked alone."""
    document, branches, part = _branches(name)
    found = []
    for index, branch in enumerate(branches):
        alone = {"$schema": document["$schema"], "$id": document["$id"], "$defs": document["$defs"], **branch}
        if not errors(alone, part(instance)):
            found.append(index)
    return found


def carrier_valid(name, instance, contract_id, label):
    valid(schema(name), instance, f"{label} via {name}")
    if name == RUN:
        valid(schema(RUN_ROOT), instance, f"{label} via {RUN_ROOT}")
    branch_of = EXPORT_BRANCH_OF if name == PACKAGE else BRANCH_OF
    assert matching_branches(name, instance) == [branch_of[contract_id]], label


def carrier_invalid(name, instance, label):
    invalid(schema(name), instance, f"{label} via {name}")


# ---------------------------------------------------------------- inputs
def joined_paths():
    paths = sorted(JOINED.glob("*.raw.json"))
    assert len(paths) == 10, "the ten current pressure-source-1 load/reference-state raws (five witnesses x sparse/dense) are required"
    return paths


def joined(stem="mixed-sparse_interactive"):
    return strict_json(JOINED / f"{stem}.raw.json")


def carrier(stem, kind):
    return strict_json(RESULTS_FIXTURES / f"pressure_source_load_state_{stem}.{kind}.json")


def _results_document(source, contract_id):
    """Minimal results 0.3 scaffold around verbatim raw producer metadata (as in test_load_reference_schema)."""
    from test_load_reference_schema import _results_document as scaffold
    evidence = source.get("contract_evidence")
    document = scaffold({**source, "contract_evidence": evidence}, contract_id)
    envelope = document["result_envelope"]
    if evidence is None:
        del envelope["contract_evidence"]
    if "source_block_recovery" in source:
        envelope["source_block_recovery"] = deepcopy(source["source_block_recovery"])
    return document


def relabelled(record, contract_id, sha):
    record = deepcopy(record)
    run = record["analysis_run"]
    for item in [run["reproducibility"]] + run["result_refs"]:
        item["semantic_contract"].update(id=contract_id, sha256=sha)
    return record


# ---------------------------------------------------------------- schema documents and identity
@pytest.mark.parametrize("name", (RESULTS, RUN, PACKAGE, RUN_ROOT, PSR, LRSCHEMA))
def test_carrier_schemas_parse_strictly_and_are_draft_2020_12(name):
    document = strict_json(SCHEMAS / name)
    assert document["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    validate_schema_document(document, schema_label=name)


def test_retired_joined_identity_has_no_table_definition_or_branch():
    assert not (RESULTS_FIXTURES / "semantic_contract_v0_3_load_reference_source_1.json").exists()
    defs = schema(RESULTS)["$defs"]
    for name in ("LoadReferenceSourceRecovery", "LoadReferenceSourceReceiptBody", "LoadReferenceSourceContractEvidence",
                 "LoadReferenceSourceExactCase", "LoadReferenceSourceStateRecord"):
        assert name not in defs, name
    for name in (RESULTS, RUN, PACKAGE, PSR, LRSCHEMA):
        text = (SCHEMAS / name).read_text(encoding="utf-8")
        assert LRS_ID not in text and LRS_PROFILE not in text and "LOAD-REFERENCE-SOURCE-1" not in text, name


def test_pressure_source_branches_carry_the_resolved_evidence_with_pinned_identities():
    ids = [branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] for branch in envelope_branches(schema(RESULTS))]
    assert ids == CARRIER_ORDER
    results = schema(RESULTS)["$defs"]
    branch = results["ResultEnvelope"]["oneOf"][BRANCH_OF[PRS_ID]]
    assert branch["required"] == ["source_block_recovery", "contract_evidence"]
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": PROFILE}
    assert branch["properties"]["source_block_recovery"] == {"$ref": "#/$defs/PressureSourceRecovery"}
    assert branch["properties"]["contract_evidence"] == {"$ref": "#/$defs/PressureSourceContractEvidence"}
    assert results["PressureSourceContractEvidence"] == RESOLVED
    assert results["PressureSourceRecovery"] == {"$ref": PSR}
    psr = schema(PSR)["$defs"]
    assert psr["pressure_source_physical_evidence"] == {"oneOf": [{"$ref": "#/$defs/pressure_source_ordinary_evidence"}, {"$ref": "#/$defs/pressure_source_resolved_evidence"}]}
    run = schema(RUN)["$defs"]
    pairs = {item["properties"]["id"]["const"]: item["properties"]["sha256"]["const"] for item in run["SemanticContract"]["oneOf"]}
    assert pairs[PRS_ID] == PRS_SHA and pairs[PRESSURE_ID] == PRESSURE_SHA
    assert run["AnalysisRun"]["properties"]["contract_evidence"] == RESOLVED
    assert run["AnalysisRun"]["properties"]["source_block_recovery"] == {"$ref": PSR}
    package = schema(PACKAGE)
    package_ids = [item["properties"]["semantic_contract"]["properties"]["id"]["const"] for item in package["oneOf"]]
    assert package_ids == CARRIER_ORDER
    branch = package["oneOf"][EXPORT_BRANCH_OF[PRS_ID]]
    assert branch["properties"]["semantic_contract"]["properties"] == {"id": {"const": PRS_ID}, "sha256": {"const": PRS_SHA}}
    assert branch["properties"]["contract_evidence"] == RESOLVED
    assert branch["properties"]["source_block_recovery"] == {"$ref": PSR}
    assert branch["required"] == ["contract_evidence", "source_block_recovery", "source_annotations"]


def test_resolved_definitions_reuse_the_load_reference_state_shapes():
    psr, lrs = schema(PSR)["$defs"], schema(LRSCHEMA)["$defs"]
    exact = psr["pressure_source_resolved_exact_case"]
    assert exact["properties"]["material_basis"] == {"const": "resolved_per_member_load_reference_state_v1"}
    assert exact["properties"]["pipe_materials"]["items"] == {"$ref": f"{LRSCHEMA}#/$defs/LoadReferencePipeMaterial"}
    record = psr["pressure_source_state_record"]
    assert record["required"] == lrs["LoadReferenceStateRecord"]["required"]
    region = psr["pressure_source_resolved_pressure_region"]
    assert region["properties"]["profile_mode"] == {"const": PROFILE} and region["properties"]["profile_version"] == {"const": "3.0.0"}
    evidence = psr["pressure_source_resolved_evidence"]
    assert evidence["required"] == ["pressure", "connector", "exact_cases", "load_reference_states"]
    assert evidence["additionalProperties"] is False and evidence["properties"]["connector"] == {"const": []}


# ---------------------------------------------------------------- the ten raws
@pytest.mark.parametrize("path", joined_paths(), ids=lambda path: path.stem)
def test_raw_evidence_and_receipt_validate(path):
    source = strict_json(path)
    assert source["producer"]["semantic_contract_id"] == PRS_ID
    assert source["formulation_basis"]["profile_id"] == PROFILE
    valid(definition("pressure_source_resolved_evidence", PSR), source["contract_evidence"], path.stem)
    valid(definition("pressure_source_physical_evidence", PSR), source["contract_evidence"], path.stem)
    valid(schema(PSR), source["source_block_recovery"], path.stem)
    assert source["source_block_recovery"]["body"]["policy"] == PRS_POLICY


@pytest.mark.parametrize("path", joined_paths(), ids=lambda path: path.stem)
def test_raw_transport_metadata_validates_on_the_pressure_source_branch(path):
    carrier_valid(RESULTS, _results_document(strict_json(path), PRS_ID), PRS_ID, f"{path.stem} scaffold")


def test_raws_cover_both_selected_and_not_joined_records():
    seen = set()
    for path in joined_paths():
        for record in strict_json(path)["contract_evidence"]["load_reference_states"]:
            seen.add((record["solve"]["recovery_method"], record["source_recovery"]["status"]))
    assert seen == {(RETAINED, "selected"), ("ordinary_sparse_structural_v1", "not_joined"),
                    ("ordinary_dense_structural_v1", "not_joined")}


# ---------------------------------------------------------------- the actual carriers
@pytest.mark.parametrize("stem", CARRIER_STEMS)
def test_actual_carriers_validate_on_the_pressure_source_branch_only(stem):
    record, packet = carrier(stem, "analysis_run"), carrier(stem, "stress_neutral")
    assert "load_reference_states" in record["analysis_run"]["contract_evidence"]
    carrier_valid(RUN, record, PRS_ID, f"{stem} run")
    carrier_valid(PACKAGE, packet, PRS_ID, f"{stem} packet")


def test_actual_result_documents_validate_on_the_pressure_source_branch_only():
    documents = sorted(RESULTS_FIXTURES.glob("pressure_source_load_state_*.document.json"))
    assert [path.name.removesuffix(".document.json").removeprefix("pressure_source_load_state_") for path in documents] == sorted(CARRIER_STEMS)
    for path in documents:
        carrier_valid(RESULTS, strict_json(path), PRS_ID, path.name)


def test_ordinary_pressure_source_carriers_keep_their_branch():
    from test_stress_neutral_physics_source import captured
    raw, analysis, packet = (deepcopy(item) for item in captured(COMPOSITE / "mixed-sparse_interactive.raw.json"))
    assert "load_reference_states" not in raw["contract_evidence"]
    carrier_valid(RESULTS, _results_document(raw, PRS_ID), PRS_ID, "ordinary scaffold")
    carrier_valid(RUN, analysis, PRS_ID, "ordinary run")
    carrier_valid(PACKAGE, packet, PRS_ID, "ordinary packet")


# ---------------------------------------------------------------- evidence and receipt negatives
def _evidence():
    return deepcopy(joined("mixed-sparse_interactive")["contract_evidence"])


def _receipt():
    return deepcopy(joined("mixed-sparse_interactive")["source_block_recovery"])


EVIDENCE_MUTATIONS = (
    "extra_key", "missing_pressure", "connector_nonempty",
    "selected_with_ordinary_method", "not_joined_with_retained_method", "selected_without_method",
    "selected_with_code", "unknown_source_recovery_status", "selected_with_other_method",
    "record_profile_is_retired_joined_profile", "exact_material_basis", "exact_without_recovery_method",
    "exact_pipe_material_without_selection_kind", "record_extra_key", "retired_profile_mode",
)


@pytest.mark.parametrize("mutation", EVIDENCE_MUTATIONS)
def test_resolved_evidence_mutations_are_rejected(mutation):
    evidence = _evidence()
    selected, ordinary = evidence["load_reference_states"]
    assert selected["source_recovery"]["status"] == "selected" and ordinary["source_recovery"]["status"] == "not_joined"
    exact = evidence["exact_cases"][0]
    if mutation == "extra_key": evidence["source_receipt"] = {}
    elif mutation == "missing_pressure": del evidence["pressure"]
    elif mutation == "connector_nonempty": evidence["connector"] = [{}]
    elif mutation == "selected_with_ordinary_method": selected["solve"]["recovery_method"] = "ordinary_sparse_structural_v1"
    elif mutation == "not_joined_with_retained_method": ordinary["solve"]["recovery_method"] = RETAINED
    elif mutation == "selected_without_method": del selected["source_recovery"]["method"]
    elif mutation == "selected_with_code": selected["source_recovery"]["code"] = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
    elif mutation == "unknown_source_recovery_status": selected["source_recovery"]["status"] = "joined"
    elif mutation == "selected_with_other_method": selected["source_recovery"]["method"] = "ordinary_sparse_structural_v1"
    elif mutation == "record_profile_is_retired_joined_profile": selected["profile"] = LRS_PROFILE
    elif mutation == "exact_material_basis": exact["material_basis"] = "resolved_per_case_v1"
    elif mutation == "exact_without_recovery_method": del exact["recovery_method"]
    elif mutation == "exact_pipe_material_without_selection_kind": del exact["pipe_materials"][0]["material_selection_kind"]
    elif mutation == "record_extra_key": ordinary["receipt"] = None
    elif mutation == "retired_profile_mode": exact["profile_mode"] = PS_PROFILE
    invalid(definition("pressure_source_physical_evidence", PSR), evidence, mutation)


def test_ordinary_and_resolved_evidence_are_disjoint():
    resolved = _evidence()
    ordinary = strict_json(COMPOSITE / "mixed-sparse_interactive.raw.json")["contract_evidence"]
    valid(definition("pressure_source_ordinary_evidence", PSR), ordinary, "ordinary evidence")
    invalid(definition("pressure_source_resolved_evidence", PSR), ordinary, "ordinary evidence as resolved")
    invalid(definition("pressure_source_ordinary_evidence", PSR), resolved, "resolved evidence as ordinary")
    # The ordinary pressure-2 load/reference-state evidence (no recovery method) is not pressure-source-1's.
    load_state = strict_json(LOAD_STATE / "connected-sparse_interactive.raw.json")["contract_evidence"]
    invalid(definition("pressure_source_physical_evidence", PSR), load_state, "pressure-2 load-state evidence as pressure-source-1")


@pytest.mark.parametrize("policy", ("PHYSICS-SOURCE-1", "SOURCE-BLOCKS-1", "LOAD-REFERENCE-SOURCE-1", None))
def test_receipt_policy_is_pinned(policy):
    receipt = _receipt()
    if policy is None:
        del receipt["body"]["policy"]
    else:
        receipt["body"]["policy"] = policy
    invalid(schema(PSR), receipt, f"policy {policy}")
    receipt = _receipt()
    receipt["body"]["extra"] = True
    invalid(schema(PSR), receipt, "receipt body extra key")


# ---------------------------------------------------------------- relabels and mixups per carrier
RESULTS_MUTATIONS = (
    "producer_id", "ref_id", "profile", "missing_receipt", "missing_evidence", "pressure_load_state_evidence",
    "relabel_as_retired_joined", "relabel_as_retired_physics_source", "relabel_as_pressure_1",
)


@pytest.mark.parametrize("mutation", RESULTS_MUTATIONS)
def test_results_pressure_source_branch_refuses_relabels_and_mixups(mutation):
    document = _results_document(joined(), PRS_ID)
    envelope = document["result_envelope"]

    def relabel(contract, profile):
        envelope["producer"]["semantic_contract_id"] = envelope["semantic_contract_ref"]["ref_id"] = contract
        envelope["formulation_basis"]["profile_id"] = profile

    if mutation == "producer_id": envelope["producer"]["semantic_contract_id"] = PRESSURE_ID
    elif mutation == "ref_id": envelope["semantic_contract_ref"]["ref_id"] = PRESSURE_ID
    elif mutation == "profile": envelope["formulation_basis"]["profile_id"] = LRS_PROFILE
    elif mutation == "missing_receipt": del envelope["source_block_recovery"]
    elif mutation == "missing_evidence": del envelope["contract_evidence"]
    elif mutation == "pressure_load_state_evidence":
        envelope["contract_evidence"] = strict_json(LOAD_STATE / "connected-sparse_interactive.raw.json")["contract_evidence"]
    elif mutation == "relabel_as_retired_joined": relabel(LRS_ID, LRS_PROFILE)
    elif mutation == "relabel_as_retired_physics_source": relabel(PS_ID, PS_PROFILE)
    elif mutation == "relabel_as_pressure_1": relabel(PRESSURE_ID, PROFILE)
    carrier_invalid(RESULTS, document, mutation)


RUN_MUTATIONS = (
    "pressure_hash", "retired_joined_pair", "retired_physics_source_pair", "unknown_id", "mixed_rows",
    "missing_receipt", "missing_evidence", "pressure_load_state_evidence", "reproducibility_hash_only",
    "relabel_as_pressure_1",
)


@pytest.mark.parametrize("mutation", RUN_MUTATIONS)
def test_analysis_run_pressure_source_branch_refuses_relabels_and_mixups(mutation):
    record = carrier("mixed_sparse", "analysis_run")
    run = record["analysis_run"]
    if mutation == "pressure_hash": record = relabelled(record, PRS_ID, PRESSURE_SHA)
    elif mutation == "retired_joined_pair": record = relabelled(record, LRS_ID, LRS_SHA)
    elif mutation == "retired_physics_source_pair": record = relabelled(record, PS_ID, PS_SHA)
    elif mutation == "unknown_id": record = relabelled(record, PRS_ID.replace("-1", "-2"), PRS_SHA)
    elif mutation == "mixed_rows":
        for item in run["result_refs"]:
            item["semantic_contract"].update(id=PRESSURE_ID, sha256=PRESSURE_SHA)
    elif mutation == "reproducibility_hash_only": run["reproducibility"]["semantic_contract"]["sha256"] = PRESSURE_SHA
    elif mutation == "missing_receipt": del run["source_block_recovery"]
    elif mutation == "missing_evidence": del run["contract_evidence"]
    elif mutation == "pressure_load_state_evidence":
        run["contract_evidence"] = strict_json(LOAD_STATE / "connected-sparse_interactive.raw.json")["contract_evidence"]
    elif mutation == "relabel_as_pressure_1": record = relabelled(record, PRESSURE_ID, PRESSURE_SHA)
    carrier_invalid(RUN, record, mutation)


@pytest.mark.parametrize("pair", ((PRS_ID, PRESSURE_SHA), (PRS_ID, LRS_SHA), (LRS_ID, LRS_SHA), (LR_ID, LR_SHA), (PS_ID, PS_SHA), (PRESSURE_ID, PRS_SHA)))
def test_semantic_contract_pairs_each_current_id_only_with_its_table_hash(pair):
    valid(definition("SemanticContract", RUN), {"id": PRS_ID, "sha256": PRS_SHA}, "pressure-source pair")
    valid(definition("SemanticContract", RUN), {"id": PRESSURE_ID, "sha256": PRESSURE_SHA}, "pressure pair")
    invalid(definition("SemanticContract", RUN), {"id": pair[0], "sha256": pair[1]}, f"pair {pair}")


PACKAGE_MUTATIONS = (
    "sha", "producer_id", "ref_id", "contract_id", "profile", "missing_receipt", "missing_evidence",
    "pressure_load_state_evidence", "missing_source_annotations", "csv_policy", "relabel_as_retired_joined",
    "relabel_as_pressure_1",
)


@pytest.mark.parametrize("mutation", PACKAGE_MUTATIONS)
def test_stress_neutral_pressure_source_branch_refuses_relabels_and_mixups(mutation):
    packet = carrier("mixed_sparse", "stress_neutral")

    def relabel(contract, sha, profile):
        packet["producer"]["semantic_contract_id"] = packet["semantic_contract_ref"]["ref_id"] = contract
        packet["semantic_contract"] = {"id": contract, "sha256": sha}
        packet["formulation_basis"]["profile_id"] = profile

    if mutation == "sha": packet["semantic_contract"]["sha256"] = PRESSURE_SHA
    elif mutation == "producer_id": packet["producer"]["semantic_contract_id"] = PRESSURE_ID
    elif mutation == "ref_id": packet["semantic_contract_ref"]["ref_id"] = PRESSURE_ID
    elif mutation == "contract_id": packet["semantic_contract"]["id"] = PRESSURE_ID
    elif mutation == "profile": packet["formulation_basis"]["profile_id"] = LRS_PROFILE
    elif mutation == "missing_receipt": del packet["source_block_recovery"]
    elif mutation == "missing_evidence": del packet["contract_evidence"]
    elif mutation == "pressure_load_state_evidence":
        packet["contract_evidence"] = strict_json(LOAD_STATE / "connected-sparse_interactive.raw.json")["contract_evidence"]
    elif mutation == "missing_source_annotations": del packet["source_annotations"]
    elif mutation == "csv_policy":
        for key in ("csv_encoding", "csv_row_order"):
            del packet["export_profile"][key]
    elif mutation == "relabel_as_retired_joined": relabel(LRS_ID, LRS_SHA, LRS_PROFILE)
    elif mutation == "relabel_as_pressure_1": relabel(PRESSURE_ID, PRESSURE_SHA, PROFILE)
    carrier_invalid(PACKAGE, packet, mutation)
