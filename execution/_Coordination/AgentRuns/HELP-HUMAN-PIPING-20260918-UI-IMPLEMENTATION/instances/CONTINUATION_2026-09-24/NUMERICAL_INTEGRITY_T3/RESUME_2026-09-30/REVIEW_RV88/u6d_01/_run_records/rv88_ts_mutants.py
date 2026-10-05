#!/usr/bin/env python3
"""RV88 (U6d review): TS mutants in a scratch lane copy of the candidate.
Re-runs a sample of I67's 103 (read from I67's committed mutants.py) and RV88's
own. Each run: I67's 11 test files plus RV88's zzRv88U6d.test.tsx; kills are
attributed by test file. A load error never counts as an assertion kill.
Usage: rv88_ts_mutants.py <I67 mutants.py> <out.json> i67|own"""
import json, os, subprocess, sys, time

WT = "WT"
LANE = f"{WT}/rv88/d_mut/projects/chirality-piping/apps/desktop"
CAND = f"{WT}/rv88/d_cand/projects/chirality-piping/apps/desktop"
S = f"{WT}/scratch/rv88_u6/u6d"
ENV = dict(os.environ, TMPDIR=f"{S}/tmp", RV88_U6D_OUT="")
ns = {}
src = open(sys.argv[1]).read()
exec(src.split("def run(")[0].replace('T3 = "WT"', f'T3 = "{WT}"'), ns)
I67, FILES = ns["M"], ns["NEW"] + ns["RELATED"]
R, Sv = "src/features/results/", "src/services/"
NRQ, RPS, KSL, LROA = R + "numericalResultQuality.ts", R + "retainedPrecisionStanding.ts", R + "knownSemanticLimitations.ts", R + "loadReferenceOutputAvailability.ts"
OWN = [
    ("V01", NRQ, "source.results.some(row => (row as", "source.results.slice(0, 1).some(row => (row as", "guard: only the first row's token is read"),
    ("V02", NRQ, 'return Object.hasOwn(source, "retained_precision")\n    ||', 'return (source as { retained_precision?: unknown }).retained_precision != null\n    ||', "guard: a null member slips through"),
    ("V03", RPS, "current.text === registration.source.text && current.negativeZeros === registration.source.negativeZeros", "current.text === registration.source.text", "registration: zero sign not fingerprinted"),
    ("V04", RPS, "cases.some((c, index) => !sameJson(c?.basis_ref, requested[index]))", "cases.some(c => !requested.some(r => sameJson(c?.basis_ref, r)))", "standing: requested refs order-insensitive"),
    ("V05", KSL, "if (Number(`${mantissa}e${exponent}`) > b) return", "if (true) return", "text: b printed to nearest, not upward"),
    ("V06", KSL, "return classes ? classBindingRefusal(classes.get(row.id)?.class) : RULE_QUANTITY_NOT_COVERED;", "return classes ? classBindingRefusal(classes.get(row.id)?.class) : null;", "binding: unregistered successor rows bind"),
    ("V07", KSL, '.replace("{unit}", RETAINED_SI_UNIT[unit] ?? unit)', '.replace("{unit}", unit)', "text: b labelled in the row's unit, not SI"),
    ("V08", LROA, ": isRetainedPrecisionRoute(source) ? RETAINED_PRECISION_OUTPUT_REFUSAL : null;", ": null;", "output: successor not refused"),
    ("V09", RPS, 'if (registration.outcome.error !== null) return { standing: "unsupported"', 'if (registration.outcome.error !== null) return { standing: "needs_recompute"', "standing: a reader refusal reads needs_recompute"),
]


def run(tag):
    out = f"{S}/mut_runs/{tag}.json"
    os.makedirs(f"{S}/mut_runs", exist_ok=True)
    if os.path.exists(out): os.remove(out)
    files = FILES + [R + "zzRv88U6d.test.tsx"]
    p = subprocess.run(["perl", "-e", "alarm shift; exec @ARGV", "1500", "../../node_modules/.bin/vitest", "run", "--maxWorkers=4", "--reporter=json", f"--outputFile={out}", *files], cwd=LANE, env=ENV, capture_output=True, text=True)
    try:
        d = json.load(open(out))
    except Exception:
        return {"exit": p.returncode, "load_error": True}
    i67, rv88, load = [], [], []
    for f in d["testResults"]:
        name = f["name"].split("/src/")[1]
        bad = [a["fullName"] for a in f["assertionResults"] if a["status"] == "failed"]
        if f["status"] == "failed" and not bad: load.append(name)
        (rv88 if "zzRv88" in name else i67).extend(bad)
    return {"exit": p.returncode, "i67_failed": len(i67), "rv88_failed": len(rv88), "load_errors": load, "first_i67": i67[:2], "first_rv88": rv88[:2], "passed": d["numPassedTests"]}


def main():
    out_path, which = sys.argv[2], sys.argv[3]
    todo = [m for k, m in enumerate(I67) if k % 4 == 0] if which == "i67" else OWN
    if subprocess.run("pgrep -f memguard.sh", shell=True, capture_output=True).returncode != 0:
        sys.exit("MEMGUARD NOT RUNNING")
    res = {"control": run(f"control_{which}"), "mutants": []}
    assert res["control"].get("i67_failed") == 0 and res["control"].get("rv88_failed") == 0 and not res["control"].get("load_errors"), res["control"]
    for mid, path, old, new, desc in todo:
        full = f"{LANE}/{path}"
        text = open(f"{CAND}/{path}").read()
        if text.count(old) != 1:
            res["mutants"].append({"id": mid, "desc": desc, "error": f"match count {text.count(old)}"}); continue
        open(full, "w").write(text.replace(old, new))
        try:
            r = run(mid)
        finally:
            open(full, "w").write(text)
        r.update({"id": mid, "file": path, "desc": desc,
                  "i67_suite": "KILLED" if r.get("i67_failed") else ("LOAD_ERROR" if r.get("load_errors") or r.get("load_error") else "SURVIVED"),
                  "rv88_suite": "KILLED" if r.get("rv88_failed") else "SURVIVED"})
        res["mutants"].append(r)
        print(mid, r["i67_suite"], r["rv88_suite"], desc, flush=True)
        json.dump(res, open(out_path, "w"), indent=1)
    json.dump(res, open(out_path, "w"), indent=1)


if __name__ == "__main__":
    main()
