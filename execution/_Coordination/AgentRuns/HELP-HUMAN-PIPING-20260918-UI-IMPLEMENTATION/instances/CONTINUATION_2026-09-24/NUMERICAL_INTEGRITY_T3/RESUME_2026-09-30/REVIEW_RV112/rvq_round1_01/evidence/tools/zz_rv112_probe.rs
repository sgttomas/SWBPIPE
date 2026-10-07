//! RV112 (RV-Q round 1): the reviewer's own probes, installed only in the reviewer's `mut` copy
//! (a child of `retained_memory`, so it reads private items). Not part of the candidate.
use super::*;
use crate::retained_product::{CaptureError, ProductCapture};
use crate::source_receipt::CapturedInvocation;
use crate::PreviewSolverMode;
use serde_json::{json, Value};
use std::cell::RefCell;

const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
const MILESTONE: &str = include_str!("../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json");

thread_local! {
    static REC: RefCell<Option<Vec<(usize, u64, usize)>>> = const { RefCell::new(None) };
}
/// Called (test builds of this copy only) immediately before G-B's site in `prepared_case_source`,
/// with or without a permit: (the case's loads, the capture's RustCapacityBytes, parked slots).
pub(crate) fn record_before_g_b(capture: &ProductCapture, case_loads: usize) {
    REC.with(|r| {
        if let Some(v) = r.borrow_mut().as_mut() {
            v.push((case_loads, capture_bytes(capture), capture.parked_cases().len()));
        }
    });
}

/// RV112's own multi-case builder: `cases` copies of the first case, ids `rv112-case-<k>`, load
/// ids suffixed `@<k>` (unique across cases).
fn cases_of(mut raw: Value, cases: usize) -> Value {
    let first = raw["model"]["load_cases"][0].clone();
    let list = raw["model"]["load_cases"].as_array_mut().unwrap();
    list.clear();
    for k in 0..cases {
        let mut case = first.clone();
        case["id"] = json!(format!("rv112-case-{k}"));
        for load in case["primitive_loads"].as_array_mut().unwrap() {
            load["id"] = json!(format!("{}@{k}", load["id"].as_str().unwrap()));
        }
        list.push(case);
    }
    raw
}
fn milestone() -> Value {
    serde_json::from_str(MILESTONE).unwrap()
}
/// W2b's construction (a cap-maximal model whose ordinary solve publishes), rebuilt here.
fn solvable_cap_maximal() -> Value {
    let mut raw = super::law_tests::cap_maximal();
    for (i, support) in raw["model"]["supports"].as_array_mut().unwrap().iter_mut().enumerate() {
        if i == 0 {
            support.as_object_mut().unwrap().remove("stiffness");
        } else {
            support["family"] = json!("spring");
            support["restraints"] = json!(["UY"]);
        }
    }
    raw
}

/// Item 4: the runner's exact out-of-domain construction (the milestone fixture with three more
/// copies of its case, ids unchanged) is refused by the producer at D1.4 with (Invocation,
/// LoadCases), in both modes, and its typed count is the runner's literal 4.
#[test]
fn zz_rv112_runner_input_is_refused_at_d1_4() {
    for mode in MODES {
        let mut raw = milestone();
        let case = raw["model"]["load_cases"][0].clone();
        for _ in 1..4 {
            raw["model"]["load_cases"].as_array_mut().unwrap().push(case.clone());
        }
        let (request, capture) = CapturedInvocation::parse(raw.clone(), mode).unwrap();
        let report = assess(&capture, &request, Entry::Direct);
        assert_eq!(report.typed.load_cases.length, 4);
        assert_eq!(report.law().domain, Some(AdmissionRefusal::Family(D1Clause::Invocation, FamilyFact::LoadCases)), "{mode:?}");
        assert!(admit(&capture, &request, Entry::Direct).is_err(), "{mode:?}: no permit in any build");
        // The same construction at C cases (identical ids) is inside D1: D1 has no id-uniqueness clause.
        let mut three = milestone();
        let case = three["model"]["load_cases"][0].clone();
        for _ in 1..caps::LOAD_CASES {
            three["model"]["load_cases"].as_array_mut().unwrap().push(case.clone());
        }
        let (request, capture) = CapturedInvocation::parse(three, mode).unwrap();
        println!("RV112_RUNNER_SHAPE_AT_C mode={} domain={:?}", mode.as_str(), assess(&capture, &request, Entry::Direct).law().domain);
    }
}

