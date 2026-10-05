#!/usr/bin/env python3
"""I67 U7 slice F part 2 mutant programme (scratch). One mutation at a time in the mutant lane
(WT/scratch/i67_u6d/lanes/mut6, the final candidate), run against the retained TS tests plus the
related existing tests, then restored. Kinds: textual (old -> new, exactly one match) in product or
test code, and data mutations of the shared case file (parsed, edited, written back in the file's
own format: json.dumps indent 2 plus a newline). A mutant counts as killed by assertion only if a
test fails; a load (compile or collection) error alone is reported separately.
Usage: mutants_f.py <out.json> [ids...]"""
import json, os, subprocess, sys, time
T3 = "WT"
P = f"{T3}/scratch/i67_u6d/lanes/mut6/projects/chirality-piping"
LANE = f"{P}/apps/desktop"
OUT_DIR = f"{T3}/scratch/i67_u6d/mut/runs_f"
ENV = dict(os.environ, TMPDIR=f"{T3}/scratch/i67_u6d/tmp")
R = "src/features/results/"
S = "src/services/"
FILES = [R + "retainedPrecision.test.ts", R + "retainedPrecisionIntegration.test.tsx", S + "retainedPrecisionAnalysisRun.test.ts", R + "retainedPrecisionOutputRefusal.test.tsx",
         R + "numericalResultQuality.test.ts", R + "knownSemanticLimitations.test.ts", R + "resultSemantics.test.ts", S + "analysisRunCompatibility.test.ts",
         S + "previewService.test.ts", S + "ruleCheckService.test.ts", R + "HistoricalRunContext.test.tsx", R + "ResultsPanel.test.tsx", R + "previewPhysicsEvidence.test.ts"]
RP, RPS, PPE, ITEST = R + "retainedPrecision.ts", R + "retainedPrecisionStanding.ts", R + "previewPhysicsEvidence.ts", R + "retainedPrecisionIntegration.test.tsx"
CASES = "fixtures/results/retained_precision_carrier_cases.json"
D74 = "D-U7-4:ts_requires_live_native_capture"


def d74(doc):
    return next(e for e in doc["declared_differences"] if e["id"] == D74)


def data(fn):
    def apply(doc):
        fn(doc)
        return doc
    return apply


def drop_d74(doc): doc["declared_differences"] = [e for e in doc["declared_differences"] if e["id"] != D74]
def ts_side_eligible(doc):
    for f in d74(doc)["forms"]: f["expected"]["typescript"] = {"standing": "numerically_eligible"}
def ts_finding(doc):
    for f in d74(doc)["forms"]: f["expected"]["typescript"]["finding"] = "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"
def no_capture(doc): del d74(doc)["forms"][0]["capture"]
def capture_other(doc): d74(doc)["forms"][0]["capture"] = "ipc"
def no_model_edits(doc): del d74(doc)["forms"][1]["current_model_edits"]
def unknown_form_field(doc): d74(doc)["forms"][0]["note"] = "x"
def unknown_case_field(doc): doc["cases"][0]["note"] = "x"
def absent_path(doc): d74(doc)["forms"][1]["current_model_edits"][0]["path"] = ["nodes", 0, "position", "w"]
def case_id_edit(doc): d74(doc)["forms"][1]["current_model_edits"] = [{"path": ["load_cases", 0, "id"], "op": "set", "value": "renamed"}]
def scope_replace(old, new):
    def fn(doc):
        assert doc["scope"].count(old) == 1, old
        doc["scope"] = doc["scope"].replace(old, new)
    return fn


G7_CLAUSE_START = " A statement whose mechanics status is not MECHANICS_SOLVED"
def drop_g7_clause(doc):
    i = doc["scope"].index(G7_CLAUSE_START)
    doc["scope"] = doc["scope"][:i]


