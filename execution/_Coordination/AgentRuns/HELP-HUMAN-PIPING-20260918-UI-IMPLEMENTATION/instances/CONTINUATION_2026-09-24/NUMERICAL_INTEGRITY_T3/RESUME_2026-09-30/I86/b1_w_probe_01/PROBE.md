# I86 B1-SW: W2b's replacement, a publishing cap-maximal input, and the three case components (a probe; records only)

TASK (Type 2), I86, role I-W, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC (2026-10-06 host local time).

**Briefs (verified before work):** `R/BRIEFS/B1_COMMON.md`, sha256 `2d170307516b98c40e8aa2dcf352cf11a13dbdd29aeddd78a4c39f3f15eb2c75`, and `R/BRIEFS/B1_SW.md`, sha256 `4559dfb9d5dcbf9d23ad8f62200a661fd2646a337a4e13c919f03ca295042f24`. I read NUM's root `AGENTS.md` and `agents/AGENT_TASK.md` first.

**The specification:** PLAN_v2 §2.0, items 1–4, with its controls and stop rules (`R/I84/b1_plan_01/PLAN_v2.md`, sha256 `c85786b704805311485b44ba27c8826ca278c1237d92010f9f997dbf3c919be0`, verified), carried with RV107's A1-N-5 (`R/REVIEW_RV107/b1_plan_01/ADDENDUM_01.md`) as B1_SW.md states. The method is I81's (`R/I81/b1_probe_01/PROBE.md` and its `_run_records`; `R/BRIEFS/B1_0_PROBE.md`).

**Basis read:** PLAN_v2 §0–§2.1, §3.4–§3.6, §4, §7 (decisions 4, 14, 24, 25), §8 and §9; RR "I81's B1-0 probe verified; W-C2's case C established; the re-basing ruled", "Owner decision: M's practical limit is 12 GiB; target machines", "I82's addendum: B1's target is S3, …" and "B1's PLAN_v2 accepted; RV107's A1 amendments; phase 1 dispatched"; RV107 ADDENDUM_01's A1-N-5 and A1-N-8; I81's PROBE.md, `instrumentation.diff`, `zz_i81_probe.rs` and `probe_run2.log`; QUAL §4 (`T/IMPLEMENTATION/F2A_D1/copies/QUALIFICATION.md`, sha256 `8edbf4b4…2c29`, as I81 recorded). Code is cited by symbol.

**Placeholders:** `WT`, `NUM`, `P`, `PP` (= `P/core/product_physics/src`), `T`, `R`, `RR` and `VENV` as in the dispatch. `ARCH` = the disposable archive `WT/scratch/i86_b1_w/arch/`. `S` = `WT/scratch/i86_b1_w/` (my scratch). √eps = 1.4901161193847656e-8.

