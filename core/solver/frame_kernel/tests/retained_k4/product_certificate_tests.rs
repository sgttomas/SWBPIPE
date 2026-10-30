//! Frozen independent Fraction targets and actual producing work custody.
use super::*;
#[path = "product_certificate_vectors.rs"]
mod vectors;

fn parse(token: &str) -> Endpoint {
    if token == "Z+" {
        return Endpoint::ZERO;
    }
    let (hex, e) = token[1..].split_once('p').unwrap();
    let digits = format!("{hex:0<256}");
    let mut limbs = [0; 16];
    for (i, v) in limbs.iter_mut().rev().enumerate() {
        *v = u64::from_str_radix(&digits[i * 16..i * 16 + 16], 16).unwrap();
    }
    Endpoint::from_parts(token.starts_with('-'), e.parse().unwrap(), limbs).unwrap()
}
fn input(mode: u8, geometry: [u64; 2], m: [u64; 9], k: [u64; 7]) -> MemberOperands {
    let m = m.map(f64::from_bits);
    let k = k.map(f64::from_bits);
    let material = match mode {
        0 => MaterialOperands::ExactENu { e: m[0], nu: m[1] },
        1 => MaterialOperands::Ordinary { e: m[0], g: m[1] },
        2 => MaterialOperands::Interpolated {
            t_lo: m[0],
            t: m[1],
            t_hi: m[2],
            e_lo: m[3],
            e_hi: m[4],
            g_lo: m[5],
            g_hi: m[6],
            e_hat: m[7],
            g_hat: m[8],
        },
        _ => panic!("fixture mode"),
    };
    MemberOperands {
        diameter: f64::from_bits(geometry[0]),
        effective_wall: f64::from_bits(geometry[1]),
        material,
        admitted: AdmittedOperands {
            e: k[0],
            g: k[1],
            a: k[2],
            j: k[3],
            iz: k[4],
            iy: k[5],
            z_hat: k[6],
        },
    }
}
fn ordinary() -> MemberOperands {
    let (_, mode, geometry, m, k, _, _) = vectors::CASES[1];
    input(mode, geometry, m, k)
}
fn covers(actual: Enclosure, reference: (&str, &str), label: &str) {
    assert!(
        actual.lo.cmp_value(&parse(reference.0)) != Ordering::Greater,
        "lower {label}"
    );
    assert!(
        actual.hi.cmp_value(&parse(reference.1)) != Ordering::Less,
        "upper {label}"
    );
}
#[test]
fn product_certificate_fraction_geometry_material_and_coefficients() {
    assert_eq!(vectors::CASES.len(), 8);
    for &(name, mode, geometry, m, k, reference, products) in vectors::CASES {
        let input = input(mode, geometry, m, k);
        let spent = member_coefficients(&input);
        assert!(spent.work().status().is_exact(), "{name}: {spent:?}");
        let v = spent.result().unwrap();
        assert_eq!(v.effective_wall_bits, geometry[1]);
        let intervals = [
            Enclosure::point(v.c),
            v.ri,
            v.p,
            v.q,
            v.geometry_g,
            v.a,
            v.i,
            v.j,
            v.z,
            v.e,
            v.g,
            v.coefficients[0],
            v.coefficients[1],
            v.coefficients[2],
            v.coefficients[3],
            v.represented_z.unwrap_or(Enclosure::point(Endpoint::ZERO)),
        ];
        for (a, r) in intervals.into_iter().zip(reference.iter().copied()) {
            covers(a, r, name);
        }
        for j in 0..4 {
            assert_eq!(
                v.admitted_products[j],
                parse(products[j]),
                "exact K {name} {j}"
            );
            assert!(
                v.coefficient_differences[j].cmp_value(&parse(products[j + 4])) != Ordering::Less,
                "delta {name} {j}"
            );
        }
        let entries = spent.work.entries.map(|n| n.exact().unwrap());
        assert_eq!(entries[Entry::R4 as usize], if mode == 2 { 4 } else { 0 });
        assert_eq!(entries[Entry::B64U as usize], 0);
        assert_eq!(spent.work.f64_arithmetic.exact(), Ok(0));
        // An entered DM with an exact zero operand has no TwoProduct core
        // call. Count the actual prefix, not a one-core-call-per-entry envelope.
        let m = m.map(f64::from_bits);
        let zero_products = if mode == 2 {
            [
                (m[2], m[3]),
                (m[1], m[3]),
                (m[1], m[4]),
                (m[0], m[4]),
                (m[2], m[5]),
                (m[1], m[5]),
                (m[1], m[6]),
                (m[0], m[6]),
            ]
            .iter()
            .filter(|(a, b)| *a == 0.0 || *b == 0.0)
            .count() as u64
        } else {
            0
        };
        assert_eq!(
            spent.work.wide.width::<16>().two_product,
            entries[Entry::Mul as usize] + entries[Entry::Div as usize] - zero_products,
            "actual TwoProduct calls {name}"
        );
        assert!(spent.work.wide.checked_lme().exact().unwrap() > 0);
        assert!(spent.work.sums.checked_lme().exact().unwrap() > 0);
    }
}
#[test]
fn product_certificate_exact_k_retains_missing_binary64_tail() {
    let x = f64::from_bits(1.0f64.to_bits() + 1);
    let mut work = NumericWork::new();
    let exact = work.exact_product(x, x).unwrap();
    let rounded = lift(x * x).unwrap();
    let delta = work.absdiff(&exact, &rounded).unwrap();
    assert_eq!(delta, shift(&Endpoint::ONE, -104).unwrap());
    assert_ne!(exact, rounded);
}
#[test]
fn product_certificate_exact_zero_and_positive_subnormal_source() {
    let mut work = NumericWork::new();
    assert_eq!(
        work.scalar(Entry::Sub, &Endpoint::ONE, &Endpoint::ONE, Toward::Up)
            .unwrap(),
        Endpoint::ZERO
    );
    assert_eq!(lift(-0.0).unwrap().parts().0, false);
    let (_, mode, g, m, k, _, _) = vectors::CASES[4];
    let spent = member_coefficients(&input(mode, g, m, k));
    let v = spent.result().unwrap();
    assert!(positive(&v.i.lo));
    assert_eq!(v.i.lo.to_binary64().value().unwrap_or(0.0), 0.0);
    // Positive wide source geometry survives even where a binary64 property
    // would be zero. This abstract input does not prove product reachability.
    assert!(v.i.lo.exponent() < -1074);
}
#[test]
fn product_certificate_source_sign_precedes_positive_rounded_material() {
    let mut x = ordinary();
    x.material = MaterialOperands::Interpolated {
        t_lo: 0.,
        t: 7.,
        t_hi: 25.,
        e_lo: -7.,
        e_hi: 18.,
        g_lo: 1.,
        g_hi: 1.,
        e_hat: 2f64.powi(-50),
        g_hat: 1.,
    };
    x.admitted.e = 2f64.powi(-50);
    x.admitted.g = 1.;
    let spent = member_coefficients(&x);
    assert!(matches!(
        spent.result(),
        Err(NumericError::NonpositiveSource)
    ));
    assert!(spent.work.status().is_exact());
    assert_eq!(spent.work.entries[Entry::R4 as usize].exact(), Ok(1));
    assert_eq!(spent.work.entries[Entry::Mul as usize].exact(), Ok(4));
    assert!(spent.work.sums.checked_lme().exact().unwrap() > 0);
}
#[test]
fn product_certificate_refusals_and_ordinary_independent_g() {
    let mut x = ordinary();
    x.material = MaterialOperands::Ordinary { e: 2., g: 7. };
    x.admitted.e = 2.;
    x.admitted.g = 7.;
    let spent = member_coefficients(&x);
    assert_eq!(spent.result().unwrap().g.lo, lift(7.).unwrap());
    x.admitted.g = 6.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::MaterialBits)
    ));
    let mut x = ordinary();
    x.effective_wall = x.diameter / 2.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::InvalidGeometry)
    ));
    let mut x = ordinary();
    x.admitted.iy = 2.;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::AxisBits)
    ));
    let mut x = ordinary();
    x.diameter = f64::NAN;
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::NonFinite)
    ));
    let mut x = ordinary();
    x.material = MaterialOperands::ExactENu {
        e: x.admitted.e,
        nu: -1.,
    };
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::InvalidMaterial)
    ));
    let mut x = ordinary();
    x.material = MaterialOperands::Interpolated {
        t_lo: 0.,
        t: 0.,
        t_hi: 1.,
        e_lo: 1.,
        e_hi: 1.,
        g_lo: 1.,
        g_hi: 1.,
        e_hat: 1.,
        g_hat: 1.,
    };
    assert!(matches!(
        member_coefficients(&x).result(),
        Err(NumericError::TemperatureOrder)
    ));
    let mut work = NumericWork::new();
    assert_eq!(
        work.scalar(Entry::Div, &Endpoint::ONE, &Endpoint::ZERO, Toward::Up),
        Err(NumericError::NonpositiveDenominator)
    );
    assert_eq!(work.entries[Entry::Div as usize].exact(), Ok(1));
    assert_eq!(work.wide.checked_lme().exact(), Ok(0));
}
#[test]
fn product_certificate_scalar_and_r4_failure_prefixes() {
    let mut work = NumericWork::new();
    let huge = shift(&Endpoint::ONE, 9000).unwrap();
    assert!(work
        .scalar(Entry::Add, &huge, &Endpoint::ONE, Toward::Up)
        .is_err());
    assert_eq!(work.entries[Entry::Add as usize].exact(), Ok(1));
    assert!(work.sums.checked_lme().exact().unwrap() > 0);
    let mut sum = ExactWideSum::new();
    sum.add_binary64(1., false).unwrap();
    sum.signum().unwrap();
    sum.test_seed_term_work(u64::MAX);
    let mut work = NumericWork::new();
    let result = scalar_owned(
        &mut work,
        Entry::Div,
        &Endpoint::ONE,
        &Endpoint::ZERO,
        Toward::Up,
        WideContext::<16>::new(1024).unwrap(),
        sum,
    );
    assert_eq!(result, Err(NumericError::NonpositiveDenominator));
    // Numeric refusal and failed aggregate work extraction coexist.
    assert!(!work.status().is_exact());
    assert_eq!(
        work.checked(result),
        Err(NumericError::NonpositiveDenominator)
    );
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX - 2);
    let mut work = NumericWork::new();
    let p = [
        lift(3.).unwrap(),
        Endpoint::ONE,
        lift(4.).unwrap(),
        Endpoint::ONE,
    ];
    assert!(r4_owned(
        &mut work,
        &p,
        Toward::Down,
        WideContext::<16>::new(1024).unwrap(),
        sum
    )
    .is_err());
    assert!(!work.status().is_exact());
    assert!(work.sums.term_limbs >= u64::MAX - 2);
}
#[test]
fn product_certificate_checked_merge_blocks_success_and_keeps_numeric_cause() {
    let mut work = NumericWork::new();
    let mut seeded = ExactWideSum::new();
    seeded.test_seed_term_work(u64::MAX);
    work.sums = seeded.work();
    let result = scalar_owned(
        &mut work,
        Entry::Add,
        &Endpoint::ONE,
        &Endpoint::ONE,
        Toward::Up,
        WideContext::<16>::new(1024).unwrap(),
        ExactWideSum::new(),
    );
    assert!(matches!(
        result,
        Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(!work.status().is_exact());
    let cause = Err::<(), _>(NumericError::InvalidGeometry);
    assert_eq!(work.checked(cause.clone()), cause);
}

#[test]
fn product_certificate_b64u_zero_normal_successor_underflow_and_overflow() {
    let one = Endpoint::ONE;
    let mut work = NumericWork::new();
    let half_ulp = shift(&one, -54).unwrap();
    let successor_input = work
        .scalar(Entry::Add, &one, &half_ulp, Toward::Up)
        .unwrap();
    let max = lift(f64::MAX).unwrap();
    let small = shift(&one, 969).unwrap();
    let max_successor = work.scalar(Entry::Add, &max, &small, Toward::Up).unwrap();
    for (x, expected, early) in [
        (Endpoint::ZERO, 0., false),
        (one, 1., false),
        (successor_input, f64::from_bits(1.0f64.to_bits() + 1), false),
        (shift(&one, -1075).unwrap(), f64::from_bits(1), false),
        (shift(&one, -1074).unwrap(), f64::from_bits(1), false),
        (max_successor, f64::INFINITY, false),
        (shift(&one, 1024).unwrap(), f64::INFINITY, true),
    ] {
        let (result, sums) = directed::binary64_up_spent(&x).into_parts();
        assert_eq!(result.as_ref().unwrap().to_bits(), expected.to_bits());
        assert_eq!(
            directed::binary64_up(&x).unwrap().to_bits(),
            expected.to_bits()
        );
        assert!(sums.checked_lme().status().is_exact());
        if early || x.is_zero() {
            assert_eq!(sums.checked_lme().exact(), Ok(0));
        } else {
            assert!(sums.checked_lme().exact().unwrap() > 0);
        }
        let mut collector = NumericWork::new();
        let collected = collector.binary64_up(&x);
        assert_eq!(collector.entries[Entry::B64U as usize].exact(), Ok(1));
        assert_eq!(collector.wide.checked_lme().exact(), Ok(0));
        assert_eq!(collector.sums, sums);
        if expected.is_infinite() {
            assert_eq!(collected, Err(NumericError::Binary64Range));
        } else {
            assert_eq!(collected.unwrap().to_bits(), expected.to_bits());
        }
    }
}
#[test]
fn product_certificate_b64u_each_refusal_prefix_and_collection_status() {
    let x = lift(1.).unwrap();
    let mut first = ExactWideSum::new();
    first.add_wide(&x, false).unwrap();
    let first_work = first.work();
    first.add_binary64(1., true).unwrap();
    let two_work = first.work();
    let prefixes = [
        (0, 0),
        (first_work.checked_lme().exact().unwrap(), 1),
        (two_work.checked_lme().exact().unwrap(), 2),
    ];
    for (budget, stage) in prefixes {
        let mut sum = ExactWideSum::new();
        sum.test_seed_term_work(u64::MAX - budget);
        let (result, sums) = directed::test_binary64_up_owned(&x, 1., sum).into_parts();
        assert!(result.is_err(), "stage {stage}");
        assert!(!sums.checked_lme().status().is_exact());
        assert!(sums.term_limbs >= u64::MAX - budget);
        if stage == 0 {
            assert_eq!(sums.term_limbs, u64::MAX);
        }
        if stage == 1 {
            assert_eq!(sums.term_limbs, u64::MAX - budget + first_work.term_limbs);
        }
        if stage == 2 {
            assert_eq!(sums.term_limbs, u64::MAX - budget + two_work.term_limbs);
        }
        let mut collector = NumericWork::new();
        let saved = result.clone();
        let collected = collector.collect_binary64(result, sums);
        assert_eq!(collected, saved.map_err(NumericError::Arithmetic));
        assert!(!collector.status().is_exact());
    }
    let mut collector = NumericWork::new();
    let mut sum = ExactWideSum::new();
    sum.test_seed_term_work(u64::MAX);
    collector.sums = sum.work();
    let (result, sums) = directed::binary64_up_spent(&x).into_parts();
    assert!(matches!(
        collector.collect_binary64(result, sums),
        Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(_)))
    ));
    assert!(!collector.status().is_exact());
    assert_eq!(
        collector.collect_binary64(Err(AttemptStop::Exponent), SumWork::default()),
        Err(NumericError::Arithmetic(AttemptStop::Exponent))
    );
}


