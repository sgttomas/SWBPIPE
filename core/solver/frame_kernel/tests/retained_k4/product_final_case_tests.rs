use super::*;
#[test]
fn dense_parity_coverage_is_typed_nonnegative_and_keeps_zero_bits() {
    for value in [0.0, -0.0, 1e-9] {
        let row = ProductFinalRow {
            id: "parity",
            case_id: "case",
            value: &value,
            unit: ProductUnit::Record,
            body: 0,
            recipe: ProductRecipe::DenseParityObservation,
        };
        let rows = [];
        let mut spent = ProductCertificateSpent::new(&rows);
        let mut seen = false;
        mark_dense_parity(&mut spent, &row, &mut seen).unwrap();
        assert!(seen);
        assert_eq!(
            mark_dense_parity(&mut spent, &row, &mut seen)
                .unwrap_err()
                .category(),
            "association"
        );
    }
    for (value, unit, body) in [
        (-1.0, ProductUnit::Record, 0),
        (f64::INFINITY, ProductUnit::Record, 0),
        (0.0, ProductUnit::Newton, 0),
        (0.0, ProductUnit::Record, 1),
    ] {
        let row = ProductFinalRow {
            id: "parity",
            case_id: "case",
            value: &value,
            unit,
            body,
            recipe: ProductRecipe::DenseParityObservation,
        };
        let rows = [];
        let mut spent = ProductCertificateSpent::new(&rows);
        let mut seen = false;
        assert_eq!(
            mark_dense_parity(&mut spent, &row, &mut seen)
                .unwrap_err()
                .category(),
            "association"
        );
        assert!(!seen);
    }
    let value = 0.0;
    let row = ProductFinalRow {
        id: "parity",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Record,
        body: 0,
        recipe: ProductRecipe::DenseParityObservation,
    };
    let rows = [];
    let mut spent = ProductCertificateSpent::new(&rows);
    spent.visits = WorkTotal::exact_count(u64::MAX);
    let mut seen = false;
    assert_eq!(
        mark_dense_parity(&mut spent, &row, &mut seen)
            .unwrap_err()
            .category(),
        "work_accounting"
    );
    assert!(!seen);
}
#[test]
fn support_empty_law_uses_unchanged_mechanical_gate_and_work() {
    let value = -0.0;
    let rows = [ProductFinalRow {
        id: "empty",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Newton,
        body: 0,
        recipe: ProductRecipe::SupportComponent {
            support: 0,
            component: Component::Ux,
        },
    }];
    let mut spent = ProductCertificateSpent::new(&rows);
    let verdict = gate(&mut spent, 0, Enclosure::point(Endpoint::ZERO), 0.0, false).unwrap();
    assert!(verdict.passed);
    assert_eq!(verdict.normalized_bits, (-0.0f64).to_bits());
    assert_eq!(
        verdict.class,
        Some(adaptive::RowClass::AbsoluteVerified { bound_bits: 0 })
    );
    let tiny = f64::from_bits(1);
    let rows = [ProductFinalRow {
        id: "tiny",
        case_id: "case",
        value: &tiny,
        unit: ProductUnit::Newton,
        body: 0,
        recipe: ProductRecipe::SupportComponent {
            support: 0,
            component: Component::Ux,
        },
    }];
    let mut spent = ProductCertificateSpent::new(&rows);
    assert!(
        gate(&mut spent, 0, Enclosure::point(Endpoint::ZERO), 1.0, false)
            .unwrap()
            .passed,
        "a nonzero empty-law output is decided by its actual allowance, not always rejected"
    );
    spent.visits = WorkTotal::exact_count(u64::MAX);
    let before = spent.scalar_operations;
    assert_eq!(spent.visit().unwrap_err().category(), "work_accounting");
    assert_eq!(spent.scalar_operations, before);
}

#[test]
fn product_final_case_raw_units_are_exactly_the_producer_normalization() {
    assert_eq!(
        ProductUnit::Millimetre.normalize(1.0).to_bits(),
        (1.0f64 / 1000.0).to_bits()
    );
    assert_eq!(
        ProductUnit::Megapascal.normalize(1.0).to_bits(),
        1_000_000.0f64.to_bits()
    );
}

#[test]
fn signed_quotient_and_disjoint_truth_hull_are_not_mixed() {
    let mut w = NumericWork::new();
    let a = Enclosure {
        lo: lift(-3.0).unwrap(),
        hi: lift(-1.0).unwrap(),
    };
    let b = Enclosure {
        lo: lift(2.0).unwrap(),
        hi: lift(4.0).unwrap(),
    };
    let q = corners(&mut w, a, b, Entry::Div).unwrap();
    assert_eq!(q.lo.cmp_value(&lift(-1.5).unwrap()), Ordering::Equal);
    assert_eq!(q.hi.cmp_value(&lift(-0.25).unwrap()), Ordering::Equal);
    let source = Enclosure::point(lift(0.0).unwrap());
    let represented = Enclosure::point(lift(1.0).unwrap());
    let joined = hull(source, represented);
    assert_eq!(
        distance(&mut w, 0.0, joined)
            .unwrap()
            .cmp_value(&Endpoint::ONE),
        Ordering::Equal
    );
    assert!(distance(&mut w, 0.0, source).unwrap().is_zero());
    assert!(w.status().is_exact());
}
#[test]
fn actual_raw_and_si_rounding_get_all_four_relative_tests() {
    let value = 1.0;
    let rows = [ProductFinalRow {
        id: "unit",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Millimetre,
        body: 0,
        recipe: ProductRecipe::NonQuantity,
    }];
    let mut spent = ProductCertificateSpent::new(&rows);
    let truth = corners(
        &mut spent.numeric,
        Enclosure::point(Endpoint::ONE),
        Enclosure::point(lift(1000.0).unwrap()),
        Entry::Div,
    )
    .unwrap();
    let v = gate(&mut spent, 0, truth, 0.001, false).unwrap();
    assert_eq!(v.normalized_bits, 0x3f50624dd2f1a9fc);
    assert_eq!(v.predicates, [Some(true); 4]);
    assert!(v.passed && spent.status().is_exact());
}
#[test]
fn small_scale_bound_and_lost_accounting_never_extract_success() {
    let value = 0.0;
    let rows = [ProductFinalRow {
        id: "small",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Newton,
        body: 0,
        recipe: ProductRecipe::NonQuantity,
    }];
    let mut spent = ProductCertificateSpent::new(&rows);
    let scale = f64::from_bits((1023u64 - 1000) << 52);
    let v = gate(
        &mut spent,
        0,
        Enclosure::point(Endpoint::ZERO),
        scale,
        false,
    )
    .unwrap();
    assert_eq!(
        v.class,
        Some(adaptive::RowClass::AbsoluteVerified { bound_bits: 1025 })
    );
    spent.verdicts.push(v);
    assert!(spent.passed());
    spent.numeric.status = WorkStatus::from_fault(WorkFault::Overflow);
    assert!(!spent.passed());
    spent.failure = Some(ProductFailure {
        cause: Cause::Numeric(NumericError::InvalidMaterial),
    });
    assert_eq!(spent.failure().unwrap().category(), "numeric");
    assert_eq!(spent.status().fault(), Some(WorkFault::Overflow));
}

#[test]
fn scalar_branches_and_body_extent_record_only_entered_operations() {
    let value = 0.0;
    let rows = [ProductFinalRow {
        id: "zero",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Newton,
        body: 0,
        recipe: ProductRecipe::NonQuantity,
    }];
    let mut zero = ProductCertificateSpent::new(&rows);
    assert!(
        gate(&mut zero, 0, Enclosure::point(Endpoint::ZERO), 0.0, false)
            .unwrap()
            .passed
    );
    assert_eq!(zero.scalar_operations.exact(), Ok(2)); // absolute_bound's divide and scale-back, no threshold.
    let mut input = ProductCertificateSpent::new(&rows);
    gate(&mut input, 0, Enclosure::point(Endpoint::ZERO), 0.0, true).unwrap();
    assert_eq!(input.scalar_operations.exact(), Ok(0));
    let mut coupled = ProductCertificateSpent::new(&rows);
    assert_eq!(
        couple([1.0, 2.0, 3.0, 4.0], 0.0, &mut coupled).unwrap(),
        [1.0, 2.0, 3.0, 4.0]
    );
    assert_eq!(coupled.scalar_operations.exact(), Ok(0));
    couple([1.0, 2.0, 3.0, 4.0], 2.0, &mut coupled).unwrap();
    assert_eq!(coupled.scalar_operations.exact(), Ok(4));
    use super::super::super::source::{PrimitiveSource, SourceParts, StraightMember};
    let source = PrimitiveSource::new(SourceParts {
        nodes: vec![[0.0; 3], [1.0, 0.0, 0.0]],
        members: vec![StraightMember {
            id: 0,
            node_i: 0,
            node_j: 1,
            elastic_modulus: 1.0,
            shear_modulus: 1.0,
            area: 1.0,
            second_moment_y: 1.0,
            second_moment_z: 1.0,
            torsion_constant: 1.0,
            y_reference: [0.0, 1.0, 0.0],
        }],
        ..SourceParts::default()
    })
    .unwrap();
    let mut extent = ProductCertificateSpent::new(&rows);
    assert_eq!(body_extent(&source, 0, &mut extent).unwrap(), 1.0);
    assert_eq!(extent.scalar_operations.exact(), Ok(9));
    // Synthetic counter seed permits three actual subtractions, then rejects before the first square.
    let mut prefix = ProductCertificateSpent::new(&rows);
    prefix.scalar_operations = WorkTotal::exact_count(u64::MAX - 3);
    assert_eq!(
        body_extent(&source, 0, &mut prefix).unwrap_err().category(),
        "work_accounting"
    );
    assert_eq!(prefix.scalar_operations.exact(), Ok(u64::MAX));
    assert_eq!(prefix.visits.exact(), Ok(5)); // two source-node visits plus three arithmetic entries.
    assert_eq!(prefix.status().fault(), Some(WorkFault::Overflow));
}
#[test]
fn produced_comparison_and_helper_errors_survive_collection_loss() {
    let value = 0.0;
    let rows = [ProductFinalRow {
        id: "failure",
        case_id: "case",
        value: &value,
        unit: ProductUnit::Newton,
        body: 0,
        recipe: ProductRecipe::NonQuantity,
    }];
    let mut exact = ProductCertificateSpent::new(&rows);
    exact.visits = WorkTotal::exact_count(u64::MAX); // Synthetic collection seed.
    let error = exact_test(&mut exact, Endpoint::ZERO, f64::NAN, 1.0, true).unwrap_err();
    assert!(matches!(
        error.cause,
        Cause::Numeric(NumericError::NonFinite)
    ));
    assert_eq!(exact.status().fault(), Some(WorkFault::Overflow));
    let mut helper = ProductCertificateSpent::new(&rows);
    helper.visits = WorkTotal::exact_count(u64::MAX);
    let error = helper
        .small_bound(f64::NAN, f64::from_bits((1023u64 - 1000) << 52))
        .unwrap_err();
    assert!(matches!(
        error.cause,
        Cause::Helper(directed::certificate::HelperError::InvalidSmallBoundInput)
    ));
    assert_eq!(helper.status().fault(), Some(WorkFault::Overflow));
    assert_eq!(helper.scalar_operations.exact(), Ok(0)); // The actual helper input guard ran, not its arithmetic.
    let mut stopped = ProductCertificateSpent::new(&rows);
    stopped.numeric.status = WorkStatus::from_fault(WorkFault::Overflow);
    assert_eq!(
        stopped.small_bound(f64::NAN, 1.0).unwrap_err().category(),
        "work_accounting"
    );
    assert_eq!(stopped.visits.exact(), Ok(0));
}

#[test]
fn i51_prepared_reserve_and_value_prefixes_are_checked() {
    let mut spent=ProductCertificateSpent::new(&[]);
    let v=spent.prepared_reserve::<Enclosure>(0,3).unwrap();
    assert_eq!(spent.prepared_capacities[0],v.capacity()*std::mem::size_of::<Enclosure>());
    spent.visits=WorkTotal::exact_count(u64::MAX);
    let before=spent.prepared_capacities;
    assert_eq!(spent.prepared_reserve::<Enclosure>(1,4).unwrap_err().category(),"work_accounting");
    assert_eq!(spent.prepared_capacities,before);
    let mut visits=WorkTotal::exact_count(u64::MAX);
    assert_eq!(values_visit(&mut visits).unwrap_err().category(),"work_accounting");
    assert_eq!(values_visit(&mut visits).unwrap_err().category(),"work_accounting");
    let specs=[ProductRowSpec::mode("mode","case",0,1).unwrap(),ProductRowSpec::parity("parity","case",0,(-0.0f64).to_bits()).unwrap()];
    assert_eq!(specs[0].observed,Some(1f64.to_bits()));
    assert_eq!(specs[1].observed,Some((-0.0f64).to_bits()));
    assert!(ProductRowSpec::mechanical("mode","case",ProductUnit::Record,0,ProductRecipe::NonQuantity).is_err());
    assert!(ProductRowSpec::parity("parity","case",0,f64::NAN.to_bits()).is_err());
}

