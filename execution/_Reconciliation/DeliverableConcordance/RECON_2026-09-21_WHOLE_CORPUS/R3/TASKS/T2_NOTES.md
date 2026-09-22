# R3 T2: unmapped SHELL and WSUI capabilities

T2 covers the 57 SHELL and WSUI capabilities in `R3/CAPABILITY_COVERAGE.csv` whose status is UNMAPPED (29) or UNMAPPED_RELATION_ONLY (28). Each one is classified in `T2_UNMAPPED.csv`: 26 PRODUCT_UNOWNED, 24 ROUTING_GAP and 7 NON_DELIVERABLE. No capability was classified SHARED_OK, DUPLICATE, UNKEYED_SCOPE_GAP or PARTIAL_UNOWNED_REMAINDER, because none has an owning answer.

Two findings account for most of the set:

- **Routing never reached the owners.** Area routing sent SHELL capabilities only to PKG-00, PKG-07, PKG-10 and PKG-12, and WSUI capabilities only to PKG-00, PKG-07 and PKG-16 (`ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`, AREA rows). Desktop commands that front feature engines in PKG-02, 03, 05, 06, 08, 14 and 16 therefore reached their plausible owners only by sampling, and mostly not at all. Of the 24 ROUTING_GAP rows, 20 name an owner whose package was never routed that capability. The other 4 were routed to the named owner, which answered NOT_MINE or COVERS.
- **No deliverable owns the desktop workspace shell.** Every PKG-07 deliverable answered NOT_MINE for the shell chrome, and the package scope does not name it (`SOFTWARE_DECOMP.md:223`). DEC-009 (`SOFTWARE_DECOMP.md:600`) adopts the Tauri shell as a runtime basis, not as a deliverable.

These are proposals only. Nothing here re-disposes a row or changes scope.

## Groups

The CSV gives one classification per capability. The groups below cut across classifications to support the R4 packets. A capability may appear in more than one group.

### T2-G1: desktop workspace shell (PRODUCT_UNOWNED, propose CREATE)

- **Capabilities (19).**
  - Shell chrome, 17: CAP-SHELL-001, 002, 004, 005, 017, 018 and 051; CAP-WSUI-001, 002, 003, 004, 005, 007, 009, 010, 020 and 043.
  - Project-management UI, 2: CAP-SHELL-033 and CAP-WSUI-012.
  - CAP-WSUI-011 (session state) is a candidate too. It is covered under G2.
- **Evidence.** Each capability drew 26–44 answers, and none claims ownership.
- **Relation answers keep the gap visible.**
  - DEL-07-06 COVERS CAP-SHELL-018 and CAP-WSUI-020 (keyboard access).
  - DEL-07-07 COVERS CAP-WSUI-002 and 004 (run/stop, status chips). Its reason says "shell-owned".
  - DEL-07-09 COVERS CAP-WSUI-043.
- **Owning authority.** SCOPE_CHANGE, with an OWNER decision on the shape.
- **Route.** SCOPE_CHANGE_HANDOFF.
- **Decision.** Who owns the workspace shell (layout, stages, menus, native window and menu bridge, toolbar, status bar, drawers, section hosting)? The options in the evidence are:
  - (a) Create a PKG-07 workspace-shell UX_UI_SLICE deliverable.
  - (b) Extend DEL-07-01 or DEL-07-02 as the SOW-020 carriers. SOW-020 notes that the GUI baseline is the Tauri shell (`SOFTWARE_DECOMP.md:128`).
  - (c) Extend DEL-07-08, under which the session extraction was already filed (G2).
- **On-ruling mechanism.** A scope-change workflow run adds the deliverable (or the scope rows) to SOFTWARE_DECOMP and the DAG, then issues keys. The run is re-routed only for these capabilities.
- **Risk if left.** The largest body of user-facing desktop code sits outside every acceptance and verification path. Known defects have no owner to route them to. One example is the dead `listenToNativeMenu` listener with a mismatched event name (CAP-SHELL-004 surface note).

### T2-G2: work filed under DEL-07-08 without SOW-076 scope

- **Capabilities.** CAP-WSUI-011 (session state), CAP-WSUI-012 (project handlers), CAP-WSUI-013 (persistence integrity on reopen) and CAP-WSUI-040 (saved-run verification on open).
- **Evidence.**
  - DEL-07-08 answered COVERS against its own `STATUS#history`/`MEMORY`. In `DEL-07-08_notes.md` ("Reverse pass") it asks R3 to decide whether this work was filed under the wrong deliverable or is undocumented scope.
- **Proposed reading.** Filed under the wrong deliverable, for three of the four:
  - CAP-WSUI-013 → DEL-02-05. REQ-02-05-012 already claims verify-on-open.
  - CAP-WSUI-040 → DEL-14-02. It owns run-record verification, CAP-SHELL-012.
  - CAP-WSUI-012 → the shell owner, with DEL-02-05 semantics.
