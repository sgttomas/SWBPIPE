# Palette organization contract — Phase A draft

Deliverable: DEL-07-09; package: PKG-07; scope: SOW-077; objectives: OBJ-006 / OBJ-015.
Run: HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY; producer: A2_MAP under N1_WI_PKG07.

This is a derivative implementation contract for manager validation, not new decomposition truth or lifecycle acceptance. Accepted upstream basis is SOFTWARE_DECOMP revision 0.12, DEC-094 and the SCA-009 accepted snapshot (`execution/_ScopeChange/SCA-009_2026-08-20_0000/ACCEPTANCE_RECORD.md` plus `Vocabulary_Annex.md`), with approved dependency snapshot `execution/_DAG/DAG-010/`. The annex retains its historical candidate header; its accepted consumption is established by DEL-07-09 `_CONTEXT.md`, revision 0.12 and DEC-094, not by editing that immutable header. Source comparison baseline is commit `740569598f9d00440636b8ea25264127f418e4ec`.

The companion `Capability_Comparison.csv` contains all 24 normative-now rows and R1–R3. Its runtime/UI claims describe that baseline, not concurrent work in this run. `Vocabulary_Coverage.csv` remains untouched: its historical closures of rows 14–16 are not reopened by this comparison. Current exposure improvements, especially family choice, are separately identified residuals. No runtime capability is inferred solely from a JSON Schema token or a solver input structure.

## Scope and invariants

DEL-07-09 owns this organization and routing contract only. Implementation lands in the annex's named deliverables. This does not create a dedicated support panel deliverable or extend a package's write authority. Requirements are grounded in DEL-07-01 `ScopeOfWork.md` REQ-01/04/05/06/07, DEL-07-02 RQ-001 through RQ-008, DEL-07-03 R-002 through R-010, and the PKG-16 deliverable ScopeOfWork acceptance criteria.

OPS-K-DATA-1/2/3, UNIT-1, IP-1/2/3, PRIV-1, AUTH-1/2 and AGENT-1/2/3/4 apply. Inputs are user-authored or lawfully imported with provenance. Missing engineering values remain missing, accompanied by actionable findings. An operation being applied to the session is not professional acceptance or a release declaration.

## One discoverable toolkit

Use one command catalog to organize entry into the existing viewport, inspector, load editor and library surfaces. Context shortcuts may call the same catalog command; they must not own independent operation builders or validation rules. A compact palette can remain beside the viewport. Its categories are:

| Group | User-facing commands | Annex rows | Destination |
|---|---|---|---|
| Build | Node, Pipe, Component, Expansion joint, Split pipe | 1, 14, 15, 19 | Viewport placement or explicit creation form; expansion-joint form in inspector |
| Supports | Restraint, Spring hanger, Nonlinear support, Equipment boundary | 2, 12, 13, 16, 21 | Inspector; explicit family and behavior choice within owning slice |
| Properties | Materials, Sections, Assign section, Imported libraries, Hanger library | 3, 17, 23 | Inspector/editor/library surface |
| Loads | Cases, Force/moment/displacement, Wind, Seismic, Combinations, Generate self-weight | 4–8, 22 | Load-case editor and selected-entity inspector |
| Edit | Remove, Copy, Rotate, Mirror, Undo, Redo | 10, 18, 20; existing deletion in 1/2/4/5/8 | Context-aware form or reviewed operation; session history |
| Select and View | Select entity, Search, Display units | 9, 24 | Tree/viewport selection and presentation preference |
| Review | Pending changes, Change details, Agent proposals | 11 and all mutations | Shared review surface; agent route retains D-58 gate |

The palette need not display every command as a permanently visible button. Search and contextual availability must make the full accepted vocabulary findable. Disabled commands retain a visible short reason and their owner/dependency in detailed help. Roadmap R1–R3 belong in an explicit roadmap/help view, not an enabled model-building menu.

Labels above are interaction labels, not operation tokens. Default human flows must use ordinary engineering language and field names. Operation IDs, hashes, change kinds and backend routing belong in optional technical/audit details, not form labels or success messages. The semantic command IDs remain stable for tests and future programmatic discovery even if user-facing labels change.

## Availability and interaction states

Availability is command-specific: ready, needs selection, needs input, waiting for capability, or held by a project gate. Ready means the form can start, not that its eventual change will apply. An enabled command must have a functioning form or interaction and a resolver-backed route. A decorative button, unsupported field editor, schema enum, or read-only example is not coverage.

Selecting a command preserves typed entity selection and the corresponding operation target `{object_type, ref}` and focuses the relevant form. Highlight the selected object in tree and viewport where representable. IDs, not labels, array indices, or visible row positions bind targets. Draft IDs are explicit and collision checked against current model and queued creations; previews bind to the actual model basis, not whatever object is selected later.