**Limits kept.** Records only; no maintained file changed. No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). No DEC-025, no evidence sweep, no installs, no native or solver-at-scale job beyond the inputs named here. Every cargo job went through `WT/tools/t3_cargo.sh` (memory guard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, with `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` unset. The item-4 runs executed the already-built test binary directly under `/usr/bin/lockf -k WT/guard/cargo_job.lock` (PLAN_v2 §3.6's method), and I never killed another job. `TMPDIR` pointed into my scratch; nothing was written to the system temp directory. ROOT's DEC-025 for SI1b held the lock when I started; my first build waited for it (`_run_records/cargo_jobs_i86.log`). Inputs run: QUAL §4's W2 and W2b and the milestone (controls), and this probe's own variants (§3–§5); nothing else.

## 0. Outcomes in brief

| Item | Outcome (both modes; Direct and the witness twin agree in every row) |
|---|---|
| 1. W2b's replacement | **Qualifying variants exist in both constructions; the stop rule did not fire.** (a) 3 variants tried (section scale s = 0.1, 0.01, 0.001): all three are **Sensitive, solved, and reach native (Selected), then fall back at Candidate.** But the reciprocal condition estimate does **not** fall below √eps (1.07e-6, 2.02e-6, 8.66e-7, against W2b's 6.23e-7): uniform section scaling is absorbed by the solver's radix scaling. They are Sensitive only through R-b′ (`recovery_demoted`), not through the report. (b) 6 variants tried: the first 3 were my construction error (S0 without a family maps to Guide, which refuses RY and RZ: `SUPPORT_INPUT_INVALID`, blocked, not attempted). The corrected 3 (S0 `anchor`, RX spring k at N0): **k = 1e3 and k = 144 are Sensitive by the report itself** (rcond 2.20e-9 and 3.18e-10 < √eps), and k = 1e4 by R-b′ (rcond 2.19e-8). All three reach native (Selected), then Candidate. **K-D5 never fired on the ring.** **Proposed replacement: `b2_k1e3`** (§3.4). |
| 2. A publishing cap-maximal input | **Yes: `c1` publishes a successor in both modes** (Direct and twin; the Rust reader PASSes). It is construction (c): 7 milestone copies plus 4 anchored, unloaded filler bodies, 128 moments with each copy's net exactly the milestone's. **It reaches the count caps for nodes, members, supports, loads, model and request materials, temperature points and the 128-byte typed and raw strings** (32, 32, 32, 128, 4, 4, 16, 128). It does not reach restraints (66/192) or springs (21/192). c = 1 only (A1-N-5). It is Sensitive by K-D5 (the D-5 line at copy 0's N1:RX, the milestone's own ratio). First variant tried; the ladder needed no second. |
| 3. The three case components | **On c1's model, three distinct 128-moment sets A, B and C each publish alone** (Sensitive by K-D5; native Selected; successor; Rust reader PASS; both modes, Direct and twin), with every provenance escaped and a raw value of depth 16. **The assembled three-case request** passes every D1 census row except D1.4 and `LoadCasesCapacity` (3/1), which are main's single-case clause that SA widens: raw values 6,249/16,384, raw string bytes 52,106/65,536 (13,430 B margin), raw depth 16/16, per-case loads 128/128/128 (total 384). **No provenance shortening was needed.** |
| 4. The early reading | §6. Informational: one mode and one run per process, `/usr/bin/time -l`, the dev/test build only, with each run's furthest phase taken from its witness twin. **The cap-maximal successors (c1, A, B, C) peak at 71.7–77.3 MiB RSS (54–60 MiB peak footprint) and take about 17 s; the Candidate fallbacks take about 20 s at 37–43 MiB.** The milestone takes 0.7 s at 23 MiB, and the floor is 3.2 MiB. |

**Controls.** The milestone, W2 and W2b through this probe reproduce I81's records field for field: 6 input-and-mode pairs, 11 fields each, 0 differences (`_run_records/compare_i81.log`). The committed `witness_w2_cap_maximal` and `witness_w2b_cap_maximal_solvable` reproduce QUAL §4 in the same build (`Fallback("Preparation")` and `Fallback("Candidate")`, both modes). Two runs of the variant set gave identical probe lines (270 of 270), and two runs of the controls gave identical lines (77 of 77) (`_run_records/compare_runs.log`).

## 1. Setup and build identity

- **Archive:** `GIT_OPTIONAL_LOCKS=0 git -C NUM archive 47a3bdfcf5a37e856465c45cf904383f10181498 | tar -x -C ARCH/`. Before editing, the three production files I instrumented were checked: PP `lib.rs` sha256 `4c33c250…9763`, `retained_product.rs` `f536bfe7…a794` and `retained_memory.rs` `fbc7c9db…b7b4`. These equal I81's recorded pre-edit hashes. Main `47a3bdfcf5` and I81's `d8c88774d0` have the same `P/core/product_physics`, RE `src` and `P/core/solver` trees (`git diff --stat` empty).
- **Target:** `WT/targets/i86-b1-w/`. The test binary is `open_pipe_stress_product_physics-61bc91b446a38174`. Its sha256, in `_run_records/test_binary.sha256`, is that of the second build, which ran rounds 2 and 3, the reruns and item 4. Round 1 ran the first build.
- **Registered build, observed:** every admission report reads `profile=Registered`, and the probe's `registered()` (I81's check of `OPS_RETAINED_BUILD_IDENTITY` against the facade tests' `REGISTERED_IDENTITY`) is `true`. The instrumentation touches none of the identity's inputs.
- **Probe-only instrumentation** (`_run_records/instrumentation.diff`; archive only; every addition is `cfg(test)`; no control-flow change):
  1. I81's prints, renamed `I86_*` (I81's diff applied with the rename): `I86_SEEDS` in `permitted_run` and at `retained_w1`'s entry, `I86_W1_START`, `I86_PREPARATION_FAILURE`, `I86_CANDIDATE_REFUSAL`, `I86_PRECOMMIT_ERROR`/`_DUMP`, and `I86_NATIVE_OUTCOME` in `solve_native`.
  2. New: `I86_MARK <phase> t_us=…` wall-clock marks after the ordinary run in `permitted_run` and at each W1 phase boundary in `retained_w1` (`w1_start`, `prepared`, `native_selected` or `native_unavailable`, `candidate`, `staged`, `serialized`, `precommit_ok`). `I86_QUIET` suppresses the seed print, which serializes the envelope, for the item-4 runs.
- **The probe module** is `_run_records/zz_i86_probe.rs`, declared in `retained_memory.rs` beside `law_tests` and `witness_tests` as I81's was. Every test is `#[ignore]`. Its `probe()` is I81's: the plain value route, then one counted Direct invocation through `run_linear_static_preview_value_with_retained_direct`, with the admission report, the W1 cause, the counts against `ONE_RUN_THROUGH_G_C`, the armed hooks before and after, the notice count, byte equality with `with_notice(plain, case, None)` and with plain, and, for a successor, the Rust reader's verdict. Additions:
  - the report's reciprocal condition estimate, refinement attempts, assembly perturbation and amplification estimates, and K-D5's evidence line, parsed from the published integrity diagnostic;
  - **the census** (`assess` with `Entry::Direct`, then `cap_rows` over the report's facts): the domain, the refusal, the required bytes, and 26 cap rows;
  - **the witness twin:** a verbatim copy of the S1 witnesses' private driver (`permitted_work`) on `on_reserved_stack(R/16 = 4 MiB, carry_test_hooks(…))`, with the furthest phase mapped by PLAN_v2 §3.5's table;
  - inputs read from JSON files (`I86_FILES`), so new variants needed no rebuild.
- **The inputs** are written by `_run_records/gen_inputs.py` (system `python3`, read-only on committed bytes). Item 1's variants start from W2b's exact committed Value, written by the probe's `zz_i86_dump_builtin` from the committed helpers (`law_tests::cap_maximal` and W2b's support edit); its Value sha256 is `d74d01ce…19cb`, equal to I81's W2b input. Item 2 and item 3 start from the milestone fixture (Value sha256 `8d1967a7…2c76`).
- **Builds:** two cargo builds of the lib test target (the second added the assembly-estimate print, before any variant ran). No other source change was made during the probe.

