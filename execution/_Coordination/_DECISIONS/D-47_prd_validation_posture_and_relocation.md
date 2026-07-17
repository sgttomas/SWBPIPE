# D-47 - PRD Validation Posture Amendment, PRD Relocation, and Impacted-Deliverable Propagation (SCA-007)

**Status:** PROPOSAL  
**Date prepared:** 2026-07-16  
**Decision ID:** D-47  
**Prepared by:** agent, from the owner's 2026-07-16 in-session direction following the D-46 O-C deferral  
**Owner act required:** yes

## 1. Decision Statement And Scope

Amend the adopted PRD so its validation posture matches the owner's stated
product theory (internal engine generates and screens candidates; an external
professional solver is the canonical validation oracle), relocate the adopted
PRD to `docs/PRD.md`, and propagate the amendment into the impacted
deliverables' scopes of work and `## Remaining` items.

This packet does not: procure or activate CAEPIPE or any external prover
(that remains a future owner-gated register row), advance any lifecycle or
target stage, accept any reproduction run, publish anything, or create a
release-readiness, professional-approval, certification, sealing,
authentication, or code-compliance claim. Ruled history is never rewritten;
all re-keying is forward-pointer and crosswalk work.

## 2. Accepted Basis And Current Evidence

- **Adopted PRD authority:** `docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md`
  per D-21 / `DEC-056` / SCA-005 (accepted 2026-07-04). The v0.1 text at
  `docs/PRD.md` is preserved history carrying a Forward Authority Note
  (SCA-005 amendment action `SCA-005-A001`).
- **The criterion at issue:** v0.2 §24 R6 exit criterion "external reviewers
  can reproduce validation examples" (`OpenPipeStress_PRD_v0.2.md:1581`),
  successor of v0.1 §22.6 "External engineers can reproduce validation
  examples" via the D-21 Annex A milestone crosswalk.
- **D-46 disposition:** D-46 (R5 external-reproduction acceptance protocol)
  was ruled **O-C — defer** on 2026-07-16; the DEL-09-04 residual remains
  open and no clean-checkout run counts toward the criterion. The D-46 row
  and ruling are recorded on the PR #253 branch
  (`codex/piping-r5-repro-ci-gateway`) pending merge.
- **Owner direction (2026-07-16, in-session, verbatim):**
  - On the product theory: *"There's no professional judgment being encoded
    in this app, it's entirely a design aid for autonomous production of
    candidates that will be fed to a canonical solver engine for validation.
    That external engine is all the validation this software needs to be
    useful. It's also how you can get feedback on how the app is working, but
    that comes at another cost for me to install a licensed version of
    CAEPIPE for validation purposes."*
  - On relocation, after the agent recommended promoting the adopted PRD to
    `docs/PRD.md` inside this scope change: *"I agree with your
    recommendation. This scope change will need to address the impacted
    deliverables too, revising their scopes of work and remaining work items.
    Proceed accordingly to the decision packet for my ruling."*
- **Evidence the pointer indirection is failing:** the standing loop plan
  (`loop/WORKPLAN_2026-07-10_piping_loop.md:27,141`, adopted six days after
  the v0.2 adoption) names `docs/PRD.md` §10/§22 as the product yardstick;
  D-46 (2026-07-15) framed its question in v0.1 "§22.6" tokens; ~118 files
  reference `docs/PRD.md` versus ~17 referencing the v0.2 path.
- **v0.2 already carries most of the product theory:** §5.1 (full analytical
  engine, non-authoritative result), §8.9 (External Prover Tool), §16
  (handoff and external prover workflow), §22.5 (prover correlation as
  future/optional validation evidence). PKG-14/15/17 already hold the
  machinery deliverables (DEL-17-04/05 CAEPIPE export/harness, DEL-14-05
  tolerance/mapping contracts, DEL-15-04 prover boundary metadata) under
  SCA-004; `DEC-016` defers only comprehensive commercial-tool ingestion.
- **Scope-change numbering:** SCA-006 was consumed by D-43 (decomposition
  revision v0.9, scope-neutral currency amendment). This packet's execution
  lane is therefore **SCA-007**. Observed discrepancy to correct in passing:
  `execution/_ScopeChange/_LATEST.md` still points at SCA-005 and SCA-006
  has no snapshot folder (it executed as a decomposition revision under
  D-43); SCA-007 must leave `_LATEST.md` correct and record this note.

## 3. Options

### O-A - Amend, relocate, and propagate (recommended)

Execute all four parts as one SCA-007 transaction:

**Part 1 — PRD text amendment (v0.2 → v0.3).** Four changes:

1. **§24 R6 exit criterion.** Replace "external reviewers can reproduce
   validation examples" with an actor-neutral criterion: *"validation
   examples reproduce from a clean checkout by following the documented
   validation-manual procedure, with recorded environment, tool versions,
   commands, exit codes, and output hashes; the reproduction is
   actor-neutral (maintainer- or agent-executable)."*
2. **§22.5 re-weighting.** Retitle to "Prover Correlation as the Principal
   Validation Posture." State that the product's principal validation
   evidence class is correlation against an external professional solver
   (external prover; e.g., CAEPIPE): the internal engine generates and
   screens candidate designs; the external prover is the canonical
   validation oracle. Activation is owner-gated on lawful access to the
   external tool; until activated, the internal benchmark and regression
   suites remain the development verification basis. The existing
   evidence-field list and the sentence "These comparisons are validation
   evidence, not automatic professional acceptance" are retained unchanged.
3. **§22.1 verification-philosophy addition.** One paragraph: the internal
   verification suite exists to develop and maintain the engine — it proves
   the equations are implemented correctly and guards regressions;
   product-level validation credibility comes from external-prover
   correlation (§22.5) and the transparency of the procedures themselves.
4. **Third-party reproduction re-homed.** One sentence (in §22.1 or beside
   the §24 R6 criteria): independent third-party reproduction of the
   validation manual is a publication-era credibility objective tied to the
   public-repository publication gates; it is not a milestone exit
   criterion. (The DEC codification row carries the D-05b-family linkage so
   the PRD text stays free of register tokens.)

**Part 2 — Relocation.** The amended v0.3 text becomes `docs/PRD.md`. The
v0.1 text moves verbatim to `docs/_history/PRD_v0.1.md`, with only its
Forward Authority Note re-pointed (a forward-pointer currency correction,
not a history rewrite). A short redirect stub remains at
`docs/_ScopeChange/OpenPipeStress_PRD_v0.2.md` for the ~17 existing
references. The new `docs/PRD.md` header carries a citation-resolution note:
references to `docs/PRD.md` dated before this amendment resolve against
`docs/_history/PRD_v0.1.md` and the D-21 Annex A crosswalk; references to
the v0.2 path resolve against this file (v0.2 text preserved in git
history). This is the same read-old-tokens-through-a-crosswalk convention
already used for FR and milestone numbers.

**Part 3 — Impacted deliverables and live pointer surfaces.** Per §5 below.

**Part 4 — Stage-record re-expression.** The `_COORDINATION.md` current
target stage record is re-expressed in amended-PRD tokens under the same
ruling (the record currently quotes v0.1 §22.6 and §16.2/§16.5). The stage
itself does not move: Phase E continues under the `DEC-054` gate read
forward through `DEC-056`; the exit-evidence criteria become the amended
actor-neutral reproduction criterion plus the unchanged protected-standards
criterion (v0.2 §20.1 / D-20 lineage).

Pros: criterion text becomes honest instead of stretched; the yardstick file
is the file named `PRD.md`; the recurring gate class D-46 represented is
retired; DEL-09-04's parked residual becomes agent-executable.  
Cons: one more crosswalk for historical `docs/PRD.md` line-cites; touches
many pointer surfaces in one transaction.

### O-B - Amend text only, no relocation

Execute Part 1 (as v0.2.1 in place), Part 3, and Part 4; fix the loop-plan
yardstick lines to point at the v0.2 path; leave both files where they are.

Pros: smaller diff; no citation-resolution note needed.  
Cons: the living authority stays housed under `docs/_ScopeChange/` and every
future surface must know the indirection that has already misfired twice in
eleven days.

### O-C - Defer

Keep the criterion, locations, and deliverable text as they stand; the D-46
O-C posture continues; DEL-09-04's reproduction residual stays parked.

Pros: no change risk.  
Cons: the yardstick keeps encoding a validation theory the owner has stated
is not the product's, and the misleading pointer surfaces keep steering
agents to v0.1 tokens.

## 4. Non-Binding Recommendation

Select **O-A**. The owner has already stated the product theory and agreed
with the relocation recommendation in session; O-A is that direction
expressed as one governed transaction. O-B is the fallback if the citation
crosswalk cost is judged too high today.

## 5. Impacted Deliverables And Surfaces (O-A/O-B propagation set)

