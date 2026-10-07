"""I96 B3-D: build the draft statics for J1 (records only; standard library only).

Reads committed bytes only. Writes into <out_dir>:
  retained_precision_prepared_exact_v1.json        the draft definition (DEF-E)
  semantic_contract_v0_3_physics_retained_1.json   the draft physics-retained-1 table
  retained_precision_mp_v2.schema.b3b.json         SCHEMA with B3b's change only (scratch copy)
  SCHEMA_ENUM.diff                                 unified diff of that change against main's SCHEMA
  b3d_statics.out.json                             hashes, controls and member-level deltas

Usage: python b3d_statics.py <P> <out_dir>
"""
import copy
import difflib
import hashlib
import json
import sys
from pathlib import Path

ORDINARY_ID = "RP-PREPARED-ORDINARY-DUAL-v1"
EXACT_ID = "RP-PREPARED-EXACT-DUAL-v1"
BASE_ID = "openpipestress.result_semantics/0.3.0/physics-1"
SUCCESSOR_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
PROFILE = "exact_straight_retained_w1a_v2"


def sha(b):
    return hashlib.sha256(b).hexdigest()


def canonical(v):
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def h(domain, payload):
    return sha(canonical({"domain": domain, "payload": payload}))


def floats(v):
    if isinstance(v, float):
        return 1
    if isinstance(v, dict):
        return sum(floats(x) for x in v.values())
    if isinstance(v, list):
        return sum(floats(x) for x in v)
    return 0


def exact_definition(o):
    d = copy.deepcopy(o)
    d["id"] = EXACT_ID
    d["version"] = 1
    d["inherits"]["base_contract"] = BASE_ID
    d["inherits"]["semantic_contract"] = SUCCESSOR_ID
    d["acceptance"]["standing"] = (
        "G0-G8 remain ordered; pre-S-I absolute/not_covered row and headline binding stays refused; "
        "no S-I activation, private interval export, ordinary-profile guarantee or physics-1 base change")
    d["lanes"]["annular_source"] = (
        "directed1024 annulus geometry; E is the exact lift of the selected common E; G is enclosed as "
        "E/(2(1+nu)) from the exact lifts of the selected common E and nu by directed binary1024 add and "
        "divide (homogeneous_isotropic_E_nu_v1), never from the represented G_hat; use its coefficients "
        "in both residual and recovery")
    d["rows"]["ancillary"] = (
        "actual observed mode/parity bits/text; no modulus-basis record under the base common E/nu "
        "selection; no mechanical proof or invented values")
    d["rows"]["maximum"] = (
        "existing binary64 span-statical coefficient bounder from final projected endpoint actions and "
        "prepared section, empty loads/no pressure state; regenerate the eight numeric fields of "
        "contract_evidence.exact_cases[].pipe_stress_extrema; output RN64(lo+RN64(0.5*RN64(hi-lo))); retain "
        "coefficient scope/location/tie/coverage and certify actual output against separate dual physical maximum")
    d["evidence"] = {
        "pipe_sections": (
            "regenerate contract_evidence.exact_cases[].pipe_sections[].As_m2,I_m4,J_m4,Z_m3 as the prepared "
            "A,I,J,Z bits; outside_diameter_m, effective_wall_thickness_m, ro_m, ri_m, Ai_m2, geometry_basis, "
            "pipe_id and order unchanged"),
        "pipe_stress_extrema": "the maximum row family's eight regenerated numeric fields; every other key unchanged",
        "unchanged": (
            "pressure [], connector [], and each case's load_case_id, profile_mode, material_basis, pipe_materials, "
            "pressure_rhs_assembly and stress_maximum_coverage (complete) byte-identical to the ordinary exact "
            "envelope; no recovery_method member"),
    }
    d["scope"] = {
        "entry": "captured_actual_request_and_mode",
        "excludes": [
            "typed_without_capture", "ordinary_profile_prepared_formation", "prepared_combinations",
            "combinations", "pressure_regions", "pressure_primitives", "load_reference_states",
            "curved_members", "components", "nonlinear_supports", "constant_effort_supports",
            "directional_springs", "nonzero_imposed_motion", "equivalent_static", "user_review_rows",
            "named_point_common_E_nu", "interpolated_common_E_nu",
        ],
        "loads": "individual_normalized_nodal_terms",
        "materials": ["base_common_E_nu_derived_G"],
        "owners": "load_case",
        "prescriptions": "exact_positive_zero",
        "pressure": "explicitly_empty_pressure_regions_and_no_pressure_primitives",
        "requires": "whole_invocation_has_no_exact_block_selection",
        "scope_limit": o["scope"]["scope_limit"],
        "source": "exact_straight_W1a",
        "supports": "existing_rigid_and_global_scalar_spring_laws",
    }
    d["trust"]["G7"] = "unchanged projected physics-1 base evidence validator"
    d["trust"]["G8"] = (
        "independent invocation/model/common E/nu rederivation with shear_modulus bits equal to "
        "RN64(E/(2*RN64(1+nu))), normalized-input/map rederivation, physics-source-1 actual-material binding "
        "over the physics-1 evidence, plus checked source/preparation associations; no historical "
        "physics-source stress recipe")
    return d


