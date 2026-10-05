#!/usr/bin/env python3
"""I67 U7 repair (u7_repair_01) TS mutant programme (scratch), derived from slice F part 2's mutants_f.py; lane mut9. One mutation at a time in the mutant lane
(WT/scratch/i67_u6d/lanes/mut6, the final candidate), run against the retained TS tests plus the
related existing tests, then restored. Kinds: textual (old -> new, exactly one match) in product or
test code, and data mutations of the shared case file (parsed, edited, written back in the file's
own format: json.dumps indent 2 plus a newline). A mutant counts as killed by assertion only if a
test fails; a load (compile or collection) error alone is reported separately.
Usage: mutants_f.py <out.json> [ids...]"""
import json, os, subprocess, sys, time
T3 = "WT"
P = f"{T3}/scratch/i67_u6d/lanes/mut9/projects/chirality-piping"
LANE = f"{P}/apps/desktop"
OUT_DIR = f"{T3}/scratch/i67_u6d/mut/runs_r9"
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


CORPUS = "fixtures/results/retained_precision_cases.json"
PROBE = "g7_not_required_quality_enum_invalid"
def sumforms(doc): return [f for f in d74(doc)["forms"] if f["subject"] == "summary"]
def drop_sumforms(doc): d74(doc)["forms"] = [f for f in d74(doc)["forms"] if f["subject"] != "summary"]
def side(lang, value):
    def fn(doc):
        for f in sumforms(doc): f["expected"][lang] = {"summary": value}
    return fn
def twin_differs(doc): del sumforms(doc)[0]["capture"]
N3 = " An invalid enum value in a not_required case's quality is refused at G7"
def drop_n3(doc):
    i = doc["scope"].index(N3); j = doc["scope"].index("I67 u7_repair_01).", i) + len("I67 u7_repair_01).")
    doc["scope"] = doc["scope"][:i] + doc["scope"][j:]
def drop_probe(doc): doc["mutations"] = [m for m in doc["mutations"] if m["id"] != PROBE]
def probe_ts(code):
    def fn(doc): next(m for m in doc["mutations"] if m["id"] == PROBE)["expected_by_reader"]["typescript"]["code"] = code
    return fn
M = [
    ("S01", CASES, data(drop_sumforms), None, "S-1: D-U7-4's summary forms removed"),
    ("S02", CASES, data(side("typescript", "by_validated_class_current")), None, "S-1: TS side flipped to Current"),
    ("S03", CASES, data(side("python", "by_validated_class")), None, "S-1: Python side flipped (TS unaffected, Python's own test)"),
    ("S04", CASES, data(side("rust", "by_validated_class")), None, "S-1: Rust side flipped (TS unaffected, Rust's own test)"),
    ("S05", CASES, data(twin_differs), None, "S-1: a summary form's inputs differ from its standing twin"),
    ("N01", CASES, data(drop_n3), None, "N-3: scope clause removed"),
    ("N02", CASES, data(scope_replace("TS SOURCE_PRODUCER_CONTRACT_UNSUPPORTED", "TS SOURCE_NUMERICAL_CASE_INVALID")), None, "N-3: clause misstates TS's code"),
    ("N03", CORPUS, data(drop_probe), None, "N-3: the shared probe removed"),
    ("N04", CORPUS, data(probe_ts("SOURCE_NUMERICAL_CASE_INVALID")), None, "N-3: the probe's TS expectation flipped"),
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
    results = {"lane": "WT/scratch/i67_u6d/lanes/mut9", "files": FILES, "mutants": []}
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
