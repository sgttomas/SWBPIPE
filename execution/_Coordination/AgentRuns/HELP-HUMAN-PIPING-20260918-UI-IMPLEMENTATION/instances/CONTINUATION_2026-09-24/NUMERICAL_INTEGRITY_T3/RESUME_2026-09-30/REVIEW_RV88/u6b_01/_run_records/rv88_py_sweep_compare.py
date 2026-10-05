"""RV88 (U6b review): compare the Python carrier sweeps (base cb03315779, candidate c89a7a986c)."""
import collections, json, sys
from pathlib import Path
S = Path(sys.argv[1]); SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
def load(n):
    d = collections.OrderedDict()
    for line in (S / n).read_text().splitlines():
        if line.startswith("#"): continue
        k, f, v = line.split("\t", 2); d.setdefault(k, {})[f] = v
    return d
b, c = load("pysweep_c_cand.tsv"), load("pysweep_b_cand_p.tsv")
out = []
plain = [k for k in c if "!" not in k]
existing = [k for k in plain if c[k]["id"] != SUCC]
out.append(f"envelopes: cand {len(plain)}, base {len([k for k in b if '!' not in k])}; existing identity {len(existing)} ({dict(collections.Counter(c[k]['id'] or '(legacy)' for k in existing))}); successor {len(plain)-len(existing)}")
diffs = {k: {f: (b[k].get(f), c[k].get(f)) for f in set(b[k]) | set(c[k]) if b[k].get(f) != c[k].get(f)} for k in existing}
diffs = {k: v for k, v in diffs.items() if v}
out.append(f"existing-identity envelopes differing from base: {len(diffs)}")
for k, v in list(diffs.items())[:10]: out.append(f"    {k}: {json.dumps(v)[:500]}")
DG = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
for form in ("receipt", "null", "token0", "tokenlast", "othertoken", "r2notice"):
    keys = [k for k in c if k.endswith("!" + form)]
    t = collections.Counter()
    for k in keys:
        v, bv = c[k], b[k]; t["n"] += 1
        if form in ("othertoken", "r2notice"):
            t["identical_to_base"] += v == bv
            if v != bv: t["differs:" + ",".join(sorted(f for f in v if v[f] != bv.get(f)))] += 1
            continue
        t["raw_DG"] += DG in v["raw"]
        t["raw_admitted"] += v["raw"].startswith("ok")
        t["standing_unsupported"] += v["standing_nq"] == 'ok:"unsupported"'
        t["ar_refused"] += v["analysis_run"].startswith("err")
        t["wrapper_refused"] += v["wrapper_0_1_0"].startswith("err")
        t["base_raw_admitted"] += bv["raw"].startswith("ok")
        if v["raw"].startswith("ok"): t["ADMITTED_ids:" + (v["id"] or "(legacy)")] += 1
    out.append(f"form !{form}: {dict(t)}")
sd = collections.Counter((b[k]["raw"][:60], c[k]["raw"][:60], c[k]["standing_nq"], c[k]["analysis_run"][:6]) for k in plain if c[k]["id"] == SUCC)
out.append(f"successor envelopes (base raw, cand raw, cand standing, AR): {dict(sd)}")
print("\n".join(out))
