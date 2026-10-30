---
doc_id: COORDINATION-RESPONSE-2026-08-02-PIPING-RUNTIME-SURFACE-NEEDS
doc_kind: coordination.response
status: inventory_response_not_authority
created: 2026-08-02
requested_by: Root TASK_MANAGEMENT
prepared_by: PROJECT_SETUP under HELP_HUMAN
repository_basis: 97678a841ef58345c73d3470ed8de57c9b1405d2
---

# Piping coordination response — runtime-surface needs

## 1. Purpose and reciprocal routing

This is the bounded, filesystem-grounded Piping response to
`projects/chirality-piping/execution/_Coordination/NOTICE_2026-08-02_ROOT_TM_RUNTIME_NEEDS_RESPONSE_REQUEST.md`
(SHA-256
`32f943eefe80d926626c5f63ae574d6df84f461cd23f0728edf6b8a13de769f1`).
It inventories current Piping surfaces and records candidate generic primitive
classes for future consideration. A candidate named here is neither a finding
of record nor an adopted requirement.

The response reciprocally cites Root rows `TM-ROOT-105` and `TM-ROOT-109` in
`execution/_Coordination/_TaskManagement/REGISTER.csv` (SHA-256
`5d8c7b3833820f24b104776f78f3637ea9fad8bacf27fb98bddfb6053f89712d`).
The presence of this response satisfies the event named in each row's recorded
trigger; it does not itself open, close, resolve, or disposition either row.
Root opening or closing either cited row requires a separate Root owner ruling.

The two product-direction records used only as context are the routed Piping
notice `projects/chirality-piping/execution/_Coordination/NOTICE_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md`
(SHA-256
`0386b64a87b49e77163bbf4b7ff467427255e5a6afe73a66bc96649637b6a73e`)
and its Root source
`execution/_Coordination/OWNER_INTENT_2026-08-02_PRODUCT_DELIVERY_DIRECTION.md`
(SHA-256
`9bbb67556765c6c83d6a35a1ace297e4d693d5169281c620dc9b2673229c7e03`).
The existing Piping boundary-intent record is
`projects/chirality-piping/execution/_Coordination/OWNER_INTENT_2026-07-31_DESIGN_TOOL_BOUNDARY.md`
(SHA-256
`c35f119adc4199751d5db3af62c01c1b2ba84be42b31424b3d1a2e411dc57eee`).
All three are intent or coordination records, not scope, and this response does
not expand them.

## 2. Survey basis and evidence ledger

The survey was performed on repository commit
`97678a841ef58345c73d3470ed8de57c9b1405d2`. DAG-009 is the active dependency
authority; its approval record is E-02 below. Receipt 86 records that
selectability must be re-derived after DAG-009 activation, so the dated
2026-07-29 slate's readiness list was not used. The ruled coordination record
keeps the current target stage at R5 (E-29); this survey does not alter it.

