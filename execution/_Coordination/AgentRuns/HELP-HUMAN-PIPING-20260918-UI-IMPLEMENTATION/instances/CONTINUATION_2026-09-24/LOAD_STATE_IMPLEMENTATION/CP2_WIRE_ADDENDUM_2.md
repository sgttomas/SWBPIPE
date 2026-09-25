# CP2 wire addendum 2

This addendum supplements two files, which are unchanged:

- `CP2_WIRE.md`, sha256 `81a7adbaa212517c518a61c5ab54cd7d9444adaee6b7f0dbec8b7b6efd168996`
- `CP2_WIRE_ADDENDUM_1.md`, sha256 `c389f5e3878c6a32d8c72b80d1374ea6f30c4350c2870ab7ac653972df3dc760`

It records the checkpoint-3 facts. These cover:

- the corrections that ROOT dispositioned from the independent checkpoint-2 review (`REVIEW_CHECKPOINT_2/RETURN.md`, sha256 `9b91c1ff…`);
- the retained-source join for resolved load/reference-state cases.

Every identity named in section 5 is a ROOT technical selection, recorded in `_run_records/session2/CP3_ROOT_SELECTIONS.json`. Readers, schemas and authoring surfaces use this addendum together with the two files above. Paths are WORKING_ROOT-relative.

## 1. Corrections to the typed boundary (review SF-A and N-6)

CP2_WIRE says "all new objects are closed", and ADDENDUM_1 §1 says "an unknown field in any object" is rejected. Neither was true at checkpoint 2 for the following cases. Checkpoint 3 makes both statements true.

**Field-free branches.** The following branches are now empty struct variants, so a sibling key is rejected. Serde ignores sibling keys on an internally tagged *unit* variant even under `deny_unknown_fields`.

- `geometry_ref {kind: "authored_model_geometry"}`
- `basis {kind: "direct_strain_reference"}`
- `fit {kind: "none"}`
- `participation {kind: "active_model_device"}` and `{kind: "inactive"}`
- `history {kind: "independent_equilibrium"}`

Each is still authored as just its discriminant, for example `{"kind": "none"}`.

**Quantities.** Every `{value, unit}` quantity inside `reference_configurations`, material `expansion_laws` and case `analysis_state` is read through a closed quantity wire. An extra key such as `basis` is a typed-boundary rejection.

The shared legacy `Quantity` used by every pre-0.4 document is deliberately left open. This is a recorded pre-existing exception: closing it could change existing document meanings.

**No authored geometry hash.** `geometry_ref` is exactly `{"kind": "authored_model_geometry"}` in `openpipestress.load_reference_state/1.0.0`. An authored `projection_sha256` is refused at the typed boundary, whether forged or correct. The published `reference_geometry.projection_sha256` is producer evidence, not an input. A future authored geometry hash would need a new selection.

**History continuation.** Branches that the reviewed interface names but this capability does not implement do not all "parse and then block", as CP2_WIRE states. `history: {"kind": "continuation"}` is an unknown discriminant in 1.0.0 and is a typed-boundary rejection (`Err`), as ADDENDUM_1 §1 already lists.

The branches that parse and then block are:

- `mass_state_ref`
- `participation` of `inactive` or `locked_equivalent_support`
- `base_motion`
- `device_reference`

## 2. Explicit null and older documents (review N-2)

An explicit `null` counts as authored presence in every document version. It is never treated as silent absence. This applies to these keys:

- model `reference_configurations`
- each model material's `expansion_laws`
- each case's `analysis_state`
- each request-level material's `expansion_laws`

How each document is handled:

- **Document 0.1–0.3 carrying any of these keys**, including with a `null` value, blocks with `LOAD_STATE_CONTRACT_VERSION_MISMATCH`. Request-level `materials[].expansion_laws` is now detected; previously it was silently ignored.
- **A 0.4.0 document with an explicit `null` for any of the first three keys** blocks with `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED` (new). Its affected ref is the key path: `reference_configurations`, `materials[<i>].expansion_laws`, or `<case id>.analysis_state`. Omit the key, or supply its value. Request-level materials remain wholly unsupported in 0.4.0 (`LOAD_STATE_REQUEST_MATERIALS_UNSUPPORTED`).
- **A malformed legacy key**, for example `reference_configurations` authored as an object, stays a typed whole-document rejection (`Err`, "invalid type").

The optional fields *inside* the closed objects are unchanged, for example `label`, `operating_temperature` and `boundary_motion`: `null` there is equivalent to absent.

