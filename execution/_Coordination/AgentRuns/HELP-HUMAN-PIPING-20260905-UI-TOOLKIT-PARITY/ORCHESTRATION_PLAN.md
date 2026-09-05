# HELP_HUMAN orchestration plan

RunID: `HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY`
Version: 1 (frozen initial plan; supersede by an explicit later version)
RecordedUTC: 2026-09-05T03:45:22.120653+00:00
Run state: IN_PROGRESS
ScopeSelectionAuthority: HUMAN
GraphSelectionAuthority: AGENT_0
Posture: MIXED
IntegrationOwner: HELP_HUMAN (cross-manager fan-in only; no source writes)

## Objective and accepted basis

Implement the accepted 24-row toolkit vocabulary and shared human/agent operation semantics within existing project authority, starting from deliverable specifications in `{WORKING_ROOT}/execution/`. This is derivative coordination evidence based on accepted HEAD `740569598f9d00440636b8ea25264127f418e4ec`, decomposition revision 0.12 / SCA-009 and DAG-010, with SCA-008 / D-58 preserved. Exact conversational authority and recommendation are in OWNER_DIRECTION.md. HEAD and branch `codex/piping-ui-toolkit-parity` were verified by N0; other basis identifiers are parent-declared.

## Frozen work graph

| Node | Role and scope | Dependency / disposition |
|---|---|---|
| N0 | Direct ephemeral Agent 2 coordination recorder | Initial plan and chat evidence only |
| N1 | WORKING_ITEMS, PKG-07 | Contract/inventory map first; UI implementation after Agent 0 accepts concrete interfaces |
| N2 | WORKING_ITEMS, PKG-16 | Operation/backend design intake first; implementation after Agent 0 contract acceptance |
| N3 | WORKING_ITEMS, PKG-02 | Model and unit contract support; exact source fence requires a versioned Agent 0 grant |
| N4 | WORKING_ITEMS, PKG-03 | Hanger-library support as necessary; exact source fence requires a versioned Agent 0 grant |
| N5 | WORKING_ITEMS, PKG-05 | Automatic self-weight support as necessary; exact source fence requires a versioned Agent 0 grant |
| N6 | WORKING_ITEMS, PKG-04 | Nozzle/support behavior as necessary; exact source fence requires a versioned Agent 0 grant |
| N7 | Fresh independent read-only review child | After implementation fan-in; reviews 100% of frozen integrated diff |
| N8 | CHANGE | After review PASS and required validation; scoped Git closeout, no merge |

## Sequencing and concurrency

Phase A permits N1 contract inventory and N2/N3 design intake concurrently, using disjoint evidence writes. Implementation starts only after Agent 0 accepts concrete routing and contract returns. Existing-capability work precedes net-new backend work under SCA-009. N4–N6 are conditional nodes, activated when their support is necessary. Independent children use terminal fan-out/fan-in; findings that affect siblings are routed through the supervising manager/Agent 0. Siblings do not message or delegate directly. An Agent 2 never delegates.

Source integration owners are N1 for desktop and N2 for operation/schema surfaces within their exact fences; HELP_HUMAN performs cross-manager fan-in only and does not write source. N2 owns operation_applier and operation schemas. N1 owns desktop src and UI tests, with types coordinated against accepted N2 interfaces. These ownership labels are coarse responsibility boundaries, not permission to bypass exact child write fences. N3/N4/N5/N6 do not edit shared source until an exact disjoint fence is versioned by Agent 0. Every shared product_physics/schema/types write is serialized with one named owner. Agent 0 resolves ownership ambiguity before the affected write. Planning dependencies are acyclic through contract acceptance; mutual implementation dependencies must be surfaced as an SCC with recorded resolution, not silently ordered.

After implementation fan-in, N7 reviews the entire frozen diff as a fresh read-only `TASK + software-code-review` child. Remediate findings and obtain re-review PASS before DEC-025. Run applicable tests/checks and DEC-025 after independent review. N8 stages only scoped files and accepted evidence after checks; existing closeout gates apply and no merge is authorized. Runtime binding/D-58 remains gated.

## Required outputs and closure conditions

- Capability mapping covering all 24 accepted vocabulary rows.
- Concrete UI and operation implementation inside existing authority.
- Meaningful tests, required validation evidence, coverage ledger and deliverable residuals.
- Exact blockers, remaining scope, derivative currency and rerun requirements.
- Durable launch brief, status, return and available execution telemetry for each actual child; manager returns and Agent 0 fan-in.

Files alone do not close the tranche. Lifecycle/release/professional acceptance and roadmap R1–R3 remain outside this implementation grant. Agent-runtime binding remains gated under D-58; no successor mechanism is inferred. New unresolved engineering semantics or ownership return to Agent 0 while independent work continues.

## Delegation evidence calibration

Execution class: delegated-harness-native, using actual available native child sessions. Role/non-delegation evidence is instruction+config asserted, not mechanism-proven. No child execution is simulated by writing a brief. Record actual model only if exposed; otherwise `unavailable`. The recorder's model is unavailable. Its CHIRALITY_INSTRUCTION_ROOT environment variable is unset; it read the requested repository TASK base contract as an ephemeral Agent 2 and does not claim a normalized TASK-shell runtime.

## Record ownership

N0 owns these initial versions and `N0_coordination_record/**` only. Later instances own their explicit instance subdirectories. Future orchestration/acceptance changes require a new version or snapshot; preserve frozen versions. N0 does not append LOOP_RECEIPTS or mutate product, lifecycle, Git, or authority surfaces.