#[test]
fn i51_c0_layout_and_accounting_only() {
    // No source, model, factor, native solve or product invocation is created.
    use std::mem::{size_of,align_of};
    macro_rules! layout { ($name:expr,$t:ty) => {
        println!("I51_C0_LAYOUT {} size={} align={}",$name,size_of::<$t>(),align_of::<$t>());
    }; }
    layout!("SectionPrepFrame",SectionPrepFrame);
    layout!("SectionPreparationWork",SectionPreparationWork);
    layout!("NumericWork",NumericWork);
    layout!("WideContext16",WideContext<16>);
    layout!("ExactWideSum",ExactWideSum);
    layout!("Endpoint",Endpoint);
    layout!("Enclosure",Enclosure);
    layout!("ScalarResult",Result<Endpoint,NumericError>);
    layout!("DirectedResult",Result<Endpoint,super::super::adaptive::AttemptStop>);
    layout!("WideResult",Result<Endpoint,super::super::wide::WideError>);
    layout!("ProductPair",(Endpoint,Endpoint));
    layout!("WidePairResult",Result<(Endpoint,Endpoint),super::super::wide::WideError>);
    layout!("RoundDetailResult",Result<(Endpoint,usize,bool),super::super::wide::WideError>);
    layout!("Magnitude",[u64;128]);
    layout!("TrimmedTerm",[u64;130]);
    layout!("ScaledTerm",[u64;17]);
    layout!("WideDouble",[u64;32]);
    layout!("Significand",[u64;16]);
    layout!("Binary64Outcome",super::super::wide::multi::Binary64Outcome);
    layout!("AnnulusResult",Result<PreparedAnnulus,SectionPreparationError>);
    layout!("AnnulusSpent",AnnulusPreparationSpent);
    assert_eq!(size_of::<SectionPrepFrame>(),27*size_of::<Endpoint>());
    assert_eq!(size_of::<Enclosure>(),2*size_of::<Endpoint>());
    let mut w=SectionPreparationWork::new();
    w.conversions=WorkTotal::exact_count(u64::MAX);
    assert_eq!(w.round(&Endpoint::ONE,0,PreparationEndpoint::Lo),Err(SectionPreparationError::Accounting));
    assert!(!w.status().is_exact());
    assert_eq!(w.initialized_endpoints.exact(),Ok(0));
    let old=w.conversions;
    assert_eq!(w.round(&Endpoint::ONE,0,PreparationEndpoint::Lo),Err(SectionPreparationError::Accounting));
    assert_eq!(w.conversions,old,"prior fault prevents a second conversion entry");
    println!("I51_C0_ACCOUNTING overflow and prior-fault prefix verified; no numerical producer run");
}

