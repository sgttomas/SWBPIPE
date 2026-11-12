---
name: swbpipe-release-limitations-v1-0
description: Record v1.0 refusals and whether Agent 0 has declared each acceptable.
metadata:
  kind: instruction
  surface: requirements
---
# SWBPIPE v1.0 release limitations

**Status:** companion to `docs/RELEASE_TARGET_v1.0.md`, for exit criterion 3:
"Every remaining refusal of a capability these criteria require is listed in the
release limitations and declared acceptable by Agent 0. The entry states
the requirement the refusal affects, what that means in practice for a
user, and the reason, and the limitations are shown to the owner with the
request for the verdict. Declaring a refusal acceptable is a limitation
decision: it does not demonstrate the refused capability, and it does not
make an unmet criterion met." This is the single list. Each entry is drafted by the
lane that owns the refusal and names the deliverable that
governs it. Agent 0 declares whether the refusal is acceptable, under the
grant in `AGENTS.md` ("Posture for this phase"), and the entry's status
records the declaration. Acceptance, issuance and release remain the owner's
acts (F-PIP-3, release target "Reserved to the owner"). The release notes'
"Known Limitations And Open Risks" section (`docs/RELEASE_NOTES_TEMPLATE.md`)
is written from the entries declared acceptable.

## Entry form

Each entry is one subsection:

- **Id:** `LIM-nnn`, assigned in order and never reused.
- **Exit criteria:** the release target's exit criteria it affects, by number.
- **Requirement affected:** the requirement or capability of the exit criteria
  the refusal affects, by its identifier where one exists.
- **Code:** the refusal or warning code the product emits, if any.
- **What is refused:** in the user's terms.
- **What it means in practice for a user:** what the user cannot do, or must
  do differently, because of the refusal.
- **Why:** the engineering or source reason, in one or two sentences.
- **What the user does instead:** the supported alternative, if any.
- **Basis:** the deliverable that governs it.
- **Status:** `not yet declared` until Agent 0 declares; then `declared
  acceptable by Agent 0` or `declared not acceptable by Agent 0`, with the PR
  that records it. A refusal declared not acceptable is removed before
  release, or the criterion it affects stays unmet. A refusal later removed
  by a landed change is marked `withdrawn` with the PR; the entry is kept,
  not deleted.

Entries are added in the same PR that makes the refusal standing. The PR names
the entry, and the entry names the PR.

## Entries

### LIM-001 — The embedded agent does not author component modifiers

- **Id:** LIM-001
- **Exit criteria:** 17
- **Requirement affected:** exit criterion 17, that the embedded agent builds
  the reference system from a person's request (DEL-16-06/REQ-010); the
  reference system's realized bends and tee with user modifiers are those of
  exit criterion 1.
- **Code:** `OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED`, `OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED`
- **What is refused:** the embedded agent cannot write a bend or tee
  flexibility factor k, a SIF, a stiffness scaling or stiffness, or the
  modifier source, and cannot realize or insert a bend. Imported proposals
  cannot write them either.
- **What it means in practice for a user:** the agent does not complete the
  reference system on its own. During the journey the person realizes each
  bend and enters k, the SIFs and their source for each bend and tee; the
  agent's model summary lists the components awaiting them, and the exact
  route does not solve the model while a bend is unrealized.
- **Why:** the owner ruled that users enter their own k and SIF. The agent
  proposes a bend as a geometry marker and a tee without modifiers. Until the
  person realizes a bend, the exact route refuses to solve, and the
  pressure-free route solves it as a straight chord, and the agent says so: k
  does not affect the chord's stiffness, a supplied positive scalar SIF applies
  to the chord, and without one no intensified measure is published.
- **What the user does instead:** after applying the agent's components, realize
  each bend with Insert bend at corner and enter k, the SIFs and their source in
  the component inspector, then let the agent continue.
- **Basis:** DEL-16-06 (catalog version 4; author routes and the trust
  boundary); DEL-16-03 (operation authorship). Made standing by PR #27 (catalog v4) with
  the applier refusal from PR #23.
- **Status:** not yet declared.

