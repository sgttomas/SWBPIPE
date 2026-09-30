//! The seeded-fault feature guard (brief Scope 9; T3 D1 §4.10: "CI checks
//! that no product manifest enables the feature"; plan §10.6).
//!
//! `frame_kernel`'s `mutation-controls` feature may be named only where it is
//! defined (FK's `[features]` table, as a key) and in this crate's
//! `[features]` table (as the value of `seeded-faults`); never in a
//! dependency table and never in a `default` list. VR's `seeded-faults`, which
//! enables it, may be named only as that table's key (RV21-1). CI's cargo
//! command passes no features. No VR source names a path under the execution tree, which the
//! CI checkout omits.
use piping_numerical_robustness::cases::crate_dir;
use std::path::{Path, PathBuf};

const FEATURE: &str = "mutation-controls";

fn piping_root() -> PathBuf {
    crate_dir().join("../../..").canonicalize().unwrap()
}

fn manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if path.is_dir() {
            if name.starts_with('.')
                || ["target", "node_modules", "execution"].contains(&name.as_str())
            {
                continue;
            }
            manifests(&path, out);
        } else if name == "Cargo.toml" {
            out.push(path);
        }
    }
}

/// The places a manifest names the feature: (table, line) pairs that are not
/// allowed. `role` is "fk", "vr" or "other".
fn violations(text: &str, role: &str) -> Vec<String> {
    let mut table = String::new();
    let mut out = Vec::new();
    for raw in text.lines() {
        let line = raw.split('#').next().unwrap().trim();
        if line.starts_with('[') {
            table = line.to_string();
            continue;
        }
        // RV21-1: VR's `seeded-faults` enables FK's feature, so naming it is
        // enabling it, in any manifest but as the key of VR's own table.
        let names_it = line.contains(FEATURE) || line.contains("seeded-faults");
        if !names_it {
            continue;
        }
        let key = line.split('=').next().unwrap().trim();
        let allowed = table == "[features]"
            && key != "default"
            && match role {
                "fk" => key == FEATURE,
                "vr" => key == "seeded-faults",
                _ => false,
            };
        if !allowed {
            out.push(format!("{table} {line}"));
        }
    }
    out
}

#[test]
fn no_manifest_enables_the_mutation_controls_feature() {
    let root = piping_root();
    let mut found = Vec::new();
    manifests(&root, &mut found);
    found.sort();
    assert!(found.len() >= 41, "{} manifests", found.len());
    let fk = root.join("core/solver/frame_kernel/Cargo.toml");
    let vr = crate_dir().join("Cargo.toml").canonicalize().unwrap();
    let mut bad = Vec::new();
    for m in &found {
        let m = m.canonicalize().unwrap();
        let role = if m == fk {
            "fk"
        } else if m == vr {
            "vr"
        } else {
            "other"
        };
        for v in violations(&std::fs::read_to_string(&m).unwrap(), role) {
            bad.push(format!("{}: {v}", m.strip_prefix(&root).unwrap().display()));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn ci_passes_no_cargo_features() {
    let text =
        std::fs::read_to_string(piping_root().join("tools/release/check_release_readiness.py"))
            .unwrap();
    assert!(text.contains("\"--offline\""));
    assert!(!text.contains("--features") && !text.contains("--all-features"));
}

#[test]
fn no_vr_source_names_the_execution_tree() {
    let needle = format!("{}/", ["execu", "tion"].concat());
    for dir in ["src", "tests", "examples"] {
        let base = crate_dir().join(dir);
        let Ok(entries) = std::fs::read_dir(&base) else {
            continue;
        };
        for e in entries {
            let p = e.unwrap().path();
            if p.extension().is_some_and(|x| x == "rs") {
                let text = std::fs::read_to_string(&p).unwrap();
                assert!(!text.contains(&needle), "{}", p.display());
            }
        }
    }
}

#[test]
fn the_guard_flags_what_it_must() {
    let enabling = "[package]\nname = \"x\"\n[dependencies]\nopen_pipe_stress_frame_kernel = { path = \"..\", features = [\"mutation-controls\"] }\n";
    assert_eq!(violations(enabling, "other").len(), 1);
    let default_on = "[features]\ndefault = [\"mutation-controls\"]\nmutation-controls = []\n";
    assert_eq!(violations(default_on, "fk").len(), 1);
    assert!(violations("[features]\nmutation-controls = []\n", "fk").is_empty());
    assert_eq!(
        violations("[features]\nmutation-controls = []\n", "other").len(),
        1
    );
    let vr_ok =
        "[features]\nseeded-faults = [\"open_pipe_stress_frame_kernel/mutation-controls\"]\n";
    assert!(violations(vr_ok, "vr").is_empty());
    let vr_default = "[features]\ndefault = [\"seeded-faults\"]\nseeded-faults = [\"open_pipe_stress_frame_kernel/mutation-controls\"]\n";
    assert_eq!(violations(vr_default, "vr").len(), 1);
    let dev = "[dev-dependencies]\nfk = { path = \"x\", features = [\"mutation-controls\"] }\n";
    assert_eq!(violations(dev, "vr").len(), 1);
}

/// RV21-1 (RV21's self-test): enabling VR's `seeded-faults` from another
/// manifest enables FK's `mutation-controls` transitively, and is flagged.
#[test]
fn a_manifest_enabling_vrs_seeded_faults_is_flagged() {
    let dep = "[dependencies]\npiping_numerical_robustness = { path = \"../x\", features = [\"seeded-faults\"] }\n";
    assert_eq!(
        violations(dep, "other").len(),
        1,
        "a dependency on VR with seeded-faults"
    );
    let fwd = "[features]\nx = [\"piping_numerical_robustness/seeded-faults\"]\n";
    assert_eq!(
        violations(fwd, "other").len(),
        1,
        "a feature forwarding to VR's seeded-faults"
    );
}
