"""I79 T3-SI1b: compare base and candidate differential dumps.

Evaluator dumps (`label\tbits\tresult` or `label\tPANIC`): every line must be
byte-identical except where the base panicked; there the candidate must be a
blocked result (value none) whose findings include NonFiniteInput, and the
candidate must have no PANIC line at all. The base panic site of each such
line (from `<dump>.panics`) is tabulated against the candidate's new finding.

Runner dumps: the plain fixture-row dump must be identical; in the extreme
dump only base PANIC lines may differ, and each must become a JSON result
with a blocking NonFiniteInput evaluator finding.
"""
import collections, hashlib, json, re, sys, pathlib

S = pathlib.Path(sys.argv[1])
report = {}

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

FIND = re.compile(r'EvaluationFinding \{ code: (\w+), subject_id: "((?:[^"\\]|\\.)*)", message: "((?:[^"\\]|\\.)*)" \}')

def classify(rest):
    if rest == "PANIC":
        return "panic"
    bits = rest.split("\t", 1)[0]
    if bits == "none":
        return "blocked"
    if bits in ("true", "false"):
        return bits
    return "quantity"

for name in ("dump", "extreme", "table"):
    base = (S / f"diff_ee/base_{name}.tsv").read_text().splitlines()
    cand = (S / f"diff_ee/cand_{name}.tsv").read_text().splitlines()
    assert len(base) == len(cand), name
    sites = {}
    for line in (S / f"diff_ee/base_{name}.tsv.panics").read_text().splitlines():
        label, site, _msg, _input = line.split("\t", 3)
        sites[label] = site
    mix_b, mix_c = collections.Counter(), collections.Counter()
    differing, bad, table = 0, [], collections.Counter()
    for b, c in zip(base, cand):
        lb, rb = b.split("\t", 1)
        lc, rc = c.split("\t", 1)
        assert lb == lc, (lb, lc)
        mix_b[classify(rb)] += 1
        mix_c[classify(rc)] += 1
        if b == c:
            continue
        differing += 1
        if rb != "PANIC" or classify(rc) != "blocked":
            bad.append(lb)
            continue
        findings = FIND.findall(rc)
        new = [f for f in findings if f[0] == "NonFiniteInput"]
        if not new:
            bad.append(lb)
            continue
        others = tuple(f[0] for f in findings if f[0] != "NonFiniteInput")
        last = findings[-1][0] == "NonFiniteInput"
        table[(sites[lb], new[0][0], new[0][1] if new[0][1] in ("divide",) else "<table_id>", new[0][2], len(findings), others, last)] += 1
    report[f"evaluator_{name}"] = {
        "lines": len(base),
        "base_sha256": sha(S / f"diff_ee/base_{name}.tsv"),
        "cand_sha256": sha(S / f"diff_ee/cand_{name}.tsv"),
        "identical_lines": len(base) - differing,
        "differing_lines": differing,
        "differing_not_base_panic_to_blocked_nonfinite": bad,
        "base_panics": mix_b["panic"],
        "cand_panics": mix_c["panic"],
        "base_mix": dict(sorted(mix_b.items())),
        "cand_mix": dict(sorted(mix_c.items())),
        "base_site_to_cand_finding": [
            {"base_site": k[0], "code": k[1], "subject": k[2], "message": k[3], "findings_on_line": k[4],
             "other_findings": list(k[5]), "new_finding_is_last": k[6], "count": v}
            for k, v in sorted(table.items())],
    }

# Interval mode over every evaluator input (four overlay variants each).
for name in ("dump", "extreme", "table"):
    bi, ci = S / f"diff_ee/base_{name}.tsv.interval", S / f"diff_ee/cand_{name}.tsv.interval"
    lines = bi.read_text().splitlines()
    report[f"interval_{name}"] = {"lines": len(lines), "base_sha256": sha(bi), "cand_sha256": sha(ci),
                                  "identical": bi.read_bytes() == ci.read_bytes(),
                                  "panics": sum(1 for l in lines if l.endswith("\tPANIC"))}

# Runner.
bp, cp = S / "diff_rcr/base_plain.tsv", S / "diff_rcr/cand_plain.tsv"
report["runner_fixture_rows"] = {"lines": len(bp.read_text().splitlines()), "base_sha256": sha(bp),
                                 "cand_sha256": sha(cp), "identical": bp.read_bytes() == cp.read_bytes()}
base = (S / "diff_rcr/base_extreme.tsv").read_text().splitlines()
cand = (S / "diff_rcr/cand_extreme.tsv").read_text().splitlines()
assert len(base) == len(cand)
differing, bad, table = 0, [], collections.Counter()
cand_panics = sum(1 for c in cand if "\tPANIC\t" in c)
for b, c in zip(base, cand):
    key_b, out_b = b.rsplit("\t", 1)[0], b
    if b == c:
        continue
    differing += 1
    fb = b.split("\t")
    fc = c.split("\t")
    assert fb[:5] == fc[:5]
    if fb[5] != "PANIC":
        bad.append(fb[:5]); continue
    result = json.loads(fc[5])
    blocked = [(i, f) for i, chk in enumerate(result["checks"]) for f in chk["evaluator_findings"]
               if f["code"] == "NonFiniteInput"]
    if not blocked:
        bad.append(fb[:5]); continue
    i, f = blocked[0]
    chk = result["checks"][i]
    others = [(x["check_id"], x["status"]) for j, x in enumerate(result["checks"]) if j != i]
    table[(fb[0].split("_")[0] if fb[0].startswith("table") else "ratio_packs", fb[6], f["code"], f["severity"],
           f["subject_id"] if f["subject_id"] == "divide" else "<table_id>", chk["status"], chk["acceptability_relation"],
           json.dumps(chk["diagnostic_codes"]), json.dumps(others))] += 1
modes = collections.Counter(b.split("\t")[4] for b in base)
bounded_identical = all(b == c for b, c in zip(base, cand) if b.split("\t")[4] not in ("plain", "b0"))
bounded_panics = sum(1 for b in base if b.split("\t")[4] not in ("plain", "b0") and "\tPANIC\t" in b)
report["runner_extreme"] = {
    "lines_by_mode": dict(sorted(modes.items())),
    "positive_bound_lines_identical": bounded_identical, "positive_bound_base_panics": bounded_panics,
    "lines": len(base), "base_sha256": sha(S / "diff_rcr/base_extreme.tsv"), "cand_sha256": sha(S / "diff_rcr/cand_extreme.tsv"),
    "differing_lines": differing, "base_panics": sum(1 for b in base if "\tPANIC\t" in b), "cand_panics": cand_panics,
    "differing_not_base_panic_to_blocked_nonfinite": bad,
    "base_site_to_cand_outcome": [
        {"group": k[0], "base_site": k[1], "code": k[2], "severity": k[3], "subject": k[4], "status": k[5],
         "acceptability_relation": k[6], "diagnostic_codes": k[7], "other_checks": k[8], "count": v}
        for k, v in sorted(table.items())],
}
print(json.dumps(report, indent=1))
