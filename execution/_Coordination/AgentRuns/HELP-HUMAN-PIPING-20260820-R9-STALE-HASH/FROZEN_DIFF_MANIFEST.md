# Frozen integrated diff manifest — review candidate v1

- RunID: `HELP-HUMAN-PIPING-20260820-R9-STALE-HASH`
- Node: `N1`
- Diff basis: worktree candidate against `cd823be3badd034c86390f2707dcf01952c782f0`
- Frozen files: 19
- Freeze condition: `git diff --check` PASS; original corpus cases 01–03 byte-identical to HEAD.
- Reviewer scope: every file below, plus the actual unified diff and directly relevant callers/contracts. This manifest and reviewer-generated status/return files are runtime records created after the candidate hash freeze and are not implementation inputs.

| SHA-256 | Path |
|---|---|
| `2047f773ad7f35018cece58fa0f5986a3329dca37d01839aac28a77d38209d35` | `projects/chirality-piping/apps/desktop/src/services/operationContractCorpus.test.ts` |
| `1714a4350dfd3cf5649ea33414a44bc318618db60e91f3a96f423d0132178cb6` | `projects/chirality-piping/core/model_operations/operation_applier/src/lib.rs` |
| `87a55f4100875509e989d6eccb9844f9c0ed2b06c6fc974d175d39ba7989521b` | `projects/chirality-piping/core/model_operations/operation_applier/tests/contract_corpus.rs` |
| `2ad56ef8f9d34a3bc326cd255a5279e657e91bf617e1ee75faf56bd701d29d3f` | `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/MEMORY.md` |
| `a83e8268040409d6906ba03af7c4c6dc5ee78247715e94f36ba4a51b1c71e965` | `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/_STATUS.md` |
| `791dde72ae7c6c1198d17791ad64f7f9c5e6b9a6f3d9bd12de43b3cb34865668` | `projects/chirality-piping/execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-04_Analysis status semantics/_run_records/WORKING_ITEMS_RUN_2026-08-20_R9_STALE_HASH.md` |
| `74df0c2033d3dc896ff362638058deacde24bcf08db01e65c8f7e458095f04e7` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/ACTIVATION.md` |
| `fac5c59f7939d47a253334a0d96d06fb473e232702c281f4a81f3404eaac72a9` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/BRIEF_AMENDMENT_V2.md` |
| `062d00461bb1cf93b704a4054deb2ddaeaa0943db99d00fa5197907511303845` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/IMPLEMENTATION_BRIEF.md` |
| `54d8007a550d7a06f879f139987b4ddc7b28d85cc6572e5229dfd1c47f4c5318` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/REVIEW_BRIEF.md` |
| `d29c3773c2faafa2034ea552906da673222dfb63e67dcec5137bf2e99d70dd59` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/WORK_GRAPH_AMENDMENT_V2.json` |
| `4086cfb48fd6ff61c5fee11d635d34fc11dc741c01a6009d78ff4bd1935cd987` | `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260820-R9-STALE-HASH/WORK_GRAPH_V1.json` |
| `9f5767c35bea4c5295b93d966898060927566ea344553980149e072f81dadc81` | `projects/chirality-piping/fixtures/model_operations/contract_corpus/README.md` |
| `81d10c3905be847c430940454b297536b59e4c1d86f3a4efb772f0bdda5f8fb4` | `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_15_accept_claimed_hash_echo.json` |
| `dc2d83baca60e4553343758e433cdf381b7679644c848b22514cf8f1d9794be5` | `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_79_block_stale_claimed_model_hash.json` |
| `ba39752ee0220ecfe10ac490756160eb2be31d6842c2eb36d408f4259d4d5ead` | `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_80_block_malformed_claimed_model_hash.json` |
| `af11a92719f25743dd082371ce9f0e484a6e340ae93ac1bc9c4922ea101e28e2` | `projects/chirality-piping/fixtures/model_operations/contract_corpus/case_81_block_unsupported_claimed_model_hash.json` |
| `ec813d20529f9a6a7806e0b6c0e99bceb6c1a04ce569d4ed6530e2171168fe1b` | `projects/chirality-piping/schemas/operation_outcome.schema.json` |
| `48c513204ff2f626c5844d49b792636b00410362f38166f3b67de18fb405d43b` | `projects/chirality-piping/tests/test_operation_result_schemas.py` |

## Verification evidence supplied to reviewer

- Focused Rust hash gate: PASS (1).
- Full operation-applier crate: PASS (76 + 1 + 2).
- Focused operation-outcome schema pytest: PASS (5).
- Wasm build and focused adapter/direct-Wasm parity: PASS (171).
- Full desktop Vitest: PASS (539); production desktop build: PASS.
- Full piping pytest with pinned dev requirements: PASS (902).
- Practitioner harness self-check: PASS/exit 0; harness pytest: PASS (350).
- DEC-025 sweep: deliberately not yet run; independent review precedes it.
