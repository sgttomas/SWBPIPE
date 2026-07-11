# R0b Convention Set Under Test

**Status:** PROVISIONAL — calibration-only. The owner directed a second R0
round on 2026-07-11 (in-session slate answer, verbatim: "Another R0 round
first" — "Run a second small calibration sample under the proposed
conventions to verify they resolve the divergences before committing the
corpus."). These conventions are the R0 reviewer's non-binding
recommendations (`R0_CALIBRATION/R0_REVIEW.md` §§5–6, 8) transcribed as the
encoding rules R0b pilots MUST follow, so R0b can test whether they resolve
the R0 divergences. They are not adopted for R1+; adoption is a separate
owner call after R0b. The pinned plan revision
(`551f84ef6be656f1603ce0acfa5e3935aa9683c7`) remains the method authority;
where these conventions refine §6/§7 encodings they bind R0b ledger cells
only.

## Conventions (from R0_REVIEW.md, condensed — read the review for rationale)

1. **Stale-prose two-signal split (D1).** Requirement/acceptance/exclusion
   rows always carry the SUBSTANCE disposition (`ALIGNED`,
   `PARTIALLY_IMPLEMENTED`, `DOCUMENTED_UNIMPLEMENTED`, …). Declaration
   staleness is ledgered on separate per-surface declared-state rows;
   `STALE_SETUP_SPECIFICATION` is reserved for those rows. §7 `ALIGNED`'s
   "declaration" means the deliverable's current declared-state surfaces,
   not the requirement sentence's own tense.
2. **`DECLARED_STATE` ClaimType (D2).** Use `ClaimType=DECLARED_STATE` for
   declared-state claim rows (R0b tests this enum extension).
3. **Residual homing before mismatch (D3).** Before `REMAINING_STATE_MISMATCH`
   for an omitted residual, check the candidate homes' `_STATUS.md`; homed
   elsewhere → no finding + cross-reference; unresolved → `UNKNOWN` with the
   smallest next check.
4. **ID column controlled values (D4).** One merged `PackageID/DeliverableID`
   column; values only `PKG-XX/DEL-XX-XX`, `PKG-XX/UNMAPPED`, or `UNMAPPED`;
   ownership hypotheses go in `RemainingWork` or notes, never the ID cell.
5. **Column defaults and rubrics (D5, T2).** `GateOrStageConstraint`:
   `NONE_RECORDED` when no residual exists; `UNGATED` only for an existing
   residual lacking a gate suffix. `ClaimClass` for diagnostics/warning
   behavior: `WORKFLOW` unless the claim asserts numeric mechanics (then
   `MECHANICS`) or report content (then `REPORTING`). A `MECHANICS` row may
   carry `ValidationEvidence=NOT_APPLICABLE` only with an explicit in-cell
   reason. `SourceReliability`: `VETTED` only for maintainer-vetted technical
   sources named as such; `REVIEWED` for human-reviewed project records;
   `UNVERIFIED` otherwise; `NOT_APPLICABLE` for non-technical prose rows.
6. **Mechanical selectability + SECURITY encoding (D6, T3).**
   `SelectableUnderCurrentLoop` is derived mechanically from DAG/lifecycle/
   gate rules only (contract-literal); the owner suspension is a run-level
   caveat in `RUN_BASIS.md`, never a per-row `UNKNOWN`. SECURITY-class
   behavior claims whose accepted scope defers sufficiency review:
   `ValidationEvidence=NONE_FOUND — sufficiency review deferred, owner-gated`;
   no `VERIFIED_NOT_VALIDATED` downgrade on that ground.
7. **Evidence execution and basis resolvability (D7, A2–A4).** Re-execute
   only side-effect-free checks inside the frozen tree (verify tree clean
   after); otherwise cite the recorded pass with run record + commit binding
   plus the standardized marker `not re-executed at frozen SHA 551f84ef6`.
   A `DecisionBasis` must resolve to an artifact in the evidence tree, or be
   marked attestation-level (`ATTESTED: <where recorded>, record not present
   in tree`); it must be a decision that governs the claim, not stage
   context.
8. **Disposition precedence (D8, D9, FN1, FP2).** `ACCEPTED_DIVERGENCE` >
   `ALIGNED` when both fit a residual row. A row whose cited class-required
   validation evidence is overtaken for the frozen source state may not be
   plain `ALIGNED` — it takes `STALE_REVIEW_OR_EVIDENCE` unless independent
   bind-current evidence is also cited. `IMPLEMENTED_UNMAPPED` is
   material-surface-grained (crates, binaries, schemas, app panels), not
   internal functions plausibly inside an existing requirement.
   `RecordedRemaining` is copied verbatim only on rows the residual's claim
   touches; `NONE_RECORDED` elsewhere. The D-41 frozen-tree observability
   caveat is stated once in `RUN_BASIS.md` — gate-state cells reflect the
   frozen register state without per-pilot re-derivation.
