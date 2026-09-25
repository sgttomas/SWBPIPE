"""Write the shared Rust/Python load-reference-1 adversarial case file.

Expectations are authored here by reading the wire (CP2_WIRE.md, addendum 1
§3) and the reader check order. They are not captured from either reader.
Output: core/reporting/result_export/tests/fixtures/load_reference_mutations.json
Usage: python make_mutations.py <WORKING_ROOT>
"""
import json
import math
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
out = root / "core/reporting/result_export/tests/fixtures/load_reference_mutations.json"

LR = "/contract_evidence/load_reference_states"
EX = "/contract_evidence/exact_cases"
PR = "/contract_evidence/pressure"
NOT_JOINED = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
P = "SOURCE_LOAD_REFERENCE_"
PHYS = P + "PHYSICS_EVIDENCE"  # inherited physics-1 check; compared by leading code

cases = []


def case(case_id, source, ops, dispatch, validator=None, **extra):
    item = {"id": case_id, "source": source, "ops": ops, "dispatch": dispatch}
    if validator is not None:
        item["validator"] = validator
    item.update(extra)
    cases.append(item)


def s(path, value):
    return {"op": "set", "path": path, "value": value}


def rm(path):
    return {"op": "remove", "path": path}


def ap(path, value):
    return {"op": "append", "path": path, "value": value}


def cp(src, dst):
    return {"op": "copy", "from": src, "path": dst}


def cpa(src, dst):
    return {"op": "copy_append", "from": src, "path": dst}


def sw(a, b):
    return {"op": "swap", "path": a, "with": b}


def rmw(path, match):
    return {"op": "remove_where", "path": path, "match": match}


def sww(path, match, key, value):
    return {"op": "set_where", "path": path, "match": match, "key": key, "value": value}


def acw(path, match, fields):
    return {"op": "append_copy_where", "path": path, "match": match, "set": fields}


def nf(path, value):
    return {"op": "nonfinite", "path": path, "value": value}


r0, r1 = f"{LR}/0", f"{LR}/1"
m0, m1 = f"{r0}/members/0", f"{r0}/members/1"
cold_nj = {"code": NOT_JOINED, "id": "diagnostic:load-state:case-cold:source-recovery-not-joined"}

# Accepted: frozen producer bytes and order-only or tolerated changes.
for src in ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"]:
    case(f"ACCEPT-{src}", src, [], "accept")
case("ACCEPT-member-order", "connected-sparse", [sw(m0, m1)], "accept")
case("ACCEPT-contribution-order", "connected-sparse", [sw(f"{r0}/contributions/0", f"{r0}/contributions/4")], "accept")
case("ACCEPT-support-component-order", "connected-sparse", [sw(f"{r0}/support_components/0", f"{r0}/support_components/1")], "accept")
case("ACCEPT-record-order", "connected-sparse", [sw(r0, r1)], "accept")
case("ACCEPT-record-provenance-text", "pressure-sparse", [s(f"{r0}/provenance", "another invented provenance")], "accept")
case("ACCEPT-not-joined-message-text", "connected-sparse", [sww("/diagnostics", cold_nj, "message", "regenerated message text")], "accept")
case("ACCEPT-source-block-unavailable-info", "connected-sparse", [ap("/diagnostics", {
    "id": "diagnostic:source-blocks:case-cold:unavailable", "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "severity": "info",
    "message": "retained-source recovery unavailable; ordinary route published", "source": "core/product_physics", "affected_refs": ["case:cold"]})], "accept")
case("ACCEPT-unsolved-empty-namespace", "pressure-sparse", [
    s("/status/mechanics", "MODEL_INCOMPLETE"), s(LR, []), s(EX, []), s(PR, []), s("/results", []),
    s("/summary/max_open_formula_stress", None)], "accept")

