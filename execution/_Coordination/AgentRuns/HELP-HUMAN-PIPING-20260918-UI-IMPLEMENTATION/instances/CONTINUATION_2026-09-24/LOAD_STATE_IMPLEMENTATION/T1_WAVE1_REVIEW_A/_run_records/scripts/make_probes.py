"""Reviewer probes for the load-reference-source-1 readers (T1_WAVE1_REVIEW_A).

Generates adversarial envelopes from committed producer raws. The reseal uses
the reviewer's own JCS/sha256 (indep_hash.py), not any project reader code.
Usage: make_probes.py <WORKING_ROOT> <OUT_DIR>
All inputs are derived from committed invented fixtures.
"""
import copy, json, pathlib, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from indep_hash import dhash

root = pathlib.Path(sys.argv[1]).resolve()
out = pathlib.Path(sys.argv[2]); out.mkdir(parents=True, exist_ok=True)
J = "openpipestress.result_semantics/0.3.0/load-reference-source-1"
JP = "resolved_straight_load_state_source_v1"
LR = "openpipestress.result_semantics/0.3.0/load-reference-1"
LRP = "resolved_straight_load_state_v1"
PS = "openpipestress.result_semantics/0.3.0/physics-source-1"
PSP = "exact_straight_pressure_v2"
SB = "openpipestress.result_semantics/0.3.0/source-blocks-1"
SBP = "product_preview_mechanics_v1"
EXACT = "retained_source_blocks_exact_v1"


def raw(rel):
    return json.loads((root / rel).read_text())


def joined(name):
    return raw(f"fixtures/product_preview/load_reference_source/{name}.raw.json")


def reseal(d, physical="joined", publication=True, receipt=True):
    body = d["source_block_recovery"]["body"]
    ev = d.get("contract_evidence")
    if physical:
        for case, exact in zip(body["cases"], ev["exact_cases"]):
            pressure = [p for p in ev["pressure"] if p["load_case_id"] == exact["load_case_id"]]
            if physical == "joined":
                rec = next(r for r in ev["load_reference_states"] if r["load_case_id"] == exact["load_case_id"])
                case["physical_evidence_sha256"] = dhash("load_reference_source_case_evidence_v1",
                                                         {"exact_case": exact, "pressure": pressure, "load_reference_state": rec})
            else:
                case["physical_evidence_sha256"] = dhash("physics_source_case_evidence_v1",
                                                         {"exact_case": exact, "pressure": pressure})
    if publication:
        body["publication_sha256"] = dhash("source_blocks_publication_v1",
                                           {k: v for k, v in d.items() if k != "source_block_recovery"})
    if receipt:
        d["source_block_recovery"]["receipt_sha256"] = dhash("source_blocks_receipt_v1", body)
    return d


probes = {}

# --- A. relabels in all directions, raw and resealed ----------------------
d = joined("mixed-sparse_interactive")
d["producer"]["semantic_contract_id"] = SB; d["formulation_basis"]["profile_id"] = SBP
d["source_block_recovery"]["body"]["policy"] = "SOURCE-BLOCKS-1"
probes["A01-joined-as-source-blocks-resealed"] = reseal(d, physical=None)

d = joined("mixed-sparse_interactive")
d["producer"]["semantic_contract_id"] = PS; d["formulation_basis"]["profile_id"] = PSP
d["source_block_recovery"]["body"]["policy"] = "PHYSICS-SOURCE-1"
del d["contract_evidence"]["load_reference_states"]
probes["A02-joined-as-physics-source-records-stripped-resealed"] = reseal(d, physical="physics")

d = raw("fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json")
d["producer"]["semantic_contract_id"] = J; d["formulation_basis"]["profile_id"] = JP
d["source_block_recovery"]["body"]["policy"] = "LOAD-REFERENCE-SOURCE-1"
d["contract_evidence"]["load_reference_states"] = []
probes["A03-physics-source-as-joined-empty-records-resealed"] = reseal(d, physical="joined") if False else None
# physics-source has no records: reseal with joined domain requires records; use per-case graft
d = raw("fixtures/product_preview/physics_source/n05-sparse_interactive.raw.json")
j = joined("n05-sparse_interactive")
d["producer"]["semantic_contract_id"] = J; d["formulation_basis"]["profile_id"] = JP
d["source_block_recovery"]["body"]["policy"] = "LOAD-REFERENCE-SOURCE-1"
d["contract_evidence"]["load_reference_states"] = copy.deepcopy(j["contract_evidence"]["load_reference_states"])
probes["A03-physics-source-as-joined-grafted-records-resealed"] = reseal(d)

