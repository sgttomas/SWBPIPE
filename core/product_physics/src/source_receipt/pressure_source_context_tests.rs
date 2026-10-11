//! J1 preparation: the captured source context and its publication check.
//! Full v3 publication/readback is exercised after shared caller/composite fan-in.
use super::*;

fn request(load_state: bool, v3: bool) -> Value {
    let mut raw: Value = serde_json::from_str(if load_state {
        include_str!("../../../../fixtures/product_preview/load_reference_source/n05.request.json")
    } else {
        include_str!("../../../../fixtures/product_preview/physics_source/n05.request.json")
    })
    .unwrap();
    if v3 {
        raw["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"});
    }
    raw
}

#[test]
fn captured_composite_context_keeps_v2_names_and_unifies_v3_model_versions() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for (load_state, v3, id, policy, domain, profile) in [
            (
                false,
                false,
                COMPOSITE_SEMANTIC_ID,
                "PHYSICS-SOURCE-1",
                "physics_source_case_evidence_v1",
                "exact_straight_pressure_v2",
            ),
            (
                true,
                false,
                LOAD_REFERENCE_SOURCE_SEMANTIC_ID,
                "LOAD-REFERENCE-SOURCE-1",
                "load_reference_source_case_evidence_v1",
                "resolved_straight_load_state_source_v1",
            ),
            (
                false,
                true,
                "openpipestress.result_semantics/0.3.0/pressure-source-1",
                "PRESSURE-SOURCE-1",
                "pressure_source_case_evidence_v1",
                "exact_pressure_v3",
            ),
            (
                true,
                true,
                "openpipestress.result_semantics/0.3.0/pressure-source-1",
                "PRESSURE-SOURCE-1",
                "pressure_source_case_evidence_v1",
                "exact_pressure_v3",
            ),
        ] {
            let (parsed, captured) =
                CapturedInvocation::parse(request(load_state, v3), mode).unwrap();
            let context = captured.composite_context().unwrap();
            assert_eq!(
                context,
                CompositeSourceContext::for_model(&parsed.model).unwrap()
            );
            assert_eq!(
                (
                    context.semantic_id(),
                    context.policy_name(),
                    context.case_evidence_domain(),
                    context.profile_id()
                ),
                (id, policy, domain, profile)
            );
            assert_eq!(
                context.profile_mode(),
                if v3 {
                    "exact_pressure_v3"
                } else {
                    "exact_straight_pressure_v2"
                }
            );
        }
    }
}

#[test]
fn publication_label_is_checked_against_the_captured_policy() {
    let v2 = CapturedInvocation::parse(request(false, false), PreviewSolverMode::SparseInteractive)
        .unwrap()
        .1;
    let v3 = CapturedInvocation::parse(request(false, true), PreviewSolverMode::SparseInteractive)
        .unwrap()
        .1;
    for capture in [&v2, &v3] {
        let context = capture.composite_context().unwrap();
        let good = json!({"producer":{"semantic_contract_id":context.semantic_id()}});
        assert_eq!(
            validate_publication_identity(&good, context.receipt_policy()),
            Ok(())
        );
        let other = if context == CompositeSourceContext::PressureSource {
            COMPOSITE_SEMANTIC_ID
        } else {
            exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID
        };
        let bad_label = json!({"producer":{"semantic_contract_id":other}});
        assert_eq!(
            validate_publication_identity(&bad_label, context.receipt_policy()),
            Err(bad(
                "publication must have source identity and omit receipt"
            ))
        );
        let with_receipt = json!({"producer":{"semantic_contract_id":context.semantic_id()},"source_block_recovery":null});
        assert!(validate_publication_identity(&with_receipt, context.receipt_policy()).is_err());
    }
}

