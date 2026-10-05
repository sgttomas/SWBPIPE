# RV89: follow-on review of U4 G6's pre-registration delta

**Reviewer:** RV89, TASK (Type 2), dispatched directly by ROOT. No descendants. This follows RV89's G6 review (`R/REVIEW_RV89/u4_g6_01/`) with the same context, copies and oracles.

**Candidate:**
- `b43378d90a` on `codex/piping-f2a-memory-20261004`, on top of `2bb81ec1ea`: 3 files, +142 / −14. It equals I65's `_run_records_g6r/candidate_g6r.diff` apart from `index` lines.
- `R/I65/u4_g6_01/registration.diff` (sha256 `976b722d…bcbf`, 5 files, 323 lines) is **not applied**. The reviewed G6 version is kept as `registration.g6.diff` (`35c72703…4fd2a`).
- I applied the new diff with `patch -p1` to my own `git archive` copy of `b43378d90a`, never in WT/f2a-memory. It applies cleanly.

**Basis read:**
- the delta in full;
- the new `registration.diff`, and its difference from `registration.g6.diff`;
- I65's RETURN.md Addendum 2;
- `_run_records_g6r/` (per_identity, controls, `mutants_g6.py`).

**Scope.** These are ROOT's items 1–5. RV87 owns the TEXT and audit side (SF-1 to SF-3, N-1, N-2), so I did not re-derive the TAV rows or the audit table. I took TAV_W and TAV_X as the generated constants and checked only what they compose to.

**Oracles:** RV89's own.
- My sweep (71 inputs × 2 modes × 5 routes), run unregistered and registered.
- My probe module in the registered copy, with my test-only solve-attempt counter in lib.rs.
- A stale build (RUSTFLAGS) and the release build.
- My profile evaluation.
- My mutants, R1–R11.
- A new probe in my copy of runner/headless.

I65's tests were only run.

## Verdict: **PASS**. All three of RV89's G6 items are closed

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 3 |

**Is there any reason not to register now? No.** None of the three NOTEs bears on registration.

**The headline:**
- **S-1 is closed.**
  - With the new `registration.diff` applied, **runner/headless is identical to base**: 85 passed, plus the 2 failures base already has (`load_reference`).
  - PP gives 699 passed, 1 failed (the Mac t13) and 11 ignored.
  - **The registered sweep is byte-identical to G6's registered sweep** (`25cce1e1…`). The unregistered sweep is byte-identical to base (`e1677d73…`).
- **S-2 is closed.**
  - `attempted_examples()` now includes K2a's partial-underflow shape, and `g_c_declines_…` asserts that its seed is `FormationFailure`.
  - **R8 is killed** by `g_c_declines_…`, and also by `registered_g_c_…` in the registered copy.
- **S-3 is closed.**
  - `admit` prices its bound only through `admission_bound`, which adds R before comparing with M. The law record carries `required`.
  - **R6 is killed**, and so are my R10 and R11. The edges at M − R − 1, M − R, M − R + 1 and M hold in I65's test and in my probe.
  - In the registered build, `law.required` equals `cap_priced_maximum(mode) + R`. Wherever admission refuses before the bound, it is `None`: in the Stale and release builds, and on a D1 violation.
- **The maximum reproduces:** **0.888054 M sparse and 0.892949 M dense**, which is 48,100,370 B and 28,389,922 B under 0.9 M. The text-error budget is 1.81% dense and 3.06% sparse.
  - From G6, the record's only changes are the regenerated TAV_W and TAV_X, and one atom, `s(ThreadPacketOutput)` 1,688 → 1,704 (`required: Option<u64>`). Every W phase moved by +861,162 B and every X phase by +845,828 B, in both modes.
  - My debug-build record equals I65's test record line for line.
  - My release-build record equals my debug record apart from the identity and two identity-gated `RECORD_SKIP` lines.
- **Unchanged, re-checked:**
  - The pinned successor: `ac6986b0…` / `6cd1d249…`, receipts `efc1a39b…` / `3e26499f…`.
  - `admit`: a milestone permit, Headless refused, and 21 D1 violations refused.
  - The attempt counter agrees 182/182.
  - The deep witness passes at R/16 and R/64.
  - I65's 25 G6 and G6R mutants equal its record one for one, all killed.

## Findings

