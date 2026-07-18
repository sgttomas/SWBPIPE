# WORKING_ITEMS Run — DEL-09-04 Clean Reproduction

- Date: `2026-07-18`
- RunID: `HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3`
- ChildInstanceID: `TASK-DEL0904-CLEAN-REPRO-01`
- ReproductionID: `REPRO_DEL0904_20260718T215424Z_f14fa77518a`
- Source commit: `f14fa77518a06f112ae72a8fcce4de0fab958d47`
- Committed candidate SHA-256: `57b9cb228b970ce2d1c3d58f6b888bc57f4b81a5afd6c718dad7124c76b27aed`
- Disposition: `FAIL`

## Result

The clean local checkout and the three bounded E1 runner cases passed their
mechanical predicates: initial/final tracked state was clean; generator output
matched the three committed fixtures; exits were `0`, `1`, and `1`; the solve
completed with 830 non-empty result references and no diagnostics; and both
blocking cases emitted their required diagnostic with no solver result in the
validation case. The headless-runner Cargo tests and Python contract test also
passed.

The tranche is nevertheless `FAIL`. The one mandatory registered evidence
sweep exited `1`: its desktop Vitest surface returned exit `127`, after three
of five surfaces ran. Its internally dispatched Cargo commands were not
recorded with the brief-required `--offline` flag and the retained log contains
`Updating crates.io index`; therefore the absolute offline/no-external-contact
contract was not established and may have been violated. This is not repaired
or provisioned in this tranche.

## Evidence and Preserved State

- Immutable derivative bundle:
  `validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/`
- Single sweep artifact:
  `validation/evidence/sweeps/SWEEP_20260718T220018Z_f14fa77518a0-dirty.json`
- DEL-09-04 remains `IN_PROGRESS`.
- The clean-checkout Remaining bullet remains open; no `_STATUS.md` or
  `MEMORY.md` closeout edit was made.
- No Receipt-56 was appended.
- No code, fixture, test, documentation, schema, tool, governance, DAG,
  decomposition, or workplan repair was made.
- No commit, push, PR, merge, publication, release, lifecycle/stage change,
  reproduction-result acceptance, prover action, or professional reliance act
  occurred.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).

## Retry Observability

One closeout-command retry occurred:

| Attempt | Detection layer | Reason | Exit | Disposition |
|---|---|---|---:|---|
| claims-language 1 | executor command-path check | The root validator was invoked from `WORKING_ROOT`, so `tools/validation/validate_claims_language.py` did not resolve. | 2 | Classified as executor wrong-CWD error; outputs retained as `claims_language_wrong_cwd`; no project repair. |
| claims-language 2 | root validator | Repeated once from the prescribed `REPO_ROOT` with identical validator arguments. | 0 | Corrected invocation passed; retry does not alter the terminal FAIL basis. |

The terminal FAIL remains based on the mandatory evidence-sweep failure and
the unproven/apparently violated offline posture, not this corrected path
invocation.

## Rerun Trigger

A fresh run ID is required after the evidence-sweep environment is made
locally complete and the registered check can itself satisfy the candidate's
offline/no-install contract. The rerun remains governed by candidate §8 and
must not overwrite this failed bundle.
