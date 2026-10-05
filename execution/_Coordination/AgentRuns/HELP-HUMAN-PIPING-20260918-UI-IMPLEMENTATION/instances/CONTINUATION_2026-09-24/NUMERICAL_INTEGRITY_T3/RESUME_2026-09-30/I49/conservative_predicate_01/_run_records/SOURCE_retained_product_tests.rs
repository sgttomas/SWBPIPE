use super::retained_product::ProductCapture;
use super::*;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
fn specimen(loaded: bool) -> serde_json::Value {
    let mut v: serde_json::Value = serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/invented_dec092_temperature_g_request.json"
    ))
    .unwrap();
    v["model"]["schema_version"] = serde_json::json!("0.2.0");
    v["model"]["nodes"][1]["position"] = serde_json::json!({"x":1.0,"y":0.0,"z":0.0});
    v["model"]["pipe_segments"][0]["section"] = serde_json::json!({"outside_diameter":{"value":0.1,"unit":"m"},"wall_thickness":{"value":0.005,"unit":"m"}});
    v["materials"][0]["elastic_modulus"] = serde_json::json!({"value":210e9,"unit":"Pa"});
    v["materials"][0]["shear_modulus"] = serde_json::json!({"value":80e9,"unit":"Pa"});
    v["model"]["combinations"] = serde_json::json!([]);
    let tip = v["model"]["nodes"][1]["id"].as_str().unwrap().to_owned();
    let loads: Vec<_> = [
        ("UX", "force", "N", "concentrated_force"),
        ("UY", "force", "N", "concentrated_force"),
        ("RX", "moment", "N*m", "concentrated_moment"),
    ]
    .into_iter()
    .map(|(direction, dimension, unit, category)| {
        serde_json::json!({
            "id":format!("i45:{direction}"),"category":category,"target":{"type":"node","node":tip},
            "direction":direction,"magnitude":{"value":if loaded {1.0}else{0.0},"unit":unit},
            "dimension":dimension,"provenance":"invented_i45_user_load"
        })
    })
    .collect();
    v["model"]["load_cases"] = serde_json::json!([{"id":if loaded{"case:i45-loaded"}else{"case:i45-zero"},"primitive_loads":loads,"provenance":"invented_i45_case"}]);
    v
}
fn observed(raw: serde_json::Value) -> (MechanicsEnvelope, ProductCapture) {
    let mode = PreviewSolverMode::SparseInteractive;
    let (request, capture) = source_receipt::CapturedInvocation::parse(raw, mode).unwrap();
    let mut observer = ProductCapture::default();
    let e = run_linear_static_preview_observed(
        request,
        mode,
        Some(&capture),
        &mut SourceRecoveryBudget::default(),
        Some(&mut observer),
    );
    (e, observer)
}
#[test]
fn actual_ordinary_zero_loaded_capture_and_final_verdict() {
    for loaded in [false, true] {
        let raw = specimen(loaded);
        println!("I45_REQUEST {}", serde_json::to_string(&raw).unwrap());
        let baseline = run_linear_static_preview_value_with_mode(
            raw.clone(),
            PreviewSolverMode::SparseInteractive,
        )
        .unwrap();
        let (e, o) = observed(raw);
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            serde_json::to_string(&baseline).unwrap(),
            "observer changed ordinary bytes"
        );
        assert_eq!(
            e.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            e.diagnostics
        );
        assert!(o.error.is_none(), "{:?}", o.error);
        assert_eq!((o.normalized_calls, o.case_calls, o.final_calls), (1, 1, 1));
        assert!(o.request_materials);
        assert_eq!(o.terms.len(), 3);
        assert_eq!(o.facts[0].diameter.to_bits(), 0.1f64.to_bits());
        assert_eq!(o.facts[0].effective_wall.to_bits(), 0.005f64.to_bits());
        let (invocation, case) = o.native.as_ref().unwrap();
        assert_eq!(invocation.calls().len(), 1);
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
            panic!("not selected")
        };
        assert_eq!(owner.publish().rows.len(), 52);
        assert_eq!(e.results.len(), 74);
        assert!(
            o.numeric_failure.is_none(),
            "{:?}; {}",
            o.numeric_failure,
            o.work
        );
        assert_eq!(o.verdicts.len(), e.results.len());
        assert!(o.observable_error.is_none(), "{:?}", o.observable_error);
        println!("I45_CASE {}", if loaded { "loaded" } else { "zero" });
        let f = &o.facts[0];
        let source = o.source.as_ref().unwrap();
        let m = &source.members()[0];
        println!(
            "I45_INPUT {}",
            serde_json::json!({"D":f.diameter,"t":f.effective_wall,"E":m.elastic_modulus,"G":m.shear_modulus,
            "A":f.area,"I":f.second_moment,"J":f.torsion_constant,"Z":f.section_modulus,"c":f.radius,"nodes":source.nodes()})
        );
        println!("I45_ROWS {}", serde_json::to_string(&e.results).unwrap());
        let verdicts:Vec<_>=o.verdicts.iter().map(|v|{
            let (class,bound)=match v.class {None=>("nonquantity",None),Some(k::RowClass::InputDerived)=>("input",None),
                Some(k::RowClass::RelativeVerified)=>("relative",None),Some(k::RowClass::AbsoluteVerified{bound_bits})=>("absolute",Some(format!("{bound_bits:016x}"))),
                _=>("unpublishable",None)};
            serde_json::json!({"row":v.row,"normalized_bits":format!("{:016x}",v.normalized_bits),"scale_bits":format!("{:016x}",v.scale_bits),
                "class":class,"bound_bits":bound,"passed":v.passed,"predicates":v.predicates,"failed":v.failed.map(|p|format!("{p:?}"))})
        }).collect();
        println!("I45_VERDICTS {}", serde_json::to_string(&verdicts).unwrap());
        println!("I45_WORK {}", o.work);
        println!("I45_PASS {}", o.numeric_pass);
        println!("I45_FULL_CASE {}", o.full_case_passed());
        println!("I45_G5A {:?}", o.g5a_error);
        println!("I45_OPERATIONAL {:?}", o.operational);
        println!("I45_G5A_WORK {:?}", o.g5a_work);
        println!("I45_ADAPTER {:?}", o.adapter);
        println!("I45_TERM_MAP {:?}", o.terms);
        let ev = owner.evidence();
        let kind = |k: k::Kind| match k {
            k::Kind::Translation => 0,
            k::Kind::Rotation => 1,
            k::Kind::Force => 2,
            k::Kind::Moment => 3,
        };
        let summary = |items: &[(u32, k::Kind, f64)]| {
            items
                .iter()
                .map(|(b, k, v)| serde_json::json!([b, kind(*k), format!("{:016x}", v.to_bits())]))
                .collect::<Vec<_>>()
        };
        let operational=o.operational.iter().map(|v|v.result.as_ref().map(|r|serde_json::json!({"L":r.length,"ka":r.axial,"kt":r.torsion,"operations":v.work.entered,"checks":v.work.checks,"lost":v.work.lost})).unwrap_or_else(|e|serde_json::json!({"failure":format!("{e:?}"),"operations":v.work.entered,"lost":v.work.lost}))).collect::<Vec<_>>();
        println!(
            "I45_G5A_DATA {}",
            serde_json::json!({"precision":owner.selected_precision(),"stop":summary(&ev.stop_rule),"estimate":summary(&ev.verification_estimate),"charge":summary(&ev.verification_charge),
            "resolution":ev.resolution_scale.iter().map(|(b,f,m)|serde_json::json!([b,format!("{f:016x}"),format!("{m:016x}")])).collect::<Vec<_>>(),
            "theta":ev.theta,"B":ev.certified_bound.iter().map(|(b,v)|serde_json::json!([b,format!("{v:016x}")])).collect::<Vec<_>>(),
            "coverage":o.summary_coverage.iter().map(|c|serde_json::json!({"body":c.body,"stop":c.stop,"estimate":c.estimate,"charge":c.charge,"data":c.has_data})).collect::<Vec<_>>(),
            "operational":operational,"failure":o.g5a_error.as_ref().map(|e|format!("{e:?}")),"full_case":o.full_case_passed()})
        );
        println!("I45_NATIVE_ROWS {}",serde_json::to_string(&owner.publish().rows.iter().enumerate().map(|(i,r)|serde_json::json!({"ordinal":i,"quantity":format!("{:?}",r.id),"kind":kind(r.kind),"body":r.body,"value_bits":r.value.value().map(|v|format!("{:016x}",v.to_bits())),"class":format!("{:?}",r.class)})).collect::<Vec<_>>()).unwrap());
        println!(
            "I45_NATIVE_IDENTITY {}",
            serde_json::json!({"method":ev.method,"policy":ev.policy,"source":ev.source_encoding,"ledger":ev.ledger_encoding,"run":case.run,"calls":invocation.calls().len()})
        );
        assert!(o
            .operational
            .iter()
            .all(|v| v.work.entered == 23 && !v.work.lost));
        assert!(o.adapter.fault.get().is_none());
        if loaded {
            assert!(o.g5a_error.is_none());
        } else {
            assert_eq!(
                o.g5a_error,
                Some(super::retained_product::G5aFailure::Zero { row: 35 })
            );
        }
        assert!(!o.full_case_passed());
        if !loaded {
            assert!(o.numeric_pass);
        }
    }
}

