use super::retained_product::ProductCapture;
use super::*;
use open_pipe_stress_frame_kernel::structural::retained_api as k;
#[test]
fn i50_bitmap_fill_counts_and_after_reserve_exhaustion() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, CaptureError};
    // One reserve enters allocation, capacity-record MapWrite and its actual
    // try_reserve boundary. The owned fill adds one validation and exactly n
    // MapWrites; no resize/bulk operation exists, including for n=0.
    for n in [0usize, 1, 4, 3, 12] {
        let mut o = ProductCapture::default();
        let mut bitmap = o.support_reserve::<bool>(n, 0).unwrap();
        o.fill_support_bitmap(&mut bitmap, n).unwrap();
        assert_eq!(bitmap, vec![false; n]);
        assert_eq!(
            o.adapter.counts.get(),
            [0, 0, 1 + n as u64, 1, 0, 0, 1, 1, 0, n as u64]
        );
        assert_eq!(o.support_capacity_bytes, [n, 0, 0, 0, 0, 0, 0]);
        println!(
            "I50_BITMAP_SUCCESS n={n} counts={:?} capacities={:?}",
            o.adapter.counts.get(),
            o.support_capacity_bytes
        );
    }
    let request: LinearStaticPreviewRequest = serde_json::from_value(i50_named_request()).unwrap();
    let mut diagnostics = Vec::new();
    let built = build_model(&request.model, &request.model.materials, &mut diagnostics).unwrap();
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    assert!(diagnostics.is_empty() && boundary.findings.is_empty());
    assert_eq!(built.supports.len(), 4);
    let mut o = ProductCapture::default();
    let mut parts = k::SourceParts::default();
    let mut seed = [0u64; 10];
    seed[E::MapWrite as usize] = u64::MAX - 1;
    o.adapter.counts.set(seed);
    let error = o
        .capture_supports(
            &request.model,
            &built,
            &boundary.restrained_dofs,
            &boundary.springs,
            &mut parts,
        )
        .unwrap_err();
    assert!(matches!(
        error,
        CaptureError::Accounting(AdapterFault::Overflow(E::MapWrite))
    ));
    // Four source-count checks, one fill shape check; first reserve succeeds,
    // first element write cannot enter, and the second allocation is untouched.
    let expected = [0, 0, u64::MAX, 5, 0, 0, 1, 1, 0, 4];
    assert_eq!(o.adapter.counts.get(), expected);
    assert_eq!(o.support_capacity_bytes, [4, 0, 0, 0, 0, 0, 0]);
    assert!(
        o.supports.is_empty()
            && o.support_fixed.is_empty()
            && o.spring_map.is_empty()
            && o.source.is_none()
    );
    assert!(parts.supports.is_empty() && parts.springs.is_empty());
    let again = o
        .capture_supports(
            &request.model,
            &built,
            &boundary.restrained_dofs,
            &boundary.springs,
            &mut parts,
        )
        .unwrap_err();
    assert!(matches!(
        again,
        CaptureError::Accounting(AdapterFault::Overflow(E::MapWrite))
    ));
    assert_eq!(o.adapter.counts.get(), expected);
    assert_eq!(o.support_capacity_bytes, [4, 0, 0, 0, 0, 0, 0]);
    println!("I50_BITMAP_AFTER_RESERVE prefix={expected:?} first_capacity=4 second_allocations=0 committed_maps=0");

    let mut o = ProductCapture::default();
    let mut bitmap = o.support_reserve::<bool>(4, 0).unwrap();
    let mut seeded = o.adapter.counts.get();
    seeded[E::MapWrite as usize] = u64::MAX - 2;
    o.adapter.counts.set(seeded);
    assert!(matches!(
        o.fill_support_bitmap(&mut bitmap, 4),
        Err(CaptureError::Accounting(AdapterFault::Overflow(
            E::MapWrite
        )))
    ));
    assert_eq!(bitmap, [false, false]);
    let expected = [0, 0, u64::MAX, 1, 0, 0, 1, 1, 0, 4];
    assert_eq!(o.adapter.counts.get(), expected);
    assert_eq!(o.support_capacity_bytes, [4, 0, 0, 0, 0, 0, 0]);
    assert!(matches!(
        o.fill_support_bitmap(&mut bitmap, 4),
        Err(CaptureError::Accounting(AdapterFault::Overflow(
            E::MapWrite
        )))
    ));
    assert_eq!(bitmap, [false, false]);
    assert_eq!(o.adapter.counts.get(), expected);
    println!("I50_BITMAP_MID_FILL prefix={expected:?} written=2 reserved=4");

    // Only two repeats, supplied directly to the isolated helper. The actual
    // prepared production boundary was already unique; this is hardening.
    let mut o = ProductCapture::default();
    let mut parts = k::SourceParts::default();
    let error = o
        .capture_supports(
            &request.model,
            &built,
            &[0, 0],
            &boundary.springs,
            &mut parts,
        )
        .unwrap_err();
    assert!(matches!(error, CaptureError::Association(ref s) if s == "rigid boundary identity"));
    assert!(o.supports.is_empty() && parts.supports.is_empty() && o.source.is_none());
    println!("I50_TWO_REPEAT_BOUNDARY {error:?}");
}

fn i50_named_request() -> serde_json::Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
    ))
    .unwrap()
}
fn i50_dump(e: &MechanicsEnvelope, o: &ProductCapture, mode: PreviewSolverMode) {
    let verdicts: Vec<_> = o.verdicts.iter().map(|v| serde_json::json!({
        "row":v.row,"normalized_bits":format!("{:016x}",v.normalized_bits),"scale_bits":format!("{:016x}",v.scale_bits),
        "class":format!("{:?}",v.class),"passed":v.passed,"failed":format!("{:?}",v.failed),"predicates":v.predicates
    })).collect();
    let source = o.source.as_ref().map(|s| serde_json::json!({"nodes":s.nodes(),
        "members":s.members().iter().map(|m|serde_json::json!({"id":m.id,"nodes":[m.node_i,m.node_j],"E":m.elastic_modulus,"G":m.shear_modulus,"A":m.area,"I":m.second_moment_y,"J":m.torsion_constant,"y_reference":m.y_reference})).collect::<Vec<_>>(),
        "springs":s.springs().iter().map(|s|serde_json::json!({"id":s.id,"node":s.dof.node,"axis":s.dof.component.index(),"k":s.stiffness})).collect::<Vec<_>>(),
        "supports":s.supports().iter().map(|s|serde_json::json!({"id":s.id,"node":s.node,"rigid":s.restrained,"springs":s.springs})).collect::<Vec<_>>() }));
    let native = o.native.as_ref().map(|(invocation,case)| match &case.outcome {
        k::ExecutionOutcome::Selected(owner) => {
            let ev=owner.evidence();
            serde_json::json!({"calls":invocation.calls().len(),"run":case.run,"precision":owner.selected_precision(),
                "source_encoding":ev.source_encoding,"ledger_encoding":ev.ledger_encoding,
                "resolution":ev.resolution_scale.iter().map(|(b,f,m)|serde_json::json!([b,format!("{f:016x}"),format!("{m:016x}")])).collect::<Vec<_>>(),
                "stop":format!("{:?}",ev.stop_rule),"estimate":format!("{:?}",ev.verification_estimate),"charge":format!("{:?}",ev.verification_charge),
                "rows":owner.publish().rows.iter().enumerate().map(|(i,r)|serde_json::json!({"ordinal":i,"id":format!("{:?}",r.id),"kind":format!("{:?}",r.kind),"body":r.body,"value_bits":r.value.value().map(|v|format!("{:016x}",v.to_bits())),"class":format!("{:?}",r.class)})).collect::<Vec<_>>()})
        }, other => serde_json::json!({"unavailable":format!("{other:?}")})
    });
    println!(
        "I50_RECORD {}",
        serde_json::json!({"mode":mode.as_str(),"request":i50_named_request(),"envelope":e,"source":source,"native":native,
        "facts":o.facts.iter().map(|f|serde_json::json!({"D":f.diameter,"t":f.effective_wall,"A":f.area,"I":f.second_moment,"J":f.torsion_constant,"Z":f.section_modulus,"c":f.radius})).collect::<Vec<_>>(),
        "verdicts":verdicts,"error":format!("{:?}",o.error),"numeric_failure":format!("{:?}",o.numeric_failure),"numeric_pass":o.numeric_pass,
        "g5a":format!("{:?}",o.g5a_error),"observable":format!("{:?}",o.observable_error),"full_case":o.full_case_passed(),
        "hooks":[o.normalized_calls,o.case_calls,o.final_calls],"observation_calls":o.observation_calls,"invocation_calls":o.invocation_calls,
        "observations":o.observations.as_ref().map(|v|serde_json::json!({"case":v.case,"mode":v.mode.as_str(),"mode_bits":format!("{:016x}",v.mode_row.value_bits),"mode_basis":v.mode_row.basis,"parity_produced":v.parity_produced,"parity":v.parity.as_ref().map(|p|serde_json::json!({"bits":format!("{:016x}",p.value_bits),"basis":p.basis}))})),"observation_capacity_bytes":o.observation_capacity_bytes,"adapter":format!("{:?}",o.adapter),"work":o.work,
        "g5a_work":format!("{:?}",o.g5a_work),"operational":format!("{:?}",o.operational),"support_capacity_bytes":o.support_capacity_bytes,"spring_map":format!("{:?}",o.spring_map)})
    );
}
#[test]
fn i50_actual_named_case_both_modes_complete_private_verdict() {
    let mut complete = true;
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        let raw = i50_named_request();
        let baseline = run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap();
        let (e, o) = observed_mode(raw, mode);
        i50_dump(&e, &o, mode);
        assert_eq!(
            serde_json::to_string(&e).unwrap(),
            serde_json::to_string(&baseline).unwrap()
        );
        assert_eq!(e.producer.semantic_contract_id, preview_physics::ID);
        assert!(e.source_block_recovery.is_none());
        assert_eq!(
            e.numerical_quality.cases[0].solve_quality,
            NumericalQualityStatus::Sensitive
        );
        assert_eq!(
            e.results.len(),
            if mode == PreviewSolverMode::SparseInteractive {
                98
            } else {
                99
            }
        );
        assert_eq!((o.normalized_calls, o.case_calls, o.final_calls), (1, 1, 1));
        assert_eq!((o.invocation_calls, o.observation_calls), (1, 1));
        assert_eq!(o.invocation_mode, Some(mode));
        assert_eq!(
            o.observations.as_ref().unwrap().parity_produced,
            mode == PreviewSolverMode::DenseScrutiny
        );
        assert!(!o.request_materials);
        complete &=
            o.error.is_none() && o.numeric_failure.is_none() && o.verdicts.len() == e.results.len();
        if let Some((invocation, case)) = &o.native {
            assert_eq!(invocation.calls().len(), 1);
            if let k::ExecutionOutcome::Selected(owner) = &case.outcome {
                assert_eq!(owner.publish().rows.len(), 58);
            }
        }
    }
    assert!(
        complete,
        "actual incomplete prefixes retained in I50_RECORD"
    );
}
#[test]
fn i50_actual_support_bijections_and_accounting_prefixes() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, CaptureError};
    let make = || {
        let request: LinearStaticPreviewRequest =
            serde_json::from_value(i50_named_request()).unwrap();
        let mut diagnostics = Vec::new();
        let built =
            build_model(&request.model, &request.model.materials, &mut diagnostics).unwrap();
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        assert!(diagnostics.is_empty() && boundary.findings.is_empty());
        (request.model, built, boundary)
    };
    let fresh = |g| {
        let mut o = ProductCapture::default();
        let mut parts = k::SourceParts::default();
        o.supports = o.adapter.reserve(g).unwrap();
        parts.supports = o.adapter.reserve(g).unwrap();
        (o, parts)
    };
    for name in [
        "duplicate built",
        "missing built",
        "extra built",
        "equal-k ids",
        "node",
        "axis",
        "dimension",
        "k bit",
        "duplicate boundary",
        "missing boundary",
        "foreign boundary",
        "spring rigid",
    ] {
        let (model, mut built, mut boundary) = make();
        match name {
            "duplicate built" => built.supports.push(built.supports[1].clone()),
            "missing built" => {
                built.supports.remove(1);
            }
            "extra built" => {
                let mut b = built.supports[1].clone();
                b.support_id = "foreign".into();
                built.supports.push(b);
            }
            "equal-k ids" => {
                let id = boundary.springs[1].support_id.clone();
                boundary.springs[1].support_id = boundary.springs[2].support_id.clone();
                boundary.springs[2].support_id = id;
            }
            "node" => boundary.springs[1].node_dof.node_index = 1,
            "axis" => boundary.springs[1].node_dof.dof = FrameDof::Rx,
            "dimension" => {
                boundary.springs[1].stiffness.dimension = QuantityDimension::Displacement
            }
            "k bit" => boundary.springs[1].stiffness.value = f64::from_bits(1e6f64.to_bits() + 1),
            "duplicate boundary" => boundary.springs.push(boundary.springs[1].clone()),
            "missing boundary" => {
                boundary.springs.pop();
            }
            "foreign boundary" => {
                let mut b = boundary.springs[1].clone();
                b.support_id = "foreign".into();
                boundary.springs.push(b);
            }
            "spring rigid" => built.supports[1].family = SupportFamily::Guide,
            _ => unreachable!(),
        }
        let (mut o, mut parts) = fresh(model.supports.len());
        let e = o
            .capture_supports(
                &model,
                &built,
                &boundary.restrained_dofs,
                &boundary.springs,
                &mut parts,
            )
            .unwrap_err();
        assert!(matches!(e, CaptureError::Association(_)), "{name}: {e:?}");
        let expected = match name {
            "duplicate built" => "duplicate built support",
            "missing built" => "missing built support",
            "extra built" | "foreign boundary" => "unconsumed built/boundary support",
            "duplicate boundary" => "duplicate/extra support spring",
            "missing boundary" => "missing/foreign support stiffness",
            "spring rigid" => "support build family/axes",
            _ => "spring source identity",
        };
        assert!(format!("{e:?}").contains(expected), "{name}: {e:?}");
        println!(
            "I50_SUPPORT_MUTATION {name} {e:?} {:?} {:?}",
            o.adapter, o.support_capacity_bytes
        );
    }
    let (model, mut built, mut boundary) = make();
    built.supports.reverse();
    boundary.springs.reverse();
    let (mut o, mut parts) = fresh(model.supports.len());
    o.capture_supports(
        &model,
        &built,
        &boundary.restrained_dofs,
        &boundary.springs,
        &mut parts,
    )
    .unwrap();
    o.check_support_maps(&parts.supports, &parts.springs, false)
        .unwrap();
    assert_eq!(
        o.spring_map
            .iter()
            .map(|m| (m.support, m.boundary))
            .collect::<Vec<_>>(),
        vec![(1, 2), (2, 1), (3, 0)]
    );
    assert_eq!(parts.supports[1].restrained, [false; 6]);
    let duplicate = parts.supports[1].springs[0];
    parts.supports[1].springs.push(duplicate);
    assert!(
        o.check_support_maps(&parts.supports, &parts.springs, false)
            .is_err(),
        "duplicate before canonical dedup"
    );
    parts.supports[1].springs.pop();
    let (_, actual) = observed(i50_named_request());
    let source = actual.source.unwrap();
    parts.nodes = source.nodes().to_vec();
    parts.members = source.members().to_vec();
    parts.constraints = source.constraints().to_vec();
    parts.loads = source.loads().to_vec();
    parts.stations = source.stations().to_vec();
    let canonical = k::PrimitiveSource::new(parts).unwrap();
    o.check_support_maps(canonical.supports(), canonical.springs(), true)
        .unwrap();
    let (mut model, mut built, boundary) = make();
    let mut repeated = model.supports[0].clone();
    repeated.id = "i50:duplicate-rigid".into();
    let mut repeated_built = built.supports[0].clone();
    repeated_built.support_id = repeated.id.clone();
    model.supports.push(repeated);
    built.supports.push(repeated_built);
    let (mut o, mut parts) = fresh(model.supports.len());
    assert!(format!(
        "{:?}",
        o.capture_supports(
            &model,
            &built,
            &boundary.restrained_dofs,
            &boundary.springs,
            &mut parts
        )
        .unwrap_err()
    )
    .contains("ambiguous rigid ownership"));
    // The public request validator may stop before this seam. A producing-map
    // control checks that the adapter itself retains, rather than drops, zero k.
    let (mut model, mut built, mut boundary) = make();
    model.supports[1].stiffness.as_mut().unwrap().value.value = 0.0;
    built.supports[1].stiffness.as_mut().unwrap().value = 0.0;
    boundary.springs[0].stiffness.value = 0.0;
    let (mut zero, mut parts) = fresh(model.supports.len());
    zero.capture_supports(
        &model,
        &built,
        &boundary.restrained_dofs,
        &boundary.springs,
        &mut parts,
    )
    .unwrap();
    assert_eq!(parts.springs.len(), 3);
    assert_eq!(parts.springs[0].stiffness, 0.0);
    parts.nodes = source.nodes().to_vec();
    parts.members = source.members().to_vec();
    parts.constraints = source.constraints().to_vec();
    parts.loads = source.loads().to_vec();
    parts.stations = source.stations().to_vec();
    assert!(matches!(
        k::PrimitiveSource::new(parts),
        Err(k::SourceError::NonPositiveSpring { id: 0 })
    ));
    for event in [
        E::SourceVisit,
        E::MapWrite,
        E::ValidationEntry,
        E::IdentityByteRead,
        E::KeyProbe,
        E::AllocationRequest,
        E::LibraryBoundary,
        E::RequestedCopyBytes,
        E::RustCapacityBytes,
    ] {
        let (model, built, boundary) = make();
        let (mut o, mut parts) = fresh(model.supports.len());
        let mut counts = o.adapter.counts.get();
        counts[event as usize] = u64::MAX;
        o.adapter.counts.set(counts);
        let e = o
            .capture_supports(
                &model,
                &built,
                &boundary.restrained_dofs,
                &boundary.springs,
                &mut parts,
            )
            .unwrap_err();
        assert!(
            matches!(e,CaptureError::Accounting(AdapterFault::Overflow(actual)) if actual==event),
            "{event:?}: {e:?}"
        );
        let prefix = o.adapter.counts.get();
        assert!(o
            .capture_supports(
                &model,
                &built,
                &boundary.restrained_dofs,
                &boundary.springs,
                &mut parts
            )
            .is_err());
        assert_eq!(prefix, o.adapter.counts.get());
        println!(
            "I50_SUPPORT_PREFIX {event:?} {prefix:?} {:?}",
            o.support_capacity_bytes
        );
    }
    let mut o = ProductCapture::default();
    assert!(matches!(
        o.support_reserve::<u64>(usize::MAX, 0),
        Err(CaptureError::CountRange(_))
    ));
    assert_eq!(o.adapter.counts.get()[E::AllocationRequest as usize], 0);
}

