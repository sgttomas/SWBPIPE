# RV83 confirmation: R-4 in I65's U4 G5

**Candidate:** `R/I65/u4_g5_01/R4_CALLGRAPH.md` with its `_run_records/` (`run_r4.sh`, `callgraph_g5.py`, `r4/`), and part 2's erratum N-6 (`part2/IMPLEMENTATION_PART2.md` §9), at NUM `ac5b318c5b`. Both SHA256SUMS files verify.

**Basis:** G4's `b1f80234dc`, read from my `git archive` snapshot in WT/scratch (core tree `eb9c9d8312`), which is the basis I65 used.

**Scope:** the three points ROOT asked about. RV89's spot-check and RV87's S-2 are not repeated.

**Abbreviations:** RX = R/REVIEW_RV83/u4_g2_04/_run_records; G5R = R/I65/u4_g5_01/_run_records/r4; G4R = R/I65/u4_g4_01/_run_records.

## Verdict: CONFIRMED

All three points hold. My probes add three NOTEs, none of which moves a figure.
- N-1: one unwrap form, `.as_deref_mut()`, is still unhandled.
- N-2: one wording detail in limit 5 is imprecise.
- N-3: the deepest call chain grows from 39 to 40 frames.

**Counts:** 0 BLOCKING, 0 SHOULD-FIX and 3 NOTE.

## 1. My 24 dropped calls now produce edges, and the audit labels are corrected: confirmed

**Every token from my u4_g2_03 list now has an edge on the R-4 graph** (RX/rv83_r4_tokens.py over G5R/edges_r4.json; 26 rows, counting the duplicate `find` and `row_range` tokens). For each, the edge runs from the innermost caller to the enclosing impl's own method:
- `has_valid_fractions`, `sig`, `frame_element`, `stored` and `set_stored`;
- `ensure_valid`, `net` and `checked_lme`;
- `row_range`, including both bracketed self-calls at sparse.rs:181 and :1679;
- `find`, `node`, `element`, `geometry`, `orientation` and `properties`;
- `add` and `neg`.

**None of these tokens appears in `audit_r4.json` any more.**

**My earlier probes on the R-4 graph show the same:**
- 0 `local = Self` or `param: Self` drops remain;
- the 29 remaining "inside [..]" tokens are std calls (`len`, `max`, `min`, `abs`, `as_str`) with no user target (RX/rv83_self_receivers.r4.out.json).

**The reachable set grows from 2,698 to 2,705, and nothing is lost** (RX/rv83_reach_diff.py). The seven new functions are exactly I65's list:
- `node` and `element` (structural_adapter.rs:1204, :1217);
- `solve_binary64` (:282);
- final_case.rs `of`, `rederive_coverage`, `coverage_facts` and `check_summary_coverage`.

**The edge changes:** +33 and −27. I spot-checked the removed edges in source and they are false. The true targets remain: `structure.pattern.column(index)` resolves to SparsePattern::column (sparse.rs:192), and `view.group()` to adaptive.rs:5304.

**The labels:** "receiver type Self has no such method (std/external)" is gone. Typed-receiver reasons now name the type (11 tokens, matching R4_CALLGRAPH §3), and the `Self::f` rows read "(derived or external)". All are `Self::default()` uses.

## 2. Limit 5's new wording is accurate: confirmed, with two NOTEs

**The class is stated correctly.** The resolver types a name once per body, so a name rebound with a different type can lose calls. I read every remaining typed-receiver audit row, and each one is such a rebinding:
- verify.rs:942, :943 and :1059, where `t` is a Wide term;
- product_certificate.rs:424 `e.cmp_value(`;
- source_recovery.rs:758–760 (`let folded_force = folded_force.finish(n)`) and :529 (a closure parameter `row`);
- canonical_json binary64.rs:352–353, which are std `Vec` calls.

**NOTE N-1:** the sentence "the unwrap idiom on a parameter is handled (R4c)" overstates the case by one form.
- At adaptive.rs:4087 (`solve_precision`) and :4288 (`verify_precision`), inside each `macro_rules! run` body, the code reads `if let Some(trace) = trace.as_deref_mut() { trace.requested(`, with `trace: Option<&mut RunTrace<'_>>`.
- R4c's pattern accepts only `as_mut`, `as_ref`, `as_deref` and `take` (callgraph_g5.py, `unwrap_re`). The call is therefore resolved on `Option` and labelled "receiver std Option: std method", although the true target is `RunTrace::requested` (origins.rs:615).
- That function stays unreached on the R-4 graph, and §3's measured list of dropped true calls (r4_general_measure.out.json) omits it.
- **The effect is nil.** Its subtree is 2 functions with 0 text sites, and it closes no cycle (RX/rv83_unwrap_forms.out.json). My probe for unrecognised unwrap forms on parameters finds only these two tokens.
- The case is still inside the stated rebinding class. The remedy is to add `as_deref_mut` to `unwrap_re`, or to name the four handled forms in the limit.

