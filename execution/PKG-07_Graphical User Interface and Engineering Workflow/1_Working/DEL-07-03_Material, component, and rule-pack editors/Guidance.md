# Guidance: DEL-07-03 Material, component, and rule-pack editors

## Purpose

This deliverable records the setup basis for GUI editors that let users create and review private materials, sections, components, load cases, supports, rule-pack references, and private libraries while keeping missing data, provenance, assumptions, and professional-boundary limits visible.

The core product value is not faster data entry by hidden defaults. The value is a reviewable workflow in which engineering inputs, sources, rule-pack readiness, and missing values are explicit before solving, checking, reporting, or sharing.

Sources: `docs/DIRECTIVE.md` sections 2 and 3; `docs/SPEC.md` section 7; `docs/_Registers/ScopeLedger.csv` row `SOW-021`.

## Principles

1. Preserve the open/public boundary. GUI editors may expose fields for user code data, private library values, manufacturer data, SIFs, flexibility factors, allowables, and rule-pack formulas, but the public project must not supply protected values as defaults.
2. Make missing data actionable. A blank or unknown required value should produce a visible finding tied to solve readiness, rule-check readiness, provenance, assumption review, nonlinear uncertainty, or IP boundary risk.
3. Keep the GUI downstream of validation contracts. Editors should route changes through application-service commands and domain validation so adapters, plugins, and UI controls cannot bypass units, provenance, diagnostics, or public/private data boundaries.
4. Separate editing state from project truth. Transient UI concerns such as selection, local form state, unsaved edits, validation display, and job progress should not be confused with the durable persisted model.
5. Avoid professional overclaiming. Editor labels and workflow states may describe software findings and user-rule-check results; they must not declare professional compliance or approval.

Sources: `docs/CONTRACT.md` invariants `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-DATA-2`, `OPS-K-UNIT-1`, `OPS-K-RULE-3`, `OPS-K-PRIV-1`, `OPS-K-AUTH-1`; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-03`, `AB-00-05`, `AB-00-06`, `AB-00-07`.

## Considerations

### Editor grouping

The register places multiple editors in this deliverable because they share a GUI domain. That grouping is acceptable for setup, but implementation should watch for split triggers:

- materials/sections/components become a private library management product area;
- load cases/supports require a separate model-editing workflow;
- rule-pack references require a separate sandbox/completeness workflow;
- validation UI tests become broad enough to require a dedicated test deliverable.

Source: `docs/_Registers/ContextBudgetQA.csv` row `DEL-07-03`.

### Missing data classes

The GUI should distinguish the reason a value matters. A solve-required missing physical input is different from a rule-check-required missing code value, weak provenance, user assumption, nonlinear convergence concern, or suspected IP/private-data exposure.

Source: `docs/SPEC.md` section 7.

### Private libraries

Private material/component/rule-pack libraries are user-controlled assets. The editor workflow can reference and validate them, but setup artifacts should not create sample private libraries, commit vendor data, or imply redistribution rights.

Source: `docs/DIRECTIVE.md` section 6; `docs/CONTRACT.md` `OPS-K-PRIV-1`.

### Implementation TBDs

The exact GUI component library, GUI state-management library, dependency versions, rule expression grammar/library, public API transport, import/export format list, and physical project package/container remain implementation-level `TBD` unless a later sealed brief or human ruling resolves them.

Source: `_CONTEXT.md` section "Architecture Basis Injection"; `docs/_Decomposition/SOFTWARE_DECOMP.md` section 8.2.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| One editor suite vs split deliverables | Keep setup unified under SOW-021, but split future implementation if scope becomes too broad for one bounded GUI slice. |
| Fast entry vs safe review | Prefer visible incompleteness and provenance checks over auto-filled engineering defaults. |
| Local form convenience vs validation authority | Local form state can improve usability, but durable mutation should pass through service commands and validation envelopes. |
| Public examples vs realistic libraries | Use invented/public-permissive examples only; do not publish protected or proprietary material/component/rule data. |
| Rule-pack editing vs rule-pack reference editing | This deliverable covers GUI references and required-input visibility; evaluator grammar and sandboxing remain PKG-06 responsibilities. |

## Examples

No engineering examples with real code values are included in this setup pass.

Permitted illustrative patterns for future invented examples:

- A material form showing a density field as `TBD` with `UNKNOWN_SOURCE` until the user records provenance.
- A component form showing SIF/flexibility inputs as user-supplied fields with source status rather than public defaults.
- A rule-pack reference panel showing checksum, version, redistribution status, and missing required inputs before a rule-check status can be shown.

These are workflow examples only, not engineering values or code-check examples.

## Semantic Lensing Application

Pass 3 applied the semantic lensing register as a worklist. Source rereads used for substantive updates:

| Lensing item | Source reread before update | Disposition |
|---|---|---|
| Clarify scope split risk for the multiple-editor surface | `_CONTEXT.md` Context Budget QA; `docs/_Registers/ContextBudgetQA.csv` row `DEL-07-03` | Incorporated into "Editor grouping" and Procedure split check. |
| Clarify application-service command and durable/transient state boundary | `_CONTEXT.md` Architecture Basis Injection; `docs/_Decomposition/SOFTWARE_DECOMP.md` `AB-00-03`, `AB-00-05` | Incorporated into Principles and Procedure. |
| Clarify private/protected data boundary | `docs/CONTRACT.md` `OPS-K-IP-1`, `OPS-K-DATA-1`, `OPS-K-PRIV-1`; `docs/DIRECTIVE.md` sections 3 and 6 | Incorporated into Private libraries and Requirements. |
| Clarify no professional compliance claim | `docs/CONTRACT.md` `OPS-K-AUTH-1`; `docs/TYPES.md` section 4 | Incorporated into Principles and Verification. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during setup. | N/A | N/A | N/A | N/A | N/A |
