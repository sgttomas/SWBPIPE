//! Dormant U5a producer controls. These construct internal mechanics inputs;
//! they do not consume the frozen 0.3 requests or assert public 0.5 admission.
use super::super::*;
use super::*;
use open_pipe_stress_frame_kernel::structural::{LoadFormula, LoadPrimitive};

fn model(anchored: bool) -> PreviewModel {
    let prior: Value = serde_json::from_str(include_str!(
        "../../../validation/references/t4_i12/u3_reference_cases.json"
    ))
    .unwrap();
    let mut connector = prior["cases"]["U3-SYS-DEMO-CONNECTOR-001"]["inputs"]["document_v3_0.3.0"]
        ["model"]["components"][0]["objective_connector"]
        .clone();
    let identity = json!([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    let zero = json!({"x":0.,"y":0.,"z":0.,"unit":"m"});
    for (end, node) in [("end_i", "node:B"), ("end_j", "node:C")] {
        connector[end] =
            json!({"node_ref":node,"initial_node_axes_global":identity,"offset_local":zero});
    }
    connector["connector_axes_global"] = identity;
    connector["topology"] = json!({"type":"replaces_span","span_ref":"pipe:J"});
    connector["q_ref"] = json!({"translation":zero,"rotation":{"x":0.,"y":0.,"z":0.,"unit":"rad"}});
    connector["reference_state"] = json!("stress_free");
    connector["stiffness"]["translation_scale"] = json!({"value":1.,"unit":"m"});
    let mut upper = Vec::new();
    for r in 0..6 {
        for c in r..6 {
            upper.push(if r == c {
                [2e5, 8e4, 8e4, 600., 900., 900.][r]
            } else {
                0.
            });
        }
    }
    connector["stiffness"]["upper_triangle"] = json!(upper);
    connector["pressure_model"] = json!({"kind":"unpressurized"});
    let pipe = |id, from, to| json!({"id":id,"from":from,"to":to,"material":"material:A","y_reference":{"x":0.,"y":1.,"z":0.},"section":{"outside_diameter":{"value":0.06,"unit":"m"},"wall_thickness":{"value":0.005,"unit":"m"},"corrosion_allowance":{"value":0.0,"unit":"m"}}});
    let mut supports = vec![
        json!({"id":"support:A","node":"node:A","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"]}),
    ];
    if anchored {
        supports.push(json!({"id":"support:D","node":"node:D","family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"]}));
    }
    serde_json::from_value(json!({"schema_version":"0.7.0","document_kind":"openpipestress.product_preview.model",
        "pressure_contract":{"version":"3.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"},"project":{"id":"project:dormant-joint","units":{"length":"m"}},
        "analysis_status":{"mechanics":"not_run","rule_check":"not_run","professional_acceptance":"not_requested"},
        "nodes":[{"id":"node:A","position":{"x":0.,"y":0.,"z":0.}},{"id":"node:B","position":{"x":2.,"y":0.,"z":0.}},{"id":"node:C","position":{"x":2.3,"y":0.,"z":0.}},{"id":"node:D","position":{"x":4.3,"y":0.,"z":0.}}],
        "pipe_segments":[pipe("pipe:L","node:A","node:B"),pipe("pipe:J","node:B","node:C"),pipe("pipe:R","node:C","node:D")],
        "materials":[{"id":"material:A","constitutive_basis":"homogeneous_isotropic_E_nu_v1","elastic_modulus":{"value":2e11,"unit":"Pa"},"poisson_ratio":{"value":0.3,"unit":"1"},"provenance":"invented dormant joint control"}],
        "components":[{"id":"component:J","kind":"expansion_joint","node":"node:B","objective_connector":connector}],"supports":supports,"combinations":[],
        "load_cases":[{"id":"case:A","primitive_loads":[],"pressure_regions":[{"id":"region:A","member_pipe_ids":["pipe:L","pipe:R"],"pressure_basis":PRESSURE_BASIS,"pressure":{"value":2e5,"unit":"Pa"},"terminals":[{"node_ref":"node:A","closure_transfer":"transfers_to_wall","provenance":"invented A closure"},{"node_ref":"node:D","closure_transfer":"transfers_to_wall","provenance":"invented D closure"}],"provenance":"invented dormant joint control"}]}]
    })).unwrap()
}

