"""I97 B2-C: build B2's draft statics for J1 (records only; standard library only).

Reads committed bytes only:
  <P>/fixtures/results/retained_precision_prepared_ordinary_v1.json            DEF-O (unchanged)
  <P>/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json  PTABLE (revised here)
  <P>/fixtures/results/semantic_contract_v0_3_preview_physics_1.json           PTABLE's inherited table (control)
  <P>/schemas/retained_precision_mp_v2.schema.json                             SCHEMA (main's)
  <I96>/_run_records/b3d_statics.py, <I96>/_run_records/revision_01/b3d_statics_r1.py   B3b's generators (imported unchanged)
  <I96>/statics/r1/semantic_contract_v0_3_physics_retained_1.json              XTABLE (receipt_bindings reference)

Writes into <out_dir>:
  retained_precision_prepared_combination_v1.json            DEF-C, the combination formation definition (draft)
  semantic_contract_v0_3_preview_physics_retained_1.json     PTABLE's one revision (draft)
  SCHEMA_B2.diff                                             B2's whole SCHEMA change against main
  retained_precision_mp_v2.schema.json                       the merged J1 SCHEMA text (B2 + B3b's SCHEMA_ENUM.diff, NA-1)
  SCHEMA_J1.diff                                             the merged J1 text against main
  b2c_statics.out.json                                       hashes, controls and member-level deltas

Usage: python b2c_statics.py <P> <I96 record folder> <out_dir>
"""
import copy
import difflib
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

ORDINARY_ID = "RP-PREPARED-ORDINARY-DUAL-v1"
COMBINATION_ID = "RP-PREPARED-COMBINATION-DUAL-v1"
EXACT_ID = "RP-PREPARED-EXACT-DUAL-v1"
FORMATION_DOMAIN = "retained_precision_formation_v1"
METHOD = "contribution_preserving_multiprecision_v1"
GATE_CODES = ["NONLINEAR_COMBINATION_REQUIRES_SOLVE", "CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE",
              "COMBINATION_MODULUS_BASIS_MIXED"]
RECEIPT_BINDINGS = {
    "canonicalization": "openpipestress_jcs_ijson_v1",
    "method": METHOD,
    "projection_policy": "RP-LOGICAL-ATTEMPTS-v1",
    "work": {"case_limit": 20000000000, "invocation_limit": 60000000000},
    "work_policy": "W1-LME-20B-60B-v1",
}
ORIGINAL_TITLE = "Standalone ordinary prepared C1/C2/C3 receipt"
ORIGINAL_PREFIX = ("Ordinary-prepared scope only; exact and prepared-combination admission require their later "
                   "selected extension.")
B2_TITLE = "Standalone prepared C1/C2/C3 receipt"
B2_PREFIX = ("Prepared ordinary case formation (ProductAttempt) and the prepared combination formation "
             "(CombinationAttempt, RP-PREPARED-COMBINATION-DUAL-v1) with its operand preparations "
             "(OperandPreparation, ordinary only); combination dispositions, operand sources and row coverage are "
             "reader gates (G3, G5, G8), not schema branches; exact admission requires its later selected extension.")
J1_PREFIX = ("Prepared ordinary and exact case formations, selected by ProductAttempt.definition_id, and the prepared "
             "combination formation (CombinationAttempt, RP-PREPARED-COMBINATION-DUAL-v1) with its operand "
             "preparations (OperandPreparation, ordinary only); the exact route's constraints are reader gates (G0, "
             "G5b, G8), and combination dispositions, operand sources and row coverage are reader gates (G3, G5, G8), "
             "not schema branches.")


def sha(b):
    return hashlib.sha256(b).hexdigest()


def canonical(v):
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def h(domain, payload):
    return sha(canonical({"domain": domain, "payload": payload}))


def pretty(v):
    return (json.dumps(v, indent=2, ensure_ascii=True) + "\n").encode()


def floats(v):
    if isinstance(v, float):
        return 1
    if isinstance(v, dict):
        return sum(floats(x) for x in v.values())
    if isinstance(v, list):
        return sum(floats(x) for x in v)
    return 0


