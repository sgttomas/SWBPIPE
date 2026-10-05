# RV86: independent review of U5, the milestone's reference comparison

**Verdict: PASS, with stated limits.** U5 (`R/I61/u5_reference_01/`, at NUM `fc5d92c56c`) establishes that both pinned milestone successors match their independent reference. In each mode, all 97 published class claims agree:

- 25 `relative_verified` rows agree within 1e-9;
- 69 `absolute_verified` rows agree within their receipt bounds;
- 3 `input_derived` rows are exact.

They agree against both of I50's oracle readouts, and against a closed-form reference RV86 wrote without the oracle's code. The worst actual relative error is 8.1e-17. The limits are in §4.4.

**Findings:**

| Severity | Count |
|---|---:|
| BLOCKING | 0 |
| SHOULD-FIX | 2 |
| NOTE | 5 |

TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN). No descendants. 2026-10-04.

**Host and lane:**
- The memory guard (PID 5387) was running at the start and at the end.
- Python only, Git reads only (`GIT_OPTIONAL_LOCKS=0`). No Cargo, installs, solver, native or DEC-025 jobs.
- The reader shells out to I52's prebuilt checked-JSON and units binaries, as U5 did. Their hashes are unchanged from I61's `inputs_sha256.txt`.
- All Python ran with `-B` and `PYTHONDONTWRITEBYTECODE=1`, so nothing was written into NUM or WT/f2a-facade.
- Writes went only to this folder and WT/scratch/rv86_u5_reference_01/.

## Findings

| ID | Severity | Item | Finding | Action |
|---|---|---|---|---|
| S-1 | SHOULD-FIX | 4 | The RETURN blames the seven represented-readout misses on A and Z, which differ by 2 ulps. Those rows depend on neither. They depend on J, through the receipt's k_t = G·J/L. I50's represented J is about 4.3 ulps above the exact annulus J. The receipt's k_t is the correctly rounded exact G·J/L, 4 ulps below k_t formed from I50's J. "Exactly the J-dependent rows" also misses N1 rx: it depends on J, but J contributes only 1.0e-4 of its value, so it shows no miss. The RETURN's conclusion is correct; only the named cause is wrong. | Correct the wording, in place or as an addendum. No rerun is needed. |
| S-2 | SHOULD-FIX | 6 | U5 can only be replayed from a 1,059,560-byte local-only scratch log. It reads only 7,240 bytes of it: mode, request, source map and facts, per record. Replaying from that extract reproduces `u5_report.json` and `u5_run.log` byte for byte. | Commit the extract, which RV86 provides in `_run_records/`, and let the script accept it under a pinned hash. Otherwise ROOT should accept the local-only dependency explicitly. |
| N-1 | NOTE | 1, 2 | U5 does not reuse oracle lines 130–155, which derive each row's class, scale and bound; it takes them from the reader. RV86 ran that formula on the successors. With the receipt's section terms, it reproduces every class, S\* and bound bit for bit. With I50's represented A and Z, it differs on 21 stress rows (S\* and bound 4 ulps lower, 2 ulps on the maximum), with the same classes. | Say in the RETURN that this block was deliberately not reused, and why. |
| N-2 | NOTE | 2 | The RETURN says the oracle gives no reference for the mode and parity rows. The omitted oracle lines 120–129 do check their values: mode bits 1.0 or 2.0, parity present only in dense, finite and ≥ 0. RV86 applied these checks; they pass in both modes. | Optional: add these checks to the script. |
| N-3 | NOTE | 3 | The comparison takes the published bound as given, so a bound inflated ×2 passes it. An inflated bound is refused by the reader instead (G1 unsealed; G5c CLASSIFICATION_MISMATCH resealed). RV86's independent recount (§2.3) confirms the actual bounds. | A stated limit of the comparison layer. |
| N-4 | NOTE | 5 | The overlay's extrema interval, [8.170865639376682e-92, 8.170865639376692e-92] Pa, does not contain the exact maximum, 0 Pa. Its declared scope excludes solution error. The interval and `global_upper_bound_pa` lie within the maximum row's bound (8.22e-18 Pa) of the truth. | Record this fact. Optionally add the within-bound check. |
| N-5 | NOTE | 3 | U5's own negative controls are coarse: two rows, each moved to twice its allowance. RV86's one-ulp edge controls, class swaps and bound corruptions are all refused (§3.2). | None needed. |

