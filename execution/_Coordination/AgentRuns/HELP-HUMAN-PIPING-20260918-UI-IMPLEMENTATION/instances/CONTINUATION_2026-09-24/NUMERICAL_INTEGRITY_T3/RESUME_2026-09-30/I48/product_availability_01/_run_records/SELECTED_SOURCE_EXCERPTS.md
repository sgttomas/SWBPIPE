
## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md:2175-2195 @ 615fdf5fae7846f6652652d917034862d4a53abc

2175: ## V-K: rulings on I17's A1 stop (THIN-A and THIN-B) (ROOT, 2026-09-29)
2176: 
2177: - **The stop.** RF-RANGE-THIN-A and THIN-B (R1's PHYS-R4 geometry: OD 4e-77 m, L = 1 m) end `Unresolved(Ceiling)` through W1a.
2178:   - The attempts: 128 is rejected by the stop rule, 256 by the stop rule, 512 by the charge test (d), and 1024 verifies with nothing above it.
2179:   - EA/(12EI/L³) ≈ 6.7e152, about 2^507, so no candidate at 512 bits or below can reach the stop rule's 2^-64 accuracy.
2180:   - K4 publishes nothing: the result is honest, but unavailable. V-K's adapter is not the cause, as I17's probe with K4's y_reference shows. Every other of V-K's 201 CI cases passes as projected.
2181: - **Ruling, subject to one confirmation. I17 first runs K4's generator (`K4T/gen_k4_vectors.py`, K4's bit oracle for the design) on THIN-A and THIN-B.** GEN must give the same attempt chain and outcome. If it does, this is the design's limit, not a K4 defect. If it does not, it is a K4 finding and a stop.
2182: - **Given that confirmation, THIN-A and THIN-B are W1a's coverage limit,** and VP-ROBUST's kernel-lane gate (§4.10) is amended as follows:
2183:   - V-K commits an **expected-unresolved list**, THIN-A and THIN-B with their reason, pinned like the `not_covered` list.
2184:   - Those cases must end honestly unresolved, with no rows published.
2185:   - Their comparisons are never counted as passes. They are reported as a separate count.
2186:   - A case that leaves or joins the list blocks the gate until ROOT reviews it.
2187:   - Every other case keeps §4.10's rule: an unsolved case fails.
2188: - **Why this is acceptable:**
2189:   - It is not a product regression. On the ordinary route, W2's scaling solves the PHYS-R4 geometry (K2b's rulings: "PHYS-R4 (b = 536) … solved and accurate").
2190:   - W1 is selected only for Sensitive and D-5-routed cases.
2191:   - W1's ladder tops out at a 512-bit candidate by design (R7). An intrinsic stiffness spread near 2^507 is beyond it, and W1a withholds honestly.
2192: - **Routed:**
2193:   - **To F2a and V-P:** THIN's product standing on the ordinary route, and what F2a publishes if such a case is routed to W1 and W1 ends unresolved.
2194:   - **To the owner's PHYS-R4 decision** (F1b's rulings): this fact is added to it.
2195:   - **To D1 and the T3-close list:** whether W1 should reach beyond 512 (a 1024 candidate with a 2048 verification) or treat decoupled stiffness spreads separately. It is not needed now.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md:4968-4985 @ 615fdf5fae7846f6652652d917034862d4a53abc

4968: ## F2a/S-G1 checkpoint0 planning released (ROOT, 2026-10-02 UTC)
4969: 
4970: Release I30_F2A_CHECKPOINT0_01 and I29_F2A_MEMORY_PLAN_02 as independent, disjoint
4971: source-only planning blocks on merged main49034a940f with selected W1 work policy.
4972: I30 owns facade/publication/readers and identity/write-set/test proposals; I29 owns
4973: the complete qualified caller/memory-admission strategy and RV40-N1 disposition.
4974: Neither grant authorizes code, experiments, runtime, a new public contract, byte
4975: guard, deployment choice or typed-input reserialization. Their brief time boxes
4976: and return-only scopes govern. No new host tooling is commissioned.
4977: 
4978: The selected facade order remains F2a atomic with S-G1, then S-I, then F2b by domain
4979: and F3; older D2 grouping language does not silently add S-I implementation here.
4980: M03-INTEGRITY-MP-v2, its private/public distinction and final-row/unit/derived
4981: qualification remain required. Exact-block coexistence and ordinary fallback
4982: standing are preserved. Genuine published-contract/product-semantics changes must
4983: be presented concretely to their owner after independent design review; missing
4984: implementation of an already accepted rule is not automatically a new permission
4985: question. ROOT reconciles the two plans before any implementation grant.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md:5183-5200 @ 615fdf5fae7846f6652652d917034862d4a53abc

5183: ## F2a source bridge conditionally selected; finite arithmetic and recipe completion (ROOT, 2026-10-02 UTC)
5184: 
5185: ROOT read RV45 REVIEW in full, verified all forty-four payloads and the exact
5186: seal scope, and preserved it atd05bb825ff. Seal:
5187: 41db4960bd930d35793fe4d17e621a353e9cd9c36a4aa5b2790c18dca8b27825.
5188: The independent source derivation preceded author exposure. Accept I33's sufficient
5189: source-action bridge at9f1ef2693d as the conditional mathematical basis for further
5190: F2a derivation: exact source/map/frame/load/constraint premises, R7's matching
5191: verification scaling and twice its upward inverse bound, data/zero-block scope,
5192: strict perturbation test and full action-functional change are all mandatory.
5193: Actual output bits, normalized coordinates, classes, scales and accuracy predicates
5194: remain unchanged. Refusal is certificate insufficiency, not a singularity claim.
5195: 
5196: This closes conditional mathematical feasibility, not implementing association,
5197: finite arithmetic/storage, actual availability or F2a/public-source reliance.
5198: No second solve, new operator, domain or public bound is selected. Missing source
5199: warrants and impossible unchanged-output predicates remain explicit. Ordinary
5200: preview truth is not silently replaced by either the exact profile or q_K.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md:5894-5925 @ 615fdf5fae7846f6652652d917034862d4a53abc

