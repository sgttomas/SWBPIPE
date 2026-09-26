# Setup and dependency workflow update

Owner-authorized Root tranche `ROOT-WORKFLOW-WAVE2A-SETUP-DEPS-20260926` revises bundled setup and dependency workflows and tools from run evidence:
- `tools/validation/validate_id_format.sh` accepts the 2- and 3-digit PKG/DEL/DEP (and CAT/KTY/SUB) identifiers used by TYPES and SPEC §6.8; it previously rejected every current project ID. A new test covers it.
- `dependency-extract` restores literal enum and CSV values corrupted by the 2026-09-09 rewrite (`AGGREGATION`, `RECONCILIATION`, `SOFTWARE_DECOMP.md`), declares its ID check, adds `ScopeOfWork.md` to `DOC_ROLE_MAP`, defines `RESET_EXTRACTED`, and states the DependencyID format and its relation to register-validator findings EVQ-003/EVQ-004/DRB-006. Its `_DEPENDENCIES.md` headings and mode vocabulary are unchanged pending a separate alignment.
- `project-setup` passes only values its callees accept, no longer claims a scope-of-work pass sets SEMANTIC_READY, adds a Phase 2.2b dependency stage (dependency-extract, audit-dep-closure, SCC cases; project-DAG construction stated as not yet provided by a bundled workflow), makes unresolved cycle edges non-gating, and documents scaffolding order and the label rule.
- `audit-dep-closure` makes the pointer move brief-controlled (default off), derives its inventory from accepted registers with exemptions, and matches the analyzer's actual outputs and `--prior-summary`.
- `audit-decomp` leaves the pointer move to the manager, adds `EXPECTED_CONSEQUENCE`, retired-unit rules and a Check 8 basis, names its tools and companion registers; a missing ledger is now a WARNING.
- `tools/evaluation` audits scan `1_Working`, `2_Checking` and `3_Issued`.

Historical runs are not rewritten. This loop decides its own adoption; this source tranche grants no release.
