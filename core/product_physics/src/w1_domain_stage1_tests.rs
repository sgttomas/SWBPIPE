//! W1 basis custody (B7a stage 1, RV125 N-2) and declared member loads (stage 2),
//! through actual preparation, native solve, freeze, publication and the full
//! Rust reader, including c = 3 and the registered/unregistered public entries.
//! Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
use super::*;

/// These tests read only the D1 domain, which never reads the caller's limit L.
const DOMAIN_ONLY: RetainedByteLimit = RetainedByteLimit::new(0);

/// n05's sensitive torque case and a renamed copy (`case-b`), each optionally naming a
/// temperature point whose E and G equal the base material's (the numerics are the base's).
fn two_torque_cases(selectors: [Option<&str>; 2]) -> Value {
    torque_cases(&selectors, &|_| 1.0)
}

/// n05's sensitive torque case and renamed copies (`case-b`, `case-c`, ...), one per selector,
/// each optionally naming a temperature point whose E and G are the base material's (Pa)
/// times `scale(point)`.
fn torque_cases(selectors: &[Option<&str>], scale: &dyn Fn(&str) -> f64) -> Value {
    let cases = (0..selectors.len()).map(|case| {
        let mut copy = n05_torques();
        if case > 0 {
            let tag = char::from(b'a' + u8::try_from(case).unwrap());
            copy["id"] = json!(format!("case-{tag}"));
            for load in copy["primitive_loads"].as_array_mut().unwrap() {
                load["id"] = json!(format!("{tag}:{}", load["id"].as_str().unwrap()));
            }
        }
        copy
    }).collect();
    let mut raw = n05_request(cases);
    let material = &mut raw["model"]["materials"][0];
    let (e, g) = (material["elastic_modulus"]["value"].as_f64().unwrap(), material["shear_modulus"]["value"].as_f64().unwrap());
    let mut points = Vec::new();
    for id in selectors.iter().flatten() {
        if !points.iter().any(|p: &Value| p["id"] == json!(id)) {
            points.push(json!({"id": id, "temperature": {"value": 20.0, "unit": "degC"},
                "elastic_modulus": {"value": e * scale(id), "unit": "Pa"}, "shear_modulus": {"value": g * scale(id), "unit": "Pa"},
                "provenance": INVENTED}));
        }
    }
    material["temperature_points"] = json!(points);
    for (case, selector) in selectors.iter().enumerate() {
        if let Some(id) = selector {
            raw["model"]["load_cases"][case]["modulus_basis_ref"] = json!(id);
        }
    }
    raw
}

/// The actual W1 stages through freeze, with `before_freeze` applied to the prepared cases.
fn stepped(raw: &Value, mode: PreviewSolverMode, before_freeze: &dyn Fn(&mut rp::PreparedCases)) -> Vec<String> {
    let (capture, observer, mut ordinary) = early_observed(raw, mode, true);
    ordinary.diagnostics.shrink_to_fit();
    let cases = w1_case_ids(&capture).unwrap();
    let triggers: Vec<CaseTrigger> = case_triggers(&ordinary.numerical_quality, &observer.ordinary, cases.ids()).collect();
    let attempted = cases.attempted(triggers.iter().copied());
    let mut prepared = match observer.prepare_cases(ordinary, cases.ids().len(), attempted.requests()) {
        Ok(prepared) => prepared,
        Err(failure) => panic!("custody: {:?}", failure.error),
    };
    prepared.native();
    before_freeze(&mut prepared);
    prepared.freeze();
    prepared.attempts.iter().map(|attempt| match &attempt.end {
        rp::AttemptEnd::Frozen(_) => "Frozen".to_owned(),
        rp::AttemptEnd::Candidate(refused) => format!("Candidate({:?})", refused.error),
        rp::AttemptEnd::Preparation => format!("Preparation({})", prepared.capture.with_case(attempt.request, |c| format!("{:?}", c.error))),
        rp::AttemptEnd::Prepared => "Prepared".to_owned(),
        rp::AttemptEnd::Native => format!("Native({:?})", attempt.trace.native_error),
        rp::AttemptEnd::Selected => "Selected".to_owned(),
    }).collect()
}

/// RV125 N-2: at c = 2, a base case and a named case, a named case first, one point reused by
/// both cases, and two distinct points each bind their own basis custody, so both attempted cases
/// freeze. Binding a case to another case's custody, or a base case to any, refuses that case's
/// freeze by its custody cause, and leaves the other case frozen.
#[test]
fn b7a_selector_cases_bind_their_own_basis_custody() {
    let shapes: [(&str, [Option<&str>; 2], usize, [Option<usize>; 2]); 4] = [
        ("base, named", [None, Some("P1")], 1, [None, Some(0)]),
        ("named, base", [Some("P1"), None], 1, [Some(0), None]),
        ("reused", [Some("P1"), Some("P1")], 1, [Some(0), Some(0)]),
        ("distinct", [Some("P1"), Some("P2")], 2, [Some(0), Some(1)]),
    ];
    for (label, selectors, bases, bound) in shapes {
        let raw = two_torque_cases(selectors);
        for mode in MODES {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None, "{label}");
            let (_, observer, ordinary) = early_observed(&raw, mode, true);
            assert_eq!(ordinary.status.mechanics, "MECHANICS_SOLVED", "{label}");
            assert_eq!(ordinary.results.iter().filter(|r| r.kind == "modulus_basis_record").count(),
                selectors.iter().flatten().count(), "{label}: one modulus row per named case");
            assert_eq!(observer.bases.len(), bases, "{label} {mode:?}: one custody per distinct basis");
            assert!(observer.bases.iter().all(|b| b.record_calls == 1 && b.record.is_some()), "{label}");
            for case in 0..2 {
                assert_eq!(observer.member_loads_basis_at(case), (false, selectors[case].is_some(), bound[case]), "{label} case {case}");
            }
            assert_eq!(stepped(&raw, mode, &|_| {}), ["Frozen", "Frozen"], "{label} {mode:?}");
            // Wrong custody: a base case bound to a basis, a named case to none, or a named case to
            // another basis's custody (whose record text differs).
            let wrong = match label {
                "base, named" => (0, Some(0)),
                "named, base" | "reused" => (0, None),
                _ => (1, Some(0)),
            };
            let ends = stepped(&raw, mode, &|prepared| {
                prepared.capture.with_case(wrong.0, |c| c.basis = wrong.1);
            });
            assert!(ends[wrong.0].starts_with("Candidate(") && (ends[wrong.0].contains("basis record capture presence")
                || ends[wrong.0].contains("modulus record association")), "{label} {mode:?}: {ends:?}");
            assert_eq!(ends[1 - wrong.0], "Frozen", "{label} {mode:?}: the other case keeps its own custody");
        }
    }
}