5894: ## Captured product refusal diagnosed; RV61-C1 closed (ROOT,2026-10-03 UTC)
5895: 
5896: ROOT read I46's complete diagnosis and correction, RV61's full independent review
5897: and backcheck, and verified their sealed inventories. The main review is preserved
5898: at1c16016a5239, correctione3e04c6344d2, backcheck62dcd9e1cb25. The correction
5899: inventory is dc9695c4d0cf3037483c5e14a7fa9a1b83b96157586c7d538e8dd02923dd5e9f;
5900: the backcheck inventory is5ea2949146415f7aba99e181a4ae255b4c5fd50d267245b477b1c1e153bde60e.
5901: Close RV61-C1. Original I46 supporting G5a arithmetic remains historically
5902: inaccurate; read it with the separate correction. Original sealed bytes, all
5903: other analysis results and the complete Rx witness remain unchanged, as verified
5904: by the same reviewer. No numerical source repair is claimed by this records fix.
5905: 
5906: Accept the bounded diagnosis in I46 original RETURN §§Ordinary versus retained
5907: point results, Decisive exact Rx counterexample, and Nearby dual-cover
5908: incompatibility, with RV61 REVIEW's precise fixed-scale/only-Rx-varies limits.
5909: The actual retained Rx is correctly rounded for the admitted K law but fails
5910: the geometric-source sharper predicate. Ordinary recovery misses and the
5911: section-source discrepancy are distinct. Do not call this a false native q_K
5912: publication or an executed false W1 product publication. Do not claim an all-pass
5913: source transfer for a future projection preserving these captured native values.
5914: Additional solver precision alone cannot discharge this particular discrepancy.
5915: 
5916: The existing selected I33/I35 conditional route permits a truthful finite refusal;
5917: that permission is not an availability exemption. No new owner decision is needed
5918: merely to finish/review the already authorized private refusal witness. RV60 still
5919: owns independent review of I45's actual source/rows/custody/accounting/complete
5920: verdict. A later product projection must be actually executed and checked across
5921: its final complete row universe, with unchanged source/predicates and truthful
5922: refusal. Any proposed change to protected truth, tolerance, output/operator
5923: contract or required availability remains an owner decision on concrete reviewed
5924: evidence; affected acceptance/merge is held at that boundary. This ruling grants
5925: no public activation, protected availability, resource qualification or T3 closure.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/DESIGN.md:665-705 @ 615fdf5fae7846f6652652d917034862d4a53abc

