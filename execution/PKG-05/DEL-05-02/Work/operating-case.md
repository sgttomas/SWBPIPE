# Operating case with nonlinear supports, and the kernel side of skewed restraints

> **Status: adopted plan; truthfulness fixes (#1695), stage 2 (#1671) and slice 1a (#1752) built; slice 1b built on branch `claude/rt-a5-slice1b` (§6, candidate, not yet integrated; reviewed in r1 and its focused confirmation; its model-free G-1 reading, option A, enabled after that review); R-6 unbuilt and scheduled for v1.0 after 1b (2026-10-31, below); `minuend_final` unbuilt (§6.9 seam); superseded by any later Design text.**
> Committed from the solver lane's working note so that the adopted decisions are
> discoverable on main. The text is unchanged except that local absolute paths are
> shortened to `scratch/…`. Working-file references (`scratch/…` and
> `truthfulness_fixes.md`) are not retained.

**Known stale statements (main at `3be888a820`):**
- Bases read: stage 2 is already on main through #1671 (`ab30066ca9`), including R-a/R-b/R-d and R-c; “Both PRs are open” is historical.
- “The source item”: the cited MBF-import Design now marks §20.3 item 3 implemented and explicitly discusses the operating case in its gaps/ruling text (§20.3, lines 2384–2396); it no longer contains “no operating-case text”.
- “The importer currently” refuses `LS` with `DV`, `STIFF` and `MU`: main admits axis-aligned `LS(0,0)` with `DV`, optional positive `STIFF` or `STIFF=Rigid`, and zero/empty `MU`; nonzero friction and nonlinear limits remain refused (importer Design §12; `importer.py:2738–2800`).
- §0 “Truthfulness fixes” and §1.1 “Text mismatch”: #1695 (`ce5ea3da23`) already corrected K-D5's nonlinear-combination wording and CLM-012's friction/seed qualification; those defects are no longer present on main.
- §1.1 “Thermal plus sustained”: main admits PF imposed motion with nonlinear supports (R-a/R-b/R-d and R-c); `IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` is no longer the main refusal (`imposed_motion.rs:13–29`).

Design note for the SWBPIPE solver lane: research and design only, and no code was
changed. This is revision 2. It applies Codex's numerical scrutiny (round 1) and Agent
0's decisions of 2026-10-27, and adds two sections:
- §4, what establishes a derived O−S row;
- §5, the engineering meaning of "expansion".

Nothing here records or implies acceptance, issuance or lifecycle standing
(F-PIP-2/F-PIP-3).

**Bases read.**
- **M:** origin/main at `023ef18784`.
- **S2:** checkout `solver-w16s2` at `10e3f8836e` (branch
  `claude/piping-w16s2c-spring-base-20261027`). It carries #1654 (stage 2: R-a, R-b,
  R-d) and #1671 (R-c). Both PRs are open.
- **W16B:** `origin/claude/piping-w16b-ex-combinations-20261025` at `471ea86a7f`.

Paths are relative to `projects/chirality-piping/`. Line numbers refer to S2 unless
marked M. `preview_physics.rs` has the same line numbers on M and S2.

| Short name | Path |
|---|---|
| PP | `core/product_physics/src` |
| NI | `core/solver/nonlinear_integration/src` |
| NS | `core/solver/nonlinear_supports/src` |
| FK | `core/solver/frame_kernel/src` |
| LSUP | `core/solver/linear_supports/src` |
| LCA | `core/loads/load_case_algebra/src` |
| RE | `core/reporting/result_export/src` |

**The source item.** On M, DEL-17-04 `Work/caepipe-mbf-import.md` §20.3 item 3
(:2105) reads "rigid, axis-aligned guides and limit stops". It contains no
operating-case text; the operating case is Agent 0's framing of what the item needs.
The importer currently:
- refuses `LS` with `DV`, `STIFF` and `MU` (:1101);
- refuses skewed `K`/`KR` (:1102);
- maps thermal loads and anchor displacements to a thermal-only case
  `case:mbf-expansion-t1` (:590);
- creates no combinations (:598).

With nonlinear supports, the imported thermal-only case can differ from operating minus
sustained (§1.5, V-1 and V-2).

---

## 0. Decisions (Agent 0, 2026-10-27, under the owner's delegation)

| Item | Decision | Reason |
|---|---|---|
| Shape | Adopted as recommended. No new case kind. Slice 1a needs no model-document change. Slice 1b is the 0.9.0 member `combinations[].difference_basis = "operating_minus_sustained"` with gates G-1 and G-2. EX refuses the member by name. | PF already solves an operating case with its own converged support state, and already forms the subtraction (§1.1). What is missing is declared meaning, operand standing and consistency gates, not a new solve. |
| Truthfulness fixes | Done early: the K-D5 text that says nonlinear combinations are withheld, and CLM-012's seed-independence claim, which gains its friction non-uniqueness qualification (`truthfulness_fixes.md`). | Both texts currently state something the code does not do, or a claim that friction does not support. |
| R-1 | The sustained case keeps its own support status. Frozen operating status (`subtrahend_support_state`) remains a later item, flagged to the owner (§5). | Own status changes nothing that exists. Frozen status is a different comparison, and choosing it is a code-interpretation matter. |
| R-2 | Static, history-free friction (DEC-067) is kept, and V-3's consequence is disclosed in DEL-04-04. A load-history model is a later item. | A history model is DEL-04-04:2's ruling. Friction may admit several states. |
| R-3 | Gap certification (G2) is adopted as work, designed after 1a. | Without it, every limit-stop operand stays unqualified (R1). |
| R-4 | Nonlinear supports on EX are adopted as the main parity item. The design is staged after 1a and 1b, and includes friction with a one-way normal source and two-sided stops on one DOF. | Pressurized imports go to EX. |
| R-5 | Publish with standing; never withhold for operand standing. | Withholding would remove every gap case and hide evidence the user can inspect. |
| R-6 | The skewed rigid stop is an exact constraint, never a penalty spring. It uses the division-free nullspace basis, with joint reduction and reaction attribution for multiple constraints as part of the same design (§2.2-§2.4). | It keeps the authored constraint dᵀu = 0 exactly, with no stiffness value to choose. |
| R-7 | `direction` is any finite nonzero 3-vector, stored as authored and never normalized. The kernel uses the scale-free form with range-safe scaling. | No binary64 vector with two or more nonzero components is exactly unit length (§2.2). |
| Order | 1a is built after §8.29 (stage 2) and priced after #1678 (§8.31). 1b follows 0.9.0. The skew kernel follows #1678 and 0.9.0. | It keeps the pricing queue: §8.27 W-16b/c → §8.28 #1686 → §8.29 stage 2 → §8.30 corrections → §8.31 #1678. |
| `minuend_final` as a callable mechanism (agreed with DEL-04-11, 2026-10-31) | The frozen-status solve that `subtrahend_support_state: minuend_final` needs is built as one product function: solve a named case with its nonlinear supports held at a named other case's final status. The subtraction calls it, and so does hanger sizing's single derived solve (DEL-04-11 `weight_support_state: design_operating_final`), so neither keeps its own copy. Its design and certificate are RT-A5's Should slice. | One mechanism, two consumers; the held-constraint caveats of §5 option 2 apply to both. |
| Skew kernel scheduled (SWBPIPE Agent 0, 2026-10-31, under the owner's 2026-10-27 scope delegation) | The skew kernel (R-6 and R-7, §2) is in v1.0, carried by the load-cases lane after 1b. It is built on the division-free nullspace basis T of §2.2, which the dynamics lane (DEL-04-09) has adopted for modal, so static and modal restrain the same subspace; modal's skew refusal lifts with the static one. It changes numerical meaning, so it takes targeted references and independent review. Until it lands, 0.9.0's `SUPPORT_DIRECTION_SKEW_UNSUPPORTED` is an interim refusal with this scheduled remover, not a release limitation. | Skewed restraints are ordinary in real systems and a CAEPIPE-class tool has them; the reference system does not need one, so it is not on criterion 1's path, but omitting it would leave a common modelling gap. |
| §4 A, derived O−S rows | Option (i) is adopted, without (ii): a per-group combination certificate, B_D = RU(B_O + B_S + exact subtraction rounding). Cancellation stays honestly unestablished, range envelopes are deferred, and readers recompute B_D from the published operand bounds. The wording is "established within its stated bound", never "certified". Design after 1a, build before R-3, independent of R-4. It also bounds W-16b's linear EX combinations. | Each operand bound is against its own intended system, so operand-state agreement is not needed numerically (Codex r2). Without a certificate, O−S rows in nonlinear models stay inspection evidence even after G2. |
| §4 A, eligibility | Not decided. Agent 0 decides at the build design, which must state both consequences: (a) established derived rows count toward the document's numerical eligibility; (b) they do not, as today. Each must show the effect on a document with one cancellation-unestablished row. The certificate is built so either answer is a one-line policy switch, not a structural fork. | This changes today's rule that combination rows never count, which is a claims decision. |
| §5 B, the meaning of "expansion" | Option (3), computed as (1), is adopted. The product reports an "operating minus sustained difference" under the member's own name, with each case at its own state. The "expansion" label and its code-range meaning stay with the user's rule pack. The held-constraint option (2) remains R-1's remaining item for the owner, and can be added later as a second declared basis without renaming. | It makes the least product claim. The 0.9.0 member is already named `operating_minus_sustained`, and the engineering meaning is a design-basis choice the user's rule pack makes. |

---

## 1. Item 1: an operating case that solves the nonlinear supports

### 1.1 (a) What already exists

**Nonlinear supports, one axis DOF each.**
- NS `NonlinearSupportBehavior` is `{OneWay, Gap, LiftOff, Friction}` (`lib.rs:31-36`),
  and each support acts on one `dof: FrameDof` (:39-46).
- The classifier is `classify_support_state` (:581).
- Validation refuses:
  - rotational gaps and rotational friction (PP `validation.rs:101`, `:113`);
  - a rigid restraint overlapping a nonlinear DOF (:152-158).
- `initial_state` is required for each support (PP `lib.rs:8091`).

**How a PF case is solved.**
1. `solve_prepared_load_case` (PP :5031) assembles the case's ledger force, thermal
   eigenloads included (:12005-12020).
2. It calls `append_nonlinear_support_loop_results` (:5607-5622; the function is at
   :7169).
3. That function builds `NonlinearFrameSolveInput` (NI `lib.rs:240-266`) from:
   - the force;
   - the base prescribed map (R-a) and ground motion (R-b) (PP :7231-7236);
   - `initial_states: built.nonlinear_initial_states` (PP :7238).

Every case starts from the same model-level seed (NI :622, :1840). No state is carried
between cases.

**The linearized boundary.**
- Active one-way, lift-off and sticking states prescribe the support's ground (0 unless
  R-b moves it).
- An active gap prescribes u_d = g_s + σc = σ(c + σg_s).
- Inactive and sliding supports are free (NI :1943-2040).

**Confirmation and failure.**
- The K-D5 confirmation runs only on a qualified, gap-free state whose last report is
  Passed (PP :7262-7290).
- A case that does not converge, or fails qualification, publishes a blocking
  `SOLVER_SYSTEM_BLOCKED` (PP :5624-5650).
- The whole document is then a blocked envelope (PP :3224-3225; test
  `preview_physics_runtime.rs:1292`).

**Which cases.**
- On PF, every primitive case of a model that has any nonlinear support (the early
  return is at PP :7186-7188).
- EX refuses nonlinear supports by name (`exact_admission.rs:347-350`).
- W1/retained never sees a nonlinear case (kd5 §4, S2 :286).

**Thermal plus sustained in one case: yes, on PF.**
- The solver ignores `load_cases[].kind` (schema `LoadCase` description on M).
- A case may mix weight, forces and ΔT (PP `parse_category` :14915-14928).
- With stage 2, imposed motion may join them in a nonlinear model. M still refuses it,
  `IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` (M `imposed_motion.rs:33`).
- The demo's `load:L-100`, "Invented operating gravity, occasional and thermal
  preview", is such a case, with a one-way stop and a friction shoe
  (`tests/friction_reversal_stop_release_runtime.rs:1-30`).
- PF has no pressure (DEL-04-07 `pressure-contract.md:5`).

**How support status is reported, per case.**

| Output | Location |
|---|---|
| State code: 0 inactive, 1 active (a closed gap is Active), 2 sticking, 3 sliding | PP :7408-7425 |
| `nonlinear_support_final_displacement` | :7436-7455 |
| `nonlinear_support_final_reaction` | :7458-7477 |
| `NONLINEAR_SUPPORT_STATE_REVIEW` | :7480-7495 |
| Iteration, residual and converged rows | ~:7325-7370 |
| `NONLINEAR_SUPPORT_LOOP_CONVERGED` / `_NOT_CONVERGED` | :7498-7505 |
| Explicit and derived normal rows: `nonlinear_support_friction_normal_reaction_input` and `_derived` | PP `lib.rs:14493–14540` |
| Stage 2: `nonlinear_support_final_relative_displacement` | :14709-14745 |

