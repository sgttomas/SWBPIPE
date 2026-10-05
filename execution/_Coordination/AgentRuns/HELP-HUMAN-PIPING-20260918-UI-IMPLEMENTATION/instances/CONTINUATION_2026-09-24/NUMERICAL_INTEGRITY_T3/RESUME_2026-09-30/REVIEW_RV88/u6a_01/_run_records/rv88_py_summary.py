"""RV88 (U6a review): summarize the D-U6-1 lane outputs (py_cand.json, py_base.json)."""
import collections, json, sys
from pathlib import Path
S = Path(sys.argv[1])
c = json.loads((S / "py_cand.json").read_text()); b = json.loads((S / "py_base.json").read_text())
t = collections.Counter(); out = []
out.append(f"flag: cand={c['flag']} base={b['flag']}; entries cand={len(c['entries'])} base={len(b['entries'])}")
for ce, be in zip(c["entries"], b["entries"]):
    assert ce["id"] == be["id"]
    pub, exp = ce["public"], ce["expected"]
    t["cand public == cand draft"] += ce["public"] == ce["draft"]
    t["cand draft == base draft"] += ce["draft"] == be["draft"]
    t["base public refused at G0 SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"] += (be["public"]["kind"], be["public"].get("gate"), be["public"].get("code")) == ("refuse", "G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    t[f"kind {ce['kind']}"] += 1
    if ce["kind"] == "mutations":
        t["mutations matching the corpus's expected gate and code"] += bool(exp) and pub["kind"] == "refuse" and (pub["gate"], pub["code"]) == (exp["gate"], exp["code"])
    else:
        t[f"{ce['kind']} passing"] += pub["kind"] == "pass"
    ft = ce["public_flag_true"]
    if pub["kind"] == "refuse":
        t["refusals unchanged with the flag forced True"] += ft == pub
    else:
        t["passes"] += 1
        t["passes with eligibility False and standing needs_recompute"] += pub["result"]["numerical_eligible"] is False and pub["result"]["standing"] == "needs_recompute"
        a, f = dict(pub["result"]), dict(ft["result"])
        el = f["numerical_eligible"]
        for k in ("numerical_eligible", "standing"):
            a.pop(k); f.pop(k)
        t["passes identical with the flag forced True except eligibility/standing"] += ft["kind"] == "pass" and a == f
        t["passes eligible with the flag forced True"] += el
        t["... of which invocation-bound"] += el and pub["result"]["invocation_bound"]
        t["passes without invocation eligible with the flag forced True"] += el and not pub["result"]["invocation_bound"]
for k, v in t.items():
    out.append(f"{k}: {v}")
for mode, m in c["milestone"].items():
    pi = m["public_inv"]
    cnt = collections.Counter(x["class"] for x in pi["result"]["classifications"])
    out.append(f"milestone {mode}: public(inv) {pi['kind']} eligible={pi['result']['numerical_eligible']} standing={pi['result']['standing']}; public(no inv) eligible={m['public_noinv']['result']['numerical_eligible']}; classes {dict(sorted(cnt.items()))}; public==draft {m['draft_inv'] == pi}; Rust-derivative receipt reattached: equal={m.get('receipt_equal')} revalidates identically={m.get('back_out_equal')}")
for mode, m in b["milestone"].items():
    out.append(f"BASE milestone {mode}: public {m['public_inv']['kind']} {m['public_inv'].get('gate')} {m['public_inv'].get('code')}; draft {m['draft_inv']['kind']}")
print("\n".join(out))
