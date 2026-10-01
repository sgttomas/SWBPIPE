
// I23 last_gaps_04: two bounded scalar discriminator proposals only.
// Existing models, helpers and all preceding tests remain unchanged.

#[test]
fn i23_k4_m9_pivot_inequality_uses_requested_precision() {
    // Accepted exact rule: d * (2^p - m) - 64*m*c > 0.
    // d=2^-100, c=1, m=1:
    // p128 gives 2^28 - 2^-100 - 64 > 0;
    // the capped-p53 fault gives 2^-47 - 2^-100 - 64 < 0.
    // This tests the primitive inequality, not an assembled-model schedule.
    let pivot = Wide::<4>::ONE.mul_pow2(-100).unwrap();
    let scale = Wide::<4>::ONE;
    let mut sum = ExactWideSum::new();
    assert_eq!(
        pivot_passes(&mut sum, &pivot, &scale, 1, 128),
        Ok(true),
        "I23:K4M9_REQUESTED_P128"
    );
    assert_eq!(
        pivot_passes(&mut sum, &pivot, &scale, 1, 53),
        Ok(false),
        "I23:K4M9_P53_COMPANION"
    );
}

#[test]
fn i23_k4_m32_scalar_rcond_uses_equilibrated_norm() {
    // The existing SPRING-CARRIED fixture has only global DOF 9 free and a
    // stored diagonal. Reuse it unchanged solely as a sparse-pattern carrier
    // for constructed K=[16]; no physical model or solver output is an oracle.
    let source = models::model("SPRING-CARRIED").source();
    let (structure, ordering, k) = constructed::<4>(&source, |_, _| 16.0);
    assert_eq!(ordering.free, vec![9], "I23 M32 setup: scalar free DOF");
    let diagonal = structure.pattern.find(9, 9).expect("I23 M32 setup: diagonal");
    assert_eq!(k[diagonal], support::lift::<4>(16.0), "I23 M32 setup: K=16");
    let mut context = ctx::<4>(128);
    let mut sum = ExactWideSum::new();
    let f = factor(&mut context, &mut sum, &StageGuard::unlimited(),
        &structure, &k, &ordering).expect("I23 M32 setup: factor");
    // e(16)=4, s=-floor(4/2)=-2: S=1/4 and Ktilde=1 exactly.
    assert_eq!(f.scale(), &[-2i64], "I23 M32 setup: radix scale");
    assert_eq!(f.get(0, 0), Wide::<4>::ONE, "I23 M32 setup: scaled pivot");
    assert_eq!(f.solve_scaled(&mut context, &[Wide::<4>::ONE]),
        Ok(vec![Wide::<4>::ONE]), "I23 M32 setup: scaled inverse action");
    // Hager–Higham: x=y=z=[1], estimate=1, z*x=1 ends the first loop.
    // Its n=1 alternating safeguard is RN128(2/3)<1, so estimate stays 1.
    // Correct norm=1 gives rcond=1. Faulty norm(K)=16 gives rcond=1/16;
    // both products are strictly below the 2^127 condition-stop boundary.
    assert_eq!(
        f.condition(&mut context, &mut sum, &structure, &k, &ordering),
        Ok(Wide::<4>::ONE),
        "I23:K4M32_EQUILIBRATED_RCOND"
    );
}
