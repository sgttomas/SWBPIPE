# Plan Completion Log

Archive ledger for landed items from the active completion plan (currently
[PLAN_2026-06-10_prd_completion.md](PLAN_2026-06-10_prd_completion.md), plus
the accepted hardening-lane plan
[PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md)). When
a plan item lands, its row in the plan is compressed to one line and the full
narrative moves here, newest entry first. This file is history, not
authority: lifecycle state lives in deliverable `_STATUS.md` files, evidence
lives in `_run_records/**`, and rulings live in the decision register and
`SOFTWARE_DECOMP.md` decision log. Nothing here is a release, professional,
certification, or code-compliance claim.

---

## 2026-06-13 - C2 editor GUI slice 4 landed: required-input / value-slot declaration form builders (`TP-C2-DECLEDITOR-001`)

Adds a structured editor for the rule pack's variable declarations — its
`required_inputs` and user-supplied `value_slots`, the named variables a
formula's `variable_ref` binds to. New
`apps/desktop/src/features/rule-packs/DeclarationsEditor.tsx` adds / removes /
edits each entry through form controls: text `input_id`/`name`/`unit_ref`,
closed-set selects for `source_kind` / `slot_kind` / `value_status` /
`required_for` / `completeness_status`, and the shared `DimensionSelect` for
the `quantity_intent` dimension. **Add** seeds a schema-valid default
(`RequiredInput` / `UserSuppliedValueSlot` shapes verified by `toEqual`) with a
fresh unique id; **Remove** is blocked at the `minItems:1` schema floor for
both arrays. The editor mounts ahead of the expression composer in
`RulePackManagerPanel`; both read the same draft document, so a variable added
through the editor is immediately bindable in the composer (no raw JSON). The
enum vocabularies are exported consts copied verbatim from the schema and an
out-of-vocabulary stored token surfaces as a "(current) X" option, never
silently snapped (CONTRACT no-silent-defaults). Lossless: a patched entry keeps
every member it carried (provenance, the const-true relaxation flags,
missing-value diagnostic), a nested `quantity_intent` edit preserves
`unit_required`/`dimension_check_required`, and untouched siblings round-trip
verbatim. `draftPlaceholderProvenance()` was extracted from
`rulePackService.ts` (byte-identical) so the template and the declaration
defaults share one private-by-default provenance source. No numeric allowable
value is invented — `UserSuppliedValueSlot` carries a dimension/unit intent and
a `value_status` (`not_provided` default), not a number.

Authority-vs-plan correction: the slice-2/3 residual wording "required-input /
value-slot / **load-combination** form builders" was a misnomer — the
authoritative `schemas/rule_pack.schema.yaml` top-level object is
`additionalProperties:false` and has **no `load_combinations` member** (load
combinations are a model concept, Phase A4). The rule pack's third
user-authored document-structure member is `check_definitions`. Per the
`_COORDINATION.md` state-tracking rule, the plan C2 row and this log are
corrected to name `check_definitions` as the remaining C2 form-builder
sub-surface; the schema (authority) is unchanged.

D-02b gate held: purely structured form controls — the editor authors variable
*declarations*, with no writable expression text syntax and no text rendering
of any AST; the typed AST stays the sole edited, checksum-bound expression form
(DEC-022). Private rule packs stay local-only; no protected tables/constants.