#[test]
fn actual_final_identity_coverage_observable_and_headline_mutations_refuse() {
    let (e, o) = observed(specimen(true));
    assert!(o.error.is_none(), "{:?}", o.error);
    let (invocation, case) = o.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    let mut changed = e.clone();
    changed.results[2].id.push_str("-foreign");
    assert!(o.bind_rows(&changed, owner).is_err());
    let mut changed = e.clone();
    changed.results[3]
        .metadata
        .as_mut()
        .unwrap()
        .sign_convention
        .push_str("-wrong");
    assert!(o.bind_rows(&changed, owner).is_err());
    let mut changed = e.clone();
    changed.results[2].basis_ref.as_mut().unwrap().ref_id = "foreign-case".into();
    assert!(o.bind_rows(&changed, owner).is_err());
    // Coherent metadata+id mutation must fail independently of value, units or duplicate ids.
    let mut changed = e.clone();
    let index = changed
        .results
        .iter()
        .position(|r| r.kind == "support_reaction_force_magnitude_v2")
        .unwrap();
    let row = &mut changed.results[index];
    row.metadata.as_mut().unwrap().component = "unregistered_magnitude".into();
    row.id = format!(
        "{}unregistered_magnitude",
        row.id.strip_suffix("force_magnitude").unwrap()
    );
    assert!(o.bind_rows(&changed, owner).is_err());
    let mut changed = e.clone();
    let index = changed
        .results
        .iter()
        .position(|r| r.kind == "element_local_torsional_shear_stress")
        .unwrap();
    changed.results.remove(index);
    let rows = o.bind_rows(&changed, owner).unwrap();
    let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
    assert_eq!(spent.failure().unwrap().category(), "association");
    assert!(!spent.passed());
    let mut changed = e.clone();
    let index = changed
        .results
        .iter()
        .position(|r| r.kind == "support_reaction_force_magnitude_v2")
        .unwrap();
    changed.results[index].value += 1e-8;
    assert_eq!(
        o.observables(&changed).unwrap_err().to_string(),
        "support guard"
    );
    let mut changed = e.clone();
    let index = changed
        .results
        .iter()
        .position(|r| r.kind == "pipe_elastic_normal_stress_maximum_v2")
        .unwrap();
    changed.results[index].value = f64::from_bits(changed.results[index].value.to_bits() + 1);
    assert_eq!(
        o.observables(&changed).unwrap_err().to_string(),
        "maximum midpoint"
    );
    let mut facts = o.facts.clone();
    facts[0].effective_wall = f64::NAN;
    let rows = o.bind_rows(&e, owner).unwrap();
    let spent = invocation.certify_product_case(case.run, owner, &facts, &rows);
    assert!(!spent.passed());
    assert!(spent.failure().is_some());
    let foreign = k::RecordedInvocation::new(
        60_000_000_000,
        k::OriginCapacity::for_calls(&[1], &[]).unwrap(),
    )
    .unwrap();
    let spent = foreign.certify_product_case(case.run, owner, &o.facts, &rows);
    assert_eq!(spent.failure().unwrap().category(), "association");

    let (mut zero, z) = observed(specimen(false));
    let tip = zero
        .results
        .iter()
        .find(|r| r.kind == "displacement_magnitude" && r.entity_ref.ends_with("TIP"))
        .unwrap();
    let h = zero.summary.max_displacement.as_mut().unwrap();
    h.result_ref = tip.id.clone();
    h.location_ref = tip.entity_ref.clone();
    assert_eq!(
        z.observables(&zero).unwrap_err().to_string(),
        "headline alias"
    );
}
#[test]
fn actual_resolver_selection_ordinals_and_validity_are_captured_without_replay() {
    for interpolation in [false, true] {
        let raw: serde_json::Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/invented_dec092_temperature_g_request.json"
        ))
        .unwrap();
        let (mut request, _capture) =
            source_receipt::CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive)
                .unwrap();
        let mut diagnostics = Vec::new();
        normalize_model_units(&mut request.model, &mut request.materials, &mut diagnostics);
        assert!(!has_blocking(&diagnostics));
        let case = &request.model.load_cases[if interpolation { 1 } else { 0 }];
        let mut observer = ProductCapture::default();
        let (resolved, _) = materials_for_modulus_basis_observed(
            &request.model,
            &request.materials,
            case,
            &mut diagnostics,
            Some(&mut observer),
        )
        .unwrap();
        assert_eq!(observer.selections.len(), 1);
        let selection = &observer.selections[0];
        assert_eq!(selection.material_ordinal, 0);
        match selection.selected {
            k::ProductMaterial::Point { ordinal, e, g } => {
                assert!(!interpolation);
                assert_eq!(ordinal, 0);
                assert_eq!(e.to_bits(), resolved[0].elastic_modulus.value.to_bits());
                assert_eq!(
                    g.to_bits(),
                    resolved[0].shear_modulus.as_ref().unwrap().value.to_bits()
                );
            }
            k::ProductMaterial::Interpolated {
                lower,
                upper,
                t_lo,
                t,
                t_hi,
                e_hat,
                g_hat,
                ..
            } => {
                assert!(interpolation);
                assert_eq!((lower, upper), (1, 2));
                assert_eq!((t_lo, t, t_hi), (300.0, 425.0, 500.0));
                assert_eq!(e_hat.to_bits(), resolved[0].elastic_modulus.value.to_bits());
                assert_eq!(
                    g_hat.to_bits(),
                    resolved[0].shear_modulus.as_ref().unwrap().value.to_bits()
                );
            }
            _ => panic!(),
        }
        if interpolation {
            request.materials[0].temperature_points[1].thermal_expansion_coefficient = None;
            let mut rejected = ProductCapture::default();
            let mut d = Vec::new();
            assert!(materials_for_modulus_basis_observed(
                &request.model,
                &request.materials,
                case,
                &mut d,
                Some(&mut rejected)
            )
            .is_none());
            assert!(rejected.selections.is_empty());
        }
    }
}
#[test]
fn actual_cancelled_individual_terms_are_preserved() {
    let mut raw = specimen(false);
    let mut term = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
    let loads = raw["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap();
    term["id"] = serde_json::json!("i45:cancel-plus");
    term["magnitude"]["value"] = serde_json::json!(1.0);
    loads.push(term.clone());
    term["id"] = serde_json::json!("i45:cancel-minus");
    term["magnitude"]["value"] = serde_json::json!(-1.0);
    loads.push(term);
    let (e, o) = observed(raw);
    assert!(o.error.is_none(), "{:?}", o.error);
    assert_eq!(e.status.mechanics, "MECHANICS_SOLVED");
    assert_eq!(o.terms.len(), 5);
    assert_eq!(o.source.as_ref().unwrap().loads().len(), 5);
    assert_eq!(
        o.terms
            .iter()
            .filter(|t| f64::from_bits(t.bits) != 0.0)
            .count(),
        2
    );
}

#[test]
fn operational_order_range_and_actual_failure_prefix_controls() {
    use super::retained_product::{
        evaluate_operational, OperationalError, ScalarOperation as O, ScalarWork,
    };
    let d = f64::from_bits(0x3e46c00000000000);
    let source = evaluate_operational([[0.0; 3], [1.0, d, d]], [1.0; 4]);
    assert_eq!(
        source.result.as_ref().unwrap().length.to_bits(),
        0x3ff0000000000001
    );
    assert_eq!(source.work.entered, 23);
    let wrong = (1.0 + (d * d + d * d)).sqrt();
    assert_eq!(wrong.to_bits(), 0x3ff0000000000000);
    let (e, a, l) = (
        f64::from_bits(0x40b9863000000000),
        f64::from_bits(0x40a5085000000000),
        f64::from_bits(0x408ad85800000000),
    );
    let source = evaluate_operational([[0.0; 3], [l, 0.0, 0.0]], [e, 1.0, a, 1.0]);
    assert_eq!(
        source.result.as_ref().unwrap().axial.to_bits(),
        0x40d3ff6019418a0a
    );
    assert_ne!(
        source.result.as_ref().unwrap().axial.to_bits(),
        (e * (a / l)).to_bits()
    );
    let degenerate = evaluate_operational([[0.0; 3]; 2], [1.0; 4]);
    assert!(matches!(
        degenerate.result,
        Err(OperationalError::Degenerate)
    ));
    assert_eq!(degenerate.work.entered, 9);
    for (e, a, l, entered, operation) in [
        (f64::MAX, 2.0, 2.0, 20, O::Mul),
        (f64::MIN_POSITIVE, 0.5, 0.5, 20, O::Mul),
        (f64::MIN_POSITIVE, 1.0, 2.0, 21, O::Div),
    ] {
        let spent = evaluate_operational([[0.0; 3], [l, 0.0, 0.0]], [e, 1.0, a, 1.0]);
        assert_eq!(spent.work.entered, entered);
        assert!(
            matches!(spent.result,Err(OperationalError::CoefficientRange{operation:actual,..}) if actual==operation)
        );
        assert!(!spent.work.lost);
    }
    // Synthetic collection-counter seed, followed by an actual overflowing multiplication.
    let mut spent = ScalarWork {
        entered: 0,
        checks: u64::MAX,
        lost: false,
    };
    assert!(matches!(
        spent.op(O::Mul, f64::MAX, 2.0),
        Err(OperationalError::NonFinite {
            operation: O::Mul,
            entered: 1
        })
    ));
    assert!(spent.lost);
    assert_eq!(spent.entered, 1);
    let mut spent = ScalarWork {
        entered: 0,
        checks: u64::MAX,
        lost: false,
    };
    assert!(matches!(
        spent.coefficient(f64::MAX, 2.0, 2.0, "EA/L"),
        Err(OperationalError::CoefficientRange {
            operation: O::Mul,
            ..
        })
    ));
    assert!(spent.lost);
    assert_eq!(spent.entered, 1);
    let mut stopped = ScalarWork {
        entered: 0,
        checks: 0,
        lost: true,
    };
    assert_eq!(
        stopped.op(O::Mul, f64::MAX, 2.0),
        Err(OperationalError::Accounting)
    );
    assert_eq!(stopped.entered, 0);
}
#[test]
fn adapter_byte_prefix_count_range_and_loss_are_typed() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, AdapterWork, CaptureError};
    let work = AdapterWork::default();
    assert!(!work.same("ab", "ac"));
    assert_eq!(work.counts.get()[E::IdentityByteRead as usize], 4);
    let range = work.reserve::<u64>(usize::MAX);
    assert!(matches!(range, Err(CaptureError::CountRange(_))));
    // Synthetic near-overflow counter; rejected read is not performed or charged.
    let mut counts = work.counts.get();
    counts[E::IdentityByteRead as usize] = u64::MAX;
    work.counts.set(counts);
    assert!(!work.same("a", "a"));
    assert_eq!(
        work.fault.get(),
        Some(AdapterFault::Overflow(E::IdentityByteRead))
    );
    assert_eq!(work.counts.get()[E::IdentityByteRead as usize], u64::MAX);
}
#[test]
fn actual_sparse_zero_and_cancelled_summary_coverage_is_not_inferred_from_net() {
    use super::retained_product::{validate_summary_shape, G5aFailure};
    let (_, zero) = observed(specimen(false));
    let (context, case) = zero.native.as_ref().unwrap();
    assert_eq!(context.calls().len(), 1);
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    assert!(!zero.summary_coverage[0].has_data);
    assert_eq!(zero.summary_coverage[0].stop, [false; 4]);
    assert!(owner.evidence().certified_bound.is_empty());
    validate_summary_shape(owner.evidence(), &zero.summary_coverage, 1).unwrap();
    let mut missing = owner.evidence().clone();
    missing.resolution_scale.clear();
    assert!(matches!(
        validate_summary_shape(&missing, &zero.summary_coverage, 1),
        Err(G5aFailure::Shape(_))
    ));
    let mut raw = specimen(false);
    let mut term = raw["model"]["load_cases"][0]["primitive_loads"][0].clone();
    let loads = raw["model"]["load_cases"][0]["primitive_loads"]
        .as_array_mut()
        .unwrap();
    term["id"] = serde_json::json!("i45:plus");
    term["magnitude"]["value"] = serde_json::json!(1.0);
    loads.push(term.clone());
    term["id"] = serde_json::json!("i45:minus");
    term["magnitude"]["value"] = serde_json::json!(-1.0);
    loads.push(term);
    let (_, cancelled) = observed(raw);
    assert!(cancelled.error.is_none());
    let (_, case) = cancelled.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    assert!(cancelled.summary_coverage[0].has_data);
    assert_eq!(cancelled.summary_coverage[0].estimate, [false; 2]);
    assert_eq!(cancelled.source_correction_calls.unwrap().exact(), Ok(1));
    assert_eq!(zero.source_correction_calls.unwrap().exact(), Ok(0));
    validate_summary_shape(owner.evidence(), &cancelled.summary_coverage, 1).unwrap();
    let mut missing = owner.evidence().clone();
    missing.certified_bound.clear();
    assert_eq!(
        validate_summary_shape(&missing, &cancelled.summary_coverage, 1),
        Err(G5aFailure::Shape("B data coverage"))
    );
    let (_, loaded) = observed(specimen(true));
    let (_, case) = loaded.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    assert_eq!(loaded.summary_coverage[0].estimate, [true; 2]);
    let mut missing = owner.evidence().clone();
    missing.verification_estimate.pop();
    assert_eq!(
        validate_summary_shape(&missing, &loaded.summary_coverage, 1),
        Err(G5aFailure::Shape("estimate"))
    );
}