fn activate_internal(model: &mut PreviewModel) {
    model.schema_version = "0.7.0".into();
    model.components[0].objective_connector.as_mut().unwrap()["pressure_model"] = json!({"kind":"constant_effective_area_reference_v1","effective_area":{"value":0.004,"unit":"m^2"},"pressure_basis":PRESSURE_BASIS,"source_reference":"invented authored joint area"});
    model.load_cases[0].pressure_regions.as_mut().unwrap()[0].member_connector_ids =
        Some(vec!["component:J".into()]);
}
struct Case {
    model: PreviewModel,
    built: BuiltModel,
    exact: ExactPressureCase,
    force: AssembledForce,
}
fn case(anchored: bool) -> Case {
    let mut model = model(anchored);
    let mut diagnostics = Vec::new();
    let mut materials = model.materials.clone();
    pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
    let mut built = build_model(&model, &materials, &mut diagnostics).unwrap();
    activate_internal(&mut model);
    let spec = joint::connector_specs(&model).remove(0);
    built.connector_records[0].pressure = spec.pressure;
    joint::classify_connectors(&model, &mut diagnostics);
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    let exact = assemble_pressure_case(
        &model,
        &built,
        &materials,
        &model.load_cases[0],
        None,
        &mut diagnostics,
    )
    .expect("internal producer");
    let mut ledger = LoadLedger::new();
    push_exact_pressure_operands(&mut ledger, &exact);
    add_connector_reference_loads(
        &mut ledger,
        &built,
        &model.load_cases[0].id,
        &mut diagnostics,
    );
    let force = ledger.finish(built.nodes.len() * 6).unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    Case {
        model,
        built,
        exact,
        force,
    }
}
fn solutions(case: &Case) -> Vec<open_pipe_stress_frame_kernel::structural::StructuralSolution> {
    let built = &case.built;
    let n = built.nodes.len() * 6;
    let boundary = prepare_boundary(built.nodes.len(), &built.supports);
    let free = (0..n)
        .filter(|dof| !boundary.restrained_dofs.contains(dof))
        .collect::<Vec<_>>();
    let prescribed = boundary
        .restrained_dofs
        .iter()
        .map(|&dof| (dof, 0.))
        .collect::<Vec<_>>();
    let BasisStiffness::Formed(formed) = form_basis_stiffness(built, &[], true).unwrap() else {
        panic!("formed")
    };
    let dense = formed.to_dense();
    let mut out = Vec::new();
    // T4-U8 (Design §4 item 2): the product's source names each frame's member.
    let members = crate::frame_member_identities(built);
    for mode in [
        LinearSolveMode::DenseScrutiny,
        LinearSolveMode::SparseInteractive,
    ] {
        let assembly = AssemblyEvidence::new(
            built.nodes.len(),
            &built.frame_elements,
            &built.connectors,
            &[],
            &[],
        )
        .unwrap()
        .with_frame_members(&members);
        out.push(
            assembly
                .solve_assembled_with_formation_check(
                    &dense,
                    &case.force,
                    &free,
                    &prescribed,
                    mode,
                    &[],
                    true,
                )
                .unwrap(),
        );
        let scale = ForceScale::new(4).unwrap();
        let scaled = dense
            .iter()
            .map(|row| row.iter().map(|value| value * 16.).collect())
            .collect::<Vec<Vec<f64>>>();
        let assembly = AssemblyEvidence::new_force_scaled(
            built.nodes.len(),
            &built.frame_elements,
            &built.connectors,
            &[],
            &[],
            scale,
        )
        .unwrap()
        .with_frame_members(&members);
        out.push(
            assembly
                .solve_force_scaled_with_formation_check(
                    &scaled,
                    &case.force,
                    &free,
                    &prescribed,
                    mode,
                    &[],
                    true,
                )
                .unwrap()
                .solution,
        );
    }
    out
}
#[test]
fn u5a_internal_j3_controls_close_and_match_independent_representation_a() {
    let pi = 125. * std::f64::consts::PI;
    for anchored in [true, false] {
        let case = case(anchored);
        // Representation A: full cap and Poisson pairs, plus B^T*[Pe,0...].
        let expected_rhs = [-0.4 * pi, 0.4 * pi - 800., 800. - 0.4 * pi, 0.4 * pi];
        for node in 0..4 {
            assert!((case.exact.assembled_loads[node * 6] - expected_rhs[node]).abs() < 1e-10);
        }
        let expected_q = if anchored {
            (16. - std::f64::consts::PI) / (4000. + 275000. * std::f64::consts::PI)
        } else {
            0.004
        };
        for solution in solutions(&case) {
            assert_eq!(
                solution.report.quality,
                SolveQuality::Passed,
                "{:?}",
                solution.formation_check
            );
            assert!(solution.member_bound.is_some());
            let q = solution.displacements[12] - solution.displacements[6];
            assert!(
                (q - expected_q).abs() <= 1e-9 * expected_q.abs() + 1e-15,
                "q={q:e}"
            );
            let mut results = Vec::new();
            let mut diagnostics = Vec::new();
            let mut records = Vec::new();
            let bodies = formation_guard::Bodies::new(
                &case
                    .built
                    .nodes
                    .iter()
                    .map(|node| node.coordinates)
                    .collect::<Vec<_>>(),
                &[(0, 1), (1, 2), (2, 3)],
            );
            append_connector_results(
                &case.built,
                Some(&case.exact),
                &solution.displacements,
                &case.model.load_cases[0].id,
                &mut results,
                &mut diagnostics,
                &case.force,
                solution.member_bound.as_ref(),
                &bodies,
                &mut records,
            );
            let gx = results
                .iter()
                .find(|row| {
                    row.kind == "connector_generalized_force_v1"
                        && row.metadata.as_ref().unwrap().component == "gt_x"
                })
                .unwrap()
                .value;
            let s = results
                .iter()
                .find(|row| row.kind == "connector_effective_axial_force_v1")
                .unwrap()
                .value;
            assert_eq!(
                results
                    .iter()
                    .find(|row| row.kind == "connector_pressure_force_v1")
                    .unwrap()
                    .value,
                800.
            );
            assert!((gx - 2e5 * expected_q).abs() < 1e-7);
            assert!((s - (gx - 800.)).abs() < 1e-10);
            assert!(records[0]
                .groups
                .iter()
                .any(|group| group.kind == "effective_axial_force"));
            assert!(!has_blocking(&diagnostics));
        }
    }
}
/// The classifier's blanket refusal remains on the retired versions. (B7b: the
/// current decoder accepts `member_connector_ids` on 0.7.0, whose positive and
/// fail-closed controls are `b7b_admission_tests`; the old decoder's refusal of
/// the key lapsed with the version.)
#[test]
fn u5a_blanket_refusal_remains_on_retired_versions() {
    for version in ["0.3.0", "0.4.0", "0.5.0", "0.6.0"] {
        let mut model = model(true);
        model.schema_version = version.into();
        let mut diagnostics = Vec::new();
        joint::classify_connectors(&model, &mut diagnostics);
        assert!(diagnostics
            .iter()
            .any(|finding| finding.code == joint::PRESSURE_INTERFACE_UNRESOLVED));
    }
}
#[test]
fn u5a_classifier_model_findings_precede_region_findings() {
    let mut model = model(true);
    activate_internal(&mut model);
    for (area, code) in [
        (
            json!({"value":-1.,"unit":"m^2"}),
            joint::PRESSURE_AREA_MISSING,
        ),
        (json!({"value":1.,"unit":"m"}), joint::PRESSURE_AREA_MISSING),
    ] {
        model.components[0].objective_connector.as_mut().unwrap()["pressure_model"]
            ["effective_area"] = area;
        let mut diagnostics = Vec::new();
        joint::classify_connectors(&model, &mut diagnostics);
        assert_eq!(
            diagnostics
                .iter()
                .map(|finding| finding.code.as_str())
                .collect::<Vec<_>>(),
            vec![code]
        );
    }
    for field in ["connector_axes_global", "stiffness"] {
        let mut model = self::model(true);
        activate_internal(&mut model);
        model.components[0]
            .objective_connector
            .as_mut()
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(field);
        let mut diagnostics = Vec::new();
        joint::classify_connectors(&model, &mut diagnostics);
        assert_eq!(
            diagnostics
                .iter()
                .map(|finding| finding.code.as_str())
                .collect::<Vec<_>>(),
            vec![joint::INPUT_INCOMPLETE]
        );
    }
}

