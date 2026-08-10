# Run basis — TM-PIP-038 through TM-PIP-040 treatment — 2026-08-09

## Status and identity

- Status: `ACTIVE — BOUNDED TREATMENT`.
- Run ID: `TM_PIP_038_040_TREATMENT_2026-08-09`.
- Parent: `HELP_HUMAN` Agent 0, current 2026-08-09 session.
- Manager: managed `RECONCILIATION` Agent 1 child
  `/root/reconciliation_tm038_040_treatment`.
- Selection authority: owner direction, 2026-08-09, activating exactly
  `TM-PIP-038`, `TM-PIP-039`, and `TM-PIP-040` after the activation record
  landed on the shared baseline.
- Posture: `MIXED`; this is a narrower-than-corpus treatment and does not
  activate or reopen corpus-wide concordance.

## Source-state binding and activation proof

- Repository root was resolved from the active checkout with
  `git rev-parse --show-toplevel`.
- Working root: `projects/chirality-piping` under that resolved repository.
- Frozen treatment source state: `origin/main` and clean `HEAD` both at
  `da40d7dc4192c9aa2f49e9438729179aae281b61`.
- Activation commit
  `3f00a351695ec3943be6d60a89643795a28f9220` is an ancestor of the frozen
  source state.
- Activation record:
  `ACTIVATION_ROUTING_RECORD.md`; Git blob
  `e8ee259b46f0ca4fa5a235c9f5ea9a5991c279e8`; SHA-256
  `e8ef649f54145e8c82b1d45bcce31bea2ec9f15d30f45bda7a464cd752f1309e`.
