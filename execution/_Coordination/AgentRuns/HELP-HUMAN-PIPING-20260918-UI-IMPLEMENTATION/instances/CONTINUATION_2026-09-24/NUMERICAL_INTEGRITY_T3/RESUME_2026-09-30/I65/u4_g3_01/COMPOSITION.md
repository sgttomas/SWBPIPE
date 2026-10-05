# Producer composition at the caps: T11–T15, T21, the per-mode tables and the gates

**The headline.** At the D1 caps, with ASSUMED layouts, the exact-block-selected branch does **not** fit M under the current D1. With the proposed **D1.11** (no control characters in input strings) and **D1.10** it fits with about 0.51 GB of headroom.

The W1 branch's G3 part needs about 1.93–1.95 GB, which leaves about 2.08–2.10 GB for G4's T16–T19. Both modes behave the same. The dominant term is T25's canonical hash of the whole envelope (RESIDUALS_G3 §T25), followed by text (TEXT.md).

| Mode | ε | Branch X: exact-block selected, E_mov + R | Fits M = 4,026,531,840? | Branch W: W1, G3 part, E_mov + R | Headroom left for G4 |
|---|---|---|---|---|---|
| sparse | 6 (current D1) | 5,543,811,429 | **no**: over by 1.52 GB | 1,930,160,927 | 2,096,370,913 |
| sparse | 2 (with D1.11) | 3,500,376,509 | yes: 526,155,331 spare | 1,930,160,927 | 2,096,370,913 |
| dense | 6 | 5,563,521,877 | **no**: over by 1.54 GB | 1,949,871,375 | 2,076,660,465 |
| dense | 2 | 3,520,086,957 | yes: 506,444,883 spare | 1,949,871,375 | 2,076,660,465 |

**The components** (ASSUMED, caps, sparse/dense):

| Component | Bytes |
|---|---|
| O without T25 | 245,209,549 / 264,919,997 |
| T25 | 3,074,072,410 (ε = 6) / 1,439,324,474 (ε = 2) |
| Text TAV | 1,540,955,362 (moving +2,599,962) |
| T11 | 204,912, including the U1 grant 2 delta |
| T12–T15 | 68,293,632 |

R = 64 MiB, per STACK_INVENTORY.md. Arithmetic: `_run_records/compose_caps.py` and `compose_caps.out.json`. Every figure depends on illustrative strides; G5 evaluates the same expressions in-build.

## 1. Branch structure

These follow I51 COMPOSITION §2 and D-15.

**After exact-block arbitration, each case takes exactly one of two branches:**
- **X, selected.** Legacy recovery selects. W1 is bypassed and SOURCE-BLOCKS finalization (T25) runs while the W1 observation prefix is still installed. G-A's ordinary span must cover it:
  `E_X = O(without T25) + T25 + Text + T11(early observation)`.
- **W, W1 runs.** The source is not selected (including a legacy attempt that failed, whose T07 prefix is inside O). G-A covers the ordinary span. G-B then admits the late old-source capture, and G-C the preparation, native run, lanes, certificate and C3 trace, while the complete ordinary owner stays live. Here O is kept whole, which is conservative:
  `E_W = O(without T25) + Text + T11 + T12 + T13 + T14 + T15 [+ G4: T16–T19]`.

**The admission decision.** `admit` is evaluated before the branch is known, so the profile's G-A bound must cover both branches: `max(E_X, E_W + G4 terms)`.

Text (T08) is a whole-invocation total-allocation-volume bound, so it is added to every branch.

## 2. The producer terms T11–T15 (`_run_records/producer_caps.py`)

Each row transcribes an accepted roster and evaluates it at the caps:
- `V_T(x)` uses the push law s(T)·PushCap(x) (I54 V3);
- `C_T(x; q)` ≤ s(T)·(4q + 2x);
- clones are exact;
- `Wide<L>` ≤ 16 + 8L by per-field rounding of {bool, i64, [u64; L]} (FK/wide.rs:204–208).