fn reader_control(mode: PreviewSolverMode) -> Value {
    reader_control_for(mode, true)
}
fn reader_control_for(mode: PreviewSolverMode, anchored: bool) -> Value {
    let mut case = case(anchored);
    // The already classified internal inputs use today's v3 mechanics
    // context (B7b: the current 0.7.0 document with its exact contract).
    case.model.schema_version = "0.7.0".into();
    let materials = case
        .model
        .materials
        .iter()
        .cloned()
        .map(|mut material| {
            material.shear_modulus = Some(Quantity {
                value: 2e11 / 2.6,
                unit: "Pa".into(),
            });
            material
        })
        .collect::<Vec<_>>();
    let boundary = prepare_boundary(case.built.nodes.len(), &case.built.supports);
    let stiffness = form_basis_stiffness(&case.built, &boundary.springs, true).unwrap();
    let mut diagnostics = Vec::new();
    let mut solve = solve_prepared_load_case(
        &case.model,
        &case.built,
        &materials,
        &stiffness,
        &boundary.restrained_dofs,
        &boundary.springs,
        &case.model.load_cases[0],
        None,
        mode,
        None,
        &mut SourceRecoveryBudget::default(),
        None,
        Some(case.exact),
        &mut diagnostics,
        None,
    )
    .unwrap();
    assert!(!has_blocking(&diagnostics), "{diagnostics:?}");
    for row in &mut solve.results {
        row.basis_ref = Some(ResultBasisRef {
            ref_type: "load_case".into(),
            ref_id: solve.load_case_id.clone(),
        });
    }
    let envelope = MechanicsEnvelope {
        contract_evidence: Some(
            json!({"connector":joint::connector_evidence(&case.model, &case.built.connectors, &case.built.connector_records),"pressure":solve.pressure_evidence,"exact_cases":[solve.exact_case_evidence.unwrap()]}),
        ),
        schema_version: MECHANICS_SCHEMA_VERSION.into(),
        producer: mechanics_producer_for_model(&case.model),
        numerical_quality: assessed_numerical_quality(&case.model, &diagnostics),
        source_block_recovery: None,
        formulation_basis: formulation_basis_for_model(&case.model),
        document_kind: "openpipestress.product_preview.mechanics_result".into(),
        run_id: "run:dormant-u5a-reader-control".into(),
        model_ref: case.model.project.id,
        status: StatusEnvelope {
            mechanics: "MECHANICS_SOLVED".into(),
            rule_check: "RULE_INPUTS_INCOMPLETE".into(),
            professional_acceptance: "NOT_PROVIDED".into(),
        },
        summary: Summary {
            node_count: 4,
            segment_count: 3,
            support_count: case.model.supports.len(),
            load_case_count: 1,
            component_stress_modifier_count: 0,
            curved_bend_macro_element_count: 0,
            spring_hanger_user_input_count: 0,
            max_displacement: solve.max_displacement,
            // D-5: an exact-contract envelope names each basis's headline.
            max_open_formula_stress: None,
            max_open_formula_stress_nominal_section: Some(solve.max_stress),
            max_open_formula_stress_reduced_section: Some(solve.max_stress_reduced),
        },
        results: solve.results,
        diagnostics,
        professional_boundary: professional_boundary(),
        accepted_model_state_mutated: false,
    };
    serde_json::to_value(envelope).unwrap()
}

