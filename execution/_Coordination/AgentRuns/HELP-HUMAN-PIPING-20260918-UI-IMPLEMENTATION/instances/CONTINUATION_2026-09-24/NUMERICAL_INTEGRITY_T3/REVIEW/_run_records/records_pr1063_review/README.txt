RV26: run records for the independent review of records PR #1063
(T3/REVIEW/RECORDS_PR1063_REVIEW.md)

Candidate: PR #1063, head 21e2285e30d47113a4d4dceae71c60696f1832ab, base main 490b75bd9 (#1062's merge).
Reviewer: RV26, a Type 2 TASK dispatched directly by ROOT (HELP_HUMAN) as a
background subagent. Read-only: no Git writes or index operations; only git
show/diff/log/rev-list/merge-base/ls-tree/cat-file, gh pr/run reads, shasum,
sysctl and folder listings. No cargo. Scripts are committed as *.sh.txt.
Paths are repository-relative or placeholders (<wt> is the T3 worktrees root,
<VENV> the repository venv, <home> the user's home).

Files (script -> output):
- (RV25's committed append_only.py.txt, unchanged) -> append_only.out.txt
    python3 T3/REVIEW/_run_records/records_pr1062_review/append_only.py.txt 490b75bd9 21e2285e3
    difflib line opcodes for the 5 modified files. (Review section 2.)
- (RV25's committed sums_coverage.py.txt, unchanged) -> sums_coverage.out.txt
    ... sums_coverage.py.txt 490b75bd9 21e2285e3 projects/chirality-piping/execution/_Coordination
    The CHECK line for REVIEW/_run_records is main's unchanged sums file, whose
    entries are relative to REVIEW/ (RV25-N5). (Section 3.)
- (RV25's committed leak_scan.py.txt, unchanged) -> leak_scan.out.txt
    RV25_EXTRA_PATTERNS_FILE=<scratch file> python3 ... leak_scan.py.txt 490b75bd9 21e2285e3
    The host patterns (computer name, local host name, user name) were supplied
    at run time from a file in the session scratch and are not recorded. (Section 4.)
- checks.sh.txt -> checks.out.txt
    1 the PR, commits, scope, modes, binaries, git diff --check;
    2 shasum -a 256 -c from each folder; hashes of RV25's review and sums at the head;
    3 DEC-025 durations from each Mac-era merge record's meta.txt;
    4 KF2's B: grant and acceptance ruling commits and its records' time span (finding S1);
    5 review windows from ruling commit times (finding N2);
    6 line numbers of the rulings and reviews behind the notes' claims and S2, N8.
- ci_job_times.sh.txt -> ci_job_times.out.txt
    "Numerical cargo suite" job durations on every Piping Desktop E2E run of
    the nine Mac-era slice PRs' heads (finding N2).
- test_counts.sh.txt -> test_counts.out.txt
    the notes' #[test] recount command (macOS /usr/bin/grep) for KF3, KF2, K6b,
    KF1 and V-K against each merge record's suites_vs_baseline.txt.
- gen8.out.txt
    GEN-8 at the head, run twice from <wt>/numerics: (1) before any RV26 file
    existed, git status empty; (2) with this folder and the review present.

Environment: macOS (aarch64), system python3 for the scripts, <VENV> python
for GEN-8, gh CLI read-only use.
