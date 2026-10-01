1: //! The scale runs' counts and estimate (checkpoint B; plan §13; `src/scale.rs`),
2: //! checked without a solve.
3: //! - V-K's O(nnz) pattern and profile counts equal K4's own `StorageCounts`, as
4: //!   the committed per-case records hold them, on every CI case that K4
5: //!   factored.
6: //! - The estimate's terms are ordered as K6b's derivation builds them.
7: use open_pipe_stress_frame_kernel::structural::retained_api::PrimitiveSource;
8: use piping_numerical_robustness::cases::{crate_dir, load_all, FAMILY_FILES};
9: use piping_numerical_robustness::scale::{counts, estimate, Sizes};
10: use serde_json::Value;
11: use std::collections::BTreeMap;
12: 
13: fn committed_storage() -> BTreeMap<String, (u64, u64)> {
14:     let mut out = BTreeMap::new();
15:     for (_, file) in FAMILY_FILES {
16:         let path = crate_dir()
17:             .join("observations/kernel_lane")
18:             .join(file.replace(".jsonl", ".json"));
19:         let records: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
20:         for r in records.as_array().unwrap() {
21:             let attempts = r["attempts"].as_array().unwrap();
22:             if let Some(a) = attempts.first() {
23:                 let s = &a["storage"];
24:                 let pair = (
25:                     s["pattern_entries"].as_u64().unwrap(),
26:                     s["profile_entries"].as_u64().unwrap(),
27:                 );
28:                 // Every attempt of a case records the same structure.
29:                 for b in attempts {
30:                     assert_eq!(b["storage"]["pattern_entries"].as_u64(), Some(pair.0));
31:                     assert_eq!(b["storage"]["profile_entries"].as_u64(), Some(pair.1));
32:                 }
33:                 out.insert(r["id"].as_str().unwrap().to_string(), pair);
34:             }
35:         }
36:     }
37:     out
38: }
39: 
40: #[test]
41: fn the_counts_equal_k4s_storage_counts_on_every_factored_ci_case() {
42:     let storage = committed_storage();
43:     let sizes = Sizes::of_this_build();
44:     let mut n = 0;
45:     for c in load_all().iter().filter(|c| !c.is_large()) {
46:         let Some(&(pattern, profile)) = storage.get(&c.id) else {
47:             continue;
48:         };
49:         let model = c.model.as_ref().unwrap();
50:         let source = PrimitiveSource::new(model.source_parts()).unwrap();
51:         let k = counts(model, &source);
52:         assert_eq!(
53:             k.pattern_entries as u64, pattern,
54:             "{}: pattern entries",
55:             c.id
56:         );
57:         assert_eq!(
58:             k.profile_entries as u64, profile,
59:             "{}: profile entries",
60:             c.id
61:         );
62:         let e = estimate(&k, &sizes);
63:         assert!(
64:             e.model < e.fixed && e.fixed < e.sel128 && e.sel128 <= e.max,
65:             "{}: {e:?}",
66:             c.id
67:         );
68:         n += 1;
69:     }
70:     // Every CI case but RF-MECH's eight refusals, which never factor.
71:     assert_eq!(n, 193);
72: }