#[test]
fn rv60_closed_evidence_shapes_and_empty_combination_gates_refuse_mutations() {
    use super::retained_product::{AdapterEvent, AdapterFault, CaptureError};
    let (e, o) = observed(specimen(true));
    o.observables(&e).unwrap();
    let original_rows = serde_json::to_string(&e.results).unwrap();
    let paths = [
        "",
        "/preview_cases/0",
        "/preview_cases/0/stress_maximum_coverage",
        "/preview_cases/0/support_attribution",
    ];
    for path in paths {
        // An additional key and a coherent same-length replacement both violate the closed shape.
        let mut changed = e.clone();
        let object = changed
            .contract_evidence
            .as_mut()
            .unwrap()
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();
        object.insert("foreign".into(), serde_json::json!(true));
        assert!(
            matches!(o.observables(&changed), Err(CaptureError::Association(_))),
            "{path}"
        );
        let mut changed = e.clone();
        let object = changed
            .contract_evidence
            .as_mut()
            .unwrap()
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap();
        let key = object.keys().next().unwrap().clone();
        let value = object.remove(&key).unwrap();
        object.insert("foreign".into(), value);
        assert!(
            matches!(o.observables(&changed), Err(CaptureError::Association(_))),
            "{path}"
        );
    }
    let mut missing = e.clone();
    missing
        .contract_evidence
        .as_mut()
        .unwrap()
        .as_object_mut()
        .unwrap()
        .remove("combination_gates");
    assert!(o.observables(&missing).is_err());
    for value in [
        serde_json::Value::Null,
        serde_json::json!({}),
        serde_json::json!([{"foreign":"entry"}]),
    ] {
        let mut changed = e.clone();
        changed.contract_evidence.as_mut().unwrap()["combination_gates"] = value;
        assert!(o.observables(&changed).is_err());
        assert_eq!(
            serde_json::to_string(&changed.results).unwrap(),
            original_rows
        );
    }
    for path in paths {
        let mut changed = e.clone();
        *changed
            .contract_evidence
            .as_mut()
            .unwrap()
            .pointer_mut(path)
            .unwrap() = serde_json::json!([]);
        assert!(o.observables(&changed).is_err(), "{path}");
    }
    println!("RV60_MISSING_COMBINATION_GATES_ACCEPTED false");
    println!("RV60_FOREIGN_COMBINATION_GATES_ACCEPTED false");
    println!("RV60_EXTRA_CASE_KEY_ACCEPTED false");
    println!("RV60_EXTRA_COVERAGE_ACCEPTED false");
    println!("RV60_EXTRA_ATTRIBUTION_ACCEPTED false");
    // Synthetic near-overflow seed: new closed-key comparison cannot masquerade as a shape mismatch.
    let mut counts = o.adapter.counts.get();
    counts[AdapterEvent::KeyProbe as usize] = u64::MAX;
    o.adapter.counts.set(counts);
    assert!(matches!(
        o.observables(&e),
        Err(CaptureError::Accounting(AdapterFault::Overflow(
            AdapterEvent::KeyProbe
        )))
    ));
    assert_eq!(
        o.adapter.counts.get()[AdapterEvent::KeyProbe as usize],
        u64::MAX
    );
}

