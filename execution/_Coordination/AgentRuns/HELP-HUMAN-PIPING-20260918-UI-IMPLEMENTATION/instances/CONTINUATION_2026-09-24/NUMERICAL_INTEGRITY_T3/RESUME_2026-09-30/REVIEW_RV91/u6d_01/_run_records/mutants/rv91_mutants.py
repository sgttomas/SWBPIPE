#!/usr/bin/env python3
"""RV91 mutant run (scratch). A sample of I67's 103 mutants (their exact
old/new text, read from I67's committed mutants.py) plus RV91's own mutants.
Each is applied in the mutant lane WT/rv91/mut only, run against I67's 3 new
test files + 8 related existing files + RV91's probe file, then restored.
Kills are attributed per file set. Usage: rv91_mutants.py <out.json> [ids...]"""
import importlib.util, json, os, subprocess, sys, time
T3 = "WT"
S = f"{T3}/scratch/rv91_u6d"
LANE = f"{T3}/rv91/mut/projects/chirality-piping/apps/desktop"
ENV = dict(os.environ, TMPDIR=f"{S}/tmp", RV91_REVIEW_OUT=f"{S}/mutants/review_obs.json", RV91_EXTRA_CASES=f"{S}/parity/extra_cases.json")
I67 = f"{T3}/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I67/u6d_typescript_01/_run_records/mutants.py"
spec = importlib.util.spec_from_file_location("i67m", I67); i67 = importlib.util.module_from_spec(spec); spec.loader.exec_module(i67)
R, Sv = "src/features/results/", "src/services/"
I67_FILES = i67.NEW + i67.RELATED
RV91_FILE = R + "zzRV91Review.test.tsx"
NRQ, RPS, KSL, ARC, RCS, HRC = R + "numericalResultQuality.ts", R + "retainedPrecisionStanding.ts", R + "knownSemanticLimitations.ts", Sv + "analysisRunCompatibility.ts", Sv + "ruleCheckService.ts", R + "HistoricalRunContext.tsx"
SAMPLE = ["N06", "N09", "N13", "N16", "N23", "S03", "S04", "S05", "S09", "S14", "S22", "S25", "K03", "K13", "K15", "K17", "L01", "R01", "A02", "A04", "A05", "P01", "P02", "C01", "C02", "RP1", "H03", "H05"]
OWN = [
  ("RV01", NRQ, 'return Object.hasOwn(source, "retained_precision")\n    ||', 'return source.retained_precision != null\n    ||', "guard: a null receipt member admitted on another identity (Rust and Python refuse it)"),
  ("RV02", NRQ, '  if (retainedPrecisionDowngrade(source)) return "unsupported";\n', '  if (source.schema_version !== "0.1.0" && retainedPrecisionDowngrade(source)) return "unsupported";\n', "guard: legacy 0.1.0 exempt from the downgrade guard"),
  ("RV03", RPS, 'captured = structuredClone(invocation);', 'captured = invocation;', "registration: holds the caller's live invocation object (not the captured copy)"),
  ("RV04", RPS, 'return structuredClone(registered(source)!.invocation);', 'return registered(source)!.invocation;', "registered invocation handed out by reference"),
  ("RV05", ARC, 'if (!same(run.retained_precision, source.retained_precision)) throw', 'if ((run.retained_precision as { receipt_sha256?: unknown } | undefined)?.receipt_sha256 !== (source.retained_precision as { receipt_sha256?: unknown } | undefined)?.receipt_sha256) throw', "AnalysisRun: copy equality on receipt_sha256 only"),
  ("RV06", KSL, 'retainedAbsoluteNotice(classified.bound_bits!, row.unit!)', 'retainedAbsoluteNotice(classified.scale_bits!, row.unit!)', "label: prints the body scale S* instead of the bound b"),
  ("RV07", KSL, 'let digits = Math.round(Number(mantissa) * 100) + 1,', 'let digits = Math.round(Number(mantissa) * 100),', "bound text: no upward step when nearest is below b"),
  ("RV08", KSL, 'if (source.producer?.semantic_contract_id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID) {\n    const classes', 'if (sourceContract(source) === "retained_preview_physics") {\n    const classes', "binding: successor selected by route, not producer id (a header-broken successor binds as ordinary)"),
  ("RV09", RPS, 'const record = (outcome: Outcome) => { if (sourcePrint) registrations.set(source, { source: sourcePrint,', 'const record = (outcome: Outcome) => { const late = fingerprint(source); if (late && sourcePrint) registrations.set(source, { source: late,', "registration: fingerprint taken after the reader's await"),
  ("RV10", RCS, 'retainedRowClassLabel(row, source) ?? N_RP_UNVALIDATED : N_SB;', 'N_RP_UNVALIDATED : N_SB;', "precheck: class label never shown"),
  ("RV11", HRC, 'catch (error) { findings.push((error as Error).message); }', 'catch { findings.push("RETAINED_PRECISION_REOPEN_REFUSED"); }', "reopen: the reader's code not preserved"),
]
def run(tag, files):
    out = f"{S}/mutants/runs/{tag}.json"; os.makedirs(os.path.dirname(out), exist_ok=True)
    if os.path.exists(out): os.remove(out)
    p = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=4", "--reporter=json", f"--outputFile={out}", *files], cwd=LANE, env=ENV, capture_output=True, text=True, timeout=1800)
    try: d = json.load(open(out))
    except Exception: return {"exit": p.returncode, "load_error": True, "stderr": p.stderr[-400:]}
    failed = [(f["name"].split("/src/")[1], a["fullName"]) for f in d["testResults"] for a in f["assertionResults"] if a["status"] == "failed"]
    load = [f["name"].split("/src/")[1] for f in d["testResults"] if f["status"] == "failed" and not any(a["status"] == "failed" for a in f["assertionResults"])]
    by_i67 = [x for x in failed if "src/" + x[0] in I67_FILES] + [x for x in load if "src/" + x in I67_FILES]
    by_rv91 = [x for x in failed if "src/" + x[0] == RV91_FILE] + [x for x in load if "src/" + x == RV91_FILE]
    return {"exit": p.returncode, "passed": d["numPassedTests"], "failed": len(failed), "load_errors": load, "killed_by_i67_suite": bool(by_i67), "killed_by_rv91": bool(by_rv91), "first_i67": by_i67[:2], "first_rv91": by_rv91[:2]}
