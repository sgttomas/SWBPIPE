# I99 B3-W: the exact route's verdicts (a probe; records only)

TASK (Type 2), I99, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC (2026-10-07 host local time).

**Brief (verified before work):** `R/BRIEFS/B2W_B3W_PROBES.md`, sha256 `b7e2fdc726f86075b01121b8ec99cc4cc94c2836a6ec1a8b2835de8b0686daea`, section "B3-W" and its host rules. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first. I98 runs B2-W from the same brief; nothing here is shared with it.

**Basis read:**
- the method: I81's `R/I81/b1_probe_01/PROBE.md` with `_run_records/zz_i81_probe.rs`, `instrumentation.diff` and `probe_run2.log`; I86's `R/I86/b1_w_probe_01/PROBE.md` with `_run_records/zz_i86_probe.rs`, `instrumentation.diff`, `sanitize.py`, `compare_runs.py` and `round1_controls.log`;
- the plan: I93's `R/I93/b2b3_plan_01/PLAN.md` §0, §1.2.6, §1.3, §1.4 and §6, and `REVISION_01.md` §5 (the B3-W outline);
- RR: "B2/B3 R1: …" (RV114's notes as RR states them; I did not open RV114's review itself), "I93's REVISION_01 accepted; …", "I95's B3-S: …", "I96's B3-D design returned; …", "RV116 (RV-D) accepts B3-D …", "RV116 confirms B3-D's revision 01; …", "#1112 merged: …" and "B2-W and B3-W dispatched as I98 and I99; …";
- B3-D: `R/I96/b3_d_01/DESIGN.md` §0, §1.3, §1.4, §3, §4, §5 (P-2, P-11, P-13), §6.3, §6.4 and §13, with `REVISION_01.md` §9;
- `R/I78/b0_contract_01/DESIGN_v2.md` T-3, T-4 and the exact-route item of §5 (coexistence is per invocation).

Code is cited by symbol, at main `0b6c5d7362`.

**Placeholders:** `WT`, `NUM`, `P`, `T`, `R`, `RR` and `VENV` as in the dispatch. `PP` = `P/core/product_physics/src`. `S` = `WT/scratch/i99_b3w/` (my scratch). `ARCH` = `S/arch/` (the disposable archive). √eps = 1.4901161193847656e-8. "4M" and "8M" are the per-case exact-block work limits 4,000,000 (`SourceRecoveryBudget::default()`, `permitted_run`'s) and 8,000,000 (`PHYSICS_SOURCE_WORK_LIMIT`, `ordinary_dispatch`'s on the exact route).