### LIM-002 — A compensated terminal at a pressure junction

- **Id:** LIM-002

- **Exit criteria:** 1.
- **Requirement affected:** exit criterion 1, the reference system with pressure and a tee solving on the exact route with no admitted family refused (DEL-04-07/REQ-005, one route for every model); the refusal is the one DEL-04-07/REQ-006 names.
- **Code:** `EXACT_PRESSURE_SHARED_TERMINAL_JUNCTION_UNSUPPORTED` (blocking).
- **What is refused:** a pressure junction, meaning a node where two or more pressure regions of one load case end, where any of those regions declares its terminal at the node `separately_supported_or_compensated`.
- **What it means in practice for a user:** a model in which the pressure thrust of a region ending at a pressure junction, where at least two pressure regions of the case end, is declared taken by a separate support or compensating device does not solve on the exact route. The refusal names the case, node and region, and the user remodels that terminal before the case gives any result.
- **Why:** the junction's pressure load is the sum of its terminals' caps. A compensated terminal would remove its member's share from the pipe solve, so the fitting's load would be analysed incomplete without notice.
- **What the user does instead:** declare `transfers_to_wall` at every terminal of the node. Where a branch's thrust really is carried by a separate device, model that device as a connector or support. A compensated terminal at a region's own closure, away from any junction, is admitted.
- **Basis:** DEL-04-07 [scope](../execution/PKG-04/DEL-04-07/ScopeOfWork.md), tees and pressure junctions (RT-A3 increment 1); made standing by RT-A3 increment 1 (PR #45).
- **Status:** not yet declared.

### LIM-003 — A combination that includes a hydrostatic test case

- **Id:** LIM-003
- **Exit criteria:** 1.
- **Requirement affected:** the combinations of exit criterion 1 (DEL-05-02/REQ-010). The hydrostatic test case itself (DEL-05-01/REQ-014) is not affected.
- **Code:** none of its own yet. Today every hydrostatic test case is refused whole (`LOAD_CASE_TEST_CONDITION_UNSUPPORTED`). The change that admits the test case gives this combination its own named withholding and records the code here.
- **What is refused:** a combination, on any basis, that has a hydrostatic test case among its operands, directly or through another combination.
- **What it means in practice for a user:** the user reads the test case's own results. A user who wants the test case in an envelope with the operating cases, for example the largest support load over test and operation, or a difference between the test case and another case, does not get it from the product and forms it outside from the published rows of each case.
- **Why:** v1.0 does not define what such a combination means. A test case is solved with its supports held and with the test contents, so it is a different system from the cases it would be combined with.
- **What the user does instead:** solve and report the test case on its own. Its result is marked as a test and states which supports were held.
- **Basis:** DEL-05-01 and DEL-05-02 [scopes](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), "Withheld in v1.0"; made standing by PR #244.
- **Status:** not yet declared.

### LIM-004 — The held-status subtrahend on the exact route

- **Id:** LIM-004
- **Exit criteria:** 1 (expansion with its basis).
- **Requirement affected:** DEL-05-02/REQ-013, a Should: on request the sustained operand of the operating minus sustained difference is solved again with every nonlinear support held in the state the operating case ended in.
- **Code:** today `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`, which refuses every declared difference basis on the exact route and this request with it. When the declared basis is admitted on the exact route, this request keeps a named refusal of its own and the code is recorded here.
- **What is refused:** a subtraction that sets `subtrahend_support_state` to `minuend_final` in a model with a pressure contract.
- **What it means in practice for a user:** the pressure-free route offers the held-status subtrahend today, so a user whose model has pressure cannot obtain something a model without pressure can. Their operating minus sustained difference is formed from two cases each solved to its own support states.
- **Why:** it is not built for v1.0. The held solve on the exact route would be a further engine change with its own memory pricing, and the capability is a Should.
- **What the user does instead:** use the default subtraction, which is published with each operand's support states and says where they differ (DEL-05-02/REQ-014).
- **Basis:** DEL-05-02 [scope](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), "Withheld in v1.0", with DEL-04-07/REQ-012 for the held-status solve; made standing by PR #244.
- **Status:** not yet declared.