#[test]
fn rv60_fixed_mode_sign_refuses_in_isolated_synthetic_zero_snapshot() {
    use super::retained_product::CaptureError;
    let (actual, o) = observed(specimen(false));
    let actual_bytes = serde_json::to_string(&actual).unwrap();
    assert!(actual.results[35].value.is_sign_negative());
    let (invocation, case) = o.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    o.bind_rows(&actual, owner).unwrap();
    // Explicit synthetic snapshot only: remove the known -0 obstruction to numeric comparison.
    // This never replaces the actual ordinary output or claims actual W0 availability.
    let mut synthetic = actual.clone();
    for row in &mut synthetic.results {
        if row.value == 0.0 {
            row.value = 0.0;
        }
    }
    let rows = o.bind_rows(&synthetic, owner).unwrap();
    assert!(invocation
        .certify_product_case(case.run, owner, &o.facts, &rows)
        .passed());
    o.observables(&synthetic).unwrap();
    let mut changed = synthetic.clone();
    let mode = changed
        .results
        .iter_mut()
        .find(|r| r.kind == "linear_solver_mode_basis")
        .unwrap();
    mode.metadata.as_mut().unwrap().sign_convention = "mode_code 1=dense_scrutiny".into();
    let error = o.bind_rows(&changed, owner).unwrap_err();
    assert!(matches!(error, CaptureError::Association(_)));
    assert_eq!(error.to_string(), "ordinary sparse mode sign");
    assert_eq!(serde_json::to_string(&actual).unwrap(), actual_bytes);
    assert!(actual.results[35].value.is_sign_negative());
    println!("RV60_SYNTHETIC_FULL_GATES_ACCEPT_WRONG_MODE_SIGN false");
}

