# T3 selection package

T3 WORKING_ITEMS manager for ROOT, 2026-09-26. **Status: FINAL, for ROOT's selection.**

- **Design basis:** D1 revision 5a.2 and D2 revision 5b.2 (§2). V1's targeted verification is **VERIFIED** (`REVIEW/VERIFY_R5.md`, `8097e13b`, at `57842c192`), including all four R5-4 conditions. No further review round, per ROOT's process ruling.
- **Not part of this selection, and still in progress:**
  - S11-K, already selected and being implemented: I1's regeneration, then RV1's independent review and the PR gates.
  - The RF-ELOAD references: a candidate under V3's refutation, selected separately before F3.

Nothing here is selected until ROOT records a selection.

## 1. What T3 asks ROOT to select

1. **The designs:**
   - D1's numerical-core design (`DESIGN_NUMERICS/DESIGN.md`), with `S11_CONTAINMENT.md` and `D5_TRIGGER.md`;
   - D2's standing, envelope and transport design (`DESIGN_STANDING/DESIGN.md`);
   - both at the revisions in §2, with ROOT's rulings (`ROOT_RULINGS_V1.md`, `ROOT_RULINGS_V2.md`) as binding amendments. Where a design and a ruling differ, the ruling wins.
2. **The slice plan and ordering** in §4, including the hard constraints.
3. **The decisions** in §5. ROOT ruled on all of them on 2026-09-26. The follow-ups changed none of them, apart from D-14's slice, corrected by ROOT to F3.
4. **R5-4**, curved bends under K-D5: pre-accepted (curved re-formation in EF). V1 has verified all four conditions. T3 asks ROOT for **final acceptance** (§6).

The references are already frozen (`ROOT_SELECTION_REFERENCES.md`). The S11 containment is already selected (`ROOT_SELECTION_S11.md`), and its kernel half, S11-K, is being implemented.

## 2. Records in the package

| Record | Revision / commit | sha256 (prefix) |
|---|---|---|
| `DESIGN_NUMERICS/DESIGN.md` (D1) | revision 5a.2 at `932698d7a` | `fb62ef4a` |
| `DESIGN_NUMERICS/S11_CONTAINMENT.md` | revision 5a.2 at `932698d7a` (selected at revision 3 plus R3) | `e6507587` |
| `DESIGN_NUMERICS/D5_TRIGGER.md` | revision 2a at `16bcbf369` | `f6e24a69` |
| `DESIGN_NUMERICS/R5_4_CURVED.md` | `932698d7a` (arctangent bound wording only; first committed at `4bc3e0696`, `4843c4e9`) | `2c9fae78` |
| `DESIGN_STANDING/DESIGN.md` (D2) | revision 5b.2 at `8b2bca91b` | `edc78f9c` |
| `REFERENCES/` (R1, frozen) | `c0f14201c` | `references.json` `7b176dbb` |
| Reviews | `REVIEW/RETURN.md`, `S11_CHECK.md`, `S11_BACKCHECK.md`, `S11_BACKCHECK_R3.md`, `BACKCHECK_R2.md`, `BACKCHECK_R3.md`, `D5_CHECK.md`, `BACKCHECK_R4.md`, `BACKCHECK_R5.md` | — |
| Targeted verification (V1) | `REVIEW/VERIFY_R5.md` at `57842c192`: **VERIFIED** | `8097e13b` |
| Reference checks | `REFERENCE_CHECK/RETURN.md`, `BACKCHECK.md`, `ERRATUM_N1.md` | — |
| Detection baseline (P1) | `DETECTION/` at `d28f69dd7` (return and typed follow-up) | `results.json` `a1188624` |
| Gate exception list | `GATE/S11_EXCEPTIONS.json` | see `GATE/SHA256SUMS` |
| Rulings | `ROOT_RULINGS_V1.md`, `ROOT_RULINGS_V2.md`, `ROOT_SELECTION_S11.md`, `ROOT_SELECTION_REFERENCES.md` | — |

## 3. What the designs do (in brief)

