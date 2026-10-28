//! Repaired product rows and the unified member-relative evidence.
use super::*;
use open_pipe_stress_frame_kernel::structural::{
    certified_group, certified_magnitude, certified_pressure_area_product, certify_arc_recovery,
    certify_straight_recovery, CertifiedMemberLoads, CertifiedMemberRecovery, CertifiedRow,
    CertifiedValue, FormationSource, RecoveryFailure,
};

pub(super) fn source<'a>(
    built: &BuiltModel,
    force: &'a AssembledForce,
    springs: &[SpringEntry],
) -> FormationSource<'a> {
    FormationSource {
        node_count: built.nodes.len(),
        loads: force.load_primitives().into(),
        frames: built.frame_elements.clone(),
        // T4-U8 (DEL-05-03 Design §4 item 2): each frame's member identity.
        frame_members: super::frame_member_identities(built),
        connectors: built.connectors.clone(),
        curved: built
            .curved_bend_elements
            .iter()
            .map(curved_formation_of)
            .collect(),
        springs: springs
            .iter()
            .map(|spring| (spring.node_dof.global_index(), spring.stiffness.value))
            .collect(),
        unavailable: Vec::new(),
    }
}
/// `source` is the case's certified formation source (`source`): a straight
/// member's recovery frame is bound to its own member's frame there (T4-U8,
/// DEL-05-03 Design §4 item 2), whether or not the member has a load primitive.
pub(super) fn member(
    pipe: &StraightPipeElement,
    bend: Option<&CurvedBendMacroBuild>,
    force: &AssembledForce,
    source: &FormationSource<'_>,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let prefix = super::member_prefix(&pipe.element_id);
    let loads = CertifiedMemberLoads::from_member_primitives(
        pipe.node_i.index,
        pipe.node_j.index,
        force.load_primitives(),
        &prefix,
    )?
    .with_member_source(source, &prefix);
    if let Some(bend) = bend {
        certify_arc_recovery(
            &curved_formation_of(bend),
            &loads,
            u,
            bound,
        )
    } else {
        certify_straight_recovery(
            &pipe.frame_element().map_err(|_| {
                RecoveryFailure::Wide(
                    open_pipe_stress_frame_kernel::structural::retained_api::WideError::CountRange(
                        "product member frame",
                    ),
                )
            })?,
            &loads,
            u,
            bound,
        )
    }
}
/// The current straight pressure model's wall increment is 2nu*pAi;
/// its effective increment is (2nu-1)*pAi. Coefficients are formed at p,
/// with Ai on the pressure bore (D-5).
pub(super) fn pressure_adjustment(
    recovery: &CertifiedMemberRecovery,
    state: &pressure_runtime::ExactPressurePipeState,
    wall: bool,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let fluid = certified_pressure_area_product(
        state.pressure.pressure_pa(),
        state.sections.outside_diameter_m(),
        state.sections.pressure_wall_m(),
    )?;
    let two_nu = CertifiedValue::exact(2.0)?
        .product(CertifiedValue::exact(state.material.poisson_ratio())?)?;
    let coefficient = if wall {
        two_nu
    } else {
        two_nu.add(CertifiedValue::exact(-1.0)?)?
    };
    let increment = fluid.product(coefficient)?;
    let mut out = recovery.clone();
    out.ends[0] = out.ends[0].add_value(increment.product(CertifiedValue::exact(-1.0)?)?)?;
    out.ends[6] = out.ends[6].add_value(increment)?;
    for section in &mut out.sections {
        section[0] = section[0].add_value(increment)?;
    }
    Ok(out)
}
/// T4-U2 (H-2) on a realized arc: the wall force N_w = N_el + pAi at both
/// end cuts and the three stations, with pAi formed at p and each row
/// rounded once (K-D5 Design section 3.2, pipe-wall rows); S = N_el is the
/// elastic recovery itself. The wall end actions lie along the arc's end
/// tangents: -N_w(cut_i) at end i and +N_w(cut_j) at end j (exact negation),
/// so the wall end groups judge the wall action while the elastic end groups
/// judge the separately published elastic action in the same tangent frame.
pub(super) fn arc_pressure_wall(
    recovery: &CertifiedMemberRecovery,
    state: &pressure_runtime::ExactPressurePipeState,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let fluid = certified_pressure_area_product(
        state.pressure.pressure_pa(),
        state.sections.outside_diameter_m(),
        state.sections.pressure_wall_m(),
    )?;
    let mut out = recovery.clone();
    for section in &mut out.sections {
        section[0] = section[0].add_value(fluid)?;
    }
    out.ends[0] = out.sections[0][0].scaled(-1.0)?;
    out.ends[6] = out.sections[4][0];
    Ok(out)
}
fn group(
    location: &str,
    kind: &str,
    moment: bool,
    rows: &[CertifiedRow],
    magnitude_published: bool,
) -> Result<formation_guard::RecoveryGroup, RecoveryFailure> {
    let value = certified_group(rows)?;
    Ok(formation_guard::RecoveryGroup {
        location: location.into(),
        kind: kind.into(),
        moment,
        q: value.q,
        zero_compatible: formation_guard::notice_zero_compatible(
            rows.iter().map(|row| row.published),
            value.bound,
        ),
        q_hat: value.q_hat,
        bound: Ok(value.bound),
        absolute_scale: value.absolute_scale,
        published_scale: if magnitude_published {
            certified_magnitude(rows)?.published
        } else {
            rows.iter()
                .map(|row| row.published.abs())
                .fold(0.0, f64::max)
        },
    })
}
pub(super) fn member_record(
    member: &str,
    body: Option<usize>,
    effective: &CertifiedMemberRecovery,
    wall: Option<&CertifiedMemberRecovery>,
) -> Result<formation_guard::RecoveryRecord, RecoveryFailure> {
    let mut groups = Vec::new();
    let locations = [
        ("end_i", &effective.ends[..6]),
        ("end_j", &effective.ends[6..]),
    ];
    for (location, rows) in locations.into_iter().chain(
        ["cut_i", "quarter_1", "midspan", "quarter_3", "cut_j"]
            .iter()
            .zip(effective.sections.iter())
            .map(|(l, r)| (*l, r.as_slice())),
    ) {
        groups.push(group(location, "axial", false, &rows[0..1], false)?);
        groups.push(group(location, "shear", false, &rows[1..3], false)?);
        groups.push(group(location, "torsion", true, &rows[3..4], false)?);
        groups.push(group(location, "bending", true, &rows[4..6], false)?);
    }
    if let Some(wall) = wall {
        for (location, rows) in [
            ("wall_end_i", &wall.ends[..6]),
            ("wall_end_j", &wall.ends[6..]),
        ]
        .into_iter()
        .chain(
            [
                "wall_cut_i",
                "wall_quarter_1",
                "wall_midspan",
                "wall_quarter_3",
                "wall_cut_j",
            ]
            .iter()
            .zip(wall.sections.iter())
            .map(|(l, r)| (*l, r.as_slice())),
        ) {
            groups.push(group(location, "pipe_wall", false, &rows[..1], false)?);
        }
    }
    Ok(formation_guard::RecoveryRecord {
        member: member.into(),
        body,
        groups,
    })
}