sbs = sorted((root / "fixtures/product_preview/source_blocks").glob("*.raw.json")) if (root / "fixtures/product_preview/source_blocks").exists() else []
if sbs:
    d = json.loads(sbs[0].read_text())
    d["producer"]["semantic_contract_id"] = J; d["formulation_basis"]["profile_id"] = JP
    d["source_block_recovery"]["body"]["policy"] = "LOAD-REFERENCE-SOURCE-1"
    probes["A04-source-blocks-as-joined-resealed"] = reseal(d, physical=None)

# joined -> load-reference-1 full downgrade forgery: strip receipt, convert the
# selected case to a not-joined record with the ordinary method and NOT_JOINED
# diagnostic. LR-1 carries no hashes, so only structure can refuse it.
d = joined("n05-sparse_interactive")
d["producer"]["semantic_contract_id"] = LR; d["formulation_basis"]["profile_id"] = LRP
del d["source_block_recovery"]
for e in d["contract_evidence"]["exact_cases"]:
    del e["recovery_method"]
for r in d["contract_evidence"]["load_reference_states"]:
    r["solve"]["recovery_method"] = "ordinary_sparse_structural_v1"
    r["source_recovery"] = {"status": "not_joined", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}
for x in d["diagnostics"]:
    if x["code"] == "SOURCE_BLOCK_RECOVERY_SELECTED":
        x["code"] = "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"; x["id"] = "diagnostic:load-state:case:source-recovery-not-joined"
probes["A05-joined-downgraded-to-load-reference-forgery"] = d

# joined with PS-domain physical hashes but joined policy (resealed)
d = joined("mixed-dense_scrutiny")
probes["A06-joined-physical-hash-physics-domain-resealed"] = reseal(d, physical="physics")

# joined id + joined profile, receipt policy PHYSICS-SOURCE-1, fully resealed
d = joined("mixed-dense_scrutiny"); d["source_block_recovery"]["body"]["policy"] = "PHYSICS-SOURCE-1"
probes["A07-joined-policy-physics-source-fully-resealed"] = reseal(d)

# LR id with joined profile; PS id with joined profile; joined profile under SB id
for tag, cid in [("A08-lr-id-joined-profile", LR), ("A09-ps-id-joined-profile", PS), ("A10-sb-id-joined-profile", SB)]:
    d = joined("n05-sparse_interactive"); d["producer"]["semantic_contract_id"] = cid
    probes[tag] = reseal(d, physical=None)

# --- B. contract facts the producer can never emit (resealed) --------------
# B01 a selected case whose ordinary attempt passed its checks.
d = joined("mixed-sparse_interactive")
body = d["source_block_recovery"]["body"]
body["cases"][0]["ordinary_attempt"]["outcome"] = "checks_passed"
d["numerical_quality"]["cases"][0]["solve_quality"] = "checks_passed"
d["numerical_quality"]["status"] = "checks_passed"
for x in d["diagnostics"]:
    if x["id"] == "diagnostic:numerical-integrity:case":
        x["code"] = "NUMERICAL_INTEGRITY_CHECKS_PASSED"; x["severity"] = "info"
probes["B01-selected-case-ordinary-checks-passed-resealed"] = reseal(d)

# B02 an ordinary (not-joined) case of a joined envelope that is sensitive.
d = joined("mixed-sparse_interactive")
body = d["source_block_recovery"]["body"]
body["cases"][1]["ordinary_attempt"]["outcome"] = "sensitive"
d["numerical_quality"]["cases"][1]["solve_quality"] = "sensitive"
for x in d["diagnostics"]:
    if x["id"] == "diagnostic:numerical-integrity:case:ordinary-pressure":
        sens = next(y for y in d["diagnostics"] if y["id"] == "diagnostic:numerical-integrity:case")
        x["code"] = sens["code"]; x["severity"] = sens["severity"]
probes["B02-ordinary-case-sensitive-resealed"] = reseal(d)

# B02b only the numerical-quality label changed for the ordinary case (receipt untouched).
d = joined("mixed-sparse_interactive")
d["numerical_quality"]["cases"][1]["solve_quality"] = "sensitive"
probes["B02b-ordinary-case-quality-label-sensitive-resealed"] = reseal(d)

# B03 the pressure region moved onto the selected case (non-empty inventory).
d = joined("mixed-sparse_interactive")
ev = d["contract_evidence"]
reg = ev["pressure"][0]; reg["load_case_id"] = "case"
m0 = copy.deepcopy(ev["exact_cases"][0]["pipe_materials"][0])
del m0["material_selection_kind"]; del m0["resolved_eigenstrain"]
m0["temperature_basis"] = "resolved_member_state"; reg["materials"] = [m0]
c = next(c for c in ev["load_reference_states"][1]["contributions"] if c["owner_kind"] == "pressure_region")
ev["load_reference_states"][1]["contributions"].remove(c)
ev["load_reference_states"][0]["contributions"].append(c)
probes["B03-pressure-region-on-selected-case-resealed"] = reseal(d)

