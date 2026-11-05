# Operating case with nonlinear supports, and the kernel side of skewed restraints

> **Status: adopted plan; truthfulness fixes (#1695), stage 2 (#1671) and slice 1a (#1752) built; slice 1b integrated (#107, §6); R-6 unbuilt and scheduled for v1.0 after 1b (2026-10-31, below); `minuend_final` designed (§7) and built as a candidate on `claude/rt-a5-minuend-final` (§7.3 status); superseded by any later Design text.**
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
| R-1 | The sustained case keeps its own support status. Frozen operating status (`subtrahend_support_state`) remains a later item, flagged to the owner (§5). | Own status changes nothing that exists. Frozen status is a different comparison, and choosing it is a code-interpretation matter. Status (2026-11-04): the owner-accepted release target (2026-10-28) carries frozen operating status as RT-A5 Should. It is designed in §7 as the declared held-constraint comparison `subtrahend_support_state: minuend_final`; own status stays the default and is never inferred. |
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
| §5 B, the meaning of "expansion" | Option (3), computed as (1), is adopted. The product reports an "operating minus sustained difference" under the member's own name, with each case at its own state. The "expansion" label and its code-range meaning stay with the user's rule pack. The held-constraint option (2) is the declared `minuend_final` comparison of §7, added without renaming; the engineering meaning of either difference stays with the user's rule pack. | It makes the least product claim. The 0.9.0 member is already named `operating_minus_sustained`, and the engineering meaning is a design-basis choice the user's rule pack makes. |

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
| `combinations[].subtrahend_support_state` (RT-A5 Should; §7) | enum `["own", "minuend_final"]` | absent = `own` | Only with `difference_basis`. Results are published under the combination, never as the sustained case's rows. |

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
`800fae7e5`). Integrated by #107 (`b32a17761`) after review r1 and its focused confirmation. Risk class: numerical
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
  general. It can establish a held solve only when the target case's own classifier keeps every
  held state and every contact branch is admissible, and otherwise returns `NotEstablished`
  without a bound (§7.3).
- **The input seam.** The dynamics lane's #97 `NonlinearStateEvidence` binds the case, mode,
  qualification, standing and final states. It lacks the orienting iterate, the support assignment
  (node, DOF, behaviour), the normal maps, an authentication of the solution vectors and an
  invocation binding. §7.4 adds them as a separate record and leaves the #97 lines untouched.

### 6.10 Pricing and registration

Registered at DEL-04-08 `retained-pricing-rules.md` §8.51, on main `573051799` (RT-U4's registered §8.50 candidate
`41142f4a3`), priced source `a82460ad7`. AC-004 is recorded (seven witnesses and the isolated challenge, 7/7 on the
registered binary at `d44e5c538`; `61c6214a2`).
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

**Status: design, phase 1, revision 1; not built.**
- **Revision 1.** It applies the independent review of `9cfbc21cd` ("revise before implementation";
  no P0 findings; every H-1, H-1b and H-2 to H-7 closed form independently confirmed). It also folds
  in Agent 0's decisions and RT-A3's S2 boundary, both recorded where they apply below.
