# I93 B2/B3-P: the plan for F2a breadth after B1 (documents only)

TASK (Type 2), I93, a planner for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/B2B3_PLAN.md`, sha256 `174550b0ea083b89abb85576dd1a62f264f136dbed8078200e0c036555170c8a`, verified before reading.

**What I did.** Documents and code reading, plus read-only Python with VENV against committed records: I82's evaluator `b1_eval.py`, unchanged, on I82's committed profile trees, and two small pricing scripts of my own (`_run_records/`). No cargo, vitest, native or solver job, no install, no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`. Scratch was `WT/scratch/i93_b2b3_plan/` only. Nothing went to the system temp directory.

**Notation.** WT, NUM, P, PP (= `P/core/product_physics/src`), T, R, RR and VENV as in the dispatch. Also:
- **RE** = `P/core/reporting/result_export`; **FK** = `P/core/solver/frame_kernel/src/structural/retained`; **PP-tests** = `P/core/product_physics/tests`.
- **RS**, **PY**, **TS** = the three retained readers: `RE/src/retained_precision.rs`, `P/core/analysis_runs/retained_precision.py`, `P/apps/desktop/src/features/results/retainedPrecision.ts`.
- **PLAN** = `R/I61/u8_plan_01/PLAN.md`; **DESIGN** = `R/I78/b0_contract_01/DESIGN_v2.md`; **PLAN_v2** = `R/I84/b1_plan_01/PLAN_v2.md`; **STUDY** and **ADD** = `R/I82/b1_cap_study_01/STUDY.md` and `ADDENDUM_01.md`.
- **C1**, **C2**, **C3** = `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`, `R/I32/f2a_wire_c2/CONTRACT_DELTA.md`, `R/I52/prepared_public_contract_02/C3_DELTA.md`; **SC1** = `R/I32/f2a_wire_c1/SOURCE_COMBINATIONS.md`.
- **DN** = `T/DESIGN_NUMERICS/DESIGN.md`; **D2** = `T/DESIGN_STANDING/DESIGN.md`; **QUAL** = `T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`; **DOMAIN** = `R/I65/u4_g2_01/DOMAIN.md`.
- **CORPUS** = `P/fixtures/results/retained_precision_cases.json`; **PTABLE** = `P/fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json`; **SCHEMA** = `P/schemas/retained_precision_mp_v2.schema.json`.
- **c** = load cases; **z** = combinations of any basis; **C_eq** = c + z ("result sets"); **A** = the attempted cases (T-4).

**Basis.**
- **Code:** NUM `1d223f78be`, whose maintained tree equals main `2007709549` (B6 merged). Cited by symbol.
- **B1, read-only and moving:** `b1` at `603e238517` (SP after I2), `b1-r` at `b5cb7faaeb`, `b1-p` at `11cc14e3e6`, `b1-t` at `7e47e51b5d`; `b1-a` (`9812c83ded`) is merged into `b1` at I2. I read their structure, not their final bytes.
- **Rulings:** RR through "B2/B3's plan dispatched as I93 while B1 implements".

## 0. Findings in brief

1. **The units.**
   - **B2 is one unit with two internal stages,** in one contract, one corpus snapshot and one PR: **B2a** (combinations with at least one selected operand, plus receipt coverage of every combination) and **B2b** (C3a: prepared operands, that is, mixed and preparation-only operands). They share the kernel certificate and the receipt families; splitting them into two PRs would buy nothing and cost a re-qualification (§1.1).
   - **B3 splits by route:** **B3a**, 0.3.0 `legacy_pressure_v1` with zero pressure, on the existing preview successor (small); and **B3b**, 0.3.0 exact under `physics-retained-1` (large).
   - **Recommended order:** designs and probes now (phase 0), then, after PR-B1 merges: B3a, then B3b on the producer lane while the kernel lane builds B2's FK extension; then B2's producer; one PR-B2 (§2).
2. **B2 needs kernel (FK) work that does not exist today.** This is the largest new fact in this plan.
   - `RecordedInvocation::solve_combination` (FK `origins.rs`) accepts only **selected** operands: any other operand returns `OriginError::MissingSelectedOrigin`.
   - The product certificate refuses combination owners: `product_case_owner` requires `NativeOwner::Case`, and the source bridge view returns `SourceBridgeViewIssue::UnsupportedCombination` whenever `prep.factors` is non-empty (FK `adaptive.rs`). I42, I43 and I44 recorded combinations as explicitly unsupported.
   - So a retained combination needs (a) SC1 §1's `PreparedCaseSource` / `CombinationOperand` / `solve_sources` API, recorded, and (b) the final-row certificate for combination owners. (b) is numerical design, reviewed as I42–I44 were. FK, a stop in B1 (PLAN_v2 R8), becomes planned scope for B2 with its own fresh numerical reviewer.
3. **Components stay out.** The kernel source has `StraightMember` only (FK `source.rs`), and DN §8 places components in W1c. B2 keeps D1.4's `components = 0` and D1.8.
4. **B2 and B3b never meet, and coexistence never fires with combinations.**
   - The exact route blocks every combination (`pressure_runtime::validate_profile`: `EXACT_PRESSURE_COMBINATION_UNSUPPORTED`, blocking).
   - Exact-block is ineligible whenever a combination exists (PP `source_eligible`: `captured && !nonlinear && !combinations`), so T-3 (c) never fires for a combination-bearing invocation.
5. **The preview successor table must be revised for B2.** PTABLE's `accuracy_classification.scope` reads "registered ordinary-prepared source only; no exact-profile or prepared-combination extension". B2 publishes exactly that extension. The table's hash is pinned in 12 maintained files (§1.2.7). Since the hash moves anyway, RV78-N1's policies can be bound in the same revision, as RV78 itself suggested ("at the next table revision").
6. **Memory: a D1-cap combination beside three D1-cap cases does not fit within 12 GiB** (priced with I82's evaluator on I82's trees; §3.3):

   | Shape at D1's model caps | Dense E+R, low / mid / high (GB) | Smallest 256 MiB step with a 5 % text budget |
   |---|---|---|
   | c = 3, z = 1 | 11.42 / 12.43 / 13.06 | 12.25 / 13.25 / 14.0 GiB: **above 12 GiB in all three** |
   | c = 2, z = 1 | 8.15 / 9.13 / 9.75 | 8.75 / 9.75 / 10.5 GiB |
   | c = 1, z = 2 | 6.60 / 8.53 / 9.75 | 7.0 / 9.0 / 10.5 GiB |

   - **The cap shape B2 needs: one tier, C_eq = c + z ≤ 3** at D1's model caps, every combination counted as one case-equivalent, with at most 3 terms per combination. Its conservative price is S3's, so M is expected to stay at B1's selection (about 10.5 GiB).
   - **Three cases plus a combination** needs M above 12 GiB (the owner's decision 17), or a trimmed second tier (ROOT's, within 12 GiB), priced in §3.3.
   - **A combination costs about 0.8 of a case** at D1's caps (mid estimate), because it has no ordinary solve but repeats the W1 row text and the envelope rows.
7. **Two standing questions arise that B1 never met** (decision 8):
   - **subtraction and range combinations,** and ordinary-only mechanics, over an operand that is not `not_required`: their rows are the ordinary superposition of uncertified operand rows, yet they would ride in an eligible successor;
   - **a `retained_unavailable` combination:** C1 §6 forbids it gaining reliance from selected operands.
   
   I recommend one row-level rule (R-COMB-1): such rows are `not_covered`, withheld from binding with D-U6-2's disclosure, and their values are unchanged.
8. **`base_withheld` cannot be produced in B2's domain.** D1.5 and D1.6 exclude every mechanics gate (nonlinear supports, constant effort, modulus bases). It is pinned synthetically only. `pre_source_refusal` is reachable only through hooks (one stiffness under D1.5; ordinary validation refuses empty or non-finite terms).
9. **B3b is a design-first unit with unpriced memory.**
   - It needs a new formation definition (`RP-PREPARED-EXACT-DUAL-v1`, suggested), the new table, and the exact route's row recipes in the certificate.
   - SCHEMA's `ProductAttempt.definition_id` is a `const` of the ordinary definition, so the schema changes.
   - The exact route lights up TEXT sites that I65's rules zero under D1.3 (for example `append_exact_pressure_results`, `source_receipt/composite.rs`, `pressure_material.rs`, the `build_pressure_case_with_members` edges). So its price is unknown until a study (B3-S) runs I82's chain with those rules rebound.
