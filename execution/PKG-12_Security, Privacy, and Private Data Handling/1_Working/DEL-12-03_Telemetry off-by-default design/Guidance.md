# Guidance: DEL-12-03 Telemetry off-by-default design

## Purpose

This guidance explains how to interpret the telemetry boundary for OpenPipeStress. The product is local-first, public-repository content must avoid protected standards data, and private project/rule/component/material data stays user controlled. Telemetry is therefore not a default feature; it is either absent/no-op or explicitly approved, opt-in, and payload-limited.

## Principles

| Principle | Guidance |
|---|---|
| Default-off | Design the product so no telemetry path is active until an explicit opt-in is recorded. |
| No hidden cloud behavior | Do not introduce endpoints, upload queues, background jobs, or remote storage as implicit behavior. |
| Private data never leaves by telemetry | Treat project models, rule packs, code-specific values, reports, hashes, paths, secrets, private libraries, and protected standards content as forbidden telemetry content. |
| Allowlist over redaction-only | If telemetry is later approved, use a small approved event allowlist. Redaction can be a second guard, not the primary permission model. |
| Local evidence | Keep approval records, config state, and test evidence local and reviewable. |
| Human authority | Telemetry status must never imply professional engineering review, certification, sealing, or code compliance. |

## Considerations

- A no-op telemetry module is acceptable for MVP if tests prove that no default outbound behavior exists.
- A future opt-in design needs a human-approved allowlist before implementation. Without that approval, event names, endpoint details, vendor choices, and payload fields remain `TBD`.
- Configuration defaults should fail closed: missing, unset, unknown, or malformed values disable telemetry.
- Product diagnostics may report that telemetry is disabled or misconfigured, but diagnostic text should not echo private payload content.
- Plugins and adapters must not create their own telemetry side channel or bypass core privacy filters.

## Trade-offs

| Option | Benefit | Risk | Current disposition |
|---|---|---|---|
| No telemetry in MVP | Small privacy surface and simple verification. | Less automatic usage insight. | Preferred by SOW-037 notes and OI-008 until human approval changes it. |
| Opt-in telemetry later | Can support voluntary product-improvement metrics. | Requires strong payload governance and user trust. | Allowed only after explicit human/security approval. |
| Redaction-only telemetry | Easier to retrofit around broad events. | Higher leakage risk if payloads are assembled before filtering. | Not sufficient as the primary design. |
| Local diagnostics instead of telemetry | Preserves reviewability without transmission. | Users must share diagnostics intentionally for support. | Compatible with local-first policy. |

## Examples

Concrete config syntax and event schemas are `TBD` because the implementation surface is not selected in this setup deliverable. The only permitted example behavior at this stage is:

- absent telemetry config means disabled;
- user has not opted in means disabled;
- MVP may contain no telemetry implementation;
- any future telemetry payload is approved by allowlist and excludes private/protected engineering data.

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified in this setup pass. | N/A | N/A | N/A | N/A | N/A |
