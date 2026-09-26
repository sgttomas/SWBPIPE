# Package home for every scope item (D-GOV-48)

Owner-directed Root tranche `ROOT-PACKAGE-HOME-D-GOV-48-20260926` (`docs/governance_harness/_DECISIONS/D-GOV-48_every_scope_item_package_home.md`) supersedes the Package-home ruling of D-GOV-47 (`NOTICE_2026-09-26_DECOMP_RULINGS_D-GOV-47.md`) for PROJECT and SOFTWARE:
- **Every scope item has exactly one Package home.** Every Scope Item, whether IN, OUT or TBD, belongs to one Package. The home allocates accountability and traceability. Only IN items map to Deliverables; OUT and TBD items keep their home and `SourceRef` with no production mapping. Material recorded as TBD at group 1, before Packages exist, is homed at group 2. `UnassignedScopeItems` counts every item without a Package and must be 0 for acceptance. This is the rule the management manual v7 and Field Book already state. Changed in `docs/TYPES.md` §1.1/§8.1, the decomposition standard (I4, ledger, glossary, extension contract), and the `software-decomp` and `project-decomp` contracts and methods.
- **Validator.** `validate_decomposition_registers.py` keeps `XRG-011` (ERROR: IN item without `PackageID`), retires `XRG-012` (an OUT/TBD item with a `PackageID` now conforms), and adds `XRG-013` (WARNING: OUT or TBD item without `PackageID`).
- **Unchanged.** D-GOV-47's combined review sitting, and DOMAIN (`domain-decomp` keeps its IN-only Category rule).

In-flight decompositions keep the rule and edition they adopted; nothing is retrofitted. This loop decides its own adoption; this source tranche grants no release.

Piping-specific: the XRG family skips this loop because it has no `_Decomposition/ScopeLedger.csv` under its execution root. Piping's register ledger (`docs/_Registers/ScopeLedger.csv`, 79 IN items, each with a `PackageID`) already conforms.
