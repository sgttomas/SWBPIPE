#!/usr/bin/env python3
"""I66 U6 repairs: rewrite P/fixtures/results/retained_precision_carrier_cases.json
as format v2. The 14 v1 cases are kept byte-for-byte in meaning and order; v2 adds
raw fixtures, the F-5 guard cases (RV88 U6b S-1, U6a S-1) and the
declared_differences section (RR "RV88 on U6a, U6c, U6b (and U6d)...")."""
import hashlib, json, sys
from pathlib import Path

P = Path(sys.argv[1])
PATH = P / "fixtures/results/retained_precision_carrier_cases.json"
v1 = json.loads(PATH.read_text())
assert v1["format"] == "I66-U6-CARRIER-CASES-v1" and len(v1["cases"]) == 14

RR = 'ROOT_RULINGS_V1 "RV88 on U6a, U6c, U6b (and U6d): all PASS; repair rounds and the shared declared-difference cases"'


def sha(rel):
    return hashlib.sha256((P / rel).read_bytes()).hexdigest()


fixtures = {k: dict(v, shape="milestone") for k, v in v1["fixtures"].items()}
LEGACY = "fixtures/product_preview/invented_mechanics_result.json"
PREVIEW = "fixtures/results/preview_physics_invented_sparse.json"
fixtures["legacy_preview_0_1"] = {"path": LEGACY, "sha256": sha(LEGACY), "shape": "raw"}
fixtures["preview_physics_1_invented_sparse"] = {"path": PREVIEW, "sha256": sha(PREVIEW), "shape": "raw"}
last = {fid: len(json.loads((P / f["path"]).read_text())["results"]) - 1 for fid, f in fixtures.items() if f["shape"] == "raw"}

FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
cases = list(v1["cases"])
# F-5 guard forms (RV88 U6b S-1; RV88 U6a S-1): every language refuses them.
for fid, label in (("legacy_preview_0_1", "legacy_0_1"), ("preview_physics_1_invented_sparse", "preview_physics_1")):
    cases.append({"id": f"{label}:token_last_row_only", "fixture": fid, "invocation": None, "requested": [],
                  "edits": [{"target": "source", "path": ["results", last[fid], "recovery_method"], "op": "set", "value": "contribution_preserving_multiprecision_v1"}],
                  "expected_standing": "unsupported", "expected_dispatch": FORBIDDEN})
    for name, value in (("receipt_member_empty", {}), ("receipt_member_null", None)):
        cases.append({"id": f"{label}:{name}", "fixture": fid, "invocation": None, "requested": [],
                      "edits": [{"target": "source", "path": ["retained_precision"], "op": "set", "value": value}],
                      "expected_standing": "unsupported", "expected_dispatch": FORBIDDEN})

MILESTONES = ["milestone_sparse_interactive", "milestone_dense_scrutiny"]
EDITED = [{"target": "source", "path": ["results", 0, "value"], "op": "set", "value": 12345.0}]
NOT_COVERED_EVERY_ROW = "every_row:RULE_QUANTITY_NOT_COVERED"
declared = [
    {"id": "I67-F1:unregistered_invalid_statement", "kind": "language",
     "ruling": f"{RR}; RV88 u6d_01 N-1; RV88 u6b_01 section 3; I67 u6d RETURN F1; PLAN section 3 rule 1",
     "description": "An edited (invalid) successor statement with no invocation. Rust and Python run the reader and read unsupported; TS standing is registration-based and reads needs_recompute until a reader runs. No language makes it eligible or binds it.",
     "subject": "standing", "fixtures": MILESTONES, "invocation": None, "requested": "invocation", "edits": EDITED,
     "expected": {"rust": {"standing": "unsupported"}, "python": {"standing": "unsupported"},
                  "typescript": {"standing": "needs_recompute", "finding": "RETAINED_PRECISION_VALIDATION_REQUIRED"}}},
    {"id": "I67-F2:display_only_binding_precheck", "kind": "language",
     "ruling": f"{RR}; RV88 u6d_01 N-1, N-2; I67 u6d RETURN F2",
     "description": "Binding of a valid successor statement with no invocation. Rust and Python refuse each row by its validated class (absolute_verified: RULE_QUANTITY_BELOW_VERIFIED_FLOOR; not_covered: RULE_QUANTITY_NOT_COVERED; other classes bind). TS's display-only precheck has no registered classes and refuses every row.",
     "subject": "binding", "fixtures": MILESTONES, "invocation": None, "requested": "invocation", "edits": [],
     "expected": {"rust": {"binding": "by_validated_class"}, "python": {"binding": "by_validated_class"},
                  "typescript": {"binding": NOT_COVERED_EVERY_ROW, "notice": "N_RP_UNVALIDATED"}}},
    {"id": "F-U6b-2:python_refuses_transport", "kind": "language",
     "ruling": f'{RR}; ROOT_RULINGS_V1 "U6b (Python carriers) verified and committed; the pin and findings ruled" (tracked to wider F2a); RV88 u6b_01 N-2; I66 u6b RETURN F-U6b-2',
     "description": "The header-only (transport) dispatch of the unedited successor: Rust for_source_metadata, Python _source_contract(check_receipt=False), TS the header route. Rust and TS run the transport checks; Python has no transport validator and refuses (fail-closed). Transport is never eligible in any language.",
     "subject": "transport", "fixtures": MILESTONES, "invocation": None, "requested": "invocation", "edits": [],
     "expected": {"rust": {"transport": "ok"}, "python": {"transport": "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"},
                  "typescript": {"transport": "ok"}}},
    {"id": "F5:refused_statement_binding", "kind": "semantics",
     "ruling": f'{RR}; ROOT_RULINGS_V1 "U6a verified and committed; fan-out" (F5 accepted fail-closed, subject to RV88); RV88 u6a_01 section 5, N-1',
     "description": "Binding of a refused (edited) successor statement: no row has a validated class, so every row is refused with RULE_QUANTITY_NOT_COVERED in every language. Declared because the code stretches D2 4.9.9's not_covered meaning to a statement with no verified class.",
     "subject": "binding", "fixtures": MILESTONES, "invocation": None, "requested": "invocation", "edits": EDITED,
     "expected": {"rust": {"binding": NOT_COVERED_EVERY_ROW}, "python": {"binding": NOT_COVERED_EVERY_ROW},
                  "typescript": {"binding": NOT_COVERED_EVERY_ROW, "notice": "N_RP_UNVALIDATED"}}},
]

note = (v1["note"] + " v2 (I66 U6 repairs): fixtures carry 'shape': 'milestone' ({id, invocation, source}) or 'raw' "
        "(the file is the source; its cases have no invocation). The F-5 guard cases (a W1 token on a non-first row only; "
        "a receipt member that is {} or null) on a legacy 0.1.0 source and on a preview-physics-1 source are refused in every language. "
        "'declared_differences' lists the only ruled differences between the languages' carriers (and F5's shared semantics), "
        "each with its ruling and one expectation per language; any other difference is a defect. Their subjects: 'standing' "
        "(as the cases), 'binding' (every row of the edited source: 'by_validated_class' or 'every_row:<code>') and 'transport' "
        "(the header-only dispatch: 'ok' or the first error code).")
v2 = {"format": "I66-U6-CARRIER-CASES-v2", "note": note, "fixtures": fixtures, "cases": cases, "declared_differences": declared}
PATH.write_text(json.dumps(v2, indent=2) + "\n")
print(len(cases), "cases;", len(declared), "declared differences;", hashlib.sha256(PATH.read_bytes()).hexdigest())
