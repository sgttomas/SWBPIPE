# DEL-04-04 — R0 calibration notes

Worker: TASK (Type 2), run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`. These
are calibration notes about the candidate conventions, not findings anyone has
accepted. Agent dispositions are not owner rulings. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

- Forward (sealed): `DEL-04-04_forward.csv`, SHA-256
  `3faa620bf95c52c02684cf4f8f22e767e3c60e55d81069cb44cbe14552b5cb8e`, 70 rows.
- Reverse: `DEL-04-04_reverse.csv`, 96 answers.
- Selectability: every row is `NOT_APPLICABLE`. Since 2026-09-19 Piping
  selects work through owner-steered work graphs, not `## Remaining` (C9).

## Path aliases

- `NS` = `projects/chirality-piping/core/solver/nonlinear_supports/src/lib.rs`
  (Cargo package `open_pipe_stress_nonlinear_supports`). This is the
  deliverable's own crate.
- `NI` = `core/solver/nonlinear_integration/src/lib.rs`. This is the DEC-044
  integration tranche, named in the SOW as `core/solver/nonlinear_integration`.
- `HB` = `core/runner/headless/src/result_envelope_binding.rs`. This is the
  R14 producer binding.
- Test cases are cited as `<file>::<fn>`. Rust unit tests live inside `lib.rs`
  (there are no `tests/` directories).
- The four documents `Datasheet.md`, `Specification.md`, `Guidance.md` and
  `Procedure.md` are now aliased to `ScopeOfWork.md`. PR #229 (commit
  `fcb21c717`) deleted them and folded their content into the SOW.
- The product name OpenPipeStress in crate and package names is the pre-SCA-010
  name. I made no identity finding on it.

## Judgment calls

- **Section wrappers.** The ontology, epistemology, praxeology and axiology
  sections span several CLM blocks. I recorded each as `NON_NORMATIVE`, not
  `CONTAINER`, because the CLM keys name the SOW, not the section, as their
  parent.
- **Setup-procedure blocks (CLM-015 to CLM-019).** I typed these `HISTORY`
  because CLM-007 and CLM-027 retain setup-era text "as historical setup
  context where applicable". Where the text is present tense and contradicts
  the frozen state, I still recorded `STALE_SETUP_SPECIFICATION` (CLM-015, 017,
  018, 019). CLM-016 is accurate as history, so I recorded it `ALIGNED`.
- **REQ-08.** I recorded `STALE_REVIEW_OR_EVIDENCE` rather than
  `PARTIALLY_IMPLEMENTED`. The producer code is present, but the binding tests
  from PR #292 were removed by PR #787 (commit `b43cc00c4`), which cut
  `result_envelope_binding.rs` from 8 tests to 2. No file outside `execution/`
  references the `NONLINEAR_ASSEMBLED_LOOP_*` codes. I gave the row
  `BaselineClass=PROTECTED_CHECK` because a removed verification check is a
  baseline event. I did not diagnose it as a defect.
- **REQ-06.** I recorded `PARTIALLY_IMPLEMENTED` with baseline class
  `OWNER_HOLD` (PDU-035). `NonlinearSupportUnitMetadata` exists, but no caller
  outside its crate builds it. Only the dimension check is tested; I found no
  tests for rejected or missing metadata.
- **Hand-calculation witnesses.** I listed them under `ValidationEvidence`
  with reliability `UNVERIFIED`. They are invented oracles authored in the
  repository, and no named human ruling covers the DEC-067 set. DEC-052 covers
  only the earlier thirteen-fixture evidence.
- **Remaining items R01 to R07.** All seven are `ALIGNED`: each text matches
  the frozen state. Per A4, none is cited as evidence that work is open.
- **CLM-002.** It is an identification table, so I recorded it `NOT_ASSESSED`
  as C1 directs, even though its "decomposition revision 0.7" cell is stale.
  That staleness is carried on the SOW surface row.

## Convention friction (smallest fix)

1. **Gate evidence cannot go in evidence columns.** `GATE_EVIDENCE/` postdates
   the freeze, so the validator rejects it as a path in the frozen tree. I
   cited it in `ContextRefs`. Fix: allow a `GATE:` token prefix in
   `VerificationEvidence` that the validator resolves against the run
   directory.
