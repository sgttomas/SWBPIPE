"""Carrier schema branches for the joined method load-reference-source-1.

Shape only. The joined identity is openpipestress.result_semantics/0.3.0/
load-reference-source-1 with profile resolved_straight_load_state_source_v1 and
receipt policy LOAD-REFERENCE-SOURCE-1 (CP2_WIRE_ADDENDUM_2 section 5, with the
CP4 corrections). Receipt hashes, case binding, case order and the "at least one
selected case" rule are executable reader validation and are not asserted here.

Positive instances:
- the ten committed joined producer raws (verbatim evidence and receipt);
- a minimal results 0.3 scaffold around their verbatim transport metadata;
- schema-level transforms of the committed load-reference-1 carriers
  (document, AnalysisRun, stress-neutral) and of physics-source-1 carriers
  built by the existing Python route. These transforms are not reader output
  and carry no hash or cross-binding claim;
- the READERS TASK's canonical carriers (fixtures/results/load_reference_source_*),
  when they exist.

Old documents are shown to match the same oneOf branch as before, and never the
new one.
"""
from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
import hashlib
import json
from pathlib import Path
from urllib.parse import urljoin

import pytest

from schema_validation import validate_schema_document

PROJECT = Path(__file__).resolve().parents[1]
SCHEMAS = PROJECT / "schemas"
JOINED = PROJECT / "fixtures/product_preview/load_reference_source"
COMPOSITE = PROJECT / "fixtures/product_preview/physics_source"
RESULTS_FIXTURES = PROJECT / "fixtures/results"

RESULTS = "results.v0.3.schema.yaml"
RUN = "analysis_run.v0.3.schema.json"
RUN_ROOT = "analysis_run.schema.json"
PACKAGE = "stress_neutral_export.v0.3.schema.json"
PSR = "physics_source_recovery.schema.json"
LRSCHEMA = "load_reference_state.schema.json"

PRECISION_ID = "openpipestress.result_semantics/0.3.0/precision-1"
PHYSICS_ID = "openpipestress.result_semantics/0.3.0/physics-1"
SOURCE_BLOCKS_ID = "openpipestress.result_semantics/0.3.0/source-blocks-1"
PS_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
PS_SHA = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"
PS_PROFILE = "exact_straight_pressure_v2"
LR_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
LR_SHA = "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d"
LR_PROFILE = "resolved_straight_load_state_v1"
LRS_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
LRS_SHA = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"
LRS_PROFILE = "resolved_straight_load_state_source_v1"
LRS_POLICY = "LOAD-REFERENCE-SOURCE-1"
RETAINED = "retained_source_blocks_exact_v1"
PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1"
# Carrier order: every branch main carries (T0R's preview-physics-1 included),
# then T1's appended identities. Positions are derived from this pinned order.
CARRIER_ORDER = [PRECISION_ID, PHYSICS_ID, SOURCE_BLOCKS_ID, PS_ID, PREVIEW_ID, LR_ID, LRS_ID]
BRANCH_OF = {contract_id: index for index, contract_id in enumerate(CARRIER_ORDER)}
NEW_BRANCH = BRANCH_OF[LRS_ID]
PS_BRANCH = BRANCH_OF[PS_ID]
NEW_DEFS = ("LoadReferenceSourceRecovery", "LoadReferenceSourceReceiptBody", "LoadReferenceSourceContractEvidence",
            "LoadReferenceSourceExactCase", "LoadReferenceSourceStateRecord")
LOCAL_EVIDENCE = {"$ref": "#/$defs/LoadReferenceSourceContractEvidence"}
LOCAL_RECOVERY = {"$ref": "#/$defs/LoadReferenceSourceRecovery"}
REMOTE_EVIDENCE = {"$ref": f"{RESULTS}#/$defs/LoadReferenceSourceContractEvidence"}
REMOTE_RECOVERY = {"$ref": f"{RESULTS}#/$defs/LoadReferenceSourceRecovery"}


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
    validator = _checked(json.dumps(schema_document, sort_keys=True))
    return sorted(f"{list(error.absolute_path)}: {error.message}"[:300] for error in validator.iter_errors(instance))


def valid(schema_document, instance, label="instance"):
    found = errors(schema_document, instance)
    assert not found, f"{label} failed JSON Schema validation:\n" + "\n".join(found[:10])


def invalid(schema_document, instance, label="instance"):
    assert errors(schema_document, instance), f"{label} was accepted"


