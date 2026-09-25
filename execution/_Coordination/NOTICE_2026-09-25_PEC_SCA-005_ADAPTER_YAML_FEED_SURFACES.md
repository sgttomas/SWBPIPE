# Coordination Notice — PEC SCA-005: `_harness/adapter.yaml` feed surfaces

**Status:** NON-BINDING NOTICE
**Receiving loops:** Chirality App (`projects/chirality-app-dev`) and Chirality Piping (`projects/chirality-piping`)
**Sending loop:** PEC (`projects/pec`), HELP_HUMAN run `HELP-HUMAN-PEC-20260923-SCA005`

PEC scope change SCA-005 (owner checkpoint-2 acceptance 2026-09-25,
register row `D-PEC-92`) moves PEC's feed model to per-loop feed profiles
declared in PEC's own loop registry (`loops.json`; selected option O-B2).
Under that model PEC reads your loop's `execution/_Coordination/WorkGraphs/`
`WORK_GRAPH.md` files, central `RECEIPT.md` files, deliverable `MEMORY.md`
run indexes and Task Management surfaces from paths its registry declares.
Your `_harness/adapter.yaml` declares none of those surfaces; PEC's
deliverable DEL-02-07 is re-purposed to read `adapter.yaml` only as a
parity-peer input.

Nothing is needed from your loop: O-B2 does not depend on `adapter.yaml`
declaring these surfaces, and PEC will not write your files. If your loop
later chooses to declare them there, PEC's parity check would compare the
two declarations.

This notice grants no authority in the receiving loop, creates no
requirement there, and asks for no write. The receiving loop may adopt,
amend, defer or decline any implication under its own instruments.
