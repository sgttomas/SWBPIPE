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

/// The committed current (0.7.0, v3, CA 0, Euler-Bernoulli) source controls.
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

/// T4-U6 (DEL-05-03 Design `section-bases-and-timoshenko.md` §2.4): limitation
/// [7] of pressure-2 (joined, unselected), verbatim from the design, with
/// O4's clauses removed (DEL-04-08 Design `o4-source-extension.md` §3.3:
/// retained-source recovery no longer requires euler_bernoulli or zero mill
/// tolerance and allowance). It replaced B7b's text, which admitted only a
/// zero allowance and euler_bernoulli; the held pre-join text went with its
/// test-only helper.
const JOINED_LIMITATION_7: &str = "Every member section declares its corrosion allowance and the contract declares its shear model. This envelope publishes ordinary structural recovery. Retained-source recovery is available only for straight members, linear restraints or springs, nodal force and moment loads and explicitly empty pressure regions; an authored load/reference-state group includes resolved eigenloads and prescribed motions. Other source families are declined by name. No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";
/// T4-U6 (§2.4): limitation [0] of pressure-2, the B7b text with the
/// shear-model parenthesis, the realized-bend clause and the section-bases
/// sentences of the design (the latter after the admitted-families sentence).
const PRESSURE_1_IDENTITY: &str = "3.0.0/exact_pressure_v3 under pressure-2 semantics. Admitted families: small-displacement homogeneous-isotropic straight circular pipe members (shear deformation as the contract declares: euler_bernoulli, or timoshenko_annulus_energy_v1 with the energy-matched annular shear factor, whose nodal rotations are section rotations; G is derived from E/nu) and linear restraints or springs; realized curved bends are Euler-Bernoulli under either shear model and, with expansion joints authored as objective connectors, are admitted only as the bend and connector limitations state. Section bases: stiffness and pipe metal mass use the nominal wall; contents mass and pressure end-cap areas use the reduced bore; every stress row is published on the nominal section and on the reduced section (wall minus mill tolerance and corrosion allowance) and names its basis, and each basis has its own maximum and headline; no governing basis is chosen. Each basis's Lamé stresses are those of its own annulus, while the axial membrane stress divides the one wall force by that basis's area. Every other component, support, combination or equivalent-static family is refused by name (EXACT_PRESSURE_FAMILY_NOT_ADMITTED) until a later contract revision admits it.";

