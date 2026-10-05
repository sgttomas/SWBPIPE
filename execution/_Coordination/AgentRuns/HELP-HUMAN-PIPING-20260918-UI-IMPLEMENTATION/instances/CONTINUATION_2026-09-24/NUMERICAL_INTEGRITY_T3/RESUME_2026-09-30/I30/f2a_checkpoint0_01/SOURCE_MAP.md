# I30 — bounded interfaces, prospective files and acceptance map

Paths below are relative to P = `projects/chirality-piping`. `new` means proposed,
not created by I30. This is the candidate maintained set for ROOT to freeze after
CP0 proof/wire and I29 reconciliation. It is not a write grant or a claim that all
dependencies are already resolved. No other maintained path is implicitly allowed.

## Source-to-interface map

| Actual source at inspected main | Interface fact / proposed change |
|---|---|
| `core/product_physics/src/lib.rs:2141,2151,2176,2214` | Typed/captured/common entries; typed has no custody. One actual invocation owns W1 context and fallback. |
| `core/product_physics/src/source_receipt.rs:100–136` | Raw custody exists, but checked digest failure aborts parse; implement accepted S5-R availability state without typed synthesis. |
| `core/runner/headless/src/lib.rs:741–752,1013`; D2 §4.8 | Checked qualified evidence is optional; raw ordinary result returns with canonical export unavailable. Existing legacy runner checksum is not substituted for checked custody. |
| `apps/desktop/src/services/previewService.ts:102–130,193–229`; `apps/desktop/src/features/workspace/workspaceSession.ts:847–916` | Direct/job registration fails closed when capture/result checked hash fails; workspace then refuses before storing result. Optional PP digest alone does not give native fallback display. |
| `core/product_physics/src/lib.rs:2364,2476,2515,2524` | Sparse basis storage, early terminal returns and late source-selection reduction require reviewed deferred arbitration. |
| `core/product_physics/src/lib.rs:3425–3708` | Ordinary ledger → solver → exact-block selection; preserve exact-block budget/selection and original reports. |
| `core/product_physics/src/lib.rs:3980,4033,4236,4504,4655,4736` | Actual displacement/magnitude/support/end/station/stress publication seams; avoid recovery from rounded displacement. |
| `core/product_physics/src/preview_physics.rs:528,638,854` | Final support/maximum replacement, component SIF row, combination rendering; certify resulting ids/values after these seams. |
| `core/product_physics/src/lib.rs:9545,9583,9680,11203` | Span statics, circular maximum, open summary and stress formulas; actual section operands are proof inputs. |
| `core/solver/frame_kernel/src/structural/retained/{source.rs:166,352;recover.rs:55,107}` | SourceParts/constructor and QuantityId/layout; strict identity, station signs, support/magnitude coverage. |
| `core/solver/frame_kernel/src/structural/retained/adaptive.rs:3383,3423,3462,3938,4342` | Evidence, owned solve/radii, private accessor, schedule and call-local cache. No report or public arithmetic helper access. |
| `core/solver/frame_kernel/src/structural/retained/combine.rs:1,80` | Own combined-ledger solve; selected-operand-only API and snapshot-cache scope; ordinary operand gap. |
| `core/reporting/result_export/src/{semantic_contract.rs:433,456;derivative.rs:97,331}` | Rust admission/standing/binding and derivative receipt copy/validation. |
| `core/reporting/result_export/src/physics_source.rs:896` | Private actual_materials helper; narrow parameterized reuse needed for successor G8, without old-policy rewrite. |
| `core/analysis_runs/{compatibility.py:217,408,520;physics_source.py:409}` | Python admission/standing/binding/carrier checks and actual-material facts. |
| `apps/desktop/src/features/results/{numericalResultQuality.ts:55;knownSemanticLimitations.ts;resultSemantics.ts}` | TS header/admission/standing, notices/binding and pinned table dispatch. |
| `apps/desktop/src/features/results/physicsSourceRecovery.ts:301` | Private async authored-fact normalization calls the existing WASM units engine; native/WASM boundary must be exercised. |
| `apps/desktop/src/services/analysisRunCompatibility.ts:142,159`; `apps/desktop/src/types.ts:504,660` | AnalysisRun receipt retention and validation; two typed carrier shapes need the successor member. |
| `apps/desktop/src/features/result-export/resultExportAdapter.ts:58,118,172` | Output refusal must remain explicit when sourceContract gains successors; no accidental T6 packaging admission. |

## Exact prospective maintained set

Braces below expand to exactly the named files, not arbitrary siblings.

