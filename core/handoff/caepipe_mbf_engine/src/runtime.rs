//! The importer's ports of the exact route's geometry checks and CAEPIPE's documented local y, operation for
//! operation (importer.py `runtime_*`, `local_y_reference`, `_unit`, `_unit_exact`).

use crate::decimal90::twice_atan2_nonnegative;
use crate::pymath::{correct_norm3, hypot, py_max_all};
use crate::rational::Q;

pub type V3 = [f64; 3];

pub const REPRESENTATION_GUARD: f64 = 64.0 * f64::EPSILON;
pub const TANGENCY_TOLERANCE_RAD: f64 = 1.0e-3;
pub const AXIS_TOLERANCE: f64 = 1.0e-12;
pub const VERTICAL_SINE: f64 = 1e-6;

pub fn sub(a: &V3, b: &V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

pub fn norm(a: &V3) -> f64 {
    hypot(&[a[0], a[1], a[2]])
}

/// `_unit(vector)`: each component divided by math.hypot of the three.
pub fn unit(vector: &V3) -> V3 {
    let length = hypot(&[vector[0], vector[1], vector[2]]);
    [vector[0] / length, vector[1] / length, vector[2] / length]
}

/// `_unit_exact(vector)`: an exact non-zero vector scaled by a power of two, rounded, then normalized.
pub fn unit_exact(vector: &[Q; 3]) -> Result<V3, crate::pystr::PyError> {
    let mut largest = vector[0].abs();
    for value in &vector[1..] {
        if value.abs() > largest {
            largest = value.abs();
        }
    }
    let shift = largest.numer_bits() as i64 - largest.denom_bits() as i64;
    let scale = Q::pow2(-shift);
    let scaled = [vector[0].mul(&scale).to_f64()?, vector[1].mul(&scale).to_f64()?, vector[2].mul(&scale).to_f64()?];
    Ok(unit(&scaled))
}

/// CAEPIPE's documented local y of a straight from `start` to `end`, in SWBPIPE's Y-vertical axes.
pub fn local_y_reference(start: &V3, end: &V3, z_vertical: bool) -> V3 {
    let x = unit(&sub(end, start));
    let horizontal = hypot(&[x[0], x[2]]);
    if horizontal <= VERTICAL_SINE {
        return if z_vertical { [0.0, 0.0, -1.0] } else { [-1.0, 0.0, 0.0] };
    }
    if z_vertical {
        return [x[2] / horizontal + 0.0, 0.0, -x[0] / horizontal + 0.0];
    }
    let projected = [-x[1] * x[0], x[0] * x[0] + x[2] * x[2], -x[1] * x[2]];
    let u = unit(&projected);
    [u[0] + 0.0, u[1] + 0.0, u[2] + 0.0]
}

/// pressure_runtime.rs `collinear_run` on the run's nodes in m.
pub fn collinear(points: &[V3]) -> bool {
    let origin = points[0];
    let end = points[points.len() - 1];
    let chord = [end[0] - origin[0], end[1] - origin[1], end[2] - origin[2]];
    let length = correct_norm3(chord[0], chord[1], chord[2]);
    if !length.is_finite() || length <= 0.0 {
        return false;
    }
    let direction = [chord[0] / length, chord[1] / length, chord[2] / length];
    let mut previous = 0.0;
    for point in &points[1..] {
        let delta = [(point[0] - origin[0]) / length, (point[1] - origin[1]) / length, (point[2] - origin[2]) / length];
        let mut projection = 0.0;
        for axis in 0..3 {
            projection += delta[axis] * direction[axis];
        }
        let residual = correct_norm3(
            delta[0] - projection * direction[0],
            delta[1] - projection * direction[1],
            delta[2] - projection * direction[2],
        );
        if !projection.is_finite()
            || !residual.is_finite()
            || residual > REPRESENTATION_GUARD
            || projection <= previous
            || projection > 1.0 + REPRESENTATION_GUARD
        {
            return false;
        }
        previous = projection;
    }
    true
}

pub fn bore_radius_m(od: f64, wall: f64, factor: f64) -> f64 {
    let od_m = od * factor + 0.0;
    let wall_m = wall * factor + 0.0;
    let reduced = (wall_m - 0.0) - (0.0 * factor + 0.0);
    od_m * 0.5 - reduced
}

pub fn section_valid(od: f64, wall: f64, factor: f64) -> bool {
    let od_m = od * factor + 0.0;
    let wall_m = wall * factor + 0.0;
    if !(od_m.is_finite() && wall_m.is_finite()) || od_m <= 0.0 || wall_m <= 0.0 || 2.0 * wall_m >= od_m {
        return false;
    }
    if wall_m > od_m || wall_m > od_m - wall_m {
        return false;
    }
    let outer = od_m * 0.5;
    let inner = outer - wall_m;
    !(inner <= 0.0 || inner >= outer)
}

pub fn bores_equal(bores: &[f64]) -> bool {
    let reference = bores[0];
    bores[1..].iter().all(|&bore| (reference - bore).abs() <= REPRESENTATION_GUARD * reference.abs().max_py(bore.abs()))
}

trait PyMax {
    fn max_py(self, other: f64) -> f64;
}
impl PyMax for f64 {
    /// Python's `max(a, b)`.
    fn max_py(self, other: f64) -> f64 {
        if other > self {
            other
        } else {
            self
        }
    }
}

pub fn dot(a: &V3, b: &V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn cross(a: &V3, b: &V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

pub fn finite(vector: &V3) -> bool {
    vector.iter().all(|value| value.is_finite())
}

/// frame_kernel `FrameElement::orientation` on positions in m: [x, y, z], or None where the kernel refuses.
pub fn frame_axes(start: &V3, end: &V3, y_reference: &V3) -> Option<[V3; 3]> {
    fn normalize(vector: &V3) -> Option<V3> {
        let magnitude = dot(vector, vector).sqrt();
        if magnitude <= AXIS_TOLERANCE {
            return None;
        }
        let inverse = 1.0 / magnitude;
        Some([vector[0] * inverse, vector[1] * inverse, vector[2] * inverse])
    }
    let axis = normalize(&[end[0] - start[0], end[1] - start[1], end[2] - start[2]])?;
    if !finite(&axis) || !finite(y_reference) {
        return None;
    }
    let x = normalize(&axis)?;
    let projection = dot(y_reference, &x);
    let y = normalize(&[
        y_reference[0] - x[0] * projection,
        y_reference[1] - x[1] * projection,
        y_reference[2] - x[2] * projection,
    ])?;
    let z = normalize(&cross(&x, &y))?;
    let axes = [x, y, z];
    if !axes.iter().all(finite) {
        return None;
    }
    if axes.iter().any(|item| (dot(item, item).sqrt() - 1.0).abs() > AXIS_TOLERANCE) {
        return None;
    }
    if dot(&x, &y).abs() > AXIS_TOLERANCE || dot(&x, &z).abs() > AXIS_TOLERANCE || dot(&y, &z).abs() > AXIS_TOLERANCE {
        return None;
    }
    let right = cross(&x, &y);
    if py_max_all(&[(right[0] - z[0]).abs(), (right[1] - z[1]).abs(), (right[2] - z[2]).abs()]) > AXIS_TOLERANCE {
        return None;
    }
    Some(axes)
}

/// curved_bend `kink`: 2 atan2(|a - t|, |a + t|) of the unit directions; None where the runtime refuses.
pub fn kink(a: &V3, t: &V3) -> Option<f64> {
    fn normalize(vector: &V3) -> Option<V3> {
        let magnitude = dot(vector, vector).sqrt();
        if magnitude <= AXIS_TOLERANCE {
            return None;
        }
        Some([vector[0] / magnitude, vector[1] / magnitude, vector[2] / magnitude])
    }
    if !(finite(a) && finite(t)) {
        return None;
    }
    let a = normalize(a)?;
    let t = normalize(t)?;
    let difference = [a[0] - t[0], a[1] - t[1], a[2] - t[2]];
    let total = [a[0] + t[0], a[1] + t[1], a[2] + t[2]];
    twice_atan2_nonnegative(dot(&difference, &difference).sqrt(), dot(&total, &total).sqrt())
}

/// A realized bend's end tangents [t_i, t_j] as the runtime builds them on positions and radius in m.
pub fn bend_tangents(start: &V3, end: &V3, radius_m: f64, y_reference: &V3) -> Option<(V3, V3)> {
    let chord = [end[0] - start[0], end[1] - start[1], end[2] - start[2]];
    if !(finite(&chord) && finite(y_reference) && radius_m.is_finite()) || radius_m <= 0.0 {
        return None;
    }
    let plain = (chord[0] * chord[0] + chord[1] * chord[1] + chord[2] * chord[2]).sqrt();
    if !(plain.is_finite() && plain > 0.0) || radius_m <= 0.5 * plain {
        return None;
    }
    let unit = [chord[0] / plain, chord[1] / plain, chord[2] / plain];
    let along = y_reference[0] * unit[0] + y_reference[1] * unit[1] + y_reference[2] * unit[2];
    let plane = [y_reference[0] - along * unit[0], y_reference[1] - along * unit[1], y_reference[2] - along * unit[2]];
    if (plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]).sqrt() <= 1.0e-9 {
        return None;
    }
    let exact: Vec<Q> = chord.iter().map(|&value| Q::from_f64(value).expect("finite")).collect();
    let squared = exact.iter().fold(Q::zero(), |total, value| total.add(&value.mul(value)));
    let length_squared = squared.to_f64().ok()?;
    let four_radius = 4.0 * radius_m;
    if !four_radius.is_finite() {
        return None;
    }
    let span = Q::from_f64(four_radius).expect("finite").mul(&Q::from_f64(radius_m).expect("finite")).sub(&squared);
    let span = span.to_f64().ok()?;
    let chord_length = length_squared.sqrt();
    if !(chord_length.is_finite() && chord_length > 0.0) || span <= 0.0 {
        return None;
    }
    let diameter = 2.0 * radius_m;
    let s = chord_length / diameter;
    let c = span.sqrt() / diameter;
    let angle = twice_atan2_nonnegative(s, c)?;
    if !(1.0e-9 <= angle && angle <= std::f64::consts::PI - 1.0e-9) {
        return None;
    }
    let d = [chord[0] / chord_length, chord[1] / chord_length, chord[2] / chord_length];
    let reference_magnitude = dot(y_reference, y_reference).sqrt();
    let mut normal = *y_reference;
    for _ in 0..2 {
        let projection = dot(&normal, &d);
        normal = [normal[0] - projection * d[0], normal[1] - projection * d[1], normal[2] - projection * d[2]];
    }
    let normal_magnitude = dot(&normal, &normal).sqrt();
    if !(normal_magnitude > AXIS_TOLERANCE * reference_magnitude) {
        return None;
    }
    let n = [normal[0] / normal_magnitude, normal[1] / normal_magnitude, normal[2] / normal_magnitude];
    let axes = [
        [-s * d[0] + c * n[0], -s * d[1] + c * n[1], -s * d[2] + c * n[2]],
        [c * d[0] + s * n[0], c * d[1] + s * n[1], c * d[2] + s * n[2]],
        cross(&n, &d),
    ];
    if !axes.iter().all(finite) || axes.iter().any(|item| (dot(item, item).sqrt() - 1.0).abs() > AXIS_TOLERANCE) {
        return None;
    }
    if dot(&axes[0], &axes[1]).abs() > AXIS_TOLERANCE
        || dot(&axes[0], &axes[2]).abs() > AXIS_TOLERANCE
        || dot(&axes[1], &axes[2]).abs() > AXIS_TOLERANCE
    {
        return None;
    }
    let right = cross(&axes[0], &axes[1]);
    if py_max_all(&[(right[0] - axes[2][0]).abs(), (right[1] - axes[2][1]).abs(), (right[2] - axes[2][2]).abs()])
        > AXIS_TOLERANCE
    {
        return None;
    }
    Some((axes[1], [c * d[0] - s * n[0], c * d[1] - s * n[1], c * d[2] - s * n[2]]))
}