/// Stage 2: authored full-span uniform loads keep per-case custody and use
/// member-aware native formation while the adjacent nodal case stays unchanged.
#[test]
fn w1_member_load_cases_form_and_freeze_in_both_case_orders() {
    let udl = |case: &str| json!({"id": format!("{case}:udl"), "category": "distributed_force", "target": {"type": "element", "pipe": "pipe"},
        "direction": "global_x", "magnitude": {"value": 10.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": INVENTED});
    for loaded in [1usize, 0] {
        let mut raw = two_torque_cases([None, None]);
        let id = raw["model"]["load_cases"][loaded]["id"].as_str().unwrap().to_owned();
        raw["model"]["load_cases"][loaded]["primitive_loads"].as_array_mut().unwrap().push(udl(&id));
        for mode in MODES {
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None);
            let (_, observer, _) = early_observed(&raw, mode, true);
            assert!(observer.error.is_none() && observer.basis_error.is_none(), "{:?}", observer.error);
            for case in 0..2 {
                assert_eq!(observer.member_loads_basis_at(case).0, case == loaded, "case {case}");
                assert!(observer.source_terminal_at(case).0, "case {case}: nodal and member cases have a source");
                assert_eq!(observer.member_source_at(case).is_some(), case==loaded);
            }
            let ends = stepped(&raw, mode, &|_| {});
            assert_eq!(ends, ["Frozen", "Frozen"], "loaded {loaded} {mode:?}");
        }
    }
    // The early hook of a selector case is required: a named case whose basis-state capture never
    // ran keeps its pending slot, and installing it refuses that case.
    let raw = two_torque_cases([None, Some("P1")]);
    let (request, _) = source_receipt::CapturedInvocation::parse(raw, PreviewSolverMode::SparseInteractive).unwrap();
    let mut observer = rp::ProductCapture::prepared_probe_on(rp::W1Route::Preview);
    let refused = observer.test_install_pending(&request.model.load_cases[1]);
    assert_eq!(refused.map_err(|e| e.to_string()), Err("early basis source pending".to_owned()));
}

/// B7a (b), stage 1 (review F3 b, c, e): at c = 3, with numerically distinct points (P1 is 1.25
/// times the base E and G, P2 0.8 times), each case's early source carries its own basis's
/// moduli (a selector case is captured at its basis state, not the base model's), every case
/// freezes, and the serialized receipt carries one basis per selector in first-use order, with
/// its moduli, and each case's own `material_basis_ref` on its ordinary attempt, product
/// attempt and source, also when a point is reused around a base case. W1 publishes that
/// successor (precommit's copy), and the full Rust reader, G0 first, finds it bound and
/// numerically eligible with the invocation.
#[test]
fn b7a_three_cases_capture_and_serialize_their_own_bases() {
    use crate::retained_tests_hooks as hooks;
    let scale = |id: &str| match id {
        "P1" => 1.25,
        "P2" => 0.8,
        other => panic!("point {other}"),
    };
    let (e, g) = (2.0e11, 8.0e10);
    for selectors in [[None, Some("P2"), Some("P1")], [Some("P1"), None, Some("P1")], [Some("P2"), Some("P2"), None]] {
        // The receipt's bases, one per selector in first-use order, and each case's index into
        // them; the capture's custody holds the named ones in the same order.
        let mut order: Vec<Option<&str>> = Vec::new();
        for selector in selectors {
            if !order.contains(&selector) {
                order.push(selector);
            }
        }
        let refs: Vec<usize> = selectors.iter().map(|s| order.iter().position(|o| o == s).unwrap()).collect();
        let named: Vec<&str> = order.iter().flatten().copied().collect();
        let raw = torque_cases(&selectors, &scale);
        for mode in MODES {
            let label = format!("{selectors:?} {mode:?}");
            let (request, capture) = source_receipt::CapturedInvocation::parse(raw.clone(), mode).unwrap();
            assert_eq!(retained_memory::assess(&capture, &request, retained_memory::Entry::Direct, DOMAIN_ONLY).law().domain, None, "{label}");
            let (_, observer, _) = early_observed(&raw, mode, true);
            assert_eq!(observer.bases.len(), named.len(), "{label}: one custody per named basis");
            for (case, selector) in selectors.iter().enumerate() {
                let custody = selector.map(|id| named.iter().position(|n| *n == id).unwrap());
                assert_eq!(observer.member_loads_basis_at(case), (false, selector.is_some(), custody), "{label} case {case}");
                let s = selector.map_or(1.0, scale);
                assert_eq!(observer.source_moduli_at(case), Some(vec![(e * s, g * s)]), "{label} case {case}: the source at its basis state");
            }
            assert_eq!(stepped(&raw, mode, &|_| {}), ["Frozen", "Frozen", "Frozen"], "{label}");
            let (capture, observer, mut ordinary) = early_observed(&raw, mode, true);
            ordinary.diagnostics.shrink_to_fit();
            let ((_, retained), _, precommit) = hooks::counted_with_successor(|| retained_w1(observer, ordinary, &capture));
            let successor = match retained {
                Ok(successor) => successor.value().clone(),
                Err(fallback) => panic!("{label}: W1 fell back: {fallback:?}"),
            };
            assert_eq!(precommit.as_ref(), Some(&successor), "{label}: the published successor is precommit's copy");
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let verdict = reader::validate(&successor, Some(&invocation)).unwrap_or_else(|e| panic!("{label}: reader: {e:?}"));
            assert!(verdict.invocation_bound && verdict.numerical_eligible, "{label}: bound and eligible");
            let body = &successor["retained_precision"]["body"];
            let bases = body["material_bases"].as_array().unwrap();
            assert_eq!(bases.len(), order.len(), "{label}");
            for (index, (basis, selector)) in bases.iter().zip(&order).enumerate() {
                let (want, selection, s) = match selector {
                    None => (json!({"kind": "base"}), json!({"kind": "base"}), 1.0),
                    Some(id) => (json!({"kind": "named", "id": id}), json!({"kind": "named_point", "point_id": id}), scale(id)),
                };
                let cases: Vec<usize> = (0..3).filter(|&case| refs[case] == index).collect();
                assert_eq!((&basis["index"], &basis["selector"], &basis["case_indices"]), (&json!(index), &want, &json!(cases)), "{label}");
                let material = &basis["materials"][0];
                assert_eq!(material["selection"], selection, "{label}");
                assert_eq!(material["elastic_modulus"], json!(format!("{:016x}", (e * s).to_bits())), "{label} basis {index}");
                assert_eq!(material["shear_modulus"], json!(format!("{:016x}", (g * s).to_bits())), "{label} basis {index}");
            }
            for case in 0..3 {
                assert_eq!(body["cases"][case]["status"], "selected", "{label} case {case}");
                assert_eq!(body["ordinary_attempts"][case]["material_basis_ref"], json!(refs[case]), "{label} case {case}");
            }
            let attempts = body["product_attempts"].as_array().unwrap();
            let sources = body["sources"].as_array().unwrap();
            assert_eq!((attempts.len(), sources.len()), (3, 3), "{label}");
            for attempt in attempts {
                let case = usize::try_from(attempt["owner_ref"]["index"].as_u64().unwrap()).unwrap();
                assert_eq!(attempt["material_basis_ref"], json!(refs[case]), "{label}: product attempt of case {case}");
            }
            for source in sources {
                let case = usize::try_from(source["owner"]["case_index"].as_u64().unwrap()).unwrap();
                assert_eq!(source["material_basis_ref"], json!(refs[case]), "{label}: source of case {case}");
            }
        }
    }
}

/// At c=3 each member sidecar follows its own selector, including reuse around
/// a base case, while distinct intensities keep the declared ledgers distinct.
#[test]
fn w1_three_member_cases_keep_distinct_and_reused_basis_custody() {
    use crate::retained_tests_hooks as hooks;
    let scale = |id: &str| if id == "P1" { 1.25 } else { 0.8 };
    for selectors in [[None, Some("P2"), Some("P1")], [Some("P1"), None, Some("P1")]] {
        let mut raw = torque_cases(&selectors, &scale);
        for case in 0..3 {
            let id = raw["model"]["load_cases"][case]["id"].as_str().unwrap().to_owned();
            let intensity=[10.0,-7.0,3.0][case];
            raw["model"]["load_cases"][case]["primitive_loads"].as_array_mut().unwrap().push(json!({
                "id":format!("{id}:distinct-member-load"),"category":"distributed_force",
                "target":{"type":"element","pipe":"pipe"},"direction":"global_x",
                "magnitude":{"value":intensity,"unit":"N/m"},
                "dimension":"force_per_length","provenance":INVENTED}));
        }
        for mode in MODES {
            let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
            ordinary.diagnostics.shrink_to_fit();
            let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
            let successor=retained.unwrap_or_else(|e|panic!("{selectors:?} {mode:?}: {e:?}")).value().clone();
            assert_eq!(precommit.as_ref(),Some(&successor));
            let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
            let verdict=reader::validate(&successor,Some(&invocation)).unwrap();
            assert!(verdict.invocation_bound && verdict.numerical_eligible);
            let body=&successor["retained_precision"]["body"];
            for case in 0..3 {
                assert_eq!(body["cases"][case]["status"],"selected");
                let source=body["sources"].as_array().unwrap().iter()
                    .find(|s|s["owner"]["case_index"]==case).unwrap();
                let basis=source["material_basis_ref"].as_u64().unwrap() as usize;
                let expected=match selectors[case] {
                    None=>json!({"kind":"base"}),
                    Some(id)=>json!({"kind":"named","id":id}),
                };
                assert_eq!(body["material_bases"][basis]["selector"],expected);
                assert_eq!(body["ordinary_attempts"][case]["material_basis_ref"],basis);
                assert_eq!(source["member_load_terms"].as_array().unwrap().len(),1);
                assert_eq!(source["member_load_terms"][0]["source_id"],
                    raw["model"]["load_cases"][case]["primitive_loads"].as_array().unwrap().last().unwrap()["id"]);
                assert_eq!(source["member_load_terms"][0]["intensity"],
                    json!([format!("{:016x}",[10.0_f64,-7.0,3.0][case].to_bits()),
                        "0000000000000000","0000000000000000"]));
            }
            if selectors[0]==selectors[2] {
                assert_eq!(body["sources"][0]["material_basis_ref"],body["sources"][2]["material_basis_ref"]);
                assert_ne!(body["cases"][0]["selection"]["ledger_sha256"],body["cases"][2]["selection"]["ledger_sha256"]);
            }
        }
    }
}

/// Invented numerical input for exact section/transport controls; it is not an
/// engineering example or an external physics reference.
fn w1_interior_member_request(asymmetric:bool)->Value {
        let mut raw=n05_request(vec![n05_torques()]);
        let provenance=INVENTED;
        // This invented numerical control makes the represented Z hull a
        // point and keeps source I rounding below 4.159e-22 relative. Scaling
        // by Z gives dyadic component stresses, independently of MPa transport.
        let z_control=f64::from_bits(0x3f3c82e3dc7ef90c);
        let w=3.0*z_control;
        assert_eq!(w.to_bits(),0x3f55622ae55f3ac9);
        raw["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"]=json!(0.25);
        raw["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"]=json!(f64::from_bits(0x3f847ae147ad8ea5));
        raw["model"]["supports"]=json!([
            {"id":"root-pin","family":"anchor","node":"root","restraints":["UX","UY","UZ","RX"],"provenance":provenance},
            {"id":"tip-pin","family":"anchor","node":"tip","restraints":["UX","UY","UZ","RX"],"provenance":provenance}]);
        raw["model"]["load_cases"][0]["primitive_loads"]=json!([
            {"id":"member:transverse","category":"distributed_force","target":{"type":"element","pipe":"pipe"},
                "direction":"global_y","magnitude":{"value":w,"unit":"N/m"},"dimension":"force_per_length","provenance":provenance}]);
        if asymmetric {
            for (id,node,value) in [("root:moment","root",z_control),("tip:moment","tip",-2.0*z_control)] {
                raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(json!({
                    "id":id,"category":"concentrated_moment","target":{"type":"node","node":node},
                    "direction":"RZ","magnitude":{"value":value,"unit":"N*m"},"dimension":"moment","provenance":provenance}));
            }
        }
        raw
}

/// Exercise the real preparation/native/freeze/overlay path on a simply
/// supported beam, entering the existing private proof driver directly.
#[test]
fn w1_loaded_maximum_is_the_kernel_projection_with_member_spans() {
    for asymmetric in [false,true] {
        let raw=w1_interior_member_request(asymmetric);
        let z_control=f64::from_bits(0x3f3c82e3dc7ef90c);
        for mode in MODES {
            let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
            let mut prepared=rp::PreparedCase::prepare_observed(request,mode,&capture)
                .unwrap_or_else(|f|panic!("preparation: {:?}",f.capture.error));
            let ordinary=serde_json::to_vec(prepared.ordinary()).unwrap();
            prepared.solve_native().unwrap();
            let candidate=prepared.project_candidate().unwrap_or_else(|f| {
                if let rp::PreparedCandidateError::Proof(p)=&f.error {
                    if let Some(d)=p.work().loaded_maximum_diagnostic() {
                        println!("W1_PP_INTERVAL_DIAGNOSTIC {}",json!({"row":d.row,"member":d.member,"lower":d.lower,"upper":d.upper}));
                    }
                }
                panic!("asymmetric={asymmetric} {mode:?}: {} rows8={:?} row75={:?}",
                    format!("{:?}",f.error).split(", work:").next().unwrap(),
                    f.ordinary.results.get(8).map(|r|(&r.id,&r.kind,&r.unit,r.value)),
                    f.ordinary.results.get(75).map(|r|(&r.id,&r.kind,&r.unit,r.value)))
            });
            assert!(candidate.certificate().passed());
            let e=candidate.envelope();
            let maximum=e.results.iter().find(|r|r.kind=="pipe_elastic_normal_stress_maximum_v2").unwrap();
            let evidence=&e.contract_evidence.as_ref().unwrap()["preview_cases"][0]["pipe_stress_extrema"][0];
            let lower=evidence["value_lower_pa"].as_f64().unwrap();
            let upper=evidence["value_upper_pa"].as_f64().unwrap();
            assert_eq!((lower+0.5*(upper-lower)).to_bits(),maximum.value.to_bits());
            assert_eq!(evidence["global_upper_bound_pa"],evidence["value_upper_pa"]);
            let z=candidate.capture().facts[0].section_modulus;
            assert_eq!(z.to_bits(),z_control.to_bits());
            // Re-run the existing bounder with independently assembled source
            // arguments. Its witness upper can be below the global upper.
            let source=candidate.capture().source.as_ref().unwrap();
            let m=&source.members()[0];let f=&candidate.capture().facts[0];
            let pipe=StraightPipeElement{element_id:"pipe".into(),
                node_i:FrameNode{index:m.node_i as usize,coordinates:source.nodes()[m.node_i as usize]},
                node_j:FrameNode{index:m.node_j as usize,coordinates:source.nodes()[m.node_j as usize]},
                section:StraightPipeSectionProperties{elastic_modulus:m.elastic_modulus,shear_modulus:m.shear_modulus,
                    area:m.area,second_moment_y:m.second_moment_y,second_moment_z:m.second_moment_z,
                    torsion_constant:m.torsion_constant,mass_per_length:None,
                    beam_theory:open_pipe_stress_frame_kernel::BeamTheory::EulerBernoulli},y_reference:m.y_reference};
            let section=DerivedSection{area:f.area,internal_area:0.0,second_moment:f.second_moment,
                torsion_constant:f.torsion_constant,section_modulus:f.section_modulus,torsion_radius:f.radius,
                wall_thickness:f.effective_wall};
            let names=["force:pipe:axial","force:pipe:shear-y","force:pipe:shear-z",
                "moment:pipe:torsion","moment:pipe:bending-y","moment:pipe:bending-z"];
            let ends:[f64;12]=std::array::from_fn(|j| {
                let id=format!("result:{}{}",names[j%6],if j<6{""}else{":end-j"});
                e.results.iter().find(|r|r.id==id).unwrap().value
            });
            let loads=straight_local_uniform_loads(&pipe,0,&[
                open_pipe_stress_primitive_loads::ElementUniformLoadContribution{
                    load_id:"member:transverse".into(),element_index:0,
                    direction:open_pipe_stress_primitive_loads::LoadDirection::GlobalY,
                    magnitude:open_pipe_stress_primitive_loads::LoadQuantity{value:3.0*z_control,
                        dimension:open_pipe_stress_primitive_loads::LoadDimension::ForcePerLength},extent:None}
            ]).unwrap();
            let pp=exact_straight_summary_extrema(&pipe,&ends,&loads,&section,None).unwrap();
            assert!(lower<=pp.value_lower&&upper>=pp.upper_bound,"evidence must contain the complete PP global interval");
            if asymmetric {assert!(pp.value_upper<pp.upper_bound,"the witness/global upper distinction is active");}
            let a=candidate.capture().facts[0].area;
            let end_max=[("result:force:pipe:axial","result:moment:pipe:bending-z"),
                ("result:force:pipe:axial:end-j","result:moment:pipe:bending-z:end-j")].into_iter()
                .map(|(n,m)|e.results.iter().find(|r|r.id==n).unwrap().value.abs()/a
                    +e.results.iter().find(|r|r.id==m).unwrap().value.abs()/z).fold(0.0_f64,f64::max);
            assert!(maximum.value>end_max,"the active interior maximum must exceed both published end values");
            assert_eq!(maximum.value.to_bits(),if asymmetric{0x4008555555555555}else{0x3ff8000000000000},
                "independent true-annulus/represented-lane midpoint reference");
            if !asymmetric {
                assert_eq!(evidence["station_fraction"],0.5);
            }
            println!("W1_LOADED_PP {}",json!({"asymmetric":asymmetric,"mode":mode.as_str(),
                "value":maximum.value,"lower":lower,"upper":upper,"station":evidence["station_fraction"],
                "ordinary_sha256":sha(&ordinary)}));
        }
    }
}

/// Add the loaded beam without changing the sensitive ordinary member.
pub(super) fn w1_interior_publication_request(asymmetric:bool,tip_scale:f64)->Value {
        let mut beam=w1_interior_member_request(asymmetric);
        if asymmetric {
            // At tip_scale=-3 the independent stress polynomial is
            // -1-8t+6t²: absolute maximum 11/3 at t=2/3, exact reaction 4Z.
            // The -2 control instead keeps the prototype's 73/24 maximum and
            // the exact (7/2)Z scale halfway used by the negative G5b check.
            let tip=beam["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap()
                .iter_mut().find(|l|l["id"]=="tip:moment").unwrap();
            tip["magnitude"]["value"]=json!(tip_scale*f64::from_bits(0x3f3c82e3dc7ef90c));
        }
        // The original torque case supplies the genuine ordinary sensitivity
        // trigger. Its independent, unloaded member retains the original path.
        let mut raw=n05_request(vec![n05_torques()]);
        for node in beam["model"]["nodes"].as_array().unwrap() {
            let mut node=node.clone();
            node["id"]=json!(format!("ss:{}",node["id"].as_str().unwrap()));
            node["position"]["x"]=json!(node["position"]["x"].as_f64().unwrap()+4.0);
            raw["model"]["nodes"].as_array_mut().unwrap().push(node);
        }
        for pipe in beam["model"]["pipe_segments"].as_array().unwrap() {
            let mut pipe=pipe.clone();
            for key in ["id","from","to"] {pipe[key]=json!(format!("ss:{}",pipe[key].as_str().unwrap()));}
            raw["model"]["pipe_segments"].as_array_mut().unwrap().push(pipe);
        }
        for support in beam["model"]["supports"].as_array().unwrap() {
            let mut support=support.clone();
            for key in ["id","node"] {support[key]=json!(format!("ss:{}",support[key].as_str().unwrap()));}
            raw["model"]["supports"].as_array_mut().unwrap().push(support);
        }
        for load in beam["model"]["load_cases"][0]["primitive_loads"].as_array().unwrap() {
            let mut load=load.clone();
            load["id"]=json!(format!("ss:{}",load["id"].as_str().unwrap()));
            let key=if load["target"]["type"]=="element"{"pipe"}else{"node"};
            load["target"][key]=json!(format!("ss:{}",load["target"][key].as_str().unwrap()));
            raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().push(load);
        }
        raw
}

/// An independent sensitive member supplies a genuine W1 trigger while the
/// loaded beam exercises complete private/public publication and G8 binding.
#[test]
fn w1_member_interior_publications_bind_the_actual_direct_entry() {
    for asymmetric in [false,true] {
        let raw=w1_interior_publication_request(asymmetric,-3.0);
        for mode in MODES {
            let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
            ordinary.diagnostics.shrink_to_fit();
            let plain_bytes=serde_json::to_vec(&ordinary).unwrap();
            let ((envelope,retained),_,captured)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
            if retained.is_err() {
                if let (Some(dir),Some(candidate))=(std::env::var_os("W1_MEMBER_INTERIOR_OUT"),captured.as_ref()) {
                    std::fs::write(std::path::Path::new(&dir).join(format!("FAILED_precommit_{asymmetric}_{}.json",mode.as_str())),
                        serde_json::to_string_pretty(&json!({"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()},
                            "status":"unreviewed_precommit_failure"})).unwrap()).unwrap();
                }
            }
            let successor=retained.unwrap_or_else(|e| {
                let (_,observer,ordinary)=early_observed(&raw,mode,true);
                let mut p=observer.prepare_cases(ordinary,1,&[0]).unwrap_or_else(|f|panic!("{:?}",f.error));
                p.native();p.freeze();
                let failures:Vec<_>=p.attempts.iter().map(|a|match &a.end {
                    rp::AttemptEnd::Candidate(f)=>format!("{}",format!("{:?}",f.error).split(", work:").next().unwrap()),
                    rp::AttemptEnd::Frozen(_)=>"Frozen".into(),
                    _=>"Other".into(),
                }).collect();
                panic!("{asymmetric} {mode:?}: {e:?}: {failures:?}")
            }).value().clone();
            assert!(serde_json::to_vec(&envelope).unwrap()==plain_bytes,"ordinary owner changed on a successful W1 transfer");
            assert_eq!(captured,Some(successor.clone()));
            let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
            let verdict=reader::validate(&successor,Some(&invocation)).unwrap();
            assert!(verdict.invocation_bound&&verdict.numerical_eligible);
            assert_eq!(successor["retained_precision"]["body"]["cases"][0]["status"],"selected");
            let maximum=successor["results"].as_array().unwrap().iter()
                .find(|r|r["kind"]=="pipe_elastic_normal_stress_maximum_v2"&&r["entity_ref"]=="ss:pipe").unwrap();
            assert_eq!(maximum["value"].as_f64().unwrap().to_bits(),
                if asymmetric{0x400d555555555555}else{0x3ff8000000000000});
            let direct_raw=raw.clone();
            let (direct,_,precommit)=hooks::counted_with_successor(||
                run_linear_static_preview_value_with_retained_direct(direct_raw,mode,RETAINED_LIMIT).unwrap());
            if registered() {
                assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Registered);
                assert_eq!(precommit,Some(successor.clone()));
                assert_eq!(published(direct),serde_json::to_vec(&successor).unwrap());
            } else {
                assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Stale);
                assert!(precommit.is_none()&&direct.successor().is_none());
                assert_eq!(published(direct),plain_bytes);
            }
            if let Some(dir)=std::env::var_os("W1_MEMBER_INTERIOR_OUT") {
                let shape=if asymmetric{"asymmetric"}else{"symmetric"};
                let id=format!("w1_member_interior_{shape}_{}",mode.as_str());
                std::fs::write(std::path::Path::new(&dir).join(format!("{id}.json")),
                    serde_json::to_string_pretty(&json!({"id":id,"source":successor,"invocation":invocation,
                        "generation_profile":if registered(){"registered_direct"}else{"stale_private_driver"}})).unwrap()+"\n").unwrap();
            }
        }
    }
}

/// W1 coverage repair §2.2 (corner 2): native quantity scales and published-row maxima
/// straddle an exact binary64 halfway in this input (force maximum (7/2)Z, coupled
/// moment 7Z). The receipt now states the final scales the product gate judged every
/// row with, so precommit G5b holds and the successor publishes in both modes; the
/// unchanged reader recomputes the same scales. Formerly
/// `w1_loaded_scale_halfway_keeps_the_existing_g5b_refusal`: the cause that refusal
/// recorded, the receipt's native provenance, is repaired, and no gate changed.
/// Maintained writer of the corpus bases `w1_member_halfway_asymmetric_{mode}`: with
/// `W1_MEMBER_HALFWAY_OUT` naming a folder, each mode's `{id, source, invocation,
/// generation_profile}` is written there after the public Direct entry's relation holds.
#[test]
fn w1_loaded_scale_halfway_publishes() {
    let raw=w1_interior_publication_request(true,-2.0);
    let z=f64::from_bits(0x3f3c82e3dc7ef90c);
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let bytes=serde_json::to_vec(&ordinary).unwrap();
        let ((envelope,retained),_,candidate)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        let successor=retained.unwrap_or_else(|f|panic!("{mode:?}: {f:?}")).value().clone();
        assert_eq!(candidate.as_ref(),Some(&successor));
        assert!(serde_json::to_vec(&envelope).unwrap()==bytes,"the ordinary owner is untouched");
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound&&verdict.numerical_eligible);
        let case=&successor["retained_precision"]["body"]["cases"][0];
        assert_eq!(case["status"],"selected");
        let maximum=successor["results"].as_array().unwrap().iter()
            .find(|r|r["kind"]=="pipe_elastic_normal_stress_maximum_v2"&&r["entity_ref"]=="ss:pipe").unwrap();
        assert_eq!(maximum["value"].as_f64().unwrap().to_bits(),0x4008555555555555,"RN64(73/24) at t = 7/12");
        // The loaded body's force scale is the final row maximum. (7/2)Z is an exact
        // binary64 tie; the published row is RN64 of the projected hull midpoint, observed
        // here to equal the tie's even neighbour RN64((7/2)Z), so this pin is an observed
        // outcome, not a property of the law. The coupled moment scale is twice it
        // (extent 2), RN64(7Z).
        let scales=&case["selection"]["body_scales"][1];
        assert_eq!(scales["force"],json!(format!("{:016x}",(3.5*z).to_bits())));
        assert_eq!(scales["moment"],json!(format!("{:016x}",(7.0*z).to_bits())));
        // The public Direct entry publishes exactly this successor in the registered
        // build and the unchanged ordinary bytes otherwise (as the interior controls).
        let direct_raw=raw.clone();
        let (direct,_,precommit)=hooks::counted_with_successor(||
            run_linear_static_preview_value_with_retained_direct(direct_raw,mode,RETAINED_LIMIT).unwrap());
        if registered() {
            assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Registered);
            assert_eq!(precommit,Some(successor.clone()));
            assert_eq!(published(direct),serde_json::to_vec(&successor).unwrap());
        } else {
            assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Stale);
            assert!(precommit.is_none()&&direct.successor().is_none());
            assert_eq!(published(direct),bytes);
        }
        if let Some(dir)=std::env::var_os("W1_MEMBER_HALFWAY_OUT") {
            let id=format!("w1_member_halfway_asymmetric_{}",mode.as_str());
            std::fs::write(std::path::Path::new(&dir).join(format!("{id}.json")),
                serde_json::to_string_pretty(&json!({"id":id,"source":successor,"invocation":invocation,
                    "generation_profile":if registered(){"registered_direct"}else{"stale_private_driver"}})).unwrap()+"\n").unwrap();
        }
    }
}