Standing:
- **Confirmed:** the evidence line (:1436).
- **Not confirmed:** the "not established" sentence (:1429-1431).
- **Gap cases:** `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (:5683-5708; R1).

**Combinations on PF.**
- **Gates apply only to `mechanics`** (`preview_physics.rs` `gate_reason` :797-830).
  The basis test is at :802. The gates are `NONLINEAR_COMBINATION_REQUIRES_SOLVE`
  (:805-807), the constant-effort unit factor sum, and `COMBINATION_MODULUS_BASIS_MIXED`.
- **Subtraction is ungated.** `result_state_subtraction` and `range_envelope` bypass
  every gate. The subtraction value is fl(minuend − subtrahend) (LCA `lib.rs:367-427`).
- **It already forms in nonlinear models:**
  `nonlinear_mechanics_combinations_are_withheld_subtraction_kept`
  (`preview_physics_runtime.rs:779-801`).
- **What is combined.** The 22 linear state kinds, including nonlinear supports'
  support-action groups (`preview_physics.rs:25-48`, :893-895, :995-1059). Nonlinear
  rows are never combined.
- **Sign text.** Rows carry `SUBTRACTION_SIGN`, "a signed difference of solved states,
  not an equilibrium state" (:18).
- **Modulus basis.** Each operand's modulus basis is recorded (PP :11286-11340) but not
  gated.
- **Evidence.** Gates only (:918-932); there is no operand standing.
- **Text mismatch.** The K-D5 text says nonlinear combinations are withheld (M kd5 :62,
  :253). The code withholds only `mechanics`. See `truthfulness_fixes.md` (a).

**Standing of combination rows.** Combination rows remain derived inspection results
without their own numerical enclosure, even when both operands pass. Ordinary PF
aggregate standing concerns primitive cases; `SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED`
applies to the pressure-source path. Gap certification can establish gap operands, but
does not establish their subtraction or confer engineering acceptance.

This distinction must survive the desktop, export and rule-check surfaces. §4 proposes
how a derived row could be established.

**EX combinations (W16B).**
- All three bases, gated on mechanics only (`exact-combinations.md` §4, :180-245).
- Standing is `derived_from_case_results`, with operand `numerical_quality` (§5-§6,
  :254-303; `exact_combinations.rs:22-24`, :33-34, :700-760, :832-850).
- No enclosure of the combination (:269-270).

**Stage 2 (#1654) and R-c (#1671).**
- **R-a:** rigid anchors and restraints move inside the loop. Anchor and nozzle thermal
  movements can join an operating case.
- **R-b:** a nonlinear support's ground moves.
- **R-d:** a mixed record's rigid restraints move.
- **R-c:** a spring's base moves.

These are in DEL-04-03 `pf-nonlinear-imposed-motion.md` §0-§1, §10 and §11.

**Admission limits relevant to imports.** All three are now part of R-4.
- Friction needs a linear normal source (PP :7957-7967).
- Two gap records on one DOF: the loop errors when both are active (NI :1989-1995), and
  the confirmation refuses a shared DOF (`confirmation.rs:185-197`).
- No gap is ever confirmed (R1).

### 1.2 (b) The smallest addition

There is no new case kind or solve flag. The item is a derived subtraction with a
declared basis and operand standing.

**Slice 1a: no model-document change, PF only.** This is the smallest useful slice:
1. **Operand standing.** For each `result_state_subtraction` and `range_envelope` in a
   model with nonlinear supports, record the operand's load-case id, modulus basis, and
   numerical quality or standing sentence, in W-16b's EX shape.
2. **Text fixes.** The `truthfulness_fixes.md` corrections.

Separable additions, to be scheduled independently:
- an info diagnostic naming supports whose final state differs between the operands;
- a warning when an operand is not Passed;
- the desktop `operating` kind token and an "operating − sustained" subtraction
  template.

**Slice 1b: the 0.9.0 member** `combinations[].difference_basis =
"operating_minus_sustained"` (§1.4). Its gates are warnings that withhold the declared
combination:
- **G-1** is a conservative containment check. The subtrahend's primitive loads, as a
  multiset, are contained in the minuend's by content: category, target, direction,
  dimension, and value and unit bits after the solve's own normalization; ids are
  ignored. Code `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`. It does **not** require
  equal final active sets.
- **G-2:** same modulus basis (`EXPANSION_BASIS_MODULUS_MIXED`). G-2 binds the resolved
  elastic basis. Future case-dependent stiffness or E(T) behavior requires comparison of
  the actual stiffness-defining inputs. Different-basis comparisons remain available as
  plain subtractions.

Rule packs bind the combination through the existing basis ref
(`core/rules/rule_pack_document/src/format.rs:73-74`).

**EX.** Nonlinear supports stay out of this item (R-4 is a separate staged design). EX
refuses `difference_basis` by name.

### 1.3 (c) Questions on the code basis

| # | Question | Today | Recommendation | Type |
|---|---|---|---|---|
| c1 | Which stress range, and what "expansion" means | Code formulas are user rule packs (PRD §12.6-12.7, M :743-768). | The product publishes signed O−S rows. The public engineering meaning of "Expansion", including any claim that it represents a history or a code-specific range, is a product and engineering decision, not a technical consequence of subtraction (§5). | Owner/engineering for the meaning; technical for the computation |
| c1′ | Modulus basis | Recorded, not gated. | G-2 under the declared basis; plain subtraction unchanged. | Technical |
| c2 | Is status carried between cases? | No: each case iterates from the same authored seed. | Own status (decided, R-1). Frozen status is §5 option 2. | Decided; frozen status flagged to the owner |
| c3 | Friction basis | Each operand is solved independently using the static, zero-reference Coulomb admissibility model. No loading trajectory, accumulated slip or continuation between cases is represented. With active friction, confirmation establishes one admissible state, not uniqueness or physical reachability through a specified history. Operand evidence identifies the selected branch, including slip sense and normal-source evidence. | Keep it (decided, R-2): static and history-free, and friction may admit several states. V-3 shows a history model would differ (0 against +512 N). DEL-04-04:2 governs any history model. | Decided; history is a later item |
| c4 | Non-converged operating case | Blocks the whole document (PP :3224-3225). | Keep. | Technical |
| c4′ | An operand that is not established | Published with a sentence; the subtraction carries no operand record. | Publish with standing, never withhold (decided, R-5). | Decided |
| c5 | Wording | — | "Operating minus sustained: a signed difference of two independently solved states, each using its own reported support state and carrying its own numerical standing under the static, history-free friction model; derived from case results; not an equilibrium state." Never "certified"; "established" only under §4. | Technical, subject to §5 |
| c6 | Gaps and standing | R1. | See the standing paragraph in §1.1. G2 (R-3) establishes gap operands; §4 is what could establish the derived row. | Decided (R-3 adopted) |

### 1.4 (d) Affected deliverables and 0.9.0 members

**Deliverables.**

| Deliverable | What changes |
|---|---|
| DEL-05-02 | The basis, G-1/G-2 and PF operand standing. |
| DEL-04-07 | PF combination evidence; `pressure-contract.md`. |
| DEL-04-01 | The kd5 text (`truthfulness_fixes.md`) and the §4 certificate. |
| DEL-04-04 | CLM-012 and the DEL-04-04:2 wording; V-3's disclosure. |
| DEL-02-01 | The 0.9.0 member. |
| DEL-17-04/-05 | A later operating-case import, pressure-free until R-4. |
| DEL-06-01 | None. |
| DEL-08-04 | Vocabulary for the new evidence and codes. |
| DEL-09-03 | V-1 to V-5. |
| Desktop load-case surfaces | The token and template. |

**Members for 0.9.0.**

| Member | Type | Default | Validation |
|---|---|---|---|
| `combinations[].difference_basis` | string enum `["operating_minus_sustained"]` | absent (a plain subtraction) | Only with `basis: result_state_subtraction` (otherwise `LOAD_COMBINATION_DIFFERENCE_BASIS_MISPLACED`). Null and unknown tokens are refused (`…_UNKNOWN`). The minuend is operating and the subtrahend sustained. Refused by name on EX. G-1/G-2 apply at solve. Written only when the user sets it. |
| `load_cases[].kind` token `operating` | free string | — | Desktop vocabulary only; the solver keeps ignoring `kind`. |
| `combinations[].subtrahend_support_state` (later, only if the owner chooses §5 option 2) | enum `["own", "minuend_final"]` | absent = `own` | Only with `difference_basis`. Results are published under the combination, never as the sustained case's rows. |

**Provisional size of 1b:** S–M.

### 1.5 Verification ideas (independent exact references, both modes)

**V-1: a gap closing under thermal growth.**
- **Model.**
  - Bar along X, anchored at x = 0, L = 4 m, k = EA/L = 2^20 N/m.
  - Tip gap on UX, closing in +, c = 2^-9 m.
  - α = 2^-16 /K and ΔT = 16 K, so ε_T = 2^-12 and δ_T = 2^-10 m.
  - Sustained: F = 1536 N in +X, so δ_s = 3·2^-11 m.
- **Expected values.**

  | Case | Tip u | Gap | Stop on the pipe | Axial N |
  |---|---|---|---|---|
  | Sustained | 3·2^-11 m | open | 0 | 1536 N |
  | Operating | c = 2^-9 m | Active | −512 N | 1024 N |
  | O−S | Δu = 2^-11 m | — | −512 N | ΔN = −512 N |
  | Thermal-only (today's MBF mapping) | 2^-10 m | open | — | 0 |

- **Twins.**
  - c = 2^-8 m: neither case closes.
  - c = 5·2^-11 m: the weak boundary (R = 0, Active, unconfirmed under R1).
  - F = 2560 N: the gap is Active in both cases. S: u = c, N = 2048 N. O: u = c,
    N = 1024 N. O−S: Δu = 0, ΔN = −1024 N. Equal state codes do not make O−S a
    thermal-only result.

**V-2: a one-way prop lifting off under anchor rise (stage 2 R-a; W-16 A-1 operands).**
s* = 5PL³/(48EI).

| s | Operating | O−S | Displacement-only cases |
|---|---|---|---|
| 2s* | Prop Inactive, u_tip = s*, M_A = PL/2 | ΔR = −5P/16, ΔM_A = 5PL/16 | Nonlinear: lifts off, so 0. Linear rigid-prop: 5PL/8 with a tensile prop, inadmissible. |
| s*/2 | R = 5P/32, M_A = 11PL/32 | ΔR = −5P/32, ΔM_A = 5PL/32 | — |

In the sustained case R = 5P/16 and M_A = 3PL/16.

**V-3: friction, a static history-free witness against a load history.**
- **Model.**
  - The V-1 bar, k = 2^20, with a tip friction support on UX.
  - Explicit normal 4096 N and μ = 0.25.
  - Sustained: F = 2048 N in +X.
  - Operating: F plus cooling, δ_T = −2^-11 m.
- **Expected values.**

  | Case | Tip u | Axial N | Friction on the pipe | State |
  |---|---|---|---|---|
  | Sustained | 2^-10 m | 1024 N | — | sliding + |
  | Operating, as solved today (each case independent) | 2^-11 m | 1024 N | −1024 N | sliding + |
  | O−S, as solved today | Δu = −2^-11 m | ΔN = 0 | — | — |
  | A sequential history (stick measured from u_s; not modelled) | 2^-10 m | 1536 N | −512 N | sticking |
  | O−S, sequential | Δu = 0 | ΔN = +512 N | — | — |

**V-4: properties.**
- Swapping operands negates finite nonzero subtraction values; zero comparisons ignore
  the sign bit.
- Verify case independence through input-seed instrumentation and case-order checks;
  V-2's final values alone do not detect seed carry-over.

**V-5: negatives and mutants.**
- Negatives:
  - a G-1 mismatch: a sustained-only load, or a magnitude differing by one ulp;
  - G-2 mixed bases;
  - `difference_basis` on `mechanics` or `range_envelope`;
  - the member on EX;
  - a non-converged operand (the document is blocked).
- Mutants:
  - G-1 comparing ids instead of content;
  - reversed operand order;
  - seed carry-over between cases, detected by the instrumentation in V-4.

---

## 2. Item 2: the kernel side of skewed restraints

### 2.1 How restraints and springs are assembled today

**Records.**
- `LinearSupport` (LSUP `lib.rs:118-125`).
- `line_stop`: one translational DOF (:161-170, :656).
- `spring` (:183-197) and `SpringEntry` (:217-221).

**Rigid restraints are a DOF partition.** `prepare_boundary` collects the indices
(:384-429), and `reduce_system_with_prescribed_displacements` eliminates them (FK
`lib.rs:1712-1730`; the NI variant is at :1737-1755).

**Springs are diagonal adds.** LSUP :461 and FK `structural/sparse.rs:743-746`. The
product mapping is PP :8611-8670 and :8724-8745.

**K-D5.**
- `FormationSource.springs: Vec<(usize, f64)>` (`formation_check.rs:354-355`; filled at
  PP `certified_publication.rs:26-29`).
- The spring residual (:757-764) and the bit-for-bit binding (:870-878).
- `unavailable` demotes (:356-358).
- `certify_support_reaction` (`certified_recovery.rs:1454`, springs at :1518-1526) and
  `certify_spring_reaction` (:1732-1747).
- W2 `force_scaled_spring_action` (FK `lib.rs:1273`).

**Other consumers of DOF indices.**
- NI `base_restrained_dofs` and its springs list (PP :7246-7249).
- The ground-constraint count (PP :2997-3019).
- `assess_rigid_body(…, ground_dofs, …)` (FK `rigid_body.rs:33-37`).
- Derived friction-normal sources (PP :7939-7975).
- Motion targets (R-a, R-c, R-d).
- EX admission.

**NS.** Axis DOFs only (NS :39-46).

### 2.2 What a skewed direction needs

**Exactness fact.** No binary64 vector with two or more nonzero components is exactly
unit length. Binary64 values are dyadic, and p² + q² (+ r²) = 4^k forces every
numerator even; descent leaves only the axes. The kernel therefore works on the authored
binary64 d (R-7), scale-free. Exact ± axis directions route to today's path, byte for
byte.

**Range-safe scaling.** Evaluate the projector with range-safe scaling. Finite nonzero d
does not guarantee representable intermediate squares; for example d = 2^600(3,4,0)
overflows a naive denominator, while 2^−600(3,4,0) underflows it.
- **Design.** Scale d by an exact power of two 2^s so that max|d_i| ∈ [1, 2). This is
  exact, and leaves every scale-free quantity unchanged.
- **When scaling is inexact.** If a smaller component would lose bits under the scaling
  (subnormal), or any exact product the certificate needs falls below the exact-split
  range (FK `connector.rs` `SPLIT_EXACT_MIN`, 2^-969), the record is refused by name
  (for example `SUPPORT_DIRECTION_RANGE_UNSUPPORTED`). It is never silently rounded.
- **Test.** 2^600(3,4,0) and 2^−600(3,4,0) must give bit-for-bit the outputs of
  (3,4,0).

**Skewed spring (translational).**
- **Stiffness.** K_s = k·d dᵀ/(dᵀd), a rank-1 block inside the node's existing 3×3
  translational block. The sparse pattern does not change; assembly gains off-diagonal
  adds.
- **Recovery.** For fixed ground, the spring-on-pipe vector is R_s = −k d(dᵀu)/(dᵀd);
  its signed scalar along +d is −k(dᵀu)/√(dᵀd). In S-1 these are (6P/25, 8P/25) and
  +2P/5. Recover both from the corrected displacement with certified division and
  square-root bounds; an ExactAccumulator alone does not make these quotients exact.
- **What the re-formation needs.** A new `FormationSource` contribution kind, with:
  - independent evaluation of the source leaves (k and d) through the solve's own
    normalization path;
  - binding of the identity and order of the six-entry contribution (the upper
    triangle of the block);
  - formation and assembly error, bounded as for every formed entry (Wide2 balls, as
    `connector_matrix`, `formation_check.rs:2695`, does);
  - action residuals −K_s(u + w + w₂) on the free rows;
  - recovery bounds for R_s and its scalar;
  - force scaling for W2/K2b (the block times 2^b, mirroring `force_scaled_spring_action`).

  Registering a rounded block is not enough.
- **Smaller first step, if wanted.** List the block in `FormationSource.unavailable`.
  The case is then demoted, with a named and truthful standing, before re-formation
  lands.

**Skewed rigid stop (R-6): the exact constraint by a division-free nullspace basis.**

Codex's general statement: Either preserve authored dᵀu = 0, using a division-free
nullspace basis, or explicitly define the represented constraint as cᵀu = 0 after
rounded division. For one independent c-constraint, R = λc and λ = (Ku − f)_m, since
c_m = 1. Its scalar projected along authored +d is λ(cᵀd)/‖d‖; when c and d are
collinear this equals sign(d_m)λ‖c‖. Formation, lifting and publication roundoff
require bounds.

**Adopted: preserve the authored dᵀu = 0.** Rounded division is not used, because it
changes the authored constraint. With d = (1,3,0), fl(1/3) defines a different plane.
A negative pivot also reverses the scalar: d = (−3,−4,0) gives −4P/5 along authored +d.

1. **Basis.** For one constraint d at a node, pick a pivot m with d_m ≠ 0. A fixed rule,
   for example the largest |d_m| with ties to the lowest index, affects conditioning
   only, not the constraint. The node's translations are u = T w, where T has the two
   columns d_m e_j − d_j e_m for j ≠ m.
   - Every entry is an authored (scaled) component or zero. No product is formed.
   - dᵀ(d_m e_j − d_j e_m) = d_m d_j − d_j d_m = 0 exactly, so dᵀu = 0 holds exactly
     for every w.
   - T has full rank because d_m ≠ 0.
2. **The reduced system.** It is TᵀK_int T w = Tᵀ f_int at that node, identity
   elsewhere. T mixes only one node's translations, so the sparse pattern is unchanged.
   - Assembly forms TᵀKT and Tᵀf in binary64.
   - K-D5 re-forms them at p from the source primitives and exact T, and certifies:
     - the formation;
     - the residual rows r̃ = Tᵀ(f − K u) on the reduced coordinates. These are
       products of a binary64 d entry with an exact accumulated row. Each term
       d·(K_ij·u_j) is a triple product: it needs an exact split of K_ij·u_j before
       the multiplication by d, because `ExactAccumulator.add_product` handles two
       binary64 factors only;
     - the correction solve with the binary64 factorization;
     - the lifting u = T(w + corrections), formed at p and rounded once for
       publication, with a bound;
     - the physical recovery rows.
3. **Reaction.** The support-on-pipe reaction is R = λd with R = (K u − f) at the node's
   translations.
   - λ = (K u − f)_m / d_m is formed with a certified division bound. The other rows
     must agree, λd_j, within the bound; this is a consistency check.
   - The published group is λd. The signed scalar along authored +d is λ‖d‖, with
     certified square-root and product bounds. Its sign follows authored +d whatever
     the pivot's sign.

### 2.3 Multiple restraints (part of R-6, not a later item)

Build the independent constraint system jointly with existing rigid restraints.
Construct a full-rank T, or refuse unsupported intersections by name. Redundant
constraints do not add rank and may prevent unique per-support reaction attribution.
Certify TᵀK_int T, Tᵀf_int, displacement lifting and physical recovery rows. Ground
screening uses independent constraint rows and their rigid-motion actions, not record
count.

The full design, per node, on its three translations:

1. **Constraint matrix.** Stack the node's constraint rows into C (r × 3): e_iᵀ for each
   axis restraint (anchor, guide, line stop, vertical support) and dᵀ for each skewed
   stop. Rotational restraints are separate and unchanged.
2. **Rank.** Decide rank(C) exactly; there is no tolerance. Form 3×3 determinants from
   exact products, including exact splits of triple products, without rounding
   intermediate minors. `ExactAccumulator.add_product` handles two binary64 factors
   and does not itself supply triple products; a triple product needs an exact split
   of the first product before its second multiplication. If rank(C) < r, the
   constraints are
   redundant and per-support attribution is not unique. That is refused by name (for
   example `SUPPORT_CONSTRAINT_REDUNDANT`), consistent with today's rule that no DOF is
   restrained twice at a node.
3. **The null basis by rank.**

   | rank(C) | Case | T |
   |---|---|---|
   | 0 | — | I |
   | 1 | one skewed d | §2.2 basis |
   | \|A\|+1 | axis restraint(s) on axes A plus one skewed d (every axis restraint prescribed zero, see below) | Restrict to the free axes F: d_A multiplies prescribed zero, so the constraint is d_Fᵀu_F = 0. If d_F = 0, the skewed row is redundant: refused. Otherwise T is the §2.2 basis on F, still authored components only. |
   | 2 | two independent rows c₁, c₂ | One column t = c₁ × c₂, whose entries are the 2×2 minors. These are division-free but not authored components, and not exact at p = 128: with ε = 2⁻¹⁰⁰, c₁ = (1,ε,0) and c₂ = (ε,1,1), the exact cross product (ε, −1, 1 − 2⁻²⁰⁰) needs 200 bits. Keep the minors and intended T as exact dyadics or exact expansions. A p=128 evaluation is an enclosure of that exact T, not its definition. Carry transformation evaluation error through reduced formation, lifting and recovery; refuse unsupported ranges by name. Assembly may use a rounded t; the published u satisfies the intended constraints only within the certified lifting bound. |
   | 3 | the node's translations are fixed | T is empty. |

   **Prescribed motion at a skewed-stop node.** A moving axis restraint changes the
   reduced constraint. For example, a UZ guide with u_z = h and a stop (3,4,5) gives
   3u_x + 4u_y = −5h. Until affine reduction is designed, refuse a skewed-stop node with
   any nonzero prescribed translational component. Supporting it requires u=u₀+Tw,
   Cu₀=g, and certification of TᵀK_intT and Tᵀ(f_int−K_intu₀), together with lifting and
   recovery.

4. **Reaction attribution.**
   - The node's translational residual r = (K u − f) equals Cᵀλ in the intended system.
   - With independent rows, λ is unique: solve Cᵀλ = r on a nonsingular r × r minor
     selected by an exact rule. This needs certified division and back-substitution
     bounds, and the remaining rows serve as a consistency check.
   - Each support publishes λ_k·c_k (an axis restraint, its component; a skewed stop,
     λ_k d_k, with scalar λ_k‖d_k‖ along +d_k).
   - Example: an axis restraint on a restrained axis that d also touches. λ for the
     skewed stop comes from the free-axis rows (d_F ≠ 0); the axis restraint's reaction
     is r_A − λ d_A. Both are unique.
5. **Ground screening.** Per connected body, translational ground directions contribute
   [cᵀ,(x×c)ᵀ], rotational restraints contribute [0,cᵀ], and positive springs contribute
   their corresponding directions. Rank screens rigid modes; it does not establish
   internal stability or numerical adequacy. This replaces the DOF-index count at PP
   :2997-3019 and the `ground_dofs` input of FK `assess_rigid_body`.
6. **First build slice.** If the joint construction is split for build, the first slice
   may admit at most one skewed stop per node and refuse everything else by name. That
   is a build slice of this design, not the design.

### 2.4 Effects elsewhere (refused by name until designed)

| Area | Treatment |
|---|---|
| Motion targets on a skewed record (R-a/R-c/R-d) | The g ≠ 0 forms are dᵀu = ‖d‖g, with ‖d‖ generally irrational, and the R-c load term k·g·d/‖d‖. Each needs its own exactness design. |
| Prescribed motion on another restraint at a skewed-stop node | Until affine reduction is designed, refuse a skewed-stop node with any nonzero prescribed translational component. Supporting it requires u=u₀+Tw, Cu₀=g, and certification of TᵀK_intT and Tᵀ(f_int−K_intu₀), together with lifting and recovery. |
| Skewed records in a nonlinear model | NI takes DOF indices and `(dof, k)`. A skewed spring is a small extension there; a T-constrained node needs `active_boundary` and the confirmation to learn T. |
| A skewed record carrying `nonlinear` | Refused. |
| A skewed stop as a derived friction-normal source | Refused. |
| EX and W1 | Refused or declined by name. |
| Pricing | FK `formation_check.rs`, `certified_recovery.rs`, `sparse.rs`, the reduction, LSUP and PP pins move (DEL-04-08 D3). |

### 2.5 What the kernel needs from W-17's member (R-7)

- Three finite binary64 global components, not all zero, stored as authored and never
  normalized.
- Only on single-DOF `line_stop` and `spring` records with a translational DOF.
  Rotational skew comes later; the same algebra applies.
- The sign convention: published scalars are positive along authored +d.
- Exact ± axis directions keep today's path.

### 2.6 Verification ideas

**Common model.** A tip with stiffness diag(k_a, k_b) in plane, load (0, −P),
d = (3,4,0), dᵀd = 25.

**S-1, spring, k_s = k_a = k_b = k.**
- u = (6P/(25k), −17P/(25k)).
- R_s = (6P/25, 8P/25), and the scalar along +d is +2P/5.
- Equilibrium: (6P/25, −17P/25) = (0, −P) + R_s.

**S-2, orthogonal pair (property).** Springs along (3,4,0) and (−4,3,0), each of
stiffness k, equal axis springs on UX and UY, within the K-D5 balls.

**S-3, range.** d = 2^600(3,4,0) and 2^−600(3,4,0) give bit-for-bit the outputs of
(3,4,0). A vector whose scaled small component goes subnormal is refused by name.

**R-1, rigid stop, isotropic.**
- λ‖d‖ = +4P/5 along +d, and R = (12P/25, 16P/25).
- u = (12P/(25k), −9P/(25k)), with 3u_x + 4u_y = 0.

**R-2, rigid stop, anisotropic (k_a = 2k).**
- +40P/41 along +d.
- u = (12P/(41k), −9P/(41k)).

**R-3, rigid stop, sign and plane.**
- d = (−3,−4,0) must publish −4P/5 along its authored +d, with the same u as R-1.
- d = (1,3,0), isotropic: the scalar along +d is 3P/√10. The published u satisfies
  u_x + 3u_y = 0 within the lifting bound. A rounded-division mutant with
  c = (fl(1/3), 1) is killed.

**R-4, joint (isotropic tip stiffness k on all three translations, load (0, −P, 0)).**
- A UZ guide (prescribed zero) plus d = (3,4,5). The constraint reduces to
  3u_x + 4u_y = 0, and λ_stop = 4P/25. The guide reaction is r_z − 5λ = −4P/5. The
  stop's scalar along +d is λ√50.
- Two skewed stops (3,4,0) and (0,3,4): t = (16, −12, 9),
  u = (192, −144, 108)·P/(481k), and λ = (64P/481, 27P/481).
- The moving-guide negative: a UZ guide with u_z = h ≠ 0 and the stop (3,4,5) is
  refused by name (§2.3).
- Redundant (d plus axis restraints on all its nonzero axes) and fully fixed cases.
- Ground-rank controls.

**Controls and negatives.**
- d = ±e_x: today's outputs bit for bit, except the along-d scalar's sign for −e_x.
- Every §2.4 refusal, a zero vector, a non-finite component, and a rotational DOF.

---

## 3. Recommendations

### Item 1

- **Scope.** PF; a declared O−S subtraction with operand standing and G-1/G-2; no new
  solve kind; EX nonlinear deferred to R-4.
- **Smallest first slice: 1a.** Operand standing in the PF combination evidence for
  nonlinear models, plus the truthfulness fixes. The state-difference diagnostic, the
  operand warning and the desktop vocabulary and template are separable.
- **Order.**
  - The truthfulness fixes go now: text only.
  - 1a is built after §8.29 (stage 2) and priced after #1678 (§8.31).
  - 1b follows 0.9.0.
  - The §4 certificate is designed after 1a.
  - G2 (R-3) is designed after 1a.
  - R-4 is staged after 1a and 1b.
- **Size.**
  - 1a: S in code, M with readers, tables and pricing.
  - 1b: S–M, provisional.
  - §4: M–L.
- **Remaining choices for the owner.**
  - The meaning of "expansion" (§5), with frozen sustained status (R-1's remaining
    item).
  - Whether an established derived row counts toward numerical eligibility (§4.4).
  - A friction load-history model, later under DEL-04-04:2.

### Item 2

- **Scope.** Translational skewed `spring` and `line_stop` on PF's ordinary route, with
  the §2.4 refusals.
- **Slices.**
  1. The skewed spring: scale-free block, range-safe scaling, certified R_s and scalar,
     and the full formation contribution (§2.2).
  2. The skewed stop by the division-free nullspace basis, with joint per-node
     construction, attribution and ground rank (§2.2-§2.3). Its first build slice may
     admit one skewed stop per node.
- **Order.** After #1678 and 0.9.0.
- **Size.** Spring M; stop L.
- **Reserved choices.** None remain open in the kernel: R-6 and R-7 are decided.

---

## 4. What establishes a derived O−S row

### 4.1 What exists

**EX combinations** carry no bound. The rows are `derived_from_case_results`, and the
evidence copies each operand's `numerical_quality` record verbatim
(`exact_combinations.rs:22-24`, :832-850). The design names the missing piece:

> "A derived enclosure, rounded up from Σ|fᵢ|·rᵢ plus δ_publication, would be needed
> for numerical eligibility. It is not part of this slice." (`exact-combinations.md`
> :269-270)

The Rust reader re-derives rows and records bit for bit
(`exact_combination_evidence.rs`, design §6).

**PF and EX primitive cases** carry K-D5 bounds per group, not per published row
(kd5-member-relative-bound §3.1-§3.2, M :195-235):
- Each group g has a published value v_g = fl(m_g) and a bound
  B_g = ‖v_g − m_g‖ + ‖r_g‖ + ‖R_g‖, rounded upward. B_g bounds ‖v_g − g_int(u*)‖
  under A1.
- The groups are member end, station and section rows (|N|, hypot(V_y,V_z), |T|,
  hypot(M_y,M_z)), pipe-wall rows, stress rows (judged through their resultants), and
  support force and moment groups.
- For a confirmed nonlinear case, Stage B supplies the same member bound, against the
  intended system at that case's converged active set (kd5-nonlinear §3.1).
- Every pre-guard-Passed case publishes certified rows even when the predicate then
  demotes it.
- B_g itself is published only in a demotion sentence. It is not carried per group in
  the evidence.

### 4.2 Proposal: a combination certificate on certified operand groups

For a subtraction D = O − S and each group g present in both operands:

1. **Condition.** Both operands carry certified rows: member bound present, pre-guard
   Passed, no load-row finding. On PF nonlinear this means confirmed (non-gap) operands,
   and gap operands after G2 (R-3).
   - Operand-state agreement is **not** required numerically. Each operand's bound is
     against its own intended system, so the bound on D holds whatever the two active
     sets are.
   - Equal states remain a meaning question (§5).
   - G-1 and G-2 apply under the declared basis.
2. **Same route.** Operands are cases of one document, so they are on one route by
   construction. The certificate is route-agnostic: it applies equally to EX linear
   combinations (W-16b) and to EX nonlinear operands after R-4.
3. **The derived value.** v_D = fl(v_O − v_S) componentwise, as published today.
4. **The derived bound.** B_D = RU(B_O,g + B_S,g + ‖v_D − (v_O − v_S)‖). The last term
   is the exact rounding error of the subtraction, formed exactly with TwoSum (zero under
   Sterbenz's condition). Then ‖v_D − (m_O − m_S)‖ ≤ B_D against the two intended
   systems.
5. **Magnitudes.** Magnitude rows (support force and moment magnitudes, hypot groups)
   are formed at p from the derived components, then rounded, with their square-root
   error included, as kd5 §3.2 does for support magnitudes.
6. **The decision.** Apply the same 1e-9 predicate with q_D = ‖v_D‖ and
   q̂_D = max(q_D, ‖v_D‖ + B_D). The 1e−9 relative inequality is unchanged. Using the
   larger operand scale is a proposed floor definition for derived groups. Floor-excluded
   groups are reported as not assessed, never established. All norms and upper-bound
   arithmetic are rounded upward; unavailable comparisons fail closed.
   - **Cancellation is real.** Where O ≈ S on a group, B_D/q_D grows. That group is
     honestly not established. The criterion is not loosened.
7. **Range envelopes.** Range-envelope certification is deferred. It must enclose the
   intended per-row envelope, including uncertain selection and components selected
   from different operands. A selected operand's group bound alone is insufficient.
8. **Mechanics combinations.** On linear models and EX, Σ|fᵢ|·Bᵢ plus the exact sum
   error; same machinery.

### 4.3 What publishes, and what readers re-check

- **Evidence.** For each certified operand case that is a combination operand, publish
  its group bounds B_g, keyed by the group's structural key. For each combination
  group, publish B_D, q_D, q̂_D, the floor scale and the verdict. This extends W-16b's
  `exact_combinations` record shape to PF and carries it on EX.
  - **Publishing the complete group bounds is enough. The demotion sentences are not:**
    they quote only the q and B of the published rows they name.
  - **Coverage.** Each bound names its covered result rows, component frame and units.
    A group bound supplies an absolute bound for each constituent, not a separate
    relative-accuracy claim. Stress rows require the resultant bound transported
    through their scale factors, evaluation and publication rounding. Uncovered rows
    remain inspection results.
- **Readers.** RE and the Python verifier re-derive bit for bit:
  - v_D from the operand rows;
  - B_D from the published operand bounds and the exact subtraction error;
  - the predicate.

  They do not re-derive B_O and B_S. Those are the operand cases' published statements,
  checked as case evidence is today (PRD §12.8: readers check the published statement
  and what they can rederive; they do not re-run the solve). The reader's recomputation
  is therefore conditional on the published operand-bound statements: it establishes
  B_D given B_O and B_S, not B_O and B_S themselves. A forged B_D, verdict or
  operand-bound binding is refused.
- **Wording.** Product text says "established within its stated bound against the two
  cases' intended systems". It never says "certified" (R5). It states that this is a
  numerical statement about the difference of two independently solved states. It
  establishes neither the meaning of "expansion" (§5) nor engineering acceptance.

### 4.4 Remaining choice, size and order

- **Eligibility, a claim decision.** Whether an established derived row may count toward
  the document's numerical-eligibility verdict changes today's rule that combination
  rows never count. That is for Agent 0 or the owner under the claims registry. The
  certificate is useful either way, since rows can then carry their standing.
- **Size: M–L.**
  - PP: retain the group bounds per operand and form the combination groups.
  - New evidence and schemas.
  - RE and Python readers.
  - Semantic tables and pricing.
  - No FK change, since the groups and bounds exist.
- **Order.**
  - Design after 1a (it needs the operand-standing record) and before or with 1b.
  - Build ahead of G2 (R-3), so that gap operands become establishable the moment G2
    lands and the derived certificate follows with no extra work.
  - Independent of R-4: on EX it first serves W-16b's linear combinations, then
    nonlinear operands when R-4 lands.
- **Without §4.** Even with G2, O−S rows in nonlinear models stay inspection evidence.

---

## 5. The engineering meaning of "expansion" in nonlinear models

This is a short decision, and the same question as R-1 seen from the other side. The
computation is fixed technically: §1.2 computes O−S with each case at its own admissible
state. What is open is what the product **calls** the difference and what it claims it
represents.

| Option | Meaning | Code-basis consequence (generic) | Type |
|---|---|---|---|
| 1 | Expansion = O − S, each case at its own admissible state | The difference includes the effect of support-state changes between cold and operating (a lift-off, a gap closing). The sustained check that a rule pack runs on S uses the cold support state. Code interpretations that evaluate sustained loads with the operating support configuration are not represented. | Computation technical (exists); calling it "expansion" is an engineering claim for the owner |
| 2 | A held-constraint comparison: S solved at O's frozen support status, expansion = O − S_frozen | Frozen support status defines a separate held-constraint comparison; it can require tensile unilateral-support reactions and must not be represented as an admissible sustained nonlinear state merely because its linear equations are certified. It matches interpretations that evaluate sustained loads with the operating configuration, but needs a new frozen-state solve and its own certificate (§1.4 `subtrahend_support_state`). | Code-interpretation matter for the owner |
| 3 | The product reports only "operating minus sustained difference"; the label "expansion" and any code-range meaning belong to the user's rule pack | No product claim that the difference is an expansion range or represents a history. The rule pack maps the combination to its expansion check. Option 2 can be added later as a second declared basis without renaming anything. | Product choice; least claim |

**Recommendation: option 3, computed as option 1.**
- The member is already named `operating_minus_sustained`, not `expansion`.
- Row text uses "operating minus sustained difference".
- The desktop template label says the same.
- The rule pack is where a user's design basis names it "expansion".
- Option 2 stays R-1's remaining item for the owner.
- Under this recommendation, §4's "established" says nothing about which option is the
  right engineering basis.

---

## 6. As built (1b)

Slice 1b on branch `claude/rt-a5-slice1b` (from main `bbf384dec`, merged with main
`800fae7e5`). Candidate: not integrated, not independently reviewed. Risk class: numerical
meaning, persistence and contract. Nothing here records acceptance, issuance or release.

### 6.1 What ships

On the pressure-free route (PF, preview-physics-2), a `result_state_subtraction` declaring
`difference_basis = "operating_minus_sustained"` (0.9.0 §2.2) is gated, published and
authored:
- **Solve.** S1's interim `LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED` withholding is
  removed (0.9.0 §2.2 "Interim … 1b removes both and adds G-1/G-2"; §3.8 row "the solver
  lane's 1b"). The combination is gated by G-1 then G-2 (§6.2). An admitted one publishes the
  plain subtraction's rows bit for bit (fl(O − S), the same pairing, the same
  `metadata.basis` `explicit_user_result_state_subtraction`) with the declared sign
  convention (§6.3), and slice 1a's operand standing is still published in a nonlinear model
  (§0 R-5, publish with standing). A withheld one publishes no row, as every gated
  combination; its dependants are withheld whole by the existing
  `LOAD_COMBINATION_OPERAND_WITHHELD`.
- **Applier.** `create_combination` writes `difference_basis` (and `subtrahend_support_state`)
  as authored on a document without a pressure contract. On a document with one it is
  refused by name, `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`; `minuend_final` stays
  refused (`LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`); `own` is authored and
  behaves as absence; an unknown or misplaced member, and a combination operand, are the
  document validator's (`…_UNKNOWN`, `…_MISPLACED`, `…_OPERAND_NOT_LOAD_CASE`). No other
  combination operation sets the member.
- **Unchanged.** EX keeps its blocking `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`
  before any gate (`exact_admission`, the EX gate code and the pressure-2 table untouched).
  Every `*_EXACT_UNSUPPORTED` refusal stands. A plain subtraction is byte-identical
  (`pressure_free_byte_identity`, V-8). The retained routes keep declining the member
  (`FamilyFact::CombinationMember`); the CAEPIPE SI writer keeps refusing it (0.9.0 §3.8,
  DEL-17-04's mapping).

Interpretations applied (manager-accepted, 2026-11-03):
- **(i) The interim warning's removal.** 0.9.0 §2.2's interim rule and §3.8's row remove
  `LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED` from the solve at 1b. A `minuend_final`
  combination is therefore withheld by `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`
  alone (its former second warning said "its gates are not built yet", which is no longer
  true). EX still blocks first. The code constant is kept for the history of its refusals.
- **(ii) The sign convention** (§1.3 c5; §5 B option 3, "Row text uses 'operating minus
  sustained difference'"): the plain subtraction's positive-value clause, then c5 under
  option 3's name (§6.3). `metadata.basis` is unchanged.

Separable additions not in 1b (§1.2 "Separable additions"; pf-nonlinear-operand-standing.md
§5): the desktop `operating` kind token and the "operating − sustained" template. The desktop
only names a declared subtraction's rows in the results navigator ("operating minus sustained
difference") and draws them as it draws a plain subtraction's.

### 6.2 Gates: semantics and order

In `preview_physics::gate_reason`, for a declared subtraction:
1. the withheld-operand rule (`LOAD_COMBINATION_OPERAND_WITHHELD`) comes first for every
   combination; a declared subtraction never meets it, since its operands are load cases
   (absolute-sum-srss §2, r1 F7, `LOAD_COMBINATION_DIFFERENCE_BASIS_OPERAND_NOT_LOAD_CASE`,
   blocking at validation);
2. **G-1**, `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`;
3. **G-2**, `EXPANSION_BASIS_MODULUS_MIXED`.

The term-basis gates (nonlinear except OQ-1, the κ gates, `COMBINATION_MODULUS_BASIS_MIXED`)
apply to `mechanics`, `absolute_sum` and `srss` only, so they never reach a subtraction. Each
withholding is a warning, id `diagnostic:preview-physics:combination-gate:<identity>`, source
`core/product_physics`, affected refs `[combination id]`; the gate record is `{withheld: true,
reason}`.

- **G-1** (§1.2 and 0.9.0 §2.2, verbatim there). Each operand case's primitive loads are read
  from the model the solve gated and solved: the lowered model (so S2's condition-generated
  thermal and added-weight primitives count by content, E7) after `normalize_model_units`.
  A primitive's content is its category, target (`node` and its node; `element` and its pipe;
  `support`, its support and DOF), direction, dimension, the magnitude's value bits and its
  unit. Tokens are compared as authored (no alias mapping, so `distributed_force` and `weight`
  differ); value bits exactly (so +0.0 and −0.0 are different contents; validation admits
  finite values only). Ids and provenance are not content. G-1 passes when, for every content,
  the subtrahend's count does not exceed the minuend's. It does not compare final active sets,
  and it reads primitive loads only: a case's `equivalent_static` generation inputs, its
  case-state records and its modulus selection are not G-1 content (G-2 reads the modulus).
  Message: "Combination withheld: it declares an operating minus sustained difference, and the
  subtrahend case's primitive loads, as a multiset by content (category, target, direction,
  dimension, value and unit after normalization; ids ignored), are not contained in the
  minuend case's."
- **G-2** (§1.2). The two cases' `modulus_basis_key` (the solve's own key: `exact:<ref>`,
  `temperature_kelvin_bits:<bits>`, or none) differ. Message: "Combination withheld: it
  declares an operating minus sustained difference, and its two load cases were solved on
  different modulus bases."

The producer is `core/product_physics/src/difference_basis.rs`; G-1 and the evidence (§6.4)
read one function (`contents`) over the one model: there is no second resolver.

### 6.3 Row text

Sign convention of an admitted declared subtraction's rows (§1.3 c5; §5 option 3):

    positive value follows explicit user result-state subtraction (minuend minus subtrahend) of matching source result sign conventions; operating minus sustained difference: a signed difference of two independently solved states, each using its own reported support state and carrying its own numerical standing under the static, history-free friction model; derived from case results; not an equilibrium state

A plain subtraction keeps "… ; a signed difference of solved states, not an equilibrium
state". Never "certified" or "established" (§4 is unbuilt).

### 6.4 G-1's derivation evidence (Agent 0's decision A)

`contract_evidence.difference_basis_derivation`, an optional member of results.v0.3's (and
stress_neutral_export.v0.3's) closed `PreviewPhysicsContractEvidence`, admitted there beside
`combination_derivation` because a reader without the model otherwise has no basis for G-1
(§10.3's reasoning). No version change; no other family widens. Present only when a gate is a
declared subtraction's, so every other document keeps its bytes:

```json
[{"combination_id": "comb:o-s", "difference_basis": "operating_minus_sustained",
  "minuend": {"load_case_id": "case:O", "primitive_contents": [
    {"category": "concentrated_force", "target": {"type": "node", "node": "node:b"},
     "direction": "global_x", "dimension": "force",
     "magnitude": {"value_bits": "4098000000000000", "unit": "N"}, "count": 1},
    {"category": "thermal", "target": {"type": "element", "pipe": "pipe:a-b"},
     "direction": "global_x", "dimension": "temperature_interval",
     "magnitude": {"value_bits": "4030000000000000", "unit": "K"}, "count": 1}]},
  "subtrahend": {"load_case_id": "case:S", "primitive_contents": [ ... ]}}]
