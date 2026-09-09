# Public Result Schema Acceptance Triage Addendum V1

Recorded: 2026-09-09T07:18:24Z
Role: fresh independent Agent 2 engineering reviewer
Basis commit: `533332349a4607eee561d4ef90fb05a62d86519e`
Scope: the 196 full-document schema failures in P5's headless witness; no physics re-review

## Verdict

`FULL_ENVELOPE_ACCEPTANCE_REQUIRES_VERSIONED_PUBLIC_CONTRACT_REPAIR`.

The current `results.schema.yaml` cannot represent every metadata value already emitted by the committed product producer and copied by the committed headless boundary. A small exact, lossless boundary mapping cannot make all three P5 documents pass. Patching only the checksum alias or coercing the remaining values would leave failures or discard public meaning.

The selected endpoint/curved-station physics repair remains independently acceptably bounded to its affected metadata rows, numerical leaves, raw-end-action preservation, real-producer parity, targeted tests, and registered review gates. Its initial whole-envelope zero-error expectation is unrealizable inside that fence. Record the whole-envelope failure as an open baseline public-contract blocker; this revised validation boundary is not a waiver of DEC-025 and is not a claim that the full document passes.

## Count and baseline proof

P5's `VALIDATION.json` (SHA-256 `de7221bd332d5889c1d67a175491ab6433eb52775842e2fa6b50d37445795ebf`) reports 196 failures: 160 in `straight_full_document.json` and 18 in each curved document. The field totals are:

| Failed field | Count | Classification |
| --- | ---: | --- |
| `canonicalization` | 6 | Exact alias is available: internal `rfc8785_jcs` and public `JCS` name the same RFC 8785/JCS canonicalization. |
| `component` | 135 | 114 are the six nodal displacement/rotation tokens; 21 are component/support review tokens. None has a complete truthful mapping into the current closed component enum. |
| `location` | 21 | The categorical part can often map to `end_i`, `end_j`, `node`, or `summary`, but pipe/node identity must remain in a typed reference; the current binding leaves `station_ref` empty and copies the compound string. |
| `coordinate_system` | 18 | `component_review` (14) and `support_local_preview` (4) have no current public token. Treating either as `global`, `element_local`, or `pipe_section` without a declared frame rule is not exact. |
| `basis` | 16 | Semicolon-encoded SIF/flexibility, expansion-joint, and support review details do not fit a current basis token and have no alternate typed value field in `QuantityResult`. |

The 114 nodal failures are 90 straight-document rows plus 12 rows in each curved document. Their six exact producer tokens are `nodal_displacement_x/y/z` and `nodal_rotation_x/y/z`.

These are baseline values at the basis commit, not effects of the endpoint repair:

- `core/product_physics/src/lib.rs` lines 7266-7319 emit all six nodal metadata tokens.
- The same file lines 7124-7137 emit the expansion-joint component, pipe-bearing location, and detailed basis; lines 7752-7782 emit the component-review token, compound endpoint location, and detailed SIF/flexibility basis; lines 8020-8135 emit the hanger/constant-load components, `support_local_preview`, compound node locations, and detailed review bases.
- `core/runner/headless/src/result_envelope_binding.rs` lines 216-223 copy all five producer metadata strings verbatim; lines 408-424 construct result rows with empty `station_ref` and `trace_chain`.
- `core/runner/headless/src/lib.rs` lines 918-929 explicitly describe RFC 8785/JCS and emit `rfc8785_jcs`.
- `schemas/results.schema.yaml` lines 559-637 close the five metadata fields to enums that omit those values. `core/reporting/result_export/src/lib.rs` lines 191-223 supply no typed review-detail field that could preserve the semicolon-encoded content elsewhere.

The committed source SHA-256 values used for this trace are `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` for product physics, `2ed810b577d5db34824e65a36e9d696d3758dd14d8c4bdd0941c1cb797cf9c85` for the headless binding, `57324f6611769ce754b87223e9007c36762e70c2dd8b2519bb6de51d6ff7e60b` for the headless runner, `dec88a2b29fe834afa9fb98b11a3bb94e7b7f06d9955d3c834cc948319831929` for the result-export model, and `a79c4c1425bd96d5fb96c4a92a64bbcfc1c040a12762c53029d6cc7d4145c1a0` for the public schema.

## Why boundary-only coercion is not valid

Omitting displacement/rotation metadata would erase structured axis and sign information. Mapping displacement/rotation to nodal force/moment tokens would be false. Mapping component-review quantities to stress components or support loads to a global axis would erase the review quantity or assume an undeclared frame. Replacing detailed bases with a broad existing token would discard user-entered SIF, flexibility, effective area, source, and consumption facts. A free-form diagnostic copy would preserve bytes but not their typed public meaning. `TBD`, family stripping, and schema weakening are outside the permitted disposition.

The checksum spelling can be normalized losslessly, and compound locations may be split losslessly after the contract identifies the reference field that retains the pipe/node identity. Those partial fixes do not unlock full acceptance and should travel with the coherent contract repair rather than create an intermediate false gate.

## Revised acceptance boundary for the selected repair

Accept the endpoint/curved-station repair only when all of these hold:

1. Every changed endpoint and curved-station row has canonical metadata and zero schema errors when validated as a schema-bound `QuantityResult`/affected-row set.
2. Real producer output proves endpoint action-to-section-cut signs and curved tangent frames; raw end-action rows remain byte-for-byte/numerically unchanged except for fixture regeneration caused by intended producer output.
3. Magnitudes, units, dimensions, result identities, row counts, and unaffected rows are preserved under the targeted repair or differences are explicitly enumerated as intended.
4. Targeted reversal, pure axial/torsion/bending, combined pressure/temperature, and pressure-eligibility tests pass, followed by the registered review gates.
5. The three full documents remain recorded as schema `FAIL` with the 196 baseline failures until the separately adopted contract tranche closes them.

## Smallest coherent future repair fence

A future versioned public-contract tranche must jointly cover:

1. An owner-adopted canonical vocabulary for the six nodal components and the finite review/support component and coordinate meanings.
2. A typed, lossless home for current review-basis facts, or an equally explicit result/reference model that preserves each fact without semicolon parsing or diagnostic escape hatches.
3. A declared compound-location split that retains pipe/node identity in an existing or newly adopted typed reference, plus the exact `rfc8785_jcs` to `JCS` export alias.
4. Synchronized changes to the public schema, result-export Rust model/serializer, product metadata model or producer as required by the adopted detail shape, and the headless boundary.
5. Consumer/docs/fixture updates and full-document schema validation proving zero errors with no row-family removal, `TBD` substitution, semantic coercion, or loss of metadata facts.

This is broader than the selected physics repair because it changes the public result contract and its representation across producer, DTO, boundary, schema, and consumers. No schema or source change is authorized or made by this addendum.
