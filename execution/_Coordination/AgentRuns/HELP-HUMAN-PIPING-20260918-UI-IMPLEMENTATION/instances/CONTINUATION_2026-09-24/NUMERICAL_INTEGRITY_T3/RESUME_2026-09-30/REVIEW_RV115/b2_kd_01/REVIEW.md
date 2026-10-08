# RV115 (RV-K): independent numerical review of B2-KD (I94's kernel design for B2)

TASK (Type 2), RV115, holding RV-K for B2's kernel work, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance and wrote none of the design. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/RV115_RVK_DESIGN_REVIEW.md`, sha256 `bc13eeef53003c0ad169e08f6022f738fb5f432fbb399f4e17fc9ad3d4d8bb2b`, verified before reading. I read NUM's `AGENTS.md` and `agents/AGENT_TASK.md` first.

**The candidate:** `R/I94/b2_kd_01/DESIGN.md`, sha256 `4e8c33a45616d4f0975aa74f159157ed9c5e0da7f3d306a002646bc704aabed3` (verified; its SHA256SUMS 4 of 4 OK).

**Method.** Documents and code reading at NUM's maintained tree (NUM HEAD `f693bf170f`, whose `P/core`, `P/fixtures` and `P/schemas` trees equal main `2007709549`; `git diff --quiet`, checked), plus one standard-library Python script of my own, run once with VENV in scratch (`evidence/`). No cargo, native or solver job, no install, no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Scratch was `WT/scratch/rv115_rvk/` only.

