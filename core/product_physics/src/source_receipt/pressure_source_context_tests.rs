//! J1/B7b: the captured source context and its publication check. Since B7b
//! deleted v2, the captured current exact contract is the only composite
//! context; the retired v2 identities and contract are refusal controls.
use super::*;

/// The retired v2 composite identities, as received text only (B7b refuses them
/// by name at header dispatch; the producer never publishes them).
const RETIRED_PHYSICS_SOURCE_ID: &str = "openpipestress.result_semantics/0.3.0/physics-source-1";
const RETIRED_LOAD_REFERENCE_SOURCE_ID: &str =
    "openpipestress.result_semantics/0.3.0/load-reference-source-1";
const RETIRED_V2_CONTRACT: &str = r#"{"version":"2.0.0","mode":"exact_straight_pressure_v2"}"#;

/// The committed current (0.5.0, v3, CA 0, Euler-Bernoulli) source controls.
fn fixture_request(family: &str, name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
        "../../fixtures/product_preview/pressure_source/{family}/{name}.request.json"
    ));
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

fn request(load_state: bool) -> Value {
    fixture_request(if load_state { "load_reference_source" } else { "physics_source" }, "n05")
}

#[test]
fn captured_composite_context_names_the_pressure_source_policy_on_both_families() {
    for mode in [
        PreviewSolverMode::SparseInteractive,
        PreviewSolverMode::DenseScrutiny,
    ] {
        for load_state in [false, true] {
            let raw = request(load_state);
            assert_eq!(raw["model"]["schema_version"], crate::MODEL_SCHEMA_VERSION);
            let (parsed, captured) = CapturedInvocation::parse(raw, mode).unwrap();
            assert_eq!(case_state::is_load_state(&parsed.model), load_state);
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
                    context.profile_id(),
                    context.profile_mode(),
                ),
                (
                    "openpipestress.result_semantics/0.3.0/pressure-source-1",
                    "PRESSURE-SOURCE-1",
                    "pressure_source_case_evidence_v1",
                    "exact_pressure_v3",
                    "exact_pressure_v3",
                )
            );
        }
    }
}

#[test]
fn publication_label_is_checked_against_the_captured_policy() {
    for load_state in [false, true] {
        let capture =
            CapturedInvocation::parse(request(load_state), PreviewSolverMode::SparseInteractive)
                .unwrap()
                .1;
        let context = capture.composite_context().unwrap();
        assert_eq!(context, CompositeSourceContext::PressureSource);
        let good = json!({"producer":{"semantic_contract_id":context.semantic_id()}});
        assert_eq!(
            validate_publication_identity(&good, context.receipt_policy()),
            Ok(())
        );
        for other in [
            exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID,
            RETIRED_PHYSICS_SOURCE_ID,
            RETIRED_LOAD_REFERENCE_SOURCE_ID,
        ] {
            let bad_label = json!({"producer":{"semantic_contract_id":other}});
            assert_eq!(
                validate_publication_identity(&bad_label, context.receipt_policy()),
                Err(bad(
                    "publication must have source identity and omit receipt"
                )),
                "{other}"
            );
        }
        let with_receipt = json!({"producer":{"semantic_contract_id":context.semantic_id()},"source_block_recovery":null});
        assert!(validate_publication_identity(&with_receipt, context.receipt_policy()).is_err());
    }
}