| Term | Roster | Caps (ASSUMED) |
|---|---|---|
| **T11** observation | `ProductCapture` fields at U1(a) `59a5de2032`: nodes, materials, selections, basis record, solver observations, member/term/support/spring identities, facts, operational records. **U1(a) F4 `OrdinarySeed`, counted from source:** one per case, holding the case id; `InitialSeed` (a cloned `StructuralError` ≤ 8N bytes, or a `FrameKernelError` with no heap, or the report code and reference); `W2Seed` (a cloned `RangeTrigger` and `ForceScalingFailure`, ≤ 8N each); `FindingSeed`, which is None in D1 because nodal-only loads return before `decide_row`; the D5 and legacy references. The late old-source capture `P1_old_source` (`Option<PrimitiveSource>`: SourceParts plus the surviving constrained and body arrays) | 204,912 (OrdinarySeed: 11,968; U1 grant 2's `invocation_digest`: 64) |
| **T12** preparation | I29 P1 §5: InputMap and registries with owned ids; SourceParts and constructor sort scratch; the source clone; the exact ledger (λ_acc = 68 limbs); prescriptions and identity encodings (E_src = 36,742; E_stf = 4,986; E_led = 107,914); layout and extents (Q = 1,472); body maps; geometry; pattern and tagging (U = 2,528; P = 4,640); RCM and free blocks | 1,996,618 |
| **T13** native | I29 P3 §2's conservative alternative: RetainSuperset, meaning 4 S_p, 4 Z_p, 3 VS_p with reports and one cached refusal, the attempts, geometry, the selected finish with publication and evidence (K4RST at L = 16), plus the sum of every scratch row (P3 §4, conservative over max). Precisions p = 128/256/512/1024 with L = 4/4/8/16, residual R = 4/8/16/16 and verification W = 8/16/16 | 42,775,846 |
| **T14** lanes, projection and maximum, final certificate | I51 §3: two lanes, each with SourceBridgeView masks N + 2B, laws, correction buffers and enclosures; projection rows, conversions and aliases ≤ P_final; one maximum helper at 262,144·s(Node) (I54 BOUND:76); the final certificate's verdicts, coverage and intervals (final_case.rs prepared reservations) | 23,204,752 |
| **T15** C3 trace | I51 §4: ≤ m prepared-member events, 9m conversions, 2 lane terminals, ≤ P_final final-row conversions, 2m operational records, one adapter snapshot and the recorded invocation and case | 316,416 |

**Monotonicity.** Every row is a sum of nonnegative terms in PushCap, linear counts, `max` and the sort law, so the lemma of ORDINARY.md §3 applies unchanged.

**What T13 is not.** It uses P3's coarse retained-superset alternative, not a per-event union. It is deliberately loose, and still small at the caps.

## 3. T21 moving terms

The invocation is single-threaded, so at most one reallocation is in flight at any instant. Each span's moving bound is therefore its requested bound plus the **largest single old backing** that can coexist with it, not a sum over owners:

| Span | Largest old backing | Bytes |
|---|---|---|
| X (ε = 6 / 2) | the T25 publication text's last doubling | 616,260,332 / 207,573,348 |
| W | the maximum helper heap's growth, 131,072·s(Node) (I54 BOUND:76), against the text sites' largest 2,599,962 | 8,388,608 |
| T07 within O | the identity JSON's last growth | 1,797,413 |

Smaller old backings are listed per family in the output files (`O_mov_extra_largest_old_backing`, `moving_extra`).

## 4. Gates and the facts they read (G2 API.md hand-off)

The cap-pricing rule (plan §2.3): every gate checks (a) that the actual facts available at that point lie within the caps, and (b) that the cap-priced bound of the phases still ahead is ≤ M. The bounds below are derived from the caps. A fact above its bound means the derivation missed something, and the gate refuses to the ordinary path instead of overrunning. The facts are borrowed, allocation-free reads.

**`LateFacts`, read at G-B** (immediately before the late old-source capture, and only with `source_selected == false`):

| Field | Read from | Must be ≤ |
|---|---|---|
| `legacy_selected` | the case's selected source | false. G-B is reached only on branch W |
| `case_rows`, `case_rows_capacity` | the case's `results` Vec | R0 = 1,891 rows |
| `case_row_text_bytes` | Σ capacity of each row's Strings | 2·R0·Text(row) = 43,394,668 |
| `diagnostics`, `diagnostics_capacity` | `Vec<Diagnostic>` | D = 11,030 |
| `diagnostic_text_bytes` | Σ capacity of id, code, severity, message, source and refs | 2·Text(diag_total) = 155,625,688 |
| `retained_error_text_bytes` | maximum and record error Strings | (3m + 1)·Text(err) = 1,589,248 |
| `observation_bytes` | `ProductCapture` capacity arrays (adapter, observation, support, prepared) | T11 bound |
| `built_counts` | BuiltModel node, member and support lengths | n, m, g (re-check) |

**`CompleteFacts`, read at G-C** (after the complete ordinary owner returns):

| Field | Read from | Must be ≤ |
|---|---|---|
| `envelope_results`, `…_capacity` | `MechanicsEnvelope.results` | P_final = 2,115 |
| `envelope_result_text_bytes` | Σ row String capacities | 2·P_final·Text(row) = 48,535,020 |
| `envelope_diagnostics`, `…_capacity` | `MechanicsEnvelope.diagnostics` | D = 11,030 |
| `envelope_diagnostic_text_bytes` | as for LateFacts | 155,625,688 |
| `contract_evidence_census` | the borrowed `Value` census of `contract_evidence` (values, objects, string and key bytes) | the PREVIEW facts in ordinary_caps.py |
| `source_cases`, `source_case_row_json_census` | `source_cases`: count, and the `actual_rows` Value census | 1; the RowJSON facts |
| `ordinary_seed_bytes` | `ProductCapture.ordinary` capacities | the T11.4 bound |
| `selected_source` | — | none (branch W) |

**`PhaseRefusal { gate, fact, observed, cap }`**, as in G2 API.md.

The gate arithmetic, including M−1, M and M+1, is G5's. G3 only fixes the fields and their cap-derived bounds.

## 5. What ROOT should decide from this

1. **D1.11** (G2_AMENDMENTS §6). Without it, branch X is over M at the caps, so `admit` would refuse every D1 invocation at G-A: the cap-priced bound already exceeds M. That is safe, but no permit could ever be minted. With D1.11, branch X fits.
2. **D1.10** (G2_AMENDMENTS §5). The text bound assumes it. Without it, the same method gives about 10.6 GB of text, and neither branch fits. The alternative is to price the self-weight path per load: about 1–2 h, stated as a remainder.
3. **G4 design-to-budget.** Branch W leaves about 2.08 GB under M for T16–T19 (ASSUMED layouts). T16 and T17 hash and validate the successor envelope by the same route as T25. At ε = 2 that costs about 1.4 GB in T25's terms; at ε = 6, about 3 GB. So D1.11 is needed for G4 as well, and the margin is thin.
4. **Tightening is advisable.** Text (1.54 GB) is now the second-largest term, and it counts every transient String as live. A lifetime-aware text model (TEXT.md §6), or smaller caps on m, g or l (which reduce D, P and the envelope), would restore a comfortable margin for G4. I recommend ROOT schedule the lifetime-aware text refinement before G4 closes its composition.