| ID | EvidenceRef | SHA-256 | Use in this response |
|---|---|---|---|
| E-01 | `projects/chirality-piping/loop/LOOP_RECEIPTS.md` | `7350840f3c8f3ddd868ac399e9bf1f16787922be474271b3144d54ba1abbbba3` | Receipt 86 and current-loop caution |
| E-02 | `projects/chirality-piping/execution/_DAG/DAG-009/APPROVAL_RECORD.md` | `f25526c4e0eec239f5d3464ca4d8e0ab8c9638ebd035bc9aa282def33989337b` | active dependency authority and re-derivation requirement |
| E-03 | `projects/chirality-piping/apps/desktop/src/App.tsx` | `9db3ac5dc41861c0fc29cbcff245d5e6946adae0ca9e96318f77c5fd6667d5a6` | implemented desktop workspace and UI routes |
| E-04 | `projects/chirality-piping/apps/desktop/src-tauri/src/lib.rs` | `d22904928064e011092f6e428c05f124896551c2a5a0869b72e8a553a3a359f7` | implemented native commands, persistence, solve jobs, and operation seam |
| E-05 | `projects/chirality-piping/apps/desktop/src/services/operationService.ts` | `8f213ac1f3a0afb1f98072b7fa476e6d89ce13bde9b44e6fa56f60a13b5670e2` | Tauri/WASM structured-operation routing and route disclosure |
| E-06 | `projects/chirality-piping/apps/desktop/src/services/previewService.ts` | `ab4cb9482a56436085ccb79b40f269768d12bb5beca82dc45c4a22bb4fd86a6e` | implemented solve start/poll/cancel surface and browser-fixture fallback |
| E-07 | `projects/chirality-piping/apps/desktop/src/features/agent-proposals/AgentProposalPanel.tsx` | `4701d2abbb40207fbe94db3ead6df68c748f75467d64b58e05aecef49d60230f` | current deterministic review-only proposal UI |
| E-08 | `projects/chirality-piping/apps/desktop/src/features/operations/OperationApplyPanel.tsx` | `6b31ffda955ad1b407b17e53d2a37cd2bd3e03f83ce7418110bc2f65bab8400c` | explicit validate/apply, user acceptance, and undo/redo UI |
| E-09 | `projects/chirality-piping/apps/desktop/src/features/operations/OperationLedgerPanel.tsx` | `85ad1673e18af061ff2058b86e7b74b1130b41dccb43be9b941351efa13a6e68` | local operation-review ledger and recorded persistence TBDs |
| E-10 | `projects/chirality-piping/schemas/model_operation.schema.json` | `a7e727a8bf16a03c74c47f39c4d0660a8dad071aa7d394926f905627f63cbbaf` | current Piping model-operation vocabulary |
| E-11 | `projects/chirality-piping/schemas/operation_outcome.schema.json` | `d526fcf10f9efb19ddb2467bdcebd995b9f3885f6238c54ce1b039cf7e0749bf` | current validate/apply result and audit boundary |
| E-12 | `projects/chirality-piping/core/runner/headless/src/bin/openpipestress-runner.rs` | `589b9c4bec1bd55a7dadd93cb4f4c820e4643662988e8af0b77d70dbcb85ad54` | implemented local structured-I/O CLI and explicit process policy |
| E-13 | `projects/chirality-piping/schemas/headless_runner.schema.yaml` | `deaa474eebb07752c0e09878809fbd9a54e9d25052f9a6f0b8bb5f720ce7a182` | current runner request/result/job contract |
| E-14 | `projects/chirality-piping/api/api_boundary_contract.yaml` | `17c78b2c08eb27be7bdd21a526dc09130f09cef43d657aafd4d2848c1d98de28` | draft public API boundary; transport and runtime TBDs |
| E-15 | `projects/chirality-piping/_harness/adapter.yaml` | `2de9d3ab3b3e3b2eb622c0959dc292eff06a11c3da7c2665798978df08c20554` | development practitioner-harness configuration only |
| E-16 | `projects/chirality-piping/schemas/plugin_manifest.schema.yaml` | `2fb470eeeae8b0b26182264c6e33dfd0f89d9d321eaa2ba6b0411161264d759b` | current deny-by-default plugin declaration contract |
| E-17 | `projects/chirality-piping/apps/desktop/src/features/redaction-controls/redactionExportControls.ts` | `573c58f2752c7897adbd0af85ec1759bef3f7f067dfb7adbcafff5b746deff73` | implemented Piping redaction/export-control semantics |
| E-18 | `projects/chirality-piping/schemas/comparison_tolerance.schema.json` | `2c2cf4089849bb42a2d8eb83d559f2a60212ca525d2fa1e0bbb910609c10525a` | current Piping tolerance-profile vocabulary |
| E-19 | `projects/chirality-piping/schemas/comparison_mapping.schema.json` | `3fdbb88e328b3bb1bce046bd0fa9342fb0e4efd9bb30d4d33ed4579cb249d759` | current Piping participant, mapping, and comparison-review vocabulary |
| E-20 | `projects/chirality-piping/core/comparison/model_state/engine.py` | `64220403f6b0cfc81e7ecbdbad9a82ec73527644a30aa722592614b08bf067a2` | implemented stable-ID/explicit-mapping model comparison |
| E-21 | `projects/chirality-piping/core/comparison/analysis_run/engine.py` | `1cb115ab65b160e19db5cab7341f94ceb09de1361d59d21ff9f41fd234432900` | implemented unit-aware analysis-run comparison |
| E-22 | `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-02_Import-export adapter framework/_STATUS.md` | `a965f1f05720bc58be0eacb450f13b0715599f9d7df2d41feab9f37519691551` | recorded remaining plugin runtime and capability-grant work |
| E-23 | `projects/chirality-piping/execution/PKG-12_Security, Privacy, and Private Data Handling/1_Working/DEL-12-03_Telemetry off-by-default design/_STATUS.md` | `1daf921161b2ecc138068a6dc765bd17c758ec3b5bb76bc8567c40fa5febc865` | recorded remaining runtime-wide guard binding |
| E-24 | `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-04_Analysis-run comparison engine/_STATUS.md` | `477fbb3cb09493cc67bc046f40948efbd0a408e7294ba4ad1330875ae0a18392` | recorded comparison coverage and validation gaps |
| E-25 | `projects/chirality-piping/execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-05_Comparison mapping, tolerance, and export contracts/_STATUS.md` | `bf00441dab6e351c1ab9a48dcc7bc3bb91fe105df1ff248b0edd366fb84bbb49` | recorded tolerance/report binding gaps |
| E-26 | `projects/chirality-piping/execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-04_Agent rationale and professional-boundary controls/_STATUS.md` | `39af5d4b4ebfcf0c0c46b122f29d33f33b0774094be317201ac8a8269572b366` | recorded remaining downstream agent-rationale binding |
| E-27 | `projects/chirality-piping/execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-01_Public API and plugin boundary/_STATUS.md` | `9788584c52607af63de2f9920d2eb431a24e66a9be884be6fa6e21c4bf692f25` | current public-API lifecycle and remaining registration work |
| E-28 | `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_STATUS.md` | `755e944ec0fa816ab999134276da23bd7ab695eb39610f7a7c5843db3b2537e5` | current solve-UX lifecycle and diagnostic-carriage remainder |
| E-29 | `projects/chirality-piping/execution/_Coordination/_COORDINATION.md` | `54e071eecfea36658dc8ba99f1ecd81d2f464d2c32be8431339cd2118d10ab5c` | ruled R5 target-stage record |