```

- One record per declared subtraction among the gates, in gate (evaluation) order, withheld or
  admitted, binding its combination id and both operand case ids.
- Each operand's `primitive_contents` is its multiset: one entry per distinct content with its
  `count`. **Count domain** (review r1): a JSON integer token from 1 to 2^53 − 1. The producer
  counts a case's primitive loads (`usize`), and a published statement is a checked-JSON
  document (`openpipestress_jcs_ijson_v1`, whose integers are bounded by ±(2^53 − 1),
  `MAX_SAFE_INTEGER`), so no larger count can be published; a float token (`1.0`), a
  boolean, a string, zero or a negative number is not a count. Both readers enforce exactly
  this (Rust `count_of`, Python `MAX_COUNT`).
- A case is resolved once, so a case id that recurs across records (as minuend or
  subtrahend) carries byte-identical contents (the same JSON with members sorted). `value_bits` is the 16 lowercase hexadecimal digits of the
  normalized binary64's IEEE 754 bits (signed zero distinct; always finite); `unit` is the
  normalized unit symbol. Canonical order: byte-wise over category, target type, target ids
  (node; pipe; support then DOF), direction, dimension, `value_bits`, unit; each content once.
- Size: at most one record per combination and, per operand, at most one entry per primitive
  load of that case, so it is bounded by the admitted model's own limits; it lives in the
  statement's contract evidence beside its rows, not in any record header, and meets the
  existing size guards. No new threshold.

### 6.5 Readers

Rust RE `preview_difference_basis.rs` (called from `preview_combination_gates::check`, so every
entry that reads PF gates reaches it: the evidence reader, `reader_verdict`, the runner,
packaging `bindings.rs` and derivatives `derivative.rs` through `semantic_contract::for_source`,
and the wasm engine); Python `tests/verifiers/preview_difference_basis.py`.
- **With the model** (decision M1). The member first passes the same validator as the
  model-free reading (closed shape, count domain, bindings, case recurrence; review r1), so a
  malformed member is refused by its own code before any comparison. G-1 is then re-derived from the model's own primitive loads,
  normalized by the shared units authority (Python: its independent unit table), and the
  published evidence must equal it record for record (`SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_MODEL`);
  the gate must be G-1's, else G-2's from `modulus_key`
  (`SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_DERIVATION`). **Limit:** the readers have no
  condition resolver (0.9.0 §3.8: the source readers decline `condition_use`), so where either
  operand case carries a `condition_use` G-1 cannot be re-derived and the document is refused
  by name, `SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`; it never falls back to the
  model-free reading. A common pure lowering for readers was not built.
- **Without the model: option A, shipped** (Agent 0's decision; enabled after review r1 and
  its focused confirmation cleared it). Any declared claim (the evidence member, a G-1 or G-2
  gate reason, or a row stating §6.3's sign) is read from the published evidence, so
  model-free consumers (packaging, derivatives, the runner without a model) read a valid
  declared document and refuse a forged one by name. The interim fail-closed switch (option
  C) is deleted, not kept as a mode: no design or consumer names it. The member's closed shape, canonical
  order, counts in the count domain and finite values are checked
  (`SOURCE_PREVIEW_PHYSICS_DIFFERENCE_BASIS_EVIDENCE`); a case id recurring across records with
  different contents is refused (`…_DIFFERENCE_BASIS_BINDING`, in both readings); its bindings to the document
  (gate present, published cases, distinct operands, the two `combination_modulus_basis_record`
  rows naming minuend then subtrahend, the rows' operand sources, and any
  `combination_operands` / `combination_derivation` expression) (`…_DIFFERENCE_BASIS_BINDING`);
  every G-1/G-2 reason and every declared-sign row has its record (`…_EVIDENCE`); G-1 is
  re-derived from the two multisets and G-2 from the published modulus-basis labels (equal
  key, equal label), and the gate must be the derived one (`…_COMBINATION_GATE_DERIVATION`);
  a declared subtraction's rows state §6.3's sign and the subtraction basis and no other
  combination's row states it (`…_DIFFERENCE_BASIS_ROW`). **Scope:** this verifies the
  record's internal derivation and its binding to the document, not its truth to the absent
  model. A coherent forgery (evidence, gate and rows altered together) passes without the
  model and is refused with it (`a_coherent_forgery_is_not_authenticated_without_the_model`,
  both languages).
- The evidence readers admit the new member and the two gate reasons, refuse the member on a
  blocked envelope, and admit a withheld subtraction's `combination_modulus_basis_record` rows
  (DEC-068 item 1; G-2's model-free source; before 1b a withheld subtraction could not arise,
  and a withheld range envelope with a load-case operand would have been refused by
  `GATED_COMBINATION_ROW`). The slice-1a model check requires an operand record only for an
  admitted subtraction or envelope.
- The desktop reads PF verdicts through the wasm reader; it re-derives no gate. Its solve
  timeline code table is unchanged (regenerated; no recording-path code added).
- **Pre-existing limit, unchanged:** PF readers do not re-derive a flat subtraction's (or
  flat mechanics combination's or envelope's) signed values (absolute-sum-srss §10.1); a
  declared subtraction is flat, so its values are checked as a plain subtraction's are, and
  its gates, evidence and sign text as above.

### 6.6 Contract texts (old → new) and digests

The preview-physics-2 `combination_policy` (and preview-physics-retained-1's copy, by hand;
timoshenko-1 by `derive`). Members not listed are unchanged; `gated_forms` stays
`["mechanics", "absolute_sum", "srss"]` (always-gated forms), the declared subtraction being
named by the new member and `gate_precedence`.
- `policy_id`: old "PREVIEW-PHYSICS-1-COMBINATION-v2"; new "PREVIEW-PHYSICS-1-COMBINATION-v3"
  (the policy's meaning grows, §10.2's rule: a v2 consumer must not read a v3 document's
  subtractions as ungated).
- `difference_basis_gates` (new, after `unsigned_gates`):
  - EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING: "the subtraction declares difference_basis
    operating_minus_sustained and the subtrahend case's primitive loads, as a multiset by
    content (category, target, direction, dimension, and the magnitude's value bits and unit
    after the solve's own unit normalization, on the lowered model, so condition-generated
    primitives count; ids and provenance are not content), are not contained in the minuend
    case's: some content occurs more often in the subtrahend; equal final support states are
    not required";
  - EXPANSION_BASIS_MODULUS_MIXED: "the subtraction declares difference_basis
    operating_minus_sustained and its two load cases have different modulus-basis keys".
- `gate_precedence`: old "operand_gate first, then the first gate in list order of the
  combination's form supplies the single withheld reason"; new: old text, then "; a
  result_state_subtraction declaring difference_basis operating_minus_sustained takes
  difference_basis_gates (its operands are load cases, so operand_gate never applies), and any
  other subtraction or range envelope is not gated".
- `result_state_subtraction`: old "signed difference of solved states over linear_state_kinds,
  with recomputed magnitudes; no equilibrium claim"; new: old text, then "; with
  difference_basis operating_minus_sustained (minuend and subtrahend load cases) its rows' sign
  convention reads \"<§6.3's text>\", their metadata basis is unchanged, and
  contract_evidence.difference_basis_derivation states both cases' primitive-content
  multisets; a subtrahend_support_state minuend_final is withheld before the gates
  (LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED) and own behaves as absence".

**Not moved (a choice for review):** PF `LIMITATIONS[6]` ("… Subtraction is a labelled
difference. …") stays true and is not restated, so no PF document's `formulation_basis`
moves and no full-envelope pin moves. The pressure-2 table and its reason enums do not move,
so physics-retained-1 does not either.

Digests (applied through `semantic_contract_tables.py`: the two tables by hand, `derive`,
`apply-u4b`, `repin --from-rev HEAD`, the carriers by their writers):

| Table | Main (bbf384dec = 800fae7e5) | 1b |
|---|---|---|
| preview-physics-2 | e7b93f41… | f41b9d3e… |
| preview-physics-timoshenko-1 | ce49ebe0… | 6f649490… |
| preview-physics-retained-1 | 26dce9c2… | b4377e73… (inherits f41b9d3e…) |
| physics-retained-1 | 040ef9a6… | unchanged |
| pressure-2 | 10455bf3… | unchanged |

Schemas: results.v0.3.schema.yaml 59d405ff… → f4fa46fa…, stress_neutral_export.v0.3
3121f979… → 4dd626d2…, analysis_run.v0.3 d30507f1… → bc5147fd… (its embedded table digests);
`tools/release/schema_manifest.json` regenerated.

### 6.7 Recorded runs (§10.4's registry)

The closed registry (Rust `recorded_tables::VARIANTS`, Python
`compatibility.RECORDED_PRE_RT_A4_TABLES`, desktop `recordedPreRtA4.ts`) gains main's three
moved tables verbatim (`fixtures/results/recorded_pre_rt_a5_1b/`): preview-physics-2 e7b93f41…,
timoshenko-1 ce49ebe0… and preview-physics-retained-1 26dce9c2… (its base the new e7b93f41…
variant), each read under the variant's table. Their rows and limitations are the current
tables', so a statement recorded under them reads as it read then. Main's records and
statements are kept verbatim in `fixtures/persistence/recorded_pre_rt_a5_1b_pf/` (README there;
as at RT-A4, main has no committed timoshenko or successor record, so those identities are
covered by main's verbatim statements on the recorded entries). `repin` protects both
directories (it had rewritten the three new registry digests on its first run; they were
restored by hand and the directories added to its protected prefixes). Selection, scope and
the display rule are §10.4's, unchanged; nothing is migrated or resealed, and no old record
becomes current. Regressions: Rust `result_export/tests/recorded_pre_rt_a5_1b.rs`, Python
`tests/test_recorded_pre_rt_a5_1b_pf.py`, desktop `src/services/recordedPreRtA51b.test.ts`.

### 6.8 Verification

- PP `tests/op1b_difference_basis.rs`, both solver modes, against the independent references
  `validation/references/op1b_difference_basis/` (standard-library generator, exact
  fractions):
  - V-1: closing, both open and both active; Δu, ΔN and Δstop against the closed forms;
    every declared row equal bit for bit to its plain twin and to fl(O − S) of its cited rows;
    evidence equal to an independently written record.
  - V-2: prop lift-off and the active twin.
  - V-4: a plain S − O negates every finite nonzero row; the declared S − O is withheld by G-1
    (a reversed-operand mutant is killed). Case independence is unchanged by 1b (no solve
    change); its seed instrumentation is `op1a_seed_tests`.
  - V-5 negatives: a sustained-only load, a one-ulp magnitude, two identical sustained
    primitives against one, each withheld by G-1, with the dependant withheld whole; two
    against two and identical content under other ids and provenance admitted (an id-comparing
    mutant is killed); G-2 mixed bases withheld, equal bases admitted, G-1 before G-2; the
    member on `mechanics` and `range_envelope` (`…_MISPLACED`) and naming a combination
    (`…_OPERAND_NOT_LOAD_CASE`), blocking; `minuend_final` withheld by its own code alone;
    `own` as absence; EX blocking by name; a blocked operand blocks the document.
  - Readers: condition_use refused with the model; forged evidence (a bit, a unit, missing,
    swapped cases), a forged gate and forged signs refused with the model; the shared corpus
    `fixtures/results/preview_physics_difference_basis_corpus.json` is the producer's output.
- RE `preview_difference_basis_tests.rs` and Python `tests/test_preview_difference_basis.py`
  over the corpus: every document reads with its model and without it; about thirty tampers
  refused by name without the model; the coherent forgery.
  Review r1 adds: a case recurring across two declared records with different contents
  (both readings, `…_BINDING`); counts 0, −1, 2^53, 2^64, `1.0`, `true` and `"1"` refused as
  `…_EVIDENCE` with and without the model, and 2^53 − 1 admitted.
- End to end: `openpipestress-runner read preview-physics`
  (`core/runner/headless/tests/reader_cli.rs`) reads three declared documents with and
  without their model and refuses a forgery inside each record by name
  (`…_COMBINATION_GATE_DERIVATION`); the derivative path (`derive_document` through
  `semantic_contract::for_source`, `result_export/tests/derivative_contract.rs`) derives and
  validates a declared document and refuses a forged one by the same name. Packaging reads its
  source through the same `for_source` (`bindings.rs`) inside a complete package request (audit
  manifest, solver identity, rendered report), so it has no separate entry short of a full
  package; the derivative test drives that function.
- V-3 (friction history) is DEL-04-04:2's disclosure (CLM-012), not a behaviour change.
- Fail-before: on main every declared subtraction is withheld by
  `LOAD_COMBINATION_DIFFERENCE_BASIS_UNSUPPORTED`, so every admission and gate assertion above
  fails there. Mutants run on this branch, each restored after: G-1 always contained (four
  `op1b_difference_basis` tests fail: V-4, V-5 G-1, V-5 G-2 ordering, the corpus); the reader's
  sign check disabled (`option_a_refuses_tampered_evidence_by_name` fails at "evidence for a
  plain subtraction"); the model-bound `condition_use` refusal removed (the condition_use test
  fails: the reader then refuses by `…_DIFFERENCE_BASIS_MODEL` instead, still closed but not
  by its name). Each other reader guard is exercised by its own named tamper.

### 6.9 Follow-on seam: `minuend_final` (designed in §7)

1b adds no held-status solve semantics. The held-status solve is designed in §7 (phase 1, not
built). §7 replaces the seam sketch that stood here and corrects two of its statements:
- **The held solve's K-D5 standing.** The sketch said that the held solve's member bound "is the
  linear certificate against the held system". `confirm_converged_state` cannot supply that in
  general. It establishes a held solve only when the target case's own classifier keeps every held
  state, and otherwise returns `NotEstablished` without a bound (§7.3).
- **The input seam.** The dynamics lane's #97 `NonlinearStateEvidence` binds the case, mode,
  qualification, standing and final states. It lacks the orienting iterate, the support assignment
  (node, DOF, behaviour) and the source's ground offsets. §7.4 adds them as a separate record and
  leaves the #97 lines untouched.

### 6.10 Pricing and registration

Registered at DEL-04-08 `retained-pricing-rules.md` §8.51, on main `573051799` (RT-U4's registered §8.50 candidate
`41142f4a3`), priced source `a82460ad7`. AC-004 is pending.
- **Rules.** The 18 full-file pins whose basis value was a file 1b changes are restated on the merged bytes, each with
  its re-audited premise; S1-ZS drops the deleted push's two entries and RT-A4-D14 pins one push (§8.46's precedents).
  The new PF1B rules (DZ, FZ, RM, BN, L, ID) rest on D1.4's refusal of every difference basis: the declared arms, the
  derivation's records and the model-free reader's claims are zero on D1, and the by-name call graph's false edges into
  the modules' private functions are removed. The analysis-only record's unmapped headers, unclassified arguments and
  text rise are all resolved by these rules.
- **Profile.** Against §8.50's tree 6 coefficients rise and none fall: STATICS, +7,116 B, +12 nodes and +48 slots (the
  three preview tables' combination policy v3 in four static owners). Every phase moves +17,484 B in both modes. W3
  stays the maximum, 2,726,118,449 B (dense) below ⌊0.9M⌋; the threshold and M are unchanged.
- **Records.** Reviewed inputs #4, #5 and #12 (the three preview tables) take the restated bytes, which clears
  `reviewed_inputs_bind_the_lock_and_the_reader_statics` and the identity inventory by registration. `PINNED_RECORD`
  and the challenge constants are re-pinned. No retained successor or carrier needed regeneration.

---

## 7. Held-status solve (`minuend_final`)

**Status: design, phase 1; not built; for independent review before any product code.** The base
is `claude/rt-a5-minuend-final` at `b8f3837e8`: RT-A3 P's reviewed candidate `78580da7e` merged
with main `b32a17761`, which carries 1b (#107). Line numbers refer to this base. The risk class is
numerical meaning, persistence and contract. Nothing here records acceptance, issuance or release.
It decides none of these, which stay reserved: eligibility policy (b), the §4 certificate's
premises, the #97 seam, P's step and the confirmation.

**Terms.**
- The **source** case is the minuend, or for hanger sizing the design operating case.
- The **target** case is the subtrahend, or for hanger sizing the weight case.
- The **held status** is the source's `final_states`, one `SupportStateRecord` per nonlinear
  support.
- The **held system** is the target's own loads, materials, conditions and imposed motion, with
  each nonlinear support's boundary formed from its held state instead of being iterated.
- A **held solve** is one solve of the held system.

### 7.1 The function and its consumers

This is the one product function of §0 ("`minuend_final` as a callable mechanism"). It is built in
two layers, and neither copies the loop.
- **NI** `solve_held_status_iteration` is a new `pub` function placed beside the step
  (`lib.rs:1559–1780`) and called by nothing else in NI. It:
  1. assembles the target's system through `LoopSystem::assemble` (`:2334`), the loop's own path;
  2. forms the held boundary with `active_boundary` (`:2108`);
  3. calls `solve_iteration_with_sliding_friction_evidence` once, with the held states and the
     orienting iterate;
  4. classifies that one solve with the target's own classifier (§7.3);
  5. issues the result's completion receipt;
  6. calls `confirm_converged_state`.

  There is no iteration, no all-active recovery and no second step. `LoopSystem`, `BoundaryState`,
  the step, the loop, `confirmation.rs` and `structural_adapter.rs` are unchanged.
- **PP** `held_status::solve_case_at_held_status` is the one product entry. It checks the source
  binding (§7.4) and solves the target through `solve_prepared_load_case`'s own assembly and
  publication, with the NI call replaced by the held wrapper (§7.11). It returns a typed held solve
  that no consumer can mistake for a case solve.
- **Consumers.**
  - The declared subtraction with `subtrahend_support_state: minuend_final` (§7.5–§7.6).
  - Later, DEL-04-11's `weight_support_state: design_operating_final`, through the contract of
    §7.7. That integration is not part of this work.

### 7.2 (a) Frozen-constraint semantics, per behaviour

The held status fixes each support's **state**, never its displacement or force. Prescribed values
and loads are always the target's own. This is the static, history-free model of R-2: sticking is
measured from the target's ground, as in every case solve. A history model that holds the source's
displaced position is DEL-04-04:2's, and §7 does not add one.

| Support, held state | DOF | Prescribed value or applied force | Classifier test at the held solve (exact binary64, the loop's own rules) |
|---|---|---|---|
| One-way or lift-off, Active | prescribed | u_d = g_s^T, the target's ground (0 unless the target moves it) | kept iff the reaction has the support's sense (`state_from_sense`); a reaction of the other sign is a **tensile held reaction** |
| One-way or lift-off, Inactive (lifted off) | free | none | kept iff no penetration past g_s^T (`state_from_penetration`) |
| Gap, Active (closed) | prescribed | u_d = σ(c + σ·g_s^T), required exact by TwoSum (`exact_gap_closing_position`); an inexact position is refused | kept iff the stop bears (closing in +: r ≤ 0; closing in −: r ≥ 0) |
| Gap, Inactive (open) | free | none | kept iff the gap does not close (u_d short of ±c from the ground) |
| Friction, Sticking | prescribed | u_t = g_t^T | kept iff \|r_t\| ≤ μ·N^T, with N^T the target's normal (below) |
| Friction, Sliding, explicit normal | free | φ = −dir^S·μ·\|N\|, where N is the model's explicit input (the same in both cases) | kept iff φ·δ_t < 0 and r_t·δ_t < 0 at the held solve (`resolve_sliding_friction_states`) |
| Friction, Sliding, derived normal | free | φ = −dir^S·μ·β·R_n(u^T), solved simultaneously by the step's coupling, with R_n the target's own reaction at the normal-source DOF and β the source's normal sign | kept iff the slip test above holds and the target's normal keeps sign β (`derived_normal_branch_admissible`) |
| Friction, Sliding, μ = 0 | free | none | kept iff δ_t ≠ 0 |
| Friction, Inactive | free | none | kept iff N^T ≤ 0 |
| Friction, Active (not a friction state) | — | refused at binding (§7.4) | — |

**Not held: springs, constant-effort supports and linear supports.** These have no state.
- Ground springs keep their stiffness, and their bases move only with the target's motion (R-c).
- A constant-effort support's force is part of the target's own ledger in every case (DEC-049).
- Rigid restraints are prescribed at the target's g_b (R-a, R-d).
- No base restraint and no spring is added or removed.

**The friction direction (the tangent and slip-direction caveat).** dir^S is the slip sense of the
**source's** final iterate: `sliding_direction` on the orienting iterate, taking u_t − g_t if
nonzero and otherwise the sign opposite the reaction.
- **Why the source's final iterate.** At a qualified Sliding state the final iterate's slip has the
  sign of the direction that iterate's force used: tangential admissibility requires
  φ·δ_t < 0 with φ = −dir·μN and μN > 0. For a derived normal, the final normal has the sign of
  the assumed branch, since admissibility requires β·R_n ≥ 0 and a zero normal classifies the
  support Inactive. So the source's final iterate orients the held step as the loop's own last
  iterate did. Regression H-10 checks this bit for bit (§7.9).
- **Which ground.** The step measures slip on that iterate against the target's g_t
  (`sliding_friction_candidates`, `:1883`), but the source slipped against its own g_t^S. A held
  sliding support is therefore admitted only when g_t^S and g_t^T are bit-equal (§7.5); a ground
  that differs between the cases is refused, never corrected.
- **The tangent is the support's own DOF axis.** Friction acts on one translational DOF, so no
  in-plane slip direction exists to hold.
- **The target's motion can oppose the held sense.** Holding the sense does not hold the motion.
  The held force then pushes with the target's motion, and the classifier releases the support
  (H-5).

**The derived normal: which case's normal applies, and why.** The **target's**, solved in the same
step (`:1607–1700`).
- Coulomb's law at a held Sliding status bounds the tangential force by μ times the normal that
  the held system itself carries. Holding the status means holding "sliding, in sense dir^S", not
  a force magnitude.
- Freezing the source's normal would add an operating-derived load to the target. That would break
  requirement 2 (the target's actual loads) and the content premise of G-1.
- The source supplies only β, the sign assumed for the branch. If the target's normal has the
  other sign, the branch is inadmissible and the support is released.
- H-6 and H-6b distinguish the target's normal from the frozen source normal (a killed mutant).

### 7.3 (b) What `confirm_converged_state` can and cannot establish

**What it requires.** `confirm_converged_state` (`confirmation.rs:79–96`, through
`confirm_with_applied`) requires `product_selected_state_is_qualified` (`lib.rs:1029–1038`). That
function needs two things:
- **The private completion receipt** (`ProductCompletion`, `:372–381`) binding the input, springs,
  mode, stiffness, final force, free DOFs, prescribed map and output.
- **`product_final_state_conditions`** (`:1161–1283`). Every non-friction, non-gap support's final
  state must be its classifier's verdict at the returned solve, and the step must be `converged`:
  zero changed supports, zero residual, admissible friction branches and Passed equilibrium rows.

**How the wrapper meets them.**
1. The wrapper forms `held_input`: the caller's input with `initial_states` replaced by the held
   records, which are the states the step actually ran from.
2. It computes the step's single iteration record with the loop's own calls (once each):
   - `structural_adapter::scrutinize_gaps` (`structural_adapter.rs:3106`, with the standard budget
     and iteration 1);
   - `build_trial_states` (`:3078`);
   - `resolve_sliding_friction_states` (`:1782`, not deferred: the orienting iterate exists);
   - `exact_gap_iteration` (`structural_adapter.rs:3430`) or
     `evaluate_active_set_iteration_with_resolved_friction_states`, with prior = held;
   - `residual_observation` with no previous iterate. The source's iterate is never used as a delta
     reference.
3. It sets `converged` by the loop's predicate (`:921–927`) with its two loop-only terms fixed:
   `recovered_path = false` and `sliding_force_deferred = false`.
4. It issues the receipt as the loop's completion block does (`:998–1018`) over `held_input`.
5. It calls `confirm_converged_state(&held_input, springs, &result, force, curved, members,
   mode)`. The call runs only when no load-row finding exists and the iteration's structural report
   is Passed, PP's own rule; otherwise the outcome is `NotEstablished { "confirmation attempt: not
   run" }`, as in kd5 §2.4 item 1.

**Why this is honest.** Every field of the result and the receipt is what the step and the
classifier computed. `converged = true` means exactly this: the target's own classifier, applied to
this held solve, keeps every held state. Nothing in the confirmation is bypassed or changed.

**The seam fact for Agent 0.** Step 2 composes the loop's post-step calls once more
(`lib.rs:878–1018` is the loop's composition, which stays untouched). It is not an iteration, but
it is a second composition of the loop's per-iteration bookkeeping, which requirement 1's
"do not copy the loop" and "do not change the loop body" leave no other way to reach. There are two
options:
- **S1 (recommended).** Compose in the wrapper, guarded by the reproduction regression H-10: held
  at a case's own final states with its own penultimate iterate must reproduce the loop's final
  iteration, result, receipt, qualification and confirmation bit for bit, in both modes, over every
  committed nonlinear model. Any drift in the composition fails it.
- **S2.** Extract the post-step block into one crate-private function that the loop and the
  wrapper both call. This changes the loop body, so it needs Agent 0 to lift that clause, with
  dense and sparse byte identity of every committed nonlinear output.

**Outcomes and the standing each publishes.** No outcome publishes
`NonlinearStateStanding::Confirmed`, `NONLINEAR_CONFIRMED_EVIDENCE_LINE` or the word "confirmed".
No outcome claims an admissible sustained nonlinear state, a loading history or uniqueness.

| Outcome | When | Held rows | Published standing (held evidence `standing.kind` and sentence) |
|---|---|---|---|
| **E** (established) | non-gap; the classifier keeps every held state; the confirmation returns `Established` | Stage B's certified values, as a confirmed case publishes them, with K-D5's member bound against the **held system** | `established`: "Held support status of <source>: the held system (this case's loads with each nonlinear support held at <source>'s final status) is established at 128-bit precision by one formation-checked solve, and each held support's condition holds at this case's loads with a margin larger than its bound. This is a held-constraint comparison: it is not this case's own support state, and it claims no admissible sustained state, loading history or uniqueness." |
| **N** (kept, not established) | non-gap; the classifier keeps every held state; the confirmation returns `NotEstablished` (weakly active, Sensitive, k > 1 coupled friction, a load-row finding, …) | binary64 iterate values; member rows are binary64 recoveries | `not_established` with the confirmation's own reason: "… not established: <reason>; the held status is kept by this case's classifier; rows are inspection evidence" |
| **R** (released: an inadmissible held reaction or separation) | the classifier changes at least one held state: a tensile held reaction at a held-active one-way or lift-off support; penetration at a held-inactive one; a closed gap pulled; stick capacity exceeded; a slip sense contradicted; a derived-normal branch inadmissible | binary64 iterate values; no bound (the confirmation returns "the converged state is not qualified") | `not_established`, plus a warning `HELD_STATUS_RELEASED` naming each released support, its held state, the classifier's state and the deciding quantity (e.g. "support:PROP held active, reaction −512 N against its positive sense"). It publishes with this standing and is never withheld (R-5) |
| **G** (gap-bearing) | any gap support (R1) | binary64; strict-gap evidence as the case path states it | `gap_inspection`: "gap supports keep their recovery-basis branch (R1); not established", plus `HELD_STATUS_RELEASED` if released. The source must still be `Qualified` (§7.4), so R1 is not bypassed |

**What the confirmation can and cannot establish here.**
- **E is genuine.** kd5 §2.3's claim holds for the held system: u\* with the held state satisfies
  the held contact and Coulomb conditions strictly. The product wording still keeps two separations
  from the target's own state:
  - The held state is selected by the source, not by the target's own solve, which is published
    separately as the case's rows.
  - Friction admits several states (R-2, c3).
- **For frictionless one-way, lift-off and gap supports**, with the stiffness positive definite on
  the base restraints alone, the static contact problem has one solution: a strictly convex energy
  on a convex set. A held frictionless status that differs from the target's own is therefore never
  kept. It is released (R) and carries no bound, up to the classifier's exact decisions at weakly
  active supports.
  - In frictionless models E arises only when the held status equals the target's own. The
    difference then equals 1b's O − S, which H-1b checks bit for bit.
  - With friction, E can arise with a held state that differs from the target's own.
- **The confirmation cannot supply a bound for a released held state.** Its certificate and its
  contact predicate are conjoined in `certify_state` (`confirmation.rs:973–1116`). The member bound
  is returned only inside `Established`, and qualification fails first in any case. So §6.9's "the
  linear certificate against the held system" is not available for R.
  - Supplying one would need a new confirmation attempt that returns Stage B's bound with the
    per-support conditions reported rather than conjoined. That is a `confirmation.rs` change owned
    by DEL-04-01/kd5, and it is outside this work.
  - A second path, certifying R's held system as a linear "held twin" on the linear route, would
    break "one helper" for systems with a derived-normal sliding support, whose force is coupled.
    It is not proposed.

**Standing without §4.** Every O − S_held row is derived inspection evidence, as every subtraction
row is today (§1.1, "Standing of combination rows"). The held operand's E standing does not
establish the difference.

**With §4, once merged.** As written on `claude/rt-a3-cert`, §4 has these premises:
- an eligible operand is a **load case** with certified rows;
- `certificate_operands` is keyed by load-case id.

A held operand is not a load case. Under those premises a `minuend_final` combination must be
`not_assessed`, and must never be assessed with the subtrahend case's own bounds, which describe a
different system (§7.11, overlap O-C2). Admitting an E held operand to §4 is a change of the
certificate's premises, which is the certificate owner's decision. It is not made here.

**Numerical eligibility.** Policy (b) is unchanged and is RT-A3's and Agent 0's decision.
Combination rows never count, and held solves are combination operands.

### 7.4 (c) The source-binding record and its refusals

**What the #97 seam carries.** `NonlinearStateEvidence` (`lib.rs:25486–25493`) has `case_id`,
`solver_mode`, `converged`, `qualification` (`Qualified` | `InspectableUnqualifiedGap`), `standing`
(`Confirmed` | `NotEstablished { reason }`) and `final_states`. It is formed only on a completed
case solve that converged and passed either qualification branch.

**What it lacks.**
1. The **orienting iterate**: the source's final `NonlinearFrameIteration`, of which the step reads
   only `displacements` and `reactions`.
2. The **support assignment**: each held support's id, node, DOF, behaviour, gap bits and μ bits.
3. The source's **ground offsets** g_s^S.

**The additive record (PP, crate-private, untouched #97 lines).**

```rust
/// RT-A5 minuend_final (operating-case §7.4): what a completed source case lends a held solve.
/// Formed only by `capture_held_status_source`, from the same completed construction as
/// `nonlinear_state`; no other constructor, no deserialization.
struct HeldStatusSource {
    case_id: String,
    solver_mode: PreviewSolverMode,
    supports: Vec<NonlinearSupport>,      // the source input's nonlinear supports, input order
    ground_motion: Vec<(String, f64)>,    // the source input's g_s
    orientation: NonlinearFrameIteration, // the source's final iteration, as the loop returned it
}
```

- **Where it lives.** A new `LoadCaseSolve` member `held_status_source:
  Option<Box<HeldStatusSource>>` (8 B), formed after the #97 capture (`lib.rs:5978–5985`, unchanged)
  by `capture_held_status_source(nonlinear_state.as_deref(), built, &ground_motion,
  selected_nonlinear.as_ref(), wanted)`.
- **When.** `wanted` holds only when the case is the minuend of some `minuend_final` combination,
  so every other document, D1 included, carries `None`. It is `None` whenever `nonlinear_state` is
  `None`.
- **Same-run provenance.** The type is crate-private and constructed in one place inside the
  invocation's own case solve, so a held source from another run, document or case cannot be
  supplied.

**The binding checks at use.** Each refusal is named and withholds the combination; none blocks
the document.

| Check | Refusal |
|---|---|
| The minuend's `nonlinear_state` and `held_status_source` are present (absent in a model without nonlinear supports, and in every early-return construction) | `HELD_STATUS_SOURCE_UNAVAILABLE` |
| `nonlinear_state.qualification == Qualified`. An `InspectableUnqualifiedGap` source has nothing to lend (R1). The source's standing (`Confirmed` or `NotEstablished`) is carried into the record, not required | `HELD_STATUS_SOURCE_UNQUALIFIED` |
| Both records' `case_id` is the combination's `minuend_id`; both `solver_mode`s are the invocation's | `HELD_STATUS_BINDING` ("… source case <id> / mode <m> …") |
| `final_states` has exactly one record per target nonlinear support: a missing, extra or repeated id is named | `HELD_STATUS_BINDING` ("… support <id> missing / not in the target / repeated") |
| Each source support equals the target's field for field (node, DOF, behaviour with sense or closing direction, gap bits, μ bits, explicit normal bits or derived source DOF) | `HELD_STATUS_BINDING` ("… support <id> <field> differs") |
| No friction support is held `Active`; each state is one of its behaviour's states | `HELD_STATUS_BINDING` ("… support <id> state <s> is not a state of its behaviour") |
| `orientation.active_set.states == final_states`, and both vectors are 6n long | `HELD_STATUS_BINDING` ("… orienting iterate is not the source's final iterate") |
| Each held Sliding friction support has a nonzero slip sense in the orienting iterate | `HELD_STATUS_BINDING` ("… support <id> has no slip sense") |
| Each held Sliding friction support has a bit-equal ground in source and target | `HELD_STATUS_SLIDING_GROUND_DIFFERS` |

NI's own validation (`normalized_initial_states`, `validate_input`, `active_boundary`) refuses the
same malformed assignments independently, as `InvalidInput`. PP maps any NI error to
`HELD_STATUS_SOLVE_FAILED` with the error text.

**The target side.** The held solve uses the target's own lowered loads, its resolved
`condition_use` (`ResolvedCase`), its modulus-basis state, the base motion, ground motion and
spring-base motion. G-1 and G-2 apply unchanged. The held record names the target's id and modulus
key, and its primitive-content multiset is 1b's own (`difference_basis_derivation`).

### 7.5 (d) Admitted scope and named refusal codes

**Admitted initially.**
- The pressure-free route (preview-physics-2 and its timoshenko variant; no pressure contract).
- A `result_state_subtraction` with `difference_basis: operating_minus_sustained` and
  `subtrahend_support_state: minuend_final`.
- Models with nonlinear supports.
- Both solver modes.
- Every behaviour of §7.2, gaps under R1 (outcome G). Gaps rest on the source being strict-gap
  `Qualified`, so most gap sources are withheld by `HELD_STATUS_SOURCE_UNQUALIFIED`.

**Gate order** for such a combination, all warnings that withhold it:
1. `LOAD_COMBINATION_OPERAND_WITHHELD` (never met; operands are load cases);
2. G-1 `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`;
3. G-2 `EXPANSION_BASIS_MODULUS_MIXED`;
4. then the held gates in this order:
   - `HELD_STATUS_SOURCE_UNAVAILABLE`;
   - `HELD_STATUS_SOURCE_UNQUALIFIED`;
   - `HELD_STATUS_BINDING`;
   - `HELD_STATUS_SLIDING_GROUND_DIFFERS`;
   - `HELD_STATUS_SOLVE_FAILED`, which covers an NI refusal, a singular held system (for example a
     support the source lifted off was the target's only stability) and an inexact held gap
     position.

The held solve runs only after G-1 and G-2 pass. Its dependants are withheld whole by the existing
`LOAD_COMBINATION_OPERAND_WITHHELD`.

**Disclosed, not withheld.**
- `HELD_STATUS_RELEASED` (warning), outcome R or G.
- The standing record, outcomes E, N, R and G.

**Unchanged refusals.**
- EX blocks first with `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED` at the solve and the
  applier. It is never lifted silently.
- `…_MISPLACED` and `…_UNKNOWN` are the validator's.
- Absence and `own` stay own, and nothing infers `minuend_final`: not the importer, the CAEPIPE SI
  writer, the desktop template or any default. The retained routes keep declining the member
  (`FamilyFact::CombinationMember`), and the SI writer keeps refusing it.

**Removed.** The interim `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED` withholding is
removed at the PF solve (`document_members.rs:826–835`) and at the applier (`operation_applier
lib.rs:6288–6289`). The constant is kept for the history of its refusals, as 1b kept
`…_DIFFERENCE_BASIS_UNSUPPORTED`.

### 7.6 (e) Published evidence and readers

**Rows.**
- The held solve's combined-kind rows (the linear state kinds the subtraction reads) publish as
  **held-operand rows under the combination**:
  - basis_ref `{ref_type: "combination", ref_id: <combination>}`;
  - id `<combination-qualified>:held:<base id>`;
  - `metadata.basis` `held_status_operand`, a new token in results.v0.3's and
    stress_neutral_export.v0.3's closed basis enum;
  - their kind's own sign convention, with the clause in §7.8 item 5.
- The O − S_held difference rows cite the minuend's case row and the held-operand row, with the
  sign text of §7.8 item 5. Their `metadata.basis` stays `explicit_user_result_state_subtraction`.
- No held value appears among the target case's rows, which are byte-identical to the document's
  without the combination. Nonlinear state rows are never combined. The held states, reactions and
  verdicts are in the evidence record below.
- **Alternative (for review).** Publish no held-operand rows, cite only the minuend's row, and carry
  the held values in evidence. That leaves the difference rows with an uncited operand and grows the
  evidence with the row count, so it is not recommended.

**Contract evidence.** `contract_evidence.held_status_derivation` is a new optional member beside
`difference_basis_derivation` in the closed `PreviewPhysicsContractEvidence`. It is present only
when a `minuend_final` combination reached the held gates, so every other document keeps its
bytes. There is one record per such combination, in gate order:

```json
{"combination_id": "comb:o-s-held", "subtrahend_support_state": "minuend_final",
 "source": {"load_case_id": "case:O", "solver_mode": "sparse_interactive",
            "qualification": "qualified", "standing": "confirmed"},
 "target": {"load_case_id": "case:S", "modulus_basis_key": null},
 "outcome": "admitted",
 "held": [{"support_id": "support:PROP", "node": "node:T", "dof": "UZ",
           "behavior": "one_way", "sense": "positive_reaction",
           "held_state": "inactive", "classified_state": "active",
           "ground_bits": "0000000000000000", "displacement_bits": "…", "reaction_bits": "…"},
          {"support_id": "support:SHOE", "node": "node:T", "dof": "UX", "behavior": "friction",
           "held_state": "sliding", "classified_state": "sliding",
           "friction": {"coefficient_bits": "…", "normal_source": "explicit", "normal_bits": "…",
                        "slip_sense": 1, "applied_force_bits": "…", "tangential_bits": "…"}}],
 "standing": {"kind": "not_established", "reason": "…"}}
```

- `outcome` is `admitted` or the held withholding code. A withheld record carries `source` and
  `target` and an empty `held` list.
- Values are raw SI binary64 bits, the trial quantities the classifier read: δ relative to the
  target's ground, and the reaction as reported. The model-free classifier re-derivation is
  therefore exact.
- `held` is in support-id order.
- The record grows with the number of nonlinear supports and is bounded by the model's own limits.
  No new threshold is added.

**Readers** (RE `preview_held_status.rs`, called from `preview_combination_gates::check` as 1b's
reader is; Python `tests/verifiers/preview_held_status.py`):
- **Without the model** (option A, as §6.5).
  - **Shape.** Closed shape, canonical order and finite bits (`…_HELD_STATUS_EVIDENCE`).
  - **Bindings** (`…_HELD_STATUS_BINDING`, each named):
    - the combination is a declared subtraction with `minuend_final`;
    - source = minuend, target = subtrahend, and the source mode is the document's;
    - `held` covers exactly the minuend's published `nonlinear_support_active_set_state_code` rows,
      by support, with each `held_state` equal to its published code and its node and DOF equal to
      the row's location (a missing, extra or forged entry is refused by name);
    - every held-operand row cites its combination and no case.
  - **Classifier re-derivation** (`…_HELD_STATUS_DERIVATION`). Each `classified_state` is
    re-derived from the published bits by §7.2's tests. `HELD_STATUS_RELEASED` is present exactly
    when some state differs and names exactly those supports. `standing.kind` is `established` only
    when none differs and no support is a gap. A gap-bearing record is `gap_inspection`.
  - **Rows** (`…_HELD_STATUS_ROW`). The difference rows state §7.8 item 5's sign and no other row
    states it. The held-operand rows have the new basis.
  - **Scope.** This verifies the record's internal derivation and binding, not its truth to the
    absent model or the held solve itself. A coherent forgery passes, and the model-bound reading
    refuses it (as §6.5).
- **With the model.** First the model-free checks. Then:
  - each support's node, DOF, behaviour, sense, gap, μ and explicit normal must equal the model's;
  - each ground is re-derived from the two cases' authored imposed motion;
  - the sliding-ground and binding refusals are re-derived.

  Where either case uses `condition_use`, the reading is refused as
  `SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`, as 1b does. It never falls back to the
  model-free reading.
- **The readers do not re-run the held solve** (PRD §12.8). They do not re-derive the held
  solve's values, which PF readers do not do for any subtraction (§6.5's pre-existing limit), or its
  bound.
- **Slice 1a's operand standing.** In 1a's record (`append_operand_standing`, `preview_physics.rs:1359`)
  the subtrahend operand of a `minuend_final` combination is the held solve, not the subtrahend
  case. It states `support_state: "held_at_minuend_final"` and the held standing, and the reader of
  `preview_combination_operands.rs` admits that member. 1a's `…_OPERAND_STATES_DIFFER` notice is
  not emitted for the held pair, whose states agree by construction; `HELD_STATUS_RELEASED` carries
  the comparison instead.
- **Interim removal.** The RE and Python `kept_combinations` (`preview_difference_basis.rs:360–378`,
  `preview_difference_basis.py:287–297`) stop removing `minuend_final` combinations. Both readers'
  admitted gate-code lists gain the held codes (`preview_physics_evidence.rs:60–70`,
  `preview_physics_evidence.py:80–86`, `preview_combination_gates.py:269`).

### 7.7 (f) The hanger-facing callable contract

This is for DEL-04-11's O-1. Hanger integration is not part of this work.

```rust
/// operating-case §7: `target` solved with every nonlinear support held at `source`'s final
/// status. One step of the loop's own unit and one confirmation attempt; never iterated.
pub(crate) fn solve_case_at_held_status(
    context: &CaseSolveContext<'_>,   // the invocation's model, basis states, boundary, springs and
                                      // solver mode, as its case solves used them
    target: &PreviewLoadCase,         // the target case as authored (resolved by the context)
    source: &HeldStatusSource,        // from the source case's LoadCaseSolve in this invocation
) -> Result<HeldCaseSolve, HeldStatusRefusal>;

pub(crate) struct HeldCaseSolve {
    pub results: Vec<ResultItem>,      // the held system's ordinary rows; never a case's rows
    pub record: serde_json::Value,     // §7.6's held_status_derivation record
    pub standing: HeldStanding,        // Established | NotEstablished { reason } | GapInspection
    pub released: Vec<ReleasedSupport>,// support id, held state, classified state, quantity
}
pub(crate) struct HeldStatusRefusal { pub code: &'static str, pub message: String }
```

**Semantics the hanger owner relies on.**
- **Held loads.** W is formed from the held system's reactions at the candidates. These are the
  weight case's own loads with every nonlinear support held at S_O,design's final status (§7.2).
- **Same model and run.** The source must come from a solve of the **same built model** in the
  **same invocation**: the same nonlinear supports, node and DOF. A sizing derived model that
  changes the nonlinear support set must solve its own source, or the call refuses with
  `HELD_STATUS_BINDING`.
- **Refusals.** Every refusal is a `HeldStatusRefusal` naming one of §7.5's codes. Sizing maps it to
  `HANGER_SIZING_DERIVED_SOLVE_FAILED` carrying that code and the derived solve's name (its §4
  rule).
- **Releases.** `released` non-empty is outcome R or G: a held reaction the support cannot supply
  ("a tensile reaction at a unilateral support"). Sizing must disclose it per its §5 ("the result
  discloses it as such") and never present W as the weight case's admissible state.
- **Standing.** `HeldStanding::Established` is outcome E's sentence and never "confirmed". It says
  nothing about the weight case's own state, which sizing publishes separately.
- **Mode.** The solver mode is the context's. Every derived solve uses the request's mode, as
  hanger §2's `solver_mode` already requires.

### 7.8 (g) Old → new texts the change will touch

Phase 2 applies these after review, with the code. Each needs the independent review the
delegation requires.

1. **DEL-05-02 ScopeOfWork, DEL-05-02:3.**
   - **Old:** "*Slice 1b (candidate, branch `claude/rt-a5-slice1b`, not yet integrated; reviewed):*
     … EX refuses the member by name. The minuend_final slice is open (seam:
     `Work/operating-case.md` §6.9)."
   - **New:** "*Slice 1b (#107):* … EX refuses the member by name. *minuend_final (candidate, branch
     `claude/rt-a5-minuend-final`):* the subtrahend is solved with each nonlinear support held at the
     minuend's final status by one held-status function (`Work/operating-case.md` §7), published
     under the combination with its held standing and held-status evidence, read with and without
     the model; hanger sizing's `design_operating_final` is not integrated."
2. **DEL-05-02 `deliverable.yaml`.**
   - **`code_paths`** gains `core/product_physics/src/held_status.rs`,
     `core/product_physics/tests/op_mf_held_status.rs`,
     `core/reporting/result_export/src/preview_held_status.rs` (and `_tests.rs`),
     `tests/verifiers/preview_held_status.py`, `tests/test_preview_held_status.py` and
     `validation/references/op_mf_held_status/**`.
   - **The `from: DEL-04-07` condition** appends: "It also needs DEL-04-07's held-status step
     (`nonlinear_integration::solve_held_status_iteration`), one step of the assembled loop's unit at
     a given support-state assignment, receipt-bound and followed by one confirmation attempt, for
     `minuend_final` (`Work/operating-case.md` §7)."
   - No other declaration moves.
3. **preview-physics-2 `combination_policy`** (retained-1 by hand, timoshenko-1 by `derive`).
   - **`policy_id`:** "PREVIEW-PHYSICS-1-COMBINATION-v3" → "…-v4". The meaning grows: a v3 consumer
     must not read a v4 held subtraction as own-state.
   - **`result_state_subtraction`.**
     - **Old tail:** "; a subtrahend_support_state minuend_final is withheld before the gates
       (LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED) and own behaves as absence".
     - **New tail:** "; own behaves as absence; with subtrahend_support_state minuend_final the
       subtrahend is the subtrahend case's loads solved once with each nonlinear support held at the
       minuend's final status (a held-constraint comparison, which can need a reaction a unilateral
       support cannot supply and is not an admissible sustained state), published as
       held_status_operand rows under the combination, never as the case's rows; its rows' sign
       convention reads \"<item 5>\" and contract_evidence.held_status_derivation states the held
       assignment, each support's classified state and the held standing".
   - **`difference_basis_gates`** appends, in order:
     - HELD_STATUS_SOURCE_UNAVAILABLE: "the minuend case has no converged nonlinear support state
       to lend";
     - HELD_STATUS_SOURCE_UNQUALIFIED: "the minuend's converged state is not qualified (an
       inspectable unqualified gap state)";
     - HELD_STATUS_BINDING: "the minuend's state does not bind to the subtrahend's nonlinear
       supports one for one by id, node, DOF and behaviour, or its orienting iterate is not its final
       iterate";
     - HELD_STATUS_SLIDING_GROUND_DIFFERS: "a support held sliding has a different ground offset in
       the two cases";
     - HELD_STATUS_SOLVE_FAILED: "the held solve was refused or singular".
   - **`gate_precedence`** appends "; a minuend_final subtraction then takes the held-status gates in
     list order after G-2".
