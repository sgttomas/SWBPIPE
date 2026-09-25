"""Additive load-reference-1 branches in the three v0.3 carrier schemas (CP3_SCHEMAS TASK).

Run from WORKING_ROOT:  <python> <this file>
Each new branch is a deep copy of that file's own physics-1 branch with only the
semantic identity constants (ID, table sha256, profile) and, where physics-1
carries it, the contract_evidence $ref replaced. Enum arrays and oneOf/anyOf
lists are extended by appending; nothing else changes. The script is
idempotent-refusing: it stops if a file already names load-reference-1.
Files are rewritten as json.dumps(indent=2)+LF (byte-identical round trip was
verified for all three before editing) through a temporary file in the same
directory followed by os.replace.
"""
from __future__ import annotations

import copy
import json
import os
import tempfile
from pathlib import Path

ROOT = Path.cwd()
PHYSICS_ID = "openpipestress.result_semantics/0.3.0/physics-1"
PHYSICS_SHA = "9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc"
PHYSICS_PROFILE = "exact_straight_pressure_v2"
LR_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
LR_SHA = "44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d"
LR_PROFILE = "resolved_straight_load_state_v1"
LR_EVIDENCE = {"$ref": "load_reference_state.schema.json#/$defs/LoadReferenceContractEvidence"}


def load(name):
    path = ROOT / "schemas" / name
    text = path.read_text(encoding="utf-8")
    assert LR_ID not in text and LR_PROFILE not in text, f"{name} already carries load-reference-1"
    data = json.loads(text)
    assert json.dumps(data, indent=2) + "\n" == text, f"{name} does not round-trip"
    return path, data


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
    """Replace exact constant values; returns the count of replacements."""
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


def physics_branch(branches, locate):
    matches = [index for index, branch in enumerate(branches) if locate(branch) == PHYSICS_ID]
    assert matches == [1], matches
    return copy.deepcopy(branches[1])


# results.v0.3 ---------------------------------------------------------------
path, doc = load("results.v0.3.schema.yaml")
envelope = doc["$defs"]["ResultEnvelope"]
props = envelope["properties"]
append(props["producer"]["properties"]["semantic_contract_id"]["enum"], LR_ID)
append(props["semantic_contract_ref"]["properties"]["ref_id"]["enum"], LR_ID)
append(props["formulation_basis"]["properties"]["profile_id"]["enum"], LR_PROFILE)
append(props["contract_evidence"]["anyOf"], LR_EVIDENCE)
branch = physics_branch(envelope["oneOf"], lambda b: b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
assert swap(branch, {PHYSICS_ID: LR_ID, PHYSICS_PROFILE: LR_PROFILE}) == 3
assert branch["properties"]["contract_evidence"] == {"$ref": "#/$defs/PhysicsContractEvidence"}
branch["properties"]["contract_evidence"] = copy.deepcopy(LR_EVIDENCE)
assert branch["not"] == {"anyOf": [{"required": ["source_block_recovery"]}]}
envelope["oneOf"].append(branch)
write(path, doc)

# analysis_run.v0.3 ----------------------------------------------------------
path, doc = load("analysis_run.v0.3.schema.json")
contract = doc["$defs"]["SemanticContract"]
append(contract["properties"]["id"]["enum"], LR_ID)
append(contract["properties"]["sha256"]["enum"], LR_SHA)
pair = copy.deepcopy(next(b for b in contract["oneOf"] if b["properties"]["id"]["const"] == PHYSICS_ID))
assert swap(pair, {PHYSICS_ID: LR_ID, PHYSICS_SHA: LR_SHA}) == 2
contract["oneOf"].append(pair)
run = doc["$defs"]["AnalysisRun"]
branch = physics_branch(run["oneOf"], lambda b: b["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["id"]["const"])
assert swap(branch, {PHYSICS_ID: LR_ID, PHYSICS_SHA: LR_SHA}) == 4
# physics-1 AnalysisRun records carry neither namespace; load-reference-1 mirrors it.
assert branch["not"] == {"anyOf": [{"required": ["source_block_recovery"]}, {"required": ["contract_evidence"]}]}
assert "contract_evidence" not in branch["properties"] and "required" not in branch
run["oneOf"].append(branch)
write(path, doc)

# stress_neutral_export.v0.3 -------------------------------------------------
path, doc = load("stress_neutral_export.v0.3.schema.json")
props = doc["properties"]
append(props["producer"]["properties"]["semantic_contract_id"]["enum"], LR_ID)
append(props["formulation_basis"]["properties"]["profile_id"]["enum"], LR_PROFILE)
append(props["semantic_contract_ref"]["properties"]["ref_id"]["enum"], LR_ID)
append(props["semantic_contract"]["properties"]["id"]["enum"], LR_ID)
branch = physics_branch(doc["oneOf"], lambda b: b["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
assert swap(branch, {PHYSICS_ID: LR_ID, PHYSICS_SHA: LR_SHA, PHYSICS_PROFILE: LR_PROFILE}) == 5
assert branch["properties"]["contract_evidence"] == {"$ref": "#/$defs/PhysicsContractEvidence"}
branch["properties"]["contract_evidence"] = copy.deepcopy(LR_EVIDENCE)
assert branch["required"] == ["contract_evidence", "source_annotations"]
assert branch["not"] == {"anyOf": [{"required": ["source_block_recovery"]}]}
doc["oneOf"].append(branch)
write(path, doc)
print("edited 3 carriers")
