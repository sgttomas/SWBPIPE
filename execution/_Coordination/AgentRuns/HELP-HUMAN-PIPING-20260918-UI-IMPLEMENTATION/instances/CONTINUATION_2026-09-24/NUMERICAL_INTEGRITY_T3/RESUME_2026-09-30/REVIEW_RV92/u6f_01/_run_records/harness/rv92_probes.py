"""RV92 (U6f): the three-language parity probe set.

Writes one JSON file per probe to <out>/ and <out>/index.json. Each probe is
{id, group, source, invocation|null, requested:[ref], load_cases:[id]}.
'requested' is what Rust and Python standing receive; 'load_cases' is the TS
standing model's load cases (the same ids, in order). Inputs come from the
candidate tree P (argv[1]); resealed probes use the candidate Python reader's
own hash helper (forged, hash-consistent statements)."""
import copy, hashlib, json, os, sys

P = sys.argv[1]
OUT = sys.argv[2]
sys.path.insert(0, P)
from core.analysis_runs import retained_precision as rp  # noqa: E402

SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
BASE = "openpipestress.result_semantics/0.3.0/preview-physics-1"
PHYS = "openpipestress.result_semantics/0.3.0/physics-1"
METHOD = "contribution_preserving_multiprecision_v1"
os.makedirs(OUT, exist_ok=True)
probes = []


def load(rel):
    return json.loads(open(os.path.join(P, rel), "rb").read())