def definition(name, root=RESULTS):
    document = schema(root)
    return {"$schema": document["$schema"], "$id": document["$id"], "$defs": document["$defs"], "$ref": f"#/$defs/{name}"}


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
    assert matching_branches(name, instance) == [BRANCH_OF[contract_id]], label


def carrier_invalid(name, instance, label):
    invalid(schema(name), instance, f"{label} via {name}")


# ---------------------------------------------------------------- inputs
def joined_paths():
    paths = sorted(JOINED.glob("*.raw.json"))
    assert len(paths) == 10, "the ten committed joined producer raws (five witnesses x sparse/dense) are required"
    return paths


def joined(stem="mixed-sparse_interactive"):
    return strict_json(JOINED / f"{stem}.raw.json")


def composite_paths():
    paths = sorted(COMPOSITE.glob("*.raw.json"))
    assert len(paths) >= 8, "the committed physics-source-1 producer raws (n05, n06, mixed, fields and variants) are required"
    return paths


LR_CARRIERS = sorted(RESULTS_FIXTURES.glob("load_reference_[cp]*_*.document.json"))
READER_OUTPUTS = sorted(RESULTS_FIXTURES.glob("load_reference_source_*.json"))


def lr_carrier(stem, kind):
    return strict_json(RESULTS_FIXTURES / f"{stem}.{kind}.json")


def lr_stems():
    stems = [path.name.removesuffix(".document.json") for path in LR_CARRIERS]
    assert len(stems) == 4, stems
    return stems


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


@lru_cache(maxsize=None)
def _captured(path):
    from test_stress_neutral_physics_source import captured
    return captured(path)


def captured(path):
    """Actual carriers from the existing Python route (AnalysisRun, stress-neutral)."""
    raw, analysis, packet = _captured(path)
    return deepcopy(raw), deepcopy(analysis), deepcopy(packet)


# ---------------------------------------------------------------- joined transforms
def joined_document_from_lr(stem, source):
    """The actual load-reference-1 document with the joined metadata, evidence and receipt swapped in."""
    document = lr_carrier(stem, "document")
    envelope = document["result_envelope"]
    for key in ("producer", "formulation_basis", "numerical_quality", "contract_evidence", "source_block_recovery"):
        envelope[key] = deepcopy(source[key])
    envelope["semantic_contract_ref"]["ref_id"] = LRS_ID
    return document


def joined_record(record, source, contract_id=LRS_ID, sha=LRS_SHA, rows=None):
    record = deepcopy(record)
    run = record["analysis_run"]
    run["reproducibility"]["semantic_contract"].update(id=contract_id, sha256=sha)
    for item in run["result_refs"]:
        item["semantic_contract"].update(id=rows[0] if rows else contract_id, sha256=rows[1] if rows else sha)
    run["source_block_recovery"] = deepcopy(source["source_block_recovery"])
    run["contract_evidence"] = deepcopy(source["contract_evidence"])
    return record


def joined_packet(packet, source):
    packet = deepcopy(packet)
    for key in ("producer", "formulation_basis", "numerical_quality", "contract_evidence", "source_block_recovery"):
        packet[key] = deepcopy(source[key])
    packet["semantic_contract_ref"]["ref_id"] = LRS_ID
    packet["semantic_contract"] = {"id": LRS_ID, "sha256": LRS_SHA}
    return packet


