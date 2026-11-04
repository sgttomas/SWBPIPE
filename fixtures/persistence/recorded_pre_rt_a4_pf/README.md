# Runs recorded before RT-A4 (historical read fixtures)

DEL-05-02 Design `absolute-sum-srss-combinations.md` §10.4. RT-A4 restated the preview-physics
combination policy and limitation 6, so the preview-physics tables' digests moved; a
preview-physics statement carries its table's limitation array, which the readers require to
equal the table's. These are the records the scoped historical read keeps readable. They are
history: `tools/serialization/semantic_contract_tables.py repin` never edits this directory or
`fixtures/results/recorded_pre_rt_a4/` (the table variants they bind).

| Directory | Files | Origin |
|---|---|---|
| `preview_physics_2/` | `preview_physics_connected_sparse.json`, `transient_run_input.json`, `recorded_run_witness.json`, `transient_run_input_rule_packs.json`, `recorded_run_witness_rule_packs.json`, `s2_unresolved_rule_pack_run.json` | Main at 6c3cb46d3, verbatim (`git show 6c3cb46d3:fixtures/results/preview_physics_connected_sparse.json` and `git show 6c3cb46d3:fixtures/persistence/<name>.json`). The desktop's own records of a preview-physics-2 run, binding 1ccc9561… |
| `preview_physics_timoshenko_1/` | `preview_physics_timoshenko_straight_sparse.json` | Main at 6c3cb46d3, verbatim (`fixtures/results/`) |
| | `preview_physics_timoshenko_straight_sparse.analysis_run.json` | Built by main's record builder (below); binds a7b83c37… |
| `preview_physics_retained_1/` | `retained_precision_milestone_successor_sparse_interactive.json` | Main at 6c3cb46d3, verbatim (`fixtures/results/`; `{id, invocation, source}`) |
| | `retained_precision_milestone_successor_sparse_interactive.analysis_run.json` | Built by main's record builder from its `source`; binds b001070e… |
| `physics_retained_1/main/` | `retained_precision_exact_successor_sparse_interactive.json` | Main at 6c3cb46d3, verbatim (`fixtures/results/`) |
| | `retained_precision_exact_successor_sparse_interactive.analysis_run.json` | Built by main's record builder from its `source`; binds cf29fd5f… |
| `physics_retained_1/pr101/` | `retained_precision_exact_successor_sparse_interactive.json` | RT-A3's final constant-effort candidate (#101) at 01e05a486, verbatim |
| | `retained_precision_exact_successor_sparse_interactive.analysis_run.json` | Built by that revision's record builder from its `source`; binds 045f6f0d… |

**Gap.** Main holds no committed record of a preview-physics-timoshenko-1,
preview-physics-retained-1 or physics-retained-1 run; its only recorded runs are the
preview-physics-2 records above. For those three identities the records here were built from
main's (or #101's) verbatim statement by that revision's own product record builder, run in a
temporary checkout of that revision with its own `openpipestress-runner`:

```
python3 -I -c "import sys; sys.path.insert(0, '<checkout>'); ..."   # from <checkout>
  build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest",
                     "ref": "input-manifest:recorded-pre-rt-a4"}, input_manifest_hash="1" * 64)
  validate_analysis_run_v0_3(record, source, expected_basis_refs=record["analysis_run"]["load_basis_refs"])
  json.dumps(record, ensure_ascii=False, indent=1) + "\n"
```

They are transient records (`created_at` null), as main's builder writes before Record Run.
