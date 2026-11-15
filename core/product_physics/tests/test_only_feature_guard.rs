//! The test-only kernel features reach no shipped build (carrier revision 5
//! §1.1, §7 "Feature guard").
//!
//! The kernel feature `test-synthetic-records` lets a test build inverse-norm
//! records that no verifier produced. Two things must hold:
//! - **Prevented:** the feature reaches no non-test build of a product crate,
//!   by any route cargo has for enabling a dependency's feature.
//! - **Available:** the tests' own use of it remains, through the
//!   dev-dependencies of `product_physics` and `nonlinear_integration`.
//!
//! **The routes, and the metadata fields each is read from**
//! (`cargo metadata --format-version 1 --no-deps`, its `packages[]` entries):
//! - **(a) A non-dev dependency entry that lists the feature:**
//!   `dependencies[]` entries whose `name` is the kernel package, with `kind`
//!   null (normal) or `"build"`, and the feature in `features`. `rename` does
//!   not change `name`. Target-specific entries are included, because every
//!   `target` appears here.
//! - **(b) A feature table that forwards to it:** a `features` value entry
//!   `"<dep>/<feature>"` or `"<dep>?/<feature>"`, where `<dep>` is the name
//!   the package gives the kernel dependency (its `rename`, or else its
//!   `name`). A forwarded feature needs no dependency entry that names it.
//!   Inside the kernel's own table, a feature other than the guarded one
//!   that lists the guarded feature's bare name counts too.
//! - **(c) A default set:** the package's `default` feature, closed over the
//!   package's own feature names, that reaches the guarded feature (in the
//!   kernel) or a forward to it (in any other package).
//! - The fourth way cargo enables a feature is on the command line
//!   (`--features`, `--all-features`). A test cannot see it. The build
//!   commands for shipped binaries are recorded in the pull request.
//!
//! **Resolved feature sets are not used.** `resolve.nodes[].features` unifies
//! the root package's dev-dependencies: at the `product_physics` root it lists
//! `test-synthetic-records`, because the tests enable it. At a shipped root,
//! such as the desktop app or a wasm engine, it would be meaningful. But it
//! needs that root's whole registry graph, which this crate's offline tests
//! do not fetch. The declaration routes need only `--no-deps`, and so read
//! every package's own manifest, offline, with no registry.
//!
//! The check is one function over metadata values. Its controls give it
//! doctored packages, one per route, each refused with that route named. The
//! live test passes it every package declared under `core/` and `apps/`.