#[test]
fn i50_support_coverage_native_non_aliasing_and_g5a() {
    use super::retained_product::{G5aFailure, ScalarWork};
    let (e, mut o) = observed(i50_named_request());
    let (invocation, case) = o.native.take().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    let empty = k::ProductRecipe::SupportComponent {
        support: 0,
        component: k::Component::Rx,
    };
    for name in [
        "missing empty",
        "duplicate empty",
        "replace reaction",
        "extra reaction",
        "replace spring",
        "wrong unit",
        "wrong group",
    ] {
        let mut rows = o.bind_rows(&e, owner).unwrap();
        let ei = rows.iter().position(|r| r.recipe == empty).unwrap();
        let ri = rows
            .iter()
            .position(|r| {
                r.recipe
                    == k::ProductRecipe::SupportComponent {
                        support: 0,
                        component: k::Component::Ux,
                    }
            })
            .unwrap();
        match name {
            "missing empty" => {
                rows.remove(ei);
            }
            "duplicate empty" => {
                let r = &rows[ei];
                let x = k::ProductFinalRow {
                    id: "i50:duplicate-empty",
                    case_id: r.case_id,
                    value: r.value,
                    unit: r.unit,
                    body: r.body,
                    recipe: r.recipe,
                };
                rows.push(x);
            }
            "replace reaction" => {
                rows[ri].recipe = k::ProductRecipe::Native(k::QuantityId::Reaction(k::Dof {
                    node: 0,
                    component: k::Component::Ux,
                }))
            }
            "extra reaction" => {
                let r = &rows[ri];
                let x = k::ProductFinalRow {
                    id: "i50:extra-native",
                    case_id: r.case_id,
                    value: r.value,
                    unit: r.unit,
                    body: r.body,
                    recipe: k::ProductRecipe::Native(k::QuantityId::Reaction(k::Dof {
                        node: 0,
                        component: k::Component::Ux,
                    })),
                };
                rows.push(x);
            }
            "replace spring" => {
                let i = rows
                    .iter()
                    .position(|r| {
                        r.recipe
                            == k::ProductRecipe::SupportComponent {
                                support: 1,
                                component: k::Component::Rx,
                            }
                    })
                    .unwrap();
                rows[i].recipe = k::ProductRecipe::Native(k::QuantityId::SpringAction {
                    spring: 0,
                    component: k::Component::Rx,
                });
            }
            "wrong unit" => rows[ei].unit = k::ProductUnit::Newton,
            "wrong group" => {
                rows[ei].recipe = k::ProductRecipe::SupportComponent {
                    support: 99,
                    component: k::Component::Rx,
                }
            }
            _ => unreachable!(),
        }
        let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
        let error = spent.failure().unwrap();
        assert_eq!(error.category(), "association", "{name}");
        let why = format!("{error:?}");
        if name.starts_with("replace") || name == "missing empty" {
            assert!(why.contains("missing support coverage"), "{name}: {why}");
        }
        if name == "extra reaction" {
            assert!(why.contains("native coverage"));
        }
        println!("I50_COVERAGE_MUTATION {name} {why}");
    }
    // Isolated synthetic final value: largest empty force component must affect
    // the FK final scale and PP G5a maximum, without changing the real witness.
    for (entity, component, expected_scale) in
        [("spring:N0:0", "Fx", 1e12f64), ("rigid:N0", "Mx", 1e12f64)]
    {
        let mut changed = e.clone();
        let index = changed
            .results
            .iter()
            .position(|r| {
                r.entity_ref == entity
                    && r.kind == "support_reaction_component_v2"
                    && r.metadata.as_ref().unwrap().component == component
            })
            .unwrap();
        changed.results[index].value = 1e12;
        let rows = o.bind_rows(&changed, owner).unwrap();
        let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
        assert!(spent.failure().is_none());
        assert_eq!(spent.verdicts()[index].scale_bits, expected_scale.to_bits());
        assert!(!spent.verdicts()[index].passed);
        o.verdicts = spent.verdicts().to_vec();
        o.g5a_work = ScalarWork::default();
        assert!(matches!(
            o.g5a(owner, &rows),
            Err(G5aFailure::Sanity { .. })
        ));
        println!(
            "I50_LARGEST_SUPPORT {component} FK_scale={} G5a_sanity_refusal",
            f64::from_bits(spent.verdicts()[index].scale_bits)
        );
    }
    // The newly entered G5a support scans preserve typed accounting and stop
    // before further work, including on a repeated call after the first fault.
    let rows = o.bind_rows(&e, owner).unwrap();
    let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
    o.verdicts = spent.verdicts().to_vec();
    for event in [
        super::retained_product::AdapterEvent::RowVisit,
        super::retained_product::AdapterEvent::ValidationEntry,
    ] {
        let before = o.adapter.counts.get();
        let mut seeded = before;
        seeded[event as usize] = u64::MAX;
        o.adapter.counts.set(seeded);
        o.g5a_work = ScalarWork::default();
        assert!(
            matches!(o.g5a(owner, &rows), Err(G5aFailure::Accounting(super::retained_product::AdapterFault::Overflow(actual))) if actual == event)
        );
        let prefix = (
            o.adapter.counts.get(),
            o.g5a_work.entered,
            o.g5a_work.checks,
        );
        assert!(o.g5a(owner, &rows).is_err());
        assert_eq!(
            prefix,
            (
                o.adapter.counts.get(),
                o.g5a_work.entered,
                o.g5a_work.checks
            )
        );
        println!("I50_G5A_PREFIX {event:?} {prefix:?}");
        o.adapter.fault.set(None);
        o.adapter.counts.set(before);
    }
    // A separate zero-load anchored specimen supplies zero uncoupled resolution.
    // This synthetic sign control is not a named-case availability witness.
    let mut raw = specimen(false);
    let node = raw["model"]["nodes"][1]["id"].as_str().unwrap().to_owned();
    raw["model"]["supports"].as_array_mut().unwrap().push(serde_json::json!({"id":"i50:zero-spring","node":node,"family":"spring","restraints":["UX"],"stiffness":{"dof":"UX","value":{"value":1.0,"unit":"N/m"}},"provenance":"invented isolated I50 G5a control"}));
    let (mut e, mut o) = observed(raw);
    let (invocation, case) = o.native.take().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    for row in &mut e.results {
        if row.value == 0.0 {
            row.value = 0.0;
        }
    }
    let i = e
        .results
        .iter()
        .position(|r| {
            r.entity_ref == "i50:zero-spring"
                && r.kind == "support_reaction_component_v2"
                && r.metadata.as_ref().unwrap().component == "Fy"
        })
        .unwrap();
    e.results[i].value = -0.0;
    let rows = o.bind_rows(&e, owner).unwrap();
    let spent = invocation.certify_product_case(case.run, owner, &o.facts, &rows);
    assert!(spent.failure().is_none(), "{:?}", spent.failure());
    assert!(spent.verdicts()[i].passed);
    assert!(matches!(
        spent.verdicts()[i].class,
        Some(k::RowClass::AbsoluteVerified { .. })
    ));
    o.verdicts = spent.verdicts().to_vec();
    o.g5a_work = ScalarWork::default();
    assert_eq!(o.g5a(owner, &rows), Err(G5aFailure::Zero { row: i }));
    println!("I50_EMPTY_NEGATIVE_ZERO row={i} gate=pass g5a=zero-refusal");
}