Validation: rule-packs Vitest **43/43** (was 26), full desktop Vitest
**285/285**, `tsc -b` clean, Playwright `-g "rule-pack manager"` **2/2** (both
viewports, declarations editor driven from blank). An independent four-lens
adversarial review (schema-conformance, governance/D-02b/IP, React correctness,
test honesty) returned a SHIP verdict; its one should-fix was folded in before
commit — the shared `DimensionSelect` silently display-snapped an
out-of-vocabulary stored dimension to `"TBD"` instead of surfacing it, so it was
given the same "(current)" escape hatch as the new `EnumSelect` (also tightening
the composer's literal/table dimension fields), with a guarding Vitest case
added. Run record:
`execution/PKG-07_.../DEL-07-03_.../_run_records/WORKING_ITEMS_RUN_2026-06-13_TP-C2-DECLEDITOR-001.md`;
SMOKE TP-MAC-151. DEL-07-03 remains CHECKING; no lifecycle/release/professional
claim. Residual: the `check_definitions` form builder (and the C4 end-to-end
rule-check path) remain the next C2/Phase C work.

## 2026-06-13 - C2 editor GUI slice 3 landed: table-node structured sub-editor (`TP-C2-TABLENODE-001`)

Completes structural authorability of the frozen grammar v1.0.0 in the
rule-pack expression composer. `interpolate` and `lookup` nodes — preserved
read-only by slice 2 — are now first-class authorable node types in
`apps/desktop/src/features/rule-packs/ExpressionComposer.tsx`. Both are added
to `EDITABLE_NODE_TYPES` (so they are reachable from every node-type selector
and `defaultExpressionNode` seeds a schema-valid default table: two
strictly-increasing rows, uppercase `"TBD"` placeholders, plus the recursive
`argument` child; `lookup` also seeds `mode: "exact"`). A structured table
sub-editor authors `table_id`, the argument/result `dimension` (via a shared
`DimensionSelect`) and `unit_ref`, the `{argument, result}` rows (**Add row**
appends last-argument + 1 to keep strict monotonicity; **Remove row** is
blocked at the single-row schema floor), the `lookup` `mode` selector, and the
recursive editor for the table's `argument` expression. With this, no grammar
node type is left to raw-JSON editing; only the refusal markers
(`unsupported_form`/`unsafe_host_access`) and unrecognized tags remain
preserved read-only. Lossless: row edits patch only the touched row, so
untouched rows and sibling subtrees round-trip verbatim.

Regression repair folded in: slice 2 defaulted a literal's dimension to the
lowercase `"tbd"` token and listed it in the shared `DIMENSIONS` vocabulary.
That token is not in the document codec (`core/rules/rule_pack_document`
`decode_quantity` → "unknown dimension token 'tbd'") nor the schema
`DimensionId` enum — both use uppercase `"TBD"` (the token the draft builder
and the evaluator's `Dimension::Tbd => "TBD"` already use). A composer-authored
default literal therefore produced a document that fails backend validation/
save; the slice-2 Vitest missed it because it asserted the produced string,
never a Rust round-trip. Fixed `DIMENSIONS`, the literal default, and the
fallback to `"TBD"`, and added a unit guard.

D-02b gate held: purely structured form controls — no writable expression text
syntax and no text rendering of the AST; the typed AST stays the sole edited,
checksum-bound form (DEC-022). No protected tables/constants; private rule
packs stay local-only.

Validation: rule-packs Vitest 26/26 (was 21), `tsc -b` clean, Playwright
`-g "rule-pack manager"` 2/2 (both viewports, table sub-editor driven from
blank). A five-lens pre-commit review caught one schema-conformance blocker
(the slice-2 default literal emitted `unit_required`/`dimension_check_required`,
which the schema's `additionalProperties:false` `ExpressionQuantity` forbids) —
fixed by emitting only `{value, dimension, unit_ref}`; details in the run
record. The full Vitest run on a busy host showed the documented load-induced
`App.test.tsx` per-test timeout flake (all 34 failures timeouts in that
untouched file); confirmed tranche-independent — once load cleared
`App.test.tsx` passed 52/52 on both my tree and pristine HEAD, and no failing
test renders the changed code. Detail and the gate-determinism follow-up are
in the run record. Evidence: `apps/desktop/SMOKE.md` TP-MAC-150;
DEL-07-03 run record `WORKING_ITEMS_RUN_2026-06-13_TP-C2-TABLENODE-001.md`.
Residual: the required-input/value-slot/load-combination form builders remain
the last C2 authoring sub-surface; C4 (engine-side rule checks on solved user
models) is a separate Phase C item.

## 2026-06-13 - C2 editor GUI slice 2 landed: structured AST expression composer (`TP-C2-COMPOSER-001`)

The PRD §14.5 "Expression editor". New
`apps/desktop/src/features/rule-packs/ExpressionComposer.tsx` is a recursive
form/tree editor that builds the frozen grammar v1.0.0 typed expression AST
(DEC-022) for a rule pack's selected formula, replacing the slice-1 raw
declarative-AST JSON editing of expressions. Structured controls cover the
full non-table node set (literal, variable_ref, unary, binary, compare,
logical, select, aggregate — with add/remove aggregate operands and a
last-operand-removal block); a variable picker / read-only browser is sourced
from the pack's declared `required_inputs`/`value_slots` (PRD §14.5 "Variable
browser"). The composer derives its tree from, and re-serializes edits back
into, the canonical document JSON the validate/checksum/save flow already
reads (single source of truth). `RulePackManagerPanel` also gains the
in-request busy guard that closes the slice-1 clobber residual: the textarea,
composer, and all actions disable with a stated reason while a backend
request is awaiting.

D-02b gate held: the composer is purely structured — no writable expression
text syntax and no text rendering of the AST (read-only rendering is itself
an open D-02b §3 Q5 question). The typed AST stays the sole edited and
checksum-bound form. Lossless preservation: table-backed (interpolate/lookup)
and unrecognized nodes render read-only with no node-type selector and
round-trip byte-for-byte — never silently dropped.

A four-lens pre-commit adversarial review (react-correctness,
governance/D-02b boundary, AST-encoding correctness, evidence-honesty) found
no implementation defects; three test-honesty findings were fixed before
commit (vacuous table-preservation `toMatchObject` → full `toEqual`; added a
test that an unrecognized node survives a sibling edit; shallow e2e
assertion → full composed-structure match).

Validation: targeted Vitest `src/features/rule-packs` 21/21; Playwright
`-g "rule-pack manager"` 2/2 (two-viewport, composer driven from blank);
`tsc -b` clean. The full-suite `App.test.tsx` timeout flakiness is
pre-existing and host-environmental (reproduces on pristine HEAD with this
tranche's panel change reverted; varying failing set), unrelated to this
slice. Residuals: table-node structured sub-editor; required-input/
value-slot/load-combination form builders; engine-side rule evaluation
(C4). Evidence: DEL-07-03 run record
`WORKING_ITEMS_RUN_2026-06-13_TP-C2-COMPOSER-001.md`; SMOKE TP-MAC-149.

## 2026-06-12 - C2 editor GUI slice 1 landed: rule-pack manager (`TP-C2-EDITOR-001`)

The first runtime rule-pack editor surface (PRD §14.5 / §14.1 "Rule-pack
manager"), consuming the TP-C2-RPLIFE-001 backend commands. New "Rule Packs"
workspace section (placed between Load Cases and Solve), `RulePackManagerPanel`,
and a `rulePackService` frontend route. Covers pack management + document-level
authoring: new private-by-default draft (privacy_class=private_user_data,
redistribution_status=private_only per PRD §12.4), list/open/save/delete against
the local-only store, validate, and compute-and-stamp checksum, with validation
findings and the computed checksum rendered. Expressions are authored in the
document's native declarative-AST JSON (DEC-022); no expression text syntax
ships (D-02b AWAITING_RULING), and the draft template is placeholder/invented
only — no protected equations or standards values (PRD §14.5). Honest browser
seam (RULE-PACK-BACKEND-DESKTOP-ONLY), honest desktop error surfacing
(RULE-PACK-BACKEND-ERROR), and a permanent boundary note. A three-lens
pre-commit review surfaced four findings; three were fixed (unhandled desktop
rejections, stale list on project switch, two vacuous e2e assertions) and one
(mid-request textarea clobber) was recorded as a residual. Validation: Vitest
251/251 (8 new; dead-control audit green), Playwright dev 6/6 (two viewports),
dist 1/1, tsc -b + vite build clean. Residuals routed to the next C2 slices:
structured AST expression composer; required-input/value-slot/load-combination
form builders; in-request busy guard. Engine-side rule evaluation on solved user
models (USER_RULE_CHECKED/USER_RULE_FAILED end-to-end) is C4.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-148; DEL-07-03 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-EDITOR-001.md`. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - C2 backend seam landed: rule-pack validation, JCS checksum, local store (`TP-C2-RPLIFE-001`)

The first production wiring of the previously-orphan `core/rules` crates.
New crate `core/rules/rule_pack_document`: a production codec between the
rule-pack document AST encoding and the frozen DEC-022 grammar (all 69
conformance-corpus expressions round-trip; decode errors carry JSON-path
breadcrumbs for editor diagnostics), grammar-version-bound JCS checksum
computation over the document minus its `checksums` member (RFC 8785 via
`core/serialization/canonical_json` + the DEL-06-04 binding constructor;
`payload_excludes` recorded, with a matching additive schema member), and
document-level validation composing shape, grammar, expression-decode,
checksum-match, and `validate_lifecycle` findings into one envelope with a
fixed professional-boundary notice. Desktop seam: six new Tauri commands
(`validate_rule_pack`, `compute_rule_pack_document_checksum`, and
save/open/list/delete local rule-pack persistence) over a v10 store
migration adding the project-scoped `local_rule_packs` table — drafts with
blocking findings remain saveable; user packs stay in local SQLite only,
never committed or transmitted (OPS-K-PRIV-1). The DEL-06-05 example pack
gained a real stamped checksum pinned by a Rust golden test and a Python
JCS recomputation (cross-engine parity witness); the DEL-11-04 toy model
was restamped accordingly. Validation: rule_pack_document 10/10, src-tauri
37/37 (4 new tests; store-migration evidence re-pinned for v10), pytest
359/359, fmt clean. Residuals: editor GUI consumes these commands next
(Playwright/Vitest evidence rides that tranche per H4); check-evaluation
composition and load-case mapping vocabulary stay routed to the C4
lead-up; cross-project library storage stays C3 scope.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-147; DEL-06-04 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-RPLIFE-001.md`. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - C2 lead-up landed: D-02b packet + DEC-022 schema absorption (`TP-C2-SCHEMA-001`)

Two C2 lead-up actions per the Phase C stage gate. First, the **D-02b decision
packet** (writable rule-expression text syntax) was prepared and registered
AWAITING_RULING — options O-A..O-D with a PROPOSAL-labeled recommendation
(O-C: defer the writable syntax behind a named usability-evidence trigger;
permit a display-only deterministic AST rendering); no writable text input
ships until the human rules. Second, the named C1 residual closed:
`schemas/rule_pack.schema.yaml` absorbed DEC-022 — required top-level
`grammar_version` (semver, sits inside the JCS-hashed `rule_pack_checksum`
payload), `ExpressionNode` declarative-AST encoding mirroring the conformance
corpus (evaluator refusal markers and literal relaxation flags deliberately
not authorable in pack documents), `UserTableValue` for interpolate/lookup
(D-02 §3 Q6), shared `DimensionId`, `grammar_status:
frozen_open_pipe_stress_declared_expression`, and conditional gates pinning
declarative-AST formulas to the frozen language. The DEL-06-05 example pack
became a real evaluable instance (grammar_version 1.0.0, declarative_ast
divide of the two demo quantities deriving the declared dimensionless ratio;
versions bumped to 0.2.0; expression_grammar/evaluator_library open-decisions
flipped to accepted with DEC-022 pointers), and the DEL-11-04 toy model's
rule-pack ref was restamped (version, raw-bytes sha256, project JCS hash).
Validation: full root pytest 358/0 (extended schema tests: seven new
negative JSON Schema cases). Residuals routed: checksum value stamping +
example-AST Rust cross-check → C2 backend seam tranche; check-evaluation
composition and load-case mapping vocabulary (surfaced TBDs, not resolved) →
C4 lead-up.

Evidence: DEL-06-01 run record
`WORKING_ITEMS_RUN_2026-06-12_TP-C2-SCHEMA-001.md`; decision packet
`execution/_Coordination/_DECISIONS/D-02b_rule_expression_text_syntax.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A3 sub-slice landed: workspace IA and usability repair (`TP-APP-R2-UXSHELL-001`)

Response to the human packaged-pass attempt-2 usability verdict (TP-MAC-141).
The shell moved from an accreted vertical panel stack to the PRD §14.1
workspace: persistent model tree | 3D viewport | property inspector core
(DEL-07-02 RQ-001/002), with the remaining surfaces behind always-visible
section tabs in A12-journey order (Operation Apply with live queue badge,
Load Cases, Solve, Results, Report, Project, Exports, Audit & Boundaries;
vocabulary from DEL-07-06 Guidance and the checklist's own terms). Inactive
sections stay mounted (drafts/queue state survive navigation); unimplemented
PRD surfaces remain absent — no placeholder controls; boundary strip stays
permanently visible (DEL-07-05 principle). Scroll/overflow fixed
structurally and browser-verified at 1440×920 and 1280×800 with a DOM
overflow probe. New permanent dead-control audit
(`App.deadControls.test.tsx`): every rendered button must produce an
observable change or carry an accessible disabled reason — caught 4
controls, each fixed or reason-labeled; all data-testids preserved.
Validation: Vitest 242/242, dev e2e 4/4 (two-viewport matrix, navigation
driven through visible controls), dist e2e 1/1, build green. Residuals:
1280×800 dock height cosmetic; WCAG target TBD (DEL-07-06-CF-001); canvas
gestures remain separate A3 scope. Nav-pattern choice recorded as PROPOSAL
(kits silent on concrete pattern).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-146; DEL-07-06 run record
`TASK_RUN_2026-06-12_1756.md`.

## 2026-06-12 - Regression repair: packaged builds ship the wasm engine (`TP-APP-R2-WASMPKG-001`)

The first human TP-MAC-141 packaged pass failed at `New blank` with
`WASM-ENGINE-ASSET-ABSENT` — the F-4 seam doing its job. Root cause: frontend
hashing requires the wasm engine in every mode (H1/F-5a, no fallback), but
the generated assets rode a vite-ignored dynamic import only the dev server
resolves; `dist/` never shipped them and `tauri build` never built them.
Fix: assets emit into the Vite publicDir (`public/wasm-engine/`, atomic
rename preserved), the loader imports a fully-qualified root URL valid in
dev/preview/`tauri://` lanes plus node fs candidates,
`beforeBuildCommand` chains the wasm build (self-sufficient `tauri build`),
a `wasm-engine-dist-guard` vite plugin fails production builds loudly on
absence (demonstrated), and a new production-dist Playwright lane
(`test:e2e:dist`) replays the regression against `vite preview` — run by the
DEC-025 sweep as a second command inside the existing e2e surface (five
surfaces unchanged). Validation: Vitest 241/241, dev e2e 2/2, dist e2e 1/1,
pytest 358/358, build green, rebuilt bundle boot-checked. Honest residual:
the `tauri://` protocol itself is exercised only by the human packaged pass.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-144 and the TP-MAC-141 attempt-1
failure record; DEL-00-08 run record `TASK_RUN_2026-06-12_1355.md`.

## 2026-06-12 - DEC-033 implemented: model document 0.2.0 (`TP-APP-R2-DOCVER-020-001`)

`SUPPORTED_MODEL_SCHEMA_VERSION` 0.1.0 → 0.2.0 on both evaluation surfaces
(src-tauri authority + projectService mirror, pending H2 unification) with
published no-op transform `model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop`.
Blank documents author 0.2.0; 0.1.0 documents migrate in memory with the
migration id in the DEC-019 ledger; browser preview keeps stored snapshot
bytes unchanged for migrated documents. Bundled preview fixtures deliberately
stay 0.1.0-era and now exercise the real migration path; contract corpus
byte-identical. Validation: src-tauri 33/33, applier 58+1+2/0, Vitest
241/241, pytest 358/358, build green. Residuals routed to the H2 row (panel
version-literal display staleness; migrated-bytes hash-integrity edge).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-145; DEL-02-05 run record
`TASK_RUN_2026-06-12_1407.md`.

## 2026-06-12 - A4 sub-slice landed: subtraction/range combination authoring (`TP-APP-R2-COMBEXPR-001`)

Closed combination basis set landed end-to-end mirroring the
`core/loads/load_case_algebra` contract vocabulary: `mechanics` (unchanged),
`result_state_subtraction` (`minuend_id`/`subtrahend_id`), `range_envelope`
(`operand_ids` + `mode` in `min|max|min_abs|max_abs`), carried as strictly
additive optional model fields (schema_version held at 0.1.0; regenerable
mechanics fixture byte-identical). Seam: per-basis create validation with
named blocking codes (`OP-COMBINATION-BASIS-UNSUPPORTED`,
`OP-CREATE-COMBINATION-PAYLOAD-INVALID`, `OP-COMBINATION-RANGE-MODE-UNKNOWN`,
`OP-COMBINATION-OPERAND-DUPLICATE`, `OP-COMBINATION-OPERAND-LOAD-NOT-FOUND`,
`OP-COMBINATION-TERM-BASIS-UNSUPPORTED`); load-case delete counts
subtraction/range references. Solve: deterministic evaluation reusing
`load_case_algebra` with pre-solve named blocks
(`LOAD_COMBINATION_SHAPE_INVALID`, `_RANGE_MODE_UNKNOWN`, `_OPERANDS_EMPTY`)
— no silent zeros or skips. UI: basis selector with conditional per-basis
create fields; the combination basis editor moved from free text to the
closed-set select with cross-shape edits blocked
(`OP-COMBINATION-BASIS-SHAPE-MISMATCH`) — a recorded behavior change (the
prior free-text edit was an implementation choice, not a DEC-recorded
decision). Corpus cases 58–65 blessed and re-run through both engines;
README inventory marks them pending human review (DEC-030 covers cases
01–57 only). Disclosed write-scope deviation ratified at fan-in:
`core/product_physics/src/validation.rs` (the pre-existing basis gate)
required extension beyond the brief's named files. Validation: applier
58+1+2 / 0 (65 corpus cases), product_physics 31/0, load_case_algebra 18/0,
Vitest 239/0, e2e 2/2 (authors a subtraction combination through visible
controls in real Chromium), pytest 358/0, build green, fmt clean.
Open review items: corpus 58–65 human review; DEC-019 whether additive
optional fields warrant a 0.2.0 document-version bump + no-op migration
entry (held additive at 0.1.0).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-143;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/TASK_RUN_2026-06-12_1138.md`.

## 2026-06-12 - A6 sub-slice landed: true directional deformed shape (`TP-APP-R2-DEFORMEDDIR-001`)

`core/product_physics::solve_load_case` now emits signed per-node
displacement component rows for all six DOF (ids
`result:disp:<node>:{ux,uy,uz,rx,ry,rz}`, kinds
`global_nodal_displacement_{x,y,z}` / `global_nodal_rotation_{x,y,z}`, mm/rad,
metadata per the established force-row pattern) after the existing
`displacement_magnitude` rows; existing rows byte-identical; `rad` joined the
combination algebra dimensions so combination rotation rows evaluate instead
of warning. `PipeViewport.buildDeformationOverlay` moves nodes along the true
(ux,uy,uz) unit vector scaled by the retained normalized display offset;
boundary discloses `vector_direction=global_cartesian_displacement_components`
(previously `TBD`), with an honest disclosed magnitude-only fallback when
component rows are absent. Canned browser fixture regenerated through the
documented workflow and independently verified purely additive (647 rows
preserved in order, 90 component rows added); first Playwright e2e asserts
the directional disclosure after solve in real Chromium (H4 default posture,
no exception). Validation: product_physics 28/28, src-tauri 32/32, Vitest
220/220, e2e 2/2, pytest 358/358, desktop build green. Residuals stay in the
A6 row; two convention calls recorded for review in the run record (basis
token `solved_from_global_linear_system` not yet in the results-schema enum;
`rad` combination-dimension mapping).

Evidence: `apps/desktop/SMOKE.md` TP-MAC-142;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_run_records/TASK_RUN_2026-06-12_1110.md`.

## 2026-06-12 - Evidence-sweep git-state hardening (`TP-R2VERIFY-FIX-002`)

Repair of verification finding F-2
([VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md)):
the DEC-025 sweep summary could record a clean-looking git block when the git
capture itself failed, because `_capture()` converts any nonzero git exit into
`None` and the state collector defaulted that to "no dirty paths".
`collect_git_state` now records `status_capture_failed` explicitly,
`working_tree_dirty` becomes `null` (never `false`) when the capture fails,
`summary_filename` marks such summaries `-gitunverified`, and `main --execute`
exits nonzero with a named message because an unbound summary does not satisfy
the DEC-025 commit-binding contract. Summary `schema_version` bumped 1 → 2 for
the new member. Tests: 3 new/updated focused cases; `tests/test_evidence_sweep.py`
16/16 green.

Evidence: `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/WORKING_ITEMS_RUN_2026-06-12_sweep_git_state_hardening.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - Authored-load preview-category mapping disclosure (`TP-R2VERIFY-FIX-001`)

Repair of verification finding F-1
([VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md)):
the A12 adapter silently coerced operation-authored load categories
(`concentrated_force`/`concentrated_moment` → `occasional`,
`distributed_force` → `weight`) at the product-physics boundary. The mapping
now surfaces as a named per-load `warning` diagnostic
(`LOAD_CATEGORY_PREVIEW_MAPPED`) stating that the preview classification is
not a user-selected engineering classification; native preview categories do
not emit it; solve status is unchanged. The extended regression pins the
diagnostic code, severity, affected refs, and the native-category negative
case. `core/product_physics` 25/25 green.

Evidence: `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_category_mapping_disclosure.md`.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - R2 exit-chain verification snapshot (`TP-INTEGRATED-VERIFY-002`)

The completion plan's Phase A exit-evidence row called for a derivative
verification snapshot (TP-INTEGRATED-VERIFY successor) for human review; it
now exists:
[VERIFICATION_2026-06-12_r2_exit_chain.md](VERIFICATION_2026-06-12_r2_exit_chain.md).
An independent WORKING_ITEMS session re-ran the R2-chain surfaces (Tauri
backend 32/32 including the A12 rehearsal and saved-project regressions;
product_physics 25/25; units 13/13; Playwright 2/2 including the from-blank
GUI journey), built and booted the packaged desktop binary
(`tauri build --debug --no-bundle`; SMOKE TP-MAC-140), and audited the landed
A9-A12/A8 work against PRD §22.3 verbatim. Verdict: the R2 exit criterion is
demonstrated in substance across two complementary verified surfaces; the
packaged-runtime GUI journey remains the one named gap (finding F-4, recorded
with its macOS automation blockers per the H4 exception). Findings F-1/F-2
were repaired same-session; F-3 routed to the next PKG-17 tranche. The
snapshot feeds the `D-14` stage-advancement decision packet.

Evidence: `execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_r2_exit_chain_verification.md`;
`apps/desktop/SMOKE.md` TP-MAC-140. No lifecycle state, stage advancement,
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - B2 export unit-system disclosure (`TP-UNITS-B2-EXPORTDISCLOSURE-001`)

PCF, CAEPIPE MBF, and stress-neutral export packages now carry explicit
DEC-018 unit-system disclosure instead of relying only on target text units or
per-row result units. The desktop export panels show a `Units` row with source
model units, target export units, result units where present, and whether
export-time conversion was performed. Downloaded JSON packages include a
`unit_system_disclosure` member with
`unit-system:dec-018-si-dual-display`, `entered_units_preserved`, model units,
target export units, result units, conversion policy/scope, DEC-018/DEL-02-02
basis refs, and protected/private-content false flags.

The schema-owned export builders now require and checksum the same member:
`core/handoff/pcf_export/package.py`,
`core/handoff/caepipe_mbf/package.py`, and
`core/handoff/stress_neutral/package.py` emit `unit_system_disclosure.json` in
their manifest package-member lists. The strict JSON schemas were widened to
make this member required, and invented fixtures were regenerated through the
deterministic builders.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_run_records/WORKING_ITEMS_RUN_2026-06-12_export_unit_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-139. Validation: focused export-package tests
passed 32/32; repository Python tests passed 356/356; full desktop Vitest
passed 216/216; desktop build passed with the pre-existing Vite chunk-size
warning; Playwright R2 smoke passed 2/2 after wasm engine build.

Residual hand-offs: B2 still owns import round-trip unit I/O, target-format
conversion witnesses beyond package disclosure, broader app unit entry/pickers
outside the already covered forms, and rule-pack unit I/O. No lifecycle state,
release-readiness, target compatibility, solver-deck validation, professional
approval, certification, sealing, authentication, protected-content,
private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector quantity unit edits (`TP-UNITS-B2-INSPECTOREDITUNITS-001`)

Existing Property Inspector material and pipe-section quantity edits now carry
unit intent instead of changing only `.value`. Material elastic modulus, shear
modulus, thermal expansion coefficient, pipe outside diameter, and pipe wall
thickness expose a `Unit` selector beside the proposed value. Browser preview
continues to show a single model-metadata option when the desktop catalog route
is unavailable; desktop/Tauri mode can use accepted DEC-018 catalog options.
Node coordinate edits remain value-only because project length-unit mutation is
a separate project-unit-system concern.

The Rust/wasm operation seam now treats `{ value, unit }` as a generic
sibling-unit quantity edit payload. Compatible entered units are accepted only
when the same validated edit also writes the sibling `.unit` field, so alternate
units cannot pass as a value-only partial update. Native regressions cover an
existing material modulus edited to `MPa`, an existing pipe outside diameter
edited to `mm`, and incompatible `mm` rejection for a stress quantity.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-01_Material library schema with provenance/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-03_Piping Components, Materials, and Library Data Model/1_Working/DEL-03-02_Pipe section and component library schema/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_quantity_unit_edits.md`;
`apps/desktop/SMOKE.md` TP-MAC-138. Validation:
operation-applier cargo suites passed 54 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create/edit forms, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage outside the witnesses named above. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - B2 primitive-load magnitude unit edits (`TP-UNITS-B2-PRIMEDITUNITS-001`)

Existing primitive-load magnitude edits now carry unit intent instead of only
editing `.value`. The Load Cases manager exposes a `Magnitude unit` selector
for the selected primitive load, labels the magnitude field with the active
unit basis, and queues an atomic value+unit payload. Browser preview remains
model-metadata-only when the desktop catalog route is unavailable; desktop mode
can use accepted DEC-018 options.

The Rust/wasm operation seam keeps legacy numeric-string primitive magnitude
edits valid, but also accepts the new `{ value, unit }` payload for
`primitive_loads.N.magnitude.value`. Compatible entered units are preserved by
writing the sibling `.unit` field in the same apply step. A native regression
covers an existing force-per-length primitive edited to `lbf/ft`, and rejects
`mm` for that dimension.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_magnitude_unit_edits.md`;
`apps/desktop/SMOKE.md` TP-MAC-137. Validation:
operation-applier cargo suites passed 53 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2 after
wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create/edit forms, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage outside the witnesses named above. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - B2 primitive-load unit selectors (`TP-UNITS-B2-LOADPICKERS-001`)

The Load Cases manager primitive-load creation form now has visible unit
selection for the entered magnitude. Browser preview remains honest with a
single model-metadata option; desktop/Tauri mode loads accepted DEC-018 unit
catalog entries for the selected primitive-load dimension and labels
canonical/display options explicitly. Category and direction changes reset the
selector to the model's default unit for that load family.

The structured operation seam now accepts DEC-018-compatible entered units for
created primitive loads while preserving the entered unit in the applied
session model. Native regressions cover a concentrated force entered in `lbf`,
a pressure load entered in `kPa`, and incompatible `mm` rejection for a force
load. Existing primitive-load magnitude edits remain out of scope because
changing a numeric `.value` without a sibling `.unit` update path would create
ambiguous partial edits.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-05_Concentrated and distributed user load application/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_primitive_load_unit_selectors.md`;
`apps/desktop/SMOKE.md` TP-MAC-136. Validation:
operation-applier cargo suites passed 52 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2
after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
and primitive-load create forms, imports/exports, rule-pack unit I/O, and
existing primitive-load magnitude-edit unit handling remain B2 work. B3 still
owns broader mixed-unit round-trip, conversion-witness, incompatible-unit
rejection, and D-04/DEC-026 tolerance coverage outside the witnesses named
above. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 rendered report body unit disclosure (`TP-UNITS-B2-REPORTBODY-001`)

The hash-bound rendered report body now carries explicit unit evidence instead
of relying only on the Report Packet JSON disclosure. The report-generator
model-input schema has a backward-compatible optional `unit_display_summary`
with entered-unit storage convention, sorted model unit map, distinct result
units, report display policy, and report-time conversion flag. The React
report adapter populates that summary from the current session model and
result rows, and the Rust renderer displays it in the `Model Input Summary`
section.

The shared invented report fixture and Tauri report-command tests now prove
the rendered HTML includes `Unit storage convention`, `Model units`, `Result
units`, and `Report-time conversion`. The A12 from-blank rehearsal helper
overrides the fixture summary with the actual authored model units and solved
result units, so the full author -> solve -> render regression also carries
this B2 evidence.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_body_unit_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-135. Validation: report-generator schema
contract test passed; report-generator cargo tests passed 10/10;
report-renderer cargo tests passed 8/8; Tauri Rust tests passed 32/32;
focused report/App Vitest passed 53/53; full desktop Vitest passed 216/216;
desktop build passed with the pre-existing Vite chunk-size warning;
Playwright R2 smoke passed 2/2 after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside material/section
create forms, imports/exports, and rule-pack unit I/O remain B2 work. B3
still owns broader mixed-unit round-trip, conversion-witness,
incompatible-unit rejection, and D-04/DEC-026 tolerance coverage outside the
witnesses already landed. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector unit selectors (`TP-UNITS-B2-UNITPICKERS-001`)

The Property Inspector material and pipe-section creation forms now have the
first B2 visible unit entry controls. Section creation has a `Length unit`
selector, material creation has `Modulus unit` and `Thermal expansion unit`
selectors, and the existing field labels/status panel follow the selected
unit basis. Browser preview remains honest with one-option selectors from
model metadata and no synthesized fallback catalog; desktop/Tauri mode draws
accepted options from the DEC-018 catalog route.

The structured operation seam now accepts DEC-018-compatible entered units for
create-section length quantities and create-material stress /
thermal-expansion quantities, preserving entered units in the applied session
model. Native Rust regressions cover `MPa` material moduli, `1/K` thermal
expansion, `mm` section geometry, and incompatible material-unit rejection.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_selectors.md`;
`apps/desktop/SMOKE.md` TP-MAC-134. Validation:
operation-applier cargo suites passed 51 unit tests plus canonical-hash and
contract-corpus tests; Tauri Rust tests passed 32/32; focused desktop Vitest
passed 165/165; full desktop Vitest passed 216/216; desktop build passed with
the pre-existing Vite chunk-size warning; Playwright R2 smoke passed 2/2
after wasm engine build.

Residual hand-offs: broader app unit entry/pickers outside these material and
section create forms, report renderer body expansion beyond packet
disclosure, imports/exports, and rule-pack unit I/O remain B2 work. B3 still
owns broader mixed-unit round-trip, conversion-witness, incompatible-unit
rejection, and D-04/DEC-026 tolerance coverage outside the witnesses named
above. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 solver-boundary unit normalization (`TP-UNITS-B2-SOLVERNORM-001`)

The preview mechanics adapter now uses the accepted DEC-018 unit catalog at
the solver boundary. `core/product_physics` depends on `core/units`, validates
unit-bearing inputs by dimension compatibility instead of exact canonical
strings, and normalizes compatible quantities to SI-canonical values before
building the frame model, supports, sections, thermal loads, pressure thrust,
and primitive loads.

The product-physics regression suite now includes a mixed-unit equivalence
witness: material moduli entered in `MPa`, pipe dimensions in `mm`, and
pressure loads in `kPa` solve to the same rounded result surface as the
original SI fixture. Incompatible material and load units still block with
`UNIT_INPUT_INVALID`; unexpected conversion failures surface as
`UNIT_CONVERSION_UNAVAILABLE`.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_solver_boundary_unit_normalization.md`;
`apps/desktop/SMOKE.md` TP-MAC-133. Validation: product_physics cargo tests
passed 25/25; Tauri Rust tests passed 32/32; headless runner cargo tests
passed 11/11; focused desktop Vitest passed 56/56; Playwright R2 smoke passed
2/2 after wasm engine build; full desktop Vitest passed 216/216; desktop build
passed with the pre-existing Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, report renderer body
expansion beyond packet disclosure, imports/exports, and rule-pack unit I/O
remain B2 work. B3 still owns the broader conversion-witness and
D-04/DEC-026 tolerance corpus beyond this product-physics boundary witness.
No lifecycle state, release-readiness, professional approval, certification,
sealing, authentication, protected-content, private-data, or code-compliance
claim is created.

## 2026-06-12 - B2 report unit-system disclosure (`TP-UNITS-B2-REPORTUNITS-001`)

The Report Packet panel and JSON export now disclose the unit system used by
the current model/result packet. The visible report line names
`unit-system:dec-018-si-dual-display`, lists model units, lists distinct
result-row units, and records `conversion=false`. The exported report JSON
adds `unit_system_disclosure` with the model unit map, result-unit inventory,
entered-unit preservation posture, and no report-time conversion claim.

The rendered-report adapter also replaces the old placeholder
`preview-display-label-set` with `unit-system:dec-018-si-dual-display` in the
existing strict `model_input_summary.unit_system_ref` field. The
report-generator schema was not widened.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-12_report_unit_system_disclosure.md`;
`apps/desktop/SMOKE.md` TP-MAC-132. Validation: focused report/App Vitest
passed 53/53; Playwright R2 smoke passed 2/2 after wasm engine build; full
desktop Vitest passed 216/216; desktop build passed with the pre-existing
Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, solver-boundary
normalization, report renderer body expansion beyond packet disclosure,
imports/exports, and rule-pack unit I/O remain B2 work. B3 still owns broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance coverage. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - B2 Property Inspector unit basis labels (`TP-UNITS-B2-INSPECTORLABELS-001`)

The desktop Property Inspector now has the first visible B2 unit display
retrofit. `unitCatalogService.ts` exposes catalog lookup/display helpers that
match DEC-018 entries by symbol and dimension, including display-only
equivalents needed by current fields such as `stress` through pressure units.
The helpers do not normalize values; browser-preview unavailability and
catalog misses are explicit states.

The Property Inspector loads the unit-catalog route once, renders a compact
`Unit basis` panel, and annotates material and pipe-section creation labels
with their unit source. Browser preview shows `m, model metadata`, `Pa, model
metadata`, and `1/degC, model metadata`; desktop/Tauri mode can display the
DEC-018 catalog basis from the `get_unit_catalog` command.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_property_inspector_unit_basis_labels.md`;
`apps/desktop/SMOKE.md` TP-MAC-131. Validation: focused
unit-catalog/App Vitest passed 48/48; Playwright R2 smoke passed 2/2 after
wasm engine build; full desktop Vitest passed 216/216; desktop build passed
with the pre-existing Vite chunk-size warning.

Residual hand-offs: broader visible unit entry/pickers, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 frontend unit catalog service (`TP-UNITS-B2-FRONTENDSVC-001`)

The desktop frontend now has a typed service route to the reviewed unit
catalog. `apps/desktop/src/services/unitCatalogService.ts` defines the catalog
payload shape, `loadUnitCatalog`, and `acceptedUnits`. In desktop/Tauri mode
the service invokes `get_unit_catalog`; in browser preview mode it returns an
explicit `UNIT-CATALOG-DESKTOP-ONLY` unavailable route instead of inventing a
fallback catalog.

Vitest coverage pins both routes. The Tauri-route test verifies DEC-018
metadata, entered-unit preservation, factor/offset text, conventional-public
provenance, and boundary flags. This prepares visible unit-picker/display work
without changing authoring forms in this tranche.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_frontend_unit_catalog_service.md`;
`apps/desktop/SMOKE.md` TP-MAC-130. Validation:
`npm test --workspace apps/desktop -- unitCatalogService` passed with 2
tests; `npm test --workspace apps/desktop` passed with 215 tests across 9
files; `npm run build --workspace apps/desktop` passed with the pre-existing
Vite chunk-size warning.

Residual hand-offs: visible app unit pickers/displays, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2 desktop unit catalog command (`TP-UNITS-B2-CATALOGCMD-001`)

The desktop backend now has a tested B2 binding to the DEC-018 units crate.
`openpipestress-desktop` depends on `open_pipe_stress_units` and exposes a
`get_unit_catalog` Tauri command. The command returns schema-facing catalog
records with stable `unit:*` ids, symbols, dimension ids, canonical flags,
transform kinds, factor representation, optional offset representation,
provenance, review status, and boundary flags. This gives the app a durable
source for future unit picker/display controls instead of copying unit labels
into each authoring surface.

The units crate now also exposes stable catalog IDs and string values for
transform kind, conversion provenance, and review status. Focused Tauri
coverage pins the important B2 metadata: `DEC-018`, entered-unit preservation,
the inch factor text, Fahrenheit offset text, lbf conventional-public-constant
provenance, project-governed semantic binding, and no protected/private data
or professional/code-compliance claim.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_desktop_unit_catalog_binding.md`;
`apps/desktop/SMOKE.md` TP-MAC-129. Validation:
`cargo fmt --manifest-path core/units/Cargo.toml --check` passed;
`cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check`
passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13
unit tests and 0 doctests; focused Tauri command test passed 1/1; full
`cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passed with 32
unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed.

Residual hand-offs: visible app unit pickers/displays, solver-boundary
normalization, report unit-system disclosures, imports/exports, and rule-pack
unit I/O remain B2 work. B3 still owns broader mixed-unit round-trip,
conversion-witness, incompatible-unit rejection, and D-04/DEC-026 tolerance
coverage. No lifecycle state, release-readiness, professional approval,
certification, sealing, authentication, protected-content, private-data, or
code-compliance claim is created.

## 2026-06-12 - B2/B3 units schema-crate contract rider (`TP-UNITS-B2B3-CONTRACT-001`)

The units lane now has an executable schema/crate contract for the two
2026-06-12 findings routed after B1. `core/units` exposes
`factor_representation`, optional `offset_representation`, explicit
`ConversionProvenance`, and `ReviewStatus` on each `UnitDefinition`, so B2
schema/app/report bindings have a crate-side place to preserve per-constant
derivation text and review state. Provenance is no longer derived as a
SI-canonical vs display-unit binary: lbf/psi-family conversions are marked as
`ConventionalPublicConstant`, and project-owned semantic canonical bindings
are marked as `ProjectGovernedDecision`.

B3 also gained the requested executable drift guard. A new Rust regression
parses `schemas/units.schema.yaml` and asserts set equality between the schema
`DimensionId` enum and the crate `DIMENSIONS` vocabulary, preventing the
current hand-checked identifier match from silently drifting.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_units_schema_crate_contract.md`.
Validation: `cargo fmt --manifest-path core/units/Cargo.toml --check`
passed; `cargo test --manifest-path core/units/Cargo.toml` passed with 13
unit tests and 0 doctests; `python3 tests/test_units_schema.py` passed;
`python3 -m pytest tests/test_units_schema.py` passed with 3 tests.

Residual hand-offs: this is not full B2 closure. B2 still must bind unit-aware
I/O through schemas, desktop fields, solver-boundary normalization, reports,
imports, exports, and rule-pack evaluation. B3 still must add broader
mixed-unit round-trip, conversion-witness, incompatible-unit rejection, and
D-04/DEC-026 tolerance corpus coverage. No lifecycle state, release-readiness,
professional approval, certification, sealing, authentication,
protected-content, private-data, or code-compliance claim is created.

## 2026-06-12 - A8 saved-project backend smoke (`TP-APP-R2-SAVEDPROJECT-SMOKE-001`)

The A8 saved-project residual now has a Tauri/backend regression. The new
test `r2_from_blank_saved_project_opens_solves_and_renders_report` reuses the
A12 rehearsal helpers, applies the from-blank script, saves the authored model
into the desktop local SQLite project store, reopens it by project id, solves
the loaded model through `run_preview_mechanics`, and renders through the A7
`render_calculation_report` command. The test verifies the store did not
silently persist mechanics or analysis-run payloads before solve, then records
the solved run/report path after reopening.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_saved_project_backend_smoke.md`;
`apps/desktop/SMOKE.md` TP-MAC-128. Validation: focused Tauri regression 1/1;
full Tauri Rust suite 31/31.

Residual hand-off: A8 still has broader SMOKE checklist parity if the project
wants every manual smoke row represented by automated assertions. No lifecycle
state, release readiness, professional approval, certification, sealing,
authentication, or code-compliance claim is created.

## 2026-06-12 - A8 from-blank GUI journey automation (`TP-APP-R2-FROMBLANK-E2E-001`)

The A8 Playwright harness now automates the A12 from-blank rehearsal script
through visible browser GUI controls. The new e2e test reads
`fixtures/product_preview/r2_from_blank_rehearsal.json`, clicks `New blank`,
fills the viewport/property-inspector/load-manager forms for the two nodes,
material, standalone section, pipe run, support, load case, primitive load,
and mechanics combination, and applies all nine operations through the Apply
Operations panel. Each applied receipt is asserted as
`route=local_wasm_engine` with `professional_approval=false`.

The test also records the current browser harness boundary. Browser mode does
not publish solved rows for an edited model; it emits
`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`. Browser mode also keeps the
hash-bound report renderer desktop-only and emits
`REPORT-RENDERER-DESKTOP-ONLY` instead of using a fallback renderer. The true
backend create -> solve -> render proof remains covered by the A12 Tauri
regression; A8 still needs the packaged Tauri saved-project smoke to bind the
GUI-authored project path to the backend desktop command seams.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-12_from_blank_gui_journey.md`;
`apps/desktop/SMOKE.md` TP-MAC-127. Validation: Playwright e2e 2/2 after
wasm engine build; desktop Vitest 213/213; desktop production build green
with the pre-existing Vite chunk-size warning.

Residual hand-offs: packaged Tauri/backend saved-project solve + render smoke
and broader SMOKE checklist parity remain in A8. Phase B2/B3 riders from the
2026-06-12 human findings are routed in the completion plan: preserve
per-constant unit derivation/provenance text, add executable schema-to-crate
unit vocabulary parity, and keep future process records in deliverable-local
evidence folders.

## 2026-06-12 - A12 from-blank R2 exit rehearsal (`TP-APP-R2-FROMBLANK-REHEARSAL-001`)

A12 is landed as a backend-backed rehearsal script and regression for the PRD
R2 exit chain. The new invented fixture
`fixtures/product_preview/r2_from_blank_rehearsal.json` starts from a blank
local model and lists structured authoring steps for two nodes, one material,
one standalone section, one straight pipe run, one anchor support, one load
case, one primitive load, and one mechanics combination. The Tauri regression
`r2_from_blank_rehearsal_authors_solves_and_renders_report` consumes that
fixture, converts each step to a structured operation envelope, applies the
steps through `apply_model_operation`, solves the authored model through
`run_preview_mechanics`, and renders the deterministic A7 HTML report through
`render_calculation_report`.

The rehearsal exposed and closed a real A4-to-A5 integration gap:
operation-authored primitive-load categories were stored as
`concentrated_force`, `concentrated_moment`, and `distributed_force`, while the
product-physics adapter only accepted the older preview labels. The adapter
now maps concentrated force/moment to the existing equivalent-static
`Occasional` mechanics category and distributed force to `Weight`, with direct
product-physics regression coverage.

Evidence:
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-12_from_blank_rehearsal.md`;
`apps/desktop/SMOKE.md` TP-MAC-126. Validation: rehearsal fixture JSON parses;
product_physics cargo tests 24/24; Tauri Rust tests 30/30; desktop Vitest
213/213; desktop production build green; Playwright R2 smoke 1/1 after wasm
engine build.

Residual hand-off: A8 must automate this A12 script as the GUI/e2e journey
evidence backbone. Browser fixture mode still honestly refuses solved rows for
edited models without the Tauri backend, so A12 does not claim browser-mode
edited-model solving. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - A11 final sub-slice: node deletion authoring (`TP-APP-R2-DELNODE-001`)

The structured operation seam now accepts `delete_node` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole-node
deletion intents with no direct mutation, requires `object_type=Node`,
`field_path=nodes`, `after=not_present`, current node label/x/y/z
before-state display, unit `none`, and dimension `dimensionless`. Apply
removes exactly one unreferenced node and returns a new model document.
Deletion is refused while any pipe endpoint, support, component symbol, or
primitive nodal load still references the node, with
`OP-NODE-DELETE-REFERENCED`; there is no hidden cascade into dependent model
entities. The cross-engine contract corpus added
`case_56_accept_delete_node.json` and
`case_57_block_delete_node_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_node` and referenced-node blocking
coverage.

The desktop Property Inspector now exposes a `Queue delete node` action for
the selected node. The action queues a review-only structured operation with
the selected node summary as the before-state guard. Applying the queued
operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, removes
the node row, and falls back to the project row after the deleted selection
disappears.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_node_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-125. Validation: operation_applier cargo
suites 49 unit + canonical hash + 57-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 117/117; focused App Vitest
2/2; full desktop Vitest 213/213; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with node delete preview coverage; local
in-app browser create-then-delete node smoke at `http://127.0.0.1:5173/` with
zero console errors.

A11 is complete: support deletion, primitive-load deletion, full-combination
deletion, load-case deletion, pipe-run deletion, node deletion, and the
previously landed combination-term deletion all route through the structured
operation seam with accepted/refusal corpus coverage. Residual hand-offs:
A12 must rehearse the full from-blank create -> solve -> report path, and the
A8 journey automation should become the R2 exit-evidence backbone immediately
after that rehearsal. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - A11 fifth sub-slice: pipe-run deletion authoring (`TP-APP-R2-DELPIPE-001`)

The structured operation seam now accepts `delete_pipe_run` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole pipe-run
deletion intents with no direct mutation, requires `object_type=Element`,
`field_path=pipe_segments`, `after=not_present`, current pipe
label/from/to/material before-state display, unit `none`, and dimension
`dimensionless`. Apply removes exactly one unreferenced pipe segment and
returns a new model document. Deletion is refused while any primitive load
still references the pipe, with `OP-PIPE-DELETE-REFERENCED`; there is no
hidden primitive-load cascade. The cross-engine contract corpus added
`case_54_accept_delete_pipe_run.json` and
`case_55_block_delete_pipe_run_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_pipe_run` and referenced-pipe blocking
coverage.

The desktop Property Inspector now exposes a `Queue delete pipe` action for
the selected pipe. The action queues a review-only structured operation with
the selected pipe summary as the before-state guard. Applying the queued
operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, removes
the pipe row, and falls back to the project row after the deleted selection
disappears.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_pipe_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-124. Validation: operation_applier cargo
suites 47 unit + canonical hash + 55-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 113/113; focused App Vitest
2/2; full desktop Vitest 207/207; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with pipe delete preview coverage; local
in-app browser pipe-delete smoke at `http://127.0.0.1:5173/` with zero
console errors.

Residual hand-offs: A11 remains open for node entity deletion only. A12 must
still rehearse the full from-blank create -> solve -> report path, and the A8
journey automation should become the R2 exit-evidence backbone immediately
after that rehearsal. No lifecycle state, release readiness, professional
approval, certification, sealing, authentication, or code-compliance claim is
created.

## 2026-06-12 - B1 unit catalog and conversion crate (`TP-UNITS-B1-CATALOG-001`)

Phase B1 is now landed crate-side. `core/units` has its own Rust crate,
`open_pipe_stress_units`, with a closed DEC-018 catalog surface: canonical
dimension identifiers and exponent-vector algebra, SI-canonical units with
common SI/US display units, exact public definitional conversion constants,
finite-value checks, incompatible-dimension rejection, affine absolute
temperature conversion, separate interval-temperature conversion, and explicit
gauge/absolute pressure conversion that requires a caller-supplied pressure
reference with provenance whenever the pressure kind changes.

During fan-in, the generic quantity conversion API was tightened so
temperature and pressure cannot silently bypass their special semantics through
`QuantityKind::UnitBearing`; those dimensions now require explicit
absolute/interval or absolute/relative quantity semantics. The unit README was
also updated from its pre-DEC-018 posture so it records the accepted B1 basis
and preserves the B2/B3 handoffs instead of listing ruled items as still open.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-02_Unit system and dimensional-analysis core contract/_run_records/WORKING_ITEMS_RUN_2026-06-12_unit_catalog_conversion_crate.md`;
`core/units/_run_records/TASK_RUN_2026-06-12_0136.md`. Validation:
`cargo fmt --manifest-path core/units/Cargo.toml --check` passed;
`cargo test --manifest-path core/units/Cargo.toml` passed with 11 unit tests
and 0 doctests; `python3 tests/test_units_schema.py` passed;
`python3 -m pytest tests/test_units_schema.py` passed with 3 tests.

Residual hand-offs: B2 must wire this crate into schema field bindings,
desktop unit entry/display, solver-boundary normalization, reports, imports,
exports, and rule-pack evaluation; B3 must add mixed-unit round-trip tests,
conversion witnesses, rejection coverage, and D-04/DEC-026 tolerance evidence.
Angle/rotation behavior beyond cataloged `rad`/`deg` conversion remains a
future bounded decision/implementation topic. No lifecycle state,
release-readiness, professional approval, certification, sealing,
authentication, protected-content, private-data, or code-compliance claim is
created.

## 2026-06-12 - A11 fourth sub-slice: load-case deletion authoring (`TP-APP-R2-DELLOADCASE-001`)

The structured operation seam now accepts `delete_load_case` as a delete-kind
operation. `core/model_operations/operation_applier` validates whole-load-case
deletion intents with no direct mutation, requires `object_type=Load`,
`field_path=load_cases`, `after=not_present`, current load-case
id/label/kind/status/primitive-count before-state display, unit `none`, and
dimension `dimensionless`. Apply removes exactly one unreferenced load case
and returns a new model document. Deletion is refused while any combination
term still references the load case, with
`OP-LOAD-CASE-DELETE-REFERENCED`; there is no hidden combination-term or
primitive-load cascade. The cross-engine contract corpus added
`case_52_accept_delete_load_case.json` and
`case_53_block_delete_load_case_referenced.json`; both native Rust and
browser wasm lanes require accepted `delete_load_case` and referenced-load
blocking coverage.

The desktop Load Cases manager now exposes a `Queue delete case` action for
the selected load case. The action queues a review-only structured operation
with the selected load-case summary as the before-state guard. Applying the
queued operation uses the existing Apply Operations panel, records the same
local-session acceptance receipt, clears stale solve/report state, and updates
the load-case list/counts without deleting combinations through a hidden
cascade.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_load_case_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-123. Validation: operation_applier cargo
suites 45 unit + canonical hash + 53-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 109/109; focused App Vitest
2/2; full desktop Vitest 201/201; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with load-case delete preview coverage;
local in-app browser create-then-delete load-case smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node and pipe-run entity deletion.
A12 must still rehearse the full from-blank create -> solve -> report path,
and the A8 journey automation should become the R2 exit-evidence backbone
immediately after that rehearsal. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A11 third sub-slice: full-combination deletion authoring (`TP-APP-R2-DELCOMBINATION-001`)

The structured operation seam now accepts `delete_combination` as a
delete-kind operation. `core/model_operations/operation_applier` validates
whole-combination deletion intents with no direct mutation, requires
`field_path=combinations`, `after=not_present`, current combination
id/label/basis/terms before-state display, unit `none`, and dimension
`dimensionless`. Apply removes exactly one combination and returns a new model
document. The cross-engine contract corpus added
`case_51_accept_delete_combination.json`; both native Rust and browser wasm
lanes require accepted `delete_combination` coverage.

The desktop Load Cases manager now exposes a `Queue delete combo` action for
the selected combination. The action queues a review-only structured
operation with the selected combination summary as the before-state guard.
Applying the queued operation uses the existing Apply Operations panel,
records the same local-session acceptance receipt, clears stale solve/report
state, and updates the combination list/counts without deleting load cases,
primitive loads, or combination terms through a hidden cascade.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_combination_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-122. Validation: operation_applier cargo
suites 44 unit + canonical hash + 51-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 105/105; focused App Vitest
1/1; full desktop Vitest 195/195; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1 with whole-combination delete preview
coverage; local in-app browser full-combination delete smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, and load case entity
deletion; `delete_combination_term` remains the existing term-level deletion
operation. A12 must still rehearse the full from-blank create -> solve ->
report path. No lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A11 second sub-slice: primitive-load deletion authoring (`TP-APP-R2-DELPRIMLOAD-001`)

The structured operation seam now accepts `delete_primitive_load` as a
delete-kind operation. `core/model_operations/operation_applier` validates
indexed primitive-load deletion intents with no direct mutation, requires
`field_path=primitive_loads.N`, `after=not_present`, current primitive
before-state display, and matching primitive unit/dimension metadata. Apply
removes exactly one primitive load from the owning load case and returns a
new model document. The cross-engine contract corpus added
`case_50_accept_delete_primitive_load.json`; both native Rust and browser wasm
lanes require accepted `delete_primitive_load` coverage.

The desktop Load Cases manager now exposes a `Queue delete` action for the
selected primitive load. The action queues a review-only structured operation
with the selected primitive id/category/target/direction/magnitude as the
before-state guard. Applying the queued operation uses the existing Apply
Operations panel, records the same local-session acceptance receipt, clears
stale solve/report state, and updates the primitive-load list/counts without
inventing any replacement load.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_primitive_load_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-121. Validation: operation_applier cargo
suites 43 unit + canonical hash + 50-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 103/103; focused App Vitest
1/1; full desktop Vitest 192/192; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1; local Chrome primitive-load delete
smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, load case, and full
combination entity deletion; `delete_combination_term` remains the existing
term-level deletion operation. A12 must still rehearse the full from-blank
create -> solve -> report path. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A11 first sub-slice: support deletion authoring (`TP-APP-R2-DELSUPPORT-001`)

The structured operation seam now accepts `delete_support` as a delete-kind
operation. `core/model_operations/operation_applier` validates explicit
support deletion intents with no direct mutation, requires `after=not_present`,
unit `none`, dimension `dimensionless`, current support label before-state
matching, and reference-integrity blocking before any model document is
returned. Deletion is refused with `OP-SUPPORT-DELETE-REFERENCED` when an
imposed-displacement primitive load still targets the support. The
cross-engine contract corpus added `case_48_accept_delete_support.json` and
`case_49_block_delete_support_referenced.json`; both native Rust and browser
wasm lanes require accepted `delete_support` coverage and the referenced
support blocking code.

The desktop Property Inspector now exposes a `Delete support` action only
when a support row is selected. The action queues a review-only structured
operation, applies through the existing Apply Operations panel, records the
same local-session acceptance receipt, clears stale solve/report state, and
falls back to the project row after the deleted support disappears. A second
app regression creates an imposed-displacement primitive load and verifies
that deleting its target support remains blocked with diagnostics and no
model mutation.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_delete_support_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-120. Validation: operation_applier cargo
suites 42 unit + canonical hash + 49-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 101/101; focused App Vitest
2/2; full desktop Vitest 189/189; desktop production build green; Tauri Rust
tests 29/29; Playwright R2 smoke 1/1; local Chrome support-delete smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 remains open for node, pipe run, load case, primitive
load, and full combination entity deletion; `delete_combination_term` remains
the existing term-level deletion operation. A12 must still rehearse the full
from-blank create -> solve -> report path. No lifecycle state, release
readiness, professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-12 - A10 third sub-slice: section creation authoring (`TP-APP-R2-CREATESECTION-001`)

The structured operation seam now accepts `create_section` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit pipe
section payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, project length-unit alignment for outside
diameter and wall thickness, and a physical geometry guard that requires wall
thickness to be less than the outside-diameter radius. The cross-engine
contract corpus added `case_47_accept_create_section.json`; both native Rust
and browser wasm lanes require `create_section` in the accepted-operation
coverage floor.

The desktop model tree and Property Inspector now expose a local preview
`sections` collection and a compact section creation form: section id, name,
type, outside diameter, wall thickness, provenance, and a `Queue section`
action that emits a review-only structured operation. Applying the queued
intent uses the same operation-apply panel, records the local-session
acceptance receipt, selects the newly created section in the model tree, and
clears stale solve/report state like other applied model edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_section_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-119. Validation: operation_applier cargo
suites 40 unit + canonical hash + 47-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 97/97; focused App Vitest 1/1;
blank-project service Vitest 5/5; full desktop Vitest 183/183; desktop
production build green; Tauri Rust tests 29/29; Playwright R2 smoke 1/1;
local Chrome smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A11 deletion coverage remains open, and A12 must still
rehearse the full from-blank create -> solve -> report path. Optional later
pipe-form reuse of standalone section refs is not claimed by this tranche.
No lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 - A10 second sub-slice: material creation authoring (`TP-APP-R2-CREATEMATERIAL-001`)

The structured operation seam now accepts `create_material` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit
material payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, project pressure-unit alignment for
elastic/shear quantities, and optional thermal-expansion quantity validation
against the project temperature unit. The cross-engine contract corpus added
`case_46_accept_create_material.json`; both native Rust and browser wasm
lanes require `create_material` in the accepted-operation coverage floor.

The desktop Property Inspector now exposes a compact material creation form:
material id, label, elastic modulus, shear modulus, optional thermal
expansion, provenance, and a `Queue material` action that emits a review-only
structured operation. Applying the queued intent uses the same operation-apply
panel, records the local-session acceptance receipt, selects the newly created
material in the model tree, and clears stale solve/report state like other
applied model edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_material_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-118. Validation: operation_applier cargo
suites 38 unit + canonical hash + 46-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 95/95; focused App Vitest 1/1;
full desktop Vitest 180/180; desktop production build green; Tauri Rust tests
29/29; Playwright R2 smoke 1/1; local Chrome smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 still needs section creation before pipe creation can
be fully fixture-independent; A11 deletion coverage is still open. No
lifecycle state, release readiness, professional approval, certification,
sealing, authentication, or code-compliance claim is created.

## 2026-06-12 — A10 first sub-slice: support creation authoring (`TP-APP-R2-CREATESUPPORT-001`)

The structured operation seam now accepts `create_support` as a create-kind
operation. `core/model_operations/operation_applier` validates explicit
support payloads with no direct mutation, no engineering defaults, no unit
conversion, duplicate-id blocking, node-reference blocking, and the existing
UX/UY/UZ/RX/RY/RZ restraint-token vocabulary. The cross-engine contract
corpus added `case_45_accept_create_support.json`; both native Rust and
browser wasm lanes require `create_support` in the accepted-operation coverage
floor.

The desktop Property Inspector now exposes a compact support creation form:
support id, label, existing node selector, restraint checkboxes, provenance,
and a `Queue support` action that emits a review-only structured operation.
Applying the queued intent uses the same operation-apply panel, records the
local-session acceptance receipt, selects the newly created support in the
model tree, and clears stale solve/report state like other applied model
edits.

Evidence:
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-03_User acceptance and operation audit trail/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-12_create_support_authoring.md`;
`apps/desktop/SMOKE.md` TP-MAC-117. Validation: operation_applier cargo
suites 36 unit + canonical hash + 45-case corpus; corpus bless and rerun
green; desktop operationContractCorpus Vitest 93/93; focused App Vitest 1/1;
full desktop Vitest 177/177; desktop production build green; Tauri Rust tests
29/29; Playwright R2 smoke 1/1; local Chrome smoke at
`http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 still needs material creation and section creation
before pipe creation can be fully fixture-independent; A11 deletion coverage
is still open. No lifecycle state, release readiness, professional approval,
certification, sealing, authentication, or code-compliance claim is created.

## 2026-06-12 — A9 landed: blank local model authoring target (`TP-APP-R2-BLANK-001`)

The desktop app now has an explicit `New blank` local project action for the
R2 from-scratch path. The action builds a user-created blank model document
(`project:blank-local-*`) with no fixture nodes, pipe segments, supports,
materials, load cases, combinations, or mechanics results; persists it through
the existing browser-memory/Tauri local project boundary; makes it the active
authoring target; resets queued operations, undo/redo, proposals, result
state, and analysis-run state; and records `create_blank` in the storage and
validation evidence panels. The blank document carries visible
`MODEL_INCOMPLETE`, `RULE_INPUTS_INCOMPLETE`, and `NOT_PROVIDED` status plus
a blocking `BLANK_PROJECT_AUTHORING_TARGET` diagnostic, so no hidden fixture
entities or engineering defaults are inserted.

Backend evidence pins the Tauri solve path: running `run_preview_mechanics`
on a blank supplied payload returns `MODEL_INCOMPLETE`, zero result rows, and
the explicit missing-input diagnostics `NODE_INPUT_MISSING`,
`PIPE_INPUT_MISSING`, and `LOAD_INPUT_MISSING`. Browser preview mode preserves
the existing edited/user-created-model boundary: a blank solve attempt
completes as a zero-row `MODEL_INCOMPLETE` envelope with
`BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL`, and the report packet shows
0 selected result refs instead of reusing solved fixture rows.

Evidence:
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-12_blank_project_authoring_path.md`;
`apps/desktop/SMOKE.md` TP-MAC-116. Validation: projectService focused
Vitest 5/5; App focused Vitest 32/32; full desktop Vitest 174/174; Tauri Rust
tests 29/29; production build green; Playwright R2 smoke 1/1; live in-app
browser smoke at `http://127.0.0.1:5173/` with zero console errors.

Residual hand-offs: A10 must add support/material/section creation operation
kinds and UI forms before a blank model can become solvable from scratch;
A11 must add deletion coverage; packaged-Tauri saved-project blank smoke
remains as an A8/A5 evidence expansion. No lifecycle state, release readiness,
professional approval, certification, sealing, authentication, or
code-compliance claim is created.

## 2026-06-11 — H5: RFC 8785 canonical number rendering in `canonical_json` (`TP-H5-JCSRENDER-001`)

The engine's canonical JSON is true RFC 8785 (JCS), closing the DEC-010
"JCS-compatible hashing" alignment gap H1 measured. New shared crate
`core/serialization/canonical_json`: ECMAScript `Number::toString` number
rendering with ryu digit selection (std's formatter breaks exact
shortest-representation ties differently from ECMAScript — measured, 7 in
20k random doubles; ryu matched node on 112,220 fuzz vectors), UTF-16
code-unit key sort, `JSON.stringify`-identical escaping; one documented
divergence (raw-text integers beyond 2^53 keep exact i64/u64 precision —
outside the I-JSON envelope, unreachable via JS transport).
`operation_applier::canonical_json` re-exports it;
`BACKEND_CANONICALIZATION` and every engine-hash label across the
frontend, schema enum, and headless runner unified to `rfc8785_jcs`.

Both `TP-H1-HASHUNIFY-001` refutations are resolved: JS transport no
longer shifts canonical bytes, so the corpus ECMA harness renderer twins
and their number-range constraint are deleted, and backend hash VALUES
(`backend_model_hash`, `applied_model_backend_hash`) joined the
full-byte-equality compared surface in every lane. `fixtures/canonical_hash/`
extended 14 → 20 cases (notation boundaries, tie-to-even, −0, beyond-2^53,
UTF-16 sort; floor 12 → 20) and all 44 contract-corpus cases re-blessed.
Headless runner checksums render through the shared crate (TASK subscope;
its previously aspirational `"JCS"` label is now true); audit_manifest
checked — caller-supplied number strings, doc pointer added, no behavior
change. Residual hand-offs (vocabulary-only label sites in
`primitive_loads`/`result_export`/fixtures; Python persistence truth-label
wording; possible future claimed-vs-backend equality evaluation) recorded
in the run record.

Evidence: `execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_run_records/WORKING_ITEMS_RUN_2026-06-11_h5_rfc8785_rendering.md`
(TASK sub-record in DEL-10-05 `_run_records/`); suites: canonical_json 8,
applier 34+20+44×2, Vitest 172/172, headless 11, audit_manifest 13,
src-tauri 28, pytest 353; DEC-025 sweep at the closing commit.

## 2026-06-11 — H1: canonical-hash unification through the wasm engine (`TP-H1-HASHUNIFY-001`)

Frontend hashing is single-sourced: `hashService.ts` now calls the wasm
exports of the engine's `canonical_json`/`sha256_hex` (new
`canonical_json_string` / `canonical_sha256_hex` in `wasm_api`, throwing
the named input diagnostic on malformed JSON), and the TS canonicalization
plus the A7 report-input WebCrypto helper (with its silent `"TBD"` soft
fallback) are deleted — no fallback hashing path exists in the frontend. A
new Rust-blessed parity corpus (`fixtures/canonical_hash/`, 14 cases,
12-case floor) feeds identical raw JSON text to both lanes and pins the
engine's canonical form and number normalization.

The allowlist-tightening rider was executed as a measurement and the
hypothesis from the T4 caveat was **partially refuted with evidence**:
`backend_canonicalization` (invariant label) joined the blessed corpus
projection across all 44 re-blessed cases, and the native runner now
asserts engine self-consistency (`applied_model_backend_hash` =
`canonical_json`+`sha256_hex` of the returned document); but the
`backend_model_hash` value exclusions stand, for measured transport
reasons now documented in the corpus README — `JSON.stringify` renders
`200.0` as `200` so input-text hashes legitimately differ across lanes,
and the engine's applied-model hash covers in-process text reproducible
only Rust-side (recorded consequence: receipt-hash verification must
re-ask the engine; no frontend recomputation is possible). The corpus's
ECMA harness renderer survives in both runners as explicitly harness-only
code with this rationale attached.

Evidence: operation_applier cargo suites green; desktop Vitest 166/166
(was 140); DEC-025 five-surface sweep overall-pass. Engine slice landed as
`6e36f5da4` mid-tranche (collision defense during the shared-worktree
incident); the adapter/corpus slice closes with this entry. Run record:
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-02_Audit manifest and model hash/_run_records/WORKING_ITEMS_RUN_2026-06-11_h1_hash_unification.md`;
SMOKE `TP-MAC-114`.

## ## 2026-06-11 — C1 landed: frozen expression grammar v1.0.0 + conformance corpus (`TP-C1-GRAMMAR-001`)

Implementation of the D-02 ruling (`DEC-022`, packet Option A) by a bounded
TASK worker in `core/rules`: the typed expression AST extended to the full
PRD §12.3 set (And/Or/Not, eager Select with blocking unselected-branch
diagnostics, n-ary Min/Max and Abs over same-dimension same-unit
quantities, piecewise-linear Interpolate and exact/step Lookup over
strictly-monotone user tables with blocking out-of-range — no
extrapolation or clamping); the two in-code dimensional TBDs resolved via
an enumerated commutative 18-relation dimension-product table with
ambiguous/unrepresentable results blocking; `GRAMMAR_VERSION = "1.0.0"`
declared-version gate; `grammar_version` bound inside the JCS-hashed
rule-pack checksum (lifecycle record field, blocking findings,
byte-containment evidence check, binding-enforcing constructor). New
blessed conformance corpus `fixtures/rule_expressions/` (69 golden cases +
checksum-binding golden hashes, all synthetic) executed as crate tests
with a runner-enforced coverage floor. Evaluator 31+1 corpus test (69
cases), lifecycle 12, completeness_checker 12, pytest 353 — all green;
zero new dependencies. Residuals and three labeled ASSUMPTIONs (dimension
table contents, derived unit-ref convention, Select branch-compatibility
rule) flagged for human review in the run record.

Evidence:
`execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_run_records/TASK_RUN_2026-06-11_TP-C1-GRAMMAR-001.md`.

## 2026-06-11 — D7 first slice landed: in-repo sparse skyline direct solver (`TP-D03-SPARSE-001`)

Implementation of the D-03 ruling (`DEC-023`, packet Option C) by a bounded
TASK worker in `core/solver`: new `sparse_direct` crate with deterministic
reverse Cuthill–McKee ordering (all ties broken by ascending
degree/original-index, permutations pinned by test), skyline profile
storage of the symmetric stiffness, in-repo LDLᵀ profile factorization and
solve, single-threaded fixed operation order, zero new dependencies;
diagnostics crate gains NonPositivePivot and sparse-error/factorization
mappings with deterministic ordering; performance harness measures the
sparse path alongside dense (profile/bandwidth reduction, pivot extrema,
parity delta, residual, repeat determinism) with no thresholds asserted
(D-04 governs). Parity vs dense at the DEC-026 analytic seed (1.0e-9
relative) on chain/grid models up to 360 reduced DOFs plus a dyadic-exact
hand-checked case. sparse_direct 18, diagnostics 24, performance_harness
18, frame_kernel (untouched) 34 — all green. Remaining D7 scope stays in
the row: live solve-path adoption (integration plan in the run record) and
profile-direct assembly.

Evidence:
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-01_3D frame stiffness kernel/_run_records/TASK_RUN_2026-06-11_TP-D03-SPARSE-001.md`.

## 2026-06-11 — A7 landed: rendered FR-016 calculation report (`TP-APP-R2-REPORTRENDER-001`)

Implementation of the D-10 ruling (`DEC-021`, packet Option B): new
`core/reporting/report_renderer` crate renders the validated
`CalculationReport` + `ReportSections` + caller result rows into a
deterministic, self-contained, scriptless single-file HTML document;
SHA-256 of the bytes is the canonical hash-bound evidence; three-point
protected-content lint gating (template surface, pre-render section text,
post-render document text) plus blocking validation diagnostics refuse
export with a visible banner; `derived_print_view` emits the labeled
non-hash-bound print/PDF view naming the canonical hash. Feature-gated
serde derives on report_generator/report_sections pin the schema-contract
spellings, proven against the canonical fixture. Desktop seam: 15th Tauri
command `render_calculation_report`, render service with an explicit
desktop-only browser route (no fallback), session-envelope adapter with
explicit TBD markers, and a Rendered Report panel whose save/print actions
are refused while export is blocked. Residuals in the A7 row: browser-mode
render seam (A8 decision), rule-pack refs population (Phase C),
report-hash persistence follow-up.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-113;
`execution/PKG-08_Reporting, Audit, and Reproducibility/1_Working/DEL-08-01_Calculation report generator/_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-APP-R2-REPORTRENDER-001.md`;
renderer crate 8/8, Vitest 148/148, src-tauri command test, production
build green; commit-bound sweep summary at the tranche push.

## 2026-06-11 — DEC-025 evidence sweep + F-4 atomic wasm build landed (`TP-SWEEP-001`)

Implementation of the D-05 ruling (`DEC-025`, packet Option D): new
deterministic entrypoint `tools/release/run_evidence_sweep.py` runs the five
evidence surfaces sequentially in F-4-safe order (cargo crate sweep, pytest,
desktop Vitest with the wasm engine built first, Playwright e2e, desktop
production build), fail-fast with `not_run` accounting, and writes a
commit-bound machine-readable summary to `validation/evidence/sweeps/`. The
sweep is the required pre-push/fan-in merge gate for parallel agent branches
(`docs/BUILD_AND_RELEASE.md` §5.1; gate pattern: commit → sweep at clean
HEAD → evidence-only closeout commit → push). F-4 rider: the wasm build now
writes glue to a sibling staging dir and renames it into place; staging dirs
gitignored. Docs: BUILD_AND_RELEASE §2/§3/§5.1/§7/§9 and
RELEASE_QUALITY_GATES §10 record the ruling; the CI-provider TBD closes with
the `D-05b` follow-up pointer. 11 focused tests in
`tests/test_evidence_sweep.py`.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-112; validation-run summary
`validation/evidence/sweeps/SWEEP_20260612T031241Z_0f402fc48424-dirty.json`
(all five surfaces pass, exit 0) plus the clean gate-run summary committed
at closeout;
`execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-04_Build, packaging, and CI-CD pipeline/_run_records/WORKING_ITEMS_RUN_2026-06-11_TP-SWEEP-001_evidence_sweep.md`.

## 2026-06-11 — Seam-verification regression repair: F-1 Playwright budget, F-2 NUL bytes (`TP-SEAM-FIX-001`)

Human-instructed repair tranche for the open findings in the independent
post-closure verification
([VERIFICATION_2026-06-11_operation_seam_unification.md](VERIFICATION_2026-06-11_operation_seam_unification.md)).
F-1: the default `test:e2e` failed at the 30s per-test budget; measurement
showed the ~10× runtime growth is the Playwright trace recorder
(`retain-on-failure` records every run and discards on pass — 8.8s trace
off vs 21.6s warm / 37.4s cold with trace on, ~21s of it in trace
finalization at `browser.close`), not the wasm engine (engine-ready wait
<1s, artifact ~330 kB); the T4 apply-flow steps run against the heaviest
DOM state, multiplying per-action snapshot cost. Fixed by raising the
budget to 120s with the measurements recorded in the config comment;
tracing kept; spec not split. F-2: the two literal NUL separators in
`operationContractCorpus.test.ts` replaced with string escapes, making the
file text again for git diff review. F-3 (corpus fixture review
disposition) remains with the human; F-4 stands as an operational note for
`D-05`. Full evidence set re-run sequentially: e2e 1/1 at 18.8s under the
default command, Vitest 140/140, cargo sweep exit 0, pytest 342/342,
production build green. Run record:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_verification_regression_repair.md`;
SMOKE entry `TP-MAC-111`.

## 2026-06-11 — Operation-seam plan T4 and plan closure: wasm route swap, TS engine deleted (`TP-SEAM-SWAP-001`)

The structured editor-operation seam is now implemented exactly once. In
every environment — packaged Tauri, Vite dev browser, jsdom Vitest,
Playwright Chrome — validation and apply outcomes are produced by
`core/model_operations/operation_applier` (native via the Tauri commands,
wasm32 otherwise). The TypeScript engine and its private semantic tables
(field rules, canonical dimension set, restraint tokens, deferred fields,
collections) no longer exist in the tree.

`operationService.ts` shrank from a 2,280-line engine to a 112-line thin
routing adapter (Tauri `invoke` else wasm; wasm input-error envelope
surfaces as an explicit thrown error; honest engine-status reporting). Its
17 engine tests (911 lines) are superseded by the contract corpus + 6
adapter tests; net deletion −3,114/+200 lines. Browser receipts now read
`route=local_wasm_engine`; the apply panel reports engine readiness via
`operation-engine-status` with the named `WASM-ENGINE-ASSET-ABSENT`
diagnostic and exact build command when the artifact is missing. The corpus
TS lane retired with the engine: the Vitest runner keeps two lanes (public
adapter seam, wasm engine direct) against the Rust-blessed reference as the
permanent native↔wasm parity + regression surface. Vitest setup pre-warms
the engine; `test:e2e` builds the artifact first; the Playwright spec waits
on engine-ready and ends with a real-browser apply asserting the wasm route
receipt.

Evidence: cargo profile sweep exit 0 (25 manifests, zero failures); pytest
342/342; Vitest 140/140; Playwright 1/1; production build green (index
chunk at the ~577 kB baseline — the engine loads dynamically).
`apps/desktop/SMOKE.md` TP-MAC-110; primary run record at DEL-16-02
`_run_records/WORKING_ITEMS_RUN_2026-06-11_t4_wasm_engine_swap.md` (fan-out
records at DEL-16-03, DEL-07-02, DEL-00-08).

**Plan closure.** With T4 landed, all §4 exit criteria of
[PLAN_2026-06-11_operation_seam_unification.md](PLAN_2026-06-11_operation_seam_unification.md)
are met: one engine; corpus executing in both `cargo test` and `npm test`
with hash parity; D-13 RULED → `DEC-020` with ADR-0001 accepted on the
stood-up ADR surface; the `LocalFeaHandoffPanel` fallback replaced; all
test surfaces green with evidence recorded. The §5 freeze rule is lifted;
the plan's Active Surface pointer in `_COORDINATION.md` was removed per the
acceptance authorization. Deferred items (plan §9) — including the R2
from-scratch authoring set and Phase B B2, both deliberately sequenced
after this unification — roll forward to completion-plan selection, which
now resumes.

## 2026-06-11 — Operation-seam plan T3: wasm build enablement (`TP-SEAM-WASM-001`)

The wasm32 build of `core/model_operations/operation_applier` is now a
working browser-loadable engine with proven parity. Landed across two
sessions (scaffolding committed as `wip(seam-T3)` at handoff, then resumed):
feature-gated `wasm` wasm-bindgen JSON-string exports
(`validate_operation_json` / `apply_operation_json`; native build and suite
unaffected); `apps/desktop/scripts/build-wasm-engine.mjs` +
`build:wasm` / `build:wasm:desktop` npm scripts (prerequisite checks with
exact remediation commands — wasm32 target, wasm-bindgen CLI pinned to the
crate's =0.2.123 — and gitignored `__generated__/` output, never committed);
a loading shim whose absent-artifact failure is the named diagnostic
`WASM-ENGINE-ASSET-ABSENT` with the build command (no silent fallback, no
fallback engine); and the corpus wasm lane in
`operationContractCorpus.test.ts`.

The plan §3 first-task spike — the 44-case corpus through the wasm engine
under Vitest/jsdom — succeeded; the §6 stop rule did not fire and no TS
fallback was written. Two environment-fit fixes only, both artifact
location/typing (engine semantics untouched): Node-branch path probing for
the wasm bytes (Vite's test transform rewrites `import.meta.url` to a
root-relative file URL, breaking module-relative resolution; loud failure
now lists every probed path) and a `Uint8Array<ArrayBuffer>` copy for
`BufferSource` typing (also fixed a latent `tsc -b` failure in the WIP
shim, caught by this tranche's production-build evidence).

Result: three-way native↔wasm↔TS parity — identical semantic outcomes and
identical corpus-harness canonical sha256 hashes — so hash evidence does not
fork by environment. Wasm-lane extras: honest `local_wasm_engine` route
assertion; structured input-error envelope
(`WASM-ENGINE-INPUT-JSON-INVALID`) instead of a trap.
`docs/BUILD_AND_RELEASE.md` §3 records the engine identity, toolchain
prerequisites, and build step.

Evidence: `cargo test` 36/36 (native unchanged); `npm run build:wasm:desktop`
OK; `npm test --workspace apps/desktop` 151/151 (46 wasm-lane); desktop
production build green (index chunk at the ~577 kB baseline — the engine is
dynamically loaded, not bundled). Primary record:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-02_Repository and module boundary architecture/_run_records/WORKING_ITEMS_RUN_2026-06-11_t3_wasm_enablement_completion.md`
(WIP handoff record superseded in the same folder; fan-out records at
DEL-00-08 and DEL-16-02).

## 2026-06-11 — Operation-seam plan T1: cross-engine contract corpus + handoff rider (`TP-SEAM-CORPUS-001`)

The editor-operation seam now has an executable cross-engine contract: 44
invented cases at `fixtures/model_operations/contract_corpus/` run inside
both engines' normal suites — a Rust integration test in the
`operation_applier` crate (the contract reference, with a `CORPUS_BLESS=1`
regeneration mode) and a Vitest spec driving `operationService` browser mode
— asserting identical semantic outcomes (validation states, diff-preview
rows, order-insensitive diagnostic records), parsed-JSON deep equality of
applied documents, and one shared corpus-harness canonical sha256 per
applied document. Engine-identity fields are excluded only via a documented
per-field allowlist. The coverage floor is enforced programmatically in both
runners: all 10 operation kinds accepted, the named block classes (19
distinct blocking codes), and the dynamic paths
`primitive_loads.N.magnitude.value` / `terms.N.factor` accepted and blocked.
Notable finding: the TypeScript mirror reproduced all 44 Rust-reference
outcomes on first execution — zero alignment fixes, zero quarantined cases —
which retroactively strengthens all existing browser-based evidence.

The boundary-hygiene rider removed the `LocalFeaHandoffPanel` fixture-ID
fallback (`pipe:P-120` / `node:N-140`): missing result-summary refs now emit
the explicit `LOCAL-FEA-RESULT-SUMMARY-REF-MISSING` finding with an honestly
degraded region id and no invented references (two new panel tests).

Execution note: the bounded TASK worker for this tranche was interrupted by
an environment connection loss after authoring its artifacts; WORKING_ITEMS
independently re-verified everything (cargo 36/36; Vitest 105/105;
Playwright 1/1 unchanged; desktop production build green) and completed the
closeout. Residual: corpus README `review_status` pending human review.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-109 and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-11_seam_contract_corpus_fanin.md`
(interrupted worker record `TASK_RUN_2026-06-11_1414.md` finalized FAILED in
the same folder; fan-out records at DEL-16-03 and DEL-00-08; rider record at
DEL-10-03).

## 2026-06-11 — Operation-seam plan T2: ADR surface + D-13 ruling transcription (`TP-SEAM-DECISION-001`)

The ADR surface anticipated by DEL-00-01 now exists: `docs/architecture/adr/`
with `template.md`, `index.md`, and
`ADR-0001_operation_seam_engine_unification.md` (status accepted, dated
2026-06-11) recording the D-13 ruling — wasm32 `operation_applier` as the
sole browser-mode engine, the TS mirror's validation/apply logic retired at
plan T4, four alternatives rejected with reasons, consequences, and
reconsideration triggers. The pre-write gate check confirmed the surface was
unbuilt by explicit deferral (PKG-00 lock review), not resolved elsewhere.
Transcription obligations completed: `DEC-020` appended to
`SOFTWARE_DECOMP.md` §12 and the D-13 row appended to
`_DECISIONS/_REGISTER.md` directly in state RULED, with the accepted plan
itself as the packet. Cross-consistency of ADR ↔ DEC-020 ↔ D-13 ↔ plan §2
verified; no code changes.

Evidence:
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-01_Architecture decision record baseline/_run_records/TASK_RUN_2026-06-11_1408.md`.

## 2026-06-11 — A6 second sub-slice: results family selector (`TP-APP-R2-RESULTFAMILY-001`)

The results panel now exposes direct result-family filters with visible counts
for the solved preview envelope: displacement, reaction, force, moment, and
stress. Selecting a family resets pagination and constrains the existing
table groups, while the existing free-text filter still composes with the
selected family. The support-reaction rows and stress rows are therefore
selectable as dedicated table views without requiring text-search strings.

The fixture-backed app test asserts the family counts
(`15/9/180/180/263`), reaction-only pagination (`9 of 647`), stress-only
pagination (`263 of 647`), active `aria-pressed` state, and representative
reaction/stress rows. The Playwright R2 smoke covers the reaction family path
after solving. In-app browser smoke at `http://127.0.0.1:5175/` confirmed the
solved app showed `result_rows=647`, reaction/stress family selection,
visible non-overflowing selector buttons, all-family restoration, and zero
browser console errors.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, governing-ratio views once ratio rows exist in result
envelopes, and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-108 and
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-05_Results viewer/_run_records/WORKING_ITEMS_RUN_2026-06-11_results_family_selector.md`.

## 2026-06-11 — A5 second sub-slice: persisted edited-load solve regression (`TP-APP-R2-PERSISTEDSOLVE-001`)

The Tauri backend test suite now proves an edited load-data model can move
through the app's structured operation and local persistence seams before
being solved from the restored model payload. The regression applies an
explicit `update_load` operation to `load:L-100` primitive
`primitive_loads.1.magnitude.value`, persists the edited model into the local
SQLite project store, reloads it by project id, and solves the restored
payload through `solve_preview_mechanics`.

The test asserts the saved/reopened model retains the edited force magnitude,
the restored solve is bound to `project:edited-load-roundtrip`, mechanics
status is `MECHANICS_SOLVED`, and `result:disp:node-N-140` changes relative
to the original fixture solve. This is backend evidence for saved edited-load
data reaching the solve boundary; it does not claim that the later packaged
Tauri GUI smoke is complete.

Residuals remain in A5: full packaged-Tauri GUI smoke over a saved edited
project snapshot, UI polish for incomplete-model diagnostics, and broader
persisted solve coverage as new authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-107;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-11_persisted_edited_load_solve_regression.md`,
the same-named record under
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/`,
and the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`.

## 2026-06-11 — A4 thirteenth sub-slice: combination creation editor (`TP-APP-R2-COMBCREATE-001`)

The Load Cases manager now exposes explicit creation for mechanics-basis load
combinations. The create form captures a new combination id, label, mechanics
basis, one existing load-case term, a finite dimensionless factor, provenance,
and rationale, then queues a structured `create_combination` operation with
`field_path=combinations`, `before=not_present`, unit `none`, and dimension
`dimensionless`.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination` payloads. Accepted intents must create a new
`Combination`, use basis `mechanics`, include at least one explicit term,
reference existing load cases, carry finite factors, avoid duplicate initial
operands, preserve structured-operation audit boundaries, and emit no
professional approval claim. Duplicate ids, missing load cases, empty terms,
invalid payloads, non-mechanics basis values, and invalid unit/dimension
metadata are blocked.

The app-level regression queues `op:load-manager-create-combination:C-300`,
applies it through `OperationApplyPanel`, verifies the manager summary changes
to two combinations, checks the new `combination:C-300` row with
`load:L-100 x 1`, selects the row into the property inspector, and confirms
zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke verifies the rendered create-combination preview without
applying so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-combination:C-300` and confirmed two combinations,
`combination:C-300` with `basis=mechanics` and `load:L-100 x 1`, property
inspector selection, zero pending operations, `applied_operations=1`, solve
state `not_started`, and no browser console errors.

Residuals remain in A4: subtraction/range expression authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-106;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 twelfth sub-slice: combination term deletion editor (`TP-APP-R2-COMBTERMDELETE-001`)

The Load Cases manager now exposes explicit deletion for a selected existing
combination term. The delete control appears in the selected-term editor,
captures a rationale, and queues a structured `delete_combination_term`
operation with `field_path=terms.N`, `before=<load_case> x <factor>`,
`after=not_present`, unit `none`, and dimension `dimensionless`. The operation
removes one indexed term only; whole-term replacement, code/rule combinations,
and broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`delete_combination_term` payloads. Accepted intents must target an existing
`Combination`, address an existing term index, carry a before-value matching
the current term display, preserve structured-operation audit boundaries, and
emit no professional approval claim. Stale before-values, out-of-range
indices, invalid unit/dimension metadata, and missing combination `terms`
arrays are blocked.

The app-level regression selects `combination:C-OPER-ALT` term 1, queues
`op:load-manager-combination:C-OPER-ALT-term-1-delete`, applies it through
`OperationApplyPanel`, and verifies `load:L-200 x 0.5` is removed while
`load:L-100 x 1` remains. The Playwright R2 smoke verifies the rendered delete
preview without applying so the solve/results/report path remains on the
unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-delete` and confirmed
`load:L-200 x 0.5` was no longer visible in the combination row, zero pending
operations, `applied_operations=1`, solve state `not_started`, and no browser
console errors.

Residuals remain in A4: broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-105;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_deletion_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eleventh sub-slice: combination term creation editor (`TP-APP-R2-COMBTERMCREATE-001`)

The Load Cases manager now exposes explicit child-term creation for existing
load combinations. The create form selects an existing `Combination`, an
existing `LoadCase`, a finite dimensionless factor, and a rationale, then
queues a structured `create_combination_term` operation with
`field_path=terms`, `before=not_present`, unit `none`, and dimension
`dimensionless`. The operation appends one term only; whole-term replacement,
term deletion, code/rule combinations, and broader algebra authoring remain
deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`create_combination_term` payloads. Accepted intents must target an existing
`Combination`, reference an existing `LoadCase`, carry JSON
`{ "load_case": string, "factor": number }`, preserve structured-operation
audit boundaries, and emit no professional approval claim. Missing load cases,
invalid payloads, non-dimensionless metadata, and missing combination `terms`
arrays are blocked.

The app-level regression creates `load:L-300`, then appends it to
`combination:C-OPER-ALT` as `load:L-300 x 0.25` through
`OperationApplyPanel`, proving existing `load:L-100 x 1` and
`load:L-200 x 0.5` terms are preserved. The Playwright R2 smoke verifies the
rendered create-term preview without applying so the solve/results/report path
remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied the default
create-load-case intent, selected `load:L-300` in the combination-term form,
applied `op:load-manager-combination:C-OPER-ALT-term-2-create`, and confirmed
`load:L-300 x 1`, zero pending operations, `applied_operations=2`, solve
state `not_started`, and no browser console errors. The smoke avoided text
entry because the Browser plugin's virtual clipboard remained unavailable for
`fill`.

Residuals remain in A4: combination term deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-104;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 tenth sub-slice: combination basis editor (`TP-APP-R2-COMBBASIS-001`)

The Load Cases manager now exposes explicit basis editing for existing load
combinations. The selected-combination editor captures a replacement
`Combination.basis` text value and rationale, then queues a structured
`update_load` operation with `field_path=basis`, unit `none`, and dimension
`dimensionless`. The operation is limited to an existing combination record;
whole-term replacement, term creation/deletion, code/rule combinations, and
broader algebra authoring remain deferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now treat
`Combination.basis` as an editable text field while retaining the explicit
deferred finding for whole `Combination.terms` edits. Accepted basis edits
must target an existing `Combination`, carry a current before-value, provide a
non-empty replacement value, and route through structured operations only. The
tests prove the edit changes only `basis` and preserves `terms` and
`provenance`.

The app-level test selects `combination:C-OPER-ALT`, changes basis from
`mechanics` to `mechanics_user_review`, queues
`op:load-manager-combination:C-OPER-ALT-basis`, applies it through
`OperationApplyPanel`, verifies the manager row and property inspector, and
confirms zero pending operations with solve state reset to `not_started`. The
Playwright R2 smoke checks the rendered basis preview without applying so the
solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` keyed
`mechanics_user_review` into the basis field, applied
`op:load-manager-combination:C-OPER-ALT-basis`, and confirmed
`basis=mechanics_user_review`, zero pending operations,
`applied_operations=1`, and solve state `not_started`. The smoke used
single-key presses because the Browser plugin's virtual clipboard remained
unavailable for `fill`/`type`.

Residuals remain in A4: combination term creation/deletion, broader algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-103;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_basis_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 ninth sub-slice: imposed-displacement primitive-load creation editor (`TP-APP-R2-IMPOSED-001`)

The Load Cases manager now exposes explicit imposed-displacement primitive
load creation for existing load cases. The create form selects
`imposed_displacement`, captures the target load case, primitive-load id,
existing support target, support DOF, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. Translational DOFs
`UX|UY|UZ` use the project length unit and dimension `displacement`.
Rotational DOFs `RX|RY|RZ` use the project angle unit and dimension
`rotation`. The invented preview model now records `project.units.angle =
"rad"` so rotational imposed-displacement previews and validation consume
explicit project metadata.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`imposed_displacement` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, concentrated-moment, pressure,
and thermal paths. Accepted imposed-displacement intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`, a
globally unique primitive-load id, category `imposed_displacement`, target
`{ type: "support", support: <existing support id>, dof: <matching DOF> }`,
finite magnitude in the expected project unit, matching dimension, and
non-empty provenance. Duplicate primitive IDs, missing support targets,
invalid DOFs, target/DOF mismatches, and missing project unit metadata are
blocked.

The app-level test creates `load:L-100-I300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new support-targeted row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`,
confirms the local receipt, and confirms stale solve state is reset. The
operation-service and Rust tests also apply a rotational `RX` payload with
dimension `rotation` and unit `rad`. The Playwright R2 smoke checks both the
translational preview and the rotational `RX` unit/dimension preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-I300-primitive` from a clean session
using the default finite magnitude `250 m`. The smoke confirmed `2 load
cases; 8 primitive loads; 1 combinations`, manager row
`load:L-100-I300; support:support:S-100; UZ; dimension=displacement`, zero
pending operations, `persistence=session_state_only_not_yet_saved`,
`professional_approval=false`, and solve state `not_started`. The browser
plugin could not type a replacement magnitude because its virtual clipboard
was unavailable; the negative translational magnitude path is covered by
Vitest.

Residuals remain in A4: combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-102;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_imposed_displacement_load_creation_editor.md`,
the same-named record under
`DEL-04-03_Linear support and restraint models/_run_records/`, the
same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 eighth sub-slice: pressure and thermal primitive-load creation editor (`TP-APP-R2-PRESSTEMP-001`)

The Load Cases manager now exposes explicit pressure and thermal primitive
load creation for existing load cases. The create form selects `pressure` or
`thermal`, captures the target load case, primitive-load id, existing pipe
target, global direction, magnitude, and provenance, then queues a structured
`create_primitive_load` operation. Pressure payloads are limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension `pressure`, and
the project pressure unit (`Pa` for the invented preview model). Thermal
payloads use the same element target contract with dimension
`temperature_interval` and the project temperature interval unit (`degC` for
the invented preview model). No gauge/absolute pressure conversion,
reference-pressure default, thermal absolute-temperature conversion, imposed
displacement, hidden default, unit conversion, or code-specific combination is
inferred.

The invented preview model now records `project.units.pressure = "Pa"` so
pressure creation consumes explicit project unit metadata rather than an
implicit pressure fallback. This remains a single-unit technical-preview
posture: D-01 has accepted future unit semantics, but Phase B unit conversion,
quantity-kind UI, reference-pressure handling, and unit picker/display
retirement remain outside this tranche.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`pressure` and `thermal` primitive-load creation alongside the previously
landed concentrated-force, distributed-force, and concentrated-moment paths.
Accepted pressure/thermal intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, the corresponding project
unit metadata, matching dimension, a globally unique primitive-load id,
category `pressure` or `thermal`, an existing pipe target, global direction,
finite magnitude in the expected unit, and non-empty provenance. Duplicate
primitive IDs and missing pipe targets are blocked.

The app-level tests create `load:L-100-P300` and `load:L-100-T300` through
the manager, apply each through `OperationApplyPanel`, verify the manager
summary and `load:L-100` primitive count, check the new element-targeted rows,
confirm the local receipt, and confirm stale solve state is reset. The
Playwright R2 smoke checks both rendered create previews without queueing so
the solve/results/report path remains on the unchanged fixture model.

In-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-P300-primitive` and
`op:load-manager-load:L-100-load:L-100-T300-primitive` in clean sessions. The
smoke confirmed `2 load cases; 8 primitive loads; 1 combinations`, pressure
row `load:L-100-P300; element:pipe:P-100; global_x; dimension=pressure`,
thermal row `load:L-100-T300; element:pipe:P-100; global_z;
dimension=temperature_interval`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: imposed-displacement authoring breadth, combination
basis editing, combination term creation/deletion, broader algebra authoring,
Phase B unit picker/display retirement, and packaged-Tauri saved-project smoke
over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-101;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_pressure_thermal_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 seventh sub-slice: concentrated moment primitive-load creation editor (`TP-APP-R2-MOMENTCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-moment
primitive load creation for existing load cases. The create form selects
`concentrated_moment`, captures the target load case, primitive-load id,
existing node target, rotational direction, magnitude, and provenance, then
queues a structured `create_primitive_load` operation. The payload is limited
to target `{ type: "node", node: <existing node id> }`, direction
`rotation_x|rotation_y|rotation_z`, dimension `moment`, and the project
force*length unit basis (`N*m` for the invented preview model). No
pressure/temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`concentrated_moment` primitive-load creation alongside the previously landed
`concentrated_force` and `distributed_force` paths. Accepted concentrated
moment intents must target an existing `Load` with `field_path=primitive_loads`,
`before=not_present`, project force and length unit metadata, dimension
`moment`, a globally unique primitive-load id, category
`concentrated_moment`, an existing node target, rotational direction, finite
magnitude in the expected unit, and non-empty provenance. Duplicate primitive
IDs and missing node targets are blocked.

The app-level test creates `load:L-100-M300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-M300; node:node:N-100; rotation_z; dimension=moment`, confirms
the local receipt, and confirms stale solve state is reset. The Playwright R2
smoke checks the rendered concentrated-moment create preview without queueing
so the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-M300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-moment manager
row with `250 N*m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: pressure/temperature primitive creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-100;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_moment_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 sixth sub-slice: distributed primitive-load creation editor (`TP-APP-R2-DISTLOAD-001`)

The Load Cases manager now exposes explicit distributed element-force
primitive load creation for existing load cases. The create form selects
`distributed_force`, captures the target load case, primitive-load id, existing
pipe target, global direction, magnitude, and provenance, then queues a
structured `create_primitive_load` operation. The payload is limited to target
`{ type: "element", pipe: <existing pipe id> }`, dimension
`force_per_length`, and the project force/length unit basis (`N/m` for the
invented preview model). No concentrated moments, pressure or temperature
primitives, imposed displacements, hidden defaults, unit conversion, or
code-specific combinations are inferred.

The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
`distributed_force` primitive-load creation alongside the previously landed
`concentrated_force` path. Accepted distributed intents must target an
existing `Load` with `field_path=primitive_loads`, `before=not_present`,
project force and length unit metadata, dimension `force_per_length`, a
globally unique primitive-load id, category `distributed_force`, an existing
pipe target, direction `global_x|global_y|global_z`, finite magnitude in the
expected unit, and non-empty provenance. Duplicate primitive IDs and missing
pipe targets are blocked.

The app-level test creates `load:L-100-D300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-D300; element:pipe:P-100; global_y; dimension=force_per_length`,
confirms the local receipt, and confirms stale solve state is reset. The
Playwright R2 smoke checks the rendered distributed create preview without
queueing so the solve/results/report path remains on the unchanged fixture
model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-D300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the distributed-force manager row
with `250 N/m`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: concentrated moments, pressure/temperature primitive
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-99;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_distributed_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fifth sub-slice: concentrated primitive-load creation editor (`TP-APP-R2-PRIMCREATE-001`)

The Load Cases manager now exposes explicit concentrated nodal-force primitive
load creation for existing load cases. The form captures target load case,
primitive-load id, existing node target, global direction, magnitude, and
provenance, then queues a structured `create_primitive_load` operation. The
payload is limited to category `concentrated_force`, dimension `force`, and the
project force unit; no distributed loads, concentrated moments, pressure or
temperature primitives, imposed displacements, hidden defaults, unit
conversion, or code-specific combinations are inferred.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_primitive_load`
intents. Accepted intents must target an existing `Load` with
`field_path=primitive_loads`, `before=not_present`, project force unit,
dimension `force`, a globally unique primitive-load id, category
`concentrated_force`, an existing node target, direction
`global_x|global_y|global_z`, finite magnitude, and non-empty provenance.
Duplicate primitive IDs and missing node targets are blocked.

The app-level test creates `load:L-100-F300` through the manager, applies it
through `OperationApplyPanel`, verifies the manager summary and `load:L-100`
primitive count, checks the new row
`load:L-100-F300; node:node:N-100; global_y; dimension=force`, confirms the
local receipt, and confirms stale solve state is reset. The Playwright R2 smoke
checks the rendered create preview without queueing so the solve/results/report
path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-load:L-100-load:L-100-F300-primitive` and confirmed `2 load
cases; 8 primitive loads; 1 combinations`, the concentrated-force manager row
with `250 N`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: distributed primitive-load creation, concentrated
moments, pressure/temperature primitive creation, imposed-displacement
authoring breadth, combination basis editing, combination term
creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-98;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_primitive_load_creation_editor.md`,
the same-named record under
`DEL-05-05_Concentrated and distributed user load application/_run_records/`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 fourth sub-slice: empty load-case creation editor (`TP-APP-R2-LOADCREATE-001`)

The Load Cases manager now exposes explicit empty load-case creation for the
invented preview model. The create form captures a user-supplied load-case id,
label, kind, status, and provenance, previews the structured create operation,
and queues only a load-case shell with `primitive_loads=0`. It does not create
primitive loads, imposed displacements, code combinations, hidden defaults, or
derived engineering values.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply `create_load_case`
intents. Accepted intents must target `Load` with `field_path=load_cases`,
`before=not_present`, unit `none`, dimension `dimensionless`, a non-duplicate
load-case id, matching JSON payload id, non-empty label/kind/status/
provenance, and absent or empty `primitive_loads`. Non-empty primitive payloads
are blocked so future primitive-load creation remains an explicit A4 tranche.

The app-level test creates `load:L-300` through the manager, applies it through
`OperationApplyPanel`, verifies the manager summary and row
`load:L-300; primitive_user_load; draft; primitives=0`, checks the property
inspector, confirms the local receipt, and confirms stale solve state is reset.
The Playwright R2 smoke checks the rendered create preview without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-create-load:L-300` and confirmed `3 load cases; 7 primitive
loads; 1 combinations`, the empty-shell manager row, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: arbitrary primitive-load creation,
imposed-displacement authoring breadth, combination basis editing, combination
term creation/deletion, broader algebra authoring, Phase B unit picker/display
retirement, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-97;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_creation_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 third sub-slice: combination term-factor editor (`TP-APP-R2-COMBFACTOR-001`)

The Load Cases manager now exposes existing combination term-factor editing
for the invented mechanics preview combination. The editor selects an
existing `Combination.terms.N.factor` row, queues a structured `update_load`
intent with explicit before/after scalar values, unit `none`, dimension
`dimensionless`, local-session audit boundaries, and no professional approval
claim. Whole `Combination.terms` replacement, combination `basis` editing,
term creation/deletion, code/rule combinations, and broader algebra authoring
remain explicitly out of scope.

The browser local operation mirror and Rust `core/model_operations/
operation_applier` crate now validate, diff, and apply the dynamic
`terms.N.factor` path as a dimensionless numeric field. The app-level test
applies `combination:C-OPER-ALT` term 1 from `0.5` to `0.75` through
`OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset. The
Rust test proves `terms.1.factor` applies without mutating the input model and
that whole `terms` editing remains an explicit deferred finding. The
Playwright R2 smoke previews the rendered factor editor without queueing so
the solve/results/report path remains on the unchanged fixture model.

An in-app browser smoke at `http://127.0.0.1:5175/` applied
`op:load-manager-combination:C-OPER-ALT-term-1-factor` and confirmed
`load:L-200 x 0.75`, zero pending operations,
`persistence=session_state_only_not_yet_saved`, `professional_approval=false`,
and solve state `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load
creation, imposed-displacement authoring breadth, combination basis editing,
combination term creation/deletion, broader algebra authoring, Phase B unit
picker/display retirement, and packaged-Tauri saved-project smoke over edited
load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-96;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-02_Load-case algebra engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_combination_term_factor_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 second sub-slice: load-case metadata editor (`TP-APP-R2-LOADMETA-001`)

The Load Cases manager now exposes selected load-case metadata editing for
`status` and `kind`. The editor queues structured `update_load` intents for
`Load.status` or `Load.kind` with explicit before/after values, unit `none`,
dimension `dimensionless`, local-session audit boundaries, and no
professional approval claim. The browser local operation mirror and Rust
`core/model_operations/operation_applier` crate now validate, diff, and apply
those two load metadata fields; combination `basis`/`terms` editing remains
explicitly deferred.

The app-level test applies `load:L-100` status from `preview_only` to `TBD`
through `OperationApplyPanel`, verifies the manager row, property inspector,
pending-operation count, applied-operation count, and stale-solve reset, then
verifies the `kind` editor previews `primitive_user_load -> TBD` without
applying a second operation. The Playwright R2 smoke checks the rendered
status/kind metadata controls without queueing so the solve/results/report
path remains on the unchanged fixture model. An in-app browser smoke at
`http://127.0.0.1:5175/` applied the rendered status edit and confirmed the
row showed `status=TBD`, pending operations returned to zero, and solve state
remained `not_started`.

Residuals remain in A4: load-case creation, arbitrary primitive-load creation,
imposed-displacement authoring breadth, full combination editing/algebra
authoring, Phase B unit picker/display retirement, and packaged-Tauri
saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-95;
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_metadata_editor.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 seventh sub-slice: viewport pipe endpoint picking (`TP-APP-R2-PIPEPICK-001`)

The viewport pipe form now has explicit endpoint-pick controls. Arming
`Pick` for the `from` endpoint and selecting a rendered viewport node fills
the pipe form's `from` field, advances to `to` picking, and still updates the
normal viewport/model-tree selection. Selecting a second rendered node fills
the `to` field and clears pick mode. Picking only supplies node references;
material, section geometry, non-zero `y_reference`, and provenance remain
explicit user-entered fields before Queue pipe can be enabled.

The app-level test picks `node:N-100` and `node:N-140` from viewport targets,
fills the remaining explicit pipe fields, queues and applies
`pipe:P-151` through `OperationApplyPanel`, and verifies the created pipe is
selected in the model tree, viewport layer, and inspector. The Playwright R2
smoke covers the rendered endpoint-pick controls before the unchanged
solve/results/report flow. An in-app browser smoke at
`http://127.0.0.1:5175/` confirmed `from=node:N-100`,
`to=node:N-140`, pick-mode advancement/clearing, and Queue pipe remaining
disabled until the remaining explicit fields are supplied.

Residuals remain in A3: canvas gesture capture beyond node drafting and
endpoint picking, component/rigid authoring, and broader editor coverage as
new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-94;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_pipe_endpoint_picking.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 sixth sub-slice: canvas node drafting (`TP-APP-R2-CANVASNODE-001`)

The viewport canvas now captures a primary pointer gesture into the explicit
node form before any operation is queued. In WebGL mode, the handler raycasts
from the Three.js camera to the `y=0` drafting plane; in fallback/test mode,
it maps the pointer into the same bounded model-drafting plane. The captured
draft fills a visible editable node id (`node:V-001` style), label, and finite
x/y/z coordinates. The existing Queue node and Apply Operations path remains
the only mutation path.

The app-level test applies a canvas-drafted node through the structured
operation seam and verifies model-tree/inspector selection. The Playwright R2
smoke now exercises the real browser canvas click path without queueing the
draft, so the solve/report smoke remains on the unchanged fixture model. An
in-app browser smoke at `http://127.0.0.1:5175/` confirmed a canvas click
drafted `node:V-001`, finite coordinates, `y=0`, and enabled Queue node.

Residuals remain in A3: canvas gesture capture beyond node drafting
(pipe/connectivity and component/rigid authoring), rigid/component authoring,
and broader editor coverage as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-93;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_canvas_node_drafting.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and the
same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A3 fifth sub-slice: explicit straight-pipe creation (`TP-APP-R2-CONNECTPIPE-001`)

The viewport editor now has an explicit straight-pipe form for user-entered
pipe id, label, endpoint nodes, material, outside diameter, wall thickness,
non-zero local `y_reference`, and provenance. The form queues a structured
`connect_pipe_run` intent with `field_path=pipe_segments` and applies it
through the same operation review/acceptance path as other local-session
edits. Legacy one-click pipe-run viewport gestures remain blocked because
they still carry only underspecified `viewport.connect_pipe_run` data.

The Rust `core/model_operations/operation_applier` crate and browser local
operation mirror both validate the explicit pipe payload: matching id,
`before=not_present`, project length unit, `dimension=length`, non-duplicate
pipe id, existing endpoint nodes/material, positive OD/wall quantities, and
non-zero `y_reference`. Applying the intent appends a new `pipe_segments`
record to the returned session model without mutating the input model in
place. The app test confirms `pipe:P-150` is created, selected in the model
tree and viewport selection layer, visible in the property inspector, and
recorded with local-session acceptance only.

Residuals remain in A3: true canvas raycast/gesture geometry capture,
rigid/component authoring, and broader editor coverage as new authoring
surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-92;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_explicit_straight_pipe_connectivity.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-11 — A4 first sub-slice: load-case primitive magnitude manager (`TP-APP-R2-LOADMGR-001`)

The desktop app now has a right-rail Load Cases manager over the invented
preview model. It surfaces load-case counts, primitive-load rows, combination
terms, and a focused primitive-load magnitude editor. Selecting
`load:L-100-P` exposes `primitive_loads.2.magnitude.value`; changing the
magnitude queues `op:load-manager-load:L-100-load:L-100-P-magnitude` as a
structured `update_load` intent. Applying the queued operation uses the
existing OperationApplyPanel, records local-session acceptance, clears stale
solve results, and leaves persistence to the Save local path.

Residuals remain in A4: load-case creation, load status/kind editing,
arbitrary primitive-load creation, imposed-displacement authoring breadth,
full combination editing/algebra authoring, unit picker/display retirement
after Phase B, and packaged-Tauri saved-project smoke over edited load data.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-91;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-11_load_case_manager_primitive_magnitude.md`,
the same-named record under
`execution/PKG-05_Loads, Load Cases, and Stress Recovery/1_Working/DEL-05-01_Primitive load case engine/_run_records/`,
and the same-named record under
`DEL-05-02_Load-case algebra engine/_run_records/`.

## 2026-06-11 — A8 first sub-slice: Playwright R2 smoke harness (`TP-APP-R2-PLAYWRIGHT-001`)

The desktop workspace now has a Playwright harness and root script
`npm run test:e2e:desktop`. The first smoke test runs the technical-preview
fixture through initial shell checks, local-only boundary checks, a nonblank
and animated Three.js viewport assertion, mechanics preview solve,
`result_rows=647`, viewport displacement-overlay availability, result
filtering/detail inspection for `pipe:P-120`, and deterministic report-packet
export checks.

Vitest is scoped to `src/**/*.test.{ts,tsx}` so the unit suite and Playwright
suite remain separate. Playwright local output directories are ignored.

Residuals remain in A8: authored create/edit -> solve -> report automation,
full manual SMOKE checklist parity, packaged Tauri saved-project solve smoke,
and CI browser provisioning policy.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-90;
`execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/WORKING_ITEMS_RUN_2026-06-11_playwright_r2_smoke_harness.md`.

## 2026-06-11 — A6 first sub-slice: viewport displacement overlay (`TP-APP-R2-DEFORMEDVIEW-001`)

The Three.js viewport now consumes the current mechanics result and renders a
review-only shape overlay after a solved preview run. The first slice uses
available `displacement_magnitude` rows by node, draws a teal overlaid
centerline/marker set, and exposes a toolbar status with node count, maximum
reported displacement magnitude, and an explicit boundary:
`scale=normalized_display_offset_not_physical_length`,
`vector_direction=TBD`, and `professional_claim=false`.

When no result is present the overlay status is `not started`; when the
current mechanics result is incomplete, such as the browser edited-model guard,
the overlay status is `blocked` and no deformed overlay is rendered.

Residuals remain in A6: true directional deformed shape once displacement
vectors exist, support-reaction visualization, stress/governing-ratio views,
and richer result-selection coupling.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-89;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-11_viewport_deformation_overlay.md`
and the same-named record under
`DEL-07-05_Results viewer/_run_records/`.

## 2026-06-11 — A5 first sub-slice: model-bound solve guard (`TP-APP-R2-SOLVEBOUND-001`)

The preview mechanics service now refuses to reuse bundled solved-result rows
for an edited model in browser fixture mode. Edited browser-session models
return a `MODEL_INCOMPLETE` mechanics result with zero result rows and an
explicit `BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL` blocking
diagnostic. The unchanged fixture model still returns the bundled solved
fixture for browser preview workflows.

The Tauri backend path was verified as model-bound: direct
`run_preview_mechanics(Some(model))` and the solve-job registry both solve the
supplied edited model payload and publish result envelopes bound to the edited
`project.id`, not the bundled fixture id.

Residuals remain in the A5 row: full packaged-Tauri GUI smoke over a saved
edited project snapshot, richer incomplete-model UI copy, and broader
persisted non-fixture solve coverage as authoring surfaces grow.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-88;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-07_Solve execution UX- progress, cancellation, and diagnostics/_run_records/WORKING_ITEMS_RUN_2026-06-11_edited_model_solve_binding.md`,
the same-named record under
`execution/PKG-14_Model States, Analysis Runs, and Comparison/1_Working/DEL-14-02_Analysis run records/_run_records/`,
and the same-named record under
`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-06_Solver diagnostics and singularity detection/_run_records/`.

## 2026-06-10 — A3 fourth sub-slice: session undo/redo checkpoints (`TP-APP-R2-UNDOREDO-001`)

The Apply Operations panel now exposes local-session Undo/Redo controls for
applied structured operations. Applying an operation records a checkpoint for
the previous session model and selection; undo restores that checkpoint,
moves the current model to redo, and clears stale solve results; redo restores
the undone model and clears stale solve results again. The history summary is
explicitly labeled local-session-only and saved-project-mutated=false.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, and broader editor coverage as new
authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-87;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_session_undo_redo_checkpoints.md`
and the same-named record under
`DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 third sub-slice: explicit node create operation (`TP-APP-R2-CREATENODE-001`)

The viewport editor now has an explicit node-geometry form for user-entered
node id, label, and finite x/y/z coordinates in the project length unit. The
form queues a structured `create_node` intent and applies it through the
existing operation seam; the browser local engine and Rust applier accept only
explicit node payloads, reject duplicate ids, and preserve the no-silent
conversion/default posture. The applied target becomes the active model-tree
and inspector selection. During browser smoke this tranche also fixed
viewport/operation-panel hit-test layout issues so the new form and apply
buttons are actionable in the live app.

Residuals remain in the A3 row: true canvas raycast/gesture geometry capture,
straight-pipe connectivity creation, undo/redo, and broader editor coverage
as new authoring surfaces land.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-86;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_explicit_node_create_operation.md`,
the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`, and same-named
records under `DEL-16-02_Operation validation and diff preview/_run_records/`
and `DEL-16-03_User acceptance and operation audit trail/_run_records/`.

## 2026-06-10 — A3 second sub-slice: property-inspector inline validation (`TP-APP-R2-INLINEVALID-001`)

The property inspector now exposes validate-only feedback for draft editor
intents before queue/apply. The UI calls the existing structured-operation
validation seam and displays application status, schema/unit/before-state
states, diff rows, diagnostics, and a no-mutation/professional-boundary note.
This landed as an A3 editor UX sub-slice; broader editor coverage, undo/redo,
and true geometry-capture workflows remain in the A3 row.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-85;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`
and
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_property_inspector_inline_validation.md`.

## 2026-06-10 — A3 first sub-slice: viewport selection binding (`TP-APP-R2-VIEWSELECT-001`)

Viewport entity selection controls for loaded nodes, straight pipes,
supports, and component markers now drive shared selection, model-tree
active state, property-inspector binding, and viewport active highlight.
Residuals (remain in the A3 row): true canvas raycast/gesture geometry
capture, node/straight-pipe creation tools with explicit
coordinates/connectivity, undo/redo, and inline validation messages.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-84;
`execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-01_3D viewport and centerline editor/_run_records/WORKING_ITEMS_RUN_2026-06-10_viewport_selection_binding.md`
and the same-named record under
`DEL-07-02_Model tree and property inspector/_run_records/`.

## 2026-06-10 — A2 landed: model-document persistence (`TP-APP-R2-PERSIST-001`)

Implemented under `DEC-019` (D-08 ruling): in-document semver authority,
application-service transform chain (migrate-in-memory-on-open /
persist-on-save), refusal semantics for newer/unsupported documents, store
v9 evidence-only migration ledger with pre/post hashes, and
validation-preflight evidence replacing the prior TBD marker. Open residuals
(remain in the A2 row): compatibility-window size (human ruling), explicit
"Migrate project" operation, sibling JSON-slot coverage.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-83;
`execution/PKG-02_Domain Model, Units, and Core Schemas/1_Working/DEL-02-05_Project persistence and round-trip serialization/_run_records/WORKING_ITEMS_RUN_2026-06-10_model_document_schema_migration.md`
(with same-day `TASK_RUN_2026-06-10_*.md` records in the same folder).

## 2026-06-10 — A1 landed: apply-operation command path (`TP-APP-R2-EDITLOOP-001`)

New `core/model_operations/operation_applier` crate plus the
`apply_model_operation` and `validate_model_operation` Tauri commands
(desktop bridge now 14 commands, two on the mutating path) and an Apply
Operations panel. Inspector modify intents apply to the session model;
viewport gesture intents block pending A3 geometry capture; unit conversion
blocks pending D-01/Phase B.

Evidence: `apps/desktop/SMOKE.md` TP-MAC-82;
`execution/PKG-16_Model Operation and Agent Proposal Framework/1_Working/DEL-16-02_Operation validation and diff preview/_run_records/WORKING_ITEMS_RUN_2026-06-10_apply_operation_command_path.md`.

## 2026-06-10 — Decision packets D-01 and D-08 prepared and ruled

Both packets drafted and ruled same day: `D-01` → `DEC-018` (SI-canonical
with dual display catalog, as the packet proposed; Phase B unblocked);
`D-08` → `DEC-019` (per-document semver transform chain,
migrate-in-memory-on-open / persist-on-save, as the packet proposed; A2
unblocked). `D-10` packet drafted the same day, `AWAITING_RULING`. Packets
and state: `execution/_Coordination/_DECISIONS/_REGISTER.md`; rulings:
`SOFTWARE_DECOMP.md` §12. This completed items 1–2 of the plan's original
"first three tranches" sequence; item 3 (A2) landed the same day, above.
