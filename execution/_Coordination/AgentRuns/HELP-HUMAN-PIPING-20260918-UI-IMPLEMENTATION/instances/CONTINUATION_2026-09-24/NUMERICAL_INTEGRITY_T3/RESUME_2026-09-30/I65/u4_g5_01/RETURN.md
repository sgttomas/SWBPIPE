# I65 U4 G5: return at a natural boundary (part 1 of 2)

**Status.** Part 1 is complete and every control passes. Part 2 has not started; I continue on ROOT's word.
- **Why I stopped here.** The candidate diff is about 2,600 lines, past the brief's 1,500. About 1,180 of them are new tests and the D-6 files.
- **What part 1 implements:**
  - G-A `admit`: the census extensions, D1.0–D1.11 in order, Headless refused, the D-6 build status, and the bound comparison;
  - D-6: `build.rs`, the one encoder, the reviewed-input record, the layout witnesses and the reader layouts;
  - G-B and G-C: the facts, `PhaseFact` and `PhaseRefusal { gate, fact, observed, cap }`;
  - R with its test override;
  - the structural budgets;
  - the two struct-construction edits.
- **The permit stays unreachable.** `REGISTERED_PROFILES` is empty, and the cap-priced maximum is `Unpriced` until part 2, so the law fails closed twice over.

**Where:**
- **Code:** WT/f2a-memory, branch `codex/piping-f2a-memory-20261004`, from `8abb5274a9`, uncommitted.
- **Records:** this folder:
  - IMPLEMENTATION.md, the code-to-derivation map;
  - R4_CALLGRAPH.md;
  - `_run_records/`, with `candidate.diff`, `changed_files_sha256.txt`, the sweeps, the test outcomes, the mutants and the R-4 outputs.

## Controls

| Control | Result |
|---|---|
| 1. No published byte changes | The fixture sweep is **byte-identical to base `8abb5274a9`**: 36 inputs × 9 route-and-mode lines, 324 lines in all, including every public admission-report field. The new report field `law` is private and not serialized |
| 2. Nothing weakened | **PP:** base's outcomes plus 23 new tests, all passing. t13 is the only failure, as at base.<br>**runner/headless:** outcomes identical.<br>No reader, schema, fixture or existing check changed. There is no test permit; a structural test pins the single `CapturePermit` construction, which draws from the empty registry.<br>The lib warnings are unchanged at 8 |
| 3. Mutants | **84 mutants, 83 killed by a test, none by a compile error** (`_run_records/mutants.out.jsonl`). They cover:<br>– every D1 clause and every family fact;<br>– cap and off-by-one comparisons, and the cap-row mapping;<br>– D1.10 and D1.11;<br>– every gate's comparison and fact readers, including the RV87 N-3 maxima;<br>– the bound at M and its overflow;<br>– identity matching, bindings and layout witnesses;<br>– the encoder, SHA-256 and the reviewed-input order;<br>– build.rs's empty-value, key-order and debug-assertions handling;<br>– R, its override and the stack refusal kind;<br>– `admit`'s verdict recording.<br>**The one survivor is equivalent by decision 7:** `build_status` ignoring the bindings cannot be observed while nothing is registered.<br>An earlier run also found `D1.2 raw text ignored` surviving. A pure-function test now kills it |

## API correction (RV87 S-3; brief)

On success, `admit` returns the permit **with** the report (U3 grant 1c, RV85 S3):

```rust
pub(super) fn admit(capture: &CapturedInvocation, request: &LinearStaticPreviewRequest, entry: Entry<'_>)
    -> Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>;
```

This replaces API_G4.md §1's `Result<CapturePermit, RetainedAdmissionReport>`. G5 keeps 1c's signature.

## Decisions for ROOT

1. **Reviewed-lock record mechanism.** build.rs hashes the PP lock and the reader's 13 statics at build time, with a self-contained SHA-256 checked against `sha2`. It emits `OPS_RETAINED_REVIEWED_INPUTS`, and a registered profile compares it byte for byte.
   - BUILD.md said the script reads only environment variables and `rustc -vV`; this extends that to reading those 14 files.
   - The alternative, `include_bytes!` plus runtime hashing, would add about 0.7 MB to every consumer binary.
   - **Accept, or rule otherwise.**
