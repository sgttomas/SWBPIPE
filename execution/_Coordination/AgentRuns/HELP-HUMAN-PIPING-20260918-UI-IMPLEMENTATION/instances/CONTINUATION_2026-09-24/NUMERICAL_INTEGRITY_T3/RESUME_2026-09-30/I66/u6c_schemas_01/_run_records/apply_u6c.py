#!/usr/bin/env python3
"""I66 U6c: the successor schema branches (C1:162; D2 4.9.6; RV78-S1) and
D-U6-2's two RowDisclosure codes admitted only in the successor branch.
Applied once to the candidate's schemas; each file keeps its exact JSON style
(indent 2; the ensure_ascii setting each file already round-trips with)."""
import json, sys
from copy import deepcopy
from pathlib import Path

P = Path(sys.argv[1])
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-1"
SUCC_SHA = "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8"
PREVIEW_SHA = "ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a"
PROFILE = "product_preview_retained_w1a_v2"
RECEIPT = {"$ref": "retained_precision_mp_v2.schema.json"}
CODES = ["retained_precision_absolute_verified", "retained_precision_not_covered"]


def load(name):
    path = P / "schemas" / name
    raw = path.read_bytes()
    data = json.loads(raw)
    for ascii_ in (True, False):
        if (json.dumps(data, indent=2, ensure_ascii=ascii_) + "\n").encode() == raw:
            return path, data, ascii_
    raise SystemExit(f"{name}: no exact JSON round trip")


def save(path, data, ascii_):
    path.write_bytes((json.dumps(data, indent=2, ensure_ascii=ascii_) + "\n").encode())


def forbid(branch, member):
    """Add `member` to the branch's not-anyOf list (creating it if absent)."""
    clause = {"required": [member]}
    if "not" in branch:
        assert list(branch["not"]) == ["anyOf"], branch["not"]
        assert clause not in branch["not"]["anyOf"]
        branch["not"]["anyOf"].append(clause)
    else:
        branch["not"] = {"anyOf": [clause]}


# results.v0.3: D-U6-2 codes in RowDisclosure, refused by every non-successor branch.
path, d, ascii_ = load("results.v0.3.schema.yaml")
enum = d["$defs"]["RowDisclosure"]["properties"]["reason_code"]["enum"]
assert not set(CODES) & set(enum)
enum.extend(CODES)
for branch in d["$defs"]["ResultEnvelope"]["oneOf"]:
    sid = branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"]
    if sid == SUCC:
        assert "allOf" not in branch
        continue
    assert branch["allOf"] == [{"not": {"required": ["retained_precision"]}}], sid
    branch["allOf"].append({"properties": {"row_disclosures": {"items": {"properties": {"reason_code": {"not": {"enum": CODES}}}}}}})
save(path, d, ascii_)

# analysis_run.v0.3: identity, receipt member and the successor branch.
path, d, ascii_ = load("analysis_run.v0.3.schema.json")
sc = d["$defs"]["SemanticContract"]
assert SUCC not in sc["properties"]["id"]["enum"]
sc["properties"]["id"]["enum"].append(SUCC)
sc["properties"]["sha256"]["enum"].append(SUCC_SHA)
sc["oneOf"].append({"properties": {"id": {"const": SUCC}, "sha256": {"const": SUCC_SHA}}})
run = d["$defs"]["AnalysisRun"]
assert "retained_precision" not in run["properties"]
run["properties"]["retained_precision"] = deepcopy(RECEIPT)
preview = None
for branch in run["oneOf"]:
    if branch["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"]["id"]["const"] == PREVIEW:
        preview = branch
    forbid(branch, "retained_precision")
succ = deepcopy(preview)
for part in (succ["properties"]["reproducibility"]["properties"]["semantic_contract"]["properties"],
             succ["properties"]["result_refs"]["items"]["properties"]["semantic_contract"]["properties"]):
    assert part["id"]["const"] == PREVIEW and part["sha256"]["const"] == PREVIEW_SHA
    part["id"]["const"], part["sha256"]["const"] = SUCC, SUCC_SHA
succ["not"]["anyOf"].remove({"required": ["retained_precision"]})
assert succ["not"] == {"anyOf": [{"required": ["source_block_recovery"]}, {"required": ["contract_evidence"]}]}
succ = {"required": ["retained_precision"], **succ}
run["oneOf"].append(succ)
save(path, d, ascii_)

# stress_neutral_export.v0.3: identity enums, receipt member and the successor branch.
path, d, ascii_ = load("stress_neutral_export.v0.3.schema.json")
props = d["properties"]
for enum in (props["producer"]["properties"]["semantic_contract_id"]["enum"],
             props["semantic_contract_ref"]["properties"]["ref_id"]["enum"],
             props["semantic_contract"]["properties"]["id"]["enum"]):
    assert PREVIEW in enum and SUCC not in enum
    enum.append(SUCC)
profiles = props["formulation_basis"]["properties"]["profile_id"]["enum"]
assert PROFILE not in profiles
profiles.append(PROFILE)
assert "retained_precision" not in props
props["retained_precision"] = deepcopy(RECEIPT)
preview = None
for branch in d["oneOf"]:
    if branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"] == PREVIEW:
        preview = branch
    forbid(branch, "retained_precision")
succ = deepcopy(preview)
pp = succ["properties"]
pp["producer"]["properties"]["semantic_contract_id"]["const"] = SUCC
pp["semantic_contract_ref"]["properties"]["ref_id"]["const"] = SUCC
assert pp["semantic_contract"]["properties"]["sha256"]["const"] == PREVIEW_SHA
pp["semantic_contract"]["properties"]["id"]["const"] = SUCC
pp["semantic_contract"]["properties"]["sha256"]["const"] = SUCC_SHA
assert pp["formulation_basis"]["properties"]["profile_id"]["const"] == "product_preview_mechanics_v1"
pp["formulation_basis"]["properties"]["profile_id"]["const"] = PROFILE
assert succ["required"] == ["contract_evidence", "source_annotations"]
succ["required"] = ["contract_evidence", "source_annotations", "retained_precision"]
succ["not"]["anyOf"].remove({"required": ["retained_precision"]})
assert succ["not"] == {"anyOf": [{"required": ["source_block_recovery"]}]}
d["oneOf"].append(succ)
save(path, d, ascii_)
print("applied")
