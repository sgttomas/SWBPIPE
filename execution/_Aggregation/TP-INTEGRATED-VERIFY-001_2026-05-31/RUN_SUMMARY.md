# TP-INTEGRATED-VERIFY-001 Run Summary

Date: 2026-05-31
Agent: WORKING_ITEMS
Snapshot type: read-only integrated verification derivative package
Snapshot path: `execution/_Aggregation/TP-INTEGRATED-VERIFY-001_2026-05-31/`

## Objective

Verify the current integrated readiness surface after the focused
`TP-DEL1104-SCHEMA-ALIGN-001_2026-05-31` DEL-11-04 schema-example remediation,
and identify the next bounded gap if one remains.

## Authority Basis

- Decomposition authority: `execution/_Decomposition/SOFTWARE_DECOMP.md`
  revision `0.7`.
- Graph authority: `execution/_DAG/DAG-005/`, pointed to by
  `execution/_DAG/_LATEST.md`.
- Graph approval record: `execution/_DAG/DAG-005/APPROVAL_RECORD.md`.
- Coordination record: `execution/_Coordination/_COORDINATION.md`.
- Implementation evidence: `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.
- Blocker queue derivatives: `execution/_Coordination/DEV-001_BLOCKER_QUEUE.md`
  and `.csv`.

This is a derivative verification snapshot. It does not replace decomposition
truth, DAG authority, implementation evidence, lifecycle files, release records,
acceptance records, or professional/code-compliance authority.

## Commands Run

Passed:

```text
python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check
VALID: DEV-001 coordination derivatives for DAG-005

python3 tools/release/check_release_readiness.py --profile all --execute
All planned checks completed successfully.
```

Failed due missing local Node workspace dependencies:

```text
npm run test:desktop
sh: vitest: command not found

npm run build:desktop
sh: tsc: command not found
```

Observed environment:

```text
find . -maxdepth 3 \( -name package.json -o -name package-lock.json -o -name pnpm-lock.yaml -o -name yarn.lock \) -print | sort
./apps/desktop/package.json
./package-lock.json
./package.json
./provenance/build-artifacts/package-lock.json

ls -ld node_modules apps/desktop/node_modules 2>/dev/null || true
<no output>
```

`git status --short` emitted no output at snapshot intake and before snapshot
file creation.

## Summary

- The DEV-001 coordination derivatives are current for `DAG-005`.
- The full provider-neutral release readiness script passed, including the
  repository Python tests, security/privacy tests, dependency schema validation,
  readiness script tests, and 24 discovered crate-local Cargo test surfaces.
- The prior DEL-11-04 schema-example residual did not recur in the release
  readiness script; repository Python contract tests passed.
- The supplemental desktop checks are not currently executable in this local
  workspace because `node_modules` is absent and the required binaries
  `vitest` and `tsc` are unavailable.

## Verdict

Integrated verification status: `PARTIAL_PASS_WITH_ENVIRONMENT_BLOCKER`.

The next bounded owner is the desktop workspace dependency/bootstrap surface:
restore or install Node workspace dependencies, then rerun
`npm run test:desktop` and `npm run build:desktop`.

No lifecycle state, DAG artifact, dependency register, DEV-001 evidence row,
blocker queue, release record, acceptance record, professional claim,
certification claim, sealing claim, authentication claim, code-compliance claim,
or release-readiness-for-reliance claim was changed or made by this snapshot.
