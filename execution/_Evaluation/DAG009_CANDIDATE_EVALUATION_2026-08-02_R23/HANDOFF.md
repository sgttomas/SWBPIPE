# Handoff — DAG-009 candidate evaluation

**Accepted upstream snapshots:** approved DAG-008; frozen Git base
`23d15899fd0acf5d1d0513f3fe396438375c9e25`; owner-adopted/amended O-A; R23
N2 fan-in PASS; the SHA-bound inputs in
`UPSTREAM_INPUT_HASHES_POST_AMENDMENT.csv`; portability policy
`f0dacfee06bdd4aac177a962f4710edc99fda0e073e53e965c88fb4d238fade4`.

**Derivative-package status:** validated immutable PROPOSAL under
`_Evaluation`; not decomposition truth and not active dependency authority.

**Evaluation closure:** COMPLETE / PASS — 12 execution closures, 12 unchanged
candidates, five unchanged holds, one anchor normalization, two valid
independent returns, and all deterministic gates pass.

**Subject closure:** NOT ACTIVATED — approved DAG-008 and root pointer are
unchanged; DEC-092 implementation remains unauthorized.

## Route to CHANGE

- Validate every precondition and hash in `APPLICATION_SPEC.csv`.
- Retain/stage the nine existing consumer paths and authorized portability
  policy exactly.
- Copy all 16 `bundle/DAG-009/` files to `execution/_DAG/DAG-009/`
  byte-for-byte.
- Re-run `VALIDATION.md` checks against the materialized proposal.
- Do not semantically edit candidate bytes, complete the approval placeholder,
  or edit root `_DAG/_LATEST.md`.
- Abort and return to HELP_HUMAN on any mismatch.

## Later owner decisions

1. Merge or decline the proposal-only tranche.
2. After durable proposal landing, accept, reject, or amend DAG-009.
3. If accepted, separately authorize the root-pointer move.

None of those acts authorizes DEC-092 implementation unless the owner performs
that separate product-work gate.

## Remaining blockers

The 12 unchanged candidates and five holds in `EDGE_DISPOSITIONS.csv` remain
recorded. Selectability must be re-derived only after accepted activation.

## Rerun requirements

Rebuild and re-evaluate if approved DAG-008, any local consumer register/index,
any target or consumer evidence file, any bundle hash, decomposition truth,
root pointer, portability policy, or any classified target hash changes before
materialization or owner acceptance.