## 3. Current surface inventory

| Surface | Current filesystem-grounded state | Classification | Evidence |
|---|---|---|---|
| Desktop product | React/Tauri desktop workspace with model tree, viewport, property inspection, operations, loads, libraries, rule packs, solve, results/comparison, reports, project storage, exports, and evidence/boundary panels. Native commands provide local persistence, solve jobs, structured operations, report-package save, and supporting queries. | Existing implemented surface | E-03, E-04 |
| User-facing operation path | GUI edit intents route through validate/apply, produce deterministic diffs and structured outcomes, require a user acceptance signal, replace rather than mutate the input document, and support session undo/redo. | Existing implemented surface | E-05, E-08, E-10, E-11 |
| Agent-proposal path | The UI can load deterministic local review proposals and display validation, audit, and professional-boundary fields. Proposal acceptance is disabled in that panel; the review ledger holds proposals and GUI intents for user review. | Existing limited, review-only surface | E-07, E-09 |
| Agent-callable structured execution | `openpipestress-runner` is a bounded local foreground CLI with JSON stdin/file input, JSON stdout, optional explicit output, no network/daemon/telemetry/hidden write surface, blocking diagnostics, and operations for solve, validate-input, export-results, benchmark, and regression. | Existing implemented operation surface; not an agent harness | E-12, E-13 |
| Public API/plugin boundary | The checked-in boundary names commands, queries, jobs, result envelopes, permissions, job cancellation, and no-bypass concepts. Its own metadata marks the public transport, endpoint syntax, plugin runtime, permission persistence, and stability level `TBD`; the owning deliverable remains `IN_PROGRESS`. | Current draft specification/contract, not an implemented public transport | E-14, E-27 |
| Plugin/adapter capability boundary | The manifest schema is deny-by-default and declares permissions, sandbox, provenance, privacy, checksums, and no-bypass constraints. Runtime execution model and bounded capability grants remain recorded work. | Current declaration contract plus recorded remaining work | E-16, E-22 |
| Tailored agent harness/runtime | `_harness/adapter.yaml` configures the repository's practitioner/development harness and points it at loop control surfaces. It is not an OpenPipeStress operative agent runtime. No current application-owned agent orchestrator, run sandbox, native-tool broker, or durable agent-resume surface was found; the proposal panel and CLI do not supply those functions. | Explicitly absent/future application surface | E-07, E-12, E-14, E-15, E-22, E-26 |
| Solve job control | The desktop exposes backend start/poll/cancel with job IDs, cancellation tokens, states, receipts, and diagnostics. The browser/test preview route may use fixtures, and the helper can fall back to a fixture when native invocation fails. | Existing product-specific job surface; not sufficient as an authoritative agent-runtime fallback rule | E-04, E-06, E-28 |
| Audit, privacy, and boundary controls | Structured outcomes carry validation, model-basis, acceptance, audit, and professional-boundary fields; export routes carry explicit redaction decisions/findings and can withhold materialization. Runtime-wide telemetry/guard interception remains recorded work. | Existing bounded controls plus recorded remaining work | E-09, E-11, E-17, E-23 |
| Comparison | Model-state comparison uses stable IDs or explicit mappings. Analysis-run comparison uses explicit mapping status, result family, dimensions, units, conversions, and caller-supplied tolerance profiles; incompatibility can block a delta. The schemas preserve hashes, provenance, review, and professional-boundary fields. Coverage, output-schema, tolerance-suitability, and report binding remain open. | Existing implemented engines and current Piping contracts plus recorded remaining work | E-18, E-19, E-20, E-21, E-24, E-25 |

