//! Literal conditional-reference regressions, not measurements of this executable.
//! No model/graph/solver runs and no evidence files are read by these tests.
use open_pipe_stress_solver_performance_harness::k6::w1::envelope::*;

fn axis_input() -> KernelInput {
    // Three members, four nodes, one axis spring and one retained load.
    KernelInput {
        nodes: 4,
        members: 3,
        axis_springs: 1,
        directional_springs: 0,
        constraints: 5,
        load_terms: 1,
        stations: 3,
        load_id_bytes: 2,
        max_load_id_bytes: 2,
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 24,
            free_dofs: 19,
            quantities: 88,
            source_encoding_bytes: 535,
            pattern_entries: 360,
            profile_entries: 142,
        }),
    }
}
fn evaluate(i: KernelInput, policy: PopulationPolicy) -> KernelEnvelope {
    let d = KernelDescriptor::new(i, SourceConstruction::VrModelV1, policy).unwrap();
    kernel_envelope(
        &d,
        &ReferenceKernelProfile::source40129_rust1971_aarch64_v1(),
    )
    .unwrap()
}
fn bytes(r: u128, m: u128) -> MetricBytes {
    MetricBytes {
        requested: r,
        moving: m,
    }
}
fn phase(e: &ScheduleEnvelope, k: KernelPhase, p: u16) -> MetricBytes {
    e.phases()
        .iter()
        .find(|x| {
            x.id == PhaseId {
                phase: k,
                precision: Some(p),
            }
        })
        .unwrap()
        .bytes
}
fn subsets(e: &KernelEnvelope) {
    assert!(e.base > 0 && e.shared[0] + e.solved[0] > 0);
    assert!(e.selected128.returned.union >= e.base + e.shared[0] + e.solved[0]);
    assert!(e.full.solve.requested >= e.selected128.solve.requested);
    assert!(e.full.solve.moving >= e.selected128.solve.moving);
    assert!(e.full.returned.union >= e.selected128.returned.union);
    for short in e.selected128.phases() {
        let full = e.full.phases().iter().find(|x| x.id == short.id).unwrap();
        assert!(full.bytes.requested >= short.bytes.requested);
        assert!(full.bytes.moving >= short.bytes.moving);
    }
    for schedule in [&e.full, &e.selected128] {
        assert!(schedule.returned.union <= schedule.solve.requested);
        assert_eq!(
            schedule.returned.union,
            schedule
                .returned
                .selected
                .max(schedule.returned.refused)
                .max(schedule.returned.unresolved)
        );
        assert_eq!(
            schedule.solve.requested,
            schedule
                .phases()
                .iter()
                .map(|x| x.bytes.requested)
                .max()
                .unwrap()
        );
        assert_eq!(
            schedule.solve.moving,
            schedule
                .phases()
                .iter()
                .map(|x| x.bytes.moving)
                .max()
                .unwrap()
        );
        assert!(schedule
            .phases()
            .iter()
            .all(|x| x.bytes.moving >= x.bytes.requested));
    }
}

#[test]
fn axis_spring_reference_and_owner_phase_regressions() {
    let e = evaluate(axis_input(), PopulationPolicy::NodesAndFreeDofsUpper);
    assert_eq!(
        e.proof_basis,
        KernelProofBasis::Source40129Rust1971Aarch64V1
    );
    assert_eq!(
        (e.base, e.original_source, e.prepared_source_clone),
        (30171, 1298, 954)
    );
    assert_eq!(e.source_constructor, bytes(7730, 7730));
    assert_eq!(e.shared, [69392, 82304, 139936, 207200]);
    assert_eq!(e.solved, [6344, 6344, 10504, 18824]);
    assert_eq!(e.verification, [56352, 97984, 125952]);
    assert_eq!(e.full.solve, bytes(19760059, 20856763));
    assert_eq!(e.selected128.solve, bytes(19066971, 20163675));
    assert_eq!(
        e.full.returned,
        ReturnedBounds {
            selected: 892930,
            refused: 857763,
            unresolved: 857763,
            union: 892930
        }
    );
    assert_eq!(e.selected128.returned.union, 280162);
    assert_eq!(e.full.r7_max, bytes(18770496, 19867200));
    assert_eq!(e.full.phases().len(), 88);
    assert_eq!(e.selected128.phases().len(), 38);
    assert!(e.selected128.r7[0].is_some());
    assert_eq!(e.selected128.r7[1..], [None, None]);
    // Independent reference values protect non-maximal owners as well as totals.
    for (p, r, m) in [
        (KernelPhase::Fallback, 1785987, 1854851),
        (KernelPhase::PassDeltaSolve, 953779, 956083),
        (KernelPhase::Shift, 1030165, 1032725),
        (KernelPhase::ReportBuild, 1053310, 1055870),
        (KernelPhase::ResolutionTop, 875107, 875107),
        (KernelPhase::ResolutionHatCheck, 874019, 874019),
        (KernelPhase::CanonicalBeforeRule, 992123, 993403),
        (KernelPhase::CanonicalCertificate, 992699, 993979),
        (KernelPhase::PublicationDraft, 1001211, 1005307),
        (KernelPhase::Certificate, 999547, 999547),
        (KernelPhase::SelectedFinish, 1025306, 1031082),
    ] {
        assert_eq!(phase(&e.full, p, 1024), bytes(r, m), "{p:?}");
    }
    subsets(&e);
}