M = [
    # retainedPrecision.ts: the flag and C1:160's conjuncts
    ("F01", RP, "const SUMMARY_COVERAGE_COMPLETE = true;", "const SUMMARY_COVERAGE_COMPLETE = false;", "the TS flag reverted"),
    ("C01", RP, "SUMMARY_COVERAGE_COMPLETE && actual !== undefined && s.status", "SUMMARY_COVERAGE_COMPLETE && s.status", "eligibility: invocation conjunct dropped"),
    ("C02", RP, " && s.status?.mechanics === 'MECHANICS_SOLVED' && b.cases.every", " && b.cases.every", "eligibility: MECHANICS_SOLVED conjunct dropped"),
    ("C03", RP, " && b.cases.every((c: Obj) => ['selected', 'not_required'].includes(c.status));", ";", "eligibility: case-status conjunct dropped"),
    ("C04", RP, "b.cases.every((c: Obj) => ['selected', 'not_required'].includes(c.status))", "b.cases.every((c: Obj) => ['selected'].includes(c.status))", "eligibility: not_required cases not admitted"),
    ("C05", RP, "standing: eligible ? 'eligible' : 'needs_recompute', publication_sha256", "standing: 'needs_recompute', publication_sha256", "the reader's standing label decoupled from eligibility"),
    # D-U7-6: the standing-text sentence
    ("T01", RPS, " The reader checks these bytes and their invocation; it does not establish which producer made them.", "", "D-U7-6 sentence removed from the standing text"),
    # G7: the blocked-envelope check of TS's base validator
    ("G01", PPE, 'if (source.status?.mechanics !== "MECHANICS_SOLVED") {', "if (false) {", "G7: blocked-envelope check removed"),
    # the shared case file (data)
    ("D01", CASES, data(scope_replace("no carrier authenticates producer origin, and none claims", "none claims")), None, "D-U7-6 scope sentence removed"),
    ("D02", CASES, data(drop_d74), None, "D-U7-4 entry removed"),
    ("D03", CASES, data(ts_side_eligible), None, "D-U7-4 TS side set to numerically_eligible"),
    ("D04", CASES, data(ts_finding), None, "D-U7-4 TS finding set to NOT_NUMERICALLY_ELIGIBLE"),
    ("D05", CASES, data(no_capture), None, "D-U7-4 form 1: capture removed"),
    ("D06", CASES, data(capture_other), None, "D-U7-4 form 1: capture set to another value"),
    ("D07", CASES, data(no_model_edits), None, "D-U7-4 form 2: current_model_edits removed"),
    ("D08", CASES, data(unknown_form_field), None, "an unknown form field"),
    ("D09", CASES, data(unknown_case_field), None, "an unknown case field"),
    ("D10", CASES, data(absent_path), None, "current_model_edits naming an absent key"),
    ("D11", CASES, data(case_id_edit), None, "current_model_edits changing a load-case id"),
    ("D12", CASES, data(drop_g7_clause), None, "G7 blocked-envelope scope clause removed"),
    ("D13", CASES, data(scope_replace("TS SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID", "TS SOURCE_PREVIEW_PHYSICS_BLOCKED_ENVELOPE")), None, "G7 clause: TS's code misstated"),
    # the TS consumer of the v4 fields and the held knob (test harness)
    ("A01", ITEST, 'captured = fromFixture && !Object.hasOwn(c, "capture");', "captured = fromFixture;", "consumer: capture ignored"),
    ("A02", ITEST, 'if (Object.hasOwn(c, "current_model_edits")) {\n    expect(fromFixture', 'if (false) {\n    expect(fromFixture', "consumer: current_model_edits ignored"),
    ("H01", ITEST, '      if (u7.held) return Object.freeze({ ...validation, numerical_eligible: false, standing: "needs_recompute" as const });\n', "", "the held knob ignored"),
]


def run(tag):
    os.makedirs(OUT_DIR, exist_ok=True)
    out = f"{OUT_DIR}/{tag}.json"
    if os.path.exists(out): os.remove(out)
    p = subprocess.run(["../../node_modules/.bin/vitest", "run", "--maxWorkers=4", "--reporter=json", f"--outputFile={out}", *FILES], cwd=LANE, env=ENV, capture_output=True, text=True, timeout=1800)
    try: d = json.load(open(out))
    except Exception: return {"exit": p.returncode, "load_error": True, "failed": None, "passed": None, "first": p.stderr[-400:]}
    failed = [(f["name"].split("/src/")[1], a["fullName"]) for f in d["testResults"] for a in f["assertionResults"] if a["status"] == "failed"]
    load = [f["name"].split("/src/")[1] + ": " + (f.get("message") or "")[:200] for f in d["testResults"] if f["status"] == "failed" and not any(a["status"] == "failed" for a in f["assertionResults"])]
    return {"exit": p.returncode, "failed": len(failed), "passed": d["numPassedTests"], "load_errors": load, "first": failed[:3]}


def main():
    out_path, ids = sys.argv[1], set(sys.argv[2:])
    results = {"lane": "WT/scratch/i67_u6d/lanes/mut6", "files": FILES, "mutants": []}
    t0 = time.time()
    ctrl = run("control")
    results["control"] = ctrl
    assert ctrl.get("failed") == 0 and not ctrl.get("load_errors"), ctrl
    for mid, path, old, new, desc in M:
        if ids and mid not in ids: continue
        full = f"{LANE}/{path}" if path.startswith("src/") else f"{P}/{path}"
        text = open(full).read()
        if callable(old):
            doc = json.loads(text)
            assert json.dumps(doc, indent=2) + "\n" == text
            mutated = json.dumps(old(doc), indent=2) + "\n"
        else:
            n = text.count(old)
            if n != 1:
                results["mutants"].append({"id": mid, "file": path, "desc": desc, "error": f"match count {n}"}); print(mid, "MATCH", n, flush=True); continue
            mutated = text.replace(old, new)
        assert mutated != text
        open(full, "w").write(mutated)
        try: r = run(mid)
        finally: open(full, "w").write(text)
        killed = bool(r.get("failed")) or bool(r.get("load_errors")) or r.get("load_error", False)
        results["mutants"].append({"id": mid, "file": path, "desc": desc, "killed": killed, "by_assertion": bool(r.get("failed")), **r})
        print(mid, "KILLED" if killed else "SURVIVED", r.get("failed"), (r.get("first") or [""])[0], flush=True)
    results["seconds"] = round(time.time() - t0)
    json.dump(results, open(out_path, "w"), indent=1)
    s = [m["id"] for m in results["mutants"] if not m.get("killed")]
    print("total", len(results["mutants"]), "survivors", s, "not by assertion", [m["id"] for m in results["mutants"] if m.get("killed") and not m.get("by_assertion")])


if __name__ == "__main__":
    main()
