# R1 Notes — Deliverable Inventory (DELIVERABLE_INVENTORY.csv)

**Run:** DELIVERABLE_CONCORDANCE_2026-07-11_1305 (activation D-41/DEC-073;
owner-adopted scale-out). **Phase:** R1 read-only project inventory per the
pinned plan §8 R1. **Source state:** ALL deliverable reads from the frozen
worktree at `551f84ef6be656f1603ce0acfa5e3935aa9683c7`
(`.claude-worktrees/piping-frozen-551f84ef6`, project root
`projects/chirality-piping`). Side-effect-free reads only
(`PYTHONDONTWRITEBYTECODE=1`; no writes anywhere in the frozen tree);
`git status --porcelain` verified clean at completion, HEAD verified at the
pinned SHA. Binding conventions: `R1_CONVENTIONS.md` (R0b conventions 1–8 +
addenda 1–13, owner-adopted).

## Method

1. **Enumeration.** Globbed `execution/PKG-*/1_Working/DEL-*/` directories in
   the frozen tree: **101 deliverable folders** across 18 packages
   (PKG-00..PKG-17). Per-package row counts (8, 4, 5, 8, 6, 5, 5, 8, 6, 5, 5,
   5, 5, 4, 5, 4, 4, 9) reproduce the Part D wave partitions exactly:
   W1=25, W2=11, W3=19, W4=20, W5=26.
2. **Lifecycle.** `**Current State:**` parsed from each `_STATUS.md`.
3. **`## Remaining`.** Section presence, ordered `- ` items (single-line at
   this SHA; a continuation-line joiner was implemented but never fired).
   Per item: verbatim 120-char truncation, `(source: …)` suffix, and
   `(gated: …)`/`(stage-gated: …)` suffix or `UNGATED`.
4. **Bootstrap item.** An item is classed BOOTSTRAP iff it contains the
   seeded text "Run claim-level concordance per the reconciliation method"
   with `(gated: D-41)`. Counted in `RemainingItemCount`, flagged in
   `BootstrapItemPresent`, excluded from `NonBootstrapItems`, `GateSuffixes`,
   and all selectability analysis per addendum 2.
5. **Requirement IDs.** Extracted from `Specification.md`: first cell of
   every Markdown table whose first header cell is `Req ID`/`Requirement ID`
   (any section) or `ID` (sections whose heading contains "Requirement").
   This captures the table-declared requirement rows for all 101
   deliverables; no deliverable needed a bullet-format fallback and none
   returned `NONE_FOUND`.
6. **SelectableUnderCurrentLoop (convention 6, mechanical).**
   `YES` iff (a) the deliverable's node is present in DAG-007
   `DeliverableNodes.csv` (all 101 are; `list_deliverable_status.py` reports
   `DAGNodePresent=TRUE` for every row), (b) `LifecycleState != ISSUED`, and
   (c) at least one non-bootstrap `## Remaining` item carries no gate/stage
   suffix (UNGATED). Else `NO`. Per convention 6 the owner suspension is a
   run-level caveat (see `RUN_BASIS.md`), never encoded per-row; per the
   frozen register state D-41 is `AWAITING_RULING`, so every bootstrap item
   is gated and contributes nothing to selectability.
   - **DAG-unblocked note.** DAG-007 carries no node-level blocked/unblocked
     flag. 59 deliverables have ≥1 ACTIVE UPSTREAM PREREQUISITE
     DELIVERABLE edge with `SatisfactionStatus` TBD/PENDING in
     `DependencyEdges.csv`; the R0b calibration baseline treated these
     evidence-grain statuses as non-blocking (DEL-07-05 and DEL-09-01 were
     encoded `YES` while carrying TBD prereq edges). This inventory follows
     that baseline: edge `SatisfactionStatus` is not a loop block; lifecycle
     and gate suffixes are the mechanical blockers.
   - **Baseline self-check.** The six re-encoded calibration deliverables
     reproduce their R0b-consistent values: DEL-05-03 `NO` (sole residual is
     the gated bootstrap item), DEL-07-05 `YES`, DEL-09-01 `YES`. R0-trio
     values derived here: DEL-04-01 `YES`, DEL-10-05 `YES`, DEL-12-02 `YES`
     (the R0 ledgers predate the conventions and are calibration evidence
     only per RUN_BASIS).

## Cross-check vs `tools/coordination/list_deliverable_status.py --dag DAG-007`

Run inside the frozen project root with `PYTHONDONTWRITEBYTECODE=1`
(read-only; porcelain clean after). Reconciliation:

- Tool rows: **101**; inventory rows: **101**. Identical ID sets.
- Lifecycle: **100 IN_PROGRESS, 1 ISSUED (DEL-01-01)** — identical in both.
- `RemainingItems` count matched the inventory's `RemainingItemCount` for
  **all 101 deliverables** (zero mismatches).
- `StatusVocabulary=CANONICAL` and `DAGNodePresent=TRUE` for all 101 rows.

