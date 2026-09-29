# K6 checkpoint 0: the plan (I15, before any code)

- **Brief:** `T3/TASK_BRIEFS/I15_K6_IMPLEMENTATION.md`, sha256 `7a4338f1fa2f4a2555e0d40d8f769644b8c60eee72f21da474928039a30b1794` (committed at `195f15ab5`), with ROOT's rulings on Q1–Q13 and stale-design items 1–12, also recorded in `T3/ROOT_RULINGS_V1.md`, "K6: spawn and rulings".
- **Base:** branch `codex/piping-k6-20260928` in `<wt>/k6`, at main `56dd72334`. `git diff --stat 24dea2dae 56dd72334` and `git diff --stat d1cc97ce4 56dd72334` over `P/{core,fixtures,validation,schemas,apps,tools,tests}` are both empty, so every line the brief cites is where it says. I re-located every line cited below on this base.
- **F1b** is read at its branch head `130445db2` (still the head of `origin/codex/piping-f1b-20260928`). **K5** (PR #1044, head `28517eaaa`) is not merged; `origin/main` is `56dd72334`.
- **Delegation:** I15 is a background subagent of ROOT's (HELP_HUMAN) session on the owner's Mac, dispatched directly by ROOT; the return path is ROOT. Type 2: no delegation, no Git writes.
- **ROOT's added ruling (from RV16's N4, received during this turn; binding):**
  - The ban on runs at 10,000 members or more covers **every mode that materializes an n² matrix**: `dense` and `lane-lu`.
  - The identity-order lane (`lane-id`) is covered for any model whose estimate fails the admission rule. CONT at 10,000 members stays never-run.
  - The runner refuses such runs **by name**. This plan carries the ruling in 2.2, 6.6, 8.1–8.2, 11 (G1, G3, H7) and 12 (K6-M12, K6-M21).
- **What this turn did:** reading and design only. No cargo, no build, no test, no observation run. Read-only `git`/`grep`, `ls -l` of three system binaries, `man time`, and two small read-only Python scripts: one computed closed-form counts, run before ROOT's `nice` instruction (under a second of CPU); one read the reference loads and section bits, run under `nice -n 19`. Nothing was written except this file.

Paths are repo-relative. `P/` = `projects/chirality-piping/`; `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`; `H/` = `P/core/solver/performance_harness/`; `FK/` = `P/core/solver/frame_kernel/src/`; `SD/` = `P/core/solver/sparse_direct/src/`; `NI/` = `P/core/solver/nonlinear_integration/`; `SA` = `NI/src/structural_adapter.rs`; `PP` = `P/core/product_physics/src/lib.rs`. `PP@F1b` means PP at `130445db2`.

## Contents

1. Files and layout
2. The observation binary: CLI, JSONL schema and stage map
3. The generator
4. The counts and their oracles
5. The allocator
6. The runner
7. The derived peak estimate per mode
8. The schedule, with admission under Q3
9. The DEC-025 sandbox question (Q9(b))
10. The Scope 8 scans
11. Tests
12. Mutants
13. Positions on everything still open (N1–N20)

## 1. Files and layout

| File | Change |
|---|---|
| `H/Cargo.toml` | `nonlinear_integration` path dependency (Q2(b)); `[[bin]] name = "k6_observe", path = "src/bin/k6_observe/main.rs", test = false`; `[[test]] name = "k6_alloc", harness = false` |
| `H/Cargo.lock` | gains exactly `open_pipe_stress_curved_bend`, `open_pipe_stress_nonlinear_integration` and `open_pipe_stress_nonlinear_supports` (read from `NI/Cargo.lock:5-40`; `linear_supports`, `primitive_loads` and `solver_diagnostics` are already in `H/Cargo.lock`). There are no `source` or `checksum` lines, so no registry package. The diff is listed at A1 |
| `H/src/lib.rs` | one line: `pub mod k6;` |
| new `H/src/k6/mod.rs`, `models.rs`, `canonical.rs`, `counts.rs`, `staged.rs`, `lanes.rs`, `parity.rs` | generator, canonical serialization, counts, the staged sequence, the two legacy lanes, parity checks |
| new `H/src/bin/k6_observe/main.rs`, `alloc.rs` | CLI, JSONL, refusals, and the counting capped allocator (registered only here) |
| new `H/tests/k6_models.rs`, `k6_counts.rs`, `k6_parity.rs`, `k6_staged.rs`, `k6_bin.rs`, `k6_alloc.rs` | tests A–G. `k6_alloc.rs` is a `harness = false` binary that registers `alloc.rs` (`#[path]`) as its own global allocator |
| new `H/runner/k6_runner.py`, `H/runner/test_k6_runner.py` | the runner (standard library only), including an independent Python model generator and the closed forms; its `unittest` suite |
| new `H/observations/k6/` | `models/` (the 21 small canonical models, section 3), `models_sha256.txt` (33 plus the extras, labelled), `counts.jsonl`, and at B the packet `k6_packet.json` and `SHA256SUMS` |
| `H/README.md` | a K6 section |
| new `P/tests/test_performance_harness_runner.py` | the thin pytest wrapper (Q9(b)) |
| `T3/IMPLEMENTATION/K6/**` | records (this plan, CHANGE_RECORD, RETURN, `_run_records/`, SHA256SUMS) |

**Why a `[[bin]]` and not an example** (the brief asks for a proposal):
- Integration tests reach it through `env!("CARGO_BIN_EXE_k6_observe")`, which cargo sets only for `[[bin]]` targets.
- `test = false` stops cargo from building a unit-test harness around the binary, so the capped allocator never becomes a test binary's allocator by accident.
- The library never registers an allocator, so a consumer of `H` inherits nothing.
- Hosted CI's `cargo test --offline --locked` builds the bin (debug), which the G and F4 tests need. It builds no release binary.

## 2. The observation binary

### 2.1 CLI

```
k6_observe --model <id> | --model-file <canonical file>
           --mode sparse|dense|lane-id|lane-lu
           --heap-cap-bytes <u64>
           [--repeats 5] [--entry-repeats <k>]          (default k = repeats)
           [--time-budget-s <u64>] [--first-repeat-limit-s 600]
           [--dump-solution <file>]                     (repeat 0's displacement bits, hex)
           [--allow-over-estimate]                      (accepted only with a heap cap <= 512 MiB)
k6_observe --emit-model --model <id>                    (canonical bytes to stdout; nothing else)
k6_observe --counts-only --model <id> --heap-cap-bytes <u64> [--dump-pattern <file>]
k6_observe --list-models
```

- **Model ids.**
  - R1's own 24: `RF-LARGE-{CHAIN,TREE,CONT}-n{00010,00100,01000,10000}-{AX,ROT}`.
  - The nine: `DEC053:<fixture_id>`, where `fixture_id` is the harness's id (`invented-cantilever-chain-8`, …, `invented-grid-frame-5x6`).
  - Outside the sealed set: `K6-CEIL-CHAIN-n01364-AX` (Q4) and `K6-GRID-<s>x<s>` (Q5).
- **Modes.** `sparse` and `dense` are SA's `SparseInteractive` and `DenseScrutiny`. `lane-id` is the identity-order DEC-050/053 lane; `lane-lu` is the dense LU lane (Q12).
- **Exit codes.** `0`: the run completed. An M03 refusal is an outcome, not an error. `2`: usage. `3`: refused by the binary. `4`: internal error. A heap-cap refusal aborts (SIGABRT, section 5).

### 2.2 JSONL on stdout

One object per line, keys in a fixed order, each line flushed at once. Every key is listed at A1 in `H/README.md`.