#[test]
fn captured_context_does_not_infer_a_contract_from_a_publication_claim() {
    let mut raw = request(false);
    raw["producer"] = json!({"semantic_contract_id":RETIRED_PHYSICS_SOURCE_ID});
    let (_, captured) = CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
    assert_eq!(
        captured.composite_context().unwrap(),
        CompositeSourceContext::PressureSource
    );
    // B7b: a present null contract fails the shared decoding boundary.
    let mut null_contract = request(false);
    null_contract["model"]["pressure_contract"] = Value::Null;
    assert!(CapturedInvocation::parse(null_contract, PreviewSolverMode::DenseScrutiny).is_err());
    for contract in [
        serde_json::from_str(RETIRED_V2_CONTRACT).unwrap(),
        json!({"version":"3.0.0","mode":"exact_straight_pressure_v2","shear_model":"euler_bernoulli"}),
        json!({"version":"2.0.0","mode":"exact_pressure_v3","shear_model":"euler_bernoulli"}),
    ] {
        let mut raw = request(false);
        raw["model"]["pressure_contract"] = contract;
        let (_, captured) =
            CapturedInvocation::parse(raw, PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(
            captured.composite_context(),
            Err(bad("composite source requires an actual exact contract"))
        );
    }
    let mut preview = request(false);
    preview["model"]
        .as_object_mut()
        .unwrap()
        .remove("pressure_contract");
    let (_, captured) =
        CapturedInvocation::parse(preview, PreviewSolverMode::DenseScrutiny).unwrap();
    assert!(captured.composite_context().is_err());
}

#[test]
fn pressure_case_context_uses_one_domain_without_changing_its_payload_shape() {
    for load_state in [false, true] {
        let (_, captured) = CapturedInvocation::parse(
            request(load_state),
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

/// The owner-approved J1 text amendment (b7-plan, 2026-10-12): the held
/// pre-join limitation [7] and its joined, unselected replacement.
const HELD_LIMITATION_7: &str = "No corrosion-allowance input or shear-deformation selection is provided. Retained-source recovery is not joined for this contract; only ordinary structural recovery is published. No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";
// B7b (finding (b)): the joined text states the current contract's allowance and
// shear-model fields and keys the eigenload clause on the authored group, not
// 0.4.0. The held pre-join text above is unchanged.
const JOINED_LIMITATION_7: &str = "Every member section declares its corrosion allowance and the contract declares its shear model; only a zero allowance and the euler_bernoulli shear model are admitted. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";

/// The regenerated v3 reader corpora are unselected pressure-1 publications:
/// each equals the joined unselected basis exactly. The held pre-join helper
/// differs from it only at limitation [7], by exactly the approved OLD/CURRENT
/// pair; the selected basis names the source publication.
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
            // B7b: behaviour keys on the group's presence, never the version.
            let load_state = case["envelope"]["contract_evidence"]
                .get("load_reference_states")
                .is_some();
            let held = exact_admission::pressure_v3_formulation_basis(load_state, connectors);
            let unselected = exact_admission::pressure_v3_formulation_basis_with_source(
                load_state, connectors, false,
            );
            // The producer (`formulation_basis_for_model_with_source`) appends
            // K-D5's two route-independent certified-recovery statements.
            let published = &case["envelope"]["formulation_basis"];
            assert_eq!(published["profile_id"], unselected.profile_id);
            let published: Vec<String> =
                serde_json::from_value(published["limitations"].clone()).unwrap();
            let (prefix, tail) = published.split_at(unselected.limitations.len());
            assert_eq!(prefix, unselected.limitations.as_slice());
            assert_eq!(tail.len(), 2);
            assert!(tail[0].starts_with("Member actions, section actions, stresses and support reactions"));
            assert!(tail[1].starts_with("The recovery is checked against the formation check's re-formed system"));
            assert_eq!(held.limitations[7], HELD_LIMITATION_7);
            assert_eq!(unselected.limitations[7], JOINED_LIMITATION_7);
            let mut amended = held.limitations.clone();
            amended[7] = JOINED_LIMITATION_7.to_string();
            assert_eq!(amended, unselected.limitations);
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
        state.reservations.push(ObservedReservation {
            case_id: selected.retained().descriptors()[0].key.case_id.clone(),
            encoded_request: encoded_request(capture.borrowed_raw()),
            inputs: reservation_inputs(selected, actual),
            actual_rows: actual.to_vec(),
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
fn snapshot(family: &str, name: &str, mode: PreviewSolverMode) -> FinalizerSnapshot {
    let raw = fixture_request(family,name);
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
fn export_raw_inputs(family: &str,name: &str,snapshot: &FinalizerSnapshot) {
    if let Some(directory) = std::env::var_os("SWBPIPE_PRESSURE_JOIN_RAW_INPUTS_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        let rows: Vec<_> = snapshot.reservations.iter().map(|input|json!({"case_id":input.case_id,"encoded_request_utf8":input.encoded_request,"inputs":input.inputs})).collect();
        let path = directory.join(format!("{family}-{name}-{}-v3.raw-inputs.json",snapshot.mode.as_str()));
        std::fs::write(path,serde_json::to_vec(&rows).unwrap()).unwrap();
    }
}
/// D5/MJ9's private half, over every committed current control (12 requests x
/// two modes): each selected case's finalization reservation equals the
/// independently evaluated complete expression `2(12E + B_k)` from its raw site
/// inputs, and each wrapper stage delta equals `1024 + 64 rows + F + 4096
/// support actions`. B7b deleted the v2 twin side; the same arithmetic is now
/// checked directly on each control instead of as a v2/v3 difference.
#[test]
fn pressure_source_private_reservation_inputs_confirm_all_twins_complete_expression() {
    let families: [(&str,&[&str]);2] = [
        ("physics_source",&["n05","n06","n05_unicode","n05_units","fields","mixed","mixed_units"]),
        ("load_reference_source",&["n05","n06","eigen_motion","fields","mixed"]),
    ];
    let mut controls = 0;
    for (family,names) in families {
        for name in names {
            for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
                let current = snapshot(family,name,mode);
                check_snapshot_inputs(&current);
                export_raw_inputs(family,name,&current);
                controls += 1;
            }
        }
    }
    assert_eq!(controls,24);
}
/// MJ1's kill, at the private finalizer: the policy and identity come from the
/// captured contract. A contradictory envelope label (the ordinary pressure-1
/// identity, or a retired v2 composite identity) fails finalization; the
/// correctly labelled controls of both families publish PRESSURE-SOURCE-1.
#[test]
fn receipt_policy_is_bound_to_captured_contract() {
    for family in ["physics_source","load_reference_source"] {
        for contradictory in [None, Some(exact_admission::PRESSURE_SEMANTIC_CONTRACT_ID),
            Some(RETIRED_PHYSICS_SOURCE_ID), Some(RETIRED_LOAD_REFERENCE_SOURCE_ID)] {
            let snapshot = snapshot(family,"n06",PreviewSolverMode::SparseInteractive);
            let (_,capture) = CapturedInvocation::parse(snapshot.raw,snapshot.mode).unwrap();
            let mut envelope = snapshot.envelope;
            let mut budget = snapshot.budget;
            if let Some(label) = contradictory {
                envelope.producer.semantic_contract_id = label.into();
            }
            let actual = FinalizedSourceBlockReceipt::finalize_composite(&capture,&envelope,snapshot.cases,&mut budget);
            if contradictory.is_some() {
                assert_eq!(actual.unwrap_err().0,"publication must have source identity and omit receipt");
            } else {
                let actual = actual.unwrap().into_wire();
                assert_eq!(actual["body"]["policy"],"PRESSURE-SOURCE-1");
                assert_eq!(envelope.producer.semantic_contract_id,capture.composite_context().unwrap().semantic_id());
                assert_eq!(envelope.producer.semantic_contract_id,exact_admission::PRESSURE_SOURCE_SEMANTIC_CONTRACT_ID);
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
    let mut snapshot = snapshot("physics_source","n06",PreviewSolverMode::SparseInteractive);
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

#[test]
fn cached_source_context_keeps_original_raw_custody_and_typed_selection() {
    for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
        for load_state in [false,true] {
            let raw=request(load_state);
            let before=raw.clone();
            let encoded=checked(&raw).unwrap().len();
            let digest=hash("source_blocks_invocation_v1",&json!({"request":&raw,"solver_mode":mode.as_str()})).unwrap();
            let (mut typed,capture)=CapturedInvocation::parse(raw,mode).unwrap();
            let selected=CompositeSourceContext::for_model(&typed.model).unwrap();
            assert_eq!(capture.context,Some(selected));
            // The caller's typed half may be normalized/changed later. Neither
            // it nor an envelope label can redefine this capture's context.
            typed.model.schema_version=crate::MODEL_SCHEMA_VERSION.into();
            typed.model.pressure_contract=None;
            for _ in 0..8 { assert_eq!(capture.composite_context(),Ok(selected)); }
            assert_eq!(capture.borrowed_raw(),&before);
            assert_eq!(capture.encoded_length(),encoded);
            assert_eq!(capture.borrowed_digest(),&digest);
            assert_eq!(capture.mode(),mode);
        }
    }
}

#[test]
fn cached_source_context_none_preserves_preview_and_invalid_contract_refusal() {
    let refusal=bad("composite source requires an actual exact contract");
    for schema in ["0.1.0","0.2.0","0.3.0","0.4.0","0.5.0"] {
        // B7b: a present null contract fails decoding (no capture is formed).
        let mut null_contract=request(false);
        null_contract["model"]["schema_version"]=json!(schema);
        null_contract["model"]["pressure_contract"]=Value::Null;
        assert!(CapturedInvocation::parse(null_contract,PreviewSolverMode::SparseInteractive).is_err());
        for contract in [None,Some(json!({})),
            Some(serde_json::from_str(RETIRED_V2_CONTRACT).unwrap()),
            Some(json!({"version":"3.0.0","mode":"exact_straight_pressure_v2"})),
            Some(json!({"version":"2.0.0","mode":"exact_pressure_v3"})),
            Some(json!({"version":"invalid","mode":"invalid"}))] {
            let mut raw=request(false);
            raw["model"]["schema_version"]=json!(schema);
            if let Some(contract)=contract { raw["model"]["pressure_contract"]=contract; }
            else {raw["model"].as_object_mut().unwrap().remove("pressure_contract");}
            let before=raw.clone();
            let (_,capture)=CapturedInvocation::parse(raw,PreviewSolverMode::SparseInteractive).unwrap();
            assert_eq!(capture.context,None);
            assert_eq!(capture.composite_context(),Err(refusal.clone()));
            assert_eq!(capture.borrowed_raw(),&before);
        }
    }
    // The current contract on a retired document version has no context.
    for schema in crate::RETIRED_MODEL_SCHEMA_VERSIONS {
        let mut raw=request(false);raw["model"]["schema_version"]=json!(schema);
        let (_,capture)=CapturedInvocation::parse(raw,PreviewSolverMode::DenseScrutiny).unwrap();
        assert_eq!(capture.composite_context(),Err(refusal.clone()));
    }
}

#[test]
fn cached_source_context_preserves_checked_hash_and_serde_failure_order() {
    // Both faults are present: checked canonical input refuses before serde.
    let mut raw=request(false);
    raw["unsafe_integer"]=json!(9_007_199_254_740_993_u64);
    raw["model"]["pressure_contract"]["version"]=json!(42);
    let expected=checked(&raw).unwrap_err();
    assert_eq!(CapturedInvocation::parse(raw,PreviewSolverMode::DenseScrutiny).unwrap_err(),expected);
    for bad_contract in [json!(42),json!([]),json!({"version":42,"mode":"exact_pressure_v3"}),json!({"version":"3.0.0","mode":[]})] {
        let mut raw=request(false);raw["model"]["pressure_contract"]=bad_contract;
        checked(&raw).unwrap();
        let error=CapturedInvocation::parse(raw,PreviewSolverMode::SparseInteractive).unwrap_err();
        assert!(error.0.starts_with("request: "),"{}",error.0);
        assert_eq!(error.1,None);
    }
}

#[test]
fn cached_source_context_accessor_has_no_clone_or_reparse_and_one_constructor() {
    let source=include_str!("../source_receipt.rs");
    let accessor=source.split("pub(super) fn composite_context(&self)").nth(1).unwrap().split("\n    }").next().unwrap();
    for forbidden in ["requested(","clone(","from_value(","from_str(","borrowed_raw("] {
        assert!(!accessor.contains(forbidden),"context accessor contains {forbidden}");
    }
    assert!(accessor.contains("self.context.ok_or_else"));
    let capture=source.split("impl CapturedInvocation {").nth(1).unwrap().split("\n}\n").next().unwrap();
    assert_eq!(capture.matches("Self {").count(),1);
    assert!(capture.contains("let context = CompositeSourceContext::from_model(&parsed.model)"));
}

#[test]
fn cached_source_context_reports_actual_positive_layout_and_lifetime() {
    use std::mem::{align_of,offset_of,size_of};
    let fields=size_of::<Value>()+size_of::<PreviewSolverMode>()+size_of::<String>()+size_of::<usize>()+size_of::<Option<CompositeSourceContext>>();
    let total=size_of::<CapturedInvocation>();
    // B7b A4 deleted the v2 variant: the single-variant context is zero-sized,
    // and the capture's inline `Option` tag remains positive (D3 prices it).
    assert_eq!(size_of::<CompositeSourceContext>(),0);
    assert!(size_of::<Option<CompositeSourceContext>>()>0);
    assert!(total>=fields);
    println!("capture_context_layout={}",json!({
        "enum":{"size":size_of::<CompositeSourceContext>(),"align":align_of::<CompositeSourceContext>()},
        "option":{"size":size_of::<Option<CompositeSourceContext>>(),"align":align_of::<Option<CompositeSourceContext>>()},
        "capture":{"size":total,"align":align_of::<CapturedInvocation>(),"field_size_sum":fields,"padding":total-fields},
        "offsets":{"raw":offset_of!(CapturedInvocation,raw),"mode":offset_of!(CapturedInvocation,mode),"digest":offset_of!(CapturedInvocation,digest),"encoded_len":offset_of!(CapturedInvocation,encoded_len),"context":offset_of!(CapturedInvocation,context)}
    }));
    // The positive inline tag owns no heap backing. Existing raw Value and
    // digest String owners remain with the non-Clone capture through its move/
    // borrow lifetime; this measurement does not infer allocator/compiler reuse.
    let (_,capture)=CapturedInvocation::parse(request(false),PreviewSolverMode::SparseInteractive).unwrap();
    let moved=capture;let borrowed=&moved;
    assert_eq!(borrowed.composite_context(),Ok(CompositeSourceContext::PressureSource));
}

/// The captured context's caching preserves every committed current public
/// serialization (12 requests x two modes). B7b deleted the optional v2 side.
#[test]
fn cached_source_context_preserves_current_public_serializations() {
    let project=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let families: [(&str,&[&str]);2]=[
        ("physics_source",&["n05","n06","n05_unicode","n05_units","fields","mixed","mixed_units"]),
        ("load_reference_source",&["n05","n06","eigen_motion","fields","mixed"]),
    ];
    let mut controls=0;
    for (family,names) in families {
        for name in names {
            let raw=fixture_request(family,name);
            for mode in [PreviewSolverMode::SparseInteractive,PreviewSolverMode::DenseScrutiny] {
                let envelope=run_linear_static_preview_value_with_mode(raw.clone(),mode).unwrap();
                let bytes=serde_json::to_string(&envelope).unwrap();
                let expected=project.join(format!("fixtures/product_preview/pressure_source/{family}/{name}-{}.raw.json",mode.as_str()));
                let expected_bytes=std::fs::read(&expected).unwrap();
                let first_difference=bytes.as_bytes().iter().zip(&expected_bytes).position(|(a,b)|a!=b);
                assert!(bytes.as_bytes()==expected_bytes,"exact public serialization {}: actual_len={}, expected_len={}, first_difference={first_difference:?}",expected.display(),bytes.len(),expected_bytes.len());
                controls+=1;
            }
        }
    }
    assert_eq!(controls,24);
    println!("cache_serialization_controls={controls}");
}
