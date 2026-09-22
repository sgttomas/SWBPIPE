# DEL-15-02 notes: rerun 1 (W3, PKG-15, worker G1)

This is a rerun under the worker brief's rerun clause. Before anything else
was written, the first worker's four files were moved unchanged into
`superseded_1/`. Their SHA-256 values after the move:

- forward: `86d7e398…3ac73`
- seal: `bb82ea0c…c5c26`
- reverse: `203d82b4…c867c`
- notes: `9a02ace7…706bd`

I did not read their contents. The forward ledger was encoded afresh and
sealed at `989d1fc9…13d75`. I read the verification report's DEL-15-02
findings (`PKG-15_VERIFICATION.md`, SHA-256 `9c9698b7…fb616`, matched) and
the routing file only after sealing.

## Path aliases

- `D/` means the deliverable folder:
  `projects/chirality-piping/execution/PKG-15_Handoff and External Prover Workflow/1_Working/DEL-15-02_Target mapping and unsupported-behavior contract/`.
  Evidence tokens use the full path, which contains spaces and resolves at
  the freeze.
- Project-root tokens (`core/…`, `schemas/…`, `tests/…`) resolve under
  `projects/chirality-piping/`.
- Governing documents are always cited as `projects/chirality-piping/docs/…`.
- The parity records cited are root-level
  `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/…` files.
- `#Lnn` anchors refer to `core/handoff/target_mapping/contract.py` and
  `schemas/target_mapping.schema.json` at `00115c719`.

## Finding groups

**FG-01: silent defaults (POSSIBLE_DEFECT, INVARIANT, VALIDATION).**
`_mapping_record` and `_behavior_flag` fill in absent inputs:

- `mapping_status` becomes `mapped`;
- `mapping_kind` and `value_kind` become `metadata`;
- provenance becomes the engine's `ENGINE_PROVENANCE`;
- refs become `TBD`/`missing`.

Diagnostics run on the already-defaulted contract, so none of these absences
is reported. The group has 12 rows:

- CLM-005.r02, R007 (CLM-011.r07), CLM-020.s01, CLM-021.r03, CLM-026,
  CLM-027.r01;
- AC-001;
- CONTEXT#context-envelope;
- MEMORY (its undated "Boundary Decisions" section).

**FG-02: units (PARTIAL_SLICE, INVARIANT, VALIDATION).** The unit check only
tests that metadata is present. It applies only when `value_kind` is one of
three names, and nothing checks the unit against the declared dimension.
Rows: CLM-005.r01, R005 (CLM-011.r05), CLM-027.r02.

**FG-03: provenance (POSSIBLE_DEFECT, INVARIANT, VALIDATION;IP_DATA).** When
no provenance is supplied, the engine's own provenance is substituted,
labelled `public_permissive` / `public_metadata`. The `library_refs` and
`rule_pack_refs` in `source_context` are closed Reference objects, so their
redistribution and review status cannot be carried. Rows: CLM-005.r03,
R006 (CLM-011.r06), CLM-027.r03.

**FG-04: construction notes name fields that never existed (IMPLEMENTED_DIFFERENTLY,
OTHER, LOCAL_DESIGN).** `git log -S` finds no `source_entity_ref`,
`target_field_ref` or `behavior_kind` in `core/` or `schemas/`, ever. Flags
have no severity, description or per-flag diagnostics field, and
`source_context` has no package reference. Rows: CLM-006.r03 to r05.

**FG-05: the promise to keep the dependency mirror unchanged was overtaken.**
The 2026-06-16 v3.1 normalization, the DAG-007 human approval of 2026-06-22,
and DAG-010 (now current) overtook it. Rows:

- CT-001;
- V-005 and CLM-021.r06, which F3 makes setup text;
- CLM-020.s02;
- CLM-019.r02, whose cause is the pin.

**FG-06: "canonical package container TBD" is overtaken by DEC-028 and
DEC-017.** Rows:

- CLM-004.r07 and CLM-020.s03;
- CLM-012.r02 and CLM-019.r09, declared after the migration;
- CONTEXT#architecture-basis-injection.s02.

Confidence is MEDIUM: DEC-028 names "native project package and public
transport form", and applying it to the handoff container is my reading.

## Judgment calls

- **Blocks split into `.rNN` rows** (C1): CLM-004, 005, 006, 011, 012, 019,
  021 and 027. They are split because their rows take different
  dispositions, and each such block is CONTAINER. CLM-003, 029 and 030 are
  assessed as single blocks, because every row in them aligns.
- **CLM-005 and CLM-027 (the rerun trigger).** Once split, their Units and
  Missing-values rows join FG-02 and FG-01 at INVARIANT. The Provenance row
  joins FG-03. The other three conditions (private data, claim fence,
  handoff role) are ALIGNED. The blocks themselves are CONTAINER, so no
  quiet row carries the gap.
- **CLM-005.r04 (private data) is ALIGNED** with a GAP_WORDING_CHECKED
  clause. The default `public_metadata` label on engine provenance is a
  provenance-labelling defect (FG-03), not an exposure of payload. Redaction
  must be affirmed before the builder emits a contract without a blocking
  diagnostic. The verifier should check this row.
- **The F7 claim-subject rule.** Every ALIGNED row whose evidence is the
  builder or the schema carries `PRODUCT_CALLER: NONE`.
  `build_target_mapping_contract` is imported only by tests. The desktop
  `HandoffPanel` emits its own `target_mapping` preview object, which does
  not follow `schemas/target_mapping.schema.json`. The claims in this ledger
  are about the contract, so the engine satisfies them.
- **CLM-011.r12 and V-006 are ALIGNED as restraints.** R012's "until
  accepted" condition is satisfied, not contradicted, by DEC-028. CP-11 is
  not engaged.
- **CT-001 is STALE_REVIEW_OR_EVIDENCE, not CP-10.** An approved governing
  record, the DAG-007 approval, adopted the normalized enums. CT-001's
  Source A path `skills/dependency-extract/SKILL.md` does not exist at the
  freeze; the method is now `workflows/dependency-extract/`.
- **CLM-021.r07 is UNKNOWN · EVIDENCE_NOT_LOCATED under CP-12.** The last
  record of the dependency-validator pass predates the 2026-06-16 rewrite,
  and the sweep does not run that tool.
- **CLM-004.r04 is STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE**, with no
  CP tag. The cited SOW-074 note has not said "deferred" since the initial
  migration, so the F3 origin rule applies to this source-pointer defect.
- **CONTEXT#architecture-basis-injection.s03 is UNKNOWN · AUTHORITY_UNCLEAR ·
  PROJECT_BASELINE, with LOW confidence.** The contract exists only in
  Python, while the Resolved Baseline names "Rust core/application
  services". This is a project-wide question and is raised once here for R3.
- **STATUS#remaining/R01 is DOCUMENTED_UNIMPLEMENTED · NOT_STARTED ·
  AuthorityNeeded REVIEW (F2).** The text is accurate. No governing row in
  this ledger carries the human finding dispositions.
- **AdoptedByReference YES** is set on CLM-008, 015, 023 and 033. They rest
  on DEC-074 O7, whose adopted text is unread excluded material.

## Canonical departures

None. All 9 CS assignments are inherited as issued.

- CP-03 applies to CLM-002, 017 and 025.
- CP-02 applies to CLM-007, CLM-019.r02 and the CONTEXT surface.
- CP-01 applies to CLM-014, CLM-021.r01 and CLM-022.
- CP-04 applies to the SOW surface, default variant. The schema's `$id` and
  title and the `ENGINE_PROVENANCE` strings carry the former name.