/// The separately published scalar norm has its own sqrt and rounding
/// terms, and keeps its own criterion and floor operands.
fn magnitude_group(
    location: &str,
    kind: &str,
    moment: bool,
    rows: &[CertifiedRow],
) -> Result<formation_guard::RecoveryGroup, RecoveryFailure> {
    let value = open_pipe_stress_frame_kernel::structural::CertifiedGroup::magnitude(rows)?;
    Ok(formation_guard::RecoveryGroup {
        location: location.into(),
        kind: kind.into(),
        moment,
        q: value.q,
        zero_compatible: formation_guard::notice_zero_compatible([value.q], value.bound),
        q_hat: value.q_hat,
        bound: Ok(value.bound),
        absolute_scale: value.absolute_scale,
        published_scale: value.q,
    })
}

pub(super) fn support_record(
    support: &str,
    body: Option<usize>,
    rows: &[CertifiedRow; 6],
) -> Result<formation_guard::RecoveryRecord, RecoveryFailure> {
    Ok(formation_guard::RecoveryRecord {
        member: support.into(),
        body,
        groups: vec![
            group("node", "support_force", false, &rows[..3], true)?,
            group("node", "support_moment", true, &rows[3..], true)?,
            magnitude_group("node", "support_force_magnitude", false, &rows[..3])?,
            magnitude_group("node", "support_moment_magnitude", true, &rows[3..])?,
        ],
    })
}
pub(super) fn unavailable_record(
    entity: &str,
    body: Option<usize>,
    reason: String,
) -> formation_guard::RecoveryRecord {
    formation_guard::RecoveryRecord {
        member: entity.into(),
        body,
        groups: vec![formation_guard::RecoveryGroup {
            location: "recovery".into(),
            kind: "all".into(),
            moment: false,
            q: 0.0,
            zero_compatible: false,
            q_hat: f64::INFINITY,
            bound: Err(reason),
            absolute_scale: 0.0,
            published_scale: 0.0,
        }],
    }
}
pub(super) fn stress(
    rows: &[CertifiedRow; 6],
    section: &DerivedSection,
) -> Result<open_pipe_stress_stress_recovery::StressRecoveryResult, RecoveryFailure> {
    let mut out = recover_section_stress(&rows.map(|row| row.published), section);
    out.components.axial_normal = Some(rows[0].divided(section.area)?.published);
    out.components.bending_normal_y = Some(rows[4].divided(section.section_modulus)?.published);
    out.components.bending_normal_z = Some(rows[5].divided(section.section_modulus)?.published);
    out.components.torsional_shear = Some(
        rows[3]
            .scaled(section.torsion_radius)?
            .divided(section.torsion_constant)?
            .published,
    );
    Ok(out)
}
pub(super) fn summary_row(
    rows: &[CertifiedRow; 6],
    section: &DerivedSection,
) -> Result<CertifiedRow, RecoveryFailure> {
    let axial = rows[0]
        .scaled(if rows[0].published.is_sign_negative() {
            -1.0
        } else {
            1.0
        })?
        .divided(section.area)?;
    // This is the established open-formula summary: the maximum of the
    // eight signed component sums, |N|/A + |My|/Z + |Mz|/Z. The circular
    // section's hypot-based governing maximum is a separate quantity.
    let absolute_bending = |row: CertifiedRow| {
        row.scaled(if row.published.is_sign_negative() {
            -1.0
        } else {
            1.0
        })?
        .divided(section.section_modulus)
    };
    let bending_y = absolute_bending(rows[4])?;
    let bending_z = absolute_bending(rows[5])?;
    // Retain the at-p midpoint through both sums and divide by the exact
    // catalog scale before publishing, even when the Pa sum exceeds MAX.
    axial
        .sum_divided(bending_y, 1_000_000.0)?
        .sum_divided(bending_z.divided(1_000_000.0)?, 1.0)
}

pub(super) fn support(
    model: &PreviewModel,
    support: &PreviewSupport,
    built: &BuiltModel,
    springs: &[SpringEntry],
    spring_ground: &[f64],
    source: &FormationSource<'_>,
    force: &AssembledForce,
    u: &[f64],
    bound: &MemberBoundInputs,
    nonlinear: Option<&HashMap<String, open_pipe_stress_nonlinear_integration::confirmation::ConfirmedRow>>,
) -> Result<[CertifiedRow; 6], RecoveryFailure> {
    use open_pipe_stress_frame_kernel::structural::{
        certify_spring_reaction, certify_support_reaction,
    };
    let mut rows = [CertifiedRow::ZERO; 6];
    // DEL-04-01 kd5-nonlinear-active-set.md §3.1: a nonlinear support's slot
    // is its confirmed row; the other five components stay exact +0.
    if let Some(nonlinear_support) = built
        .nonlinear_supports
        .iter()
        .find(|item| item.support_id == support.id)
    {
        let row = nonlinear
            .and_then(|rows| rows.get(&support.id))
            .ok_or(RecoveryFailure::Wide(
                open_pipe_stress_frame_kernel::structural::retained_api::WideError::CountRange(
                    "nonlinear support row",
                ),
            ))?;
        let slot = dof_index(nonlinear_support.dof);
        if slot >= 6 || row.global_dof % 6 != slot {
            return Err(RecoveryFailure::Wide(
                open_pipe_stress_frame_kernel::structural::retained_api::WideError::CountRange(
                    "nonlinear support slot",
                ),
            ));
        }
        rows[slot] = row.row;
    }
    if is_constant_effort_support(support) {
        if let Ok(application) = classify_constant_effort_consumption(model, support) {
            rows[dof_index(application.dof)] =
                CertifiedRow::ZERO.add_value(CertifiedValue::exact(application.force_newtons)?)?;
        }
        return Ok(rows);
    }
    if let Some(linear) = built
        .supports
        .iter()
        .find(|item| item.support_id == support.id)
    {
        for dof in &linear.restrained_dofs {
            let slot = dof_index(*dof);
            rows[slot] =
                certify_support_reaction(source, force, 6 * linear.node_index + slot, u, bound)?;
        }
    }
    // W-16 R-c: -k·(u + w + w₂ - g) for a moved base, the exact +k·g in the
    // row's accumulator; an unmoved base (g = ±0) is the fixed-base row.
    for (spring, &ground) in springs
        .iter()
        .zip(spring_ground)
        .filter(|(spring, _)| spring.support_id == support.id)
    {
        let dof = spring.node_dof.global_index();
        rows[dof % 6] = certify_spring_reaction(dof, spring.stiffness.value, ground, u, bound)?;
    }
    Ok(rows)
}

