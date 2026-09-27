# S11-G — the formation-noise guard (design note)

D1 (Type 2 design TASK), 2026-09-27. Commissioned by ROOT ("I4's F12 stop", `ROOT_RULINGS_V1.md`, option (c)). Brief: `TASK_BRIEFS/D1_S11G_GUARD_NOTE.md` at `c24f8d5b4`, with ROOT's addendum at `42b300344`.

**Basis**
- Pins: `GATE/FORMATION_EXCEPTIONS.json` (7 triples, 14 rows) and `GATE/S11_EXCEPTIONS.json` (221 triples: 87 captured, 134 typed), as re-pinned at `db665f2cb` and amended at `42b300344`. `DESIGN.md` 5a.2 still quotes the older 228-triple pin; the 221 + 7 split used here supersedes that figure. Updating DESIGN's text is not in this note's scope.
- Sources: `S11_CONTAINMENT.md` 5a.2 (§2.2 declared formation, §5, §6, §10 items 2 and 3), `DESIGN.md` 5a.2 (§4.1.6 S\* and the floor, §4.1.6.1 body membership and L_b).
- Code: S11-K on main (`FK/load_ledger.rs`, `FK/exact_sum.rs`, at `3488a236a`), read-only. I4's S11-F candidate on `codex/piping-s11f-20260927`, read-only in I4's worktree. PP line numbers below are that candidate's as read on 2026-09-27; the K-D5 line (PP:3965) is from I3's worktree.
- Frozen references: R1 (`REFERENCES/references.json`, `7b176dbb…`) and P1 (`DETECTION/results.json`, `a1188624…`).

No product code was written or built. The forecast is one standard-library script: `_run_records/s11g_forecast.py` (§10).

---

## 1. Answer in brief

**The load-row guard catches all 6 load-formation rows, and nothing else.**
- The 6 rows are UDL-W1e80 th.S1.RZ (typed, both modes) and UDL-W1e8 th.S1.RZ (captured and typed, both modes). The guard runs before the solve, on the ledger, so it fires in every entry and every mode that builds the case.
- It demotes none of:
  - the 221 repaired triples;
  - any frozen-reference case that passes today, including UDL-W1e5;
  - S11-F's committed probe-A test;
  - any of the 164 committed model/case entries with formed loads.
- **No committed byte changes.**

**It uses signed, exact formation defects instead of a-priori per-term bounds.**
- Each formed term carries the exact value of its own declared formula on the operands the product holds. The guard sums the signed defects exactly.
- Where a term has no implemented exact form, it falls back to a stated bound.
- **Why not a-priori bounds.** The brief's literal Σ|term|·bound form cannot meet its own coverage conditions. A rigorous a-priori bound:
  - demotes UDL-W1e5, which passes today (bound 2.9× over the threshold);
  - demotes S11-F's committed probe-A test (G, 0.3, −G on one member), where q and −q give exactly negated terms and exactly negated defects.
- With signed exact defects, UDL-W1e5 stays silent with a 25× margin and probe A with a margin of about 3e7. UDL-W1e8 fires at 48×, and UDL-W1e80 at 2.6e81×.
- No envelope field is added. The design uses the existing field set, so there is no stop.

**The recovery side: the 4 INPLANE triples (8 rows) cannot be caught in binary64 without demoting passing rows.** Per-row justification for waiting on F2 (W1a) is in §5, for ROOT.
- Every guard that sees these rows is row-relative. Each such candidate demotes 16 to 90 passing case-modes.
- The body-scale bound, which is the product's own accuracy contract, sees none of the 8 rows.
- An exactly rounded recovery from the binary64 displacement would still leave the rows 145–2949× over the criterion. The error is in u, not in the K_e·u arithmetic.
- S11-G therefore ships no recovery-side demotion.

**The conflict with K-D5 is disjoint.**
- K-D5 changes only the solve call inside `solve_preview_reduced_system`, plus FK `finish_checked_factor` and SA.
- S11-G changes the ledger push sites, a new PP module, the source-recovery eligibility line and `append_integrity_report`.
- Either may land first (§9).

---

## 2. The seven triples after S11-F

