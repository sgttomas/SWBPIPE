# I9 return: the skew M03 pin (tests only)

**Status: complete, and green. No stop rule triggered.** The candidate is the working tree of `codex/piping-m03-skew-pin-20260928` in `<wt>/skewpin`, on main `eb52114e9`, with two test files changed and these records added. I9 made no Git writes; ROOT commits.

- **Kernel pin:** RV7's table is reproduced and pinned case by case, with every refusal `Range("arithmetic outside normal range")`. The exact errors of 2EI/L and 4EI/L match RV7's figures, and 6EI/L² is shown to be the limiting coefficient in RV7's cases. Below them a second limit appears, a product underflow inside the bound (§3.4).
- **Representation parity:** today's dense M03 and K1's pattern path give identical outcomes, byte for byte in `Debug`, on all 18 case–member pairs in two orders. The adapter's two evidences are also identical in both modes on the 8 accepted pairs (§4).
- **Guard:** a test fails if M03 starts refusing the accepted skew rows or accepting an axis-aligned row. Its doc comment says the rows are today's documented limitation, not desired behaviour (§5).
- **Product evidence:** 864 product runs on archives of `134eefc24` and `eb52114e9`. **Pre-K2a main never publishes a trusted value**: 0 runs are `checks_passed`. With G = E/2.6, M03 refuses every member. In a supplementary G sweep, main publishes 52 **Sensitive** results, with errors up to 1.9e-4. Current main refuses every run at formation, by name (§6).
- **Mutations:** a NONE control, the three required mutants, one of my own and three more. All seven mutants are killed at behavioural assertions (§7).
- **Suites:** frame_kernel, sparse_direct and nonlinear_integration against my Mac baseline of `eb52114e9`: 0 changed, 0 removed, 6 added, all passing. **T9 (Mac-only):** 112 of 112 outputs byte-identical (§8).

Records use `<wt>`, `<home>` and `<VENV>` placeholders; there are no machine paths.

## 1. Brief, basis and delegation

- **Brief:** `TASK_BRIEFS/I9_M03_SKEW_PIN.md`, read from the numerics worktree (sha256 `d96ad9bb…`; uncommitted at dispatch). Also:
  - `_COMMON.md` (`6bb845bf…`);
  - "The Mac host" in `I8R_K1_RESUME.md` (`090844da…`), which overrides `_COMMON.md`'s host section;
  - ROOT's dispatch message, with its placeholders and host rules.
- **Delegation:** I9 is a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN). It ran as a background subagent of ROOT's session on the owner's Mac. The return path is ROOT, and I9 did not delegate.
- **Read:**
  - Root `AGENTS.md` and `agents/AGENT_TASK.md`;
  - `REVIEW/K2A_REVIEW.md` (`2c54cd98…`): B1, §4 and §8;
  - `IMPLEMENTATION/K2A/RETURN_ADDENDUM_1.md` (`b696e806…`): §1 to §1.4;
  - RV7's `REVIEW/_run_records/k2a/`: `README.txt`, `rv7_rotated_m03.rs.txt` (`2931c37a…`), `rv7_skew_runtime.rs.txt`, `rv7_skew_exact_ref.py.txt` and `rv7_transform_roundoff_replica.py.txt` with their outputs, and `logs/fk_base3.log`;
  - `IMPLEMENTATION/K1/RETURN.md` (`4c78bd84…`): §1–§3, §7–§10, §12 (the F1b interface) and §16, and the start of its CHANGE_RECORD;
  - K1's combined suites and T9 scripts;
  - `.agents/skills/chirality-change/SKILL.md`.
- **Source read at `eb52114e9`:**
  - FK `local_stiffness` (`lib.rs:711`), `transform_global_stiffness`, the orientation, and `normalize`;
  - `structural.rs`: `checked_*`, `gamma` and `transform_roundoff` (`:2084-2118`);
  - `sparse.rs`'s public API;
  - SA's `EvidenceParts::new` and both evidences' solve entries;
  - NI's `k1_tests.rs` helpers;
  - PP `derive_pipe_section` (`lib.rs:8479-8519`);
  - main's pre-K2a `local_stiffness` at `134eefc24` (`lib.rs:699-744`).
