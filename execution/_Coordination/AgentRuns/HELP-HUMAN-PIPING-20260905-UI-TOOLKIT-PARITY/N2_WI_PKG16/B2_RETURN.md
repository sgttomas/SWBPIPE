# B2 implementation return

RUN_STATUS: SUCCESS — bounded module implemented and tested; not lifecycle closure.
Role: ephemeral Agent 2; no delegation; parent N2_WI_PKG16; native instruction-asserted.
Basis: sealed B2_BRIEF.md, AMENDMENT_B2.md, existing consumer interfaces cited by A2_RETURN.md. This return is derivative implementation evidence, not decomposition authority.
TaskSkill: software-bounded-implementation v1 (explicit parent read/write/bash and targeted offline Cargo grant).
WriteAuthorization: module rich_authoring.rs and B2_RETURN.md/B2_CHECKS.json only; no lib.rs/Cargo/receipt edits.
ToolsUsed: read/search via bash cat/sed/rg, Python bounded edits/report writes, rustfmt module only, offline Cargo crate tests with {CARGO_TARGET_DIR}.
ToolPolicyCompliance: PASS under explicit sealed brief and parent test authorization. Broad registered sweep deferred to parent.

## Integration API

`pub(crate) resolve(model: &Value, object_type: &str, target_ref: &str, field_path: &str, before: &str, after: &str, unit: &str, dimension: &str) -> Result<Option<RichEdit>, RichError>`.

Returns None for unowned type/path. Caller MUST gate operation kind: `update_support` Support/configuration; `set_field` Material/temperature_points; `update_load` Load/equivalent_static.wind.exposure. No arbitrary JSON patch surface.

RichEdit fields: `writes: Vec<(Vec<String>, Option<Value>)>` entity-relative writes (None means delete optional member); `current_display: String` exact canonical before display; `warnings: Vec<String>` readiness warnings to surface in validation/preview. RichError fields `code: &'static str`, `message: String`.

`validate_support_configuration(model: &Value, support: &Value) -> Result<()>` validates rich members of a full support; preserves responsibility of legacy properties and create identity validation in existing caller. Parent integrated this serially; B2 did not change lib.rs.

## Behavior

Support replacement changes only family/restraints/stiffness/hanger/nonlinear/provenance; omitted optional members clear old slots. Identity/label/node are retained. Validates complete known hanger input rules, stiffness units tied to explicit DOF, consumer nonlinear enums/aliases, normal-reaction source validity, and prospective effects on downstream derived reaction consumers. Hanger+nonlinear remains permitted per existing consumer precedence. Constant-effort missing acting DOF retains consumer review-only behavior; no invented force direction. Spring family requires explicit stiffness and DOF rather than allowing inferred consumer DOF. Unknown hanger types and unknown fields in all replacement nested objects block; legacy family text is preserved.

Material replacement preserves entered units and arbitrary signed finite alpha. Incomplete unselected rows return readiness warnings. Unique point ids and normalized temperatures enforced; selected exact basis cannot lose required E/G or its point; interpolation requires strict adjacent bracket and endpoint E/G/alpha, no fallback/extrapolation. Same basis ids in different material tables remain valid.

Wind replacement atomically replaces whole refs and partial spans; validates references, dimensions, fractions, overlap and whole/partial collisions. Adjacent spans and authored list order preserved. Empty exposure returns not-solve-ready warning; pressure/shape/direction untouched.

Envelope requires none/dimensionless; each nested quantity uses catalog units independently, including established dimensionless none alias. Stale canonical before blocks. No-op replacement preserves exact original authored Value representations and units.

## Verification

B2_CHECKS.json records exact command, source hash and result: **101 unit +1 native contract +2 canonical hash tests passed**. Ten tests belong to B2. Initial failures were corrected and rerun: ownership move/borrow issue and invalid test-only unit symbols/canonical number comparisons. No source input mutated in validator tests. No build warnings except current_display unused by current parent integration (API remains available).

## Handoff

Outputs complete for module slice. Parent owns shared lib.rs integration, browser/Wasm and broader registered checks, independent frozen-diff review, governing snapshot/handoff and closeout. D58 remains held. No physics formulas, solver behavior, protected data, authority documents or agent binding changed.
MISSING: none for B2 bounded module; external review and whole-tranche verification remain.
NEEDS_HUMAN_RULING: none.
DEPENDENCY_NOTES: module now usable by parent integration; UI must consume frozen accepted fields/before contract. No work outside B2 write scope was modified by this instance.