# ---------------------------------------------------------------- schema documents and identity
@pytest.mark.parametrize("name", (RESULTS, RUN, PACKAGE, RUN_ROOT))
def test_carrier_schemas_parse_strictly_and_are_draft_2020_12(name):
    document = strict_json(SCHEMAS / name)
    assert document["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    validate_schema_document(document, schema_label=name)


def test_joined_table_identity_is_pinned_not_inferred():
    path = RESULTS_FIXTURES / "semantic_contract_v0_3_load_reference_source_1.json"
    assert hashlib.sha256(path.read_bytes()).hexdigest() == LRS_SHA
    table = strict_json(path)
    assert table["semantic_contract_id"] == LRS_ID
    assert table["formulation_profile_id"] == LRS_PROFILE
    assert table["source_block_policy"] == LRS_POLICY


def test_joined_branches_are_appended_with_pinned_identities():
    results = schema(RESULTS)["$defs"]["ResultEnvelope"]
    ids = [branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] for branch in results["oneOf"]]
    assert ids == CARRIER_ORDER
    branch = results["oneOf"][NEW_BRANCH]
    assert branch["required"] == ["source_block_recovery", "contract_evidence"]
    assert "not" not in branch
    assert branch["properties"]["semantic_contract_ref"]["properties"]["ref_id"] == {"const": LRS_ID}
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": LRS_PROFILE}
    assert branch["properties"]["source_block_recovery"] == LOCAL_RECOVERY
    assert branch["properties"]["contract_evidence"] == LOCAL_EVIDENCE
    assert branch["properties"]["result_sets"] == results["oneOf"][PS_BRANCH]["properties"]["result_sets"]
    props = results["properties"]
    assert props["contract_evidence"]["anyOf"][-1] == LOCAL_EVIDENCE
    assert props["source_block_recovery"]["oneOf"][-1] == LOCAL_RECOVERY
    for key in ("producer", "semantic_contract_ref"):
        field = "semantic_contract_id" if key == "producer" else "ref_id"
        assert props[key]["properties"][field]["enum"][-1] == LRS_ID
    assert props["formulation_basis"]["properties"]["profile_id"]["enum"][-1] == LRS_PROFILE

    run = schema(RUN)["$defs"]
    assert run["SemanticContract"]["oneOf"][-1] == {"properties": {"id": {"const": LRS_ID}, "sha256": {"const": LRS_SHA}}}
    assert run["SemanticContract"]["properties"]["sha256"]["enum"][-1] == LRS_SHA
    branch = run["AnalysisRun"]["oneOf"][NEW_BRANCH]
    assert len(run["AnalysisRun"]["oneOf"]) == len(CARRIER_ORDER)
    assert branch["required"] == ["source_block_recovery", "contract_evidence"]
    assert branch["properties"]["source_block_recovery"] == REMOTE_RECOVERY
    assert branch["properties"]["contract_evidence"] == REMOTE_EVIDENCE
    assert run["AnalysisRun"]["properties"]["contract_evidence"] == {
        "anyOf": [{"$ref": f"{PSR}#/$defs/physical_evidence"}, REMOTE_EVIDENCE]}
    assert run["AnalysisRun"]["properties"]["source_block_recovery"]["oneOf"][-1] == REMOTE_RECOVERY

    package = schema(PACKAGE)
    assert len(package["oneOf"]) == len(CARRIER_ORDER)
    branch = package["oneOf"][NEW_BRANCH]
    assert branch["properties"]["semantic_contract"]["properties"] == {"id": {"const": LRS_ID}, "sha256": {"const": LRS_SHA}}
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": LRS_PROFILE}
    assert branch["properties"]["contract_evidence"] == REMOTE_EVIDENCE
    assert branch["properties"]["source_block_recovery"] == REMOTE_RECOVERY
    assert branch["required"] == ["contract_evidence", "source_block_recovery", "source_annotations"]
    for key in ("export_profile", "manifest"):
        assert branch["properties"][key] == package["oneOf"][PS_BRANCH]["properties"][key]
    assert schema(RUN_ROOT)["oneOf"][2]["allOf"][1] == {"$ref": RUN}


def _rebased(node, target):
    if isinstance(node, dict):
        return {key: (target + value if key == "$ref" and isinstance(value, str) and value.startswith("#/$defs/")
                      else _rebased(value, target)) for key, value in node.items()}
    if isinstance(node, list):
        return [_rebased(item, target) for item in node]
    return node


def _drop(value, *pointers):
    value = deepcopy(value)
    for pointer in pointers:
        parent = value
        for part in pointer[:-1]:
            parent = parent[part]
        parent.pop(pointer[-1])
    return value


