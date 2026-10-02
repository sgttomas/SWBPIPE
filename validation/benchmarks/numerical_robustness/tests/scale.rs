//! The scale runs' counts and estimate (checkpoint B; plan §13; `src/scale.rs`),
//! checked without a solve.
//! - V-K's O(nnz) pattern and profile counts equal K4's own `StorageCounts`, as
//!   the committed per-case records hold them, on every CI case that K4
//!   factored.
//! - The estimate's terms are ordered as K6b's derivation builds them.
use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
use piping_numerical_robustness::cases::{crate_dir, load_all_described, FAMILY_FILES};
use piping_numerical_robustness::envelope::{
    PopulationPolicy, ReferenceKernelProfile, ReferenceVRProfile, VrEstimateContext, VrInvocation,
};
use piping_numerical_robustness::scale::{counts, estimate};
use serde_json::Value;
use std::collections::BTreeMap;

fn committed_storage() -> BTreeMap<String, (u64, u64)> {
    let mut out = BTreeMap::new();
    for (_, file) in FAMILY_FILES {
        let path = crate_dir()
            .join("observations/kernel_lane")
            .join(file.replace(".jsonl", ".json"));
        let records: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        for r in records.as_array().unwrap() {
            let attempts = r["attempts"].as_array().unwrap();
            if let Some(a) = attempts.first() {
                let s = &a["storage"];
                let pair = (
                    s["pattern_entries"].as_u64().unwrap(),
                    s["profile_entries"].as_u64().unwrap(),
                );
                // Every attempt of a case records the same structure.
                for b in attempts {
                    assert_eq!(b["storage"]["pattern_entries"].as_u64(), Some(pair.0));
                    assert_eq!(b["storage"]["profile_entries"].as_u64(), Some(pair.1));
                }
                out.insert(r["id"].as_str().unwrap().to_string(), pair);
            }
        }
    }
    out
}

#[test]
fn the_counts_equal_k4s_storage_counts_on_every_factored_ci_case() {
    let storage = committed_storage();
    let (cases, family_facts) = load_all_described();
    let mut n = 0;
    for c in cases.iter().filter(|c| !c.is_large()) {
        let Some(&(pattern, profile)) = storage.get(&c.id) else {
            continue;
        };
        let model = c.model.as_ref().unwrap();
        let source = PrimitiveSource::new(model.source_parts()).unwrap();
        let k = counts(model, &source);
        assert_eq!(
            k.pattern_entries as u64, pattern,
            "{}: pattern entries",
            c.id
        );
        assert_eq!(
            k.profile_entries as u64, profile,
            "{}: profile entries",
            c.id
        );
        let family = FAMILY_FILES
            .iter()
            .position(|(f, _)| *f == c.family)
            .unwrap();
        let invocation = VrInvocation::single_case_family_reference(&c.id);
        let context = VrEstimateContext::capture(
            c,
            model,
            &source,
            &k,
            family_facts[family],
            None,
            invocation,
            PopulationPolicy::NodesAndFreeDofsUpper,
        )
        .unwrap();
        drop(source); // The context deliberately does not borrow the source.
        let e = estimate(
            &context,
            &ReferenceKernelProfile::source40129_rust1971_aarch64_v1(),
            &ReferenceVRProfile::source40129_rust1971_aarch64_v1(),
        )
        .unwrap()
        .moving;
        assert!(
            e.model < e.fixed && e.fixed < e.sel128 && e.sel128 <= e.max,
            "{}: {e:?}",
            c.id
        );
        n += 1;
    }
    // Every CI case but RF-MECH's eight refusals, which never factor.
    assert_eq!(n, 193);
}
