# T1_WAVE1_REVIEW_A return: independent review of the joined readers, schemas and packager

- **Role.** A TASK (Type 2) acting as a fresh-context, non-author reviewer. I wrote none of the bytes I reviewed, and I delegated nothing.
- **Reporting to.** The T1 WORKING_ITEMS manager (`a56f5deb01ca844b0`). ROOT spawned me at the manager's request.
- **Brief.** `LSI/TASK_BRIEFS/T1_WAVE1_REVIEW_A.md`, read with `_T1_COMMON.md`.
- **Paths.** Relative to WORKING_ROOT. `LSI` is this folder's parent, and `RR` is `LSI/T1_WAVE1_REVIEW_A/_run_records`.
- **Candidate.** Commit `bfef71b19` (parent `c1e130818`), plus the reader re-pin in `31dc7ce08`. I reviewed the committed bytes at `6ded3e347`, the commit named in the spawn request.
  - The branch moved to `cad59d01e` during the review (`203396e4d`: docs; `cad59d01e`: VP-STATIC records).
  - Neither commit touches any path reviewed here. Every reviewed path in the working tree is byte-identical to `6ded3e347`, checked with `git diff --stat 6ded3e347 -- <paths>`.
  - Probes and mutants ran on `git archive` copies. The pre-existing base comparison ran on a `git archive` copy of `c1e130818`.
- **Writes and Git.** I wrote only in this folder and made no Git writes.

## 1. Verdict: CLEAR

The D1 gate is met. I found no blocking or should-fix issue.

Across every probe of relabelling, tampering, resealing and binding, neither joined reader accepted an envelope that the joined contract rules out, except for the three inherited and documented limits below. The per-case physical-evidence hash reproduces the producer's rule independently. The standing edit, the one changed expectation, the schemas, the packager edit and all 30 carriers are as ruled.

There are three non-blocking notes. None of them affects admission of a producer-valid envelope or numerical standing. They are optional hardening and record-keeping.

## 2. Findings