def physics_retained_table(p1_bytes, definition_hash):
    t = json.loads(p1_bytes)
    t["semantic_contract_id"] = SUCCESSOR_ID
    t["inherited_semantic_contract_sha256"] = sha(p1_bytes)
    t["formulation_profile_id"] = PROFILE
    t["receipt_policy"] = "M03-INTEGRITY-MP-v2"
    t["receipt_schema"] = "retained_precision_mp_v2.schema.json"
    t["product_formation_definitions"] = [{"id": EXACT_ID, "sha256": definition_hash}]
    t["accuracy_classification"] = {
        "policy": "RP-FACADE-SI-v2",
        "relative_threshold_bits": "3dd0000000000000",
        "small_scale_threshold_bits": "0230000000000000",
        "classes": ["relative_verified", "absolute_verified", "input_derived", "non_quantity", "not_covered"],
        "absolute_bound": "RU64(2^-64 S); for 0<S<2^-988, one RU64 of b0+r+minimum_subnormal; for S=0, canonical +0",
        "small_components": "b0 and r use selected RN64 divide, multiply-back and conditional next-up operations",
        "standing": ("G0 through G8 and actual invocation required; before S-I, absolute_verified and not_covered "
                     "quantity/headline binding stays refused"),
        "scope": ("registered exact-prepared source only: model 0.3.0 with the 2.0.0/exact_straight_pressure_v2 "
                  "contract, explicitly empty pressure regions and the base common E/nu selection; no "
                  "pressure-region, prepared-combination, load-reference-state or ordinary-profile extension"),
    }
    t["formation_warrant"] = {
        "definition_id": EXACT_ID,
        "affected_historical_clauses": ["D2 section4.9.10", "D2 section4.11.2", "D2 section5 I-9"],
        "native": "p/P and stop records describe native operational convergence only",
        "final": "actual final values use the direct certificate against both physical readouts with unchanged b and inequalities",
        "standing": "no S-I activation or new interval-binding grant",
    }
    t["receipt_bindings"] = {
        "canonicalization": "openpipestress_jcs_ijson_v1",
        "method": "contribution_preserving_multiprecision_v1",
        "projection_policy": "RP-LOGICAL-ATTEMPTS-v1",
        "work": {"case_limit": 20000000000, "invocation_limit": 60000000000},
        "work_policy": "W1-LME-20B-60B-v1",
    }
    return (json.dumps(t, indent=2, ensure_ascii=True) + "\n").encode()


