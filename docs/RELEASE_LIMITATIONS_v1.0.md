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

DEL-04-05 AX-003 also requires unmet targets to be listed here. For an unmet-target entry, "What is refused" states what is not met, and the other fields describe the target and its practical effect. Its status records Agent 0's limitation declaration; neither listing it nor declaring it acceptable makes its criterion met or supplies acceptance, issuance or release.

The list also carries applicability limitations, which exit criterion 3 does not require. An applicability limitation is a method that the exit criteria rely on and the product applies without refusal, but whose application range SWBPIPE's verification has not established. It causes no automatic refusal. Its entry says so in its Kind field. "Code" names any diagnostic or recorded field that bears on it, or none. "What is refused" says that nothing is refused and states what the method omits, and the other fields describe the method and its practical effect. Its status records Agent 0's limitation declaration. Declaring an applicability limitation acceptable does not establish that the method is adequate for any model, does not demonstrate the unestablished range and does not make an unmet criterion met.

The list also carries omissions outside the exit criteria, which exit criterion 3 does not require either. An omission is a capability that no exit criterion requires, that the product does not provide, and for which nothing emits a refusal; it is listed so that the release notes state it. Its entry says so in its Kind field. "Exit criteria" and "Requirement affected" say none and why, "Code" says none, and "What is refused" says that nothing is refused and what the user cannot do. Its status records Agent 0's limitation declaration. Declaring an omission acceptable does not make an unmet criterion met.

The list also records known product defects: demonstrated failures to meet stated product requirements. A known product defect's entry says so in its Kind field and identifies the affected requirements, the observed consequences and any containment. Its status records Agent 0's limitation declaration. Recording a known product defect, or declaring it, does not repair it and does not make an unmet criterion met.

## Entry form

Each entry is one subsection:

- **Id:** `LIM-nnn`, assigned in order and never reused.
- **Kind:** present only for an applicability limitation: `applicability
  limitation`, with the statement that it causes no automatic refusal; or for
  an omission: `omission outside the exit criteria`, with the statement that
  no exit criterion requires the capability and nothing emits a refusal; or for a known product defect: `known product defect`, with the statement of whether it causes an automatic refusal.
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