# ---------------------------------------------------------------------------------------------
# DEF-C: the combination formation definition (CONTRACT.md section 3).
# ---------------------------------------------------------------------------------------------
def combination_definition(defo):
    d = copy.deepcopy(defo)
    d["id"] = COMBINATION_ID
    d["version"] = 1
    d["inherits"]["operand_definition"] = {"id": ORDINARY_ID, "sha256": h(FORMATION_DOMAIN, defo)}
    d["acceptance"]["composition"] = (
        "no operand radius, published row, retained state, verification or certificate enters the proof; the "
        "combination's rows are its own solve on the prepared stiffness and are not the factor-weighted sum of its "
        "operands' published rows; operand cases keep their values, evidence, cache snapshots and standing")
    d["hash_domains"] = {
        "definition": FORMATION_DOMAIN,
        "publication": defo["hash_domains"]["publication"],
        "receipt": defo["hash_domains"]["receipt"],
        "source": defo["hash_domains"]["source"],
    }
    d["lanes"]["admitted_k"] = (
        "exact admitted products E*A,G*J,E*Iz,E*Iy of the representative source's members and exactly zero "
        "coefficient differences; actual common frame and support law; loads per lanes.loads")
    d["lanes"]["loads"] = (
        "each loaded DOF's exact combined net N_g (the owner's K4LED: every operand term's exact product c_i*v_ij, "
        "summed exactly per DOF) enclosed outward at 1024 bits as [RD1024(N_g),RU1024(N_g)], in free residual rows "
        "and constrained reaction offsets; prescribed coordinates exact zero; never an operand's individual terms, "
        "a binary64 product or a rounded net")
    d["lanes"]["owner"] = (
        "the combination's recorded selected run (NativeOwner::Combination), its K4CMB source and combined ledger, "
        "its cache (imports from selected operands plus its own builds) and one nonreused proof anchor")
    d["preparation"] = {
        "combination": "not_entered",
        "operands": (
            "every operand source is a prepared source of operand_definition: a selected or unavailable operand "
            "through its case ProductAttempt's preparation (retained_precision_preparation_v1), a not_required "
            "operand through one OperandPreparation (retained_precision_operand_preparation_v1); "
            "RP-PREPARED-ANNULUS-v1 unchanged"),
    }
    d["rows"]["ancillary"] = "none: a mechanics combination publishes no mode, parity or modulus record"
    d["rows"]["coverage"] = (
        "exactly the combination's own final rows: 6n displacement components (constrained ones input-derived "
        "+0), n displacement magnitudes, 12m end actions and 18m station actions, 20m signed stresses, 6g signed "
        "support components and 2g support magnitudes over its g attributed supports (7n+50m+8g); derivative "
        "slots 0-19 of every member required and slot 20 empty; no record required; a missing, duplicated or extra "
        "row fails the whole proof")
    d["rows"]["maximum"] = (
        "not published: no combination maximum (COMBINATION_STRESS_MAXIMUM_UNAVAILABLE) and no intensified row")
    d["rows"]["prescribed"] = (
        "exact zero prescription of every operand term, so the combined prescription is exactly +0; prescribed "
        "classification, raw/SI equality")
    d["scope"] = {
        "entry": "captured_actual_request_and_mode",
        "excludes": [x for x in defo["scope"]["excludes"] if x != "prepared_combinations"] + [
            "result_state_subtraction", "range_envelope", "nested_combinations", "repeated_operand_cases",
            "combinations_without_selected_operand", "combination_maxima", "intensified_rows",
            "nonzero_operand_prescriptions"],
        "loads": "combined_exact_ledger",
        "materials": copy.deepcopy(defo["scope"]["materials"]),
        "operand_equality": (
            "equal K4STF bytes, layout, stations and supports (kernel OperandsDiffer) and equal prepared section "
            "facts across all operands and the combination: material basis, normalized D, effective wall and the "
            "prepared A,I,J,Z,c bits of every member, as the operands' preparations and CaseSource section_terms "
            "state them"),
        "operand_sources": (
            "selected operand: its selected solve (cache imports allowed); unavailable operand with a CaseSource: "
            "that registered source, rebuilt and K4SRC-identity-checked (no import); not_required operand: the one "
            "registered OperandPreparation source shared by every combination that requests it (no import); no "
            "operand is solved, mutated or re-prepared for a combination"),
        "owners": "mechanics_combination",
        "prescriptions": "exact_positive_zero",
        "requires": defo["scope"]["requires"],
        "scope_limit": defo["scope"]["scope_limit"],
        "source": defo["scope"]["source"],
        "supports": defo["scope"]["supports"],
        "trigger": (
            "a mechanics combination that the T0R gates admit, whose terms name distinct load cases, at least one "
            "of them selected after T-9; its group is the first selected operand's and its representative source "
            "is operand 0's"),
    }
    d["stages"] = {
        "entered": ["native", "proof_start", "projection", "maxima", "values", "aliases", "certificate",
                    "observables", "g5a"],
        "not_entered": ["preparation"],
        "maxima": "completes with no maximum row",
        "aliases": "completes with no headline alias: headlines cover load cases only",
    }
    d["trust"]["G8"] = (
        "independent invocation/model/material/normalized-input/map rederivation; the combination expression "
        "(case ids and factor bits in authored order) equal to the invocation's; each operand CaseSource "
        "rederived as a case's; K4CMB recomputed from the operands' K4SRC bytes and factor bits and bound to "
        "kernel_source_sha256 with one common K4STF; scope.operand_equality; checked source/preparation "
        "associations; the combined ledger bound by ledger_sha256 as attestation; no historical physics-source "
        "stress recipe")
    d["work"]["additional_product"] = (
        "entered lanes/projection/conversion/comparison/adapter/maximum-completion work separately retained with "
        "actual status and failed prefixes; combined-net enclosures counted as add entries; no preparation; no "
        "all-in LME total or new price")
    d["work"]["native_meter"] = (
        "C1/C2 physical/logical partition and recorded native debit unchanged; the combination's own Run on the "
        "invocation's one meter under its own 20B case limit")
    return d