/// W1 coverage repair §2.2 and §5: only the receipt's `body_scales` change provenance. The
/// native publication keeps its own scales, the coupled maxima of its own rows, which its
/// certificate judged with. On the 73/24 Pa control they differ from the gate's final scales
/// at the loaded body, so a native scale replaced by a final one is visible here.
#[test]
fn w1_native_scales_stay_native_evidence() {
    let raw=w1_interior_publication_request(true,-2.0);
    for mode in MODES {
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut prepared=rp::PreparedCase::prepare_observed(request,mode,&capture)
            .unwrap_or_else(|f|panic!("{:?}",f.capture.error));
        prepared.solve_native().unwrap();
        let candidate=prepared.project_candidate().unwrap_or_else(|_|panic!("{mode:?}: the control certifies"));
        let (_,case)=candidate.capture().native_pair().unwrap();
        let k::ExecutionOutcome::Selected(owner)=&case.outcome else{panic!("{mode:?}: native selected")};
        let native=&owner.evidence().body_scales;
        let final_scales=candidate.certificate().final_scales();
        assert_eq!(native.len(),4*final_scales.len());
        let publication=owner.publish();
        assert_eq!(&publication.body_scales,native,"the evidence states the native publication's scales");
        // Body 1 (extent 2): the coupled maxima of the native rows themselves.
        let mut s=[0.0f64;4];
        for row in publication.rows.iter().filter(|r|r.body==1&&r.class!=k::RowClass::InputDerived) {
            let slot=[k::Kind::Translation,k::Kind::Rotation,k::Kind::Force,k::Kind::Moment].iter().position(|q|*q==row.kind).unwrap();
            if let Some(x)=row.value.value() {s[slot]=s[slot].max(x.abs());}
        }
        // Coupling with extent 2, written out here rather than through the kernel's
        // `coupled_scales`: [max(T, 2R), max(R, T/2), max(F, M/2), max(M, 2F)].
        let [t,r,f,m]=s;
        let expected=[t.max(2.0*r),r.max(t/2.0),f.max(m/2.0),m.max(2.0*f)];
        for (kind,want) in expected.iter().enumerate() {
            let (body,_,bits)=native[4+kind];
            assert_eq!((body,bits),(1,want.to_bits()),"{mode:?}: native body 1 kind {kind}");
        }
        assert_ne!(native[4+2].2,final_scales[1][2].to_bits(),"{mode:?}: the control's native and final force scales differ");
    }
}