## 2. Controls

1. **Against I81** (`_run_records/compare_i81.py`, `compare_i81.log`): for the milestone, W2 and W2b in both modes, the input sha, plain sha and length, published verdicts, W1 cause, notice count, both byte checks, successor flag, and published sha and length all equal I81's `probe_run2.log`. So main `47a3bdfcf5`'s producer behaves as I81's `d8c88774d0` did on those inputs.
2. **QUAL §4's committed W2 and W2b witnesses**, run unchanged with `--ignored` in the same build (`_run_records/round1_controls.log`): `I65_G5_WITNESS W2 … ran=Fallback("Preparation")` and `W2b … ran=Fallback("Candidate")` in both modes, as QUAL §4's table records. W2b's candidate refuses at `Predicate { row: 7 (sparse) / 8 (dense), SharperExact }`, as I81 recorded.
3. **Determinism** (`_run_records/compare_runs.py`, `compare_runs.log`): the ten-input set of round 2, run twice, gave 270 identical probe lines apart from timings. The controls, run twice, gave 77 identical lines. The first controls run predates the assembly print, so that one field is ignored there (`--ignore-assembly`).
4. **Facts about the controls that the variants are read against:**
   - **The milestone** is Sensitive by **K-D5**: its report has rcond 2.606e-8 (above √eps), and the D-5 line demotes it at `N1:RX` with trigger ratio 2.43 (sparse) and 4.85 (dense). It publishes a successor in both modes, and its twin reaches the successor.
   - **W2b** is Passed: rcond 6.234e-7, amplification 5.9e-11, no D-5 line, no R-b′.
   - **W2** is blocked at validation (`not_assessed`, no seed): G-C declines it on Direct, and its twin falls back at Preparation, as I81 recorded.

## 3. Item 1: W2b's replacement

**The test** (PLAN_v2 §2.0): a variant qualifies when its published verdict is in A (Sensitive), it is solved (`MECHANICS_SOLVED`), and W1 reaches the native stage. Each variant ran in both modes through the census, the plain route, Direct and the witness twin at 4 MiB. Every variant keeps W2b's cap-maximal counts (32 nodes, a 32-member ring, 32 supports, 128 loads, 4 + 4 materials with 16 temperature points, a 128-byte project id), and every one is inside D1 (`domain=None`, `required` 3,575,778,286 B sparse and 3,595,488,734 B dense).

### 3.1 Construction (a): the ring's sections scaled by s (outside diameter and wall)

| Variant (input sha256) | Mode | Ordinary | Verdict | rcond | Amplification | Why Sensitive | W1 on Direct | Native | Twin (4 MiB) | Qualifies |
|---|---|---|---|---|---|---|---|---|---|---|
| `a_s0.1` (`f13181a8…2246`) | sparse / dense | solved; 2,113 / 2,114 results | `sensitive` | 1.067e-6 | 3.7e-11 | R-b′ only (`recovery_demoted`; report `checks_passed`; no D-5 line) | `Candidate`: `Predicate { row 11 / 12, SharperExact }`; 1 notice; bytes = `with_notice(plain, "case", None)`; `ONE_RUN_THROUGH_G_C`; no hooks | Selected | `Fallback(Candidate)`; phase W2 | **Yes** |
| `a_s0.01` (`9ca1406e…6224`) | sparse / dense | solved; 2,113 / 2,114 | `sensitive` | 2.019e-6 | 1.7e-11 | R-b′ only | `Candidate` (rows 2 / 3); as above | Selected | `Fallback(Candidate)`; W2 | **Yes** |
| `a_s0.001` (`c6fed9df…e633`) | sparse / dense | solved; 2,113 / 2,114 | `sensitive` | 8.658e-7 | 5.4e-11 | R-b′ only | `Candidate` (rows 25 / 26); as above | Selected | `Fallback(Candidate)`; W2 | **Yes** |

**The construction's premise does not hold.** Scaling every ring section by 0.1, 0.01 or 0.001 does not lower rcond: it stays at 1.4 to 3.2 times W2b's. The solver's row and column radix scaling absorbs a uniform change of section. The verdict still moves to Sensitive, but through R-b′: the report is `checks_passed` with no D-5 line, `recovery_demoted = true`, and a `HIGH_DISPLACEMENT_REVIEW` warning appears. **An R-b′ seed can never publish** (`OrdinarySeed::recovery_demoted`: "no wire member carries that finding, so the serializer declines"; DESIGN_v2's known limit N-5). These variants stop at Candidate first anyway. I stopped construction (a) at 3 of its 6 variants, because the ladder could not move rcond and all three already qualified.

### 3.2 Construction (b), as first built: invalid