#[test]
fn i50_observation_custody_presence_fields_and_failure_prefixes() {
    use super::retained_product::{AdapterEvent as E, AdapterFault, CaptureError};
    let raw = i50_named_request();
    let (request, capture) =
        source_receipt::CapturedInvocation::parse(raw.clone(), PreviewSolverMode::DenseScrutiny)
            .unwrap();
    let case = &request.model.load_cases[0];
    let (e, actual) = observed_mode(raw, PreviewSolverMode::DenseScrutiny);
    assert!(actual.error.is_none());
    let prefix: Vec<_> = e
        .results
        .iter()
        .filter(|r| {
            matches!(
                r.kind.as_str(),
                "linear_solver_mode_basis" | "sparse_live_path_dense_parity_relative_delta"
            )
        })
        .cloned()
        .map(|mut r| {
            r.basis_ref = None;
            r
        })
        .collect();
    assert_eq!(prefix.len(), 2);
    // Fresh synthetic seam owners and copies of actual immutable producing
    // fields isolate custody failures; these are not new runtime witnesses.
    let fresh = || {
        let mut o = ProductCapture::default();
        o.invocation(Some(&capture), PreviewSolverMode::DenseScrutiny);
        o.case_id = case.id.clone();
        o.case_calls = 1;
        o
    };
    let captured = || {
        let mut o = fresh();
        o.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
        assert!(o.error.is_none());
        o
    };
    for name in [
        "missing mode",
        "duplicate mode",
        "duplicate parity",
        "wrong mode",
        "fallback",
        "foreign case",
        "foreign mode",
    ] {
        let mut o = fresh();
        let mut rows = prefix.clone();
        let mut other = case.clone();
        let mut mode = PreviewSolverMode::DenseScrutiny;
        match name {
            "missing mode" => {
                rows.remove(0);
            }
            "duplicate mode" => rows.push(rows[0].clone()),
            "duplicate parity" => rows.push(rows[1].clone()),
            "wrong mode" => rows[0].value = 1.0,
            "fallback" => rows[0].value = 3.0,
            "foreign case" => other.id = "foreign".into(),
            "foreign mode" => mode = PreviewSolverMode::SparseInteractive,
            _ => unreachable!(),
        }
        o.solver_observations(&other, mode, &rows);
        assert!(o.error.is_some(), "{name}");
        assert!(o.observations.is_none());
        println!("I50_OBSERVATION_CAPTURE_REFUSAL {name} {:?}", o.error);
    }
    for name in [
        "missing hook",
        "duplicate hook",
        "missing snapshot and row",
        "missing snapshot",
        "wrong capture mode",
        "wrong invocation mode",
        "wrong case",
        "wrong count",
        "extra final",
        "missing final",
        "missing final mode",
    ] {
        let mut o = captured();
        let mut final_e = e.clone();
        match name {
            "missing hook" => {
                o.observations = None;
                o.observation_calls = 0;
            }
            "duplicate hook" => {
                o.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix)
            }
            "missing snapshot and row" => {
                o.observations.as_mut().unwrap().parity = None;
                final_e.results.remove(1);
            }
            "missing snapshot" => o.observations.as_mut().unwrap().parity = None,
            "wrong capture mode" => {
                o.observations.as_mut().unwrap().mode = PreviewSolverMode::SparseInteractive
            }
            "wrong invocation mode" => {
                o.invocation_mode = Some(PreviewSolverMode::SparseInteractive)
            }
            "wrong case" => o.observations.as_mut().unwrap().case = "foreign".into(),
            "wrong count" => o.observation_calls = 2,
            "extra final" => final_e.results.push(final_e.results[1].clone()),
            "missing final" => {
                final_e.results.remove(1);
            }
            "missing final mode" => {
                final_e.results.remove(0);
            }
            _ => unreachable!(),
        }
        assert!(o.bind_observations(&final_e).is_err(), "{name}");
    }
    for index in [0, 1] {
        for field in [
            "id",
            "kind",
            "entity",
            "unit",
            "component",
            "coordinate",
            "location",
            "sign",
            "source refs",
            "case ref",
            "ref type",
            "missing metadata",
            "value bit",
            "negative",
            "nonfinite",
            "basis same length",
            "basis space",
            "basis unicode",
        ] {
            let o = captured();
            let mut changed = e.clone();
            let row = &mut changed.results[index];
            match field {
                "id" => row.id.push('x'),
                "kind" => row.kind.push('x'),
                "entity" => row.entity_ref.push('x'),
                "unit" => row.unit.push('x'),
                "component" => row.metadata.as_mut().unwrap().component.push('x'),
                "coordinate" => row.metadata.as_mut().unwrap().coordinate_system.push('x'),
                "location" => row.metadata.as_mut().unwrap().location.push('x'),
                "sign" => row.metadata.as_mut().unwrap().sign_convention.push('x'),
                "source refs" => row.source_result_refs.push("foreign".into()),
                "case ref" => row.basis_ref.as_mut().unwrap().ref_id = "foreign".into(),
                "ref type" => row.basis_ref.as_mut().unwrap().ref_type = "foreign".into(),
                "missing metadata" => row.metadata = None,
                "value bit" => row.value = f64::from_bits(row.value.to_bits() + 1),
                "negative" => row.value = -1.0,
                "nonfinite" => row.value = f64::INFINITY,
                "basis same length" => row
                    .metadata
                    .as_mut()
                    .unwrap()
                    .basis
                    .replace_range(0..1, "X"),
                "basis space" => row.metadata.as_mut().unwrap().basis.push(' '),
                "basis unicode" => row.metadata.as_mut().unwrap().basis.push('λ'),
                _ => unreachable!(),
            }
            assert!(o.bind_observations(&changed).is_err(), "{index}: {field}");
        }
    }
    // Explicit terminal absence represents either reviewed no-row branch. This
    // is a synthetic boundary control, not evidence of a naturally absent run.
    let mut absent = fresh();
    absent.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix[..1]);
    assert!(!absent.observations.as_ref().unwrap().parity_produced);
    let mut absent_final = e.clone();
    absent_final.results.remove(1);
    absent.bind_observations(&absent_final).unwrap();
    assert!(absent.bind_observations(&e).is_err());
    for zero in [0.0f64, -0.0f64] {
        let mut o = fresh();
        let mut rows = prefix.clone();
        rows[1].value = zero;
        o.solver_observations(case, PreviewSolverMode::DenseScrutiny, &rows);
        assert!(o.error.is_none());
        let mut final_e = e.clone();
        final_e.results[1].value = zero;
        o.bind_observations(&final_e).unwrap();
        assert!(o.observations.as_ref().unwrap().parity_produced);
        assert_eq!(
            o.observations
                .as_ref()
                .unwrap()
                .parity
                .as_ref()
                .unwrap()
                .value_bits,
            zero.to_bits()
        );
    }
    let mut sparse = fresh();
    sparse.invocation_mode = Some(PreviewSolverMode::SparseInteractive);
    let mut rows = prefix.clone();
    rows[0].value = 1.0;
    sparse.solver_observations(case, PreviewSolverMode::SparseInteractive, &rows);
    assert!(format!("{:?}", sparse.error).contains("parity mode/value"));
    for event in [
        E::SourceVisit,
        E::MapWrite,
        E::ValidationEntry,
        E::IdentityByteRead,
        E::KeyProbe,
        E::AllocationRequest,
        E::LibraryBoundary,
        E::RequestedCopyBytes,
        E::RustCapacityBytes,
    ] {
        let mut o = fresh();
        let mut counts = o.adapter.counts.get();
        counts[event as usize] = u64::MAX;
        o.adapter.counts.set(counts);
        o.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
        assert!(
            matches!(o.error,Some(CaptureError::Accounting(AdapterFault::Overflow(actual))) if actual==event),
            "{event:?}: {:?}",
            o.error
        );
        assert!(o.observations.is_none());
        let saved = o.adapter.counts.get();
        let calls = o.observation_calls;
        o.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
        assert_eq!(saved, o.adapter.counts.get());
        assert_eq!(calls, o.observation_calls);
        println!(
            "I50_OBSERVATION_PREFIX {event:?} {saved:?} {:?}",
            o.observation_capacity_bytes
        );
    }
    let mut overflow = fresh();
    overflow.observation_calls = usize::MAX;
    overflow.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
    assert!(matches!(
        overflow.error,
        Some(CaptureError::CountRange("observation calls"))
    ));
    let mut storage = fresh();
    storage.observation_storage_failure = Some(2);
    storage.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
    assert!(matches!(storage.error, Some(CaptureError::Storage(_))));
    assert!(storage.observations.is_none());
    assert!(
        storage.observation_capacity_bytes[0] > 0
            && storage.observation_capacity_bytes[1] > 0
            && storage.observation_capacity_bytes[2] == 0
    );
    let saved = storage.adapter.counts.get();
    storage.solver_observations(case, PreviewSolverMode::DenseScrutiny, &prefix);
    assert_eq!(saved, storage.adapter.counts.get());
    println!("I50_OBSERVATION_SYNTHETIC absence_and_signed_zero_pass storage_prefix={saved:?}");
    // Independent typed FK maximum coverage, bypassing final-id validation only
    // for isolated synthetic descriptors; mode remains mandatory.
    let (invocation, case) = actual.native.as_ref().unwrap();
    let k::ExecutionOutcome::Selected(owner) = &case.outcome else {
        panic!()
    };
    for name in [
        "duplicate",
        "unit",
        "missing mode",
        "parity substitutes mode",
        "negative",
        "negative zero",
    ] {
        let mut rows = actual.bind_rows(&e, owner).unwrap();
        let p = rows
            .iter()
            .position(|r| r.recipe == k::ProductRecipe::DenseParityObservation)
            .unwrap();
        let negative = -1.0;
        let negative_zero = -0.0;
        match name {
            "duplicate" => {
                let r = &rows[p];
                let copy = k::ProductFinalRow {
                    id: "distinct-parity",
                    case_id: r.case_id,
                    value: r.value,
                    unit: r.unit,
                    body: r.body,
                    recipe: r.recipe,
                };
                rows.push(copy);
            }
            "unit" => rows[p].unit = k::ProductUnit::Newton,
            "missing mode" => {
                let i = rows
                    .iter()
                    .position(|r| r.recipe == k::ProductRecipe::NonQuantity)
                    .unwrap();
                rows.remove(i);
            }
            "parity substitutes mode" => {
                let i = rows
                    .iter()
                    .position(|r| r.recipe == k::ProductRecipe::NonQuantity)
                    .unwrap();
                rows[i].recipe = k::ProductRecipe::DenseParityObservation;
                rows.remove(p);
            }
            "negative" => rows[p].value = &negative,
            "negative zero" => rows[p].value = &negative_zero,
            _ => unreachable!(),
        }
        let spent = invocation.certify_product_case(case.run, owner, &actual.facts, &rows);
        if name == "negative zero" {
            assert!(spent.failure().is_none());
            assert_eq!(spent.verdicts()[p].class, None);
            assert_eq!(spent.verdicts()[p].normalized_bits, (-0.0f64).to_bits());
        } else {
            assert_eq!(spent.failure().unwrap().category(), "association");
        }
    }
}

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
    observed_mode(raw, PreviewSolverMode::SparseInteractive)
}
fn observed_mode(raw: serde_json::Value, mode: PreviewSolverMode) -> (MechanicsEnvelope, ProductCapture) {
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
    assert!(o.bind_rows(&no_mode, owner).is_err(), "completed observation custody independently requires mode");
    // Retain the original FK missing-mode discriminator after the stricter PP
    // boundary, by removing mode from otherwise valid typed descriptors.
    let mut rows = o.bind_rows(&e, owner).unwrap();
    rows.retain(|r| r.recipe != k::ProductRecipe::NonQuantity);
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


#[test]
fn i51_first_prepared_native_both_modes() {
    for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
        let raw=i50_named_request();
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut observer=ProductCapture::prepared_probe();
        let e=run_linear_static_preview_observed(request,mode,Some(&capture),&mut SourceRecoveryBudget::default(),Some(&mut observer));
        assert!(observer.native.is_none(),"no old source solve");
        assert!(observer.error.is_none(),"{:?}",observer.error);
        let mut prepared=observer.prepare_case(e.clone()).unwrap_or_else(|o|panic!("prepare {:?} {:?}",o.capture.error,o.preparation_error));
        for (p,w) in prepared.preparations.iter().zip(&prepared.preparation_work) {
            println!("I51_PREPARATION mode={} input={:?} prepared={:?} work={:?}",mode.as_str(),p.input_bits(),p.section_bits().bits(),w);
            assert_eq!(p.section_bits().bits(),[0x3f7872fa3a37ac13,0x3efc52664442210a,0x3f0c52664442210a,0x3f31b37feaa954a6,0x3fb999999999999a]);
            assert!(w.status().is_exact());
            assert_eq!(w.layout_bytes[0],27*w.layout_bytes[1],"named prep frame, helper frames separate");
            assert_eq!(w.conversions.exact(),Ok(9));
        }
        prepared.solve_native().unwrap();
        i50_dump(&e,prepared.capture(),mode);
        let (invocation,case)=prepared.capture().native.as_ref().unwrap();
        assert_eq!(invocation.calls().len(),1);
        if let k::ExecutionOutcome::Selected(owner)=&case.outcome {
            println!("I51_NATIVE mode={} p={} evidence={:?} late_source_hooks={}",mode.as_str(),owner.selected_precision(),owner.evidence(),prepared.capture().prepared_late_calls);
        } else {panic!("prepared unavailable");}
        let ordinary=run_linear_static_preview_value_with_mode(raw,mode).unwrap();
        assert_eq!(serde_json::to_string(&e).unwrap(),serde_json::to_string(&ordinary).unwrap());
    }
}