def test_new_definitions_reuse_the_physics_source_and_load_reference_shapes_except_at_documented_pointers():
    defs = schema(RESULTS)["$defs"]
    psr, lrs = schema(PSR), schema(LRSCHEMA)["$defs"]
    assert set(NEW_DEFS) <= set(defs)
    # Receipt: the physics-source-1 receipt envelope and body; only the policy and the body ref differ.
    body = defs["LoadReferenceSourceReceiptBody"]
    assert body["properties"]["policy"] == {"const": LRS_POLICY}
    base = _rebased(psr["$defs"]["receipt_body"], PSR)
    assert _drop(body, ["description"], ["properties", "policy"]) == _drop(base, ["properties", "policy"])
    recovery = defs["LoadReferenceSourceRecovery"]
    assert recovery["properties"]["body"] == {"$ref": "#/$defs/LoadReferenceSourceReceiptBody"}
    envelope = {key: psr[key] for key in ("type", "additionalProperties", "required", "properties")}
    assert _drop(recovery, ["description"], ["properties", "body"]) == _drop(envelope, ["properties", "body"])
    # Exact case: the physics-source-1 exact case; only material_basis and pipe_materials items differ.
    exact = defs["LoadReferenceSourceExactCase"]
    assert exact["properties"]["material_basis"] == {"const": "resolved_per_member_load_reference_state_v1"}
    assert exact["properties"]["pipe_materials"]["items"] == {"$ref": f"{LRSCHEMA}#/$defs/LoadReferencePipeMaterial"}
    base = _rebased(psr["$defs"]["physical_evidence"]["properties"]["exact_cases"]["items"], PSR)
    documented = (["properties", "material_basis"], ["properties", "pipe_materials", "items"])
    assert _drop(exact, ["description"], *documented) == _drop(base, *documented)
    # Record: the load-reference-1 record; only solve's retained member, source_recovery and the pairing differ.
    record = defs["LoadReferenceSourceStateRecord"]
    base = _rebased(lrs["LoadReferenceStateRecord"], LRSCHEMA)
    solve = record["properties"]["solve"]
    assert solve["properties"]["recovery_method"]["enum"] == base["properties"]["solve"]["properties"]["recovery_method"]["enum"] + [RETAINED]
    assert solve["oneOf"] == base["properties"]["solve"]["oneOf"] + [{"properties": {"recovery_method": {"const": RETAINED}}}]
    assert record["properties"]["source_recovery"]["oneOf"][0] == base["properties"]["source_recovery"]
    documented = (["properties", "solve", "properties", "recovery_method"], ["properties", "solve", "oneOf"],
                  ["properties", "source_recovery"])
    assert _drop(record, ["description"], ["oneOf"], *documented) == _drop(base, ["description"], *documented)
    # Evidence: the load-reference-1 evidence namespace; only exact_cases and record items differ.
    evidence = defs["LoadReferenceSourceContractEvidence"]
    assert evidence["required"] == ["pressure", "connector", "exact_cases", "load_reference_states"]
    assert evidence["additionalProperties"] is False
    assert evidence["properties"]["connector"] == {"const": []}
    assert evidence["properties"]["exact_cases"]["items"] == {"$ref": "#/$defs/LoadReferenceSourceExactCase"}
    assert evidence["properties"]["load_reference_states"]["items"] == {"$ref": "#/$defs/LoadReferenceSourceStateRecord"}
    base = _rebased(lrs["LoadReferenceContractEvidence"], LRSCHEMA)
    documented = (["description"], ["properties", "exact_cases", "items"], ["properties", "load_reference_states", "items"])
    assert _drop(evidence, *documented) == _drop(base, *documented)


# ---------------------------------------------------------------- the ten joined raws
@pytest.mark.parametrize("path", joined_paths(), ids=lambda path: path.stem)
def test_joined_raw_evidence_and_receipt_validate(path):
    source = strict_json(path)
    assert source["producer"]["semantic_contract_id"] == LRS_ID
    assert source["formulation_basis"]["profile_id"] == LRS_PROFILE
    valid(definition("LoadReferenceSourceContractEvidence"), source["contract_evidence"], path.stem)
    valid(definition("LoadReferenceSourceRecovery"), source["source_block_recovery"], path.stem)
    assert source["source_block_recovery"]["body"]["policy"] == LRS_POLICY


@pytest.mark.parametrize("path", joined_paths(), ids=lambda path: path.stem)
def test_joined_raw_transport_metadata_validates_in_every_carrier(path):
    source = strict_json(path)
    carrier_valid(RESULTS, _results_document(source, LRS_ID), LRS_ID, f"{path.stem} scaffold")
    stem = "load_reference_connected_" + ("dense" if "dense" in path.stem else "sparse")
    carrier_valid(RESULTS, joined_document_from_lr(stem, source), LRS_ID, f"{path.stem} over {stem} document")
    carrier_valid(RUN, joined_record(lr_carrier(stem, "analysis_run"), source), LRS_ID, f"{path.stem} over {stem} run")
    carrier_valid(PACKAGE, joined_packet(lr_carrier(stem, "stress_neutral"), source), LRS_ID, f"{path.stem} over {stem} packet")


def test_joined_raws_cover_both_selected_and_not_joined_records():
    seen = set()
    for path in joined_paths():
        for record in strict_json(path)["contract_evidence"]["load_reference_states"]:
            seen.add((record["solve"]["recovery_method"], record["source_recovery"]["status"]))
    assert seen == {(RETAINED, "selected"), ("ordinary_sparse_structural_v1", "not_joined"),
                    ("ordinary_dense_structural_v1", "not_joined")}


