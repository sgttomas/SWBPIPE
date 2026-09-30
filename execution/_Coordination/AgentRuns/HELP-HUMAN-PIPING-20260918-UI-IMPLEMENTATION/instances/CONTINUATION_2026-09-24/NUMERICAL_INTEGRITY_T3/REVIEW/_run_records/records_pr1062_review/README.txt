RV25: run records for the independent review of records PR #1062
(T3/REVIEW/RECORDS_PR1062_REVIEW.md)

Candidate: PR #1062, head b0e3579856fbf7bb4c6a68e8b2c46a060231e037, base main 7ad3a9adf.
Reviewer: RV25, a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN) as a
background subagent. Read-only: no Git writes or index operations; only git
show/diff/log/rev-list/merge-base/patch-id, gh pr/run/api reads and shasum.
Scripts are committed as *.py.txt / *.sh.txt. Paths are repository-relative or
placeholders (<wt> is the T3 worktrees root, <VENV> the repository venv).

Files (script -> output):
- append_only.py.txt -> append_only.out.txt
    difflib line opcodes for the 8 modified files, base 7ad3a9adf4 -> head
    b0e3579856; SHA256SUMS files compared entry by entry. (Review section 2.)
- sums_coverage.py.txt -> sums_coverage.out.txt
    every SHA256SUMS at the head whose folder holds a changed path: entries
    verified against head blobs, tracked files unlisted, and changed paths no
    sums file lists. The CHECK line for REVIEW/_run_records is main's
    unchanged sums file, whose entries are relative to REVIEW/ (finding N5).
- shasum_c.sh.txt -> shasum_c.out.txt
    `shasum -a 256 -c` from each folder (run from the T3 folder); the last
    line runs REVIEW/_run_records/SHA256SUMS from REVIEW/. (Section 3.)
- leak_scan.py.txt -> leak_scan.out.txt
    GEN-8's MACHINE_ABS_PATH_RE, broad path forms, model identifiers, and the
    host's computer name, local host name and user name. The host patterns
    were supplied at run time from a file in the session scratch and are not
    recorded; their hits print as <redacted>. (Section 4.)
- gen8.out.txt
    GEN-8 at the head, run twice from <wt>/numerics:
    (1) with this folder moved aside, so the tree was exactly b0e357985;
    (2) with this folder present, untracked, to show these records pass.
- merge_facts.py.txt -> merge_facts.out.txt
    each new *_MERGE/RECORD.md against gh (PR, merge SHA and time, head,
    runs, dispatch target_base from the run log) and git (parents,
    ancestry). (Section 5.)
- dec025_chain.py.txt -> dec025_chain.out.txt
    each record's dec025/suites.log against its stated baseline's, keyed by
    manifest; pytest and vitest summaries; baseline piping tree vs the named
    main. (Section 5.)
- cited_hashes.py.txt -> cited_hashes.out.txt
    sha256 prefixes and line counts that ROOT_RULINGS_V1.md cites, found in
    Git history. (Section 6.)
- figure_sample.py.txt -> figure_sample.out.txt
    figures V1 restates, searched for in the source each cites. A hit shows
    the source states the figure; misses were read by hand (review section 6
    and findings S1, S2, N3, N4).
- misc_checks.sh.txt -> misc_checks.out.txt
    the head's merge of main; KF3's merge-order slip; slice merges' parents
    and remerge diffs; spawn bases; the A0 export; CI runs cited only in the
    rulings and the PR's own runs; K5's baseline tree; V1's in-place edits
    inside the PR's history; uncommitted evidence hashes (<wt>/scratch).

Environment: macOS (aarch64), system python3 for the scripts, <VENV> python
for GEN-8, gh CLI authenticated read-only use. No cargo.