4. **PF `LIMITATIONS[6]`.** **Not moved (recommended, as 1b):** "Subtraction is a labelled
   difference." stays true. The alternative appends "A minuend_final subtraction is a held-constraint
   comparison." That moves every PF document's `formulation_basis` and every full-envelope pin. It
   is a choice for review.
5. **Row sign text (new `held_status::SIGN`):** "positive value follows explicit user result-state
   subtraction (minuend minus subtrahend) of matching source result sign conventions; operating
   minus sustained difference at the operating support status: the minuend case's own solved state
   minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final
   status, a held-constraint comparison that can need a reaction a unilateral support cannot supply
   and is not an admissible sustained state, under the static, history-free friction model; derived
   from case results; not an equilibrium state". The held-operand rows' clause: "held-status operand
   of <combination>: the subtrahend case's loads solved with the minuend's final support status
   held; not the subtrahend case's result".
6. **DEL-02-01 `model-document-0.9.0.md`.**
   - **§2.2.**
     - **Old:** "Interim until its slice: `own` behaves as absence; the solve withholds a
       `minuend_final` combination with the warning
       `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`, and the applier refuses authoring it."
     - **New:** "`own` behaves as absence. On the pressure-free route the solve forms a
       `minuend_final` combination by DEL-05-02 `operating-case.md` §7 and the applier authors it; EX
       refuses the declared basis by name first."
   - **§3.8 row.**
     - **Old:** "| `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED` … | solve; applier |
       RT-A5's `minuend_final` slice (DEL-05-02 `operating-case.md` §5) |"
     - **New:** "… | solve; applier | RT-A5's `minuend_final` slice (DEL-05-02 `operating-case.md`
       §7); removed by it on PF |".
