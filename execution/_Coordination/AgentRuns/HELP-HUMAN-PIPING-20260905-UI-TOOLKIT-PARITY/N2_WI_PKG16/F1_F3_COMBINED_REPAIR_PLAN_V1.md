# Combined F1/F3 bounded operation repair — planned, source HELD

Basis: parent accepts F1_REPAIR_DESIGN_V2 direction and N6 F3_READ_BOUNDARY_DISPOSITION_V1; full N7 return/release remains mandatory before source writes. This plan supersedes F1 V2's incorrect stiffness catalog claim and F3 V1's proposed broader class-consistency checks. No code/contract source changed.

## Exact behavior

F1: validate both support.stiffness and support.hanger.stiffness independently through existing strict structure, accepted DOF aliases, finite-positive values and **unit_by_symbol/convert_for_dimension**. Compare case-normalized DOF and exact canonical normalized f64 when both present. Different DOF/value blocks; equal definitions and either single source remain valid and preserved byte-semantically as original JSON. No tolerance, silent clearing, priority rewrite or catalog restrictions.

Correction to prior read-only catalog assessment: `unit_supports_dimension` explicitly permits ForcePerLength units for LinearStiffness. Thus N/m, lbf/ft and lbf/in are supported through the existing converter; the US hanger path is not defective. Do not replace that converter with a hand-picked stiffness-unit list or new constants. Tests include both US single-source quantities and equivalent normalized duplicate values, deriving comparison values through the accepted converter rather than introducing conversion formulas. Exact comparison remains fail-closed for genuinely unequal normalized values, including adjacent representable values.

F3: family absent **or JSON null** is permitted and source-preserved; both retain the consumer's existing inference. Explicit string allowed exactly: anchor, guide, line_stop, vertical_support, spring, variable_spring_hanger, spring_hanger, constant_effort_support, nonlinear. Reject unknown/blank/padded/PascalCase/non-string non-null values with actionable token diagnostics; never trim/rewrite source. Move family out of optional_text validation and give it this explicit optional/null-aware gate; leave provenance validation unchanged. Genuine DOF/nonlinear/hanger aliases remain untouched. No new mixed-family/hanger/nonlinear payload exclusivity or classification-consistency matrix. Existing precedence remains; token validity does not claim payload class coherence.

## Source graph and exact ownership after release

N2 implementation Agent2: only `core/model_operations/operation_applier/src/rich_authoring.rs` plus new `core/model_operations/operation_applier/tests/support_authoring_conflicts.rs` if public-route regression can use existing public interfaces. Unit tests remain in owned rich module; no model-operation schema enum, lib routing, physics, desktop, unit catalog or other module writes. Manager verifies current released predecessor hashes, creates sealed implementation brief/telemetry and archives frozen predecessor diff evidence before dispatch. If test setup requires shared lib.rs edits, report exact need for manager serialization first; do not expand child fence silently.

N6 owns matching physics family boundary and explicit guide arm in product_physics exclusively, using their accepted plan after release. N1 owns canonical UI family tokens. These source files are disjoint and may run in parallel after parent releases freeze, then independently reviewed and integrated. N2 never edits physics.

## Acceptance

Public operation validate/apply create_support and configuration replacement must reject unequal duplicate stiffness and invalid explicit families with no applied model; late failing batch must roll back earlier valid steps/no acceptance. Positive tests: equal duplicate stiffness exact raw preservation, translational/rotational identity, one-source US lbf/in+lbf/ft, canonical-equivalent US duplicate, accepted case aliases, canonical family list, absent/null family preservation. Confirm F3 invalid tokens reject even with otherwise valid spring/hanger/nonlinear payload so no precedence bypass. Preserve existing full support configuration requirements; use complete public fixtures so the intended diagnostic is decisive. No new class-consistency assertions.

Run full operation crate including public integration tests and schema regression. Independent fresh reviewer covers100% exact new diff and freezes source hashes; N7 F1 public Wasm reproduction reruns only after parent-coordinated new artifact build. Parent composes physics/UI tests and full integrated final review. Previously accepted C/RN snapshots remain historical, not silently relabelled current after repair.

## Gates and residuals

Await N7 full return and explicit parent source release. Mixed canonical-family/payload contradictions remain an explicitly unmodified semantic-consistency residual, not fixed by token spelling validation. D58 and durable acceptance-history residual remain held/recorded. No need for new physics, schema or professional acceptance rules.
