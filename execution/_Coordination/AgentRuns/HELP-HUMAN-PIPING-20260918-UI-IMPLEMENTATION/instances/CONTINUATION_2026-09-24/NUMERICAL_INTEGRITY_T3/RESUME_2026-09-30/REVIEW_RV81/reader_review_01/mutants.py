"""RV81 mutation runner: one single-edit mutant at a time on WT/rv81's copy, restored after each."""
import hashlib, json, os, re, shutil, subprocess, sys
WT = os.environ['WT']; L = WT + '/scratch/rv81_reader_review'
D = WT + '/rv81/projects/chirality-piping/apps/desktop'; SRC = D + '/src/features/results/retainedPrecision.ts'
PRISTINE_SHA = '7c802881a280385926930a90e624fb60a65069100268bdffcd3a77eb84d18845'
M = [
 ('M01_flag_true', "const SUMMARY_COVERAGE_COMPLETE = false;", "const SUMMARY_COVERAGE_COMPLETE = true;", 'fail-closed hold'),
 ('M02_n4_vref_dropped', "fail(escalates(stopOf(vr.outcome)) && vr.verification_shared_build_ref === null && vr.work.verification_lme === 0);", "fail(escalates(stopOf(vr.outcome)) && vr.work.verification_lme === 0);", 'known diff 1 discriminator (continuation)'),
 ('M03_reason_locator_removed', "if (reason?.quantity) fail(", "if (false) fail(", 'known diff 2'),
 ('M04_candidate_shape_lme', "fail(record.verification === null && record.verification_shared_build_ref === null && record.work.verification_lme === 0);", "fail(record.verification === null && record.verification_shared_build_ref === null);", 'known diff 3'),
 ('M05_listed_refs_unchecked', "fail(unique(a.diagnostic_refs)); for (const ref of a.diagnostic_refs) diagnostic(ref, cid);", "fail(unique(a.diagnostic_refs));", 'known diff 4 (every listed ref names the case)'),
 ('M06_selected_quality_set', "['sensitive', 'unresolved', 'failed'].includes(q.solve_quality)", "['checks_passed', 'sensitive', 'unresolved', 'failed'].includes(q.solve_quality)", 'known diff 4 (selected quality)'),
 ('M07_budget_scope_swapped', "work(terminal.reason.scope === 'case' ? caseOver : (!caseOver && (invOver || uint(run.invocation_before) >= uint(b.work.invocation_limit))));", "work(terminal.reason.scope === 'case' ? (caseOver || invOver) : (invOver || uint(run.invocation_before) >= uint(b.work.invocation_limit)));", 'N17 precedence'),
 ('M08_r3_some', "STATUS_FAULTS[o.fault].every(x => seen.has(x)));", "STATUS_FAULTS[o.fault].some(x => seen.has(x)));", 'R3 both-fault containment'),
 ('M09_ceil_no_increment', "if ((m - (q << s)) !== 0n) q++;", "if ((m - (q << s)) !== 0n && false) q++;", 'upward rounding'),
 ('M10_phi_le', "if (!(nearest * back < hat)) return nearest;", "if (!(nearest * back <= hat)) return nearest;", 'phi_512 next_up'),
 ('M11_g8_member_sequence', "if (a.operational.old_coverage === 'complete') fail(same(a.operational.old.map((m: Obj) => m.member), sequence(pipes.length)));", "if (a.operational.old_coverage === 'complete') fail(a.operational.old.length === pipes.length);", 'member index sequence (only check without a source)'),
 ('M12_empty_roster_g3', "fail(cov.length >= 1 && cov.length === b.sources[a.source_ref].body_membership.length", "fail(cov.length === b.sources[a.source_ref].body_membership.length", 'parity rule: empty non-null roster'),
 ('M13_nodata_theta', "for (const e of cov) if (!e.has_data) fail(record.theta.find((v: Obj) => v.body === e.body)?.value === ZERO);", "", 'I57 s4 item 4 theta=+0 without data'),
 ('M14_g5_order_products_first', "gate = 'G5'; nativeRuns(b); ordinaryAttempts(b, s); productAttempts(b, rows);", "gate = 'G5'; nativeRuns(b); productAttempts(b, rows); ordinaryAttempts(b, s);", 'within-G5 order (C3:304)'),
 ('M15_floor_or_dropped', "positive[2] ||= force; positive[3] ||= moment;", "positive[2] ||= false; positive[3] ||= moment;", 'positive floor forces force coverage'),
 ('M16_ceiling_threshold', "fail(last.precision * 4 > 512); expected", "fail(last.precision * 4 >= 512); expected", 'N8 verification-solve Ceiling threshold'),
]
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
assert sha(SRC) == PRISTINE_SHA, 'copy not pristine'
pristine = open(SRC, encoding='utf8').read(); results = []
only = sys.argv[1:]
for mid, old, new, target in M:
    if only and mid not in only: continue
    assert pristine.count(old) == 1, (mid, pristine.count(old))
    open(SRC, 'w', encoding='utf8').write(pristine.replace(old, new))
    log = f'{L}/mutant_{mid}.log'
    with open(log, 'w') as fh:
        rc = subprocess.run(['npm', 'test', '--', 'src/features/results/retainedPrecision.test.ts', '--maxWorkers=2'], cwd=D, stdout=fh, stderr=subprocess.STDOUT).returncode
    text = open(log, encoding='utf8', errors='replace').read()
    tests = re.findall(r'Tests\s+(.*)', text); failed = re.findall(r'FAIL\s+src/features/results/retainedPrecision\.test\.ts > (.*)', text)
    shutil.copyfile(SRC + '.tmp', SRC) if False else open(SRC, 'w', encoding='utf8').write(pristine)
    assert sha(SRC) == PRISTINE_SHA
    results.append({'id': mid, 'target': target, 'exit': rc, 'killed': rc != 0, 'tests': tests[-1].strip() if tests else None, 'failing': sorted(set(x.strip() for x in failed))[:12], 'log_sha256': sha(log)})
    print(mid, 'KILLED' if rc else 'SURVIVED', results[-1]['tests'])
json.dump(results, open(f'{L}/mutant_results{"_" + "_".join(only) if only else ""}.json', 'w'), indent=1)
print('restored', sha(SRC) == PRISTINE_SHA)
