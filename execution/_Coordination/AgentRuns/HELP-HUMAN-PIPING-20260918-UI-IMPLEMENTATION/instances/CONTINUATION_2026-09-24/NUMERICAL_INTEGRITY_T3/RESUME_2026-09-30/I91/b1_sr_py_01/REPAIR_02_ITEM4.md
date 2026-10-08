# I91 B1 SR-PY, repair 02, item 4: the transport header check at G2, committed as ruled

TASK (Type 2), I91 (I-PY), for ROOT (HELP_HUMAN, Agent 0), the return path. No delegation. 2026-10-08 UTC.

**Basis.**
- ROOT's ruling, relayed by the coordinator, RR "I91's and I92's rounds verified; the header move's nine transport changes ruled in; …":
  - "No 07m verdict may change" applies to the input, bound and unbound verdicts.
  - The transport verdicts of 07m entries 277 and 286–293 (RV113 N-1's nine) change as item 4 rules: G7 → G2, with the code and detail unchanged, equal to RS's reading.
  - Any other change is still a stop.
- The steps: apply and commit the held patch; re-run the census; run the four retained-reader files and the full Python suite against R01; run the item-4 mutants; record it all here, leaving `REPAIR_02.md` and `SHA256SUMS.repair_02` as they are.
- **Brief:** still `R/BRIEFS/B1_SR_PY_REPAIR_02.md` (sha256 `6fcb24e4…e793e3`).

**Placeholders** as in `REPAIR_02.md`, plus `S`, `R01` = `11cc14e3e6`, `R02` = `70d4a68bd7` (REPAIR_02's head) and `HEAD` = `2843a59a16`. Outputs are in `_run_records/repair_02_item4/`, covered by `SHA256SUMS.repair_02_item4`.

## 0. Summary

- **HEAD = `2843a59a16fabb76a8e55cabdb2133b0a3c0ec5f`**, one commit on R02 on `codex/piping-t3-b1-p-20261007` in `WT/b1-p`, ending with the agent trailer. Not pushed: ROOT pushes. Its diff is byte-identical to the held patch (`357098ba…4d54682`, REPAIR_02 `item4/item4_held.diff`).
- **The census at HEAD, 339 × 3 verdicts against R01: exactly the nine ruled changes.** Each is transport, G7 → G2, with the same code and detail, and each equals RS at `6e3e4fe219` on gate and code. There are **0 other changes** (input, bound, unbound or transport) and **0 misses** against the corpus. My own census agrees: 9 of 1,017 outcomes differ, the same nine.
- **Suites against R01: 1,937 → 1,944 passed, 30 skipped, rc 0.** 7 tests were added (REPAIR_02's five, and item 4's new test in its two modes), 0 removed and 0 changed in outcome. Against R02's run, the difference is exactly item 4's two. The four retained-reader files pass in full: 551 of 551.
- **Mutants: all three item-4 mutants are killed by assertions.** They are: the header skipped, the G7 label kept, and the metadata check at G2. N0 passes 470 of 470. PRE (the head tests on R02's reader) fails exactly item 4's four test runs.

## 1. The change (`2843a59a16`)

The held patch is applied unchanged (REPAIR_02 §5). `_transport_g7` becomes `_transport_base`. It still runs `_source_contract(projection, check_receipt=False)` on the transport projection: the base header check, then the preview-physics transport metadata check.
- At G7: `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, which only the metadata check raises, and the no-code fallback `SOURCE_PREVIEW_PHYSICS_INVALID`.
- At **G2**: every other code, which is the header's, as Rust reports it (RV113 N-1).
- Raw reads are unchanged: both checks are still reported at G7 there.
- The transport docstring states the post-item-4 gates.

Tests (`tests/test_retained_precision_carriers.py`):
- the RV108 N1 retained-reader test's transport row expects G2;
- the new `test_repair02_transport_reports_the_header_at_g2_and_the_metadata_at_g7` checks, on transport, the header at G2 (status, case and formulation defects) and the metadata at G7 (a malformed `combination_gates`, and limitations that differ from the profile's). On raw reads, both stay at G7.

Diff: `retained_precision.py` +14 −10; the carriers test file +40 −5 (`diff/`). R01..HEAD in total: `retained_precision.py` +58 −13, the carriers test +40 −5, the contract test +247 −7.

## 2. The census over 07m at HEAD (`census/`)

I ran RV113's harness, as published, with my census, as one `t3_slot.sh` job (`scripts/job_item4.sh`, which is `job_r02.sh` with the slot wrapper). It ran on `S/head6`, the archive of HEAD without `execution/_Coordination`, with my own builds.

- **RV113's harness: 339 entries × 3 verdicts, 9 changes against R01, 0 misses** against the corpus's Python expectations (`compare.txt`, `probes/compare.json`).
- **`scripts/item4_nine.py`** (`nine.txt`, `nine.json`) stops unless the only changes are those nine transport verdicts, each G7 → G2 with the same code and detail and each equal to RS's gate and code. Result: **OK, other changes 0.**

| 07m entry | id | transport at R01 | transport at HEAD | RS at `6e3e4fe219` |
|---|---|---|---|---|
| 277 | `g7_not_required_quality_enum_invalid` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 286 | `g7_selected_quality_enum_invalid` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 287 | `g7_unavailable_quality_enum_invalid` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 288 | `g7_quality_case_evidence_ref_empty` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 289 | `g7_quality_case_extra_member` | G7 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` | G2 `SOURCE_NUMERICAL_CASE_INVALID` |
| 290 | `g7_quality_status_invalid` | G7 `SOURCE_NUMERICAL_QUALITY_INVALID` | G2 `SOURCE_NUMERICAL_QUALITY_INVALID` | G2 `SOURCE_NUMERICAL_QUALITY_INVALID` |
| 291 | `g7_formulation_limitations_empty` | G7 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` | G2 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` | G2 `SOURCE_FORMULATION_BASIS_UNSUPPORTED` |
| 292 | `g7_contract_evidence_null` | G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` | G2 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` | G2 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED` |
| 293 | `g7_source_block_recovery_present` | G7 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` | G2 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` | G2 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` |

- **Each row's detail is unchanged:** in every row PY's detail equals its code, as at R01.
- **RS's detail text differs from PY's.** The ruling and the comparison are on gate and code.
- **Transport against RS, all of 07m, gate and code:** 330 of 339 equal at R01, **339 of 339 at HEAD**.
- **My census** (`census_i91.json`, `compare_i91_census.txt`) gives 9 of 1,017 outcomes differing from R01's run: the same nine, on transport.
- **Same outputs as the patched-tree run.** The census and probe outputs are byte-identical to REPAIR_02's run with the held patch applied (`notes.txt`).
- **Inputs used.** R01's census is RV113's published `py_head.jsonl`. RS's is I90's `_run_records/repair_02/out/census_head.jsonl` (sha256 `e33aed6b…47d4d3`, as in I90's `SHA256SUMS.repair_02`). The two harnesses hash mutated inputs differently, so entries are matched by corpus position and id (`notes.txt`). PY's input hashes equal R01's on all 339.

**RV113's probes at HEAD** (`probes/`, against PY at R01, RS and TS as RV113 ran them):
- **Bound and unbound:** as in REPAIR_02 §8.
- **v5 transport: 10 change, each G7 → G2 with the same code.** Eight are RS's verdict: the six `n6_structural_status_*`, `n6_model_matrix_fidelity_*` and `n6_accuracy_evidence_*` probes, and the two `n6_status_*` probes. The other two are the declared N6 classes: `n6_carrier_evidence_with_case_defect` and `n6_contract_evidence_null_and_source_block_recovery`. On those, each reader's first header refusal is its own code (RS's I90 REPAIR_02 §2 records the same pair). Transport now equals RS on 101 of 103 (93 at R01).
- **fg and rp transport:** unchanged, and equal in all three readers.

## 3. Suites against R01 (`suites/`)

I83's Python set (27 files) ran as one `t3_slot.sh` job (`scripts/suites_item4.sh`, which is `suites.sh` with the slot wrapper). It ran on `S/head6` with my own builds.
- **1,944 passed, 30 skipped, rc 0** (`py_r02_item4.*`; the junit's `hostname` attribute removed).
- **Against R01's run (`repair_01/suites/py_r01b`): +7, 0 removed, 0 changed in outcome** (`py_compare_r02_item4.*`):
  - REPAIR_02's five;
  - `test_repair02_transport_reports_the_header_at_g2_and_the_metadata_at_g7`, in each of its two modes.
- **Against R02's run (REPAIR_02 `suites/py_r02`): +2 (item 4's), 0 removed, 0 changed** (`py_compare_r02_r02_item4.*`). The RV108 N1 test's transport row changed expectation and passes under the same id.
- **The four retained-reader files**, within this run (`retained_reader_files.txt`): the contract 418, the carriers 52, the schema 58 and the dispatcher 23, **551 passed, 0 failed.**

## 4. Mutants (`mutants/`)

`scripts/mutants_item4.py` is `mutants_r02.py` with the item-4 mutants. Each makes one asserted-unique textual edit to `retained_precision.py` on `S/mut6`, a copy of `S/head6`, and restores it byte for byte (`mut6` equals `head6` afterwards).
- **Test scope:** each mutant runs the B1, repair, N1 and item-4 tests of both retained files (`-k "b1_ or rv108_n1 or repair02_transport"`, 22 tests). N0 and PRE run both files in full (470 tests).
- **How the runs were held:** each pytest was its own `t3_slot.sh` job.
- **Kill evidence:** `kinds.txt` reads each junit report.

| Mutant | Edit | Killed by (failure) |
|---|---|---|
| T-header-skipped | the base header check skipped: the transport step runs only the preview-physics metadata check | the N1 test's transport row and the item-4 test, both modes (4): `pytest.raises` fails, DID NOT RAISE |
| T-g7-label-kept | every base-step failure keeps the G7 label (the pre-item-4 gate) | the N1 test's transport row and the item-4 test, both modes (4): `AssertionError` |
| T-metadata-at-g2 | the metadata check's code reported at G2 with the header's | the item-4 test's metadata assertion, both modes (2): `AssertionError` |

- **No kill is an error or a crash.**
- **N0:** 470 of 470 pass.
- **PRE** (the head tests on R02's reader, a baseline, not a mutant) fails exactly the four item-4 test runs, at their G2 assertions. The other 466 pass.

## 5. Host

- **Cargo:** my three CLI builds were deleted at REPAIR_02's cleanup, so I rebuilt them through `WT/tools/t3_cargo.sh` (`scripts/build_bins.sh`). They are byte-identical to REPAIR_02's (`host/binaries.txt`). HEAD changes no CLI source. Every Python job set the three `OPENPIPESTRESS_*_BIN` variables to them.
- **Heavy jobs, each its own `t3_slot.sh` job, one of mine at a time:** the census-and-probes job, the five mutant-lane runs (N0, three mutants, PRE), and the full suite (`host/lock_log.txt`, 10 starts and 10 ends with the three builds).
- **Waits:** one per job, bounded, each ending when the job's process had gone; none left.
- **Records:** placeholder paths only (each file copied through `scripts/sanitize.py`). The junit XML has its `hostname` attribute removed. There is no symlink and no `build` folder. Every file was screened, gzipped files decompressed, for the four host-path forms, the machine's three host-name forms and any junit `hostname` attribute: 0 hits. This record names none of the host forms literally.
- **Sealed records:** `REPAIR_02.md`, `SHA256SUMS.repair_02`, `_run_records/repair_02/` and every earlier record are unchanged.
- **Not done:** no DEC-025, no install, and no other agent's job touched. The commit is on my branch only. No push.
- **Cleanup:**
  - deleted: `S/head6` and `S/mut6`, `WT/targets/i91-b1-sr-py/` (sha256 in `host/binaries.txt`) and the temporary folders under `S/tmp/`;
  - `WT/b1-p` is clean at HEAD, and no process or wait of mine remains.

## 6. Records

`REPAIR_02_ITEM4.md` and `SHA256SUMS.repair_02_item4` (over `_run_records/repair_02_item4/`).
- `diff/`: `commits.txt`, `diffstat.txt`, `repair_02_item4.diff` (R02..HEAD), `diffstat_r01_to_head.txt`, `equal_to_held_patch.txt`.
- `census/`: `census_rv113.jsonl`, `census_i91.json`, `compare.txt`, `compare_i91_census.txt`, `nine.txt`, `nine.json`, `notes.txt`, the job's `log.txt` and `stamps.txt`.
- `probes/`: `probes_v5.jsonl`, `probes_fg.jsonl`, `probes_rp.jsonl`, `compare.json` (the full report: census and probes).
- `suites/`: `py_r02_item4.xml.gz`, `.tail`, `.rc.txt`; `py_compare_r02_item4.{txt,json}` (R01..HEAD); `py_compare_r02_r02_item4.{txt,json}` (R02..HEAD); `retained_reader_files.txt`.
- `mutants/`: `mutants_item4.jsonl`, `mutants_item4.log`, `kinds.txt`.
- `scripts/`: `mutants_item4.py`, `mutant_kinds.py`, `item4_nine.py`, `job_item4.sh`, `suites_item4.sh`, `r02_compare.py`, `census_compare.py`, `compare_suites.py`, `build_bins.sh`, `sanitize.py`. RV113's harness is used as published (REPAIR_02 `probes/inputs.txt`).
- `host/`: `binaries.txt`, `lock_log.txt`.