/// The stage-2 boundary member: D = 0.2 m, t = 0.01 m, a 3.5 N/m global-y UDL on a
/// simply supported 2 m member, whose represented Z is the non-point hull(Ẑ, Î/c)
/// (relative width 98.41 × 2^-64).
fn w1_represented_hull_member_request()->Value {
    let mut raw=w1_interior_member_request(false);
    raw["model"]["pipe_segments"][0]["section"]["outside_diameter"]["value"]=json!(0.2);
    raw["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"]=json!(0.01);
    raw["model"]["load_cases"][0]["primitive_loads"][0]["magnitude"]["value"]=json!(3.5);
    raw
}
/// The same member appended beside n05's independent sensitive member, which supplies
/// a genuine W1 trigger (as the interior publication controls).
pub(super) fn w1_represented_hull_publication_request()->Value {
    w1_loaded_publication_request(0.2,0.01)
}
/// The appended simply supported 2 m member on the section (D, t), 3.5 N/m global-y.
fn w1_loaded_publication_request(d:f64,t:f64)->Value {
    let mut raw=w1_interior_publication_request(false,-3.0);
    let pipe=raw["model"]["pipe_segments"].as_array_mut().unwrap().iter_mut().find(|p|p["id"]=="ss:pipe").unwrap();
    pipe["section"]["outside_diameter"]["value"]=json!(d);
    pipe["section"]["wall_thickness"]["value"]=json!(t);
    let load=raw["model"]["load_cases"][0]["primitive_loads"].as_array_mut().unwrap().iter_mut()
        .find(|l|l["id"]=="ss:member:transverse").unwrap();
    load["magnitude"]["value"]=json!(3.5);
    raw
}