fn i51_support_specs(order:&[usize])->Vec<ProductRowSpec<'static>> {
    order.iter().map(|&i|ProductRowSpec::mechanical(["fx","fy","fz"][i],"case",ProductUnit::Newton,0,
        ProductRecipe::SupportComponent{support:3,component:Component::ALL[i]}).unwrap()).collect()
}
#[test]
fn i51_support_hypot_identity_order_zero_subnormal_and_failed_prefixes() {
    for input in [[3.0,4.0,12.0],[-0.0,0.0,-0.0],[f64::from_bits(1),f64::from_bits(2),f64::from_bits(3)]] {
        let expected=input[0].hypot(input[1]).hypot(input[2]);
        for order in [[0,1,2],[2,0,1]] {
            let specs=i51_support_specs(&order);let values=order.map(|i|input[i]);let mut w=ProductCertificateSpent::new(&[]);
            let value=support_hypot(&specs,&values,3,0,ProductUnit::Newton,&mut w).unwrap();
            assert_eq!(value.to_bits(),expected.to_bits());assert_eq!(w.scalar_operations.exact(),Ok(2));
            if input[0].is_subnormal(){assert!(value.is_subnormal());}
        }
    }
    for variant in 0..5 {
        let mut specs=i51_support_specs(&[0,1,2]);let mut values=vec![3.0,4.0,12.0];
        match variant {0=>{specs.pop();values.pop();},1=>{specs.push(i51_support_specs(&[0]).remove(0));values.push(3.0);},
            2=>specs[1].unit=ProductUnit::NewtonMetre,3=>specs[1].recipe=ProductRecipe::SupportComponent{support:4,component:Component::Uy},_=>values[0]=f64::NAN}
        let mut w=ProductCertificateSpent::new(&[]);assert!(support_hypot(&specs,&values,3,0,ProductUnit::Newton,&mut w).is_err());
        assert_eq!(w.scalar_operations.exact(),Ok(0));
    }
    let specs=i51_support_specs(&[0,1,2]);let mut w=ProductCertificateSpent::new(&[]);
    w.scalar_operations=WorkTotal::exact_count(u64::MAX-1);
    assert_eq!(support_hypot(&specs,&[3.0,4.0,12.0],3,0,ProductUnit::Newton,&mut w).unwrap_err().category(),"work_accounting");
    assert_eq!(w.visits.exact(),Ok(14));assert!(w.status().fault().is_some());
    let before=w.scalar_operations;
    assert!(support_hypot(&specs,&[3.0,4.0,12.0],3,0,ProductUnit::Newton,&mut w).is_err());assert_eq!(w.scalar_operations,before);
    let mut w=ProductCertificateSpent::new(&[]);assert!(support_hypot(&specs,&[f64::MAX,f64::MAX,0.0],3,0,ProductUnit::Newton,&mut w).is_err());
    assert_eq!(w.scalar_operations.exact(),Ok(2));
}
fn i51_seed_source()->super::super::super::source::PrimitiveSource {
    use super::super::super::source::*;
    PrimitiveSource::new(SourceParts{nodes:vec![[0.;3],[1.,0.,0.]],
        members:vec![StraightMember{id:7,node_i:0,node_j:1,elastic_modulus:200e9,shear_modulus:80e9,
            area:f64::from_bits(0x3f7872fa3a37ac13),second_moment_y:f64::from_bits(0x3efc52664442210a),
            second_moment_z:f64::from_bits(0x3efc52664442210a),torsion_constant:f64::from_bits(0x3f0c52664442210a),y_reference:[0.,1.,0.]}],
        constraints:Component::ALL.iter().map(|&component|Constraint{dof:Dof{node:0,component},value:if component==Component::Ux{0.001}else{0.}}).collect(),
        loads:vec![NodalLoad{dof:Dof{node:1,component:Component::Ux},value:1.,source_id:"tip".into()}],
        supports:vec![SupportGroup{id:3,node:0,restrained:[true;6],springs:vec![],directional_springs:vec![]}],..SourceParts::default()}).unwrap()
}
#[test]
fn i51_seed_is_same_draft_bound_and_prescriptions_are_never_seeded() {
    use super::super::super::origins::*;
    use crate::structural::retained_api::{CaseLimit,ExecutionOutcome};
    let source=i51_seed_source();let mut invocation=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let cases=invocation.solve_cases(&[source],CaseLimit::new(u64::MAX)).unwrap();
    let owner=match &cases[0].outcome {ExecutionOutcome::Selected(v)=>v,_=>panic!("native fixture")};
    let anchor=std::sync::Arc::new(ProofAnchor{owner:invocation.product_owner_stamp(cases[0].run,owner).unwrap()});
    let mode_value=1.0;let only_mode=[ProductFinalRow{id:"mode",case_id:"case",value:&mode_value,unit:ProductUnit::Record,body:0,recipe:ProductRecipe::NonQuantity}];
    let mut bound=ProductCertificateSpent::new(&only_mode);assert!(row_scales(owner,&[],&mut bound).is_err());
    let ptr=bound.verdicts.as_ptr();let capacity=bound.verdicts.capacity();assert!(capacity>=1);
    assert!(row_scales(owner,&[],&mut bound).is_err());assert_eq!(bound.verdicts.as_ptr(),ptr);assert_eq!(bound.verdicts.capacity(),capacity);

    let stale=std::sync::Arc::new(ProofAnchor{owner:invocation.product_owner_stamp(cases[0].run,owner).unwrap()});
    let mut foreign_inv=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let foreign_cases=foreign_inv.solve_cases(&[i51_seed_source()],CaseLimit::new(u64::MAX)).unwrap();
    let foreign_owner=match &foreign_cases[0].outcome{ExecutionOutcome::Selected(v)=>v,_=>panic!()};
    let foreign_anchor=std::sync::Arc::new(ProofAnchor{owner:foreign_inv.product_owner_stamp(foreign_cases[0].run,foreign_owner).unwrap()});
    let mut builder=ProductValuesBuilder{values:vec![2.0,3.0],maxima:vec![(0,7)],anchor:std::sync::Arc::clone(&anchor),visits:WorkTotal::zero()};
    assert_eq!(builder.value(0).unwrap(),None);assert_eq!(builder.value(1).unwrap(),Some(3.0));assert_eq!(builder.visits.exact(),Ok(3));
    builder.visits=WorkTotal::exact_count(u64::MAX);assert_eq!(builder.value(1).unwrap_err().category(),"work_accounting");
    assert!(builder.abandon().visits.status().fault().is_some());

    let law=[bridge::ProposedMemberLaw{member:&owner.source().members()[0],diameter:0.2,effective_wall:0.01,
        material:MaterialOperands::Ordinary{e:200e9,g:80e9},represented_z:f64::from_bits(0x3f31b37feaa954a6)}];
    let first=source_residual::source_residual_for_law(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&law,source_residual::ReadoutLaw::AdmittedK);
    let post=source_residual::source_residual_for_law(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&law,source_residual::ReadoutLaw::AdmittedK);
    let (post_error,post_work)=post.into_readouts(&foreign_anchor);assert!(post_error.is_err());assert_eq!(post_work.correction.calls.exact(),Ok(1));
    let mut only_k=ProductCertificateSpent::new(&[]);assert!(only_k.source_correction_calls().is_none());only_k.native_k=Some(post_work);
    assert_eq!(only_k.source_correction_calls().unwrap().exact(),Ok(1));assert!(only_k.native.is_none());
    let (seed,mut kw)=first.into_readouts(&anchor);let mut seed=seed.unwrap();assert_eq!(kw.correction.calls.exact(),Ok(1));
    let source_run=|seed:&source_residual::LaneReadouts,anchor:&std::sync::Arc<ProofAnchor>|source_residual::source_residual_prepared(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&law,anchor,seed);
    let foreign=source_run(&seed,&foreign_anchor);assert!(foreign.result().is_err());assert_eq!(foreign.work.correction.calls.exact(),Ok(0));
    let stale_run=source_run(&seed,&stale);assert!(stale_run.result().is_err());assert_eq!(stale_run.work.correction.calls.exact(),Ok(0));assert!(stale_run.work.data_capacity>0);
    let last=seed.rows.pop().unwrap();let missing=source_run(&seed,&anchor);assert!(missing.result().is_err());assert_eq!(missing.work.correction.calls.exact(),Ok(0));seed.rows.push(last);
    let saved=seed.rows[6];seed.rows[6]=Enclosure{lo:Endpoint::ONE,hi:Endpoint::ZERO};let inverted=source_run(&seed,&anchor);assert!(inverted.result().is_err());assert_eq!(inverted.work.correction.calls.exact(),Ok(0));seed.rows[6]=saved;
    seed.rows[0]=Enclosure::point(lift(123.0).unwrap());
    let good=source_run(&seed,&anchor);let value=good.result().unwrap();let exact=lift(0.001).unwrap();
    assert_eq!(value.rows[0].lo,exact);assert_eq!(value.rows[0].hi,exact);assert_eq!(good.work.correction.calls.exact(),Ok(1));
    let (wrong_law,_)=good.into_readouts(&anchor);let wrong_law=wrong_law.unwrap();assert!(source_run(&wrong_law,&anchor).result().is_err());
    kw.readout_law=source_residual::ReadoutLaw::AnnularSource;kw.visits=WorkTotal::exact_count(u64::MAX);
    let view=owner.source_bridge_view(owner.source(),&owner.evidence().source_encoding,owner.selected_precision()).result.unwrap();
    assert!(source_residual::validate_seed(&view,&anchor,&seed,&mut kw).is_err());assert!(kw.status().fault().is_some());
}

