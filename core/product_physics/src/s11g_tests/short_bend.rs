//! HELP_HUMAN's ruling 5 (2026-10-10; DEL-04-01 "Stable small-angle
//! evaluation"): a realized bend that is short against its neighbours. On
//! T4-I22's L line (A-(3 m)-B, bend R = 0.2286 m / phi B-C, C-(4 m)-D, anchored
//! at A and D, self-weight on every member) the pivot screen refuses phi =
//! 1e-4 rad (R*phi/L = 7.6e-6), and the refusal names the bend and the ratio;
//! a Passed 90 degree bend and a Sensitive 1 degree bend carry no such
//! diagnostic. Inputs are invented.
use super::*;

const R: f64 = 0.2286;
const LA: f64 = 3.0;
const LB: f64 = 4.0;

/// T4-I22's L line: the realized bend B-C between the straights.
fn l_line(phi: f64) -> Value {
    let mut model = preview_model("t4-short-bend");
    model["materials"] = json!([material()]);
    let b = [LA, 0.0];
    let c = [b[0] + R * phi.sin(), R * (1.0 - phi.cos())];
    let d = [c[0] + LB * phi.cos(), c[1] + LB * phi.sin()];
    let section = json!({"outside_diameter": {"value": 0.168, "unit": "m"}, "wall_thickness": {"value": 0.007, "unit": "m"}});
    let with_section = |mut p: Value| {
        p["section"] = section.clone();
        p
    };
    model["nodes"] = json!([node("A", 0.0, 0.0, 0.0), node("B", b[0], b[1], 0.0), node("C", c[0], c[1], 0.0), node("D", d[0], d[1], 0.0)]);
    model["pipe_segments"] = json!([
        with_section(pipe("AB", "A", "B", [0.0, 1.0, 0.0])),
        with_section(pipe("BC", "B", "C", [0.0, -1.0, 0.0])),
        with_section(pipe("CD", "C", "D", [0.0, 0.0, 1.0]))]);
    model["load_cases"][0]["primitive_loads"] = json!([
        uniform("w:AB", "AB", "global_z", -450.0), uniform("w:BC", "BC", "global_z", -450.0), uniform("w:CD", "CD", "global_z", -450.0)]);
    model["supports"] = json!([support("aA", "A", &ALL), support("aD", "D", &ALL)]);
    let chord = ((c[0] - b[0]).powi(2) + (c[1] - b[1]).powi(2)).sqrt();
    model["components"] = json!([{
        "id": "component:BC", "label": "Invented bend", "kind": "bend", "node": "C",
        "geometry": {"bend_pipe_ref": "BC", "bend_radius": {"value": R, "unit": "m"},
            "bend_angle": {"value": 2.0 * (0.5 * chord / R).asin(), "unit": "rad"},
            "bend_plane_orientation": "global_xy_preview",
            "bend_geometry_source_reference": "invented_user_entered_preview_geometry"},
        "modifiers": {"sif_user_value": {"value": 1.15, "unit": "none"},
            "flexibility_factor_user_value": {"value": 1.0, "unit": "none"},
            "source_reference": "invented_user_entered_preview_no_code_table"},
        "mechanics_interface": {"solver_consumption": "curved_bend_macro_element",
            "rule_check_consumption": "user_rule_pack_inputs_only"},
        "completeness": [{"finding_id": "finding:BC:geometry", "status": "complete",
            "diagnostic_code": "BEND_GEOMETRY_INCOMPLETE", "missing_field_kinds": []}],
        "provenance": INVENTED}]);
    request_of(model)
}

fn short_bend(envelope: &MechanicsEnvelope) -> Vec<&Diagnostic> {
    envelope.diagnostics.iter().filter(|d| d.code == CURVED_BEND_SHORT).collect()
}

#[test]
fn a_pivot_screen_refusal_names_a_bend_short_against_its_neighbours() {
    for entry in [Entry::Captured, Entry::Typed] {
        for mode in MODES {
            let label = format!("{entry:?} {mode:?}");
            let envelope = run(entry, &l_line(1e-4), mode).unwrap();
            // The screen refused the case (its reason, verbatim, unchanged).
            let refusal = integrity(&envelope, "case");
            assert_eq!(refusal.severity, "blocking", "{label}");
            assert!(refusal.message.contains(PIVOT_SCREEN_REASON), "{label}: {}", refusal.message);
            let found = short_bend(&envelope);
            assert_eq!(found.len(), 1, "{label}: {:?}", envelope.diagnostics);
            let finding = found[0];
            assert_eq!(finding.severity, "blocking", "{label}");
            assert_eq!(finding.id, "diagnostic:curved-bend-short:case", "{label}");
            assert_eq!(finding.affected_refs, ["case", "component:BC", "BC"], "{label}");
            // R*phi / min(3 m, 4 m) = 0.2286e-4 / 3.
            let ratio = R * 1e-4 / LA;
            assert_eq!(
                finding.message,
                format!("Load case case: realized bend component:BC (pipe BC): arc length / shorter adjacent member length = {ratio:.3e}, below 1e-4; {CURVED_BEND_SHORT_TEXT}"),
                "{label}"
            );
            assert!(finding.message.contains("too short relative to its neighbours for a reliable solution"));
            assert!(finding.message.contains("7.620e-6"), "{label}: {}", finding.message);
        }
    }
}

#[test]
fn a_passed_or_sensitive_bend_carries_no_short_bend_diagnostic() {
    for (phi, quality) in [
        (90f64.to_radians(), NumericalQualityStatus::ChecksPassed),
        (1f64.to_radians(), NumericalQualityStatus::Sensitive),
        // Below the ratio, but Sensitive rather than refused: not named.
        (1e-3, NumericalQualityStatus::Sensitive),
    ] {
        for entry in [Entry::Captured, Entry::Typed] {
            for mode in MODES {
                let label = format!("{phi} {entry:?} {mode:?}");
                let envelope = solved(entry, &l_line(phi), mode);
                assert_eq!(case_quality(&envelope, "case"), quality, "{label}");
                assert!(short_bend(&envelope).is_empty(), "{label}");
            }
        }
    }
}

/// The ratio test itself: the bend's arc over its shorter neighbour, and the
/// threshold's place between T4-I22's measured regimes (refused at 3.8e-5,
/// Sensitive at 7.6e-5 on the L line).
#[test]
fn the_ratio_uses_the_shorter_neighbour_and_sits_at_the_refusal_boundary() {
    for phi in [1e-4, 1f64.to_radians(), 90f64.to_radians()] {
        let built = prepared(&l_line(phi)).built;
        let (bend, ratio) = shortest_bend_against_neighbours(&built).unwrap();
        assert_eq!(bend.component_id, "component:BC");
        let expected = bend.arc_length / LA;
        assert_eq!(ratio.to_bits(), expected.to_bits(), "{phi}");
        assert!(((bend.arc_length - R * phi) / (R * phi)).abs() < 1e-9, "{phi}");
    }
    assert!(R * 5e-4 / LA < CURVED_BEND_SHORT_RATIO);
    assert!(R * 1e-3 / LA < CURVED_BEND_SHORT_RATIO);
    assert!(R * 1f64.to_radians() / LA > CURVED_BEND_SHORT_RATIO);
}
