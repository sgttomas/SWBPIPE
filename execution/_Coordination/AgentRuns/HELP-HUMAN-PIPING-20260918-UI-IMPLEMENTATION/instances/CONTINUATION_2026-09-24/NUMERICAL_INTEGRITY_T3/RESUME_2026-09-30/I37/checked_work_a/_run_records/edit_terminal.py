from pathlib import Path
import re
p=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/adaptive.rs');s=p.read_text()
s=s.replace('self.charged.exact().is_err_or(|v| v >= self.limit)','self.charged.exact().map_or(true, |v| v >= self.limit)')
s=s.replace('    fn total(&self) -> u64 {\n        lme(&self.ctx)','    fn total(&self) -> WorkTotal {\n        lme(&self.ctx)')
s=s.replace('#[derive(Debug, Clone, PartialEq)]\npub struct AttemptRecord {','#[derive(Clone, PartialEq)]\npub struct AttemptRecord {\n    work_status: WorkStatus,').replace('let mut record = AttemptRecord {','let mut record = AttemptRecord {\n        work_status: WorkStatus::default(),')
a=s.index('/// A refusal: no rows, no escalation.')
methods='''impl AttemptRecord {
    pub fn work_status(&self) -> WorkStatus {
        self.work_status.join(self.work.checked_lme().status()).join(self.k4_work.checked_lme().status())
            .join(self.stages.checked_total().status()).join(self.shared_stages.checked_total().status())
    }
    fn latch(&mut self, work: WorkTotal) { self.work_status = self.work_status.join(work.status()); }
    pub fn checked_own_work(&self) -> WorkTotal { self.work.checked_lme().add(self.k4_work.checked_lme()).join_status(self.work_status()) }
    pub fn checked_shared_work(&self) -> WorkTotal { WorkTotal::exact_count(self.shared_work).join_status(self.work_status()) }
    pub fn checked_verification_shared_work(&self) -> WorkTotal { WorkTotal::exact_count(self.verification_shared_work).join_status(self.work_status()) }
    pub fn checked_verification_work(&self) -> WorkTotal { WorkTotal::exact_count(self.verification_work).join_status(self.work_status()) }
    pub fn checked_stop_rule_work(&self) -> WorkTotal { WorkTotal::exact_count(self.stop_rule_work).join_status(self.work_status()) }
    pub fn checked_case_charge(&self) -> WorkTotal { self.checked_own_work().add(self.checked_shared_work()).add(self.checked_verification_shared_work()) }
    pub fn checked_invocation_increment(&self) -> WorkTotal {
        self.checked_own_work().add(self.checked_shared_work().mul(u64::from(self.shared_built_here)))
            .add(self.checked_verification_shared_work().mul(u64::from(self.verification_shared_built_here)))
    }
}
impl std::fmt::Debug for AttemptRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("AttemptRecord");
'''
fields=re.findall(r'    pub (\w+):',s[s.index('pub struct AttemptRecord'):a])
methods+=''.join(f'        d.field("{f}", &self.{f});\n' for f in fields)
methods+='''        if !self.work_status().is_exact() { d.field("work_status", &self.work_status()); }
        d.finish()
    }
}

''';s=s[:a]+methods+s[a:]
# Actual event increments, never meter subtraction.
s=s.replace('    used: WorkTotal,\n}','    used: WorkTotal,\n    invocation_increment: WorkTotal,\n}')
a=s.index('/// The shared stages at p: reused')
s=s[:a]+'''impl CaseBudget {
    fn charge(&mut self, case: WorkTotal, invocation: WorkTotal, meter: &mut InvocationMeter) {
        self.used = self.used.add(case);
        self.invocation_increment = self.invocation_increment.add(invocation);
        meter.charge(invocation);
        let status = self.used.status().join(self.invocation_increment.status()).join(meter.checked_charged().status());
        self.used = self.used.join_status(status);
        self.invocation_increment = self.invocation_increment.join_status(status);
        meter.charge(WorkTotal::zero().join_status(status));
    }
    fn retain<T>(&mut self, result: Result<T, AttemptStop>, record: &mut AttemptRecord, meter: &mut InvocationMeter) -> Result<T, AttemptStop> {
        record.latch(self.used);
        record.latch(meter.checked_charged());
        if let Err(AttemptStop::WorkAccounting(fault)) = &result { record.work_status = record.work_status.join(WorkStatus::from_fault(*fault)); }
        let status = record.checked_case_charge().status().join(record.checked_invocation_increment().status());
        record.work_status = record.work_status.join(status);
        self.charge(WorkTotal::zero().join_status(status), WorkTotal::zero().join_status(status), meter);
        match result { Ok(v) => { record.checked_case_charge().exact()?; Ok(v) }, Err(e) => Err(e) }
    }
}

'''+s[a:]
s=s.replace('record.shared_work = shared_total;','record.latch(shared_total);\n            record.shared_work = shared_total.legacy_saturated();')
s=s.replace('budget.used = budget.used.add(shared_total);\n            meter.charge(invocation_spent);','budget.charge(shared_total, invocation_spent, meter);')
s=s.replace('Err(stop) => (Err(stop), record),','Err(stop) => { let result = budget.retain(Err(stop), &mut record, meter); (result, record) },')
s=s.replace('budget.used = budget.used.add(spent.total);\n                    meter.charge(spent.total);','record.latch(spent.total);\n                    budget.charge(spent.total, spent.total, meter);')
s=s.replace('(Ok(PrecisionState::$variant(Arc::new(solved))), record)','{ let result = budget.retain(Ok(PrecisionState::$variant(Arc::new(solved))), &mut record, meter); (result, record) }')
s=s.replace('record.verification_shared_work = vs_total;','record.latch(vs_total);\n            record.verification_shared_work = vs_total.legacy_saturated();')
s=s.replace('record.shared_stages.add(&vs_stages);','let stage_merge = record.shared_stages.add(&vs_stages);')
s=s.replace('if built { vs_total } else { 0 }','if built { vs_total } else { vs_total.mul(0) }')
s=s.replace('budget.used = budget.used.add(vs_total);\n            meter.charge(invocation_spent);','budget.charge(vs_total, invocation_spent, meter);')
s=s.replace('            let vs = vs?;','            let vs = budget.retain(vs, record, meter)?;\n            stage_merge?;')
s=s.replace('if vs_total > case_room {','if vs_total.exact()? > case_room.exact()? {')
s=s.replace('record.stages.add(&spent.stages);','let stage_merge = record.stages.add(&spent.stages);')
s=s.replace('record.verification_work = spent.total;','record.latch(spent.total);\n            record.verification_work = spent.total.legacy_saturated();')
s=s.replace('budget.used = budget.used.add(spent.total);\n            meter.charge(spent.total);\n            let report_state = VerificationState::$variant(Arc::new(spent.result?));','budget.charge(spent.total, spent.total, meter);\n            let result = budget.retain(spent.result, record, meter)?;\n            stage_merge?;\n            let report_state = VerificationState::$variant(Arc::new(result));')
# Record stop stages before custody, then preserve the original error if present.
s=s.replace('record.stop_rule_work += decision.total;\n            record.stages.stop_rule += decision.total;','record.latch(decision.total);\n            let stop_work = record.checked_stop_rule_work().add(decision.total);\n            record.latch(stop_work);\n            record.stop_rule_work = stop_work.legacy_saturated();\n            let _ = record.stages.add_to(Stage::StopRule, decision.total);')
s=s.replace('budget.used = budget.used.add(decision.total);\n        meter.charge(decision.total);\n        match decision.result {','budget.charge(decision.total, decision.total, meter);\n        let decision_result = budget.retain(decision.result.clone(), &mut attempts[candidate_index], meter);\n        match decision_result {')
s=s.replace('record.stop_rule_work = record.stop_rule_work.saturating_add(certificate.total);\n                record.stages.stop_rule = record.stages.stop_rule.saturating_add(certificate.total);','record.latch(certificate.total);\n                let stop_work = record.checked_stop_rule_work().add(certificate.total);\n                record.latch(stop_work);\n                record.stop_rule_work = stop_work.legacy_saturated();\n                let _ = record.stages.add_to(Stage::StopRule, certificate.total);')
s=s.replace('budget.used = budget.used.add(certificate.total);\n                meter.charge(certificate.total);\n                match certificate.result {','budget.charge(certificate.total, certificate.total, meter);\n                let certificate_result = budget.retain(certificate.result, record, meter);\n                match certificate_result {')
s=s.replace('match stop {\n        AttemptStop::Budget','match stop {\n        AttemptStop::WorkAccounting(fault) => Ok(UnresolvedReason::WorkAccounting { fault: *fault, prior: None }),\n        AttemptStop::Budget')
# Core outcome is retained until the explicit legacy projection.
a=s.index('/// The schedule of one case');b=s.index('/// Geometry first, then',a)
chunk=s[a:b].replace('CaseOutcome','ExecutionOutcome')
chunk=chunk.replace('pub(crate) fn run_schedule(','fn run_schedule_inner(').replace('    case_limit: CaseLimit,','    budget: &mut CaseBudget,')
chunk=chunk.replace('    let mut budget = CaseBudget {\n        limit: case_limit.get(),\n        used: WorkTotal::zero(),\n    };','')
chunk=chunk.replace('&mut budget,','budget,')
chunk=chunk.replace('        if stop.escalates() {','        if attempts.iter().any(|a| !a.work_status().is_exact()) { return finish_terminal(&stop, attempts, geometry); }\n                        if stop.escalates() {')
chunk=chunk.replace('    match terminal(stop) {','''    let status = attempts.iter().fold(WorkStatus::default(), |status, a| status.join(a.checked_case_charge().status()));
    if let Some(fault) = status.fault() {
        return ExecutionOutcome::Unresolved { reason: UnresolvedReason::WorkAccounting {
            fault, prior: if matches!(stop, AttemptStop::WorkAccounting(_)) { None } else { Some(stop.clone()) }
        }, attempts, geometry };
    }
    match terminal(stop) {''')
