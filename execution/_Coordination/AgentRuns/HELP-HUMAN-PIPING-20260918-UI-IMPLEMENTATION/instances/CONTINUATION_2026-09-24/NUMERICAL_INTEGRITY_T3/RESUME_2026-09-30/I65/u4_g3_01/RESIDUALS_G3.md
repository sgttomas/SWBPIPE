# G3 residual repairs: T07 (S-1), T22 (S-2) and the new term T25 (B-1)

These sections replace G2 text, as G2_AMENDMENTS.md maps. Citations are at NUM `5ae5fe4f0f`. No maintained source, schema, fixture or app file changed between `fe38ea55bc` and that revision (Git diff read), so G2's citations at `a2c26cc885` still apply. Abbreviations as in the plan: P, PP, FK (= FKS/retained), FKS. Numbers marked ASSUMED use the illustrative 64-bit layouts in `_run_records/g3lib.py` (the same values G2 and RV83 used). They are not qualified values; G5 evaluates every atom in the actual build.

---

## T07 Generic deep legacy-exact (replaces RESIDUALS.md:97–144; RV83 S-1)

**Result.** At the caps, every owner of the legacy-exact path, all counted live at once:
- **requested: 39,805,125 bytes (ASSUMED)**, about 38.0 MiB;
- **moving: 41,602,538 bytes (ASSUMED)**, requested plus the largest single old backing (the identity JSON's last reallocation). Only one reallocation is in progress at a time, so moving adds one old backing, not one per owner.

The symbolic form at the caps (integer coefficients on layout atoms plus a constant) is in `_run_records/t07_repair.caps.out.json`. G2's published 36,474,868 B is withdrawn.

**The path is unchanged from G2.** `source_recovery::solve` (PP/source_recovery.rs:1434–1600) runs prepare_sources, Context, solve, FunctionalPlan, evaluate, the projections, retain and the selected output. It is entered once per case at most (the single call site, PP/lib.rs:3711, through `solve_ordinary`, :128–142). The caller first builds a dense copy of the stiffness, `formed.to_dense()` (PP/lib.rs:3679–3687), only when the attempt can run (N ≤ 256).

**What changed, by RV83 item:**

| Item | Repair | Source |
|---|---|---|
| S-1(a) | Both `Snapshot.identity` copies are rows: Context's copy (`identity.into()`, capacity = length) and the retained copy (`self.context.source.clone()`, capacity = length) | FKS/exact_boundary.rs:466, :1429 |
| S-1(b) | The identity String from `serde_json::to_string(&(names, bits))` has capacity ≤ max(128, 2e) (COEFFICIENTS J3), with e ≤ 5 + 771·names + 21·bits. At the caps names = 1,992, bits = 12,456, e = 1,797,413 and the capacity bound is 3,594,826 | PP/source_recovery.rs:462–501; serde_json ser.rs:2213–2257 |
| S-1(c) | Descriptors #1 are bounded by their construction law, not by `descriptors_charge` (see below). #2 and #3 exist only after `descriptors_charge` passes, so the 16,384-unit law bounds them | PP/source_recovery.rs:964–1240, 1398–1432; FKS functionals.rs:309–322, 427–476, 805 |
| S-1(d) | The Response temporaries are a row: at most 11 Expansion children live during the two-DOF block solve, and 9 during the reaction loop. Rows are also added for the evaluation temporaries (12) and the projection temporaries (6) | FKS/exact_boundary.rs:525–597, 236–243, 769–851; functionals.rs:550–606 |
| S-1(e) | Exactly **three** descriptor generations are live at once: #1 `Sources.descriptors`, #2 `FunctionalPlan.descriptors` (`to_vec`), #3 `RetainedFunctionalSet.descriptors` (`clone`). The G2 script's seven was an error | PP/source_recovery.rs:376, 1453; functionals.rs:319, 805 |

**Owners G2 omitted, now rows:**
- `Sources.assembly`, a dense `AssemblyEvidence`: two retained N×N stores, plus two more during its construction (structural_adapter.rs:62–93);
- `Sources.force_terms`, with their cloned load ids;
- the prepare_sources locals: the folded stiffness (N×N), the folded force ledger, seven hash maps and sets, the boundary vectors, and the identity builder that coexists with its JSON output in `finish()`;
- the caller's dense recovery stiffness;
- the two `descriptors_charge` ordering vectors and the one `Arc<()>` attempt token.

**The descriptors #1 construction law** (per member, D1 straight members). The local stiffness has at most 2, 4, 4, 2, 4, 4 nonzeros in rows 0–5, and the same in rows 6–11: 40 in total. The matrix starts at zero and only `set_symmetric`, `add_bending_z` (indices 1, 5, 7, 11) and `add_bending_y` (indices 2, 4, 8, 10) write it (FK lib.rs:798–809, 1696–1737). Per member:
- **12 end rows**, cloned into `descriptors` with exact capacities. Each holds two key Strings (≤ 2·128). They have 40 terms in total, each of s(AffineTerm) + s(Vec) + 8, plus at most 2 offsets.
- **30 station rows**, built by `scaled_row` and `append_scaled_products`:
  - each term's product Vec is cloned (capacity 1), then `extend_from_slice` grows it to capacity 4, so 32 bytes;
  - the terms Vec is push-built, with capacity 4 for 2–4 terms and 8 for components 4 and 5, which append four shear terms;
  - one offset Vec of capacity 4 per fraction, for component 0.
- **Nodal (6n) and spring (s) rows:** two key Strings and one term each (`vec!`, exact).
- **Support rows (6g):** contribution terms only on DOFs the support rigidly owns. Each DOF has at most one rigid owner (source_recovery.rs:842–846), so Σ terms ≤ C + s. Support ids are unique (:783–789), so each spring matches one row. Offsets total ≤ l. Push capacities are bounded by Σ max(4, 2t) ≤ 4·6g + 2·Σt.
- **The outer Vec** is simulated through its actual sequence of 12-element extends and pushes. Final capacity 3,072, previous capacity 1,536.

At the caps, #1 holds 41,760 descriptor units, so `descriptors_charge` refuses at step 4 while #1 is live. That is a prefix, and it is covered. #2 and #3 are bounded by 16,384 units each, whatever m is.

**Monotonicity.** Every row is a sum, with nonnegative coefficients, of the counts n, m, g, s, l, r, N, F, C and Z, or of PushCap, `buckets`, `max` or `min` of them. Each is nondecreasing in every count taken separately, and each count has its own independent cap, so the value at the cap vector bounds every D1 input. The one non-monotone input is F = N − k. It is replaced by its independent upper F ≤ N.

**Failure paths** remain prefixes of the success path (RESIDUALS.md:140, confirmed). The deep path's largest refusal prefix, at step 4 with #1 live, is covered because #1 is a row.

**Placement.** T07 sits in the ordinary span as `LegacyRecoveryPrefix_or_Exact` (ORDINARY.md). When it selects, finalization follows (T25 below). Both then lie inside G-A's ordinary span, alongside observation.

---

## T22 Closed pre-execution scalar admission (replaces RESIDUALS.md:195–212 and the T22 citations at D4_RECONCILIATION.md:42, :93; RV83 S-2)

**The rule each row satisfies.** Each scalar either:
- is computed by a checked operation whose failure is a typed stop (`CountRange`, `Budget`, `ExponentRange` or `SourceError`), which refuses to the ordinary path, never a wrap; or
- is representable for **every** input, by the stated lemma.

The value at the caps is given for each row. The counts are reproducible with `_run_records/t22_sites.py`.

**Reproducible census** (non-test source, text before the first `#[cfg(test)] mod`):
- FK/retained: 60 `CountRange(` tokens (36 with a literal label), 9 `u32::try_from`, 30 `checked_mul` (5 of them `checked_mul(64)`), 3 `ceil_sqrt`;
- PP retained_product.rs: 38 `CountRange(` (36 labelled), 8 `u32::try_from`, 26 `checked_mul`.

RV83's 62 FK tokens include the two inside FK/source.rs's test module. G2's "57" is withdrawn.

**The I34 DESIGN.md:255–265 classes, site by site:**

| I34 class | Sites | Check or lemma | Value at the caps | Width |
|---|---|---|---|---|
| Node×6 and DOF endpoints; node/member/spring/station/support/raw child counts; u32 ids, encoded lengths and encoding capacities | FK/source.rs:383–425: `u32::try_from` for each of the 8 source lengths, `nodes·6` checked, `Layout::array`, the byte total checked, each `source_id` length `u32`; PP retained_product.rs:179–186 (`checked_mul(6)`, `u32::try_from` for nodes, 3m and supports); PP/source_recovery.rs:507–512 (`checked_mul(DOF_PER_NODE)`) | checked, `CountRange("source representation")` | N = 192; nodes 32; 3m = 96; source ids ≤ 128 B; encoding bytes ≤ 38 + 24n + 84m + 17s + 13k + 17l + 16t + 22g + Σids + 4s ≤ 36,742 (t ≤ 3m = 96, k ≤ 192) | u32, usize |
| Pattern and contribution uppers (144m+s+9d), dense N², profile F(F+1)/2, prefix sums and row/allocation lengths | FK/source.rs:427–444: p = 144m+9d+s checked, z = min(n·n, p) with n·n checked, 2·count and count+1 checked, `Layout::array`; FK/source.rs:356–368 `checked_profile_count`: **free·(free+1)** checked, the u32 block sentinel, `Layout::array(product/2)` | checked, `CountRange("free profile" / "free block sentinel" / "profile layout" / "source representation")` | p = 4,640; n·n = 36,864; **free·(free+1) = 37,056** (G2's "free·n 36,864" was mislabelled); profile 18,528 | usize |
| Layout upper 7n+12m+6t+s+3d+r+2g; operand, prescribed, combination and support membership multiplicities; address and capacity expressions | FK/source.rs:409–425 (q, checked, then `u32::try_from(q)` and `Layout::array`); FK/origins.rs:43–80, 354–450 (`CountRange` "case runs", "operands", "calls", "runs", "physical records", "builds", "record allocation", and the ordinals); FK/product_certificate/final_case.rs:445, 535, 1116–1181, 1623, 1673 (prepared capacity bytes, array layout, derivative and support rows, support capacity bytes and slot, proof anchor layout); PP retained_product.rs:505–527, 772–860, 1168, 1992, 2934–2979, 3084–3097 (observation, support, stations, adapter and prepared layout and capacity) | checked, labelled `CountRange` | q = 7·32 + 12·32 + 6·96 + 32 + 0 + 192 + 2·32 = 1,472; support components 6g = 192 | u32, usize |
| Residual and fallback count → m = 2·count + 2 → 64m | FK/adaptive.rs:1651–1673, 1716–1724, 1825–1839 | `checked_add(1)`, `checked_mul(2)` then `checked_add(2)`, `checked_mul(64)`: `CountRange("residual row" / "residual operations" / "residual multiplier")` | count ≤ F = 192: m ≤ 386, 64m ≤ 24,704 | u64 |
| Factor operations and 64·operations; condition multiplier | FK/factor.rs:587–590 (`u64::try_from(i − first[i])`, ·2, +2); FK/factor.rs:458–462 and FK/adaptive.rs:1548–1552 (`checked_mul(64)`, "pivot multiplier"); FK/factor.rs:813–815 (3n, "condition multiplier") | checked | ops ≤ 2·191 + 2 = 384; 64·ops ≤ 24,576; 3n = 576 | u64 |
| Tracker offered sequence; tracker held − before + after | FK/adaptive.rs:716–720 (`offered.checked_add(1)`, "tracker sequence"); FK/adaptive.rs:872–876 (`held.checked_sub(before).and_then(checked_add(after))`, "tracker capacity") | checked | bounded by the offers of one attempt; the check stops before any wrap | u64, usize |
| Scale, exponent and index products | Wide exponents: `mul_pow2` widens to i128 and then `checked_exponent` (FK/wide.rs:372–377, 486–492; \|exponent\| ≤ 2^62, `EXPONENT_LIMIT` :135). Every later i64 use, such as `exponent − 127 + shift` (:426), stays within i64 because \|exponent\| ≤ 2^62 and shift ≤ precision ≤ 1,024. Precision shifts `i64::from(p)`, p ∈ {128, 256, 512, 1024} | widened then checked (`WideError::ExponentRange`), or representable by the 2^62 range lemma | precision ≤ 1,024 | i64, i128 |
| `ceil_sqrt` and r·r | FK/bound.rs:996–1006 (`ceil_sqrt`); :1044 (`2 * ceil_sqrt(n_c)`, an **unchecked** u64 multiply); :1156 (`ceil_sqrt(n_c) as f64`) | **Representability lemma, valid for every usize input on the 64-bit D1 target** (BUILD.md identity): <br>– `n as u64` is lossless; <br>– `(n as f64).sqrt() as u64` saturates, which is defined behaviour; <br>– both correction loops compare u128 products, and u64² < 2^128 cannot overflow; <br>– the result r ≤ 2^32 for any n < 2^64, so `r += 1` cannot overflow; <br>– `2·r ≤ 2^33` cannot overflow u64; <br>– `r as f64` is exact (r < 2^53). <br>This is not a checked operation with a typed stop; it is a total-function lemma. I29's sufficient F ≤ 2^32 − 2 premise is not needed for these three sites | n_c ≤ F = 192: `ceil_sqrt` ≤ 14, so 2·`ceil_sqrt` ≤ 28 | u64, u128 |

**Remaining, outside this table:** the work counters themselves. They are under checked/sticky custody (D4_RECONCILIATION.md §2) and the E-only projection (D-4, as ruled).

**For the D-4 ruling:** the reconciliation's premise "closed pre-execution scalar admission" (I34 API_PLAN.md:304–306) now points at this table, not at RESIDUALS.md:195–212. The outcome is unchanged: input-derived scalars are small at the caps, and operation-derived scalars are checked.

---

## T25 Selected source-blocks finalization (new term; RV83 B-1)

**The finding is confirmed.** When a D1 case's ordinary solve is `Sensitive` or fails, the case can enter legacy source recovery (T07; PP/lib.rs:3660–3722). If recovery selects, the case continues into SOURCE-BLOCKS-1 finalization. All of this runs on the ordinary route with the W1 observer still installed: the receipt is finalized at PP/lib.rs:2796–2798, and `observer.finish` follows at :2813. So T25 lies inside G-A's ordinary span. It is an alternative to the W1 run, because an exact-block selection bypasses W1 (D-15; RR:189).

A field predicate cannot exclude these inputs, as RV83 says, so T25 is priced rather than excluded. "Remaining: none" at G2 RESIDUALS.md:142 is withdrawn, and so is the DOMAIN.md:3 claim that D1 stays "inside the source paths I54/RV75 priced".

**`composite` is false in D1.** It requires `pressure_runtime::is_exact` (PP/lib.rs:2743), so `finalize` takes the SOURCE-BLOCKS policy, and source_receipt/composite.rs is never entered.

**Owner roster.** Per case, the steps run in order; each step's locals drop at its return. Per invocation, the publication Value and the second typed request live to the end, and the wire/body chain grows. The arithmetic is in `_run_records/t25_caps.py`; the T07 rows it reuses come from `t07_repair.caps.out.json`.

| Step | Source | Owners priced |
|---|---|---|
| S1 `check_input` → `check_input_with_physical` | source_receipt.rs:270–366 | <br>– `requested()`: a raw clone (≤ the T02 raw bound) plus the typed request (T03 at the typed capacity caps), plus serde's `#[serde(flatten)]` and internally tagged `Content` buffering (≤ 16,384 · s((Content,Content)) + 262,144 B);<br>– a materials clone;<br>– the O-N transients;<br>– `build_model`, its helper maps and the boundary (I54 rows);<br>– the dense stiffness Mat(f64,N,N);<br>– loads, application, the force ledger and AssembledForce (I54 rows);<br>– the prescribed and free vectors;<br>– `replay_against`: Sources #2 with the prepare_sources locals (T07 A1–B7), Context #2, Response #2, values #2 and the temporaries (T07 C1–G2), plus two ordering vectors |
| S2 `check_binding_against` | source_recovery.rs:330–346 | Sources #3 with its locals; one ordering vector |
| S3 `rows::bind` | source_receipt/rows.rs:562–700 | <br>– the `by_index`, `used`, `actual` and `ledger` BTree maps;<br>– Fn projections;<br>– the primary and derived rows;<br>– R RowTreatments with their id and input clones;<br>– the supports Value, twice (`json!` copies it) |
| S4 `source::commitment` | source_receipt/source.rs:252–410 | <br>– the payload Value: C contribution objects, m frame objects carrying 3·144 bit strings, springs, prescribed, loads, the lowered descriptors (≤ 41,760 units), the DOF map;<br>– `hash(payload)` by the hash route;<br>– the `functions` and `plan` Values with `hash(plan)`;<br>– the block vectors and sets |
| S5 the case outputs | source_receipt.rs:690–700 | `actual_rows` (I54 RowJSON law), the work Value and the retained `FinalizedSourceBlockCase` |
| I1–I3 `finalize_for` | source_receipt.rs:869–1060 | <br>– `serialized(envelope)`, the publication Value;<br>– `requested()` twice;<br>– the two assessed-quality Values;<br>– the `ids` and `accounted` sets and the observations;<br>– per case, the actual-rows clone, its serialization and the wire entry;<br>– the `body` `json!`, which copies the wire, with `hash(publication)` evaluated inside it;<br>– `hash(body)`;<br>– the `into_wire` deep copy;<br>– the receipt retained in the envelope |

**Composition.** T25 = the carried case outputs + max(per-case stage peak, per-invocation stage peak). G5 takes the max in-build over the evaluated stage expressions. The conservative sum of every step is also recorded, for reference.

**The hash route** (source_receipt.rs:30–37; I54 COEFFICIENTS J1–J10) is applied to every hashed Value. These all coexist at the canonical render:
- the `json!` wrapper copy;
- the `to_string` text, capacity ≤ max(128, 2e);
- the checked-parse tree with its seen-key clones and scratch, plus the Bigint 2,080 B;
- the canonical text, capacity ≤ max(8, 2J).

The escaped length is e ≤ ε·(string bytes) + key bytes + 8·(values) + 24·(numbers), where ε is the input text's worst escape factor.

**The envelope Value at the caps** (`facts.publication`) has:
- 66,528 array slots, 17,466 objects and 98,618 entries;
- **102,171,746 string bytes:** P·Text(row) + the retained diagnostic bytes + the preview strings, both from TEXT.md;
- 1,848,800 key bytes.

**Results at the caps (ASSUMED layout):**

| Escape factor | Publication text e | T25 requested | Moving extra (the publication text's last growth) |
|---|---|---|---|
| ε = 6: any input byte may be a control character, written `\u00XX` | 616,260,332 | **3,074,072,410** | 616,260,332 |
| ε = 2: proposed **D1.11**, no control character in any input string (quotes and backslashes still double) | 207,573,348 | **1,439,324,474** | 207,573,348 |

Both cases peak at the per-invocation stage I1, where `hash(publication)` runs inside the body `json!`.

**This is the largest single term at the caps.** Under ε = 6 it does not fit M together with the ordinary and text terms (COMPOSITION.md). D1.11 (G2_AMENDMENTS.md §6) is the cheap repair: the census can check every raw string and key byte, allocation-free. With D1.11, branch X fits M with about 0.5 GB to spare (ASSUMED). ROOT decides; COMPOSITION.md shows both cases.

**Monotonicity.** Every count enters the stage expressions as a nonnegative-coefficient polynomial, PushCap, `buckets`, `j3cap` (nondecreasing) or `max`. So each stage expression, and the maximum over stages, is nondecreasing in every count, and the value at the caps bounds every D1 input. The text atoms (TEXT.md) are themselves cap-evaluated upper bounds.

**Failure paths.** Every T25 step returns through `?` (source_receipt.rs:668–700, 869–1060). A failure leaves a prefix of the same owners and then pushes one `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` diagnostic (PP/lib.rs:2806–2809), which is in the TEXT.md inventory. The success-path stage maxima bound it.

**Residuals in T25.** The per-object coefficients for the commitment payload, functions and plan come from a reading of source.rs:252–410. They are the largest coefficient set G3 did not prove line by line: the `function()` and `recipes()` bodies (source.rs) are bounded by object, entry and string counts per descriptor and per row, not traced field by field. RV84 should re-derive them. They contribute S4 ≈ 0.78 GB (ε = 6) or 0.51 GB (ε = 2), and do not set the peak.
