"""Additive load-reference-source-1 branches in the three v0.3 carrier schemas (T1_WP1_JOINED_SCHEMAS).

Run from WORKING_ROOT:  <python> <this file>

Each new branch is a deep copy of that file's own physics-source-1 branch with
only the semantic identity constants (ID, table sha256, profile) and the
receipt / contract_evidence $refs replaced. Enum arrays and oneOf/anyOf lists
are extended by appending. New $defs are added to results.v0.3 only; the
AnalysisRun and stress-neutral branches reference them there.

The one non-append change is AnalysisRun `properties/contract_evidence`: a
closed `$ref` to the physics-source physical evidence becomes an `anyOf` whose
first member is that same `$ref` (manager ruling (c), 2026-09-26). Every
document that validated before validates identically.

The script refuses to run twice (it stops if a file already names
load-reference-source-1). Files are rewritten as json.dumps(indent=2)+LF (the
byte-identical round trip is asserted before editing) through a temporary
file in the same directory followed by os.replace.
"""
from __future__ import annotations

import copy
import json
import os
import tempfile
from pathlib import Path

ROOT = Path.cwd()
PS_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
PS_SHA = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"
PS_PROFILE = "exact_straight_pressure_v2"
LRS_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
LRS_SHA = "d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337"
LRS_PROFILE = "resolved_straight_load_state_source_v1"
LRS_POLICY = "LOAD-REFERENCE-SOURCE-1"
RETAINED = "retained_source_blocks_exact_v1"
ORDINARY = ["ordinary_sparse_structural_v1", "ordinary_dense_structural_v1"]
PSR = "physics_source_recovery.schema.json"
LRSCHEMA = "load_reference_state.schema.json"
RESULTS = "results.v0.3.schema.yaml"

LOCAL_RECOVERY = {"$ref": "#/$defs/LoadReferenceSourceRecovery"}
LOCAL_EVIDENCE = {"$ref": "#/$defs/LoadReferenceSourceContractEvidence"}
REMOTE_RECOVERY = {"$ref": f"{RESULTS}#/$defs/LoadReferenceSourceRecovery"}
REMOTE_EVIDENCE = {"$ref": f"{RESULTS}#/$defs/LoadReferenceSourceContractEvidence"}


def load(name):
    path = ROOT / "schemas" / name
    text = path.read_text(encoding="utf-8")
    assert LRS_ID not in text and LRS_PROFILE not in text, f"{name} already carries load-reference-source-1"
    data = json.loads(text)
    assert json.dumps(data, indent=2) + "\n" == text, f"{name} does not round-trip"
    return path, data


def read(name):
    return json.loads((ROOT / "schemas" / name).read_text(encoding="utf-8"))


def write(path, data):
    text = json.dumps(data, indent=2) + "\n"
    json.loads(text)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".tmp", dir=path.parent)
    with os.fdopen(fd, "w", encoding="utf-8") as handle:
        handle.write(text)
    os.chmod(temporary, 0o644)
    os.replace(temporary, path)


def append(values, item):
    assert item not in values
    values.append(item)


def swap(node, replacements):
    """Replace exact constant string values; returns the count of replacements."""
    count = 0
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "const" and isinstance(value, str) and value in replacements:
                node[key] = replacements[value]
                count += 1
            else:
                count += swap(value, replacements)
    elif isinstance(node, list):
        for item in node:
            count += swap(item, replacements)
    return count


def rebase(node, target):
    """Rewrite local `#/$defs/X` refs of a copied definition to `<target>#/$defs/X`."""
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "$ref" and isinstance(value, str) and value.startswith("#/$defs/"):
                node[key] = target + value
            else:
                rebase(value, target)
    elif isinstance(node, list):
        for item in node:
            rebase(item, target)
    return node


def composite_branch(branches, locate):
    matches = [index for index, branch in enumerate(branches) if locate(branch) == PS_ID]
    assert matches == [3], matches
    assert len(branches) == 5, len(branches)
    return copy.deepcopy(branches[3])


# ----------------------------------------------------------------- new definitions (results.v0.3)
psr = read(PSR)
lrs = read(LRSCHEMA)

receipt_body = rebase(copy.deepcopy(psr["$defs"]["receipt_body"]), PSR)
assert receipt_body["properties"]["policy"] == {"const": "PHYSICS-SOURCE-1"}
receipt_body["properties"]["policy"] = {"const": LRS_POLICY}
receipt_body = {"description": (
    "Receipt body of openpipestress.result_semantics/0.3.0/load-reference-source-1: the physics-source-1 receipt body "
    "(physics_source_recovery.schema.json#/$defs/receipt_body) with policy LOAD-REFERENCE-SOURCE-1; every nested shape is "
    "reused by $ref. Closed shape only; hashes, bindings and the physical-evidence domain are executable reader validation."),
    **receipt_body}

