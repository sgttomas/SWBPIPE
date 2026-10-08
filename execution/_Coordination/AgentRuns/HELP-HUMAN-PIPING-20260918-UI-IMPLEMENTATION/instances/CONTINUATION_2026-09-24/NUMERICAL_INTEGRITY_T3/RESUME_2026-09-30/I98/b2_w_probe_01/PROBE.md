# I98 B2-W: the combination witnesses' one-case proxies (a probe; records only)

TASK (Type 2), I98, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. I am a fresh instance. 2026-10-07 UTC.

**Brief (verified before work):** `R/BRIEFS/B2W_B3W_PROBES.md`, sha256 `b7e2fdc726f86075b01121b8ec99cc4cc94c2836a6ec1a8b2835de8b0686daea`, its "B2-W" section and its host rules. I read NUM's root `AGENTS.md` (byte-equal to the Root instructions in my context) and `agents/AGENT_TASK.md` first. **Mid-task addition from ROOT:** RV115's NB-2 (`R/REVIEW_RV115/b2_kd_01/ADDENDUM_02.md`; RR "RV115 (RV-K) accepts DEF-C's numerical content"): a W1a confirmation of R-7's row count. It is §5.

**Basis read** (sha256, first and last 8 hex):
- the method: I81's `R/I81/b1_probe_01/PROBE.md` (`3e32726d…a8a7392c`), brief `R/BRIEFS/B1_0_PROBE.md` (`8502f267…ec88d646`), its `zz_i81_probe.rs` and `probe_run2.log` (`82c55cf3…e37a657a`); I86's `R/I86/b1_w_probe_01/PROBE.md` (`7470a726…cf3e6c9c`), brief `R/BRIEFS/B1_SW.md` (`4559dfb9…95042f24`), its `instrumentation.diff`, `zz_i86_probe.rs`, `sanitize.py`, `compare_runs.py`, `compare_i81.py`, inputs and `round2_run1.log` (`5c0a77c6…b3e9267b`);
- the plan: I93's `PLAN.md` (`e1147dbd…f8a1238a`) §1.2.1–§1.2.6 and §6, and `REVISION_01.md` (`63abb73f…5e8bba0c`) §3 (N-7) and §5 (the B2-W outline); RV114's `REVIEW.md` (`bc6918ce…0a2a9ee6`) N-7; RR "B2/B3 R1: …", "I93's REVISION_01 accepted; …", "I81's B1-0 probe verified; …", "I86's SW probe accepted; …" and "B2-W and B3-W dispatched as I98 and I99; …";
- the designs: B2-C `R/I97/b2_c_01/CONTRACT.md` (`165cd4b1…a81d9d28`) §0, §2.3–§2.5, §4 and §10.2 (C-1, C-3, C-9); C2 `R/I32/f2a_wire_c2/CONTRACT_DELTA.md` (`923da0b9…7c890869`), its `nodal_terms` row (line 104). B3-D (`R/I96/b3_d_01/DESIGN.md`) bears on B3-W only; I did not need it.
- code, cited by symbol, in the archive of main.

**Placeholders:** `WT`, `NUM`, `P`, `T`, `R`, `RR` and `VENV` as in the dispatch. `PP` = `P/core/product_physics/src`; `RE` = `P/core/reporting/result_export/src`; `ARCH` = the disposable archive `WT/scratch/i98_b2w/arch/`; `S` = `WT/scratch/i98_b2w/` (my scratch). In the logs, `HOME` replaces the home directory and `VENV_ROOT` the virtual environment's folder. M = 0.0048/0.0096/0.0096 N·m (the milestone's three moments).

**Limits kept.** Records only; no maintained file changed. No Git writes (reads used `GIT_OPTIONAL_LOCKS=0`). No DEC-025, no evidence sweep, no installs. Every cargo job went through `WT/tools/t3_cargo.sh` (memory guard up), `--locked --offline`, `CARGO_BUILD_JOBS=4`, with `RUSTFLAGS` and `CARGO_ENCODED_RUSTFLAGS` unset: 18 jobs, all rc 0 (`_run_records/cargo_jobs_i98.log`). The eight Python reader runs ran under `/usr/bin/lockf -k WT/guard/cargo_job.lock` directly (§2.4); the first four failed in setup, as §10 item 7 states. Before each job I waited while more than one `/usr/bin/lockf` process was running; I99's jobs interleaved with mine, and no B1 job was queued while I ran. Every wait ended with its job. I stopped no process. `TMPDIR` was `S/tmp`; nothing went to the system temp directory.

## 0. Outcomes in brief

