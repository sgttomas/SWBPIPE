//! W-15 slice c, R-c1 convention tests (DEL-03-05 Design `eccentric-reducer-offset.md`
//! §3.6, §7.5, §12). These are the only convention-dependent tests: the
//! implemented convention is option A (the offset is measured from the
//! component's node to the body's other end). A ruling for option B changes
//! `rigid_component::eccentric_offset_ends`, the desktop's
//! `ECCENTRIC_OFFSET_CONVENTION` text and the expected signs here, and nothing
//! else. Inputs are invented; the expectations are the frozen W15C references.
#[path = "common/w15c_bench.rs"]
mod common_w15c;
use common_w15c::*;
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::json;

/// W15C-R: the body member reversed (B → A), the reducer at A. Under option A the
/// offset is still (0, −50, 0) mm from A, and the case solves as W15C-P.
/// W15C-O: the reducer at B. Under option A the offset is (0, +50, 0) mm from B.
#[test]
fn option_a_convention_rows_solve_as_the_axial_step() {
    for id in ["W15C-R-S1000-EB", "W15C-O-S1000-EB"] {
        for route in [Route::Preview, Route::Exact] {
            run(id, route, |r| Authoring { loads: tip_force(r), ..Default::default() });
        }
    }
}

/// The other option's sign on each convention row is refused by name, with a
/// 0.1 m residual.
#[test]
fn the_other_options_sign_is_refused() {
    for id in ["W15C-R-S1000-EB", "W15C-O-S1000-EB"] {
        for route in [Route::Preview, Route::Exact] {
            let reference = case_ref(id);
            let mut wire = request(&reference, route, Authoring { loads: tip_force(&reference), ..Default::default() });
            let offset = &mut wire["model"]["components"][0]["geometry"]["eccentric_offset"];
            let y = offset["y"].as_f64().unwrap();
            offset["y"] = json!(-y);
            let out = run_linear_static_preview_value_with_mode(wire, PreviewSolverMode::SparseInteractive).unwrap();
            assert_ne!(out.status.mechanics, "MECHANICS_SOLVED", "{id} {route:?}");
            assert!(out.diagnostics.iter().any(|d| d.code == "RIGID_COMPONENT_ECCENTRIC_OFFSET_MISMATCH" && d.message.contains("residual 1e-1 m")),
                "{id} {route:?}: {:?}", out.diagnostics.iter().filter(|d| d.severity == "blocking").map(|d| &d.message).collect::<Vec<_>>());
        }
    }
}
