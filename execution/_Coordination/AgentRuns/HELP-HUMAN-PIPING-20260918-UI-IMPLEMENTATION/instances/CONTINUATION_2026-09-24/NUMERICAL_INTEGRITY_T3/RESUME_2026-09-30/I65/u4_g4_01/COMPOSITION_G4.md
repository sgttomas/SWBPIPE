# T24: composition per caller and mode, the margin rule, and the sensitivity table

**The headline.** At the D1 caps, with illustrative strides and ε = 2 (D1.11), the admission maximum is **0.912 M (sparse) and 0.916 M (dense)**. Both fit M, but both are **over the 0.9 M margin rule**, by 46 MB and 66 MB. The maximum is phase **W3**: publication on branch W. W4, the precommit reader, is within 10 MB of it.

Branch X, the selected finalization, is at 0.855 M / 0.860 M and passes.

Any one of these single-cap reductions brings the maximum under 0.9 M: m ≤ 30, n ≤ 28, g ≤ 24, r ≤ 96, l ≤ 160, or the text cap ≤ 64 (§4). A phase-aware ordinary span, a records-only refinement, also passes: 0.856 M / 0.860 M (§5).

**Basis and method.**
- **Source:** NUM `b1f80234dc` (U1 and U3 grant 1 merged), read from a `git archive` snapshot. NUM moved twice during the grant; RETURN.md explains.
- **Arithmetic:** `_run_records/g4_caps.py` builds T16–T19 and the phases. The chain is t08_closure, ordinary_caps, t25_g4, producer_caps and t07_repair over the text runs.
- **Strides:** g3lib.ASSUMED plus a few G4 strides, all illustrative; G5 evaluates in-build.
- **Outputs:** `g4_caps.caps.eps2.out.json`, `…eps6…` and `…milestone…`.

## 1. The admission law (RR "RV84 on U4 G3", S-5)

`admit` covers the maximum, over both branches, of every phase through caller completion. Each phase is priced as its requested bytes, plus the largest single old backing that can coexist with it (T21; the invocation is single-threaded), plus R = 64 MiB.

| Phase | What is live (beyond R) | Sparse | Dense |
|---|---|---|---|
| **X1** | O without T25 (all of it, conservatively) + T25 (corrected: G3_REPAIRS.md, S-3 and S-4) + TAV_X + T11; moving: the T25 publication text's last growth | 0.8550 | 0.8599 |
| **X2** | X's completion: O + the retained receipt + TAV_X + T11 + the T18 reserve + T19 | 0.4525 | 0.4574 |
| **W1** | the ordinary span: O + TAV_W + T11 | 0.4729 | 0.4778 |
| **W2** | G-B and G-C plus the W1 phases: + T12–T15 + the T18 (N1) reserve | 0.4900 | 0.4949 |
| **W3** | publication: + the staged copy (T18.1) + T16's peak; moving: the publication text's last growth | **0.9115** | **0.9164** |
| **W4** | the precommit: + the successor + the invocation Value + the 13 reader statics + T17's peak | 0.9095 | 0.9144 |
| **W5** | transfer and Direct completion: + the successor + T19 | 0.5387 | 0.5436 |

**The parts, in bytes** (caps, ε = 2):

| Part | Bytes |
|---|---|
| O without T25 | 245,257,549 / 264,967,997 (sparse/dense) |
| T25 | 1,475,185,857 |
| TAV_X | 1,445,193,238 |
| TAV_W | 1,583,158,378 |
| T11 | 204,912 |
| T12–T15 | 68,293,632 |
| T16 peak | 1,389,829,540 |
| Staged copy | 105,778,121 |
| T17 peak | 1,275,668,333 |
| Successor | 190,637,407 |
| Invocation Value | 15,782,080 |
| Reader statics | 5,397,696 |
| N1 reserve | 757,746 |
| T19 | 10,240 |
| R | 67,108,864 |

TAV_X and TAV_W are the per-branch T08 totals: each zeroes the functions the other branch never runs (G3_REPAIRS §2.3). This replaces G3's one whole-invocation TAV, which counted the route twice (S-7).

**The moving terms** (RV84 N-11, enumerated):
- The candidates are:
  - the text sites' largest, 2,599,962;
  - I54's maximum-helper growth, 8,388,608;
  - the N1 reserve's old diagnostics backing, 1,734,016;
  - the T25 publication text, 209,737,314;
  - the T16/T17 publication text, 209,961,095.