# Unknown key at each object level.
for cid, src, path, code in [
    ("UNKNOWN-evidence", "connected-sparse", "/contract_evidence", "EVIDENCE_SHAPE"),
    ("UNKNOWN-record", "connected-sparse", r0, "RECORD_SHAPE"),
    ("UNKNOWN-reference-geometry", "connected-sparse", f"{r0}/reference_geometry", "REFERENCE_GEOMETRY_SHAPE"),
    ("UNKNOWN-history", "connected-sparse", f"{r0}/history", "HISTORY_SHAPE"),
    ("UNKNOWN-solve", "connected-sparse", f"{r0}/solve", "SOLVE_SHAPE"),
    ("UNKNOWN-source-recovery", "connected-sparse", f"{r0}/source_recovery", "SOURCE_RECOVERY_SHAPE"),
    ("UNKNOWN-member", "connected-sparse", m0, "MEMBER_SHAPE"),
    ("UNKNOWN-material-point", "connected-sparse", f"{m0}/consumed_material_points/0", "MATERIAL_POINT_SHAPE"),
    ("UNKNOWN-fit-input", "connected-sparse", f"{m1}/fit_input", "FIT_BINDING"),
    ("UNKNOWN-support-component", "connected-sparse", f"{r0}/support_components/0", "SUPPORT_COMPONENT_SHAPE"),
    ("UNKNOWN-member-state-contribution", "connected-sparse", f"{r0}/contributions/0", "CONTRIBUTION_SHAPE"),
    ("UNKNOWN-support-state-contribution", "connected-sparse", f"{r0}/contributions/2", "CONTRIBUTION_SHAPE"),
    ("UNKNOWN-stored-contribution", "connected-sparse", f"{r0}/contributions/4", "CONTRIBUTION_SHAPE"),
    ("UNKNOWN-pressure-region-contribution", "pressure-sparse", f"{r0}/contributions/1", "CONTRIBUTION_SHAPE"),
    ("UNKNOWN-excluded-source", "connected-sparse", f"{r0}/excluded_sources/0", "EXCLUDED_SHAPE"),
    ("UNKNOWN-exact-case", "connected-sparse", f"{EX}/0", "CASE_SHAPE"),
    ("UNKNOWN-case-material", "connected-sparse", f"{EX}/0/pipe_materials/0", "MATERIAL_SHAPE"),
    ("UNKNOWN-region", "pressure-sparse", f"{PR}/0", "REGION_SHAPE"),
    ("UNKNOWN-region-material", "pressure-sparse", f"{PR}/0/materials/0", "REGION_MATERIAL_SHAPE"),
]:
    case(cid, src, [s(f"{path}/unreviewed_extra", "x")], P + code)
case("UNKNOWN-override", "connected-sparse", [s(f"{m0}/analysis_basis_override", {"reason": "r", "provenance": "p", "extra": 1})], P + "OVERRIDE_SHAPE")
case("UNKNOWN-law-segment", "connected-sparse", [s(f"{m0}/consumed_law_segments", [{"use": "integration_interval", "lower_index": 0, "upper_index": 1, "start_k": 293.15, "end_k": 293.15, "extra": 1}])], P + "LAW_SEGMENT_SHAPE")
case("UNKNOWN-case-section-inherited", "connected-sparse", [s(f"{EX}/0/pipe_sections/0/unreviewed_extra", "x")], PHYS)
case("UNKNOWN-maximum-coverage-inherited", "connected-sparse", [s(f"{EX}/0/stress_maximum_coverage/unreviewed_extra", "x")], PHYS)

# Missing key at each object level (null-valued optionals included).
for cid, src, path, code in [
    ("MISSING-load-reference-states", "connected-sparse", f"{LR}", "EVIDENCE_SHAPE"),
    ("MISSING-contract-evidence", "connected-sparse", "/contract_evidence", "EVIDENCE_SHAPE"),
    ("MISSING-record-source-recovery", "connected-sparse", f"{r0}/source_recovery", "RECORD_SHAPE"),
    ("MISSING-projection-sha256", "connected-sparse", f"{r0}/reference_geometry/projection_sha256", "REFERENCE_GEOMETRY_SHAPE"),
    ("MISSING-history-kind", "connected-sparse", f"{r0}/history/kind", "HISTORY_SHAPE"),
    ("MISSING-solve-eigenload", "connected-sparse", f"{r0}/solve/eigenload", "SOLVE_SHAPE"),
    ("MISSING-source-recovery-code", "connected-sparse", f"{r0}/source_recovery/code", "SOURCE_RECOVERY_SHAPE"),
    ("MISSING-member-null-optional", "connected-sparse", f"{m0}/analysis_basis_override", "MEMBER_SHAPE"),
    ("MISSING-member-datum", "connected-sparse", f"{m0}/coefficient_datum_k", "MEMBER_SHAPE"),
    ("MISSING-material-point-flag", "connected-sparse", f"{m0}/consumed_material_points/0/retained_G_ignored", "MATERIAL_POINT_SHAPE"),
    ("MISSING-fit-length-change", "connected-sparse", f"{m1}/fit_input/length_change_m", "FIT_BINDING"),
    ("MISSING-support-state-source", "connected-sparse", f"{r0}/support_components/0/physical_state_source", "SUPPORT_COMPONENT_SHAPE"),
    ("MISSING-member-state-refs", "connected-sparse", f"{r0}/contributions/0/consumed_input_refs", "CONTRIBUTION_SHAPE"),
    ("MISSING-support-state-value", "connected-sparse", f"{r0}/contributions/2/value", "CONTRIBUTION_SHAPE"),
    ("MISSING-stored-applied-magnitude", "connected-sparse", f"{r0}/contributions/4/applied_magnitude", "CONTRIBUTION_SHAPE"),
    ("MISSING-pressure-region-factor", "pressure-sparse", f"{r0}/contributions/1/factor", "CONTRIBUTION_SHAPE"),
    ("MISSING-excluded-reason", "connected-sparse", f"{r0}/excluded_sources/0/reason", "EXCLUDED_SHAPE"),
    ("MISSING-case-material-basis", "connected-sparse", f"{EX}/0/material_basis", "CASE_SHAPE"),
    ("MISSING-case-resolved-eigenstrain", "connected-sparse", f"{EX}/0/pipe_materials/0/resolved_eigenstrain", "MATERIAL_SHAPE"),
    ("MISSING-region-temperature-basis", "pressure-sparse", f"{PR}/0/materials/0/temperature_basis", "REGION_MATERIAL_SHAPE"),
]:
    case(cid, src, [rm(path)], P + code)

