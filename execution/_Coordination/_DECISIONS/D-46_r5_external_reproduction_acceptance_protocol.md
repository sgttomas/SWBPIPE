# D-46 - R5 External-Reproduction Acceptance Protocol

**Status:** PROPOSAL  
**Date prepared:** 2026-07-15  
**Decision ID:** D-46  
**Prepared by:** agent, from the DEL-09-04 R5-exit residual  
**Owner act required:** yes

## 1. Decision Statement And Scope

What evidence is sufficient to satisfy the PRD §22.6 exit criterion
"External engineers can reproduce validation examples," and how must that
evidence be stored and accepted?

This packet governs the acceptance protocol only. It does not change solver or
runner behavior, select final public-benchmark tolerances, activate hosted CI,
publish a repository or release, or make a professional-reliance or
code-compliance claim.

## 2. Accepted Basis And Current Evidence

- PRD §22.6 names external reproduction as an R5 exit criterion.
- DEL-09-04 is DAG-unblocked under DAG-007 and records clean-environment
  external reproduction as open R5-exit evidence.
- `DEC-065` supplies the stable local `openpipestress-runner` command/process
  policy. `docs/validation_manual/headless_runner_reproduction.md` and
  TP-VALIDMANUAL-RUNNERREPRO-001 provide a maintainer-authored draft procedure
  over invented public fixtures.
- TP-E2-VALMANUAL-001 assembled 63 deterministic case pages and re-ran the
  benchmark suites and runner procedure, but expressly made no clean-
  environment or external-engineer claim.
- The current records do not define "external engineer," "clean
  environment," the evidence-bundle home, or the acceptance act that changes
  a reproduction record from project-owned verification into R5 validation
  evidence. An agent cannot silently select those meanings.
- DEL-10-05's broader benchmark/regression runner binding is not selectable in
  this loop iteration because active DAG-007 execution-upstream rows remain
  `TBD`. The existing suite-test reproduction path therefore bounds every
  option below.

## 3. Options

### O-A - Independent reproducer plus owner acceptance (recommended)

Use a two-stage protocol:

1. A maintainer may first run a clean-checkout rehearsal. It is recorded only
   as `MAINTAINER_REHEARSAL`, never as external reproduction.
2. A person who did not author the selected evidence tranche runs the frozen
   procedure from a fresh clone/check-out on a separately identified host or
   user environment with no repository build outputs or caches carried in.
3. The reproducer records commit SHA, operating system, tool versions,
   prerequisite-install steps, exact commands, exit codes, observed
   diagnostic codes, generated-output hashes, deviations, identity/role, and
   a statement that no private project data or protected standards content was
   introduced.
4. The immutable derivative bundle is stored under
   `validation/evidence/external_reproduction/<run-id>/`, cites the accepted
   DEL-09-04 procedure and source commit, and remains
   `PENDING_OWNER_ACCEPTANCE` until the owner reviews it.
5. Only the owner's recorded acceptance may count the run toward PRD §22.6 or
   promote named case pages from `DRAFT_EVIDENCE`. Promotion remains page-
   specific; it does not settle final tolerances, release readiness, or
   professional reliance.

Pros: preserves the ordinary meaning of "external," separates rehearsal from
validation, and creates a checkable acceptance chain.  
Cons: requires coordination with another human and cannot be completed by an
agent-only loop.

### O-B - Clean-checkout maintainer reproduction is sufficient

Allow a maintainer/agent-run fresh clone with no carried build outputs to
satisfy the criterion after owner review. Store the same fields and bundle
shape as O-A, but classify the actor as `PROJECT_MAINTAINER` rather than
independent.

Pros: executable immediately with existing local infrastructure.  
Cons: weakens the text's externality signal and risks treating project-owned
verification as independent workflow validation.

### O-C - Defer the acceptance protocol

Keep current draft procedures and case pages unchanged. No clean-checkout run
counts toward PRD §22.6 and no page is promoted.

Pros: avoids premature acceptance semantics.  
Cons: leaves a direct R5 exit criterion open.

## 4. Non-Binding Recommendation

Select **O-A**. A local clean-checkout rehearsal is useful evidence but cannot
by itself establish that an external engineer reproduced the examples. O-A
preserves that distinction while giving the eventual reviewer a bounded,
immutable evidence contract.

## 5. On-Ruling Mechanism

- Append the ruling to this packet and update D-46 in `_REGISTER.md`.
- Codify the accepted protocol in `execution/_Decomposition/SOFTWARE_DECOMP.md`
  §12 only if the owner directs codification.
- If O-A or O-B is selected, prepare/adopt a bounded DEL-09-04 execution brief
  for the rehearsal/reproduction procedure and evidence bundle. No run may be
  described as accepted until the required owner act is recorded.
- If O-C is selected, leave the DEL-09-04 residual open and record no
  substitute completion claim.

## 6. Current Gateway

`AWAITING_RULING`. No external-reproduction acceptance, case-page promotion,
or R5-exit closure is authorized by this proposal.
