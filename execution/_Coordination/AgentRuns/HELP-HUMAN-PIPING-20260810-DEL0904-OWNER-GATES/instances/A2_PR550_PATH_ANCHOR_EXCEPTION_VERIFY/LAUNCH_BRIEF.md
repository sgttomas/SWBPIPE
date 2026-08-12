# Sealed Brief — A2-PR550-PATH-ANCHOR-EXCEPTION-VERIFY

RequestedBy: `HELP_HUMAN`

RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`

ParentInstanceID: `WORKING_ITEMS-A1-PR550-PATH-ANCHOR-REPAIR`

ChildInstanceID: `A2-PR550-PATH-ANCHOR-EXCEPTION-VERIFY`

Agent form: fresh non-delegating ephemeral Agent 2 verifier

PackageID: `PKG-09`

Bounded integration scope: PR #550 historical-control portability repair

Repository resolution: resolve `REPO_ROOT` with
`git rev-parse --show-toplevel`; set `WORKING_ROOT` to
`${REPO_ROOT}/projects/chirality-piping`. Create any scratch root with
`mktemp -d` and keep all generated test/cache output outside the repository.

Accepted basis: branch `codex/piping-del0904-owner-gates-20260810` at
`1613ebfae29634242cf140d55b7309e945ba43f1`; accepted author return SHA-256
`741c031758a963fa4f5e9a3ac0984b2923287194199943bc3576b870260556dd`.

## Objective

Independently verify that the author preserved the completed V1 sealed brief,
added exactly one lawful hash-bound historical-control exception, cleared the
PR #550 harness defect without changing execution meaning, and introduced no
unrelated or repository-side drift. Return terminal `PASS` or precise `HOLD`;
do not repair author output.

## Declared reads

- root and project `AGENTS.md`
- `tools/validation/README.md`
- path-anchor validator and practitioner-harness role/policy implementation
- Amendment V5, Amendment V6, and the V1/V2 verifier records
- author sealed brief, return, status, and candidate policy
- Receipt 99, final handoff, branch/commit history, and PR-repair delta

## Allowed tools

Read-only shell/Git, JSON parsing, SHA-256 and Git-blob hashing, repository
validators, and non-writing pytest with bytecode/cache disabled and temporary
output outside the repository. Do not delegate.

## Allowed write targets

- this instance's `RETURN.md`
- this instance's `STATUS.json`

## Required verification

1. Exact branch and original head; no staged or ignored paths; original PR
   payload remains intact outside the bounded repair delta.
2. Frozen V1 brief remains SHA-256
   `d937f558ee2b38d6d1458790de5efd2d987d9a2ce584ba9cfec2869bebca4a41`
   and Git blob `f52c0bf13f4ece6ed2631a0a3c7c941c7a6451d5`; V1/V2 provenance language
   remains unchanged.
3. Policy delta is exactly one appended `control_path_exception` with the
   required repo-relative path, hash, role, reason, and authority; all prior
   entries are field-identical and order-preserved.
4. Live path-anchor validator passes with zero findings, zero
   unacknowledged-control, zero active-unclassified, and zero policy issues.
5. Exact formerly failing live-baseline/harness test, focused policy tests,
   full practitioner-harness suite, and harness self-check pass. Direct every
   cache/temp/generated output outside the repository; if a command cannot be
   made non-writing, do not invoke it and record a precise caveat.
6. Candidate whitespace, `git diff --check`, claims-language, receipt contract,
   JSON validity, and exact containment pass.
7. Receipt 99, DEL-09-04 surfaces, TM-PIP-037 closure, owner rulings, DEC-025
   summary, final handoff, and all other prior bytes remain unchanged.
8. Locally observed `origin/main` has the pinned base as ancestor, a complete
   Piping tree identical to the pinned base, and zero intervening overlap with
   the repair paths. Record the no-fetch/currentness caveat.

## Escalation and exclusions

Return `HOLD` on any failure, unexpected write, or ambiguity. Do not edit the
candidate, any historical record, receipt, deliverable, register, evidence,
owner ruling, lifecycle, release, or other path. Do not stage, commit, push,
merge, fetch, rebase, reset, clean, or delete anything.

## Return contract

Write a structured `RETURN.md` with verdict, identities, delta proof, test and
validator results, containment/currentness caveats, non-effects, and
attestation. Write a machine-readable `STATUS.json` naming parentage, verdict,
hashes, counts, tests, findings, and terminal Git/ignored/staged state.