- **`transform_roundoff` is byte-identical at `134eefc24` and `eb52114e9`.** K2a and K1 did not change M03's element-entry floor.
- **Platform:** `aarch64-apple-darwin`, rustc and cargo 1.97.1, and stable rustfmt for formatting only (`_run_records/toolchain.txt`). The host rules were followed as §10 records.

## 2. Files

| File | Change | Lines | sha256 |
|---|---|---|---|
| `P/core/solver/frame_kernel/tests/m03_skew_scope.rs` | new | 947 | `824fc637c995672e…` |
| `P/core/solver/nonlinear_integration/src/structural_adapter/k1_tests.rs` | +150, appended; nothing else changed | 1559 | `be78e9788fde45c3…` (base `ccd97e94…`) |
| `T3/IMPLEMENTATION/M03_SKEW_PIN/` | new records | – | `SHA256SUMS` |

There is no product source, fixture, reference, Cargo or lockfile change. Both test files are rustfmt-clean with stable rustfmt, and the S11 and S11-F site tests read neither file.

**Suggested commits:** the two test files as "I9: the skew M03 pin (tests only)", then the records.

## 3. The kernel pin (`m03_skew_scope.rs`)

### 3.1 How it forms the cases

- **Inputs** (invented; RV7's confirmed cases):
  - L = 2^-39 m; pipe OD 1e-11 m and wall 1e-12 m, with the section formed as PP `derive_pipe_section` forms it; G = 1e-100 Pa;
  - E = (1/(12·I))·2^-1000·2^(t+1000) for t = −1030, −1040, −1045, −1050 and −1055;
  - S6a: E = (2.5/(12·I))·2^-1000·2^-75.
- **The local matrix** is main's pre-K2a one, formed operation for operation (`134eefc24`, `lib.rs:716-741`). It uses FK's own orientation, `transform_global_stiffness` and `transform_roundoff`.
- **Preconditions** (`m03_skew_pin_preconditions_hold`):
  - The replica equals FK's own formation bit for bit on a normal section, on all three members.
  - Every member has length exactly 2^-39 m.
  - K2a refuses every case at formation, by name (`12EIy/L^3: (12*E)*Iy`).
  - The floor is 2^-974.584963 (|log2 + 974.585| < 1e-3), and it is the axis member's floor behaviourally. A single coefficient at 4EIz/L's diagonal is accepted at floor·(1 + 2^-30) and refused at floor·(1 − 2^-30).

### 3.2 Outcomes (every refusal is `Range("arithmetic outside normal range")`)

| Case | E (Pa) | 6EI/L² | 4EI/L | 2EI/L | Axis x, yref +y | (1,1,1), yref +z | (1,2,2), yref +x |
|---|---|---|---|---|---|---|---|
| (12E)I = 2^-1030 | 2.4992e-266 | 2^-953.00 | 2^-992.58 | 2^-993.58 | refused | **accepted** | **accepted** |
| 2^-1040 | 2.4407e-269 | 2^-963.00 | 2^-1002.58 | 2^-1003.58 | refused | **accepted** | **accepted** |
| 2^-1045 | 7.6271e-271 | 2^-968.00 | 2^-1007.58 | 2^-1008.58 | refused | **accepted** | **accepted** |
| 2^-1050 | 2.3835e-272 | 2^-973.00 | 2^-1012.58 | 2^-1013.58 | refused | **accepted** | **accepted** |
| 2^-1055 | 7.4483e-274 | 2^-978.00 | 2^-1017.58 | 2^-1018.58 | refused | refused | refused |
| S6a | 1.7758e-279 | 2^-996.00 | 0 | 0 | refused | refused | refused |

- **On the accepted rows** the test asserts that:
  - all four (kE)·I are subnormal and nonzero;
  - 4EI/L and 2EI/L are normal but below the floor;
  - 6EI/L² is above the floor.
- **On the refused rows,** 6EI/L² is below the floor.
- **On the accepted skew members,** the least bound equals the least coupling-block bound (rows UX..UZ, columns RX..RZ), bit for bit, and it matches RV7's figure to 0.005 binade:
  - (1,1,1): 2^-1001.21, 2^-1011.21, 2^-1016.21 and 2^-1021.21;
  - (1,2,2): 2^-1002.00, 2^-1012.00, 2^-1017.00 and 2^-1022.00.
- **The least rotational-block bound** is 2^-492.47 on (1,1,1) and 2^-494.05 on (1,2,2), lifted by GJ/L, as RV7 found.

### 3.3 Exact-reference errors

The test computes each error exactly, in integers. L is a power of two here, so k·E·I/Lⁿ is an integer times a power of two. Only the final quotient is formed in binary64.

`_run_records/exact_reference.py.txt` is an independent Fraction check. It recomputes the inputs, confirms the E bits equal the Rust test's printed bits, and agrees to all printed digits.

| (12E)I | 2EI/L exact | RV7 | 4EI/L exact | RV7 |
|---|---|---|---|---|
| 2^-1030 | 1.137801e-13 | 1.1e-13 | 5.675013e-14 | 5.7e-14 |
| 2^-1040 | 1.164154e-10 | 1.16e-10 | 5.820757e-11 | 5.8e-11 |
| 2^-1045 | 3.725290e-9 | 3.73e-9 | 1.862645e-9 | 1.86e-9 |
| 2^-1050 | 1.192093e-7 | 1.19e-7 | 5.960464e-8 | 5.96e-8 |

- **Assertions:** each figure equals RV7's to half a unit in RV7's last stated digit, and the Fraction value to five digits.
- **12EI/L³ and 6EI/L²** are accurate by construction (exact relative error 9.3e-17). E was chosen so that (12E)·I and (6E)·I round to powers of two. RV7's 2.22e-16 for these came from its binary64 reference.
- **S6a is not exactly 2.5·2^-1075.** The brief and RV7 call (12E)·I = 2.5·2^-1075 exact, but the exact product of the binary64 operands is 2.5·2^-1075·(1 − 3.8e-17). It still rounds as RV7 states: (12E)·I = (6E)·I = 2^-1074, and (4E)·I = (2E)·I = 0. For the 2^t rows, the exact product is 2^t·(1 − 1.85e-16).

### 3.4 6EI/L² is the limiting coefficient, and a second limit below RV7's cases

`m03_skew_pin_6eil2_is_the_limiting_coefficient` moves only the 6EI/L² coefficients across the floor, by a factor of 8, and leaves 4EI/L and 2EI/L as formed, below the floor.
- **Lowering 6EI/L²** refuses every accepted skew row with `Range`.
- **Raising 6EI/L²** turns the refused skew cases into acceptances: 2^-1055 on (1,1,1), and S6a on both members.
- **The exception is 2^-1055 on (1,2,2).** It stays refused, with **`Range("product overflow or underflow")`**. This is a second limit, not in RV7's table:
  - `transform_roundoff` checks each product |T_ki|·magnitude_kj of its inherited sum.
  - On (1,2,2) the least nonzero |T| is 0.2357. The bound entry (RY_i, RY_j) takes the term 0.2357 × (2EI/L·0.2357) = 2^-1022.75, which underflows. The replica `_run_records/development/counterfactual_diagnosis` locates it.
  - On (1,1,1) the least |T| is 0.4082, the term is 2^-1021.17, and the member is accepted.
  - In RV7's own rows the coupling bound refuses first, with `Range`.
- **On the axis member**, the outcome is accepted exactly when every nonzero coefficient is at or above the floor. S6a with a raised 6EI/L² is accepted there.
- **Consequence for later slices:** M03's skew acceptance of subnormal-derived 1/L coefficients ends either at the 6EI/L² floor or where 2EI/L·|T|² falls below 2^-1022, whichever comes first. The test documents this, and M-UNCHECKED-PRODUCTS (§7) pins it.

The first draft of this test used a factor of 2. It failed on this very case, which is how the second limit was found (`_run_records/development/fk_m03_run1_counterfactual_failed.log`).

## 4. The same outcomes through both representations

### 4.1 FK level (`m03_skew_pin_dense_and_pattern_outcomes_are_identical`)

- **Today's dense M03:**
  - `transform_roundoff` gives the element's evidence;
  - the dense store is accumulated with `+=`;
  - the allowances use the adapter's arithmetic: the bound plus gamma(1)·|value|, with count 48 + 1;
  - the system has contributions, a global RZ moment of 0.1·EI/L at N1, N0 prescribed and N1 free;
  - it runs `prepare_structural`, then `factor_structural_profile` and `finish_structural`.
- **K1's pattern path:**
  - the same evidence;
  - `assemble_sparse_stiffness` with the realized matrix as a `StiffnessBlock`;
  - one allowance per pattern entry, by the same arithmetic;
  - `SparseStructuralSystem::new`, `prepare_sparse_structural`, `factor_sparse_structural_profile` and `finish_sparse_structural`.
- **Result:** the outcomes are identical in `Debug` bytes for all 18 pairs, in natural and in reversed order.
  - The refused pairs are `Err(Range("arithmetic outside normal range"))` in both. This is the same by construction, because both evidences are formed by the one `transform_roundoff` call, as the adapter's `EvidenceParts::new` forms them for either store. The test's doc comment says so.
  - The 16 accepted runs reach the gate, and both representations give `NumericallyUnresolved { "nonpositive or cancellation-unresolved structural pivot", global_dof: Some(10) }`.
  - The product's dense mode, dense Cholesky, is also recorded: it gives the same floor outcome and the same pivot refusal at DOF 10.

### 4.2 Adapter level (`k1_tests.rs`, appended)

- **Test:** `i9_skew_m03_accepted_rows_are_identical_through_both_evidences_in_both_modes`.
- **Setup:** main's pre-K2a matrix is carried as an explicit slot. It is a `CurvedBendStiffnessElement` with its own `transform_roundoff` bounds and counts, so both evidences record exactly a frame's bounds, counts and scatter allowance.
- **Result:** `AssemblyEvidence` and `SparseAssemblyEvidence` give identical outcomes in both modes, on the 8 accepted pairs:
  - DenseScrutiny, pivot DOF 10;
  - SparseInteractive, pivot DOF 11.
  - These are exactly RV7's product observations on main (`pp_main_skew3.log`).
- **Limit:**
  - Post-K2a, SA's frame route cannot carry these matrices, because `local_stiffness` refuses them.
  - The slot differs from a frame, in both evidences alike, only in two ways: it is unqualified for the rigid-body screen, and it changes the report's symmetry-basis text.
  - Refused pairs cannot reach either evidence (§4.1).

The pattern path carried a pre-K2a matrix with **no product change** (FK: `StiffnessBlock`; SA: an explicit slot), so the brief's stop condition did not arise.

## 5. The behavioural guard

- **Test:** `m03_skew_scope_guard_documented_limitation_not_desired_behaviour`.
- **What it asserts:** the list of accepted pairs is exactly the 8 skew pairs of the 2^-1030…2^-1050 rows, and every other pair is refused with `Range("arithmetic outside normal range")`. So it fails if M03 starts refusing those skew rows, starts accepting any axis-aligned row, or changes a refusal's text.
- **Doc comment:** the accepted rows are pinned as today's **documented limitation, not desired behaviour**. It cites K2a's `RETURN_ADDENDUM_1.md` §1.4, notes that K2a refuses these formations first, and names the slices that lean on M03 (K2b, K5 and F1b).
- **Demonstration:** my own mutant M-CLOSE-GAP (M03 refuses every sub-floor coefficient on every orientation, a plausible future "fix") is killed by the guard (§7).

## 6. Product evidence (a run, not a test)

### 6.1 Model and method

- **Model:** one member N0→N1, with N0 anchored (UX…RZ) and N1 free. Invented inputs throughout:
  - L = 2^-39 m on the three members of §3;
  - pipe OD 1e-11 m and wall 1e-12 m;
  - E with (12E)·I rounding to 2^t, for t = −1030, −1040, −1045 and −1050, so (4E)·I and (2E)·I are subnormal and 4EI/L and 2EI/L are subnormal-derived;
  - a global RZ moment of 0.1·EI/L at N1 (RV7's load).
- **G values:**
  - **the primary, "G comparable to E": G = E/2.6** (an isotropic ν = 0.3);
  - **a supplementary sweep:** G = E·2^k for k = 20, 30, 36, 40, 48, 56 and 64, plus RV7's G = 1e-100 Pa. The primary model turns out to be refused by M03 outright (§6.2), so the sweep maps where main's standing changes.
- **Runs:**
  - the probe `_run_records/product/i9_skew_product_probe.rs.txt`, placed only in scratch archive copies;
  - both entries (the captured value and typed), both modes;
  - on a `git archive` of **`134eefc24`** (pre-K2a) and of **`eb52114e9`**: 432 runs each.
- **Exact reference** (`product_reference.py.txt`):
  - It takes the represented inputs exactly: E, G and the moment; A, I and J as PP forms them; FK's length and axes. All are printed as bits by the probe.
  - It solves T^T K T and the free node in Fractions.
  - The criterion is the protected relative form |obs − exp| ≤ 1e-9·max(|exp|, scale), with scale = the largest |exp| of the same kind in the run (translations in mm, or rotations in rad).
  - The script also reports the error of an exact solve of main's formed coefficients: the formation error alone.
- **Records:** every run's standing, quality, published N1 values, exact values and errors are in `product/product_runs.jsonl`. The table is in `product/product_table.txt`.

### 6.2 Standing (each cell: axis / (1,1,1) / (1,2,2); all four runs of a cell agree)

**Pre-K2a main `134eefc24`:**

| t \ G | E/2.6 | E·2^20 | E·2^30 | E·2^36 | E·2^40 | E·2^48, 2^56, 2^64, 1e-100 |
|---|---|---|---|---|---|---|
| −1030 | R R R | R R R | R S(6e-8) S(1e-7) | R S(1e-10) S(8e-6) | R S(6e-5) S(1e-4) | R P P |
| −1040 | R R R | R R R | R R R | R S(1e-10) S(8e-6) | R S(6e-5) S(1e-4) | R P P |
| −1045 | R R R | R R R | R R R | R S(2e-6) R | R S(6e-5) S(2e-4) | R P P |
| −1050 | R R R | R R R | R R R | R R R | R R R | R P P |

Key:
- **R:** M03 `Range("arithmetic outside normal range")`; `unresolved`, and nothing is published.
- **P:** `NumericallyUnresolved` at a pivot; `unresolved`, and nothing is published.
- **S(e):** values are published with status **`sensitive`** (case: `passive_model_basis` / `sensitive` / `not_claimed`), and e is the largest criterion error in the cell.

**Current main `eb52114e9`:** every run is refused at formation by name, as `SOLVER_SYSTEM_BLOCKED`, with status `not_assessed`, and nothing is published. The site is `GJ/L: G*J` where G·J is subnormal (G = E/2.6 at every t; also E·2^20 at t ≤ −1040, and E·2^30 at t = −1050), and `12EIy/L^3: (12*E)*Iy` otherwise.

### 6.3 Findings

1. **The stop rule was not triggered.** No run on either tree has status `checks_passed`: 0 of 864. result_export's `numerically_eligible` requires `checks_passed`, so no value is trusted.
2. **The primary model (G = E/2.6) is refused by M03 on every member,** skew members included. With G comparable to E, GJ/L is itself subnormal-derived and below the floor, so nothing lifts the rotational block's bounds. Main publishes nothing.
3. **Main's downstream standing on skew members is not "refused" in general.** In the sweep, where GJ/L lifts the rotational block (G/E from 2^30 to 2^40), main publishes values in 52 runs, all **Sensitive**. Their criterion errors run from 7.3e-12 to 1.87e-4, and 42 of the 52 exceed 1e-9.
   - The formation error alone (the exact solve of main's formed coefficients) is 2.3e-13, 2.3e-10 and **7.45e-9** at t = −1030, −1040 and −1045, which is 4× the 4EI/L error of §3.3.
   - The rest comes from the binary64 solve of a rotational block with GJ/L ≫ bending.
   - At larger G the pivot screen refuses (P), as in RV7's probe.
4. **K2a's product-reach statement stands.** Main never trusted a wrong value here.
   - Addendum 1 §1.3 recorded, from RV7's G = 1e-100 probe, that main's pivot screen refused these skew cases, and left it open whether that holds in general. The answer is now that it does not: in the intermediate-G sweep, main publishes the values as Sensitive rather than refusing them.
   - K2a now refuses all of these at formation, so its benefit on skew members includes withholding these Sensitive publications. That wording is ROOT's to adopt; I9 edits no K2a record.

## 7. Mutations (`_run_records/mutations/`)

- **Method:**
  - Each mutant got a fresh copy of the candidate archive tree and its own target, under `<wt>/skewpin-mut/<id>/`. Both were deleted afterwards.
  - The NONE control ran first, alone. The rest ran three at a time, at `-j 4`.
  - Each ran FK `--test m03_skew_scope` and NI `--lib i9_`, both `--no-fail-fast`.
  - The patches are in `mutate_i9.py.txt`; every anchor matches exactly once. The kill sites are in `kill_sites.txt`.
- **Control:** NONE passes 5/5 (FK) and 1/1 (NI).

| Mutant | Change (FK `transform_roundoff`) | Killed at (`m03_skew_scope.rs` unless stated) |
|---|---|---|
| **M-AXIS-BOUND** (required) | Each entry is bounded by the axis-aligned per-entry bound 2g·\|c\| of the local coefficient at its position. This is today's bound exactly on an axis-x member | pin `:562` (2^-1030 on (1,1,1) refused, against RV7's table); parity `:869`; counterfactual `:637`; guard `:931`; NI `k1_tests.rs:1525` |
| **M-FLOOR-UP** (required) | The floor is raised sixteenfold, to about 2^-970.585 | preconditions `:444` (the floor check); counterfactual `:637`; pin `:562` (2^-1050 refused); guard `:931`; parity `:869`; NI `:1525` |
| **M-NO-COUPLING** (required) | The coupling block is dropped from the bound (no bound and no check) | counterfactual `:637` (lowered 6EI/L² accepted); pin `:551` (the least bound is no longer the coupling block's); guard `:921` (2^-1055 on (1,2,2) now refused as a product underflow); parity `:869` (2^-1055 on (1,1,1) passes the floor). NI survives, being a parity test |
| **M-CLOSE-GAP** (my own) | M03 refuses every sub-floor local coefficient on every orientation: a plausible future "fix" | pin `:562`; parity `:869`; counterfactual `:637`; **guard `:931`**; NI `:1525` |
| M-UNCHECKED-PRODUCTS (extra) | The bound's inherited and second products are unchecked: §3.4's second limit is removed | counterfactual `:637` only (2^-1055 on (1,2,2) accepted) |
| M-AXIS-BOUND-GLOBAL (extra) | Each entry is bounded by its transformed entry's own magnitude, 2g·\|K_global(i,j)\| | counterfactual `:637` only |
| M-FLOOR-DOWN (extra) | The floor is lowered: a subnormal bound down to 2^-1040 is accepted | all 5 FK tests (`:444`, `:562`, `:637`, `:869`, `:921`) |

- **Every kill is at a behavioural assertion.**
  - Pin `:551` is a figure assertion: which block holds the least bound.
  - The others are outcome assertions: acceptance, refusal and the refusal text.
- **M-AXIS-BOUND-GLOBAL, stated as found:** a per-entry bound computed from the transformed entry leaves every outcome in RV7's table unchanged. No global entry there is a cancellation residue, and at the two decimals checked it does not move the least bounds either. Only the counterfactual kills it: 2^-1055 on (1,1,1) with a raised 6EI/L² is refused. It is recorded as found, not claimed as a kill by the pin.

## 8. Suites and T9

- **Suites** (`_run_records/suites/`):
  - frame_kernel, sparse_direct and nonlinear_integration in full, `--no-fail-fast`, one manifest at a time, `CARGO_BUILD_JOBS=8` and `RUST_TEST_THREADS=4`;
  - on `git archive` trees of `eb52114e9` (base, my Mac baseline) and of the candidate.

| Manifest | Base `eb52114e9` | Candidate |
|---|---|---|
| core/solver/frame_kernel | 179 / 0 / 0 | **184** / 0 / 0 |
| core/solver/sparse_direct | 30 / 0 / 0 | 30 / 0 / 0 |
| core/solver/nonlinear_integration | 101 / 0 / 0 | **102** / 0 / 0 |

- **Per test** (`suites_compare.txt`): 0 changed, 0 removed, 6 added, all passing. The three crates had no Mac platform failure on base.
- **T9, Mac-only** (`_run_records/t9/`):
  - **Harness:** S11-K's `fixdiff_main.rs` (`ec089c1d…`), with ROOT's harness lock, `--release`.
  - **Inputs:** every committed JSON request or model under P/core, P/fixtures and P/validation, in both modes.
  - **Result:** **112 of 112 byte-identical**, base against candidate, and the raw outputs are identical, as expected with no product change.
  - **Cross-check:** the base equals K1's combined T9 candidate and ROOT's Mac main hashes on 112 of 112.
  - This is a Mac-only comparison; no Mac output is compared with a Linux record.
- **Final bytes:** after the last comment edits to both test files, the candidate suites, T9's candidate and all eight mutants were re-run on the final bytes. The records are from those runs.

## 9. What was not done, and open items

- **Not run:** the other 36 manifests of CI's profile, including PP, whose sources and tests are untouched here; hosted CI; the DEC-025 sweep; independent review; GEN-8 after the commit (§10). The brief gives these to ROOT.
- **Not probed:** a subnormal-derived 12EI/L³ dominated by EA/L in the translational block of a skew member. Addendum 1 §1.3 lists it as "not established", and it is outside this brief.
- **Adapter test uses an explicit slot:** the SA-level pin cannot run SA's frame route post-K2a, so it uses an explicit slot (§4.2).
- **No record edits:** no K2a record, ruling or work graph is edited. Two items are for ROOT:
  - §6.3 item 4, main's Sensitive publications;
  - §3.4, the second limit, relevant to K2b's scaling and to F1b.
- **Exploratory scripts:** standard-library Python replicas used during development are not recorded, except `development/counterfactual_diagnosis`. The Rust tests are the evidence.
- **Scratch kept for verification:** the scratch trees under `<wt>/scratch/i9/trees/` are kept for ROOT's verification. The build targets are pruned (§10).

## 10. Records and host

- **`_run_records/`** is built by `assemble_run_records.py.txt` from I9's scratch directory (see its `README.txt`). It contains:
  - `toolchain.txt` and `trees.txt`;
  - the final FK and NI test logs, with every figure;
  - `exact_reference.*`;
  - `development/`;
  - `product/`, `mutations/`, `suites/` and `t9/`.
- **Sanitization:**
  - Machine paths became `<wt>`, `<home>`, `<VENV>` and `<scratch>`.
  - In logs, trailing spaces and trailing blank lines were stripped. A line over 600 characters is cut, with a marker giving the characters removed and the full line's sha256.
  - The probe logs and `product_runs.jsonl` are data and are not cut.
- **Machine-path scan:**
  - GEN-8 (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, run from `<wt>/skewpin` with `<VENV>`) passed: 1 passed, 10 deselected. It scans tracked files only, so it covered the changed `k1_tests.rs` but not the new, untracked files.
  - The harness's own `MACHINE_ABS_PATH_RE` (`tools/practitioner_harness/surface_roles.py`) was therefore applied directly to every file in this folder: 61 files, 0 hits.
  - ROOT re-runs GEN-8 after committing.
- **`SHA256SUMS`** covers every file in this folder except itself.
- **Host:** the cargo, mutant, memory and target rules held throughout:
  - at most two cargo jobs at once, at `-j 8` or below;
  - mutants three at a time, at `-j 4`;
  - `RUST_TEST_THREADS=4`;
  - every model has one member;
  - no memory-guard kill (`<wt>/guard/memguard.log`);
  - targets under `<wt>/skewpin-target/`, pruned at the end.
