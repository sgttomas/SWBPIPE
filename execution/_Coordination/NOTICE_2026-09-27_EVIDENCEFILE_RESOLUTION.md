# EVQ-006 resolves EvidenceFile in each allowed form

Owner-directed Root tranche `ROOT-EVIDENCEFILE-RESOLUTION-20260927` (manifest `docs/governance_harness/tranche_manifests/ROOT-EVIDENCEFILE-RESOLUTION-20260927.yaml`) changes `tools/validation/validate_decomposition_registers.py`, which every loop that checks decomposition registers runs:
- **`EVQ-006` now tries three forms and fires only when none names a regular file.** The validator used to resolve `EvidenceFile` only from the project root (the parent of the execution root). The forms now tried are:
  1. relative to the deliverable folder. Root SPEC §6.5 calls the cell "the source document filename", and `dependency-extract` writes `EvidenceFile=_DEPENDENCIES.md`;
  2. relative to the project root (`--evidence-root`), per SPEC §0.2.4;
  3. relative to the instruction root, but only for the instruction surface SPEC §0.2.4 names: `agents/`, `workflows/`, `tools/`, root `docs/` and `AGENTS.md`. The instruction root is `--instruction-root`, else `CHIRALITY_INSTRUCTION_ROOT`, else the checkout holding the tool.
- **Still findings.** Checkout-relative paths to project material (`projects/<name>/...`), paths that leave the project root, absolute paths, directories and files that exist in no form.
- **Order matters.** Forms are tried in the order above. A bare name missing from the deliverable folder can resolve to a same-named file at the project root. The JSON report gains `instruction_root`, `instruction_root_source`, `evidence_file_resolution_forms` and `evidence_file_multi_form`.
- **Unchanged.** Every other check, its severity and the exit codes. No register was edited.

The owner's direction, 2026-09-27: "You can take care of those remaining items now.  Include the items with the "other owners".  You can make changes as necessary."

Piping-specific: `EVQ-006` falls from 701 to 504. The 197 newly resolved cells are relative to the deliverable folder, mostly `_CONTEXT.md`. The 870 cells relative to the project root resolve as before, and no cell needs the instruction root. The 504 that remain are real:
- 126 bare filenames (109 `ACTIVE`, 17 `RETIRED`) naming `Specification.md`, `Datasheet.md`, `Procedure.md` or `Guidance.md`, which are not in the deliverable folder;
- 333 paths (313 `ACTIVE`, 20 `RETIRED`) to files that do not exist. 330 name one of those four files in the row's own deliverable folder; 3 name `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`;
- 45 cells (44 `ACTIVE`, 1 `RETIRED`) that list several files separated by `;`. The schema names one source document, so the validator reads such a cell as one path and does not split it. In 44 of these cells every listed file exists.

`DRB-006` (1,395) and `EVQ-004` (1) are unchanged, and the exit code is still 1. Repairing the rows is this loop's call through `dependency-extract` or its own register conventions. This loop decides its own adoption; this source tranche grants no release.
