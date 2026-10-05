"""RV81 confirmation mutation runner: the 16 review mutants (M05/M08 re-anchored to the new code)
plus 15 repair-targeted mutants (R*). One single edit at a time on WT/rv81's copy, restored and
sha256-verified after each. Usage: WT=... python3 mutants2.py [ids...]"""
import hashlib, json, os, re, subprocess, sys
WT = os.environ['WT']; L = WT + '/scratch/rv81_confirm04'
D = WT + '/rv81/projects/chirality-piping/apps/desktop'; SRC = D + '/src/features/results/retainedPrecision.ts'
PRISTINE_SHA = '136f39108d30760695e3c632d699462dac66650ae5d364b162931482ad60c8dc'
M = [
 ('M01_flag_true', "const SUMMARY_COVERAGE_COMPLETE = false;", "const SUMMARY_COVERAGE_COMPLETE = true;", 'fail-closed hold'),
 ('M02_n4_vref_dropped', "const solveFailure = (vr: Obj) => vr.verification_shared_build_ref === null && vr.work.verification_lme === 0 && vr.verification === null;", "const solveFailure = (vr: Obj) => vr.work.verification_lme === 0 && vr.verification === null;", 'D5b/D21 v-build indicator'),
 ('M03_reason_locator_removed', "if (reason?.quantity) fail(", "if (false) fail(", 'D5d'),
 ('M04_candidate_shape_lme', "fail(record.verification === null && record.verification_shared_build_ref === null && record.work.verification_lme === 0);", "fail(record.verification === null && record.verification_shared_build_ref === null);", 'D5a'),
 ('M05_refs_resolve_dropped', "fail(unique(a.diagnostic_refs) && a.diagnostic_refs.every((ref: string) => ds.some(d => d.id === ref)));", "fail(unique(a.diagnostic_refs));", 'D6a resolve (names-the-case dropped by D6a)'),
 ('M06_selected_quality_set', "['sensitive', 'unresolved', 'failed'].includes(q.solve_quality)", "['checks_passed', 'sensitive', 'unresolved', 'failed'].includes(q.solve_quality)", 'D6b'),
 ('M07_budget_scope_swapped', "work(terminal.reason.scope === 'case' ? caseOver : (!caseOver && (invOver || uint(run.invocation_before) >= uint(b.work.invocation_limit))));", "work(terminal.reason.scope === 'case' ? (caseOver || invOver) : (invOver || uint(run.invocation_before) >= uint(b.work.invocation_limit)));", 'N17'),
 ('M08_r3_some', "STATUS_FAULTS[o.fault].every(x => statuses(owner).has(x))", "STATUS_FAULTS[o.fault].some(x => statuses(owner).has(x))", "R3' both"),
 ('M09_ceil_no_increment', "if ((m - (q << s)) !== 0n) q++;", "if ((m - (q << s)) !== 0n && false) q++;", 'upward rounding'),
 ('M10_phi_le', "if (!(nearest * back < hat)) return nearest;", "if (!(nearest * back <= hat)) return nearest;", 'phi_512'),
 ('M11_g8_member_sequence', "if (a.operational.old_coverage === 'complete') fail(same(a.operational.old.map((m: Obj) => m.member), sequence(pipes.length)));", "if (a.operational.old_coverage === 'complete') fail(a.operational.old.length === pipes.length);", 'G8 backstop of D1'),
 ('M12_empty_roster_g3', "fail(cov.length >= 1 && cov.length === sourced.body_membership.length", "fail(cov.length === sourced.body_membership.length", 'empty roster'),
 ('M13_nodata_theta', "for (const e of cov) if (!e.has_data) fail(record.theta.find((v: Obj) => v.body === e.body)?.value === ZERO);", "", 'D13 theta'),
 ('M14_g5_order_products_first', "gate = 'G5'; nativeRuns(b); ordinaryAttempts(b, s); productAttempts(b, rows);", "gate = 'G5'; nativeRuns(b); productAttempts(b, rows); ordinaryAttempts(b, s);", 'D3/D17 order'),
 ('M15_floor_or_dropped', "positive[2] ||= force; positive[3] ||= moment;", "positive[2] ||= false; positive[3] ||= moment;", 'floor coverage'),
 ('M16_ceiling_threshold', "fail(last.precision * 4 > 512); expected", "fail(last.precision * 4 >= 512); expected", 'D13 Ceiling'),
 ('R01_b1_g3_members_unique_only', "fail(fresh.length <= pm.length && pm.length <= old.length && [old, pm, fresh].every(list => same(list.map((m: Obj) => m.member), sequence(list.length))));", "fail(unique(old.map((m: Obj) => m.member)) && fresh.length <= pm.length && pm.length <= old.length && same(pm.map((m: Obj) => m.member), old.slice(0, pm.length).map((m: Obj) => m.member)) && same(fresh.map((m: Obj) => m.member), pm.slice(0, fresh.length).map((m: Obj) => m.member)));", 'B1/D1 (restores the reviewed code)'),
 ('R02_b2_d4c_removed', "for (const c of b.cases) if (c.status === 'unavailable' && c.reason?.cause?.kind === 'prepared_product_failure') fail(", "for (const c of b.cases) if (false) fail(", 'B2/D4c'),
 ('R03_s1_captured_assoc_removed', "if (a.operational.old_coverage === 'captured_prefix') fail(a.source_ref === null && a.run_ref === null && a.result.kind === 'unavailable');", "", 'S1/D1 association at G5'),
 ('R04_s2_versions_removed', "fail(source.schema_version === '0.2.0' && producer.component_name === 'open_pipe_stress_product_physics' && producer.component_version === '0.2.0');", "", 'S2/D2 producer versions'),
 ('R05_d18_positivity_removed', "actual[k] === section[k] && decodeBinary64(section[k]) > 0)", "actual[k] === section[k])", 'D18'),
 ('R06_d2_table_bytes_removed', "fail(await sha256Text(tableBytes) === TABLE_HASH && await sha256Text(inheritedTableBytes) === table.inherited_semantic_contract_sha256);", "", 'D2 bundled bytes (unreachable from a receipt)'),
 ('R07_d3_work_immediate', "try { nativeClass(b, ok => { deferred.push(!!ok); }); }", "try { nativeClass(b, ok => { need(ok, 'G5', 'WORK_MISMATCH'); }); }", 'D3 class-1 convention'),
 ('R08_d6d_code_work', "const w = at(b.legacy_source_work ?? [], a.legacy_source.work_ref, 'G5', 'ATTEMPT_MISMATCH');", "const w = at(b.legacy_source_work ?? [], a.legacy_source.work_ref, 'G5', 'WORK_MISMATCH');", 'D6d code'),
 ('R09_d5c_removed', "fail(a.verification?.phase === 'failed');", "fail(true);", 'D5c'),
 ('R10_r4_removed', "return [r1, r2, r3, r4];", "return [r1, r2, r3, true];", 'D8 R4'),
 ('R11_kernel_scope_removed', "fail(!locate([kernelRuns, b.builds, b.groups.map((g: Obj) => g.preparation)]).some(([, o]) => o.tag === 'work_accounting'));", "", 'D8 kernel scope'),
 ('R12_d1_unsourced_count_removed', "else if (a.operational.old_coverage === 'complete') fail(b.sources.every((s: Obj) => s.id_maps.members.length === old.length));", "", 'D1 unsourced complete count'),
 ('R13_d2_body_absent_skipped', "fail(isObj(b));", "if (!isObj(b)) return;", 'settled reading 3'),
 ('R14_r2_operational_dropped', "&& !operational.some(e => isObj(e) && e.kind === 'accounting' && !Object.hasOwn(e, 'event'))", "", "D8 R2' operational"),
 ('R15_d6c_exponent', "if (a.w2.kind === 'published') fail(a.w2.force_scale_exponent !== 0);", "", 'D6c exponent'),
 ('R16_d21_summary_indicator_dropped', "vr.work.verification_lme === 0 && vr.verification === null;", "vr.work.verification_lme === 0;", 'D21 third indicator'),
 ('R17_d21_lme_indicator_dropped', "const solveFailure = (vr: Obj) => vr.verification_shared_build_ref === null && vr.work.verification_lme === 0 && vr.verification === null;", "const solveFailure = (vr: Obj) => vr.verification_shared_build_ref === null && vr.verification === null;", 'D21 lme indicator'),
 ('R18_d22_g3_on_dangling', "if (a.source_ref !== null) { if (sourced) fail(same(", "if (a.source_ref !== null) { fail(sourced && same(", 'D22 (G3 fails on a dangling source_ref)'),
 ('R19_d29_removed', "fail(s.body_membership.length >= 1 && s.index === i", "fail(s.index === i", 'D29'),
 ('R20_d27_reverted', "const exhausted = uint(run.invocation_before) >= uint(b.work.invocation_limit);", "const exhausted = current >= uint(b.work.invocation_limit);", 'D27'),
 ('R21_d20_removed', "if (c.status === 'selected') fail(c.product_attempt_ref !== null);", "", 'D20'),
 ('R22_d19_unavailable_half_removed', "if (own.result.kind === 'unavailable') fail(", "if (false) fail(", 'D19 unavailable half'),
 ('R23_d19_ready_half_removed', "if (own.result.kind === 'ready') fail(", "if (false) fail(", 'D19 Ready half'),
 ('R24_d20_back_in_ordinary', "if (c.status === 'selected') fail(a.initial.kind !== 'not_attempted' &&", "if (c.status === 'selected') fail(c.product_attempt_ref !== null && a.initial.kind !== 'not_attempted' &&", 'D20 code (selected without attempt reported ATTEMPT in the ordinary pass)'),

 ('R25_d33_removed', "if (reason?.space === 'attempt' && reason.tag === 'verification_estimate') fail(['force', 'moment'].includes(reason.kind));", "", 'D33'),
 ('R26_d33_admits_rotation', "fail(['force', 'moment'].includes(reason.kind));", "fail(['force', 'moment', 'rotation'].includes(reason.kind));", 'D33 kind set (rotation admitted)'),
 ('R27_d33_force_only', "fail(['force', 'moment'].includes(reason.kind));", "fail(['force'].includes(reason.kind));", 'D33 kind set (moment refused)'),
 ('R28_d31_reverted', "['0.1.0', '0.2.0', '0.3.0'].includes(model.schema_version)", "['0.2.0', '0.3.0'].includes(model.schema_version)", 'D31'),
 ('R29_d31_admits_040', "['0.1.0', '0.2.0', '0.3.0'].includes(model.schema_version)", "['0.1.0', '0.2.0', '0.3.0', '0.4.0'].includes(model.schema_version)", 'D31 0.4.0 exclusion'),
 ('R30_uint_admits_neg_zero', "function uint(v: number): bigint { need(Number.isSafeInteger(v) && v >= 0 && !Object.is(v, -0), 'G2', 'ENCODING_MISMATCH'); return BigInt(v); }", "function uint(v: number): bigint { need(Number.isSafeInteger(v) && v >= 0, 'G2', 'ENCODING_MISMATCH'); return BigInt(v); }", 'D32 -0 in uint'),
 ('R31_encoding_admits_neg_zero', "if (tag === 'uint' || tag === 'i32') need(Number.isSafeInteger(v) && !Object.is(v, -0) && v >= spec.minimum", "if (tag === 'uint' || tag === 'i32') need(Number.isSafeInteger(v) && v >= spec.minimum", 'D32 -0 in the G2 encoding'),

]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
assert sha(SRC) == PRISTINE_SHA, 'copy not pristine'
pristine = open(SRC, encoding='utf8').read(); results = []; only = sys.argv[1:]
for mid, old, new, target in M:
    if only and mid not in only: continue
    assert pristine.count(old) == 1, (mid, pristine.count(old))
    open(SRC, 'w', encoding='utf8').write(pristine.replace(old, new))
    log = f'{L}/mutant_{mid}.log'
    with open(log, 'w') as fh:
        rc = subprocess.run(['npm', 'test', '--', 'src/features/results/retainedPrecision.test.ts', '--maxWorkers=2'], cwd=D, stdout=fh, stderr=subprocess.STDOUT).returncode
    text = open(log, encoding='utf8', errors='replace').read()
    tests = re.findall(r'Tests\s+(.*)', text); failed = re.findall(r'FAIL\s+src/features/results/retainedPrecision\.test\.ts > (.*)', text)
    open(SRC, 'w', encoding='utf8').write(pristine); assert sha(SRC) == PRISTINE_SHA
    results.append({'id': mid, 'target': target, 'exit': rc, 'killed': rc != 0, 'tests': tests[-1].strip() if tests else None, 'failing': sorted(set(x.strip() for x in failed))[:8], 'log_sha256': sha(log)})
    print(mid, 'KILLED' if rc else 'SURVIVED', results[-1]['tests'], flush=True)
json.dump(results, open(f'{L}/mutant_results{"_" + "_".join(only) if only else ""}.json', 'w'), indent=1)
print('restored', sha(SRC) == PRISTINE_SHA)
