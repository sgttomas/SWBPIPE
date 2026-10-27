//! W-15 slice c product-path benchmarks (DEL-03-05 Design `eccentric-reducer-offset.md`
//! §7; PRD §22.2): the eccentric step under an axial tip force (P·‖D⊥‖ in the
//! upstream run), the z-offset step under a transverse force (torsion), the
//! concentric control, free and anchored thermal growth, at s = 1 and 1e3,
//! Euler-Bernoulli on both routes and Timoshenko on the exact route.
//!
//! Every expectation is read from the frozen references in
//! `validation/references/w15c_eccentric_reducer/references.json` (two exact
//! formulations that agree; see that package's README), pinned here to the hash in
//! its SHA256SUMS. Nothing here derives an expectation from product code. Inputs are
//! invented. The reducer carries its offset under R-c1 option A (from the
//! component's node); the convention rows W15C-R and W15C-O are bound in
//! `w15c_offset_convention.rs`.
//!
//! Criterion (DEC-024/DEC-026, unchanged): |obs - exp| <= 1e-9 |exp|; an exactly zero
//! expectation uses 1e-9 times the case's frozen zero scale of its family. Both solver
//! modes run against the same reference, and the product's readers admit every envelope.
#[path = "common/w15c_bench.rs"]
mod common_w15c;
use common_w15c::*;
use sha2::{Digest, Sha256};

const SUMS: &str = include_str!("../../../validation/references/w15c_eccentric_reducer/SHA256SUMS");

#[test]
fn references_are_the_frozen_bytes() {
    let digest: String = Sha256::digest(REFERENCES.as_bytes()).iter().map(|b| format!("{b:02x}")).collect();
    let line = SUMS.lines().find(|l| l.ends_with("  references.json")).expect("SHA256SUMS lists references.json");
    assert_eq!(&line[..64], digest, "the references must equal the frozen bytes");
    assert!(refs()["crosscheck"].as_array().unwrap().iter().all(|c| c["agrees"] == true));
}

#[test]
fn w15c_p_axial_step_and_concentric_control_euler_bernoulli_both_routes() {
    for id in ["W15C-P-S1-EB", "W15C-P-S1000-EB", "W15C-P0-S1-EB", "W15C-P0-S1000-EB"] {
        for route in [Route::Preview, Route::Exact] {
            run(id, route, |r| Authoring { loads: tip_force(r), ..Default::default() });
        }
    }
}

#[test]
fn w15c_z_torsion_step_euler_bernoulli_both_routes() {
    for id in ["W15C-Z-S1-EB", "W15C-Z-S1000-EB"] {
        for route in [Route::Preview, Route::Exact] {
            run(id, route, |r| Authoring { loads: tip_force(r), ..Default::default() });
        }
    }
}

#[test]
fn w15c_p_and_z_timoshenko_exact_route() {
    for id in ["W15C-P-S1-TIM", "W15C-P-S1000-TIM", "W15C-Z-S1-TIM", "W15C-Z-S1000-TIM"] {
        run(id, Route::Exact, |r| Authoring { loads: tip_force(r), timoshenko: true, ..Default::default() });
    }
}

#[test]
fn w15c_t_free_thermal_growth_both_routes() {
    for route in [Route::Preview, Route::Exact] {
        run("W15C-T-S1000", route, |_| Authoring { thermal: true, ..Default::default() });
    }
}

#[test]
fn w15c_tf_anchored_thermal_growth() {
    for id in ["W15C-TF-S1-EB", "W15C-TF-S1000-EB"] {
        for route in [Route::Preview, Route::Exact] {
            run(id, route, |_| Authoring { thermal: true, ..Default::default() });
        }
    }
    for id in ["W15C-TF-S1-TIM", "W15C-TF-S1000-TIM"] {
        run(id, Route::Exact, |_| Authoring { thermal: true, timoshenko: true, ..Default::default() });
    }
}
