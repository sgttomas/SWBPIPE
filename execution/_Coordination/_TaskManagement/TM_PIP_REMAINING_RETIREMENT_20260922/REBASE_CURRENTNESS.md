# Current-base backcheck after PRs #859, #860 and #861

The isolated Piping branch was rebased onto `origin/main` at
`46ac555032bac3d7ad345bc2bdfcc1cd5255e682`. Its reviewed source basis
was the merged instruction commit
`b3e2ce4ec74e01d6f393fc0bc069699bb079df91`; the historical R5 census
remains SHA-256 `646470f87ea916b939bdbf1d66877ae8181fcd61bd1922b27f63900b30e70551`.
Before this backcheck record, the branch changed 44 paths relative to this
main; this record adds one coordination path. Main changed 99 paths since
the old basis, with **zero changed-path overlap**. PR #860's
management manual/guide and PR #861's App retirement are present only in the
base, outside this branch's diff.

PR #859 changed DEL-09-04 `MEMORY.md`, validation case/test/evidence records
and a coordination notice. The exact DEC-092 case page, validation-manual
index and DEL-09-04 `_STATUS.md` bytes are unchanged between the old and
current bases: SHA-256 respectively
`f6019a2e216cdbc37493c6e77d388f6ad39ced5363e1e75db1d05f5820933110`,
`f6d611059545505bd8ec94595406411487db4db2d23db9983a4162e93e691ee6`
(whole index file), and
`0ae4fa9cf2e2b506e19f0b6f8836e1ceba309c057680e44d8f042acd8421e889`.
Its new MEMORY entry does not reverse the recorded 2026-08-09 DEC-092
derivative completion. `DEL0502_DEC092_COMPLETION_BACKCHECK.md` binds the
separate producing result and receiving page/index/run evidence to all four
original DEL-05-02 census keys; it does not infer whole-manual acceptance.

At this rebased head, all 239 pinned source tuples plus the two same-source
DEL-12-01 extraction omissions retain 241 unique identities. All 106
current `_STATUS.md` files match their expected hashes: 19 intentionally
changed status files match `APPLIED_ROW_LEDGER.csv` postimages, while the
other 87 match pinned source bytes. The ledger has 22 applied keys (18 empty
markers and four DEL-05-02 keys representing two bullets); 219 rows remain
held. Exact reversal confirms each of the 18 marker edits removed only an
empty heading and DEL-05-02 removed only two stale bullets. Lifecycle and
history bytes are otherwise unchanged.

The original 34 Piping register rows are a byte-identical prefix of the
44-row current `REGISTER.csv`. All ten new `DEFERRED` rows have current
SourceSha values, owner-held assignment TBD, priority TBD, checkable triggers
and retained source holds. `taskmgmt validate` passes. The two DEL-17-06
semantic derivatives have only the reviewed 448-byte historical prefix each;
their dated bodies remain identical to current main. The ISSUED DEL-01-01
Scope of Work is unchanged at SHA-256
`05a0604b10c568728eb7bcefb22219d2680c1efc2844da13dfbaba9bb19b856b`.

The 13 pending receiving-document preimages match `PATCH_TARGETS.csv`, and
both the non-issued and issued **unapplied** patches pass `git apply --check`.
`git diff origin/main..HEAD --check` passes. This is a currentness and
mechanical backcheck, not a new product test pass, human disposition for the
219 held rows, issued change, release, or downstream adoption.