| Boundary | Existing files proposed for edit | New files proposed |
|---|---|---|
| Kernel integration accessor | `core/solver/frame_kernel/src/structural.rs`; `core/solver/frame_kernel/src/structural/retained/adaptive.rs`; `core/solver/frame_kernel/tests/retained_k4/publication_tests.rs` | none |
| Producer W1a and S5-R prerequisite | `core/product_physics/src/{lib.rs,source_receipt.rs,preview_physics.rs}` | `core/product_physics/src/{retained_source.rs,retained_publication.rs,retained_receipt.rs}`; `core/product_physics/src/retained_publication/bounds.rs`; `core/product_physics/tests/{retained_publication.rs,retained_routing.rs}` |
| Rust S-G1 + canonical derivatives | `core/reporting/result_export/src/{lib.rs,semantic_contract.rs,derivative.rs,physics_source.rs}` | `core/reporting/result_export/src/retained_precision.rs`; `core/reporting/result_export/tests/retained_precision_contract.rs` |
| Python S-G1 + AnalysisRun | `core/analysis_runs/{compatibility.py,physics_source.py}` | `core/analysis_runs/retained_precision.py`; `tests/{test_retained_precision_contract.py,test_retained_precision_schema.py}` |
| TS S-G1, display, binding | `apps/desktop/src/features/results/{numericalResultQuality.ts,resultSemantics.ts,knownSemanticLimitations.ts,KnownSemanticNotices.tsx,physicsSourceRecovery.ts}`; `apps/desktop/src/{types.ts,services/analysisRunCompatibility.ts,services/previewService.ts,features/workspace/resultsSessionState.ts}` | `apps/desktop/src/features/results/{retainedPrecision.ts,retainedPrecision.test.ts,retainedPrecisionIntegration.test.tsx}`; `apps/desktop/src/services/retainedPrecisionAnalysisRun.test.ts` |
| T6 output fence | `apps/desktop/src/features/results/loadReferenceOutputAvailability.ts`; `apps/desktop/src/features/result-export/resultExportAdapter.ts` | `apps/desktop/src/features/results/retainedPrecisionOutputRefusal.test.tsx` |
| Atomic tables/schema | `schemas/{results.v0.3.schema.yaml,analysis_run.v0.3.schema.json,stress_neutral_export.v0.3.schema.json}` | `schemas/retained_precision_mp_v2.schema.json`; `fixtures/results/{semantic_contract_v0_3_preview_physics_retained_1.json,semantic_contract_v0_3_physics_retained_1.json,retained_precision_cases.json}` |
| Both-entry/headless bridge checks | none | `core/runner/headless/tests/retained_precision_admission.rs` |
| Required bounded V-P product integration | `validation/benchmarks/numerical_robustness/{Cargo.toml,Cargo.lock,src/lib.rs}` | `validation/benchmarks/numerical_robustness/src/product.rs`; `validation/benchmarks/numerical_robustness/tests/product.rs` |

Conditional additions, **not** part of the preferred base set until selected:
`core/solver/frame_kernel/src/structural/retained/combine.rs` and
`core/solver/frame_kernel/tests/retained_k4/combine_tests.rs` for a source-based
combination entry; exact cache/session API needs CP0 proof before broader files.
Native unsafe-digest inspection requires its own concrete state/authority decision;
likely additional seams are `apps/desktop/src/features/workspace/workspaceSession.ts`
and its solve state/display tests, but no open-ended write grant follows from that
dependency. Do not call the base set a completed native fallback implementation.
I29's separately owned admission files/interfaces must be enumerated by ROOT on
integration; this packet neither invents them nor authorizes duplicate guards.

New producer raws/requests and carrier examples should be embedded in the single
`retained_precision_cases.json` positive/negative corpus where practical. Before any
separate fixture files are needed, freeze their exact names and source/hash manifest.
Do not regenerate historical fixtures/tables/oracles or alter protected predicates.
No existing Cargo manifest needs a new arithmetic dependency. The bounded product
lane alone needs its PP path dependency/lock closure; validate its actual lock delta.
Do not add a general framework, policy registry, host script, library or unit engine.
Run records/documentary integration outside I30 are ROOT's separately owned work.

## Identity collision check (proposal, not reservation)

At HEAD252e97404, fixed-string search under P/{core,apps,schemas,fixtures}, excluding
target/node_modules/dist, returned no match (rg exit1) for all six proposed strings:

| Proposed identity | Role |
|---|---|
| `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Preview successor; already a D1 suggestion, no maintained registration found. |
| `openpipestress.result_semantics/0.3.0/physics-retained-1` | Exact-profile successor; no maintained registration found. |
| `product_preview_retained_w1a_v2` | Distinct preview formulation profile. |
| `exact_straight_retained_w1a_v2` | Distinct zero-pressure exact formulation profile. |
| `retained_precision_receipt_mp_v2` | Proposed receipt hash domain. |
| `retained_precision_publication_mp_v2` | Proposed publication hash domain. |

The checked canonical profile and `M03-INTEGRITY-MP-v2` are intentional reuse, not
new reservations. Capture's existing source_blocks_invocation_v1 payload can be
reused only with its exact raw-request-plus-mode scope pinned in the new table.
No conclusion is made about untracked work on another branch. Recheck at grant/freeze.

## Bounded validation and checkpoints (all future/unrun here)

| Check | Concrete controls/oracle; acceptance |
|---|---|
| CP0 independent proof | For every covered recipe: source truth, exact-bit operands, interval math, final unit normalization, raw relative predicate and actual b. Independent exact rational/simple analytic source controls plus directed sqrt references; no producer self-oracle. Include nonzero mm normalization error, alternate input units with newly decoded primitives, cancellation, zero, subnormal, large/small L, section ratios and sign/orientation changes. |
| Accessor/lifetimes | Existing A1 mismatch tests plus foreign case/source/precision/body/kind/index, absent radius vs exact +0, mutated class, partial draft drop, borrow/move and actual clone counts. No Serialize of private radii; no stored report; combination gets its own certificate. |
| Main routing controls | Both modes × both public entries; no-capture/unsafe-digest fallback, ordinary Passed, Sensitive, D5 demotion, supported NegativeEnergy, scaled Range, invalid/asymmetric/mechanism, unsupported zero-valued producers and nonlinear no-op. Include early failing case followed by would-be exact selection, exact selected followed by unrecoverable case, and all-no-exact cases. |
| Coexistence | Every committed source-blocks/physics-source request in both modes, plus later-case exact selection: compare complete serialized envelope bytes and full hash to same-platform main; prove W1 work=0. Keep joined 0.4.0, source-block current/history and nonlinear routes unchanged. Stock N05/N06 that select exact-block must keep that route. |
| Actual new-route rows | Skew/order>2 N05/N06-class cases where exact-block is naturally unavailable, RF-CHAIN/RF-SKEW/D5 true positives, independent source truth and protected all-row comparisons. Verify selected publication token/policy only after certificate and G5 preflight. Never bypass coexistence just to make a positive test select W1. |
| Combinations | All-selected, mixed ordinary/selected, repeated source ids, independent stiffness/layout/station/support mismatch, A+B−A2 cancellation, (P,epsilon)−P escalation, subtraction/range and T0R gates; stopped combination leaves operand values/standing unchanged. Mixed policy is selected before accepting its results. |
| Resource behavior | Frozen independent stage/work ledgers; same-batch reuse vs separate calls; exhausted case/invocation and recorded overshoot; rejected/failed verification and certificate work; fallback does not reset meter. Lowered-limit guard tests and I29 preallocation/allocation-failure/drop checks preserve ordinary rows/standing. No self-reported count as its own oracle. |
| G0–G8 parity | Shared producer positives and one discriminating mutation per branch in Rust/Python/TS: policy v1/unknown, hash/profile/table, unsafe number/nonfinite/negative-zero bits, wrong order/coverage, token/diagnostic mixing, p/2p/reuse/work loss, stop ratio one ulp high, E/Φ/b/class mutation, wrong DOF/hanger/pressure/member/material/mode/source association. Independently rehash semantic mutations to reach their intended branch. Assert exact failure code/order. |
| Binding/carriers/UI | Missing invocation stays needs_recompute; transport never eligible; absolute/NotCovered quantity and headline cannot pass. Raw unit/b_SI display mismatch killed; duplicate/missing class lists, stale receipt, dropped AnalysisRun copy, derivative disclosures, save/reopen and T6 output refusal all covered. Label-only promotion must fail. |
| Required regression | Preserve all R7/A1 publication/floor/accuracy controls, C17 and the selected F17 scope, K4/O9/zero/range, K6c accounting/envelope, S11 ledger, K2b/W2, DEC-053 nine observations and existing three-language source/physics/preview/load-reference tests. Run unchanged frozen R1 truth; report expected-unresolved differences, never rebaseline to pass. |
| Paired T9 + both-entry | Reconstruct the current complete committed-fixture inventory and same-platform main baseline, then candidate in the same admitted environment. Do not assume historic T9's112 count is current. Both entries use full-envelope hashes; no-Passed-breach exceptions remain empty. Report refused/uncovered separately from passed, including typed capture-ineligible routes and authorized large-case omissions/timeouts. |
| Native witness | Admitted owner-Mac build: natural W1-selected skew/chain case in each mode, stock exact-block N05/N06 continuity, mixed cases, outside-domain refusal/ordinary result, source-unit re-entry, actual result inspection/class notices/binding refusal, save/reopen, interruption/recovery and export refusal. Record model/case/build/command/environment and actual observations. Browser/CI is no substitute for native/WASM behavior. |
| Frozen final gates | Fresh independent complete-diff review and repair backcheck; registered desktop build/tests, Python tests and clean DEC-025 sweep; practitioner self-check/receipt obligations; hosted numerical CI plus full surface-4 dual-viewport dispatch on actual candidate/base; paired T9/both-entry/native evidence invalidated by later changes must be rerun. Preserve failures/skips and platform qualifications. |

The product lane's bounded correctness adapter is required for F2a; broader V-P
scale/timing/memory qualification and VP-ORACLES/VP-ROBUST closure follow in their
accepted order. Retirement condition3's row/check-level side-by-side parity is a
future F2b/F3 gate after S-I, not an excuse to retire now or a claimed pass here.
PHYS-R4, LEF-small/large, the 1,000-member native/scale observation and other D1
§7.5 obligations remain explicitly scheduled/qualified; their accepted criteria
and owner resource decisions are unchanged. Any realistic Passed breach or changed
protected ordinary availability stops the affected acceptance and returns to ROOT.