/// The regenerated v3 reader corpora are unselected pressure-2 publications:
/// each equals the joined unselected basis exactly; the selected basis names
/// the source publication and differs only at limitations [0] and [7].
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
            assert_eq!(unselected.limitations[0], PRESSURE_1_IDENTITY);
            assert_eq!(unselected.limitations[7], JOINED_LIMITATION_7);
            let selected = exact_admission::pressure_v3_formulation_basis_with_source(
                load_state, connectors, true,
            );
            assert_eq!(selected.profile_id, unselected.profile_id);
            assert_eq!(selected.limitations.len(), unselected.limitations.len());
            assert_eq!(&selected.limitations[1..7], &unselected.limitations[1..7]);
            assert_eq!(&selected.limitations[8..], &unselected.limitations[8..]);
            assert!(selected.limitations[0].contains("under pressure-source-1 semantics"));
            // O4 §3.3: the selected response follows the contract's shear
            // model on both section bases; no O4 decline is stated.
            assert!(selected.limitations[0].contains("The selected retained-source response uses the contract's shear model and publishes its stress rows on both section bases."));
            assert!(selected.limitations[7].contains("selected retained-source response"));
            for text in [&selected.limitations[0], &selected.limitations[7], &unselected.limitations[7]] {
                for retired in ["Euler-Bernoulli with coincident section bases", "zero mill tolerance", "zero corrosion allowance", "Timoshenko documents"] {
                    assert!(!text.contains(retired), "{retired}");
                }
            }
            assert!(unselected.limitations[7].contains("publishes ordinary structural recovery"));
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
/// captured contract. A contradictory envelope label (the ordinary pressure-2
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
    for schema in ["0.1.0","0.2.0","0.3.0","0.4.0","0.5.0","0.6.0","0.7.0"] {
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

/// T4-U4b §4.3 (S4): the pre-U4b work counters of every current control, as the S1 producer
/// committed them (f468da4651, `fixtures/product_preview/pressure_source/*/*.raw.json`): (family,
/// name, mode, selected case id, its `work.charged`,
/// `invocation_work.publication_charged`). No case then published a derived row.
const PRE_U4B_WORK: [(&str, &str, &str, &str, u64, u64); 24] = [
    ("physics_source","n05","sparse_interactive","case",797059,264800),
    ("physics_source","n05","dense_scrutiny","case",797059,264908),
    ("physics_source","n06","sparse_interactive","case",791447,164276),
    ("physics_source","n06","dense_scrutiny","case",791447,164228),
    ("physics_source","n05_unicode","sparse_interactive","case:温度:α",833105,265448),
    ("physics_source","n05_unicode","dense_scrutiny","case:温度:α",833105,265556),
    ("physics_source","n05_units","sparse_interactive","case",797257,264608),
    ("physics_source","n05_units","dense_scrutiny","case",797257,264716),
    ("physics_source","fields","sparse_interactive","case",1045422,325524),
    ("physics_source","fields","dense_scrutiny","case",1045422,327792),
    ("physics_source","mixed","sparse_interactive","case",820709,535952),
    ("physics_source","mixed","dense_scrutiny","case",820709,536156),
    ("physics_source","mixed_units","sparse_interactive","case",825683,540536),
    ("physics_source","mixed_units","dense_scrutiny","case",825683,540740),
    ("load_reference_source","n05","sparse_interactive","case",979590,291012),
    ("load_reference_source","n05","dense_scrutiny","case",979590,291120),
    ("load_reference_source","n06","sparse_interactive","case",974108,190488),
    ("load_reference_source","n06","dense_scrutiny","case",974108,190440),
    ("load_reference_source","eigen_motion","sparse_interactive","case:join",1264873,284580),
    ("load_reference_source","eigen_motion","dense_scrutiny","case:join",1264873,284664),
    ("load_reference_source","fields","sparse_interactive","case",1316705,357808),
    ("load_reference_source","fields","dense_scrutiny","case",1316705,360076),
    ("load_reference_source","mixed","sparse_interactive","case",1019368,581884),
    ("load_reference_source","mixed","dense_scrutiny","case",1019368,582040),
];
const DERIVED_KINDS: [&str; 2] = ["element_local_normal_stress_extreme", "element_local_transverse_shear_stress_maximum"];
/// One row's identity and metadata bytes as `finalization_size` counts them, derived here from
/// the row alone.
fn row_bytes(row: &ResultItem) -> u64 {
    let mut n = row.id.len() + row.kind.len() + row.entity_ref.len() + row.unit.len();
    if let Some(m) = &row.metadata {
        n += m.component.len() + m.coordinate_system.len() + m.location.len() + m.basis.len() + m.sign_convention.len();
    }
    n as u64
}

/// R-A's terms for a control's selected case (DEL-04-08 Design
/// `source-work-nondyadic-inputs.md` §3.1, §4), measured on the receipt replay's own input,
/// rebuilt here from the captured invocation exactly as `check_input_with_physical` and
/// `check_load_state_input` build it.
fn ra_work_for(family: &str, raw: &Value, mode: PreviewSolverMode, case_id: &str) -> crate::source_recovery::j7_rf_census::ra_gating::RaWork {
    use crate::source_recovery::j7_rf_census::ra_gating::ra_work;
    let label = format!("{family} {case_id} {}", mode.as_str());
    let (_, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
    let mut diagnostics = Vec::new();
    let measured = if family == "load_reference_source" {
        let replay = capture.captured_load_state_case(case_id).unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let LoadStateReplay { model, resolved, built, .. } = &replay;
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        let stiffness = assemble_case_stiffness(built, &boundary.springs).unwrap();
        let case = &resolved.effective_case;
        let loads = build_load_case_primitive_loads(model, case, &mut diagnostics);
        let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
        let eigen = load_state_eigen_loads(resolved, built).unwrap_or_else(|p| panic!("{label}: {p}"));
        let force = nodal_and_eigen_case_force(&application, &eigen, built).unwrap();
        let prescribed: Vec<_> = boundary.restrained_dofs.iter().map(|&d| (d, resolved.prescribed.get(&d).copied().unwrap_or(0.0))).collect();
        let free: Vec<_> = (0..force.len()).filter(|d| !boundary.restrained_dofs.contains(d)).collect();
        assert!(!has_blocking(&diagnostics) && boundary.findings.is_empty(), "{label}");
        ra_work(&source_recovery::Input {
            model, built, stiffness: &stiffness, force: &force, free: &free, prescribed: &prescribed,
            spring_entries: &boundary.springs, load_case: case, load_application: &application,
            thermal_loads: &eigen, load_state: Some(resolved),
        })
    } else {
        let request = capture.requested().unwrap();
        let mut model = request.model;
        let mut materials = if request.materials.is_empty() { model.materials.clone() } else { request.materials };
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        pressure_material::resolve_base(&model, &mut materials, &mut diagnostics);
        let case = model.load_cases.iter().find(|c| c.id == case_id).unwrap().clone();
        if modulus_basis_key(&case, &mut diagnostics).is_some() {
            materials = materials_for_modulus_basis(&model, &materials, &case, &mut diagnostics).unwrap().0;
        }
        let built = build_model(&model, &materials, &mut diagnostics).unwrap();
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        let mut stiffness = assemble_global_stiffness_with_connectors(built.nodes.len(), &built.frame_elements, &built.connectors).unwrap();
        add_curved_bend_stiffness_contributions(&mut stiffness, &built.curved_bend_elements);
        for spring in &boundary.springs {
            let dof = spring.node_dof.global_index();
            stiffness[dof][dof] += spring.stiffness.value;
        }
        let loads = build_load_case_primitive_loads(&model, &case, &mut diagnostics);
        let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
        let force = nodal_and_eigen_case_force(&application, &[], &built).unwrap();
        let prescribed: Vec<_> = boundary.restrained_dofs.iter().map(|&d| {
            (d, boundary.imposed_displacements.iter().find(|p| p.node_dof.global_index() == d).map(|p| p.displacement.value).unwrap_or(0.0))
        }).collect();
        let free: Vec<_> = (0..force.len()).filter(|d| !boundary.restrained_dofs.contains(d)).collect();
        assert!(!has_blocking(&diagnostics), "{label}");
        ra_work(&source_recovery::Input {
            model: &model, built: &built, stiffness: &stiffness, force: &force, free: &free, prescribed: &prescribed,
            spring_entries: &boundary.springs, load_case: &case, load_application: &application,
            thermal_loads: &[], load_state: None,
        })
    };
    measured.unwrap_or_else(|e| panic!("{label}: {e}"))
}

/// T4-U4b §4.3's complete work law, checked independently on every current control (twelve
/// requests x two modes) against the pre-U4b counters, with R-A's change since then: for each
/// selected case, `work.charged` changes by exactly 24ΔE + ΔF_k + ΔQ_k + ΔR_k, and
/// `publication_charged` by 24ΔE + ΔP + ΔD_k.
/// - ΔE: the encoded request delta (the committed requests are unchanged: 0).
/// - ΔF_k: twice the new rows' identity and metadata bytes (`finalization_size`'s 2(12E + B_k)).
/// - ΔQ_k: the composite wrapper's own 64 per new row; the new treatment reserves no recipe
///   work (no `derived_recipe_entry`, no 4096 per row), so its per-recipe charge is zero.
/// - ΔP: the publication's 32 per new row and 256 per new row per load case.
/// - ΔR_k (R-A, DEL-04-08 Design `source-work-nondyadic-inputs.md` §3.1 "As built", §4):
///   measured by `ra_work` on the replay's own input against the pre-R-A path (the plain
///   plan and the long evaluation). The live attempt changes by the plan term (two units
///   per descriptor for the hint rule, checked) plus the evaluation term; the replay, on the
///   same ledger, by the same two units per descriptor (it rebuilds the hints from the rule)
///   plus the same evaluation term, since it re-evaluates the same descriptors on the
///   bit-identical response.
/// - ΔD_k: twelve per character that the selected diagnostic's printed method work (the live
///   attempt's) gains or loses with R-A's live change: 12 × (digits(W) − digits(W − ΔR_live)).
/// The table keeps the pre-U4b counters; R-A adds a measured term rather than rebasing it, so
/// the U4b terms are still checked against the same history. Each term is derived from the
/// actual rows or the measured evaluations; finalization, the wrapper-only terms, the other
/// local charges (unchanged) and publication are checked separately, then reconciled with the
/// counters exactly. Every control still selects under the unchanged 8,000,000 case limit;
/// each new charge is printed.
#[test]
fn u4b_source_work_law_reconciles_every_control_with_its_pre_u4b_counters() {
    let mut maximum = 0u64;
    for (family, name, mode_name, case_id, old_charged, old_publication) in PRE_U4B_WORK {
        let mode = if mode_name == "sparse_interactive" { PreviewSolverMode::SparseInteractive } else { PreviewSolverMode::DenseScrutiny };
        let label = format!("{family}/{name} {mode_name}");
        let snapshot = snapshot(family, name, mode);
        check_snapshot_inputs(&snapshot);
        // ΔE = 0: the encoded request the counters were formed from is the committed one.
        let e = encoded_request(&snapshot.raw);
        assert_eq!(e, encoded_request(&fixture_request(family, name)), "{label}");
        let observed = snapshot.reservations.iter().find(|r| r.case_id == case_id).unwrap_or_else(|| panic!("{label}: selected case"));
        let new_rows: Vec<&ResultItem> = observed.actual_rows.iter().filter(|r| DERIVED_KINDS.contains(&r.kind.as_str())).collect();
        let members = snapshot.raw["model"]["pipe_segments"].as_array().unwrap().len();
        assert_eq!(new_rows.len(), 30 * members, "{label}: σ⁺, σ⁻ and τ at five stations on both bases per member");
        // Finalization: the new rows' bytes, charged twice; checked against the actual site.
        let delta_f: u64 = 2 * new_rows.iter().map(|r| row_bytes(r)).sum::<u64>();
        let without: Vec<ResultItem> = observed.actual_rows.iter().filter(|r| !DERIVED_KINDS.contains(&r.kind.as_str())).cloned().collect();
        let mut stripped = observed.inputs.clone();
        stripped.rows = reservation_inputs(snapshot.cases.iter().find(|c| c.case_id == case_id).unwrap()._selected.as_ref().unwrap(), &without).rows;
        assert_eq!((independent_finalization(e, &observed.inputs) - independent_finalization(e, &stripped)) as u64, delta_f, "{label}: ΔF_k");
        // Wrapper-only terms: 64 per new row; no recipe work for the new treatment.
        let delta_q = 64 * new_rows.len() as u64;
        assert_eq!((independent_wrapper(e, &observed.inputs) - independent_wrapper(e, &stripped)) as u64, delta_f + delta_q, "{label}: ΔF_k + ΔQ_k");
        let recipes = snapshot.stages.iter().filter(|s| s["stage"] == "derived_recipe_entry").count();
        let supports = snapshot.cases.iter().find(|c| c.case_id == case_id).unwrap()._selected.as_ref().unwrap().support_actions().len();
        // The maximum is reserved at publication and again at finalization; each support norm once.
        assert_eq!(recipes, 2 * members + supports, "{label}: recipe reservations per maximum and support norm, none per derived row");
        // Publication.
        let all_new = snapshot.envelope.results.iter().filter(|r| DERIVED_KINDS.contains(&r.kind.as_str())).count() as u64;
        let load_cases = snapshot.raw["model"]["load_cases"].as_array().unwrap().len() as u64;
        let delta_p = (32 + 256 * load_cases) * all_new;
        // R-A: the measured live and replay terms, and the selected diagnostic's digit term.
        let ra = ra_work_for(family, &snapshot.raw, mode, case_id);
        assert_eq!(ra.plan, 2 * ra.hints, "{label}: the hinted plan charges two units per descriptor");
        let delta_r_live = ra.plan + ra.evaluation;
        let delta_r = delta_r_live + 2 * ra.hints + ra.evaluation;
        let selected: Vec<&str> = snapshot.envelope.diagnostics.iter()
            .filter(|d| d.code == "SOURCE_BLOCK_RECOVERY_SELECTED" && d.affected_refs.iter().any(|r| r == case_id))
            .map(|d| d.message.as_str()).collect();
        assert_eq!(selected.len(), 1, "{label}: one selected diagnostic");
        let printed: i64 = selected[0].split("work: WorkReport { charged: ").nth(1).and_then(|s| s.split(',').next())
            .and_then(|s| s.parse().ok()).unwrap_or_else(|| panic!("{label}: the printed method work"));
        let digits = |n: i64| n.to_string().len() as i64;
        let delta_d = 12 * (digits(printed) - digits(printed - delta_r_live));
        // Reconcile with the actual counters.
        let (_, capture) = CapturedInvocation::parse(snapshot.raw.clone(), snapshot.mode).unwrap();
        let envelope = snapshot.envelope;
        let mut budget = snapshot.budget;
        let receipt = FinalizedSourceBlockReceipt::finalize_composite(&capture, &envelope, snapshot.cases, &mut budget)
            .unwrap_or_else(|e| panic!("{label}: {e:?}")).into_wire();
        let body = &receipt["body"];
        let case = body["cases"].as_array().unwrap().iter().find(|c| c["basis_ref"]["ref_id"] == case_id).unwrap();
        assert_eq!(case["outcome"], "qualified", "{label}: still selects");
        assert_eq!(case["selected_method"], "retained_source_blocks_exact_v1", "{label}");
        let charged = case["work"]["charged"].as_u64().unwrap();
        assert_eq!(charged as i64, (old_charged + delta_f + delta_q) as i64 + delta_r, "{label}: work.charged = old + 24ΔE + ΔF_k + ΔQ_k + ΔR_k");
        let publication = body["invocation_work"]["publication_charged"].as_u64().unwrap();
        assert_eq!(publication as i64, (old_publication + delta_p) as i64 + delta_d, "{label}: publication_charged = old + 24ΔE + ΔP + ΔD_k");
        let cases: u64 = body["cases"].as_array().unwrap().iter()
            .map(|c| c["work"]["charged"].as_u64().unwrap() + c["work"]["reserved_unobserved_failure"].as_u64().unwrap()).sum();
        assert_eq!(body["invocation_work"]["charged"].as_u64().unwrap(), cases + publication, "{label}: invocation work");
        assert!(charged <= case["work"]["limit"].as_u64().unwrap() && case["work"]["limit"].as_u64().unwrap() <= 8_000_000, "{label}");
        maximum = maximum.max(charged);
        println!("U4B_SOURCE_WORK {label} case={case_id} charged={charged} (pre-U4b {old_charged}; +{delta_f} finalization +{delta_q} wrapper; R-A {delta_r}: hints {} plan {} evaluation {} x2) publication={publication} (+{delta_p}; R-A digits {delta_d})", ra.hints, ra.plan, ra.evaluation);
    }
    println!("U4B_SOURCE_WORK maximum per-case charge {maximum} of 8000000");
}

/// T4-U4b §4.3 (S4): genuine `derived_not_qualified` receipts on both families and modes, and
/// their named refusals. Each selected case's σ⁺/σ⁻ and τ rows carry the treatment with no
/// projection, their own formula recipe and exactly the station rows the formula reads; the
/// Rust reader admits it neutrally (no finding, eligibility unchanged) for the two kinds only.
/// Mutations, each resealed so the treatment checks are reached: a wrong recipe, a wrong
/// treatment (the derived row as `checked_derived` or `inspection_only`), a wrong kind (the
/// treatment on another row) and wrong inputs (reordered, foreign basis, missing).
#[test]
fn u4b_derived_not_qualified_receipts_are_genuine_and_their_mutations_refuse_by_name() {
    use open_pipe_stress_result_export::{physics_source as ps, reader_verdict, source_blocks::domain_hash};
    let reseal = |doc: &mut Value| {
        doc["source_block_recovery"]["receipt_sha256"] =
            json!(domain_hash("source_blocks_receipt_v1", &doc["source_block_recovery"]["body"]).unwrap());
    };
    for family in ["physics_source", "load_reference_source"] {
        for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
            let raw = fixture_request(family, "n05");
            let label = format!("{family} {}", mode.as_str());
            let doc = crate::run_linear_static_preview_value_with_mode(raw.clone(), mode).unwrap();
            let doc = serde_json::to_value(&doc).unwrap();
            let invocation = json!({"request": raw, "solver_mode": mode.as_str()});
            let read = |d: &Value| ps::validate_in(d, Some(&invocation), ps::SourceContract::PressureV3);
            assert_eq!(read(&doc), Ok(family == "physics_source"), "{label}: genuine (0.4.0 evidence stays ineligible)");
            if family == "physics_source" {
                let standing = reader_verdict::standing(&doc, Some(&raw["model"]), Some(&invocation));
                assert_eq!(standing["findings"], json!([]), "{label}: neutral: {standing}");
                assert_eq!(standing["eligible"], json!(true), "{label}");
            }
            let rows = doc["source_block_recovery"]["body"]["cases"][0]["rows"].as_array().unwrap().clone();
            let results = doc["results"].as_array().unwrap();
            let kind = |id: &Value| results.iter().find(|r| r["id"] == *id).unwrap()["kind"].as_str().unwrap().to_owned();
            let derived: Vec<usize> = (0..rows.len()).filter(|&i| DERIVED_KINDS.contains(&kind(&rows[i]["result_id"]).as_str())).collect();
            assert_eq!(derived.len(), 30 * raw["model"]["pipe_segments"].as_array().unwrap().len(), "{label}");
            let (mut sigma, mut tau) = (None, None);
            for &i in &derived {
                let t = &rows[i];
                assert_eq!((t["treatment"].as_str(), t["projection_id"].is_null()), (Some("derived_not_qualified"), true), "{label}");
                if kind(&t["result_id"]) == DERIVED_KINDS[0] {
                    assert_eq!(t["recipe_id"], "section_extremes_from_published_stress_rows_v1");
                    let inputs: Vec<String> = t["input_result_ids"].as_array().unwrap().iter().map(kind).collect();
                    assert_eq!(inputs, ["element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z"]);
                    sigma.get_or_insert(i);
                } else {
                    assert_eq!(t["recipe_id"], "transverse_shear_from_published_shear_rows_v1");
                    let inputs: Vec<String> = t["input_result_ids"].as_array().unwrap().iter().map(kind).collect();
                    assert_eq!(inputs, ["element_local_shear_force_y", "element_local_shear_force_z"]);
                    tau.get_or_insert(i);
                }
            }
            let (sigma, tau) = (sigma.unwrap(), tau.unwrap());
            let other = (0..rows.len()).find(|&i| kind(&rows[i]["result_id"]) == "support_reaction_component_v2").unwrap();
            let mutate = |at: usize, edit: &dyn Fn(&mut Value)| {
                let mut d = doc.clone();
                edit(&mut d["source_block_recovery"]["body"]["cases"][0]["rows"][at]);
                reseal(&mut d);
                read(&d).unwrap_err()
            };
            let reversed = |t: &mut Value| { let mut ids = t["input_result_ids"].as_array().unwrap().clone(); ids.reverse(); t["input_result_ids"] = json!(ids); };
            let tau_inputs = rows[tau]["input_result_ids"].clone();
            for (name, at, edit, code) in [
                ("wrong recipe", sigma, Box::new(|t: &mut Value| t["recipe_id"] = json!("transverse_shear_from_published_shear_rows_v1")) as Box<dyn Fn(&mut Value)>, "SOURCE_BLOCKS_NOT_QUALIFIED_RECIPE"),
                ("unregistered recipe", tau, Box::new(|t: &mut Value| t["recipe_id"] = json!("retained_source_straight_stress_v1")), "SOURCE_BLOCKS_NOT_QUALIFIED_RECIPE"),
                ("treatment checked_derived", sigma, Box::new(|t: &mut Value| t["treatment"] = json!("checked_derived")), "SOURCE_BLOCKS_NOT_QUALIFIED_TREATMENT"),
                ("treatment inspection_only", tau, Box::new(|t: &mut Value| { t["treatment"] = json!("inspection_only"); t["recipe_id"] = Value::Null; t["input_result_ids"] = json!([]); }), "SOURCE_BLOCKS_NOT_QUALIFIED_TREATMENT"),
                ("projection named", sigma, Box::new(|t: &mut Value| t["projection_id"] = json!("projection:none")), "SOURCE_BLOCKS_NOT_QUALIFIED_ROW"),
                ("wrong kind", other, Box::new(|t: &mut Value| { t["treatment"] = json!("derived_not_qualified"); t["projection_id"] = Value::Null;
                    t["recipe_id"] = json!("section_extremes_from_published_stress_rows_v1"); }), "SOURCE_BLOCKS_NOT_QUALIFIED_KIND"),
                ("inputs reordered", sigma, Box::new(reversed), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
                ("inputs of τ on σ", sigma, Box::new(move |t: &mut Value| t["input_result_ids"] = tau_inputs.clone()), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
                ("input missing", tau, Box::new(|t: &mut Value| { let ids = t["input_result_ids"].as_array().unwrap()[..1].to_vec(); t["input_result_ids"] = json!(ids); }), "SOURCE_BLOCKS_NOT_QUALIFIED_INPUTS"),
            ] {
                assert_eq!(mutate(at, edit.as_ref()), code, "{label}: {name}");
            }
        }
    }
}