/// Item 1 (G-B's LateObservationBytes): the capture's RustCapacityBytes immediately before each
/// case's G-B, on the private route (no permit, so no gate skips a capture), and at G-C. At case k
/// the fact already holds the earlier cases' late old-source captures.
#[test]
fn zz_rv112_g_b_bytes_per_case() {
    let late_bound = phase_caps().late[LATE_FACTS - 1];
    println!("RV112_BUILD registered={} status={:?}", COMPILED_IDENTITY == Some(REGISTERED_PROFILES[0].identity), build_status().err());
    println!("RV112_FORMS T11={} T11_late={} T11_seed={} late_bound={} complete_T11={}",
        profile_bytes(profile::F_T11), profile_bytes(profile::F_T11_LATE_CAPTURE), profile_bytes(profile::F_T11_ORDINARY_SEED),
        late_bound, phase_caps().complete[15]);
    println!("RV112_SIZES CaseSlot={} ProductCapture={} OrdinarySeed={} NestedTypedFacts={} AdmissionLaw={} RetainedAdmissionReport={}",
        std::mem::size_of::<crate::retained_product::CaseSlot>(), std::mem::size_of::<ProductCapture>(),
        std::mem::size_of::<crate::retained_product::OrdinarySeed>(), std::mem::size_of::<NestedTypedFacts>(),
        std::mem::size_of::<AdmissionLaw>(), std::mem::size_of::<RetainedAdmissionReport>());
    for (label, base) in [("milestone", milestone()), ("solvable_cap_maximal", solvable_cap_maximal())] {
        for mode in MODES {
            let mut previous: Option<Vec<u64>> = None;
            for cases in 1..=caps::LOAD_CASES {
                let (request, capture) = CapturedInvocation::parse(cases_of(base.clone(), cases), mode).unwrap();
                assert_eq!(assess(&capture, &request, Entry::Direct).law().domain, None, "{label} {cases}: inside D1");
                let mut observer = ProductCapture::prepared_probe();
                REC.with(|r| *r.borrow_mut() = Some(Vec::new()));
                let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
                let rec = REC.with(|r| r.borrow_mut().take().unwrap());
                let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: &observer, requested_cases: cases });
                let get = |fact| o.iter().find(|x| x.fact == fact).unwrap().observed;
                let before: Vec<u64> = rec.iter().map(|r| r.1).collect();
                println!("RV112_G_B input={label} mode={} cases={cases} mechanics={} seeds={} before_g_b={:?} parked_at_g_b={:?} at_g_c={} seed_bytes={} over_late_bound={:?}",
                    mode.as_str(), ordinary.status.mechanics, observer.ordinary.len(), before, rec.iter().map(|r| r.2).collect::<Vec<_>>(),
                    get(PhaseFact::ObservationBytes), get(PhaseFact::OrdinarySeedBytes), before.iter().map(|b| *b > late_bound).collect::<Vec<_>>());
                assert_eq!(rec.len(), cases, "{label} {cases}: one G-B site per case");
                // The mechanism: each later case's G-B sees strictly more bytes than the earlier one's.
                assert!(before.windows(2).all(|w| w[1] > w[0]), "{label} {cases}: the fact accumulates");
                // Case 0's site does not depend on c (nothing is parked yet); later sites include the
                // parked-slot reservation, (c - 1) CaseSlots, made once at case 1's early hook.
                if let Some(p) = &previous {
                    assert_eq!(before[0], p[0], "{label} {cases}: case 0's site is independent of c");
                }
                previous = Some(before);
            }
        }
    }
}

/// Item 5: RetainedErrorTextBytes at c = C (three slots: two parked, one in the capture's own
/// fields). Each slot's error and observable_error is counted, the last parked slot included.
#[test]
fn zz_rv112_retained_error_text_reads_every_slot_at_c() {
    for mode in MODES {
        let (request, capture) = CapturedInvocation::parse(cases_of(milestone(), caps::LOAD_CASES), mode).unwrap();
        let mut observer = ProductCapture::prepared_probe();
        let ordinary = crate::run_linear_static_preview_observed(request, mode, Some(&capture), &mut crate::SourceRecoveryBudget::default(), Some(&mut observer));
        assert_eq!((observer.cases_seen(), observer.parked_cases().len()), (3, 2), "{mode:?}");
        let fact = |observer: &ProductCapture| {
            let o = complete_observations(&CompleteFacts { ordinary: &ordinary, capture: observer, requested_cases: 3 });
            o.iter().find(|x| x.fact == PhaseFact::RetainedErrorTextBytes).unwrap().observed
        };
        assert_eq!(fact(&observer), 0, "{mode:?}: no error text");
        observer.with_case(1, |case| case.observable_error = Some(CaptureError::Association(String::with_capacity(101))));
        assert_eq!(fact(&observer), 101, "{mode:?}: the second (last) parked slot's observable_error");
        observer.with_case(0, |case| case.error = Some(CaptureError::Association(String::with_capacity(13))));
        observer.with_case(1, |case| case.error = Some(CaptureError::Association(String::with_capacity(17))));
        observer.error = Some(CaptureError::Association(String::with_capacity(5)));
        assert_eq!(fact(&observer), 101 + 13 + 17 + 5, "{mode:?}: every slot");
        // A non-association error owns no text.
        observer.with_case(0, |case| case.observable_error = Some(CaptureError::CountRange("x")));
        assert_eq!(fact(&observer), 101 + 13 + 17 + 5, "{mode:?}");
    }
}
