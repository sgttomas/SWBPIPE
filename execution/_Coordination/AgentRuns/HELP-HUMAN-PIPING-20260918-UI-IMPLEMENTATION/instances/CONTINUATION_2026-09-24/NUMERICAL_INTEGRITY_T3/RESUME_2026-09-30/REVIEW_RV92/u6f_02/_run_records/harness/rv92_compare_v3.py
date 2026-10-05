"""RV92 (U6f): build the three-language parity table from the per-language
JSONL columns. argv: probes_dir rust.jsonl py.jsonl ts.jsonl out_prefix"""
import json, sys, collections

DIR, RS, PY, TS, OUT = sys.argv[1:6]
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
index = {e["id"]: e for e in json.load(open(f"{DIR}/index.json"))}
load = lambda f: {d["id"]: d for d in map(json.loads, open(f))}
R, P, T = load(RS), load(PY), load(TS)
probes = {}
for pid, e in index.items():
    p = json.load(open(f"{DIR}/{e['file']}"))
    src = p["source"]
    probes[pid] = {"carrier_evidence": "carrier_evidence" in src, "succ": isinstance(src.get("producer"), dict) and src["producer"].get("semantic_contract_id") == SUCC,
                   "inv": p["invocation"] is not None, "group": e["group"], "nrows": len(src.get("results") or [])}


def norm(x):
    """ok / the first error code (a G7 detail keeps its leading code)."""
    if x is None:
        return None
    if x.startswith("ok"):
        return "ok"
    code = x[4:] if x.startswith("err:") else x
    code = code.replace("RPE:", "")
    return code.split(":")[0].split(" ")[0]


def binding_kind(b, cls=None):
    vals = set(map(str, b))
    if not b:
        return "no_rows"
    if len(vals) == 1:
        v = next(iter(vals))
        return "none" if v == "None" else f"every_row:{v}"
    return "mixed:" + ",".join(f"{k}={sum(1 for x in b if str(x) == k)}" for k in sorted(vals))


rows = []
for pid in index:
    r, p, t, meta = R.get(pid), P.get(pid), T.get(pid), probes[pid]
    if not (r and p and t):
        rows.append({"id": pid, "missing": [n for n, x in (("rust", r), ("python", p), ("ts", t)) if not x]})
        continue
    row = {"id": pid, "group": meta["group"], "succ": meta["succ"], "inv": meta["inv"], "carrier_evidence": meta["carrier_evidence"]}
    row["transport"] = (norm(r["transport"]), norm(p["transport"]), norm(t["transport"]))
    row["transport_validator_ts"] = norm(t.get("transport_validator"))
    row["raw"] = (norm(r["raw"]), norm(p["raw"]), norm(t["raw"]))
    ts_standing = t["standing"]
    row["standing"] = (r["standing"], p["standing"], ts_standing)
    row["standing_ts_findings"] = t["standing_findings"]
    row["binding"] = (binding_kind(r["binding"]), binding_kind([x for x in p["binding"]]), binding_kind(t["binding"]))
    row["binding_rp_equal"] = [str(x) for x in r["binding"]] == [str(x) for x in p["binding"]]
    row["binding_rt_equal"] = [str(x) for x in r["binding"]] == [str(x) for x in t["binding"]]
    row["headline"] = (r["headline_binding"], p["headline_binding"], t["headline_binding"])
    row["summary_rp_equal"] = r["summary"] == p["summary"]
    row["summary_rt_equal"] = r["summary"] == t["summary"]
    row["summary"] = (len(r["summary"]), len(p["summary"]), len(t["summary"]))
    row["fresh"] = (r["fresh"], p["fresh"], t["fresh"])
    row["ar_build"] = (norm(p.get("ar_build")), norm(t.get("ar_build")))
    row["ar_validate"] = (norm(p.get("ar_validate")), norm(t.get("ar_validate")))
    row["ar_receipt_equal"] = (p.get("ar_receipt_equal"), t.get("ar_receipt_equal"))
    row["ar_mutations"] = {k: (norm(p.get("ar_mutations", {}).get(k)), norm(t.get("ar_mutations", {}).get(k))) for k in sorted(set(p.get("ar_mutations", {})) | set(t.get("ar_mutations", {})))}
    row["ar_reopen"] = (p.get("ar_reopen"), t.get("ar_reopen"))
    row["ar_v02"] = (norm(p.get("ar_v02")), norm(t.get("ar_v02")))
    row["wrapper_010_py"] = norm(p.get("ar_wrapper_010"))
    row["derive"] = norm(r.get("derive"))
    row["validate"] = norm(r.get("validate"))
    row["derivative_mutations"] = {k: norm(v) for k, v in (r.get("derivative_mutations") or {}).items()}
    row["disclosures"] = r.get("disclosures")
    row["derived_receipt_equal"] = r.get("derived_receipt_equal")
    row["headless_metadata"] = norm(r.get("headless_metadata"))
    row["binding_ms"] = (r["binding_ms"], p["binding_ms"], t["binding_ms"])
    row["ts_output_refusal"] = t.get("output_refusal")
    row["ts_route"] = t.get("route")
    rows.append(row)


