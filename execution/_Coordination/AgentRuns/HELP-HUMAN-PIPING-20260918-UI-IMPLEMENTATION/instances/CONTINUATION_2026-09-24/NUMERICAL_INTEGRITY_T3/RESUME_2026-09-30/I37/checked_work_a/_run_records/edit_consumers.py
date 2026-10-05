from pathlib import Path
import re
p=Path('projects/chirality-piping/core/solver/performance_harness/src/k6/w1/staged.rs');s=p.read_text().replace('InvocationMeter, PrimitiveSource, SourceError, StageWork, UnresolvedReason,','InvocationMeter, PrimitiveSource, SourceError, StageWork, UnresolvedReason, WorkFault, WorkTotal,')
s=s.replace('pub charged: u64,','pub charged: WorkTotal,').replace('charged: meter.charged(),','charged: meter.checked_charged(),')
a=s.index('    stage_fields(w)');b=s.index('\n}',a);s=s[:a]+'    w.checked_total().exact().unwrap_or(u64::MAX)'+s[b:]
a=s.index('pub fn work_closes(');b=s.index('\n/// The shared work',a)
s=s[:a]+'''pub fn validate_attempts(attempts: &[AttemptRecord]) -> Result<(), WorkFault> {
    for a in attempts {
        a.checked_case_charge().exact()?;
        a.checked_invocation_increment().exact()?;
        a.checked_verification_work().exact()?;
        a.checked_stop_rule_work().exact()?;
        a.stages.checked_total().exact()?;
        a.shared_stages.checked_total().exact()?;
    }
    Ok(())
}
pub fn work_closes(attempts: &[AttemptRecord], charged: WorkTotal) -> bool {
    let checked = || -> Result<bool, WorkFault> {
        validate_attempts(attempts)?;
        let total = attempts.iter().try_fold(0u64, |sum, a| sum.checked_add(a.checked_invocation_increment().exact()?).ok_or(WorkFault::Overflow))?;
        Ok(total == charged.exact()?)
    };
    checked().unwrap_or(false)
}
'''+s[b:]
s=s.replace('    unstaged(a) == (0, 0)','    validate_attempts(std::slice::from_ref(a)).is_ok() && unstaged(a) == (0, 0)')
s=s.replace('    attempts.iter().all(|a| {\n        stage_sum','    validate_attempts(attempts).is_ok() && attempts.iter().all(|a| {\n        stage_sum')
a=s.index('fn add_stages(');b=s.index('/// Each attempt',a)
s=s[:a]+'''fn add_stages(a: &mut StageWork, b: &StageWork) -> Result<(), WorkFault> { a.merge(b) }

'''+s[b:]
s=s.replace('pub fn work_by_precision(attempts: &[AttemptRecord]) -> Vec<PrecisionWork> {','pub fn work_by_precision(attempts: &[AttemptRecord]) -> Result<Vec<PrecisionWork>, WorkFault> {\n    validate_attempts(attempts)?;')
s=s.replace('add_stages(&mut entry.own, &a.stages);','add_stages(&mut entry.own, &a.stages)?;').replace('add_stages(&mut entry.shared, &a.shared_stages);','add_stages(&mut entry.shared, &a.shared_stages)?;')
s=s.replace('entry.own_total.saturating_add(own_total(a))','entry.own_total.checked_add(own_total(a)).ok_or(WorkFault::Overflow)?').replace('entry.shared_total.saturating_add(shared_total(a))','entry.shared_total.checked_add(shared_total(a)).ok_or(WorkFault::Overflow)?')
s=s.replace('    out.sort_by_key(|w| w.precision);\n    out','    out.sort_by_key(|w| w.precision);\n    Ok(out)')
s=s.replace('pub fn segments(attempts: &[AttemptRecord]) -> Vec<Segment> {','pub fn segments(attempts: &[AttemptRecord]) -> Result<Vec<Segment>, WorkFault> {\n    validate_attempts(attempts)?;')
a=s.index('pub fn segments(');b=s.index('/// Parity `w1_prefix_segments`',a);chunk=s[a:b]
chunk=re.sub(r'\.saturating_add\(([^()]+)\)',r'.checked_add(\1).ok_or(WorkFault::Overflow)?',chunk)
chunk=re.sub(r'\.saturating_sub\(([^()]+)\)',r'.checked_sub(\1).ok_or(WorkFault::Inconsistent)?',chunk)
chunk=chunk.replace('    out\n}','    Ok(out)\n}');s=s[:a]+chunk+s[b:]
s=s.replace('    let full_segments = segments(full);\n    let own = segments(attempts_of(prefix));','    let Ok(full_segments) = segments(full) else { return false; };\n    let Ok(own) = segments(attempts_of(prefix)) else { return false; };')
s=s.replace('pub fn prefix_limits(attempts: &[AttemptRecord]) -> Vec<(String, u64)> {\n    let segs = segments(attempts);\n    segs.iter()','pub fn prefix_limits(attempts: &[AttemptRecord]) -> Result<Vec<(String, u64)>, WorkFault> {\n    let segs = segments(attempts)?;\n    Ok(segs.iter()').replace('        .map(|s| (s.label.clone(), s.end))\n        .collect()','        .map(|s| (s.label.clone(), s.end))\n        .collect())')
p.write_text(s)
p=Path('projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/w1.rs');s=p.read_text().replace('AttemptRecord, CaseOutcome, GateTest, UnresolvedReason,','AttemptRecord, CaseOutcome, GateTest, UnresolvedReason, WorkFault,').replace('attempts_of, charged_by,','validate_attempts, attempts_of, charged_by,')
s=s.replace('pub fn budget_reached(solve: &W1Solve) -> bool {\n    solve.exhausted','pub fn budget_reached(solve: &W1Solve) -> Result<bool, WorkFault> {\n    solve.charged.exact()?;\n    Ok(solve.exhausted').replace('        )\n}\n\n/// The repeat', '        ))\n}\n\n/// The repeat')
s=s.replace('pub fn repeat_digest(solve: &W1Solve) -> (u64, u64) {','pub fn repeat_digest(solve: &W1Solve) -> Result<(u64, u64), WorkFault> {\n    solve.charged.exact()?;\n    validate_attempts(attempts_of(&solve.outcome))?;')
s=s.replace('solve.charged.to_le_bytes()','solve.charged.exact()?.to_le_bytes()').replace('    (len, h.finish())','    Ok((len, h.finish()))')
# Change writer signatures and insert whole-input validation before any output.
for fn in ['outcome_line','attempt_lines','parity_lines','prefix_line']:
 a=s.index('pub fn '+fn+'(');brace=s.index(' {',a);s=s[:brace]+' -> Result<(), WorkFault>'+s[brace:]
 # find closing function via brace balance
 brace=s.index(' {',a)+1;depth=1;i=brace+1
 while depth:
  if s[i]=='{':depth+=1
  if s[i]=='}':depth-=1
  i+=1
 s=s[:i-1]+'    Ok(())\n'+s[i-1:]
 validation='\n    validate_attempts(attempts)?;' if fn=='attempt_lines' else '\n    solve.charged.exact()?;\n    validate_attempts(attempts_of(&solve.outcome))?;'
 if fn=='prefix_line':validation+='\n    validate_attempts(full)?;'
 s=s[:brace+1]+validation+s[brace+1:]
