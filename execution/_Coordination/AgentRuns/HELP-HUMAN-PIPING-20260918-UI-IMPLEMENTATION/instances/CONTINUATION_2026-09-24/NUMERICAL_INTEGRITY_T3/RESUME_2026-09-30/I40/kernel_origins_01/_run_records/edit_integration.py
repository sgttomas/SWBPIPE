from pathlib import Path
k=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained')
p=k/'adaptive.rs';s=p.read_text()
s=s.replace('use super::work::{WorkFault, WorkStatus, WorkStream, WorkTotal};','use super::work::{WorkFault, WorkStatus, WorkStream, WorkTotal};\nuse super::origins::{BatchRecording, OriginSlot, RunPhase, RunTrace, SlotSnapshot};')
s=s.replace('    fn into_legacy(self) -> CaseOutcome {','    pub(crate) fn into_legacy(self) -> CaseOutcome {',1)
s=s.replace('impl CoreRun {\n','''impl CoreRun {
    fn idle(outcome: ExecutionOutcome, before: WorkTotal) -> Self {
        let zero = WorkTotal::zero().join_status(before.status());
        Self { outcome, work: RunWork { case: zero, invocation_before: before,
            invocation_increment: zero, invocation_after: before } }
    }
''',1)
needle='''    let invocation_before = meter.checked_charged();
    let mut budget = CaseBudget {'''
assert s.count(needle)==1
s=s.replace(needle,'''    run_core_with_origins(prep, group, cache, case_limit, meter, None)
}

pub(crate) fn run_core_with_origins(
    prep: Arc<CasePrep>,
    group: Arc<GroupPrep>,
    cache: &mut GroupCache,
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
    trace: Option<&mut RunTrace<'_>>,
) -> CoreRun {
    let invocation_before = meter.checked_charged();
    let mut budget = CaseBudget {''',1)
s=s.replace('run_schedule_inner(prep, group, cache, &mut budget, meter)','run_schedule_inner(prep, group, cache, &mut budget, meter, trace)',1)
# Only schedule, solve_precision and verify_precision signatures receive trace.
for fn in ['run_schedule_inner','solve_precision','verify_precision']:
 start=s.index('fn '+fn+'(');end=s.index(') -> ',start)
 s=s[:end]+('    mut trace: Option<&mut RunTrace<\'_>>,\n' if fn=='run_schedule_inner' else '    mut trace: Option<&mut RunTrace<\'_>>,\n    physical_record: usize,\n')+s[end:]
s=s.replace('solve_precision(p, &prep, &group, cache, budget, meter);','solve_precision(p, &prep, &group, cache, budget, meter,\n                    trace.as_deref_mut(), attempts.len());',1)
s=s.replace('solve_precision(verification_p, &prep, &group, cache, budget, meter);','solve_precision(verification_p, &prep, &group, cache, budget, meter,\n                trace.as_deref_mut(), attempts.len());',1)
s=s.replace('            &mut attempts[v_index],\n        )','            &mut attempts[v_index],\n            trace.as_deref_mut(),\n            v_index,\n        )',1)
s=s.replace('''                obtain::<$L, $R>($slot, p, $q, &prep.source, group, guard);
            record.latch(shared_total);''','''                obtain::<$L, $R>($slot, p, $q, &prep.source, group, guard);
            if let Some(trace) = trace.as_deref_mut() {
                trace.requested(OriginSlot::solve(p), physical_record, built,
                    shared.as_ref().err(), shared_total, &shared_stages);
            }
            record.latch(shared_total);''',1)
s=s.replace('''                obtain_verify::<$L, $R, $W>($vslot, &shared, &prep.source, group, guard);
            record.latch(vs_total);''','''                obtain_verify::<$L, $R, $W>($vslot, &shared, &prep.source, group, guard);
            if let Some(trace) = trace.as_deref_mut() {
                trace.requested(OriginSlot::verify(state.precision()), physical_record, built,
                    vs.as_ref().err(), vs_total, &vs_stages);
            }
            record.latch(vs_total);''',1)