# Classification v3 (post-U6f): every difference must be one of the five entries
# (I67-F1, I67-F2, F-U6b-2, F5, RV92-N2-N5) or the case file's N-4 scope sentence.
def classify(row):
    out = []
    if "missing" in row:
        return [("MISSING", ",".join(row["missing"]))]
    s, inv = row["succ"], row["inv"]
    tr, tp, tt = row["transport"]
    ts_route = row["ts_route"]
    if not (tr == tp == tt):
        if s and tp == "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED" and tr == tt:
            out.append(("transport", "declared:F-U6b-2"))
        elif not s and tr == tp == "ok" and tt == "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN":
            out.append(("transport", "declared:RV92-N2-N5 (N-2)"))
        elif s and tp == "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED" and tr == "ok" and tt == "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" and row.get("carrier_evidence"):
            out.append(("transport", "scope:carrier_evidence (Python F-U6b-2)"))
        elif s and tp == "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED" and tr != "ok" and tt == "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED":
            out.append(("transport", "UNNAMED-INHERITED:TS header-route refusal code vs reader code (Python F-U6b-2)"))
        else:
            out.append(("transport", f"UNDECLARED:{row['transport']}"))
    rr, rp_, rt = row["raw"]
    if rr != rp_:
        if rr != "ok" and rp_ != "ok" and rt == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID":
            out.append(("raw R/P", "scope:G7 dispatch text"))
        else:
            out.append(("raw R/P", f"UNDECLARED:{row['raw']}"))
    if (rr == "ok") != (rt == "ok"):
        out.append(("raw R/T", f"HARNESS:{row['raw']}"))
    sr, sp, st = row["standing"]
    if sr != sp:
        out.append(("standing R/P", f"UNDECLARED:{row['standing']}"))
    if s or ts_route == "unsupported":
        if st != sr:
            if s and not inv and sr == "unsupported" and st == "needs_recompute" and row["standing_ts_findings"][:1] == ["RETAINED_PRECISION_VALIDATION_REQUIRED"]:
                out.append(("standing R/T", "declared:I67-F1"))
            else:
                out.append(("standing R/T", f"UNDECLARED:{row['standing']}:{row['standing_ts_findings']}"))
    if not row["binding_rp_equal"]:
        out.append(("binding R/P", f"UNDECLARED:{row['binding']}"))
    if not row["binding_rt_equal"]:
        br, _, bt = row["binding"]
        if s and bt == "every_row:RULE_QUANTITY_NOT_COVERED" and br.startswith("mixed") and rr == "ok":
            out.append(("binding R/T", "declared:I67-F2 (" + ("refused registration" if inv else "none") + ")"))
        elif not s and br.startswith("mixed") and "RULE_SOURCE_BLOCKS_SUMMARY_NOT_RELIABLE" in br and bt == "none" and ts_route == "unsupported":
            out.append(("binding R/T", "declared:RV92-N2-N5 (N-5)"))
        else:
            out.append(("binding R/T", f"UNDECLARED:{row['binding']}"))
    if not row["summary_rp_equal"]:
        out.append(("summary R/P", f"UNDECLARED:{row['summary']}"))
    if not row["summary_rt_equal"]:
        if s and not inv and row["summary"][2] == 0 and rr == "ok":
            out.append(("summary R/T", "declared:I67-F2 (none:summary)"))
        else:
            out.append(("summary R/T", f"UNDECLARED:{row['summary']}:inv={inv}"))
    if row["ar_build"][0] != row["ar_build"][1]:
        a, b = row["ar_build"]
        if a != "ok" and b != "ok":
            out.append(("ar_build P/T", "scope:AnalysisRun builders' refusal codes"))
        else:
            out.append(("ar_build P/T", f"HARNESS-OR-UNDECLARED:{row['ar_build']}"))
    if row["ar_validate"][0] != row["ar_validate"][1]:
        out.append(("ar_validate P/T", f"UNDECLARED:{row['ar_validate']}"))
    for k, (a, b) in row["ar_mutations"].items():
        if a != b and not (a is None or b is None):
            out.append((f"ar_mut:{k} P/T", f"UNDECLARED:{(a, b)}"))
    if row["ar_v02"][0] != row["ar_v02"][1]:
        a, b = row["ar_v02"]
        out.append(("ar_v02 P/T", "scope:AnalysisRun builders' refusal codes" if a != "ok" and b != "ok" else f"UNDECLARED:{row['ar_v02']}"))
    return out


summary = collections.Counter()
with open(OUT + "_table.jsonl", "w") as f:
    for row in rows:
        row["differences"] = classify(row)
        for b, kind in row["differences"]:
            summary[(b, ":".join(kind.split(":")[:2]))] += 1
        f.write(json.dumps(row, sort_keys=True) + "\n")
with open(OUT + "_summary.txt", "w") as f:
    f.write(f"probes {len(rows)}; complete {sum(1 for r in rows if 'missing' not in r)}\n")
    for (b, k), n in sorted(summary.items()):
        f.write(f"{n:4d}  {b:22s} {k}\n")
print(open(OUT + "_summary.txt").read())
