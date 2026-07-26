# D-57 — R18–R20 Staged-Whitespace Closeout Exception

**Status:** RULED; `READY_FOR_COMMIT`
**Date:** 2026-07-25
**Owner:** Ryan Tufts
**Codification:** `DEC-090` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12
**Managed ruling run:**
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-R18-R20-GIT-EXCEPTION-R21/`

## 1. Matter and owner ruling

CHANGE staged the terminal R18–R20 tranche. Its ordinary staged whitespace
check remains truthfully exit `2` because 26 evidence-bound Markdown/JSON
files preserve a second terminal newline. The owner authorized this exact
case-specific closeout:

<!-- BEGIN OWNER RULING VERBATIM D-57 -->
APPROVE: record a one-off, nonprecedential exception for exactly the 26 staged “new blank line at EOF” findings; preserve the evidence-bound bytes and commit the staged R18–R20 tranche; do not push.
<!-- END OWNER RULING VERBATIM D-57 -->

Canonical SHA-256 of the exact 204 UTF-8 bytes between the markers, without a
trailing newline:
`c5854dbbf128b4dd0b3fcb864359173eb1832f0c41cddee936ed41175cc10e49`.

This ruling follows the bounded closeout pattern of D-53/`DEC-086` and
D-55/`DEC-088`, but neither reuses nor extends either exception.

## 2. Exact exception boundary

The exception applies only while every predicate below remains true at commit
closeout:

1. Branch is `codex/piping-candidate-briefs-20260725`; source HEAD is
   `2f8d35ceb30da734ca6dff24dcab36dded8c9b35`.
2. The pre-governance index contains exactly 105 newline-delimited paths.
   Their exact ordered list is
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-R18-R20-GIT-EXCEPTION-R21/PRE_GOVERNANCE_STAGED_PATHS.txt`,
   SHA-256
   `3652393639a6d41dfef45325ccf7ac5f0bd945c7ce346990594b95080be26202`.
3. `git diff --cached --check` remains truthfully exit `2` with exactly the
   26 path/line/message findings in
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-R18-R20-GIT-EXCEPTION-R21/PRE_GOVERNANCE_DIFF_CHECK.txt`,
   SHA-256
   `b70bb7c7ab50aa47b2ee5e43e2ea4efe3f5840b3db74348b4e6ae06487b3aaf8`.
   Every line ends with the exact message `new blank line at EOF.` No 27th,
   different, or additional finding is permitted.
4. The staged bytes of all 26 affected files match
   `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260725-R18-R20-GIT-EXCEPTION-R21/AFFECTED_STAGED_SHA256SUMS.txt`,
   which has exactly 26 unique path/hash rows and SHA-256
   `7ac60894469af980142f8585f350aaca7596d3a2b219a32d79c4d04e4416b7b6`.
5. The exception-governance addition is limited to this ruling, the single
   D-57 register row, the single `DEC-090` append, Receipt-75, and the R21
   managed subtree. Every new text file has exactly one newline at EOF and
   adds no whitespace finding.
6. At commit-ready closeout, all intended R18–R21 tranche paths are staged,
   with no unstaged or untracked tranche content. Any scope change, partial
   staging, or extra path outside the bound pre-governance inventory and the
   exact governance addition returns closeout to `HOLD`.
7. The terminal semantics of R18, R19, and R20 remain unchanged: the D-56 and
   D-06b rulings, DEL-11-01 guide-only execution, DEL-07-06 prerequisite hold,
   DEL-08-01 proposal-only stop, App ID result, unsigned artifact posture,
   residual gates, and Receipts 71–74 retain their recorded meanings.
8. Receipt validation, JSON parsing, claims/path checks, protected-hash
   checks, and exact staged-scope verification remain passing apart from the
   exact bound 26 whitespace findings.

Any byte/hash/count/path/finding drift, a 27th or different finding,
unstaged/untracked tranche content, scope change, protected-state drift, or
failed validation returns closeout to `HOLD`. The command is never represented
as PASS. The case-specific gate is
`PASS_WITH_OWNER_EXCEPTION_DEC_090`.

## 3. Authorized and excluded effects

Authorized: after fresh independent verification, CHANGE may stage and commit
the exact 105-path R18–R20 base plus the bounded D-57/`DEC-090`/register/R21/
Receipt-75 governance addition while preserving the 26 affected staged blobs
byte-for-byte.

Not authorized: normalizing or otherwise editing any of the 26 bound files;
altering the substantive R18–R20 tranche; product/configuration, deliverable
lifecycle, DAG, build, packaging, signing, notarization, publication, release,
or external action; push, merge, or network use.

This exception is one-off and nonprecedential. Git closeout does not adopt a
candidate, change a ruling, clear a hold, accept a lifecycle transition, or
alter any recorded external result.

## 4. Review applicability and verification

DEC-083 S5 does not apply. This record transcribes and bounds a completed
case-specific owner ruling; it does not amend a standing instrument,
Shared-Block v1, D-53/`DEC-086`, or D-55/`DEC-088`.

Fresh read-only Agent 2 verification and deterministic closeout checks remain
mandatory and fail closed on any boundary drift.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
