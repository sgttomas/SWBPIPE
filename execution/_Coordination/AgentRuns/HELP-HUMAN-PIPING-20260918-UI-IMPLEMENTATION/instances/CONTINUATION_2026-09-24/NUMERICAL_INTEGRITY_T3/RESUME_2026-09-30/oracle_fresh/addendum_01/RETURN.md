# Addendum 01 — validated preserved-output comparison

Status: complete for ROOT's bounded follow-up. Original freeze manifest,
CHECKPOINT_0, exact_oracle.py, TRUTH.json, and all other 12 originally inventoried
files remain byte-identical. New work is confined to oracle_fresh/addendum_01.

The additive validate_compare.py wrapper closes the two reported parser gaps
without modifying expectations or the frozen comparator. Before invoking it,
the wrapper requires a known singleton STATUS; one well-formed SELECTED record
with an allowed p/2p pair for selected results; no selection/publication records
for unselected results; duplicate-free, complete, finite, nonnegative floors
at 512 and no floors otherwise; four unique finite nonnegative scales; and
well-formed unique ROW records. Value records require finite lowercase 16-hex
bits and a value-bearing class. Underflow/Overflow require no value bits,
Unpublishable and no bound. Absolute bounds must be finite and nonnegative;
other classes cannot carry a bound. Negative zero is rejected as noncanonical.
The wrapper pins the unchanged oracle and truth hashes before comparison.

Grammar controls accepted three valid forms and rejected 21 malformed forms,
including unknown status, duplicate/missing/malformed selected metadata,
duplicate/missing/negative/nonfinite floors, malformed bit width/case,
nonfinite/negative-zero value bits, invalid outcome/value/class combinations,
missing/negative bounds and duplicate rows. These small synthetic controls test
only prevalidation, not solver behavior or row completeness. The original
comparator still checks exact row coverage and source layout semantics.

## Preserved observations

All raw files were read after the original truth freeze and after the wrapper
and controls passed. Origins are at response revision
520d7dfb790bcedabc03e92b9692884ce295be54 under
`instances/A1-DIAGNOSIS/<continuation>/cases/<case>/guard-workload.log`.

| Origin | Raw SHA256 | Result | Admission standing |
|---|---|---|---|
| continuation_03 / B01 | f8b8781df709efdb98b17d0d689b0f5c6eb45e460dc944e45a55fac58e7e7a3e | 128 verified at 256; 37 rows pass | Prior clean admitted B01, as ROOT released it |
| continuation_03 / B02 | a293af61d547adba0ff93fe8d3df3824340f5c9919578e177b0f9e9cddad6c19 | 256 verified at 512; 37 rows pass | Guard-failed; forensic output only |
| continuation_04 / B02 | a293af61d547adba0ff93fe8d3df3824340f5c9919578e177b0f9e9cddad6c19 | 256 verified at 512; 37 rows pass | Guard-failed; forensic output only |

Every run has 11 InputDerived and 26 AbsoluteVerified rows, with no
RelativeVerified or Unpublishable rows. All 37 published bits in each run equal
the independently frozen direct-rounding baseline. B01's only nonzero exact
error is D:9; B02's are D:6 and M:1. Each error is h/4=2^-1076, against an exact
absolute publication allowance 3h*(1+2^-22). The worst exact ratio is
1048576/12582915, below 1. There is no exact-vs-binary64 predicate disagreement.
The zero-bound rows agree exactly, without a fallback allowance.

The two B02 raw logs are byte-identical. Their honest numerical values do not
repair their guard failures or qualify fresh admitted evidence. The public
1e-9 relative predicate and sharper scale-relative predicate have no applicable
rows in these particular outputs; these comparisons do not validate those
predicates generally or close the coupled-scale proof gap.

Each log reports SOURCE_COMMIT 3bddc2b05f6106e969c7cf43373b230845c7cc66 and limits
100000000/100000000. Independent read-only Git tree queries confirm that the
entire core/solver/frame_kernel tree is the same object at that source revision
and at d01ad98a754698631f927709d08284c272de85e8:
197bdfdf1af7ca7e05b0192bc3f70f6a6ff86e86. This establishes source-tree equivalence,
not renewed host admission. SOURCE_BINDING.json preserves the queries' results
and header observations.

## Exact CLI for I22

From the A1 checkout root, set R to the repository-relative resumed-run path:

```sh
R='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30'
PYTHONDONTWRITEBYTECODE=1 python3 "$R/oracle_fresh/addendum_01/validate_compare.py" "$R/oracle_fresh" "$R/oracle_fresh/addendum_01/raw/continuation_03_B01.log" "$R/oracle_fresh/addendum_01/comparison/B01.rerun.json"
```

For new I22 outputs, replace the last two arguments with the actual raw TSV and
a new report path in I22's own authorized write scope. Do not overwrite sealed
reports. Exit 0 means prevalidation and applicable exact comparisons pass; exit
1 means a numerical/comparison finding; exit 2 means invalid input, frozen-hash
mismatch, or comparator exception. Numerical pass is separate from guard/run
admission and candidate provenance. Keep raw stdout/stderr and exit code.
COMPARISON_RUNS.json retains the actual argv and outputs used for all three
preserved comparisons. The original unwrapped comparator is not the recommended
entry point for new outputs.

To rerun grammar controls without replacing sealed evidence:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 "$R/oracle_fresh/addendum_01/validation_controls.py" <new-owned-control-report.json>
```

## Read ordering, provenance and limits

This is a direct follow-up to `/root/a1_oracle_fresh`; no new agent or delegation
was created. It uses ROOT's explicit follow-up authorization and the same pinned
instructions/briefs as CHECKPOINT_0. No extra skill or workflow body was loaded.
Actual tool results, in order:

- 43000b: rehash original frozen files, 12 unchanged.
- 801bcf: create additive wrapper/controls and run controls successfully; no
  preserved solver output had yet been opened.
- 565a35: sequential `GIT_OPTIONAL_LOCKS=0 git show` reads of precisely the three
  released logs; save unchanged raw bytes; run the wrapper for each; retain
  comparison JSON, raw stdout/stderr and exit codes. No old comparator read.
- 5839ce: summarize only the newly generated comparison reports with Fraction
  arithmetic; write SUMMARY.json.
- fce5f8: read the copied headers and query the two FK tree objects with
  `GIT_OPTIONAL_LOCKS=0 git rev-parse <revision>:<path>`; write SOURCE_BINDING.json.
- Addendum seal: rehash all originally frozen files, write this return and the
  additive manifest containing new write inventory/hashes.

Standard-library Python only; no Rust, solver execution, host-tool changes,
maintained source edits, Git/index writes, new expected values, or governance
acceptance. No source-encoded identity replay, retained verification-scale
reconstruction, general source reachability proof, p=512 runtime floor witness,
G5a/unit-conversion qualification, or general A1 design closure is claimed.
The original audit's scale-transfer proof gap remains open.