7. **Applier** (`operation_applier lib.rs:6276–6293`).
   - The rustdoc's "a `minuend_final` subtrahend support state stays refused until its solve is
     built (`own` behaves as absence)" becomes "a `minuend_final` subtrahend support state is
     authored on the pressure-free route (operating-case §7); `own` behaves as absence".
   - The refusal arm is removed.
8. **Docs.**
   - **`docs/theory/load_combinations.md:149–151`.**
     - **Old:** "and a `minuend_final` subtrahend remains withheld until its own slice."
     - **New:** "and a `minuend_final` subtrahend is solved with the minuend's final support status
       held, a held-constraint comparison that can need a reaction a unilateral support cannot supply
       and is not an admissible sustained state."
   - **`docs/user_guide/index.md:491–492`.**
     - **Old:** "a `subtrahend_support_state` of `minuend_final` stay refused at authoring;"
     - **New:** "stay refused at authoring (a `subtrahend_support_state` of `minuend_final` is
       authored without a pressure contract);"
   - **`docs/user_guide/index.md:537`.**
     - **Old:** "`subtrahend_support_state: minuend_final` is still withheld, and"
     - **New:** "With `subtrahend_support_state: minuend_final` the subtrahend is solved with the
       operating case's support status held (a held-constraint comparison, disclosed when a held
       support would need a reaction it cannot supply), and".
