# D-53 — One-File Raw-Evidence Whitespace Exception for Terminal FAIL Closeout

**Status:** RULED; `READY_FOR_COMMIT`
**Date:** 2026-07-18
**Owner:** Ryan Tufts
**Codification:** `DEC-086` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12
**Managed run:**
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3/`

## 1. Matter and Ruling

CHANGE staged the terminal `FAIL` evidence tranche for
`HELP-HUMAN-PIPING-20260718-DEL0904-EXEC-R3`. The staged command
`git diff --cached --check` exits `2` solely with:

```text
projects/chirality-piping/validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/stdout/cargo_test.txt:42: new blank line at EOF.
```

The file is immutable raw captured stdout. Editing it would invalidate its
recorded checksum and falsify exact capture. The owner therefore rules the
following exact one-off exception.

<!-- BEGIN OWNER RULING VERBATIM D-53 -->
APPROVE: record and codify a one-file raw-evidence whitespace exception for stdout/cargo_test.txt:42; preserve its exact bytes and checksum, and commit the terminal FAIL evidence tranche. This grants no reproduction acceptance, lifecycle, receipt, release, or external effect.
<!-- END OWNER RULING VERBATIM D-53 -->

Canonical SHA-256 of the 276 UTF-8 bytes between the markers, excluding the
marker lines and adjacent delimiter newlines:
`87f4c4e17af2be6518517740b3ff4ec3f800decc708fbfec21e4ea110aa36602`.

## 2. Exact Exception Boundary

The exception applies only when all of these remain true at commit closeout:

1. The sole staged whitespace finding is the exact path, line, and message in
   §1. Any second finding returns the closeout to `HOLD`.
2. The raw file remains exactly 2,095 bytes with SHA-256
   `2e4793f43ee28ed96122cf1e967e8950a8e2680ba55afce30662f9e9c88bd962`.
3. `SHA256SUMS.txt` remains exactly 9,422 bytes with SHA-256
   `2c21d2a9fdd15488a3284faf8ab17a427f7f9be2681f099fc7ce7ee5c34a0d14`
   and continues to bind `./stdout/cargo_test.txt` to the hash above.
4. The single sweep artifact remains exactly 3,274 bytes with SHA-256
   `107b5d8d81f16f531f5928ac79c557702c1278235de9f9514380dbaba791af9d`.
5. Bundle checksum verification, JSON parsing, receipt validation, staged path
   containment, and the protected no-diff checks remain passing.
6. The committed tranche remains terminal `FAIL`; DEL-09-04 remains
   `IN_PROGRESS` with its clean-reproduction Remaining item open; Receipt-55
   remains latest.

The command result remains truthfully `exit 2`. The closeout gate is recorded
as `PASS_WITH_OWNER_EXCEPTION_DEC_086`, not as a command PASS. This ruling
does not waive `git diff --check` generally, normalize generated/raw output,
authorize any other whitespace finding, or create precedent for another file,
line, bundle, run, or commit.

## 3. Authorized and Excluded Effects

Authorized: CHANGE may commit the exact terminal `FAIL` evidence tranche plus
this ruling, its decision-register row and codification, and the minimum R3
closeout records that cite it.

Not authorized: editing any file under
`validation/evidence/reproduction/REPRO_DEL0904_20260718T215424Z_f14fa77518a/`,
editing the sweep artifact, rerunning reproduction, appending a receipt,
changing DEL-09-04 status/Remaining or lifecycle, accepting the reproduction,
promoting evidence posture, release, push, merge, publication, prover action,
or any external effect.

## 4. Review Applicability

No additional independent governance review is required. This record
transcribes and bounds a completed case-specific owner ruling; it does not
amend D-52/`DEC-085`, Shared-Block v1, D-49/D-50/D-51, the workplan, or a
cross-project instrument. DEC-083 S5 applies only at instrument-amendment time.
Deterministic closeout verification remains mandatory and fails closed on any
boundary drift.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