/// W1 coverage repair §1.2 and §5 (corner 1). The stage-2 boundary member's represented
/// Z is a non-point hull, so the admitted lane's search is driven at z_lo: it converges
/// under the unchanged stop rule and limits, and the unchanged gate decides the case.
/// The expected outcome is the gate's own, derived here from its verdicts in each mode
/// (the census, §3.5, found it passing). Formerly
/// `w1_loaded_maximum_refuses_irreducible_represented_section_width`, which pinned the
/// search's exhaustion on this member: the cause it recorded, a convergence rule applied
/// to the input hull's own width, is repaired, and no gate, tolerance or limit changed.
/// Maintained writer of the corpus bases `w1_member_hull_d02_{mode}`: with
/// `W1_MEMBER_HULL_OUT` naming a folder, each mode's `{id, source, invocation,
/// generation_profile}` is written there after the public Direct entry's relation holds.
#[test]
fn w1_represented_z_hull_converges_on_ordinary_sections() {
    let raw=w1_represented_hull_member_request();
    for mode in MODES {
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut prepared=rp::PreparedCase::prepare_observed(request,mode,&capture)
            .unwrap_or_else(|f|panic!("{:?}",f.capture.error));
        prepared.solve_native().unwrap();
        // The represented Z is not a point: Ẑ·c − Î ≠ 0 exactly (one rounding), so the
        // exact Î/c differs from Ẑ.
        let f=&prepared.capture().facts[0];
        assert_ne!(f.section_modulus.mul_add(0.5*f.diameter,-f.second_moment),0.0,"{mode:?}: the represented-Z hull is not a point");
        let (certificate,passed)=match prepared.project_candidate() {
            Ok(candidate)=>{
                let c=candidate.certificate();
                (c.verdicts().to_vec(),c.passed())
            }
            Err(failure)=>{
                if let rp::PreparedCandidateError::Proof(proof)=&failure.error {
                    let mut copies=k::TraceCopyWork::default();
                    assert!(!matches!(proof.failure().typed_cause(&mut copies),k::ProductFailureView::Association("loaded maximum search exhausted")),
                        "{mode:?}: the represented-Z hull still exhausts the search");
                    assert!(proof.work().loaded_maximum_diagnostic().is_none(),"{mode:?}");
                }
                let c=failure.certificate().unwrap_or_else(|| panic!("{mode:?}: refused before the gate: {}",
                    format!("{:?}",failure.error).split(", work:").next().unwrap()));
                (c.verdicts().to_vec(),c.passed())
            }
        };
        // The gate decides: the case's outcome is exactly its verdicts' conjunction.
        assert!(!certificate.is_empty(),"{mode:?}: the gate judged the rows");
        assert_eq!(passed,certificate.iter().all(|v|v.passed),"{mode:?}: the outcome is the verdicts'");
        assert!(passed,"{mode:?}: the unchanged gate passes every row (census §3.5): {:?}",
            certificate.iter().filter(|v|!v.passed).map(|v|(v.row,v.failed)).collect::<Vec<_>>());
    }
    // The same member published beside an independent trigger: selected exactly when the
    // gate passes, read by the full reader, and the public Direct entry's relation holds.
    let raw=w1_represented_hull_publication_request();
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let bytes=serde_json::to_vec(&ordinary).unwrap();
        let ((envelope,retained),_,candidate)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        let successor=retained.unwrap_or_else(|f|panic!("{mode:?}: {f:?}")).value().clone();
        assert_eq!(candidate.as_ref(),Some(&successor));
        assert!(serde_json::to_vec(&envelope).unwrap()==bytes,"the ordinary owner is untouched");
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound&&verdict.numerical_eligible);
        let case=&successor["retained_precision"]["body"]["cases"][0];
        assert_eq!(case["status"],"selected");
        let attempt=&successor["retained_precision"]["body"]["product_attempts"][0];
        assert_eq!(attempt["proof"]["checks"]["certificate"]["kind"],"passed");
        assert!(attempt["proof"].get("loaded_maximum_diagnostic").is_none_or(Value::is_null));
        let maximum=successor["results"].as_array().unwrap().iter()
            .find(|r|r["kind"]=="pipe_elastic_normal_stress_maximum_v2"&&r["entity_ref"]=="ss:pipe").unwrap();
        assert!(maximum["value"].as_f64().unwrap()>0.0);
        let direct_raw=raw.clone();
        let (direct,_,precommit)=hooks::counted_with_successor(||
            run_linear_static_preview_value_with_retained_direct(direct_raw,mode,RETAINED_LIMIT).unwrap());
        if registered() {
            assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Registered);
            assert_eq!(precommit,Some(successor.clone()));
            assert_eq!(published(direct),serde_json::to_vec(&successor).unwrap());
        } else {
            assert_eq!(direct.admission().unwrap().profile,ProfileStatus::Stale);
            assert!(precommit.is_none()&&direct.successor().is_none());
            assert_eq!(published(direct),bytes);
        }
        if let Some(dir)=std::env::var_os("W1_MEMBER_HULL_OUT") {
            let id=format!("w1_member_hull_d02_{}",mode.as_str());
            std::fs::write(std::path::Path::new(&dir).join(format!("{id}.json")),
                serde_json::to_string_pretty(&json!({"id":id,"source":successor,"invocation":invocation,
                    "generation_profile":if registered(){"registered_direct"}else{"stale_private_driver"}})).unwrap()+"\n").unwrap();
        }
    }
}

