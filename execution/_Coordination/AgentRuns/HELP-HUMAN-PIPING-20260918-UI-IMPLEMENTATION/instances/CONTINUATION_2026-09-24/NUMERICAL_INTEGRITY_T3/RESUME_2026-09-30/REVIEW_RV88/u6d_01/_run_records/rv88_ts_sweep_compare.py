"""RV88 (U6d review): compare the desktop sweep TSVs (base 844448112f, candidate 9555b6ffc2)."""
import collections, json, sys
from pathlib import Path
S = Path(sys.argv[1])
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"


def load(name):
    d = collections.OrderedDict()
    for line in (S / name).read_text().splitlines():
        if line.startswith("#"):
            continue
        k, f, v = line.split("\t", 2)
        d.setdefault(k, {})[f] = v
    return d


b, c = load("sweep_d_base.tsv"), load("sweep_d_cand.tsv")
out = []
plain = [k for k in c if "!" not in k]
existing = [k for k in plain if c[k]["id"] != f'{SUCC}']
succ = [k for k in plain if c[k]["id"] == SUCC]
out.append(f"envelopes: cand {len(plain)}, base {len([k for k in b if '!' not in k])}; existing identity {len(existing)}; successor {len(succ)}")
ids = collections.Counter(c[k]["id"] or "(legacy, no producer)" for k in existing)
out.append(f"existing identities: {dict(ids)}")
fields = sorted({f for k in existing for f in c[k]})
diffs = {k: {f: (b[k].get(f), c[k].get(f)) for f in fields if b[k].get(f) != c[k].get(f)} for k in existing}
diffs = {k: v for k, v in diffs.items() if v}
out.append(f"existing-identity envelopes differing from base in any of {len(fields)} fields {fields}: {len(diffs)}")
for k, v in list(diffs.items())[:20]:
    out.append(f"    {k}: {json.dumps(v)[:600]}")
for form in ("receipt", "null", "token0", "tokenlast", "othertoken"):
    keys = [k for k in c if k.endswith("!" + form)]
    t = collections.Counter()
    for k in keys:
        v, bv = c[k], b[k]
        t["n"] += 1
        if form == "othertoken":
            t["identical_to_base"] += v == bv
            continue
        t["route_unsupported"] += v["route"] == '"unsupported"'
        st = json.loads(v["standing_model"]) if not v["standing_model"].startswith("THROW") else {}
        t["standing_finding_DOWNGRADE"] += st.get("findings", [None])[0] == "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
        t["not_fresh"] += v["fresh"] == "false"
        t["analysis_run_refused"] += v["analysis_run"].startswith("THROW")
        t["base_route_supported"] += bv["route"] != '"unsupported"'
        t["base_analysis_run_built"] += not bv["analysis_run"].startswith("THROW")
    out.append(f"form !{form}: {dict(t)}")
sd = collections.Counter()
for k in succ:
    sd[(b.get(k, {}).get("route"), c[k]["route"], c[k]["fresh"], c[k]["analysis_run"][:6], json.loads(c[k]["standing_model"]).get("findings", [None])[0] if not c[k]["standing_model"].startswith("THROW") else "THROW")] += 1
out.append(f"successor envelopes (base route, cand route, fresh, analysis_run, standing finding): {dict(sd)}")
print("\n".join(out))
