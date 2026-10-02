//! The adapter (plan §4.3, §4.1, C10) and K4's canonical order (plan Q7).
//! - Every CI model, built into K4's `PrimitiveSource`, has the canonical bytes
//!   the generator built independently from `references.py --model` (their
//!   sha256 is committed): the adapter against R1's model, rounded once, node
//!   and member order included.
//! - RF-MECH-K0's literal model (with R1's k = 0 spring) is refused by the
//!   source, naming the spring (C10).
//! - Permuting every list of a source changes neither its canonical bytes nor,
//!   for the solved RF-INVARIANCE cases, a single published bit.
use open_pipe_stress_frame_kernel::structural::retained_api::{
    solve_case, CaseLimit, CaseOutcome, Component, Dof, InvocationMeter, PrimitiveSource,
    SourceError, SourceParts, Spring,
};
use piping_numerical_robustness::cases::{load_all, load_family};
use piping_numerical_robustness::sha256::sha256_hex;

#[test]
fn every_ci_models_canonical_bytes_equal_the_generators_from_references_py_model() {
    let mut n = 0;
    for c in load_all().iter().filter(|c| !c.is_large()) {
        let parts = c.model.as_ref().unwrap().source_parts();
        let source = PrimitiveSource::new(parts).unwrap_or_else(|e| panic!("{}: {e:?}", c.id));
        assert_eq!(sha256_hex(&source.encoding()), c.k4src_sha256, "{}", c.id);
        n += 1;
    }
    assert_eq!(n, 201);
}

#[test]
fn rf_mech_k0s_literal_model_is_refused_by_the_source_naming_its_spring() {
    let c = load_family("RF-MECH")
        .into_iter()
        .find(|c| c.id == "RF-MECH-K0")
        .unwrap();
    let model = c.model.as_ref().unwrap();
    assert_eq!(model.omitted_springs, ["S.N0.0"]);
    let mut parts = model.source_parts();
    // R1's spring: kind rotation, direction (1, 0, 0), k = 0, the first spring (id 1).
    parts.springs.push(Spring {
        id: 1,
        dof: Dof {
            node: 0,
            component: Component::Rx,
        },
        stiffness: 0.0,
    });
    match PrimitiveSource::new(parts) {
        Err(e) => {
            assert_eq!(e, SourceError::NonPositiveSpring { id: 1 });
            println!("RF-MECH-K0 literal model: {e:?}");
        }
        Ok(_) => panic!("the k = 0 spring was accepted"),
    }
}

fn reversed(parts: &SourceParts) -> SourceParts {
    let mut p = parts.clone();
    p.members.reverse();
    p.springs.reverse();
    p.directional_springs.reverse();
    p.constraints.reverse();
    p.loads.reverse();
    p.stations.reverse();
    p
}

#[test]
fn permuting_every_list_changes_no_canonical_byte() {
    for c in load_all().iter().filter(|c| !c.is_large()) {
        let parts = c.model.as_ref().unwrap().source_parts();
        let a = PrimitiveSource::new(parts.clone()).unwrap().encoding();
        let b = PrimitiveSource::new(reversed(&parts)).unwrap().encoding();
        assert_eq!(a, b, "{}", c.id);
    }
}

#[test]
fn permuting_every_list_changes_no_published_bit_on_rf_invariance() {
    let mut n = 0;
    for c in load_family("RF-INVARIANCE")
        .iter()
        .filter(|c| !c.id.contains("TREE100"))
    {
        let parts = c.model.as_ref().unwrap().source_parts();
        let solve = |p: SourceParts| {
            let mut meter = InvocationMeter::new(u64::MAX);
            match solve_case(
                PrimitiveSource::new(p).unwrap(),
                CaseLimit::new(u64::MAX),
                &mut meter,
            ) {
                CaseOutcome::Selected(s) => format!("{:?} {:?}", s.publish(), s.evidence()),
                other => panic!("{}: {other:?}", c.id),
            }
        };
        assert_eq!(solve(parts.clone()), solve(reversed(&parts)), "{}", c.id);
        n += 1;
    }
    assert_eq!(n, 21);
}
