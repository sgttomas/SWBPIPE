# ROOT checkpoint B — compile, ledger freeze and bare-b oracle

Source candidate is A1 `3cf296e36645d97e4c657c8ad1a6322bc4163f16`, a clean
merge of records-only main into preserved source WIP ae781a8745. Numerical source
hashes equal I22/implementation_a/SOURCE_SHA256SUMS. ROOT read the complete diff,
verified hashes/write set and preserved all returned seals. PR1068 is merged;
it is not A1 closure. COMMON and earlier scope limits continue.

## WORKING_ITEMS manager and I22

Resume only existing I22. Grant one M5 host slot, own `<wt>/a1-target`, existing
guard running, installed Rust 1.97.1, auto-install=0, offline/locked -j4,
incremental=0, test threads 2. First compile/run the authored publication_tests
filter in FK lib, then H k6b_export only. Preserve exact argv/env/head/raw outputs.
Initial checkpoint bound: 30 minutes; no repeat of a numerical failure. Compiler
errors can receive bounded fixes within the four granted maintained files, at
most two repair attempts before returning. Freeze each changed-source hash and
delta; source/tool blocker or unexpected protected standing returns immediately.
Do not run broad old suites, external probes, mutants or scale jobs yet.
No accounting counter dump is requested; RV29 freezes source-derived expectations
without observations. Current tests cannot establish full accounting closure.

The extra controls' independent truth is now released from
oracle_fresh/addendum_03_source_controls (manifest c6cc8d878eec3fd9efcfdc8c5839daeb278dd91d1e9ed53f8bf84435495c8673;
truth aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee).
Read the source IDs exactly; existing proposed builders may need their IDs made
identical to the pinned inputs before identity checks. Do not change input truth.
Review this for the next bounded tests; first return compile/focused-test status.
Write additive I22/implementation_b and manager/implementation_b plus owned
scratch. Prior seals stay fixed. I21 remains source-only pending settled kernel.

## Independent reviewer RV29

Read frozen candidate blobs with GIT_OPTIONAL_LOCKS=0 git show (I22 may later
repair working files). Finalize the specified RU64/row-bound and terminal/short-
budget absolute operation ledger from source, before observing new helper
counters. Write additive source_review_RV29/candidate_01. No Rust execution yet.
Then inspect the whole source diff for actionable correctness findings and missing
requirements; this remains preliminary until complete tests/gates. Tell ROOT
promptly if arithmetic/counter order is wrong. Do not author maintained source.
Preparation budget 30 minutes; a source ambiguity returns a concrete finding.

## Independent oracle TASK

Add a bare-b selected-contract comparator in oracle_fresh/addendum_04_bare_b,
without changing prior truth, checker or manifests. Read the accepted design and
correction as contract, not implementation-generated truth. For B/C17 and three
extra controls, require exact absolute |x-truth|<=reported b, public relative
and both exact/binary64 sharper predicates. Keep shape/source/finite/canonical
validation from the frozen interfaces; reject absent or wrong source binding.
Unresolved/refused never means accuracy pass. Do not certify bound construction
or infer expected solver selection. Historical qualified checks stay historical.
Freeze this additive checker and synthetic boundary validation before repaired
outputs are supplied. Standard-library exact arithmetic only; no Rust, new cases,
Git mutations or host tooling. 30-minute bounded return.