**D1, the numerical core:**
- **W1, general accuracy.** A retained-precision structural method: elements re-formed from binary64 primitives in basic-deformation form at precision p, a sparse profile LDLᵀ, and the stop rule `2^-64·max(|q_2p|, S*)` at p against 2p. Every source-level sum and combination is exact. Coverage comes in phases: W1a (straight frames, nodal loads, combinations), W1b (element, thermal, thrust and constant-effort loads, and prescribed motion), W1c (curved, with T4).
- **Floor enforcement.** Values are classified on the published binary64 value, with R = 2^-34. S\* is recomputable, and the stress propagation factors are pinned (1, √2, 2, √2·i; 2√2 and 4 for span statics). `absolute_verified` and `not_covered` quantities are withheld or interval-bound, and never counted Passed.
- **W2, range.** Exact power-of-two force scaling, which removes the PHYS-R4 refusal. Current results stay bit-identical.
- **W3, M32.** One sparse assembly, reduction and reaction path. Dense scrutiny stays explicit and guarded.
- **W4.** A constrained-body null-space witness, and the SUP-17 wording.
- **W5, VP-ROBUST.** A harness with kernel and product lanes. The "no Passed breach" gate runs through both entries, with the pinned S11 exception triples.
- **D-5, K-D5.** An exact-residual formation-and-solve error estimate, EF = K⁻¹(f − K_int·u), with factor 2 on the coupled S\*. Passed cases above it are demoted to Sensitive, and routed to W1 after F2. V1 verified that it has no misses on 230 + 105 + 18 cases.
- **S11.** An exact per-case load ledger plus exact recovery sums (S11-K, S11-F).
- **K2a.** Checked formation.