# ---------------------------------------------------------------------------------------------
# PTABLE's one revision (CONTRACT.md section 7).
# ---------------------------------------------------------------------------------------------
def ptable_revision(ptable_raw, defo_h, defc_h, xtable):
    t = json.loads(ptable_raw)
    assert t["product_formation_definitions"] == [{"id": ORDINARY_ID, "sha256": defo_h}]
    assert t["accuracy_classification"]["scope"] == (
        "registered ordinary-prepared source only; no exact-profile or prepared-combination extension")
    assert "receipt_bindings" not in t
    assert xtable["receipt_bindings"] == RECEIPT_BINDINGS
    t["accuracy_classification"]["scope"] = (
        "registered ordinary-prepared source, and registered prepared mechanics combinations "
        "(RP-PREPARED-COMBINATION-DUAL-v1: their own solve on the combined exact ledger, at least one operand "
        "selected); no exact-profile, subtraction or range extension")
    t["product_formation_definitions"] = [{"id": ORDINARY_ID, "sha256": defo_h},
                                          {"id": COMBINATION_ID, "sha256": defc_h}]
    warrant = t["formation_warrant"]
    assert warrant["definition_id"] == ORDINARY_ID
    combination_warrant = copy.deepcopy(warrant)
    combination_warrant["definition_id"] = COMBINATION_ID
    t["formation_warrant"] = [warrant, combination_warrant]
    t["receipt_bindings"] = copy.deepcopy(xtable["receipt_bindings"])
    return pretty(t)


# ---------------------------------------------------------------------------------------------
# SCHEMA: B2's $defs (CONTRACT.md section 6).
# ---------------------------------------------------------------------------------------------
def obj(properties, required=None):
    return {"type": "object", "properties": properties,
            "required": list(properties) if required is None else required, "additionalProperties": False}


def ref(name):
    return {"$ref": f"#/$defs/{name}"}


def arr(items, **bounds):
    a = {"type": "array", "items": items}
    a.update(bounds)
    return a


def null_or(spec):
    return {"oneOf": [{"type": "null"}, spec]}


SLOT = {"enum": ["s128", "s256", "s512", "s1024", "v256", "v512", "v1024"]}