| Item | Outcome (both modes in every row) |
|---|---|
| 1. W-CB1's proxy (`A + B`, 256 loads, unnetted) | **The over-cap facts:** G-A refuses `Cap { Loads, 256, 128 }` (D1.9), so Direct publishes the plain bytes with no W1; G-B, with G-A bypassed, refuses `CaseLoads 256/128`. **With G-A and G-B bypassed** (probe-only, R = 64 MiB) **and on the witness twin** (4 MiB): G-C passes, **native `Selected`** (p128, verified at p256), **the candidate certificate passes**, staging and the serializer pass, and **precommit refuses at G5 `ATTEMPT_MISMATCH`. Furthest phase: W4.** The cause is D6b, a *case* rule: a `selected` case must not be ordinarily `checks_passed`, and the proxy is `checks_passed` because **A + B nets to exactly zero on all 21 loaded DOFs**. With the loads authored in canonical order and D6b neutralised in memory, the Python reader **passes** the proxy's successor. **Prediction: W-CB1 is `retained_selected`, but every combination row is exactly 0** (the ordinary route's 2,080 combination rows are all 0). Informational: `A + C` falls back at Candidate (`SharperExact`); **`1·A + 0.5·B` publishes a successor** (Rust reader PASS) |
| 2. W-CB3's candidates | **Variant 1 of at most 6 qualifies; the stop rule did not fire.** The L = 0 base with B = a 1 N `global_y` force on the fully restrained isolated node N2. **B alone is `checks_passed`** (report Passed, W2 not triggered, legacy `not_required`), so it is `not_required`. **The proxy A + B is `sensitive` (K-D5's own milestone row), native `Selected`, and publishes a successor** that the Rust and Python readers PASS. Its rows equal A-alone's except N2's two reaction rows |
| 3. W-CB2's prediction | **Case C at main reproduces I81 exactly:** `sensitive`, W2 published b = 518, W1 runs, native `Unresolved(Ceiling)` on the same four-attempt ladder, `Fallback(Native)`, one notice. It still predicts W-CB2's `retained_unavailable` with `combination_unresolved` (phase `kernel`) |
| 4. Controls | **Two rounds identical:** 410 probe lines and 22 output files byte-identical. **Reproduction:** I81's case C, L = 0 and milestone and I86's case A: 108 fields plus 8 seed and native lines, 0 differences |
| 5. R-7 (NB-2) | **Confirmed on four W1a requests,** in both modes: combination rows = 7n + 50m + 8g with n, m and g from distinct entity references. W-CB2's base gives 168 rows (4, 2, 5); W-CB3's base 111 (3, 1, 5); W-CB1's base and its 0.5 variant 2,080 each (32, 32, 32). The families are 6n / n / 30m / 20m / 6g / 2g |
| Found on the way | **A latent producer/reader disagreement on main** (§7): an in-domain one-case input whose loads are not authored in canonical nodal-term order falls back at precommit G8 `PREPARATION_MISMATCH`. The milestone with its three moments reversed does so on Direct, with one notice. The same lines are on all four B1 branch heads |

## 1. Setup and build identity

- **Archive:** `GIT_OPTIONAL_LOCKS=0 git -C NUM archive 0b6c5d7362ff21104e078d8168f62e3f38928cf9 | tar -x -C ARCH/`. Main was `0b6c5d7362` throughout (origin read at the end). Before editing, PP `lib.rs`, `retained_product.rs` and `retained_memory.rs` hashed `4c33c250…`, `f536bfe7…` and `fbc7c9db…`: I81's and I86's pre-edit hashes. From I81's `d8c88774d0` and I86's `47a3bdfcf5` to main, nothing changed under PP, the solver, RE `src`, `P/core/model` or `P/fixtures/product_preview` (`git diff --stat` empty), and no `Cargo.lock` or `Cargo.toml`.
- **Target:** `WT/targets/i98-b2w/`. One build. All 14 test runs used the same binary, `open_pipe_stress_product_physics-e08917abb99aa3a9` (`_run_records/test_binary.sha256`). Two small release tools for the Python reader went to `WT/targets/i98-b2w-json/` and `WT/targets/i98-b2w-units/` (§2.4).
- **Registered build, observed:** every admission report reads `profile=Registered`, and the probe's `registered()` is `true`.
- **Probe-only instrumentation** (`_run_records/instrumentation.diff`; archive only; all `cfg(test)`; no control-flow change): **I86's diff, renamed `I86` → `I98`.** Its `+`/`-` lines equal I86's after the rename. The prints are the seeds, the W1 start, preparation failures, candidate refusals, precommit errors and dumps, the native outcome, and phase marks.
- **The probe module** is `_run_records/zz_i98_probe.rs`, declared in `retained_memory.rs` as I81's and I86's were. It is I86's module renamed, with three additions:
  - **`bypass`, W-CB1's probe-only path** (RV114 N-7). It is lib.rs's `permitted_dispatch` and `permitted_run` with G-A and G-B bypassed:
    1. G-A's census and refusal are recorded;
    2. G-B's refusal is recorded from one observed ordinary run whose observer holds a probe-built permit (`CapturePermit { _profile: &REGISTERED_PROFILES[0] }`, reachable because the module is a child of `retained_memory`);
    3. the permitted run proper runs on the reserved stack R = 64 MiB, with an observer that holds no permit, so G-B is not consulted;
    4. G-C (`check_complete`) is evaluated with a probe-built permit and recorded;
    5. `retained_w1` runs whatever G-C says.
  - `I98_DUMP_PLAIN`, which writes the plain envelope for R-7's count.
  - `zz_i98_dump_builtin`, which writes U8's committed helpers (copied verbatim, as I81 copied them) and I81's case C to files. Their Value sha256s equal I81's: milestone `8d1967a7…`, two-body A `ec6c8e65…`, two-body B `cf688351…`, L = 0 `5e562856…`, case C `3649b4dc…`.