| `kind` | When | Fields |
|---|---|---|
| `start` | first line, before any allocation above the model | `schema` = `k6-observe-v1`, `pid`, `model`, `mode`, `repeats`, `entry_repeats`, `heap_cap_bytes`, `time_budget_s`, `allow_over_estimate` |
| `counts` | once, after the counts phase and before the repeats (N1) | section 4's counts, both estimates (`estimate_f1b_bytes`, `estimate_adm_bytes`), `model_canonical_len`, `phase_peak_heap` of the counts phase |
| `refusal` | instead of the repeats; exit 3 | `reason`, `estimate_adm_bytes`, `heap_cap_bytes`, `heap_peak_so_far`. The reason is one of: `n2_mode_at_or_above_10000_members` (`dense` or `lane-lu` with m ≥ 10,000; checked from the model's member count before any count or n² allocation); `cont_n10000_identity_lane` (`lane-id` on either CONT n10000 model; Q12); `estimate_exceeds_half_cap` (any mode, `lane-id` included) |
| `stage_begin` | before each stage (N1) | `repeat`, `stage`, `heap_current` |
| `stage` | after each stage | `repeat`, `stage`, `ok`, `elapsed_ns`, `heap_current_begin`, `heap_current_end`, `heap_peak`, `heap_peak_move`, `alloc_calls`, and `error` (`Debug` text) when `ok` is false |
| `outcome` | per repeat | `repeat`, `class` (`Passed`, `Sensitive`, or the `StructuralError` variant name), `failed_stage`, `formation_demoted`, `load_fidelity_flagged`, `solution_debug_len`, `solution_debug_hash` |
| `parity` | per check | `item`, `repeat`, `equal`, plus item fields (section 11 E and D) |
| `summary` | last line | `repeats_completed`, `stop_reason` (`null`, `first_repeat_over_limit` or `time_budget`), `heap_peak`, `heap_peak_move`, per-phase peaks |

- `solution_debug_hash` is a 64-bit FNV-1a of the `Debug` stream of `Result<StructuralSolution, StructuralError>`, fed through a `fmt::Write` sink, so no large string is ever built. FNV-1a is a fixed, stated function; std's `DefaultHasher` is not stable across releases.
- Exact `Debug` string equality is the tests' predicate (E). The hash is the observation run's record at sizes where the string would be tens of MB.
- The binary prints no path, host name or time of day.
- The runner computes medians and minima (section 6). The binary prints only raw per-repeat values.

### 2.3 The stage map (Q6(a): public-API boundaries, bundling disclosed)

The staged sequence mirrors SA's `solve_assembled_with_formation_check(…, selected = true)` (`SA:644-699`), the entry PP@F1b calls (`PP@F1b:5388`), in SA's own order.

| Stage | Exact calls | Bundled design stages |
|---|---|---|
| (model) | the generator; recorded, not timed as a solver stage | — |
| `assembly` | `assemble_sparse_stiffness(n_nodes, &frames, &[], &[], &[], &SparseAssemblyOptions::new())` (`FK/structural/sparse.rs:592`) | assembly |
| `evidence` | `SparseAssemblyEvidence::new(k.pattern(), n_nodes, &frames, &[], &[], &[])` (`SA:468`) | — |
| `ledger` | `LoadLedger::new()`, one `push("load:<dof>", dof, value)` per nonzero load in ascending DOF order, `finish(n)` (`FK/load_ledger.rs:126-197`) | — |
| `reduce` | `reduce_assembled_sparse_system(&k, &f, &restrained, None)` (`FK/structural/sparse.rs:729`); its `free_dofs` are the SA call's `free`, and `prescribed` = restrained DOFs at 0.0. This is the product's call at `PP@F1b:3510` | — |
| `geometry` | `k.pattern() == evidence.pattern()` (SA's private `check_pattern`, `SA:586-593`), then `evidence.geometry(&prescribed)` (`SA:540`) | — (O(N·m) in `SA:1211-1240`, the cost K5's brief sends to K6) |
| `densify` (dense only) | `k.to_dense()` (`FK/structural/sparse.rs:299`) and `evidence.dense_symmetry_view()` (`SA:573`), where SA does them, after geometry (N1) | — |
| `prepare` | the basis string (N5) and the `FormationSource` (N5); `SparseStructuralSystem::assembled(…).with_formation_source(&source)` then `prepare_formation_checked_sparse_structural` (`sparse.rs:1268`); dense: `StructuralSystem::assembled(…).with_formation_source(&source)` then `prepare_formation_checked_structural` (`FK/structural.rs:1174`) | validate, contribution **audit**, scaling |
| `factor` | sparse: `order_sparse_structural` (`SD/structural.rs:56`) then `factor_sparse_structural_profile(&prepared, &order, &first)` (`sparse.rs:1727`), which together are `factor_sparse_structural_ldlt` (`SD/structural.rs:79`); dense: `factor_structural_cholesky` (`FK/structural.rs:1865`) | factor, with the pivot screens |
| `witness` (only after a refused factor) | sparse: `sparse_negative_pair_witness(&prepared)?.unwrap_or(error)` (`sparse.rs:1921`); dense: `negative_pair_witness(&prepared)?.unwrap_or(error)` (`FK/structural.rs:2166`) | — (N1) |
| `finish` | `finish_sparse_structural` (`sparse.rs:1784`) / `finish_structural` (`FK/structural.rs:1883`) | **rcond**, **solve**, **residual**, intended-action audit, load fidelity, the K-D5 formation check |
| `recovery` | `k.reactions(&u, &f)` (`sparse.rs:360`, in both modes, as `PP@F1b` does) and each frame's `force_scaled_end_actions(&u, ForceScale::UNSCALED)` (`FK/lib.rs:991`) | **recovery** |
| `entry_checked` | `evidence.solve_assembled_with_formation_check(&k, &f, &free, &prescribed, mode, &[], true)` (`SA:644`) | the product's whole call |
| `entry_plain` | `evidence.solve_assembled(&k, &f, &free, &prescribed, mode)` (`SA:596`) | ditto, without the formation check |

- **Scoping.** Everything from `densify` to `finish` lives in one scope, dropped before `recovery`, so only the `StructuralSolution` (O(n)) survives. The `entry_*` stages reuse `k`, `evidence` and `f`, as the product does, and allocate their own views. So no stage's peak includes another stage's n² buffers.
- **Mapping to SA.** `unscaled_evidence` (`SA:1359-1369`) is trivially met by `new`. The dense branch's `solve_prepared` (`SA:1761-1778`, private) is replicated from its public calls: `factor_structural_cholesky`, then `negative_pair_witness` on a refusal, then `finish_structural`.
- **The equality test.** The staged result must be bit-identical in `Debug` to `entry_checked`'s, and the plain variant to `entry_plain`'s (tests E). The observation run records it at every size (`parity` item `staged_vs_entry_checked`). A mismatch is a stop.
- **The lane stages.**
  - `lane-id`: `assembly`; `lane_entries` (K6's copy of PP's private builder, N19); `lane_solve` = `solve_symmetric_system_from_entries(dim, &entries, &force)` (`SD/lib.rs:700`).
  - `lane-lu`: `assembly`; `ledger`; `densify_lu` = `k.to_dense()`; `lane_reduce` = `reduce_assembled_system(&dense, &f, &restrained)` (`FK/lib.rs:1426`), then `drop` of the dense view, as at `PP@F1b:3849`; `lane_solve` = `solve_dense(&reduced.stiffness, reduced.force.values())` (`FK/lib.rs:1631`), as `legacy_dense_observation` (`PP@F1b:3285-3289`).
  - Each lane mode records the lane's own counts: `original_profile_entry_count` and the rest.

## 3. The generator (Q10(a))

### 3.1 Families, in R1's order (read from `T3/REFERENCES/references.py`, sha256 `80d473a7…`)

The generator ports R1's closed forms, with each node and member in R1's insertion order:
- **CHAIN** (`lg_chain_defn`, `:1747-1755`): nodes `N0..Nn` at (3i, 0, 0); members `M1..Mn` = (N_{i-1}, N_i); N0 fixed in six DOFs; one load at `Nn`: F = 2^s·(6, −3, 9), M = 2^{s2}·(3, 6, −3).
- **TREE** (`lg_tree_defn`, `:1758-1776`): nodes P0, then P1, B1, P2, B2, … interleaved; members S_k = (P_{k−1}, P_k), then Q_k = (P_k, B_k), interleaved.
  - B_k is at +3 in y when k is odd and +3 in z when k is even.
  - The load at B_k is 2^{sc}·3·((k mod 7) − 3) along z (k odd) or along y (k even); a zero load is omitted.
  - P0 is fixed.
- **CONT** (`continuous_defn`/`lg_cont_defn`, `:1779-1808`): nodes S0..Ss, **then** C1..Cs; members A_j = (S_{j−1}, C_j) and B_j = (C_j, S_j), interleaved.
  - S0 is fixed in six DOFs; S1..Ss are pinned in UX, UY, UZ.
  - The load at C_j is F = Q·(0, 96((j mod 5) − 2), 96((j mod 3) − 1)), omitted when zero.
- **ROT:** every coordinate and load vector is multiplied by Q3 = (1/3)[[1,2,2],[2,1,−2],[−2,2,−1]] (`:1298`), in **exact integer arithmetic**. All coordinates are multiples of 3 and every load component is divisible by 3 (derived for each family below), so each ROT value is an exact integer times a power of two. Restraint sets are rotation-invariant triples (`rot_defn`, `:1701-1717`).
  - CHAIN: Q3·(6, −3, 9) = (6, −3, −9) and Q3·(3, 6, −3) = (3, 6, 3).
  - TREE: Q3·(0, 0, 3c) = c·(2, −2, −1) and Q3·(0, 3c, 0) = c·(2, 1, 2).
  - CONT: loads are multiples of 96, and 96 is divisible by 3.
- **Exponents.** They are tabulated per n, with their source, and never computed with `log2`.
  - CHAIN (s, s2) = (−6, 1), (−13, −2), (−20, −5), (−26, −9) for n = 10, 100, 1,000, 10,000. They equal R1's `round(math.log2(1.264/n²))` and `round(math.log2(24.5/n))`, re-derived by exact rational comparison, 2^{2k−1} ≤ x² < 2^{2k+1}. They match the tip loads in `references.json`'s purpose strings.
  - TREE sc = 0, −6, −14, −20. These depend on R1's exact solution (`:1889`: `round(log2(1e-4/max_rotation))`), so they are data from `references.json`'s purpose strings, cross-checked by the recorded comparison.
  - The Q4 chain (n = 1,364) uses the same exact rule: (s, s2) = (−20, −6). It is outside the sealed set.
- **The nine** are the harness's own fixtures, reached from the new child module through the crate-private `sparse_default_promotion_fixture_specs()` (`H/src/lib.rs:704-752`). A child module may read its ancestor's private items, so `lib.rs` changes by one line and the nine are exactly DEC-053's. Their section, `y_reference`, force and restraints come unchanged from `invented_cantilever_chain_fixture`/`invented_grid_frame_fixture` (`:347-472`).
- **Q5 grids** are `invented_grid_frame_fixture(s, s)`, outside the sealed set.
- **`y_reference` for RF-LARGE (N4):** P1's rule (`T3/DETECTION/scripts/gen.py.txt`, `y_reference`): the global axis least aligned with the member, preferring Y, then Z, then X on ties. It is computed from the exact integer coordinate differences. This is what P1's product requests carry, so the kernel frames equal the product's frames except for the section bits.

### 3.2 The section formula (Q10; no `powi`)

- od = 0.2 and id = 0.18, each rounded once from R1's decimal. E = 2e11 and G = 8e10 are exact.
- `od2 = od*od`, `id2 = id*id`, then:
  - `A = PI*(od2 − id2)/4.0`;
  - `I = PI*(od2*od2 − id2*id2)/64.0`;
  - `J = 2.0*I`, with Iy = Iz = I.
- `PI` is `std::f64::consts::PI` (bits `400921fb54442d18`). Evaluation is left to right as written, with + − × ÷ only.
- These operations are correctly rounded whether LLVM folds them at compile time or not, which is the lesson of "K3: Q7 reversed": no function of unspecified precision.
- The bits, computed now by the same formula with Python's `math.pi` (identical bits), are pinned by test B4:
  - A = `3f7872fa3a37ac1b`;
  - I = `3efc526644422115`;
  - J = `3f0c526644422115`.
- **Disclosed.** A is 9 ulp from the correctly rounded exact value 0.0019π (`3f7872fa3a37ac12`), because the formula runs on the rounded od and id. PP forms A, I and J from od and the wall with `powi` (`PP:8515-8519`). So kernel outcomes compare with product outcomes as context only, as the brief says.

### 3.3 Canonical serialization (`k6-model v1`)

ASCII, LF, no trailing space, one final newline. Every float is 16 lower-case hex digits of its bits.

```
k6-model v1
id <id>
source <r1:references.json@7b176dbb | dec053:<fixture_id> | k6-invented>
counts nodes <N> members <m> loads <L>
section E <hex> G <hex> A <hex> Iy <hex> Iz <hex> J <hex>
node <index> <label> <x> <y> <z>                    (R1's order; label = R1's node id)
member <index> <label> <i> <j> <yref_x> <yref_y> <yref_z>
restraint <node> <UX UY UZ RX RY RZ as a 0/1 string>  (ascending node)
load <global dof> <value>                           (ascending dof; nonzero only)
```

- The sha256 is computed in Python (`hashlib`), never in Rust (no `sha2`).
- `models_sha256.txt` lists the 33 sealed models. The extras (Q4's chain and the Q5 grids) are listed under a separate, labelled heading.

### 3.4 Two generators and the cross-check

- **The Rust generator** (`H/src/k6/models.rs`) is the kernel model.
- **An independent Python generator** in `k6_runner.py` builds the same canonical bytes from R1's rules and the stated formula. It uses Python floats, and every value is exact except the section, whose formula Python evaluates with identical IEEE operations. It is the runner's source for the closed-form counts, and the unit tests use it to check `models_sha256.txt` for all 33 models with no binary needed (B3).
- **The recorded cross-check** (A1; `T3/IMPLEMENTATION/K6/_run_records/crosscheck/`):
  - a standard-library script imports `references.py` once (P1's `load_refs` method), builds RF-LARGE, and takes `model_json(defn, full=True)`, which is exactly what `--model` prints (`references.py:2985-2994`);
  - it then compares node order and labels, coordinates, members, restraints, loads, E, G, OD and ID, each rounded once with `float(Fraction)`, against the binary's `--emit-model` output parsed back, for all 24 cases;
  - it also runs the literal `references.py --model <id>` for the 10-member cases and one 10,000-member case, and diffs that against the imported path;
  - any difference is a stop; a reference is never edited.

## 4. The counts and their oracles

### 4.1 The counts line

| Field | Source |
|---|---|
| `nodes`, `members`, `dofs` n = 6N, `free_dofs` n_f | model |
| `pattern_entries` (both triangles) | `SparseStiffness::storage_counts().stored_entries` (`sparse.rs:309`) |
| `lower_entries` | `.lower_entries` |
| `free_lower_entries`, `free_lower_nonzero` | the free block's lower entries (`SparsePreparedSystem::lower_entries`, `sparse.rs:1243`), all and value ≠ 0 |
| `rcm_profile_entries`, `rcm_half_bandwidth` | K6's O(nnz) skyline count on the order from `reverse_cuthill_mckee(&adjacency_from_symmetric_entries(…))` (`SD/lib.rs:480`, `:505`). It allocates no profile values (K1 review N3: `order_sparse_structural` allocates first). The `factor` stage records `order_sparse_structural`'s own `profile_entry_count` and half-bandwidth, and the `parity` item `rcm_count_equals_ordering` must be true |
| `identity_profile_entries`, `identity_half_bandwidth` | F1b's `observation_lane_profile` rule (`PP@F1b:2966-3002`) on the lane's entries (N19): O(nnz), no profile storage |
| `contributions` | `SparseAssemblyEvidence::storage_counts().contributions` (`SA:565`) |
| `dense_entries` | n² (u128) |
| `estimate_f1b_bytes` | 96·n² (dense, `PP@F1b:2878`) or 24·identity entries (`lane-id`, `PP@F1b:2958`), quoted with source commit `130445db2`; null for the other modes |
| `estimate_adm_bytes` | section 7's admission formula for this mode |
| `size_of` | `Expansion` is `pub(crate)`, so its 32 bytes are derived (`FK/structural.rs:675-678`: `Vec<f64>` + `usize`). The binary prints `size_of` of `StiffnessContribution`, `ContributionRounding`, `ResidualRow`, `PivotEvidence`, `FrameElement` and `SymmetricMatrixEntry`, so section 7's coefficients are fixed from facts at A1 |

### 4.2 Closed forms (derived; they hold for these models)

The premises: every node carries at least one frame; no two frames join the same node pair; there are no springs, user elements or blocks. All three hold for RF-LARGE, the nine and the grids by construction.

- **Pattern entries = 36·(N + 2m).**
  - `SparsePattern::from_connectivity` (`sparse.rs:82-130`) gives each node's row set as the union of the 6×6 blocks of the node and its element neighbours. So the pattern is one diagonal block per node (36N) and two off-diagonal blocks per distinct neighbour pair (72m).
  - This is K1's figure (`T3/IMPLEMENTATION/K1/RETURN.md:416`).
- **Lower entries = (36(N + 2m) + 6N)/2 = 18(N + 2m) + 3N.** The pattern is symmetric with a full diagonal.
- **Contributions = 144·m.** One contribution per entry of each frame's 12×12 matrix, both triangles (`SA:1189` in `EvidenceParts::element`, called once per frame).
- **Dense entries = (6N)².**
- **Free-block entries (both triangles) = Σ_k f_k² + 2·Σ_(a,b) f_a·f_b**, where f_k is node k's free DOF count and the second sum runs over members. Free lower entries = (that + n_f)/2. By family:
  - CHAIN and TREE (root fixed; the root's single member contributes 0): 108n − 72 entries, n_f = 6n, **free lower = 57n − 36** (534 at n = 10, as K1's table).
  - CONT (s = n/2; S0 has 0 free DOFs, S_i 3, C_j 6): 117s − 36 entries, n_f = 9s, **free lower = 63s − 18**.
  - Grid x × y, bottom row fixed: 36x(y−1) + 72[(x−1)(y−1) + x(y−2)] entries, n_f = 6x(y−1).
- **Identity-order profile, full-block bound** (every entry of a coupled 6×6 block nonzero), on the lane's reduced numbering:
  - CHAIN and TREE: **57n − 36**, the free lower count, because the numbering is already banded.
  - CONT: **27s² + 36s − 18**. Derivation: S_i's rows give 1 + 2 + 3; C_j's row d has length 3s + 3j + d + 1 for j ≥ 2, and 3s + d + 1 for j = 1. Summing gives 6s + (18s + 21) + Σ_{j=2..s}(18s + 18j + 21) = 27s² + 36s − 18.
  - At s = 5,000 this is **675,179,982**, F1b's figure for CONT n10000 (`ROOT_RULINGS_V1.md:1454`), and at s = 5 it is 837. Exact zeros in the element matrices, which the lane skips, can only lower it (the AX cases). The exact values come from the counts and the oracle.

### 4.3 The oracles, independent of K6's Rust counting

1. **Closed forms** (4.2), in Rust (C1, ≤ 100 members and the nine) and in the runner's Python (all sizes).
2. **K1's table** (`T3/IMPLEMENTATION/K1/RETURN.md:407-414`), as a cross-check with its limits stated (N12):
   - stored, lower and free lower depend only on N, m and the restrained set, so they must match at every size and orientation;
   - K1's `large()` (`SD/structural/k1_tests.rs:708-771`) numbers the comb's spine before its branches, chooses its own y-references, and rotates in f64. So nonzero counts, RCM profile and half-bandwidth are compared only where K6's model equals K1's: CHAIN-AX, which should equal K1's figures because Iy = Iz and the transform entries there are 0 and ±1. Elsewhere a difference is expected and explained, and the oracle decides.
3. **A standard-library Python port** of `adjacency_from_symmetric_entries`, `reverse_cuthill_mckee` (with `pseudo_peripheral_start`, `bfs_eccentricity` and the (degree, index) tie-break, `SD/lib.rs:480-632`), the skyline first-column rule (`from_entries_with_order`, `:215-275`), and F1b's identity rule.
   - It reads the binary's `--dump-pattern` output: free lower nonzero positions for the RCM, and the lane's entry positions for the identity profile.
   - It computes RCM profile, half-bandwidth and identity profile for all 33 models plus the extras, and compares them with `counts.jsonl`.
   - It runs once at A1–A2, recorded in `_run_records/oracle/`.
   - The values for the ≤ 100-member models and the nine are also committed as constants in `H/tests/k6_counts.rs`, so CI pins the oracle's result.
   - The reviewer re-derives them independently (brief).

## 5. The allocator (`H/src/bin/k6_observe/alloc.rs`, registered only in the binary and in `tests/k6_alloc.rs`)

- **State** (atomics, single-threaded use): `CURRENT`, `PEAK`, `PEAK_MOVE`, `STAGE_PEAK`, `STAGE_PEAK_MOVE`, `CALLS`, and `CAP`, which is `usize::MAX` until the CLI sets it, before the model is built.
- **`alloc`, `alloc_zeroed`:** reserve `size` against `CAP` (`current + size > cap` refuses; exactly at the cap succeeds). Then forward to `System.alloc`/`System.alloc_zeroed`, keeping calloc behaviour for RSS. Release the reservation on a null from the system. Update all peaks.
- **`dealloc`:** forward, then `CURRENT -= size`.
- **`realloc` growing (new > old):**
  - reserve `new − old`, the in-place model, which is the Mac gate probe's rule (`T3/PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt:29-34`);
  - **and** set `PEAK_MOVE`/`STAGE_PEAK_MOVE` to at least `current_before + new`: the old block and the new block alive together, which is what F1b's 24-byte derivation assumes (`PP@F1b:2950-2957`).
- **`realloc` shrinking:** `CURRENT -= old − new`.
- **Two peaks, both deterministic** for a given binary and input (N8):
  - `heap_peak` (in-place) is what the cap enforces, as on the gate;
  - `heap_peak_move` bounds the transient.
- **Refusal** (`alloc`, `alloc_zeroed` or a growing `realloc` that would exceed `CAP`):
  - write a fixed marker, `k6_observe: heap cap refused <size> bytes (current <c>, cap <cap>)`, to fd 2 through `std::fs::File::from_raw_fd(2)` + `write_all` + `mem::forget`, with the integers formatted into a stack buffer, so nothing allocates;
  - then return null. Rust's alloc-error path prints `memory allocation of N bytes failed` and aborts (SIGABRT), as the gate probe does.
  - The marker is what separates `heap_cap_abort` from `rlimit_abort` (Linux), whose failure prints only Rust's line.
- **Stage peaks:** at each `stage_begin`, `STAGE_PEAK = STAGE_PEAK_MOVE = CURRENT`. The `stage` line reports them as absolute peaks, including live data from earlier stages.
- **Scope:** never in `H`'s library, so no consumer inherits it (brief). `k6_alloc.rs` includes the same file through `#[path]` and registers it in a `harness = false` binary, so an accounting regression can abort only that test binary.

## 6. The runner (`H/runner/k6_runner.py`, standard library only)

### 6.1 Process tree and PID

- **macOS:** `subprocess.Popen(["/usr/bin/time", "-l", "-o", <run>.time.txt, binary, …], start_new_session=True, stdout=PIPE, stderr=PIPE)`.
- **Linux:** `/usr/bin/time -v -o …`, with `preexec_fn` applying `resource.setrlimit(RLIMIT_AS, (C, C))` in the wrapper before exec. The binary inherits it. P1's method is `T3/DETECTION/scripts/run.py.txt:29-35`.
- `-o` keeps time's report apart from the binary's stderr. Both BSD and GNU `time` accept `-o`; BSD's is read from `man time` on this host.
- **The binary's PID:** `pgrep -P <wrapper pid>`, polled every 10 ms until found, for at most 2 s. It is then cross-checked against the binary's own `start` line `pid` when that line exists, and a mismatch is recorded as an `error`.
  - The watchdog polls **that PID**, never the wrapper's (K6-M3).
  - Using `pgrep -P` rather than the start line keeps the runner able to drive any child, including the test's Python child.
- **Pumps:** two threads drain stdout and stderr, as P1 did. Stdout lines are also teed to `<run>.jsonl` as they arrive, so a killed process keeps every flushed line.

### 6.2 Watchdog, kill and limits

- **The macOS watchdog** (`DESIGN.md:826`): every 100 ms, `ps -o rss= -p <binary pid>` (KiB).
  - It records `ps_samples`, `ps_max_kib` and the poll count.
  - Above C, it sends **SIGKILL to the process group** (`os.killpg(pgid, SIGKILL)`; pgid = the wrapper's pid, the session leader) and records `killed_by_rss_watchdog`, the last reading (KiB) and the time since start.
- **Timeouts:** 600 s, or 1,800 s at ≥ 1,000 members (P1's, `run.py.txt:21-22`). The same group SIGKILL, then `timed_out`.
- **After a kill:** the runner confirms no survivor with `pgrep -g <pgid>` (empty) and records it. The no-survivor assertion of test H uses the same check.
- **Heap cap:** passed as `--heap-cap-bytes C − 512 MiB` (Q3). The ceiling run gets C = 16 GiB and 15.5 GiB.
- **The host quiet check before each run** (P1's `cargo_busy`, and `_COMMON.md:31`, `:33`):
  - `pgrep -x cargo` and `pgrep -f 'python[0-9.]* .*run_evidence_sweep'` must be empty, else wait 30 s and re-check;
  - `sysctl -n kern.memorystatus_level` is recorded, and the run is deferred while it is below 80 (N18);
  - the TASK checks `<wt>/guard/memguard.log` after each tier. A SIGKILL the runner did not send is classified `error` (detail `external_sigkill`) and is a stop.

### 6.3 Parsers and units

| Source | macOS | Linux | Normalized to |
|---|---|---|---|
| `/usr/bin/time` | `^\s*(\d+)\s+maximum resident set size` (**bytes**); also `^\s*(\d+)\s+peak memory footprint` (bytes; counts compressed pages, which answers the brief's compression caveat) | `Maximum resident set size \(kbytes\): (\d+)` (**KiB**) | bytes |
| `os.wait4(wrapper)` `ru_maxrss` | bytes | KiB | bytes |
| `ps -o rss=` | KiB | KiB | bytes |

- `wait4` on the wrapper includes the binary only when the wrapper reaped it. After a group SIGKILL, time's report is absent and `ru_maxrss` is the wrapper's own, so killed runs report `ps_max_kib` as their RSS, with the source named.
- The parser tests use recorded outputs. The macOS `-l` fixture is recorded live at A2. The Linux `-v` fixture is written from GNU time's documented format, disclosed as constructed, because no Linux host is available.

### 6.4 Classification (one per process)

`ok` · `timed_out` · `killed_by_rss_watchdog` · `heap_cap_abort` · `rlimit_abort` · `refused_by_binary` · `error`

| Class | Rule |
|---|---|
| `ok` | wrapper exit 0 and a `summary` line |
| `refused_by_binary` | exit 3 and a `refusal` line |
| `timed_out`, `killed_by_rss_watchdog` | the runner's own kills |
| `heap_cap_abort` | no `summary`, the K6 marker on stderr, abnormal termination (BSD: time's "terminated abnormally"; GNU: "Command terminated by signal 6") |
| `rlimit_abort` | Linux only: no `summary`, Rust's `memory allocation of N bytes failed` without the K6 marker, under an applied `RLIMIT_AS` |
| `error` | everything else, with the detail |

### 6.5 Aggregation and outputs

- **Per (model, mode):**
  - per stage, over completed repeats: **median** (the middle element of the sorted values; the mean of the two middle ones for an even count) and **minimum** of `elapsed_ns`, over the repeat axis only (K6-M7);
  - per-stage and overall `heap_peak`/`heap_peak_move` (constant across repeats if deterministic; the determinism check records any variation);
  - RSS from every source, the outcome class per repeat, the parity lines, the counts, load before and after (`os.getloadavg()`), and `memorystatus_level` before and after.
- **Cross-mode lines,** computed by the runner:
  - outcome-class parity, sparse against dense, per model up to 1,000 members;
  - the DEC-053 basis: `max|u_s − u_d| / max|u_d|` over the global displacement vectors, read exactly from both processes' `--dump-solution` bits in Python;
  - bitwise K comes from the dense process (11 D).
- **Outputs:**
  - raw `<run>.jsonl`, `<run>.stderr.txt`, `<run>.time.txt` and `<run>.record.json` per process;
  - `records.jsonl`, one runner record per process;
  - the packet (`k6_packet.json`: metadata, models, per-run table, fits, F1b ratios, parity table, outcome classes, and the sha256 of every raw file).
- **Fits** (Constraints): log-log least squares of `heap_peak_move` against members, per family and mode, with points and residuals. The same for peak RSS, with the process baseline (the 10-member RSS) stated. Observed fits, never thresholds.
- **Metadata per packet:** OS version (`sw_vers -productVersion`/`-buildVersion`, `platform.release()`); architecture (`platform.machine()`); CPU model (`sysctl -n machdep.cpu.brand_string`); `hw.physicalcpu`, `hw.logicalcpu` and `hw.memsize`; `rustc -Vv` and `cargo -Vv`; the build profile (release, from `cargo build --release --offline --locked` in the archive); the binary's sha256; the source commit and its `git archive` tree SHA, given to the runner by the TASK.
  - **Never recorded:** `platform.node()`, `uname -n`/`-a`, any user name, serial or path. Paths are written as placeholders (`<scratch>`, `<wt>`). A unit test asserts the metadata dict has none of these keys and no `/`-rooted value except the fixed `/usr/bin/time`.

### 6.6 Modes

- **`--plan`:** the schedule (section 8) with each run's estimates, C, heap cap, admission and reason, and projected time. It starts no child process; its counts are the Python closed forms plus `counts.jsonl` (N7).
- **`--smoke`:** the A2 dry run, with its heap cap at 512 MiB, so its peak is ≤ 512 MiB by construction (I). It covers the 10- and 100-member RF-LARGE models in `sparse` and `dense`, the nine in both, one watchdog kill of the Python test child, and one heap-cap abort (the binary, 100-member dense, `--allow-over-estimate`, heap cap 8 MiB).
- **`--run --tier <t>`:** checkpoint B, only in ROOT's slot. The admission rule is re-evaluated before each run from the records so far. Every run, admitted or not, is recorded with its reason.
- **Refusal by name (ROOT's ruling, RV16 N4).** Before any admission arithmetic, and in every mode (`--plan`, `--smoke`, `--run`), the runner refuses without starting a process:
  - `never:n2_mode_at_or_above_10000_members`: `dense` or `lane-lu` on any model with ≥ 10,000 members;
  - `never:cont_n10000_identity_lane`: `lane-id` on `RF-LARGE-CONT-n10000-AX`/`-ROT`.

  Then, for every other run, `lane-id` included:
  - `deferred:estimate_fails_admission`, when the admission rule (8.1) is not met.

  The binary's own refusals (2.2) repeat the first two by name as a second, independent barrier.

## 7. The derived peak estimate per mode

The sizes of the Rust structs used below are read from their definitions and confirmed at A1 by the binary's `size_of` fields; the coefficients are then fixed in `--plan` before B.

- Expansion: 32 bytes (`FK/structural.rs:675-678`).
- `StiffnessContribution`: 24 bytes (`:32-36`).
- `ContributionRounding`: 96 bytes before its two heap `Vec`s (`:283-292`).
- `ResidualRow`: 112 bytes (`:265-281`, estimated).
- `PivotEvidence`: 48 bytes (`:256-263`).
- `FrameElement`: 136 bytes (`FK/lib.rs:429-561`).
- `SymmetricMatrixEntry`: 24 bytes (`SD/lib.rs:98-102`).

### 7.1 dense: F1b's constant, and the buffers at the peak

- **F1b's estimate:** `96·n²` (`PP@F1b:2871-2878`, commit `130445db2`).
- **Re-derived on this base.** At the dense peak, inside `prepare_bound`'s `audit_contributions`, these are alive together:
  - the dense view, 8n² (`sparse.rs:299-308`);
  - the two symmetry views, 8n² + 8n² (`SA:573-585`);
  - the prepared matrix, 8·n_f² (`FK/structural.rs:1260`);
  - the contribution sums, 32n² (`:775`);
  - the differences, 32n² (`:796`).
  - Total ≤ 96n², with n_f ≤ n.
- **The later stages are lower:**
  - factor: view + symmetry + prepared + Cholesky L (`:1822`) = 40n²;
  - finish: those + the intended-action sums (`audit_intended_action` → `contribution_sums`, `:886-891`) = 72n².
  - So the peak is at `prepare`, as F1b counted.
- **The admission estimate adds the O(nnz) state alive beside it** (N9):
  - `E_base` = 24·nnz + 8n (k) + [24·C + 32·nnz + 8n + 32N + 160m] (evidence) + 136m (formation source) + 8n_f + 16r, with r = restrained DOFs;
  - `E_dense,adm = 96·n² + E_base`.
- **The claim ratios** (Constraints item 2) are `heap_peak_move / 96n²` and `peak RSS / 96n²`.

### 7.2 sparse: derived, max over the three candidate peaks

Counts: Z = nonzero pattern entries, R = contribution-rounding rows, L_f = free lower entries, A_off = off-diagonal free lower nonzeros, P = RCM profile. For admission, Z ≤ nnz and R ≤ nnz (upper bounds). R is recorded afterwards from the report's `contribution_rounding.len()`.

- **E_prep** (`sparse_contribution_sums`, `sparse.rs:1168-1197`; `sparse_audit_contributions`, `:1503-1601`): 8n (`prescribed_position`) + 32·nnz_f + 28·n_f (prepared) + 32·nnz + 32·Z (sums and their term `Vec`s, first-push capacity 4) + 32·nnz + 32·Z (differences, cloned then re-added) + 112·R (rounding rows and their two clones).
- **E_fact:**
  - prepared 32·nnz_f + 28n_f;
  - `order_sparse_structural`'s transients (`SD/structural.rs:56-76`): entries 24·L_f; adjacency and the RCM neighbour copy, each ≤ 2 × 16·A_off with capacity doubling; the profile's `resize` growth, ≤ 24·P in the move model; about 48·n_f of index vectors;
  - afterwards, the factor's rows 8P + 24n_f and ProfileFactor's `first`/`order`/`work` 24n_f.
- **E_fin:** prepared + factor (8P + 48n_f) + pivots and their clone 96n_f + two `ResidualRow` lists 224n_f + intended-action sums 32·nnz + 32·Z + u, y and the solve vectors ~24n.
- **E_sparse,adm = E_base + max(E_prep, E_fact, E_fin).**
- **Order of size, with the upper bounds:** about 330·nnz + 24·C bytes, which is about 390 MB at 10,000 members. The sparse ratio will be below 1 wherever R ≪ nnz. That is expected and reported as such.

### 7.3 lane-id: F1b's constant, and the lane's full count

- **F1b's estimate:** `24 × identity-order profile entries` (`PP@F1b:2950-2958`: the last `resize` growth of `from_entries`' values, old + new ≤ 3 × the final length).
- **Derived for admission** (`solve_symmetric_system_from_entries`, `SD/lib.rs:700-746`). `original_profile` is a binding that lives to the end of the function, so it is alive through the ordered build and the factor:
  - the lane's entries, ≤ 72·E_lane under push growth (E_lane ≤ 78m);
  - force 8n_f;
  - adjacency ≤ 32·E_off, and RCM's copy ≤ 32·E_off;
  - then max(24·P_id, 8·P_id + 24·P_rcm, 8·P_id + 16·P_rcm + 8n_f), plus ~64·n_f of index vectors (`from_entries_with_order`, `:215-275`; `factorize_ldlt` clones the ordered profile, `:382-388`).
- **Expected:** for CONT, 24·P_id dominates (ratio ≈ 1). For CHAIN and TREE, P_id is tiny and the entries and adjacency dominate (ratio well above 1). Both are reported as observations of F1b's constant.

### 7.4 lane-lu: derived

- The dense view (8n²) and the reduced matrix (`FK/lib.rs:1532`, 8n_f²) are alive together in `reduce_assembled_system`. The view is then dropped (`PP@F1b:3849`), and `solve_dense` clones the reduced matrix (`FK/lib.rs:1641`).
- So `E_lu = 24·nnz + 8n + max(8n² + 8n_f², 16n_f²) + O(n)`, about **16 bytes per n² entry**. F1b sets no constant for this lane.

## 8. The schedule, with admission under Q3

### 8.1 The rule as the runner implements it

- **C** = 8 GiB (RSS watchdog); heap cap = C − 512 MiB = 7.5 GiB; C/2 = 4 GiB.
- A run is **admitted** if either:
  - (i) P1's Linux peak for the same case and mode (`T3/DETECTION/RETURN.md:228-277`, product level on `c61a540ea`, `sparse_interactive` → `sparse`, `dense_scrutiny` → `dense`) is ≤ C/2; or
  - (ii) `E_adm × ρ ≤ C/2`, where ρ is the largest `max(peak_rss, heap_peak_move)/E_adm` measured at smaller sizes of the same family and mode. The families are CHAIN, TREE, CONT, DEC053 and GRID, with orientations pooled. ρ = 2 when none has been measured yet.
- P1's peaks never admit a lane mode, because P1 did not isolate the lanes.
- **Ascent:** 10 → 100 → 1,000 → 10,000 per family and mode. A size runs only after the previous size's records are written and the rule is re-evaluated.
- **Never,** whatever the rule says (ROOT's ruling, RV16 N4), refused by name by both the runner and the binary:
  - every mode that materializes an n² matrix at ≥ 10,000 members, that is `dense` and `lane-lu`;
  - CONT n10000 `lane-id` (Q12; estimate about 16.2 GB).
- Any other `lane-id` run whose estimate fails the admission rule is **deferred by name** (`deferred:estimate_fails_admission`) and never started.
- **A stop:** a watchdog kill or a heap-cap abort on an admitted run.

### 8.2 The table (138 scheduled rows: 124 to run, 1 of them conditional, and 14 refused by name as never)

Estimates are in MiB, from 4.2's closed forms: dense 96n²; lane-lu 8n² + 8n_f²; lane-id 24·P_id, with P_id at its full-block bound. The sparse and lane-id admission estimates are fixed at A2 from `counts.jsonl`.

| Tier | Runs | Estimates | Admission (initial) |
|---|---|---|---|
| T1: n = 10 (6 cases × 4 modes) and the nine (× 4) | 60 | dense ≤ 0.4; lane-lu ≤ 0.1; lane-id ≤ 0.1; sparse < 2 | (i) 34.4–34.6 MiB (sparse, dense); (ii) with ρ = 2 (lanes, the nine) |
| T2: n = 100 (6 × 4) | 24 | dense 33.6; lane-lu 4.3–5.5; lane-id CHAIN/TREE ≈ 0.1–0.2, CONT 1.6; sparse ≈ 4 | (i) 49.6–52.7 MiB; (ii) for the lanes |
| T3a: n = 1,000 sparse and lane-id (12) | 12 | sparse ≈ 40; lane-id CHAIN 1.3, TREE 1.7, CONT 154.9 | (i) 3,372.8–3,617.2 MiB ≤ 4,096 (sparse); (ii) (lane-id) |
| **Projection report to ROOT (Q7)** | — | — | — |
| T3b: n = 1,000 dense and lane-lu (12) | 12 | dense 3,302.5; lane-lu CHAIN/TREE 549.9, CONT 429.7 | (i) P1 ≤ C/2 (dense; the premise of Q3, confirmed by the first run); (ii) lane-lu |
| T4: n = 10,000 sparse (6) and lane-id CHAIN/TREE (4) | 10 | sparse ≈ 390 (upper bound); lane-id CHAIN 13.0, TREE 17.2 | (ii) only (P1 has none ≤ C/2) |
| T4 never (refused by name) | 14 | dense 329,655.8 (322 GiB) × 6; lane-lu 42,920.8–54,937.1 × 6; lane-id CONT 15,453.6 × 2 | `never:n2_mode_at_or_above_10000_members` (12: every n² mode at ≥ 10,000 members, ROOT's RV16-N4 ruling and the host rule); `never:cont_n10000_identity_lane` (2; Q12) |
| T5: Q5 grids, sparse only | 4 + 1 | P_id bounds (MiB of 24P): 16×16 3.1; 32×32 25.8; 64×64 211.2; 96×96 718.3; 128×128 1,708.9 | (ii). 128×128 only if its ρ-scaled estimate fits at 96×96's ratio (N14) |
| T6: Q4 ceiling chain, dense, C = 16 GiB | 1 | 6,141.0 (96 × 67,076,100 = 6,439,305,600 bytes; F1b's ceiling is 6,442,450,944) | (ii) against C/2 = 8 GiB, with CHAIN-dense's ρ; last and alone, in ROOT's separate slot |

- **The count.**
  - RF-LARGE: 24 cases × 4 modes = 96 rows (24 at each size).
  - The nine: 9 × 4 = 36.
  - Q5 grids: 5, the last conditional.
  - Q4: 1.
  - Total 138. Of these, 14 are refused by name (T4 never) and 124 are to run, including the conditional 128×128 grid.
  - Every `lane-id` run is also subject to `deferred:estimate_fails_admission`. On the initial estimates, none is deferred.
- **Mode order:** within a tier, model i runs its modes in the order `[sparse, dense, lane-id, lane-lu]` rotated by i (Q7(a)). The exception is n = 1,000, split into T3a and T3b so that the Q7 projection precedes the dense runs (N11).
- **The first dense 1,000-member run** (CONT-n01000-AX, the cheapest) confirms or refutes Q3's premise that the kernel peak is below P1's, under the heap cap. A refutation is a stop.

### 8.3 Projected quiet-host time (a projection, not a measurement or claim)

**The basis.**
- P1's product dense 1,000-member times, on a shared 15 GiB Linux host: CONT 204–215 s (n_f = 4,500), CHAIN-AX 487 s, TREE-ROT 451 s (n_f = 6,000); CHAIN-ROT and TREE-AX were killed at 1,800 s.
- Those times included both the dense Cholesky (n_f³/6 checked multiply-adds, about 3.6·10¹⁰ at 6,000) and the legacy LU (n_f³/3 multiply-adds plus about n_f³/2 finite checks). The ratio 487/204 ≈ (6,000/4,500)³ = 2.37 fits an n_f³ cost.

| Group | Projection |
|---|---|
| T1 + T2 (84 processes) | under 10 min |
| T3a + T4 sparse and lane-id (22) | 20–40 min. Geometry is O(N·m) ≈ 10⁸ edge visits at 10,000 members (`SA:1211-1240`) |
| T3b dense, four ordinary cases, 5 staged repeats + entries in repeat 0 only (N3) | 10–25 min each |
| T3b dense, CHAIN-ROT and TREE-AX | **likely 30 min each, timed out** (N10) |
| T3b lane-lu (6), 5 repeats | 5–15 min each |
| T5 grids | 20–40 min |
| T6 ceiling | ≤ 30 min, separate slot |

- **Total for B without T6: about 3.5–5 h, of which T3b alone is about 2–3.5 h.** Q7's two-hour stop will very probably trigger before T3b.
- The binding projection is made at T3a/T2 from measured 100-member first-repeat stage times, scaled by each stage's order: factor and LU by (n_f,1000/n_f,100)³ ≈ 1,000; n² stages by about 100; O(nnz) stages by about 10. See N11 for the options I propose ROOT rule on now.

## 9. The DEC-025 sandbox question (Q9(b))

**Answer: it cannot be settled by reading. It needs one run in the invoking context.** What the records and code establish:

1. **The sweep applies no OS sandbox itself.**
   - `run_evidence_sweep.py` runs each surface command with a plain `subprocess.run(command, cwd=root, env=env)` (`P/tools/release/run_evidence_sweep.py:343-347`).
   - "sandboxed" is a declared execution-capability label (`:49`, `:175-236`), validated as a label: the `python_pytest` surface must be declared "sandboxed" (`:829-830`).
   - `--require-capability`/`--only-capability` only select or refuse surfaces (`:1025-1060`, `:1120-1136`).
   - So the new pytest wrapper runs in whatever context invokes the sweep. Because its surface is declared "sandboxed", it should work in a sandboxed agent context.
2. **The Mac sweeps of record** ran `--only-capability sandboxed` from ROOT's session (`T3/IMPLEMENTATION/K1_MERGE/RECORD.md:55`; K2b's summary `T3/IMPLEMENTATION/K2B_MERGE/dec025/SWEEP_20260928T113531Z_33e33c723a97.json`). The summary's `runtime` holds only platform, Python, node and cargo versions. No record states whether an OS sandbox (Claude Code's or Codex's Seatbelt profile) was active in that context.
3. **On this Mac,** `/bin/ps` is **setuid root** (`-rwsr-xr-x root wheel`); `/usr/bin/pgrep` and `/usr/bin/time` are ordinary executables (`ls -l`, this turn). Under a Seatbelt profile, whether exec of a setuid `ps`, process-info on a child, and `killpg` of the child's own group are allowed depends on the profile. That cannot be read from the repository.
4. **No existing `P/tests` test** uses `ps`, `killpg`, `setsid` or `start_new_session` (`git grep` on the base), so there is no precedent under the sweep.

**Proposal.**
- At A2, `python -m pytest -q tests/test_performance_harness_runner.py` is run once in exactly the context ROOT uses for DEC-025, by ROOT or by me as ROOT directs, and recorded.
- If `ps`, `pgrep` or `killpg` is denied there, the test **fails**. Nothing is skipped. I report, and ROOT rules.
- Separately (N16): on a Linux sweep host, the live test uses `RLIMIT_AS` without the `time` wrapper, because a Linux container may lack GNU `/usr/bin/time`.

## 10. The Scope 8 scans (read-only, on `56dd72334`)

- **Package name** `open_pipe_stress_solver_performance_harness` outside run records (`git grep -l … -- ':!projects/*/execution/**' ':!P/validation/evidence/**'`):
  - `H/Cargo.toml:2`;
  - `H/Cargo.lock:35`;
  - `H/examples/sparse_default_promotion_observation.rs:1` (inside `H`; the brief's list omitted it);
  - `P/provenance/build-artifacts/core__solver__performance_harness__Cargo.lock:34`.
- **Manifests naming `performance_harness`:** only `H/Cargo.toml`. So no crate depends on `H`.
- **Readers of `provenance/build-artifacts`** in `P/tests`, `P/tools`, `tools/` and `.github/`: none. **References to `performance_harness`** in `P/tools`, `tools/` and `.github/`: none.
- **The only `P/tests` readers of `H`:** `test_sparse_default_promotion_observation.py:18` and `test_sparse_suitability_observation.py:14`. They read `H/src/lib.rs` as text and assert substrings (`:95-103`; `:50-67`). One added `pub mod k6;` line removes none.
- **CI:**
  - `H` is discovered automatically (`P/tools/release/check_release_readiness.py:70-82`, `discover_cargo_manifests`) and run as `cargo test --offline --locked` debug (`P/tools/ci/numerical_ci.py:26-47`). That is why the lock must carry the three path packages.
  - The CI checkout omits `projects/*/execution/` (`.github/workflows/piping-desktop-e2e.yml:186-188`), so no test reads T3 records or R1's references.
  - Hosted CI's pytest step runs only `test_ci_*.py` (`:53-54`), and the wrapper's name avoids that selector.
- **Conclusion:**
  - No binary that T9 or the both-entry gate builds can change, since K6 touches no crate in PP's or the headless runner's dependency closure.
  - T9 and the gate are not run (brief).
  - The reviewer re-runs these scans on the candidate.

## 11. Tests

The predicate wherever a value is compared is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, or the DEC-053 basis. No time or memory bound is asserted anywhere.

| Id | File :: test | What |
|---|---|---|
| A1 | `H`'s existing tests | unchanged, all pass |
| A2 | `P/tests/test_sparse_default_promotion_observation.py`, `test_sparse_suitability_observation.py` | pass |
| A3 | (record) | `git diff` of `H/src/lib.rs` = the one line; the legacy functions, constants and example byte-identical |
| A4 | (record) | `cargo build --release --bin k6_observe` and the lib build have no warnings; `#[allow(dead_code)] // <consumer>` only per item |
| B1 | `k6_models.rs::canonical_bytes_equal_committed_models` | the 12 RF-LARGE models at 10 and 100 members and the nine equal `H/observations/k6/models/*.k6model` (`include_str!`) |
| B2 | (record, A1) | the cross-check against `references.py` (3.4), all 24 cases |
| B3 | `test_k6_runner.py::ModelHashes` | the Python generator's 33 canonical models hash to `models_sha256.txt`; the committed small files hash to the same lines |
| B4 | `k6_models.rs::section_bits_are_pinned`, `test_k6_runner.py::SectionBits` | A, I and J bits (3.2), independently in Rust and Python |
| C1 | `k6_counts.rs::closed_forms` | 4.2's forms on every ≤ 100-member model and the nine |
| C2 | `k6_counts.rs::k1_table_cross_check` | stored, lower and free lower against K1's table rows for n = 10 and 100 (all orientations); nonzero and profile for CHAIN-AX (N12); the 1,000 and 10,000 rows are recorded from `counts.jsonl` |
| C3 | `k6_counts.rs::oracle_constants` | RCM profile, half-bandwidth and identity profile equal the recorded oracle's constants; `rcm_count_equals_ordering` (K6's count = `order_sparse_structural`'s) |
| C4 | `test_k6_runner.py::CountsClosedForms` | `counts.jsonl` against the Python closed forms at all sizes |
| D1 | `k6_parity.rs::bitwise_k` | every stored entry equals `assemble_global_stiffness`'s entry bit for bit (`to_bits`), and every unstored dense entry is `+0.0` (bits 0) |
| D2 | `k6_parity.rs::outcome_class_parity` | staged sparse class = staged dense class |
| D3 | `k6_parity.rs::dec053_basis` | `max|u_s − u_d|/max|u_d| ≤ 1e-9` |
| E1 | `k6_staged.rs::staged_equals_sa_entry_checked` | staged (formation-checked) `Debug` string == `solve_assembled_with_formation_check(…, true)`'s, both modes |
| E2 | `k6_staged.rs::staged_plain_equals_sa_solve_assembled` | the staged sequence without the formation source (`prepare_assembled_*`) == `solve_assembled`'s, both modes |
| F1 | `k6_alloc.rs` (harness = false) | accounting over a known sequence: alloc, dealloc, realloc up (in-place and move peaks), realloc down, `alloc_zeroed` |
| F2 | `k6_alloc.rs` | cap boundary: exactly at the cap succeeds; one byte over returns null (direct `GlobalAlloc::alloc` calls, so no abort) |
| F3 | `k6_bin.rs::peak_is_deterministic` | two runs of one model and mode (10-member sparse and dense) give equal `heap_peak` and `heap_peak_move` |
| F4 | `k6_bin.rs::heap_cap_abort_is_marked`; `test_k6_runner.py::ClassifyHeapCapAbort` | the debug binary on CHAIN-n00100-AX dense at an 8 MiB cap, with `--allow-over-estimate`, aborts (SIGABRT) with the K6 marker; the runner classifies the recorded outputs `heap_cap_abort`; `--smoke` exercises the live path |
| G1 | `k6_bin.rs::n2_modes_at_10000_refused_before_n2` | for **both** `dense` and `lane-lu` on CHAIN-n10000-AX: exit 3, reason `n2_mode_at_or_above_10000_members`, `heap_peak_so_far` < 64 MiB (≪ 8·n² ≈ 26.8 GiB) |
| G2 | `k6_bin.rs::estimate_over_half_cap_refused` | exit 3, reason `estimate_exceeds_half_cap`, for `dense` and for `lane-id` (CHAIN-n00100 at caps where the counts phase fits but the estimate exceeds half the cap) |
| G3 | `k6_bin.rs::cont_n10000_identity_lane_refused` | `lane-id` on CONT-n10000-AX and -ROT: exit 3, reason `cont_n10000_identity_lane`, before any lane entry is built |
| H1 | `test_k6_runner.py::WatchdogLive` (macOS) | section 6's watchdog, under `/usr/bin/time -l`, 128 MiB cap. The Python child grows 1 MiB per 10 ms, touching each page, with a failsafe at 512 MiB (exit 1). It must be killed by group SIGKILL, with `killed_by_rss_watchdog` true, the last reading ≥ the cap and `last − cap ≤` the largest consecutive-sample increase, and no survivor. Reaching the failsafe fails the test |
| H2 | `…::WatchdogNegativeControl` | the child grows to 64 MiB and exits 0; not killed; `ok` |
| H3 | `…::LiveLimitLinux` (Linux) | the same child under `RLIMIT_AS` = 128 MiB, without the wrapper, ends in `rlimit_abort`. On macOS and Linux each runs its own live limit test; neither is skipped |
| H4 | `…::RlimitOnlyOnLinux` | with a mocked `resource`, the preexec calls `setrlimit(RLIMIT_AS, (C, C))` exactly on Linux, and never on Darwin |
| H5 | `…::Parsers` | recorded `-l` (bytes) and `-v` (KiB) outputs normalized to bytes; `ru_maxrss` per platform; `ps` KiB |
| H6 | `…::Classification`, `…::Aggregation`, `…::Schema`, `…::MetadataHasNoHostIdentifiers` | every class; median and minimum over repeats per stage; the JSONL schema; no host identifiers |
| H7 | `…::PlanAdmission` | `--plan`'s and `--run`'s decisions: every ≥ 10,000-member `dense` and `lane-lu` row refused by name (`never:n2_mode_at_or_above_10000_members`), both CONT n10000 `lane-id` rows refused by name (`never:cont_n10000_identity_lane`), a `lane-id` row whose estimate fails the rule deferred by name (`deferred:estimate_fails_admission`), and no child process started for any of them; the ρ rule; P1's branch; the ceiling run's C |
| I | (A2 record) | `--plan` over the full schedule; `--smoke` |

- **The pytest wrapper** `P/tests/test_performance_harness_runner.py`:
  - it adds `H/runner` to `sys.path` and imports `test_k6_runner`'s `TestCase` classes into its namespace, so pytest collects them;
  - it is under 10 s: the live children take about 2 s together;
  - its only children are the self-limiting Python child (128 MiB watchdog cap; 512 MiB failsafe) and `ps`/`pgrep`;
  - no `test_ci_` prefix; no skip anywhere.
- **The debug-time budget:** `H`'s debug suite time is recorded before (base) and after at A1.
  - E and D at 100 members in **dense** debug mode cost about three dense gate evaluations each at n = 606.
  - I propose E and D on all six 10-member models and the nine in both modes, and all six 100-member models in sparse mode, with dense at 100 members for CHAIN-AX and CONT-ROT only. The final set is fixed by the A1 measurement (N17). An increase over 2 min is reported.
- **Memory in tests:** no dense matrix above 100 members in any test. G1 generates a 10,000-member model (O(m)) and is refused before any n² allocation.

## 12. Mutants (from clean `git archive` copies under `<wt>/k6-mut/<m>/`, at most three at once at `-j 4`; NONE first)

| # | Mutant | Intended kill |
|---|---|---|
| K6-M1 | watchdog never kills (comparison inverted or skipped) | H1 |
| K6-M2 | watchdog compares KiB with bytes | H1 (KiB × 1024 never reaches the cap) or H2 |
| K6-M3 | watchdog polls the `time` wrapper's PID | H1 (the wrapper's RSS never reaches 128 MiB) |
| K6-M4 | SIGKILL to the child only, not the group | H1's no-survivor assertion (the wrapper survives) |
| K6-M5 | `RLIMIT_AS` not applied on Linux, or applied on Darwin | H4 |
| K6-M6 | `-l` bytes read as KiB, or `-v` KiB as bytes | H5 |
| K6-M7 | mean for median, or minimum over the wrong axis | H6 aggregation (a fixture with an odd and an even count, skewed values) |
| K6-M8 | pattern entries from one triangle, or the diagonal counted twice | C1 |
| K6-M9 | identity profile reported as RCM, or the reverse | C3 (and C4) |
| K6-M10 | `dealloc`/`realloc` accounting wrong | F1, F3 |
| K6-M11 | cap off by one (`>=` for `>`) | F2 |
| K6-M12 | the binary's ≥ 10,000 n² refusal removed, or narrowed to `dense` only (so `lane-lu` passes) | G1 (the heap cap then aborts, or the peak exceeds 64 MiB; either fails G1's assertions for the mode concerned) |
| K6-M13 | admission admits an estimate > C/2 | H7; G2 |
| K6-M14 | Q3 transposed in the generator | B1, B3 (and the cross-check) |
| K6-M15 | n − 1 members, or R1's node order not kept (for example CONT interleaved) | B1, B3; C1 |
| K6-M16 | staged sequence skips the contribution audit (contributions `None`) | E1/E2 (`contribution_audit_performed` differs) |
| K6-M17 (own) | `alloc_zeroed` forwarded without accounting | F1 |
| K6-M18 (own) | the basis string's family text changed (the "mixed" branch) | E1 (`symmetry_basis` differs in `Debug`) |
| K6-M19 (own) | ρ taken as the smallest ratio, not the largest | H7 |
| K6-M20 (own) | TREE branch directions swapped (+z at odd k) | B1, B3 |
| K6-M21 (own; RV16 N4) | the runner's by-name refusal covers `dense` only, so `lane-lu` at 10,000 members is admitted | H7 |

- §7.3 item 8 is K1's and is not re-run (brief).
- An allocator abort outside F4 is not a kill. A survivor is reported, never killed by weakening a test.

## 13. Positions on everything still open

- **N1. Stage map refinements** (within Q6(a)).
  - I add three stages to the brief's map:
    - `reduce`, the product's call at `PP@F1b:3510`;
    - `densify`, where SA does it, after geometry, instead of inside `assembly`/`evidence`;
    - `witness`, only after a refused factor, which localizes the O(n⁴) dense path.
  - I also add `stage_begin` lines, so a killed run shows its stage, and print the counts line before the repeats, so a timed-out run keeps its counts.
  - **Ask:** approve.
- **N2. Where the binary lives:** `[[bin]]` with `test = false`, and `[[test]] k6_alloc` with `harness = false` (section 1). **Ask:** approve.
- **N3. Entry stages every repeat by default.**
  - For dense at ≥ 1,000 members and the ceiling run: `--entry-repeats 1`. Each entry is a whole dense gate evaluation, so five would triple the time.
  - The equality is still recorded at every size, in repeat 0. **Ask:** approve.
- **N4. RF-LARGE `y_reference` = P1's rule** (3.1). The brief is silent on it. **Ask:** confirm.
- **N5. Replicated SA private items:**
  - the symmetry basis string, copied verbatim from `SA:1291-1297` (frames only: the "objective welded unreleased straight-frame family" branch);
  - the `FormationSource`, built from its public fields as `SA:1302-1357` does for frames with no curved elements.
  - Both are pinned by E1/E2 and K6-M18. Any drift in SA fails E, which is a stop. There is no SA edit. **Ask:** confirm.
- **N6. K5.**
  - On `28517eaaa`, the selected entry calls a private `constrained_geometry` (with `w4`) instead of the public `geometry`. The staged `geometry` stage calls the public one (`w4: None`).
  - For frame-only models, I expect E to hold. If K5 merges before B, I merge main in, re-run E and the geometry and entry observations, and state which code each figure measures. If E fails there, I stop. **Ask:** noted.
- **N7. Counts-only invocations above 100 members before the schedule is approved.**
  - `--counts-only` at 1,000 and 10,000 members does O(nnz) work (about 100 MB at most), runs no solve and no n² allocation, and runs under a 512 MiB heap cap.
  - I propose running them at A1–A2 (one at a time, not on a timed slot) to fill `counts.jsonl` and feed the oracle, so `--plan`'s admission estimates are exact.
  - **Ask:** confirm these are not "runs above 100 members" under Q13. If not, `--plan` uses the Python closed forms and full-block bounds, and exact counts come at B.
- **N8. Two peak models** (5). The cap is enforced on the in-place model, as the gate's probe does. The F1b ratios are reported against both peaks and RSS; ρ uses `max(RSS, move)`. **Ask:** approve.
- **N9. Dense admission estimate = 96n² + E_base.** With F1b's bare 96n², the ratio at 66 DOFs would carry the O(nnz) state (roughly 25% at n = 10) up to the 8,190-DOF ceiling run and could refuse it under Q3's letter. The claim ratio stays against F1b's bare 96n². **Ask:** approve.
- **N10. The two known dense timeouts** (CHAIN-ROT and TREE-AX at 1,000 members; P1, and gate part 2).
  - **My reading of the code, a hypothesis only:** the dense Cholesky screens each pivot with an operation count 2j + 2 that grows with the row (`FK/structural.rs:1834`). The skyline uses 2(i − first_i) + 2 (`:1982-1988`), so the dense screen can refuse a pivot the skyline passes. A refused dense factor then runs `negative_pair_witness`: O(n_f²) pairs, each an O(n_f²) check (`:2166-2186`). That is about 10¹⁴–10¹⁵ operations at n_f = 6,000, and never finishes in 1,800 s.
  - With K6's `factor`/`witness` stage lines, the kernel run will show it. The prepare-stage memory is still recorded before the kill.
  - If the dense `factor` refuses where the sparse one publishes, that is **an outcome-class divergence between modes**, a stop under the brief.
  - **Ask:** rule in advance whether, for these two known cases, I record it and finish the running tier before stopping (my recommendation), or stop at once.
- **N11. The slot, and Q7.** Section 8.3 projects about 3.5–5 h for B without the ceiling run, with T3b alone at about 2–3.5 h. **I recommend:**
  - **(a)** split B into B1 (T1, T2, T3a, T4, T5) and B2 (T3b), in two slots;
  - **(b)** in T3b, `--repeats 1` for the two N10 cases and for lane-lu. Peak requested heap is deterministic, so one repeat gives the memory, and their timing is context only.
  - The measured projection is still reported before T3b, with the two-hour stop kept.
- **N12. K1's table is a partial cross-check** (4.3 item 2): full for stored, lower and free lower; nonzero and profile for CHAIN-AX only. **Ask:** accept.
- **N13. The Python generator in the runner** is a second, independent implementation (3.4). It carries B3 and the closed forms without a Rust build. **Ask:** accept its placement inside `k6_runner.py`. The alternative is a separate `H/runner/k6_models.py`, a one-file write-set addition.
- **N14. Q5 ladder:** 16×16, 32×32, 64×64 and 96×96, then 128×128 only if admitted at 96×96's measured ρ and projected under about 20 min. RCM half-bandwidth on these grids grows as about 6s, so the largest profile is about 31M (96) or 75M (128) entries. **Ask:** approve.
- **N15. DEC-025:** section 9. Cannot be settled without the A2 run.
- **N16. Linux:** the live Linux test (H3) uses `RLIMIT_AS` with no `time` wrapper, so a Linux sweep host without GNU `time` still runs it. The runner itself requires `/usr/bin/time` for observations, which only the Mac runs (Scope 6). Disclosed.
- **N17. The debug-time test set** is fixed at A1 from measurement (11). A dense test at 100 members is not skipped: it is either in the set or not written.
- **N18. Quiet-host guard in `--run`:** no cargo, no sweep, and `memorystatus_level ≥ 80` before each run, otherwise wait. The memguard log is checked after each tier. **Ask:** approve the threshold of 80. G1 recorded 94–96 on a quiet host (`T3/GATE_BASELINE_MAC_E7D930D49/README.md:109`).
- **N19. The lane-id entries.**
  - `lane-id` reproduces PP's private `assemble_reduced_sparse_entry_system`/`append_reduced_element_entries` (`PP@F1b:5646-5823`; the same function is called at `PP:4464`, `:4601` on the base) for frames and restraints only, as a documented copy. PP cannot be a dependency: it pulls registry crates.
  - **Checks:** the lane's own `original_profile_entry_count` equals K6's F1b-rule count at every lane run (a `parity` line), which F1b's ruling also required of its estimate. CONT's full-block form reproduces F1b's 675,179,982.
  - **Ask:** accept the copy.
- **N20. Disclosures carried to RETURN:**
  - the section's 9 ulp against 0.0019π (3.2);
  - `/bin/ps` setuid (9);
  - no Linux observation; the constructed `-v` fixture;
  - that the only Linux peaks are P1's product-level ones.

**Stops so far: none.** Nothing found on the base contradicts the brief. N10 is a predicted stop condition to rule on before B, not a stop now.
