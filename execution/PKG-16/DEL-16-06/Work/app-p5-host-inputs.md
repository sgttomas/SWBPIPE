# App P5: host inputs and remaining joined evidence

**Consumer:** App v4 publication punchlist P5, coordinated by the two projects'
Agent 0s. **Exit criteria:** 17, 7 and 2. Code/input basis:
`sgttomas/SWBPIPE@8b02f8b9ec07a02e7427fb07107cb41e6bc2a2e0`.
This account selects source for offline preparation, not a native executable,
run, receiving adoption or completed witness. Read the source at that revision;
later source changes require an affected-input comparison before use.

App retains its receiving selection at
`9b6281e7eaca924e6166336b87b6d25937c1ca04`. The host catalog remains
`swbpipe-catalog/4`, SHA-256
`8e9a4b4e3eec027d092033ea5d95f1648030520e797c5a660ab387531f23be17`.
The maintained file is now
[`apps/desktop/src-tauri/resources/agent-capability-catalog.json`](../../../../apps/desktop/src-tauri/resources/agent-capability-catalog.json);
the former Work copy was removed. Identical catalog bytes do not establish
equivalence of changed producers. App owns receiving reconciliation after the
projects select a concrete later host candidate; no App adoption changes here.

## Current activation and attribution route

The person opens Operations > Agent and explicitly selects/saves a provider
revision and context window, starts a session and sends a request.
[`EmbeddedAgentPanel.tsx`](../../../../apps/desktop/src/features/embedded-agent/EmbeddedAgentPanel.tsx)
uses [`embeddedAgentSession.ts`](../../../../apps/desktop/src/services/embeddedAgentSession.ts)
and the native `embedded_agent_host` command. The command/bridge, `live_host.rs`,
`live_runtime.rs`, `macos.rs` and Rust-owned HTTP relay own registration,
supplier startup, bounded dispatch and actual outcomes; these modules live in
[`embedded_agent/`](../../../../apps/desktop/src-tauri/src/embedded_agent).
Availability checks do not start a supplier. Unqualified platforms refuse.

Agent operations enter the existing Batch review as agent proposals. The sole
operation engine validates against the current basis; the person's explicit
Apply publishes the changed model and correlated outcome hash. A message,
claimed author type or scripted click is not personal acceptance. Original
agent attribution and assumptions survive review. Engineering acceptance,
issuance and release remain human acts (F-PIP-3).

Compared with App's adopted producer, the selected source adds strict
engine/catalog hash conversion, uses shared deletion-summary formatters and
reads native result pages through native custody with a post-await ownership
check. App's old retained producer/test snapshots are not replaced by this
description. The actual diff is bounded by the two revisions above, especially
`embeddedAgentSession.ts`, `embeddedAgentM2.test.ts`,
`features/model-tree/removalIntents.ts`, `nativeResult.ts` and `resultDetail.ts`.

### Catalog offering and admission

The selected catalog has no exposure or agreement fields; it does not classify
any entry as non-exposed or unagreed, nor establish agreement by their absence.
`catalog.rs::Catalog::load` checks the seven supported tools, effects, schemas
and admitted operation tuples. `Catalog::declarations` emits all seven in
`Tool::ALL` order, without an exposure filter. `protocol.rs::start_thread`
supplies those declarations in `thread/start.params.dynamicTools`.

The host ingress is `lib.rs::embedded_agent_host`, reached through
`embeddedAgentSession.ts`'s native invoke. It bounds ingress and binds the
actual main webview/origin; `live_host.rs` owns workspace/start authority,
`live_runtime.rs` starts the supplier and `session_driver.rs` loads the catalog
for the protocol. Supplier `item/tool/call` requests enter
`protocol.rs::tool_call`: current thread/turn, unique identities, supported
tool/no namespace, bounded schema-valid arguments, proposal tuples/cap and
pending capacity. `dispatcher.rs` rechecks arguments, current basis and custody
before execution. None of these decisions reads an exposure classification or
routes through App's receiving projection. The bridge's `offered: Option<Claim>`
is a one-use execution claim, not catalog exposure metadata.

This establishes the source-selected offering and admission path, not the
supplier's complete serialized Responses tool set. The corrected HC-1 capture
has not run; historical captures had extra supplier tools. App's local
`exposure.E != exposed` rejection therefore has no demonstrated applicability
to this native ingress. No supplied host entry is locally withheld here solely
on an exposure reading; an unsupported tool is a different refusal. Whether a
later App integration calls its receiving projection must be established on
that connecting route. This finding creates no additional P5 blocker or
consumer change.