- **Paths per input:** the census, the plain value route, one counted Direct invocation, and the witness twin (the S1 witnesses' private driver at R/16 = 4 MiB, which skips G-A, G-B and G-C). The bypass ran on W-CB1's proxies and the cause check.
- **Inputs** are written by `_run_records/gen_inputs.py` (VENV) from the builtin dumps and from I86's accepted component files (`R/I86/b1_w_probe_01/_run_records/inputs/i3_c1_case_{a,b,c}.json`, sha256 `39c44bc3…`, `544330c8…`, `b6e8138f…`, equal to RR's). Every input and its sha256 is in §8 and `_run_records/inputs/`.

## 2. Item 1: W-CB1's proxy

### 2.1 The input

`cb1_ab` (file `30065a8d…341b`, Value `a50edfb7…f3ec`) is SW's case A request, which is c1's model with every provenance escaped and a raw value of depth 16. It carries one case, `case:ab`, holding **A's 128 loads, then B's 128, unnetted**: 2l = 256 nodal moments with ids `a:…` and `b:…`.

**A + B cancels exactly.** A's net on every copy DOF is +M, and B's is −M (I86 §5). Each part is an integer multiple of a common 2^E. So **all 21 loaded DOFs net exactly 0**: I checked this in exact rationals, and the assembly reports load perturbation 0.0.

### 2.2 Outcomes

| | sparse_interactive | dense_scrutiny |
|---|---|---|
| **Census (the over-cap fact)** | D1.9 `Loads` 256/128 and `LoadsCapacity` 256/128: domain and refusal `Cap { Loads, 256, 128 }`. Every other row is SW's A: Nodes, Members and Supports 32/32, materials 4/4, temperature points 16/16, typed and raw text 128/128, raw depth 16/16 | the same (the census does not read the mode) |
| Ordinary | solved; 2,113 results; **`checks_passed`**; rcond 2.038e-8; no D-5 line; amplification 2.7e-9, load perturbation 0.0; plain `4898d5d6…` | solved; 2,114 results; **`checks_passed`**; rcond 2.038e-8; plain `1687ff12…` |
| Seed | `report/checks_passed`; W2 `not_triggered`; legacy `not_required`; no load-row finding; `recovery_demoted` false | identical |
| **Direct** | G-A refuses; no permit, **no W1**; the published bytes are the plain bytes; 0 notices; counts `{runs 1, complete_gates 0}` | the same |
| **G-B, with G-A bypassed** | `PhaseRefusal { Late, CaseLoads, 256, 128 }`; the late capture is skipped | the same |
| **Bypass (G-A and G-B bypassed; R = 64 MiB)** | **G-C `Ok`**. W1 runs: **native `Selected`** (p128 accepted, verification completed at p256; Run work 19,685,082); **the candidate certificate passes** (all ten product stages `completed`, result `ready`); staging and the serializer pass; **precommit: G5 `RETAINED_PRECISION_ATTEMPT_MISMATCH`**. Furthest phase **W4** | identical |
| **Witness twin (4 MiB)** | `Fallback(Precommit { G5, ATTEMPT_MISMATCH })`; W4 | identical |

**The furthest phase reached is W4 (precommit), on both drivers in both modes.**

### 2.3 The precommit refusal's cause

