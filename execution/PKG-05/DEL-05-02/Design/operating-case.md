# Operating case with nonlinear supports, and the kernel side of skewed restraints

> **Status: adopted plan; truthfulness fixes (#1695) and stage 2 (#1671) built; slice 1a in flight (`503d9495f8`); R-6 unbuilt; superseded by any later Design text.**
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

**The source item.** On M, DEL-17-04 `Design/caepipe-mbf-import.md` §20.3 item 3
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
