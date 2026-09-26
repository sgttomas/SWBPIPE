# TASK — load-reference-1 readers in result_export and analysis_runs

Parent of record: the session-2 WORKING_ITEMS load-state manager (SendMessage id
`a3675abb28ada0834`). ROOT spawns this TASK on the manager's request. Report to
the manager by SendMessage and send your final report to ROOT. Do not delegate
further. Paths are WORKING_ROOT-relative (`WORKING_ROOT = <checkout>/projects/chirality-piping`,
checkout = the manager's load-state worktree, branch `codex/piping-load-states-20260925`).
`LSI` = `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.

## Objective

The producer now publishes raw 0.2.0 envelopes for model 0.4.0. Each carries
producer `semantic_contract_id` `openpipestress.result_semantics/0.3.0/load-reference-1`,
`formulation_basis.profile_id` `resolved_straight_load_state_v1`, and
`contract_evidence` with `pressure`, `connector`, `exact_cases` and the new
`load_reference_states`. Today every consumer rejects these envelopes as
`SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`.

Add an explicit, closed new-method branch to each existing reader:

1. **Rust `core/reporting/result_export`.**
   - Register the pinned table `fixtures/results/semantic_contract_v0_3_load_reference_1.json`,
     beside `physics_contract()`.
   - Accept `load-reference-1` in header dispatch (`for_source_metadata`), bound to
     profile `resolved_straight_load_state_v1`. This is the only profile accepted for
     it, and no other contract may claim that profile.
   - Add a closed raw-evidence validator (`validate_load_reference_evidence`).
     Dispatch `for_source` to it.
   - Produce the canonical results 0.3, stress-neutral 0.3 and derivative outputs
     the same way physics-1 does, by preserving the source metadata and evidence
     verbatim under the existing metadata policy.
2. **Python `core/analysis_runs`.**
   - Register the contract ID, the table sha256 and the path in `compatibility.py`.
   - Add a closed validator module mirroring the Rust one.
   - Create AnalysisRun v0.3 records for load-reference-1 as physics-1 does.
3. The Rust and Python validators must accept and reject the same inputs. Prove
   it with shared adversarial cases.

The validator must enforce everything in `LSI/CP2_WIRE_ADDENDUM_1.md` §3 as
closed shapes: exact key sets, finite numbers, and the stated enums.

It must also cross-bind the evidence:

- each `numerical_quality.cases` load case has exactly one
  `load_reference_states` record (by `load_case_id`) and one `exact_cases`
  record;
- `members[]` is a bijection with the pipes named in `exact_cases.pipe_sections`;
- `exact_cases[i].pipe_materials` has E/nu/G equal to that case's
  `members[].selected_E_pa`/`selected_nu`/`derived_G_pa`;
- every pressure-region material equals the member pair of its case;
- `exact_cases[i].material_basis` is `resolved_per_member_load_reference_state_v1`;
- `source_recovery` is `{status: not_joined, code: LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED}`;
- exactly one info diagnostic `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED` names each case;
- `source_block_recovery` is absent;
- every `contributions[].source_id` is unique;
- every `stored_primitive` contribution has a finite nonzero `factor`;
- `excluded_sources` IDs are disjoint from the included ones.

Result rows use the table's physics-1 signatures unchanged.

The following keep their meanings, bytes and hashes unchanged:

- every existing semantic table;
- existing dispatch for precision-1, physics-1, source-blocks-1 and physics-source-1;
- existing fixtures;
- existing tests.

Also reject:

- a load-reference-1 envelope that carries a source receipt;
- a physics-1 envelope that carries `load_reference_states`;
- any profile or contract mix-up.

## Frozen inputs (read-only; sha256)

| Input | sha256 |
|---|---|
| `LSI/CP2_WIRE.md` | `81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996` |
| `LSI/CP2_WIRE_ADDENDUM_1.md` | `c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760` |
| `fixtures/results/semantic_contract_v0_3_load_reference_1.json` (manager-owned, derived from physics-1 by `LSI/_run_records/session2/cp3_make_semantic_table.py`) | `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d` |
| `fixtures/product_preview/load_reference/connected.request.json` | `ff0ce8724d7734c2cd8d10ce866085d80741adadc0da529521084dc8382453a2` |
| `fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json` | `6c1dd176e5600bdbec08066345372a26459746ed34d665fa4767569594dd03bf` |
| `fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json` | `333515b39d15e1fc270b40f267d6ff47c5de24acd733ee52660d4488a3c31c11` |
| `fixtures/product_preview/load_reference/pressure.request.json` | `52375ad60d05074ed6064b72d78d9c24d19b1a53915bcc46c2b424be7c39ef69` |
| `fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json` | `20802f97f3e00fe7043d3adf5d78e926270f0e0f9c80f4b724ef4d122fc9ab75` |
| `fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json` | `8215cad29a51e0e9d59f90b5db014256787592cf514b82ccef56fd814ce94be3` |

The raw envelopes are actual producer output at the checkpoint-2 route, generated with
`core/product_physics/examples/physics_source_connected.rs`. They are not hand-written.
Producer source (read-only reference): `core/product_physics/src/{lib.rs,case_state/*}`.

## Exclusive write boundary

- `core/reporting/result_export/**` (src, tests, tests/fixtures)
- `core/analysis_runs/**`
- new `tests/test_load_reference_readers.py`
- new derived fixtures `fixtures/results/load_reference_*`: canonical results,
  stress-neutral and AnalysisRun outputs derived from the frozen raw envelopes by
  your readers
- evidence `LSI/CP3_READERS/` (RETURN.md, `_run_records/`)

Everything else is off limits. In particular, `schemas/**` belongs to the
parallel schema TASK. For its carrier branches, you rely on your outputs
preserving raw `contract_evidence` verbatim.

The existing physics readers load `$defs` from `schemas/results.v0.3.schema.yaml`
for transport-shape checks (`physics_evidence.py:309`, `physics_evidence.rs:1005`).
For load-reference-1 the schema TASK owns the new
`schemas/load_reference_state.schema.json`, with `$defs/LoadReferenceContractEvidence`
and the authored-namespace `$defs`. Start with the closed code validators. Add a
transport-shape check against that `$def` (read-only) only after the manager
relays its frozen sha256. Until then, do not read an unfrozen schema. Also off limits: any
`core/product_physics/**`, the semantic tables, the raw fixtures, desktop, Git.
If a reader needs a change to the producer or to the table, stop and report it
to the manager.

## Acceptance

- New Rust tests (e.g. `tests/load_reference_contract.rs`) and Python tests
  accept all four frozen raw envelopes.
- The same tests reject targeted mutations with the same error in both
  languages. The mutations cover:
  - an unknown or missing key at each object level;
  - a forged E/nu/G mismatch;
  - a cross-case swap;
  - a missing not-joined diagnostic;
  - `source_recovery.status` changed;
  - a source receipt present;
  - a wrong profile;
  - a wrong table hash or ID;
  - `load_reference_states` added to a physics-1 envelope;
  - a duplicate `source_id`;
  - a zero factor;
  - a non-finite number.
- Canonical/stress-neutral/AnalysisRun outputs are produced for all four inputs,
  and existing outputs are byte-identical.
- All pre-existing result_export tests and the relevant pre-existing Python
  tests still pass unchanged. Run at least `tests/test_analysis_run_*.py`,
  `tests/test_physics_consumer_contract.py`, `tests/test_physics_source_contract.py`
  and `tests/test_stress_neutral_*.py`.

## Resources and method

- Cargo: `cargo +1.97.1 test --locked --offline -j 1` in `core/reporting/result_export`
  only, with your own `CARGO_TARGET_DIR` outside the repository. Delete the target when
  done; disk is tight, about 7 GB free on a shared machine.
- Python: `python3 -m pytest -q <specific files>` (no `-n auto`).
  Use a Python interpreter with `requirements-dev.txt` installed. The system
  `python3` lacks `jsonschema`, so the manager supplies a local environment path
  in the spawn message. Record the interpreter you used in `_run_records/`.
- No browser, native, npm or UI. No Git writes.
- Machine-specific paths only in `_run_records/`.

SendMessage the manager an early plan with the exact new API names. Then send the
return:

- files with their sha256;
- commands, with raw logs;
- pass/fail with the original failure text;
- the list of accepted and rejected mutation IDs, shared by Rust and Python;
- any producer or evidence ambiguity.

Write RETURN.md in `LSI/CP3_READERS/`.
