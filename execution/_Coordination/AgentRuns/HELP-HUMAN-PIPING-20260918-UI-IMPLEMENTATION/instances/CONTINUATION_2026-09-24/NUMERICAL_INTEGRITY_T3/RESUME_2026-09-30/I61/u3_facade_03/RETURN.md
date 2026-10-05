# I61 RETURN: U3 grant 1c (RV85's no-permit findings S3, N1, N6, N7)

**Status: complete, with no stop.** All four items are uncommitted in WT/f2a-facade, on top of `4b31bbf23a`.
- **Controls 1–5 hold.** No published byte changes without a permit.
- **Mutants:** 23 of 23 killed, none by a compile error.
- **Out of scope here:** the grant-2 items (S-6's G-B hook, S-7's one-run test and N2's V07 test) need I65's API.md or the real permit, so they are not in this grant.
- **ROOT's addendum item for U5** is also done; it is reported last.

**Run facts.**
- **Role and basis:** TASK Type 2 under ROOT, with no descendants. The grant is ROOT's message and RR "RV85 on U3 grant 1: PASS, merged; U3 grant 1b and U5 verified; reviews dispatched" (NUM `fc5d92c56c`). The findings' text is from RV85's completed grant-1 review, `R/REVIEW_RV85/u3_facade_01/REVIEW.md`, read only.
- **Time:** 2026-10-04, 06:51Z to about 07:15Z.
- **Host:** the memory guard (PID 5387) was running throughout. Default toolchain; `--locked --offline`; `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2`; one Cargo job at a time, each under a 1200 s alarm.
- **Git:** no Git writes. Reads used `GIT_OPTIONAL_LOCKS=0`.
- **Other agents' files:** I did not touch R/REVIEW_RV85/, R/REVIEW_RV86/, WT/rv85/, WT/rv86/ or I65's files.
- **Writes:**
  - WT/f2a-facade, within the fence: `lib.rs`, the shim in `retained_memory.rs`, `retained_product.rs` (the frozen candidate's overlay) and `retained_facade_tests.rs`;
  - WT/scratch/i61_u3_facade_03/: `cand`, `stub`, `mut`, `mutstub`;
  - WT/targets/i61-u3/;
  - this folder;
  - `R/I61/u5_reference_01/ADDENDUM_01.md`, with its SHA256SUMS line.

## 1. S3: a permitted output keeps its G-A report

**The shim** (`retained_memory.rs` :376, :403):

```rust
pub(super) fn admit(capture, request, entry)
    -> Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport>
// ...
admission(report).map(|permit| (permit, report))
```

`RetainedAdmissionReport` is `Copy`. `assess` matches `Ok((permit, _))`. **For I65:** this is the API.md §2 amendment ROOT routed.

**The dispatch** (`lib.rs` :2288) is `Some(Ok((permit, report))) => return permitted_dispatch(permit, report, …)`. The report travels with the permit, and every permitted output carries `admission: Some(report)`:
- StackReservation (on the caller's thread);
- Domain;
- the coexistence, G-B and G-C fallbacks;
- every W1 outcome.

No permitted path drops it. The doc "None means the existing pre-parse refusal returned before census entry" is now accurate for every entry.

**Tests:**
- The committed `u3_permitted_outputs_keep_the_report_and_gate_order` is structural, because the permitted path needs a permit:
  - no `admission: None` on the permitted path;
  - exactly three `Some(report)` sites;
  - the shim's signature and its `map`.
- **Behind the archive stub,** `admission()` is `Some` for:
  - success;
  - LateGate, CompleteGate and StackReservation;
  - the carried Precommit, Native and Staging faults;
  - every one of the 72 permit-everything fixture invocations (Domain, Coexistence, Preparation, Candidate and successor);
  - exact selection.

## 2. N1: G-C only after exact selection and G-B's outcome (I51 COMPOSITION §2)

**`permitted_run`** (`lib.rs` :2993) now checks, after the finalization check:
1. `ordinary.source_block_recovery.is_some()`, which falls back as `Coexistence`;
2. `observer.late_refusal()`, which falls back as `LateGate`;
3. only then G-C, `check_complete`, and on success `retained_w1`.

**The order** follows COMPOSITION §2. G-B "authorize[s] … only after … exact-block arbitration", and "Only here may R_complete replace R_late". So exact selection is recorded first, then G-B's refusal.

**G-C is never consulted after either,** so a G-C refusal can no longer mask the true cause. `retained_w1` keeps its own coexistence and G-B checks for the private driver; on the permitted path they are now redundant and never fire.

**Tests:**
- The committed structural test asserts the order: exact, then late, then `check_complete`, each with its own cause.
- **Behind the stub:**
  - `check_complete` panics if consulted in `RefuseLate` or the new `NoCompleteExpected` mode;
  - RefuseLate gives `LateGate` with exact bytes;
  - the new `u3_archive_stub_exact_selection_skips_g_c` runs a source-block fixture (`source_blocks/n05-sparse_interactive.request.json`) in both modes and gives `Coexistence` with exact bytes, with G-C never called.

## 3. N6: the staging overlay falls back typed instead of panicking

**`apply_prepared_overlay`** (`retained_product.rs` :3769) returns `Result<(), StagingFault>`. Every former `unwrap`, `expect` and index on the overlay is now a typed site:
- `values`;
- `preview_cases`;
- `pipe_stress_extrema`;
- `pipe_stress_extrema[]`;
- `pipe_stress_extrema[].key`.

**On the facade path,** `FrozenCandidate::staged_envelope` returns `Result`. In `retained_w1`, a fault becomes the new `W1Fallback::Staging(StagingFault)` (`lib.rs` :2219, :3127). That returns the untouched ordinary owner with R-2's notice, because W1 work ran. Nothing on the facade's staging path can panic.

**The private driver's `commit_private`** (test-only path) still stops on a broken invariant, with `expect("frozen overlay invariant")`. Its bytes and behaviour are unchanged; it is not on the facade path.

**Not changed: `PreparedCase::into_ordinary`'s `expect`** (RV85 N6's second site). The owned ordinary is `Some` until `freeze_candidate` consumes `self`, so that `expect` cannot fire on the native-fallback path. There would also be no ordinary to fall back to: making it typed would need either an ordinary copy or a split of `PreparedCase`'s ownership. I propose leaving it as an invariant. If ROOT wants it typed, it is a grant-2 refactor.

**Tests:**
- A committed fault, `break_next_staging`, points the first maxima patch past the extrema. It gives `Staging(StagingFault("pipe_stress_extrema[]"))` with plain + notice.
- Behind the stub, the same fault carried onto the actual Direct entry gives the same result.

## 4. N7: the single-parse guard, strengthened

`u3_n9_single_parse_custody` now also asserts the following.

**The permitted path contains none of:**
- `from_str(`, `from_slice(`, `from_reader(` or `deserialize(`;
- `Deserialize`;
- `LinearStaticPreviewRequest {`;

in addition to `parse(`, `from_value(`, clones and the struct literal.

**Its raw custody is read in exactly two places:**
- the attempted case's id, in `w1_case_id`;
- the precommit invocation.

**Exact call-site counts in lib.rs** (the definition plus one call):

| Function | Occurrences |
|---|---:|
| `permitted_dispatch(` | 2 |
| `permitted_run(` | 2 |
| `retained_w1(` | 2 |
| `carry_test_hooks(` | 1 call (the generic definitions do not match) |

The dispatch's single `permitted_dispatch(` call sits in its `Some(Ok((permit, report)))` arm.

## Controls

1. **No published byte changes without a permit.**
   - **The fixture sweep:** 324 outputs (36 fixtures × 5 routes × 2 modes, including the admission report), **byte-identical** to base `b54caba7ab` (`faed2518…`).
   - **PP:** 662 outcomes, which is grant 1b's 661 plus `u3_permitted_outputs_keep_the_report_and_gate_order`. All base outcomes are unchanged, and t13 (Mac) is the only failure.
   - **runner/headless:** 87 outcomes, identical to base.
   - **Production warnings:** equal to base, apart from result_export's own existing warning.
2. **The pinned successor bytes are unchanged** through the private driver and the stub's actual entry (`outputs_sha256.txt`).
3. **Fault controls:** grant 1b's set, plus Staging (plain + notice) and the N1 causes (exact bytes).
   - **Under a permit-everything stub,** all 72 fixture invocations keep their bytes and their report.
   - The stub's linear-permit structural test fails there by construction, because the stub replaces the struct.
4. **Nothing is weakened.**
   - No reader, schema, fixture or existing check changed.
   - There is no maintained permit constructor.
5. **Mutants: 23 of 23 killed, none by a compile error** (`mutants_summary.txt`).
   - **S3:** C01–C03, each in both trees.
   - **N1:** C04 in both trees; C05 behind the stub.
   - **N6:** C06 (fault ignored), C07 (bad patch skipped), C08 (wrong cause) and C09 (no notice).
   - **N7:** C10 (raw re-read), C11 (`from_str` re-derivation) and C12 (a second `permitted_dispatch(` call site).
   - **The permit-only set re-run on the new text:** Y01 and Y03–Y08.

## 5. ROOT's addendum to U5 (separate item)

**`R/I61/u5_reference_01/ADDENDUM_01.md`** has its own line appended to that folder's SHA256SUMS; every line verifies OK. It corrects two statements without changing any result:
- **S-1.** The seven represented-readout misses depend on J, through the receipt's k_t; they do not depend on A or Z.
  - I50's J is about 4.3 ulps above the exact annulus J, and the receipt's k_t is correctly rounded.
  - N1 rx is also J-dependent, but too insensitive (J share about 1e-4) to miss.
- **N-2.** The oracle's lines 120–129 do check the mode and parity rows. RV86 applied them, and they pass.

**It also records** that U3 grant 2's rerun will switch `u5_compare.py` to RV86's committed 7,240-byte extract (sha `a66a8a49…`), keeping the hash assertion on I50's full log. I did not make the change here.

## Records (`_run_records/`)

All paths are placeholders, with no machine paths.

**Code and hashes**
- `changed_files_sha256.txt`: against `4b31bbf23a`.
- `candidate.diff`, `candidate_status.txt`.

**Suites, warnings and the sweep**
- `suite_{base,final}_{pp,runner}.outcomes`, `build_{base,final}.warnings`.
- `fixture_sweep_compare.txt`.

**Stub and mutants**
- `stub_patch.py`, `stub_test_tail.rs`, `stub_e2e.txt`.
- `mutants.py`, `mutants_summary.txt`.

**Other records**
- `outputs_sha256.txt`, `run_final.sh`.
