# T3 D1 — numerical core: general accuracy, range and sparse scale

HELPS_HUMANS-style design record (TASK D1) for the T3 WORKING_ITEMS manager and ROOT, **revision 5**, 2026-09-26. It stays a proposal until ROOT selects it after V1's narrow re-pass.

- **Basis.** Revision 1 was written at `e14f7fd13`; revision 2 was started at `12f2122cd` and completed at `4862a72a9`; revision 3 at `7a6e6f00b`; revision 4 at `ef9cf487e`; revision 5 at `c6a96f8a9` (ROOT's BACKCHECK_R4 rulings) with D2 revision 5 at `e757f544f`. All sit on T3 branch `codex/piping-numerical-integrity-20260926`.
- **Product basis and line numbers (revision 5).** The product basis is the merged tree `303609725` (origin/main with T1's `5aa4285c2`, merged into the T3 branch).
  - Nothing under `P/core/solver/**`, `P/core/loads/**`, `P/core/rules/**` or `P/core/result_export/**` changed between `c61a540ea` and `303609725` (checked with `git diff --stat`). So every `FK`, `SA`, `SP`, `CB`, sparse-direct, load-algebra and rules citation is valid at both. The headless runner (`P/core/runner/headless/src/lib.rs`) gained two lines near its top, so its entry is cited at the merged tree (`run_preview_in_memory_mode` at `:804`).
  - `PP` and `source_receipt.rs` citations in the active text are remapped to `303609725` through the diff hunks (`linemap.py`, a scratch helper; each remapped line was spot-checked). T1's own line numbers at `f3270ea79` are unchanged at `303609725`.
  - The "Read for revision N" lists in §10 stay at the basis they record.
- **Revision 1** is archived unchanged as `_run_records/DESIGN_revision1.md` (sha256 `7199390f…`). That is the text V1 reviewed.
- **Revision 2** (committed at `63c3d503c`) is archived unchanged as `_run_records/DESIGN_revision2.md` (sha256 `3ff9c1fd…`). That is the text V1's BACKCHECK_R2 reviewed.
- **Revision 3** (committed at `9377f32db`) is archived unchanged as `_run_records/DESIGN_revision3.md` (sha256 `48f35144…`). That is the text V1's BACKCHECK_R3 reviewed.
- **Revision 4** (committed at `e94af71f2`) is archived unchanged as `_run_records/DESIGN_revision4.md` (sha256 `7ca6fb9e…`). That is the text V1's BACKCHECK_R4 reviewed.
- **Companion addendum.** `D5_TRIGGER.md` **revision 2** (revision 1, sha256 `0d14db3b…`, archived as `_run_records/D5_TRIGGER_revision1.md`). The trigger itself is now specified here, in §4.3.1. D5_TRIGGER keeps the option analysis and its history, and it marks which of its sections §4.3.1 supersedes.
- **Companion note.** `S11_CONTAINMENT.md` **revision 5** (delivered with this revision; revision 4, sha256 `8f5d5df8…`, archived as `_run_records/S11_CONTAINMENT_revision4.md`) governs S11. Revision 5 adds ROOT's S-H/S11-F ordering, S11-F's two-entry tests and the named exceptions, and it remaps its citations.
- **T1.** Merged (`5aa4285c2`). It changed no file under `P/core/solver/**` or `P/validation/benchmarks/**`.
- **Paths.** `P/` means `projects/chirality-piping/`. `PP` means `P/core/product_physics/src/lib.rs`. `FK` means `P/core/solver/frame_kernel/src/`. `SA` means `P/core/solver/nonlinear_integration/src/structural_adapter.rs`. `T3/` is this tranche's records folder.
- **Roles read.** Root `AGENTS.md` (sha256 `c8ce87ef…`), `agents/AGENT_TASK.md` (`1a13a5b0…`). I also consulted `agents/AGENT_HELPS_HUMANS.md` (`a0c9fb94…`) deliberately, for the design posture, as the brief asked.
- **Other agents' records read for revision 5.** V1's `REVIEW/D5_CHECK.md` and `REVIEW/BACKCHECK_R4.md`, and V1's probe `REVIEW/_run_records/d5_check/probe_d5_check.py.txt` (sha256 `d13cf7c8…`). That probe is imported, not copied, by `recal_d5.py` (§4.3.1).
- **What I did not do.** I changed no product source, test, fixture, reference or other record, and ran no Git write. I ran no cargo build, test or Rust probe; the host is held. I ran standard-library Python probes at low priority, and read public crates.io metadata (§10).

## Revision 5 — what changed and why

**Inputs:**
- `T3/REVIEW/D5_CHECK.md` (V1, BLOCKING on D5C-1, at `9f792d741`);
- `T3/REVIEW/BACKCHECK_R4.md` (V1, BLOCKING only through D5C-1, at `e27181fb2`);
- ROOT's rulings through `c6a96f8a9`: D-15 and option C (`de60993b5`), the S-H/S11-F ordering (`b6fe1eb75`), D5_CHECK (`5c359dea4`) and BACKCHECK_R4;
- D2 revision 5 (`e757f544f`), and D2's reply on C and B.

| Finding | Change | Where |
|---|---|---|
| **D5C-1** (BLOCKING; ROOT adopted V1's fix) | The trigger uses the exact residual of the intended system: EF = K̃⁻¹ρ, ρ = f − K_int·u, as one `ExactAccumulator` sum per free row, rounded once. K_int is every frame element re-formed in `Wide<2>` from its primitives (local coefficients included), plus springs and the prescribed coupling. f is the ledger terms. The published binary64 residual and element-level ΔK are not used. The FK intended-action audit path is extended. Three controls and three mutations are added | §4.3.1, §6 (K-D5), §7.3 (26–28) |
| **D5C-4** | Recalibrated on the product-faithful path, importing V1's probe (`recal_d5.py`, 160 R1 cases, both modes). EF matches the actual error to within ±1.1e-5 relative on every row with an actual ratio of 0.1 to 1e3. The factor is **2**, on the coupled body scale S\*: no miss outside S11's named class; **0 false positives** in the Passed band (at 8, three false positives, with actual 0.20–0.38). (a1) gives 16 (c = 1) and 28 (c = 8) false positives | §4.3.1 |
| **Order** (ROOT, D5_CHECK 3) | EF needs `Wide`, because the re-formation has divisions and square roots and must be accurate far below 2^-53. So the order is **S11-K → K3a → K-D5 → K2a → K1 → K2b → K5**. K3a is the part of K3 that K-D5 needs: `Wide<2>`, correctly rounded + − × ÷ √, exact lift from f64 and exact split into binary64 terms, the work counter, and their vectors and differential. The rest of K3 continues in parallel | §6 |
| **D5C-2** (ROOT: demote, never silent) | A case with any stiffness contribution EF cannot re-form (curved bend, user stiffness element, expansion-joint stiffness) is **not eligible for Passed**. K-D5 demotes it to Sensitive with the reason `formation_check_unavailable`. **Over-demotion on committed fixtures: none.** No committed request that solves on the merged tree realizes a curved bend or a user-stiffness element: bends are `mechanics_geometry_only`, and the ordinary route already refuses a realized user-stiffness joint (`JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED`). Two product tests that build DEC-070 curved-arc models, and possibly the `mechanics` and `nonlinear` benchmark crates, change quality. They are declared expected changes | §4.3.1, §6 |
| **D5C-3** | EF is not a field of `StructuralReport`, which is `Debug`-rendered into committed raws (`PP:1057`). It is a separate `FormationCheck` record on `StructuralSolution`, present only when the check demotes. A byte-identity test covers every committed raw. F1 renders its evidence line | §4.3.1, §6 |
| **D5C-5** | The "no Passed breach" gate lists its S11 exceptions as (entry, case, quantity) triples. The predicted list (`s11_exceptions.py`, from R1's frozen controls) has **106 triples in 13 cases on the captured entry**, and 168 triples in 22 cases on the typed entry (adding the nine G = 1e80 cases the captured entry refuses at capture). ROOT's "12" leaves out RF-CANCEL-UDL-W1e8, which P1 reports as a Passed breach (ratio 46.5). The committed list is P1's record. A test requires it to be empty once S11-F merges | §4.10 |
| **N-1, N-2** | Member actions, reactions and load formation are outside EF, and that is stated. Loads route to S11 (after S11-F, ρ uses exact ledger terms and the load-fold class vanishes). Actions and reactions route to W1's recovery before rounding, after F2a. P1's survey is asked for member and reaction comparisons | §4.3.1 |
| **N-3, N-4** | With the product estimator (V1), M11's cond is 2.6e6 and M9's 3.2e5. (a1) at c = 1 does not demote M11; at c = 8 it does. The textbook-LDLᵀ 122 figures are not the product's. P1's y_ref is recorded as (1,0,0) or (0,1,−1) | §4.3.1, `D5_TRIGGER.md` |
| **ROOT D5_CHECK 2** | The no-interim ruling stands, with its correction recorded: the Passed-band breach class is broad in synthetic space (soft springs and solve error up to about 4e-9 relative, cond about 1e6 to 6.7e7), not a single case | §4.3.1 |
| **R4-1** | Span-statics rows use k = 2√2 (circular maximum) and k = 4 (open-formula summary). The proof now covers resultants rebuilt from one end's actions, which is exact through S11-K's E4/E6 sums. V1's membrane clarification is under the carve-out. Mutation 25 gains a span-statics row | §4.1.6, §4.1.6.1, §7.3 |
| **R4-2** (D-15) | Gate condition 3 is row-level: per-case withheld counts for both identities side by side, successor ≤ retiring. F2 splits into **F2a** (W1 wiring and identities, no retirement) and **F2b** (retirement domain by domain, only where the gate passes). **Coexistence rule:** no W1 attempt in any invocation where exact-block selects a case. "Almost all exact zeros" is corrected (signed-companion 46 of 57, eigen_motion 39 of 62). The proof-carrying B is specified, with its coverage | §4.4, §4.4.1, §6, §8.1 |
| **R4-3, C** (ROOT's conservative binding) | C is D2's (S-I1, S-I2), placed after F2a and S-G1 and before F2b and F3's retirement. On the committed families **C is the restorer in every domain**: each family has a case that B cannot fully cover. D1 supplies b and the receipt terms. Its cost is in §8.1 | §6, §8.1 |
| **R4-4** | The closed table is the single source. The intensified k_i rounds upward (D2's rule). All eleven mismatches are resolved in the table (units; the receipt field `input_derived_dofs`; echoed-input kinds; `constant_effort_support_applied_load`; the explicit non_quantity list; the member-entity rule for curved and uncovered rows; the pressure condition; k = 1 for magnitudes; the receipt field set; N-2). §5 item 1 is fixed: twist and extension scales are harness-only, formed from the receipt's k_t and k_a | §4.1.6.1, §5 |
| **ROOT on I1's S11-K stop** (`5a9212de6`) | KS1–KS3 are live on main in the nonlinear active-set loop, so S11 §2.4, §4.6 and §8.1 are corrected. Option (c): the loop keeps a named binary64 legacy variant, pinned by a test, and DEC-046 is untouched; recorded as T5 open work with the friction fold. B1 and B2 join §8.3's expected diffs. Lesson applied here too: K-D5 enumerates every caller of the changed structural solve (§6) | S11 revision 5, §6 |
| **ROOT S-H/S11-F** (`b6fe1eb75`) | S-H never lands before S11-F. S11-F's tests and the "no Passed breach" gate run through both entries. The ledger sits in the shared `solve_load_case` (`PP:2137`, force built at `:2248`) for both | §6, §4.10, S11 §8.2 |

## Revision 4 — what changed and why

**Inputs:** `T3/REVIEW/BACKCHECK_R3.md` (V1, FINDINGS, nothing blocking, at `ef9cf487e`, sha256 `47700af5…`) and ROOT's rulings on it; ROOT's rulings on P1's skew breach (`d84e66bff`) and its pre-acceptance of D-5 option O1; `D5_TRIGGER.md`; D2 revision 4 (§0.2, §4.9.10, marked [align D1-r4]); P1's relayed realistic-fixture survey.

| Finding | Change | Where |
|---|---|---|
| **D-5** (ROOT: required; O1 pre-accepted) | The Passed-case trigger is `D5_TRIGGER.md` option (a2), adopted by reference: K-D5 lands early after K3 as a kernel-local Passed→Sensitive demotion and routes to W1 after F2. Thresholds come from product data; P1's measured 122 case is a required true positive. The "no Passed breach" gate is added, with RF-CANCEL's 12 breaches as named S11 exceptions until S11-F. P1's relayed fe-trigger run (54 false positives on passing R1 runs, misses only S11) supports rejecting a pure condition or fe trigger | §4.3, §4.10, §6, §7.3 (23–25), §9 |
| **R3B-1** (ROOT: pinned factor, with proof) | Stress S\* carries a propagation factor k per row kind: 1, √2 (circular maximum), 2 (open-formula summary), √2·i (intensified, row's own i); magnitude rows are formed at p (k = 1). A proof that the floor holds at 1e-9 is given; where it cannot hold, the row is `not_covered` | §4.1.6, §4.1.6.1 item 7 |
| **R3B-2** (ROOT: closed list; counts; cost) | One closed (kind, unit) → class table, defaulting to `not_covered`, with the classes translation, rotation, force, moment, stress(k), input_derived and non_quantity. `reaction_resultant` → force and `open_formula_stress_summary` → stress (k = 2, only without a pressure-longitudinal term). "Physical" is removed. Per-case withheld counts go into the gate. The combined withholding figure and the owner framing are in the new §8.1 | §4.1.6.1 item 2, §4.4.1, §8.1 |
| R3B-3 | Section terms (A, Z, L, E·A/L, G·J/L) travel in the receipt as bit strings, cross-checked against published section evidence (`RETAINED_PRECISION_SECTION_MISMATCH`); twist and extension scales are harness-only | §4.1.6.1 item 7, §5 |
| R3B-4 | D2's side; this revision's table and formulas are what G5b and G5c mirror (D2 §4.9.10) | §4.1.6.1 |
| R3B-5 | S11 erratum: the site-test constant is keyed by function plus match count | `S11_CONTAINMENT.md` §4.3 |
| N-1 | Counts corrected: of 314 variant-A twist and extension flags, 303 clear (293 of the 295 outside RF-WEAK) | revision-3 table, §4.10 |
| N-2 | The threshold is exact for S\* ≥ 2^-988; smaller S\* classifies every row of that kind `absolute_verified` | §4.1.6 |
| N-3 | The harness correspondence check compares classes, not bits | §4.10 |
| N-4 | S11-K's PR record carries the fixture-diff sizes | `S11_CONTAINMENT.md` §8.3 |
| N-5 | D2's two refusal codes adopted | §4.1.6 item 4 |
| P1's survey (relayed) | Committed Passed cases that solve on main: rcond 1.06e-3 to 0.061, fe ≤ 2.85e-12; none demote. The demo-model figure is corrected: it comes from committed result envelopes, and `invented_preview_model.json` itself is blocked on main | §8.1, `D5_TRIGGER.md` §7 note |

## Revision 3 — what changed and why

**Inputs:**
- `T3/REVIEW/BACKCHECK_R2.md` (V1, FINDINGS, nothing blocking, at `3ea78add8`) and ROOT's rulings on it (`T3/ROOT_RULINGS_V1.md` at `0d1b18d97` and `af0bb908b`: S2-R final wording, SCALE-W in K2b);
- `T3/REFERENCE_CHECK/RETURN.md` (V2, at `1fd1cb398`) finding F2, and ROOT's rulings on it (`T3/ROOT_RULINGS_V2.md` §1 and §3, at `3592032fa`);
- ROOT's ruling on D2's probe finding F-P2 (`T3/ROOT_RULINGS_V1.md`, after `7a6e6f00b`; `T3/DESIGN_STANDING/DESIGN.md` §12.1);
- `S11_CONTAINMENT.md` revisions 3 and 4 (V1's `S11_BACKCHECK.md` and `S11_BACKCHECK_R3.md`, ROOT at `45cfc92b1`);
- D2 revision 3 at `7a6e6f00b` (§4.9.3 G5b and G5c, §4.9.9, §4.5.2), whose reader checks are the other side of the S8-R interface;
- R1's references revision 2 at `c0f14201c` (`references.json`, read by the floor probe only).

| Finding | Change | Where |
|---|---|---|
| **S8-R** (ROOT: the floor is enforced) | Classification is on the **published** binary64 value: `absolute_verified` iff `|q| < fl(R·S*)`. **R = 2^-34** exactly (bits `0x3DD0000000000000`), chosen with a 7 % margin over 10^9·2^-64 so the publication rounding and the published-versus-2p scale difference stay inside 1e-9; the threshold product is then exact. **S\* is defined on published rows** with a pinned kind mapping, unit factors, body membership, L_b formula and operation order, so D2's G5b and G5c can recompute it bit for bit. `absolute_verified` and not-covered quantities are **withheld from reliance** (no exemption proposed) and never counted as Passed, as D2 §4.9.9 enforces | §4.1.6, §5 |
| **F2-P** (ROOT, `ROOT_RULINGS_V2.md` §3) | **Twist and extension are scale kinds**, with per-member S\* = S\*(moment)·(L/GJ)_m and S\*(force)·(L/EA)_m. The harness **derives** them from the published torque and axial force (T/(GJ/L), N/(EA/L)), never from differences of rotations or translations, so they inherit the verified accuracy. **A below-floor or not-covered comparison never counts as a pass.** Probe `floor_kinds.py` on R1 revision 2: of V2's 314 variant-A twist and extension flags, 303 clear (293 of the 295 outside RF-WEAK; corrected in revision 4, N-1); 46 RF-WEAK and 3 RF-CANCEL comparisons stay not covered, plus 2 RF-SKEW twist rows whose own torque is below the moment floor. F8 (expected values below the binary64 range) is compared absolutely and reported as such. The RF-CANCEL net-governed column is the binding scale (ROOT, V2 ruling 1) | §4.1.6, §4.10, §7 |
| **S2-R** (ROOT's final wording) | Gate condition 3: "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages", with identical, fail-closed standing in each language. D2's H-a log-law cases are listed as `needs_recompute` under both identities. The switch happens at F3 | §4.4.1 |
| **SCALE-W** (ROOT: K2b) | The kernel half of formation-time scaling (`FK/lib.rs` scaled formation and the sparse assembly entry that takes b) joins K2b's write set. F1 wires it | §4.7, §6 |
| S11K-W, and S11 revisions 3 and 4 | S11-K's write set now carries ROOT's S11B-1 sites (`FK/structural.rs:603-606`, `FK/lib.rs:870-877`, and the refinement numerator at `:697-760`), the typed force seams and E15/E16 routing; T1's support-motion fixtures are pre-registered as expected diffs, because S11-K lands after T1 (S11 R3-1); the "share only `FK/lib.rs`" text is corrected | §6 |
| S5-R | W1 attempts require D2's S-H `digest_ok()`; otherwise `RETAINED_PRECISION_UNAVAILABLE` (`invocation_not_representable`), and the case publishes ordinarily | §4.3 |
| FORM-C, and S11B-9 | LEF-small corrected: 6EI/L², 4EI/L, 2EI/L and GJ/L are exactly 0, but 12EI/L³ is a normal value 3.5 % wrong, from a least-subnormal intermediate. K2a checks every intermediate product and quotient, including partial underflow | §4.7, §6 |
| MOD-D | K3 owns the `mod retained;` declaration in `FK/structural.rs`; K4 adds only files under `retained/` | §6 |
| DD-11 | Closed: IF-1 is adopted, so DD-11 does not arise | §5 |
| F-P2 (ROOT, D2's probe) | Open with F2, the general method that recovers rejected unrecoverable cases; D2's corrected probe PR-2b joins F2's tests; the whole-invocation-blocking half stays with T6 | §7, §8 |

## Revision 2 — what changed and why

**Inputs:**
- `T3/REVIEW/RETURN.md` (V1, verdict BLOCKING) and its run records;
- `T3/ROOT_RULINGS_V1.md`, including the further rulings and the S11 pre-acceptance;
- `T3/MANAGER_NOTES/{V1_DISPOSITIONS, S11_MAP}.md`;
- `T3/REVIEW/S11_CHECK.md` (V1's check of S11 revision 1, BLOCKING, at `56b651282`) and ROOT's rulings on it, with ROOT's adopted no-interim text (`T3/ROOT_RULINGS_V1.md` at `2d07cad7f`);
- R1's references at `6c448d260` (`T3/REFERENCES/README.md` only);
- D2 revision 2 at `d566713e9` (§4.9);
- the manager's instructions of 2026-09-26.

| Finding | Change | Where |
|---|---|---|
| **V1-B1** (BLOCKING; ROOT ruling 1) | **Every multi-term sum outside the factorization is now one exact expansion, rounded once.** That covers per-DOF load contributions (the load ledger), stiffness entries, the reduced right-hand side with `K_fc u_c`, reactions, basic-deformation recovery sums, and combinations over retained states. **Combination outputs are now under the stop rule.** New probe evidence: V1's check L is exact at 128 bits with the ledger, but accepted with error 0.5 when loads are folded at p. The absorbed-term combination A + B − A2 is exact when combined exactly, but accepted with error 1.0 when folded. A combination that needs escalation rejects 128 and accepts 256. V1's cancellation case and two combination cases are added as negative controls (§7.3, 13–16). **The §3.1 claim "never accepted a failing candidate" is withdrawn** and restated as limited to precision-dependent error | §3.1, §3.2, §4.1.1, §4.1.2, §4.1.4-§4.1.6, §4.1.9, §7.3 |
| **V1-S11** (ROOT ruling 2), then **S11-V1 to S11-V7** and ROOT's seven additions | C3-full, redefined in `S11_CONTAINMENT.md` revision 2 as an exact per-case load ledger **plus exact recovery sums** (14 recovery-side load sums enumerated, S11 §2.2). The force vector is a type only the ledger can build, with an enumerated site test. Contribution granularity is defined per producer. One correctly rounded accumulator (`FK/exact_sum.rs`, from `pressure_sum::exact_sum`) replaces `Expansion::rounded()` and every fold. T1's three sites are named. The invariant "never newly silent" is stated and tested on probe A. S11-K is T1-disjoint and partly live (`SP`, `load_case_algebra`, the `FK` rounding sites); S11-F follows T1. A detected loss makes the case Sensitive with `LOAD_CONTRIBUTION_ABSORBED`. ROOT's pre-acceptance is suspended until V1's backcheck; ROOT's decisions on D-S11-1 to D-S11-4 are in the row "ROOT's S11 decisions" below | `S11_CONTAINMENT.md`; §4.1.2, §4.3, §6, §7, §9 |
| New finding N-S11-R | `Expansion::rounded()` (naive, not correctly rounded) feeds the published, byte-compared M03 intended-action `ResidualRow`; exact-zero residuals publish −0.0 (245 in committed fixtures) because the standard library's float `Sum` starts from −0.0. S11 proposes a zero witness so committed bytes stay unchanged (D-S11-1) | `S11_CONTAINMENT.md` §4.1.3, §7 |
| **V1-S8** (ROOT ruling 1) | 1e-9 cannot be proved for every published quantity: exact zeros and quantities far below the body scale have no relative guarantee. **The floor is stated and enforced.** Relative 1e-9 holds for \|q\| ≥ R·S\*, with R = 2^-64/1e-9 ≈ 5.42e-11. Below that the guarantee is absolute (2^-64·S\*). The receipt classifies every published quantity. Readers never present a below-floor quantity as relative-verified. VP-ROBUST requires every reference zero scale to be at least R·S\*. A weak-coupling control is added: its far-node quantities sit at 2e-21 to 6e-16 of S\*, yet are exact at 128 bits in the probe. Withholding below-floor quantities is offered to ROOT as an alternative, and not recommended because it would withhold structural zeros | §4.1.6, §4.10, §5, §9 D-12 |
| **V1-S2** (ruling 3, adopted) | **A shared retirement gate**, defined here and cited by D2 (S-F, S-G). Four conditions: coverage, budgets, three-language standing, and value agreement with the exact-block oracle. It is applied per identity family at F2 and at F3 | §4.4.1 |
| **V1-S1** (R-3(a), R-3(b), R-7 ruled) | Stated as ruled. `load-reference-source-1` stays fresh until F3 lands and retires at F3 under the gate. S-E1 is built with F3. Historical physics-source-1 stays eligible. Historical all-selected source-blocks-1 stays Current (R-7 (i)) | §4.4 |
| **V1-S3** | D2 owns the successor-identity readers (S-G, D2 §4.9). The receipt (§5) is their interface. **IF-1 is adopted:** `numerical_quality.cases[i]` keeps the ordinary attempt's outcome; the precision-p outcome lives only in the receipt (DD-11 does not arise) | §4.5, §5 |
| **V1-S4**, aligned with D2 (G1, G2, G5, IF-1, DD-11) | The receipt uses the checked profile `openpipestress_jcs_ijson_v1`. Every value that can exceed 2^53 − 1, be subnormal or be negative zero is a 16-hex binary64 bit string. Plain numbers are only exact integers within range. **Per-case hashing failure:** that case becomes `unavailable` (`receipt_encoding`) with ordinary standing. A publication-hash failure republishes the invocation under its base identity, with every attempt declined, as T1's SF-1 does. Never an `Err` | §5 |
| **V1-S5** (ruling 4) | Capture refuses \|x\| ≥ 2^53 on every route. D2 designs the capture fix (R-6). D1 adds capture-boundary cases to RF-RANGE and VP-ROBUST, and states W1 and W2's handling once capture admits such values | §4.7, §4.10 |
| **V1-S7** | `force_scale_exponent` stays out of `StructuralReport`, so the `Debug`-published report is byte-identical when b = 0. When b ≠ 0 it is published in its own evidence line. The b-selection rule is normative, including the subnormal cases. W2 publication applies the §5 representability outcomes | §4.7, §5 |
| **V1-S9** | Targeted hard-case classes for the arithmetic, a large seeded `Fraction` differential, and seeded rounding mutants | §4.11 |
| V1-N1 | §3.1 column (C) is relabelled "assembled binary64 matrix, solved exactly". V1's contribution-exact results are added beside it | §3.1 |
| V1-N2 | A negative-energy witness against represented binary64 entries now triggers the method for supported families. The method's check at p against the primitive model decides the outcome | §4.3 |
| V1-N4 and the manager | K5 is serialized after K2b. S11-K, K1, K2b and K5 all write `SA` and are serialized in that order | §6 |
| ROOT's S11 decisions (after `4663cdbb6`, recorded at `4862a72a9`) | D-S11-1: the zero witness is adopted **only for published diagnostic renderings** (the intended-action `ResidualRow` at `FK/structural.rs:554`); the ledger, the force vector, every recovery sum and every value that feeds a bit-equality check or a receipt use +0.0. D-S11-2: `Expansion::rounded()` replaced at all three `FK` sites, under the fixture stop rule. D-S11-3: S11-K is a full product slice with full gates, as its own PR to main. D-S11-4: no in-band marker. **Formation silent zero:** its reach is stated (only LEF-small among R1's RF-RANGE vectors; no realistic case), and checked formation is split out as **K2a, next after S11-K**, as its own slice | §4.1.2, §4.7, §6, §9 |
| V1-N5 | Components and releases go to T4 and T7 (proposed). Equivalent-static has no owning tranche in the graph, so it is recorded as open for ROOT | §4.2 |
| V1-N6 | `RLIMIT_AS` is enforced on Linux only. On macOS the runner uses an RSS watchdog | §4.8 |
| V1-N3 | No change; P1 checks the Passed boundary | — |
| R1's findings (`T3/REFERENCES/README.md` at `6c448d260`, relayed by the manager) | Finding 1: `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6` are compared on the represented basis. Finding 2: the k/a ≈ 1e-4 cases are continuity controls, and discrimination is claimed only from k/a ≤ 1e-8. Finding 3: LEF-small and LEF-large are solved by checked formation plus scaling at formation, never a silent zero coefficient. Finding 4: directional-spring cases run in the kernel lane only. Finding 5: RF-CANCEL is compared on R1's net-governed scale, and its relation to the S11 row scale is in `S11_CONTAINMENT.md` §5.2 | §4.7, §4.10, §7.1, §7.2 |

## 1. Recommendation in brief

**W1, general accuracy.** Implement the selected contribution-preserving multiprecision method (`CORRECTNESS_DESIGN/CONTRIBUTION_PRECISION/CONTRACT.md`) as the one method for N05-class accuracy and for general retained-source recovery. Add five concrete choices to it:

1. **Formation.** Rebuild every element at precision p from its binary64 primitives, in basic-deformation form, `K_e = Bᵀ D B`. The probe (§3) shows why: the rounded binary64 element matrix breaks the element's rigid-body null space, even for an axis-aligned member.
2. **Solver.** Use a sparse profile LDLᵀ at precision p, on the same sparse structure W3 introduces.
3. **Exact sums (revision 2, V1-B1).** Every multi-term sum outside the factorization is one exact expansion, rounded once. That covers loads per DOF, stiffness entries, the reduced right-hand side, reactions, recovery sums and combinations over retained states. This removes the common-mode loss V1 found: a contribution lost identically at p and at 2p.
4. **Stop rule.** A candidate at p is accepted only when a fresh solve at 2p agrees with it on every published quantity, combination outputs included. The screen is `2^-64` of the connected body's scale for that kind of quantity (§4.1.6). Relative 1e-9 is guaranteed only above the floor R·S\*, with R = 2^-34 ≈ 5.82e-11 (revision 3). Below it the guarantee is absolute; the receipt classifies every published quantity on its published value, readers recompute the classification, and a below-floor quantity is withheld from reliance and never counted as Passed (V1-S8, S8-R). ROOT registers the constant as method policy; it is the acceptance threshold for a selected case, not a comparison tolerance.
5. **Arithmetic.** Write a small, dependency-free fixed-limb binary float type in `frame_kernel`. No lockfile changes.

Two alternatives are rejected. Generalizing the merged exact-block method is exact on the wrong system: the probe finds scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion, and finds a matrix that is not positive definite for N06-class skewed cases. A global relative-coordinate reformulation is deferred.

**Coverage.** Three phases.
- **W1a:** straight frames, global-axis linear springs, rigid restraints, nodal loads, and combinations through T0R's gates.
- **W1b:** element uniform loads, thermal eigen axial loads, pressure thrust on straight members, constant effort, prescribed support motion (0.4.0, after T1) and user matrices.
- **W1c:** curved bends, with T4.

Nonlinear and contact recovery stays with T5. Every excluded family is refused per case with a named reason; the case keeps its ordinary standing.

**Triggers and identities.**
- The method runs automatically, per case, on the ordinary route and on the exact route, whenever the ordinary M03 outcome is not `Passed`. Witnessed mechanisms, asymmetry and invalid input never trigger it. A negative-energy witness found only against the represented binary64 entries does trigger it for supported families (V1-N2).
- It publishes under proposed successor identities that allow mixed envelopes (placeholders for ROOT, §4.4). No invocation-level failure path exists for them.
- I recommend retiring exact-block selection for fresh solves. The exact-block method stays as an independent oracle in tests.
- **Retirement gate** (§4.4.1). Retirement happens only under one shared gate with four conditions: coverage, budgets, three-language standing (row-level since revision 5: side-by-side per-case withheld counts), and value agreement. It applies at F2b, domain by domain, for source-blocks-1 and physics-source-1, and at F3's retirement step for `load-reference-source-1`, as ROOT ruled in R-3(a) and D-15. Until a family retires, exact-block stays selected and W1 is not attempted in its invocations (coexistence rule, §4.4). On the committed families, D2's C precedes every retirement (§8.1).
- **D-5 (revision 5).** Passed cases pass the exact-residual formation check (§4.3.1), which demotes them in K-D5 before W1 and routes them to W1 after F2a.

**S11, cancelled load contributions (a live silent-wrong path on main).** C3-full; the design is `S11_CONTAINMENT.md` revision 2.
- **The rule.** Every force contribution goes, term by term, into one exact per-case ledger, rounded once, and the force vector can be built only from it. Every recovery-side load sum (end forces, stations, extrema, curved sections, reactions, combinations) is one exact sum of its individual terms, rounded once through the same accumulator. Retained source and both receipt replays use the same ledger.
- **S11-K** is the first T3 slice, a full-gate product PR that ROOT wants landed soon after T1 merges. It is T1-disjoint: the accumulator and the ledger types (now in `FK`), typed force seams beside the old ones, the kernel audit entry point, the exact prescribed-motion right-hand side (S11B-1), and live exact recovery in `SP`, exact combinations in `load_case_algebra`, and the `FK` rounding sites.
- **S11-F** is the first facade slice after T1 merges: the ledger at every producer, the switch to the typed seams, the `PP` recovery composition (E1–E16) and T1's three sites.
- **The guard.** The audit marks any remaining loss Sensitive, with `LOAD_CONTRIBUTION_ABSORBED`. It is defence in depth, with a stated floor.
- ROOT's pre-acceptance is suspended until V1 backchecks revision 2. ROOT has decided its four sub-decisions (`4862a72a9`): the zero witness only for published diagnostic renderings (§4.1.2), all three `FK` rounding sites, S11-K as a full-gate product slice, and no in-band marker. K2a (checked formation) follows S11-K as the next early slice. ROOT's adopted no-interim text governs: no interim containment before T1 merges.

**W2, range.** Scale the ordinary structural system exactly, by a power of two in force units, inside the kernel. The scale exponent is 0 whenever today's unscaled evaluation succeeds, so every current result stays bit-identical. The PHYS-R4 refusal is removed. The admitted range and its refusal reason are defined in §4.7. The W1 path has an internal 64-bit exponent.

**W3, sparse scale (M32).**
- One sparse pattern holds assembly, the M03 gate, reduction, reactions and the nonlinear linearized solves.
- Dense scrutiny stays as an explicitly selected mode, built from the same sparse values, with a resource guard.
- There is no automatic dense fallback. Main has none today either.
- Parity and memory protocols are in §4.8.

**W4, M03 residual items.**
- A constrained-body null-space witness covers bodies that contain user matrices, and curved elements that pass an objectivity screen.
- The SUP-17 text is reworded.

**W5, VP-ROBUST.** A new validation crate, `P/validation/benchmarks/numerical_robustness/`, with two lanes:
- a kernel lane, which can run before T1 merges;
- a product lane, through the public entry in both modes.

Seeded faults sit behind a kernel feature that product builds never enable. A scale runner measures peak memory in fresh processes.

**No owner decision is needed** if ROOT selects this. If ROOT prefers `rug` (GMP/MPFR), linking LGPL-3.0+ code into the MIT-licensed signed macOS app becomes a governance question (§9).

## 2. What main does today

### 2.1 The ordinary solve is dense from end to end

| Stage | Site | Shape |
|---|---|---|
| Global assembly | `FK/lib.rs:768` (`vec![vec![0.0; total_dofs]; total_dofs]`); called at `PP:1620`, and again per modulus basis at `PP:1751` | dense n×n |
| Evidence | `SA:35` `absolute_roundoff` and `operation_counts`, plus `n×n` temporaries (`SA:117-118`) | dense n×n ×2 |
| Reduction | `FK/lib.rs:867`; called at `PP:2330-2342` | dense n_f×n_f |
| M03 gate input | `structural.rs:26` `stiffness: &[Vec<f64>]`; `validate` audits all n² pairs (`:243-263`); `prepare_structural` builds dense `a` (`:597`) | dense |
| Contribution audit | `contribution_sums` allocates `n×n` `Expansion`s (`:402`), and `contribution_differences` clones them (`:423`). `audit_intended_action` allocates them again (`:517`) | 32 bytes per entry, twice |
| "Sparse" factor | `sparse_direct/src/structural.rs:10,14` builds adjacency and profile from the dense prepared matrix | dense input |
| Residual | `evaluate_original_residual` walks full dense rows (`:717`) | O(n²) per evaluation |
| Negative witness | `negative_pair_witness` tries every pair (`:1335-1336`), each with an O(n²) check (`:1299-1300`) | O(n⁴) on the failure path |
| Reactions | `multiply_matrix_vector(stiffness, …)` (`PP:2697`) | dense |

Reading the code, a peak of at least about 100 bytes per n² entry follows: dense K, reduced K, two evidence arrays, the prepared matrix, and two Expansion arrays. This is an estimate from the source, not a measurement. For a straight chain of m members (n = 6(m+1)):

| Members | n | Dense-path lower bound |
|---:|---:|---:|
| 10 | 66 | 0.45 MB |
| 100 | 606 | 38 MB |
| 1,000 | 6,006 | 3.8 GB |
| 10,000 | 60,006 | 374 GB |

RF-LARGE at 1,000 and 10,000 members cannot run today in either mode. The DEC-053 observation set stops at a 48-member chain and a 7×8 grid (`P/validation/benchmarks/sparse_default_promotion_observation.dec053.json`).

**Fallback.** No dense fallback runs today. `dense_fallback_message` is only ever `None` (`PP:4004`). Mode code 3 is therefore never emitted, and `LinearSolveMode::solution_basis(true)` is never called (`nonlinear_integration/src/lib.rs:52-58`, called only with `false` at `:2085` and `:2148`).

### 2.2 Accuracy

- The ordinary gate is binary64 throughout: `PreparedSystem.matrix: Vec<Vec<f64>>`, the Cholesky and profile factors, and `StructuralSolution.displacements: Vec<f64>`.
- An absorbed positive diagonal contribution is rejected as unresolved (`structural.rs:410-414`).
- The condition boundary is `rcond ≤ EPSILON`, which is unresolved (`:868`); `rcond < √EPSILON` makes the result Sensitive (`:934`).
- Retained-source recovery runs only when the ordinary result is Sensitive or rejected, a capture exists, there are no nonlinear supports and no combinations (`PP:2382-2391`).

### 2.3 The exact-block method's scope

- Every free connected block has order ≤ 2 (`FK/structural/exact_boundary.rs:419-420`).
- Systems have at most 256 DOFs (`:292`).
- The arithmetic is exact expansions on the represented binary64 contributions.
- Transforms must be signed permutations (`source_recovery.rs:349`).
- Nodal loads only (`:470`), straight frames only (`:435`).

Contributions are rounded global element entries. For axis-aligned members they equal the local entries. The local bending coefficients `12EI/L³`, `6EI/L²`, `4EI/L` and `2EI/L` are each rounded independently (`FK/lib.rs:719-726`). A bending soft mode needs at least three coupled DOFs, so I believe the order ≤ 2 limit is what keeps these out of scope today. V1 should check that reasoning.

### 2.4 Range (PHYS-R4)

`structural::transform_roundoff` (`structural.rs:1248-1283`) rejects any subnormal result through `checked_value`. For the PHYS-R4 inputs (E = 1 Pa, OD 4e-77 m, wall 1e-77 m, L = 1 m), every stiffness entry is normal, but the allowance at `[UY,UY]` is 7.53e-321 (`ENGINE_INTEGRATION/RETURN.md`). By my hand arithmetic: EA/L ≈ 9.4e-154, 12EI/L³ ≈ 1.4e-306 and GJ/L ≈ 1.1e-307 N·m. The allowance is about γ(24) ≈ 2.7e-15 times the latter entries. The refusal is correct inside a normal-range error model. Storing the allowance as a subnormal would lose bits and could round it down.

### 2.5 M03 residual items

- **Rigid-null witness.** `AssemblyEvidence::geometry` skips any body that contains a user or curved edge (`SA:208-217`). Its basis text says so (`SA:264`).
- **User elements.** Their energy is `Σ k_d (Δ_d)²` over six relative local DOFs, and all four stiffnesses must be positive (`FK/lib.rs:635-638`, `1021-1043`). The zero-energy set is therefore "both nodes share translation and rotation". That is not the rigid-motion set when the element has length.
- **Curved elements.** They are built from an inverted tip flexibility with a rigid chord transfer (`curved_bend/src/lib.rs:241-251`). That construction has the rigid-motion null space in exact arithmetic. The represented matrix is not screened for it.
- **SUP-17.** `PP:1604` names "missing global rigid-body DOF classes". This precheck counts directly restrained DOF classes (`PP:11433-11455`). One test pins the old text (`PP:20615`).

### 2.6 Dependencies and CI

- `frame_kernel` has no dependencies at all. Its lockfile has one package. `sparse_direct` has two and `nonlinear_integration` eight, all of them path crates.
- 23 product lockfiles contain `open_pipe_stress_frame_kernel`. The list is in §4.11.
- CI's numerical job (`.github/workflows/piping-desktop-e2e.yml:172-204`, `P/tools/ci/numerical_ci.py:27-47`) runs `cargo fetch --locked` for every manifest found under `core/` and `validation/benchmarks/`. It then runs `cargo test --offline --locked`. A new crate under `validation/benchmarks/` is picked up automatically and needs its own `Cargo.lock`.
- Releases target macOS arm64 (`desktop-release-template.yml:30,297`). The project licence is MIT (`docs/CONTRACT.md` OPS-K-GOV-1).

## 3. Options for the general accuracy method (W1)

| | (A) Contribution-preserving multiprecision (selected basis) | (B) Global relative-coordinate or basic-deformation reformulation | (C) Generalized exact-block |
|---|---|---|---|
| Model solved | The intended binary64-input model, re-formed at p | The intended model, in transformed coordinates | The represented binary64 contributions, exactly |
| Skewed members | Yes | Yes | No: exact on a model whose rigid null space is broken by O(u·a) |
| Order > 2, weak coupling | Yes, with a sparse factor | Needs cycle compatibility, supports and multi-point constraints in the basis | Exact elimination on expansions; expansion growth and a 256-DOF, 256-term budget |
| Scale | Sparse profile at p. Scalars cost more, but the method runs only on triggered cases | Transformation fill and conditioning unknown | Not feasible beyond small blocks |
| Range | Internal 64-bit exponent | binary64 | Checked binary64 range |
| Reuse | `Expansion` audit, M03 structure, sparse ordering | Little | All of `exact_boundary` |
| Evidence | Refutation and prototype (2-coordinate); this probe (3D, §3.1) | Scalar oracle only | This probe refutes it |

### 3.1 Probe evidence (standard-library Python, `_run_records/probe_skew_precision.py`)

**Cases.** One or six members of the N-series section, so every coordinate is an exact integer and every frame axis is an exact rational. The root node has its translations fixed and its rotations held only by three global rotational springs k. A moment acts at the tip. The reference is exact rational arithmetic from the binary64-decoded section and load inputs.

**Error measure.** Each cell is the worst `|obs − exp| / max(|exp|, S)` at binary64 publication. S is the body-level coupled scale of §4.1.6, used here as a zero scale for convenience only; R1 owns the real zero scales. "Fail" means above 1e-9.

**Column (C), relabelled in revision 2 (V1-N1).** My probe's column solves the *assembled* binary64 matrix exactly, after a + k has already been rounded in binary64. The merged exact-block method instead sums the represented contributions exactly. V1 reran the probe with that method (`REVIEW/_run_records/v1_stop_rule_probe.*`, check C); its results are in column (C′).

| Case | (C) Assembled binary64 matrix, solved exactly | (C′) Represented contributions summed exactly, solved exactly (V1) | Rounded matrix promoted to 128 bits | Ordinary binary64 | Primitive rebuild, p = 128 | Stop rule |
|---|---|---|---|---|---|---|
| Axis-aligned, k = 1e-4 (bending soft mode) | 1.05e-5 (8 of 22 fail) | 1.05e-5 (5 fail) | 1.05e-5 | 1.06e-5 | 7.5e-28 | 128 accepted (8.1e-28) |
| Skew (3,4,0), k = 100 (ordinary scale) | 4.7e-12 | 4.7e-12 | 4.7e-12 | 9.4e-12 | 1.7e-34 | 128 accepted |
| Skew (3,4,0), k = 1e-4 (N05 class) | 5.5e-6 (10 fail) | 4.7e-6 (10 fail) | 5.5e-6 | 8.7e-6 | 0 | 128 accepted (1.3e-28) |
| Skew (3,4,0), k = 1e-12 (N06 class) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 0 | 128 accepted (1.9e-20) |
| Oblique (2,3,6), k = 1e-4 | 1.07e-3 (19 fail) | 1.07e-3 (19 fail) | 1.07e-3 | 2.7e-3 | 2.6e-25 | 128 accepted |
| Skew, k = 1e-28 (arithmetic stress) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 1.8e-4 (10 fail); the pivot screen passes, margin 3.4 | 128 rejected (1.8e-4). p = 256 has error 5.0e-43 and is accepted against 512 (5.3e-43) |
| Six-member skew run, k = 1e-12 (39 free DOFs in one block) | nonpositive pivot | nonpositive pivot | nonpositive pivot | nonpositive pivot | 2.1e-19 | 128 rejected (2.1e-19 > 2^-64 ≈ 5.4e-20). p = 256 has error 0; its 512 verification was not run |
| k = 0, a genuine mechanism | — | — | — | — | nonpositive pivot at 128, 256 and 512 (screen margins −0.003 to −0.005) | never solved |

**What follows:**
- **(C) is refuted.** Solved exactly, the represented binary64 contributions still have scaled errors of 5e-6 to 1e-3 against the 1e-9 criterion; this holds in both (C) and (C′). For N06-class skewed cases, the represented matrix is not even positive definite in exact arithmetic. Promoting the rounded matrix changes nothing.
- **An axis-aligned transform is not enough.** In the first row the transform is a signed permutation. The error comes from the independently rounded bending coefficients, which break the element's rigid-rotation null space.
- **Primitive basic-deformation formation meets 1e-9** at 128 bits, with more than 15 orders of margin, for N05 and N06 classes, axis-aligned, skewed and oblique, and for a block of order 39.
- **The pivot screen alone is not enough.** At k = 1e-28 and 128 bits it passes (margin 3.4), yet the answer is 1.8e-4 wrong. The 2p agreement rule catches this. This is why the stop rule is required, not optional.
- **The stop rule, on these cases.** It escalated the six-member case, whose 128-bit answer already met 1e-9. On these cases it accepted no failing candidate.
  - Revision 1 went further and said it "never accepted a failing candidate". **That claim is withdrawn** (V1-B1). The stop rule detects only error that depends on the precision.
  - V1's check L shows the failure mode: a contribution lost identically at p and at 2p leaves the two candidates in agreement on a wrong answer.
  - Revision 2 removes every such common-mode class it knows of, by forming sums exactly (§4.1.2, §4.1.9). It does not claim the stop rule is a proof.
- **Precision does not regularize a mechanism.**

This is the evidence the refutation lacked ("a numerical analogue, not a deployed spatial-frame test"). It still is not Rust, MPFR, the product, or a performance result.

### 3.2 Revision 2 evidence: exact sums and combinations (V1-B1, V1-S8)

`_run_records/probe_rev2_b1.py` → `probe_rev2_b1.stdout.json`. It uses revision 1's emulation, imported unchanged, on D1's N05-class skew case. Loads of 1e80 are arithmetic stress only, outside any physical claim.

| Check | Rule | Stop rule | Error of the accepted candidate |
|---|---|---|---|
| B1-L: V1 check L, moment contributions (1e80, 1e-8, −1e80) on RX plus 2e-8 on RY | Loads folded at p (revision 1) | 128 vs 256 agree to 1.8e-28 → **128 accepted** | 0.5 of the body scale; strict relative 2.0 (7 quantities fail) |
| | **Load ledger: exact per-DOF sum, rounded once (revision 2)** | 128 accepted (1.3e-28) | **0**, no failures |
| B1-C: combination A + B − A2, with A = A2 = 1e80 and B = 1e-8 on RX | Combined at p term by term | 128 vs 256 agree exactly → **128 accepted** | 1.0 (8 quantities fail) |
| | **Combined as one exact expansion, rounded once, under the stop rule** | 128 accepted (5.4e-28) | 5.4e-28 of the body scale; no failures |
| B1-E: combination (P, ε) − P, ε/P = 1e-45 (the true value needs more than 128 bits) | Combined exactly, under the stop rule | **128 rejected** (1.0); 256 accepted against 512 (1.3e-21) | 1.3e-21 of the body scale; no failures |
| S8-W: a stiff member, then a soft member (E scaled by 1e-10 or 1e-14) to a node grounded by 1e12 springs | Revision 2 | 128 accepted (2.5e-36) | No failures. The far node's quantities sit at 2e-17 to 6e-16 (s = 1e-10) and 2e-21 to 6e-20 (s = 1e-14) of S\*, below the floor, yet exact at binary64 publication |

The loss V1 showed is a property of how sums are formed, not of the stop rule. Once the sums are exact, the same stop rule accepts correct candidates and rejects the one that needs escalation. S8-W shows that below-floor quantities can be exact in practice, but the stop rule does not guarantee them. The guarantee is stated in §4.1.6.

**Recommendation.** (A), with basic-deformation element formation. Keep (C) as a kernel test oracle within its merged scope. Defer (B)'s global transformation. Its useful insight, keeping each element's deformation coordinates, is already taken up by (A)'s formation.

## 4. Recommended design

### 4.1 W1 — the retained-precision structural method

Proposed names, all placeholders for ROOT:
- method token `contribution_preserving_multiprecision_v1`;
- policy `M03-INTEGRITY-MP-v1`, a precision-aware reading of M03-INTEGRITY-v1;
- diagnostics `RETAINED_PRECISION_SELECTED` and `RETAINED_PRECISION_UNAVAILABLE`.

#### 4.1.1 Kernel home and types (new files under `FK/structural/retained/`)

- **`wide.rs`: the arithmetic (§4.11).**
  - `Wide<const L: usize>`: sign, `i64` exponent and `[u64; L]` significand, with L = 2, 4, 8, 16 for 128, 256, 512 and 1024 bits.
  - `+ − × ÷ √`, each rounded to nearest, ties to even, at a runtime precision p ≤ 64L.
  - Exact conversion from `f64`.
  - Correctly rounded conversion to `f64`, with an explicit outcome: normal, subnormal (with its relative precision bound), underflow to zero, or overflow.
  - A work counter.
  - Integer-only arithmetic, so results are bitwise reproducible across platforms.
- **`source.rs`: `PrimitiveSource`,** an immutable per-case declaration with an identity digest.
  - Node coordinates (binary64).
  - `StraightMember {node_i, node_j, E, G, A, Iy, Iz, J, y_reference}`, all binary64 operands.
  - Springs `(dof, k > 0)`.
  - Constraints `(dof, value)`.
  - Nodal loads `(dof, value, source_id)`.
  - W1b adds member uniform loads, eigen axial forces, end thrusts, constant-effort forces and user relative-DOF springs.
  - Only supported families can be constructed, so exclusion is a type-level fact.
  - Validation rejects nonfinite or nonpositive properties and incomplete partitions. It also rejects any derived primitive that is subnormal, because its bits were lost before this boundary.
- **`assemble.rs`, `factor.rs`, `recover.rs`, `adaptive.rs`:** formation, assembly and reduction; the profile LDLᵀ and screens; recovery; the schedule, stop rule and evidence.
- **The opaque result.** `RetainedSolve` is bound to its source and precision. `RetainedSolve::publish()` rounds each quantity once.
  - `RetainedCombination::form(&[(factor, &RetainedSolve)], p)` is revised for V1-B1. It forms `Σ cᵢ·uᵢ` and `Σ cᵢ·fᵢ` as one exact expansion per component: TwoProduct of the binary64 factor and the p-bit state, then TwoSum accumulation. Each is rounded once to p, and actions and reactions are recovered from the combined state at p.
  - It is formed at p and at 2p from the two precisions' retained states. **Its published outputs go through the stop rule** exactly as case outputs do, with S\* taken from the combination's own body scale.
  - A combination whose candidates disagree escalates independently of its operand cases. At the ceiling, it is withheld with `RETAINED_PRECISION_UNAVAILABLE` (reason `combination_unresolved`), and its operand cases keep their standing.

No caller can supply a matrix, factor, closure or label. This mirrors the KREV trust boundary of `finish_structural` (`structural.rs:1086-1107`).

#### 4.1.2 Formation, assembly and reduction at precision p

Every operation below is rounded to p bits, and every input is a binary64 value lifted exactly.

1. **Frame.** `d = x_j − x_i`, `L = √(d·d)`, `e_x = d/L`. Gram-Schmidt of `y_reference` gives `e_y`, normalized, and `e_z = e_x × e_y`, normalized. This is the product's own algorithm (`FK/lib.rs:511-525`), so the axes converge to the exact axes of the binary64 geometry. No axis tolerance is used at p.
2. **Basic deformations.** `B_local` (6×12) holds the axial extension, the twist, and the end rotations relative to the chord in both local planes, using `1/L`. `B = B_local·T`.
3. **Constitutive operator.** `D` (6×6) is `diag(EA/L, GJ/L) ⊕ (EI_z/L)[[4,2],[2,4]] ⊕ (EI_y/L)[[4,2],[2,4]]`. `Bᵀ D B` reproduces the product's local matrix; the probe agrees to 1.3e-16 in binary64, with the same zero pattern.
4. **Assembly (exact, revision 2).** Each pattern entry of K is formed as one exact expansion of its p-bit element contributions and binary64 spring stiffnesses, using TwoSum in `Wide`. It is rounded once to p.
5. **Loads: the load ledger (revision 2).** Each DOF's load is the exact sum of its identified contributions, rounded once to p. Nodal forces and moments are exact binary64 values. W1b's equivalents (uniform, thermal, thrust, constant effort) are p-bit values formed from binary64 inputs. It is the same ledger S11-F introduces for the ordinary route (`S11_CONTAINMENT.md` §4.2–§4.3), with the same contribution granularity, carried at precision p: `FK/exact_sum.rs`'s `ExactAccumulator` holds the exact sum and gains one further projection, to `Wide<L>` at p, beside its binary64 rounding.
6. **Reduction (exact, revision 2).** Free and prescribed maps come from the source. Each `rhs_i` is one exact expansion: the ledger terms plus the exact products `−K_ic·u_c` of the p-bit coefficients and binary64 prescribed values, rounded once to p. Neither K nor rhs is ever rounded back to binary64.

Because `B·r = 0` holds exactly for every rigid motion r of the exact geometry, the artificial rigid-mode stiffness is O(2^-p·a), not O(2^-53·a).

**The exact-sum rule (V1-B1).** Every multi-term sum outside the factorization and the triangular solves is formed exactly and rounded once. That is: loads, stiffness entries, the reduced right-hand side, reactions (§4.1.5), basic-deformation recovery sums (§4.1.5), combinations (§4.1.1) and the residuals (§4.1.4). §4.1.9 explains why this is the class the stop rule cannot see, and what remains. Every such sum uses one accumulation discipline: exact accumulation of its binary64 or p-bit terms, then one correct rounding (to nearest-even), shared with the binary64 route's `FK/exact_sum.rs`. An exact zero is +0.0. **Zero-witness boundary (ROOT, D-S11-1).** The only exception is a published diagnostic rendering whose replaced expression publishes −0.0 for an exact zero today: the intended-action `ResidualRow` fields `residual` and `normalized_residual` produced at `FK/structural.rs:554`. There the exact zero keeps −0.0, written as a literal, so committed evidence bytes are unchanged. Those two fields are consumed only by the `Debug` rendering in diagnostics (`PP:1057`, `:2608`, `:2619`) and by the gate ratio through `r.abs()`, where the sign has no effect. `ResidualRow` has no `Serialize`, and `intended_residual_rows` is read elsewhere only for size accounting (`SA:765`). The witness never applies to the ledger, the force vector, a recovery sum, a published result row (rows are bound byte for byte by receipts, `source_receipt.rs:996`), a `source_recovery` or `exact_boundary` comparison, or any value at precision p. All of those use +0.0. This narrows `S11_CONTAINMENT.md` §4.1.3, which had also proposed the witness at the recovery sums E3, E4 and E5. Any committed −0.0 that passed through one of those sums as its zero therefore becomes +0.0, and S11-K's fixture stop rule reports it before any regeneration.

#### 4.1.3 Factor, screens and mechanism handling

- **Geometry first.** The geometric rigid-body assessment runs before any factor (W4 generalizes it). A witnessed mechanism is refused and never escalated.
- **Factor.** RCM ordering from the pattern. The ordering is integer data, shared with the binary64 sparse path. Profile LDLᵀ at p.
- **Pivot screen** at precision p: `d_i > 64·γ_p(m_i)·c_i`, where `γ_p(m) = m·2^-p/(1 − m·2^-p)`. This is the M03 structure with `u_p = 2^-p`. A failed pivot at p escalates to the next p; at the ceiling the result is unresolved. It is never read as a mechanism.
- **Negative energy.** Checked for pattern pairs only, at p, against the intended K.
- **Condition estimate.** Hager–Higham with the p-factor. `rcond ≤ 2^-(p-1)` counts as unresolved at p and escalates. The estimate is published as model information, with the label "sensitivity to matrix-entry perturbation, not to authored parameters".

#### 4.1.4 Solve and refinement

1. Solve with the p-factor.
2. Evaluate `r = f − K u` against the intended system re-formed at p + 64, from the same primitives. The load term is the same ledger rounded once to p + 64, not a fold. Each `r_i` is one exact expansion of the p-bit products and the ledger terms (revision 2).
3. Gate each free row with the componentwise guarded ratio against `64·γ_p(m_i)`.
4. Allow at most three corrections with the p-factor, as today (`structural.rs:960`).
5. If it still fails, escalate.

#### 4.1.5 Recovery before rounding

For each member, at precision p:
- `d_local = T u`;
- `e = B_local d_local`;
- `Q = D e`;
- end actions (node-on-element, local) = `B_localᵀ Q`;
- station actions, which are linear for nodal loads (W1b adds the load term at p).

Also at p: spring actions `−k u`, and reactions `(K u − f)` on the constrained rows. Only then is each published quantity rounded once, with its representability outcome.

**Revision 2.** Each component of `d_local`, `e`, `Q` and the end actions is one exact expansion of its (at most five) product terms, rounded once. Each reaction is one exact expansion of `K_cj·u_j` products and the ledger terms, rounded once.

**Derived stresses** keep the existing binary64 publication methods: the circular maximum `exact_straight_summary_extrema`, and stations. They are applied to the once-rounded actions. This is a second rounding, but of a well-conditioned function: a sum of non-negative terms. It is not a recovery from rounded displacements. The row provenance says so.

#### 4.1.6 Adaptive schedule and stop rule

**Schedule.** Candidates at p = 128, 256 and 512, each verified at 2p. The ceiling is 1024 bits, as CONTRACT requires. The verification solve at 2p repeats formation from the binary64 operands; it does not reuse p quantities. Solves are reused: a rejected 128 candidate's 256 verification becomes the next candidate. So at most four solves run: 128, 256, 512 and 1024.

**Stop rule** (proposed; ROOT registers it as method policy):
- **Accept p** when every published quantity q satisfies `|q_p − q_2p| ≤ 2^-64 · max(|q_2p|, S*)`.
- **The scale S\*** is built per connected body and per kind (revision 3: eight kinds; §4.1.6.1 gives the exact definition on published rows). Let S(kind) be the largest |q| of that kind in the body, and L_b the body's extent. Then:
  - translation: `S* = max(S(translation), L_b·S(rotation))`;
  - rotation: `S* = max(S(rotation), S(translation)/L_b)`;
  - force (member, wall and reaction forces): `S* = max(S(force), S(moment)/L_b)`;
  - moment: `S* = max(S(moment), L_b·S(force))`;
  - **twist of member m** (revision 3, F2; harness-only): `S*_tw(m) = S*(moment)·(L/GJ)_m`, formed as `fl(mo/k_t)` with the receipt's k_t = G·J/L (§4.1.6.1 item 7);
  - **extension of member m** (revision 3, F2; harness-only): `S*_ext(m) = S*(force)·(L/EA)_m`, formed as `fl(fo/k_a)` with k_a = E·A/L;
  - **stress at member m, for a stress row kind with propagation factor k** (revision 4, R3B-1): `S*_σk(m) = S*(force)/A_m + k·S*(moment)/Z_m`, with A the section area and Z = I/c the section modulus. k = 1 for component stresses, √2·i for an intensified row with its own i (both formed from one end's published actions), and, for the span-statics rows whose section resultants are rebuilt from one end's actions (revision 5, R4-1), **2√2 for the circular maximum and 4 for the open-formula summary** (§4.1.6.1 item 7);
  - **input_derived** rows (a class, not a kind): values that do not depend on the solve (pressure-only Lamé and pressure stresses, prescribed displacement rows, echoed user inputs). No S\*, no threshold; they can be bound like relative rows and appear in neither receipt list (D2 §4.9.10).
  - **Proof that the floor holds at 1e-9 with k (revision 4, R3B-1; revision 5, R4-1).** Let ε = 2^-64 and write fo = S\*(force), mo = S\*(moment). The stop rule bounds the error of every published action by ε·S\*_kind. Derived stress rows are formed in binary64 from published actions, in one of two ways.
    - **From one end's own published actions**, with no rebuilt resultant: component stress σ = N/A, M/Z or T/(2Z) (error ≤ ε·fo/A or ε·mo/Z, plus one division's rounding ≤ u·|σ|); the intensified row q = i·hypot(σ_by, σ_bz) built from the end rows (pinned by the test at `PP:15964-15983`), for which hypot is 1-Lipschitz in the Euclidean norm, so |δq| ≤ i·√2·ε·mo/Z plus roundings. So k = 1 and k = √2·i.
    - **From section resultants rebuilt by span statics from one end's actions** (`straight_section_resultants`, `PP:8102-8130`, used by `exact_straight_summary_extrema`, `PP:8140-8208`, and by the open-formula summary's `straight_summary_extrema`, `PP:8223`, called at `:3246-3259`). The rebuilt moment is M(x) = M_i − V_i·x plus load terms, and through S11-K it is one exact sum rounded once (E4/E6, `S11_CONTAINMENT.md` §2.2), with V_i·x an exact product. So |δM(x)| ≤ |δM_i| + x·|δV_i| ≤ ε·mo + L·ε·fo, and L·fo ≤ L_b·fo = max(L_b·S_fo, S_mo) = mo, because a member's length is at most its body's extent. Hence |δM(x)| ≤ 2ε·mo per component, and the rebuilt axial force carries ε·fo. The circular maximum q = |N_w|/A_s + hypot(M_y, M_z)/Z then has |δq| ≤ ε(fo/A + 2√2·mo/Z), so **k = 2√2**. The open-formula summary q = |σ_ax| + |σ_by| + |σ_bz| (pressure-longitudinal term zero) has |δq| ≤ ε(fo/A + 4·mo/Z), so **k = 4**. The exact rebuild's own rounding is ≤ u·|M(x)|, relative to the resultant itself.
    - Magnitude rows are formed at p, rounded once, and checked by the stop rule themselves, so k = 1.

    In every covered formula the terms are nonnegative (or a single term), and each rebuilt resultant is rounded once relative to itself. So each term and each operand is at most q in magnitude, and the binary64 roundings add at most c·u·|q| with c ≤ 6. For a row classified `relative_verified`, |q| ≥ t = 2^-34·S\*_σk (exact for S\* ≥ 2^-988), so |δq|/|q| ≤ ε·S\*_σk·(1 + 4u)/(2^-34·S\*_σk) + 6u = 2^-30·(1 + 4u) + 6u ≈ 9.3132e-10 < 1e-9. The (1 + 4u) covers the formation of S\*_σk (four roundings) and a k constant rounded upward; the published-versus-2p S\* difference, of order (ε + u)·S\*, is inside the same margin, which is about 6.9e-11. V1's probe J gives 9.31e-10 at the threshold with k = 2√2 and k = 4 (BACKCHECK_R4 §2), and the recount with these k changes no withheld count on the committed selected cases (V1's probe W; this revision's `b_proof.py` uses the same k).

    **Where the proof does not hold, the row is `not_covered`:**
    - the open-formula summary when a pressure-longitudinal term is present (base = σ_ax + σ_pl is a signed sum, so an operand can exceed q and its rounding u·|σ_ax| is not bounded by q);
    - any stress formula on pressure members whose wall force is not itself a published, stop-rule-checked row. This includes the pressure membrane inside `exact_straight_summary_extrema` (`recover_wall_effective_membrane(r[0], …)`, `PP:8183-8189`, the call at `:8186`), which takes the axial force rebuilt by span statics, not the published wall-force row (V1, BACKCHECK_R4 §2).

  - Twist and extension are not product rows. They exist so that the harness's derived twist and extension (§4.10) have a stated scale, and they inherit their torque's and axial force's verification exactly. Stresses are published as derived rows (§4.1.5); their scale maps the action scales through the section, so a stress is verified to the accuracy its actions are.
- **A body with all scales zero** is unloaded and unmoving, and must agree exactly.
- **Rationale.** `2^-64` is the binary64 significand plus 11 guard bits. The coupling gives a structurally zero kind a physical scale, so noise can be accepted. The probe (§3.1) shows that per-group scales without this coupling never accept the noise in structural zeros.
- **Stricter option for ROOT.** Per-member scales with a body-level floor. It verifies small actions against their own member, but forces 256 bits on many skewed soft models.
- **What it is not.** The stop rule is operational evidence of numerical convergence, not a forward-error enclosure. It changes no comparison criterion. It is the acceptance threshold for a selected case (V1-S8).
- **Scope (revision 2).** Every published quantity of a selected case, and every published output of a combination formed over retained states (§4.1.1).

**What acceptance guarantees (V1-S8; enforced in revision 3, S8-R).**
- If q_2p is accurate well beyond 2^-64·S\*, acceptance bounds the candidate's error by `2^-64·S*`.
- **The floor.** Relative 1e-9 on a published q follows for `|q| ≥ R·S*` with **R = 2^-34** (≈ 5.82e-11, bits `0x3DD0000000000000`). Below the floor the guarantee is the absolute bound `fl(2^-64·S*)`. For exact zeros, relative accuracy is undefined.
  - **The threshold's own rounding (revision 4, N-2).** `t = fl(2^-34·S*)` is exact for S\* ≥ 2^-988. Below that the product is subnormal and can round. Such a body scale is below 1e-297 in SI units. It is handled explicitly: when S\* < 2^-988, every row of that body and kind is classified `absolute_verified`. Readers apply the same rule.
  - **Why 2^-34, not 10^9·2^-64.** 10^9·2^-64 = 1953125·2^-55 is itself an exact binary64 value (bits `0x3DCDCD6500000000`), so exactness is not the reason. The reason is margin: at R = 2^-34, `2^-64·S*/|q| ≤ 2^-30 ≈ 9.31e-10` above the floor, which leaves room for the publication rounding (u ≈ 1.1e-16) and for the difference between the published S\* and the 2p S\* the stop rule used (of order 2^-64). And `fl(2^-34·S*)` is exact whenever S\* is normal, so the threshold itself is not rounded. The floor rises by 7 %; on R1's references this changes no count (probe `floor_kinds.py`, both constants).
- **A proof for every quantity is not available.** No agreement rule can separate a structural zero computed as noise from a legitimately tiny value, without a floor. So the floor is stated and enforced:
  1. **Classification on the published value.** For every published row of a selected case whose class in the closed table (§4.1.6.1 item 2) is a scaled kind, and every published output of a retained-state combination, the producer computes `t = fl(R·S*)` for the row's body and kind from published data (§4.1.6.1), and classifies the row `absolute_verified` iff `|q| < t`, else `relative_verified`. It never classifies on the unpublished precision-p value. A row whose (kind, unit) is not in the closed table is `not_covered` (the default). `input_derived` and `non_quantity` rows carry no threshold.
  2. **The receipt** carries R's bits, S\* per body and kind as bit strings, the `absolute_verified` result ids with their bound `fl(2^-64·S*)`, and any `not_covered` ids (§5).
  3. **Readers recompute** S\* and the classification bit for bit (D2's G5b and G5c). A mismatch makes the result `unsupported`.
  4. **Standing and rule binding (ROOT).** `absolute_verified` and `not_covered` quantities are **withheld from reliance**: rule binding refused with D2's two codes, `RULE_QUANTITY_BELOW_VERIFIED_FLOOR` for `absolute_verified` and `RULE_QUANTITY_NOT_COVERED` for `not_covered` (revision 4, N-5), never counted as Passed, a headline refused when it names one, shown as uncovered with their absolute bound, and carried with their class in every canonical and exported form (D2 §4.9.9). The case's other quantities keep their standing. **The cost is measured in revision 4 (§8.1): on the committed selected cases, 8 to 68 of 80 to 111 rows are withheld, almost all of them exact structural zeros.** §8.1 sets out the owner-level options, including D2's interval binding (DD-13).
  5. **VP-ROBUST** requires every reference quantity's comparison scale to be at least R·S\* of its case. A comparison that is not is **not covered by the guarantee**, reported as such, and **never counted as a pass** (§4.10).
  6. **The weak-coupling control S8-W** (§3.2) stays in VP-ROBUST as a positive control: its below-floor quantities are published, classified `absolute_verified`, withheld from reliance, and exact in the probe.
- **D-12 is resolved** by ROOT's S8-R ruling: withhold, as above.

#### 4.1.6.1 S\* on published rows (the interface D2's G5b recomputes)

Every step is binary64, in the stated order, so readers reproduce S\* bit for bit.

1. **Bodies.** The connected components of the invocation model's element graph: straight members, curved spans and user stiffness elements connect their nodes. Springs to ground, rigid restraints and imposed motions do not connect. Each row belongs to the body of its node, member or support node.
2. **The closed (kind, unit) table (revision 4, R3B-2; revision 5, R4-4: the single source, which D2 mirrors).** Every row kind the three base identities (preview-physics-1, physics-1, load-reference-1) and their sources emit is listed once, with one class and its admitted units. The table is carried in each successor identity's pinned semantic table (D2's DD-14). **Any (kind, unit) not listed is `not_covered`.** Two entity rules sit beside the (kind, unit) key, because a (kind, unit) key alone cannot decide them (items 2a and 2b).

   | Class | Row kinds (admitted units) |
   |---|---|
   | translation | `global_nodal_displacement_{x,y,z}` (m, mm); `displacement_magnitude` (m, mm), formed at p |
   | rotation | `global_nodal_rotation_{x,y,z}` (rad) |
   | force | `element_local_axial_force`, `element_local_shear_force_{y,z}`, `pipe_wall_axial_force_v2`, `pipe_effective_axial_force_v2` (N, kN); `support_reaction_component_v2` and `pipe_wall_endpoint_action_v2` (N, kN); `support_reaction_force_magnitude_v2` and `reaction_resultant` (N), formed at p |
   | moment | `element_local_torsional_moment`, `element_local_bending_moment_{y,z}` (N·m, kN·m); `support_reaction_component_v2` and `pipe_wall_endpoint_action_v2` (N·m, kN·m); `support_reaction_moment_magnitude_v2` (N·m), formed at p |
   | stress, k = 1 | `element_local_axial_normal_stress`, `element_local_bending_normal_stress_{y,z}`, `element_local_torsional_shear_stress` (MPa, Pa); `pipe_axial_membrane_stress_v2` (Pa), where its member carries no pressure (otherwise `not_covered`, §4.1.6) |
   | stress, k = √2·i | `component_equal_factor_intensified_bending_stress_v1` (Pa), with the row's own i from the invocation's component input; k_i rounded upward (item 7) |
   | stress, k = 2√2 | `pipe_elastic_normal_stress_maximum_v2` (Pa): span statics (R4-1); `not_covered` where its member carries pressure (the rebuilt membrane, §4.1.6) |
   | stress, k = 4 | `open_formula_stress_summary` (MPa): span statics (R4-1), **only where the case has zero pressure**; otherwise `not_covered` |
   | input_derived | `pipe_lame_hoop_stress_v2`, `pipe_lame_radial_stress_v2` (Pa); `pipe_section_pressure_hoop_stress`, `pipe_section_pressure_longitudinal_stress` (MPa); `constant_effort_support_applied_load` (N); the echoed-input review kinds `component_user_stress_multiplier_review` (MPa), `component_user_stiffness_macro_element_review` (N/m, N·m/rad), `constant_effort_user_input_review` (N, m), `spring_hanger_user_input_review` (N, m, N/m, N·m/rad), `expansion_joint_pressure_thrust_load_review` (N, E16's exact sum of inputs); units as committed envelopes publish them; and, by the entity rule 2a, displacement and rotation rows at an `input_derived_dofs` DOF |
   | non_quantity | exactly these four: `sparse_live_path_dense_parity_relative_delta`, `linear_solver_mode_basis`, `modulus_basis_record`, `combination_modulus_basis_record`. Observations and records: never bound to a rule, never in a receipt list |
   | not_covered | every `nonlinear_support_*` kind, `curved_bend_macro_element_review`, every row that entity rule 2b excludes, the pressure cases above, and anything unlisted. W1 never selects a nonlinear or curved case, so these rows do not appear in a selected case today |

   **2a. Restrained and prescribed DOFs (entity rule).** A displacement or rotation row at a rigidly restrained or prescribed DOF is `input_derived`: its value is the prescription (`finish_checked_factor` sets u_i = v, V1, BACKCHECK_R4 §8). The receipt carries the set as `input_derived_dofs`, a list of (node id, component) pairs covered by the source identity digest. Readers classify from that list. With an invocation, D2's G5c requires the list to equal the set derived from the invocation's restraint sets and 0.4.0 boundary motions, in both directions.

   **2b. Member entity rule.** A member-entity row (element-local actions and stresses, wall rows, stations, maxima) is covered only if its member appears in the receipt's per-member section terms (item 7); otherwise it is `not_covered`. This separates curved arc and station rows, which share straight-row kinds, from straight members. They cannot occur in a selected case today.

   **The eleven BACKCHECK_R4 §3 mismatches, resolved here:**
   1. units: listed per kind above;
   2. restrained and prescribed rows: rule 2a, with `input_derived_dofs`;
   3. echoed-input review kinds: `input_derived`, bindable, in neither receipt list;
   4. `constant_effort_support_applied_load` (N): `input_derived`;
   5. non_quantity: the explicit list of four. A future kind whose category is not a physical quantity, and which is not in the list, is `not_covered`, never non_quantity;
   6. curved: `curved_bend_macro_element_review` is `not_covered`, and curved rows are excluded by rule 2b;
   7. the summary's condition: zero pressure in the case. W1a's coverage requires zero pressure (§4.3), so it always holds in a W1a-selected case, and readers check it from the receipt's route. W1b adds a per-member `pressure_longitudinal_zero` flag to the receipt;
   8. intensified k_i: rounded upward (item 7);
   9. magnitudes: k = 1 only (formed at p); there is no √3 alternative;
   10. receipt section terms: A, Z, L, E·A/L, G·J/L; no twist or extension scales in the receipt (item 7, §5);
   11. N-2: S\* < 2^-988 makes every row of that body and kind `absolute_verified`, and G5c mirrors it.

   The table was built from the semantic-contract constants and from every row kind in committed envelopes (`scan_row_kinds.py`).

   **Where the two named kinds actually appear.** P1 (relayed) reports that the ordinary pressure-route cases (physics-1, and physics-source's ordinary-pressure case) publish neither `reaction_resultant` nor `open_formula_stress_summary`. Their headline comes from the stress rows (`pipe_elastic_normal_stress_maximum_v2`). So no headline is refused on that route by this table.
3. **Units.** Rows are read in their published unit and converted with pinned factors, each one binary64 operation: m, rad, N, N·m, Pa ×1; mm ÷1e3; kN and kN·m ×1e3; MPa ×1e6. A row in any other unit is `not_covered`.
4. **S(kind)** is the largest |q| over the body's rows of that kind (exact).
5. **L_b.** With the body's node coordinates from the invocation model: `d_a = fl(max_a − min_a)` per axis, `L_b = fl(sqrt(fl(fl(fl(d_x·d_x) + fl(d_y·d_y)) + fl(d_z·d_z))))`. A single-node body has L_b = 0; then the coupled terms are omitted.
6. **Coupling,** in this order: `tr = max(S_tr, fl(L_b·S_rot))`; `ro = max(S_rot, fl(S_tr/L_b))`; `fo = max(S_fo, fl(S_mo/L_b))`; `mo = max(S_mo, fl(L_b·S_fo))`.
7. **Per-member kinds (revision 4, R3B-3; revision 5, R4-1 and R4-4).** The section terms come from the receipt, as bit strings: for each member, A, Z, L, k_a = E·A/L and k_t = G·J/L exactly as the product formed them (whatever route and temperature interpolation it used), covered by the source identity digest. Readers take them from there and cross-check them against published section evidence where it exists (only the exact route publishes A_s and Z, `PP:8133-8137`; elsewhere L, E·A/L and G·J/L rest on the digest). A mismatch is D2's `RETAINED_PRECISION_SECTION_MISMATCH`. Then `σ_k(m) = fl(fl(fo/A) + fl(k·fl(mo/Z)))`, with the pinned constants:
   - k₁ = 1;
   - k√2 = `0x3FF6A09E667F3BCD` (1.4142135623730951, the nearest double to √2, which is ≥ √2);
   - k_{2√2} = 2·k√2 = `0x4006A09E667F3BCD` (exact doubling, ≥ 2√2);
   - k₄ = 4;
   - k_i = fl↑(k√2·i), rounded **upward** (D2's rule, R4-4), so k_i ≥ √2·i always. The rounding direction is the only bit-level choice, and the three languages implement it as "nearest, then next-up if the nearest is below the exact product" (the exact product of two binary64 values is decidable with an error-free product).
   
   Twist and extension scales are **harness-only** and are not in the receipt. The harness forms `tw(m) = fl(mo/k_t)` and `ext(m) = fl(fo/k_a)` from the receipt's k_t and k_a. That matches the derived twist T/k_t and extension N/k_a (§4.10) operation for operation. Curved spans are not covered (rule 2b).
8. **Combinations** over retained states use their own body's rows (§4.1.1).

**Failure.**
- At the ceiling, or when the budget runs out, the case is unresolved: `RETAINED_PRECISION_UNAVAILABLE` with the attempted precisions and the reason.
- The case keeps its ordinary outcome and standing.
- Nothing is relabelled as solved.

#### 4.1.7 Budgets and reuse

- **Work units.** Counted in limb-multiply equivalents per attempt. Successful, failed and verification work is all charged.
- **Limits.** ROOT selects the per-case and per-invocation limits from the W3/W5 measurements, as `COMPOSITE_ENGINE/RESOURCE_POLICY.md` did for physics-source-1. No implementing slice ships without them.
- **Factor reuse.** Linear cases that share a modulus basis and state share one p-factor per precision, with multiple right-hand sides.

#### 4.1.8 Determinism and replay

- The arithmetic is integer-only and the ordering is fixed. The same `PrimitiveSource` therefore gives a bit-identical retained state on every platform.
- A reader in Rust can replay a captured invocation and compare a retained-state digest. That digest is sha256 over the canonical limbs of u_p and every member's Q.
- The full p-state is not persisted by default; replay reproduces it (§5).
- **Replay is not a standing input.** In D2's revision 2 (§4.9.4) it is a Rust validation-lane audit in `numerical_robustness`, so all three languages decide standing on the same basis.

#### 4.1.9 What the stop rule can and cannot see (revision 2, V1-B1)

- **The stop rule detects error that depends on the precision.** Such error is of order 2^-p at p and 2^-2p at 2p, so the two candidates differ by about the error itself.
- **It cannot detect common-mode loss:** a contribution lost identically at p and at 2p. That needs a sum whose exact value is smaller than the rounding of an intermediate partial sum at both precisions. There are two known forms:
  - **Mixed-sign sums of exact source values.** V1's check L: binary64 loads are exact inputs, so their fold loses the same bits at every precision below the span of the sum.
  - **Sums in which bit-identical computed operands cancel.** V1's combination A + B − A2: A and A2 are the same computation, identical at each precision. So are two identical members meeting at a node, or a rigid translation shared by both ends of a member.
- **Revision 2 removes both forms.** Every such sum is formed as an exact expansion and rounded once (§4.1.2, §4.1.4, §4.1.5, §4.1.1). Only the final rounding remains, relative to the sum itself.
- **What remains inside the factorization and triangular solves.** Every operand there carries formation error of order 2^-p, which changes at 2p. Error that grows from it is precision-dependent, and the refinement residual at p + 64, formed against the exact-expansion right-hand side, checks the solve independently of the factor.
- **This is an argument, not a proof.** The negative controls (§7.3, items 13–16) make each removed form a test that must fail when reintroduced.
- **The same rule holds on the ordinary binary64 route through S11-K and S11-F:** one exact load ledger, rounded once, and exact recovery sums through the same accumulator, with an audit against the exact per-DOF loads (`S11_CONTAINMENT.md` §4). The F-slices supersede that route's recovery with recovery at p.

### 4.2 Coverage and refusals

| Family | W1a | W1b | W1c | Refusal reason while excluded |
|---|---|---|---|---|
| Straight frame members | yes | | | — |
| Global-axis linear springs (k > 0) | yes | | | — |
| Rigid restraints, zero value | yes | | | — |
| Nonzero prescribed support motion (0.4.0 `ResolvedCase`) | kernel yes | facade, after T1 | | `prescribed motion unsupported` |
| Nodal forces and moments | yes | | | — |
| Combinations (T0R-admitted `mechanics`, subtraction, range) | at the facade: exact expansion over the retained states, rounded once, under the stop rule (§4.1.1); T0R gates unchanged | | | `combination withheld by gate` (existing codes); `RETAINED_PRECISION_UNAVAILABLE` (`combination_unresolved`) at the ceiling |
| Element uniform loads, including weight | | yes | | `element load producer unsupported` |
| Thermal eigen axial load (0.4.0 resolved) | | yes | | `thermal producer unsupported` |
| Pressure thrust on straight members; exact-route pressure regions | | thrust yes; regions in W1c | yes | `pressure producer unsupported` |
| Constant-effort support forces | | yes | | `constant-effort producer unsupported` |
| User stiffness elements | | yes (relative-DOF B form) | | `user-matrix element unsupported` |
| Curved bend macro elements | | | with T4 | `curved element unsupported` |
| Components, releases | | | proposed owners T4 (joints M07, bends M02) and T7 (specialized components M17, M20), each when its element model is qualified (V1-N5) | `component or release unsupported` |
| Equivalent static | | | **open**: no owning tranche in the work graph; recorded for ROOT to assign (V1-N5) | `equivalent-static unsupported` |
| Nonlinear or contact supports | T5 | T5 | T5 | `nonlinear support family (T5)` |
| Legacy `imposed_displacement` | refused upstream (T0R SF-E) | | | — |

**How a refusal works.** One info diagnostic per case, `RETAINED_PRECISION_UNAVAILABLE`, naming the family, the model entity and the case. The case keeps its ordinary rows and standing, so a Sensitive case is withheld from Current as today.

**The effect of this phasing.** Real piping models almost always carry weight and elbows. W1a therefore repairs the N05 class and the frozen references, not most real models. W1b and W1c are required before "general accuracy" can be claimed for typical models (§8).

**References.** R1's brief covers nodal loads only. W1b needs an R1 addendum (RF-ELOAD) with independent references for uniform, thermal and thrust loads before it is implemented (§9, D-11).

### 4.3 When the method runs

- **Routes.** The ordinary route (models 0.1.0 and 0.2.0, and 0.3.0 `legacy_pressure_v1` with zero pressure) and the exact route (0.3.0 exact; W1a requires an explicitly empty pressure-region list, as physics-source-1 does). The 0.4.0 load-state route follows after T1 merges (W1b).
- **Invocation digest (revision 3, S5-R).** A W1 attempt requires D2's S-H `digest_ok()`: the captured invocation must have a representable digest. Otherwise the case gets `RETAINED_PRECISION_UNAVAILABLE` with reason `invocation_not_representable`, keeps its ordinary outcome and publishes ordinarily. D2's G4 admits that reason (D2 I-7, closed).
- **Trigger**, per case: a captured invocation with `digest_ok()` exists, and the ordinary attempt ended in one of:
  - `SolveQuality::Sensitive`;
  - `NumericallyUnresolved` for an absorbed contribution, assembly amplification, a condition estimate at the working boundary, a failed intended action or original residual, or an unresolved pivot;
  - a `Range` error that W2 scaling could not resolve.
- **Never triggered by** `Mechanism`, `Asymmetric` or `InvalidInput`.
- **`NegativeEnergy` (revision 2, V1-N2).**
  - The ordinary witness is verified against the stored binary64 matrix (`FK/structural.rs:1287-1327`, `:580-587`). For skewed or bending-soft models that matrix can be indefinite while the primitive model is positive definite (§3.1).
  - For a case in the supported family, a `NegativeEnergy` outcome therefore triggers the method, and the ordinary witness is kept as evidence.
  - The method's own result decides: selected if it passes at p, unresolved otherwise. `NUMERICAL_INTEGRITY_NEGATIVE_ENERGY` stays only when a negative direction is also verified at p against the primitive model. Frames and positive springs have non-negative energy by construction, so no such direction exists in the supported family.
  - Outside the family, the ordinary `NegativeEnergy` outcome stands.
- **The S11 load audit (S11-F).** A case marked Sensitive because a load contribution was absorbed is an ordinary Sensitive case and triggers the method in the same way. The method's ledger is exact, so the loss does not recur.
- **Passed cases: the D-5 trigger (revision 5; ROOT pre-accepted O1, fixed per D5C-1).** The exact-residual formation check of §4.3.1. Before F2a it demotes Passed to Sensitive in K-D5; from F2a it routes the case to W1 (subject to the coexistence rule, §4.4).
- **The ordinary attempt always runs first.** Its M03 report is kept as evidence, in the same place `OrdinaryAttempt` sits today (`PP:2372-2375`).

#### 4.3.1 The D-5 formation check (revision 5; replaces `D5_TRIGGER.md` §3's (a2) rule, §5 and §9)

**Why it exists.** Main calls a case Passed when rcond ≥ √eps (cond ≤ 6.7e7). In that band a case can breach 1e-9: P1's RF-SKEW-T-CANT-OFF-122-r1e-04 (2.43 dense, 1.21 sparse). **ROOT's recorded correction (D5_CHECK ruling 2):** the Passed-band breach class on main is broad in synthetic space. It covers axis-aligned soft springs partly absorbed by a stiff diagonal (V1: 230 Passed breaches) and pure solve error (V1: 105), up to about 4e-9 relative, with cond from about 1e6 to 6.7e7. It is not a single case. The no-interim ruling stands (the committed fixtures are clean, and the realistic models are at 1.5e-13 or better).

**The estimate (D5C-1, V1's fix, adopted by ROOT).** After the ordinary attempt passes its residual gate and would publish Passed:
1. **The intended system.** K_int is:
   - every straight frame element re-formed from its binary64 primitives (end coordinates, y reference, E, G, A, I_y, I_z, J) in `Wide<2>` at p = 128. The frame, L, the local coefficients (EA/L, GJ/L, 12EI/L³, 6EI/L², 4EI/L, 2EI/L) and Tᵀ K T are all formed at p. Nothing is shared with binary64 `local_stiffness` or its rounded frame, because shared binary64 local coefficients bias EF low (V1, D5_CHECK §3);
   - the ground springs, as their exact binary64 values;
   - on the prescribed route, the coupling K_fc·u_c.
2. **The residual.** For every free row i, ρ_i = f_i − Σ_j K_int[i][j]·u_j is **one `ExactAccumulator` sum** (S11-K's `FK/exact_sum.rs`), rounded once.
   - f_i enters as its terms: the `AssembledForce` ledger terms where the caller supplies them (after S11-F), otherwise the folded force the kernel received.
   - Each `Wide<2>` coefficient enters as its exact split into at most three binary64 terms, each taken through `add_product` with the binary64 u_j. The split is exact unless a piece falls below 2^-1074; in that case its truncation, below 2^-1074·|u_j|, is added to a per-row absolute allowance and the row's EF is widened by it.
   - Neither the binary64 published residual nor element-level ΔK is used.
   - This extends the FK intended-action audit path (`contribution_sums` and `audit_intended_action`, `FK/structural.rs:395-411`, `:513-571`). That path already forms exact sums of binary64 contributions times u; K-D5 replaces the binary64 frame contributions with their `Wide<2>` re-formation.
3. **The correction.** w = K̃⁻¹ρ, with the same mode's existing factor (one solve pair, in scaled variables as refinement does).
4. **The rule.** Let q_i be each published free nodal translation and rotation, and S\*_kind the coupled body scale of §4.1.6.1 items 4–6 (translation and rotation), formed from the binary64 u. The case is routed when, for any i:
   - **2·|w_i| > 1e-9·max(|q_i|, S\*_kind)**; or
   - max(|q_i|, S\*_kind) = 0 and w_i ≠ 0. That is a body published as unmoving while the intended solution moves; the G = 1e80 fold cases show it.

**Calibration on the product path (D5C-4).** `recal_d5.py` imports V1's product-faithful emulation (`probe_d5_check.py`: product normalize, two-stage transform, `+=` springs, radix scaling with the symmetric average, dense Cholesky or RCM profile LDLᵀ, `estimate_rcond`, residual gate and refinement). It runs 160 R1 cases (at most 12 members) in both modes: 194 Passed, 109 Sensitive, 17 failed or unresolved case-modes.

| Measure | Result |
|---|---|
| EF against the actual error, rows with actual ratio 0.1 to 1e3 (99 case-modes) | **0.999989 to 1.000011** |
| Passed breaches (actual > 1) | 28 case-modes: 122-r1e-04 (dense 1.586, sparse 2.042 with this y_ref) and 26 RF-CANCEL fold cases (S11) |
| Misses at factor 1, 2 or 8, folded f | only the 26 RF-CANCEL case-modes, S11's named exceptions |
| Misses with ledger terms (after S11-F) | none at a finite scale. The eight G = 1e80 case-modes (the probe solves with the folded force, which is 0, so u = 0 and S\* = 0 while w ≠ 0; after S11-F the product solves with the exact force and they do not breach) are caught only by the zero-scale clause, which the probe does not evaluate. The captured route refuses them at capture anyway |
| False positives in the Passed band, folded f | factor 1: 0; **factor 2: 0**; factor 8: 3 (RF-CHAIN-T-n03-r1e-06 dense 0.237, RF-CHAIN-A-n03-r1e-06 dense 0.380, RF-INVARIANCE-LFRAME-RELABEL dense 0.198) |
| (a1) cond·u trigger, false positives in the Passed band | c = 1: 16; c = 8: 28. It misses the same 26 RF-CANCEL case-modes |
| Uncoupled scale S(kind) instead of S\* | 6 false positives at factor 1 (structurally zero kinds whose largest value is noise), so S\* stays |

- **The factor 2.** EF equals the first-order error to about 1e-5 relative in the band; V1 reports a minimum EF/actual of 0.99999993 on its 230-case sweep, and every one of its 105 solve-error breaches caught. So the factor only has to cover the difference between the kernel's S\* and R1's class scales, and the second-order terms, which are ≲ 6.7e7·n·u ≪ 1. A factor of 2 does that with no false positive in the emulation. At 8, it would demote three correct cases with actual 0.20 to 0.38.
- **Product data (ROOT's condition).** P1's 122 case has actual 2.43 dense and 1.21 sparse, and EF ≈ actual, so 2·EF > 1 in both modes. V1's sweeps have every breach at actual > 1. The required true positive therefore holds by construction, and K-D5's tests assert it on the product.
- **Limits.** This is an emulation, not a product run. The product constant for 122 depends on y_ref: P1's adapter used (1,0,0) or (0,1,−1), which gives 2.413 and 1.214. The emulation here uses (0,0,1) for non-vertical members (D1's R1 adapter rule), which gives 1.586 and 2.042. D1's textbook-LDLᵀ figures for 122 (0.3–0.65) are not the product's (N-4).

**Stiffness EF cannot re-form (D5C-2; ROOT: demote, never silent).** SA knows which contributions are straight frames, user stiffness elements (including expansion-joint stiffness) and curved bend macro-elements (`AssemblyEvidence::new`, `SA:25-143`). If a case contains any user or curved contribution, EF is not formed. A case that would publish Passed is demoted to Sensitive, with the formation-check reason `formation_check_unavailable` naming the family. This holds until each family gains a `Wide` re-formation (curved with W1c; user elements when their primitives are defined).
- **Over-demotion on committed fixtures: none.**
  - No committed request that solves on the merged tree realizes a curved bend or a user-stiffness element.
  - The only committed bends are `mechanics_geometry_only`, so they are straight chords. Only the portable `results/invented/result_export_v0_2.json` snapshot names `curved_bend_macro_element`, and it is not a solve fixture.
  - The ordinary route already refuses a realized user-stiffness joint (`JOINT_ELEMENT_EQUILIBRIUM_UNQUALIFIED`, T0R M07). That is why the shared test basis drops the demo joint (`PP` `fn request()`).
  - The committed envelopes `invented_mechanics_result*.json` (four user-stiffness macro-elements, Passed) are historical outputs, and no committed test regenerates their bytes. The one test that reads them compares metadata only.
  - P1's survey of committed Passed cases (32 solved runs) contains no such contribution.
- **Tests that change:** `preview_physics_runtime.rs` `b1_arc_signed_rows_frames_and_withheld_maximum` and `b2_indeterminate_arc_resultants_depend_on_k`, which build DEC-070 curved-arc models. Possibly also cases in the `validation/benchmarks/mechanics` and `nonlinear` crates that build user or curved elements. K-D5 lists each changed outcome as a declared expected change, with its reason, before merging.

**The evidence record (D5C-3).** `StructuralReport` derives `Debug` and is rendered into committed raws (`PP:1057`), so EF is **not** a field of it.
- K-D5 adds a separate `FormationCheck` record (the demoting row, 2·|w_i|, the scale, the ratio, and the reason: `estimate` or `formation_check_unavailable`) as an `Option` on `StructuralSolution`, set only when the check demotes. `StructuralSolution` is not `Debug`-rendered anywhere in `PP` or `SA` (checked).
- A demoted report differs from today's only in `quality: Sensitive`.
- **Byte-identity test:** every committed raw's integrity text, and every existing suite, is unchanged when nothing demotes.
- F1 renders the record as one evidence line in the integrity diagnostic, only when present, as W2's b is.
- Note: a demoted case enters the existing Sensitive path, including exact-block recovery for in-scope signed-permutation cases (`PP:2382-2391`). That is today's behaviour for Sensitive cases, and it publishes correct values where it selects.

**What EF does not see (N-1, N-2), and where each class goes:**
- **Load formation.** Before S11-F the kernel receives the folded force, so a load-fold error is invisible to EF. That class is S11's, carried as the named exceptions (§4.10) until S11-F. After S11-F, ρ uses the ledger terms, and the right-hand side is itself exact.
- **Member actions and reactions.** They are recovered in binary64 from u. For the formation class the rigid mode carries no force, but recovery cancellation is a separate class that EF does not see. It routes to W1's recovery before rounding (§4.1.5) from F2a, and meanwhile to the backstop gate's member and reaction comparisons (§4.10). **P1's survey is asked to add member and reaction comparisons for Passed-band cases.**
- **Nonlinear and pressure-region routes.** These are outside K-D5's scope, as they are outside W1a's.

**Realistic models (N-3).** With the product's estimator (V1's probe B), M11 (the skew line) has cond 2.6e6 and M9 3.2e5, and their actual errors are ≤ 1.5e-13 relative. EF does not fire on them. (a1) at c = 1 (cond > 9.0e6) does not demote M11; at c = 8 (cond > 1.13e6) it does. D1's `cond_realistic.py` figures come from a textbook estimator and are superseded where V1 recomputed them.

**Cost.** One `Wide<2>` formation pass over the members (O(members)) and one exact residual sum per free row (O(nnz) exact products), plus one solve pair with the existing factor: about one refinement step. It runs only when the ordinary attempt would publish Passed.

### 4.4 Identities and what happens to existing ones

Proposals only; ROOT reserves names and versions.

| Identity | After D1 |
|---|---|
| `preview-physics-1` | Byte-unchanged for every envelope in which no case selects the new method |
| `<preview-retained>`, suggested `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Ordinary route, emitted when at least one case selects the method. It inherits the preview-physics-1 table and adds a closed per-case receipt (§5). Other cases are rendered under preview-physics-1 semantics, with their ordinary standing |
| `<physics-retained>`, suggested `.../physics-retained-1` | The exact-route equivalent, inheriting physics-1 semantics |
| `source-blocks-1`, `physics-source-1` | Readers, fixtures and hashes unchanged. **Recommended (D-4 option A), ordered per D-15 (revision 5):** exact-block selection stays the selected method for its domain until **F2b** retires it there, and F2b runs for a family only once the retirement gate (§4.4.1, row-level condition 3) passes, which on the committed families requires C (§8.1). Until then the coexistence rule below applies. After retirement the exact-block method stays a kernel test oracle: in its scope, the new method's rows must match its projections within the unchanged criterion |
| `load-reference-source-1` (T1) | **As ruled (R-3(a)).** It stays fresh until F3 (W1b, including the 0.4.0 load states) lands, and it stops being fresh at F3, when the gate passes for it. Joined results stay `needs_recompute` until then. D2's S-E1 is built with F3, and S-E2 only if F3 will not land within T3. The 0.4.0 successor is `<load-reference-retained>` (D2 §4.9.1, S-G2) |
| Historical physics-source-1 | **As ruled (R-3(b)):** stays eligible after fresh retirement, through its existing reader. A defect found later reopens this |
| Historical all-selected source-blocks-1 | **As ruled (R-7 (i)):** stays Current, with the notice and the summary rule-binding refusal. Retirement for fresh solves does not change it |

**Coexistence rule (revision 5, R4-2; ROOT).** While exact-block selection is retained for a domain, **W1 is not attempted in any invocation in which exact-block selects a case.** Such an invocation publishes exactly as today: the same identity, `numerical_quality`, diagnostics and bytes. The case is decided per invocation after the ordinary attempts and exact-block's own selection, which are unchanged. An invocation in which exact-block selects no case may attempt W1 for its eligible cases (§4.3), and publishes under the successor identity if one is selected. So one envelope never carries both `SOURCE_BLOCK_RECOVERY_SELECTED` and `RETAINED_PRECISION_SELECTED`, which keeps D2's G4. A case exact-block leaves unrecovered in an exact-block-selected invocation keeps today's outcome; W1's recovery of it (F-P2's half) waits for that domain's F2b.

Under option A, after retirement, a fresh invocation never mixes two selected methods. The ordinary cases in an extended envelope carry repaired preview-physics-1 semantics, not precision-1 semantics. That removes, for fresh solves, the reason behind T0R's `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`. D2 owns that reader decision.

**Option B.** Keep exact-block first and the new method for the rest. It needs a mixed-method identity and two receipt families in one envelope. I do not recommend it.

#### 4.4.1 The shared retirement gate (revision 2, V1-S2; adopted by ROOT, defined here, cited by D2)

**Where it applies.** An identity family's exact-block selection is retired for fresh solves only when all four conditions hold on the actual candidate. The families and their slices:
- source-blocks-1 at F2b, with D2's S-F;
- physics-source-1 at F2b;
- load-reference-source-1 at F3's retirement step.

(Revision 5: source-blocks-1 also retires at F2b, not F2a. F2a wires W1 and the identities and retires nothing.)

**The conditions.**
1. **Coverage.** Take every committed request under `P/fixtures/product_preview/source_blocks/` and `P/fixtures/product_preview/physics_source/`. For F3, also take every committed joined `load-reference-source-1` request (T1's `load_reference_states` carriers and fixtures). Each is solved fresh in both modes, and every case that exact-block selects today must be selected by the new method. A case outside W1's coverage fails the gate, and the family is not retired. (D2's H-a subset concerns standing, not selection; it is handled under condition 3.)
2. **Budgets.** Those cases complete within the D-8 limits ROOT selects from measurement. The per-case and per-invocation charges are recorded.
3. **Standing (revision 3, ROOT's final wording for S2-R; revision 5, row-level per ROOT's D-15 and R4-2).** "The successor identity's standing is no worse than the retiring identity's, case by case, in all three languages." **Row level:** for every case, the successor's withheld rows (those D2's `classification_summary` counts as withheld: `not_covered`, and `absolute_verified` rows that neither C nor a verified B makes bindable) number no more than the retiring identity's withheld rows, which are zero where it publishes the case as Current. The gate report shows **both counts side by side per case**, per language. A successor count above the retiring count fails the condition for the family. In addition, each language's standing is identical and fail-closed across the whole family, through D2's S-G readers and the shared case files (D2 §4.7). The condition does not require every case to become eligible. A declared out-of-scope subset reads `needs_recompute` under both identities, and passes.
   - **Listed out-of-scope subset: D2's H-a log-law cases** (D2 §4.2.1, §4.9.5). A 0.4.0 case in which any member uses a definition whose value needs host `exp` or `exp_m1` (today only the `logarithmic_per_current_length` law) is `needs_recompute` under the joined identity `load-reference-source-1` and under its successor `<load-reference-retained>`, in all three languages. The gate records each such case by request and case id.
   - For source-blocks-1, an all-selected envelope is Current today, so the successor result for the same request must be Current too; a mixed envelope is `needs_recompute` today, so any fail-closed successor standing passes (D2 §4.5.2).
   - **The joined family switches at F3**, as R-3(a) ruled.
4. **Values.** Every published quantity that the exact-block projection also publishes agrees with it within the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. The scale is the case's R1-style zero scale, or, for these fixtures, the body-level coupled scale stated in the test. Signed six-component reactions and circular maxima, which exact-block's source-blocks-1 rows do not carry, are checked against T0R's preview-physics-1 rules instead.

**Withheld rows (revision 4, R3B-2; revision 5: a pass condition, not information).** The side-by-side per-case counts are condition 3's row-level test. §8.1 gives the committed figures before and after C and B. On the committed families every domain has at least one case that fails the condition without C, so F2b follows D2's C slices (§6).

**Evidence.** The gate runs as one committed test per family in `numerical_robustness` (product lane), plus D2's S-G parity files. Its record lists every request, mode, case, method, charge and standing. A gate failure blocks retirement for that family only; the other families and W1's selection are unaffected.

### 4.5 Interaction with M03-INTEGRITY-v1 and M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1

- **The ordinary attempt** is judged by M03-INTEGRITY-v1, unchanged, in binary64. Its report stays as evidence.
- **The selected extended state** is judged by the proposed `M03-INTEGRITY-MP-v1`: the same structure with `u_p = 2^-p`, the precision-specific pivot and condition screens, residuals at p + 64, and the stop rule. A binary64 rcond screen is never applied to a p-bit factor, and no factor is relabelled.
- **Selected-state equilibrium.** M03-PRODUCT-PREVIEW-EQUILIBRIUM-v1's componentwise measurement is evaluated on the retained state against the intended system, with `γ_p`, and published with the basis label `retained_precision_p`. ROOT either registers it as that policy's precision-aware reading or gives it a successor id; that is within its existing authority (`CONTRIBUTION_PRECISION/SOURCE_QUALIFICATION.md`, "Reliance boundary").
- **Public projection residual.** The residual of the published binary64 displacements on the represented binary64 K is published as a separately labelled observation, not a gate. It can legitimately be worse. For N05 the represented K has the wrong k.
- **Claims.** No certified inertia, forward-error bound or physical accuracy is claimed. Historical DEC-046, DEC-050 and DEC-053 records, and the 64γ binary64 policy, are unchanged.
- **Where each outcome is recorded (revision 2, D2's IF-1).**
  - `numerical_quality.cases[i]` keeps the ordinary attempt's M03-INTEGRITY-v1 outcome for every case, selected ones included.
  - The precision-p outcome ("checks passed at precision p; 2p agreement verified") is recorded only in the receipt.
  - The producer never writes `checks_passed` into `numerical_quality` for a selected case, so D2's DD-11 does not arise, and it is closed (revision 3). Readers derive a selected case's standing only from the verified receipt (D2 §4.9.4).

### 4.6 Boundary with T5

- **D1 supplies** a passive linear kernel API: for a given selected active state (a fixed restraint set and springs), solve at p and recover.
- **D1 does not** classify contact, decide gap signs at precision, iterate active sets, handle friction, or qualify the mixed recovery basis. `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` and the strict-gap law stay with T5 (M06).
- **At the facade,** D1 refuses every model with a nonlinear support record.
- **The nonlinear loop.** W3 converts the loop's base assembly and linearized solves to the sparse representation, without changing their semantics. The friction influence solves are forced to dense scrutiny today (`nonlinear_integration/src/lib.rs:1336-1342`); changing that is T5's (SUP-16).

### 4.7 W2 — range

**Mechanism: exact force-radix scaling of the ordinary system, in the kernel.**
- `AssemblyEvidence` gains a private `force_scale_exponent: i32`.
- When b ≠ 0, local stiffness is scaled by 2^b before `transform_roundoff`, and `solve()` scales K and f by 2^b.
- Every scaling is exact because it maps normal numbers to normal numbers. Displacements are unchanged. Actions, reactions and residual records are unscaled exactly for publication, with the outcomes defined below.
- Every M03 screen is componentwise-relative, so the scaled evidence is equivalent.
- **Revision 2 (V1-S7):** b is **not** a field of `StructuralReport`. The `Debug`-published report is therefore byte-identical for every current result. When b ≠ 0, the facade publishes one extra evidence line in the integrity diagnostic: `range_scaling: force_scale_exponent=<b>; basis=exact power-of-two`. It appears only in cases that are refused on main today, so no committed byte changes.

**Choosing b (normative, revision 2).**
1. **Evaluate with b = 0**, exactly as today. If the evaluation completes without a `StructuralError::Range`, then b = 0 and nothing else in this section applies.
2. **Only if step 1 fails with `Range`**, collect the nonzero values: every local element stiffness entry, every spring stiffness and every load component.
   - If any of them is subnormal, refuse with the reason "range: subnormal stiffness or load at formation". Its bits were lost before the kernel.
   - Otherwise let `e_min` and `e_max` be the least and greatest binary exponents (`binary_exponent`, `FK/structural.rs:181-189`).
3. **Feasible window.**
   - Let `b_lo = −1022 + 64 − e_min`, the allowance headroom: γ(48) ≈ 2^-47 plus operation growth.
   - Let `b_hi = 1023 − 8 − e_max`.
   - If `b_lo > b_hi`, refuse with the reason "range: exponent span [e_min, e_max] exceeds the binary64 normal window after exact power-of-two scaling".
   - Otherwise `b = ⌊(b_lo + b_hi)/2⌋`, with floor division toward −∞.
4. **Evaluate once with that b.** If this second evaluation also fails with `Range`, refuse with the reason "range: scaled evaluation outside normal range". There is no third attempt.
5. **Unscaling for publication.** Each published action, reaction and residual record is multiplied by 2^-b in bounded exact steps.
   - A normal result is exact.
   - A subnormal result is published with the representability outcome `subnormal`, with its relative precision stated in the evidence line (§5 item 6).
   - A nonzero result that would underflow to zero, or overflow, makes the case `NUMERICAL_INTEGRITY_UNRESOLVED` with the reason "range: publication outside binary64". It is never flushed.
   - Displacements are never scaled.

**Admitted range (ordinary path).** A feasible b exists when every nonzero stiffness entry, spring stiffness and load component is normal binary64 in SI units, and their joint exponent span is at most about 1,980 bits. The displacements must also be normal or exactly zero, since force scaling does not change them.

**Refusal.** The case stays `NUMERICAL_INTEGRITY_UNRESOLVED` with a precise reason, for example: "range: exponent span of stiffness [e_min, e_max] and loads [·,·] exceeds the binary64 normal window after exact power-of-two scaling". The global DOF map is included where it applies. No new diagnostic code is added, so readers need not change.

**PHYS-R4.** By my arithmetic, b ≈ 500 puts EA/L near 3e-3, GJ/L near 3.5e-157, and the `[UY,UY]` allowance near 2.5e-170. All are normal. The public fixture then passes the evidence stage, and the free-DOF set is empty. The published stress path was already repaired (`membrane_publication_range.rs`). The detection run P1 records the actual end-to-end result.

**The extended path** has an internal 64-bit exponent, so range stops mattering inside it. Only publication can refuse: a nonzero quantity that underflows or overflows binary64 is explicit, and a subnormal result carries its reduced relative precision. See the D2 note (§5).

**Formation range (revision 2, from R1's finding 3: `RF-RANGE-…-LEF-small` and `…-LEF-large`).**
- In these cases every input and output is normal binary64, but the product GJ (or EI) is not: it is 2^-1078 and 2^1122 respectively.
- On main, `FK/lib.rs:717-726` forms each coefficient in binary64 (`g * j / length`, `12.0 * e * iy / length3`).
  - An overflow becomes ∞. `validate_named_finite_slice` then fails, and the whole envelope is blocked with a non-finite error.
  - **An underflow becomes an exact 0.** That passes the finiteness check and silently removes that member's torsion or bending stiffness.
  - Downstream that surfaces as a zero diagonal (unresolved), a spurious mechanism witness, or, when a parallel path carries the load, a silently wrong answer. It needs moduli near 1e-300 Pa, so its realistic reach is nil. It is still a silent path, and I record it as a finding for the manager.
- **Revision 2 changes formation in two steps.**
  1. **Checked formation (K2a, `FK/lib.rs`, T1-disjoint, live; its own slice, after K-D5 in revision 5).** `local_stiffness` forms each coefficient with checked operations, **checking every intermediate product and quotient, not only the final coefficient** (revision 3; V1's S11B-9 and FORM-C). Any intermediate of nonzero finite operands that is zero, subnormal or non-finite is a new `FrameKernelError::NumericalRange { name }`, never a zero or imprecise coefficient. That catches partial underflow too: the coefficients underflow at different thresholds (12EI/L³ against 4EI/L), which would otherwise change the element's stiffness relations, not only its scale. The one exhaustive match outside `FK`, in `P/core/solver/diagnostics/src/lib.rs:285-362`, gains its mapping in the same slice. On main this turns the silent zero into a refusal; the case is blocked as any formation error is today.
  2. **Scaling at formation (revision 3, SCALE-W: the kernel half is in K2b's write set, `FK/lib.rs` scaled formation and the sparse assembly entry that takes b; F1 wires it in the facade).** When checked formation raises `NumericalRange`, the b-selection above runs on the predicted exponent of each coefficient: the sum of its operand exponents, which is exact to within ±1 and needs no product to be formed. The kernel then forms the elements with E, G, spring stiffnesses and load contributions scaled by 2^b. Every scaled operand and coefficient is normal, and each scaling is exact.
- **Result for R1's cases.** LEF-small and LEF-large are then **solved** on the ordinary path, and W1 solves them anyway for supported families, because its exponent is 64-bit. **A named range refusal** ("range: exponent span … exceeds the binary64 normal window …") remains only when no single b fits every coefficient and load together. That is a refusal of a representable problem, and VP-ROBUST records it as a failure of that case, not a pass.
- Until F1 lands, checked formation refuses these two cases with its named reason. P1 records main's behaviour.
- **Reach of the silent zero on main (ROOT's question).**
  - **R1's RF-RANGE vectors.** Only the vector −(200, 300, 600) (LEF-small) is affected. There E·I and G·J are about 2^-1078, below the smallest subnormal. **Corrected in revision 3 (V1's FORM-C, probe `REVIEW/_run_records/r2_backcheck/probe_r2_formation.*`):** 6EI/L², 4EI/L, 2EI/L and GJ/L round to exactly 0, but **12EI/L³ is a normal value 3.5 % wrong**, because its intermediate `12.0 * e * iy` is the least subnormal (5e-324) before the division by the tiny L³. The element is therefore inconsistent, not simply stiffness-free. This is not a 1e-9 question. The published result is an unresolved zero diagonal, a spurious mechanism witness, or, when another path carries the load, a value wrong by far more than 1e-9 (P1 records which). K2a's intermediate check refuses it. The mirror vector +(200, 300, 600) (LEF-large) overflows to a blocked envelope instead, which is loud, not silent. Every other RF-RANGE vector keeps every coefficient normal: for example (0, −1000, 0) gives E·I about 2^-977 and G·J about 2^-978, and ±(−120, 500, 260) gives E·I of 2^42 and 2^2.
  - **Subnormal coefficients.** These are not zero but lose relative precision (a coefficient near 2^-1050 keeps about 24 bits, so about 6e-8). They exceed 1e-9 in the same way, and checked formation refuses them as well.
  - **Realistic models.** No case exists. A zero or subnormal coefficient needs a section stiffness product E·I, G·J or E·A (divided by at most L³) below about 2.2e-308 in SI units, about 300 orders of magnitude below any physical pipe section. Realistic coefficients lie between about 1e-3 and 1e15.
  - **Conclusion.** The path is silent but unreachable in realistic models. In R1's LEF-small it is a total loss of stiffness, not a small error.

**What W2 does not cover:** length-unit scaling, which would need facade formation in scaled units, and cross-unit display (D2).

**The capture boundary (revision 2, V1-S5, ROOT ruling 4).**
- Today, any request containing a finite `|x| ≥ 2^53` is refused at invocation capture, before W2 or W1 can act, on every route and on T1 as well. An example is a 1e16 N/m "rigid" spring.
- D2 designs the capture fix (R-6). T3 owns it as part of M34 range.
- Once capture admits such values, W2 handles them without change: a 1e16 N/m spring beside N-series stiffness spans about 40 binary exponents.
- W1 handles them exactly: the spring is an exact binary64 input.
- VP-ROBUST adds capture-boundary cases to RF-RANGE (§4.10). A request value of exactly 2^53 − 1, one of 2^53, and one of 1e16 must each give the outcome of the capture design selected at the time: a refusal code today, a solve after the fix.

### 4.8 W3 — sparse assembly, reduction and reactions (M32)

**One representation.** A kernel `SparsePattern` built from element connectivity (12×12 blocks), springs, and user and curved blocks. Values are accumulated in the same element order as today's dense assembly, so every coalesced entry is bit-identical to the dense entry. That makes parity of the represented equations exact.

**The same gate in both modes, rewritten over the pattern.**
- `validate`: symmetry over pattern pairs.
- `prepare`: scaled values and `K_fc u_c` from sparse columns.
- Contribution audit: Expansions per pattern entry, so memory is O(nnz).
- Residual and intended action: sparse rows.
- Pivot screens: the existing profile factor, now built from entries with `SymmetricProfileMatrix::from_entries_with_order` and ordered by `adjacency_from_symmetric_entries`.
- `rcond`: norm from the pattern.
- `negative_pair_witness` and `verify_negative_direction`: pattern pairs only, O(nnz).

The dense `StructuralSystem` API stays, unchanged, for auxiliary callers and for dense scrutiny.

**Modes.**
- **Sparse interactive**, the default, uses only the pattern.
- **Dense scrutiny** materializes a dense view from the same values and keeps today's dense Cholesky, its labels (`dense_structural_integrity_primary`), and the protected DEC-050/053 legacy LU observation (`PP:2520-2535`).
- **Resource guard.** Dense scrutiny refuses above a declared ceiling, with a blocking `SOLVER_SYSTEM_BLOCKED` naming the estimated bytes. Sparse refuses above a profile ceiling. ROOT picks both from measurement.
- **No automatic fallback.** A sparse integrity outcome is never rescued (NP-07). A resource refusal is explicit, and dense needs more memory anyway. Mode code 3 stays reserved and unused, as today.

**Facade after T1 merges.**
- `PP:1620` and `:1751`: sparse assembly per modulus basis.
- `PP:2330-2342`: partition maps.
- `PP:2697`: reactions from sparse rows.
- `solve_preview_reduced_system` takes the pattern.
- `source_recovery` gets a dense view for n ≤ 256 only, if option B were chosen.
- The nonlinear loop moves to sparse (§4.6).

**Parity protocol.** No new tolerance.
1. Bitwise equality of the coalesced K between pattern and dense assembly, for every fixture.
2. M03 outcome class parity between modes on N01–N09, R01–R07, NP-B, NP-D, the T0R references and the R1 families. A divergence near a screen boundary is recorded, never tuned.
3. Published quantities agree within the existing DEC-053 parity basis: 1e-9 relative, scaled by the dense magnitude (`performance_harness/README.md`).
4. A model with two modulus bases, each case on its own pattern.
5. Nonlinear gap, one-way and friction models: the same final active state, and quantities within the DEC-053 basis.
6. Relabelling and permutation give the same answers against the references.

**Memory and runtime protocol.**
- **Sealed models.** R1's RF-LARGE families (10, 100, 1,000 and 10,000 members; determinate and indeterminate; axis-aligned and rotated) and the nine DEC-053 observations. Each is generated deterministically into a product request, and the request's sha256 is committed.
- **One fresh process per model and mode**, run by a standard-library Python runner. It records peak RSS from `/usr/bin/time -v` (Linux) or `-l` (macOS).
  - **Host protection (revision 2, V1-N6).** On Linux the runner applies `RLIMIT_AS` through `resource.setrlimit`. **macOS does not enforce `RLIMIT_AS`.** There the runner polls the child's RSS every 100 ms with `ps -o rss= -p <pid>` and kills it above the cap, recording `killed_by_rss_watchdog` with the last reading.
  - The watchdog is coarser than a hard limit. Runs on the owner's Mac therefore start at sizes whose Linux peak is known to fit the cap with a factor of two to spare. Stage timings (assembly, audit, factor, rcond, solve, residual, recovery) are printed by the binary as JSONL: five repeats, median and minimum.
- **Deterministic storage counts:** pattern nnz, profile entries, contributions, and for W1, limbs per entry.
- **Hardware metadata, toolchain and release profile** are recorded.
- **Observations only**, as DEC-053 did. The single claim is the before-and-after growth: peak memory in sparse mode grows about linearly with n on chains, against about n² before. It is stated as an observed fit, not a threshold.

**Homes.**
- `P/core/solver/performance_harness/`, disjoint: kernel-level sparse and dense observations.
- `P/validation/benchmarks/numerical_robustness/`, new: product-level runs through the public entry.

### 4.9 W4 — M03 residual items

**The rigid-null witness for user and curved bodies** (`SA:208-217`, `SA:264`; `FK/rigid_body.rs`).
- **New function.** `assess_constrained_bodies(sub_bodies, ties, grounds)` sits beside `assess_rigid_body`.
  - **Objective sub-bodies** are node sets joined by straight frames, and by curved elements that pass the screen below. Each has six rigid parameters (t, θ); node motion is `u = t + θ×(x − o)`, rotation θ.
  - **Ties.** Each user element imposes `u_a = u_b` and `θ_a = θ_b`. Its energy is zero only for equal nodal motion, because every stiffness is positive.
  - **Grounds** are the restrained DOFs and positive springs, as today.
  - **Rank.** The null space of the stacked map uses the existing SVD rank screen and τ_B form. It returns Restrained, MechanismWitnessed (with the direction mapped to nodes), or NumericallyUnresolved.
- **Proof sketch.** Each family's energy is non-negative, and its zero set is the stated linear space: rigid motions for frames and screened curved elements, equal motion for user elements. So total energy is zero exactly on the intersection with the grounds. This extends the existing welded-frame argument.
- **Curved objectivity screen.** For the six rigid vectors r_k at the element's nodes, `|K_e r_k|` must lie within the formation allowance from `curved_formation` (`SA:291-402`). If it passes, the element joins the objective sub-body. If it fails, the body stays unqualified for a witness, with a reason, and the matrix gate still runs.
- **Coordination.** T4 should confirm the curved construction's null-space claim.

**SUP-17** (`PP:1604`). Proposed text: "fewer than six independent ground constraints including positive springs: the six rigid-body modes of a connected structure cannot all be removed; directly restrained global DOF classes: {restrained}; global DOF classes with no direct restraint: {missing} (not a rigid-body mode analysis; separated restraints can resist rotations); support contributions: …". Update the test at `PP:20615` and record the change. The line sits outside T1's hunks, but in T1's file, so it lands after the merge.

### 4.10 W5 — the VP-ROBUST harness

**Crate** (new): `P/validation/benchmarks/numerical_robustness/`, with its own `Cargo.lock`, discovered by CI automatically.
- Dependencies: `product_physics`, `frame_kernel`, `sparse_direct`, and `serde_json` with `float_roundtrip`, the same set `numerical_integrity` uses.
- `cases/` holds adapters from R1's `references.json` to kernel `PrimitiveSource` values and to product requests. The adapter is reviewed code; R1's values are never edited.
- Examples, not tests, for the scale runs.

**Kernel lane (before T1 merges).**
- R1 families RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE and RF-MECH, through the kernel method, and through the binary64 sparse gate for the RF-MECH and RF-LARGE parity checks.
- **Limit.** Springs can lie along any direction at kernel level. The product supports only global-axis springs and restraints (`linear_supports/src/lib.rs:217-221`), so R1 cases with springs along a skewed axis run in the kernel lane only.

**Product lane (after T1 merges and the facade slices land).**
- Every authorable R1 case, through `run_linear_static_preview_value_with_mode` in both modes. Each variant runs as its own request.

**What is compared.**
- Global nodal displacements and rotations.
- The six signed reaction components per support (T0R v2 rows).
- Member invariants: axial force, torque and `hypot(My, Mz)` at the ends and stations.
- **Twist and extension (revision 3, F2), derived, never differenced.** The harness derives each member's twist as `T / k_t` and extension as `N / k_a` from the published torque T and axial force N, with `k_t = fl(fl(G·J)/L)` and `k_a = fl(fl(E·A)/L)` formed as `FK/lib.rs` forms the torsion and axial coefficients. They therefore inherit T's and N's verified accuracy, to within two further roundings (≤ 2.3e-16 relative). They are never computed as differences of published rotations or translations (θ_j − θ_i, u_j − u_i), whose binary64 cancellation would limit their relative accuracy regardless of the stop rule.
- Method evidence: the selected method, the precisions, the M03 class.
- RF-MECH must be refused with a witness or an unresolved status, and no rows. Any recovered answer fails.

**Predicate.** Every comparison uses `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with R1's stated zero scales. No new tolerance.

**R1's package (candidate at `6c448d260`, not yet frozen; revision 2).**
- **Represented basis.** Two cases are compared on R1's `expected_represented` values, not on the intended ones: `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6`. Their intended-input comparison fails through input rounding alone, which no solver can recover. So does every case whose `finite_input` field marks the represented basis.
- **Discriminating controls.** Mutation and stop-rule controls use only the negative controls R1 marks `discriminates`. For the soft families that means k/a from about 1e-8 down; the k/a ≈ 1e-4 cases are continuity controls.
- **Directional springs.** Cases flagged `needs_directional_spring` run in the kernel lane only. The product authors global-axis springs only (`linear_supports/src/lib.rs:217-221`).
- **RF-CANCEL scale.** RF-CANCEL is compared with R1's recommended net-governed scale, which ROOT ruled is **the binding comparison scale** (`ROOT_RULINGS_V2.md` §1). The criterion is unchanged; where the net scale falls below |exp| (84 `mixed` rows, V2 §3.1) the comparison is exactly relative. See `S11_CONTAINMENT.md` §5.2 for how it relates to the S11 row scale: under C3-full the acceptance does not depend on the row scale, which matters only on the guard path.
- **RF-RANGE.** LEF-small and LEF-large must be solved (§4.7). A named range refusal is recorded as a failure.
- **Capture-boundary cases (V1-S5)**: 2^53 − 1, 2^53 and 1e16 in a request (§4.7).

**The zero-scale floor check (V1-S8, F2).** For every reference comparison, the comparison scale `max(|exp|, scale)` must be at least `R·S*` of the case, with R = 2^-34 and S\* computed from the reference values by §4.1.6.1's kinds (twist and extension per member). A comparison below that is **not covered by the guarantee**.

**How the harness reports each comparison (revision 3, ROOT's F2 ruling).**

| Outcome | When | Counts as a pass |
|---|---|---|
| `pass` | Covered, and the predicate holds | yes |
| `fail` | Covered, and the predicate fails | no; blocks the gate |
| `not_covered` | The comparison scale is below R·S\*. The observed error is recorded, and so is whether the predicate held | **never**, whatever the observed error |
| `pass_absolute_range` / `fail` | F8: the expected value is below the binary64 range (RF-LARGE-CONT-n10000, 1e-714 to 1e-2864). It is parsed exactly from its decimal string, without underflow, and compared as an absolute comparison against its class scale | the pass is reported as an absolute-range pass, separately counted |

- **The gate.** VP-ROBUST passes when there is no `fail`, every discriminating negative control fails, and the `not_covered` set equals the enumerated list committed with the harness (below). A new `not_covered` comparison, or one that leaves the list, blocks the gate until ROOT reviews it. `not_covered` comparisons are never added to the pass count; the report shows passes, absolute-range passes and not-covered comparisons as three separate numbers.
- **"No Passed breach" (revision 4, D-5; revision 5, ROOT `b6fe1eb75` and D5C-5).** For every R1 case in the product lane, if any covered comparison fails, the case's published outcome must not be `Passed`.
  - **Both entries.** The gate runs every case through the captured entry (`run_linear_static_preview_value_with_mode`, `PP:1407`) and through the historical typed entry, as the headless runner reaches it (`run_preview_in_memory_mode`, `P/core/runner/headless/src/lib.rs:804`, into `run_linear_static_preview_with_mode`, `PP:1397`). A case the captured entry refuses at capture (G = 1e80, V1-S5) is not a pass there. It is checked on the typed entry.
  - **Named exceptions, as (entry, case, quantity) triples.** Until S11-F merges, the only admitted breaches are S11's. The committed list comes from P1's frozen-reference record, and a breach not in it fails the gate, even inside an RF-CANCEL case. The prediction from R1's frozen controls (`_run_records/s11_exceptions.py`, main folds in authored order) is:
    - **captured entry: 106 triples in 13 cases**: F and M at G = 1e7 and 1e8 in orders GnG and nGG (11 quantities each: seven member-moment and root-reaction rows, and the rotation RZ and translation UY of N1 and N2); F and M G1e8-GnG-ORTHO (6 and 5); F and M G1e8-GnG-INPLANE (3 member-moment rows each); and RF-CANCEL-UDL-W1e8 `th.S1.RZ`;
    - **typed entry: 168 triples in 22 cases**: the same 106, plus the nine G = 1e80 cases (F and M GnG, nGG, ORTHO and INPLANE, and UDL-W1e80).
    - **Count note.** ROOT's rulings speak of 12 captured-route breaches. P1's relayed early report also lists RF-CANCEL-UDL-W1e8 as a Passed breach (ratio 46.5), which makes 13. The committed list follows P1's record, and any difference from this prediction is reported, not absorbed.
  - **Removal.** A test pins the list to its committed source and requires it to be **empty once S11-F merges**. The exceptions are removed only when **both** entries are clean (ROOT).
  - **Negative controls:** mutation (23) with the D-5 trigger disabled, and a seeded non-S11 breach inside an RF-CANCEL case (for example, a formation-class perturbation), which must fail the gate although the case is named.
- **Relation to product standing.** A `not_covered` comparison's quantity is, in the product, `absolute_verified` or `not_covered` by the receipt's classification (§4.1.6 item 1), because its reference magnitude is below the floor. So it is withheld from reliance there too (§4.1.6 item 4). The harness checks this correspondence for every product-lane case, **comparing classes, not bits** (revision 4, N-3). Since revision 5 the harness forms both the derived twist `fl(T/k_t)` and its scale `fl(mo/k_t)` from the same receipt k_t, so the remaining differences are the product's own roundings, far inside the margin.
- **The enumerated list, from R1 revision 2 at `c0f14201c`** (`_run_records/floor_kinds.*`, variant F, which is this rule; identical at R = 10^9·2^-64 and at R = 2^-34):
  - **RF-WEAK: 46**: W-AX-rho1e-12 (29 far-region and 7 coupling-region comparisons), W-3D-rho1e-12 (2 far, 5 coupling and 2 body-class) and W-3D-rho1e-08 (1 coupling). These are V2's 43, plus the soft coupling member's twist and extension (`tw.C` and `ext.C` in W-AX-rho1e-12, `tw.C` in W-3D-rho1e-08), whose own torque and axial force sit below the floor;
  - **RF-CANCEL: 3**, under the binding net-governed scale: F-G1e80-GnG-ORTHO `u.N1.UY`, M-G1e80-GnG-ORTHO `u.N1.UY` and F-G1e80-GnG-INPLANE `Mb.M2.mid`;
  - **RF-SKEW: 2**, `tw.M1` in RF-SKEW-T-CANT-AX-122-r1e-12 and -345-r1e-12. Their comparison scale is their own magnitude (5.3e-12 and 1.7e-12 of S\*_tw), because the soft torque itself is about 1e-12 of the moment scale. The soft path stays covered through the rotations θ and the absolute torque comparison;
  - **of V2's 314 variant-A twist and extension flags, 303 are cleared** (293 of the 295 outside RF-WEAK; the two `tw.M1` rows stay, and 9 RF-WEAK rows stay as in variant B; corrected in revision 4, N-1). V2's variant B (twist and extension scaled by their own largest magnitude) gives 43 RF-WEAK and 0 RF-SKEW, but it is not a safe runtime rule: a structurally zero twist has S(twist) equal to noise at 2p, so the stop rule would never accept it. Variant F ties the scale to the published torque and axial force instead.

**Discrimination check.** Each of R1's negative-control values must fail the same predicate. A control that does not is reported as non-discriminating, not dropped.

**Seeded faults.**
- They sit behind `frame_kernel/mutation-controls`, `#[cfg(any(test, feature = …))]`, enabled only by this crate's mutation run. CI checks that no product manifest enables the feature.
- The kill matrix is recorded. Each fault must fail at least one comparison (§7.3).

**Memory and runtime.** The runner and records of §4.8, committed as JSON with hashes under `numerical_robustness/observations/`.

### 4.11 Build feasibility (facts)

| Backend | Latest (index, 2026-09-26) | Licence (crates.io) | Resolved non-optional dependencies | Declared `rust-version` | Other build needs |
|---|---|---|---|---|---|
| In-repo `wide.rs` | — | MIT (project) | none | — | none |
| `dashu-float` 0.6.1 | 0.6.1 | MIT OR Apache-2.0 | 6: `dashu-base`, `dashu-int`, `num-modular`, `num-order`, `static_assertions`, `cfg-if` | 1.68 | pure Rust; heap-allocated values |
| `rug` 1.30.0 | 1.30.0 | LGPL-3.0+ | 4: `gmp-mpfr-sys` 1.7.1 (LGPL-3.0+, `links = "gmp"`), `az`, `libc`, `libm` | 1.85 | C build of GMP/MPFR from bundled sources, per the crate's documentation; not verified on this host |
| `malachite-float` 0.12.0 | 0.12.0 | LGPL-3.0-only | 12 | 1.90.0 | — |
| `astro-float` 0.9.6 | 0.9.6 | MIT | 9, including proc macros | not declared | — |
| `twofloat` 0.8.4 | 0.8.4 | BSD-3-Clause | 9 | not declared | double-double only: fixed ~106 bits; the refutation's two-word prototype lost the whole twist and action in the k = 1e-28 case (`INDEPENDENT_REFUTATION.md` §3) |

The dependency sets are from the crates.io sparse index, taking the latest non-yanked version that satisfies each requirement. That approximates what `cargo` would resolve; nothing was fetched or built.

**Lockfile effect of an external backend in `frame_kernel`.** Each of these 23 lockfiles would change:
- `P/apps/desktop/src-tauri`;
- `P/core/loads/{load_case_algebra, primitive_loads, self_weight_wasm, stress_recovery, user_loads}`;
- `P/core/model_operations/operation_applier`;
- `P/core/product_physics`;
- `P/core/runner/headless`;
- `P/core/solver/{curved_bend, diagnostics, frame_kernel, linear_supports, nonlinear_integration, nonlinear_supports, performance_harness, sparse_direct, straight_pipe}`;
- `P/validation/benchmarks/{mechanics, nonlinear, numerical_integrity, physics_audit_regression, stress}`.

The kernel would stop being dependency-free. CI would fetch the crate at its `cargo fetch --locked` step, and the owner's Mac would need network access once.

**Recommendation: in-repo.**
- No lockfile changes, no C toolchain, no licence question.
- Stack-allocated limbs mean no allocation per operation.
- Bitwise determinism across platforms.
- A small trust base that can be tested thoroughly (expanded in revision 2, V1-S9):
  - at p = 53 against hardware binary64, bitwise, over random normal-range operands;
  - at p = 64L against exact-rational test vectors, generated by a standard-library Python script checked in beside the tests;
  - **targeted hard classes** at every p ∈ {53, 128, 256, 512, 1024} and for every operation:
    - exact ties to even at each limb boundary (bits 63/64, 127/128, …);
    - carry-out and renormalization on addition and multiplication;
    - massive cancellation (operands equal to within one ulp, and to within 2^-p');
    - exact and near-exact division and square root (perfect squares, and values one ulp from them);
    - sticky-bit paths (a nonzero discarded tail beyond the round bit);
    - TwoSum and TwoProduct error-free transformations, the basis of the exact-sum rule (§4.1.2);
    - conversion to binary64 across the subnormal boundary, including results that round up into the normal range, without double rounding;
    - overflow and underflow of the binary64 conversion, reported as representability outcomes;
    - `i64` exponent extremes, which must be refused, never wrapped;
  - **a large seeded differential** against the Python `Fraction` oracle: at least 10^6 operations per precision, generated with a fixed recorded seed, with the vectors committed and their hash recorded;
  - **seeded rounding mutants** (round-toward-zero, a dropped sticky bit, ties away from zero, an off-by-one limb shift). Each must be killed by the suite.
- Estimated at about 800 to 1,200 lines with its tests. This is an estimate.
- `dashu-float` is the fallback if review rejects the in-repo arithmetic.
- `rug` is not recommended: LGPL-3.0+ static linking into the MIT macOS bundle, a C build, and `links = "gmp"`.

## 5. Interface note for D2

What D1 publishes and what readers would need to verify. **D2 owns the successor-identity readers** (S-G, D2 revision 3 §4.9; ROOT's application of V1-S3). This section is their interface, and it follows D2's G1–G8, including revision 3's G5b and G5c.

1. **The receipt** is the top-level closed member (D2's placeholder `retained_precision`), with a `body` and a `receipt_sha256`. There is one entry per load case, in request order: `selected`, `unavailable` or `not_required`. A `selected` entry holds:
   - method token and policy id;
   - source identity digest;
   - the attempts list (p, outcome, reason, work), ending with the accepted attempt;
   - selected p and verification p;
   - stop-rule summary: the worst normalized disagreement per body and kind, and, for combinations, per combination;
   - **R's bits** (`0x3DD0000000000000`, R = 2^-34), which must equal the registered constant;
   - **S\* per body and kind** as bit strings (translation, rotation, force, moment), computed from published rows exactly as §4.1.6.1 specifies, so G5b recomputes them bit for bit. Per-member stress scales are derived by readers from these and the section terms (§4.1.6.1 item 7). **Twist and extension scales are harness-only and are not in the receipt** (revision 5, R4-4);
   - **`input_derived_dofs`** (revision 5): the rigidly restrained and prescribed DOFs as (node id, component) pairs (§4.1.6.1 rule 2a);
   - **`structural_zero`** (revision 5, optional, B): ids proven exactly zero by §8.1.2's pattern proof, with the pattern digest (§8.1.2), when the producer implements B;
   - **the verified-accuracy classification** (§4.1.6), made on the published binary64 value: the list of `absolute_verified` result ids, each with its bound `fl(2^-64·S*)` (k-inclusive S\* for stress rows; the b that D2's C binds), and the list of `not_covered` ids. Every other published quantity takes its class from the closed table (§4.1.6.1 item 2): `relative_verified` for scaled kinds above the floor, or `input_derived` or `non_quantity`. G5c recomputes the classification. Readers withhold `absolute_verified` and `not_covered` quantities from reliance and never count them as Passed (D2 §4.9.9);
   - pivot margin minimum, rcond estimate at p, and the retained-residual summary;
   - the load-ledger digest (the exact per-DOF expansions, hashed);
   - retained-state digest;
   - a reference to the ordinary attempt, bound to `numerical_quality.cases[i]` and its diagnostic;
   - the invocation digest, which a W1 attempt requires (`digest_ok()`, §4.3).

   An `unavailable` entry carries its reason, its attempts and a reference to its `RETAINED_PRECISION_UNAVAILABLE` diagnostic. A selected case never carries its own unavailable diagnostic.
2. **Canonical profile and encoding (V1-S4, aligned with D2 G1, G2 and G5).**
   - **The profile is the checked profile `openpipestress_jcs_ijson_v1`,** the one today's source receipts use (`source_receipt.rs:30-37`). Every reader already has it. The scientific profile would need a TS canonicalizer that does not exist.
   - **Section terms (revision 4, R3B-3).** Per member, A, Z, L, E·A/L and G·J/L as the product formed them, as bit strings, covered by the source identity digest (§4.1.6.1 item 7). This is the complete per-member field set; there are no twist or extension scales.
   - **Classes (revision 4).** The receipt lists `absolute_verified` and `not_covered` ids only; `input_derived` and `non_quantity` rows appear in neither list (D2 §4.9.10).
   - **Encoding.** Every receipt value that can exceed 2^53 − 1 in magnitude, be subnormal or be negative zero is a 16-hex binary64 bit string, as `source_receipt.rs:38-40` `bits()` does. That covers the stop-rule ratios, the pivot margin (typically 1e16 to 1e140), rcond, S\*, R and every bound. Plain JSON numbers appear only for exact integers within ±(2^53 − 1): counts, precisions and work units.
   - **Per-case hashing failure.** If a selected case's entry cannot be encoded, the case becomes `unavailable` with reason `receipt_encoding` and its ordinary standing. With the rule above this should be unreachable; it is kept as the defined outcome.
   - **Publication-hash failure.** `publication_sha256` covers the envelope minus the receipt. It fails when a published row value is ≥ 2^53 in magnitude, which is today's capture and carrier range limit (V1-S5, R-6). In that case the invocation is republished under its base identity (preview-physics-1, physics-1 or load-reference-1), with every attempt declined as `RETAINED_PRECISION_UNAVAILABLE` (reason `publication_hash_range`), each case keeping its ordinary standing. This follows T1's SF-1 pattern, continuing the same work ledger. **Never an `Err`, and never a blocked envelope.**
3. **`numerical_quality` (IF-1).** It keeps the ordinary attempt's outcome for every case (§4.5). Standing for a selected case comes only from the verified receipt.
4. **Row provenance.** Every row of a selected case carries `recovery_method = contribution_preserving_multiprecision_v1`, in the field `PP:3482` already uses. Derived stress rows say "from once-rounded retained actions". Combination rows formed over retained states carry the same token.
5. **Identity and emission.** See §4.4. The envelopes are mixed per case by design, so no invocation-level `Err` exists for them. Retirement of exact-block selection is gated per family (§4.4.1) at F2b, and until then the coexistence rule holds (§4.4).
5a. **D-5 evidence (revision 5).** The formation check's record is not in the receipt or in `StructuralReport`. From F1 it appears as one evidence line in the integrity diagnostic of a demoted case (§4.3.1).
6. **Replay** is a Rust validation-lane audit, not a standing input (D2 §4.9.4). Rust replays bit-identically from the capture.
7. **Representability per published quantity:** normal, subnormal (reduced precision stated), underflow refused, or overflow refused. Transport and display of subnormals belong to D2.
8. **W2.** `force_scale_exponent` is **not** in `StructuralReport` (V1-S7). When b ≠ 0 there is one added evidence line in the integrity diagnostic. Envelopes with b = 0 are byte-identical to today.
9. **S11.** `LOAD_CONTRIBUTION_ABSORBED` (warning), with `affected_refs` = case id plus contributing load ids. Under C3-full it should not occur for correctly assembled cases (`S11_CONTAINMENT.md` §6).
10. **SUP-17.** A message text change only.
11. **Selected-UNAVAILABLE alignment.** Under option A, fresh solves no longer emit `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` beside a selected case. D2's alignment then concerns historical and joined envelopes only.
12. **If ROOT chooses option B instead,** D2's composite and alignment designs must also cover mixed exact-block and multiprecision envelopes.

## 6. Slices, order and T1 serialization

**Owners.** Kernel slices are T3 TASKs with one writer per file at a time. Facade slices go to the single `core/product_physics` integration owner named in the graph, after T1 merges and main is merged into the T3 branch. ROOT serializes that owner against other tranches.

| Slice | When | Write set | Tests |
|---|---|---|---|
| **S11-K**: S11 kernel and library half (`S11_CONTAINMENT.md` revision 4 §8.1) | **first T3 slice; ROOT: lands soon after T1 merges**, as its own full-gate PR to main | `FK/exact_sum.rs` and `FK/load_ledger.rs` (new) and `FK/lib.rs` (their `mod` declarations; `reduce_system*` `:820-880` with the exact reduced right-hand side at `:870-877`, KS2, and typed entry points beside the old ones); `FK/structural.rs` (`Expansion::rounded()` replaced at `:369-377`, `:406-414`, `:554` with the zero witness at `:554` only; the audit with exact per-DOF force terms; the exact right-hand side in `prepare_structural` `:603-606`, KS1; the exact numerator in `evaluate_original_residual` `:697-760` on prescribed-coupled rows, KS3; a typed `StructuralSystem` constructor); `FK/structural/exact_boundary.rs:361`, `:387`; `SA` (`AssemblyEvidence::with_force_terms`, a typed `StructuralAssembly::solve`); `nonlinear_integration/src/lib.rs` (typed entry-point variants only); `primitive_loads` (re-exports); `SP` (E1–E4, E6); `CB` (`arc_section_resultant_terms`); `load_case_algebra` and its `Cargo.toml` (E13); `pressure_sum.rs` (wrapper) | `S11_CONTAINMENT.md` §9 S11-K tests K1–K12, including the kernel prescribed-motion test K11 and K4's axial-effect case; mutations M1a–M1e and M1m (`SP` and `load_case_algebra` sites), M6, M7, M10–M15; every existing suite passing; the committed-fixture diff with its stop rule and its pre-registered list of T1 support-motion fixtures (S11 §8.3). Live: `SP` recovery, combinations, the `FK` rounding sites; KS1–KS3 where a prescribed value is nonzero: T1's support-motion fixtures, and, **corrected in revision 5, the nonlinear active-set loop's closed-gap solves on main**, which by ROOT's ruling (option (c)) keep calling a named, unchanged binary64 legacy variant, pinned by a test, so DEC-046's exact-zero benchmark is untouched (`S11_CONTAINMENT.md` revision 5 §4.6, §8.1) |
| K1: W3 kernel sparse representation and sparse M03 gate | after S11-K (shares `FK/structural.rs` and `SA`) | `FK/structural.rs`, new `FK/structural/sparse.rs`; `P/core/solver/sparse_direct/src/{lib.rs, structural.rs}`; `SA` (sparse `AssemblyEvidence`, carrying S11-K's force terms) | Existing frame_kernel, sparse_direct and nonlinear_integration suites unchanged and passing. New: K bitwise parity, KREV-01 to KREV-05 on sparse, O(nnz) negative witness, dense and sparse outcome parity on N and R and NP-B and NP-D, and S11-K's audit in both representations |
| **K2a**: checked formation (ROOT: early; revision 5: after K-D5) | **after K-D5** (ROOT, D5_CHECK 3), as its own PR to main with full gates | `FK/lib.rs` (checked `local_stiffness`, every intermediate product and quotient; `FrameKernelError::NumericalRange`); `P/core/solver/diagnostics/src/lib.rs:285-362` (its mapping) | Every existing suite passing and byte-identical; LEF-small (exact zeros and the 3.5 %-wrong 12EI/L³) and LEF-large refused with the named reason; a partial-underflow control (12EI/L³ normal while 4EI/L underflows) and a subnormal-intermediate control refused; mutation 18; the committed-fixture diff (expected unchanged: no committed intermediate is zero or subnormal) |
| K2b: W2 force-radix scaling, **and the kernel half of formation-time scaling** (SCALE-W, ROOT) | after K1 (shares `SA`, `FK/structural.rs`) and after K2a (shares `FK/lib.rs`) | `SA`; `FK/structural.rs` helpers; `FK/lib.rs` (scaled formation from E, G, spring stiffnesses and load contributions times 2^b, and the sparse assembly entry that takes b) | b = 0 bit-identity on all existing tests and on every `Debug`-published report; a synthetic PHYS-R4 element; RF-RANGE kernel cases, including LEF-small and LEF-large **solved** at kernel level through scaled formation; the normative b-rule's branches (§4.7) |
| **K3a**: the part of W1's arithmetic K-D5 needs (revision 5) | **after S11-K, before K-D5** | new `FK/structural/retained/mod.rs` and `retained/wide.rs` with `Wide<2>` only: correctly rounded + − × ÷ √ at p ≤ 128, exact lift from f64, exact split into at most three binary64 terms (for `ExactAccumulator::add_product`), the work counter; **the one `mod retained;` declaration in `FK/structural.rs`** (MOD-D; it now merges after S11-K and before K1); test vectors and generator under `P/core/solver/frame_kernel/tests/` | §4.11 classes for L = 2, the seeded `Fraction` differential for + − × ÷ √ at p = 128, the split's exactness and its sub-2^-1074 allowance, rounding mutants |
| K3: the rest of W1's arithmetic | in parallel with K-D5 and K1, after K3a | `retained/wide.rs` (L = 4, 8, 16; runtime p; conversion outcomes to f64: normal, subnormal, underflow, overflow) | §4.11 classes, the differential and the mutants |
| K4: W1a kernel method | after K1 and K3 | new files under `FK/structural/retained/` only (`source`, `ledger`, `assemble`, `factor`, `recover`, `combine`, `adaptive`), declared from `retained/mod.rs` | N05, N06, NP-A (intended), R1's discriminating RF-CHAIN, RF-SKEW, RF-WEAK and RF-FINITE (represented basis where marked), RF-MECH, RF-CANCEL, the k = 1e-28 case, the B1 and S8 controls (§7.3), the published-row S\* and classification (§4.1.6.1), the exact-block oracle in scope, and mutation controls |
| K5: W4 witness and curved screen | after K2b (V1-N4; shares `SA`) | `FK/rigid_body.rs`, `SA` (geometry, curved screen) | A user-element internal mechanism (NP-C-like), a stabilized companion, near-collinear ties, a curved screen positive and a seeded negative |
| K6: harness observations | after K1 | `P/core/solver/performance_harness/**` | Kernel sparse and dense observations on RF-LARGE; runner dry run, including the macOS watchdog path |
| **K-D5**: the D-5 formation check (§4.3.1; ROOT pre-accepted O1, fixed per D5C-1) | **after K3a, before K2a** (ROOT, D5_CHECK 3); it writes `FK/structural.rs` and `SA`, so it is serialized after S11-K and before K1 | `FK/structural.rs` (EF after the residual gate in `finish_checked_factor`, `:877`; the Passed→Sensitive demotion; the typed optional formation source on `StructuralSystem`; the `FormationCheck` record on `StructuralSolution`, never on `StructuralReport`); new `FK/structural/formation_check.rs` (the `Wide<2>` re-formation of straight frames from primitives, and ρ as one `ExactAccumulator` sum per free row, extending `contribution_sums`/`audit_intended_action`); `SA` (the formation source from `AssemblyEvidence::new`: frame primitives, and a flag when any user or curved contribution is present). **Every caller of `solve_structural_dense`/`_sparse` is enumerated** (ROOT's recorded lesson): `SA:280-282` supplies the formation source for linear cases; `nonlinear_integration/src/lib.rs:1933-1936` (active-set loop) and `product_equilibrium.rs:122-153` supply none and are pinned unchanged by a test. Nonlinear cases are outside K-D5 and W1a, recorded as T5 open work beside the KS legacy variant and the friction fold | **P1's RF-SKEW-T-CANT-OFF-122-r1e-04 must demote in both modes (required true positive)**; 345, the RF-CHAIN r1e-04 continuity controls and the invented M11 must not; **D5C-1's controls:** a solve-error-only case (V1's probe D class: an axis-aligned member with an exactly representable spring sum near cond 6.6e7), an absorbed-spring case (probe C: k_X ≈ 0.05 against GJ/L with a 1.1e-9 loss) and an axis-aligned bending-soft case near the criterion, each demoting where its actual error exceeds 1/2 of the criterion; the zero-scale clause (a body published as unmoving, intended moving); **D5C-2:** a curved-arc and a user-stiffness case demote with `formation_check_unavailable`; **D5C-3:** every committed raw byte-identical when nothing demotes, and `StructuralReport` unchanged; the committed-fixture diff (expected unchanged apart from the declared curved-arc tests, §4.3.1); the "no Passed breach" gate through both entries; mutations (23), (26)–(28) |
| V-K: VP-ROBUST kernel lane | after R1's references are frozen and K4 | new `P/validation/benchmarks/numerical_robustness/**` | §4.10 kernel lane, the zero-scale floor check |
| **S11-F**: the S11 facade half | **the first facade slice after T1 merges** | `PP` (the ledger at every producer of `S11_CONTAINMENT.md` §4.2, with pushed terms, including T1's eigen sites; `AssembledForce` seams; recovery sums E5, E7–E12; the Sensitive mapping and `LOAD_CONTRIBUTION_ABSORBED`; the enumerated site test); `P/core/product_physics/src/pressure_runtime.rs` (push group operands); T1's `source_recovery.rs:609-667` including `:1270-1274`, `source_receipt.rs:218-219` and `:320` | `S11_CONTAINMENT.md` §9 S11-F tests 1–8, including the invariant test and V1's 0.4.0 test; **through both entries** (the captured route, and the historical typed entry via headless `run_preview_in_memory_mode`), with RF-CANCEL at G = 1e80 (F, M, ORTHO, INPLANE, UDL-W1e80) on the typed entry and on the captured route once S-H is present (ROOT `b6fe1eb75`); **S-H never lands before S11-F** (same PR, or S-H's PR after S11-F is on main, re-running the 1e80 cases through the captured route); the nonlinear loop stays on the legacy KS variant when `PP` moves to the typed path; the named exceptions list emptied; mutations M2–M5, M8, M9; the committed-fixture diff with disclosure (S11 §8.3) |
| F1: facade sparse wiring, W2 at formation, SUP-17 | after S11-F | `PP` (assembly `:1620`, `:1751`, now the kernel's sparse assembly with K2b's formation-time scaling; reduction `:2330-2342`; reactions `:2697`; `solve_preview_reduced_system`); `source_recovery.rs` (refuse scaled evidence); the nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5) | Full product suites, the PHYS-R4 public fixture, LEF-small and LEF-large solved, the parity protocol, the dense-scrutiny guard |
| **F2a**: facade W1a wiring and identities, **no retirement** (revision 5, R4-2) | after F1 and ROOT's identity reservation; **atomic with D2's S-G1** | `PP` (case loop, receipt, rows, exact combinations, the coexistence rule of §4.4), new `P/core/product_physics/src/retained_publication.rs`, tables in `P/fixtures/results/`, schemas | Product lane of VP-ROBUST; W1 selected on Sensitive and D-5-routed cases outside exact-block selection; **coexistence:** every committed exact-block-selected request publishes byte-identically, and no envelope carries both selected diagnostics; the producer's published-row S\* and classification (including `input_derived_dofs`), checked against D2's G5b and G5c; the D-5 routing (the demotion becomes routing to W1 where the invocation has no exact-block selection) |
| **S-I1, S-I2** (D2): option C, interval binding (DD-13, per ROOT's conservative-binding constraint and R4-3) | S-I1 (evaluator, runner, Python reference; rules crates only) any time; S-I2 (binding wiring) with or right after F2a/S-G1; **both before F2b and F3's retirement** | D2's write sets (§8.1.3 gives D1's side and the cost) | D2's plan: three-valued pass/fail/indeterminate, outward endpoints, covered-row outcomes unchanged, parity in three languages |
| **F2b**: retirement of exact-block selection, **domain by domain** | after S-I2; per family, only when the §4.4.1 gate passes on the actual candidate, including row-level condition 3 | `PP` (selection order for the retired family); the characterization tests for retired exact-block selection, replaced and recorded | **The retirement gate (§4.4.1) for source-blocks-1 and physics-source-1**, with the side-by-side withheld counts; N05 and N06 through the public entry; **D2's corrected probe PR-2b (F-P2)**: in P12, one case is selected and another case's ordinary attempt is rejected (for example N06-class `ASSEMBLY_UNRESOLVED`), and W1 must recover the rejected case; a family whose gate fails stays on exact-block, unaffected by the others |
| F3: W1b families and the 0.4.0 successor | after F2a and the RF-ELOAD addendum, its retirement step after S-I2; **atomic with D2's S-G2 and S-E1** | `PP` load builders to `PrimitiveSource` and the ledger; kernel element-load primitives | RF-ELOAD; 0.4.0 prescribed and eigen cases; **the gate for `load-reference-source-1`** (row-level; eigen_motion needs C), which then stops being fresh (R-3(a)) |
| R: readers | per D2 | D2's write sets (S-G and the rest) | D2's plan |
| V-P: product lane and scale runs | after F1 and F2a | `numerical_robustness/**` | §4.10, §4.8 |
| Join | last | ROOT and the manager | §7.5 |

**Order (revision 5).**
- **Kernel, now (T1 merged).** **S11-K → K3a → K-D5 → K2a → K1 → K2b → K5** (ROOT, D5_CHECK 3, second form). The exact-residual EF needs a re-formation accurate far below 2^-53 through divisions and square roots, so it needs `Wide`. K3a carries only `Wide<2>` and its operations, and is small next to K3.
  - S11-K, K-D5, K1, K2b and K5 share `SA` and `FK/structural.rs`, so they stay serialized.
  - K2a shares only `FK/lib.rs` (`local_stiffness`) with S11-K and K2b, and follows K-D5 because K-D5 guards a broad Passed-band class while K2a affects only sub-2^-1022 stiffness (ROOT).
  - The rest of K3 runs in parallel after K3a. K4 follows K1 and K3, then K6 and V-K.
- **Why K2a is separate from S11-K.** The two share only `FK/lib.rs`, in different functions: S11-K adds its `mod` declarations and changes `reduce_system*` (`:820-880`), while K2a changes `local_stiffness` (`:699-768`). Kept apart, each fixture diff is attributable to one change.
- **Facade, after the merge of main (done at `303609725`).**
  - **S11-F comes first.** D2's S-H lands in the same PR or after S11-F is on main, never before (ROOT `b6fe1eb75`).
  - Then F1 (which renders the D-5 evidence line), then F2a atomic with S-G1, then D2's S-I (C), then **F2b per domain**, then F3 atomic with S-G2 and S-E1, with its retirement step after S-I. Then V-P and the join. W1c follows in T4's window.
  - D2's optional B slice (S-J) follows S-I and is on no retirement's critical path (§8.1.2).
- **Timing.** ROOT's S11 no-interim ruling reopens if P1 finds a Passed breach in a realistic model (ROOT's adopted text).

**Atomic PRs.** F2a with D2's S-G1, and F3 with S-G2 and S-E1: producer, readers and gates merge together, as T0R did. F2b is its own PR per family.

**T1 overlap (revision 5: T1 has merged).** The T1-overlap hold is lifted. The facade slices (S11-F, F1, F2a, F2b, F3) edit `PP`, `source_recovery.rs`, `source_receipt.rs`, the schemas and the fixtures, which T1's merge changed, so they build on `303609725` or later. The kernel slices (S11-K, K3a, K-D5, K1 to K6, V-K) touch no product-facade file. `nonlinear_integration` is T5's future area, so ROOT serializes it; K-D5 changes only `SA` there and pins the nonlinear loop's solves unchanged. The `diagnostics` crate (K2a) is facade-disjoint.

## 7. Verification plan

### 7.1 Frozen references (none edited)

- **N05, N06, NP-A.** The new method meets the unchanged 1e-9 intended-answer criterion on every quantity, including internal torque and the spring action.
  - NP-A's represented-matrix expectations stay the oracle for the ordinary route.
  - A new-method result equal to the represented solution is a failure: it would mean the rounded matrix was promoted.
- **N01 to N09, R01 to R07, NP-B, NP-D.**
  - Both modes, with outcome classes unchanged.
  - R01 to R05-type mutations are killed by the retained residual gate as well.
  - NP-C (connector) is refused as an unsupported family.
- **T0R references.** Kernel-lane "force extended" runs reproduce them. This is a test-only switch showing continuity at ordinary scale.
- **R1 families (revision 2)**, once V2 refutes them and ROOT freezes them, with R1's zero scales. Specifically:
  - the represented basis for `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6`, and for every case whose `finite_input` marks it;
  - the discriminating negative controls as mutation tests;
  - RF-CANCEL on the net-governed scale;
  - LEF-small and LEF-large solved;
  - the zero-scale floor check (§4.10).
- **The DEC-053 nine observations,** under their unchanged predicates.

### 7.2 Detection run P1 (on main, before implementation)

Expected, recorded per quantity:

| Family | Expected on main |
|---|---|
| RF-CHAIN | k/a ≈ 1e-4 (and 1e-6): passes. These are continuity controls (R1 finding 2). **From about 1e-8 down**: Sensitive rows that miss 1e-9, or unresolved (absorbed contribution). R1's lost-soft, stored-assembly and subtract-rounded controls discriminate from 1e-8 down. Order > 2 means exact-block is unavailable |
| RF-SKEW, RF-FINITE (soft) | Misses of 1e-6 to 1e-3 from k/a ≈ 1e-8 down, or unresolved; exact-block unavailable (not a signed permutation). The two represented-basis cases are compared on represented inputs |
| Axis-aligned bending soft modes | Misses of about 1e-5 (§3.1), or unresolved |
| RF-WEAK | Likely passes, or unresolved through an absorbed contribution; to be observed |
| RF-LARGE | 1,000 and 10,000 members: refused, or out of memory under the runner's cap (§2.1). Long cantilever chains may also be unresolved on conditioning. That is my expectation, not a measurement |
| RF-RANGE, PHYS-R4 | `NUMERICAL_INTEGRITY_UNRESOLVED` (range) at the extremes. **LEF-large:** a blocked envelope with a non-finite formation error. **LEF-small:** the underflowed GJ becomes an exact 0, so an unresolved zero diagonal, a spurious mechanism, or a wrong value; P1 records which |
| RF-CANCEL | (G, n, −G) and (n, G, −G) Passed and Current with the fold's answer: within 1e-9 at G = 1e5 and 1e6, failing at 1e7 and 1e8, and the net lost at 1e80 (S11). (G, −G, n) exact |
| S11 recovery (V1's probe A as a product model: one member with uniform loads (G, 0.3, −G)) | Passed with the recovery fold's member actions: root shear and midspan moment off by more than 1e-9 of their own magnitude at G ≥ 1e7 (probe: 6.8e-9 at 1e7), and the solve's force also folded. A Passed breach with realistic magnitudes is reported to the manager at once (ROOT's reopen trigger) |
| RF-ZERO, RF-INVARIANCE (well-conditioned) | Pass |
| RF-MECH | Refused with a witness for qualified bodies. Large ones: memory, as for RF-LARGE |

Every P1 run protects the host: `RLIMIT_AS` on Linux, the RSS watchdog on macOS.

**Additions to P1 (revision 5).** For every Passed-band case (cond from 1e6 to 6.7e7), P1 also records member-action and reaction comparisons (N-1), since EF checks nodal quantities only. P1 records the y_ref its adapter used (N-4). For the 12–13 RF-CANCEL breaches, P1's record of (case, quantity) pairs on both entries becomes the committed exception list of §4.10 (D5C-5).

### 7.3 Mutation controls (each must fail at least one comparison)

Mutations run against R1's discriminating controls. A mutation that no discriminating comparison kills is reported, and its case set is extended before implementation continues.

1. Promote the rounded binary64 K to p. N05 then has the wrong k and N06 a nonpositive pivot.
2. Round u to binary64 before recovery. N06's torque becomes 0.
3. Drop `K_fc u_c`, or round the reduced RHS to binary64 (the KREV-02 control, and 0.4.0 prescribed cases).
4. Omit one contribution, or flip an axis sign in `B`.
5. Fix the precision at 128 with no escalation. The k = 1e-28 case fails.
6. Skip the 2p verification, accepting on the pivot screen alone. The k = 1e-28 case fails, as in the probe.
7. Disable the geometric mechanism check. RF-MECH is then "recovered".
8. Omit a pattern entry, such as a spring, in sparse mode only (the R05 analogue).
9. Publish without unscaling by 2^b (W2).
10. Make the order depend on labels (relabelling fails).
11. Reuse one modulus basis for every case.
12. Treat a user-element tie as a rigid link (W4).
13. **(revision 2, V1-B1)** Fold loads at p instead of the ledger. V1's check L is then accepted with error 0.5 (§3.2 B1-L), and RF-CANCEL fails.
14. **(revision 2)** Combine retained states term by term at p. The combination A + B − A2 is accepted with error 1.0 (§3.2 B1-C).
15. **(revision 2)** Take combination outputs out of the stop rule. The combination (P, ε) − P is published from 128 bits with error 1.0 (§3.2 B1-E).
16. **(revision 2)** Assemble stiffness entries or recovery sums by sequential addition at p. A duplicate-operand cancellation control fails: two identical members meeting at a node, plus a third member 2^-300 as stiff.
17. **(revision 2, V1-S8)** Drop the verified-accuracy classification. S8-W's far-node quantities are then labelled `relative_verified`, and the reader check fails.
18. **(revision 2, K2a; revision 3)** Revert to unchecked formation, or check only the final coefficient. LEF-small's torsion coefficient becomes 0 and its 12EI/L³ is 3.5 % wrong; the underflow, partial-underflow and subnormal-intermediate controls fail.
19. **(S11)** Mutations M1a–M1o and M2–M15 of `S11_CONTAINMENT.md` revision 4 §9, with the required kill set G = 1e8 and 1e80: one mutant per E-site, the restored ledger fold, a producer or copied vector outside the typed seams, a pre-summed curved thermal, a naive rounding, a −0.0 zero, the subnormal copy-bits shortcut, each missed T1 site, and the three kernel prescribed-motion sites KS1–KS3.
20. **(revision 3, S8-R)** Classify on the precision-p value instead of the published value, or use R = 10^9·2^-64 with a rounded threshold, or compute S\* from anything other than published rows. D2's G5b or G5c then finds a mismatch on a constructed boundary case (a quantity within one ulp of the threshold).
21. **(revision 3, F2)** Derive twist and extension in the harness as differences of published rotations and translations. An RF-CHAIN N05-class twist then fails its relative comparison, while the torque-derived value passes.
22. **(revision 3, F2)** Count a `not_covered` comparison as a pass. The gate's three-number report then disagrees with the committed not-covered list, and the gate test fails.
23. **(revision 4, D-5)** Disable the D-5 trigger. RF-SKEW-T-CANT-OFF-122-r1e-04 is then published Passed, and the "no Passed breach" gate fails.
24. **(revision 4, D-5; revision 5)** Compute the trigger from the bound EB instead of EF, or use factor 8 instead of 2. With EB, M11 and the RF-CHAIN r1e-04 continuity controls demote; at factor 8, RF-CHAIN-T-n03-r1e-06 and RF-CHAIN-A-n03-r1e-06 (dense) demote (emulated). This is the false-positive control.
25. **(revision 4, R3B-1; revision 5, R4-1)** Drop the propagation factor k (k = 1 for every stress kind), or use the one-end values √2 and 2 for the span-statics rows. A constructed circular-maximum row just above the floor (V1's probe S bound 1.32e-9), and a constructed **span-statics** circular-maximum and open-formula-summary row just above the floor (V1's probe J: 1.86e-9 with k = √2 and k = 2), then carry an error above 1e-9 relative, and the reader's recomputed class differs.
26. **(revision 5, D5C-1)** Form EF with the product's binary64 published residual in place of the exact intended residual. The solve-error-only control is then missed (V1's probe D: 28 of 105 missed, EF/actual down to 1.2e-16).
27. **(revision 5, D5C-1)** Form EF from element-level ΔK (springs, assembly sums and the symmetric average left out). The absorbed-spring control is then missed (V1's probe C: 108 of 230).
28. **(revision 5, D5C-1)** Re-form the frames from the binary64 local coefficients (reusing `local_stiffness`'s 12EI/L³, 6EI/L², 4EI/L, 2EI/L). The axis-aligned bending-soft control is then under-read three times (V1: actual 0.931, EF 0.296), and is missed at the criterion.
29. **(revision 5, D5C-2 and D5C-3)** Silently pass a case with a curved or user contribution (skip the demotion), or put the EF record into `StructuralReport`. The curved-arc control then publishes Passed, or the byte-identity test fails on every committed raw.
30. **(revision 5, D-15)** Attempt W1 in an invocation where exact-block selects a case, or retire a family whose side-by-side withheld counts show a worse case. The coexistence test (byte-identical exact-block envelopes) or the row-level gate then fails.

### 7.4 Arithmetic (V1-S9)

§4.11's targeted classes, the seeded `Fraction` differential and the seeded rounding mutants run in `frame_kernel`'s own suite, and so in hosted CI.

### 7.5 Native witnesses (owner's Mac)

- N05 and N06 through the desktop, with the new method's token selected.
- An RF-SKEW soft case (k/a ≤ 1e-8).
- A ten-member RF-CHAIN.
- A 1,000-member model: time and peak memory recorded.
- The PHYS-R4 public fixture.
- An RF-CANCEL case and the realistic thermal case (a 4.1e7 N thermal pair after a 1.3 N co-axial load; `S11_CONTAINMENT.md` §9 F3), after S11-F.
- Save and reopen, and the export surfaces.
- A model outside coverage, such as an elbow, showing `RETAINED_PRECISION_UNAVAILABLE` with ordinary standing.

Record the candidate, the model hashes and the case ids. If a witness is not available, record it as outstanding.

### 7.6 Gates

- A complete-diff independent review.
- Hosted CI: the numerical cargo suite, including the new crate, and the full dual-viewport dispatch for surface 4.
- A clean DEC-025 sweep.
- The native witnesses.
- The retirement gate (§4.4.1), before any family is retired, with row-level condition 3 (side-by-side withheld counts) and, on the committed families, after D2's C slices.
- VP-ORACLES and VP-ROBUST passing on the merged candidate. This is the graph's closure rule.
- **Early live slices (ROOT, D-S11-3).** S11-K and K2a each land as their own PR to main, with a complete-diff independent review, hosted CI including the surface-4 dual-viewport dispatch, a clean DEC-025 sweep, and the committed-fixture diff with its stop rule (`S11_CONTAINMENT.md` §8.3). Their change records carry the disclosure of §8.3 there (no in-band marker, D-S11-4).

## 8. What T3 completes and what remains, per group

| Group | D1 completes (when merged and verified) | Remains |
|---|---|---|
| M03 | General accuracy for the W1a and W1b families; **the S11 load-cancellation repair on every route, force and recovery side (S11-K and S11-F)**; the PHYS-R4 and formation range (W2, checked formation); the rigid-null witness for user and screened curved bodies; the SUP-17 wording; VP-ROBUST | Curved bends in the method (W1c, with T4); components and releases (proposed T4 and T7); equivalent-static (open, no owner); nonlinear mixed recovery and gap classification (T5); the S11 audit inside the nonlinear loop's linearized solves, and friction terms added in binary64 (`nonlinear_integration/src/lib.rs:1642-1659`), open with T5 (S11-V5) |
| M32 | Sparse assembly, reduction, reactions and the gate in both modes; the parity and memory protocol; dense scrutiny kept, with a guard | Friction influence solves (T5, SUP-16); resource ceilings chosen by ROOT from measurement |
| M34 | Solve-side range (W2 at formation); publication representability outcomes; the capture-boundary cases in VP-ROBUST | The capture-hash fix (D2, R-6), display and transport range, the scientific carrier, comparison policy (D2 and T6) |
| N05 ordinary accuracy | Repaired on the ordinary and exact routes for W1a models | — |
| General retained-source recovery | Order > 2, skewed and weakly coupled systems, and larger systems up to the measured budgets; W1b load producers; exact combinations over retained states | Curved, pressure regions, components (W1c and later) |
| Standing (with D2) | The retirement gate (§4.4.1) and the receipt interface, including the published-row S\* and classification that D2's readers recompute; `load-reference-source-1` retires at F3 under the gate | Case-scoped standing, including S11's envelope-level Sensitive side effect and **F-P2's whole-invocation blocking by a rejected, unrecoverable case** (T6, as ROOT recorded). F-P2's recovery half is open with W1 (F2, PR-2b) |

Real piping models almost always contain weight and elbows. Until W1b and W1c land, most of them get ordinary standing, or the unavailable diagnostic, when Sensitive. S11's repair of load cancellation (recovery sums in `SP` and combinations from S11-K; the ledger and the rest from S11-F) applies to every route and every producer from the first slice after T1 merges.

### 8.1 Combined withholding, and D-15's resolution (revision 4; revision 5 per ROOT's D-15, C and R4-2 rulings)

What a model loses from Current under this design has three sources:
- D-5 demotions (K-D5, before W1);
- rows of a W1-selected case classified `absolute_verified` (below the floor);
- rows classified `not_covered` by the closed table.

Ordinary (not selected) cases publish ordinary rows with no class, so only the first source applies to them.

#### 8.1.1 The committed figures, corrected (R4-2)

Rows withheld per case under A (withholding) are shown against the retiring identity (0 where it publishes the case as Current). The table then shows what the proof-carrying B (§8.1.2) and C (§8.1.3) restore. Counts come from `withheld_rows.py` (revision 4, reproduced by V1) and `b_proof.py` (this revision), on every committed exact-block-selected case at the merged tree, dense and sparse alike.

| Committed selected case | Rows | Withheld under A (retiring: 0) | Of which exact zeros | Proven zero by B | Zero, not proven | Non-zero (a user may rely on them) | Withheld after B | After C |
|---|---|---|---|---|---|---|---|---|
| N05 and N06 family (physics_source n05, n05_unicode, n05_units, n06; source_blocks n05, n06 and ui; load_reference_source n05, n06) | 79–82 | 61–62 | all | **all** | 0 | 0 | **0** | 0 |
| physics_source and load_reference_source mixed, `case` | 81–82 | 62 | all | 62 | 0 | 0 | **0** | 0 |
| physics_source and load_reference_source fields | 81–82 | 8 | all | 8 | 0 | 0 | **0** | 0 |
| source_blocks multicase (and ui), `case` | 79–80 | 61 | all | 61 | 0 | 0 | **0** | 0 |
| source_blocks rejected_stress_range | 79–80 | 59 | all | 59 | 0 | 0 | **0** | 0 |
| **source_blocks multicase (and ui), `case:signed-companion`** | 79–80 | 57 | **46** | 46 | 0 | **11** | 11 | 0 |
| **mixed `case:ordinary-pressure`** (physics_source, mixed_units, load_reference_source) | 109–111 | 68 (plus 25 input_derived) | all | 8 | 60 | 0 | 60 | 0 |
| **load_reference_source eigen_motion `case:join`** | 89–90 | 62 | **39** | 32 | 7 | **23** | 30 | 0 |
| Committed Passed cases that solve (P1: 32 runs), the committed demo envelopes, invented models M1–M11 | — | 0 | — | — | — | — | 0 | 0 |

**The correction (R4-2).** Revision 4 said the withheld rows are "almost all exact structural zeros". That holds for the N05, N06, fields, mixed `case`, multicase `case` and rejected families. **It is overstated for signed-companion (46 of 57 zeros) and eigen_motion (39 of 62).** Their non-zero rows are the soft-path responses these fixtures exist to test:
- in signed-companion, the soft support's reaction moment (2.0e-8 N·m), five member torsional moments (±2.0e-8 N·m) and five torsional shear stresses (−3.70e-11 MPa);
- in eigen_motion, the soft spring's reaction moment (−1.0e-8 N·m) with its magnitude row, member torques, torsional shear stresses, and anchor, bending and bending-stress rows at the 1.86e-13 N·m and 6.9e-16 MPa noise level (V1, BACKCHECK_R4 §6).

A support-load rule or a sign check on the soft restraint relies on them, and exact-block publishes them as Current today. **So under A alone the successor is worse, row for row, in every one of the three families**, and F2b waits for C.
- The eigen_motion zeros that B cannot prove (7 rows: the member's in-plane shear and bending rows, and one anchor reaction) are zero because the anchor's prescribed UY and RZ form a compatible rigid motion. That is a cancellation, not a structural decoupling.
- The ordinary-pressure zeros are not proven because B seeds a pressure member conservatively as fully loaded.

#### 8.1.2 B: the proof-carrying exact-zero exemption (ROOT: permitted only where mechanical)

**The proof.** It runs on integer pattern data and exact rational tests, and holds for every stiffness value with that pattern. `b_proof.py` is the reference implementation of these steps.
1. **Frame zero pattern, exact.** From each straight member's binary64 end coordinates and y reference, form the exact rational vectors d = x_j − x_i, y_c = y − ((y·d)/(d·d))·d and d × y_c. Their zero entries are the local axes' zero entries: normalization scales a vector, so it does not change which entries are zero.
2. **Element pattern.** The local stiffness couples only within four families: axial (local x translations), torsion (local x rotations), the x-y plane (y translations, z rotations) and the x-z plane (z translations, y rotations). A family's global DOF set is the union, over both ends, of the global components its local axes touch. The element couples every pair in one family set. This Boolean pattern of Tᵀ K T is a superset of its nonzeros for every value.
3. **Seeds.**
   - Free DOFs with a nodal load term.
   - For a member load, pressure region or unrecognized 0.4.0 element state: every free DOF of that member.
   - For a uniform axial eigen strain (0.4.0 `explicit_interval_strain` or `fit_strain`): the member's axial-family DOFs.
   - Free DOFs coupled to a nonzero prescribed DOF (0.4.0 boundary motion).
   - Any load term the producer cannot place, a component (joint, user, curved element) or a history other than independent equilibrium gives **no proof for the case**.
4. **Reach.** Breadth-first search over the free-DOF coupling graph from the seeds. K_ff is positive definite for a solved case, and the unreached block has a zero right-hand side and no coupling to the reached block, so its displacements are exactly zero for every stiffness value with this pattern.
5. **Rows.** A row is proven zero when every DOF its formula reads is proven zero and no load term enters it:
   - nodal rows, by DOF;
   - member end and station actions, by the families they read, only on a member with no member load (an axial eigen strain excludes only the axial-family rows);
   - stresses, by the families they read, with pressure-bearing kinds excluded when the case has pressure;
   - support reaction components, from the restrained DOF's element couplings when no load acts at that DOF; a spring reaction from its DOF; components with no restraint on them are structurally zero;
   - magnitudes and resultants, from all their components.
6. **Exemption.** A row is exempt from withholding only if it is proven zero **and** published as ±0.0. It then binds as the exact point +0, and its error is exactly zero.

**Why it complements C.** C binds an `absolute_verified` row as the interval q ± b, so `equal` and `not_equal` on a zero row are indeterminate. B lets a proven zero bind as the exact point 0, as exact-block does today.

**Cost and home.**
- **Producer.** An integer graph search plus exact rational zero tests on geometry, O(members + nnz). The receipt lists `structural_zero` ids and a pattern digest (§5).
- **Readers.** With an invocation, a reader re-derives the proof and requires the recomputed set to equal the listed set in both directions; otherwise `RETAINED_PRECISION_STRUCTURAL_ZERO_MISMATCH` (unsupported). Without an invocation there is no eligibility, so nothing depends on B. This is D2's contract (D2's reply; slice S-J, after S-I).
- **Parity.** Three-language parity needs the steps above pinned exactly, including the BFS order (by DOF index) and the digest payload. D2 asked for this, and D1 supplies it with the producer slice.
- **Status: optional, off every retirement's critical path.** On the committed families it restores 46 of 57 withheld rows in signed-companion, 32 of 62 in eigen_motion, 8 of 68 in ordinary-pressure, and every withheld row elsewhere. But each family keeps at least one case with rows only C can restore.
- **What B does not cover:** symmetry zeros, cancellation zeros (as in eigen_motion's rigid support motion) and tiny non-zero rows. It never claims a zero it has not proven.

#### 8.1.3 C: interval binding, the restorer (ROOT: within delegated authority, under the conservative constraint)

- **The constraint (ROOT, `de60993b5`).** A rule bound to q ± b passes only if it holds for every value in [q − b, q + b]; otherwise it fails or reads indeterminate. It never changes how a covered row binds.
- **Owner and design.** D2 (DD-13, D2 revision 5 §4.11, S-I1 and S-I2), per R4-3:
  - sound, outward-rounded interval evaluation over the whole rule formula language, three-valued (pass, fail, indeterminate);
  - division by an interval containing 0, and `equal` or `not_equal` on overlapping intervals, are indeterminate;
  - `RULE_RESULT_INDETERMINATE` is never counted as a pass;
  - scope is `absolute_verified` rows only; relative and input_derived rows bind as points, as today;
  - b comes from the receipt, with outward endpoints next_down(q − b) and next_up(q + b);
  - the basis of b is stated as operational stop-rule evidence, not an enclosure, and b is not doubled;
  - a C-bindable row stops counting as withheld once S-I is merged.
- **D1's side.** The receipt's per-row b = fl(2^-64·S\*_row), with k-inclusive S\* for stress rows (2√2 and 4 for span statics, k_i rounded upward), and the classification rules above. D1 adds no endpoint arithmetic.
- **Cost** (D2's estimate, as D1 reads it):
  - an interval evaluator and runner in the Rust rules crates, the Python reference (`compatibility.py`), the TS mirror and the desktop runner;
  - one shared parity case file;
  - a new result code in summaries and headlines.
  - No kernel or solve change, and no change to any covered row's outcome.
- **Slice and place.** S-I1 (evaluator; rules crates only) can land any time. S-I2 (binding wiring) lands with or right after F2a/S-G1. **Both precede F2b and F3's retirement step in every domain**, because C is the restorer everywhere on the committed families (§8.1.1).
- **After C.** Every committed case's successor withheld count is 0 against the retiring identity's 0, so condition 3 can pass. Retirement then still needs conditions 1, 2 and 4 on the actual candidate.

**Reading.**
- For ordinary and realistic models the combined loss is **none**. K-D5 fires on none of them, D5C-2's demotion touches no committed solving fixture (§4.3.1), and the closed table withholds nothing from an unselected case.
- For today's exact-block-selected cases, A would withhold 10–77 % of the rows. That loss never reaches users under ROOT's ordering: exact-block stays selected (coexistence rule) until C, with B optionally alongside, restores no-worse row standing and the gate passes per domain.
## 9. Decisions for ROOT

| ID | Decision | Options | Recommendation |
|---|---|---|---|
| D-1 | General method | (A) multiprecision with basic-deformation formation; (B) global reformulation; (C) generalized exact-block | (A). The probe refutes (C) (§3.1) |
| D-2 | Arithmetic backend | In-repo `wide.rs`; `dashu-float`; `rug` | In-repo, with §4.11's test plan (V1 agrees in V1-S9); `dashu-float` as the fallback |
| D-3 | Method policy (revised) | `M03-INTEGRITY-MP-v1`: the exact-sum rule (§4.1.2); the stop rule `2^-64·max(\|q_2p\|, S*)` with body-level coupled scales, applied to case and combination outputs; the equilibrium basis `retained_precision_p`; the 128/256/512 schedule with a 1024 ceiling | Register as proposed. The stricter per-member variant is the alternative |
| D-4 | Method order and identities | (A) retire exact-block selection for fresh solves, exact-block kept as oracle; (B) exact-block first plus the new method, in a mixed identity | (A), under the retirement gate (§4.4.1). Reserve three successor identities (names are placeholders): `<preview-retained>` and `<physics-retained>` at F2a, `<load-reference-retained>` at F3; exact-block retired per domain at F2b (F3's retirement step) under the row-level gate, after C (revision 5) |
| D-5 | Trigger for Passed cases (revision 5) | O1: the formation check as K-D5, then routing to W1; O2 and O3 as in `D5_TRIGGER.md` | **ROOT pre-accepted O1**, O2 rejected. **Revision 5 fixes D5C-1** (exact-residual EF, §4.3.1): factor 2 on the coupled S\*, the zero-scale clause, demotion for contributions that cannot be re-formed (D5C-2), and the evidence kept out of `StructuralReport` (D5C-3). The "no Passed breach" gate runs through both entries with named (entry, case, quantity) S11 exceptions. O1 becomes final when V1's re-pass confirms no misses on its 230 and 105 sweeps and the 122 case |
| D-6 | W2 | Force-radix scaling at formation, with the normative b-rule and checked formation; the admitted range; no new diagnostic code (one new `FrameKernelError` variant) | As proposed |
| D-7 | W3 fallback and guards | No automatic fallback, dense scrutiny explicit and guarded; or automatic dense fallback on sparse representation errors | No automatic fallback. Ceilings from measurement |
| D-8 | Budgets | Per-case and per-invocation work limits for the method | Select after the K6 and V-P measurements, as RESOURCE_POLICY did. The gate's coverage condition must fit inside them |
| D-9 | W4 | Constrained-body witness; curved objectivity screen | As proposed; T4 confirms the curved construction |
| D-10 | Serialization (revision 5) | S11-K → K3a → K-D5 → K2a → K1 → K2b → K5 (EF needs `Wide<2>`), K3's rest in parallel; facade: S11-F (S-H never before it) → F1 → F2a with S-G1 → S-I (C) → F2b per domain → F3 (retirement step after S-I) | As in §6 |
| D-11 | R1 addendum | RF-ELOAD references (uniform, thermal, thrust, constant effort, prescribed motion) before W1b | Commission it after V2 |
| D-12 | Below-floor quantities (V1-S8) | (a) publish them classified, readers enforcing the label; (b) withhold them | **Ruled (S8-R):** classify on the published value with R = 2^-34, readers recompute, and `absolute_verified` and `not_covered` quantities are withheld from reliance and never counted as Passed; no exemption proposed (§4.1.6) |
| D-13 (new) | S11 containment | C3-full as S11-K and S11-F (ledger plus exact recovery, `S11_CONTAINMENT.md` revision 4); C3-detect as the fallback; no interim containment (ROOT's adopted text); sub-decisions D-S11-1 (zero witness), D-S11-2 (all three `FK` rounding sites), D-S11-3 (live T1-disjoint repairs in S11-K), D-S11-4 (no in-band marker) in S11 §11 | ROOT decided D-S11-1 (witness for published diagnostic renderings only), D-S11-2, D-S11-3 (full gates, own PR) and D-S11-4 at `4862a72a9`. The pre-acceptance of C3-full awaits V1's backcheck of S11 revision 2 |
| D-14 (new) | Equivalent-static owner (V1-N5) | Assign a tranche, or record it as an open remainder | ROOT assigns |
| D-15 | Rows of W1-selected cases below the floor (§8.1) | **Ruled by ROOT:** no owner question; exact-block is not retired where the successor withholds rows that are Current today; B only as a proof-carrying exemption; C recommended, under the conservative-binding constraint; row-level gate | **As ruled.** C (D2's S-I) is the restorer in every committed domain and precedes F2b and F3's retirement; B (§8.1.2) is mechanical and optional; coexistence rule until then (§4.4) |

**Owner-level.** None, if D-2 is in-repo or `dashu-float`. If ROOT prefers `rug`, then statically linking LGPL-3.0+ GMP and MPFR into the MIT-licensed, signed macOS bundle raises relinking obligations. That is a licence and governance matter for the owner. The options are: avoid it (recommended); ship with LGPL compliance measures; or dynamic linking, which is not how gmp-mpfr-sys builds by default. No protected comparison predicate changes; STAGE0 §5.3 stays as ROOT ruled. R1's net-governed scale for RF-CANCEL is a scale choice within the unchanged predicate, not a predicate change.

## 10. Sources, probes and limits

**Read for revision 1.**
- `T3/{STAGE0_MAP, STAGE1_PLAN, OWNER_DIRECTION}.md`; `T3/TASK_BRIEFS/{_COMMON, D1_NUMERICS_DESIGN, D2_STANDING_DESIGN, R1_REFERENCES}.md`.
- `CORRECTNESS_DESIGN/{NUMERICAL_IMPLEMENTATION, M03_RESIDUAL_SUCCESSOR_ADOPTION}.md`, `CONTRIBUTION_PRECISION/{CONTRACT, INDEPENDENT_REFUTATION, RETURN, SOURCE_QUALIFICATION}.md`, `NUMERICAL_POLICY_REVIEW/RETURN.md`, `COMPOSITE_ENGINE/{SELECTION, RESOURCE_POLICY}.md`.
- `DEFAULT_ROUTE_DESIGN/{DESIGN, ROOT_SELECTION}.md`; `ENGINE_INTEGRATION/RETURN.md` (PHYS-R4, conversion scope).
- SUP-17 in `SOLVER_FINDINGS_ASSESSMENT/_run_records/{SUPPLIED_FINDINGS, supports/RETURN}.md` and `T0_REASSESSMENT/{RETURN, INDEPENDENT_CHECK}.md`.
- `P/validation/benchmarks/numerical_integrity/{README, COVERAGE}.md`.
- Source at `c61a540ea`:
  - `FK/{lib.rs, structural.rs, rigid_body.rs}` and the scope, API and limits of `FK/structural/exact_boundary.rs`;
  - `P/core/solver/sparse_direct/src/{lib.rs outline, structural.rs}`;
  - `SA`; `nonlinear_integration/src/{lib.rs sites, product_equilibrium.rs}`;
  - `performance_harness/{README.md, Cargo.toml}`;
  - `curved_bend/src/lib.rs:241-268`; `linear_supports/src/lib.rs:18-25, 217-228`;
  - `PP` at the sites cited, and `source_recovery.rs` (scope and signed permutation);
  - the CI workflow and `numerical_ci.py`, `desktop-release-template.yml`, and the lockfile inventory.
- From T1: `git diff 82b43f9bd f3270ea79` for `P/core/product_physics`, `P/core/solver` and `P/validation` (stat), plus the `source_recovery.rs` hunks.

**Read for revision 2.**
- `T3/REVIEW/RETURN.md` and `T3/REVIEW/_run_records/v1_stop_rule_probe.*` (check C values);
- `T3/ROOT_RULINGS_V1.md` (at `12f2122cd`, and again at `2d07cad7f` and `4862a72a9` for the S11 rulings and decisions);
- `T3/REVIEW/S11_CHECK.md` (at `56b651282`);
- `T3/MANAGER_NOTES/{V1_DISPOSITIONS, S11_MAP, D2_ON_D1_RETIREMENT}.md`;
- `T3/TASK_BRIEFS/R1_ADDENDUM_CANCEL.md`;
- D2 revision 2 at `d566713e9` (§3.1, §4.9, §4.6.2 interface rows);
- R1's `README.md` at `6c448d260` (not `references.json`).
- Source at `c61a540ea`:
  - `primitive_loads/src/lib.rs:1389-1410`;
  - `source_recovery.rs:520-600`, `:1285-1310`;
  - `source_receipt.rs:130-175`, `:570-667`;
  - `source_receipt/composite.rs:936-948`;
  - `exact_boundary.rs:355-400`;
  - `PP:868-960`, `:1424-1500`, `:1660-1700`, `:1752-1842`, `:2880-2935`, and the force-accumulation sites;
  - `result_export/src/semantic_contract.rs:360-440`, `source_blocks.rs:1225-1245`;
  - `FK/lib.rs:317-366`;
  - `diagnostics/src/lib.rs:285-362`;
  - for S11 revision 2: `SP` (recovery sums), `CB` (`arc_section_resultants_with_radial_pressure`), `load_case_algebra/src/lib.rs:291-350`, `pressure_sum.rs`, `self_weight.rs:470-490`, `FK/structural.rs:60-130`, `:320-420`, `:500-570`, `:940-980`, `FK/rigid_body.rs:225-250`, `exact_boundary.rs:195-240`, `:720-800`, `:1012-1032`, `nonlinear_integration/src/lib.rs:1636-1662`, `PP:2118-2145`, `:2320-2345`, `:7485-7600`, `:7650-7680`, `:7730-7800`, `:7960-8356`; and at T1, `source_recovery.rs:600-670`, `:1262-1280`, `source_receipt.rs:200-225`, `:305-325` (with `git show`).
- Not read: `NUMERICAL_REFERENCE.md` in full, and `DEFAULT_ROUTE_DESIGN/ROOT_RULINGS.md` beyond what `ROOT_SELECTION` and DESIGN quote.

**Read for revision 5.**
- `T3/REVIEW/D5_CHECK.md` (`9f792d741`), `T3/REVIEW/BACKCHECK_R4.md` (`e27181fb2`), V1's `d5_check` probes (read; `probe_d5_check.py.txt` imported by `recal_d5.py` from a scratch copy, sha256 `d13cf7c8…`, recorded in the output); `T3/ROOT_RULINGS_V1.md` through `5a9212de6` (D-15, C, S-H/S11-F, D5_CHECK, BACKCHECK_R4, the main merge, I1's S11-K stop report); D2 revision 5 at `e757f544f` (§4.11, §4.9.10, by D2's summary) and D2's reply on C and B.
- Source at `303609725`: `FK/structural.rs:60-135`, `:300-571`, `:877-1110`; `SA:1-300`; the callers of `solve_structural_dense`/`_sparse` (`SA:280-282`, `nonlinear_integration/src/lib.rs:1933-1936`, `product_equilibrium.rs:122-153`); `PP:1057`, `:1397-1470`, `:1682`, `:1711`, `:1779`, `:2137-2248`, `:2330-2342`, `:2382-2391`, `:3639`, `:5303-5400`, `:5425-5440`, `:8102-8230`, `:10225`; `P/core/runner/headless/src/lib.rs:804`; `preview_physics_runtime.rs:834-1000`; the committed fixtures (component kinds, solver consumption, user-stiffness counts, review-kind units).

**Read for revision 3.**
- `T3/REVIEW/BACKCHECK_R2.md` (at `3ea78add8`); `T3/REVIEW/S11_BACKCHECK.md` (at `61b228543`); `T3/REFERENCE_CHECK/RETURN.md` §3.1, §3.5 and §4 (at `1fd1cb398`); `T3/ROOT_RULINGS_V1.md` (through `af0bb908b` and the F-P2 ruling) and `T3/ROOT_RULINGS_V2.md` (at `3592032fa`);
- D2 revision 3 at `7a6e6f00b`: §4.2.1, §4.5.2, §4.9.3 (G5b, G5c), §4.9.5, §4.9.9, §12.1;
- `REFERENCE_CHECK/v2_floor.py` (V2's floor computation, whose variant B my probe reproduces);
- source at `c61a540ea`: `FK/structural.rs:60-130`, `:540-612`, `:697-760`; `FK/lib.rs:815-885`; the published row kinds in the committed `physics_source` and `source_blocks` raws.

**Ran** (all standard-library Python 3.11.15, `nice 19`; each under 1 s except `floor_kinds.py`, about 10 s).
- `probe_skew_precision.py` → `.stdout.json`: revision 1's evidence, unchanged. Its emulation rounds every operation to p bits with an unbounded exponent. It is not Rust or MPFR, and not a product run. Its scales are a probe convenience, not references.
- `probe_rev2_b1.py` → `.stdout.json`: §3.2 (B1-L, B1-C, B1-E, S8-W). It imports the revision-1 emulation unchanged.
- **Revision 5:**
  - `recal_d5.py` → `recal_d5.json`, `recal_d5.stdout.json` (§4.3.1): 160 R1 cases, both modes, about 2 minutes. It imports V1's `probe_d5_check.py` unchanged from a scratch copy, passed as an argument.
  - `b_proof.py` → `b_proof_main.json` (§8.1.2): every committed exact-block-selected case at the merged tree, in seconds.
  - `s11_exceptions.py` → `s11_exceptions.stdout.json` (§4.10), from R1's frozen `references.json`.
- **Revision 4:** `withheld_rows.py` → `withheld_rows_main.json` and `withheld_rows_t1.json` (§8.1), run over `P/fixtures/product_preview` on main and over T1's changed fixtures extracted with `git show f3270ea79` into a scratch folder; `scan_row_kinds.py` → `row_kinds.json` (§4.1.6.1 item 2), over the same two roots. The D-5 records are listed in `D5_TRIGGER.md` §12.
- **Revision 3:** `floor_kinds.py` → `floor_kinds.json`, `floor_kinds.stdout.txt` (R = 10^9·2^-64) and `floor_kinds_r34.stdout.txt` (R = 2^-34): §4.10's not-covered list. It reads R1's `references.json` at `c0f14201c` (not edited) and V2's per-case L_b from `REFERENCE_CHECK/_run_records/v2_compare.json`, and recomputes V2's variant B exactly (43 RF-WEAK, 3 RF-CANCEL on the net-governed scale) as a check of its basis. S(kind) comes from R1's published expected rows, so for the n = 10000 cases (sampled rows) S is a lower bound, and their member length is bounded by L_b; both can only add flags. Run from `T3/` with `nice 19`, each run about 10 s.
- `probe_s11_audit.py` and `scan_load_fold.py` → `.stdout.json`: `S11_CONTAINMENT.md` (revision 1 evidence, kept).
- `probe_s11_rev2.py` and `scan_element_loads.py` → `.stdout.json`: `S11_CONTAINMENT.md` revision 2 (invariant, mutation, orders, combinations, guard amplification, zero witness; element loads per member).
- `index_probe.py` → `.stdout.json`, and `crates_api_probe.txt`: crates.io metadata. Nothing was downloaded or built.
- Python versions: `_run_records/python_version.txt`. Hashes: `_run_records/SHA256SUMS`, with paths relative to this folder (`DESIGN_NUMERICS/`).

**Limits.**
- **Estimates.** The dense-memory figures (§2.1), the PHYS-R4 scale arithmetic (§4.7), the in-repo arithmetic size and every cost statement are estimates from source and arithmetic, not measurements.
- **Expectations.** The P1 table (§7.2) states expectations only.
- **Probe scope.** One body, at most six members, nodal loads only, dense LDL in emulation. The exact expansions are emulated with `Fraction` sums rounded once, not with TwoSum in `Wide`. No sparse profile at p, Rust arithmetic, budgets or performance is tested.
- **Not a proof.** §4.1.9's argument that the remaining factorization error is precision-dependent is an argument, not a proof.
- **Unverified here.** The `rug` C-build fact comes from crate documentation, not from this host.
- **To confirm.** T4 should confirm the curved null-space claim. R1's references (revision 2) are candidates until V2's narrow recheck and ROOT's selection.
- **Floor probe scope (revision 3).** `floor_kinds.py` uses R1's reference values, not product output, and one body per case. The not-covered list is recomputed by the harness on the frozen references; a difference from §4.10's list is reported, not absorbed.
- **The row-kind mapping of §4.1.6.1** was built from the row kinds in committed raws. D2's G5b treats any unmapped kind as `not_covered`, so a missing kind withholds a quantity rather than overclaiming it.
- **Drift.** Line numbers drift with T1.
- **Not run.** Existing suites, CI, the native app and a DEC-025 sweep.
