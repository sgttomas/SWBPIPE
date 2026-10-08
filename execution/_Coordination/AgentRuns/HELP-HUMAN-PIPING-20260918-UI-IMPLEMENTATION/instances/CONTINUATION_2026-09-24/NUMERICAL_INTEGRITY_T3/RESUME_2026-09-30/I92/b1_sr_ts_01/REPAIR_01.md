# I92 B1-SR-TS, repair round 1: the three-reader alignment set in the TypeScript reader

TASK (Type 2), I92 (I-TS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

## The basis

- **The brief:** `R/BRIEFS/B1_SR_TS_REPAIR_01.md`, sha256 `40740052f9f84b8b1af88d6cda0ce856dd16b7be2f580315e91277aaef4adb8a` (verified). I re-read `B1_COMMON.md` (`2d170307…2c75`) and `B1_SR_TS.md` (`9698cd33…edbfe`), both verified.
- **The specification:** RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled", items 1–4. I also read the RR sections after it, up to the owner's memory clarification (the four-slot host rule).
- **Read:**
  - RV113's SR-TS review, `R/REVIEW_RV113/rvr_sr_ts_01/REVIEW.md` (`d44dec19…2b85`, verified): S-1, N-1 and N-2, and its TS harness;
  - RV113's SR-PY review, `rvr_sr_py_01/REVIEW.md` (`d8611e59…c3dc`, verified): §10, with `evidence/fg/`, `evidence/probes/` and `evidence/repair_probes/`;
  - I90's `R/I90/b1_sr_rs_01/REPAIR_02.md`, with its `_run_records/repair_02/` (`harness/`, `out/`);
  - RS at `6e3e4fe219` (`git show`, read only): `validate_transport_metadata`, `g5_ordinary` and the G8 model-scope check; and `RE/src/semantic_contract.rs`'s `for_source_metadata` at I1.

**State.** Done, with **one stop fired** (§3). One commit on `codex/piping-t3-b1-t-20261007` in `WT/b1-t`: **`b82b932923`** over `7e47e51b5d`, for items 1 and 2. Item 4 is confirmed with no change. **Item 3 is held, not committed,** under the brief's stop rule: it changes nine 07m transport verdicts. It is a tested patch in this record for ROOT's ruling. Not pushed; ROOT pushes.

**Placeholders:** WT, NUM, P, T, R, RR and VENV as in the dispatch; TS, TT, RS, PY, NMS and S as in `RETURN.md`; S2 = `S/r1` (this round's scratch).

## 0. In brief

| Brief item | Result |
|---|---|
| **The stop (the brief's rule: "No change to any 07m verdict is allowed. If one would change, stop and return")** | **Fired for item 3, the transport header at G2.** It changes **nine 07m transport verdicts** (entries 277 and 286–293, RV113 N-1's nine). Each moves to exactly RS's transport verdict at `6e3e4fe219`. No bound or unbound verdict changes, and no corpus expectation is missed. As the rule requires, **item 3 is not committed.** It is written, tested and measured, and held as a patch for ROOT's ruling (§3, §8) |
| 1. (g) at G8 INVOCATION | **Done** (`b82b932923`). RV113's five (g) probes that TS admitted now read G8 INVOCATION, as in RS. A non-empty `combinations` list still meets TS's G3 combination-coverage conjunct first (§6, for ROOT) |
| 2. C2's precondition keying (N-2) | **Done** (`b82b932923`). `unavailable_precondition`'s code is keyed one-to-one by `precondition`; `receipt_failure`'s set form stands |
| 3. The transport header at G2 (N-1) | **Held** (the stop above). The patch makes TS's transport equal RS's on all 339 07m entries and all 135 probes |
| 4. (f)'s placements | **Confirmed, no change.** RV113's seven (f) probes read G3 COVERAGE bound and unbound, and its four basis-reference probes read G5 ATTEMPT, as ruled |
| The census over 07m at the head | **0 changes** against `7e47e51b5d` on all three verdicts (bound, unbound, transport), and **0 misses** against the corpus's TS expectations |
| vitest and tsc | vitest 3,626 → **3,628**: +2 added (this round's two tests), 0 removed, 0 changed. `tsc --noEmit` clean on both |
| Mutants | **18 of 18 killed by assertions:** 12 at the head (items 1 and 2), and 6 with the held patch (item 3). None is equivalent; none was killed by a load failure |

## 1. Commits and the held patch

| Item | Where | Content |
|---|---|---|
| Commit `b82b932923` | `codex/piping-t3-b1-t-20261007`, over `7e47e51b5d` | Items 1 and 2: TS +13/−2; TT +91/−0, adding two tests and changing none. TS `1fb03701…f909`, TT `351e3f23…40a2` |
| **The held patch** (not committed) | `_run_records/repair_01/item3/item3_transport_header_g2.patch` | Item 3: TS +22/−7 (`headerCode`, the transport's G2 check, the docs); TT +47/−0 (one appended `describe` with one test). It applies cleanly to `b82b932923` with `git apply -p1`. Patched: TS `0b8e99c9…d9a9`, TT `95587111…418f` |

Not pushed; ROOT pushes. The worktree is clean at `b82b932923`.

## 2. Item by item

### Item 1, (g): G8 INVOCATION_MISMATCH in `invocationBinding`'s model-scope check

**Change.** `fail(!model.pressure_contract && !model.combinations?.length && !model.components?.length, …)` becomes PP's acceptance, as ruled:
- no `reference_configurations` member, null included;
- `pressure_contract` absent or null;
- `combinations` and `components` each absent or `[]`.

The check keeps its place, after the invocation keys, digest, project and schema version and before every PREPARATION check.

**Evidence.**
- **RV113's (g) probes** (`fg` set): the five that TS admitted now read G8 INVOCATION bound and stay admitted unbound: `g_reference_configurations_null`, `_empty`, `g_pressure_contract_false`, `g_combinations_null` and `g_combinations_object`. The other three (`pressure_contract` `{}` and `components` `"x"` refused; `pressure_contract` null admitted) read as before. All eight equal RS at `6e3e4fe219` on all three verdicts.
- **Test** "(g): the model-scope members are PP's acceptance, …":
  - refused at G8 INVOCATION, bound, and admitted unbound: `reference_configurations` null, `[]` or `{}`; `pressure_contract` false, `{}` or 0; `combinations` null or `{"x": 1}`; `components` null, `"x"` or `{}`; and `combinations` null beside an unnormalized coordinate (07m's `mm_unnormalized_coordinate`, PREPARATION on its own), which shows that (g) comes first;
  - admitted: `pressure_contract` null; both lists `[]`; both absent;
  - a non-empty `combinations` (`[{}]`, or `"x"`, whose `length` is 1) stays **G3 COVERAGE** first (§6).

### Item 2, C2's cause table: the `precondition` keying (N-2)

**Change.** In `ordinaryAttempts`, G5 ATTEMPT_MISMATCH, ordinary class. An `unavailable_precondition` case's code must now equal `PRECONDITION_CODES[precondition]`:
- `caller` → `caller_not_qualified`;
- `resource_admission` → `resource_admission_not_available`;
- `upstream_no_wrap` → `upstream_no_wrap_not_established`;
- `capture` and `source_family` → `source_unavailable`.

Before, the code only had to be one of the four. Phase `routing` or `preparation` and "no Run" are unchanged. The other branches keep TS's present form, which is the ruled table: `source_error`, `receipt_failure` (the set form), `facade_failure` and the kernel reason.

**Evidence.**
- **Test** "C2's cause table: …":
  - Through `ordinaryAttempts`, on 07j's base case 1: every precondition × the four codes × the phases `routing`, `preparation` and `kernel`. That is 60 rows; only the keyed code in `routing` or `preparation` passes, and every other row is G5 ATTEMPT. A precondition beside a Run is G5 ATTEMPT.
  - Through `validateRetainedPrecision`, with case 1 unavailable through a precondition (its product attempt removed): each of the five keyed pairs is admitted (`needs_recompute`). Two cross-code pairs, one in `routing`, and a keyed code in phase `receipt` are G5 ATTEMPT.
- **RV113's C2 probes** (`c2_*` and `x_reason_cause_receipt_failure`) read as before: TS already had them. They equal RS at `6e3e4fe219`, as I90 reported. No probe of RV113's carries a precondition cause, and none is in 07m.

### Item 3, the transport scope (N-1): the header at G2 — **held** (§3)

### Item 4, (f): confirmed, no change

TS already has the ruled placements: `coverage` (G3) checks `sources[].index`, the owner's kind and case, `material_bases[].index`, and `case_indices` (unique and in range); `ordinaryAttempts` (G5) checks the ordinary attempt's basis.
- RV113's seven (f) probes (`f_mb_index_1`, `f_mb_index_swapped`, `f_src_index_1`, `f_src_index_swapped`, `f_mb_case_indices_duplicate`, `f_mb_case_indices_out_of_range`, `f_src_owner_case_id_other`) read **G3 COVERAGE bound and unbound**.
- The basis-reference probes `r_b_basis_ref_7`, `r_c_basis_omits_case_1` and `r_c_missing_sourceless_basis` read **G5 ATTEMPT bound and unbound**. The invocation-derived controls `r_b_basis_ref_1_second_basis`, `r_c_cases_out_of_order` and `r_c_extra_empty_basis` read G8 PREPARATION bound and are admitted unbound.
- Each equals RS at `6e3e4fe219`. No probe disagrees with the ruling, so nothing changed.

## 3. The stop: item 3 changes nine 07m transport verdicts

**The rule.** The brief says "No change to any 07m verdict is allowed. If one would change, stop and return." Its census asks for "0 changes against `7e47e51b5d` on all three verdicts". The coordinator restated it: "No 07m verdict may change; if one would, stop and return."

**What item 3 does to 07m.** RV113's harness reads every 07m entry three ways: bound, unbound and transport. Its transport reading runs `validateRetainedPrecisionTransport` on each entry's whole source. With the header at G2, **exactly nine entries' transport verdicts change**. They are the nine that RV113's N-1 named: the ruled change, and nothing else.

| 07m entry | Transport at `7e47e51b5d` | Transport with item 3 | RS at `6e3e4fe219` |
|---|---|---|---|
| 277 `g7_not_required_quality_enum_invalid` | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` | the same |
| 286 `g7_selected_quality_enum_invalid` | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` | the same |
| 287 `g7_unavailable_quality_enum_invalid` | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` | the same |
| 288 `g7_quality_case_evidence_ref_empty` | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` | the same |
| 289 `g7_quality_case_extra_member` | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` | the same |
| 290 `g7_quality_status_invalid` | admitted | G2 `SOURCE_NUMERICAL_QUALITY_INVALID` | the same |
| 291 `g7_formulation_limitations_empty` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` | the same |
| 292 `g7_contract_evidence_null` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` | the same |
| 293 `g7_source_block_recovery_present` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` | the same |

Their bound and unbound verdicts (the corpus's pinned G7 first failures) do not change. No corpus entry pins a transport verdict (RV113 N-1), and the whole desktop suite passes with the patch (§7).

**So the brief's census condition and item 3 cannot both hold.** RS met both because it already had the header at G2 (its 07m transport verdicts did not move in I90's round). TS adds the header, so its transport verdicts must move to RS's, as the ruling intends. That is a contradiction in the brief, and ROOT should decide it. I stopped item 3 at the commit:
- **Items 1, 2 and 4 are committed**, or confirmed, at `b82b932923`. Their census is 0 changes on all three verdicts.
- **Item 3 is held as a patch,** with its evidence complete, so that a ruling to apply it needs no new work:
  - its census and probes (§4, §5);
  - its own test, appended in its own `describe`, so that no committed test is renamed;
  - its suite and tsc (§7) and its mutants (§8).

**ROOT's options.**
1. **Apply it.** Rule that 07m's unpinned transport readings are outside the stop rule, then apply the patch (`git apply -p1` at `b82b932923`, then one commit). TS's transport then equals RS's on all 339 entries and all 135 probes.
2. **Keep the stop.** The transport header waits for a ruling on 07m's transport verdicts (for example, SC pinning them in 07n).

## 4. The census over 07m (`_run_records/repair_01/census/`)

**The harness is RV113's,** used unchanged: `rv113Census.test.ts`, sha256 `abcc6e360b2ad09f4d4d59e56cba4c16bd4c88209ad6273d7ed6153f93fe2b45`, copied from `R/REVIEW_RV113/rvr_sr_ts_01/evidence/harness/`.
- It records each 07m entry's input sha256 and three verdicts: bound, unbound, and transport on the whole source. It also records the 135 probes the same way.
- It went into my scratch archives only (P without `execution/`, NMS linked, the eight wasm assets copied), never into the worktree.
- Each side ran as one vitest job through `WT/tools/t3_slot.sh`.

**The base reproduces RV113's own head census.** My run at `7e47e51b5d` (`census_base.jsonl`) is byte-identical to RV113's `census_ts_head.jsonl`.

`compare_r1.py` gives:

| Version | Input | Bound | Unbound | Transport | Misses (TS expectations) |
|---|---|---|---|---|---|
| **The head, `b82b932923`** (items 1 and 2) | 0 | 0 | 0 | **0** | **0** |
| With the held patch (item 3) | 0 | 0 | 0 | **9** (§3) | 0 |

The misses are against `expected_by_reader.typescript` (else `expected`) for the 294 first failures, and against the stated eligibility and standing for the 28 must-pass entries and the 17 bases.

## 5. RV113's probes, and TS against RS (`_run_records/repair_01/probes/`)

**The probes are RV113's 135,** rebuilt by I90's `build_probes.py` from RV113's records (sha256 `19613cfd61305636f3579fe33911d3fa1029ff3472f7006b7679b964a59e8c7a`, as I90 recorded): the 103 `v5` probes (with the six C2 probes), the 15 (f)/(g) probes and the 17 repair-01 probes.

**RS's side** is I90's `out/census_head.jsonl` and `out/probes_head.jsonl` at `6e3e4fe219`. The comparison normalizes each verdict to (gate, code), or to (admitted, invocation_bound, numerical_eligible, publication_sha256).

| | The head `b82b932923` | With the held patch |
|---|---|---|
| Probes changed against `7e47e51b5d` | 5 (the (g) five) | 15 (the (g) five, the eight N6 header probes' transport, and the two compound probes' transport) |
| **07m: TS equals RS on all three verdicts** | 329 of 339 | **338 of 339** |
| **Probes: TS equals RS on all three verdicts** | 125 of 135 | **133 of 135** |
| What still differs | the nine §3 transport verdicts; the ten N6 probes' transport (item 3); and the declared G7 codes below | only the declared G7 codes below |

**The declared G7 differences** are each reader's own G7 base code. They are unchanged, and they are the same as in I90's REPAIR_02 §3:
- 07m 139 `g7_maximum_off_enclosure`, bound and unbound: TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, RS `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`. This is the corpus's per-reader entry.
- `n6_carrier_evidence_with_case_defect`, bound and unbound: TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, RS `SOURCE_NUMERICAL_CASE_INVALID`.
- `n6_contract_evidence_null_and_source_block_recovery`, bound and unbound: TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`, RS `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`.

So the coordinator's 15 probes (I90 REPAIR_02 §3) agree as follows:
- the five (g) probes agree **now**;
- the eight header probes and the two compound probes' transport readings agree **with the held patch**;
- the compound probes' raw G7 codes stay per reader, as declared.

## 6. For ROOT

1. **The stop (§3).** Rule whether 07m's unpinned transport readings are inside the "no 07m verdict" rule. Then either apply the held patch (one `git apply -p1` and one commit at `b82b932923`; its evidence is complete here) or keep item 3 waiting.
2. **A (g)-family difference that remains, and is outside my items.** TS's G3 `coverage` refuses an invocation whose `combinations` has a non-zero `length` (a non-empty list, or a string such as `"x"`) at **G3 COVERAGE**. This is TS's stand-in for C1's G3 "separate exhaustive combination coverage" row; B1 has no combination coverage. RS at `6e3e4fe219` has no such G3 conjunct, and neither has PY at I1 (its only `combinations` test is at G8), so they refuse the same input at G8 INVOCATION under the ruled (g) rule.
   - The ruling says the model-scope members "go to G8 INVOCATION_MISMATCH". The brief scoped TS's change to `invocationBinding`'s model-scope check, so I did not move G3's conjunct.
   - TT pins TS's present reading (two rows), so that a later ruling changes it deliberately.
   - No 07m entry and none of RV113's probes carries a non-empty `combinations`, and the B1 producer emits none.
   - **ROOT to rule:** move TS's conjunct to G8 (a one-line removal from `coverage`, after which the (g) check refuses it), or declare it until B2's combination coverage defines G3's row.
3. **No probe carried a `precondition` cause.** Item 2's keying is pinned only by TT's new test. SC's 07n should pin each precondition's keyed code, satisfied and broken, as the ruling says.

## 7. vitest and tsc, test by test (`_run_records/repair_01/suites/`)

Each run was one job through `WT/tools/t3_slot.sh`, in archives of P without `execution/`, with NMS linked and the eight wasm assets copied (I71's eight, `wasm_assets.sha256`).

| Suite | `7e47e51b5d` | The head `b82b932923` | With the held patch |
|---|---|---|---|
| vitest, the whole desktop suite (141 files) | 3,626 passed | **3,628** passed: **+2 added** (this round's two tests), 0 removed, 0 changed | 3,629 passed: against the head, **+1 added** (the transport test), 0 removed, 0 changed |
| `tsc --noEmit -p tsconfig.json` | rc 0, no output | rc 0, no output | rc 0, no output |

The only differences are this round's added tests. No test changed.

## 8. Mutants (`_run_records/repair_01/mutants/`)

`mutants_r1.py` (its set names follow the RR numbering: `items12` for the brief's items 1 and 2, `item4` for the brief's item 3): one exact string edit to TS in a scratch copy; then TT alone (vitest, JSON reporter); then TS restored and its sha256 checked. A mutant is **killed only when TT loaded** (all its tests collected: 492 at the head, 493 with the patch) **and an assertion failed.** Each set ran as one job through `t3_slot.sh`. N0, the unmutated control, passes in both.

**At the head (items 1 and 2): 12 of 12 killed.**

| Mutant | Edit | Killed by |
|---|---|---|
| G-1 | (g): the `reference_configurations` conjunct dropped | the (g) test |
| G-2 | (g): `pressure_contract` back to a falsy test | the (g) test (`false`, `0`) |
| G-3 | (g): `combinations` back to `!combinations?.length` | the (g) test (null, an object) |
| G-4 | (g): `components` back to `!components?.length` | the (g) test |
| G-5 | (g): the check's code PREPARATION instead of INVOCATION | the (g) test |
| G-6 | (g): "absent or []" admits null | the (g) test |
| C-1 | C2: the keying coarsened back to the set of four codes | the C2 test |
| C-2 to C-6 | C2: each precondition keyed to a wrong code (`caller`, `resource_admission`, `upstream_no_wrap`, `capture`, `source_family`) | the C2 test, each |

**With the held patch (item 3): 6 of 6 killed.**

| Mutant | Edit | Killed by |
|---|---|---|
| T-1 | the transport header check skipped | the transport test |
| T-2 | the transport header in Python's order (the raw path's) | the transport test (the two compound rows) |
| T-3 | the header refusal labelled G7 | the transport test |
| T-4 | Rust's order: `source_block_recovery` no longer before `contract_evidence` | the transport test |
| T-5 | Rust's order: a `carrier_evidence` branch added | the transport test |
| T-6 | the raw path's order changed (Python's evidence-first order lost) | the transport test's raw column, and SR-TS's own N6 test |

An earlier run of the item-3 set (`mutants/item3_v1/`) was against a first form of the patch, which put the transport test inside the round's first `describe` and renamed it. That gave the same 6 of 6. I re-cut the patch as its own appended `describe`, so that no committed test is renamed, and re-ran the set; the table is that run.

## 9. Host

- **Every heavy job ran through `WT/tools/t3_slot.sh`,** one job of mine at a time, in this order:
  1. the census and probes, at base, with the full version, and at the head;
  2. four single-file TT runs while writing;
  3. the suites and tsc, at base and at the head, then with the patch (twice: once per form of the patch);
  4. the three mutant programmes.

  `jobs.txt` and `suites/*_rc.txt` record each start and end. No cargo ran.
- **One slip, disclosed.** My first suites command split its arguments wrongly under zsh. Its four slot jobs each exited at the script's first line, with no vitest started and no output. I re-ran them correctly.
- **Waits:** one per job, its background completion notice or the foreground call. None remains.
- **Scratch:** `S2` only, with `TMPDIR` in it. The archives (`base`, `dev`, `head`, `mut12`, `mut4`) carried their own NMS link and wasm copies, and I deleted them at the end. The worktree `WT/b1-t` received only TS and TT, and is clean.
- **Records:** no symlink, no `build` folder, placeholder paths only. Screened with the strict pattern and the machine's host name (§10).
- No DEC-025, installs or pushes. Git reads used `GIT_OPTIONAL_LOCKS=0`. I killed no job.

## 10. Records

`_run_records/repair_01/`:
- `census/`: `census_base.jsonl`, `census_head.jsonl` and `census_item3.jsonl`; `compare_head.out` and `compare_item3.out`;
- `probes/`: `probes_base.jsonl`, `probes_head.jsonl` and `probes_item3.jsonl`; `probes_all.sha256`;
- `harness/`: `rv113Census.test.ts` (RV113's), `build_probes.py` (I90's), `compare_r1.py`, `run_census.sh`, `run_tt.sh`, `suites.sh`, `compare_suites.py` and `mutants_r1.py`;
- `suites/`: `base_rc.txt`, `head_rc.txt` and `item3_rc.txt`; `suites_compare_head.json` and `suites_compare_item3.json`; `suites_summary.txt`; `wasm_assets.sha256`;
- `mutants/`: `items12/`, `item3/` and `item3_v1/` (each `mutants.jsonl` and `mutants.out`);
- `item3/`: `item3_transport_header_g2.patch`;
- `diff/`: `repair_01.diff` (`7e47e51b5d..b82b932923`) and `commits.txt`;
- `jobs.txt`.

`SHA256SUMS.repair_01` covers this file and every file above.