// I47 retains the admitted I45 geometry/load specimen and the original material/point ids.
// Values are fixed before execution; the small endpoint offsets distinguish exact
// source interpolation from the separately rounded represented selection.
fn i47_selected_specimen(interpolated: bool, loaded: bool) -> serde_json::Value {
    let mut raw = specimen(loaded);
    let points = raw["materials"][0]["temperature_points"]
        .as_array_mut()
        .unwrap();
    for (i, e, g) in [
        (0, 195e9, 55e9),
        (1, 200000000000.00003, 60000000000.00001),
        (2, 180e9, 40e9),
    ] {
        points[i]["elastic_modulus"] = serde_json::json!({"value":e,"unit":"Pa"});
        points[i]["shear_modulus"] = serde_json::json!({"value":g,"unit":"Pa"});
    }
    points[1]["temperature"] = serde_json::json!({"value":293.0,"unit":"K"});
    points[2]["temperature"] = serde_json::json!({"value":313.0,"unit":"K"});
    if interpolated {
        raw["model"]["load_cases"][0]["modulus_basis_temperature"] =
            serde_json::json!({"value":303.0,"unit":"K"});
    } else {
        raw["model"]["load_cases"][0]["modulus_basis_ref"] =
            serde_json::json!("temperature-point:exact");
    }
    raw
}
fn i47_selection_json(o: &ProductCapture) -> serde_json::Value {
    let s = &o.selections[0];
    let h = |v: f64| format!("{:016x}", v.to_bits());
    let values = match s.selected {
        k::ProductMaterial::Point { ordinal, e, g } => {
            serde_json::json!({"kind":"point","ordinal":ordinal,"e":h(e),"g":h(g)})
        }
        k::ProductMaterial::Interpolated {
            lower,
            upper,
            t_lo,
            t,
            t_hi,
            e_lo,
            e_hi,
            g_lo,
            g_hi,
            e_hat,
            g_hat,
        } => {
            serde_json::json!({"kind":"interpolated","lower":lower,"upper":upper,"t_lo":h(t_lo),"t":h(t),"t_hi":h(t_hi),"e_lo":h(e_lo),"e_hi":h(e_hi),"g_lo":h(g_lo),"g_hi":h(g_hi),"e_hat":h(e_hat),"g_hat":h(g_hat)})
        }
        _ => panic!("base substitution"),
    };
    serde_json::json!({"case":s.case,"material":s.material,"material_ordinal":s.material_ordinal,"point_ids":s.point_ids,"selector":format!("{:?}",s.selector),"alpha":s.alpha.map(h),"source_alpha":s.source_alpha.map(|v|v.map(h)),"values":values,"base":o.materials.iter().map(|(id,e,g)|serde_json::json!({"id":id,"e":h(*e),"g":h(*g)})).collect::<Vec<_>>()})
}
fn i47_assert_selected(o: &ProductCapture, interpolated: bool) {
    assert_eq!(o.selections.len(), 1);
    let s = &o.selections[0];
    assert_eq!(s.material_ordinal, 0);
    assert_eq!(s.material, "material:invented-dec092");
    assert_eq!(s.case, o.case_id);
    assert!(s.alpha.is_some());
    let (e, g) = match s.selected {
        k::ProductMaterial::Point { ordinal, e, g } => {
            assert!(!interpolated);
            assert_eq!(ordinal, 0);
            assert_eq!(s.point_ids, ("temperature-point:exact".into(), None));
            assert_eq!((e, g), (195e9, 55e9));
            (e, g)
        }
        k::ProductMaterial::Interpolated {
            lower,
            upper,
            t_lo,
            t,
            t_hi,
            e_lo,
            e_hi,
            g_lo,
            g_hi,
            e_hat,
            g_hat,
        } => {
            assert!(interpolated);
            assert_eq!((lower, upper), (1, 2));
            assert_eq!((t_lo, t, t_hi), (293.0, 303.0, 313.0));
            assert_eq!(
                s.point_ids,
                (
                    "temperature-point:lower".into(),
                    Some("temperature-point:upper".into())
                )
            );
            assert_eq!(
                (e_lo, e_hi, g_lo, g_hi),
                (200000000000.00003, 180e9, 60000000000.00001, 40e9)
            );
            assert!(s.source_alpha.iter().all(Option::is_some));
            (e_hat, g_hat)
        }
        _ => panic!("selected route replaced by base"),
    };
    let m = &o.source.as_ref().unwrap().members()[0];
    assert_eq!(e.to_bits(), m.elastic_modulus.to_bits());
    assert_eq!(g.to_bits(), m.shear_modulus.to_bits());
    assert_ne!(e.to_bits(), o.materials[0].1.to_bits());
    assert_ne!(g.to_bits(), o.materials[0].2.to_bits());
    assert_eq!(
        format!("{:?}", s.selected),
        format!("{:?}", o.facts[0].material)
    );
}

