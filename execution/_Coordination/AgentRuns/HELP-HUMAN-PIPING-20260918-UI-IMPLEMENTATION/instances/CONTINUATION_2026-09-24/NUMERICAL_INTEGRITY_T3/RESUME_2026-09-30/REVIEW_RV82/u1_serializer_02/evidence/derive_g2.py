#!/usr/bin/env python3
"""RV82 grant-2 item 2: G-i translations.
(a) RV82's own spot derivation: expected wire values written by hand from C2 §2-§4,
    C3 §3, RR:7784 (work_accounting carries the fault only) and the schema $defs,
    compared with the serializer's output for the same native variant.
(b) Every corpus value validated against its schema $def with jsonschema (own run).
Inputs (env): P = a candidate tree's projects/chirality-piping; CORPUS = g2_corpus.json."""
import json, os, struct, sys
import jsonschema

P, CORPUS = os.environ["P"], os.environ["CORPUS"]
schema = json.load(open(f"{P}/schemas/retained_precision_mp_v2.schema.json"))
corpus = {e["label"]: e for e in json.load(open(CORPUS))}
bits = lambda x: "%016x" % struct.unpack(">Q", struct.pack(">d", x))[0]
DOF = {"node": 3, "component": "RY"}
Q0 = {"tag": "displacement", "dof": DOF}
ENC = [{"check": "encoding"}]; ASSOC = [{"check": "association"}]
S = lambda tag, **kw: {"space": "stop", "tag": tag, **kw}
# label -> (expected value, expected failure checks, the contract basis)
EXPECTED = {
 "wide_0": (None, ENC, "schema WideError has no count_range: unencodable (C1 §4)"),
 "wide_1": (None, ENC, "schema WideError has no work_accounting (I34 API-02 proposal not adopted)"),
 "wide_2": ({"tag": "invalid_precision", "precision": 96}, [], "C2 §2 WideError"),
 "wide_3": ({"tag": "non_finite"}, [], "C2 §2"),
 "wide_11": ({"tag": "accumulator", "error": {"tag": "non_finite"}}, [], "C2 §2 accumulator{error:SumError}"),
 "wide_14": ({"tag": "operand_precision"}, [], "C2 §2"),
 "stop_0": (S("count_range", name="rows"), [], "schema Stop count_range{name}"),
 "stop_1": (S("work_accounting", fault="overflow"), [], "I34 API-02 stop/work_accounting{fault}"),
 "stop_3": (S("work_accounting", fault="both"), [], "fault=overflow|inconsistent|both"),
 "stop_4": (S("budget", scope="case"), [], "C2 §2 stop/budget{scope}"),
 "stop_8": (S("arithmetic", error={"tag": "division_by_zero"}), [], "C2 §2 stop/arithmetic{error:WideError}"),
 "stop_10": (S("pivot", global_dof=7), [], "C2 §2 stop/pivot{global_dof}"),
 "stop_12": (S("negative_energy", i=1, j=2), [], "C2 §2 stop/negative_energy{i,j}"),
 "stop_14": (S("residual_gate", global_dof=9), [], "C2 §2"),
 "stop_18": (S("resolution_scale", body=1, kind="moment"), [], "C2 §2 stop/resolution_scale{body,kind}"),
 "stop_19": (S("publication_certificate", index=None, issue="shape"), [], "C2 §2 {index:null|U,issue}"),
 "stop_28": (S("publication_certificate", index=4, issue="radius_class_mismatch"), [], "C2 §2"),
 "quantity_0": (Q0, [], "C2 §2 QuantityId displacement{dof}"),
 "quantity_4": ({"tag": "end_action", "member": 1, "end": "j", "component": "RZ"}, [], "C2 §2 end_action{member,end,component}"),
 "quantity_8": ({"tag": "support_moment_magnitude", "support": 1}, [], "C2 §2"),
 "attempt_0": ({"space": "attempt", "tag": "stop", "stop": S("condition")}, [], "C2 §2 attempt/stop{stop:Stop}"),
 "attempt_3": ({"space": "attempt", "tag": "charge", "quantity": Q0, "body": 1, "kind": "moment"}, [], "C2 §2 attempt/charge{quantity,body,kind}"),
 "attempt_4": ({"space": "attempt", "tag": "verification_failed"}, [], "C2 §2"),
 "attempt_8": ({"space": "attempt", "tag": "publication_enclosure", "quantity": Q0, "body": 0, "kind": "rotation", "predicate": "sharper_binary64"}, [], "C2 §2"),
 "attempt_11": ({"space": "attempt", "tag": "g_validity", "member": 4}, [], "C2 §2 attempt/g_validity{member}"),
 "outcome_failed_4": ({"kind": "failed", "reason": {"space": "attempt", "tag": "verification_failed"}}, [], "C1 §4 Outcome"),
 "outcome_2": ({"kind": "solved"}, [], "C1 §4"),
 "unresolved_1": ({"space": "unresolved", "tag": "work_accounting", "fault": "both"}, [], "RR:7784 / C3:261-263: prior not on the wire"),
 "unresolved_7": ({"space": "unresolved", "tag": "arithmetic", "error": {"tag": "invalid_precision", "precision": 8}}, [], "C2 §2"),
 "unresolved_10": ({"space": "unresolved", "tag": "publication_certificate", "index": 3, "issue": "row_identity"}, [], "C2 §2"),
 "refusal_0": ({"space": "refusal", "tag": "mechanism_witnessed", "body": 0,
                "rigid_parameters": [bits(1.0), bits(0.0), bits(-0.5), bits(0.25), bits(2.0), bits(0.0)]}, [], "C2 §2 {body,rigid_parameters:[B;6]}"),
 "refusal_1": ({"space": "refusal", "tag": "geometry_unavailable", "body": 1, "error": {"tag": "range", "detail": "geometry"}}, [], "C2 §2 + StructuralError range{detail}"),
 "refusal_3": ({"space": "refusal", "tag": "ledger_unavailable", "error": {"tag": "accumulator", "error": {"tag": "non_representable"}}}, [], "C2 §2 LedgerError"),
 "block_0": ({"block": 0, "bound": "uc", "kind": "span", "pass": "forward", "location": {"kind": "block_step"}}, [], "C2 §2 BlockRefusal; usize::MAX is block_step"),
 "block_10": ({"block": 10, "bound": "uc", "kind": "exponent", "pass": "shift_form", "location": {"kind": "row", "index": 10}}, [], "C2 §2"),
 "numeric_0": ({"kind": "arithmetic", "cause": S("exponent")}, [], "C3 §3 NumericError arithmetic{cause:Stop}"),
 "numeric_9": ({"kind": "binary64_range"}, [], "C3 §3"),
 "view_0": ({"kind": "certificate", "issue": "non_finite"}, [], "C3 §3 ViewIssue"),
 "view_7": ({"kind": "work", "fault": "inconsistent"}, [], "C3 §3"),
 "bridge_1": ({"kind": "numeric", "cause": {"kind": "axis_bits"}}, [], "C3 §3 BridgeError"),
 "bridge_4": ({"kind": "missing_radius", "row": 11}, [], "C3 §3"),
 "bridge_9": ({"kind": "alpha_condition", "block": 3}, [], "C3 §3; alpha endpoint private"),
 "product_2": ({"kind": "native_source", "cause": {"kind": "storage"}}, [], "C3:272 native_source{cause:BridgeError}"),
 "product_7": ({"kind": "numeric_helper", "cause": {"kind": "arithmetic", "cause": S("structure")}}, [], "C3 §3 HelperError"),
 "product_16": ({"kind": "numeric_predicate", "row": 6, "predicate": "input_derived"}, [], "C3 §3"),
 "source_10": (None, ASSOC, "C2 §3: ZeroDirection impossible -> typed internal association failure"),
 "source_12": ({"tag": "duplicate_constraint", "dof": DOF}, [], "C2 §3"),
 "source_16": ({"tag": "unknown_member", "station": 11, "member": 12}, [], "C2 §3 unknown_member{station,member}"),
 "source_40": ({"tag": "subnormal_derived_primitive", "member": 1, "property": "y_reference"}, [], "C2 §3"),
 "origin_3": ({"kind": "missing_selected_origin", "operand": 1}, [], "C3 §3 OriginError"),
 "capture_7": ({"kind": "prepared_proof", "cause": {"kind": "numeric", "cause": {"kind": "non_finite"}}}, [], "C3 §3 CaptureError"),
 "capture_11": ({"kind": "accounting", "event": "map_write"}, [], "C3 §3"),
 "operational_7": ({"kind": "coefficient_range", "coefficient": "EA/L", "operation": "sqrt"}, [], "C3 §3 OperationalError"),
 "g5a_7": ({"kind": "sanity", "body": 1, "quantity_kind": None}, ENC, "schema quantity_kind in {0,1}: unencodable"),
 "g5a_8": ({"kind": "operational", "member_index": 3, "cause": {"kind": "non_finite", "operation": "div", "entered": 3}}, [], "C3 §3 G5aError"),
 "section_2": ({"kind": "ambiguous_rounding", "property": "radius"}, [], "C3 §3 SectionError property index 4"),
 "section_4": (None, ENC, "property index 5 has no name: unencodable"),
 "structural_2": ({"tag": "asymmetric", "row": 1, "col": 2, "relative_skew": bits(1e-3)}, [], "C2 §4 StructuralError"),
 "structural_4": ({"tag": "numerically_unresolved", "detail": "pivot", "global_dof": None}, [], "C2 §4 {detail,global_dof:null|U}"),
 "public_0": ({"kind": "preparation", "capture": {"kind": "origin", "cause": {"kind": "allocation"}}, "section": {"kind": "invalid_geometry"}}, [], "C3 §3 PublicError"),
 "public_2": ({"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "allocation"}}}, [], "D38: native failure before any Run -> capture{cause}"),
 "public_4": ({"kind": "numeric", "cause": None}, [], "C3 §3 numeric{cause:null|ProductError}"),
}
ok = bad = 0
out = []
for label, (value, fails, basis) in EXPECTED.items():
    e = corpus[label]
    got_fail = [{"check": f["check"]} for f in e["failures"]]
    same = (e["value"] == value) and (got_fail == fails)
    ok += same; bad += not same
    out.append(("SAME " if same else "DIFF ") + f"{label} [{e['def']}] basis: {basis}" + ("" if same else f"\n   expected {value} {fails}\n   got      {e['value']} {got_fail}"))
print("\n".join(out))
print(f"SPOT DERIVATION: {ok} same, {bad} different, of {len(EXPECTED)}")

# (b) schema validation of every corpus value against its $def
def validator_for(def_name):
    if def_name == "BlockRefusal":
        sub = schema["$defs"]["Physical"]["properties"]["bound_refusals"]["items"]
    else:
        sub = {"$ref": f"#/$defs/{def_name}"}
    full = dict(sub); full["$defs"] = schema["$defs"]
    return jsonschema.Draft202012Validator(full)
counts = {"valid": 0, "invalid": 0, "refused": 0}
from collections import Counter
per_def = Counter()
invalid = []
for e in corpus.values():
    per_def[e["def"]] += 1
    if e["failures"]:
        counts["refused"] += 1
        continue
    errs = list(validator_for(e["def"]).iter_errors(e["value"]))
    if errs: counts["invalid"] += 1; invalid.append((e["label"], errs[0].message[:200]))
    else: counts["valid"] += 1
print(f"SCHEMA: {len(corpus)} entries over {len(per_def)} defs; {counts}")
for x in invalid: print("  INVALID", x)
refused = [(e["label"], e["def"], [f["check"] + ":" + f["path"] for f in e["failures"]]) for e in corpus.values() if e["failures"]]
print("REFUSED (typed, no wire form):"); [print("  ", r) for r in refused]
sys.exit(1 if bad or counts["invalid"] else 0)