#[test]
fn captured_context_does_not_infer_a_contract_from_a_publication_claim() {
    let mut raw = request(false, true);
    raw["producer"] = json!({"semantic_contract_id":COMPOSITE_SEMANTIC_ID});
    let (_, captured) = CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(
        captured.composite_context().unwrap(),
        CompositeSourceContext::PressureSource
    );
    for contract in [
        Value::Null,
        json!({"version":"3.0.0","mode":"exact_straight_pressure_v2"}),
        json!({"version":"2.0.0","mode":"exact_pressure_v3"}),
    ] {
        let mut raw = request(false, true);
        raw["model"]["pressure_contract"] = contract;
        let (_, captured) =
            CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(
            captured.composite_context(),
            Err(bad("composite source requires an actual exact contract"))
        );
    }
    let mut preview = request(false, false);
    preview["model"]["schema_version"] = json!("0.1.0");
    preview["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    let (_, captured) =
        CapturedInvocation::parse(preview, PreviewSolverMode::DenseScrutiny).unwrap();
    assert!(captured.composite_context().is_err());
    // The ordinary preview receipt remains its separate policy.
    assert_eq!(ReceiptPolicy::SourceBlocks.semantic_id(), SEMANTIC_ID);
}

#[test]
fn pressure_case_context_uses_one_domain_without_changing_its_payload_shape() {
    for load_state in [false, true] {
        let (_, captured) = CapturedInvocation::parse(
            request(load_state, true),
            PreviewSolverMode::SparseInteractive,
        )
        .unwrap();
        let context = captured.composite_context().unwrap();
        let proof = if load_state {
            json!({"exact_case":{"profile_mode":"exact_pressure_v3"},"pressure":[],"load_reference_state":{"load_case_id":"case"}})
        } else {
            json!({"exact_case":{"profile_mode":"exact_pressure_v3"},"pressure":[]})
        };
        let independently_named =
            checked(&json!({"domain":"pressure_source_case_evidence_v1","payload":proof})).unwrap();
        assert_eq!(
            hash(context.case_evidence_domain(), &proof).unwrap(),
            format!("{:x}", Sha256::digest(independently_named.as_bytes()))
        );
        assert_ne!(
            hash(context.case_evidence_domain(), &proof).unwrap(),
            hash("physics_source_case_evidence_v1", &proof).unwrap()
        );
    }
}

#[test]
fn formulation_helpers_preserve_held_callers_and_name_the_actual_publication() {
    for (text, connectors) in [
        (
            include_str!("../../../../fixtures/results/pressure_v3_straight_reader_corpus.json"),
            false,
        ),
        (
            include_str!("../../../../fixtures/results/pressure_v3_connector_reader_corpus.json"),
            true,
        ),
    ] {
        let corpus: Value = serde_json::from_str(text).unwrap();
        for case in corpus["cases"].as_array().unwrap() {
            let load_state = case["document_schema_version"] == "0.4.0";
            let held = exact_admission::pressure_v3_formulation_basis(load_state, connectors);
            assert_eq!(
                serialized(&held).unwrap(),
                case["envelope"]["formulation_basis"]
            );
            for selected in [false, true] {
                let joined = exact_admission::pressure_v3_formulation_basis_with_source(
                    load_state, connectors, selected,
                );
                assert_eq!(joined.profile_id, held.profile_id);
                assert_eq!(joined.limitations.len(), held.limitations.len());
                assert_eq!(&joined.limitations[1..7], &held.limitations[1..7]);
                assert_eq!(&joined.limitations[8..], &held.limitations[8..]);
                assert!(!joined.limitations[7].contains("not joined"));
                if selected {
                    assert!(joined.limitations[0].contains("under pressure-source-1 semantics"));
                    assert!(joined.limitations[7].contains("selected retained-source response"));
                } else {
                    assert_eq!(joined.limitations[0], held.limitations[0]);
                    assert!(
                        joined.limitations[7].contains("publishes ordinary structural recovery")
                    );
                }
            }
        }
    }
}


// J7: only an explicitly armed private unit can take these owned inputs. The
// intercepted run is discarded; it is not a successful producer publication.
// Unarmed tests and production continue their original finalizer exactly once.
#[derive(Default)]
struct PrivateCapture {
    armed: bool,
    reservations: Vec<ObservedReservation>,
    snapshot: Option<FinalizerSnapshot>,
}
thread_local! {
    static PRIVATE_CAPTURE: std::cell::RefCell<PrivateCapture> = std::cell::RefCell::new(PrivateCapture::default());
}
struct CaptureArm;
impl CaptureArm {
    fn new() -> Self {
        PRIVATE_CAPTURE.with(|state| {
            let mut state = state.borrow_mut();
            assert!(!state.armed && state.snapshot.is_none());
            state.armed = true;
            state.reservations.clear();
        });
        composite::start_trace();
        Self
    }
    fn take(&self) -> FinalizerSnapshot {
        PRIVATE_CAPTURE.with(|state| state.borrow_mut().snapshot.take()).expect("selected fixture reached composite finalizer")
    }
}
impl Drop for CaptureArm {
    fn drop(&mut self) {
        PRIVATE_CAPTURE.with(|state| *state.borrow_mut() = PrivateCapture::default());
        let _ = composite::take_trace();
    }
}
struct FinalizerSnapshot {
    raw: Value,
    mode: PreviewSolverMode,
    envelope: MechanicsEnvelope,
    cases: Vec<FinalizedSourceBlockCase>,
    budget: SourceRecoveryBudget,
    reservations: Vec<ObservedReservation>,
    stages: Vec<Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct DescriptorInputs {
    case_id_utf8: usize,
    offset_product_lengths: Vec<usize>,
    term_product_lengths: Vec<Vec<usize>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RowInputs {
    identity_utf8: [usize; 4], // id, kind, entity, unit
    metadata_utf8: Option<[usize; 5]>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ReservationInputs {
    source_identity_utf8: usize,
    stiffness_dimension: usize,
    descriptors: Vec<DescriptorInputs>,
    rows: Vec<RowInputs>,
    support_actions: usize,
}
struct ObservedReservation {
    case_id: String,
    encoded_request: usize,
    inputs: ReservationInputs,
    actual_rows: Vec<ResultItem>,
    // Kept private: exact replay/functionals inputs, not a counter oracle or
    // serialized evidence. Equality establishes identical remaining work inputs.
    remaining_inputs: String,
}
fn encoded_request(raw: &Value) -> usize {
    let text = serde_json::to_string(raw).unwrap();
    open_pipe_stress_canonical_json::canonical_json_checked_v1_text(&text).unwrap().len()
}
fn reservation_inputs(selected: &SelectedSourceRecovery, actual: &[ResultItem]) -> ReservationInputs {
    let response = selected.retained().response();
    ReservationInputs {
        source_identity_utf8: response.source_identity().as_bytes().len(),
        stiffness_dimension: response.source_system().stiffness.len(),
        descriptors: selected.retained().descriptors().iter().map(|d| DescriptorInputs {
            case_id_utf8: d.key.case_id.as_bytes().len(),
            offset_product_lengths: d.offset.iter().map(Vec::len).collect(),
            term_product_lengths: d.terms.iter().map(|t| t.products.iter().map(Vec::len).collect()).collect(),
        }).collect(),
        rows: actual.iter().map(|row| RowInputs {
            identity_utf8: [row.id.as_bytes().len(),row.kind.as_bytes().len(),row.entity_ref.as_bytes().len(),row.unit.as_bytes().len()],
            metadata_utf8: row.metadata.as_ref().map(|m| [m.component.as_bytes().len(),m.coordinate_system.as_bytes().len(),m.location.as_bytes().len(),m.basis.as_bytes().len(),m.sign_convention.as_bytes().len()]),
        }).collect(),
        support_actions: selected.support_actions().len(),
    }
}
pub(super) fn observe_reservation_inputs(capture: &CapturedInvocation, selected: &SelectedSourceRecovery, actual: &[ResultItem]) {
    PRIVATE_CAPTURE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.armed { return; }
        let response = selected.retained().response();
        state.reservations.push(ObservedReservation {
            case_id: selected.retained().descriptors()[0].key.case_id.clone(),
            encoded_request: encoded_request(capture.borrowed_raw()),
            inputs: reservation_inputs(selected, actual),
            actual_rows: actual.to_vec(),
            remaining_inputs: format!("{:?}\n{:?}\n{:?}\n{}", response.source_system(),response.force_basis(),selected.retained().descriptors(),serde_json::to_string(actual).unwrap()),
        });
    });
}
pub(super) fn intercept_finalizer(capture: &CapturedInvocation, envelope: &MechanicsEnvelope,
    cases: &mut Vec<FinalizedSourceBlockCase>, budget: &mut SourceRecoveryBudget) -> bool {
    PRIVATE_CAPTURE.with(|state| {
        let mut state = state.borrow_mut();
        if !state.armed { return false; }
        state.armed = false;
        let reservations = std::mem::take(&mut state.reservations);
        state.snapshot = Some(FinalizerSnapshot {
            raw: capture.borrowed_raw().clone(), mode: capture.mode(), envelope: envelope.clone(),
            cases: std::mem::take(cases), budget: std::mem::take(budget), reservations,
            stages: composite::take_trace(),
        });
        true
    })
}
fn fixture_request(family: &str, name: &str, v3: bool) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../fixtures/product_preview/{family}/{name}.request.json"));
    let mut raw: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    if v3 { raw["model"]["pressure_contract"] = json!({"version":"3.0.0","mode":"exact_pressure_v3"}); }
    raw
}
fn snapshot(family: &str, name: &str, v3: bool, mode: PreviewSolverMode) -> FinalizerSnapshot {
    let raw = fixture_request(family,name,v3);
    let arm = CaptureArm::new();
    let _discarded = crate::run_linear_static_preview_value_with_mode(raw.clone(),mode);
    let snapshot = arm.take();
    assert_eq!(snapshot.raw,raw);
    assert!(!snapshot.reservations.is_empty());
    snapshot
}
// Wide integer evaluation of the declared complete expression, with one cap
// at the usize ceiling. Admitted shapes prove every interior sum is finite.
// It never calls the production reservation/counter routines as an oracle.
fn independent_finalization(e: usize, inputs: &ReservationInputs) -> usize {
    let mut inner = (inputs.source_identity_utf8 as u128).saturating_mul(12)
        .saturating_add((inputs.stiffness_dimension as u128).saturating_pow(2).saturating_mul(32))
        .saturating_add((e as u128).saturating_mul(12));
    for descriptor in &inputs.descriptors {
        inner = inner.saturating_add((descriptor.case_id_utf8 as u128).saturating_mul(12));
        for length in &descriptor.offset_product_lengths { inner = inner.saturating_add((*length as u128).saturating_mul(48)); }
        for term in &descriptor.term_product_lengths {
            inner = inner.saturating_add(48);
            for length in term { inner = inner.saturating_add((*length as u128).saturating_mul(48)); }
        }
    }
    for row in &inputs.rows {
        for length in row.identity_utf8 { inner = inner.saturating_add(length as u128); }
        if let Some(metadata) = row.metadata_utf8 { for length in metadata { inner = inner.saturating_add(length as u128); } }
    }
    inner.saturating_mul(2).min(usize::MAX as u128) as usize
}
fn independent_wrapper(e: usize, inputs: &ReservationInputs) -> usize {
    (1024u128 + 64u128 * inputs.rows.len() as u128 + independent_finalization(e,inputs) as u128
        + 4096u128 * inputs.support_actions as u128).min(usize::MAX as u128) as usize
}
fn stage_charged(stage: &Value) -> i128 { stage["charged"].as_u64().unwrap().into() }
fn check_snapshot_inputs(snapshot: &FinalizerSnapshot) {
    let (_, capture) = CapturedInvocation::parse(snapshot.raw.clone(),snapshot.mode).unwrap();
    let entries: Vec<_> = snapshot.stages.iter().filter(|stage| stage["stage"] == "source_case_finalization_entry").collect();
    let reserved: Vec<_> = snapshot.stages.iter().filter(|stage| stage["stage"] == "source_case_finalization_reserved").collect();
    assert_eq!(entries.len(),snapshot.reservations.len());
    assert_eq!(reserved.len(),snapshot.reservations.len());
    for (index, observed) in snapshot.reservations.iter().enumerate() {
        let case = snapshot.cases.iter().find(|case| case.case_id == observed.case_id).unwrap();
        let selected = case._selected.as_ref().expect("genuinely selected case retains its owner");
        assert_eq!(reservation_inputs(selected,&observed.actual_rows),observed.inputs);
        assert_eq!(case.actual_rows,serialized(&observed.actual_rows).unwrap(),"reservation rows and finalized rows must be identical");
        let published: Vec<_> = snapshot.envelope.results.iter().filter(|row| row.basis_ref.as_ref().is_some_and(|basis|basis.ref_type == "load_case" && basis.ref_id == case.case_id)).cloned().collect();
        assert_eq!(serialized(&published).unwrap(),case.actual_rows,"final envelope inventory is corroboration, not the input oracle");
        let expected = independent_finalization(observed.encoded_request,&observed.inputs);
        assert!(expected < usize::MAX && expected <= PHYSICS_SOURCE_WORK_LIMIT);
        // Actual-under-test only: expected was computed exclusively from the raw site inputs.
        assert_eq!(finalization_size(&capture,selected,&observed.actual_rows),expected);
        let wrapper = independent_wrapper(observed.encoded_request,&observed.inputs);
        assert_eq!(stage_charged(reserved[index])-stage_charged(entries[index]),wrapper as i128);
    }
}
fn export_raw_inputs(family: &str,name: &str,v3: bool,snapshot: &FinalizerSnapshot) {
    if let Some(directory) = std::env::var_os("SWBPIPE_PRESSURE_JOIN_RAW_INPUTS_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        let rows: Vec<_> = snapshot.reservations.iter().map(|input|json!({"case_id":input.case_id,"encoded_request_utf8":input.encoded_request,"inputs":input.inputs})).collect();
        let path = directory.join(format!("{family}-{name}-{}-{}.raw-inputs.json",snapshot.mode.as_str(),if v3 {"v3"} else {"v2"}));
        std::fs::write(path,serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}
#[test]
fn pressure_source_private_reservation_inputs_confirm_all_twins_complete_expression() {
    let families: [(&str,&[&str]);2] = [
        ("physics_source",&["n05","n06","n05_unicode","n05_units","fields","mixed","mixed_units"]),
        ("load_reference_source",&["n05","n06","eigen_motion","fields","mixed"]),
    ];
    let mut twins = 0;
    for (family,names) in families {
        for name in names {
            for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
                let v2 = snapshot(family,name,false,mode);
                let v3 = snapshot(family,name,true,mode);
                check_snapshot_inputs(&v2);check_snapshot_inputs(&v3);
                assert_eq!(v2.reservations.len(),v3.reservations.len());
                for (a,b) in v2.reservations.iter().zip(&v3.reservations) {
                    assert_eq!(a.case_id,b.case_id);
                    assert_eq!(a.inputs,b.inputs,"non-E reservation inputs: {family}/{name}/{}",mode.as_str());
                    assert_eq!(a.remaining_inputs,b.remaining_inputs,"remaining replay/functionals inputs: {family}/{name}/{}",mode.as_str());
                    let delta = b.encoded_request as i128-a.encoded_request as i128;
                    assert_eq!(independent_finalization(b.encoded_request,&b.inputs) as i128-independent_finalization(a.encoded_request,&a.inputs) as i128,24*delta);
                }
                assert_eq!(v2.stages.len(),v3.stages.len());
                let mut selected = 0;
                let mut prefix_delta = 0;
                for (a,b) in v2.stages.iter().zip(&v3.stages) {
                    assert_eq!(a["stage"],b["stage"]);
                    if a["stage"] == "source_case_finalization_entry" {
                        prefix_delta = 0; // equal raw retained system/force/descriptor/row inputs
                    } else if a["stage"] == "source_case_finalization_reserved" {
                        let left = &v2.reservations[selected];let right = &v3.reservations[selected];
                        prefix_delta += independent_wrapper(right.encoded_request,&right.inputs) as i128-independent_wrapper(left.encoded_request,&left.inputs) as i128;
                        selected += 1;
                    }
                    assert_eq!(stage_charged(b)-stage_charged(a),prefix_delta,"remaining work path must have zero delta after equal raw inputs");
                    assert_eq!(a["rejected"],b["rejected"]);
                }
                export_raw_inputs(family,name,false,&v2);export_raw_inputs(family,name,true,&v3);
                twins += 1;
            }
        }
    }
    assert_eq!(twins,24);
}
#[test]
fn receipt_policy_is_bound_to_captured_contract_at_the_private_finalizer() {
    for family in ["physics_source","load_reference_source"] {
        for v3 in [false,true] {
            for contradictory in [false,true] {
                let snapshot = snapshot(family,"n06",v3,PreviewSolverMode::SparseInteractive);
                let (_,capture) = CapturedInvocation::parse(snapshot.raw,snapshot.mode).unwrap();
                let mut envelope = snapshot.envelope;
                let mut budget = snapshot.budget;
                let expected_policy = if v3 {"PRESSURE-SOURCE-1"} else if family == "load_reference_source" {"LOAD-REFERENCE-SOURCE-1"} else {"PHYSICS-SOURCE-1"};
                if contradictory {
                    envelope.producer.semantic_contract_id = if v3 { if family == "load_reference_source" { LOAD_REFERENCE_SOURCE_SEMANTIC_ID } else { COMPOSITE_SEMANTIC_ID } } else { exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID }.into();
                }
                let actual = FinalizedSourceBlockReceipt::finalize_composite(&capture,&envelope,snapshot.cases,&mut budget);
                if contradictory {
                    assert_eq!(actual.unwrap_err().0,"publication must have source identity and omit receipt");
                } else {
                    let actual = actual.unwrap().into_wire();
                    assert_eq!(actual["body"]["policy"],expected_policy);
                    assert_eq!(envelope.producer.semantic_contract_id,capture.composite_context().unwrap().semantic_id());
                }
            }
        }
    }
}
#[test]
fn private_reservation_expression_retains_saturation_without_changing_admitted_values() {
    let empty = ReservationInputs {source_identity_utf8:0,stiffness_dimension:0,descriptors:vec![],rows:vec![],support_actions:0};
    assert_eq!(independent_finalization(1,&empty),24);
    assert_eq!(independent_finalization(usize::MAX,&empty),usize::MAX);
    let mut square = empty.clone();square.stiffness_dimension=usize::MAX;
    assert_eq!(independent_finalization(0,&square),usize::MAX);
    assert_eq!(independent_wrapper(usize::MAX,&empty),usize::MAX);
    // An explicit private oversize fault, not an admitted captured request.
    // It exercises the production saturation/refusal without allocating a
    // usize-sized input or changing any real fixture/counter/limit.
    let mut snapshot = snapshot("physics_source","n06",false,PreviewSolverMode::SparseInteractive);
    let (_,mut capture) = CapturedInvocation::parse(snapshot.raw.clone(),snapshot.mode).unwrap();
    capture.encoded_len = usize::MAX;
    let observed = &snapshot.reservations[0];
    let case = snapshot.cases.iter_mut().find(|case|case.case_id == observed.case_id).unwrap();
    let selected = case._selected.as_mut().unwrap();
    let expected = independent_finalization(usize::MAX,&observed.inputs);
    assert_eq!(finalization_size(&capture,selected,&observed.actual_rows),expected);
    assert_eq!(expected,usize::MAX);
    let before = selected.summary().work;
    let refused = selected.charge_finalization(expected).unwrap_err();
    assert_eq!(refused.stage,"receipt finalization");
    assert_eq!(refused.work.limit,before.limit);
    assert_eq!(refused.work.charged,before.charged);
}
