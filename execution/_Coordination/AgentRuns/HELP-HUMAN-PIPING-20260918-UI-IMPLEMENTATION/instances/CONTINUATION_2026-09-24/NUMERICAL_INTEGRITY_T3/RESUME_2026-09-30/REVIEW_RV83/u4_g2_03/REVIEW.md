# RV83 confirmation: R-1, R-2 and R-3 in I65's G4, and the status of B-2

**Candidate:** `R/I65/u4_g4_01/` at NUM `406e4c8916`. SHA256SUMS verifies 77/77.
- The relevant part is G3_REPAIRS.md §1, §2, §3, §6 and §7, with its run records.
- The ruling is RR "U4 G4: the margin rule trips; l ≤ 128 adopted…".

**Basis:** G4 reads source at `b1f80234dc`, so I read that revision from a `git archive` snapshot in WT/scratch (core tree `eb9c9d8312`). Since then U3 grant 1d has changed four PP files; I did not use them.

**Scope:** whether my G3 findings R-1 to R-3 are repaired, and whether B-2 can close. RV87 reviews G4 as a whole and RV84 its own items; I do not repeat that work.

**Abbreviations:** RX = R/REVIEW_RV83/u4_g2_03/_run_records; G4R = R/I65/u4_g4_01/_run_records.

## Verdict: NOT CONFIRMED (narrowly)

| Item | Status |
|---|---|
| R-2 | Fixed |
| R-3 | Fixed |
| R-1 | **The two dropped-call classes I reported are fixed.** My evidence re-checks exactly: 64/64, 33/36, and zero remaining generic or chained drops. **The stated limits that replace "never under-counts" are not yet honest.** They say a wrong adjudication rule is "the one way to under-count", and the audit labels every remaining zero-edge token "std or external". Two further receiver forms still drop calls to user methods; see R-4. |
| B-2 | **Its text criterion is met.** By my probes, every text-producing site on the D1 graph is now covered by a stated bound. The text below every dropped R-4 target is zero, so R-4 moves no text figure. |

**New findings:** 0 BLOCKING, 1 SHOULD-FIX (R-4) and 2 NOTE.

**If ROOT accepts R-4 as a carry-over** (a resolver fix plus corrected limit wording, at the l ≤ 128 addendum or G5), R-1 and B-2 close and this verdict becomes CONFIRMED.

## Per item

### R-1: dropped calls, re-checked on the repaired extractor

**My G3 probes re-run unchanged on G4's graph** (G4R/callgraph_edges.json, Direct root, 2,698 reachable functions):

| Probe | G3 graph | G4 graph | Output |
|---|---|---|---|
| Generic-receiver calls whose same-named definitions are unreached | 24 calls, 64 hidden functions | **0** | RX/rv83_missed_edges.g4.out.json |
| The same, where the target is reachable by another path | 15 | **0** | as above |
| Chained calls (`f(x).g(`, `x?.g(`) with unreached same-named definitions | 21 names, 44 hidden | 11 names, 30 functions, **0 text rows** | RX/rv83_chained_calls.g4.out.json |

**My G3 hidden sets, mapped to G4 nodes** by (file, name) (RX/rv83_g3_evidence_on_g4.py):
- the generic-receiver set is reached in full: 64/64;
- the chained-call definitions are reached 33/36.

**The three exceptions match I65's account, which I confirmed in source:**
- primitive_loads `positive` is called only from FK, and FK has no dependencies (frame_kernel Cargo.toml `[dependencies]` is empty);
- adaptive.rs's two `into_legacy` are not the target of combine.rs:116. `solve_recorded` returns `RecordedCombination` (combine.rs:119–123), whose own `into_legacy` is at :91.

