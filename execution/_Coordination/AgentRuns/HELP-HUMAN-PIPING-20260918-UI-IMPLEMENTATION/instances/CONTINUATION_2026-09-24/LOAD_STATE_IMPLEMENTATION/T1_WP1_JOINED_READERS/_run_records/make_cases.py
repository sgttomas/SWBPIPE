"""Author the shared load-reference-source-1 adversarial cases.

Writes core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json,
read by the Rust test tests/load_reference_source_contract.rs and the Python
test tests/test_load_reference_source_readers.py. Run from WORKING_ROOT.

Every expectation is authored from CP2_WIRE_ADDENDUM_2 section 5, the CP4
corrections and the declared check order (J0 identity, J1 pre-pass S1-S13,
J2 receipt R1-R7, J3 projected physics-source-1), never captured from a reader.
The numeric edge cases are reused from load_reference_mutations.json (read
only) with their source switched to a joined envelope.
"""
import json
from pathlib import Path

OUT = Path("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json")
LR_CASES = json.loads(Path("core/reporting/result_export/tests/fixtures/load_reference_mutations.json").read_text())

JOINED = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
JOINED_PROFILE = "resolved_straight_load_state_source_v1"
LR = "openpipestress.result_semantics/0.3.0/load-reference-1"
LR_PROFILE = "resolved_straight_load_state_v1"
PS = "openpipestress.result_semantics/0.3.0/physics-source-1"
PS_PROFILE = "exact_straight_pressure_v2"
EXACT = "retained_source_blocks_exact_v1"
NOT_JOINED = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"
P = "SOURCE_LOAD_REFERENCE_"
INHERITED = P + "JOIN_PHYSICS_SOURCE"
REC = "/contract_evidence/load_reference_states"
EXA = "/contract_evidence/exact_cases"
BODY = "/source_block_recovery/body"
ZERO = "0" * 64

SOURCES = {}
for name in ["n05", "n06", "fields", "mixed", "eigen_motion"]:
    for short, mode in [("sparse", "sparse_interactive"), ("dense", "dense_scrutiny")]:
        SOURCES[f"{name}-{short}"] = f"fixtures/product_preview/load_reference_source/{name}-{mode}.raw.json"
SOURCES["lr-connected-sparse"] = "fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json"
SOURCES["lr-fallback-sparse"] = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json"
SOURCES["lr-fallback-dense"] = "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-dense_scrutiny.raw.json"
SOURCES["ps-n05-sparse"] = "fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json"
JOINED_SOURCES = [name for name in SOURCES if not name.startswith(("lr-", "ps-"))]

FULL = {"op": "reseal", "physical": "joined", "publication": True, "receipt": True}
PUBLICATION = {"op": "reseal", "physical": None, "publication": True, "receipt": True}
RECEIPT = {"op": "reseal", "physical": None, "publication": False, "receipt": True}


def s(path, value):
    return {"op": "set", "path": path, "value": value}


def label(contract, profile):
    return [s("/producer/semantic_contract_id", contract), s("/formulation_basis/profile_id", profile)]


cases = []


def case(case_id, source, ops, dispatch, **extra):
    entry = {"id": case_id, "source": source, "ops": ops, "dispatch": dispatch}
    entry.update(extra)
    cases.append(entry)


# Accepted producer envelopes: joined dispatch; the other two readers refuse them.
for name in JOINED_SOURCES:
    case(f"ACCEPT-{name}", name, [], "accept", lr=P + "FOREIGN_METHOD_EVIDENCE", ps="PHYSICS_SOURCE_RECEIPT_SHAPE")
# SF-1 fallback envelopes stay ordinary load-reference-1.
for name in ["lr-fallback-sparse", "lr-fallback-dense"]:
    case(f"ACCEPT-{name}", name, [], "accept", accept_contract=LR, joined=P + "JOIN_IDENTITY", lr="accept")

# Accepted edits that the wire leaves free: message text, provenance text, an
# integer literal for a binary64 zero (same canonical bytes, same hashes).
case("ACCEPT-not-joined-message-text", "mixed-sparse",
     [{"op": "set_where", "path": "/diagnostics", "match": {"code": NOT_JOINED}, "key": "message", "value": "invented replacement text"}, PUBLICATION],
     "accept")