- **Owner decision for CAP-WSUI-011.** Either option G1(a) or DEL-07-08 scope growth.
- **Route.** OWNER_DECISION for CAP-WSUI-011. SCOPE_CHANGE_HANDOFF for the rest.
- **Risk.** DEL-07-08's history records work that its SOW cannot accept, and the history of the real owners is incomplete.

### T2-G3: persistence commands not routed to PKG-02 (ROUTING_GAP → DEL-02-05)

- **Capabilities.** CAP-SHELL-030, 031, 032, 034 and 036, and CAP-WSUI-013.
- **Evidence.**
  - DEL-02-05's forward ledger already cites these commands as ALIGNED evidence: REQ-02-05-001, REQ-02-05-012, CLM-005.r07 and CLM-019.r01.
  - DEL-00-04, DEL-00-03 and DEL-12-01 all name the persistence package in their relation reasons.
  - CAP-SHELL-032 was already flagged in `PKG-02_VERIFICATION.md` §7 and `W2_GATE_ASSESSMENT.md:128`.
- **Exception kept visible: CAP-SHELL-048.** This is the atomic `.opsproj` writer, and W2 listed it with the PKG-02 gap. It currently writes report packages, and DEL-08-01 holds the `.opsproj` archive (CAP-COREC-010, UNKEYED). DEL-02-05's own Remaining R02 says project-package open/round-trip is not built. T2 proposes DEL-08-01 (MEDIUM). DEL-02-05 is the alternative once the project container exists.
- **Route.** SCOPE_CHANGE_HANDOFF: key assignment within DEL-02-05 or DEL-08-01. It is not a new deliverable.

### T2-G4: operations UI deflected in a circle

- **Capabilities.** CAP-WSUI-027 (apply panel) and CAP-WSUI-028 (batch and draft queue).
- **Evidence.**
  - DEL-00-05 names DEL-07-09 as owner.
  - DEL-07-09 records only the route: "implementation lands in session and PKG-16".
  - DEL-16-02 says orchestration is a workspace capability.
  - DEL-07-08 answered NOT_MINE, although its SOW CLM-004 lists "operation diff review" among its intended surfaces, and CAP-WSUI-029 (operation ledger) is PARTIAL DEL-07-08.
- **Proposed owner.** DEL-07-08, with DEL-07-09 as the alternative.
- **Route.** SCOPE_CHANGE_HANDOFF.

### T2-G5: desktop fronts of engines whose owners were never asked (ROUTING_GAP)

| Capability | Proposed owner | Basis |
|---|---|---|
| CAP-SHELL-042 | DEL-03-07 (HIGH) | OUT-001 ALIGNED names `validate_library_import`; three relation answers agree |
| CAP-SHELL-046 | DEL-08-01 (HIGH) | R-08-01-009 ALIGNED names `render_calculation_report` |
| CAP-WSUI-042 | DEL-08-01 | Report-package request, CAP-COREC-011 |
| CAP-SHELL-049 | DEL-08-04 | Frontend caller CAP-FEATC-027 is PARTIAL DEL-08-04 |
| CAP-SHELL-040 and 041 | DEL-06-03 | Four PKG-06 ledgers cite `run_rule_checks`; the demo pack goes to DEL-06-05 |
| CAP-SHELL-021 | DEL-05-01 | Engine is OWNED_UNKEYED DEL-05-01 |
| CAP-SHELL-019 and 022 | DEL-16-02 | Operation seam and wasm exposure |
| CAP-SHELL-015 and 016 | DEL-16-04 | Agent-proposal flow |
| CAP-WSUI-040 | DEL-14-02 | Run-record verification |
| CAP-SHELL-027 | DEL-02-02 | DEC-018 catalog |
| CAP-SHELL-013 | DEL-07-08 | Answered NOT_MINE; its comparison panel consumes the output |
| CAP-SHELL-009 | DEL-07-07 | Answered NOT_MINE; its background job calls the same solve body |
| CAP-SHELL-024 | DEL-10-04 | Answered NOT_MINE |
| CAP-WSUI-018 | DEL-07-07 | Answered COVERS while its own REQ-07-07-007 cites the file |

- **Route.** SCOPE_CHANGE_HANDOFF for the assignments, or R5_RECORD_REPAIR where the owner only needs a key.
- **On-ruling mechanism.** A targeted re-route of these capabilities to the named deliverables' reverse passes, or direct key issue under a scope-change ruling.

### T2-G6: capabilities that need an owner reading before assignment

- **CAP-SHELL-008** (startup preview fixtures, LOW). Is the preview model product default content or demo scaffolding? The design-knowledge half is cited by DEL-13-01 REQ-13-01-011, which is IMPLEMENTED_DIFFERENTLY with cause POSSIBLE_DEFECT.
- **CAP-SHELL-011** (result-row dimension binding). The only owner-side answer defers to "the preview service", which no deliverable owns. DEL-08-04 is proposed.
- **CAP-SHELL-026** (display conversion). DEL-02-02 was asked and answered COVERS, calling it desktop code. DEL-02-02 is still the best existing owner.
- **CAP-WSUI-035** (aggregate rule-check status, LOW). This belongs with whoever receives the rule-check run surface; CAP-FEATB-008 is in T1 scope.
- **Route.** OWNER_DECISION for CAP-SHELL-008. SCOPE_CHANGE_HANDOFF for the others.