fn i51_c0_mode_observation() -> ResultItem {
    ResultItem {id:"result:solver-mode:linear-solve-basis".into(),kind:"linear_solver_mode_basis".into(),
        value:1.,unit:"mode_code".into(),entity_ref:"solver:linear_static_preview".into(),basis_ref:None,source_result_refs:Vec::new(),
        metadata:Some(ResultMetadata{component:"linear_solver_mode".into(),coordinate_system:"reduced_system".into(),
            location:"case".into(),basis:"isolated successful ordinary observation fixture".into(),
            sign_convention:"mode_code 1=sparse_interactive, 2=dense_scrutiny, 3=dense_fallback_after_sparse_failure".into()})}
}
fn i51_c0_envelope(mut mode:ResultItem)->MechanicsEnvelope {
    mode.basis_ref=Some(ResultBasisRef{ref_type:"load_case".into(),ref_id:"case".into()});
    let mut producer=mechanics_producer();producer.semantic_contract_id=preview_physics::ID.into();
    MechanicsEnvelope {contract_evidence:None,schema_version:MECHANICS_SCHEMA_VERSION.into(),producer,
        numerical_quality:unassessed_numerical_quality(),source_block_recovery:None,
        formulation_basis:preview_physics::formulation_basis(),document_kind:"isolated_C0_fixture".into(),
        run_id:"isolated_C0_no_solve".into(),model_ref:"fixture".into(),
        status:StatusEnvelope{mechanics:"MECHANICS_SOLVED".into(),rule_check:"not_performed".into(),professional_acceptance:"not_provided".into()},
        summary:Summary{node_count:2,segment_count:1,support_count:4,load_case_count:1,component_stress_modifier_count:0,
            component_user_stiffness_macro_element_count:0,component_pressure_thrust_load_count:0,spring_hanger_user_input_count:0,
            max_displacement:None,max_open_formula_stress:None},results:vec![mode],diagnostics:Vec::new(),
        professional_boundary:professional_boundary(),accepted_model_state_mutated:false}
}
#[test]
fn i51_c0_isolated_late_hook_custody_and_prefixes() {
    use super::retained_product::{AdapterEvent as E,CaptureError};
    // Only construct ordinary inputs and exercise capture. No public preview,
    // full model solve, native invocation, factor or certificate is called.
    for name in ["valid","missing early","missing observations","foreign case","wrong invocation mode",
        "wrong observation mode","wrong observation owner","wrong parity presence","wrong mode bits",
        "multiple cases","combination","selected","duplicate","inner capture failure",
        "key accounting","identity accounting","marker accounting","inner accounting","missing late",
        "final identity","final status","final source selected","final value","final metadata","prepared reserve fault"] {
        let raw=i50_named_request();
        let (mut request,inv)=source_receipt::CapturedInvocation::parse(raw,PreviewSolverMode::SparseInteractive).unwrap();
        let mut diagnostics=Vec::new();
        let built=build_model(&request.model,&request.model.materials,&mut diagnostics).unwrap();
        let boundary=prepare_boundary(built.nodes.len(),&built.supports);
        assert!(diagnostics.is_empty() && boundary.findings.is_empty());
        let application=LoadApplication{nodal_loads:request.model.load_cases[0].primitive_loads.iter().enumerate().map(|(i,l)|
            open_pipe_stress_primitive_loads::NodalLoadContribution{load_id:l.id.clone(),node_index:1,global_dof:9+i,value:l.magnitude.value}).collect(),
            element_uniform_loads:Vec::new(),imposed_displacements:Vec::new(),findings:Vec::new()};
        let mut o=ProductCapture::prepared_probe();
        o.invocation(Some(&inv),PreviewSolverMode::SparseInteractive);
        o.normalized(&request.model,&request.model.materials,false);
        let mode_row=i51_c0_mode_observation();
        if name!="missing early" {
            o.case_source(&request.model,&built,&request.model.materials,&request.model.load_cases[0],
                &boundary.restrained_dofs,&boundary.springs,&application,&[],&[]);
        }
        assert!(o.source.is_none() && o.source_capture_entries==0 && o.native.is_none());
        if name!="missing observations" && name!="missing early" {
            o.solver_observations(&request.model.load_cases[0],PreviewSolverMode::SparseInteractive,std::slice::from_ref(&mode_row));
            assert!(o.error.is_none(),"{name}: {:?}",o.error);
        }
        match name {
            "foreign case"=>request.model.load_cases[0].id="foreign".into(),
            "wrong invocation mode"=>o.invocation_mode=Some(PreviewSolverMode::DenseScrutiny),
            "wrong observation mode"=>o.observations.as_mut().unwrap().mode=PreviewSolverMode::DenseScrutiny,
            "wrong observation owner"=>o.observations.as_mut().unwrap().case="foreign".into(),
            "wrong parity presence"=>o.observations.as_mut().unwrap().parity_produced=true,
            "wrong mode bits"=>o.observations.as_mut().unwrap().mode_row.value_bits=2f64.to_bits(),
            "multiple cases"=>request.model.load_cases.push(request.model.load_cases[0].clone()),
            "combination"=>request.model.combinations.push(serde_json::from_value(serde_json::json!({"id":"C","basis":"mechanics"})).unwrap()),
            "inner capture failure"=>request.model.load_cases[0].primitive_loads[0].magnitude.value=1.,
            "key accounting"|"identity accounting"|"marker accounting"|"inner accounting"=>{
                let mut counts=o.adapter.counts.get();
                let (event,value)=match name {"key accounting"=>(E::KeyProbe,u64::MAX),"identity accounting"=>(E::IdentityByteRead,u64::MAX),
                    "marker accounting"=>(E::MapWrite,u64::MAX),_=>(E::MapWrite,u64::MAX-2)};
                counts[event as usize]=value;o.adapter.counts.set(counts);
            },_=>{}
        }
        if name!="missing late" {
            o.prepared_case_source(name=="selected",&request.model,&built,&request.model.materials,&request.model.load_cases[0],
                &boundary.restrained_dofs,&boundary.springs,&application,&[],&[]);
        }
        if name=="duplicate" {
            let entries=o.source_capture_entries;
            o.prepared_case_source(false,&request.model,&built,&request.model.materials,&request.model.load_cases[0],
                &boundary.restrained_dofs,&boundary.springs,&application,&[],&[]);
            assert_eq!(o.source_capture_entries,entries);
        }
        assert!(o.native.is_none(),"zero native calls");
        if name.ends_with("accounting") {
            assert!(matches!(o.error,Some(CaptureError::Accounting(_))),"{name}: {:?}",o.error);
            let counts=o.adapter.counts.get();
            o.prepared_case_source(false,&request.model,&built,&request.model.materials,&request.model.load_cases[0],
                &boundary.restrained_dofs,&boundary.springs,&application,&[],&[]);
            assert_eq!(o.adapter.counts.get(),counts,"prior fault blocks repeat source work");
        }
        // B1 SP (T-2): each requested case's late capture is its own, so a request naming a
        // second case no longer refuses the first case's capture; custody refuses it (below).
        let after_capture=matches!(name,"valid"|"duplicate"|"inner capture failure"|"final identity"|"final status"|"final source selected"|"final value"|"final metadata"|"prepared reserve fault"|"multiple cases");
        assert_eq!(o.source_capture_entries,usize::from(after_capture),"{name}");
        if name=="valid" || name.starts_with("final ") || name=="missing late" || name=="prepared reserve fault" || name=="multiple cases" {
            let mut e=i51_c0_envelope(mode_row);
            match name {"final identity"=>e.producer.semantic_contract_id="foreign".into(),
                "final status"=>e.status.mechanics="blocked".into(),
                "final source selected"=>e.source_block_recovery=Some(serde_json::json!({})),
                "final value"=>e.results[0].value=2.,
                "final metadata"=>e.results[0].metadata.as_mut().unwrap().basis="foreign".into(),_=>{}}
            let bytes=serde_json::to_vec(&e).unwrap();
            o.finish(&e);
            assert_eq!(serde_json::to_vec(&e).unwrap(),bytes,"capture never mutates envelope");
            if name=="multiple cases" {
                // B1 SP (T-6): two cases requested, one seen. Invocation custody refuses once,
                // before any product attempt.
                assert!(o.error.is_none(),"{:?}",o.error);assert!(o.source.is_some());assert_eq!(o.cases_seen(),1);
                let failure=match o.prepare_cases(e.clone(),2,&[0,1]) {Ok(_)=>panic!("one case seen of two requested"),Err(f)=>f};
                assert!(matches!(&failure.error,CaptureError::Association(m) if m=="prepared case count"),"{:?}",failure.error);
                assert!(failure.capture.native.is_none() && failure.capture.prepared_capacity_bytes==[0;16],"no attempt started");
                println!("I51_C0_SEAM {name} source_entries=1 custody={:?}",failure.error);
                continue;
            }
            if name=="valid" {
                assert!(o.error.is_none(),"{:?}",o.error);assert!(o.source.is_some());
                let prepared=o.prepare_case(e.clone()).unwrap_or_else(|f|panic!("{:?}",f.capture.error));
                assert_eq!(prepared.preparations.len(),1);assert_eq!(prepared.preparation_work.len(),1);
                assert!(prepared.capture().native.is_none());
                assert_eq!(prepared.capture().prepared_capacity_bytes[11],3*std::mem::size_of::<u32>());
                assert_eq!(prepared.capture().prepared_capacity_bytes[10],18);
                println!("I51_C0_SEAM valid prepared source_entries=1 preparation_entries=1 native_calls=0 capacities={:?} work={:?}",
                    prepared.capture().prepared_capacity_bytes,prepared.capture().adapter);
                continue;
            }
            else if name=="prepared reserve fault" {
                let mut counts=o.adapter.counts.get();counts[E::RustCapacityBytes as usize]=u64::MAX;o.adapter.counts.set(counts);
                let failure=match o.prepare_case(e.clone()) {Ok(_)=>panic!("injected preparation allocation must fail"),Err(f)=>f};
                assert!(matches!(failure.capture.error,Some(CaptureError::Accounting(_))));
                assert!(failure.capture.prepared_capacity_bytes[0]>0,"successful reserve survives later counter fault");
                assert!(failure.preparations.is_empty() && failure.capture.native.is_none());
                println!("I51_C0_SEAM {name} source_entries={} preparation_entries={} native_calls=0 capacities={:?} work={:?}",
                    failure.capture.source_capture_entries,failure.preparation_work.len(),failure.capture.prepared_capacity_bytes,failure.capture.adapter);
                continue;
            } else {assert!(o.error.is_some(),"{name}");}
        } else {assert!(o.error.is_some(),"{name}");}
        println!("I51_C0_SEAM {name} late={} source_entries={} preparation_entries=0 native_calls=0 support_caps={:?} work={:?} error={:?}",
            o.prepared_late_calls,o.source_capture_entries,o.support_capacity_bytes,o.adapter,o.error);
    }
}


