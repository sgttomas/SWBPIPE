# I91 B1 SR-PY, repair 01: PY's G8 false accepts (b), (c), (d1), (d2), and a fifth, (e)

TASK (Type 2), I91 (I-PY), for ROOT (HELP_HUMAN, Agent 0), the return path. No delegation. 2026-10-07 UTC.

**Instruction:** ROOT's message dispatching this round, and RR "I91's SR-PY verified; the Build check accepted; PY's four false accepts repaired before RV-R" (read in NUM's `T/ROOT_RULINGS_V1.md`). Placeholders as in `RETURN.md` (sha256 `bc7fa865…f50a`, unchanged): WT, NUM, P, PY, T, R, RR, VENV; `S` = `WT/scratch/i91_b1_sr_py`; `PRE` = `75132d2673` (SR-PY as returned); `HEAD` = `11cc14e3e6`.

## 0. Summary

- **(b), (c), (d1) and (d2) are repaired,** each with a test whose gate and code equal Rust's and TS's (sites cited in §2), and each with mutants killed by assertions.
- **A fifth false accept, (e), found while repairing (c), is repaired too, in its own commit, for ROOT to confirm (§3):** PY checked a material basis's materials only through a CaseSource that uses it, so a basis used only by sourceless cases (07j's `not_required` case on its own named basis) was admitted with any material list. Rust and TS check every basis.
- **The census holds:** over 07m, three ways, 0 of 1,017 outcomes differ from BASE, after (b)–(d2) and again after (e).
- **Suites:** the Python set at HEAD against `75132d2673`'s run, test by test: 1,934 → 1,937 passed (30 skipped in both); **+3 added (the three repair tests), 0 removed, 0 changed.**
- **Mutants:** 10 new ones for the repair and the 30 earlier ones re-run: **40 of 40 killed by assertions.** One repair mutant (the basis count dropped) first survived, refused through other paths; a pinned input now kills it (§4.3).
- **Two more differences found and left for ROOT (§5):** a wrong `material_bases[].index` (Rust G8 PREPARATION, TS G3 COVERAGE, PY admits) and a `reference_configurations` member in the invocation's model (Rust G8 INVOCATION; TS and PY admit). The readers do not agree among themselves on these, so neither is a pure PY alignment.

## 1. Head and commits

`codex/piping-t3-b1-p-20261007` in `WT/b1-p`, **head `11cc14e3e65363790f43943b09933a57f4da28e4`**, three commits over `75132d2673`, clean. 2 files, +148/−18 (PY and the contract test only; `compatibility.py` and the carrier test unchanged):

| Commit | What |
|---|---|
| `a53d79a7a8` | (d1), (d2), (b), (c), with two contract tests |
| `03642b5abc` | (e), with one contract test |
| `11cc14e3e6` | Test only: (c)'s basis count pinned with a missing sourceless basis (§4.3) |

File sha256 at HEAD: PY `6bac1f04…b053`, contract test `7b0a7535…5469`; `compatibility.py` `ec122dd1…ba77` and the carrier test `c9565e66…3da1` as returned. CORPUS unchanged (`c21112fd…6807`).

## 2. The four findings

All in `_g8`. Rust sites are in `RE/src/retained_precision.rs` `g8`, TS sites in `P/apps/desktop/src/features/results/retainedPrecision.ts` `invocationBinding`, line numbers at I1 (unchanged in `WT/b1-p`).

