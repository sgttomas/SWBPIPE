# T1_WP1_JOINED_SCHEMAS: RETURN

- **Role:** Type 2 TASK for the T1 WORKING_ITEMS manager (`a56f5deb01ca844b0`), spawned by ROOT.
- **Paths:** WORKING_ROOT-relative. `LSI` is this folder's parent.
- **Records:** scripts are in `_run_records/`, and logs are listed in section 5.
- **Git:** none. No staging and no commit.

**Result: complete.**

- Additive `load-reference-source-1` branches are in the three v0.3 carriers.
- `analysis_run.schema.json` needs no edit, because it dispatches only on `schema_version`.
- The preservation check passes. It shows appends only, plus the one ruled wrap.
- The new test file passes. Every existing load-reference-1, physics-source-1, physics-1, source-blocks-1 and precision-1 carrier keeps its branch.
- Of 26 schema-weakening mutations, 25 are caught by instance-level tests. M15 alone is an equivalent mutant, and the combined mutant M26 (M15 plus the pairing it depends on) is caught.
- Two protected tests pinned the branch count, and a ruling allowed me to edit each one minimally (section 3).

## 1. Basis and inputs

- **Briefs:**
  - `TASK_BRIEFS/_T1_COMMON.md`: `0bc6a1bd…2046`.
  - `TASK_BRIEFS/T1_WP1_JOINED_SCHEMAS.md`: `907581ad…be7`.
- **Base:** `d8f0dc4f7`. I sent four questions to the manager, and all four were ruled on 2026-09-26:
  1. the pin in `test_load_reference_schema.py` may get one minimal edit;
  2. the new `LoadReferenceSource*` `$defs` live in the carrier files, and reuse the LR and physics-source sub-defs by `$ref`;
  3. `AnalysisRun.properties.contract_evidence` gets exactly one `anyOf` wrap, and the joined AnalysisRun requires the receipt and the evidence, like physics-source-1;
  4. the pin in `test_source_block_schema_contract.py` may get one minimal edit.
- **Read-only inputs, unchanged:**
  - `schemas/load_reference_state.schema.json`: `640fd447…af65`.
  - `schemas/physics_source_recovery.schema.json`: `3bb96955…c5c`.
  - The joined table `fixtures/results/semantic_contract_v0_3_load_reference_source_1.json`: `d1628194…8337`. It is pinned by hash, and the test also checks its id, profile and policy.
- **Interpreter:** the given venv, Python 3.11.15, with jsonschema 4.26.0, referencing 0.37.0 and pytest 9.1.1.
- **Run settings:**
  - `PYTHONDONTWRITEBYTECODE=1`, `-p no:cacheprovider`.
  - `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` pointed at the existing built helpers. No Cargo was invoked.

## 2. Findings that shaped the design

The brief assumed that joined evidence would validate as `load_reference_state.schema.json#/$defs/LoadReferenceContractEvidence`. That is false for all ten raws:

- **`exact_cases`** have the physics-source-1 exact-case shape: `recovery_method`, plus the retained `pipe_stress_extrema` alternatives. They do not have the physics-1 shape that `LoadReferenceExactCase` copies.
- **`LoadReferenceStateRecord.solve.recovery_method`** admits only the two ordinary methods.
- **`source_recovery`** is const `not_joined`, but a selected case carries `{status: selected, method: retained_source_blocks_exact_v1}` (ADDENDUM_2 §5.3).
- **The receipt** has the physics-source-1 receipt shape exactly, except `body.policy`. With the policy swapped back, it validates against `physics_source_recovery.schema.json`.

## 3. Files changed