#[test]
fn i47_actual_selected_material_zero_loaded_capture_and_final_verdict() {
    let mut blockers = Vec::new();
    for (interpolated, loaded) in [(false, false), (false, true), (true, false), (true, true)] {
        let raw = i47_selected_specimen(interpolated, loaded);
        println!(
            "I47_CASE {}",
            if interpolated {
                if loaded {
                    "interpolated-loaded"
                } else {
                    "interpolated-zero"
                }
            } else if loaded {
                "point-loaded"
            } else {
                "point-zero"
            }
        );
        println!("I47_REQUEST {}", serde_json::to_string(&raw).unwrap());
        let baseline = run_linear_static_preview_value_with_mode(
            raw.clone(),
            PreviewSolverMode::SparseInteractive,
        )
        .unwrap();
        let (e, o) = observed(raw);
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            serde_json::to_string(&baseline).unwrap(),
            "observer changed ordinary bytes"
        );
        assert_eq!(
            e.status.mechanics, "MECHANICS_SOLVED",
            "{:?}",
            e.diagnostics
        );
        if o.error.is_some() {
            blockers.push(format!(
                "interpolated={interpolated}, loaded={loaded}: {:?}",
                o.error
            ));
        }
        println!(
            "I47_BOUNDARY {}",
            serde_json::json!({"error":format!("{:?}",o.error),"calls":[o.normalized_calls,o.case_calls,o.final_calls],"verdict_count":o.verdicts.len(),"g5a_entered":o.g5a_work.entered,"observable_error":format!("{:?}",o.observable_error),"full_case":o.full_case_passed()})
        );
        assert_eq!((o.normalized_calls, o.case_calls, o.final_calls), (1, 1, 1));
        assert!(o.request_materials);
        assert_eq!(o.terms.len(), 3);
        assert_eq!(o.facts[0].diameter.to_bits(), 0.1f64.to_bits());
        assert_eq!(o.facts[0].effective_wall.to_bits(), 0.005f64.to_bits());
        let (invocation, case) = o.native.as_ref().unwrap();
        assert_eq!(invocation.calls().len(), 1);
        let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
            panic!("not selected")
        };
        assert_eq!(owner.publish().rows.len(), 52);
        assert_eq!(e.results.len(), 75);
        assert!(
            o.numeric_failure.is_none(),
            "{:?}; {}",
            o.numeric_failure,
            o.work
        );
        if o.error.is_none() {
            assert_eq!(o.verdicts.len(), e.results.len());
        }
        assert!(o.observable_error.is_none(), "{:?}", o.observable_error);
        i47_assert_selected(&o, interpolated);
        println!("I47_SELECTION {}", i47_selection_json(&o));
        println!(
            "I47_BASIS_CAPTURE {}",
            serde_json::json!({"case":o.basis_record.as_ref().unwrap().case,"text":o.basis_record.as_ref().unwrap().text,"calls":o.basis_record_calls,"expected":o.basis_expected})
        );
        println!("I47_ANCILLARY {}", serde_json::to_string(&e).unwrap());
        println!(
            "I47_CAPTURE {:?}",
            (
                &o.nodes,
                &o.materials,
                &o.members,
                &o.supports,
                &o.facts,
                &o.capacities,
                &o.source_correction_calls
            )
        );
        let f = &o.facts[0];
        let source = o.source.as_ref().unwrap();
        let m = &source.members()[0];
        println!(
            "I47_INPUT {}",
            serde_json::json!({"D":f.diameter,"t":f.effective_wall,"E":m.elastic_modulus,"G":m.shear_modulus,
            "A":f.area,"I":f.second_moment,"J":f.torsion_constant,"Z":f.section_modulus,"c":f.radius,"nodes":source.nodes()})
        );
        println!("I47_ROWS {}", serde_json::to_string(&e.results).unwrap());
        let verdicts:Vec<_>=o.verdicts.iter().map(|v|{
            let (class,bound)=match v.class {None=>("nonquantity",None),Some(k::RowClass::InputDerived)=>("input",None),
                Some(k::RowClass::RelativeVerified)=>("relative",None),Some(k::RowClass::AbsoluteVerified{bound_bits})=>("absolute",Some(format!("{bound_bits:016x}"))),
                _=>("unpublishable",None)};
            serde_json::json!({"row":v.row,"normalized_bits":format!("{:016x}",v.normalized_bits),"scale_bits":format!("{:016x}",v.scale_bits),
                "class":class,"bound_bits":bound,"passed":v.passed,"predicates":v.predicates,"failed":v.failed.map(|p|format!("{p:?}"))})
        }).collect();
        println!("I47_VERDICTS {}", serde_json::to_string(&verdicts).unwrap());
        println!("I47_WORK {}", o.work);
        println!("I47_PASS {}", o.numeric_pass);
        println!("I47_FULL_CASE {}", o.full_case_passed());
        println!("I47_G5A {:?}", o.g5a_error);
        println!("I47_OPERATIONAL {:?}", o.operational);
        println!("I47_G5A_WORK {:?}", o.g5a_work);
        println!("I47_ADAPTER {:?}", o.adapter);
        println!("I47_TERM_MAP {:?}", o.terms);
        let ev = owner.evidence();
        println!(
            "I47_NATIVE_WORK {:?}",
            (invocation.calls(), invocation.runs(), &ev.attempts)
        );
        let kind = |k: k::Kind| match k {
            k::Kind::Translation => 0,
            k::Kind::Rotation => 1,
            k::Kind::Force => 2,
            k::Kind::Moment => 3,
        };
        let summary = |items: &[(u32, k::Kind, f64)]| {
            items
                .iter()
                .map(|(b, k, v)| serde_json::json!([b, kind(*k), format!("{:016x}", v.to_bits())]))
                .collect::<Vec<_>>()
        };
        let operational=o.operational.iter().map(|v|v.result.as_ref().map(|r|serde_json::json!({"L":r.length,"ka":r.axial,"kt":r.torsion,"operations":v.work.entered,"checks":v.work.checks,"lost":v.work.lost})).unwrap_or_else(|e|serde_json::json!({"failure":format!("{e:?}"),"operations":v.work.entered,"lost":v.work.lost}))).collect::<Vec<_>>();
        println!(
            "I47_G5A_DATA {}",
            serde_json::json!({"precision":owner.selected_precision(),"stop":summary(&ev.stop_rule),"estimate":summary(&ev.verification_estimate),"charge":summary(&ev.verification_charge),
            "resolution":ev.resolution_scale.iter().map(|(b,f,m)|serde_json::json!([b,format!("{f:016x}"),format!("{m:016x}")])).collect::<Vec<_>>(),
            "theta":ev.theta,"B":ev.certified_bound.iter().map(|(b,v)|serde_json::json!([b,format!("{v:016x}")])).collect::<Vec<_>>(),
            "coverage":o.summary_coverage.iter().map(|c|serde_json::json!({"body":c.body,"stop":c.stop,"estimate":c.estimate,"charge":c.charge,"data":c.has_data})).collect::<Vec<_>>(),
            "operational":operational,"failure":o.g5a_error.as_ref().map(|e|format!("{e:?}")),"full_case":o.full_case_passed()})
        );
        println!("I47_NATIVE_ROWS {}",serde_json::to_string(&owner.publish().rows.iter().enumerate().map(|(i,r)|serde_json::json!({"ordinal":i,"quantity":format!("{:?}",r.id),"kind":kind(r.kind),"body":r.body,"value_bits":r.value.value().map(|v|format!("{:016x}",v.to_bits())),"class":format!("{:?}",r.class)})).collect::<Vec<_>>()).unwrap());
        println!(
            "I47_NATIVE_IDENTITY {}",
            serde_json::json!({"method":ev.method,"policy":ev.policy,"source":ev.source_encoding,"ledger":ev.ledger_encoding,"run":case.run,"calls":invocation.calls().len()})
        );
        assert!(o
            .operational
            .iter()
            .all(|v| v.work.entered == 23 && !v.work.lost));
        assert!(o.adapter.fault.get().is_none());
        if o.error.is_some() {
            continue;
        }
        if loaded {
            assert!(o.g5a_error.is_none());
        } else {
            assert_eq!(
                o.g5a_error,
                Some(super::retained_product::G5aFailure::Zero {
                    row: e
                        .results
                        .iter()
                        .position(|r| (r.kind.starts_with("element_local_")
                            && (r.kind.contains("force") || r.kind.contains("moment"))
                            || r.kind == "support_reaction_component_v2")
                            && r.value == 0.0
                            && r.value.is_sign_negative())
                        .expect("actual negative-zero mechanical row")
                })
            );
        }
        assert!(!o.full_case_passed());
        if !loaded {
            assert!(o.numeric_pass);
        }
    }
    assert!(
        blockers.is_empty(),
        "selected material final-binding dependency: {blockers:?}"
    );
}

