# Coordination Record

**Epistemic status (rewritten 2026-07-10 at owner adoption, K-AUTH-1).** This
file is now a ruled-record surface and pointer, not the operative protocol.
The development-loop instructions live in the newest `WORKPLAN_*.md` under
`loop/`. This file carries only the owner-ruled records that decision packets
and register rows name it for. It must never accumulate status, queues, or
counts; the pre-2026-07-10 text is preserved in git history.

Path anchor: resolve `REPO_ROOT` with `git rev-parse --show-toplevel`; paths
below are relative to `{REPO_ROOT}/projects/chirality-piping`.

## Current Target Stage (ruled record)

Ruling lineage: `DEC-048` set the R4 target stage; `D-26`/`DEC-053` held R4
pending the named sparse default-promotion repair; `D-27`/`DEC-054` accepted
the conditional R4 gate and advanced the target stage to R5. Register rows
`D-14`, `D-26`, and `D-27` name this file as the recording surface for the
Working Desktop Application Standard current target stage.

**Current target stage - PRD R5 exit criterion (advanced 2026-06-23 by
`DEC-054`, after the conditional R4 target-stage gate in `D-27`).** Phase D/R4
evidence was reviewed through
`plans/VERIFICATION_2026-06-23_r4_exit_chain_refresh.md`
(`TP-R4-D9-EXITCHAINREFRESH-001`) and clean-head sweep
`validation/evidence/sweeps/SWEEP_20260623T051552Z_16cca07f3b64.json`.
The human project authority accepted the refreshed packet as a conditional R4
gate and authorized R4-to-R5 target-stage advancement. The ordinary in-stage
program is now Phase E: engineering beta and release machinery. The exit
criteria, expressed in amended-PRD tokens per `DEC-080`, are: validation
examples reproduce from a clean checkout by following the documented
validation-manual procedure, with recorded environment, tool versions,
commands, exit codes, and output hashes, actor-neutral (maintainer- or
agent-executable) — amended PRD (v0.3) §24 R6; and the public repository
contains no known protected standards data (v0.3 §20.1 / D-20 lineage). The
historical v0.1 token "PRD §22.6: external engineers can reproduce validation
examples" resolves through the D-21 Annex A crosswalk. The complete
benchmark/manual evidence system — historically cited as PRD §16.2 / §16.5,
now the v0.3 §22.2 required solver benchmarks plus the §24 R6
validation-manual deliverable — remains explicit residual work to gather
later, including once the agent harness is active and can participate.
Boundary prohibitions continue unchanged: no protected standards content,
private-data defaults, unapproved release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance claim.
This record was re-expressed 2026-07-16 in amended-PRD tokens under
`DEC-080`/SCA-007; the target stage itself is unchanged.

Agents propose stage advancement with evidence; only a human-approved
coordination update advances the target stage recorded here. This standard is
not a release gate, legal clearance, professional reliance record, or final
`ISSUED` lifecycle decision.

## State Tracking Rules (ruled records)

`DEC-040` cycle-driven resolution augmentation, unchanged: SCCs are the
primary diagnostic of undecided ordering: any edge that participates in a
cycle is non-gating until the SCC is resolved by a recorded move (decompose /
invert / merge / cut; cut/merge are human-gated). The active edge set is kept
acyclic by construction; unresolved cycles live only in candidate worklists
pending resolution. Later approved DAG versions are event-driven by a
decomposition revision / SCA or another explicit governance rectification,
not periodic. Canonical doctrine: the shared repo-root
`docs/CYCLE_DRIVEN_RESOLUTION.md`. This project's adoption and rollout record:
`plans/PLAN_2026-06-13_cycle_driven_resolution_doctrine.md`.

Guidance-surface correction rule, unchanged (applies to the decision register
and the standing workplans under `loop/`): they are non-governing guidance
surfaces. They do not replace decomposition truth, DAG authority, deliverable
lifecycle state, or human gates. When they disagree with authoritative
surfaces, surface the discrepancy and correct the plan or register, not the
authority.

## Application Integration And Issuance Loop

The steps of this loop are absorbed into the newest `WORKPLAN_*.md` under
`loop/` as of 2026-07-10 (owner-adopted, K-AUTH-1). Historical references to
this heading — e.g. decision packets "prepared per the Application Integration
And Issuance Loop decision-escalation step" — resolve to the corresponding
protocol steps of that workplan. The pre-2026-07-10 loop text is in git
history at this path.

## Ruled workflow records

- UI evidence posture (human-approved H4 amendment, 2026-06-11; draft basis
  `plans/DRAFT_2026-06-11_H4_coordination_evidence_posture.md`) remains
  binding on the workplan's validation step: user-visible desktop behavior
  changes default to Playwright e2e spec extensions; manual `SMOKE.md`-only
  evidence is the recorded exception; new React components land with unit
  tests or a recorded evidence gap; homogeneous UI slices may use one
  template-level test plus per-instance smoke assertions.
- Pre-push local evidence sweep (`DEC-025`) and issuance waves W1–W7
  (`D-11`/`DEC-062`) are codified in
  `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, not here.

## Pointers

- Session entry: `init/init-prompt.md` → `loop/LOOP_INIT.md` → the newest
  `loop/WORKPLAN_*.md` → `loop/LOOP_RECEIPTS.md`.
- Work surface: deliverable folders (`execution/PKG-*/1_Working/DEL-*/`) and
  `tools/coordination/list_deliverable_status.py` (read-only discovery).
- Decision register: `execution/_Coordination/_DECISIONS/_REGISTER.md`.
- Codified rulings: `execution/_Decomposition/SOFTWARE_DECOMP.md` §12.
- Approved DAG pointer: `execution/_DAG/_LATEST.md`.
- `execution/_Coordination/NEXT_INSTANCE_PROMPT.md`: dated historical map
  (pre-loop session entry; never authority).