def main():
    out_path, ids = sys.argv[1], set(sys.argv[2:])
    i67m = {m[0]: m for m in i67.M}
    plan = [(i67m[k][0], i67m[k][1], i67m[k][2], i67m[k][3], i67m[k][4], "I67") for k in SAMPLE] + [(*m, "RV91") for m in OWN]
    files = I67_FILES + [RV91_FILE]
    res = {"lane": "WT/rv91/mut", "files": files, "i67_total": len(i67.M), "mutants": []}
    t0 = time.time()
    ctrl = run("control", files); res["control"] = ctrl
    assert ctrl.get("failed") == 0 and not ctrl.get("load_errors"), ctrl
    print("control", ctrl["passed"], flush=True)
    for mid, path, old, new, desc, origin in plan:
        if ids and mid not in ids: continue
        full = f"{LANE}/{path}"; text = open(full).read(); n = text.count(old)
        if n != 1: res["mutants"].append({"id": mid, "origin": origin, "desc": desc, "error": f"match count {n}"}); print(mid, "MATCH", n, flush=True); continue
        open(full, "w").write(text.replace(old, new))
        try: r = run(mid, files)
        finally: open(full, "w").write(text)
        killed = bool(r.get("failed")) or bool(r.get("load_errors")) or r.get("load_error", False)
        res["mutants"].append({"id": mid, "origin": origin, "file": path, "desc": desc, "killed": killed, **r})
        print(mid, origin, "KILLED" if killed else "SURVIVED", "i67" if r.get("killed_by_i67_suite") else "-", "rv91" if r.get("killed_by_rv91") else "-", flush=True)
    res["seconds"] = round(time.time() - t0)
    json.dump(res, open(out_path, "w"), indent=1)
if __name__ == "__main__": main()
