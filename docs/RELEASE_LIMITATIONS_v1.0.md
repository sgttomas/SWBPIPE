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
- **What it means in practice for a user:** once the test case itself is solved (today it is refused whole; see Code), the user reads the test case's own results. A user who wants the test case in an envelope with the operating cases, for example the largest support load over test and operation, or a difference between the test case and another case, does not get it from the product and forms it outside from the published rows of each case.
- **Why:** v1.0 does not define what such a combination means. A test case uses its selected test contents and may hold its supports, so it can be a different system from the cases it would be combined with.
- **What the user does instead:** once the test case is solved, report it on its own; its result is marked as a test and states which supports were held. Until then the product offers no alternative.
- **Basis:** DEL-05-01 and DEL-05-02 [scopes](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), "Withheld in v1.0"; made standing by PR #244.
- **Status:** not yet declared.

### LIM-004 — The held-status subtrahend on the exact route

- **Id:** LIM-004
- **Exit criteria:** 1 (expansion with its basis).
- **Requirement affected:** DEL-05-02/REQ-013, a Should: on request the sustained operand of the operating minus sustained difference is solved again with every nonlinear support held in the state the operating case ended in.
- **Code:** today `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`, which refuses every declared difference basis on the exact route and this request with it. When the declared basis is admitted on the exact route, this request keeps a named refusal of its own and the code is recorded here.
- **What is refused:** a subtraction declaring `difference_basis: operating_minus_sustained` that sets `subtrahend_support_state` to `minuend_final`, in a model with a pressure contract. (Without the declared basis that member is refused as misplaced on either route, `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED`.)
- **What it means in practice for a user:** the pressure-free route offers the held-status subtrahend today, so a user whose model has pressure cannot obtain something a model without pressure can. Their operating minus sustained difference is formed from two cases each solved to its own support states.
- **Why:** it is not built for v1.0. The held solve on the exact route would be a further engine change with its own memory pricing, and the capability is a Should.
- **What the user does instead:** use a plain subtraction, with neither `difference_basis` nor `subtrahend_support_state`. Each operand is then solved to its own support states, and each case's own result publishes them. On the exact route today the subtraction's record gives each operand's load case, modulus selection and numerical standing; it does not yet set the operands' support states side by side or say where they differ, which DEL-05-02/REQ-014 and AC-005 require, so the user compares the two cases' published states. The same disclosure is missing from exact-route envelopes; comparing cases manually does not meet AC-005.
- **Basis:** DEL-05-02 [scope](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), "Withheld in v1.0", with DEL-04-07/REQ-012 for the held-status solve; made standing by PR #244.
- **Status:** not yet declared.

### LIM-005 — A shareable report for a model that uses the user's library records

- **Id:** LIM-005
- **Exit criteria:** 4
- **Requirement affected:** exit criterion 4, that a user releases a report for results on either route (DEL-08-01/REQ-019, REQ-022).
- **Code:** `REPORT-USER-RELEASE-LIBRARY-REFERENCED`
- **What is refused:** a shareable (user-released) report package for a model in which a material, section, component or hanger was applied from one of the user's private libraries.
- **What it means in practice for a user:** after applying a library record, for example after choosing a hanger from their own hanger catalogue for a support, the user cannot save the shareable package of that model's report. The shareable-package dialog says that the model uses a library record and that a shareable package cannot be saved for it. Saving, printing and packaging the local-private report are unaffected.
- **Why:** applying a record writes its values, and some of its reference text, into the model, and they appear in the report: as model inputs and, for a hanger, as its loads, travel and stiffness among the results. The user's statement covers their own project data; v1.0 does not establish that it also covers values taken from a library, which may be a manufacturer's.
- **What the user does instead:** save the local-private package of the same report, which carries the library references.
- **Basis:** DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md), REQ-019 (Content); DEL-12-02/REQ-016. The refusal was standing before this entry; the entry was added with PR #243, which corrected the scope text to state it. Whether a library reference is admitted under user release for v1.0 is not decided (DEL-08-01, "Not decided").
- **Status:** not yet declared.

### LIM-006 — A shareable report for a model that names the user's rule pack