**Limits kept.** Records only; no maintained file changed. No Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`. No DEC-025, no evidence sweep, no installs. Every cargo job went through `WT/tools/t3_cargo.sh` (memory guard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, with `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` unset. Before each heavy job the runner waited while more than one `/usr/bin/lockf` process ran; I98's run held the lock before my first probe run, which queued behind it (`_run_records/cargo_jobs_i99.log`). Each job had one wait, which ended with its process. `TMPDIR` pointed into `S`; nothing was written to the system temp directory. The read-only tools ran with the host's `python3` (standard library), as I81's and I86's did; VENV was not needed.

## 0. Outcomes in brief

| Item | Outcome (both modes unless stated) |
|---|---|
| 1. The milestone as 0.3.0 exact, n05, n06 | **n05 and n06 select** (physics-source-1 `qualified`, `retained_source_blocks_exact_v1`; 3,059,209 / 3,061,027 and 2,932,985 of the 8M per-case limit). **The milestone authored as exact (`m3x`) does not:** its exact-block attempt is refused at source closure, `Unsupported("actual transform is not a signed permutation")` (the skew member), after 46,628 units. `m3x` is **`sensitive`** by K-D5's D-5 line at `N1:RX` (trigger ratio 2.4279 sparse, 4.9173 dense; rcond 2.6065e-8 is above √eps), exactly as the 0.1.0 milestone is. So **`m3x` is the exact successor's witness and n05 and n06 are the coexistence pins.** n05 is `sensitive` by its report (rcond 1.058e-11); n06 is `unresolved` (`NumericallyUnresolved`, "positive diagonal contribution absorbed by assembly", `global_dof: Some(3)`). With empty regions `m3x` publishes **98 / 99 rows = 7n + 51m + 8g + the mode row (+ the parity row in dense)**, n = 2, m = 1, g = 4: I96 §3's families exactly, **no pressure-evidence row**. |
| 2. A mixed exact base | **Found, in the third of three variants run (one of the four unused): `m3x_mix_anchor`.** The milestone's model, authored as exact, with a second case `case:b` that is a 1 N force on N0's rigidly restrained UX. Verdicts `case: sensitive`, `case:b: checks_passed` in both modes; physics-source-1 selects neither (case: refused at source closure; case:b: not attempted, legacy `not_required`). Under T-3 (c) and T-4 it is a two-case exact invocation with `case` in A and `case:b` `not_required`. The other two variants (`axial`, `lateral`) give that shape in dense only: in sparse, `case:b` is also `sensitive` (K-D5 at `N1:RX`). |
| 3. B3a | **The ordinary bytes do not equal the 0.1.0 milestone's except for a model echo.** The ordinary envelope carries no model echo (`model_ref` is the project id, unchanged). **They differ in exactly one JSON path:** the `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` message, whose legacy exact-block refusal becomes "legacy source-blocks namespace requires model0.1/0.2 without pressure contract or regions", charged 38,336 instead of 46,628 (+45 bytes). The private driver's W1 on it reaches precommit: native Selected, candidate certified, staged, serialized, then **G8 `RETAINED_PRECISION_INVOCATION_MISMATCH`** (the Rust reader requires `pressure_contract` null). That successor differs from the 0.1.0 milestone's in exactly 3 paths: the invocation hash (the model echo), `legacy_source_work[0].charged`, and the receipt hash. |
| 4. P-2's budget parity (a note) | **Selection differs for one committed exact input.** `fields` (the committed physics-source fixture, added for this note only) selects at 8M (6,791,052 / 6,792,870 charged per case). At 4M the attempt selects, but its finalization replay exceeds the budget: `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (blocking), `MODEL_INCOMPLETE`, 0 results, contract physics-1, and no `source_block_recovery`. n05 and n06 select under both, and their bytes differ in 4 paths (`work.limit`, the selected message, `publication_sha256` and `receipt_sha256`). `m3x` and the variants differ in 1 path (the unavailable message's `limit`). **So P-2 is load-bearing, not cosmetic.** |
| 5. Controls | **Runs 2 and 3 are identical** (300 of 300 probe lines; 77 of 77 written outputs). Run 1's 65 outputs equal run 2's same-named files. **I81's and I86's milestone lines are reproduced:** 16 and 17 fields per mode, 0 differences. The value route's n05, n06 and `fields` outputs equal the committed raw fixtures as JSON values (6 of 6). |

**No stop rule fired.** B3-W's brief section names none of its own; item 2 found a base within its four variants.

## 1. Setup and build identity

- **Archive:** `GIT_OPTIONAL_LOCKS=0 git -C NUM archive 0b6c5d7362ff21104e078d8168f62e3f38928cf9 -- . ':(exclude)projects/chirality-piping/execution' | tar -x -C ARCH/`.
  - **`P/execution` was excluded** (2.35 GB of records). No source under PP, `P/core/solver` or RE `src` reads it (`git grep -l 'execution/'` over those trees finds only test fixtures, a README and test files).
  - Before editing, the three production files I instrumented were checked: PP `lib.rs` sha256 `4c33c25031622db865bb50df1d45ad41429d16de1b7143b9ac6a22915d399763`, `retained_product.rs` `f536bfe786684baa0246552e013f668c544acdcab810580496991f5d26aba794` and `retained_memory.rs` `fbc7c9db1e8aae3e3c058159f07f89b35317af5d7cb229cf0ed72e81b7b6c7b4`. These equal I81's and I86's recorded pre-edit hashes.
  - Between I81's `d8c88774d0`, I86's `47a3bdfcf5` and main `0b6c5d7362`, `P/core/product_physics`, `P/core/solver`, RE `src` and `P/fixtures/product_preview` are unchanged (`git diff --stat` lists only `P/fixtures/results` and `P/schemas` files).
- **Target:** `WT/targets/i99-b3w/`. The test binary is `open_pipe_stress_product_physics-2e7eada646dc82a0`, sha256 `078d158e…206e` (`_run_records/test_binary.sha256`). One build ran every probe run.
- **Registered build, observed:** the probe's `registered()` is `true`, and every admission report reads `profile=Registered`.
- **Probe-only instrumentation** (`_run_records/instrumentation.diff`, applied by `apply_instrumentation.py`; archive only; every addition is `cfg(test)`; no control-flow change): I86's prints renamed `I99_`, without I86's timing marks. They are `I99_SEEDS` in `permitted_run`, `I99_W1_START`, `I99_PREPARATION_FAILURE`, `I99_CANDIDATE_REFUSAL` and `I99_PRECOMMIT_ERROR`/`_DUMP` in `retained_w1`, and `I99_NATIVE_OUTCOME` in `solve_native`. The module is declared in `retained_memory.rs` beside `law_tests` and `witness_tests`, as I81's and I86's were.
- **The probe module** is `_run_records/zz_i99_probe.rs`. Every test is `#[ignore]`. For each input and mode it records four things.
  - **The ordinary route:** `run_linear_static_preview_value_with_mode`, which is `ordinary_dispatch` (8M on the exact route). It records the contract, the verdicts, physics-source-1's per-case selection and work, the rows per case and kind, the diagnostics, the reciprocal condition estimate, K-D5's line and the contract evidence. The bytes are written to `S/out/`.
  - **One counted Direct invocation:** I81's and I86's `probe`, with the admission report, the W1 cause and the byte equalities.
  - **The witness twin:** the S1 witnesses' private driver (`permitted_work`, copied as I81 and I86 did), on `on_reserved_stack(4 MiB, carry_test_hooks(…))`, with `SourceRecoveryBudget::default()` (4M). It skips G-A, G-B and G-C. The seeds and the capture error are printed after its ordinary run, so a coexistence run shows them too.
  - **P-2's note:** `run_linear_static_preview_captured` at 4M and at 8M, each compared with the ordinary route's bytes.
- **Inputs** are files written by `_run_records/gen_inputs.py` from committed bytes (§2).

## 2. Inputs (all in `_run_records/inputs/`; sha256 in `_run_records/inputs.sha256`)

| Label | What it is | File sha256 | Value sha256 |
|---|---|---|---|
| `milestone` | the committed 0.1.0 milestone (`P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json`, built in) | (I86's `dcdc8e65…140b` pretty dump) | `8d1967a756b09072f7359cafa5818aad259c2e01dc45f9305cdf5459cd422c76` |
| **`m3x`** | **the milestone authored as 0.3.0 exact:** schema `0.3.0`; contract `{2.0.0, exact_straight_pressure_v2}`; the material's shear modulus removed and `constitutive_basis: homogeneous_isotropic_E_nu_v1`, `poisson_ratio: {0.25, "1"}` added, so E = 2e11 Pa and ν = 0.25 (G = E/(2(1+ν)) = 8e10 Pa exactly, the 0.1.0 milestone's G); `pressure_regions: []` on the case. Nothing else changed | `0ffbea359c2e22b18b5274ad26172dfeab9720be7506f94f004faf2b44bae617` | `c920a96dc542c3ecadb63f724cfcf243d73d5dd71692e4f48827a620bb0e5497` |
| `n05`, `n06` | the committed `P/fixtures/product_preview/physics_source/n05.request.json` and `n06.request.json`, copied byte for byte | `332319ee6f47870a074a6c16fbf2f43c0171e376a0b4cd1cb7f7ad8b59254b00`; `5551f164b9e8ab88f04f3abce810e1e1b1ff47236bba6a296e32905f628be931` | `ebc8f81e…07e3`; `73d38e3b…315b` |
| `m3x_mix_axial` | item 2, variant 1: `m3x`'s model with `case` (the milestone's moments) and `case:b`: forces at N1 of 1, 2 and 2 N in global X, Y and Z (axial: the member's exact direction (1, 2, 2)/3, 3 N) | `93c6ca05617b69cbf5760b81896a8a182e404d5c2816f61aa4438a4870c3eb31` | `3ea5d82b…67f1` |
| **`m3x_mix_anchor`** | item 2, variant 2: `case:b` is a 1 N force in global X at N0, whose translations are rigid (no free load term) | **`3c9a6fdb912d9305d65a034af020f6bfe78933529b14f3945845d38126e7a693`** | **`6ca777a6e3ed658bcf58813d277e839033ac07f0b84efb8526351a20c67d1ee0`** |
| `m3x_mix_lateral` | item 2, variant 3: `case:b` is a force at N1 of (0, 1, 1) N, whose moment about N0, (0, −1, 1) N·m, has no global-X component (the soft RX spring) | `4be33eab9b38054ce92eaee54e818dde82205312ea37713fc467c5c548279741` | `69f8e69a…8ceb` |
| `m1_twin_axial`, `m1_twin_anchor`, `m1_twin_lateral` | each item 2 `case:b`'s one-case 0.1.0 twin (U8-0's proxy method): the committed milestone with that case's loads in place of its own | `9bd7bfd6…710a`; `898878221a…04d2`; `9e4fc5eb…215a` | `4a70c5df…9a7b`; `947b6206…304e`; `e12fe377…2c42` |
| **`m3l`** | **the milestone authored as 0.3.0 `legacy_pressure_v1` with zero pressure:** schema `0.3.0` and contract `{1.0.0, legacy_pressure_v1}`; nothing else changed (no pressure primitive, no regions, E and G kept) | **`c32170b3c224f90cf4481eaea99e44d21ae155fbe56055a5b396aac28a6384f5`** | **`2f5ff465bfa97005a581d88e02c0a5478ea24da3bdc1c2eb9fcda491803fc0c3`** |
| `fields` | the committed `physics_source/fields.request.json`, copied byte for byte; **used for item 4 only** | `7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002` | `5ab9edb3…1a2b` |

