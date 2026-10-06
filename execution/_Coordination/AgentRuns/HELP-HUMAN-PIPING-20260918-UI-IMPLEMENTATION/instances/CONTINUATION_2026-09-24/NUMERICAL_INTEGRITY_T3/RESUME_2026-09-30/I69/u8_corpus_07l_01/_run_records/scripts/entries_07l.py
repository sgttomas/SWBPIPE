"""I69 U8-2: the snapshot-07l entries on the two producer-solved L = 0 bases (shared by the builder and probes).

SNAPSHOT_05_PLAN s1.2 (I62) lists, for the L = 0 base (body 1 = one memberless node, all six DOFs
restrained, extent L = 0):
  isolated_rotation_stop      body 1 stop[1] = T plus its stop_rule entry          -> G5a SCALE (no non-input rotation row)
  isolated_has_data           body 1 has_data = T plus B and the record bound      -> G5a SCALE (no free DOF)
  isolated_estimate_coupled   body 1 E = (f > 0, m = 0) in selection and record;
                              estimate [force, moment]                              -> G5a SCALE (L = 0 keeps the hats uncoupled)
  isolated_estimate_uncoupled as above with estimate [force]                       -> pass (must-pass)
I69 adds one discriminating pair for the feasibility rule's own L = 0 branch (PLAN s1.2, "what it proves"):
  isolated_translation_stop           body 1 stop [T,F,F,F] plus its stop_rule entry -> pass (must-pass)
  isolated_translation_rotation_stop  body 1 stop [T,T,F,F] plus both entries        -> G5a SCALE
A reader that coupled translation and rotation at L = 0 would refuse the first and admit the second.
Every attested value is an explicit synthetic attestation on a producer-solved receipt (rehash "all").
"""
B = ["retained_precision", "body"]
COV1 = B + ["product_attempts", 0, "proof", "summary_coverage", 1]
SEL = B + ["cases", 0, "selection"]
VER = B + ["cases", 0, "run", "records", 1, "verification"]
ZERO, ONE = "0000000000000000", "3ff0000000000000"
G5A_SCALE = {"gate": "G5a", "code": "RETAINED_PRECISION_SCALE_MISMATCH"}
ELIGIBLE = {"invocation_bound": True, "numerical_eligible": True, "standing": "eligible"}


def _set(path, value):
    return {"path": path, "op": "set", "value": value}


def _stop_rule(selection, extra):
    return selection["stop_rule"] + [{"body": 1, "kind": k, "value": ZERO} for k in extra]


def _e_body1(selection, record, force, moment):
    """Body 1's E in the selection's resolution_scale and the verification record's resolution."""
    e = {"body": 1, "force": force, "moment": moment}
    sel = [x if x["body"] != 1 else e for x in selection["resolution_scale"]]
    rec = [x if x["body"] != 1 else e for x in record["resolution"]]
    return [_set(SEL + ["resolution_scale"], sel), _set(VER + ["resolution"], rec)]


def entries(base_id, mode, source):
    body = source["retained_precision"]["body"]
    selection = body["cases"][0]["selection"]
    record = body["cases"][0]["run"]["records"][1]["verification"]
    coverage = body["product_attempts"][0]["proof"]["summary_coverage"]
    # The base facts every entry relies on (asserted, not assumed).
    assert [x["body"] for x in coverage] == [0, 1] and coverage[1] == {"body": 1, "has_data": False, "stop": [False] * 4}
    assert [b["body"] for b in body["sources"][0]["body_membership"]] == [0, 1]
    assert body["sources"][0]["body_membership"][1]["members"] == [] and body["sources"][0]["body_membership"][1]["nodes"] == [2]
    assert selection["precision"] == 128 and selection["floor"] is None
    assert [x["body"] for x in selection["stop_rule"]] == [0, 0, 0, 0]
    assert [(x["body"], x["kind"]) for x in selection["verification_estimate"]] == [(0, "force"), (0, "moment")]
    assert [(x["body"], x["kind"]) for x in selection["verification_charge"]] == [(0, "force"), (0, "moment")]
    assert [x["body"] for x in selection["certified_bound"]] == [0]
    assert record["bound"][1] == {"body": 1, "value": None} and record["data_blocks"] == 1
    assert selection["resolution_scale"][1] == {"body": 1, "force": ZERO, "moment": ZERO} == record["resolution"][1]
    est = lambda kinds: selection["verification_estimate"] + [{"body": 1, "kind": k, "value": ZERO} for k in kinds]
    chg = lambda kinds: selection["verification_charge"] + [{"body": 1, "kind": k, "value": ZERO} for k in kinds]
    bound = [x if x["body"] != 1 else {"body": 1, "value": ONE} for x in record["bound"]]
    mutations = [
        {"id": f"isolated_rotation_stop_{mode}", "base": base_id, "edits": [
            _set(COV1 + ["stop"], [False, True, False, False]),
            _set(SEL + ["stop_rule"], _stop_rule(selection, ["rotation"]))], "rehash": "all", "expected": G5A_SCALE},
        {"id": f"isolated_has_data_{mode}", "base": base_id, "edits": [
            _set(COV1 + ["has_data"], True),
            _set(SEL + ["certified_bound"], selection["certified_bound"] + [{"body": 1, "value": ONE}]),
            _set(VER + ["bound"], bound),
            _set(VER + ["data_blocks"], 2)], "rehash": "all", "expected": G5A_SCALE},
        {"id": f"isolated_estimate_coupled_{mode}", "base": base_id, "edits":
            _e_body1(selection, record, ONE, ZERO) + [
            _set(SEL + ["verification_estimate"], est(["force", "moment"])),
            _set(SEL + ["verification_charge"], chg(["force", "moment"]))], "rehash": "all", "expected": G5A_SCALE},
    ]
    must_pass = [
        {"id": f"isolated_estimate_uncoupled_{mode}", "base": base_id, "edits":
            _e_body1(selection, record, ONE, ZERO) + [
            _set(SEL + ["verification_estimate"], est(["force"])),
            _set(SEL + ["verification_charge"], chg(["force"]))], "rehash": "all", "expected": "pass",
         "expected_eligibility": dict(ELIGIBLE)},
    ]
    extra_mutations = [
        {"id": f"isolated_translation_rotation_stop_{mode}", "base": base_id, "edits": [
            _set(COV1 + ["stop"], [True, True, False, False]),
            _set(SEL + ["stop_rule"], _stop_rule(selection, ["translation", "rotation"]))], "rehash": "all", "expected": G5A_SCALE},
    ]
    extra_must_pass = [
        {"id": f"isolated_translation_stop_{mode}", "base": base_id, "edits": [
            _set(COV1 + ["stop"], [True, False, False, False]),
            _set(SEL + ["stop_rule"], _stop_rule(selection, ["translation"]))], "rehash": "all", "expected": "pass",
         "expected_eligibility": dict(ELIGIBLE)},
    ]
    return mutations, must_pass, extra_mutations, extra_must_pass
