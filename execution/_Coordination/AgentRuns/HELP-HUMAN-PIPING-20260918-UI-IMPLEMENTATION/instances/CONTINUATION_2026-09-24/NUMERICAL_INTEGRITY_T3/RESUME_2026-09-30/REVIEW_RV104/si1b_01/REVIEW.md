# RV104: independent complete-diff review of T3-SI1b (the point-path panic repair)

TASK (Type 2) RV104, an independent reviewer dispatched by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. I am a fresh instance, I wrote none of this change, and I delegated nothing. Brief: `R/BRIEFS/RV104_SI1B_REVIEW.md` (sha256 `512f5c84d16f4b95…`, verified before reading).

Placeholders: `WT` is the T3 host root, `NUM = WT/numerics`, `P = projects/chirality-piping`, and `T`, `R`, `RR` and `VENV` are as in the dispatch. No machine paths are recorded here or in `evidence/`.

## Verdict

**PASS.** BLOCKING 0, SHOULD-FIX 0, NOTE 6.

The repair does what it says:
- No panic is reachable from `evaluate` or from the runner's point path on any input I generated (0 panics in 125,133 point evaluations and 45,604 point runs). Static reading agrees: the three `.expect`s are now unreachable by their own invariants.
- Every input that did not panic on main gives byte-identical bytes. Each input that did panic now blocks with the truthful `NonFiniteInput` finding for its site, as the last finding.
- Interval mode is unchanged. That covers 501,588 `evaluate_interval` results, every bounded runner line that did not panic on main, the 94 shared cases, and `test_rule_interval.py` (193 passed on both sides).

The NOTEs are about test adequacy, documentation wording, behaviour that predates this change, and one host-rule observation. None of them changes the verdict.

## Findings

| ID | Severity | Path | Evidence | Remedy |
|---|---|---|---|---|
| N-1 | NOTE | `P/core/rules/expression_evaluator/src/lib.rs` `mod tests`; `rule_check_runner/tests/point_path_non_finite_run.rs` | 3 of RV104's 11 mutants survive the committed tests. My differential kills all three (§4.5). **R2** blocks a non-finite quotient over a dimensionless divisor. **R3** blocks a non-finite derived quotient. Both over-block inputs that did not panic. **R6** lets the ratio block continue with a 0 ratio instead of returning `None`, after which the enclosing expression keeps evaluating. In 445 of my inputs that panicked on main, the ratio finding is then no longer last: 345 gain a spurious `DivisionByZero` (e.g. `(a/b)/(c/d)` with `a/b` overflowing and `d = 0`), and others gain `UnsupportedExpressionForm`, `DimensionMismatch`, `MissingVariable` and so on. | Optional, small. Add to `same_dimension_quotients_that_did_not_panic_are_unchanged` that `Moment/Length` and `Stress/ratio` overflows still carry a non-finite value with no finding (kills R2 and R3). Add a nested-quotient case whose findings must equal `[NON_FINITE_RATIO]` exactly, with its `source_variable_ids` (kills R6). Either now, if a repair round opens, or in T3-SI1c, which touches the same lines. |
| N-2 | NOTE | `lib.rs`, doc comment on `FindingCode::NonFiniteInput` | The comment reads as an exhaustive list: binding, literal, same-dimension ratio, NaN interpolation or step argument. It omits the fifth emitter, interval mode's "interval binding ends must be finite" (`build_interval_overlays`). | Wording only. Add "or an interval binding end (interval mode)", or make the list illustrative. RR kept the comment (I79 item 2), so the wording is ROOT's call. |
| N-3 | NOTE | `P/tests/test_rule_interval.py` (`966113396e`) | The commit is not strictly comments only. It also edits one docstring, `Block`'s ("blocks (or panics)" → "blocks"). The ASTs are equal with docstrings masked, there are 4,201 non-comment tokens on each side, the docstring is the only one that differs, and nothing reads `__doc__`. With "(or panics)" gone, the docstring's unqualified "blocks" overstates slightly: the oracle also raises `Block` for non-ratio non-finite quotients, which Rust carries. The new comment on the divide branch says exactly that. | None required. If touched again: "…blocks here, or the oracle reads it conservatively as a block". |
| N-4 | NOTE (predates this change) | `P/core/rules/rule_check_runner/src/lib.rs` (`run_one_check` binding; the limit block near line 1019) | **Identical on base.** A NaN or ±inf caller value is reported as `MissingRequiredValue` ("required variable has no supplied value"), while its `bound_inputs` entry says `supplied: true` with `value: null`. A NaN or +inf slot limit is reported as `RULE_EVALUATOR_ERROR`, "value-slot limit has missing or unknown unit/dimension metadata". Both block and never pass, but the wording names the wrong cause. Labels `k_demo_24_10_0`, `k_demo_10_10_1` and `k_demo_10_10_4` in the runner dumps. | Route with T3-SI1c, or to a runner-wording unit. Not SI1b's: changing it changes outputs for inputs that do not panic today. |
| N-5 | NOTE (predates this change) | `lib.rs`, the doc comments on `Expression::Logical` and `Select` ("so diagnostics in either operand always surface") | `eval_expression` short-circuits with `?` when an operand blocks, so a later operand's diagnostics do not surface. Probe 17: `and(ratio > 1, missing > 1)` reports only the ratio block. This was already true for every left-operand block on main (e.g. `DivisionByZero`). The repair inherits it. The outcome is unaffected (blocked). | Wording, to be routed: "evaluation does not short-circuit on values; it stops at the first blocking operand". |
| N-6 | NOTE (host) | `P/tests/conftest.py` `pytest_sessionstart`; `R/I79/si1b_01/_run_records/tools/run_py.sh` | Unless `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` are set, the session conftest runs `cargo build --locked --release` itself, outside `WT/tools/t3_cargo.sh` and without `--offline`. I79's base pytest run compiled 25 crates that way (16:54:48Z–16:54:56Z). No lock job overlapped it (`evidence/host/i79_pytest_conftest_cargo.txt`). My runs set both variables to unused paths: `test_rule_interval.py` does not use those binaries, so no cargo ran. | For future briefs that name pytest: set both variables, or pre-build the two binaries under the lock. No effect on this candidate's evidence. |