Every 0.3.0 input is refused by main's Direct census at D1.3, `Family(Namespace, SchemaVersion)`. The Direct entry then takes `ordinary_dispatch`, and its bytes equal the ordinary route's in every 0.3.0 row (`bytes_eq_plain=true`, no notice).

## 3. Item 1: the milestone authored as exact, n05 and n06

| | `m3x` sparse / dense | `n05` sparse / dense | `n06` sparse / dense |
|---|---|---|---|
| Contract; profile | physics-1; `exact_straight_pressure_v2` | **physics-source-1**; same profile | **physics-source-1**; same profile |
| Status; results | `MECHANICS_SOLVED`; 98 / 99 | `MECHANICS_SOLVED`; 81 / 82 | `MECHANICS_SOLVED`; 81 / 81 |
| Ordinary bytes (sha256, length) | `5978e4dc9f8e7810fa858eff2a2decb3ba7d69284a6656502cf17b8f164f257a` (69,456 B) / `d616999bdea43eb4dfcdba51ecfb5aed5f54473b1eeeba0ba5c94f935419dea7` (70,540 B) | `f222706855fc10360ec2468e2a13a2614274fd56be958376f648e55d276966d7` (115,957 B) / `d963aa24e0e1642aaea6eb12bf9df376964001f1b94715b8210c772df412b15c` (117,214 B) | `f283835844a1dee6ac2711b2bce36dd43b2505c19fdbfcb9ad0a0bb363817e13` (108,899 B) / `64c8113d9828faed2b6af06da085e2acfbf99c5c8a659fd44038662e38e77a55` (108,887 B) |
| **Verdict** | **`sensitive`** (`passive_model_basis`) | **`sensitive`** | **`unresolved`** (`numerically_unresolved`) |
| Why | **K-D5:** the D-5 line at `N1:RX`, trigger ratio 2.4278870 (sparse) and 4.9172710 (dense); rcond 2.6065e-8, above √eps | the report: rcond 1.0578e-11, below √eps; no D-5 line | `NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED` |
| **Seed** (private driver) | `initial: report/NUMERICAL_INTEGRITY_SENSITIVE`; `w2: not_triggered`; D-5 ref set; legacy `unavailable(stage=source closure)`; `recovery_demoted: false`; no load-row finding | `initial: report/…SENSITIVE`; `w2: not_triggered`; legacy **`exact_selected`** | `initial: structural_failure/numerically_unresolved`; `w2: not_triggered`; legacy **`exact_selected`** |
| **physics-source-1** | **not selected:** `RecoveryFailure { stage: "source closure", error: Unsupported("actual transform is not a signed permutation"), charged 46,628, limit 8,000,000 }` | **selected**, `retained_source_blocks_exact_v1`, `qualified`; 3,059,209 / 3,061,027 of 8M | **selected**, `retained_source_blocks_exact_v1`, `qualified`; 2,932,985 of 8M |
| Direct (main) | D1.3 refusal; ordinary bytes; 0 notices | the same | the same |
| **Private driver (W1 today)** | W1 starts and falls back at **Preparation** (furthest phase W2). The capture refuses the exact model: `Association("outside private ordinary no-component/no-combination scope")` (`ProductCapture::normalized`) | **`ExactSelected`** (coexistence; no W1 work) | **`ExactSelected`** |
| Native terminal | **not reached on main.** Proxy below | none (coexistence) | none |