## Proposed input selection

These are existing maintained inputs at the selected source, not generated run
evidence. Their actual run-time bytes and clean checkout revision must be bound
again by any later authorized invocation.

| Input | Selected source and purpose |
| --- | --- |
| Supplier | Stock ARM Codex 0.160.0; executable SHA-256 `112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b`, recorded in `fixtures/embedded_agent/agent-witness-pin.json`. The retained bundle copy is present and its bytes match; it was only read. No acquisition or supplier re-signing is needed for this proposal. |
| Production configuration | `embedded_agent/macos.rs::CONFIG_BODY` and `supplier_config`; HC-1 disables `features.goals` and `tools.experimental_request_user_input.enabled`. The recipe remains read-only/never, with built-in execution disabled and a fresh per-start relay capability. |
| Protocol/instructions | `embedded_agent/protocol.rs`, `live_runtime.rs::INSTRUCTIONS`, and client version from `apps/desktop/src-tauri/Cargo.toml`. Capture follows the selected initialization/thread/two-turn shapes. |
| Capture adapter | `tools/agent_supplier_capture.py`; original process/pump/cleanup from `tools/agent_host_witness.py::Native`; scripted exchange from `tools/agent_journey_fixture.py::Script`, `StubProvider`, `TURNS`. Capture uses the corrected source recipe, not the historical witness's different configuration. |
| Synthetic exchange | `fixtures/model_operations/agent_w9a_journey.json` and `agent_w9a_capture_outcomes.json`; seven declared tools, two turns, six host calls and eight expected Responses bodies. Outcomes are invented fixture data, not current host execution. |
| Native W9-A journey | `tools/agent_journey_fixture.py`, `tools/agent_live_session_fixture.py`, `apps/desktop/native-smoke/agent-journey.js`, `tools/agent_journey_memory_monitor.py`, the exact native executable/frontend/operation and self-weight wasm inputs, catalog and W9-A numerical references. No corrected executable is selected by this account. |
| Full reference system | DEL-09-01 [`reference-system.md`](../../../PKG-09/DEL-09-01/Work/reference-system.md) §§9–11.3; `fixtures/reference_system/reference_system.json`, `authoring.json`, `key_results.json`, generator and captured self-weight; independent `validation/references/reference_system/`. Carried or pending fixture members do not establish solve/catalog admission. |

The complete capture input list is bound by the adapter's `run()` result,
including the catalog, both exchange fixtures, native instruction/configuration
sources, Cargo manifest and three Python sources. The capture checks checkout
cleanliness before preparing a listener/supplier and again after cleanup.
Its direct no-auth stub endpoint differs from the product's authenticated
Rust relay; this experiment does not qualify that relay or the webview.

## W9-A and the full reference system

| Need | W9-A / current evidence | Missing input or capability for full P5 evidence |
| --- | --- | --- |
| Build/propose/apply | W9-A is an 11-operation pressure-free L-frame batch. The historical scripted native journey exercised review/Apply/solve plumbing. Component/wasm journeys add separate invented examples. | The reference system needs phased proposals within the 16-operation limit, exact current basis/before texts, actual review and attributable personal Apply between phases. No current full-system native/live journey is established. |
| Materials, members, components and supports | Catalog v4 admits material/node/pipe/support creation, component symbols, support configuration and connector hardware/pressure-model edits. | Rich material/section/state payloads and their named identities need a connected reference-system authoring path. Named section creation/assignment, project-unit changes and generated self-weight have no dedicated admitted catalog operation; inline section/initial project units do not alone prove the required complete path. |
| Bends, k and SIF | A bend symbol is an agent-authored geometry marker. Agent/imported operations cannot author modifiers or realize a bend. | The person realizes bends and enters k, SIFs and source after the component proposal and before attachments prevent realization. Preserve that author route and request the personal inputs; do not stamp an agent payload as manual entry. |
| Loads, states and combinations | Primitive load-case/load creation, pressure-region update and pressure-profile activation are admitted. W9-A uses one thermal case. | The catalog has no dedicated operations for load/reference-state authoring, operating-condition sets, generated wind/seismic, combinations, hydrotest or spectra. Inspect the actual admitted tuple and applier payload for each proposed extension; a generic enum or fixture field grants no new operation. |
| Analyses and results | Ordinary solve, proposal observation and bounded filtered result reads exist. W9-A checks six node:c motions against its independent reference and the displayed report. | Full-system key results are DEL-09-01 §9.1. EX/reference-family coverage, CE/S6 correction and subsequent modal/spectrum/hydrotest work remain separate prerequisites. Modal, spectrum and hanger-sizing request tools are not admitted. Hanger and H1 selection constraints remain in force. |
| Lifecycle, isolation and recovery | The recorded ARM lifecycle/isolation matrix and scripted W9-A have explicitly bounded historical configurations. Native actuals distinguish domain outcome from delivery; unknown cleanup retains custody/markers and refuses restart. | HC-1 changes configuration embedded in the executable. Rebuild/rebind a selected native/frontend/wasm candidate and establish applicable isolation, EOF/app-death, interruption, stale basis and recovery evidence. Do not reuse historical executable hashes as corrected bytes, infer a restart sweeper or signal through remembered PIDs. |
| Model behavior and person attribution | Synthetic provider/tool outcomes establish plumbing only. A driver click is a qualification stand-in. | A joined model-backed proposal/apply/outcome/result witness and the owner's attended Apply remain absent. None of these observations supplies professional acceptance or release. |