#[test]
fn u5a_prepared_publication_keeps_wall_force_and_the_joint_interface_identity() {
    for mode in [
        PreviewSolverMode::DenseScrutiny,
        PreviewSolverMode::SparseInteractive,
    ] {
        for anchored in [true, false] {
            let envelope = reader_control_for(mode, anchored);
            let rows = envelope["results"].as_array().unwrap();
            let s = rows
                .iter()
                .find(|row| row["kind"] == "connector_effective_axial_force_v1")
                .unwrap()["value"]
                .as_f64()
                .unwrap();
            let pi = 125. * std::f64::consts::PI;
            let mut checked = 0;
            for row in rows
                .iter()
                .filter(|row| row["kind"] == "pipe_wall_axial_force_v2")
            {
                let nw = row["value"].as_f64().unwrap();
                assert!(
                    (nw - pi - s).abs() < 1e-9,
                    "wall interface {}: {nw:e} {s:e}",
                    row["entity_ref"]
                );
                if !anchored {
                    assert!((nw - pi).abs() < 1e-9, "free wall force remains Pi");
                }
                checked += 1;
            }
            assert_eq!(checked, 10);
        }
    }
}

#[test]
fn u5a_prepared_composition_and_maintained_participating_reader_control() {
    let envelope = reader_control(PreviewSolverMode::DenseScrutiny);
    open_pipe_stress_result_export::semantic_contract::validate_pressure_evidence(&envelope)
        .unwrap();
    if std::env::var_os("U5A_READER_GENERATE").is_some() {
        println!("U5A_READER_JSON={}",serde_json::to_string(&json!({"scope":"invented dormant U5a prepared producer; result reader control, no public model 0.7.0 admission claim","envelope":envelope})).unwrap());
    } else {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/results/pressure_v3_joint_reader_control.json");
        let control: Value = serde_json::from_str(
            &std::fs::read_to_string(path)
                .expect("maintained control; generate explicitly with U5A_READER_GENERATE"),
        )
        .unwrap();
        assert_eq!(envelope, control["envelope"]);
    }
    let mut inactive = model(true);
    activate_internal(&mut inactive);
    let public = run_linear_static_preview_with_mode(
        LinearStaticPreviewRequest {
            model: inactive,
            materials: vec![],
        },
        PreviewSolverMode::DenseScrutiny,
    );
    let blocking = public.diagnostics.iter().filter(|d| d.severity == "blocking")
        .map(|d| (d.code.clone(), d.affected_refs.clone(), d.message.clone())).collect::<Vec<_>>();
    assert!(public.results.is_empty(), "{blocking:?}");
    // B7a recognized pressure-free 0.5.0 and refused its contract-present form
    // until B7b (PREVIEW_CONTRACT_VERSION_MISMATCH). B7b admits that form, so
    // the internal control (no authored provenance) is not a public document:
    // the public entry refuses it by the input's own named findings instead.
    assert!(!public.diagnostics.iter().any(|finding| finding.code == "PREVIEW_CONTRACT_VERSION_MISMATCH"), "{blocking:?}");
    assert!(!blocking.is_empty() && blocking.iter().all(|(code, _, _)| code == "PROVENANCE_INPUT_MISSING"), "{blocking:?}");
}