pub(super) fn connector_record(
    entity: &str,
    body: Option<usize>,
    certificate: &open_pipe_stress_frame_kernel::structural::CertifiedConnectorRecovery,
    effective_axial: Option<&CertifiedRow>,
    tie_rods: &[(&str, CertifiedRow)],
) -> Result<formation_guard::RecoveryRecord, RecoveryFailure> {
    let mut groups = Vec::new();
    for (location, rows) in [
        ("end_i", &certificate.ends[..6]),
        ("end_j", &certificate.ends[6..]),
        ("connector_local", &certificate.g[..]),
    ] {
        groups.push(group(location, "force", false, &rows[..3], false)?);
        groups.push(group(location, "moment", true, &rows[3..], false)?);
    }
    if let Some(row) = effective_axial {
        groups.push(group("connector_local", "effective_axial_force", false, std::slice::from_ref(row), false)?);
    }
    // T4-U5b: each tie rod's certified force, a published force row.
    for (rod, row) in tie_rods {
        groups.push(group(&format!("tie_rod:{rod}"), "tie_rod_force", false, std::slice::from_ref(row), false)?);
    }
    Ok(formation_guard::RecoveryRecord {
        member: entity.into(),
        body,
        groups,
    })
}

pub(super) fn summary(
    rows: &[CertifiedRow; 6],
    section: &DerivedSection,
) -> Result<f64, RecoveryFailure> {
    Ok(summary_row(rows, section)?.published)
}

/// Final public MPa conversion is performed on the retained midpoint.
/// The binary64 Pa value is never an operand of this conversion.
pub(super) fn stress_mpa(
    rows: &[CertifiedRow; 6],
    section: &DerivedSection,
) -> Result<StressComponents, RecoveryFailure> {
    Ok(StressComponents {
        axial_normal: Some(rows[0].divided(section.area)?.divided(1e6)?.published),
        bending_normal_y: Some(
            rows[4]
                .divided(section.section_modulus)?
                .divided(1e6)?
                .published,
        ),
        bending_normal_z: Some(
            rows[5]
                .divided(section.section_modulus)?
                .divided(1e6)?
                .published,
        ),
        torsional_shear: Some(
            rows[3]
                .scaled(section.torsion_radius)?
                .divided(section.torsion_constant)?
                .divided(1e6)?
                .published,
        ),
    })
}

pub(super) fn append_stress_mpa(
    results: &mut Vec<ResultItem>,
    pipe_id: &str,
    location: &str,
    rows: &[CertifiedRow; 6],
    section: &DerivedSection,
    endpoint: bool,
    basis: &str,
    sign: Option<&str>,
) -> Result<(), RecoveryFailure> {
    let values = stress_mpa(rows, section)?;
    let suffix = stable_suffix(pipe_id);
    let located = if endpoint {
        endpoint_id_location(location)
    } else {
        station_id_location(location)
    };
    for (tail, kind, component, value, label) in [
        (
            "axial-normal",
            "element_local_axial_normal_stress",
            "axial_normal_stress",
            values.axial_normal,
            "axial resultant",
        ),
        (
            "bending-normal-y",
            "element_local_bending_normal_stress_y",
            "bending_normal_stress_y",
            values.bending_normal_y,
            "y bending resultant",
        ),
        (
            "bending-normal-z",
            "element_local_bending_normal_stress_z",
            "bending_normal_stress_z",
            values.bending_normal_z,
            "z bending resultant",
        ),
        (
            "torsional-shear",
            "element_local_torsional_shear_stress",
            "torsional_shear_stress",
            values.torsional_shear,
            "torsional resultant",
        ),
    ] {
        let sign_convention=sign.map(str::to_string).unwrap_or_else(||format!("positive {} stress follows the section-equilibrium element-local {label} at this station; j-side section action in the element-local frame (x toward end j); section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction",if tail=="torsional-shear" {"torsional shear"} else if tail=="axial-normal" {"normal"} else {"bending normal"}));
        results.push(ResultItem { component_body: None,
            id: format!("result:stress:{suffix}:{located}:{tail}"),
            kind: kind.into(),
            value: value.expect("complete certified stress"),
            unit: "MPa".into(),
            entity_ref: pipe_id.into(),
            basis_ref: None,
            source_result_refs: Vec::new(),
            metadata: Some(ResultMetadata {
                component: component.into(),
                coordinate_system: "element_local".into(),
                location: location.into(),
                basis: basis.into(),
                sign_convention,
            }),
        });
    }
    Ok(())
}

/// The engine's local witness upper bound is weaker than its global
/// maximum upper bound. Publish the latter, while requiring it to contain
/// the independently recovered public stress samples as well.
pub(super) fn maximum_cover(
    maximum: open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum,
    member: &open_pipe_stress_frame_kernel::structural::CertifiedMemberRecovery,
    section: &DerivedSection,
) -> Result<open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum, String> {
    for (station, rows) in member.sections.iter().enumerate() {
        let stress = stress_mpa(rows, section)
            .map_err(|error| format!("certified station stress: {error:?}"))?;
        for value in [
            stress.axial_normal,
            stress.bending_normal_y,
            stress.bending_normal_z,
        ]
        .into_iter()
        .flatten()
        {
            let sample = public_mpa_magnitude_in_pa_upper(value)?;
            if !sample.is_finite() || sample > maximum.upper_bound {
                return Err(format!("global maximum enclosure does not contain published normal-stress sample: station={station}, sample_pa={sample:.17e}, global_upper_pa={:.17e}",maximum.upper_bound));
            }
        }
    }
    Ok(maximum)
}

/// Exact unit conversion of the public binary64 value, rounded upward only
/// when its correctly rounded product lies below the exact product.
fn public_mpa_magnitude_in_pa_upper(value: f64) -> Result<f64, String> {
    let mut exact = ExactAccumulator::new();
    exact
        .add_product(value.abs(), 1e6)
        .map_err(|_| "published stress unit conversion range".to_string())?;
    let rounded = exact
        .round()
        .map_err(|_| "published stress unit conversion range".to_string())?;
    exact
        .add(-rounded)
        .map_err(|_| "published stress unit conversion range".to_string())?;
    Ok(if exact.signum() > 0 {
        rounded.next_up()
    } else {
        rounded
    })
}