#[test]
fn i51_c0_isolated_guard_accounting_boundaries() {
    use super::retained_product::{AdapterEvent as E,CaptureError};
    let raw=i50_named_request();let (request,inv)=source_receipt::CapturedInvocation::parse(raw,PreviewSolverMode::SparseInteractive).unwrap();
    let mut d=Vec::new();let built=build_model(&request.model,&request.model.materials,&mut d).unwrap();
    let boundary=prepare_boundary(built.nodes.len(),&built.supports);
    let empty=LoadApplication{nodal_loads:Vec::new(),element_uniform_loads:Vec::new(),imposed_displacements:Vec::new(),findings:Vec::new()};
    let case=&request.model.load_cases[0];
    let mut checked=0;
    for late in [false,true] {
        for (event,count) in [(E::KeyProbe,if late{4}else{1}),
            (E::IdentityByteRead,if late{32}else{8}),
            (E::ValidationEntry,if late{6}else{2}),
            (E::MapWrite,if late{3}else{3})] {
            for offset in 0..count {
                let mut o=ProductCapture::prepared_probe();o.invocation(Some(&inv),PreviewSolverMode::SparseInteractive);
                o.normalized(&request.model,&request.model.materials,false);
                if late {
                    o.case_source(&request.model,&built,&request.model.materials,case,&boundary.restrained_dofs,&boundary.springs,&empty,&[],&[]);
                    o.solver_observations(case,PreviewSolverMode::SparseInteractive,&[i51_c0_mode_observation()]);
                }
                let mut counters=o.adapter.counts.get();counters[event as usize]=u64::MAX-offset;o.adapter.counts.set(counters);
                if late {o.prepared_case_source(false,&request.model,&built,&request.model.materials,case,&boundary.restrained_dofs,&boundary.springs,&empty,&[],&[]);}
                else {o.case_source(&request.model,&built,&request.model.materials,case,&boundary.restrained_dofs,&boundary.springs,&empty,&[],&[]);}
                assert!(matches!(o.error,Some(CaptureError::Accounting(_))),"late={late} event={event:?} offset={offset}: {:?}",o.error);
                assert_eq!(o.source_capture_entries,0);assert!(o.source.is_none() && o.native.is_none());
                let after=o.adapter.counts.get();
                o.prepared_case_source(false,&request.model,&built,&request.model.materials,case,&boundary.restrained_dofs,&boundary.springs,&empty,&[],&[]);
                assert_eq!(o.adapter.counts.get(),after);checked+=1;
                println!("I51_C0_BOUNDARY late={late} event={event:?} offset={offset} source_entries=0 native_calls=0 prefix={after:?}");
            }
        }
    }
    assert_eq!(checked,59);
}


#[test]
fn i51_complete_prepared_candidate_both_modes() {
    let mut all=true;
    for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
        let raw=i50_named_request();let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut prepared=retained_product::PreparedCase::prepare_observed(request,mode,&capture).unwrap_or_else(|e|panic!("{:?}",e.capture.error));
        let original=serde_json::to_vec(prepared.ordinary()).unwrap();
        prepared.solve_native().unwrap();
        match prepared.project_candidate() {
            Ok(candidate)=>{
                println!("I51_CANDIDATE mode={} committed_private=true",mode.as_str());
                assert!(candidate.certificate().passed());
                let (work,associations)=candidate.local_work();assert_eq!(work.entered,3);assert!(!work.lost);
                assert_eq!(associations.len(),1);assert_eq!(associations[0].old_source[5],0x3f0c52664442210e);
                assert_eq!(associations[0].old_facts[3],associations[0].old_source[3]);
                assert_eq!(associations[0].old_facts[4],associations[0].old_source[5]);
                assert_eq!(associations[0].prepared,[0x3f7872fa3a37ac13,0x3efc52664442210a,0x3f0c52664442210a,0x3f31b37feaa954a6,0x3fb999999999999a]);
                println!("I51_ASSOCIATION mode={} {:?}",mode.as_str(),candidate.local_work());
                assert_eq!(candidate.certificate().work().source_correction_calls().unwrap().exact(),Ok(2));
                i50_dump(candidate.envelope(),candidate.capture(),mode);
            }
            Err(failure)=>{
                println!("I51_CANDIDATE mode={} committed_private=false error={:?}",mode.as_str(),failure.error);
                assert_eq!(serde_json::to_vec(&failure.ordinary).unwrap(),original,"failure preserves fallback");
                i50_dump(&failure.ordinary,failure.capture(),mode);
                all=false;
            }
        }
    }
    assert!(all,"both private prepared modes must pass; first failures preserved");
}

#[test]
fn i51_actual_exact_pressure_selection_suppresses_all_prepared_work() {
    let raw:serde_json::Value=serde_json::from_str(include_str!("../tests/fixtures/exact_pressure_connected_request.json")).unwrap();
    for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut observer=ProductCapture::prepared_probe();
        let ordinary=run_linear_static_preview_observed(request,mode,Some(&capture),&mut SourceRecoveryBudget::default(),Some(&mut observer));
        assert_eq!(observer.source_capture_entries,0);
        assert!(observer.source.is_none());assert!(observer.native.is_none());
        assert_eq!(observer.prepared_capacity_bytes,[0;16]);
        let failure=match observer.prepare_case(ordinary.clone()) {Err(e)=>e,Ok(_)=>panic!("exact selection must suppress prepared W1")};
        assert!(failure.preparations.is_empty());assert!(failure.preparation_work.is_empty());
        assert!(failure.capture.native.is_none());
        let public=run_linear_static_preview_value_with_mode(raw.clone(),mode).unwrap();
        assert_eq!(serde_json::to_vec(&ordinary).unwrap(),serde_json::to_vec(&public).unwrap());
    }
}

fn i51_ready_for_controls()->(MechanicsEnvelope,retained_product::PreparedCase) {
    let mode=PreviewSolverMode::SparseInteractive;let raw=i50_named_request();
    let (request,capture)=source_receipt::CapturedInvocation::parse(raw,mode).unwrap();
    let mut prepared=retained_product::PreparedCase::prepare_observed(request,mode,&capture).unwrap_or_else(|e|panic!("{:?}",e.capture.error));
    let ordinary=prepared.ordinary().clone(); // Test oracle copy; never accepted by projection.
    prepared.solve_native().unwrap();(ordinary,prepared)
}

#[test]
fn i51_frozen_owner_values_and_numeric_refusal_controls() {
    for variant in 0..5 {
        let (ordinary,prepared)=i51_ready_for_controls();
        let (invocation,case)=prepared.capture().native.as_ref().unwrap();let owner=match &case.outcome{k::ExecutionOutcome::Selected(v)=>v,_=>panic!()};
        let base=prepared.capture().bind_rows(&ordinary,owner).unwrap();
        let specs:Vec<_>=base.iter().map(|r|match r.recipe {
            k::ProductRecipe::NonQuantity=>k::ProductRowSpec::mode(r.id,r.case_id,r.body,1).unwrap(),
            k::ProductRecipe::DenseParityObservation=>k::ProductRowSpec::parity(r.id,r.case_id,r.body,r.value.to_bits()).unwrap(),
            k::ProductRecipe::ModulusBasisRecord=>k::ProductRowSpec::material_record(r.id,r.case_id,r.body),
            _=>k::ProductRowSpec::mechanical(r.id,r.case_id,r.unit,r.body,r.recipe).unwrap(),}).collect();
        let draft=invocation.begin_prepared_product(case.run,owner,&prepared.capture().facts,&specs).into_ready().unwrap();
        let (projected,builder)=draft.project().into_ready().unwrap();
        let maxima:Vec<_>=base.iter().enumerate().filter_map(|(i,r)|if let k::ProductRecipe::CircularMaximum{member}=r.recipe {
            Some(k::ProductMaximumValue::new(member,i,if variant==4{1.0}else{*r.value}).unwrap())}else{None}).collect();
        let (values,value_work)=builder.complete_maxima(&maxima).into_ready().unwrap();
        let mut actual:Vec<_>=base.iter().enumerate().map(|(i,r)|k::ProductFinalRow{id:r.id,case_id:r.case_id,value:values.value(i).unwrap(),unit:r.unit,body:r.body,recipe:r.recipe}).collect();
        let bad_value=123.0;
        match variant {
            0=>actual[0].value=&bad_value,
            1=>actual[1].id="stale-id",
            2=>actual[1].unit=k::ProductUnit::Pascal,
            3=>{let at=base.iter().position(|r|matches!(r.recipe,k::ProductRecipe::Stress{stress:k::ProductStress::Torsion,..}) && r.value.to_bits()!=values.value(base.iter().position(|q|q.id==r.id).unwrap()).unwrap().to_bits()).unwrap();actual[at].value=base[at].value;},
            _=>{},
        }
        let failure=match projected.certify_final(&values,&actual,value_work).into_ready(){Err(e)=>e,Ok(_)=>panic!("mutant certified {variant}")};
        assert_eq!(failure.work().source_correction_calls().unwrap().exact(),Ok(2));
        if variant==4 {assert_eq!(failure.failure().category(),"numeric_predicate");assert!(failure.work().verdicts().iter().any(|v|!v.passed));}
        else {assert_eq!(failure.failure().category(),"association");}
    }
    // A different recorded owner is rejected before any residual or correction.
    let (ordinary,a)=i51_ready_for_controls();let (_,b)=i51_ready_for_controls();
    let (ai,ac)=a.capture().native.as_ref().unwrap();let (_,bc)=b.capture().native.as_ref().unwrap();
    let foreign=match &bc.outcome{k::ExecutionOutcome::Selected(v)=>v,_=>panic!()};
    let failed=match ai.begin_prepared_product(ac.run,foreign,&a.capture().facts,&[]).into_ready(){Err(e)=>e,Ok(_)=>panic!("foreign accepted")};
    assert_eq!(failed.failure().category(),"association");assert!(failed.work().source_correction_calls().is_none());drop(ordinary);
}
#[test]
fn i51_old_source_refuses_and_private_driver_cannot_reenter() {
    let mode=PreviewSolverMode::SparseInteractive;let raw=i50_named_request();let (request,capture)=source_receipt::CapturedInvocation::parse(raw,mode).unwrap();
    let mut observer=ProductCapture::prepared_probe();let ordinary=run_linear_static_preview_observed(request,mode,Some(&capture),&mut SourceRecoveryBudget::default(),Some(&mut observer));
    let old_source=observer.source.clone().unwrap();let old_facts=observer.facts.clone();let original=serde_json::to_vec(&ordinary).unwrap();
    let mut prepared=observer.prepare_case(ordinary.clone()).unwrap_or_else(|e|panic!("{:?}",e.capture.error));
    prepared.test_capture_mut().source=Some(old_source);prepared.test_capture_mut().facts=old_facts;prepared.solve_native().unwrap();
    let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("old source certified")};
    assert_eq!(serde_json::to_vec(&refusal.ordinary).unwrap(),original);
    assert!(matches!(&refusal.error,retained_product::PreparedCandidateError::Proof(p) if p.failure().category()=="numeric_predicate"));
    let (again,old)=refusal.test_reentry();assert_eq!(serde_json::to_vec(&again.ordinary).unwrap(),original);
    assert!(matches!(old,retained_product::PreparedCandidateError::Proof(p) if p.work().source_correction_calls().unwrap().exact()==Ok(2)));
    let (_ordinary,prepared)=i51_ready_for_controls();let candidate=match prepared.project_candidate(){Ok(v)=>v,Err(e)=>panic!("{:?}",e.error)};
    let original=serde_json::to_vec(candidate.envelope()).unwrap();let (again,proof)=candidate.test_reentry();assert!(proof.passed());
    assert_eq!(proof.work().source_correction_calls().unwrap().exact(),Ok(2));assert_eq!(serde_json::to_vec(&again.ordinary).unwrap(),original);
}

#[test]
fn i51_atomic_commit_accounting_refusal_keeps_all_checked_work() {
    use retained_product::{AdapterEvent as E,PreparedCandidateError,CaptureError};
    retained_product::i51_overlay_layout();
    let (_ordinary,prepared)=i51_ready_for_controls();let before=prepared.capture().adapter.counts.get()[E::MapWrite as usize];
    let candidate=match prepared.project_candidate(){Ok(c)=>c,Err(e)=>panic!("{:?}",e.error)};
    let writes=candidate.capture().adapter.counts.get()[E::MapWrite as usize]-before;
    assert!(writes>108);let (ordinary,prepared)=i51_ready_for_controls();let original=serde_json::to_vec(&ordinary).unwrap();
    let mut counts=prepared.capture().adapter.counts.get();counts[E::MapWrite as usize]=u64::MAX-(writes-1);prepared.capture().adapter.counts.set(counts);
    let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("commit permit should refuse")};
    assert_eq!(serde_json::to_vec(&refusal.ordinary).unwrap(),original);
    assert!(matches!(refusal.error,PreparedCandidateError::Capture(CaptureError::Accounting(_))));
    let proof=refusal.certificate().expect("numerical work survives commit refusal");assert!(proof.passed());
    assert_eq!(proof.work().source_correction_calls().unwrap().exact(),Ok(2));assert!(refusal.capture().adapter.fault.get().is_some());
}

