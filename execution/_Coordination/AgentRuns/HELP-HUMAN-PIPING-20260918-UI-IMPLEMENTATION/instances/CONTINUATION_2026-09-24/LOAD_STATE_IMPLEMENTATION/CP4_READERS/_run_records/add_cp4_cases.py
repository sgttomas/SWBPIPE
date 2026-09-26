"""CP4_READERS: append the N-2 and N-3 shared cases to the load-reference case file.

Run from WORKING_ROOT (projects/chirality-piping). Append-only: existing cases,
their order and their bytes are unchanged; only the description gains one
sentence about the new `json_text` op. Expectations are authored from
CP3_WIRE_ADDENDUM.md section 1 and serde_json's number semantics, not captured
from either reader.
"""
import json
from pathlib import Path

PATH = Path("core/reporting/result_export/tests/fixtures/load_reference_mutations.json")
P = "SOURCE_LOAD_REFERENCE_"
r0 = "/contract_evidence/load_reference_states/0"
m0 = f"{r0}/members/0"
BIG = 100000000000000000000          # 1e20, exactly representable in binary64
BIG_PLUS_ONE = 100000000000000000001  # outside u64; rounds to 1e20
NEXT_ABOVE_BIG = 100000000000000016384  # the binary64 successor of 1e20
OVERFLOW = "1" + "0" * 400            # integer literal beyond binary64 range
MAX_FINITE_INT = 2**1024 - 2**970 - 1  # largest integer that rounds to the largest finite binary64
FIRST_OVERFLOW = 2**1024 - 2**970      # the tie above it rounds to even, i.e. to 2^1024: overflow


def s(path, value):
    return {"op": "set", "path": path, "value": value}


def temps(operating, selection, point):
    return [s(f"{m0}/operating_temperature_k", operating), s(f"{m0}/material_selection_temperature_k", selection), s(f"{m0}/consumed_material_points/0/temperature_k", point)]


def seg(use, lower, upper, start, end):
    return {"use": use, "lower_index": lower, "upper_index": upper, "start_k": start, "end_k": end}


I, S = "integration_interval", "interpolation_sample"
CARRIER_NOTE = "both readers admit the document, but the checked-JSON canonical carrier profile (openpipestress_jcs_ijson_v1) refuses any integral number beyond 2^53-1 in magnitude in both languages, so no derivative document (Rust) or AnalysisRun (Python) is built; the codes are those of the pre-existing canonical-JSON layers"


def carrier(python_code):
    return {"carrier_rust": "UNSAFE_JSON_NUMBER", "carrier_python": "CHECKED-JSON-" + python_code, "note": CARRIER_NOTE}
LITERAL_NOTE = "an integer literal beyond binary64 range has no serde_json Value (the JSON text is refused as number out of range); Python holds it as an int and refuses it"