9. **`docs/RELEASE_TARGET_v1.0.md` RT-A5 status, "`minuend_final` unbuilt".** Agent 0's reserved
   wording, not edited here. Proposed: "`minuend_final` (candidate): held-status solve
   (`operating-case.md` §7)".
10. **This document's stale status lines on this base** (applied with phase 2).
    - **Line 3.**
      - **Old:** "slice 1b built on branch `claude/rt-a5-slice1b` (§6, candidate, not yet
        integrated; …); … `minuend_final` unbuilt (§6.9 seam)"
      - **New:** "slice 1b integrated (#107, §6); … `minuend_final` designed (§7)".
    - **§6 opening.**
      - **Old:** "Candidate: not integrated, not independently reviewed."
      - **New:** "Integrated by #107 (`b32a17761`) after review r1 and its focused confirmation."
    - **§6.10.**
      - **Old:** "AC-004 is pending."
      - **New:** "AC-004 is recorded (seven witnesses and the isolated challenge, 7/7 on the
        registered binary at `d44e5c538`; `61c6214a2`)."
    - **§0 rows R-1 and §5 B, and the §1.4 member row.** Each says option 2 "remains R-1's
      remaining item for the owner" / "(later, only if the owner chooses §5 option 2)". Proposed
      status addition, leaving the decision text unchanged: "Status: the owner-accepted release
      target (2026-10-28) carries it as RT-A5 Should; built as §7's held-status solve." The §0
      rows are Agent 0's decision records, so the wording is Agent 0's.