recovery = {key: copy.deepcopy(value) for key, value in psr.items() if key in {"type", "additionalProperties", "required", "properties"}}
assert recovery["properties"]["body"] == {"$ref": "#/$defs/receipt_body"}
recovery["properties"]["body"] = {"$ref": "#/$defs/LoadReferenceSourceReceiptBody"}
recovery = {"description": (
    "Closed LOAD-REFERENCE-SOURCE-1 source_block_recovery receipt; the physics-source-1 receipt envelope with its own body."),
    **recovery}

exact = rebase(copy.deepcopy(psr["$defs"]["physical_evidence"]["properties"]["exact_cases"]["items"]), PSR)
assert exact["properties"]["material_basis"] == {"type": "string", "minLength": 1}
assert exact["properties"]["pipe_materials"]["items"] == {"$ref": f"{PSR}#/$defs/PhysicsMaterial"}
exact["properties"]["material_basis"] = {"const": "resolved_per_member_load_reference_state_v1"}
exact["properties"]["pipe_materials"]["items"] = {"$ref": f"{LRSCHEMA}#/$defs/LoadReferencePipeMaterial"}
exact = {"description": (
    "One contract_evidence.exact_cases[] item under load-reference-source-1: the physics-source-1 exact case "
    "(recovery_method and retained pipe_stress_extrema) with material_basis resolved_per_member_load_reference_state_v1 "
    "and resolved-member pipe_materials (load_reference_state.schema.json#/$defs/LoadReferencePipeMaterial)."),
    **exact}

record = rebase(copy.deepcopy(lrs["$defs"]["LoadReferenceStateRecord"]), f"{LRSCHEMA}")
record.pop("description")
solve = record["properties"]["solve"]
assert solve["properties"]["recovery_method"] == {"enum": ORDINARY}
assert len(solve["oneOf"]) == 2
append(solve["properties"]["recovery_method"]["enum"], RETAINED)
append(solve["oneOf"], {"properties": {"recovery_method": {"const": RETAINED}}})
not_joined = record["properties"]["source_recovery"]
assert not_joined["properties"] == {"status": {"const": "not_joined"}, "code": {"const": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}}
record["properties"]["source_recovery"] = {"oneOf": [not_joined, {
    "type": "object", "additionalProperties": False, "required": ["status", "method"],
    "properties": {"status": {"const": "selected"}, "method": {"const": RETAINED}}}]}
assert "oneOf" not in record
record["oneOf"] = [
    {"properties": {"solve": {"properties": {"recovery_method": {"const": RETAINED}}},
                    "source_recovery": {"properties": {"status": {"const": "selected"}}}}},
    {"properties": {"solve": {"properties": {"recovery_method": {"enum": list(ORDINARY)}}},
                    "source_recovery": {"properties": {"status": {"const": "not_joined"}}}}},
]
record = {"description": (
    "One contract_evidence.load_reference_states[] record under load-reference-source-1 (CP2_WIRE_ADDENDUM_2 section 5.3): "
    "the load-reference-1 record (load_reference_state.schema.json#/$defs/LoadReferenceStateRecord, nested shapes reused by $ref) "
    "except that a selected case has solve.recovery_method retained_source_blocks_exact_v1 with source_recovery "
    "{status: selected, method: retained_source_blocks_exact_v1}, and every other case keeps its ordinary method with "
    "{status: not_joined, code: LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED}. Case order, at least one selected case and the "
    "receipt binding are executable reader validation."),
    **record}

evidence = rebase(copy.deepcopy(lrs["$defs"]["LoadReferenceContractEvidence"]), f"{LRSCHEMA}")
assert evidence["properties"]["exact_cases"]["items"] == {"$ref": f"{LRSCHEMA}#/$defs/LoadReferenceExactCase"}
assert evidence["properties"]["load_reference_states"]["items"] == {"$ref": f"{LRSCHEMA}#/$defs/LoadReferenceStateRecord"}
evidence["properties"]["exact_cases"]["items"] = {"$ref": "#/$defs/LoadReferenceSourceExactCase"}
evidence["properties"]["load_reference_states"]["items"] = {"$ref": "#/$defs/LoadReferenceSourceStateRecord"}
evidence["description"] = (
    "Raw producer contract_evidence for semantic contract openpipestress.result_semantics/0.3.0/load-reference-source-1 "
    "(profile resolved_straight_load_state_source_v1): exactly {pressure, connector, exact_cases, load_reference_states}, "
    "connector []. Carriers preserve it verbatim. Closed shape only; cross-binding is executable reader validation.")