| File | sha256 before | sha256 after |
|---|---|---|
| `schemas/results.v0.3.schema.yaml` | `c303219b89c63f41bdda71b49af1ed15175c106b035ff4f5b50e2eaf2cdc57ed` | `633c21bd82d30dc5b88f29e0e814862fd2c0709a5ccd0c9567d59ca1b5975fea` |
| `schemas/analysis_run.v0.3.schema.json` | `2045d3651709ec32c53b17a16f504c4818c9f14d7898bdfacf7dbc4068477410` | `c3dba5510f82d20b32cbae8999da8e10469ad1d9f0e6fafb6adaf805cebf2781` |
| `schemas/stress_neutral_export.v0.3.schema.json` | `80acc90bcf7170c7f07da2f1569a258fc56c4de455c8e936bf8da476e6478e60` | `7c8c4edd00a07ec249c4cf9d17bfa0b52379cf403eb6bb8d29cbd72f2ced3556` |
| `schemas/analysis_run.schema.json` | `0c399a2a62ad247bc38f9d3e40c176bac0e56b5ab6604c0189f552834a8b8a83` | unchanged (not needed) |
| `tests/test_load_reference_source_schema.py` (new) | none | `6bf32c0284e4589e738f3fecb095c6417708d88433a256704a989242d08916a7` |
| `tests/test_load_reference_schema.py` (ruling 1) | `9c02dbba8a909c51b124e07de88bc50114e25eb987cbfe705bc23879f4e6c51d` | `67742df144fded76f87e27c5da1f9dee961bf70a3561804ba7769e22507e89c5` |
| `tests/test_source_block_schema_contract.py` (ruling 4) | `61dcfd33b6977bdeaf4cdcabeae9d01271c5c85525677b0213f02ec97f9f6a5e` | `0f73a514df746445d8c8f3994e405b581372efaf654fa63f307417a170ece41e` |

**Generation.** `_run_records/edit_carriers.py` generated the schema bytes:

- It is deterministic and refuses to run twice.
- It asserts the byte round trip before editing.
- It writes atomically, through a temporary file in `schemas/` and `os.replace`.

Pre-edit hashes are in `_run_records/pre_edit_sha256.txt`.

### 3.1 `results.v0.3.schema.yaml`

**Appended to existing lists:**

- the joined ID to the `producer.semantic_contract_id` and `semantic_contract_ref.ref_id` enums;
- the joined profile to the `formulation_basis.profile_id` enum;
- `#/$defs/LoadReferenceSourceContractEvidence` to the `contract_evidence` `anyOf`;
- `#/$defs/LoadReferenceSourceRecovery` to the `source_block_recovery` `oneOf`.

**`ResultEnvelope.oneOf[5]`** is a deep copy of the physics-source-1 branch (`oneOf[3]`) with these changes:

- the ID and profile consts are swapped;
- `source_block_recovery` is set to `#/$defs/LoadReferenceSourceRecovery`;
- `contract_evidence` is set to `#/$defs/LoadReferenceSourceContractEvidence`;
- `required` is `[source_block_recovery, contract_evidence]`;
- `result_sets` items stay `PhysicsSourceResultSet`. The joined raws and the READERS documents carry `retained_source_endpoint_normal_max_v1` maximum metadata, which `ResultSet` refuses.

**New `$defs`.** Each one is a copy of an existing definition. Its local refs are rebased to the source file, so every nested shape is reused by `$ref`.

| Definition | Copy of | Differs only at |
|---|---|---|
| `LoadReferenceSourceRecovery` | the `physics_source_recovery.schema.json` root | `body` → `LoadReferenceSourceReceiptBody` |
| `LoadReferenceSourceReceiptBody` | `physics_source_recovery…#/$defs/receipt_body` | `policy` const `LOAD-REFERENCE-SOURCE-1` |
| `LoadReferenceSourceContractEvidence` | `load_reference_state…#/$defs/LoadReferenceContractEvidence`: exactly `{pressure, connector, exact_cases, load_reference_states}`, with `connector` const `[]` | `exact_cases` and `load_reference_states` items |
| `LoadReferenceSourceExactCase` | `physics_source_recovery…#/$defs/physical_evidence/properties/exact_cases/items` | `material_basis` const `resolved_per_member_load_reference_state_v1`; `pipe_materials` items → `load_reference_state…#/$defs/LoadReferencePipeMaterial` |
| `LoadReferenceSourceStateRecord` | `load_reference_state…#/$defs/LoadReferenceStateRecord`; members, support components, contributions and excluded sources stay `$ref`s into that file | see below |

`LoadReferenceSourceStateRecord` differs from its source in three ways:

- `retained_source_blocks_exact_v1` is appended to the `solve.recovery_method` enum and to `solve.oneOf`;
- `source_recovery` becomes a `oneOf` of two members: the unchanged `not_joined` object, and the closed object `{status: selected, method: retained_source_blocks_exact_v1}`;
- a record-level `oneOf` pairs selected with retained, and not_joined with the ordinary methods.

