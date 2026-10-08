# RV118 (RV-C) ADDENDUM_02: confirmation of I97's B2-C revision 02

TASK (Type 2), RV118, holding RV-C for B2, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

This addendum answers the coordinator's request: "please confirm I97's B2-C revision 02 against your ADDENDUM_01 and the RR rulings". The sealed `REVIEW.md`, `ADDENDUM_01.md` and their sums are untouched: `SHA256SUMS` is 7 of 7 OK, and `SHA256SUMS.addendum_01` 7 of 7 OK.

**The subject:**
- `R/I97/b2_c_01/REVISION_02.md`, sha256 `79007dcd768b2c92183236ee4aec4dbaa921ef26d441388b8bcf2365f686d85f` (verified). `SHA256SUMS.revision_02` is 9 of 9 OK; the sealed `SHA256SUMS` is 13 of 13 and `.revision_01` 10 of 10.
- The revision's brief, `R/BRIEFS/B2C_REVISION_02.md`, is `d82dba65…0c56` (verified).

**What I read:**
- the revision and its brief, and my ADDENDUM_01;
- RR "RV118 confirms B2-C revision 01; the exact rounding is formed in the kernel's projection (B2-K); …" and "I97's B2-C revision 02 verified; RV115 and RV118 confirm it";
- the code sites in `addendum_02/checks_addendum_02.txt`.

**Scope.** RV115 confirms (ii)'s numerics in parallel, so I checked only their contract placement. My two NOTEs on the kernel sketch (B-2) are routing observations for RV115 and B2-K's brief, not numerical findings.

**Method.**
- **Reproduction.** I ran I97's two r2 scripts unchanged, twice each, from copies in my scratch. Each copy is `cmp`-equal to the record, and the outputs went to scratch.
- **One script of my own,** `rv118_a02_checks.py`, standard library only, run twice with byte-identical output. It loads my sealed ADDENDUM_01 scripts by path, unchanged.
- **Code reading** at NUM.
- **Limits on execution:** VENV's Python only; no host `python3`, cargo, install or Git write.
- **NUM moved** from `c8e54918cd` to `7955efb86f` while I worked (erratum E-16, records only). P's `core`, `fixtures`, `schemas`, `apps` and `tests` trees are equal at both heads.

## Verdict

**CONFIRMED. 0 BLOCKING, 1 SHOULD-FIX (routing only), 2 NOTE.** None of the three changes revision 02's statics or J1's inputs.

**Item 1, A-1 to A-6.** Each is applied as I asked.

**Item 2, (ii)'s placement.** (ii) is placed consistently:
- DEF-C r2 changes exactly two paths from r1, and PTABLE r2 exactly one;
- the observables stage carries NC-2's formula verbatim;
- §10.1's G7 row, m69 and the witnesses all stand.

**Item 3, J1.** The reviewed inputs are SCHEMA `abf3225c…` (unchanged), PTABLE r2 `b2b4a54d…` and DEF-C r2 `3cebce55…`. My own JCS gives DEF-C r2's H as `d3fde142…`, the value J1's G0 lists carry. The names stay reserved.

**Item 4, the estimates.** B2-K is 21.5–33.5 h and B2-P is unchanged. Both are consistent.

**On the SHOULD-FIX, B-1.** It corrects one assertion of SC2's new TS test. ROOT's ruling can carry it into SC2's brief, with no revision 03. **With RV115's confirmation, B2-C is then final for J1.**

## Findings