## 1. Is the oracle reuse faithful?

**Yes.** Check `_run_records/c1_slices.py`.

**The pins bind the right files.**
- The script asserts the oracle's sha256, `b1b58639…4d56`. That equals the committed file and I50's SEAL entry: 17,396 bytes.
- It asserts the log's sha256, `5ced66b5…0b69`. That equals I50's BULK_MANIFEST entry for `runtime02/pp_debug_final.log`: 1,059,560 bytes.
- The successors and the reader are pinned outside the script, in `inputs_sha256.txt` and in the report's `successor_file_sha256`. RV86 re-hashed them:
  - the successors equal U1's pins and U3's stub-dispatch outputs, `ac6986b0…` and `6cd1d249…`;
  - the reader `d77008e2…` is byte-identical at NUM `fc5d92c56c`, at `bee3dc07ca`, and in WT/f2a-facade at `4b31bbf23a`.
- The packet's SHA256SUMS verify, and its folder is clean at the revision under review.

**The slices are exact and unmodified.** Each anchor occurs exactly once in the oracle. The script executes three slices:

| Oracle lines | Content | Glue added |
|---|---|---|
| 1–55 | the helpers | none |
| 56–115 | `check` through the nested `truth` | one appended `return truth` |
| 214–230 | the observable checks | wrapped as `def observables(record,rows,m)` with `return True` |

The derivation runs on I50's record. The successor is bound to it by the asserted request equality. RV86 confirmed that the request is also type-strictly equal to the committed fixture `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json` (sha256 `2aa51bee…`).

**What is omitted.**
- Lines 116–213:
  - the mode and parity checks against I50's captured observations (N-2);
  - the oracle's own scale, class and bound derivation, and its per-predicate comparison with I50's private verdicts (N-1);
  - the `len(results)==97` assertion. U5 counts 97 class claims per mode instead.
  - G5a, which needs I50's native resolution and does not apply to the successor.
- Lines 231–239: the return value and the CLI.

**The tolerances are unchanged.**
- The relative test uses the oracle's own DecimalSi and DecimalRaw expressions, `abs(F(n))/10**9` and `abs(F(r['value']))/10**9`.
- The informational test uses its SharperExact expression.
- The verdict rule is the oracle's: pass if hi ≤ a, fail if lo > a, otherwise unproved.
- The oracle took the maximum over both readouts; U5 tests each readout separately and stops on any non-pass. These are equivalent.

**Two choices are not the oracle's, and both are correct.**
- **The class claim.** A `relative_verified` row is held to relative 1e-9 on the published value. D1 §4.1.6 states this ("Relative 1e-9 on a published q"), and says the stop rule is "operational evidence … not a forward-error enclosure". SharperExact is therefore informational. I50's four predicates were checks of the candidate's own predicate claims, and the successor publishes none.
- **The source of the bounds.** Absolute allowances are the published (receipt) bounds, not the oracle's recomputed ones. "Within its published class" requires this; see N-1 for what the oracle's own computation gives.

## 2. Is the comparison complete and correctly mapped?

**Yes.** Checks `c2_classes.py`, `c3_truths.py` and `c5_ancillary_extrema.py`.

### 2.1 Independent counts from the successor bytes

These counts come from RV86's own implementation of D1 §4.1.6 and §4.1.6.1, applied to each file's rows, its invocation and its receipt section terms:

| Mode | Rows | relative | absolute | input_derived | non_quantity | not_covered |
|---|---:|---:|---:|---:|---:|---:|
| sparse_interactive | 98 | 25 | 69 | 3 | 1 (mode) | 0 |
| dense_scrutiny | 99 | 25 | 69 | 3 | 2 (mode, parity) | 0 |

RV86's results agree with the successors and the reader at every point:
- The computed per-kind S\* equal the receipt's `body_scales` bit for bit. L_b = 3.
- `input_derived_dofs` (N0 UX, UY, UZ) equals the invocation's rigid restraint set.
- The independently computed absolute set equals the receipt's `absolute_verified` list exactly: the same 69 ids and the same bound bits.
- The receipt's `not_covered` list is empty.
- Every class, `scale_bits` and `bound_bits` equals the reader's.