#[test]
fn prepared_trace_actual_conversion_success_and_refusal_prefixes() {
    let spent=prepare_product_annulus(0.1,0.005);
    let section=spent.result().unwrap().section_bits().bits();
    let outcomes=spent.work().conversion_outcomes();assert_eq!(outcomes.len(),9);
    for (i,entry) in outcomes.iter().enumerate() {
        let e=entry.as_ref().unwrap();let property=if i==8{4}else{i/2};
        assert_eq!(e.property,property);
        assert_eq!(e.endpoint,if i==8{PreparationEndpoint::Exact}else if i%2==0{PreparationEndpoint::Lo}else{PreparationEndpoint::Hi});
        assert!(matches!(e.outcome,super::super::wide::multi::Binary64Outcome::Normal(v) if v.to_bits()==section[property]));
    }
    let invalid=prepare_product_annulus(0.0,0.005);
    assert!(matches!(invalid.result(),Err(SectionPreparationError::InvalidGeometry)));
    assert!(invalid.work().conversion_outcomes().is_empty());
    for (d,t,overflow) in [(1e200,1e190,true),(1e-200,1e-210,false)] {
        let spent=prepare_product_annulus(d,t);assert!(spent.result().is_err());
        assert_eq!(spent.work().conversion_outcomes().len(),1);
        let outcome=spent.work().conversion_outcomes()[0].unwrap().outcome;
        assert!(if overflow{matches!(outcome,super::super::wide::multi::Binary64Outcome::Overflow{..})}
            else{matches!(outcome,super::super::wide::multi::Binary64Outcome::Underflow{..})});
        assert_eq!(spent.work().conversions.exact(),Ok(1));
    }
    let mut reads=TraceCopyWork::default();let trace=spent.work().numeric_trace(&mut reads);
    assert_eq!(trace.entries,spent.work().numeric.entries);
    assert_eq!(trace.wide_lme,spent.work().numeric.wide.checked_lme());
    assert!(reads.events.exact().unwrap()>0 && reads.bytes.exact().unwrap()>0);
    println!("I51_TRACE_LAYOUT SectionPreparationWork={} PreparationConversion={} TraceCopyWork={} NumericTrace={}",
        std::mem::size_of::<SectionPreparationWork>(),std::mem::size_of::<PreparationConversion>(),std::mem::size_of::<TraceCopyWork>(),std::mem::size_of::<NumericTrace>());
}

