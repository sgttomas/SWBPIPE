# DEL-10-02 Import/export adapter framework — worker notes (W3 PKG-10 G1)

Forward ledger `DEL-10-02_forward.csv` (72 rows: 70 required keys and 2
`.sNN` sub-claims of the Architecture Basis Injection block). It is sealed in
`DEL-10-02_SEAL.txt`. The reverse file `DEL-10-02_reverse.csv` covers 387
capabilities. Shared judgments are in `../_WORKER_DEL-10-01_NOTES.md`.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework/`.
- Implementation files:
  - `core/adapters/framework/adapter_framework.py`, which holds
    `validate_adapter_declaration` and `gate_adapter_runtime_dispatch`;
  - `schemas/adapter_framework.schema.yaml`;
  - `fixtures/adapters/invented/invented_adapter_framework.json`;
  - `tests/test_adapter_framework_contract.py`.
- Desktop preview: `apps/desktop/src/features/adapter-framework/AdapterFrameworkPanel.tsx`.
- SOW parity records sit at the repository root under `execution/_Coordination/AgentRuns/SOW-STAGE2-EXEC-20260712-01/`.

## Judgment calls

1. **Declaration-only seam (F7 and CP-11; FG-DEL-10-02-01).** The framework
   only validates adapter declarations. It parses no external file and runs no
   import or export. The gate never dispatches: it returns
   `BLOCKED_RUNTIME_NOT_SELECTED`. The Python validator and gate are called
   only by tests.
   - Claims about the validator or gate are `ALIGNED` with
     `PRODUCT_CALLER: NONE`: CLM-003, CLM-004, CLM-010, REQ-01, REQ-06,
     REQ-09, REQ-10, CLM-024.
   - Claims about behaviour on import paths, export paths or hooks are
     `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`: REQ-02 to REQ-05, REQ-07,
     REQ-08, CLM-005, CLM-006, CLM-020, AC-001, the OUT-001 purpose statement
     and the CONTEXT description. The obligation exists only as a required
     declaration field.
   - `AuthorityNeeded OWNER` on REQ-02 to REQ-05, REQ-07 and REQ-08, because
     the runtime and execution model are held for the owner (STATUS R02).
2. **REQ-10-02-07** cites DEC-074 O7/E5. The option text sits in an excluded
   July `PROPOSED_*` file and was not read (flag per R0 review §7). The
   codified DEC-074 row is the basis used.
3. **CLM-013 "Current bounded verification includes …"** is a T3-era
   declaration (`ed3a79e15`). No unit-conversion, dimensional-consistency,
   private-export-warning or adapter report-boundary test was located. The
   selected-seam tests exist. Disposition: `PARTIALLY_IMPLEMENTED`.
4. **STATUS R01 and R02** are accurate open actions, so under F2 they are
   `ALIGNED`:
   - R01 carries `OPEN_ACTION: REQ-10-02-07`;
   - R02 carries `OPEN_ACTION: REQ-10-02-02`.

   **STATUS R03** is CP-07: D-12 was ruled (DEC-078) without dispositioning
   the FR-023 residual, so its "or D-12" gate has passed.
5. **Stale citations.** CLM-005 and several REQ source columns cite
   `INIT.md`, which is not in the frozen project, and PRD sections 6.6, 13.5,
   18.2 and 18.3, which PRD v0.4 renumbered. These are recorded in Notes (J7).
   CLM-007 References takes CP-01, because it points at PKG-00
   `Specification.md` files that D-43 replaced with `ArchitectureBasis.md`.
6. **CLM-019 Steps** is `ALIGNED`. It is method guidance for adding hooks; it
   does not state that hooks exist. Hook state is judged on REQ-07 and REQ-08.

## Possible defect for the verifier (not keyed; no row changed)

`REQUIRED_TBD_DECISIONS` in `adapter_framework.py`, and `FrameworkStatus` in
the schema, still require `ci_provider` and `physical_project_container` to be
`TBD`. The validator rejects a declaration that resolves either one, although
both have since been ruled: the CI provider by D-05/DEC-025 and the package
container by D-09/DEC-028. No DEL-10-02 claim key covers this list. CLM-019
step 12 names only the runtime, grant, format and redaction TBDs. It is
recorded here for R3; it may be a code-behind-ruling item.

## Canonical departures

None. All seven keyed CS rows inherit their fields.

## Convention friction

- F7 separates engine claims from runtime claims. In this SOW the two are
  interleaved within single blocks, for example Conditions and Construction.
  Each block was judged by its unmet runtime elements.

## UNKNOWN rows

None.

## Reverse pass

- `CLAIMED_BY`:
  - RC-10-0048 (gate);
  - RC-10-0282 (validator);
  - RC-10-0277 (schema);
  - RC-10-0179 (invented fixture).
- `PARTIAL`: RC-10-0202 (desktop adapter-framework panel). It renders this
  deliverable's packet, but no key covers the GUI surface.
- Every other capability is `NOT_MINE`. The paths that overlap with this
  ledger's evidence each have a capability-specific reason (F5):
  - the API contract (DEL-10-01);
  - the registers;
  - `CONTRACT.md`.
- RC-10-0007 (plugin verification in the same package folder, DEL-02-04) got
  a specific `NOT_MINE` reason.

The reverse pass did not change my view of any sealed row.

## Batch consistency

`--batch` over the three forward ledgers: **PASS, 0 findings**.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9). Since
2026-09-19, Piping selects work through owner-steered work graphs, not
through `## Remaining`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