The pairing covers a single record only. The cross-record rules stay reader validation: case order, at least one selected case, and receipt binding.

Nothing is changed in `load_reference_state.schema.json` or in `physics_source_recovery.schema.json`. A test pins each copy as equal to its source, except at the documented pointers listed above.

### 3.2 `analysis_run.v0.3.schema.json`

**Appended:**

- to `SemanticContract`: the joined ID and table hash in its enums, and the pair `{id, sha256}` in its `oneOf`;
- to `AnalysisRun.properties.source_block_recovery.oneOf`: `results.v0.3.schema.yaml#/$defs/LoadReferenceSourceRecovery`.

**`AnalysisRun.oneOf[5]`** is a copy of the physics-source-1 branch with these changes:

- the ID and table hash are pinned on `reproducibility` and on every `result_refs[]`;
- it requires `source_block_recovery` and `contract_evidence`;
- both point to the results `$defs`.

**The only non-append change (ruling 3).** `AnalysisRun/properties/contract_evidence` changed:

- from `{"$ref": "physics_source_recovery.schema.json#/$defs/physical_evidence"}`
- to `{"anyOf": [<that same $ref>, {"$ref": "results.v0.3.schema.yaml#/$defs/LoadReferenceSourceContractEvidence"}]}`.

Old outcomes are unchanged:

- Every pre-existing branch pins its own evidence. Physics-source-1 still requires `physical_evidence`, and the other branches forbid `contract_evidence`.
- So joined evidence fits only the joined branch.
- The tests show this for:
  - physics-source-1 runs (still valid, on branch 3);
  - load-reference-1 runs carrying joined evidence (refused);
  - a physics-source-1 run carrying joined evidence (refused);
  - physics-1 evidence in a physics-source-1 run (refused).

### 3.3 `stress_neutral_export.v0.3.schema.json`

**Appended:** the joined ID to the producer, `semantic_contract_ref` and `semantic_contract.id` enums, and the joined profile to the `formulation_basis` enum.

**`oneOf[5]`** is a copy of the physics-source-1 branch with these changes:

- `semantic_contract` is `{joined ID, d1628194…}`;
- the profile const is swapped;
- `contract_evidence` and `source_block_recovery` point to the results `$defs`;
- the physics-source-1 UTF-8 CSV and manifest canonicalization are kept;
- `required` is `[contract_evidence, source_block_recovery, source_annotations]`.

At the top level, the properties `contract_evidence` and `source_block_recovery` are already `{"type": "object"}` in this file, so nothing was wrapped.

### 3.4 Minimal edits to two protected tests (rulings 1 and 4)

**`tests/test_load_reference_schema.py`, `test_carrier_branches_are_appended_after_the_existing_methods`.**

The pins on the first five branches and on the LR entries stay exact, at their positions. The joined entry is asserted to come after them.

Before (lines 603–615):

```python
    assert ids == ["openpipestress.result_semantics/0.3.0/precision-1", PHYSICS_ID,
                   "openpipestress.result_semantics/0.3.0/source-blocks-1",
                   "openpipestress.result_semantics/0.3.0/physics-source-1", LR_ID]
    assert results["properties"]["contract_evidence"]["anyOf"][-1] == LR_EVIDENCE_REF
    assert results["oneOf"][4]["properties"]["contract_evidence"] == LR_EVIDENCE_REF
    assert results["oneOf"][4]["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": LR_PROFILE}
    run = schema("analysis_run.v0.3.schema.json")["$defs"]
    assert run["SemanticContract"]["oneOf"][-1] == {"properties": {"id": {"const": LR_ID}, "sha256": {"const": LR_SHA}}}
    assert len(run["AnalysisRun"]["oneOf"]) == 5
    assert run["AnalysisRun"]["oneOf"][4]["not"] == {"anyOf": [{"required": ["source_block_recovery"]},
                                                              {"required": ["contract_evidence"]}]}
    package = schema("stress_neutral_export.v0.3.schema.json")
    assert len(package["oneOf"]) == 5
```

After:

```python
    assert ids == ["openpipestress.result_semantics/0.3.0/precision-1", PHYSICS_ID,
                   "openpipestress.result_semantics/0.3.0/source-blocks-1",
                   "openpipestress.result_semantics/0.3.0/physics-source-1", LR_ID,
                   "openpipestress.result_semantics/0.3.0/load-reference-source-1"]
    assert results["properties"]["contract_evidence"]["anyOf"][2] == LR_EVIDENCE_REF
    assert len(results["properties"]["contract_evidence"]["anyOf"]) == 4
    assert results["oneOf"][4]["properties"]["contract_evidence"] == LR_EVIDENCE_REF
    assert results["oneOf"][4]["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": LR_PROFILE}
    run = schema("analysis_run.v0.3.schema.json")["$defs"]
    assert run["SemanticContract"]["oneOf"][4] == {"properties": {"id": {"const": LR_ID}, "sha256": {"const": LR_SHA}}}
    assert len(run["SemanticContract"]["oneOf"]) == 6
    assert len(run["AnalysisRun"]["oneOf"]) == 6
    assert run["AnalysisRun"]["oneOf"][4]["not"] == {"anyOf": [{"required": ["source_block_recovery"]},
                                                              {"required": ["contract_evidence"]}]}
    package = schema("stress_neutral_export.v0.3.schema.json")
    assert len(package["oneOf"]) == 6
```

The rest of that test (the package `oneOf[4]` pins) and every other line of the file are unchanged. The new test file separately pins the joined entries as the last ones.

**`tests/test_source_block_schema_contract.py`, `test_actual_composite_maximum_metadata_has_a_method_scoped_canonical_route`, line 377.**

Before:

```python
        target = "PhysicsSourceResultSet" if index == 3 else "ResultSet"
```

After:

```python
        target = "PhysicsSourceResultSet" if index in (3, 5) else "ResultSet"
```

Nothing else in that file changed.

## 4. The new tests (`tests/test_load_reference_source_schema.py`: 161 cases in the worktree with the 30 joined carriers, 132 in an isolated tree without them)

**Validation machinery.** Validation uses jsonschema Draft 2020-12 with the same local resource registry as `tests/schema_validation.py`, built once for speed. Every schema document is also metaschema-checked.

**Branch mapping.** `matching_branches` validates the instance against each carrier `oneOf` branch alone. Every positive asserts both of these:

- the instance validates against the full carrier schema, and additionally against `analysis_run.schema.json` for AnalysisRun records;
- it matches exactly one branch, the expected one.

### Positive tests

**Identity:**

- all four carrier documents parse strictly and pass the draft 2020-12 metaschema;
- the joined table is pinned by hash, id, profile and policy;
- the joined branches come last, with every pin as described in section 3;
- the copy-equality test from section 3.1.

**The ten joined raws** (`fixtures/product_preview/load_reference_source/*.raw.json`):

- verbatim `contract_evidence` validates as `LoadReferenceSourceContractEvidence`;
- the verbatim receipt validates as `LoadReferenceSourceRecovery`;
- each raw validates on branch 5 in four carriers:
  - (a) a minimal results scaffold around verbatim raw transport metadata;
  - (b) the committed load-reference-1 document, with the joined metadata, evidence and receipt swapped in;
  - (c) the committed load-reference-1 AnalysisRun, rebound in the same way;
  - (d) the committed load-reference-1 stress-neutral packet, rebound in the same way.
- The raws together cover `selected`/retained and `not_joined` with both ordinary methods (the `mixed` witness).
- A joined AnalysisRun and a joined packet built over actual physics-source-1 route carriers also validate.

**Old documents keep their branch:**

| Carriers | Branch |
|---|---|
| The four committed load-reference-1 documents, AnalysisRuns and stress-neutral packets | 4 |
| The four physics-1 raws of the existing route | 1 |
| The 14 physics-source-1 raws | 3 |
| The six source-blocks-1 raws | 2 |
| The two precision-1 raws (results scaffold) | 0 |

- For physics-1, physics-source-1 and source-blocks-1, the carriers are the results scaffold plus the AnalysisRun and packet built by the existing Python route (`captured`).
- None of them matches branch 5.

**Joined carriers.** All 30 committed carriers validate on branch 5 (section 5): the READERS TASK's 20 canonical carriers (`fixtures/results/load_reference_source_*.{document,analysis_run}.json`), and the 10 `*.stress_neutral.json` packets from the manager's packager. If none exists, the test is skipped with a reason.

### Negative tests

**Evidence (16):**