The row lists in the RETURN match this partition.

### 2.2 The mapping, checked against an independent reference

`c3_truths.py` is RV86's own closed-form solution, written without the oracle's code. The member carries pure torsion, T = M·e = 3α; this holds because the applied couple is exactly parallel to the chord.
- **N0** rotates by θ = M/k.
- **N1** translates by θ × (1,2,2) and rotates by θ + T·L·e/(G·J).
- **The actions:** the torsional moment is −T at end i and +T elsewhere; τ = T·c/J. Every other action, stress and maximum is 0.
- **The supports:** the rigid support reacts zero. Spring i reacts −M_i on its own axis, with zero force and zero off-axis moments.
- **The arithmetic:** exact rationals, π by Machin's formula with alternating-series bounds, and isqrt for the norms.

Results:
- Every one of the 97 class-claim rows, in both modes, overlaps the oracle's recorded source-annulus and represented intervals.
- All 97 class claims pass against RV86's exact-annulus truth.
- The largest relative error on a `relative_verified` row is 8.1e-17, on N0 ry and rz.

How each group of rows is mapped:
- **Support rows.** All 32 rows (four supports × eight) are mapped by kind, entity and component through the oracle's `truth`. The oracle's support-norm guards also run on the successor and pass.
- **The maxima.** `pipe_elastic_normal_stress_maximum_v2` maps to 0, which is correct under pure torsion. It is `absolute_verified` and within its bound.
- **The overlay.** The `pipe_stress_extrema` patch satisfies the oracle's midpoint identity exactly.
- **The headlines.** `max_displacement` (result:disp:N1) and `max_open_formula_stress` (the elastic-maximum row) satisfy the oracle's headline identities.
- **The ancillary rows.** The mode row (bits 1.0 sparse, 2.0 dense, with a matching basis) and the dense-only parity row (finite, ≥ 0) pass the value checks the oracle states (N-2).

No row needed a reading, and none was reinterpreted.

## 3. Rerun and negative controls

### 3.1 Rerun

The unchanged script was run twice: with the reader root at NUM, and with the recorded reader root WT/f2a-facade. Each run took 0.34 s.

**The report reproduces byte for byte:** `u5_report.json` sha256 `b8546c97…87b3`, and `u5_run.log` `271eeeee…9268`. The exit status was 0 and the stop list `[]`.

### 3.2 Negative controls

These are in `c4_controls.py` and `c4b_abs_edge.py`.

**Method.** The controls ran through the unchanged script; its text is hash-checked. The real reader validated the unmutated file. A wrapper then applied each mutation to the rows or receipt and to the reader's returned classifications, so that the script's consistency assertions still held and the comparison itself had to decide.

The harness baseline reproduces `b8546c97…` exactly. Edge values were found by bisection on the raw bits, using the actual script. Every edge control was then run in both modes.

| Control | Rows | Result |
|---|---|---|
| One ulp beyond the class: relative | N1 ry (rad), torsional shear end i (MPa), N1 uy (mm) | Each one-ulp pair straddles the 1e-9 edge. The inside value passes. The outside value is refused (Relative1e-9Si and Raw fail), in both modes. |
| One ulp beyond the class: absolute | element axial force (N), N1 ux (mm), bending stress y end i (MPa) | A value exactly at the bound passes. The next ulp is refused (AbsoluteBound fails), in both modes. |
| Edge on rigid Fx | rigid Fx | Refused on both sides by the oracle's support-norm guard, before the class test. This shows the observable checks are live; the controls above isolate the class test. |
| Swap two rows' classes, reader and receipt together | torsional shear ↔ bending stress y; N1 ry ↔ rigid Fx; N0 ux (input) ↔ N0 rx (relative) | All refused, by the misclassified row failing its class. Examples: the swapped torsional shear fails AbsoluteBound; the swapped bending stress fails Relative1e-9; N0 rx fails InputDerived exactness. |
| Swap two rows' classes, reader only | the first two swaps | Refused by the script's receipt-bound assertion. |
| Corrupt one bound: shrink below the actual error | rigid Fx | Refused (AbsoluteBound fails). |
| Corrupt one bound: receipt only, 1 ulp off | rigid Fx | Refused by the receipt-bound assertion. |
| Corrupt one bound: inflate ×2, reader and receipt | rigid Fx | **Passes the comparison**, because a larger bound is a weaker claim (N-3). |
| Bytes-level mutations, run end to end | inflated bound; value moved by 4e-9; class swapped | All refused by the reader. Unsealed: G1 RECEIPT_MISMATCH. Resealed with the reader's own `_hash`: G5c CLASSIFICATION_MISMATCH for the bound, G5 PRODUCT_ATTEMPT_MISMATCH for the moved value, and G5c for the swap. So the reader refuses a moved value before the oracle comparison sees it. |