| Finding | PY at HEAD | Rust | TS | Gate and code (all three) |
|---|---|---|---|---|
| (d1) an invocation member other than `request` and `solver_mode` | `:1464`, first check of `_g8`: `type(invocation) is dict and set(invocation) == {"request", "solver_mode"}` | `:3427`, the first `need`: `o.len() == 2 && o.contains_key("request") && o.contains_key("solver_mode")` (with the digest) | `:1110`, the first `fail`: `same(Object.keys(invocation).sort(), ['request', 'solver_mode'])` | G8 `RETAINED_PRECISION_INVOCATION_MISMATCH` |
| (d2) a solver mode other than the two | `:1464`–`:1465`, the same check: `invocation["solver_mode"] in ("sparse_interactive", "dense_scrutiny")` | `:3441`, the second `need`: `matches!(mode, "sparse_interactive" \| "dense_scrutiny")` | `:1110`, the same first `fail`: `[…].includes(invocation.solver_mode)` | G8 `RETAINED_PRECISION_INVOCATION_MISMATCH` |
| (b) DESIGN §3.3 step 2 for every case | `:1530`, in the per-case loop: `o["material_basis_ref"] == case_bases[i]`, the case's selector numbered in first-seen order | `:3512`: `u(&o["material_basis_ref"]) == index as u64` | `:1155`: `ordinary.material_basis_ref === bi && …` | G8 `RETAINED_PRECISION_PREPARATION_MISMATCH` |
| (c) exactly one basis per selector, each with exactly its cases | `:1542`–`:1545`: the count, then each basis's `selector` and `case_indices` | `:3535`–`:3546`: `len() == expected_selectors.len()`, then `selector` and `case_indices == …` | `:1155` (`case_indices.includes(ci)`), `:1157` (the count), `:1159` (`same(mb.case_indices, …)`) | G8 `RETAINED_PRECISION_PREPARATION_MISMATCH` |

**The order inside G8 is DESIGN §3.3's:** the invocation shape now comes first (before the digest; every check there is INVOCATION), then the project and model-scope checks, then one loop in request order with 1 the requested mode, 2 the material basis, 3 P1, 4 P2–P4, then the basis list. The loop moved below `_g8`'s helper definitions, because step 2 needs `unit`; no check lies between its old and new place.

**Tests** (`P/tests/test_retained_precision_contract.py`):
- `test_b1_repair01_g8_invocation_members_and_solver_mode`: an extra member (on `ordinary_prepared_synthetic` and on 07j's two-case statement); `solver_mode` removed, `"foo"`, `"SPARSE_INTERACTIVE"`, `null` and a list: each G8 INVOCATION. Control: `dense_scrutiny` with the requested modes unedited stays G8 PREPARATION (step 1).
- `test_b1_repair01_g8_material_basis_of_every_case_and_exact_case_indices`: (b) the `not_required` case's `material_basis_ref` 7, and 1 beside a second basis listing it; (c) the basis omitting the `not_required` case, listing its cases out of order, an extra basis with no case, and on `two_case_two_groups_synthetic` the second basis's case moved into the first: each G8 PREPARATION. Controls: 07j's statement and the two-group base unedited are admitted and eligible.

**Before and after** (`probes/gaps_repair01.txt` against `RETURN.md`'s `probes/gaps_{base,head}.txt`): (b) 7 admitted eligible → G8 PREPARATION; (c) admitted eligible → G8 PREPARATION; (d1) admitted eligible → G8 INVOCATION; (d2) `"foo"` admitted at BASE, G8 PREPARATION at PRE → G8 INVOCATION.

## 3. The fifth: (e), every material basis's materials

**Found while repairing (c).** PY checked a basis's material list (the used materials in input order; each material's id, selection, explicit G, E and G) only inside its per-CaseSource loop, through the source's `material_basis_ref`. A basis that no CaseSource names, which is a basis used only by `not_required` cases (or by cases without a source), was never checked. Rust (`g8`'s material bases loop, `:3536`–`:3570`: `…eq(expected_material_indices…)`, then per material `selected_material(raw, &cases[ci])`) and TS (`:1158`–`:1164`: `same(mb.materials.map(…input_index), …)`, then per material and per case `selectedMaterial(raw, cases[ci])`) check every basis, G8 PREPARATION.

**Probe** (`probes/sourceless_basis_{pre,repair01}.txt`): 07j's statement with case 1 given a named modulus point (`tp:b1`, an invocation edit), its ordinary attempt on a second basis, and that basis's materials correct: admitted and eligible at PRE and HEAD. With that basis's E wrong, its selection the base's, no material, or another material id: **admitted and eligible at PRE; G8 PREPARATION at HEAD.**