/// Same-source polynomial enclosures used by both maximum laws. The extra
/// constant axial enclosure transports at-p evaluation and final catalog-unit
/// publication; it is derived by the certificate producer, not from samples.
pub(super) fn straight_stress_spans(
    pipe: &StraightPipeElement,
    force: &AssembledForce,
    source: &FormationSource<'_>,
    u: &[f64],
    bound: &MemberBoundInputs,
    section: &DerivedSection,
    pressure: Option<&pressure_runtime::ExactPressurePipeState>,
) -> Result<
    Vec<open_pipe_stress_stress_recovery::elastic_extrema::EnclosedQuadraticStressSpan>,
    String,
> {
    use open_pipe_stress_frame_kernel::structural::certify_straight_stress_spans;
    use open_pipe_stress_stress_recovery::elastic_extrema::{
        EnclosedQuadraticStressSpan, StressCoefficientBounds,
    };
    let prefix = super::member_prefix(&pipe.element_id);
    let loads = CertifiedMemberLoads::from_member_primitives(
        pipe.node_i.index,
        pipe.node_j.index,
        force.load_primitives(),
        &prefix,
    )
    .map_err(|error| format!("maximum member loads: {error:?}"))?
    .with_member_source(source, &prefix);
    let frame = pipe
        .frame_element()
        .map_err(|error| format!("maximum physical frame: {error:?}"))?;
    let increment = pressure
        .map(|state| {
            let fluid = certified_pressure_area_product(
                state.pressure.pressure_pa(),
                state.sections.outside_diameter_m(),
                state.sections.pressure_wall_m(),
            )?;
            let coefficient = CertifiedValue::exact(2.0)?
                .product(CertifiedValue::exact(state.material.poisson_ratio())?)?;
            fluid.product(coefficient)
        })
        .transpose()
        .map_err(|error| format!("maximum pressure wall increment: {error:?}"))?;
    let spans = certify_straight_stress_spans(
        &frame,
        &loads,
        u,
        bound,
        section.area,
        section.section_modulus,
        increment,
    )
    .map_err(|error| format!("maximum source coefficient enclosure: {error:?}"))?;
    spans
        .into_iter()
        .map(|span| {
            let mut axial = [StressCoefficientBounds {
                lower: 0.0,
                upper: 0.0,
            }; 3];
            for (target, source) in axial.iter_mut().zip(span.axial) {
                *target = StressCoefficientBounds {
                    lower: directed_transport_sum(
                        source.lower,
                        -span.publication_radius_pa,
                        false,
                    )?,
                    upper: directed_transport_sum(source.upper, span.publication_radius_pa, true)?,
                };
            }
            Ok(EnclosedQuadraticStressSpan {
                start: span.start,
                end: span.end,
                axial,
                bending_y: span.bending_y.map(|coefficient| StressCoefficientBounds {
                    lower: coefficient.lower,
                    upper: coefficient.upper,
                }),
                bending_z: span.bending_z.map(|coefficient| StressCoefficientBounds {
                    lower: coefficient.lower,
                    upper: coefficient.upper,
                }),
            })
        })
        .collect()
}

fn directed_transport_sum(value: f64, transport: f64, upward: bool) -> Result<f64, String> {
    if !value.is_finite() || !transport.is_finite() {
        return Err("maximum publication transport range".into());
    }
    let mut sum = ExactAccumulator::new();
    sum.add(value)
        .map_err(|_| "maximum publication transport range".to_string())?;
    sum.add(transport)
        .map_err(|_| "maximum publication transport range".to_string())?;
    let rounded = sum
        .round()
        .map_err(|_| "maximum publication transport range".to_string())?;
    if !rounded.is_finite() {
        return Err("maximum publication transport range".into());
    }
    sum.add(-rounded)
        .map_err(|_| "maximum publication transport range".to_string())?;
    Ok(if upward && sum.signum() > 0 {
        rounded.next_up()
    } else if !upward && sum.signum() < 0 {
        rounded.next_down()
    } else {
        rounded
    })
}

/// L1 summary conversion retains the midpoint until the exact catalog division.
pub(super) fn maximum_summary_mpa(
    maximum: open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum,
) -> Result<f64, String> {
    let half =
        CertifiedValue::exact(0.5).map_err(|error| format!("summary midpoint: {error:?}"))?;
    CertifiedValue::exact(maximum.value_lower)
        .and_then(|lower| lower.product(half))
        .and_then(|lower| {
            CertifiedValue::exact(maximum.upper_bound)
                .and_then(|upper| upper.product(half))
                .and_then(|upper| lower.add(upper))
        })
        .and_then(|midpoint| midpoint.divided(CertifiedValue::exact(1_000_000.0)?))
        .and_then(|midpoint| midpoint.published())
        .map_err(|error| format!("summary midpoint/unit publication: {error:?}"))
}

pub(super) fn circular_maximum(
    spans: &[open_pipe_stress_stress_recovery::elastic_extrema::EnclosedQuadraticStressSpan],
    member: &CertifiedMemberRecovery,
    section: &DerivedSection,
) -> Result<open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum, String> {
    open_pipe_stress_stress_recovery::elastic_extrema::bound_piecewise_enclosed_elastic_maximum(
        spans,
    )
    .map_err(|error| format!("{error:?}"))
    .and_then(|maximum| maximum_cover(maximum, member, section))
}

