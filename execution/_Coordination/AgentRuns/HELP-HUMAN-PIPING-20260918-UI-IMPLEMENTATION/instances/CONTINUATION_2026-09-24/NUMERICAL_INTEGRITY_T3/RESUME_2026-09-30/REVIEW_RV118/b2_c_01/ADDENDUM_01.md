# RV118 (RV-C) ADDENDUM_01: confirmation of I97's B2-C revision 01

TASK (Type 2), RV118, holding RV-C for B2, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

This addendum answers the coordinator's request: "please confirm I97's B2-C revision 01 against your review and ROOT's rulings". It also takes in the coordinator's later message about RR "RV115 confirms S-4 (a)'s soundness with SA3-1; …", the option (ii) ruling. The sealed `REVIEW.md` and `SHA256SUMS` are untouched (7 of 7 still OK).

**The subject:**
- `R/I97/b2_c_01/REVISION_01.md`, sha256 `6f6a583f601fa4f6111001196eeb1488a71f674547ff12bb109fc7cddcea38a1` (verified). `SHA256SUMS.revision_01` is 10 of 10 OK, and the sealed `SHA256SUMS` 13 of 13 OK.
- The revision's brief, `R/BRIEFS/B2C_REVISION_01.md`, is `2afb5904…2b74` (verified).

**What I read:**
- the revision and its brief;
- my REVIEW.md;
- RR's rulings:
  - "RV118 (RV-C) accepts B2-C with amendments; …";
  - "RV115 (RV-K) accepts DEF-C's numerical content";
  - "I98's B2-W verified; …";
  - "I91's item 4 verified and `b1-p` pushed; I97's B2-C revision 01 verified; …";
  - "RV115 confirms S-4 (a)'s soundness with SA3-1; …";
- RV115's `ADDENDUM_02.md` (`7cd7a1b4…`) and `ADDENDUM_03.md` (`4bd3e234…`; its sums are 4 of 4 OK);
- I98's `PROBE.md` (`e224899a…`);
- I94's KD §5.2, §5.7, §6 and §9.2;
- the code sites in `addendum_01/checks_addendum.txt`.

**Method.**
- **Reproduction.** I ran I97's three r1 scripts unchanged, from copies in my scratch (each copy `cmp`-equal to the record). Their outputs went to scratch.
- **Two scripts of my own**, standard library only, each run twice with byte-identical output:
  - `rv118_r1_checks.py`: hashes, leaf deltas, S-2's layout and S-4's guard;
  - `rv118_a01_option_ii.py`: what option (ii) touches in the items I confirm.
- **Code reading** at NUM and on B1's four branches.
- **Limits on execution:** read-only Python with VENV; no cargo, native job, install or Git write.
- **NUM moved** from `d1d6517455` to `827dc41c2e` while I worked (records only). P's `core`, `fixtures`, `schemas`, `apps` and `tests` trees are equal at both heads (`git diff --quiet`).

**Notation** is REVIEW.md's. "§n" is REVISION_01's section, and "C§n" is CONTRACT.md's. "(ii)" is the ruled option: RN64 of the exact 3-norm of the frozen published components.

## Verdict

**CONFIRMED. 0 BLOCKING, 1 SHOULD-FIX, 5 NOTE.** The SHOULD-FIX is routing only: it needs no change to revision 01's statics and no change to J1's SCHEMA.

**Every amendment and NOTE is applied as ruled:**
- S-1 to S-6;
- N-1 to N-12, N-14 and N-15 (ROOT declined N-13, and the revision rightly omits it);
- C-4 as an amendment of C3a rule 4;
- I98's witnesses;
- RV115's NB-1 and NB-3.

**I accept all eight §9.2 choices** (§2 below).

**§10 is consistent:**
- 19 bases and 69 mutations; every producer-solved and hook-produced base is in D1.4, and the two synthetic bases are labelled;
- every designed first failure is reachable under S-6's seven-step rehash, with m10–m12 as `after_rehash` edits.

The witnesses match I98's, and D6b is case-only.

