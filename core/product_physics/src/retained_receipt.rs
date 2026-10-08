//! Private typed C3 seam. No serializer, public receipt, profile or selection.
#![allow(dead_code)]
use super::retained_product as p;
use open_pipe_stress_frame_kernel::structural::retained_api as k;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(super) enum Stage {Preparation=0,Native,ProofStart,Projection,Maxima,Values,Aliases,Certificate,Observables,G5a}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub(super) enum StageState {#[default] NotEntered,Entered,Completed,Failed}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub(super) enum CheckState {#[default] NotEntered,Passed,Failed}
#[derive(Debug,Clone,Copy,PartialEq,Eq,Default)]
pub(super) enum OldCoverage {#[default] CapturedPrefix,Complete}
#[derive(Debug)]
pub(super) enum PreparationResult {Entered,Prepared(k::PreparedSectionBits),Refused(k::SectionPreparationError)}
#[derive(Debug)]
pub(super) struct PreparationEntry {
    pub member:u32,pub old_source:[u64;6],pub old_facts:[u64;7],
    pub result:PreparationResult,pub work_index:usize,
}
/// Additional typed instrumentation owner. These are local events/copy bytes,
/// not native LME or a complete memory profile. Loss never erases a prior event.
#[derive(Debug,Default)]
pub(super) struct TraceCosts {pub events:u64,pub copy_bytes:u64,pub lost:bool}
impl TraceCosts {
    pub fn record<T>(&mut self) {
        match (self.events.checked_add(1),u64::try_from(std::mem::size_of::<T>()).ok().and_then(|n|self.copy_bytes.checked_add(n))) {
            (Some(e),Some(b))=>{self.events=e;self.copy_bytes=b;},_=>self.lost=true,
        }
    }
}
#[derive(Debug)]
pub(super) struct PrivateAdapterSnapshot {
    pub counts:[u64;10],pub fault:Option<p::AdapterFault>,pub prepared_capacity_bytes:[usize;16],
    pub observation_capacity_bytes:[usize;3],pub support_capacity_bytes:[usize;7],
}
#[derive(Default)]
pub(super) struct PreparedTrace {
    pub stages:[StageState;10],pub checks:[CheckState;3],pub old_coverage:OldCoverage,
    pub old_vector_swapped:bool,pub members:Vec<PreparationEntry>,pub source_ready:bool,
    pub native_error:Option<p::CaptureError>,pub proof_ready:bool,pub private_commit_precharged:bool,
    pub private_committed:bool,pub adapter:Option<PrivateAdapterSnapshot>,pub costs:TraceCosts,
}
impl PreparedTrace {
    pub fn enter(&mut self,stage:Stage) {self.costs.record::<StageState>();self.stages[stage as usize]=StageState::Entered;}
    pub fn completed(&mut self,stage:Stage) {self.costs.record::<StageState>();self.stages[stage as usize]=StageState::Completed;}
    pub fn fail_entered(&mut self) {
        self.costs.record::<[StageState;10]>();
        for s in &mut self.stages {if *s==StageState::Entered {self.costs.record::<StageState>();*s=StageState::Failed;}}
    }
    pub fn checked(&mut self,stage:Stage,index:usize,passed:bool) {
        self.costs.record::<CheckState>();self.checks[index]=if passed{CheckState::Passed}else{CheckState::Failed};
        self.costs.record::<StageState>();self.stages[stage as usize]=if passed{StageState::Completed}else{StageState::Failed};
    }
    /// One private terminal snapshot. It makes no claim about the later public
    /// transaction, receipt work or future public commit-plan precharges.
    pub fn freeze(&mut self,capture:&p::ProductCapture) {
        if self.adapter.is_some(){return;}
        self.costs.record::<PrivateAdapterSnapshot>();
        self.adapter=Some(PrivateAdapterSnapshot {counts:capture.adapter.counts.get(),fault:capture.adapter.fault.get(),
            prepared_capacity_bytes:capture.prepared_capacity_bytes,observation_capacity_bytes:capture.observation_capacity_bytes,
            support_capacity_bytes:capture.support_capacity_bytes});
    }
}
#[cfg(test)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(super) enum TraceFault {AfterPrelude,AfterHelper,AfterEvaluation,SourceConstruction,Maxima,ValuesCompletion}

#[derive(Debug)]
pub(super) enum TraceProjectionError {MissingTerminal,MissingFailure,UnreturnedHelper,WorkAssociation,LostTrace,StageConsistency}
#[derive(Debug)]
pub(super) enum FailureRef<'a> {
    Preparation{capture:&'a p::CaptureError,section:Option<&'a k::SectionPreparationError>},
    Native(&'a p::CaptureError),Candidate(&'a p::PreparedCandidateError),
    Proof(&'a k::ProductFailure),Observable(&'a p::CaptureError),G5a(&'a p::G5aFailure),
}
#[derive(Debug)]
pub(super) enum ResultRef<'a> {Ready,Unavailable(FailureRef<'a>)}
#[derive(Debug)]
pub(super) enum CheckRef<'a> {NotEntered,Passed,Failed(FailureRef<'a>)}
#[derive(Debug)]
pub(super) enum CompletionRef {NotEntered,Merged,SeparateFailure{visits:k::WorkTotal,capacity_bytes:usize}}
#[derive(Default)]
pub(super) struct ProjectionWork {pub local:TraceCosts,pub kernel:k::TraceCopyWork}
/// I57 §1 closed compact body of C3 `summary_coverage`. Estimate and charge are
/// rederived from public facts and are never carried here.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub(super) struct CoverageBody {pub body:u32,pub stop:[bool;4],pub has_data:bool}
/// The proof-owned roster after its rederivation check. Only `project` builds it.
#[derive(Debug,Clone,Copy)]
pub(super) struct CompleteCoverage<'a> {entries:&'a [k::ProductSummaryCoverage]}
impl<'a> CompleteCoverage<'a> {
    pub fn len(&self)->usize {self.entries.len()}
    /// Ascending native body ids 0..body_count-1, one compact entry each.
    pub fn bodies(&self)->impl Iterator<Item=CoverageBody>+'a {
        self.entries.iter().map(|c|CoverageBody{body:c.body,stop:c.stop,has_data:c.has_data})
    }
}
/// Typed C3 coverage of a non-null ProofTrace. Null means no complete vector was
/// retained by this proof; it is never zero coverage and never a prefix.
#[derive(Debug,Clone,Copy)]
pub(super) enum SummaryCoverage<'a> {Null,Complete(CompleteCoverage<'a>)}
/// I57 §3 custody and stage rules. The vector is the proof's own borrowed slice,
/// never `ProductCapture.summary_coverage`. No coverage is computed here.
fn summary_coverage<'a>(trace:&PreparedTrace,capture:&'a p::ProductCapture,result:&ResultRef<'_>,
    proof:Option<&k::ProductProofTrace<'a>>,costs:&mut ProjectionWork)->Result<Option<SummaryCoverage<'a>>,TraceProjectionError> {
    let Some(proof)=proof else {return Ok(None)};
    costs.local.record::<SummaryCoverage<'_>>();
    let entries=proof.summary_coverage;
    if entries.is_empty() {
        // final_case assigns the vector only after every body completes, so empty is
        // null. A Ready product, completed certificate or passed G5a requires coverage.
        if matches!(result,ResultRef::Ready) || trace.checks[0]==CheckState::Passed || trace.checks[2]==CheckState::Passed {
            return Err(TraceProjectionError::StageConsistency);
        }
        return Ok(Some(SummaryCoverage::Null));
    }
    let completed=|s:Stage|trace.stages[s as usize]==StageState::Completed;
    let lanes=matches!(&proof.lanes,[Some(a),Some(b)] if a.law==k::ReadoutLaw::AdmittedK && a.result.is_ok()
        && b.law==k::ReadoutLaw::AnnularSource && b.result.is_ok());
    if !lanes || !trace.source_ready || capture.source.is_none()
        || ![Stage::Preparation,Stage::Native,Stage::ProofStart,Stage::Projection,Stage::Maxima,Stage::Values,Stage::Aliases].into_iter().all(completed)
        || trace.stages[Stage::Certificate as usize]==StageState::NotEntered || trace.checks[0]==CheckState::NotEntered {
        return Err(TraceProjectionError::StageConsistency);
    }
    let Some(case)=capture.native.as_ref() else {return Err(TraceProjectionError::StageConsistency)};
    let k::ExecutionOutcome::Selected(owner)=&case.outcome else {return Err(TraceProjectionError::StageConsistency)};
    match proof.check_summary_coverage(owner,&mut costs.kernel) {
        Ok(())=>Ok(Some(SummaryCoverage::Complete(CompleteCoverage{entries}))),
        Err(e) if e.category()=="association"=>Err(TraceProjectionError::WorkAssociation),
        Err(_)=>Err(TraceProjectionError::LostTrace),
    }
}
pub(super) struct PreparedAttemptView<'a> {
    pub case:&'a str,pub result:ResultRef<'a>,pub stages:&'a [StageState;10],
    pub old_coverage:OldCoverage,pub old_vector_swapped:bool,
    pub members:&'a [PreparationEntry],pub preparation_work:&'a [k::SectionPreparationWork],
    pub operational_old:&'a [p::OperationalSpent],pub operational_new:&'a [p::OperationalSpent],
    pub prepared_source:Option<&'a k::PrimitiveSource>,pub native_run:Option<usize>,
    pub proof:Option<k::ProductProofTrace<'a>>,
    /// None iff `proof` is None; otherwise null or the complete proof-owned roster.
    pub summary_coverage:Option<SummaryCoverage<'a>>,pub proof_ready:bool,pub checks:[CheckRef<'a>;3],
    pub numeric_failure:Option<&'a k::ProductFailure>,pub capture_failure:Option<&'a p::CaptureError>,
    pub completion:CompletionRef,pub adapter:&'a PrivateAdapterSnapshot,
    pub overlay_work:&'a p::ScalarWork,pub g5a_work:&'a p::ScalarWork,pub trace_costs:&'a TraceCosts,
    pub private_commit_precharged:bool,pub private_committed:bool,
}
pub(super) fn project<'a>(trace:&'a PreparedTrace,capture:&'a p::ProductCapture,
    preparation_work:&'a [k::SectionPreparationWork],old_operational:&'a [p::OperationalSpent],overlay:&'a p::ScalarWork,
    result:ResultRef<'a>,proof:Option<&'a k::ProductCertificateSpent<'static>>,proof_error:Option<&'a k::ProductFailure>,
    values_failure:Option<&'a k::ProductValuesFailure>,costs:&mut ProjectionWork)->Result<PreparedAttemptView<'a>,TraceProjectionError> {
    costs.local.record::<PreparedAttemptView<'_>>();
    let adapter=trace.adapter.as_ref().ok_or(TraceProjectionError::MissingTerminal)?;
    if trace.costs.lost || costs.local.lost {return Err(TraceProjectionError::LostTrace);}
    for entry in &trace.members {
        costs.local.record::<usize>();
        if matches!(entry.result,PreparationResult::Entered){return Err(TraceProjectionError::UnreturnedHelper);}
        let work=preparation_work.get(entry.work_index).ok_or(TraceProjectionError::WorkAssociation)?;
        if !work.trace_copy_work.status().is_exact(){return Err(TraceProjectionError::LostTrace);}
    }
    if costs.local.lost {return Err(TraceProjectionError::LostTrace);}
    if trace.stages.contains(&StageState::Entered) {return Err(TraceProjectionError::StageConsistency);}
    if matches!(result,ResultRef::Ready) && (trace.stages.iter().any(|s|*s!=StageState::Completed)
        || trace.checks.iter().any(|s|*s!=CheckState::Passed)) {return Err(TraceProjectionError::StageConsistency);}
    let certificate=match trace.checks[0] {CheckState::NotEntered=>CheckRef::NotEntered,CheckState::Passed=>CheckRef::Passed,
        CheckState::Failed=>CheckRef::Failed(FailureRef::Proof(proof_error.ok_or(TraceProjectionError::MissingFailure)?))};
    let observable=match trace.checks[1] {CheckState::NotEntered=>CheckRef::NotEntered,CheckState::Passed=>CheckRef::Passed,
        CheckState::Failed=>CheckRef::Failed(FailureRef::Observable(capture.observable_error.as_ref().ok_or(TraceProjectionError::MissingFailure)?))};
    let g5a=match trace.checks[2] {CheckState::NotEntered=>CheckRef::NotEntered,CheckState::Passed=>CheckRef::Passed,
        CheckState::Failed=>CheckRef::Failed(FailureRef::G5a(capture.g5a_error.as_ref().ok_or(TraceProjectionError::MissingFailure)?))};
    if trace.stages[Stage::ProofStart as usize]!=StageState::NotEntered && proof.is_none() {
        return Err(TraceProjectionError::MissingFailure);
    }
    // RV77-N4 (I61 U2, failure path): proof work, certified or refused, binds
    // structurally to this capture's selected owner through the proof anchor.
    if let Some(work)=proof {
        let owner=match capture.native.as_ref().map(|case|&case.outcome) {
            Some(k::ExecutionOutcome::Selected(owner))=>owner,_=>return Err(TraceProjectionError::WorkAssociation)};
        if !work.owner_matches(owner) {return Err(TraceProjectionError::WorkAssociation);}
    }
    let proof=proof.map(|p|p.typed_trace(&mut costs.kernel));
    let completion=if let Some(f)=values_failure {CompletionRef::SeparateFailure{visits:f.visits,capacity_bytes:f.capacity}}
        else if proof.as_ref().is_some_and(|p|p.completion_merged){CompletionRef::Merged}else{CompletionRef::NotEntered};
    if !costs.kernel.status().is_exact() || proof.as_ref().is_some_and(|p|!p.trace_copy_work.status().is_exact()) {
        return Err(TraceProjectionError::LostTrace);
    }
    let summary_coverage=summary_coverage(trace,capture,&result,proof.as_ref(),costs)?;
    if !costs.kernel.status().is_exact() || costs.local.lost {return Err(TraceProjectionError::LostTrace);}
    let (old,new)=if trace.old_vector_swapped {(old_operational,capture.operational.as_slice())}
        else{(capture.operational.as_slice(),&[][..])};
    Ok(PreparedAttemptView {case:&capture.case_id,result,stages:&trace.stages,old_coverage:trace.old_coverage,
        old_vector_swapped:trace.old_vector_swapped,members:&trace.members,preparation_work,
        operational_old:old,operational_new:new,prepared_source:if trace.source_ready{capture.source.as_ref()}else{None},
        native_run:capture.native.as_ref().map(|c|c.run),proof,summary_coverage,proof_ready:trace.proof_ready,
        checks:[certificate,observable,g5a],numeric_failure:capture.numeric_failure.as_ref(),capture_failure:capture.error.as_ref(),
        completion,adapter,overlay_work:overlay,g5a_work:&capture.g5a_work,
        trace_costs:&trace.costs,private_commit_precharged:trace.private_commit_precharged,private_committed:trace.private_committed})
}