s=s.replace('.n("meter_charged", solve.charged)','.n("meter_charged", solve.charged.exact()?)').replace('budget_reached(solve))','budget_reached(solve)?)').replace('for w in work_by_precision(attempts) {','for w in work_by_precision(attempts)? {')
p.write_text(s)
p=Path('projects/chirality-piping/core/solver/performance_harness/src/bin/k6_observe/main.rs');s=p.read_text()
s=s.replace('w1::outcome_line(repeat, &solve);','w1::outcome_line(repeat, &solve).unwrap_or_else(work_failure);').replace('w1::attempt_lines(repeat, attempts_of(&solve.outcome));','w1::attempt_lines(repeat, attempts_of(&solve.outcome)).unwrap_or_else(work_failure);').replace('w1::parity_lines(repeat, w1_counts, &solve);','w1::parity_lines(repeat, w1_counts, &solve).unwrap_or_else(work_failure);').replace('w1::repeat_digest(&solve)','w1::repeat_digest(&solve).unwrap_or_else(work_failure)').replace('prefix_limits(full).iter()','prefix_limits(full).unwrap_or_else(work_failure).iter()').replace('w1::prefix_line(j + 1, label, *limit, full, &solve);','w1::prefix_line(j + 1, label, *limit, full, &solve).unwrap_or_else(work_failure);')
s+='''
fn work_failure<T>(fault: open_pipe_stress_frame_kernel::structural::retained_api::WorkFault) -> T {
    eprintln!("k6_observe: retained work accounting {fault}");
    std::process::exit(4)
}
''';p.write_text(s)
p=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/src/records.rs');s=p.read_text().replace('AttemptRecord, CaseOutcome, InvocationMeter, StageWork,','AttemptRecord, CaseOutcome, InvocationMeter, StageWork, WorkFault,')
s=s.replace('pub fn case_record(case: &Case, o: &CaseOutcome, t: &Tally, meter: &InvocationMeter) -> Value {','''pub fn case_record(case: &Case, o: &CaseOutcome, t: &Tally, meter: &InvocationMeter) -> Result<Value, WorkFault> {
    meter.checked_charged().exact()?;
    let attempts = match o {
        CaseOutcome::Selected(s) => s.evidence().attempts.as_slice(),
        CaseOutcome::Unresolved { attempts, .. } => attempts.as_slice(),
        CaseOutcome::Refused { .. } => &[],
    };
    let mut charged = 0u64;
    for a in attempts {
        a.checked_case_charge().exact()?;
        charged = charged.checked_add(a.checked_invocation_increment().exact()?).ok_or(WorkFault::Overflow)?;
        a.checked_verification_work().exact()?;
        a.checked_stop_rule_work().exact()?;
        a.stages.checked_total().exact()?;
        a.shared_stages.checked_total().exact()?;
    }''').replace('    Value::Object(r)','    Ok(Value::Object(r))')
