# Executed final merge checks

Run from ROOT's clean resume-records checkout, using the supplied Piping Python
3.13.14 venv. <ROOT> is that checkout; <wt> is its parent. All Git subprocesses
set GIT_OPTIONAL_LOCKS=0. The initial review's skill and TASK context persist;
current origins/hashes are in CONTEXT.json.

Read-only commands included git rev-parse HEAD, status --porcelain=v1, show -s
--format for parentage, merge-base, diff-tree --cc --no-commit-id --name-status
-r, diff --name-only, and rev-parse of each relevant tree/blob identity.
The retained standard-library helper ran as:

```
<VENV>/bin/python <wt>/scratch/rv27-resume/check_merge.py <ROOT>
```

It read git ls-tree -r -z --full-tree for the original base, prior reviewed head,
new main, and final head; constructed the unique disjoint union in memory;
compared every path/mode/blob with the final tree; verified existing seals;
compared complete raw diffs; and streamed both complete --binary --no-ext-diff
--no-renames patches into SHA256. Exit 0, results in evidence/CHECKS.json.
No merge-tree, remerge object creation, index mutation, or full binary-patch
copy was needed. The compact complete raw diff and incoming path inventory are
retained. No App-v4 artifact was edited or accepted by this check.

From <ROOT>, executed:

```
GIT_OPTIONAL_LOCKS=0 PYTHONDONTWRITEBYTECODE=1 <VENV>/bin/python -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8
```

Exit 0. Captured original stdout is evidence/gen8.log; post-check Git status
remained empty. A read-only gh pr view 1068 with state, headRefOid, baseRefOid,
and url confirmed the pushed final head and new main. Writes were only the
assigned additive mergecheck subtree and owned scratch helper. Prior sealed
review packets remain intact. No current CI dispatch or DEC-025 was executed
or claimed here.