#[test]
fn i51_owned_ordinary_cannot_be_substituted_after_finish() {
    let (mut substitute,prepared)=i51_ready_for_controls();
    let expected=(prepared.ordinary().schema_version.clone(),prepared.ordinary().producer.clone(),
        prepared.ordinary().document_kind.clone(),prepared.ordinary().run_id.clone(),prepared.ordinary().model_ref.clone(),prepared.ordinary().status.clone());
    // This detached copy has identical rows and observations, but every challenged header differs.
    substitute.schema_version="foreign".into();substitute.producer.semantic_contract_id="foreign".into();
    substitute.document_kind="foreign".into();substitute.run_id="foreign".into();substitute.model_ref="foreign".into();
    substitute.status.mechanics="BLOCKED".into();substitute.source_block_recovery=Some(serde_json::json!({"foreign":true}));
    let candidate=match prepared.project_candidate(){Ok(c)=>c,Err(e)=>panic!("{:?}",e.error)};
    let actual=candidate.envelope();assert_eq!((&actual.schema_version,&actual.producer,&actual.document_kind,&actual.run_id,&actual.model_ref,&actual.status),
        (&expected.0,&expected.1,&expected.2,&expected.3,&expected.4,&expected.5));assert!(actual.source_block_recovery.is_none());
    assert_ne!(actual.run_id,substitute.run_id);assert!(candidate.certificate().passed());
}

fn prepared_trace_observed()->(ProductCapture,MechanicsEnvelope) {
    let mode=PreviewSolverMode::SparseInteractive;
    let (request,capture)=source_receipt::CapturedInvocation::parse(i50_named_request(),mode).unwrap();
    let mut observer=ProductCapture::prepared_probe();
    let ordinary=run_linear_static_preview_observed(request,mode,Some(&capture),&mut SourceRecoveryBudget::default(),Some(&mut observer));
    assert!(observer.error.is_none());(observer,ordinary)
}
#[test]
fn prepared_trace_preserves_prelude_swap_helper_evaluator_and_source_failure_prefixes() {
    use retained_receipt::{TraceFault as F,OldCoverage as O,PreparationResult as P,Stage,StageState};
    for fault in [F::AfterPrelude,F::AfterHelper,F::AfterEvaluation,F::SourceConstruction] {
        let (mut observer,ordinary)=prepared_trace_observed();let original=serde_json::to_vec(&ordinary).unwrap();
        observer.trace_fault=Some(fault);
        let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!("fault not entered")};
        let mut reads=retained_receipt::ProjectionWork::default();let view=failed.typed_trace(&mut reads).unwrap();
        assert_eq!(serde_json::to_vec(&failed.ordinary).unwrap(),original);
        assert_eq!(view.old_coverage,O::Complete);assert_eq!(view.operational_old.len(),1);
        assert_eq!(view.operational_old[0].member,Some(0));
        assert_eq!(view.stages[Stage::Preparation as usize],StageState::Failed);
        assert!(view.prepared_source.is_none() && view.native_run.is_none() && view.proof.is_none());
        let (helpers,new,swapped)=match fault{F::AfterPrelude=>(0,0,false),F::AfterHelper=>(1,0,true),_=>(1,1,true)};
        assert_eq!(view.members.len(),helpers);assert_eq!(view.operational_new.len(),new);assert_eq!(view.old_vector_swapped,swapped);
        if helpers==1 {
            assert!(matches!(view.members[0].result,P::Prepared(_)));
            let work=&view.preparation_work[view.members[0].work_index];assert_eq!(work.conversion_outcomes().len(),9);
            assert_eq!(work.conversions.exact(),Ok(9));
            assert_eq!(view.operational_old[0].inputs[9],view.members[0].old_source[5]);
        }
        if new==1 {
            assert_ne!(view.operational_old[0].inputs[9],view.operational_new[0].inputs[9]);
            assert_eq!(view.operational_new[0].member,Some(0));
        }
        if fault==F::AfterEvaluation {assert!(failed.preparations.is_empty());}
        if fault==F::SourceConstruction {assert!(matches!(failed.capture.error,Some(retained_product::CaptureError::Source(_))));}
        assert!(reads.local.events>0 && !reads.local.lost);
        println!("I51_TRACE_PREFIX {:?} old={} helper={} new={} swapped={} costs={:?} capacity14={}",fault,view.operational_old.len(),helpers,new,swapped,view.trace_costs,view.adapter.prepared_capacity_bytes[14]);
    }
}
#[test]
fn prepared_trace_actual_helper_refusal_and_earlier_old_prefix_are_retained() {
    use retained_receipt::{OldCoverage,PreparationResult};
    let (mut observer,ordinary)=prepared_trace_observed();observer.facts[0].diameter=0.0;
    let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!("invalid helper geometry")};
    let mut reads=retained_receipt::ProjectionWork::default();let view=failed.typed_trace(&mut reads).unwrap();
    assert_eq!(view.old_coverage,OldCoverage::Complete);assert_eq!(view.members.len(),1);
    assert!(matches!(view.members[0].result,PreparationResult::Refused(k::SectionPreparationError::InvalidGeometry)));
    assert_eq!(view.members[0].old_facts[0],0f64.to_bits());
    assert!(view.preparation_work[0].conversion_outcomes().is_empty());assert!(view.operational_new.is_empty());
    let (mut observer,ordinary)=prepared_trace_observed();
    // Actual earlier evaluator refusal is retained verbatim by the prefix view;
    // this injected incomplete owner is deliberately not a Ready/source claim.
    let mut actual=retained_product::evaluate_operational([[0.0;3];2],[1.0;4]);
    actual.member=observer.operational[0].member;observer.operational[0]=actual;
    observer.error=Some(retained_product::CaptureError::Association("test earlier capture failure".into()));
    let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!("earlier failed capture")};
    let view=failed.typed_trace(&mut reads).unwrap();assert_eq!(view.old_coverage,OldCoverage::CapturedPrefix);
    assert!(!view.old_vector_swapped);assert!(view.members.is_empty() && view.operational_new.is_empty());
    assert!(view.operational_old[0].result.is_err());
}
#[test]
fn prepared_trace_ready_is_private_and_later_g5a_failure_does_not_erase_proof() {
    use retained_receipt::{ResultRef,CheckRef,CompletionRef};
    let (_,prepared)=i51_ready_for_controls();
    let candidate=match prepared.project_candidate(){Ok(c)=>c,Err(e)=>panic!("{:?}",e.error)};
    let mut reads=retained_receipt::ProjectionWork::default();let view=candidate.typed_trace(&mut reads).unwrap();
    assert!(matches!(view.result,ResultRef::Ready));assert!(view.proof_ready);
    assert!(view.private_commit_precharged && view.private_committed);
    assert!(view.checks.iter().all(|c|matches!(c,CheckRef::Passed)));assert!(matches!(view.completion,CompletionRef::Merged));
    let proof=view.proof.as_ref().unwrap();assert_eq!(proof.projection_conversions.exact(),Ok(88));assert_eq!(proof.projection_outcomes.len(),88);
    for (i,lane) in proof.lanes.iter().enumerate() {let lane=lane.as_ref().unwrap();assert!(lane.result.is_ok());assert_eq!(lane.work.correction_calls.exact(),Ok(1));
        assert_eq!(lane.law,if i==0{k::ReadoutLaw::AdmittedK}else{k::ReadoutLaw::AnnularSource});}
    assert_eq!(view.operational_old[0].inputs[9],view.members[0].old_source[5]);
    assert_ne!(view.operational_old[0].inputs[9],view.operational_new[0].inputs[9]);
    let mut lost=retained_receipt::ProjectionWork::default();lost.local.events=u64::MAX;
    assert!(matches!(candidate.typed_trace(&mut lost),Err(retained_receipt::TraceProjectionError::LostTrace)));
    let (reentry,prior)=candidate.test_reentry();assert!(prior.passed());
    assert!(matches!(reentry.typed_trace(&mut reads),Err(retained_receipt::TraceProjectionError::MissingFailure)));
    let (ordinary,mut prepared)=i51_ready_for_controls();prepared.test_capture_mut().g5a_work.lost=true;
    let failure=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("lost G5a must refuse")};
    let view=failure.typed_trace(&mut reads).unwrap();assert!(view.proof_ready);assert!(!view.private_committed);
    assert!(matches!(view.checks[0],CheckRef::Passed));assert!(matches!(view.checks[2],CheckRef::Failed(_)));
    assert!(matches!(view.result,ResultRef::Unavailable(_)));assert_eq!(serde_json::to_vec(&ordinary).unwrap(),serde_json::to_vec(&failure.ordinary).unwrap());
    println!("I51_TRACE_LAYOUT PreparedTrace={} PreparationEntry={} PrivateAdapterSnapshot={} PreparedAttemptView={} ProjectionWork={} OperationalSpent={}",
        std::mem::size_of::<retained_receipt::PreparedTrace>(),std::mem::size_of::<retained_receipt::PreparationEntry>(),
        std::mem::size_of::<retained_receipt::PrivateAdapterSnapshot>(),std::mem::size_of::<retained_receipt::PreparedAttemptView<'static>>(),
        std::mem::size_of::<retained_receipt::ProjectionWork>(),std::mem::size_of::<retained_product::OperationalSpent>());
    println!("I51_TRACE_PP_DEPENDENCY_LAYOUT SectionPreparationWork={} ProductCertificateSpent={} ProductProofFailure={} ProductProofTrace={} CertifiedProductProof={}",
        std::mem::size_of::<k::SectionPreparationWork>(),std::mem::size_of::<k::ProductCertificateSpent<'static>>(),
        std::mem::size_of::<k::ProductProofFailure>(),std::mem::size_of::<k::ProductProofTrace<'static>>(),std::mem::size_of::<k::CertifiedProductProof>());
}
#[test]
fn prepared_trace_certificate_refusal_preserves_actual_later_checks_and_native_refusal() {
    use retained_receipt::{CheckRef,Stage,StageState};
    let (observer,ordinary)=prepared_trace_observed();let old_source=observer.source.clone().unwrap();let old_facts=observer.facts.clone();
    let mut prepared=observer.prepare_case(ordinary).unwrap_or_else(|_|panic!("preparation"));
    prepared.test_capture_mut().source=Some(old_source);prepared.test_capture_mut().facts=old_facts;prepared.solve_native().unwrap();
    let failure=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("old source must refuse certificate")};
    let mut reads=retained_receipt::ProjectionWork::default();let view=failure.typed_trace(&mut reads).unwrap();
    assert!(matches!(view.checks[0],CheckRef::Failed(_)));assert!(!view.proof_ready);
    assert_ne!(view.stages[Stage::Observables as usize],StageState::NotEntered);
    assert_ne!(view.stages[Stage::G5a as usize],StageState::NotEntered);
    assert!(view.proof.as_ref().unwrap().lanes.iter().all(|l|l.as_ref().is_some_and(|l|l.result.is_ok())));
    let (observer,ordinary)=prepared_trace_observed();let mut prepared=observer.prepare_case(ordinary).unwrap_or_else(|_|panic!());
    prepared.test_capture_mut().source=None;assert!(prepared.solve_native().is_err());
    let view=prepared.native_refusal_trace(&mut reads).unwrap();assert_eq!(view.stages[Stage::Native as usize],StageState::Failed);
    assert!(view.native_run.is_none() && view.proof.is_none());
}

