# D-58 — Effective-State Closeout

Status: `EFFECTIVE_STATE_CLOSEOUT`
DecisionID: `D-58`
Date: 2026-07-27
Owning loop: Chirality Piping
Candidate basis: `main@2979e3ff85a3bc8bc106c4cd01c63c5a2c7b4bc1`
Accepted proposal SHA-256: `5047a23b791c4a0e747a8e5adc7674b8e602a43d359c95b385df7f45e5b28943`
Original ruling SHA-256: `b9c0c50e1fbd5d350a24f9d1a8d57eda22dfee1cb9a796e20d4ea3772c56b377`
PublicationSHA: `042004fedc46839f3637a59164d5f5e5f28b4fb8`
ApplicationCommit: `042004fedc46839f3637a59164d5f5e5f28b4fb8`
EffectiveSHA: `2979e3ff85a3bc8bc106c4cd01c63c5a2c7b4bc1`
EffectivePR: `#378`
RecordConvention: additive closeout; accepted proposal and original ruling remain byte-identical

## Purpose

Record the completed Git integration of the D-58 G5-A current-mechanism
disposition. This file creates no new authority. It supersedes only the stale
current-state statement in the Piping decision-register row that says durable
effect remains pending Git closeout.

## Deterministic identity

Git establishes:

1. `042004fedc46839f3637a59164d5f5e5f28b4fb8` is the first publication and
   scoped application commit for the accepted proposal, original ruling, and
   one new D-58 register row;
2. `2979e3ff85a3bc8bc106c4cd01c63c5a2c7b4bc1` is the two-parent merge of PR
   #378 and contains that publication/application commit;
3. the three applied Piping paths are byte-identical between the application
   commit and the merge;
4. the accepted proposal remains SHA-256
   `5047a23b791c4a0e747a8e5adc7674b8e602a43d359c95b385df7f45e5b28943`;
   and
5. the original ruling remains SHA-256
   `b9c0c50e1fbd5d350a24f9d1a8d57eda22dfee1cb9a796e20d4ea3772c56b377`.

Publication and application share one commit in this tranche, but their roles
are recorded separately. The EffectiveSHA is the owner-authorized merge into
`main`.

## Effect

D-58 is durably integrated and effective at PR #378 merge
`2979e3ff85a3bc8bc106c4cd01c63c5a2c7b4bc1`.

Current reliance on the App-era `D-APP-48` / `D-30`
synchronized-consumption mechanism remains retired. D-30/D-31 and
DEC-041/DEC-063 remain historical acts; Piping remains outside the
Root-runtime and App-harness client sets; Piping PRD R7 and its product
outcomes remain unchanged; and the automation-condition mechanism remains
unresolved.

Piping `SCOPE_CHANGE` Gate 1 is eligible to be presented under the original
ruling. This closeout does not open or approve that gate.

## Scope limits

This closeout does not:

- amend the accepted proposal or original D-58 ruling;
- rewrite D-30, D-31, DEC-041, or DEC-063;
- amend the Piping PRD or decomposition;
- open or approve SCA-008 or add DEC-091;
- adopt a successor mechanism or make Piping a Root-runtime or App-harness
  client;
- change source, runtime, product, package, deliverable, lifecycle, release,
  or professional-reliance state; or
- pre-approve any later owner gate.

Git identity and passing checks are evidence of integration, not a new human
decision.