pub(super) fn l1_summary(
    spans: &[open_pipe_stress_stress_recovery::elastic_extrema::EnclosedQuadraticStressSpan],
) -> Result<f64, String> {
    open_pipe_stress_stress_recovery::elastic_extrema::bound_piecewise_enclosed_l1_maximum(spans)
        .map_err(|error| format!("{error:?}"))
        .and_then(maximum_summary_mpa)
}
// Each pair bounds an ideal expression over the whole member and the error
// of the actual binary64 expression. Every primitive rounding contributes
// u*(magnitude+propagated_error)+min_subnormal; subsequent products/divisions
// propagate that absolute quantum through their real operands.
#[derive(Clone, Copy)]
struct FallbackError {
    magnitude: f64,
    error: f64,
}
impl FallbackError {
    const ZERO: Self = Self {
        magnitude: 0.0,
        error: 0.0,
    };
    fn checked(magnitude: f64, error: f64) -> Result<Self, String> {
        if magnitude.is_finite() && error.is_finite() && magnitude >= 0.0 && error >= 0.0 {
            Ok(Self { magnitude, error })
        } else {
            Err("fallback polynomial arithmetic transport range".into())
        }
    }
    fn exact(value: f64) -> Result<Self, String> {
        Self::checked(value.abs(), 0.0)
    }
    fn rounded(self) -> Result<Self, String> {
        if self.magnitude == 0.0 && self.error == 0.0 {
            return Ok(Self::ZERO);
        }
        let error = fallback_sum_up(
            self.error,
            fallback_sum_up(
                fallback_product_up(
                    f64::from_bits(970_u64 << 52),
                    fallback_sum_up(self.magnitude, self.error),
                ),
                f64::from_bits(1),
            ),
        );
        Self::checked(self.magnitude, error)
    }
    fn accumulate(self, other: Self) -> Result<Self, String> {
        Self::checked(
            fallback_sum_up(self.magnitude, other.magnitude),
            fallback_sum_up(self.error, other.error),
        )
    }
    fn add(self, other: Self) -> Result<Self, String> {
        self.accumulate(other)?.rounded()
    }
    fn mul(self, other: Self) -> Result<Self, String> {
        if (self.magnitude == 0.0 && self.error == 0.0)
            || (other.magnitude == 0.0 && other.error == 0.0)
        {
            return Ok(Self::ZERO);
        }
        let magnitude = fallback_product_up(self.magnitude, other.magnitude);
        let error = fallback_sum_up(
            fallback_sum_up(
                fallback_product_up(self.magnitude, other.error),
                fallback_product_up(other.magnitude, self.error),
            ),
            fallback_product_up(self.error, other.error),
        );
        Self::checked(magnitude, error)?.rounded()
    }
    fn divided(self, denominator: f64) -> Result<Self, String> {
        if !(denominator > 0.0 && denominator.is_finite()) {
            return Err("fallback polynomial stress denominator".into());
        }
        Self::checked(
            fallback_div_up(self.magnitude, denominator),
            fallback_div_up(self.error, denominator),
        )?
        .rounded()
    }
    fn exact_scaled(parts: (f64, i32)) -> Result<Self, String> {
        Self::checked(fallback_power_up(parts.0.abs(), parts.1)?, 0.0)
    }
    fn divided_scaled(self, parts: (f64, i32)) -> Result<Self, String> {
        if !(parts.0 > 0.0 && parts.0.is_finite()) {
            return Err("fallback scaled stress denominator".into());
        }
        Self::checked(
            fallback_ratio_up(self.magnitude, parts)?,
            fallback_ratio_up(self.error, parts)?,
        )?
        .rounded()
    }

    fn min(self, other: Self) -> Result<Self, String> {
        Self::checked(
            self.magnitude.min(other.magnitude),
            self.error.max(other.error),
        )
    }
    fn max(self, other: Self) -> Result<Self, String> {
        Self::checked(
            self.magnitude.max(other.magnitude),
            self.error.max(other.error),
        )
    }
    // Positive clamp is 1-Lipschitz. The ideal active length is at most L,
    // while the subtraction error already includes a possible branch crossing.
    fn active_length(self, start: Self, length: f64) -> Result<Self, String> {
        let difference = self.add(start)?;
        Self::checked(length, difference.error)
    }
}
fn fallback_power_up(mut value: f64, mut exponent: i32) -> Result<f64, String> {
    while value != 0.0 && exponent != 0 {
        let step = exponent.clamp(-512, 512);
        value = fallback_product_up(value, 2f64.powi(step));
        exponent -= step;
        if !value.is_finite() {
            return Err("fallback scaled constituent range".into());
        }
    }
    Ok(value)
}
fn fallback_ratio_up(value: f64, denominator: (f64, i32)) -> Result<f64, String> {
    if value == 0.0 {
        return Ok(0.0);
    }
    let numerator = pressure_exact::Scaled::from_f64(value).dyadic_parts();
    fallback_power_up(
        fallback_div_up(numerator.0.abs(), denominator.0),
        numerator.1 - denominator.1,
    )
}
fn fallback_denominator_relative_error(
    denominator: (f64, i32),
    rounded: f64,
) -> Result<f64, String> {
    let parts = pressure_exact::Scaled::from_f64(rounded).dyadic_parts();
    if parts == denominator {
        return Ok(0.0);
    }
    // Normalize before division, then outward-round both ratio ends. This
    // preserves the half-min-subnormal difference hidden by a normal output.
    let ratio = parts.0 / denominator.0;
    let shift = parts.1 - denominator.1;
    let upper = fallback_power_up(ratio.next_up(), shift)?;
    let mut lower = ratio.next_down();
    let mut remaining = shift;
    while remaining != 0 {
        let step = remaining.clamp(-512, 512);
        lower = (lower * 2f64.powi(step)).next_down().max(0.0);
        remaining -= step;
    }
    Ok((upper - 1.0).abs().max((lower - 1.0).abs()).next_up())
}

/// The two pressure consumers use distinct area reference laws at a coarse
/// publication boundary. Charge their difference against the entire wall
/// stress, not just the distributed-load slope.
fn fallback_reference_law_error(
    area: (f64, i32),
    published_area: f64,
    generic_magnitude: f64,
) -> Result<f64, String> {
    let relative = fallback_denominator_relative_error(area, published_area)?;
    let error = fallback_product_up(relative, generic_magnitude);
    if error.is_finite() {
        Ok(error)
    } else {
        Err("fallback pressure reference-law transport range".into())
    }
}

/// A local directed positive product. FMA-residual tests alone cannot
/// detect a positive sub-half-min-subnormal residual of a nonzero product.
/// Exact products also preserve tight normal/MAX endpoints without blanket
/// inflation. Failure returns an unavailable positive bound to the caller.
fn fallback_product_up(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    if !a.is_finite() || !b.is_finite() || a < 0.0 || b < 0.0 {
        return f64::INFINITY;
    }
    let mut product = ExactAccumulator::new();
    if product.add_product(a, b).is_err() {
        return f64::INFINITY;
    }
    let Ok(rounded) = product.round() else {
        return f64::INFINITY;
    };
    if !rounded.is_finite() || product.add(-rounded).is_err() {
        return f64::INFINITY;
    }
    if product.signum() > 0 {
        rounded.next_up()
    } else {
        rounded
    }
}

fn fallback_div_up(value: f64, denominator: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else {
        (value / denominator).next_up()
    }
}

fn fallback_sum_up(a: f64, b: f64) -> f64 {
    if a == 0.0 && b == 0.0 {
        0.0
    } else {
        (a + b).next_up()
    }
}