p.write_text(s)
p=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/src/lane.rs');s=p.read_text().replace('pub record: Value,','pub record: Option<Value>,').replace('let record = records::source_refused_record(case, &format!("{e:?}"), &tally);','let record = Some(records::source_refused_record(case, &format!("{e:?}"), &tally));')
s=s.replace('let record = records::case_record(case, &outcome, &tally, &meter);','''let record = match records::case_record(case, &outcome, &tally, &meter) {
        Ok(record) => Some(record),
        Err(fault) => { failures.push(format!("{}: retained work accounting {fault}", case.id)); None }
    };''');p.write_text(s)
p=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/examples/vk_records.rs');s=p.read_text().replace('            if show', '            if show')
s=s.replace('            records.push(run.record.clone());','''            let Some(record) = run.record.clone() else {
                eprintln!("{}: no exact work record: {:?}", run.id, run.failures);
                std::process::exit(1);
            };
            records.push(record);''')
s=s.replace('            if show.as_', '            if show.as_')
# Ensure guard precedes any --show record printing, not only saving.
s=s.replace('                println!("{}", serde_json::to_string_pretty(&run.record).unwrap());','                let record = run.record.as_ref().unwrap_or_else(|| { eprintln!("{}: no exact work record", run.id); std::process::exit(1) });\n                println!("{}", serde_json::to_string_pretty(record).unwrap());')
p.write_text(s)
p=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/examples/vk_scale.rs');s=p.read_text().replace('let record = run.record.clone();','let record = run.record.clone().unwrap_or_else(|| { eprintln!("{}: no exact work record: {:?}", run.id, run.failures); std::process::exit(1) });');p.write_text(s)
p=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/tests/lane.rs');s=p.read_text().replace('records.push(run.record);','assert!(run.failures.is_empty(), "{:?}", run.failures);\n        records.push(run.record.expect("exact record after successful run"));');p.write_text(s)
