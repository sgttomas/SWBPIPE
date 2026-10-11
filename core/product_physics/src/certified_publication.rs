//! Repaired product rows and the unified member-relative evidence.
use super::*;
use open_pipe_stress_frame_kernel::structural::{
    certified_group, certified_magnitude, certified_pressure_area_product, certify_arc_recovery,
    certify_straight_recovery, CertifiedMemberLoads, CertifiedMemberRecovery, CertifiedRow,
    CertifiedValue, FormationSource, RecoveryFailure,
};

pub(super) fn source(
    built: &BuiltModel,
    force: &AssembledForce,
    springs: &[SpringEntry],
) -> FormationSource {
    FormationSource {
        node_count: built.nodes.len(),
        loads: force.load_primitives().to_vec(),
        frames: built.frame_elements.clone(),
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
pub(super) fn member(
    pipe: &StraightPipeElement,
    bend: Option<&CurvedBendMacroBuild>,
    force: &AssembledForce,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let prefix = format!("member:{}:{}:", pipe.element_id.len(), pipe.element_id);
    let loads = CertifiedMemberLoads::from_member_primitives(
        pipe.node_i.index,
        pipe.node_j.index,
        force.load_primitives(),
        &prefix,
    )?;
    if let Some(bend) = bend {
        certify_arc_recovery(
            &curved_formation_of(bend),
            &pipe.frame_element().map_err(|_| {
                RecoveryFailure::Wide(
                    open_pipe_stress_frame_kernel::structural::retained_api::WideError::CountRange(
                        "product arc frame",
                    ),
                )
            })?,
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
/// its effective increment is (2nu-1)*pAi. Coefficients are formed at p.
pub(super) fn pressure_adjustment(
    recovery: &CertifiedMemberRecovery,
    state: &pressure_runtime::ExactPressurePipeState,
    wall: bool,
) -> Result<CertifiedMemberRecovery, RecoveryFailure> {
    let fluid = certified_pressure_area_product(
        state.pressure.pressure_pa(),
        state.annulus.outside_diameter_m(),
        state.annulus.effective_wall_thickness_m(),
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
    let bending = certified_magnitude(&rows[4..6])?.divided(section.section_modulus)?;
    axial.sum_scaled(bending, 1e-6)
}

pub(super) fn support(
    model: &PreviewModel,
    support: &PreviewSupport,
    built: &BuiltModel,
    springs: &[SpringEntry],
    source: &FormationSource,
    force: &AssembledForce,
    u: &[f64],
    bound: &MemberBoundInputs,
) -> Result<[CertifiedRow; 6], RecoveryFailure> {
    use open_pipe_stress_frame_kernel::structural::{
        certify_spring_reaction, certify_support_reaction,
    };
    let mut rows = [CertifiedRow::ZERO; 6];
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
    for spring in springs
        .iter()
        .filter(|spring| spring.support_id == support.id)
    {
        let dof = spring.node_dof.global_index();
        rows[dof % 6] = certify_spring_reaction(dof, spring.stiffness.value, u, bound)?;
    }
    Ok(rows)
}

pub(super) fn connector_record(
    entity: &str,
    body: Option<usize>,
    certificate: &open_pipe_stress_frame_kernel::structural::CertifiedConnectorRecovery,
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