`b_k1e5`, `b_k1e4` and `b_k1e3` (input sha256 `d4a9a47d…64c8`, `de0da592…5b4a` and `380bbafc…ab59`). S0 restrains UX, UY, UZ, RY and RZ, with no family. S31 (N31's UY spring) becomes an RX spring of k N·m/rad at N0.
- **Without a family, a support with fewer than six restraints maps to Guide** (PP `lib.rs`: `_ if restrained_dofs.len() == 6 => SupportFamily::Anchor, _ => SupportFamily::Guide`). Guide refuses rotational restraints (`dof_allowed_for_family`).
- So all three are blocked before the solve, with 2 × `SUPPORT_INPUT_INVALID`, `not_assessed` and no seed. On Direct, G-C declines (`CompleteGate(OrdinarySolveNotAttempted)`): exact bytes, 0 notices. The twin falls back at Preparation, as W2 does.
- **Not qualifying. This is my construction error, recorded as tried;** these count against (b)'s six.

### 3.3 Construction (b), corrected: S0's family `anchor`

| Variant (input sha256) | Mode | Ordinary | Verdict | rcond | Amplification | Why Sensitive | W1 on Direct | Native | Twin | Qualifies |
|---|---|---|---|---|---|---|---|---|---|---|
| `b2_k1e4` (`d5f5c574…2ecd`) | sparse / dense | solved; 2,113 / 2,114 | `sensitive` | 2.188e-8 | 1.7e-9 | R-b′ only (report `checks_passed`; **K-D5 did not fire**) | `Candidate` (rows 10 / 11); 1 notice; bytes = `with_notice`; `ONE_RUN_THROUGH_G_C`; no hooks | Selected | `Fallback(Candidate)`; W2 | **Yes** (R-b′) |
| **`b2_k1e3`** (`1ea4a168…67a0`) | sparse / dense | solved; 2,113 / 2,114 | `sensitive` | **2.204e-9** | 1.7e-8 | **The report** (rcond < √eps); no R-b′, no D-5 line | `Candidate` (rows 6 / 7); as above | Selected | `Fallback(Candidate)`; W2 | **Yes** |
| `b2_k144` (`044e27d4…1626`) | sparse / dense | solved; 2,113 / 2,114 | `sensitive` | 3.176e-10 | 1.2e-7 | The report (rcond < √eps) | `Candidate` (rows 27 / 28); as above | Selected | `Fallback(Candidate)`; W2 | **Yes** |

**The rotational spring sets rcond, almost in proportion to k:** 2.2e-8 at 1e4, 2.2e-9 at 1e3, 3.2e-10 at 144. The solve succeeds with amplification far below 1. **K-D5 did not demote any (b) variant.** At k = 1e4 the report is Passed and the formation check stays quiet, and at k ≤ 1e3 the report is already Sensitive, so K-D5 does not run. Construction (b)'s literal aim, a K-D5 demotion, was therefore not met within its six (three of them invalid). Two of its variants qualify through the report's own estimate instead, and one through R-b′. All six of (b)'s variants are used.

### 3.4 Proposal for ROOT

- **W2b's replacement: `b2_k1e3`** (file `_run_records/inputs/b2_k1e3.json`, file sha256 `89b05619…fd91`, Value sha256 `1ea4a168…67a0`). It is W2b's committed input with two support edits: S0 becomes `{family: "anchor", restraints: [UX, UY, UZ, RY, RZ]}`, and S31 becomes `{family: "spring", node: N0, restraints: [RX], stiffness RX 1000 N·m/rad}`.
  - It is Sensitive by the report's own condition estimate, not by R-b′, so N-5 does not bind it.
  - It is solved, and W1 runs the full native ladder (Selected), then falls back at Candidate in both modes, on Direct and on the 4 MiB twin.
  - **W2b's committed assertion `Fallback("Candidate")` therefore carries over unchanged.** Under T-4 it is in A (`sensitive`), so ST's `NoTriggeredCase` change does not touch it.
  - **Caps reached:** nodes, members and supports 32; loads 128; model and request materials 4; temperature points 16; typed and raw string bytes 128. Restraints are 36/192 and springs 31/192.
- **Alternatives:** `b2_k144` (the same, with more margin below √eps) and `a_s0.1` (the smallest edit, but R-b′ only).
- `c1` (§4) also passes item 1's test, but it is not a ring, and it publishes.

## 4. Item 2: a cap-maximal input that publishes a successor

### 4.1 The construction (c), `c1` (`gen_inputs.py build_c("c1")`)

- **7 copies of the milestone's body** (A1-N-5's maximum). Each is 2 nodes, 1 member (OD 0.2 m, wall 0.01 m, `y_reference` (1, 0, 0)) and 4 supports at N0: rigid UX, UY and UZ, and springs RX 144, RY 1e6 and RZ 1e6 N·m/rad.
  - Copy k is offset by 10k m along X, so every coordinate difference is exact.
  - Each copy's member uses `mat:0`, which has the milestone's E = 2e11 Pa and G = 8e10 Pa, and carries 16 temperature points (id and provenance only, as `cap_maximal`'s do).
- **The filler: 4 anchored, connected, unloaded bodies,** with 18 nodes, 25 members and 4 supports. The bodies are K5 on 5 nodes (10 members), a 5-node pyramid (6), K4 (6) and a 3-member chain on 4 nodes (3).
  - Each body's first node has one support with six rigid restraints and no family, as L = 0's `rigid:N2`.
  - Sections are OD 0.2 m and wall 0.01 m, on `mat:1`. The bodies sit away from the copies.
- **128 concentrated moments at the copies' N1** (RX, RY, RZ). The 21 copy-DOF slots get 6 parts each, and copy 0's RX and RY get a seventh, for 128 in all.
  - **Each copy's net on each DOF equals the milestone's exactly** (0.0048, 0.0096, 0.0096 N·m).
  - **Each split is an integer split of the binary64 significand.** Every part is an integer multiple of 2^E, and the parts sum to the significand. So every partial sum, in any order, is exact. The audit agrees: no load-row finding.
- Model and request materials are 4 each (`mat:0`–`mat:3`), each with 16 temperature points. The project id is 128 bytes; units and analysis status are the milestone's.
- **The file:** `_run_records/inputs/c1.json`, file sha256 `719acbb0…742f`, Value sha256 `c5d2e932…88ed`.

### 4.2 Caps reached (A1-N-5)

**From the census:**
- **At the cap:** Nodes 32/32, Members 32/32, Supports 32/32, Loads 128/128 (capacity 128), ModelMaterials 4/4, RequestMaterials 4/4, TemperaturePoints 16/16, TypedTextBytes 128/128 and RawTextBytes 128/128.
- **Below the cap:** Restraints 66/192, Springs 21/192, RawValues 3,150/16,384, RawDepth 7/16 (c1 carries no depth stress; item 3's components do), RawStringBytes 24,765/65,536, RawKeyBytes 16,630/65,536 and RawKeyTextBytes 23/128.

The support cap is what limits the copies (A1-N-5): restraints and springs cannot also reach their caps with the milestone's support shape.

### 4.3 Outcome

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| Ordinary | solved; 2,113 results; plain `6d456433…4767` (1,381,860 B) | solved; 2,114 results; plain `ab49c90a…06a6` (1,383,263 B) |
| Verdict and why | `sensitive` by **K-D5**: the D-5 line at `C0:N1:RX`, trigger ratio 2.4279 (the milestone's own row and ratio); rcond 2.038e-8; amplification 2.7e-9 | the same, ratio 4.8526; rcond 2.038e-8 |
| Seed | `report/sensitive`; `w2: not_triggered`; D-5 ref set; no load-row finding; `recovery_demoted: false`; legacy `unavailable(source closure)` | identical |
| Admission | Registered; refusal None; domain None; required 3,575,778,286 B | the same; 3,595,488,734 B |
| **Direct** | **`Ok(successor)`**; native Selected; `ONE_RUN_THROUGH_G_C`; no hooks; 0 notices; successor `52573b15…20af` (2,176,063 B); receipt `dcf0bd69…c877`; **the Rust reader PASSes** (`invocation_bound`, `numerical_eligible`, 2,113 classifications); case status `selected` | **`Ok(successor)`**; the same; successor `246f844e…6a9f` (2,177,405 B); receipt `d3771df3…1101`; **PASS** (2,114 classifications) |
| **Twin (4 MiB)** | **`Successor`** (furthest phase: E_mov,max) | **`Successor`** |

**So a cap-maximal c = 1 input publishes.** The phases after the candidate, which PLAN_v2 names W3–W5 (staging, the serializer, precommit and the transfer), run at the count caps above. It took one variant (c1); c2 (c1 without temperature points) was generated but not run.

## 5. Item 3: the three cap-maximal case components

**The sets, on c1's model** (`gen_inputs.py i3_sets`; each has 128 moments, 6 or 7 per copy DOF slot, with integer-significand splits and exact nets):
- **A** (`case:a`): each copy's net is +M (the milestone's), in equal parts;
- **B** (`case:b`): each copy's net is −M, in parts weighted 1…n;
- **C** (`case:c`): copy k's net is 2^(k−3)·M (k = 0…6), in parts weighted n…1.

Load ids carry the case prefix (`a:`, `b:`, `c:`).

**Stresses** (PLAN_v2 §2.0 item 3, N-7): `" q\"b\\"` is appended to every provenance, and a raw value of depth 16 sits at `model.unknown_depth_witness` (W2's helpers, transcribed in `gen_inputs.py`).

### 5.1 Each component alone (one-case requests)

| Component (file sha256; Value sha256) | Census | Verdict and why | Direct (both modes) | Twin (both modes) |
|---|---|---|---|---|
| A (`39c44bc3…3fac`; `8756006a…15b0`) | domain None; RawDepth 16/16; RawStringBytes 26,828; RawValues 3,165 | `sensitive` by K-D5 at `C0:N1:RX` (2.43 / 4.85); rcond 2.038e-8 | `Ok(successor)`, native Selected, Rust reader PASS; successors `5ef1b317…ee88` / `3c602c8a…b0e7` | Successor |
| B (`544330c8…5651`; `c533f567…5e24`) | the same | the same | `Ok(successor)`, PASS; `7dbc9576…00af` / `5af50c74…819f` | Successor |
| C (`b6e8138f…a997`; `b4a22b37…92fb`) | the same | the same | `Ok(successor)`, PASS; `97916a60…4f6e` / `cb466a0a…99a7` | Successor |

**Each is Sensitive, solved and reaches native (Selected), so |A| = 3 under T-4.** Each also publishes alone.

### 5.2 The assembled three-case request (`i3_c1_three_case.json`, file sha256 `d05b5996…1bfc`, Value sha256 `2ca29f02…c508`)

`assess` (main's D1, `LOAD_CASES = 1`) gives:
- **domain and refusal:** `Family(Invocation, LoadCases)`, which is D1.4, as main must;
- **census complete;** `required` None (the bound is not reached).

**The cap rows:**
- **Over:** only `LoadCasesCapacity` = 3/1. The typed vector's capacity is exactly 3, so no growth.
- **Every other row is within its cap:**
  - Nodes, Members and Supports 32/32;
  - Loads 128/128 (the row reads the first case);
  - Model and Request materials 4/4; TemperaturePoints 16/16;
  - TypedTextBytes and RawTextBytes 128/128; RawDepth 16/16;
  - RawValues 6,249/16,384 (margin 10,135);
  - **RawStringBytes 52,106/65,536 (margin 13,430 B, 20.5 %)**;
  - RawKeyBytes 34,643/65,536; RawArrayCapacity 699/32,768.
- **Per-case loads** are 128, 128 and 128; the total is 384 (= L).

**What follows:**
- **D1.10** (provenance) is not reached in the domain order, because D1.4 refuses first. Each component's own census (domain None) shows its loads pass D1.10.
- **So the request is inside D1 at C = 3 by every fact except the two that SA changes** (D1.4's case count and the `LoadCasesCapacity` row), provided SA's per-case and total load rows are l = 128 and L = 384, as option S3 and PLAN_v2 §0 item 7 state.
- **No provenance shortening was needed.** RV107's estimate was 6,384 values and 57,043 escaped string bytes; the measured figures are 6,249 and 52,106.
- **This is an emulation on main's D1. I changed no cap.** SQ's witness asserts it through `assess` after SA.
- **Batch outcomes are untested.** No multi-case invocation runs on main (D1.4), so whether A, B and C keep these outcomes inside one `CaseBatchCall` is SP's (R8, N-16).

## 6. Item 4: the early reading (informational; SQ measures)

**Method** (`_run_records/timing.sh`, `timing2.sh`, `timing_input.sh`; outputs in `_run_records/timing/`):
- **Each run is its own process, in one mode:** `/usr/bin/time -l <test binary> retained_memory::zz_i86_probe::zz_i86_once --exact --ignored --nocapture --test-threads=1`. The binary runs directly, not through cargo, with `I86_QUIET=1`. The binary is the registered dev/test lib test binary (`test_binary.sha256`).
- **Three paths per input and mode:**
  - `value`: the ordinary value route only, which is W1's control;
  - `direct`: the Direct entry, the permitted path;
  - `witness`: the twin on the private driver at 4 MiB. **Its outcome gives the furthest phase**, as A1-S-1 asks, and it agreed with Direct's cause in every row.
- **The floor:** the same binary with a filter that matches no test.
- **Locking:** every process ran under `lockf -k WT/guard/cargo_job.lock`. The milestone's six runs and the floor each took the lock separately. Each other input's six runs held it once, together.
- **One repetition. The dev/test build only; the release build was not run.**

| Input | Mode | Twin outcome (furthest phase) | Direct: real s / max RSS MiB / peak footprint MiB | Twin: real s / RSS / footprint | Value route only: real s / RSS / footprint | Direct's W1 marks, s after W1 start: prepared, native, candidate, serialized, precommit |
|---|---|---|---|---|---|---|
| milestone | sparse | Successor (E_mov,max) | 0.74 / 23.4 / 8.3 | 0.73 / 23.4 / 8.3 | 0.04 / 15.2 / 3.6 | 0.00, 0.05, 0.60, 0.63, 0.67 |
| milestone | dense | Successor (E_mov,max) | 0.66 / 23.4 / 8.3 | 0.67 / 23.3 / 8.3 | 0.00 / 15.2 / 3.6 | 0.00, 0.04, 0.59, 0.61, 0.65 |
| W2 (committed) | sparse | Fallback(Preparation) (W2) | 0.02 / 13.5 / 5.7 (G-C declines; no W1) | 0.02 / 12.9 / 5.7 | 0.02 / 12.3 / 5.5 | no W1 on Direct |
| W2 (committed) | dense | Fallback(Preparation) (W2) | 0.02 / 13.3 / 5.5 (no W1) | 0.02 / 12.9 / 5.7 | 0.02 / 12.2 / 5.4 | no W1 on Direct |
| a_s0.1 | sparse | Fallback(Candidate) (W2) | 20.28 / 36.9 / 23.5 | 20.49 / 37.0 / 23.7 | 0.09 / 30.8 / 19.4 | 0.02, 0.99, refused at about 20 s |
| a_s0.1 | dense | Fallback(Candidate) (W2) | 20.36 / 39.3 / 25.9 | 20.31 / 40.9 / 27.5 | 0.16 / 34.5 / 23.1 | 0.02, 0.98, refused |
| **b2_k1e3** | sparse | Fallback(Candidate) (W2) | 20.32 / 40.9 / 27.4 | 20.29 / 41.0 / 27.6 | 0.09 / 33.5 / 21.9 | 0.02, 1.11, refused |
| **b2_k1e3** | dense | Fallback(Candidate) (W2) | 20.37 / 43.2 / 29.8 | 20.30 / 43.1 / 29.7 | 0.17 / 35.5 / 23.9 | 0.02, 1.11, refused |
| **c1** | sparse | Successor (E_mov,max) | 17.00 / 73.8 / 56.5 | 16.93 / 73.2 / 56.0 | 0.09 / 33.0 / 21.3 | 0.02, 0.61, 15.63, 16.00, 16.89 |
| **c1** | dense | Successor (E_mov,max) | 17.16 / 77.3 / 60.0 | 17.01 / 76.4 / 59.1 | 0.14 / 35.6 / 23.8 | 0.02, 0.62, 15.73, 16.12, 17.02 |
| A | sparse | Successor (E_mov,max) | 16.99 / 71.7 / 54.4 | 16.88 / 73.8 / 56.5 | 0.09 / 31.2 / 19.5 | 0.02, 0.61, 15.59, 15.97, 16.89 |
| A | dense | Successor (E_mov,max) | 16.94 / 76.0 / 58.3 | 16.93 / 76.1 / 58.5 | 0.13 / 35.6 / 23.9 | 0.02, 0.60, 15.54, 15.91, 16.80 |
| B | sparse | Successor (E_mov,max) | 16.89 / 73.8 / 56.6 | 16.96 / 73.1 / 55.6 | 0.09 / 31.3 / 19.6 | 0.02, 0.61, 15.53, 15.90, 16.79 |
| B | dense | Successor (E_mov,max) | 16.96 / 76.0 / 58.7 | 16.88 / 76.5 / 59.1 | 0.13 / 35.6 / 23.9 | 0.02, 0.61, 15.52, 15.90, 16.82 |
| C | sparse | Successor (E_mov,max) | 17.10 / 73.9 / 56.6 | 16.84 / 73.3 / 55.7 | 0.09 / 32.9 / 21.2 | 0.02, 0.61, 15.70, 16.08, 16.99 |
| C | dense | Successor (E_mov,max) | 16.97 / 76.5 / 59.2 | 16.91 / 76.3 / 59.1 | 0.13 / 33.8 / 22.1 | 0.02, 0.62, 15.57, 15.94, 16.83 |
| floor | – | (no test) | 0.02 s; RSS 3.2 MiB; footprint 2.1 MiB | | | |

(MiB = 2^20 B. The byte values and every run's full `time -l` output are in `_run_records/timing_summary.md` and `timing/`.)

**Readings, informational only:**
- **Memory.** The largest peak at the caps is **77.3 MiB RSS (81,068,032 B) and 60.0 MiB of peak footprint, for c1 dense on Direct.** That is a successor through W3–W5.
  - The value route alone peaks at 35.6 MiB for the same input. So W1 adds about 42 MiB of RSS on this input.
  - For comparison: D1's priced required bytes are 3,575,778,286 B (sparse) and 3,595,488,734 B (dense) on main's profile, and option S3's E_mov,max is 9,680,616,814 B.
  - **The priced bound is a worst case over every input at the caps.** These inputs are far from it (for example, restraints 66/192 and springs 21/192). So the gap is expected, and it is not a measure of the bound's slack.
- **Time.** **About 17 s for a cap-maximal successor in the debug build, with the candidate's proof taking about 15 s of it** (native about 0.6 s, serializer about 0.4 s, precommit about 0.9 s). The Candidate refusals (a_s0.1, b2_k1e3) take about 20 s, almost all of it in the proof after a native run of about 1 s. Debug-build times are pessimistic; the release times are B7's and B8's to read.
- **Host.** This host has 128 GB and was not under memory pressure. Other T3 jobs ran between inputs, but never during a run. These are single runs, with the probe's prints present.

## 7. For ROOT

1. **Item 1's stop rule did not fire.** Both constructions have qualifying variants. Proposed: **`b2_k1e3`** as W2b's replacement (§3.4). It keeps W2b's witness meaning: a full native run at the cap counts, then Candidate, and the committed assertion `Fallback("Candidate")` carries over.
2. **PLAN_v2's premise for construction (a) does not hold** on this producer. Uniform section scaling does not lower the reciprocal condition estimate (radix scaling absorbs it). The (a) variants are Sensitive only through R-b′, which can never publish (N-5). The condition can be moved by the spring arrangement ((b): rcond roughly proportional to k). **K-D5 did not fire on the ring** in any variant tried.
3. **Item 2's stop rule did not fire. `c1` publishes at the count caps in both modes,** so SQ can measure W3–W5 on a cap-maximal c = 1 input. PLAN_v2 §3.6's plain "W3–W5 unmeasured at the caps" statement is not needed for c = 1. **Caps not reached:** restraints (66/192) and springs (21/192), because of the support cap (A1-N-5). No published input here reaches all of D1's caps at once. W2 still holds restraints at 192 but is never attempted.
4. **Item 3:** A, B and C each publish alone, and the stressed three-case request is inside D1 at C = 3 apart from SA's two changes, with 13,430 B of raw-string margin.
   - All three share c1's stiffness: the same rcond, and the same K-D5 row and ratio. They differ only in loads (+M, −M, 2^(k−3)·M). In a batch, every case would be in A and, alone, each is Selected.
   - Whether one `CaseBatchCall` keeps that is SP's question (N-16).
5. **Item 4, for SQ and R9 (informational):** a cap-maximal c = 1 successor peaked at 77.3 MiB RSS and took about 17 s in the debug build, against D1's priced 3.58–3.60 GB and S3's 9.68 GB. The priced figure bounds the worst case at the caps, and these inputs are not that case. SQ's counting-allocator and RSS measurements, the release build and three repetitions remain SQ's.
6. **A construction error of mine, recorded:** the first (b) ladder omitted S0's family (§3.2). It cost (b) three of its six variants and had no other effect.
7. **A fact about the milestone, for the record:** its Sensitive verdict comes from K-D5 (the D-5 line at `N1:RX`; rcond 2.61e-8 is above √eps), not from the report's condition estimate. c1 and the item-3 components inherit exactly that row and ratio.
8. **Host notes.**
   - The host accepted writes into NUM at this records folder. No write guard refused anything.
   - Other T3 jobs (I83, I85) interleaved with mine on the lock throughout. My first item-4 script took the lock per run. It was slow under that contention, so I stopped it, while it was only waiting, and reran the rest with one lock per input (§6). Only my own processes were stopped.
   - The recorded copies of `timing*.sh` take `WT` from the environment; the copies run had the machine path, which is the only difference.
9. **A1-N-8:** SW ran in phase 1, beside ST (I85), and ends here.

## 8. Commands run (exact; `cd` into the stated directory first)

1. `mkdir -p WT/scratch/i86_b1_w/{arch,tmp,out,logs} && GIT_OPTIONAL_LOCKS=0 git -C NUM archive 47a3bdfcf5a37e856465c45cf904383f10181498 | tar -x -C WT/scratch/i86_b1_w/arch/`
2. Instrumentation (I81's diff with `I81`→`I86`, plus the marks) and the probe module written into `ARCH` (`_run_records/instrumentation.diff`, `zz_i86_probe.rs`).
3. In `ARCH/P/core/product_physics`, with `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=S/tmp CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=WT/targets/i86-b1-w`:
   - `WT/tools/t3_cargo.sh test --locked --offline --lib --no-run` (rc 0; `build_pp_lib.log`)
   - Round 1, with `I86_OUT=S/out RUST_TEST_THREADS=1`: `WT/tools/t3_cargo.sh test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact retained_memory::zz_i86_probe::zz_i86_dump_builtin retained_memory::zz_i86_probe::zz_i86_builtin_controls retained_memory::witness_tests::witness_w2_cap_maximal retained_memory::witness_tests::witness_w2b_cap_maximal_solvable` (rc 0; 4 passed; `round1_controls.log`)
   - `python3 gen_inputs.py S/out <milestone fixture> S/inputs c:c1 c:c2 i3:c1`, then `… a:0.1 a:0.01 a:0.001 b:1e5 b:1e4 b:1e3`
   - Round 2 (rebuilds with the assembly print): `I86_FILES=<a_s0.1, a_s0.01, a_s0.001, b_k1e5, b_k1e4, b_k1e3, c1, i3_c1_case_a, i3_c1_case_b, i3_c1_case_c> WT/tools/t3_cargo.sh test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact retained_memory::zz_i86_probe::zz_i86_files` (rc 0; `round2_run1.log`)
   - `I86_FILES=<i3_c1_three_case, i3_c1_case_a, c1> … --exact retained_memory::zz_i86_probe::zz_i86_assess` (rc 0; `round2_assess.log`)
   - `python3 gen_inputs.py … b2:1e4 b2:1e3 b2:144`; round 3: `I86_FILES=<b2_k1e4, b2_k1e3, b2_k144> … zz_i86_files` (rc 0; `round3_b2.log`)
   - Determinism: round 2's command again (`round2_run2.log`), and `… --exact retained_memory::zz_i86_probe::zz_i86_builtin_controls` (`round1_controls_run2.log`) (rc 0 each)
4. Item 4: `timing.sh` (one lock per run; stopped while waiting, §7 item 8), then `timing2.sh <binary> S/timing 1 <inputs>` (one lock per input), both running `/usr/bin/time -l <binary> retained_memory::zz_i86_probe::zz_i86_once --exact --ignored --nocapture --test-threads=1` with `I86_QUIET=1 I86_FILE I86_MODE I86_PATH`, and the floor `/usr/bin/time -l <binary> i86_no_such_test_floor --exact --ignored --test-threads=1`.
5. Read-only Python (system `python3`): `sanitize.py`, `summarize.py`, `compare_i81.py`, `compare_runs.py`, `timing_summary.py`.

The lock lines for my cargo jobs are in `_run_records/cargo_jobs_i86.log`.

## 9. Records, cleanup and limits

- **`_run_records/`:**
  - **probe sources:** `zz_i86_probe.rs`, `instrumentation.diff` and `gen_inputs.py`;
  - **the inputs that qualify,** exactly as run: `inputs/b2_k1e3.json`, `inputs/a_s0.1.json`, `inputs/c1.json`, `inputs/i3_c1_case_{a,b,c}.json` and `inputs/i3_c1_three_case.json`. Every other variant is reproducible with `gen_inputs.py` from W2b's committed Value; all input sha256s are in the logs' `I86_FILE` and `I86_BEGIN` lines;
  - **logs:** `build_pp_lib.log`, `round1_controls.log`, `round1_controls_run2.log`, `round2_run1.log`, `round2_run2.log`, `round2_assess.log`, `round3_b2.log`, `timing/` and `cargo_jobs_i86.log`;
  - **tools and outputs:** `sanitize.py`; `summarize.py` with `summary.md`; `compare_i81.py` with `compare_i81.log`; `compare_runs.py` with `compare_runs.log`; `timing.sh`, `timing2.sh`, `timing_input.sh`, and `timing_summary.py` with `timing_summary.md`;
  - `test_binary.sha256` and `probe_outputs.sha256` (the unsanitized logs and the successor files, which stay in `S/` outside NUM).

  Machine paths are replaced by `WT`, `ARCH` and `~`. In the logs, every line over 4,000 bytes that is not one of this probe's `I86_` lines is cut to 1,000 bytes, with its full length and sha256 (the producer's committed `cfg(test)` prints `I51_DUAL_LANES` and `I51_FROZEN_*`).
- **Cleanup on return:** the archive `ARCH` and the target `WT/targets/i86-b1-w/` are deleted. The archive is reproducible from `47a3bdfcf5` plus `_run_records`.
- **Basis drift:** NUM moved to `28831fc260` during the probe, and origin/main to `025c1cf326` (#1106). From `47a3bdfcf5` to either, only two rules-crate files under `P/core` or `P/fixtures` changed (SI1b), and PP does not depend on them. So the producer facts here hold at both heads.
- **Limits:**
  - **One-case requests only.** The three-case request was assessed, not run (D1.4).
  - **c = 1 only for item 2** (A1-N-5).
  - **The dev/test build only.** The release build was not built or run (§6).
  - **The runs carry the probe prints.** They establish outcomes, not S1 stack margins; the S1 witnesses are B1's, on the uninstrumented re-qualification build.
  - **Item 4 is one repetition per run, informational,** in a build with probe prints and committed `cfg(test)` prints.
