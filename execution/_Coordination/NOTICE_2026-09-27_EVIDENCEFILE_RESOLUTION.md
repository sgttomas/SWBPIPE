# EVQ-006 resolves EvidenceFile in each allowed form

Owner-directed Root tranche `ROOT-EVIDENCEFILE-RESOLUTION-20260927` (manifest `docs/governance_harness/tranche_manifests/ROOT-EVIDENCEFILE-RESOLUTION-20260927.yaml`) changes `tools/validation/validate_decomposition_registers.py`, which every loop that checks decomposition registers runs:
- **`EVQ-006` now tries three bases and fires only when none names a regular file.** The validator used to resolve `EvidenceFile` only from the project root (the parent of the execution root). Root SPEC §6.5 calls the cell "the source document filename", and `dependency-extract` writes `EvidenceFile=_DEPENDENCIES.md`, so a bare filename relative to the deliverable folder is a valid form. SPEC §0.2.4 adds working-root references and instruction-root references. The validator now tries the deliverable folder, then the working root (`--evidence-root`), then the instruction root (new `--instruction-root`, default: the checkout holding the tool).
- **Still findings.** Absolute paths, paths that leave both roots, directories and files that exist in no form. The JSON report gains `instruction_root` and `evidence_file_resolution_forms`.
- **Unchanged.** Every other check, its severity and the exit codes. No register was edited.

The owner's direction, 2026-09-27: "You can take care of those remaining items now.  Include the items with the "other owners".  You can make changes as necessary."

Piping-specific: `EVQ-006` falls from 701 to 504. The 197 newly resolved cells are deliverable-relative, mostly `_CONTEXT.md`, and 870 cells resolve working-root-relative as before. The 504 that remain are real:
- 126 bare filenames (109 `ACTIVE`, 17 `RETIRED`) naming `Specification.md`, `Datasheet.md`, `Procedure.md` or `Guidance.md`, which are not in the deliverable folder;
- 333 paths (313 `ACTIVE`, 20 `RETIRED`) to files that do not exist. 330 name one of those four files in the row's own deliverable folder; 3 name `plans/DAG-001_EXECUTION_DEPENDENCY_GRAPH_PLAN.md`;
- 45 cells (44 `ACTIVE`, 1 `RETIRED`) that list several files separated by `;`. The schema names one source document, so the validator reads such a cell as one path and does not split it. In 44 of these cells every listed file exists.

`DRB-006` (1,395) and `EVQ-004` (1) are unchanged, and the exit code is still 1. Repairing the rows is this loop's call through `dependency-extract` or its own register conventions. This loop decides its own adoption; this source tranche grants no release.