## 3. Material records (review N-3)

Model and request material records are read through a typed wire. That wire flattens the unchanged `MaterialInput` fields and adds the material-owned `expansion_laws`. This restores two behaviours that checkpoint 2 lost:

- a duplicated key inside a material record is a `duplicate field` error, not last-wins;
- errors carry their text position.

## 4. Missing member section (review N-5)

A resolved member with a nonzero eigenstrain but no built section blocks the case with `LOAD_STATE_MEMBER_SECTION_MISSING`. Its affected refs are the case and the pipe. The eigen load is never dropped. This state is unreachable with a successful build and is pinned by a unit test.

## 5. Retained-source join (checkpoint 3)

### 5.1 Identities and when each is published

| Semantic contract | Profile | Published when | Receipt |
|---|---|---|---|
| `openpipestress.result_semantics/0.3.0/load-reference-1` (table sha256 `44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d`, unchanged) | `resolved_straight_load_state_v1` (exclusive) | no case of the invocation publishes a retained-source response | none; `source_block_recovery` absent |
| `openpipestress.result_semantics/0.3.0/load-reference-source-1` (table `fixtures/results/semantic_contract_v0_3_load_reference_source_1.json`, derived from physics-source-1) | `resolved_straight_load_state_source_v1` (exclusive) | at least one case publishes its selected retained-source response and the invocation receipt finalizes | policy `LOAD-REFERENCE-SOURCE-1`, required |

ROOT's reservation governs `load-reference-source-1` and its profile. They are carried only inside the join candidate on `codex/piping-load-states-20260925`, and are active only once that frozen candidate passes independent review.

The joined envelope's formulation basis keeps the load-reference-1 limitations, except the last one. That limitation states the joined method.

### 5.2 Case eligibility and attempt

For a 0.4.0 case, retained-source recovery is attempted under the same eligibility as for any exact-profile case:

- a captured value invocation;
- no nonlinear supports;
- no combinations;
- an ordinary attempt that is `sensitive` or rejected.

The source adapter admits the case only if every one of these holds:

- The recovery input carries that case's resolved case. A 0.4.0 model without one, or a pre-0.4 model with one, is refused.
- Each member's selected E and derived G are the built frame operands, bitwise. The frame area equals the exact annulus wall area.
- The actual eigen element loads equal, bitwise and in member order, `E_member*A_s*eps*` for each member with nonzero `eps*`. They are folded into the force after the nodal loads, in the order of the product's axial-equivalent-load step, and reproduce the actual force vector bitwise.
- Each resolved support motion is owned by exactly one rigid restrained DOF, and it is that DOF's prescribed value. Every other rigid DOF is an explicit zero.
- The case's pressure-region inventory is explicitly empty, and no other non-nodal producer is present.

Otherwise the attempt fails with the existing `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` (info) diagnostic, and the case publishes its ordinary response.

In the retained source:

- Each eigen load contributes identified force terms: `-a*x_local` at end i and `+a*x_local` at end j. The owner id is `load_state_eigenstrain:<len(pipe_id)>:<pipe_id>`. A primitive load with that ID is refused.
- Member-end axial functionals carry the eigen offset exactly: `+a` on end-i row 0 and `-a` on end-j row 0. Section functionals inherit it.
- Support actions subtract every identified force term at their DOF.

### 5.3 Per-case record and diagnostics

`contract_evidence.load_reference_states[i]` keeps the ADDENDUM_1 §3 shape. The following fields vary with the published method:

| Case published | `solve.recovery_method` | `source_recovery` | Diagnostic |
|---|---|---|---|
| ordinary | `ordinary_dense_structural_v1` or `ordinary_sparse_structural_v1` | `{status: "not_joined", code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}` | exactly one info `LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED` naming the case |
| retained source | `retained_source_blocks_exact_v1` | `{status: "selected", method: "retained_source_blocks_exact_v1"}` | `SOURCE_BLOCK_RECOVERY_SELECTED` (info), no NOT_JOINED |

In both semantics, `not_joined` means "the published response for this case is the ordinary structural route". The NOT_JOINED message now states the attempt fact:

```
requested_mode=<mode>; the published response for this case is the ordinary structural route; no retained-source response is joined into it; retained_source_attempt=<not_required_ordinary_checks_passed|unavailable|not_eligible>
```

A failed attempt additionally emits the info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, as physics-1 does. Readers must not validate message text.

