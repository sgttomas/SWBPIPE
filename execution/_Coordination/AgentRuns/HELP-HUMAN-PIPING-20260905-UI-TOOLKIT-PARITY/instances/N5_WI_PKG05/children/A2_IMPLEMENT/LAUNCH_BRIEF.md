# Sealed held implementation brief
RequestedBy: WORKING_ITEMS N5_WI_PKG05; ParentRun: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY.
ChildInstanceID: A2_IMPLEMENT; role: ephemeral generalist Agent2 (no delegation).
PackageID: PKG-05. DeliverableIDs: DEL-05-01; Scope SOW-013 OBJ-003, OUT-001 AC-001 VER-001 and requirements 004/005/009/010 unit/diagnostic/test/provenance boundaries.
Objective: implement exactly PHASE_B_MODULE_CONTRACT.md after parent wave release. Read actual project/root AGENTS and source contract. Instruction role asserted, not mechanically enforced.
AcceptedBasis: decomposition revision 0.12/SCA-009 row22/DAG-010; parent accepted independent PKG05 design review in N_CROSS_CONTRACT_REFUTE/RETURN.md; parent phase release REQUIRED before dispatch.
ScopePath: {WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/children/A2_IMPLEMENT.
WorkingRoot: {REPO_ROOT}/projects/chirality-piping, resolved using git rev-parse --show-toplevel.
AllowedWriteTargets: {WORKING_ROOT}/core/product_physics/src/self_weight.rs; own ScopePath records only.
AllowedTools: local file reads/edits, shell targeted cargo test, read-only git status/diff, scope checker. Network/install/release/commit/push prohibited. No shared export edits; request owner integration.
DeclaredReads: PHASE_B_MODULE_CONTRACT.md, independent design review, project instructions/profile, DEL-05-01/02 scopes/status/context, product_physics/units/operation_applier source and fixtures.
EXCLUSIONS: any source outside exact module; UI, primitive engine taxonomy, solver formulas, schema, lib.rs, Cargo manifests, mass defaults, component/nozzle/support weights, lifecycle/receipt/authority.
AcceptanceCriteria: all module contract test rows verified; no duplication of mass formulas; original model untouched; deterministic complete draft output; reference/cache policy honored; all errors return no partial plan.
Checks: cargo test --manifest-path {WORKING_ROOT}/core/product_physics/Cargo.toml self_weight (after owner export). Profile broader tests run by parent to avoid overlapping suite ownership. Validate changed paths and freeze module SHA256 on return.
ExpectedReturn: files/hash, implemented boundary, exact tests/results, findings/limitations and integration obligations; no closure claim. Parent persists return/telemetry. Report model attribution as exposed, unavailable otherwise.