def test_joined_carriers_over_physics_source_route_carriers_validate():
    source = joined("mixed-sparse_interactive")
    _, analysis, packet = captured(COMPOSITE / "mixed-sparse_interactive.raw.json")
    carrier_valid(RUN, joined_record(analysis, source), LRS_ID, "joined over physics-source run")
    carrier_valid(PACKAGE, joined_packet(packet, source), LRS_ID, "joined over physics-source packet")


# ---------------------------------------------------------------- old documents keep their branch
@pytest.mark.parametrize("stem", lr_stems())
def test_committed_load_reference_1_carriers_keep_their_branch(stem):
    carrier_valid(RESULTS, lr_carrier(stem, "document"), LR_ID, f"{stem} document")
    carrier_valid(RUN, lr_carrier(stem, "analysis_run"), LR_ID, f"{stem} run")
    carrier_valid(PACKAGE, lr_carrier(stem, "stress_neutral"), LR_ID, f"{stem} packet")


def _route_paths():
    from test_stress_neutral_physics_source import ACTUAL_PATHS
    return list(ACTUAL_PATHS) + composite_paths()


@pytest.mark.parametrize("path", _route_paths(), ids=lambda path: f"{path.parent.name}/{path.stem}")
def test_physics_1_physics_source_1_and_source_blocks_1_carriers_keep_their_branch(path):
    raw, analysis, packet = captured(path)
    contract = raw["producer"]["semantic_contract_id"]
    assert contract in {PHYSICS_ID, PS_ID, SOURCE_BLOCKS_ID}
    carrier_valid(RESULTS, _results_document(raw, contract), contract, f"{path.stem} scaffold")
    carrier_valid(RUN, analysis, contract, f"{path.stem} run")
    carrier_valid(PACKAGE, packet, contract, f"{path.stem} packet")


@pytest.mark.parametrize("name", ("precision_connected_ui_mechanics_sparse", "precision_connected_ui_mechanics_dense"))
def test_precision_1_scaffold_keeps_its_branch(name):
    raw = strict_json(RESULTS_FIXTURES / f"{name}.json")
    carrier_valid(RESULTS, _results_document(raw, PRECISION_ID), PRECISION_ID, name)


# ---------------------------------------------------------------- the READERS TASK's canonical carriers
@pytest.mark.skipif(not READER_OUTPUTS, reason="fixtures/results/load_reference_source_* not yet produced (READERS TASK)")
@pytest.mark.parametrize("path", READER_OUTPUTS or [None], ids=lambda path: path.name if path else "absent")
def test_reader_canonical_carriers_validate_on_the_joined_branch(path):
    document = strict_json(path)
    kind = path.name.rsplit(".", 2)[-2]
    name = {"document": RESULTS, "analysis_run": RUN, "stress_neutral": PACKAGE}[kind]
    carrier_valid(name, document, LRS_ID, path.name)


# ---------------------------------------------------------------- evidence and receipt negatives
def _evidence():
    return deepcopy(joined("mixed-sparse_interactive")["contract_evidence"])


def _receipt():
    return deepcopy(joined("mixed-sparse_interactive")["source_block_recovery"])


EVIDENCE_MUTATIONS = (
    "extra_key", "missing_load_reference_states", "missing_pressure", "connector_nonempty",
    "selected_with_ordinary_method", "not_joined_with_retained_method", "selected_without_method",
    "selected_with_code", "unknown_source_recovery_status", "selected_with_other_method",
    "record_profile_is_joined_profile", "exact_material_basis", "exact_without_recovery_method",
    "exact_pipe_material_without_selection_kind", "exact_physics_pipe_material", "record_extra_key",
)


@pytest.mark.parametrize("mutation", EVIDENCE_MUTATIONS)
def test_joined_evidence_mutations_are_rejected(mutation):
    evidence = _evidence()
    selected, ordinary = evidence["load_reference_states"]
    assert selected["source_recovery"]["status"] == "selected" and ordinary["source_recovery"]["status"] == "not_joined"
    exact = evidence["exact_cases"][0]
    if mutation == "extra_key": evidence["source_receipt"] = {}
    elif mutation == "missing_load_reference_states": del evidence["load_reference_states"]
    elif mutation == "missing_pressure": del evidence["pressure"]
    elif mutation == "connector_nonempty": evidence["connector"] = [{}]
    elif mutation == "selected_with_ordinary_method": selected["solve"]["recovery_method"] = "ordinary_sparse_structural_v1"
    elif mutation == "not_joined_with_retained_method": ordinary["solve"]["recovery_method"] = RETAINED
    elif mutation == "selected_without_method": del selected["source_recovery"]["method"]
    elif mutation == "selected_with_code": selected["source_recovery"]["code"] = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
    elif mutation == "unknown_source_recovery_status": selected["source_recovery"]["status"] = "joined"
    elif mutation == "selected_with_other_method": selected["source_recovery"]["method"] = "ordinary_sparse_structural_v1"
    elif mutation == "record_profile_is_joined_profile": selected["profile"] = LRS_PROFILE
    elif mutation == "exact_material_basis": exact["material_basis"] = "resolved_per_case_v1"
    elif mutation == "exact_without_recovery_method": del exact["recovery_method"]
    elif mutation == "exact_pipe_material_without_selection_kind": del exact["pipe_materials"][0]["material_selection_kind"]
    elif mutation == "exact_physics_pipe_material":
        for key in ("material_selection_kind", "resolved_eigenstrain"):
            del exact["pipe_materials"][0][key]
    elif mutation == "record_extra_key": ordinary["receipt"] = None
    invalid(definition("LoadReferenceSourceContractEvidence"), evidence, mutation)