**J1.** Revision 01's reviewed inputs are SCHEMA `abf3225c…`, PTABLE `791c0a0d…` and DEF-C `6467c733…`, and the names are reserved per §7. **Under ROOT's (ii), however, J1's PTABLE and DEF-C hashes move with revision 02.** SCHEMA does not move: it carries no definition or table hash, and (ii) changes no id. So B2-C becomes final for J1 when revision 02 is confirmed, as RR's (ii) ruling already orders.

**Option (ii) and the items I confirm.** (ii) leaves unchanged:
- the observables stage (§1.2);
- §10.1's G7 row;
- m69;
- the witnesses' expectations.

G7's guard still holds by construction. My model gives at most 3 ulps, 3/64 of the allowance; RV115 gives 2.5 ulps. (ii) does sharpen A-1: the exact 3-norm is new code in FK, not in PP.

## Findings

| ID | Severity | Where | Finding and evidence | Required change |
|---|---|---|---|---|
| **A-1** | SHOULD-FIX (routing) | §1.1 (no site named), §8 ("B2-K is unchanged"); RR's (ii) ruling ("the kernel's cost in B2-P"; the 4 h threshold) | **The combination's displacement-magnitude formation is kernel code in FK, lane K (B2-K), not PP.**<br>• **Only FK forms a quantity row's published value.** FK `ProductProofDraft::project` (`final_case.rs:1799–1830`) hull-projects every `Native` recipe in pass 1, including `DisplacementMagnitude` (`:1807–1813`). It forms support magnitudes in a second pass, by `support_hypot` (`:1831–1853`), with counted visits and `scalar_operations`.<br>• **PP cannot supply or replace the value.** PP's only input of bits is `ProductRowSpec.observed`, and it is set for record rows only (`:1566–1577`). `certify_final` refuses a frozen value that differs from `observed` (`:1891`), and `matches_values` ties the proof to FK's own values (anchor).<br>• **Consequence.** S-4 (a), r1's nested hypot or (ii)'s exact 3-norm, needs a branch for combination owners in `project`. That branch is like `support_hypot` but takes the node's three `Displacement` translations in mm. Case displacement magnitudes must keep DEF-O's hull recipe.<br>• **KD does not cover it.** KD §5.7 lists the projection among the owner-generic parts ("Nothing else"). §8 puts S-4's recipe in B2-P ("the recipe and the combination observables stage (0.5–1)"). Only the observables stage is PP's.<br>• **(ii) adds** an exact sum of squares and one correctly rounded square root. FK has the wide machinery for it (`wide.rs`, `wide/multi.rs`, `final_case.rs:598` `sqrt`), and the cost is counted work in the receipt. | **Revision 02 states the site:** FK `final_case.rs` `project`, combination owner only. It moves the recipe's cost from B2-P's line to B2-K's (KD §9.2); B2-P keeps the observables stage.<br>**ROOT reads (ii)'s 4 h threshold against B2-K.**<br>**B2-K's brief gains:**<br>• the branch, in KD §5.7;<br>• K-09's diagnose check that a combination's published displacement-magnitude bits equal RN64 of the exact 3-norm of its published component bits (integer square root, ties to even);<br>• K-13's byte identity for case displacement magnitudes;<br>• an S11 site-table row only if a counted fold is added.<br>The file is already in S-13's list, so no stop fires. **RV115 (RV-K) reviews it as kernel code.** |
| A-2 | NOTE | §2 S-3, "The entry's other members" | **The parenthesis "(`normalized_bits` and `scale_bits` null)" is half wrong.** `normalized_bits` is never null: RS's `RowClassification.normalized_bits` is a `u64` that G5c computes from the row's value (`retained_precision.rs:36`, `:3348`). Only `scale_bits` is null (`None`) for a `non_quantity` row. The rule itself is right: "follow a selected case's row of the same class". | In revision 02, read "`scale_bits` null and `normalized_bits` from the row's value, as for a selected case's record row" |
| A-3 | NOTE | §5.1 | **Two must-pass bases are predicted, not observed:**<br>• **W-CB1z's full-gate PASS.** I98's A + B proxy stopped at D6b in G5, a property of the proxy's one-case shape (§4.5). G5a–G5c's S = 0 path and G6–G8 on an all-zero combination were not observed. RS's `absolute_bound(0, 0)` returns the scale-only bound, so I see no reader obstacle.<br>• **`b2_c1_range_mechanics`.** No probe ran it. The prediction (2·case's ledger is the case's scaled exactly by 2) is sound. | None now. B2-P's pins and SC2's corpus establish both, and a difference goes by decision 20 |
| A-4 | NOTE | §5.3 m69, m50 | **Two row choices for SC2, which make the designed first failures deterministic:**<br>• **m69:** "not the combination's largest magnitude" keeps G5b's scales. The row must also sit away from G5c's class threshold \|n\| = 2⁻³⁴·S, so that G5c's `absolute_verified` list is unchanged. A relative-class row of ordinary size is the simple choice. Then the edit reaches G7 under r1 and under (ii) alike: it exceeds the 64ε allowance by at least 63.7× for every normal magnitude in my check.<br>• **m50:** the refused record's new `source_ref` should name an existing CaseSource, such as a case's. Then G3 (strict index) passes, and the edit reaches G5 under §4.3's restriction. | State both in 07o's row text, or leave them to SC2 |
| A-5 | NOTE | (ii), for revision 02's recipe text and RV115's confirmation | **What (ii) must state:**<br>• **RN64 means ties to even.** An exact midpoint is reachable. For example, components (2²⁷+1)·2⁻⁶⁰ and (2⁵³+2²⁷)·2⁻⁶⁰ mm with z = 0 have the exact norm (2⁵³+2²⁷+1)·2⁻⁶⁰, a binary64 midpoint. So the implementation must decide exactness, which FK's wide square root can report.<br>• **Carry over r1's other clauses:** finite only (a norm beyond binary64's range refuses the row: `facade_certificate`), canonical +0, and SI by the projection's mm rule.<br>• **W-CB1z pins +0:** every all-zero triple, signed zeros included, gives +0. | Revision 02's DEF-C text |
| A-6 | NOTE | §3.2 | **The shape count:** I count 19 in-domain shapes, with subtraction in both orders; I97 counts 18. Revision 01's rule holds for every one of mine, and v0's fails exactly I97's two. | None |

## 1. The amendments and NOTEs, against RR's rulings

| Item | Ruling | Revision | Confirmed by |
|---|---|---|---|
| **S-1** | Four B1 sites | §3.1. **T-2′:** three hooks lifted to "within D1.4" by one shared predicate, the same as T-4's. **T-9′:** the observables shape and consistency check. C§2.1's "T-2 Unchanged" is corrected | **Sites at `b1` `03f55e7178`** (unchanged from `603e238517`): `normalized` `:474–508` (refusal `:503–506`), `prepared_case_seen` `:3469–3475`, `prepared_case_source` `:3504–3517`, `observables_view` `:2254–2272`. **Identical at z = 0.** No adapter event is added, because the ordinary route forms combination rows algebraically after the case loop. **Applied** |
| **S-2** | T-6′ follows the producer | §3.2: case blocks end at the first `combination`-basis row; only mechanics combinations must be contiguous; records bind by basis id; each case's scope is its own block | **My own append-order model** (`rv118_r1_checks.out.json` `s2_layout`): r1's rule holds for all 19 shapes, and v0's fails `c=1 range(A), mech` and `c=1 range(A), range(A)`, as §3.2 says. **Producer sites** as in REVIEW (`lib.rs:2802`, `:10019`). **Applied** |
| **S-3** | Records stay `non_quantity` | §2 | **NONQUANTITY includes the record** in all three readers: RS `:2493`, PY `:1149`, TS `:107`. Pinned by W-CB4a, W-CB4b and `b2_c1_range_mechanics`. **Applied**, with A-2's wording |
| **S-4 (a)** | Placement and consequences only; the numerics are RV115's | §1.1; DEF-C r1 `rows.displacement_magnitude`. (ii) has replaced this recipe | **Placement.** The recipe sits in the right static member, the observables stage checks it, and §10.1's G7 row relies on it. **Not stated:** the formation site and its lane (**A-1**). **Static.** `support_magnitude` equals DEF-O's byte for byte. **Applied**, with A-1 |
| **S-4 (b)** | The observables stage is defined | §1.2 (DEF-C r1 `stages.observables`) | **The stage runs** after the certificate and before G5a, in this order: adapter, gate entry `withheld: false`, supports (six components, two magnitudes, 64ε, uniqueness), displacements (one magnitude plus x/y/z per node, 64ε), nothing case-only.<br>**On failure:** stage `failed`, `g5a` `not_entered`, `{kind: unavailable, error: {kind: observable, cause}}`. C§4 maps that to (`facade_certificate`, `facade`), Run selected, per combination. This is decision 7.<br>**Identical at z = 0.** C§2.4 (iii) 4 and §10.1's G7 row are replaced as ruled. **Applied** |
| **S-5** | Split W-CB4; fix m28 | §5.1, §5.3 | W-CB4a and W-CB4b each have c = 2, z = 1 (C_eq = 3). m28 is on W-CB5, in domain, with an added record. m18 and m19 are on the c = 1, z = 2 base. **Applied** (§3 below) |
| **S-6** | Define 07o's rehash | §5.2: seven steps; m10–m12 `after_rehash`; the three harnesses' additions, with PY's guarded reads | Step 2's payload equals C3a-5's. I97's `s6_rehash` check reproduces: the stated order is a fixed point, 07e's rule and a wrong order leave a relation false. Reachability is in §3 below. **Applied** |
| N-1 | Any rebuild refusal leads to `CombinationCustody` | §3.3 | **Applied** |
| N-2 / C-4 | An amendment of C3a rule 4 | §3.3: rule 4's sentence rewritten, superseding KD §2.1 | **Applied** |
| N-3 | Name the branch points | §4.1: 12 RS rows, covering the attempt bijection, Runs/`execution_order`, the owner lookup and `g5_native` | **Spot-checked at the readers' heads:** RS `g5_native` `:904`, `g5_products` `:2061`, `g8` `:3488`; PY `:1821` (owner kind `case`), `:1846` (`execution_order`); TS `:268`. **Applied** |
| N-4 | Correct m50, m36 and `b2_base_withheld` | §4.3 | m50 stays at G5 (§9.2 choice 5). m36 adds its diagnostic. `b2_base_withheld` fails at G8, with RS `:3518–3520`, PY `:1503` and TS `:1143` verified. **Applied** |
| N-5 | G5 refuses an origin capture error | §3.3 (m66, m67) | SCHEMA admits the shapes (I97's `n12_n5_schema`, reproduced). **Applied** |
| N-6 | `formation_warrant` | §4.6 | **Applied** |
| N-7 | PTABLE states R-COMB-1 | §2 | **Verified:** PTABLE r1's `accuracy_classification.scope` text. v0 to r1 changes only `scope` and `product_formation_definitions`. **Applied** |
| N-8 | J1's RS edit is a constant list | §4.6, extended to TS (§9.2 choice 6) | **Applied** |
| N-9 | The freeze's inputs | §3.3: operand 0's slot (maps, Φ) | **Applied** |
| N-10 | B2-A's census | §4.6 | **Verified at `b1`:** `retained_memory.rs:291–297` (combination owners not read); `domain_clauses` splits at `CAP_ROWS − 1` (`:893–899`); `ControlBytes` is the last row (`:869`). **Applied** |
| N-11 | G3 (h), (i) | §4.2 | **(h) is vacuous in B2's domain:** a record needs a selected operand, and c ≤ 2 leaves at most one `not_required` case. (i) is pinned by m68. **Applied** |
| N-12 | The schema gate refuses DEF-C's id | §4.4 | RS G1 checks shape first (`need(shape(r, schema()), "G1", "RECEIPT_MISMATCH")`, `:569`). I97's check is reproduced: jsonschema and PY's walker both refuse it, and G0 row 9 passes. **Applied** |
| N-13 | Declined | Not taken | **Correct** |
| N-14 | Reserve `count_range` in `combination` | §4.6, §7 | **SCHEMA has the same tag and member** in `Stop`, `Unresolved`, `SourceError` and kind enums. **Applied** |
| N-15 | Restate the estimates | §8 | **Applied, except that B2-K moves (A-1)** |
| I98 | W-CB1 rebased; W-CB1z; W-CB2; W-CB3 v1; D6b | §4.5, §5.1 | §4 below. **Applied** |
| NB-1, NB-3 | DEF-C wording | §6 | **The texts are as RV115 asked;** RV115's NC-5 also confirms them. Exactly four DEF-C leaves change from v0 (my own leaf diff). **Applied** |

**The statics reproduce.**
- `b2c_statics_r1.py`, run twice, is byte-identical and equal to the record. DEF-C r1 raw is `6467c733…`, and PTABLE r1 is `791c0a0d…`.
- **My own JCS:**
  - DEF-C r1's H is `0c43cf42…`; the alternative domain gives `82fccc74…`; DEF-O's `a7ed7ca0…` is the control;
  - DEF-C r1's raw bytes are their own JCS form, ASCII, with no trailing newline;
  - `operand_definition.sha256` equals H(DEF-O).
- **PTABLE r1 against main:** three members change and `receipt_bindings` is added. `receipt_bindings` equals XTABLE's, and the formation hashes recompute.
- **The other reruns:**
  - `b2c_checks_r1.py`, run twice, equals the record;
  - `b2c_collisions_r1.sh`'s body equals the record's.

## 2. I97's §9.2 choices

**Accepted:**
1. **T-9′'s gate check: shape and consistency.**
   - The stricter "all `withheld: false`" would couple every case freeze to a combination's gate.
   - A withheld entry is T-10a's `base_withheld`, and under D1.5 and D1.6 neither option is reachable.
   - It is one of the two forms S-1 offered.
2. **W-CB1z in 07o, sparse only.**
   - The readers' all-zero edge (S = 0 in G5b and G5c) needs one mode.
   - B2-P pins A + B in both modes as a producer pin.
3. **The new base `b2_c1_range_mechanics`.** One in-domain base (c = 1, z = 2, C_eq = 3) serves m18, m19 and S-2's layout pin. It needs no synthetic label. Its predictions are sound (A-3).
4. **m28 on W-CB5, with an added OperandPreparation.**
   - It keeps a producer-solved, in-domain base.
   - The added record's hash is step 2's, and its CaseSource sits where G3 (i) wants it.
   - So the first failure is C3a mutation 2's G3 conjunct.
5. **m50 at G5, by restricting G3 (d) and (g) to prepared records.**
   - This follows D22's precedent.
   - It keeps C3a mutation 3's intended gate.
   - G5 already ties `completed` ⇔ `prepared` ⇔ a non-null `source_ref` (see A-4 for the edit's target).
6. **N-8's trim extended to TS.**
   - TS's `header` gains DEF-C's `{id, H}` only.
   - Packaging DEF-C and checking its file H go with B2's TS work, as with RS.
   - No J1 corpus carries a combination attempt.
7. **The operand-preparation mutations and hook base move to W-CB3.**
   - W-CB3 v1 is the probe-qualified operand preparation; W-CB2's path was not observed (I98 §4).
   - What stays on W-CB2 (m32, m52, m53, m56, m67) does not touch its operand preparation.
8. **`statics/r1/` holds only the two changed statics.**
   - The J1 SCHEMA (`abf3225c…`) contains no DEF-O, DEF-C or PTABLE hash. I checked for their v0 and r1 hashes: 0 hits.
   - The DEF-C id is unchanged.

## 3. §10: 19 bases and 69 mutations

**Bases** (§5.1); the producer-solved and hook-produced ones are in D1.4:
- **Producer-solved, C_eq = 3 each:**
  - W-CB1 ×2, W-CB1z ×1, W-CB2 ×2, W-CB3 ×2, W-CB4a ×2, W-CB4b ×2, W-CB5 ×2 (c = 2, z = 1);
  - `b2_c1_range_mechanics` ×2 (c = 1, z = 2).
- **Synthetic,** labelled: `b2_base_withheld`, `b2_pre_source_refusal`.
- **Hook-produced,** labelled: `b2_operand_preparation_failure` on W-CB3, `b2_operand_source_unavailable` on W-CB1.
- **Totals:** 15 + 2 + 2 = 19.
- **The must-pass standings** follow decision 20's rule. `needs_recompute` applies only where case B is `unavailable`.

**Mutations:**
- **The count.** v0's m1–m64 (with m3–m7 as one row of five) plus m65–m69 gives **69**.
- **Every W-CB4 row is reassigned:**
  - to W-CB4a: m14, m20, m33, m36, m58, m60;
  - to W-CB4b: m59, m61;
  - to `b2_c1_range_mechanics`: m18, m19.
- **Each edit fits its base:**
  - the range edits (m59, m61) are on 4b;
  - the subtraction and ordinary edits are on 4a;
  - the two-entry edits are on the c = 1, z = 2 base.
- **Every base a row names exists.** The two older CORPUS bases (`ordinary_prepared_synthetic`, `two_case_synthetic`) are unchanged.

**Reachability under S-6's rule:**

| Rows | What rehash must recompute | Steps | Designed first failure |
|---|---|---|---|
| m27, m28, m29, m30, m64, m65 | The operand preparation's hash (C3a-5 payload), then the identities that depend on it | 2, then 3–5 | G3 (m27–m30); G8 `PREPARATION_MISMATCH` (m64); G1 schema `const` (m65, which RS checks before any hash) |
| m39, m40 | The CombinationSource's identity (representative or `case_index` edit) | 5 | G5 `ATTEMPT_MISMATCH` |
| m62, m63, m68 | The resealed CaseSource identities and the operand identities | 3, 4, 5 | G8 `PREPARATION_MISMATCH` (m62, m63); G3 (i) (m68) |
| m10, m11, m12 | Steps 5, 4 and 2 would undo the edits, so they are `after_rehash` (07b's D24) | — | G1 `RECEIPT_MISMATCH`, with three reader-local unit tests per reader for the inner conjuncts |

- **No other row edits** a hashed payload that steps 2, 4 or 5 recompute.
- **m51** changes `stage`, which is outside C3a-5's payload.
- **m2's** edit of `definition_id` is refused at G0 first.

**The restated gates follow:**
- m18 and m19 at G3;
- m36 at G5, once its diagnostic is added;
- m50 at G5, under §4.3's restriction;
- m66 and m67 at G5 (N-5);
- m69 at G7, with A-4.

**"Today"** is a code reading, and SC2 fixes the actual first failures.

## 4. The witnesses and D6b

**The input hashes equal I98's** (PROBE §5, §8):
- W-CB1, `r7_cb1_halfb` `7af8c049…`;
- W-CB1z, `r7_cb1` `0c346f49…`;
- W-CB2, `r7_cb2` `ce52d528…`;
- W-CB3, `r7_cb3_v1` `76bb9831…`.

The other bases derive from W-CB3's or the milestone's cases.

**D6b is stated case-only** (§4.5), with no combination analogue.
- **I98's D6b refusal** came from the A + B proxy, whose one case was `selected` and ordinarily `checks_passed` (PROBE §2.3, RS and PY alike).
- **A real combination** has no ordinary attempt and no `numerical_quality.cases` entry. T-4 never puts it in A.

**The witnesses under (ii).**
- **I98's observation that 1·A + 0.5·B publishes** was made with B1's hull-projected magnitudes, which is v0's recipe. r1's nested hypot would have weakened that evidence (SA3-1). (ii) restores v0's availability in RV115's model, so I98's observation is again the best predictor.
- **W-CB1z's** displacement magnitudes are +0 under (ii) (A-5).

## 5. J1's reviewed inputs and the names

**Revision 01's reviewed inputs** are SCHEMA `abf3225c…`, PTABLE `791c0a0d…` and DEF-C `6467c733…`:
- **SCHEMA** is v0's J1 text: `SCHEMA_J1.diff` `f674370d…` and `SCHEMA_B2.diff` `ff3b8895…` are unchanged;
- **the PTABLE cascade** (RV114 §6's 12 files) moves `c74742ce…` → `791c0a0d…`.

**Under (ii), revision 02 regenerates DEF-C, its H and PTABLE.** So J1's PTABLE and DEF-C entries, and the cascade's target, become revision 02's. SCHEMA stays `abf3225c…`, because no hash or id in it changes.

**The names (§7)** are reserved as stated:
- the `count_range` pair in space `combination` with member `name`;
- the eight 07o base ids;
- the two suggested internal names.

Each has 0 hits at NUM `d1d6517455` (I97's `b2c_collisions_r1.sh`, rerun; body equal to the record). P's tree is unchanged at `827dc41c2e`. (ii) adds no wire name.

## 6. S-4 (a): placement, consequences and option (ii)

**The placement in the contract is right:**
- The recipe is DEF-C's `rows.displacement_magnitude`.
- The observables stage (§1.2) checks G7's guard on the frozen rows before G5a, so any failure costs only that combination.
- §10.1's G7 row then says a precommit G7 refusal can come only from a producer defect.

**The missing piece is the formation site** (A-1): FK's `project`, for combination owners. It is not PP. B2-P keeps the observables stage, which reuses B1's case support guard.

**What (ii) changes and leaves:**
- **It changes** DEF-C's `rows.displacement_magnitude`, `stages.observables` (NC-2's formula), DEF-C's H, PTABLE's hash and the FK branch's arithmetic.
- **It leaves unchanged:**
  - **The observables stage's order and failure mapping.** Its guard is still the readers' nested hypot at 64ε·max(\|p\|, MIN_POSITIVE).
  - **§10.1's G7 row.** It refers to "DEF-C's magnitude recipes" and holds under (ii). In my check (`rv118_a01_option_ii.out.json`), 20,000 triples and 88,350 faithful-reader pairs give 0 failures, at most 3 ulps and 3/64 of the allowance. RV115 bounds it at 2.5 ulps, a 25× margin.
  - **m69 and its designed gate.**
  - **The witnesses' expected dispositions.**
  - **SCHEMA and the readers.**
- **One more fact for revision 02:** §1.1's "one formation rule on both routes" no longer holds for displacement magnitudes. The ordinary route keeps PP's nested hypot (`lib.rs:13328`), which passes G7 as today. Revision 02 rewrites §1.1 anyway.

## 7. Execution record and limits

**Executed** with VENV (Python 3.13.14), `PYTHONDONTWRITEBYTECODE=1`, `-B`, `TMPDIR` in my scratch, and Git reads with `GIT_OPTIONAL_LOCKS=0`. `addendum_01/RUN_ADDENDUM.md` has the commands with placeholders, and `addendum_01/checks_addendum.txt` has the hashes and code sites.

**Writes:**
- my scratch;
- this file, `addendum_01/` and `SHA256SUMS.addendum_01`.

**Host slip:** I made one host `python3 -c` call (not VENV) that read a scratch JSON output and printed three values. It wrote nothing.

**Limits:**
- **Nothing was compiled or run in Rust or TS.** A-1 and §1's code facts are readings.
- **My S-2 check models the producer's append order** from code. No committed envelope has a subtraction or range record.
- **The option (ii) script** models faithful reader libraries as the correctly rounded value ±1 ulp per call. It does not re-derive RV115's availability study. (ii)'s numerics are RV115's to confirm in revision 02.
- **The reachability in §3 is by reading.** SC2 fixes the actual first failures.