`exact_cases[i]` is unchanged from ADDENDUM_1: `material_basis` is `resolved_per_member_load_reference_state_v1`, with the member `pipe_materials`. Only `recovery_method` follows the table above.

### 5.4 Receipt (`LOAD-REFERENCE-SOURCE-1`)

The receipt body has the physics-source-1 structure, with these differences:

- `policy` is `LOAD-REFERENCE-SOURCE-1`.
- Each case's `physical_evidence_sha256` hashes the domain `load_reference_source_case_evidence_v1` over `{exact_case, pressure, load_reference_state}`, where `load_reference_state` is that case's published record.
- `contract_evidence` has exactly `{pressure, connector, exact_cases, load_reference_states}`. `connector` is `[]`, and there is one `load_reference_states` record per case, in case order.
- A selected case's source commitment payload adds `load_reference_state`:
  - `resolved_evidence_sha256`: domain `load_reference_state_resolved_evidence_v1`, over the resolver record before `solve` and `source_recovery` are added.
  - `members[]`: `{pipe_id, material_id, selection_kind, E_bits, nu_bits, G_bits, thermal_strain_bits, fit_strain_bits, total_eigenstrain_bits}`.
  - `eigen_loads[]`: `{pipe_id, force_source, axial_load_bits, eigenstrain_bits, recovery_offset}`.
  - `prescribed_motions[]`: `{dof, value_bits}`.
- Eigen force terms have owner `{kind: "member_eigenstrain", id: <pipe_id>}` and are not listed in `supported_loads`.
- Pre-0.4 payload bytes are unchanged.

At finalization the producer re-derives every case from the captured request through the resolver pipeline:

- validation, normalization and `resolve_case`;
- a member-pair build;
- the effective-case loads plus eigen loads;
- the resolved prescribed values.

It then:

- replays the selected response against that re-derivation;
- checks current-source binding;
- binds `exact_cases`, `pressure` and the case's `load_reference_states` record to the re-derivation.

A resolved operand that differs between the capture and the published run is refused. This covers a member pair, an eigenstrain, a prescribed motion and the eigen offset sign. Every other case of a joined envelope must be an ordinary `checks_passed` case, and is bound in the same way.

## 6. Producer-generated artifacts (checkpoint 3)

**load-reference-1 raw envelopes.** These were regenerated by the producer and never hand-edited. They differ from the CP3-frozen inputs only in the NOT_JOINED message. The frozen hashes are reproduced byte-exactly by substituting the checkpoint-2 message back (`_run_records/session2/cp3_regen_diff_check.{py,log}`).

| File | Before | After |
|---|---|---|
| `fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json` | `6c1dd176…` | `915965a446ff04c6c4b7f0209ffff34c4ac040a0d4b8243d875dc7217a9cb74c` |
| `fixtures/product_preview/load_reference/connected-dense_scrutiny.raw.json` | `333515b3…` | `3824035cabf2d2a151756e04b46112ccbdcf4ce8237f49e68b5d3b36b8dfb2bf` |
| `fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json` | `20802f97…` | `71be3e4fb1af54e962d5e4939a518fcab6f7896842ab4f08212a8212ae47f134` |
| `fixtures/product_preview/load_reference/pressure-dense_scrutiny.raw.json` | `8215cad2…` | `b46eeb8a5dc82bb9e0398b8d58db07eb73906eb25110d68b0032fd12df1d0707` |

**load-reference-source-1 witnesses** (`fixtures/product_preview/load_reference_source/`, all inputs invented).

- `{n05,n06,fields,mixed}.request.json` are 0.4.0 unchanged-state companions of the physics-source-1 witnesses. `eigen_motion.request.json` is a sensitive member with thermal and fit eigenstrain and prescribed motion.
- The requests are authored by `_run_records/session2/cp3_author_join_requests.py`.
- Each request has `*-sparse_interactive.raw.json` and `*-dense_scrutiny.raw.json`. These are producer output from `core/product_physics/examples/physics_source_connected.rs`, and every one is qualified.
- In the companions, every result row is bit-identical to the 0.3.0 physics-source-1 raw envelopes. The only differences are intended:
  - 0.4.0 has no case-wide modulus-basis row and no `EXACT_PRESSURE_REDUNDANT_G_IGNORED` warning; the member evidence records the ignored G instead;
  - pressure `temperature_basis` is `resolved_member_state`;
  - the retained identity digests differ, because the identity binds the resolved case.