- T12–T15 hold no owner whose last growth exceeds 8.4 MB (RV84 N-11's reading).
- G3's undocumented 2 MiB candidate is removed.

**The callers.** Only Direct (D-2). Headless is refused by `admit` (U3 F-5, routed to G5). The thread hop is T19.

**Modes.** The difference comes only from O's dense parity tail (+19.7 MB). Every G4 term is mode-independent.

## 2. The margin rule

| Mode | Admission maximum, E_mov + R | Fraction of M | ≤ 0.9 M? | ≤ M? |
|---|---|---|---|---|
| sparse | 3,670,349,837 | 0.9115 | **no**: over by 46,471,181 | yes |
| dense | 3,690,060,285 | 0.9164 | **no**: over by 66,181,629 | yes |

**At ε = 6** (today's D1 without D1.11, for the table only), W3 is at 1.43 M.

**At the milestone's own facts** (the same expressions, `which = milestone`), the maximum is X1 at 338 MB, which is 0.084 M.

**The rule trips. ROOT chooses among §4–§6.**

## 3. Why W3 and W4 are the maximum

**At W3**, W's whole text total (1.58 GB) is live together with three envelope-sized things:
- T16's publication hash route, about 1.13 GB: the json! wrapper copy, the to_string text at ≤ 2e, the checked-parse tree with seen keys, the canonical text at ≤ 2e, and RV84 S-3's per-string temporary;
- the successor Value being built, 0.19 GB;
- the staged typed copy, 0.11 GB.

**W4 is the same shape.** The accepted reader clones the whole successor at G1 (retained_precision.rs:591) and hashes the publication again.

**All of O is still counted** at W3 and W4 (§5).

## 4. Sensitivity table (`_run_records/sensitivity.out.jsonl`, `sens.py`)

**The method.** Each row re-runs the whole cap-priced chain at one cap vector, from composite text through `g4_caps`, with the diagnostics fixpoint D recomputed. The call graph and lexicon are structural, so they are reused.

| Cap changed (others at D1) | D | TAV (whole) | Sparse max | Dense max | Dense X1 | Dense W4 | Passes 0.9 M? |
|---|---|---|---|---|---|---|---|
| none (D1 caps), ε = 2 | 17,574 | 2.145 GB | 0.9115 | 0.9164 | 0.860 | 0.914 | no |
| none, **ε = 6** | 17,574 | 2.145 | 1.4297 | 1.4346 | 1.378 | 1.433 | no (> M) |
| n = 30 | 17,542 | 2.121 | 0.9027 | 0.9069 | 0.854 | 0.905 | no |
| **n = 28** | 17,510 | 2.096 | 0.8938 | 0.8974 | 0.849 | 0.896 | yes |
| n = 24 | 17,446 | 2.048 | 0.8764 | 0.8787 | 0.837 | 0.877 | yes |
| n = 16 | 17,318 | 1.953 | 0.8408 | 0.8414 | 0.815 | 0.841 | yes |
| **m = 30** | 17,354 | 2.072 | 0.8841 | 0.8890 | 0.836 | 0.887 | yes |
| m = 28 | 17,134 | 1.998 | 0.8550 | 0.8600 | 0.811 | 0.858 | yes |
| m = 24 | 16,694 | 1.852 | 0.8005 | 0.8056 | 0.763 | 0.804 | yes |
| m = 16 | 15,814 | 1.559 | 0.6895 | 0.6948 | 0.667 | 0.694 | yes |
| g = 28 | 17,342 | 2.089 | 0.8952 | 0.9001 | 0.845 | 0.898 | no (dense) |
| **g = 24** | 17,110 | 2.035 | 0.8793 | 0.8842 | 0.830 | 0.882 | yes |
| g = 16 | 16,646 | 1.931 | 0.8483 | 0.8532 | 0.801 | 0.851 | yes |
| s = 16 | 17,574 | 2.123 | 0.9060 | 0.9109 | 0.855 | 0.909 | no |
| s = 0 | 17,574 | 2.106 | 0.9014 | 0.9063 | 0.851 | 0.904 | no |
| r = 144 | 17,286 | 2.119 | 0.9004 | 0.9053 | 0.847 | 0.903 | no |
| **r = 96** | 16,998 | 2.094 | 0.8891 | 0.8940 | 0.834 | 0.892 | yes |
| r = 48 | 16,710 | 2.069 | 0.8780 | 0.8829 | 0.821 | 0.881 | yes |
| **l = 160** | 16,134 | 2.094 | 0.8808 | 0.8857 | 0.830 | 0.884 | yes |
| l = 128 | 14,694 | 2.044 | 0.8510 | 0.8559 | 0.800 | 0.854 | yes |
| l = 96 | 13,254 | 1.996 | 0.8216 | 0.8265 | 0.771 | 0.825 | yes |
| l = 48 | 11,094 | 1.925 | 0.7777 | 0.7826 | 0.729 | 0.781 | yes |
| text cap 112 | 17,574 | 2.121 | 0.9037 | 0.9086 | 0.852 | 0.907 | no |
| text cap 96 | 17,574 | 2.097 | 0.8959 | 0.9008 | 0.844 | 0.899 | no (dense) |
| **text cap 64** | 17,574 | 2.049 | 0.8803 | 0.8852 | 0.829 | 0.883 | yes |
| raw totals ½ (8,192 values; 32,768 B strings and keys) | 17,574 | 2.145 | 0.9076 | 0.9125 | 0.852 | 0.908 | no |
| raw totals ¼ | 17,574 | 2.145 | 0.9057 | 0.9106 | 0.848 | 0.906 | no |

**Reading the table:**
- **m dominates.** It drives P, C, the integrity message text and the T25 payload.
- **l is next,** through D, the diagnostics and the per-load text.
- **The text-cap row varies the `ident` class only.** Text(row)'s 1,024-byte result-id class is held fixed, which is conservative. A smaller text cap would also shrink Text(row).
- **RV84's levers:**
  - **N-6** (price T25 only where selection is feasible) acts only on X1, which already passes. It does not move W3.
  - **S-7** (the double-counted route text) is **already applied**. The G4 text is rooted at the Direct entry only, and `run_linear_static_preview_observed` is capped at one run per invocation, a U3 budget satisfied by U3 grant 1's dispatch (TRANSFER_COMPLETION.md §3, B-1).

## 5. An additional lever: the phase-aware ordinary span

**What it changes.** After G-C the observed ordinary run has returned (PP lib.rs:2934; G-C at :2938), so its locals are gone. Of O, only these remain:
- the captured raw request (15.8 MB);
- the returned envelope's own backings: the results Vec (1.2 MB), the preview tree (0.2 MB) and the diagnostics Vec (5.0 MB). Their strings are already in TAV.

Every other O family is dead at W3–W5: the built model, assembly, the T07 attempt, staging, the rewrite, the aggregation clones, and the typed request (moved into the run).

**The effect.** Counting only those 22.2 MB at W3–W5, the maximum becomes **0.8561 M (sparse) and 0.8599 M (dense)**. X1 is then the maximum (`lever_phase_aware_O` in the output).

## 6. Options for ROOT, each with a cost and risk line

1. **Tighter caps (ROOT's preferred kind).** Candidates with at least a 1% margin: m ≤ 30 (0.889 M), g ≤ 24 (0.884), l ≤ 160 (0.886), r ≤ 96 (0.894), text cap ≤ 64 (0.885). A comfortable margin needs m ≤ 28 (0.860) or l ≤ 128 (0.856).
   - **Cost:** a D1 cap change (DOMAIN.md §2 and one census constant in G5), then a re-run of this chain, which takes minutes (`sens.py`). The milestone (m = 1, l = 3, g = 4) is unaffected.
   - **Risk:** narrower D1 coverage. With a thin margin (m = 30, 1.1%), G5's in-build strides could trip the rule again.
2. **The lifetime-aware text model (a separate grant).** TAV_W (1.58 GB) is the largest term at W3 and W4, and it counts every transient String as live for the whole invocation.
   - **Cost:** a new derivation grant, about 1–2 days plus review.
   - **Risk:** a more complex model that is harder to review. Its benefit is likely the largest of all, over 1 GB.
3. **A bound evaluated per invocation at the actual census facts.** The monotonicity lemmas allow it. At the milestone the maximum is 0.084 M.
   - **Cost:** G5 adds an allocation-free evaluator of the same expressions in retained_memory.rs, plus tests.
   - **Risk:** several atoms are derived, not census facts (D, D_env, the text atoms, the fixed byte classes). They stay cap-priced or need their own per-invocation formulas. The permit also becomes input-dependent, which needs explaining to reviewers.
4. **(Not on ROOT's list) The phase-aware ordinary span (§5),** a records-only refinement.
   - **Cost:** about 1 h of records, as a G4 addendum or at G5, plus review of the per-family "dead after G-C" argument.
   - **Risk:** low to moderate. The argument is local to Rust drop order at the end of `run_linear_static_preview_observed`, but it must be checked family by family.

**My recommendation.** Combine 1 with 4 if ROOT accepts a records refinement. If not, take 1 alone with m ≤ 28 or l ≤ 128, for a margin that absorbs stride differences. Option 2 is worth scheduling later, for headroom beyond D1.