- The record's pre-landing status text is historical. Its own "Activated scope
  upon shared-baseline landing" condition is satisfied by merge commit
  `da40d7dc4192c9aa2f49e9438729179aae281b61` (PR #531).
- Routed handoff: Git blob
  `5e05568fe2276f753858bffb993d98109a12d9a4`; SHA-256
  `7bca6073f2ba9aa1c4350ee694e979fb1b04fee561ab09329ba09a1ae3ebdd30`.
- Task Management register: `execution/_Coordination/_TaskManagement/REGISTER.csv`;
  Git blob `8574d9df2ff4fdf2ca85cd51dd1b74ddd99fefdd`; SHA-256
  `60a8e4956c4f94cc7b64a886fb5c8060f026b010c0bc012d8296fd2044b2a30c`;
  rows `TM-PIP-038` through `TM-PIP-040` are `OPEN`. This run has no register
  write or disposition authority.

## Frozen authorities and current pointers

| Surface | Frozen binding |
|---|---|
| Root runtime doctrine | `AGENTS.md`; blob `826bccb9cf33a2079921fc9ef36fc7d63bd72e85`; SHA-256 `268becd0bac9da8421b30089e4e4167a5e5f79bf3892d0f72ad41a63180a3aeb` |
| Project rider | `projects/chirality-piping/AGENTS.md`; blob `79a70b4ac8e4a53ba2842290fda62aab982887bb`; SHA-256 `9e5dec7df1a56f05ff947ad8fea982613e7350e5b4833b4be4f69875a922fb2b` |
| Manager contract | `agents/AGENT_RECONCILIATION.md`; blob `a358ab08c5fb22e77c9c9a8353d95a32505e6eb7`; SHA-256 `46bca06f907c4da765b1b1177ecd51c6858fdf45bf7620341175c3b847a3e4f7` |
| Ratified shared method | `docs/DELIVERABLE_CONCORDANCE_METHOD.md`; blob `137209cb37e8d8204a7f2bd78114b4b5753c6c2e`; SHA-256 `abf3e78fce606c4557d61cdbfbdb7292a3d858838f6526da6b433d1bcd0ef627` |
| Project adoption/provenance plan | `plans/PLAN_2026-07-10_deliverable_implementation_reconciliation.md`; blob `634ca34c18433c3415cac9572a9f75e8a06e6ca4`; SHA-256 `5abc0c80d5f3964ba2ff6ceed787a2d7d0b5568980f6263f781a2f64336f9422` |
| Decision register | `execution/_Coordination/_DECISIONS/_REGISTER.md`; blob `fc9dd5bd3b468d982e4d3ed1ba4b8cbbd3b44b7e`; SHA-256 `727071b652ec4d7506cd515687b1b45884de687354342b7b0d66245b4020badf` |
| Current decomposition pointer | `execution/_Decomposition/_LATEST.md` → `SOFTWARE_DECOMP.md` revision `0.11`; pointer blob `45de7c70c7c7441d1fcf73f670da8433a1c8e57b`; decomposition blob `a64b02b77248c26d3d17987624131a35a5acbb71` |
| Current DAG pointer | `execution/_DAG/_LATEST.md` → approved `DAG-009`; pointer blob `5441c7127aceecdefe242bef25e5ca9cd5a330b4`; approval blob `0fe2817a3937ba690af466ea0509cb77368940e3f` |
| Lifecycle authority | `docs/TYPES.md` section 9, blob `d278ba00506ce15910c4e10e20238e6777949474`; authority map `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md`, blob `a903e339420b2cd2661a00afbf7c956babc3e5a3` |

The current decomposition, DAG, decisions, and lifecycle surfaces are inputs
and fences only. This treatment changes none of them and makes no lifecycle,
release, professional-reliance, or issuance claim.

## Treatment scope and preservation posture

1. `TM-PIP-038`: read-only discovery first. The target summaries are members
   of the accepted closed discovery snapshot
   `DELIVERABLE_CONCORDANCE_2026-07-11_1305`. The manager contract SPEC 13 and
   R6 require that accepted discovery snapshot to remain immutable; historical
   generic reconciliation evidence is also explicitly immutable. Therefore no
   historical summary edit is presumed. Any lawful outcome must preserve exact
   source bytes and bind a new derivative correction/supersession record unless
   an authority proves otherwise.
2. `TM-PIP-039`: create a new supersession record in this run root only. The
   historical `RUN_BASIS.md` and its owner quote/act remain byte-for-byte
   unchanged.
3. `TM-PIP-040`: bounded committed-provenance investigation and an owner-ready
   decision packet only. No `RESTORED`, `LOST`, or `UNDETERMINED` act is
   authorized; absence of the old worktree is not outcome evidence.

## Write and execution fences

- Primary persistent writes:
  `execution/_Reconciliation/DeliverableConcordance/TM_PIP_038_040_TREATMENT_2026-08-09/**`.
- Conditional historical-summary repair authority is held pending affirmative
  preservation proof and applies only to `PKG-06.md`, `PKG-07.md`, and
  `PKG-08.md`. The frozen preservation posture above currently fails that
  prerequisite, so those files remain read-only.
- All other repository paths are read-only.
- No Task Management register, routed handoff, historical run basis, decision,
  decomposition, DAG, lifecycle, deliverable, source, test, receipt, Git
  stage/commit/push/PR/merge, or evidence-outcome disposal write is authorized.

## Concurrent and overlapping work

- This branch and worktree were clean at dispatch and were based exactly on
  current `origin/main`.
- Other listed worktrees exist on separate branches. None is an authorized
  input or write target for this run. This run uses a dedicated worktree and
  the canonical run root as its only persistent write surface.
- Material movement of `origin/main`, any frozen input blob, any of the three
  historical summary blobs, the historical run-basis blob, or a declared
  recoverable TM-PIP-040 source makes the affected result `STALE_INPUT` and
  triggers a bounded rerun from a new frozen basis.

## Required terminal meaning

The manager returns treatment verdicts, exact evidence, validation, and a
handoff to the owner. Treatment verdicts are agent recommendations, not owner
rulings. Task Management retains all register disposition authority under a
separate owner closure ruling.
