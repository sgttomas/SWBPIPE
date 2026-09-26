# T3 selection package (DRAFT)

T3 WORKING_ITEMS manager for ROOT, 2026-09-26. **Status: DRAFT.** Three things are still pending:
- D1's and D2's narrow final follow-up (R5-1, R5-2, R5-3, R5-5, N-1 to N-6, and R5-4's curved bound);
- V1's verification of those items;
- I1's S11-K pre-regeneration report.

The hashes in §2 are the latest committed revisions and will be updated when the follow-up lands. Nothing here is selected until ROOT records a selection.

## 1. What T3 asks ROOT to select

1. **The designs:**
   - D1's numerical-core design (`DESIGN_NUMERICS/DESIGN.md`), with `S11_CONTAINMENT.md` and `D5_TRIGGER.md`;
   - D2's standing, envelope and transport design (`DESIGN_STANDING/DESIGN.md`);
   - both at the revisions in §2, with ROOT's rulings (`ROOT_RULINGS_V1.md`, `ROOT_RULINGS_V2.md`) as binding amendments. Where a design and a ruling differ, the ruling wins.
2. **The slice plan and ordering** in §4, including the hard constraints.
3. **The decisions** in §5. ROOT ruled on all of them on 2026-09-26, subject to the follow-ups.
4. **R5-4**, curved bends under K-D5. It is open: either ROOT selects D1's bound, or ROOT takes owner options forward (§6).

The references are already frozen (`ROOT_SELECTION_REFERENCES.md`). The S11 containment is already selected (`ROOT_SELECTION_S11.md`), and its kernel half, S11-K, is being implemented.

## 2. Records in the package

| Record | Revision / commit | sha256 (prefix) |
|---|---|---|
| `DESIGN_NUMERICS/DESIGN.md` (D1) | revision 5 at `490f02982` (5a pending) | `42bba414` |
| `DESIGN_NUMERICS/S11_CONTAINMENT.md` | revision 5 at `490f02982` (selected at revision 3 plus R3) | `776a6309` |
| `DESIGN_NUMERICS/D5_TRIGGER.md` | revision 2 at `490f02982` | `da8ce574` |
| `DESIGN_STANDING/DESIGN.md` (D2) | revision 5a at `8bf23f794` (5b pending) | `04a466b1` |
| `REFERENCES/` (R1, frozen) | `c0f14201c` | `references.json` `7b176dbb` |
| Reviews | `REVIEW/RETURN.md`, `S11_CHECK.md`, `S11_BACKCHECK.md`, `S11_BACKCHECK_R3.md`, `BACKCHECK_R2.md`, `BACKCHECK_R3.md`, `D5_CHECK.md`, `BACKCHECK_R4.md`, `BACKCHECK_R5.md` | — |
| Reference checks | `REFERENCE_CHECK/RETURN.md`, `BACKCHECK.md`, `ERRATUM_N1.md` | — |
| Detection baseline (P1) | `DETECTION/` (pending P1's return) | — |
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
- **The no-Passed-breach gate.** It runs through both entries. Its exceptions are exactly P1's pinned (entry, case, quantity) triples (106 captured and 168 typed, re-pinned from P1's final results). The list is empty after S11-F. Any other triple is a gate failure.
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
| D-14 | Equivalent-static owner | **T3, as a W1b extension in F2b** (ROOT, 2026-09-26). The generated magnitude is computed exactly from the user's inputs (mass_per_length × g_factor × g; pressure × shape_factor × exposed_diameter) as the intended source, the same principle as re-forming stiffness from inputs. It is not the rounded binary64 product treated as input. D1 pins this in revision 5a. RF-ELOAD covers generated loads |
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
- The VP-ROBUST exception list pins frozen-reference cases only, per entry. It is re-pinned from P1's typed-entry follow-up on the 15 captured S11 cases and the skew case.
- P1's own S11-PROBE-A cases are not in the gate. They become S11-K/F test cases with exact expected nets.
- The typed-entry result for RF-SKEW-T-CANT-OFF-122-r1e-04 goes into K-D5's tests: K-D5 must fire on both entries.

Items left for the final package: R5-4, the re-pinned exception triples, and anything the follow-ups change.

## 6. R5-4: curved bends under K-D5 (OPEN)

D5C-2 demotes any Passed invocation containing a contribution EF cannot re-form. Realized DEC-070 curved bends are user-selectable (`modelView.ts:154`). ROOT asked D1 for four things, with V1 to check the result:
- how a user realizes a curved bend today;
- the committed and realistic models affected;
- a conservative formation-error bound for curved contributions entered into EF, and its cost;
- whether T4's null-space confirmation is a prerequisite.

**Pending D1's note.** If a sound bound exists within T3, ROOT selects it. Otherwise ROOT takes owner options forward with numbers:
- (a) demote until W1c: ROOT's lean, if few models are affected;
- (b) exempt with a caveat: ROOT recommends against it;
- (c) delay K-D5: only if (a) would demote most realistic models.

## 7. What T3 completes and what remains

To be filled from D1 §8 and D2 §8 when the follow-up lands, per group: M03, M32, M34, N05 ordinary accuracy, general retained-source recovery, the composite finding, the T1-routed items, the T0R carries, and the stage-1 findings (STAGE0_MAP §6). Nothing closes by design: every group closes only when fixed, verified and merged, with its paired validation evidence.

**Routed elsewhere:**
- T5: friction fold, nonlinear closed-gap prescribed solves, mixed recovery basis, gap classification.
- T6: case-scoped standing, binary64 carriers, desktop export.
- T4: curved null space, W1c.
- T9: the comparison policy stays unchanged, plus the M32 baseline.

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
