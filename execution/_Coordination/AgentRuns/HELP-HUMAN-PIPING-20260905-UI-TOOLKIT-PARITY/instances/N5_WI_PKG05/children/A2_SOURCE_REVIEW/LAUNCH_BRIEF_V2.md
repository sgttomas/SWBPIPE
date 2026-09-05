# Sealed fresh source review brief
RequestedBy: WORKING_ITEMS N5_WI_PKG05; RunID HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY.
Role: TASK Agent2 (no delegation); TaskSkill: software-code-review.
INSTRUCTION_ROOT runtime binding: {REPO_ROOT}; CHIRALITY_INSTRUCTION_ROOT={REPO_ROOT} supplied explicitly by manager.
WorkingRoot: {REPO_ROOT}/projects/chirality-piping.
ScopePath: {WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N5_WI_PKG05/children/A2_SOURCE_REVIEW.
PackageID: PKG-05; DeliverableID DEL-05-01; SOW-013 OBJ-003; requirements unit/provenance/explicit-input/deterministic mechanics boundaries, AC-001 VER-001.
ImplementationBrief: ../A2_IMPLEMENT/LAUNCH_BRIEF.md plus manager PHASE_B_MODULE_CONTRACT.md and PHASE_C_READINESS_V2.md.
AcceptedBasis: parent source release SOURCE_RELEASE_V3.md, N2 B3 accepted, N3 SECTION_ACCEPTED_SNAPSHOT_V1, N6 B1 accepted; design review N_CROSS_CONTRACT_REFUTE/RETURN.md.
DiffBasis: manager SOURCE_FROZEN_DIFF.patch, SOURCE_FROZEN_SNAPSHOT.json; 100% new self_weight.rs and exactly one pub mod self_weight lib.rs addition versus LIB_PRE_EXPORT.rs. Other current parent-tranche changes are context only, not owned diff.
VerificationEvidence: manager PHYSICS_TEST_RESULT.txt and child return, to be provided at dispatch.
PROFILE_PATH: {WORKING_ROOT}/software-workflow.json.
ApplyEdits: false. AllowedWriteTargets: []. No test execution, install, release, writes or delegation. Parent persists runtime launch/status/return; read-only TASK exception applies.
AllowedTools: read files and read-only shell/git; deterministic skill tools scope validator/select checks as permitted by skill. First validate source paths against exact module and lib.rs fence. Read required skill companions. Review all changed lines and test adequacy, stale source hash semantics, atomic integration boundaries, referential projection, ID collisions, all units/finite checks, evidence retention and no hidden defaults. Return only actionable findings with precise anchors, PASS or REVISE verdict, limitations and scope audit; no implementation/lifecycle acceptance. Source changes required for findings route through manager, then fresh rereview.

## V2 combined frozen review amendment
Parent ordered one fresh combined review after adapter V2 design independent PASS (N_SELFWEIGHT_ADAPTER_REFUTE) and accepted source release. Review 100% module self_weight.rs + single lib.rs export + all new core/loads/self_weight_wasm/{Cargo.toml,Cargo.lock,src/lib.rs}. Source originals preexport LIB_PRE_EXPORT.rs and empty new files. Frozen source artifact SOURCE_COMBINED_DIFF.patch / SOURCE_COMBINED_SNAPSHOT.json supplied at dispatch. Child adapter brief ../A2_IMPLEMENT/ADAPTER_LAUNCH_BRIEF.md and ADAPTER_DESIGN_V2.md are controlling extension. Generator source hash format sha256:<64 lowerhex>, strict request/gravity unknown-field rejection. Wrapper must authenticate full original Value canonical hash before typed lossy view, and native/Wasm share generate_plan semantics, errors yield no plan/partial model/receipt. Validate dependency paths/version parity and wasm exported feature ABI. Physics tests V2 118 PASS; adapter cargo unit+Wasm build evidence supplied by author; parent consumer checks remain deferred. Reviewer no source/test writes, parent records return.
