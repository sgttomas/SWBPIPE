# T3 V1: combined backcheck R3 (DESIGN r3, S11 r4, D2 r3)

Type 2 TASK V1, 2026-09-26. The T3 manager asked for this pass as the last design check before the selection package.

- **Scope.** Read-only. Standard-library Python only, single-threaded, at `nice 19`. No cargo, no build, no test suite, no Git write. T1 was read only through `git show f3270ea79`.
- **Inputs on the T3 branch at `11d24caf5`.** Each sha256 was verified against the working tree; the branch head has since moved to `d84e66bff`, and none of these files changed:
  - `T3/DESIGN_NUMERICS/DESIGN.md` revision 3 (`48f35144…`);
  - `T3/DESIGN_NUMERICS/S11_CONTAINMENT.md` revision 4 (`2be14785…`), diffed against `_run_records/S11_CONTAINMENT_revision3.md` (`561c7200…`);
  - `T3/DESIGN_STANDING/DESIGN.md` revision 3 (`43c7d672…`), §4.5.2, §4.9.3, §4.9.9, §5 I-8 and §12.1;
  - `T3/ROOT_RULINGS_V1.md`, `T3/ROOT_RULINGS_V2.md` and `T3/ROOT_SELECTION_S11.md`;
  - `T3/REFERENCE_CHECK/RETURN.md` §3.5 and `_run_records/v2_floor.json`;
  - D1's `DESIGN_NUMERICS/_run_records/floor_kinds.{py,json,stdout.txt}` and `T3/REFERENCES/references.json` (R1 revision 2, `c0f14201c`, the revision ROOT froze).
- **Line numbers.**
  - `D1:n` is DESIGN revision 3, `S11:n` is S11 revision 4 and `D2:n` is DESIGN_STANDING revision 3.
  - `PP` is `P/core/product_physics/src/lib.rs` at main `c61a540ea`. The product code on the T3 branch is identical to it.

## 1. Verdict: **FINDINGS**

Every BACKCHECK_R2 finding is resolved as ROOT ruled: S8-R, the final S2-R wording, SCALE-W in K2b and F2-P. S11 revision 4 applies R3-1, R3-2, R3-N1 to R3-N4 and most of R3-3 faithfully. D1's not-covered counts reproduce exactly from its probe and V2's records.

Nothing is BLOCKING. There are five SHOULD-FIX items:

- **R3B-1.** The floor for derived stress rows ignores how action errors propagate through `hypot` and the intensification factor. For `pipe_elastic_normal_stress_maximum_v2`, a `relative_verified` stress row can be off by up to 1.32e-9 relative. For the intensified measure the bound is i·1.32e-9. Both exceed 1e-9, in a thin band just above the floor.
- **R3B-2.** The kind mapping leaves out two row kinds that the retiring identities publish: `reaction_resultant`, and `open_formula_stress_summary`, which is the source of the stress headline. Both would therefore be withheld. Separately, the design never defines "physical", yet it treats every unlisted row as `relative_verified` by default.
- **R3B-3.** The per-member section terms (A, I, Z, E, G, L) are not pinned for bit-exact recomputation in Python and TS. The product forms them with `powi`, and the formula differs by route.
- **R3B-4.** D2's G5b and G5c do not yet mirror D1 §4.1.6.1 exactly:
  - they have no stress kind;
  - input-derived rows have no class;
  - the `not_covered` list is checked in one direction only.
- **R3B-5.** S11's rule 8 matches accumulations function by function. ROOT's R3-3 condition names every individual accumulation. So a new published sum inside an already-listed function in `PP` would pass the test.

**D-5 is pending.** D1's W1 trigger ("not Passed") does not reach P1's Passed breach, RF-SKEW-T-CANT-OFF-122-r1e-04. D1 is writing `DESIGN_NUMERICS/D5_TRIGGER.md`, which I will check narrowly when the manager sends it (§6).

## 2. BACKCHECK_R2 findings: resolved as ruled