def main():
    p, out = Path(sys.argv[1]), Path(sys.argv[2])
    out.mkdir(parents=True, exist_ok=True)
    res = p / "fixtures/results"
    defo_raw = (res / "retained_precision_prepared_ordinary_v1.json").read_bytes()
    defo = json.loads(defo_raw)
    report = {"inputs": {
        "DEF-O": sha(defo_raw),
        "physics-1 table": sha((res / "semantic_contract_v0_3_physics_1.json").read_bytes()),
        "preview-physics-1 table": sha((res / "semantic_contract_v0_3_preview_physics_1.json").read_bytes()),
        "PTABLE": sha((res / "semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes()),
        "SCHEMA": sha((p / "schemas/retained_precision_mp_v2.schema.json").read_bytes()),
    }}
    # Controls: DEF-O's form and H; PTABLE rebuilt from preview-physics-1 by the same method.
    report["control_defo_canonical_bytes"] = canonical(defo) == defo_raw
    report["control_defo_H"] = h("retained_precision_formation_v1", defo)
    pp1 = (res / "semantic_contract_v0_3_preview_physics_1.json").read_bytes()
    pt = json.loads((res / "semantic_contract_v0_3_preview_physics_retained_1.json").read_bytes())
    rebuilt = json.loads(pp1)
    rebuilt["semantic_contract_id"] = pt["semantic_contract_id"]
    rebuilt["inherited_semantic_contract_sha256"] = sha(pp1)
    rebuilt["formulation_profile_id"] = pt["formulation_profile_id"]
    for k in ["receipt_policy", "receipt_schema", "product_formation_definitions", "accuracy_classification", "formation_warrant"]:
        rebuilt[k] = pt[k]
    report["control_ptable_method_reproduces"] = sha((json.dumps(rebuilt, indent=2, ensure_ascii=True) + "\n").encode()) == report["inputs"]["PTABLE"]

    defe = exact_definition(defo)
    defe_raw = canonical(defe)
    assert all(b < 128 for b in defe_raw) and floats(defe) == 0
    (out / "retained_precision_prepared_exact_v1.json").write_bytes(defe_raw)
    defe_h = h("retained_precision_formation_v1", defe)
    report["definition"] = {
        "file": "retained_precision_prepared_exact_v1.json", "bytes": len(defe_raw), "raw_sha256": sha(defe_raw),
        "H_retained_precision_formation_v1": defe_h, "ascii_only": True, "json_floats": 0,
        "preparation_subobject_identical_to_DEF_O": defe["preparation"] == defo["preparation"],
        "top_level_members": sorted(defe),
        "members_differing_from_DEF_O": sorted(
            f"{k}" if not isinstance(defe.get(k), dict) or not isinstance(defo.get(k), dict) else f"{k}.{kk}"
            for k in sorted(set(defe) | set(defo))
            for kk in (sorted(set(defe.get(k, {})) | set(defo.get(k, {}))) if isinstance(defe.get(k), dict) and isinstance(defo.get(k), dict) else [None])
            if (defe.get(k) != defo.get(k)) and (kk is None or defe.get(k, {}).get(kk) != defo.get(k, {}).get(kk))),
    }

    p1 = (res / "semantic_contract_v0_3_physics_1.json").read_bytes()
    table = physics_retained_table(p1, defe_h)
    (out / "semantic_contract_v0_3_physics_retained_1.json").write_bytes(table)
    tj, p1j = json.loads(table), json.loads(p1)
    report["table"] = {
        "file": "semantic_contract_v0_3_physics_retained_1.json", "bytes": len(table), "sha256": sha(table),
        "inherited_semantic_contract_sha256": tj["inherited_semantic_contract_sha256"],
        "members_changed_from_physics_1": [k for k in p1j if tj[k] != p1j[k]],
        "members_added": [k for k in tj if k not in p1j],
        "rows_identical_to_physics_1": tj["rows"] == p1j["rows"],
        "reserved_inactive_successors_identical": tj["reserved_inactive_successors"] == p1j["reserved_inactive_successors"],
        "roundtrip_indent2_ascii": (json.dumps(tj, indent=2, ensure_ascii=True) + "\n").encode() == table,
    }

    sp = p / "schemas/retained_precision_mp_v2.schema.json"
    sraw = sp.read_bytes()
    s = json.loads(sraw)
    assert s["$defs"]["ProductAttempt"]["properties"]["definition_id"] == {"const": ORDINARY_ID}
    s["$defs"]["ProductAttempt"]["properties"]["definition_id"] = {"enum": [ORDINARY_ID, EXACT_ID]}
    old_comment = s["$comment"]
    prefix = "Ordinary-prepared scope only; exact and prepared-combination admission require their later selected extension."
    assert old_comment.startswith(prefix)
    s["$comment"] = ("Ordinary-prepared and exact-prepared scope; prepared-combination admission requires its later "
                     "selected extension." + old_comment[len(prefix):])
    snew = (json.dumps(s, indent=2, ensure_ascii=True) + "\n").encode()
    assert (json.dumps(json.loads(sraw), indent=2, ensure_ascii=True) + "\n").encode() == sraw
    (out / "retained_precision_mp_v2.schema.b3b.json").write_bytes(snew)
    diff = difflib.unified_diff(sraw.decode().splitlines(True), snew.decode().splitlines(True),
                                "a/P/schemas/retained_precision_mp_v2.schema.json",
                                "b/P/schemas/retained_precision_mp_v2.schema.json", n=3)
    (out / "SCHEMA_ENUM.diff").write_text("".join(diff))
    report["schema"] = {"main_sha256": sha(sraw), "b3b_only_sha256": sha(snew),
                        "note": "B3b's change alone, against main; J1 merges it with B2-C's SCHEMA changes"}
    (out / "b3d_statics.out.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps(report, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