**Placeholders.** WT, NUM, P, T, R, RR and VENV as in the dispatch. **FK** = `P/core/solver/frame_kernel` (`open_pipe_stress_frame_kernel` in PP's `Cargo.toml`), **FKR** = `FK/src/structural/retained`, **FKT** = `FK/tests`, **PP** = `P/core/product_physics`. DEF-O, SC1, C1–C3, PLAN, REV, RV114, h, N_g and P2 as DESIGN.md defines them. "The design" is I94's DESIGN.md; "§n" is its section.

## Verdict

**ACCEPT WITH AMENDMENTS.** 0 BLOCKING, 4 SHOULD-FIX, 10 NOTE.

The certificate is sound. I re-derived it, including the R7 transfer and the fixed-anchor uniqueness, and every premise holds for a combination owner. Under the design as specified, nothing published for a combination can lie outside its enclosure. The kernel facts, the S-2 decisions and both code constraints hold in the code, and the API is additive.

The amendments are confined to custody and tests, and none needs a second design round:
- one capacity predicate is stated loosely enough to admit a reservation overflow (SF-1);
- the oracle never exercises one of the two load-read sites that B2-K changes (SF-2);
- one oracle claim (C2) and one mutation control (K-10, C6) do not discriminate what they say (SF-3, SF-4).

## Findings

| ID | Severity | Where | Finding | Required change |
|---|---|---|---|---|
| **SF-1** | SHOULD-FIX | §1.2 (second note), §2.2 (b1), K-02 | **The "dormant check" does not stop a prepared ordinal being spent as a batch case.** §1.2 says the check is "case runs never exceed runs". Counterexample (evidence §7): `for_invocation([2],[2],1)` gives cases 3, runs 3, combinations 1.<br>• A batch of 3 passes `used_cases + 3 ≤ cases` and that predicate.<br>• The combination then passes today's checks (`origins.rs:422–427`) and needs a fourth Run against three reserved.<br>• As a result, `runs`, `groups` and `selected` grow past their reservations, which is a silent reallocation.<br>• `RunTrace::requested`'s `assert!(id < self.builds.capacity())` (FKR/origins.rs:634) can also panic once a run builds past 7·runs.<br>PP's intended order (batch, then registrations) never reaches this, but custody is meant to protect itself ("Exact maxima … No default allowance", origins.rs:24). | **State the predicate exactly.** A sufficient check, dormant under `for_calls` (proof in §2.2; evidence §7): at every case batch, `runs.len() + n + (combinations − used_combinations) ≤ capacity.runs`, else `Capacity` with nothing recorded.<br>**Add to K-02:**<br>• a batch that would spend a prepared ordinal is refused;<br>• a registration that takes a batch ordinal before the batch makes the batch refuse;<br>• no recorded registry ever exceeds its reservation (no panic, no reallocation).<br>**Add the invariant to the stop list** beside S-4. |
| **SF-2** | SHOULD-FIX | §6 operand table; K-08, K-10 | **No specimen exercises the reaction-offset site.** Every specimen term is at the free tip, and none is at a constrained DOF. So the reaction-offset site (FKR/product_certificate/source_residual.rs:1161–1169) is never discriminated, although it is one of the two load-read sites B2-K changes (§3.3). On all of C1–C6, reading the representative's loads there leaves every row unchanged.<br>K-10's "representative loads" mutation is caught only at the free-row site (by C1, C2, C4 and C5). Constrained-DOF terms are ordinary in product models (end-equivalent and self-weight terms at anchors), and the case path already treats them (I43 §4; I44's constrained-load controls). | **Add an operand that has a constrained-DOF term the representative lacks.** For example, R: Fy = 5 at the restrained root, used in A + R.<br>Also add:<br>• a cross-operand cancelling pair at the root;<br>• the oracle's exact root-reaction truths for these;<br>• a K-10 mutation "representative's loads at constrained DOFs". |
| **SF-3** | SHOULD-FIX | §6, C2 row ("exact products") | **C2 does not discriminate exact products.** Every product c_i·v_ij in C1–C6 is exact in binary64 (evidence §3). A ledger that formed fl(c·v) would therefore pass all six specimens, and C2 does not test what its row claims. | **Add one product that needs more than 53 bits.** For example, factor 0.1 on B's Fz = 3: the exact product is 10808639105689191·2^-55 (54 bits), and fl(0.1·3) differs from it by 2^-54 relative. Give its exact net and its K4LED bytes. Containment then fails a binary64-product kernel by about 2^950. |
| **SF-4** | SHOULD-FIX | §7, K-10 (third mutation) | **K-10's C6 mutation cannot fail at the row level.** K-10 says "nearest instead of outward net rounding (C6) … fails containment or a predicate". The omitted 2^-600 is:<br>• 2^-177 of one K-lane 1024-bit outward step at \|N\| = 2^600;<br>• 2^-689 of the G lane's π width there (evidence §8).<br>So no row enclosure or predicate can see it. It is visible only at the net enclosure, where RN1024(N) = RD1024(N) = 2^600 excludes N = 2^600 + 2^-600. | **State this control at the net level,** as K-08's assertion: lo ≤ N ≤ hi with lo < hi, while the nearest-rounding mutation gives lo = hi = 2^600, which excludes N. Drop the row-containment wording for C6. |
| N-1 | NOTE | §5.4, E5 | E5 verified: δ_g ≤ 2^(⌊log2\|N_g\|⌋−1023), and δ_g = 0 exactly when N_g has ≤ 1024 significant bits. The evidence:<br>• 4,010 random nets (sums of 1–6 binary64 products over the whole exponent range, binade crossings included), with 0 violations;<br>• C6's width is exactly 2^-423.<br>The comparison "never wider than a case's would be" is not a premise of the proof. | None |
| N-2 | NOTE | §1.1, S-4, S-5, K-14 | `CombinationReason` is stored inline in `CallResult::PreSourceRefusal` inside `CallOrigin` (atom `CALL_ORIGIN`, FK/src/structural/retained_resource.rs:82, used in PP's `RegistryE` atom, PP/src/retained_memory.rs:1770). A unit variant is very unlikely to change that size, but only K-14 proves it. S-5 names PP only, while `performance_harness` imports `CombinationReason` (FK's sibling crate, `tests/k6b_export.rs:20`; it does not match on it). | At J3, also compile every FK dependent in the workspace |
| N-3 | NOTE | §3.2 step 2 | P2's `terms[i].1 == 0.0` admits −0.0. That is harmless: `exact_publication` gives +0, and `gather`, `motion` and the InputDerived check compare by value. | Optional: compare bits with +0.0, matching DEF-O's "exact_positive_zero" literally |
| N-4 | NOTE | §1.4 step 4 | Because step 4 precedes custody, an all-prepared request with a bad source id gets a pre-source refusal (per-combination `retained_unavailable`) rather than an origin refusal (whole-successor abandonment, decision 7). PP cannot reach this (decision 5). C2's operand-validation stage lists three reasons (CONTRACT_DELTA line 127); it now has four. | B2-C maps `no_selected_operand` under operand_validation |
| N-5 | NOTE | R-1 | Reusing `MissingSelectedOrigin` is safe only through decision 7. `CaptureError::Origin` is serializable in a product-attempt failure (PP/src/retained_wire.rs:592, :1212–1234). | B2-C states that a combination's origin refusal never becomes a capture error. FK's doc comment widens the variant's meaning |
| N-6 | NOTE | §1.2, K-02 | `for_calls ≡ for_invocation(·,·,0)` must keep `OriginError::CountRange`'s detail strings ("case runs", "operands", "calls", "runs", "physical records", "builds"), because PP serializes them (retained_wire.rs:575) | Include them in K-02's equality |
| N-7 | NOTE | §6 diagnose | The diagnose (as I44's does) trusts the kernel's printed scale S*. Its output list also omits stress truths, which cover 20 of the 50 per-member rows. | Recompute S* from the published rows. Add exact stress truths so that K-09's containment is end-to-end |
| N-8 | NOTE | §3.2, K-07 | RV56-F1's lesson applies to the new P2 branch: book its visits before it refuses | Test the refusal's work prefix in K-07 |
| N-9 | NOTE | §6 | C6's 1201-bit net comes from a single operand's own terms. A cross-operand wide net (for example 2^600·A + 2^-600·A2 at one DOF) would also exercise `RetainedLedger::combined`'s exact cross-operand sum in the E5 case | Optional |
| N-10 | NOTE | §5.2 | Notation: in 7n + 50m + 8g, n is nodes and g is the support count (g is a DOF index elsewhere). The count assumes three stations per member, which `row_scales`' completeness already forces | None |

## 1. The certificate is sound (item 1)

### 1.1 Re-derivation for a combination owner

**Setting.** Let O be a selected combination run with:
- representative source S (operand 0's `PrimitiveSource`, cloned into the combined `CasePrep`);
- combined exact ledger N, where N_g = Σ_i c_i Σ_{j: dof(ij)=g} v_ij (K4LED, `RetainedLedger::combined`, FKR/ledger.rs:209–229);
- every prescribed term zero (P2).

For each law L ∈ {K, G}, built on S's members, frames, springs and constrained set C, the exact problem is L_FF u_F = N_F with u_C = 0. Under P2 the coupling term L_FC u_C vanishes exactly.

**(a) The perturbation bound (the R7 transfer), re-derived.**
- The view requires, for every data block's body:
  - a finite positive `certified_bound` B;
  - θ ∈ [0, ½] (FKR/adaptive.rs:5544–5566).
- The native verification computes θ = B·‖S Ā S‖·2^(7−p) (FKR/verify.rs:1055–1059; `sas` = ‖SĀS‖, :570–571). R7's formation premise is that 2^(7−p)·Ā majorizes, entrywise, the difference between the exact K and the matrix whose inverse B certifies. I take that constant from R7 and did not re-derive it.
- So ‖S(K − K̃)S‖∞ ≤ θ/B, and the Neumann series gives ‖(S K_FF S)^-1‖∞ ≤ B/(1 − θ) ≤ 2B = β on the block. The ∞-norm bound transfers to the 1-norm by symmetry.
- Every input is stiffness, ordering, scale and precision: no load enters.
- For O, B and θ come from O's own verification report (`evidence.certified_bound` and `evidence.theta`, FKR/adaptive.rs:4906). The view and the native verification decide which blocks need B by the same predicate:
  - FKR/bound.rs `data_blocks` and `fill_data_blocks`;
  - inputs: O's ledger flags, the prescribed signs (all zero under P2), and O's own state.
- A data block without B refuses (`BodyBound`), so it never yields a wrong number.

**(b) Fixed-anchor uniqueness (RV56), re-derived.**
- For a straight member in its exact orthonormal frame, the energy is zero exactly when the six basic strains are zero. The axial and torsion coefficients are positive, and each bending block is [[4,2],[2,4]]·EI/L with eigenvalues 2 and 6. So the member moves rigidly.
- A body connected through members, with one node fully restrained, therefore has a positive definite free stiffness, by induction over the member graph.
- Positive springs only strengthen this, and positive coefficient changes keep the same null space. So the result holds for K and for every G in the positive coefficient box.
- No-data blocks are closed: the view checks that no free pattern entry crosses a block (FKR/adaptive.rs:5409–5422). On a no-data block, every individual product at its free DOFs is zero, there is no adjacency to a nonzero prescription, and the state is zero. So the block's right-hand side is exactly zero, and u = 0 there.
- None of this reads loads, except the individual-product flag. For a combination that flag is `factor != 0 && value != 0` (ledger.rs:225), so cancelled nets stay data.
- `anchored` reads only `constraint(g).is_some()` (source_residual.rs:644–658), and the constrained set is common through K4STF.

**(c) I43's theorem with N.**
- Take any finite center y, with y_C = 0 (`gather` returns S's constraint value, which P2 forces to zero).
- Then e = u^L − y has e_C = 0 and L_FF e_F = N_F − L_{F,all} y_all = r.
- Scaling gives (S L_FF S)z = S r, with z = S^-1 e_F.
- Blocks do not couple through free entries, so per data block c: ‖z_c‖∞ ≤ β_c ω_c/(1 − α_c), where ω_c ≥ max|S r| on the block. The bound holds whenever the enclosure ρ ⊇ S r is outward.
- The only change from the case path is the load term. [RD1024(N_g), RU1024(N_g)] ∋ N_g by directed rounding: `round_toward` rounds to nearest, takes the exact sign of the remainder, and steps one ulp (FKR/directed.rs:65–87). So ρ stays outward.
- Recovery is the same functional of the displacement box. Reactions are (L u)_C − N_C, with [N_g] subtracted at each constrained g (§3.3). That is exactly the native reaction law (FKR/recover.rs:419–426 uses `ledger.add_to(g, sum, true)`).
- The hull and DEF-O's predicates then bound each published value. The proof holds for any center, so the correction's and the native solve's accuracy are not premises.

**(d) The premises, checked one by one against I43 §3 and §4 and I44.**
- Immutable owner: O's `RetainedSolve`, through `product_owner`.
- Source and ledger identity:
  - `validate_publication_owner_spent` checks K4CMB against `prep.identity` and `evidence.source_encoding` (FKR/adaptive.rs:3756–3783);
  - `encoding_matches` checks K4LED against `evidence.ledger_encoding`.
- Maps, frames, springs and supports: common through `validate_operands` (FKR/combine.rs:173–194), that is, K4STF bytes, layout, stations and supports.
- Group: from the first selected operand. Groups are keyed by K4STF (`solve_cases_projected`; `prepare_group`, FKR/adaptive.rs:4932–4960, reads only stiffness).
- The verification factor at P: O's own cache slot, checked by `checked_slot!`.
- Rows, radii and classes: O's own publication.
- Positive source laws: the member-fact checks against S (FKR/product_certificate/final_case.rs:1052–1068).

All of these hold.

### 1.2 E5 and E6

**E5.** Wide<16> at 1024 bits has a 1024-bit significand and an exponent range of ±2^62 (FKR/wide.rs:32). So:
- for N_g ≠ 0 with e = ⌊log2|N_g|⌋, one ulp is 2^(e−1023), and RU − RD ≤ 2^(e−1023) ≤ 2^-1023·|N_g|;
- crossing into the next binade does not change the width;
- the width is 0 exactly when N_g is representable (≤ 1024 significant bits).

The exact sum spans at most 4,196 bits for nets of binary64 products, against the 8,128-bit limit (FKR/wide_sum.rs:21–29). So the net enclosure never refuses on span. Checked numerically (evidence §2):
- C6: RD = 2^600, RU = 2^600 + 2^-423, RN = 2^600 ≠ N;
- 0 violations in 4,010 random nets.

**E6.** E6 is exactly 0 under P2.

### 1.3 No operand quantity enters the proof

The proof reads:
- O's prep (combined ledger, S's stiffness data, combined prescribed terms);
- O's group;
- O's cache slot at P;
- O's evidence (B, θ, scales, radii);
- O's publication.

No operand's radius, row, state, verification report or certificate is read. The cache slot may be imported from a selected operand, but it is stiffness-only. It enters through the correction, which is not a premise (any center works), and through O's own verification, which recomputes B and θ for O's data blocks. Operand 0's loads and constraint values are read by exactly two sites (source_residual.rs:623 and :1161), which §3.3 replaces, and by `constraint(g)` (`gather`, bridge `motion`), which P2 makes exactly zero. I found no other reader in `product_certificate/*` (grep).

### 1.4 The row set and the coverage rule

**Native layout** (FKR/recover.rs `layout`): 6·nodes + nodes + 12m + 6·stations + springs + constraints + 2·supports. That is 52 for the one-member specimen (evidence §6, matching I44's 52).

**`row_scales`** (final_case.rs:1114–1288):
- stress slots are j = mi·21 + site·4 + component, with sites End I, End J and the stations at 0.25, 0.5 and 0.75, so slots 0–19;
- the circular maximum is slot 20 (:1235, :1250);
- completeness requires every native row, every derivative slot and the mode record (`nonquantity`).

A case therefore has 7n + 51m + 8g final rows plus its records (73 for the specimen). A combination without maxima has 7n + 50m + 8g (72).

**PP's current mechanics combination publication** (PP/src/preview_physics.rs:853–1006) has:
- the `LINEAR_STATE_KINDS` rows (pressure and constant-effort kinds are outside DEF-O's scope);
- the combined vector magnitude;
- six signed support components and two support magnitudes;
- no maximum (`COMBINATION_STRESS_MAXIMUM_UNAVAILABLE`), no intensified row, and no mode or parity record.

The modulus record appears only for subtraction and range (PP/src/lib.rs:10019). §5.2's rule matches this:
- the four families are refused;
- slots 0–19 are required, and slot 20 stays empty;
- no mode record is required.

S-14 covers a later change. Maxima rows do not feed S*, so S* formation is unchanged.

### 1.5 Could anything published be outside its enclosure?

**No, under the design as specified:**
- P2 is enforced by the view;
- nets are read at both load sites;
- the owner is admitted by `product_owner` with the I9 kind check;
- the coverage branch refuses unexpected rows rather than publishing them uncovered.

The residual risk is implementation error at the two load-read sites and in the ledger's exact products. SF-2 and SF-3 give the tests the power to catch both. What the certificate does not prove (§5.6) is correctly scoped: the binding to the authored expression is G8's and PP's.

## 2. Kernel facts and S-2's decisions (item 2)

### 2.1 (a) Rebuild, not retain, with a full K4SRC identity check: holds

- **No prep in the failed outcomes.** `ExecutionOutcome::{Refused, Unresolved}` hold attempts and geometry only (FKR/adaptive.rs:4404–4416).
- **The rebuild cannot fail differently.** `CasePrep::new`'s only failure is the ledger's (:948–956), and it is deterministic.
- **K4SRC equality means the same source.** K4SRC (FKR/source.rs:777–808) encodes every authored field of `PrimitiveSource`: nodes, members with E, G, A, Iy, Iz, J and y_ref, springs, directional springs, constraints with their values, loads with DOF, source_id and value, stations and supports. `constrained` and `body_of_node` are derived. So equal K4SRC bytes imply an equal prep.
- **The registered identity is the same bytes.** The registered `SourceOrigin.identity` is `source.encoding()` (origins.rs:367–373).
- **What rejecting retention keeps.** It keeps `ExecutionOutcome` and the `RECORDED_CASE` atom unchanged.

### 2.2 (b) Registration through the existing `cases` count: holds, with SF-1

`OriginCapacity` (origins.rs:25–33) and `RecordedInvocation` (:301–308) gain no field.

**Equivalence with prepared = 0.** `for_invocation(a, b, 0)` gives the same calls, cases, combinations, operands, runs and builds as `for_calls(a, b)`. Under `for_calls`, the new reservation for sources (cases + combinations) equals today's (runs) (evidence §7). N-6 adds the detail strings.

**SF-1's proposed check is dormant under `for_calls`.** At any case batch of n:
- runs.len() ≤ used_cases + used_combinations;
- used_cases + n ≤ cases.

So runs.len() + n + (combinations − used_combinations) ≤ cases + combinations = runs.

**Reservations under `for_invocation` with that check:**
- **sources:** batch sources and registrations share `cases`, and combination sources fit in `combinations`;
- **runs, groups and selected:** each batch keeps room for every remaining combination;
- **builds:** stay within 7·runs.

**The shared counter.** It means the declared prepared count is enforced jointly with the batch ordinals. That is exact under PP's order, and the batch-side check makes it safe under any order.

### 2.3 (c) The first selected operand's group: holds

- **The group depends only on stiffness.** `GroupPrep` (structure, ordering, geometry, blocks) comes from `prepare_group(source)`, which reads only stiffness data. `solve_cases` already shares groups by K4STF (FKR/adaptive.rs, the `stiffness_encoding()` key in `solve_cases_projected`). So every operand's group would be equal, and the view re-validates ordering, blocks and pattern against S anyway.
- **Operand 0 stays the representative.** `CasePrep::combination` reads operand 0's `constraints()` only for the DOF set and each operand's own `constraint(g)` for the values (:990–1001), and reads loads only through `RetainedLedger::combined`.
- **The native solve never uses operand 0's loads or constraint values.** It reads `prep.ledger` and `prep.prescribed` only:
  - the reduced right-hand side (FKR/assemble.rs:826–841);
  - reactions (FKR/recover.rs:419–426);
  - the verification residual and its prescribed signs (FKR/verify.rs:799–850).

## 3. The two code constraints (item 3)

**The layout.** FK/src/structural/retained_resource.rs exports `size_of` of `CasePrep`, `RecordedInvocation`, `RetainedSolve`, `RecordedCase`, `CallOrigin`, `SourceOrigin`, `GroupOrigin`, `BuildOrigin`, `RunOrigins`, `ProductOwnerStamp`, `ProductCertificateSpent`, `ProductRowSpec`, `ProductFinalRow`, `ProductMemberFacts`, `LedgerNet` and others. PP builds its atoms from them:
- `ArcCasePrep` at :1656;
- `RecordedInvocation` at :1768;
- `RegistryE` (`CALL_ORIGIN` + `SOURCE_ORIGIN` + …) at :1770.

The design adds no field to any of these or to anything stored inline in them. The new types (`PreparedCaseSource`, `CombinationOperand`, `RecordedOperand`) are not stored in any exported type.

**PP's exhaustive matches.** I verified them on FK's enums:
- `ViewFailure` (PP/src/retained_wire.rs:468–479);
- `BridgeFailure` (:482–495);
- `ProductFailureView` (:507–519);
- `OriginError` (:573–579);
- `ReadoutLaw` (:1243);
- `NativeOwner` and `GroupPreparation` (:1385–1389).

The design adds no variant to any of them: `UnsupportedCombination` stays, and R-1 reuses `MissingSelectedOrigin`. The view and residual refusals reuse `PairIdentity`, `UnsupportedCombination` and `bad(..)`.

**Additive API.**
- `solve` keeps calling `solve_recorded(..).into_legacy()`.
- `solve_combination` maps each operand to `Selected`. For all-selected input, step 4 cannot fire, merged caches and groups are operand 0's as today, and the check order is today's. So both are byte-identical wrappers.
- `CombinationReason::NoSelectedOperand` is the only new variant. N-2 covers its inline size.

**Does PP match `CombinationReason` exhaustively? No.** PP does not reference `CombinationReason` at main or on `b1` (`603e238517`, which changes no file under `P/core/solver`). PP serializes no combination call result today: `invocation_arrays` writes every call as `case_batch` with result `runs`, so B2-W and B2-C add that.

Outside PP:
- FK's tests use `assert_eq!`;
- `performance_harness/tests/k6b_export.rs` imports `CombinationReason` without matching on it.

## 4. The source view (item 4)

**The bridge (I42).** `bridge.rs` reads no loads. Its reaction offsets cancel between u_K and u_G (RV56 DERIVATION), and `motion` and the InputDerived check read |constraint(g)| and constraint(g), compared by value (bridge.rs:156–174, :695–706). Under P2 both are exactly zero, so the bridge needs no change.

**Tightening (I43).** `source_correction` applies O's P-slot factor only (FKR/adaptive.rs:5663–5773): no change.

**The residual (I44).** It has exactly two load-read sites:
- free rows (source_residual.rs:622–629);
- reaction offsets (:1161–1169).

§3.3 replaces both with O's nets. The reaction site has no test (SF-2).

**`UnsupportedCombination`.** The view's blanket refusal (FKR/adaptive.rs:5359–5361) is lifted for P2. The variant and its wire tag are kept for any nonzero prescribed term, including one in a non-representative operand, which C06's FK part tests.

## 5. The oracle, N-1 (item 5)

**Independence: adequate as specified.**
- Standard library only, with no import of production code and no copy of kernel arithmetic.
- An exact Fraction element solve and a separate π bracket.
- A diagnose step that reproduces predicates from published bits.

This is the I41 and I44 practice. The author is B2-K's implementer, so RV-K's subset re-derivation is the independence control. I have started it (evidence):
- my own nets, with I94's C1–C6 values matched exactly;
- K4LED canonical forms and the sha256 of each K4LED (tip node 1), for the code round;
- the tip flexibility by curvature integration (unit-load method), not by stiffness inversion, confirming uy/Mz = rz/Fy = +L²/(2EI) and uz/My = ry/Fz = −L²/(2EI) at L = 1 and 2;
- C3's discriminator with my own binary64 EA: the operand rows sum to exactly 0, and the truth is 2^-60/EA, about 2^-89.

**Coverage of the certificate's new terms:**

| Term or path | Specimen | Discriminates? |
|---|---|---|
| E5, nondegenerate (1201-bit net) | C6 | At the net level only (SF-4) |
| 2^±700 factors, nets far below the products | C5 | Yes |
| Prepared first operand; group from operand 1 | C4 | Yes |
| Cancellation; cancelled DOFs stay data | C1, C3 | Yes |
| Free-row load site | C1, C2, C4, C5 | Yes |
| Reaction-offset load site | none | **No (SF-2)** |
| Exact products, not fl(c·v) | C2 claims it | **No (SF-3)** |
| Final stress rows against the truth | not in the outputs | Partly (N-7) |

## 6. The stop list and the estimate (item 6)

**S-1 to S-14 are sound.** S-13's file list keeps `ledger.rs`, `bound.rs` and `verify.rs` frozen, which also protects the data-flag predicate.

**Missing:**
- SF-1's reservation invariant: no recorded registry exceeds its reservation, with no assertion panic and no fallback reallocation;
- N-2's compile of every FK dependent at J3.

**The estimate.** 18–28 h is plausible. The amendments add about 1–2 h:
- operand R and an inexact product in the oracle;
- the capacity test.

RV-K's code round of 5–8 h, plus about 1 h for the oracle's independence, stands.

## 7. I94's points R-1 to R-11 (item 7)

| Point | Position | Reason |
|---|---|---|
| R-1: reuse `MissingSelectedOrigin` for a prepared operand's custody refusal | **AGREE** | A new variant breaks PP's exhaustive `origin_error` (retained_wire.rs:573–579). Under decision 7, an origin refusal abandons the successor. Condition (N-5): B2-C states it never becomes a capture error, and FK's doc comment widens the meaning |
| R-2: P2, with nonzero terms kept as `UnsupportedCombination` | **AGREE** | DEF-O binds `exact_positive_zero` and excludes `nonzero_imposed_motion`, so no in-scope operand loses anything. P1 would change `gather`, the bridge, the InputDerived rows and the data flag |
| R-3: `NoSelectedOperand` | **AGREE** | The group must come from a selected operand (decision 5). This is a guard PP cannot reach. Its size is covered by S-4 and K-14 (N-2), and B2-C adds the tag (N-4) |
| R-4: layout and enum neutrality as stops, with K-14 | **AGREE** | Verified (§3). Add the compile of every FK dependent (N-2) |
| R-5: the declared re-pin of `source_residual_combination_and_missing_uniqueness_are_explicit_refusals` | **AGREE** | Its combination half (FKT/retained_k4/source_residual_tests.rs:787–804) asserts `UnsupportedCombination` for a 1·s combination that P2 admits. No other FK or PP test pins that refusal (grep). PP's wire test only enumerates the variant, which stays |
| R-6: lane K's write set as S-13 | **AGREE** | SF-1 and SF-2 land inside it (`origins_tests.rs`, the oracle files, `source_residual_tests.rs`) |
| R-7: the combination coverage rule | **AGREE** | Verified against PP's mechanics publication and `row_scales`' slots (§1.4). B2-C confirms the row set, and S-14 holds |
| R-8: operand facts equality in DEF-C | **AGREE** | K4STF carries E, G, A, Iy, Iz, J and the frames, but not D, t_eff, Z, c or the material selection. The certificate is valid for O's own facts either way, so R-8 protects meaning (one physical law across the operands and the combination) and must be bound |
| R-9: the public E/ν route for B3 | **AGREE** | `ProductMaterial` has `Base`, `Point` and `Interpolated` only (final_case.rs:20–43). `MaterialOperands::ExactENu` is `pub(super)`, with checked −1 < ν < ½ and directed G = E/(2(1+ν)) (product_certificate.rs:46, :416–431). PP's matches on `ProductMaterial` have wildcard arms |
| R-10: DEF-C's definition hash domain | **AGREE** (recommendation) | A domain names an object type, and the ids differ. There is no numerical consequence, so this is ROOT's naming decision at B2-C |
| R-11: a freeze-failed operand passed as `Prepared`, with the kernel accepting a prepared operand whose source has a selected run | **AGREE** | The rebuild is deterministic and K4SRC-checked, so the numbers are identical. The cost is the lost cache import (a possible rebuild charge). This matches decision 6 |

## 8. B3b's answer and the E/ν gap (item 8)

**B3b's answer: AGREE.** No exact annulus version is needed for anything in B2:
- `pressure_runtime` refuses every combination on the exact route (PP/src/pressure_runtime.rs:201–203, `EXACT_PRESSURE_COMBINATION_UNSUPPORTED`);
- `prepare_product_annulus(diameter, effective_wall)` (product_certificate.rs:730) is material- and route-independent.

B3-D must still confirm the exact producer's section bits, as §8 says.

**The E/ν gap: confirmed.** It is reachable only by adding a public variant (B3-K). `ProductMemberFacts`' size is governed by `Interpolated` (2 usize + 9 f64), so a two-f64 variant should not grow it. K-14's rule applies.

## 9. Basis, execution record and limits

**Read** (sha256 prefixes at the time of reading):

| Input | sha256 |
|---|---|
| NUM `AGENTS.md`; `agents/AGENT_TASK.md` | `f96feb19d297c74e`; `1a13a5b00b3ce01f` |
| The brief | `bc13eeef53003c0a` |
| DESIGN.md; its `b2kd_checks.py` and its output | `4e8c33a45616d4f0`; `4110be14ebe67a38`, `29de2771441c268c` |
| B2-KD's brief | `c7b84097648369c5` |
| PLAN; REV; RV114 | `e1147dbda247c470`; `63abb73fe85c8dbc`; `bc6918ce474a188d` |
| SC1; C2 | `28bc4dd617d3d929`; `923da0b97eb5beca` |
| I43 RETURN; I44 RETURN | `cf3b682b0c6dddb2`; `8a1ef094bd68d410` |
| RV56 REVIEW; DERIVATION | `9add028888f4a5c4`; `c7a2b54e4334bb2b` |
| I42 RETURNs | read by grep (combinations, prescriptions, ledger) |
| RR | the sections "B2/B3 R1…", "I93's REVISION_01 accepted…" and "I94's B2-KD design returned…". RR is append-only and moved while I read it; `0ce52867113e1ae2` at the last read |
| DEF-O | `3e0779a45a74cf0b` |
| FKR `combine.rs`, `origins.rs`, `adaptive.rs`, `ledger.rs`, `source.rs` | `730b7d24f9180aad`, `c4bde5de1961f736`, `6a2fc382bf8cae05`, `20c3b86a834c0657`, `9956c08421eebb44` |
| FKR `bound.rs`, `verify.rs`, `assemble.rs`, `recover.rs` | `bdffbeb81c54ed99`, `66022cc78bb5779d`, `7f993858d4a17882`, `e452e3467608a468` |
| FKR `wide.rs`, `wide_sum.rs`, `directed.rs`, `product_certificate.rs` | `1c6b773dfd4c3668`, `dc4f210bf5d6bdc3`, `031457491dcb9c2a`, `64e09224aacdc328` |
| FKR `product_certificate/bridge.rs`, `source_residual.rs`, `final_case.rs` | `0cbc204925748ecb`, `3f66c03f568418c5`, `d4dbaf3cb455dd50` |
| FK `src/structural/retained_resource.rs` | `f8471544f05df04f` |
| FKT `s11_site_table.rs`, `retained_k4/source_residual_tests.rs`, `retained_k4/product_certificate_vectors.py`, `retained_k4/source_residual_vectors.py` | `2ee1079dd7ad1fc4`, `eadf3bcae3497496`, `88fc0765939e9dff`, `5242da1e85cff920` |
| PP `src/retained_wire.rs`, `retained_memory.rs`, `preview_physics.rs` | `4f383c04752baaa8`, `fbc7c9db1e8aae3e`, `2b8265403dd50613` |
| PP `src/lib.rs`, `pressure_runtime.rs`, `retained_product.rs` | `4c33c25031622db8`, `5a4f07f40ea3b08e`, `f536bfe786684baa` |

The FK, FKT and PP prefixes equal the ones I94 cited, so the design and this review read the same code.

**Executed** once, with VENV's Python 3.13.14 (`evidence/RUN.md` has the command with placeholders):
- the script `evidence/rv115_checks.py`, standard library only, reading only I94's run output for the comparison;
- exit status 0;
- output in `evidence/rv115_checks.out.json`.

**Limits:**
- **No code was compiled or run.** Byte identity, layout neutrality and pass counts are for B2-K's runs and RV-K's code round.
- **Partial re-derivation.** I did not re-derive:
  - R7's formation constant 2^(7−p) (only the Neumann transfer from it);
  - DEF-O's projection, its predicates or the stress recipes.

  These are load-independent or owner-generic, and the design leaves them unchanged.
- **Bounded sweep.** My random E5 sweep is a check, not a proof; the proof is in §1.2.