| Finding | Ruling | Revision 3 | Status |
|---|---|---|---|
| **S8-R: classification on the published value** | ROOT: classify on the published binary64 value | D1:355 `absolute_verified` iff `|q| < fl(R·S*)`, "never classifies on the unpublished precision-p value"; D1:754 receipt lists; D2:470 G5c recomputes | **Resolved** |
| **S8-R: R = 2^-34** | R is an exact binary64 constant | D1:352-353. **D1 is right that 10^9·2^-64 is exact:** it equals 1953125·2^-55, bits `0x3DCDCD6500000000` (probe R). So 2^-34 (`0x3DD0000000000000`, checked) is chosen for margin, not exactness. The margin is sound: 2^30/10^9 = 1.0737, so above the floor `2^-64·S*/|q| ≤ 2^-30 ≈ 9.31e-10`. That leaves 7 % for publication rounding (u ≈ 1.1e-16) and for the published-versus-2p S\* difference, which is of order (2^-64 + u)·S\* because each published row is within that of q_2p. "`fl(2^-34·S*)` is exact whenever S\* is normal" is slightly too strong (N-2) | **Resolved** (N-2) |
| **S8-R: S\* recomputable** | D2's G5 gains S\* and the classification check | D1 §4.1.6.1 (D1:363-381) pins bodies, the kind mapping, unit factors, S(kind), the L_b formula and the coupling order. The section terms in item 7 are not pinned (R3B-3) | **Resolved with R3B-3** |
| **S8-R: withholding, never Passed** | Withheld from Current, or explicitly exempt; never counted as Passed | D1:358 and D2 §4.9.9 (D2:519-548): quantity-level withholding, rule binding refused, headline refused, class carried in canonical forms, no exemption. The VP harness never counts a not-covered comparison as a pass (D1:672-679) | **Resolved** |
| **S2-R** | "the successor identity's standing is no worse than the retiring identity's, case by case, in all three languages" | Verbatim at D1:485 and D2:327, plus identical fail-closed standing per language; the H-a subset is `needs_recompute` under both identities (D2 §4.9.5) | **Resolved** (see R3B-2 on quantity-level withholding) |
| **SCALE-W** | The kernel half of formation-time scaling goes in K2b | D1:554 and the K2b row D1:787: scaled formation in `FK/lib.rs` and the sparse assembly entry that takes b; F1 wires it (D1:794); serialization order D1:803 | **Resolved** |
| **F2-P** | Twist and extension get their own kinds; anything still below the floor is not covered and never counts as a pass | Per-member S\*_tw = S\*(moment)·(L/GJ)_m and S\*_ext = S\*(force)·(L/EA)_m (D1:339-340, D1:380). The harness derives twist as T/k_t and extension as N/k_a from published T and N, never by differencing (D1:654). Three-number report and enumerated list (D1:670-685). The 3 RF-CANCEL rows use the binding net-governed scale (D1:664, D1:683) | **Resolved** (counts in §3; N-1, N-3) |

## 3. Not-covered counts (probe F)

I checked D1's `floor_kinds.json` row by row against V2's `v2_floor.json`.

