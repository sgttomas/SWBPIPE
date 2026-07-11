# R0b Pilot Notes — DEL-09-01 Mechanics benchmark suite

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 / R0B_CALIBRATION (activation D-41/DEC-073)
**Pilot posture:** second calibration round under the provisional convention set
(`R0B_CONVENTIONS.md`). All dispositions are agent dispositions, never owner or
engineering rulings. All evidence reads from the frozen worktree at
`551f84ef6be656f1603ce0acfa5e3935aa9683c7`; frozen tree verified clean
(`git status --porcelain` empty) after this pilot's side-effect-free
re-executions. No frozen-tree file was modified.

**Ledger:** `R0B_CLAIM_CONCORDANCE_DEL-09-01.csv` — 25 rows.
Disposition histogram: ALIGNED 15, STALE_SETUP_SPECIFICATION 4,
PARTIALLY_IMPLEMENTED 3, ACCEPTED_DIVERGENCE 1, UNKNOWN 1,
REMAINING_STATE_MISMATCH 1.
Row types: REQUIREMENT 7 (RQ-001..007), EXCLUSION 1 (C01), DECLARED_STATE 8
(C02–C06, C08–C10), ACCEPTANCE 7 (C11–C17), REMAINING_WORK 2 (C07, C18).
IMPLEMENTED_UNMAPPED rows: 0 (see convention 8 below).

## 1. Sources read

Deliverable folder (frozen tree, full four-document kit and context):
`Specification.md`, `Datasheet.md`, `Guidance.md`, `Procedure.md`,
`_CONTEXT.md`, `_STATUS.md` (incl. `## Remaining` and History), `_REFERENCES.md`,
`_DEPENDENCIES.md`, `Dependencies.csv`, `MEMORY.md` (all entries 2026-05-02 →
2026-06-21), `_REVIEW.md` (DEV-001 PKG-02 audit + TP-PKG09-READINESS-GATE
SELF_CHECK), `Review_Findings.csv`, and `_run_records/**` (readiness-gate
review run, TP-R4-D9-BRANCHASSEMBLY-001, TP-VERIFY-012E, parent fan-ins).

Implementation/verification/validation surfaces (frozen tree):
`validation/benchmarks/mechanics/**` (Cargo.toml, src/lib.rs, README.md,
fixtures/tp_phys_014_canonical_analytical_payload.json),
`validation/hand_calcs/mechanics/**` (README + all 22 witness notes, sampled in
depth: `tp_pmm_p3_occloadgen_equivalent_static.md`),
`validation/evidence/sweeps/` (newest sweeps, incl.
`SWEEP_20260711T040758Z_e648462f1d05.json` parsed for commit binding, surface
statuses, and dirty-tree flags).

Authority and homing surfaces (frozen tree):
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 rows DEC-024, DEC-026,
DEC-046, DEC-054, DEC-060, DEC-066, DEC-070; `docs/VALIDATION_STRATEGY.md`
(§§1–6 headings; §2 benchmark families, §5 benchmark source rule),
`docs/SPEC.md` (§4.x/§9 headings), `docs/_Registers/ScopeLedger.csv` row
SOW-026; `plans/PLAN_2026-06-17_prd_completion.md` §3 (D9 rows, D9 exit-refresh
row); `## Remaining` sections of candidate homes DEL-09-02, DEL-09-04,
DEL-09-05, DEL-08-04, DEL-10-05.