The full system's remaining mechanisms and operation coverage must be designed
and verified under their owning deliverables; this mapping neither expands the
catalog nor selects those implementation lanes. CE/S6 remains first. App P5
stays open and does not block App's independent J2 continuation.

## Bounded decisions prepared, not granted

**First useful observation:** the supplier-only envelope capture defined in
[`agent-d4-capture-proposal.md`](agent-d4-capture-proposal.md), HC-1. Both
earlier grants are consumed; no attempt is authorized. The governing proposal
holds the attempt bounds, isolation, caps and stop/cleanup requirements. The
concrete retained executable locator and fresh output are supplied in the
PR's owner-decision preparation.

After a successful authorized capture, recount its actual serialized input
through the selected provider conversion and strict tokenizer/template path,
then reconcile D4's context/output reserves. A source-only reconstruction,
saved 32,768 setting or catalog byte count is not measured wire fit or actual
served context. A capture pass would close this prerequisite only.

**Later native/model decision:**
[`agent-d4-run-plan.md`](agent-d4-run-plan.md) D4-5 requires a corrected
native executable and selected frontend/wasm/controller/monitor input binding;
its historical executable/asset identities are not an HC-1 build. Native
preflight/launch, any private-copy seal, credential handling and model/server
use need the applicable separate authority and a concrete selected packet.
Preserve the plan's authenticated readiness, actual served context/output
allowance, owned cleanup, cancellation/monitoring and 50 GiB headroom rules.
Do not infer such authority from this preparation or request an unready live
attempt. Full reference-system coverage and personal Apply still follow their
own requirements; a successful live W9-A would not complete criterion 17.

## Expected order and timing dependency

HC-1 is integrated source and the person-start route exists; qualification of
that changed configuration is the unfinished work. The achievable order is:

1. The owner's new supplier-only capture authorization controls whether the
   first actual observation can occur. The bounded proposal above is ready
   for that decision independently of CE/S6 completion.
2. If authorized, make that one capture and inspect its actual outcome. Only a
   successful, complete packet permits the provider-equivalent recount and
   context/output reconciliation; refusal returns a finding without retry.
3. Select and rebuild a corrected native/frontend/wasm candidate and prepare
   the applicable isolation/lifecycle and W9-A packet. CE/S6 keeps priority on
   shared build/qualification resources. The exact selected candidate and
   remaining proof, rather than the old executable, determine readiness.
4. Bring the concrete native/model/credential and resource scope to the owner
   under D4 before execution. Its authorization, effective context/readiness,
   headroom, owned cleanup and the actual observed results control the joined
   live W9-A qualification. Arrange the required personal Apply separately;
   neither a scripted click nor an unattended run supplies it.
5. Extend to the full reference system only after the missing catalog,
   authoring and mechanics paths in the table are implemented and verified,
   with owner selections for constrained lanes when needed. Bind App receiving
   evidence to the agreed concrete host candidate; retain its old adoption
   until then. A successful W9-A is a prerequisite result, not the full-system
   publication result.

No credible completion date is established. The next owner dependency is the
single HC-1 supplier-only capture decision, followed by its actual packet and
recount. Native qualification additionally depends on candidate/build selection
and its separate concrete run authority; full P5 depends on the capability
gaps and personal participation above. These dependencies leave P5 open without
changing CE/S6 priority or making a new delivery commitment.

ScopeOfWork, acceptance criteria, dependency conditions and implementation are
unchanged. This is a current input/coverage account for an identified consumer;
checks and review outcomes belong in its PR. It records no run, grant,
acceptance, issuance, release or receiving adoption.
