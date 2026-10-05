from pathlib import Path
import re
k=Path('projects/chirality-piping/core/solver/frame_kernel/src/structural/retained')
# Fallible raw observations, with the explicitly bounded fresh helper exceptions.
for name in ['adaptive','verify','assemble','bound','directed','factor','source']:
 p=k/(name+'.rs');s=p.read_text()
 s=s.replace('.signum()', '.signum()?').replace('.make_absolute();','.make_absolute()?;')
 if name=='adaptive':
  s=s.replace('Some(sum.signum()? > 0)','Some(sum.signum().ok()? > 0)')
  # This accumulator is legacy ExactAccumulator, not ExactWideSum.
  s=s.replace('accumulator.signum()?','accumulator.signum()')
  for n in ['n','den','num']:
   s=s.replace(n+'.is_zero()',n+'.is_zero()?')
  # avoid modifying Wide suffix identifiers, e.g. denominator not relevant
 if name=='factor': s=s.replace('Some(!sum.is_zero())','Some(!sum.is_zero().ok()?)')
 if name=='source':
  s=s.replace('s.is_zero()','s.is_zero().expect("fresh chord difference: work <= 1152")').replace('c.is_zero()','c.is_zero().expect("fresh chord cross product: work <= 2688")')
 p.write_text(s)