**NOTE N-2:** "later calls are resolved on the *first* declared type" is imprecise.
- The default resolver (callgraph_g5.py:614–619) keeps the parameter type, or the type of the **last** typed or `Type::`/`Type {`-initialized `let` in the body, and applies it to every call on that name regardless of position.
- Untyped rebindings (`for`, closure parameters, patterns, untyped `let`) do not change it.
- For the examples given, this agrees with "first". It differs only where a body has two explicitly typed bindings of one name.
- Suggested wording: "resolved on one declared type per name (the parameter's, or the last explicitly typed binding's)".

## 3. TEXT and the recursion inventory are unchanged: confirmed, with one NOTE

I compared row by row, using RX/rv83_r4_unchanged.py, G4's l ≤ 128 outputs (G4R/text_budget{,_env,_X,_W}.caps.l128.out.json) against R-4's (G5R/text_budget{,_env,_X,_W}.caps.r4.out.json):

| Run | G4 TAV | R-4 TAV | Moving | D | Rows differing |
|---|---|---|---|---|---|
| full | 2,044,161,940 | 2,044,161,940 | equal (2,046,761,902) | 14,694 / 14,694 | 1 |
| env | 615,069,498 | 615,069,498 | equal | 9,360 / 9,360 | 1 |
| X | 1,356,903,574 | 1,356,903,574 | equal | equal | 1 |
| W | 1,511,208,042 | 1,511,208,042 | equal | equal | 1 |

**The one differing row is N-6's,** and it is identical in all four runs: load_ledger.rs:133 `into_text` in `push`. Only its multiplicity changes (full: 915,466,156 → 915,000,428). Its requested bytes stay at 163,840, because they are capped by the explicit per-invocation total.

**The other counts also match the erratum:**
- `function_multiplicity` gains exactly N-6's 10 entries: six `typed_trace`, `summary_coverage`, `project`, `coverage_facts` and `check_summary_coverage`;
- `sites_with_positive_multiplicity` is 1,426 in both.

**The admission summary is identical:** `g4_caps.caps.eps2.l128` equals `g4_caps.caps.eps2.r4` key for key, at 0.8510 / 0.8559 M.

**The recursion inventory is identical:** the explicit cyclic components are 22 = 22 and the implicit ones 40 = 40, as identical component sets.

**NOTE N-3:** `root_depths` for the Direct root goes from 39 to 40 (G4R/callgraph.out.json against G5R/cg_r4.out.json); R4_CALLGRAPH.md does not state this.
- The extra frame comes from the newly reached chains.
- On G3_REPAIRS §3's arithmetic (frames, plus 18 Value levels, plus 36 reader schema levels, at ≤ 16 KiB each), the total moves from 93 to 94 frames, about 1.47 MiB, still under the stated 1.6 MiB.
- R = 64 MiB and k = 16 are unaffected. The G6 record should carry 40.

## Execution record

- **Who and when:** RV83, TASK (Type 2) under ROOT, with no descendants. Started 05:21 MDT and finished within the 1-hour box.
- **Memory guard:** PID 5387, running at the start.
- **Not run:** Cargo, rustc, solver, native and DEC-025 jobs. Git use was reads only (`rev-parse`, `log`, `status`, `diff --stat`), with `GIT_OPTIONAL_LOCKS=0`.
- **Run:** stdlib Python over I65's committed records and my existing `b1f80234dc` snapshot.
- **Writes:** RX/ and this REVIEW.md, plus WT/scratch/rv83_u4_g2_01/. Nothing went to the system temp directory.
- **Run records** (RX/), all covered by SHA256SUMS:
  - my u4_g2_03 probes re-run on the R-4 graph: `rv83_zero_edge_calls.r4.out.json`, `rv83_self_receivers.r4.out.json`, `rv83_zero_edge_impact.r4.out.json`;
  - `rv83_r4_tokens.py` and its output;
  - `rv83_unwrap_forms.py` and its output;
  - `rv83_r4_unchanged.py` and its output;
  - `rv83_reach_diff.py` and its output;
  - `ORIGINS.json`.