- The Rust reader's `g5_ordinary` has check **D6b**: a `selected` case's ordinary quality must be `sensitive`, `unresolved` or `failed`. The proxy's case is `selected` with ordinary quality `checks_passed`.
- The Python reader on the same dumps fails at its own D6b (`_g5_ordinary`'s `fail(quality[i]["solve_quality"] in (…))`), with the same code (`_run_records/logs/py_reader_d6b.log`).
- **With D6b neutralised in memory** (the reader text otherwise unchanged; nothing written; §2.4), its next failure is **G8 `PREPARATION_MISMATCH` at `nodal_terms`**. That is §7's disagreement, reached because the proxy authors A's loads before B's, which is not canonical order.
- **`cb1_ab_canonical`** (the same 256 loads in canonical nodal-term order; Value `38b6ec5d…ecd4`) behaves identically up to precommit (G5, D6b) on both drivers in both modes. **With D6b neutralised, the Python reader PASSes its successor:** invocation-bound, numerically eligible, 2,113 / 2,114 classifications (`_run_records/logs/py_reader_d6b_extra.log`).

**So every reader gate except D6b passes on the cancelling cap-maximal proxy.** D6b concerns ordinary attempts, which B2-C gives a combination none of:
- §0 item 6 and §4: a combination has no ordinary attempt, and `CombinationAttempt` is its own closed `$def`;
- §2.1: under T-4, combinations are never in A.

The proxy's one-case shape is therefore the cause of the refusal. Under PLAN §6 item 8, if the committed W-CB1 publishes, its difference from this proxy is D6b's case scope.

### 2.4 How the Python reader was run (probe-only)

- `_run_records/py_reader_d6b.py` (VENV) loads the archive's `P/core/analysis_runs/retained_precision.py` (sha256 `9d1156ed…`) as a member of its own package. It runs it unchanged, then with exactly one line replaced in memory: D6b's `fail(…)` becomes `fail(True)`.
- It wraps `_need` to print the reader line of a failing check.
- The reader needs the checked-JSON and units authorities. I built them under the lock with `t3_cargo.sh build --locked --offline --release` (crates `canonical_json` with `checked-cli`, and `units` with `cli`) into `WT/targets/i98-b2w-json/` and `…-units/`, and passed them by `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN`.
- **Control:** the unchanged reader PASSes all eight published successors of this probe (milestone, L = 0, W-CB3's proxy and SW's A, both modes; `py_reader_successors.log`), as the Rust reader does.

### 2.5 Would the proxy select?

**The kernel selects, and the certificate passes at the caps with 2l = 256.** The only refusal is a case-only rule. **Prediction: W-CB1 (A + B) is `retained_selected`.**

**But it is degenerate.** Its combined ledger cancels to zero, so its native solution is exactly zero:
- the proxy's published rows are all 0, apart from the solver-mode record;
- the ordinary route's 2,080 rows for combination A + B on W-CB1's base (`r7_cb1`) are all 0 (§5).

A zero witness exercises NB-3's "cancelled nets stay data", but it may not exercise the certificate's predicates, or S1, the challenge and RSS, on a non-trivial solution.

### 2.6 Informational variants (beyond the brief's letter; two, for ROOT's choice of W-CB1)

| Input (Value sha256) | Nets per copy | Verdict | Native | W1 (bypass at R and twin at 4 MiB, both modes) | Implied combination |
|---|---|---|---|---|---|
| `cb1_ac_canonical`, SW's A + C (`08e6f809…6179`) | (1 + 2^(k−3))·M | `sensitive` (K-D5 at `C3:N1:RX` sparse, `C1:N1:RX` dense) | `Selected` | **`Fallback(Candidate)`**: `Predicate { row 6 (sparse) / 7 (dense), SharperExact }`; W2 | `retained_unavailable`, `facade_certificate` |
| **`cb1_a_halfb_canonical`, 1·A + 0.5·B** (`9f789113…3cba`; B's terms times 0.5, exact) | +M/2 | `sensitive` (K-D5 at `C0:N1:RX`, ratio 2.4279 / 4.8526) | `Selected` | **`Successor`**: the Rust reader PASSes (invocation-bound, eligible, 2,113 / 2,114); successor `cff7df87…` / `ed7b1fa3…`, receipt `601e118f…` / `2828996e…`; twin `Successor` | **`retained_selected`, non-degenerate** |

G-A and G-B refuse both inputs, as for `cb1_ab`, and G-C passes. **B + C would cancel at copy 3** (2^0·M − M), so I did not try it.

## 3. Item 2: W-CB3's candidates

**The construction** (variant v1, the brief's example) is U8's L = 0 base: the milestone plus node N2 at (3, 0, 0), referenced by no member, with one six-DOF rigid support `rigid:N2`.
- **A** = the milestone's three moments (`case`).
- **B** = one `concentrated_force`, `global_y`, 1.0 N on N2 (`case:b`).
- **The proxy** = A's loads then B's (`case:ab`), which is canonical order.

| Input (file; Value) | Mode | Ordinary verdict and why | Seed | Direct (W1) | Twin | T-4 class |
|---|---|---|---|---|---|---|
| `cb3_a`, A alone (= U8's L = 0; `b4230910…`; `5e562856…`) | sparse / dense | `sensitive` by K-D5 (`N1:RX`, 2.4279 / 4.8526); rcond 2.606e-8 | report/sensitive; W2 not triggered | **`Ok(successor)`**; Rust PASS; receipt `c00cbe76…` / `0b4250c8…` (= I81's) | `Successor` | A |
| **`cb3_v1_b`, B alone** (`f09d0664…`; `295a3287…`) | sparse / dense | **`checks_passed`**: report Passed, rcond 2.606e-8, no D-5 line, assembly perturbations 0.0 | **report/checks_passed; W2 not triggered; legacy `not_required`** | Today: native `Selected`, then Candidate refuses `Native(MissingUniquenessWarrant(0))`; one notice; bytes = `with_notice(plain, "case:b", None)` | `Fallback(Candidate)` | **`not_required`** (and the reader's `not_required` rule holds: quality, initial and W2 as required) |
| **`cb3_v1_ab`, the proxy** (`63fad09d…`; `e588aba3…`) | sparse / dense | **`sensitive` by K-D5**, the same row and ratio as A | report/sensitive; W2 not triggered | **`Ok(successor)`**: native `Selected` (p128, verified at p256; Run work 2,503,695 against A's 2,392,403); certificate, staging, serializer and precommit pass; **Rust PASS** (invocation-bound, eligible, 113 / 114); case `selected`; successor `80abadc6…` / `ebae7245…`, receipt `7fab5632…` / `d24b74a4…`; **the Python reader PASSes** too | `Successor` | A |

**B does not change A's numerics** (`_run_records/cb3_rows.log`). The proxy's published rows equal A-alone's, keyed by kind, entity, location and component, except:
- N2's two reaction rows: `Fy` 0 → −1.0 N, and the force magnitude 0 → 1.0 N;
- the solver-mode record, whose location is the case id.

Every row carries the method token.

**What this establishes for W-CB3** (`retained_selected` with a prepared operand):
- **B is a `not_required` operand.** Under B2 it gets one `OperandPreparation`, CasePrep only. Its prepared source builds on main, because B's own W1 got past preparation to native.
- **The combination's ledger (A's terms, then B's) selects and certifies as a one-case proxy.**
- What remains B2's: the combination runs on A's group with B prepared (REVISION_01 S-2), and DEF-C's certificate replaces the case certificate. **The stop rule did not fire; variants v2–v6 were not needed.** v2 (a moment on N2) is defined in `gen_inputs.py`, but was not run.

## 4. Item 3: W-CB2's prediction

**Case C** (`cb2_case_c`, a byte copy of the builtin dump, Value `3649b4dc…`; I81's construction: two-body A's three moments, then B's tip force and torque) **re-run at main equals I81's lines field for field** in both modes (§6):
- verdict `sensitive`; the seed is `structural_failure/range`, then W2 published with b = 518 (trigger `Evaluation(Range)`), legacy `unavailable(source closure)`, D-5 reference set;
- W1 runs;
- **native `Unresolved`, reason `Ceiling`:** (128, Candidate, `StopRule { Displacement(node 3, Uy), body 1, Translation }`), (256, VerificationThenCandidate, `StopRule { EndAction(member 1, I, Ux), body 1, Force }`), (512, VerificationThenCandidate, `Charge { … }`), (1024, Verification, Solved);
- **`Fallback(Native)`**, one notice, bytes = `with_notice(plain, "case", None)`; published `e28f03dc…` / `8a399b23…`, which are I81's. The twin also gives `Fallback(Native)`.

**The prediction stands.** W-CB2's combination A + B has exactly case C's ledger (A's terms then B's, in canonical order) over the same stiffness, and case C's native run ends at Ceiling. So W-CB2 predicts `retained_unavailable`, `combination_unresolved`, phase `kernel`, with R-COMB-1's rows.

**What is not observed:** the combination Run itself (multi-case and combination Calls are not on main), the operand-preparation path for B, and the group choice of S-2 (c). Two-body A's dense half still needs B1's Text B (I81 §2.3).

## 5. R-7's row count on W1a models (NB-2)

The ordinary route only (`zz_i98_plain`), both modes, on four in-domain requests. Each request has two load cases, `case:a` and `case:b`, and one mechanics combination `combination:ab` (A + B, factors 1 and 1, or 1 and 0.5). Ids are disjoint (C-9). Every model is W1a: no components, constant-effort or nonlinear supports, or pressure. On main the census refuses them only at D1.4 (`Family(Invocation, LoadCases)`).

Counted by `_run_records/r7_count.py` (`r7_count.log`): **n, m and g are the distinct node, member and support entity references among the combination's rows,** each compared with the model.

| Base (file; Value) | Model | n, m, g | Combination rows | 7n + 50m + 8g | Families: displacement components / magnitudes / action / stress / support components / magnitudes |
|---|---|---|---|---|---|
| **W-CB2's**, `r7_cb2` (`ce52d528…`; `7f07d08e…`) | two-body: 4 nodes, 2 members, 5 supports | 4, 2, 5 | **168** (sparse and dense) | 168 | 24 / 4 / 60 / 40 / 30 / 10 |
| **W-CB3's**, `r7_cb3_v1` (`76bb9831…`; `05e9ae15…`) | L = 0: 3 nodes (isolated N2 included), 1 member, 5 supports | 3, 1, 5 | **111** | 111 | 18 / 3 / 30 / 20 / 30 / 10 |
| **W-CB1's**, `r7_cb1` (`0c346f49…`; `9c21d45e…`) | c1: 32, 32, 32 | 32, 32, 32 | **2,080** | 2,080 | 192 / 32 / 960 / 640 / 192 / 64 |
| W-CB1's alternative, `r7_cb1_halfb` (`7af8c049…`; `c1b85bd4…`) | c1, with combination 1·A + 0.5·B | 32, 32, 32 | **2,080** | 2,080 | the same |

**The per-row checks pass in all eight runs:**
- each member has 10 rows (6 actions and 4 stresses) at each of `end_i`, `end_j`, `quarter_1`, `midspan` and `quarter_3`;
- each support has 6 component rows and 2 magnitudes;
- row ids, and (kind, entity, location, component) keys, are unique;
- there is no other kind, so no maximum, intensified, mode, parity or modulus row;
- the combination's rows follow the case rows contiguously;
- every model node and every support appears, including the isolated N2 and the spring supports.

**g here counts every support.** Each support has reaction rows in these models, unlike I97's fixture, where g counted only the attributed support.

**The kinds:**
- displacement: `global_nodal_{displacement,rotation}_{x,y,z}` and `displacement_magnitude`;
- action: `element_local_{axial_force, shear_force_y, shear_force_z, torsional_moment, bending_moment_y, bending_moment_z}`;
- stress: `element_local_{axial_normal_stress, bending_normal_stress_y, bending_normal_stress_z, torsional_shear_stress}`;
- supports: `support_reaction_component_v2`, `support_reaction_{force,moment}_magnitude_v2`.

The case rows are 171 (two-body) and 2,113 / 2,114 (c1). L = 0's are 113 / 114. On `r7_cb1` the combination's 2,080 rows are all exactly 0 (§2.5).

## 6. Controls

- **Two rounds, identical** (`_run_records/compare_runs.log`):
  - every set ran twice: main, W-CB1, the cause check and canonical proxy, A + C, A + 0.5·B, R-7 and R-7b;
  - **the probe lines are identical** with timings and phase marks removed: 24, 176, 40, 92, 36, 34 and 8 lines;
  - **all 22 output files are byte-identical**: 8 successors, 6 plain envelopes and 6 precommit dumps in the first comparison, and 2 plain envelopes in the second. Every compared file was rewritten by round 2.
- **Reproduction of I81 and I86** (`_run_records/compare_controls.py`; `compare_controls.log` for round 1, `compare_controls_round2.log` for round 2):
  - **I81:** case C, L = 0 and the milestone, both modes;
  - **I86:** case A, both modes;
  - fields compared: input sha, plain sha and length, verdicts, W1 cause, notices, both byte checks, the successor flag, published sha and length, receipt sha256, the Rust reader's verdict, case status, and I86's twin outcome; also case C's seed, W1-start and native lines in order (8);
  - **109 comparisons, 0 different, in each round.**
- **No rebuild between runs:** all 14 test runs used one test binary.

## 7. Found on the way: the nodal-term ordinal (producer and readers disagree)

**What happens.** An in-domain one-case request whose primitive loads are not authored in the kernel's canonical nodal-term order publishes no successor. It falls back at precommit with **G8 `RETAINED_PRECISION_PREPARATION_MISMATCH`** and one notice.

**The witness: `cause_milestone_reversed`** (Value `f610fa6b…03e0`): the milestone with its three moments authored RZ, RY, RX. It is inside D1 (`domain=None`), `sensitive` by K-D5, native `Selected`, and its certificate, staging and serializer pass. **Direct: `Err(Precommit { G8, PREPARATION_MISMATCH })`, one notice, bytes = `with_notice(plain, "case", None)`, in both modes.** The twin and the bypass agree. The Python reader fails at the same check: `_g8`'s `need(s["nodal_terms"] == terms)`.

**The mechanism** (main, by symbol):
- **The producer:** PP `retained_wire.rs`, the CaseSource `nodal_terms` loop. It writes `constructor_ordinal: i`, the term's position in `source.loads()`, which is canonical order, and `primitive_load_index: t.original`, the authored index.
- **The Rust reader:** RE `retained_precision.rs` G8. It re-derives each term with `constructor_ordinal = primitive_load_index = authored index`, then sorts by (DOF, source id bytes, value bits, ordinal). PY's `_g8` does the same.
- **So the two agree only when authored order is canonical.** Every input published so far was authored that way: the milestone, the U8 inputs, SW's components and `c1`.
- **The contract:** C2's row (`CONTRACT_DELTA.md`:104) reads "Array order is kernel canonical (DOF,UTF-8 source_id,value bits), stable constructor_ordinal breaks indistinguishable duplicates for provenance only." I do not rule on which side departs from it.

**Where else.** The same producer line and RS line are present on `codex/piping-t3-b1-20261007` (`603e238517`) and on the `-p`, `-r` and `-t` heads (`11cc14e3e6`, `b5cb7faaeb`, `7e47e51b5d`). I did not check TS.

**For B2.** C2 says a `CombinationSource` has no combined `nodal_terms`; it reaches its operands' CaseSources. W-CB1 to W-CB3's operand cases are each authored in canonical order, so this does not touch the witnesses. But it does touch any combination over an operand authored out of order, and every one-case proxy that concatenates operands with interleaved DOFs, as `cb1_ab` does.

## 8. Recommended witnesses (inputs in `_run_records/inputs/`)

| Witness | Recommended input | Probe evidence | Prediction |
|---|---|---|---|
| **W-CB1** | **ROOT's choice between:** (a) **as planned:** SW's A (`39c44bc3…`) and B (`544330c8…`) plus combination A + B; base `r7_cb1.json` (file `0c346f49bd4bbe28c6f3c63c4930af6ffad89b78015d46e642c51f39ac42adcb`); and (b) **recommended:** the same cases with **combination 1·A + 0.5·B**; base `r7_cb1_halfb.json` (file `7af8c0495438505c5137c6d474a27ec567e11387b03f5e1b071109cdead5584d`) | (a) proxy `cb1_ab.json` (`30065a8de024a1b578a3f31b6a4172a7140804dd0935fe3645440ecd02e5341b`) and `cb1_ab_canonical.json` (`b6ef31eb63a80499c2301cada1cb9742d210b494e118625b452eb97722594a50`): native Selected, certificate passes, only D6b refuses. (b) proxy `cb1_a_halfb_canonical.json` (`012fec523898bc8323e95d0ff6074e22152cbbcffb03ef8d729c5cb6a6a43a83`): publishes, Rust PASS | (a) `retained_selected`, all rows 0. (b) `retained_selected`, non-degenerate. Both are C_eq = 3 at D1's model caps with l = 128 per case and L = 256. I recommend (b) as the S1, challenge and RSS input, with (a) optionally kept as a cancellation pin (NB-3) |
| **W-CB2** | U8's two-body cases A and B plus A + B; base `r7_cb2.json` (`ce52d528d5bdfe8dcb66e10ba52cd33a4006c0b8430f6ad94d295e445e2aee8b`) | proxy case C, `cb2_case_c.json` (`ce1b76a8f0ddce84bdd86e9bd416b1d61a6a8a675cdd8bd51dfe5bb2dcca9884`; Value `3649b4dc…`): `Unresolved(Ceiling)` | `retained_unavailable`, `combination_unresolved`, phase `kernel` |
| **W-CB3** | The L = 0 base with case A (the milestone's moments) and case B (1 N `global_y` at N2) plus A + B; base `r7_cb3_v1.json` (`76bb983124a7ebc6562c83f5197a5e550160a12ff5d19ee0516a1318b9c9772b`) | B alone `cb3_v1_b.json` (`f09d0664dbed500b6dc45ac5c6651f7aea6073e8ab3390e2ccc716cd2a309fe5`): `not_required`. A alone `cb3_a.json` (`b42309107c13b5c31972a0bf24f61b96fd128f75f91326e5b6db4a25f209a6fa`): selected. Proxy `cb3_v1_ab.json` (`63fad09d4e186f8ac9d0f9aad81c35689686632fb36d5c2cc0cbce0f3754faa9`): publishes; Rust and Python PASS | `retained_selected` with one `OperandPreparation` (B) |

**The other inputs:**
- the cause check `cause_milestone_reversed.json` (`d28e1317…be9a`);
- `cb1_ac_canonical.json` (`fa7f5d3c…9066`);
- the builtin dumps: `builtin_milestone.json` (`dcdc8e65…`), `builtin_two_body_case_a.json` (`e10ae998…`), `builtin_two_body_case_b.json` (`776b81b7…`), `builtin_l0_isolated_node.json` (`b4230910…` = `cb3_a.json`) and `builtin_case_c.json` (`ce1b76a8…` = `cb2_case_c.json`).

SW's own files are cited, not copied: `i3_c1_case_a.json` `39c44bc3…` ran as the I86 control.

## 9. For ROOT

1. **W-CB1 as planned selects only through exact cancellation.** A + B's nets are zero on every DOF, so every combination row is exactly 0.
   - Its proxy reaches native `Selected` and passes the certificate at 2l = 256. It is refused only at precommit, by D6b, a case rule that does not apply to a combination (§2.3).
   - **Recommended: rebase W-CB1 on 1·A + 0.5·B** (§2.6, §8). Its proxy publishes, and its rows are non-zero.
   - A + C is not an alternative: it would be `retained_unavailable` (`facade_certificate`).
   - If you keep A + B, then RSS_TIME and the challenge measure a zero solution.
2. **PLAN §6 item 7's stop rules: neither fired.**
   - W-CB3's variant 1 qualifies.
   - For W-CB1, the proxy's furthest phase is W4 (precommit), with a named, proxy-only cause, and the selection prediction holds. PLAN §6 item 8: if the committed W-CB1 publishes where the proxy did not, the cause is D6b's case scope.
3. **The nodal-term ordinal disagreement (§7)** is a latent false fallback on main, and on B1, for any one-case input authored out of canonical order.
   - It is fail-safe: plain bytes plus one notice. But it denies a successor the producer computed and certified.
   - It needs a ruling on which side follows C2's `nodal_terms` row (producer or readers), and an owner: a fix slice, or B2's readers' G8 work. A 07-series corpus mutation is likely too.
   - B2's combination witnesses avoid it as constructed.
4. **R-7 is confirmed on W1a** (§5), as NB-2 asked: 7n + 50m + 8g on three distinct W1a models in both modes, with n, m and g from entity references.
5. **Recorded, not rulings:**
   - **B alone (W-CB3) falls back at Candidate on main** with `MissingUniquenessWarrant(0)`: a case with zero free-DOF load. Under B1's T-4 it is `NoTriggeredCase`, and under B2 it is prepared only, so neither reaches it. A one-case pin of it would assert that Candidate cause today.
   - **The W-CB1 proxy's Run work** is 19,685,082, against SW's A's 29,992,750. The W-CB3 proxy's is 2,503,695, against L = 0's 2,392,403.
   - **Informational timing** (debug build, probe prints present; not a measurement): the twin took about 15 s on the cancelling proxy, 17 s on 1·A + 0.5·B and 17 s on A alone.
6. **Host notes.**
   - The host accepted writes into NUM at this records folder.
   - One empty heredoc invocation of the host's `python3` produced no output and touched no file. All analysis ran under VENV.
   - I also called an unrelated documents-tool help action by mistake. It returned a menu and changed nothing.
   - While editing this record, one unquoted heredoc of mine made the shell try two backtick fragments of the text. `/usr/bin/lockf` got the lock path with no command, printed its usage and exited, taking no lock and running nothing; a glob matched nothing. No file changed.

## 10. Commands run (exact; `cd` into the stated directory first)

1. `mkdir -p S/{arch,tmp,out,logs,inputs,records} && GIT_OPTIONAL_LOCKS=0 git -C NUM archive 0b6c5d7362ff21104e078d8168f62e3f38928cf9 | tar -x -C S/arch/`
2. The instrumentation (I86's diff renamed, applied with `patch` per file) and the probe module were written into `ARCH`.
3. In `ARCH/P/core/product_physics`, with `env -u RUSTFLAGS -u CARGO_ENCODED_RUSTFLAGS TMPDIR=S/tmp CARGO_BUILD_JOBS=4 CARGO_TARGET_DIR=WT/targets/i98-b2w`:
   - `WT/tools/t3_cargo.sh test --locked --offline --lib --no-run` (rc 0; `logs/build_pp_lib.log`)
   - with `I98_OUT=S/out RUST_TEST_THREADS=1`: `WT/tools/t3_cargo.sh test --locked --offline --lib -- --ignored --nocapture --test-threads=1 --exact retained_memory::zz_i98_probe::zz_i98_dump_builtin` (rc 0)
4. `VENV _run_records/gen_inputs.py S/out R/I86/b1_w_probe_01/_run_records/inputs S/inputs <names>` (§8's inputs)
5. `WT=… _run_records/run_round.sh <1|2> r7 main cb1 extra r7b` (the sets as in the script; `extra` with `I98_EXTRA_NAME` and `I98_EXTRA_BYPASS=1` for the cause check with the canonical proxy, A + C and A + 0.5·B). Each set is one `t3_cargo.sh test … --exact retained_memory::zz_i98_probe::{zz_i98_files|zz_i98_plain}` job (rc 0 each; `logs/round{1,2}_*.log`)
6. The Python reader's tools:
   - in `ARCH/P/core/serialization/canonical_json`: `WT/tools/t3_cargo.sh build --locked --offline --release --features checked-cli --bin openpipestress_jcs_ijson --target-dir WT/targets/i98-b2w-json`;
   - in `ARCH/P/core/units`: `… --features cli --bin openpipestress_units --target-dir WT/targets/i98-b2w-units` (rc 0 each).
7. Under `/usr/bin/lockf -k WT/guard/cargo_job.lock`, with `OPENPIPESTRESS_{CHECKED_JSON,UNITS}_BIN`, `PYTHONPATH=ARCH/P` and `PYTHONDONTWRITEBYTECODE=1`: `VENV _run_records/py_reader_d6b.py ARCH/P/core/analysis_runs/retained_precision.py <dumps or wrapped successors>` (`logs/py_reader_*.log`). The first four attempts failed in setup (the module path, the two authorities' binaries, and the package context for relative imports); their logs were overwritten by the runs recorded.
8. Read-only VENV: `r7_count.py`, `compare_controls.py`, `compare_runs.py`, `cb3_rows.py`, `runs.py`, `summarize.py` (`summary.md`) and `sanitize.py`.

## 11. Records, cleanup and limits

- **`_run_records/`:**
  - **the harness:** `zz_i98_probe.rs`, `instrumentation.diff`, `gen_inputs.py` and `run_round.sh`;
  - **the analysis tools:** `py_reader_d6b.py`, `r7_count.py`, `compare_controls.py`, `compare_runs.py`, `cb3_rows.py`, `runs.py`, `summarize.py` and `sanitize.py`;
  - **`inputs/`:** the inputs run;
  - **`logs/`:** the builds, the builtin dump, both rounds of every set, and the Python reader runs;
  - **the analysis outputs:** `r7_count.log`, `compare_controls.log`, `compare_controls_round2.log`, `compare_runs.log`, `cb3_rows.log`, `runs.log`, `summary.md` (one row per input, mode and path) and `cargo_jobs_i98.log`;
  - **hashes:** `test_binary.sha256`, and `probe_outputs.sha256` for the unsanitized logs, successors, plain envelopes, precommit dumps and pre-edit copies, which stay in `S/` outside NUM.
- **Sanitizing:** machine paths are replaced by `ARCH`, `S`, `WT`, `VENV_ROOT` and `HOME`. In the logs, every line over 4,000 bytes that is not an `I98_` or reader line is cut to 1,000 bytes, with its full length and sha256. Those lines are the producer's committed `cfg(test)` prints `I51_DUAL_LANES` and `I51_FROZEN_*`.
- **Cleanup on return:** the archive `ARCH` and the targets `WT/targets/i98-b2w`, `i98-b2w-json` and `i98-b2w-units` are deleted. The archive is reproducible from `0b6c5d7362` plus `_run_records`.
- **Basis drift:** NUM moved from `187ae16d75` to `a0d4f98a00` during the probe, in records only. Main stayed at `0b6c5d7362`. Nothing under `P/core`, `P/fixtures` or `P/schemas` differs between main and NUM's head.
- **Limits:**
  - **One-case proxies only.** No multi-case invocation or combination Call exists on main (D1.4). The combination outcomes are predictions from proxies, as PLAN §1.2.6 intends.
  - The bypass and the D6b neutralisation are probe-only and in memory. They establish where refusals come from, not product behaviour.
  - **The dev/test build only,** with probe prints present. The runs establish outcomes, not S1 stack margins or measurements.
  - TS's reader was not run.