def b2_schema(s):
    """B2's whole change to a SCHEMA value (title and $comment excluded; see apply_b2)."""
    D = s["$defs"]
    pa = D["ProductAttempt"]["properties"]
    # Closed copies of the existing ProductAttempt members a combination attempt shares.
    result = copy.deepcopy(pa["result"])
    adapter = copy.deepcopy(pa["adapter"])
    operational = copy.deepcopy(pa["operational"])
    stage = copy.deepcopy(pa["stages"]["properties"]["native"])
    preparation_error = copy.deepcopy(D["PublicError"]["oneOf"][0])
    assert preparation_error["properties"]["kind"] == {"const": "preparation"}
    group = D["Group"]
    basis_case = D["Case"]["oneOf"][2]["properties"]["reason"]
    assert set(basis_case["properties"]) == {"code", "phase", "cause"}

    new = {}
    new["CombinationExpression"] = {"oneOf": [
        obj({"kind": {"const": "mechanics"},
             "terms": arr(obj({"case_id": ref("Text"), "factor": ref("Bits")}), minItems=1)}),
        obj({"kind": {"const": "result_state_subtraction"}, "minuend_id": ref("Text"),
             "subtrahend_id": ref("Text")}),
        obj({"kind": {"const": "range_envelope"}, "operand_ids": arr(ref("Text"), minItems=1),
             "mode": {"enum": ["min", "max", "min_abs", "max_abs"]}}),
    ]}
    common = {
        "basis_ref": obj({"ref_type": {"const": "combination"}, "ref_id": ref("Text")}),
        "expression": ref("CombinationExpression"),
        "result_ids": arr(ref("Text")),
        "diagnostic_refs": arr(ref("Text")),
    }

    def branch(disposition, extra, result_ids=None):
        p = copy.deepcopy(common)
        p["disposition"] = {"const": disposition}
        if result_ids is not None:
            p["result_ids"] = result_ids
        p.update(extra)
        return obj(p)

    new["Combination"] = {"oneOf": [
        branch("retained_selected", {
            "method": {"const": METHOD},
            "call_ref": ref("U"),
            "product_attempt_ref": ref("U"),
            "run": ref("Run"),
            "source_ref": ref("U"),
            "source_identity_sha256": ref("Hash"),
            "selection": ref("Selection"),
        }),
        branch("retained_unavailable", {
            "call_ref": null_or(ref("U")),
            "product_attempt_ref": null_or(ref("U")),
            "reason": copy.deepcopy(basis_case),
            "diagnostic_ref": ref("Text"),
            "run": null_or(ref("Run")),
            "source_ref": null_or(ref("U")),
        }),
        branch("ordinary", {"reason": {"const": "no_retained_mechanics"}}),
        branch("base_withheld", {"reason": {"enum": list(GATE_CODES)}}, result_ids=arr(ref("Text"), maxItems=0)),
    ]}
    new["CombinationReason"] = {"oneOf": [
        obj({"space": {"const": "combination"},
             "tag": {"enum": ["no_operands", "nested_combination", "operands_differ", "no_selected_operand"]}}),
        obj({"space": {"const": "combination"}, "tag": {"const": "ledger_unavailable"}, "error": ref("LedgerError")}),
        obj({"space": {"const": "combination"}, "tag": {"const": "count_range"}, "name": ref("Text")}),
    ]}
    new["CombinationSource"] = obj({
        "index": ref("U"),
        "owner": obj({"kind": {"const": "combination"}, "combination_index": ref("U"), "combination_id": ref("Text")}),
        "kernel_source_sha256": ref("Hash"),
        "ledger_sha256": ref("Hash"),
        "stiffness_sha256": ref("Hash"),
        "representative_source_ref": ref("U"),
        "operands": arr(obj({"case_index": ref("U"), "factor": ref("Bits"), "source_ref": ref("U"),
                             "source_identity_sha256": ref("Hash")}), minItems=1),
    })
    new["MechanicsCombinationCall"] = obj({
        "id": ref("U"),
        "kind": {"const": "mechanics_combination"},
        "owner_refs": arr(obj({"kind": {"const": "combination"}, "index": ref("U")}), minItems=1, maxItems=1),
        "requested_operands": arr(obj({"source_ref": ref("U"), "factor": ref("Bits")})),
        "source_refs": arr(ref("U"), maxItems=1),
        "run_refs": arr(ref("U"), maxItems=1),
        "invocation_before": ref("U"),
        "invocation_after": ref("U"),
        "result": {"oneOf": [
            obj({"kind": {"const": "runs"}}),
            obj({"kind": {"const": "pre_source_refusal"},
                 "stage": {"enum": ["operand_validation", "combined_preparation"]},
                 "reason": ref("CombinationReason")}),
        ]},
    })
    cg = copy.deepcopy(group)
    cg["properties"]["imports"] = arr(obj({"operand_index": ref("U"), "selected_run": ref("U"), "slot": SLOT,
                                            "build": ref("U")}))
    cg["required"] = list(cg["required"]) + ["imports"]
    new["CombinationGroup"] = cg
    new["CombinationAttempt"] = obj({
        "id": ref("U"),
        "definition_id": {"const": COMBINATION_ID},
        "owner_ref": obj({"kind": {"const": "combination"}, "index": ref("U")}),
        "material_basis_ref": ref("U"),
        "source_ref": ref("U"),
        "run_ref": ref("U"),
        "result": result,
        "stages": obj({k: copy.deepcopy(stage) for k in
                       ["native", "proof_start", "projection", "maxima", "values", "aliases", "certificate",
                        "observables", "g5a"]}),
        "proof": null_or(ref("ProofTrace")),
        "adapter": adapter,
        "overlay_work": ref("ScalarTrace"),
        "g5a_work": ref("ScalarTrace"),
    })
    new["OperandPreparation"] = obj({
        "id": ref("U"),
        "definition_id": {"const": ORDINARY_ID},
        "owner_ref": obj({"kind": {"const": "case"}, "index": ref("U")}),
        "ordinary_attempt_ref": ref("U"),
        "material_basis_ref": ref("U"),
        "purpose": {"const": "combination_operand"},
        "requested_by": arr(ref("U"), minItems=1),
        "source_ref": null_or(ref("U")),
        "result": {"oneOf": [obj({"kind": {"const": "prepared"}}),
                             obj({"kind": {"const": "refused"}, "error": preparation_error})]},
        "stage": {"enum": ["completed", "failed"]},
        "preparation": obj({"members": arr(ref("PreparedMember"))}),
        "adapter": copy.deepcopy(adapter),
        "operational": operational,
    })
    for name, spec in new.items():
        assert name not in D, name
        D[name] = spec

    # Existing $defs: additive branches only.
    prep = D["CaseSource"]["properties"]["preparation"]["oneOf"]
    assert len(prep) == 2
    prep.append(obj({"operand_preparation_ref": ref("U"), "sha256": ref("Hash")}))
    cause = D["UnavailableCause"]["oneOf"]
    assert cause[-1] == ref("Reason")
    cause[-1:-1] = [obj({"kind": {"const": "operand_preparation_failure"}, "operand_preparation_ref": ref("U")}),
                    obj({"kind": {"const": "operand_source_unavailable"}, "operand_index": ref("U")})]
    cause.append(ref("CombinationReason"))

    body = D["Body"]["properties"]
    assert body["combinations"] == {"type": "array", "items": {}, "minItems": 0, "maxItems": 0}
    body["combinations"] = arr(ref("Combination"))
    assert body["sources"]["items"] == ref("CaseSource")
    body["sources"]["items"] = {"oneOf": [ref("CaseSource"), ref("CombinationSource")]}
    assert body["calls"]["items"] == ref("Call")
    body["calls"]["items"] = {"oneOf": [ref("Call"), ref("MechanicsCombinationCall")]}
    assert body["groups"]["items"] == ref("Group")
    body["groups"]["items"] = {"oneOf": [ref("Group"), ref("CombinationGroup")]}
    assert body["product_attempts"]["items"] == ref("ProductAttempt")
    body["product_attempts"]["items"] = {"oneOf": [ref("ProductAttempt"), ref("CombinationAttempt")]}
    # C3a's member, present only when non-empty (CONTRACT.md decision C-6): B1's bytes keep no new member.
    assert "operand_preparations" not in body
    new_body = {}
    for k, v in body.items():
        new_body[k] = v
        if k == "product_attempts":
            new_body["operand_preparations"] = arr(ref("OperandPreparation"), minItems=1)
    D["Body"]["properties"] = new_body
    assert "operand_preparations" not in D["Body"]["required"]
    return s