def test_both_evidence_namespaces_are_disjoint():
    lr_evidence = strict_json(PROJECT / "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json")["contract_evidence"]
    valid(definition("LoadReferenceContractEvidence", LRSCHEMA), lr_evidence, "load-reference-1 evidence")
    invalid(definition("LoadReferenceSourceContractEvidence"), lr_evidence, "load-reference-1 evidence as joined")
    invalid(definition("LoadReferenceContractEvidence", LRSCHEMA), _evidence(), "joined evidence as load-reference-1")
    physical = strict_json(COMPOSITE / "mixed-sparse_interactive.raw.json")["contract_evidence"]
    invalid(definition("LoadReferenceSourceContractEvidence"), physical, "physics-source evidence as joined")
    invalid(definition("physical_evidence", PSR), _evidence(), "joined evidence as physics-source")


@pytest.mark.parametrize("policy", ("PHYSICS-SOURCE-1", "SOURCE-BLOCKS-1", "LOAD-REFERENCE-1", None))
def test_joined_receipt_policy_is_pinned(policy):
    receipt = _receipt()
    if policy is None:
        del receipt["body"]["policy"]
    else:
        receipt["body"]["policy"] = policy
    invalid(definition("LoadReferenceSourceRecovery"), receipt, f"policy {policy}")


def test_joined_and_physics_source_receipts_are_disjoint():
    receipt = _receipt()
    invalid(schema(PSR), receipt, "joined receipt as physics-source")
    invalid(schema("source_block_recovery.schema.json"), receipt, "joined receipt as source-blocks")
    physical = strict_json(COMPOSITE / "mixed-sparse_interactive.raw.json")["source_block_recovery"]
    valid(schema(PSR), physical, "physics-source receipt")
    invalid(definition("LoadReferenceSourceRecovery"), physical, "physics-source receipt as joined")
    receipt["body"]["extra"] = True
    invalid(definition("LoadReferenceSourceRecovery"), receipt, "receipt body extra key")


# ---------------------------------------------------------------- relabels and mixups per carrier
RESULTS_MUTATIONS = (
    "producer_id", "ref_id", "profile", "missing_receipt", "physics_source_receipt", "missing_evidence",
    "physics_source_evidence", "load_reference_evidence", "relabel_as_load_reference_1", "relabel_as_physics_source_1",
    "load_reference_1_relabelled_as_joined", "physics_source_1_relabelled_as_joined", "joined_profile_under_load_reference_1",
    "joined_profile_under_physics_source_1",
)