#[test]
fn i47_actual_selection_and_resolver_validity_controls() {
    // Separate actual invocations. No retained owner or resolver result is replayed.
    let raw = i47_selected_specimen(false, true);
    let (point, captured) = observed(raw.clone());
    i47_assert_selected(&captured, false);
    let mut base_changed = raw.clone();
    base_changed["materials"][0]["shear_modulus"]["value"] = serde_json::json!(70e9);
    let unchanged = run_linear_static_preview_value_with_mode(
        base_changed.clone(),
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap();
    assert_eq!(
        serde_json::to_string(&point).unwrap(),
        serde_json::to_string(&unchanged).unwrap()
    );
    println!(
        "I47_CONTROL {}",
        serde_json::json!({"name":"base-G-does-not-substitute","request":base_changed,"unchanged_envelope":unchanged})
    );
    let mut wrong_point = raw.clone();
    wrong_point["model"]["load_cases"][0]["modulus_basis_ref"] =
        serde_json::json!("temperature-point:lower");
    let (different, selected) = observed(wrong_point.clone());
    assert_eq!(different.status.mechanics, "MECHANICS_SOLVED");
    println!("I47_WRONG_POINT_BOUNDARY {:?}", selected.error);
    let k::ProductMaterial::Point { ordinal, e, g } = selected.selections[0].selected else {
        panic!()
    };
    assert_eq!(ordinal, 1);
    assert_eq!(
        selected.selections[0].point_ids.0,
        "temperature-point:lower"
    );
    assert_eq!((e, g), (200000000000.00003, 60000000000.00001));
    let rx = |env: &MechanicsEnvelope| {
        env.results
            .iter()
            .find(|r| r.kind == "global_nodal_rotation_x" && r.entity_ref == "node:N-DEC092-TIP")
            .unwrap()
            .value
            .to_bits()
    };
    assert_ne!(rx(&point), rx(&different));
    println!(
        "I47_CONTROL {}",
        serde_json::json!({"name":"wrong-point-is-observably-distinct","request":wrong_point,"selection":i47_selection_json(&selected),"envelope":different})
    );
    for failure in [
        "missing-alpha",
        "missing-selected-G",
        "unknown-point",
        "at-point-temperature",
    ] {
        let mut invalid = i47_selected_specimen(failure != "unknown-point", true);
        match failure {
            "missing-alpha" => {
                invalid["materials"][0]["temperature_points"][1]
                    .as_object_mut()
                    .unwrap()
                    .remove("thermal_expansion_coefficient");
            }
            "missing-selected-G" => {
                invalid["materials"][0]["temperature_points"][1]
                    .as_object_mut()
                    .unwrap()
                    .remove("shear_modulus");
            }
            "unknown-point" => {
                invalid["model"]["load_cases"][0]["modulus_basis_ref"] =
                    serde_json::json!("temperature-point:missing");
            }
            _ => {
                invalid["model"]["load_cases"][0]["modulus_basis_temperature"]["value"] =
                    serde_json::json!(293.0);
            }
        }
        let (refused, o) = observed(invalid.clone());
        assert_ne!(refused.status.mechanics, "MECHANICS_SOLVED");
        assert!(o.selections.is_empty());
        assert!(o.native.is_none());
        assert_eq!(o.normalized_calls, 1);
        assert_eq!(o.case_calls, 0);
        assert!(!o.full_case_passed());
        println!(
            "I47_REFUSAL {}",
            serde_json::json!({"name":failure,"request":invalid,"envelope":refused,"capture_error":format!("{:?}",o.error),"calls":[o.normalized_calls,o.case_calls,o.final_calls],"adapter":format!("{:?}",o.adapter)})
        );
    }
}

#[test]
fn i47_modulus_record_closed_binding_and_independent_presence() {
    let (e, mut o) = observed(i47_selected_specimen(false, true));
    assert!(o.error.is_none());
    assert!(o.basis_expected);
    assert_eq!(o.basis_record_calls, 1);
    let (invocation, case) = o.native.take().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    let index = e
        .results
        .iter()
        .position(|r| r.kind == "modulus_basis_record")
        .unwrap();
    assert_eq!(
        e.results[index].metadata.as_ref().unwrap().basis,
        o.basis_record.as_ref().unwrap().text
    );
    for (i, row) in e.results.iter().enumerate() {
        if matches!(
            row.kind.as_str(),
            "modulus_basis_record" | "linear_solver_mode_basis"
        ) {
            let v = &o.verdicts[i];
            assert_eq!(v.row, i);
            assert_eq!(v.normalized_bits, row.value.to_bits());
            assert_eq!(v.scale_bits, 0);
            assert_eq!(v.class, None);
            assert_eq!(v.predicates, [None; 4]);
            assert!(v.passed);
        }
    }
    let mutations: [(&str, fn(&mut ResultItem)); 19] = [
        ("id", |r| r.id.push('x')),
        ("kind", |r| r.kind = "foreign_record".into()),
        ("value", |r| r.value = f64::from_bits(1.0f64.to_bits() + 1)),
        ("unit", |r| r.unit = "mode_code".into()),
        ("entity", |r| r.entity_ref.push('x')),
        ("basis-type", |r| {
            r.basis_ref.as_mut().unwrap().ref_type = "material".into()
        }),
        ("basis-id", |r| {
            r.basis_ref.as_mut().unwrap().ref_id.push('x')
        }),
        ("basis-missing", |r| r.basis_ref = None),
        ("metadata-missing", |r| r.metadata = None),
        ("component", |r| {
            r.metadata.as_mut().unwrap().component.push('x')
        }),
        ("coordinate", |r| {
            r.metadata.as_mut().unwrap().coordinate_system.push('x')
        }),
        ("location", |r| {
            r.metadata.as_mut().unwrap().location.push('x')
        }),
        ("sign", |r| {
            r.metadata.as_mut().unwrap().sign_convention.push('x')
        }),
        ("dynamic-same-byte-count", |r| {
            r.metadata.as_mut().unwrap().basis.replace_range(0..1, "X")
        }),
        ("dynamic-length", |r| {
            r.metadata.as_mut().unwrap().basis.push('x')
        }),
        ("dynamic-whitespace", |r| {
            r.metadata.as_mut().unwrap().basis.push(' ')
        }),
        ("dynamic-utf8", |r| {
            r.metadata.as_mut().unwrap().basis.replace_range(0..2, "é")
        }),
        ("source-refs", |r| {
            r.source_result_refs.push("foreign".into())
        }),
        ("same-valued-foreign-case", |r| {
            r.entity_ref = "case:foreign".into();
            r.basis_ref.as_mut().unwrap().ref_id = "case:foreign".into();
            r.metadata.as_mut().unwrap().location = "case:foreign".into();
        }),
    ];
    for (name, mutate) in mutations {
        let mut changed = e.clone();
        mutate(&mut changed.results[index]);
        assert!(o.bind_rows(&changed, owner).is_err(), "{name}");
        println!("I47_FIXED_MUTATION {name} refused");
    }
    let mut missing = e.clone();
    missing.results.remove(index);
    assert!(o.bind_rows(&missing, owner).is_err());
    let capture = o.basis_record.take().unwrap();
    assert!(o.bind_rows(&e, owner).is_err());
    o.basis_record_calls = 0;
    assert!(
        o.bind_rows(&missing, owner).is_err(),
        "missing both cannot waive selection"
    );
    o.basis_record = Some(capture);
    o.basis_record_calls = 1;
    o.bind_rows(&e, owner).unwrap();
    for new_id in [false, true] {
        let mut changed = e.clone();
        let mut copy = e.results[index].clone();
        if new_id {
            copy.id.push_str("-duplicate");
        }
        changed.results.push(copy);
        assert!(o.bind_rows(&changed, owner).is_err());
    }
    let mut no_mode = e.clone();
    no_mode
        .results
        .retain(|r| r.kind != "linear_solver_mode_basis");
    let rows = o.bind_rows(&no_mode, owner).unwrap();
    let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
    assert_eq!(spent.failure().unwrap().category(), "association");
    // Direct typed mutations isolate the FK maximum-coverage rule from PP fixed-id refusal.
    for duplicate in [
        k::ProductRecipe::ModulusBasisRecord,
        k::ProductRecipe::NonQuantity,
    ] {
        let mut rows = o.bind_rows(&e, owner).unwrap();
        let i = rows.iter().position(|r| r.recipe == duplicate).unwrap();
        let r = &rows[i];
        let copy = k::ProductFinalRow {
            id: "independent-duplicate-id",
            case_id: r.case_id,
            value: r.value,
            unit: r.unit,
            body: r.body,
            recipe: r.recipe,
        };
        rows.push(copy);
        let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
        assert_eq!(spent.failure().unwrap().category(), "association");
    }
    let mut rows = o.bind_rows(&e, owner).unwrap();
    rows[index].unit = k::ProductUnit::Pascal;
    assert_eq!(
        invocation
            .certify_product_case(case.run, owner, &o.facts, &rows)
            .failure()
            .unwrap()
            .category(),
        "association"
    );
    let (mut base, mut b) = observed(specimen(true));
    assert!(!b.basis_expected && b.basis_record.is_none() && b.basis_record_calls == 0);
    let (_, bc) = b.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(bo) = &bc.outcome else {
        panic!()
    };
    base.results.push(e.results[index].clone());
    assert!(b.bind_rows(&base, bo).is_err());
    base.results.pop();
    b.basis_record = o.basis_record.take();
    b.basis_record_calls = 1;
    assert!(
        b.bind_rows(&base, bo).is_err(),
        "base with unexpected capture"
    );
    o.basis_record = b.basis_record.take();

    let (other, _) = observed(i47_selected_specimen(true, true));
    let mut wrong_selection = e.clone();
    wrong_selection.results[index] = other
        .results
        .iter()
        .find(|r| r.kind == "modulus_basis_record")
        .unwrap()
        .clone();
    assert!(o.bind_rows(&wrong_selection, owner).is_err());
    println!("I47_COVERAGE_CONTROLS independent selected presence; closed ancillary types; mandatory mode: passed");
}

#[test]
fn i47_successful_aggregate_capture_sticky_errors_and_work_prefixes() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, AdapterWork, CaptureError};
    let raw = i47_selected_specimen(false, true);
    let (request, _) = source_receipt::CapturedInvocation::parse(
        raw.clone(),
        PreviewSolverMode::SparseInteractive,
    )
    .unwrap();
    let case = &request.model.load_cases[0];
    let (_, mut o) = observed(raw.clone());
    let text = o.basis_record.as_ref().unwrap().text.clone();
    let before = o.adapter.counts.get();
    o.successful_basis_record(case, &text);
    assert!(o
        .error
        .as_ref()
        .unwrap()
        .to_string()
        .contains("duplicate successful"));
    assert_eq!(o.basis_record_calls, 2);
    let after = o.adapter.counts.get();
    assert_eq!(
        after[E::SourceVisit as usize],
        before[E::SourceVisit as usize] + 1
    );
    assert_eq!(
        after[E::ValidationEntry as usize],
        before[E::ValidationEntry as usize] + 1
    );
    assert_eq!(
        after[E::MapWrite as usize],
        before[E::MapWrite as usize] + 1
    );
    assert_eq!(
        after[E::RequestedCopyBytes as usize],
        before[E::RequestedCopyBytes as usize]
    );
    let first = format!("{:?}", o.error);
    o.successful_basis_record(case, "ignored");
    assert_eq!(format!("{:?}", o.error), first);
    assert_eq!(o.adapter.counts.get(), after);
    // Synthetic fresh hook owner made from actual selections; no resolver replay and
    // never used as an actual invocation witness. Exercise individual new hook exits.
    let fresh = || {
        let (_, mut x) = observed(raw.clone());
        x.basis_record = None;
        x.basis_record_calls = 0;
        x.adapter = AdapterWork::default();
        x
    };
    let mut foreign = fresh();
    let mut foreign_case = case.clone();
    foreign_case.id = "case:foreign".into();
    foreign.successful_basis_record(&foreign_case, &text);
    assert!(matches!(foreign.error, Some(CaptureError::Association(_))));
    assert!(foreign.basis_record.is_none());
    let mut mismatch = fresh();
    let mut wrong_selector = case.clone();
    wrong_selector.modulus_basis_ref = Some("temperature-point:lower".into());
    mismatch.successful_basis_record(&wrong_selector, &text);
    assert!(mismatch.error.is_some());
    for event in [
        E::ValidationEntry,
        E::SourceVisit,
        E::MapWrite,
        E::KeyProbe,
        E::IdentityByteRead,
        E::AllocationRequest,
        E::RequestedCopyBytes,
        E::RustCapacityBytes,
    ] {
        let mut x = fresh();
        let mut counts = x.adapter.counts.get();
        counts[event as usize] = u64::MAX;
        x.adapter.counts.set(counts);
        x.successful_basis_record(case, &text);
        assert!(
            matches!(x.error,Some(CaptureError::Accounting(AdapterFault::Overflow(e))) if e==event),
            "{event:?}: {:?}",
            x.error
        );
        assert!(x.basis_record.is_none());
        assert_eq!(x.adapter.counts.get()[event as usize], u64::MAX);
        let prefix = x.adapter.counts.get();
        x.successful_basis_record(case, &text);
        assert_eq!(x.adapter.counts.get(), prefix);
        println!("I47_HOOK_ACCOUNTING {event:?} {prefix:?}");
    }
    let mut count = fresh();
    count.basis_record_calls = usize::MAX;
    count.successful_basis_record(case, &text);
    assert!(matches!(
        count.error,
        Some(CaptureError::CountRange("basis record calls"))
    ));
    assert_eq!(count.adapter.counts.get()[E::MapWrite as usize], 0);
    let mut good = fresh();
    good.successful_basis_record(case, &text);
    assert!(good.error.is_none());
    let expected_bytes =
        (case.id.len() + case.modulus_basis_ref.as_ref().unwrap().len() + text.len()) as u64;
    assert_eq!(good.adapter.counts.get()[E::AllocationRequest as usize], 3);
    assert_eq!(
        good.adapter.counts.get()[E::RequestedCopyBytes as usize],
        expected_bytes
    );
    assert_eq!(
        good.adapter.counts.get()[E::RustCapacityBytes as usize],
        (good.basis_record.as_ref().unwrap().case.capacity()
            + good.basis_record.as_ref().unwrap().text.capacity()
            + match &good.basis_record.as_ref().unwrap().selector {
                super::retained_product::MaterialSelector::Point(s) => s.capacity(),
                _ => 0,
            }) as u64
    );
    println!("I47_HOOK_SUCCESS {:?}", good.adapter.counts.get());
    // Actual aggregate failure after one valid material selection: no successful
    // aggregate record may be captured. Failure happens before building either pipe.
    let mut invalid = i47_selected_specimen(true, true);
    let mut material = invalid["materials"][0].clone();
    material["id"] = serde_json::json!("material:missing-alpha");
    material["temperature_points"][1]
        .as_object_mut()
        .unwrap()
        .remove("thermal_expansion_coefficient");
    invalid["materials"].as_array_mut().unwrap().push(material);
    let mut pipe = invalid["model"]["pipe_segments"][0].clone();
    pipe["id"] = serde_json::json!("pipe:invalid-second");
    pipe["material"] = serde_json::json!("material:missing-alpha");
    invalid["model"]["pipe_segments"]
        .as_array_mut()
        .unwrap()
        .push(pipe);
    let (refused, partial) = observed(invalid.clone());
    assert_ne!(refused.status.mechanics, "MECHANICS_SOLVED");
    assert_eq!(partial.selections.len(), 1);
    assert!(partial.basis_record.is_none());
    assert_eq!(partial.basis_record_calls, 0);
    assert_eq!(partial.case_calls, 0);
    assert_eq!(partial.final_calls, 0);
    println!(
        "I47_AGGREGATE_REFUSAL {}",
        serde_json::json!({"request":invalid,"envelope":refused,"selections":partial.selections.len(),"basis_calls":partial.basis_record_calls,"adapter":format!("{:?}",partial.adapter)})
    );
}