| # | Surface | Current state | Proposed amendment |
|---|---|---|---|
| 1 | DEL-09-04 `_STATUS.md` `## Remaining` | "Demonstrate external reproduction on a clean environment via the E1 runner (stage-gated: R5 exit evidence)" | Reword to the actor-neutral criterion: demonstrate clean-checkout reproduction of the validation examples via the documented E1 runner procedure, recording environment/tool versions/commands/exit codes/output hashes into the ruled evidence-bundle home; stage-gate token re-keyed to the amended criterion. Becomes agent-executable ordinary loop work. |
| 2 | DEL-09-04 `_STATUS.md` E2 residual "evidence-bundle storage policy open" | Open since TP-E2-VALMANUAL-001 | Resolved by this ruling naming the home: `validation/evidence/reproduction/<run-id>/` (immutable derivative bundles citing procedure + source commit). |
| 3 | DEL-09-04 `ScopeOfWork.md` | May cite the v0.1/v0.2 criterion wording | Citations re-keyed at the SCA-007 impact assessment; no scope expansion. |
| 4 | DEL-10-04 `_STATUS.md` D-06b signing item | Cites "the explicit PRD §22.6 deviation" (v0.1 token) | Citation re-keyed to the historical v0.1 §22.6 read through D-21 Annex A / the `DEC-056` R6-entry-residual rider; item remains `(gated: D-06b)` unchanged. |
| 5 | DEL-10-05 `_STATUS.md` | "E2 per-case reproduction currently runs through suite tests" (method description) | Verify-no-change at impact assessment; no scope change expected. |
| 6 | DEL-14-05, DEL-15-04, DEL-17-04, DEL-17-05 (prover machinery) | Scopes already admit CAEPIPE export/harness/metadata/tolerance under SCA-004 | No scope expansion. Where scope text frames prover correlation as merely optional/future, add a currency note citing the ruling: prover correlation is the principal validation posture, activation owner-gated. Enumerated exactly at impact assessment. |
| 7 | `loop/WORKPLAN_2026-07-10_piping_loop.md:27,141` | Yardstick named as `docs/PRD.md` §10/§22 (v0.1 tokens) | Re-point to the amended PRD (`docs/PRD.md` after O-A; the v0.2 path after O-B) with correct section tokens (§11 functional requirements, §24 release milestones). |
| 8 | `docs/PLAN.md` rows 32/42/88 | Point at the v0.2 path and speak the read-forward convention | Pointer updates to the relocated file; Phase E row cites the amended criterion. |
| 9 | `execution/_Coordination/_COORDINATION.md` stage record | Quotes v0.1 §22.6 and §16.2/§16.5 | Part 4 re-expression (owner-approved within this ruling; no stage advancement). |
| 10 | D-46 packet | RULED O-C (2026-07-16, PR #253 branch) | Append a supersession note: the question is superseded by this ruling; the O-C deferral disposition is preserved as immutable history. |
| 11 | `execution/_ScopeChange/_LATEST.md` | Still points at SCA-005 | Move to SCA-007 with its snapshot folder; record the SCA-006 folderless-execution note (§2). |
| 12 | `execution/_Decomposition/SOFTWARE_DECOMP.md` | Revision v0.9 | Revision v0.10 records SCA-007; `DEC-XXX` codification row (next free at ruling time). |

Not in scope: the ~118 historical references to `docs/PRD.md` in ruled
records, run records, and receipts are **not** edited — they resolve through
the citation-resolution note (O-A Part 2), exactly as ruled history already
resolves v0.1 FR/milestone tokens through Annex A.

## 6. On-Ruling Mechanism

- Append the ruling to this packet; flip the D-47 register row to RULED.
- Execute the selected option as **SCA-007** under the governed SCOPE_CHANGE
  workflow: snapshot folder `execution/_ScopeChange/SCA-007_<date>/` with
  impact assessment, amendment preview, propagation plan, owner acceptance
  of the amendment actions, then gate execution inside the accepted write
  lane — the same shape as SCA-005's D-29 lane.
- Codify the ruling as the next free `DEC-XXX` row in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (carrying the
  D-05b-family linkage for the re-homed third-party-reproduction objective
  and the evidence-bundle home).
- Deliverable-local edits (§5 rows 1–6) land as SCA-007 amendment actions
  with `_STATUS.md` history lines citing the DEC row.
- Branch-first + PR; the owner merges. Sequencing note: this packet's
  register row appends after D-46, whose row lands with PR #253 — merge
  PR #253 first; a trivial adjacent-append rebase may be needed here.
- A future owner-gated register row (not this one) activates external-prover
  correlation when the owner elects to procure CAEPIPE access.
- Append one versioned minimal receipt per the loop ledger rules and rerun
  the receipt validator.

## 7. Current Gateway

`AWAITING_RULING`. No PRD text change, relocation, deliverable-scope edit,
stage-record re-expression, prover activation, or reproduction acceptance is
authorized by this proposal.