#[test]
fn u5a_formed_family_matches_exact_binary64_product_and_w2_scaling() {
    use open_pipe_stress_frame_kernel::structural::certify_load_primitive;
    let case = case(true);
    for primitive in case.force.load_primitives() {
        if let LoadFormula::JointEffectiveArea {
            pressure,
            effective_area,
            sign,
            ..
        } = &primitive.formula
        {
            let mut exact = ExactAccumulator::new();
            exact.add_product(*pressure, *effective_area).unwrap();
            let expected = exact.round().unwrap() * sign;
            let row = certify_load_primitive(primitive).unwrap();
            assert_eq!(row.published, expected);
            let mut scaled = primitive.clone();
            scaled.force_scale_exponent = 4;
            assert_eq!(
                certify_load_primitive(&scaled).unwrap().published,
                exact.round_scaled(4).unwrap() * sign
            );
        }
    }
}

#[test]
fn u5a_source_nonzero_fold_zero_keeps_primitive_and_absolute_formation_reserve() {
    let mut case = case(true);
    let area = f64::from_bits(1);
    case.built.connector_records[0]
        .pressure
        .as_mut()
        .unwrap()
        .effective_area = area;
    case.model.load_cases[0].pressure_regions.as_mut().unwrap()[0]
        .pressure
        .as_mut()
        .unwrap()
        .value = 0.25;
    let mut diagnostics = Vec::new();
    let exact = assemble_pressure_case(
        &case.model,
        &case.built,
        &case.model.materials,
        &case.model.load_cases[0],
        None,
        &mut diagnostics,
    )
    .unwrap();
    let mut checked = 0;
    for (index, sources) in exact.closure_sources.iter().enumerate() {
        if sources
            .iter()
            .any(|(_, formula)| matches!(formula, LoadFormula::JointEffectiveArea { .. }))
        {
            assert_eq!(exact.assembled_operands[index].1, 0.0);
            assert_eq!(exact.assembled_operand_bounds[index], f64::from_bits(1));
            assert_eq!(sources.len(), 1);
            checked += 1;
        }
    }
    assert_eq!(checked, 2);
}