- **Base.** `claude/rt-a5-minuend-final` at `b8f3837e8`: RT-A3 P's reviewed candidate `78580da7e`
  merged with main `b32a17761`, which carries 1b (#107). Line numbers refer to this base.
- **Risk class.** Numerical meaning, persistence and contract.
- **Reserved.** Nothing here records acceptance, issuance or release. These stay reserved:
  eligibility policy (b), unchanged; the §4 certificate's premises; the #97 seam; and P's frozen
  candidate.

**Terms.**
- The **source** case is the minuend, or for hanger sizing the design operating case.
- The **target** case is the subtrahend, or for hanger sizing the weight case.
- The **held status** is the source's `final_states`.
- The **held system** is the target's own loads, materials, conditions and imposed motion, with each
  nonlinear support's boundary formed from its held state instead of being iterated.
- A **held solve** is one solve of the held system.

### 7.1 The function and its consumers

This is the one product function of §0 ("`minuend_final` as a callable mechanism"), in two layers.
Neither copies the loop.
- **NI** `solve_held_status_iteration` is a new `pub` function placed beside the step
  (`lib.rs:1559–1780`). It:
  1. assembles the target's system through `LoopSystem::assemble` (`:2334`);
  2. forms the held boundary (`active_boundary`, `:2108`);
  3. calls `solve_iteration_with_sliding_friction_evidence` once;
  4. passes the solved iterate to the loop's own tail, `complete_iteration`, with `IterationExit::Now`
     (§7.3, S2, decided);
  5. then calls `confirm_converged_state` separately and unchanged.

  There is no iteration, no all-active recovery and no second step. `LoopSystem`, `BoundaryState`,
  the step, `confirmation.rs` and `structural_adapter.rs` are unchanged. The loop body changes only
  by S2's pure extraction of its tail.
- **PP** `held_status::solve_case_at_held_status` is the one product entry. It enforces the source
  binding (§7.4) and solves the target through `solve_prepared_load_case`'s own assembly and
  publication, with the NI call replaced by the held wrapper (§7.11). It returns a typed held solve.
- **Consumers.**
  - The declared subtraction with `subtrahend_support_state: minuend_final` (§7.5–§7.6).
  - Later, DEL-04-11's `weight_support_state: design_operating_final`, through §7.7's contract.
    Hanger integration is excluded from this work (Agent 0).

### 7.2 (a) Frozen-constraint semantics, per behaviour

The held status fixes each support's **state**, never its displacement or force. Prescribed values
and loads are always the target's own.
- This is the static, history-free model of R-2. Sticking is measured from the target's ground, as
  in every case solve.
- A history model that holds the source's displaced position is DEL-04-04:2's, and it is not added.
  H-17 and H-17c kill that mutant (§7.9).

| Support, held state | DOF | Prescribed value or applied force | Classifier test at the held solve (exact binary64, the loop's own rules) |
|---|---|---|---|
| One-way or lift-off, Active | prescribed | u_d = g_s^T (0 unless the target moves it) | kept iff the reaction has the support's sense, strictly (`state_from_sense`); otherwise a **tensile held reaction** |
| One-way or lift-off, Inactive (lifted off) | free | none | kept iff no penetration, i.e. δ = u_d − g_s^T is not on the contact side (`state_from_penetration`; δ = 0 is kept, which is weak contact, H-18) |
| Gap, Active (closed) | prescribed | u_d = σ(c + σ·g_s^T), required exact by TwoSum (`exact_gap_closing_position`); an inexact position is refused | kept iff the stop bears (closing in +: r ≤ 0; closing in −: r ≥ 0); under strict-gap qualification the decision uses the exact reaction sign (§7.6) |
| Gap, Inactive (open) | free | none | kept iff the gap does not close; under strict-gap qualification, by the exact gap sign |
| Friction, any state, normal N^T ≤ 0 | — | — | **classified Inactive first**, whatever μ and the held state: a zero or negative normal is Inactive before any stick or slip rule. The trial normal is the explicit input N, or \|R_n\| for a derived normal (`friction_normal_for_support`, `lib.rs:3115–3145`) |
| Friction, Inactive | free | none | kept iff N^T ≤ 0. Otherwise the trial's tangential reaction is 0, below μ·N^T, so it is classified Sticking (H-16b) |
| Friction, Sticking | prescribed | u_t = g_t^T | kept iff \|r_t\| ≤ μ·N^T |
| Friction, Sliding, explicit normal, nonzero limit fl(μ·N^T) > 0 | free | φ = −dir^S·μ·\|N\|, with N the model's explicit input (the same in both cases) | kept iff φ·δ_t < 0 and r_t·δ_t < 0 (`resolve_sliding_friction_states`); otherwise classified Sticking |
| Friction, Sliding, derived normal, μ > 0 | free | φ = −dir^S·μ·β^S·R_n(u^T), solved simultaneously by the step's coupling (`:1607–1700`), with R_n the target's own signed reaction at the normal-source DOF | **branch:** admissible iff β^S·R_n^T ≥ 0 (or R_n^T = 0 when β^S = 0) (`derived_normal_branch_admissible`, `:1976–1988`). An inadmissible branch keeps the classification **Sliding** and skips the tangential test (`:1835–1844`). Otherwise, as for the explicit normal, including the zero-limit row below |
| Friction, Sliding, zero limit fl(μ·N^T) = 0 (μ = 0, or a positive μ whose binary64 product with N^T underflows to zero) | free | none (the step forms no force when −dir·μ·\|N\| is zero) | after the zero-normal rule and, for a derived normal, the branch rule: kept iff δ_t ≠ 0 and no force is applied (the classifier's `limit == 0.0` branch, `lib.rs:1846–1855`). The branch is selected by the product, never by μ alone. The held sense carries nothing (no force), so a reversed slip is kept (H-15a, and H-15c for the underflow) |
| Friction, Active (not a friction state) | — | refused at binding (§7.4) | — |

**Not held: springs, constant-effort supports and linear supports.** These have no state.
- Ground springs keep their stiffness, and their bases move only with the target's motion (R-c).
- A constant-effort force is part of the target's ledger (DEC-049).
- Rigid restraints are prescribed at the target's g_b (R-a, R-d).

**The friction direction (the tangent and slip-direction caveat).** dir^S is the slip sense of the
source's final iterate (`sliding_direction`), and β^S is the sign of the source's final normal.
- **Why the final iterate.** At a qualified Sliding state the final iterate's slip has the sign of
  the force direction that iterate used, since tangential admissibility requires φ·δ_t < 0 with
  μN > 0. Its normal has the sign of the assumed branch, since admissibility requires β·R_n ≥ 0 and
  a zero normal is Inactive. So the source's final iterate orients the held step as the loop's own
  last iterate did. H-10 checks this.
- **Which ground.** The step measures slip on that iterate against the target's g_t (`:1883–1946`).
  A held sliding support is therefore admitted only when g_t^S and g_t^T are bit-equal (§7.5); a
  differing ground is refused, never corrected.
- **The tangent.** Friction acts on its one translational DOF, so the tangent is that axis. There is
  no in-plane slip direction to hold.
- **Motion opposing the held sense.** If the target's motion opposes the held sense, the classifier
  releases the support (H-5, H-14r).

**The derived normal: which case's normal applies, and why.** The **target's**, solved in the same
step.
- Coulomb's law at a held Sliding status bounds the tangential force by μ times the normal the held
  system carries. Holding the status holds "sliding, in sense dir^S, on branch β^S", not a force
  magnitude.
- Freezing the source's normal would add an operating-derived load to the target. That would break
  the target's actual loads (requirement 2) and G-1's content premise.
- A target normal of the other sign is an **inadmissible branch**, which §7.3 counts as released
  (H-13).
- Sticking and Inactive derived-normal supports use \|R_n\|, exactly as the loop does, so a sign
  reversal there is not a release. Their signed R_n^T and β^S are still published (§7.6).
- H-6b and H-14r kill the frozen-source-normal mutant. H-14 and H-14r kill an uncoupled-normal
  mutant.

### 7.3 (b) The held step, the confirmation, and what each outcome publishes

**S2, the shared tail (decided).** Agent 0 approved S2 for the extraction only, and RT-A3, the loop
owner, fixed its boundary. The loop's post-step composition (NI `lib.rs:878–1018` on this base) is
replaced by one crate-private function, which the loop and the held wrapper both call:

```rust
/// Everything the loop does with one solved iterate: gap scrutiny, trial states,
/// friction resolution, the active-set step, the converged predicate, residual
/// observation, the iteration record, and (on exit) diagnostics, result and
/// ProductCompletion receipt. Pure extraction of the loop tail (RT-A5 S2).
fn complete_iteration(
    ctx: &IterationContext<'_>,          // input, springs, &LoopSystem, gap views, &BoundaryState,
                                         // friction_normals, derived_friction_normals, linear_solve_mode
    iteration_index: usize,
    sliding_solve: SlidingIterationSolve, // moved in, as today
    recovered_path: bool,
    sliding_force_deferred: bool,
    state: &mut LoopState,               // iterations, current_states, final_diagnostics,
                                         // exact_gap_remaining_work
    exit: IterationExit,                 // AtConvergenceOrCap (loop) | Now (held wrapper)
) -> Result<Option<NonlinearFrameSolveResult>, NonlinearIntegrationError>
```

- **The loop** calls it with `AtConvergenceOrCap`, then `if let Some(result) = …? { return Ok(result) }`.
  The finish condition `converged || blocked || iteration_index == max_iterations` moves inside
  unchanged, and `Now` only adds `|| true`. These stay in the loop: the step call, the all-active
  seed recovery and `sliding_force_deferred`'s computation (everything before `:878`).
- **The held wrapper** does the following.
  1. It forms `held_input`: the caller's input with `initial_states` replaced by the held records,
     which are the states the step ran from.
  2. It builds `LoopState` with:
     - no seed iterations, so residual deltas are `None` and the source's iterate is never a delta
       reference;
     - `current_states` = the held records;
     - `final_diagnostics` = `policy_diagnostics(&input.convergence)`;
     - the same gap allotment the loop starts with (`GAP_RUN_WORK_LIMIT`).
  3. It calls the step once, with the source's final iterate as `previous`.
  4. It calls `complete_iteration(…, 1, solve, false, false, &mut state, Now)`, which always returns
     `Some(result)`.
  5. It calls `confirm_converged_state` separately and unchanged. The call runs only when no
     load-row finding exists and the iteration's structural report is Passed, PP's own rule;
     otherwise the outcome is `NotEstablished { "confirmation attempt: not run" }`.
- **`IterationContext` and `LoopState`** are crate-private bundles of what the loop already holds.
  They add no meaning, and nothing is `pub`.
- **Preserved exactly:**
  - the evaluation order, including `current_states = active_set.states.clone()` before
    `residual_observation`, which reads `iterations.last()` before the push;
  - every `?` early return;
  - the moves of `sliding_solve`'s fields;
  - the recovery-path predicate;
  - the non-converged exit diagnostic, so a released held result carries it in its own
    `diagnostics`. PP never republishes held solver diagnostics as case diagnostics (§7.5);
  - `ProductCompletion`, built from `system.stiffness()` and the last iterate.

  Nothing new is introduced: no duplicated composition, no new active-set or confirmation logic,
  and no changed rounding, mutation or receipt provenance.
- **Why this boundary (RT-A3).**
  - R-3's G2 will work inside `scrutinize_gaps` and the `StrictGapEvidence` match, both in this
    function, so S2 lands before R-3 and G2 edits one copy.
  - Dynamics has no hunk in or around `:878–1018`. I3a does not touch NI. Its future modal guard
    counters sit in `LoopSystem::solve_iterate` and `ConfirmationContext`, and a per-iterate count
    would hook `complete_iteration`'s entry.
  - The certificate branch and R-3 add nothing in `:878–1018`, and P's D3 does not change those
    lines.
- **S1, rejected.** S1 would compose the same post-step calls a second time in the wrapper. That
  duplicates the active-set bookkeeping, which only a regression would catch drifting, and G2
  would then need parallel edits in two copies.

**S2's conditions** (requirements on the extraction commit):
1. Dense and sparse byte identity of every committed nonlinear output.
2. H-10 as a projection test (§7.9), applied both to the loop before and after the extraction and to
   the held wrapper.
3. A K-1 **trace re-read** against the moved sites, recorded in the re-check paragraph of DEL-04-07
   `k1-twin-bound.md` and in DEL-04-07's K-1 need. The loop rules that the K-1 script encodes, read
   at their new sites, are: the authored seeds, no friction force on the first iterate, the deferred
   first iterate, and the slip direction and normal branch taken from the previous iterate.
   Re-running the script is not enough.
4. The S11-K source-site assertions for the moved sites in NI `s11k_tests.rs`, updated with their
   owners: DEL-04-07/RT-A3 for the loop, DEL-04-01 for K-D5's S11-K ruling.
5. A measured cost with the pins restated in our own D3 slot. Coefficient identity is a target to
   prove, not an assumption.
6. Frame, stack and lifetime impact assessed independently. The struct borrows and the moved
   `SlidingIterationSolve` must not lengthen lifetimes on the retained path. The loop is D1-refused,
   and the assessment shows it.
7. No witness exemption for being a pure extraction.

**S2's ownership and order.**
- RT-A3 owns the boundary review, and reviews the extraction commit before the work widens.
- P's frozen candidate is never modified.
- The extraction is its own first commit on this stack, after P's final reviewed basis, in its own D3
  slot after P and the current I3a queue.
- The final merge comes after P.

**Status: S2 built (phase 2 step 1, `d02ec8789`; for RT-A3's boundary review).** Nothing for
`minuend_final` itself is built. The commit holds only the extraction and the S11-F row that moves
with it (PP `tests/s11f_site_test.rs` rule 6: `complete_iteration` now reads the input force for the
receipt).
- **Conditions 1 and 2, byte identity and H-10 in its extraction form.** A scratch, uncommitted hook
  recorded each loop call's full `Debug` (mode, springs, input, result, receipt included) before and
  after the extraction, over every loop call of the NI suite (1,264 calls) and the PP suite (1,139
  calls), in both modes. Excluding nothing, the before and after multisets are identical, and so are
  the stored record texts.
- **Suites.** The suites pass on `d02ec8789`: NI 181, FK `s11_site_table` 3, and PP 1,725 passed, 0
  failed, 133 ignored, as before the extraction.
- **Condition 3, the K-1 trace re-read,** is recorded in DEL-04-07 `k1-twin-bound.md` and its K-1
  need.
- **Condition 4.** No S11-K row moves: the moved block has no compound assignment or fold, and FK's
  table passes unchanged.
- **Conditions 5 and 6, pricing and frame/stack/lifetime.** These are in the step's return.
  Retained pricing is coefficient-identical to the parent `30b6b34e7` (1,602 equal, 0 higher, 0
  lower), measured read-only because P's D3 pins are still pending. Nothing is registered.

**Status: candidate built (phase 2 step 2, branch `claude/rt-a5-minuend-final`; not integrated, not
independently reviewed).** The NI wrapper, the PP held path, the applier, the RE and Python readers,
the TS display, policy v4 and the schemas are built as this section and §7.4 to §7.8 describe, with
these approvals and deviations:
- **R2, option A (RT-A3, 2026-11-04).** NI `lib.rs` names `confirm_converged_state` exactly once
  outside the loop's own attempt: inside `fn solve_held_status_iteration(` only, strictly after its
  `complete_iteration(` (S11-K's R2 rule, four mutants; kd5 `kd5-nonlinear-active-set.md` notes the
  amendment). **The gap rule** sits beside the attempt: a held input with a gap support is never
  confirmed ("confirmation attempt: not run"), so G is never promoted.
- **Residual deltas (a deviation from "None" above; RT-A3's requirement).** The wrapper seeds the
  loop state's iterations with the orienting iterate (the source's final one), so the held iterate's
  displacement and reaction deltas are measured from it, and `result.iterations` is [the source's
  final iterate, the held iterate]. H-10's projection excludes the deltas for that reason.
- **No `metadata.held_status_ref` (a deviation from §7.6).** The closed `ResultMetadata` classes of
  PP, RE and both result schemas are not widened. A held-operand row's identity is its id
  (`result:combination:<combination>:held:<base id>`), its `basis_ref` (the combination), its typed
  basis and its clause; the record binds the combination to its source and target cases.
- **The held-operand rows published** are the held rows the difference rows cite, closed under their
  own `source_result_refs` (so every reference resolves), in the held solve's order, before the
  difference rows.
- **The record's `held_solve` member** (`converged`, and the loop's exit diagnostic when it did not
  converge) is added: a release publishes as not converged with its exit diagnostic and is never
  promoted (the manager's condition at `solve.converged || held`).
- **The held operand's standing.** Its `combination_operands` operand states `support_state:
  "held_at_minuend_final"` and `held_standing`; its `numerical_quality` is null (the held system's
  is not an envelope case's), and it is excluded from `COMBINATION_OPERAND_CHECKS_NOT_PASSED` as
  from `…_OPERAND_STATES_DIFFER`.
- **Readers' first refusal.** The operand reader binds `held_standing` to the record and the held rows
  to the difference rows, and runs before the gate reader, so a forged standing or a dropped held row
  is refused as `…_COMBINATION_OPERANDS_SHAPE` or `…_ROWS` rather than by a held code.
- **The hanger callable (§7.7)** is the path the subtraction uses (`held_status::held_solve` through
  `CaseRole::Held`); `solve_case_at_held_status` itself is not added, since hanger integration is
  excluded.

**Why the held result is honest.**
- Every field of the result and the receipt is what the step and the shared tail computed for
  `held_input`.
- `converged = true` means exactly this: the target's own classifier, applied to this held solve,
  changes no held state, and every contact branch is admissible.
- `product_selected_state_is_qualified` and `confirm_converged_state` then judge the result on their
  unchanged conditions. Nothing is bypassed, and no qualification is fabricated.

**Outcomes and their standing.**
- **Classification.** A held solve is **released** (outcome R) when the shared tail reports either of
  these:
  1. **a changed state:** `active_set.changed_supports` is non-empty, which includes every
     tangential inadmissibility, since the resolver then classifies Sticking;
  2. **an inadmissible derived-normal branch:** `contact_branches_admissible == false`. The support's
     actual classification stays Sliding and is published as Sliding.

  This is the case when the result is not `converged` and not blocked. A blocked result is
  `HELD_STATUS_SOLVE_FAILED`.
- **Wording.** No outcome publishes `NonlinearStateStanding::Confirmed`,
  `NONLINEAR_CONFIRMED_EVIDENCE_LINE` or the word "confirmed". None qualifies the natural state of
  either case.

| Outcome | When | Held rows | Standing (`standing.kind` and sentence) |
|---|---|---|---|
| **E** | non-gap; not released; every required predicate of the confirmation passes (`Established`) | Stage B's certified values, with K-D5's member bound against the **held system** | `established`: "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state, an admissible sustained state, a load history or uniqueness." (Agent 0's wording.) The per-support conditions are listed in the evidence (§7.6), not in prose |
| **N** | non-gap; not released; the confirmation returns `NotEstablished` (weakly active, H-18; Sensitive; k > 1 coupled friction; a load-row finding; …) | binary64 iterate values; member rows are binary64 recoveries | `not_established`: "Not established: <the confirmation's reason>. The held status is kept by this case's classifier; rows are inspection evidence." |
| **R** | released, as above | binary64; no bound (the confirmation returns "the converged state is not qualified") | `not_established`: "Not established: the held status is not consistent with this case's loads; rows are inspection evidence." Plus a warning `HELD_STATUS_RELEASED` naming each released support: its held state, its classified state, the reason (changed state, or derived-normal branch with β^S and the signed R_n^T) and the deciding quantity. It publishes with this standing and is never withheld (R-5) |
| **G** | any gap support (R1) | binary64; strict-gap evidence as the case path states it | `gap_inspection`: "Gap supports keep their recovery-basis branch (R1); not established." Plus `HELD_STATUS_RELEASED` if released. The source must still be `Qualified` (§7.4), so R1 is not bypassed |

**What E's evidence lists** (§7.6). This replaces revision 0's sentence "each held support's
condition holds … with a margin larger than its bound", which is false in general. Each support's
applied condition from kd5 §2.2 is listed by class:
- **Exact input decision.**
  - Explicit-normal friction with N ≤ 0 is Inactive (H-16a): no margin, an input decision.
  - The explicit-normal precondition N > 0 for Sticking and Sliding.
  - μ = 0 means no force.
- **Strict ball test.**
  - A one-way or lift-off contact sign or separation.
  - Stick capacity (explicit: N > 0 and μN > \|m_t\| + ρ_t; derived: 0 ∉ R_n ball and
    μ(\|m_n\| − ρ_n) > \|m_t\| + ρ_t).
  - A slip sign.
  - A derived normal's 0 ∉ R_n ball with sign β^S.
- **Precedence.** The zero-normal rule applies first, so μ = 0 sliding requires N > 0 before its slip
  test. Sticking requires N > 0.

For a strict ball test, the evidence carries the certified slot row's published value and radius
(`CertifiedRow.published`, `.radius`) where `ConfirmedState` returns one: a constrained support's
reaction, a sliding friction force, or a derived normal. A free one-way support's displacement ball
is not returned by the confirmation, so it is listed as "passed" without values. These are limits,
stated in the record.

**What the confirmation cannot supply.** It cannot supply a bound for a released held state.
- **Why.** The certificate and the contact predicate are conjoined in `certify_state`
  (`confirmation.rs:973–1116`), and qualification fails first in any case.
- **Frictionless supports.** With a stiffness positive definite on the base restraints alone, the
  static contact problem has one solution, so a held frictionless status that differs from the
  target's own is always released. This holds up to the classifier's exact decisions at weakly
  active supports.
- **What would be needed.** A bound for R needs a new confirmation attempt owned by DEL-04-01/kd5. It
  is not proposed here, and no second certification path is proposed either.

**Standing without §4.** Every O − S_held row is derived inspection evidence, as every subtraction
row is today.

**With §4.** The §4 certificate keeps a non-default `subtrahend_support_state` `not_assessed` until
its bounds bind the **exact held operand**, never the subtrahend case's own bounds by id (Agent 0).
RT-A3 is adding that exclusion on `claude/rt-a3-cert`, so O-C2 is resolved upstream (§7.11).

**Numerical eligibility.** Policy (b) is unchanged.

### 7.4 (c) The source-binding record and its enforcement

**What the #97 seam carries and lacks.** `NonlinearStateEvidence` (`lib.rs:25486–25493`) has
`case_id`, `solver_mode`, `converged`, `qualification`, `standing` and `final_states`. A held solve
also needs four things the seam lacks.
1. The **orienting iterate**.
2. **Authenticated** source solution vectors. Matching states does not authenticate the
   displacement and reaction vectors.
3. The **model context**. `NonlinearSupport` (`nonlinear_supports lib.rs:39–46`) carries neither the
   explicit normal nor the derived-normal source, which live in the input's two normal maps
   (`friction_normal_reactions`, `derived_friction_normal_reactions`; PP `lib.rs:7508–7511`), and no
   node identity.
4. An **invocation binding**. Crate privacy alone does not stop a source captured in another
   invocation, with identical ids and assignments, from being passed in.

**The additive record (PP, crate-private; the #97 lines untouched).**

```rust
/// RT-A5 minuend_final (operating-case §7.4): what a completed source case lends a held solve.
/// Formed only by `capture_held_status_source`, in the source case's own solve, from the same
/// completed construction as `nonlinear_state`. No other constructor, no Clone outside tests,
/// no deserialization.
struct HeldStatusSource {
    invocation: InvocationToken,          // minted once per document solve (or sizing request)
    evidence: NonlinearStateEvidence,     // the typed #97 record, as captured (immutable copy)
    input: NonlinearFrameSolveInput,      // the source's actual loop input (both normal maps incl.)
    springs: Vec<(usize, f64)>,           // the source's ordered springs
    solve: NonlinearFrameSolveResult,     // the source's actual result, with NI's private receipt
    node_ids: Vec<String>,                // built-model node index -> node id
}
```

- **The token.** `InvocationToken` is a crate-private value minted once per invocation context, from
  a process-wide counter. It is never published, so it moves no output byte, and repeat calls stay
  bit-identical. A source from any other invocation fails the comparison.
- **Where the record lives.** It is a new `LoadCaseSolve` member `held_status_source:
  Option<Box<HeldStatusSource>>` (8 B in the carrier), formed after the #97 capture
  (`lib.rs:5978–5985`, unchanged).
- **When it is formed.** Only when the case is the minuend of some `minuend_final` combination. Every
  other document, D1 included, carries `None`.
- **What it retains.** It retains the source's input and result: n-sized vectors and the iteration
  records, and only for such minuends. §7.10 prices this.

**What the callable enforces at use.** Every check is in `solve_case_at_held_status`, so the hanger
consumer gets the same guarantees. Each refusal is named and withholds the combination, or refuses
the sizing request. None blocks the document.

| # | Check | Refusal |
|---|---|---|
| 1 | The minuend's `LoadCaseSolve` carries both `nonlinear_state` and `held_status_source`. They are absent in a model without nonlinear supports and in every early-return construction | `HELD_STATUS_SOURCE_UNAVAILABLE` |
| 2 | `evidence.qualification == Qualified`. An `InspectableUnqualifiedGap` source has nothing to lend (R1). The source's `standing` is carried, not required | `HELD_STATUS_SOURCE_UNQUALIFIED` |
| 3 | `invocation` equals the calling context's token | `HELD_STATUS_BINDING` ("… source is from another invocation") |
| 4 | `evidence` equals, by value, the minuend `LoadCaseSolve`'s own `nonlinear_state`; `evidence.case_id` is the minuend id; `evidence.solver_mode` is the context's | `HELD_STATUS_BINDING` ("… source evidence / case <id> / mode <m> …") |
| 5 | `product_selected_state_is_qualified(&input, &springs, &solve) == Ok(true)`. NI's private receipt then authenticates that `solve`'s final states, displacements, reactions and iterations are the loop's own output for `input` (public edits invalidate it, `lib.rs:1056–1132`), and `solve.final_states == evidence.final_states` | `HELD_STATUS_BINDING` ("… source solution is not receipt-bound to its input") |
| 6 | **Model context.** `input` equals the target's assembled input in every member except the case's own (`force`, `base_prescribed`, `ground_motion`): `node_count`, `elements`, `connectors`, `curved_bend_elements`, `base_restrained_dofs`, `nonlinear_supports` (id, node, DOF, behaviour, gap, μ), `friction_normal_reactions`, `derived_friction_normal_reactions` and `convergence`. Also `springs` (ordered) and `node_ids` equal the target's. Each mismatch names its member and support | `HELD_STATUS_BINDING` ("… <member> differs …") |
| 7 | `final_states` has one record per support, each a state of its behaviour (no friction `Active`) | `HELD_STATUS_BINDING` ("… support <id> state <s> …") |
| 8 | The orienting iterate is `solve.iterations.last()`, taken from the authenticated result and never supplied separately. Each held Sliding support has a nonzero slip sense there | `HELD_STATUS_BINDING` ("… support <id> has no slip sense") |
| 9 | Each held Sliding support's ground is bit-equal in `input.ground_motion` and the target's | `HELD_STATUS_SLIDING_GROUND_DIFFERS` |

- **NI's own validation** (`normalized_initial_states`, `validate_input`, `active_boundary`) refuses
  malformed assignments independently. PP maps any NI error to `HELD_STATUS_SOLVE_FAILED` with its
  text.
- **The target side.** The held solve uses the target's own lowered loads, resolved `condition_use`,
  modulus-basis state, base motion, ground motion and spring-base motion. G-1 and G-2 apply
  unchanged.

### 7.5 (d) Admitted scope and named refusal codes

**Admitted initially.**
- The pressure-free route (preview-physics-2 and its timoshenko variant).
- A `result_state_subtraction` with `difference_basis: operating_minus_sustained` and
  `subtrahend_support_state: minuend_final`.
- Models with nonlinear supports.
- Both solver modes.
- Every behaviour of §7.2, gaps under R1 (outcome G).

**Gate order**, all warnings that withhold the combination:
1. `LOAD_COMBINATION_OPERAND_WITHHELD` (never met; operands are load cases);
2. G-1 `EXPANSION_BASIS_SUSTAINED_NOT_IN_OPERATING`;
3. G-2 `EXPANSION_BASIS_MODULUS_MIXED`;
4. then the held gates in this order:
   - `HELD_STATUS_SOURCE_UNAVAILABLE`;
   - `HELD_STATUS_SOURCE_UNQUALIFIED`;
   - `HELD_STATUS_BINDING`;
   - `HELD_STATUS_SLIDING_GROUND_DIFFERS`;
   - `HELD_STATUS_SOLVE_FAILED`, which covers an NI refusal, a blocked held result, a singular held
     system and an inexact held gap position.

The held solve runs only after G-1 and G-2 pass. Dependants are withheld whole by
`LOAD_COMBINATION_OPERAND_WITHHELD`.

**The code prefix.** `HELD_STATUS_*` is used, so the codes are not combination-specific; the hanger
consumer reuses them (Agent 0).

**Disclosed, not withheld.**
- `HELD_STATUS_RELEASED` (warning; outcome R or G).
- The standing record (E, N, R, G).
- The held solver's own diagnostics (policy diagnostics, the active-set diagnostics, the
  non-converged exit diagnostic) are not republished as case diagnostics.

**Unchanged.**
- EX blocks first with `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`, never lifted silently.
- `…_MISPLACED` and `…_UNKNOWN` are the validator's.
- Absence and `own` stay own, and nothing infers `minuend_final` (importer, SI writer, desktop
  template, defaults). The retained routes keep declining the member, and the SI writer keeps
  refusing it.

**Removed.** The interim `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED` withholding is
removed at the PF solve (`document_members.rs:826–835`) and at the applier (`lib.rs:6288–6289`). The
constant is kept for its history.

### 7.6 (e) Published evidence and readers

**Rows. `held_status_operand` is a closed typed basis (Agent 0).**
- **Held-operand rows.** The held solve's combined-kind rows publish **under the combination**:
  - basis_ref `{ref_type: "combination", ref_id: <combination>}`;
  - id `<combination-qualified>:held:<base id>`;
  - `metadata.basis` `held_status_operand`;
  - `metadata.held_status_ref`: the combination id, the target case id and the source case id, the
    row's actual identity.
- **Where the typed basis appears.** `held_status_operand` is a new member of results.v0.3's and
  stress_neutral_export.v0.3's closed basis enum and of the tables' basis vocabulary. Its own row
  class has its own required members; the old enum is never widened silently. The readers admit it
  only on a `minuend_final` combination's own rows.
- **It never fakes case rows.** No held row has a `load_case` basis_ref or a case-qualified id, and
  the target case's own rows are byte-identical to the document without the combination.
- **Difference rows.** The O − S_held rows cite the minuend's case row and the held-operand row, keep
  `metadata.basis` `explicit_user_result_state_subtraction`, and state §7.8 item 5's sign.
- **Held states.** Nonlinear state rows are never combined. The held states, decisions and
  conditions are in the evidence record.

**Contract evidence.** `contract_evidence.held_status_derivation` is optional, sits beside
`difference_basis_derivation`, and is present only when a `minuend_final` combination reached the
held gates. There is one record per such combination, in gate order. There are two evidence
contracts per support, kept apart:

```json
{"combination_id": "comb:o-s-held", "subtrahend_support_state": "minuend_final",
 "source": {"load_case_id": "case:O", "solver_mode": "sparse_interactive",
            "qualification": "qualified", "standing": "confirmed"},
 "target": {"load_case_id": "case:S", "modulus_basis_key": null},
 "outcome": "admitted",
 "held": [
   {"support_id": "support:PROP", "node": "node:T", "dof": "UZ", "behavior": "one_way",
    "sense": "positive_reaction", "held_state": "inactive", "classified_state": "active",
    "decision": "ordinary", "released": "changed_state",
    "inputs": {"ground_bits": "…", "displacement_bits": "…", "reaction_bits": "…"}},
   {"support_id": "support:SHOE", "node": "node:C", "dof": "UX", "behavior": "friction",
    "held_state": "sliding", "classified_state": "sliding", "decision": "ordinary",
    "released": "branch",
    "inputs": {"ground_bits": "…", "displacement_bits": "…", "reaction_bits": "…",
               "coefficient_bits": "…", "normal_source": "derived",
               "normal_source_ref": {"support": "support:Z", "node": "node:C", "dof": "UZ"},
               "normal_trial_bits": "…",
               "normal_signed_bits": "…", "beta": 1, "branch_admissible": false,
               "slip_sense": 1, "applied_force_bits": "…"}},
   {"support_id": "support:GAP", "node": "node:T", "dof": "UZ", "behavior": "gap",
    "closes_when": "negative_displacement", "held_state": "active", "classified_state": "inactive",
    "decision": "strict_gap", "released": "changed_state",
    "strict_gap": {"prior": "active", "gap_sign": "negative", "reaction_sign": "positive"}}],
 "conditions": [],
 "standing": {"kind": "not_established", "reason": "…"}}
```

- **`decision: "ordinary"`** applies to every non-gap support, and to gap supports when the strict-gap
  evidence is `Unsupported` (inspection only). `inputs` carries exactly the binary64 values the
  classifier read:
  - the trial δ (u − g where the case moves the ground, otherwise u) and the reported reaction;
  - for friction: μ, the trial normal (\|R_n\| or the explicit N), the signed normal, β, the branch
    verdict, the slip sense and the applied force.
- **`decision: "strict_gap"`** applies to gap supports when the strict-gap evidence is `Qualified`.
  The decision uses exact-ratio signs from the exact proof, not the published binary64 values
  (`structural_adapter.rs:3248–3265`). Published u and r are projections (`:3290–3324`), so a
  reclassification from their bits can disagree. The record carries the `GapDecision`'s prior,
  `gap_sign`, `reaction_sign` and state, never binary64 inputs as decision inputs.
- **`released`** is `none`, `changed_state` or `branch`.
- **`conditions`** is present for outcome E only. It holds one entry per support: the kd5 §2.2
  condition applied, its class (`input_decision` or `strict_ball`), and for a strict ball test the
  certified row's published value and radius where `ConfirmedState` returns one (§7.3).
- **A withheld record** has an empty `held` list and no `conditions`.
- **Values** are raw SI binary64 bits. Records grow with the nonlinear supports, bounded by the
  model's limits; no new threshold is added.
- **A derived normal's source** (`normal_source_ref`, derived entries only): the support, its node
  and the DOF whose reaction the classifier read, closed and typed.
- **Provenance.** A record withheld by `HELD_STATUS_SOURCE_UNAVAILABLE` states no qualification and
  no standing; every other record states both, `inspectable_unqualified_gap` exactly under
  `HELD_STATUS_SOURCE_UNQUALIFIED` and `qualified` otherwise.

**Readers** (RE `preview_held_status.rs`, called from `preview_combination_gates::check`; Python
`tests/verifiers/preview_held_status.py`).
- **Without the model (option A, as §6.5).**
  - **Shape.** Closed shape, canonical order and finite bits (`…_HELD_STATUS_EVIDENCE`).
  - **Every record** (`…_HELD_STATUS_BINDING`): the combination is a declared `minuend_final`
    subtraction; source = minuend and target = subtrahend; the source mode is the document's; the
    outcome is a held code or `admitted`. The source's qualification follows the outcome as above,
    and its standing is `confirmed` exactly when the minuend's integrity diagnostic carries kd5
    §3.3's confirmed evidence line. The target's `modulus_basis_key` is the one the subtrahend's
    modulus record in this combination states (null: `material_base_values`). Each derived normal's
    `normal_source_ref` names the support and DOF of the minuend's derived-normal row; an explicit
    normal has no such row.
  - **Admitted records only** (coverage, `…_HELD_STATUS_BINDING`): `held` covers exactly the
    minuend's published `nonlinear_support_active_set_state_code` rows, by support, with each
    `held_state` equal to its code and its node and DOF equal to the row's location (a missing,
    extra or forged entry is named). The held-operand rows exist and cite their combination and no
    case.
  - **Ordinary decisions** (`…_HELD_STATUS_DERIVATION`): each `classified_state`, `released` and the
    branch verdict are re-derived from `inputs` by §7.2's rules, in binary64 exactly as the
    classifier evaluates them, in the classifier's order (`lib.rs:1834–1870`): a normal N^T ≤ 0 is
    Inactive; then an inadmissible derived branch keeps Sliding and skips the tangential test; then
    the limit fl(μ·N^T) selects the rule. A zero limit (μ = 0, or a positive μ whose product
    underflows) keeps Sliding iff no force was applied and δ_t ≠ 0. A nonzero limit requires the
    strict products φ·δ_t < 0 and r_t·δ_t < 0. A reader that branches on μ > 0 instead of the
    product rejects truthful evidence (H-15c).
  - **Strict-gap decisions:** only the decision rule over the published exact signs and prior is
    checked. The reader never reclassifies a strict-gap decision from binary64 bits, and it does not
    verify the exact signs, which are the proof's.
  - **Convergence** (`…_HELD_STATUS_DERIVATION`): `held_solve.converged` holds exactly when no
    support is released; a non-converged result carries `NONLINEAR_SUPPORT_NONCONVERGENCE` with a
    non-empty message, and a converged one no exit diagnostic.
  - **Standing:** `HELD_STATUS_RELEASED` is present exactly when some `released ≠ none`, and names
    exactly those supports. `standing.kind` is `established` only with no release and no gap, and
    `gap_inspection` with a gap. `conditions` is present exactly for `established`, and each
    entry's condition and class match the support's behaviour and state by kd5 §2.2's table. An
    input decision is re-derived from the published inputs.
  - **Rows** (`…_HELD_STATUS_ROW`): §7.8 item 5's sign only on the difference rows; the typed basis
    only on held-operand rows.
- **With the model.** The model-free checks first. Then:
  - each support's node, DOF, behaviour, sense or closing direction, gap, μ, explicit normal and
    derived-normal source (its support, DOF and that support's node) are checked against the model;
  - the target's modulus key is the subtrahend case's own;
  - the grounds are re-derived from both cases' authored imposed motion, and with them the
    sliding-ground rule and the trial δ's ground term;
  - the gate derivation is re-derived.

  Where either case uses `condition_use`, the reading is refused as
  `SOURCE_PREVIEW_PHYSICS_COMBINATION_GATE_UNVERIFIED`, as 1b does.
- **What is claimed.** Both readings detect a forgery of what they check: shapes, bindings,
  decisions against their published inputs and, with the model, the support assignment and inputs.
  **Solved values are never recomputed** (u, r, φ, R_n, the strict-gap signs and the certified rows),
  in either reading (PRD §12.8). A forgery that alters solved values coherently with their decisions
  passes both readings. Revision 0's claim that the model-bound reading refuses every coherent
  forgery is withdrawn.
- **Slice 1a operand standing** (`preview_physics.rs:1359`). The subtrahend operand of a
  `minuend_final` combination states `support_state: "held_at_minuend_final"` and the held standing,
  and `preview_combination_operands.rs` admits that member. `…_OPERAND_STATES_DIFFER` is not emitted
  for the held pair; `HELD_STATUS_RELEASED` carries that comparison.
- **Interim removal and gate lists.** `kept_combinations` stops removing `minuend_final` (RE
  `preview_difference_basis.rs:360–378`, Python `:287–297`), and the admitted gate-code lists gain
  the held codes (`preview_physics_evidence.rs:60–70`, `.py:80–86`, `preview_combination_gates.py:269`).
- **History.** The historic recorded variants are preserved: the §10.4 registry gains the moved
  tables verbatim, as §6.7 did, with no silent reseal (Agent 0).

### 7.7 (f) The hanger-facing callable contract

This is for DEL-04-11's O-1. Integration is excluded from this work.

```rust
/// operating-case §7: `target` solved with every nonlinear support held at `source`'s final status.
/// One step of the loop's own unit through its shared tail, then one confirmation attempt.
pub(crate) fn solve_case_at_held_status(
    context: &CaseSolveContext<'_>,   // the invocation's token, model, basis states, boundary,
                                      // springs and solver mode, as its case solves used them
    target: &PreviewLoadCase,         // as authored; the context resolves it
    source: &HeldStatusSource,        // from a source case's LoadCaseSolve in this invocation
) -> Result<HeldCaseSolve, HeldStatusRefusal>;

pub(crate) struct HeldCaseSolve {
    pub results: Vec<ResultItem>,       // the held system's ordinary rows; never a case's rows
    pub record: serde_json::Value,      // §7.6's held_status_derivation record
    pub standing: HeldStanding,         // Established | NotEstablished { reason } | GapInspection
    pub released: Vec<ReleasedSupport>, // id, held and classified state, reason, deciding quantity
}
pub(crate) struct HeldStatusRefusal { pub code: &'static str, pub message: String }
```

**Semantics.**
- **Held loads.** W is the held system's reactions at the candidates: the weight case's own loads
  with every nonlinear support held at S_O,design's final status (§7.2).
- **Same invocation and model.** The source must come from the **same invocation** (token) and the
  **same model context** (§7.4 checks 3–6).
  - A sizing request mints its own context.
  - A derived model that changes the assembled input must solve its own source.
- **Refusals.** Every refusal is a `HeldStatusRefusal` with a `HELD_STATUS_*` code. Sizing maps it to
  `HANGER_SIZING_DERIVED_SOLVE_FAILED` carrying the code and the derived solve's name.
- **No G-1.** G-1 is the subtraction's gate, not the callable's, so a sizing request can hold a
  source with a moved sticking ground against a target with another (H-17).
- **Releases.** `released` non-empty is R or G, including a reversed derived normal. Sizing must
  disclose it (its §5) and never present W as the weight case's admissible state.
- **Standing.** `HeldStanding::Established` carries E's sentence. It says nothing of the weight
  case's own state, which sizing publishes separately.
- **Mode.** The context's.

### 7.8 (g) Old → new texts (final candidate text, for the numerical contract review)

Phase 2 applies the reviewed text with the code. None of it makes a code-compliance claim.

1. **DEL-05-02 ScopeOfWork, DEL-05-02:3.**
   - **Old:** "*Slice 1b (candidate, branch `claude/rt-a5-slice1b`, not yet integrated; reviewed):*
     … EX refuses the member by name. The minuend_final slice is open (seam:
     `Work/operating-case.md` §6.9)."
   - **New:** "*Slice 1b (#107):* … EX refuses the member by name. *minuend_final (candidate, branch
     `claude/rt-a5-minuend-final`):* the subtrahend is solved with each nonlinear support held at the
     minuend's final status by one held-status function (`Work/operating-case.md` §7), published
     under the combination as typed held-operand rows with their held standing and held-status
     evidence, read with and without the model; hanger sizing's `design_operating_final` is not
     integrated."
2. **DEL-05-02 `deliverable.yaml`.**
   - **`code_paths`** gains `core/product_physics/src/held_status.rs`,
     `core/product_physics/tests/op_mf_held_status.rs`,
     `core/reporting/result_export/src/preview_held_status.rs` and `_tests.rs`,
     `tests/verifiers/preview_held_status.py`, `tests/test_preview_held_status.py` and
     `validation/references/op_mf_held_status/**`.
   - **The `from: DEL-04-07` condition** appends: "It also needs DEL-04-07's held-status step
     (`nonlinear_integration::solve_held_status_iteration`): one step of the assembled loop's unit at a
     given support-state assignment, completed by the loop's own shared tail (`complete_iteration`),
     receipt-bound, and followed by one unchanged confirmation attempt, for `minuend_final`
     (`Work/operating-case.md` §7)."
   - **DEL-04-07's K-1 need** gains, after "the slip direction and normal branch taken from the
     previous iterate": "; these rules sit in the shared tail `complete_iteration` and the step since
     RT-A5 S2".
3. **preview-physics-2 `combination_policy`** (retained-1 by hand, timoshenko-1 by `derive`).
   - **`policy_id`:** "PREVIEW-PHYSICS-1-COMBINATION-v3" → "…-v4".
   - **`result_state_subtraction`.**
     - **Old tail:** "; a subtrahend_support_state minuend_final is withheld before the gates
       (LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED) and own behaves as absence".
     - **New tail:** "; own behaves as absence; with subtrahend_support_state minuend_final the
       subtrahend is the subtrahend case's loads solved once with each nonlinear support held at the
       minuend's final status (a held-constraint comparison, which can need a reaction a unilateral
       support cannot supply and is not an admissible sustained state), published as rows of basis
       held_status_operand under the combination, never as the case's rows; its rows' sign
       convention reads \"<item 5>\" and contract_evidence.held_status_derivation states the held
       assignment, each support's decision and its inputs, any release, and the held standing".
   - **`difference_basis_gates`** appends, in order:
     - HELD_STATUS_SOURCE_UNAVAILABLE: "the minuend case has no converged nonlinear support state
       to lend";
     - HELD_STATUS_SOURCE_UNQUALIFIED: "the minuend's converged state is not qualified (an
       inspectable unqualified gap state)";
     - HELD_STATUS_BINDING: "the minuend's state does not bind to this invocation, to its own
       receipt-bound solve, or to the subtrahend's model context and nonlinear supports one for one";
     - HELD_STATUS_SLIDING_GROUND_DIFFERS: "a support held sliding has a different ground offset in
       the two cases";
     - HELD_STATUS_SOLVE_FAILED: "the held solve was refused, blocked or singular".
   - **`gate_precedence`** appends "; a minuend_final subtraction then takes the held-status gates in
     list order after G-2".
   - **Basis vocabulary** gains `held_status_operand`: "a row of the held system that a minuend_final
     subtraction subtracts, published under the combination; not a load case's result and not an
     admissible sustained state".
4. **PF `LIMITATIONS[6]`** is not moved (Agent 0).
5. **Row sign text (`held_status::SIGN`):** "positive value follows explicit user result-state
   subtraction (minuend minus subtrahend) of matching source result sign conventions; operating
   minus sustained difference at the operating support status: the minuend case's own solved state
   minus the subtrahend case's loads solved with each nonlinear support held at the minuend's final
   status, a held-constraint comparison that can need a reaction a unilateral support cannot supply
   and is not an admissible sustained state, under the static, history-free friction model; derived
   from case results; not an equilibrium state". The held-operand rows' clause: "held-status operand
   of <combination>: the subtrahend case's loads solved with the minuend's final support status
   held; not the subtrahend case's result".
6. **E's standing sentence** (Agent 0's wording): "Established within its stated numerical bound for
   the explicitly held system at this case's loads. The verdict does not establish this case's own
   support state, an admissible sustained state, a load history or uniqueness."
7. **DEL-02-01 `model-document-0.9.0.md`.**
   - **§2.2.**
     - **Old:** "Interim until its slice: `own` behaves as absence; the solve withholds a
       `minuend_final` combination with the warning
       `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_UNSUPPORTED`, and the applier refuses authoring it."
     - **New:** "`own` behaves as absence. On the pressure-free route the solve forms a
       `minuend_final` combination by DEL-05-02 `operating-case.md` §7 and the applier authors it; EX
       refuses the declared basis by name first."
   - **§3.8 row.**
     - **Old:** "… | solve; applier | RT-A5's `minuend_final` slice (DEL-05-02 `operating-case.md`
       §5) |"
     - **New:** "… | solve; applier | RT-A5's `minuend_final` slice (DEL-05-02 `operating-case.md`
       §7); removed by it on PF |".
8. **Applier** (`lib.rs:6276–6293`).
   - The rustdoc's "a `minuend_final` subtrahend support state stays refused until its solve is built
     (`own` behaves as absence)" becomes "a `minuend_final` subtrahend support state is authored on
     the pressure-free route (operating-case §7); `own` behaves as absence".
   - The refusal arm is removed.
9. **Docs.**
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
10. **`docs/RELEASE_TARGET_v1.0.md` RT-A5 status** (Agent 0 applies the reviewed text).
    - **Old:** "`minuend_final` unbuilt"
    - **New:** "`minuend_final` (candidate, `claude/rt-a5-minuend-final`): the subtrahend solved with
      each nonlinear support held at the minuend's final status, published under the combination as
      a held-constraint comparison with its own standing (DEL-05-02 `operating-case.md` §7)".
11. **This document** (applied with phase 2).
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
    - **§0 R-1, the Reason cell.** The old text is kept, and this is appended: "Status (2026-11-04):
      the owner-accepted release target (2026-10-28) carries frozen operating status as RT-A5
      Should. It is designed in §7 as the declared held-constraint comparison
      `subtrahend_support_state: minuend_final`; own status stays the default and is never inferred."
    - **§0 "§5 B", the Decision cell.**
      - **Old:** "The held-constraint option (2) remains R-1's remaining item for the owner, and can
        be added later as a second declared basis without renaming."
      - **New:** "The held-constraint option (2) is the declared `minuend_final` comparison of §7,
        added without renaming; the engineering meaning of either difference stays with the user's
        rule pack."
    - **§1.4, the member row.**
      - **Old:** "`combinations[].subtrahend_support_state` (later, only if the owner chooses §5
        option 2)"
      - **New:** "`combinations[].subtrahend_support_state` (RT-A5 Should; §7)".
12. **DEL-04-11 `hanger-sizing.md` O-1** is the hanger owner's. §7.7 is the contract sent, not edited
    here.

### 7.9 (h) Reference plan

**Where and how.**
- The references live in `validation/references/op_mf_held_status/`, with the generator at
  `generators/held_status_reference.py`. It uses the standard library only: exact
  `fractions.Fraction`, an exact rational 2D Euler–Bernoulli frame in the XZ plane, and enumeration
  of every candidate state.
- It reads no product code, uses invented values and has op1b's layout.
- **Recorded values.** Every value below was computed by an independent scratch generator of that
  design, run with `python3 -I` from `/tmp/mf1-gen-20261104b/` on 2026-11-04. It enumerates own
  states and keeps the unique admissible one; held solves apply §7.2's rules once. The generator
  committed in phase 2 must reproduce these values.
- **Comparison.** Values are exact rationals; dyadic ones are written exactly, and others are shown
  with their nearest binary64. The comparison rule is op1b's 1e-9.
- **Product models.** The product models are 3D, with the generator's plane as XZ and every
  out-of-plane DOF unloaded.

**Models.**
- **The bar.** Along X, anchored at x = 0, L = 4 m, EA = 2²², so k = 2²⁰ N/m; α = 2⁻¹⁶ /K, so
  ε = α·ΔT and δ_T = 4ε.
- **The tip-UZ bar** adds a UZ base restraint at the tip, the derived-normal source, with R_n = the
  vertical reaction. A vertical tip load goes straight into it.
- **The V-2 cantilever.** s\* = 5PL³/(48EI) = 2⁻¹⁰ m.
- **The L-frame (H-14).**
  - Nodes A(0,0,0), anchored; B(1,0,0); C(1,0,−1).
  - Members AB and BC, EA = 2²⁴ N, EI = 2²⁰ N·m².
  - At C: a UZ base restraint as the normal source, and UX friction with a derived normal,
    μ = 1/4.
  - The coupling is g = ∂R_n/∂φ = −24/19, so μg = −6/19: strongly coupled.
- **Friction.** μ = 1/4 unless stated.

**Case references.** Displacement rows are in m, forces in N; φ is the friction force on the pipe.
Every O and S combination satisfies G-1 except H-17, which is callable-level.

| Id | Loads (S ⊂ O by content) and held behaviour | O own | S own | S held | O − S_held | Outcome |
|---|---|---|---|---|---|---|
| H-1 | V-2, rise s = 2s\*; prop held Inactive | inactive, u_tip = s\*, R = 0, M_A = PL/2 | active, R = 5P/16 | u_tip = −s\*, R = 0, M_A = PL/2 | Δu = 2s\*, ΔR = 0, ΔM_A = 0 | R (changed: penetration) |
| H-1b | V-2, s = s\*/2; prop held Active | active, R = 5P/32 | active, R = 5P/16 | = S own | = 1b's O − S, bit for bit | E or N |
| H-2 | bar, one-way +; S: F = 512; O: F, ΔT = −16 K | active, u = 0, N = 1024, R = 512 | inactive, u = 2⁻¹¹, N = 512 | u = 0, N = 0, **R = −512** | Δu = 0, ΔN = 1024, ΔR = 1024 | R (changed: tensile) |
| H-3 | bar, explicit N = 4096; S: F = 2048; O: F, ΔT = −24 K | sticking, u = 0, N = 1536, R_t = −512 | sliding +, u = 2⁻¹⁰, φ = −1024 | u = 0, N = 0, **R_t = −2048** | Δu = 0, ΔN = 1536, ΔR_t = 1536 | R (changed: capacity) |
| H-4 | as H-3, O: ΔT = −8 K (V-3) | sliding +, u = 2⁻¹¹, φ = −1024 | sliding +, u = 2⁻¹⁰, φ = −1024 | = S own | Δu = −2⁻¹¹, Δφ = 0 | E (kept) |
| H-5 | as H-3, O: ΔT = −64 K | sliding −, u = −2⁻¹⁰, N = 3072, φ = +1024 | sliding +, u = 2⁻¹⁰ | φ = +1024, **u = +3·2⁻¹⁰** | Δu = −2⁻⁸, ΔN = 0 | R (changed: slip sense) |
| H-6 | tip-UZ bar, derived; S: V = 4096 down, F = 2048; O: V, V′ = 4096 down, F, ΔT = −80 K | R_n = 8192, sliding −, u = −2⁻¹⁰, φ = +2048 | sliding +, u = 2⁻¹⁰ | R_n = 4096, φ = +1024, u = 3·2⁻¹⁰ | Δu = −2⁻⁸, Δφ = 1024 | R (changed) |
| H-6b | as H-6 with F = 3072 in both; O: ΔT = −8 K | R_n = 8192, sliding +, u = 2⁻¹¹, φ = −2048 | sliding +, u = 2⁻⁹, φ = −1024 | = S own (frozen-normal mutant: φ = −2048, u = 2⁻¹⁰) | Δu = −3·2⁻¹¹, Δφ = −1024 | E or N |
| H-7 | V-1 bar gap closing +, c = 2⁻⁹; S: F = 1536; O: F, ΔT = +16 K | active, u = c, N = 1024, stop −512 | open, u = 3·2⁻¹¹ | u = c, N = 2048, **stop +512** | Δu = 0, ΔN = −1024, Δstop = −1024 | G + R, or `SOURCE_UNQUALIFIED` per the source's qualification |
| **H-7g** (product witness for G) | PP rendering of NI `p_tests.rs` `gap_chain(1)`: unit cantilever along X, E = 100, A = 1, I = 1 (bending in the gap's plane), tip gap closing in −, c = 2⁻¹²; S: 2⁻⁶ N toward the gap; O: S plus 1 N | active, u = −2⁻¹², R = 965/1024 | open, u = −1/19200 | u = −2⁻¹², **R = −59/1024 (pulled)** | Δu = 0, ΔR = 1 | G + R. Required end to end: the source must reach `Qualified` (NI `p_tests.rs:100–105`; PP `lib.rs:7532–7536` maps it), and phase 2 asserts it |
| H-13 (normal reversal) | tip-UZ bar, derived; S: U = 4096 up, F = 2048; O: U, V′ = 8192 down, F, ΔT = −8 K | R_n = +4096, sliding +, u = 2⁻¹¹, φ = −1024 | R_n = −4096, sliding + (its own β = −1), u = 2⁻¹⁰, φ = −1024 | β^S = +1, **R_n^T = −4096**, φ = +1024, u = 3·2⁻¹⁰; classified **Sliding**, branch inadmissible | Δu = −5·2⁻¹¹, Δφ = −2048, ΔR_n = 8192 | R (branch) |
| H-14 (strongly coupled) | L-frame; S: W = 4096 N down at B, P = 2048 N +X at C; O: S, ΔT = +64 K on AB | sliding +, u_C = 617/319488, R_n = 16384/13, φ = −4096/13 | sliding +, u_C = 305/319488 (9.5465244391025638e-4), R_n = 16384/13 (1260.3076923076924), φ = −4096/13 | = S own (uncoupled mutant: φ = −4096/19, u_C = 1.0271715215258541e-3) | Δu_C = 2⁻¹⁰, ΔR_n = 0, Δφ = 0 | E or N (k = 1, Stage A) |
| H-14r | L-frame; S as H-14; O: S, P′ = −4096 N at C | sliding −, u_C = −593/614400, R_n = 114688/25, φ = +28672/25 | as H-14 | β^S = +1, R_n = 16384/25 (655.36), φ = +4096/25 (163.84), **u_C = 267/204800 (1.3037109375e-3)** (frozen mutant: φ = 1146.88, u_C = 2.0201994243421052e-3; uncoupled mutant: φ = 4096/19, u_C = 1.3414208578601107e-3) | Δu_C = −697/307200, ΔR_n = 98304/25, Δφ = 24576/25 | R (changed: slip sense) |
| H-15a (μ = 0) | bar, explicit N = 4096, μ = 0; S: F = 512; O: F, ΔT = −16 K | sliding −, u = −2⁻¹¹, φ = 0 | sliding +, u = 2⁻¹¹ | sliding (held −, no force), u = 2⁻¹¹; kept (δ ≠ 0) | Δu = −2⁻¹⁰ | E or N |
| H-15b (μ = 0, no slip) | as H-15a; S: 512 N at the anchor node only; O: S, ΔT = −16 K | sliding −, u = −2⁻¹⁰ | sticking, u = 0 | u = 0; classified **Sticking** | Δu = −2⁻¹⁰ | R (changed) |
| H-15c (positive μ, zero limit) | as H-15a, with explicit N = 1/4 and μ = 2⁻¹⁰⁷⁴ (fl(μ·N) = fl(2⁻¹⁰⁷⁶) = 0, so the step forms no force) | sliding −, u = −2⁻¹¹, φ = 0 | sliding +, u = 2⁻¹¹ | sliding (held −, no force), u = 2⁻¹¹; kept (zero limit, δ ≠ 0) | Δu = −2⁻¹⁰ | E or N. Mutant: a μ > 0 rule requiring strict force products classifies Sticking (R) and must be killed. If the document validator refuses a subnormal μ, this witness is callable-level (NI wrapper), and that refusal is recorded |
| H-16a (inactive, input decision) | bar, explicit N = 0, μ = 1/4; S: F = 512; O: F, ΔT = −16 K | inactive, u = −2⁻¹¹ | inactive, u = 2⁻¹¹ | inactive, u = 2⁻¹¹, kept | Δu = −2⁻¹⁰ | E, with an `input_decision` condition |
| H-16b (inactive, derived) | tip-UZ bar, derived; S: V = 4096 down, F = 512; O: V, U = 4096 up, F (so R_n^O = 0 exactly) | inactive, u = 2⁻¹¹ | sticking, u = 0, R_t = −512 | inactive, u = 2⁻¹¹; trial normal 4096 > 0, so **Sticking** | Δu = 0 | R (changed) |
| H-17 (sticking, moved ground; **callable level**, no G-1) | bar, explicit N = 4096; source: F = 2048 (as 768 + 1280), shoe ground g^S = 3·2⁻¹¹; target: F = 768, g^T = 2⁻¹¹ | sticking, u = 3·2⁻¹¹, R_t = −512 | sticking, u = 2⁻¹¹, R_t = −256 | u = 2⁻¹¹, N = 512, R_t = −256, kept (**hold-source-displacement mutant:** u = 3·2⁻¹¹, R_t = +768) | Δu = 2⁻¹⁰, ΔR_t = −256 | E or N |
| H-17c (the same at combination level) | as H-17, target: F = 768, no ground motion (g^T = 0) | as H-17 | sticking, u = 0, R_t = −768 | u = 0, R_t = −768, kept (mutant: u = 3·2⁻¹¹, R_t = +768) | Δu = 3·2⁻¹¹, ΔR_t = 256 | E or N |
| H-18 (weak contact) | bar, one-way +; S: 512 N at the anchor node only; O: S, ΔT = +16 K | inactive, u = 2⁻¹⁰ | inactive, u = 0 | u = 0 exactly: kept (no penetration); the separation ball contains 0 | Δu = 2⁻¹⁰ | **N** ("separation not established") |

- **H-3.** H-3 cannot kill the hold-source-displacement mutant, since its source sticks at its ground,
  0, which is also the target's. H-17 and H-17c kill it. Under G-1 a target ground primitive must
  equal the source's by content, so a nonzero, differing pair exists only at the callable level.
- **Linear held twins.** These are the same models with each held support replaced by its held
  linear boundary, solved on the linear route:
  - H-1: the prop removed;
  - H-2 and H-3: a rigid UX restraint at the tip;
  - H-17: a rigid UX at the tip with prescribed 2⁻¹¹;
  - H-4: −1024 N at the tip; H-5: +1024 N; H-6: +1024 N; H-13: +1024 N;
  - H-15 and H-16: no restraint.

  The rows equal S_held's closed forms by the 1e-9 rule. Bit equality between routes is not
  claimed.

**The remaining references.**
- **H-8, bindings** (PP tests through `solve_case_at_held_status`, each refused by its name, §7.4):
  - a missing, extra or repeated support state;
  - a friction support held `Active`;
  - another invocation's source, built identically (token);
  - `evidence` not equal to the minuend's `nonlinear_state`, or its case id or mode differing;
  - one bit of the source's `solve.displacements` or `reactions` edited (the receipt fails);
  - each model-context member changed in turn: an element, a base DOF, a support's node, DOF,
    behaviour, gap bits or μ bits, an explicit-normal value, a derived-normal source node or DOF, a
    spring and a node id;
  - a source orienting sense of zero (defensive);
  - a sliding ground that differs (R-b ground motion on the shoe in O only);
  - an `InspectableUnqualifiedGap` source;
  - a linear model (`SOURCE_UNAVAILABLE`);
  - a held system made singular by a held lift-off (`SOLVE_FAILED`, with the document not blocked).
- **H-9, repeatability and modes.** Each held document is solved twice in one process and once in a
  fresh process, with identical bytes. Every closed form is checked in dense and in sparse mode.
- **H-10, the projection test.** It has two uses: **S2's extraction proof**, which is the loop's
  output before and after `complete_iteration`, and **the held wrapper against the loop**, which is
  the wrapper at a case's own final states with its own penultimate iterate, and separately with its
  own final iterate, against the loop's final iteration. Both run in both modes over every committed
  nonlinear model's converged cases.
  - **Compared, bit for bit:**
    - the boundary (DOFs and prescribed bits);
    - the applied sliding forces and the final force;
    - the displacement and reaction bits, and the sparse evidence of the final solve;
    - the classifier decisions: the active-set states, the changed supports and the residual
      count, the tangential and derived-branch admissibility, and the strict-gap evidence and
      decisions, compared as the strict-gap fields enumerated below;
    - the converged predicate;
    - `product_selected_state_is_qualified`'s verdict, **as a value**: false verdicts are preserved
      and compared, never required to be true. A converged but unqualified state (the committed
      mixed-gap inspection case, NI `lib.rs:5527–5535`, converged with qualification false) must
      give false on both sides;
    - the confirmation's outcome: its variant, and its reason or its `ConfirmedState` rows' bits.
  - **Excluded, named:** the iteration number and history, the residual deltas from a previous
    iterate, the receipt's `initial_states` and history, the first-iteration deferral and recovery
    flags, and the order and position of policy and exit diagnostics.
  - **Strict-gap evidence (`ExactGapReport`), wrapper against loop.** Compared: `free_dofs`,
    `prescribed`, `decisions`, `projections`, `ordinary_displacements` and
    `corrected_coordinate_count`. Excluded, named: `source_identity` (it embeds the iteration
    number, `structural_adapter.rs:3199`), `work` and the private retained proof, limits and work
    snapshot (budget and proof metadata that depend on the iteration's position, `:2700–2715`). For
    **S2's extraction proof** (the same loop before and after `complete_iteration`) nothing is
    excluded: every output, including the full strict-gap report, the iteration records and the
    receipt, is byte-identical.
  - **Each receipt against its own inputs: authentication, not qualification.** Receipt validity
    is `product_completion_matches_source` (`lib.rs:1056`): the held receipt must authenticate
    against `held_input` and its own one-iteration result, and the loop's against the loop's
    input and result. Qualification is a separate projected verdict (above), and an authenticated
    receipt may carry a false one. A
    one-field mutation of either result's public members invalidates its own receipt (as
    `successor_completion_receipt_rejects_public_report_and_source_mutations`).
- **H-11, negatives and order:**
  - G-1 and G-2 failures withhold before any held gate, and the held solve is not run (an
    instrumentation count);
  - the member on `mechanics` or `range_envelope` is `…_MISPLACED`;
  - `own` is byte-identical to absence;
  - EX blocks first by name;
  - a non-converged minuend blocks the document as today;
  - a `minuend_final` combination named by another combination withholds its dependant whole when
    it is itself withheld;
  - the target case's own rows are byte-identical with and without the combination.
- **H-12, mutants** (each restored after its run):
  - holding the source displacement (H-17, H-17c);
  - freezing the source normal (H-6b, H-14r);
  - an uncoupled normal (H-14, H-14r);
  - the slip sense taken from the target's motion (H-5);
  - treating an inadmissible branch as kept (H-13);
  - skipping the classifier (H-2);
  - reclassifying a strict-gap decision from bits in the reader (a tamper whose exact signs disagree
    with the bits is accepted with its signs);
  - held rows published as case rows, or with the old basis;
  - "confirmed" wording on a held standing;
  - coverage checks applied to a withheld record (the withheld corpus member is refused);
  - each reader binding check disabled in turn (its tamper);
  - a §4 lookup of own-case bounds for a held combination (refused by RT-A3's exclusion).
- **Readers.** The shared corpus `fixtures/results/preview_physics_held_status_corpus.json` is the
  producer's output, and every document reads with and without its model. There are tampers, each
  refused by name in both languages, for:
  - each binding field, each `inputs` value and each classified state;
  - `released` and RELEASED in either direction;
  - the standing kind, and the `conditions` presence and class;
  - the sign text and the typed basis;
  - a held row cited as a case row;
  - the source mode;
  - a coverage check on an admitted record;
  - a strict-gap rule violation over published signs.

  Plus one coherent solved-value forgery, which passes both readings by design (§7.6).

### 7.10 (i) Pricing outlook (no registration in this work)

The change moves the retained route's statics and code, and its registration is DEL-04-08's after
phase 2. There are two D3 slots: S2's own, then the held slice's.
- **S2 (NI `lib.rs`).** `complete_iteration`, `IterationContext`, `LoopState` and `IterationExit`.
  - Its cost is measured and its pins restated. Coefficient identity is to be proven.
  - Frame, stack and lifetime impact are assessed independently: the borrows in the context, the
    moved `SlidingIterationSolve` and `LoopState`'s ownership of `iterations`.
  - The loop is D1-refused, which is shown, not assumed.
- **The held slice.**
  - **PP.** `LoadCaseSolve` +8 B (the carrier 1,888 → 1,896 B, and +8 B more if §4's
    `certificate_operand` lands). The held dispatch in `solve_prepared_load_case`, the capture, the
    held solves after the case loop, and `held_status.rs`, `preview_physics.rs` and
    `document_members.rs`. The retained source copy (input, result, iterations) exists only for
    `minuend_final` minuends, so it is zero on D1.
  - **NI.** The wrapper, `HeldStatusSolve` and the invocation token's type.
  - **Tables.** Combination policy v4 in the three preview tables (STATICS, as §8.51's v3), with the
    recorded-run registry gaining the moved tables.
  - **Rules** (`o10_rules.py`). The S1 `minuend_final` push entries (`:4113`, `:4226–4228`) drop. The
    A4 family counts (`:4776–4805`, `:5211`) stay. New rules rest on D1.4's refusal of every
    difference basis, as §8.51's PF1B rules did.
- **Not changed.** FK, SD, `structural_adapter.rs` and confirmation code.

### 7.11 Seam map for phase 2

Line numbers are on `b8f3837e8`. The order is the S2 extraction commit first (RT-A3 reviews its
boundary), then the held slice.

| Area | File and lines | Phase-2 change |
|---|---|---|
| NI loop tail (S2, first commit) | `lib.rs:766–1024`, the post-step `:878–1018` | moved into `complete_iteration` (§7.3), with `IterationContext`, `LoopState` and `IterationExit` crate-private beside it. The loop keeps `:766–877` (guard, validation, assembly, gap views, the seed and its recovery, the step and `sliding_force_deferred`) and calls the tail |
| NI step (read only) | `lib.rs:1559–1780` | unchanged |
| NI wrapper (new) | `lib.rs`, after `:1780` | `pub fn solve_held_status_iteration`, `pub struct HeldStatusSolve`. It calls `dense_loop_memory_guard`, `validate_input`, `normalized_initial_states`, `LoopSystem::assemble`, `gap_views`, `active_boundary`, the two normal maps, the step once, `complete_iteration(…, Now)` and `confirmation::confirm_converged_state` |
| NI receipt, qualification, confirmation | `lib.rs:355–381`, `:1029–1283`; `confirmation.rs` | unchanged |
| NI tests | `lib.rs` test module or a new `held_tests.rs`; `s11k_tests.rs` (S11-K site assertions for the moved sites) | H-10, the wrapper's validation, S2's identity |
| DEL-04-07 records (S2) | `Work/k1-twin-bound.md` (the re-check paragraph at `:242`), `deliverable.yaml` (the K-1 need, `:298–314`) | the trace re-read, recorded |
| PP `LoadCaseSolve` | `lib.rs:2650–2673`; literals at `:5310`, `:5347`, `:5455`, `:5496`, `:5763`, `:5796`, `:5894`, `:7381`, `:15728` | + `held_status_source` (the `nonlinear_state` line unchanged) |
| PP case solve | `lib.rs:5267–5283` (signature), `:5849–5868` (a held dispatch beside the unchanged call), `:5869–5895` (the held path does not require `converged`), `:5972–5976` (a `NonlinearStanding::HeldEstablished` arm), `:6000–6005` (E's held line in place of the confirmed line), after `:5985` (the capture) | the #97 capture `:5978–5985`, the third tuple member and `confirm && passed && !gap` (`:7546`) are unchanged |
| PP nonlinear append | `lib.rs:7438–7818` | unchanged, except an optional behaviour-preserving extraction of the input construction (`:7488–7513`) shared with the held sibling (a review choice) |
| PP held module (new) | `held_status.rs` | `InvocationToken`, `HeldStatusSource`, `capture_held_status_source`, `solve_case_at_held_status`, the binding checks, `append_held_status_results`, the evidence record, `SIGN` |
| PP document solve | `lib.rs:3257–3412`; insert after `:3412` | the context and its token, and the held solves for admitted `minuend_final` combinations |
| PP combinations | `preview_physics.rs:848–905` (the held gates after G-2), `:973` and `:1036–1046` (the held map, subtrahend lookup and sign), `:1359` (the held subtrahend operand record) | — |
| PP evidence | `lib.rs:3524`, `:3661` | `held_status_derivation` beside `DERIVATION_MEMBER` |
| PP interim | `document_members.rs:826–835` | drop the push; the constant `:69–70` stays |
| Schema | `schemas/results.v0.3.schema.yaml:725`, `:4852` (the typed basis and its row class); `PreviewPhysicsContractEvidence` | stress_neutral_export.v0.3 and analysis_run.v0.3 follow; `schema_manifest.json` regenerated |
| RE | `preview_difference_basis.rs:360–378`, `preview_combination_gates.rs:465–470`, `preview_physics_evidence.rs:60–70`, `preview_combination_operands.rs`, new `preview_held_status.rs` and `_tests.rs` | — |
| Python | `tests/verifiers/preview_difference_basis.py:287–297`, `preview_combination_gates.py:269`, `:361`, `preview_physics_evidence.py:80–86`, `preview_combination_operands.py`, new `preview_held_status.py`; `tests/test_preview_held_status.py`, `test_model_document_0_9_0_members.py:68` | — |
| Applier | `operation_applier/src/lib.rs:6276–6293`, `:15968–15973`; `tests/condition_set_authoring.rs:554–569`, `tests/model_document_0_9_0.rs:43` | — |
| TS | `apps/desktop/src/features/viewport/resultColourOverlay.ts:145–149`, `derivedRowNote.test.ts` | the held sign note and the typed basis |
| Tables and registry | the three `semantic_contract_v0_3_preview_physics_*.json` through `semantic_contract_tables.py`; `recorded_tables::VARIANTS`, `compatibility.py`, `recordedPreRtA4.ts` | §7.8 item 3; historic variants preserved verbatim |
| Pricing | `tools/retained_pricing/rules/o10_rules.py`, `base/s1_owners_model.py:47` | §7.10 |

**Overlaps.**
- **O-P1, P's D3 comment at PP `lib.rs:3209–3214`.** There is no textual overlap.
- **O-P2, P's pins in NI `structural_adapter.rs` and FK.** These are untouched. S2 changes NI
  `lib.rs`, so S2 takes its own D3 slot after P's, and restates any NI pin there.
- **O-D, dynamics.** It has no hunk in or around `:878–1018`. Its future counters sit in
  `LoopSystem::solve_iterate` and `ConfirmationContext` (RT-A3).
- **O-C1, textual (§4, `claude/rt-a3-cert`).** `certificate_operand` and `held_status_source` are
  added at the same `LoadCaseSolve` literal sites. The conflict is mechanical: the branch that merges
  second adds its member beside the other's. The certificate's map sits at `lib.rs:~3447–3475` and
  its `publish` at `:~3693`; the held solves insert at `:3412`.
- **O-C2, semantic: resolved upstream.** RT-A3 is adding on `claude/rt-a3-cert` the exclusion
  "not_assessed for any non-default `subtrahend_support_state`". It stays until the certificate's
  bounds bind the exact held operand, never own-case bounds by id (Agent 0).

### 7.12 Decisions recorded, and what remains open

**Decided (Agent 0 and RT-A3, 2026-11-04).**
- S2, with RT-A3's boundary (§7.3).
- E's wording (§7.3).
- A reversed derived normal is a release.
- The `HELD_STATUS_*` codes.
- Combination-owned held rows with the typed basis `held_status_operand`.
- `LIMITATIONS[6]` unchanged.
- Hanger integration excluded.
- Historic recorded variants preserved.
- O-C2 resolved upstream.
- Policy (b) unchanged.

**Open for the numerical contract review.**
- The final candidate texts of §7.8, including §0 R-1, §5 B, §1.4 and RT-A5.

**Decided by the manager (2026-11-04).** Gap-bearing models are admitted with outcome G: the
source must be `Qualified` (§7.4), no held gap outcome is ever confirmed (R1), and H-7g is the
product witness. No refusal until R-3 is kept.
