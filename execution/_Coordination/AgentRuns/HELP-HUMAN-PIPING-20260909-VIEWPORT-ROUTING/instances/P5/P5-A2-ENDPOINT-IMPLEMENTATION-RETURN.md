# P5-A2 endpoint implementation return

RUN_STATUS: FAILED

ControlSurface: FILE
TaskProfile: NONE
TaskSkill: software-bounded-implementation
ScopePath: `{PROJECT_ROOT}/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-03_Fundamental stress recovery module`
WriteAuthorization: ALLOWED_WRITE_TARGETS
ToolPolicyCompliance: PASS

## Outcome

The frozen implementation repairs endpoint stress signs and curved endpoint frames by recovering mechanical stress from section-cut resultants at fractions 0 and 1. It preserves raw endpoint action output, stable result IDs, result kinds, case keys, and station numeric leaves. The V3 additive pressure-eligibility repair uses one genuine-pressure predicate for both thrust and pressure stress.

The product diff is frozen and ready for the required fresh independent 100% source-and-fixture review. The task return is FAILED because the released complete headless/schema route exposed 196 pre-existing broader producer/schema errors outside P5's source fence. The diff is not accepted, merged, lifecycle-closed, or ready for DEC-025.

## Accepted inputs and hash binding

- Repository source: `533332349a4607eee561d4ef90fb05a62d86519e`.
- Sealed brief V2 historical launch SHA-256: `9d654abc649631e5827c7f7c858160a6e82a3150489797f65bf1a31b76166900`; normalized active SHA-256: `359e51a63635395377c48dd91354ad67786ba30ae4f9850536cf9e8c33cbe98c`.
- Additive amendment V3 historical launch SHA-256: `678a67726a1a9ba8ec0fb064caefd94e8ba6b89f55ebf7a0107f0a64844df896`; normalized active SHA-256: `1030101e3764f6a27a967574d8e27cebb1170ba7b2c3b33e6ed7d25339f075f3`.
- Whitespace-normalization capture SHA-256: `a2ea118cee22f52c885be38212b58c217d8d83c488629ac111a865389e7e575d`.
- Frozen P5 graph SHA-256: `187faef757eb482dfcd53830bd57c019d1d59a1877cc96a4d62588def7240a71`.
- Pressure eligibility addendum SHA-256: `dc443b7dea76802072a7564fb8047b43221b37c143bd40ce4ccf32306956d3f9`.

## Changed product paths

| Path | Before SHA-256 | After SHA-256 | Diff |
|---|---|---|---|
| `core/product_physics/src/lib.rs` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` | `dbfff6d5c2fa9241e972042fd2c3a6ce35c6d60f4a973dd5a083a7fe5dfb9c58` | 706 insertions, 109 deletions |
| `fixtures/product_preview/invented_mechanics_result.json` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` | `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c` | 167 insertions, 167 deletions |

The source diff includes colocated oracles. Repository-wide `rustfmt` style was not propagated to unrelated source files. Concurrent viewport/UI and evaluation changes visible in the worktree were not touched or claimed.

## Implementation

- Straight endpoints call `straight_section_resultants` at 0/1 and recover stress from the returned cut.
- Realized curved endpoints call the generalized curved section helper at 0/1. The helper rotates the recovered chord-frame j-end action to global and delegates equilibrium to `arc_section_resultants_with_radial_pressure`, which returns each actual tangent/radial/bend-normal arc frame.
- Raw endpoint force/moment rows still publish `corrected_local_forces` unchanged.
- Mechanical endpoint and curved-station metadata uses `coordinate_system=element_local` and `basis=recovered_from_local_element_stiffness`. Curved rows carry the exact required convention.
- `genuine_pressure_element_target` is shared by `build_pressure_thrust_loads` and `pressure_for_pipe`. It requires both `category=pressure` and `dimension=pressure`, while retaining the existing sum of every eligible record.

The independent curved frame/equilibrium proof is in `evidence/ARC_FRAME_PROOF_V1.md`. It compares all six endpoint and station resultants to a separately constructed curved-bend crate element, proves raw chord-frame rows against an independent `K d - p` solve, then checks recovered published stresses and exact metadata.

## Focused and full Cargo evidence

The complete attempt history is preserved:

1. Focused attempt 1 compile-failed on two test-code defects: a missing raw-loop `slot` binding and an out-of-scope `station` identifier.
2. Focused attempt 2 ran 3/4 PASS; one raw-row comparison hit a six-decimal rounding boundary, `-1541.1961 != -1541.196101`.
3. Focused attempt 3 ran 3/4 PASS; the corresponding station/public-row six-decimal boundary remained. The authorized tolerance correction was applied.
4. Final amended command `cargo test --manifest-path core/product_physics/Cargo.toml endpoint_section_cut -- --nocapture`: PASS, 8 passed, 0 failed, 137 filtered out.
5. Pre-regeneration full command `cargo test --manifest-path core/product_physics/Cargo.toml`: expected stale-fixture result, 144 passed and 1 failed (`generated_result_surface_matches_fallback_fixture_force_metadata`).
6. Final full command after atomic fixture publication: PASS, 145 passed, 0 failed; doc-tests 0 failed.