# Forged E/nu/G and resolved-member facts.
e_forged = 199e9
g_forged = e_forged / (2.0 * (1.0 + 0.3))
g_one_ulp = math.nextafter(76923076923.07692, math.inf)  # produced G plus one binary64 ulp
case("FORGE-case-E", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/E_pa", e_forged)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-nu", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/nu", 0.29)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-G-one-ulp", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/G_pa", g_one_ulp)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-consistent-E-G", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/E_pa", e_forged), s(f"{EX}/0/pipe_materials/0/G_pa", g_forged)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-member-E-only", "connected-sparse", [s(f"{m0}/selected_E_pa", e_forged)], P + "MEMBER_G_BINDING")
case("FORGE-member-consistent-pair", "connected-sparse", [s(f"{m0}/selected_E_pa", e_forged), s(f"{m0}/derived_G_pa", g_forged), s(f"{m0}/consumed_material_points/0/E_pa", e_forged)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-member-G", "connected-sparse", [s(f"{m0}/derived_G_pa", 1.0)], P + "MEMBER_G_BINDING")
case("FORGE-member-nu-range", "connected-sparse", [s(f"{m0}/selected_nu", 0.5)], P + "MEMBER_MATERIAL_RANGE")
case("FORGE-region-E", "pressure-sparse", [s(f"{PR}/0/materials/0/E_pa", e_forged)], P + "REGION_MATERIAL_BINDING")
case("FORGE-region-nu", "pressure-sparse", [s(f"{PR}/0/materials/0/nu", 0.29)], P + "REGION_MATERIAL_BINDING")
case("FORGE-region-G", "pressure-sparse", [s(f"{PR}/0/materials/0/G_pa", g_one_ulp)], P + "REGION_MATERIAL_BINDING")
case("FORGE-case-selection-kind", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/material_selection_kind", "temperature_interpolation")], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-resolved-eigenstrain", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/resolved_eigenstrain", 0.5)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-thermal-consumed", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/thermal_consumed", True), s(f"{EX}/0/pipe_materials/0/alpha_per_kelvin", 1e-5)], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-material-id", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/material_id", "material:other")], P + "MEMBER_MATERIAL_BINDING")
case("FORGE-case-null-provenance", "connected-sparse", [s(f"{EX}/0/pipe_materials/0/provenance", None)], P + "STRING_INVALID")

# Cross-case swaps.
case("SWAP-record-case-ids", "connected-sparse", [s(f"{r0}/load_case_id", "case:hot"), s(f"{r1}/load_case_id", "case:cold")], P + "MEMBER_MATERIAL_BINDING")
case("SWAP-record-members", "connected-sparse", [sw(f"{r0}/members", f"{r1}/members")], P + "MEMBER_MATERIAL_BINDING")
case("SWAP-case-materials", "connected-sparse", [sw(f"{EX}/0/pipe_materials", f"{EX}/1/pipe_materials")], P + "MEMBER_MATERIAL_BINDING")
case("SWAP-region-materials", "pressure-sparse", [sw(f"{PR}/0/materials", f"{PR}/1/materials")], P + "REGION_MATERIAL_BINDING")
case("SWAP-record-contributions", "connected-sparse", [sw(f"{r0}/contributions", f"{r1}/contributions")], P + "MEMBER_CONTRIBUTION_BINDING")
case("SWAP-pressure-record-case-ids", "pressure-sparse", [s(f"{r0}/load_case_id", "case:hot-pressure"), s(f"{r1}/load_case_id", "case:cold-pressure")], P + "MEMBER_MATERIAL_BINDING")
case("SWAP-not-joined-affected-case", "connected-sparse", [sww("/diagnostics", cold_nj, "affected_refs", ["case:hot"])], P + "NOT_JOINED_DIAGNOSTIC")
case("SWAP-numerical-evidence-inherited", "connected-sparse", [sw("/numerical_quality/cases/0/evidence_refs", "/numerical_quality/cases/1/evidence_refs")], PHYS)

# Not-joined diagnostic.
case("NOTJOINED-missing-one", "connected-sparse", [rmw("/diagnostics", cold_nj)], P + "NOT_JOINED_DIAGNOSTIC")
case("NOTJOINED-missing-all", "pressure-sparse", [rmw("/diagnostics", {"code": NOT_JOINED})], P + "NOT_JOINED_DIAGNOSTIC")
case("NOTJOINED-severity", "connected-sparse", [sww("/diagnostics", cold_nj, "severity", "warning")], P + "NOT_JOINED_DIAGNOSTIC")
case("NOTJOINED-extra-copy", "connected-sparse", [acw("/diagnostics", cold_nj, {"id": "diagnostic:load-state:case-cold:source-recovery-not-joined-2"})], P + "NOT_JOINED_DIAGNOSTIC")
case("NOTJOINED-wrong-id", "connected-sparse", [sww("/diagnostics", cold_nj, "id", "diagnostic:load-state:case:cold:source-recovery-not-joined")], P + "NOT_JOINED_DIAGNOSTIC")

# Source recovery statement.
case("RECOVERY-status-joined", "connected-sparse", [s(f"{r0}/source_recovery/status", "joined")], P + "SOURCE_RECOVERY")
case("RECOVERY-code", "connected-sparse", [s(f"{r0}/source_recovery/code", "SOURCE_BLOCK_RECOVERY_UNAVAILABLE")], P + "SOURCE_RECOVERY")
case("RECOVERY-status-recovered-second-case", "pressure-sparse", [s(f"{r1}/source_recovery/status", "recovered")], P + "SOURCE_RECOVERY")

# Source receipt and foreign namespaces.
case("RECEIPT-present", "connected-sparse", [s("/source_block_recovery", {})], "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN", validator=P + "FOREIGN_METHOD_EVIDENCE")
case("RECEIPT-present-pressure", "pressure-dense", [s("/source_block_recovery", {"receipt_sha256": "0" * 64, "body": {}})], "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN", validator=P + "FOREIGN_METHOD_EVIDENCE")
case("RECEIPT-inside-evidence", "connected-sparse", [s("/contract_evidence/source_block_recovery", {})], P + "EVIDENCE_SHAPE")
case("CARRIER-present", "connected-sparse", [s("/carrier_evidence", {})], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", validator=P + "FOREIGN_METHOD_EVIDENCE")

# Profile and contract mix-ups.
case("PROFILE-exact", "connected-sparse", [s("/formulation_basis/profile_id", "exact_straight_pressure_v2")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator="accept")
case("PROFILE-source-successor", "connected-sparse", [s("/formulation_basis/profile_id", "resolved_straight_load_state_source_v1")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator="accept")
case("PROFILE-preview", "pressure-sparse", [s("/formulation_basis/profile_id", "product_preview_mechanics_v1")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator="accept")
case("PROFILE-record", "connected-sparse", [s(f"{r0}/profile", "exact_straight_pressure_v2")], P + "RECORD_PROFILE")
case("PROFILE-exact-case-mode-inherited", "connected-sparse", [s(f"{EX}/0/profile_mode", "resolved_straight_load_state_v1")], PHYS)
case("PROFILE-physics-envelope-claims-load-profile", "physics-sparse", [s("/formulation_basis/profile_id", "resolved_straight_load_state_v1")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator=P + "EVIDENCE_SHAPE")
case("CONTRACT-physics-envelope-relabelled", "physics-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/load-reference-1"), s("/formulation_basis/profile_id", "resolved_straight_load_state_v1")], P + "EVIDENCE_SHAPE")
case("CONTRACT-physics-envelope-relabelled-keeps-profile", "physics-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/load-reference-1")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator=P + "EVIDENCE_SHAPE")
case("CONTRACT-record-contract", "connected-sparse", [s(f"{r0}/contract", "openpipestress.load_reference_state/2.0.0")], P + "RECORD_CONTRACT")

# Wrong table identity at dispatch (the table hash itself is in table_cases).
case("TABLE-id-successor-2", "connected-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/load-reference-2")], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", validator="accept")
case("TABLE-id-reserved-source-successor", "connected-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/load-reference-source-1")], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", validator="accept")
case("TABLE-id-physics-1", "connected-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/physics-1")], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", validator="accept")
case("TABLE-id-physics-1-exact-profile", "connected-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/physics-1"), s("/formulation_basis/profile_id", "exact_straight_pressure_v2")], P + "EVIDENCE_FORBIDDEN", validator="accept")
case("TABLE-id-physics-source-1-exact-profile", "pressure-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/physics-source-1"), s("/formulation_basis/profile_id", "exact_straight_pressure_v2")], P + "EVIDENCE_FORBIDDEN", validator="accept")
case("TABLE-id-precision-1", "connected-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/precision-1")], None,
     validator="accept", dispatch_rust="SOURCE_FORMULATION_BASIS_UNSUPPORTED", dispatch_python="SOURCE_PHYSICS_CONTRACT_MISMATCH",
     note="inherited header order for precision-1 (unchanged by this TASK); both readers reject")
