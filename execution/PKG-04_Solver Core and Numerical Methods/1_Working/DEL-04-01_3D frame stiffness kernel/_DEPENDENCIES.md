# Dependencies: DEL-04-01 3D frame stiffness kernel

## Generated Dependency Register
- **Status:** REFRESHED_TP_DAG_004
- **Source Basis:** `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.7 plus local sealed context `_CONTEXT.md`
- **Local Register:** `Dependencies.csv`
- **Rows:** 8 total; 8 ACTIVE; 0 CANDIDATE.
- **Generated:** 2026-05-03
- **Refreshed:** 2026-05-10
- **Mode:** UPDATE
- **Strictness:** CONSERVATIVE
- **Consumer Context:** RECONCILIATION

## Refresh Summary
- Edge set unchanged from the prior local register.
- All rows remain `RegisterSchemaVersion=v3.1` and `Status=ACTIVE`.
- SCA-001 architecture-basis rows remain satisfied context-injection evidence for AB-00-01, AB-00-02, AB-00-03, AB-00-06, and AB-00-08.
- Inferred PKG-02 upstream rows remain unresolved as `SatisfactionStatus=TBD`; this refresh does not promote maturity or compute blockers.
- No new SCA-002 design-engine dependency was added for DEL-04-01 because the frame stiffness kernel consumes solver-ready analytical contracts and the current decomposition does not make physical-design transformation a direct prerequisite for this deliverable.

## Authority Boundary
- Aggregate `DAG-002` remains the sequencing and blocker-computation authority within its approval boundary.
- This local register is a synchronized mirror/evidence surface, not an independent graph authority.
- `CANDIDATE` rows remain non-gating until later RECONCILIATION plus CHANGE approval.
- `PKG-00` architecture-basis rows are preserved here as injected context evidence; `PKG-00` does not receive local dependency registers.

## Validation Notes
- Schema target: dependency register v3.1.
- Enum validation target: `DependencyClass`, `AnchorType`, `Direction`, `DependencyType`, `TargetType`, `Explicitness`, `RequiredMaturity`, `ProposedMaturity`, `SatisfactionStatus`, `Confidence`, `Origin`, and `Status`.
- Closeout result: schema and enum validation passed on 2026-05-10.
