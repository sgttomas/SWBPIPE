# XRG-004 allows supporting deliverables in another Package

Owner-directed Root tranche `ROOT-XRG004-SUPPORTING-DELIVERABLES-20260926` (manifest `docs/governance_harness/tranche_manifests/ROOT-XRG004-SUPPORTING-DELIVERABLES-20260926.yaml`) changes `tools/validation/validate_decomposition_registers.py`, which every loop that checks decomposition registers runs:
- **`XRG-004` is now a WARNING and fires only when an item's home Package holds none of its linked deliverables.** It was an ERROR for every linked deliverable outside the item's `PackageID`. That contradicted D-GOV-48 and management manual v7: "Several Deliverables may contribute to an included obligation, including supporting contributions from another Package." A linked deliverable in another Package is now a supporting contribution, not a finding. The check reports once per item, and not at all for an item that `XRG-014` already reports for naming several Packages.
- **New option `--registers-dir DIR`.** It reads `Deliverables.csv`, `ScopeLedger.csv` and `ContextBudgetQA.csv` from `DIR` instead of `<EXECUTION_ROOT>/_Decomposition/`. The default is unchanged. The JSON report gains a `registers_dir` field.
- **Unchanged.** Every other check, its severity and the exit codes.

The owner's ruling, 2026-09-26: "Warn only if home holds none (Recommended)".

Piping-specific: this loop's own records of the same change are `D-77` and `DEC-115` (Package homes, applied in the same change under `PIPING-PACKAGE-HOMES-DAG-WORDING-20260926`) and `D-78` (D-GOV-49 wording). This loop's registers can now be checked directly:

```
python3 tools/validation/validate_decomposition_registers.py projects/chirality-piping/execution --families XRG --registers-dir projects/chirality-piping/docs/_Registers
```

With the `D-77` homes it reports no `XRG-014` and no `XRG-004`. The earlier `XRG-005` (29 errors) and `XRG-008` (106 warnings) findings remain; they are separate matters for this loop. Run over all families, `--registers-dir` also lets the DRB checks read `Deliverables.csv`, which surfaces the pre-existing `DRB-004` ×14 (warnings) and `DRB-008` ×8 (warnings) findings that a run without the option cannot see. This source tranche grants no release.