| # | Severity | Location | Concrete failure scenario | Suggested repair |
|---|---|---|---|---|
| N-1 | Note (parity record) | J2 R3/R4/R7 hash calls: `load_reference_source.rs` `receipt()` → `source_blocks::domain_hash`; `load_reference_source.py` `_receipt()` → `source_blocks.domain_hash` | An integral JSON literal above 2^53−1 anywhere in the hashed publication (probe C01: extra top-level key; C10: `results[0].value = 2^60`; C11: a key in a diagnostic) reaches R4. Both readers refuse. The strings differ, though: Rust gives `CHECKED-JSON-UNSAFE-INTEGER: 1152921504606846976` and Python gives `CHECKED-JSON-UNSAFE-INTEGER`. The difference is inherited: the physics-source-1 dispatch shows the same divergence (probe PS-C10). But the joined parity record (`T1_WP1_JOINED_READERS/_run_records/parity_summary.json`: 131 cases, "0 undeclared") has no case for it. | Add one shared case to `load_reference_source_mutations.json` and declare it language-specific, as for `NUM-N2-*`. Alternatively, map a canonical-hash failure to one fixed `SOURCE_LOAD_REFERENCE_*` code in both readers. |
| N-2 | Note (inherited limit) | S13 in `load_reference::prepass` / `_prepass`, joined method | A selected case that also carries an info `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming that case is accepted in both languages after a full reseal (probe B04). The producer never emits this: an unavailable attempt is never selected. S13 constrains the NOT_JOINED and SELECTED diagnostics, but not UNAVAILABLE. Physics-source-1 accepts the same edit (probe PS-B04). Standing stays `needs_recompute`, so there is no numerical-use consequence. | Optional: in S13 (joined), refuse any `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` whose `affected_refs` names a selected case, in both languages. Otherwise record it as an accepted limit alongside the physics-source-1 one. |
| N-3 | Note (implicit dependency) | R7 in both readers (`case_pressure(pressure, id)` for every case) vs producer `composite.rs` (`case_physical_evidence(evidence, &[], load_record)` for an exact case) | For a selected case, the producer hashes `pressure: []`, but the readers hash that case's pressure slice. The two agree only because J3 (physics-source-1 `SOURCE_PRESSURE_INVENTORY`) later refuses a selected case with a non-empty inventory. Probe B03 moves the pressure region onto the selected case and reseals with the readers' formula. It passes R7 and is refused at J3 in both languages. The physics-source-1 reader has the same structure. | Add a one-line comment at R7 in both readers saying that the equivalence with the producer's `[]` relies on the J3 inventory check. No code change is needed. |

**Pre-existing parity divergences seen by my probes, outside T1.** Each reproduces with identical outcomes at `c1e130818` (`RR/probe_outcomes/py_base.jsonl`, `rs_base.jsonl`):

- A01 and A10: an envelope carrying source-blocks-1 metadata plus `contract_evidence`. Python gives `SOURCE_PHYSICS_CONTRACT_MISMATCH`. Rust gives `SOURCE_BLOCKS_RAW_FIELDS` for A01 and `SOURCE_FORMULATION_BASIS_UNSUPPORTED` for A10. This is source-blocks dispatch order, which T0R has reserved.
- A05: the inherited physics-1 detail string under the load-reference-1 wrapper (`SOURCE_PHYSICS_EXTREMA_SHAPE` in Rust vs `SOURCE_PHYSICS_EVIDENCE_INVALID: extrema shape` in Python).

Both readers refuse all three. I report them for information only.

## 3. What I verified, and how

### 3.1 The joined readers (item 1)

**Can either reader admit an envelope the joined contract rules out?** I read the Rust and Python readers line by line against CP2_WIRE_ADDENDUM_2 §5, CP4 and rulings §1, §2, §4, §6, §7 and §8, then probed.

- **Generating and running the probes.** `RR/scripts/make_probes.py` builds the probes from committed raws. Its reseal uses my own JCS and sha256 implementation (`RR/scripts/indep_hash.py`), not reader code. Both readers ran through my runners: `RR/scripts/probe_runner.py` for Python, and `RR/scripts/zz_review_probe.rs`, a test added only to the scratch copy, for Rust. Each runner records dispatch (`for_source` / `_source_contract`), the direct validator, the transport validator and numerical standing.
- **Scale.** 30 joined probes, 2 physics-source-1 analogs and 3 unsafe-integer probes, giving 143 language-paired outcomes (`RR/probe_outcomes/probe_parity.txt`).
- **Relabels in every direction, raw and resealed:**
  - joined → source-blocks-1;
  - joined → physics-source-1 with records stripped and physics-domain hashes;
  - physics-source-1 → joined, with grafted records and a joined-domain reseal;
  - source-blocks-1 → joined;
  - a full joined → load-reference-1 "downgrade" forgery (receipt removed, records and diagnostics converted to not-joined);
  - joined with physics-domain case hashes;
  - joined with policy `PHYSICS-SOURCE-1`, fully resealed;
  - the joined profile under the load-reference-1, physics-source-1 and source-blocks-1 IDs.

  Every one is refused, in both languages.

  These probes add to the author's 131 shared cases, which already cover relabel pairs without a reseal.
- **Contract facts the producer cannot emit, fully resealed.** All are refused in both raw readers:
  - B01: a selected case whose ordinary attempt passed its checks → `PHYSICS_SOURCE_SOURCE_FALLBACK_TRIGGER`;
  - B02: an ordinary case that is sensitive → `SOURCE_BLOCKS_ORDINARY_SELECTION`;
  - B02b: only its numerical-quality label changed → `SOURCE_BLOCKS_ORDINARY_REPORT`;
  - B03: a pressure region on the selected case → `SOURCE_PRESSURE_INVENTORY`;
  - B05: an extra NOT_JOINED diagnostic → `NOT_JOINED_DIAGNOSTIC`;
  - B06: the ordinary case promoted to selected → `PHYSICS_SOURCE_PHYSICAL_SHAPE`;
  - malformed receipts (null, array, body with only a policy, `selected_method` null) and a boolean record ID.
- **Accepted after a reseal, as expected.** Consistent edits the reader cannot re-derive are accepted: B07 changes a prescribed motion together with its contribution, and B08 changes record provenance. This is the documented limit. Hash consistency is not custody, and standing stays `needs_recompute`. B04 is note N-2.
- **Transport.** The transport validator accepts B01, B02, B02b and B05, because transport carries no rows or diagnostics. This is the inherited physics-source-1 transport scope. Transport never qualifies a publication. It is a limit, not a finding.

**Do the two languages agree in order and code?**

- In all joined-reader outcomes of my probes, they agree exactly, apart from the inherited detail string in N-1.
- The author's shared file (111 raw, 6 table and 14 transport cases) passes in both languages, in my `cargo test` run and my pytest run.
- I compared the Rust and Python sources hunk by hunk: J0 → J1 (S1–S13) → J2 (R1–R7) → J3, with the same code names. The type-sensitive comparisons (`==` in Rust against `_same`/`_eq`/`==` in Python) meet only values that were already validated as strings.

**Independent physical-evidence hash.** `RR/scripts/indep_hash.py` reimplements RFC 8785 JCS. Its self-test reproduces both `hash_vectors` of the joined table.

- The rule comes from the producer source: `source_receipt.rs` `hash` and `case_evidence_domain`, and `composite.rs` `case_physical_evidence`, with `pressure: []` for an exact case.
- It recomputes `physical_evidence_sha256` for all 10 committed joined raws, 12 cases in all. These include the mixed witness's selected and ordinary cases.
- It also recomputes the receipt hash and the publication hash.
- All match (`RR/logs/indep_hash.log`).
- Two controls, the physics domain and the record omitted, do not match.

### 3.2 The standing edit (item 2)

- **Rust.** The diff of `semantic_contract.rs` against `c1e130818` has exactly one hunk inside any standing function. It is the early `return "needs_recompute"` in `numerical_use_standing_with_context`, placed immediately after `let Ok((_, version)) = for_source(source) else { return "unsupported"; };`.
- **Python.** The diff of `compatibility.py` has exactly one hunk in `numerical_use_standing`: the early return immediately after the `_source_contract` try block.
- **The other hunks** in both files are the enumeration, dispatch, table and AnalysisRun registration described in the WP1 return §4. Nothing else in the Current, rule or export standing paths changed. Downstream eligibility (the headless `result_envelope_binding.rs` and the desktop) goes through `numerical_use_standing_with_context`, so joined results stay `needs_recompute` there too.
- **Placement is pinned.** Mutants RS- and PY-STANDING-BEFORE-VALIDATION move the return before validation, and both are killed.
- **The claimed fall-through equivalence holds.** Mutants RS- and PY-STANDING-EARLY-RETURN-REMOVED survive: an equivalent mutant, as ruled.

**Pre-existing outputs, rerun independently:**

- **Rust.** All 204 `RESULTS_RUST_CONTRACT_OUTPUT_DIR` files are byte-identical between `c1e130818` (base archive) and the candidate, and equal to the author's recorded after-hashes (`RR/logs/rust_out_{base,cand}.sha256`). Base passes 64 tests and the candidate 69.
- **Python.** I widened the author's digest to 50 pre-existing raws (`RR/scripts/digest_wide.py`). It adds the source-blocks-1 raws, `fixtures/product_preview/*.json` and the result_export test raws. All 50 entries are identical between base and candidate (`RR/logs/py_digest_{base,cand}.json`), covering contract, standing, transport, AnalysisRun digest and stress-neutral digest. The only extra candidate entries are the two new fallback raws.

### 3.3 The changed expectation `TABLE-id-reserved-source-successor` (item 3)

- A parsed comparison of `load_reference_mutations.json` against `c1e130818` finds exactly one changed case. Only its `dispatch` changes, from `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` to `SOURCE_FORMULATION_BASIS_UNSUPPORTED`, and a `note` is added. `validator` stays `accept`, per ruling §8.
- The new code is the direct D1 consequence. The ID now passes the producer enumeration and fails the exclusive-profile check, identically in both languages.
- No pre-existing Rust or Python test file changed apart from the two ruled schema pins (§3.4).

### 3.4 Schemas (item 4)

- **Preservation.** My own walk compares base and candidate (`RR/scripts/schema_preservation.py`, `RR/logs/schema_preservation.log`):
  - results.v0.3: 6 list appends and 5 new `$defs`;
  - analysis_run.v0.3: 5 list appends and the one ruled wrap. `properties.contract_evidence` becomes `anyOf`, and `anyOf[0]` equals the base `$ref`;
  - stress_neutral_export.v0.3: 5 appends;
  - no other change.

  `analysis_run.schema.json`, `load_reference_state.schema.json` and `physics_source_recovery.schema.json` are unchanged.
- **Tightness.** Every results, AnalysisRun and stress-neutral branch either pins or forbids `contract_evidence` and `source_block_recovery`, so the widened top-level `anyOf`/`oneOf` weakens no older branch.
  - Branch 5 differs from the physics-source-1 branch 3 at exactly five pointers: the ID, profile, `semantic_contract_ref` and the two `$ref`s.
  - The `LoadReferenceSource*` definitions are closed (`additionalProperties: false`): the receipt requires the policy const `LOAD-REFERENCE-SOURCE-1`, and the evidence namespace is exactly four keys with `connector: []`.
  - The record pairs selected with retained in a `oneOf`, and each exact case requires `recovery_method`.
- **Cross-carrier `$ref`.** With my own `referencing` registry (`RR/scripts/validate_carriers.py`), all 30 joined carriers validate and match exactly one branch, index 5 (`RR/logs/carriers.log`).
  - The 12 pre-existing load-reference-1 carriers keep branch 4 under both the base and candidate schemas.
  - Relabelling any joined carrier to load-reference-1 or physics-source-1 fails validation (0 of 60).
- **The two pin edits weaken nothing.** In `test_load_reference_schema.py`, `anyOf[-1]` / `oneOf[-1]` become fixed indices plus exact lengths, and the counts go from 5 to 6. The same positions stay pinned, and the old "LR is last" now reads "LR at 4, joined after it". In `test_source_block_schema_contract.py`, `index == 3` becomes `index in (3, 5)`, and branches 0–4 stay exact.

### 3.5 Packager edit and packages (item 5)

- `package_v0_3.py` adds the joined ID to exactly `SUPPORTED_METHODS`, `PHYSICAL_METHODS` and `RECEIPT_METHODS`, plus the import. Every other method test in the file is `== / != PRECISION_CONTRACT_ID` or set membership, so the joined ID takes the physics-source-1 UTF-8 path.
- `LSI/_run_records/session4/t1_joined_stress_neutral_outputs.py <WORKING_ROOT> --check` passes. All 10 packages reproduce byte for byte, and the output equals the recorded check log (`RR/logs/sn_check.log`).

### 3.6 Carriers (item 6)

- The 10 documents are asserted byte for byte by Rust `canonical_documents_…` and pass (`RR/logs/cargo_test_worktree.log`).
- The 10 AnalysisRun records are asserted byte for byte by Python `test_analysis_run_carrier_…` and pass (`RR/logs/python_suites.log`).
- All 20 validate on the joined branch (§3.4).

### 3.7 Reader re-pin (`31dc7ce08`)

`tools/validation/qualification_load_reference.py` pins `MODULE_SHA256 = eff1fb3b…c08ffe`. That equals the sha256 of the committed `core/analysis_runs/load_reference_evidence.py`; the pre-WP1 value was `f7d50f1c…`. The rest of that file is REVIEW_B's scope.

## 4. Checks (commands and counts)

| Check | Result | Log |
|---|---|---|
| `cargo +1.97.1 test --locked --offline -j 2` (result_export, candidate) | 69 passed (22 unit, 6, 5, 5, 10, 7, 4, 10) | `RR/logs/cargo_test_worktree.log`, `cargo_cand.log` |
| The same on `c1e130818` | 64 passed | `RR/logs/cargo_base.log` |
| pytest, the 12 files of `session4/t1_wp1_integration_python.log`, `PYTHONDONTWRITEBYTECODE=1 -p no:cacheprovider` | **1521 passed, 16 skipped** | `RR/logs/python_suites.log` |
| Stress-neutral `--check` | 10/10 reproduce | `RR/logs/sn_check.log` |
| Independent hashes | 12/12 physical, 10/10 receipt, 10/10 publication | `RR/logs/indep_hash.log` |
| Carriers | 30/30 on branch 5; 12/12 load-reference-1 carriers keep branch 4; 0/60 relabels validate | `RR/logs/carriers.log` |
| Pre-existing outputs | Rust 204/204 and Python 50/50 identical, base vs candidate | `RR/logs/` |
| Probes | 143 paired outcomes; joined-reader differences only as in N-1 | `RR/probe_outcomes/` |

The helpers `OPENPIPESTRESS_CHECKED_JSON_BIN` and `OPENPIPESTRESS_UNITS_BIN` pointed at the existing builds under `core/serialization/canonical_json/target` and `core/units/target`.

## 5. Mutation evidence (scratch `git archive` copy of `6ded3e347`)

`RR/scripts/mutants.py` requires each anchor to occur exactly once. It applies one mutant at a time and restores the file, checking its sha256 each time; all restores verified (`RR/mutants/mutation_prehash.txt`).

- Rust runs `--test load_reference_source_contract --test derivative_contract --test load_reference_contract`.
- Python runs `tests/test_load_reference_source_readers.py -x`.
- These mutants target checks that are not among the author's 50.

| Mutant | Result |
|---|---|
| RS- / PY-R7-WHOLE-PRESSURE (the case proof uses all regions) | killed / killed |
| RS- / PY-STANDING-BEFORE-VALIDATION | killed / killed |
| RS- / PY-STANDING-EARLY-RETURN-REMOVED | survived / survived: equivalent to the fall-through, as ruled |
| RS-PROJECT-KEEPS-RECORDS, RS-PROJECT-STALE-PHYSICAL | killed, killed |
| RS- / PY-J0-PROFILE-CLAUSE | killed / killed |
| RS- / PY-S10B-ANY-TO-ALL | killed / killed |
| RS- / PY-S13-SELECTED-ID-DASHED (selected diagnostic ID with `:`→`-`) | killed / killed |
| RS-DERIVATIVE-EVIDENCE-COPY, PY-ANALYSIS-RUN-EVIDENCE-COPY | killed, killed |

Result: 14 of 16 killed. The 2 survivors are the same equivalent mutant, one in each language. Logs are in `RR/mutants/`.

## 6. Not done, and limits

- I did not run the headless (64) or desktop suites. The headless metadata gate (`core/runner/headless/src/lib.rs` `for_source_metadata`) now admits joined metadata, as the WP1 return §9.6 declared. `T1_WIRE_ADDENDUM.md` §1 does not list it among the downstream consumers.
- I read the producer only for the hash, receipt and diagnostic-ID rules. I made no producer-correctness claim.
- My probes are targeted, not exhaustive. Resealed, internally consistent edits the reader cannot re-derive are accepted by design (B07, B08).
- My mutants are targeted (16), and add to the author's 50.
- I did not review `T1_WIRE_ADDENDUM.md` (`203396e4d`) as part of the candidate. I read it once, and it agrees with what I verified.
- The scratch copies (`git archive`) and my cargo target were deleted at the end. Absolute machine paths in the copied logs are replaced by placeholders (`<WORKING_ROOT>`, `<CARGO_TARGET_DIR>`, `<SCRATCH>`, `<VENV>`).

## 7. Files written

The only files I wrote are new ones under `LSI/T1_WAVE1_REVIEW_A/`: this `RETURN.md` and `_run_records/{scripts,logs,probe_outcomes,mutants}/`. I changed no repository file outside this folder. For reference, sha256 of the reviewed files at `6ded3e347`:

```
372dc950140e4a326f15390172c3cb83478a541601976d76fa4033f5bc703f9d  core/reporting/result_export/src/load_reference_source.rs
650ae942176a9969a9e6bf0d2631a51d281125ead08c0cd33da12fd4b828d8b5  core/reporting/result_export/src/load_reference.rs
756835606454a1f75cecbeeb870da7a9ab63baa012c67d6a6db982f6ce6c89c9  core/reporting/result_export/src/semantic_contract.rs
06a02c904b347597024c943cac8f0368af7fec3afffdc3eb3ca357c20c05ce5b  core/reporting/result_export/src/derivative.rs
212791d876f30a835078571a772fe868873dc90088ce4a16d35790ebcd7297b2  core/reporting/result_export/src/lib.rs
94e26a141e09cd1e3edf5b524246ed743597824f28889803bc9568a9c84ffdc7  core/analysis_runs/load_reference_source.py
eff1fb3b8b7f8fee99aeb5344c55ec42ae37d3e60e7354908caa780d41c08ffe  core/analysis_runs/load_reference_evidence.py
c71cf01df995a92d746e41c1a6ae576b5002c76cc568ec61611505ae2684db33  core/analysis_runs/compatibility.py
75e90a212b3e9e131ac6e56c92d0103f8c72b4530f1c04b62baeeb4ca658927b  core/handoff/stress_neutral/package_v0_3.py
633c21bd82d30dc5b88f29e0e814862fd2c0709a5ccd0c9567d59ca1b5975fea  schemas/results.v0.3.schema.yaml
c3dba5510f82d20b32cbae8999da8e10469ad1d9f0e6fafb6adaf805cebf2781  schemas/analysis_run.v0.3.schema.json
7c8c4edd00a07ec249c4cf9d17bfa0b52379cf403eb6bb8d29cbd72f2ced3556  schemas/stress_neutral_export.v0.3.schema.json
```

## 8. Design questions for the manager

1. N-1: should the joined readers normalise canonical-hash failures to one reader code, or should the language-specific detail be declared in the parity record, as for `NUM-N2-*`?
2. N-2: should S13 (joined) refuse an UNAVAILABLE diagnostic on a selected case? Doing so would make the joined reader stricter than physics-source-1 on the same fact.
