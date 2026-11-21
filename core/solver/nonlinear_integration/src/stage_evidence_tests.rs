//! Carrier revision 4 §2 and §7: collection by state. Every case starts from a
//! fresh empty recipient slot; donors come from a second attempt whose plan
//! has identical labels, so the evaluations differ only in identity.

use super::*;
use open_pipe_stress_frame_kernel::structural::{synthetic, NotVerifiedReason};

const DIGEST: u64 = 0x5eed;

fn b() -> StageKey {
    StageKey::new(BoundStage::ConfirmationStageB, None)
}
fn a() -> StageKey {
    StageKey::new(BoundStage::ConfirmationStageA, None)
}
fn influence(id: &str) -> StageKey {
    StageKey::new(BoundStage::ConfirmationInfluence, Some(id.into()))
}

fn inputs() -> MemberBoundInputs {
    MemberBoundInputs { w: vec![0.0, 1.0], w2: vec![0.0, 0.0], exponents: vec![0, 0], lambda_sigma: 0.5, lift_radius: Vec::new() }
}

fn evidence(keys: Vec<StageKey>) -> StageEvidence {
    let mut e = StageEvidence::planned(keys);
    e.set_matrix_digest(DIGEST);
    e
}

fn binding(e: &StageEvidence, slot: usize) -> EvaluationBinding {
    e.stages()[slot].binding.clone()
}

/// The slot's own evaluation: bound to it, with inputs and a verified record minted under it.
fn own_verified(e: &StageEvidence, slot: usize) -> FormationEvaluation {
    let bound = binding(e, slot);
    let i = inputs();
    let record = synthetic::record(bound.key().clone(), Some(&bound), synthetic::verified(1, 1), DIGEST, 1.0, Some(&i));
    FormationEvaluation::synthetic(Some(bound), None, Some(i), Some(record), VerificationProgress::VerifierReached)
}

#[test]
fn legal_plans_are_b_alone_or_a_influences_b() {
    assert!(plan_is_legal(&[b()]));
    assert!(plan_is_legal(&[a(), influence("S1"), b()]));
    assert!(plan_is_legal(&[a(), influence("S1"), influence("S2"), b()]));
    assert!(!plan_is_legal(&[]));
    assert!(!plan_is_legal(&[a(), b()]), "A without an influence");
    assert!(!plan_is_legal(&[influence("S1"), b()]), "an influence without A");
    assert!(!plan_is_legal(&[a(), influence("S1"), influence("S1"), b()]), "duplicate influence id");
    assert!(!plan_is_legal(&[b(), b()]));
    assert!(!plan_is_legal(&[a(), influence("S1")]), "no B");
}

#[test]
fn the_slots_own_verified_evaluation_is_admitted_and_covers_the_plan() {
    let mut e = evidence(vec![a(), influence("S1"), b()]);
    for slot in 0..3 {
        let offer = own_verified(&e, slot);
        let parts = e.collect(slot, Ok(offer)).expect("own evaluation");
        assert!(parts.inputs.is_some(), "the caller keeps the inputs");
    }
    assert!(e.association_failures().is_empty());
    assert!(e.verifies_all_required_premises());
    assert!(e.binds_stage_b(&inputs()));
    let mut other = inputs();
    other.lambda_sigma = 0.25;
    assert!(!e.binds_stage_b(&other), "a swapped bound fails the Stage B fingerprint");
}

#[test]
fn a_foreign_record_free_evaluation_adopts_nothing() {
    let mut recipient = evidence(vec![b()]);
    let donor = evidence(vec![b()]);
    let offer = FormationEvaluation::synthetic(Some(binding(&donor, 0)), None, None, None, VerificationProgress::StoppedBeforeVerifier);
    assert!(matches!(recipient.collect(0, Ok(offer)), Err(CollectRefusal::Foreign)));
    assert_eq!(recipient.stages()[0].progress(), &StageProgress::NotCalled, "no foreign progress");
    assert!(recipient.stages()[0].record().is_none());
    assert_eq!(recipient.association_failures(), &[AssociationFailure { slot: Some(0), kind: AssociationKind::ForeignEvaluation }]);
}

