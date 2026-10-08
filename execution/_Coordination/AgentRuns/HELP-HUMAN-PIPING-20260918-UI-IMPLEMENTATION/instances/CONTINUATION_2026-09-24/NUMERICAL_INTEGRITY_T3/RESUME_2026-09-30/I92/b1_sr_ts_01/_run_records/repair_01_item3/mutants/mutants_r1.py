"""I92 repair 01 mutants: one exact string edit each to TS in a scratch copy, then the TS test file (TT) run with vitest's
JSON reporter, then TS restored and its sha256 checked. Killed only when TT loaded (all tests collected) and at least
one assertion failed. Run the whole programme as one job through WT/tools/t3_slot.sh.
Usage: mutants_r1.py <set: items12|item4> <copy> <expected test count>"""
import hashlib, json, os, subprocess, sys
S2 = "S2"
SET, COPY, TOTAL = sys.argv[1], sys.argv[2], int(sys.argv[3])
DESK = f"{S2}/{COPY}/projects/chirality-piping/apps/desktop"; TS = DESK + "/src/features/results/retainedPrecision.ts"
OUT = f"{S2}/logs/mutants_{SET}"; os.makedirs(OUT, exist_ok=True)
ORIGINAL = open(TS, "rb").read(); SHA = hashlib.sha256(ORIGINAL).hexdigest()
G = "  fail(!Object.hasOwn(model, 'reference_configurations') && model.pressure_contract == null && absentOrEmpty('combinations') && absentOrEmpty('components'), 'INVOCATION_MISMATCH');"
C = "&& Object.hasOwn(PRECONDITION_CODES, cause.precondition) && c.reason.code === PRECONDITION_CODES[cause.precondition]);"
M = {
 "items12": [
  ("N0", "the unmutated head", None, None),
  ("G-1", "(g): the reference_configurations conjunct dropped", "!Object.hasOwn(model, 'reference_configurations') && model.pressure_contract", "model.pressure_contract"),
  ("G-2", "(g): pressure_contract back to a falsy test", "model.pressure_contract == null && absentOrEmpty", "!model.pressure_contract && absentOrEmpty"),
  ("G-3", "(g): combinations back to `!combinations?.length`", "absentOrEmpty('combinations') && absentOrEmpty('components')", "!model.combinations?.length && absentOrEmpty('components')"),
  ("G-4", "(g): components back to `!components?.length`", "&& absentOrEmpty('components'), 'INVOCATION_MISMATCH');", "&& !model.components?.length, 'INVOCATION_MISMATCH');"),
  ("G-5", "(g): the check's code PREPARATION instead of INVOCATION", G, G.replace(", 'INVOCATION_MISMATCH');", ");")),
  ("G-6", "(g): absent-or-[] admits null", "const absentOrEmpty = (key: string) => !Object.hasOwn(model, key) ||", "const absentOrEmpty = (key: string) => model[key] == null ||"),
  ("C-1", "C2: precondition keying coarsened back to the set of four codes", C, "&& ['source_unavailable', 'resource_admission_not_available', 'upstream_no_wrap_not_established', 'caller_not_qualified'].includes(c.reason.code));"),
  ("C-2", "C2: caller keyed to source_unavailable", "{ caller: 'caller_not_qualified',", "{ caller: 'source_unavailable',"),
  ("C-3", "C2: resource_admission keyed to caller_not_qualified", "resource_admission: 'resource_admission_not_available',", "resource_admission: 'caller_not_qualified',"),
  ("C-4", "C2: upstream_no_wrap keyed to source_unavailable", "upstream_no_wrap: 'upstream_no_wrap_not_established',", "upstream_no_wrap: 'source_unavailable',"),
  ("C-5", "C2: capture keyed to caller_not_qualified", "capture: 'source_unavailable',", "capture: 'caller_not_qualified',"),
  ("C-6", "C2: source_family keyed to upstream_no_wrap_not_established", "source_family: 'source_unavailable' };", "source_family: 'upstream_no_wrap_not_established' };"),
 ],
 "item4": [
  ("N0", "the unmutated item-4 version", None, None),
  ("T-1", "transport: the header check skipped", "const base = projection(s), headerRefusal = headerCode(base, 'rust');", "const base = projection(s), headerRefusal = null as string | null;"),
  ("T-2", "transport: the header in Python's order (the raw path's)", "headerRefusal = headerCode(base, 'rust');", "headerRefusal = headerCode(base, 'python');"),
  ("T-3", "transport: the header refusal labelled G7", "throw new RetainedPrecisionError(gate, headerRefusal);", "throw new RetainedPrecisionError('G7', headerRefusal);"),
  ("T-4", "Rust's order: source_block_recovery no longer before contract_evidence", "const first = order === 'rust' ? recovery ?? evidence : evidence ?? recovery;", "const first = evidence ?? recovery;"),
  ("T-5", "Rust's order: a carrier_evidence branch added", "if (order === 'python' && Object.hasOwn(p, 'carrier_evidence'))", "if (Object.hasOwn(p, 'carrier_evidence'))"),
  ("T-6", "the raw path's order changed (Python's evidence-first order lost)", "const first = order === 'rust' ? recovery ?? evidence : evidence ?? recovery;", "const first = recovery ?? evidence;"),
 ],
 "final": [
  ("N0", "the unmutated final head (6fa6a64658)", None, None),
  ("T-1", "transport: the header check skipped", "const base = projection(s), headerRefusal = headerCode(base, 'rust');", "const base = projection(s), headerRefusal = null as string | null;"),
  ("T-2", "transport: the header in Python's order (the raw path's)", "headerRefusal = headerCode(base, 'rust');", "headerRefusal = headerCode(base, 'python');"),
  ("T-3", "transport: the header refusal labelled G7", "throw new RetainedPrecisionError(gate, headerRefusal);", "throw new RetainedPrecisionError('G7', headerRefusal);"),
  ("T-4", "Rust's order: source_block_recovery no longer before contract_evidence", "const first = order === 'rust' ? recovery ?? evidence : evidence ?? recovery;", "const first = evidence ?? recovery;"),
  ("T-5", "Rust's order: a carrier_evidence branch added", "if (order === 'python' && Object.hasOwn(p, 'carrier_evidence'))", "if (Object.hasOwn(p, 'carrier_evidence'))"),
  ("T-6", "the raw path's order changed (Python's evidence-first order lost)", "const first = order === 'rust' ? recovery ?? evidence : evidence ?? recovery;", "const first = recovery ?? evidence;"),
  ("G3-1", "TS's G3 combinations conjunct re-added", "  if (invocation) fail(same(ids, invocation.request?.model?.load_cases?.map((c: Obj) => c.id)));", "  if (invocation) fail(same(ids, invocation.request?.model?.load_cases?.map((c: Obj) => c.id)) && !(invocation.request?.model?.combinations?.length));"),
 ],
}[SET]
results = []
for mid, what, old, new in M:
    text = ORIGINAL.decode()
    if old is not None:
        assert text.count(old) == 1, (mid, text.count(old)); open(TS, "w").write(text.replace(old, new))
    try:
        out = f"{OUT}/{mid}.json"
        p = subprocess.run(["../../node_modules/.bin/vitest", "run", "src/features/results/retainedPrecision.test.ts", "--reporter=json", f"--outputFile={out}"],
                           cwd=DESK, env=dict(os.environ, TMPDIR=f"{S2}/tmp/mut"), capture_output=True, text=True)
        r = json.load(open(out))
        failed = [a["fullName"] for t in r["testResults"] for a in t["assertionResults"] if a["status"] == "failed"]
        load = [t.get("message", "") for t in r["testResults"] if not t["assertionResults"]]
    finally:
        open(TS, "wb").write(ORIGINAL); assert hashlib.sha256(open(TS, "rb").read()).hexdigest() == SHA
    total = r["numTotalTests"]
    state = ("passes" if not failed else "FAILS") if mid == "N0" else ("killed" if failed and not load and total == TOTAL else ("LOAD_FAILURE" if load or total != TOTAL else "survived"))
    rec = {"id": mid, "what": what, "old": old, "new": new, "rc": p.returncode, "total": total, "result": state, "failed_tests": failed}
    results.append(rec); print(json.dumps({k: rec[k] for k in ("id", "result", "total")}), "|", "; ".join(f.split(" > ")[-1][:80] for f in failed[:3]), flush=True)
open(f"{OUT}/mutants.jsonl", "w").write("".join(json.dumps(r) + "\n" for r in results))
print("restored", hashlib.sha256(open(TS, "rb").read()).hexdigest() == SHA)