case("TABLE-producer-version", "connected-sparse", [s("/producer/component_version", "0.3.0")], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", validator="accept")

# load_reference_states on a physics-1 envelope.
case("PHYSICS1-with-load-reference-states", "physics-sparse", [{"op": "graft", "source": "connected-sparse", "from": LR, "path": LR}], P + "EVIDENCE_FORBIDDEN", validator=P + "MATERIAL_BASIS")
case("PHYSICS1-with-empty-load-reference-states", "physics-sparse", [s(LR, [])], P + "EVIDENCE_FORBIDDEN", validator=P + "MATERIAL_BASIS")

# Duplicate source IDs and other duplicates.
case("DUPLICATE-stored-contribution", "connected-sparse", [cpa(f"{r0}/contributions/4", f"{r0}/contributions")], P + "CONTRIBUTION_DUPLICATE")
case("DUPLICATE-source-id-across-kinds", "connected-sparse", [s(f"{r0}/contributions/4/source_id", "member_state:pipe:first")], P + "CONTRIBUTION_DUPLICATE")
case("DUPLICATE-excluded-included", "connected-sparse", [s(f"{r0}/excluded_sources/0/source_id", "load:transverse:case:cold")], P + "EXCLUDED_OVERLAP")
case("DUPLICATE-excluded", "connected-sparse", [cpa(f"{r0}/excluded_sources/0", f"{r0}/excluded_sources")], P + "EXCLUDED_OVERLAP")
case("DUPLICATE-member", "connected-sparse", [cpa(m0, f"{r0}/members")], P + "MEMBER_DUPLICATE")
case("DUPLICATE-support-component", "connected-sparse", [cpa(f"{r0}/support_components/0", f"{r0}/support_components")], P + "SUPPORT_COMPONENT_DUPLICATE")
case("DUPLICATE-record", "connected-sparse", [cpa(r0, LR)], P + "RECORD_DUPLICATE")
case("DUPLICATE-exact-case", "connected-sparse", [cpa(f"{EX}/0", EX)], P + "CASE_DUPLICATE")

# Factors.
st = f"{r0}/contributions/4"
case("FACTOR-zero", "connected-sparse", [s(f"{st}/factor", 0)], P + "CONTRIBUTION_FACTOR")
case("FACTOR-negative-zero", "connected-sparse", [s(f"{st}/factor", -0.0)], P + "CONTRIBUTION_FACTOR")
case("FACTOR-null", "connected-sparse", [s(f"{st}/factor", None)], P + "CONTRIBUTION_FACTOR")
case("FACTOR-string", "connected-sparse", [s(f"{st}/factor", "1")], P + "CONTRIBUTION_FACTOR")
case("FACTOR-changed-without-magnitude", "connected-sparse", [s(f"{st}/factor", 2.0)], P + "CONTRIBUTION_APPLIED_MAGNITUDE")
case("FACTOR-pressure-region-nonnull", "pressure-sparse", [s(f"{r0}/contributions/1/factor", 1.0)], P + "CONTRIBUTION_FACTOR")

# Non-finite numbers. serde_json cannot hold them: Rust rejects the JSON text.
for cid, path, value in [
    ("NONFINITE-member-E", f"{m0}/selected_E_pa", "Infinity"),
    ("NONFINITE-factor", f"{st}/factor", "NaN"),
    ("NONFINITE-row-value", "/results/0/value", "-Infinity"),
    ("NONFINITE-region-pressure", f"{PR}/0/p_pa", "Infinity"),
]:
    case(cid, "pressure-sparse" if "region" in cid else "connected-sparse", [nf(path, value)], None,
         dispatch_rust="JSON_PARSE_REJECTED", dispatch_python=P + "NUMBER_INVALID",
         validator_rust="JSON_PARSE_REJECTED", validator_python=P + "NUMBER_INVALID",
         note="non-finite numbers are unrepresentable in serde_json Value; Rust rejects at the JSON text boundary")

# Other closed statements and bindings.
for cid, src, ops, code in [
    ("CLOSED-history-kind", "connected-sparse", [s(f"{r0}/history/kind", "continuation")], "HISTORY"),
    ("CLOSED-solve-mode-method", "connected-sparse", [s(f"{r0}/solve/requested_mode", "dense_scrutiny")], "SOLVE"),
    ("CLOSED-solve-mode-consistency", "connected-sparse", [s(f"{r0}/solve/requested_mode", "dense_scrutiny"), s(f"{r0}/solve/recovery_method", "ordinary_dense_structural_v1")], "SOLVE_CONSISTENCY"),
    ("CLOSED-solve-boundary-text", "connected-sparse", [s(f"{r0}/solve/boundary", "prescribed")], "SOLVE"),
    ("CLOSED-geometry-hash", "connected-sparse", [s(f"{r0}/reference_geometry/projection_sha256", "ABC")], "REFERENCE_GEOMETRY"),
    ("CLOSED-geometry-kind", "connected-sparse", [s(f"{r0}/reference_geometry/kind", "deformed_geometry")], "REFERENCE_GEOMETRY"),
    ("CLOSED-geometry-consistency", "connected-sparse", [s(f"{r1}/reference_geometry/projection_sha256", "0" * 64)], "REFERENCE_GEOMETRY_CONSISTENCY"),
    ("CLOSED-material-basis", "connected-sparse", [s(f"{EX}/0/material_basis", "base_material_common_E_nu")], "MATERIAL_BASIS"),
    ("CLOSED-selection-kind", "connected-sparse", [s(f"{m0}/material_selection_kind", "library_lookup")], "MATERIAL_SELECTION_KIND"),
    ("CLOSED-thermal-definition", "connected-sparse", [s(f"{m0}/thermal_definition", "secant")], "THERMAL_DEFINITION"),
    ("CLOSED-fit-kind", "connected-sparse", [s(f"{m0}/fit_kind", "fit_up")], "FIT_KIND"),
    ("CLOSED-fit-strain-stretch", "connected-sparse", [s(f"{m1}/fit_strain", -0.002)], "STRETCH_BINDING"),
    ("CLOSED-thermal-stretch", "connected-sparse", [s(f"{m0}/thermal_stretch", 1.1)], "STRETCH_BINDING"),
    ("CLOSED-fit-length-change", "connected-sparse", [s(f"{m1}/fit_input/length_change_m", -0.002)], "FIT_BINDING"),
    ("CLOSED-fit-none-input", "connected-sparse", [s(f"{m0}/fit_input", {"strain": 0.0})], "FIT_BINDING"),
    ("CLOSED-reference-basis", "connected-sparse", [s(f"{m0}/reference_basis", "direct_strain_reference")], "REFERENCE_BASIS"),
    ("CLOSED-reference-basis-unknown", "connected-sparse", [s(f"{m0}/reference_basis", "installed")], "REFERENCE_BASIS"),
    ("CLOSED-law-id-null", "connected-sparse", [s(f"{m0}/expansion_law_id", None)], "THERMAL_LAW_BINDING"),
    ("CLOSED-law-datum-stretch-null", "connected-sparse", [s(f"{m0}/installation_datum_stretch", None)], "THERMAL_LAW_BINDING"),
    ("CLOSED-exact-point-without-point", "connected-sparse", [s(f"{m0}/consumed_material_points", [])], "MATERIAL_SELECTION"),
    ("CLOSED-exact-point-fraction", "connected-sparse", [s(f"{m0}/interpolation_fraction", 0.5)], "MATERIAL_SELECTION"),
    ("CLOSED-exact-point-temperature", "connected-sparse", [s(f"{m0}/material_selection_temperature_k", 300.0)], "MATERIAL_SELECTION"),
    ("CLOSED-law-indices-order", "connected-sparse", [s(f"{m0}/consumed_law_point_indices", [2, 1])], "LAW_INDICES"),
    ("CLOSED-law-indices-integer", "connected-sparse", [s(f"{m0}/consumed_law_point_indices", [1.5])], "INTEGER_INVALID"),
    ("CLOSED-law-segment-use", "connected-sparse", [s(f"{m0}/consumed_law_segments", [{"use": "extrapolation", "lower_index": 0, "upper_index": 1, "start_k": 293.15, "end_k": 373.15}])], "LAW_SEGMENT"),
    ("CLOSED-law-segment-sample", "connected-sparse", [s(f"{m0}/consumed_law_segments", [{"use": "interpolation_sample", "lower_index": 0, "upper_index": 1, "start_k": 293.15, "end_k": 373.15}])], "LAW_SEGMENT"),
    ("CLOSED-temperature-range", "connected-sparse", [s(f"{m0}/operating_temperature_k", -1.0)], "TEMPERATURE_RANGE"),
    ("CLOSED-retained-flag-type", "connected-sparse", [s(f"{m0}/retained_G_ignored", "false")], "BOOLEAN_INVALID"),
    ("CLOSED-g-basis-text", "connected-sparse", [s(f"{m0}/G_basis", "retained G")], "MEMBER_BASIS"),
    ("CLOSED-support-dof", "connected-sparse", [s(f"{r0}/support_components/0/dof", "RX")], "SUPPORT_COMPONENT_DOF"),
    ("CLOSED-support-unit", "connected-sparse", [s(f"{r0}/support_components/0/unit", "rad")], "SUPPORT_COMPONENT_UNIT"),
    ("CLOSED-support-law", "connected-sparse", [s(f"{r0}/support_components/0/law_kind", "spring")], "SUPPORT_COMPONENT_LAW"),
    ("CLOSED-support-node", "connected-sparse", [s(f"{r0}/support_components/1/node_id", "node:other")], "SUPPORT_COMPONENT_NODE"),
    ("CLOSED-support-contribution-value", "connected-sparse", [s(f"{r0}/contributions/2/value", 1.0)], "SUPPORT_CONTRIBUTION_BINDING"),
    ("CLOSED-support-contribution-missing", "connected-sparse", [rm(f"{r0}/contributions/2")], "SUPPORT_CONTRIBUTION_COVERAGE"),
    ("CLOSED-member-contribution-value", "connected-sparse", [s(f"{r0}/contributions/0/value", 0.5)], "MEMBER_CONTRIBUTION_BINDING"),
    ("CLOSED-member-contribution-refs", "connected-sparse", [s(f"{r0}/contributions/0/consumed_input_refs", ["reference:installed:pipe:first", "case:hot:element_state:pipe:first"])], "MEMBER_CONTRIBUTION_BINDING"),
    ("CLOSED-member-contribution-missing", "connected-sparse", [rm(f"{r0}/contributions/0")], "MEMBER_CONTRIBUTION_COVERAGE"),
    ("CLOSED-pressure-contribution-missing", "pressure-sparse", [rm(f"{r0}/contributions/1")], "PRESSURE_CONTRIBUTION_COVERAGE"),
    ("CLOSED-pressure-contribution-unowned", "connected-sparse", [ap(f"{r0}/contributions", {"source_id": "pressure_region:region:none", "owner_kind": "pressure_region", "classification": "pressure_eigen_and_closure", "factor": None})], "PRESSURE_CONTRIBUTION_COVERAGE"),
    ("CLOSED-contribution-kind", "connected-sparse", [s(f"{st}/owner_kind", "base_motion")], "CONTRIBUTION_KIND"),
    ("CLOSED-contribution-classification", "connected-sparse", [s(f"{st}/classification", "excluded")], "CONTRIBUTION_CLASSIFICATION"),
    ("CLOSED-applied-magnitude", "connected-sparse", [s(f"{st}/applied_magnitude", -999.0)], "CONTRIBUTION_APPLIED_MAGNITUDE"),
    ("CLOSED-excluded-classification", "connected-sparse", [s(f"{r0}/excluded_sources/0/classification", "ordinary_applied")], "EXCLUDED_CLASSIFICATION"),
    ("CLOSED-member-removed", "connected-sparse", [rm(m1)], "MEMBER_COVERAGE"),
    ("CLOSED-record-removed", "connected-sparse", [rm(r1)], "CASE_COVERAGE"),
    ("CLOSED-exact-case-removed", "connected-sparse", [rm(f"{EX}/1")], "CASE_COVERAGE"),
    ("CLOSED-connector", "connected-sparse", [s("/contract_evidence/connector", [{}])], "CONNECTOR_UNSUPPORTED"),
    ("CLOSED-region-temperature-basis", "pressure-sparse", [s(f"{PR}/0/materials/0/temperature_basis", {"selection": "base_material"})], "REGION_TEMPERATURE_BASIS"),
    ("CLOSED-region-case-moved", "pressure-sparse", [s(f"{PR}/0/load_case_id", "case:missing")], "PRESSURE_CONTRIBUTION_COVERAGE"),
    ("CLOSED-unsolved-with-evidence", "connected-sparse", [s("/status/mechanics", "MODEL_INCOMPLETE")], "UNSOLVED_EVIDENCE"),
    ("CLOSED-numerical-basis-type", "connected-sparse", [s("/numerical_quality/cases/0/basis_ref/ref_type", "combination")], "NUMERICAL_CASE_BASIS"),
    ("CLOSED-record-provenance-empty", "connected-sparse", [s(f"{r0}/provenance", "")], "STRING_INVALID"),
]:
    case(cid, src, ops, P + code)

# Inherited physics-1 checks (rows, extrema, RHS, headline) on the projection.
case("INHERITED-maximum-unit", "connected-sparse", [sww("/results", {"kind": "pipe_elastic_normal_stress_maximum_v2"}, "unit", "MPa")], PHYS)
case("INHERITED-extrema-gap", "connected-sparse", [s(f"{EX}/0/pipe_stress_extrema/0/certified_gap_pa", 1e9)], PHYS)
case("INHERITED-rhs-method", "pressure-sparse", [s(f"{EX}/0/pressure_rhs_assembly/method", "source_block_recovery_v1")], PHYS)
case("INHERITED-headline", "connected-sparse", [s("/summary/max_open_formula_stress/value", 1.0)], PHYS)
case("INHERITED-section-diameter", "connected-sparse", [s(f"{EX}/0/pipe_sections/0/outside_diameter_m", 0.3)], PHYS)
case("INHERITED-region-pressure-rhs", "pressure-sparse", [s(f"{PR}/0/p_pa", 1.0)], PHYS)

table_cases = [
    {"id": "TABLEBYTES-pinned", "find": None, "replace": None, "expect": "accept"},
    {"id": "TABLEBYTES-id-edit", "find": "0.3.0/load-reference-1", "replace": "0.3.0/load-reference-2", "expect": P + "TABLE_HASH"},
    {"id": "TABLEBYTES-profile-edit", "find": "\"resolved_straight_load_state_v1\"", "replace": "\"exact_straight_pressure_v2\"", "expect": P + "TABLE_HASH"},
    {"id": "TABLEBYTES-trailing-space", "find": "\n}", "replace": "\n} ", "expect": P + "TABLE_HASH"},
]

transport_cases = [
    *({"id": f"TRANSPORT-ACCEPT-{src}", "source": src, "ops": [], "expect": "accept"} for src in ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"]),
    {"id": "TRANSPORT-member-extra-key", "source": "connected-sparse", "ops": [s(f"{m0}/extra", 1)], "expect": P + "TRANSPORT_SHAPE"},
    {"id": "TRANSPORT-record-profile", "source": "connected-sparse", "ops": [s(f"{r0}/profile", "exact_straight_pressure_v2")], "expect": P + "TRANSPORT_SHAPE"},
    {"id": "TRANSPORT-forged-case-E", "source": "connected-sparse", "ops": [s(f"{EX}/0/pipe_materials/0/E_pa", e_forged)], "expect": P + "MEMBER_MATERIAL_BINDING"},
    {"id": "TRANSPORT-forged-region-E", "source": "pressure-sparse", "ops": [s(f"{PR}/0/materials/0/E_pa", e_forged)], "expect": P + "REGION_MATERIAL_BINDING"},
    {"id": "TRANSPORT-record-swap", "source": "pressure-sparse", "ops": [s(f"{r0}/load_case_id", "case:hot-pressure"), s(f"{r1}/load_case_id", "case:cold-pressure")], "expect": P + "MEMBER_MATERIAL_BINDING"},
    {"id": "TRANSPORT-no-diagnostic-binding", "source": "connected-sparse", "ops": [rmw("/diagnostics", {"code": NOT_JOINED})], "expect": "accept"},
    {"id": "TRANSPORT-receipt", "source": "connected-sparse", "ops": [s("/source_block_recovery", {})], "expect": P + "FOREIGN_METHOD_EVIDENCE"},
]

document = {
    "description": "Shared adversarial cases for the load-reference-1 readers (Rust result_export tests/load_reference_contract.rs and Python tests/test_load_reference_readers.py). 'dispatch' is the raw-dispatch error (Rust semantic_contract::for_source, Python compatibility._source_contract); 'validator' is validate_load_reference_evidence called directly and defaults to 'dispatch'. A value ending in PHYSICS_EVIDENCE is compared by leading code, the inherited physics-1 detail being language-specific. Language-specific keys (dispatch_rust, dispatch_python, validator_rust, validator_python) replace a shared value where the languages cannot agree, with a note. Transport cases run validate_load_reference_transport_metadata on {schema_version, producer, numerical_quality, formulation_basis, contract_evidence}.",
    "sources": {
        "connected-sparse": "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json",
        "connected-dense": "fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json",
        "pressure-sparse": "fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json",
        "pressure-dense": "fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json",
        "physics-sparse": "fixtures/results/physics_connected_mechanics_sparse.json",
    },
    "cases": cases,
    "table_cases": table_cases,
    "transport_cases": transport_cases,
}
ids = [c["id"] for c in cases] + [c["id"] for c in table_cases] + [c["id"] for c in transport_cases]
assert len(ids) == len(set(ids)), "duplicate case IDs"
out.parent.mkdir(parents=True, exist_ok=True)
out.write_text(json.dumps(document, indent=1) + "\n")
print(json.dumps({"cases": len(cases), "table_cases": len(table_cases), "transport_cases": len(transport_cases), "path": str(out.relative_to(root))}))