p=k/'adaptive.rs';s=p.read_text()
s=s.replace('use super::wide_sum::{ExactWideSum, SumRefusal, SumWork};','use super::wide_sum::{CloneWork, ExactWideSum, SumRefusal, SumWork};\nuse super::work::{WorkFault, WorkStatus, WorkStream, WorkTotal};')
s=s.replace('    charged: u64,','    charged: WorkTotal,').replace('Self { limit, charged: 0 }','Self { limit, charged: WorkTotal::zero() }').replace('        self.charged\n','        self.charged.legacy_saturated()\n')
s=s.replace('        self.charged >= self.limit','        self.charged.exact().is_err_or(|v| v >= self.limit)').replace('    fn room(&self) -> u64 {\n        self.limit.saturating_sub(self.charged)','    pub fn checked_charged(&self) -> WorkTotal { self.charged }\n    fn room(&self) -> WorkTotal {\n        self.charged.room(self.limit)').replace('fn charge(&mut self, amount: u64)','fn charge(&mut self, amount: WorkTotal)').replace('self.charged.saturating_add(amount)','self.charged.add(amount)')
s=s.replace('pub enum AttemptStop {','pub enum AttemptStop {\n    WorkAccounting(WorkFault),').replace('SumRefusal::Span =>','SumRefusal::WorkAccounting(fault) => Self::WorkAccounting(fault),\n            SumRefusal::Span =>').replace('WideError::ExponentRange =>','WideError::WorkAccounting(fault) => Self::WorkAccounting(fault),\n            WideError::ExponentRange =>')
s=s.replace('pub enum UnresolvedReason {','pub enum UnresolvedReason {\n    WorkAccounting { fault: WorkFault, prior: Option<AttemptStop> },')
s=s.replace('pub(crate) fn lme<const L: usize>(ctx: &WideContext<L>) -> u64','pub(crate) fn lme<const L: usize>(ctx: &WideContext<L>) -> WorkTotal').replace('    ctx.work().limb_multiply_equivalents(L)','    ctx.work().checked_lme(L)')
s=s.replace('    base: u64,\n    case_room: u64,\n    invocation_room: u64,','    base: WorkTotal,\n    case_room: WorkTotal,\n    invocation_room: WorkTotal,')
s=s.replace('base: 0,','base: WorkTotal::zero(),').replace('case_room: u64::MAX,','case_room: WorkTotal::exact_count(u64::MAX),').replace('invocation_room: u64::MAX,','invocation_room: WorkTotal::exact_count(u64::MAX),').replace('            case_room,\n            invocation_room: WorkTotal','            case_room: WorkTotal::exact_count(case_room),\n            invocation_room: WorkTotal')
s=s.replace('pub(crate) fn with_base(self, base: u64) -> Self {\n        Self { base, ..self }','pub(crate) fn with_base(self, base: WorkTotal) -> Result<Self, AttemptStop> {\n        base.exact()?; self.case_room.exact()?; self.invocation_room.exact()?;\n        Ok(Self { base, ..self })')
s=s.replace('pub(crate) fn test(&self, used: u64) -> Result<(), AttemptStop> {\n        if used > self.case_room {','pub(crate) fn test(&self, used: WorkTotal) -> Result<(), AttemptStop> {\n        let used = used.exact()?;\n        let case_room = self.case_room.exact()?;\n        let invocation_room = self.invocation_room.exact()?;\n        if used > case_room {').replace('used > self.invocation_room','used > invocation_room')
s=s.replace('.saturating_add(lme(ctx))','.add(lme(ctx))').replace('.saturating_add(sum.work().limb_multiply_equivalents())','.add(sum.work().checked_lme())')
# Stage ledger: retain public numeric compatibility fields but gate composition.
fields=['formation','assembly','residual_formation','factor','condition','rhs','solve','refinement','recovery','stop_rule','bounded_gate','scale','estimate','charge','bound','shift','bounded_formation','wide_formation','uc']
variants={f:''.join(x.title() for x in f.split('_')) for f in fields}
s=s.replace('#[derive(Debug, Clone, Default, PartialEq, Eq)]\npub struct StageWork {','#[derive(Clone, Default, PartialEq, Eq)]\npub struct StageWork {\n    status: WorkStatus,').replace('pub(crate) enum Stage {','pub(crate) enum Stage {\n    StopRule,')
a=s.index('impl StageWork {');b=s.index('/// Formation, assembly,',a)
s=s[:a]+'''impl StageWork {
    pub fn checked_total(&self) -> WorkTotal {
        ['''+', '.join('self.'+f for f in fields)+'''].into_iter().fold(WorkTotal::zero().join_status(self.status), |t, n| t.add(WorkTotal::exact_count(n)))
    }
    pub(crate) fn total(&self) -> u64 { self.checked_total().legacy_saturated() }
    fn slot(&mut self, stage: Stage) -> &mut u64 {
        match stage {
'''+''.join(f'            Stage::{v} => &mut self.{f},\n' for f,v in variants.items())+'''        }
    }
    pub(crate) fn set(&mut self, stage: Stage, work: WorkTotal) -> Result<(), WorkFault> {
        self.status = self.status.join(work.status());
        *self.slot(stage) = work.legacy_saturated();
        self.status = self.status.join(self.checked_total().status());
        self.checked_total().exact().map(|_| ())
    }
    pub(crate) fn add_to(&mut self, stage: Stage, work: WorkTotal) -> Result<(), WorkFault> {
        let old = WorkTotal::exact_count(*self.slot(stage));
        self.set(stage, old.add(work))
    }
    pub(crate) fn close_stopped<T, E>(&mut self, result: &Result<T, E>, current: Stage, total: WorkTotal) {
        self.status = self.status.join(total.status()).join(self.checked_total().status());
        if result.is_err() {
            let rest = total.remainder(self.checked_total());
            self.status = self.status.join(rest.status());
            if rest.status().is_exact() { let _ = self.add_to(current, rest); }
        }
    }
    pub fn merge(&mut self, other: &Self) -> Result<(), WorkFault> {
        self.status = self.status.join(other.status);
'''+''.join(f'        let _ = self.add_to(Stage::{v}, WorkTotal::exact_count(other.{f}));\n' for f,v in variants.items())+'''        self.checked_total().exact().map(|_| ())
    }
    pub(crate) fn add(&mut self, other: &Self) -> Result<(), WorkFault> { self.merge(other) }
}
impl std::fmt::Debug for StageWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("StageWork");
'''+''.join(f'        d.field("{f}", &self.{f});\n' for f in fields)+'''        if !self.status.is_exact() { d.field("work_status", &self.status); }
        d.finish()
    }
}

impl From<WorkFault> for AttemptStop { fn from(fault: WorkFault) -> Self { Self::WorkAccounting(fault) } }

pub(crate) fn finish_work<T>(result: &mut Result<T, AttemptStop>, total: WorkTotal, stages: &StageWork) -> WorkTotal {
    let mut total = total.join_status(stages.checked_total().status());
    if let Err(AttemptStop::WorkAccounting(fault)) = result { total = total.join_status(WorkStatus::from_fault(*fault)); }
    if result.is_ok() { if let Err(fault) = total.exact() { *result = Err(fault.into()); } }
    total
}

'''+s[b:]
# all actual accounting calculations use typed totals
s=s.replace('total: u64','total: WorkTotal').replace('total: 0,','total: WorkTotal::zero(),')
s=s.replace('.limb_multiply_equivalents()', '.checked_lme()')
s=s.replace('.saturating_add(self.sums.checked_lme())','.add(self.sums.checked_lme())')
s=s.replace('let result = run();','let mut result = run();')
s=s.replace('stages.close_stopped(&result, current, total);','stages.close_stopped(&result, current, total);\n    let total = finish_work(&mut result, total, &stages);')
s=s.replace('    StopDecision {\n        result,','    let total = finish_work(&mut result, total, &StageWork::default());\n    StopDecision {\n        result,')
s=s.replace('    used: u64,','    used: WorkTotal,').replace('used: 0,','used: WorkTotal::zero(),').replace('budget.limit.saturating_sub(budget.used)','budget.used.room(budget.limit)').replace('budget.used.saturating_add(', 'budget.used.add(')
s=s.replace('(AttemptStop, u64, StageWork','(AttemptStop, WorkTotal, StageWork').replace('AttemptStop>, u64, bool, StageWork','AttemptStop>, WorkTotal, bool, StageWork')
s=s.replace('    u64,\n    bool,\n    StageWork,','    WorkTotal,\n    bool,\n    StageWork,')
s=s.replace('if built { shared_total } else { 0 }','if built { shared_total } else { shared_total.mul(0) }').replace('shared_total > case_room','shared_total.exact().is_ok_and(|v| case_room.exact().is_ok_and(|room| v > room))').replace('invocation_room.saturating_sub(invocation_spent)','invocation_spent.room(invocation_room.legacy_saturated()).join_status(invocation_room.status())')
# raw fallible calls and all guards before entering a stage
s=re.sub(r'\.with_base\(([^;\n]+)\)([;,])',r'.with_base(\1)?\2',s)
s=s.replace('.with_base(0)?','.with_base(WorkTotal::zero())?')
s=s.replace('&guard.with_base(guard.base + base),','&guard.with_base(guard.base + base)?,')
# Work snapshots: one cohort for each staged context owner; never mixed subsets.
for start,end,expr in [('fn build_shared<','/// The residual gate','lme(&ctx) + lme(&ctx_q) + lme(&ctx16) + lme(&ctx64) + sum.work().checked_lme()'),('fn solve_case_at<','// ------------------------------------------------------------ the stop rule','total(&ctx, &ctx_q, &ctx16, &ctx64, &sum)')]:
 a=s.index(start);b=s.index(end,a);chunk=s[a:b]
 chunk=chunk.replace('    let mut stages = StageWork::default();','    let mut stages = StageWork::default();\n    let stream = WorkStream::new();')
 chunk=re.sub(r'let (t[0-9]|tf) = .*?;',lambda m:f'let {m[1]} = stream.snapshot({expr})?;',chunk,flags=re.S)
 chunk=re.sub(r'\b(t[0-9]|tf) - (t[0-9]|tf)\b',r'\1.delta_since(\2)',chunk)
 chunk=chunk.replace('let mut fallback_work = 0u64;','let mut fallback_work = WorkTotal::zero();').replace('total(&ctx, &ctx_q, &ctx16, &ctx64, &sum) - tf','stream.snapshot(total(&ctx, &ctx_q, &ctx16, &ctx64, &sum))?.delta_since(tf)').replace('let base = t3 -','let base = t3.total() -')
 chunk=re.sub(r'stages\.(\w+) = ([^;]+);',lambda m:f'stages.set(Stage::{variants[m[1]]}, {m[2]})?;',chunk)
 s=s[:a]+chunk+s[b:]
