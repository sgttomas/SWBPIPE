# I62 return: checkpoint A (shared schema, table and corpus)

I62 is a TASK (Type 2) dispatched directly by ROOT (HELP_HUMAN). It had no descendants. This return covers checkpoint A only. It ran from 2026-10-03T19:39:19Z to the 19:57:16Z freeze on the M5 host. The memory guard (PID 5387) was running.

No Git writes or index operations were made. No Cargo, install, new tooling, solver or native job ran. I61's worktree was not touched. Paths below use the placeholders WT, NUM, READER, P, T3 and R from the brief. CONTROL is the control checkout that holds VENV.

## Changed files (READER, inside the fence)

| File | Snapshot 03 | Snapshot 04 |
|---|---|---|
| P/schemas/retained_precision_mp_v2.schema.json | 5652929173… | f943ebd3511e31bcdfe09bac2788362d3a5734040bc3088fbe585361b571cd21 (174354 B) |
| P/fixtures/results/retained_precision_cases.json | 67d5cbcc00… | 8e333e632cde3b70cb7fcce13d5f9eb724fbc22246c304a30dbdbefcd0cce760 (686570 B) |

Unchanged, with the reason recorded in SHARED_SNAPSHOT_04.json:
- **Definition** 3e0779a45a. Its numerical contents and domain hash are unchanged, as I57 §5 requires.
- **Preview table** c74742ce6a. It lists no ProofTrace members, and all three readers pin its bytes.
- **results.v0.3.schema.yaml** 4585a45fcf. It only `$ref`s the receipt schema.
- **retained_precision.py and both test files** are byte-identical to SOURCE_FREEZE.

### What changed

- **Schema.** `ProofTrace` gains a required, closed, nullable `summary_coverage`, exactly as I57 §1 defines it.
  - It has no `minItems`, so an empty array fails G3 on cardinality rather than G1 on shape.
- **Corpus, existing cases.** Both complete cases carry `[{body:0, stop:[T,T,T,T], has_data:true}]`, derived and asserted as in I57 §2.
  - Only `receipt_sha256` changed in each. Publication, preparation and source-identity hashes are byte-equal (asserted).
- **Corpus, new case.** One new synthetic positive control, `ordinary_prepared_no_data_synthetic`: the zero-load, no-data body. Its coverage is `[{body:0, stop:[F,F,F,F], has_data:false}]`, and it has no stop, estimate, charge or B entries.
- **Corpus, mutations.** 47 new rehashed first-failure mutations. The 30 existing mutations are byte-identical, and none moves; the reason is in the snapshot.
- **Build method.** I extended I58's own method: edit the JSON, rehash through the test `apply_mutation(rehash="all")` and `rebind_invocation`, which call the reader's `_hash`, `_source_hash` and `_preparation_payload`, then serialize with `json.dumps(indent=2)+"\n"`.
  - Script: WT/scratch/i62_coverage_shared_python_01/build_corpus_04.py.

## Mutations: expected and observed first failures

The full table is in SHARED_SNAPSHOT_04.json `new_mutations`: id, base, expected gate and code, I57 row, observed pre-B Python result, and the jsonschema verdict. The counts by expected first failure:

| Expected | Count |
|---|---|
| G1 RECEIPT_MISMATCH | 14 |
| G2 ENCODING_MISMATCH | 4 |
| G3 COVERAGE_MISMATCH | 6 (one is the existing owner check) |
| G5 PRODUCT_ATTEMPT_MISMATCH | 2 |
| G5 ATTEMPT_MISMATCH | 1 (ordering pin) |
| G5a SCALE_MISMATCH | 20 |

- **Observed, current draft reader:** 34 of the 47 already produce the expected first failure.
- **The 13 that don't yet** need the checkpoint B checks: G3 coverage cardinality, G5 null-on-Ready, and the G5a exact rosters and data rules. They are:
  - coverage_empty_array, coverage_duplicate_body, coverage_foreign_body, coverage_extra_body;
  - coverage_duplicate_then_flag_and_method, coverage_null_on_ready, coverage_null_then_method;
  - coverage_has_data_false, coverage_drop_certified_bound, coverage_no_data_claim_with_free_loads;
  - coverage_stop_forbidden_entry, coverage_flag_defect_then_method, coverage_flag_defect_then_body_scale.
- **Six no-data mutations match only by accident.** The draft's Cartesian roster fails them first. Checkpoint B must make them fail for the I57 reason.

## Test commands

All runs started in READER/P with both OPENPIPESTRESS_*_BIN paths from the brief, `VENV/bin/python -m pytest -q -rA tests/test_retained_precision_contract.py tests/test_retained_precision_schema.py`, and a 1,200 s subprocess wall. Records are python_schema_A1–A4.json/.log in scratch.

| Run | Corpus state | Result |
|---|---|---|
| A1 | before the no-data case | 93 collected: 80 passed, 13 failed |
| A2 | with the no-data case | 99 collected: 85 passed, 14 failed |
| A3 | with the below-p512 charge control | 100 collected: 86 passed, 14 failed |
| A4 (final, same bytes as A3 apart from one qualification string) | final | 100 collected: 86 passed, 14 failed |