#[test]
fn directional_operators_are_distinct_from_axis_springs() {
    let i = KernelInput {
        nodes: 2,
        members: 1,
        axis_springs: 0,
        directional_springs: 1,
        constraints: 6,
        load_terms: 2,
        stations: 1,
        load_id_bytes: 4,
        max_load_id_bytes: 2,
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 12,
            free_dofs: 6,
            quantities: 41,
            source_encoding_bytes: 343,
            pattern_entries: 144,
            profile_entries: 21,
        }),
    };
    let e = evaluate(i, PopulationPolicy::NodesAndFreeDofsUpper);
    assert_eq!(
        (e.base, e.original_source, e.prepared_source_clone),
        (17858, 1148, 580)
    );
    assert_eq!(e.shared, [25480, 30824, 52328, 75112]);
    assert_eq!(e.verification, [20960, 36160, 46976]);
    assert_eq!(e.full.solve, bytes(19130019, 20229283));
    assert_eq!(e.selected128.solve, bytes(18869539, 19968803));
    assert_eq!(e.full.returned.union, 346229);
    assert_eq!(e.selected128.returned.union, 117557);
    subsets(&e);
}

#[test]
fn cancellation_keeps_all_loads_and_non_power_of_two_body_capacity() {
    // The reference input retains all three authored load terms, not a net load.
    let i = KernelInput {
        nodes: 3,
        members: 2,
        axis_springs: 0,
        directional_springs: 0,
        constraints: 6,
        load_terms: 3,
        stations: 2,
        load_id_bytes: 6,
        max_load_id_bytes: 2,
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 18,
            free_dofs: 12,
            quantities: 63,
            source_encoding_bytes: 445,
            pattern_entries: 252,
            profile_entries: 78,
        }),
    };
    let e = evaluate(i, PopulationPolicy::NodesAndFreeDofsUpper);
    assert_eq!(e.base, 23945);
    assert_eq!(e.full.solve, bytes(19431296, 20530560));
    assert_eq!(e.selected128.solve, bytes(18962576, 20061840));
    assert_eq!(e.full.returned.union, 608726);
    assert_eq!(e.selected128.returned.union, 194006);
    assert_eq!(
        phase(&e.full, KernelPhase::ResolutionTop, 1024),
        bytes(596465, 596465)
    );
    subsets(&e);
}

