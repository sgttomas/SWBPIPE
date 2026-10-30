# R3 Exit-Chain Verification Packet - TP-R3VERIFY-001 (2026-06-20)

**Epistemic status:** derivative verification package for human R3 exit review.
This packet assembles already-recorded implementation, validation, packaging,
and human-ruling evidence for the PRD §22.4 R3 criterion. It is not a lifecycle
transition, release-readiness claim, professional approval, certification,
sealing, authentication, or code-compliance acceptance. Only the human project
authority can rule the R3 exit review or advance the current target stage.

## Scope

R3 criterion from `docs/PRD.md` §22.4:

- User can define a private non-code rule pack and run checks.
- Software blocks pass/fail when required inputs are missing.

This verification chain also addresses the two `DEC-035` R3-exit residuals:

- F-4: packaged GUI journey evidence.
- A3: authoring-journey usability.

## Human C5 Closure Record

The stale C5.7 packaged re-pass path (`TP-MAC-189`) was bypassed, not passed.
The replacement C5 criterion was the human-directed product posture recorded on
2026-06-20: the primary screen must be dominated by the 3D model and visual
interaction with it; workflow/data-entry chrome must not clutter the default
screen; a primary agent panel must be present because the agent will manage
data, workflows, and future model-redesign proposals from user input.

Human ruling recorded 2026-06-20:

> I accept those conditions have been fulfilled. C5 may be closed and you can proceed.

Disposition: F-4/A3 are closed for the C5 replacement criterion and this C5.8
evidence package may proceed. `TP-MAC-189` remains historical bypass evidence,
not a retroactive pass. R3 exit review and any R3-to-R4 stage advancement remain
human-gated.

## Evidence Matrix

| Gate | Requirement | Evidence | Verification status |
|---|---|---|---|
| C1 | Rule expression grammar frozen and implementation-ready | `DEC-022` accepted the typed AST grammar; `DEC-037` preserved no writable parser and allowed read-only AST text rendering; SMOKE TP-MAC-180 records the read-only preview follow-up | Demonstrated for R3 |
| C2 | User can define a private non-code rule pack | SMOKE TP-MAC-147..152 and TP-MAC-180 cover rule-pack lifecycle, editor, structured composer, table nodes, declaration/check builders, and read-only text preview; run records are under DEL-06-01/DEL-06-02/DEL-07-03 | Demonstrated for R3 |
| C3 | Private libraries are available without repository-default private data writes | SMOKE TP-MAC-153..155 and TP-MAC-157..161 cover import validation, local-only store/CRUD, private-library GUI, `library_value_ref` authoring/resolution/ratification, and richer resolution preview; `DEC-038` ratified the schema addition | Demonstrated for R3 |
| C4 | GUI can run rule checks and blocks pass/fail on missing inputs | SMOKE TP-MAC-156 covers GUI rule-check run/pass-fail-blocked aggregate; TP-MAC-162 wires the aggregate into the app-held analysis-run envelope; TP-MAC-163..167 cover acceptability relation and solver-result reference authoring/preview/ratification; `DEC-039` ratified both schema additions | Demonstrated for R3 |
| C5.1-C5.5 | R3 exit readiness and guided-workbench usability basis | SMOKE TP-MAC-183..187 and DEL-07-06 run records cover R3 exit plan, usability baseline, guided shell, A12 guided authoring, and R3 guided rule-pack/private-library flow | Landed |
| C5.6 | Packaged successor kit prepared | SMOKE TP-MAC-188 records package build, boot check, and C5.7 checklist prep | Prepared; later bypassed by human direction |
| C5.7 | Old human packaged pass route | Human attempt failed 2026-06-18 on usability; later bypassed 2026-06-20 by human direction | Bypassed, not passed |
| C5.7R | Replacement 3D/model-first app posture | SMOKE TP-MAC-272..277 and run records cover workspace redesign, grid mode, rebuilt package kit, CAD-grade 3D viewport, menu IA, collapsible rails, object-creation toolbar, local agent workbench, and primary-canvas drawer declutter | Landed, rebuilt, packaged, and human-accepted for C5 closure |
| C5.8 | R3 exit evidence package | This packet; SMOKE TP-MAC-190; DEL-07-06 run record `WORKING_ITEMS_RUN_2026-06-20_TP-R3VERIFY-001.md` | Assembled for human R3 exit review |
| Automated validation | Current app evidence remains green after the latest C5.7R code tranche | Latest app-code validation at commit `3abf5d9bb`: focused App Vitest 57/57; focused Playwright 8/8; full desktop Vitest 19 files / 406 tests; desktop build; Playwright 18/18; dist Playwright 1/1; Tauri `.app` build; packaged boot probe clean; DEC-025 sweep `validation/evidence/sweeps/SWEEP_20260621T014041Z_9586deb6c15e.json` passed overall | Accepted as current app build evidence |

## Boundary Review

- No live embedded-agent runtime or external SDK/harness is consumed.
- The `Design Agent` panel is a deterministic local workbench over existing
  app state, consistent with `DEC-042`; no `D-21` scope promotion occurs.
- No autonomous accepted model-state mutation path is introduced.
- No solver, schema, evaluator grammar, persistence contract, unit-storage
  contract, protected-content source, private-data write path, network path, or
  telemetry feature is changed by this verification package.
- All examples and screenshots cited are invented/local technical-preview
  evidence. User-created models and private libraries/rule packs remain local
  app storage and are not committed by default.
- No report, UI, or coordination artifact asserts code compliance, professional
  approval, certification, sealing, authentication, release readiness, or
  lifecycle issuance.

## Coordination Note

The current helper command:

```text
python3 tools/coordination/list_deliverable_status.py --dag DAG-006 --format table --summary
```

reported 101 rows with `CHECKING=8`, `IN_PROGRESS=92`, and `ISSUED=1` on
2026-06-20. This conflicts with older prompt prose saying all deliverables are
`CHECKING` or `ISSUED`. No lifecycle states were changed in this tranche. Treat
the helper output as a discovery discrepancy to surface at R3 review or later
lifecycle housekeeping, not as a blocker to the app-centric R3 verification
package.

## Verdict

The R3 product criterion is demonstrated in substance by the landed C1-C4
implementation evidence and the C5 replacement usability closure accepted by
the human project authority on 2026-06-20.

This packet is ready to present for the human R3 exit review. It does not by
itself advance the project to R4. If the human accepts the R3 exit review, the
next ordinary product work is Phase D/R4 under the already-recorded D-16,
D-18, and D-19 rulings, with D-15/D-17/D-20 still deferred to their named
lead-ups.