#[test]
fn a_foreign_record_bearing_evaluation_adopts_nothing() {
    let mut recipient = evidence(vec![b()]);
    let donor = evidence(vec![b()]);
    let offer = own_verified(&donor, 0);
    assert!(matches!(recipient.collect(0, Ok(offer)), Err(CollectRefusal::Foreign)));
    assert_eq!(recipient.stages()[0].progress(), &StageProgress::NotCalled);
    assert!(recipient.stages()[0].record().is_none());
    assert!(!recipient.verifies_all_required_premises(), "never verified");
}

#[test]
fn an_unbound_evaluation_is_foreign() {
    let mut recipient = evidence(vec![b()]);
    let offer = FormationEvaluation::synthetic(None, None, Some(inputs()), None, VerificationProgress::VerifierNotRun);
    assert!(matches!(recipient.collect(0, Ok(offer)), Err(CollectRefusal::Foreign)));
    assert_eq!(recipient.stages()[0].progress(), &StageProgress::NotCalled);
}

#[test]
fn a_record_only_donor_gives_the_slots_own_progress_and_no_record() {
    let mut recipient = evidence(vec![b()]);
    let donor = evidence(vec![b()]);
    let i = inputs();
    let donor_binding = binding(&donor, 0);
    let donor_record = synthetic::record(b(), Some(&donor_binding), synthetic::verified(1, 1), DIGEST, 1.0, Some(&i));
    let offer = FormationEvaluation::synthetic(Some(binding(&recipient, 0)), None, Some(i), Some(donor_record), VerificationProgress::VerifierReached);
    assert!(recipient.collect(0, Ok(offer)).is_ok(), "the evaluation is the slot's own");
    assert_eq!(recipient.stages()[0].progress(), &StageProgress::Evaluated(VerificationProgress::VerifierReached));
    assert!(recipient.stages()[0].record().is_none(), "the donor record is refused");
    assert_eq!(recipient.association_failures(), &[AssociationFailure { slot: Some(0), kind: AssociationKind::ForeignRecord }]);
    assert!(!recipient.verifies_all_required_premises());
}

#[test]
fn an_unbound_donor_record_is_refused() {
    let mut recipient = evidence(vec![b()]);
    let i = inputs();
    let unbound = synthetic::record(b(), None, synthetic::verified(1, 1), DIGEST, 1.0, Some(&i));
    let offer = FormationEvaluation::synthetic(Some(binding(&recipient, 0)), None, Some(i), Some(unbound), VerificationProgress::VerifierReached);
    assert!(recipient.collect(0, Ok(offer)).is_ok());
    assert!(recipient.stages()[0].record().is_none());
    assert_eq!(recipient.association_failures()[0].kind, AssociationKind::ForeignRecord);
}

#[test]
fn a_record_whose_fingerprint_does_not_match_its_inputs_is_refused() {
    let mut recipient = evidence(vec![b()]);
    let bound = binding(&recipient, 0);
    let minted = inputs();
    let record = synthetic::record(b(), Some(&bound), synthetic::verified(1, 1), DIGEST, 1.0, Some(&minted));
    let mut live = inputs();
    live.w[1] = 2.0;
    let offer = FormationEvaluation::synthetic(Some(bound), None, Some(live), Some(record), VerificationProgress::VerifierReached);
    assert!(recipient.collect(0, Ok(offer)).is_ok());
    assert!(recipient.stages()[0].record().is_none());
    assert_eq!(recipient.association_failures()[0].kind, AssociationKind::FingerprintMismatch);
}

#[test]
fn a_duplicate_offer_leaves_the_first_admitted_record() {
    let mut e = evidence(vec![b()]);
    let first = own_verified(&e, 0);
    e.collect(0, Ok(first)).unwrap();
    let second = own_verified(&e, 0);
    assert!(matches!(e.collect(0, Ok(second)), Err(CollectRefusal::Duplicate)));
    assert!(e.stages()[0].record().is_some_and(|r| r.is_verified()), "the first record stands");
    assert_eq!(e.association_failures(), &[AssociationFailure { slot: Some(0), kind: AssociationKind::DuplicateOffer }]);
    assert!(!e.verifies_all_required_premises(), "an association failure never verifies");
}