#[test]
fn prepared_trace_completion_merge_and_separate_failure_keep_distinct_owners() {
    use retained_receipt::{TraceFault,Stage,StageState,CompletionRef};
    for fault in [TraceFault::Maxima,TraceFault::ValuesCompletion] {
        let (ordinary,mut prepared)=i51_ready_for_controls();prepared.test_capture_mut().trace_fault=Some(fault);
        let failure=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("injected completion refusal")};
        let mut reads=retained_receipt::ProjectionWork::default();let view=failure.typed_trace(&mut reads).unwrap();
        assert_eq!(serde_json::to_vec(&failure.ordinary).unwrap(),serde_json::to_vec(&ordinary).unwrap());
        let proof=view.proof.as_ref().unwrap();assert_eq!(proof.projection_outcomes.len(),88);
        assert_eq!(view.stages[Stage::Aliases as usize],StageState::NotEntered);
        match fault {
            TraceFault::Maxima=>{assert!(matches!(view.completion,CompletionRef::Merged));assert!(proof.completion_merged);
                assert_eq!(view.stages[Stage::Maxima as usize],StageState::Failed);assert_eq!(view.stages[Stage::Values as usize],StageState::NotEntered);},
            TraceFault::ValuesCompletion=>{
                let original=match &failure.error{retained_product::PreparedCandidateError::Values{failure,..}=>failure,_=>panic!()};
                match view.completion {CompletionRef::SeparateFailure{visits,capacity_bytes}=>{assert_eq!(visits,original.visits);assert_eq!(capacity_bytes,original.capacity);},_=>panic!()}
                assert!(!proof.completion_merged);assert_eq!(view.stages[Stage::Values as usize],StageState::Failed);
            },_=>unreachable!(),
        }
    }
}


#[test]
fn prepared_trace_prior_owned_cause_precedes_sticky_adapter_and_fresh_prelude_errors() {
    use retained_product::{CaptureError,AdapterEvent,AdapterFault};
    use retained_receipt::{ResultRef,FailureRef,OldCoverage,Stage,StageState};
    for sticky in [false,true] {
        let (mut observer,ordinary)=prepared_trace_observed();let bytes=serde_json::to_vec(&ordinary).unwrap();
        let old_inputs=observer.operational[0].inputs;
        let old_result=observer.operational[0].result.as_ref().unwrap();
        let old_bits=[old_result.length.to_bits(),old_result.axial.to_bits(),old_result.torsion.to_bits()];
        let old_work=(observer.operational[0].work.entered,observer.operational[0].work.checks,observer.operational[0].work.lost);
        let mut cause=String::with_capacity(127);cause.push_str("original capture cause: member identity");
        let cause_ptr=cause.as_ptr();let cause_capacity=cause.capacity();
        observer.error=Some(CaptureError::Association(cause));
        if sticky {
            let mut counts=observer.adapter.counts.get();counts[AdapterEvent::MapWrite as usize]=u64::MAX;
            observer.adapter.counts.set(counts);observer.adapter.fault.set(Some(AdapterFault::Overflow(AdapterEvent::MapWrite)));
        }
        let counts=observer.adapter.counts.get();let fault=observer.adapter.fault.get();
        let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!("prior cause must refuse")};
        let mut projection=retained_receipt::ProjectionWork::default();let view=failed.typed_trace(&mut projection).unwrap();
        match &view.result {
            ResultRef::Unavailable(FailureRef::Preparation{capture,section})=>{
                assert!(section.is_none());
                match capture {CaptureError::Association(text)=>{
                    assert_eq!(text,"original capture cause: member identity");
                    assert_eq!(text.as_ptr(),cause_ptr,"same owned String allocation, no replacement clone");
                    assert_eq!(text.capacity(),cause_capacity);
                },other=>panic!("original variant was replaced: {other:?}")}
            },_=>panic!("wrong typed terminal"),
        }
        assert_eq!(view.adapter.counts,counts);assert_eq!(view.adapter.fault,fault);
        assert_eq!(failed.capture.adapter.counts.get(),counts);assert_eq!(failed.capture.adapter.fault.get(),fault);
        assert_eq!(view.old_coverage,OldCoverage::CapturedPrefix);assert!(!view.old_vector_swapped);
        assert_eq!(view.operational_old[0].inputs,old_inputs);
        let old=view.operational_old[0].result.as_ref().unwrap();assert_eq!([old.length.to_bits(),old.axial.to_bits(),old.torsion.to_bits()],old_bits);
        assert_eq!((view.operational_old[0].work.entered,view.operational_old[0].work.checks,view.operational_old[0].work.lost),old_work);
        assert!(view.members.is_empty() && view.preparation_work.is_empty() && view.operational_new.is_empty());
        assert!(failed.preparations.is_empty() && view.prepared_source.is_none() && view.native_run.is_none() && view.proof.is_none());
        assert_eq!(view.stages[Stage::Preparation as usize],StageState::Failed);
        assert!(view.stages[1..].iter().all(|s|*s==StageState::NotEntered));
        assert_eq!(serde_json::to_vec(&failed.ordinary).unwrap(),bytes);
        println!("I51_PRIOR_CAUSE sticky={sticky} cause=Association(original capture cause: member identity) same_owner=true adapter_fault={fault:?} helper=0 new_evaluator=0 native=0 old_work={old_work:?} trace_costs={:?}",view.trace_costs);
    }
    // Without a prior cause, a genuine fresh prelude error keeps its own cause.
    for sticky in [false,true] {
        let (mut observer,ordinary)=prepared_trace_observed();assert!(observer.error.is_none());
        if sticky {observer.adapter.fault.set(Some(AdapterFault::Overflow(AdapterEvent::MapWrite)));}
        else {observer.final_calls=0;}
        let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!("fresh prelude failure")};
        let mut projection=retained_receipt::ProjectionWork::default();let view=failed.typed_trace(&mut projection).unwrap();
        match &view.result {
            ResultRef::Unavailable(FailureRef::Preparation{capture,..})=>match (sticky,capture) {
                (true,CaptureError::Accounting(AdapterFault::Overflow(AdapterEvent::MapWrite)))=>{},
                (false,CaptureError::Association(s))=>assert_eq!(s,"prepared case custody/permit"),
                _=>panic!("fresh cause replaced: {capture:?}"),
            },_=>panic!(),
        }
        assert!(view.members.is_empty() && view.operational_new.is_empty() && view.native_run.is_none());
    }
}

