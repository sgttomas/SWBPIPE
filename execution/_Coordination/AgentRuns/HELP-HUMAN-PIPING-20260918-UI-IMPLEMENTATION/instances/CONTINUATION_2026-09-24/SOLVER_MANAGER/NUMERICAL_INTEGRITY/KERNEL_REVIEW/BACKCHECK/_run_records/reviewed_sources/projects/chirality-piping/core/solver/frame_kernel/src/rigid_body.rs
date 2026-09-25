//! Geometric rigid-motion screen, separate from matrix invertibility.
use crate::structural::{gamma, Expansion, StructuralError};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectiveFamily {
    /// Welded unreleased straight frames with a positive objective elastic energy.
    WeldedUnreleasedElasticFrames,
    /// Geometry is diagnostic only; no physical-mechanism assertion is licensed.
    Unqualified,
}
#[derive(Debug, Clone, PartialEq)]
pub enum RigidBodyStatus {
    Restrained,
    MechanismWitnessed,
    NumericallyUnresolved,
    UnqualifiedFamily,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RigidBodyAssessment {
    pub status: RigidBodyStatus,
    pub singular_values: [f64; 6],
    pub rank_screen: f64,
    pub characteristic_length: f64,
    pub origin: [f64; 3],
    /// Dimensionless [tx,ty,tz,rx,ry,rz]; translations at nodes are length*rigid translation.
    pub rigid_parameters: Option<[f64; 6]>,
    pub node_motion: Option<Vec<[f64; 6]>>,
    pub maximum_constraint_action: Option<f64>,
    pub iterations: usize,
}
/// Coordinates and ground indices are LOCAL to one actual connected body supplied by caller.
/// Positive ground springs and actually selected active contacts may occur in ground_dofs.
/// Inter-node connectors are not grounds. This utility does not validate connectivity or internal modes.
pub fn assess_rigid_body(
    coordinates: &[[f64; 3]],
    ground_dofs: &[usize],
    family: ObjectiveFamily,
) -> Result<RigidBodyAssessment, StructuralError> {
    if coordinates.is_empty()
        || coordinates.iter().flatten().any(|v| !v.is_finite())
        || ground_dofs.iter().any(|&i| i >= 6 * coordinates.len())
    {
        return Err(StructuralError::InvalidInput("rigid geometry"));
    }
    let origin = coordinates[0];
    let mut relative = Vec::new();
    let mut length: f64 = 0.0;
    for p in coordinates {
        let r = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
        if r.iter().any(|v| !v.is_finite()) {
            return Err(StructuralError::Range("relative coordinates"));
        }
        length = length.max(r[0].hypot(r[1]).hypot(r[2]));
        relative.push(r);
    }
    if !length.is_finite() {
        return Err(StructuralError::Range("nonfinite characteristic length"));
    }
    if length == 0.0 {
        length = 1.0;
    }
    for r in &mut relative {
        for x in r {
            *x /= length;
        }
    }
    let motion_rows = |node: usize| {
        let [x, y, z] = relative[node];
        [
            [1.0, 0.0, 0.0, 0.0, z, -y],
            [0.0, 1.0, 0.0, -z, 0.0, x],
            [0.0, 0.0, 1.0, y, -x, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        ]
    };
    let mut original = Vec::new();

    for &d in ground_dofs {
        let mut row = motion_rows(d / 6)[d % 6];

        let norm = row.iter().map(|x| x * x).sum::<f64>().sqrt();
        for x in &mut row {
            *x /= norm;
        }
        original.push(row);
    }
    // One-sided Jacobi SVD avoids B^T B and its squared condition number.
    let mut b = original.clone();
    let mut v = [[0.0; 6]; 6];
    for i in 0..6 {
        v[i][i] = 1.0;
    }
    let mut converged = false;
    let mut iterations = 0;
    for sweep in 0..64 {
        iterations = sweep + 1;
        let mut changed = false;
        for p in 0..6 {
            for q in p + 1..6 {
                let alpha: f64 = b.iter().map(|r| r[p] * r[p]).sum();
                let beta: f64 = b.iter().map(|r| r[q] * r[q]).sum();
                let cross: f64 = b.iter().map(|r| r[p] * r[q]).sum();
                if cross.abs() <= 8.0 * f64::EPSILON * (alpha.sqrt() * beta.sqrt()) {
                    continue;
                }
                let zeta = (beta - alpha) / (2.0 * cross);
                let t = zeta.signum() / (zeta.abs() + zeta.hypot(1.0));
                let t = if zeta == 0.0 { 1.0 } else { t };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = c * t;
                for row in &mut b {
                    let a = row[p];
                    let d = row[q];
                    row[p] = c * a - s * d;
                    row[q] = s * a + c * d;
                }
                for row in &mut v {
                    let a = row[p];
                    let d = row[q];
                    row[p] = c * a - s * d;
                    row[q] = s * a + c * d;
                }
                changed = true;
            }
        }
        if !changed {
            converged = true;
            break;
        }
    }
    let singular = std::array::from_fn(|j| b.iter().map(|r| r[j] * r[j]).sum::<f64>().sqrt());
    let max = singular.iter().copied().fold(0.0, f64::max);
    let screen = 64.0 * gamma(ground_dofs.len().max(6)) * max;
    let index = (0..6)
        .min_by(|&i, &j| singular[i].total_cmp(&singular[j]))
        .unwrap();
    let mut result = RigidBodyAssessment {
        status: RigidBodyStatus::NumericallyUnresolved,
        singular_values: singular,
        rank_screen: screen,
        characteristic_length: length,
        origin,
        rigid_parameters: None,
        node_motion: None,
        maximum_constraint_action: None,
        iterations,
    };
    if family == ObjectiveFamily::Unqualified {
        result.status = RigidBodyStatus::UnqualifiedFamily;
        return Ok(result);
    }
    if converged && singular[index] > screen {
        result.status = RigidBodyStatus::Restrained;
        return Ok(result);
    }
    // Rounded/centered rows only generate candidates. The final witness below uses
    // exact expansions of the ORIGINAL represented coordinates, not these rows.
    let mut candidates = vec![std::array::from_fn(|i| v[i][index])];
    for axis in 0..6 {
        let mut c = [0.0; 6];
        c[axis] = 1.0;
        candidates.push(c);
    }
    for r in &relative {
        if r.iter().any(|x| *x != 0.0) {
            candidates.push([0.0, 0.0, 0.0, r[0], r[1], r[2]]);
        }
    }
    // Unnormalized authored differences can preserve exact axes (e.g. (3,4,0))
    // that division by a non-radix characteristic length would perturb.
    for p in coordinates {
        let r = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
        if r.iter().any(|x| *x != 0.0) {
            candidates.push([0.0, 0.0, 0.0, r[0], r[1], r[2]]);
        }
    }
    for candidate in candidates {
        // Unsupported expansion range or an inexact recovered motion is numerical
        // uncertainty. Neither turns a nonzero original constraint action into zero.
        if let Ok(Some(motions)) =
            original_rigid_witness(coordinates, ground_dofs, length, &candidate)
        {
            result.status = RigidBodyStatus::MechanismWitnessed;
            result.rigid_parameters = Some(candidate);
            result.node_motion = Some(motions);
            result.maximum_constraint_action = Some(0.0);
            break;
        }
    }
    Ok(result)
}
fn original_rigid_witness(
    coordinates: &[[f64; 3]],
    ground: &[usize],
    length: f64,
    candidate: &[f64; 6],
) -> Result<Option<Vec<[f64; 6]>>, StructuralError> {
    if candidate.iter().any(|v| !v.is_finite()) || candidate.iter().all(|v| *v == 0.0) {
        return Ok(None);
    }
    let origin = coordinates[0];
    let mut exact_motions = Vec::new();
    for p in coordinates {
        let mut delta: [Expansion; 3] = std::array::from_fn(|_| Expansion::default());
        for axis in 0..3 {
            delta[axis].add(p[axis])?;
            delta[axis].add(-origin[axis])?;
        }
        let mut motion: [Expansion; 6] = std::array::from_fn(|_| Expansion::default());
        for axis in 0..3 {
            motion[axis].add_product(length, candidate[axis], 0)?;
            motion[axis + 3].add(candidate[axis + 3])?;
        }
        // omega cross (x-original_origin), with exact subtraction and products.
        for (component, positive_axis, positive_omega, negative_axis, negative_omega) in
            [(0, 2, 4, 1, 5), (1, 0, 5, 2, 3), (2, 1, 3, 0, 4)]
        {
            for &part in &delta[positive_axis].terms {
                motion[component].add_product(part, candidate[positive_omega], 0)?;
            }
            for &part in &delta[negative_axis].terms {
                motion[component].add_product(-part, candidate[negative_omega], 0)?;
            }
        }
        exact_motions.push(motion);
    }
    if ground
        .iter()
        .any(|&d| !exact_motions[d / 6][d % 6].is_zero())
    {
        return Ok(None);
    }
    // Publish node_motion only if it exactly represents this common original-coordinate
    // rigid motion. A rounded recovery is not silently presented as an exact witness.
    let mut motions = Vec::new();
    for exact in exact_motions {
        let mut motion = [0.0; 6];
        for j in 0..6 {
            let Some(value) = exact[j].exact_scalar()? else {
                return Ok(None);
            };
            motion[j] = value;
        }
        motions.push(motion);
    }
    Ok(Some(motions))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oblique_geometry_and_actual_rotation_restraint() {
        let coords = [[0.0, 0.0, 0.0], [1.2, 1.6, 0.0]];
        let translations = [0, 1, 2, 6, 7, 8];
        let open = assess_rigid_body(
            &coords,
            &translations,
            ObjectiveFamily::WeldedUnreleasedElasticFrames,
        )
        .unwrap();
        assert_ne!(open.status, RigidBodyStatus::Restrained);
        let mut wrong = translations.to_vec();
        wrong.push(5);
        assert_ne!(
            assess_rigid_body(
                &coords,
                &wrong,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
        let mut right = translations.to_vec();
        right.push(3);
        assert_eq!(
            assess_rigid_body(
                &coords,
                &right,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
    }
    #[test]
    fn full_ground_rank_does_not_assert_internal_stiffness_rank() {
        let coords = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
        let ground = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            assess_rigid_body(
                &coords,
                &ground,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
        let k = vec![vec![1.0, 0.0], vec![0.0, 0.0]];
        let f = [0.0, 0.0];
        let free = [0, 1];
        let s = crate::structural::StructuralSystem {
            stiffness: &k,
            force: &f,
            free_dofs: &free,
            prescribed: &[],
            contributions: None,
            symmetry: None,
        };
        assert!(crate::structural::solve_structural_dense(&s).is_err());
        assert_eq!(
            assess_rigid_body(&coords, &ground, ObjectiveFamily::Unqualified)
                .unwrap()
                .status,
            RigidBodyStatus::UnqualifiedFamily
        );
    }
    #[test]
    fn empty_ground_has_explicit_null_and_origin_shift_does_not_change_rank() {
        let coords = [[1e8, 1e8, 1e8], [1e8 + 2.0, 1e8, 1e8]];
        let result =
            assess_rigid_body(&coords, &[], ObjectiveFamily::WeldedUnreleasedElasticFrames)
                .unwrap();
        assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
        assert!(result.node_motion.is_some());
        let ground = [0, 1, 2, 3, 4, 5];
        assert_eq!(
            assess_rigid_body(
                &coords,
                &ground,
                ObjectiveFamily::WeldedUnreleasedElasticFrames
            )
            .unwrap()
            .status,
            RigidBodyStatus::Restrained
        );
    }
    #[test]
    fn overflowing_geometry_length_is_range_uncertainty() {
        let coords = [[0.0, 0.0, 0.0], [1.5e308, 1.5e308, 1.5e308]];
        assert!(matches!(
            assess_rigid_body(&coords, &[], ObjectiveFamily::WeldedUnreleasedElasticFrames),
            Err(StructuralError::Range(_))
        ));
    }
    #[test]
    fn krev01_original_geometry_refutes_coarsened_null_witnesses() {
        let examples = [
            [[0.0, 0.0, 0.0], [1e200, 0.0, 0.0], [0.0, 1e-200, 0.0]],
            [
                [1e16, 1e16, 0.0],
                [-1e16, -1e16, 0.0],
                [-1e16, -9999999999999998.0, 0.0],
            ],
        ];
        let ground = [0, 1, 2, 6, 7, 8, 12, 13, 14];
        for coordinates in examples {
            for units in [0.5, 1.0, 2.0] {
                let coordinates = coordinates.map(|p| p.map(|x| x * units));
                let result = assess_rigid_body(
                    &coordinates,
                    &ground,
                    ObjectiveFamily::WeldedUnreleasedElasticFrames,
                )
                .unwrap();
                assert_ne!(result.status, RigidBodyStatus::MechanismWitnessed);
                assert!(result.node_motion.is_none());
            }
        }
        let coords = [[0.0, 0.0, 0.0], [1e200, 0.0, 0.0], [0.0, 1e-200, 0.0]];
        assert!(
            original_rigid_witness(&coords, &ground, 1e200, &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0])
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn krev01_valid_oblique_witness_survives_origin_and_radix_units() {
        let ground = [0, 1, 2, 6, 7, 8];
        for units in [0.5, 1.0, 2.0] {
            for shift in [0.0, 8.0] {
                let coords = [
                    [shift, shift, 0.0],
                    [shift + 3.0 * units, shift + 4.0 * units, 0.0],
                ];
                let result = assess_rigid_body(
                    &coords,
                    &ground,
                    ObjectiveFamily::WeldedUnreleasedElasticFrames,
                )
                .unwrap();
                assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
                for motion in result.node_motion.unwrap() {
                    assert_eq!(&motion[..3], &[0.0, 0.0, 0.0]);
                }
            }
        }
    }
}