NEW_DEFS = {
    "LoadReferenceSourceRecovery": recovery,
    "LoadReferenceSourceReceiptBody": receipt_body,
    "LoadReferenceSourceContractEvidence": evidence,
    "LoadReferenceSourceExactCase": exact,
    "LoadReferenceSourceStateRecord": record,
}

# ----------------------------------------------------------------- results.v0.3
path, doc = load(RESULTS)
envelope = doc["$defs"]["ResultEnvelope"]
props = envelope["properties"]
append(props["producer"]["properties"]["semantic_contract_id"]["enum"], LRS_ID)
append(props["semantic_contract_ref"]["properties"]["ref_id"]["enum"], LRS_ID)
append(props["formulation_basis"]["properties"]["profile_id"]["enum"], LRS_PROFILE)
append(props["contract_evidence"]["anyOf"], LOCAL_EVIDENCE)
append(props["source_block_recovery"]["oneOf"], LOCAL_RECOVERY)
branch = composite_branch(envelope["oneOf"], lambda b: b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
assert swap(branch, {PS_ID: LRS_ID, PS_PROFILE: LRS_PROFILE}) == 3
assert branch["properties"]["source_block_recovery"] == {"$ref": PSR}
assert branch["properties"]["contract_evidence"] == {"$ref": f"{PSR}#/$defs/physical_evidence"}
branch["properties"]["source_block_recovery"] = copy.deepcopy(LOCAL_RECOVERY)
branch["properties"]["contract_evidence"] = copy.deepcopy(LOCAL_EVIDENCE)
envelope["oneOf"].append(branch)
for name, definition in NEW_DEFS.items():
    assert name not in doc["$defs"]
    doc["$defs"][name] = definition
write(path, doc)

# ----------------------------------------------------------------- analysis_run.v0.3
path, doc = load("analysis_run.v0.3.schema.json")
contract = doc["$defs"]["SemanticContract"]
append(contract["properties"]["id"]["enum"], LRS_ID)
append(contract["properties"]["sha256"]["enum"], LRS_SHA)
append(contract["oneOf"], {"properties": {"id": {"const": LRS_ID}, "sha256": {"const": LRS_SHA}}})
run = doc["$defs"]["AnalysisRun"]
append(run["properties"]["source_block_recovery"]["oneOf"], REMOTE_RECOVERY)
physical = {"$ref": f"{PSR}#/$defs/physical_evidence"}
assert run["properties"]["contract_evidence"] == physical
run["properties"]["contract_evidence"] = {"anyOf": [physical, copy.deepcopy(REMOTE_EVIDENCE)]}
branch = composite_branch(run["oneOf"], lambda b: b["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["id"]["const"])
assert swap(branch, {PS_ID: LRS_ID, PS_SHA: LRS_SHA}) == 4
assert branch["properties"]["source_block_recovery"] == {"$ref": PSR}
assert branch["properties"]["contract_evidence"] == physical
branch["properties"]["source_block_recovery"] = copy.deepcopy(REMOTE_RECOVERY)
branch["properties"]["contract_evidence"] = copy.deepcopy(REMOTE_EVIDENCE)
run["oneOf"].append(branch)
write(path, doc)

# ----------------------------------------------------------------- stress_neutral_export.v0.3
path, doc = load("stress_neutral_export.v0.3.schema.json")
props = doc["properties"]
append(props["producer"]["properties"]["semantic_contract_id"]["enum"], LRS_ID)
append(props["semantic_contract_ref"]["properties"]["ref_id"]["enum"], LRS_ID)
append(props["semantic_contract"]["properties"]["id"]["enum"], LRS_ID)
append(props["formulation_basis"]["properties"]["profile_id"]["enum"], LRS_PROFILE)
branch = composite_branch(doc["oneOf"], lambda b: b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
assert swap(branch, {PS_ID: LRS_ID, PS_SHA: LRS_SHA, PS_PROFILE: LRS_PROFILE}) == 5
assert branch["properties"]["source_block_recovery"] == {"$ref": PSR}
assert branch["properties"]["contract_evidence"] == {"$ref": f"{PSR}#/$defs/physical_evidence"}
branch["properties"]["source_block_recovery"] = copy.deepcopy(REMOTE_RECOVERY)
branch["properties"]["contract_evidence"] = copy.deepcopy(REMOTE_EVIDENCE)
doc["oneOf"].append(branch)
write(path, doc)
print("edited")
