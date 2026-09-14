# Engineering field and availability map V2

Supersedes the preliminary concept map V1 only for derived UI field/availability planning. Consumes root-relayed ENGINEERING/UI_BACKFILL_V1.md SHA256 `0be6de5972d913f3bb362b731e5adb893bbe6207331eded5dc8748076692c9ae`, composite candidate manifest V2 `cd77084b55e07f83561d30120547839f0f0614c59bd52174809456b0fc7ec610`. Both hashes verified. Engineering review remains pending; this is proposed backfill, not runtime activation. The frozen prototype V2 source is unchanged during witness.

| Field group / human task | Concrete design mapping | Availability and truthful behavior |
|---|---|---|
| Material model profile | Show E and nu as authoritative pair for proposed exact model profile; G derived read-only; retained G labelled original source data | Model-wide exact E/nu basis; missing pair allows edit/save and blocks exact solve. Do not infer missing nu or allow an authoritative third constant |
| Legacy pressure | Existing pressure primitive/magnitude/pipe target and legacy bore/EJ effective-area source | Current production source route remains LIVE_LEGACY. The offline prototype itself is always simulated; it does not execute that route |
| Exact pressure region | Explicit member set, two physical terminal nodes, pressure basis/unit and each terminal closure-transfer path | Proposed model0.3.0 backfill; no automatic closed-end inference. New exact mode visibly unavailable to live solver |
| Pressure actions/results | Separate cap/eigen/wall force, wall action/section/effective force, inner/outer radial/hoop/axial membrane components | Explicit unit/dimension/frame/region/material provenance. Exact pressured region suppresses legacy governing scalar/max stress: unavailable, not zero. Initial combinations unsupported |
| Connector ends | Two attachment/end-plane nodes, initial triads/offsets, Q and explicit graph roles/targets | Design fields require future operation support. Symbol creation does not activate objective mechanics |
| Constitutive reference | q_ref/reference and prestress/initial residual; declared scale | Reference and current geometry stay distinct; do not erase initial residual |
| Coupled stiffness | Independently typed fixed-SI H work-parameter record | Display H as Work coefficient [N*m] parameter evidence, never moment result. Preserve typed coefficient roles and scaling; no guessed diagonalization |
| Topology | Explicit replacement/series/parallel role and target members | Pressure-joint composition is UNSUPPORTED_COMPOSITION until separate load-path proof. Connector with legacy pressure does not depend on E/nu |
| Finite insert-in-run | Preview exact replaced graph, nodes/spans/loads/support/provenance ownership and unowned-load blockers | Backfill only; future atomic structured batch, no direct JSON mutation if field unsupported |
| Navigation/routing | Existing view/query state and typed route operations | Ready for current professional workspace design without pressure/connector kernel completion |

Engineering handoff labels LIVE_LEGACY, DESIGN_BACKFILL, SUPPORTED_INCOMPLETE, SUPPORTED_READY and UNSUPPORTED_COMPOSITION are design terms only. Map them to existing capability wording; do not create canonical model enums or an independent registry. They describe capability availability, separately from editing lifecycle Draft/Busy/Stale/Cancelled/Blocked and result Current/Historical/Unavailable.

Versioned conversion legacy→exact changes model basis and invalidates Current. Authentic untouched saved attachments remain Historical on reopen; no display reclassification rewrites historical witness/dimension/serializer hashes. Any proposed v2 result is explicitly a backfill illustration and must never be represented as a current actual solver result. This prototype uses invented legacy-like examples only; no exact pressure/joint result activation is implemented.

UI implementation remains DEL-07-01/02 under SCA009 landings; DEL-07-09 coverage only; PKG16 owns supported operations. Root's review/acceptance and following source-bound Rust evidence own effective semantics. This derivative map introduces no state/dependency/decision effect. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