| Triple (entry) | Rows | Class | Published after S11-F | Ratio to the criterion | S11-G |
|---|---|---|---|---|---|
| UDL-W1e80 th.S1.RZ (typed) | 2 | Load formation | Net of the represented terms 2.63e64; rotation of about 1.2e57 rad | 2.63e81 | **Caught** (load-row guard) |
| UDL-W1e8 th.S1.RZ (captured, typed) | 4 | Load formation | Net 0.4916666902601719 against the intended 59/120 | 47.99 (ROOT's addendum); guard ratio 47.99 (§6.1) | **Caught** (load-row guard) |
| F-G1e80-GnG-INPLANE Mb.M1.j (typed) | 2 | Formed K_e·u | 1e-8 N·m ± 6e-15 (emulated) | 628 (emulated) | Waits for F2 (§5) |
| F-G1e80-GnG-INPLANE Mb.M2.i (typed) | 2 | Formed K_e·u | ± 3.6e-14 (emulated) | 3635 (emulated) | Waits for F2 (§5) |
| M-G1e80-GnG-INPLANE Mb.M2.i (typed) | 2 | Formed K_e·u | ± 1.5e-14 (emulated) | 1504 (emulated) | Waits for F2 (§5) |
| M-G1e80-GnG-INPLANE Mb.M2.j (typed) | 2 | Formed K_e·u | ± 6e-15 (emulated) | 628 (emulated) | Waits for F2 (§5) |

**The UDL rows.**
- The published UDL-W1e8 net is reproduced bit for bit by the script: SP's two formulas on the represented operands, summed exactly (§6.1).
- The UDL coverage does not depend on published response values: the guard acts on the load row before the solve.
- The script's guard ratio equals the published error ratio of 47.99, because the rotation is linear in the net.

**The INPLANE rows.**
- The INPLANE figures are D1's emulation of the S11-F state: the exact net at N2, binary64 dense elimination, and binary64 K_e·u.
- The brief quotes I4's measured errors, up to 1.2e-6 relative. The emulation gives up to 3.6e-6; the elimination order differs.
- ROOT's addendum asks for coverage against I4's post-S11-F records (`IMPLEMENTATION/S11F/_run_records/formation_rows/`). Those are not yet committed. Once they are, the INPLANE column is re-checked against them (§11 item 1). The conclusion in §5 does not depend on the exact ratios.

---

## 3. The load-row guard

### 3.1 Formed terms and input terms

**The rule.** A ledger term is **formed** when the product computes its binary64 value from other held operands with at least one rounding. Its **formation defect** is:

  ε_t = value_t − formula_t(held operands), evaluated exactly.

A term is an **input** when its value is itself a held operand, a correctly represented user or case quantity. It then carries **no defect**, by construction.

**Held operands** are the binary64 values the product holds and uses on both the force side and the recovery side:
- the SI load quantities in `LoadApplication` and the resolved case (nodal values, intensities, pressures, temperature intervals, strains);
- member length L, span fractions (a, b) and transform entries T;
- a load record's `axial_load`.

The guard measures loss **in forming a load term** from those operands, not in representing the model. That is the same boundary as S11's "represented inputs" (§5.3) and K-D5's represented-model stance. Decimal-to-binary parsing and model-geometry formation (L from coordinates, A from OD and t) are shared by K and f. They are W1's class, not S11-G's.

**Consequences.**
- A nodal load, a constant-effort force and every authored nodal (G, n, −G) are inputs: ε = 0.
- Exact input cancellation (the 221) cannot move the statistic. This holds by construction, not by a margin.
- Two formed terms produced by the **same expression** on negated operands, such as q and −q on one member (S11-F probe A), or ±P at the two ends of a straight member, are exactly negated. Their defects are exactly negated too, so the signed sum cancels them.
- Terms produced by **different expressions** do not cancel. UDL-W1e80 is the example: SP's rotation_i polynomial 0.5b² − (2/3)b³ + 0.25b⁴ at one member's i end, against rotation_j −b³/3 + 0.25b⁴ at the other's j end. The signed sum keeps exactly the defect that the formation introduced.

### 3.2 Per-family formation records

Every ledger push site in S11-F is listed below with its S11-G class. "Exact" means the defect is computed exactly from the declared formula. "Bound" gives a stated worst-case magnitude.

| Site (S11-F candidate) | Family | S11-G class | Defect form | Operand bound β (relative to \|t\|) |
|---|---|---|---|---|
| `push_nodal_loads` PP:2204 | Nodal loads | Input (`push`) | none | 0 |
| `add_constant_effort_support_loads` PP:10739 | DEC-049 constant effort (user-entered force) | Input (`push`) | none | 0 |
| straight `add_uniform_element_loads` PP:8635 (via SP `equivalent_global_nodal_loads_with_spans`) | Straight uniform fixed-end terms, full or spanned | Formed | **Exact**: SP evaluates its own formula a second time over exact expansions. Global = Tᵀ·FEM(T·q_g) on held q_g, L, a, b and T; the constants 1/3 and 2/3 are cleared by scale D = 3 (3·rotation_i = q L²(1.5b² − 2b³ + 0.75b⁴ − …), 3·rotation_j = q L²(−b³ + 0.75b⁴ − …)) | 0; for generated intensities (self-weight before D-14's exact form) γ₄, a same-sign chain: mass sum, ×g-factor, ×g |
| `add_pressure_thrust_loads` PP:8846–8847 | Straight thrust fl(P_ax·x_a) at each end | Formed | **Exact**: rounded product, ε = fl(a·b) − a·b = −lo(a·b) (one FMA) | 0 (`axial_load` held) |
| `add_thermal_equivalent_loads` PP:8924–8925 | Straight thermal and eigen axial fl(N·x_a) | Formed | **Exact**: rounded product, as above | 0 (`axial_load` held) |
| curved thermal `push_product` PP:8948 | K_rc·fl(ε·chord_c) | Formed | **Exact**: scaled rounded product, ε = K_rc·(fl(ε·c) − ε·c) = −K_rc·lo(ε·c) | 0 |
| curved `add_uniform_element_loads` PP:8595 | CB `consistent_uniform_nodal_loads` | Formed | **Bound**: \|ε\| ≤ γ₆₄·‖f_source‖∞ (the source's own 12-vector) | — |
| curved pressure PP:8883–8888 | End caps along the arc tangents, plus CB `consistent_radial_pressure_nodal_loads` | Formed | Caps: exact rounded product. Wall vector: bound γ₆₄·‖f_source‖∞ | — |
| `push_exact_pressure_operands` PP:2285 | D-14 exact-pressure group operands | Formed | **Bound** of half an ulp (2⁻⁵³·\|t\|), if `pressure_runtime` rounds each operand once from its exact value. The implementation confirms this against `pressure_runtime`; if it does not hold, the family bound is that module's own γ count | — |
| `nodal_and_eigen_case_force` PP:2266 (retained replay) | Same producers | Same classes (shared functions) | — | — |

**What the classes mean.**
- **"Stated number of ulp"** (the brief's item 1) is the bound column. For exact families the bound is not needed, because the defect itself is known.
- **Why the curved families use a bound.** Carrying CB's internal arc integrals exactly is F3's work (W1b). For them, S11-G keeps the γ₆₄ bound on the source vector and sums the bounds without sign, which is conservative. No committed case carries a curved-span load (§6.3).
- **Short partial spans** (S11 §10 item 2) are covered by the exact family. A span fraction near 1e-8 gives a real defect, and the guard demotes such a case. That is fail-closed and correct. No committed product-path model carries a partial-span load (§6.3).

### 3.3 Carriage in the ledger (FK `load_ledger.rs`); `AssembledForce` unchanged

```rust
/// How a formed term's binary64 value relates to its declared formula (S11-G).
pub enum Formation {
    /// `scaled_intended` is an exact nonoverlapping expansion of
    /// scale * formula(held operands); scale is 1, 3 or 12.
    Exact { scale: f64, scaled_intended: Vec<f64> },
    /// value = k * fl(a * b) with k exact; defect = -k * lo(a * b), exact.
    RoundedProduct { k: f64, a: f64, b: f64 },
    /// No implemented exact form: |defect| <= bound (rounded upward).
    Bounded { bound: f64 },
}
impl LoadLedger {
    /// A formed term: `value` is pushed exactly as `push` pushes it today.
    pub fn push_formed(&mut self, source: impl Into<String>, dof: usize, value: f64,
                       formation: Formation, operand_bound: f64);
}
impl AssembledForce {
    /// Per DOF with at least one formed term: the exact defect sum (rounded
    /// once), the bound sum (rounded upward) and the intended net.
    pub fn formation_rows(&self) -> Result<Vec<FormationRow>, SumError>;
}
```

**Storage.** Formation records sit in a vector parallel to `terms`: `formations: Vec<Option<Formation>>`, which is `None` for `push` and `push_product`. The following are unchanged:
- `ForceTerm`, `ForceTermKind`, `values`, `by_dof` and `evidence`;
- `finish`'s arithmetic;
- the absence of any public constructor, `Clone` or `&mut`.

So `AssembledForce`'s exactness, and every `Debug` rendering of a term, are byte-identical. A formed term's value is the value pushed today.

**`formation_rows`, per DOF.**
- **Exact terms.** A12 accumulates (12/scale)·(scale·value − scaled_intended) with `ExactAccumulator::add_product`. That is 12·Σε exactly: 12/scale is an integer, 12·value is one product, and the expansion components are binary64.
- **Rounded products.** A1 accumulates −k·lo(a·b) with `add_product`, exactly.
- **The defect.** E = fl(round(A12)/12 + round(A1)). This is a statistic only; its relative error of about 3u is far inside every margin in §6.
- **The bound sum.** B = Σ bound_t + Σ β_t·|value_t|, rounded upward.
- **The intended net.** n_int = fl(net − E).

**Exact expansions.** The multi-factor products (q·L²·bᵏ·T·T) are exact nonoverlapping expansions: repeated scale-expansion with an FMA two-product, the primitive FK's `Expansion` already uses.

**Range failure is fail-closed; never `Err`.**
- If an expansion component overflows or underflows, the term falls back to `Bounded { bound: γ₁₆·Σ|monomials| }`.
- If that bound is itself not finite, the row is treated as firing.
- If `formation_rows` returns `SumError`, the case is also treated as firing.

### 3.4 The statistic, the scale and the threshold

For each loaded row d of a connected body (body membership and L_b as in DESIGN §4.1.6.1), the row **fires** when:

  |E_d| + B_d > 1e-9 · max(|n_int,d|, S\*_d)

**The scale S\*_d uses the DESIGN §4.1.6 coupling on intended ledger nets.**
- **Free rows.** Let F and M be the largest |n_int| over the body's free force rows (global DOF % 6 < 3) and free moment rows. Then S\* = fo = max(F, M/L_b) for a force row, and S\* = mo = max(M, L_b·F) for a moment row.
- **Restrained rows.** These feed reaction rows. They use the same formula over all of the body's loaded rows, free and restrained. That scale is at least the free-row scale.
- **Why free rows use only free rows.** In UDL-W1e80 the fixed-end rows carry 3.3e79. A scale taken over all rows would give a threshold of 3.3e70, above the 2.6e64 defect, and the formation row would escape. Mutation M5 (§8.2) checks this.

**Degenerate scale.** If every free row's intended net in the body is exactly zero, S\*_f = 0 and any nonzero free-row defect fires. That is fail-closed. No frozen or committed case is in this state with a free formed row: the 14 committed entries with S\*_f = 0 have no free formed row.

**Precision.** The comparison is binary64. Bounds are rounded upward, and n_int and S\* are rounded to nearest. Nothing here is bit-critical: every forecast row is at least 25× from the threshold on either side.

### 3.5 What firing does: demote to Sensitive, never `Err`, no new field

1. PP computes `formation = formation_guard(&force, …)` once per case, right after `finish_case_ledger` (PP:2490). The new module `product_physics/src/formation_guard.rs` returns `Option<FormationFinding>`: the first firing rows, their DOF, E, B, n_int, S\* and source ids.
2. **Routing is unchanged, except for retained-source recovery.**
   - `source_eligible` gains `&& formation.is_none()`.
   - Source recovery solves the same `AssembledForce`, so it would reproduce the formation defect. If it were selected, it would restore `numerically_eligible` (P1: every Sensitive case with `recovery_selected` has that standing).
   - The guard therefore never triggers recovery, and never lets recovery re-qualify a fired case.
   - The load-state diagnostic then reports `retained_source_attempt=not_eligible`, which is existing text.
3. **The verdict.** `append_integrity_report` gains `formation: Option<&FormationFinding>`. When it is `Some`:
   - the code becomes `NUMERICAL_INTEGRITY_SENSITIVE` at severity `warning`, whatever `report.quality` says;
   - one sentence is appended to the existing message: "Load formation: ledger row(s) … formed-term defect E (+ bound B) exceeds 1e-9 × max(|intended net|, S\*) …; source ids …". The `StructuralReport` is not altered and its `Debug` text stays truthful.
   - Both call sites change, the linear route (PP:2790) and the nonlinear route (PP:2830).
   - The `RECOVERY_BASIS_UNQUALIFIED` branches already map to Unresolved, which ranks above Sensitive, so they are left alone.
4. **The effect.** `assessed_numerical_quality` maps the case to `NumericalQualityStatus::Sensitive`, so the case standing is `needs_recompute` (P1's standing rule). Envelope-level Current is withheld and rows are kept, exactly as S11 §6 describes for a flagged loss. Nothing is refused, no `Err` is returned, and no new envelope field, code or enum value is added.

**Granularity is the load case.** Every row of a fired case is demoted, including rows that pass on their own: the 40 passing rows of each UDL-W1e8 case-mode, and every row of UDL-W1e80. A per-row verdict would need a new envelope field (brief item 3). This note does not propose one. If ROOT wants row granularity, that is a separate ruling.

---

## 4. The recovery-side guard: analysis and decision

**Scope examined.** Member end bending actions from the formed K_e·u (the 8 INPLANE rows are all end moments). Station actions derive from the same end actions (S11 §2.2 E4 and E6), so they would inherit any end-action guard.

**Candidates.** Each was evaluated on P1's 146 passing case-modes and on the INPLANE rows in the S11-F state. B = γ₁₆·Σ|K_loc||T u|; S\*_moment is the body's coupled moment scale.

| Candidate | Rule | INPLANE rows caught | Passing case-modes demoted | Cost per member end |
|---|---|---|---|---|
| R-a (sound bound, body scale) | B > 1e-9·max(\|q\|, S\*_moment) | **0 of 8** | 4 (8 rows: RF-SKEW-T-CANT-OFF-345, RF-ZERO-TORSION) | 24 multiply-adds |
| R-b (sound bound, row-relative) | B > 1e-9·\|q\| and \|q\| > 2¹⁰·B | 8 of 8 | **16** (28 rows: RF-INVARIANCE-LFRAME ×4, RF-LARGE-CONT-n00100 ×2, RF-WEAK-W-3D and -AX rho1e-08) | 24 multiply-adds |
| R-c (actual formation error by exact recomputation, row-relative above the floor 2⁻³⁴·S\*) | \|q − q_exact\| > 1e-9·max(\|q\|, 2⁻³⁴·S\*) | 8 of 8 | **90** (152 rows in 46 cases) | 24 exact products |

**The underlying fact.** Recomputing K_e·u exactly from the binary64 solution leaves the INPLANE rows at 145, 1291, 2949 and 1175× the criterion. The error sits in δu, the solve's ordinary binary64 accuracy: |K|·|δu| is about 1e-14 against a 1e-8 net.

**Why every catching candidate fails.**
- A guard that catches these rows must judge them relative to their own magnitude.
- The frozen references use that scale only for RF-CANCEL, R1's net-governed scale. Every other passing row is judged on its class scale.
- In binary64, the rows R-b and R-c flag carry the same row-relative noise as the INPLANE rows. The product cannot tell which scale a reference chose.
- Under the product's own contract (DESIGN §4.1.6, the body scale S\*), the INPLANE rows are well inside 1e-9·S\*. The contract is met there only at precision p, by W1.

**Decision.** S11-G ships no recovery-side demotion. The 4 INPLANE triples (8 rows) wait for F2 (W1a), with the justification in §5 going to ROOT. A row-relative recovery guard in binary64 would demote at least 16 passing frozen-reference case-modes, which the brief forbids.

---

## 5. Per-row justification for ROOT: the INPLANE rows wait for F2 (W1a)

**Common facts** for both cases: 2 m cantilever on x, members M1 (N0→N1) and M2 (N1→N2), N0 anchored.
- **Loads.** The ordinary load is at N1: 50 N in y (F case) or 20 N·m about z (M case). The authored (1e80, 1e-8, −1e80) at N2 is repaired to its exact net 1e-8 by S11-F; this is input cancellation and correct.
- **The error mechanism.** The expected end moments are net-governed (scale 1e-8). The binary64 solution carries the N1 load's rigid motion of M2. Its relative error δu/u of about 1e-16 becomes |K_e|·|δu|, about 1e-14 N·m, in M2's end actions and in M1's j-end action.
- **What does not help.** The load-row guard is silent because every N2 term is an input, as it must be for the 221. Exact recovery does not clear the rows (§4). A row-relative guard demotes 16 to 90 passing case-modes (§4).
- **What repairs them.** F2's W1a: recovery at precision p from u at precision p, under the stop rule |q_p − q_2p| ≤ 2⁻⁶⁴·S\* (DESIGN §4.1.6).
  - F case: S\*_moment = max(50, L_b·50) = 100 N·m, so the accepted error is at most 2⁻⁶⁴·100 ≈ 5.4e-18 N·m, below the 1e-17 criterion. q = 1e-8 is above the floor 2⁻³⁴·S\* ≈ 5.8e-9, so the row is `relative_verified`.
  - M case: S\*_moment is about 20 N·m, the error is at most about 1.1e-18, and the floor is about 1.2e-9.
- **Current standing.** Pre-existing on main (P1 baseline), and made no worse by S11-F (ROOT's ruling).

| Row (typed, both modes) | Expected | Emulated after S11-F | Ratio | Exactly rounded recovery from binary64 u | R-a | Why it waits |
|---|---|---|---|---|---|---|
| F-G1e80-GnG-INPLANE Mb.M1.j | 1e-8 N·m | 9.9999937e-9 | 628 | 145 | silent | δu from the N1 load's rigid motion; no binary64 guard separates it from passing rows; F2 repairs it at 5.4e-18 |
| F-G1e80-GnG-INPLANE Mb.M2.i | 1e-8 N·m | 1.0000036e-8 | 3635 | 1291 | silent | Same |
| M-G1e80-GnG-INPLANE Mb.M2.i | 1e-8 N·m | 1.0000015e-8 | 1504 | 2949 | silent | Same; F2 repairs it at about 1.1e-18 |
| M-G1e80-GnG-INPLANE Mb.M2.j | 1e-8 N·m | 9.9999937e-9 | 628 | 1175 | silent | Same |

**Until F2 lands:** these 8 rows stay on the formation list. The pin owners are already "S11-G recovery guard, then F2 (W1a)". This note asks ROOT to re-point them to "F2 (W1a)" alone. The gate keeps failing on any triple outside both lists.

---

## 6. Forecast (script `s11g_forecast.py`; output `s11g_forecast.json` and `.stdout.json`)

### 6.1 Frozen references: the formed-load cases (Part A)

Only three R1 cases carry element loads: RF-CANCEL-UDL-W1e5, -W1e8 and -W1e80. All other R1 cases have nodal inputs only, so their defects are identically zero and the guard cannot fire on them. That covers every RF-CANCEL nodal case, every RF-SKEW case and the 221.

Free row S1.RZ (S1's RX and RY are free and unloaded):

| Case | Net of represented terms | Intended net | Exact defect | Threshold | Statistic / threshold | Fires | A-priori c = 16 bound / threshold |
|---|---|---|---|---|---|---|---|
| UDL-W1e5 (passes today) | 0.49166666668606923 | 59/120 | 1.94e-11 | 4.92e-10 | **0.0395** | no | 2.89 (a false demotion) |
| UDL-W1e8 | **0.4916666902601719** | 59/120 | 2.36e-8 | 4.92e-10 | **47.99** | **yes** | 2890 |
| UDL-W1e80 | 2.6328e64 | 1e-8 | 2.63e64 | 1e-17 | **2.6e81** | **yes** | 1.4e83 |

**Restrained rows** (S0.UY, S0.RZ, S1.UY, S2.UY, S2.RZ) in all three cases fire in none. Their defects are 0 (the transverse terms are exact) or up to 1.7e64 against 3.3e70 for W1e80, and 1.4e-8 against 0.033 for W1e8. That holds even against the smaller free-row scale the script uses.

**Probe A (Part A2).** S11-F's committed test, one member carrying (G, 0.3, −G) and (0.3, G, −G) at G = 1e8 and 1e80, asserts `CHECKS_PASSED`.
- The exact defects at the tip are 0 (UY) and 1.85e-17 (RZ, the 0.3 term's own defect). The G terms cancel together with their defects. Against thresholds of 3e-10 and 6e-10, the margin is at least 3e7.
- The a-priori bound fires on every row. This is the second reason §1 gives for using exact defects.

**Coverage by row.**
- UDL-W1e8 th.S1.RZ: captured × 2 modes, typed × 2 modes (4 rows).
- UDL-W1e80 th.S1.RZ: typed × 2 modes (2 rows). Capture still refuses |x| ≥ 2⁵³ until S-H lands.
- **6 of 6 load-formation rows caught.** The 8 INPLANE rows are covered by §5.

### 6.2 The 221, RF-CANCEL and RF-SKEW

- **The 221 triples** (87 in 12 cases captured, 134 in 18 cases typed) are all nodal-input cancellation. Every term there is an input, E = B = 0, and the statistic is 0. **None is demoted.** This follows by construction, and mutation M3 (§8.2) pins it.
- **The other RF-CANCEL cases** are nodal only (statistic 0), plus UDL-W1e5 at 0.0395.
- **RF-SKEW and every other frozen family** are nodal only (statistic 0).
- **The recovery side** is not shipped (§4), so no RF-SKEW row is touched by it.

### 6.3 Committed models (Part B)

**Scope.** Every committed JSON under `P` was scanned, except `execution/`, `schemas/`, `node_modules/` and `target/`. The scan found:
- **164 model/case entries in 98 files** that carry formed loads. They cover fixtures, test fixtures, validation qualification inputs, the witness input and result-export fixtures.
- **622 loaded rows** carrying formed terms: 399 free, 223 restrained.
- **18 entries** where formed terms meet each other or an input at a free node:
  - physics_thermal_ui (tip);
  - invented_preview_model and its copies in test fixtures, result-export fixtures and the witness input (N-110, N-120, N-130, N-140);
  - connected cold and hot (middle);
  - eigen_motion and the result-export fallback request (tip);
  - self-weight wasm (tip);
  - two contract-corpus cases (N-2).

**The magnitude screen replaces each exact defect with the uniform bound 17·γ₁₆·Σ|formed term|.** That bound exceeds every family's exact defect: SP's worst amplification is 17 and γ₁₆ covers the formation chain. It also exceeds the curved and pressure family bounds.

**Result:**
- The **minimum margin is 1577** (connected, the middle node's UX row). Two identical free-length thermal terms meet there, with the screen's strains overestimated at 1e-2. With signed exact defects these two terms cancel exactly.
- **Every other entry has margin ≥ 33115**, which equals 1e-9/(17·γ₁₆), the single-term case.
- There are no screen errors.
- **No committed case fires.**

**Families covered.** The scan found 90 distributed and 11 weight element loads, 17 element pressure loads, 6 element thermal loads, pressure regions, and 82 free-length, 4 explicit-strain and 4 constant-alpha element states, plus fits.
- **No curved-bend span carries a load.** Bends in these models are node-marker components.
- **No product-path model has a partial-span load.** The only `start_fraction` is in a domain model, not a `PreviewModel`.
- **The numerical-integrity benchmark fixtures** (`FX-*`) carry only `tip_FY_N` nodal loads.

### 6.4 Committed-byte forecast: none

- **Ledger.** Values, terms, `ForceTerm`'s `Debug`, `by_dof`, evidence and `finish` arithmetic are unchanged (§3.3).
- **SP.** The formation variant returns values bit-identical to `equivalent_global_nodal_loads_with_spans`; test T7 pins this. PP pushes the same values it pushes today.
- **Diagnostics and envelope.** A code and message change only when the guard fires, and §6.1–6.3 forecast that it fires on no committed case. `source_eligible` changes only for fired cases.
- **Therefore no committed envelope, raw, golden or hash pin changes.** That includes the `load_reference_contract.rs` and `test_load_reference_readers.py` FROZEN hashes and the `n05-*` raws.
- **The S11-G PR's regeneration must show zero committed-byte diff.** Any diff is a stop and goes to the manager.
- **What does change** is frozen-reference harness output, which is not committed product bytes. UDL-W1e8 (both entries) and UDL-W1e80 (typed) move from Passed to Sensitive, with standing `needs_recompute`.

---

## 7. Write set (S11-G slice, on main after S11-F)

| File | Change |
|---|---|
| `FK/load_ledger.rs` | `Formation`, `push_formed`, parallel `formations`, `AssembledForce::formation_rows`, `FormationRow`; unit tests |
| `FK/exact_sum.rs` (only if needed) | No change planned. `add_product` covers every accumulation |
| `FK/lib.rs` | Re-export only |
| `straight_pipe/src/lib.rs` (SP) | `equivalent_global_nodal_loads_with_spans_formed`, returning today's values plus `Formation::Exact` per slot. The existing function is unchanged |
| `product_physics/src/formation_guard.rs` (new) | Bodies, S\*, the threshold, `FormationFinding`, the reason sentence |
| `product_physics/src/lib.rs` (PP) | Push sites PP:8595, 8635, 8846–8847, 8883–8888, 8924–8925, 8948 (formed) and 2285 (bound); the guard call after PP:2490; `source_eligible`; `append_integrity_report` and its two call sites |
| `product_physics/src/s11f_tests.rs` or a new `s11g_tests.rs` | Tests T1–T6 and T10 (§8) |
| `product_physics/tests/s11f_site_test.rs` | Site table extended with the formed/input/product class for every push (T8) |
| `T3/GATE/pin_s11_exceptions.py` and `FORMATION_EXCEPTIONS.json` | Not in S11-G's product PR. ROOT decides whether the 6 UDL rows leave the list and whether the 8 INPLANE owners are re-pointed (§5) |

No change is made to SA, FK `structural.rs` (`finish_checked_factor`), `solve_preview_reduced_system`, CB, `pressure_runtime` (read only, to confirm §3.2's half-ulp claim) or any library or code-rule data.

---

## 8. Tests and mutations

Every test that pins a verdict carries a **paths-differ precondition** (RV1's lesson): it first shows that the guarded and unguarded paths would publish different verdicts, or that the case contains the arithmetic the guard must see.

### 8.1 Behavioural tests

| ID | Test | Precondition (paths differ) | Pin |
|---|---|---|---|
| T1 | UDL-W1e8 through R1's frozen request, captured and typed, both modes | The ordinary `StructuralReport.quality` is `Passed`. The ledger's S1.RZ exact defect is nonzero and the statistic exceeds the threshold (so without the guard the code would be `CHECKS_PASSED`) | Code `NUMERICAL_INTEGRITY_SENSITIVE`; case quality `sensitive`; standing `needs_recompute`; the reason sentence names S1.RZ and both loads |
| T2 | UDL-W1e80, typed, both modes (captured asserted to refuse) | As T1. The represented net is 2.63e64 against the intended 1e-8 | As T1 |
| T3 | UDL-W1e5, captured and typed, both modes | The S1.RZ exact defect is **nonzero** (1.94e-11), so the guard sees a defect and must weigh it | `CHECKS_PASSED`; statistic/threshold < 0.1 |
| T4 | Nodal (G, n, −G) at a free DOF: RF-CANCEL-F-G1e80-GnG, typed, and F-G1e8-GnG, both entries | The binary64 fold differs from the exact net (S11-F's precondition); `formation_rows()` is empty (every term is an input) | Not demoted by S11-G (the code is whatever S11-F gives today) |
| T5 | Probe A: S11-F's F2 test, (G, 0.3, −G) and (0.3, G, −G) at 1e8 and 1e80 | The G terms' individual defects are **nonzero** (checked on the ledger), while their signed sum is 0 | `CHECKS_PASSED` (S11-F's assertion is kept) |
| T6 | A synthetic collinear skew pair (two members with the same direction vector, equal thermal `axial_load`), free middle node, no other free load | fl(N·x) is inexact, so each end term's defect is nonzero | `CHECKS_PASSED` (signed cancellation) |
| T7 | SP formation variant against the existing function: full-span and spanned loads, several orientations | — | Values bit-identical; the intended expansion equals a rational oracle (hand-derived: 3·rotation_i at b = 1, a = 0 is qL²/4) |
| T8 | Site table: every `ledger.push*` in PP is classified | — | A formed site using plain `push` fails the test |
| T9 | Committed regeneration | — | Zero committed-byte diff (the PR precondition, §6.4) |
| T10 | Source recovery is not selected for a fired case: UDL-W1e8 captured | The case is `source_eligible` apart from the guard (the capture exists, no nonlinear support, no combination) | No `SOURCE_BLOCK_RECOVERY_SELECTED`; standing `needs_recompute` |
| T11 | I4's F1/F11/F12 test extended | — | Every breach on the formation list is either published non-Passed (the 6 UDL rows) or on ROOT's F2-waiting list (the 8 INPLANE rows) |

### 8.2 Mutations (each must be killed)

| ID | Mutation | Killed by |
|---|---|---|
| M1 | Drop a formed term's defect, so SP's uniform `Formation` reports scaled_intended = scale·value | T1, T2 (not demoted) |
| M2 | Tag a formed term as input (`push` instead of `push_formed` at PP:8635) | T1, T2, T8 |
| M3 | Apply the guard to input terms, giving `push` terms a β = u bound | T4 (the statistic becomes u·2e80 ≫ 1e-17, so the 221 class is demoted) |
| M4 | Replace SP's exact defect with the a-priori c = 16 bound | T3 (UDL-W1e5 demoted), T5 |
| M5 | Compute S\* for free rows over all rows | T1, T2 (thresholds 0.033 and 3.3e70; not demoted) |
| M6 | Sum \|ε\| instead of Σε | T5, T6 |
| M7 | Drop the `source_eligible` gate | T10 |
| M8 | Demote through `report.quality` before routing (so recovery is triggered) | T10 |
| M9 | Map a fired case to `Err` or a blocking diagnostic | T1 (standing and code), plus the existing no-refusal tests |

---

## 9. PP conflict boundary with K-D5

| | K-D5 (I3) | S11-G |
|---|---|---|
| PP | Only the solve call inside `solve_preview_reduced_system` (PP:3965 in I3's tree, where `assembly.solve(...)` becomes `solve_with_formation_check(...)`) | Push sites in `add_uniform_element_loads`, `add_pressure_thrust_loads`, `add_curved_bend_pressure_thrust_load`, `add_thermal_equivalent_loads`, `add_curved_bend_thermal_equivalent_load`, `push_exact_pressure_operands`; one call after `finish_case_ledger`; the `source_eligible` line; `append_integrity_report` and its two calls |
| FK | `structural.rs` (`finish_checked_factor`), the new `formation_check.rs`, `retained/*` | `load_ledger.rs` only |
| SA | `structural_adapter.rs` | none |
| SP | none | The new formation variant (existing function unchanged) |

**No function is edited by both slices,** so a textual merge is clean in either order.

**Semantically, the two compose.**
- K-D5's D-5 check can set `SolveQuality::Sensitive` inside the solve. That triggers the existing source-recovery path, unless S11-G's guard has fired, in which case `source_eligible` is false.
- S11-G's demotion is applied at the verdict, and demoting an already-Sensitive case is idempotent.
- Standing is `needs_recompute` whichever lands first.

**The merge-forward note for whichever lands second.**
- K-D5 on S11-F passes `&force` (`AssembledForce`) where it passes `global_force` today. S11-G does not touch that argument.
- A later option is to let K-D5's ρ use f_intended = f − E, so that EF sees load formation. It is not proposed here; it would change K-D5's calibration.

---

## 10. Run records (`_run_records/`)

| File | Content |
|---|---|
| `s11g_forecast.py` | Standard library, deterministic (checked under two `PYTHONHASHSEED` values). Run from T3/: `PYTHONDONTWRITEBYTECODE=1 nice -n 19 python3 DESIGN_NUMERICS/_run_records/s11g_forecast.py <P> DESIGN_NUMERICS/_run_records/s11g_forecast.json`, with `<P>` = `projects/chirality-piping` (given relatively as `../../../../../../..`). Parts: A (R1 formed-load cases, every loaded row), A2 (S11-F probe A), B (committed models: topology and magnitude screen), C (recovery candidates R-a, R-b, R-c, and INPLANE in the S11-F state) |
| `s11g_forecast.json` | Full output |
| `s11g_forecast.stdout.json` | Summary printed by the script |

The script imports `withheld_rows.py` (the support-family rule) and `sweep_d5_r1.py` (the `num` parser) from `_run_records/`. It is Python 3.11.15, as in `python_version.txt`.

---

## 11. Open items

1. **Cross-check against I4's records.** When I4's `formation_rows/` records are committed on the S11-F branch, re-run the §2 and §5 INPLANE columns against the published post-S11-F values. The UDL rows need no re-run: the net 0.4916666902601719 already matches ROOT's figure bit for bit.
2. **The half-ulp claim for exact-pressure operands** (§3.2) is confirmed against `pressure_runtime` during implementation. If it fails, that family takes `pressure_runtime`'s γ count. The committed screen already charges every formed term 17·γ₁₆ (about 272u) and still leaves every committed row at least 1577× below its threshold, so any plausible γ count for that family changes no forecast.
3. **ROOT decisions requested:**
   - (a) accept the exact-defect form in place of the brief's literal a-priori bound (evidence in §6.1: UDL-W1e5 and probe A);
   - (b) the per-row F2 justification for the 8 INPLANE rows (§5), and re-pointing their owners;
   - (c) case-level granularity of the demotion (§3.5).