#[test]
fn prepared_trace_full_conversion_storage_refuses_before_entered_counter() {
    let mut work=SectionPreparationWork::new();
    for _ in 0..9 {assert_eq!(work.round(&Endpoint::ONE,0,PreparationEndpoint::Lo),Ok(1.0));}
    let count=work.conversions;let prefix=work.conversion_outcomes;
    assert_eq!(work.round(&Endpoint::ONE,0,PreparationEndpoint::Lo),Err(SectionPreparationError::Accounting));
    assert_eq!(work.conversions,count);assert_eq!(work.conversion_outcomes,prefix);
    assert_eq!(work.conversions.exact(),Ok(9));assert_eq!(work.conversion_outcomes().len(),9);
}

// ---------------------------------------------------------------- B3-K (K3-3)
// K3-2 computes represented Z for every material. These tests add SA-2's
// controls and NA-6's own nu = 0.3125 control; the frozen oracle test above is
// unchanged (NA-6) and covers the two ExactENu vectors' new hulls.
fn b3k_div_entries(spent: &MemberSpent) -> u64 {
    spent.work.entries[Entry::Div as usize].exact().unwrap()
}
#[test]
fn b3k_exact_e_nu_represented_z_axis_bits_and_restored_material_gate() {
    let exact: Vec<_> = vectors::CASES.iter().filter(|c| c.1 == 0).collect();
    assert_eq!(exact.len(), 2);
    for &&(name, mode, geometry, m, k, reference, _) in &exact {
        let x = input(mode, geometry, m, k);
        let spent = member_coefficients(&x);
        let v = spent.result().unwrap();
        let z = v.represented_z.expect("K3-2: ExactENu has represented Z");
        covers(z, reference[15], name);
        assert!(positive(&z.lo), "{name}: a positive hull, not the old placeholder");
        assert_ne!(reference[15], ("Z+", "Z+"), "{name}: the declared oracle lines");
        // The restored pre-K3-2 gate (test-only): no represented Z, and exactly
        // the represented-Z division pair (lo, hi) fewer; everything before it unchanged.
        hooks::set_material_gate(true);
        let gated = member_coefficients(&x);
        let mut skew = x;
        skew.admitted.iy = f64::from_bits(skew.admitted.iz.to_bits() + 1);
        let gated_skew = member_coefficients(&skew);
        hooks::set_material_gate(false);
        assert!(gated.result().unwrap().represented_z.is_none(), "{name}");
        assert_eq!(b3k_div_entries(&gated) + 2, b3k_div_entries(&spent), "{name}");
        // Under the old gate ExactENu never checked the axis bits.
        assert!(gated_skew.result().unwrap().represented_z.is_none(), "{name}");
        // K3-2 extends the Iy = Iz refusal to ExactENu, keeping its work prefix:
        // every entry before the represented-Z division, and nothing after.
        let refused = member_coefficients(&skew);
        assert_eq!(refused.result().unwrap_err(), NumericError::AxisBits, "{name}");
        assert!(refused.work.status().is_exact(), "{name}");
        for e in 0..7 {
            assert_eq!(refused.work.entries[e], gated_skew.work.entries[e], "{name} entry {e}");
        }
        assert_eq!(refused.work.wide.checked_lme(), gated_skew.work.wide.checked_lme(), "{name}");
        assert_eq!(refused.work.sums.checked_lme(), gated_skew.work.sums.checked_lme(), "{name}");
        assert!(refused.work.sums.checked_lme().exact().unwrap() > 0, "{name}");
    }
    // The gate is material-specific: the ordinary vectors keep represented Z.
    hooks::set_material_gate(true);
    let ordinary_gated = member_coefficients(&ordinary());
    hooks::set_material_gate(false);
    assert!(ordinary_gated.result().unwrap().represented_z.is_some());
}
fn b3k_specimen(material: MaterialOperands, g: f64) -> MemberOperands {
    // RV56's specimen: E = 210e9, D = 0.1, t = 0.005; the K section is the
    // correctly rounded prepared annulus.
    let (d, t) = (f64::from_bits(0x3fb999999999999a), f64::from_bits(0x3f747ae147ae147b));
    let [a, i, j, z, _] = prepare_product_annulus(d, t).result().unwrap().section_bits().values();
    MemberOperands {
        diameter: d,
        effective_wall: t,
        material,
        admitted: AdmittedOperands { e: 210e9, g, a, j, iz: i, iy: i, z_hat: z },
    }
}
/// sign(x * 2(1 + nu) - e), exactly (nu a binary64 in (-1, 1/2)).
fn b3k_sign_times_two_one_plus_nu(x: &Endpoint, nu: f64, e: f64) -> i8 {
    let mut sum = ExactWideSum::new();
    sum.add_wide_scaled(x, false, 2, 0).unwrap();
    if nu != 0.0 {
        let bits = nu.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i64;
        assert!(biased > 0, "normal nu");
        let m = (bits & ((1u64 << 52) - 1)) | (1u64 << 52);
        sum.add_wide_scaled(x, nu < 0.0, m, biased - 1075 + 1).unwrap();
    }
    sum.add_binary64(e, true).unwrap();
    sum.signum().unwrap()
}
#[test]
fn b3k_nu_0_3125_control_and_exact_g_against_represented_g() {
    // NA-6: K3-3's own nu = 0.3125 control (RV56's lived in its review fixture).
    // E/(2(1 + 0.3125)) = 210e9/2.625 = 80e9 exactly, so the exact route's
    // enclosures equal the ordinary route's with G = 80e9, bit for bit.
    let exact = member_coefficients(&b3k_specimen(MaterialOperands::ExactENu { e: 210e9, nu: 0.3125 }, 80e9));
    let ordinary = member_coefficients(&b3k_specimen(MaterialOperands::Ordinary { e: 210e9, g: 80e9 }, 80e9));
    let (x, o) = (exact.result().unwrap(), ordinary.result().unwrap());
    assert_eq!(x.g.lo, lift(80e9).unwrap());
    assert_eq!(x.g.hi, lift(80e9).unwrap());
    assert_eq!(b3k_sign_times_two_one_plus_nu(&x.g.lo, 0.3125, 210e9), 0);
    let pairs = |v: &MemberEnclosures| {
        let mut out = vec![v.ri, v.p, v.q, v.geometry_g, v.a, v.i, v.j, v.z, v.e, v.g];
        out.extend(v.coefficients);
        out.push(v.represented_z.unwrap());
        out.into_iter().map(|e| (e.lo, e.hi)).collect::<Vec<_>>()
    };
    assert_eq!(pairs(x), pairs(o));
    assert_eq!(x.admitted_products, o.admitted_products);
    assert_eq!(x.coefficient_differences, o.coefficient_differences);
    // The exact route's own G costs one add and one divide pair more.
    assert_eq!(exact.work.entries[Entry::Add as usize].exact().unwrap(), ordinary.work.entries[Entry::Add as usize].exact().unwrap() + 2);
    assert_eq!(b3k_div_entries(&exact), b3k_div_entries(&ordinary) + 2);
    // nu = 0.3: G = E/(2(1 + nu)) is not binary64. The source lane encloses the
    // exact G strictly and excludes the represented G-hat; G-hat*J enters as a
    // nonzero coefficient difference (DEF-E's E2), not as the G lane's law.
    let nu = 0.3;
    let g_hat = 210e9 / (2.0 * (1.0 + nu));
    let spent = member_coefficients(&b3k_specimen(MaterialOperands::ExactENu { e: 210e9, nu }, g_hat));
    let v = spent.result().unwrap();
    assert_eq!(b3k_sign_times_two_one_plus_nu(&v.g.lo, nu, 210e9), -1);
    assert_eq!(b3k_sign_times_two_one_plus_nu(&v.g.hi, nu, 210e9), 1);
    let represented = lift(g_hat).unwrap();
    assert!(represented.cmp_value(&v.g.lo) == Ordering::Less || represented.cmp_value(&v.g.hi) == Ordering::Greater);
    assert!(positive(&v.coefficient_differences[1]));
    assert_eq!(v.admitted_products[1], NumericWork::new().exact_product(g_hat, b3k_specimen(MaterialOperands::ExactENu { e: 210e9, nu }, g_hat).admitted.j).unwrap());
    assert!(v.represented_z.is_some());
}
