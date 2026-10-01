# RV27 commands and reproduction

Execution used the supplied Piping Python 3.13.14 venv, standard-library checks,
read-only Git/GitHub, and the existing GEN-8 pytest. <CANDIDATE> is ROOT's clean
resume-records checkout at the full reviewed head; <wt> is its parent;
<OUT> is <wt>/scratch/rv27-resume. Actual host paths are in the launch/tool record.
No Rust, solver or DEC-025 command was invoked.

Every Git read, including reads from Python subprocesses, used
`GIT_OPTIONAL_LOCKS=0`. Python/pytest subprocesses disabled bytecode writes;
GEN-8 disabled pytest cache. Initial direct file inspection used cat/rg and
small Python reads of the selected files. `git show` at response head supplied
the scope handoff before response errata or review material. One initial handoff
read from the candidate failed because it belongs to the pinned response commit;
that was corrected with `git show`.

Executed read-only Git commands included rev-parse, status --porcelain=v1,
diff --name-only/--name-status/--stat and bounded textual diff, show (full object
identities), and ls-remote for the response/records branches. GitHub reads:
`gh pr view 1063/1066/1068 --repo sgttomas/chirality --json <recorded fields>`.
JSON and remote outputs are under evidence/. No API mutation was performed.

The scope command was:

```
GIT_OPTIONAL_LOCKS=0 PYTHONDONTWRITEBYTECODE=1 <VENV>/bin/python tools/software_workflow/validate_change_scope.py <CANDIDATE> --base d01ad98a754698631f927709d08284c272de85e8 --head 90b6bcbbf64b13975211038bf3f33bb87273e646 --allowed <T3>/RESUME_2026-09-30 --allowed <T3>/ROOT_RULINGS_V1.md --allowed <T3>/REVIEW/RECORDS_PR1063_REVIEW.md --allowed projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md
```

It returned exit 0 / PASS. The complete allowed set and paths are in scope.json.

The retained standard-library scripts were executed with:

```
<VENV>/bin/python <OUT>/check_records.py <CANDIDATE>
<VENV>/bin/python <OUT>/check_bindings.py <CANDIDATE>
```

They write only <OUT>, set GIT_OPTIONAL_LOCKS=0 internally, stream gzip restore
without a large in-memory matrix, and retain CHECKS.json/BINDINGS.json plus
stdout logs. Both final executions exited 0. The first check_records attempt
used the wrong audit-manifest location and stopped with FileNotFoundError;
its own helper was corrected to AUDIT/SHA256SUMS and rerun. No checked artifact
was repaired. Scripts were then parameterized for the supplied candidate root
and rerun in full; retained scripts/outputs are the executed final versions.

A principal-input mismatch in CHECKS.json means current candidate versus the
old audit snapshot, not a failed claimed-basis verification: ROOT_RULINGS_V1.md
and WORK_GRAPH.md intentionally changed. Their d01ad98a base hashes match all
61 original declared inputs. The review records that distinction explicitly.

Fresh GEN-8, run from <CANDIDATE>:

```
GIT_OPTIONAL_LOCKS=0 PYTHONDONTWRITEBYTECODE=1 <VENV>/bin/python -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8
```

Exit 0; 1 passed, 10 deselected in 32.30 s; original captured output gen8.log.
The candidate stayed clean. No replay of the historical Rust/solver workload,
current CI dispatch or final merge check is represented by this review.