| ID | Sev | Where | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | `P/core/runner/headless/tests/retained_precision_admission.rs:87–96` once registered (S-1's oracle) | **S-1's oracle makes the runner workspace call the Direct entry, and that call is admitted and publishes.**<br>– The test reads the expected profile from `run_linear_static_preview_value_with_retained_direct(ordinary(), mode)`, on `rf_skew_t_cant_off_122_r1e-04`. That input is inside D1, so in the registered build the call is admitted and runs W1 to a successor.<br>– The runner's lock resolves serde_json 1.0.151. PP's reviewed-input record hashes PP's own lock, which has 1.0.149. This is G6's N-1 lock residual, now exercised by a test.<br>– **It is harmless as observed.** My probe in my copy of the runner (`evidence/g6r/runner_direct_probe.txt`) gives `Registered` and a successor in both modes. Its bytes are sha256 **`1d9ba709…`** (113,733 B, sparse) and **`7c5fe5c5…`** (114,894 B, dense), identical to PP's own registered sweep for the same fixture.<br>– The runner's product source still never names the Direct entry: the guard at :151 scans `../src/lib.rs` only, and that still holds. | None needed to register. If a test-only Direct call outside PP's lock is unwanted, read the profile from an input D1 refuses (for example the two-case variant). The report carries the build's profile on every refusal, so W1 would not run. Otherwise, record the runner test as a known Direct caller under BUILD.md's lock residual |
| N-2 | NOTE | Mutants | **R9 survives, as before.** It reads only the first seed for the attempt fact, which is equivalent inside D1 because D1.4 admits one case and so one seed | None |
| N-3 | NOTE | I65 RETURN.md Addendum 2, "Status" | **The record's status line is out of date.** It says the repair is "uncommitted in WT/f2a-memory". It is now committed as `b43378d90a`, and that commit equals `candidate_g6r.diff` | Note the commit in the addendum or in the RR when registering |

## 1. S-1: the runner/headless flip, applied (ROOT item 1)

**What changed in `registration.diff`** (my diff against `registration.g6.diff`):
- **A new hunk for `runner/headless/tests/retained_precision_admission.rs`.**
  - The expected profile is the profile that the Direct entry in the same build reports for the same input.
  - It must be `Registered` or `Stale`, and is never `Missing`.
  - The Headless assertions are unchanged: refused, output equal to the ordinary run, and the `CallerCompletion` term.
- **In `admit_grants_…`:** `report.law().required == Some(required)` when admitted, and `None` when refused before the bound.
- **Elsewhere:** hunk offsets moved for the delta's added lines. No other content changed.

**Results in my copies:**

| Copy | PP | runner/headless | Sweep |
|---|---|---|---|
| registered (reg4) | 699 / 1 (t13) / 11. Equal to G6's registered PP plus the new `admission_bound_adds_r_…` | **identical to base**: 85 ok, plus base's 2 `load_reference` failures | byte-identical to G6's registered sweep (`25cce1e1…`) |
| unregistered (cand4) | 697 / 1 (t13) / 11 | identical to base | byte-identical to base (`e1677d73…`) |

The registered and unregistered PP outcomes differ only by registration's renamed or added tests: `admit_grants_…`, `registered_g_c_…` and `the_registered_profile_…`, in place of `no_profile_or_permit_is_constructible`.

## 2. S-2: the deferred-formation arm (ROOT item 2)

- **`attempted_examples()`** (law tests :1201–1213) adds `DEFERRED_FORMATION`, which is `k2a_partial_underflow()` (:1220). It is described as K2a's `PARTIAL_UNDERFLOW`: one 2⁻²⁰ m member, E = 1.3e-292 Pa, G = 1e-200 Pa, a 1e-307 N load, and UY free.
- **`g_c_declines_…`** asserts for that example that there is one seed and that it is `InitialSeed::FormationFailure` (:1281–1286).
- **`registered_g_c_…`** iterates the same examples (registration.diff).

**My probe agrees** (`evidence/g6r/probe_stale_witness.txt`, `RV89_G6_K2A`). In both modes my K2a partial-underflow reconstruction is inside D1. My counter reads 1 attempt, the seed is `FormationFailure`, the fact is true, and the envelope is `MECHANICS_SOLVED`. Under registration W1 falls back at `Candidate`.

**R8 is killed.** It stops counting `FormationFailure` as attempted. In my registered mutant copy, `g_c_declines_…` and `registered_g_c_…` both fail. I65's G6R version of R8 is killed by `g_c_declines_…` in the unregistered copy.

## 3. S-3: R is added at admission (ROOT item 3)

**The code** (line numbers at `b43378d90a`):
- `retained_memory.rs:2300–2302`: `admission_bound(maximum, threshold)` = `bound_admits(maximum?, RESERVED_STACK_BYTES as u64, threshold)`. `bound_admits` computes `maximum.checked_add(R)` and then compares the result with the threshold (:2288–2295).
- **R is the constant.** It never reads `RESERVED_STACK_OVERRIDE`, which sizes only the witness thread.
- `bound_required` (:2305–2310) gives `Some(required)` for `Ok` or `Exceeds`, and `None` for `Unpriced` or `Overflow`.
- `AdmissionLaw.required` (:2327) is set in `admit` (:2923–2929). The closure is `FnOnce`, which `law_order` calls at most once, after the caller, build and domain clauses pass (:2845, :2850). So `required` is exactly the one bound evaluated.
- **`admission_bound` is the only path.** No other code calls `bound_admits`, and only `admit` calls `admission_bound`.

**The tests:**
- **The edges.** I65's `admission_bound_adds_r_before_comparing_with_m` (law tests :658–691) asserts:
  - M − R − 1 → `Ok(M − 1)`;
  - M − R → `Ok(M)`;
  - M − R + 1 → `Exceeds{M + 1, M}`;
  - M → `Exceeds{M + R, M}`;
  - `Unpriced` passes through, and the near-`u64::MAX` case gives `Overflow`;
  - `bound_required` on each;
  - the override is ignored;
  - each mode's maximum + R ≤ 0.9 M;
  - source pins on `admit`'s body;
  - `required` is `None` in a build that refuses before the bound.
- **My probe** (`rv89g6r_required_is_maximum_plus_r`) asserts the same three edges with my own expectations. In the registered build it also asserts `required == cap_priced_maximum(mode) + R` for the milestone: **3,575,778,286 B sparse and 3,595,488,734 B dense** (`RV89_G6R_REQUIRED`). A 0.3.0-schema request gives `None`.
- **In my Stale (RUSTFLAGS) and release builds**, the milestone is refused with `Profile(Stale)` and `required == None`, and the edge test passes on its Stale branch. `admit_grants_…` was also run in the RUSTFLAGS build, and passes on its Stale branch.

**Mutants.** R6 is RV89's G6 survivor, now at its new home (`bound_admits(maximum?, 0, threshold)`). Each of the following is killed:

| Mutant | Killed by |
|---|---|
| R6: the bound drops R | `admission_bound_adds_r_…` and `admit_grants_…` |
| R10: the bound ignores M (`u64::MAX`) | `admission_bound_adds_r_…` |
| R11: `required` records the maximum without R | `admission_bound_adds_r_…` and `admit_grants_…` |
| I65's G6R S-3 mutants (5): R dropped; `admit` bypasses the named bound; the override read; `required` not recorded; an exceeded bound records nothing | `admission_bound_adds_r_…` |

## 4. The code delta from RV87's SF-1 and SF-2 (ROOT item 4)

**What moved** (my diff of the delta):
- **The generated constants:**
  - `TAV_W` 1,569,180,716 → 1,570,041,862 (+861,146);
  - `TAV_X` 1,439,555,190 → 1,440,401,002 (+845,812);
  - the `TEXT_TAV_TEXT_*` record constants;
  - the chain-assumed `PYTHON_CHECK` pair, now 3,437,874,885 / 3,457,585,333.
- **One atom:** `s(ThreadPacketOutput)` 1,688 → 1,704. That is the 16 B of `Option<u64>` in the admission report inside `RetainedPreviewOutput`.
- **The challenge pins** (`tests/retained_memory_challenge.rs`):
  - `W1_PHASE_BYTES` [1,856,156,348, 1,875,866,796] is W1 requested + moving, in both modes;
  - `MAX_PHASE_BYTES` [3,508,669,422, 3,528,379,870] is E_mov,max.
  - Both equal my evaluation.
- **No other atom or form changed.**

**Reproduced** with my composition (`evidence/g6r/profile_check_g6r.out`):
- All 47 forms and all 244 atoms transcribe with 0 mismatches.
- All 7 phases equal the record in both modes.

| Mode | Largest phase | E_mov,max | + R | Fraction of M | Under 0.9 M |
|---|---|---|---|---|---|
| Sparse | W3 | 3,508,669,422 | 3,575,778,286 | **0.888054** | 48,100,370 |
| Dense | W3 | 3,528,379,870 | 3,595,488,734 | **0.892949** | 28,389,922 |

- **The phase deltas from G6:** +861,162 B on every W phase and +845,828 B on every X phase, which is TAV + 16 B. These are I65's figures.
- **The text-error budget** (margin / TAV_W): 1.808% dense, 3.064% sparse.
- **ESTIMATES = 0**, so every atom is InBuild, SourceUpper or Text.

**The identity-gated pinned record:**
- `profile_in_build_record` and `challenge_bounds_are_the_profile` pass in my registered debug build.
- My printed record (262 lines) equals I65's `per_identity/profile_record.test.txt` for every line those tests print.
- In my **release** build, the same tests pass. They print `I65_G6_RECORD_SKIP` twice ("the compiled identity is not the pinned record's", and the challenge bounds pinned to the debug identity), and every other line equals the debug record.
- So the pin binds the one registered identity, and other identities are skipped rather than failed (`evidence/g6r/record_{debug,release}.txt`).

## 5. Anything against registering now (ROOT item 5)

**Nothing.** For the record:
- **The registration package is complete.** No test turns red under registration, in PP or in runner/headless. The only PP failure is the Mac t13, which base has too.
- **Stale is still enforced.** `RUSTFLAGS="--cfg rv89_stale"` and `--release` each compile a different identity, give `build_status() == Err(Stale)`, refuse at D1.1, and leave `required` as `None`.
- **The publication and refusal behaviour from G6 is unchanged** in my probe:
  - the milestone publishes U1's pinned successor through the facade, and `into_publication()` is that successor;
  - Headless and 21 D1 violations refuse, with the ordinary bytes;
  - G-C's fact agrees with my counter in 182/182 runs (78 in D1, 0 mismatches);
  - the deep-input witness publishes at R/16 and R/64.
- **The margin is thinner by 0.86 MB:** 28.39 MB, a 1.81% text budget. This is inside the ruled standard (≤ 0.9 M after the audit, budget stated).
- **N-1** is a test-only exercise of an accepted residual, with byte-identical output. N-3 is record hygiene.

## Mutants

All were run with `cargo test --lib retained_memory`, one job at a time.

| Set | Copy | Run | Killed | Survived | Compile-only |
|---|---|---|---|---|---|
| I65 G6R (6: 5 on S-3, R8 at S-2) | mutU4, restored from my unregistered `b43378d90a` before each | 6 | 6 | 0 | 0 |
| I65 G6 (19: FK exports, profile bindings, Q10, G-C fact) | same | 19 | 19 | 0 | 0 |
| **RV89 R1–R11** (`evidence/g6r/mutants_rv89_g6r.py`) | mutR4, restored from a registered pristine copy | 11 | 10 | R9 (equivalent in D1) | 0 |

I65's 25 equal its record (`controls/mutants_g6r.out.jsonl`) mutant for mutant.

## Execution record

- **Who.** RV89, TASK (Type 2) under ROOT. No descendants.
- **When.** 2026-10-04, about 08:30–08:50 MDT, within the 1.5-hour box.
- **Memory guard.** `memguard.sh` PID 5387 was running, and every cargo job and mutant checked it.
- **Cargo.** The default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (`--test-threads=1` for the probes and records), and `TMPDIR` in scratch. One cargo job at a time.
- **Copies.**
  - WT/rv89/cand4 (unregistered) and reg4 (registered, `patch -p1`), as full archives of `b43378d90a`.
  - mutU4, pristR4 and mutR4, as subset archives (pristR4 and mutR4 patched).
  - RV89's attempt counter, probe module and runner probe exist only in reg4.
- **Not run.** No Git writes or index operations (Git reads used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, and no native, solver-at-scale or DEC-025 jobs. Nothing was written to the system temp directory.
- **Writes.** Only R/REVIEW_RV89/u4_g6_02/, WT/rv89/, WT/targets/rv89/ and WT/scratch/rv89_u4_g5/. Machine paths in the evidence are replaced by `WT`. My copies and targets are deleted after this report.
- **Evidence** (`evidence/`):
  - `g6r/`: the probe module and its output, with the stale-build and deep-witness output; the debug and release records and the release run; the runner probe and its output; my profile check; both mutant outputs and my mutant script; the run scripts.
  - `outcomes/`: the registered and unregistered PP and runner/headless outcomes.
  - The sweeps are not copied again. They are byte-identical to `u4_g6_01/evidence/g6/sweep_{registered,unregistered}.tsv`, sha256 `25cce1e14090048263ef285cb1a571c41ccc34955e57de0fb388df598940ccbb` and `e1677d736f60e6a1b63e8eccc2e2fc9b0265187b507d6767e266a62eed6d0a3d`.