**D2, standing, envelopes and transport:**
- the composite-finalization fallback (F1 port of T1's SF-1);
- joined eligibility (S-E1 core with F3; S-E2 conditional);
- the selected-UNAVAILABLE tightening;
- source-blocks-1 re-homing under the shared gate;
- the capture fix S-H (never before S11-F);
- display representability;
- the transport split with T6;
- the successor-identity readers (S-G1, S-G2), with a closed (kind, unit) → class table;
- **C**, conservative interval binding (S-I1, S-I2), and the optional B reader contract (S-J).

## 4. Slice plan and hard constraints

**Kernel.** S11-K → K3a → K-D5 → K2a → K1 → K2b → K5. The rest of K3 runs in parallel after K3a. Then K4, then K6 and V-K.

**Facade (after the main merge at `303609725`).** S11-F (S-H in the same PR or after) → F1 → F2a (atomic with S-G1) → S-I1 and S-I2 (C) → F2b per domain → F3 (atomic with S-G2 and S-E1, its retirement step after S-I) → V-P → join. S-J is optional, after S-I. W1c comes with T4.

**Hard constraints (ROOT's rulings):**
- **S-H never lands before S11-F.** S11-F's tests run through both entries (captured, and typed via headless), including RF-CANCEL at 1e80.
- **No retirement where rows regress.** F2b retires exact-block selection domain by domain, and only when gate condition 3 passes at row level (side-by-side per-case withheld counts) and at check level (committed rule packs: no decided check becomes indeterminate). C or a proven B precedes F2b in every committed family.
- **Coexistence.** No W1 attempt in an invocation where exact-block selects any case.
- **The no-Passed-breach gate.** It runs through both entries. Its exceptions are exactly the (entry, case, quantity) triples in `GATE/S11_EXCEPTIONS.json`, re-pinned from P1's final results on frozen-reference cases only: 88 triples in 13 cases captured, and 140 in 22 cases typed. This supersedes the earlier 106/168, which was D1's prediction including the unpublished `Mb.*.mid` quantities (`ROOT_RULINGS_V1.md`). RF-SKEW-T-CANT-OFF-122-r1e-04 is not an exception; it is K-D5's required true positive on both entries. The list is empty after S11-F. Any other triple is a gate failure.
- **The nonlinear active-set loop**, including `product_equilibrium` on the loop path, stays on the legacy binary64 KS variant until T5 designs its residual policy. DEC-046's zero limits are untouched.
- **Fixture stop rule.** Every committed-byte change is reported to ROOT with its size before regeneration, by the producer only.
- **Gates for every slice.** Complete-diff independent review; hosted CI, including the surface-4 dual-viewport dispatch; a clean DEC-025 sweep; native witnesses on the owner's Mac where native paths are touched (recorded as outstanding if unavailable).

## 5. Decisions: ROOT's rulings of 2026-09-26

ROOT ruled on these before the final package, to save a round. They become final with the package, unless D1 revision 5a, D2 revision 5b or V1's verification changes something. Earlier rulings are not repeated here: D-5 (O1 final, subject to R5-4), D-12, D-13 (the S11 subdecisions), D-15, DD-7, DD-11, DD-13 (C), DD-14, and the process rulings.

| ID | Decision | ROOT ruling |
|---|---|---|
| D-1 | General method | A: retained-precision multiprecision with basic-deformation formation |
| D-2 | Arithmetic backend | In-repo `wide.rs` with V1-S9's hard-case test plan. `dashu-float` is the fallback, used only if `wide.rs` fails its tests. `rug` is rejected, so no LGPL owner question arises |
| D-3 | Method policy | Register `M03-INTEGRITY-MP-v1` as specified in D1 §9 |
| D-4 | Identities | A: reserve the successor identities. Exact-block selection is retired per domain under the gate and kept as an oracle. ROOT approves the concrete identity names at F2a; they are proposed in that slice's brief |
| D-6 | W2 | As proposed |
| D-7 | W3 fallback | No automatic dense fallback. Ceilings come from measurement |
| D-8 | Budgets | Deferred until the K6 and V-P measurements |
| D-9 | W4 | As proposed. T4 confirms the curved construction |
| D-10 | Serialization | As in §4 |
| D-11 | RF-ELOAD | Commission it now: a product-code-blind author, then a V2-style refutation. It lands before F3. Brief: `TASK_BRIEFS/R1_ADDENDUM_ELOAD.md`, including D-14's generated loads |
| D-14 | Equivalent-static owner | **T3, as a W1b extension in F3**, with W1b's element-load work and atomic with its RF-ELOAD gate; refused as `equivalent-static unsupported` until then (ROOT, 2026-09-26; "F2b" corrected to F3 after `16bcbf369`). The generated magnitude is computed exactly from the user's inputs (mass_per_length × g_factor × g; pressure × shape_factor × exposed_diameter) as the intended source, the same principle as re-forming stiffness from inputs. It is not the rounded binary64 product treated as input. The rule covers only the solve-time seismic and wind generators. Generated self-weight is an authoring-time document value, so it is an ordinary input (ROOT, after `28d96084f`). D1 pins this in revision 5a. RF-ELOAD covers generated loads |
| DD-1, DD-2 | Composite finalization | F1 port of SF-1, with R-1a and R-1b open until W1 |
| DD-3 | TS position | TS-a |
| DD-4 | Host-rounded exp fields | H-a |
| DD-5 | Selected-UNAVAILABLE | Tighten |
| DD-6 | Re-homing | Under the shared gate and D-15 |
| DD-8 | Transport split | As proposed |
| DD-9 | Eligible load-reference, joined and successor results | As proposed |
| DD-10 | Historical joined standing after F3 | Keep eligibility, only if S-E2 is built |
| DD-12 | Capture fix | H-1 (S-H), under the S-H/S11-F ordering |
| DD-15 | S-J | Build it, after S-I. Under R5-5's check-level gate, proven exact zeros are what let equality and sign checks stay decided rather than indeterminate, so S-J is likely what makes retirement possible in the B-covered domains. Otherwise it stays off the critical path |

**Exception pin and test routing (ROOT, 2026-09-26):**
- The VP-ROBUST exception list pins frozen-reference cases only, per entry. It was re-pinned from P1's typed-entry follow-up on the 15 captured S11 cases and the skew case (`GATE/S11_EXCEPTIONS.json`: 88 triples in 13 cases captured, 140 in 22 cases typed).
- P1's own S11-PROBE-A cases are not in the gate. They become S11-K/F test cases with exact expected nets.
- The typed-entry result for RF-SKEW-T-CANT-OFF-122-r1e-04 goes into K-D5's tests: K-D5 must fire on both entries.

Items left for the final package: V1's verification of R5-4, and anything the follow-ups change. The exception triples are pinned (§4).

## 6. R5-4: curved bends under K-D5 (pre-accepted; V1 VERIFIED; final acceptance requested)

D1's note `DESIGN_NUMERICS/R5_4_CURVED.md` (commit `4bc3e0696`, sha256 `4843c4e9`) found a sound design within T3, so no owner options are needed.
- **Design.** EF re-forms each curved contribution (and each user-stiffness joint, without the arctangent) in `Wide<2>` from its binary64 inputs, as an objective element built from the actual chord x_j − x_i. It then enters the exact residual like a frame. The cost is a Wide arctangent added to K3a, O(6³) Wide operations per element, and no extra solve.
- **Data.**
  - Realized curved bends are opt-in and cannot be set from the desktop; no committed model uses one.
  - Six realistic elbow models (12 case-modes, all Passed): revision 5's D5C-2 demotes 12 of 12; the re-formed check demotes 0, with its largest reading at 0.0019 of the criterion.
  - Four curved Passed breaches on main (up to 1.48×): all caught, with EF equal to the actual error to three digits. A check reusing the product's binary64 curved matrix misses 2.
  - T4's null-space confirmation is not a prerequisite.
- **ROOT (2026-09-26):** pre-accepted. Options (a), (b) and (c) fall away. Final acceptance depends on V1's targeted verification (`ROOT_RULINGS_V1.md`).
- **V1 (`VERIFY_R5.md`, VERIFIED):** all four conditions hold, reproduced both with D1's script and with V1's own independent re-formation (quadrature flexibility, K = AᵀXA from the actual nodes).
  1. **Rotation-consistent by construction:** rigid-mode residual 1.7e-59 (D1) and 4.4e-60 (V1), against 2.4e-16 for the product's binary64 matrix. No libm dependence.
  2. **All 4 curved Passed breaches caught:** trigger values 2.07–2.96.
  3. **None of the 12 E1–E6 case-modes demoted:** largest trigger value 0.0038.
  4. **Re-formation required:** reusing the product matrix misses 2 (0.798 and 0.509).
- **ROOT on V1's verification (2026-09-26):** the K3a arctangent bound is stated as V1's measurement (≤ 2.69 ulp at p = 128, over 13 angles), not a proof. That is acceptable for selection, provided the K3a slice's test vectors carry a few-ulp tolerance and the implementation slice owns any proof.
- **Findings:** the curved Passed breaches (M03, under the no-interim ruling, fixed by K-D5), and the product's curved element not being rotation-consistent on binary64 inputs (routed to T4 and W1c).

## 7. What T3 completes and what remains

This section is drawn from D1 §8 and D2 §8, at the final revisions. **Nothing closes by design.** A group closes only when it is fixed, verified and merged, with its paired validation evidence (the closure rule).

| Group or item | T3 completes (when merged and verified) | Slice(s) | Remains, and where |
|---|---|---|---|
| **M03** | General accuracy for the W1a and W1b families, including generated equivalent-static loads formed exactly from their inputs (D-14). The S11 load-cancellation repair on every route, on both the force and recovery sides. The PHYS-R4 and formation range (W2, checked formation). The formation-error trigger (K-D5), including curved and joint re-formation (R5-4). The rigid-null witness. The SUP-17 wording. VP-ROBUST | S11-K, S11-F, K-D5, K1–K6, F1–F3, V-K, V-P | Curved bends in the method (W1c, with T4); components and releases (T4, T7); nonlinear mixed recovery, gap classification, the S11 audit inside the nonlinear loop, the friction fold (T5) |
| **VP-ORACLES** (near-zero and finite-accuracy budgets) | The frozen R1 references (`c0f14201c`) and RF-ELOAD (a candidate, pending V3 and selection). Budgets are selected from the K6 and V-P measurements (D-8) | V-K, V-P, F3 | Curved references (W1c) |
| **VP-ROBUST** | The harness: kernel and product lanes, both entries, and the no-Passed-breach gate with the pinned S11 exceptions (`GATE/S11_EXCEPTIONS.json`), which must be empty after S11-F. The skew and curved cases are K-D5's true positives | V-K, V-P | — |
| **M32** | Sparse assembly, reduction, reactions and the gate in both modes; the parity and memory protocol; dense scrutiny kept, with a guard | K4, F-slices | Friction influence solves (T5); resource ceilings chosen by ROOT from measurement (P1 baseline: at n1000, sparse took 15–19 s and dense 204–487 s; every n10000 run aborted at 6 GiB) |
| **M34** | Solve-side range (W2). Publication representability. Display range (S-B). Transport range requirement, references and fallback. The capture fix (S-H, after S11-F). The comparison policy is stated, with predicates unchanged | K-slices, S-B, S-H | Carrier adoption, persistence and canonical export of \|x\| ≥ 2^53 results (T6) |
| **N05 ordinary accuracy** | Repaired on the ordinary and exact routes for W1a models | F2 | — |
| **General retained-source recovery** | Order > 2, skewed, weakly coupled and larger systems up to the measured budgets; W1b producers; exact combinations over retained states | F2, F3 | Curved, pressure regions, components (W1c and later) |
| **Composite SOURCE_BLOCKS_FINALIZATION_FAILED** | Closed: no `Err` (S-D, the F1 port of SF-1), and no fresh exact-block selection after F2 | F1, F2 | R-1a and R-1b in the pre-F2 window only |
| **T1-routed: joined load-reference-source-1 eligibility** | Scheduled by R-3(a): S-E1 with F3; S-E2 conditional | F3 | Historical joined standing after F3, only if S-E2 is built (DD-10) |
| **T1-routed: binding route** | Automatic for eligible identities | F2, F3 | Desktop export of the load-reference and successor identities (T6) |
| **T1-routed: selected-UNAVAILABLE alignment** | Closed (S-A), built into S-G | S-A | — |
| **T0R carries R-1, R-2** | Fresh results: moot after F2. Historical: containment kept by ruling (R-7 (i)) | F2 | None, unless a defect is found |
| **Standing (D-15)** | Today's selected-case withholding restored by C (S-I1, S-I2), and proven zeros by B (S-J). Retirement proceeds domain by domain, only under gate condition 3 (row level and check level) | S-I, S-J, F2b | Rows a check cannot decide within ±b read indeterminate, never pass. Case-scoped standing and F-P2's whole-invocation blocking (T6) |

**Open into implementation (T3):**
- V1's NOTE 8: the scripts `withheld_rows.py` and `b_proof.py` do not read `hanger.stiffness`, which `support_stiffness_input` also reads. It does not change the restrained set. This is a script fix for the K-D5/W1 slice (ROOT, 2026-09-26).
- The K3a arctangent: a test-vector tolerance of a few ulp, with any proof of the bound owned by the implementation slice.

**Routed elsewhere:**
- **T5:** the friction fold; the nonlinear closed-gap prescribed solves (option (c), legacy binary64 variants pinned); mixed recovery basis; gap classification.
- **T6:** case-scoped standing and F-P2; binary64 carriers and F-P7; desktop and canonical export.
- **T4:** the curved element's non-objectivity on binary64 inputs (R5-4 finding, for T4's graph row); curved null space; W1c.
- **T9:** the comparison policy stays unchanged; the M32 baseline.

## 8. Findings on main recorded during stage 1

All are in `STAGE0_MAP.md` §6, with the governing rulings:
- S11 cancelled loads: Passed breaches at gross/net ≥ about 3e7, confirmed by P1. The typed entry publishes gross-wrong values at ratios ≥ 2^53. No interim fix; S11-K and S11-F repair it.
- The skew and absorbed-spring Passed-band class, up to about 4e-9 relative at cond 1e6–6.7e7. It is broad in synthetic space, and the committed fixtures are clean. No interim fix; K-D5 repairs it.
- The capture refusal of |x| ≥ 2^53 (V1-S5), repaired by S-H after S11-F.
- The formation silent zero, repaired by K2a.
- N-S11-R, the naive residual diagnostic, repaired by S11-K.
- F-P2, a rejected case blocking the whole invocation. It fails closed; T6 and W1.
- F-P7, the binary64 carrier migration hazard, for T6.
- KS live in the nonlinear loop, handled by option (c) in S11-K.
