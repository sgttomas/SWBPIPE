//! The combination contract fixtures under `fixtures/combination/` (unit S4).
//!
//! Each fixture is an input document and the output document the combinations give for it. The
//! inputs are built here from frozen references' published signed contributions (or from such
//! an input with one stated mutation); the outputs are the product's own earlier output and only
//! guard against change. The values the outputs agree with are checked against the references
//! in `f2_combinations.rs`, not here.
//!
//! The test fails when a committed fixture differs from what it rebuilds. After a deliberate
//! change, regenerate with `COMBINATION_FIXTURES_BLESS=1 cargo test --test contract_fixtures`
//! and review the diff. The committed fixtures are read with `include_str!`, a compile-time read
//! that CI routing resolves from the source.
mod support;

use open_pipe_stress_response_spectrum::combination_json::{combine_json, request_from_json, request_to_json};
use open_pipe_stress_response_spectrum::{CombinationRequest, Direction, DirectionalRule, ModalRule, ModeIndex, RigidRule};
use serde_json::Value;
use std::path::PathBuf;
use support::*;

const C4: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c4_close_pair.json");
const C9: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c9_pipe_diagonal.json");
const C10: &str = include_str!("../../../../validation/references/f2_spectrum/discrete/c10_cluster_witnesses.json");

const NOT_A_RESULT: &str = "This is a combination-unit contract fixture: its contributions were not produced by the \
product, it is not a spectrum solve result or a spectrum-1 document, and it has no standing as an engineering result.";

fn provenance(source: &str, analysis: &str, change: &str) -> String {
    format!("Inputs: the published signed contributions, modal metadata (mode, scope cluster, frequency) and rigid \
responses of analysis {analysis} of {source} (frozen), each as its binary64. {change}{NOT_A_RESULT}")
}

fn analysis(text: &str, case: usize) -> (Value, Value) {
    let doc: Value = serde_json::from_str(text).unwrap();
    let case = doc["cases"][case].clone();
    let analysis = case["analyses"][0].clone();
    (case, analysis)
}

struct Fixture {
    name: &'static str,
    request: CombinationRequest,
    provenance: String,
}

fn fixtures() -> Vec<Fixture> {
    let mut out = Vec::new();
    let discrete = "validation/references/f2_spectrum/discrete/";

    // C4-BASE: CQC on a close pair with opposite signs, rigid response by absolute sum, 100-40-40.
    let (case, a) = analysis(C4, 0);
    let c4 = discrete_analysis(&case, &a, None);
    let base = c4.request(ModalRule::Cqc, Some(RigidRule::AbsoluteSum), DirectionalRule::HundredFortyForty);
    out.push(Fixture { name: "c4_base", request: base.clone(), provenance: provenance(&format!("{discrete}c4_close_pair.json"), "C4-BASE", "") });
    let mut flipped = base.clone();
    let i = c4.keys.iter().position(|k| k == "N3.uy").unwrap();
    let entry = flipped.periodic.iter_mut().find(|p| p.mode == ModeIndex(2) && p.direction == Direction::X).unwrap();
    entry.values[i] = -entry.values[i];
    out.push(Fixture {
        name: "c4_mutated_sign",
        request: flipped,
        provenance: provenance(&format!("{discrete}c4_close_pair.json"), "C4-BASE",
            "Mutation: the sign of mode 2's X contribution to N3.uy is reversed, so CQC of N3.uy changes. "),
    });
    let mut zero = base.clone();
    zero.damping_ratio = 0.0;
    out.push(Fixture {
        name: "c4_refused_cqc_zero_damping",
        request: zero,
        provenance: provenance(&format!("{discrete}c4_close_pair.json"), "C4-BASE", "Mutation: the damping ratio is 0 under CQC. "),
    });

    // C10-W3: clusters of two unequal frequencies, the first straddling f_r, periodic as a whole.
    let (case, a) = analysis(C10, 1);
    let w3 = discrete_analysis(&case, &a, None);
    let base = w3.request(ModalRule::Srss, Some(RigidRule::Srss), DirectionalRule::Srss);
    let source = format!("{discrete}c10_cluster_witnesses.json");
    out.push(Fixture { name: "c10_w3_straddling_cluster", request: base.clone(), provenance: provenance(&source, "C10-W3", "") });
    let mut moved = base.clone();
    moved.modes[1].cluster.0 = 2;
    moved.modes[2].cluster.0 = 3;
    moved.modes[3].cluster.0 = 3;
    out.push(Fixture {
        name: "c10_w3_refused_cluster_split",
        request: moved,
        provenance: provenance(&source, "C10-W3",
            "Mutation: cluster 1 is split, mode 2 standing alone above f_r, so its contributions are those of a rigid cluster. "),
    });
    let mut premise = base;
    for r in &mut premise.rigid {
        r.periodic_clusters_left_out.clear();
    }
    out.push(Fixture {
        name: "c10_w3_refused_rigid_premise",
        request: premise,
        provenance: provenance(&source, "C10-W3", "Mutation: the rigid responses state that their load left out no periodic cluster. "),
    });

    // C9-Y in member axes: rows whose frame is the member's.
    let (case, a) = analysis(C9, 0);
    let c9 = discrete_analysis(&case, &a, Some("element_end_forces_member_axes"));
    out.push(Fixture {
        name: "c9_y_member_axes",
        request: c9.request(ModalRule::Cqc, None, DirectionalRule::Srss),
        provenance: provenance(&format!("{discrete}c9_pipe_diagonal.json"), "C9-Y (element_end_forces_member_axes)", ""),
    });
    out
}