// ---------------------------------------------------------------- I61 summary coverage
// The typed C3 seam borrows the proof's own vector (I57 §1–§3). Every test names
// its I57 §5 control; "actual" means a producer run of an existing fixture.
fn i61_owner(capture:&ProductCapture)->&k::RetainedSolve {
    match &capture.native.as_ref().unwrap().1.outcome {k::ExecutionOutcome::Selected(v)=>v,_=>panic!("native owner")}
}
fn i61_complete<'a>(view:&retained_receipt::PreparedAttemptView<'a>)->retained_receipt::CompleteCoverage<'a> {
    match view.summary_coverage {Some(retained_receipt::SummaryCoverage::Complete(c))=>c,ref other=>panic!("complete coverage expected: {other:?}")}
}
fn i61_assert_roster(view:&retained_receipt::PreparedAttemptView<'_>,owner:&k::RetainedSolve,proof:&[k::ProductSummaryCoverage]) {
    let complete=i61_complete(view);let trace=view.proof.as_ref().unwrap();
    assert_eq!(trace.summary_coverage.as_ptr(),proof.as_ptr(),"borrowed proof-owned slice");
    assert_eq!(complete.len(),owner.source().body_count() as usize);
    let bodies:Vec<_>=complete.bodies().collect();assert_eq!(bodies.len(),owner.source().body_count() as usize);
    for (i,(b,c)) in bodies.iter().zip(proof).enumerate() {
        assert_eq!(b.body as usize,i);assert_eq!((b.body,b.stop,b.has_data),(c.body,c.stop,c.has_data));
    }
}
fn i61_null(view:&retained_receipt::PreparedAttemptView<'_>) {
    assert!(view.proof.is_some());
    assert!(matches!(view.summary_coverage,Some(retained_receipt::SummaryCoverage::Null)),"{:?}",view.summary_coverage);
}
#[test]
fn i61_ready_carries_complete_proof_owned_coverage_and_lower_p_charge_is_estimate() {
    // §5: Ready → complete actual array; p128/p256 with the fixed 1024 product
    // proof → charge = estimate, no floor. Actual producer run, both modes.
    for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
        let raw=i50_named_request();let (request,capture)=source_receipt::CapturedInvocation::parse(raw,mode).unwrap();
        let mut prepared=retained_product::PreparedCase::prepare_observed(request,mode,&capture).unwrap_or_else(|e|panic!("{:?}",e.capture.error));
        prepared.solve_native().unwrap();
        let candidate=match prepared.project_candidate(){Ok(c)=>c,Err(e)=>panic!("{:?}",e.error)};
        let mut reads=retained_receipt::ProjectionWork::default();let view=candidate.typed_trace(&mut reads).unwrap();
        let owner=i61_owner(candidate.capture());let p=owner.selected_precision();
        assert!(p==128||p==256);assert!(owner.evidence().floor.is_none());
        i61_assert_roster(&view,owner,candidate.certificate().summary_coverage());
        let proof=view.proof.as_ref().unwrap();assert_eq!(proof.projection_conversions.exact(),Ok(88),"fixed 1024 projection ran");
        for c in proof.summary_coverage {assert_eq!(c.charge,c.estimate);}
        assert_eq!(candidate.capture().summary_coverage,candidate.certificate().summary_coverage(),"adapter copy complete on Ready");
        assert!(reads.kernel.events.exact().unwrap()>=4 && !reads.local.lost);
        println!("I61_PP_LAYOUT SummaryCoverage={} CoverageBody={} PreparedAttemptView={}",std::mem::size_of::<retained_receipt::SummaryCoverage<'static>>(),
            std::mem::size_of::<retained_receipt::CoverageBody>(),std::mem::size_of::<retained_receipt::PreparedAttemptView<'static>>());
        println!("I61_READY mode={} p={p} coverage={:?} costs local={:?} kernel={:?}",mode.as_str(),proof.summary_coverage,(reads.local.events,reads.local.copy_bytes),reads.kernel);
    }
}
#[test]
fn i61_specimen_zero_and_cancelled_bodies_keep_actual_flags() {
    // §5: zero/no-data body in a complete source; +x/−x individual free-DOF
    // loads with zero net give has_data=true (source witness: the cancelled control
    // in actual_sparse_zero_and_cancelled_summary_coverage_is_not_inferred_from_net).
    let mode=PreviewSolverMode::SparseInteractive;
    let mut cancelled=specimen(false);
    let mut term=cancelled["model"]["load_cases"][0]["primitive_loads"][0].clone();
    let loads=cancelled["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap();
    term["id"]=serde_json::json!("i61:plus");term["magnitude"]["value"]=serde_json::json!(1.0);loads.push(term.clone());
    term["id"]=serde_json::json!("i61:minus");term["magnitude"]["value"]=serde_json::json!(-1.0);loads.push(term);
    for (label,raw,data) in [("zero",specimen(false),false),("cancelled",cancelled,true)] {
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw,mode).unwrap();
        let mut prepared=retained_product::PreparedCase::prepare_observed(request,mode,&capture).unwrap_or_else(|e|panic!("{label}: {:?}",e.capture.error));
        prepared.solve_native().unwrap();
        let (view_coverage,proof):(Vec<_>,Vec<k::ProductSummaryCoverage>)=match prepared.project_candidate() {
            Ok(candidate)=>{let mut reads=retained_receipt::ProjectionWork::default();let view=candidate.typed_trace(&mut reads).unwrap();
                i61_assert_roster(&view,i61_owner(candidate.capture()),candidate.certificate().summary_coverage());
                (i61_complete(&view).bodies().collect(),candidate.certificate().summary_coverage().to_vec())},
            Err(refusal)=>{let mut reads=retained_receipt::ProjectionWork::default();let view=refusal.typed_trace(&mut reads).unwrap();
                println!("I61_SPECIMEN {label} refusal={:?}",refusal.error);
                let proof=view.proof.as_ref().unwrap().summary_coverage.to_vec();
                (i61_complete(&view).bodies().collect(),proof)},
        };
        assert_eq!(view_coverage.len(),1);assert_eq!(view_coverage[0].has_data,data,"{label}");
        if !data {assert_eq!(view_coverage[0].stop,[false;4]);assert_eq!(proof[0].estimate,[false;2]);}
        println!("I61_SPECIMEN {label} compact={view_coverage:?} proof={proof:?}");
    }
}
#[test]
fn i61_failure_prefixes_null_complete_and_adapter_prefix() {
    use retained_product::{AdapterEvent as E,PreparedCandidateError,CaptureError};
    use retained_receipt::{TraceFault as F,ProjectionWork};
    // §5: no proof → no trace and no coverage object (actual preparation/native refusals).
    let (mut observer,ordinary)=prepared_trace_observed();observer.trace_fault=Some(F::AfterPrelude);
    let failed=match observer.prepare_case(ordinary){Err(f)=>f,Ok(_)=>panic!()};
    let view=failed.typed_trace(&mut ProjectionWork::default()).unwrap();assert!(view.proof.is_none() && view.summary_coverage.is_none());
    let (observer,ordinary)=prepared_trace_observed();let mut prepared=observer.prepare_case(ordinary).unwrap_or_else(|_|panic!());
    prepared.test_capture_mut().source=None;assert!(prepared.solve_native().is_err());
    let view=prepared.native_refusal_trace(&mut ProjectionWork::default()).unwrap();assert!(view.proof.is_none() && view.summary_coverage.is_none());
    // §5: proof-start failure before any lane → null (actual).
    let (_,mut prepared)=i51_ready_for_controls();let r=prepared.test_capture_mut().facts[0].radius;prepared.test_capture_mut().facts[0].radius=r*2.0;
    let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!()};
    let view=refusal.typed_trace(&mut ProjectionWork::default()).unwrap();i61_null(&view);
    println!("I61_PROOF_START lanes={:?}",view.proof.as_ref().unwrap().lanes.iter().map(|l|l.as_ref().map(|l|l.result.is_ok())).collect::<Vec<_>>());
    // §5: an actual admitted-K lane refusal (invalid proposed material) → null with the entered lane prefix.
    let (_,mut prepared)=i51_ready_for_controls();prepared.test_capture_mut().facts[0].material=k::ProductMaterial::Base{e:-1.0,g:-1.0};
    let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!()};
    let view=refusal.typed_trace(&mut ProjectionWork::default()).unwrap();i61_null(&view);
    let lanes:Vec<_>=view.proof.as_ref().unwrap().lanes.iter().map(|l|l.as_ref().map(|l|(l.law,l.result.is_ok()))).collect();
    println!("I61_LANE_FAILURE error={:?} lanes={lanes:?}",refusal.error);
    assert!(matches!(lanes[0],Some((k::ReadoutLaw::AdmittedK,false))),"actual K lane failure");
    // §5: maxima abandonment and values-completion failure → null (actual hooks).
    for fault in [F::Maxima,F::ValuesCompletion] {
        let (_,mut prepared)=i51_ready_for_controls();prepared.test_capture_mut().trace_fault=Some(fault);
        let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!()};
        let view=refusal.typed_trace(&mut ProjectionWork::default()).unwrap();i61_null(&view);
        assert!(view.proof.as_ref().unwrap().lanes.iter().all(|l|l.as_ref().is_some_and(|l|l.result.is_ok())));
    }
    // §5: summary completed, then the certificate fails → complete actual array (old source).
    let (observer,ordinary)=prepared_trace_observed();let old_source=observer.source.clone().unwrap();let old_facts=observer.facts.clone();
    let mut prepared=observer.prepare_case(ordinary).unwrap_or_else(|_|panic!());
    prepared.test_capture_mut().source=Some(old_source);prepared.test_capture_mut().facts=old_facts;prepared.solve_native().unwrap();
    let refusal=match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!()};
    let PreparedCandidateError::Proof(failure)=&refusal.error else {panic!("{:?}",refusal.error)};
    assert_eq!(failure.failure().category(),"numeric_predicate");
    let view=refusal.typed_trace(&mut ProjectionWork::default()).unwrap();
    i61_assert_roster(&view,i61_owner(refusal.capture()),failure.work().summary_coverage());
    assert!(matches!(view.checks[0],retained_receipt::CheckRef::Failed(_)),"complete coverage does not imply certificate success");
    // §5: adapter copy fails partway → the proof-owned complete vector, never the
    // adapter prefix. Actual run; the MapWrite permit is located by bisection so the
    // first coverage write is the one refused.
    let (_,prepared)=i51_ready_for_controls();let before=prepared.capture().adapter.counts.get()[E::MapWrite as usize];
    let candidate=match prepared.project_candidate(){Ok(c)=>c,Err(e)=>panic!("{:?}",e.error)};
    let writes=candidate.capture().adapter.counts.get()[E::MapWrite as usize]-before;
    let rows=candidate.certificate().verdicts().len();let nb=candidate.certificate().summary_coverage().len();drop(candidate);
    let probe=|allowed:u64|{let (_,prepared)=i51_ready_for_controls();let mut counts=prepared.capture().adapter.counts.get();
        counts[E::MapWrite as usize]=u64::MAX-allowed;prepared.capture().adapter.counts.set(counts);
        match prepared.project_candidate(){Err(e)=>e,Ok(_)=>panic!("permit {allowed} committed")}};
    let copied=|r:&retained_product::PreparedCandidateRefusal|r.certificate().is_some() && r.capture().verdicts.len()==rows;
    let (mut lo,mut hi)=(0u64,writes-1);assert!(copied(&probe(hi)));
    while lo<hi {let mid=lo+(hi-lo)/2;if copied(&probe(mid)){hi=mid;}else{lo=mid+1;}}
    let refusal=probe(lo);
    assert!(matches!(refusal.error,PreparedCandidateError::Capture(CaptureError::Accounting(_))));
    let adapter=&refusal.capture().summary_coverage;assert!(adapter.len()<nb,"adapter holds a strict prefix");
    let certified=refusal.certificate().unwrap();assert_eq!(certified.summary_coverage().len(),nb);
    let view=refusal.typed_trace(&mut ProjectionWork::default()).unwrap();
    i61_assert_roster(&view,i61_owner(refusal.capture()),certified.summary_coverage());
    println!("I61_ADAPTER_PREFIX permit={lo} writes={writes} rows={rows} bodies={nb} adapter_prefix={}",adapter.len());
}
fn i61_fk_certificate_failure(variant:usize)->(retained_product::PreparedCase,k::ProductProofFailure) {
    // Actual FK proof/certificate on the actual prepared owner (i51 frozen control
    // pattern): 0 = frozen value refused before the summary; 4 = row predicate
    // refused after the summary assignment.
    let (ordinary,prepared)=i51_ready_for_controls();
    let failure={
        let (invocation,case)=prepared.capture().native.as_ref().unwrap();let owner=i61_owner(prepared.capture());
        let base=prepared.capture().bind_rows(&ordinary,owner).unwrap();
        let specs:Vec<_>=base.iter().map(|r|match r.recipe {
            k::ProductRecipe::NonQuantity=>k::ProductRowSpec::mode(r.id,r.case_id,r.body,1).unwrap(),
            k::ProductRecipe::DenseParityObservation=>k::ProductRowSpec::parity(r.id,r.case_id,r.body,r.value.to_bits()).unwrap(),
            k::ProductRecipe::ModulusBasisRecord=>k::ProductRowSpec::material_record(r.id,r.case_id,r.body),
            _=>k::ProductRowSpec::mechanical(r.id,r.case_id,r.unit,r.body,r.recipe).unwrap(),}).collect();
        let draft=invocation.begin_prepared_product(case.run,owner,&prepared.capture().facts,&specs).into_ready().unwrap();
        let (projected,builder)=draft.project().into_ready().unwrap();
        let maxima:Vec<_>=base.iter().enumerate().filter_map(|(i,r)|if let k::ProductRecipe::CircularMaximum{member}=r.recipe {
            Some(k::ProductMaximumValue::new(member,i,if variant==4{1.0}else{*r.value}).unwrap())}else{None}).collect();
        let (values,value_work)=builder.complete_maxima(&maxima).into_ready().unwrap();
        let mut actual:Vec<_>=base.iter().enumerate().map(|(i,r)|k::ProductFinalRow{id:r.id,case_id:r.case_id,value:values.value(i).unwrap(),unit:r.unit,body:r.body,recipe:r.recipe}).collect();
        let bad_value=123.0;if variant==0 {actual[0].value=&bad_value;}
        match projected.certify_final(&values,&actual,value_work).into_ready(){Err(e)=>e,Ok(_)=>panic!("variant {variant} certified")}
    };
    (prepared,failure)
}
fn i61_stage_trace(capture:&ProductCapture,certificate:Option<bool>,ready:bool)->retained_receipt::PreparedTrace {
    // Synthetic stage record (labelled): completed preparation through aliases, then
    // the given certificate outcome; Ready also completes observables and G5a.
    use retained_receipt::Stage as S;
    let mut t=retained_receipt::PreparedTrace::default();t.source_ready=true;
    for s in [S::Preparation,S::Native,S::ProofStart,S::Projection,S::Maxima,S::Values,S::Aliases] {t.enter(s);t.completed(s);}
    if let Some(passed)=certificate {t.enter(S::Certificate);t.checked(S::Certificate,0,passed);}
    if ready {t.enter(S::Observables);t.checked(S::Observables,1,true);t.enter(S::G5a);t.checked(S::G5a,2,true);}
    t.freeze(capture);t
}
#[test]
fn i61_certificate_prefixes_and_stage_rules_with_actual_fk_proofs() {
    use retained_receipt::{project,ResultRef,FailureRef,TraceProjectionError as T,ProjectionWork,Stage,StageState};
    static ZERO:retained_product::ScalarWork=retained_product::ScalarWork{entered:0,checks:0,lost:false};
    // §5: certificate entered, fails before the summary assignment → null.
    let (prepared,early)=i61_fk_certificate_failure(0);assert!(early.work().summary_coverage().is_empty());
    let trace=i61_stage_trace(prepared.capture(),Some(false),false);
    let view=project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(early.failure())),
        Some(early.work()),Some(early.failure()),None,&mut ProjectionWork::default()).unwrap();
    i61_null(&view);
    // Ready (or a passed certificate) never projects null, even if handed an empty proof (synthetic).
    let trace=i61_stage_trace(prepared.capture(),Some(true),true);
    assert!(matches!(project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Ready,Some(early.work()),None,None,&mut ProjectionWork::default()),
        Err(T::StageConsistency)));
    let trace=i61_stage_trace(prepared.capture(),Some(true),false);
    assert!(matches!(project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(early.failure())),
        Some(early.work()),None,None,&mut ProjectionWork::default()),Err(T::StageConsistency)));
    // Projection refused after a completed proof start (synthetic stage record, actual proof work) → null.
    let mut trace=retained_receipt::PreparedTrace::default();trace.source_ready=true;
    for s in [Stage::Preparation,Stage::Native,Stage::ProofStart] {trace.enter(s);trace.completed(s);}
    trace.enter(Stage::Projection);trace.fail_entered();trace.freeze(prepared.capture());
    let view=project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(early.failure())),
        Some(early.work()),None,None,&mut ProjectionWork::default()).unwrap();
    i61_null(&view);
    // §5: summary completed, then a certificate predicate fails → complete array.
    let (prepared,late)=i61_fk_certificate_failure(4);assert_eq!(late.failure().category(),"numeric_predicate");
    let owner=i61_owner(prepared.capture());
    let trace=i61_stage_trace(prepared.capture(),Some(false),false);
    let view=project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
        Some(late.work()),Some(late.failure()),None,&mut ProjectionWork::default()).unwrap();
    i61_assert_roster(&view,owner,late.work().summary_coverage());
    // Non-null coverage requires every prerequisite stage and the certificate entered (synthetic stage defects).
    for defect in 0..3 {
        let mut trace=i61_stage_trace(prepared.capture(),Some(false),false);
        match defect {0=>trace.stages[Stage::Values as usize]=StageState::Failed,1=>trace.source_ready=false,
            _=>{trace.stages[Stage::Certificate as usize]=StageState::NotEntered;trace.checks[0]=retained_receipt::CheckState::NotEntered;}}
        let failure=if defect==2 {None} else {Some(late.failure())};
        assert!(matches!(project(&trace,prepared.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
            Some(late.work()),failure,None,&mut ProjectionWork::default()),Err(T::StageConsistency)),"defect {defect}");
    }
    // RV77-N4, deliberately flipped (I61 U1 grant 2, U2 on the failure path): this seam
    // formerly accepted a proof *failure* projected against a different selected owner
    // with identical public facts (custody only; I57 §5, RV77-N3). The proof work now
    // carries its anchor, so the foreign owner is refused structurally; the same failure
    // against its own owner still projects (the late-failure roster above).
    let (_,other)=i51_ready_for_controls();let mut capture_trace=i61_stage_trace(other.capture(),Some(false),false);capture_trace.source_ready=true;
    let view=project(&capture_trace,other.capture(),&[],&[],&ZERO,ResultRef::Unavailable(FailureRef::Proof(late.failure())),
        Some(late.work()),Some(late.failure()),None,&mut ProjectionWork::default());
    println!("I61_FOREIGN_OWNER same_public_facts_result_ok={}",view.is_ok());
    assert!(matches!(view,Err(T::WorkAssociation)),"foreign owner refused structurally at the seam");
    assert!(late.owner_matches(i61_owner(prepared.capture())) && !late.owner_matches(i61_owner(other.capture())));
}

// ---------------------------------------------------------------- RV77 reviewer tests (RV77-S1)
// Adopted verbatim from REVIEW_RV77/coverage_producer_01/tests/rv77_pp.rs (body unchanged;
// wrapped as an inline child module of this test file, as in RV77's copy).
mod rv77_pp {
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
}