- an extra key; a missing `load_reference_states`; a missing `pressure`; a non-empty `connector`;
- selected with an ordinary method, and not_joined with the retained method;
- selected without `method`, and selected with `code`; an unknown status; selected with a non-retained method;
- a record profile set to the joined profile;
- an exact-case `material_basis` other than the pinned one; a missing `recovery_method`;
- a pipe material without `material_selection_kind`, and physics pipe materials;
- an extra record key.

**Namespaces:**

- joined evidence and load-reference-1 evidence do not cross-validate;
- joined evidence and physics-source-1 evidence do not cross-validate.

**Receipt:**

- `policy` set to `PHYSICS-SOURCE-1`, `SOURCE-BLOCKS-1` or `LOAD-REFERENCE-1`, or missing: each refused;
- the joined receipt is refused by the physics-source and source-blocks receipt schemas;
- the physics-source receipt is refused as joined;
- a body extra key is refused.

**`SemanticContract` pairs (4):** the joined ID with the LR or physics-source hash is refused, and so is the LR or physics-source ID with the joined hash.

**Relabels and mix-ups per carrier:**

- **Results (14):**
  - each pin; the receipt missing or physics-source; evidence missing, physics-source or LR;
  - joined relabelled as load-reference-1 and as physics-source-1;
  - load-reference-1 and physics-source-1 each relabelled as joined;
  - the joined profile under the LR and physics-source IDs.
- **AnalysisRun (16):**
  - the LR or physics-source hash; the LR hash on `reproducibility` only (`reproducibility_hash_only`); the joined hash under the physics-source ID; an unknown ID; mixed row contracts;
  - the receipt or evidence missing, or foreign;
  - load-reference-1 and physics-source-1 runs relabelled as joined, and the reverse of each;
  - a physics-source-1 run carrying joined evidence.
- **Stress-neutral (15):**
  - each pin; the receipt or evidence missing, or foreign; `source_annotations` missing;
  - the precision CSV policy;
  - load-reference-1 relabelled as joined, and joined relabelled as load-reference-1 and as physics-source-1.

**Scope.** These are schema-level instances only. They make no hash, receipt-binding or reader claim, and every input is a committed producer or route artifact. The test never rewrites a fixture.

## 5. Checks

| Check | Where | Result |
|---|---|---|
| Baseline: 10 existing suites (`test_load_reference_schema`, `test_results_schema`, `test_analysis_run_schema`, three stress-neutral suites, `test_source_block_schema_contract`, `test_physics_consumer_contract`, `test_physics_source_contract`, `test_load_reference_readers`) | isolated `git archive` of `d8f0dc4f7`, pristine | **1214 passed, 15 skipped** |
| Same 10 suites + new file, first candidate run (before ruling 4) | isolated HEAD + candidate | 1339 passed, 16 skipped, **1 failed**: the `test_source_block_schema_contract` pin, repaired by ruling 4 |
| `test_source_block_schema_contract.py` after ruling 4 | isolated | **99 passed, 11 skipped** (skips: the existing headless-artifact gates) |
| Final: same 10 suites + new file | isolated HEAD + all final candidate files (byte-identical to the worktree copies) | **1345 passed, 16 skipped, 0 failed** (`_run_records/isolated_final.log`). The extra skip is the joined-carrier test, which has no carriers in HEAD |
| `test_load_reference_schema.py` | within the runs above | **527 passed**, as at baseline |
| New file + `test_load_reference_schema.py`, with the 20 READERS carriers present | worktree | **678 passed** (151 + 527), 0 failed (`_run_records/worktree_new_and_lr.log`) |
| Joined-carrier test after the 10 stress-neutral packets appeared | worktree | **30 passed** |
| Preservation | `_run_records/preservation_check.py` against HEAD | **PASS** (below) |
| Mutations | `_run_records/mutation_check.py` on the isolated copy | **PASS** (below) |

**Preservation.** Base pointers compared, with 0 failures:

| File | Pointers |
|---|---|
| results | 2223 |
| analysis_run.v0.3 | 648 |
| stress_neutral | 2194 |
| analysis_run.schema.json | 42 |

Every changed list was extended only by appending:

- the results enums and the `anyOf`/`oneOf` lists (6);
- the AnalysisRun `SemanticContract` enums, its `oneOf`, the receipt `oneOf` and the branch `oneOf` (5);
- the stress-neutral enums and its `oneOf` (5).