def reseal(src):
    src = copy.deepcopy(src)
    body = src["retained_precision"]["body"]
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in src.items() if k != "retained_precision"})
    src["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return src


def refs(ids):
    return [{"ref_type": "load_case", "ref_id": i} for i in ids]


def add(pid, group, source, invocation=None, ids=None):
    ids = ids if ids is not None else []
    probes.append({"id": pid, "group": group, "source": source, "invocation": invocation, "requested": refs(ids), "load_cases": ids})


def setpath(t, path, v):
    for k in path[:-1]:
        t = t[k]
    t[path[-1]] = v


# 1. The shared case file: every case and every declared-difference entry.
cases = load("fixtures/results/retained_precision_carrier_cases.json")
docs = {}
for fid, spec in cases["fixtures"].items():
    raw = open(os.path.join(P, spec["path"]), "rb").read()
    assert hashlib.sha256(raw).hexdigest() == spec["sha256"], fid
    d = json.loads(raw)
    docs[fid] = d if spec["shape"] == "milestone" else {"source": d, "invocation": None}


def shared(c, fid, pid, group):
    doc = docs[fid]
    src, inv = copy.deepcopy(doc["source"]), copy.deepcopy(doc["invocation"])
    for e in c["edits"]:
        setpath(src if e["target"] == "source" else inv, e["path"], e["value"])
    ids = [lc["id"] for lc in doc["invocation"]["request"]["model"]["load_cases"]] if c["requested"] == "invocation" else [r["ref_id"] for r in c["requested"]]
    add(pid, group, src, inv if c["invocation"] is not None else None, ids)


for c in cases["cases"]:
    shared(c, c["fixture"], "shared:" + c["id"], "shared_case")
for e in cases["declared_differences"]:
    for fid in e["fixtures"]:
        shared(e, fid, f"declared:{e['id']}:{fid}", "declared")

# 2. Own successor probes, both modes.
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = docs[f"milestone_{mode}"]
    s0, inv0 = doc["source"], doc["invocation"]
    ids0 = [lc["id"] for lc in inv0["request"]["model"]["load_cases"]]
    receipt = s0["retained_precision"]

    def succ(name, fn=None, inv=True, ids=None, resealed=False):
        s = copy.deepcopy(s0)
        if fn:
            fn(s)
        if resealed:
            s = reseal(s)
        add(f"own:{mode}:{name}", "own_successor", s, copy.deepcopy(inv0) if inv is True else inv, ids0 if ids is None else ids)

    succ("plain")
    succ("plain_no_invocation", inv=None)
    succ("receipt_sha_zero", lambda s: setpath(s, ["retained_precision", "receipt_sha256"], "0" * 64))
    succ("receipt_sha_zero_no_invocation", lambda s: setpath(s, ["retained_precision", "receipt_sha256"], "0" * 64), inv=None)
    succ("receipt_body_edit", lambda s: setpath(s, ["retained_precision", "body", "work", "charged"], s["retained_precision"]["body"]["work"]["charged"] + 1))

    def body_edit_resealed(s):
        s["retained_precision"]["body"]["work"]["charged"] += 1
        s["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", s["retained_precision"]["body"])
    succ("receipt_body_edit_receipt_resealed", body_edit_resealed)

    def nq(s):
        s["numerical_quality"]["status"] = "checks_passed"
        s["numerical_quality"]["cases"][0].update(solve_quality="checks_passed", structural_status="passive_model_basis", model_matrix_fidelity="represented_equations_retained")
    succ("nq_checks_passed", nq)
    succ("nq_checks_passed_resealed", nq, resealed=True)
    succ("mechanics_incomplete_resealed", lambda s: setpath(s, ["status", "mechanics"], "MODEL_INCOMPLETE"), resealed=True)
    succ("diagnostic_message_edit", lambda s: setpath(s, ["diagnostics", 0, "message"], "edited"))
    succ("contract_evidence_edit_resealed", lambda s: s["contract_evidence"].__setitem__("rv92", 1), resealed=True)
    succ("profile_wrong", lambda s: setpath(s, ["formulation_basis", "profile_id"], "product_preview_mechanics_v1"))
    succ("component_version_wrong", lambda s: setpath(s, ["producer", "component_version"], "0.3.0"))
    succ("schema_0_1_0", lambda s: setpath(s, ["schema_version"], "0.1.0"))
    succ("schema_0_3_0", lambda s: setpath(s, ["schema_version"], "0.3.0"))
    succ("receipt_null", lambda s: setpath(s, ["retained_precision"], None))
    succ("receipt_array", lambda s: setpath(s, ["retained_precision"], []))
    succ("receipt_string", lambda s: setpath(s, ["retained_precision"], "x"))
    succ("receipt_empty", lambda s: setpath(s, ["retained_precision"], {}))
    succ("receipt_absent", lambda s: s.pop("retained_precision"))
    succ("contract_evidence_absent", lambda s: s.pop("contract_evidence"))
    succ("carrier_evidence_added", lambda s: setpath(s, ["carrier_evidence"], {}))
    succ("source_block_recovery_added", lambda s: setpath(s, ["source_block_recovery"], None))
    succ("row_token_removed", lambda s: s["results"][0].pop("recovery_method"))
    succ("row_token_removed_resealed", lambda s: s["results"][0].pop("recovery_method"), resealed=True)
    succ("row_token_other_resealed", lambda s: setpath(s, ["results", 0, "recovery_method"], "other_method"), resealed=True)
    succ("extra_member_resealed", lambda s: setpath(s, ["rv92_extra"], 1), resealed=True)
    succ("requested_twice", ids=ids0 + ids0)
    succ("requested_other", ids=["other"])
    succ("requested_none", ids=[])

    def inv_model_edit():
        i = copy.deepcopy(inv0)
        i["request"]["model"]["load_cases"][0]["id"] = "other"
        return i
    succ("invocation_model_edit", inv=inv_model_edit())
    succ("invocation_model_edit_requested_other", inv=inv_model_edit(), ids=["other"])
    succ("invocation_empty_object", inv={})

    def no_results(s):
        s.pop("results")
    succ("transport_no_results", no_results, inv=None)
    succ("transport_no_results_receipt_zero", lambda s: (s.pop("results"), setpath(s, ["retained_precision", "receipt_sha256"], "0" * 64)), inv=None)

    # Relabels that keep the receipt or the tokens (downgrades).
    def relabel_base(s):
        s["producer"]["semantic_contract_id"] = BASE
        s["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    succ("relabel_preview1_with_receipt_and_tokens", relabel_base)
    succ("relabel_preview1_with_receipt_only", lambda s: (relabel_base(s), [r.pop("recovery_method", None) for r in s["results"]]))
    succ("relabel_preview1_with_tokens_only", lambda s: (relabel_base(s), s.pop("retained_precision")))
    succ("relabel_physics1_with_receipt", lambda s: (setpath(s, ["producer", "semantic_contract_id"], PHYS), setpath(s, ["formulation_basis", "profile_id"], "exact_straight_pressure_v2")))

    def legacy(s):
        for k in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
            s.pop(k, None)
        s["schema_version"] = "0.1.0"
    succ("relabel_legacy010_with_receipt_and_tokens", legacy, inv=None, ids=[])
    succ("relabel_legacy010_with_tokens_only", lambda s: (legacy(s), s.pop("retained_precision")), inv=None, ids=[])

    # The projected base (the reader's G7 view): an ordinary preview-physics-1 envelope.
    def projected(s):
        s.pop("retained_precision")
        relabel_base(s)
        for r in s["results"]:
            r.pop("recovery_method", None)
    succ("projected_base", projected)
    n = len(s0["results"])
    for name, idx, method in (("first", 0, METHOD), ("mid", n // 2, METHOD), ("last", n - 1, METHOD), ("other_method", 0, "other_method"), ("null_method", 0, None)):
        succ(f"projected_base_token_{name}", lambda s, idx=idx, method=method: (projected(s), setpath(s, ["results", idx, "recovery_method"], method)))
    succ("projected_base_receipt_null", lambda s: (projected(s), setpath(s, ["retained_precision"], None)))
    succ("projected_base_receipt_obj", lambda s: (projected(s), setpath(s, ["retained_precision"], copy.deepcopy(receipt))))

    def r2(s):
        projected(s)
        s["diagnostics"].append({"id": f"diagnostic:retained-precision:{ids0[0]}:unavailable", "code": "RETAINED_PRECISION_UNAVAILABLE", "severity": "info",
                                 "message": "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics.",
                                 "source": "core/product_physics", "affected_refs": [ids0[0]]})
    succ("projected_base_r2_notice", r2)

# 3. Every other identity: plain, and each downgrade form.
others = {
    "legacy_0_1": "fixtures/product_preview/invented_mechanics_result.json",
    "precision_1": "fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json",
    "physics_1": "fixtures/results/physics_connected_mechanics_sparse.json",
    "preview_physics_1": "fixtures/results/preview_physics_invented_sparse.json",
    "source_blocks_1": "fixtures/product_preview/source_blocks/n06-sparse_interactive.raw.json",
    "physics_source_1": "fixtures/product_preview/physics_source/n06-sparse_interactive.raw.json",
    "load_reference_1": "fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json",
    "load_reference_source_1": "fixtures/product_preview/load_reference_source/n06-sparse_interactive.raw.json",
}
receipt = docs["milestone_sparse_interactive"]["source"]["retained_precision"]
for name, rel in others.items():
    s0 = load(rel)
    ids = [c["basis_ref"]["ref_id"] for c in (s0.get("numerical_quality") or {}).get("cases", []) if isinstance(c, dict)]
    n = len(s0["results"])

    def other(form, fn=None):
        s = copy.deepcopy(s0)
        if fn:
            fn(s)
        add(f"other:{name}:{form}", "other_identity", s, None, ids)
    other("plain")
    other("receipt_obj", lambda s: setpath(s, ["retained_precision"], copy.deepcopy(receipt)))
    other("receipt_null", lambda s: setpath(s, ["retained_precision"], None))
    other("receipt_string", lambda s: setpath(s, ["retained_precision"], "x"))
    other("token_first", lambda s: setpath(s, ["results", 0, "recovery_method"], METHOD))
    other("token_last", lambda s: setpath(s, ["results", n - 1, "recovery_method"], METHOD))
    other("token_other_method", lambda s: setpath(s, ["results", 0, "recovery_method"], "other_method"))
    other("producer_successor_id", lambda s: setpath(s, ["producer"], dict(s.get("producer") or {}, semantic_contract_id=SUCC)))

index = []
for i, p in enumerate(probes):
    fn = f"p{i:04d}.json"
    with open(os.path.join(OUT, fn), "w") as f:
        json.dump(p, f, separators=(",", ":"))
    index.append({"file": fn, "id": p["id"], "group": p["group"]})
json.dump(index, open(os.path.join(OUT, "index.json"), "w"), indent=0)
print(len(probes), "probes")
