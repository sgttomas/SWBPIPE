# I17 return: slice V-K (the VP-ROBUST kernel lane, `numerical_robustness`)

> **D, with B done.** KF1 (PR #1056) has merged, and V-K is on the new main
> (ROOT's merge `485320e95`), with the post-KF1 re-run done (§12.4). B ran in
> ROOT's slot, 16:26:11–16:27:28 (§14).
> **V3's figures (10,000 members) are marked pre-KF3.** Five of the six
> 10,000-member models end `Unresolved(ExactSumSpan)`, the KF3 availability
> finding. V3 re-runs after KF3 merges, and those figures are replaced then.

- **Brief:** `T3/TASK_BRIEFS/I17_VK_IMPLEMENTATION.md`. As dispatched it had sha256 `06b16b82…`, committed at `306e225fe`, read at the numerics head.
- **Branch:** `codex/piping-vk-20260929`, from main `ab02ee3a6` (K4 merged). The checked head is `485320e95`: ROOT's merge of main `0f5d8c7b4` (KF1) into the D draft `24449b5c8`. The runner and README edits of §12.4 are not yet committed.
- **Platform:** `aarch64-apple-darwin` (macOS 26.6.2, arm64), with rustc and cargo 1.97.1 (`_run_records/a1/toolchain.txt`).
  - **Every observation is Mac-only.**
- **Observations only.** No test or record asserts a threshold, a time or a memory bound. V-K records outcomes and work. ROOT sets the limits.
- **Paths:** records use the placeholders `<wt>`, `<scratch>` and `<home>`.
  - `P/` = `projects/chirality-piping/`.
  - `T3/` = `P/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/`.
  - `FK` = `P/core/solver/frame_kernel/`, `SD` = `P/core/solver/sparse_direct/`, `K4R` = `FK/src/structural/retained/`, `K4T` = `FK/tests/retained_k4/`.
  - `VR` = `P/validation/benchmarks/numerical_robustness/` (new).

## 1. Brief, basis, delegation and rulings

- **Delegation.**
  - ROOT (HELP_HUMAN, the chirality-piping session) dispatched I17 directly as a Type 2 TASK. It used the host's native background-subagent mechanism (D-GOV-35), with the brief as the assignment.
  - ROOT sent every ruling, pause, resume and slot as an in-session message.
  - I17 delegated nothing, since Type 2 does not delegate. It made no Git write and no index operation.
  - The session continued across context compactions. Each continuation carried a written summary of the earlier work, and the work went on from it.
  - **Scopes and enforcement:**
    - the scope was the brief's write set and host rules, together with ROOT's rulings and slot holds;
    - the host enforced its session permission system only. It did not restrict writes to the write set, block Git, or cap memory. I17 kept to those rules itself, and the memory guard ran throughout;
    - returns went to ROOT at each checkpoint, through the subagent return path.
  - **ROOT made every commit:**
    - `40421b7f1`: checkpoint 0, the plan;
    - `3018343c2`: K6b's A0 export, cherry-picked by ROOT, with the same patch-id as `bb89e4f8f`;
    - `37bff1780`: A1;
    - `c1fea8574`: A2;
    - `e24e911e6`: C;
  - `24449b5c8`: D, the draft;
  - `485320e95`: main `0f5d8c7b4` (KF1) merged in, with no conflict.
- **Basis:**
  - D1 `DESIGN.md` revision 5a.2 (`fb62ef4a…`):
    - §4.10, "W5 — the VP-ROBUST harness" (`:850`), with its kernel lane (`:857`);
    - §4.8's parity items 1–3;
    - §4.1.6.1's S\* and R;
    - §7.3's mutation list;
    - §6's V-K row.
  - The W1a method as K4 implements it, revision 5a.3 with amendment A1 (K4 `RETURN.md`, §12, §16 and §17).
  - R1's frozen references, read-only and hash-pinned by the generator:
    - `REFERENCES/references.json`, `7b176dbb…`;
    - `REFERENCES/references.py`, `80d473a7…`;
    - D1's `DESIGN_NUMERICS/_run_records/floor_kinds.json`, `83265615…`.
  - R7 §7's mutants, for the seeded faults.
  - The checkpoint-0 plan, `IMPLEMENTATION/VK/PLAN_CHECKPOINT0.md` (`adaf697a…`).
  - `_COMMON.md`, and the Mac host rules of `I8R_K1_RESUME.md:24-50`.
- **Rulings** (`ROOT_RULINGS_V1.md`):
  - "K6b and V-K: spawn";
  - "V-K: rulings on I17's checkpoint-0 plan" (Q1–Q8, C1–C12);
  - "K6b: rulings on I16's checkpoint-0 plan, and A0's export";
  - "K6b A0 accepted; the export is on both branches";
  - "V-K: rulings on I17's A1 stop (THIN-A and THIN-B)";
  - "V-K: THIN confirmed by GEN; the expected-unresolved list stands";
  - "V-K: A1 accepted";
  - "V-K: A2 accepted";
  - "V-K: C's harness mutants; three tests added";
  - "V-K: C accepted".
  - Messages ROOT sent in the session and that are not yet in `ROOT_RULINGS_V1.md` are cited where they are used. For example, B waits for KF1.

## 2. Files and line counts (against the export base `3018343c2`)

**V-K's product-tree change is 73 files and 41,086 lines, all insertions.**

| Path | Lines | What |
|---|---:|---|
| `FK/Cargo.toml` | +6 | the `mutation-controls` feature, empty, with no dependency change |
| `K4R/seeded.rs` (new) | 126 | the fault selector, cfg-gated |
| `K4R/{adaptive,assemble,bound,factor,ledger,mod,recover,source,verify}.rs`, `FK/src/structural/sparse.rs` | +180 | the cfg-gated fault sites and their comments |
| `VR/Cargo.toml`, `VR/Cargo.lock` | 19, 118 | the crate; the `seeded-faults` feature |
| `VR/README.md` | 117 | |
| `VR/src/*.rs` (11 files) | 2,820 | `cases` 569, `exact` 676, `lane` 366, `parity` 305, `invariance` 230, `floor` 198, `compare` 158, `records` 137, `sha256` 93, `rcm` 60, `lib` 28 |
| `VR/tests/*.rs` (8 files) | 1,067 | `files` 322, `lane` 273, `feature_guard` 137, `adapter` 100, `rcm` 79, `engine` 78, `parity` 54, `invariance` 24 |
| `VR/examples/vk_records.rs` | 141 | regenerates the committed records (release) |
| `VR/cases/gen_vk_cases.py` | 714 | the generator, standard library only |
| `VR/runner/*.py` | 737 | `run_harness_mutants` 432, `run_seeded_faults` 216, `check_fault_sites` 89 |
| `VR/cases/*` (data) | 2,990 | 10 family files, the lists, the engine vectors, `large_models.sha256`, `SHA256SUMS` |
| `VR/observations/**` (data) | 32,051 | the per-case records, parity, invariance, the kill matrix and the harness matrix, each with `SHA256SUMS` |

- **FK:** the diff is insertions only. It adds 186 lines to 11 existing files (`Cargo.toml` and 10 sources) and the new 126-line `seeded.rs`. Every inserted code line sits behind `#[cfg(any(test, feature = "mutation-controls"))]` (§12.1).
- **Records:** everything else is under `T3/IMPLEMENTATION/VK/` (§19).

## 3. Scope, item by item

1. **The crate.**
   - `VR` has its own `Cargo.lock`. Its dependencies are FK, SD, and `serde_json` with `float_roundtrip`.
   - It does not depend on `product_physics`: by ruling Q1, V-P adds it.
   - `VR`'s `seeded-faults` feature is never a default, and it is the only thing that enables FK's `mutation-controls`.
2. **The case adapters.**
   - The generator's path A (`cases/gen_vk_cases.py`) and `Model::source_parts` (`src/cases.rs`) together turn R1's `references.json` into K4's `SourceParts`.
   - Path B builds K4's canonical source bytes independently, from `references.py --model`'s JSON.
   - Each case file line carries path B's `k4src_sha256`, and the tests require `sha256(PrimitiveSource::encoding())` of path A's model to equal it on all 201 CI models (§10).
   - The generator pins R1's three inputs by sha256. `--check` regenerates in memory and compares bytes (Q2).
   - No CI test reads `execution/`, and the feature guard checks that no VR source names it.
3. **The kernel lane.**
   - It runs every R1 case of RF-CHAIN, RF-SKEW, RF-WEAK, RF-LARGE (10 and 100 members in CI), RF-INVARIANCE, RF-RANGE, RF-ZERO, RF-FINITE, RF-MECH and RF-CANCEL through `solve_case`. `CaseLimit` and `InvocationMeter` are both `u64::MAX`, and each record carries them.
   - RF-CANCEL's three UDL cases are W1b's and are excluded by the generator.
   - The 22 `needs_directional_spring` cases use K4's `DirectionalSpring`.
   - The binary64 sparse gate runs RF-LARGE and RF-MECH for §4.8's parity items (§8).
4. **What is compared.**
   - The quantities: displacements and rotations; the six signed reaction components; spring actions; N, T, and the bending magnitude at the ends and at the mid-station.
   - **The magnitude** is decided exactly, without forming the square root (C8).
   - **Twist and extension** are derived as T/k_t and N/k_a, never differenced (C1 for LEF).
   - **The method evidence** is the outcome, the precisions, the attempts and the work. It is compared through the committed per-case records.
   - **RF-MECH:** the eight mechanisms are refused with a witness and publish no rows. RF-MECH-K0's literal k = 0 spring is refused by the source, naming it (C10).
5. **The predicate.**
   - It is `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, unchanged, decided exactly on R1's decimal strings by the in-crate bignum engine (`src/exact.rs`).
   - **The represented basis** applies on the two cases R1 marks, RF-SKEW-A-CANT-AX-122-r1e-12 and RF-FINITE-THIRTIETHS-O1e6.
     - The generator takes `expected_represented` there. It refuses a case whose basis and `finite_input` flag disagree.
     - VK-H8 shows this is live (§13).
6. **The floor check and the reporting.**
   - R = 2^-34 exactly, and S\* is computed in binary64 by §4.1.6.1's items 4–6. V-K's S\* equals K4's exported `coupled_scales` and `body_extent` bit for bit on every CI case.
   - Twist and extension use variant F.
   - `covered` is decided exactly, with no rounding of R·S\*.
   - The report keeps passes, absolute-range passes and not-covered comparisons as three separate numbers. Structural zeros and expected-unresolved rows are counted apart.
   - **The committed `not_covered` list** is 51 entries: RF-WEAK 46, RF-CANCEL 3, RF-SKEW 2. The lane requires each case's set to equal it (§5).
   - **`pass_absolute_range`:**
     - CI's pinned count is 0 (C11), because the five sub-range rows belong to RF-LARGE-CONT-n10000.
     - The path is CI-tested on R1's five rows by `tests/engine.rs` (C's ruling).
     - At B (pre-KF3), RF-LARGE-CONT-n10000-AX gives 4 `pass_absolute_range`: `u.C2500.UZ`, `u.C4999.UZ`, `R.S1250.UZ` and `Mb.A3750.i`. The fifth sub-range row, CONT-n10000-ROT's `Mb.A3750.i`, is in a case that ends `Unresolved(ExactSumSpan)`, so it counts as `unresolved_availability` (§14).
7. **The discrimination check.**
   - All 506 discriminating controls of the CI cases fail the predicate: 481 value controls and 25 outcome controls.
   - The 171 non-discriminating controls are reported, and none fails unexpectedly.
   - The 38 discriminating controls of the 12 large cases also fail, from R1's values alone.
8. **RF-RANGE.**
   - LEF-small and LEF-large are solved and pass on all three bases (CHAIN, SKEW, CONT). So K4's source admits LEF-small, which answers `ROOT_RULINGS_V1.md`'s stale item 10.
   - No RF-RANGE case is refused by the source.
   - THIN-A and THIN-B end `Unresolved(Ceiling)` and are on the expected-unresolved list (§6).
9. **The seeded faults.**
   - 15 faults sit behind FK's `mutation-controls` feature. All are killed, with NONE first (§12).
   - The feature guard is `VR/tests/feature_guard.rs`, which CI runs.
10. **RCM equality.** K4's `reverse_cuthill_mckee` equals SD's on all 201 CI models' free–free adjacency and on 404 seeded synthetic graphs (§11).
11. **Per-case records.**
    - `VR/observations/kernel_lane/<family>.json` (`vk-case-record-v1`, with `SHA256SUMS`) holds the outcome, the selected and verification precisions, every attempt with its role, outcome, gate, corrections, own, shared and verification work, its stage work, and its storage counts.
    - The lane tests compare the records with the run. They never write them.
    - **Time:** the CI-scale release run's per-case times are in `_run_records/a1/vk_records_release_2_per_case.err`, without the load (disclosure 5). With the load: B's records give each scale run's time with the 1-, 5- and 15-minute load before and after (§14).
12. **Scale runs:** done in B (§14). V3's figures are pre-KF3.

## 4. Checkpoint-0 positions and conflicts, as ruled

- **Q1:** V-P adds `product_physics`.
- **Q2:** a generated, committed cases file with R1's sha256 pinned and `--check`.
  - Expected values are R1's decimal strings, byte for byte.
  - The engine is an in-crate bignum with no registry dependency. It is tested against Python's `fractions` on 2,428 sampled vectors (`cases/engine_vectors.jsonl`) and by its own unit tests.
  - The 1,000- and 10,000-member models are generated on demand, with their sha256 committed (`cases/large_models.sha256`).
- **Q3:** CI runs every case up to 100 members. RF-LARGE at 1,000 and 10,000 members runs as examples. The CI cost is in §15.
- **Q4:** the seeded faults are as approved, plus VK-F05 by the THIN ruling, 15 in all.
  - F06, F07 and R28 may be killed on evidence (O5's condition). F17's extra check holds: a not-covered row must be `absolute_verified`, except the three input-derived rows.
  - The selector and its `mod` line count as fault sites. An unknown id panics.
- **Q5:** the scale runs use V-K's own models, one release process each, under K6's runner, with the heap cap at C − 512 MiB and the ascent 100 → 1,000 → 10,000. Slot per tier, and 10,000 only as ROOT approves. As run: V1, V2 and V3 in one slot, with 10,000 members approved (§14).
- **Q6 and Q7:**
  - The one-off bit comparison with K4's adapter output is recorded (§10).
  - Bit identity is asserted for list permutations only (§9).
- **Q8:** the export (K6b's A0) covers V-K. V-K imports 28 of its names (§16.5), and no FK visibility change is V-K's.
- **C1:** k_t and k_a use the exact power-of-two pre-scaling where §4.10's fl(G·J) is not normal.
  - The pre-scaling is the RF-RANGE LEF cases: fl(G·J) is 2^-1078, which rounds to 0, on LEF-small, and 2^1122, which overflows, on LEF-large.
  - **The derivation:** choose s = −(e(a) + e(b)) so that a·(b·2^s) lies in [1, 4). Then form fl(fl(a·b·2^s)/L) and unscale by 2^-s exactly.
    - A rounding to nearest commutes with an exact power-of-two scaling whenever both results are normal. So wherever the design's intermediates are normal, the two forms agree bit for bit.
  - **The test** `c1_the_prescaled_coefficients_equal_the_designs_wherever_it_is_defined` compares 2·2,977 − 32 = 5,922 coefficients bit for bit. The formula is undefined only for k_t on the 6 LEF cases: LEF-small and LEF-large on three bases, 32 members in all (16 per LEF vector: 5 + 1 + 10).
- **C2:** §7.3's items 3, 16 and the relabelling half of 10 are not seeded. Each is cited to K4's killing mutant (§12.3). Item 5 became VK-F05.
- **C3:** item 7 is killed on evidence. The design's "recovered" is recorded as inexact: the mechanisms end `Unresolved`, and none is published.
- **C4:** parity is §4.8 items 1–3, at up to 100 members.
- **C5:** V-K's scale runs use R1's own rounding and are separate from K6b's.
- **C7:** no site is in the five byte-identical `factor.rs` functions or in `bound.rs`'s shift loop. K4's source scan and the S11 site table ran before any site was written, after, and after rustfmt. None was flagged.
- **C8:** `hypot` is decided exactly.
- **C9:** "compute it both ways and report" applies to the n ≥ 1,000 floor sets: S from R1's sampled rows and from all rows.
  - At B both sets are empty on all 12 models at 1,000 and 10,000 members.
  - The sampled set equals the committed (empty) list, and the S_full set is `[]` (§14).
- **C10:** RF-MECH-K0's k = 0 spring is omitted by V-K's adapter, and the case is refused by geometry. The literal model is refused by the source as `NonPositiveSpring { id: 1 }`, which names the spring (`tests/adapter.rs`).
- **C11 and C12:** recorded.
  - C12: the ordinary route's `AXIS_TOLERANCE` refuses nine RF-RANGE cases. That is for V-P (§16.4).

## 5. The kernel lane at CI scale (201 cases; `tests/lane.rs`, pinned)

| Family | Cases | Selected / refused / unresolved | Rows | Pass | Not covered | Structural zero | Expected unresolved | Fail | Controls discriminated / non-discriminating |
|---|---:|---|---:|---:|---:|---:|---:|---:|---|
| RF-CHAIN | 30 | 30 / 0 / 0 | 2,760 | 2,700 | 0 | 60 | 0 | 0 | 82 / 38 |
| RF-SKEW | 36 | 36 / 0 / 0 | 1,080 | 982 | 2 | 96 | 0 | 0 | 105 / 33 |
| RF-WEAK | 9 | 9 / 0 / 0 | 618 | 566 | 46 | 6 | 0 | 0 | 23 / 13 |
| RF-LARGE (10, 100) | 12 | 12 / 0 / 0 | 9,054 | 9,054 | 0 | 0 | 0 | 0 | 38 / 0 |
| RF-INVARIANCE | 25 | 25 / 0 / 0 | 7,096 | 7,056 | 0 | 40 | 0 | 0 | 47 / 7 |
| RF-RANGE | 32 | 30 / 0 / 2 | 2,720 | 2,590 | 0 | 80 | 50 | 0 | 80 / 12 |
| RF-ZERO | 4 | 4 / 0 / 0 | 158 | 158 | 0 | 0 | 0 | 0 | 9 / 0 |
| RF-FINITE | 6 | 6 / 0 / 0 | 748 | 748 | 0 | 0 | 0 | 0 | 6 / 6 |
| RF-MECH | 9 | 1 / 8 / 0 | 26 | 26 | 0 | 0 | 0 | 0 | 26 / 0 |
| RF-CANCEL | 38 | 38 / 0 / 0 | 1,444 | 1,441 | 3 | 0 | 0 | 0 | 90 / 62 |
| **Total** | **201** | **191 / 8 / 2** | **25,704** | **25,321** | **51** | **282** | **50** | **0** | **506 / 171** |

- **No comparison on a covered row fails.** No case is refused by the source, and no absolute-range pass occurs in CI.
- **Every selected case is selected at 128 and verified at 256.**
  - 189 of the 191 needed no correction. RF-SKEW-A-CANT-OFF-122-r1e-08 and -r1e-12 needed one each.
  - 42 of the 197 verifications took one shifted factorization. The 197 are the 191 selected cases' and THIN's three each.
- **The not-covered set equals the committed list on every case.**
  - Every not-covered row's published row is `absolute_verified` or unpublishable (F17's extra check).
  - The exceptions are the three input-derived rows ruled at A1: RF-WEAK-W-AX-rho1e-12 `u.N5.UX/UY/UZ`, displacements at a restrained node published as their exact prescription (rule 2a). They are pinned in the lane test.
- **Structural zeros** are spring components that cannot exist: the off-axis components of a global-axis spring, or a force on a rotational spring and a moment on a translational one. R1's value there is exactly 0 in all 282, and `tests/files.rs` checks it.

## 6. THIN-A and THIN-B (the A1 stop, and the expected-unresolved list)

- **The finding.** RF-RANGE-THIN-A and THIN-B use R1's PHYS-R4 geometry: OD 4e-77 m, L = 1 m, and EA/(12EI/L³) ≈ 6.7e152 ≈ 2^507.67. They end `Unresolved(Ceiling)` with no rows:
  - 128 is rejected by the stop rule;
  - 256 is rejected by the stop rule;
  - 512 is rejected by the charge test (d);
  - 1024 verifies, and nothing lies above it.
- **Not the adapter:** the probe with K4's y_reference gives the same chain (`_run_records/a1/thin_probe.*`).
- **GEN confirms it.** K4's generator, `gen_k4_vectors.py`, with its own adapter and G stated exactly, gives the same chain: rejections at rows 7, 14 and 14, then the ceiling (`_run_records/a1/gen_thin_confirm.*`).
- **Ruled as W1a's coverage limit.**
  - `VR/cases/expected_unresolved.json` lists THIN-A and THIN-B with the reason and log2 of the spread, 507.670.
  - A listed case must end unresolved with no rows. Its 25 rows each are counted as expected unresolved, never as passes.
  - A case leaving or joining the list fails the gate. VK-F06 and VK-H13 show both directions are live.
- **Work:** THIN-A's charged work is 4,442,076 LME over 4 attempts (§16.1).
- **Routed by ROOT:**
  - THIN's product standing, and what F2a publishes for a W1-unresolved case, to F2a and V-P;
  - the fact, to the owner's PHYS-R4 decision;
  - W1 beyond 512 bits, to D1 and the T3-close list.

## 7. RF-RANGE findings (Scope 8)

- **All 30 non-THIN cases are selected at 128 and pass.** They cover L±240, E−1000, E+960, F±960, LEF-small, LEF-large, SIM-a and SIM-b on CHAIN, SKEW and CONT.
- **There is no range refusal:** K4's `PrimitiveSource` admits LEF-small and LEF-large. That answers stale item 10.
- **C1's pre-scaled k_t** is used exactly on the six LEF cases (§4).
- **For V-P (C12):** the ordinary route's `AXIS_TOLERANCE` (1e-12 m) refuses nine RF-RANGE cases: L-240, LEF-small and SIM-a on the three bases. The kernel lane is unaffected.

## 8. Parity: the binary64 sparse gate against dense (§4.8 items 1–3; `observations/kernel_lane/parity.json`)

| Models | Bitwise K (item 1) | Class (item 2) | DEC-053 basis (item 3) |
|---|---|---|---|
| RF-LARGE, 6 frames at 10 members | equal | Passed in both | within: at most 0.038 of the allowance (CHAIN-n00010-ROT) |
| RF-LARGE CONT-n00100-AX and -ROT | equal | Passed in both | within: at most 4.0e-5 of the allowance |
| RF-LARGE CHAIN- and TREE-n00100, AX and ROT | equal | Sensitive in both | recorded, not asserted (§4.8 item 3 applies to Passed publications): 2.23, 345, 14.3 and 11.2 allowances |
| RF-MECH, 8 mechanisms | equal (7 with dense; LINE-IN-CHAIN1000 sparse only) | NumericallyUnresolved in both | none published |
| RF-MECH-LINE345-RX-COMPANION | equal | Passed in both | within: 4.0e-7 of the allowance |

- **CI** (`tests/parity.rs`) asserts items 1 and 2 on the 12 RF-LARGE frames and the six RF-MECH models of up to 100 members. It asserts item 3 where both modes are Passed.
- **RF-MECH-DISC-CHAIN100 and -SPRING** (103 members) are recorded by `vk_records` only. The binary64 gate's dense witness takes about 300 s each in release (`_run_records/a1/mech_parity_time.*`). That is K6's N10 cost (KF2), not a new finding.

## 9. Invariance (Q7; `observations/kernel_lane/invariance.json`)

- **List permutations are asserted bit for bit** (`tests/adapter.rs`):
  - reversing every list of a source changes no canonical byte, on all 201 CI models;
  - it changes no published bit (rows and evidence) on the 21 RF-INVARIANCE cases below 100 members.
- **Cross-variant differences are recorded, not asserted.** Each variant is gated by its own R1 expectations.
  - The offsets (OFF-1e3 and OFF-1e6) are bit-identical on every row, on all four bases.
  - RELABEL is bit-identical on PINTOR and LFRAME. On CONT4 it is 127 of 128 rows (largest difference 4.3e-30 of the allowance); on TREE100, 1,063 of 1,312 (7.4e-25). These are the roundings that follow from RCM's order.
  - ROT-Q3, ROT-Q9 and UNITS-mm are not bit-identical. Their largest difference is 1.5e-7 of the allowance.
  - Every row maps: 0 unmatched.

## 10. The adapter checks (Scope 2, Q6)

- **K4SRC:** path A's source bytes equal path B's on all 201 CI models (`tests/adapter.rs`).
  - VK-H10a, b and c (node order, a dropped member, G := E, all in the Rust half) are each killed by it, and so is VK-H10d (the Python path A's y_ref tie rule). That shows path B does not share path A's code (§13).
- **The one-off comparison with K4's adapter output** (`K4T/r1_cases.txt` and `r1_large.txt`; `_run_records/a1/compare_k4.json`) covers 140 cases, with 0 unexpected differences. Every difference is on plan §4.4's list:
  - y_reference, 120 cases: V-K's rule is the smallest chord component, ties to the lowest index;
  - spring representation at mixed nodes, 6: V-K uses 1 global-axis spring where K4 uses 0;
  - the k = 0 spring, 1: omitted by V-K, kept by K4 (C10).
- **The 12 large models** (1,000 and 10,000 members) are generated on demand. Their model and K4SRC sha256 are committed in `cases/large_models.sha256`. At B's setup all 12 model files and all 12 K4SRC digests match (§14).

## 11. RCM equality (K4's brief Q7; `tests/rcm.rs`)

- K4's `reverse_cuthill_mckee` (exported) and SD's give the same order:
  - on all 201 CI models' free–free adjacency (members' 12×12 blocks, spring diagonals, and directional springs' 3×3 node blocks, restrained DOFs removed);
  - on 404 seeded synthetic graphs: a path, a star, a grid, an empty graph, and 400 random graphs with self edges, duplicates and one-sided edges.

## 12. The seeded faults and the kill matrix (A2; `VR/observations/seeded/`)

### 12.1 The feature and the sites

- **The feature.** `FK/Cargo.toml` declares `mutation-controls = []`. `VR`'s `seeded-faults` is the only thing that enables it, and `tests/feature_guard.rs`, run in CI, checks that:
  - no other manifest under `P/` names the feature;
  - no dependency table enables it;
  - no `default` list includes it;
  - CI's cargo command passes no features.
- **Every site is gated** by `#[cfg(any(test, feature = "mutation-controls"))]`.
  - The selector is `K4R/seeded.rs`. It reads `FK_SEEDED_FAULT` once. Unset, empty or `NONE` means no fault, and an unknown id panics.
  - `VR/runner/check_fault_sites.py 37bff1780` strips the gated items and finds only added comment lines. So with the feature off and outside `cfg(test)`, FK is its base code.
  - After the merge, `check_fault_sites.py 0f5d8c7b4 --allow-commit 3018343c2` checks against main while allowing only A0's own patch lines, which are on this branch but not yet on main. It reports OK (§12.4).
  - No site is on KF1's `ExtremeTracker` lines, as ROOT required.
- **FK's suite with the variable unset passes 394,** the baseline count, and 402 after KF1's merge (§12.4).
  - K4's source scan and the S11 site table pass. `seeded.rs`, which neither scans, was checked by hand against both pattern sets.

### 12.2 The matrix (NONE first)

At A2 it ran VR's 40 tests. The post-KF1 re-run ran 43 and matched A2 row for row (§12.4). The committed `observations/seeded/kill_matrix.jsonl` is now the post-KF1 run, and A2's is in `_run_records/a2/`.

| Fault | Maps to | The fault | Verdict | Killed by |
|---|---|---|---|---|
| NONE | – | – | pass (40 of 40) | – |
| VK-F01 | §7.3-1 | the assembled K promoted from binary64 | killed | outcome (cases not selected); the floor list |
| VK-F02 | §7.3-2 | u rounded to binary64 before recovery | killed | value; evidence |
| VK-F03 | §7.3-4 | the smallest diagonal contribution dropped | killed | outcome; the floor list |
| VK-F04 | §7.3-4 | e_x's first two components swapped | killed | value; outcome; the floor list |
| VK-F05 | §7.3-5 | no escalation after a rejection | killed | evidence (THIN's attempts) |
| VK-F06 | §7.3-6 | accepted without the verification's verdict | killed | outcome (THIN selected at 128, off the list) |
| VK-F07 | §7.3-7 | the geometric mechanism check disabled | killed | evidence; outcome (RF-MECH 8 refused → 8 unresolved; none published) |
| VK-F08 | §7.3-8 | member 1's coupling block left out, sparse mode only | killed | parity (both bitwise-K tests) |
| VK-F10 | §7.3-10 (list order) | the canonical sort skipped | killed | bit (the canonical bytes, the permutations) |
| VK-F13 | §7.3-13 | loads folded at p = 128 in listed order | killed | value (`rf_cancel`) |
| VK-F17 | §7.3-17 | every scaled row `relative_verified` | killed | class; evidence |
| VK-R02 | R7-M2 | ê uncoupled | killed | outcome (RF-INVARIANCE-TREE100 ×4, RF-LARGE); evidence |
| VK-R28 | R7-M28 | no shift (Uc alone) | killed | evidence (availability, below) |
| VK-S1 | directional springs | k·n·nᵀ without the division by nᵀn | killed | value (`rf_skew`, `rf_invariance`) |
| VK-S2 | NC-SIGN | reactions with the opposite sign | killed | value |
| VK-UNKNOWN | – | an unknown id | panics as required | – |

- **VK-R28's release check on RF-LARGE:** every one of the 9,054 rows still passes, but the selected precisions rise.
  - CHAIN-n00100-AX goes to 512 and TREE-n00100-AX to 256, as R7 §7 predicts.
  - CHAIN- and TREE-n00100-ROT go to 512, and CONT-n00100-ROT to 256.
  - CONT-n00100-AX and the 10-member frames stay at 128.
- The kill kinds are read from the test output by a heuristic, refined after the live run and rebuilt from the same logs (`--from-logs`). The failing tests are exact.

### 12.3 The §7.3 kernel-lane items not seeded (C2; `observations/seeded/evidence.json`)

| Item | Why V-K cannot discriminate it | K4's killing mutant |
|---|---|---|
| §7.3-3 (drop K_fc u_c) | R1 has no nonzero prescribed motion | D3: `the_reduced_rhs_enters_prescribed_columns_exactly_and_rounds_once` |
| §7.3-10, the relabelling half | W1's answer does not depend on the order within its claim; the list-order half is VK-F10 | D10: `retained_states_equal_the_generators_emulation_of_the_method_bit_for_bit`, `e_unit_g_…`; also V-K's RCM equality |
| §7.3-16 (sequential addition at p) | R1's smallest ratio is 1e-12, not the control's 2^-300 | D16a (DUP, ASMBITS), D16b (R_FOLD) |

- The other §7.3 items are outside the kernel lane: 9, 11, 12, 14, 15, 18–20 and 23–32. Items 21 and 22 are harness mutants (VK-H5, VK-H3). R7's other mutants need constructed controls outside R1, and K4's controls kill them.

### 12.4 After KF1: the merge and the kill-matrix re-run (`_run_records/kf1_merge/`)

- **The merge.** ROOT merged main `0f5d8c7b4` (KF1, PR #1056) into the V-K branch as `485320e95`, with no conflict.
  - `adaptive.rs` merged automatically, with V-K's three fault sites and KF1's bounded trackers both present. In base coordinates KF1's hunks are at lines 542–595, 999–1852 and 3152–3156; V-K's sites are at 409, 2793 and 2830.
  - The merge changed no VR path. So the harness matrix (§13) is not re-run.
- **`check_fault_sites` against the new base** (`0f5d8c7b4`, `--allow-commit 3018343c2`): **OK.**
  - Every changed FK source, with the gated items removed, equals main apart from added comment lines and A0's own patch lines, with 0 other differences.
  - A0 accounts for 525 lines, for example 281 in `adaptive.rs`, 159 in `source.rs` and 25 in `structural.rs`.
  - The option is new in this step. It allows each line of that commit's patch at most as often as the patch has it.
- **The scans:**
  - the S11 site table passes 3 of 3, with KF1's added row;
  - K4's source scan passes;
  - FK and VR compile without warnings with the feature off, with it on, and for tests and examples;
  - the merged `adaptive.rs` and `seeded.rs` are rustfmt-clean.
- **FK's full suite with the variable unset passes 402, with 0 failed.** The lib goes from 328 to 336 (KF1's tests), and every integration binary's count is unchanged.
- **VR's suite passes 43 of 43.** The lane tests compare the committed per-case records byte for byte, work counts included. So **KF1 moves no CI case's evidence or work**, as ROOT's KF1 ruling expected at T = 512.
- **The kill matrix (668 s):** NONE passes 43 of 43, all 15 faults are killed, and the unknown id panics. Each row's verdict, failing tests and kill kinds equal A2's. Only `tests_run` changes, from 40 to 43, because the three `tests/engine.rs` tests use constructed inputs and no fault reaches them.
- **VK-R28's release check** (rebuilt in release on the merged tree) is identical to A2's:
  - NONE selects all 12 RF-LARGE frames at 128;
  - under VK-R28, CHAIN-n00100-AX goes to 512 and TREE-n00100-AX to 256, the two ROT frames to 512 and CONT-ROT to 256, and all 9,054 rows pass.
- **Changed in this step:**
  - `VR/runner/check_fault_sites.py` (the `--allow-commit` option);
  - `VR/README.md` (one bullet on it);
  - `VR/observations/seeded/kill_matrix.jsonl` and `SHA256SUMS` (the post-KF1 run);
  - `_run_records/kf1_merge/`;
  - RETURN, CHANGE_RECORD and `SHA256SUMS`.
  - The sha256 values are in the checkpoint report.

## 13. The harness mutants (C; `VR/observations/harness/harness_matrix.jsonl`)

- **How they ran:**
  - each run had a clean `git archive` copy of the piping tree at `c1fea8574`, without `execution/` but with the generator's three pinned inputs, and its own fresh target;
  - one cargo job at a time;
  - `FK_SEEDED_FAULT` was unset;
  - generator mutants regenerated the case files first.
- **Run c1:** 20 runs, 26.5 min. **Run c2:** NONE and the three former survivors, re-run with `tests/engine.rs` overlaid; each overlay file's sha256 is in its row.

| Id | Run | Mutant | Verdict | Killed by |
|---|---|---|---|---|
| NONE | c2 | the clean copy | pass (43 of 43; c1: 40 of 40) | – |
| NONE-GEN | c1 | case files regenerated by the unmutated generator | pass (byte-identical; 40 of 40) | – |
| VK-H1 | c1 | every `ext.*` row skipped | killed | the pins of all 10 lanes; FLOOR |
| VK-H2 | c1 | RF-CANCEL on the class scale | killed | the not-covered list (3 → 0); the discrimination test |
| VK-H2b | c1 | S\* left uncoupled | killed | S\* against K4's exported functions; the list |
| VK-H3 | c1 | `not_covered` counted as a pass (§7.3-22) | killed | the pins |
| VK-H3b | c1 | a not-covered row left off the list | killed | FLOOR |
| VK-H4 | c1 | R = 10⁹·2⁻⁶⁴ | killed | the R unit test (the list is unchanged, as §4.10 predicts) |
| VK-H5 | c1 | tw and ext differenced (§7.3-21) | killed | RF-CHAIN twists, and 5 more families |
| VK-H6 | c2 (c1 survived) | max(\|obs\|, scale) | killed | `the_comparison_scale_is_exps_magnitude_never_obss` |
| VK-H7 | c1 | magnitude upper bound only | killed | the magnitude unit test; the engine vectors |
| VK-H8 | c1 | represented basis ignored (generator) | killed | RF-SKEW-A-CANT-AX-122-r1e-12; RF-FINITE-THIRTIETHS-O1e6 |
| VK-H9 | c2 (c1 survived) | an absolute-range pass counted as a pass | killed | `r1s_five_absolute_range_rows_are_counted_apart_from_the_passes` |
| VK-H10a–c | c1 | adapter, Rust half: nodes swapped, a member dropped, G := E | killed | K4SRC |
| VK-H10d | c1 | adapter, path A in Python: y_ref ties | killed | K4SRC |
| VK-H11 | c2 (c1 survived) | a passing discriminating control dropped | killed | `a_discriminating_control_that_passes_is_listed_never_dropped` |
| VK-H12 | c1 | the feature guard ignores dependency tables | killed | the guard's self-test |
| VK-H13 | c1 | an expected-unresolved case counted as a pass | killed | RF-RANGE's pin |

- **The three c1 survivors** were not harness defects. R1's committed CI data never reaches those paths.
  - Two tests that plan §14.1 promised had not been delivered at A1.
  - A1's own unit test for H6 did not discriminate what its comment claimed (disclosure 2).
  - By ROOT's option (a), `tests/engine.rs` adds three constructed-input tests. Each kills exactly its own mutant.
- **Data-equivalent paths,** for example a failing comparison recorded as a pass, are evidenced by A2's kill matrix, whose faults show every comparison kind is live. That argument is recorded; those paths were not run as mutants.

## 14. Scale runs (B; `_run_records/b/`)

- **Status:** done in ROOT's slot, 16:26:11–16:27:28, after the post-KF1 re-run (§12.4).
  - It ran V1, V2 and V3 in order, with 10,000 members approved.
  - All 18 runs classify ok, with no stop, no watchdog kill, no heap-cap abort and no memory-guard KILLED line.
  - **V3's figures are pre-KF3**, re-run after KF3 merges.
- **Written during K6b's slot K6B-S3, then built and checked at small sizes after it** (`_run_records/b_prep/`):
  - `VR/src/scale.rs` holds W1's O(nnz) counts (K4's pattern and skyline rules) and the admission estimate. The estimate is K6b's E_max (`performance_harness/src/k6/w1/counts.rs` at `082990c8d`, recomputed on KF1's trackers), ported term for term; only the model term is V-K's. It replaces the plan's provisional table.
  - `VR/tests/scale.rs` requires the counts to equal K4's `StorageCounts` in the committed records on all 193 factored CI cases.
  - `VR/examples/vk_scale.rs` runs one model per release process. It checks the model's and K4SRC's sha256, prints the counts and the estimate, and refuses (exit 3) above half the heap cap. It then runs `lane::run_case_with` with unlimited budgets and prints the report (with C9's S_full set), the record, RCM equality and the binary64 sparse class. Its counting, capped allocator records each phase's heap peaks.
  - `VR/runner/vk_scale_runner.py` uses K6's runner by path: `launch()`, `wait_for_quiet_host()`, `metadata()` and `admission()`. Rows the binary's backstop would refuse are deferred by name, and the memory guard's KILLED lines are counted per tier. It has `--counts`, `--plan` and `--run V1|V2|V3`, and V3 runs only with `--approve-10000`.
- **The small-size check** (release; no 1,000- or 10,000-member run):
  - the build has no warnings, and the three new files are rustfmt-clean;
  - `tests/scale.rs` passes: on all 193 factored CI cases, V-K's pattern and profile counts equal K4's `StorageCounts`;
  - VR's suite passes 44 of 44;
  - `vk_scale` on RF-LARGE-CHAIN-n00010-AX and CONT-n00100-ROT:
    - both are selected at 128 and every row passes;
    - the counts match storage, RCM is equal, and the binary64 class is Passed;
    - each emitted per-case record is byte-identical to the committed one;
  - the no-op, `--counts-only`, backstop (exit 3), heap-cap (the marker), unknown-case (exit 4) and usage (exit 2) paths all behave as specified;
  - the runner's `--counts --tiers V1`, `--plan` and `--run V1`, into a scratch records directory:
    - all six 100-member models are ok and selected at 128, with no stop and no memory-guard kill;
    - ρ on the footprint basis is 0.35–0.45 of the estimate.
  - This was a check run, not B. B's V1 re-runs in its slot.
- **Expected at 10,000 members, and ROOT's ruling:** K6b's W1 run on RF-LARGE-CHAIN-n10000-AX ended deterministically `Unresolved(ExactSumSpan)`. The 128 candidate was rejected, then the 256 verification's shared build stopped with `Span` in `gamma_m` or `uc_bounds` (`verify.rs:471-485`).
  - ROOT rules it the KF3 availability finding: record it and continue the tier.
  - The runner implements this as one named, narrow exception, `kf3_availability_exact_sum_span`. It applies only when all of these hold:
    - the model has 10,000 members;
    - the outcome is `Unresolved ExactSumSpan`;
    - nothing was published and every row is counted as failed;
    - the only failure is that case's "not selected" line.
  - The case's rows are then recorded as `unresolved_availability`, never as passes. Any other unresolved reason, covered-row failure or stop condition still stops.
  - `runner/test_vk_scale_runner.py` (8 tests, standard library) pins each edge.
  - V3's final figures are re-run after KF3 merges.
- **B's setup** (`_run_records/b/setup/`):
  - B's binary was built in release from a `git archive` of `64470c6ba`, fresh (8 crates), sha256 `8f5d6316…`.
  - The 12 large models were generated by `gen_vk_cases.py --large` in the same archive (241 s). All 12 match `cases/large_models.sha256`, and the archive's regenerated case files are byte-identical to the committed ones.
  - The counts-only runs of all 18 scale models (no solve, 512 MiB cap) are ok, and each large model's K4SRC equals the committed one.
  - E_max is 46 MiB at 100 members, 285–292 MiB at 1,000 and 2,676–2,752 MiB at 10,000, under half the heap cap (3,840 MiB).
- **As run:**
  - the binary is `8f5d6316…`, from a `git archive` of `64470c6ba`; the runner is at `a4b8c1957`;
  - the models are in `<wt>/scratch/i17/b_models` (uncommitted, their sha256 committed), and the counts are the setup's;
  - the host is an Apple M5 Max (18 cores, 128 GiB, macOS 26.6.2);
  - one release process per model, one run each, C = 8 GiB, a heap cap of C − 512 MiB, and the quiet-host wait before each run;
  - the no-op baseline: 1.87 MB RSS and 1.16 MB footprint.
  - `_run_records/b/summary.txt` is `summary.py`'s table of every run.

### 14.1 Outcomes and the report

| Model | Outcome | Rows | Pass | Absolute-range | Not covered (sampled; C9 from S_full) | unresolved_availability | Binary64 sparse class |
|---|---|---:|---:|---:|---|---:|---|
| CHAIN-n00100-AX, -ROT | Selected at 128 | 1,312 each | all | 0 | 0; (no `s_full` below 1,000) | 0 | Sensitive |
| TREE-n00100-AX, -ROT | Selected at 128 | 1,312 each | all | 0 | 0; – | 0 | Sensitive |
| CONT-n00100-AX, -ROT | Selected at 128 | 1,462 each | all | 0 | 0; – | 0 | Passed |
| CHAIN-n01000-AX, -ROT | Selected at 128 | 103 each | all | 0 | 0; `[]` | 0 | Sensitive |
| TREE-n01000-AX, -ROT | Selected at 128 | 194 each | all | 0 | 0; `[]` | 0 | Sensitive |
| CONT-n01000-AX, -ROT | Selected at 128 | 215 each | all | 0 | 0; `[]` | 0 | Passed |
| CHAIN-n10000-AX *(pre-KF3)* | Unresolved ExactSumSpan | 103 | 0 | 0 | 0; `[]` | 103 | NumericallyUnresolved (scaled condition estimate at working-precision boundary) |
| CHAIN-n10000-ROT *(pre-KF3)* | Unresolved ExactSumSpan | 103 | 0 | 0 | 0; `[]` | 103 | the same |
| TREE-n10000-AX *(pre-KF3)* | Unresolved ExactSumSpan | 194 | 0 | 0 | 0; `[]` | 194 | the same |
| TREE-n10000-ROT *(pre-KF3)* | Unresolved ExactSumSpan | 194 | 0 | 0 | 0; `[]` | 194 | the same |
| CONT-n10000-AX *(pre-KF3)* | **Selected at 128** | 215 | 211 | **4** | 0; `[]` | 0 | Sensitive |
| CONT-n10000-ROT *(pre-KF3)* | Unresolved ExactSumSpan | 215 | 0 | 0 | 0; `[]` | 215 | Sensitive |

- **On every run:**
  - the report accounts for every row;
  - the not-covered set equals the committed (empty) list;
  - there are no class mismatches, undiscriminated controls or unexpectedly failing controls;
  - RCM is equal;
  - V-K's counts equal K4's storage counts.
- **Every selected case** is verified at 256, and no row fails.
- **CONT-n10000-AX publishes 265,013 rows,** of which 101,289 are `absolute_verified` and 0 are unpublishable. Its four absolute-range passes are `u.C2500.UZ`, `u.C4999.UZ`, `R.S1250.UZ` and `Mb.A3750.i`.
- **The five ExactSumSpan cases** take the KF3 exception exactly as committed. Each publishes nothing, has only the "not selected" failure, and every other check passes.

### 14.2 Where the ExactSumSpan cases stop (pre-KF3)

All five take the same two attempts:
1. **The 128 candidate** is `Rejected(VerificationFailed)`.
   - Its own stages are solve, refinement, recovery and rhs, with no stop-rule work.
   - It built the 128 shared state: formation, assembly, residual formation, the factor (724.4 M; 376.2 M on CONT) and condition estimation.
2. **The 256 verification** is `Failed(Stop(Span))` in the verification's shared build.
   - It records bounded formation (320.9 M) and wide formation (926.5 M on AX, about 1,304 M on ROT), with verification-shared work of 1,261–1,699 M.
   - **It records no `uc` work,** where the selected CONT-n10000-AX records 106.4 M of `uc` at the same point.
   - So the Span arises after the bounded and wide formation, before the Uc bounds complete. That is consistent with ROOT's locus in `gamma_m` or `uc_bounds` (`verify.rs:471-485`), which the records alone cannot name.
- The CONT-n10000-AX verification needed no shifted factorization.

### 14.3 Work, heap, RSS and time (load 4.1–5.1 throughout, including the known external process)

| Size | Charged work (M LME) | W1 phase | W1 heap (in-place = move) | Peak RSS | Peak footprint | Estimate E_max | ρ (footprint, net of the baseline) | Process wall |
|---|---|---|---|---|---|---|---|---|
| 100 | 69.3–111.0 | 0.06–0.16 s | 9.1–11.2 MiB (move 9.3–12.3) | 19.1–22.8 MiB | 17.2–20.8 MiB | 46 MiB | 0.35–0.43 | 0.1–0.3 s |
| 1,000 | 799.3–1,228.3 | 0.62–1.54 s | 81.4–86.2 MiB | 118.9–127.3 MiB | 117.0–125.5 MiB | 285–292 MiB | 0.40–0.44 | 0.8–1.7 s |
| 10,000 *(pre-KF3)* | 6,638.7–8,298.7 | 5.05–10.10 s | 816.5–835.0 MiB | 927.4–1,042.1 MiB | 881.0–991.6 MiB | 2,676–2,752 MiB | 0.32–0.37 | 6.1–11.2 s |

- **At 10,000 members,** CONT-n10000-AX, the one selected case, charges 8,135.1 M, of which the stop rule at 128 is 2,596.8 M. The ExactSumSpan cases charge 6,638.7–8,298.7 M up to their stop.
- **The ROT frames** take 1.6–2.0 times the AX frames' time at each size.
- **W1's heap** grows 7.5–9.0× from 100 to 1,000 members and 9.7–10.0× from 1,000 to 10,000. The last factor is pre-KF3, and five of its six runs stop in the 256 verification.
- **These are observations with their load, not a timing claim.** Timing is K6b's.
## 15. Suites, CI time, toolchain and host

- **VR's suite, 43 tests, all passing on `e24e911e6` plus `tests/engine.rs`.** They are:
  - lib units 7;
  - `adapter` 4, `engine` 3, `feature_guard` 4, `files` 10, `invariance` 1, `lane` 10, `parity` 2 and `rcm` 2.
- **VR's CI cost** (debug, `-j 4`, `RUST_TEST_THREADS=2`, on the Mac):
  - the fresh-target build takes 3.8 s (A1, `_run_records/a1/vr_fresh_build_final.log`);
  - the test binaries sum to 37.9 s at C, with `lane` 19.6 s, `parity` 11.7 s and `adapter` 3.7 s (`_run_records/c/run2/vr_suite_worktree.log`). At A1, with 40 tests, they summed to 35.1 s.
  - That is well under the 3-minute reporting line.
- **FK:**
  - the full suite passes 394 with the variable unset (A2; lib 328 in 692 s debug; `_run_records/a2/a2_fk_suite.log`);
  - FK compiles without warnings with the feature off, with it on, and for tests;
  - after KF1's merge the full suite passes 402, with lib 336 in 682 s debug (`_run_records/kf1_merge/fk_suite.log`).
- **VR's suite after the merge** passes 43 of 43, with the test binaries summing to about 35 s (`_run_records/kf1_merge/vr_suite.log`).
- **Not run by V-K** (the brief's gates, run by ROOT on the PR): DEC-025's sweep with VR's manifest, hosted CI, and the independent review. T9 and the both-entry gate are not run, because no product path changes (§12.1).
- **Toolchain:** rustc and cargo 1.97.1 (`8bab26f4f`, 2026-07-14), LLVM 22.1.6, `aarch64-apple-darwin`.
  - `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, one cargo job at `-j 4`, `RUST_TEST_THREADS=2`.
  - Targets: `<wt>/vk-target` for suites and scans, and `<wt>/vk-mut/` for the mutation runs. The mutant copies were deleted after C.
  - Python 3 is used with the standard library only.

## 16. Interface for ROOT's W1 limits, V-P and F2a

### 16.1 Outcomes, precisions and work per family (CI scale; `_run_records/d/work_summary.{py,txt,json}`)

Work is in K4's limb-multiply equivalents (LME), taken from the committed records.
- **"Charged"** is the case's `invocation_charged`.
- **"Work at p"** is the sum of the attempts at precision p: each attempt's own stages, plus its shared stages where it built the shared state.

| Family | Outcomes | Selected | Charged: min / median / max (case at max) | Work at 128: max (case) | Work at 256: max (case) |
|---|---|---|---|---|---|
| RF-CHAIN | 30 selected | 128 ×30 | 2.92 M / 4.68 M / 9.97 M (A-n10-r1e-04) | 4.84 M (A-n10-r1e-04) | 5.13 M (A-n10-r1e-06) |
| RF-SKEW | 36 selected | 128 ×36 | 1.09 M / 1.35 M / 1.75 M (A-CANT-AX-122-r1e-12) | 0.79 M | 0.98 M |
| RF-WEAK | 9 selected | 128 ×9 | 1.77 M / 3.77 M / 3.96 M (W-3D-rho1e-12) | 1.79 M | 2.17 M |
| RF-LARGE (10, 100) | 12 selected | 128 ×12 | 7.04 M / 40.2 M / 111.0 M (CHAIN-n00100-ROT) | 36.6 M (CHAIN-n00100-ROT) | 74.5 M (CHAIN-n00100-ROT) |
| RF-INVARIANCE | 25 selected | 128 ×25 | 1.94 M / 4.15 M / 91.1 M (TREE100-OFF-1e6) | 33.7 M | 57.3 M |
| RF-RANGE | 30 selected, 2 unresolved | 128 ×30 | 1.35 M / 4.66 M / 7.04 M (CONT-SIM-b) | 3.11 M | 3.93 M; at 512: 1.46 M; at 1024: 2.30 M (THIN-A) |
| RF-ZERO | 4 selected | 128 ×4 | 1.23 M / 1.56 M / 3.26 M (SYM) | 1.68 M | 1.59 M |
| RF-FINITE | 6 selected | 128 ×6 | 2.32 M / 9.00 M / 9.21 M (THIRDS-O0) | 3.53 M | 5.68 M |
| RF-MECH | 1 selected, 8 refused | 128 ×1 | 0 / 0 / 1.03 M (LINE345-RX-COMPANION) | 0.59 M | 0.44 M |
| RF-CANCEL | 38 selected | 128 ×38 | 1.70 M / 1.75 M / 2.41 M (M-G1e80-GnG-ORTHO) | 1.54 M | 0.89 M |

- **The largest stages.**
  - At 128 the stop rule is the largest stage in every family. The maximum is 14.7 M, on RF-LARGE-CONT-n00100-ROT. Condition estimation and the factor come next.
  - At 256 the largest are residual formation and wide formation, then the shift and condition estimation. The per-(p, stage) maxima and the cases at them are in `work_summary.txt`.
- **The CI maximum is 111.0 M LME per case,** on RF-LARGE-CHAIN-n00100-ROT.
- **RF-LARGE at scale (B, §14.3):**
  - 1,000 members: all six selected at 128, charging 799.3–1,228.3 M; the maximum is CHAIN-n01000-ROT.
  - 10,000 members *(pre-KF3)*: CONT-n10000-AX is selected at 128 and charges 8,135.1 M. The other five end `Unresolved(ExactSumSpan)` after 6,638.7–8,298.7 M; the maximum is CHAIN-n10000-ROT.
  - W1's heap is 81–86 MiB at 1,000 members and 817–835 MiB at 10,000, and ρ against E_max is 0.32–0.44.
- **Refused cases charge 0.** The eight RF-MECH mechanisms are refused by geometry before any factor.
- **After KF1:** no figure moves. On the merged tree VR's lane tests match every committed record byte for byte, work included (§12.4).

### 16.2 The per-case record (`vk-case-record-v1`; `VR/src/records.rs`)

- **Case fields:**
  - `id`, `family`, `outcome` (`Selected at p`, `Refused <refusal>`, `Unresolved <reason> [<geometry>]`, or `source_refused`);
  - `selected_precision`, `verification_precision`, `corrections`;
  - `geometry`;
  - `published_rows`, `absolute_verified_rows`, `unpublishable_rows`;
  - `report` (the seven tallies);
  - `k4src_sha256`, `case_limit`, `invocation_charged`.
- **`attempts[]`**, one per attempt:
  - `precision`, `role`, `outcome`, `residual_basis`, `corrections` and `gate`;
  - `own_work` and `exact_sum_work`;
  - `stages{19}` and `shared_stages{19}`;
  - `shared_work` and `shared_built_here`;
  - `stop_rule_work`, `verification_work`, `verification_shared_work` and `verification_shared_built_here`;
  - `storage{pattern_entries, profile_entries, limbs_per_entry}`;
  - `verification{data_blocks, shift_factorizations, uc_missing, g_max, g_violation}`.
- **Regeneration** is a decision: `cargo run --release --example vk_records -- --write [family]`. The lane tests compare the records and never write them.

### 16.3 For F2a

- **Evidence F2a must publish.** VK-F05, F06, F07 and R28 are killed on the outcome, the attempts and the selected precision against V-K's committed records (O5's condition). F2a's publication of the retained evidence is what lets a product-lane check do the same.
- **A W1-unresolved case** (THIN, §6): what F2a publishes when a routed case ends `Unresolved(Ceiling)` is routed to F2a by ROOT.
- **The retained-state digest** (§4.1.8), if F2a needs it: ask for an accessor that exposes no private type. `PrecisionState` stays crate-private (K6b ruling C-2).

### 16.4 For V-P: extending VR to the product lane

- **Dependencies:** add `product_physics`, as ruled at Q1. The kernel lane stays as it is.
- **Reuse:**
  - the case files and their pins;
  - the key map (`Model::resolve`);
  - the engine (`exact`, `compare::judge` and the tallies);
  - the floor (`floor`, the not-covered and expected-unresolved lists);
  - the discrimination check (`lane::value_controls`);
  - the records' report shape.
- **New in V-P:**
  - a product adapter from `Model` to PP's request, independent of the kernel adapter and checked the same way;
  - a product-side `observe`, with twist and extension still derived as T/k_t and N/k_a;
  - a product record kind beside `vk-case-record-v1`.
- **Known product-route facts to pin:**
  - the nine RF-RANGE `AXIS_TOLERANCE` refusals on the ordinary route (C12);
  - THIN's ordinary-route standing, since K2b's rulings say W2's scaling solves PHYS-R4;
  - the five sub-range rows as `pass_absolute_range` at product level;
  - RF-CANCEL's UDL cases (W1b).
- **Mutation:**
  - `run_harness_mutants.py` takes new mutants as exact edits with checked counts (`--dry-run`), plus `--overlay` and `--carry` for re-runs;
  - `run_seeded_faults.py` runs the FK faults against any VR test set.

### 16.5 The export V-K uses (K6b's A0, `3018343c2`)

- **A0 exports** 72 items, 38 methods and 130 fields through `FK::structural::retained_api`.
- **V-K imports 28 names:**
  - `solve_case`, `PrimitiveSource`, `SourceParts`, `SourceError`, `StraightMember`, `Spring`, `SpringKind`, `DirectionalSpring`, `Constraint`, `NodalLoad`, `Station`, `Dof`, `Component`, `End`;
  - `CaseLimit`, `InvocationMeter`, `CaseOutcome`, `Refusal`, `PublishedRow`, `QuantityId`, `RowClass`, `Binary64Outcome`, `AttemptRecord`, `StageWork`;
  - `reverse_cuthill_mckee`, `coupled_scales`, `body_extent`, `FLOOR_RATIO_BITS`.
- **Unused and still private:** `Kind::{ALL, index}`, and the fields of `AttemptWork`, `WidthWork` and `SumWork`.
- **V-K made no visibility change in FK.**

## 17. What was not done

- **V3's final figures:** V3 re-runs after KF3 merges. Until then its figures are pre-KF3, and five of its six models end in the KF3 availability finding.
- **The product lane** (V-P, after F2a), and every product-level run.
- **Linux or Windows runs.** Every observation is Mac-only.
- **Any limit or threshold.** ROOT sets W1's limits from K6, K6b and V-K's records.
- **RF-CANCEL's three UDL cases** (W1b's).
- **§7.3's items outside the kernel lane,** and R7's mutants that need controls outside R1 (§12.3).
- **The independent review, hosted CI and DEC-025.** These are ROOT's PR gates.

## 18. Disclosures

1. **The A1 stop (THIN)** was reported, not tuned. It was ruled a coverage limit after GEN's confirmation (§6).
2. **A1's unit test for H6 did not discriminate what its comment claims.** It asserts that `max(|exp|, scale)` uses |exp| and never |obs|, but none of its cases distinguishes the two. VK-H6 survived run c1 as a result. `tests/engine.rs` closes it.
3. **Two tests promised by plan §14.1 were not delivered at A1:** the absolute-range path on R1's five rows, and the controls test. VK-H9 and VK-H11 survived c1 as a result. `tests/engine.rs` closes both.
4. **Kill kinds are a heuristic.** The kill matrix's `kinds` are read from the test output, and they were refined after the live run by `--from-logs` over the unchanged logs. The failing-test lists are exact.
5. **Scope 11's time:** the CI-scale per-case times (release) were recorded without the load, and no time claim is made. B recorded each scale run's time with its load (§14.3).
6. **A2's ordering:** stable rustfmt changed whitespace only at VK-F13's site in `ledger.rs` after the kill matrix, FK's suite and the R28 release check had run. The scans, `check_fault_sites`, the compile checks and VR's suite ran after it.
7. **A1's first debug parity run** hung on the RF-MECH-DISC dense witness, which takes about 300 s in release. It was stopped, and ROOT ruled CI parity at up to 100 members (C4).
8. **VK-H10b's committed log is trimmed** to the first 20 FAILURE lines per family. The full log's sha256, `e5ede5a8…3627` over 2.85 MB, is in its header.
9. **One Git write:** a `git fetch` in `<wt>/vk`, made on ROOT's first post-KF1 instruction before ROOT's correction arrived.
   - It updated one remote-tracking ref, `origin/claude/chirality-app-v4-60-percent-a41fd5`, and nothing else.
   - No merge was run, and no index, branch or working-tree state was touched by Git. ROOT made the merge.

## 19. Records (`T3/IMPLEMENTATION/VK/`)

- `PLAN_CHECKPOINT0.md`: the plan (`adaf697a…`).
- `RETURN.md` (this file) and `CHANGE_RECORD.md`.
- **`_run_records/a1/`:**
  - the generator runs and checks (`gen_*`);
  - the THIN probe and GEN's confirmation;
  - the K4 adapter comparison;
  - the dense RF-MECH timing;
  - the release records runs;
  - the fresh-target build and test logs, and the suites;
  - the toolchain.
- **`_run_records/a2/`:**
  - the scans before, after and final;
  - FK's suite and VR's suites;
  - the compile checks;
  - `check_fault_sites`;
  - the R28 release check;
  - the kill matrix, its live run log, its rebuild log, and the 17 per-fault logs;
  - the toolchain.
- **`_run_records/c/`:**
  - run c1: the matrix, the run log, the NONE pre-check, and 43 logs and diffs;
  - the proposed tests and their check runs;
  - `run2/`: the matrix, the run log, VR's suite in the worktree, and 8 logs and diffs;
  - the toolchain.
- **`_run_records/d/`:** `work_summary.py` and its outputs (§16.1).
- **`_run_records/kf1_merge/`:**
  - `check_fault_sites`, the scans and the compile checks;
  - FK's and VR's suites;
  - the kill matrix, its run log, its rebuild log and the 17 per-fault logs;
  - the VK-R28 release check (the build, NONE, and VK-R28);
  - the toolchain.
- **`_run_records/b_prep/`:** B's code built and checked at small sizes:
  - the build logs, formatting, the counts test and VR's suite;
  - the direct `vk_scale` checks;
  - the runner's V1 check, with its counts, plan and records.
- **`_run_records/b/`:**
  - `setup/`: the binary's build and sha256, the large models' generation and check, the counts-only runs (`counts.jsonl`), and the runner test's log;
  - `runs/`: the runner's `records.jsonl`, each run's JSONL, record, stderr and time file, the baselines, `metadata.json` and the three tier logs;
  - `summary.py` and `summary.txt`.
- **`SHA256SUMS`** covers every file under `IMPLEMENTATION/VK/` except itself. It is regenerated whenever a file changes, and last at D's close.