- **Id:** LIM-006
- **Exit criteria:** 4
- **Requirement affected:** exit criterion 4, that a user releases a report for results on either route (DEL-08-01/REQ-019, REQ-022).
- **Code:** `REPORT-USER-RELEASE-RULE-PACK-REFERENCED`
- **What is refused:** a shareable (user-released) report package for a model that names one of the user's rule packs, whether or not a rule check has been run.
- **What it means in practice for a user:** after adding a rule-pack reference to the model and solving, the user cannot save the shareable package of that model's report; the dialog says that the model uses a rule pack and that a shareable package cannot be saved for it. A shareable report never shows rule-check outcomes. The local-private report is unaffected.
- **Why:** The application derives the shareable report from the bound model and solve and refuses detectable library and rule-pack marks; it does not establish the origin or redistribution rights of unmarked values. In v1.0 that derivation does not include a rule pack's identity, so a model that names a pack is refused and not reported without it.
- **What the user does instead:** First save a local-private report package if the rule check and its outcomes must be kept. Then remove all rule-pack references from the model, solve again and save the shareable report; with other mechanics inputs unchanged, the mechanics values are unchanged because the solver does not read rule-pack references or content.
- **Basis:** DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md), REQ-019 (Content), REQ-008; DEL-06-04/REQ-008. The refusal was standing before this entry; the entry was added with PR #243. Whether a rule-pack reference is admitted under user release for v1.0 is not decided (DEL-08-01, "Not decided").
- **Status:** not yet declared.

### LIM-007 — Lining, refractory and coating weight

- **Id:** LIM-007
- **Exit criteria:** 1, through the pipe-mass basis used by its analyses. Lining, refractory and coating are not named requirements of that criterion, and its specified reference system contains none.
- **Requirement affected:** PRD §12.5, "insulation, lining, refractory, and coating weight". Insulation weight is provided. Lining, refractory and coating weight are not.
- **Code:** none for the missing input: a pipe's section has no member for a lining, a refractory or a coating, and a model document that carries an unknown member there is refused by the model schema and by the solve. Two named refusals and one notice meet a lining datum where one can arrive: `MBF-IMPORT-SECTION-FIELD-UNSUPPORTED` (a CAEPIPE model file whose section has a non-zero lining density or lining thickness); `SECTION_BINDING_UNSUPPORTED` (a pipe bound to a shared section record that holds any property other than outside diameter and wall thickness); and, when a library section that holds a lining thickness is applied, that dimension is listed to the person as not copied.
- **What is refused:** the product has no input for the weight of a lining, a refractory or a coating on a pipe, and computes none. A lining's reduction of the bore is not modelled.
- **What it means in practice for a user:** for a lined, refractory-lined or coated line: unless the user represents it through supported inputs, this weight is absent from the pipe-mass inputs used to generate self-weight and static seismic loads and to assemble modal mass. Hanger sizing works from the loads of the user's chosen weight and operating cases. Response-spectrum analysis is not built yet; its scope requires the modal mass basis. The weight and centre-of-gravity summary inherits the omission. The contents weight is taken on the unlined bore, so it is too large by the contents the lining displaces. A CAEPIPE model file with a non-zero lining on any section is not imported.
- **Why:** the section and mass contract defines mass per length for metal, contents and insulation (DEL-03-08), and the solve reads a closed pipe section with those members. Lining and coating mass, and the bore a lining leaves, were not built for v1.0.
- **What the user does instead:** carry the weight in the insulation, as an equivalent mass model with limits. Enter an insulation thickness and an insulation density for which the insulation annulus's mass per length equals the true mass per length of the insulation, lining, refractory and coating together, calculated by the user and stated in their own records.
  - *Where the mass goes.* For members that retain their distributed pipe mass, this equivalent mass enters generated static seismic loads and modal mass on both routes, and self-weight when the user generates or refreshes and applies its loads. It does not add to a component's replacement weight or modal body mass, or to an objective connector's replaced span.
  - *Its limits.* On a span marked for generated wind, insulation thickness also sets the projected diameter: outside diameter plus twice that thickness. Use the actual external build-up and adjust the equivalent density to it. If the external build-up is zero, this workaround cannot add lining mass while preserving the correct generated wind diameter, and no equivalent input is established for that case. The modal model also places this mass in the external insulation annulus for torsional and rotary inertia; matching mass per length does not reproduce the actual lined section's inertia. The user must assess that approximation before relying on modal results.
  - *What it is called.* The model, self-weight evidence and weight summary identify this equivalent input as insulation; keep its true composition with the calculation. The current CAEPIPE model-file exporter refuses nonzero insulation thickness (`MBF-SI-SECTION-UNSUPPORTED`) and does not export section densities (`MBF-SI-SECTION-FIELD-NOT-EXPORTED`), so this equivalent mass model cannot be transferred through that export.
  - *What is not equivalent.* Raising the contents density to carry the weight: a hydrostatic test case is defined to take its contents density from the condition it selects (DEL-05-01/REQ-014), so weight carried in the section's contents density would not be in it. A weight entered as a load on the weight case: it is not in the generated seismic loads or in the modal mass.
  - *The bore.* To correct contents mass only, enter in the section the actual contents density multiplied by the remaining lined flow area divided by the bore area the selected route uses, and record that substitution. DEL-05-01/REQ-014 requires a hydrostatic test case to take its contents density from its selected condition, so that density would also need this correction. Hydrostatic test mechanics are not built yet; a non-test case's use of a condition's contents density is not decided. Both are currently refused by name. This does not change the model's bore geometry or pressure areas.