665: ### 4.4 Identities and what happens to existing ones
666: 
667: Proposals only; ROOT reserves names and versions.
668: 
669: | Identity | After D1 |
670: |---|---|
671: | `preview-physics-1` | Byte-unchanged for every envelope in which no case selects the new method |
672: | `<preview-retained>`, suggested `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Ordinary route, emitted when at least one case selects the method. It inherits the preview-physics-1 table and adds a closed per-case receipt (§5). Other cases are rendered under preview-physics-1 semantics, with their ordinary standing |
673: | `<physics-retained>`, suggested `.../physics-retained-1` | The exact-route equivalent, inheriting physics-1 semantics |
674: | `source-blocks-1`, `physics-source-1` | Readers, fixtures and hashes unchanged. **Recommended (D-4 option A), ordered per D-15 (revision 5):** exact-block selection stays the selected method for its domain until **F2b** retires it there, and F2b runs for a family only once the retirement gate (§4.4.1, row-level condition 3) passes, which on the committed families requires C (§8.1). Until then the coexistence rule below applies. After retirement the exact-block method stays a kernel test oracle: in its scope, the new method's rows must match its projections within the unchanged criterion |
675: | `load-reference-source-1` (T1) | **As ruled (R-3(a)).** It stays fresh until F3 (W1b, including the 0.4.0 load states) lands, and it stops being fresh at F3, when the gate passes for it. Joined results stay `needs_recompute` until then. D2's S-E1 is built with F3, and S-E2 only if F3 will not land within T3. The 0.4.0 successor is `<load-reference-retained>` (D2 §4.9.1, S-G2) |
676: | Historical physics-source-1 | **As ruled (R-3(b)):** stays eligible after fresh retirement, through its existing reader. A defect found later reopens this |
677: | Historical all-selected source-blocks-1 | **As ruled (R-7 (i)):** stays Current, with the notice and the summary rule-binding refusal. Retirement for fresh solves does not change it |
678: 
679: **Coexistence rule (revision 5, R4-2; ROOT).** While exact-block selection is retained for a domain, **W1 is not attempted in any invocation in which exact-block selects a case.** Such an invocation publishes exactly as today: the same identity, `numerical_quality`, diagnostics and bytes. The case is decided per invocation after the ordinary attempts and exact-block's own selection, which are unchanged. An invocation in which exact-block selects no case may attempt W1 for its eligible cases (§4.3), and publishes under the successor identity if one is selected. So one envelope never carries both `SOURCE_BLOCK_RECOVERY_SELECTED` and `RETAINED_PRECISION_SELECTED`, which keeps D2's G4. A case exact-block leaves unrecovered in an exact-block-selected invocation keeps today's outcome; W1's recovery of it (F-P2's half) waits for that domain's F2b.
680: 
681: Under option A, after retirement, a fresh invocation never mixes two selected methods. The ordinary cases in an extended envelope carry repaired preview-physics-1 semantics, not precision-1 semantics. That removes, for fresh solves, the reason behind T0R's `SOURCE_BLOCKS_ORDINARY_CASE_LEGACY_SEMANTICS`. D2 owns that reader decision.
682: 
683: **Option B.** Keep exact-block first and the new method for the rest. It needs a mixed-method identity and two receipt families in one envelope. I do not recommend it.
684: 
685: #### 4.4.1 The shared retirement gate (revision 2, V1-S2; adopted by ROOT, defined here, cited by D2)
686: 
687: **Where it applies.** An identity family's exact-block selection is retired for fresh solves only when all four conditions hold on the actual candidate. The families and their slices:
688: - source-blocks-1 at F2b, with D2's S-F;
689: - physics-source-1 at F2b;
690: - load-reference-source-1 at F3's retirement step.
691: 
692: (Revision 5: source-blocks-1 also retires at F2b, not F2a. F2a wires W1 and the identities and retires nothing.)
693: 
694: **The conditions.**
695: 1. **Coverage.** Take every committed request under `P/fixtures/product_preview/source_blocks/` and `P/fixtures/product_preview/physics_source/`. For F3, also take every committed joined `load-reference-source-1` request (T1's `load_reference_states` carriers and fixtures). Each is solved fresh in both modes, and every case that exact-block selects today must be selected by the new method. A case outside W1's coverage fails the gate, and the family is not retired. (D2's H-a subset concerns standing, not selection; it is handled under condition 3.)
696: 2. **Budgets.** Those cases complete within the D-8 limits ROOT selects from measurement. The per-case and per-invocation charges are recorded.
697: 3. **Standing (revision 3, ROOT's final wording for S2-R; revision 5, row-level per ROOT's D-15 and R4-2).** "The successor identity's standing is no worse than the retiring identity's, case by case, in all three languages." **Row level:** for every case, the successor's withheld rows (those D2's `classification_summary` counts as withheld: `not_covered`, and `absolute_verified` rows that neither C nor a verified B makes bindable) number no more than the retiring identity's withheld rows, which are zero where it publishes the case as Current. The gate report shows **both counts side by side per case**, per language. A successor count above the retiring count fails the condition for the family. **Check level (revision 5a, R5-5; ROOT).** The row count alone can pass while a check changes outcome: a C-bound zero row that B cannot prove binds as [−b, b], so a sign or equality check on it reads `RULE_RESULT_INDETERMINATE` where exact-block, publishing +0 as a point, decides it today. So condition 3 also runs the committed rule packs, and the run fixtures that bind successor rows, under both identities, and reports every check that moves from **decided** to **undecided**, in D2's wording, which ROOT accepted: decided means `USER_RULE_CHECKED` or `USER_RULE_FAILED`; undecided includes `RULE_RESULT_INDETERMINATE` and the withheld-row refusals. Rust computes the comparison, Python recomputes it, and TS gives the same binding decision. **Any such move stops that domain's retirement.** On the committed families the candidates are the 60 unproven zeros of mixed ordinary-pressure and the 7 cancellation zeros of eigen_motion (§8.1.1), and more if B is not built. In addition, each language's standing is identical and fail-closed across the whole family, through D2's S-G readers and the shared case files (D2 §4.7). The condition does not require every case to become eligible. A declared out-of-scope subset reads `needs_recompute` under both identities, and passes.
698:    - **Listed out-of-scope subset: D2's H-a log-law cases** (D2 §4.2.1, §4.9.5). A 0.4.0 case in which any member uses a definition whose value needs host `exp` or `exp_m1` (today only the `logarithmic_per_current_length` law) is `needs_recompute` under the joined identity `load-reference-source-1` and under its successor `<load-reference-retained>`, in all three languages. The gate records each such case by request and case id.
699:    - For source-blocks-1, an all-selected envelope is Current today, so the successor result for the same request must be Current too; a mixed envelope is `needs_recompute` today, so any fail-closed successor standing passes (D2 §4.5.2).
700:    - **The joined family switches at F3**, as R-3(a) ruled.
701: 4. **Values.** Every published quantity that the exact-block projection also publishes agrees with it within the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`. The scale is the case's R1-style zero scale, or, for these fixtures, the body-level coupled scale stated in the test. Signed six-component reactions and circular maxima, which exact-block's source-blocks-1 rows do not carry, are checked against T0R's preview-physics-1 rules instead.
702: 
703: **Withheld rows (revision 4, R3B-2; revision 5: a pass condition, not information).** The side-by-side per-case counts are condition 3's row-level test. §8.1 gives the committed figures before and after C and B. On the committed families every domain has at least one case that fails the condition without C, so F2b follows D2's C slices (§6).
704: 
705: **Evidence.** The gate runs as one committed test per family in `numerical_robustness` (product lane), plus D2's S-G parity files. Its record lists every request, mode, case, method, charge and standing. A gate failure blocks retirement for that family only; the other families and W1's selection are unaffected.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/DESIGN.md:851-906 @ 615fdf5fae7846f6652652d917034862d4a53abc