@pytest.mark.parametrize("mutation", RESULTS_MUTATIONS)
def test_results_joined_branch_refuses_relabels_and_mixups(mutation):
    source = joined()
    document = _results_document(source, LRS_ID)
    envelope = document["result_envelope"]
    physics_source = strict_json(COMPOSITE / "mixed-sparse_interactive.raw.json")
    load_reference = strict_json(PROJECT / "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json")

    def relabel(target, contract, profile):
        target["producer"]["semantic_contract_id"] = target["semantic_contract_ref"]["ref_id"] = contract
        target["formulation_basis"]["profile_id"] = profile

    if mutation == "producer_id": envelope["producer"]["semantic_contract_id"] = PS_ID
    elif mutation == "ref_id": envelope["semantic_contract_ref"]["ref_id"] = LR_ID
    elif mutation == "profile": envelope["formulation_basis"]["profile_id"] = LR_PROFILE
    elif mutation == "missing_receipt": del envelope["source_block_recovery"]
    elif mutation == "physics_source_receipt": envelope["source_block_recovery"] = physics_source["source_block_recovery"]
    elif mutation == "missing_evidence": del envelope["contract_evidence"]
    elif mutation == "physics_source_evidence": envelope["contract_evidence"] = physics_source["contract_evidence"]
    elif mutation == "load_reference_evidence": envelope["contract_evidence"] = load_reference["contract_evidence"]
    elif mutation == "relabel_as_load_reference_1": relabel(envelope, LR_ID, LR_PROFILE)
    elif mutation == "relabel_as_physics_source_1": relabel(envelope, PS_ID, PS_PROFILE)
    elif mutation == "load_reference_1_relabelled_as_joined":
        document = _results_document(load_reference, LR_ID)
        relabel(document["result_envelope"], LRS_ID, LRS_PROFILE)
    elif mutation == "physics_source_1_relabelled_as_joined":
        document = _results_document(physics_source, PS_ID)
        relabel(document["result_envelope"], LRS_ID, LRS_PROFILE)
    elif mutation == "joined_profile_under_load_reference_1":
        document = _results_document(load_reference, LR_ID)
        document["result_envelope"]["formulation_basis"]["profile_id"] = LRS_PROFILE
    elif mutation == "joined_profile_under_physics_source_1":
        document = _results_document(physics_source, PS_ID)
        document["result_envelope"]["formulation_basis"]["profile_id"] = LRS_PROFILE
    carrier_invalid(RESULTS, document, mutation)


RUN_MUTATIONS = (
    "load_reference_hash", "physics_source_hash", "joined_hash_under_physics_source_id", "unknown_id", "mixed_rows",
    "missing_receipt", "missing_evidence", "physics_source_receipt", "physics_source_evidence", "load_reference_evidence",
    "load_reference_1_relabelled_as_joined", "physics_source_1_relabelled_as_joined", "joined_relabelled_as_physics_source_1",
    "joined_relabelled_as_load_reference_1", "physics_source_1_carrying_joined_evidence", "reproducibility_hash_only",
)


@pytest.mark.parametrize("mutation", RUN_MUTATIONS)
def test_analysis_run_joined_branch_refuses_relabels_and_mixups(mutation):
    source = joined()
    base = lr_carrier("load_reference_connected_sparse", "analysis_run")
    physics_source_raw, physics_source_run, _ = captured(COMPOSITE / "mixed-sparse_interactive.raw.json")
    load_reference_raw = strict_json(PROJECT / "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json")
    record = joined_record(base, source)
    run = record["analysis_run"]
    if mutation == "load_reference_hash": record = joined_record(base, source, sha=LR_SHA)
    elif mutation == "physics_source_hash": record = joined_record(base, source, sha=PS_SHA)
    elif mutation == "joined_hash_under_physics_source_id": record = joined_record(base, source, contract_id=PS_ID)
    elif mutation == "unknown_id": record = joined_record(base, source, contract_id=LRS_ID.replace("-1", "-2"))
    elif mutation == "mixed_rows": record = joined_record(base, source, rows=(LR_ID, LR_SHA))
    elif mutation == "reproducibility_hash_only": run["reproducibility"]["semantic_contract"]["sha256"] = LR_SHA
    elif mutation == "missing_receipt": del run["source_block_recovery"]
    elif mutation == "missing_evidence": del run["contract_evidence"]
    elif mutation == "physics_source_receipt": run["source_block_recovery"] = physics_source_raw["source_block_recovery"]
    elif mutation == "physics_source_evidence": run["contract_evidence"] = physics_source_raw["contract_evidence"]
    elif mutation == "load_reference_evidence": run["contract_evidence"] = load_reference_raw["contract_evidence"]
    elif mutation == "load_reference_1_relabelled_as_joined":
        record = deepcopy(base)
        for item in [record["analysis_run"]["reproducibility"]] + record["analysis_run"]["result_refs"]:
            item["semantic_contract"].update(id=LRS_ID, sha256=LRS_SHA)
    elif mutation == "physics_source_1_relabelled_as_joined":
        record = deepcopy(physics_source_run)
        for item in [record["analysis_run"]["reproducibility"]] + record["analysis_run"]["result_refs"]:
            item["semantic_contract"].update(id=LRS_ID, sha256=LRS_SHA)
    elif mutation == "joined_relabelled_as_physics_source_1": record = joined_record(base, source, contract_id=PS_ID, sha=PS_SHA)
    elif mutation == "joined_relabelled_as_load_reference_1": record = joined_record(base, source, contract_id=LR_ID, sha=LR_SHA)
    elif mutation == "physics_source_1_carrying_joined_evidence":
        record = deepcopy(physics_source_run)
        record["analysis_run"]["contract_evidence"] = source["contract_evidence"]
    carrier_invalid(RUN, record, mutation)