2. **Section wrappers versus CONTAINER.** C1 assumes a block's children are its
   `ParentKey` children. The extractor instead parents CLM blocks to the SOW,
   so the section blocks have no children and duplicate their content. Fix:
   have the extractor either parent CLM blocks to their section or emit
   sections as `NON_NORMATIVE` by default. Alternatively, state in C1 that
   section wrappers are `NON_NORMATIVE`.
3. **Mis-parented items.** The extractor parents AC-001 to CLM-013 and VER-001
   to CLM-019, although each sits outside those blockquotes and is unrelated to
   their content. Fix: parent unquoted `AC`/`VER`/`OUT` bullets to their `##`
   section.
4. **HISTORY has no disposition rule.** C2 allows `HISTORY` outside `MEMORY`,
   but C6 does not say what disposition a history block takes: `ALIGNED` when
   accurate, stale when present tense? Fix: add one line — HISTORY rows are
   `ALIGNED` when accurate as history, and stale only where they still read as
   current obligations.
5. **Removed test versus untested code.** A test that existed and was deleted
   is a different fact from code that was never verified. No disposition or
   cause separates them; `EVIDENCE_OVERTAKEN` comes closest. Fix: add a cause
   tag `VERIFICATION_REMOVED`, or require `BaselineClass=PROTECTED_CHECK` with a
   Notes pointer.
6. **Mixed blocks.** One stale row inside a mostly aligned block (CLM-003)
   forces a single stale disposition unless I split it into `.sNN`
   sub-claims. Splitting blocks with several table rows is allowed but never
   triggered. Fix: guidance on when a split is expected, for example when rows
   would take different dispositions.
7. **Tier for setup-era mentions of rulings.** Stale text that restates a
   ruling (REQ-09, CLM-005) could be `PROJECT_BASELINE`. But the fix is a
   deliverable catch-up, which the profile describes under `LOCAL_DESIGN`. I
   used `PROJECT_BASELINE`. Fix: define the tier by what the text asserts
   today, not by which ruling overtook it.
8. **One capability, several keys.** The Part D reverse schema allows one
   `ClaimKey` per answer. CAP-SOLVER-015 fits both REQ-03 and REQ-08. Fix:
   allow a semicolon list.

## Unit grain

For the SOW the grain was mostly right. REQ items and CLM blocks map to
distinct claims. The cost is 13 structural rows (9 `NOT_ASSESSED` and 4
`COVERED_BY_CHILDREN`) out of 70. The duplicate CLM-007 and CLM-027 doubled
one claim. The CONTEXT blocks are very fine-grained: scope-coverage,
objective-support and envelope are one-line identity facts that could be a
single row. MEMORY as one unit worked. It could not show that a MEMORY entry's
cited tests had since disappeared, but REQ-08 caught that.

## UNKNOWN rows

None. The nearest to unknown is REQ-08. The smallest next check: read the
PR #787 diff to see whether envelope-level nonlinear-context coverage moved
somewhere my search did not find (for example a Python or TypeScript test
reading the envelope diagnostics under another name).

## Did the reverse pass change my view?

Only a little. I made no edit to the sealed file.

- **Ownership gap.** CAP-SOLVER-016, 017, 019 and 020 live in
  `nonlinear_integration`, which DEC-044 places outside DEL-04-04. Yet no
  deliverable key owns that tranche, and DEL-04-04's CLM-007 and CLM-012 claim
  the DEC-067 bounded-friction and seed-independence behaviour that exists only
  there. My forward rows cite `NI` as implementation evidence throughout. That
  stays right for this deliverable's claims, but R3 should treat
  `nonlinear_integration` as an ownership gap, not as DEL-04-04's code.
- **Contact initialisation.** The 2026-09-13 contact-initialisation work
  (CAP-SOLVER-019) was anchored to DEL-04-04 in MEMORY but is named by no SOW
  key. With hindsight, the SOW surface row could have noted it as
  `IMPLEMENTED_UNDOCUMENTED`.
- **REQ-09 staleness confirmed.** According to the inventory's description,
  which I did not check against the code, CAP-SOLVER-027 shows the diagnostics
  crate still emits a standing "sparse default adoption unresolved" diagnostic even
  though DEC-053 promoted sparse to the default. That tension belongs to the
  diagnostics owner, but it supports my REQ-09 row. REQ-09 is stale text, and
  code in another deliverable carries the same stale status.