**The 11 remaining chained names are correctly unreached.** I spot-checked them:
- `with_formation_source` narrows by R1l to the `Assembled*` impls. `StructuralSystem::assembled` returns `AssembledStructuralSystem` (structural.rs:133–140), and `SparseStructuralSystem::assembled` returns `AssembledSparseStructuralSystem` (sparse.rs:919–926);
- `prepared_lane_work` is called only inside `#[cfg(test)]` blocks (retained_product.rs:3694, :3722);
- the unreached `summary_coverage` (final_case.rs:1463) is a free fn, so `.summary_coverage(` cannot call it;
- the rest are std or out-of-crate names: `parse`, `transpose`, `value`, `eq`, `add`, `div`.

**FKS/formation_check.rs is reached and priced** (G4R/text_budget.caps.out.json): `detail`'s four `format!` sites and `check`'s one each have multiplicity 224, and the text budget is `complete: true` with 0 unmapped and 0 unclassified.

**The stated limits are not yet honest (R-4).** G3_REPAIRS.md §1.2 limit 5 says a wrong rule is "the one way to under-count", and §1.1 says "the remaining zero-edge tokens are std or external calls".

My own probe (RX/rv83_zero_edge_calls.py) differs from I65's: it uses its own lexer, blanks `cfg(test)` items, and attributes each token to its innermost function. It finds 24 calls to the enclosing impl's own methods that still produce no edge, in two forms (RX/rv83_self_receivers.out.json):

**A receiver typed `Self`.** This covers a local bound to `Self {..}` (20 calls) and a parameter typed `Self` or `&Self` (4 calls). I65's resolver types the receiver as `Self` and finds no methods owned by "Self". Examples:
- `extent.has_valid_fractions()` (primitive_loads lib.rs:1099);
- `value.sig()` (FK wide.rs:289);
- `element.frame_element()?` (straight_pipe lib.rs:441);
- `matrix.stored` and `matrix.set_stored` (sparse_direct lib.rs:271–272);
- `donor.ensure_valid()`, `b.net()?` and `out.checked_lme()` (wide_sum.rs:154, :357, :716);
- `result.node(` and `result.element(` (structural_adapter.rs:1123–1166);
- `addend.add(` and `other.neg()` (pressure_exact.rs:109, :81).

I65's audit records these 34 tokens under the reason "receiver type Self has no such method (std/external)". That is not true: the impl does have the method.

**A self-call inside index brackets,** such as `&self.columns[self.row_range(row)]` (sparse.rs:181, :1679; also :1644). The receiver pattern swallows `self.columns[self` and types the call as a `Vec` method; the audit says "receiver std Vec: std method".

**The effect is small. I measured each part:**
- the true targets carry **no text** below them: 0 B per execution, 0 B requested, from I65's own row sizes;
- they close **no cycle**: no caller is reachable from its target (RX/rv83_self_receivers.out.json, `cycles_closed_by_dropped_edges` = []). T20's "no mutual recursion" therefore stands;
- **two functions are unreached because of these drops:** structural_adapter.rs:1204 `node` and :1217 `element`, which are AssemblyEvidence builder steps. They are non-recursive, and their errors are `&'static str`.

### R-2: `validate_profile` priced — fixed

- The `fn_zero` exclusion is gone. The edge `validate_profile → problem` carries `edge_per_call: l` (G4R/loop_bounds.g4.json), with its D1.3 reasoning.
- In G4R/text_budget.caps.out.json, `validate_profile` has M = 3 (lib.rs:2358, pressure_runtime.rs:435 and source_receipt.rs:162) and `problem` has M = 576. The five sites (format, diag, join, and two `to_string`) total **25,622,784 B**. That is a loose upper bound against my about 0.4 MB at real spellings; loose is sound.
- `build_pressure_case_with_members`'s callees after `if has_blocking(diagnostics) || !is_exact(model) { return None; }` (pressure_runtime.rs:435–438) are edge-zeroed. That is correct: `validate_profile` is called before the return, and its edge is kept.
- `check_suffixes` is zeroed with a correct citation: it is called only inside `if exact`.

### R-3: the O-N provenance parse — fixed

G4R/ordinary_caps.py:112–120 adds the row "O-N per-load provenance parse transient": `48·Node(String,Value) + 384·s(Value) + 384`, which is **48,000 B** at the illustrative strides.

