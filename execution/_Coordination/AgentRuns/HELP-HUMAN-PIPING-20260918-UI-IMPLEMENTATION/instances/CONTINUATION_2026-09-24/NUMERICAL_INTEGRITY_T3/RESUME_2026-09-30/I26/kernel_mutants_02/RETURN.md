# I26 — remaining kernel mutants

**NONE passed, and all seven authorized M06–M12 mutations compiled and were
detected by the unchanged fixed tests.** No survivor, normal-control failure or
compilation-only failure occurred. This completes the bounded execution block;
ROOT and independent review retain acceptance and wider programme closure.

The maintained frozen basis is `81c03849033f3ce745668f581f446530789397b8`.
Records-only HEAD `da91674ed6f32d1e7b5722922227933af775b68e` had identical core
bytes. The normal control and each standalone mutation used a fresh owned core
archive of the frozen basis, with the exact same tests throughout. P means
`projects/chirality-piping`; H is `P/core/solver/performance_harness`; FK is
`P/core/solver/frame_kernel`; R is this packet's resumed T3 root. No maintained
source or test was edited.

The fixed selections were all ten tests in `H/tests/k6c_envelope.rs` and
`k6b_w1::the_committed_counts_carry_this_codes_estimate`. NONE passed both;
the latter checked all 33 committed count rows. Mutated H33 executions abort at
their first failed assertion, so no complete mutated row traversal is implied.
These are arithmetic/reference checks, with the already reviewed tiny public
Uc source witness permitted. No benchmark, large model/count construction or
solver run occurred, including for the count row named `n10000`.

| Variant | Actual isolated mutation and outcome |
|---|---|
| M06_RES_EXACT | Active `res=G16(B)` became `16B`. Four kernel tests and H33 failed numerically. For the five-body upper fixture, both full-solve metrics were 48 bytes low, exactly the lost capacity. |
| M07_TOP_DROP | Removed only the ResolutionTop insertion. The full roster was 85 versus 88; direct TOP lookups failed in three fixtures. H33 passed. This is an identified named-phase contract failure. |
| M08_HATCHECK_DROP | Removed only the ResolutionHatCheck insertion. The full roster was 85 versus 88; direct HATCHECK lookups failed in two fixtures. H33 passed. |
| M09_SUMMARY_MISSING | Removed only prepaid `40*B` per verification attempt. Five kernel tests and H33 failed. Five-body retained prefix was 1,089,326 versus 1,089,926: 600 bytes low. |
| M10_SUMMARY_DOUBLE | Replaced that single `40*B` by `80*B`. Five kernel tests and H33 failed. Five-body retained prefix was 1,090,526 versus 1,089,926: 600 bytes high. This is an overcount identity check. |
| M11_ACTIVE_OLD_DROP | Removed only TOP's current `old(16,B)` surcharge. One kernel assertion failed: requested/moving 1,097,686/1,097,686 versus 1,097,686/1,097,750. Requested stayed fixed; moving lost exactly 64 bytes. H33 passed. |
| M12_REBUILD_OLD_DROP | In standalone tracker rebuild, changed `table+kept` to `kept`, retaining the active kept-buffer growth surcharge and every other alternative. Ten kernel tests passed. H33 failed on the committed `RF-LARGE-CHAIN-n10000-AX` counts: each solve field, max and sel128 was 221,440 bytes low. |

The targets follow the source lifetimes. FK `verify.rs::resolution_scale` holds
the two-Wide TOP array while fallibly collecting RES; `resolution_hats` is a
separate collection while RES remains. The current collection's realloc-old
buffer exists only at that active growth, explaining M11's narrow TOP site.
FK `adaptive.rs::summarize` constructs the resolution/theta/bound summary payload
and `record.verification` receives it once; its prepaid 40 bytes per body must
not be omitted or duplicated. In `BoundedExtremeTracker::prune`, the original
`self.table` backing survives its drain while `kept` grows and shrinks, until
`self.table=kept`; M12 removes precisely that old original table, not the kept
buffer or other tracker owners. Exact source replacements, expected observables
and actual diffs are in `_run_records/VARIANT_PLAN.json` and each variant folder.
M07/M08 credit is tied to those missing named phases and the actual roster/lookup
failures, not merely an unsuccessful process.

M12's actual H33 solve fields were
`[534440384,569001536,653661184,684735872]` against
`[534661824,569222976,653882624,684957312]`. Max was 3,266,234,797 against
3,266,456,237; sel128 was 1,314,473,405 against 1,314,694,845. The small kernel
fixtures' passing results do not establish equivalence of the omitted owner.
M09/M10 changed H max and sel128; H's `fixed` diagnostic stayed unchanged,
refining the possible symptom identified in the pre-run plan.

Original M01–M04 evidence remains attached to its original source/test identities.
H M05's original survivor and the later I26/RV35 reviewed discrimination remain
separate historical controls. None was rerun or retrospectively relabeled in
this block. All 116 files in those three prior packets remain byte-identical.
`_run_records/HISTORICAL_CONTROLS.json` preserves their exact result references.

Every run used installed Rust 1.97.1, auto-install disabled, locked/offline,
incremental disabled, `-j4`, testthreads2 and a fresh owned target. The existing
M5 guard was checked before each Cargo invocation. All mutant test binaries
differed from NONE. Exact argv/environment, tool versions, original/mutant/test/
binary hashes, full raw outputs and assertion excerpts are retained below
`_run_records`. Each patch was materialized before its test; normal source was
restored after every trial. Final checking verified all 564 archived files and
all 564 maintained core files against the frozen basis without differences.
Last compiled artifacts remain M12 diagnostics, not restored normal binaries.
Cargo is idle and its lane has been released to ROOT.

Numerical evidence here is conditional identity and named-phase discrimination,
not measured allocation, full proof, executable qualification or engineering
acceptance. Supporting evidence establishes inputs, unchanged tests, real
compilation, distinct binaries and byte preservation. No authorized M06–M12
variant remains unexecuted. Fresh independent review and ROOT acceptance remain;
no automatic follow-on, other mutation programme or performance slot is taken.
The packet seal is `_run_records/SHA256SUMS`.