10. **The explicit-row rule: adopt it at PR-B2.** B3b adds many non-candidate text sites, and PR-B2 is followed by B7, F2b, F3 and S-I2, each re-qualifying. A mechanical rule pays off where a by-type sweep would have to be repeated each time (decision 11).
11. **B4 is not needed for B2 or B3.** B2 fits under B1's M by counting combinations against the case budget. B4's real question is product reach, which needs the owner's view of B1's measured peaks (R9). A study question is given in §4.
12. **The estimates** (reading estimates, calibrated against B1's plan; §2.5):
    - **Agent:** about 195–300 h. **Review:** about 60–90 h.
    - **Elapsed:** about 10–15 working sessions after PR-B1 merges, if phase 0's designs are accepted by then.
    - **Against I61's 50–80 h,** the growth comes mainly from the kernel certificate, B3b's formation definition and the readers' combination validation.
13. **Nothing found changes the owner's F2a order.** Three conditions would (§7):
    - B3b proves FK-heavy, or does not fit;
    - the owner wants three cases plus a combination at D1's caps;
    - the owner judges D1-scale activation not worth it after R9.

## 1. The units

### 1.1 Boundaries and order

| Unit | Scope | Identity | Changes the D1 call graph? | Widens the domain | Kernel (FK)? | Size |
|---|---|---|---|---|---|---|
| **B2a** | Mechanics combinations with ≥ 1 selected operand, retained (own solve, own certificate); receipt coverage of every combination (`ordinary`, `base_withheld`) | `preview-physics-retained-1` (table revised) | Yes | D1.4: combinations, with C_eq ≤ 3 | Yes: combination certificate; recorded combination owner | Large |
| **B2b** | C3a: `not_required` operands prepared without a solve (`OperandPreparation`); unavailable-with-source operands; mixed combinations | Same | Yes | No further clause | Yes: `PreparedCaseSource` / `CombinationOperand` / `solve_sources` | Medium |
| **B3a** | 0.3.0 `legacy_pressure_v1` with zero pressure (DN §4.3), no new identity (DESIGN decision 15) | `preview-physics-retained-1` | Barely: admission and three readers' G8 | D1.3: one contract value | No | Small |
| **B3b** | 0.3.0 exact with explicitly empty pressure regions | `physics-retained-1`, profile `exact_straight_retained_w1a_v2` (reserved) | Yes | D1.3 (exact contract), D1.5 (empty regions on the exact route) | Possibly: an exact annulus preparation version (B3-D decides) | Large |
| **B4** | Cap growth | — | Possibly | D1.9 | — | Not needed for B2 or B3 (§4) |

**Why B2 is one unit.**
- The certificate for combination owners is needed whether the operands are selected or prepared: the combination's own source is K4CMB over its operands' `CasePrep`s either way (FK `combine.rs`; C2 §3).
- The receipt families (combination entries, `CombinationSource`, the mechanics Call, Group imports) are shared.
- B2a alone would ship DESIGN §4.2's P3 ("retain a combination only when every operand is selected"), which RR:7051 forbids for full F2a except as a declared interim. Inside one branch and one PR that interim never reaches main.
- The stages remain useful as an implementation order inside B2: the producer builds B2a first, then B2b.

**Why B3 splits.** B3a is a one-value namespace widening on an existing identity; B3b is a new identity, table, formation definition and reader branch. B3a lands early and proves the namespace-widening and registration path at little cost.

**Recommended order** (detail in §2):
1. **Phase 0, now, documents and probes only, as slots free:** B2-C (contract), B2-KD (kernel certificate design), B3-D (B3b design), their reviews; B3-S (exact-route pricing); B2-W and B3-W (witness probes, one case at a time on main's registered build).
2. **After PR-B1 merges:** branch `b2` from main. Lane K builds B2-K (FK). Lane P builds B3b's producer. Lane A carries B3a's, B2's and B3b's admission changes.
3. Lane P then builds B2's producer on B2-K (B2a, then B2b).
4. Reader lanes follow the contract text: RS first, then PY and TS. Then corpus 07o, carriers, and the T6S policy entry.
5. **PR-B2's qualification** (SQ2), integration, gates and merge.

### 1.2 B2: combinations, mixed and preparation-only

#### 1.2.1 Definitions

- **A retained mechanics combination:** a `mechanics` combination that passes T0R's gates and has at least one operand case `selected` after T-9 (decision 5; SC1 §2 table, row 1).
- **Operand sources** (decision 6; C3a rule 1):
  - a `selected` operand supplies its `RetainedSolve`;
  - an `unavailable` operand with a prepared source (its Run ended unresolved or refused, or its freeze failed) supplies that prepared `CaseSource`;
  - a `not_required` operand gets one `OperandPreparation` (C3a), shared by every retained combination that needs it;
  - an `unavailable` operand without a prepared source (T-7 preparation failure) makes the combination `retained_unavailable` with no second preparation.
- **"Mixed":** a retained combination with both a selected and a prepared operand. **"Preparation-only operand":** a `not_required` case whose only W1 work is its operand preparation.
- **Every other combination** has a coverage entry and no W1 work:
  - `ordinary` with reason `no_retained_mechanics`: subtraction, range, and mechanics with no selected operand ("ordinary-only mechanics");
  - `base_withheld`: a T0R-gated mechanics combination.

#### 1.2.2 Contract changes (B2-C writes them; RV-C reviews; ROOT selects)

**The C3 amendment (C3a).** DESIGN §4.2's P1 text, selected at B0, with ROOT's reserved spellings (`operand_preparations`, `OperandPreparation`, `operand_preparation_failure`, `retained_precision_operand_preparation_v1`, `combination_operand`). B2-C adds only what P1 left open:
- **Placement:** operand preparations run after T-9, before the first combination Call, in the authored order of the combinations that need them, at most one per owner case.
- **Record point:** the record's terminal stage, as T-11's snapshots.
- **Work:** C3 preparation work as it actually happened, outside LME and outside every budget (P1 rule 2).
- **Gate placement and codes:** G3 coverage; G5 stage, result and work (`…_PRODUCT_ATTEMPT_MISMATCH`, `…_WORK_MISMATCH`); G8 preparation binding (`…_PREPARATION_MISMATCH`).

**The combination transaction (the C1 §5 and C2 §3–§4 completions):**
- **T-9b, after T-9.** For each model combination in authored order, its disposition (§1.2.1).
- **T-9c.** For each retained combination:
  - one `MechanicsCombinationCall` on the invocation's one meter (C1 §2: 20B per combination, 60B for the invocation; chained after the `CaseBatchCall`);
  - its `CombinationSource`;
  - its Group, importing only the selected operands' snapshots in authored order (C2 §4; prepared operands import none);
  - its Run;
  - then its own freeze (certificate, observables, G5a) over its own rows.
- **Failures** (decision 7). The combination alone becomes `retained_unavailable` on:
  - a pre-source refusal;
  - an unresolved or refused Run;
  - a facade certificate failure;
  - an operand preparation refusal (`operand_preparation_failure`, null `call_ref`; C3a rule 4).

  The whole successor is abandoned (T-12) on:
  - an origin or capacity refusal (an invariant failure);
  - staging, any serializer failure, or precommit (DESIGN decision 5's set).

  T-12 is unchanged: notices are reserved and published only for the cases in A, never for combinations.
- **Staging.** After the cases, in authored order:
  - a `retained_selected` combination overlays its rows with the method token;
  - a `retained_unavailable` combination keeps its ordinary rows and gets one `RETAINED_PRECISION_UNAVAILABLE` diagnostic whose `affected_refs` names the combination (C1 G4's analogue);
  - a `retained_selected` combination gets one `RETAINED_PRECISION_SELECTED` diagnostic naming it.
- **Execution order:** the case Runs, then the combination Runs in authored order (`execution_order` entries of kind `combination`).
- **Standing, R-COMB-1** (decision 8). Two kinds of row are classed `not_covered`, with D-U6-2's disclosure, and their values are unchanged:
  - every row of an `ordinary` combination whose expression references a case that is not `not_required`;
  - every row of a `retained_unavailable` combination.

  Envelope standing is otherwise as D2 §4.9.4 (an unavailable case still makes it `needs_recompute`).
- **D1.4** (decision 10):
  - 1 ≤ c ≤ 3 and c + z ≤ 3;
  - at most 3 terms per mechanics combination (repeats counted) and at most 3 range operands;
  - components 0.
  
  D1.5–D1.8 are unchanged.
- **SCHEMA** (B2-C specifies, the readers' lanes install). New `$defs`:
  - `Combination`, with its four dispositions;
  - `CombinationSource`;
  - the mechanics `Call` branch (`requested_operands`; results `runs` and `pre_source_refusal`);
  - `Group.imports`;
  - `OperandPreparation`, and `CaseSource.preparation`'s second branch.

  Also: `combinations` loses `maxItems 0`. (`execution_order`'s items are already `Owner`, which admits `combination`, and `Basis` already admits `ref_type: combination`.) SCHEMA is a reviewed static, so its change enters the registration (§3.2).
- **PTABLE revision** (decision 9). `accuracy_classification.scope` gains the prepared-combination extension. At the same revision, RV78-N1's policies are bound:
  - `RP-LOGICAL-ATTEMPTS-v1`;
  - `W1-LME-20B-60B-v1` with its 20B and 60B limits;
  - the method token `contribution_preserving_multiprecision_v1`;
  - the canonicalization profile `openpipestress_jcs_ijson_v1`.

  The contract id, profile, rows and inherited hash are unchanged.

#### 1.2.3 Kernel changes (B2-KD designs; B2-K implements; RV-K reviews both)

| Change | Files | Basis |
|---|---|---|
| `PreparedCaseSource::new(PrimitiveSource)` (CasePrep only: no solve, factor, cache or radius); `CombinationOperand::{Retained, Prepared}`; `RetainedCombination::solve_sources`, with the same validation order (no operands → nested → operands differ); `solve` stays a byte-identical compatibility wrapper | `FK/combine.rs`, `FK/adaptive.rs` | SC1 §1 |
| `RecordedInvocation::solve_combination_sources`: selected operands must match their recorded selected origins (as today); prepared operands must be sources registered in this invocation; it requires at least one selected operand, and returns a typed refusal otherwise (decision 5 makes the no-selected path unused) | `FK/origins.rs` | C2 §4; SC1 §1 |
| The product owner generalized to `NativeOwner::Combination` (`product_case_owner`, `product_owner_stamp`, `certify_product_case`, `begin_prepared_product`) | `FK/origins.rs`, `FK/product_certificate/final_case.rs` | I31's "combination owns a fresh solve/certificate" |
| **The combination source view** in the bridge, residual and tightening. It reads the combined exact ledger (K4LED: the exact products cᵢ·v, `RetainedLedger::combined`) and the h·k prescribed pairs, with the representative source's maps. It replaces `UnsupportedCombination` with a checked combination view. **This is the numerical design item** | `FK/adaptive.rs` (`SourceBridgeView`), `FK/product_certificate/{bridge,source_residual}.rs` | I42–I44 (the case versions); C2 §3 (provenance as `(operand_index, term_index)`) |
| Re-exports | `.../frame_kernel/src/structural.rs` | — |
| Tests: SC1 §5's C01–C06 and W05–W06 that apply. All-selected old API against the new one gives identical bytes; mixed operands select with no ordinary operand solve; the combination certificate on cancellation (A + B − A2) and on a factor-scaled ledger | `.../frame_kernel/tests/retained_k4/{combine,origins}_tests.rs` and the product-certificate tests | SC1 §5 |

**The stop:** any change to the stop rule, the schedule, prices, the ceiling or an existing case's bytes is an R8-style stop.

#### 1.2.4 Producer and admission changes

**PP producer** (lane P):
- **`lib.rs`:** `w1_case_ids` admits combinations within D1.4, keeping the domain re-check (c = 0, c > C, C_eq > 3, terms > 3: `Domain`). `w1_transaction` gains T-9b and T-9c between T-10's check and staging.
- **`retained_product.rs`:**
  - operand preparation (C3a), reusing `prepare_cases`' per-case machinery for one owner without native;
  - combination attempts and their freeze over combination rows (a combination `CaseScope`: its rows are contiguous in append order, `append_combination_results`);
  - the staging overlays.
- **`retained_wire.rs`:**
  - `combinations[]`, `CombinationSource`, the mechanics Call, Group imports, `operand_preparations[]`, and `CaseSource.preparation`'s second branch;
  - `execution_order` entries of kind `combination`;
  - `owner_refs` of kind `combination`;
  - the ordinal mapping (FK's `NativeOwner::Combination(i)` to authored indices).
- **`retained_receipt.rs`:** the projection for combination Runs (C1 §1, unchanged law).
- **Hooks (`retained_tests_hooks/grant2.rs`):** `fail_operand_preparation(case)`, `fail_combination_call(index)`, and a combination freeze fault.

**PP admission** (lane A, `retained_memory.rs` outside the generated block):
- D1.4 as §1.2.2;
- new `cap_rows` (`Combinations`, `CombinationTerms`, `CaseEquivalents`);
- G-C's `EnvelopeResults ≤ C_eq·P_final`, which is numerically today's 3·P_final;
- `RetainedErrorTextBytes ≤ C_eq·(3m+1)·Text(err)`;
- unchanged: G-B (no combination has its own loads) and T-3 (e).

**Out-of-domain oracles** re-based to C_eq + 1, as PLAN_v2's SF-2 did, including the runner's literal and its tie test. No D1 `src` visibility changes for a test.

#### 1.2.5 Reader changes (three languages; one census and one D38-style audit per lane)

| Gate | Combination obligations (C1 §5–§6, C2, C3a, R-COMB-1) |
|---|---|
| G1/G2 | The new `$defs`; Bits on factors; safe integers |
| G3 | One entry per model combination in authored order; combination-basis `result_ids` exhaustive and disjoint; every `not_required` operand of a retained combination has exactly one `OperandPreparation`, whose owner is `not_required`; execution-order bijection including combination Runs |
| G4 | One selected or unavailable diagnostic per retained combination, naming it; none for `ordinary` or `base_withheld` |
| G5 | The mechanics Call (results `runs` / `pre_source_refusal`, null `call_ref` only with `operand_preparation_failure`); Group imports (backward only, first occupied slot in authored order, no import from a prepared operand); `CombinationSource` (`representative_source_ref` = operands[0]; operand identities resolve); C3a stages and work; the meter chain across Calls |
| G5a–G5c | Combination rows use the combination's own body scales and section terms (through the representative source); R-COMB-1 assigns `not_covered` |
| G6 | The token only on `retained_selected` combination rows |
| G7 | Unchanged base validator, including T0R's combination gates |
| G8 | Model combinations allowed (PY and TS refuse any today; RS requires `combinations` empty); expressions equal the invocation's (terms, factor bits, subtraction operands, sorted range ids, mode); operand statuses; C3a preparation binding |

Each reader's census over 07n must show zero changes (R5's rule). The table-hash cascade (§1.2.7) rides the same lanes.

#### 1.2.6 Corpus and witnesses

**Witnesses** (B2-W probes their one-case proxies first, I61's U8-0 method: a combination's native outcome is predicted by a one-case request carrying the combined loads):

| Witness | Construction | Expected | Status |
|---|---|---|---|
| W-CB1, all-selected, selected | SW's cap-maximal cases A and B (each publishes alone; RR "I86's SW probe accepted…") plus `A + B` | `retained_selected`, C_eq = 3 at D1's caps: also the S1, challenge and RSS input | Proxy to probe |
| W-CB2, mixed, unavailable | U8's two-body model: case A (selected) plus case B (`not_required`, so an operand preparation), and `A + B`. Its loads are W-C2 case C's, which ends `Unresolved(Ceiling)` (RR "I81's B1-0 probe verified…") | `retained_unavailable`, `combination_unresolved`; R-COMB-1 rows | Predicted by case C |
| W-CB3, mixed, selected | A `not_required` case on a body that does not change A's numerics, for example a force on the L = 0 base's fully restrained isolated node, plus the milestone case | `retained_selected` with a prepared operand | **Unknown:** the probe establishes the verdict. Stop rule in §6 |
| W-CB4, subtraction and range over a selected case | Two-body A − B; range over (A, B) | `ordinary`; R-COMB-1 `not_covered` rows | Producer-emittable |
| W-CB5, ordinary-only mechanics | Two-body: A selected, B `not_required`, combination `2·B` | `ordinary` (`no_retained_mechanics`) | Producer-emittable |
| Hook-only | Operand preparation refused; a combination Call refused; a combination freeze fault | As §1.2.2's failure rules | Hooks |
| Synthetic | `base_withheld`; `pre_source_refusal` with a typed reason | Labelled "not producer-emittable in B2's domain" | Corpus only |

**Corpus 07o** (one writer, append-only after B1's 07n):
- **Bases:** W-CB1 to W-CB5 in both modes (D-U6-5 copies), and the synthetic bases.
- **Must-pass entries** for each base.
- **Mutations:** one or more per new gate branch above, each with a single first failure shared by all three readers, fixed from the readers.
- **The C3a mutations:** owner not `not_required`; a `requested_by` naming a non-retained combination; a refused record with a source; a prepared operand importing slots.

**PP fixtures:** `P/fixtures/results/retained_precision_combination_successor_{sparse_interactive,dense_scrutiny}.json` (W-CB1 or W-CB3, whichever selects).

#### 1.2.7 Write sets, by file

| Lane | Files |
|---|---|
| K (FK) | `FK/combine.rs`, `FK/adaptive.rs`, `FK/origins.rs`, `FK/product_certificate/{final_case,bridge,source_residual}.rs`, `.../frame_kernel/src/structural.rs`; tests `.../frame_kernel/tests/retained_k4/{combine,origins}_tests.rs` and the product-certificate tests. SC1 §4's `Refused` consumer list (performance harness, numerical-robustness lane) only if a pattern breaks, mechanically, with outputs unchanged |
| P (PP producer) | `PP/lib.rs` (retained section), `PP/retained_product.rs`, `PP/retained_wire.rs`, `PP/retained_receipt.rs`, `PP/retained_tests_hooks/grant2.rs`, `PP/retained_facade_tests.rs`, `PP/retained_product_tests.rs`, `PP/retained_wire_tests.rs`; `PP-tests/s11f_site_test.rs` (rule-8 rows), `PP-tests/retained_precision_admission.rs` (expected sections); the new combination successor fixtures |
| A (admission) | `PP/retained_memory.rs` (outside the generated block), `PP/retained_memory_law_tests.rs`, `P/core/runner/headless/tests/retained_precision_admission.rs` |
| RS | `RE/src/retained_precision.rs`, `RE/src/semantic_contract.rs` (the table hash), `RE/tests/retained_precision_contract.rs`, `RE/tests/retained_precision_carriers.rs` |
| PY | `P/core/analysis_runs/retained_precision.py`, `P/core/analysis_runs/compatibility.py` (the hash constant), `P/tests/test_retained_precision_contract.py`, `test_retained_precision_schema.py`, `test_retained_precision_carriers.py`, `test_load_reference_source_schema.py` (the hash) |
| TS | `retainedPrecision.ts`, `retainedPrecision.test.ts`, `numericalResultQuality.ts` (the hash), `retainedPrecisionIntegration.test.tsx` if a pin moves; T6S: the successor path's combination basis reference in `features/stress-neutral/StressNeutralExportPanel.tsx` and its tests, if `sourceBasisReference` does not already map `combination` (I74 §4.3's B2 note) |
| Statics (one owner, lane A or T) | SCHEMA; PTABLE; `P/schemas/analysis_run.v0.3.schema.json` (four hash constants) and `P/schemas/stress_neutral_export.v0.3.schema.json` (one) |
| Corpus (one writer) | CORPUS (07o); `P/fixtures/results/retained_precision_carrier_cases.json` if a carrier case is added |

**The 12 maintained files that pin PTABLE's hash** today (`git grep` at NUM): the two TS files, `compatibility.py`, `retained_precision.py`, `retained_memory.rs` (`reviewed_inputs`), `retained_memory_law_tests.rs`, RS, RE `semantic_contract.rs`, the AnalysisRun and stress-neutral schemas, and two Python schema tests. The derivative fixtures do not pin it.

**Never touched without a stop:**
- FK outside lane K's list;
- base (non-retained) readers;
- the ordinary route's behaviour;
- physics-1's table bytes (DESIGN decision 14);
- any c = 1 or B1 multi-case successor byte;
- the visibility of a D1 `src` item for a test.

#### 1.2.8 What B2 needs from B1's final state

| From B1 | Why |
|---|---|
| The n-case transaction T-1 to T-13 (`w1_transaction`, `prepare_cases`, the `CaseBatchCall`, per-case freeze, staging, `serialize_cases`, T-12) at PR-B1's head | B2 inserts T-9b and T-9c and extends staging and the serializer. It must not reopen T-1 to T-13 |
| S3's admission (`LOAD_CASES` = 3, `TOTAL_LOADS` = 384) and B1's registered M (expected 10.5 GiB) and profile | B2's cap shape is S3's with C_eq in place of c |
| RS, PY and TS aligned (D38 (4b), F-1 text B, G5 `not_required`) and corpus 07n | B2's readers and 07o build on them; R5's census runs over 07n |
| U8's two-body model, W-C2's three cases, SW's cap-maximal A, B, C and `c1` | The witnesses above |
| SQ's TEXT loop rules for B1's loops, `RSS_TIME.md`'s method, the challenge's phase mapping, Pass B's tooling, and R9's report to the owner | SQ2 reuses them |
| RV87's sweep as re-run at PR-B1 | The baseline for the explicit-row rule |

### 1.3 B3a: 0.3.0 `legacy_pressure_v1` with zero pressure

- **Contract:**
  - **D1.3:** admit `schema_version == "0.3.0"` with `pressure_contract == {version "1.0.0", mode "legacy_pressure_v1"}`.
  - **D1.7 unchanged:** "zero pressure" means no pressure primitive loads, so a zero-magnitude pressure load stays outside D1.7 (decision 12). The ordinary route already refuses a non-zero legacy pressure load (`PRESSURE_MODEL_REAUTHOR_REQUIRED`).
  - **The identity stays `preview-physics-retained-1`** (DESIGN decision 15). The successor carries the model's contract in its invocation only.
- **Coexistence:** exact-block cannot select here. `source_recovery` requires model 0.1/0.2 without a pressure contract on the legacy route, so T-3 (c) never fires.
- **Producer:** I expect no change. The capture (`retained_product.rs`) refuses only `is_exact`, load-state, combinations and components. B3a-W confirms this through the private driver.
- **Readers:** G8's namespace check admits this one contract value. Today RS requires `pressure_contract` null, PY refuses any contract, and TS refuses any contract. Corpus pins: one producer-solved base (the milestone authored as 0.3.0 legacy) and mutations (contract mode or version changed; a non-zero pressure load).
- **Admission and pricing:** `validate_profile` already runs for every model, and its 0.3.0 legacy arm emits nothing for a valid legacy model (no regions, no non-zero pressure load). The namespace's raw strings are inside the raw caps. I expect no profile change beyond rebinding any TEXT rule that zeroes that arm under D1.3; SQ2 confirms it.
- **Write sets:** lane A (`retained_memory.rs` D1.3, law tests); three readers' G8 and tests; 07o entries.
- **Needs from B1:** S3's admission code and the aligned readers.

### 1.4 B3b: 0.3.0 exact under `physics-retained-1`

**Contract (B3-D designs; RV-D reviews RV69-style; ROOT reserves):**
1. **`RP-PREPARED-EXACT-DUAL-v1`** (suggested; collision-checked and reserved at B3-D), with its own H domain. It is the exact profile's preparation:
   - normalized OD and effective wall per `SOURCE_ODWALL_EXPECTATIONS` (C2 §3);
   - E and ν from the exact material basis (`homogeneous_isotropic_E_nu_v1`), with G derived (`shear_origin: derived_e_nu`, already a SCHEMA branch);
   - `section_terms.geometry.route: exact` (already a SCHEMA enum value);
   - the dual-readout proof over the exact route's actual final rows.
   
   B3-D lists every row family the exact route publishes with empty regions (including any zero pressure-evidence rows from `append_exact_pressure_results`) and its recipe or class. It also says whether FK's annulus preparation (`prepare_product_annulus`, `AnnulusPreparationVersion`) needs an exact version; if so, that is lane K's second item.
2. **The table** `P/fixtures/results/semantic_contract_v0_3_physics_retained_1.json`, as DESIGN §5 item 1:
   - the inherited hash is the raw sha256 of `semantic_contract_v0_3_physics_1.json` (`9a2cf626…`, rechecked at NUM);
   - `product_formation_definitions` names the exact definition;
   - **RV78-N1's policies bound from version 1** (RR "I86's SW probe accepted…", ruling 1);
   - physics-1's table bytes are untouched, including `reserved_inactive_successors` (DESIGN decision 14).
3. **D1.3 and D1.5:**
   - admit 0.3.0 with `{2.0.0, exact_straight_pressure_v2}`;
   - on this route only, every case's `pressure_regions` is `Some([])` (explicitly empty, as physics-source-1 requires);
   - sections stay absent (the committed exact requests have none);
   - D1.6 is already implied (the exact route refuses nonlinear and constant-effort supports);
   - combinations cannot occur (§0 item 4).
4. **Coexistence:** physics-source-1 sets `source_block_recovery`, so T-3 (c) applies unchanged. Every invocation where it selects a case publishes exact ordinary bytes.
5. **SCHEMA:** `ProductAttempt.definition_id` (and `OperandPreparation`'s, from B2) becomes an enum of the two definitions. The receipt's exact-route members (`shear_origin: derived_e_nu`, `geometry.route: exact`) are already present.

**Producer (lane P):**
- `retained_product.rs`: exact capture and preparation (lift the `is_exact` refusal for this route only);
- `retained_wire.rs`: identity and profile per route, the definition id, material and geometry specifics;
- `lib.rs`: route selection.

**Readers (three lanes):** S-G1's `<physics-retained>` branch (D2 §4.9.1, §4.9.3):
- G0: identity, table and definition;
- G7: physics-1's base validator (`contract_evidence.exact_cases`, pressure, connector);
- G8: physics-source-1's `actual_materials` through S-C;
- G5b: exact section truth.

**Carriers and T6S (lane T):**
- successor branches in `results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json` and `stress_neutral_export.v0.3.schema.json` (D2 §4.9.6);
- the exhaustive output policy's entry (`outputPolicy.ts`: semantic table, `contract_evidence` kind, transport validator). Admit the two panels at eligible standing with a Rust golden for the exact successor's derivative, as T6S-2 did (decision 21), because B8's checklist item 4 needs it;
- the TS `SourceContract` union.

**Statics:** the new table and the definition JSON join `REVIEWED_INPUTS` (14 → 16 in `PP/build_identity.rs` and `PP/build.rs`), so they are part of the registration (§3.2).

**Witnesses and corpus:**
- **B3-W probes,** on main's build through the ordinary route:
  - the verdict and physics-source-1's selection for the milestone authored as 0.3.0 exact (skewed, so exact-block is expected not to select);
  - n05 and n06 (expected to select, so coexistence holds).
- **After B3b-P:** the exact successor pinned in both modes; coexistence pins on n05 and n06; 07o entries for G0, G7, G8 and G5b.

**Write sets, by file:**

| Lane | Files |
|---|---|
| A | `PP/retained_memory.rs` (D1.3, D1.5 for the exact route), `PP/retained_memory_law_tests.rs`, the runner's admission test if an oracle moves |
| P | `PP/lib.rs` (route selection in the retained section), `PP/retained_product.rs` (exact capture and preparation), `PP/retained_wire.rs` (identity, profile, definition, material and geometry), `PP/retained_facade_tests.rs`, `PP/retained_product_tests.rs`; new `P/fixtures/results/retained_precision_exact_successor_{sparse_interactive,dense_scrutiny}.json` |
| K (only if B3-D requires it) | `FK/product_certificate.rs` (`AnnulusPreparationVersion`, `prepare_product_annulus`) and its tests |
| Statics | new `P/fixtures/results/semantic_contract_v0_3_physics_retained_1.json`; new `P/fixtures/results/retained_precision_prepared_exact_v1.json` (the definition; the name follows B3-D); SCHEMA (`definition_id`); `PP/build_identity.rs` and `P/core/product_physics/build.rs` (`REVIEWED_INPUTS` 14 → 16) |
| RS, PY, TS | the three readers and their contract tests (the `<physics-retained>` branch); TS `numericalResultQuality.ts` and `resultSemantics.ts` if the route list lives there |
| T | `P/schemas/results.v0.3.schema.yaml`, `analysis_run.v0.3.schema.json`, `stress_neutral_export.v0.3.schema.json`; `P/apps/desktop/src/features/results/outputPolicy.ts` and its test; the T6S golden and its RE test |
| Corpus | CORPUS (07o entries) |

**Memory:** unpriced. B3-S rebinds the D1.3 zero rules for the exact route (§0 item 9) in I82's chain and prices the exact route at C = 3. If it does not fit within the B1 M with 5 %, ROOT chooses route caps (for example c ≤ 2 on the exact route) or raises M up to 12 GiB. Above 12 GiB it is the owner's.

**Needs from B1:** as B2, plus T-3 (c)'s n-case coexistence.

## 2. Slices, owners and estimates

### 2.1 Lanes and worktrees

At most **three implementers** at once; **one heavy job** at a time under `WT/guard/cargo_job.lock` (cargo through `WT/tools/t3_cargo.sh`; heavy pytest and vitest, and direct test binaries, under `lockf`); one wait per job. Each lane that builds PP or RE has its own worktree and branch from the common base, and lanes are split by file (PLAN_v2's SF-1). ROOT integrates with `--no-ff` at the J points. Nothing reaches NUM before SQ2 (PLAN_v2 decision 17's rule).

| Lane | Owner role | Worktree / branch | Slices |
|---|---|---|---|
| D | I-D (designers, fresh) | records only | B2-C, B2-KD, B3-D |
| W | I-W (fresh) | disposable archives | B2-W, B3-W, B3-S (Python only) |
| K | I-K | `WT/b2-k` / `codex/piping-t3-b2-k-<date>` from J0 | B2-K, and B3-K if B3-D requires it |
| P | I-P | `WT/b2` / `codex/piping-t3-b2-<date>` from main after PR-B1 (J0) | B3b-P, then B2-P (B2a, then B2b) |
| A | I-A | `WT/b2-a` from J0 | B3a-A, B2-A, B3b-A; the statics; later SQ2 |
| RS, PY, TS | I-RS, I-PY, I-TS | `WT/b2-r`, `WT/b2-p`, `WT/b2-t` from J0 | B3a, then B3b, then B2, per language; I-TS also carries B3b-T and the T6S entry |
| S | I-PY (one corpus writer) | `WT/b2` after J5 | SC2: corpus 07o; then I-RS and I-TS update pins |
| G, B, K2 | fresh | archives and records | SG2, SB2, SK2 |

### 2.2 Slices

Estimates are agent hours, without repair rounds (§2.5 adds them).

| Slice | Content | Depends on | Estimate |
|---|---|---|---|
| **B2-C** | §1.2.2's contract text, decisions 5–10 as selected, the SCHEMA `$defs` text, PTABLE's revised text, the D1.4 text, the gate and code placement | ROOT's rulings on this plan | 8–12 |
| **B2-KD** | §1.2.3's kernel design: API signatures, the combination source view (ledger and prescription terms by `(operand, term)`), owner generalization, the test list. No code | — | 5–8 |
| **B3-D** | §1.4's contract 1–5: the definition (draft DEFINITION JSON), table text, row-recipe coverage on the exact route, the reader branch, carriers, the FK question | — | 10–16 |
| **B3-S** | Exact-route pricing at C = 3 with I82's chain (Python), D1.3's zero rules rebound | — | 3–5 |
| **B2-W** | One-case proxies: W-CB1's `A + B` loads at the caps; W-CB3's candidate `not_required` cases; verdicts and native terminals in both modes (I81's method; cargo under the lock) | A free slot and the host | 3–5 |
| **B3-W** | Exact-route verdicts and physics-source-1's selection (milestone-as-exact, n05, n06), both modes | Same | 3–5 |
| **B3a-A** | D1.3 for the legacy contract; law tests; the oracles | J0 | 2–3 |
| **B2-A** | D1.4 (C_eq, terms); `cap_rows`; G-C facts; law tests; mutants (C_eq bound, term bound, combination counting) | J0, B2-C | 4–7 |
| **B3b-A** | D1.3 and D1.5 for the exact route; law tests | J0, B3-D | 2–3 |
| **Statics** | SCHEMA's `$defs`; PTABLE's revision; the physics-retained-1 table; the definition JSON; `REVIEWED_INPUTS` 14 → 16 (with B3b) | B2-C, B3-D | 3–5 |
| **B2-K** | §1.2.3 implemented with tests; the old-API byte identity | B2-KD accepted | 12–20 |
| **B3-K** | An exact annulus version, only if B3-D requires one | B3-D | 0–8 |
| **B3b-P** | §1.4's producer, pins and coexistence pins | B3-D, B3b-A (J1) | 12–18 |
| **B2-P** | §1.2.4's producer; W-CB1 to W-CB5 pins (private driver first, then Direct once admission is in, PLAN_v2 A1-S-2's pattern); hooks; mutants (disposition rule, operand-source mapping, imports, the R-COMB-1 producer side, failure set, ordinal mapping) | B2-K (J2), B2-A | 20–30 |
| **Readers, B3a + B3b** | RS 5–8, PY 4–6, TS 4–6 | B3-D; B3a-A | 13–20 |
| **Readers, B2** | RS 14–20, PY 9–13, TS 10–14 | B2-C (synthetic receipts first; producer bytes at SC2) | 33–47 |
| **B3b-T** | Carrier branches, the output-policy entry, the golden | B3-D; B3b-P's successor | 5–8 |
| **SC2** | Corpus 07o (§1.2.6, §1.3, §1.4); pins | RV-P2's round 2 settled; readers merged | 8–12 |
| **SQ2** | §3: G5 (combination and exact-route loops, the explicit-row rule), G6, M, witnesses, challenge, RSS and time | Code freeze (J6) | 22–32 |
| **SG2, SB2, SK2** | Direct-entry gates; Pass B; the package | SQ2 | 10–15 |

### 2.3 Phases and integration points

1. **Phase 0, now to PR-B1's merge (documents and probes; no maintained edit).**
   - Designers take B2-C, B2-KD and B3-D as slots free; RV-C, RV-K (design) and RV-D review.
   - B3-S runs (Python, no lock).
   - B2-W and B3-W run when the host and a slot allow, behind B1's heavy jobs.
   - ROOT rules on this plan's decisions and selects the designs.
   - **J0:** after PR-B1 merges and NUM absorbs main, ROOT cuts `b2` from main.
2. **Phase 1. Implementers: I-K (B2-K), I-A (B3a-A, then B2-A and B3b-A, then the statics), I-RS (B3a, then B3b RS).**
   - **J1:** lane A's admission merged into `b2` after RV-Q2 round 1 (expressions).
   - I-P starts B3b-P when I-A's slot frees, or earlier if I-RS has not started.
3. **Phase 2. Implementers: I-P (B3b-P, then B2-P), I-K until B2-K is confirmed, then I-PY.**
   - **J2:** B2-K merged after RV-K round 2 (code) and its repairs.
   - **J3:** B3b-P merged after RV-P2 round 1. B2-P starts on J2 and J3.
4. **Phase 3. Implementers: I-P (B2-P), I-PY (B3 then B2), I-TS (B3 then B2, B3b-T); I-RS returns for B2's RS when a slot frees.**
   - **J4:** readers (B3a, B3b) merged after RV-R2.
5. **Phase 4.**
   - RV-P2 round 2 on B2-P, then its repairs.
   - **J5:** B2-P merged. B2's readers merged after RV-R2.
   - SC2 writes 07o; I-RS and I-TS update pins.
   - **J6, the code freeze.**
6. **Phase 5.** SQ2 (I-A): G5 (with G5-early in phase 4) → R6a′ (provisional M) → G6, witnesses, challenge, RSS and time → RV-Q2 with its rounds → R6b′ (final M) → ROOT applies the registration → **R9′:** ROOT reads RSS_TIME against the owner's target machines and reports it.
7. **Phase 6.** `b2` merges into NUM, with no other unmerged product slice. Then: the full suite and the src-tauri suite; T9 and both-entry part 1; SG2; PR-B2 cut; SB2 Pass B (RV-Q2 confirms); SK2's package; RV-X2's complete-diff review with its repairs; CI with the full-SHA dispatch; GEN-8; an exact-head DEC-025 with the src-tauri suite; R7′; the merge (`--merge --match-head-commit`).

### 2.4 The critical path

1. B3-D and its review (phase 0).
2. B3b-A, then B3b-P (12–18 h).
3. RV-P2 round 1.
4. B2-P (20–30 h, on B2-K, which runs alongside from J0).
5. RV-P2 round 2 and its repairs.
6. SC2 (8–12 h).
7. SQ2 (22–32 h) with RV-Q2 and its rounds.
8. The PR stage (SB2, SK2, RV-X2, the gates).

That is about 75–110 h of serial agent work and 30–45 h of serial review. **B2-K is near-critical:** if the combination certificate needs a second design round, B2-P waits on it.

### 2.5 Estimates

| | Agent (h) | Review (h) |
|---|---|---|
| **Phase 0 designs and probes:** B2-C, B2-KD, B3-D, B3-S, B2-W, B3-W | 32–51 | RV-C 3–5; RV-K design 3–5; RV-D 5–8 |
| **Kernel:** B2-K (+ B3-K if needed) | 12–28 | RV-K code 5–8 |
| **Admission and statics:** B3a-A, B2-A, B3b-A, statics | 11–18 | in RV-Q2 |
| **Producer:** B3b-P, B2-P | 32–48 | RV-P2: 12–18 (two rounds with confirmations) |
| **Readers:** B3a + B3b, B2 | 46–67 | RV-R2: 10–15 |
| **Carriers and corpus:** B3b-T, SC2 | 13–20 | in RV-R2 |
| **Qualification and packaging:** SQ2, SG2, SB2, SK2 | 32–47 | RV-Q2: 14–22 (two rounds) |
| **Repair rounds** (two on RV-Q2's lane and two on RV-K's, as U4's G5–G7 and I42–I44 needed; one each elsewhere) | 20–35 | in the above |
| **PR-B2's complete-diff review** | — | RV-X2: 7–11 |
| **Total** | **about 195–300** | **about 60–90** |

- **ROOT:** rulings, six integration points, the registration, R9′, the gates and the merge: 15–22 h.
- **Machine:** the full suite and src-tauri (candidate and head), T9, RSS and time on both routes: 8–12 h.
- **Elapsed:** about 10–15 working sessions after PR-B1 merges, given phase 0 done by then.
- **Calibration.** I61 put B2 and B3 at 25–40 h each. I84 put B1 at 88–137 h against I61's 30–45 h, a factor of about 3, and SP's re-estimate added about 20 %. U8 and T6S finished faster than planned (RV107 N-10). The new costs here are the kernel certificate, B3b's definition and the readers' combination validation.

## 3. Re-qualification at PR-B2

### 3.1 What PR-B2 re-qualifies once

| Obligation | What it must show | Produced by / reviewed by |
|---|---|---|
| **G5** | TEXT complete on the frozen code: I-A writes loop rules for every new loop (combination stage, operand preparation, combination Calls, the readers' combination gates, the exact route). It rebinds I65's D1.4 zero rules (`combination|operand_id`, `append_combination`, the reader's `for gate in gates`) and D1.3 zero rules (§0 item 9) to their real bounds. No multi-member SCC; a converged D fixpoint; the identifier audit enforced. The regenerated forms with `ESTIMATES` = 0. **A control run at c = 1, z = 0 and one at S3 (c = 3, z = 0) equal B1's records, or differ only by named B2/B3 loops**. E+R at C_eq = 3 with a combination, and on the exact route, within the 5 % budget or explained | SQ2 / RV-Q2 |
| **The identifier residual** | **The explicit-row rule** (decision 11): every non-candidate text site on the B2 graph carries an explicit row with its class, and Pass B fails closed on an unrowed site. The 410 non-candidates and B1's set get rows, reviewed by type | SQ2 / RV-Q2 |
| **M** | D-7: the smallest 256 MiB step with E_mov,max + R ≤ 0.9 M and ≥ 5 % text budget in both modes and both routes, ≤ 12 GiB. Expected: B1's M (§3.3) | ROOT at R6a′ and R6b′ |
| **G6** | `admission_bound` at M − R ± 1; the pure maximum with each phase largest; the 0.9 M rule; `registration.diff` applied in a scratch copy with PP's suite passing (apart from the known Mac `t13`); `QUAL_B2.md` | SQ2 / RV-Q2 |
| **S1 witnesses** (R/16 = 4 MiB; one entry per mode per process) | B1's set unchanged; W-CB1 (C_eq = 3 at D1's caps, cap-maximal, every provenance escaped, raw depth 16, `assess` in-domain); W-CB2; the B3a and B3b successors; W-CB1 at 1 MiB as headroom; the deepest chain re-derived (expected 40 frames: a loop adds no recursion) | SQ2 / RV-Q2 |
| **The challenge** | Each run's counting-allocator peak ≤ its furthest phase's bound (PLAN_v2 §3.5's mapping), for W-CB1, W-CB2 and the exact successor, with the milestone's peak still within bound | SQ2 / RV-Q2 |
| **Peak RSS and run time** | B1's method (PLAN_v2 §3.6) on W-CB1 and on the exact route's cap-maximal input, both modes, three repetitions, with the phase-coverage, host and non-claim statements; then R9′ | SQ2 / RV-Q2; ROOT |
| **Pass B** | On PR-B2's head: tree, entry equal to the applied registration, statics, line map, TEXT and forms equal to SQ2's, every production hunk classified, the explicit rows complete, witnesses and challenge equal | SB2 / RV-Q2 |
| **The full 40-manifest suite and src-tauri** | Before the freeze and on the exact head (PLAN_v2 §3.9) | ROOT |
| **T9 and both-entry, part 1** | Byte-identical (the observer call sites in `lib.rs`'s ordinary bodies) | ROOT |
| **Direct-entry gates (SG2)** | Pressure (a non-empty region still refused); coexistence on n05, n06 and their exact-route forms; the registered and Stale sweep, every registered row explained (combinations, the legacy contract, the exact route, R-COMB-1), Stale byte-identical to base; the caller scan | SG2; RV-P2 reads the sweep delta |
| **T6S** | The T6S suites in the full suite; I74 §4.3's B2 and B3 notes met; a changed disclosure meaning goes back to ROOT | Full suite; SK2 records it |

### 3.2 How B2's and B3's changes enter one registration

**One `registration.diff`, prepared at SQ2 on the frozen code (J6), applied by ROOT after RV-Q2.** It carries:
1. **The generated profile:** one form set, priced at C_eq = 3, whose phase maxima bound both routes. If B3-S shows the exact route needs different counts, `cap_priced_maximum` takes the maximum over route-specific forms, as STUDY §4.1 described for tiers.
2. **`threshold_bytes`** (M), selected at R6b′.
3. **The cap constants:** `LOAD_CASES` = 3 (unchanged), the new case-equivalent and term caps, and the D1.3 and D1.5 route clauses. Like B1's (RV107 A1-N-11), these land at their slices (J1), and SQ2 re-pins the profile-dependent values.
4. **`reviewed_inputs`:** SCHEMA's new hash, PTABLE's new hash, and the two new statics (the physics-retained-1 table and the exact definition), with `REVIEWED_INPUTS` grown from 14 to 16 in `build_identity.rs` and `build.rs`.
5. **`READER_LAYOUTS`:** unchanged; a change is a stop.

The identity text (dev/test) is unchanged. **B3a's, B3b's and B2's code must all be frozen before SQ2's G5:** one freeze, one G5, one G6, one Pass B. B7 then registers the release identity once on top (PLAN §2.2).

### 3.3 The memory picture at M ≤ 12 GiB with combinations

**The method** (`_run_records/b2_bracket.py`, `b2_mid.py`; outputs beside them).
- **The tool:** I82's evaluator, unchanged (sha256 `c404e8db…c74b`), with I72's in-build atoms (`cbf34c52…313d`), on I82's committed profile trees (sums in §9).
- **The hybrid tree.** For (c, z), each form comes from I82's tree at c or at c + z, by scope:
  - **always c:** seeds and observations (T11), the raw request, T19, the statics, T07, and the X branch (exact-block cannot select with combinations);
  - **always c + z:** T13–T15 (a Run, a frozen candidate and a trace per retained combination), and every envelope and receipt form (STAGED, T16, T17, SUCC, BODY), each combination priced at P_final rows and one case-like receipt entry;
  - **bracketed:** O_base, TAV_W, T12 and the notice. LOW takes c for these (a combination has no ordinary solve, no notice, and its operand preparations stay within a = c). HIGH takes c + z, plus a ledger surcharge for the combined ledger's h·l terms (T12 is only 1.57 MB per case, so the surcharge is small).
- **MID** replaces the bracket for the two large forms:
  - TAV_W grows by 0.651 of a case's increment. That is the share of I65's c = 1 TEXT bytes in W1-owner, serializer, precommit-reader and preview-row sites (1,022,212,240 of 1,570,041,862 B), which a retained combination repeats.
  - O_base grows only by its envelope-row atoms.
- **LOW and MID are estimates, not bounds.** HIGH is the conservative price, apart from the combination-specific TEXT sites that today's rules zero (source refs per combination row, about h·1,024 B per row: some MB per combination). G5 prices those.

**Results (D1's model caps, a = c; E+R in bytes; 5 % = the smallest 256 MiB step with a 5 % text budget):**

| c | z | Dense LOW / MID / HIGH | Sparse LOW / MID / HIGH | 5 % M, LOW / MID / HIGH |
|---|---|---|---|---|
| 1 | 1 | 5,046,259,089 / 5,991,518,154 / 6,592,396,996 | 5,026,548,641 / 5,971,807,706 / 6,552,976,100 | 5.5 / 6.5 / 7.0 GiB |
| 2 | 1 | 8,150,366,023 / 9,127,476,129 / 9,749,149,984 | 8,110,945,127 / 9,088,055,233 / 9,690,018,640 | 8.75 / 9.75 / 10.5 GiB |
| 1 | 2 | 6,604,085,788 / 8,526,454,959 / 9,747,441,022 | 6,584,375,340 / 8,506,744,511 / 9,688,309,678 | 7.0 / 9.0 / 10.5 GiB |
| **3** | **1** | **11,416,844,017 / 12,427,112,010 / 13,063,805,472** | 11,357,712,673 / 12,367,980,666 / 12,984,963,680 | **12.25 / 13.25 / 14.0 GiB** |
| 2 | 2 | 9,819,484,362 / 11,806,862,462 / 13,063,663,144 | 9,780,063,466 / 11,767,441,566 / 12,984,821,352 | 10.5 / 12.5 / 14.0 GiB |
| 3 | 3 | 16.62 GB / — / 21.73 GB | 16.56 GB / — / 21.61 GB | 17.5 / — / 23.0 GiB |

- **The 0.9 × 12 GiB budget** is 11,596,411,699 B. W3 binds at every point.
- **A combination at D1's caps costs** 2,679,386,332 B dense at MID (MID at c = 3, z = 1, minus S3's 9,747,725,678 B), against 3,313,088,854 B for a fourth case: about 0.81 of a case.

**What this means for B2's cap shape:**
- **Three cases plus one combination at D1's caps does not fit within 12 GiB with the 5 % margin standard under any of the three estimates.** LOW leaves 179,567,682 B, only 4.3 % of TAV_W.
- **C_eq ≤ 3 fits under HIGH at B1's expected M (10.5 GiB):** 9,749,149,984 B dense, 9.5 % budget. That is S3's price to within 1.5 MB, because HIGH prices a combination as a case. **This is the recommended shape** (decision 10). It needs no new tier, no new form set and no new M, and leaves the 1.5 GiB below 12 GiB to absorb G5's real combination and exact-route text.
- **The owner's view, with no machine claim:**

  | Shape | Priced worst-case heap of one W1 invocation | Against 16 GiB / 32 GiB |
  |---|---|---|
  | C_eq = 3 (HIGH) | 9.02 GiB | 56 % / 28 %, as S3 |
  | Three cases plus a combination at D1's caps, MID | 11.51 GiB | 72 % / 36 % |
  | Three cases plus a combination at D1's caps, HIGH | 12.10 GiB | 76 % / 38 % |

- **Second-tier options, if ROOT wants C_eq = 4 when combinations are present** (each priced as four case-equivalents, the conservative HIGH, from I82's points):

  | Tier 2 (C_eq ≤ 4) | Dense E+R | 5 % M |
  |---|---|---|
  | m ≤ 20 | 10,512,635,788 | 11.25 GiB |
  | l ≤ 32 | 11,213,200,292 | 12.0 GiB (7.5 %) |
  | m ≤ 24 | 11,348,074,996 | 12.0 GiB (5.3 %) |
  | n = m = g ≤ 20, l 128 | 9,027,966,167 | 9.75 GiB |
  | n = m = g ≤ 16, l 128 | 7,917,586,207 | 8.5 GiB |
  | n = m = g ≤ 12, l 64 | 5,358,966,100 | 5.75 GiB |

  Each adds a tier: one more form set, tier selection, per-tier G-B and G-C, and per-tier witnesses (STUDY §4.1's costs: about +8–10 h agent and +2–3 h review). I recommend deciding this after PR-B2's G5 gives the real combination price and R9′ gives the measured peaks.
- **B3b is unpriced** (B3-S). B3a is expected to be unchanged.

## 4. B4

**Not needed for B2 or B3.** B2 fits under B1's M with C_eq ≤ 3, and B3a's price is expected unchanged. B3b's price may force route caps, which is a B3 decision, not growth.

**What B4 would be for:** product reach. D1's caps (32 nodes, members and supports, 128 nodal loads per case) and C_eq ≤ 3 are small for real piping, and W1a admits nodal loads only. At 12 GiB, S3 already uses 10.5 GiB. **Growth needs either cheaper pricing or cheaper code,** not more M.

**The study's question** (3–4 h, Python and reading, if ROOT or the owner wants it): within M ≤ 12 GiB, the owner's target machines and B1's measured RSS and time, which growth of (n, m, g, l, C_eq) can be admitted, and at what cost, through:
- **(a) I82's untaken credits,** each needing its own review: sequential per-case ordinary transients, group-shared slot builds, G3's phase-aware span (−0.085 M in reserve), and the c² sites;
- **(b) code that lowers W3's binding terms:** TAV_W, and T16's P2 stage (the body `json!` plus the publication hash, 37 % of W3 at P1 and 3.6 GB at S3), for example by streaming the hashes;
- **(c) a c = 1 tier at larger model caps?**

The output: priced options with reach, M, text budget and measured-peak ratio.

**Timing** (decision 15): the owner's order has "B4 if ruled → PR-B2". If B4 is wanted inside F2a, it must be ruled before PR-B2's code freeze. Later growth re-qualifies both registered identities after B7. I recommend ROOT rules "not now" and puts the question to the owner with R9's report. If the owner wants more reach before activation, that is an order question (§7).

## 5. Decisions

None below is on the owner-held list unless marked **Owner**; for those I prepare and do not decide.

| # | Decision | Recommendation | Decider |
|---|---|---|---|
| 1 | B2's boundary | **One unit, two internal stages** (B2a, B2b), one contract, one 07o, one PR. Not two PRs (a re-qualification for no gain; P3's interim never reaches main) | ROOT |
| 2 | B3's boundary | **B3a (legacy contract, preview successor) and B3b (exact, `physics-retained-1`)**, both in PR-B2 | ROOT |
| 3 | Order | **Phase 0 now** (B2-C, B2-KD, B3-D with reviews; B3-S; B2-W and B3-W as the host allows). **After PR-B1:** B3a's admission first; B3b-P on lane P while B2-K runs on lane K; then B2-P; one SQ2 and one PR-B2 (§2.3). Alternative: B2 before B3b on lane P, which waits on B2-K | ROOT |
| 4 | Components | **Out of B2** (D1.4 and D1.8 unchanged): the kernel has straight members only; components are W1c (DN §8) | ROOT |
| 5 | The combination trigger | **A mechanics combination is retained iff it passes T0R's gates and ≥ 1 operand is `selected` after T-9.** Otherwise: mechanics `ordinary`; subtraction and range `ordinary`; gated `base_withheld`. A successor then always has a selected case (D2 §4.9.2 unchanged). Alternative: ≥ 1 operand in A, which needs a GroupPrep from a common source and a successor rule for "only a combination selected" | ROOT |
| 6 | Operand sources | **As §1.2.1** (C3a rule 1 restated): selected → its solve; unavailable with a source → that source; `not_required` → one shared `OperandPreparation`; unavailable without a source → the combination `retained_unavailable` | ROOT |
| 7 | Combination failures and notices | **Per-combination `retained_unavailable`** for pre-source refusal, an unresolved or refused Run, a facade certificate failure, or an operand preparation refusal. **Whole-successor abandonment** for origin or capacity refusals, staging, serializer and precommit. **No N1 notice for a combination;** T-12 unchanged | ROOT |
| 8 | Standing of uncertified combination rows (R-COMB-1) | **Row-level `not_covered`** (withheld from binding, D-U6-2's disclosure, values unchanged) for every row of an `ordinary` combination referencing a case that is not `not_required`, and of a `retained_unavailable` combination. Alternative: envelope `needs_recompute` (simpler, coarser). Re-deriving subtraction values from published rows needs its own recipe (C1 §5) and is not proposed. **The owner is informed:** it shapes what B8 shows, though successors are not public before B8 | ROOT |
| 9 | PTABLE | **Revise in place** (same contract id): the scope clause, plus RV78-N1's four policy bindings at the same revision. This amends RR's "not revised for it" only because the hash moves anyway. The 12-file cascade rides PR-B2. Alternative: a new successor identity (heavier; two preview successors) | ROOT |
| 10 | B2's cap shape | **One tier: c ≤ 3, C_eq = c + z ≤ 3** (every combination of any basis counts), terms ≤ 3 per mechanics combination and ≤ 3 range operands; D1's model caps; M by D-7 at PR-B2 (expected B1's). Alternatives: a second tier (§3.3, ROOT within 12 GiB), or C_eq = 4 at D1's caps (decision 22, the owner) | ROOT |
| 11 | The identifier residual (QUAL §11; PLAN_v2 §3.7) | **Adopt the explicit-row rule at PR-B2.** Alternative: re-run RV87's by-type sweep over the new non-candidates, and again at B7, F2b, F3 and S-I2 | ROOT |
| 12 | B3a and D1.7 | **D1.7 unchanged:** zero pressure means no pressure load | ROOT |
| 13 | B3b's definition and table | **`RP-PREPARED-EXACT-DUAL-v1`** (name and H domain reserved at B3-D after a collision check); the table binds RV78-N1's policies from version 1; physics-1's table untouched (DESIGN decision 14) | ROOT |
| 14 | B3b's coexistence | **T-3 (c) unchanged:** physics-source-1 selecting any case means the exact ordinary bytes | ROOT |
| 15 | B4 | **Not now.** Ask the owner with R9's report whether reach matters before activation. If B4 is wanted inside F2a, rule it before PR-B2's freeze; the study question is §4 | ROOT (owner consulted) |
| 16 | Kernel scope | **B2-K is planned FK scope** (§1.2.3), with a fresh numerical reviewer (RV-K) for design and code. Other FK changes stay stops | ROOT |
| 17 | Phase 0 | **Start now**, documents and Python only. B2-W and B3-W cargo jobs only under the lock, behind B1's heavy jobs. No B2 code in NUM before PR-B1 merges (NUM sequencing) | ROOT |
| 18 | Reviewers | **Fresh:** RV-C (contract), RV-K (kernel, both rounds), RV-D (B3 design, RV69-style), RV-P2 (producer, ledger), RV-R2 (readers and 07o), RV-Q2 (admission and qualification), RV-X2 (complete diff) | ROOT |
| 19 | PR-B2's gate set | **PR-B1's set** (RR "T3's gate set…", items 1–7, with the src-tauri suite, RV-X2, Pass B, T9 and both-entry, the Direct gates) | ROOT |
| 20 | 07o | **One writer, append-only after 07n;** `base_withheld` and `pre_source_refusal` synthetic and labelled; first failures fixed from the three readers, differences declared only by ruling | ROOT |
| 21 | T6S consistency | **B2:** the successor path's combination basis references in the stress-neutral packet and the AnalysisRun; the T6S suites. **B3b:** the output-policy entry admitting the two panels at eligible standing, with a Rust golden for the exact successor's derivative | ROOT |
| 22 | **Decision 17, prepared:** M above 12 GiB, or a supported-machine statement | **Not needed if decision 10 is adopted.** It is needed only for three cases plus a combination at D1's caps: about 13.25 GiB (MID) to 14.0 GiB (HIGH) with 5 %; priced worst-case heap 11.5–12.1 GiB, 72–76 % of a 16 GiB machine. B1's R9 measurement should come first | **Owner** |
| 23 | **Decision 18, prepared:** R-2 for B8 | **Not needed for B2 or B3.** Note: R-COMB-1 adds `not_covered` combination rows, so more rows carry withheld unit witnesses in stress-neutral packages | **Owner** |
| 24 | **Decision 19, prepared:** the native-app witnesses | **Unchanged.** Note: B3b adds `physics-retained-1` to B8's native Current witness (its two panels) | **Owner** |
| 25 | Witnesses | **As §1.2.6:** W-CB2 (two-body A + B at Ceiling) is the mixed `retained_unavailable` pin; W-CB1 the cap-maximal selected one; W-CB3 under a stop rule (§6) | ROOT |
| 26 | B3b's memory | **B3-S before B3b-A.** If the exact route does not fit at B1's M with 5 %, choose route caps (for example c ≤ 2 on the exact route) or raise M within 12 GiB. Above 12 GiB, the owner | ROOT |
| 27 | Packaging contingency | **Keep one PR-B2.** If B3-D or B3-K shows heavy FK or certificate work, ROOT may split off PR-B3 (B3b) at the cost of one more re-qualification (about 20–30 h agent plus gates, B1's calibration). The owner's order (breadth before B7) is unaffected | ROOT |

## 6. Risks and stop rules

1. **The combination certificate (B2-KD/B2-K).**
   - **Stop:** RV-K finds the combination source view unsound, or it needs a change to the stop rule, schedule, prices or ceiling.
   - **Then ROOT chooses between:**
     - a second design round;
     - shipping B2 with every mechanics combination `ordinary` (coverage only), a declared narrowing of F2a's promised combination route. RR:7051 makes that an owner-facing change of scope.
2. **Memory.** The G5 price at C_eq = 3 exceeds the 5 % budget at B1's M.
   - **The ladder:** 256 MiB steps up to 12 GiB (ROOT); then C_eq ≤ 2 for combination-bearing invocations; then the owner.
   - G5-early (phase 4) gives warning.
3. **B3b's price** (B3-S, then G5). Route caps per decision 26. Above 12 GiB, the owner.
4. **B3b's definition.** If B3-D finds exact-route rows without a recipe, they are classed `not_covered` (C1's default) only if RV-D agrees no reliance is lost against physics-1's base. Otherwise stop. An FK need beyond an annulus version is a stop.
5. **Byte identity.** Any change to a c = 1 or B1 multi-case successor byte, or to an exact-block-selected publication, is a stop. The PTABLE cascade changes hashes and constants, never envelope bytes.
6. **The re-pin census.** Any changed 07n outcome in a reader lane is a stop (R5).
7. **Witnesses.**
   - W-CB3: if no candidate in at most 6 variants gives a `not_required` operand beside a selecting combination, return. The mixed-selected shape then rests on the kernel test (C01) plus a labelled synthetic corpus base (ROOT rules).
   - W-CB1: if `A + B` does not select at the caps, its asserted outcome stands, and RSS_TIME states the furthest phase.
8. **Batch and proxy outcomes.** A combination's product outcome that differs from its one-case proxy is a finding with its cause (PLAN_v2 N-16's rule).
9. **Kernel compatibility.** The old `solve` API's bytes or any existing FK test outcome changes: stop.
10. **Source-text guards.** As PLAN_v2 §1 (s11f's rule 8, the admission section guard, the serializer's banned reads). A weakened assertion is a stop; a new rule-8 site gets a reviewed row.
11. **Integration conflicts.** Lanes are split by file. Any conflict outside a named seam is a stop at that J point.
12. **B1 moves.** If PR-B1 lands with a different C, M or transaction shape than PLAN_v2's, ROOT re-reads §1.2.8 and §3.3 before J0. A change to S3 is a re-plan of §3.3.
13. **Main's movement.** PLAN §4's absorption procedure before each J point and before the PR cut.
14. **The host.** One heavy job at a time; Python pricing outside the lock; every wait ends when its job's process has gone.

## 7. What would change the owner's F2a order

**Nothing found here changes it:** U8 → breadth (B1, then B2/B3, PR-B2) → B7 → B8 → S-I2, F2b, F3. B4 is not needed. These would raise an order question for the owner:
1. **B3b proves FK-heavy or unaffordable** (decision 26 or 27; risk 4). The option then is to activate on the preview route first and move B3b after B8. That changes the order, so it is the owner's.
2. **The owner wants three cases plus a combination at D1's caps.** That is decision 22 (M above 12 GiB), not an order change, but it moves the 16 GB floor (72–76 % of a 16 GiB machine, priced).
3. **The owner judges D1-scale activation not worth it,** after R9's measured peaks and this plan's reach statement. Then B4, or a code change that lowers W3, would move ahead of B7, which is an order change.
4. **R-COMB-1 (decision 8)** changes what activation shows (withheld combination rows). That is not an order change, but the owner should know before B8.

## 8. Carried notes, placed

| Note | Placed in |
|---|---|
| C3a's reserved names (RR "B0 selected on DESIGN_v2…") | B2-C (§1.2.2) |
| RV78-N1's policies (RR "I86's SW probe accepted…", ruling 1) | B3-D's table from version 1; also PTABLE's revision (decision 9) |
| The explicit-row rule (PLAN_v2 §3.7) and RV87's sweep (QUAL §11) | SQ2 (decision 11) |
| I74 §4.3: B2's combination basis references; B3's output-policy entry | B2's TS lane and B3b-T (decision 21) |
| DESIGN §5's N-8 (the `legacy_pressure_v1` G8 widening; the successor-list precedent) | B3a (§1.3); decision 13 |
| DESIGN §4.2's disclosed properties (a combination is not the factor-weighted sum of its operands' published rows; preparations are not shared) | B2-C restates them; RV-C checks them |
| PLAN_v2 R9 (RSS and time to the owner) | R9′ at SQ2; decision 15's question rides it |
| RR:7051 (no hidden solves, no fake readiness, no availability exception) | Decisions 5–8 and risk 1 |
| SC1 §2's "no silent disappearance" | Decision 7 (rows stay; diagnostics name the combination) |

## 9. What I read, execution record and limits

**Read** (sha256):

| Input | sha256 |
|---|---|
| The brief | `174550b0…0c8a` |
| `AGENTS.md`, `agents/AGENT_TASK.md`; the work graph's T3 section | — |
| PLAN (§1.3, §2, §5) | `f274a614…3def` |
| DESIGN (whole) | `5933b90b…1114` |
| PLAN_v2 (whole) | `c85786b7…9be0` |
| STUDY, ADD | `d8b18220…7188`, `7c155ceb…0ce2` |
| RV107's REVIEW (N-11) | `03c3111d…a426` |
| RV78 `carried_artefacts_01` REVIEW (N1) | `40c35ef4…5c35` |
| C1, C2 (§1–§4), C3 (§1), SC1 (whole) | `c8ab2318…67e3`, `923da0b9…0869`, `fd00d2c1…292e`, `28bc4dd6…4054` |
| DN (§4.3, §4.4, §8 and the combination passages) | `fb62ef4a…7a74` |
| D2 (§4.9 headings and the combination passages) | — |
| QUAL (§3, §4, §7, §10, §11); DOMAIN (whole) | `8edbf4b4…2c29`; `08a72dde…c9fd` |
| I74's PLAN (§1, §3–§6) | `0350c918…2ed9` |
| I65's ID_CLASS_AUDIT (residual); I30's ROUTING (§6); I31's, I42's, I43's and I44's combination passages | — |

**RR,** by section heading: "B0 selected on DESIGN_v2…", "I81's B1-0 probe verified…", "I82's study…", the three owner memory decisions, "I82's addendum…", "R1…", "B1's PLAN_v2 accepted…", "I86's SW probe accepted…", the B1 sections through "I91's SR-PY repair verified…", "B2/B3's plan dispatched as I93…"; also RR:7030–7060, 8790–8836, 9450–9470, 10620–10720, 11850–11960.

**Code read at NUM `1d223f78be` (= main `2007709549`):**
- PP `lib.rs`: the model types, `source_eligible`, the ordinary case loop and combination append, `mechanics_producer_for_model`;
- PP `retained_product.rs` (the capture scope), `retained_memory.rs` (`family_clauses`, `RegisteredProfile`, `REGISTERED_PROFILES`), `build_identity.rs` (`REVIEWED_INPUTS`), `pressure_runtime.rs` (`is_exact`, `validate_profile`), `source_recovery.rs` (the namespace);
- FK `combine.rs`, `origins.rs` (`solve_cases`, `solve_combination`, `certify_product_case`, `product_case_owner`), `adaptive.rs` (`SourceBridgeViewIssue`), `source.rs` (types);
- SCHEMA (`combinations`, `Call`, `Owner`, `definition_id`, `geometry.route`, `shear_origin`); PTABLE's members; the exact-route request fixtures;
- the readers' namespace checks; T6S's `basisReference`.

**On B1's branches:** `retained_w1` and `w1_transaction` in `b1`'s `lib.rs`; item lists of `retained_product.rs` and `retained_wire.rs`; RS's combination checks on `b1-r`.

**Executed** (Python 3.13, VENV; `_run_records/RUN.md` has the commands with placeholders):
- `b2_bracket.py` and `b2_bracket_lib.py`: I82's `b1_eval.py` on I82's trees `d1_c1` (`148cc1c8…`), `d1_c2` (`0217e679…`), `d1_c3` (`0a1d5257…`), `d1_c4` (`3432c18e…`), `d1_c8` (`d4cfe710…`), the addendum's `d1_c6` (`bbba4d82…`), `c4_l32`, `c4_m24`, `c4_m20` and `c4_l48`, and `t_c4_k12_l64` (`7589bee1…`), with I72's law record (`cbf34c52…`);
- `b2_mid.py`: the same, plus I65's c = 1 `text_budget_W.caps.out.json` (`914b5c41…`).

Each script ran once, in seconds. Outputs: `b2_bracket.out.json` and `b2_mid.out.json`. The scratch copies in `WT/scratch/i93_b2b3_plan/` are disposable.

**Limits.**
- **Nothing was built or run beyond that Python.**
- **Every producer outcome is predicted, not established:** W-CB1's and W-CB3's selection, W-CB2's Ceiling (from case C), B3a's no-change, and the exact route's verdicts. Each is for the probes.
- **The memory figures are emulations on today's call graph** (STUDY §2.4's limits apply). MID's 0.651 text share is a file-level classification of I65's c = 1 TEXT, not B2's code. B3b is unpriced. G5 replaces all of them.
- **The kernel design is scoped from code and records,** not designed. B2-KD may find more FK change (risk 1).
- **B1 is moving.** §1.2.8 names what B2 needs from it, by role, not by final symbol.
- **Estimates are reading estimates** (§2.5's calibration).