case("ACCEPT-selected-message-text", "n05-sparse",
     [{"op": "set_where", "path": "/diagnostics", "match": {"code": "SOURCE_BLOCK_RECOVERY_SELECTED"}, "key": "message", "value": "invented replacement text"}, PUBLICATION],
     "accept")
case("ACCEPT-record-provenance-text-resealed", "eigen_motion-sparse", [s(f"{REC}/0/provenance", "invented_replacement_provenance"), FULL], "accept")
case("ACCEPT-integer-zero-literal", "n05-sparse", [s(f"{REC}/0/members/0/thermal_strain", 0)], "accept",
     note="An integer literal 0 for the binary64 zero: the readers compare numbers by value and the checked canonical JSON encodes both as 0, so every hash is unchanged.")

# Identity, profile and table dispatch.
case("ID-reserved-successor", "n05-sparse", [s("/producer/semantic_contract_id", "openpipestress.result_semantics/0.3.0/load-reference-source-2")],
     "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", joined=P + "JOIN_IDENTITY")
case("ID-producer-version", "n05-sparse", [s("/producer/component_version", "0.3.0")], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", joined=P + "JOIN_PUBLICATION_HASH")
case("PROFILE-load-reference", "n05-sparse", [s("/formulation_basis/profile_id", LR_PROFILE)], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", joined=P + "JOIN_IDENTITY")
case("PROFILE-physics-source", "n05-sparse", [s("/formulation_basis/profile_id", PS_PROFILE)], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", joined=P + "JOIN_IDENTITY")
case("PROFILE-empty-limitations", "n05-sparse", [s("/formulation_basis/limitations", [])], "SOURCE_FORMULATION_BASIS_UNSUPPORTED", joined=P + "JOIN_PUBLICATION_HASH")
case("PROFILE-record-joined-profile", "n05-sparse", [s(f"{REC}/0/profile", JOINED_PROFILE)], P + "RECORD_PROFILE")

# Relabels in each direction.
case("RELABEL-joined-as-load-reference", "n05-sparse", label(LR, LR_PROFILE), "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN",
     joined=P + "JOIN_IDENTITY", lr=P + "FOREIGN_METHOD_EVIDENCE")
case("RELABEL-joined-as-load-reference-id-only", "n05-sparse", [s("/producer/semantic_contract_id", LR)], "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN",
     joined=P + "JOIN_IDENTITY")
case("RELABEL-joined-as-load-reference-receipt-removed", "n05-sparse", label(LR, LR_PROFILE) + [{"op": "remove", "path": "/source_block_recovery"}],
     P + "CASE_SHAPE", joined=P + "JOIN_IDENTITY", lr=P + "CASE_SHAPE")
case("RELABEL-joined-as-physics-source", "n05-sparse", label(PS, PS_PROFILE), "SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN",
     joined=P + "JOIN_IDENTITY", ps="PHYSICS_SOURCE_RECEIPT_SHAPE")
case("RELABEL-joined-as-physics-source-resealed", "mixed-sparse", label(PS, PS_PROFILE) + [{"op": "set", "path": f"{BODY}/policy", "value": "PHYSICS-SOURCE-1"}, FULL],
     "SOURCE_LOAD_REFERENCE_EVIDENCE_FORBIDDEN", joined=P + "JOIN_IDENTITY", ps="PHYSICS_SOURCE_PHYSICAL_SHAPE")
case("RELABEL-joined-as-physics-source-id-only", "n05-sparse", [s("/producer/semantic_contract_id", PS)], "SOURCE_FORMULATION_BASIS_UNSUPPORTED",
     joined=P + "JOIN_IDENTITY")
case("RELABEL-load-reference-as-joined", "lr-connected-sparse", label(JOINED, JOINED_PROFILE), P + "JOIN_RECEIPT_REQUIRED", lr="accept")
case("RELABEL-load-reference-as-joined-id-only", "lr-connected-sparse", [s("/producer/semantic_contract_id", JOINED)], "SOURCE_FORMULATION_BASIS_UNSUPPORTED",
     joined=P + "JOIN_IDENTITY")
case("RELABEL-fallback-as-joined", "lr-fallback-dense", label(JOINED, JOINED_PROFILE), P + "JOIN_RECEIPT_REQUIRED", lr="accept")
case("RELABEL-load-reference-as-joined-with-grafted-receipt", "lr-connected-sparse",
     label(JOINED, JOINED_PROFILE) + [{"op": "graft", "path": "/source_block_recovery", "source": "n05-sparse", "from": "/source_block_recovery"}],
     P + "CASE_SHAPE")
case("RELABEL-physics-source-as-joined", "ps-n05-sparse", label(JOINED, JOINED_PROFILE), P + "EVIDENCE_SHAPE", ps="SOURCE_BLOCKS_PRODUCER")
case("RELABEL-physics-source-as-joined-id-only", "ps-n05-sparse", [s("/producer/semantic_contract_id", JOINED)], "SOURCE_FORMULATION_BASIS_UNSUPPORTED",
     joined=P + "JOIN_IDENTITY")

# Namespace.
case("NS-missing-load-reference-states", "n05-sparse", [{"op": "remove", "path": REC}], P + "EVIDENCE_SHAPE")
case("NS-extra-evidence-key", "n05-sparse", [s("/contract_evidence/source_recovery_states", [])], P + "EVIDENCE_SHAPE")
case("NS-connector-nonempty", "n05-sparse", [s("/contract_evidence/connector", [{"invented": True}])], P + "CONNECTOR_UNSUPPORTED")
case("NS-carrier-evidence", "n05-sparse", [s("/carrier_evidence", {})], "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", joined=P + "FOREIGN_METHOD_EVIDENCE")
case("NS-receipt-removed", "n05-sparse", [{"op": "remove", "path": "/source_block_recovery"}], P + "JOIN_RECEIPT_REQUIRED")
case("NS-exact-case-without-recovery-method", "n05-sparse", [{"op": "remove", "path": f"{EXA}/0/recovery_method"}], P + "CASE_SHAPE")
case("NS-exact-case-load-reference-basis-removed", "n05-sparse", [s(f"{EXA}/0/material_basis", "base_material_common_E_nu")], P + "MATERIAL_BASIS")

# Per-case selected / not-joined consistency (ADDENDUM_2 section 5.3).
case("SEL-selected-record-not-joined-recovery", "n05-sparse", [s(f"{REC}/0/source_recovery", {"status": "not_joined", "code": NOT_JOINED})], P + "SOURCE_RECOVERY_SHAPE")
case("SEL-selected-status-joined", "n05-sparse", [s(f"{REC}/0/source_recovery/status", "joined")], P + "SOURCE_RECOVERY")
case("SEL-selected-method-ordinary", "n05-sparse", [s(f"{REC}/0/source_recovery/method", "ordinary_sparse_structural_v1")], P + "SOURCE_RECOVERY")
case("SEL-selected-extra-key", "n05-sparse", [s(f"{REC}/0/source_recovery/code", NOT_JOINED)], P + "SOURCE_RECOVERY_SHAPE")
case("SEL-ordinary-solve-with-selected-recovery", "n05-sparse", [s(f"{REC}/0/solve/recovery_method", "ordinary_sparse_structural_v1")], P + "SOURCE_RECOVERY_SHAPE")
case("SEL-ordinary-solve-wrong-mode", "n05-sparse", [s(f"{REC}/0/solve/recovery_method", "ordinary_dense_structural_v1")], P + "SOLVE")
case("SEL-record-ordinary-exact-selected", "n05-sparse",
     [s(f"{REC}/0/solve/recovery_method", "ordinary_sparse_structural_v1"), s(f"{REC}/0/source_recovery", {"status": "not_joined", "code": NOT_JOINED})],
     P + "JOIN_RECOVERY_METHOD")
case("SEL-ordinary-case-claims-selected", "mixed-sparse",
     [s(f"{REC}/1/solve/recovery_method", EXACT), s(f"{REC}/1/source_recovery", {"status": "selected", "method": EXACT})],
     P + "JOIN_RECOVERY_METHOD")
case("SEL-ordinary-case-claims-selected-with-exact-case", "mixed-sparse",
     [s(f"{REC}/1/solve/recovery_method", EXACT), s(f"{REC}/1/source_recovery", {"status": "selected", "method": EXACT}), s(f"{EXA}/1/recovery_method", EXACT)],
     P + "JOIN_SELECTED_DIAGNOSTIC")
case("SEL-no-case-selected", "n05-sparse",
     [s(f"{REC}/0/solve/recovery_method", "ordinary_sparse_structural_v1"), s(f"{REC}/0/source_recovery", {"status": "not_joined", "code": NOT_JOINED}),
      s(f"{EXA}/0/recovery_method", "ordinary_sparse_structural_v1")],
     P + "JOIN_SELECTION_REQUIRED")
case("SEL-selected-other-mode", "n05-sparse", [s(f"{REC}/0/solve/requested_mode", "dense_scrutiny"), FULL], P + "JOIN_REQUESTED_MODE")
case("SEL-mixed-record-modes", "mixed-sparse", [s(f"{REC}/0/solve/requested_mode", "dense_scrutiny")], P + "SOLVE_CONSISTENCY")
case("SEL-unknown-mode", "n05-sparse", [s(f"{REC}/0/solve/requested_mode", "exact_mode")], P + "SOLVE")

# Diagnostics.
SELECTED_ID = "diagnostic:source-recovery:case:selected"
case("DIAG-selected-missing", "n05-sparse", [{"op": "remove_where", "path": "/diagnostics", "match": {"code": "SOURCE_BLOCK_RECOVERY_SELECTED"}}], P + "JOIN_SELECTED_DIAGNOSTIC")
case("DIAG-selected-duplicate", "n05-sparse",
     [{"op": "append_copy_where", "path": "/diagnostics", "match": {"id": SELECTED_ID}, "set": {"id": "diagnostic:source-recovery:case:selected:copy"}}],
     P + "JOIN_SELECTED_DIAGNOSTIC")
case("DIAG-selected-warning", "n05-sparse", [{"op": "set_where", "path": "/diagnostics", "match": {"id": SELECTED_ID}, "key": "severity", "value": "warning"}], P + "JOIN_SELECTED_DIAGNOSTIC")
case("DIAG-selected-affected-refs", "n05-sparse", [{"op": "set_where", "path": "/diagnostics", "match": {"id": SELECTED_ID}, "key": "affected_refs", "value": ["case", "case"]}],
     P + "JOIN_SELECTED_DIAGNOSTIC")
case("DIAG-selected-wrong-id", "n05-sparse", [{"op": "set_where", "path": "/diagnostics", "match": {"id": SELECTED_ID}, "key": "id", "value": "diagnostic:source-recovery:case-selected"}],
     P + "JOIN_SELECTED_DIAGNOSTIC")
case("DIAG-not-joined-on-selected-case", "n05-sparse",
     [{"op": "append", "path": "/diagnostics", "value": {"id": "diagnostic:load-state:case:source-recovery-not-joined", "code": NOT_JOINED, "severity": "info",
                                                        "message": "invented", "source": "core/product_physics", "affected_refs": ["case"]}}],
     P + "NOT_JOINED_DIAGNOSTIC")
case("DIAG-not-joined-missing", "mixed-sparse", [{"op": "remove_where", "path": "/diagnostics", "match": {"code": NOT_JOINED}}], P + "NOT_JOINED_DIAGNOSTIC")
case("DIAG-not-joined-warning", "mixed-dense", [{"op": "set_where", "path": "/diagnostics", "match": {"code": NOT_JOINED}, "key": "severity", "value": "warning"}], P + "NOT_JOINED_DIAGNOSTIC")

# Review A note N-2: no UNAVAILABLE diagnostic may name a selected case (the
# producer never emits one). A resealed publication reaches S13 in either case;
# the control names the ordinary case of the mixed witness and stays accepted.
UNAVAILABLE_DIAG = {"id": "diagnostic:source-recovery:case:unavailable", "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE", "severity": "info",
                    "message": "invented", "source": "core/product_physics", "affected_refs": ["case"]}
case("DIAG-unavailable-on-selected", "n05-sparse", [{"op": "append", "path": "/diagnostics", "value": UNAVAILABLE_DIAG}], P + "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC")
case("DIAG-unavailable-on-selected-resealed", "eigen_motion-dense",
     [{"op": "append", "path": "/diagnostics", "value": {**UNAVAILABLE_DIAG, "id": "diagnostic:source-recovery:case:join:unavailable", "affected_refs": ["case:join"]}}, PUBLICATION],
     P + "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC")
case("DIAG-unavailable-among-refs-of-selected-resealed", "mixed-sparse",
     [{"op": "append", "path": "/diagnostics", "value": {**UNAVAILABLE_DIAG, "affected_refs": ["case:ordinary-pressure", "case"]}}, PUBLICATION],
     P + "JOIN_SELECTED_UNAVAILABLE_DIAGNOSTIC")
case("ACCEPT-unavailable-on-ordinary-case-resealed", "mixed-sparse",
     [{"op": "append", "path": "/diagnostics", "value": {**UNAVAILABLE_DIAG, "id": "diagnostic:source-recovery:case:ordinary-pressure:unavailable", "affected_refs": ["case:ordinary-pressure"]}}, PUBLICATION],
     "accept", lr=P + "FOREIGN_METHOD_EVIDENCE")

# Review A note N-1: an integral literal above 2^53-1 in the hashed publication
# reaches the checked canonical hash at R4 and is refused in both languages; the
# inherited checked-JSON detail differs (Rust appends the number).
case("NUM-unsafe-integer-in-publication", "n05-sparse", [s("/results/0/value", 2 ** 60)], None,
     dispatch_rust="CHECKED-JSON-UNSAFE-INTEGER: 1152921504606846976", dispatch_python="CHECKED-JSON-UNSAFE-INTEGER",
     joined_rust="CHECKED-JSON-UNSAFE-INTEGER: 1152921504606846976", joined_python="CHECKED-JSON-UNSAFE-INTEGER",
     note="The checked canonical JSON profile refuses integral magnitudes above 2^53-1 at the R4 publication hash; the inherited detail differs: Rust's canonical_json error carries the literal and Python's adapter does not. Both refuse.")

# Receipt policy, shape and hashes.
case("RCPT-policy-physics-source", "n05-sparse", [s(f"{BODY}/policy", "PHYSICS-SOURCE-1")], P + "JOIN_RECEIPT_POLICY")
case("RCPT-policy-source-blocks", "mixed-sparse", [s(f"{BODY}/policy", "SOURCE-BLOCKS-1")], P + "JOIN_RECEIPT_POLICY")
case("RCPT-policy-missing", "n05-sparse", [{"op": "remove", "path": f"{BODY}/policy"}], P + "JOIN_RECEIPT_POLICY")
case("RCPT-policy-physics-source-resealed", "n05-sparse", [s(f"{BODY}/policy", "PHYSICS-SOURCE-1"), RECEIPT], P + "JOIN_RECEIPT_POLICY")
case("RCPT-extra-body-key", "n05-sparse", [s(f"{BODY}/invented", 1), RECEIPT], P + "JOIN_RECEIPT_SHAPE")
case("RCPT-status-partial", "n05-sparse", [s(f"{BODY}/status", "partial"), RECEIPT], P + "JOIN_RECEIPT_SHAPE")
case("RCPT-receipt-hash", "n05-sparse", [s("/source_block_recovery/receipt_sha256", ZERO)], P + "JOIN_RECEIPT_HASH")
case("RCPT-body-changed", "eigen_motion-dense", [s(f"{BODY}/invocation_work/publication_charged", 1)], P + "JOIN_RECEIPT_HASH")
case("PUB-row-value", "n05-sparse", [s("/results/0/value", 1.0)], P + "JOIN_PUBLICATION_HASH")
case("PUB-row-value-resealed", "n05-sparse", [s("/results/0/value", 1.0), PUBLICATION], INHERITED)
case("PUB-evidence-changed-receipt-only", "n05-sparse", [s(f"{REC}/0/provenance", "invented_replacement_provenance"), RECEIPT], P + "JOIN_PUBLICATION_HASH")
case("HASH-physical-zero", "n05-sparse", [s(f"{BODY}/cases/0/physical_evidence_sha256", ZERO), RECEIPT], P + "JOIN_PHYSICAL_CASE_HASH")
case("HASH-physical-physics-source-domain", "mixed-sparse", [{"op": "reseal", "physical": "physics", "publication": False, "receipt": True}], P + "JOIN_PHYSICAL_CASE_HASH")
case("HASH-physical-without-record", "eigen_motion-sparse", [{"op": "reseal", "physical": "joined_without_record", "publication": False, "receipt": True}],
     P + "JOIN_PHYSICAL_CASE_HASH")
case("HASH-record-changed-physical-stale", "eigen_motion-sparse", [s(f"{REC}/0/provenance", "invented_replacement_provenance"), PUBLICATION],
     P + "JOIN_PHYSICAL_CASE_HASH")
case("HASH-ordinary-record-changed-physical-stale", "mixed-sparse", [s(f"{REC}/1/provenance", "invented_replacement_provenance"), PUBLICATION],
     P + "JOIN_PHYSICAL_CASE_HASH")
case("HASH-exact-case-changed-physical-stale", "mixed-dense",
     [s(f"{EXA}/1/pipe_materials/0/provenance", "invented_replacement_provenance"),
      s("/contract_evidence/pressure/0/materials/0/provenance", "invented_replacement_provenance"), PUBLICATION],
     P + "JOIN_PHYSICAL_CASE_HASH")
case("RCPT-requested-mode-mismatch", "mixed-sparse",
     [s(f"{BODY}/cases/1/requested_mode", "dense_scrutiny"), s(f"{BODY}/cases/1/ordinary_attempt/requested_mode", "dense_scrutiny"), RECEIPT],
     P + "JOIN_REQUESTED_MODE")
case("RCPT-selected-method-ordinary", "mixed-sparse", [s(f"{BODY}/cases/0/selected_method", "ordinary_sparse_structural_v1"), RECEIPT], P + "JOIN_RECOVERY_METHOD")

# Missing or extra record and order.
case("REC-missing", "n05-sparse", [{"op": "remove", "path": f"{REC}/0"}], P + "CASE_COVERAGE")
case("REC-missing-ordinary", "mixed-sparse", [{"op": "remove", "path": f"{REC}/1"}], P + "CASE_COVERAGE")
case("REC-extra", "mixed-sparse", [{"op": "copy_append", "path": REC, "from": f"{REC}/0"}], P + "RECORD_DUPLICATE")
case("REC-extra-renamed", "n05-sparse", [{"op": "copy_append", "path": REC, "from": f"{REC}/0"}, s(f"{REC}/1/load_case_id", "case:invented")],
     P + "CASE_COVERAGE")
case("REC-order-swap", "mixed-sparse", [{"op": "swap", "path": f"{REC}/0", "with": f"{REC}/1"}], P + "JOIN_PUBLICATION_HASH")
case("REC-order-swap-resealed", "mixed-dense", [{"op": "swap", "path": f"{REC}/0", "with": f"{REC}/1"}, PUBLICATION], P + "JOIN_CASE_ORDER")
case("EXACT-order-swap-resealed", "mixed-sparse", [{"op": "swap", "path": f"{EXA}/0", "with": f"{EXA}/1"}, PUBLICATION], P + "JOIN_CASE_ORDER")
case("RCPT-case-order-swap-resealed", "mixed-sparse", [{"op": "swap", "path": f"{BODY}/cases/0", "with": f"{BODY}/cases/1"}, RECEIPT], P + "JOIN_CASE_ORDER")

# Resolved-state bindings reached before the receipt.
case("BIND-exact-E-forged", "n05-sparse", [s(f"{EXA}/0/pipe_materials/0/E_pa", 199999999999.0)], P + "MEMBER_MATERIAL_BINDING")
case("BIND-member-eigen-contribution", "eigen_motion-sparse", [{"op": "set_where", "path": f"{REC}/0/contributions", "match": {"source_id": "member_state:member"}, "key": "value", "value": 0.0001}],
     P + "MEMBER_CONTRIBUTION_BINDING")
case("BIND-support-motion", "eigen_motion-dense", [s(f"{REC}/0/support_components/0/prescribed_value", 0.002)], P + "SUPPORT_CONTRIBUTION_BINDING")
case("BIND-region-temperature-basis", "mixed-sparse", [s("/contract_evidence/pressure/0/materials/0/temperature_basis", {"selection": "base_material"})],
     P + "REGION_TEMPERATURE_BASIS")
case("BIND-resolved-eigenstrain", "eigen_motion-sparse", [s(f"{EXA}/0/pipe_materials/0/resolved_eigenstrain", 0.0001)], P + "MEMBER_MATERIAL_BINDING")

# Inherited physics-source-1 checks on the projection (resealed so that the
# receipt admits the change).
case("PS-extremum-value-resealed", "n05-sparse", [s(f"{EXA}/0/pipe_stress_extrema/0/value_pa", 1.0), FULL], INHERITED)
case("PS-selected-over-pressure-resealed", "mixed-sparse",
     [s(f"{EXA}/1/recovery_method", EXACT), s(f"{REC}/1/solve/recovery_method", EXACT), s(f"{REC}/1/source_recovery", {"status": "selected", "method": EXACT}),
      {"op": "set_where", "path": "/diagnostics", "match": {"code": NOT_JOINED}, "key": "code", "value": "SOURCE_BLOCK_RECOVERY_SELECTED"},
      {"op": "set_where", "path": "/diagnostics", "match": {"code": "SOURCE_BLOCK_RECOVERY_SELECTED", "affected_refs": ["case:ordinary-pressure"]}, "key": "id",
       "value": "diagnostic:source-recovery:case:ordinary-pressure:selected"},
      FULL],
     P + "JOIN_RECOVERY_METHOD")

# Numeric edge cases reused from load_reference_mutations.json.
REUSE_SOURCE = {"connected-sparse": "n05-sparse", "pressure-dense": "eigen_motion-dense", "pressure-sparse": "mixed-sparse"}
for original in LR_CASES["cases"]:
    cid = original["id"]
    reuse = cid.startswith("N3-") or cid in {"NONFINITE-member-E", "N2-integer-temperature-negative", "N2-integer-overflow-positive", "N2-integer-first-overflow", "NONFINITE-region-pressure"}
    if not reuse:
        continue
    entry = {key.replace("validator", "joined"): value for key, value in original.items() if key not in {"id", "source"}}
    entry = {"id": f"NUM-{cid}", "source": REUSE_SOURCE[original["source"]], **entry, "reused_from": cid}
    cases.append(entry)


TABLE_CASES = [
    {"id": "TABLEBYTES-pinned", "find": None, "replace": None, "expect": "accept"},
    {"id": "TABLEBYTES-id-edit", "find": "0.3.0/load-reference-source-1\"", "replace": "0.3.0/load-reference-source-2\"", "expect": P + "SOURCE_TABLE_HASH"},
    {"id": "TABLEBYTES-profile-edit", "find": "\"resolved_straight_load_state_source_v1\"", "replace": "\"resolved_straight_load_state_v1\"", "expect": P + "SOURCE_TABLE_HASH"},
    {"id": "TABLEBYTES-policy-edit", "find": "\"LOAD-REFERENCE-SOURCE-1\"", "replace": "\"PHYSICS-SOURCE-1\"", "expect": P + "SOURCE_TABLE_HASH"},
    {"id": "TABLEBYTES-physics-source-table", "file": "fixtures/results/semantic_contract_v0_3_physics_source_1.json", "find": None, "replace": None, "expect": P + "SOURCE_TABLE_HASH"},
    {"id": "TABLEBYTES-load-reference-table", "file": "fixtures/results/semantic_contract_v0_3_load_reference_1.json", "find": None, "replace": None, "expect": P + "SOURCE_TABLE_HASH"},
]

TRANSPORT = []


def transport(case_id, source, ops, expect):
    TRANSPORT.append({"id": case_id, "source": source, "ops": ops, "expect": expect})


for name in ["n05-sparse", "mixed-dense", "eigen_motion-sparse", "fields-dense"]:
    transport(f"TRANSPORT-ACCEPT-{name}", name, [], "accept")
transport("TRANSPORT-policy", "n05-sparse", [s(f"{BODY}/policy", "PHYSICS-SOURCE-1")], P + "JOIN_RECEIPT_POLICY")
transport("TRANSPORT-receipt-hash", "mixed-sparse", [s("/source_block_recovery/receipt_sha256", ZERO)], P + "JOIN_RECEIPT_HASH")
transport("TRANSPORT-physical-hash", "mixed-sparse", [s(f"{BODY}/cases/1/physical_evidence_sha256", ZERO), RECEIPT], P + "JOIN_PHYSICAL_CASE_HASH")
transport("TRANSPORT-record-missing", "mixed-sparse", [{"op": "remove", "path": f"{REC}/1"}], P + "CASE_COVERAGE")
transport("TRANSPORT-selected-inconsistent", "n05-sparse", [s(f"{REC}/0/source_recovery", {"status": "not_joined", "code": NOT_JOINED})], P + "SOURCE_RECOVERY_SHAPE")
transport("TRANSPORT-no-selection", "n05-sparse",
          [s(f"{REC}/0/solve/recovery_method", "ordinary_sparse_structural_v1"), s(f"{REC}/0/source_recovery", {"status": "not_joined", "code": NOT_JOINED}),
           s(f"{EXA}/0/recovery_method", "ordinary_sparse_structural_v1")],
          P + "JOIN_SELECTION_REQUIRED")
transport("TRANSPORT-load-reference-relabel", "lr-connected-sparse", label(JOINED, JOINED_PROFILE), P + "JOIN_RECEIPT_REQUIRED")
# Transport has no raw rows, so a resealed extremum value is not detectable
# there (as for physics-source-1 transport); a resealed endpoint link is.
transport("TRANSPORT-extremum-value-resealed", "n05-sparse", [s(f"{EXA}/0/pipe_stress_extrema/0/value_pa", 1.0), FULL], "accept")
transport("TRANSPORT-endpoint-link-resealed", "n05-sparse", [s(f"{EXA}/0/pipe_stress_extrema/0/endpoints/0/station_fraction", 0.25), FULL], INHERITED)
transport("TRANSPORT-no-diagnostic-binding", "n05-sparse", [{"op": "remove_where", "path": "/diagnostics", "match": {"code": "SOURCE_BLOCK_RECOVERY_SELECTED"}}], "accept")

DESCRIPTION = (
    "Shared adversarial cases for the load-reference-source-1 readers (Rust result_export tests/load_reference_source_contract.rs and Python "
    "tests/test_load_reference_source_readers.py). Authored by LSI/T1_WP1_JOINED_READERS/_run_records/make_cases.py from the wire and the check order, "
    "not captured from either reader. 'dispatch' is the raw-dispatch outcome (Rust semantic_contract::for_source, Python compatibility._source_contract); "
    "an accepted case selects 'accept_contract' (default load-reference-source-1). 'joined' is validate_load_reference_source_evidence called directly and "
    "defaults to 'dispatch'. 'lr' (validate_load_reference_evidence) and 'ps' (physics-source-1 validator without an invocation) are checked where present. "
    "A value equal to SOURCE_LOAD_REFERENCE_JOIN_PHYSICS_SOURCE is compared by leading code, the inherited physics-source-1 detail being recorded "
    "separately. Keys suffixed _rust or _python replace a shared value where the languages cannot agree, with a note. Ops are those of "
    "load_reference_mutations.json plus 'reseal': in order it recomputes each case's physical_evidence_sha256 ('joined' over "
    "{exact_case, pressure, load_reference_state} with domain load_reference_source_case_evidence_v1, 'joined_without_record' without the record, "
    "'physics' over {exact_case, pressure} with domain physics_source_case_evidence_v1, null to keep), then the publication_sha256, then the receipt_sha256, "
    "so that a check after the receipt can be reached. Transport cases run validate_load_reference_source_transport_metadata on "
    "{schema_version, producer, numerical_quality, formulation_basis, contract_evidence, source_block_recovery, carrier_evidence}. Table cases run the "
    "pinned-table verifier on the joined table bytes (or on 'file')."
)
data = {"description": DESCRIPTION, "sources": SOURCES, "cases": cases, "table_cases": TABLE_CASES, "transport_cases": TRANSPORT}
ids = [c["id"] for c in cases + TABLE_CASES + TRANSPORT]
assert len(ids) == len(set(ids)), "duplicate case id"
OUT.write_text(json.dumps(data, indent=1, ensure_ascii=False) + "\n")
print(OUT, len(cases), len(TABLE_CASES), len(TRANSPORT))