# terminal propagation from tracker collapse before prune
s=s.replace('self.offered += 1;','self.offered = self.offered.checked_add(1).ok_or(AttemptStop::Structure)?;')
s=s.replace('pub(crate) fn collapse(&mut self, ctx16: &mut WideContext<16>) {','pub(crate) fn collapse(&mut self, ctx16: &mut WideContext<16>) -> Result<(), AttemptStop> {').replace('if lazy.is_empty() {\n            return;','if lazy.is_empty() {\n            return Ok(());').replace('                Err(stop) => Evaluated::Refused','                Err(stop @ AttemptStop::WorkAccounting(_)) => return Err(stop),\n                Err(stop) => Evaluated::Refused').replace('        self.prune();\n    }','        self.prune();\n        Ok(())\n    }')
s=s.replace('.collapse(ctx16);','.collapse(ctx16)?;')
# private clone delta interface
start=s.index('/// Only newly incurred clone work');end=s.index('/// The candidate\'s new work',start);s=s[:start]+s[end:]
a=s.index('    fn clone_delta<T>(');b=s.index('\n    fn reaches(',a)
s=s[:a]+'''    fn collect_clone<T>(&mut self, operation: (Result<T, SumRefusal>, SumWork)) -> Result<T, AttemptStop> {
        let (result, delta) = operation;
        self.sums.merge(&delta);
        self.checked(result.map_err(AttemptStop::from))
    }
'''+s[b:]
a=s.index('        let mut n = num.clone();',s.index('    fn round_up('));b=s.index('        let quotient =',a)
s=s[:a]+'''        let mut n = CloneWork::new(num);
        let sign = self.collect_clone(n.signum())?;
        if sign < 0 { return Err(certificate_error(None, CertificateIssue::NegativeField)); }
        if sign == 0 { return Ok(0.0); }
        let operation = n.round(&mut self.ctx);
        let nv = self.collect_clone(operation)?;
        let mut den = ExactWideSum::new();
        let formed = den.add_binary64(1.0, false).map_err(AttemptStop::from);
        self.collect(&den, formed)?;
        let mut d = CloneWork::new(&den);
        let operation = d.round(&mut self.ctx);
        let dv = self.collect_clone(operation)?;
'''+s[b:]
p.write_text(s)
# verify: same checked totals and same-cohort snapshots
p=k/'verify.rs';s=p.read_text().replace('use super::wide_sum', 'use super::work::{WorkStream, WorkTotal};\nuse super::adaptive::finish_work;\nuse super::wide_sum')
s=s.replace('total: u64','total: WorkTotal').replace('total: 0,','total: WorkTotal::zero(),').replace('.limb_multiply_equivalents()', '.checked_lme()')
s=s.replace('    let mut stages = StageWork::default();','    let mut stages = StageWork::default();\n    let stream = WorkStream::new();')
s=re.sub(r'let (t[0-9]) = spent\(([^;]+)\);',r'let \1 = stream.snapshot(spent(\2))?;',s)
s=re.sub(r'\b(t[0-9]) - (t[0-9])\b',r'\1.delta_since(\2)',s)
s=re.sub(r'stages\.(\w+) = ([^;]+);',lambda m:f'stages.set(Stage::{variants[m[1]]}, {m[2]})?;',s)
s=re.sub(r'\.with_base\(([^;\n]+)\)([;,])',r'.with_base(\1)?\2',s)
s=s.replace('let result = run();','let mut result = run();').replace('stages.close_stopped(&result, current, total);','stages.close_stopped(&result, current, total);\n    let total = finish_work(&mut result, total, &stages);')
p.write_text(s)
