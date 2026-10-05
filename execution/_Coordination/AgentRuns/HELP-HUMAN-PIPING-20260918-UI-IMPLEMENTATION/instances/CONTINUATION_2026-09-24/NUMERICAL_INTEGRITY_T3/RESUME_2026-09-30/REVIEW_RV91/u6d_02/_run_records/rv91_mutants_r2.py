#!/usr/bin/env python3
"""RV91 round 2 mutants (scratch). Applied in WT/rv91/mut2 (an APFS clone of the
repair commit 968adb44fe), run against the COMMITTED tests only (I67's 3 U6d test
files + 8 related existing files; no RV91 probe), then restored.
Usage: rv91_mutants_r2.py <out.json>"""
import importlib.util, json, os, subprocess, sys, time
T3 = "WT"
S = f"{T3}/scratch/rv91_u6d"
LANE = f"{T3}/rv91/mut2/projects/chirality-piping/apps/desktop"
ENV = dict(os.environ, TMPDIR=f"{S}/tmp")
spec = importlib.util.spec_from_file_location("r1", f"{S}/mutants/rv91_mutants.py"); r1 = importlib.util.module_from_spec(spec); spec.loader.exec_module(r1)
FILES = r1.I67_FILES
R, Sv = "src/features/results/", "src/services/"
RPS, ARC, NRQ = R + "retainedPrecisionStanding.ts", Sv + "analysisRunCompatibility.ts", R + "numericalResultQuality.ts"
own = {m[0]: m for m in r1.OWN}
PLAN = [own[k] for k in ("RV03", "RV04", "RV08", "RV09")] + [
  ("RVb1", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', '  if (result.schema_version === "0.1.0" && retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', "v0.2: guard applied to schema 0.1.0 only (a 0.2.0 legacy shape escapes)"),
  ("RVb2", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', '  if (Object.hasOwn(result, "retained_precision") || (result.results as { recovery_method?: unknown }[])[0]?.recovery_method === "contribution_preserving_multiprecision_v1") throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);', "v0.2: token checked on the first row only"),
  ("RVb3", ARC, '  if (retainedPrecisionDowngrade(result)) throw new Error(ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN);\n', '', "v0.2: guard removed"),
  ("RVb4", RPS, 'unsupported, values shown for inspection only. ${tail}`;', 'unsupported, values shown for inspection only. Selected cases: ${receiptCases(source).filter(c => c?.status === "selected").length} of ${receiptCases(source).length}. ${tail}`;', "text: count leaks into the refused text"),
  ("RVb5", RPS, 'needs recompute. ${tail}`;', 'needs recompute. Selected cases: ${receiptCases(source).filter(c => c?.status === "selected").length} of ${receiptCases(source).length}. ${tail}`;', "text: count leaks into the unvalidated text"),
]
def run(tag):
    out = f"{S}/r2/mutant_runs/{tag}.json"; os.makedirs(os.path.dirname(out), exist_ok=True)
    if os.path.exists(out): os.remove(out)
    p = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=4", "--reporter=json", f"--outputFile={out}", *FILES], cwd=LANE, env=ENV, capture_output=True, text=True, timeout=1800)
    try: d = json.load(open(out))
    except Exception: return {"exit": p.returncode, "load_error": True}
    failed = [(f["name"].split("/src/")[1], a["fullName"]) for f in d["testResults"] for a in f["assertionResults"] if a["status"] == "failed"]
    load = [f["name"].split("/src/")[1] for f in d["testResults"] if f["status"] == "failed" and not any(a["status"] == "failed" for a in f["assertionResults"])]
    return {"exit": p.returncode, "passed": d["numPassedTests"], "failed": len(failed), "load_errors": load, "first": failed[:2]}
def main():
    res = {"lane": "WT/rv91/mut2 (968adb44fe)", "files": FILES, "mutants": []}
    t0 = time.time(); ctrl = run("control"); res["control"] = ctrl
    assert ctrl.get("failed") == 0 and not ctrl.get("load_errors"), ctrl
    print("control", ctrl["passed"], flush=True)
    for mid, path, old, new, desc in PLAN:
        full = f"{LANE}/{path}"; text = open(full).read(); n = text.count(old)
        if n != 1: res["mutants"].append({"id": mid, "desc": desc, "error": f"match count {n}"}); print(mid, "MATCH", n, flush=True); continue
        open(full, "w").write(text.replace(old, new))
        try: r = run(mid)
        finally: open(full, "w").write(text)
        killed = bool(r.get("failed")) or bool(r.get("load_errors")) or r.get("load_error", False)
        res["mutants"].append({"id": mid, "file": path, "desc": desc, "killed": killed, "by_assertion": bool(r.get("failed")), **r})
        print(mid, "KILLED" if killed else "SURVIVED", (r.get("first") or [""])[0], flush=True)
    res["seconds"] = round(time.time() - t0)
    json.dump(res, open(sys.argv[1], "w"), indent=1)
if __name__ == "__main__": main()