851: 
852: **Crate** (new): `P/validation/benchmarks/numerical_robustness/`, with its own `Cargo.lock`, discovered by CI automatically.
853: - Dependencies: `product_physics`, `frame_kernel`, `sparse_direct`, and `serde_json` with `float_roundtrip`, the same set `numerical_integrity` uses.
854: - `cases/` holds adapters from R1's `references.json` to kernel `PrimitiveSource` values and to product requests. The adapter is reviewed code; R1's values are never edited.
855: - Examples, not tests, for the scale runs.
856: 
857: **Kernel lane (before T1 merges).**
858: - R1 families RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE, RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE and RF-MECH, through the kernel method, and through the binary64 sparse gate for the RF-MECH and RF-LARGE parity checks.
859: - **Limit.** Springs can lie along any direction at kernel level. The product supports only global-axis springs and restraints (`linear_supports/src/lib.rs:217-221`), so R1 cases with springs along a skewed axis run in the kernel lane only.
860: 
861: **Product lane (after T1 merges and the facade slices land).**
862: - Every authorable R1 case, through `run_linear_static_preview_value_with_mode` in both modes. Each variant runs as its own request.
863: 
864: **What is compared.**
865: - Global nodal displacements and rotations.
866: - The six signed reaction components per support (T0R v2 rows).
867: - Member invariants: axial force, torque and `hypot(My, Mz)` at the ends and stations.
868: - **Twist and extension (revision 3, F2), derived, never differenced.** The harness derives each member's twist as `T / k_t` and extension as `N / k_a` from the published torque T and axial force N, with `k_t = fl(fl(G·J)/L)` and `k_a = fl(fl(E·A)/L)` formed as `FK/lib.rs` forms the torsion and axial coefficients. They therefore inherit T's and N's verified accuracy, to within two further roundings (≤ 2.3e-16 relative). They are never computed as differences of published rotations or translations (θ_j − θ_i, u_j − u_i), whose binary64 cancellation would limit their relative accuracy regardless of the stop rule.
869: - Method evidence: the selected method, the precisions, the M03 class.
870: - RF-MECH must be refused with a witness or an unresolved status, and no rows. Any recovered answer fails.
871: 
872: **Predicate.** Every comparison uses `|obs − exp| ≤ 1e-9·max(|exp|, scale)` with R1's stated zero scales. No new tolerance.
873: 
874: **R1's package (candidate at `6c448d260`, not yet frozen; revision 2).**
875: - **Represented basis.** Two cases are compared on R1's `expected_represented` values, not on the intended ones: `RF-SKEW-A-CANT-AX-122-r1e-12` and `RF-FINITE-THIRTIETHS-O1e6`. Their intended-input comparison fails through input rounding alone, which no solver can recover. So does every case whose `finite_input` field marks the represented basis.
876: - **Discriminating controls.** Mutation and stop-rule controls use only the negative controls R1 marks `discriminates`. For the soft families that means k/a from about 1e-8 down; the k/a ≈ 1e-4 cases are continuity controls.
877: - **Directional springs.** Cases flagged `needs_directional_spring` run in the kernel lane only. The product authors global-axis springs only (`linear_supports/src/lib.rs:217-221`).
878: - **RF-CANCEL scale.** RF-CANCEL is compared with R1's recommended net-governed scale, which ROOT ruled is **the binding comparison scale** (`ROOT_RULINGS_V2.md` §1). The criterion is unchanged; where the net scale falls below |exp| (84 `mixed` rows, V2 §3.1) the comparison is exactly relative. See `S11_CONTAINMENT.md` §5.2 for how it relates to the S11 row scale: under C3-full the acceptance does not depend on the row scale, which matters only on the guard path.
879: - **RF-RANGE.** LEF-small and LEF-large must be solved (§4.7). A named range refusal is recorded as a failure.
880: - **Capture-boundary cases (V1-S5)**: 2^53 − 1, 2^53 and 1e16 in a request (§4.7).
881: 
882: **The zero-scale floor check (V1-S8, F2).** For every reference comparison, the comparison scale `max(|exp|, scale)` must be at least `R·S*` of the case, with R = 2^-34 and S\* computed from the reference values by §4.1.6.1's kinds (twist and extension per member). A comparison below that is **not covered by the guarantee**.
883: 
884: **How the harness reports each comparison (revision 3, ROOT's F2 ruling).**
885: 
886: | Outcome | When | Counts as a pass |
887: |---|---|---|
888: | `pass` | Covered, and the predicate holds | yes |
889: | `fail` | Covered, and the predicate fails | no; blocks the gate |
890: | `not_covered` | The comparison scale is below R·S\*. The observed error is recorded, and so is whether the predicate held | **never**, whatever the observed error |
891: | `pass_absolute_range` / `fail` | F8: the expected value is below the binary64 range (RF-LARGE-CONT-n10000, 1e-714 to 1e-2864). It is parsed exactly from its decimal string, without underflow, and compared as an absolute comparison against its class scale | the pass is reported as an absolute-range pass, separately counted |
892: 
893: - **The gate.** VP-ROBUST passes when there is no `fail`, every discriminating negative control fails, and the `not_covered` set equals the enumerated list committed with the harness (below). A new `not_covered` comparison, or one that leaves the list, blocks the gate until ROOT reviews it. `not_covered` comparisons are never added to the pass count; the report shows passes, absolute-range passes and not-covered comparisons as three separate numbers.
894: - **"No Passed breach" (revision 4, D-5; revision 5, ROOT `b6fe1eb75` and D5C-5).** For every R1 case in the product lane, if any covered comparison fails, the case's published outcome must not be `Passed`.
895:   - **Both entries.** The gate runs every case through the captured entry (`run_linear_static_preview_value_with_mode`, `PP:1407`) and through the historical typed entry, as the headless runner reaches it (`run_preview_in_memory_mode`, `P/core/runner/headless/src/lib.rs:804`, into `run_linear_static_preview_with_mode`, `PP:1397`). A case the captured entry refuses at capture (G = 1e80, V1-S5) is not a pass there. It is checked on the typed entry.
896:   - **Named exceptions, as (entry, case, quantity) triples (revision 5a: ROOT's re-pin, `7e3cc1f31`).** Until S11-F merges, the only admitted breaches are S11's, and the committed list is **`GATE/S11_EXCEPTIONS.json`** (sha256 `8f3687f4…`), generated by `GATE/pin_s11_exceptions.py` from P1's final `DETECTION/results.json`. It covers frozen-reference cases only:
897:     - **captured entry: 88 triples in 13 cases**: F and M at G = 1e7 and 1e8 in orders GnG and nGG, F and M G1e8-GnG-ORTHO and -INPLANE, and RF-CANCEL-UDL-W1e8 `th.S1.RZ`;
898:     - **typed entry: 140 triples in 22 cases**: the same 88 (the typed entry is bit-identical to the captured entry on them), plus 52 triples from the nine G = 1e80 cases the captured entry refuses at capture.
899:     - Any triple outside the list is a gate failure, even inside an RF-CANCEL case.
900:     - P1's own probes (S11-PROBE-A-\*) and V1-CHECK-L-\* are not gate cases; the probes become S11-K/F tests with exact expected nets. RF-SKEW-T-CANT-OFF-122-r1e-04 is not an exception: it is K-D5's required true positive on both entries.
901:     - **Correction.** Revision 5's "106 / 168" was D1's prediction (`s11_exceptions.py`), wrongly quoted as P1's record. It included R1's midspan bending magnitudes `Mb.M1.mid` and `Mb.M2.mid` (46 triples), which the product does not publish. The script now drops them and reconciles exactly with the pin (228 of 228 triples).
902:   - **Removal.** A test pins the list to its committed source and requires it to be **empty once S11-F merges**. The exceptions are removed only when **both** entries are clean (ROOT).
903:   - **Negative controls:** mutation (23) with the D-5 trigger disabled, and a seeded non-S11 breach inside an RF-CANCEL case (for example, a formation-class perturbation), which must fail the gate although the case is named.
904: - **Relation to product standing.** A `not_covered` comparison's quantity is, in the product, `absolute_verified` or `not_covered` by the receipt's classification (§4.1.6 item 1), because its reference magnitude is below the floor. So it is withheld from reliance there too (§4.1.6 item 4). The harness checks this correspondence for every product-lane case, **comparing classes, not bits** (revision 4, N-3). Since revision 5 the harness forms both the derived twist `fl(T/k_t)` and its scale `fl(mo/k_t)` from the same receipt k_t, so the remaining differences are the product's own roundings, far inside the margin.
905: - **The enumerated list, from R1 revision 2 at `c0f14201c`** (`_run_records/floor_kinds.*`, variant F, which is this rule; identical at R = 10^9·2^-64 and at R = 2^-34):
906:   - **RF-WEAK: 46**: W-AX-rho1e-12 (29 far-region and 7 coupling-region comparisons), W-3D-rho1e-12 (2 far, 5 coupling and 2 body-class) and W-3D-rho1e-08 (1 coupling). These are V2's 43, plus the soft coupling member's twist and extension (`tw.C` and `ext.C` in W-AX-rho1e-12, `tw.C` in W-3D-rho1e-08), whose own torque and axial force sit below the floor;

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_NUMERICS/DESIGN.md:1023-1031 @ 615fdf5fae7846f6652652d917034862d4a53abc

1023: | V-K: VP-ROBUST kernel lane | after R1's references are frozen and K4 | new `P/validation/benchmarks/numerical_robustness/**` | §4.10 kernel lane, the zero-scale floor check |
1024: | **S11-F**: the S11 facade half | **the first facade slice after T1 merges** | `PP` (the ledger at every producer of `S11_CONTAINMENT.md` §4.2, with pushed terms, including T1's eigen sites; `AssembledForce` seams; recovery sums E5, E7–E12; the Sensitive mapping and `LOAD_CONTRIBUTION_ABSORBED`; the enumerated site test); `P/core/product_physics/src/pressure_runtime.rs` (push group operands); T1's `source_recovery.rs:609-667` including `:1270-1274`, `source_receipt.rs:218-219` and `:320` | `S11_CONTAINMENT.md` §9 S11-F tests 1–8, including the invariant test and V1's 0.4.0 test; **through both entries** (the captured route, and the historical typed entry via headless `run_preview_in_memory_mode`), with RF-CANCEL at G = 1e80 (F, M, ORTHO, INPLANE, UDL-W1e80) on the typed entry and on the captured route once S-H is present (ROOT `b6fe1eb75`); **S-H never lands before S11-F** (same PR, or S-H's PR after S11-F is on main, re-running the 1e80 cases through the captured route); the nonlinear loop stays on the legacy KS variant when `PP` moves to the typed path; the named exceptions list emptied; mutations M2–M5, M8, M9; the committed-fixture diff with disclosure (S11 §8.3) |
1025: | F1: facade sparse wiring, W2 at formation, SUP-17 | after S11-F | `PP` (assembly `:1620`, `:1751`, now the kernel's sparse assembly with K2b's formation-time scaling; reduction `:2330-2342`; reactions `:2697`; `solve_preview_reduced_system`); `source_recovery.rs` (refuse scaled evidence); the nonlinear loop in `nonlinear_integration/src/lib.rs` (with T5) | Full product suites, the PHYS-R4 public fixture, LEF-small and LEF-large solved, the parity protocol, the dense-scrutiny guard |
1026: | **F2a**: facade W1a wiring and identities, **no retirement** (revision 5, R4-2) | after F1 and ROOT's identity reservation; **atomic with D2's S-G1** | `PP` (case loop, receipt, rows, exact combinations, the coexistence rule of §4.4), new `P/core/product_physics/src/retained_publication.rs`, tables in `P/fixtures/results/`, schemas | Product lane of VP-ROBUST; W1 selected on Sensitive and D-5-routed cases outside exact-block selection; **coexistence:** every committed exact-block-selected request publishes byte-identically, and no envelope carries both selected diagnostics; the producer's published-row S\* and classification (including `input_derived_dofs`), checked against D2's G5b and G5c; the D-5 routing (the demotion becomes routing to W1 where the invocation has no exact-block selection) |
1027: | **S-I1, S-I2** (D2): option C, interval binding (DD-13, per ROOT's conservative-binding constraint and R4-3) | S-I1 (evaluator, runner, Python reference; rules crates only) any time; S-I2 (binding wiring) with or right after F2a/S-G1; **both before F2b and F3's retirement** | D2's write sets (§8.1.3 gives D1's side and the cost) | D2's plan: three-valued pass/fail/indeterminate, outward endpoints, covered-row outcomes unchanged, parity in three languages |
1028: | **F2b**: retirement of exact-block selection, **domain by domain** | after S-I2; per family, only when the §4.4.1 gate passes on the actual candidate, including row-level condition 3 | `PP` (selection order for the retired family); the characterization tests for retired exact-block selection, replaced and recorded | **The retirement gate (§4.4.1) for source-blocks-1 and physics-source-1**, with the side-by-side withheld counts; N05 and N06 through the public entry; **D2's corrected probe PR-2b (F-P2)**: in P12, one case is selected and another case's ordinary attempt is rejected (for example N06-class `ASSEMBLY_UNRESOLVED`), and W1 must recover the rejected case; a family whose gate fails stays on exact-block, unaffected by the others |
1029: | F3: W1b families and the 0.4.0 successor | after F2a and the RF-ELOAD addendum, its retirement step after S-I2; **atomic with D2's S-G2 and S-E1** | `PP` load builders to `PrimitiveSource` and the ledger; kernel element-load primitives; **the equivalent-static extension (D-14, revision 5a): `GeneratedUniform` sources formed at p from the user's inputs (§4.2)** | RF-ELOAD, including its generated-load cases and the "binary64 product as input" negative control; 0.4.0 prescribed and eigen cases; **the gate for `load-reference-source-1`** (row-level; eigen_motion needs C), which then stops being fresh (R-3(a)) |
1030: | R: readers | per D2 | D2's write sets (S-G and the rest) | D2's plan |
1031: | V-P: product lane and scale runs | after F1 and F2a | `numerical_robustness/**` | §4.10, §4.8 |

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_STANDING/DESIGN.md:523-545 @ 615fdf5fae7846f6652652d917034862d4a53abc

523: - Each case entry is one of:
524:   - **`selected`:** D1 §5.1's closed receipt (method token, policy, source identity digest, attempts list, selected p and verification p, the stop-rule summary per body and kind, pivot margin minimum, rcond at p, retained-residual summary, retained-state digest, and the ordinary-attempt reference);
525:   - **`unavailable`:** the attempt failed. It carries a reason, the attempts list and a reference to its `RETAINED_PRECISION_UNAVAILABLE` diagnostic;
526:   - **`not_required`:** the ordinary attempt passed.
527: - **Mixed envelopes are normal** (D1 §5.3). A successor identity is emitted only when at least one case is `selected`.
528: - **`numerical_quality.cases[i]` keeps the ordinary attempt's M03-INTEGRITY-v1 outcome** for every case, selected ones included. The precision-p outcome lives only in the receipt. Readers never derive a selected case's standing from `numerical_quality`. D1 adopted this as IF-1 (D1 revision 2 §4.5 and §5 item 3, confirmed by V1's backcheck), so DD-11 is closed. G5 still refuses a selected case whose `numerical_quality` claims `checks_passed` without a matching ordinary attempt.
529: - Every row of a selected case carries `recovery_method = contribution_preserving_multiprecision_v1` in its evidence (D1 §5.2).
530: 
531: #### 4.9.3 Reader checks (identical order and codes in Rust, Python and TS)
532: 
533: | Step | Check |
534: |---|---|
535: | G0 | Identity, profile and table sha256 |
536: | G1 | Receipt shape against the closed schema; `receipt_sha256` over the body, and `publication_sha256` over the envelope minus the receipt, **with the profile named in the body**. Readers support exactly the profiles ROOT registers for the policy: the checked profile needs nothing new; the scientific profile needs a TS canonicalizer that does not exist today (`hashService.ts` has only the checked one). An unknown profile gives `unsupported` |
537: | G2 | Encoding. Every receipt number that can exceed 2^53 − 1, be subnormal or be negative zero is a 16-hex bit string decoding to a finite binary64. Plain JSON numbers in the receipt are only exact integers (counts, precisions, work) within the profile's integer range |
538: | G3 | Case coverage and order against the request (when an invocation is supplied) and `numerical_quality`. Case ids unique |
539: | G4 | Diagnostics. Exactly one `RETAINED_PRECISION_SELECTED` per selected case and one `RETAINED_PRECISION_UNAVAILABLE` per unavailable case, each with `affected_refs == [case id]`. No `RETAINED_PRECISION_UNAVAILABLE` or `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` names a selected case (the S13 rule from the start). No `SOURCE_BLOCK_RECOVERY_SELECTED` anywhere: a successor envelope never mixes methods (D1 option A; ROOT's coexistence rule, revision 5: no W1 attempt in an invocation where exact-block selects any case, so such invocations publish exactly as today under their source identity, and source-identity readers already refuse a `retained_precision` member through their closed field lists) |
540: | G5 (G5a) | Per selected case: p ∈ {128, 256, 512}; verification p = 2p ≤ 1024; the attempts list ends with the accepted attempt, and every earlier attempt has a rejection reason; work within the registered limits (D-8); every stop-rule entry decodes to a value ≤ 2^-64 (exact binary64 comparison with the constant); pivot margin and rcond decode to finite positive values; the retained-state and load-ledger digests are 64 lowercase hex; `input_derived_dofs` is a list of unique (node id, component) pairs with components in {UX, UY, UZ, RX, RY, RZ}; the optional `structural_zero` member (S-J) has the shape of §4.12; the ordinary-attempt reference binds to `numerical_quality.cases[i]` and its diagnostic, as source-blocks `ordinary()` does. `RETAINED_PRECISION_UNAVAILABLE` reasons admitted in G4 include `receipt_encoding`, `publication_hash_range` and `invocation_not_representable` (D1 §5 item 2; S5-R) |
541: | G5b (revisions 3–4) | S\* recomputation from published rows and receipt section terms, bit for bit; `RETAINED_PRECISION_SCALE_MISMATCH` or `RETAINED_PRECISION_SECTION_MISMATCH` on mismatch. Detail below the table |
542: | G5c (revisions 3–4) | Classification recomputation against the closed row-kind list, with exact set equality for both the `absolute_verified` and the `not_covered` lists; `RETAINED_PRECISION_CLASSIFICATION_MISMATCH` on mismatch. Detail below the table |
543: | G6 | Rows. Every row of a selected case carries the method token. No row of an unselected case carries it |
544: | G7 | The base-identity validator on a projection. It removes only the receipt, the method-token evidence and the identity, profile and policy constants that G0–G6 bound. It then runs the unchanged preview-physics-1, physics-1 or load-reference-1 evidence validator (T1's projection pattern, `load_reference_source.rs:184-220`) |
545: | G8 | With an invocation: invocation shape and hash (with the named profile); model project id; case coverage; requested mode. Model-derived operands: `<physics-retained>` uses physics-source-1's `actual_materials` through S-C's parameter, over the physics-1 evidence; `<load-reference-retained>` uses S-E1 (J4.2–J4.8; `Scope` → `needs_recompute`); `<preview-retained>` compares the materials and sections its preview-physics-1 evidence publishes, and what it does not publish, the receipt's source identity digest covers (the physics-source-1 trust level, stated as a limit) |

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_STANDING/DESIGN.md:595-608 @ 615fdf5fae7846f6652652d917034862d4a53abc

595: #### 4.9.4 The standing basis
596: 
597: - **Chosen: a verified receipt plus invocation binding, without replay.** Python and TS cannot run the kernel (D1 §5.5), and physics-source-1 is Current today on this basis. Rust-only replay as a standing input would break parity or force delegation.
598: - **Replay** of every committed successor raw and every VP-ROBUST product-lane output, comparing retained-state digests, is a **Rust validation-lane audit** in `numerical_robustness` (D1's W5). It is not a standing input. A replay mismatch is a producer defect that reopens standing (as ROOT's R-3(b) wording provides).
599: 
600: | Standing | Condition |
601: |---|---|
602: | `numerically_eligible` | G0–G8 pass with an invocation; requested refs equal the case order; every case is `selected` (G5 passed) or `not_required` with the base identity's ordinary eligibility (`checks_passed`, `passive_model_basis`, `represented_equations_retained`, evidence refs resolve; the existing rules at `semantic_contract.rs:410-452`); `MECHANICS_SOLVED`; T0R's combination gates respected by the base validator |
603: | `needs_recompute` | No invocation; any `unavailable` case; any `not_required` case not ordinarily eligible; a `Scope` result; differing requested refs |
604: | `unsupported` | Any G-check fails |
605: 
606: #### 4.9.5 The 0.4.0 successor (S-G2)
607: 
608: G8 uses S-E1. The H-a rule (DD-4) means a D1 0.4.0 successor case that uses the logarithmic law is `needs_recompute`, unless ROOT chooses H-b. The same case is `needs_recompute` under the retiring joined identity, so ROOT's gate condition 3 ("no worse than the retiring identity's, case by case, in all three languages") passes for it (S2-R ruling). D1 records the H-a subset in the gate definition as a declared out-of-scope subset.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_STANDING/DESIGN.md:625-627 @ 615fdf5fae7846f6652652d917034862d4a53abc

625: #### 4.9.8 Parity and controls
626: 
627: - **Positive controls.** A shared case file `retained_precision_cases.json` (raw, table and transport cases) consumed by the three readers. Positive controls use producer outputs from D1's F2 and F3 (N05 and N06 on both routes, mixed envelopes, both modes).

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/DESIGN_STANDING/DESIGN.md:1038-1044 @ 615fdf5fae7846f6652652d917034862d4a53abc

1038:   - S-J (DD-15) after S-I, with or after D1's B producer slice, and before F2b in any domain whose check-level comparison needs it.
1039: - **Merge gate:**
1040:   - complete-diff independent review;
1041:   - hosted CI, including the dual-viewport dispatch whenever TS changes;
1042:   - a clean DEC-025 sweep;
1043:   - native witnesses on the owner's Mac, recorded as outstanding if not available. S-H: a 1e16 N/m spring solves in the native app. S-G1: a successor result is shown Current with its invocation.
1044: - **Each slice is one atomic PR** across its languages.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I35/f2a_certificate_integration_02/NEXT_CODE_AND_WITNESS.md:53-69 @ 615fdf5fae7846f6652652d917034862d4a53abc

53: ## 3. First vertical numerical witness plan
54: 
55: Use a future specifically authorized private integration harness that calls the **actual ordinary builder and retained solve once**, then supplies their actual immutable source/final rows to the certificate. Do not synthesize arbitrary radius/value pairs. Start with this minimal intended W1a specimen:
56: 
57: - two nodes at (0,0,0) and (1,0,0) metres; one straight circular pipe, y_reference=(0,1,0), normalized OD=0.1 m, effective wall=0.005 m; actual selected base E=210e9 Pa and independent G=80e9 Pa;
58: - all six base-node DOFs restrained at zero, no member/eigen/pressure/nonlinear/directional/curved loads or elements, no component SIF row;
59: - W0: all individual free load terms exactly zero; W1: three explicit end-node terms Fx=1 N, Fy=1 N and Mx=1 N*m, with their individual source identities preserved.
60: 
61: The specimen is a planned input, not an observed admission result. The harness first proves the actual builder/profile admits it; if not, report the exact source boundary and select an already admitted protected straight cantilever without weakening eligibility. Record actual normalized bits, member/source/ledger/verification identities and final row roster. No production input/output or reference generator is edited.
62: 
63: **W0's decisive purpose:** exercise legitimate exact homogeneous data flags, missing-B handling on no-data blocks, real-pi material/section construction with exact zero actions, b=0, source/represented hull, zero nonlinear maximum and actual observable midpoint/magnitude. A fabricated absent radius or merely cancelled nonzero ledger must not enter that path. Expected successful zero certificates are conditioned on the actual producer's exact zero values/evidence and the checked uniqueness premise; failure is reported, not hidden by changing the model or output.
64: 
65: **W1's decisive purpose:** exercise real data blocks, selected P=2p scaling and 2B, strict alpha, both action-change terms, signed ends/stations, ordinary represented/source stresses, the two-end circular maximum and raw/SI predicates. The independent source oracle uses exact normalized bits and the reviewed pi bracket in the closed axis-aligned cantilever formulas: u_x=Fx L/(E A), u_y=Fy L³/(3 E I_z), rotation_x=Mx L/(G J), rotation_z=Fy L²/(2 E I_z), with source section quantities and the established action sign convention. It checks enclosure containment and actual final error/predicate outcomes without calling product arithmetic as its reference.
66: 
67: A correct **finite numeric refusal is an acceptable first integration result**, provided the exact failing predicate/source/permit reason and spent prefix agree with the independent oracle and unchanged output. W1 success is not promised: the I33 majorant can be too conservative for the fixed public predicates. This witness establishes source-to-verdict correspondence, not protected availability. Any later need to tighten the majorant is separate bounded proof work; weakening a predicate is not a repair.
68: 
69: After W0/W1, add the same geometry with an actual ordinary interpolation selection at T=303 K between named 293 K/313 K points, independent positive E/G and the alpha fields the resolver requires. Verify selected point identity, no at-point/extrapolated/base-G substitution, four exact CK products and the represented Z cover. An exact-profile counterpart uses its actual common E/nu resolver and explicitly empty pressure-region list. SIF association and surviving-open/combination contexts are subsequent source-specific witnesses; do not invent their eligibility for the initial two-node specimen.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md:128-133 @ 615fdf5fae7846f6652652d917034862d4a53abc

128:    - A refusal above the ceiling is a new refusal class for very large dense-scrutiny models. It is recorded as ROOT's provisional product decision and reported to the owner.
129: 9. **Q9: (a).** `source_recovery::Input.stiffness` keeps its type. PP builds the dense view only when n ≤ 256, and a test pins that the budget refusal comes before every read. `source_receipt.rs` is unchanged.
130: 10. **Q10: (a) for both. Scope §6 is approved as the expected outcomes;** its predictions become results at A2, where ROOT rules on the product-run table.
131:     - **PHYS-R4:** the design's "the public fixture then passes the evidence stage" is **restated** as a named refusal, provided A2's product run confirms that the fixture's exact-pressure end-cap operand is subnormal at formation. A no-pressure variant must solve.
132:     - The unmet design expectation, a scaled exact-pressure operand formation (kernel scope), goes on the T3-close list for a decision.
133:     - `k2a_formation_range_runtime.rs`'s linear variants and, only if Q6's template requires it, `pressure_membrane_range.rs` are declared edits.

## projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md:214-230 @ 615fdf5fae7846f6652652d917034862d4a53abc

214: ### 6. The LEF, reach and PHYS-R4 expectations at product level (restated; ruling C and I7 addendum 1)
215: 
216: These restate §6's F1 row, §4.10 and §7.1 ("LEF-small and LEF-large solved") at the level where they apply. **Each line is a required outcome, or a prediction to be confirmed by a product run at checkpoint A.** A prediction that the run contradicts is a stop, not a relabel. The lesson is ROOT's: a claim about product behaviour needs a product run.
217: 
218: | Case | Entry | Required outcome | Why (code on `e7d930d49`) |
219: |---|---|---|---|
220: | RF-RANGE-{CHAIN,SKEW,CONT}-LEF-small | both | **Unchanged: refused at model build,** `PIPE_ELEMENT_INPUT_INVALID` (blocking), byte-identical to Mac main. W2 never engages, because nothing is formed | `build_model_for_members` `PP:5620-5631` → `StraightPipeElement::new` (`SP:427`, calls `frame_element()` at `:441`) → `FrameElement::new` (`FK:563`, orientation at `:582`) → `from_x_axis_and_y_reference` (`:525`) → `normalize` (`:1864-1867`): \|x_j − x_i\| ≤ `AXIS_TOLERANCE` = 1e-12 m (`FK:24`) → `DegenerateAxis`. The gate records `refused_blocked` on both entries (K2a gate `final_result.json`) |
221: | LEF-large | captured | **Unchanged:** `Err`, no envelope | `CapturedInvocation::parse` (`PP:1620`) → `validate_checked_value` (`canonical_json/src/lib.rs:94-116`): an integral float above 2^53 − 1 is `CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT` (`:106-107`). The capture fix is D2's R-6 |
222: | LEF-large | typed | **W2 engaged** (not source-eligible). Required: published, with every displacement component equal bit for bit to its RF base case's published component times the exact power of two (translations 2^(pf−pm−pl), rotations 2^(pf−pm−2pl); pl = 200, pm = 300, pf = 600; K2b RETURN §9 C), in both modes. Published forces are 2^pf and moments 2^(pf+pl) times the base's, or refused by name. **Standing:** never above the base's. Under Q5(a) it is Sensitive (R-b′ fail-closed); under Q5(b) it is the base's (CONT Passed; CHAIN and SKEW Sensitive). The b is the census's; derive it at checkpoint 0 (K2b's kernel b is −702) | Today K2a refuses it at the basis assembly (`PP:1826`) → `SOLVER_SYSTEM_BLOCKED` (gate: `refused_blocked`, typed). A derived stress or magnitude that leaves binary64 (`require_finite_mechanics`, `PP:2332`) would block it again: that is a finding, reported, not relabelled |
223: | reach_zero, reach_lef (PP `tests/k2a_formation_range_runtime.rs`, linear variants) | both | **W2 engaged.** Published; every displacement component within 1e-9 of an exact reference computed in the test; standing as ruled (Q5). K2b solved both at kernel level (b = 734) | Today: `SOLVER_SYSTEM_BLOCKED` (K2a, `12EIy/L^3: (12*E)*Iy`) |
224: | spring-carried (G = 1e-300 Pa; same file, `:166`) | both | **A named refusal:** `ScaledEvaluation` with the trigger `GJ/L: G*J`, in both modes, under Q6's template. "Force scaling cannot restore it" (ruling A) | M03's contribution audit refuses at every b. The ratio is scale-free |
225: | partial underflow (same file, `:250`, linear variant) | both | Established by product run at A: published within 1e-9, with ruling B's record outcomes rendered, or a named refusal. The PP model may differ from K2b's kernel case (load 1e-307 N, b = 898). Compare them and state which | Today: `SOLVER_SYSTEM_BLOCKED` (K2a) |
226: | the same file's `open_gap` (nonlinear) variants | both | **Unchanged,** byte-identical to Mac main (Q2) | The loop forms at b = 0 (NI `lib.rs:581`) and is pinned to reach no scaled entry (K2b) |
227: | PHYS-R4 public fixture (`tests/pressure_membrane_range.rs:90-122`) | typed (F1b adds captured) | **Predicted: a named refusal,** "range: subnormal stiffness or load at formation", with the step-1 trigger `Range("arithmetic outside normal range")`. It is **not solved** (Q10) | The stiffness coefficients are all normal (EA/L 9.4e-154, 12EI/L³ 1.41e-306, GJ/L 1.07e-307). But the fixture's own exact-pressure end-cap operand p·π·r_i² is about 2.99 quanta of 2^-1074: subnormal at formation (`<wt>/scratch/briefs/f1b_scratch/phys_r4_operands.py`). It is formed by `pressure_group_value` (`pressure_exact/source_geometry.rs:116-134`; `scaled_output`, `pressure_exact.rs:433-440`, accepts any finite output). It is pushed by `push_exact_pressure_operands` (`PP:2426-2442`) as a plain `Term` (`push_formed`, `FK/load_ledger.rs:152-163`), at restrained DOFs. The orchestrator's census reads every term (`SA:1597-1599`), and `ForceScaleCensus::load_term` flags a subnormal `Term` (`FK/lib.rs:1217-1219`, `value` at `:1146-1153`; K2b ruling 2) |
228: | PHYS-R4 without pressure (same section, a nodal tip load) | both | Published, within 1e-9 of an exact reference (K2b's synthetic element: b = 536). Standing as ruled (Q5) | A new product test |
229: | K2b's documented-limitation chain (E = 2^440; K2b RETURN §13.3) | typed (captured refuses 2^440) | **A named refusal** (`ScaledEvaluation`), pinned as a documented limitation, not "fixed" | §4.7 step 4, as ruled |
230: 
