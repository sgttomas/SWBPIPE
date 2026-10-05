# Snapshot 05 plan (I62 checkpoint C0)

**Status: plan only, for ROOT disposition. Nothing is built.**
- No READER file or shared file was edited, and there were no Git writes.
- Run window: 2026-10-03T20:09:13Z to the freeze below.
- Steering: "I62 checkpoint B verified; snapshot 05 is planned before it is built" (NUM 027c0912e7).
- Paths use the brief's placeholders. FK is P/core/solver/frame_kernel/src/structural/retained, and PP is P/core/product_physics/src.
- All file:line references are at CODE/NUM 652ad0cc1f. NUM's working tree has no diff from it for FK/PP.

## 0. Rules every snapshot-05 item follows

### Native-faithful

1. **Every base starts from a model the preview route admits.** It is edited only through the invocation.
2. **Every structural record is derived by a cited native rule**, then encoded with the reader-side helpers I58 and I62 already use. Structural records here are:
   - id maps, bodies, layout, constraints and nodal terms;
   - K4SRC/K4STF;
   - the native schedule shape, cache and build slots;
   - the stage/check/lane prefix;
   - the coverage array.
3. **Private values are explicit synthetic attestations, labelled synthetic**, under the existing trust boundary (C1 §6, C3 §4, I57 §5). Private values here are: work magnitudes, stop-rule comparisons, rejection triggers, numerical summaries, and private nonzero/data predicates that no public rule fixes. No base claims a solve.
4. **Where a source path exists but the trigger is not a plausible input,** the item says so and proposes deferring it.

### Coverage

- Coverage is derived per I57 §2 from the base's own layout, extent, E, floor and data blocks. The build asserts the same relations that `build_corpus_04.py:derive()` asserts.
- The floor is never invented. At p512 it is the native Φ = `phi_512(ê)` (§1.4).

### Rehash scopes

All mutations use `rehash: "all"`, the same harness as snapshot 04.
- **The receipt** always rehashes.
- **The publication** rehashes only if a non-receipt field changed.
- **Preparation** rehashes only if a member, owner, ordinary or material reference changed.
- **Source identity** rehashes only if the source or its preparation reference changed.
- **The invocation digest, K4SRC/K4STF and group stiffness** are recomputed only for new bases whose invocation changed. This uses `rp._native_source_encoding` and `rebind_invocation`.
- **The 77 snapshot-04 mutations stay byte-identical.** Snapshot 05 is purely additive, unless ROOT rules on §6 item (a).

### Expected outcome

- One gate and code, identical in Python, Rust and TypeScript (shared contract).
- **Positive bases** must pass G0–G8 with their `expected_classifications`; `numerical_eligible` stays false.
- **Unavailable positive bases** pass with `status: unavailable`, with classifications only for selected cases.

## 1. Remaining I57 §5 controls

### 1.1 Two bodies with distinguishable constraints: base `two_body_synthetic` (p128)

**Native path**
- Bodies are member-connected components numbered by lowest node (`FK/source.rs:665–716`).
- An unanchored second body is a mechanism and not solved (`PP/lib.rs:13692–13718`). So **both bodies are anchored cantilevers.**
  - Body 0 is case 0's loaded span.
  - Body 1 is a new unloaded anchored span at y=5, with no primitive loads.