#[path = "../../reporting/result_export/tests/support/retained_precision_corpus.rs"]
mod storage;
fn corpus_member_cases(prefixes:&[&str])->Vec<Value> {
    let corpus=storage::expand(serde_json::from_str(include_str!("../../../fixtures/results/retained_precision_cases.json")).unwrap());
    corpus["cases"].as_array().unwrap().iter()
        .filter(|c|prefixes.iter().any(|p|c["id"].as_str().unwrap().starts_with(p))).cloned().collect()
}
/// The committed producer's successor for a corpus invocation, through the private
/// retained_w1 body its writers use (`hook` arms a writer's fault hook first).
fn corpus_successor(case:&Value,hook:&dyn Fn())->Value {
    let raw=&case["invocation"]["request"];
    let mode=MODES.into_iter().find(|m|m.as_str()==case["invocation"]["solver_mode"]).unwrap();
    let (capture,observer,mut ordinary)=early_observed(raw,mode,true);
    ordinary.diagnostics.shrink_to_fit();
    hook();
    let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
    retained.map(|s|s.value().clone()).ok().or(precommit).unwrap_or_else(|| panic!("{}: no successor",case["id"]))
}

/// W1 coverage repair §1.2 and §5: a represented-Z hull that is a point (D/2 a power of
/// two) takes exactly today's search, with the same operations, work and bytes. The four
/// committed `w1_member_interior_*` records, and the two `w1_member_halfway_*` records on
/// the same point-hull section, are reproduced byte for byte, work counters included.
#[test]
fn w1_point_hull_search_is_unchanged() {
    let cases=corpus_member_cases(&["w1_member_interior_","w1_member_halfway_"]);
    assert_eq!(cases.len(),6);
    for case in cases {
        let id=case["id"].as_str().unwrap();
        let current=corpus_successor(&case,&||{});
        assert!(serde_json::to_vec(&current).unwrap()==serde_json::to_vec(&case["source"]).unwrap(),
            "{id}: the point-hull record moved");
    }
}