2. **RV87 S-4, G-B capacities.** The hook receives `restrained` and `springs` as slices, and the owning Vecs are lib.rs locals outside the D-5 exception. So G-B reads lengths, and the capacities stay priced in O.
   - A capacity fact needs I61 to pass the Vecs through `prepared_case_source`.
   - **Choose:** that hook change, or the current bound.
3. **The late capture as its own fact (T11.P1).** G-C's `ObservationBytes`, the adapter's capacity tally, includes the late capture after it is made. A separate P1 fact needs the observer to keep its G-B tally, which is a retained_product.rs field outside the fence.
   - **Choose:** that edit, by I61 or as a widened exception, or the combined fact.
4. **R-4's remaining limit** (R4_CALLGRAPH.md §3). The resolver's flow-insensitive typing of rebound names is a stated limit. The true calls it drops at this basis carry no text and close no cycle.
   - The general rule is measured but not adopted: its loose fan-out gives 3.26 GB and an incomplete text run.
   - **Accept as stated, or schedule the adjudication work.**
5. **Part 2: go-ahead.**

## Each routed item

**RV83 R-4** (records): done. See R4_CALLGRAPH.md.
- R4a (`Self`) and R4b (`[`) are as asked. R4c (the unwrap idiom) is found by this record's own audit, and so are two true calls G4 dropped.
- TEXT is unchanged at 2,044,161,940 B, and the recursion inventory is unchanged.
- Reachable functions go from 2,698 to 2,705: RV83's two, plus five without text.
- Limit 5 and the audit labels are corrected.

**RV84's NOTEs** (RR "RV84 confirms its G3 findings in G4"):

| Item | Disposition |
|---|---|
| C-N1 (a) | **Done for the gate:** G-C bounds the longest string by L_PUB = 2,599,962. Part 2 uses it in the hash route |
| C-N1 (b) | T25's receipt diagnostic ids are ≤ 2,330. **Part 2**, in T25's expression |
| C-N2 | the per-term `dof` numbers, about 1.6 MB. **Part 2**, in T25 |
| C-N3 (a) | T19's spawn heap belongs in X1 and W1–W4. **Part 2**, in the phase expressions |
| C-N3 (b) | the reader statics count in every phase. **Part 2** constants; the G6 record states it |
| C-N3 (c) | `RECEIPT_X` was unused in g4_caps.py. Part 2's expressions supersede that script and carry no such term |
| C-N4 | **Done:** G-C's `RetainedErrorTextBytes` reads `error` and `observable_error` (their `Association` Strings). `g5a_error` holds no String |
| C-N5 | 56 headers, not 57. Already in ADDENDUM_L128.md §5's errata |

**RV87** (RR "RV87 on U4 G4"):

| Item | Disposition |
|---|---|
| S-1 | T16's third copy of `run_v` and `selection_v`, 13.25 MB. **Part 2**, in T16's expression |
| S-2 | six result-id copies at the 1,024-B class, up to 9.75 MB per branch. **Part 2**, in the TAV constants. As T08's owner, I re-price the six sites (lib.rs:2728, :2730, :2738, :5292 ×2; preview_physics.rs:194) in the text run before G6 |
| S-3 | **Done** (above) |
| S-4 | **Answered.** For the capacities, see decision 2. The late capture's actual owners are `ProductCapture.source` (`Option<k::PrimitiveSource>`), `case_id`, and the `facts`, `members`, `operational`, `supports` and `terms` Vecs, all allocated through `adapter.reserve`/`copy` (retained_product.rs:1281–1340). See decision 3 |
| S-5 | the l = 128 census constant: **done** (`caps::LOADS`). The test that the identity carries every key in order: **done**. RV85 U1 (optional): **part 2** |
| N-1 (a)–(d) | **Part 2**, in the T16, T18 and BODY expressions |
| N-2 | the reader's `objects`/`located` pushes and the `basis_ref` clone. **Part 2**, in V4–V6 |
| N-3 | **Done:** G-C reads the longest string (≤ L_PUB) and the longest diagnostic id (≤ L_DIAGID = 2,330) |
| N-4 | a lever, not used |
| N-5 | harmless; left as is, and T16's P1 prices it |
| N-6 | X2's receipt proxy is conservative. The wording is corrected in part 2's record |
| N-7 | the TRANSFER_COMPLETION §7 citations: `prepare_case` is at :3106 and `w1_case_id` at :3073. Corrected here |
| N-8 | G5 measures `s(MechanicsEnvelope)` in-build. The budgets already use `size_of::<RetainedPreviewOutput>()` |