- Layout comes from `FK/recover.rs:101` (the reader's G8 mirror). E, the bound, theta and `data_blocks` are per body (`FK/verify.rs:1083–1138`, `FK/adaptive.rs:2569–2607`, as RV76 cites).
- An unloaded body's zero coverage is natively witnessed: zero loads give `has_data=false`, stop `[F;4]` and no B (`PP/retained_product_tests.rs:1442–1453`).

**Coverage:** `[{0,[T,T,T,T],T},{1,[F,F,F,F],F}]`. Body 1 has E=0, so no estimate/charge/B entries, θ=+0, a null bound, and `data_blocks` 1.

**Synthetic parts:** body 0's stop bits, E and B, as in snapshot 04. Body 1's zeros are source-forced.

**Mutations**

| Id | Edit | Expected first failure |
|---|---|---|
| `two_body_swap_has_data` | has_data swapped between the bodies | G5a SCALE (B list, then the free-load rule on body 0) |
| `two_body_swap_stop_with_rosters` | stop vectors **and** stop_rule entries swapped consistently | **Passes every public check.** Body 1 may attest stop `[T;4]` (it has non-input rows of every kind), and body 0 may attest `[F;4]`. Only producer custody/replay catches it. Listed as an undetectable positive variant (a Python-only "must pass" test), **not** a shared failure control |
| `two_body_swap_stop_only` | stop vectors swapped, stop_rule unchanged | G5a SCALE (stop list) |
| `two_body_body_order_swapped` | coverage `[{1…},{0…}]` | G3 COVERAGE |
| `two_body_missing_body` | coverage `[{0…}]` | G3 COVERAGE |

**Rehash:** the new base rehashes everything. The mutations rehash the receipt only.

**Effort:** high, about 2.5 h.
- Second-body maps, K4SRC/K4STF, group stiffness and layout.
- 70+ new result rows and classifications.
- Contract evidence: extrema coverage partition and support attribution (`preview_physics_evidence.py:288–360`).
- G7/G8 rebinding.

### 1.2 Absent kind, and L=0 against L≠0

**Absent kind is not natively producible on this route. I propose dropping it as a base.**
- Every node contributes six displacement rows, so translation and rotation are always present (`FK/recover.rs:101`; the reader's G8 mirror adds all six components per node).
- A solvable body needs force and moment rows. A membered body has end actions. An isolated node must have every DOF restrained or sprung, or it is a mechanism (`PP/lib.rs:13692`), and that yields reaction or spring-action rows of both kinds.
- The faithful substitute is **"present but no non-input row"** (below). It exercises the same `A[k]` exclusion in the feasibility rule.

**L=0, base `isolated_node_synthetic`:** case 0 plus one memberless, fully restrained node.
- A single-node body has extent 0 (`FK/adaptive.rs:321`). `coupled_scales` and `e_hat` keep the uncoupled values (`FK/adaptive.rs:338–352`, `FK/verify.rs:321–334`).
- The body's six displacement rows are all input-derived. Its translation has one non-input `displacement_magnitude` row; **rotation has no non-input row.** Its force and moment come from six reaction rows.

**Coverage for body 1:** stop `[F,F,F,F]`, has_data `false`.
- No free DOF forces has_data false.
- Unloaded constrained reactions are zero, as a private attestation consistent with the source.

**Build blocker:** I have not established that the preview producer admits a node no member references. Before building, C1 must confirm admission in the PP model/route source. If it is refused, **defer L=0**: no other native path produces a single-node body.

**Mutations, if admitted:**

| Id | Edit | Expected first failure | Reason |
|---|---|---|---|
| `isolated_rotation_stop` | body 1 stop[1]=T, plus a stop_rule entry | G5a SCALE | feasibility: no non-input rotation row |
| `isolated_has_data` | body 1 has_data=T, plus B and record bound | G5a SCALE | no-free-DOF rule |
| `isolated_estimate_coupled` | body 1 E=(f>0, m=0) in selection and record; estimate `[force, moment]` | G5a SCALE | L=0 keeps the hats uncoupled, so only `force` is required |
| `isolated_estimate_uncoupled` | as above with estimate `[force]` | positive (Python-only must-pass) | |

**Effort:** moderate-high, about 1.5 h, after admission is confirmed.

### 1.3 p512 with zero and positive floors: base `p512_ladder_synthetic`

The case is two bodies (1.1's geometry), reached through the native ladder.

**Native path**
- `run_schedule_inner` always starts at c=0, p=128 (`FK/adaptive.rs:4519–4521`).
- A rejected candidate makes its verification the next candidate (`VerificationThenCandidate`, `pending`) for stop-rule rejection (`4738–4758`) and publication-enclosure rejection (`4713–4735`).
- Accepted plus Verified ends the ladder through `finish_selected` (`4696–4711`).
- The records are:

  | Record | Precision | Role | Outcome |
  |---|---|---|---|
  | rec0 | p128 | candidate | rejected |
  | rec1 | p256 | verification, then candidate (reused) | rejected |
  | rec2 | p512 | verification, then candidate (reused) | accepted |
  | rec3 | p1024 | verification | verified |

- There are 3 attempts, the second and third with `origin: reused_verification`.
- Residual bases are 192/320/576/1024; limbs are 4/4/8/16 (`FK/adaptive.rs:1–9`).
- Cache and build slots are s128, v256→reuse, v512→reuse and v1024, per the existing G5 slot rules.

**Floor**
- When `verification_precision == 1024` and a report exists, Φ = `[phi_512(ê_fo), phi_512(ê_mo)]` per body (`FK/adaptive.rs:2294–2307`).
- `phi_512` = fl↑(2^-438·ê) (`FK/verify.rs:365–376`), and ê = `e_hat(E, L)` (`FK/verify.rs:321–334`).
- So a floor component is **positive if and only if ê>0**:
  - body 0 (loaded, E>0) has a positive floor;
  - body 1 (unloaded, E=0) has a zero floor.

**Coverage**
- Body 0: stop `[T,T,T,T]`. The positive floor forces force/moment positive (`final_case.rs:1402–1409`).
- Charge = `present ∧ positive` (`final_case.rs:1434–1438`), equal to stop[2..3].
- Body 1: stop `[F,F,F,F]`, has_data F, zero floor, so no charge.
- `[{0,[T,T,T,T],T},{1,[F,F,F,F],F}]`.

**Synthetic parts:** the two rejection triggers (stop-rule comparisons are private), E, work magnitudes and B. The schedule shape, floors and coverage are source-derived.

**Mutations**

| Id | Edit | Expected first failure | Reason |
|---|---|---|---|
| `p512_positive_floor_forces_stop` | body 0 stop[2]=F; stop_rule and charge drop force | G5a SCALE | feasibility |
| `p512_charge_follows_estimate` | body 1 (zero floor, E=0, so estimate empty) attests stop `[T,T,T,T]` with a consistent four-entry stop_rule; charge left empty, following the estimate | G5a SCALE | the charge list must be `[force, moment]` = stop at p512. This feasible stop (non-input rows of every kind; no floor forcing) separates the p512 rule from the lower-p rule. The same edit with charge `[force, moment]` is a Python-only must-pass positive |
| `p512_floor_null` | floor null | G5a SCALE | existing p/floor rule |
| `p512_floor_not_phi` | body 0 Φ changed by 1 ulp | **ROOT ruling needed (§6 a)** | |
| `p512_fresh_first_attempt` | first attempt p512, fresh | **ROOT ruling needed (§6 a)** | |

On `p512_fresh_first_attempt`: the Rust reader already checks the "p128/fresh schedule start" (I59). Python and TypeScript must align; the expected failure is G5 ATTEMPT if ROOT adopts it.

**Effort:** the highest, about 3 h. It depends on 1.1.

### 1.4 Second owner or proof: base `two_case_synthetic`

**Native path:** one model with two load cases.
- Two owners, two ProductAttempts, two Runs in one call, positions 0 and 1, and one group sharing stiffness. This uses the C2 call/group/build graph that `_g5_native` already checks.
- Case 0 is loaded. Case 1 has every load at 0, so it carries the no-data coverage witnessed at `PP/retained_product_tests.rs:1442–1453`.

**Mutations**

| Id | Edit | Expected first failure |
|---|---|---|
| `copy_flags_into_loaded` | case 1's no-data coverage copied into case 0, with case 0's stop_rule/B/record made consistent | **detectable:** G5a SCALE. Case 0's individually nonzero free-DOF terms force has_data=T |
| `copy_flags_into_unloaded` | case 0's coverage copied into case 1, with case 1's stop_rule/B/record/data_blocks made consistent | **passes every public check**, so it is producer custody only. Case 1 has free DOFs and no nonzero term, so has_data=T is attestable, and stop `[T;4]` is feasible. Goes into the Python-only must-pass list |
| `rebind_source_run_only` | attempt 1 `source_ref`/`run_ref` → case 0's, owner preserved | G5 PRODUCT_ATTEMPT. The Run/source owner differs from the attempt owner, so `_g5_products`/`_g5_coverage` rejects it |
| `owner_only` | | G3 (exists) |

**Effort:** high, about 2.5 h: two runs, call positions, invocation accounting, and two result sets.

### 1.5 Every I57 §3 failure row, with unavailable and failed-certificate attempts

All of these derive from one shared **unavailable-case template**:
- case `status: unavailable` with the C3 reason code and phase (`_g5_products` mapping);
- no recovery_method on rows (G6);
- an UNAVAILABLE diagnostic (G4);
- `result: {kind: unavailable, error}`.

Each row lists its native path and coverage. The bases are positive. "Mutation" lists the detectable negative.

| Base | Native path | Stages, proof and coverage | Mutation → expected |
|---|---|---|---|
| `fail_preparation_synthetic` | `PreparedCaseFailure::typed_trace` passes no proof (`PP:3605–3611`) | proof `null`; no coverage object | — |
| `fail_native_synthetic` | native refusal or unresolved: `finish_terminal` (`FK/adaptive.rs:4546–4552`); `native_refusal_trace` passes no proof (`PP:3614–3617`) | run terminal `refused`/`unresolved`, proof `null` | — |
| `fail_lane_k_synthetic` | `begin_prepared_product` → `retain_lane` on the AdmittedK error (`final_case.rs:1572–1574, 344`); `Proof` error (`PP:3469`); proof work kept (`PP:3629`) | proof_start failed, lanes `[K failed]`, coverage `null` | coverage non-null → G5 PRODUCT_ATTEMPT |
| `fail_lane_source_synthetic` | Source lane error after K (`final_case.rs:1575–1577`) | lanes `[K completed, Source failed]`, `null` | same |
| `fail_maxima_abandon_synthetic` | `prepared_maxima` Err → `Abandoned{abandon_values}` (`PP:3475–3476`); `completion_merged=true` (`final_case.rs:1766–1767`) | maxima failed, values not entered, completion `merged`, `null` | completion `separate_failure` → G5 (existing I59 rule) |
| `fail_values_synthetic` | `complete_maxima` Err → `Values{proof:abandon()}` (`PP:3480–3481`) | values failed, completion `separate_failure`, `null` | |
| `fail_cert_before_summary` | `certify_final` fails at the frozen owner/shape/descriptor checks or `visit()` accounting **before** `check_intervals` (`final_case.rs:1777–1787`), so `spent.coverage` stays empty (1195 not reached) | certificate failed, `null` | **Trigger caveat:** the shape/descriptor checks are defensive and unreachable for a correct producer. The only plausible trigger is a resource/accounting fault. **Propose deferring** until a real accounting-fault trigger is specified, or label the trigger synthetic and say so |
| `fail_cert_after_summary` | `check_intervals` assigns coverage atomically (`final_case.rs:1195`), then a predicate fails (`1789–1790`, `Cause::Predicate`); `PreparedCandidateRefusal` keeps the certificate work (`PP:3626–3634`) | certificate failed, coverage **complete**, unavailable `facade_certificate` | coverage null **passes** publicly (null is allowed on a failed certificate; only producer custody distinguishes them); a coverage roster defect → G3/G5a as usual |
| `fail_after_cert_adapter_copy` | certificate passed (`PP:3493–3494`), then `prepared_verdict_copy` fails partway (`PP:3362–3366`). The proof-owned vector is complete and the adapter copy is partial and **never** published | certificate passed, observables not entered or failed, coverage **complete** | **coverage null → G5 PRODUCT_ATTEMPT** (a passed certificate requires coverage); a short (prefix) roster → G3 COVERAGE |
| `fail_g5a_after_cert_synthetic` | certificate passed, G5a check failed (receipt `checks.g5a` failed; `retained_receipt.rs:114–119`) | coverage complete | coverage null → G5 |

**Effort:** the template is about 1 h. Each extra row is about 20–30 min, so about 4 h for the set. Rows to drop or defer:
- `fail_cert_before_summary`, as above;
- `fail_preparation_synthetic` and `fail_native_synthetic`, which duplicate §2's prefix cases. Build them once there.

### 1.6 Positive cancelling ±x loads: base `cancelled_loads_synthetic`

**Native path**
- `PP/retained_product_tests.rs:1460–1487` is an actual execution witness. ±1 terms on a zero specimen give:
  - has_data **true** and estimate `[F,F]`;
  - B required: removing B fails `B data coverage`.
- The data definition comes from the individual nonzero ledger terms: `FK/ledger.rs:151` `nonzero_term_spent` and `FK/bound.rs:162` `fill_data_blocks`.
- The source keeps the two terms separately: the nodal_terms sort keeps distinct source_id/value (G8 mirror).

**Build:** from the no-data base, replace the six zero loads with two loads at node 1 UX, +1000 N and −1000 N.
- Rows stay zero.
- E=0, so no estimate/charge entries.
- has_data T, B one positive (synthetic value), record bound non-null, `data_blocks` 1.
- Two category diagnostics; K4SRC rehashed.

**Coverage:** `[{0,[F,F,F,F],T}]`. Stop F is consistent with an exactly zero state, as a synthetic attestation; the witness test does not assert stop.

**Mutations**

| Id | Edit | Expected first failure |
|---|---|---|
| `cancelled_no_data_claim` | has_data F, B removed, record bound null, data_blocks 0 | G5a SCALE (direct free-DOF nonzero-term rule; net or zero rows never imply no data) |
| `cancelled_drop_B_only` | B removed, record rebound | G5a SCALE (B list) |

**Effort:** low, about 40 min. **Do it first among the new bases.**

## 2. Shared failure-prefix controls (I59 RETURN item 3; I58 list)

These are all unavailable bases on the §1.5 template. Each is positive.

| Base | Native path | Expected record shape (existing C3 rules) |
|---|---|---|
| `prefix_captured` | Failure before `old_coverage=Complete` (`PP:3141–3165`): prior capture cause (3145–3147), custody/permit (3150–3154), or old-source/facts coverage (3157–3159) | old_coverage `captured_prefix`; preparation and new empty; source_ref/run_ref null; unavailable `preparation`; proof null (`_g5_products` captured_prefix rule) |
| `prefix_pre_helper` | After 3165 and before the first `prepare_product_annulus`: reserve/capture failures (3167–3198) or old-to-old section mismatch (3202–3205) | old_coverage `complete`; preparation members empty, so no fabricated zero-work entry (C3_DELTA) |
| `prefix_helper_refused` | `prepare_product_annulus` Err (3215–3222) | members `[prepared…, refused]`; new shorter (`fail(j == len(pm)-1 and j >= len(new))`) |
| `prefix_unequal_helper_new` | Helper succeeds; failure before `evaluate_member_operational` (3224–3241) | `len(new) < len(pm)`. The trigger for 3224 (`prepared input bits`) is defensive; 3226–3241 are adapter accounting. **Same trigger caveat as `fail_cert_before_summary`.** Recommend it only if ROOT accepts synthetic accounting-fault triggers |
| `prefix_source_construction` | `PrimitiveSource::new(parts)` Err (3249), e.g., a prepared area making the source invalid; test fault at 3248 | members complete; `source_ref` null; no preparation hash; C2 `source_decline` keeps the constructor error. **The plausible input trigger must be identified in C1** (an invalid prepared section the helper accepts but the source refuses); otherwise defer |
| `prefix_old_err_new_ready` | The ordinary path's old operational (`PP:1296`) is refused while the prepared new operational (3242) succeeds | Already a Python-only positive (`test_old_operational_error_is_retained_independently_of_new_ready`); **promote it to a shared base.** Low effort |
| `prefix_k_only` and `prefix_source_failed` | = §1.5 `fail_lane_k`/`fail_lane_source` | Lane work kept, as for K-only/Source-failed work |
| `prefix_post_native_unavailable` | = §1.5 `fail_native`. Selected native, then later unavailable = §1.5 rows with run terminal `selected` | |
| `prefix_maxima_abandon` | = §1.5 `fail_maxima_abandon` | Merged completion before values (I59's correction) |

**Effort:** about 3–4 h in total once the template exists. Each also serves the readers' remaining resume step-3 audit.

## 3. Promotions to shared mutations (on snapshot-04 bases)

These need no new base: all edits are on `ordinary_prepared_synthetic`.

| Id | Edit | Expected first failure |
|---|---|---|
| `layout_force_row_input_derived` | a reaction force row `input_derived=true` | G5a SCALE |
| `layout_constrained_displacement_not_input_derived` | first input-derived row → false | G5a SCALE |
| `layout_nonzero_prescription` | `constraints[0].value` = 1.0 bits | G5a SCALE |
| `coverage_null_and_product_work` | coverage null + `product_attempts[0].g5a_work.lost=true` | **G5 PRODUCT_ATTEMPT** (association before the Ready-block WORK, per I57) |
| `product_work_only` | `g5a_work.lost=true` | G5 WORK |
| `native_work_then_coverage_null` | coverage null + `cases[0].run.case_charge=18` | **G5 WORK** (existing schedule first) |

- The three layout controls are what checkpoint B tests in Python only.
- The last three rows pin I57's order: schedule, then association, then the original WORK pass.
- All rehash the receipt; the layout and prescription edits also rehash source identity.
- **All six expected failures are confirmed against the current Python reader.** A read-only probe ran on snapshot 04 with no edits (WT/scratch/i62_coverage_shared_python_01/c0_probe_promotions.py and .log).
- `g5a_work` is the existing ScalarTrace `{entered, checks, lost}`, so the edit is shape-valid at G1/G2.

**Effort:** about 20 min. **Do these first.**

## 4. Python G5a direct checks for unavailable attempts that keep complete coverage

**Scope:** a non-null `summary_coverage` on an attempt whose case is not `selected`, in rows `fail_cert_after_summary`, `fail_after_cert_adapter_copy` and `fail_g5a_after_cert`. G3 and G5 already apply to these (checkpoint B).

**Proposed additions, at G5a SCALE_MISMATCH.** They run in the G5a pass in case order, after the selected case's existing checks for earlier cases:

1. **The canonical layout and +0 prescriptions** of the attempt's source.
2. **Feasibility** with:
   - the source's present/non-input kinds;
   - the extent;
   - floor positivity **derived** from the Run's selected verification record. Φ>0 if and only if ê>0 at P=1024 (`FK/adaptive.rs:2294–2307`); unavailable cases have no Selection floor.
3. **The verification record relations:**
   - bound non-null if and only if has_data;
   - θ=+0 for no-data bodies;
   - `data_blocks` 0 if and only if no body has data, and otherwise at least the true count.

   The record is the selected Run's last verification record. The Run is selected because G5 requires it for non-null coverage.
4. **The direct data facts:** no free DOF implies F; an individually nonzero free-DOF term implies T.

**Not applied:**
- the Selection lists for stop, estimate, charge and B (no Selection exists);
- the numerical classification;
- any check that presupposes a passed certificate or G5a.

**Why these and only these.** Native coverage is computed from the owner's layout, evidence and data blocks before any certificate verdict (`final_case.rs:1371–1448`, assigned at 1195). So every complete native vector satisfies items 1–4 whatever the later failure. I57 §4 keeps "structural, ownership, stage and direct source-consistency checks" for unavailable attempts and forbids applying selected pass conditions.

**Tests:** these need §1.5's bases, and are built with them in C1.

## 5. Build order if time runs short (total about 14–17 h)

1. §3 promotions: 20 min.
2. §1.6 cancelled ±x: 40 min.
3. §1.5 template plus `fail_after_cert_adapter_copy`, `fail_cert_after_summary` and `fail_lane_k`/`fail_lane_source`, with §4's Python checks: about 3 h.
4. §2 prefix bases: `captured`, `pre_helper`, `helper_refused`, `old_err_new_ready`, `maxima_abandon`, `post_native`: about 3 h.
5. §1.1 two bodies: 2.5 h.
6. §1.4 two cases: 2.5 h.
7. §1.3 p512 ladder: 3 h, after 5.
8. §1.2 L=0: 1.5 h, only if node admission is confirmed.

**Suggested split:** C1a = items 1–4, which close every coverage-null/complete row and the prefix audit. C1b = items 5–8. Each needs a fresh snapshot version (05a/05b), so I63/I64 can test incrementally.

## 6. Rulings needed from ROOT

- **(a) Two schedule/floor checks that no reader-gate assignment yet covers:**
  - the native p128 fresh-first schedule start, which Rust already enforces;
  - Φ = `phi_512(ê)` floor equality, which today's readers consume as attested (I58).

  Proposal: Φ equality at G5a SCALE, as I57's "actual native p/P/floor rules", and the schedule start at G5 ATTEMPT. All three readers adopt both before the p512 base.
- **(b) No ruling is needed for the p512 charge control.** A positive floor forces stop on body 0, so the discriminating control is on the zero-floor body 1 (§1.3 `p512_charge_follows_estimate`). The lower-p side is already pinned by snapshot 04's `charge_follows_stop_below_p512`.
- **(c) Synthetic accounting-fault triggers** (`fail_cert_before_summary`, `prefix_unequal_helper_new`): build them labelled as synthetic triggers, or defer.
- **(d) Node admission for L=0:** confirm in C1, or defer.
- **(e) The undetectable variants** (consistent stop swap, consistent copied flags in the detectable-proof-only direction, null on a post-summary certificate failure) become Python-only must-pass tests. I propose the same for the Rust/TypeScript suites, so no reader over-rejects.

## Sources read for this plan (sha256 prefix, blob at 652ad0cc1f)

| File | sha256 prefix | Blob |
|---|---|---|
| FK/adaptive.rs | 6a2fc382bf8cae05 | 347385684092 |
| FK/verify.rs | 66022cc78bb5779d | bcf6df25bf29 |
| FK/source.rs | 9956c08421eebb44 | 47e1df6011b5 |
| FK/recover.rs | e452e3467608a468 | 9f5ae091231e |
| FK/bound.rs | bdffbeb81c54ed99 | ea65ed7b198b |
| FK/ledger.rs | 20c3b86a834c0657 | b1d853dd1835 |
| FK/product_certificate/final_case.rs | 3bc84bf1138b227f | da17af52899e |
| PP/retained_product.rs | d07383fc026e61e4 | bba370af01f9 |
| PP/retained_receipt.rs | 84810876aaae59e1 | 61eecb63572a |
| PP/retained_product_tests.rs | 9bb2ac0319f339d8 | 6040b0bd87d1 |
| PP/lib.rs (disconnected-body test only) | 4fff1a331c754f66 | 1467a2fc89c7 |

Records read:
- ROOT_RULINGS_V1 at 027c0912e7 (the checkpoint-B ruling);
- R/I59/rust_reader_01/RETURN.md.

## Freeze

Frozen at 2026-10-03T20:15:56Z. Bulk files: WT/scratch/i62_coverage_shared_python_01/c0_probe_promotions.py (sha256 6f7d39521237046d0cb4c3b2d3df28405154b418851af1d191693a99fa2c1bc7, 1511 B) and c0_probe_promotions.log (sha256 9b90f463b06ce8ce442270db41140bc5878f37fc1e05d2cc074fbc151fdd8def, 474 B).