Method files (working tree, as directed): `R0B_CONVENTIONS.md`; pinned plan
§§3/6/7 (read in full at the frozen revision); `RUN_BASIS.md`;
`R0_CALIBRATION/R0_REVIEW.md` (rationale for the conventions, per the
conventions file's own pointer).

Re-executions (side-effect-free, per convention 7):
- `cargo test --manifest-path validation/benchmarks/mechanics/Cargo.toml
  --locked` at the frozen SHA with an **external `CARGO_TARGET_DIR`**
  (scratchpad): **33 passed / 0 failed / 0 ignored**. Frozen tree
  `git status --porcelain` clean afterward.
- Read-only greps: zero `domains/piping-design` references under
  `validation/benchmarks/mechanics/**` and `validation/hand_calcs/mechanics/**`
  (equation-source boundary clean); `PKG09-0901-PKG02-001` appears only in
  DEL-09-01-local surfaces, in no `## Remaining` anywhere.
- Read-only git checks: `git diff e648462..551f84ef6` over `validation/` and
  `core/` is empty except the sweep JSON itself (sweep-commit surfaces
  content-identical to frozen state); `DEV-001_DISPATCH_DEL-09-01.md` absent
  from the entire frozen checkout (`find`).

## 2. Validation-class observations specific to this deliverable

- **Provenance:** every reference value in the suite is project-original
  invented mechanics with per-fixture `public_original` provenance structs and
  per-note provenance sections; grep-verified zero PDF/OCR
  `domains/piping-design` contamination. The equation-source boundary is clean
  across all 25 rows.
- **Source-state binding of cited runs:** the newest DEC-025 sweep binds commit
  `e648462f1d05` (an ancestor of the frozen SHA). I proved content-identity of
  the swept `validation/` + `core/` surfaces to the frozen state (diff empty
  except the sweep artifact itself) and *additionally* re-executed the crate
  tests at the frozen SHA, so no `ALIGNED` row rests on overtaken evidence
  (convention 8 / R0 FN1 checked, not triggered).
- **Verification-vs-validation for a validation deliverable:** the crate's 33
  tests *verify the benchmark harness* (inventory completeness, README
  mirroring, unit-basis presence, provenance presence, TBD-marker discipline,
  forbidden-reliance scan) and *execute the comparisons*; the hand-calculation
  witnesses are the validation basis for the mechanics numbers. I kept the two
  in separate columns on every row.

## 3. Per-convention report (did it apply / unambiguous / changed encoding)

**Convention 1 — stale-prose two-signal split.** APPLIED, and it changed the
encoding. DEL-09-01 is the strongest stale-setup case yet piloted: the entire
four-document kit is written in setup-pass future tense while a 22-fixture,
33-test suite exists. Without the convention I would likely have done what the
R0 DEL-04-01 pilot did — `STALE_SETUP_SPECIFICATION` on the requirement rows
themselves — producing a documentation-crisis histogram for a deliverable whose
substance is in good shape. Under the convention the 7 requirement rows carry
substance dispositions (5 ALIGNED, 1 PARTIALLY_IMPLEMENTED, 1 ALIGNED-bounded)
and the staleness is isolated to four per-surface DECLARED_STATE rows
(C02–C05). Unambiguous in application; the R0 FP1 example was additionally
needed to keep the still-valid exclusion row (C01) ALIGNED instead of stale —
that guidance lives in the review, not the condensed convention text, so a
pilot reading only `R0B_CONVENTIONS.md` could still mis-encode exclusion rows.

**Convention 2 — `DECLARED_STATE` ClaimType.** APPLIED clean; 8 rows use it
(C02–C06, C08–C10). The enum extension is exactly what this deliverable needed:
without it, the setup-prose surfaces and the TBD-register declarations would
have been force-fit to REQUIREMENT rows. No ambiguity. One grain question the
convention leaves open: whether an *accurate* current declared-state surface
(the crate READMEs, C06) merits its own row. I included it because it is the
surface that contradicts C02–C05 and carries the record-hygiene observation;
a row-grain rule would prevent drift here.

**Convention 3 — residual homing before mismatch.** APPLIED heavily — this is
the convention this deliverable exercises hardest, and it prevented four false
findings. The suite's declared TBD register (tolerance policy, release
thresholds, CI gates, export/runner integration) looks like omitted residuals
from inside DEL-09-01, but the candidate homes' `## Remaining` sections carry
them: tolerance promotion at DEL-09-04 (gated: owner threshold promotion),
coverage-floor/CI and release-label policy at DEL-09-05, runner payload binding
at DEL-10-05 and DEL-09-04. Encoded as cross-references on C08/C09, no finding.
Both terminal branches were also exercised: C10 (fixture schema TBD) resolved
to *unresolved after checking homes* → `UNKNOWN` + smallest next check;
C18 (pending human disposition of PKG09-0901-PKG02-001) resolved to *homed
nowhere* → genuine `REMAINING_STATE_MISMATCH`. Friction: the convention does
not bound the candidate-home search — I checked five homes chosen by judgment
(TP-VERIFY-012E's recommended-owner list made this tractable); at corpus scale
an unbounded search rule will produce inconsistent diligence between pilots.

**Convention 4 — ID column controlled values.** APPLIED clean. All 25 rows are
`PKG-09/DEL-09-01`; no unmapped rows were emitted, so the `PKG-XX/UNMAPPED`
values were not needed. Ownership hypotheses (e.g. fixture-schema plausibly
DEL-08-04) went to `RemainingWork` cells per the rule. Unambiguous.

**Convention 5 — column defaults and rubrics.** APPLIED with friction in two
places.
- `GateOrStageConstraint` rule: clean. `NONE_RECORDED` on no-residual rows;
  `UNGATED` on C07/C17 because the §16.2 item carries no `(gated:)` suffix.
- `ClaimClass` rubric: **ambiguous for this deliverable's signature claims.**
  RQ-005/C16 (benchmark outputs shall preserve diagnostics/result-envelope
  fields) is simultaneously "diagnostics behavior" (→ WORKFLOW default) and
  "report content" (→ REPORTING); and a case can be made for VALIDATION since
  the claim is about evidence fidelity. I chose REPORTING; without the
  convention I would have chosen VALIDATION. The rubric also never says whether
  benchmark-coverage claims on a validation deliverable are VALIDATION or
  MECHANICS — I used VALIDATION for coverage/harness claims and MECHANICS only
  where the claim asserts numeric/unit correctness (RQ-004/C14). A
  validation-class clause in the rubric would remove this judgment.
- `MECHANICS` + `ValidationEvidence=NOT_APPLICABLE`: not exercised (both
  MECHANICS rows carry populated validation cells).
- `SourceReliability` rubric: unambiguous but **its ladder lacks a rung this
  deliverable sits on.** The technical basis of nearly every mechanics row is
  project-original invented derivations, agent-audited (DEV-001 PKG-02 audit,
  SELF_CHECK gate) with `HumanDisposition=TBD`. That is neither "maintainer-
  vetted" nor "human-reviewed project records", so the rule forces
  `UNVERIFIED` — which I applied on 10 rows, but which reads harsher than the
  actual posture (structural provenance + test-enforced TBD discipline).
  Without the convention I would have used `REVIEWED`. Recommend the rubric
  name the "project-original, agent-reviewed, human-disposition-pending" case
  explicitly, whichever value it is assigned.

**Convention 6 — mechanical selectability + SECURITY encoding.** Selectability
half APPLIED clean: C07/C17 are `YES` by the contract-literal derivation
(IN_PROGRESS, ungated item, no blocking gate), with the owner suspension left
to the RUN_BASIS run-level caveat — exactly the encoding the R0 three-way
contradiction needed. Rows with no recorded residual got `NO` (nothing to
select); the convention doesn't actually state this default — worth one line.
SECURITY half NOT EXERCISED (no SECURITY-class rows in this deliverable).

**Convention 7 — evidence execution and basis resolvability.** APPLIED
heavily, one friction point.
- Re-execution: `cargo test` inside a frozen tree is only side-effect-free if
  the build directory is redirected (`CARGO_TARGET_DIR` external to the tree).
  The convention's "side-effect-free" is undefined for compiled-language
  suites; a pilot without this trick either skips re-execution (weaker
  evidence) or writes `target/` into the frozen worktree (ignored by git, so
  the mandated `git status --porcelain` check would *not* catch it). Recommend
  the convention name build-artifact redirection explicitly.
- Recorded-pass marker: applied verbatim on the sweep citation ("not
  re-executed at frozen SHA 551f84ef6"). Friction: the convention has no slot
  for the *stronger-than-recorded-pass* case I actually had — a recorded pass
  at an ancestor commit whose relevant surfaces are proven content-identical
  to the frozen state by an empty diff. I recorded the proof as free-text
  annotation next to the marker; a standardized "content-identical at frozen
  SHA (diff empty over <paths>)" qualifier would make these rows queryable.
- Basis resolvability: applied clean. The DEV-001 implementation dispatch for
  this deliverable does not exist in the frozen tree (same A2 pattern as R0
  DEL-04-01) → encoded with the `ATTESTED:` marker on all rows that lean on
  it, citing MEMORY.md 2026-05-02 + TP-RECON-01 (commit b34ecd6). All DEC-XXX
  bases cited (DEC-026, DEC-046, DEC-054, DEC-060, DEC-070) were read and
  resolve in `SOFTWARE_DECOMP.md` §12 at the frozen SHA, and each governs the
  claim it sits on (no stage-context citation).

**Convention 8 — disposition precedence.** APPLIED, and it changed an
encoding.
- `ACCEPTED_DIVERGENCE` > `ALIGNED`: C07 (the §16.2 residual) is accurately
  recorded in `## Remaining` *and* human-permitted by DEC-054's conditional R4
  gate — both fit; without the precedence rule I would plausibly have written
  `ALIGNED` (accurate residual). Encoded `ACCEPTED_DIVERGENCE` per the rule.
- Overtaken-evidence rule: checked on every ALIGNED row; not triggered,
  because the crate tests were re-executed at the frozen SHA and the sweep's
  surfaces were proven content-identical (see convention 7). Had I relied on
  the sweep alone at its own commit without the identity proof, this rule
  would have forced `STALE_REVIEW_OR_EVIDENCE` on structurally sound rows —
  the "content-identical" qualifier above is what keeps the rule from
  over-firing on ancestor-commit evidence.
- `IMPLEMENTED_UNMAPPED` material-surface grain: applied — result is **zero
  unmapped rows**. The three post-2026-06-21 fixtures added to this
  deliverable's crate by other programs' tranches (D-34/DEC-070 curved-bend
  exit evidence; DEC-068 item 2 occasional-load generation) are sub-surface
  grain inside an already-mapped crate, so they are not unmapped surfaces;
  they surface instead as a record-hygiene observation on C06 (deliverable-
  local MEMORY/_run_records end 2026-06-21 and do not log them; the
  test-enforced READMEs do). Material surfaces adjacent to this deliverable
  (`validation/benchmarks/stress|nonlinear`, `validation/witness`,
  `validation/evidence/**`) are plausibly owned by DEL-09-02/-03/-05 and
  DEL-10-05 and were left to their owners' ledgers.
- `RecordedRemaining` verbatim-only-touched: applied — the §16.2 item is
  copied verbatim on C07 and C17 only (the rows its claim touches),
  `NONE_RECORDED` elsewhere.
- D-41 gate-state codification: applied — the bootstrap item
  "Run claim-level concordance per the reconciliation method (source:
  plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md §§6–8 at
  the D-41-pinned main revision) (gated: D-41)" is quoted here verbatim per
  the special rule, excluded from residual analysis, and no per-pilot
  re-derivation of the D-41 time-split was done (RUN_BASIS codifies it:
  frozen register AWAITING_RULING; run authority is the live RULED row).

## 4. Per-convention one-line verdicts

1. Two-signal split — **applied-clean** (changed encoding vs. no-convention
   baseline; exclusion-row guidance lives only in the review, not the
   condensed text).
2. `DECLARED_STATE` ClaimType — **applied-clean** (8 rows; row-grain for
   accurate declared-state surfaces unstated).
3. Residual homing — **applied-with-friction** (prevented 4 false mismatches,
   exercised both terminal branches; candidate-home search unbounded).
4. ID column controlled values — **applied-clean** (UNMAPPED values not
   needed).
5. Defaults and rubrics — **applied-with-friction** (ClaimClass rubric
   ambiguous for result-envelope and validation-suite claims;
   SourceReliability ladder lacks the project-original/agent-reviewed rung).
6. Mechanical selectability + SECURITY — **applied-clean** on selectability
   (SECURITY half not-exercised; `NO` default for no-residual rows unstated).
7. Evidence execution + basis resolvability — **applied-with-friction**
   ("side-effect-free" undefined for compiled suites — build-dir redirection
   needed; no slot for content-identical ancestor-commit evidence; ATTESTED
   marker worked as designed).
8. Disposition precedence — **applied-clean** (changed C07 to
   ACCEPTED_DIVERGENCE; overtaken-evidence rule checked and correctly not
   triggered; zero unmapped rows at material grain).

## 5. Residual ambiguities the conventions do NOT cover

1. **Self-evidence for validation-class deliverables.** This deliverable's
   `ValidationEvidence` cells cite its own products (hand-calc witnesses) —
   the suite *is* the evidence infrastructure. The conventions never say
   whether a validation deliverable's validation basis may be deliverable-
   internal, or what independent leg (here: the DEV-001 PKG-02 audit,
   SELF_CHECK gate, and DEC-026 governed tier) must accompany it. I cited
   both legs on every such row; a stated rule would prevent silent
   self-certification at scale.
2. **Acceptance-table row grain.** The Specification's Verification table
   restates the requirements at acceptance grain; extracting both (7 RQ + 7
   ACCEPTANCE rows) doubles the weight of this deliverable in any histogram.
   A dedup/mirror rule (e.g. acceptance rows cross-reference rather than
   re-disposition) would stabilize package summaries.
3. **Deliverable-local record lag on third-party additions.** Other programs
   (DEC-070, DEC-068) lawfully wrote fixtures into this deliverable's owned
   crate without updating its MEMORY/_run_records. Neither
   `STALE_SETUP_SPECIFICATION` (not setup prose), `REMAINING_STATE_MISMATCH`
   (no residual omitted), nor `IMPLEMENTED_UNMAPPED` (sub-surface grain) fits;
   I parked it as a `RemainingWork` observation on C06. A named
   record-hygiene encoding (or an explicit "not ledger-worthy" ruling) is
   needed.
4. **`AuthorityNeeded` semantics on ALIGNED declared-state rows** whose TBDs
   resolve only through future owner rulings homed elsewhere (C08/C09): I
   used `NO` (nothing owed *on this row*), but `OWNER` is defensible; the
   column contract does not say whether it tracks the row's finding or the
   claim's ultimate closure.
5. **MECHANICS rows with review-pending validation.** RQ-004/C14 carry
   validation evidence whose human disposition is pending
   (TECHNICALLY_ADDRESSED_PENDING_HUMAN). The conventions cover overtaken
   evidence and absent evidence, but not *pending-disposition* evidence; I
   encoded PARTIALLY_IMPLEMENTED on independent grounds (unit-catalog grain),
   so nothing hinged on it here, but a corpus rule will be needed.

## 6. Deferred claims

None deferred. The one non-settled row is C10 (fixture-schema TBD homing),
encoded `UNKNOWN` with the smallest next check per convention 3 rather than
deferred: confirm at R3 whether DEL-09-01's §16.2 evidence-system residual
subsumes fixture-schema selection or whether a DEL-08-04/schema-owner
`## Remaining` item is missing.