#[test]
fn i51_proof_layout_and_arc_capacity_are_concrete() {
    macro_rules! layout {($t:ty)=>{println!("I51_PROOF_LAYOUT {} {} {}",stringify!($t),std::mem::size_of::<$t>(),std::mem::align_of::<$t>());};}
    layout!(ProofAnchor);layout!(ProofData<'static,'static>);layout!(ProductProofDraft<'static,'static>);
    layout!(ProjectedProofDraft<'static,'static>);layout!(ProductProjectionSpent<'static,'static>);
    layout!(ProductRowSpec<'static>);layout!(ProductFinalRow<'static>);layout!(ProductCertificateSpent<'static>);
    layout!(source_residual::LaneReadouts);layout!(ResidualWork);layout!(ProductValuesBuilder);layout!(FrozenProductValues);
    layout!(ProductMaximumValue);layout!(ValuesCompletionWork);layout!(CertifiedProductProof);layout!(ProductProofFailure);
    layout!(ProductRowVerdict);layout!(ProductSummaryCoverage);layout!(MemberEnclosures);layout!((usize,super::super::super::wide::multi::Binary64Outcome));
    assert!(std::alloc::Layout::array::<source_residual::LaneReadouts>(usize::MAX).is_err());
}

#[test]
fn i51_projection_conversion_failure_retains_entered_arithmetic() {
    let mut work=ProductCertificateSpent::new(&[]);work.visits=WorkTotal::exact_count(u64::MAX-2);
    let e=project_hull(Enclosure::point(lift(1.).unwrap()),ProductUnit::Newton,0,&mut work).unwrap_err();
    assert_eq!(e.category(),"work_accounting");assert!(work.numeric.wide.checked_lme().exact().unwrap()>0);
}

#[test]
fn i51_projection_keeps_normal_subnormal_underflow_and_overflow_outcomes() {
    use super::super::super::wide::multi::Binary64Outcome as B;
    for (x,expected) in [(lift(1.).unwrap(),0),(lift(f64::from_bits(1)).unwrap(),1),
        (shift(&Endpoint::ONE,-1076).unwrap(),2),(shift(&Endpoint::ONE,1024).unwrap(),3)] {
        let mut work=ProductCertificateSpent::new(&[]);work.projection_outcomes=work.prepared_reserve(5,1).unwrap();
        let value=project_hull(Enclosure::point(x),ProductUnit::Newton,17,&mut work);
        assert_eq!(work.projection_conversions.exact(),Ok(1));assert_eq!(work.projection_outcomes.len(),1);
        assert_eq!(work.projection_outcomes[0].0,17);
        match work.projection_outcomes[0].1 {
            B::Normal(v)=>{assert_eq!(expected,0);assert_eq!(value.unwrap(),v);},
            B::Subnormal{value:v,relative_precision}=>{assert_eq!(expected,1);assert_eq!(value.unwrap(),v);assert!(relative_precision>0.);},
            B::Underflow{negative}=>{assert_eq!(expected,2);assert!(!negative);assert_eq!(value.unwrap().to_bits(),0);},
            B::Overflow{negative}=>{assert_eq!(expected,3);assert!(!negative);assert_eq!(value.unwrap_err().category(),"numeric");},
        }
    }
}

#[test]
fn prepared_trace_typed_failure_has_no_endpoint_and_absence_is_not_zero_work() {
    let error=ProductFailure{cause:Cause::Native(bridge::BridgeError::Alpha{block:7,alpha_hi:Endpoint::ONE})};
    let mut copies=TraceCopyWork::default();
    assert!(matches!(error.typed_cause(&mut copies),ProductFailureView::Native(source_residual::BridgeFailure::AlphaCondition{block:7})));
    let mut work=ProductCertificateSpent::new(&[]);
    let typed=work.typed_trace(&mut copies);
    assert!(typed.lanes.iter().all(Option::is_none));assert!(!typed.completion_merged);
    assert!(typed.projection_outcomes.is_empty());
    work.projection_outcomes=work.prepared_reserve(5,1).unwrap();
    let _=project_hull(Enclosure::point(Endpoint::ONE),ProductUnit::Newton,17,&mut work).unwrap();
    let typed=work.typed_trace(&mut copies);assert_eq!(typed.projection_outcomes.len(),1);
    assert_eq!(typed.projection_outcomes[0].0,17);
    assert!(matches!(typed.projection_outcomes[0].1,super::super::super::wide::multi::Binary64Outcome::Normal(1.0)));
    assert_eq!(typed.projection_conversions.exact(),Ok(1));
    println!("I51_TRACE_LAYOUT ProductCertificateSpent={} ProductProofTrace={} ProductFailureView={}",
        std::mem::size_of::<ProductCertificateSpent<'static>>(),std::mem::size_of::<ProductProofTrace<'static>>(),std::mem::size_of::<ProductFailureView<'static>>());
}

#[test]
fn prepared_trace_actual_completed_k_and_failed_source_remain_an_entered_prefix() {
    use super::super::super::origins::*;
    use crate::structural::retained_api::{CaseLimit,ExecutionOutcome};
    let mut invocation=RecordedInvocation::new(u64::MAX,OriginCapacity::for_calls(&[1],&[]).unwrap()).unwrap();
    let cases=invocation.solve_cases(&[i51_seed_source()],CaseLimit::new(u64::MAX)).unwrap();
    let owner=match &cases[0].outcome{ExecutionOutcome::Selected(v)=>v,_=>panic!("native fixture")};
    let anchor=std::sync::Arc::new(ProofAnchor{owner:invocation.product_owner_stamp(cases[0].run,owner).unwrap()});
    let stale=std::sync::Arc::new(ProofAnchor{owner:invocation.product_owner_stamp(cases[0].run,owner).unwrap()});
    let law=[bridge::ProposedMemberLaw{member:&owner.source().members()[0],diameter:0.2,effective_wall:0.01,
        material:MaterialOperands::Ordinary{e:200e9,g:80e9},represented_z:f64::from_bits(0x3f31b37feaa954a6)}];
    let first=source_residual::source_residual_for_law(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&law,source_residual::ReadoutLaw::AdmittedK);
    let (result,work)=first.into_readouts(&anchor);let mut proof=ProductCertificateSpent::new(&[]);
    let seed=proof.retain_lane(result,work).unwrap();let mut reads=TraceCopyWork::default();
    {let view=proof.typed_trace(&mut reads);assert!(view.lanes[0].as_ref().unwrap().result.is_ok());assert!(view.lanes[1].is_none());}
    let second=source_residual::source_residual_prepared(owner,owner.source(),&owner.evidence().source_encoding,owner.selected_precision(),&law,&stale,&seed);
    let (result,work)=second.into_readouts(&stale);assert!(proof.retain_lane(result,work).is_err());
    let view=proof.typed_trace(&mut reads);assert!(matches!(view.lanes[1].as_ref().unwrap().result,Err(source_residual::BridgeFailure::MemberOwner)));
    assert_eq!(view.lanes[0].as_ref().unwrap().work.correction_calls.exact(),Ok(1));
    assert_eq!(view.lanes[1].as_ref().unwrap().work.correction_calls.exact(),Ok(0));
    assert!(view.lanes[1].as_ref().unwrap().work.view_data_capacity>0);
    assert!(view.projection_outcomes.is_empty() && !view.completion_merged);
}

// ---------------------------------------------------------------- I61 summary coverage
// Actual native runs of the K4 model inventory (models.txt). Each vector is the
// proof's own summary_coverage_data on the selected owner; the trace only borrows it.
fn i61_model(name:&str)->super::super::super::source::PrimitiveSource {
    use super::super::super::source::*;
    let hf=|s:&str|f64::from_bits(u64::from_str_radix(s,16).unwrap());
    let dof=|n:&str,c:&str|Dof{node:n.parse().unwrap(),component:Component::from_index(c.parse().unwrap())};
    let mut parts=SourceParts::default();let mut inside=false;
    for line in include_str!("models.txt").lines() {
        let f:Vec<&str>=line.split_whitespace().collect();
        if f.is_empty(){continue;}
        if f[0]=="model"{if inside{break;}inside=f[1]==name;continue;}
        if !inside{continue;}
        match f[0] {
            "node"=>parts.nodes.push([hf(f[1]),hf(f[2]),hf(f[3])]),
            "member"=>parts.members.push(StraightMember{id:f[1].parse().unwrap(),node_i:f[2].parse().unwrap(),node_j:f[3].parse().unwrap(),
                elastic_modulus:hf(f[4]),shear_modulus:hf(f[5]),area:hf(f[6]),second_moment_y:hf(f[7]),second_moment_z:hf(f[8]),
                torsion_constant:hf(f[9]),y_reference:[hf(f[10]),hf(f[11]),hf(f[12])]}),
            "spring"=>parts.springs.push(Spring{id:f[1].parse().unwrap(),dof:dof(f[2],f[3]),stiffness:hf(f[4])}),
            "constraint"=>parts.constraints.push(Constraint{dof:dof(f[1],f[2]),value:hf(f[3])}),
            "load"=>parts.loads.push(NodalLoad{dof:dof(f[1],f[2]),value:hf(f[3]),source_id:f[4].to_string()}),
            "expect"|"seed"|"end"=>{},
            other=>panic!("{name}: unsupported model line {other}"),
        }
    }
    assert!(inside,"{name}");PrimitiveSource::new(parts).unwrap()
}
fn i61_actual(name:&str)->(Box<adaptive::RetainedSolve>,Vec<ProductSummaryCoverage>) {
    let mut meter=adaptive::InvocationMeter::new(u64::MAX);
    let owner=match adaptive::solve_case(i61_model(name),adaptive::CaseLimit::new(u64::MAX),&mut meter) {
        adaptive::CaseOutcome::Selected(v)=>v,other=>panic!("{name}: {other:?}")};
    let view=owner.source_bridge_view(owner.source(),&owner.evidence().source_encoding,owner.selected_precision()).result.unwrap();
    let mut spent=ProductCertificateSpent::new(&[]);
    let coverage=summary_coverage_data(&owner,view.data(),&mut spent).unwrap();
    drop(view);(owner,coverage)
}
fn i61_check(owner:&adaptive::RetainedSolve,coverage:Vec<ProductSummaryCoverage>)->Result<(),ProductFailure> {
    let mut proof=ProductCertificateSpent::new(&[]);proof.coverage=coverage;
    let mut copies=TraceCopyWork::default();
    let trace=proof.typed_trace(&mut copies);
    assert_eq!(trace.summary_coverage.as_ptr(),proof.coverage.as_ptr(),"borrowed proof slice, not a copy");
    trace.check_summary_coverage(owner,&mut copies)
}
#[test]
fn i61_actual_native_coverage_is_borrowed_and_all_nine_flags_rederive() {
    // §5 controls: zero/no-data body (ALL-ZERO-BODY body 1); native p512 (SKEW-K1E-60)
    // charge = force/moment stop; native p128/p256 charge = estimate. Actual native runs.
    let mut seen=std::collections::BTreeSet::new();
    for name in ["ALL-ZERO-BODY","SKEW-K1E-60","SKEW-K1E-28","SKEW6-K1E-12","TWO-SPAN","PRESCRIBED","REACTIONS-ONLY","ZERO-TORSION-345"] {
        let (owner,coverage)=i61_actual(name);let p=owner.selected_precision();seen.insert(p);
        assert_eq!(coverage.len(),owner.source().body_count() as usize);
        for (i,c) in coverage.iter().enumerate() {
            assert_eq!(c.body as usize,i);
            if p==512 {assert_eq!(c.charge,[c.stop[2],c.stop[3]],"{name}");} else {assert_eq!(c.charge,c.estimate,"{name}");}
        }
        assert_eq!(owner.evidence().floor.is_some(),p==512,"{name}");
        let mut copies=TraceCopyWork::default();
        let mut proof=ProductCertificateSpent::new(&[]);proof.coverage=coverage.clone();
        let before=copies.events;let trace=proof.typed_trace(&mut copies);
        trace.check_summary_coverage(&owner,&mut copies).unwrap();
        assert!(copies.events.exact().unwrap()>before.exact().unwrap()+3*coverage.len() as u64,"per-body projection accounted");
        println!("I61_ACTUAL {name} p={p} floor={:?} coverage={:?} copies={:?}",owner.evidence().floor,coverage,copies);
        if name=="ALL-ZERO-BODY" {
            assert_eq!(p,128);assert_eq!(coverage.len(),2);
            assert!(coverage[0].has_data);
            assert!(!coverage[1].has_data,"the unloaded body keeps its own entry");assert_eq!(coverage[1].stop,[false;4]);
        }
        if name=="SKEW-K1E-60" {assert_eq!(p,512);}
    }
    assert!(seen.contains(&512) && seen.contains(&128));
    println!("I61_LAYOUT ProductProofTrace={} CoverageBody={} CoverageFacts={} ProductSummaryCoverage={} coordinate={}",
        std::mem::size_of::<ProductProofTrace<'static>>(),std::mem::size_of::<CoverageBody>(),std::mem::size_of::<CoverageFacts>(),
        std::mem::size_of::<ProductSummaryCoverage>(),std::mem::size_of::<[f64;3]>());
}
#[test]
fn i61_rederivation_refuses_partial_reordered_and_mismatched_vectors() {
    // Synthetic tamper of actual native vectors. Every derivable bit and identity
    // must be refused; stop/has_data are attested and are not derivable at p<512.
    for name in ["ALL-ZERO-BODY","SKEW-K1E-60"] {
        let (owner,coverage)=i61_actual(name);let p=owner.selected_precision();
        i61_check(&owner,coverage.clone()).unwrap();
        let mut dropped=coverage.clone();dropped.pop();
        assert_eq!(i61_check(&owner,dropped).unwrap_err().category(),"association");
        assert_eq!(i61_check(&owner,Vec::new()).unwrap_err().category(),"association","empty is null in PP, never checked complete");
        let mut extra=coverage.clone();extra.push(*coverage.last().unwrap());
        assert!(i61_check(&owner,extra).is_err());
        if coverage.len()>1 {let mut swapped=coverage.clone();swapped.swap(0,1);assert!(i61_check(&owner,swapped).is_err());}
        for b in 0..coverage.len() {
            for bit in 0..4 {
                let mut m=coverage.clone();
                if bit<2 {m[b].estimate[bit]^=true;} else {m[b].charge[bit-2]^=true;}
                assert_eq!(i61_check(&owner,m).unwrap_err().category(),"association","{name} body {b} bit {bit}");
            }
            let mut m=coverage.clone();m[b].body^=1;assert!(i61_check(&owner,m).is_err());
            if p==512 {
                for k in 2..4 {let mut m=coverage.clone();m[b].stop[k]^=true;assert!(i61_check(&owner,m).is_err(),"{name} p512 stop {k}");}
            }
        }
    }
}
#[test]
fn i61_synthetic_rederivation_precision_floor_extent_and_absent_kinds() {
    // Synthetic public facts (labelled): p512 zero/positive floor, p128/p256 under
    // the fixed 1024 proof, L=0 against L!=0, and absent kinds.
    let facts=|present:[bool;4],extent:f64,e:[f64;2],floor:Option<[f64;2]>,precision:u32|CoverageFacts{present,
        extent_bits:extent.to_bits(),resolution_bits:[e[0].to_bits(),e[1].to_bits()],floor_bits:floor.map(|f|[f[0].to_bits(),f[1].to_bits()]),precision};
    let body=|stop:[bool;4],has_data:bool|CoverageBody{body:3,stop,has_data};
    let all=[true;4];
    // p512, actual zero floor: charge is the stop pair even where estimate differs.
    let c=rederive_coverage(body([true,true,false,true],true),&facts(all,2.0,[1.0,0.0],Some([0.0,0.0]),512)).unwrap();
    assert_eq!(c.estimate,[true,true]);assert_eq!(c.charge,[false,true]);assert_eq!((c.body,c.has_data),(3,true));
    let c=rederive_coverage(body([false;4],false),&facts(all,2.0,[1.0,1.0],Some([0.0,0.0]),512)).unwrap();
    assert_eq!(c.charge,[false,false]);assert_ne!(c.charge,c.estimate);
    // p512, positive floor forces its present kind's stop; charge still equals stop.
    assert!(rederive_coverage(body([false;4],false),&facts(all,2.0,[0.0,0.0],Some([1.0,0.0]),512)).is_err());
    let c=rederive_coverage(body([false,false,true,false],false),&facts(all,2.0,[0.0,0.0],Some([1.0,0.0]),512)).unwrap();
    assert_eq!(c.charge,[true,false]);assert_eq!(c.estimate,[false,false]);
    // p128/p256: charge = estimate whatever the stop flags; proof precision 1024 is not an input.
    for p in [128,256] {
        let c=rederive_coverage(body([true;4],true),&facts(all,2.0,[0.0,1.0],None,p)).unwrap();
        assert_eq!(c.estimate,[true,true]);assert_eq!(c.charge,c.estimate);
        let c=rederive_coverage(body([false;4],false),&facts(all,0.0,[0.0,1.0],None,p)).unwrap();
        assert_eq!(c.estimate,[false,true],"L=0 creates no coupling");assert_eq!(c.charge,c.estimate);
        assert!(rederive_coverage(body([false;4],false),&facts(all,1.0,[0.0,0.0],Some([0.0,0.0]),p)).is_err(),"no floor below p512");
    }
    for p in [512,1024,64] {assert!(rederive_coverage(body([false;4],false),&facts(all,1.0,[0.0,0.0],None,p)).is_err());}
    // Absent kinds: no estimate without layout presence and no stop on an absent kind.
    let some=[true,true,false,true];
    let c=rederive_coverage(body([true,false,false,false],true),&facts(some,2.0,[1.0,1.0],None,128)).unwrap();
    assert_eq!(c.estimate,[false,true]);
    assert!(rederive_coverage(body([false,false,true,false],false),&facts(some,2.0,[1.0,1.0],None,128)).is_err());
    let c=rederive_coverage(body([false;4],false),&facts([false;4],0.0,[0.0,0.0],None,256)).unwrap();
    assert_eq!((c.estimate,c.charge),([false;2],[false;2]));
}

// ---------------------------------------------------------------- RV77 reviewer tests (RV77-S1)
// Adopted verbatim from REVIEW_RV77/coverage_producer_01/tests/rv77_enum.rs (body unchanged;
// wrapped as an inline module of this test file instead of a #[path] child of final_case.rs).
mod rv77_enum {
// RV77 independent enumeration (reviewer-owned; not part of the candidate).
// Included into final_case.rs only in RV77's private archive copy, as
//   #[cfg(test)] #[path = "<scratch>/rv77_enum.rs"] mod rv77_enum;
// The oracle is RV77's own row-level transcription of native
// summary_coverage_data (final_case.rs at c618675e84, lines 1461-1515), written
// from the source, not from I61's tests or rederive_coverage.
use super::*;

#[derive(Clone, Copy)]
struct Row { kind: usize, input: bool, nonzero: bool }

/// RV77 model of native summary_coverage_data for one body.
fn native_model(body: u32, rows: &[Row], extent: f64, e: [f64; 2], floor: Option<[f64; 2]>, p: u32, has_data: bool)
    -> ProductSummaryCoverage {
    let mut positive = [false; 4];
    let mut present = [false; 4];
    for r in rows {
        present[r.kind] = true;
        if !r.input && r.nonzero { positive[r.kind] = true; }
    }
    if extent != 0.0 {
        positive = [positive[0] || positive[1], positive[0] || positive[1], positive[2] || positive[3], positive[2] || positive[3]];
    }
    if let Some(f) = floor { positive[2] |= f[0] > 0.0; positive[3] |= f[1] > 0.0; }
    let mut stop = [false; 4];
    for r in rows { stop[r.kind] |= positive[r.kind] || r.nonzero; }
    let mut hats = [e[0] > 0.0, e[1] > 0.0];
    if extent != 0.0 { hats = [hats[0] || hats[1]; 2]; }
    let estimate = [present[2] && hats[0], present[3] && hats[1]];
    let charge = if p == 512 { [present[2] && positive[2], present[3] && positive[3]] } else { estimate };
    ProductSummaryCoverage { body, stop, estimate, charge, has_data }
}

fn facts_of(rows: &[Row], extent: f64, e: [f64; 2], floor: Option<[f64; 2]>, p: u32) -> CoverageFacts {
    let mut present = [false; 4];
    for r in rows { present[r.kind] = true; }
    CoverageFacts { present, extent_bits: extent.to_bits(), resolution_bits: [e[0].to_bits(), e[1].to_bits()],
        floor_bits: floor.map(|f| [f[0].to_bits(), f[1].to_bits()]), precision: p }
}

/// Row types per kind: bit0 non-input zero, bit1 non-input nonzero, bit2 input zero, bit3 input nonzero.
fn rows_of(masks: [u8; 4]) -> Vec<Row> {
    let mut v = Vec::new();
    for k in 0..4 {
        for t in 0..4 {
            if masks[k] & (1 << t) != 0 { v.push(Row { kind: k, input: t >= 2, nonzero: t % 2 == 1 }); }
        }
    }
    v
}

const EXTENTS: [f64; 3] = [0.0, 2.0, 5e-324];
const ES: [f64; 2] = [0.0, 1.0];
fn modes() -> Vec<(u32, Option<[f64; 2]>)> {
    let mut m = vec![(128, None), (256, None)];
    for f in [0.0, 3.0] { for g in [0.0, 3.0] { m.push((512, Some([f, g]))); } }
    m
}

#[test]
fn rv77_genuine_domain_rederives_bit_for_bit_and_is_never_refused() {
    // Genuine domain: recover::layout marks only displacement rows input-derived,
    // so force/moment masks use only bits 0..1.
    let (mut cases, mut p512, mut differ_est_charge) = (0u64, 0u64, 0u64);
    for m0 in 0..16u8 { for m1 in 0..16u8 { for m2 in 0..4u8 { for m3 in 0..4u8 {
        let rows = rows_of([m0, m1, m2, m3]);
        for &x in &EXTENTS { for &ef in &ES { for &em in &ES { for (p, floor) in modes() { for hd in [false, true] {
            let native = native_model(7, &rows, x, [ef, em], floor, p, hd);
            let facts = facts_of(&rows, x, [ef, em], floor, p);
            let payload = CoverageBody { body: 7, stop: native.stop, has_data: native.has_data };
            let got = rederive_coverage(payload, &facts)
                .unwrap_or_else(|e| panic!("genuine refused {:?} rows={m0},{m1},{m2},{m3} x={x} e={ef},{em} p={p} floor={floor:?}", e.category()));
            assert_eq!(got, native, "rows={m0},{m1},{m2},{m3} x={x} e={ef},{em} p={p} floor={floor:?}");
            // Tampering any derivable bit is detected by the equality the seam uses.
            for bit in 0..4 {
                let mut t = native;
                if bit < 2 { t.estimate[bit] ^= true; } else { t.charge[bit - 2] ^= true; }
                assert_ne!(got, t);
            }
            if p == 512 { p512 += 1; if native.estimate != native.charge { differ_est_charge += 1; } }
            cases += 1;
        }}}}}
    }}}}
    println!("RV77_GENUINE cases={cases} p512={p512} p512_estimate_ne_charge={differ_est_charge} refusals=0 mismatches=0");
    assert_eq!(cases, 16 * 16 * 4 * 4 * 3 * 2 * 2 * 6 * 2);
}

#[test]
fn rv77_full_payload_domain_matches_independent_spec() {
    // Every compact payload (16 stop patterns x has_data) against every public-fact
    // pattern, including invalid precision/floor pairings. RV77's spec of the
    // candidate's contract: refuse iff (p,floor) invalid, or a stop on an absent
    // kind, or a present force/moment kind with positive floor and no stop.
    let mut precisions: Vec<(u32, Option<[f64; 2]>)> = modes();
    for p in [64u32, 1024, 512] { precisions.push((p, None)); }
    for p in [128u32, 256, 1024] { precisions.push((p, Some([0.0, 0.0]))); }
    let (mut ok, mut refused) = (0u64, 0u64);
    for present_mask in 0..16u8 {
        let present = [present_mask & 1 != 0, present_mask & 2 != 0, present_mask & 4 != 0, present_mask & 8 != 0];
        for &x in &EXTENTS { for &ef in &ES { for &em in &ES { for &(p, floor) in &precisions {
            for stop_mask in 0..16u8 { for hd in [false, true] {
                let stop = [stop_mask & 1 != 0, stop_mask & 2 != 0, stop_mask & 4 != 0, stop_mask & 8 != 0];
                let facts = CoverageFacts { present, extent_bits: x.to_bits(), resolution_bits: [ef.to_bits(), em.to_bits()],
                    floor_bits: floor.map(|f| [f[0].to_bits(), f[1].to_bits()]), precision: p };
                let r = rederive_coverage(CoverageBody { body: 1, stop, has_data: hd }, &facts);
                let valid_mode = matches!((p, floor), (128 | 256, None) | (512, Some(_)));
                let absent_stop = (0..4).any(|k| stop[k] && !present[k]);
                let floor_unforced = floor.is_some_and(|f| (0..2).any(|k| present[2 + k] && f[k] > 0.0 && !stop[2 + k]));
                let expect_refuse = !valid_mode || absent_stop || floor_unforced;
                match r {
                    Err(e) => { assert!(expect_refuse, "unexpected refusal"); assert_eq!(e.category(), "association"); refused += 1; }
                    Ok(c) => {
                        assert!(!expect_refuse, "missing refusal p={p} floor={floor:?} present={present:?} stop={stop:?}");
                        let mut hats = [ef > 0.0, em > 0.0];
                        if x != 0.0 { hats = [hats[0] || hats[1]; 2]; }
                        let estimate = [present[2] && hats[0], present[3] && hats[1]];
                        let charge = if p == 512 { [stop[2], stop[3]] } else { estimate };
                        assert_eq!(c, ProductSummaryCoverage { body: 1, stop, estimate, charge, has_data: hd });
                        ok += 1;
                    }
                }
            }}
        }}}}
    }
    println!("RV77_PAYLOAD_SPEC accepted={ok} refused={refused}");
}

#[test]
fn rv77_input_derived_force_moment_would_break_the_p512_identity() {
    // Shows the coverage_facts refusal of input-derived force/moment rows is
    // necessary: if such rows existed, native charge (present && positive) could
    // differ from stop at p512. recover::layout never produces them.
    let mut breaks = 0u64;
    for m2 in 0..16u8 { for m3 in 0..16u8 {
        let rows = rows_of([1, 1, m2, m3]);
        for &x in &EXTENTS { for f in [[0.0, 0.0], [3.0, 0.0]] {
            let native = native_model(0, &rows, x, [1.0, 1.0], Some(f), 512, true);
            if native.charge != [native.stop[2], native.stop[3]] { breaks += 1; }
        }}
    }}
    println!("RV77_INPUT_DERIVED_FM p512_identity_breaks={breaks}");
    assert!(breaks > 0);
}
}

// ---------------------------------------------------------------- B3-K (K3-3)
// The public `ProductMaterial::BaseENu` through the whole prepared proof
// (begin -> project -> complete_maxima -> certify_final), with SA-2's controls.
const B3K_D: u64 = 0x3fb999999999999a; // 0.1
const B3K_T: u64 = 0x3f747ae147ae147b; // 0.005
const B3K_E: f64 = 210e9;
type B3kRow = (String, ProductUnit, u32, ProductRecipe);
fn b3k_section() -> [f64; 5] {
    prepare_product_annulus(f64::from_bits(B3K_D), f64::from_bits(B3K_T)).result().unwrap().section_bits().values()
}
// Every row of this specimen certifies on the ordinary and both exact routes. Most
// nearby load sets lose one to ten rows to DEF-O's projection rounding (the same
// rows on every route), an availability limit outside B3-K (RV115 NC-1).
const B3K_LOADS: [f64; 6] = [26000.0, 326.0, -278.0, 63.0, 33.5, -56.0];
/// A cantilever, root fully restrained (support 3), tip loaded in all six
/// components; stations at 0.25, 0.5 and 0.75; the prepared annulus section.
fn b3k_source_loaded(g: f64, tip: [f64; 6]) -> super::super::super::source::PrimitiveSource {
    use super::super::super::source::*;
    let [a, i, j, _, _] = b3k_section();
    let loads: Vec<_> = Component::ALL.iter().zip(tip).filter(|(_, v)| *v != 0.0).map(|(&c, v)| (c, v)).collect();
    PrimitiveSource::new(SourceParts {
        nodes: vec![[0.0; 3], [1.0, 0.0, 0.0]],
        members: vec![StraightMember { id: 7, node_i: 0, node_j: 1, elastic_modulus: B3K_E, shear_modulus: g,
            area: a, second_moment_y: i, second_moment_z: i, torsion_constant: j, y_reference: [0.0, 1.0, 0.0] }],
        constraints: Component::ALL.iter().map(|&component| Constraint { dof: Dof { node: 0, component }, value: 0.0 }).collect(),
        loads: loads.iter().map(|&(component, value)| NodalLoad { dof: Dof { node: 1, component }, value,
            source_id: format!("tip-{component:?}") }).collect(),
        stations: [0.25, 0.5, 0.75].iter().enumerate()
            .map(|(k, &fraction)| Station { id: 17 + k as u32, member: 7, fraction }).collect(),
        supports: vec![SupportGroup { id: 3, node: 0, restrained: [true; 6], springs: vec![], directional_springs: vec![] }],
        ..SourceParts::default()
    }).unwrap()
}
fn b3k_facts(material: ProductMaterial) -> Vec<ProductMemberFacts> {
    let [a, i, j, z, _] = b3k_section();
    let d = f64::from_bits(B3K_D);
    vec![ProductMemberFacts { member: 7, diameter: d, effective_wall: f64::from_bits(B3K_T), material,
        area: a, second_moment: i, torsion_constant: j, section_modulus: z, radius: d / 2.0 }]
}
/// The product's complete case row set: every native row (reactions through
/// their support components), 6 support components per group, 20 stress
/// slots and the circular maximum per member, and the mode record.
fn b3k_rows(owner: &adaptive::RetainedSolve, maxima: bool, mode: bool) -> Vec<B3kRow> {
    let source = owner.source();
    let mut out = Vec::new();
    for r in &owner.publish().rows {
        if matches!(r.id, QuantityId::Reaction(_)) { continue; }
        let unit = match r.kind { Kind::Translation => ProductUnit::Millimetre, Kind::Rotation => ProductUnit::Radian,
            Kind::Force => ProductUnit::Newton, Kind::Moment => ProductUnit::NewtonMetre };
        out.push((format!("{:?}", r.id), unit, r.body, ProductRecipe::Native(r.id)));
    }
    for g in source.supports() {
        for component in Component::ALL {
            let unit = if component.index() < 3 { ProductUnit::Newton } else { ProductUnit::NewtonMetre };
            out.push((format!("support-{}-{component:?}", g.id), unit, source.body_of_node(g.node),
                ProductRecipe::SupportComponent { support: g.id, component }));
        }
    }
    for m in source.members() {
        let body = source.body_of_node(m.node_i);
        let mut sites = vec![ProductSite::End(End::I), ProductSite::End(End::J)];
        sites.extend(source.stations().iter().filter(|s| s.member == m.id).map(|s| ProductSite::Station(s.id)));
        for site in sites {
            for stress in [ProductStress::Axial, ProductStress::BendingY, ProductStress::BendingZ, ProductStress::Torsion] {
                out.push((format!("stress-{}-{site:?}-{stress:?}", m.id), ProductUnit::Megapascal, body,
                    ProductRecipe::Stress { member: m.id, site, stress }));
            }
        }
        if maxima {
            out.push((format!("maximum-{}", m.id), ProductUnit::Pascal, body, ProductRecipe::CircularMaximum { member: m.id }));
        }
    }
    if mode { out.push(("mode".into(), ProductUnit::Record, 0, ProductRecipe::NonQuantity)); }
    out
}
fn b3k_specs<'a>(rows: &'a [B3kRow], case_id: &'a str) -> Vec<ProductRowSpec<'a>> {
    rows.iter().map(|(id, unit, body, recipe)| match recipe {
        ProductRecipe::NonQuantity => ProductRowSpec::mode(id, case_id, *body, 1).unwrap(),
        _ => ProductRowSpec::mechanical(id, case_id, *unit, *body, *recipe).unwrap(),
    }).collect()
}
/// The circular maximum the way the kernel projects any row: the RN64 of the
/// RN1024 midpoint of the hull of both lanes' recipe enclosures.
fn b3k_maxima(p: &ProjectedProofDraft<'_, '_>) -> Result<Vec<ProductMaximumValue>, ProductFailure> {
    let mut out = Vec::new();
    for (i, spec) in p.data.specs.iter().enumerate() {
        let ProductRecipe::CircularMaximum { member } = spec.recipe else { continue };
        let mut w = ProductCertificateSpent::new(&[]);
        w.projection_outcomes = w.prepared_reserve(5, 1)?;
        let section = section_for(p.data.owner, p.data.facts, spec.recipe, &mut w)?;
        let k = recipe(p.data.owner, &mut w, &p.data.k.rows, spec.recipe, section.as_ref(), p.data.facts, true)?;
        let s = recipe(p.data.owner, &mut w, &p.data.source.rows, spec.recipe, section.as_ref(), p.data.facts, false)?;
        out.push(ProductMaximumValue::new(member, i, project_hull(hull(k, s), ProductUnit::Pascal, i, &mut w)?)?);
    }
    Ok(out)
}
enum B3kOutcome { Certified(CertifiedProductProof, Vec<f64>), Refused(ProductProofFailure) }
fn b3k_prove(invocation: &super::super::super::origins::RecordedInvocation, run: usize, owner: &adaptive::RetainedSolve,
    facts: &[ProductMemberFacts], specs: &[ProductRowSpec<'_>]) -> B3kOutcome {
    let draft = match invocation.begin_prepared_product(run, owner, facts, specs).into_ready() {
        Ok(v) => v, Err(e) => return B3kOutcome::Refused(e) };
    let (projected, builder) = match draft.project().into_ready() { Ok(v) => v, Err(e) => return B3kOutcome::Refused(e) };
    let maxima = b3k_maxima(&projected).unwrap();
    let (values, work) = builder.complete_maxima(&maxima).into_ready().unwrap();
    let rows: Vec<_> = specs.iter().enumerate().map(|(i, s)| s.row(values.value(i).unwrap())).collect();
    let frozen: Vec<f64> = (0..values.len()).map(|i| *values.value(i).unwrap()).collect();
    match projected.certify_final(&values, &rows, work).into_ready() {
        Ok(v) => B3kOutcome::Certified(v, frozen), Err(e) => B3kOutcome::Refused(e) }
}
fn b3k_case(g: f64) -> (super::super::super::origins::RecordedInvocation, super::super::super::origins::RecordedCase) {
    b3k_case_loaded(g, B3K_LOADS)
}
fn b3k_case_loaded(g: f64, tip: [f64; 6]) -> (super::super::super::origins::RecordedInvocation, super::super::super::origins::RecordedCase) {
    use super::super::super::origins::*;
    use crate::structural::retained_api::CaseLimit;
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let mut cases = invocation.solve_cases(&[b3k_source_loaded(g, tip)], CaseLimit::new(u64::MAX)).unwrap();
    (invocation, cases.remove(0))
}
fn b3k_owner(case: &super::super::super::origins::RecordedCase) -> &adaptive::RetainedSolve {
    match &case.outcome {
        adaptive::ExecutionOutcome::Selected(v) => v,
        other => panic!("native specimen did not select: {other:?}"),
    }
}
fn b3k_certified(material: ProductMaterial, g: f64) -> (Vec<ProductRowVerdict>, Vec<f64>, Vec<B3kRow>) {
    let (invocation, case) = b3k_case(g);
    let (run, owner) = (case.run, b3k_owner(&case));
    let rows = b3k_rows(owner, true, true);
    let specs = b3k_specs(&rows, "case");
    match b3k_prove(&invocation, run, owner, &b3k_facts(material), &specs) {
        B3kOutcome::Certified(proof, values) => {
            assert!(proof.passed());
            let mut copies = TraceCopyWork::default();
            let trace = proof.work().typed_trace(&mut copies);
            for lane in &trace.lanes { assert!(lane.as_ref().unwrap().result.is_ok(), "both lanes"); }
            assert_eq!(trace.lanes[0].as_ref().unwrap().law, source_residual::ReadoutLaw::AdmittedK);
            assert_eq!(trace.lanes[1].as_ref().unwrap().law, source_residual::ReadoutLaw::AnnularSource);
            (proof.verdicts().to_vec(), values, rows)
        }
        B3kOutcome::Refused(e) => panic!("{material:?}: {:?} verdicts={:?}", e.failure(),
            e.work().verdicts().iter().filter(|v| !v.passed).collect::<Vec<_>>()),
    }
}
#[test]
fn b3k_base_e_nu_maps_to_exact_operands_and_refuses_through_the_public_variant() {
    let MaterialOperands::ExactENu { e, nu } = (ProductMaterial::BaseENu { e: B3K_E, nu: 0.3125 }).operands() else {
        panic!("K3-1: BaseENu is the exact E/nu route") };
    assert_eq!((e.to_bits(), nu.to_bits()), (B3K_E.to_bits(), 0.3125f64.to_bits()));
    // NA-4: ProductMaterial keeps its 96 bytes (Interpolated governs; no niche).
    assert_eq!(std::mem::size_of::<ProductMaterial>(), 96);
    assert_eq!(std::mem::size_of::<ProductMemberFacts>(), 160);
    let [a, i, j, z, _] = b3k_section();
    let member = |material: ProductMaterial| member_coefficients(&MemberOperands {
        diameter: f64::from_bits(B3K_D), effective_wall: f64::from_bits(B3K_T), material: material.operands(),
        admitted: AdmittedOperands { e: B3K_E, g: 80e9, a, j, iz: i, iy: i, z_hat: z } });
    assert!(member(ProductMaterial::BaseENu { e: B3K_E, nu: 0.3125 }).result().is_ok());
    let e_up = f64::from_bits(B3K_E.to_bits() + 1);
    assert_eq!(member(ProductMaterial::BaseENu { e: e_up, nu: 0.3125 }).result().unwrap_err(), NumericError::MaterialBits);
    for nu in [-1.0, 0.5, -1.5, 0.75, f64::from_bits(0.5f64.to_bits() + 1)] {
        assert_eq!(member(ProductMaterial::BaseENu { e: B3K_E, nu }).result().unwrap_err(), NumericError::InvalidMaterial, "nu={nu}");
    }
    for nu in [f64::from_bits((-1.0f64).to_bits() - 1), f64::from_bits(0.5f64.to_bits() - 1), 0.0, -0.0] {
        assert!(member(ProductMaterial::BaseENu { e: B3K_E, nu }).result().is_ok(), "nu={nu}");
    }
    // Through the whole prepared proof: an E-bit mismatch refuses in the lanes.
    let (invocation, case) = b3k_case(80e9);
    let (run, owner) = (case.run, b3k_owner(&case));
    let rows = b3k_rows(owner, true, true);
    let specs = b3k_specs(&rows, "case");
    match b3k_prove(&invocation, run, owner, &b3k_facts(ProductMaterial::BaseENu { e: e_up, nu: 0.3125 }), &specs) {
        B3kOutcome::Refused(e) => assert_eq!(e.failure().category(), "native_source"),
        B3kOutcome::Certified(..) => panic!("E-bit mismatch certified"),
    }
}
#[test]
fn b3k_base_e_nu_stress_and_maximum_rows_certify_in_both_lanes_nu_0_3125_control() {
    // NA-6: with E = 210e9 and nu = 0.3125, G = E/(2(1 + nu)) = 80e9 exactly, so
    // the exact route must certify exactly as the ordinary route with G = 80e9:
    // the same frozen values and the same verdicts, bit for bit.
    let (exact, exact_values, rows) = b3k_certified(ProductMaterial::BaseENu { e: B3K_E, nu: 0.3125 }, 80e9);
    let (ordinary, ordinary_values, _) = b3k_certified(ProductMaterial::Base { e: B3K_E, g: 80e9 }, 80e9);
    assert_eq!(exact_values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), ordinary_values.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    assert_eq!(exact.len(), rows.len());
    for (x, o) in exact.iter().zip(&ordinary) {
        assert_eq!((x.row, x.normalized_bits, x.scale_bits, x.class, x.passed, x.predicates),
            (o.row, o.normalized_bits, o.scale_bits, o.class, o.passed, o.predicates));
    }
    let stress = rows.iter().filter(|r| matches!(r.3, ProductRecipe::Stress { .. })).count();
    let maximum = rows.iter().filter(|r| matches!(r.3, ProductRecipe::CircularMaximum { .. })).count();
    assert_eq!((stress, maximum), (20, 1));
    for (v, r) in exact.iter().zip(&rows) {
        if matches!(r.3, ProductRecipe::Stress { .. } | ProductRecipe::CircularMaximum { .. }) {
            assert!(v.passed && v.class.is_some(), "{}", r.0);
            if matches!(r.3, ProductRecipe::Stress { stress: ProductStress::BendingY | ProductStress::BendingZ, site: ProductSite::End(End::I), .. }) {
                assert_ne!(exact_values[v.row], 0.0, "{}: a loaded bending row", r.0);
            }
        }
    }
    // nu = 0.3: G-hat = RN64(E/(2(1 + nu))) differs from the exact G; the exact
    // route still certifies its stress and maximum rows in both lanes.
    let nu = 0.3;
    let g_hat = B3K_E / (2.0 * (1.0 + nu));
    let (verdicts, _, rows) = b3k_certified(ProductMaterial::BaseENu { e: B3K_E, nu }, g_hat);
    assert!(verdicts.iter().zip(&rows).all(|(v, _)| v.passed));
}
#[test]
fn b3k_restored_material_gate_fails_stress_and_maximum_rows_with_represented_z() {
    // SA-2: the test-only mutation restoring the pre-K3-2 material gate makes
    // every BaseENu stress and maximum row fail with bad("represented Z"); with
    // K3-2 both certify (the test above). The ordinary route is unaffected.
    let (invocation, case) = b3k_case(80e9);
    let (run, owner) = (case.run, b3k_owner(&case));
    let rows = b3k_rows(owner, true, true);
    let specs = b3k_specs(&rows, "case");
    let facts = b3k_facts(ProductMaterial::BaseENu { e: B3K_E, nu: 0.3125 });
    let association = |f: &ProductFailure| match &f.cause { Cause::Association(s) => *s, other => panic!("{other:?}") };
    super::super::hooks::set_material_gate(true);
    let gated = b3k_prove(&invocation, run, owner, &facts, &specs);
    let ordinary = b3k_prove(&invocation, run, owner, &b3k_facts(ProductMaterial::Base { e: B3K_E, g: 80e9 }), &specs);
    super::super::hooks::set_material_gate(false);
    match gated {
        B3kOutcome::Refused(e) => assert_eq!(association(e.failure()), "represented Z"),
        B3kOutcome::Certified(..) => panic!("gated BaseENu certified"),
    }
    assert!(matches!(ordinary, B3kOutcome::Certified(ref p, _) if p.passed()));
    // Each row family directly: the represented (K) recipe needs represented Z;
    // the geometric recipe does not.
    let draft = invocation.begin_prepared_product(run, owner, &facts, &specs).into_ready().unwrap();
    let (projected, _builder) = draft.project().into_ready().unwrap();
    for (i, r) in rows.iter().enumerate() {
        if !matches!(r.3, ProductRecipe::Stress { .. } | ProductRecipe::CircularMaximum { .. }) { continue; }
        for gate in [false, true] {
            super::super::hooks::set_material_gate(gate);
            let mut w = ProductCertificateSpent::new(&[]);
            let section = section_for(owner, &facts, r.3, &mut w).unwrap();
            let k = recipe(owner, &mut w, &projected.data.k.rows, r.3, section.as_ref(), &facts, true);
            let s = recipe(owner, &mut w, &projected.data.source.rows, r.3, section.as_ref(), &facts, false);
            super::super::hooks::set_material_gate(false);
            assert!(s.is_ok(), "{} geometric", r.0);
            if gate { assert_eq!(association(&k.unwrap_err()), "represented Z", "{} row {i}", r.0); }
            else { assert!(k.is_ok(), "{}", r.0); }
        }
    }
}

// ---------------------------------------------------------------- B2-K
// The combination specimen (I94 KD §6, with RV115's SF-2, SF-3 and N-9): one
// cantilever, eight operand cases, C1-C9 through the recorded source API, and
// the independent oracle's exact targets (product_certificate_vectors.py).
#[path = "product_certificate_combination_vectors.rs"]
mod b2k_oracle;
use super::super::super::origins::{NativeOwner, OriginCapacity, RecordedCase, RecordedInvocation,
    RecordedKernelCombination, RecordedOperand};
const B2K_D: f64 = 0.1;
const B2K_T: f64 = 0.005;
const B2K_E: f64 = 210e9;
const B2K_G: f64 = 80e9;
fn b2k_parse(token: &str) -> Endpoint {
    if token == "Z+" { return Endpoint::ZERO; }
    let (hex, e) = token[1..].split_once('p').unwrap();
    let digits = format!("{hex:0<256}");
    let mut limbs = [0; 16];
    for (i, v) in limbs.iter_mut().rev().enumerate() {
        *v = u64::from_str_radix(&digits[i * 16..i * 16 + 16], 16).unwrap();
    }
    Endpoint::from_parts(token.starts_with('-'), e.parse().unwrap(), limbs).unwrap()
}
/// The oracle's token form of an endpoint (the inverse of `b2k_parse`).
fn b2k_token(x: &Endpoint) -> String {
    if x.is_zero() { return "Z+".into(); }
    let (negative, exponent, limbs) = x.parts();
    let hex: String = limbs.iter().rev().map(|v| format!("{v:016x}")).collect();
    format!("{}{}p{exponent}", if negative { '-' } else { '+' }, hex.trim_end_matches('0'))
}
fn b2k_source_parts(name: &str) -> super::super::super::source::SourceParts {
    use super::super::super::source::*;
    let [a, i, j, _, _] = b2k_oracle::SECTION.map(f64::from_bits);
    let terms = b2k_oracle::OPERANDS.iter().find(|o| o.0 == name).unwrap().1;
    SourceParts {
        nodes: vec![[0.0; 3], [1.0, 0.0, 0.0]],
        members: vec![StraightMember { id: 7, node_i: 0, node_j: 1, elastic_modulus: B2K_E, shear_modulus: B2K_G,
            area: a, second_moment_y: i, second_moment_z: i, torsion_constant: j, y_reference: [0.0, 1.0, 0.0] }],
        constraints: Component::ALL.iter().map(|&component| Constraint { dof: Dof { node: 0, component }, value: 0.0 }).collect(),
        // source ids independent of the operand's name: A and A2 have equal K4SRC bytes.
        loads: terms.iter().enumerate().map(|(k, &(node, c, v))| NodalLoad {
            dof: Dof { node, component: Component::from_index(c as usize) }, value: f64::from_bits(v),
            source_id: format!("t{k}") }).collect(),
        stations: [0.25, 0.5, 0.75].iter().enumerate()
            .map(|(k, &fraction)| Station { id: 17 + k as u32, member: 7, fraction }).collect(),
        supports: vec![SupportGroup { id: 3, node: 0, restrained: [true; 6], springs: vec![], directional_springs: vec![] }],
        ..SourceParts::default()
    }
}
fn b2k_source(name: &str) -> super::super::super::source::PrimitiveSource {
    super::super::super::source::PrimitiveSource::new(b2k_source_parts(name)).unwrap()
}
fn b2k_facts() -> Vec<ProductMemberFacts> {
    let [a, i, j, z, _] = b2k_oracle::SECTION.map(f64::from_bits);
    vec![ProductMemberFacts { member: 7, diameter: B2K_D, effective_wall: B2K_T,
        material: ProductMaterial::Base { e: B2K_E, g: B2K_G }, area: a, second_moment: i,
        torsion_constant: j, section_modulus: z, radius: B2K_D / 2.0 }]
}
fn b2k_laws(owner: &adaptive::RetainedSolve) -> [bridge::ProposedMemberLaw<'_>; 1] {
    [bridge::ProposedMemberLaw { member: &owner.source().members()[0], diameter: B2K_D, effective_wall: B2K_T,
        material: MaterialOperands::Ordinary { e: B2K_E, g: B2K_G },
        represented_z: f64::from_bits(b2k_oracle::SECTION[3]) }]
}
struct B2kSpecimen {
    invocation: RecordedInvocation,
    cases: Vec<RecordedCase>,
    prepared: Vec<(&'static str, usize, adaptive::PreparedCaseSource)>,
    combinations: Vec<(&'static str, RecordedKernelCombination)>,
}
fn b2k_selected(case: &RecordedCase) -> &adaptive::RetainedSolve {
    match &case.outcome { adaptive::ExecutionOutcome::Selected(s) => s, other => panic!("{other:?}") }
}
impl B2kSpecimen {
    fn case(&self, name: &str) -> &adaptive::RetainedSolve {
        b2k_selected(&self.cases[b2k_oracle::OPERANDS.iter().position(|o| o.0 == name).unwrap()])
    }
    fn combination(&self, name: &str) -> (usize, &adaptive::RetainedSolve) {
        match &self.combinations.iter().find(|c| c.0 == name).unwrap().1 {
            RecordedKernelCombination::WithRun { case, .. } => (case.run, b2k_selected(case)),
            other => panic!("{name}: {other:?}"),
        }
    }
}
/// The eight operand cases in one batch, the prepared operands registered after
/// it (C3a's placement), then each named combination's recorded Call.
fn b2k_specimen(names: &[&str]) -> B2kSpecimen {
    use crate::structural::retained_api::CaseLimit;
    let sources: Vec<_> = b2k_oracle::OPERANDS.iter().map(|o| b2k_source(o.0)).collect();
    let combos: Vec<_> = b2k_oracle::COMBINATIONS.iter().filter(|c| names.contains(&c.0)).collect();
    let mut prepared_names: Vec<&'static str> = Vec::new();
    for c in &combos {
        for &(_, name, prepared) in c.1 {
            if prepared && !prepared_names.contains(&name) { prepared_names.push(name); }
        }
    }
    let lengths: Vec<usize> = combos.iter().map(|c| c.1.len()).collect();
    let capacity = OriginCapacity::for_invocation(&[sources.len()], &lengths, prepared_names.len()).unwrap();
    let mut invocation = RecordedInvocation::new(u64::MAX, capacity).unwrap();
    let cases = invocation.solve_cases(&sources, CaseLimit::new(u64::MAX)).unwrap();
    let mut prepared = Vec::new();
    for name in prepared_names {
        let p = adaptive::PreparedCaseSource::new(b2k_source(name)).unwrap();
        let id = invocation.register_prepared_source(&p).unwrap();
        prepared.push((name, id, p));
    }
    let mut combinations = Vec::new();
    for c in combos {
        let operands: Vec<(f64, RecordedOperand<'_>)> = c.1.iter().map(|&(f, name, is_prepared)| {
            let operand = if is_prepared {
                let (_, source, p) = prepared.iter().find(|x| x.0 == name).unwrap();
                RecordedOperand::Prepared { source: *source, prepared: p }
            } else {
                RecordedOperand::Selected(b2k_selected(&cases[b2k_oracle::OPERANDS.iter().position(|o| o.0 == name).unwrap()]))
            };
            (f64::from_bits(f), operand)
        }).collect();
        let out = invocation.solve_combination_sources(&operands, CaseLimit::new(u64::MAX)).unwrap();
        combinations.push((c.0, out));
    }
    B2kSpecimen { invocation, cases, prepared, combinations }
}
const B2K_ALL: [&str; 9] = ["C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9"];
fn b2k_key(id: QuantityId) -> String {
    let e = |end: End| if end == End::I { "I" } else { "J" };
    match id {
        QuantityId::Displacement(d) => format!("u{}.{}", d.node, d.component.index()),
        QuantityId::DisplacementMagnitude(n) => format!("m{n}"),
        QuantityId::EndAction { member, end, component } => format!("e{member}.{}.{}", e(end), component.index()),
        QuantityId::StationAction { station, component } => format!("s{station}.{}", component.index()),
        QuantityId::Reaction(d) => format!("r{}.{}", d.node, d.component.index()),
        QuantityId::SupportForceMagnitude(s) => format!("sf{s}"),
        QuantityId::SupportMomentMagnitude(s) => format!("sm{s}"),
        other => panic!("not in the specimen: {other:?}"),
    }
}
fn b2k_recipe_key(r: ProductRecipe) -> String {
    match r {
        ProductRecipe::Native(id) => b2k_key(id),
        ProductRecipe::SupportComponent { component, .. } => format!("r0.{}", component.index()),
        ProductRecipe::Stress { member, site, stress } => format!("z{member}.{}.{}", match site {
            ProductSite::End(End::I) => "I".to_string(), ProductSite::End(End::J) => "J".to_string(),
            ProductSite::Station(s) => s.to_string() }, stress as usize),
        other => panic!("{other:?}"),
    }
}
/// [K-law truth, G-law truth] of a row, outward 1024-bit tokens.
fn b2k_truth(combination: &str, key: &str) -> [(Endpoint, Endpoint); 2] {
    let c = b2k_oracle::COMBINATIONS.iter().find(|c| c.0 == combination).unwrap();
    let r = c.4.iter().find(|r| r.0 == key).unwrap_or_else(|| panic!("{combination}: no truth for {key}"));
    [(b2k_parse(r.1 .0), b2k_parse(r.1 .1)), (b2k_parse(r.2 .0), b2k_parse(r.2 .1))]
}
fn b2k_contains(e: &Enclosure, t: &(Endpoint, Endpoint)) -> bool {
    e.lo.cmp_value(&t.0) != Ordering::Greater && e.hi.cmp_value(&t.1) != Ordering::Less
}
fn b2k_lanes<'a>(owner: &'a adaptive::RetainedSolve, laws: &'a [bridge::ProposedMemberLaw<'a>])
    -> [source_residual::ResidualSpent<'a>; 2] {
    [source_residual::ReadoutLaw::AdmittedK, source_residual::ReadoutLaw::AnnularSource].map(|law|
        source_residual::source_residual_for_law(owner, owner.source(), &owner.evidence().source_encoding,
            owner.selected_precision(), laws, law))
}
#[test]
fn b2k_k08_residual_and_recovery_contain_the_oracle_and_nets_are_exact_outward() {
    let s = b2k_specimen(&B2K_ALL);
    // The K-law section is the correctly rounded annulus: FK's own preparation agrees.
    assert_eq!(prepare_product_annulus(B2K_D, B2K_T).result().unwrap().section_bits().bits(), b2k_oracle::SECTION);
    for c in b2k_oracle::COMBINATIONS {
        let (_, owner) = s.combination(c.0);
        // C03 and the ledger: the combined ledger's K4LED bytes are the oracle's.
        let hex: String = owner.prep.ledger.encoding().iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex, c.2, "{}: K4LED", c.0);
        assert_eq!(owner.evidence().ledger_encoding, owner.prep.ledger.encoding());
        // Each nonzero net: [RD1024, RU1024] exactly; nearest rounding (SF-4's
        // control, test-only) gives one point.
        for &(g, rd, ru, rn) in c.3 {
            let mut w = NumericWork::new();
            let net = w.ledger_net(&owner.prep.ledger, g as usize).unwrap();
            assert_eq!((net.lo, net.hi), (b2k_parse(rd), b2k_parse(ru)), "{} net {g}", c.0);
            assert_eq!(w.entries[Entry::Add as usize].exact(), Ok(2));
            assert!(w.status().is_exact());
            super::super::hooks::set_nearest_net(true);
            let mut w = NumericWork::new();
            let nearest = w.ledger_net(&owner.prep.ledger, g as usize).unwrap();
            super::super::hooks::set_nearest_net(false);
            assert_eq!((nearest.lo, nearest.hi), (b2k_parse(rn), b2k_parse(rn)));
        }
        let laws = b2k_laws(owner);
        let lanes = b2k_lanes(owner, &laws);
        for (lane, spent) in lanes.iter().enumerate() {
            let native = spent.result().unwrap_or_else(|e| panic!("{} lane {lane}: {e:?}", c.0));
            assert_eq!(native.rows.len(), owner.publish().rows.len());
            for (i, row) in owner.publish().rows.iter().enumerate() {
                let key = b2k_key(row.id);
                assert!(b2k_contains(&native.rows[i], &b2k_truth(c.0, &key)[lane]),
                    "{} lane {lane} {key}: {:?}", c.0, native.rows[i]);
            }
        }
        // Both load branches alone: two Add entries per nonzero net, plus the
        // interval add (free) or subtract (reaction); exact-zero nets add nothing.
        let view = owner.source_bridge_view(owner.source(), &owner.evidence().source_encoding, owner.selected_precision()).result.unwrap();
        let (free, reaction, w) = source_residual::test_combination_loads(&view).unwrap();
        let ordering = &owner.group.ordering;
        let nonzero = |g: usize| owner.prep.ledger.net(g).is_some_and(|n| !n.is_zero());
        let kf = ordering.free.iter().filter(|&&g| nonzero(g)).count() as u64;
        let kc = owner.source().constraints().iter().filter(|x| nonzero(x.dof.global())).count() as u64;
        assert_eq!(w.numeric.entries[Entry::Add as usize].exact(), Ok(4 * kf + 2 * kc), "{}", c.0);
        assert_eq!(w.numeric.entries[Entry::Sub as usize].exact(), Ok(2 * kc), "{}", c.0);
        assert_eq!(w.visits.exact(), Ok((ordering.free.len() + owner.source().constraints().len()) as u64));
        for (a, &g) in ordering.free.iter().enumerate() {
            if !nonzero(g) { assert!(free[a].lo.is_zero() && free[a].hi.is_zero(), "{} free {g}", c.0); }
        }
        for (ci, x) in owner.source().constraints().iter().enumerate() {
            if !nonzero(x.dof.global()) { assert!(reaction[ci].lo.is_zero() && reaction[ci].hi.is_zero()); }
        }
    }
    // C6 (SF-4): 2^600 + 2^-600 needs 1201 bits; its outward enclosure is
    // nondegenerate, and the nearest 1024-bit point is 2^600, below the net.
    let c6 = b2k_oracle::COMBINATIONS.iter().find(|c| c.0 == "C6").unwrap();
    let (_, rd, ru, rn) = c6.3[0];
    assert!(b2k_parse(rd).cmp_value(&b2k_parse(ru)) == Ordering::Less);
    assert_eq!(b2k_parse(rn), b2k_parse(rd));
    assert_eq!(b2k_parse(rn), shift(&Endpoint::ONE, 600).unwrap());
}
#[test]
fn b2k_k07_view_admits_p2_refuses_corruption_and_keeps_native_data_flags() {
    let s = b2k_specimen(&B2K_ALL);
    for name in B2K_ALL {
        let (_, owner) = s.combination(name);
        let spent = owner.source_bridge_view(owner.source(), &owner.evidence().source_encoding, owner.selected_precision());
        let view = spent.result.unwrap_or_else(|e| panic!("{name}: {e:?}"));
        // The view's data flags are the native verification's own (count of data blocks).
        let attempt = owner.evidence().attempts.iter()
            .find(|a| a.precision == owner.evidence().verification_precision && a.verification.is_some()).unwrap();
        assert_eq!(view.data().iter().filter(|d| **d).count(), attempt.verification.as_ref().unwrap().data_blocks, "{name}");
    }
    // A cancelled combination (A - A2: every net exactly 0, every product
    // nonzero) keeps its free block as data.
    let mut cancelled = b2k_specimen(&[]);
    let (a, a2) = (cancelled.case("A").clone(), cancelled.case("A2").clone());
    let out = cancelled.invocation.solve_combination(&[(1.0, &a), (-1.0, &a2)], adaptive::CaseLimit::new(u64::MAX));
    assert!(matches!(out, Err(_)), "no combination was declared: capacity refuses");
    let combined = match super::super::super::combine::RetainedCombination::solve(&[(1.0, &a), (-1.0, &a2)],
        adaptive::CaseLimit::new(u64::MAX), &mut adaptive::InvocationMeter::new(u64::MAX)) {
        super::super::super::combine::CombinationOutcome::Selected(s) => s, other => panic!("{other:?}") };
    let view = combined.source_bridge_view(combined.source(), &combined.evidence().source_encoding, combined.selected_precision()).result.unwrap();
    assert_eq!(view.data(), &[true]);
    // Corruption of the combined prescribed terms: factor bits or term count
    // -> PairIdentity; a nonzero (or -0.0, N-3) value -> UnsupportedCombination.
    // Visits are booked before each check (N-8): the refusal's prefix grows by
    // one per term examined.
    let (_, owner) = s.combination("C1");
    let corrupt = |f: &dyn Fn(&mut Vec<(usize, Vec<(f64, f64)>)>)| {
        let mut changed = owner.clone();
        let p = &owner.prep;
        let mut prescribed = p.prescribed.clone();
        f(&mut prescribed);
        changed.prep = std::sync::Arc::new(adaptive::CasePrep { source: p.source.clone(), ledger: p.ledger.clone(),
            prescribed, factors: p.factors.clone(), identity: p.identity.clone(), layout: p.layout.clone(),
            extents: p.extents.clone() });
        let spent = changed.source_bridge_view(changed.source(), &changed.evidence().source_encoding, changed.selected_precision());
        (spent.result.map(|_| ()), spent.work.visits.exact().unwrap())
    };
    use adaptive::{CertificateIssue, SourceBridgeViewIssue as V};
    // The first constrained DOF's own check refuses after its one visit (vd).
    let (rd, vd) = corrupt(&|p| p[0].0 = 99);
    assert_eq!(rd, Err(V::Certificate(CertificateIssue::PairIdentity)));
    let (r0, v0) = corrupt(&|p| p[0].1[0].0 = 2.0);
    assert_eq!(r0, Err(V::Certificate(CertificateIssue::PairIdentity)));
    assert_eq!(v0, vd + 1, "the term's visit is booked before its check");
    let (r1, v1) = corrupt(&|p| p[0].1[2].1 = 1e-3);
    assert_eq!(r1, Err(V::UnsupportedCombination));
    assert_eq!(v1, vd + 3, "visits booked per term before its check");
    let (r2, v2) = corrupt(&|p| p[0].1[0].1 = -0.0);
    assert_eq!(r2, Err(V::UnsupportedCombination));
    assert_eq!(v2, vd + 1);
    let (r3, _) = corrupt(&|p| { p[1].1.pop(); });
    assert_eq!(r3, Err(V::Certificate(CertificateIssue::PairIdentity)));
    let (r4, _) = corrupt(&|p| p[5].1[1].1 = -1.0);
    assert_eq!(r4, Err(V::UnsupportedCombination));
    let (r5, v5) = corrupt(&|_| {});
    assert!(r5.is_ok() && v5 > v1);
}
#[test]
fn b2k_c06_a_nonzero_prescription_in_a_later_operand_is_unsupported() {
    use crate::structural::retained_api::CaseLimit;
    // Operand 0 (A) has zero prescriptions; operand 1 (A with root Ux = 1e-3)
    // does not. The view never reads operand 0's value as the combined value.
    let mut parts = b2k_source_parts("A");
    parts.constraints[0].value = 1e-3;
    let sources = [b2k_source("A"), super::super::super::source::PrimitiveSource::new(parts).unwrap()];
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[2], &[2]).unwrap()).unwrap();
    let cases = invocation.solve_cases(&sources, CaseLimit::new(u64::MAX)).unwrap();
    let operands = [(1.0, b2k_selected(&cases[0])), (1.0, b2k_selected(&cases[1]))];
    let RecordedKernelCombination::WithRun { case, .. } = invocation.solve_combination(&operands, CaseLimit::new(u64::MAX)).unwrap()
        else { panic!() };
    let owner = b2k_selected(&case);
    assert_eq!(owner.prep.source.constraint(0), Some(0.0), "the representative's own value is zero");
    let spent = owner.source_bridge_view(owner.source(), &owner.evidence().source_encoding, owner.selected_precision());
    assert!(matches!(spent.result, Err(adaptive::SourceBridgeViewIssue::UnsupportedCombination)));
    let laws = b2k_laws(owner);
    for lane in b2k_lanes(owner, &laws) {
        assert!(matches!(lane.result(), Err(bridge::BridgeError::View(adaptive::SourceBridgeViewIssue::UnsupportedCombination))));
        assert_eq!(lane.work.correction.calls.exact(), Ok(0));
    }
    let facts = b2k_facts();
    let rows = b3k_rows(owner, false, false);
    let specs = b3k_specs(&rows, "combination");
    let failure = invocation.begin_prepared_product(case.run, owner, &facts, &specs).into_ready().err().unwrap();
    assert_eq!(failure.failure().category(), "native_source");
}
#[test]
fn b2k_k12_the_i42_bridge_holds_for_a_p2_combination_owner() {
    let s = b2k_specimen(&B2K_ALL);
    for name in B2K_ALL {
        let (_, owner) = s.combination(name);
        let laws = b2k_laws(owner);
        let spent = bridge::source_bridge(owner, owner.source(), &owner.evidence().source_encoding, owner.selected_precision(), &laws);
        let native = spent.result().unwrap_or_else(|e| panic!("{name}: {e:?}"));
        for (i, row) in owner.publish().rows.iter().enumerate() {
            let key = b2k_key(row.id);
            assert!(b2k_contains(&native.rows[i].source, &b2k_truth(name, &key)[1]), "{name} bridge {key}");
        }
    }
}
#[test]
fn b2k_k10_mutation_controls_fail_containment() {
    let s = b2k_specimen(&B2K_ALL);
    let failing = |name: &str, lane: usize| -> Vec<String> {
        let (_, owner) = s.combination(name);
        let laws = b2k_laws(owner);
        let spent = &b2k_lanes(owner, &laws)[lane];
        let native = spent.result().unwrap();
        owner.publish().rows.iter().enumerate().filter_map(|(i, row)| {
            let key = b2k_key(row.id);
            (!b2k_contains(&native.rows[i], &b2k_truth(name, &key)[lane])).then_some(key)
        }).collect()
    };
    // (a) The representative's loads at the free rows instead of the ledger:
    // containment fails, and the K-lane tip motions enclose the oracle's
    // response to operand 0's loads alone (KD §6 item 4 (ii)).
    super::super::source_residual::hooks::set_representative(true, false);
    for name in ["C1", "C2", "C4"] {
        let bad = failing(name, 0);
        assert!(bad.iter().any(|k| k.starts_with("u1.")), "{name}: {bad:?}");
        let (_, owner) = s.combination(name);
        let laws = b2k_laws(owner);
        let native = b2k_lanes(owner, &laws)[0].result().unwrap().rows.clone();
        let rep = b2k_oracle::REPRESENTATIVE_TIP_K.iter().find(|r| r.0 == name).unwrap();
        for c in 0..6 {
            let i = owner.publish().rows.iter().position(|r| b2k_key(r.id) == format!("u1.{c}")).unwrap();
            assert!(b2k_contains(&native[i], &(b2k_parse(rep.1[c].0), b2k_parse(rep.1[c].1))), "{name} rep u1.{c}");
        }
        assert!(!failing(name, 1).is_empty(), "{name}: source lane");
    }
    // (b) SF-2: the representative's loads at the constrained DOFs: C7's root
    // terms (operands R and R2; A has none there) are lost from the reactions.
    super::super::source_residual::hooks::set_representative(false, true);
    for lane in 0..2 {
        let bad = failing("C7", lane);
        assert!(bad.contains(&"r0.1".to_string()) && bad.iter().all(|k| k.starts_with("r0.") || k.starts_with("sf")), "C7 lane {lane}: {bad:?}");
    }
    // The free-row mutation does not touch C7's reactions, and vice versa.
    assert!(failing("C1", 0).is_empty());
    super::super::source_residual::hooks::set_representative(false, false);
    for name in B2K_ALL { for lane in 0..2 { assert!(failing(name, lane).is_empty(), "{name} lane {lane}"); } }
    // (c) C3: the binary64 sum of the operands' published rows is exactly 0;
    // the combination's own enclosures contain the truth 2^-60/EA and exclude 0.
    let ux = |owner: &adaptive::RetainedSolve| owner.publish().rows.iter()
        .find(|r| b2k_key(r.id) == "u1.0").unwrap().value.value().unwrap();
    assert_eq!(ux(s.case("P")) + ux(s.case("Q")), 0.0);
    let (_, c3) = s.combination("C3");
    let laws = b2k_laws(c3);
    let i = c3.publish().rows.iter().position(|r| b2k_key(r.id) == "u1.0").unwrap();
    for lane in b2k_lanes(c3, &laws) {
        let e = lane.result().unwrap().rows[i];
        assert!(positive(&e.lo), "C3 excludes the operand-row sum 0: {e:?}");
    }
    assert!(ux(c3) > 0.0);
}
#[test]
fn b2k_c05_the_combination_coverage_rule() {
    let s = b2k_specimen(&["C1"]);
    let (run, owner) = s.combination("C1");
    let facts = b2k_facts();
    let begin = |rows: &[B3kRow]| {
        let specs = b3k_specs(rows, "C1");
        s.invocation.begin_prepared_product(run, owner, &facts, &specs).into_ready().err()
            .map(|e| match &e.failure().cause { Cause::Association(a) => *a, other => panic!("{other:?}") })
    };
    let rows = b3k_rows(owner, false, false);
    assert_eq!(rows.len(), 7 * 2 + 50 + 8 - 6 + 6, "7n + 50m + 8g rows (reactions as support components)");
    assert_eq!(begin(&rows), None);
    assert_eq!(begin(&b3k_rows(owner, true, false)), Some("combination row family"), "a maximum");
    assert_eq!(begin(&b3k_rows(owner, false, true)), Some("combination row family"), "a mode record");
    for recipe in [ProductRecipe::DenseParityObservation, ProductRecipe::ModulusBasisRecord] {
        let mut extra = rows.clone();
        extra.push(("record".into(), ProductUnit::Record, 0, recipe));
        let specs: Vec<_> = extra.iter().map(|(id, unit, body, r)| match r {
            ProductRecipe::DenseParityObservation => ProductRowSpec::parity(id, "C1", *body, 0).unwrap(),
            ProductRecipe::ModulusBasisRecord => ProductRowSpec::material_record(id, "C1", *body),
            _ => ProductRowSpec::mechanical(id, "C1", *unit, *body, *r).unwrap() }).collect();
        let e = s.invocation.begin_prepared_product(run, owner, &facts, &specs).into_ready().err().unwrap();
        assert!(matches!(e.failure().cause, Cause::Association("combination row family")), "{recipe:?}");
    }
    let mut missing = rows.clone();
    missing.retain(|r| !matches!(r.3, ProductRecipe::Stress { site: ProductSite::Station(18), stress: ProductStress::Torsion, .. }));
    assert_eq!(missing.len(), rows.len() - 1);
    assert_eq!(begin(&missing), Some("missing final coverage"));
    // A case owner still requires its maximum and mode record.
    let case = s.case("A");
    let case_run = s.cases[0].run;
    let specs_rows = b3k_rows(case, false, false);
    let specs = b3k_specs(&specs_rows, "A");
    let e = s.invocation.begin_prepared_product(case_run, case, &facts, &specs).into_ready().err().unwrap();
    assert!(matches!(e.failure().cause, Cause::Association("missing final coverage")));
}
fn b2k_print_row(name: &str, i: usize, spec: &ProductRowSpec<'_>, value: f64, k: &Enclosure, g: &Enclosure,
    v: &ProductRowVerdict, components: Option<[f64; 3]>) {
    let unit = match spec.unit { ProductUnit::Millimetre => "mm", ProductUnit::Radian => "rad", ProductUnit::Newton => "N",
        ProductUnit::NewtonMetre => "N.m", ProductUnit::Megapascal => "MPa", ProductUnit::Pascal => "Pa", ProductUnit::Record => "record" };
    let (class, bound) = match v.class {
        Some(adaptive::RowClass::InputDerived) => ("input", 0),
        Some(adaptive::RowClass::AbsoluteVerified { bound_bits }) => ("absolute", bound_bits),
        Some(adaptive::RowClass::RelativeVerified) => ("relative", 0),
        other => panic!("{other:?}"),
    };
    let comps = components.map_or("null".to_string(), |c| format!("[{}]", c.iter().map(|x| format!("\"{:016x}\"", x.to_bits())).collect::<Vec<_>>().join(",")));
    let preds: Vec<String> = v.predicates.iter().map(|p| p.map_or("null".into(), |b| b.to_string())).collect();
    println!("B2K_ROW {{\"combination\":\"{name}\",\"row\":{i},\"key\":\"{}\",\"unit\":\"{unit}\",\"body\":{},\"raw\":\"{:016x}\",\"n\":\"{:016x}\",\"class\":\"{class}\",\"bound\":\"{bound:016x}\",\"scale\":\"{:016x}\",\"k\":[\"{}\",\"{}\"],\"g\":[\"{}\",\"{}\"],\"pass\":{},\"predicates\":[{}],\"components\":{comps}}}",
        b2k_recipe_key(spec.recipe), spec.body, value.to_bits(), v.normalized_bits, v.scale_bits,
        b2k_token(&k.lo), b2k_token(&k.hi), b2k_token(&g.lo), b2k_token(&g.hi), v.passed, preds.join(","));
}
#[test]
fn b2k_k09_full_prepared_proof_certifies_combinations_and_prints_the_diagnose_rows() {
    let s = b2k_specimen(&B2K_ALL);
    let facts = b2k_facts();
    let mut certified = Vec::new();
    for name in B2K_ALL {
        let (run, owner) = s.combination(name);
        assert_eq!(s.invocation.product_owner(run, owner), Some(NativeOwner::Combination(B2K_ALL.iter().position(|n| *n == name).unwrap())));
        let rows = b3k_rows(owner, false, false);
        let specs = b3k_specs(&rows, name);
        let draft = s.invocation.begin_prepared_product(run, owner, &facts, &specs).into_ready()
            .unwrap_or_else(|e| panic!("{name}: begin {:?}", e.failure()));
        let (projected, builder) = draft.project().into_ready().unwrap_or_else(|e| panic!("{name}: project {:?}", e.failure()));
        // Every row's two lane enclosures as the gate will hull them.
        let mut lanes = Vec::new();
        for spec in specs.iter() {
            let mut w = ProductCertificateSpent::new(&[]);
            let section = section_for(owner, &facts, spec.recipe, &mut w).unwrap();
            let k = recipe(owner, &mut w, &projected.data.k.rows, spec.recipe, section.as_ref(), &facts, true).unwrap();
            let g = recipe(owner, &mut w, &projected.data.source.rows, spec.recipe, section.as_ref(), &facts, false).unwrap();
            let [tk, tg] = b2k_truth(name, &b2k_recipe_key(spec.recipe));
            assert!(b2k_contains(&k, &tk) && b2k_contains(&g, &tg), "{name} {}: truth outside a lane", spec.id);
            lanes.push((k, g));
        }
        let (values, work) = builder.complete_maxima(&[]).into_ready().unwrap();
        let frozen: Vec<f64> = (0..values.len()).map(|i| *values.value(i).unwrap()).collect();
        // (ii): every displacement magnitude is RN64 of the exact 3-norm of its
        // node's frozen components (mm).
        let mut components = vec![None; specs.len()];
        for (i, spec) in specs.iter().enumerate() {
            let ProductRecipe::Native(QuantityId::DisplacementMagnitude(node)) = spec.recipe else { continue };
            let c: [f64; 3] = std::array::from_fn(|j| frozen[specs.iter().position(|x| x.recipe
                == ProductRecipe::Native(QuantityId::Displacement(Dof { node, component: Component::ALL[j] }))).unwrap()]);
            assert_eq!(frozen[i].to_bits(), rn64_norm3(c).result.unwrap().to_bits(), "{name} m{node}");
            components[i] = Some(c);
        }
        let final_rows: Vec<_> = specs.iter().enumerate().map(|(i, x)| x.row(values.value(i).unwrap())).collect();
        let outcome = projected.certify_final(&values, &final_rows, work).into_ready();
        let verdicts = match &outcome { Ok(p) => p.verdicts().to_vec(), Err(e) => e.work().verdicts().to_vec() };
        assert_eq!(verdicts.len(), specs.len(), "{name}: every row has a verdict");
        let hex: String = owner.prep.ledger.encoding().iter().map(|b| format!("{b:02x}")).collect();
        println!("B2K_LEDGER {{\"combination\":\"{name}\",\"k4led\":\"{hex}\",\"precision\":{},\"certified\":{}}}",
            owner.selected_precision(), outcome.is_ok());
        for (i, spec) in specs.iter().enumerate() {
            b2k_print_row(name, i, spec, frozen[i], &lanes[i].0, &lanes[i].1, &verdicts[i], components[i]);
        }
        certified.push((name, outcome.is_ok(), verdicts.iter().filter(|v| !v.passed).count()));
        // The net-case control: a case whose loads are the combination's exact
        // products (each split exactly into binary64 terms) has the same K4LED
        // and the same publication, and its proof decides every shared row the
        // same way. A combination row that fails is DEF-O's availability limit,
        // which an ordinary case with the same loads meets too.
        let net = b2k_net_case(name);
        let net_owner = b2k_selected(&net.1);
        assert_eq!(net_owner.prep.ledger.encoding(), owner.prep.ledger.encoding(), "{name}: net case K4LED");
        let net_rows = b3k_rows(net_owner, true, true);
        let net_specs = b3k_specs(&net_rows, "net");
        let net_verdicts = match b3k_prove(&net.0, net.1.run, net_owner, &facts, &net_specs) {
            B3kOutcome::Certified(p, _) => p.verdicts().to_vec(), B3kOutcome::Refused(e) => e.work().verdicts().to_vec() };
        assert_eq!(net_verdicts.len(), net_specs.len());
        for (i, spec) in specs.iter().enumerate() {
            if matches!(spec.recipe, ProductRecipe::Native(QuantityId::DisplacementMagnitude(_))) { continue; }
            let j = net_specs.iter().position(|x| x.recipe == spec.recipe && x.unit == spec.unit).unwrap();
            // Equal normalized bits: the same frozen value; and the same verdict.
            assert_eq!((verdicts[i].passed, verdicts[i].normalized_bits), (net_verdicts[j].passed, net_verdicts[j].normalized_bits),
                "{name} {}: the net case decides the same", spec.id);
        }
    }
    println!("B2K_CERTIFIED {certified:?}");
}
/// The net case of a combination: one case whose loads are every exact
/// product c_i*v_ij, split exactly into binary64 terms (the product and its
/// fused-multiply-add error).
fn b2k_net_case(name: &str) -> (RecordedInvocation, RecordedCase) {
    use crate::structural::retained_api::CaseLimit;
    let c = b2k_oracle::COMBINATIONS.iter().find(|c| c.0 == name).unwrap();
    let mut parts = b2k_source_parts("A");
    parts.loads.clear();
    for (i, &(f, operand, _)) in c.1.iter().enumerate() {
        let f = f64::from_bits(f);
        for (k, &(node, comp, v)) in b2k_oracle::OPERANDS.iter().find(|o| o.0 == operand).unwrap().1.iter().enumerate() {
            let v = f64::from_bits(v);
            let p = f * v;
            let e = f.mul_add(v, -p);
            assert!(p.is_finite() && (e == 0.0 || e.is_normal()));
            for (part, value) in [(0, p), (1, e)] {
                if part == 1 && value == 0.0 { continue; }
                parts.loads.push(super::super::super::source::NodalLoad {
                    dof: Dof { node, component: Component::from_index(comp as usize) }, value,
                    source_id: format!("{i}.{k}.{part}") });
            }
        }
    }
    let source = super::super::super::source::PrimitiveSource::new(parts).unwrap();
    let mut invocation = RecordedInvocation::new(u64::MAX, OriginCapacity::for_calls(&[1], &[]).unwrap()).unwrap();
    let mut cases = invocation.solve_cases(&[source], CaseLimit::new(u64::MAX)).unwrap();
    (invocation, cases.remove(0))
}
#[test]
fn b2k_k06_both_certificate_entries_accept_a_recorded_combination_owner_only() {
    let s = b2k_specimen(&["C1"]);
    let (run, owner) = s.combination("C1");
    let facts = b2k_facts();
    let rows = b3k_rows(owner, false, false);
    let specs = b3k_specs(&rows, "C1");
    assert!(s.invocation.begin_prepared_product(run, owner, &facts, &specs).into_ready().is_ok());
    let values: Vec<f64> = owner.publish().rows.iter().map(|_| 0.0).collect();
    drop(values);
    let zero = 0.0;
    let probe = [ProductFinalRow { id: "x", case_id: "C1", value: &zero, unit: ProductUnit::Millimetre, body: 0,
        recipe: ProductRecipe::Native(QuantityId::DisplacementMagnitude(1)) }];
    let spent = s.invocation.certify_product_case(run, owner, &facts, &probe);
    assert!(!matches!(spent.failure().map(|f| &f.cause), Some(Cause::Association("recorded owner"))), "{:?}", spent.failure());
    // Another run id, and a legacy unrecorded combination of the same operands.
    let other = s.cases[0].run;
    let e = s.invocation.begin_prepared_product(other, owner, &facts, &specs).into_ready().err().unwrap();
    assert!(matches!(e.failure().cause, Cause::Association("prepared recorded owner")));
    let legacy = match super::super::super::combine::RetainedCombination::solve(
        &[(1.0, s.case("A")), (1.0, s.case("B")), (-1.0, s.case("A2"))],
        adaptive::CaseLimit::new(u64::MAX), &mut adaptive::InvocationMeter::new(u64::MAX)) {
        super::super::super::combine::CombinationOutcome::Selected(x) => x, other => panic!("{other:?}") };
    assert_eq!(legacy.publish(), owner.publish());
    let e = s.invocation.begin_prepared_product(run, &legacy, &facts, &specs).into_ready().err().unwrap();
    assert!(matches!(e.failure().cause, Cause::Association("prepared recorded owner")));
    let spent = s.invocation.certify_product_case(run, &legacy, &facts, &probe);
    assert!(matches!(spent.failure().map(|f| &f.cause), Some(Cause::Association("recorded owner"))));
    assert_eq!(s.invocation.product_owner(run, &legacy), None);
    assert_eq!(s.invocation.product_owner(s.cases[0].run, s.case("A")), Some(NativeOwner::Case(0)));
}
/// SA4-1's vectors and the A-5 midpoints (REVISION_02 §1.5), each expected bit
/// pattern computed by the oracle's independent integer square root; MIN = MIN_POSITIVE.
const B2K_NORM3: &[(u64, u64, u64, Option<u64>)] = &[
    (0x3de0000002000000, 0x3f80000004000000, 0, Some(0x3f80000004000000)), // A-5: a tie, to even
    (0x3de0000002000000, 0x3f80000004000000, 1, Some(0x3f80000004000001)), // the tie + 2^-1074
    (0x000fffffffffffff, 0x0000000004000001, 0, Some(0x0010000000000000)), // RV115's counterexample: MIN
    (0x000fffffffffffff, 0x0000000004000000, 0, Some(0x0010000000000000)), // RV118's witness (2^-1048): MIN
    (0x000fffffffffffff, 0x0000000003ffffff, 0x0000000000002d42, Some(0x0010000000000000)), // near threshold: MIN
    (0x000fffffffffffff, 0x0000000003ffffff, 0x0000000000002d41, Some(0x000fffffffffffff)), // its control
    (0x7fefffffffffffff, 0, 0, Some(0x7fefffffffffffff)),
    (0x7fefffffffffffff, 0x7e40000000000000, 0, Some(0x7fefffffffffffff)), // MAX, 2^997
    (0x7fefffffffffffff, 0x7e50000000000000, 0, None),                     // MAX, 2^998: beyond binary64
    (0x7fefffffffffffff, 0x7fefffffffffffff, 0, None),
    (0x8000000000000000, 0, 0x8000000000000000, Some(0)),                   // signed zeros: +0
    (1, 0, 0, Some(1)),
    (0xc008000000000000, 0x4010000000000000, 0x4028000000000000, Some(0x402a000000000000)), // (-3, 4, 12) = 13
];
#[test]
fn b2k_rn64_norm3_vectors_signed_zeros_refusal_and_work() {
    for &(x, y, z, expected) in B2K_NORM3 {
        let spent = rn64_norm3([x, y, z].map(f64::from_bits));
        match expected {
            Some(e) => assert_eq!(spent.result.as_ref().map(|v| v.to_bits()), Ok(e), "{x:016x} {y:016x} {z:016x}"),
            None => assert_eq!(spent.result, Err(directed::certificate::HelperError::Binary64Range), "{x:016x} {y:016x}"),
        }
        if x | y | z != 0 && (x | y | z) & 0x7fff_ffff_ffff_ffff != 0 {
            assert!(spent.sums.checked_lme().exact().unwrap() > 0);
        }
    }
    // Component search: a missing, duplicated or non-mm component refuses.
    let spec = |i: u32, unit| ProductRowSpec::mechanical(["x", "y", "z", "w"][i as usize], "C", unit, 0,
        ProductRecipe::Native(QuantityId::Displacement(Dof { node: 1, component: Component::ALL[(i % 3) as usize] }))).unwrap();
    let full = [spec(0, ProductUnit::Millimetre), spec(1, ProductUnit::Millimetre), spec(2, ProductUnit::Millimetre)];
    let mut w = ProductCertificateSpent::new(&[]);
    assert_eq!(displacement_norm(&full, &[3.0, 4.0, 12.0], 1, &mut w).unwrap(), 13.0);
    assert_eq!(w.scalar_operations.exact(), Ok(1));
    for variant in 0..4 {
        let mut specs: Vec<_> = vec![spec(0, ProductUnit::Millimetre), spec(1, ProductUnit::Millimetre), spec(2, ProductUnit::Millimetre)];
        let mut values = vec![3.0, 4.0, 12.0];
        match variant {
            0 => { specs.pop(); values.pop(); }
            1 => { specs.push(spec(3, ProductUnit::Millimetre)); values.push(1.0); }
            2 => specs[1] = spec(1, ProductUnit::Radian),
            _ => values[2] = f64::INFINITY,
        }
        let mut w = ProductCertificateSpent::new(&[]);
        assert_eq!(displacement_norm(&specs, &values, 1, &mut w).unwrap_err().category(), "association", "variant {variant}");
        assert_eq!(w.scalar_operations.exact(), Ok(0));
    }
}
#[test]
fn b2k_rn64_norm3_agrees_with_the_oracle_vectors() {
    // The oracle's independent integer square root on curated SA4-1 vectors,
    // constructed exact midpoints and their one-ulp neighbours, and random
    // patterns over the whole range.
    assert!(b2k_oracle::NORM3.len() > 300);
    let mut ties = 0;
    for &(x, y, z, expected) in b2k_oracle::NORM3 {
        let spent = rn64_norm3([x, y, z].map(f64::from_bits));
        match expected {
            Some(e) => assert_eq!(spent.result.as_ref().map(|v| v.to_bits()), Ok(e), "{x:016x} {y:016x} {z:016x}"),
            None => assert_eq!(spent.result, Err(directed::certificate::HelperError::Binary64Range), "{x:016x} {y:016x} {z:016x}"),
        }
        ties += usize::from(expected.is_some_and(|e| e & 1 == 0) && x & 0xfff != 0);
    }
    assert!(ties > 0);
}