Added keys: the five `LoadReferenceSource*` `$defs`, in results only. The one wrap is `/$defs/AnalysisRun/properties/contract_evidence`, and its base value is kept as `anyOf[0]`.

**Mutations.** 26 mutants. Each one weakens a single pointer: a pin, the receipt, an exact key or a pairing. The logs are `_run_records/mutation_check_part1.log` (M01–M19) and `mutation_check_part2.log` (M15–M26).

- Each mutant runs the new file with `-x`, with the two structural pin tests deselected. A kill therefore comes from an instance-level test: a refused relabel, or an accepted instance that fails.
- The bytes are restored after each mutant, checked by sha256.
- M01–M14 and M16–M26 are killed. Each log line names the killing test.
- **M15** removes the AnalysisRun branch's pin of the table hash on `reproducibility`. It survives, and is an equivalent mutant: `SemanticContract.oneOf[5]` already pairs the joined ID with its hash. This redundancy is inherited from the physics-source-1 branch.
  - **M26** removes both pins. It is killed by the added run case `reproducibility_hash_only`.
  - **M20** removes the pairing alone. It is killed by the `SemanticContract` pair tests.
- **Process.** Part 1 stopped at M20 on a bug in the script's list-index check. After that fix, part 2 reran from M15 against the final test file, which adds the `reproducibility_hash_only` case and the `SemanticContract` pair tests. M01–M14 ran against the previous test file (`72e95e43…`); the final file only adds cases, so those kills still hold.

**Isolated runs.** The isolated runs use a `git archive` of HEAD, without `apps/` or `execution/`, plus only my candidate files. They therefore exclude the other lanes' in-flight edits (`core/analysis_runs`, `result_export` and others). The worktree run includes them.

## 6. Not done, and notes

- **Joined carriers.** All 30 were validated as they stood at the end of my run (hashes below): 20 READERS carriers, which the manager reports final, plus 10 stress-neutral packets from the manager's packager. If any is regenerated, rerun the new test file.
- **Scratch and build output.** Scratch space was deleted, and no Cargo target was created.
- **One denied write.** The harness's auto-mode classifier refused one command, which would have saved the test's before/after text and a `git diff` into `_run_records/`. I did not work around it. The before/after text is in section 3.4, as the manager asked.
  - A stale `lr_schema_test_after.txt` was left behind. It held pre-edit text, from a command that ran before the edit landed, and I removed it.
  - `lr_schema_test_before.txt` is correct.
- **Interruption.** An account usage limit paused the run, and ROOT resumed it. No files were left mid-edit, and the scratch schema bytes were verified restored.

Joined carriers validated, with their sha256 at validation time:

```
6abcdbafd35cb7473b08716fa487653316bc55e970b2a51a42e42d06c0964a8a  load_reference_source_eigen_motion_dense.analysis_run.json
96dd0bff00bf5e12e11c4233a964939a222d653a6307603e666d3fd5f3199a52  load_reference_source_eigen_motion_dense.document.json
993db7d2e3006c8edef52320701901a0664e3bd38068a9c1b50c89b975be4f8b  load_reference_source_eigen_motion_dense.stress_neutral.json
11afdf795c009bd1c5a6be651631262ee6876968a335bcb83cd01c4d8c44992f  load_reference_source_eigen_motion_sparse.analysis_run.json
2309a1a89fdc4003b515a2016ff9de263195e21c1573231bf6e0f4d2e5728614  load_reference_source_eigen_motion_sparse.document.json
9962f11f7b3c0c7cc247f71354878da64ef98fd78a45cd01e2058633475cbffe  load_reference_source_eigen_motion_sparse.stress_neutral.json
954a926fa16f9775919d6c188e6263fb774b42ba266f13d63a446091b7901fe7  load_reference_source_fields_dense.analysis_run.json
3675811c9dc9559b8f2466dbfc7c12b84e2dd501a1b526ead3526ccdb51abe93  load_reference_source_fields_dense.document.json
115e10efb046b2d557508e9b2526b860fa58da84858d6b72c328220cfdbd6379  load_reference_source_fields_dense.stress_neutral.json
3674b53403162a93f6e600a31a41368bf4729002daef891a81325370f6e3aed9  load_reference_source_fields_sparse.analysis_run.json
292f02a22b17f21e30afb0835808cffeb6ee8fcc461a9dbc44d694cfa32fda1d  load_reference_source_fields_sparse.document.json
2824bc8ae7c4b9c565c6a53dcf0489e4d87f000d964d1cf36152d680b5afccd3  load_reference_source_fields_sparse.stress_neutral.json
d471d2013291fcae2a4250801ae3f95a3d4866d46e51f074c3224d2c4299a72b  load_reference_source_mixed_dense.analysis_run.json
3037d95c944287210bae45b7f0ef63ff92551d7beb8b9c780ee49cc1977aa885  load_reference_source_mixed_dense.document.json
e9e53fdd385de5947366a1e083cd58faed60af726c3d08f98d048c275ac23376  load_reference_source_mixed_dense.stress_neutral.json
3b23eb988f96ef409e99c5f706ce74a32f0c3b91502cb431d33ddf6e877f7d11  load_reference_source_mixed_sparse.analysis_run.json
cfe0e2fb96faca887198a93205e0303f94451ed9371fddea246c487533083edc  load_reference_source_mixed_sparse.document.json
a6fc6f6de1dabb0746943ef8e3e53e4cbba8271f8e7fe3d4aa136740c97b0696  load_reference_source_mixed_sparse.stress_neutral.json
42cca52a066323282f574fb9c96e38ad47d4c11556d9db2fea67a10f24bc8b36  load_reference_source_n05_dense.analysis_run.json
1cd988936860ceb4449b4574f004220a1a2e505f3fa4a2fbe3a55d6ce670c67a  load_reference_source_n05_dense.document.json
714a7285c5f987214db7035e274adde9acecae2fd9a879304e03ee000c31a1cb  load_reference_source_n05_dense.stress_neutral.json
3d04ec9e8191f173b1631a6675553d2f5e161ffdf5a4922e6f11627f1d1e4c29  load_reference_source_n05_sparse.analysis_run.json
fca2d8f8512fb58eadc418b18ecf402a46616f413298f2d3b739ecd5fa713ec6  load_reference_source_n05_sparse.document.json
5133c8da46f8425a143c5fae81982da8bb2260555fcd1f8e83cdb77bedcaf2e9  load_reference_source_n05_sparse.stress_neutral.json
a8c68a60d0a28230a54db7aa8ebe30420080ce8f092b1b2b49d916faa41bb8b4  load_reference_source_n06_dense.analysis_run.json
1fa53e5a8892e1f1c563f76146128c2e3cc62d13dd6c9e8425408579badd6222  load_reference_source_n06_dense.document.json
bbd0657b22578728fc4b144d29a05c9c77aaf6a4a1e67265b4d8abb49d292716  load_reference_source_n06_dense.stress_neutral.json
1f599c68432e9d9d8e255f3175de74b20d795d5db043df1b69779f7a990d3257  load_reference_source_n06_sparse.analysis_run.json
baba6a89e822441def77f857b9dbb746edf40ee6be05a6f186756144780b5066  load_reference_source_n06_sparse.document.json
0d6202b3071cfb82bc236007c03b1eca1ffecd9819c6d1e5666fb599f27d9670  load_reference_source_n06_sparse.stress_neutral.json
```

## 7. Design questions for the manager

1. **Where the joined `$defs` live.** They are in results.v0.3, and the AnalysisRun and stress-neutral carriers reach them by `$ref` to `results.v0.3.schema.yaml#/$defs/…`. This is the first cross-carrier `$ref`; the earlier shared shapes live in library files (`load_reference_state`, `physics_source_recovery`).
   - A cleaner home would be `load_reference_state.schema.json` or a new library file. That needs a write grant, and `load_reference_state.schema.json`'s closed-shape vocabulary test permits only local one-level refs, so it would need copies rather than refs.
   - The Rust, Python and TS code checkers read only `PhysicsContractEvidence` / `physical_evidence` from these files, so they are unaffected.
2. **Wire.** ADDENDUM_2 §5.3 says `load_reference_states[i]` "keeps the ADDENDUM_1 §3 shape". The shape does not stay the same: `solve.recovery_method` and `source_recovery` widen, and `exact_cases` take the physics-source-1 shape, not the physics-1 shape. It would help to record this in the next addendum.
3. **Pairing encoded in the schema.** Selected↔retained within one record is shape-level and is encoded. The rule that `exact_cases[i].recovery_method` agrees with the record is cross-array, so it is left to the readers.