- **Basis:** DEL-03-08 [scope](../execution/PKG-03/DEL-03-08/ScopeOfWork.md), "Not provided in v1.0". The capability was absent before this entry; SWBPIPE's Agent 0 decided that it is not built for v1.0, added the entry with PR #291 and noted the exception in PRD §12.5.
- **Status:** not yet declared.

### LIM-010 — Outputs that need a full copy of the result cannot be made above 64 MiB, and report generation beyond the committed witnesses is not established

- **Id:** LIM-010
- **Exit criteria:** 4, 5, 8
- **Requirement affected:** Exit criterion 4: a user releases, saves and prints a report for results on either route (DEL-08-01/REQ-019, REQ-022 and AC-011). Exit criterion 8 through DEL-07-06/AC-007's use cases that end in a report, a comparison or a rule check. Exit criterion 5 for the comparison tables' CSV exports (DEL-08-04/REQ-014: "Every results table in the desktop, comparison tables included, exports to CSV").
- **Code:** `RESULT-DETAIL-WEBVIEW-LIMIT` when the full result or required run record exceeds 64 MiB; `RESOURCE_UNAVAILABLE` when a full-copy or report-command memory reservation is refused. Other report gates retain their own named refusals.
- **What is refused:** The desktop report, with its local-private and user-released report packages, requires a full copy of the result in the web view. Above 64 MiB that copy is refused with `RESULT-DETAIL-WEBVIEW-LIMIT`, so the report cannot be made through this route. The result stays held natively; its summary, tables, viewport and paged result detail remain available through the native readers, whose own query and memory refusals still apply. The same full copy is required, and its refusal carries the same code, for the comparison within a run and of the current run with a recorded run, rule checks, the rule-pack editor's declarations (including its solved-result selector offers), the result JSON and stress-neutral exports, the local FEA handoff, the Exports section's Runner JSON and Native JSON Package exports and Preview handoff summary, and the sample proposal. File > Save Handoff… does not need this full copy; this limit does not refuse that package. The CSV export of a comparison that needs the full copy is unavailable with that comparison; the other results tables' CSV exports do not need the full copy.
- **What it means in practice for a user:** For a result above 64 MiB the result can still be read in the desktop and none of these outputs can be made there. The run record is a second way to meet the same refusal: the loader these outputs share checks the current run's record against the same 64 MiB before it hands anything over, so a record above it refuses every one of them although the result fits. At or under the limit the report is formed in the web view from the full copy and submitted to the native host. The native exact-report renderer rechecks the submitted members after reserving the delivered-text parses and a working-memory allowance. That allowance is checked on committed witness inputs; it is not a proven bound for larger inputs. The maintained native test renders reports for five committed exact-route witnesses; their largest received result is 398,566 bytes (`pressure_2_realized_bend`). The cited exact-route witness tests do not establish report generation through the desktop for larger results or establish the pressure-free route's report capacity. They do not show that such reports will fail. Whether the reference system of release exit criterion 1 can be reported through the desktop is not established.
- **Why:** The product fixes the full-result-copy limit from web-view memory measurements. Reporting still builds whole-result structures in the web view, and the native renderer's working-memory allowance is established only for the tested witness inputs.
- **What the user does instead:** Reduce the cases and combinations or split the model as the full-copy refusal advises, then solve again. Passing the full-copy limit does not establish that the report can be completed.
- **Basis:** DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md), REQ-022 and AC-011; DEL-07-07 [scope](../execution/PKG-07/DEL-07-07/ScopeOfWork.md), "Held result" (the full-result gate and the displayable limit). The limit was standing before this entry; the entry was added with PR #289.
- **Status:** not yet declared.
