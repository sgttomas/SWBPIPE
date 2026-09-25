//! Geometric rigid-motion screen, separate from matrix invertibility.
use crate::structural::{gamma, StructuralError};
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
    let mut actual_constraints = Vec::new();
    for &d in ground_dofs {
        let mut row = motion_rows(d / 6)[d % 6];
        actual_constraints.push(row);
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
    let mut candidate: [f64; 6] = std::array::from_fn(|i| v[i][index]);
    let mut max_action = actual_constraints
        .iter()
        .map(|r| {
            r.iter()
                .zip(candidate)
                .map(|(a, b)| a * b)
                .sum::<f64>()
                .abs()
        })
        .fold(0.0, f64::max);
    // Probe exact represented axes as well as the SVD vector. In the common two-node
    // oblique torsion case the authored coordinate direction cancels exactly whereas
    // its normalized SVD approximation need not. Every candidate is checked anew.
    if max_action != 0.0 {
        let mut candidates = Vec::new();
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
        for c in candidates {
            let action = actual_constraints
                .iter()
                .map(|r| r.iter().zip(c).map(|(a, b)| a * b).sum::<f64>().abs())
                .fold(0.0, f64::max);
            if action == 0.0 && actual_constraints.iter().all(|row| exact_dot_zero(row, &c)) {
                candidate = c;
                max_action = 0.0;
                break;
            }
        }
    }
    result.maximum_constraint_action = Some(max_action);
    // A roundoff-small residual is NOT an exact physical null witness. Only exact represented
    // constraint annihilation is labelled witnessed; near-collinear ambiguity remains unresolved.
    if actual_constraints
        .iter()
        .all(|row| exact_dot_zero(row, &candidate))
    {
        let mut motions = Vec::new();
        for i in 0..coordinates.len() {
            let rows = motion_rows(i);
            let mut motion = [0.0; 6];
            for j in 0..6 {
                motion[j] = rows[j].iter().zip(candidate).map(|(a, b)| a * b).sum();
                if j < 3 {
                    motion[j] *= length;
                }
            }
            if motion.iter().any(|v| !v.is_finite()) {
                return Err(StructuralError::Range("rigid motion recovery"));
            }
            motions.push(motion);
        }
        result.status = RigidBodyStatus::MechanismWitnessed;
        result.rigid_parameters = Some(candidate);
        result.node_motion = Some(motions);
    }
    Ok(result)
}

// Error-free product (FMA residual) and expansion addition check exact annihilation
// of represented geometry. Tiny products outside the normal error-free-transform
// assumptions are uncertainty, not a null proof.
fn exact_dot_zero(a: &[f64; 6], b: &[f64; 6]) -> bool {
    let mut expansion = Vec::<f64>::new();
    for (&x, &y) in a.iter().zip(b) {
        if x == 0.0 || y == 0.0 {
            continue;
        }
        let product = x * y;
        if !product.is_finite() || product.abs() < f64::MIN_POSITIVE / f64::EPSILON {
            return false;
        }
        let error = x.mul_add(y, -product);
        for value in [error, product] {
            let mut q = value;
            let mut next = Vec::new();
            for &e in &expansion {
                let sum = q + e;
                let z = sum - q;
                let low = (q - (sum - z)) + (e - z);
                if low != 0.0 {
                    next.push(low);
                }
                q = sum;
            }
            if q != 0.0 {
                next.push(q);
            }
            expansion = next;
        }
    }
    expansion.is_empty()
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
}