## 4. Piping-local definitions and candidate generic primitives

“Current local” below means already present in the cited Piping filesystem. It
does not mean this response newly accepts or promotes anything. “Candidate
generic” names a possible reusable primitive class only; it is not a selected
Root contract design.

| Material need | Owning Piping surface and current local definition | Candidate reusable generic primitive | Evidence |
|---|---|---|---|
| Run and subject identity | Piping owns project/model/state/run/operation IDs, unit-system and load-basis references, canonical payload hashes, schema versions, and analysis statuses. | A versioned run-identity envelope able to bind caller, subject hashes, operation/tool versions, policy/sandbox identity, parent run, and correlation ID without interpreting Piping domain fields. | E-10, E-11, E-13, E-18, E-19 |
| Query surface | Piping owns the objects and fields for model design, components, analysis runs, results, diagnostics, reports, and comparison records. | A typed query invocation/result envelope, pagination or continuation token, structured diagnostic channel, and explicit unsupported-field result. | E-03, E-13, E-14, E-19 |
| Mutation authorization | Piping owns operation kinds, preconditions, validation, diff semantics, units, model basis, stale-state checks, and the rule that model changes remain structured and user-gated. | A deny-by-default capability grant and per-operation authorization receipt that can require a human gate but does not decide Piping mutation meaning. | E-08, E-10, E-11, E-16 |
| Native-tool authorization | Piping owns which solver, comparison, export, report, rule, and persistence operations are meaningful and which product boundary each crosses. | A tool registry plus exact tool/version/capability binding, scoped filesystem/network/process rights, request-time grant evaluation, and denial receipt. | E-12, E-14, E-16, E-22 |
| Audit | Piping owns model/operation/result evidence, diagnostics, provenance, redaction, acceptance records, and professional-boundary fields. | A durable event envelope for authorization, start, tool call, progress, denial, cancellation, result, and failure, with canonical hashes and explicit partial/truncated status. | E-09, E-11, E-13, E-17 |
| Interruption | Piping owns solve-job states and the product meaning of a cancelled or incomplete solve. | A cancellation-token and interruption protocol with requested/acknowledged/terminal states, timeout behavior, cleanup evidence, and a rule that partial outputs are not silently final. | E-04, E-06, E-13 |
| Recovery | Piping currently has session undo/redo, local project persistence, model hashes, and migration/refusal checks; the review ledger still records durable operation-receipt persistence as unresolved. | A checkpoint/resume identity tuple and stale-resume refusal primitive binding subject hash, schema/app/tool versions, policy/sandbox identity, completed event cursor, and output inventory. | E-04, E-08, E-09, E-11 |
| Result budget | Piping has structured result envelopes and an explicit-output CLI, but the surveyed surfaces define no application-agent result-byte/item/event budget or continuation behavior. | A caller-visible budget object for bytes, records, events, wall time, and tool calls, plus deterministic truncation, continuation, and budget-exhausted outcomes. | E-12, E-13, E-14 |
| Fail-closed behavior | Piping owns blocking diagnostics for invalid/stale operations, missing units/dimensions, unsafe export, private/protected content, unsupported operation categories, and professional-boundary violations. Browser/test fixture fallback is a preview behavior and must not be mistaken for authoritative agent execution. | A generic deny/error state machine that distinguishes unavailable, denied, blocked, cancelled, partial, failed, and succeeded, and never converts runtime failure into fixture/synthetic success. | E-05, E-06, E-11, E-12, E-17, E-21 |
| Privacy and professional boundary | Piping owns privacy classifications, redaction actions, protected-content rules, local/private intent, engineering-review notices, and which outputs remain decision support. | A generic carrier for consumer-defined policy labels and evidence, without Root defining Piping's classifications or judgment boundary. | E-11, E-13, E-16, E-17 |
| UI/API semantic equivalence | Piping owns what each operation/query/job means and the same units, validation, diagnostics, diff, boundary, and acceptance semantics across human and agent routes. | A transport-neutral conformance fixture format that can run the same consumer-owned cases through UI service seams and agent/API seams and compare structured outcomes. | E-03, E-05, E-08, E-10, E-11, E-14 |

