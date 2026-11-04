# Runs recorded before operating-case slice 1b (historical read fixtures)

DEL-05-02 Work `operating-case.md`, "As built (1b)". Slice 1b restated the preview-physics
combination policy (`PREVIEW-PHYSICS-1-COMBINATION-v2` → `-v3`: the declared difference basis's
gates G-1 and G-2), so the preview-physics-2, preview-physics-timoshenko-1 and
preview-physics-retained-1 digests moved (e7b93f41… → f41b9d3e…, ce49ebe0… → 6f649490…,
26dce9c2… → b4377e73…). Rows and limitations did not move. These are main's records and
statements from before 1b; they read on the recorded-run entries under the registry variant
they bind (`fixtures/results/recorded_pre_rt_a5_1b/`, main's table bytes verbatim). They are
history: `tools/serialization/semantic_contract_tables.py repin` never edits this directory.

| Directory | Files | Origin |
|---|---|---|
| `preview_physics_2/` | `preview_physics_connected_sparse.json`, `transient_run_input.json`, `recorded_run_witness.json`, `transient_run_input_rule_packs.json`, `recorded_run_witness_rule_packs.json`, `s2_unresolved_rule_pack_run.json` | Main at bbf384dec, verbatim (`git show bbf384dec:fixtures/results/preview_physics_connected_sparse.json`, `git show bbf384dec:fixtures/persistence/<name>.json`; identical at 800fae7e5). The desktop's own records of a preview-physics-2 run, binding e7b93f41… |
| `preview_physics_timoshenko_1/` | `preview_physics_timoshenko_straight_sparse.json` | Main at bbf384dec, verbatim (`fixtures/results/`) |
| `preview_physics_retained_1/` | `retained_precision_milestone_successor_sparse_interactive.json` | Main at bbf384dec, verbatim (`fixtures/results/`; `{id, invocation, source}`) |

**Gap.** As at RT-A4, main holds no committed record (analysis run) of a
preview-physics-timoshenko-1 or preview-physics-retained-1 run; those two identities are
covered here by main's verbatim statements read on the recorded entries under the variant
binding (Rust `recorded_pre_rt_a5_1b.rs`), not by a record built with main's builder.