## Generated fixture and preservation

The exact generator payload was run to a P5 evidence tempfile:

`cargo run --quiet --manifest-path core/product_physics/Cargo.toml --example preview_result > execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260909-VIEWPORT-ROUTING/instances/P5/evidence/invented_mechanics_result.candidate.json`

This is output-equivalent to the registered `npm run generate:product-preview-mechanics` script, differing only in the destination required for atomic publication. `python3 -m json.tool` passed before publication. Candidate SHA-256 was `fb6724aafca965509b999390b3abd4a3eed6b8b85551a6b308ec5aa529c2a58c`. After the root/U7 release, the candidate was atomically renamed onto the fixture; a retained evidence copy is byte-identical.

`evidence/FIXTURE_PRESERVATION_COMPARISON_V1.json` records PASS: the old/new result-ID sets are identical at 830 rows, all 144 raw endpoint action rows are byte-equal, and all 378 station rows retain their numeric leaves. Eighty-five rows changed as expected for corrected endpoint stress values/metadata and their deterministic combination propagation.

## Fixture consumer inventory

Direct runtime and validation consumers:

- `apps/desktop/e2e/gui-workflow-validation.spec.ts`
- `apps/desktop/src/services/previewService.ts`
- `core/product_preview/service.py`
- `tests/test_analysis_run_records.py`
- `tests/test_results_schema.py`
- `core/product_physics/src/lib.rs` fixture-parity test

Related command/readiness/documentation references:

- `package.json` registered generator
- `apps/desktop/src/features/build-readiness/BuildReadinessPanel.tsx`
- `apps/desktop/SMOKE.md`

## Schema/headless evidence state

Root sealed and released `FULL_ENVELOPE_SCHEMA_WITNESS_PLAN_V1.md`, SHA-256 `2ffbcf56f9a6dad5ca34b6c20a362a5d1aadacafada9ef692b83e94d0cd95dc9`. The evidence-only scratch harness calls public `run_preview_in_memory`, requires `MECHANICS_SOLVED`, explicitly calls `result_envelope_binding::build_result_export_document`, requires equality with the runner-attached document, and emits three complete wrappers: straight-full, curved-tip-weight, and curved-pressure.

Scratch Cargo attempt 1 compile-failed only because the evidence harness passed `serde_json::Value` where the public request requires `PreviewModel`. After the confined conversion, attempt 2 passed and emitted all documents. The harness's dedicated P5 assertions passed: curved endpoint and station rows carry canonical enum values and the exact arc-frame/equilibrium convention.

The initial system-Python schema attempt failed before validation because `jsonschema` was absent. The environment-qualified Python at `{QUALIFIED_PYTHON}` supplied `jsonschema 4.26.0` and ran `Draft202012Validator.check_schema` followed by full `iter_errors` validation:

| Complete document | SHA-256 | Schema errors |
|---|---|---:|
| straight-full | `004ea7feba57919c335931a0471f0888e74edf8410ee7a8fbeea9c118482a705` | 160 |
| curved-tip-weight | `bfeaa07893d74d7b500aa691efb02c92431f338f6b57ad1cf30429bbe6e21e58` | 18 |
| curved-pressure | `60418440ce86957d6aae7b8f4a5b56843b2d5c34ace6125ee2f0c32e5108bd20` | 18 |

`schema_witness/VALIDATION.json`, SHA-256 `bba2573874051bcce6cc8d21b4c52eb32433e63b2d0040f32402f95a07a18f34`, records all 196 errors. The failures do not name corrected endpoint/station rows. They expose broader existing mismatches: headless checksums serialize canonicalization as `rfc8785_jcs` while the schema admits `JCS`, and exported displacement/component-review rows carry metadata values outside the schema's closed component/basis/location/coordinate-system enums. The sealed escalation forbids widening P5 source/schema scope, so these failures are returned unchanged.

A manager-side unreserved headless `--list` invocation is an external tooling observation only and is not validation evidence.

The final explicit write-scope validation passed with zero violations; `evidence/SCOPE_VALIDATION_V1.json` binds the product, fixture, run record, P5 return/evidence, and schema-witness paths to the four authorized roots.

## Remaining limitations and unchanged semantics

- The failed complete headless/schema route requires root disposition, and independent 100% review remains required before DEC-025.
- Legacy broader pressure semantics remain unchanged: the current effective-force-like resultants, absence of Poisson pressure behavior, absence of closure-topology inference, and existing longitudinal pressure row design were not redesigned.
- No schema, public quantity, result kind, diagnostic vocabulary, topology, dependency/DAG, lifecycle, decision, receipt, `MEMORY.md`, or `_STATUS.md` surface changed.

MISSING: a passing complete headless/schema envelope result and the later independent-review verdict.
NEEDS_HUMAN_RULING: root must route or disposition the out-of-scope headless producer/schema mismatch before DEC-025.
DEPENDENCY_NOTES: source and fixture remain frozen; DEL-05-03 stays IN_PROGRESS until combined review PASS.