## 5. UI/API semantic-equivalence obligations

These are Piping-local equivalence obligations derived from current surfaces,
not a new product-direction decision.

| Obligation | Piping-local equivalence condition | Current gap or caution | Evidence |
|---|---|---|---|
| Read/query equivalence | Human and agent routes identify the same model/state/run/result objects, use the same stable references and hashes, and return the same diagnostic and limitation meanings. | The public transport and endpoint binding are TBD. | E-13, E-14, E-19 |
| Edit equivalence | Both routes submit the same structured operation, validate against the same model basis, show the same diff and diagnostics, and require the same explicit acceptance before a new model document is retained. | The current agent proposal panel is review-only and does not accept; a separate future agent route must not bypass the operation seam. | E-05, E-07, E-08, E-10, E-11 |
| Job equivalence | Start, progress, cancellation, failure, and result retrieval preserve the same job identity and terminal-state semantics. | Product solve cancellation exists; a general application-agent interruption/recovery protocol does not. | E-04, E-06, E-13 |
| Security equivalence | The API route cannot bypass UI-visible permission, privacy, redaction, protected-content, sandbox, provenance, checksum, or persistence boundaries. | Plugin runtime/grant selection and runtime-wide guard binding remain open. | E-16, E-17, E-22, E-23 |
| Comparison equivalence | UI and agent routes use the same participant identities, explicit mappings, unit/dimension normalization, tolerance-profile references, diagnostics, and hashes. | Default numerical tolerances are deliberately not defined by the schema; coverage, suitability, output-schema, and report bindings remain open. | E-18, E-19, E-21, E-24, E-25 |
| Claims equivalence | Neither route upgrades mechanics, comparison, rule-check, handoff, or proposal output into a human or professional acceptance state. | Human acceptance remains a separate record and gate. | E-07, E-11, E-13, E-19 |