## The carry list (NOTES_G4.md §5, items 1–11, and RV85 U1)

| # | Item | Status |
|---|---|---|
| 1 | `rerun-if-changed=src/build_identity.rs` | done |
| 2 | an empty variable is a value | done (`target.env=` on this build) |
| 3 | `collect::<String>` in the text lexicon (RV83 R-N3) | **part 2**, with S-2's text re-pricing |
| 4 | `admit` refuses Headless | done (D1.0) |
| 5 | the D1.10 and D1.11 census readers | done |
| 6 | reader layout witnesses | done (`READER_LAYOUTS`; a mismatch is Stale) |
| 7 | the statics' hashes bound | done (the reviewed-input record) |
| 8 | the reader's deepest schema chain (36) | **part 2**, with the witness tests |
| 9 | `PhaseFact` and the gate facts | done |
| 10 | pin the N1 append | already met by U3 grant 1d at base: `publish`'s `cfg(test)` capacity assertions (lib.rs:3063–3082), exercised under the fault controls by `u3_each_stage_fault_falls_back_to_the_ordinary_bytes`. Nothing new is needed |
| 11 | the admission constant in-build, under the margin rule | **part 2** |
| RV85 U1 | bind the permit to its invocation; check linearity structurally (optional) | **part 2** |

## Part 2: what remains (the estimate is the rest of the 14 h)

1. **The cap-priced constants as named in-build expressions.** This means O, T25, TAV_X and TAV_W, T11–T19 and the phases X1–W5. It uses:
   - `size_of`/`align_of`;
   - a new FK resource module exporting the kernel's strides and bounds, plus an SR stride export if T14 needs one;
   - every RV84 and RV87 correction above;
   - the margin check at in-build strides.

   This fills `cap_priced_maximum`, the `UNPRICED` gate bounds and the byte budgets.
2. **The witness tests W1–W7 at R/16,** through the private driver, including W2's cap-maximal input. `cap_maximal()` already exists in the tests, and is inside D1 in both modes.
3. **The allocation challenge** in an isolated test binary.
4. **Carry items 3 and 8, and RV85 U1.**

## Execution record

- **Who.** TASK I65 (Type 2) under ROOT. No descendants.
- **When.** Granted at about 02:00 MDT; this return is at about 03:05 MDT.
- **Memory guard.** `memguard.sh`, PID 5387, was running, and every cargo run checked it first.
- **This build's identity, reviewed inputs and reader layouts** are printed by the tests to `_run_records/build_record_this_build.txt`, for G6's registration.
- **Cargo.**
  - The default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`.
  - One cargo job at a time, with targets in `WT/targets/i65-g5/`.
  - **Runs:** base (sweep, PP and runner); the candidate twice (sweep, PP and runner); the mutants.
- **Not run.** No Git writes, installs, new tooling, native, solver-at-scale or DEC-025 jobs.
- **Scratch.** WT/scratch/i65_u4_g5_01/, including the base snapshot (`git archive 8abb5274a9`) and the sweep test file, which never entered the worktree.
- **The write fence:**
  - `retained_memory.rs`;
  - the new `build.rs` and `build_identity.rs`;
  - `Cargo.toml`, the `build` key only;
  - one new test file, `retained_memory_law_tests.rs`, mounted from `retained_memory.rs`;
  - exactly the two struct-construction lines (`retained_product.rs` `LateFacts{…,capture:&*self}`; `lib.rs` `CompleteFacts { …, capture: &observer }`).
  
  `retained_facade_tests.rs` and every other file are unchanged.
- **Records.** Only R/I65/u4_g5_01/. There are no machine paths.
