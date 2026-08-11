# Sealed Brief — A2-APPLICATION-VERIFY-POST-CLEANUP

RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`

ParentInstanceID: `WORKING_ITEMS-A1-APPLICATION`

ChildInstanceID: `A2-APPLICATION-VERIFY-POST-CLEANUP`

Agent form: fresh non-delegating ephemeral Agent 2 verifier

Accepted base and HEAD: `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`

Worktree: `/private/tmp/chirality-piping-del0904-owner-gates-20260810`

Baseline ignored inventory: exactly zero.

## Objective

Independently verify the complete ruled C-B/V-D/O-B/MR-A application after
exact cleanup. Re-read current bytes and frozen child returns; do not rely on
the interrupted prior verifier as terminal evidence. Return terminal `PASS`
or a precise `HOLD` without repairing any artifact.

## Write scope

Only:

- `instances/A2_APPLICATION_VERIFY_POST_CLEANUP/RETURN.md`
- `instances/A2_APPLICATION_VERIFY_POST_CLEANUP/STATUS.json`

No other repository write is authorized. Do not delegate.

## Tool and side-effect policy

- Create one unique temporary root outside the repository under
  `/private/tmp`; direct every output, cache, build, target, and generated
  comparison there.
- Use `PYTHONDONTWRITEBYTECODE=1` and Python `-B`; use pytest only with
  `-p no:cacheprovider` and an external temp/cache root.
- Do not invoke Cargo or any command that may transitively invoke Cargo. If a
  check cannot be proven without Cargo, record the frozen author/verifier
  evidence and an explicit caveat instead of running it.
- Task Management federation is prohibited, including dry-run invocation.
- Do not install dependencies or access the network.
- Do not stage, commit, push, fetch, rebase, reset, clean, delete, copy an
  artifact into the repository, or invoke any release/receipt action.
- Before writing the return, recheck ignored inventory is zero. After writing
  only the two authorized records, recheck ignored inventory remains zero and
  no other path changed. Any side effect or semantic defect means `HOLD`; stop
  and preserve it without repair.

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

## Return contract

Write a structured `RETURN.md` with terminal verdict, requirements matrix,
exact counts/hashes, tested commands and side-effect safeguards, current-main
caveat, DEC-025 disposition, remaining human gates, and attestation. Write a
machine-readable `STATUS.json` naming parentage, verdict, timestamps if known,
and final ignored/staged/path-containment results.