@pytest.mark.parametrize("pair", ((LRS_ID, LR_SHA), (LRS_ID, PS_SHA), (LR_ID, LRS_SHA), (PS_ID, LRS_SHA)))
def test_semantic_contract_pairs_the_joined_id_only_with_its_table_hash(pair):
    valid(definition("SemanticContract", RUN), {"id": LRS_ID, "sha256": LRS_SHA}, "joined pair")
    invalid(definition("SemanticContract", RUN), {"id": pair[0], "sha256": pair[1]}, f"pair {pair}")


def test_analysis_run_contract_evidence_wrap_keeps_old_outcomes():
    """The one non-append pointer: every old outcome is unchanged, and joined evidence only fits the joined branch."""
    _, physics_source_run, _ = captured(COMPOSITE / "mixed-sparse_interactive.raw.json")
    carrier_valid(RUN, physics_source_run, PS_ID, "physics-source run")
    for stem in lr_stems():
        record = lr_carrier(stem, "analysis_run")
        record["analysis_run"]["contract_evidence"] = joined()["contract_evidence"]
        carrier_invalid(RUN, record, f"{stem} load-reference-1 run carrying evidence")
    physics = strict_json(RESULTS_FIXTURES / "physics_connected_mechanics_sparse.json")["contract_evidence"]
    record = deepcopy(physics_source_run)
    record["analysis_run"]["contract_evidence"] = physics
    carrier_invalid(RUN, record, "physics-1 evidence in a physics-source run")


PACKAGE_MUTATIONS = (
    "sha", "producer_id", "ref_id", "contract_id", "profile", "missing_receipt", "physics_source_receipt",
    "missing_evidence", "physics_source_evidence", "load_reference_evidence", "missing_source_annotations",
    "precision_csv_policy", "load_reference_1_relabelled_as_joined", "joined_relabelled_as_load_reference_1",
    "joined_relabelled_as_physics_source_1",
)


@pytest.mark.parametrize("mutation", PACKAGE_MUTATIONS)
def test_stress_neutral_joined_branch_refuses_relabels_and_mixups(mutation):
    source = joined()
    base = lr_carrier("load_reference_connected_sparse", "stress_neutral")
    physics_source = strict_json(COMPOSITE / "mixed-sparse_interactive.raw.json")
    load_reference = strict_json(PROJECT / "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json")
    packet = joined_packet(base, source)

    def relabel(target, contract, sha, profile):
        target["producer"]["semantic_contract_id"] = target["semantic_contract_ref"]["ref_id"] = contract
        target["semantic_contract"] = {"id": contract, "sha256": sha}
        target["formulation_basis"]["profile_id"] = profile

    if mutation == "sha": packet["semantic_contract"]["sha256"] = LR_SHA
    elif mutation == "producer_id": packet["producer"]["semantic_contract_id"] = LR_ID
    elif mutation == "ref_id": packet["semantic_contract_ref"]["ref_id"] = PS_ID
    elif mutation == "contract_id": packet["semantic_contract"]["id"] = LR_ID
    elif mutation == "profile": packet["formulation_basis"]["profile_id"] = PS_PROFILE
    elif mutation == "missing_receipt": del packet["source_block_recovery"]
    elif mutation == "physics_source_receipt": packet["source_block_recovery"] = physics_source["source_block_recovery"]
    elif mutation == "missing_evidence": del packet["contract_evidence"]
    elif mutation == "physics_source_evidence": packet["contract_evidence"] = physics_source["contract_evidence"]
    elif mutation == "load_reference_evidence": packet["contract_evidence"] = load_reference["contract_evidence"]
    elif mutation == "missing_source_annotations": del packet["source_annotations"]
    elif mutation == "precision_csv_policy":
        for key in ("csv_encoding", "csv_row_order"):
            del packet["export_profile"][key]
    elif mutation == "load_reference_1_relabelled_as_joined":
        packet = deepcopy(base)
        relabel(packet, LRS_ID, LRS_SHA, LRS_PROFILE)
    elif mutation == "joined_relabelled_as_load_reference_1": relabel(packet, LR_ID, LR_SHA, LR_PROFILE)
    elif mutation == "joined_relabelled_as_physics_source_1": relabel(packet, PS_ID, PS_SHA, PS_PROFILE)
    carrier_invalid(PACKAGE, packet, mutation)
