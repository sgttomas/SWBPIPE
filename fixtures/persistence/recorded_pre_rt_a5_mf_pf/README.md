# Runs recorded before RT-A5 `minuend_final` (historical read fixtures)

DEL-05-02 Work `operating-case.md` §7. `minuend_final` restated the preview-physics combination
policy (`PREVIEW-PHYSICS-1-COMBINATION-v3` → `-v4`: the held-status gates and the
`held_status_operand` basis), so the preview-physics-2, preview-physics-timoshenko-1 and
preview-physics-retained-1 digests moved (f41b9d3e… → 563db3c7…, 6f649490… → f64473f5…,
b4377e73… → d0bab350…). Rows and limitations did not move. These are main's records and
statements from before it; they read on the recorded-run entries under the registry variant
they bind (`fixtures/results/recorded_pre_rt_a5_mf/`, main's table bytes verbatim). They are
history: `tools/serialization/semantic_contract_tables.py repin` never edits this directory.

| Directory | Files | Origin |
|---|---|---|
| `preview_physics_2/` | `preview_physics_connected_sparse.json`, `transient_run_input.json`, `recorded_run_witness.json`, `transient_run_input_rule_packs.json`, `recorded_run_witness_rule_packs.json`, `s2_unresolved_rule_pack_run.json` | Main at b32a17761, verbatim (`git show b32a17761:fixtures/results/preview_physics_connected_sparse.json`, `git show b32a17761:fixtures/persistence/<name>.json`; identical at 400a409d7). The desktop's own records of a preview-physics-2 run, binding f41b9d3e… |
| `preview_physics_timoshenko_1/` | `preview_physics_timoshenko_straight_sparse.json` | Main at b32a17761, verbatim (`fixtures/results/`) |
| `preview_physics_retained_1/` | `retained_precision_milestone_successor_sparse_interactive.json` | Main at b32a17761, verbatim (`fixtures/results/`; `{id, invocation, source}`) |

**Gap.** As at slice 1b, main holds no committed record (analysis run) of a
preview-physics-timoshenko-1 or preview-physics-retained-1 run; those two identities are
covered here by main's verbatim statements read on the recorded entries under the variant
binding (Rust `recorded_pre_rt_a5_mf.rs`), not by a record built with main's builder.
