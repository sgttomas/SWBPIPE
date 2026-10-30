# D-55 — R8 Terminal-FAIL Raw-Evidence Whitespace Exception

**Status:** RULED; `READY_FOR_COMMIT`
**Date:** 2026-07-19
**Owner:** Ryan Tufts
**Codification:** `DEC-088` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12
**Managed ruling run:**
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-DEL0904-R8-GIT-EXCEPTION-R9/`
**Terminal evidence run:**
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260718-DEL0904-CLEAN-REPRO-R8/`

## 1. Matter and owner ruling

The immutable R8 reproduction is terminal `FAIL`, and CHANGE found that its
exact 93-file staged tranche cannot pass the ordinary whitespace check because
the captured Cargo stdout ends with one raw blank line. HELP_HUMAN explained
that D-53/`DEC-086` is R3-specific and cannot be reused, but that a new
R8-specific exception could authorize Git closeout only when bound to the sole
R8 finding, exact raw/checksum hashes, exact staged inventory, truthful exit
`2`, and all preserved terminal-FAIL boundaries. The owner then responded:

<!-- BEGIN OWNER RULING VERBATIM D-55 -->
You may proceed accordingly.
<!-- END OWNER RULING VERBATIM D-55 -->

This is explicit adoption by reference of the proposed new case-specific
exception. Canonical SHA-256 of the exact 28 UTF-8 bytes between the markers,
without a trailing newline:
`91bcd18370d6004c9d9c4a2b7b7de1fc65b5ad71078c9ccb6bbdf854d34cd54d`.

This ruling is new and R8-specific. It does not reuse, extend, reinterpret, or
create precedent from D-53/`DEC-086`.

## 2. Exact exception boundary

The exception applies only while all of these remain true at commit closeout:

1. Branch is `codex/piping-del0904-clean-repro-20260718-r8`; HEAD is
   `89a93d7ca21d64c57cc344955d17deef709fd685`.
2. The pre-governance R8 inventory is exactly 93 staged added paths: 83 fresh
   bundle files including `SHA256SUMS.txt`, one sweep, one DEL-09-04 run
   record, and eight R8 managed-run files. Its bytewise-sorted,
   newline-delimited path-list SHA-256 is
   `9f2e0415cbfdbc815d4af6609b7b12fd7091a5af39fd6f335d07c18d355cc012`.
3. `git diff --cached --check` remains truthfully exit `2` with exactly this
   sole finding and no second finding:

   ```text
   projects/chirality-piping/validation/evidence/reproduction/REPRO_DEL0904_20260719T033249Z_89a93d7ca21d/stdout/cargo_test.txt:42: new blank line at EOF.
   ```

4. That raw stdout remains exactly 2,095 bytes with SHA-256
   `625969f5f2c2dcf8a450818ce1e370e0aa8f991a70b88ee04a515068829e489e`.
5. Its `SHA256SUMS.txt` remains exactly 7,617 bytes with SHA-256
   `003db610d9bfee47ea73349c81dae79bc196af8ee56aaeb0fb1b85a54bcf7c77`,
   verifies exactly 82 unique payload entries with no missing or extra bundle
   payload, and binds the raw stdout hash exactly once.
6. The single sweep remains
   `validation/evidence/sweeps/SWEEP_20260719T034442Z_89a93d7ca21d-dirty.json`,
   exactly 4,316 bytes with SHA-256
   `9994c20815dd1d5a64d90cf93f9956f61085b896ed93ec9b07017a1da0d3dd64`,
   with all five registered surfaces `pass`.
7. The exact objective remains `FAIL`: `harness-pytest` exit `1`, with 264
   passed and two failed; R8 RETURN, HANDOFF, and STATUS continue to record
   terminal `FAIL`.
8. DEL-09-04 remains `IN_PROGRESS`; its clean-reproduction Remaining item and
   MEMORY remain unchanged; Receipt 56 remains latest and Receipt 57 remains
   absent; protected R3, R7, and P1 history remains unchanged.
9. Bundle checksum verification, JSON parsing, receipt validation, staged
   path containment, and protected no-diff checks remain passing.

Any second finding, size/hash drift, checksum/count failure, staged-scope
drift, or protected-state drift returns closeout to `HOLD`. The check command
is never represented as PASS. The case-specific closeout gate is
`PASS_WITH_OWNER_EXCEPTION_DEC_088`.

## 3. Authorized and excluded effects

Authorized: CHANGE may commit the exact 93 staged R8 terminal-`FAIL` files
plus this D-55 ruling, its register row and `DEC-088` codification, the R9
governance run record, and the minimum R8 manager RETURN/HANDOFF/STATUS updates
that cite the exception.

Not authorized: editing any R8 bundle or sweep file; rerunning reproduction,
tests, or sweep; repairing harness baselines; appending Receipt 57; changing
DEL-09-04 status, MEMORY, lifecycle, or Remaining; altering R3/R7/P1;
accepting reproduction; promoting evidence; advancing lifecycle/stage/release;
push, merge, publication, prover action, or any external effect.

This exception is one-off, nonprecedential, and limited to the exact R8
terminal-FAIL closeout above. Git closeout remains source-control hygiene and
does not alter the evidence or lifecycle verdict.

## 4. Review applicability and independent verification

DEC-083 S5 does not apply. This record transcribes and bounds a completed
case-specific owner ruling; it does not amend Shared-Block v1, a cross-project
instrument, the workplan, D-53/`DEC-086`, or any standing delegation.

Exactly one independent, read-only Agent 2 verifier under the TASK base
contract recomputed every §2 predicate and returned `COMMIT-SAFE`. Its durable
return is under the managed R9 ruling run. Deterministic closeout verification
remains mandatory and fails closed on any later boundary drift.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