/// Whole-member error replay of the unchanged E4/E6 binary64 section recipe.
/// Exact source leaves are the held end actions, local intensities and L.
fn fallback_section_errors(
    ends: &[f64],
    loads: &[SpannedUniformLocalLoad],
    length: f64,
) -> Result<[FallbackError; 6], String> {
    if ends.len() != 12 {
        return Err("fallback polynomial end-action count".into());
    }
    let l = FallbackError::exact(length)?;
    let distance = l.rounded()?; // x=fraction*L, fraction in [0,1].
    let mut sections = [FallbackError::ZERO; 6];
    for (target, &value) in sections.iter_mut().zip(&ends[..6]) {
        *target = FallbackError::exact(value)?;
    }
    sections[4] = sections[4].accumulate(FallbackError::exact(ends[2])?.mul(distance)?)?;
    sections[5] = sections[5].accumulate(FallbackError::exact(ends[1])?.mul(distance)?)?;
    for load in loads.iter().filter(|load| load.force_per_length != 0.0) {
        let start = l.mul(FallbackError::exact(load.span.start_fraction)?)?;
        let end = l.mul(FallbackError::exact(load.span.end_fraction)?)?;
        // This max bridges the actual early-return inactive branch. Its
        // zero-length lever is zero because both squared endpoints coincide.
        let active_end = distance.min(end)?.max(start)?;
        let active = active_end.active_length(start, length)?;
        let lever = distance.mul(active)?.add(
            active_end
                .mul(active_end)?
                .add(start.mul(start)?)?
                .mul(FallbackError::exact(0.5)?)?,
        )?;
        let q = FallbackError::exact(load.force_per_length)?;
        let axis = match load.direction {
            LocalLoadDirection::X => 0,
            LocalLoadDirection::Y => 1,
            LocalLoadDirection::Z => 2,
        };
        sections[axis] = sections[axis].accumulate(q.mul(active)?)?;
        if axis != 0 {
            let moment = if axis == 1 { 5 } else { 4 };
            sections[moment] = sections[moment].accumulate(q.mul(lever)?)?;
        }
    }
    for (index, row) in sections.iter_mut().enumerate() {
        if index != 3 {
            *row = row.rounded()?;
        }
    }
    Ok(sections)
}

/// Source-expression arithmetic/publication transport for a noncertified
/// fallback. It preserves the supplied f64 coefficients and every prior
/// action/stress/support byte, with a separately derived constant enclosure.
pub(super) fn fallback_circular_maximum(
    spans: &[open_pipe_stress_stress_recovery::elastic_extrema::QuadraticStressSpan],
    ends: &[f64],
    loads: &[SpannedUniformLocalLoad],
    length: f64,
    section: &DerivedSection,
    pressure: Option<(&pressure_runtime::ExactPressurePipeState, pressure_exact::SectionBasis)>,
) -> Result<open_pipe_stress_stress_recovery::elastic_extrema::CertifiedStressMaximum, String> {
    use open_pipe_stress_stress_recovery::elastic_extrema::{
        bound_piecewise_enclosed_elastic_maximum, EnclosedQuadraticStressSpan,
        StressCoefficientBounds,
    };
    if !(length > 0.0 && length.is_finite()) {
        return Err("fallback polynomial length".into());
    }
    let rows = fallback_section_errors(ends, loads, length)?;
    let mut sample = [
        rows[0].divided(section.area)?,
        rows[4].divided(section.section_modulus)?,
        rows[5].divided(section.section_modulus)?,
    ];
    let mut pressure_extra = 0.0;
    if let Some((state, basis)) = pressure {
        // D-5: Ai is the pressure bore's, As the basis's own; the supplied
        // section must be that basis's.
        let (ai, area) = state.sections.area_dyadic_parts(basis);
        if state.sections.basis(basis).wall_area_m2().to_bits() != section.area.to_bits() {
            return Err("fallback scaled section area binding".into());
        }
        let fluid = FallbackError::exact(state.pressure.pressure_pa())?
            .mul(FallbackError::exact_scaled(ai)?)?;
        let coefficient = FallbackError::exact(2.0)?
            .mul(FallbackError::exact(state.material.poisson_ratio())?)?;
        // DIRECT order: mechanical E4 -> wall FMA -> Scaled area division.
        // One extra whole-constituent RN allowance bounds exponent alignment
        // (<=2*min_subnormal*S) alongside the mul/add/FMA rounding. Exact
        // power normalization contributes no rounding. The source area is
        // its actual dyadic; a normal getter is never presumed exact.
        let wall = fluid.mul(coefficient)?.add(rows[0])?.rounded()?;
        sample[0] = wall.divided_scaled(area)?;
        // GENERIC order: endpoint FMA -> wall publication -> E4 -> A div.
        // The extra post-wall E4 allowance is explicitly charged against the
        // pressure-inclusive whole-member scale, not the mechanical scale.
        let generic = wall.rounded()?.rounded()?.divided(section.area)?;
        pressure_extra = fallback_sum_up(
            generic.error,
            fallback_reference_law_error(area, section.area, generic.magnitude)?,
        );
        // Original slope coefficients divide by the published area, while
        // direct membrane publication divides by its exact stored dyadic.
        let relative = fallback_denominator_relative_error(area, section.area)?;
        let mut uniform_axial = FallbackError::ZERO;
        for load in loads
            .iter()
            .filter(|load| load.direction == LocalLoadDirection::X)
        {
            uniform_axial = uniform_axial.accumulate(
                FallbackError::exact(load.force_per_length)?.mul(FallbackError::exact(length)?)?,
            )?;
        }
        pressure_extra = fallback_sum_up(
            pressure_extra,
            fallback_product_up(relative, uniform_axial.divided(section.area)?.magnitude),
        );
    }
    let mut sample_error = 0.0;
    for value in sample {
        sample_error = fallback_sum_up(sample_error, value.error);
    }
    let mut enclosures = Vec::with_capacity(spans.len());
    for span in spans {
        let h = FallbackError::exact(span.end)?
            .add(FallbackError::exact(span.start)?)?
            .mul(FallbackError::exact(length)?)?;
        let mut intensity = [FallbackError::ZERO; 3];
        for load in loads.iter().filter(|load| {
            load.span.start_fraction <= span.start && load.span.end_fraction >= span.end
        }) {
            let axis = match load.direction {
                LocalLoadDirection::X => 0,
                LocalLoadDirection::Y => 1,
                LocalLoadDirection::Z => 2,
            };
            intensity[axis] =
                intensity[axis].accumulate(FallbackError::exact(load.force_per_length)?)?;
        }
        for value in &mut intensity {
            *value = value.rounded()?;
        }
        let powers = [
            [
                sample[0],
                intensity[0].mul(h)?.divided(section.area)?,
                FallbackError::ZERO,
            ],
            [
                sample[1],
                rows[2].mul(h)?.divided(section.section_modulus)?,
                FallbackError::exact(0.5)?
                    .mul(intensity[2])?
                    .mul(h)?
                    .mul(h)?
                    .divided(section.section_modulus)?,
            ],
            [
                sample[2],
                rows[1].mul(h)?.divided(section.section_modulus)?,
                FallbackError::exact(0.5)?
                    .mul(intensity[1])?
                    .mul(h)?
                    .mul(h)?
                    .divided(section.section_modulus)?,
            ],
        ];
        let mut coefficient_error = 0.0;
        for component in powers {
            let controls = [
                component[0],
                component[0].add(FallbackError::exact(0.5)?.mul(component[1])?)?,
                component[0].add(component[1])?.add(component[2])?,
            ];
            coefficient_error = fallback_sum_up(
                coefficient_error,
                controls
                    .into_iter()
                    .map(|value| value.error)
                    .fold(0.0, f64::max),
            );
        }
        let mut unit_error = 0.0;
        for value in sample {
            let converted = value.divided(1_000_000.0)?;
            unit_error = fallback_sum_up(
                unit_error,
                fallback_product_up(converted.error, 1_000_000.0),
            );
        }
        // The strict comparison uses exact catalog conversion of the public
        // MPa value. Pa floating conversion/nextUp is not part of the bound.
        let transport = fallback_sum_up(
            coefficient_error,
            fallback_sum_up(fallback_sum_up(sample_error, unit_error), pressure_extra),
        );
        if !transport.is_finite() {
            return Err("fallback polynomial arithmetic transport range".into());
        }
        let mut axial = [StressCoefficientBounds {
            lower: 0.0,
            upper: 0.0,
        }; 3];
        for (target, &value) in axial.iter_mut().zip(&span.axial) {
            *target = StressCoefficientBounds {
                lower: directed_transport_sum(value, -transport, false)?,
                upper: directed_transport_sum(value, transport, true)?,
            };
        }
        enclosures.push(EnclosedQuadraticStressSpan {
            start: span.start,
            end: span.end,
            axial,
            bending_y: span.bending_y.map(StressCoefficientBounds::exact),
            bending_z: span.bending_z.map(StressCoefficientBounds::exact),
        });
    }
    bound_piecewise_enclosed_elastic_maximum(&enclosures).map_err(|error| format!("{error:?}"))
}