### Non-deliverables (7)

| Capability | Kind | Note |
|---|---|---|
| CAP-SHELL-007 | Packaged self-test | Kept visible: it ships in the product binary behind a flag; DEL-10-04 is the closest home if governed |
| CAP-SHELL-028 | Dead display formatter | Only its test imports it; CODE_FIX_CANDIDATE to retire or adopt |
| CAP-SHELL-054 | Frontend type declarations | Types follow the owning contracts |
| CAP-SHELL-055 | Unit-test setup | |
| CAP-SHELL-056 | Playwright configs | DEL-00-08 names DEL-10-04 as owner; DEL-10-04 declines it |
| CAP-SHELL-062 | Dist e2e specs | Used as verification evidence by DEL-17-06 and DEL-14-02 |
| CAP-SHELL-068 | SMOKE.md | Cited as evidence by DEL-05-05, DEL-13-01 and DEL-17-04 |

Route: NO_ACTION, except CAP-SHELL-028 (CODE_FIX_CANDIDATE).

## Owner-decision candidates

1. Workspace-shell ownership (T2-G1): create a deliverable, extend DEL-07-01/02, or extend DEL-07-08.
2. CAP-WSUI-011 session state (T2-G2): the shell owner, or DEL-07-08 scope growth.
3. CAP-SHELL-008 preview fixtures (T2-G6): product content or demo scaffolding.
4. CAP-SHELL-048 (T2-G3): DEL-08-01 now, or DEL-02-05 once the project container exists.

## Coverage

- **Population.** Filtered from `R3/CAPABILITY_COVERAGE.csv` with Status in {UNMAPPED, UNMAPPED_RELATION_ONLY} and Area in {SHELL, WSUI}.
  - 57 capabilities: SHELL 38 (UNMAPPED 21, UNMAPPED_RELATION_ONLY 17) and WSUI 19 (UNMAPPED 8, UNMAPPED_RELATION_ONLY 11).
- **Check.** A read-only script compared the CSV body with that filter.
  - Result: 57 rows, 0 missing, 0 extra, 0 duplicates.
  - All classifications are from the brief's vocabulary, and all confidences are HIGH, MEDIUM or LOW.
- **Counts by classification.**
  - PRODUCT_UNOWNED: 26 (SHELL 11, WSUI 15).
  - ROUTING_GAP: 24 (SHELL 20, WSUI 4).
  - NON_DELIVERABLE: 7 (SHELL 7).
- **Confidence.** HIGH 21, MEDIUM 33, LOW 3.
- **ROUTING_GAP owners.**
  - DEL-02-05: 6.
  - DEL-08-01: 3.
  - DEL-06-03, DEL-07-07, DEL-16-02 and DEL-16-04: 2 each.
  - DEL-02-02, DEL-03-07, DEL-05-01, DEL-07-08, DEL-08-04, DEL-10-04 and DEL-14-02: 1 each.
- **Inputs read.**
  - The 1,956 reverse answers for these 57 capabilities, joined through `SAMPLE_MANIFEST.csv`, excluding `superseded_<n>/`.
  - `IMPLEMENTATION_SURFACES.csv` entry points and notes.
  - Forward-ledger citations of those entry points.
  - Deliverable SOW OUT-001 texts at the freeze.
  - `SOFTWARE_DECOMP.md` at the freeze.
  - `PKG-02_VERIFICATION.md` §7, `W2_GATE_ASSESSMENT.md`, and `DEL-07-08_notes.md`.
  - Adopted resolutions: none touches these 57 capabilities. W3 RESOLUTIONS row 323 concerns CAP-SHELL-038, which is outside T2 scope.
  - No draft resolutions file, other task output or July material was read.

## R3 observations

- **R3_OBSERVATION 1.** CAP-WSUI-018: DEL-07-07's reverse answer (COVERS, "shared workspace code") sits beside forward rows DEL-07-07:SOW#CLM-011/REQ-07-07-007 and CLM-005.r05. Both are ALIGNED and cite `statusLabels.ts` as implementation evidence. Either the forward rows rely on code DEL-07-07 does not own, or the reverse answer should have been PARTIAL. This is not a correction. It is routed as REVIEW or scope assignment in the CSV.
- **R3_OBSERVATION 2.** Several ALIGNED DEL-05-05 rows cite `apps/desktop/SMOKE.md` as evidence (for example DEL-05-05:SOW#CLM-010.r01). The implementation-surface note says its early checks name a former product header and fixture strings that may no longer match the shell. T9 or T7 may want to weigh that staleness. T2 did not re-assess those rows.
- **R3_OBSERVATION 3.** The same routing-plan limit likely affects areas outside T2, wherever area routing omitted the feature owner's package. It is recorded here as the main cause behind the SHELL routing gaps. T2 did not check other areas.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This note describes records and evidence. It makes no claim of certification, code compliance, professional approval or engineering acceptance.