11. **DEL-04-11 `hanger-sizing.md` O-1.** The text "whose contract that lane sends when drafted"
    is the hanger owner's. §7.7 is that contract, and it is sent, not edited here.

### 7.9 (h) Reference plan

The references live in `validation/references/op_mf_held_status/`. The generator,
`generators/held_status_reference.py`, uses the standard library only (exact `fractions.Fraction`),
reads no product code and uses invented values. The layout follows op1b's (README, SHA256SUMS,
`python3 -I … [--write]`).

**Common models.**
- **The V-1 bar.** Along X, anchored at x = 0, L = 4 m, k = EA/L = 2²⁰ N/m, α = 2⁻¹⁶ /K, so
  δ_T = α·ΔT·L.
- **The V-2 cantilever.** s\* = 5PL³/(48EI) = 2⁻¹⁰ m, in units of P and PL, as op1b.
- **Friction.** μ = 0.25.

**Case references, with closed forms.** S is the target case.

| Id | Model and held behaviour | O (own) | S own | S held | O − S_held | Outcome |
|---|---|---|---|---|---|---|
| H-1 | V-2, anchor rise s = 2s\*; one-way prop held **Inactive** | prop inactive, u_tip = +s\*, R = 0, M_A = PL/2 | active, u = 0, R = 5P/16, M_A = 3PL/16 | u_tip = −s\* (penetration s\*), R = 0, M_A = PL/2 | Δu = 2s\*, ΔR = 0, ΔM_A = 0 (1b: s\*, −5P/16, 5PL/16) | R: released (inactive → active) |
| H-1b | V-2, s = s\*/2; prop held **Active** | active, R = 5P/32 | active, R = 5P/16 | = S own | = 1b's O − S, bit for bit | E or N (as S own's standing) |
| H-2 | Bar; tip one-way on UX, positive sense; S: F = +512 N; O: F, ΔT = −16 K (δ_T = −2⁻¹⁰); held **Active** | active, u = 0, N = 1024, R = +512 | inactive, u = 2⁻¹¹, N = 512, R = 0 | u = 0, N = 0, **R = −512 N (tensile)** | Δu = 0, ΔN = +1024, ΔR = +1024 | R: released (active → inactive) |
| H-3 | Bar; tip friction on UX, explicit N = 4096 N (μN = 1024); S: F = 2048; O: F, ΔT = −24 K (δ_T = −3·2⁻¹¹); held **Sticking** | sticking, u = 0, N = 1536, R_t = −512 | sliding +, u = 2⁻¹⁰, N = 1024, f = −1024 | u = 0, N = 0, **R_t = −2048 (\|R_t\| > μN)** | Δu = 0, ΔN = 1536, ΔR_t = +1536 | R: released (sticking → sliding) |
| H-4 | V-3 (as H-3 but O: ΔT = −8 K, δ_T = −2⁻¹¹); held **Sliding +**, explicit | sliding +, u = 2⁻¹¹, N = 1024, f = −1024 | sliding +, u = 2⁻¹⁰, N = 1024, f = −1024 | = S own, bit for bit | Δu = −2⁻¹¹, ΔN = 0 | E (kept) |
| H-5 | As H-3 but O: ΔT = −64 K (δ_T = −2⁻⁸); held **Sliding −**, explicit | sliding −, u = −2⁻¹⁰, N = 3072, f = +1024 | sliding +, u = 2⁻¹⁰ | f = +1024, **u = +3·2⁻¹⁰ (slip opposes held sense)** | Δu = −2⁻⁸, ΔN = 0 | R: released (sliding → sticking) |
| H-6 | Bar; tip UZ base restraint as the normal source; friction on UX, derived normal; S: V = 4096 N down at the tip, F = 2048; O: V plus V′ = 4096 N, F, ΔT = −80 K (δ_T = −5·2⁻¹⁰); held **Sliding −** | R_n = 8192, sliding −, u = −2⁻¹⁰, f = +2048 | R_n = 4096, sliding +, u = 2⁻¹⁰ | R_n = 4096 (the **target's**), f = +1024, u = 3·2⁻¹⁰ (frozen-source-normal mutant: f = +2048, u = 2⁻⁸) | Δu = −2⁻⁸ | R: released |
| H-6b | As H-6 but S: F = 3072; O: V + V′, F = 3072, ΔT = −8 K (δ_T = −2⁻¹¹); held **Sliding +**, derived | R_n = 8192, sliding +, u = 2⁻¹¹, f = −2048 | sliding +, u = 2⁻⁹, f = −1024 | = S own (target's normal: f = −1024; mutant f = −2048, u = 2⁻¹⁰) | Δu = −3·2⁻¹¹ | E or N (k = 1 coupled; Stage A) |
| H-7 | V-1 closing (c = 2⁻⁹, S: F = 1536; O: F, ΔT = 16 K); gap held **Active** | active, u = c, N = 1024, stop −512 | open, u = 3·2⁻¹¹ | u = c, N = 2048, **stop +512 (pulls)** | Δu = 0, ΔN = −1024, Δstop = −1024 | G and released, if the source is strict-gap `Qualified`; otherwise withheld `HELD_STATUS_SOURCE_UNQUALIFIED`. Phase 2 tests both arms and records which model reaches each; if no product model reaches the `Qualified` arm, the held-gap closed form is pinned by an NI unit test of the wrapper |

The derivations for H-2 to H-6b use N = k(u − δ_T) and the tip balance F + R − N = 0 (friction
f = R_t on the pipe). Each "own" state is the unique solution of its static, history-free problem.
The generator checks every alternative branch for contradiction (for example H-3's operating
sliding ±, and H-5's sticking and sliding +).

**Linear held twins.** The same model with each held support replaced by its held linear boundary,
solved on the ordinary linear route:
- H-1: the prop removed;
- H-2 and H-3: a rigid UX restraint at the tip;
- H-4: the shoe removed and a constant −1024 N at the tip;
- H-5 and H-6: +1024 N;
- H-6b: −1024 N, with the UZ restraint kept.

The twin's rows equal S_held's closed forms, by op1b's 1e-9 rule. Bit equality between the routes is
not claimed. The twin is an independent check of the held boundary.

**Bindings and refusals** (PP tests over a captured `HeldStatusSource`, each by its name):
- a missing, extra or repeated support;
- the node, DOF, behaviour, sense, gap bits or μ bits changed;
- the source case is another case;
- the mode differs;
- an `InspectableUnqualifiedGap` source;
- a linear model (`SOURCE_UNAVAILABLE`);
- a friction support held `Active`;
- an orienting iterate that is not the final one;
- a sliding ground that differs: R-b ground motion on the shoe in O only;
- a held system made singular by a held lift-off: `SOLVE_FAILED`, with the document not blocked.

**Order and placement.**
- G-1 and G-2 failures withhold before the held gates.
- The member on `mechanics` is `…_MISPLACED`.
- On EX the exact refusal comes first.
- `own` is byte-identical to absence.

**Repeatability and modes.** Each held document is solved twice in one process and once in a fresh
process: the bytes are identical. Every closed form is checked in dense and in sparse mode.

**H-10, the reproduction regression (NI).** Over every committed nonlinear model's converged cases,
in both modes:
- held at the case's own final states with its own **penultimate** iterate equals the loop's final
  iteration record, result, receipt, qualification and confirmation outcome, bit for bit;
- held with its own **final** iterate equals the same, which validates §7.2's orientation.

**Mutants** (each restored after):
- holding the source's displacement instead of the state (killed by H-3);
- freezing the source's normal (H-6, H-6b);
- the slip sense taken from the target's own motion (H-5: that mutant keeps the support and changes
  u);
- skipping the classifier, so every state is kept (H-2: the standing and `RELEASED` are wrong);
- held rows published as the case's (the case-row bytes check);
- "confirmed" wording on a held standing (the text check);
- the readers' state-binding check disabled (the forged-entry tamper);
- a §4 lookup of the subtrahend's own bounds for a held combination (once §4 is merged; §7.11
  O-C2).

**Readers.** A shared corpus `fixtures/results/preview_physics_held_status_corpus.json` is the
producer's output. Every document reads with and without its model, and the coherent forgery is
included. There are tampers, each refused by name in both languages, for: each binding field; each
classified state; RELEASED present without a release or absent with one; the standing kind; the
sign text; the held-row basis; a held row cited as a case row; the source mode; and a case recurring
with different contents.

### 7.10 (i) Pricing outlook (no registration in this work)

The change moves the retained route's statics and code. It needs a DEL-04-08 re-pricing and
registration of its own after phase 2.
- **PP (retained).** `lib.rs`:
  - `LoadCaseSolve` +8 B, taking the carrier from 1,888 B to 1,896 B, and +8 B more if §4's
    `certificate_operand` also lands;
  - the held dispatch in `solve_prepared_load_case`;
  - the capture;
  - the held solves after the case loop.

  Also new `held_status.rs`, plus `preview_physics.rs`, `difference_basis.rs` (or `held_status.rs`
  for the sign) and `document_members.rs` (the interim push removed).
- **NI (retained).** `lib.rs`: the wrapper and `HeldStatusSolve`. `confirmation.rs` and
  `structural_adapter.rs` are unchanged.
- **Tables.** The three preview tables' combination policy v4 (STATICS rise, as §8.51's v3 did),
  with the recorded-run registry gaining the moved tables (as §6.7).