#[test]
fn an_unknown_slot_touches_no_stage() {
    let mut e = evidence(vec![b()]);
    let offer = own_verified(&e, 0);
    assert!(matches!(e.collect(5, Ok(offer)), Err(CollectRefusal::UnknownSlot)));
    assert_eq!(e.stages()[0].progress(), &StageProgress::NotCalled);
    assert_eq!(e.association_failures(), &[AssociationFailure { slot: None, kind: AssociationKind::UnknownSlot }]);
    let mut none = StageEvidence::not_determined();
    let offer = FormationEvaluation::synthetic(None, None, None, None, VerificationProgress::StoppedBeforeVerifier);
    assert!(matches!(none.collect(0, Ok(offer)), Err(CollectRefusal::UnknownSlot)));
    assert!(none.stages().is_empty());
    assert!(matches!(e.evaluate_slot(None, |_| unreachable!("no call for an unplanned stage")), Err(CollectRefusal::UnknownSlot)));
}

#[test]
fn a_call_error_is_the_slots_own_and_is_handed_back() {
    let mut e = evidence(vec![b()]);
    let refusal = e.evaluate_slot(Some(0), |_| Err(StructuralError::InvalidInput("confirmation system")));
    assert!(matches!(refusal, Err(CollectRefusal::CallError(StructuralError::InvalidInput("confirmation system")))));
    assert!(matches!(e.stages()[0].progress(), StageProgress::CallError(_)));
    assert!(e.association_failures().is_empty(), "an error is not an association failure");
}

#[test]
fn evaluate_slot_passes_the_slots_own_binding() {
    let mut e = evidence(vec![a(), influence("S1"), b()]);
    let expected = binding(&e, 1);
    let parts = e
        .evaluate_slot(e.slot_of(BoundStage::ConfirmationInfluence, Some("S1")), |given| {
            assert!(given.same_as(&expected));
            Ok(FormationEvaluation::synthetic(Some(given.clone()), None, Some(inputs()), None, VerificationProgress::VerifierNotRun))
        })
        .unwrap();
    assert_eq!(parts.progress, VerificationProgress::VerifierNotRun);
    assert_eq!(e.stages()[1].progress(), &StageProgress::Evaluated(VerificationProgress::VerifierNotRun));
}

#[test]
fn coverage_needs_every_slot_verified_with_the_attempts_matrix() {
    // A missing record: not covered.
    let mut e = evidence(vec![a(), influence("S1"), b()]);
    for slot in [0, 2] {
        let offer = own_verified(&e, slot);
        e.collect(slot, Ok(offer)).unwrap();
    }
    assert!(!e.verifies_all_required_premises());
    // An admitted NotVerified record: not covered, and still kept.
    let mut e = evidence(vec![b()]);
    let bound = binding(&e, 0);
    let record = synthetic::record(b(), Some(&bound), synthetic::not_verified(2, NotVerifiedReason::LemmaENonpositivePivot), DIGEST, 1.0, None);
    let offer = FormationEvaluation::synthetic(Some(bound), None, None, Some(record), VerificationProgress::VerifierReached);
    e.collect(0, Ok(offer)).unwrap();
    assert!(e.stages()[0].record().is_some_and(|r| !r.is_verified()), "NotVerified is retained");
    assert!(!e.verifies_all_required_premises());
    // Another matrix: not covered.
    let mut e = StageEvidence::planned(vec![b()]);
    e.set_matrix_digest(DIGEST + 1);
    let offer = own_verified(&e, 0);
    e.collect(0, Ok(offer)).unwrap();
    assert!(!e.verifies_all_required_premises());
    // NotDetermined discharges nothing.
    assert!(!StageEvidence::not_determined().verifies_all_required_premises());
}

#[test]
fn publication_use_never_replaces_stage_progress() {
    let mut e = evidence(vec![b()]);
    let offer = own_verified(&e, 0);
    e.collect(0, Ok(offer)).unwrap();
    assert_eq!(e.publication(), &PublicationUse::NotConsulted);
    e.set_publication(PublicationUse::Discarded { reason: DiscardReason::LoadRowFinding });
    assert_eq!(e.stages()[0].progress(), &StageProgress::Evaluated(VerificationProgress::VerifierReached));
    assert!(e.stages()[0].record().is_some());
    assert!(e.verifies_all_required_premises(), "coverage is independent of publication use");
}
