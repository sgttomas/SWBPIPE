// RV77 reviewer-owned stage-rule checks (not part of the candidate). Included as a
// child of retained_product_tests only in RV77's private archive copy:
//   #[cfg(test)] #[path = "<scratch>/rv77_pp.rs"] mod rv77_pp;
// Proof work is actual FK work on the actual prepared owner (I61's helper); the
// stage records are synthetic and labelled as such.
use super::*;

#[test]
fn rv77_stage_rules_null_and_non_null_prerequisites() {
    use retained_receipt::{project,ResultRef,FailureRef,TraceProjectionError as T,ProjectionWork,Stage,StageState,CheckState,SummaryCoverage};
    static ZERO:retained_product::ScalarWork=retained_product::ScalarWork{entered:0,checks:0,lost:false};
    // (a) Null with a passed G5a (certificate failed, not Ready) is refused.
    let (prepared,early)=i61_fk_certificate_failure(0);
    assert!(early.work().summary_coverage().is_empty());
    let mut t=i61_stage_trace(prepared.capture(),Some(false),false);
    t.enter(Stage::Observables);t.checked(Stage::Observables,1,true);t.enter(Stage::G5a);t.checked(Stage::G5a,2,true);
    let r=project(&t,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(early.failure())),
        Some(early.work()),Some(early.failure()),None,&mut ProjectionWork::default());
    assert!(matches!(r,Err(T::StageConsistency)),"null with passed G5a: {:?}",r.as_ref().err());
    // Control: the same record with G5a failed is not reachable without a G5a failure value; skip.
    // (b) A complete vector needs each of the seven prerequisite stages Completed.
    let (mut prepared,late)=i61_fk_certificate_failure(4);
    assert!(!late.work().summary_coverage().is_empty());
    let ok=i61_stage_trace(prepared.capture(),Some(false),false);
    let r=project(&ok,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
        Some(late.work()),Some(late.failure()),None,&mut ProjectionWork::default());
    assert!(matches!(r.as_ref().map(|v|v.summary_coverage),Ok(Some(SummaryCoverage::Complete(_)))),"control");
    for s in [Stage::Preparation,Stage::Native,Stage::ProofStart,Stage::Projection,Stage::Maxima,Stage::Values,Stage::Aliases] {
        let mut t=i61_stage_trace(prepared.capture(),Some(false),false);
        t.stages[s as usize]=StageState::Failed;
        let r=project(&t,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
            Some(late.work()),Some(late.failure()),None,&mut ProjectionWork::default());
        assert!(matches!(r,Err(T::StageConsistency)),"stage {s:?}");
    }
    // (c) Certificate stage not entered while its check is recorded (and the converse).
    for which in 0..2 {
        let mut t=i61_stage_trace(prepared.capture(),Some(false),false);
        if which==0 {t.stages[Stage::Certificate as usize]=StageState::NotEntered;} else {t.checks[0]=CheckState::NotEntered;}
        let failure=if which==1 {None} else {Some(late.failure())};
        let r=project(&t,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
            Some(late.work()),failure,None,&mut ProjectionWork::default());
        assert!(matches!(r,Err(T::StageConsistency)),"certificate-entry defect {which}: {:?}",r.as_ref().err());
    }
    // (d) The attempt's own prepared source must be present for non-null coverage.
    prepared.test_capture_mut().source=None;
    let t=i61_stage_trace(prepared.capture(),Some(false),false);
    let r=project(&t,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
        Some(late.work()),Some(late.failure()),None,&mut ProjectionWork::default());
    assert!(matches!(r,Err(T::StageConsistency)),"missing capture.source: {:?}",r.as_ref().err());
    println!("RV77_PP_STAGE_RULES ok");
}
