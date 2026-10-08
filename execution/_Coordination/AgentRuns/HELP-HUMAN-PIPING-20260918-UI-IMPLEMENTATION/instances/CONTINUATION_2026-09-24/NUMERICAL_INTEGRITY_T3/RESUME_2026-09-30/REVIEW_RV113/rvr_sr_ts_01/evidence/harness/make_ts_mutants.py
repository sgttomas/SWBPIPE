"""RV113 (RV-R): a mutant schema in the reviewer's own copy (WT/rv113/ts-mut) of SR-TS's head.

Each mutant is a source edit of TS guarded by `mx('<id>')`, true only when the environment's RV113_MUT equals the id;
with RV113_MUT unset the copy behaves as the head (checked by a control run). Usage:
python make_ts_mutants.py <retainedPrecision.ts> <manifest out>
"""
import json
import sys

path, manifest_out = sys.argv[1], sys.argv[2]
ts = open(path).read()
manifest = []


def sub(old, new, mid, desc, family, extra=()):
    global ts
    assert ts.count(old) == 1, (mid, ts.count(old), old[:90])
    ts = ts.replace(old, new)
    manifest.append({"id": mid, "family": family, "description": desc})
    for e in extra:
        manifest.append(e)


anchor = "const MAX_BITS = 0x7fefffffffffffffn;\n"
assert ts.count(anchor) == 1
ts = ts.replace(anchor, anchor + "const mx = (id: string): boolean => (globalThis as any).process?.env?.RV113_MUT === id;\n")

# --- R-D38 (4b) ------------------------------------------------------------------------------------------------
sub("""    fail(stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || (stage.native === 'failed' && d38CaptureBeforeRun(a, c, s)));""",
    """    fail(mx('T03') ? (stage.native === 'not_entered') === (a.run_ref === null)
      : mx('T02') ? (stage.native === 'not_entered' ? a.run_ref === null : stage.native === 'failed' ? d38CaptureBeforeRun(a, c, s) : a.run_ref !== null)
      : stage.native === 'not_entered' ? a.run_ref === null : a.run_ref !== null || ((mx('T01') || stage.native === 'failed') && (mx('T04') || d38CaptureBeforeRun(a, c, s))));""",
    "T01", "(4b) branch without `native === 'failed'` (a completed native stage with no Run reaches (4b))", "D38",
    [{"id": "T02", "family": "D38", "description": "every failed native stage must be (4b) ((4a), a failed stage with its Run, refused)"},
     {"id": "T03", "family": "D38", "description": "the relaxed check restored: native entered <=> a Run"},
     {"id": "T04", "family": "D38", "description": "(4b)'s predicate replaced by true"}])
sub("""function d38CaptureBeforeRun(a: Obj, c: Obj, s: Obj | null): boolean {
  const stage = a.stages;
  return a.result.kind""",
    """function d38CaptureBeforeRun(a: Obj, c: Obj, s: Obj | null): boolean {
  const stage = a.stages;
  if (mx('T05')) return a.source_ref === c.source_ref;
  return a.result.kind""",
    "T05", "(4b)'s predicate reduced to the source equality alone (the minimal form)", "D38")
sub("""    && s !== null && s.preparation?.attempt_ref === a.id && a.source_ref === c.source_ref;""",
    """    && (mx('T07') || (s !== null && s.preparation?.attempt_ref === a.id)) && (mx('T06') || a.source_ref === c.source_ref);""",
    "T06", "(4b)'s source equality dropped", "D38",
    [{"id": "T07", "family": "D38", "description": "(4b)'s 'a source binding this attempt' dropped"}])

# --- G8 per case ------------------------------------------------------------------------------------------------
sub("""    const c = cases[ci], ordinary = b.ordinary_attempts[ci]; fail(ordinary.requested_mode === invocation.solver_mode);""",
    """    const c = cases[ci], ordinary = b.ordinary_attempts[ci]; fail(ordinary.requested_mode === invocation.solver_mode, mx('T16') ? 'INVOCATION_MISMATCH' : undefined);
    const rv113Only = (mx('T18') && ci !== 0) || (mx('T19') && b.cases[ci]?.status !== 'selected');""",
    "T16", "the requested-mode check's code back to INVOCATION_MISMATCH", "G8",
    [{"id": "T18", "family": "G8", "description": "P1-P4 for case 0 only (the per-case loop narrowed)"},
     {"id": "T19", "family": "G8", "description": "P1-P4 for selected cases only"}])
sub("""    const own = (kind: string) => source.results.filter((r: Obj) => r.basis_ref?.ref_id === c.id && r.kind === kind);""",
    """    const own = (kind: string) => source.results.filter((r: Obj) => r.basis_ref?.ref_id === (mx('T21') ? cases[0].id : c.id) && r.kind === kind);""",
    "T21", "P1-P4 read case 0's rows for every case (rows mis-indexed)", "G8")