#[cfg(test)]
mod fallback_transport_tests {
    use super::*;
    #[test]
    fn actual_scaled_normal_boundary_and_subnormal_constituents_are_enclosed() {
        let boundary = pressure_exact::Scaled::from_f64(1.0_f64.next_down())
            .mul(pressure_exact::Scaled::from_f64(f64::MIN_POSITIVE));
        let parts = boundary.dyadic_parts();
        assert_eq!(boundary.to_f64(), f64::MIN_POSITIVE);
        assert_eq!(parts, (1.0_f64.next_down(), -1022));
        // Exact cross-multiplication: relative*m >=1-m, proving that the
        // real half-min-subnormal difference hidden by to_f64 is enclosed.
        let relative = fallback_denominator_relative_error(parts, boundary.to_f64()).unwrap();
        let mut proof = ExactAccumulator::new();
        proof.add_product(relative, parts.0).unwrap();
        proof.add(parts.0).unwrap();
        proof.add(-1.0).unwrap();
        assert!(proof.signum() >= 0);
        assert!(relative > 0.0);
        let subnormal = pressure_exact::Scaled::from_f64(f64::from_bits(1))
            .mul(pressure_exact::Scaled::from_f64(0.75));
        let actual = pressure_exact::Scaled::from_f64(f64::from_bits(1))
            .div(subnormal)
            .unwrap()
            .to_f64();
        let envelope = FallbackError::exact(f64::from_bits(1))
            .unwrap()
            .divided_scaled(subnormal.dyadic_parts())
            .unwrap();
        assert_eq!(actual.to_bits(), (4.0_f64 / 3.0).to_bits());
        // Independent rational division: actual±E contains4/3. Compare the
        // squared-free dyadic products exactly, without a tolerance.
        for sign in [-1.0, 1.0] {
            let mut proof = ExactAccumulator::new();
            proof.add_product(3.0, envelope.error).unwrap();
            proof.add_product(sign * 3.0, actual).unwrap();
            proof.add(-sign * 4.0).unwrap();
            assert!(proof.signum() >= 0);
        }
        let one_and_quarter = pressure_exact::Scaled::from_f64(f64::from_bits(1))
            .mul(pressure_exact::Scaled::from_f64(1.25));
        let upper = FallbackError::exact_scaled(one_and_quarter.dyadic_parts())
            .unwrap()
            .magnitude;
        let mut proof = ExactAccumulator::new();
        proof.add(upper).unwrap();
        proof.add_product(-1.25, f64::from_bits(1)).unwrap();
        assert!(proof.signum()>=0,"a nonzero rounded product with a lost sub-half-Tiny residual still requires upward correction");
        let quotient = fallback_ratio_up(f64::from_bits(1), (0.8, 0)).unwrap();
        let mut proof = ExactAccumulator::new();
        proof.add_product(quotient, 0.8).unwrap();
        proof.add(-f64::from_bits(1)).unwrap();
        assert!(
            proof.signum() >= 0,
            "the normalized ratio power scaling is directed too"
        );
        let exact_upper = FallbackError::exact_scaled(subnormal.dyadic_parts())
            .unwrap()
            .magnitude;
        let mut proof = ExactAccumulator::new();
        proof.add(exact_upper).unwrap();
        proof.add_product(-0.75, f64::from_bits(1)).unwrap();
        assert!(proof.signum() >= 0);
    }

    #[test]
    fn pressure_reference_law_transport_binds_whole_wall_at_subnormal_area() {
        let tiny = f64::from_bits(1);
        let area =
            pressure_exact::Scaled::from_f64(tiny).mul(pressure_exact::Scaled::from_f64(1.25));
        let wall = pressure_exact::Scaled::from_f64(1000.0 * tiny);
        let direct = wall.div(area).unwrap().to_f64();
        let generic = wall.to_f64() / area.to_f64();
        assert_eq!(direct, 800.0);
        assert_eq!(generic, 1000.0);
        let transported =
            fallback_reference_law_error(area.dyadic_parts(), area.to_f64(), generic).unwrap();
        assert!(
            transported >= (generic - direct).abs(),
            "the producer must enclose the reference-law difference even with no axial line load"
        );
        let exact_area = pressure_exact::Scaled::from_f64(0.5);
        assert_eq!(
            fallback_reference_law_error(exact_area.dyadic_parts(), exact_area.to_f64(), 1000.0)
                .unwrap(),
            0.0
        );
    }