new_cases = [
    # N-2: any finite JSON integer is a number, with the binary64 value Rust obtains.
    {"id": "N2-integer-temperatures-positive", "source": "pressure-dense", "ops": temps(BIG, BIG, BIG), "dispatch": "accept", **carrier("UNSAFE-INTEGER")},
    {"id": "N2-exponent-temperatures-positive", "source": "pressure-dense", "ops": temps(1e20, 1e20, 1e20), "dispatch": "accept", **carrier("NUMBER-OUTSIDE-PROFILE")},
    {"id": "N2-integer-segment-negative", "source": "pressure-dense", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, -BIG, BIG)])], "dispatch": "accept", **carrier("UNSAFE-INTEGER")},
    {"id": "N2-exponent-segment-negative", "source": "pressure-dense", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, -1e20, 1e20)])], "dispatch": "accept", **carrier("NUMBER-OUTSIDE-PROFILE")},
    {"id": "N2-integer-binary64-equality", "source": "pressure-dense", "ops": temps(BIG_PLUS_ONE, BIG_PLUS_ONE, 1e20), "dispatch": "accept", **carrier("NUMBER-OUTSIDE-PROFILE")},
    {"id": "N2-integer-u64-rounding-equality", "source": "pressure-dense", "ops": temps(9007199254740993, 9007199254740993, 9007199254740992.0), "dispatch": "accept", **carrier("NUMBER-OUTSIDE-PROFILE")},
    {"id": "N2-integer-binary64-inequality", "source": "pressure-dense", "ops": temps(BIG, BIG_PLUS_ONE, NEXT_ABOVE_BIG), "dispatch": P + "MATERIAL_SELECTION"},
    {"id": "N2-integer-temperature-negative", "source": "pressure-dense", "ops": [s(f"{m0}/operating_temperature_k", -BIG)], "dispatch": P + "TEMPERATURE_RANGE"},
    {"id": "N2-integer-overflow-positive", "source": "pressure-dense", "ops": [{"op": "json_text", "path": f"{m0}/operating_temperature_k", "value": OVERFLOW}], "dispatch": None, "dispatch_rust": "JSON_PARSE_REJECTED", "dispatch_python": P + "NUMBER_INVALID", "validator_rust": "JSON_PARSE_REJECTED", "validator_python": P + "NUMBER_INVALID", "note": LITERAL_NOTE},
    {"id": "N2-integer-overflow-negative", "source": "pressure-dense", "ops": [{"op": "json_text", "path": f"{m0}/consumed_material_points/0/temperature_k", "value": "-" + OVERFLOW}], "dispatch": None, "dispatch_rust": "JSON_PARSE_REJECTED", "dispatch_python": P + "NUMBER_INVALID", "validator_rust": "JSON_PARSE_REJECTED", "validator_python": P + "NUMBER_INVALID", "note": LITERAL_NOTE},
    {"id": "N2-integer-largest-finite", "source": "pressure-dense", "ops": [s(f"{m0}/operating_temperature_k", MAX_FINITE_INT)], "dispatch": "accept", **carrier("UNSAFE-INTEGER")},
    {"id": "N2-integer-first-overflow", "source": "pressure-dense", "ops": [{"op": "json_text", "path": f"{m0}/operating_temperature_k", "value": str(FIRST_OVERFLOW)}], "dispatch": None, "dispatch_rust": "JSON_PARSE_REJECTED", "dispatch_python": P + "NUMBER_INVALID", "validator_rust": "JSON_PARSE_REJECTED", "validator_python": P + "NUMBER_INVALID", "note": LITERAL_NOTE},
    # N-3: CP3_WIRE_ADDENDUM section 1 for every consumed and consulted segment entry.
    {"id": "N3-segment-reviewer-example", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 3, 300, 300)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-segment-non-adjacent", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 2, 293.15, 373.15)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-segment-same-index", "source": "connected-sparse", "ops": [s(f"{m0}/consulted_law_segments", [seg(S, 1, 1, 300.0, 300.0)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-segment-descending-index", "source": "connected-sparse", "ops": [s(f"{m0}/consulted_law_segments", [seg(S, 2, 1, 300.0, 300.0)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-interval-equal-ends", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, 300.0, 300.0)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-interval-equal-ends-consulted", "source": "connected-sparse", "ops": [s(f"{m0}/consulted_law_segments", [seg(I, 1, 2, 323.15, 323.15)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-interval-reversed", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, 373.15, 293.15)])], "dispatch": P + "LAW_SEGMENT"},
    {"id": "N3-duplicate-consumed", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, 293.15, 323.15), seg(S, 1, 2, 373.15, 373.15), seg(I, 0, 1, 293.15, 323.15)])], "dispatch": P + "LAW_SEGMENT_DUPLICATE"},
    {"id": "N3-duplicate-consulted", "source": "connected-sparse", "ops": [s(f"{m0}/consulted_law_segments", [seg(S, 1, 2, 373.15, 373.15), seg(S, 1, 2, 373.15, 373.15)])], "dispatch": P + "LAW_SEGMENT_DUPLICATE"},
    {"id": "N3-duplicate-numeric-form", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, 300, 400), seg(I, 0, 1, 300.0, 400.0)])], "dispatch": P + "LAW_SEGMENT_DUPLICATE"},
    {"id": "ACCEPT-N3-consumed-consulted-overlap", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 1, 2, 323.15, 373.15), seg(S, 1, 2, 373.15, 373.15)]), s(f"{m0}/consulted_law_segments", [seg(S, 1, 2, 373.15, 373.15), seg(I, 1, 2, 323.15, 373.15)])], "dispatch": "accept"},
    {"id": "ACCEPT-N3-distinct-entries", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 1, 293.15, 323.15), seg(I, 0, 1, 323.15, 373.15), seg(S, 0, 1, 323.15, 323.15), seg(S, 1, 2, 400.0, 400.0)])], "dispatch": "accept"},
]
new_transport = [
    {"id": "TRANSPORT-N3-segment-non-adjacent", "source": "connected-sparse", "ops": [s(f"{m0}/consumed_law_segments", [seg(I, 0, 3, 300, 400)])], "expect": P + "LAW_SEGMENT"},
    {"id": "TRANSPORT-N3-duplicate", "source": "connected-sparse", "ops": [s(f"{m0}/consulted_law_segments", [seg(S, 1, 2, 373.15, 373.15), seg(S, 1, 2, 373.15, 373.15)])], "expect": P + "LAW_SEGMENT_DUPLICATE"},
    {"id": "TRANSPORT-N2-integer-temperatures", "source": "pressure-dense", "ops": temps(BIG, BIG, BIG), "expect": "accept"},
]
SENTENCE = " The 'json_text' op splices its string value into the document as JSON text (Rust: into the serialized text before parsing; Python: json.loads of the value), so that literals a serde_json Value cannot hold are exercised; in Rust such a case must be refused at the text boundary. 'carrier_rust' and 'carrier_python' on an accepted case give the exact refusal of the canonical carrier build (Rust derivative::derive_document, Python build_analysis_run) where the checked-JSON number profile refuses a document that both readers admit."

raw = PATH.read_text()
doc = json.loads(raw)
assert json.dumps(doc, indent=1) + "\n" == raw
existing = {c["id"] for key in ("cases", "table_cases", "transport_cases") for c in doc[key]}
assert not existing & {c["id"] for c in new_cases + new_transport}
assert SENTENCE not in doc["description"]
doc["description"] += SENTENCE
doc["cases"].extend(new_cases)
doc["transport_cases"].extend(new_transport)
PATH.write_text(json.dumps(doc, indent=1) + "\n")
print(json.dumps({"cases": len(doc["cases"]), "table_cases": len(doc["table_cases"]), "transport_cases": len(doc["transport_cases"]), "added": len(new_cases) + len(new_transport)}))