#[test]
fn i47_modulus_binding_accounting_failure_remains_typed() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, AdapterWork, CaptureError};
    let (e, o) = observed(i47_selected_specimen(false, true));
    let (_, case) = o.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    for event in [
        E::ValidationEntry,
        E::RowVisit,
        E::KeyProbe,
        E::IdentityByteRead,
        E::AllocationRequest,
        E::RustCapacityBytes,
        E::MapWrite,
        E::LibraryBoundary,
    ] {
        o.adapter.counts.set([0; 10]);
        o.adapter.fault.set(None);
        let mut seed = [0; 10];
        seed[event as usize] = u64::MAX;
        o.adapter.counts.set(seed);
        assert!(
            matches!(o.bind_rows(&e,owner),Err(CaptureError::Accounting(AdapterFault::Overflow(actual))) if actual==event)
        );
        let prefix = o.adapter.counts.get();
        assert_eq!(prefix[event as usize], u64::MAX);
        assert!(o.bind_rows(&e, owner).is_err());
        assert_eq!(o.adapter.counts.get(), prefix);
        println!("I47_BINDING_ACCOUNTING {event:?} {prefix:?}");
    }
    // Supported allocation/count refusal: existing layout helper rejects before allocator entry.
    let layout = AdapterWork::default();
    assert!(matches!(
        layout.reserve::<u64>(usize::MAX),
        Err(CaptureError::CountRange(_))
    ));
    assert_eq!(layout.counts.get(), [0; 10]);
}