## 1. Basis read (origins and sha256)

| Record | sha256 |
|---|---|
| `NUM/AGENTS.md` | `f96feb19d297c74e…` |
| `NUM/agents/AGENT_TASK.md` | `1a13a5b00b3ce01f…` |
| `R/BRIEFS/RV104_SI1B_REVIEW.md` | `512f5c84d16f4b95…` |
| `R/BRIEFS/SI1B_POINT_PATH_PANICS.md` (the brief the change answered) | `a3c8954f41e6da93…` |
| `R/I79/si1b_01/RETURN.md`, read after my static review and harness design | `44d82fc89d45cfb5…` |
| `R/I73/s_i1_01/CHECKPOINT_1.md` §5, §6.3 | `b992efcfa216f8da…` |
| `R/REVIEW_RV99/s_i1_01/REVIEW.md` (N-2), `ADDENDUM_02.md` (N-10) | `48cfdbc0fc148299…`, `fc08166e2303ec7b…` |
| `RR`: "I73's checkpoint 1 and I74's plan ruled; …" (ruling 3); "RV103 passes #1103; …" (I79's items 1–5) | file `e8dd42b16da6f68a…` when read (append-only) |

NUM was at `2b1f244faf` at dispatch and at `90f7e3a2f9` when these records were written. ROOT committed in between; I read no NUM content beyond the records above.

## 2. The candidate

- **Branch** `codex/piping-t3-si1b-20261006`, **head `966113396e7e2f5f3497ccc75f0cc11262559585`**. That is the pushed head (remote ref equal), and `WT/s-i1b` is clean. It sits three commits over main `f8ed4f055126cf19315d2d8b06d7d11796786a82`, which is the merge base.
- **`90d2c1ebf8`** changes `expression_evaluator/src/lib.rs` only (+353 −3). The sha256 goes from `fbc42fee05211c45…` to `1e380b2ae43990bb…`.
- **`da0758064e`** adds `rule_check_runner/tests/point_path_non_finite_run.rs` only (+233, `de59c4177135ac10…`).
- **`966113396e`** changes `tests/test_rule_interval.py` only (+5 −3). The sha256 goes from `41f021dfb4f73bc4…` to `46ee5156d657039d…`.
- **The commit messages are accurate**, each carries the agent co-author line, and author and committer are the owner's configured identity.

## 3. Method (my own oracles)

- **Copies.** I made `git archive` copies of `P/core`, `examples`, `fixtures`, `tests` and `schemas` at each revision:
  - `WT/rv104/base` and `WT/rv104/cand`;
  - `WT/rv104/graft`: base code with the candidate's six evaluator tests grafted into `mod tests`. It differs from the candidate only in the five non-test hunks;
  - `WT/rv104/mut`, for mutants.

  Each had its own fresh target, `WT/targets/rv104-{base,cand,graft,mut}`. The copies' file hashes equal the ones above.
- **The lock.** Every cargo command ran through `WT/tools/t3_cargo.sh` with `--offline --locked`. My first job waited behind ROOT's DEC-025 `T6S_d953e12187` (23:28–00:16Z), and I left it alone (`evidence/host/cargo_jobs_rv104.txt`).
- **Builds.** The default toolchain is 1.97.1. Test profile, so integer-overflow checks are on.
- **The harnesses** are written from scratch, not derived from I73's or I79's (`evidence/harness/`). They use the public API only. I placed them in the copies' `rule_check_runner/tests/` and removed them before running the committed suites.
  - `rv104_ee_diff.rs`: point `evaluate`, and `evaluate_interval` with 4 overlay variants (none, b = 0, b = 1, relative), over five families:
    - the 69 committed conformance-corpus cases;
    - quotients: 10 dimension and unit pairs, covering the same-dimension, unit-mismatch, dimensionless-divisor, derived, ambiguous and unrepresentable arms. Each runs a 24 × 24 grid of extreme finite values (±0, ±5e-324, `MIN_POSITIVE`, 1e-308, 1e±154/155, 1e300, 1e308, ±`MAX`, `next_down(MAX)`, `next_up`/`next_down(1)`, …), plus 9 × 9 operand forms over a 6 × 6 grid. The forms are: carried ±inf, `inf − inf`, `0·inf`, underflow, `x − x`, a literal copy. There are also 6,000 nested ratios: `(a/b)/(c/d)`, `a/(b/z)`, `((a/b)/z)/z`, `(a/b)·c/d`, `(a+c)/(b−d)`, `a/(b−c) ≤ 1`;
    - tables: 12 tables (regular, ±`MAX` rows, rise overflow, subnormal spacing, one row, NaN row, inf row, non-monotone, −0 row, empty, stress argument, blank id) × 3 modes × 27 argument values × 9 forms × 4 wraps (bare, `table/s`, `s/table`, `table ≤ 100`), plus unit- and dimension-mismatched NaN arguments;
    - non-finite bindings, literals and overlays: NaN, −NaN, a payload NaN, ±inf, 5e-324, −0 and `MAX`, as bindings, literal limits and table arguments, with required-variable variants and non-finite or inverted interval overlays;
    - 9,000 generated formulas × 5 binding sets over the whole grammar: 8 dimension/unit kinds, extreme leaves, 1-in-40 non-finite bindings, missing and duplicate bindings, status sets, and a wrong grammar version.
  - `rv104_run_diff.rs`: `run_rule_checks` (full JSON), and `run_rule_checks_with_bounds` with b = 0 on every solver input, b = 1, b relative, b = 1e300, a NaN bound and a duplicate bound. The packs are:
    - both committed packs (`examples/rule_packs/invented_demo.yaml`, `fixtures/product_preview/invented_demo_rule_pack.json`) over 29 actual × 29 limit-quantity values, including NaN, ±inf and subnormals, with slot values including NaN, +inf, `MAX` and 5e-324;
    - a catalog-unit pack (Pa, kPa, MPa, psi);
    - 3,500 generated single-check packs × 4 value sets;
    - 1,500 three-check packs × 3 value sets, each check also run alone;
    - a table family: interpolate, step and exact checks over user inputs (point path even inside a bounded run), a bounded boolean check, and a table check over a solver input, with 5 NaN-producing argument forms × 29 × 5 values.
  - `rv104_compare.py`, `rv104_mutants.py`, `rv104_survivor_diff.py` and `rv104_probe.rs` (31 named inputs).

## 4. The review, by item

### 4.1 No reachable panic (item 1)

**Static.**
- **The point path:**
  - `quantities[0]` sits behind the non-empty check.
  - `table.rows[0]` and `rows.len() − 1` sit behind `validate_table` and its early return.
  - `windows(2)` indexing is safe.
  - The step `.expect` is unreachable: x is not NaN and lies in [first, last], and `rows[0].argument = first ≤ x`.
  - The interpolation `.expect` is unreachable: with no row equal to x, first < x < last over strictly increasing finite rows, so a bracketing pair exists.
  - `ratio_quantity` now sees only finite values.
  - No integer arithmetic depends on a value.
- **The runner's point path:**
  - It has no non-test `expect` or `unwrap`.
  - A `Quantity::new` error becomes a missing binding.
  - `units::convert_for_dimension` validates finiteness and returns `Result`.
  - `encode_dimension`'s `expect` is total: 29 tokens for 29 variants.
  - `completeness_checker` sees no values.
  - The decoder rejects non-numbers with `DecodeError`, and JSON cannot carry non-finite literals.
- **Not examined:** stack depth on structurally deep expressions. It is not numeric, so it is outside the brief.

**Empirical.**
- **Candidate panics: 0** in all 125,133 point and 501,588 interval evaluations, and in all 216,478 runner lines across seven modes.
- **Main panicked 5,877 times** in my evaluator set:
  - 3,893 at the ratio `expect` (line 1457);
  - 1,080 at the step `expect` (968);
  - 904 at the interpolation `expect` (983).
- **And 2,609 times** in my point runs: ratio 1,676, interpolate 584, step 349.
- **No other panic site** appeared on either side, in a debug build with overflow checks on.

### 4.2 Byte identity where main did not panic (item 2)

`evidence/differential/ee_report.json` and `run_report.json`. Dump hashes are in `dump_sha256.txt`; the dumps are kept in `WT/scratch/rv104_si1b_01/dumps/`.

**Evaluator.**
- **Byte-identical:** 119,256 of 125,133 point lines (81,619 of them blocked identically).
- **Differing:** exactly the 5,877 base panics. Each is now `value: None`, with the site's finding as the **last** finding:
  - `NonFiniteInput` / `divide` / "same-dimension quotient (ratio) must be finite";
  - or `NonFiniteInput` / table id / "table argument must be finite: a NaN argument is neither inside nor outside the table range".
- **Earlier findings:** 307 of those lines also carry findings raised before evaluation reached the site. They are binding `NonFiniteInput`, `StatusBoundaryViolation`, `DuplicateBinding`, `MissingRequiredValue` or `UnsupportedGrammarVersion`.
- **Unexpected differences: 0.**

**Runner.**
- **Point runs:** 42,995 of 45,604 identical. The other 2,609 are base panics, each now a JSON result in which the panicking check reads:
  - `RULE_INPUTS_INCOMPLETE`;
  - a blocking `NonFiniteInput` evaluator finding;
  - no computed value.
- **b = 0:** the candidate's b = 0 bytes equal its plain bytes in all 28,479 cases. On main they are equal in all 26,911 that did not panic, as RV99 N-2 read.
- **Unexpected differences: 0.**

**My 31-input probe** (`evidence/probe/`) agrees: the 13 base panics now block, and the other 18 lines are identical.

**Code reading** gives the same answer:
- **Ratio.** In the same-dimension arm, every non-finite ratio used to reach `Quantity::dimensionless(…).expect`, and every finite ratio still takes the same `ratio_quantity(left.value / right.value)` path.
- **Tables.** A NaN x passed both range comparisons and always reached an `.expect`. A non-NaN x never meets the new check.

### 4.3 Interval mode unchanged (item 3)

- **The interval path never reaches the changed lines.** It calls `eval_binary` only with shadow values of 1.0, so `divide` there computes 1/1. It never calls `eval_table_expression`.
- **`evaluate_interval`:** all 501,588 results equal on both sides, with 0 panics on either side. Each is compared by its value text, finding codes, note codes and a 64-bit hash of the full `Debug`.
- **`run_rule_checks_with_bounds` with b > 0** (b = 1, relative, 1e300; 28,479 each): identical except where main panicked.
  - All 235 such lines per mode come from point-path table checks over user inputs that ran inside a bounded run. Every b = 1 one is content-verified as now blocked.
  - The bounded (interval) checks themselves never panicked on either side.
  - The NaN and duplicate bounds are identical except for 289 base panics each, from the same point-path checks.
- **S-I1's 94 shared cases** pass on both sides: `rule_interval_cases`, 1 test over 94 cases.
- **`test_rule_interval.py`:** 193 passed on both sides, with VENV's python (§N-6 for the environment).

### 4.4 The findings tell the truth (item 4)

From the probe (`evidence/probe/probe_cand.txt`; each line below is that file's output):

| Cause | Finding |
|---|---|
| `1e308/1e-308`, `MAX/next_down(1)`, carried `inf/inf`, a finite numerator over a carried NaN divisor, a carried overflow from an interpolated result | `[NonFiniteInput, "divide", "same-dimension quotient (ratio) must be finite"]` |
| The same overflow with a unit mismatch | `[UnitMismatch, "divide", …]` alone: the unit check precedes the ratio check |
| A NaN numerator over 0 or −0 | `[DivisionByZero]`: the zero check precedes everything |
| `MAX/1`; a finite numerator over carried inf; `5e-324/MAX` | a finite ratio (`MAX`, 0, 0), with no finding, as on main |
| A stress/length overflow | `UnsupportedExpressionForm` (dimension resolution precedes values) |
| Moment/length, stress/ratio and ratio/ratio overflows | carried `inf` with no finding, as on main. Not SI1b's (RR routed carried non-finite values to T3-SI1c) |
| NaN interpolation, NaN step, `0·inf` interpolation, NaN over ±`MAX` rows | `[NonFiniteInput, <table id>, "table argument must be finite: a NaN argument…"]` |
| A padded table id `"  padded_t  "` | subject `padded_t` (trimmed, as for every table finding) |
| Exact NaN | `TableKeyNotFound`, unchanged |
| ±inf in interpolation or step | `TableOutOfRange` ("table argument inf is outside…"), unchanged |
| NaN argument with a unit mismatch, a dimension mismatch, or a malformed table | `UnitMismatch`, `DimensionMismatch`, `TableMalformed` alone: those checks precede the NaN check |
| An untaken select branch that overflows | blocks (the point path is eager); main panicked |

- **Every message names its actual cause.** I accept the RR ruling on `NonFiniteInput`, which is consistent with what I see.
- **In the runner,** the wire form is `code "NonFiniteInput"`, `severity "blocking"`, subject `divide` or the table id. The check reads `RULE_INPUTS_INCOMPLETE` with `RULE_EVALUATOR_ERROR`, `acceptability_relation "none"`, and no computed value.
- **A run with a blocked check still evaluates the others:**
  - Over 17,125 per-check comparisons on the candidate (14,525 on main), every check's outcome inside a three- or five-check run equals the same check run alone, and the aggregate is worst-of.
  - Of 674 multi-check runs that panicked on main, 1,041 panicking checks now carry `NonFiniteInput`, and the other 1,559 checks equal **main's own** single-check runs of them.
- **Wording outside the change:** N-4 and N-5.

### 4.5 Tests and mutants (item 5)

**Fail on main where they should:**
- **The graft** (base code + candidate tests): 51 pass and 4 fail. The four `blocks_*` tests panic at base lines 1457 and 983. The two `…did_not_panic_are_unchanged` tests pass on main, as they should.
- **The runner file on main:** 0 of 2 pass (panics at 1457 and 983).
- **The step site on main is masked.** Each NaN test loops interpolation first, so on main the step site is never reached. I79's S1 and my S1 and S2 show the step assertions are live on the candidate.

**Suites:**

| Suite | Base | Candidate |
|---|---|---|
| `expression_evaluator` lib / conformance / doc | 49 / 1 / 0 | 55 / 1 / 0 |
| `rule_check_runner` lib / acceptability / interval_bounds / invented_demo / rule_interval_cases / point_path_non_finite_run | 14 / 4 / 11 / 3 / 1 / — | 14 / 4 / 11 / 3 / 1 / 2 |
| `rule_pack_document` lib / corpus_parity / invented_demo_document | 6 / 1 / 3 | 6 / 1 / 3 |
| `test_rule_interval.py` | 193 passed | 193 passed |

- **Test-name sets:** none removed. Exactly the 6 + 2 named tests were added.
- **Formatting:** `cargo fmt --check` is clean on the evaluator crate, and `rustfmt --check --edition 2021` is clean on the new runner file.
- **Dependents:** within `core/rules/`, only `rule_pack_document` depends on the evaluator. The runner's sole dependent is `apps/desktop/src-tauri`, outside `core/rules/`.

**My mutants: 11, distinct from I79's 10, at least one per site.** `evidence/mutants/mutants.json`. The two killing tests are "lib", the evaluator's `--lib` tests, and "run", `point_path_non_finite_run`.

| Mutant | Site | Result |
|---|---|---|
| R1: the ratio check NaN-only (the counterpart of I79's Q2) | ratio | killed (lib, run) |
| R2: also block a non-finite quotient over a dimensionless divisor | ratio arm's neighbour | **survives**. My differential: 5,170 evaluator lines and 1,213 runner lines where main did not panic now differ |
| R3: also block a non-finite derived (unique) quotient | ratio arm's neighbour | **survives**. Differential: 1,150 evaluator lines where main did not panic now differ |
| R4: ratio subject `divide` → `ratio` | ratio | killed (lib, run) |
| R5: ratio message changed | ratio | killed (lib, run) |
| R6: the ratio block continues with a 0 ratio instead of `None` | ratio | **survives**. Differential: 445 base-panic lines no longer end with the ratio finding (345 with a spurious `DivisionByZero`); 931 evaluator and 122 runner lines differ from the candidate |
| S1: step subject → `"lookup"` | step | killed (lib, run) |
| S2: step pushes the finding but falls through to the `.expect` | step | killed (panics) |
| I1: interpolation of NaN silently returns the first row's result | interpolate | killed (lib, run) |
| I2: interpolation pushes the finding but falls through | interpolate | killed (panics) |
| N1: the NaN-argument message changed | both table sites | killed (lib) |

8 of 11 are killed by the committed tests, and all 11 by the committed tests plus my differential (N-1).

### 4.6 Scope (item 6)

- **Exactly 3 files** change over the whole repository, one per commit. There is no `Cargo.toml`, lockfile, schema or dependency change.
- **Rust bytes at head** equal `da0758064e`'s.
- **The Python change** is comments plus one docstring (N-3). There is no behaviour change, and 193 passed on both sides.

## 5. I79's return, spot-checked against my own runs

Confirmed:
- the file hashes;
- the suite counts (49 → 55, 33 → 35, 10, 193);
- the fmt state;
- the three sites and their findings;
- "no other panic site";
- "plain = b = 0";
- "interval mode identical";
- the out-of-scope boolean observation, already routed to T3-SI1c (my probes 12–14 show the same carried values).

I did not re-run I79's harness or dumps.

## 6. For ROOT to rule on

1. **N-1:** whether to add the three test assertions now (a small repair round), or carry them into T3-SI1c's test plan. Either is sound, since the shipped code is correct and the property is shown by two independent differentials.
2. **N-2:** the doc-comment wording (ROOT kept the comment).
3. **N-4 and N-5:** routing. Both predate this change, block, and never pass. Changing N-4 changes outputs for inputs that do not panic.
4. **N-6:** a host-rule clarification for future pytest briefs.

## 7. Host and cleanup

- **Cargo:** every cargo command went through the lock with `--offline --locked`. There was no DEC-025, sweep, native job or install.
- **pytest:** only `tests/test_rule_interval.py`, with VENV's python, the memory guard checked. The session conftest's cargo builds were bypassed (N-6).
- **Git:** no writes. Reads were with `GIT_OPTIONAL_LOCKS=0`; the one network read was `ls-remote`, to confirm the pushed head.
- **Scratch:** `WT/scratch/rv104_si1b_01/` (`TMPDIR` there). Nothing went to the system temp directory.
- **Deleted afterwards:** `WT/rv104/` and `WT/targets/rv104-*`.
- **Kept in scratch:** the dumps (gzip), for re-inspection.
- **NUM's write guard** did not refuse, so these records are at `R/REVIEW_RV104/si1b_01/`.

## 8. Evidence

`evidence/` (sealed in `SHA256SUMS`):
- `harness/`: the four Rust and Python harnesses, the probe, the mutant scripts and the driver;
- `differential/`: `ee_report.json`, `run_report.json`, `dump_sha256.txt`;
- `probe/`: base and candidate probe outputs;
- `mutants/`: `mutants.json`, the per-survivor differential reports and logs, `mutant_logs_summary.txt`;
- `suites/`: suite, graft, test-list, pytest and fmt logs, `python_ast_check.txt`;
- `host/`: `driver.log`, the harness run logs, `cargo_jobs_rv104.txt`, `i79_pytest_conftest_cargo.txt`.
