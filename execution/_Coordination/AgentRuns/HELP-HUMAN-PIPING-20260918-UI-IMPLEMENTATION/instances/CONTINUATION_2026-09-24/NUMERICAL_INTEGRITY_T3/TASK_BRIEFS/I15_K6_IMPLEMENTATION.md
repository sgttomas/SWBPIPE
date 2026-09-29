# I15: implement slice K6 (harness observations: kernel sparse and dense memory and runtime on RF-LARGE, and the runner)

> This is an implementation TASK. Read Root `AGENTS.md`, `agents/AGENT_TASK.md` and `_COMMON.md` first. The Mac host rules in `I8R_K1_RESUME.md` ("The Mac host" and "Platform calibration", `I8R_K1_RESUME.md:24-50`) override `_COMMON.md`'s host section, and apply to you in full, with K6's paths below in place of K1's.

## Roles

- ROOT (HELP_HUMAN) dispatches you directly, as a background subagent, and is your return path. There is no separate T3 manager on the Mac.
- Make no Git writes and no index operations. ROOT commits.
- Record the delegation mechanism in RETURN.

## Paths and bases used in this brief

- `P/` and `T3/` are as in `_COMMON.md:14`.
- `H` is `P/core/solver/performance_harness/`. `FK` is `P/core/solver/frame_kernel/src/`. `SD` is `P/core/solver/sparse_direct/src/`. `NI` is `P/core/solver/nonlinear_integration/`, and `SA` is `NI/src/structural_adapter.rs`. `PP` is `P/core/product_physics/src/lib.rs`.
- **Code is cited at main `d1cc97ce4`.** Its piping tree equals `24dea2dae`'s (`git diff 24dea2dae d1cc97ce4` over `P/{core,fixtures,validation,schemas,apps,tools,tests}` is empty). At drafting, `origin/main` had moved to `41aeb2a02`, which changes only `projects/chirality-app-v4/**`. Re-locate every line on your base.
- **F1b is cited at its branch head `130445db2`** (`codex/piping-f1b-20260928`, PR pending).
- **T3 records are cited at the numerics head `e26cc3be4`.**

## Purpose

