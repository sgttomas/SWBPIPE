# RV83 R-4: the call graph's `Self` receivers and bracketed calls (records)

**The routing:** RR "RV83 on the G4 repairs…" (RV83 `R/REVIEW_RV83/u4_g2_03/REVIEW.md`, R-4).

**The result:**
- **TEXT is unchanged.** At l ≤ 128 and ε = 2, the total is 2,044,161,940 B and the run is complete; the admission maximum is unchanged at 0.8510 / 0.8559 M.
- **The recursion inventory is unchanged:** 22 self-recursive functions, no mutual recursion, and the same 40 cyclic components with implicit calls modelled.
- **Seven more functions are reached from the Direct root** (2,698 → 2,705). Two are RV83's. The other five are found by this record's own audit, and none carries text.
- **One further dropped-call class remains as a stated limit:** flow-insensitive typing of rebound names (§3). The true calls it drops carry no text and close no cycle.

**Basis.** The work runs at G4's basis: NUM `b1f80234dc`, from the same `git archive` snapshot. It uses G4's committed scripts unchanged, with two overlays:
- `callgraph_g5.py`, a copy of `callgraph_g4.py`. `CG_R4=0` reproduces G4's edges byte for byte; this was checked.
- `loop_bounds.r4.json`, which is G4's file with one rule added (§4).

**Run with** `_run_records/run_r4.sh`; the outputs are in `_run_records/r4/`.

## 1. The repairs

| Id | Receiver form | G4's resolution | Repair |
|---|---|---|---|
| R4a (RV83) | **A receiver typed `Self`:** a local bound to `Self {..}` or `Self::..(..)`, or a parameter typed `Self`, `&Self` or `&mut Self` | it typed the receiver as "Self", which owns no methods, so the call was dropped and labelled "std/external" | `Self` resolves to the enclosing impl's owner. A trait default body fans out, as R1a does |
| R4b (RV83) | **A call inside index brackets** (`&self.columns[self.row_range(row)]`) | the receiver pattern swallowed `self.columns[self`; the call was typed as a `Vec` method or fanned out from a garbled receiver | a receiver may carry whole index expressions (`x[i].f(`), and an index may contain dots (`by_dof[term.dof].push(`). A call inside the brackets is its own call |
| R4c (this record) | **The unwrap idiom on a parameter:** `if let Some(p) = p` / `p.as_mut()` / `p.as_ref()` / `p.take()`, also `while let`, let-else and `Ok(p)` | it kept the parameter's declared `Option<T>`, so the payload's calls were labelled "std Option" and dropped | `p` is typed `T` |
| labels | the audit's "std/external" | applied to typed user receivers | now reads "receiver typed X: no method of that name on it or a scanned trait", "Self::f: no fn of that name on the impl type (derived or external)" and "Type::f: … (derived, std or external)" |

**What R4c found, in source** (both calls are true calls that G4 dropped):
- adaptive.rs:5024, `recording.group(position, …)` in `solve_cases_projected`, where `recording: Option<BatchRecording>` is unwrapped by `if let Some(recording) = recording.as_mut()`. The target is `BatchRecording::group` (origins.rs:733).
  - In G4 that function was reached only through a *false* edge: R4b's garbled receiver `view.group().ordering.position[g]` in source_residual.rs:490.
  - R4b removes the false edge and R4c adds the true one, so it stays reached.
- retained_receipt.rs:128, `proof.check_summary_coverage(owner, …)`, after `let Some(proof) = proof else …`. This newly reaches final_case.rs:315 `check_summary_coverage`, :283 `coverage_facts`, :263 `rederive_coverage` and :247 `of`.

## 2. The comparison with G4 (`r4/r4_compare.out.json`)