- **Rules.** The pricing rules are `tools/retained_pricing/rules/o10_rules.py`.
  - The S1 `interim_reasons` entries for the `minuend_final` push (`:4113`, `:4226–4228`) drop.
  - The A4 family counts at `:4776–4805` and `:5211` stay, since the retained routes keep declining
    the member.
  - New rules rest on D1.4's refusal of every difference basis: the held arms are zero on D1, and
    the by-name call graph gains false edges into the held functions, which need the same removal
    as §8.51's PF1B rules.
- **Not changed.** FK, SD, `structural_adapter.rs` and confirmation code. P's D3 pins in those
  files are unaffected, unless a full-file pin covers NI `lib.rs` (§7.11, O-P2).

### 7.11 Seam map for phase 2

Paths are as in the table at the top of this document. Line numbers are on `b8f3837e8`.

| Area | File and lines | Phase-2 change |
|---|---|---|
| NI step (read only) | `lib.rs:1559–1780` `solve_iteration_with_sliding_friction_evidence` | unchanged; called once by the wrapper |
| NI wrapper (new) | `lib.rs`, after `:1780` and before `resolve_sliding_friction_states` (`:1782`) | new `pub fn solve_held_status_iteration` and `pub struct HeldStatusSolve`. It calls `dense_loop_memory_guard` (`:531`), `validate_input` (`:1338`), `normalized_initial_states` (`:2005`), `LoopSystem::assemble` (`:2334–2425`), `loop_representation` (`:2262`), `gap_views` and `gap_source` (`:2443–2476`), `active_boundary` (`:2108`), `friction_normal_map` and `derived_friction_normal_map` (`:2048`, `:2068`), the step, `structural_adapter::scrutinize_gaps` (`:3106`) and `exact_gap_iteration` (`:3430`), `build_trial_states` (`:3078`), `resolve_sliding_friction_states`, `residual_observation` (`:2971`), `add_applied_forces` (`:1948`), `policy_diagnostics` (`:3228`), `assembled_loop_assumptions` and `assembled_loop_limitations` (`:1285`, `:1297`), and then `confirmation::confirm_converged_state` |
| NI loop (reference only) | `lib.rs:766–1024` (step call `:815`, post-step `:878–1018`, completion `:1009`) | unchanged; the composition is H-10's subject (§7.3 S1/S2) |
| NI receipt, qualification, confirmation | `lib.rs:355–381`, `:1029–1283`; `confirmation.rs:79–404`, `:973–1116` | unchanged |
| NI tests (new) | `lib.rs` test module or a new `held_tests.rs` | H-10, H-7's NI arm, wrapper validation |
| PP `LoadCaseSolve` | `lib.rs:2650–2673`; constructor literals at `:5310`, `:5347`, `:5455`, `:5496`, `:5763`, `:5796`, `:5894`, `:7381`, `:15728` | + `held_status_source` beside `nonlinear_state` (that line is unchanged) |
| PP case solve | `lib.rs:5267–5283` (signature), `:5849–5868` (the NI call, a held dispatch beside the unchanged call), `:5869–5895` (the held path does not require `converged`), `:5972–5976` (a `NonlinearStanding::HeldEstablished` arm), `:6000–6005` (the held evidence line in place of the confirmed one), after `:5985` (the held-source capture) | the #97 capture `:5978–5985`, the third tuple member and `confirm && passed && !gap` (`:7546`) are unchanged |
| PP nonlinear append | `lib.rs:7438–7818` `append_nonlinear_support_loop_results` | unchanged, except an optional behaviour-preserving extraction of its input construction (`:7488–7513`) into a helper shared with the held sibling. Otherwise the held sibling restates about 30 lines (a review choice) |
| PP held sibling (new) | `held_status.rs` | `append_held_status_results` (rows and diagnostics of the held solve, in a local diagnostics vector), `capture_held_status_source`, `solve_case_at_held_status`, the binding checks, the held evidence record, `SIGN` |
| PP document solve | `lib.rs:3257–3412` (case loop), insert after `:3412` and before `:3413` (`let built = …`) | the held solves for admitted `minuend_final` combinations, which need `basis_solve_states` in scope |
| PP combinations | `preview_physics.rs:848–905` `gate_reason` (held gates after G-2), `:973` `append_combination_results` (+ held map: the subtrahend lookup `:1041–1046` reads the held rows; the sign `:1036–1040` takes the held SIGN), `:1359` `append_operand_standing` (held subtrahend record) | — |
| PP evidence | `lib.rs:3524` (beside `difference_basis::derivation`), `:3661` (insert `held_status_derivation` beside `DERIVATION_MEMBER`) | — |
| PP interim | `document_members.rs:826–835` (drop the push); constant `:69–70` kept | — |
| PP schema enum | `schemas/results.v0.3.schema.yaml:725`, `:4852` (basis enum + `held_status_operand`); the `PreviewPhysicsContractEvidence` member | stress_neutral_export.v0.3 and analysis_run.v0.3 follow, and `schema_manifest.json` is regenerated |
| RE | `preview_difference_basis.rs:360–378` (`kept_combinations`), `preview_combination_gates.rs:465–470`, `preview_physics_evidence.rs:60–70` (gate codes), `preview_combination_operands.rs` (the held operand member), new `preview_held_status.rs` and `_tests.rs` | — |
| Python | `tests/verifiers/preview_difference_basis.py:287–297`, `preview_combination_gates.py:269`, `:361`, `preview_physics_evidence.py:80–86`, `preview_combination_operands.py`, new `preview_held_status.py`; tests `test_preview_held_status.py`, `test_model_document_0_9_0_members.py:68` (expectation) | — |
| Applier | `operation_applier/src/lib.rs:6276–6293` (refusal arm and rustdoc), `:15968–15973` (test), `tests/condition_set_authoring.rs:554–569`, `tests/model_document_0_9_0.rs:43` | `minuend_final` authored on PF; EX keeps `…_EXACT_UNSUPPORTED` |
| TS | `apps/desktop/src/features/viewport/resultColourOverlay.ts:145–149` (`declaredDifferenceNote` gains the held sign: "operating minus sustained difference at the operating support status"), `derivedRowNote.test.ts`, the wasm reader through RE | `types.ts:541` and `modelView.ts:296` are unchanged |
| Semantic tables | `fixtures/results/semantic_contract_v0_3_preview_physics_{2,timoshenko_1,retained_1}.json` through `semantic_contract_tables.py`; recorded-run registry (`recorded_tables::VARIANTS`, `compatibility.py`, `recordedPreRtA4.ts`) | §7.8 item 3 |
| Pricing | `tools/retained_pricing/rules/o10_rules.py`, `base/s1_owners_model.py:47` | §7.10; registration later |

**Overlaps with P's remaining D3 edits.**
- **O-P1. PP `lib.rs:3209–3214`** (the §8.30 correction-2 comment about the dense loop). Held work
  inserts at `:3412` and does not touch this comment. There is no textual overlap. The comment's
  claim ("the nonlinear active-set loop is dense in both modes") should be revised by P in any case.
  The held solve inherits the same guard, already passed for the model.
- **O-P2. Pricing pins in NI `structural_adapter.rs` and FK `structural.rs`/`sparse.rs`.** Held work
  changes none of those files. If P's D3 restates a full-file pin over NI `lib.rs`, the wrapper
  moves it: phase 2 must merge after P's D3 and restate it.

**Overlaps with the §4 certificate** (`claude/rt-a3-cert`, `a3fdee573`, unmerged).
- **O-C1 (textual).** The certificate adds `certificate_operand` to `LoadCaseSolve` and every
  constructor literal (the same sites as `held_status_source`), and a `certificate_operands` map at
  `lib.rs:~3447–3475` beside `rows_by_base_id`. Its `combination_certificate::publish` call is near
  the envelope end (`:~3693`). The held solves insert at `:3412`, and the held evidence at
  `:3524`/`:3661`. The literals conflict line for line, which is mechanical. The branch that merges
  second adds its member beside the other's.
- **O-C2 (semantic, must be resolved before both land).** `combination_certificate::publish` keys
  operands by load-case id. For a `minuend_final` combination it would bind the subtrahend case's
  own group bounds to an O − S_held row, which bounds a different system. Before both land, either
  the certificate refuses `minuend_final` combinations as `not_assessed` (the minimal change, within
  its current premises), or its owner decides to admit E held operands. The latter is a premise
  change. The held design does not make either change.

### 7.12 Open questions for the load-cases manager and Agent 0

1. **S1 or S2** (§7.3): the wrapper's one composition of the loop's post-step calls, guarded by H-10,
   or a shared extraction that edits the loop body.
2. **Outcome E's wording.** E's sentence states that each held support's condition holds at the
   target's loads, because the confirmation genuinely establishes it. Is that within requirement 3,
   or must E state only "established within its stated bound against the held system"?
3. **Gaps.** Admit gap-bearing models with outcome G (recommended: the source must be `Qualified`,
   so R1 is not bypassed), or refuse them by name until R-3?
4. **Rows.** Publish held-operand rows under the combination with a new basis token (recommended),
   or the evidence-only alternative (§7.6)?
5. **`LIMITATIONS[6]`**: unmoved (recommended) or appended (§7.8 item 4)?
6. **Code names**: `HELD_STATUS_*` (proposed) or a `LOAD_COMBINATION_HELD_STATUS_*` prefix.
7. **§4 interplay (O-C2)**: for the certificate owner. Held work assumes "not assessed" until the
   owner decides.
8. **§0 R-1 / §5 B / §1.4 status wording** (§7.8 item 10): Agent 0's.
