#!/usr/bin/env python3
"""I66 post-U6f round: rewrite P/fixtures/results/retained_precision_carrier_cases.json
as format v3 (RR "RV92 (U6f) on the whole of U6: PASS; the post-U6f repair round;
U7 preconditions"). The 20 shared cases are kept unchanged. declared_differences
becomes a list of entries, each with one or more forms; each form names its
fixtures, invocation, requested refs, edits, subject and one expectation per
language. Adds a top-level 'scope' sentence (N-4), widens I67-F2 (N-3, with the
summary subject) and adds the fifth entry (N-2, with N-5)."""
import hashlib, json, sys
from pathlib import Path

P = Path(sys.argv[1])
PATH = P / "fixtures/results/retained_precision_carrier_cases.json"
v2 = json.loads(PATH.read_text())
assert v2["format"] == "I66-U6-CARRIER-CASES-v2" and len(v2["cases"]) == 20 and len(v2["declared_differences"]) == 4
old = {entry["id"]: entry for entry in v2["declared_differences"]}

RR92 = 'ROOT_RULINGS_V1 "RV92 (U6f) on the whole of U6: PASS; the post-U6f repair round; U7 preconditions"'
TS92 = "TS expectation from RV92's probes (R/REVIEW_RV92/u6f_01 section 1)"
SPARSE, DENSE = "milestone_sparse_interactive", "milestone_dense_scrutiny"
MILESTONES = [SPARSE, DENSE]
LEGACY, PREVIEW, BLOCKS = "legacy_preview_0_1", "preview_physics_1_invented_sparse", "source_blocks_n05_sparse"
TOKEN = "contribution_preserving_multiprecision_v1"
NOT_COVERED_EVERY_ROW = "every_row:RULE_QUANTITY_NOT_COVERED"

fixtures = dict(v2["fixtures"])
BLOCKS_PATH = "fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json"
fixtures[BLOCKS] = {"path": BLOCKS_PATH, "sha256": hashlib.sha256((P / BLOCKS_PATH).read_bytes()).hexdigest(), "shape": "raw"}


def form(label, fixture_ids, subject, expected, invocation=None, requested="invocation", edits=()):
    assert set(expected) == {"rust", "python", "typescript"}
    return {"label": label, "fixtures": list(fixture_ids), "invocation": invocation, "requested": requested,
            "edits": list(edits), "subject": subject, "expected": expected}


def carried(entry_id, label, **changes):
    """A v2 entry as one form, unchanged."""
    e = old[entry_id]
    return form(label, e["fixtures"], e["subject"], e["expected"], e["invocation"], e["requested"], e["edits"], **changes)


def entry(entry_id, kind, ruling, description, forms):
    return {"id": entry_id, "kind": kind, "ruling": ruling, "description": description, "forms": forms}


EDITED = old["I67-F1:unregistered_invalid_statement"]["edits"]
CLASS_BINDING = {"binding": "by_validated_class"}
TS_UNVALIDATED = {"binding": NOT_COVERED_EVERY_ROW, "notice": "N_RP_UNVALIDATED"}
f2_refused = []
for fid, other in ((SPARSE, "dense_scrutiny"), (DENSE, "sparse_interactive")):
    mode = fid.split("_", 1)[1]
    f2_refused.append(form(f"refused_registration:foreign_mode:{mode}", [fid], "binding",
                           {"rust": CLASS_BINDING, "python": CLASS_BINDING, "typescript": TS_UNVALIDATED},
                           invocation="fixture", edits=[{"target": "invocation", "path": ["solver_mode"], "op": "set", "value": other}]))
f2_refused.append(form("refused_registration:edited_model", MILESTONES, "binding",
                       {"rust": CLASS_BINDING, "python": CLASS_BINDING, "typescript": TS_UNVALIDATED},
                       invocation="fixture", edits=[{"target": "invocation", "path": ["request", "model", "load_cases", 0, "id"], "op": "set", "value": "case-edited"}]))
f2_refused.append(form("refused_registration:empty_invocation", MILESTONES, "binding",
                       {"rust": CLASS_BINDING, "python": CLASS_BINDING, "typescript": TS_UNVALIDATED}, invocation={}))