def apply_b2(main):
    s = b2_schema(copy.deepcopy(main))
    assert s["title"] == ORIGINAL_TITLE and s["$comment"].startswith(ORIGINAL_PREFIX)
    s["title"] = B2_TITLE
    s["$comment"] = B2_PREFIX + s["$comment"][len(ORIGINAL_PREFIX):]
    return s


def apply_j1(main, schema_r1):
    """B3b's SCHEMA_ENUM.diff first (I96's own function, unchanged), then B2's, then one coherent text."""
    b3b = schema_r1(pretty(main))
    s = b2_schema(copy.deepcopy(b3b))
    assert s["title"] == B2_TITLE  # B3b's title, which B2 also uses
    b3b_prefix = ("Prepared ordinary and exact formations, selected by ProductAttempt.definition_id; the exact "
                  "route's constraints are reader gates (G0, G5b, G8), not schema branches; prepared-combination "
                  "admission requires its later selected extension.")
    assert s["$comment"].startswith(b3b_prefix)
    s["$comment"] = J1_PREFIX + s["$comment"][len(b3b_prefix):]
    return s, b3b


def udiff(a, b, name):
    return "".join(difflib.unified_diff(a.decode().splitlines(True), b.decode().splitlines(True),
                                        f"a/P/{name}", f"b/P/{name}", n=3))


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def main():
    p, i96, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    res = p / "fixtures/results"
    v0_path = i96 / "_run_records/b3d_statics.py"
    r1_path = i96 / "_run_records/revision_01/b3d_statics_r1.py"
    load(v0_path, "b3d_statics")  # r1 imports it by this name
    r1 = load(r1_path, "b3d_statics_r1")
    xtable_raw = (i96 / "statics/r1/semantic_contract_v0_3_physics_retained_1.json").read_bytes()
    xtable = json.loads(xtable_raw)

    defo_raw = (res / "retained_precision_prepared_ordinary_v1.json").read_bytes()
    defo = json.loads(defo_raw)
    ptable_raw = (res / "semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes()
    pp1_raw = (res / "semantic_contract_v0_3_preview_physics_1.json").read_bytes()
    schema_raw = (p / "schemas/retained_precision_mp_v2.schema.json").read_bytes()
    main_schema = json.loads(schema_raw)

    report = {"inputs": {
        "DEF-O": sha(defo_raw), "PTABLE": sha(ptable_raw), "preview-physics-1 table": sha(pp1_raw),
        "SCHEMA": sha(schema_raw), "XTABLE (I96 r1)": sha(xtable_raw),
        "b3d_statics.py (I96)": sha(v0_path.read_bytes()), "b3d_statics_r1.py (I96)": sha(r1_path.read_bytes()),
    }}
    defo_h = h(FORMATION_DOMAIN, defo)
    # Controls: DEF-O's canonical form and pinned H; PTABLE's and SCHEMA's serialization conventions.
    pt = json.loads(ptable_raw)
    rebuilt = json.loads(pp1_raw)
    rebuilt["semantic_contract_id"] = pt["semantic_contract_id"]
    rebuilt["inherited_semantic_contract_sha256"] = sha(pp1_raw)
    rebuilt["formulation_profile_id"] = pt["formulation_profile_id"]
    for k in ["receipt_policy", "receipt_schema", "product_formation_definitions", "accuracy_classification",
              "formation_warrant"]:
        rebuilt[k] = pt[k]
    report["controls"] = {
        "defo_canonical_bytes": canonical(defo) == defo_raw,
        "defo_H": defo_h,
        "ptable_roundtrip_indent2_ascii": pretty(pt) == ptable_raw,
        "ptable_method_reproduces_from_preview_physics_1": pretty(rebuilt) == ptable_raw,
        "schema_roundtrip_indent2_ascii": pretty(main_schema) == schema_raw,
        "b3b_only_schema_sha256_reproduces_I96": sha(pretty(r1.schema_r1(schema_raw))),
        "xtable_receipt_bindings": xtable["receipt_bindings"],
    }

    # DEF-C
    defc = combination_definition(defo)
    defc_raw = canonical(defc)
    assert all(b < 128 for b in defc_raw) and floats(defc) == 0
    (out / "retained_precision_prepared_combination_v1.json").write_bytes(defc_raw)
    defc_h = h(FORMATION_DOMAIN, defc)
    changed = []
    for k in sorted(set(defc) | set(defo)):
        a, b = defo.get(k), defc.get(k)
        if isinstance(a, dict) and isinstance(b, dict):
            changed += [f"{k}.{kk}" for kk in sorted(set(a) | set(b)) if a.get(kk) != b.get(kk)]
        elif a != b:
            changed.append(k)
    report["definition"] = {
        "file": "retained_precision_prepared_combination_v1.json", "bytes": len(defc_raw),
        "raw_sha256": sha(defc_raw), "H_retained_precision_formation_v1": defc_h,
        "H_alternative_domain_retained_precision_combination_formation_v1": h(
            "retained_precision_combination_formation_v1", defc),
        "ascii_only": True, "json_floats": 0, "paths_changed_from_DEF_O": changed,
        "unchanged_from_DEF_O": sorted(k for k in defo if defo[k] == defc.get(k)),
    }

    # PTABLE's revision
    table = ptable_revision(ptable_raw, defo_h, defc_h, xtable)
    (out / "semantic_contract_v0_3_preview_physics_retained_1.json").write_bytes(table)
    tj = json.loads(table)
    report["table"] = {
        "file": "semantic_contract_v0_3_preview_physics_retained_1.json", "bytes": len(table), "sha256": sha(table),
        "members_changed": [k for k in pt if tj[k] != pt[k]],
        "members_added": [k for k in tj if k not in pt],
        "accuracy_classification_keys_changed": [k for k in pt["accuracy_classification"]
                                                 if tj["accuracy_classification"][k] != pt["accuracy_classification"][k]],
        "receipt_bindings_canonical_equal_to_XTABLE": canonical(tj["receipt_bindings"]) == canonical(
            xtable["receipt_bindings"]),
        "receipt_bindings_text_equal_to_XTABLE": json.dumps(tj["receipt_bindings"], indent=2) == json.dumps(
            xtable["receipt_bindings"], indent=2),
        "product_formation_definitions": tj["product_formation_definitions"],
        "roundtrip_indent2_ascii": pretty(tj) == table,
    }

    # SCHEMA: B2 alone, then the merged J1 text.
    b2 = apply_b2(main_schema)
    b2_raw = pretty(b2)
    (out / "SCHEMA_B2.diff").write_text(udiff(schema_raw, b2_raw, "schemas/retained_precision_mp_v2.schema.json"))
    j1, b3b = apply_j1(main_schema, r1.schema_r1)
    j1_raw = pretty(j1)
    (out / "retained_precision_mp_v2.schema.json").write_bytes(j1_raw)
    (out / "SCHEMA_J1.diff").write_text(udiff(schema_raw, j1_raw, "schemas/retained_precision_mp_v2.schema.json"))
    # Composition checks: J1 = B2 with B3b's definition_id enum, and = B3b with B2's $defs, apart from $comment.
    b2_with_enum = copy.deepcopy(b2)
    b2_with_enum["$defs"]["ProductAttempt"]["properties"]["definition_id"] = copy.deepcopy(
        b3b["$defs"]["ProductAttempt"]["properties"]["definition_id"])
    b2_with_enum["$comment"] = j1["$comment"]
    report["schema"] = {
        "main_sha256": sha(schema_raw), "b2_only_sha256": sha(b2_raw), "j1_merged_sha256": sha(j1_raw),
        "b3b_only_sha256": sha(pretty(b3b)),
        "j1_equals_b2_plus_b3b_enum": b2_with_enum == j1,
        "j1_defs_minus_b2_defs": sorted(k for k in j1["$defs"] if j1["$defs"][k] != b2["$defs"][k]),
        "j1_defs_minus_b3b_defs": sorted(k for k in j1["$defs"] if b3b["$defs"].get(k) != j1["$defs"][k]),
        "new_defs": sorted(k for k in j1["$defs"] if k not in main_schema["$defs"]),
        "changed_existing_defs": sorted(k for k in main_schema["$defs"] if j1["$defs"][k] != main_schema["$defs"][k]),
        "j1_title": j1["title"], "j1_comment": j1["$comment"], "b2_comment": b2["$comment"],
        "roundtrip_indent2_ascii": pretty(json.loads(j1_raw)) == j1_raw,
    }
    report["reviewed_inputs_cascade"] = {
        "new_reviewed_input": "../../fixtures/results/retained_precision_prepared_combination_v1.json",
        "new_hashes": {"SCHEMA": sha(j1_raw), "PTABLE": sha(table), "DEF-C": sha(defc_raw)},
    }
    (out / "b2c_statics.out.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps(report, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