chunk=chunk.replace('Err(refusal) => ExecutionOutcome::Refused { refusal, geometry },','Err(refusal) => ExecutionOutcome::Refused { refusal, attempts, geometry },')
s=s[:a]+chunk+s[b:]
a=s.index('/// The schedule of one case')
s=s[:a]+'''/// The actual work of one core run, retained before compatibility projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunWork {
    case: WorkTotal,
    invocation_before: WorkTotal,
    invocation_increment: WorkTotal,
    invocation_after: WorkTotal,
}
impl RunWork {
    pub fn case(&self) -> WorkTotal { self.case }
    pub fn invocation_before(&self) -> WorkTotal { self.invocation_before }
    pub fn invocation_increment(&self) -> WorkTotal { self.invocation_increment }
    pub fn invocation_after(&self) -> WorkTotal { self.invocation_after }
}
#[derive(Debug, Clone)]
pub enum ExecutionOutcome {
    Selected(Box<RetainedSolve>),
    Refused { refusal: Refusal, attempts: Vec<AttemptRecord>, geometry: Vec<BodyGeometry> },
    Unresolved { reason: UnresolvedReason, attempts: Vec<AttemptRecord>, geometry: Vec<BodyGeometry> },
}
impl ExecutionOutcome {
    fn into_legacy(self) -> CaseOutcome {
        match self {
            Self::Selected(s) => CaseOutcome::Selected(s),
            Self::Refused { refusal, geometry, .. } => CaseOutcome::Refused { refusal, geometry },
            Self::Unresolved { reason, attempts, geometry } => CaseOutcome::Unresolved { reason, attempts, geometry },
        }
    }
}
pub(crate) struct CoreRun { pub(crate) outcome: ExecutionOutcome, pub(crate) work: RunWork }
impl CoreRun { pub(crate) fn into_legacy(self) -> CaseOutcome { self.outcome.into_legacy() } }
pub(crate) fn run_core(prep: Arc<CasePrep>, group: Arc<GroupPrep>, cache: &mut GroupCache, case_limit: CaseLimit, meter: &mut InvocationMeter) -> CoreRun {
    let invocation_before = meter.checked_charged();
    let mut budget = CaseBudget { limit: case_limit.get(), used: WorkTotal::zero(), invocation_increment: WorkTotal::zero() };
    let outcome = if let Some(fault) = invocation_before.status().fault() {
        budget.used = budget.used.join_status(invocation_before.status());
        budget.invocation_increment = budget.invocation_increment.join_status(invocation_before.status());
        ExecutionOutcome::Unresolved { reason: UnresolvedReason::WorkAccounting { fault, prior: None }, attempts: Vec::new(), geometry: group.geometry.clone() }
    } else { run_schedule_inner(prep, group, cache, &mut budget, meter) };
    CoreRun { outcome, work: RunWork { case: budget.used, invocation_before, invocation_increment: budget.invocation_increment, invocation_after: meter.checked_charged() } }
}
pub(crate) fn run_schedule(prep: Arc<CasePrep>, group: Arc<GroupPrep>, cache: &mut GroupCache, case_limit: CaseLimit, meter: &mut InvocationMeter) -> CaseOutcome {
    run_core(prep, group, cache, case_limit, meter).into_legacy()
}

'''+s[a:]
s=s.replace('    for source in sources {\n        if meter.exhausted() {','''    for source in sources {
        if let Some(fault) = meter.checked_charged().status().fault() {
            out.push(CaseOutcome::Unresolved { reason: UnresolvedReason::WorkAccounting { fault, prior: None }, attempts: Vec::new(), geometry: Vec::new() });
            continue;
        }
        if meter.exhausted() {''')
# Certificate failure must transfer detached local faults into retained total.
s=s.replace('    PublicationSpent {\n        result,','    let total = finish_work(&mut result, meter.total(), &StageWork::default());\n    PublicationSpent {\n        result,').replace('        total: meter.total(),','        total,')
p.write_text(s)
p=p.parent.parent.parent/'structural.rs';s=p.read_text().replace('StageWork, StorageCounts,','ExecutionOutcome, RunWork, StageWork, StorageCounts,');p.write_text(s)