/// The committed fixtures, read at compile time.
const COMMITTED: [(&str, &str, &str); 7] = [
    ("c4_base", include_str!("../fixtures/combination/c4_base.input.json"), include_str!("../fixtures/combination/c4_base.output.json")),
    ("c4_mutated_sign", include_str!("../fixtures/combination/c4_mutated_sign.input.json"), include_str!("../fixtures/combination/c4_mutated_sign.output.json")),
    ("c4_refused_cqc_zero_damping", include_str!("../fixtures/combination/c4_refused_cqc_zero_damping.input.json"), include_str!("../fixtures/combination/c4_refused_cqc_zero_damping.output.json")),
    ("c10_w3_straddling_cluster", include_str!("../fixtures/combination/c10_w3_straddling_cluster.input.json"), include_str!("../fixtures/combination/c10_w3_straddling_cluster.output.json")),
    ("c10_w3_refused_cluster_split", include_str!("../fixtures/combination/c10_w3_refused_cluster_split.input.json"), include_str!("../fixtures/combination/c10_w3_refused_cluster_split.output.json")),
    ("c10_w3_refused_rigid_premise", include_str!("../fixtures/combination/c10_w3_refused_rigid_premise.input.json"), include_str!("../fixtures/combination/c10_w3_refused_rigid_premise.output.json")),
    ("c9_y_member_axes", include_str!("../fixtures/combination/c9_y_member_axes.input.json"), include_str!("../fixtures/combination/c9_y_member_axes.output.json")),
];

fn committed(name: &str) -> (&'static str, &'static str) {
    COMMITTED.iter().find(|(n, _, _)| *n == name).map(|(_, i, o)| (*i, *o)).unwrap_or_else(|| panic!("no committed fixture {name}"))
}

fn path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/combination").join(name)
}

fn pretty(v: &Value) -> String {
    serde_json::to_string_pretty(v).unwrap() + "\n"
}

#[test]
fn fixtures_are_current() {
    let bless = std::env::var("COMBINATION_FIXTURES_BLESS").is_ok();
    let mut stale = Vec::new();
    for f in fixtures() {
        let input = pretty(&request_to_json(&f.request, &f.provenance));
        // The input document decodes to the same request.
        let (_, decoded) = request_from_json(&input).unwrap();
        assert_eq!(decoded.unwrap(), f.request, "{}", f.name);
        let output = pretty(&combine_json(&input).unwrap());
        let (committed_input, committed_output) = committed(f.name);
        for (file, text, committed) in [
            (format!("{}.input.json", f.name), &input, committed_input),
            (format!("{}.output.json", f.name), &output, committed_output),
        ] {
            if bless {
                let p = path(&file);
                std::fs::create_dir_all(p.parent().unwrap()).unwrap();
                std::fs::write(&p, text).unwrap();
            } else if committed != text.as_str() {
                stale.push(file);
            }
        }
    }
    assert_eq!(fixtures().len(), COMMITTED.len());
    assert!(stale.is_empty(), "stale fixtures (regenerate with COMBINATION_FIXTURES_BLESS=1 and review): {stale:?}");
}

#[test]
fn fixtures_state_their_outcome() {
    let outcome = |name: &str| -> Value {
        serde_json::from_str(committed(name).1).unwrap()
    };
    for (name, expected) in [
        ("c4_base", "combined"),
        ("c4_mutated_sign", "combined"),
        ("c10_w3_straddling_cluster", "combined"),
        ("c9_y_member_axes", "combined"),
        ("c4_refused_cqc_zero_damping", "SPECTRUM_CQC_DAMPING_UNSUPPORTED"),
        ("c10_w3_refused_cluster_split", "SPECTRUM_CONTRIBUTION_NOT_PERIODIC"),
        ("c10_w3_refused_rigid_premise", "SPECTRUM_RIGID_RESPONSE_PREMISE_MISMATCH"),
    ] {
        let o = outcome(name);
        if expected == "combined" {
            assert_eq!(o["outcome"], "combined", "{name}");
            assert!(o["rows"].as_array().unwrap().iter().all(|r| r["sign_status"] == "unsigned_combined_magnitude"));
        } else {
            assert_eq!(o["outcome"], "refused", "{name}");
            assert!(o["refusals"].as_array().unwrap().iter().all(|r| r["code"] == expected), "{name}: {o}");
        }
        assert!(o["provenance"].as_str().unwrap().contains("not a spectrum solve result"));
    }
    // The sign mutation changes CQC of N3.uy under X and nothing under Y.
    let (base, flipped) = (outcome("c4_base"), outcome("c4_mutated_sign"));
    let row = |o: &Value| o["rows"].as_array().unwrap().iter().find(|r| r["node"] == "N3" && r["component"] == "uy").unwrap().clone();
    assert_ne!(row(&base)["per_direction"][0]["periodic"], row(&flipped)["per_direction"][0]["periodic"]);
    assert_eq!(row(&base)["per_direction"][1], row(&flipped)["per_direction"][1]);
}