**I checked that it bounds every realizable parse of a ≤ 128-byte non-object JSON text** (D1.10):
- **array slots:** push-built arrays need at most Σ max(4, 2h) ≤ 4·arrays + 2·elements ≤ 4·64 + 2·64 = 384 slots;
- **object nodes:** non-empty objects contribute at most 25 + ⌊25/5⌋ nodes, and the row allows 48, counting empty objects as well;
- **strings and keys:** at most 128 bytes, exact capacity;
- **error and scratch:** one `ErrorImpl` (40 B) and the parser's escape scratch (≤ 2·len).

The node and slot allowances cannot all be used within 128 bytes, so the parser scratch, which is not labelled separately, is absorbed. One transient lives at a time. The "about 40 B" text is withdrawn (G3_REPAIRS.md §7).

### B-2

**Every file from my G2 list is now covered by a stated bound or a correct exclusion:**
- the five files covered in my u4_g2_02 confirmation;
- FKS/formation_check.rs, now reached (R-1);
- pressure_runtime.rs, now priced (R-2);
- self_weight.rs, excluded under D1.10 with its parse priced (R-3).

**No text sits behind any edge that is still dropped** (R-4), so B-2's own criterion, a text bound over the whole D1 call graph, is met. The closure waits only on ROOT accepting R-4 as a carry-over.

## New findings

| ID | Sev. | Where | Evidence | Remedy |
|---|---|---|---|---|
| R-4 | SHOULD-FIX | G4R/callgraph_g4.py (receiver typing); G3_REPAIRS.md §1.1 (audit summary) and §1.2 limit 5 | **Two receiver forms still drop calls:** receivers typed `Self` (24 calls) and self-calls inside index brackets. The audit labels both "std/external", and limit 5 says a wrong rule is the only under-count route.<br>No text is lost and no cycle is hidden, but two functions are unreached. | **In the extractor:** resolve `Self`, whether a local bound to `Self {..}` or `Self::..`, or a parameter typed `Self`, `&Self` or `&mut Self`, to the enclosing impl's owner; and stop the receiver pattern at `[`.<br>**In the limits:** state these classes, or remove them by the fix, and correct the "std/external" labels.<br>Then re-run TEXT and T20 and compare. Both should be unchanged apart from the two functions. |
| R-N4 | NOTE | ROOT ruling (l ≤ 128) | The addendum re-runs the chain at l = 128. R-2's `edge_per_call: l` and R-3's per-load term follow `l` automatically. | Nothing to do beyond the addendum. |
| R-N5 | NOTE | G4R/text_budget.caps.out.json, `function_multiplicity` | `validate_profile` M = 3 counts source_receipt.rs:162 (`captured_load_state_case`, load-state only) and pressure_runtime.rs:435. Both are conservative in D1. | None. |

## Execution record

- **Who and when:** RV83, TASK (Type 2) under ROOT, with no descendants. Started 02:00 MDT and finished within the 90-minute box.
- **Memory guard:** PID 5387, running at the start.
- **Not run:** Cargo, rustc, solver, native and DEC-025 jobs. Git use was reads only (`rev-parse`, `log`, `status`, `diff --stat`, `show`, and `archive` into WT/scratch), with `GIT_OPTIONAL_LOCKS=0`.
- **Run:** stdlib Python over I65's committed G4 records and the snapshot.
- **Writes:** RX/ and this REVIEW.md, plus WT/scratch/rv83_u4_g2_01/ (the snapshot). Nothing went to the system temp directory, and no write left my fence this time.
- **Run records** (RX/), all covered by SHA256SUMS:
  - `rv83_missed_edges.g4.out.json` and `rv83_chained_calls.g4.out.json`: my u4_g2_02 probes, unchanged, re-run on G4's graph;
  - `rv83_g3_evidence_on_g4.py` and its output;
  - `rv83_zero_edge_calls.py` and its output;
  - `rv83_zero_edge_impact.py` and its output;
  - `rv83_self_receivers.py` and its output;
  - `ORIGINS.json`.