use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The guarded kernel features. Milestone 2 adds `test-reject-seam` here.
const GUARDED: &[&str] = &["test-synthetic-records"];
const KERNEL: &str = "open_pipe_stress_frame_kernel";
/// The test graphs that keep the guarded features, through dev-dependencies only.
const AVAILABLE_TO: &[&str] = &["open_pipe_stress_product_physics", "open_pipe_stress_nonlinear_integration"];

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Route {
    NonDevDependency,
    FeatureForward,
    DefaultSet,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Finding {
    route: Route,
    package: String,
    detail: String,
}

fn str_of<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

fn dependencies(package: &Value) -> impl Iterator<Item = &Value> {
    package.get("dependencies").and_then(Value::as_array).into_iter().flatten()
}

fn feature_entries<'a>(package: &'a Value, feature: &str) -> Vec<&'a str> {
    package
        .get("features")
        .and_then(|f| f.get(feature))
        .and_then(Value::as_array)
        .map(|list| list.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn feature_names(package: &Value) -> Vec<String> {
    package.get("features").and_then(Value::as_object).map(|m| m.keys().cloned().collect()).unwrap_or_default()
}

/// The names by which `package` refers to the kernel in its feature table.
fn kernel_aliases(package: &Value) -> BTreeSet<String> {
    dependencies(package)
        .filter(|d| str_of(d, "name") == KERNEL)
        .map(|d| d.get("rename").and_then(Value::as_str).unwrap_or(KERNEL).to_string())
        .collect()
}

/// Whether one feature-table entry of `package` enables a guarded feature.
fn entry_enables(package: &Value, entry: &str, guarded: &[&str]) -> Option<String> {
    if let Some((dep, feature)) = entry.split_once('/') {
        let dep = dep.trim_end_matches('?');
        if kernel_aliases(package).contains(dep) && guarded.contains(&feature) {
            return Some(feature.to_string());
        }
    } else if str_of(package, "name") == KERNEL && guarded.contains(&entry) {
        return Some(entry.to_string());
    }
    None
}

/// The routes by which the guarded features could reach a non-test build.
fn findings(packages: &[Value], guarded: &[&str]) -> Vec<Finding> {
    let mut out = Vec::new();
    for package in packages {
        let name = str_of(package, "name").to_string();
        // (a) non-dev dependency entries.
        for dep in dependencies(package) {
            let kind = dep.get("kind").and_then(Value::as_str);
            if str_of(dep, "name") == KERNEL && kind != Some("dev") {
                for feature in dep.get("features").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str) {
                    if guarded.contains(&feature) {
                        let kind = kind.unwrap_or("normal");
                        out.push(Finding { route: Route::NonDevDependency, package: name.clone(), detail: format!("{kind} dependency on {KERNEL} lists `{feature}`") });
                    }
                }
            }
        }
        // (b) feature-table forwards, other than the default set and the guarded feature's own definition.
        for feature in feature_names(package) {
            if feature == "default" || (str_of(package, "name") == KERNEL && guarded.contains(&feature.as_str())) {
                continue;
            }
            for entry in feature_entries(package, &feature) {
                if let Some(target) = entry_enables(package, entry, guarded) {
                    out.push(Finding { route: Route::FeatureForward, package: name.clone(), detail: format!("feature `{feature}` = \"{entry}\" enables `{target}`") });
                }
            }
        }
        // (c) the default set, closed over the package's own features.
        let mut seen = BTreeSet::new();
        let mut stack = vec!["default".to_string()];
        while let Some(feature) = stack.pop() {
            if !seen.insert(feature.clone()) {
                continue;
            }
            for entry in feature_entries(package, &feature) {
                if let Some(target) = entry_enables(package, entry, guarded) {
                    out.push(Finding { route: Route::DefaultSet, package: name.clone(), detail: format!("`default` reaches \"{entry}\" (through `{feature}`), enabling `{target}`") });
                } else if !entry.contains('/') && !entry.starts_with("dep:") {
                    stack.push(entry.to_string());
                }
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The tests' own use: each package of `AVAILABLE_TO` has a dev-dependency on
/// the kernel listing every guarded feature.
fn availability(packages: &[Value], guarded: &[&str]) -> Vec<String> {
    let mut problems = Vec::new();
    for wanted in AVAILABLE_TO {
        let Some(package) = packages.iter().find(|p| str_of(p, "name") == *wanted) else {
            problems.push(format!("{wanted}: not found"));
            continue;
        };
        for feature in guarded {
            let ok = dependencies(package).any(|d| {
                str_of(d, "name") == KERNEL
                    && d.get("kind").and_then(Value::as_str) == Some("dev")
                    && d.get("features").and_then(Value::as_array).into_iter().flatten().any(|f| f.as_str() == Some(feature))
            });
            if !ok {
                problems.push(format!("{wanted}: no dev-dependency on {KERNEL} lists `{feature}`"));
            }
        }
    }
    problems
}

// ---------------------------------------------------------------- controls

fn dep(name: &str, kind: Option<&str>, features: &[&str], rename: Option<&str>) -> Value {
    json!({ "name": name, "kind": kind, "features": features, "rename": rename, "target": null, "optional": false, "uses_default_features": true })
}

/// A clean graph: the kernel defines the feature, the product's tests enable
/// it through dev-dependencies, and an app depends on the kernel without it.
fn clean() -> Vec<Value> {
    vec![
        json!({ "name": KERNEL, "dependencies": [], "features": { "test-synthetic-records": [], "default": [] } }),
        json!({ "name": "open_pipe_stress_product_physics", "features": {},
                "dependencies": [dep(KERNEL, None, &[], None), dep(KERNEL, Some("dev"), &["test-synthetic-records"], None)] }),
        json!({ "name": "open_pipe_stress_nonlinear_integration", "features": {},
                "dependencies": [dep(KERNEL, None, &[], None), dep(KERNEL, Some("dev"), &["test-synthetic-records"], None)] }),
        json!({ "name": "app", "features": { "default": [] }, "dependencies": [dep(KERNEL, None, &[], None)] }),
    ]
}

fn with(mut packages: Vec<Value>, name: &str, edit: impl FnOnce(&mut Value)) -> Vec<Value> {
    edit(packages.iter_mut().find(|p| str_of(p, "name") == name).unwrap());
    packages
}

fn routes(packages: &[Value]) -> Vec<Route> {
    findings(packages, GUARDED).into_iter().map(|f| f.route).collect()
}

#[test]
fn the_clean_graph_passes_and_keeps_its_dev_use() {
    assert_eq!(findings(&clean(), GUARDED), vec![]);
    assert_eq!(availability(&clean(), GUARDED), Vec::<String>::new());
}

#[test]
fn each_route_is_refused_by_name() {
    // (a) a normal dependency entry that lists the feature.
    let a = with(clean(), "app", |p| p["dependencies"][0]["features"] = json!(["test-synthetic-records"]));
    assert_eq!(routes(&a), vec![Route::NonDevDependency], "{:?}", findings(&a, GUARDED));
    // (a) a build dependency, which is a non-test build too.
    let a_build = with(clean(), "app", |p| p["dependencies"] = json!([dep(KERNEL, Some("build"), &["test-synthetic-records"], None)]));
    assert_eq!(routes(&a_build), vec![Route::NonDevDependency]);
    // (b) a feature table forwarding to it, with no dependency entry naming it.
    let b = with(clean(), "app", |p| p["features"]["extras"] = json!([format!("{KERNEL}/test-synthetic-records")]));
    assert_eq!(routes(&b), vec![Route::FeatureForward], "{:?}", findings(&b, GUARDED));
    // (b) the weak form through a renamed dependency.
    let b_weak = with(clean(), "app", |p| {
        p["dependencies"] = json!([dep(KERNEL, None, &[], Some("fk"))]);
        p["features"]["extras"] = json!(["fk?/test-synthetic-records"]);
    });
    assert_eq!(routes(&b_weak), vec![Route::FeatureForward], "{:?}", findings(&b_weak, GUARDED));
    // (b) inside the kernel: another feature that turns it on.
    let b_internal = with(clean(), KERNEL, |p| p["features"]["convenience"] = json!(["test-synthetic-records"]));
    assert_eq!(routes(&b_internal), vec![Route::FeatureForward]);
    // (c) the kernel's default set.
    let c = with(clean(), KERNEL, |p| p["features"]["default"] = json!(["test-synthetic-records"]));
    assert_eq!(routes(&c), vec![Route::DefaultSet], "{:?}", findings(&c, GUARDED));
    // (c) another package's default set forwarding to it.
    let c_app = with(clean(), "app", |p| p["features"]["default"] = json!([format!("{KERNEL}/test-synthetic-records")]));
    assert_eq!(routes(&c_app), vec![Route::DefaultSet]);
    // (c) reached through an intermediate feature: both routes are named.
    let c_through = with(clean(), "app", |p| {
        p["features"]["default"] = json!(["extras"]);
        p["features"]["extras"] = json!([format!("{KERNEL}/test-synthetic-records")]);
    });
    assert_eq!(routes(&c_through), vec![Route::FeatureForward, Route::DefaultSet], "{:?}", findings(&c_through, GUARDED));
    // A dev-dependency listing it is the permitted use, not a finding.
    let dev = with(clean(), "app", |p| p["dependencies"] = json!([dep(KERNEL, Some("dev"), &["test-synthetic-records"], None)]));
    assert_eq!(findings(&dev, GUARDED), vec![]);
}

#[test]
fn losing_the_tests_dev_use_is_refused() {
    let lost = with(clean(), "open_pipe_stress_product_physics", |p| p["dependencies"][1]["features"] = json!([]));
    assert_eq!(availability(&lost, GUARDED), vec![format!("open_pipe_stress_product_physics: no dev-dependency on {KERNEL} lists `test-synthetic-records`")]);
}

// ---------------------------------------------------------------- live

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

fn manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if path.is_dir() {
            if !name.starts_with('.') && !["target", "node_modules"].contains(&name.as_str()) {
                manifests(&path, out);
            }
        } else if name == "Cargo.toml" {
            out.push(path);
        }
    }
}

/// Every package declared under `core/` and `apps/`, from `cargo metadata
/// --no-deps` (offline, no registry), one call per manifest: the repository
/// has no single cargo workspace.
fn declared_packages() -> Vec<Value> {
    let root = repository_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut paths = Vec::new();
    for area in ["core", "apps"] {
        manifests(&root.join(area), &mut paths);
    }
    paths.sort();
    let mut packages = Vec::new();
    for manifest in &paths {
        let output = Command::new(&cargo)
            .args(["metadata", "--format-version", "1", "--no-deps", "--offline", "--manifest-path"])
            .arg(manifest)
            .output()
            .expect("cargo metadata runs");
        assert!(output.status.success(), "cargo metadata failed for {}: {}", manifest.display(), String::from_utf8_lossy(&output.stderr));
        let metadata: Value = serde_json::from_slice(&output.stdout).unwrap();
        packages.extend(metadata["packages"].as_array().unwrap().iter().cloned());
    }
    packages
}

#[test]
fn test_only_kernel_features_reach_no_shipped_build() {
    let packages = declared_packages();
    let kernel = packages.iter().find(|p| str_of(p, "name") == KERNEL).expect("the kernel is declared under core/");
    for feature in GUARDED {
        assert!(kernel["features"].get(feature).is_some(), "the kernel no longer defines `{feature}`: update GUARDED");
    }
    assert_eq!(findings(&packages, GUARDED), vec![], "a guarded test-only feature can reach a non-test build");
    assert_eq!(availability(&packages, GUARDED), Vec::<String>::new(), "the tests lost their dev-dependency use");
}
