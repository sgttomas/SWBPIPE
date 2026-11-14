//! Carrier revision 4 §1: records, bindings and the v1 digest streams.

use super::*;

fn key(stage: BoundStage) -> StageKey {
    StageKey::new(stage, None)
}

fn inputs() -> MemberBoundInputs {
    MemberBoundInputs {
        w: vec![1.0, -0.0, 2.5],
        w2: vec![0.0, 3.0, -1.0],
        exponents: vec![0, -3, i32::MIN],
        lambda_sigma: 0.125,
        lift_radius: Vec::new(),
    }
}

#[test]
fn a_binding_is_the_same_only_as_itself_and_its_clones() {
    let a = EvaluationBinding::for_stage(key(BoundStage::ConfirmationStageB));
    let b = EvaluationBinding::for_stage(key(BoundStage::ConfirmationStageB));
    assert!(a.same_as(&a.clone()));
    assert!(!a.same_as(&b), "identical keys, distinct identities");
    assert_eq!(a.key(), b.key());
}

#[test]
fn record_equality_excludes_the_binding_and_belongs_to_does_not() {
    let a = EvaluationBinding::for_stage(key(BoundStage::ConfirmationStageB));
    let b = EvaluationBinding::for_stage(key(BoundStage::ConfirmationStageB));
    let i = inputs();
    let ra = synthetic::record(key(BoundStage::ConfirmationStageB), Some(&a), synthetic::verified(7, 2), 11, 3.0, Some(&i));
    let rb = synthetic::record(key(BoundStage::ConfirmationStageB), Some(&b), synthetic::verified(7, 2), 11, 3.0, Some(&i));
    assert_eq!(ra, rb, "deterministic fields equal");
    assert!(ra.belongs_to(&a) && !ra.belongs_to(&b));
    assert!(ra.clone().belongs_to(&a), "a clone keeps its binding");
    let unbound = synthetic::record(key(BoundStage::ConfirmationStageB), None, synthetic::verified(7, 2), 11, 3.0, Some(&i));
    assert!(!unbound.belongs_to(&a), "an unbound record belongs to no binding");
}

#[test]
fn the_fingerprint_matches_only_its_own_inputs_and_not_verified_has_none() {
    let i = inputs();
    let verified = synthetic::record(key(BoundStage::FormationCheck), None, synthetic::verified(1, 1), 0, 1.0, Some(&i));
    assert!(verified.is_verified());
    assert!(verified.fingerprint_matches(Some(&i)));
    assert!(!verified.fingerprint_matches(None));
    let mut other = i.clone();
    other.lambda_sigma = 0.25;
    assert!(!verified.fingerprint_matches(Some(&other)));
    let refused = synthetic::record(
        key(BoundStage::FormationCheck),
        None,
        synthetic::not_verified(1, NotVerifiedReason::LemmaDContractionNotBelowOne),
        0,
        1.0,
        Some(&i),
    );
    assert!(!refused.is_verified());
    assert_eq!(refused.inputs_digest(), None);
    assert!(refused.fingerprint_matches(None) && !refused.fingerprint_matches(Some(&i)));
}

#[test]
fn the_inputs_stream_distinguishes_every_field_and_signed_zero() {
    let base = inputs();
    let d = inputs_digest(&base);
    assert_eq!(d, inputs_digest(&base.clone()), "deterministic");
    let mut changes: Vec<MemberBoundInputs> = Vec::new();
    let mut m = base.clone();
    m.w[1] = 0.0; // -0.0 to +0.0
    changes.push(m);
    let mut m = base.clone();
    m.w2[0] = -0.0;
    changes.push(m);
    let mut m = base.clone();
    m.exponents[1] = -2;
    changes.push(m);
    let mut m = base.clone();
    m.lambda_sigma = -0.125;
    changes.push(m);
    let mut m = base.clone();
    m.lift_radius = vec![0.0];
    changes.push(m);
    // Moving a value between w and w2 changes the stream (field tags and counts).
    let mut m = base.clone();
    let moved = m.w.pop().unwrap();
    m.w2.insert(0, moved);
    changes.push(m);
    for (index, changed) in changes.iter().enumerate() {
        assert_ne!(inputs_digest(changed), d, "change {index}");
    }
}

#[test]
fn the_inputs_stream_is_the_specified_byte_sequence() {
    let i = MemberBoundInputs { w: vec![1.0], w2: Vec::new(), exponents: vec![-1], lambda_sigma: 2.0, lift_radius: Vec::new() };
    let mut bytes: Vec<u8> = b"SWB-MBI-v1".to_vec();
    bytes.push(0x01);
    bytes.extend(1u64.to_le_bytes());
    bytes.extend(1.0f64.to_bits().to_le_bytes());
    bytes.push(0x02);
    bytes.extend(0u64.to_le_bytes());
    bytes.push(0x03);
    bytes.extend(1u64.to_le_bytes());
    bytes.extend((-1i32).to_le_bytes());
    bytes.push(0x04);
    bytes.extend(2.0f64.to_bits().to_le_bytes());
    bytes.push(0x05);
    bytes.extend(0u64.to_le_bytes());
    let mut h = Fnv64::new();
    h.bytes(&bytes);
    assert_eq!(inputs_digest(&i), h.finish());
}

#[test]
fn fnv_1a_64_matches_its_published_vectors() {
    let digest = |s: &[u8]| {
        let mut h = Fnv64::new();
        h.bytes(s);
        h.finish()
    };
    assert_eq!(digest(b""), 0xcbf2_9ce4_8422_2325);
    assert_eq!(digest(b"a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(digest(b"foobar"), 0x8594_4171_f739_67e8);
}

#[test]
fn the_matrix_stream_fills_implicit_zeros_and_ignores_storage() {
    // A 3x3 logical lower triangle [[4],[0,5],[1,0,6]] with exponents [0,1,-1].
    let exponents = [0, 1, -1];
    let dense = [[4.0, 0.0, 1.0], [0.0, 5.0, 0.0], [1.0, 0.0, 6.0]];
    let full = matrix_digest_rows(&exponents, |r, emit| {
        for c in 0..=r {
            emit(c, dense[r][c]);
        }
    });
    let stored: [&[(usize, f64)]; 3] = [&[(0, 4.0)], &[(1, 5.0)], &[(0, 1.0), (2, 6.0)]];
    let sparse = matrix_digest_rows(&exponents, |r, emit| {
        for &(c, v) in stored[r] {
            emit(c, v);
        }
    });
    assert_eq!(full, sparse, "implicit zeros are written as +0.0");
    let changed_entry = matrix_digest_rows(&exponents, |r, emit| {
        for &(c, v) in stored[r] {
            emit(c, if (r, c) == (2, 0) { 2.0 } else { v });
        }
    });
    assert_ne!(full, changed_entry);
    let changed_exponent = matrix_digest_rows(&[0, 1, 0], |r, emit| {
        for &(c, v) in stored[r] {
            emit(c, v);
        }
    });
    assert_ne!(full, changed_exponent);
}