# B04 a selected case that also carries SOURCE_BLOCK_RECOVERY_UNAVAILABLE.
d = joined("n05-sparse_interactive")
d["diagnostics"].append({"id": "diagnostic:source-recovery:case", "code": "SOURCE_BLOCK_RECOVERY_UNAVAILABLE",
                         "severity": "info", "message": "invented", "affected_refs": ["case"]})
probes["B04-selected-case-with-unavailable-diagnostic-resealed"] = reseal(d)

# B05 a selected case with a NOT_JOINED-coded diagnostic under a different id.
d = joined("n05-sparse_interactive")
d["diagnostics"].append({"id": "diagnostic:invented", "code": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED",
                         "severity": "info", "message": "invented", "affected_refs": ["case"]})
probes["B05-extra-not-joined-diagnostic-other-id-resealed"] = reseal(d)

# B06 both cases selected in mixed (ordinary case relabelled selected everywhere).
d = joined("mixed-sparse_interactive")
ev = d["contract_evidence"]; body = d["source_block_recovery"]["body"]
ev["exact_cases"][1]["recovery_method"] = EXACT
ev["load_reference_states"][1]["solve"]["recovery_method"] = EXACT
ev["load_reference_states"][1]["source_recovery"] = {"status": "selected", "method": EXACT}
body["cases"][1]["selected_method"] = EXACT
for x in d["diagnostics"]:
    if x["code"] == "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED":
        x["code"] = "SOURCE_BLOCK_RECOVERY_SELECTED"; x["id"] = "diagnostic:source-recovery:case:ordinary-pressure:selected"
probes["B06-ordinary-case-promoted-to-selected-resealed"] = reseal(d)

# B07 documented limit: a consistent edit of a prescribed motion (record value
# and its contribution) after a full reseal. The reader cannot re-derive.
d = joined("eigen_motion-sparse_interactive")
r = d["contract_evidence"]["load_reference_states"][0]
r["support_components"][0]["prescribed_value"] = 0.002
next(c for c in r["contributions"] if c["source_id"] == "support_state:anchor:UY")["value"] = 0.002
probes["B07-LIMIT-prescribed-motion-consistent-edit-resealed"] = reseal(d)

# B08 documented limit: a member thermal/fit strain split changed consistently.
d = joined("eigen_motion-sparse_interactive")
r = d["contract_evidence"]["load_reference_states"][0]
r["reference_configuration_id"] = r["reference_configuration_id"]  # unchanged anchor
r["provenance"] = "invented"
probes["B08-LIMIT-record-provenance-resealed"] = reseal(d)

# --- C. parity edges -------------------------------------------------------
d = joined("n05-sparse_interactive"); d["zz_review_extra"] = 2 ** 60
probes["C01-top-level-integer-above-2p53"] = d
d = joined("n05-sparse_interactive"); d["source_block_recovery"]["body"]["invocation_work"]["charged"] = 2 ** 60
probes["C02-receipt-integer-above-2p53"] = d
d = joined("n05-sparse_interactive"); d["zz_review_extra"] = {"x": 1}
probes["C03-unknown-top-level-key-resealed"] = reseal(d)
d = joined("n05-sparse_interactive"); d["contract_evidence"]["exact_cases"][0]["recovery_method"] = 1
probes["C04-exact-recovery-method-number"] = d
d = joined("n05-sparse_interactive"); d["source_block_recovery"] = None
probes["C05-receipt-null"] = d
d = joined("n05-sparse_interactive"); d["source_block_recovery"] = []
probes["C06-receipt-array"] = d
d = joined("n05-sparse_interactive"); d["source_block_recovery"]["body"] = {"policy": "LOAD-REFERENCE-SOURCE-1"}
probes["C07-receipt-body-policy-only"] = d
d = joined("mixed-sparse_interactive"); d["contract_evidence"]["load_reference_states"][1]["load_case_id"] = True
probes["C08-record-case-id-boolean"] = d
d = joined("n05-sparse_interactive"); d["source_block_recovery"]["body"]["cases"][0]["selected_method"] = None
probes["C09-receipt-selected-method-null-resealed"] = reseal(d, physical=None, publication=False)

for k, v in probes.items():
    if v is None:
        continue
    (out / f"{k}.json").write_text(json.dumps(v, ensure_ascii=False))
print(len([v for v in probes.values() if v is not None]), "probes written")