**Which input selects (I96 §13's open question).** n05 and n06 select, so each is a T-3 (c) coexistence invocation: exact ordinary bytes. `m3x` does not select, and its verdict is in A under T-4. **So `m3x` is the exact successor's witness, and n05 and n06 are P-13's coexistence pins.** This is as PLAN §1.4 expected.

**The rows the exact route publishes with empty regions** (`m3x`, case `case`; `_run_records/summary.md` lists every input):
- 2 `displacement_magnitude`; 2 each of `global_nodal_displacement_{x,y,z}` and `global_nodal_rotation_{x,y,z}`;
- 24 `support_reaction_component_v2`, 4 `support_reaction_force_magnitude_v2`, 4 `support_reaction_moment_magnitude_v2`;
- 5 each (end_i, end_j and the three stations) of the six `element_local_*` actions and the four `element_local_*_stress` kinds;
- 1 `pipe_elastic_normal_stress_maximum_v2`;
- 1 `linear_solver_mode_basis`, and in dense 1 `sparse_live_path_dense_parity_relative_delta`.

That is 7n + 51m + 8g + 1 (+ 1) with n = 2, m = 1, g = 4: **I96 §3's table exactly.** No `pipe_wall_*`, `pipe_axial_membrane_*`, `pipe_lame_*`, `pipe_section_pressure_*`, `open_formula_stress_summary`, `reaction_resultant` or modulus record is published.
- **Contract evidence:** `{pressure: [], connector: [], exact_cases: [1 case]}`. The exact case has eight keys: `load_case_id`, `material_basis`, `pipe_materials`, `pipe_sections`, `pipe_stress_extrema`, `pressure_rhs_assembly`, `profile_mode` and `stress_maximum_coverage`; there is no `recovery_method`. `pipe_materials`: E 2e11, **G 8e10 (derived)**, ν 0.25.
- **`pipe_sections` are SourceAnnulus's bits:** A `3f7872fa3a37ac13`, I `3efc52664442210b`, J `3f0c52664442210b`, Z `3f31b37feaa954a7`. That is I96 §1.3's "published" row exactly, which confirms B3D-3's premise on a produced output.
- **n05 and n06 (selected)** publish 81 rows, the same families minus the mode row: no ordinary mode row beside a selected source. n05 adds the parity row in dense (82); n06 has none in dense (81). Their exact cases carry `recovery_method` (nine keys).

**The native terminal: a proxy, not a measurement.** On main the capture refuses every exact model before preparation, so no exact input reaches the native kernel. Its one-case 0.1.0 twin is the committed milestone. W1's native inputs are the same for both:
- the prepared section comes from (OD 0.2 m, effective wall 0.01 m) on both routes (`prepare_product_annulus`; the exact evidence shows `effective_wall_thickness_m` 0.01);
- E = 2e11 and G = 8e10 on both;
- the supports and loads are the same.

For the twin, **native is Selected, and the successor publishes and passes the Rust reader in both modes** (§7). On the exact route the candidate certificate uses `ExactENu` (B3-K), so the candidate's outcome there is B3b-P's to establish. This proxy predicts only the native stage.

## 4. Item 2: a mixed exact base (3 of at most 4 variants; all on `m3x`'s skew model)

Because the skew transform is refused at source closure for any case, physics-source-1 can select no case on this model. What remains is to find a second case that is `checks_passed` in both modes beside the milestone's `sensitive` case.

| Variant (`case:b`) | Mode | Verdicts (`case`, `case:b`) | `case:b`'s evidence | physics-source-1 | Classes under T-3 (c) / T-4 | `case:b`'s 0.1.0 twin on main's Direct entry |
|---|---|---|---|---|---|---|
| `axial` (3 N along the member) | sparse | sensitive, **sensitive** | K-D5 at `N1:RX`, ratio 2.8724; rcond 2.6065e-8 | neither selected (`case:b` refused at source closure, 46,998) | no coexistence; both in A | sensitive; native Selected; Candidate `Predicate { row 2, SharperExact }` |
| | dense | sensitive, **checks_passed** | no D-5 line | neither (`case:b` not attempted, legacy `not_required`) | `case` in A; `case:b` not_required | checks_passed; native Selected; Candidate (row 3) |
| **`anchor`** (1 N on N0's rigid UX) | **sparse** | **sensitive, checks_passed** | no D-5 line | **neither** (`case` refused at source closure, 46,628; `case:b` not attempted, legacy `not_required`) | **`case` in A; `case:b` not_required** | checks_passed; native Selected; Candidate `Native(MissingUniquenessWarrant(0))` |
| | **dense** | **sensitive, checks_passed** | no D-5 line | **neither** | **the same** | the same |
| `lateral` ((0, 1, 1) N at N1) | sparse | sensitive, **sensitive** | K-D5 at `N1:RX`, ratio 1.3306 | neither (`case:b` refused, 46,374) | both in A | sensitive; Candidate (row 10) |
| | dense | sensitive, checks_passed | no D-5 line | neither | `case` in A; `case:b` not_required | checks_passed; Candidate (row 11) |

In every variant and mode, `case` keeps the one-case `m3x`'s verdict, ratio and rcond. The two cases share the model's stiffness.

**Recommended mixed exact base: `m3x_mix_anchor`.** Its predicted B3b outcome is a two-case exact successor: **`case` `selected`** (by §3's proxy) **beside `case:b` `not_required`**, with no coexistence.
- **`case:b` is degenerate by construction:** its load enters only N0's rigid restraint, so every free displacement is exactly 0 and the reaction carries the force. That is sufficient for a `not_required` operand, whose rows stay ordinary.
- **No non-degenerate both-mode variant was found in three.** `axial` and `lateral` are mixed only in dense. The fourth variant was not used.
- **On main the two-case W1 cannot run:** the private driver returns `Domain` for two cases (`w1_case_id`), before the capture's scope refusal matters. The base is therefore established up to its verdicts and selections; its successor is B3b-P's, after B1's n-case path.
- The `case:b` twins' Candidate refusals are recorded only because main has no T-4: under T-4 a `checks_passed` case gets no product attempt.

**The other reading of item 2, exact-block mixing,** is already committed and needs no variant. The `physics_source/mixed` fixture's committed raw output (read, not run) has `case` selected beside `case:ordinary-pressure` (`ordinary_*_structural_v1`). That invocation is coexistence under T-3 (c), so it publishes exact ordinary bytes and is no successor base. It also has a non-empty region, so it is outside D1.5-exact.

## 5. Item 3: B3a, the milestone as 0.3.0 `legacy_pressure_v1` with zero pressure (`m3l`)

- **Ordinary route:** preview-physics-1; `MECHANICS_SOLVED`; 98 / 99 rows; `sensitive` with **K-D5's line identical to the 0.1.0 milestone's** (`doubled_correction=8.093801056572112e-14`, ratio 2.427897612739378 sparse; 4.852606290728372 dense). The bytes are `92c18d907561332fa3ffc9a5b084c7f1018fd4fd392b42d07d74899c0c36540a` (68,295 B) and `d4ecc214cf195702955b094a4e22ccb2f6884bae397df41cdf680327f1a57331` (69,411 B).
- **The difference from the 0.1.0 milestone's ordinary bytes** (`9c7ec1a1…0871`, 68,250 B; `21ca629c…278a`, 69,366 B), from `_run_records/b3a_diff.log`: **exactly one JSON path in each mode, `$.diagnostics[3].message`** (`SOURCE_BLOCK_RECOVERY_UNAVAILABLE`).
  - 0.1.0: `… error: Unsupported("actual transform is not a signed permutation"), work: WorkReport { charged: 46628, rejected: 0, limit: 4000000 } }`
  - 0.3.0 legacy: `… error: Unsupported("legacy source-blocks namespace requires model0.1/0.2 without pressure contract or regions"), work: WorkReport { charged: 38336, rejected: 0, limit: 4000000 } }`

  So `source_recovery`'s namespace check now refuses before the transform check, as I96 §4.1 read it ("cheaper than a 0.1/0.2 attempt"). **The ordinary envelope has no model echo:** `model_ref` is `project.id` and `schema_version` is the mechanics constant `0.2.0`, both unchanged. **The brief's "equal except the model echo" therefore does not hold literally for the ordinary bytes.** The exception is this one diagnostic message.
- **Direct (main):** D1.3 refuses (`SchemaVersion`). The bytes are the ordinary route's, with 0 notices.
- **Private driver:** W1 runs to **precommit**:
  - native Selected; the candidate certified; staged; serialized;
  - then **`Fallback(Precommit { G8, RETAINED_PRECISION_INVOCATION_MISMATCH })`** (furthest phase W4). The accepted Rust reader's G8 namespace check requires `pressure_contract` null (`retained_precision.rs`, D31's predicate). That is exactly the reader change B3-D §6.3 names.
- **The successor it built** (from the precommit dump) **against the 0.1.0 milestone's published successor:** exactly three paths in each mode:
  - `retained_precision.body.invocation.value`, the model echo: `87eb7e84…1b5a` → `f8811f51…67c3` sparse, `c6922374…4309` → `e529f94d…299e` dense;
  - `retained_precision.body.legacy_source_work[0].charged`: 46,628 → 38,336;
  - `retained_precision.receipt_sha256`.

  The unavailable diagnostic is in neither successor: the selected case omits it (T1 (a), P-11).
- **For B3a:** I93's "no producer change" holds on main's producer. What remains is D1.3 (admission) and the readers' G8 predicate. B3a's two pins can state the exceptions exactly:
  - **ordinary bytes:** the 0.1.0 milestone's except that one message;
  - **successor:** the 0.1.0 milestone's except `invocation.value`, `legacy_source_work[0].charged` and `receipt_sha256`.

## 6. Item 4: P-2's budget parity (a note; `_run_records/budget_diff.log`)

| Input | 8M (the ordinary route) | 4M (`permitted_run`'s default) | Paths that differ |
|---|---|---|---|
| `n05` | physics-source-1, selected (3,059,209 / 3,061,027) | **selected** (same charge), `work.limit` 4,000,000 | 4: the selected message's limit, `cases[0].work.limit`, `publication_sha256`, `receipt_sha256` |
| `n06` | selected (2,932,985) | **selected** | 4 (the same) |
| **`fields`** | **physics-source-1, selected** (6,791,052 / 6,792,870); 81 / 82 results | **not published:** the attempt selects (3,161,034 charged), but the finalization replay exceeds 4M, giving `SOURCE_BLOCK_RECOVERY_FINALIZATION_FAILED` (blocking; `Exact(Budget)` at the expected-invocation replay: charged 3,999,156 sparse and 3,988,134 dense, rejected 12,800, of 4,000,000). The envelope is **`MODEL_INCOMPLETE`, 0 results, contract physics-1, no `source_block_recovery`**. The seed still reads legacy `exact_selected` | 14 |
| `m3x` (and every item 2 variant) | not selected | not selected | 1: the unavailable message's `limit` |

**Physics-source-1 selects differently under 4M than under 8M.** Without P-2, `fields` on a B3b Direct entry would:
- lose T-3 (c), because there is no `source_block_recovery`;
- publish a blocked envelope instead of the qualified physics-source-1 result, which is the opposite of T-3 (c)'s "exact ordinary bytes whenever exact-block selects".

n05 and n06 only discriminate through `work.limit`, as I96's N-3 said.

**Two further consequences, for B3b-P:**
- The successor receipt's `legacy_source_work[]` carries `limit` (4,000,000 in the milestone's), so P-2 also fixes that member's value on the exact route.
- The witness twin (the private driver) uses 4M. So until P-2 lands it is not byte-faithful to the Direct entry on exact inputs: here its ordinary bytes differed from the ordinary route's for every 0.3.0 exact input.

## 7. Controls

1. **Determinism** (`_run_records/compare_runs.log`): runs 2 and 3, the final input set, gave 300 of 300 identical `I99_` lines, and their 77 written outputs are identical (`diff -r`). Run 1 (nine input files, before the `lateral` variant and its twin were added) wrote 65 outputs, all byte-equal to run 2's same-named files.
2. **I81 and I86 reproduced** (`_run_records/compare_i81_i86.py`, `compare_i81_i86.log`). For the 0.1.0 milestone in both modes, I compared these fields: the input sha256; the ordinary results, plain sha256 and length, and verdicts; the Direct cause, counts, notices, both byte checks, published sha256 and length; the receipt sha256, Rust reader verdict and case status; and, for I86, the witness twin. I81's 16 fields per mode and I86's 17 give **0 differences** (published successors `1d9ba709…a8c9` and `7c5fe5c5…a595`; `rust_reader=PASS`; `case_status="selected"`). So main `0b6c5d7362` behaves as `d8c88774d0` and `47a3bdfcf5` did on that input.
3. **The committed physics-source raw fixtures** (`_run_records/compare_committed_raw.log`): the ordinary route's n05, n06 and `fields` outputs equal `{n05,n06,fields}-{sparse_interactive,dense_scrutiny}.raw.json` as JSON values, 6 of 6.

## 8. Recommended witnesses

| Use | Input | sha256 (file; Value) | Expected, as observed on main |
|---|---|---|---|
| **B3b: the exact successor** (P-13) | `m3x` | `0ffbea359c2e22b18b5274ad26172dfeab9720be7506f94f004faf2b44bae617`; `c920a96dc542c3ecadb63f724cfcf243d73d5dd71692e4f48827a620bb0e5497` | `sensitive` (K-D5), physics-source-1 refused at source closure, 98 / 99 rows; W1 runs after B3b. Native predicted Selected (§3's proxy) |
| **B3b: coexistence** (P-13, and P-2's byte pins) | `n05`, `n06` (committed requests) | `332319ee…4b00`, `5551f164…e931` | selected; exact ordinary bytes; ordinary sha256 as §3 |
| **B3b: P-2's selection discriminator** (proposed addition) | `fields` (committed) | `7f8ff9d5e23712cedd6ca58a86a690517a8e820b9b072962ca43ad60be286002` | selected at 8M; blocked at 4M (§6) |
| **B3b: the mixed exact base** (P-13, 07o) | `m3x_mix_anchor` | `3c9a6fdb912d9305d65a034af020f6bfe78933529b14f3945845d38126e7a693`; `6ca777a6e3ed658bcf58813d277e839033ac07f0b84efb8526351a20c67d1ee0` | `case` sensitive (A), `case:b` checks_passed (not_required), no exact-block selection |
| **B3a: the 0.3.0 legacy base** (07o B3a base) | `m3l` | `c32170b3c224f90cf4481eaea99e44d21ae155fbe56055a5b396aac28a6384f5`; `2f5ff465bfa97005a581d88e02c0a5478ea24da3bdc1c2eb9fcda491803fc0c3` | ordinary bytes = 0.1.0's except one diagnostic; W1 reaches precommit and G8 refuses on main |

## 9. For ROOT

1. **Item 1 answers I96 §13's open question:**
   - the exact successor's witness is `m3x`;
   - n05 and n06 are coexistence pins.

   The witness choices in I96 §5 (P-13) and §6.4 can be fixed on these inputs.
2. **B3a's expectation needs restating.** The 0.3.0 legacy ordinary bytes differ from 0.1.0's in one diagnostic message (the legacy exact-block's refusal reason and its work), not in a model echo. The model echo appears only in the successor, as `invocation.value`, together with `legacy_source_work[0].charged`. I propose B3a's pins state those exceptions exactly (§5). The producer needs no other change. On main, W1 already reaches precommit, and only the reader's G8 namespace predicate refuses.
3. **P-2 is load-bearing.**
   - `fields` changes from a qualified physics-source-1 publication (8M) to a blocked `MODEL_INCOMPLETE` envelope (4M), which removes T-3 (c)'s trigger.
   - I recommend adding `fields` to P-2's pins, beside the n05 and n06 byte pins.
   - B3b-P should also note that the receipt's `legacy_source_work[].limit` follows the same budget.
   - **Separately, an observation:** under 4M `fields`'s seed reads legacy `exact_selected` while the envelope carries no `source_block_recovery`. Today that pairing reaches W1 (the private driver's coexistence check reads the envelope).
4. **The mixed exact base's `case:b` is zero-response.** If ROOT wants a non-degenerate `not_required` operand in both modes, none was found in three variants (one remains under the brief's limit). The `axial` and `lateral` variants qualify in dense only.
5. **The exact route's native terminal is a proxy here, not a measurement.** Main's capture refuses exact models before preparation. The exact candidate (with B3-K's `ExactENu`) and the two-case W1 are B3b-P's to establish.
6. **Host notes:**
   - the host accepted every write into NUM at this folder, and no write guard refused anything;
   - the archive excluded `P/execution` (§1);
   - the first probe run queued behind I98's job on the lock, and nothing of another job was touched.

## 10. Commands run (exact; `cd` into the stated directory first)

1. `mkdir -p S/{arch,tmp,out,logs,inputs,records} && cd NUM && GIT_OPTIONAL_LOCKS=0 git archive 0b6c5d7362ff21104e078d8168f62e3f38928cf9 -- . ':(exclude)projects/chirality-piping/execution' | tar -x -C S/arch/`
2. `python3 gen_inputs.py ARCH/projects/chirality-piping S/inputs` (items 1–4), then, after adding `lateral`, `… item2`.
3. `cp zz_i99_probe.rs ARCH/P/core/product_physics/src/ && python3 apply_instrumentation.py ARCH/P/core/product_physics/src` (`instrumentation.diff` is `diff -u` of the three files against their pre-edit copies).
4. In `ARCH/P/core/product_physics`: `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=S/tmp CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=WT/targets/i99-b3w WT/tools/t3_cargo.sh test --locked --offline --lib --no-run` (rc 0; `build_pp_lib.log`).
5. `run_probe.sh run1`, then `run_probe.sh run2` and `run_probe.sh run3`. Each waits while more than one `lockf` runs, then runs in that directory, with the same environment plus `RUST_TEST_THREADS=1 I99_OUT=S/out/<run> I99_FILES=<the inputs>`: `WT/tools/t3_cargo.sh test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact retained_memory::zz_i99_probe::zz_i99_dump_builtin retained_memory::zz_i99_probe::zz_i99_controls retained_memory::zz_i99_probe::zz_i99_files` (rc 0 each; 3 passed).
6. Read-only Python (the host's `python3`): `sanitize.py`, `compare_runs.py`, `compare_i81_i86.py`, `summarize.py`, `b3a_diff.py`, `budget_diff.py` and `compare_committed_raw.py`, whose outputs are in `_run_records/`.

The lock lines for my four cargo jobs are in `_run_records/cargo_jobs_i99.log`.

## 11. Records, cleanup and limits

- **`_run_records/`:**
  - **probe sources:** `zz_i99_probe.rs`, `instrumentation.diff`, `apply_instrumentation.py`, `gen_inputs.py` and `run_probe.sh` (it takes `WT` from the environment);
  - **the inputs exactly as run:** `inputs/` (11 files) and `inputs.sha256`;
  - **logs:** `build_pp_lib.log`, `probe_run1.log`, `probe_run2.log`, `probe_run3.log` and `cargo_jobs_i99.log`;
  - **tools and their outputs:** `sanitize.py`; `compare_runs.py` with `compare_runs.log`; `compare_i81_i86.py` with `compare_i81_i86.log`; `summarize.py` with `summary.md` (every input and mode, from run 2); `b3a_diff.py` with `b3a_diff.log`; `budget_diff.py` with `budget_diff.log`; `compare_committed_raw.py` with `compare_committed_raw.log`;
  - `test_binary.sha256`, and `probe_outputs.sha256` (run 2's 77 written outputs, which stay in `S/out/` outside NUM).

  Machine paths are replaced by `WT`, `ARCH` and `~`, and the host name by `<host>`. In the logs, every line over 4,000 bytes that is not an `I99_` line is cut to 1,000 bytes, with its full length and sha256: the producer's committed `cfg(test)` prints.
- **Cleanup on return:** the archive `ARCH` and the target `WT/targets/i99-b3w/` are deleted. The archive is reproducible from `0b6c5d7362` plus `_run_records`. `S/out/` and `S/logs/` keep the unsanitized outputs.
- **Limits:**
  - **No exact input reaches W1's native stage on main;** §3's native terminal is a proxy through the 0.1.0 twin, with identical native inputs argued from the code and the published evidence.
  - **Two-case runs stop at `Domain` on main** (D1.4's one case). Item 2's base is established by its verdicts and selections only.
  - **`fields` was used for item 4 only.** It is a committed fixture outside the brief's named inputs.
  - **The dev/test build only, with the probe's prints present.** These runs establish outcomes, not stack margins or timings.
