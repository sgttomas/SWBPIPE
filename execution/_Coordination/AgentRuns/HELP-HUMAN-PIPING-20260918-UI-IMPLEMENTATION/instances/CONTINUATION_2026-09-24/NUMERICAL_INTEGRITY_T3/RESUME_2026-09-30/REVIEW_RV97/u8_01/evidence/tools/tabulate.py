"""RV97: tabulate the probe's DIRECT and STAGES records. Usage: tabulate.py LOG"""
import json, re, sys
recs = {"DIRECT": [], "STAGES": [], "INPUT": []}
for line in open(sys.argv[1], encoding="utf-8"):
    m = re.search(r"RV97 (DIRECT|STAGES|INPUT) (\{.*\})\s*$", line)
    if m:
        recs[m.group(1)].append(json.loads(m.group(2)))
for r in recs["INPUT"]:
    print("INPUT", r["label"], "equal" if r["equal"] else "DIFFERENT", r["mine_sha256"][:12])
print()
for r in recs["DIRECT"]:
    a = r["admission"]
    print("DIRECT", r["label"], r["mode"], "| adm", a["profile"], a["refusal"], a["domain"], a["required"], "census", a["census_complete"],
          "| W1", r["retained"], "| succ", r["successor"], "| tally", r["tally"], "mine", r["rv97_counters"],
          "| rest", r["at_rest_before"], r["at_rest_after"], "| B'", r["b_prime"],
          "| plain", r["plain"]["sha256"][:12], r["plain"]["results"], "| pub", r["published"]["sha256"][:12], "n1", r["published"]["n1"],
          "eqplain", r["published"]["equals_plain"], "|", r["published"]["oracle"])
    for n in r["native_direct"]:
        print("    native(direct):", n[:400])
    if "successor_detail" in r:
        d = r["successor_detail"]
        print("    receipt", d["receipt_sha256"], "published_is_successor", d["published_is_successor"], "rust", json.dumps(d["rust_reader"]))
print()
for r in recs["STAGES"]:
    print("STAGES", r["label"], r["mode"], "| observed=plain", r["observed_is_plain"], "coexist", r["coexistence"], "| stage", r.get("stage"),
          "| eq_direct", r.get("equals_direct_successor"), "| receipt", r.get("receipt_sha256"))
    for n in r.get("native", []):
        print("    native(read):", n[:600])
    if "detail" in r:
        print("    detail:", r["detail"][:700])
    if "reader" in r:
        print("    reader:", json.dumps(r["reader"]))
