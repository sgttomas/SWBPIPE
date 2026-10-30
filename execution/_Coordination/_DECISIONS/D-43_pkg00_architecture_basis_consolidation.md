# D-43 — PKG-00 Architecture-Basis Consolidation

Date: 2026-07-15
Authority: `HUMAN` — owner instruction of 2026-07-15: "proceed with the
in-place consolidation of PKG-00 and complete the cleanup targets list."
Status: `RULED — IMPLEMENTED BY THIS TRANCHE`
Basis: HUMAN-STEER-PKG00-EXCLUSION-001 (PKG-00 is retained
governance/architecture-basis context, not a conversion package); root
Stage-2 closure `STAGE2_CLOSED — H2-R COMPATIBILITY RETAINED`; the
2026-07-15 eight-member validity audit finding all members
PARTIALLY_STALE in wrapper metadata with normative content valid and
load-bearing.

## Ruling

Each of the eight Piping `PKG-00` members is consolidated in place from its
four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`,
`Guidance.md`) into one canonical reference document, `ArchitectureBasis.md`,
carrying the schema marker `chirality-architecture-basis/v1` and the
classification `ARCHITECTURE_BASIS_REFERENCE`.

1. **Identity preserved.** Member directories, `DEL-00-0X` identifiers, and
   all `REQ-0X-YY` requirement identifiers are unchanged. Downstream
   citations by identifier remain resolvable.
2. **Currency folded in.** The D-41 R5 T7 PDU-054/PDU-055 current
   declarations, accepted refinements previously recorded only in member
   `MEMORY.md` (notably TP-DIAG-019 for REQ-06-02), ruled former TBDs
   (DEC-009, DEC-017, DEC-019, DEC-023, DEC-025/059, DEC-026/060, DEC-028,
   DEC-033, DEC-046, DEC-057, DEC-064/065, DEC-074), the current package
   universe (`PKG-01` through `PKG-17`), and the `SOFTWARE_DECOMP.md`
   revision pin are stated as current normative text. Genuinely open items
   (public API transport protocol, endpoint syntax, external format list,
   GUI state-library selection) remain listed as open.
3. **No lifecycle act.** `_STATUS.md`, `MEMORY.md`, `_SEMANTIC.md`,
   `_CONTEXT.md`, and `_run_records/` are byte-identical. The D-41 R5 T6
   PDU-007 formal-review hold on `DEL-00-02` REQ-02-05 and the
   TP-SEAM-WASM-001 boundary anomaly routed to REVIEW remain open and are
   carried in the affected member's `ArchitectureBasis.md`.
4. **Sources hash-bound, then removed.** The 32 kit files are hash-bound in
   `execution/PKG-00_Software Architecture Runway/CONSOLIDATION_MANIFEST.md`
   and removed from the working tree; their bytes remain in git history at
   the pre-consolidation commit recorded in the manifest.
5. **Format-census posture.** PKG-00 members are outside the
   PROJECT/SOFTWARE production-contract population (per the exclusion steer
   and the frozen Stage-2 census). `resolve_production_format` intentionally
   does not recognize `ArchitectureBasis.md`; PKG-00 directories must not be
   passed to production-format tooling. This is a documented boundary, not a
   defect.
6. **Preserved-source citations.** Converted `ScopeOfWork.md` documents and
   `_SEMANTIC*` control files that quote kit paths inside preserved source
   blocks are NOT edited: those bytes are bound by the Stage-2 preservation
   gates and describe the source basis as it existed at conversion time.
   Such paths resolve via git history; the live successor for every cited
   kit surface is the member's `ArchitectureBasis.md`.
7. **Relation to H2-R.** The root H2-R ruling retained the eight PKG-00
   legacy kits pending a later act. This decision is that later act for the
   PKG-00 files themselves, under direct owner authority. Retirement of the
   root compatibility machinery (`four-documents` skill, legacy validator,
   format-aware callers) remains a separate root-level ruling; after this
   consolidation there are zero live four-document kits in either project,
   which that future ruling may cite.

## SCA-006 — scope-neutral decomposition currency amendment

Under the same owner authority, `SOFTWARE_DECOMP.md` advances revision
0.8 → 0.9 recording SCA-006. SCA-006 changes no scope item, dependency,
requirement, or package membership. It corrects stale currency notes that
contradict ruled decisions and re-states PKG-00's classification:

- SOW-059 notes (`SOFTWARE_DECOMP.md` §scope table and detail table;
  `docs/_Registers/ScopeLedger.csv` SOW-059 row): "migration framework
  remains TBD" → ruled by DEC-019 (D-08), refined by DEC-033.
- DEC-010 notes: import/export container ruled by DEC-028 and DEC-057;
  migration framework ruled by DEC-019; public transport protocol remains
  TBD.
- Closing statement: `PKG-00` is retained architecture-basis reference
  context per HUMAN-STEER-PKG00-EXCLUSION-001 and this decision, not a
  production package awaiting issuance.
- `docs/_Registers/Deliverables.csv` DEL-00-01..08 rows: artifact column
  updated to name `ArchitectureBasis.md` plus the realized artifact
  locations (with owning deliverable where owned elsewhere); notes columns
  corrected where contradicted by ruled decisions (solver library DEC-023;
  CI posture DEC-025/DEC-059). Notes that remain genuinely open are kept.
- Member `_DEPENDENCIES.md` / `_REFERENCES.md` revision pins updated to the
  current decomposition revision.

## Explicitly out of scope

- Any edit to converted `ScopeOfWork.md` deliverables (including the
  DEL-09-05 preserved-source reference to DEL-00-08's former Specification;
  its live successor citation is a matter for that deliverable's own
  production workflow).
- Any root canon change (`docs/TYPES.md` / `docs/SPEC.md`). A root
  reference-classification type for architecture-basis documents is a named
  follow-up for HELPS_HUMANS, not part of this act.
- Resolution of the PDU-007 hold or the TP-SEAM-WASM-001 anomaly (both
  remain routed to REVIEW).
- Retirement of root four-document compatibility machinery.

## Rollback

Revert the consolidation commit(s) on this branch. No history rewriting; the
manifest binds pre- and post-consolidation identities.

## Post-consolidation alignment verification (2026-07-15)

Downstream dependency surfaces were swept after the consolidation commit:

- Zero live dependency surfaces (`Dependencies.csv`, `_DEPENDENCIES.md`,
  `_CONTEXT.md`) bind the removed kit file paths; all 93 downstream
  `Dependencies.csv` and 93 `_CONTEXT.md` surfaces bind by member ID
  (`DEL-00-0X`) or basis-row ID (`AB-00-0X`), both unchanged.
- `SOFTWARE_DECOMP.md` AB-00-01..08 rows carry constraint text and member
  IDs only; no kit file references.
- `audit_dag.py --strict` on DAG-007 passes: edge schema valid, 1480 edges /
  101 nodes, 0 endpoint issues, 0 SCCs. Edge rows anchor member
  directories (retained) and downstream `_CONTEXT.md` sections (untouched).
- Known stale descriptor, no mechanical consumer: DAG-007
  `DeliverableNodes.csv` `HasFourDocumentKit=TRUE` for the eight PKG-00
  nodes. DAG snapshots are immutable derivatives; the next DAG refresh
  regenerates node attributes and should reflect the consolidated format.
- Pre-existing (not introduced here): downstream `_CONTEXT.md` basis
  injections still pin decomposition revision 0.7, describe PKG-00 as
  `SEMANTIC_READY`, and list since-ruled items as TBD. These sealed
  setup-era contexts predate D-40/D-43 and remain the concordance
  program's territory; this decision does not edit sealed contexts.
