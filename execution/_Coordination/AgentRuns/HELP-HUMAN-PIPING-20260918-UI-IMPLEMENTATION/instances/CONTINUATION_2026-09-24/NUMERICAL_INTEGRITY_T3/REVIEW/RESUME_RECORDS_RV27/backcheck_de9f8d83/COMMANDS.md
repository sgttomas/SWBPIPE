# Executed backcheck commands

<ROOT> is the clean resume-records checkout; <wt> is its parent. The supplied
Piping venv is <VENV>. Run-specific path aliases are resolved by ROOT's launch
record. This backcheck used the same software-code-review skill/context as the
initial review; origins of changed/selected sources are in CONTEXT.json.

With GIT_OPTIONAL_LOCKS=0: `git rev-parse HEAD`, `git status --porcelain=v1`,
`git diff --stat` and the complete `git diff` between the two named candidates,
plus `git show` and a standard-library character diff to inspect all changes
within the graph's long table row. The complete delta is retained as
`evidence/delta.patch`. Read-only `gh pr view 1068 --repo sgttomas/chirality
--json state,headRefOid,baseRefOid,url` confirmed the pushed candidate.

Executed `<VENV>/bin/python <wt>/scratch/rv27-resume/check_delta.py <ROOT>`.
The retained script sets GIT_OPTIONAL_LOCKS=0 for every Git subprocess, checks
the exact two-path delta, every entry of both seals, prior review identities,
source/evidence/manifest immutability, and writes only this backcheck subtree.
Exit 0; results are in evidence/CHECKS.json. No archive restore was repeated
because the complete binaries and their verified hash manifests are unchanged.

From <ROOT>, executed:

```
GIT_OPTIONAL_LOCKS=0 PYTHONDONTWRITEBYTECODE=1 <VENV>/bin/python -m pytest -q -p no:cacheprovider tools/practitioner_harness/test_live_baseline.py -k gen8
```

Exit 0; stdout was captured directly as evidence/gen8.log. Both bytecode and
pytest cache writes were disabled. Post-check status remained empty. No tests,
policies or evidence bytes were weakened or changed. The initial review seal
and this additive backcheck seal are separate; no self-referential hash is claimed.