| ID | Severity | Where | Finding and evidence | Required change |
|---|---|---|---|---|
| **B-1** | SHOULD-FIX (routing) | §4.5 (NC-3's TS test) | **The test's second assertion fails on three of the vectors it names.**<br>• **What it specifies:** `consistentNorm` "refuses p·(1 + 2⁻⁴⁰)" on, among others, "the subnormal cases".<br>• **Why it fails.** For a subnormal or zero p, p·(1 + 2⁻⁴⁰) rounds back to p, which the guard accepts. This happens for three named vectors: "smallest subnormal", "two smallest subnormals" and "three smallest subnormals" (`rv118_a02_checks.out.json` `nc3_ts_test_vectors`).<br>• **The rest pass.** For every named vector with a normal p, the edit is refused, and every p is accepted. | **For SC2's brief:**<br>• for p ≥ MIN_POSITIVE, the test refuses p·(1 + 2⁻⁴⁰);<br>• for p < MIN_POSITIVE, +0 included, it refuses p + 256·2⁻¹⁰⁷⁴. That is beyond the guard's absolute allowance of 64·2⁻¹⁰⁷⁴ plus the reader's few-ulp difference, and refused in my check for all three. |
| B-2 | NOTE | §2.2 steps 3 and 4 (the kernel sketch); for RV115 and B2-K's brief | **Two boundary cases where the sketch differs from DEF-C r2's exact definition.** The definition and the reference `rn64_norm3` are right; only the suggested FK steps differ.<br>• **(a) At y₀ = 2⁻¹⁰²²,** the lower neighbour is the largest subnormal, one full quantum below: the subnormal quantum equals the first normal binade's. So the lower midpoint is 2⁻¹⁰²² − 2⁻¹⁰⁷⁵, not "one half as far" (2⁻¹⁰²² − 2⁻¹⁰⁷⁶).<br>&nbsp;&nbsp;– **Witness:** x = `000fffffffffffff`, y = 2⁻¹⁰⁴⁸ (`0000000004000000`), z = 0. RN64 is MIN_POSITIVE, but √S lies below the half-as-far midpoint, so the sketch would step down to the largest subnormal.<br>&nbsp;&nbsp;– `exact_norm_vectors.json` lacks this vector (`step4_edge_at_min_positive`).<br>• **(b) Step 3 refuses on the 1,024-bit estimate's `Overflow`.** The exact boundary is S against (2¹⁰²⁴ − 2⁹⁷⁰)², with equality refused by ties to even, which is `to_binary64`'s own threshold. I found no reachable case where the estimate's verdict differs, but one more `signum` decides it exactly. | **B2-K's brief:**<br>• correct (a)'s sentence;<br>• add the witness to the unit tests and K-09's vectors;<br>• optionally decide (b) exactly.<br>**RV115** may fold both into its confirmation of the sketch. |
| B-3 | NOTE | §6 | **The estimates are consistent and slightly conservative.**<br>• B2-K: RR's 19–30 h plus 2.5–3.5 h gives 21.5–33.5 h.<br>• B2-P: r1's line "S-4: the recipe and the combination observables stage (0.5–1)" was partly the recipe, which was never PP's. Keeping B2-P unchanged overstates it by at most about 0.5 h.<br>• SC2: its low end, 9.5, rounds up 9.25. | None |

## 1. A-1 to A-6

| Item | ADDENDUM_01 asked | Revision 02 | Confirmed by |
|---|---|---|---|
| **A-1** | **The site:** FK `project`, combination owners only. **The cost** moves to B2-K, and B2-P keeps the observables stage. **B2-K's brief gains:** the KD §5.7 branch, K-09's check, K-13 and RV-K's review | §2.1–§2.4. **Pass 1** also skips `DisplacementMagnitude` for a combination owner. **Pass 2's helper** (suggested `displacement_norm`) sits beside `support_hypot`. **The brief** gains five items: those four plus unit tests. The S11 row is added only with a counted fold, and the file is in S-13's list | **Code at NUM:**<br>• `project` `final_case.rs:1799–1829`; `support_hypot` `:1831–1853`; `certify_final`'s frozen-bits check `:1889–1891`;<br>• `ProductRowSpec.observed` is set only for records (`:1566–1577`);<br>• `NativeOwner::Combination` (`origins.rs:161–164`).<br>**Every primitive it names exists:**<br>• `ExactWideSum::add_product`, `signum` and `round` (`wide_sum.rs:591`, `:680`, `:761`; spans of 6,244 within 8,128 bits);<br>• `WideContext::sqrt` (`wide/multi.rs:1214`) and `to_binary64` with its `Overflow` and `Subnormal` outcomes (`:677–761`);<br>• `sqrt_owned`'s "one nearest sqrt and the exact q·q − a side" (`directed/certificate.rs`);<br>• the `w.wide.merge` / `w.sums.merge` pattern (`final_case.rs:598–609`).<br>**Constrained translations** are still `Native(Displacement)` specs, of class InputDerived with a point enclosure, so step 1 finds three per node at an anchor too. `displacement_norm` has 0 hits at NUM. **Applied**, with B-2 on the sketch |
| **A-2** | S-3's parenthesis | §4.1: "`scale_bits` null, and `normalized_bits` from the row's value" | **Applied** |
| **A-3** | Mark the two bases predicted, and say where each is first observed and what happens if it fails | §4.2: B2-P's producer pins in both modes, then SC2's must-pass. Each fallback outcome is named:<br>• **W-CB1z,** producer side: the pin records the outcome, the label stays, the must-pass is restated under decision 20, and ROOT is told (NB-3's pin moves). Reader side: a lane defect, declared;<br>• **`b2_c1_range_mechanics`:** an unselected 2·case is restated under decision 20, and the base still pins S-2 and serves m18 and m19. A custody refusal is a B2-P defect | **Applied.** The outcomes are complete and fail-safe |
| **A-4** | m69 away from G5c's threshold; m50 names an existing CaseSource | §4.3:<br>• **m69:** a row with 2⁻²⁰·S ≤ \|n\| < max, 2¹⁴ above G5c's 2⁻³⁴·S, and not the largest;<br>• **m50:** `source_ref` → 0, case A's CaseSource (the hook base has no other) | **Applied** |
| **A-5** | Ties to even, with r1's clauses | §1.1, in DEF-C r2's text: "RN64, ties to even, …; finite only …, canonical +0, SI by the projection's mm rule" | DEF-C r2's text, verbatim. All 411 vectors agree with my `rn64_sqrt` (bits and tie flags): 2 ties, 1 refusal. The midpoint and midpoint + 2⁻¹⁰⁷⁴ cases are as stated. **Applied** |
| **A-6** | The shape count | §4.4: 19 listed, subtraction in both orders | **Identical, shape for shape,** to my own enumeration (ADDENDUM_01). v0 fails the same two. **Applied** |

## 2. (ii)'s placement

- **DEF-C r2 against r1:** exactly `rows.displacement_magnitude` and `stages.observables` change.
  - Each text appears verbatim in REVISION_02, in §1.1 and §3.
  - `support_magnitude` equals DEF-O's.
  - `operand_definition.sha256` is H(DEF-O).
  - The raw bytes are their own JCS form, ASCII, with no trailing newline.
  - `stages.entered` is r1's.
- **PTABLE r2 against r1:** only `product_formation_definitions` changes.
  - Both formation hashes recompute: DEF-O's `a7ed7ca0…` and DEF-C r2's `d3fde142…`.
  - `receipt_bindings` equals XTABLE's.
  - N-7's `scope` is r1's.
  - The file is indent-2 ASCII with one trailing newline.
- **The observables stage:** NC-2's formula 64ε·max(|p|, MIN_POSITIVE) is in the text. Its order and failure mapping are REVISION_01 §1.2's, unchanged.
- **§10.1's G7 row** stands. Its "DEF-C's magnitude recipes (§1.1)" now reads through revision 02's §1, and G7 holds by construction. I97 measures 0 failures over 144,598 pairs, at most 3 ulps; the numerics are RV115's.
- **m69** stands: the edit exceeds the allowance by ≥ 63.7× under (ii) (ADDENDUM_01). A-4 fixes its row.
- **The witnesses** are unchanged. W-CB1z's magnitudes are +0 under (ii): all eight signed-zero triples give +0.
- **The site and lane** are as A-1 asked. PP, `support_hypot` and every case path are unchanged.

**I97's r2 scripts reproduce:**
- `b2c_statics_r2.py` twice: `diff -r` identical, and DEF-C r2, PTABLE r2 and its report equal the record;
- `b2c_checks_r2.py` twice: identical, and its output and `exact_norm_vectors.json` equal the record.

## 3. J1

- **SCHEMA `abf3225c…`** is unchanged. It contains no DEF-C r2 H and no raw hash of DEF-C r2 or PTABLE r2, and (ii) changes no id or member.
- **The reviewed inputs:** PTABLE r2 `b2b4a54d…` and DEF-C r2 `3cebce55…`.
- **The PTABLE cascade** goes `c74742ce…` → `b2b4a54d…`.
- **J1's RS and TS G0 constant lists** carry `{DEF-C id, d3fde142…}`.
- **R-10's alternative-domain H** for DEF-C r2 is `941972e0…`, for the record. C-15 stands.
- **The names:** reservations are unchanged and no wire name is added. The suggested internal `displacement_norm` has 0 hits.

## 4. Execution record and limits

**Executed** with VENV's Python 3.13.14 only (`-B`, `PYTHONDONTWRITEBYTECODE=1`, `TMPDIR` in my scratch), with Git reads under `GIT_OPTIONAL_LOCKS=0`. `addendum_02/RUN_ADDENDUM_02.md` has the commands with placeholders. No host `python3` was used.

**Writes:**
- my scratch;
- this file, `addendum_02/` and `SHA256SUMS.addendum_02`.

**Limits:**
- **Nothing was compiled or run in Rust or TS.** A-1's sites and B-2 are code readings plus exact Python models.
- **B-2 (b)'s "no reachable case"** is not a proof.
- **B-1's check uses this host's `math.hypot`** for r. TS's `Math.hypot` may differ by a few ulps, well inside both edits' margins.