| Quantity | G4 | R-4 |
|---|---|---|
| Graph nodes / edges | 3,320 / 13,112 | 3,320 / 13,118 (+33, −27) |
| Reachable from the Direct root | 2,698 | **2,705**:<br>– structural_adapter.rs:1204 `node` and :1217 `element` (RV83's two);<br>– final_case.rs `check_summary_coverage`, `coverage_facts`, `rederive_coverage`, `of` (R4c);<br>– structural_adapter.rs:282 `solve_binary64`.<br>No function is lost |
| Explicit cyclic components | 22, all self-loops | **the same 22** |
| With implicit calls (`CG_IMPLICIT=1`) | 40 | **the same 40** |
| TEXT at l = 128 (`sens.py`, ε = 2) | 2,044,161,940 B, complete | **2,044,161,940 B, complete**. One multiplicity changes: load_ledger.rs:131 `push` loses false edges and carries no text. The `g4_caps` maximum is unchanged |
| Audit tokens with no edge | 5,123 | 5,077 |

**Each of the 27 removed edges was checked in source to be false.** They come from R4b's corrected receivers:
- `displacements[base + UX].powi(2)` is `f64::powi`, not units' `powi(self, i8)`;
- `by_dof[term.dof].push(` and `terms_w[*g].push(` are `Vec::push` on locals declared `Vec<Vec<…>>` (structural.rs:1047, sparse.rs:1825, verify.rs:800);
- `structure.pattern.column(index)` resolves to sparse.rs:192, and `view.group()` to adaptive.rs:5304;
- `result.contributions.push(` on a `Self {..}` local is a `Vec` field (R4a).

## 3. The stated limits, corrected (replaces G3_REPAIRS.md §1.1's audit summary and §1.2 limit 5)

**§1.1's summary sentence** ("the remaining zero-edge tokens are std or external calls") is replaced by this:

> The remaining zero-edge tokens are calls the resolver attributes to a std type, a derived impl, an external crate or no scanned definition. Each carries its reason in `audit_r4.json`.
> - The typed-receiver reasons now name the type: 11 tokens.
> - One class can still hide a user call: **a name rebound with a different type in the same body** (below).

**Limit 5** is replaced by:

> Adjudication rules replace a call's targets on a cited reading of the receiver type, and a wrong rule is one way to under-count.
>
> **The other is the resolver's flow-insensitive typing.** It types a name once per function body. Where a body rebinds a name with a different type, later calls are resolved on the first declared type. Examples are `let x: A = …; … for x in &bs { x.f() }`, or `let ledger = ledger.finish(n)`.
>
> The unwrap idiom on a parameter is handled (R4c).

**The rest of this class is measured** (`r4/r4_general_measure.out.json`, `CG_R4_GENERAL=1`).

The general rule: a name bound more than once is untyped unless every binding has the same declared type. It adds 191 edges and closes no cycle. Under the precise rule, the true calls it still finds dropped are:
- verify.rs:942–943 and :1059, `t.is_sign_negative()`, `t.abs()` and `t.mul_pow2(` on Wide terms;
- product_certificate.rs:424, `e.cmp_value(`;
- source_recovery.rs:758–760, `folded_force.values()`.

**Their targets and everything below them carry no text:** 12 functions and 0 text sites. They close no cycle. **TEXT and T20 are therefore unaffected at this basis.**

The general rule is not adopted, because its name fan-out is loose. It raises TAV to 3.26 GB, and the text run is then incomplete: one unmapped loop header, behind false fan-out. Adopting it would need a cited adjudication rule for each false fan-out. That is a candidate for a later grant if a future basis puts text behind such a call.

## 4. The one added loop rule

`loop_bounds.r4.json` adds one rule:
- **The loop:** `for (i,entry) in self.summary_coverage.iter().enumerate()` in final_case.rs:317, now reached through R4c.
- **The bound is n.** The function first checks `summary_coverage.len() == body_count` (:316), and bodies ≤ n (G4 PUBLICATION_READER.md §1).
- Its loop body allocates no text, so the bound changes no figure.

## 5. Execution

- **Who:** TASK I65, as a records item inside U4 G5.
- **Tools:** stdlib Python only. No Cargo.
- **Basis:** read from the G4 snapshot at NUM `b1f80234dc` in WT/scratch.
- **Writes:** only in R/I65/u4_g5_01/.