K6 is the §6 row "K6: harness observations | after K1 | `P/core/solver/performance_harness/**` | Kernel sparse and dense observations on RF-LARGE; runner dry run, including the macOS watchdog path" (`T3/DESIGN_NUMERICS/DESIGN.md:1021`; `DESIGN.md` revision 5a.2, sha256 `fb62ef4a…`, hash-pinned, don't edit).

- **Order.** "K4 follows K1 and K3, then K6 and V-K" (`DESIGN.md:1038`). The row itself needs only K1, which merged as PR #1034 at `eb52114e9` (`IMPLEMENTATION/K1_MERGE/RECORD.md:3-4`; the pattern path at `FK/structural/sparse.rs:1-35`, `SD/structural.rs:1-9`).
- **Why it matters now.** Three open decisions wait on measurement:
  - **D-8, W1's budgets.** "F2a does not merge without ROOT's per-case and per-invocation limits." ROOT sets them "from K6's measurements and V-K's kernel-lane runs, both of which precede F2a, together with K4's deterministic work counts" (`ROOT_RULINGS_V1.md:1173-1174`; D-8 at `DESIGN.md:1270`; §4.1.7 at `DESIGN.md:516-520`).
  - **The dense-scrutiny ceiling.** F1b ships a provisional 6 GiB ceiling on an estimate of 96 bytes per n² entry, "revisited from K6's and V-P's measurements" (`TASK_BRIEFS/I13_F1B_IMPLEMENTATION.md:126`; `ROOT_RULINGS_V1.md:1234`, `:1273`; the constants at `PP:2871-2882` on `130445db2`).
  - **The observation-lane guard.** F1b guards the legacy DEC-050/053 lane at the same provisional ceiling, on an estimate of 24 bytes per identity-order profile entry (`ROOT_RULINGS_V1.md:1455-1463`; `PP:2950-2958` on `130445db2`). §4.8's "ROOT picks both from measurement" (`DESIGN.md:804`) is still unmet (`ROOT_RULINGS_V1.md:1254`).
- **What §4.8 asks for** (`DESIGN.md:823-834`):
  - sealed models, "R1's RF-LARGE families (10, 100, 1,000 and 10,000 members; determinate and indeterminate; axis-aligned and rotated) and the nine DEC-053 observations", each generated deterministically, with its sha256 committed (`:824`);
  - "one fresh process per model and mode, run by a standard-library Python runner", with peak RSS from `/usr/bin/time -v` (Linux) or `-l` (macOS) (`:825`);
  - `RLIMIT_AS` on Linux; on macOS a 100 ms `ps -o rss= -p <pid>` watchdog that kills above the cap and records `killed_by_rss_watchdog` with the last reading (`:826`; V1-N6 at `:146`);
  - "Runs on the owner's Mac therefore start at sizes whose Linux peak is known to fit the cap with a factor of two to spare", and stage timings printed as JSONL, "five repeats, median and minimum" (`:827`);
  - deterministic storage counts, "and for W1, limbs per entry" (`:828`); hardware, toolchain and release-profile metadata (`:829`);
  - "Observations only … The single claim is the before-and-after growth … stated as an observed fit, not a threshold" (`:830`);
  - the kernel-level home is `H`; product-level runs belong to `P/validation/benchmarks/numerical_robustness/` (`:832-834`).
- **What the harness does today** (`H` at `d1cc97ce4`):
  - Its only paths are dense: `run_fixture_repeat` assembles with `assemble_global_stiffness` and reduces with `reduce_system` (`H/src/lib.rs:772-773`), solves with `solve_dense` (`:792`), and its "sparse" observation calls `solve_symmetric_system` on the reduced **dense** matrix (`:897`; `SD/lib.rs:651-667`).
  - So it cannot observe K1's pattern path, and it would allocate n² at every size. RF-LARGE at 1,000 and 10,000 members is out of its reach (`DESIGN.md:228`).
  - It records the DEC-053 packet's nine observations (`H/src/lib.rs:597-677`, `:704-763`), pinned by two pytest files that read `H/src/lib.rs` as text (`P/tests/test_sparse_default_promotion_observation.py:18`, `:101-103`; `P/tests/test_sparse_suitability_observation.py:14`, `:55-67`).
  - Its README asserts no timing or memory thresholds (`H/README.md:26`, `:34-39`).

### What K6 builds on (merged; every line on `d1cc97ce4`)

- **K1's pattern path.**
  - `assemble_sparse_stiffness` (`FK/structural/sparse.rs:592-679`), `SparseStiffness::storage_counts` (`:206-216`, `:309-317`), `to_dense`, which "allocates n²" (`:298-308`), and `reactions` (`:360`).
  - `reduce_assembled_sparse_system` (`:729`), `prepare_assembled_sparse_structural` (`:1262`), `factor_sparse_structural_profile` (`:1727`), `finish_sparse_structural` (`:1784`), and `SparseSymmetryEvidence`, whose fields are public (`:849-853`).
  - `order_sparse_structural` returns the RCM order, the skyline and `profile_entry_count` "before the factor allocates its rows" (`SD/structural.rs:43-77`).
- **The dense gate.** `StructuralSystem` with `SymmetryEvidence`, two n×n arrays (`FK/structural.rs:40-53`); `prepare_assembled_structural` (`:1182`), `factor_structural_cholesky` (`:1865`), `finish_structural` (`:1883`).
- **SA's pattern evidence** (`SA:451-906`): `SparseAssemblyEvidence::new` stores allowances and counts per pattern entry (`:468-502`). Its accessors are public: `contributions` (`:553`), `absolute_roundoff` (`:557`), `operation_counts` (`:561`), `storage_counts` (`:565`), `dense_symmetry_view` (`:573`).
  - Its `DenseScrutiny` branch materializes `k.to_dense()` and `dense_symmetry_view()` and runs the dense gate (`:609-625`, and `:663-683` for the formation-checked entry). This is the path F1b's 96-byte estimate counts (`PP:2871-2878` on `130445db2`).
  - `solve_assembled_with_formation_check` (`:644-697`) is the entry F1b moves PP onto (`I13_F1B_IMPLEMENTATION.md:161`; `I14_K5_IMPLEMENTATION.md:256`).
  - `BodyEvidence::geometry` scans every edge for every node (`SA:1212-1240`). K5's brief records this cost as K6's to time (`I14_K5_IMPLEMENTATION.md:179`).
- **The legacy lanes the product still runs.** The DEC-050/053 dense LU (`FK/lib.rs:1631`, called at `PP:2508`), and the identity-order sparse lane `solve_symmetric_system_from_entries`, which builds `SymmetricProfileMatrix::from_entries` in identity order (`SD/lib.rs:700-716`; called at `PP:4464`, `:4601`).
- **K1's counts** for RF-LARGE-shaped chains and combs at every size, through the pattern path only (`IMPLEMENTATION/K1/RETURN.md:399-416`; the test at `SD/structural/k1_tests.rs:704-814`, whose comment reads "no time or memory claim; K6 measures", `:774`).

### What K6 does not do

- **No product, kernel or adapter change.** No edit to FK, SD, NI (including SA), PP, the headless runner, `validation/benchmarks/**`, `provenance/**`, `.github/**` or `tools/**`.
- **No change to the legacy harness.** The DEC-023, DEC-050 and DEC-053 functions, their outputs, their tests, `H/examples/sparse_default_promotion_observation.rs` and the committed `P/validation/benchmarks/*.dec05{0,3}.json` stay byte-identical. The DEC-050/053 observations are frozen (`_COMMON.md:38`).
- **No threshold.** No time or memory limit is asserted anywhere, in tests or records (§4.8's "observations only", `DESIGN.md:830`; `H/README.md:26`).
- **No W1 observation** unless ROOT rules otherwise (Q1). K4 is unmerged and blocked on D1 revision 5a.3 (`ROOT_RULINGS_V1.md:1369`, `:1516`).
- **No product-level run.** Runs through PP's public entry are V-P's (`DESIGN.md:834`, `:1031`).
- **No ceiling or budget value.** K6 measures; ROOT rules.
- **No registry dependency** and no lockfile change outside `H/Cargo.lock` (Q2).

## ROOT rulings for this slice

ROOT reviewed this brief (drafted by a TASK, sha256 `664bf204…` as drafted) and rules as follows on 2026-09-28. The rulings are also recorded in `ROOT_RULINGS_V1.md`, section "K6: spawn and rulings". Where a ruling adds a condition, the condition binds.

- **Q1: (a).** K6 ships binary64 kernel observations now. **K6b**, in `H` after K4 merges, adds limbs per entry, work units and seconds per work unit for W1.
- **Q2: (b).** `H` takes `nonlinear_integration` as a path dependency. **Condition:** the lock may gain only in-repo packages, no registry package. List the lock diff at A1.
- **Q3: (a).** C = 8 GiB (RSS watchdog), with the heap cap at C − 512 MiB, and the admission rule as written. Dense at 1,000 members is admitted. Its first run confirms or refutes the premise that the kernel peak is below P1's; a refutation is a stop.
- **Q4: yes.** One dense run of the invented 1,364-member chain at C = 16 GiB (heap cap 15.5 GiB).
  - It is the **last** run in the schedule, alone on the host, in a slot ROOT grants separately after reviewing the B results below it.
  - It is disclosed as outside the sealed set.
- **Q5: yes.** Use `H`'s own invented grids, sparse mode only, disclosed as outside the sealed set, under the same admission rule.
- **Q6: (a).** Use public-API stage boundaries, and disclose the bundling. No FK timing hooks.
- **Q7: (a).** As written. Report the projection before the dense 1,000-member runs, and stop above two hours.
- **Q8: (a).** The runner goes in `H/runner/`, the compact packet in `H/observations/k6/`, and the raw JSONL in `T3/IMPLEMENTATION/K6/_run_records/`.
- **Q9: (b).** Add `P/tests/test_performance_harness_runner.py` to the write set. **Conditions:**
  - it runs in under 10 s;
  - it spawns only self-limiting children (a 128 MiB cap);
  - it is named so that no `test_ci_` selector picks it up;
  - it skips nothing. If DEC-025's sandbox denies `ps` or process-group kills, report at checkpoint 0.
- **Q10: (a).** Use a Rust generator that keeps R1's node order, with the section formed from explicit products and π. Record a one-off check of all 24 models against `references.py --model`. A difference is a stop; never edit a reference.
- **Q11: as recommended.** Commit kernel-model sha256s for all 33 models, and cite P1's product request hashes for V-P.
- **Q12: (a).** Observe the dense LU lane and the identity-order lane where the admission rule admits them, against F1b's 24-byte estimate. CONT at 10,000 members is never run.
- **Added (RV16-N4, 2026-09-28): no run at 10,000 members or more in any mode that materializes an n² matrix,** the dense-LU lane mode included. The binary's refusal by name, test G and K6-M12 cover every such mode.
- **Q13:** one slice, with checkpoints 0, A1, A2, B, C and D.
  - Until ROOT releases the host, work is read and design only (checkpoint 0).
  - No run above 100 members before ROOT approves the `--plan` schedule.
- **Stale-design items 1–12:** recorded as rulings, as listed in the last section of this brief. `DESIGN.md` is not edited.

## Scope

### 1. The split: binary64 now, W1 later (Q1)

- **The row needs only K1.** K6's binary64 kernel observations (sparse and dense) can ship now.
- **W1 is not available.** §4.8's "for W1, limbs per entry" (`DESIGN.md:828`) and several expectations placed on K6 need K4's merged method:
  - "K6 turns work units into time on the owner's Mac under `_COMMON.md`'s interleaved-run rule" (`I12_K4_IMPLEMENTATION.md:584`);
  - "W1's runtime multiple is an estimate until K6 measures it" (`DESIGN_NUMERICS/D5_TRIGGER.md:260`);
  - "the cost of a 1024-bit verification on large models, are K6's and V-K's to measure" (`DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION.md:574`).
  - K4's `retained` module is private (`FK/structural.rs:5`), and "the first consumer outside FK adds the `pub use`" (`ROOT_RULINGS_V1.md:1149`).
- **Recommended (Q1(a)):** K6 now, binary64 only, with the runner built to drive any observation binary. **K6b** follows K4's merge in `H`, under its own brief: W1's limbs per entry, work units by stage and precision on RF-LARGE, peak memory per precision, and seconds per work unit by interleaved runs.

### 2. What ROOT's limits and ceilings need from K6

**For D-8's W1 limits** (`ROOT_RULINGS_V1.md:1173-1176`; §4.1.7, `DESIGN.md:518-519`):
1. **The binary64 baseline cost of the ordinary attempt** that W1's budget is added to: per-stage median and minimum time, and peak memory, for the kernel sparse path at 10 to 10,000 members and the dense path at 10 to 1,000, on every RF-LARGE family and the DEC-053 nine.
2. **The deterministic counts W1's work scales with:** pattern entries, free lower entries (all and nonzero), RCM profile entries and half-bandwidth, and contributions. W1's factor runs on the same RCM profile ("RCM ported into `factor.rs`", `ROOT_RULINGS_V1.md:1134`, `:1144`). With K4's golden work counts (`I12_K4_IMPLEMENTATION.md:400`, `:582`), ROOT can then predict W1's work at 10,000 members without running W1 there.
3. **The growth fits,** so a limit can be checked against the gate's coverage condition, which "must fit inside them" (`DESIGN.md:1270`).
4. **The runner,** so that K6b's and V-K's W1 runs are measured in the same way.
5. What K6 cannot give: seconds per work unit (K6b or V-K), and W1's memory per precision.

**For the dense-scrutiny ceiling** (`DESIGN.md:804`; `I13_F1B_IMPLEMENTATION.md:126`):
1. **The measured peak against F1b's estimate** 96·n² (`PP:2871-2878` on `130445db2`), on SA's `DenseScrutiny` path, the path the estimate counts. Both in **peak requested heap bytes** (the estimate's own terms, deterministic) and **peak RSS** (what the host sees). Sizes: 66, 606 and 6,006 DOFs (RF-LARGE 10, 100 and 1,000), the DEC-053 nine, and, if ruled, a run at the ceiling (Q4).
2. The dense wall time at those sizes. P1 recorded product-level dense runs at 1,000 members taking 204–487 s, and two timing out at 1,800 s (`DETECTION/RETURN.md:254-265`, `:287`), so a ceiling may also have a time dimension.
3. **The estimate at 10,000 members, not run:** 96 × 3,600,720,036 = 345,669,123,456 bytes, about 322 GiB (n² from `IMPLEMENTATION/K1/RETURN.md:410`).

**For the observation-lane guard:** the lane's measured peak against 24 bytes per identity-order profile entry (`PP:2950-2958` on `130445db2`), at the sizes the ascent rule admits (Q12). CONT at 10,000 members has 675,179,982 such entries (`ROOT_RULINGS_V1.md:1454`), an estimate of about 16.2 GB, and is never run.

**For a sparse profile ceiling** (`DESIGN.md:804`; F1b sets none, `ROOT_RULINGS_V1.md:1463`): bytes per RCM profile entry and per pattern entry on the kernel sparse path. RF-LARGE's profiles are small (at most 679,915 entries at 10,000 members, `IMPLEMENTATION/K1/RETURN.md:407-414`), so they cannot stress a profile ceiling (Q5).

**What the ceiling rulings need that K6 alone cannot give:**
- **PP's own overhead** in dense scrutiny beyond the SA path: the legacy lanes if they overlap the gate's peak, the envelope, the receipts, and the n ≤ 256 dense view for source recovery (`ROOT_RULINGS_V1.md:1274` records F1b's `DENSE_SOURCE_DOF_LIMIT`). This is V-P's product-level measurement, or a product spot-check ROOT may ask for.
- **The target-machine policy.** Measurements come from one 128 GB Mac with no swap (`REVIEW/_run_records/k5_review/toolchain.txt:1`). How much memory the product may assume on a user's machine is a product decision; F1b reported the refusal class to the owner (`ROOT_RULINGS_V1.md:1261`).
- **Other platforms.** RSS and allocator behaviour on Linux and Windows are not measured.
- **Whether a ceiling should also bound time.**

### 3. Models and sizes (Q10, Q11)

- **RF-LARGE, 24 cases** (`REFERENCES/README.md:193-199`): CHAIN (a cantilever of 3 m members) and TREE (a comb) are determinate; CONT (n/2 spans of 6 m, continuous over pinned supports) is indeterminate. Each at n = 10, 100, 1,000 and 10,000 members, axis-aligned (AX) and rotated by Q3 (ROT). Every case has n + 1 nodes (`references.json`: 11 nodes at n = 10, and the generator note of the n = 10,000 cases).
  - The JSON abbreviates the models above 1,000 members; `references.py --model <case id>` prints the complete model (`REFERENCES/README.md:43`).
  - References: `references.json` sha256 `7b176dbb…`, `references.py` `80d473a7…` (frozen at `c0f14201c`, `DETECTION/RETURN.md:20`). Never edit them.
- **The nine DEC-053 observations**: the harness's invented chains and grids (`H/src/lib.rs:704-763`), as kernel fixtures.
- **Node and member order are R1's.** RCM's start and the identity-order profile depend on numbering. CONT lists its support nodes before its midspan nodes (`references.json`, `RF-LARGE-CONT-n00010-AX`), so each midspan node couples to support nodes about n/2 positions earlier; its identity-order profile at 10,000 members is 675,179,982 entries (`ROOT_RULINGS_V1.md:1454`). The kernel model must use exactly R1's order, which is also P1's request order (`DETECTION/scripts/gen.py.txt:101`).
- **Inputs.** Each exact input is rounded once to binary64, as P1 did (`DETECTION/RETURN.md:23`).
  - **The section.** R1 gives E, G, OD and ID (`references.json`, section `N` and `S`). The kernel needs A, I and J. PP forms them with `powi` (`PP:8515-8519`), a function of unspecified precision under ROOT's standing lesson (`I14_K5_IMPLEMENTATION.md:150-152`). **Recommended (Q10):** explicit products and `std::f64::consts::PI` only, with the formula stated. K6's section bits may then differ from PP's by rounding, so kernel outcomes are compared with product outcomes as context only.
- **Hashes (Q11).** The sha256 of each kernel model's canonical serialization (33 models) is committed. The 24 RF-LARGE product requests already have hashes (`PLATFORM_CALIBRATION_MAC/gate/gen_out_sha256.txt:122-145`, from P1's `gen.py`), which K6 cites for V-P and does not regenerate.
- **The cross-check (Q10).** A standard-library script, run once and recorded, compares every kernel model with `references.py --model <case id>` rounded once, for all 24 cases.

### 4. The observation binary (Q2, Q6)

- **One process per (model, mode).** The binary takes a model (by id, or a canonical model file), a mode (`sparse`, `dense`, and the lane modes of Q12), a repeat count, and a heap cap.
- **Five in-process repeats** of the full stage sequence. It prints one JSONL line per stage per repeat, then one counts line and one summary line. The runner computes the median and the minimum per stage (`DESIGN.md:827`).
- **Stages (Q6), at public-API boundaries.** The design's list is "assembly, audit, factor, rcond, solve, residual, recovery" (`DESIGN.md:827`). The public API bundles them, so the recommended map is:
  - model generation (recorded, not a solver stage);
  - `assembly`: `assemble_sparse_stiffness`, plus `to_dense` in dense mode;
  - `evidence`: `SparseAssemblyEvidence::new`, plus `dense_symmetry_view` in dense mode;
  - `ledger`: the `AssembledForce`;
  - `geometry`: `SparseAssemblyEvidence::geometry` (`SA:540`);
  - `prepare`: validation, the contribution **audit** and scaling;
  - `factor`: `order_sparse_structural` and the skyline LDLᵀ, or the dense Cholesky;
  - `finish`: the triangular **solve**, the **residual**, **rcond** and the intended-action audit, which `finish_*` runs as one call (`FK/structural.rs:1883-1897`);
  - `recovery`: `SparseStiffness::reactions` and each element's end actions at b = 0 (`FrameElement::force_scaled_end_actions`, `FK/lib.rs:954`, `:991`);
  - `entry`: SA's `solve_assembled_with_formation_check` (selected) and `solve_assembled`, end to end, as the product calls them. The difference is the formation check's cost, as an observation.
  - The staged sequence must give displacements and report bit-identical to SA's `entry` in the same mode (Required tests E). Otherwise the stage map is not the product's kernel path, and the work stops.
- **The allocator.** A counting global allocator in the binary tracks current and peak requested bytes, per stage and per repeat, with a hard cap: a refused allocation returns null and the process aborts, as the Mac gate's probe does (`PLATFORM_CALIBRATION_MAC/gate/heap_cap_appended_to_p1_probe.rs.txt:1-38`).
  - Peak requested bytes are a deterministic count for a given binary and input. That makes them the right quantity to compare with F1b's estimates, which count requested bytes.
  - It lives only in the binary, never in `H`'s library, whose consumers must not inherit it.
- **Refusals in the binary itself** (belt and braces for the host rule, `I8R_K1_RESUME.md:32-35`):
  - dense mode is refused, before any n² allocation, for a model with 10,000 or more members;
  - any mode is refused when its derived estimate exceeds half the cap the runner passes (Q3).
- **The counts line.** Nodes, members, DOFs, free DOFs, pattern entries (both triangles), lower entries, free lower entries (all and nonzero), RCM profile entries and half-bandwidth, identity-order profile entries (an O(nnz) count, as F1b's `observation_lane_profile` forms it, `PP:2960-2993` on `130445db2`), contributions, dense entries n², F1b's two estimates, and the model's sha256.
- **The parity lines** (§4.8 items 1–3, K6's part; Scope 7).

### 5. The runner (Q8, Q9)

- **Standard-library Python only**, in `H/runner/` (Q8). It drives any observation binary, so K6b, V-K and V-P can reuse it by path.
- **Process tree.** The runner starts `/usr/bin/time -l` (macOS) or `/usr/bin/time -v` (Linux) around the binary, in a new session.
  - **The watchdog must poll the binary, not the `time` wrapper.** `ps -o rss= -p <time's pid>` reads the wrapper's own small RSS. The runner finds the binary's PID (for example `pgrep -P <wrapper pid>`) and polls that.
  - **A kill is SIGKILL to the process group,** so no orphan survives.
- **macOS:** the 100 ms `ps -o rss=` watchdog (KiB), killing above the cap and recording `killed_by_rss_watchdog`, the last reading, the poll count and the time (`DESIGN.md:826`). The in-process heap cap is the primary limit (Q3). RSS lags allocation (it counts touched pages, sampled every 100 ms), and under memory pressure macOS compresses a process's pages out of its resident set, so RSS can understate the footprint. The calibration's crash happened "with the compressor at 100% of its limit and no swap" (`PLATFORM_CALIBRATION_MAC/RECORD.md:83-87`).
- **Linux:** `resource.setrlimit(RLIMIT_AS, (cap, cap))` in the child before exec, as P1 did (`DETECTION/scripts/run.py.txt:20-31`). macOS cannot set it (`PLATFORM_CALIBRATION_MAC/RECORD.md:63-65`).
- **Peak RSS, with units recorded:** `/usr/bin/time -l` reports bytes (macOS); `-v` reports kbytes (Linux). `os.wait4`'s `ru_maxrss` is a cross-check (P1's method, `DETECTION/scripts/run.py.txt:62`); it is bytes on macOS and KiB on Linux. So is the maximum `ps` sample.
- **Timeouts:** 600 s, and 1,800 s at 1,000 members or more, as P1 used (`DETECTION/scripts/run.py.txt:21-22`; Q7).
- **Classification per process:** `ok`, `timed_out`, `killed_by_rss_watchdog`, `heap_cap_abort`, `rlimit_abort`, `refused_by_binary`, `error`.
- **Metadata per packet:** OS version, architecture, CPU model, core counts, memory size, rustc and cargo `-Vv`, the build profile, the binary's sha256, the source commit, and the `git archive` tree. **No hostname, user name, serial number or path** (`_COMMON.md:40-41`).
- **Load per run:** `os.getloadavg()` before and after, and `kern.memorystatus_level` on macOS, as G1 recorded (`GATE_BASELINE_MAC_E7D930D49/RECORD.md:31`).
- **Modes:** `--plan` (the schedule, with each run's derived estimate, cap and admission, and no child process); `--smoke` (the dry run, checkpoint A2); `--run` (checkpoint B, only in ROOT's slot).
- **Outputs:** raw JSONL per process, one runner record per process, and the aggregated packet (Q8).

### 6. Where each run happens, and the Mac rules (Q3, Q4, Q7)

- **The Mac** (`aarch64-apple-darwin`, 128 GB, no swap; `REVIEW/_run_records/k5_review/toolchain.txt:1`; `I8R_K1_RESUME.md:26`) runs every observation:
  - **Sparse:** all 24 RF-LARGE cases at every size, and the DEC-053 nine.
  - **Dense:** RF-LARGE at 10, 100 and 1,000 members, and the DEC-053 nine. **Never at 10,000 members or more** (`I8R_K1_RESUME.md:32-35`); the 10,000-member dense figure is the estimate only (Scope 2).
  - **The legacy lanes,** where Q12's admission allows.
- **Linux runs none of the observations.** No Linux host is available to T3 on the Mac. Hosted CI runs only `cargo test --offline --locked` on `H` (debug), which builds but does not run examples (`P/tools/release/check_release_readiness.py:28-31`, `:146-160`; `P/tools/ci/numerical_ci.py:26-47`; `.github/workflows/piping-desktop-e2e.yml:172-195`). The CI checkout omits `projects/*/execution/` (`:188`), so no CI test may read T3's records or R1's references.
  - **The Linux `RLIMIT_AS` path** is implemented, unit-tested with a mocked `resource` module, and exercised live only when a Linux host runs the runner (a Linux DEC-025 sweep under Q9(b), or V-P). This is disclosed.
  - **The only Linux peaks known** are P1's, at product level on `c61a540ea` (`DETECTION/RETURN.md:224-289`): about 34–35 MiB at 10 members, 50–53 MiB at 100, 3,373–3,617 MiB at 1,000 in either mode, and aborts at the 6 GiB `RLIMIT_AS` at 10,000.
- **The cap and the ascent rule (Q3; recommended):**
  - the RSS watchdog's cap C = 8 GiB, with the heap cap at C − 512 MiB, so a heap overrun aborts deterministically before RSS reaches C. A watchdog kill then signals non-heap growth, which is a finding;
  - **a run is admitted** only if a known Linux peak for that size and mode is at most C/2, or its derived estimate from the counts, multiplied by the largest actual-to-estimate ratio measured at smaller sizes of the same family and mode (by 2 where none has been measured yet), is at most C/2;
  - **dense at 1,000 members** is admitted by the design's letter: P1's 3,617 MiB ≤ C/2 = 4 GiB. P1's product-level dense path on main held more n² buffers than the kernel path does (dense K, reduced K, two evidence arrays, the prepared matrix and two `Expansion` arrays, `DESIGN.md:207-219`). At 3,617.2 MiB over 36,072,036 entries it is about 105 bytes per entry, including the process baseline, against F1b's count of 96 for SA's path. **That the kernel peak is below P1's product peak is a premise:** the first dense 1,000 run confirms it, under the heap cap;
  - **ascent:** each family goes 10 → 100 → 1,000 → 10,000, and a size runs only after the previous size's measurement has been recorded and the rule re-evaluated;
  - a watchdog kill or heap-cap abort on an admitted run is a **stop:** the prediction was wrong.
- **Near the ceiling (Q4):** one dense run of an invented 1,364-member chain (RF-LARGE-CHAIN geometry, 8,190 DOFs, estimate 96 × 67,076,100 = 6,439,305,600 bytes, just under F1b's 6,442,450,944), at C = 16 GiB, alone on the host. It is outside the sealed set and disclosed.
- **Repeats and time (Q7):** five in-process repeats per process. A process whose first repeat exceeds 600 s stops after it, and records `repeats_completed: 1` with the reason. Modes alternate per model (which mode runs first alternates by model index), in one pass. Before the dense 1,000-member runs, report the measured first-repeat time and the projected slot to ROOT, and wait for its go-ahead if the projection exceeds two hours.
- **Host (`I8R_K1_RESUME.md:24-40`; `_COMMON.md:32`):**
  - builds: `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, `-j 8` at most, `RUST_TEST_THREADS=4`, at most two cargo jobs of your own, your own target `<wt>/k6-target`;
  - observation runs: `--release`, from a `git archive` of the exact commit; **one observation process at a time;** a quiet host in a slot ROOT grants (no cargo, no gate, no sweep); load recorded per run; the memory guard running (floor 35%, `PAUSE_2026-09-28.md:16`); check `<wt>/guard/memguard.log` after every heavy phase;
  - **until ROOT releases the host, read and design only.** A timed gate run is using the Mac (F1b's part 2; `PAUSE_2026-09-28.md:10`, `:24`).

### 7. The parity checks of §4.8 that are K6's

§4.8's parity protocol (`DESIGN.md:815-821`) spans several slices. K1 covered N01–N09, R01–R07, NP-B, NP-D and the T0R references at kernel level (`IMPLEMENTATION/K1/RETURN.md:322-345`). K6 takes the R1 family it runs, RF-LARGE, and the DEC-053 nine:
1. **Bitwise K** (`DESIGN.md:816`): every stored entry of the pattern assembly equals the dense assembly's entry, and every entry outside the pattern is +0.0. It is recorded at up to 1,000 members, and tested at up to 100 members and on the nine.
2. **M03 outcome class parity between modes** (`:817`): recorded per model at up to 1,000 members. "A divergence near a screen boundary is recorded, never tuned."
3. **The DEC-053 parity basis** (`:818`; `H/README.md:54`): the sparse–dense displacement delta relative to the dense solution's magnitude, recorded, and asserted at the existing 1e-9 in the tests only. No new tolerance.
- Items 4–6 (two modulus bases, the nonlinear models, relabelling) are not K6's.

### 8. What may change in published bytes: nothing

- K6 writes only `H`, K6's records and, if Q9(b) is ruled, one pytest file. **No crate outside `H` depends on the harness:** its package name occurs only in `H/Cargo.toml:2`, `H/Cargo.lock:35` and a provenance copy of the lock, `P/provenance/build-artifacts/core__solver__performance_harness__Cargo.lock:34`, which nothing in `P/tests`, `P/tools` or `tools/` reads.
- **So no binary that T9 or the both-entry gate builds can change.** Re-verify both scans on your base, and record them.
- **T9: not run.** Byte identity holds by construction; the evidence is the path scan and the dependency-closure scan. ROOT may ask for a run.
- **The both-entry gate: not run.** It compares published envelopes through PP's two entries. K6 touches no crate in PP's or the headless runner's dependency closure, so every run is byte-identical by construction. K1 and K2b changed kernel code and ran no gate (`ROOT_RULINGS_V1.md:858`, `:918`); K6 changes no kernel code at all.

## Write set (re-locate every line on your base)

| File | Change | Status |
|---|---|---|
| `H/Cargo.toml` | the observation binary's target; the `NI` path dependency, if Q2(b) is ruled | certain / proposed |
| `H/Cargo.lock` | in-repo path packages only (`curved_bend`, `nonlinear_integration`, `nonlinear_supports`; `NI/Cargo.toml` deps), if Q2(b) is ruled. No registry package. The lock is tracked although `H/.gitignore:2` lists it; ROOT commits it explicitly | proposed |
| `H/src/lib.rs` | one additive `pub mod` line and nothing else. The DEC-050/053 pytest pins read this file as text | certain |
| new `H/src/k6/**` | the model generator, the canonical serialization, the counts, the staged sequence, the parity checks | certain |
| new observation binary (`H/examples/k6_observe.rs`, or a `[[bin]]` so integration tests can run it through `CARGO_BIN_EXE_*`; propose at checkpoint 0) | the counting capped allocator, the CLI, the JSONL output, the binary's refusals | certain |
| new `H/tests/k6_*.rs` | Rust tests; the allocator and abort tests in their own test binary | certain |
| new `H/runner/k6_runner.py` and `H/runner/test_k6_runner.py` | the runner and its unit tests (standard library) | certain |
| new `H/observations/k6/` | the packet, the model hashes and `SHA256SUMS` (Q8) | proposed |
| `H/README.md` | a K6 section: scope, the observation-only boundary, how to run | certain |
| new `P/tests/test_performance_harness_runner.py` | a thin pytest wrapper so DEC-025's pytest surface runs the runner's tests (Q9(b)) | **proposed; a declared extension** |
| `T3/IMPLEMENTATION/K6/**` in `<wt>/k6` | records | certain |

**Not in scope. Stop and ask before touching any of these:**
- FK, SD, NI (including SA), PP, `P/core/runner/headless/**`, `curved_bend`, `straight_pipe`;
- the legacy harness's functions, tests, constants and example (`H/src/lib.rs` beyond the one line; `H/examples/sparse_default_promotion_observation.rs`);
- `P/validation/benchmarks/**`, including the DEC-050/053 JSON records and `numerical_robustness` (V-K, V-P);
- `P/tests/test_sparse_default_promotion_observation.py`, `P/tests/test_sparse_suitability_observation.py`, and any `P/tests/test_ci_*.py`;
- `P/provenance/**`, `.github/**`, `tools/**`, `P/tools/**`;
- R1's `REFERENCES/**`, the T0R references, `GATE/*.json`, and every committed fixture and hash pin.

## Constraints

- **Observation only.** Tests assert determinism of counts and structure, the existing 1e-9 DEC-026/DEC-053 criterion, and the harness's own mechanics. No test or record asserts a time or memory bound.
- **The claims RETURN may make:**
  1. the observed growth fits of peak requested heap and of peak RSS against n, per family and mode (log-log least squares, with points and residuals);
  2. the actual-to-estimate ratios for F1b's two estimates at the measured sizes.
  Everything else (times, speed ratios) is an observation with its load, not a claim. A comparison of two builds needs the interleaved A, B, A, B method (`_COMMON.md:32`; `ROOT_RULINGS_V1.md:756`), and K6 makes none.
- **Baselines.** At 10 and 100 members, peak RSS is dominated by the process baseline (about 34 MiB in P1's runs, `DETECTION/RETURN.md:230-241`). Fit on peak requested heap, and report the RSS fit beside it with the baseline stated.
- **No function of unspecified precision** in any value K6 compares or hashes: no `powi`, `powf`, `hypot`, `exp*`, `ln*`, trigonometric or hyperbolic function, or `cbrt` in the generator or the section formula (the standing lesson, `I14_K5_IMPLEMENTATION.md:150-152`).
- **No registry dependency** (no `sha2`, no `libc`). Rust tests compare canonical bytes; sha256 is computed in Python (`hashlib`).
- **Memory in tests:** no dense matrix at 1,000 members or more in any test. Debug tests stay at 100 members or fewer and the DEC-053 nine. A test that could reach a larger allocation runs under the capped allocator in its own test binary, so a regression aborts that binary instead of exhausting the host.
- **The CI budget.** The hosted numerical job runs all manifests in 45 minutes (`.github/workflows/piping-desktop-e2e.yml:176`). Record `H`'s debug suite time before and after; report an increase of more than two minutes.
- **Every general claim is derived and independently checkable** (ROOT's standing lesson). That covers the closed-form counts, the staged sequence's equality with SA's entry, the byte-identity claims of Scope 8, every mutant equivalence, and the ascent rule's admissions.
- **`dead_code`:** items with no non-test caller get a per-item `#[allow(dead_code)] // <consumer>` ("K6b API", "V-P API"). The non-test build has no warnings.

## Basis (read in this order)

T3's records are read at `<wt>/numerics`, which carries ROOT's latest rulings. Code is read on your base.

1. Root `AGENTS.md`, `agents/AGENT_TASK.md`, `_COMMON.md`, and `I8R_K1_RESUME.md` "The Mac host" and "Platform calibration".
2. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (`fb62ef4a…`; hash-pinned, don't edit): §2.1 (`:205-230`); §4.1.7 (`:516-520`); **§4.8, all of it** (`:786-834`); §4.10's kernel lane and its memory line (`:850-858`, `:917`); §6's K1, K4, K6, V-K and V-P rows and the order (`:1014`, `:1019`, `:1021`, `:1023`, `:1031`, `:1034-1038`); §7.2's RF-LARGE row and the host line (`:1080`, `:1087`); §7.5 (`:1132-1143`); D-7 and D-8 (`:1269-1270`); V1-N6 (`:146`).
3. `T3/ROOT_SELECTION_DESIGNS.md:39-42` (C4).
4. `T3/ROOT_RULINGS_V1.md` (at drafting, sha256 `47936458…`):
   - "K4: spawn and rulings" (`:1125-1150`) and **"K4: Q5 amended"** (`:1169-1177`);
   - **"F1b: spawn and rulings"** (`:1223-1261`), and "F1b: rulings on I13's checkpoint-0 plan" (`:1263-1298`), for the guard's constant;
   - "F1b: rulings on I13's A2" (`:1395-1418`);
   - **"Resume after the pause, and F1b's heap-cap finding"** (`:1447-1464`);
   - "S11-G performance finding withdrawn; the method for performance claims" (`:752-756`);
   - "K1: spawn timing and no both-entry gate" (`:851-859`), "K2b: kernel only" (`:913-927`);
   - "K4: A1 findings F-1 to F-3" (`:1351-1371`) and the D1 5a.3 sections (`:1420-1516`), for K4's status.
5. The host records: `PLATFORM_CALIBRATION_MAC/RECORD.md` (`94f57abe…`), §2's two deviations and §4, the crash; `GATE_BASELINE_MAC_E7D930D49/RECORD.md` (`7ed3ef89…`); `PAUSE_2026-09-28.md`; `OPERATING_NOTES_FOR_LOCAL_ROOT.md` §3 and §6; `HANDOFF_2026-09-28_TO_LOCAL.md` §4; `OWNER_DIRECTION.md:23-34`.
6. P1's measurements and runner: `DETECTION/RETURN.md` §5 (`:222-289`), `DETECTION/scripts/run.py.txt` (`8d9a32a3…`) and `gen.py.txt`.
7. **The references:** `REFERENCES/README.md:193-199` (RF-LARGE), `references.json` (`7b176dbb…`), `references.py` (`80d473a7…`).
8. **Merged records:** `IMPLEMENTATION/K1/RETURN.md` §9 and §11 (`:322-345`, `:399-416`) and §12 (the F1b interface).
9. **The briefs** `I12_K4_IMPLEMENTATION.md` (Q5, `:575-590`), `I13_F1B_IMPLEMENTATION.md` (Q8, `:124-129`, `:653-660`; the guard, `:169-175`) and `I14_K5_IMPLEMENTATION.md` (for format and the constraint lists).
10. **The code on your base:**
    - `H` entire: `README.md`, `Cargo.toml`, `.gitignore`, `Cargo.lock`, `src/lib.rs` (`:1-50`, `:597-763`, `:765-955`), `examples/sparse_default_promotion_observation.rs` (`:164-173`, its `ps -o rss=` precedent);
    - `FK/structural/sparse.rs:1-35`, `:200-330`, `:470-760`, `:845-1010`, `:1205-1280`, `:1711-1800`; `FK/structural.rs:1-75`, `:1157-1200`, `:1845-1930`; `FK/structural/formation_check.rs:40-80`; `FK/lib.rs:985-1000`, `:1286-1360`, `:1631`;
    - `SD/structural.rs:1-130`; `SD/lib.rs:630-735`; `SD/structural/k1_tests.rs:700-814`;
    - `SA:440-700`, `:890-915`, `:1212-1290`; `NI/Cargo.toml`; `NI/src/lib.rs:43-46`;
    - `PP:2508`, `:4464`, `:4601`, `:8515-8519`;
    - **F1b at `130445db2`:** `PP:2809-3060` (the guard, the lane guard, their constants and formulas).
11. The CI plumbing: `P/tools/ci/numerical_ci.py:26-47`, `P/tools/release/check_release_readiness.py:28-31`, `:70-82`, `:146-160`; `tools/validation/discover_test_surfaces.py:151-158`; `.github/workflows/piping-desktop-e2e.yml:53-54`, `:172-195`.
12. `.agents/skills/chirality-change/SKILL.md`, for the change record.

## Base, branch and paths

- **Branch:** `codex/piping-k6-<yyyymmdd>`, from current main. ROOT creates it in `<wt>/k6` and records the SHA at spawn.
  - At drafting, main's piping tree equals `24dea2dae`'s. **K5 (PR #1044) changes SA's geometry** (`I14_K5_IMPLEMENTATION.md:97-103`). If it merges before K6's runs, K6's `geometry` and `entry` stages measure K5's code; record which.
- **Target:** `<wt>/k6-target`. **Scratch:** `<wt>/scratch/i15`. **Mutants:** one clean `git archive` copy and one clean target per mutant, under `<wt>/k6-mut/<mutant>/`; at most three at once at `-j 4`; delete each target afterwards.
- **Python:** `<VENV>`, standard library only.
- **Observation builds:** `--release` from `git archive` of the exact candidate commit into `<scratch>`, never from the working tree.

## Coordination

- **F1b (I13; PR pending).** Its write set is PP, `source_recovery.rs`, NI's `s11k_tests.rs` and PP tests (`I13_F1B_IMPLEMENTATION.md:248-271`). **No overlap with K6.**
  - F1b's constants are PP-private. **K6 quotes them, with their source commit, and never imports them.** If F1b changes them before it merges, K6 re-labels its ratios; it re-runs nothing.
  - The observation-lane guard and the dense guard stay F1b's. K6 measures; it proposes no constant.
- **K4 (I12; blocked on D1 5a.3).** No overlap. K6 does not read `retained`. K4's work counts join K6's profile counts under Q1.
- **K5 (I14; PR #1044).** No overlap (`I14_K5_IMPLEMENTATION.md:147` lists `performance_harness` as K6's). See Base.
- **V-K and V-P.** They reuse K6's runner by path (Q8), and own product-level runs and `numerical_robustness`.
- **Merge order.** Any order. If main moves under K6, merge main in (a merge commit), re-run `H`'s suite, and re-run only the observations whose measured code changed (K5's geometry, for example), disclosing the base of every figure.

## Required tests

The predicate, wherever a value is compared, is the unchanged `|obs − exp| ≤ 1e-9·max(|exp|, scale)`, or the harness's existing DEC-053 basis. No new tolerance anywhere.

**A. Nothing existing moves**
- `H`'s existing tests pass unchanged; the legacy functions, constants and example are byte-identical in source (a diff in RETURN).
- `P/tests/test_sparse_default_promotion_observation.py` and `test_sparse_suitability_observation.py` pass.
- `H/src/lib.rs`'s diff is the one `pub mod` line.
- The non-test build has no warnings.

**B. Models**
- The generator's canonical bytes for the 10- and 100-member RF-LARGE models and the DEC-053 nine equal committed files or committed constants (CI-runnable; no reference read).
- The recorded cross-check (Scope 3): all 24 RF-LARGE models equal `references.py --model` rounded once, node and member order included.
- `models_sha256.txt` matches the generator's output for all 33 models (a Python check in the runner's tests).
- The section formula is pinned by exact bits on one section.

**C. Counts**
- Closed forms, derived in RETURN: pattern entries = 36 × (nodes + 2 × members) (`IMPLEMENTATION/K1/RETURN.md:416`); contributions = 144 × members; dense entries = DOFs².
- K1's table as a cross-check for CHAIN and TREE at every size and orientation (`IMPLEMENTATION/K1/RETURN.md:407-414`).
- The RCM profile, half-bandwidth and identity-order profile against an independent oracle proposed at checkpoint 0 (for example a standard-library port of `reverse_cuthill_mckee` and the skyline rule, run once and recorded). The reviewer re-derives them independently.

**D. Parity (Scope 7)**
- Bitwise K, outcome class parity and the DEC-053 basis at up to 100 members and on the nine.

**E. The staged sequence**
- In each mode, the staged sequence's displacements and report `Debug` are bit-identical to SA's `solve_assembled` (and, with the formation source, to `solve_assembled_with_formation_check`) at up to 100 members and on the nine. The observation run records the same equality at every size.

**F. The allocator**
- Accounting: alloc, dealloc, realloc up and down, and zeroed allocation, over a known sequence.
- The cap boundary: exactly at the cap succeeds, one byte over fails.
- Determinism: two runs of the same model and mode give equal peak requested bytes.
- The abort: the binary at a small heap cap on a 100-member dense model aborts, and the runner classifies it `heap_cap_abort` (a subprocess test).

**G. The binary's refusals**
- Dense mode on a 10,000-member model is refused by name, with peak requested heap far below 8·n² (the refusal comes before any n² allocation).
- A run whose estimate exceeds half the passed cap is refused by name.

**H. The runner** (standard library `unittest`)
- **The watchdog (macOS):** a deliberately growing child, a standard-library Python process that writes a page in each step, run under the `/usr/bin/time` wrapper with a 128 MiB cap. It must be killed by SIGKILL to its process group, with `killed_by_rss_watchdog` true, the last reading at or above the cap and within one poll's growth of it, and no survivor.
  - The child is **self-limiting:** it stops growing at 512 MiB and exits nonzero, so a broken watchdog cannot exhaust the host. Reaching the failsafe fails the test.
- **The negative control:** a child that grows to half the cap and exits 0 is not killed.
- **Linux:** the pre-exec function calls `setrlimit(RLIMIT_AS, (cap, cap))` only on Linux (with a mocked `resource`), and never on macOS.
- **Parsers:** recorded `/usr/bin/time -l` and `-v` outputs, with the units normalized; `ru_maxrss` units per platform.
- Classification, aggregation (median and minimum), the JSONL schema, the metadata's lack of host identifiers, and `--plan`'s admission decisions (the 10,000-member dense runs refused, with the reason).

**I. The dry run (A2):** `--plan` for the full schedule; `--smoke` runs the 10- and 100-member models in both modes and the DEC-053 nine end to end, the watchdog path, and the heap-cap abort path. Its peak memory is at most 512 MiB by construction.

## Observation runs (checkpoint B, not tests)

- **The schedule is `--plan`'s,** as ROOT approves it at A2. Every run is recorded, admitted or not, with its reason.
- **Per process:** the classification, wall time, the per-stage median and minimum over the completed repeats, peak requested heap per stage and overall, peak RSS from every source with units, the counts, the parity lines, load before and after, and `memorystatus_level`.
- **The packet:** per (model, mode), and the fits of Constraints; F1b's estimates against the measured peaks, with their ratios; the parity table; the outcome classes; the metadata.
- **Stops:** see Checkpoints.

## Mutants

Run from clean copies, with a NONE control first. Each mutant must be killed at an assertion; name the killing test. An allocator abort outside the abort tests is not a kill. A survivor is a defect to report; never weaken a test to kill it. Where no admissible control exists, derive the equivalence and report it; ROOT rules.

**The design's §7.3:** item 8 (a pattern entry omitted in sparse mode only, `DESIGN.md:1102`) is K1's and stays killed there; K6 re-runs no §7.3 item.

**K6's own:**

| # | Mutant | Intended kill |
|---|---|---|
| K6-M1 | The watchdog never kills (inverted or skipped comparison) | H, the watchdog test |
| K6-M2 | The watchdog compares KiB with bytes | H, the watchdog test or its negative control |
| K6-M3 | The watchdog polls the `/usr/bin/time` wrapper, not the binary | H, the watchdog test under the wrapper |
| K6-M4 | SIGKILL to the child only, not the process group | H, the no-survivor assertion |
| K6-M5 | `RLIMIT_AS` not applied on Linux, or applied on macOS | H, the mocked `resource` test |
| K6-M6 | `/usr/bin/time -l`'s bytes read as KiB, or `-v`'s KiB as bytes | H, the parser tests |
| K6-M7 | The mean in place of the median, or the minimum over the wrong axis | H, the aggregation test |
| K6-M8 | Pattern entries counted from one triangle, or the diagonal counted twice | C, the closed form |
| K6-M9 | The identity-order profile reported as the RCM profile, or the reverse | C, the oracle |
| K6-M10 | `dealloc` or `realloc` accounting wrong (peak drifts) | F, accounting and determinism |
| K6-M11 | The heap cap off by one (`>=` for `>`) | F, the cap boundary |
| K6-M12 | The binary's dense refusal at 10,000 members removed | G |
| K6-M13 | The admission rule admits a run whose estimate exceeds C/2 | H, `--plan`'s decisions; G |
| K6-M14 | Q3's rotation transposed in the generator | B, the canonical bytes (and the recorded cross-check) |
| K6-M15 | The generator builds n − 1 members, or R1's node order is not kept | B; C |
| K6-M16 | A staged sequence that skips the contribution audit | E, bit identity with SA's entry |
| — | Your own, at least two | — |

## Gates (ROOT runs the PR)

- **Suites.** `H`'s `cargo test --offline --locked` (debug, the CI form), with its time; the two DEC-050/053 pytest pins; the runner's tests (and the `P/tests` wrapper, under Q9(b)).
- **T9 and the both-entry gate: not run** (Scope 8). The evidence is the path and dependency-closure scans in RETURN, which the reviewer re-runs.
- **An independent complete-diff review** (a fresh reviewer; builds only from `git archive` of the exact head; `OPERATING_NOTES_FOR_LOCAL_ROOT.md:18-21`), with oracles independent of K6's tests:
  - an independent derivation of the counts, including the RCM profile;
  - the generator's cross-check against `references.py`, re-run;
  - the watchdog and heap-cap tests re-run, and K6's mutants re-run;
  - every number in RETURN and the packet traced to the raw JSONL and its sha256;
  - the ascent rule's admissions checked against the records;
  - the claims checked to stay within Constraints (fits and ratios only).
- **Hosted CI** green on the candidate head, and **the full-SHA dispatch** of `piping-desktop-e2e.yml` with a real 40-hex `target_base` that is an ancestor of the head, merging main first by a merge commit if needed (`OPERATING_NOTES_FOR_LOCAL_ROOT.md:67`). Record the numerical job's time.
- **DEC-025** under the owner's Mac decision (`OWNER_DIRECTION.md:23-34`):
  - the Mac sweep, whose only cargo failures are the three known platform tests, identical to Mac main;
  - pytest, vitest and the build pass;
  - hosted Linux CI's numerical cargo job supplies the clean Linux cargo run;
  - the deviation is recorded in the merge record.
- **GEN-8** (`pytest tools/practitioner_harness/test_live_baseline.py -k gen8`, from the repository root, in a Git working tree of the candidate) before every records commit that goes to main (`_COMMON.md:41`; `OPERATING_NOTES_FOR_LOCAL_ROOT.md:65`).
- **Merge** under the owner's standing Git authorization, when required CI passes and the review has no unresolved blocking finding, on the actual candidate head (Root `AGENTS.md`, "Execution and governance").
- **No native witness.** §7.5's 1,000-member witness (`DESIGN.md:1137`) is product-level and a join item.

## Checkpoints

End your turn at each one with a status for ROOT: the changed files, the results, and any stop. ROOT verifies, commits and resumes you.

- **0: a plan, before any code.** It covers:
  - the binary's CLI, the JSONL schema and the stage map (Q6), with each stage's exact call;
  - the generator (Q10): the family logic, R1's node order, the section formula, the canonical serialization, and the cross-check script;
  - the counts and their oracles, with the closed forms derived;
  - the allocator: accounting, cap, abort, and where it lives;
  - the runner: the process tree, how the binary's PID is found, the watchdog, the kill, `RLIMIT_AS`, the parsers, the classification, the metadata;
  - **the derived peak estimate per mode:** F1b's 96·n² for dense and 24 bytes per identity-order profile entry for the lane, each quoted with its source; for the sparse path and the dense LU lane, a formula derived from the code by counting the buffers alive at the peak, as F1b derived its constants;
  - **the schedule:** every (model, mode) with its derived estimate, its admission under Q3 and the reason, and the projected quiet-host time where P1's records allow a projection;
  - whether DEC-025's sandbox permits `ps` on a child and a process-group kill (Q9(b)). If it does not, report; no test is skipped;
  - the scans of Scope 8;
  - the test and mutant lists;
  - your position on every open question still unresolved.
- **A1: the Rust side.** A clean compile; A to G; a warning-free non-test build; `H`'s debug suite time.
- **A2: the runner and the dry run.** H and I; `--plan`'s full schedule for ROOT's approval. **No run above 100 members before ROOT approves the schedule.**
- **B: the observation runs,** in ROOT's slot, in the approved order, with the packet and the fits. ROOT rules on the results before C.
- **C:** the mutation table, with the NONE control first.
- **D:** CHANGE_RECORD (following `.agents/skills/chirality-change/SKILL.md`) and RETURN, with `_run_records/` and SHA256SUMS.

**Stop and report** (end your turn) on any of these:
- any change to a legacy harness function, test, constant, example or DEC-050/053 output, or a failing DEC-050/053 pytest pin;
- a needed edit outside the write set, above all in FK, SD, NI, PP, `validation/benchmarks` or `P/tests`;
- a staged sequence that is not bit-identical to SA's entry;
- a bitwise-K parity failure, a DEC-053-basis breach, or an outcome-class divergence between modes (record it, never tune it, and report);
- a model that differs from `references.py --model` (never edit a reference);
- a watchdog kill or heap-cap abort on an admitted run;
- a run the approved schedule does not contain, above all any dense run at 10,000 members or more;
- a projected slot above two hours before the dense 1,000-member runs (Q7);
- a SIGKILL from the memory guard (check `<wt>/guard/memguard.log`; do not retry blindly);
- a surviving mutant.

## Return

- **Files:** `T3/IMPLEMENTATION/K6/` on the K6 branch: CHANGE_RECORD, RETURN, `_run_records/` (raw JSONL, runner records, the cross-check, the scans, the oracle) and SHA256SUMS.
  - Use placeholders only (`<wt>`, `<scratch>`, `<VENV>`, `<home>`), with no machine paths, host identifiers or model identifiers.
  - State the platform (`aarch64-apple-darwin`, rustc 1.97.1) and that every observation is Mac-only.
- **RETURN covers:**
  - the files and their line counts;
  - each Scope item, with the design's words and the rulings;
  - the checkpoint-0 positions as ruled;
  - the schedule as run, with every admission and deferral;
  - **the results tables:** per (model, mode), the stage medians and minima, the peaks, the counts, the outcome class and the parity lines;
  - **the two claims** (Constraints), with their data, and the F1b estimate ratios;
  - **the derivations:** the closed-form counts, the staged sequence's equality with SA's entry, Scope 8's byte identity, and each "unreachable" path;
  - the mutation table;
  - `H`'s debug suite time, before and after;
  - the toolchain and host, and the delegation mechanism;
  - what was not done: W1 (Q1), Linux runs, product-level runs, and anything ROOT deferred.
- **RETURN has an "Interface for F1b's ceilings, D-8, K6b, V-K and V-P" section** with exact signatures and schemas, as K1's §12 did:
  - the binary's CLI, the JSONL and packet schemas, and how K6b adds a W1 mode;
  - the runner's CLI, its record schema and its admission rule;
  - the counts API, for K4's work formula;
  - the measured actual-to-estimate ratios for F1b's constants, with the sizes they cover and do not cover;
  - what ROOT still needs for each ruling that K6 did not measure (Scope 2).

## Open questions for ROOT (each with options and a recommendation)

**Q1. The split with W1.** The row needs only K1, but §4.8's "for W1, limbs per entry" and the expectation that "K6 turns work units into time" (`I12_K4_IMPLEMENTATION.md:584`) need K4, which is blocked on D1 5a.3 (`ROOT_RULINGS_V1.md:1516`).
- **(a) K6 now, binary64 only;** K6b in `H` after K4 merges, for W1's limbs per entry, work units and seconds per work unit, by interleaved runs through K6's runner.
- **(b) K6 waits for K4.** F1b's ceiling data and D-8's baseline are delayed by K4's whole path.
- **(c) K6 now; W1's measurements move to V-K,** which runs W1 in the kernel lane anyway (`DESIGN.md:858`) and precedes F2a (`ROOT_RULINGS_V1.md:1174`). That adds timing to a validation slice.
- **Recommendation: (a).** It serves F1b's ceilings and D-8's baseline now, keeps timing in the design's kernel-level home (`DESIGN.md:833`), and leaves K6b small. Whether K6b or V-K adds FK's `retained` export (`ROOT_RULINGS_V1.md:1149`) is ruled then.

**Q2. Which kernel path is "dense" and "sparse".**
- **(a) FK and SD only** (today's dependencies, `H/Cargo.toml:11-14`). K6 builds its own contributions and symmetry arrays. It would not be the product's path: SA's allowances come from SA's private `EvidenceParts`, and F1b's 96 bytes count SA's `DenseScrutiny` buffers.
- **(b) Add `NI` as a path dependency** and observe SA's `solve_assembled_with_formation_check` in both modes, the path F1b moves PP onto, plus the FK/SD staged sequence built from SA's public accessors. `H/Cargo.lock` gains three in-repo path packages and no registry package (`NI/Cargo.toml` dependencies; `NI/Cargo.lock` lists only in-repo packages).
- **Recommendation: (b).** It measures exactly what F1b's guard estimates, and the equality test (E) ties the stages to it.

**Q3. The Mac cap and the ascent rule.** §4.8 says Mac runs "start at sizes whose Linux peak is known to fit the cap with a factor of two to spare" (`DESIGN.md:827`). The only Linux peaks are P1's product-level ones (`DETECTION/RETURN.md:224-289`); none exists at kernel level, or for any 10,000-member sparse run.
- **(a) C = 8 GiB,** the heap cap at C − 512 MiB as the primary limit, and the watchdog as the design's backstop. A run is admitted when a known Linux peak, or the derived estimate times the largest measured ratio at smaller sizes, is at most C/2. The ascent goes size by size. Dense 1,000 is admitted by P1's 3,617 MiB. Dense at 10,000 is never run.
- **(b) The design's letter only:** admit only sizes with a known Linux peak at most C/2. No 10,000-member sparse run qualifies, which defeats the row's purpose.
- **(c) C = 6 GiB,** the gate's heap cap. Dense 1,000 is then excluded by the factor of two (3,617 MiB > 3 GiB).
- **Recommendation: (a).** The heap cap refuses at allocation, which removes the watchdog's coarseness for heap growth, the reason for the factor of two. The factor is still applied to every prediction.

**Q4. One dense run at the ceiling** (an invented 1,364-member chain, 8,190 DOFs, estimate 6,439,305,600 bytes against F1b's 6,442,450,944; C = 16 GiB, alone on the host).
- **(a) Yes:** the estimate's ratio is measured where the ceiling actually binds, not only 1.9 times below it (6,006 DOFs). One process, five repeats if the first is under 600 s.
- **(b) No:** RF-LARGE sizes only.
- **Recommendation: (a),** disclosed as outside the sealed set.

**Q5. A sparse-profile calibration ladder.** RF-LARGE's RCM profiles are at most 679,915 entries (Scope 2), too small to calibrate a profile ceiling.
- **(a) Add the harness's own invented grids** (`invented_grid_frame_fixture`, `H/src/lib.rs:403`) at sizes chosen at checkpoint 0 so that the largest profile exercises a large-bandwidth sparse factor within C/2, sparse mode only, outside the growth claim.
- **(b) RF-LARGE only;** the sparse ceiling waits for V-P.
- **Recommendation: (a).** It costs little and it is the only kernel data for §4.8's sparse ceiling.

**Q6. Stage granularity.** The design's stages (`DESIGN.md:827`) are finer than the public API: the audit is inside `prepare`, and solve, residual and rcond are inside `finish`.
- **(a) Public-API boundaries,** with the map of Scope 4 and the bundling disclosed.
- **(b) Timing hooks in FK behind a feature,** a write-set extension into FK.
- **Recommendation: (a).** No kernel edit for an observation slice. If ROOT needs a finer split later, it can come with K6b.

**Q7. Repeats, timeouts and the slot.** P1's product-level dense 1,000-member runs took 204–487 s, and two timed out at 1,800 s (`DETECTION/RETURN.md:254-265`, `:287`).
- **(a)** Five in-process repeats; stop after the first repeat when it exceeds 600 s; P1's timeouts; modes interleaved per model in one pass; a projection to ROOT before the dense 1,000-member runs.
- **(b)** The design's five repeats everywhere, and two passes of opposite mode order. That roughly doubles the quiet-host time, for no claim K6 makes.
- **Recommendation: (a).** K6's claims are memory fits and ratios, not timing comparisons.

**Q8. Where the runner and the packet live.** §4.8 puts kernel-level observations in `H` (`DESIGN.md:833`), and §4.10 puts "the runner and records of §4.8" under `numerical_robustness/observations/` (`:917`), which does not exist yet and is outside K6's write set.
- **(a)** The runner in `H/runner/`; a compact packet with the model hashes and `SHA256SUMS` in `H/observations/k6/` (as DEC-053's packet sits in the product tree); the raw JSONL in `T3/IMPLEMENTATION/K6/_run_records/`. V-K and V-P call the runner by path.
- **(b)** Everything in T3's records; nothing in the product tree but code.
- **Recommendation: (a).** The packet is small and observation-only, and no test pins its timing values. A test may pin its deterministic counts.

**Q9. Where the runner's tests run.** The piping pytest surface runs only `P/tests` (`tools/validation/discover_test_surfaces.py:151-158`). Hosted CI runs only `P/tests/test_ci_*.py` there (`.github/workflows/piping-desktop-e2e.yml:53-54`).
- **(a)** Beside the runner only: K6 and the reviewer run them; no CI or sweep does.
- **(b)** Also a thin `P/tests/test_performance_harness_runner.py`, so every DEC-025 sweep runs them: the macOS path on the Mac, and the `RLIMIT_AS` path on any Linux sweep. It is a declared write-set extension. Checkpoint 0 confirms that the sweep's sandbox allows `ps` and a process-group kill.
- **(c)** A `test_ci_` name, so hosted CI's selection job runs them. That job is the selection policy's; this would misuse it.
- **Recommendation: (b).**

**Q10. Model generation and the section formula.**
- **(a) A Rust family generator in `H`** (as K1's `large()` did, `SD/structural/k1_tests.rs:704-771`), with R1's node order. A one-off standard-library script, recorded, cross-checks all 24 models against `references.py --model`. CI tests stay self-contained.
- **(b) A Python generator importing R1's builders** (P1's method, `DETECTION/RETURN.md:23`), with the small models committed as files. The large models are generated locally and hash-checked. CI cannot read R1's records.
- **Section:** A, I and J from OD and ID with explicit products and `PI`; no `powi` (PP uses `powi`, `PP:8515-8519`).
- **Recommendation: (a),** with the stated section formula.

**Q11. Whose request hashes.** §4.8 commits "the request's sha256" (`DESIGN.md:824`), but K6's inputs are kernel models, not product requests.
- **Recommendation:** K6 commits the kernel-model sha256 of all 33 models and cites P1's RF-LARGE product-request hashes (`PLATFORM_CALIBRATION_MAC/gate/gen_out_sha256.txt:122-145`) for V-P. The DEC-053 nine have no product request; generating them is V-P's.

**Q12. The legacy lanes.** The product still runs the dense LU lane in dense scrutiny (`PP:2508`) and the identity-order sparse lane (`PP:4464`, `:4601`), which F1b guards on the 24-byte estimate.
- **(a) Observe both** as separate modes, at the sizes the ascent rule admits: the identity-order lane's peak against 24 bytes per entry (never CONT at 10,000, about 16.2 GB estimated), and the dense LU lane's bytes per n² entry.
- **(b) Counts only,** with no lane run.
- **Recommendation: (a).** It is the only measurement that can replace the lane guard's provisional basis.

**Q13. One slice, or two.**
- **Recommendation: one slice,** with checkpoint A split into A1 (the Rust side) and A2 (the runner and the dry run). The observation runs (B) wait for ROOT's approval of the schedule.

## Design text made stale or inconsistent by merged slices (for ROOT; `DESIGN.md` stays hash-pinned)

None of these is yet recorded as a ruling.

1. **§4.8's "generated deterministically into a product request"** (`DESIGN.md:824`): K6's inputs are kernel models. The RF-LARGE product requests already exist, hashed by P1 (`gen_out_sha256.txt:122-145`). See Q11.
2. **§4.8's stage list** (`:827`) is finer than the kernel's public API, and omits `geometry` and the formation check, which the product path now runs (`SA:540`, `:644-697`). See Q6.
3. **§4.8's "Linux peak is known"** (`:827`): the only Linux peaks are P1's product-level ones on `c61a540ea`, and T3 now runs on the Mac without a Linux host. See Q3.
4. **§4.8's host protection** does not mention the in-process heap cap that the platform calibration adopted because macOS cannot set `RLIMIT_AS` (`PLATFORM_CALIBRATION_MAC/RECORD.md:63-65`). See Q3.
5. **§4.8's "for W1, limbs per entry"** (`:828`), I12's "K6 turns work units into time" (`I12_K4_IMPLEMENTATION.md:584`), `D5_TRIGGER.md:260` and the 5a.3 candidate's `:574` all need K4, which the K6 row (`:1021`, "after K1") does not wait for. See Q1.
6. **§4.10's home for "the runner and records of §4.8"** (`:917`) conflicts with §4.8's kernel-level home (`:833`). See Q8.
7. **D-8 and C4's "from the K6 and V-P measurements"** (`:1270`; `ROOT_SELECTION_DESIGNS.md:42`) is amended to K6 and V-K (`ROOT_RULINGS_V1.md:1169-1177`). §4.8's ceilings still cite measurement generally (`:804`).
8. **§2.1's "at least about 100 bytes per n² entry"** (`:219`) is superseded for the guard by F1b's count of 96 (`ROOT_RULINGS_V1.md:1273`). P1's measured product peak at 1,000 members is about 105 bytes per entry with the process baseline (Scope 6).
9. **§4.8's "the protected DEC-050/053 legacy LU observation (`PP:2520-2535`)"** (`:803`) has drifted; the call is `PP:2508` on `d1cc97ce4`. The identity-order lane, which §4.8 does not name, is what exhausted the heap cap at CONT 10,000 (`ROOT_RULINGS_V1.md:1453-1454`). See Q12.
10. **The harness's "sparse" observation takes a dense matrix** (`H/src/lib.rs:897`; `SD/lib.rs:651`), so "kernel sparse observations" means a new path, not an extension of the existing one.
11. **§7.2's RF-LARGE expectation** (`:1080`) is now measured at product level: P1 on main (`DETECTION/RETURN.md:254-289`), and F1b's gate, where 8 of the 12 sparse 10,000-member runs complete with named M03 refusals (`ROOT_RULINGS_V1.md:1453`).
12. **The K6 row does not list the DEC-050/053 pytest pins** that read `H/src/lib.rs` (`P/tests/test_sparse_default_promotion_observation.py:18`; `test_sparse_suitability_observation.py:14`), which constrain any edit to that file. See the write set.

**Recommendation:** ROOT records items 1–12 as rulings when it rules on this brief, as it did for K4's, F1b's and K5's lists. `DESIGN.md` is not edited.