**Repair** (`03642b5abc`, `:1543`–`:1552`): in the basis loop of (c), every basis's material list and each material's values, selected for the basis's first case (all of its cases share one selector, so this equals TS's per-case check). The per-source check stays. **Test** `test_b1_repair01_g8_every_material_basis_has_its_materials_checked`: the correct second basis admitted and eligible; E wrong, G wrong, the selection the base's, another id, a derived shear origin, no material: each G8 PREPARATION.

ROOT ruled four findings; this is a fifth of the same class (G8's material-basis checks, which Rust and TS apply to every basis). It is in its own commit so that ROOT can drop it. If kept, SC may add a 07n entry for it beside (b)–(d2)'s.

## 4. Evidence

### 4.1 The census (R5)

`scripts/census.py`, as in `RETURN.md` §2, over 07m (`c21112fd…6807`; 17 bases, 294 mutations, 28 must-pass), raw with the invocation, raw without, and transport, against the same BASE outcomes (`census/base.json`):

| Reader | Outcomes | Differ from BASE |
|---|---|---|
| `a53d79a7a8` ((b), (c), (d1), (d2)) | 1,017 | **0** |
| `03642b5abc` (with (e); HEAD's PY is byte-identical) | 1,017 | **0** |

### 4.2 The suites against `75132d2673`

The 27-file set (`../scripts/suites.sh`, I83's), under `lockf -k WT/guard/cargo_job.lock`, with my own CLI builds, in `git archive` copies; compared from junit XML test by test (`../scripts/compare_suites.py`):

| Run | Passed | Skipped | Failed | Against `75132d2673` (1,934 passed, 30 skipped) |
|---|---|---|---|---|
| `03642b5abc` | 1,937 | 30 | 0 | +3 added (all pass), 0 removed, 0 changed |
| HEAD `11cc14e3e6` | 1,937 | 30 | 0 | **+3 added (all pass), 0 removed, 0 changed**; identical to `03642b5abc`'s run, test by test |

The three added are the repair tests: `test_b1_repair01_g8_invocation_members_and_solver_mode`, `test_b1_repair01_g8_material_basis_of_every_case_and_exact_case_indices`, `test_b1_repair01_g8_every_material_basis_has_its_materials_checked`. Against BASE (I1): 1,922 → 1,937, +15, all SR-PY's.

### 4.3 Mutants

`scripts/mutants_r01.py` (`RETURN.md`'s harness, extended): one textual edit to a copy of the head tree, asserted to match once, restored byte for byte. N0, PRE and the repair mutants run both retained test files in full (463 tests); the 30 earlier mutants run the B1 and N1 tests (`-k "b1_ or rv108_n1"`), which hold every test that killed them before. Part 1 ran on `03642b5abc` (HEAD's PY); part 2 on HEAD (`11cc14e3e6`).

**The repair's mutants:**

| Id | Mutant | Outcome | Killed by |
|---|---|---|---|
| N0 | none | passes 463/463 (both parts) | — |
| R-d1 | the invocation's member set unchecked | killed | the invocation test |
| R-d2 | the solver mode's domain unchecked (an unknown mode then fails G8 PREPARATION through the fallback) | killed | the invocation test |
| R-d2-prep | the domain checked at PREPARATION instead of INVOCATION | killed | the invocation test |
| R-b | step 2 removed | killed | the material-basis test |
| R-b-sourced | step 2 only for cases with a product attempt (PRE's coverage) | killed | the material-basis test |
| R-c-count | the basis count unchecked | **survived part 1**; killed in part 2 | the (e) test's missing-basis assertion, added in `11cc14e3e6` |
| R-c-cases | a basis's case list unchecked | killed | the material-basis test |
| R-c-superset | a basis need only include its cases (order and extras free) | killed | the material-basis test |
| R-e-list | a basis's material list unchecked | killed | the (e) test |
| R-e-values | a basis's materials' values unchecked | killed | the (e) test |
| PRE | HEAD's tests on `75132d2673`'s PY | exactly the 3 repair tests fail | — |

**Why R-c-count first survived:** with the count dropped, an extra basis still failed G8 PREPARATION through the fail-closed fallback (its selector index is out of range), and a missing *sourced* basis through the CaseSource's own `_at` reference. A missing basis used only by the sourceless `not_required` case is checked by nothing else, so `11cc14e3e6` pins it (G8 PREPARATION in all three readers: Rust's count, TS's `material_bases[bi]?.case_indices.includes(ci)`).

**The 30 earlier mutants** (`RETURN.md` §7) are all still killed by the same tests. Two G8 mutants now also fail the repair tests: dropping non-selected cases (or all but the first) from the loop leaves the selector list short, so more statements are refused. Every kill is an `AssertionError` on a pinned verdict (checked in the junit XML).

**Totals at HEAD: 40 mutants, 40 killed by assertions, none surviving.**

## 5. For ROOT

1. **(e) is my addition (§3).** Please confirm or drop `03642b5abc`.
2. **Two G8-adjacent differences, not changed, because the three readers disagree** (`probes/gaps_repair01.txt`, (f) and (g)):
   - **(f)** a `material_bases[].index` that is not its position: Rust refuses at G8 PREPARATION (`:3538`, `u(&mb["index"]) == mi`), TS at **G3 COVERAGE** (`:264`, `m.index === i`), and PY admits it (eligible). Aligning PY needs a ruled gate.
   - **(g)** an invocation model carrying `reference_configurations`: Rust refuses at G8 INVOCATION (`:3452`–`:3455`); TS and PY admit it.
   Both could become per-reader 07n entries, or a ruled alignment.
3. **For SC (07n):** ROOT's ruling adds an entry for each of (b), (c), (d1), (d2), expected the same in all three readers. The inputs in §2's tests are ready-made: 07j's statement for (b) and (c), `ordinary_prepared_synthetic` for (d1) and (d2). For (e), §3's construction.

## 6. Host

- **Cargo:** the two CLI authorities rebuilt through `WT/tools/t3_cargo.sh` into `WT/targets/i91-b1-sr-py/` (I had deleted it at the first return). Both binaries' sha256 equal the first build's (`549cc2ca…d9a9`, `57064fa9…3a33`).
- **pytest:** the suite under `lockf -k WT/guard/cargo_job.lock` with my own binaries; the census, probes, quick runs and the mutant lane with my own binaries. No built test binary was run directly.
- **Waits:** bounded (at most about 9 minutes), each ending when its job's process was gone; none left.
- **Not done:** no DEC-025, install, or other agent's job touched. Commits on my branch only.
- **Cleanup:** the `git archive` copies (`head3`, `head4`, the mutant copy, which matched HEAD after its last restore, and the pre-repair tree) and `WT/targets/i91-b1-sr-py/` are deleted. `WT/b1-p` is clean at `11cc14e3e6`. No process, wait or monitor of mine is running.

## 7. Records

`REPAIR_01.md` and `SHA256SUMS.repair_01` (over the files below), in `R/I91/b1_sr_py_01/`; `RETURN.md`, `SHA256SUMS` and the files they cover are unchanged.
- `_run_records/repair_01/diff/`: `commits.txt`, `diffstat.txt`, `repair_01.diff` (`75132d2673..03642b5abc`).
- `_run_records/repair_01/census/`: `head_r1a.json`, `head_r1.json`, `compare_r1a.txt`, `compare_r1.txt`.
- `_run_records/repair_01/probes/`: `gaps_pre.txt`, `gaps_repair01.txt`, `sourceless_basis_pre.txt`, `sourceless_basis_repair01.txt`.
- `_run_records/repair_01/suites/`: junit XML (gzipped), tail and return code for `03642b5abc` (`py_r01`) and HEAD (`py_r01b`); `py_compare_r01*` (each against `75132d2673`'s run, `../suites/py_head2.xml.gz`, already recorded) and `py_compare_r01_r01b.json` (the two against each other).
- `_run_records/repair_01/mutants/`: `mutants_r01_part1.jsonl` and `mutants_r01_part2.jsonl`, with their logs.
- `_run_records/repair_01/scripts/`: `mutants_r01.py`, `probe_sourceless_basis.py`, the extended `probe_gaps.py`. The census, sanitizer and suite scripts are `RETURN.md`'s (`_run_records/scripts/`), unchanged.