- **Schema tests:** all 8 pass, including jsonschema validation of all three receipts.
- **The 53 pre-existing tests** all still pass for the two existing cases.
- **Failures appear only where the reader meets the new field:**
  - the 13 mutations listed above;
  - `test_complete_synthetic_draft_control_is_not_qualification`, on the no-data case. That case meets the draft Cartesian G5a roster. With the Cartesian roster substituted, it passes G0–G8 with exactly the expected classifications (observe_04.json).

## Deviations

1. **One control omitted.** I dropped the unsafe-U body control (2^53). The checked canonical JSON refuses unsafe integers, so that receipt cannot be rehashed, and its first failure would be ambiguous.
2. **One ordering pin added.** I added `certified_bound_unbound_drop_existing_g5`. Changing B in the selection alone first meets the existing G5 selection/verification-record binding. The B controls at G5a therefore rebind the record.
3. **Part of the §5 table is not expressible on single-body p128 bases.** These need new synthetic base cases:
   - multi-body;
   - absent kind or L=0;
   - a faithful p512 escalation ladder;
   - a second owner or proof;
   - failed-prefix and unavailable attempts.
   - A fresh-first p512 prototype passed the draft but was discarded: the native ladder always starts at p128 (adaptive.rs:4519–4521).

   The snapshot lists each one. Publicly undetectable consistent rewrites are also listed there; they are deliberately not controls.

## Open items for ROOT

- **Rule on whether snapshot 04 is complete enough to release I63/I64,** or whether the missing §5 bases (deviation 3) must come first. If they must, they need a new grant and time: each needs native-faithful synthetic records.
- **Checkpoint B:** the Python G1/G2/G3/G5/G5a coverage checks, replacing the Cartesian roster.

## Files read (sha256)

NUM is at 3fc80230c0.

| sha256 | File |
|---|---|
| c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd | NUM/AGENTS.md (the CONTROL copy is identical) |
| 1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 | NUM/agents/AGENT_TASK.md |
| d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792 | NUM/P/AGENTS.md |
| 84f0e3f9498152cc678bc5afd29a13acacff3ba438dda923ab00944c932dda80 | R/BRIEFS/I62_COVERAGE_SHARED_PYTHON.md |
| 845d5258bf6ec61734242e8be958bbf544a13af3c6d6d8871d7f536f0b44ad59 | R/I57/summary_coverage_01/ADDENDUM.md |
| 10341323b974d7a24155f23013d16b43e803f1d3cb6638df052a6ce2f51a61f3 | R/REVIEW_RV76/summary_coverage_01/REVIEW.md |
| b66b99a2ae70b847342e5bd5df4e544b651b2c57369567593c0c6347f2597699 | T3/ROOT_RULINGS_V1.md (the two named sections) |
| 7f418873e6b40c4532293d92e4ce3e10c626f73d23d460f03dce8eea07c6ccb4 | R/I58/python_shared_01/RETURN.md |
| 506bb5373e73b9ae074e8b472f4a96c61c3aa07b00d0c55ce4ecb5e62c0ef503 | R/I58/python_shared_01/SHARED_SNAPSHOT_03.json |
| ee1c8060c2d5453bdfceb94153b4d5b6f6c716750cea0366b057d0c537950f3c | R/I58/python_shared_01/SOURCE_FREEZE.json |
| 293daf236f33b505a2df5ff848e7683b986aa82a5782201952b84b822f9ce8e3 | R/I58/python_shared_01/CHECKS.json (head) |
| b4fcdbade5a164285de81c48e801340899f64b4cfbe5f90afd76415020c08901 | R/I58/python_shared_01/BULK_MANIFEST.json (head) |
| 7b446dcc17e9f7b6aca10687d4f25f274f2a77893ee27014dcb1538c141ecbac | R/I58/python_shared_01/python_schema_03.json |
| fd00d2c1e8f4f7e2077f304560a63830e2b7b61cb5b1d47aff994c7874ed292e | R/I52/prepared_public_contract_02/C3_DELTA.md (ProductAttempt/ProofTrace/hash-scope section) |
| 6a2fc382bf8cae0502c41030da8ac9bc0cfe1f1b80b7aceac60ff7e40f345eda | NUM/P/core/solver/frame_kernel/src/structural/retained/adaptive.rs (ladder lines only) |
| 0583473988e883dfea8d37cbbc563a8688975822a42a79d5dac053e2c770f794 | READER/P/core/analysis_runs/preview_physics_evidence.py (maxima/headline section) |

I also read the READER fenced files at their snapshot-03 hashes.

These contract files were not opened: the C1 WIRE_CONTRACT, the C2 CONTRACT_DELTA, DEFINITION.json, the correction-03 ADDENDUM, and the 06/07/08 addenda. Their rules were relied on only through the existing corpus mutations and reader gate codes.

Bulk files are listed in the snapshot with path, sha256 and size: logs, scripts, observation, diff, the frozen snapshot copies and the pre-edit copies.
