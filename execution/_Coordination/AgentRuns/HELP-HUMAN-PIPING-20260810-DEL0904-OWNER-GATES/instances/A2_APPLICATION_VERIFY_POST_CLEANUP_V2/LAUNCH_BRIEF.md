# Sealed Brief — A2-APPLICATION-VERIFY-POST-CLEANUP-V2

RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`

ParentInstanceID: `WORKING_ITEMS-A1-APPLICATION`

ChildInstanceID: `A2-APPLICATION-VERIFY-POST-CLEANUP-V2`

Agent form: fresh non-delegating ephemeral Agent 2 verifier

Accepted base and HEAD: `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`

Repository resolution: set `REPO_ROOT` from
`git rev-parse --show-toplevel`; set `WORKING_ROOT` to
`${REPO_ROOT}/projects/chirality-piping`. Do not hard-code an absolute
workspace path.

Scratch resolution: create a unique external scratch directory with
`mktemp -d`; store its resolved path in a task-specific variable. Do not
hard-code a machine-local scratch path.

Baseline ignored inventory: exactly zero.

## Objective

Independently verify the complete ruled C-B/V-D/O-B/MR-A application after
exact cleanup. Re-read current bytes and frozen child returns. Treat the V1
post-cleanup verifier as preserved process evidence only, not as the terminal
verification basis. Return terminal `PASS` or a precise `HOLD` without
repairing any artifact.

## Write scope

Only:

- `instances/A2_APPLICATION_VERIFY_POST_CLEANUP_V2/RETURN.md`
- `instances/A2_APPLICATION_VERIFY_POST_CLEANUP_V2/STATUS.json`

No other repository write is authorized. Do not delegate.

## Tool and side-effect policy

- Direct every generated output, cache, build, target, and comparison to the
  unique scratch root created with `mktemp -d`.
- Use `PYTHONDONTWRITEBYTECODE=1` and Python `-B`; use pytest only with
  `-p no:cacheprovider` and an external temp/cache root.
- Do not invoke Cargo or any command that may transitively invoke Cargo. If a
  check cannot be proven without Cargo, record frozen author evidence and an
  explicit caveat instead of running it.
- Task Management federation is prohibited, including dry-run invocation.
- Do not install dependencies or access the network.
- Do not stage, commit, push, fetch, rebase, reset, clean, delete, copy an
  artifact into the repository, or invoke any release/receipt action.
- Before writing the return, recheck ignored inventory is zero. After writing
  only the two authorized records, recheck ignored inventory remains zero and
  no other path changed. Any side effect or semantic defect means `HOLD`; stop
  and preserve it without repair or deletion.

## Required verification

1. Exact branch/base, preparation packet and final child-return hashes, exact
   owner ruling, and current-main caveat.
2. C-B exact five-class zero-count/zero-floor/cap-four/final-zero policy;
   policy-not-release and non-comparison fences; all 12 historical nonlinear
   JSON records byte-identical; schema/consumer compatibility from safe
   non-writing checks and frozen focused-test evidence.
3. V-D manifest/hash integrity, raw/projection identity, current 25/11/0/14
   capture, 206/91/115 reconciliation, historic 13-case preservation,
   DEC-092 as the fourteenth block, per-kind floor analysis, unit-normalized
   sparse design, and zero public numeric selection or implementation repair.
4. O-B exact source-pinned qualified acceptance and immutable R14 bundle:
   tree/checksum identity, all 74/74 entries, `INTERNALLY_VERIFIED`, and no
   current-head reproduction claim.
5. MR-A exact deterministic 64-page membership (21/15/28), exactly 63
   historical page changes with DEC-092 byte-identical, all pages
   `DRAFT_EVIDENCE`, 64-row hash-bound zero-disposition instrument, 1/63
   witness mapping, reproducible aggregate, and no review/promotion.
6. DEL-09-04 lifecycle `IN_PROGRESS`; `_STATUS.md` Last Updated 2026-08-11;
   first Remaining bullet byte-identical to base; second changed only to
   record C-B and preserve open public comparison numeric selection; no other
   Remaining change; memory/run-record currentness truthful.
7. Exact path containment; no Task Management register, receipt, claims,
   PRD, decomposition, DAG, GUI, export/CAEPIPE, reliance, lifecycle/release,
   runner/data/fixture/binding, or other forbidden change; no staged paths;
   whitespace/diff checks; zero ignored drift before and after.
8. Applicable non-writing schema/check/generator tests with all outputs
   external. Record DEC-025 as triggered but commit-bound/post-commit, not a
   pre-commit defect. State any environment limitation without inflating it
   into a pass or a blocker unless it prevents evidence-backed verification.
9. Path-anchor validation must distinguish the two frozen V1-brief findings
   from current V2 control quality: verify this V2 brief itself passes in an
   isolated external scan and the live repository scan contains exactly the
   two preserved V1 findings with no new finding.

## Return contract

Write a structured `RETURN.md` with terminal verdict, requirements matrix,
exact counts/hashes, tested commands and side-effect safeguards, current-main
caveat, DEC-025 disposition, preserved V1 finding treatment, remaining human
gates, and attestation. Write a machine-readable `STATUS.json` naming
parentage, verdict, timestamps if known, and final
ignored/staged/path-containment results.