Entries are added in the same PR that makes the refusal standing. A previously unlisted standing refusal is added when identified; its Basis distinguishes the refusal's existing standing from the PR that records the entry. An unmet-target entry is added in the PR that records the unmet target. An applicability limitation is added when identified; its Basis distinguishes the method's existing standing from the PR that records the entry. The PR names the entry, and the entry names the PR.

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
- **Status:** declared acceptable by Agent 0 (PR #363).

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
- **Status:** declared acceptable by Agent 0 (PR #363).

### LIM-004 — The held-status subtrahend on the exact route

- **Id:** LIM-004
- **Exit criteria:** 1 (expansion; its declared basis on the exact route follows after v1.0, LIM-038).
- **Requirement affected:** DEL-05-02/REQ-013, a Should: on request the sustained operand of the operating minus sustained difference is solved again with every nonlinear support held in the state the operating case ended in.
- **Code:** today `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`, which refuses every declared difference basis on the exact route and this request with it. When the declared basis is admitted on the exact route, this request keeps a named refusal of its own and the code is recorded here.
- **What is refused:** a subtraction declaring `difference_basis: operating_minus_sustained` that sets `subtrahend_support_state` to `minuend_final`, in a model with a pressure contract. (Without the declared basis that member is refused as misplaced on either route, `LOAD_COMBINATION_SUBTRAHEND_SUPPORT_STATE_MISPLACED`.)
- **What it means in practice for a user:** the pressure-free route offers the held-status subtrahend today, so a user whose model has pressure cannot obtain something a model without pressure can. Their operating minus sustained difference is formed from two cases each solved to its own support states.
- **Why:** it is not built for v1.0. The held solve on the exact route would be a further engine change with its own memory pricing, and the capability is a Should.
- **What the user does instead:** use a plain subtraction, with neither `difference_basis` nor `subtrahend_support_state`. Each operand is then solved to its own support states, and each case's own result publishes them. On the exact route today the subtraction's record gives each operand's load case, modulus selection and numerical standing; it does not yet set the operands' support states side by side or say where they differ, which DEL-05-02/REQ-014 and AC-005 require, so the user compares the two cases' published states. The same disclosure is missing from exact-route envelopes; comparing cases manually does not meet AC-005.
- **Basis:** DEL-05-02 [scope](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), "Withheld in v1.0", with DEL-04-07/REQ-012 for the held-status solve; made standing by PR #244.
- **Status:** declared acceptable by Agent 0 (PR #363).

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
- **Basis:** DEL-03-08 [scope](../execution/PKG-03/DEL-03-08/ScopeOfWork.md), "Not provided in v1.0". The capability was absent before this entry; SWBPIPE's Agent 0 decided that it is not built for v1.0 and added the entry with PR #291. PRD §12.5 is unchanged and still says the product shall support these weights; amending the PRD is the owner's, and this contradiction is put to the owner for that.
- **Status:** declared acceptable by Agent 0 (PR #363).

### LIM-010 — Outputs that need a full copy of the result cannot be made above 64 MiB, and report generation beyond the committed witnesses is not established

- **Id:** LIM-010
- **Exit criteria:** 4, 5, 8
- **Requirement affected:** Exit criterion 4: a user releases, saves and prints a report for results on either route (DEL-08-01/REQ-019, REQ-022 and AC-011). Exit criterion 8 through DEL-07-06/AC-007's use cases that end in a report, a comparison or a rule check. Exit criterion 5 for the comparison tables' CSV exports (DEL-08-04/REQ-014: "Every results table in the desktop, comparison tables included, exports to CSV").
- **Code:** `RESULT-DETAIL-WEBVIEW-LIMIT` when the full result or required run record exceeds 64 MiB; `RESOURCE_UNAVAILABLE` when a full-copy or report-command memory reservation is refused. Other report gates retain their own named refusals.
- **What is refused:** The desktop report, with its local-private and user-released report packages, requires a full copy of the result in the web view. Above 64 MiB that copy is refused with `RESULT-DETAIL-WEBVIEW-LIMIT`, so the report cannot be made through this route. The result stays held natively; its summary, tables, viewport and paged result detail remain available through the native readers, whose own query and memory refusals still apply. The same full copy is required, and its refusal carries the same code, for the comparison within a run and of the current run with a recorded run, rule checks, the rule-pack editor's declarations (including its solved-result selector offers), the result JSON and stress-neutral exports, the local FEA handoff, the Exports section's Runner JSON and Native JSON Package exports and Preview handoff summary, and the sample proposal. File > Save Handoff… does not need this full copy; this limit does not refuse that package. The CSV export of a comparison that needs the full copy is unavailable with that comparison; the other results tables' CSV exports do not need the full copy.
- **What it means in practice for a user:** For a result above 64 MiB the result can still be read in the desktop, unless its parse is priced above the reader's limit (LIM-018), and none of these outputs can be made there. The run record is a second way to meet the same refusal, for the outputs for which the loader checks the current run's record: the desktop report and its rendered view, the local-private and user-released report packages, the comparison of the current run with a recorded run, the result JSON and stress-neutral exports, the local FEA handoff, and the Exports section's Runner JSON and Native JSON Package exports and Preview handoff summary. For these the loader checks the current run's record against the same 64 MiB before it hands anything over, so a record above it refuses them although the result fits. For the comparison within a run, rule checks, the rule-pack editor's declarations (including its solved-result selector offers) and the sample proposal the loader does not check the record, and its size does not refuse them. At or under the limit the report is formed in the web view from the full copy and submitted to the native host. The native exact-report renderer rechecks the submitted members after reserving the delivered-text parses and a working-memory allowance. That allowance is checked on committed witness inputs; it is not a proven bound for larger inputs. The maintained native test renders reports for five committed exact-route witnesses; their largest received result is 398,566 bytes (`pressure_2_realized_bend`). The cited exact-route witness tests do not establish report generation through the desktop for larger results or establish the pressure-free route's report capacity. They do not show that such reports will fail. Whether the reference system of release exit criterion 1 can be reported through the desktop is not established.
- **Why:** The product fixes the full-result-copy limit from web-view memory measurements. Reporting still builds whole-result structures in the web view, and the native renderer's working-memory allowance is established only for the tested witness inputs.
- **What the user does instead:** Reduce the cases and combinations or split the model as the full-copy refusal advises, then solve again. Passing the full-copy limit does not establish that the report can be completed.
- **Basis:** DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md), REQ-022 and AC-011; DEL-07-07 [scope](../execution/PKG-07/DEL-07-07/ScopeOfWork.md), "Held result" (the full-result gate and the displayable limit). The limit was standing before this entry; the entry was added with PR #289.
- **Status:** not yet declared.

### LIM-012 — Some combinations of library findings are refused by the Diagnostics check

- **Id:** LIM-012
- **Exit criteria:** 4
- **Requirement affected:** Exit criterion 4: a user releases, saves and prints a report for results on either route (DEL-08-01/REQ-022 and AC-011).
- **Code:** `SESSION-DIAGNOSTICS-CHECK-LIMIT`, carried in the detail of `REPORT-EXACT-SECTIONS-MISMATCH`.
- **What is refused:** Rendering a natively bound exact-route or pressure-free Timoshenko report, or saving an exact-route or pressure-free Timoshenko local-private package, when the check reaches an appended library finding in a group whose reference sites give a shared finding different preceding appended findings. Shared finding text alone is not refused. The refusal says this is a limit of the check, not a fault of the report. The native save command also applies this check on route 010 before that route's library-reference refusal; the desktop's Save Shareable Report Package action refuses library-referenced models before submitting a request.
- **What it means in practice for a user:** it arises only where a library identifier or a record identifier contains a backtick together with the words of the finding's own sentence, so that two different references read as the same text; for example a record identifier ``r` of material library `M`` on one reference beside a library identifier ``M` of material library `L`` on another reference of the same entity, with the report carrying the first reference's library-not-held finding and the record-not-held finding that both references spell. Identifiers without a backtick do not meet it. Such an identifier can reach a model by two supported routes: a library document a person imports, whose library and record identifiers need only be nonblank, applied to an entity from the library panel; and an operation batch that writes an entity's provenance record, whose library reference identifiers need only be nonblank text. A model imported from a CAEPIPE file carries no library reference.
- **Why:** The check counts entries using a forest of finding keys and does not implement general matching. When the appended findings give a shared key different predecessors, that counting method does not decide the assignment, so the check refuses the group by name.
- **What the user does instead:** Give the library or record an identifier without a backtick, import the corrected library and apply its record again, or resolve the library finding. Refresh the library findings, solve the model again if it changed, then render or save again.
- **Basis:** DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md), production method (the admission of Diagnostics entries after the result's own). The refusal is made standing by PR #292.
- **Status:** declared acceptable by Agent 0 (PR #363).

### LIM-013 — A text value that reads as JSON naming a person-entered member is refused from the agent, an imported file and a requeued record

- **Id:** LIM-013
- **Exit criteria:** 8, 17
- **Requirement affected:** Exit criterion 17, that the embedded agent builds the reference system from a person's request (DEL-16-06/REQ-010), and exit criterion 8 through DEL-07-06/AC-007's use cases that import a proposal or requeue a stored record (DEL-16-03/REQ-013 and AC-007). It narrows PRD §11.1 FR-MOD-004, "Users can label states/models using flexible naming conventions without enforced prover status", and PRD §24 R7's exit criterion "agents can propose schema-valid operations", for a text of this kind.
- **Code:** none; the refusal is a message from the agent's executor, offline intake or requeue, naming the operation.
- **What is refused:** An operation from the embedded agent, from an imported proposal file or from a stored record requeued after reopening whose value is a text that reads as JSON naming a member only the person enters, for example a label `{"modifiers":{}}`. The text is refused like the member itself. The refusal says the value names the member and cannot be told apart from it, unless a refusal for the value's length, depth or member count comes first, in which case it names that bound and not the member; it does not say that a member was found.
- **What it means in practice for a user:** An imported file or an agent proposal carrying such a label is refused whole for that operation, and nothing of that batch is queued. A stored record carrying one is not requeued, and it stays stored. That includes the person's own record: once it is stored and the project reopened, its author cannot be verified, so it is refused like anyone's, and the person enters the text again in the inspector.
- **Why:** The web view cannot tell literal text from a person-entered member without the engine's field kinds. Only the operation engine's field rules know which fields it writes as literal text, and the web view has no export of them.
- **What the user does instead:** Enter the text yourself in the inspector, which is not refused, or change the label so that it does not read as JSON naming such a member.
- **Basis:** DEL-16-03 [scope](../execution/PKG-16/DEL-16-03/ScopeOfWork.md), REQ-013 and AC-007; DEL-16-06 [scope](../execution/PKG-16/DEL-16-06/ScopeOfWork.md) (author routes and the trust boundary). Made standing by PR #261.
- **Status:** not yet declared.

### LIM-014 — The nonlinear operating reference rack is refused from 1,775 nodes

- **Id:** LIM-014
- **Exit criteria:** 9
- **Requirement affected:** DEL-04-05/AC-001's nonlinear operating profile: on the 2,000-node nonlinear reference rack, on a 16 GiB base machine, the measured clock is at most 60 s, with the solve within the 12 GiB per-invocation budget.
- **Code:** `NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED` (blocking).
- **What is refused:** a solve, in either solver mode, of a model with a nonlinear support whose estimated leading dense storage exceeds the provisional limit. The nonlinear operating reference rack exceeds it from 1,775 nodes, so criterion 9's nonlinear profile cannot be run at 2,000 nodes.
- **What it means in practice for a user:** a model with a nonlinear support above the estimate's limit is refused with that code before any case is solved; for the nonlinear operating reference rack, that is from 1,775 nodes.
- **Why:** the sparse mode keeps the dense active-set loop's storage estimate as a provisional admission cap. That cap is not an established bound on the sparse iteration's memory (DEL-04-07, Completion and Reliance Basis, "Not met").
- **What the user does instead:** the refusal's message names no alternative. A model under the limit passes this guard; that does not establish that it will solve or fit within the per-invocation budget.
- **Basis:** DEL-04-07 [scope](../execution/PKG-04/DEL-04-07/ScopeOfWork.md), Completion and Reliance Basis, "Not met" (the nonlinear admission cap). The refusal was standing before this entry; the entry was added with PR #311.
- **Status:** not yet declared.

### LIM-015 — Exit criterion 9 is not met

- **Id:** LIM-015
- **Exit criteria:** 9
- **Requirement affected:** DEL-04-05/AC-001: on the 2,000-node reference racks, on a 16 GiB base machine, the measured clock is at most 10 s for linear static and at most 60 s for nonlinear operating, with every solve within the 12 GiB per-invocation budget.
- **Code:** none.
- **What is refused:** nothing; this is an unmet target, not a refusal, listed under DEL-04-05 AX-003, and for it "what is refused" is read as "what is not met". No measurement shows criterion 9's linear static clock at most 10 s, or its nonlinear operating clock at most 60 s, on a base machine.
- **What it means in practice for a user:** only what was measured is established. On the development host, an Apple M5 Max with 128 GiB on macOS 27.2, at the engine of commit 16039fa62, the timing driver's entry stage for the 2,000-node linear rack took 457.9 and 479.5 s. After that stage, the route's footprint passed the 12 GiB budget. No base-machine performance measurement is supplied here. The nonlinear profile cannot be run at 2,000 nodes (LIM-014). Whatever the engine's time, the desktop cannot read a result whose parse is priced above 12 GiB (LIM-018). By extrapolation that includes the 2,000-node linear rack's result, so on main criterion 9's clock cannot reach first paint there.
- **Why:** the linear rack's time and memory at 2,000 nodes are measured but not yet attributed to the work that takes them.
- **What the user does instead:** none.
- **Basis:** DEL-04-05 [scope](../execution/PKG-04/DEL-04-05/ScopeOfWork.md), AC-001, AX-003 and "Measured and not met, as of 2026-11-13". The entry was added with PR #311.
- **Status:** not yet declared.

### LIM-016 — The desktop refuses report-package saves and user releases of results whose numerical standing is not eligible

- **Id:** LIM-016
- **Exit criteria:** 4
- **Requirement affected:** exit criterion 4, that a user can release, save and print a report for results on either route; DEL-08-01/REQ-024's present account ("Until the package save and the user release state numerical standing for every result the reader reads, they keep refusing a result that is not numerically eligible").
- **Code:** `REPORT-EXACT-STANDING-NOT-ELIGIBLE`, from the desktop's native package save, which the user release goes through; the desktop's package request refuses first with `CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`. The rendered report no longer refuses on this ground.
- **What now works:** the rendered report and its print view admit a solved exact-route result that the product reader reads, whatever its numerical standing, and state that standing: the standing line, each load case's numerical health, integrity code and reasons, and each nonlinear support's computed state and confirmation outcome (DEL-08-01 REQ-017, REQ-024). A result the reader cannot read, or whose mechanics are not solved, still refuses, by `REPORT-RESULT-NOT-READABLE` or `REPORT-MECHANICS-NOT-SOLVED`. On the desktop, the pressure-free rendered report still takes only an eligible result; the native render also admits a readable, solved pressure-free Timoshenko result whose standing is not eligible, which no desktop path sends. Observed through the desktop's report build step and the native render, with live custody, on two committed exact-route witnesses (`fixtures/reports/invented/report_standing_witnesses/`): a gap case, every load case quality unresolved, and a coupled sliding friction case, sensitive. The print view is derived from the rendered HTML (REQ-013); a native print was not exercised.
- **What is refused:**
  - On the exact route (`pressure`) and the pressure-free route's preview-physics result: the report package save and the user release require eligible standing, which on these routes means numerical quality `checks_passed` and no other finding from the production reader; passing this standing gate does not establish that the remaining report gates will pass (`core/reporting/result_export/src/reader_verdict.rs`, `standing()`). The four other quality values, and how each is known:
    - `unresolved`: observed. The gap witness above is refused by the desktop's package request.
    - `sensitive`: observed. The coupled friction witness above is refused the same way.
    - `not_assessed`: read from `standing()`'s rule, not observed.
    - `failed`: read from `standing()`'s rule, not observed.
  - On the pressure-source route the standing is decided by the source's own findings, not numerical quality. A pressure-source result whose diagnostics carry `NUMERICAL_INTEGRITY_SENSITIVE` is reportable: the maintained witness `pressure_source_1` is admitted with that warning named on its standing line.
  - A pressure-free Timoshenko report that carries the native source binding: a crossed pair is a reader refusal, so its render now refuses as not readable (`report_render_pf_timoshenko_tests.rs`).
  - On the pressure-free routes, the desktop's package request refuses a result whose standing is not eligible (`CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE`), and so does the headless runner, before any package, at its result-envelope production; the native save has no eligibility gate of its own there. Such a package carries no standing record (DEL-08-01 REQ-024's present account).
  - An unbound pressure-free report reaches the renderer without the binding (DEL-08-01 line 149, which leaves open "whether such a report should exist or be bound"). It carries no standing record (DEL-08-01 REQ-024's present account). What it does with an ineligible result is not established here.
- **What it means in practice for a user:** a solved model whose results the product marks numerically unresolved or sensitive (on these routes) shows those results in the desktop, with their standing. On the exact route it can now be rendered and printed as a report that states that standing; on the pressure-free routes it still cannot. It cannot yet be saved as a report package or user-released: those still refuse its standing. The refusal of the package request is observed; the user release, which goes through the same package save, was not exercised for these results.
  - Observed at source revision `9ce545040`, before the rendered report admitted such results; the observations in this bullet and the three below it describe the refusal as it stood then, at admission and rendering. Observed in both solver modes, on two test-only overlays of the reference system of exit criterion 1. Each is the committed fixture plus RS-R1, RS-CE1, RS-LO1 and RS-FR1, with RS-FR1's normal from the interim linear RS-FR1V:
    - with RS-G1, every case is unresolved (LIM-017);
    - without RS-G1, every case is sensitive (LIM-017).
  - Both are refused.
  - So neither tested overlay gives an exact-route result whose report package is admitted. Neither contains every support the reference system specifies. With RS-FR1's intended one-way normal source, the same overlays solve and are not confirmed: the confirmation admits only a base restraint as a derived normal's source. So without RS-G1 every case is sensitive with that named reason, and with RS-G1 every case is unresolved. A maintained test observes these standings (`core/product_physics/tests/rt_a3_s6_reference_system.rs`); their report package refusal is read from `reader_verdict::standing()`'s rule, not observed, and no test renders their report or applies the reader to them.
- **Why:** staged implementation. The rendered report states numerical standing (DEL-08-01 REQ-024) and admits such exact-route results first; the package save and the user release take the same admission when their own changes land (their package version change, and the user-release record's vocabulary, which today names ineligible exact-route results as not carried).
- **What the user does instead:** for an exact-route result, renders and prints the report from the rendered view, with its standing stated. A report package or a user release of the same results still needs eligible standing; source binding and the other report conditions still apply.
- **Basis:**
  - DEL-08-01 [scope](../execution/PKG-08/DEL-08-01/ScopeOfWork.md) REQ-017 and AC-009;
  - `apps/desktop/src-tauri/src/exact_source_binding.rs` step 4 (`SourceAdmission`: the render admits a readable solved result; the package save keeps eligibility);
  - the connecting-path tests `apps/desktop/src/features/report/reportStandingWitness.test.tsx` and `report_render_exact_tests.rs` (`a_readable_solved_result_that_is_not_checks_passed_renders_with_its_standing`).
  - The observation was made in a test-only probe at commit `9ce545040`. The probe called the desktop's production admission and rendering, with custody delivered by a scripted host and the straight witness's report material reused. It did not exercise a desktop journey, a file write, printing or a user-release interaction.
  - Both overlays' results were refused at step 4 in both modes. The committed reference system passes the standing gate; it then fails the reused report's source-mismatch check. The genuine straight witness, the control, is admitted and rendered.
  - The refusal was standing before this entry; the entry was added with PR #313.
- **Not met by declaring it:** declaring this refusal acceptable does not meet exit criterion 4 for these results, and does not make an unmet criterion met.
- **Status:** declared not acceptable by Agent 0 (PR #363).

### LIM-017 — Nonlinear results of a model with a gap support are published unqualified and unresolved

- **Id:** LIM-017
- **Exit criteria:** 1, 4 (and 18, whose outcome rule it follows)
- **Requirement affected:**
  - exit criterion 1, that the reference system, with a gap, lift-off and friction among its supports, solves on the exact route with no admitted family refused;
  - exit criterion 4, through LIM-016;
  - DEL-04-01/REQ-016 and DEL-04-07/CLM-002, the confirmation of a converged nonlinear state;
  - DEL-04-01/REQ-015, the case integrity check.
- **Code:** `NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED` (warning), with numerical quality `unresolved`. The reader's standing gives `NUMERICAL_INTEGRITY_NOT_QUALIFIED` and `NUMERICAL_CASE_EVIDENCE_INCOMPLETE`.
- **What is refused:** every load case that is published from a solve whose nonlinear input contains a `gap` support is published with its mechanics "for inspection only". Some models with a gap are refused before any case is published, for reasons of their own: an invalid friction normal source (`NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID`), the nonlinear memory guard (`NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED`, LIM-014), failed qualification or non-convergence.
  - **Model outside strict-gap scrutiny:** if the model has any of these, strict-gap proof is unavailable:
    - a non-gap nonlinear support (lift-off, friction, one-way);
    - a connector;
    - a realized (curved) bend;
    - a body outside the objective welded unreleased straight-frame family;
    - a friction normal source, authored or derived;
    - an applied sliding-friction force.
    
    Its message reads "strict-gap proof supports only gap-only objective straight-frame bodies with declared nodal loads and linear springs; mixed/curved/connector/affine source capability remains unqualified" (`core/solver/nonlinear_integration/src/structural_adapter.rs`, about lines 3379 to 3387).
  - **Model inside it:** the strict-gap proof selects the support state, and displacement and support reactions are selected at exact ratio. Member fields are recovered from projected binary64 displacement, and that member recovery is not qualified. The case is published with the same code (`core/product_physics/src/lib.rs`, about lines 6065 to 6080).
  - **Either way:** the support states of a gap model are not confirmed. The confirmation runs only on a qualified non-gap state (`lib.rs` about line 7676, `confirm && passed && !gap`). No member quantity is certified. A combination of such cases is published "not assessed against a stated bound … inspection results", and does not count toward eligibility.
- **What it means in practice for a user:**
  - The user sees displacements, reactions, member forces, stresses and support states for every case, under a red "quality unresolved" badge with the text that the support-state solution is kept for inspection (DEL-07-05 REQ-023).
  - None of it is presented as checked or confirmed.
  - A report package or user release of it is refused (LIM-016); its rendered report states each case's standing (DEL-08-01 REQ-024).
  - Observed at source revision `9ce545040`, in both solver modes, on a test-only overlay of the reference system of exit criterion 1. The overlay is the committed fixture plus RS-R1, RS-CE1, RS-LO1, RS-FR1 (on the interim linear RS-FR1V) and RS-G1. Every case and both subtraction combinations behave this way.
- **Why:** the strict-gap proof covers only gap-only objective straight-frame bodies with declared nodal loads and linear springs. Member recovery is not qualified for any published gap case, inside that scope or outside it. The state confirmation does not cover gap supports.
- **What the user does instead:** none for a model that needs a gap. Removing the gap does not give an eligible result on the overlay tested. The same test-only overlay without RS-G1 was observed at source revision `9ce545040`, in both solver modes:
  - each case's converged support state is confirmed ("established at 128-bit precision by one formation-checked solve; each support's state has a sign margin larger than its bound, and a sliding friction force satisfies the friction law of that state within its bound");
  - its certified rows are published;
  - both subtraction combinations are assessed and partially established. Their certificates keep apart groups established, groups not established, groups not assessed (below the floor) and rows not covered by any group. Rows not covered are inspection results, not withheld rows. The combination evidence states that these rows do not count toward the document's numerical eligibility.
  - Each subtraction also withholds two RS-EJ1 tie-rod rows. The committed reference system withholds the same rows, so they belong to the model.
  - The S11-G recovery guard demotes every case to `NUMERICAL_INTEGRITY_SENSITIVE`. At that revision it counted 495 (RS-SUS), 141 (RS-OPE) and 285 (RS-SAM) groups of the certified rows that "exceed the 1e-9 member-relative criterion or have an unavailable comparison". Its diagnostic names the first six in record order.
  - A scratch record of every group at that revision found every failing group a finite exceedance, from 1.001 to 674 times the criterion.
  - These are the guard's bounds. A bound is not a measured error.
  - So the case standing is not eligible; a report package or user release of it is refused (LIM-016), and its rendered report states that standing.
- **Basis:**
  - the cited source: `core/solver/nonlinear_integration/src/structural_adapter.rs` and `core/product_physics/src/lib.rs`;
  - `core/product_physics/tests/rt_a3_s6_reference_system.rs`. It asserts the unqualified recovery basis on every case of the overlay with RS-G1. Its known-limitation witness observes the overlay without RS-G1: every case confirmed and `NUMERICAL_INTEGRITY_SENSITIVE`, with the guard's counts printed, not asserted. Both are test-only overlays.
  - `validation/references/w16_nonlinear_motion/README.md` (about line 155): "Gap cases (R1) are never confirmed".
  - The behaviour was standing before this entry; the entry was added with PR #313.
- **Not met by declaring it:** declaring this acceptable does not demonstrate exit criterion 1 or 4 for the reference system, and does not make either met. Criterion 1 is also not demonstrated for other reasons, assessed separately: the families and analyses pending in the reference system's ledger, and the interim linear friction-normal support.
- **Status:** not yet declared.

### LIM-018 — A solved result whose parse is priced above 12 GiB cannot be read in the desktop

- **Id:** LIM-018
- **Exit criteria:** 1, 4, 5, 8, 9
- **Requirement affected:** DEL-07-07/REQ-013 and REQ-015: after a solve, the display reads the held result's verdict, summary and pages and paints them. Through them, every output of a solved result, and DEL-04-05/AC-001's clock, which ends when the result is verified and first painted.
- **Code:** `RESOURCE_UNAVAILABLE`, with the memory pool's refusal `ingress_exceeds_budget`, from the native reader's calls on the held result.
- **What is refused:** reading a solved result in the desktop when the memory priced for parsing it is above 12 GiB. The solve completes and the result is held natively. The reader's verdict, the summary, the run record and the result pages are not formed, so the result cannot be shown, recorded or used for any output.
- **What it means in practice for a user:**
  - What is proven from source: the reader's verdict, the summary, the run record's construction and the first build of the result-detail index each admit the whole result's parse, under a lease priced from the text before parsing, and refuse it above the smaller of the user's per-invocation budget and 12 GiB. A budget the user sets above 12 GiB does not raise this limit.
  - What is measured at main's result form, by the after-worker harness (PR #320; built at 6478772, which is main 1f4d322fe with only the harness test added): the linear reference rack, with its seven cases and five combinations, published 487,604,496, 980,750,268 and 1,963,121,108 bytes of result text at 250, 500 and 1,000 nodes. The parse was priced at 8.27, 8.26 and 8.26 times the text. At 1,000 nodes the reader refused the first whole-result parse, which needed 16,220,267,648 bytes against 12,884,901,888. The measurement used synthetic headroom (this host's physical memory, reported as both capacity and free headroom) to isolate the budget limit.
  - What is extrapolated from those measurements: at the same rate the limit falls near 795 nodes of that rack, and its 2,000-node result would be priced at about 32 GB. A model with fewer cases and combinations reaches the limit at more nodes.
- **Why:** the reader forms the whole result as a parsed tree and admits that tree's priced memory before parsing. The parse lease is bounded by the owner's 12 GiB per-invocation default (`apps/desktop/src-tauri/src/memory_pool.rs`, "Policy (fixed by the owner)") whatever budget is set.
- **What the user does instead:** reduce the cases and combinations in one run, or split the model, then solve again.
- **Basis:** DEL-07-07 [scope](../execution/PKG-07/DEL-07-07/ScopeOfWork.md), REQ-013, REQ-015 and "Held result"; DEL-04-05 [scope](../execution/PKG-04/DEL-04-05/ScopeOfWork.md), the clock. The refusal was standing before this entry, from the native custody of PR #59. The entry was added with PR #319, and its measured sizes with PR #320. The exact size at which main's linear rack is first refused is not measured: the parse was admitted at 500 nodes and refused at 1,000.
- **Status:** not yet declared.

### LIM-019 — A friction support whose normal comes from a one-way support can cycle although an admissible state exists

- **Id:** LIM-019
- **Exit criteria:** 1
- **Requirement affected:** DEL-04-04/REQ-012, that the nonlinear support classes converge on the validation cases and that the reference system's gap, lift-off and friction supports converge; DEL-04-07's assembled nonlinear iteration. The refusal satisfies DEL-04-04/AC-004: a case whose states cycle is reported as not converged and never published as converged. It does not establish REQ-012's convergence.
- **Code:** `NONLINEAR_SUPPORT_NONCONVERGENCE` and `SOLVER_SYSTEM_BLOCKED`, both blocking; the mechanics status is `MODEL_INCOMPLETE`.
- **What is refused:** The current active-set iteration cycles on a demonstrated friction case whose normal comes from a one-way support, although an admissible equilibrium exists. Both solver modes report non-convergence and refuse the mechanics result. The refusal does not establish that the model has no equilibrium.
- **What it means in practice for a user:** for such a model no case is solved and no result row is published. The non-convergence message names the supports still changing and each support's state at the last iteration. How often this arises in ordinary models is not established.
- **Why:** the iteration updates every support's state at once. In the solver-level witness, sticking makes the one-way support pull so both release; with both free, lateral motion re-engages the one-way support; with the friction support free and the one-way support bearing, the friction support's zero trial force makes it stick again, repeating without reaching the admissible bearing-and-sliding state.
- **What the user does instead:** no workaround has been demonstrated for this case.
- **Basis:**
  - DEL-04-04 [scope](../execution/PKG-04/DEL-04-04/ScopeOfWork.md) (REQ-012, AC-004) and DEL-04-07 [scope](../execution/PKG-04/DEL-04-07/ScopeOfWork.md) (the nonlinear iteration).
  - Solver-level witness: `core/solver/nonlinear_integration/src/oneway_source_tests.rs`, `a_lift_off_friction_cycle_ends_in_named_non_convergence`. The model is an off-axis chord (3, −4, 0) under −100 along X and +3 along Y, against a positive-reaction one-way support at μ = 0.3. In both solver modes the states repeat with period three over 16 iterations. The admissible state on the nominal chord, computed exactly by the independent numerical reading of the change that records this entry, is N = 23915/773, T = 14349/1546, u_x = −126125/18552.
  - Product-level witness: `core/product_physics/tests/friction_one_way_normal_source.rs`, `a_lift_off_friction_cycle_is_refused_by_name_with_no_result`. It is the same chord geometry as a model document, with this file's pipe section and material, so its stiffness is not the nominal chord's. In both modes it gives the two codes above and no result row.
  - A separate observation, not generalized: the frozen reference's exact lift-boundary case LIFT-BOUNDARY (`validation/references/friction_one_way_normal`) converges in dense mode, and reaches its four-iteration cap in sparse mode, where it is refused with the same codes. This is not a general failure of sparse mode, and dense mode is not a workaround.
  - The refusal became standing with PR #326, which admits a one-way or lift-off support as a friction normal source and records this entry. Before it, such a model was refused at model build (`NONLINEAR_FRICTION_NORMAL_SOURCE_INVALID`).
- **Status:** declared acceptable by Agent 0 (PR #363).

### LIM-020 — Modal analysis refuses a model with Timoshenko straight members, which exact-route authoring writes by default

- **Id:** LIM-020
- **Exit criteria:** none. Exit criterion 1 listed modal and response-spectrum analysis until Agent 0 moved them after v1.0 on 2026-11-20 (LIM-035); this entry applies when they are taken up.
- **Requirement affected:** the modal and response-spectrum analyses, which follow after v1.0 (LIM-035), for a model whose straight members are Timoshenko; DEL-04-09/REQ-006, which admits models on both routes; DEL-04-10, which works on DEL-04-09's modal basis. The reference system selects Euler–Bernoulli members explicitly (`fixtures/reference_system/SPECIFICATION.md`, "Route"), so this refusal does not prevent the criterion's demonstration on it. The Timoshenko exclusion narrows the supported modal domain relative to the same linear model described by PRD FR-SOL-016 and §§12.1 and 12.9; §12.9 also expressly requires named refusal of what is not admitted.
- **Code:** `MODAL_TIMOSHENKO_MASS_UNSUPPORTED`, blocking (`core/product_physics/src/modal/prepare.rs`, "The modal interim refusals, each by name").
- **What is refused:** forming a modal solution for a model that selects Timoshenko straight members (`timoshenko_annulus_energy_v1`), on either route: on the exact route through `pressure_contract.shear_model`, on the pressure-free route through `model_settings.shear_model`. Nothing is formed. Modal analysis has no desktop surface and no engineering admission yet (DEL-04-09, "Admission"). A user meets the refusal through the headless runner's experimental `modal` command; otherwise it is met only in the implemented modal preparation and its readers.
- **What it means in practice for a user:**
  - On the exact route, new pressure authoring writes Timoshenko straight members by default (`apps/desktop/src/features/pressure-authoring/pressureContract.ts`, `DEFAULT_SHEAR_MODEL`; `core/model_operations/operation_applier/src/pressure_authoring.rs`). A model authored with the default is refused modal analysis, and so has no basis for response-spectrum analysis.
  - Selecting Euler–Bernoulli members explicitly changes the mechanical model: straight members lose shear flexibility in every analysis of that model, static ones included. It is not an equivalent alternative.
  - On the pressure-free route Timoshenko is opt-in, so a document that does not select it is not affected.
- **Why:** the implemented modal mass is Euler–Bernoulli consistent mass with Rayleigh rotary inertia, so a model selecting shear-flexible Timoshenko straight members is refused rather than paired with it. A Timoshenko consistent mass is committed in DEL-04-09 (REQ-001) and its element and exact element references exist (PR #335), but the product's mass assembly, the integrity re-formation and the independent readers do not yet form it, so the refusal stands (REQ-006).
- **What the user does instead:** no user-facing modal alternative is available yet. Explicitly selecting Euler–Bernoulli removes this particular preparation refusal and changes the straight-member theory for the whole model; the headless runner's `modal` command runs a modal request only as an experiment, and modal analysis in the desktop, or as an engineering result on any surface, still awaits the Admission conditions of DEL-04-09.
- **Basis:** DEL-04-09 [scope](../execution/PKG-04/DEL-04-09/ScopeOfWork.md), REQ-001 ("Straight members", the Timoshenko consistent mass) and REQ-006 ("Refused by name", the temporary refusal while the integration is incomplete, with the narrowing against the PRD); DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md), on DEL-04-09's modal basis. The refusal was standing before this entry, from PR #94, which brought modal preparation to main; the exact-route Timoshenko authoring default predates it (T4-U8). The entry was added with PR #333. When a landed change removes the refusal, the entry is updated to the remaining gap. It is withdrawn only when DEL-04-09 AC-012's verification exists, the continuum Timoshenko benchmarks included.
- **Status:** not yet declared.

### LIM-021 — The application range of a realized bend's element is not established

- **Id:** LIM-021
- **Kind:** applicability limitation. It causes no automatic refusal.
- **Exit criteria:** 1
- **Requirement affected:** DEL-03-03's method for a realized bend: REQ-012, and AC-002's closed-form end flexibility of a circular curved beam with the user's factors on the two bending terms. Exit criterion 1 includes the realized bends of the reference system.
- **Code:** none. No refusal code is emitted for this limitation. A result records each realized arc's `beam_theory` as `euler_bernoulli`. Under the pressure-free route's Timoshenko selection, a result also carries the information diagnostic `PF_TIMOSHENKO_REALIZED_ARCS_EULER_BERNOULLI`. The diagnostic states that realized arcs omit shear deformation. Neither the diagnostic nor the recorded field states the curved-bar or ovalization omissions, or that the element's application range is not established.
- **What is refused:** nothing is refused. A realized bend is solved with the end flexibility of a thin curved beam. That flexibility includes bending, multiplied by the user's in-plane and out-of-plane flexibility factors, together with axial and torsional flexibility. It omits two kinds of term, of the same order as each other:
  - transverse shear deformation;
  - the curved-bar terms: the offset of the neutral axis from the centroid, and the coupling between axial force and bending moment.

  The element also omits cross-section ovalization and its restraint. How much that matters, and how far the user's flexibility factor represents it, has not been established here. SWBPIPE's verification has not established an annular application range for this element.
- **What it means in practice for a user:**
  - The comparisons below depend on the loading and on:
    - the section's radius of gyration over the bend radius, r_g/R. For a thin wall r_g ≈ 0.354·D, so r_g/R is about 0.24 when R = 1.5·D and 0.35 when R = D;
    - r_g over the lengths of the legs that carry moment into the bend;
    - the section's wall ratio D/t.

    The user's flexibility factor multiplies the bending term those omissions are compared with.
  - No limit in these measures has been established below which the omissions are negligible.
  - Three illustrations follow, from three different kinds of evidence, kept apart. Each is a sampled comparison, not a bound. None gives the error of, or a dependable sign for, any individual result or reaction.
    1. **Exact elasticity, rectangular bar, one loading.**
       - *Geometry and loading:* a quarter-circle bar of rectangular cross-section in plane stress, with depth h and centreline radius R, Poisson's ratio 0.3. A radial force acts at one end; the curved faces are free. This is the classical Golovin/Ribière solution, which prescribes distributed tractions on the end cuts, not a clamped end.
       - *Compared:* the strain energy of the exact stress field against elementary beam energy built from the same section forces.
       - *Normalized as:* 100·(model energy / exact energy − 1). Positive means the model stores more energy, so it is more flexible.
       - *Results, at h/R of 0.1, 0.3 and 0.6:*
         - the shear energy with a straight bar's shear coefficient (5/6): +0.093%, +0.85% and +3.54%;
         - the total, thin bending without shear: −0.07%, −0.57% and −1.9%;
         - the total, thin bending with shear added: +0.19%, +1.7% and +7.0%.
    2. **Annular curved-bar model, one loading.**
       - *Geometry and loading:* a quarter-circle arc of annular section, outside diameter D and wall thickness t, under an in-plane radial force at its tip, the other end fixed. The sampled values are D/t = 5, 10, 20, 50 and 100, and R/r_o = 2, 3, 4, 6 and 10, where R is the centreline bend radius and r_o = D/2.
       - *Model:* elementary curved-bar energy with plane sections, the annulus's neutral axis in closed form, and the product's energy-matched annular shear factor (Poisson's ratio 0.3).
       - *Normalized as:* the change in the arc's radial tip flexibility from each omitted term, divided by its thin bending flexibility with a factor of 1.
       - *Results:* over the sampled D/t values from 10 to 100, the transverse-shear contribution is about +(5.10 to 5.20)·(r_g/R)² and the curvature contribution about −(2.50 to 2.56)·(r_g/R)². At D/t = 5 they are about +4.73·(r_g/R)² and −(2.61 to 2.66)·(r_g/R)². These summarize the stated samples. Positive means more flexible.
    3. **Invented assembled systems.**
       - *Geometry and loading:* a planar L-shaped pipe run, two straight legs of equal length L joined by a 90° bend of radius R. Both ends are anchored. The run is loaded by a free thermal growth of 0.1% of the anchor-to-anchor offset in each direction. D = 0.3 m, sampled at R of 1, 1.5 and 3 diameters, D/t of 5, 20 and 100, and factors of 1 and 5. The legs are the same thin beams in every model; only the bend's model changes.
       - *Compared:* the six anchor reaction components of both anchors (moments divided by L), between the thin bend and the bend with an omitted term added.
       - *Normalized as:* the largest change of any component, divided by the largest reaction component of the thin model. The sign shown is that of the change in magnitude of that largest reaction. The figure summarizes the whole run; it is not any individual reaction's error.
       - *Results:*
         - transverse shear alone: up to 4.9%, 0.36% and 0.02% for legs of 3, 10 and 30 diameters, decreasing the largest reaction;
         - the curvature terms, with the factor scaling the curved-bar bending term: up to 10.7%, 3.0%, 1.7% and 0.67% for legs of 3, 10, 30 and 100 diameters, increasing it;
         - both together: at legs of 3 diameters, the sign varied with geometry;
         - at a factor of 5, the curvature-only reaction-change measure with the factor on the curved-bar bending term was 1.18 to 4.73 times that with the factor on the thin bending term only, with the curvature correction and axial–moment coupling added unscaled, across the sampled runs.
  - The examined comparisons do not justify adding transverse shear alone as an accuracy improvement for annular bends. In illustration 1, adding shear alone moved the total energy further from the exact value than leaving it out, at every sampled ratio (h/R from 0.02 to 1.6). That is a counterexample to a general improvement; it does not show that adding shear is harmful throughout the annular domain.
  - Nothing shown here says that the present element is more accurate than an alternative, or that its errors are conservative.
- **Why:** DEL-03-03 defines the element as the thin curved beam's closed-form end flexibility, with the factors on the bending terms only, and VER-002 compares it with closed-form values of that same model (`validation/references/t4_i6`), which shows agreement with the model, not its adequacy for a given bend. No reference of a more complete model has been identified for annular bends, whether a beam model with shear and curvature terms or an elasticity solution, and what the user's flexibility factor would mean against such a model is not defined.
- **What the user does instead:** the flexibility factors are the user's own inputs, and whether they suit the fitting is the user's engineer's judgment (DEL-03-03). No product option changes a realized bend's formulation; the Timoshenko selection applies to straight members only.
- **Basis:**
  - DEL-03-03 [scope](../execution/PKG-03/DEL-03-03/ScopeOfWork.md): the method; REQ-012, AC-002 and VER-002. The element's formulation has been standing since realized bends were introduced. PR #334 records this entry.
  - Evidence, by pointer, in the inverse-norm lane's scratch: `/Users/ryan/dev/chirality-t3/scratch/swbpipe/inverse-norm-repair/arc-shear/`. It holds the formulation note `NOTE.md` and the scripts whose saved outputs give each figure above:
    - illustration 1: `golovin_energy.py` and `golovin_energy.out`;
    - illustration 2: `annulus_orders.py` and `annulus_orders.out`;
    - illustration 3: `chain_range.py` and `chain_range.out`, whose bend energy is checked against illustration 2's model by `cross_term_check.py` (`cross_term_check.out`).

    The note had an independent reading (`review/return.md`), and its corrections are integrated.
  - Agent 0 decided that a second-order curved-bar element is not to be built before v1.0. Its prerequisites are the factor's meaning against a curved-bar baseline, an independent annular reference, and an element that is formed consistently in every place the product forms it.
  - Declaring this limitation acceptable does not establish that the element is physically adequate for any bend.
- **Status:** not yet declared.

### LIM-022 — Response-spectrum analysis applies one ground motion to every support

- **Id:** LIM-022
- **Exit criteria:** none. Exit criterion 1 listed modal and response-spectrum analysis until Agent 0 moved them after v1.0 on 2026-11-20 (LIM-035); this entry applies when they are taken up.
- **Requirement affected:** DEL-04-10/REQ-011 and AC-009, independent (multi-level) support motion, a Should commitment of the deliverable that owns response-spectrum analysis, which follows after v1.0 (LIM-035). The reference system's spectrum request applies one spectrum to every support in X, Y and Z (`fixtures/reference_system/SPECIFICATION.md` §8.5), so this refusal does not prevent the criterion's demonstration on it. PRD §12.9 places response-spectrum seismic analysis in scope, after v1.0, without naming support-motion forms, and does not state independent support motion; the decision narrows the capability DEL-04-10 states within that broad requirement and contradicts no explicit PRD statement.
- **Code:** `SPECTRUM_INDEPENDENT_SUPPORT_MOTION_UNSUPPORTED`, blocking, committed by DEL-04-10/REQ-011 and AC-009. The spectrum table and supplied-contribution combination units exist, but no complete spectrum request entry emits this support-group refusal yet.
- **What is refused:** a spectrum request with more than one support group: different spectra for different groups of supports, with each group's own influence vectors. Every support moves with one ground motion per direction (uniform support motion, DEL-04-10 CLM-001).
- **What it means in practice for a user:**
  - Supports at different elevations or on different structures cannot be given different floor or building spectra in one analysis.
  - A common practice where group spectra are not available is to apply one envelope spectrum to every support. It is the user's engineer's judgment whether that suits the system; SWBPIPE does not determine it.
  - The pseudo-static part of differential support motion is seismic anchor movement, a static load case either way, and is unaffected.
- **Why:** Agent 0 decided that independent support motion is not built for v1.0. It is a Should commitment (REQ-011), and the reference system's spectrum request uses uniform support motion. REQ-011 stays the deliverable's commitment for a later release, with its refusals of a group that moves a skewed restraint and of CQC.
- **What the user does instead:** one spectrum per direction applied to every support, chosen by the user's engineer, for example as an envelope of the group spectra.
- **Basis:** DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md): CLM-001 ("Uniform support motion"), "Decided while response spectrum was a v1.0 requirement", REQ-011 and AC-009. The decision was made by Agent 0 under the grant in `AGENTS.md` ("Posture for this phase") and is recorded with this entry by PR #355. Whether the PRD should be amended to say so is the owner's; Agent 0 brings it to the owner.
- **Status:** declared acceptable by Agent 0 (PR #379).

### LIM-023 — A response-spectrum result cannot be combined with static load cases

- **Id:** LIM-023
- **Kind:** omission outside the exit criteria. No exit criterion requires this capability, and nothing emits a refusal for it.
- **Exit criteria:** none. Exit criterion 1 lists three-axis static seismic with SRSS and absolute-sum combination; response spectrum follows after v1.0 (LIM-035).
- **Requirement affected:** none of the release target. The reference system's occasional combinations, RS-OCC-SRSS and RS-OCC-ABS (key result KR-02), combine RS-SUS with RS-EQ-SRSS or RS-EQ-ABS of the static seismic terms RS-dEQX, RS-dEQY and RS-dEQZ (`fixtures/reference_system/SPECIFICATION.md` §8.2 and "Key results"). Its spectrum results (KR-09, EQ-7) stand alone. The load-case algebra's operands are load cases and combinations (DEL-05-02). The PRD states no combination of a spectrum result with static cases.
- **Code:** none. A spectrum result is not a load case or combination, so no combination can name it; there is no request to refuse.
- **What is refused:** nothing is refused. A combination such as sustained plus seismic cannot use a response-spectrum result as its seismic term; it uses the static seismic cases.
- **What it means in practice for a user:**
  - Sustained-plus-seismic and other occasional combinations are formed from static seismic cases only.
  - A spectrum result is read on its own, as unsigned combined magnitudes. Its signed modal contributions do not give a combined magnitude an algebraic sign. Any combination with sustained or other results is done outside SWBPIPE by the user's engineer.
  - Stresses from spectrum responses are not computed either (DEL-04-10 CLM-003), so no code check reads a spectrum result.
- **Why:** an unsigned combined magnitude cannot be added to a signed static result as if it were signed, and the rules for doing so (absolute sum or SRSS with the static case, per component) are not committed by any deliverable. No result the release target requires needs it, so Agent 0 decided that it is omitted for v1.0.
- **What the user does instead:** the static seismic cases and their combinations (DEL-05-01, DEL-05-02) for occasional combinations; the spectrum result on its own for the dynamic response.
- **Basis:** DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md): CLM-003 and "Decided while response spectrum was a v1.0 requirement"; its `deliverable.yaml` need on DEL-05-02, which applies only if a spectrum result is later offered as an operand. The omission was standing in DEL-04-10's CLM-003; Agent 0 decided it for v1.0 under the grant in `AGENTS.md`, and PR #355 records this entry. No PRD amendment is needed, because the PRD states no such combination.
- **Status:** declared acceptable by Agent 0 (PR #379).

### LIM-024 — Spectrum evaluation may refuse when its accuracy argument is inconclusive

- **Id:** LIM-024
- **Exit criteria:** none. Exit criterion 1 listed modal and response-spectrum analysis until Agent 0 moved them after v1.0 on 2026-11-20 (LIM-035); this entry applies when they are taken up.
- **Requirement affected:** DEL-04-10/REQ-001 and REQ-002: spectral ordinates interpolated by the spectrum's stated rule over positive finite tables and queries inside their range. Each published ordinate, pseudo-acceleration S and scaled s·S has a relative error bound of at most 1e-9 (REQ-008), conditional on IEEE 754 binary64 round to nearest and the library-function assumption below.
- **Code:** `SPECTRUM_EVALUATION_ACCURACY_UNSUPPORTED`, blocking, emitted by the spectrum table evaluation (`core/analysis/response_spectrum`, unit S2). There is not yet an end-to-end spectrum solve. The separate supplied-contribution combination refusal is described in LIM-025.
- **What is refused:** an evaluation for which this implemented error argument does not establish the stated relative accuracy for every published value, including when a computed approximation rounds to zero or outside the finite range. Nothing is published for that evaluation. The refusal is conservative: it is not proof that no sufficiently accurate binary64 answer exists. Invalid records and queries outside the table retain their own codes.
- **Assumption and evidence:** logarithmic operations rely on each platform `ln`, `ln_1p` and `exp` having relative error at most 2⁻⁵⁰. This is assumed, not checked or established by platform documentation, for Apple's libm on macOS, glibc on Linux and the Microsoft C runtime on Windows. The maintained higher precision sweep compares final results on the configurations actually exercised and recorded in local or hosted runs; passing it does not directly check or establish the primitive-function premise. A supported platform found to violate the constant reopens the premise and affected accuracy claims even if final-result tests still pass. Declaring this limitation would accept reliance on the assumption; it would not establish the assumption or demonstrate exit criterion 1.
- **What it means in practice for a user:** the known conservative refusals occur at binary64 range edges. An exact `1e-320` knot at scale 1 is refused despite having zero representation error, because the argument adds a worst-case half-subnormal spacing. A constant `f64::MAX` table may be refused at an interior query because the computed blend rounds above the finite range, although its exact answer is representable. Other refusals include true overflow or underflow of an ordinate, S or s·S. This does not promise acceptance of every exact normal-range value. No refusal at ordinary engineering magnitudes has been demonstrated by these witnesses.
- **Why:** Agent 0 requires the implemented argument to support each published value to 1e-9 relative under its stated assumptions, without an absolute floor. A worst-case error allowance exceeding that threshold warrants refusal, not a claim that accurate representation is impossible. The conservative policy is retained without an arithmetic repair; a later argument may justifiably publish the present counterexamples.
- **What the user does instead:** the current evaluation cannot supply this result. Use a method able to establish its accuracy; changing physical inputs solely to bypass the refusal would change the problem.
- **Basis:** DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md), "Decided while response spectrum was a v1.0 requirement". Agent 0 settled the conditional claim and conservative refusal policy under the grant in `AGENTS.md`; PR #367 records the exact wording and review.
- **Status:** not yet declared.


### LIM-025 — Spectrum combinations may refuse when their accuracy is not established

- **Id:** LIM-025
- **Exit criteria:** none. Exit criterion 1 listed modal and response-spectrum analysis until Agent 0 moved them after v1.0 on 2026-11-20 (LIM-035); this entry applies when they are taken up.
- **Requirement affected:** DEL-04-10/REQ-003 and REQ-004, combining signed modal and rigid contributions by the requested rules, under REQ-008's 1e-9 verification tolerance and the scope's publication policy. PRD §12.9 places response-spectrum analysis in scope, after v1.0; this finite-precision refusal restricts the inputs on which the combination unit publishes. It does not demonstrate a required reference-system request that it refuses.
- **Code:** `SPECTRUM_COMBINATION_ACCURACY_UNSUPPORTED`, blocking, emitted by the supplied-contribution combination unit (`core/analysis/response_spectrum`, unit S4). Its diagnostic begins “this implemented error argument does not establish the required relative accuracy” and names the row and field. There is not yet an end-to-end spectrum solve.
- **What is refused:** the whole combination request if the implemented outward enclosure does not establish every published magnitude to 1e-9 relative of its selected rule on the represented binary64 inputs, including final binary64 conversion. Refinement stops at 2,048 bits. Zero is published only when exact arithmetic proves it zero; an interval containing zero is insufficient. No partial result or unresolved row is published. A result beyond finite binary64 range retains the separate `SPECTRUM_COMBINATION_NONFINITE` refusal.
- **What it means in practice for a user:**
  - Cancellation can require more precision than the available ladder establishes. The maintained 88-term exact-binomial witness refuses at 2,048 bits; a 40-term witness publishes only at that top rung. These are represented-input arithmetic witnesses, not evidence about physical input accuracy or a judgment that small responses are negligible.
  - Output quantization can also prevent the accuracy requirement: 100-40-40 of three smallest positive binary64 values is exactly 1.8 such quanta, whose nearest binary64 is 2 quanta. The exact directional coefficient is 2/5, not binary64 0.4.
  - Normal-range answers are not promised unconditional publication. Refusal does not prove that another error argument or algorithm could not establish a result. No absolute floor absorbs a failed relative criterion, and no change to a physical input is supplied to evade it.
- **Why:** Agent 0 chose the 2,048-bit ladder cap as a reversible policy about how far refinement proceeds. The actual enclosure, not a planning error model or a positive radicand alone, governs publication. The cap bounds neither work nor memory; accumulated endpoint integers can exceed the refinement precision. Arithmetic on represented inputs establishes no physical-model accuracy and does not lift the modal basis's standing or surface admission conditions.
- **What the user does instead:** this implementation cannot supply the refused combination result. A method with a justified error argument or further precision may establish it; changing the engineering problem or replacing the refusal with a rounded display value is not a resolution.
- **Basis:** DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md), “Combined magnitudes are certified against their exact values”; the local exact-rational-kernel and outward-dyadic implementation and independently checked witnesses in `core/analysis/response_spectrum/tests/represented_input_oracle.rs`. PR #369 records the numerical review and Agent 0's cap and refusal-name decisions. LIM-024 separately states interpolation's conditional library premise; this entry neither establishes nor removes it.
- **Status:** not yet declared.

### LIM-026 — Ordinary results can say checks passed or established without a verified inverse-norm bound

- **Id:** LIM-026
- **Kind:** known product defect; it causes no automatic refusal.
- **Exit criteria:** 18; also 1 and 4 where the reference-system analyses and their outputs depend on the affected static solves. Listing this limitation does not make any of these criteria met.
- **Requirement affected:** DEL-04-01 REQ-015's integrity bound and unavailable disposition, REQ-016's nonlinear state enclosures, and the consumers required to preserve their standing by release exit criterion 18. This includes a static state-case solve used by modal analysis; the modal eigenproblem's own integrity checks and the retained-precision route's separate certificate are outside this entry.
- **Code:** `NUMERICAL_INTEGRITY_CHECKS_PASSED`, the ordinary result's `checks_passed` quality, the diagnostic sentence reporting a converged support state as established, and ordinary-reader `numerically_eligible` standing. These are present output claims, not warnings or refusals of this limitation.
- **What is refused:** nothing is automatically refused on this ground. The ordinary integrity check takes 1,024 times an estimate of the inverse norm of its scaled stiffness matrix as an upper bound without establishing that it is an upper bound. Consequently its intended-system error bounds and a nonlinear state confirmation relying on them are not established by that check. Development main violates what exit criterion 18 says those claims mean. This is a known defect. Its repair, a verified bound with the legacy-document consumer change, is not complete, and no separate interim containment has been implemented. For v1.0 Agent 0 decided on 2026-11-20 that these outputs present the integrity check as not verified rather than as passed, and that the verified bound follows after v1.0 (release target, Direction, "Numerical claims"); until that change lands, the defect stands as described here. The defect remains present on development main, and exit criterion 18 remains unmet. The defect is not acceptable merely because the software is unreleased.
- **What it means in practice for a user:** raw ordinary result output and desktop numerical-health badges can show “checks passed”, a result diagnostic can report a support state “established”, and the ordinary reader can return `numerically_eligible`, although the inverse-norm premise needed by those claims is unproved. These tokens record product behavior; they are not certified accuracy or independent evidence that the support state or its error interval is correct.
  - The maintained constructed false-pass and false-state witness, [inverse_norm_witness](../validation/references/inverse_norm_witness/README.md) ([PR #327](https://github.com/sgttomas/SWBPIPE/pull/327)), has true scaled inverse norm 262,144 while the estimator returns 1. The inverse-norm allowance used is about 256 times too small. Its linear cases pass their checks despite an incorrect displacement; its one-way cases report an inactive support with its state established where the independent exact solution has it active. These are constructed examples, not a demonstration that all ordinary results are wrong. How far the failure reaches into ordinary models is not established.
  - The rendered exact report describes such solver-reported confirmation as “Not confirmed”, conditional on the inverse-norm step, and explains the failed premise. That protects the report's presentation only; it does not verify the raw output, established diagnostic, desktop badges or ordinary numerical eligibility.
- **Why:** a condition estimate is not a verified upper bound. Multiplying it by a fixed number does not establish the missing inequality. The bound must be verified against each actual scaled matrix, or the integrity check must be unavailable with its named reason. A verified producer also does not supply missing evidence to an older document.
- **What the user does instead:** treat the affected values and state tokens as inspection results, retaining the rendered report's conditional standing. Do not use a legacy established or checks-passed token as evidence for another analysis or assurance claim. No supported ordinary solve option establishes the missing premise; a different solver mode or repeating the solve is not a certificate.
- **Basis:** DEL-04-01 [scope](../execution/PKG-04/DEL-04-01/ScopeOfWork.md), VER-008's REQ-015 account and VER-009; [release target](RELEASE_TARGET_v1.0.md), exit criterion 18; the independent witness and maintained known-limitation test introduced by PR #327. This entry records the existing defect and supplies no acceptance, issuance or release.
- **Status:** not yet declared.

### LIM-027 — A bare structured quantity edit can store its number beside the old unit

- **Id:** LIM-027
- **Kind:** known product defect; it causes no automatic refusal.
- **Exit criteria:** 17, through the shared operation route used by the embedded agent. The same defect is reachable through imported structured proposals; the witness is not an executed desktop-agent use case.
- **Requirement affected:** DEL-16-02 REQ-010 and AC-006 require unit and target-dimension validation without an unrequested change of physical value; DEL-07-02 REQ-003 requires unit-bearing edits to preserve their unit meaning and refuse a wrong dimension by name.
- **Code:** no finding refuses this mismatched bare-number edit. The executed witness later reports `MECHANICS_SOLVED`; `HIGH_DISPLACEMENT_REVIEW` is a warning; it does not diagnose or block the unit error.
- **What is refused:** nothing is automatically refused on this ground. A structured quantity edit can declare a unit different from the stored unit, supply only a bare number as its new value, and have that number written beside the old unit. The stored quantity then means something different from the operation's declared value. This is a known product defect; no acceptability declaration has been made, and its shared-family repair is authorised and has not started.
- **What it means in practice for a user:** [issue #382](https://github.com/sgttomas/SWBPIPE/issues/382) records an executed operation-to-document-to-solve witness at source `fb93f240183e3a42030f5044faf621f780335b9c`. Starting with elastic modulus E = 200000000000 Pa, an edit with bare `after="210"`, unit `GPa` and dimension `stress` is applied as E = 210 Pa. An invented two-span axial cantilever with a 1 N tip load then publishes 186 result rows and a tip displacement magnitude of 4665.3165982762575 mm, with no blocking finding. The explicit value/unit pair 210 GPa gives 4.6653165982762576e-6 mm. The explicit pair 210 Pa produces a stored document and complete result byte-identical to the bare edit. This demonstrates a changed consumed input and published output, not a display discrepancy. These are observed product outputs, not independent certification of the solver's numerical claims (see LIM-026), and no native UI capture is claimed.
  - The agent operation schema and offline/imported-proposal intake can carry the bare form. The ordinary property inspector and material table emit explicit value/unit pairs; their normal explicit-pair entry paths are not affected by this bare-number defect. The headless operation library was exercised directly, not a generic headless apply-operation command.
  - Source inspection identifies the shared quantity/unit rule in material E and expansion coefficient, inline pipe diameter/thickness, primitive-load magnitudes and temperature-point shear modulus. Base shear modulus also uses the affected rule in the inspected main source. The operation catalog does not expose every such field; some are reachable only through other applier entrypoints. Target-dimension trust is part of the same repair. These are source-traced exposures, not executed wrong-result witnesses for every field or surface.
  - A bounded audit identified no maintained document or recorded journey written through the different-unit bare form. Stored quantities without the original operation provenance cannot establish their own history; this is not evidence that every existing model is unaffected.
- **Why:** the shared resolver can admit the declared unit but, without an explicit value/unit payload, update only the numeric sibling and leave the old unit. It also does not consistently derive the required dimension from the target field. The resulting stored quantity can be structurally valid, so a later solve cannot infer the intended entry unit.
- **What the user does instead:** use an explicit value/unit pair with the correct target dimension and inspect the stored pair after applying it. For a bare scalar update, keep the current stored unit. Do not reinterpret or automatically repair an existing model without its authoring provenance. The explicit-pair controls establish this alternative for the stated E witness, not a complete audit of every quantity path. Using the correct form for a later edit does not discover or automatically repair a document already affected.
- **Basis:** DEL-16-02 [scope](../execution/PKG-16/DEL-16-02/ScopeOfWork.md), REQ-010 and AC-006; DEL-07-02 [scope](../execution/PKG-07/DEL-07-02/ScopeOfWork.md), REQ-003; issue #382's executed witness, three-way control and source-path assessment. This entry records the existing defect in [PR #381](https://github.com/sgttomas/SWBPIPE/pull/381); it changes no product behavior and supplies no acceptance, issuance or release.
- **Status:** not yet declared.

### LIM-028 — Static seismic generation gives a member mapped by a weighted component its pipe's load where the component's force is required

- **Id:** LIM-028
- **Kind:** known product defect with no component-specific refusal.
- **Exit criteria:** 1 and 2, for the three-axis static seismic cases of the reference system, whose valve, flanges and eccentric reducer carry weights the self-weight plan applies. Listing this limitation does not make either criterion met.
- **Requirement affected:** DEL-05-01's static-equivalent generation definition, REQ-008 and AC-007 require the mapped-component force and its eccentric moment in place of the member's distributed seismic load. The definition also requires a named refusal when an eligible component's scaled force or moment fails its range check. REQ-008 also requires a seismic model-check warning for entered weight-category loads, for added operating-condition weight on the pressure-free route, and for unequal known gravities under its applied-component-weight condition.
- **Code:** none. No finding refuses or warns on this ground.
- **What is refused:** the generator has no refusal specific to component replacement or to the scaled component terms. An invalid substituted pipe intensity can still block a case whose component terms are in range. Where DEL-05-01 admits component replacement, successful static seismic generation today applies the member's load per length (g-factor × the seismic case's gravity × the member's mass per length as a pipe). It is a substitution of one load for another on that member. Before rounding, the difference between the component force and the resultant of the pipe load is the g-factor times the difference between W and the member's weight as a pipe at the seismic case's gravity. The pipe load supplies no component centre-of-gravity moment. Whether the substituted load is larger or smaller than the required one depends on the component; neither direction is established in general. The substitution affects only members for which the scope requires the component force and moment. The product also omits the named refusal when the scaled component force or moment fails the definition's range check. The product has no seismic model-check warning of REQ-008's kind. In the witness with the component weight applied at 9.79 m/s², the 10 m/s² seismic case solves without the unequal-gravity warning on either route. This is a known product defect on both routes and in both solver modes. Its repair is authorised and has not landed.
- **What it means in practice for a user:** when a case affected by the substitution solves, it publishes displacements, reactions, forces and stresses for the substituted load, with no finding that identifies the substitution.
  - The witness is the invented cantilever of [seismic_component_weight](../validation/references/seismic_component_weight/README.md), whose references were derived without product code: a valve of weight 2350 N with an offset centre of gravity, mapped to a 0.6 m member, beside the same model with the valve's weight entered as zero, which keeps the member's load per length. At main `d8349a1c427dc884dfa29c15171c432acad70318`, the applied-load resultants inferred from the published support reactions match the zero-weight and no-component controls in all six components in all five seismic cases, on each route in both solver modes. They match the pipes-only reference within 2.4 × 10⁻¹⁶ on each route when force and moment are normalized separately by their largest reference component in each case. With g-factors (0.3, 0.35, 0.4) at 9.79 m/s², the pressure-free route gives (1534.35, 1790.07, 2045.80) N against (2039.22, 2379.08, 2718.95) N. The exact route gives (1674.73, 1953.85, 2232.98) N against (2161.29, 2521.50, 2881.72) N. On each route the product gives zero X moment against 9.9875 N·m. The weight case includes the valve's 2350 N force and its eccentric moments.
  - The exact route's published generation record lists seismic primitive IDs for the mapped member and names the route's mass method. It records what was applied; it does not say that the component's force was required.
  - These are observed product outputs on one invented model, compared with an independent reference. They are not independent certification of the solver's numerical claims (see LIM-026), and how the difference bears on any other model is not established.
- **Why:** SWBPIPE's Agent 0 decided the component-force rule on 2026-11-12 and recorded it with the reference package. DEL-05-01 retained the earlier distributed-load wording, and the engine still generates a distributed seismic load on each remaining member. The scope and AC-007 now state the decided rule; the engine does not yet implement it.
- **What the user does instead:** where DEL-05-01 admits component replacement, its scaled terms pass the range check, and the product generates the case, you can add the component's nodal force g-factor × W and its moments r × F as explicit loads. This leaves the generated pipe load on the mapped member. For a case that passes DEL-05-01's admission and range checks, omit seismic generation and author the distributed pipe loads and the component nodal forces and moments required by DEL-05-01 to apply the intended seismic contribution. On an exact load/reference-state case, list each of those stored primitives once in `analysis_state.load_sources` with factor 1 to apply its entered magnitude. These alternatives are supported by the input and solve paths. A maintained witness of the alternatives is still due.
- **Basis:** DEL-05-01 [scope](../execution/PKG-05/DEL-05-01/ScopeOfWork.md), the definition of static-equivalent generation, REQ-008, REQ-015, AC-007 and VER-005; the frozen reference package and its decision record. The defect was standing before this entry; the entry is added by the pull request titled "State the seismic component-force rule in DEL-05-01 and record LIM-028". It changes no product behavior and supplies no acceptance, issuance or release.
- **Status:** not yet declared.

### LIM-029 — The embedded agent reads and proposes within fixed sizes

- **Id:** LIM-029
- **Exit criteria:** 17. By a packing calculation, not a run of the criterion's journey, the reference system's records fit in at least 28 proposals within the sizes below. The limitation concerns other models, larger or with longer texts, and the replacement of a stored analysis state on the reference system itself.
- **Requirement affected:** Exit criterion 17, that the embedded agent builds the reference system from a person's request (DEL-16-06/REQ-010). What is limited is the same reading and proposing on another model, and the replacement of a stored analysis state that does not fit one operation. `docs/PRD.md` states no model size for the agent (§5.3, §11.8 FR-AGENT-001 to FR-AGENT-005, §24 R7), so nothing it states is contradicted; whether the agent is meant to work on the whole of an ordinary model is a question of intention that Agent 0 has put to the owner.
- **Code:** none. A read that cannot carry a record answers `Record <id> exceeds this catalog page.`, and a tool call whose arguments exceed the limit is not admitted.
- **What is refused:** What is tested, and can be relied on, comes first. On the reference system as committed (26 pipe members, three load cases), the agent's read of the load cases returns each of the three once across pages, with its analysis state and its pressure regions as stored; primitive loads are read as a family of their own. And the families the agent's catalog newly admits (the project's units other than length, the material's temperature points and expansion law, the sections and each pipe's binding to one, the reference configuration, each load case's pressure regions and analysis state, and the combinations) are entered through proposals within the sizes below, in a route check with scripted inputs. That is no promise for another model, of that size or any other: adding an analysis-basis override of 512 ASCII characters of reason and 512 of provenance to each of the 26 members adds 26,624 bytes before keys and escaping, which with the existing state text exceeds 28,672 bytes. What is refused follows from four fixed sizes: 28,672 bytes for the arguments of one tool call and for each `recordText` or before text returned by a record read; 48 KiB for one record together with the texts its edits need; 16 operations in one proposal; and 64 tool calls in one agent session. What follows from them is stated here in kind. Measurements on the reference system's pattern repeated to larger member counts are given as examples in the description of the pull request that records this entry; another model's records differ, the model schema sets no length for an override's texts, and no member count is a figure a model can be checked against.
  - A load case's analysis state is one field and is written whole. A state whose entry or removal does not fit the arguments of one tool call cannot be entered or removed by the agent. Replacing a stored state in one operation names the stored state and the new one together, so it needs room for both. Where that one operation does not fit, the agent does not carry the replacement through from a single request: it leaves the stored state in place and says so. The person then asks for the removal and for the new entry separately, each a proposal that must fit by itself, and the case has no analysis state from the time the removal is applied until a new state is entered. For the committed operating case, RS-OPE, carrying its current state text in both fields exceeds the argument size. A changed replacement needs separate requests when its two texts make the operation exceed that size. This is narrower than before the pull request that records this entry, when the agent was told to make both proposals itself. Agent 0 decided it for v1.0: the two proposals are separated by the person's Apply and can be separated by the end of a session, and an instruction cannot hold a sequence across that.
  - A load case is read with its analysis state and its pressure regions. A case whose state is over the text size is not returned: the read fails naming it. So is a case whose state and pressure regions are each within the text size but exceed the record size together, which two proposals within the argument size can produce. So can a person's own acts in the desktop, an edit or the requeue of a stored record of an agent's proposal onto a later model among them; those are the person's and no check on the agent's proposals reaches them. What the agent can write is therefore not always what it can read back.
  - When a load case cannot be returned, the load cases listed after it cannot be reached either, until the read is repaired ([issue #401](https://github.com/sgttomas/SWBPIPE/issues/401)). The case itself stays unreadable until a read exists that returns it with the oversized texts withheld by name ([issue #403](https://github.com/sgttomas/SWBPIPE/issues/403)).
  - The project record carries the reference configurations and the model settings together and is read as one record; when it exceeds the record size it is not returned. The reference configurations, and one case's pressure regions, are each one field written whole, and meet the argument size as an analysis state does.
  - An agent session admits 64 tool calls. The next is refused and the session ends; starting a new session clears the conversation. Reading every family of a model, or building one, takes a number of calls that grows with the model.
- **What it means in practice for a user:** Where one of these sizes is exceeded, the agent cannot make that proposal or that read, and the person does the work concerned. For a load case whose record the agent cannot read, it has none of the stored texts that an edit or a deletion of that case must name; the case's primitive loads are read as a family of their own. Work with the agent that needs more tool calls than a session admits ends at that limit before it is finished.
- **Why:** Every tool call and every result is bounded so that the native host can admit it before parsing it, under fixed native ceilings of 64 KiB for a call and for a result. The operation engine admits a load case's analysis state only as the whole field, so a state cannot be divided between calls.
- **What the user does instead:** Start from what is reported: a replacement the agent says it cannot make in one operation (its instruction is to say so and leave the stored state in place), a reported proposal or page limit, or a session ending at its limit of tool calls. A change that has simply not appeared is not by itself a sign of this limit, since every agent proposal waits for the person's Apply. Then, in the order a person meets it:
  1. Where the agent says it cannot carry a replacement of a stored analysis state through, either edit the state in the desktop's load state editor, for what that editor exposes (see point 3), or ask the agent to remove the stored state, apply that proposal, and then ask it to enter the new state, giving the state again if a new session has started. If the agent says the new state's entry is over the argument size, or that it cannot tell whether it fits, do not ask for the removal: use the desktop editor or the import route below. Between the removal and the entry the case has no analysis state and a solve that needs it refuses the case by name.
  2. Otherwise enter or edit the record in the desktop, where its editor exposes the field.
  3. The desktop's load state editor enters an element state's analysis-basis override where the state holds none, as a reason and a provenance, each field taking up to 512 typed characters. It shows a stored override as text that cannot be edited in place: to change one, remove it and enter it again. It keeps a stored override when the state is edited and leaves overrides out when a row is filled to other members or a state is copied from another case. A whole analysis state can also be submitted as a structured proposal under "Review an imported proposal", which validates it through the same operation engine; the person reviews the batch and applies it. That route is not subject to the agent's tool sizes; it keeps the far larger checks every person-entered proposal has on a value's length, depth and member count, and the native and engine checks still apply. It rests on reading the import and the engine: no test imports and applies an analysis state this way.
  4. Go on asking the agent for the records that are entered one at a time.
  5. When a session ends at its limit, start a new one and say again what is wanted, since the conversation is cleared.
- **Basis:** DEL-16-06 [scope](../execution/PKG-16/DEL-16-06/ScopeOfWork.md), which bounds every read and states no size. The record size, the operations in a proposal and the tool calls in a session were standing before this entry, which records them. The argument and text size is set by the pull request titled "Raise the embedded agent's argument bound to 28,672 bytes and record its fixed sizes (LIM-029)", which raised it from 16,384 bytes and gives the ground; that pull request records this entry.
- **Status:** not yet declared.

### LIM-030 — A static case that meets a binary64 range failure leaves the ordinary solve's document with no result rows, unless retained-source recovery or force scaling publishes it

- **Id:** LIM-030
- **Exit criteria:** 1, for the static analyses it names, on a model whose solve meets a range failure; 3.
- **Requirement affected:** PRD §12.1 and §12.2, the linear static frame solve with distributed, concentrated, thermal and imposed-displacement loads (FR-SOL-002, FR-SOL-006). A range failure in one case triggers this limitation.
- **Code:** `NUMERICAL_INTEGRITY_UNRESOLVED` (blocking), the case's integrity diagnostic, which on a linear model names the force-scaling outcome, for example `range: family not admitted under force scaling: uniform_element_load` (a structural failure of the force-scaling attempt other than a range failure keeps the integrity code it has on an ordinary case); on a model with a nonlinear support, `SOLVER_SYSTEM_BLOCKED` (blocking) for a range failure at stiffness formation, or the case's `NUMERICAL_INTEGRITY_UNRESOLVED` for one returned by the case's structural evaluation; on a linear model with a skewed restraint, `SUPPORT_DIRECTION_SKEW_FORCE_SCALED_UNSUPPORTED` (blocking).
- **What is refused:** a static load case whose ordinary solve meets a binary64 range failure (DEL-04-07's scope names the two triggers). Then, in this order:
  - on a model with any nonlinear support, force scaling is never attempted and the case is refused;
  - on a linear model with a skewed restraint, the case is refused;
  - on any other linear model, an exact-route case may be published from retained-source recovery; otherwise the product attempts force scaling, which can itself be refused, and publishes the case only if the model and case carry none of these families, named in this order, the first found: `objective_connector`, `curved_bend_macro_element`, `thermal_or_eigen_load`, `exact_pressure_operand`, `uniform_element_load`, `constant_effort_support`, `non_nodal_load_term`, `zero_nodal_load_term`.

  A case refused in any of these ways blocks the whole document: the ordinary solve's document is the blocked, model-incomplete one with no result rows for any case.
- **What it means in practice for a user:** in the trials made for this entry the refusal was met only at magnitudes tens to hundreds of orders of magnitude from the cantilever's starting values. On a 1 m cantilever of 0.12 m outside diameter and 0.01 m wall with a 100 N tip load and a 50 N/m distributed load, a range failure appeared with an elastic modulus of about 3e-289 Pa, with loads of about 1.5e-301 N or about 7e307 N, with a span of about 3e79 m, and with an outside diameter of about 7e-77 m or about 9e74 m. The larger load, span and outside diameter reached the solve only through the typed entry because the checked request entry rejected those values before solving. Tried on four models (that cantilever, an invented nonlinear model, an invented exact-route connector system and the reference system), lengths, moduli and loads each multiplied or divided by a thousand or a million, as a unit mistake would, did not produce a range failure, nor did a 1e12 to 9e15 N/m spring beside a 1e-3 or 1e-6 N/m spring: no range failure was observed in these trials (some of the variants were refused earlier, for another reason). Where a linear model does meet one and force scaling reaches its admission, a case that carries pipe weight is refused, naming the first excluded family it carries. The ordinary solve then gives the user no displacements, forces, stresses or reactions for any case of the document, and a blocking diagnostic.
- **Why:** publication under force scaling is not supported for the excluded families, for a model with a nonlinear support or for one with a skewed restraint. The force-scaling attempt can also fail its checks before family admission or while unscaling a result for publication.
- **What the user does instead:** the ordinary solve offers no option to recover this result. In a registered build, DEL-04-08/REQ-003(d) permits a retained-precision attempt after certain failed single-case solves; whether that attempt returns a result for a range-failed case has not been established. In the trials a factor of a thousand or a million, as a unit mistake would give, did not reach it; checking the entered magnitudes is where to start.
- **Basis:** DEL-04-07 [scope](../execution/PKG-04/DEL-04-07/ScopeOfWork.md), the range-failure description under "Method choices that define the solve" (the product solve's composition) and "Blocked document"; DEL-04-01 [scope](../execution/PKG-04/DEL-04-01/ScopeOfWork.md), the range-failure description under "Method choices that define the kernel" (the kernel's range errors); the pull request "Record the range-failure refusal as main does it (DEL-04-07, DEL-04-01, LIM-030)". This refusal already stands on main; the change that adds this entry records it and does not introduce it, and it supplies no acceptance, issuance or release.
- **Status:** not yet declared.

### LIM-034 — The pressure-free route refuses wind on a marked part of a Timoshenko straight member

- **Id:** LIM-034
- **Exit criteria:** none. Exit criterion 1 requires the reference system's wind case on the exact route.
- **Requirement affected:** PRD §12.5 requires wind load support. DEL-05-01 REQ-008 specifies generation on both routes with the v1.0 exception in DEL-05-03 REQ-011. For v1.0, SWBPIPE's Agent 0 decided on 2026-11-18 that DEL-05-01 AC-007 and VER-005 state the partial-span Timoshenko wind comparison on the exact route only. DEL-05-03 REQ-011 states the refusal.
- **Code:** `PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED`, blocking (`core/product_physics/src/pf_timoshenko.rs`).
- **What is refused:** solving a pressure-free document that selects Timoshenko straight members (`model_settings.shear_model` set to `timoshenko_annulus_energy_v1`) when a load case's wind generation lists a straight member in `equivalent_static.wind.exposed_spans`. Each such entry is refused, whatever its fractions, by one blocking finding that names the load case and the member.
- **What it means in practice for a user:**
  - With Timoshenko selected on the pressure-free route, entering a straight member's wind span by fractions stops the entire solve, even when the fractions run from 0 to 1.
- **Why:** the pressure-free Timoshenko rollout deferred partial-span load-formation verification and pressure-free references. The frozen reference package `validation/references/w16_timoshenko_partial_wind` specifies the comparison on the exact route only.
- **What the user does instead:** to apply wind over the whole member, replace its exposed-span entry with an entry in `equivalent_static.wind.exposed_pipe_refs`. To keep the fractional wind marking, select `euler_bernoulli` if that theory suits the model. This selection applies to every straight member.
- **Basis:** DEL-05-03 [scope](../execution/PKG-05/DEL-05-03/ScopeOfWork.md), REQ-011; DEL-05-01 [scope](../execution/PKG-05/DEL-05-01/ScopeOfWork.md), AC-007, VER-005 and Agent 0's decision recorded there. The refusal was standing before this entry, from the change that added the pressure-free Timoshenko selection on 2026-10-27; `core/product_physics/tests/pf_timoshenko_runtime.rs` observes it, with the whole-member marking and the Euler–Bernoulli twin solving. The entry is added by the pull request titled "State the seismic component-force rule in DEL-05-01 and record LIM-028".
- **Status:** not yet declared.

### LIM-035 — Modal analysis and response-spectrum analysis are not part of v1.0

- **Id:** LIM-035
- **Kind:** omission outside the exit criteria. No exit criterion requires this capability, and nothing emits a refusal for it.
- **Exit criteria:** none. Exit criterion 1 listed modal and response-spectrum analysis until Agent 0 moved them after v1.0 on 2026-11-20 (release target, Direction, "Seismic").
- **Requirement affected:** none of the release target. The omission narrows the PRD for v1.0: §12.9 (modal analysis, and response-spectrum seismic analysis on its basis), FR-SOL-016 (modal analysis, a Should), §7 item 15 (snubbers as dynamic-only restraints in response-spectrum analysis; LIM-036) and §24 V1, whose exit criteria are the release target's. The PRD amendment of 2026-11-20 records that response spectrum follows after v1.0 and that v1.0 does not require modal analysis; FR-SOL-016 remains a requirement of the product.
- **Code:** none. Modal analysis has no desktop surface and no engineering admission (DEL-04-09, "Admission"), and no product surface admits a spectrum request for an engineering result (DEL-04-10, "Experimental reach").
- **What is refused:** nothing is refused. SWBPIPE v1.0 gives no natural frequencies, mode shapes or response-spectrum seismic response as an engineering result. The headless runner's `modal` command runs a modal request only as an experiment, not as a public engineering result.
- **What it means in practice for a user:**
  - Seismic analysis in v1.0 is three-axis static seismic, with SRSS and absolute-sum combination, and seismic anchor movement (exit criterion 1). Whether a static seismic analysis suits a system is the user's engineer's judgment; SWBPIPE does not determine it.
  - Frequencies are not among v1.0's published results, and a snubber has no use in v1.0 (LIM-036).
- **Why:** Agent 0 decided on 2026-11-20, under the grant in `AGENTS.md` and the owner's instruction "Static seismic is enough for v1.0 but response spectrum will follow later so keep what's there.", that static seismic is v1.0's seismic analysis and that modal and response-spectrum analysis follow after it. The work already done on them is kept in the repository.
- **What the user does instead:** the static seismic cases and their combinations (DEL-05-01, DEL-05-02). A dynamic analysis, where the user's engineer needs one, is made in other software.
- **Basis:** [release target](RELEASE_TARGET_v1.0.md), Direction, "Seismic"; DEL-04-09 [scope](../execution/PKG-04/DEL-04-09/ScopeOfWork.md) and DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md); PRD §12.9 as amended on 2026-11-20. LIM-020 and LIM-022 to LIM-025 describe limits of the kept work and apply when it is taken up. This entry is added by PR #475.
- **Status:** declared acceptable by Agent 0 (PR #475).

### LIM-036 — A model with a snubber is refused on every static solve

- **Id:** LIM-036
- **Exit criteria:** none. Exit criterion 1 lists no snubber among the reference system's supports, and snubbers serve response-spectrum analysis, which follows after v1.0 (LIM-035).
- **Requirement affected:** DEL-04-10/REQ-005 and AC-003: a snubber restrains in the dynamic stiffness only, and every static load-case solve ignores it and discloses it as "dynamic-only restraint, not active in static cases"; DEL-04-09/REQ-006, the modal basis that includes it on request. PRD §7 item 15 and §12.9 state snubbers as dynamic-only restraints. Issue #418 is the unit that would build both.
- **Code:** `SNUBBER_DYNAMIC_ONLY_UNSUPPORTED`, blocking, on every static solve (`core/product_physics/src/document_members.rs`, `interim_support_refusals`, marked interim).
- **What is refused:** solving a model that contains a support of the snubber family. The finding names the support, with the message "snubbers are dynamic-only restraints; the response-spectrum analysis that uses them is not built yet".
- **What it means in practice for a user:** a model with a snubber gives no static result until the snubber record is removed. Under REQ-005 a static solve ignores a snubber, so the model without the record is the static model REQ-005 defines; the result then carries no snubber disclosure.
- **Why:** the static leave-out with its disclosure and the modal inclusion are planned as one landing (issue #418), which is not built. With dynamic analysis after v1.0, Agent 0 decided on 2026-11-20 that it follows after v1.0 too.
- **What the user does instead:** leave snubbers out of a model for static analysis.
- **Basis:** DEL-04-10 [scope](../execution/PKG-04/DEL-04-10/ScopeOfWork.md), REQ-005 and AC-003; DEL-04-09 [scope](../execution/PKG-04/DEL-04-09/ScopeOfWork.md), REQ-006; issue #418. The refusal was standing before this entry; this entry is added by PR #475.
- **Status:** declared acceptable by Agent 0 (PR #475).

### LIM-037 — The state of a sliding friction support on a derived normal is not established on verified evidence

- **Id:** LIM-037
- **Kind:** omission outside the exit criteria. No exit criterion requires this capability, and nothing emits a refusal for it.
- **Exit criteria:** none. Exit criterion 18 requires that a converged nonlinear state not be presented as confirmed unless DEL-04-01/REQ-016's enclosures establish it, not that it be confirmed; exit criterion 1 requires the reference system's friction support to solve, which this entry does not affect.
- **Requirement affected:** none of the release target: no exit criterion requires this confirmation. DEL-04-01/REQ-016 covers confirmation of a friction support whose normal is derived from another support's reaction; neither the friction unit of issue #453 (a normal derived from a base restraint) nor the later, unstarted work on a one-way or lift-off normal source is built (DEL-04-03). No PRD section states this confirmation; PRD §12.4 and FR-SOL-014, that friction converges on validation cases and reports diagnostics, are not narrowed.
- **Code:** none.
- **What is refused:** nothing is refused. v1.0 does not present the state of a sliding friction support whose normal is derived from another support's result as established on verified evidence. The friction unit of issue #453, the retained operand and its publication through the evidence carrier's finalization boundary, is not built, and nothing is presented as establishing a state until the verified inverse bound is on main, which follows after v1.0 (release target, Direction, "Numerical claims").
- **What it means in practice for a user:** such a support solves as any friction support does, and its converged state is a statement about the discrete model that is not confirmed on verified evidence. A "checks passed" or "established" token for it on development main is the observation LIM-026 describes, not evidence. Where the normal comes from a one-way support, LIM-019's separate non-convergence refusal may apply; this entry concerns only the confirmation of a converged state.
- **Why:** confirmation on a derived normal depends on the verified inverse-norm bound and the evidence carrier's finalization boundary, neither of which is on main, and Agent 0 decided on 2026-11-20 that the verified bound follows after v1.0.
- **What the user does instead:** treat the support's state as an inspection result, and have the user's engineer judge whether it is admissible.
- **Basis:** issue #453; DEL-04-01 [scope](../execution/PKG-04/DEL-04-01/ScopeOfWork.md), REQ-016; DEL-04-03 [scope](../execution/PKG-04/DEL-04-03/ScopeOfWork.md); [release target](RELEASE_TARGET_v1.0.md), exit criterion 18; LIM-019 and LIM-026. This entry is added by PR #475.
- **Status:** declared acceptable by Agent 0 (PR #475).

### LIM-038 — A declared difference basis is refused on the exact route

- **Id:** LIM-038
- **Exit criteria:** 1 (expansion). Until PR #475 the criterion read "expansion with its basis"; Agent 0 revised it in that pull request to "expansion as the operating minus sustained difference", with the declared basis on the exact route after v1.0. Listing this refusal, or declaring it acceptable, does not make DEL-05-02 AC-012 met.
- **Requirement affected:** DEL-05-02/REQ-012, that on both routes a subtraction declared `operating_minus_sustained` is published only when the subtrahend's primitive loads are contained in the minuend's (G-1) and the two cases have the same modulus basis (G-2); AC-003 on the exact route; and AC-012, that the reference system's operating-minus-sustained difference with its declared basis is published on the exact route in the desktop, which is unmet. No PRD section states a declared difference basis; PRD §12.6 and FR-SOL-009, combination and range cases from solved cases, are not narrowed.
- **Code:** `LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED`, blocking (`core/product_physics/src/document_members.rs`, `interim_exact_combination_refusals`).
- **What is refused:** on a model with a pressure contract, every combination that declares a `difference_basis`. The finding names the combination.
- **What it means in practice for a user:** the user gets the expansion difference, operating minus sustained, as a plain result-state subtraction with no declared basis, as the reference system's RS-EXP is solved today (`fixtures/reference_system/SPECIFICATION.md`, "solved as a subtraction, with no `difference_basis`"). The product does not check, for that subtraction, that the sustained case's loads are contained in the operating case's or that the two cases share a modulus basis; the user's engineer checks that the two cases form the intended expansion pair. The pressure-free route admits the declared basis. LIM-004's held-status subtrahend is refused with it.
- **Why:** the declared-difference gate on the exact route (issue #451) is not built and its design is not final. Agent 0 decided on 2026-11-20 that it follows after v1.0.
- **What the user does instead:** a plain subtraction, with neither `difference_basis` nor `subtrahend_support_state`, of two cases the user's engineer has checked.
- **Basis:** DEL-05-02 [scope](../execution/PKG-05/DEL-05-02/ScopeOfWork.md), REQ-012, AC-003, AC-012 and "Withheld in v1.0"; issue #451, with its draft observer #444; LIM-004. The refusal was standing before this entry; this entry is added by PR #475.
- **Status:** declared acceptable by Agent 0 (PR #475). The declaration is a limitation decision: the declared basis is not demonstrated on the exact route, and DEL-05-02 AC-012 stays unmet.