/// Late re-observation binds the member sidecar's actual authored occurrence,
/// including bits, target and count, before any preparation or publication.
#[test]
fn w1_member_load_late_hook_reobserves_exact_occurrences() {
    let mut raw = two_torque_cases([None, None]);
    raw["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({"id": "case-b:udl", "category": "distributed_force",
        "target": {"type": "element", "pipe": "pipe"}, "direction": "global_y", "magnitude": {"value": 10.0, "unit": "N/m"},
        "dimension": "force_per_length", "provenance": INVENTED}));
    let mut without = raw.clone();
    without["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().pop();
    let mut changed=raw.clone();
    changed["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().last_mut().unwrap()["magnitude"]["value"]=json!(11.0);
    for mode in MODES {
        let (_, mut observer, _) = early_observed(&raw, mode, true);
        assert!(observer.member_loads_basis_at(1).0);
        assert!(observer.test_recheck_occurrences(&raw,1).is_ok());
        assert!(observer.test_recheck_occurrences(&without,1).is_err());
        assert_eq!(observer.test_recheck_occurrences(&changed,1).map_err(|e|e.to_string()),Err("early member occurrence identity/bits".to_owned()));
        observer.member_sources.as_mut().unwrap().clear();
        assert_eq!(observer.test_recheck_occurrences(&raw,1).map_err(|e|e.to_string()),Err("early member-load case coverage".to_owned()));
    }
}

/// Maintained real product writer: capture -> preparation -> native solve ->
/// corrected recovery/certificate -> serialized successor -> full native reader.
/// Optional output is evidence for the Python and desktop wasm consumers.
#[test]
fn w1_member_load_product_consuming_successor() {
    use crate::retained_tests_hooks as hooks;
    let mut raw=two_torque_cases([None,Some("P1")]);
    for (case,axis,value) in [(0,"global_x",10.0),(1,"global_x",-7.0)] {
        let id=raw["model"]["load_cases"][case]["id"].as_str().unwrap().to_owned();
        raw["model"]["load_cases"][case]["primitive_loads"].as_array_mut().unwrap().push(json!({
            "id":format!("{id}:udl"),"category":"distributed_force","target":{"type":"element","pipe":"pipe"},
            "direction":axis,"magnitude":{"value":value,"unit":"N/m"},"dimension":"force_per_length","provenance":INVENTED}));
    }
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        if let (Some(dir),Some(candidate))=(std::env::var_os("I61_U1_OUT"),precommit.as_ref()) {
            let name=format!("w1_member_load_{}",mode.as_str());
            let document=json!({"id":name,"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()}});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
        let successor=retained.unwrap_or_else(|fallback|panic!("{mode:?}: {fallback:?}")).value().clone();
        assert_eq!(precommit.as_ref(),Some(&successor));
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound && verdict.numerical_eligible);
        let b=&successor["retained_precision"]["body"];
        for case in 0..2 {
            assert_eq!(b["cases"][case]["status"],"selected");
            let source=b["sources"].as_array().unwrap().iter().find(|s|s["owner"]["case_index"]==case).unwrap();
            assert_eq!(source["member_load_terms"].as_array().unwrap().len(),1);
        }
        assert_eq!(successor["diagnostics"].as_array().unwrap().iter().filter(|d|d["code"]==super::retained_wire::SELECTED_CODE
            && d["message"]==super::retained_wire::SELECTED_MEMBER_LOAD_MESSAGE).count(),2);
        if let Some(dir)=std::env::var_os("I61_U1_OUT") {
            let name=format!("w1_member_load_{}",mode.as_str());
            let document=json!({"id":name,"source":successor,"invocation":invocation});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
    }
}

/// The genuine-exhaustion control, hook-driven (W1 coverage repair §1.4, option 2). After
/// the corner-1 repair no committed or census input exhausts the loaded-maximum search at
/// its production limits (census §3.5), so a real member, T18's transverse `case-b` on
/// the D = 0.2 m section, is driven to exhaustion by the kernel's capacity hook at a
/// reduced split limit (0, with the production 256 live nodes and depth 64; the member's
/// driven search needs one split). Its exact diagnostic and all spent arithmetic survive
/// beside a successfully retained nodal case, without retained publication for the
/// failure. The output claims no production limit.
///
/// Compiled only under the opt-in `loaded-maximum-hooks` feature, which no
/// `[dev-dependencies]` entry enables; its maintained command passes it explicitly:
///
/// ```text
/// cargo test --offline --features loaded-maximum-hooks --lib \
///     w1_member_load_exhaustion_retains_typed_interval_work
/// ```
///
/// With `I61_U1_OUT` naming a folder, it writes `w1_member_exhaustion_{mode}.json`.
#[cfg(feature = "loaded-maximum-hooks")]
#[test]
fn w1_member_load_exhaustion_retains_typed_interval_work() {
    use crate::retained_tests_hooks as hooks;
    use open_pipe_stress_frame_kernel::structural::retained_api::loaded_maximum_hooks as capacity;
    const REDUCED:(usize,u8,usize)=W1_EXHAUSTION_CONTROL_LIMITS;
    let raw=n05_request(vec![n05_torques(),n05_tip_noise("case-b")]);
    for mode in MODES {
        // Without the hook the same member converges and its case is selected.
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let (_,unhooked)=retained_w1(observer,ordinary,&capture);
        let unhooked=unhooked.unwrap_or_else(|f|panic!("{mode:?}: {f:?}"));
        assert_eq!(unhooked.value()["retained_precision"]["body"]["cases"][1]["status"],"selected","{mode:?}: production limits converge");
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let ((_,retained),_,precommit)=capacity::with(REDUCED,||hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture)));
        if let (Some(dir),Some(candidate))=(std::env::var_os("I61_U1_OUT"),precommit.as_ref()) {
            let name=format!("w1_member_exhaustion_{}",mode.as_str());
            let document=json!({"id":name,"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()}});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
        let successor=retained.unwrap_or_else(|fallback|panic!("{mode:?}: {fallback:?}")).value().clone();
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap_or_else(|e|panic!("{mode:?}: {e:?}"));
        assert!(verdict.invocation_bound);
        assert!(!verdict.numerical_eligible,"the unavailable case keeps invocation-wide eligibility withheld");
        let b=&successor["retained_precision"]["body"];
        assert_eq!(b["cases"][0]["status"],"selected");
        assert_eq!(b["cases"][1]["status"],"unavailable");
        let attempt=&b["product_attempts"][1];
        assert_eq!(attempt["stages"]["projection"],"failed");
        for stage in ["maxima","values","aliases","certificate","observables","g5a"] {
            assert_eq!(attempt["stages"][stage],"not_entered");
        }
        assert_eq!(attempt["result"]["error"],json!({"kind":"proof",
            "cause":{"kind":"association","detail":"loaded maximum search exhausted"}}),"{mode:?}");
        assert!(attempt["proof"]["loaded_maximum_diagnostic"].is_object());
        assert_eq!(attempt["proof"]["numeric"]["wide_lme"]["kind"],"exact");
        assert!(attempt["proof"]["numeric"]["wide_lme"]["value"].as_u64().unwrap()>0);
        let unavailable_rows:Vec<_>=successor["results"].as_array().unwrap().iter()
            .filter(|r|r["basis_ref"]["ref_type"]=="load_case" && r["basis_ref"]["ref_id"]=="case-b").collect();
        assert!(!unavailable_rows.is_empty());
        assert!(unavailable_rows.iter().all(|r|r.get("recovery_method").is_none()));
    }
}

/// A member-aware values failure transfers its numeric work into the proof
/// exactly once. A second, unloaded member gives the existing completion fault
/// a real maximum to omit; the adjacent nodal case remains selectable.
#[test]
fn w1_member_load_values_failure_keeps_merged_work() {
    use crate::retained_tests_hooks as hooks;
    let mut raw=two_torque_cases([None,None]);
    for (id,x) in [("aux-root",3.0),("aux-tip",4.0)] {
        let mut node=raw["model"]["nodes"][1].clone();node["id"]=json!(id);node["position"]["x"]=json!(x);
        raw["model"]["nodes"].as_array_mut().unwrap().push(node);
        raw["model"]["supports"].as_array_mut().unwrap().push(json!({"id":format!("fixed:{id}"),"node":id,
            "family":"anchor","restraints":["UX","UY","UZ","RX","RY","RZ"],"provenance":INVENTED}));
    }
    let mut auxiliary=raw["model"]["pipe_segments"][0].clone();
    auxiliary["id"]=json!("auxiliary");auxiliary["from"]=json!("aux-root");auxiliary["to"]=json!("aux-tip");
    raw["model"]["pipe_segments"].as_array_mut().unwrap().push(auxiliary);
    raw["model"]["load_cases"][1]["primitive_loads"].as_array_mut().unwrap().push(json!({
        "id":"case-b:udl","category":"distributed_force","target":{"type":"element","pipe":"pipe"},
        "direction":"global_x","magnitude":{"value":10.0,"unit":"N/m"},"dimension":"force_per_length","provenance":INVENTED}));
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        hooks::fault_case(1,crate::retained_receipt::TraceFault::ValuesCompletion);
        let ((_,retained),_,precommit)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        if let (Some(dir),Some(candidate))=(std::env::var_os("I61_U1_OUT"),precommit.as_ref()) {
            let name=format!("w1_member_values_failure_{}",mode.as_str());
            let document=json!({"id":name,"source":candidate,"invocation":{"request":raw,"solver_mode":mode.as_str()}});
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.json")),serde_json::to_string_pretty(&document).unwrap()+"\n").unwrap();
        }
        let successor=retained.unwrap_or_else(|fallback|panic!("{mode:?}: {fallback:?}")).value().clone();
        let invocation=json!({"request":raw,"solver_mode":mode.as_str()});
        let verdict=reader::validate(&successor,Some(&invocation)).unwrap();
        assert!(verdict.invocation_bound && !verdict.numerical_eligible);
        let b=&successor["retained_precision"]["body"];
        assert_eq!(b["cases"][0]["status"],"selected");
        assert_eq!(b["cases"][1]["status"],"unavailable");
        let attempt=&b["product_attempts"][1];
        assert_eq!(attempt["stages"]["values"],"failed");
        assert_eq!(attempt["result"]["error"]["kind"],"values");
        assert_eq!(attempt["proof"]["completion"]["kind"],"merged");
        assert_eq!(attempt["stages"]["aliases"],"not_entered");
    }
}

/// A successor with the proof's metered work and the receipt hash binding it erased: each
/// product attempt's `proof.numeric` entry counts, `wide_lme` and `exact_sum_lme` values,
/// and `receipt_sha256`. Its JCS SHA-256.
fn work_masked_sha256(source:&Value)->String {
    let mut v=source.clone();
    v["retained_precision"]["receipt_sha256"]=json!("");
    for attempt in v["retained_precision"]["body"]["product_attempts"].as_array_mut().unwrap() {
        let Some(numeric)=attempt.get_mut("proof").and_then(|p|p.get_mut("numeric")).filter(|n|n.is_object()) else {continue};
        for key in ["wide_lme","exact_sum_lme"] {numeric[key]["value"]=json!(0);}
        for entry in numeric["entries"].as_array_mut().unwrap() {entry["value"]=json!(0);}
    }
    sha(open_pipe_stress_canonical_json::canonical_json(&v).as_bytes())
}
/// The committed documents with their work erased, as `work_masked_sha256` forms them. The two
/// `w1_member_load_*` pins are the pre-repair documents' (main 257b44b910's corpus, unchanged
/// through the corner-2 head). The six `w1_member_values_failure_*`, `w1_member_multi_*` and
/// `w1_member_combination_*` pins are those pre-repair documents plus W-8's one inserted
/// `MODEL_TOPOLOGY_DISCONNECTED` warning and the hashes binding it (W-8 topology warning added;
/// no number moved), so the work-only rule below still holds against them. They bind model 0.6.0
/// requests; see `as_pre_repair_invocation`. W-7 (DEL-04-01 `global-end-actions.md`; DEL-04-08 §8.20) adds
/// each member's twelve not-covered global end rows per case to all eight documents, with the figures
/// they move (the proof's row capacity, prepared capacity bytes and visits, the adapter counters, the
/// later rows' projection positions) and the hashes binding them; no pre-W-7 row, class or status moved.
const PRE_REPAIR_WORK_MASKED:[(&str,&str);8]=[
    ("w1_member_load_sparse_interactive","9fe8f34a1a092973bdb482f2ba68ce13faca88908eed5256d7f6942c9d8ed67f"),
    ("w1_member_load_dense_scrutiny","e3a876d7443f5d537bb8484c4ec7b884095837f2fdb36ed446c3bb21130789ad"),
    ("w1_member_values_failure_sparse_interactive","dbb52d431f2aa98515cb0c3e946c5bd00167fb0cd9a9ed5b4371a79b59fffc51"),
    ("w1_member_values_failure_dense_scrutiny","e8e0a0ad17ea8195c4455b68a9397765d20d2ac50848aedfdfb6a538d5d15d6e"),
    ("w1_member_multi_sparse_interactive","ac429de965114d1bcac0e2c89fa75ddb1f72795df17d315b7c56e12992d0ed44"),
    ("w1_member_multi_dense_scrutiny","616d2aa5e448e6d0309a1d4c76ccda8095ad4140a7a74199a27ea377425db442"),
    ("w1_member_combination_sparse_interactive","d049d9643d2fbd33b30b84dc3d52cc09d3445e26e9d04cd5fdce6aaeaace35a3"),
    ("w1_member_combination_dense_scrutiny","e53469fbb12ce5d6ecf51177169fbbd69c56c2fb6a6e87977cddda2e6d43c67b"),
];
/// W1 coverage repair §1.4 and §5: the committed loaded members on the D = 0.2 m
/// interval-hull section that converged before the repair (axial loads, so the bending
/// lower bound is zero at every evaluated point) take the driven path, whose hull-valid
/// evaluations are metered. On the regenerated `w1_member_load_*`, `w1_member_multi_*`,
/// `w1_member_combination_*` and (same pipe) `w1_member_values_failure_*` entries only the
/// proof's work counters and the receipt hash binding them move: every other byte (every
/// number, class, selection, maximum value and status) equals the pre-repair document's
/// (with W-8's warning, where it applies).
/// The live producer reproduces each committed entry byte for byte.
/// The model version the pre-repair documents' requests were authored at. Model 0.7.0
/// (DEL-02-01 Design `model-document-0.7.0.md` §2; W-13 S1b) re-authored the corpus
/// requests by their version token alone, which moves only the successor's invocation
/// digest. The pins stay the pre-repair documents': the successor is compared with its
/// invocation digest restored to that of the same request at the pre-repair version, so
/// any other byte change still fails.
const PRE_REPAIR_MODEL_VERSION: &str = "0.6.0";
fn as_pre_repair_invocation(source:&Value,invocation:&Value)->Value {
    let mut raw=invocation["request"].clone();
    assert_eq!(raw["model"]["schema_version"],json!(crate::MODEL_SCHEMA_VERSION),"the current model version");
    raw["model"]["schema_version"]=json!(PRE_REPAIR_MODEL_VERSION);
    let mode=MODES.into_iter().find(|m|m.as_str()==invocation["solver_mode"]).unwrap();
    let (_,capture)=source_receipt::CapturedInvocation::parse(raw,mode).unwrap();
    let mut v=source.clone();
    v["retained_precision"]["body"]["invocation"]["value"]=json!(capture.borrowed_digest());
    v
}

#[test]
fn w1_hull_repair_moves_only_work_on_converging_members() {
    let cases=corpus_member_cases(&["w1_member_load_","w1_member_values_failure_","w1_member_multi_","w1_member_combination_"]);
    assert_eq!(cases.len(),PRE_REPAIR_WORK_MASKED.len());
    for (id,pin) in PRE_REPAIR_WORK_MASKED {
        let case=cases.iter().find(|c|c["id"]==id).unwrap_or_else(|| panic!("{id}"));
        let values_fault=id.starts_with("w1_member_values_failure_");
        let current=corpus_successor(case,&|| if values_fault {
            hooks::fault_case(1,crate::retained_receipt::TraceFault::ValuesCompletion);
        });
        println!("W1_HULL_WORK_MASKED {id} committed={} live={}",work_masked_sha256(&case["source"]),work_masked_sha256(&current));
        assert_eq!(work_masked_sha256(&as_pre_repair_invocation(&current,&case["invocation"])),pin,"{id}: a byte other than work moved");
        assert!(serde_json::to_vec(&current).unwrap()==serde_json::to_vec(&case["source"]).unwrap(),"{id}: the committed entry is the live producer's");
    }
}

/// REQ-006 (f) (the owner's ruling of 2026-10-20 on the W1 coverage repair): a loaded
/// member's maximum whose certified hull is too wide for the unchanged gate keeps its
/// ordinary result, and the case is unavailable with its named cause. The appended simply
/// supported member on the 0.12 m × 0.01 m section (census §3.5, C1-b: a *Z-hull only*
/// failure) converges at the represented hull's binding endpoint, but its maximum row's
/// enclosure fails the unchanged SharperExact predicate in both modes (beside the inherited
/// two-readout corner on its midspan bending stress). No search or gate refusal is
/// relaxed: the case is refused by the gate, and the public entry publishes the ordinary
/// bytes with the one notice.
#[test]
fn w1_loaded_maximum_too_wide_for_the_gate_keeps_the_ordinary_result() {
    let raw=w1_loaded_publication_request(0.12,0.01);
    for mode in MODES {
        let (request,capture)=source_receipt::CapturedInvocation::parse(raw.clone(),mode).unwrap();
        let mut prepared=rp::PreparedCase::prepare_observed(request,mode,&capture)
            .unwrap_or_else(|f|panic!("{:?}",f.capture.error));
        prepared.solve_native().unwrap();
        let Err(failure)=prepared.project_candidate() else {panic!("{mode:?}: the gate passed")};
        let rp::PreparedCandidateError::Proof(proof)=&failure.error else {panic!("{mode:?}: not a gate refusal")};
        let mut copies=k::TraceCopyWork::default();
        assert!(matches!(proof.failure().typed_cause(&mut copies),
            k::ProductFailureView::Predicate{predicate:k::ProductPredicate::SharperExact,..}),"{mode:?}: the gate's named cause");
        // The search converged (no exhaustion diagnostic); the gate judged the maximum row.
        assert!(proof.work().loaded_maximum_diagnostic().is_none(),"{mode:?}");
        let row=failure.ordinary.results.iter().position(|r|r.kind=="pipe_elastic_normal_stress_maximum_v2"&&r.entity_ref=="ss:pipe").unwrap();
        let verdict=proof.work().verdicts().iter().find(|v|v.row==row).unwrap_or_else(|| panic!("{mode:?}: the maximum was judged"));
        assert!(!verdict.passed&&verdict.failed==Some(k::ProductPredicate::SharperExact),"{mode:?}: {verdict:?}");
    }
    for mode in MODES {
        let (capture,observer,mut ordinary)=early_observed(&raw,mode,true);
        ordinary.diagnostics.shrink_to_fit();
        let ((_,retained),_,candidate)=hooks::counted_with_successor(||retained_w1(observer,ordinary,&capture));
        assert!(matches!(retained,Err(W1Fallback::Candidate)),"{mode:?}: {retained:?}");
        assert!(candidate.is_none(),"{mode:?}: nothing reaches precommit");
        let plain=plain(mode,&raw);
        let (output,_)=direct(&raw,mode);
        if registered() {
            assert!(matches!(output.retained().and_then(|r|r.as_ref().err()),Some(W1Fallback::Candidate)),"{mode:?}");
            assert!(output.successor().is_none());
            let bytes=published(output);
            assert_eq!(notices(&bytes),1,"{mode:?}: exactly one notice");
            assert_eq!(bytes,with_notice(&plain,"case",None),"{mode:?}: the ordinary bytes, then the notice");
        } else {
            assert!(output.retained().is_none());
            assert_eq!(published(output),plain,"{mode:?}");
        }
    }
}