sub("""    fail(modeRows.length === 1 && modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1));""",
    """    fail(rv113Only || (modeRows.length === 1 && (mx('T15') ? (invocation.solver_mode === 'dense_scrutiny' ? modeRows[0].value === 2 : [1, 3].includes(modeRows[0].value)) : modeRows[0].value === (invocation.solver_mode === 'dense_scrutiny' ? 2 : 1))), mx('T17') ? 'INVOCATION_MISMATCH' : undefined);""",
    "T15", "mode code 3 restored in sparse_interactive", "G8",
    [{"id": "T17", "family": "G8", "description": "P1's code back to INVOCATION_MISMATCH"}])
sub("""    fail(parityRows.length <= 1);
    fail(!parityRows.length || invocation.solver_mode === 'dense_scrutiny');
    fail(!parityRows.length || ordinary.w2?.kind !== 'published');""",
    """    fail(rv113Only || parityRows.length <= (mx('T13') ? 2 : 1));
    fail(rv113Only || mx('T14') || !parityRows.length || invocation.solver_mode === 'dense_scrutiny');
    const p4w2 = mx('T10') ? b.ordinary_attempts[0].w2 : ordinary.w2;
    fail(rv113Only || mx('T12') || !parityRows.length || (mx('T11') ? p4w2?.kind === 'not_triggered' : p4w2?.kind !== 'published'));""",
    "T10", "P4 reads case 0's ordinary attempt", "G8",
    [{"id": "T11", "family": "G8", "description": "P4 stricter: a parity row only beside W2 not_triggered"},
     {"id": "T12", "family": "G8", "description": "P4 dropped"},
     {"id": "T13", "family": "G8", "description": "P2 widened to at most two"},
     {"id": "T14", "family": "G8", "description": "P3 dropped"}])

# --- G5 not_required --------------------------------------------------------------------------------------------
sub("""    if (c.status === 'not_required') fail(c.product_attempt_ref === null && a.initial.kind !== 'not_attempted' && q.solve_quality === 'checks_passed');""",
    """    if (c.status === 'not_required') fail((mx('T30') || c.product_attempt_ref === null) && (mx('T32') || a.initial.kind !== 'not_attempted') && (mx('T31') || q.solve_quality === 'checks_passed')
      && (!mx('T33') || a.initial.kind === 'report') && (!mx('T34') || a.initial.outcome === 'checks_passed') && (!mx('T35') || a.w2.kind === 'not_triggered'));""",
    "T30", "not_required: `product_attempt_ref === null` dropped (RV113 S-1's conjunct)", "G5",
    [{"id": "T31", "family": "G5", "description": "not_required: verdict checks_passed dropped"},
     {"id": "T32", "family": "G5", "description": "not_required: `initial !== not_attempted` dropped"},
     {"id": "T33", "family": "G5", "description": "not_required: Rust's dropped conjunct `initial.kind === report` added"},
     {"id": "T34", "family": "G5", "description": "not_required: Rust's dropped conjunct `initial.outcome === checks_passed` added"},
     {"id": "T35", "family": "G5", "description": "not_required: Rust's dropped conjunct `w2 === not_triggered` added"}])
sub("""    if (a.initial.kind === 'report') { diagnostic(a.initial.report_diagnostic_ref, cid, true); fail(a.diagnostic_refs.includes(a.initial.report_diagnostic_ref) && a.initial.outcome === q.solve_quality); }""",
    """    if (a.initial.kind === 'report') { diagnostic(a.initial.report_diagnostic_ref, cid, true); fail(a.diagnostic_refs.includes(a.initial.report_diagnostic_ref) && (mx('T36') || a.initial.outcome === q.solve_quality)); }""",
    "T36", "the report-outcome equality dropped", "G5")

# --- RV108 N4 -------------------------------------------------------------------------------------------------------
sub("""  if (Array.isArray(p.results)) for (const row of p.results) if (isObj(row) && Object.hasOwn(row, 'recovery_method')) delete row.recovery_method;""",
    """  if (Array.isArray(p.results)) for (const row of p.results) if ((mx('T40') || isObj(row)) && Object.hasOwn(row, 'recovery_method')) delete row.recovery_method;""",
    "T40", "RV108 N4's rule removed (a non-object row reaches Object.hasOwn)", "N4")

open(path, "w").write(ts)
json.dump(manifest, open(manifest_out, "w"), indent=1)
print(len(manifest), "mutants:", ", ".join(m["id"] for m in manifest))
