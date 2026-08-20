# Piping SCA-009 Decision Log

**State: `GATE 1 CONFIRMED — GATE 2 PENDING`**

## Pending register (Gate-1 candidate package, historical)

The table below is the original Gate-1 candidate pending register, preserved
unchanged as history. Every item in it has since been ruled; the ruling
table that follows is the current record.

| Seq | Item | Blocked by | Owner | Decision | State |
|---|---|---|---|---|---|
| D1 | Gate-1 ruling: confirm, correct, or decline the parsed change request (Brief.md Section 7) as the owner's intent | — | Ryan Tufts | — | `PENDING` |
| D2 | Resolution shape: Option A (ADD DEL-07-09), Option B (MODIFY DEL-16-01 + DEL-07-01/02), or an owner-directed variant | D1 | Ryan Tufts | — | `PENDING` |
| D3 | Ratify, amend, or reject the candidate reference operation vocabulary (Brief.md Section 4) as the normative coverage list | D1 | Ryan Tufts | — | `PENDING` |
| D4 | Palette-surface ownership shape: single owner vs split across existing GUI deliverables | D1, D2 | Ryan Tufts | — | `PENDING` |
| D5 | Disposition of adjacent ownership residuals: DEL-07-03-R-005/R-006 (load-case, support/restraint editors) and the DEL-16-04 route/support candidate-generator ownership — fold into this SCA or leave to separate acts | D1, D2 | Ryan Tufts | — | `PENDING` |
| D6 | If Option A: new scope row (SOW-077) vs explicit SOW-020/SOW-021 coverage remap | D1, D2 | Ryan Tufts | — | `PENDING` |

## Ruling table

Verbatim ruling text is transcribed in `ACCEPTANCE_RECORD.md` (in-session
chat rulings, 2026-08-20). Dispositions are human acts.

| Date | Gate / act | Actor | Decision | State |
|---|---|---|---|---|
| 2026-08-20 | Gate 1 (D1) | Ryan Tufts | Confirmed the parsed change request in Gate-1 package SHA-256 `2458c1dce9b175330c8b28a4a0e4647988213539ee4edb8d27f7ae74e0e9adc0` (merged as PR #592, merge `01a8dd4c0aabd4fe1f71bba7201a4345f9e6cfdc`; original durable basis `7584de0a8d53d69a135c22fe39a78cb4a30b6cb2`) as the owner's intent — "D1 is confirmed." Opens Gate 2 only. | `CONFIRMED` |
| 2026-08-20 | D2 — resolution shape | Ryan Tufts | Accepted as recommended: **Option A** — ADD `DEL-07-09` "Interactive operation vocabulary and tool palette contract" as a new PKG-07 deliverable. | `RULED — OPTION A` |
| 2026-08-20 | D3 — vocabulary ratification | Ryan Tufts | Ruled after the tiered-coverage discussion: two-class vocabulary ratified — **NORMATIVE-NOW** (Tier 1 + Tier 2 + accepted Tier-3 items: element insert/split; copy/rotate/mirror; nozzle/equipment boundary conditions; automatic self-weight case generation; spring-hanger selection from user-imported hanger libraries per DEC-049; project-wide unit switching as a display-system toggle) and **ROADMAP** (deferred: node renumbering; snubbers; cold spring / cut-short — owner: rarely used, add later). Sequencing: wire existing backend capability first; net-new capability separately and afterwards. Vocabulary binds to the implemented operation taxonomy, not the schema `OperationKind` enum. Candidate normative text: `Vocabulary_Annex.md`. | `RULED` |
| 2026-08-20 | D4 — palette ownership | Ryan Tufts | Accepted as recommended: single palette-surface owner — `DEL-07-09`. | `RULED` |
| 2026-08-20 | D5 — adjacent residuals | Ryan Tufts | Accepted as recommended: fold the `DEL-07-03-R-005` (load-case editor) and `DEL-07-03-R-006` (support/restraint editor) landing into `DEL-07-09`; the `DEL-16-04` route/support candidate-generator ownership stays out, reserved to a separate decomposition act. | `RULED` |
| 2026-08-20 | D6 — scope-row shape | Ryan Tufts | Accepted as recommended: new scope row `SOW-077`; no SOW-020/SOW-021 remap. | `RULED` |

Notes:

- These rulings open Gate 2 only. No amendment, propagation, implementation,
  dependency, estimate, schedule, pointer, lifecycle, release, or Git action
  is approved by them.
- Gate-2 and later rulings, when they occur, are appended to this table and
  transcribed verbatim in `ACCEPTANCE_RECORD.md`.