## 6. Comparison-basis identity boundary

| Must remain Piping-local | Requires identity across consumers if a future generic runtime carries compatibility claims |
|---|---|
| Result-family meaning; entity/component/model semantics; analysis-status vocabulary; unit and dimension taxonomy; mapping admissibility; normalization and tolerance rules; solver and rule-pack meaning; local privacy/protected-content semantics; professional-boundary and human-review rules. | Basis ID and version; canonical basis hash; compared subject IDs/hashes/versions; consumer and operation/tool versions; sandbox/policy identity; unit-system and tolerance-profile references as opaque governed IDs; evidence and provenance references; comparison mode; result status; diagnostics; claimant/caller identity; timestamp; and explicit missing/incompatible/budget-exhausted outcomes. |

The left column is defined by E-10, E-11, E-17, E-18, E-19, E-20, and E-21.
The cross-consumer identity need in the right column is a coordination candidate
arising from the versioned/hash-bound fields already carried by E-11, E-13,
E-18, and E-19. It does not make Piping tolerances generic, define a Root
equivalence rule, or establish that two consumers match, conform, or are
compatible.

## 7. Recorded gaps and unknowns

1. No application-owned operative agent harness/runtime, native-tool broker,
   durable agent checkpoint/resume service, or application-agent result-budget
   contract is implemented in the surveyed Piping surface (E-07, E-12, E-14,
   E-15, E-22, E-26).
2. Public API transport, endpoint syntax, plugin runtime, loading/signing/
   isolation, permission persistence, stability level, and code generation are
   still `TBD` in the current API boundary (E-14, E-27).
3. Adapter/plugin runtime dispatch and bounded capability grants remain
   unselected; the current manifest boundary proves declaration/no-bypass
   structure, not runtime execution (E-16, E-22).
4. Runtime-wide telemetry/guard interception is incomplete across plugin,
   adapter, import/export, report, and private-library runtime paths (E-23).
5. Durable operation-audit persistence and final actor identity remain recorded
   TBDs in the current review ledger (E-09).
6. Comparison category coverage, output-schema conformance, tolerance
   suitability/independent validation, and report/export binding remain open
   (E-24, E-25).
7. The current browser/test preview helper's fixture fallback is not an
   acceptable evidence basis for an authoritative agent-run success and would
   need an explicit fail-closed distinction in any future agent route (E-06).

## 8. Effect boundary and return

This response is inventory and coordination only. It creates no adoption,
product-basis, scope, activation, implementation, lifecycle, status, memory,
DAG, release, publication, professional-reliance, generic Root contract, or
Root-row closure effect. Those effects remain with their owning instruments
and human gates. In particular, this response does not select a generic Root
contract design and does not authorize Piping product work.

The return contract is satisfied as follows: the inbound notice and hash are
cited in section 1; concrete needs and owning Piping surfaces are named in
sections 3–5; current local definitions and candidate generic primitives are
separated in section 4; every material need cites SHA-bound evidence through
section 2; `TM-ROOT-105` and `TM-ROOT-109` are reciprocally cited in section 1;
and the owning-instrument/human-gate boundary is stated above. Root may consume
this response as trigger evidence, but Root opening or closing either row still
requires a separate Root owner ruling.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
