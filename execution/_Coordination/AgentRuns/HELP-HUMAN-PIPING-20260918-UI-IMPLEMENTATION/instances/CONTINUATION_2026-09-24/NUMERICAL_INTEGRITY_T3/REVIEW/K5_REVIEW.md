# RV14: independent full-diff review of slice K5

**Verdict: PASS.** There are no BLOCKING findings. There are 4 SHOULD-FIX findings and 5 NOTEs.
- **The code holds.** I found no false witness, no wrong or non-canonical direction, and no missed mechanism that ends `Restrained`. My checks were:
  - 4,226 cases against my own exact-rational oracle of the unreduced stacked map, plus 700 huge-coordinate cases;
  - an SA probe comparing main with the head;
  - PP runs from archives.
- **Byte identity holds.** Frame-only bodies, the unselected entries and the nonlinear loop behave as on main, and the contact-seed guard stays closed.
- **The Q2 condition holds.** I checked RETURN §2.3 step by step, and no matched-slot counterexample survives the code as written.
- **P6 and the prefilter equivalence hold.**
- **The three test-gap findings.** Three of my own mutants survive the whole K5 test set, and none is equivalent:
  - RV14-1: the exact cycle-tie check, the only guard against a false witness from a cycle in the τ_B band;
  - RV14-2: the curved-to-curved coordinate agreement that RETURN §2.3 Step 2 relies on;
  - RV14-3: the K5-C2 mapping, a published change class that no test pins.
- **The fourth.** RV14-4: a public FK field publishes +∞ at tiny coordinates.
- **Reach.** None of the four is reachable from the product today. RV14-3's mutant would, however, silently remove a change the product publishes.

## Reviewer, brief and delegation

- **Reviewer.** RV14 is a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN). It ran as a background subagent of ROOT's session, started through the Agent tool, and ROOT is its only return path.
  - It was paused once on ROOT's order (low battery) with no process running, then resumed on ROOT's message. Nothing had moved on the candidate in between.
  - I did not design, implement or test K5. I made no Git writes and delegated nothing.
  - My writes are this file and `REVIEW/_run_records/k5_review/**`, left uncommitted.
- **Read:**
  - Root `AGENTS.md`, `agents/AGENT_TASK.md`, `TASK_BRIEFS/_COMMON.md`, and `I8R_K1_RESUME.md`'s "The Mac host" and "Platform calibration";
  - `DESIGN.md` revision 5a.2 (`fb62ef4a…`): §4.9, §4.1.3, §4.3.1, §4.10's RF-MECH, the K5 row of §6, and §7.3;
  - `R5_4_CURVED.md` §2 and §5;
  - `TASK_BRIEFS/I14_K5_IMPLEMENTATION.md`, with its ROOT rulings;
  - `ROOT_RULINGS_V1.md`, "K5: spawn and rulings" and "K5: rulings on I14's checkpoint-0 plan";
  - the candidate's `IMPLEMENTATION/K5/`: CHANGE_RECORD, RETURN and the `_run_records/` I cite;
  - the complete diff, commit by commit, and the code it relies on: FK `structural.rs` (`Expansion`, `exact_radix`, `gamma`), FK `lib.rs` (the user element, orientation), PP's linear route and curved-macro builder, and NI's contact-seed path.
- **Placeholders.**
  - `<wt>` is the T3 worktrees root. My scratch is `<wt>/scratch/rv14`, and my targets are under `<wt>/rv14-target`.
  - `FK`, `SA`, `NI`, `PP` and `CB` are as in K5's RETURN.
  - Line numbers are at the head `b379e5b27` unless stated.
- **Host.** Everything I built came from `git archive` copies of `24dea2dae` and `b379e5b27`.
  - I ran one cargo job at a time, at `-j 4` with `RUST_TEST_THREADS=2`, using rustc 1.97.1 with `--offline --locked`.
  - `<wt>/k5` was used read-only, for GEN-8 only.
  - No dense matrix was formed at 10,000 or more members. The largest SA or PP model had 6 nodes, and the largest FK case 400 nodes.
  - The memory guard log was unchanged throughout: 2 start lines, sha256 `79e2ce8e…` (`memguard.txt`).

## Revisions reviewed (`revisions.txt`)

- **PR #1044** is open and mergeable, on `codex/piping-k5-20260928`, at head `b379e5b27`.
  - I verified the head after a `git fetch`, before and after the pause.
  - Hosted checks: 12 SUCCESS and 4 SKIPPED, as ROOT reported.
