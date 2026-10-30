//! E17's representation controls exercise the actual G7 magnitude callers.
//! The independent scalar publication may differ from the norm of the published
//! components. These controls provide no producer authentication or native witness.
use super::*;
use serde_json::json;

fn binary64(bits: &Value) -> f64 {
    f64::from_bits(u64::from_str_radix(bits.as_str().unwrap(), 16).unwrap())
}

fn support(values: [f64; 3], magnitude: f64, moment: bool) -> Check {
    let (force, couple) = if moment { ([0.0; 3], values) } else { (values, [0.0; 3]) };
    let values = [force[0], force[1], force[2], couple[0], couple[1], couple[2],
        if moment { 0.0 } else { magnitude }, if moment { magnitude } else { 0.0 }];
    let rows: Vec<_> = values.into_iter().map(|value| json!({"value":value})).collect();
    let slots = SUPPORT_COMPONENTS.into_iter().zip(&rows).collect();
    magnitudes(&slots)
}

fn combined(values: [f64; 3], magnitude: f64, kind: &str) -> Check {
    let components = match kind {
        "displacement_magnitude" => ["nodal_displacement_x", "nodal_displacement_y", "nodal_displacement_z"],
        SUPPORT_FORCE => ["Fx", "Fy", "Fz"],
        SUPPORT_MOMENT => ["Mx", "My", "Mz"],
        _ => unreachable!(),
    };
    let mut rows: Vec<_> = components.into_iter().zip(values).enumerate().map(|(axis, (component, value))| {
        let row_kind = if kind == "displacement_magnitude" {
            ["global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z"][axis]
        } else { SUPPORT_COMPONENT };
        json!({"kind":row_kind,"entity_ref":"invented:combined","value":value,"metadata":{"component":component}})
    }).collect();
    rows.push(json!({"kind":kind,"entity_ref":"invented:combined","value":magnitude}));
    combination_magnitudes(&rows.iter().collect::<Vec<_>>())
}

#[test]
fn e17_all_g7_magnitude_callers_keep_the_declared_guard_and_nonnegative_range() {
    let controls: Value = serde_json::from_str(include_str!("../../../../fixtures/results/preview_magnitude_controls.json")).unwrap();
    for control in controls["controls"].as_array().unwrap() {
        let values = std::array::from_fn(|i| binary64(&control["component_bits"][i]));
        let magnitude = binary64(&control["magnitude_bits"]);
        let admitted = control["admitted"].as_bool().unwrap();
        for moment in [false, true] {
            let result = support(values, magnitude, moment);
            assert_eq!(result.is_ok(), admitted, "{} support moment={moment}: {result:?}", control["id"]);
            if !admitted { assert_eq!(result.unwrap_err(), "SOURCE_PREVIEW_PHYSICS_SUPPORT_MAGNITUDE"); }
        }
        for kind in ["displacement_magnitude", SUPPORT_FORCE, SUPPORT_MOMENT] {
            let result = combined(values, magnitude, kind);
            assert_eq!(result.is_ok(), admitted, "{} {kind}: {result:?}", control["id"]);
            if !admitted { assert_eq!(result.unwrap_err(), "SOURCE_PREVIEW_PHYSICS_COMBINATION_MAGNITUDE"); }
        }
    }
}

#[test]
fn e17_generic_guard_and_signed_components_keep_their_existing_range() {
    let s = f64::from_bits(1);
    assert!(guarded(-s, -s));
    assert_eq!(number(&json!(-s)).unwrap().to_bits(), (-s).to_bits());
    assert!(support([-s, 0.0, 0.0], s, false).is_ok());
    assert_eq!((64.0 * f64::EPSILON * f64::MIN_POSITIVE).to_bits(), 64);
}
