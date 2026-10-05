"""RV88 (U6a review): compare the sweep TSVs (base and candidate, runs 1 and 2).

Usage: python3 rv88_sweep_compare.py <scratch dir>  -> prints the summary.
Run 1 walks identified envelopes; run 2 adds legacy 0.1.0 raw envelopes and
the U3 R-2 notice form. `validate_document_seeded_receipt` is RV88's own
adversarial probe (a receipt placed on an existing identity's derivative), so
it is reported separately from the identity comparison.
"""
import collections
import sys
from pathlib import Path

S = Path(sys.argv[1])
SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
DG = "err:RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
ADV = "validate_document_seeded_receipt"


def load(name):
    d = collections.OrderedDict()
    for line in (S / name).read_text().splitlines():
        if line.startswith("#"):
            continue
        k, f, v = line.split("\t", 2)
        d.setdefault(k, {})[f] = v
    return d


def diff(a, b, skip=()):
    return {f: (a.get(f), b.get(f)) for f in set(a) | set(b) if f not in skip and a.get(f) != b.get(f)}


out = []
runs = {n: load(f"sweep_{n}.tsv") for n in ("base_1", "cand_1", "base_2", "cand_2") if (S / f"sweep_{n}.tsv").exists()}
for run in ("1", "2"):
    if f"base_{run}" not in runs:
        continue
    b, c = runs[f"base_{run}"], runs[f"cand_{run}"]
    plain = [k for k in c if "!" not in k]
    existing = [k for k in plain if c[k]["id"] != SUCC]
    legacy = [k for k in existing if c[k]["id"] == ""]
    out.append(f"run {run}: envelopes cand={len(plain)} base={len([k for k in b if '!' not in k])}; existing identity {len(existing)} (legacy 0.1.0: {len(legacy)}); successor {len(plain) - len(existing)}")
    missing = [k for k in existing if k not in b]
    out.append(f"run {run}: existing envelopes absent from base: {missing}")
    ids = collections.Counter(c[k]["id"] or "(legacy 0.1.0)" for k in existing)
    out.append(f"run {run}: identities {dict(ids)}")
    dd = {k: diff(b[k], c[k], skip=(ADV,)) for k in existing}
    dd = {k: v for k, v in dd.items() if v}
    out.append(f"run {run}: existing-identity envelopes differing from base (excluding the adversarial probe): {len(dd)}")
    for k, v in dd.items():
        out.append(f"    {k}: {v}")
    adv = collections.Counter((b[k].get(ADV), c[k].get(ADV)) for k in existing if ADV in c[k])
    out.append(f"run {run}: adversarial seeded-receipt probe (base -> cand): {dict(adv)}")
    acc = sum(1 for k in existing if c[k]["for_source"].startswith("ok"))
    der = sum(1 for k in existing if c[k]["derive"].startswith("ok"))
    out.append(f"run {run}: existing accepted by for_source {acc}, derived {der}")
    for form in ("receipt", "null", "token0", "tokenlast", "othertoken", "r2notice"):
        keys = [k for k in c if k.endswith("!" + form)]
        if not keys:
            continue
        t = collections.Counter()
        for k in keys:
            v, bv = c[k], b[k]
            t["n"] += 1
            if form in ("othertoken", "r2notice"):
                t["identical_to_base_excl_adv"] += not diff(bv, v, skip=(ADV,))
            else:
                t["for_source_DG"] += v["for_source"] == DG
                t["metadata_DG"] += v["for_source_metadata"] == DG
                t["derive_DG"] += v["derive"] == DG
                st = {v[f] for f in v if f.startswith("standing_") and f != "standing_reason"}
                t["standing_all_unsupported"] += st == {"unsupported"}
                t["base_for_source_ok"] += bv["for_source"].startswith("ok")
                t["base_derive_ok"] += bv["derive"].startswith("ok")
        out.append(f"run {run}: form !{form}: {dict(t)}")
for lane in ("base", "cand"):
    if f"{lane}_1" in runs and f"{lane}_2" in runs:
        r1, r2 = runs[f"{lane}_1"], runs[f"{lane}_2"]
        common = [k for k in r1 if k in r2]
        dd = {k: diff(r1[k], r2[k]) for k in common}
        dd = {k: v for k, v in dd.items() if v}
        out.append(f"{lane} run1 vs run2 (same code; determinism): {len(common)} common keys, {len(dd)} differ")
        for k, v in dd.items():
            out.append(f"    {k}: {v}")
print("\n".join(out))