- CP-05 applies to STATUS.
- CP-09 applies to the matrix OUT-001.
- CP-12 applies to CLM-021.r07.

## Convention friction

- **F3 and section references.** F3 exempts pins, review states and metadata
  from the origin test, but not section references, whereas CP-02 names
  section references under STALE_REVIEW_OR_EVIDENCE. For migration-origin
  source pointers I applied F3 (CLM-004.r04), without a CP tag.
- **F8 across one finding.** Rows restating the same gap take the same tier,
  even where a row, such as CLM-026 or the context envelope, does not cite
  the invariant itself. Verification-coverage rows keep LOCAL_DESIGN because
  their gap is a missing check, not the invariant: V-004 and VER-001.
- **The DivergenceLayers vocabulary has no units or data-completeness layer.**
  I used VALIDATION ("validation and provenance") for OPS-K-UNIT-1 and
  OPS-K-DATA-2 gaps. The first verifier suggested BASELINE with BaselineClass
  NONE. R3 may want one choice for the corpus.

## Smallest checks for UNKNOWN rows

- **CLM-021.r07.** Run
  `projects/chirality-piping/tools/validation/validate_dependencies_schema.py`
  read-only on the frozen `D/Dependencies.csv` and record the result.
- **CONTEXT#architecture-basis-injection.s03.** The owner or R3 gives one
  project-wide answer on whether the Python contract builders under
  `core/handoff` fall within the "Rust core/application services" baseline.

## After sealing: verification report and reverse pass

- **The report confirms the core finding.** It confirmed by code reading that
  omitted `mapping_status`, `mapping_kind` and `value_kind` default silently
  and skip the unit check. My FG-01 and FG-02 carry the same substance.
- **Where my ledger departs from the report's suggested values:**
  - **Disposition.** I used PARTIALLY_IMPLEMENTED, not
    IMPLEMENTED_DIFFERENTLY, following F1 ("the disposition of the unmet
    element, usually PARTIALLY_IMPLEMENTED").
  - **Layers.** I used VALIDATION, not BASELINE; see Convention friction.
  - **Findings the report did not name:** the absence of any dimensional
    check (FG-02), the provenance substitution and closed references
    (FG-03), and the container TBD being overtaken (FG-06).
  - **Finding-group numbers differ from the first worker's.** Mine are
    FG-01 for silent defaults and FG-02 for units.

  Reading the report did not change my view of any sealed row.
- **The reverse pass did not change my view of any sealed row either.** One
  observation for R3: in DAG-010, `DEL-15-02-D001` and `D002` (the handovers
  to DEL-15-03 and DEL-15-04) are RETIRED, while the local
  `Dependencies.csv` still shows them ACTIVE. No DEL-15-02 claim key asserts
  those rows, so the ledger has no row for this drift.
- **Reverse answers:**
  - CLAIMED_BY: RC-15-0235 (builder) and RC-15-0253 (schema).
  - COVERS: RC-15-0001, the shared authority-term screen that the builder
    calls.
  - PARTIAL: RC-15-0108. HandoffPanel's `target_mapping` block is DEL-15-02
    subject matter, but it does not conform to the schema.
  - NOT_MINE: 286. Where a capability's EntryPoints hit a path the forward
    ledger cites, the reason addresses that capability (F5): RC-15-0025,
    0046, 0072, 0103, 0127, 0152, 0158, 0212, 0226 and 0268.

## Batch consistency

`--batch` over the one forward file returned PASS with 0 consistency
findings. The parent runs the package batch. Shared bodies to watch there:

- `output-and-evaluation-matrix` and its OUT-001 (SharedTextCount 2):
  CONTAINER, and CP-09 STALE_REVIEW_OR_EVIDENCE · EVIDENCE_OVERTAKEN.
- The CS-keyed CONTEXT rows, which are inherited as issued.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
`## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). The
dispositions here are agent judgments, not owner rulings. They make no
release, approval, compliance or certification claim.
