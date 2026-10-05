#!/usr/bin/env python3
"""I61 07h: the Python outcome delta. 07g reader on 07g against the 07h reader on 07h, and both readers on 07h."""
import json, sys
L = sys.argv[1]; OUT = sys.argv[2]
o = {k: json.load(open(f"{L}/py_outcomes_{k}_reader.json")) for k in ("07g_old", "07h_old", "07g_new", "07h_new")}
rep = {"readers": {k: v["reader_sha256"] for k, v in o.items()}, "corpora": {k: v["corpus_sha256"] for k, v in o.items()}}
def diff(a, b):
    d = []
    for kind in ("cases", "mutations", "must_pass"):
        for key in sorted(set(a[kind]) | set(b[kind])):
            if a[kind].get(key, {}).get("observed") != b[kind].get(key, {}).get("observed") or \
               a[kind].get(key, {}).get("classifications_equal") != b[kind].get(key, {}).get("classifications_equal"):
                d.append({"kind": kind, "id": key, "a": a[kind].get(key, {}).get("observed"), "b": b[kind].get(key, {}).get("observed")})
    return d
rep["07g_reader_on_07g__vs__07h_reader_on_07h"] = diff(o["07g_old"], o["07h_new"])
rep["07g_reader__vs__07h_reader__on_07h"] = diff(o["07h_old"], o["07h_new"])
rep["07g_reader__vs__07h_reader__on_07g"] = diff(o["07g_old"], o["07g_new"])
rep["07h_reader_on_07h_mismatches"] = [k for kind in ("mutations", "must_pass") for k, v in o["07h_new"][kind].items() if not v["match"]]
rep["07h_new_entries"] = {k: o["07h_new"][kind][k] for kind in ("mutations", "must_pass") for k in o["07h_new"][kind] if k not in o["07g_old"][kind]}
json.dump(rep, open(OUT, "w"), indent=1); open(OUT, "a").write("\n")
for k in ("07g_reader_on_07g__vs__07h_reader_on_07h", "07g_reader__vs__07h_reader__on_07h", "07g_reader__vs__07h_reader__on_07g", "07h_reader_on_07h_mismatches"):
    print(k, json.dumps(rep[k]))
print(json.dumps(rep["07h_new_entries"]))