Totals: 150 `## Remaining` items = 100 bootstrap + 50 non-bootstrap
(26 UNGATED, 18 gated, 6 stage-gated). 26 deliverables carry ≥1
non-bootstrap item; **16 are `SelectableUnderCurrentLoop=YES`**
(DEL-01-03, DEL-02-05, DEL-04-01, DEL-04-03, DEL-04-04, DEL-05-01,
DEL-07-02, DEL-07-05, DEL-07-06, DEL-08-01, DEL-08-04, DEL-09-01,
DEL-09-04, DEL-10-04, DEL-10-05, DEL-12-02); the other 10
(DEL-04-05, DEL-05-02, DEL-08-05, DEL-09-05, DEL-10-02, DEL-10-03,
DEL-14-01, DEL-15-01, DEL-16-02, DEL-16-04) have only gated/stage-gated
residuals.

## Anomalies

1. **DEL-01-01 (ISSUED)** has no `## Remaining` section and no bootstrap
   item — expected per the plan §4 packet seeding (already ISSUED) and the
   task addendum; encoded `RemainingPresent=ABSENT`, count 0.
2. **All other 100 deliverables** carry exactly one seeded bootstrap item,
   gated `D-41` — none missing, none duplicated.
3. **Requirement-ID scheme heterogeneity: 99 distinct schemes across 101
   deliverables.** 1100 ID tokens total, only 1089 distinct. Notables:
   - **Cross-deliverable ID collision:** DEL-03-03 and DEL-03-05 both
     declare the identical token set `R01`–`R11` (11 colliding IDs).
     Wave ledgers must qualify these with the deliverable ID (`ClaimID`
     format `DEL-XX-XX-<TYPE>-NNN` per addendum 12 already does this).
   - **19 deliverables use schemes that do not embed their own deliverable
     number** and are collision-prone at corpus grain: all of PKG-00
     (`REQ-01-*` … `REQ-08-*`, keyed to deliverable ordinal, not package),
     DEL-02-02 (`U-*`), DEL-03-03/DEL-03-05 (`R*`), DEL-10-05 (`R-*`),
     DEL-09-04 (`VAL-REQ-*`), DEL-09-05 (`RQG-*`), DEL-11-01 (`UG-REQ-*`),
     DEL-12-01 (`LFSP-REQ-*`), DEL-12-02 (`REXC-REQ-*`), DEL-12-03
     (`TEL-REQ-*`), DEL-12-05 (`STM-REQ-*`).
   - DEL-17-01 includes a suffixed insertion ID `DEL-17-01-REQ-002A`.
4. **DEL-17-01 and DEL-17-02 have no single `## Requirements` heading** —
   requirements are distributed across multiple `## … Requirements` sections
   (Source Authority / Boundary / Architecture-Basis / Export Package /
   Profile / Stable ID Map / Manifest / Loss Report / Downstream /
   Acceptance), all as `Req ID` tables; all captured.
5. **Compound gate parentheticals with internal semicolons** in two items:
   DEL-09-05 (`gated: D-11 waves, owner-paced; DEL-01-01 is the ISSUED
   precedent`) and DEL-15-01 (`stage-gated: v0.2 R6; DEL-17-01 vendor
   questions gate CAEPIPE MBF-specific claims`). To keep `GateSuffixes` a
   parseable semicolon-separated list, internal `;` inside a single item's
   gate/source value is rendered as `,` in the CSV (only these cells are
   affected; item truncations in `NonBootstrapItems` remain verbatim).
   DEL-08-05 and DEL-10-04 each have one item whose single parenthetical
   carries both a `gated:` and a `stage-gated:` clause — kept as one
   compound (non-UNGATED) entry.
6. **Gate vocabulary is heterogeneous:** decision-register gates (D-05b,
   D-06b, D-07b, D-11, D-12, D-38, "new D-XX …"), owner/human gates
   ("owner/counsel", "owner review", "owner threshold promotion",
   "owner sole signatory per DEC-058", "PB-TBD-003, human",
   "owner re-disposition where not closed by evidence"), and stage gates
   (R5 release/exit evidence, first release candidate, first public
   publication, v0.2 R3/R6/R7, "app-dev F3 live-binding per DEC-063").
   Wave pilots transcribe verbatim per convention 5; `UNGATED` only when
   no suffix exists.
7. **No missing kit documents:** all 101 deliverables have Specification.md,
   Datasheet.md, Guidance.md, Procedure.md, and `_STATUS.md`.

## For wave pilots

- `NonBootstrapItems` cell format: items joined by `; `, each as
  `[<first-120-chars-verbatim>] src=<source suffix or NONE_NAMED>`;
  `GateSuffixes` is order-aligned with those items.
- Selectability here is deliverable-grained (any ungated residual);
  claim-row encoding in wave ledgers stays per-residual per addendum 3.
- The DEL-09-01 fixture/witness count repair (21, per Part C of
  `R1_CONVENTIONS.md`) applies to that deliverable's R2 wave ledger, not to
  this inventory.
- Run-level caveats (owner suspension; D-41 frozen-register observability)
  live once in `RUN_BASIS.md` — do not re-derive per row.