## 4. The informational finding

### 4.1 Is the attribution correct?

**The conclusion is correct; the named cause is not.** See S-1. The section terms compare as follows (check `c3_truths.py`):

| Term | Successor receipt | Versus I50's represented value |
|---|---|---|
| A | the correctly rounded exact annulus, 0 ulps | 2 ulps below |
| Z | the correctly rounded exact annulus, 0 ulps | 2 ulps below |
| k_t = G·J/L | the correctly rounded exact G·J/L | 4 ulps below k_t formed from I50's J |
| k_a | 0 ulps from the exact value | 3 ulps from E·A/L formed from I50's A |

I50's represented J is about 4.32 ulps above the exact annulus J.

**The seven rows depend on J, not on A or Z.** They are N1's y and z rotations (θ + twist) and the five torsional-shear stations (T·c/J). N1 rx also depends on J, but J contributes only 1.0e-4 of its value, so it shows no miss.

How the seven rows sit against the readouts, in multiples of the SharperExact allowance:

| Rows | Readout separation (annulus vs I50 represented) | Published error vs exact annulus | Published error vs I50 represented | Readout 3: receipt's k_t bracket |
|---|---:|---:|---:|---|
| N1 ry, rz | 0.999 | 0.053 | 1.053 | pass |
| Torsional shear, 5 stations | 4.875 | 0.521 | 4.354 | unproved: the half-ulp k_t bracket is too coarse at this level |

The other 18 relative rows pass the sharper test against all three readouts.

**What follows:**
- **For torsional shear,** the readout separation exceeds twice the allowance, so no published value could meet the sharper bound against both readouts. This is I50's dual-readout obstruction, which is a property of the two readouts, not of the producer.
- **The successor tracks the exact source annulus,** as D1's prepared formation states.

### 4.2 Does it affect the class claims?

**No.** The class claim is relative 1e-9. The actual errors are at most 8.1e-17 relative, against both readouts and against RV86's own reference. The represented readout differs from the annulus by about 1e-16 relative, which is seven orders of magnitude inside 1e-9.

### 4.3 Is it a defect?

**No; it is a faithful consequence of the representation.**
- **The producer** uses the correctly rounded annulus section, as the prepared route specifies.
- **The contract,** D1 §4.1.6, promises relative 1e-9 above the floor and the absolute bound below it. It calls the stop rule operational evidence, not an enclosure. It publishes no sharper claim.
- **The comparison** correctly reports the sharper test as information only.

There is one caution. The "represented" readout here is I50's ordinary-route representation, not the successor's own. The successor's own representation, its receipt k_t, never fails the sharper test: 20 rows pass and 5 are unproved, with 0 fails.

### 4.4 Does "matches its independent reference" hold?

**Yes, with these limits:**
1. **It covers the published class claims only.** In each mode these are 25 relative rows at 1e-9 (in SI and in the published unit), 69 absolute rows within their receipt bounds, and 3 input rows exactly. The claims hold against both oracle readouts.
2. **It does not extend to the stop rule's sharper bound** measured against I50's ordinary represented section. That bound is not published, and on 7 rows it cannot hold against both readouts at once.
3. **It does not include enclosure of the exact maximum by the overlay's extrema interval** (N-4). The maximum row itself is within its bound.
4. **It rests on specific bytes:** those written by PP's committed test and by the disposable-stub dispatch of the actual Direct entry, which have identical hashes. Re-confirmation on the maintained facade under a real permit waits for U3 grant 2. The script is reusable unchanged.
5. **The correctness of the classes and bounds rests on the reader's G5c,** confirmed independently by RV86's recount (§2.1). The oracle comparison alone accepts an inflated bound (N-3).

## 5. The extrema intervals

