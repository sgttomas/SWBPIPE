use open_pipe_stress_solver_performance_harness::k6::w1::{
    counts::compute_described, h_envelope::*,
};
use open_pipe_stress_solver_performance_harness::k6::{
    canonical,
    models::{model, model_described},
};
fn profile() -> ReferenceHProfile {
    ReferenceHProfile::source40129_rust1971_aarch64_v1()
}

#[test]
fn actual_arguments_and_repeat_digits_are_bound_without_reference_substitution() {
    let (m, origin) = model_described("RF-LARGE-CHAIN-n00010-AX").unwrap();
    let (c, error, source) = compute_described(&m);
    assert!(error.is_none());
    let facts = HModelFacts::capture(&m, origin).unwrap();
    let launch = HLaunch {
        retained_arguments: [Some(&m.id), None, None, None, None, None],
        repeats: 5,
        prefixes: true,
    };
    let a = estimate(facts, source.unwrap(), launch, &profile()).unwrap();
    let mut changed = launch;
    changed.retained_arguments[5] = Some("a retained unused or published path");
    let b = estimate(facts, source.unwrap(), changed, &profile()).unwrap();
    assert_eq!(
        b.full.moving - a.full.moving,
        "a retained unused or published path".len() as u128
    );
    let mut repeats = launch;
    repeats.repeats = 100;
    let extended = estimate(facts, source.unwrap(), repeats, &profile()).unwrap();
    // Error/Line formatting receives two additional digits even when the solve
    // remains the global maximum. Inspect the actual prefix window separately.
    assert!(extended.prefix_window.moving >= a.prefix_window.moving);
    let (plain, error) = stage_line_bounds(5, &profile()).unwrap();
    let (plain100, error100) = stage_line_bounds(100, &profile()).unwrap();
    assert_eq!((plain.requested, plain.moving), (708, 1023));
    assert_eq!((error.requested, error.moving), (2756016, 3937177));
    assert_eq!((plain100.requested, plain100.moving), (712, 1029));
    assert_eq!((error100.requested, error100.moving), (2756020, 3937183));
    assert_eq!(HSourceFacts::from_counts(&m, &c).unwrap(), source.unwrap());
    let reference = original_k6b_pair(&m.id, &c, &profile()).unwrap();
    assert_ne!(a.legacy.fixed, reference.legacy.fixed);
}

#[test]
fn constructor_origin_is_captured_on_real_routes_and_keeps_models_identical() {
    for id in [
        "RF-LARGE-CHAIN-n00010-AX",
        "K6-CEIL-CHAIN-n01364-AX",
        "K6-GRID-16x16",
    ] {
        let (m, origin) = model_described(id).unwrap();
        assert_eq!(m, model(id).unwrap());
        let text = canonical::serialize(&m);
        let (parsed, canonical_origin) = canonical::parse_described(&text).unwrap();
        assert_eq!(m, parsed);
        assert_ne!(origin, canonical_origin);
        assert!(HModelFacts::capture(&m, origin).unwrap().retained_bytes() > 0);
        assert!(
            HModelFacts::capture(&parsed, canonical_origin)
                .unwrap()
                .retained_bytes()
                > 0
        );
    }
    let id = open_pipe_stress_solver_performance_harness::k6::models::sealed_model_ids()
        .into_iter()
        .find(|id| id.starts_with("DEC053:"))
        .unwrap();
    let (m, origin) = model_described(&id).unwrap();
    let (_, canonical_origin) = canonical::parse_described(&canonical::serialize(&m)).unwrap();
    assert_ne!(origin, canonical_origin);
}

#[test]
fn refused_raw_source_never_uses_successful_count_identities() {
    let mut m = model("RF-LARGE-CHAIN-n00010-AX").unwrap();
    m.restraints.extend(std::iter::repeat_n((0, [true; 6]), 12));
    m.loads.push((usize::MAX, 0.0));
    let (m, origin) = canonical::parse_described(&canonical::serialize(&m)).unwrap();
    let (c, error, source) = compute_described(&m);
    assert!(!c.source_ok && error.is_some());
    assert!(c.constraints > c.dofs);
    let source = source.unwrap();
    assert_eq!(source.kind(), HOutcomeKind::SourceRefused);
    let e = estimate(
        HModelFacts::capture(&m, origin).unwrap(),
        source,
        HLaunch {
            retained_arguments: [None; 6],
            repeats: 100,
            prefixes: true,
        },
        &profile(),
    )
    .unwrap();
    assert_eq!(e.kind, HOutcomeKind::SourceRefused);
    assert!(e.legacy.max > e.legacy.fixed && e.legacy.fixed > 0);
    assert_eq!(e.legacy.max, e.legacy.sel128);
    assert_eq!(
        (
            e.legacy.decide,
            e.legacy.shared,
            e.legacy.state,
            e.legacy.solve,
            e.legacy.verify,
            e.legacy.pass
        ),
        (0, [0; 4], [0; 4], [0; 4], [0; 3], [0; 3])
    );
    assert_eq!(HSourceFacts::from_counts(&m, &c).unwrap(), source);
}