- **The base** is main `24dea2dae`, which is the merge base.
  - Main has since moved to `df6d59e3c`. Its delta touches only `projects/chirality-app-v4/`, and nothing under `projects/chirality-piping/` (RV14-N5).
- **The commits.** The slice is `0c732061b` (A1), `6bf64f5a9` (A2), `416b0d456` (B, records), `f89662e1f` (C) and `b379e5b27` (D, records).
  - After A2, the code changes are tests only: 5 files, +47 −2.
- **The totals.** 199 files change, +28,091 −5. In `core/`, 11 files change, +5,654 −5.
  - The only deleted lines are SA's `rigid_body` import and the four `geometry(prescribed)` calls in the selected bodies.

## Findings

| ID | Severity | Site | Evidence | Resolution |
|---|---|---|---|---|
| RV14-1 | SHOULD-FIX | **FK** `WitnessContext::null_translations`, the exact tie loop (`rigid_body.rs:927-932`). It is a test gap, not a code defect. | **Construction** (`oracle/cases_all.txt`, `T1_cycle_band_0..4`; minimal case `B_cycle_band_min` in `cases_rv14_4.txt`):<br>– 4 nodes (0,0,0), (1,0,0), (0,1,0), (1−ε, 1, −1); sub-bodies {0,2} and {1,3};<br>– ties 0-1 (the tree tie) and 2-3 (a cycle, with exact offset c = (ε, 0, 1));<br>– grounds d0 to d4 (node 0's translations, RX and RY); ε from 3e-16 to 1e-13.<br>**Behaviour:**<br>– The oracle gives exact nullity 0: the cycle restrains the rotation about z.<br>– σ_min ≈ 7e-16 against τ_B ≈ 9.8e-14, so the case is in the band.<br>– The head is correct: `NumericallyUnresolved`.<br>**Mutant RV14-M1** deletes only the exact tie loop and keeps the ground checks:<br>– it survives FK's k5 tests (8 + 14 + 1);<br>– it publishes `MechanismWitnessed` with θ = e_z on all five T1 cases and on `M0809`. The oracle refutes all six ("tie violated"; `mutations/logs/RV14-M1/oracle_report.txt`).<br>**Why nothing catches it.** In the band, the violated cycle row has \|θ×ĉ\| ≈ ε, which passes the 2^-20 prefilter, so the exact tie loop is the only guard. B1 excludes the τ_B band by construction, and B4's cycle cases are decided by the screen before any candidate.<br>**Reach:**<br>– FK API: yes.<br>– SA-built evidence: yes, through SA's public entries with two positive-stiffness user elements (`UserStiffnessElement::new` accepts them) between two frame sub-bodies.<br>– Product: no. PP realizes no user element (a nonzero lateral is refused at `PP:1740`, and a zero lateral is no tie), and curved elements never form ties. | Add `T1_cycle_band_0` (expectation U) to `cases.txt` or `k5_b4_cycles`. Show RV14-M1 killed. |
| RV14-2 | SHOULD-FIX | **SA** `W4Context::body`, `evidence.coordinates[global].or(curved_points[l])` (`SA:1460`): agreement between two curved slots at a node that only curved slots touch. It is a test gap, not a code defect. | **Construction** (probe P1, `probes/rv14_sa_mutprobe.rs.txt`):<br>– two matched bends: A, node 0 (0,0,0) → node 1 (0.25, 0.25, 0), centre (0, 0.25, 0); B, node 1 (0.5, 0.25, 0) → node 2 (0.75, 0.5, 0), centre (0.5, 0.5, 0);<br>– their macro sources disagree on node 1, which no frame or user touches;<br>– translation pins at nodes 0 and 2.<br>**Head and main** (byte-identical): the body is `CurvedCoordinates`, and the matrix gate refuses it (pivot unresolved).<br>**Mutant RV14-M2** drops `.or(curved_points[l])`:<br>– it survives NI's library (127);<br>– it publishes `Mechanism` with θ = (3, 2, 0) and u(1) = (0, 0, −0.25), in every selected branch and both modes.<br>– That is not a zero-energy motion of A's intended element: u₁ − u₀ ≠ θ × (x₁ᴬ − x₀). K's actual null motion is θ ∥ (1, 1, 0).<br>**Why it matters.** RETURN §2.3 Step 2 relies on this check ("or from another matched macro's at the same node"). K5's only coordinate test is the frame-against-curved case.<br>**Reach:**<br>– SA's public entries with caller-supplied `curved_sources`: yes.<br>– Product: no. PP builds every macro element from the built node list (`build_curved_bend_macro_elements`, `nodes[from_index]`), one coordinate per node. | Add P1 as an SA test expecting `W4Unqualified::CurvedCoordinates { node: 1 }` and the unselected outcome. Show RV14-M2 killed. |
| RV14-3 | SHOULD-FIX | **SA** `W4Context::screen`, the arm mapping any other status to `NumericallyUnresolved { reason: "constrained-body rank unresolved" }` (`SA:1406-1409`; the arm starts at `:1406`): the K5-C2 mapping, which no test pins. | **Mutant RV14-M4** maps that arm to `Ok(())`, so the body proceeds to the matrix gate:<br>– it survives NI's library (127) and PP's `k5_curved_mechanism_runtime` (3);<br>– no test under `core/` names the reason string.<br>**Construction:** PP's `constructed_mechanism_r0.2_o0`, I14's product input, re-run through my PP harness (`mutations/logs/RV14-M4.ppprobe/`). It is an exact mechanism whose canonical witness is not representable (0.2 − 1.2 needs 54 bits).<br>– Under the mutant, all four entry × mode runs return Mac main's envelopes.<br>– The head's envelopes differ (sha256 `682476dd…` and `90fa0a19…` against main's `3019362d…`, `0978222a…`, `c9a5257a…` and `0da67104…`).<br>**Reach: product, yes.** K5-C2 occurs in the product: 8 runs in I14's table, and 4 in my re-run. The code is correct, but a pre-registered published class could be removed with no test failing. | Pin K5-C2: an SA test on the R = 0.2 geometry, and/or a PP test that asserts the reason, on both entries and in both modes. Show RV14-M4 killed. |
| RV14-4 | SHOULD-FIX | **FK** `WitnessContext::publish` (`rigid_body.rs:985-992`): `rigid_parameters = [r₀/L, r₁/L, r₂/L, r₃, r₄, r₅]` is formed after verification, with no exactness or finiteness check. | **Construction** (`H_tiny_free_x` in `cases_rv14_4.txt`):<br>– nodes (0,0,0) and (2^-1070, 0, 0), one sub-body, grounds d1 to d5;<br>– head: `MechanismWitnessed` with exact `node_motion` (1, 0, 0) at both nodes, but L = 2^-1070 and **`rigid_parameters` = [+∞, 0, 0, 0, 0, 0]**.<br>– On my 1,500-case subnormal corpus (`S*`), **349 witnesses publish a +∞ component** (`oracle/report_all.txt`). Their node motions are exact and canonical.<br>– At huge L the field stayed exact on every case: 0 mismatches in 700 cases with coordinates of 1e16 to 1e200, near 2^1000, and shifted by 2^1020.<br>**Contract.** RETURN §9.1 says `MechanismWitnessed` "carries the exact, canonical witness", and names the field `[t/L, θ]` of the witness. `assess_rigid_body`'s field is the verified candidate itself.<br>**Reach:**<br>– FK API only (K4's directional follow-up, F3, W1c, V-K).<br>– SA reads only `node_motion`; no non-test code reads `rigid_parameters`.<br>– SA-built bodies have element spans above 1e-12 (`AXIS_TOLERANCE` for frames and users; a tiny bend has no finite matrix and stays unmatched), so L is at least about 2^-40.<br>– Product: no. | Publish the field only when every r_i/L is finite and exact; otherwise give `None`, or publish [t, θ]. Add the tiny-L case to B5. |
| RV14-N1 | NOTE | RETURN §6, §2.5, §10 | **§6's NI kill sites** use the round-1 tree, before C inserted 32 lines in `k5_tests.rs`. K5-M13's `:1065` is `:1097` at the head, and the table does not say which tree it uses (K5-M9 and M9b are head numbers).<br>**§2.5** cites `SA:1900`, the base line, for `permits_contact_seed_trial`; §10 cites `SA:2142`, the head line. | State the tree for each line number, or renumber to the head. |
| RV14-N2 | NOTE | SA spring grounds in `W4Context::body` (`SA:1487-1492`) | **Mutant RV14-M3** drops the spring grounds from W4:<br>– it is killed only by `Result::unwrap()` panics in K-D5's and K1's test helpers (`kd5_tests.rs:188` ×4, `k1_tests.rs:349`, `:1364`), never at a K5 assertion;<br>– my probe P2 (K5's curved line held about a–d by an RX spring of 1e6) gives a false `Mechanism` under the mutant, while the head publishes as main. | Optional: add P2 to `k5_tests.rs` as an assertion. |
| RV14-N3 | NOTE | FK candidate completeness | **`M0438`** has exact nullity 1, a small-integer null motion in (t, θ), and a representable canonical witness, yet it ends `NumericallyUnresolved`.<br>– σ = 2.46e-19 (the exact null), but σ₂ = 2.17e-11 lies just above τ_B = 3.46e-13, which spoils the SVD vector beyond the 2^-24 snap tolerance.<br>– The contract permits this, and it is conservative: the product would see a K5-C2 unresolved, not a publication.<br>– It is 1 of 3,377 exactly deficient cases. B1's "every small-integer null motion must be witnessed" holds only on B1's band-free corpus. | None required. |
| RV14-N4 | NOTE | RETURN.md:12 | The delegation line names the host ("a Claude Code background subagent"). It is not a model identifier. The path, user-name and model scans are otherwise clean. | None. |
| RV14-N5 | NOTE | Merge basis | Main moved from `24dea2dae` to `df6d59e3c` (PR #1043, app-v4 documents only). The piping tree is identical, so every comparison here stands for the merge. | None. |

## 1. Scope and byte identity

**`assess_rigid_body` and `original_rigid_witness` are byte-identical.**
- `rigid_body.rs:1-248` hashes `d9598efa…` on base and head.
- The file's diff deletes nothing. It is one insertion at line 250, and the old test module follows unchanged.
- Q4(b) keeps the frame screen untouched, so its `hypot` at `:52` and `:108` is unchanged. It is on the T3-close list.

**W4 runs in exactly the four selected branches.** I derived this by reading.
- `BodyEvidence` gains `w4: Option<W4Context>`. The public `AssemblyEvidence::geometry` and `SparseAssemblyEvidence::geometry` pass `None`, and so every unselected path gets `None`:
  - `solve`, `solve_binary64`, `solve_assembled` and `solve_force_scaled`;
  - the `if !selected` early returns, which precede any W4 call in all four formation-checked entries (dense `SA:246-250` and `:426-435`, sparse `:698-703` and `:879-892`).
- With `None`, the only new branch (`if !qualified { if let Some(w4) … }`) is inert. The seed-order search, the qualification test, the coordinates, the ground order and the `assess_rigid_body` call are today's lines.
- With `Some`, W4 runs only for a body with a non-objective edge. A frame-only body keeps today's path, and a node with no edge is skipped as today.
- `force_scaled_outcome` passes `Mechanism` through unscaled (`unscale_structural_error` rewrites only `NegativeEnergy`), so the force-scaled branches publish b = 0's bits.
- A caller scan of the head agrees with RETURN §8:
  - the only non-test caller of `solve_assembled_with_formation_check` outside SA is `PP:4441`, with `selected = built.nonlinear_supports.is_empty()`;
  - `solve_force_scaled_with_formation_check` and `solve_with_force_scaling` have no caller outside SA;
  - NI `lib.rs` is unchanged, and it and PP name no W4 item (K5's text pin checks the same).

**The nonlinear loop and the contact-seed guard.**
- The loop's solves go through `solve_binary64` (`w4: None`).
- PP's contact-seed arm (`PP:2884`) needs both `permits_contact_seed_trial(&error, …)` and `eligible_contact_dofs(…, &built.nonlinear_supports, …).is_some()`.
- A W4 `Mechanism` arises only when `nonlinear_supports` is empty, and then `eligible_contact_dofs` returns `None` (NI `lib.rs:439-441`). So the guard stays closed.
- The loop's own arm (NI `lib.rs:645-652`) never sees a W4 error.

**My frame-only byte-identity check** (`probes/rv14_sa_probe.rs.txt`, `probes/probe_comparisons.txt`).
- **The probe.** It is an NI integration test built identically on both archives. It prints the full `Debug` of every outcome of:
  - `solve_assembled_with_formation_check` and `solve_force_scaled_with_formation_check`, selected and unselected;
  - the plain `solve_assembled`;
  - on dense and sparse evidence, at b = 0 and b = 2, in both modes.
- **The models:**
  - **F1**, a 3-4-5 L frame with a **free x translation** (node 0 held in uy, uz and every rotation; node 2 in uz);
  - F2, a free translation along y at 5e6 + 0.3 m;
  - F3, a rotation about an oblique non-dyadic line (today's screen ends unresolved);
  - F4, a restrained frame with a spring (it publishes);
  - F5, two frame bodies, the second with a free z translation;
  - X1, a frame mechanism seeded before a restrained curved body;
  - X2, a curved mechanism seeded before a frame mechanism.
- **Result: 182 lines, byte-identical base against head, except X2's selected entries.** There the head reports the curved body's W4 witness θ = (1, 1, 0) instead of main's frame translation. That is K5-C1, and first-failing-body order holds.
- **F1 also discriminates K5-M9.** Under K5-M9, 36 lines change: today's witness translates by the characteristic length (5.0), W4's canonical one by 1.0 (`mutations/logs/K5-M9.saprobe2/`).

## 2. The mathematics, against my own exact oracle

**The oracle** (`oracle/rv14_oracle.py.txt`). I wrote it without reading K5's generator's internals.
- It forms the **unreduced** stacked map in `fractions.Fraction`:
  - six unknowns (t_S, θ_S) per sub-body, with u = t_S + θ_S × (x − o);
  - six rows per tie (u and θ equal);
  - one row per `Dof` or directional ground.
- It decides the exact nullity by elimination, and checks every published witness from its node motions alone:
  - rigid on every sub-body, equal at every tie, zero on every ground, and nonzero;
  - `rigid_parameters`·L equal to node 0's [u, θ];
  - **canonical**: p_j an integer k in 1..=64, and no smaller k with every component representable;
  - with nullity 1, bit-equal to the canonical representative I compute from the exact null vector.
- The FK probe (`probes/rv14_fk_probe.rs.txt`) runs `assess_constrained_bodies` on a clean archive of the head.

**The corpora:**
- `cases_all.txt`, 4,226 cases:
  - 1,500 random bodies (1–9 nodes, 1–5 sub-bodies, tree and extra ties including ties inside one sub-body, DOF and directional grounds), with integer, decimal, 5e6-offset, mixed-magnitude (1e-9 to 1e9) and 2^40-offset coordinates;
  - 1,200 constructed mechanisms, with grounds chosen to vanish on a small-integer (t, θ), cycles along θ, shifts of 1e3, 5e6 + 0.3 and 2^30, and every fifth case perturbed by one ulp;
  - 26 targeted cases: band cycles, near-collinear virtual pins at 0 to 2^52, tie chains of 12–25 sub-bodies with steps of 0.1 to 1e15, 200- and 400-node lines with 600 and 1,200 ground rows, non-dyadic directional rows, a coincident tie, and subnormal and 1e-300 geometry;
  - 1,500 tiny and subnormal bodies.
- `cases_huge.txt`, 700 cases: coordinates of 1e16 to 1e200 and near 2^1000, and mechanisms shifted by 2^1000, 1e200 and 2^1020.

**Results** (`oracle/comparisons.txt`):

| Check | Result |
|---|---|
| Published witnesses that fail the exact check, or witnesses on a full-rank map (**false witnesses**) | **0** of 2,947 |
| `Restrained` where the exact map has nullity > 0 (**missed mechanisms**) | **0** of 1,536 `Restrained` |
| Non-canonical witnesses, or a nullity-1 witness that differs from my canonical representative | **0** |
| Non-finite `rigid_parameters` | 349, all at L < 2^-1023 (RV14-4) |
| `ERR` outcomes | 0 |
| Input order: sub-bodies, nodes in them, ties, tie ends and grounds shuffled (seed 1404) | results **byte-identical** on all 4,226 |
| Coordinates × 2^40 and × 2^-40 | statuses identical (2,726 and 2,721 cases) |

**The tie reduction (RETURN §2.2).** My re-derivation agrees.
- A tree tie (a ∈ A, b ∈ B) gives θ_B = θ_A and t_B = t_A + θ × (x_a − x_b). The code's `offset[far] = offset[near] + x_near − x_far` is this, with near in the current sub-body.
- Hence v(x) = x − o + s_B.
- A non-tree tie adds θ × (v_a − v_b) = 0, and a tie inside one sub-body gives c = x_a − x_b.
- The map (t, θ) ↦ (t + θ × s_B, θ)_B is injective, and a connected tie graph spans every sub-body. So null(stacked) = P(null(reduced)).
- The oracle's agreement on every case above is the empirical check, and it uses the unreduced map only.

**The exact witness and its scaling.**
- `null_translations` checks every tie (tree ties are exact zeros by construction, cycles are checked) and every ground kind with `Expansion`. I checked its cross-product index table term by term.
- `canonical` accepts r_i only when y·p_j = k·p_i exactly, and re-verifies r with `exact_motions` (`exact_scalar`).
- `publish` copies the verified node motions. Only `rigid_parameters` is formed afterwards (RV14-4).

**The libm-free screen** (`libm_scan.txt`).
- K5's non-test FK block (lines 250–1179), with comments and strings removed, calls no `hypot`, `pow*`, `exp*`, `ln*`, `log*`, trigonometric, hyperbolic or `cbrt` function.
- Its only floating-point methods are `abs`, `sqrt`, `max`, `min`, `signum` and `total_cmp`, plus `Expansion` and integer bit operations.
- The tie rule's `orientation()` normalizes with `sqrt` (FK `lib.rs:1881-1883`).
- The one transitive exception is FK's pre-existing `exact_radix`, which uses `2.0_f64.powi(step)` with \|step\| ≤ 512 and a round-trip self-check. RETURN §2.6 discloses it, and an inexact result could only withhold a witness.

**My attempts at a false witness or a missed mechanism at FK level** were:
- band cycles (T1: the head is correct, and only the exact tie loop decides; RV14-1);
- one-ulp perturbations of exact mechanisms (240 `M*` cases);
- near-collinear virtual pins far from the origin;
- a 25-link tie chain whose s_B accumulates;
- many-ground lines (m up to 1,200, τ_B up to 1.9e-10);
- subnormal, 1e-300 and 2^1020 geometry.

None produced one. The perturbation analysis agrees: rounding the virtual positions, cycle offsets and directional cross products moves a unit row by at most about 12u, far below τ_B = 64·γ(max(m, 6))·σ_max, with σ_max ≥ 1.

## 3. The Q2 condition (RETURN §2.3)

I checked each step.
1. **What W4 verifies.** A published d is exactly zero on every tie and ground row (verified exactly), and rigid on every sub-body by construction. My oracle confirms this for every witness in §2.
2. **Which coordinates.** Frames and users record their nodes in `EvidenceParts`. A curved-only node takes its matched macro's coordinates, and `CurvedCoordinates` refuses disagreement:
   - with a recorded frame or user coordinate (tested by K5, `k5_curved_slots_qualify_by_their_matched_source`, `k5_tests.rs:987`);
   - with another matched macro at the same node (the code is correct, but untested: RV14-2).
3. **Every family's intended energy vanishes on d.**
   - **Frames:** E = 0 on rigid motions of the pair.
   - **Qualified curved elements.** R5-4's K_int = Tᵀ[[H K_t Hᵀ, −H K_t], [−K_t Hᵀ, K_t]]T, with H from the actual chord c = R(x_j − x_i). It annihilates u_j = u_i + θ × Δ, θ_j = θ_i, because R(θ × Δ) = (Rθ) × (RΔ) for proper orthogonal R. This holds for any K_t, and needs neither K_t's accuracy nor its definiteness.
   - **Users** (as represented). K_L = Σ k_d (e_d − e_{d+6})(e_d − e_{d+6})ᵀ, and `transform_global_stiffness` (`multiply_transpose_left`, FK `lib.rs:1779-1797`) forms each 3×3 block with the same operation sequence on ±k_d: zeros add exactly, and the order of the nonzero terms is the same. So G = [[A, −A], [−A, A]] in value, and it annihilates (b, b). K5's tripwire test asserts this.
   - **Springs** vanish on their grounds, and a zero spring is no ground.
4. **Singularity.** Each family's matrix annihilates d, so K_int·d = 0, with d nonzero only on free DOFs (every prescribed DOF is a ground).
   - The linear route's `free` is every non-prescribed DOF (`PP` `solve_preview_reduced_system`), and M03's audit binds K to exactly these contributions.
   - **So a K5-C1 refusal never refuses a model the intended physics restrains.**
   - W4 runs before the audit, as today's frame screen does. A caller passing a K with extra stiffness would be refused either way.
5. **The product's element along d.** Its energy comes only from the chord mismatch, R̃'s departure from orthogonality and the formation rounding (R5-4 §2). I agree in form. This step is descriptive and carries no soundness weight.

**Counterexample attempts with a matched slot:**
- two matched macros disagreeing at a curved-only node (P1): the head refuses to qualify the body; the mutant is RV14-2;
- a macro disagreeing with a frame node: K5's test;
- matching at 2^b: K5-M13 is killed;
- a lone bend pinned at both ends, at 0 and 4,096 m;
- a dyadic mechanism at 12,345.678 m (§6).

None refused a model the intended physics restrains.

## 4. P6 and the prefilter (RETURN §2.7, §2.8)

**P6.** I checked it by reading.
- Coordinates are finite.
- The sub-body partition and the tie graph are complete by construction.
- Element spans are bounded: a frame's √(Δ·Δ) must be finite; a user element's axis must normalize; an infinite-R bend has no global matrix and stays unmatched.
- So `Range("constrained relative coordinates")` would need T + 2(N − 1) ≥ 2^457.
- Empirically, none of my 4,932 FK cases returned `ERR`, including coordinates of about 1e307. The `Geometry` branch leaves the body to the matrix gate in any case.

**The prefilter.** I checked §2.8's steps: the ideal rows, the per-kind bounds and case A's margin of 0.42·2^-20·size. Case B holds:
- an `Ok` exact check forces θ_b·v_{x,c} ∈ ηℤ, so θ has one nonzero component, and the dot is exactly zero, or below η/2 for kind (c).

**I re-ran I14's declared-equivalent mutant** (the row loop removed) on all 4,226 cases:
- the corpus includes 1,500 subnormal and tiny bodies (Case B) and the band cycles (Case A with cycle rows);
- the FK probe's output is **byte-identical** to the head's (sha256 `862d4614…`).

## 5. Mutations (`mutations/MUTANTS.txt`)

**Method.**
- Each mutant ran in its own clean `git archive` copy of the head, with its own target, deleted afterwards. NONE ran first.
- The test sets were FK's `k5_constrained_bodies`, `k5_scale` and `rigid_body::` and `k5_` unit tests (FK filtered), NI's full library (127 tests), and PP's `k5_curved_mechanism_runtime`, as each mutant needed.
- A kill counts only at a test assertion. No run had a compile error, an abort or a signal.

| Mutant | Kind | Result (assertion sites at the head) |
|---|---|---|
| NONE | control | passes: FK 8 + 14 + 1, NI 127 |
| K5-12 (a tie as a rigid link) | re-kill | killed: `k5_constrained_bodies.rs:235`, `k5_scale.rs:104` |
| K5-M1 (cycles ignored) | re-kill | killed: `:234` |
| K5-M6 (exact check skipped) | re-kill | killed: `:242`, `:249` |
| K5-M17 (directional ground checked in binary64) | re-kill | killed: `:249` |
| K5-M9 (every body through W4) | re-kill | killed: `k5_tests.rs:730` (C's pin) |
| K5-M13 (b = 0 bits at 2^b) | re-kill | killed: `k5_tests.rs:1097` (I14's `:1065`, RV14-N1), and `:493` |
| K1-PIN-BINARY64 (K1 pin) | re-kill | killed: `s11k_tests.rs:1478`, `:807`, `:577`; product panics `lib.rs:4781` and `:4827` not counted |
| K1-PIN-LOOP-B (loop pin; K1's `mutate_k1.py`) | re-kill | killed: `k5_tests.rs:613` (K5's loop pin), `k1_tests.rs:1000`, `s11k_tests.rs:519`, `:834`, `:924`, `:1141`, `:1283`, `:1478` |
| KD5-M32a (loop pin) | re-kill | killed: `k1_tests.rs:1000`, `s11k_tests.rs:577`, `:807`, `:1176`, `:1283`, `:1478` |
| K5-PREFILTER | I14's declared equivalent | survives, as derived; output identical on 4,226 cases (§4) |
| **RV14-M1** (exact tie loop removed) | own | **survives** FK: 6 false witnesses on my corpus (RV14-1) |
| **RV14-M2** (curved-to-curved agreement dropped) | own | **survives** NI: a false `Mechanism` on P1 (RV14-2) |
| RV14-M3 (spring grounds dropped from W4) | own | killed only by `unwrap` panics in K1 and K-D5 helpers; a false `Mechanism` on P2 (RV14-N2) |
| **RV14-M4** (K5-C2 mapped to `Ok`) | own | **survives** NI and PP; reverts K5-C2 in the product (RV14-3) |

## 6. The product runs and the change classes

**Reclassification** (`product_runs_reclass.txt`). I reclassified I14's raw records (`a2/product_runs_{main,candidate}.txt`) without I14's classifier:
- 152 runs of 38 inputs;
- 128 byte-identical;
- 16 K5-C1: main's `NUMERICAL_INTEGRITY_UNRESOLVED` becomes `NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM`;
- 8 K5-C2: `NUMERICAL_INTEGRITY_UNRESOLVED` stays, with the new reason;
- **0 outside the two classes, 0 changes with a gap support, and 0 changed runs where main published** (`MECHANICS_SOLVED`).

So no published result is removed. The table (`product_runs_table.md`) agrees.

**Re-run from archives** (`probes/pp_runs_*.txt`, `probe_comparisons.txt`).
- My harness reuses I14's model functions on a reduced corpus: R 0.25 at 1,024 m (mechanism, companion and gap), R 0.2 at 0, R 0.3 at 1,000, `arc_model_k2` and `pp_route_utm_5e6`.
- I added 3 inputs of my own: a lone arc pinned in translation at both ends, at 0 and at 4,096 m; and the mechanism at 12,345.678 m.
- I ran both entries in both modes, on both trees.
- **All 28 lines that overlap I14's records reproduce byte for byte on both trees.**
- My new inputs are K5-C1 on the head, where main refused them `NUMERICAL_INTEGRITY_UNRESOLVED`. The lone arc's witness is θ = (1, 1, 0) with zero translations, the rotation about its chord.
- On the head copy, PP's `k5_curved_mechanism_runtime` (3) and `formation_check_runtime` (5) pass.

**Gates.**
- T9 is 112/112 byte-identical, and the base equals the Mac calibration's hashes (`b/t9_compare.txt`, re-diffed).
- Gate part 1: 884/884 runs identical, and `gate_check` passes (`b/gate_part1_compare.log`).
- Both are recorded as Mac-only comparisons against Mac main.

## 7. Records and hygiene (`records_checks.txt`, `gen8.txt`)

- **Checksums:**
  - K5's `SHA256SUMS`: 187 of 187 OK, set-equal with the folder;
  - the FK vectors' `SHA256SUMS`: OK;
  - `gen_k5_vectors.py --check`: OK.
- **GEN-8** passes at the head in `<wt>/k5`: 1 passed. The working tree was clean before and after.
- **Scans** (with `/usr/bin/grep`) of the 199 slice files:
  - 0 machine paths;
  - 0 hits of the user's name;
  - the only model-scan hit is RETURN.md:12's host name (RV14-N4).
- **T9 and the gate are stated as Mac-only** (RETURN:10, :484; CHANGE_RECORD:18, :60).
- **Whitespace.** `git diff --check` finds exactly the disclosed items (RETURN §14):
  - `a2/checkpoint0_probe_main.txt`: 24 lines with trailing whitespace;
  - `b/gate_part1_record.txt`: one line with trailing whitespace;
  - `c/logs/C-VERIFY/ni5.txt`: a blank line at the end of the file;
  - `b/gate_part1_result_candidate.json`: no final newline.
  - Nothing under `core/` is affected.
- **Interfaces.** RETURN §11's line counts and every §9 interface anchor and signature are exact at the head. FK: `:265`, `:274`, `:292`, `:315`, `:330`, `:351`, `:464`. SA: `:146`, `:575`, `:1249`, `:1347`, `:1356`, `:1368`, `:1418`, `:2142`.
- **Build.** NI's non-test build at the head has no warnings.

## 8. What I did not do

- I did not re-run the 39-manifest suites, T9 or gate part 1. I re-checked their records only (§6), and the code is unchanged since B apart from tests.
- I did not run the PP full suite.
- I did not verify hosted CI myself beyond the check summary.
- I did not test P6's curved-bend `Range` argument by construction; I checked it by reading only.
- I did not check the reason text's rendering in the headless entry beyond PP's typed entry.

## Records (`REVIEW/_run_records/k5_review/`, with its own `SHA256SUMS`)

`README.txt` indexes the folder:
- the revision, records, GEN-8, libm, product-run, memory-guard and toolchain checks;
- `oracle/`: the oracle, the corpora (seeds 1402, 1403, 1404, 1405), the results and reports;
- `probes/`: the FK, SA, SA-mutant and PP probes with both trees' outputs;
- `mutations/`: my patch script, K1's script (`c2b54737…`), the runner, the table and every log;
- `build_records.py.txt`, which assembled the folder with placeholders and checked it for machine paths.