#[test]
fn u5a_complete_manifest_kills_dropped_extra_duplicate_and_wrong_bindings_on_ordinary_and_w2() {
    use super::super::certified_loads::supplied_mutant;
    for mutation in 0..10 {
        supplied_mutant::arm(Box::new(move |_, supplied: &mut Vec<LoadPrimitive>| {
            let face = supplied
                .iter()
                .position(|p| p.identity.source.ends_with(":joint_face_cap"));
            let area = supplied
                .iter()
                .position(|p| matches!(p.formula, LoadFormula::JointEffectiveArea { .. }));
            let index = if matches!(mutation, 0 | 2 | 3 | 5) {
                let Some(index) = face.filter(|_| supplied.len() > 1) else {
                    return false;
                };
                index
            } else {
                let Some(index) = area else {
                    return false;
                };
                index
            };
            match mutation {
                0 | 1 => {
                    supplied.remove(index);
                }
                2 => supplied.push(supplied[index].clone()),
                3 => {
                    let mut extra = supplied[index].clone();
                    extra.identity.ordinal += 100;
                    supplied.push(extra);
                }
                4 => {
                    let LoadFormula::JointEffectiveArea { end, .. } = &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    *end = 1 - *end;
                }
                5 => {
                    let LoadFormula::PressureOperand {
                        geometry:
                            open_pipe_stress_frame_kernel::structural::PressureGeometry::Straight {
                                frame,
                            },
                        ..
                    } = &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    frame.node_i.index += 2;
                    frame.node_j.index += 2;
                }
                6 => {
                    let LoadFormula::JointEffectiveArea { effective_area, .. } =
                        &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    *effective_area *= 2.;
                }
                7 => {
                    let LoadFormula::JointEffectiveArea { pressure, .. } =
                        &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    *pressure *= 2.;
                }
                8 => {
                    let LoadFormula::JointEffectiveArea { connector, .. } =
                        &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    *connector = super::super::joint::build_connector(
                        &super::super::joint::connector_specs(&model(true))[0],
                        &[
                            FrameNode::new(0, [0.; 3]).unwrap(),
                            FrameNode::new(1, [2., 0., 0.]).unwrap(),
                            FrameNode::new(2, [2.3, 0., 0.]).unwrap(),
                        ],
                        &HashMap::from([("node:B", 0), ("node:C", 1)]),
                        Some(1),
                    )
                    .unwrap()
                    .0;
                }
                _ => {
                    let LoadFormula::JointEffectiveArea { axis, .. } = &mut supplied[index].formula
                    else {
                        unreachable!()
                    };
                    *axis = 1;
                }
            }
            true
        }));
        let case = case(true);
        assert!(
            supplied_mutant::disarm().is_some(),
            "mutant {mutation} not armed"
        );
        for solution in solutions(&case) {
            assert_ne!(
                solution.report.quality,
                SolveQuality::Passed,
                "mutant {mutation}"
            );
            assert!(solution.member_bound.is_none());
            assert!(
                format!("{:?}", solution.formation_check).contains("closure"),
                "{:?}",
                solution.formation_check
            );
        }
    }
}