declared = [
    entry("I67-F1:unregistered_invalid_statement", "language", old["I67-F1:unregistered_invalid_statement"]["ruling"],
          old["I67-F1:unregistered_invalid_statement"]["description"],
          [carried("I67-F1:unregistered_invalid_statement", "edited_row:no_invocation")]),
    entry("I67-F2:display_only_binding_precheck", "language",
          old["I67-F2:display_only_binding_precheck"]["ruling"] + f"; widened by {RR92} (RV92 u6f_01 N-3); {TS92}",
          "A valid successor statement with no valid registration in TS: none (no invocation), or one the reader refused "
          "(a foreign solver mode, an edited invocation model, an empty invocation). Rust and Python bind each row by its "
          "validated class, because binding validates without the invocation (absolute_verified: RULE_QUANTITY_BELOW_VERIFIED_FLOOR; "
          "not_covered: RULE_QUANTITY_NOT_COVERED; other classes bind), and with no invocation they summarise the validated classes. "
          "TS's display-only precheck and summary read registered classes only: it refuses every row (notice N_RP_UNVALIDATED) and "
          "its summary is empty. With a refused invocation every language's summary is empty, which is not a difference. Fail-closed; "
          "TS's rule check needs eligible standing, and Tauri's binding runs after the standing gate.",
          [form("none:binding", MILESTONES, "binding", {"rust": CLASS_BINDING, "python": CLASS_BINDING, "typescript": TS_UNVALIDATED}),
           form("none:summary", MILESTONES, "summary", {"rust": {"summary": "by_validated_class"}, "python": {"summary": "by_validated_class"},
                                                       "typescript": {"summary": "empty"}})] + f2_refused),
    entry("F-U6b-2:python_refuses_transport", "language", old["F-U6b-2:python_refuses_transport"]["ruling"],
          "The header-only (transport) dispatch of the unedited successor: Rust for_source_metadata, Python "
          "_source_contract(check_receipt=False), TS its transport route. Rust and TS run the transport checks (TS through "
          "validateRetainedPrecisionTransport, I67's post-U6f round, RV88/RV92 N-1); Python has no transport validator and "
          "refuses (fail-closed). Transport is never eligible in any language.",
          [carried("F-U6b-2:python_refuses_transport", "unedited")]),
    entry("F5:refused_statement_binding", "semantics", old["F5:refused_statement_binding"]["ruling"],
          old["F5:refused_statement_binding"]["description"],
          [carried("F5:refused_statement_binding", "edited_row:no_invocation")]),
    entry("RV92-N2-N5:ts_refuses_token_rows_at_the_header", "language",
          f"{RR92} (N-2 declared as a fifth entry; N-5 declared with it); RV92 u6f_01 N-2 (28 probes), N-5 (5 probes); {TS92}",
          "A non-successor source with the W1 token on a row, or a receipt member. (N-2) Rust's and Python's header-only dispatch "
          "reads no rows and admits a token-row source (their raw dispatch refuses it); TS has one dispatch, which reads rows when "
          "present, and refuses it with RETAINED_PRECISION_DOWNGRADE_FORBIDDEN. Stricter; raw dispatch agrees in all three. "
          "(N-5) A source-blocks envelope carrying a receipt member or a token row: Rust and Python still refuse its summary "
          "stress row (open_formula_stress_summary or the max_open_formula_stress headline) with RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE, "
          "keyed on the producer id; TS keys that branch on the dispatched route, which is unsupported for the downgraded envelope, "
          "so it refuses no row there. No reliance differs: every language's standing for it is unsupported, so each rule gate "
          "refuses first.",
          [form(f"header:token_row:{label}", [fid], "transport",
                {"rust": {"transport": "ok"}, "python": {"transport": "ok"}, "typescript": {"transport": "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"}},
                requested=[], edits=[{"target": "source", "path": ["results", 0, "recovery_method"], "op": "set", "value": TOKEN}])
           for fid, label in ((LEGACY, "legacy_0_1"), (PREVIEW, "preview_physics_1"))]
          + [form(f"source_blocks_summary_row:{label}", [BLOCKS], "binding",
                  {"rust": {"binding": "source_blocks_summary:RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"},
                   "python": {"binding": "source_blocks_summary:RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE"},
                   "typescript": {"binding": "every_row:none"}},
                  requested=[], edits=edits)
             for label, edits in (("receipt_member", [{"target": "source", "path": ["retained_precision"], "op": "set", "value": {}}]),
                                  ("token_row", [{"target": "source", "path": ["results", 0, "recovery_method"], "op": "set", "value": TOKEN}]))]),
]

scope = ("Differences inherited from the base carriers are not U6 differences and are not listed here: the G7 dispatch text "
         "(each language forwards its own base validator's detail), Rust's header dispatch ignoring carrier_evidence, and the "
         "AnalysisRun builders' refusal codes. G7 parity compares the reader's (gate, code), not the dispatch text "
         f"({RR92}; RV92 u6f_01 N-4).")
note = (v2["note"] + " v3 (I66 post-U6f): 'declared_differences' entries carry 'forms'; each form names its fixtures, "
        "'invocation' (null, 'fixture', or a literal object such as {}), 'requested', 'edits', one 'subject' and one expectation "
        "per language. Subjects: 'standing'; 'transport' ('ok' or the first error code); 'binding' over every row of the edited "
        "source ('by_validated_class', 'every_row:<code>', 'every_row:none', or 'source_blocks_summary:<code>': the "
        "open_formula_stress_summary rows and the max_open_formula_stress headline row refused, every other row bound); and "
        "'summary' ('by_validated_class': one entry per receipt case counting the reader's classes, interval_bindable 0, the "
        "held withheld count; or 'empty'). An optional 'finding' or 'notice' is TS's.")
v3 = {"format": "I66-U6-CARRIER-CASES-v3", "note": note, "scope": scope, "fixtures": fixtures, "cases": v2["cases"],
      "declared_differences": declared}
PATH.write_text(json.dumps(v3, indent=2) + "\n")
print(len(v3["cases"]), "cases;", len(declared), "entries;", sum(len(e["forms"]) for e in declared), "forms;",
      hashlib.sha256(PATH.read_bytes()).hexdigest())