**The milestone claim does not need the interval to enclose the truth, and that assertion would be false.** The patch's `enclosure_scope` reads "supplied_binary64_polynomial_coefficients; solution and coefficient formation error are separate". In both modes the interval is [8.170865639376682e-92, 8.170865639376692e-92] Pa, and the exact maximum is 0 Pa, outside it. I61 was right not to assert enclosure, but its stated reason ("the oracle does not state that check") understates the case. The record should say plainly that the interval excludes the exact value, by design.

**The milestone claim is about the published maximum row.** Against its class it passes: 8.17e-92 Pa against a bound of 8.22e-18 Pa. The midpoint and headline identities also pass.

**The oracle can cheaply supply one more consistent check.** Every interval endpoint, and `global_upper_bound_pa`, lies within the maximum row's absolute bound of the exact truth. RV86 checked this, and it holds in both modes (`c5_ancillary_extrema.py`). The `global_upper_bound_pa` does bound the exact value from above. `value_lower_pa` is not a lower bound on it.

## 6. Reproducibility

To reproduce U5, a later reviewer needs these inputs:

| Input | Where it is | Status |
|---|---|---|
| The committed oracle | `R/I50/first_publishing_component_02/named_oracle.py` | committed |
| The committed script | U5's `_run_records/u5_compare.py` | committed |
| The reader | at NUM | committed |
| I52's prebuilt checked-JSON and units binaries | WT | local; rebuildable by Cargo, outside this lane |
| The two successor files | WT/scratch | local; regenerable by PP's committed `u3_permitted_path_publishes_the_pinned_successor`, which is a Cargo job, and pinned by hash |
| I50's log | WT/scratch | local-only |

**From I50's log, U5 reads only four fields per record:**
- `mode`;
- `request`: 2,747 bytes, equal to the committed fixture (sha256 `2aa51bee…`);
- `source`: I50's captured source map, 774 bytes;
- `facts`: I50's represented A, D, I, J, Z, c and t.

The request, source and facts are identical across the two modes.

**RV86 derived a 7,240-byte extract,** `_run_records/i50_oracle_inputs_extract.log` (sha256 `a66a8a49…4fda`), using `_run_records/make_extract.py`. That script asserts the log's hash. A copy of `u5_compare.py` whose only change is the pinned input hash (`_run_records/u5_compare_extract_variant.diff`) reproduces `u5_report.json` and `u5_run.log` **byte for byte** from the extract.

**If the scratch log is lost,** the extract plus that two-line change suffice, provided the extract is committed (S-2).

**The facts cannot be derived cheaply without the log.** They are the ordinary route's binary64 section values, and would need a Cargo run or a re-implementation of PP's ordinary section formula. Committing the extract is the cheap path.

## For ROOT to rule on

1. **The wording of the claim.** Adopt §4.4's limits as the stated scope of "matches its independent reference" for U7's condition. In short, the claim covers the published class claims only, not the sharper stop-rule bound and not extrema truth-enclosure.
2. **S-2, reproducibility.** Either commit the extract (with the script accepting it) or accept the local-only dependency explicitly.
3. **S-1, the attribution wording.** Decide whether the correction must land before U7 or may follow as an addendum. It changes no result.

## Records (`_run_records/`)

| File | Content |
|---|---|
| `run_rv86.sh` | the invocations, with placeholders |
| `c1_slices.py`, `.out` | the slice anchors, the executed and omitted oracle lines, and the tolerance expressions |
| `c2_classes.py`, `.out` | the independent D1 class, S\* and bound recount; its comparison with the receipt and the reader; the oracle formula with I50's facts and with the receipt's section terms |
| `c3_truths.py`, `.out`, `.rows.json` | the independent closed-form reference; the overlap with the oracle's intervals; the class claims; the three-readout sharper analysis |
| `c4_controls.py`, `.out`; `c4b_abs_edge.py`, `.out` | the negative controls through the unchanged script |
| `c5_ancillary_extrema.py`, `.out` | the mode and parity value checks; the extrema interval against the truth and the bound |
| `make_extract.py`, `i50_oracle_inputs_extract.log`, `u5_compare_extract_variant.diff` | the reproducibility extract and its replay |

The mutated successor files and the per-control reports are in WT/scratch/rv86_u5_reference_01/ only. All paths here are placeholders.
