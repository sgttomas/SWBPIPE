# RV29 runtime checkpoint 01 — frozen compiler repair and full FK suite

**FAILED required check; not ready for acceptance/fan-in.** One authorized command
ran on immutable dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1 and exited101. It
reported **440 passed, 3 failed, 1 ignored**. No retry, mutant patch, maintained
source/test edit, tolerance change, or expectation-table repair was performed.
This is a bounded runtime checkpoint, not final implementation review.

The command was `cargo test --offline --locked -j 4 --no-fail-fast`, with no target
filter, installed toolchain1.97.1, auto-install0, incremental0 and two test threads.
It ran from an RV29 git archive into the distinct `<WT>/rv29-a1-target` with
existing guard PID5387 alive immediately before launch and after completion.
The 122 source/fixture files, including the six adjacent source files required by
FK's include_str sites, matched the frozen Git blobs before and after the run.
No moving A1 working source was used. Eight executed test binaries are hashed
in AFTER.json. The run was untimed for performance purposes; UTC start/end
06:56:42–07:09:31 record execution provenance, not a performance measurement.

## Results by target

| Target | Passed | Failed | Ignored |
|---|---:|---:|---:|
| FK lib unit tests | 375 | 2 | 1 |
| k1_k2a_interaction | 3 | 0 | 0 |
| k2a_checked_formation | 13 | 0 | 0 |
| k2b_force_scaling | 20 | 0 | 0 |
| k5_constrained_bodies | 15 | 0 | 0 |
| k5_scale | 1 | 0 | 0 |
| m03_skew_scope | 5 | 0 | 0 |
| s11_site_table | 2 | 1 | 0 |
| FK doc tests | 6 | 0 | 0 |
| **Total** | **440** | **3** | **1** |

The single ignored test is the existing release-only KF2 cost test at6,006 DOFs;
this grant did not authorize running ignored tests. All11 publication tests in
the frozen candidate passed, including C17, zero-source radii, bare-bound/relative
checks and RU conversion. These are the earlier checkpoint tests, not I22's
separately authored augmented PC matrix. The existing all-controls R7 schedule/
honesty test and the listed RF-LARGE source honesty/selection tests passed.
No numerical-accuracy or availability assertion failed in this full run. This
claim is limited to its actually executed tests and does not close outstanding
source probes, PC40–43/PM16, other mutants, or final-head gates.

## Confirmed failures and their interpretation

**RV29-R1 — accounting expectation mismatch, blocks this test.**
`kf1_golden_stop_rule_work_where_collapses_occur` fails at
FK/tests/retained_k4/kf1_tracker_tests.rs:994 for RF-LARGE-CHAIN-n00100-AX at
T=infinity. The p128 stop-rule work is54,140,795 versus the old8,625,285,
a difference45,515,510. Every other displayed field and the p256 verification
row are identical. This old absolute number covers R7 alone, whereas the selected
new implementation adds certificate work to the same record/stage. Therefore the
mismatch is not itself evidence of wrong charging or changed availability.
The first assertion aborts this test; its later T512/T64 and second-model checks
were not executed here and cannot be claimed passed through this test.

**RV29-R2 — accounting expectation mismatch, blocks this test.**
`golden_work_counts` fails at adaptive_tests.rs:563 against unchanged A3B_WORK,
first at N05. The old/observed p128 context counts are530,316/1,125,520;
sum counts2,789/13,035; stop counts492,883/1,098,333. Thus
595,204 context +10,246 sum =605,450 added stop work. The verification row and
other displayed columns match. The test completed its A3a outside-new-stages
and shared-stage comparisons for all four models before comparing A3b rows.

The captured output prints all four models' A3b rows. Independent parsing against
the unchanged source literals confirms that all nine printed rows retain the
other columns, and every context+sum delta equals its stop-rule delta. Nonzero
deltas are N05/p128605,450; N06/p128606,259; TWO-SPAN/p128771,179;
SKEW6-K1E-12/p2562,240,843. GOLDEN_DIAGNOSTIC_ANALYSIS.json preserves the exact
components. These observed differences are diagnostic evidence only: they are
**not replacement goldens or an independent proof of correct certificate work**.