Pointer placement produces a visible coordinate draft with units. It does not immediately alter the model. Routing continuation states carry the explicit start endpoint and show the next endpoint before queueing. Switching or cancelling a tool clears only transient draft/armed-tool state; it must not delete model content. Keyboard entry must provide an equivalent path for coordinates, refs, quantities and submission. Focus returns predictably after cancel, queue, apply or validation failure.

The generic Remove action resolves the selected entity's supported deletion capability. It previews the exact target and any blocking references. It must never infer cascade deletion, reassignment, detachments, or engineering replacement values. Unsupported entity deletion remains unavailable with a reason.

## Draft, validate, review and apply

1. The user fills an explicit draft. Quantity fields display value, entered unit, dimension-appropriate choices and source/provenance where required. Optional means absent is allowed by the owning model; it does not mean zero. Do not inject engineering defaults merely to make a button active.
2. Queueing produces the shared PKG-16 structured intent and shows a plain-language summary. Queueing is not application. Field-level prechecks assist entry; the Rust operation engine remains authoritative for references, units, constraints and deterministic diff.
3. Validation is against the current model/hash and the exact proposed operation. Display before/after values and added/removed entities, affected refs, warnings and remediation. Keep solve-blocking and rule-check-blocking classes distinct. Failure leaves the model unchanged and returns focus to the relevant field or selected object.
4. Apply is an explicit user act on a currently valid proposal. Revalidate stale basis/preconditions; do not silently retarget or replace `before` values. Only the returned successful applied model may update the session. Preserve returned receipt/audit data and invalidate previous solve/results as required by the existing application state architecture.
5. Multi-step topology or generation commands require an owner-approved batch/transaction contract. A UI must not advertise atomic completion while independently applying part of a sequence. The PKG-05 proposed ordered self-weight operation list therefore needs a reviewed failure/rollback presentation before GUI application.
6. Undo/redo restores session checkpoints under DEL-00-05 architecture and invalidates stale previews/bases. Show it as session history with its implemented limit, not durable accepted-state reversal. New successful edits clear redo.

Selection, camera, palette focus, search and display-system switching are transient/query or presentation actions. They do not create physical model operations. The single PKG-16 route applies to every new physical-model mutation; it does not require inventing mutation tokens for non-mutating view actions or reimplementing the accepted session checkpoint architecture.

## Quantities and provenance

Project display-system preference must not rewrite stored quantities, original entered units, source records, canonical calculations or model hashes. Converted display values are labelled as such. Editing starts from the entered value/unit pair (or an explicit user-selected conversion whose subsequent edit is validated), never from silently rounded display text. Unknown conversion or dimension is shown honestly; no numerical substitute is generated. All relevant renderers must consume the common preference before claiming project-wide coverage.

Material temperature points use stable IDs and explicit temperature/E/G/alpha quantities. Hanger and nonlinear forms mirror the backend's discriminated field and constraint contracts. Imported hanger selection is a human choice of a user-imported record with provenance and source identity; it is not catalog sizing. No protected tables or manufacturer defaults are bundled. Provenance/private or redistribution metadata remains visible where the existing library/editor contracts require it.

## Human and agent semantic parity

The common capability description must distinguish a command's human interaction from its structured meaning: target types, input schema, preconditions, dimensions, provenance, returned diagnostics/diff and application boundaries. For the same model basis and explicit parameters, human and future agent-originated operations must validate and preview through `operationService.ts` and the same `operation_applier` engine; a TypeScript substitute is prohibited.

D-58 remains held. The current deterministic proposal panel is review-only, and `EditorOperationIntent.author_type` is `user` at the baseline. This contract does not connect a provider, permit agent apply, adopt a successor runtime mechanism, change Piping client membership, or waive human acceptance. Capability contract production and implementation of supported human operations can continue within this boundary. Gate resolution remains routed to the parent authority.

## Acceptance evidence and handoff

Required implementation evidence is command-to-form navigation, accessible labels/keyboard paths, correct stable targets, explicit unit/provenance entry, resolver-produced valid diff/apply, and unchanged model on invalid/stale input. Quantity switching additionally requires byte/hash-equivalent model evidence; destructive and topology commands require reference and partial-failure tests. Fresh independent code review and the project's applicable deterministic checks remain separate publication requirements.

This phase delivers a draft contract and baseline comparison, not closed toolkit coverage. Parent N1_WI_PKG07 validates and freezes the accepted Phase A inputs before dependent implementation consumes them. Required derivative refresh is the final comparison against the integrated frozen diff and actual test evidence. Historical coverage remains preserved; lifecycle/register/pointer changes are outside this child's scope. Outstanding dependencies and proposed disjoint implementation nodes are in `Palette_Operation_Routing.md`.