#[test]
fn five_body_upper_preserves_resolution_move_and_report_aliases() {
    // RF-ZERO-SYM: five nodes and tagged B_upper=5, not an exact body claim.
    // Literal independent reference values exercise the first RES capacity grow.
    let i = KernelInput {
        nodes: 5,
        members: 4,
        axis_springs: 0,
        directional_springs: 0,
        constraints: 12,
        load_terms: 2,
        stations: 4,
        load_id_bytes: 4,
        max_load_id_bytes: 2,
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 30,
            free_dofs: 18,
            quantities: 119,
            source_encoding_bytes: 752,
            pattern_entries: 468,
            profile_entries: 135,
        }),
    };
    let e = evaluate(i, PopulationPolicy::NodesAndFreeDofsUpper);
    assert_eq!(e.full.retained_prefix, 1089926);
    assert_eq!(e.selected128.retained_prefix, 319654);
    assert_eq!(e.full.solve, bytes(20023653, 21120357));
    assert_eq!(e.selected128.solve, bytes(19150389, 20247093));
    assert_eq!(e.full.returned.union, 1130506);
    assert_eq!(e.selected128.returned.union, 349866);
    for (p, top, hat, report, report_move) in [
        (256, 1097686, 1097334, 1167932, 1168956),
        (512, 1102774, 1102102, 1216412, 1217948),
        (1024, 1112950, 1111638, 1313372, 1315932),
    ] {
        assert_eq!(
            phase(&e.full, KernelPhase::ResolutionTop, p),
            bytes(top, top + 64)
        );
        assert_eq!(
            phase(&e.full, KernelPhase::ResolutionHatCheck, p),
            bytes(hat, hat + 64)
        );
        assert_eq!(
            phase(&e.full, KernelPhase::ReportBuild, p),
            bytes(report, report_move)
        );
    }
    subsets(&e);
}

fn chain_input(h: bool) -> KernelInput {
    KernelInput {
        nodes: 11,
        members: 10,
        axis_springs: 0,
        directional_springs: 0,
        constraints: 6,
        load_terms: 6,
        stations: 10,
        load_id_bytes: if h { 30 } else { 12 },
        max_load_id_bytes: if h { 5 } else { 2 },
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 66,
            free_dofs: 60,
            quantities: 263,
            source_encoding_bytes: if h { 1512 } else { 1494 },
            pattern_entries: 1116,
            profile_entries: 534,
        }),
    }
}
#[test]
fn h_and_vr_construction_classes_and_tagged_population_policy() {
    let profile = ReferenceKernelProfile::source40129_rust1971_aarch64_v1();
    let exact = PopulationPolicy::Exact {
        bodies: 1,
        free_blocks: 1,
    };
    let hd = KernelDescriptor::new(chain_input(true), SourceConstruction::HModelV1, exact).unwrap();
    let h = kernel_envelope(&hd, &profile).unwrap();
    assert_eq!(
        (h.base, h.original_source, h.prepared_source_clone),
        (77660, 2800, 2770)
    );
    assert_eq!(h.full.solve, bytes(21652923, 22734267));
    assert_eq!(h.selected128.solve, bytes(19648251, 20729595));
    let vr = evaluate(chain_input(false), exact);
    assert_eq!(vr.original_source, 3488);
    assert_eq!(vr.full.solve, bytes(21653557, 22734901));
    // Exact B=1: the first resolution allocation carries no old grow request.
    for (p, top, hat) in [
        (256, 2649846, 2649814),
        (512, 2660438, 2660342),
        (1024, 2681622, 2681398),
    ] {
        assert_eq!(
            phase(&vr.full, KernelPhase::ResolutionTop, p),
            bytes(top, top)
        );
        assert_eq!(
            phase(&vr.full, KernelPhase::ResolutionHatCheck, p),
            bytes(hat, hat)
        );
    }
    let upper = evaluate(chain_input(false), PopulationPolicy::NodesAndFreeDofsUpper);
    assert!(upper.base >= vr.base);
    for a in vr.full.phases() {
        let b = upper.full.phases().iter().find(|b| b.id == a.id).unwrap();
        assert!(b.bytes.requested >= a.bytes.requested && b.bytes.moving >= a.bytes.moving);
    }
    subsets(&h);
    subsets(&vr);
    subsets(&upper);
}