| Quantity | Result |
|---|---|
| Variant B, D1's recomputation against V2's | **Identical sets, 43 rows.** D1's probe reproduces V2's variant B exactly |
| Variant F (D1's rule) | **46 RF-WEAK + 2 RF-SKEW**, and **3 RF-CANCEL** under the net-governed scale (`F-G1e80-GnG-ORTHO u.N1.UY`, `M-G1e80-GnG-ORTHO u.N1.UY` and `F-G1e80-GnG-INPLANE Mb.M2.mid`), as D1:682-684 lists |
| F against B | F contains all of B. The difference is exactly 5 rows: `tw.C` and `ext.C` in W-AX-rho1e-12, `tw.C` in W-3D-rho1e-08, and `tw.M1` in RF-SKEW-T-CANT-AX-122-r1e-12 and -345-r1e-12. **This matches D1's description** |
| V2's variant A twist and extension flags | **314**: 295 outside RF-WEAK and 19 inside. F clears 303 of them and keeps 11. The 11 are the 2 `tw.M1` rows, which were variant A flags, and 9 RF-WEAK rows already in B. So "the 295 … are cleared" (D1:27, D1:685) is off by two (N-1). The enumerated list itself is correct |
| Variant B rejected | D1:685 argues that under B a structurally zero twist has S(twist) equal to noise at 2p, so a stop rule on B would never accept it. **I agree, and there is a stronger reason.** Under F, twist and extension are not product rows (D1:343). Their accuracy is inherited from T and N, so the harness floor has to be tied to T's and N's floors. B's own-magnitude scale is tied to nothing the product guarantees. So F is the rule consistent with the product's guarantee, and a variant-B pass would claim more than the receipt does |

RF-SKEW `tw.M1` is covered consistently. Its torque is about 1e-12 of the moment scale, so in the product the torque is `absolute_verified` and withheld. The harness's correspondence check (D1:680) ties the two together.

## 4. The D1/D2 interface (I-8)

| Item | D1 revision 3 | D2 revision 3 | Match |
|---|---|---|---|
| R's bits in the receipt, equal to the registered constant | D1:752 | G5c D2:470 | Yes |
| t = fl(R·S\*), `abs(q) < t` on the published value | D1:355 | G5c | Yes |
| Bound fl(2^-64·S\*) per `absolute_verified` row, bit for bit | D1:754 | G5c | Yes |
| Selected-case rows and retained-state combination outputs; unselected rows carry no class | D1:355, D1:381 | G5c | Yes |
| Kind mapping, unit factors, S(kind) = max \|q\|, L_b formula, coupling order | D1:368-379 | G5b D2:469 ("exactly as D1 revision 3 specifies") | Yes by reference |
| Per-member **stress** S\* = fl(fl(fo/A) + fl(mo/Z)), carried in the receipt | D1:341, D1:380, D1:753 | G5b names translation, rotation, force, moment, twist and extension only | **No** (R3B-4a) |
| Input-derived rows: no S\*, `relative_verified` by §5's default | D1:342, D1:374, D1:754 | G5c computes t "for that row's body and kind", which is undefined for them | **Gap** (R3B-4b) |
| `not_covered` ids listed in the receipt | D1:754 | G5c checks only that listed ids are outside the mapping, not that every such row is listed | **One-directional** (R3B-4c) |
| Section terms for twist, extension and stress | "as the product forms it" (D1:380) | "checked against the invocation model with D1's formula" | **Formula not pinned** (R3B-3) |
| Consumer rule | D1:358: withheld, `RULE_QUANTITY_BELOW_VERIFIED_FLOOR`, never Passed, headline refused, envelope standing kept | D2 §4.9.9: the same, with a second code `RULE_QUANTITY_NOT_COVERED` for `not_covered` | **Yes** (codes differ, N-5) |
| Gate condition 3 (S2-R) | D1:485 | D2:327 | Yes, verbatim |

D2's I-8 (D2:559-565) says that where D1 differs, "S-G follows D1 and keeps the structure". That covers the intent. The text should still be exact before the package, because G5b and G5c are what the three readers get built from.

**D2 §12.1 probes.** I read them against the design. PR-5 and PR-7 confirm the capture and canonical-profile limits that the receipt encoding (D1:764) relies on. PR-6's display conversions (underflow shown as `converted` 0, the sign of −0 dropped) are display-only and create no reliance path. F-P2 and F-P7 are ruled (ROOT_RULINGS_V1). No design change follows.

## 5. S11 revision 4: the R3 fixes

| Item | ROOT's condition | Revision 4 | Status |
|---|---|---|---|
| **R3-1** | Pre-register T1's support-motion fixtures and their derived documents; scope "bit-identical" to all-zero prescribed values; report diffs with sizes before regeneration | S11:141, S11:327-328 (scoped claim), S11:451 (list, catch-all for any other request with a nonzero prescribed value, stop to the T3 manager and ROOT before regeneration). **All 18 listed files exist at `f3270ea79`.** Only `load_reference/connected.request.json` and `load_reference_source/eigen_motion.request.json` carry support-motion fields there, so the list is complete. The condition's "with the size of each change" is covered by S11:451's report to ROOT. Stating the sizes explicitly in S11-K's PR record would match the condition's wording | **Applied** |
| **R3-2** | F8 (G, 0.3, −G), on the precondition list, with cancelling thermal strains; K4 axial-effect case (G, n, −G); M1h–M1k and M1b killed | S11:492 (F8 with loads (G, 0.3, −G) and thermal strains (ε, ε_n, −ε), the Sterbenz trap stated); S11:458 (F8 added to the precondition list); S11:470 (K4 axial-effect loads in orders (G, n, −G) and (n, G, −G), G ∈ {1e8, 1e80}, n = 0.3). Both orders fail the binary64 fold at both G, so the preconditions can hold. Kill table S11:500 | **Applied** |
| **R3-3** | The §2.5 table becomes the site test's constant; every floating-point compound assignment or sum/fold in the named modules must appear in it; the `nonlinear_integration:1333` unit-force solves are allow-listed | Rule 8 (S11:279), with §2.5 as the constant (S11:168), and limit 4 corrected (S11:270). **The constant is keyed by function name** (R3B-5) | **Applied, with R3B-5** |
| R3-N1 | Allow-list the unit-force vectors | S11:269, lines 1332-1337, which I checked on main | **Applied** |
| R3-N2 | Scale before rounding | `round_scaled(e)` (S11:191, S11:328): one rounding after an exact power-of-two scaling | **Applied** |
| R3-N3 | Record V1's scan | S11:166 | **Applied** |
| R3-N4 | Fix the label count | M1a–M1o over 15 sites (E1–E13, E15, E16), with the mapping at S11:500 | **Applied** |

## 6. Findings

**R3B-1 (SHOULD-FIX): the floor for derived stress rows leaves out error propagation.**
- Derived stresses are computed from the once-rounded actions (D1:326). Their S\* is `σ(m) = fl(fl(fo/A) + fl(mo/Z))` (D1:380). The stress kind includes `pipe_elastic_normal_stress_maximum_v2` and the intensified measures (D1:373).
- The product forms those rows as follows:
  - the maximum as `|Nw/As| + hypot(My, Mz)/Z` (`PP:2751-2753`);
  - the equal-factor measure as `i·hypot(σ_by, σ_bz)` from the published component rows (`PP:15344-15363`).
- The stop rule bounds each moment component's error by 2^-64·mo. The error in `hypot(My, Mz)` can therefore reach √2·2^-64·mo, and in the intensified measure i·√2·2^-64·mo.
- At the threshold t = 2^-34·σ(m), probe S gives these worst-case relative errors for a row classified `relative_verified`:

  | Row | Worst relative error |
  |---|---|
  | Circular maximum | 1.32e-9 |
  | Intensified, i = 1.15 | 1.51e-9 |
  | Intensified, i = 2 | 2.63e-9 |
  | Intensified, i = 5 | 6.59e-9 |

  All of these exceed the 1e-9 that S8-R's classification promises.
- The excess applies only within a factor √2·i above the floor, about 1e-10 of the body's stress scale. So it cannot change a realistic rule outcome. It is still a breach of the guarantee the classification exists to enforce.
- **Required change.** Give each stress row kind its propagation factor k. Use k = 1 for the component stresses, √2 for hypot-based rows and √2·i for intensified rows, each as a pinned binary64 constant rounded upward. Then set σ_k(m) = fl(fl(fo/A) + fl(k·fl(mo/Z))). An equivalent fix is to classify stress rows against 2^-34/k. D2's G5b mirrors the result.
- **Also state** that `displacement_magnitude` and the reaction magnitudes are formed at p and rounded once. D1:322 implies it. If they were formed from rounded components, the same issue would arise with a factor of √3.

**R3B-2 (SHOULD-FIX): the kind mapping leaves out two kinds the retiring identities publish, and "physical" is not defined.**
- `reaction_resultant` (N, `PP:2230-2239`) and `open_formula_stress_summary` (MPa, `PP:2799-2808`) are not in D1:369-374.
  - Physics-source-1 recovers both today (`P/core/product_physics/src/source_receipt/rows.rs:397`, `:548`, `:741`; `P/core/reporting/result_export/src/source_blocks.rs:458`, `:476`).
  - Under D1:375 they become `not_covered`, so they are withheld.
  - `open_formula_stress_summary` feeds the `max_stress` headline (`PP:2790-2797`). So on the ordinary pressure route the headline would be refused for every selected case.
- This fails closed. But the cost is not stated anywhere, and gate condition 3 compares envelope standing only. The gate would pass while the successor withholds rows that the retiring identity publishes as Current.
- D1:375 makes "any other **physical** row kind" `not_covered`. D1:754 makes "every other published quantity" `relative_verified`. "Physical" is not defined, so a row the producer judges non-physical is bindable by default, and no reader check applies.
- **Required change.**
  - Enumerate every row kind the three base identities emit, taken from their semantic-contract tables, and give each one class: mapped kind, input-derived, `not_covered`, or non-quantity. G5c reads that closed list, and any unlisted kind defaults to `not_covered`.
  - Map `reaction_resultant` to force and `open_formula_stress_summary` to stress, with its factor from R3B-1.
  - Have the retirement gate report, per case, how many rows are withheld under the successor against the retiring identity.

**R3B-3 (SHOULD-FIX): the section terms are not pinned for three-language bit-exact recomputation.**
- D1:380 uses the section "as the product forms it", and leaves L unstated. G5b promises bit-for-bit recomputation in Rust, Python and TS (D2:469).
- The product forms these terms in different ways:
  - on the ordinary route, `PI * (od.powi(2) - id.powi(2)) / 4.0` and `PI * (od.powi(4) - id.powi(4)) / 64.0` (`PP:7418-7427`);
  - on the exact-pressure route, an exact annulus (`PP:4478-4482`);
  - E and G can be temperature-interpolated (`PP:7101-7175`).
- `powi` is not correctly rounded, and neither Python nor TS has a guaranteed bit-identical equivalent. Section evidence is published only on the exact route (`PP:7521-7525`).
- The failure is closed: a false `RETAINED_PRECISION_SCALE_MISMATCH` makes the result `unsupported`. But it would break parity on ordinary models.
- **Required change.** Either:
  - the receipt carries the per-member terms (A, Z, L/GJ, L/EA, or their inputs) as bit strings, covered by the source identity digest. G5b takes them from there and cross-checks them against published section evidence where it exists. That is the physics-source-1 trust level D2 already states for G8. Or:
  - pin every formula as explicit `fl` steps with no `powi`.
- **Optional.** Twist and extension are harness-only (D1:343), so their per-member S\* could be left out of the receipt (D1:753), which removes two recomputations from the readers.

**R3B-4 (SHOULD-FIX): D2's G5b and G5c need three edits to mirror D1 exactly.**
- (a) Add the per-member stress kind to G5b.
- (b) State that input-derived rows (D1:342, D1:374) have no t and are `relative_verified`.
- (c) G5c should require the receipt's `not_covered` set to equal the recomputed set, as it already does for `absolute_verified`.
  - Today a receipt that omits a `not_covered` id passes. Readers still withhold that row, because they use their own G5c class.
  - But `derive_document`'s `row_disclosures`, and any downstream canonical consumer that reads the receipt's list, would miss it.

  These are the D2 side of I-8. D2 wrote G5b and G5c before D1 revision 3 existed.

**R3B-5 (SHOULD-FIX): S11 rule 8 is keyed by function, but ROOT's R3-3 is per accumulation.**
- ROOT's condition: "every floating-point compound assignment or sum/fold in the named modules must appear in it". Rule 8 (S11:279) only requires each function that contains a match to appear in the constant.
- Probe G, on `PP` outside tests: 31 functions hold 59 matching lines, and 16 of them hold more than one.
  - `solve_load_case` (1,208 lines, 3 matches) and `run_linear_static_preview_captured` (428 lines, 4 matches) will be listed, so a new published sum inside either would pass.
  - E15's own function `pressure_for_pipe` is small (25 lines, one match), so E15 itself would be caught. The gap is a future sum inside a listed function.
- **Required change.** Key the constant by function plus its count of matching lines, or by function plus accumulator binding, so that each accumulation is its own entry.

**NOTEs**
- **N-1.** D1:27 and D1:685, "the 295 twist and extension flags of V2's variant A are cleared": in fact 293 of the 295 outside RF-WEAK clear, and the two `tw.M1` rows stay. Of all 314 variant-A twist and extension flags, 303 clear. The `tw.M1` rows are new relative to variant B, not to variant A.
- **N-2.** D1:353, "`fl(2^-34·S*)` is exact whenever S\* is normal": this holds for S\* ≥ 2^-988. Below that the product is subnormal and can round (probe R: every one of 3,247 inexact samples out of 200,000 lies below 2^-988; none above). It is harmless, because fl is deterministic and readers reproduce it, and S\* would be below 1e-297. The sentence should still be corrected.
- **N-3.** The harness forms k_t = fl(fl(G·J)/L) and twist = fl(T/k_t) (D1:654), while the scale is fl(mo·fl(L/fl(G·J))) (D1:380). The operation orders differ by a few ulps, well inside the margin. The correspondence check (D1:680) should compare classes, not bits.
- **N-4.** The R3-1 condition asks for "the size of each change". Put the sizes in S11-K's PR record as well as in the report to ROOT.
- **N-5.** D1:358 names one refusal code for both classes; D2 §4.9.9 uses two (`RULE_QUANTITY_BELOW_VERIFIED_FLOOR` and `RULE_QUANTITY_NOT_COVERED`). Align D1 to D2's two codes.

**D-5 (pending).**
- P1's `RF-SKEW-T-CANT-OFF-122-r1e-04` is published Passed and Current-eligible on main. Its RX rotations are at 2.43 times the criterion (dense) and 1.21 times (sparse), about 2.4e-9 relative, at ordinary magnitudes.
- D1 revision 3's W1 trigger runs only for cases that are not Passed, so it misses this case. The S8 floor does not help either: the breach is on the ordinary binary64 route, and W1 is never attempted for a Passed case.
- ROOT has ruled D-5 a required decision; that ruling is recorded at `d84e66bff`, after the revision I reviewed.
- I will check `DESIGN_NUMERICS/D5_TRIGGER.md` narrowly when the manager sends it. This verdict does not cover D-5.

## 7. Not checked

- No build, run or fixture diff. The worst cases in R3B-1 are bounds derived from the stop-rule guarantee, not observed errors.
- I did not enumerate every published row kind of the three base identities (R3B-2 names the two found by reading the recovered-row code).
- D1 sections not touched by the BACKCHECK_R2 items or the S8-R interface were not re-read.

## 8. Run records (`T3/REVIEW/_run_records/r3_backcheck/`)

- `probe_r3.py.txt` → `probe_r3.stdout.json`. Run it with the T3 folder as its argument.
  - check R: R's bits, exactness and margin;
  - check S: stress-floor propagation;
  - check F: the not-covered lists, row by row, from `floor_kinds.json`, `v2_floor.json` and `references.json`.
- `probe_rule8_granularity.py.txt` → `probe_rule8_granularity.stdout.json` (check G). Run it from `P/` with argument `.`.
- Python 3.11.15, `nice 19`, `PYTHONDONTWRITEBYTECODE=1`. Hashes are in `T3/REVIEW/_run_records/SHA256SUMS`.