    #[test]
    fn pressure_fallback_uses_actual_scaled_membrane_and_preserves_affine_loads() {
        // Zero mill tolerance and allowance: the bases and the bore coincide.
        let sections = pressure_exact::ExactSectionBases::derive(0.2, 0.01, 0.0, 0.0).unwrap();
        let annulus = sections.nominal();
        let material = pressure_exact::IsotropicENu::new(200e9, 0.25).unwrap();
        let pressure = pressure_exact::InternalDifferentialPressure::new(1024.0).unwrap();
        let state = pressure_runtime::ExactPressurePipeState {
            region_id: "independent:test".into(),
            sections,
            pressure,
            material,
            arc: None,
            eigenload_pair: [0.0, 0.0],
        };
        let nominal = pressure_exact::SectionBasis::Nominal;
        let membrane = |mechanical: f64| {
            let forces = sections.recover_wall_effective(mechanical, material, pressure).unwrap();
            sections.membrane(nominal, forces).unwrap()
        };
        let area = annulus.wall_area_m2();
        let c0 = membrane(0.125);
        let slope = -0.0625 / area;
        let span = open_pipe_stress_stress_recovery::elastic_extrema::QuadraticStressSpan {
            start: 0.0,
            end: 1.0,
            axial: [c0, c0 + 0.5 * slope, c0 + slope],
            bending_y: [0.0; 3],
            bending_z: [0.0; 3],
        };
        let section = DerivedSection {
            area,
            internal_area: annulus.internal_area_m2(),
            wall_thickness: 0.01,
            second_moment: annulus.second_moment_m4(),
            torsion_constant: annulus.polar_moment_m4(),
            section_modulus: annulus.section_modulus_m3(),
            torsion_radius: annulus.outer_radius_m(),
        };
        let ends = [
            -0.125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0625, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let load =
            SpannedUniformLocalLoad::new(LocalLoadDirection::X, 0.03125, UniformLoadSpan::full())
                .unwrap();
        let maximum =
            fallback_circular_maximum(&[span], &ends, &[load], 2.0, &section, Some((&state, nominal)))
                .unwrap();
        // Exact held-area reference: max sigma=(1/8+512Ai)/As, attained
        // at x=0 because q=1/32 makes N decrease along this two-metre span.
        let mut proof = ExactAccumulator::new();
        proof.add_product(maximum.upper_bound, area).unwrap();
        proof.add(-0.125).unwrap();
        proof
            .add_product(-512.0, annulus.internal_area_m2())
            .unwrap();
        assert!(proof.signum() >= 0);
        assert!(maximum.upper_bound - maximum.value_lower <= 1e-12 + 1e-12 * maximum.value_lower);
        for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let mechanical = 0.125 - 0.0625 * fraction;
            let published = membrane(mechanical);
            assert!(published.abs() <= maximum.upper_bound);
        }
    }

    #[test]
    fn underflow_quantum_is_propagated_through_downstream_source_factor() {
        let tiny = f64::from_bits(1);
        let envelope = FallbackError::exact(tiny)
            .unwrap()
            .mul(FallbackError::exact(0.5).unwrap())
            .unwrap()
            .mul(FallbackError::exact(f64::MAX).unwrap())
            .unwrap();
        let actual = (tiny * 0.5) * f64::MAX;
        // Exact dyadic reference MAX*2^-1075, evaluated by two exact power
        // scalings whose intermediate values stay normal. The first actual
        // product instead rounds the tie to zero.
        let reference = (f64::MAX * f64::MIN_POSITIVE) * 2f64.powi(-53);
        assert_eq!(actual, 0.0);
        assert!(reference > 0.0);
        assert!(
            envelope.error >= reference,
            "a bare unweighted min-subnormal allowance misses this source-factor amplification"
        );
    }
}

#[cfg(test)]
mod u4a_record_tests {
    use super::*;
    #[test]
    fn u4a_end_groups_recompute_from_actual_tangent_row_certificates() {
        use open_pipe_stress_frame_kernel::structural::CurvedFormation;
        let arc=CurvedFormation {node_i:0,node_j:1,coordinates_i:[1.0,0.0,0.0],coordinates_j:[0.0,1.0,0.0],radius:1.0,y_reference:[1.0,1.0,0.0],elastic_modulus:8.0,shear_modulus:4.0,area:2.0,second_moment:3.0,torsion_constant:6.0,in_plane_flexibility_factor:2.0,out_of_plane_flexibility_factor:2.0};
        let bound=MemberBoundInputs {w:vec![2f64.powi(-30);12],w2:vec![-2f64.powi(-45);12],exponents:vec![0;12],lambda_sigma:2f64.powi(-35)};
        let u=[0.0,0.0,0.0,0.0,0.0,0.0,0.25,0.125,-0.03125,0.0625,-0.125,0.25];
        let recovery=certify_arc_recovery(&arc,&CertifiedMemberLoads::default(),&u,&bound).unwrap();
        let record=member_record("arc",Some(0),&recovery,None).unwrap();
        for (end,offset) in [("end_i",0),("end_j",6)] {
            for (kind,range) in [("axial",0..1),("shear",1..3),("torsion",3..4),("bending",4..6)] {
                let rows=&recovery.ends[offset+range.start..offset+range.end];
                assert!(rows.iter().any(|row|row.remainder>0.0));
                let expected=certified_group(rows).unwrap();
                let actual=record.groups.iter().find(|g|g.location==end && g.kind==kind).unwrap();
                assert_eq!(actual.q.to_bits(),expected.q.to_bits());
                assert_eq!(actual.q_hat.to_bits(),expected.q_hat.to_bits());
                assert_eq!(actual.bound.as_ref().unwrap().to_bits(),expected.bound.to_bits());
                assert_eq!(actual.absolute_scale.to_bits(),expected.absolute_scale.to_bits());
                // The unchanged predicate consumes this exact certificate.
                for scale in [1.0,1e15] {
                    assert_eq!(formation_guard::member_group_fires(actual.q,actual.q_hat,*actual.bound.as_ref().unwrap(),scale),formation_guard::member_group_fires(expected.q,expected.q_hat,expected.bound,scale));
                }
            }
        }
    }
}