#[test]
fn missing_invalid_and_unproved_inputs_return_errors_not_totals() {
    let policy = PopulationPolicy::NodesAndFreeDofsUpper;
    let make = |i| KernelDescriptor::new(i, SourceConstruction::VrModelV1, policy);
    let mut i = axis_input();
    i.structure = None;
    assert_eq!(
        make(i),
        Err(EnvelopeError::MissingDescriptor("structural counts"))
    );
    let mut i = axis_input();
    i.structure.as_mut().unwrap().quantities += 1;
    assert_eq!(
        make(i),
        Err(EnvelopeError::InvalidDescriptor(
            "inconsistent derived counts"
        ))
    );
    let mut i = axis_input();
    i.structure.as_mut().unwrap().profile_entries = 1000;
    assert_eq!(
        make(i),
        Err(EnvelopeError::InvalidDescriptor(
            "pattern/profile population"
        ))
    );
    let mut i = axis_input();
    i.support_groups = 1;
    assert_eq!(
        make(i),
        Err(EnvelopeError::MissingProof("aggregate support children"))
    );
    let mut i = axis_input();
    i.nonzero_prescribed_terms = 1;
    assert_eq!(
        make(i),
        Err(EnvelopeError::MissingProof("nonzero prescribed operands"))
    );
    assert_eq!(
        KernelDescriptor::new(
            axis_input(),
            SourceConstruction::VrModelV1,
            PopulationPolicy::Exact {
                bodies: 0,
                free_blocks: 1
            }
        ),
        Err(EnvelopeError::InvalidDescriptor("body/block counts"))
    );
    assert_eq!(
        KernelDescriptor::new(axis_input(), SourceConstruction::HModelV1, policy),
        Err(EnvelopeError::InvalidDescriptor("H construction premise"))
    );
}
#[test]
fn maximum_id_length_must_fit_the_retained_nonempty_ids() {
    let policy = PopulationPolicy::NodesAndFreeDofsUpper;
    let mut vr = axis_input();
    vr.load_terms = 2;
    vr.structure.as_mut().unwrap().source_encoding_bytes += 17;
    // Two nonempty IDs cannot total two bytes if one has length two.
    assert_eq!(
        KernelDescriptor::new(vr, SourceConstruction::VrModelV1, policy),
        Err(EnvelopeError::InvalidDescriptor("load ID byte counts"))
    );
    let mut h = chain_input(true);
    h.load_id_bytes = 24;
    h.structure.as_mut().unwrap().source_encoding_bytes -= 6;
    // Six k6:decimal IDs have length >=4; one length-five ID needs >=25 bytes.
    assert_eq!(
        KernelDescriptor::new(h, SourceConstruction::HModelV1, policy),
        Err(EnvelopeError::InvalidDescriptor("H construction premise"))
    );
    h.load_id_bytes = 25;
    h.structure.as_mut().unwrap().source_encoding_bytes += 1;
    assert!(KernelDescriptor::new(h, SourceConstruction::HModelV1, policy).is_ok());
}

#[test]
fn count_and_encoding_overflow_fail_closed() {
    let mut i = axis_input();
    i.nodes = u128::MAX;
    assert_eq!(
        KernelDescriptor::new(
            i,
            SourceConstruction::VrModelV1,
            PopulationPolicy::NodesAndFreeDofsUpper
        ),
        Err(EnvelopeError::ArithmeticOverflow)
    );
    let mut i = axis_input();
    i.load_id_bytes = u128::MAX;
    assert_eq!(
        KernelDescriptor::new(
            i,
            SourceConstruction::VrModelV1,
            PopulationPolicy::NodesAndFreeDofsUpper
        ),
        Err(EnvelopeError::ArithmeticOverflow)
    );
}

#[test]
fn reference_request_width_failure_returns_no_envelope() {
    // Synthetic scalar-width boundary, not a generated model or a claimed graph.
    // Construction validates the numeric relationships; provenance remains external.
    let count = u128::from(u32::MAX);
    let i = KernelInput {
        nodes: count,
        members: count,
        axis_springs: 0,
        directional_springs: 0,
        constraints: 0,
        load_terms: 0,
        stations: 0,
        load_id_bytes: 0,
        max_load_id_bytes: 0,
        support_groups: 0,
        nonzero_prescribed_terms: 0,
        structure: Some(StructuralCounts {
            dofs: 6 * count,
            free_dofs: 6 * count,
            quantities: 19 * count,
            source_encoding_bytes: 38 + 108 * count,
            pattern_entries: 144 * count,
            profile_entries: u128::from(u64::MAX),
        }),
    };
    let d = KernelDescriptor::new(
        i,
        SourceConstruction::VrModelV1,
        PopulationPolicy::NodesAndFreeDofsUpper,
    )
    .unwrap();
    assert_eq!(
        kernel_envelope(
            &d,
            &ReferenceKernelProfile::source40129_rust1971_aarch64_v1()
        ),
        Err(EnvelopeError::ReferenceWidthExceeded),
    );
}
