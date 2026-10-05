#!/usr/bin/env python3
"""RV82 item 4: per-row class parity py = rs = ts = the certificate's verdicts."""
import json, os, re, collections
D, OUT = os.environ["RV82_READ_DIR"], os.environ["OUT"]
MAP = {"RelativeVerified": "relative_verified", "InputDerived": "input_derived"}
def cert_class(c):
    if c == "None": return "non_quantity"
    if c.startswith("Some(AbsoluteVerified"): return "absolute_verified"
    return MAP[re.match(r"Some\((\w+)\)", c).group(1)]
for m in ["sparse_interactive", "dense_scrutiny"]:
    f = f"u1_milestone_{m}.json"
    rd = {r: [l.split("|") for l in open(f"{D}/{f}.{r}_classes.txt").read().splitlines()] for r in ("py", "rs", "ts")}
    facts = json.load(open(f"{OUT}/facts_{m}.json"))
    cert = [(v["row_id"], cert_class(v["class"])) for v in facts["verdicts"]]
    same = rd["py"] == rd["rs"] == rd["ts"]
    agree = [(x[0], x[3]) for x in rd["py"]] == cert
    print(f"{m}: rows py={len(rd['py'])} rs={len(rd['rs'])} ts={len(rd['ts'])} cert={len(cert)}; py=rs=ts (id|normalized|scale|class): {same}; "
          f"= certificate verdict classes: {agree}; parity {sum(1 for a, b in zip([(x[0], x[3]) for x in rd['py']], cert) if a == b)}/{len(cert)}; "
          f"classes {dict(collections.Counter(c for _, c in cert))}")
