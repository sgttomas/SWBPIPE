# I92 B1-SR-TS, repair round 1, item 3 and the G3 conjunct: ROOT's two rulings applied

TASK (Type 2), I92 (I-TS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

**The basis.** This follows the coordinator's message relaying ROOT's rulings on my REPAIR_01, `R/I92/b1_sr_ts_01/REPAIR_01.md` (`592d6164…6ace`):
1. **Item 3.** The brief's "no 07m verdict may change" covers the input, bound and unbound verdicts. The transport verdicts of 07m entries 277 and 286–293 change as item 3 rules, onto RS's G2 reading. Any other change is still a stop. I was to apply the held patch at `b82b932923` with `git apply -p1`, as one commit.
2. **The G3 conjunct.** TS's G3 combinations conjunct moves to G8: I was to remove it, update TT's two rows, and make that a separate commit.

REPAIR_01.md and SHA256SUMS.repair_01 are untouched.

**State.** Done. Two commits on `codex/piping-t3-b1-t-20261007` in `WT/b1-t`, over `b82b932923`. **New head: `6fa6a64658f6e94c817d26e0cab28e5087b2cf01`.** No stop fired. Not pushed; ROOT pushes.

**Placeholders:** as in REPAIR_01.md.

## 1. The commits

| Commit | Content | Files |
|---|---|---|
| `1d9455c714` | **Item 3:** the held patch, applied with `git apply -p1` at `b82b932923`, byte for byte. TS and TT equal the tested patch's (`0b8e99c9…d9a9`, `95587111…418f`) | TS +22/−7; TT +48/−0 |
| `6fa6a64658` | **The G3 conjunct:** `coverage`'s `&& !(invocation.request?.model?.combinations?.length)` is removed, so the (g) model-scope rule at G8 INVOCATION refuses a non-empty `combinations`, as in RS and PY. TT: the two rows that pinned the G3 reading now expect G8 INVOCATION bound, and two rows are added for admission unbound | TS +3/−1; TT +6/−4 |

At the head, TS is `40bfdd4e…f515` and TT is `d664ed5b…ad1e`. Against `7e47e51b5d`, the round as a whole changes TS by +38/−10 and TT by +141/−0. Each was tested in a scratch archive before its commit, and is byte-identical to what was tested.

**A correction to REPAIR_01** (which stays sealed). Its §1 gave the held patch's TT as "+47/−0". Git's count for the applied commit is **+48/−0**: my count there skipped one empty added line. The bytes are the ones tested; only the stated count was wrong.

## 2. The census over 07m at the head

The census uses RV113's harness, unchanged (`rv113Census.test.ts` `abcc6e36…2b45`), in a scratch archive of the head, as one `t3_slot.sh` job. It is compared with my `7e47e51b5d` census, which is byte-identical to RV113's own.

**`CENSUS 339 entries, 7e47e51b5d → 6fa6a64658: input 0, bound 0, unbound 0, transport 9; 0 misses against the corpus's TypeScript expectations.`** The nine are exactly the ruled ones: entries 277 and 286–293. Each moves onto RS's G2 reading at `6e3e4fe219`:

| 07m entry | Transport at `7e47e51b5d` | At the head (= RS) |
|---|---|---|
| 277, 286, 287, 288, 289 (`g7_*` case defects) | admitted | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 290 `g7_quality_status_invalid` | admitted | G2 `SOURCE_NUMERICAL_QUALITY_INVALID` |
| 291 `g7_formulation_limitations_empty` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` |
| 292 `g7_contract_evidence_null` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` |
| 293 `g7_source_block_recovery_present` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` | G2 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` |

There is no other change. The head's census and probe outputs are byte-identical to REPAIR_01's held-patch runs (`census_item3.jsonl`, `probes_item3.jsonl`), so the G3 conjunct's removal moved no 07m entry and no probe.

## 3. The equality counts against RS

RS is I90's `out/census_head.jsonl` and `out/probes_head.jsonl` at `6e3e4fe219`, normalized as in REPAIR_01 §5.
- **07m: TS equals RS on 338 of 339 entries, on all three verdicts.** The one difference is entry 139 `g7_maximum_off_enclosure`, bound and unbound: TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, RS `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`. This is the corpus's declared per-reader G7 code.
- **Probes: TS equals RS on 133 of 135, on all three verdicts.** The two differences are the compound probes' raw G7 codes, bound and unbound, which each reader declares as its own:
  - `n6_carrier_evidence_with_case_defect`: TS `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, RS `SOURCE_NUMERICAL_CASE_INVALID`;
  - `n6_contract_evidence_null_and_source_block_recovery`: TS `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`, RS `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN`.

  Their transport readings now agree.
- **Probes changed from `7e47e51b5d`: 15.** These are the five (g) probes (bound), the eight N6 header probes' transport, and the two compound probes' transport.
- **The G3 conjunct.** No 07m entry and none of RV113's probes carries a non-empty `combinations`, so its move shows only in TT's rows and in mutant G3-1.

## 4. vitest and tsc against `7e47e51b5d`

Each ran as one `t3_slot.sh` job in a scratch archive, with NMS linked and the eight wasm assets copied.

| Suite | `7e47e51b5d` | Head `6fa6a64658` | Test by test |
|---|---|---|---|
| vitest, the whole desktop suite (141 files) | 3,626 passed | **3,629** passed | **+3 added, 0 removed, 0 changed**: the three B1 SR-TS repair 01 tests in TT, all passed (below) |
| `tsc --noEmit -p tsconfig.json` | rc 0, no output | rc 0, no output | — |

The three added tests:
1. "(g): the model-scope members are PP's acceptance, at G8 INVOCATION before any PREPARATION check; unbound reads admit them". **Changed since `b82b932923`:** its two `combinations` rows (`[{}]` and `"x"`) now expect G8 INVOCATION bound instead of G3 COVERAGE, and it gains the same two rows unbound (admitted, not eligible).
2. "C2's cause table: an unavailable_precondition code is keyed one-to-one by its precondition (N-2); the receipt_failure set form stands". Unchanged since `b82b932923`.
3. "the header at G2 with Rust's code, then the preview-physics metadata at G7; the raw path is unchanged", in the `describe` "B1 SR-TS repair 01: the transport reading's base header at G2 (Rust's code), then its metadata at G7". Added at `1d9455c714`.

No test that existed at `7e47e51b5d` changed.

## 5. Mutants at the head

`mutants_r1.py`, set `final`, ran as one `t3_slot.sh` job in a scratch copy of the head:
- each mutant is one exact string edit to TS, then TT alone, then TS restored and its sha256 checked;
- a mutant is killed only when TT loaded (all 493 tests collected) and an assertion failed;
- N0 (unmutated) passes 493 of 493.

**8 of 8 killed (item 3's six, and the conjunct's re-addition).**

| Mutant | Edit | Killed by |
|---|---|---|
| T-1 | the transport header check skipped | the transport test |
| T-2 | the transport header in Python's order | the transport test |
| T-3 | the header refusal labelled G7 | the transport test |
| T-4 | Rust's order: `source_block_recovery` no longer before `contract_evidence` | the transport test |
| T-5 | Rust's order: a `carrier_evidence` branch added | the transport test |
| T-6 | the raw path's order changed | the transport test's raw column, and SR-TS's N6 test |
| **G3-1** | **TS's G3 combinations conjunct re-added** | **the (g) test: `combinations` `[{}]` and `"x"` read G3 COVERAGE, not G8 INVOCATION** |

## 6. Host

- **Every heavy job ran through `WT/tools/t3_slot.sh`,** one job of mine at a time:
  1. a single-file TT run of the conjunct change, before its commit;
  2. the census and probes;
  3. vitest;
  4. tsc;
  5. the mutant set.

  `jobs.txt` and `suites/head_rc.txt` record each start and end. No cargo ran.
- **Waits:** one per job, its foreground call or background completion notice. None remains.
- **The shell's working directory** was my scratch folder `S2` for every command; Git ran with `git -C WT/b1-t`, and reads used `GIT_OPTIONAL_LOCKS=0`.
- **Scratch:** the archives (`head2`, `mut5`) had their own NMS link and wasm copies. I deleted them at the end. NMS's `.vite-temp/` is empty.
- **Git:** two commits on my branch only, with no push. The worktree is clean at the head.
- **Records:** no symlink, no `build` folder, placeholder paths only. Screened with the strict pattern and the machine's host name.
- No DEC-025 or installs, and I killed no job.

## 7. Records

`_run_records/repair_01_item3/`:
- `census/`: `census_head.jsonl`, `probes_head.jsonl`, `compare_head.out`;
- `suites/`: `head_rc.txt`, `suites_compare.json` (against `7e47e51b5d`), `suites_summary.txt`;
- `mutants/`: `mutants.jsonl`, `mutants.out`, `mutants_r1.py`;
- `diff/`: `item3_and_g3.diff` (`b82b932923..6fa6a64658`) and `commits.txt`;
- `jobs.txt`.

`SHA256SUMS.repair_01_item3` covers this file and every file above. REPAIR_01.md and SHA256SUMS.repair_01 are unchanged.