ROOT's source-qualified disposition at COORD4599aa7eea, R/BRIEFS/
A1_KF1_WORK_GOLDEN_DISPOSITION.md adopts criterion-preserving component separation
in principle. ACCOUNTING_TEST_PROPOSAL.md gives the bounded method: preserve old
R7/A3B literals, test the same paired states/report directly, separately check
inclusive closure with the certificate components, and preserve all tracker
price/collapse, unrelated-stage and selected/publication assertions. New absolute
certificate truth remains the independently derived operation ledger and required
PM16 tests. No `>=` relaxation, copied observed total, deleted assertion or updated
historical literal is proposed. Concrete test patches still need ROOT's grant and
RV29 backcheck; no maintained edit is made in this packet.

**RV29-R3 — missing source-site inventory entry, blocks S11.**
`site_table_is_exactly_the_accumulations_of_the_scanned_files` fails at
s11_site_table.rs:623: adaptive.rs/run_schedule has table5 versus source6.
The source independently has six compound assignments at adaptive.rs3971,3997,
4055,4056,4133,4157. The added site is `c += 1` at4133 in the new
PublicationDecision::Rejected branch. It is an integer schedule increment,
not a numerical load/force sum. Existing table row272 already labels this
function's category as integer schedule index and stop-rule work counts.

Minimal proposed repair: change this exact inventory count5→6 and document the
additional publication-rejection schedule increment, preserving every other
count, scanned file and scanner criterion. This is an explicit source inventory
reconciliation, not a numeric tolerance/oracle relaxation. No edit or rerun is
performed; ROOT must grant that additional maintained test path after review.
It does not change the previously established kernel-only execution reach.

## Compiler backcheck and ledger correction

BACKCHECK.md and COMPILE_REPAIR.diff verify only the three authorized repairs:
two explicit Wide::<4> associated items and the distinct new diagnostic match arm.
The successful compile closes candidate_01 findings RV29-C1/C2 at dd1. Helpers
and price primitives are unchanged; original instruction/skill and source review
origins remain linked from BASIS.json.

During this run ROOT separately reported a PC40 max-span mismatch in I22's new
test. RV29 confirmed an error in its own candidate_01 metadata, not in production:
b=2 contributes high exponent+1, H contributes low−1074, so the pre-net term span
is1076. H alone is1075. The additive sealed correction is
`../candidate_01_span_addendum_01`, manifest
90732af11fb8a8271500b53322f294cd0a17c0c9b0e798a76f4fc9126fa84cbb.
All original sealed bytes remain unchanged. Priced work18103/159/17563 and every
budget/PM16 price discriminator remain unchanged; max_span is maximum-only
evidence. RV29's current full-suite command was not altered or repeated for this.

## Evidence and remaining boundary

Original stdout is FK.stdout. Original stderr remains untouched in owned scratch;
FK.stderr.portable replaces only the host checkout prefix with `<WT>` for the
canonical packet. RAW_OUTPUTS.json gives original byte hashes, paths and the
transformation. Source/archive and executed-binary hashes, actual argv/environment,
exit and timestamps are retained. There was one cargo invocation, not a rerun.
The failed `rg` diagnostic extraction initially omitted `--` before a dash-leading
pattern; bounded sed then read the actual diagnostics. A preliminary build.rs
lookup found none; the crate has no build.rs and its Cargo.toml has no dependencies.
Neither lookup failure changed or repeated the runtime command.

No new arithmetic, publication or protected availability failure is established
by these three failures, but the required suite has not passed. Final review is
still blocked pending concrete accounting-test/site-inventory dispositions,
reviewed test repairs, augmented PC tests, independent source/probe comparisons,
required mutants, K6c and the exact-final-candidate merge gates. Author outputs
from later moving test files are outside this executed candidate and need their
own frozen review. No F2a, product/native, engineering, lifecycle or release
acceptance is implied.
