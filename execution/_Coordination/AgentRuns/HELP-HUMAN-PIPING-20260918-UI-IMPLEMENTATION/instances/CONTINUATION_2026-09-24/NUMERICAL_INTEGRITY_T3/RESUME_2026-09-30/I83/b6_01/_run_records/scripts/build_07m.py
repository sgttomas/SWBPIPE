"""B6 (I83): snapshot 07m from 07l. Writes the corpus with the file's own serialization (json indent 2 + newline,
which reproduces 07l byte for byte). Changes: (1) mutation 277's TS expectation, aligned (PLAN decision 11; the one
in-place value change); (2) eight G7 mutations appended after index 285, so no existing slice moves.
Usage: build_07m.py <corpus path>"""
import hashlib, json, sys
path = sys.argv[1]
raw = open(path, "rb").read()
assert hashlib.sha256(raw).hexdigest() == "5ac13296c69745ae837e41b93c42e4daa0c4100dc863229f8e6a4cdbe2692ccd", "not 07l"
c = json.loads(raw)
assert (json.dumps(c, indent=2) + "\n").encode() == raw
assert (len(c["cases"]), len(c["mutations"]), len(c["must_pass"])) == (17, 286, 28)
G7 = lambda code: {"gate": "G7", "code": code}
m277 = c["mutations"][277]
assert m277["id"] == "g7_not_required_quality_enum_invalid"
assert m277["expected_by_reader"] == {"python": G7("SOURCE_NUMERICAL_CASE_INVALID"), "typescript": G7("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"), "rust": G7("SOURCE_NUMERICAL_CASE_INVALID")}
m277["expected_by_reader"]["typescript"] = G7("SOURCE_NUMERICAL_CASE_INVALID")
O, T = "ordinary_prepared_synthetic", "two_case_preparation_failure_synthetic"
NQ = lambda i, field: ["numerical_quality", "cases", i, field]
def entry(name, base, path, value, code):
    return {"id": name, "base": base, "edits": [{"path": path, "op": "set", "value": value}], "rehash": "all", "expected": G7(code)}
appended = [
    # The N-3 class at its full width: any case status, any part of the base case rule.
    entry("g7_selected_quality_enum_invalid", O, NQ(0, "structural_status"), "mechanism_detected", "SOURCE_NUMERICAL_CASE_INVALID"),
    entry("g7_unavailable_quality_enum_invalid", T, NQ(1, "model_matrix_fidelity"), "reduced", "SOURCE_NUMERICAL_CASE_INVALID"),
    entry("g7_quality_case_evidence_ref_empty", O, NQ(0, "evidence_refs"), [""], "SOURCE_NUMERICAL_CASE_INVALID"),
    entry("g7_quality_case_extra_member", O, NQ(0, "extra"), 1, "SOURCE_NUMERICAL_CASE_INVALID"),
    # The sibling header classes, each the base readers' own header code.
    entry("g7_quality_status_invalid", O, ["numerical_quality", "status"], "estimated", "SOURCE_NUMERICAL_QUALITY_INVALID"),
    entry("g7_formulation_limitations_empty", O, ["formulation_basis", "limitations"], [], "SOURCE_FORMULATION_BASIS_UNSUPPORTED"),
    entry("g7_contract_evidence_null", O, ["contract_evidence"], None, "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED"),
    entry("g7_source_block_recovery_present", O, ["source_block_recovery"], None, "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"),
]
ids = {e["id"] for e in c["mutations"] + c["must_pass"]}
assert not ids & {e["id"] for e in appended}
c["mutations"].extend(appended)
out = (json.dumps(c, indent=2) + "\n").encode()
open(path, "wb").write(out)
print(hashlib.sha256(out).hexdigest(), len(out), (len(c["cases"]), len(c["mutations"]), len(c["must_pass"])))