start=s.index('pub fn solve_cases(');end=s.index('/// One case (a group of one;',start)
s=s[:start]+'''pub fn solve_cases(
    sources: &[PrimitiveSource],
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
) -> Vec<CaseOutcome> {
    // Project before appending each result: legacy refuses retain no extra
    // terminal attempts and allocate no origin inventory.
    solve_cases_projected(sources, case_limit, meter, None,
        Vec::with_capacity(sources.len()), |run, _| run.into_legacy())
}

pub(crate) fn solve_cases_projected<T>(
    sources: &[PrimitiveSource],
    case_limit: CaseLimit,
    meter: &mut InvocationMeter,
    mut recording: Option<BatchRecording<'_>>,
    mut out: Vec<T>,
    mut project: impl FnMut(CoreRun, Option<usize>) -> T,
) -> Vec<T> {
    let mut groups: Vec<(
        Vec<u8>,
        Result<Arc<GroupPrep>, (Refusal, Vec<BodyGeometry>)>,
        GroupCache,
    )> = Vec::new();
    for (position, source) in sources.iter().enumerate() {
        let before = meter.checked_charged();
        let early = if let Some(fault) = before.status().fault() {
            Some(UnresolvedReason::WorkAccounting { fault, prior: None })
        } else if meter.exhausted() {
            Some(UnresolvedReason::Budget(BudgetScope::Invocation))
        } else { None };
        if let Some(reason) = early {
            let run = CoreRun::idle(ExecutionOutcome::Unresolved {
                reason, attempts: Vec::new(), geometry: Vec::new(),
            }, before);
            let id = recording.as_mut().map(|r| r.finish(position, None,
                RunPhase::InvocationEntry, &run, None));
            out.push(project(run, id));
            continue;
        }
        let identity = source.stiffness_encoding();
        let index = match groups.iter().position(|g| g.0 == identity) {
            Some(k) => k,
            None => {
                let prepared = prepare_group(source).map(Arc::new);
                if let Some(recording) = recording.as_mut() {
                    recording.group(position, prepared.as_ref().err().map(|e| &e.0));
                }
                groups.push((identity, prepared, GroupCache::default()));
                groups.len() - 1
            }
        };
        let (_, group, cache) = &mut groups[index];
        let group = match group {
            Ok(g) => g.clone(),
            Err((refusal, geometry)) => {
                let run = CoreRun::idle(ExecutionOutcome::Refused {
                    refusal: refusal.clone(), attempts: Vec::new(), geometry: geometry.clone(),
                }, before);
                let id = recording.as_mut().map(|r| r.finish(position, Some(index),
                    RunPhase::GroupPreparation, &run, None));
                out.push(project(run, id));
                continue;
            }
        };
        let prep = match CasePrep::new(source.clone()) {
            Ok(p) => Arc::new(p),
            Err(e) => {
                let run = CoreRun::idle(ExecutionOutcome::Refused {
                    refusal: Refusal::LedgerUnavailable(e), attempts: Vec::new(),
                    geometry: group.geometry.clone(),
                }, before);
                let id = recording.as_mut().map(|r| r.finish(position, Some(index),
                    RunPhase::SourcePreparation, &run, None));
                out.push(project(run, id));
                continue;
            }
        };
        let mut trace = recording.as_mut().map(|r| r.trace(index));
        let run = run_core_with_origins(prep, group, cache, case_limit, meter, trace.as_mut());
        let capture = trace.map(RunTrace::finish);
        let id = recording.as_mut().map(|r| r.finish(position, Some(index),
            RunPhase::Schedule, &run, capture));
        out.push(project(run, id));
    }
    out
}

'''+s[end:]
s=s.replace('impl GroupCache {\n','''impl GroupCache {
    pub(crate) fn matches_origins(&self, slots: &SlotSnapshot) -> bool {
        let occupied = [self.s128.is_some(), self.s256.is_some(), self.s512.is_some(),
            self.s1024.is_some(), self.v256.is_some(), self.v512.is_some(), self.v1024.is_some()];
        occupied.iter().zip(slots).all(|(present, origin)| *present == origin.is_some())
    }
''',1)
p.write_text(s)
p=k/'combine.rs';s=p.read_text()
start=s.index('        if operands.is_empty()');end=s.index('        let mut cache = GroupCache::merged',start)
old=s[start:end]
s=s[:start]+'''        if let Err(reason) = validate_operands(operands) {
            return withheld(reason);
        }
        let prep = match prepare_operands(operands) {
            Ok(prep) => prep,
            Err(reason) => return withheld(reason),
        };
        let first = operands[0].1;
'''+s[end:]
# Same native validation and preparation functions shared with origin path.
start=old.index('        if operands.is_empty()');end=old.index('        let preps:')
validation=old[start:end].replace('return withheld(', 'return Err(')
prepare=old[end:].replace('return withheld(', 'return Err(')
prepare=prepare.replace('        let prep = match CasePrep::combination(&preps) {','        match CasePrep::combination(&preps) {').replace('            Ok(p) => Arc::new(p),','            Ok(p) => Ok(Arc::new(p)),')
assert prepare.endswith('        };\n')
prepare=prepare[:-len('        };\n')]+'        }\n'
s=s.replace('#[cfg(test)]','''pub(crate) fn validate_operands(operands: &[(f64, &RetainedSolve)])
    -> Result<(), CombinationReason>
{
'''+validation+'''    Ok(())
}

pub(crate) fn prepare_operands(operands: &[(f64, &RetainedSolve)])
    -> Result<Arc<CasePrep>, CombinationReason>
{
'''+prepare+'''}

#[cfg(test)]''',1)
p.write_text(s)
p=k/'mod.rs';s=p.read_text();s+='\npub(crate) mod origins;\n';p.write_text(s)
p=k.parent.parent/'structural.rs';s=p.read_text();i=s.index('    pub use super::retained::work::');s=s[:i]+'''    pub use super::retained::origins::{
        BuildOrigin, BuildPhase, BuildState, CacheImport, CallKind, CallOrigin, CallResult,
        CombinationStage, GroupOrigin, GroupPreparation, NativeOwner, OriginCapacity,
        OriginError, OriginSlot, RecordBuildLinks, RecordedCase, RecordedInvocation,
        RecordedKernelCombination, RequestedOperand, RunOrigins, RunPhase, SlotSnapshot,
        SourceOrigin,
    };
'''+s[i:];p.write_text(s)
