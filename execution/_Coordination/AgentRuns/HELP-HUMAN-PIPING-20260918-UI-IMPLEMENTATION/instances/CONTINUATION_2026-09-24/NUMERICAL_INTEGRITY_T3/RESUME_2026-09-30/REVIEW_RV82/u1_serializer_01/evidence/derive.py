#!/usr/bin/env python3
"""RV82 item 2: derive the expected retained_precision members for the milestone
independently (contract C1/C2/C3, decisions 1-2, T1 (a), D39, the in-tree fixtures,
the plain ordinary envelope and the probe's native facts) and compare them with
the serializer's output. The emitter of experiments 02/03 is NOT used.

Inputs (env): P = a candidate tree's projects/chirality-piping; OUT = probe output dir.
"""
import hashlib, json, os, re, sys

P = os.environ["P"]; OUT = os.environ["OUT"]
sys.path.insert(0, P)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1  # reader-side canonicalizer

def jcs_simple(v):
    # RFC 8785 for payloads holding only strings, ints, bools, null (no floats).
    def chk(x):
        if isinstance(x, float): raise ValueError("float")
        if isinstance(x, dict): [chk(y) for y in x.values()]
        if isinstance(x, list): [chk(y) for y in x]
    chk(v)
    return json.dumps(v, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
def H_simple(domain, payload):
    return hashlib.sha256(jcs_simple({"domain": domain, "payload": payload}).encode()).hexdigest()
def H_reader(domain, payload):
    return canonical_sha256_checked_v1({"domain": domain, "payload": payload})

req = json.load(open(f"{P}/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"))
definition = json.load(open(f"{P}/fixtures/results/retained_precision_prepared_ordinary_v1.json"))
table = json.load(open(f"{P}/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json"))
schema = json.load(open(f"{P}/schemas/retained_precision_mp_v2.schema.json"))
SELECTED_MESSAGE = ("Retained-precision recovery (contribution_preserving_multiprecision_v1) is selected for this load case. "
    "Its published rows carry recovery_method; the retained_precision receipt binds their certified classes, the native attempts "
    "and work, and the ordinary-attempt evidence.")
RP_NON_QUANTITY = {"NonQuantity", "ModulusBasisRecord", "DenseParityObservation"}
results = []
def check(name, ok, detail=""):
    results.append((name, bool(ok), detail)); return ok

for mode in ["sparse_interactive", "dense_scrutiny"]:
    plain = json.load(open(f"{OUT}/plain_{mode}.json"))
    succ = json.load(open(f"{OUT}/successor_{mode}.json"))
    facts = json.load(open(f"{OUT}/facts_{mode}.json"))
    rp = succ["retained_precision"]; body = rp["body"]
    env = {k: v for k, v in succ.items() if k != "retained_precision"}
    case_id = req["model"]["load_cases"][0]["id"]
    m = mode + ": "

    # --- the plain ordinary facts this derivation starts from
    integrity_id = f"diagnostic:numerical-integrity:{case_id}"
    legacy_id = f"diagnostic:source-recovery:{case_id}"
    pd = {d["id"]: d for d in plain["diagnostics"]}
    integrity = pd[integrity_id]; legacy = pd[legacy_id]
    check(m + "plain bytes are the protected ordinary bytes", (facts["plain_len"], facts["plain_sha256"]) in
          [(68250, "9c7ec1a144a729f456a25fedb5dbe6a7dafde67d52b9cc8caff6c39bb0050871"), (69366, "21ca629c27e6ca03b1411c8c51097f7a50f90429045b1e36313163014dd4278a")])
    check(m + "control B (captured ordinary run = plain)", facts["captured_ordinary_equals_plain"])

    # --- G-a and T1 (a): the envelope transformation
    exp_diags = [d for d in plain["diagnostics"] if d["id"] != legacy_id] + [{
        "id": f"diagnostic:retained-precision:{case_id}:selected", "code": "RETAINED_PRECISION_SELECTED", "severity": "info",
        "message": SELECTED_MESSAGE, "source": "core/product_physics", "affected_refs": [case_id]}]
    check(m + "T1(a)+G-a: diagnostics = plain minus the legacy disclosure plus one selected", env["diagnostics"] == exp_diags)
    check(m + "T1(a): the omitted disclosure was SOURCE_BLOCK_RECOVERY_UNAVAILABLE naming the case",
          legacy["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" and legacy["affected_refs"] == [case_id])
    check(m + "G4: no legacy-unavailable diagnostic names the selected case",
          not any(d["code"] == "SOURCE_BLOCK_RECOVERY_UNAVAILABLE" and case_id in d.get("affected_refs", []) for d in env["diagnostics"]))
    check(m + "G-a: identity and profile from the semantic table",
          env["producer"]["semantic_contract_id"] == table["semantic_contract_id"] and env["formulation_basis"]["profile_id"] == table["formulation_profile_id"])
    method = body["cases"][0]["method"]
    check(m + "G-a: method token in the schema", json.dumps(schema).count('"%s"' % method) >= 1 and method == "contribution_preserving_multiprecision_v1")
    check(m + "G-a: every row of the case carries recovery_method, none other",
          all((r.get("recovery_method") == method) == (r["basis_ref"]["ref_id"] == case_id) for r in env["results"]) and
          all("recovery_method" not in r for r in plain["results"]))
    # Everything else in the envelope outside diagnostics/results/identity is the candidate's own (overlay values are upstream, I51).
    check(m + "envelope keys unchanged", set(env) == set(plain))

    # --- G-b: initial / w2 / formation from the plain report
    oa = body["ordinary_attempts"][0]
    q = plain["numerical_quality"]["cases"]; qi = [i for i, x in enumerate(q) if x["basis_ref"]["ref_id"] == case_id]
    outcome = {"NUMERICAL_INTEGRITY_SENSITIVE": "sensitive", "NUMERICAL_INTEGRITY_CHECKS_PASSED": "checks_passed"}[integrity["code"]]
    check(m + "G-b initial = report(integrity id, outcome from the code) and equal to numerical_quality",
          oa["initial"] == {"kind": "report", "report_diagnostic_ref": integrity_id, "outcome": outcome} and q[qi[0]]["solve_quality"] == outcome)
    check(m + "G-b w2 = not_triggered (no range_scaling line, no W2 diagnostic)",
          oa["w2"] == {"kind": "not_triggered"} and "range_scaling:" not in integrity["message"] and not any("force-scal" in d["id"] for d in plain["diagnostics"]))
    d5_line = " formation_check: reason=" in integrity["message"]
    check(m + "F1/G-b: d5_diagnostic_ref names the integrity diagnostic iff K-D5's line is in it",
          oa["formation"]["d5_diagnostic_ref"] == (integrity_id if d5_line else None) and d5_line)
    check(m + "G-b: load_row_finding null (no S11-G finding captured; native seed)",
          oa["formation"]["load_row_finding"] is None and "load_row_finding: None" in facts["seed_debug"])

    # --- G-l and D39: from the published Debug text of the legacy failure (oracle only)
    mt = re.search(r'stage: "([^"]+)".*?charged: (\d+), rejected: (\d+), limit: (\d+)', legacy["message"])
    stage, charged, rejected, limit = mt.group(1), int(mt.group(2)), int(mt.group(3)), int(mt.group(4))
    helper = re.search(r"helper_stage: (\w+)", legacy["message"]).group(1)
    snake = re.sub(r"(?<!^)(?=[A-Z])", "_", helper).lower()
    # D39: capture present, no nonlinear support, no combination => eligible; report Sensitive => needs recovery;
    # charged > 0 => an actual attempt ran (a decline is 0/0/0) => unavailable.
    eligible = req["model"]["combinations"] == [] and not any(s.get("family") in ("nonlinear", "gap", "contact") for s in req["model"]["supports"])
    needs = outcome == "sensitive"
    disposition = "not_eligible" if not eligible else "not_required" if not needs else ("unavailable" if charged > 0 else "declined_without_attempt")
    check(m + "D39 row derived = unavailable", disposition == "unavailable")
    check(m + "T1(a): legacy_source = {unavailable, null, 0}",
          oa["legacy_source"] == {"disposition": disposition, "diagnostic_ref": None, "work_ref": 0})
    check(m + "T1(a): legacy_source_work[0] = the actual WorkReport",
          body["legacy_source_work"] == [{"case_index": 0, "stage": stage, "helper_stage": snake, "charged": charged, "rejected": rejected,
                                           "limit": limit, "settlement": "booked"}], f"{stage} {snake} {charged} {rejected} {limit}")

    # --- decision 2 (A2/D6a): diagnostics naming the case, once, envelope order, minus RETAINED_PRECISION_* and the T1 omission
    exp_refs = [d["id"] for d in plain["diagnostics"] if case_id in d.get("affected_refs", []) and d["id"] != legacy_id
                and not d["code"].startswith("RETAINED_PRECISION_")]
    check(m + "decision 2: exact diagnostic_refs", oa["diagnostic_refs"] == exp_refs, str(exp_refs))
    check(m + "decision 2: no invocation-level diagnostic attributed",
          all(case_id in {d["id"]: d for d in env["diagnostics"]}[r]["affected_refs"] for r in oa["diagnostic_refs"]))
    check(m + "G-c: ordinary member fields", (oa["case_index"], oa["case_id"], oa["material_basis_ref"], oa["requested_mode"]) == (0, case_id, 0, mode))

    # --- G-j: one case, one call/run/source, indices from the invocation
    c = body["cases"][0]
    check(m + "G-j: case indices", c["basis_ref"] == {"ref_type": "load_case", "ref_id": case_id} and c["status"] == "selected"
          and c["ordinary"] == {"attempt_ref": 0, "quality_binding": {"kind": "present", "index": qi[0]}} and c["product_attempt_ref"] == 0
          and c["source_ref"] == 0 and c["run"]["id"] == 0 and c["run"]["origin"]["owner_ref"] == {"kind": "case", "index": 0}
          and c["run"]["origin"]["source_ref"] == 0)
    check(m + "G-j: one call/run/source in the native invocation", (facts["runs"], facts["calls"], facts["sources"], facts["case_run"]) == (1, 1, 1, 0))
    check(m + "G-j: work frame", body["work"]["execution_order"] == [{"kind": "case", "index": 0}] and body["work"]["case_limit"] == 20_000_000_000
          and body["work"]["invocation_limit"] == facts["meter_limit"] == 60_000_000_000 and body["work"]["charged"] == c["run"]["invocation_after"])
    check(m + "G-j: calls", len(body["calls"]) == 1 and body["calls"][0]["owner_refs"] == [{"kind": "case", "index": 0}]
          and body["calls"][0]["source_refs"] == [0] and body["calls"][0]["run_refs"] == [0]
          and body["calls"][0]["invocation_after"] == c["run"]["invocation_after"])

    # --- decision 1 (A1): raw SHA256 of the K4RST bytes
    check(m + "A1: K4RST magic", facts["k4rst_prefix_hex"] == "4b3452535401")
    check(m + "A1: retained_state_sha256 = sha256(K4RST bytes)", c["selection"]["retained_state_sha256"] == facts["k4rst_sha256"])
    check(m + "ledger_sha256 = sha256(K4LED bytes)", c["selection"]["ledger_sha256"] == facts["ledger_sha256"])

    # --- G-d: support_indices from the request's own rigid restraints
    nodes = [n["id"] for n in req["model"]["nodes"]]; comps = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
    src = body["sources"][0]
    exp_constraints = [{"node": k["node"], "component": comps[k["component"]]} for k in facts["kernel_constraints"]]
    check(m + "G-d: constraint dofs = kernel constraints", [x["dof"] for x in src["constraints"]] == exp_constraints)
    for x in src["constraints"]:
        owners = [i for i, s in enumerate(req["model"]["supports"]) if s.get("family") != "spring"
                  and nodes.index(s["node"]) == x["dof"]["node"] and x["dof"]["component"] in s["restraints"]]
        check(m + f"G-d: support_indices of {x['dof']}", x["support_indices"] == owners, str(owners))

    # --- G-e: not_covered and absolute_verified from the certificate's verdicts
    rows = [r["id"] for r in env["results"]]
    verdicts = facts["verdicts"]
    exp_nc = [v["row_id"] for v in verdicts if v["class"] == "None" and v["recipe"].split("(")[0].split(" ")[0] not in RP_NON_QUANTITY]
    check(m + "G-e: not_covered from verdicts", c["selection"]["not_covered"] == exp_nc, str(exp_nc))
    exp_abs = []
    for v in verdicts:
        mm = re.match(r"Some\(AbsoluteVerified \{ bound_bits: (\d+) \}\)", v["class"])
        if mm: exp_abs.append({"result_id": v["row_id"], "bound": "%016x" % int(mm.group(1))})
    check(m + "G-e: absolute_verified from verdicts", c["selection"]["absolute_verified"] == exp_abs, f"{len(exp_abs)} rows")
    check(m + "verdict rows = envelope rows", [v["row_id"] for v in verdicts] == rows and all(v["passed"] for v in verdicts))

    # --- definition and table hashes from the in-tree fixtures
    dsha = H_simple("retained_precision_formation_v1", definition)
    check(m + "definition hash (own JCS) = table entry", table["product_formation_definitions"] == [{"id": "RP-PREPARED-ORDINARY-DUAL-v1", "sha256": dsha}], dsha)
    check(m + "definition hash (reader canonicalizer) agrees", H_reader("retained_precision_formation_v1", definition) == dsha)
    pa = body["product_attempts"][0]
    check(m + "product attempt binds the definition", pa["definition_id"] == "RP-PREPARED-ORDINARY-DUAL-v1")
    check(m + "policies from the table", body["policy"] == table["receipt_policy"] and body["facade_policy"] == table["accuracy_classification"]["policy"])
    prep_payload = {"definition_id": pa["definition_id"], "definition_sha256": dsha, "owner_ref": pa["owner_ref"],
                    "ordinary_attempt_ref": pa["ordinary_attempt_ref"], "material_basis_ref": pa["material_basis_ref"],
                    "members": [{"member": x["member"], "old_source": x["old_source"], "old_facts": x["old_facts"], "section": x["result"]["section"]}
                                for x in pa["preparation"]["members"]]}
    check(m + "sources[].preparation.sha256 recomputed", src["preparation"] == {"attempt_ref": 0, "sha256": H_simple("retained_precision_preparation_v1", prep_payload)})
    binding = {k: v for k, v in src.items() if k != "index"}
    check(m + "source_identity_sha256 recomputed", c["source_identity_sha256"] == H_simple("retained_precision_source_mp_v2", binding))
    check(m + "publication_sha256 recomputed (reader canonicalizer)", body["publication_sha256"] == H_reader("retained_precision_publication_mp_v2", env))
    check(m + "receipt_sha256 recomputed (own JCS)", rp["receipt_sha256"] == H_simple("retained_precision_receipt_mp_v2", body))
    check(m + "receipt_sha256 recomputed (reader canonicalizer)", rp["receipt_sha256"] == H_reader("retained_precision_receipt_mp_v2", body))
    check(m + "material basis: base selector, explicit G, model materials (request materials empty)",
          body["material_bases"] == [{"index": 0, "selector": {"kind": "base"}, "case_indices": [0], "materials": [
              {"input_index": 0, "id": req["model"]["materials"][0]["id"],
               "elastic_modulus": "%016x" % int.from_bytes(__import__("struct").pack(">d", req["model"]["materials"][0]["elastic_modulus"]["value"]), "big"),
               "shear_modulus": "%016x" % int.from_bytes(__import__("struct").pack(">d", req["model"]["materials"][0]["shear_modulus"]["value"]), "big"),
               "shear_origin": {"kind": "explicit_g"}, "selection": {"kind": "base"}}]}] and req["materials"] == [])
    print(f"{mode}: receipt_sha256={rp['receipt_sha256']} publication_sha256={body['publication_sha256']}")

fails = [r for r in results if not r[1]]
for name, ok, detail in results:
    print(("PASS " if ok else "FAIL ") + name + (f"  [{detail}]" if detail and not ok else ""))
print(f"TOTAL {len(results)} checks, {len(fails)} failed")
sys.exit(1 if fails else 0)
