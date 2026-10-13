# SCA-012 — Elastic tie rods on expansion joints

Status: **ACCEPTED** and executed. The owner answered on 2026-10-12, relayed by HELP_HUMAN to T4's WORKING_ITEMS: **"Lift for tie rods only"**. The question was whether PRD §7 non-goal 19 (expansion-joint hardware kinematics, recorded by D-37 / DEC-069) is lifted for T4-U5, whose plan scope includes "elastic tie rods as elements; tie force rows" (T4 plan 01 §3.1).

## The amendment

| Surface | Before | After |
|---|---|---|
| `docs/PRD.md` §7 item 19 | Model expansion-joint hardware kinematics (tie rod, hinge, gimbal) beyond the configuration metadata already specified for expansion joints. | Model expansion-joint hinge or gimbal hardware kinematics beyond the configuration metadata already specified for expansion joints. Elastic tie rods are in scope as explicit rod elements (§12.3; SCA-012, owner 2026-10-12: "Lift for tie rods only"). |
| `docs/PRD.md` §12.3, Expansion Joints | tie rod, hinge, gimbal, or limit rod metadata | elastic tie rods as explicit axial rod elements, with each rod's force published; hinge, gimbal, or limit rod metadata |
| `docs/PRD.md` header and amendment note | — | amended 2026-10-12 by SCA-012 |
| `execution/_Decomposition/SOFTWARE_DECOMP.md` DEC-069 | names "expansion-joint hardware kinematics beyond metadata" | the same row, amended in place to record the promotion |

`execution/_Coordination/_DECISIONS/D-37_RULING_2026-07-09.md` is historical and unchanged.

## What enters scope, and what does not

- **In scope:** elastic tie rods as explicit elements on objective-connector expansion joints: bilateral linear axial rods with authored attachments, stiffness and reference extension, assembled and recovered as members, with each rod's force published.
- **Not in this amendment:** hinge and gimbal kinematics (they stay named non-goals); ideal inextensible ties and the collective axial constraint, which need a multipoint constraint (plan 01 §4.3 item 5: a later unit only if users need it); tension-only, gapped or limit rods, stops, nuts and friction (limit rods stay metadata).
- **Unchanged:** packages, deliverables, stable IDs, the scope ledger and the dependency graph. The joint mechanics stay with their current owners (DEL-03-06 for the component model, DEL-04-07 for the solve integration). No lifecycle, release-readiness, professional, certification or code-compliance claim follows.

## Consumer

T4-U5b (batch 7) implements the elastic rods, re-derives JR's ideal-tie rows (J3 rows 2 and 4) as finite-rod cases and tests the tied J4 control on the rod force. Its starting design is [DEL-04-07 Design, joint-pressure-thrust.md, "What T4-U5b adds"](../../PKG-04/DEL-04-07/Design/joint-pressure-thrust.md). DEL-03-06's open hardware taxonomy (`TBD`, R-004) gains its first named rod kind through U5b's design, under this ruling.

## Method

The `scope-change` workflow's grouped checkpoints are not used. The change is one owner-decided scope statement, with no structural decomposition effect, executed by editing the governed text in place under the repository's current record rule (AGENTS.md: a record needs a concrete consumer; decisions edit the text they govern). `_LATEST.md` now selects this amendment.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
