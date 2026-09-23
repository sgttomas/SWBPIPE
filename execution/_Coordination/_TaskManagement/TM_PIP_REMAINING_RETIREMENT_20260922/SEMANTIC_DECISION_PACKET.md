# Piping Remaining — candidate semantic disposition

**Original decision-support proposal, now partially applied.** `CANDIDATE_ROW_ACCOUNT.csv` keeps all 239
historical census keys, their exact recorded text, source path and source hash,
plus two omitted live bullets as separate keys. At the initial candidate
commit all 241 rows were pending and unapplied. `OWNER_DIRECTION.md` and
`APPLIED_ROW_LEDGER.csv` now record the owner's conditional group approval and
18 verified empty-marker removals, four DEL-05-02 census keys and nine
additional bounded-result/true-duplicate keys now applied; 210 rows remain
held. The DEL-05-02 comparison is in `DEL0502_DEC092_COMPLETION_BACKCHECK.md`;
the nine-key source/destination check is in
`BOUNDED_RESULT_BACKCHECK_20260922.md`.
This packet groups proposals for
the owner and gives the exceptions that must be resolved before a source entry
can be removed. It does not assign a new work graph or promote a register row.

## Population and source defect

The historical census hash matches its pinned value. It lists 20 `NONE`
markers and 219 apparent tasks across 106 deliverables. Comparing its rows to
the **anchored** `## Remaining` sections at checkout
`b3e2ce4ec74e01d6f393fc0bc069699bb079df91` yields 215 actual live
bullets and 20 markers. All 106 source hashes still match; this difference is
the historical extractor's interpretation of the same bytes, not a later
source edit:

| Census keys | Actual source meaning | Proposed treatment |
|---|---|---|
| `DEL-12-01:1`–`:4` | Four fragments of the old D-41 declaration and `Current State`/`Last Updated` above the anchored Remaining heading; none is a live bullet. | Preserve all four census rows as extraction evidence; do not migrate their fragments as tasks. |
| `LIVE:DEL-12-01:1` | Anchored bullet for LFSP-REQ-011 runtime private-path resolution and bounded mapping of test families to actual native/storage/report evidence. | Retain until the witness is done or its exact surviving scope is verified in `DEL-12-01/ScopeOfWork.md` LFSP-REQ-011 and the receiving work is selected. Existing test definitions are candidate evidence, not a pass. |
| `LIVE:DEL-12-01:2` | Anchored bullet for `RF-001` and `RF-002` with `HumanDisposition=TBD`. | Retain pending owning human review; technical homes do not close the findings. |
| `DEL-05-02:3`–`:5` | One multiline bullet originally said DEL-09-04 derivative regeneration remained deferred. The receiving DEL-09-04 case page, index and owner-adopted 2026-08-09 run show the exact derivative was regenerated at `DRAFT_EVIDENCE`; the current DEL-09-04 status retains only other owner-gated work. | Treat as **one fulfilled historical bullet** with three original census keys. Its stale source bullet was removed after the key-by-key comparison in `DEL0502_DEC092_COMPLETION_BACKCHECK.md`; no new DEL-09-04 regeneration amendment is proposed. The separate DEL-05-02 item 2 bounded-result bullet was proved and removed on its own grounds. |

The DEL-12-01 live bullets are **census extraction omissions in the same hashed
source**, not post-snapshot additions. No original census row is silently
replaced by them.

## Grouped candidate treatments

The account's treatment field gives the full key membership. The following
groups describe how to read it; several rows have both a past-result and a
surviving-work meaning. A path's existence alone is not proof that its text
preserves the exact obligation.

| Group | Candidate disposition and destination |
|---|---|
| No-current-task markers | Twenty `NONE` rows record an empty historical list entry only. They can retire as markers after checking each lifecycle/claim/formal-change source; no delivery completion follows. `DEL-01-01:NONE` is an explicit exception below. |
| Bounded results, duplication and historical observations | The nineteen `RECORDED_BOUNDED_RESULT` rows record accepted slices or reviewed evidence. The completed part should point to its actual run/decision source from a terse MEMORY retirement row if useful; any unfinished part remains in its Scope of Work or actual owner. `DEL-07-05:1/:3` duplicate the rotational-overlay obligation; `DEL-05-03:2` is a dated 196-error raw-envelope observation, not a current universal count; `DEL-09-05:3` records no currently actionable issuance wave, while DEC-062 governs any later one. |
| Ordinary delivery or verification | The `GOVERNING_SCOPE_CANDIDATE_WITH_UNFINISHED_WITNESS` and `PDU_HOLD` rows identify work or evidence not completed by record repair. The applicable `ScopeOfWork.md` or PKG-00 `ArchitectureBasis.md` is the candidate governing home, with claim and named PDU references in the CSV. Before removal, verify each named test, unit, result, privacy, report, native or engineering boundary is explicit there; if not, apply an owning amendment or leave that bullet live. A future requirement needs no invented graph node. |
| Program scope and accepted ownership routes | Phase G/H/I and post-beta rows have accepted scope and staged gates. SCA-011 assigned DEL-04-07, DEL-07-11, DEL-07-12 and DEL-16-06 as four responsibility homes; its twelve local bullets (`:1`–`:3` for each) still require claim-specific evidence and preserve holds. `LOOP_INIT.md` selects no successor graph, so an owner route is not present execution allocation. |
| Owner decisions and formal review | PDU, RF, dependency-row, legal, policy, review, source-admission, lifecycle and release questions remain with their recorded authorities. Do not infer approval from technical repair, a named route or this census. The individually named cases are below. |
| Existing Task Management relationship | PKG-17 admitted-source questions already have deferred local rows `TM-PIP-002`–`022`; do not duplicate them from `DEL-17-*` bullets. D-58 successor-mechanism concern already has `TM-PIP-001`. No **new** row qualifies for promotion on present evidence: ordinary work has governing scope, reserved decisions have an owner, and the one explicitly unowned candidate-generator landing has an identified scope-change route. If that route cannot be used, reassess its materiality and missing home through a separate bounded intake. |

## Individually named exceptions and exact routes

1. **`DEL-01-01:NONE`, `DEL-01-02:NONE`, `DEL-01-03:1`–`:3`,
   and `DEL-11-02:1` — D-74 and issuance.**
   `D74_INTEGRATION_2026-09-22/HANDOFF.md` records the owner's MIT selection,
   but DEL-01-01 remains `ISSUED`. Its license consequence requires the
   deliverable's formal-change authority before touching its issued baseline.
   DEL-01-03 remains `IN_PROGRESS`: DEC-027/079 keep external intake closed;
   legal-instrument choice and pre-release §17.5 review require owner/counsel;
   sanitized export still must wire the public issue templates (also
   DEL-10-04). DEL-01-03 `ScopeOfWork.md` carries intake/legal sufficiency but
   does not expressly name §17.5 issue-template review or root wiring. The
   bounded D-74 handoff is evidence, not the issued formal change. A further
   **current-carrier conflict** is concrete: the DEL-01-03 Scope of Work still
   states `PolyForm-Noncommercial-1.0.0` as the *selected current* license in
   its Datasheet, current declarations, requirement verification, Procedure,
   Guidance and conflict table (for example lines 50, 124, 139, 180, 240,
   289, 301, 322, 347 and 406). D-74 explicitly selected MIT. Those current
   assertions need a source-faithful D-74 amendment while preserving genuine
   historical statements as history; simple Remaining deletion would leave a
   false current license declaration.
   The same selected-license conflict appears in `DEL-01-02/ScopeOfWork.md`
   (line 136) and `DEL-11-02/ScopeOfWork.md` (line 447); both should receive
   source-faithful current-authority corrections. The ISSUED
   `DEL-01-01/ScopeOfWork.md` contains many current PolyForm assertions,
   including normative requirement and acceptance text. D-74's MIT act does
   not itself rewrite that issued baseline: its formal-change procedure must
   govern the exact amendment. These obligations exist despite the
   DEL-01-01 and DEL-01-02 `NONE` markers.
2. **Architecture and formal review.** `DEL-00-02:1/:2` need the actual
   layer/module map and architecture REVIEW sufficiency; `DEL-03-04:2`,
   `DEL-04-04:1`, `DEL-04-05:2`, and `DEL-04-06:1` retain their formal REVIEW
   or dimensional-basis acts. PKG-00 uses `ArchitectureBasis.md`, not a
   deliverable Scope of Work. No authored map or formal outcome is inferred.
3. **D-72 performance and D-68 usability.** `DEL-07-01:3`,
   `DEL-07-02:2`, `DEL-07-06:2`, `DEL-07-09:2` share the redesigned-product
   performance, settled-frame and resource obligations. D-72 and its addendum
   have ruled all six criteria; S-1/S-2 apply and S-3 was rejected. The future
   demonstration remains open. The four proposed local carriers do not yet
   name these D-72 duties; use the exact decision and addendum as the governing
   destination and add the cross-references proposed below before removing
   these four bullets. `DEL-07-01:5`, `DEL-07-02:4`,
   `DEL-07-06:4`–`:6`, `DEL-07-09:4`, and `DEL-07-03:2` preserve the observed
   short-panel/AX limits and independent usability/security validation; the
   D-68 touched-control target is not whole-product WCAG evidence. These are
   shared obligations, not separate new register concerns. The specific
   short-panel/AX observation is in the bounded ROOT
   `FINAL_ACCEPTANCE.json` (`accessibility` and `observations`), but is not
   stated by those terms in the governing Scope of Work. Retain these four
   observation bullets until the exact carrier amendment below is applied;
   the broader D-68 text alone is insufficient preservation.
4. **`DEL-07-09:1`–`:9` — bespoke form.** This deliverable has no
   `ScopeOfWork.md`. Its accepted SCA-009 Vocabulary Annex, current
   `Capability_Comparison.csv` and `Palette_Operation_Routing.md` are the
   proposed destination for coverage and residuals. Historical rows 14–16
   are closed only for their bounded evidence; R1–R3, D-58 provider,
   DEL-16-03 durable history, broader toolkit and lifecycle remain open. Check
   the complete compound meaning against those records before deletion.
   `DEL-07-09:9` also records a distinct evidence-recovery duty: the formerly
   cited `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` was unavailable in the
   inspected tree, and its asserted v11 revalidation is unverified. Locate
   the record or perform bounded replacement verification before relying on
   that assertion. The existing N7 V2 PASS is evidence for its own frozen
   source only; apply the exact routing-ledger amendment below or retain the
   source bullet.
5. **`DEL-07-05:1/:3` — one rotational visualization obligation.** The
   accepted result viewer Scope of Work expressly says emitted `rx/ry/rz`
   rows are reviewable but its overlay is translational only. Retain one
   obligation, including the claim-specific acceptance hooks in item 3;
   do not treat duplicated wording as implementation.
6. **PDU/RF owner dispositions.** `DEL-09-01:4`, `DEL-09-03:1`,
   `DEL-10-01:2`, `DEL-11-04:1`, `DEL-12-03:3`, `DEL-12-04:2`,
   `LIVE:DEL-12-01:2`, `DEL-13-02:2`, `DEL-14-03:3`, `DEL-15-01:3`,
   `DEL-15-02:2`, `DEL-15-03:1`, `DEL-15-04:1`, `DEL-16-01:2`,
   `DEL-16-04:4`, `DEL-17-03:1`, `DEL-17-05:3`, and `DEL-17-07:2`
   explicitly retain `HumanDisposition=TBD` or an equivalent owning act.
   `DEL-13-02:2` additionally warns that force-per-length enum parity is not
   evidenced. Preserve technical and human tracks separately. `DEL-17-05:2`
   stays `VERIFIED_NOT_VALIDATED` pending the user-owned live CAEPIPE profile;
   a fixture does not authorize that execution.
7. **`DEL-16-04:3` — candidate generator owner.** DEC-094 and the SCA-009
   landing explicitly reserve route/support candidate-generator ownership to
   a separate act. The existing `DEL-16-04` rationale/boundary contract and
   Phase I program do not assign its production implementation. Seek the
   owning decomposition/scope-change act if this work is selected; do not
   create a presumed DEL-16-04 task or duplicate `TM-PIP-001`'s D-58
   mechanism concern.
8. **`DEL-05-04:1` — negative evidence.** The implemented applier's
   stale-model-hash check does not invalidate external human-acceptance
   records. `FG-DEL-05-04-01` and `REQ-05-04-008/014` remain the owning
   negative until an authorized product/evidence basis or scope decision.
9. **`DEL-17-06:1`–`:3` — versioned export and stale derivatives.** The
   accepted 0.2 foundation did not close actual producer/export proof. The
   830-row fixture has exactly two withheld unit-preservation witnesses,
   named in item 2; this does not allege dropped rows or a universal defect.
   `_SEMANTIC_LENSING.md` still calls hash partitioning TBD, and
   `_DEPENDENCIES.md` names DAG-007 despite the accepted DAG-011. Its item 3
   called for a DAG-010 refresh at that earlier source time; the actual
   dependency workflow must now use DAG-011. Choose current-source
   regeneration or explicit retirement from current use for the two semantic
   derivatives, and refresh dependencies through their owning workflow before
   removing this live reminder.
10. **`DEL-02-05:3` — H2/F-5b migration hardening.** The present Scope of
    Work describes the interim TypeScript `evaluateModelDocumentLocal` mirror
    and migration status but does not preserve the planned WASM-compilable
    shared evaluator, migrated-byte hash-integrity edge, or cross-engine
    parity tests. The retired, non-governing PRD completion plan H2 row is
    historical provenance; the current `DEL-02-05:3` source carries the
    surviving obligation. The three stale UI version comparisons were separately closed by
    DEC-074/PDU-024. Add the exact future-work clause below before removing
    this source bullet.
11. **`DEL-04-04:7` — nonlinear export vocabulary.** The bullet expressly
    assigns the count/flag/state-code and residual-observation vocabulary
    boundary to DEL-08-04. Neither the local solver Scope of Work nor the
    current DEL-08-04 export contract contains that exact duty. Add the
    receiving export-contract clause and source-owner cross-reference below;
    the producer and export responsibilities stay distinct.
12. **`DEL-05-03:3` — connector ownership unresolved.** The local stress
    recovery Scope of Work preserves pressure behavior and public result
    naming, but its bounded implemented contract is straight-pipe/mechanics
    recovery and does not name connector treatment. Preserve this compound
    row's connector clause in `Remaining` until the owning scope/decomposition
    decision identifies its real receiving contract; do not infer that
    DEL-05-03 or a different deliverable already owns it.
13. **`DEL-08-01:1` — package compatibility.** The calculation-report Scope
    of Work is not the home for `.opsproj` package compatibility-window and
    versioning policy. DEL-02-05 `REQ-02-05-021` retains the migration and
    compatibility-window hold; DEC-028 governs the portable package boundary.
    Add the exact physical-package/versioning cross-reference below or keep
    the DEL-08-01 source bullet.
14. **`DEL-10-04:2` — two release horizons.** Its Scope of Work preserves
    DEC-089's future Developer ID/notarization policy and DEC-057's unsigned
    posture, but does not identify the separate historical PRD §22.6 R6-entry
    release-machinery residual under D-21/DEC-056. Add that distinction below
    before removing this compound bullet; no signing or release is implied.

## Exact proposed amendments before source removal

These are text proposals, **not applied changes**. The owner must authorize
any substantive scope or issued-baseline change. They are the minimum visible
gaps found in the governing forms, not a claim that all other rows have passed
destination-level semantic backcheck.

| Destination | Proposed amendment and authority condition |
|---|---|
| `DEL-01-01/ScopeOfWork.md` issued baseline | Under its formal-change route, reconcile all present-tense selected-license, requirement, verification, acceptance, procedure and guidance statements to the D-74 MIT ruling while preserving the dated 2026-06-03 PolyForm decision as superseded history. Do not edit the issued artifact as an ordinary in-progress document or imply legal review/issuance from D-74 alone. |
| `DEL-01-02/ScopeOfWork.md` current protected-data boundary and `DEL-11-02/ScopeOfWork.md` license/contributor row | Replace each present-tense “selected project license is `PolyForm-Noncommercial-1.0.0`” with “The owner selected MIT under D-74; current repository license is MIT.” Retain contributor-instrument, legal sufficiency and external-intake gates under DEC-027/079. Preserve any genuine historical 2026-06-03 reference as history. |
| `DEL-01-03/ScopeOfWork.md`, every **current** license assertion | Replace “selected license is `PolyForm-Noncommercial-1.0.0`” (and equivalent present-tense clauses) with “The owner selected MIT under D-74; the current repository license is MIT. Contributor legal-instrument choice, legal sufficiency and future intake activation remain separately owner/counsel held under DEC-027/079.” Review each of the named current Datasheet, declaration, requirement/verification, Procedure, Guidance and conflict-table occurrences; preserve dated historical source claims with explicit historical framing. Link the D-74 ruling and overlay. This is a warranted authority-currentness amendment, not a retroactive legal conclusion or the DEL-01-01 issued formal change. |
| `DEL-01-03/ScopeOfWork.md`, current delivery/verification section | “Before any public release, complete PRD §17.5 owner/counsel legal review of public-facing language, including the E6 issue templates. When the sanitized public export is exercised, verify that its root `.github/ISSUE_TEMPLATE/` receives the project templates. Contributor intake remains closed under DEC-027/079 until a later owner activation and legal-instrument act.” Preserve DEL-10-04's export owner; do not represent MIT as counsel review. |
| `DEL-02-05/ScopeOfWork.md`, migration verification and current-mirror passage | Add: “H2/F-5b remains future hardening as recorded in the current DEL-02-05 status, with the retired PRD completion plan H2 row as historical provenance: relocate DEC-019 migration evaluation into a WASM-compilable shared crate, replace `projectService.ts::evaluateModelDocumentLocal`, and test cross-engine parity including migrated-byte hash integrity. The present TypeScript mirror is interim behavior. DEC-074/PDU-024 closed three stale UI version comparisons only; that closure does not discharge H2/F-5b.” Preserve existing DEC-019/033 status semantics and obtain the owning implementation/evidence route before claiming completion. |
| `DEL-08-04/ScopeOfWork.md`, result-export contract, with a DEL-04-04 source-owner cross-reference | Add: “Define and accept the result-export vocabulary for nonlinear active-set count, flags, state codes and residual observations supplied by DEL-04-04. Preserve producer meaning, unit/status context and omission diagnostics at the export boundary; no unpublished naming or acceptance threshold is inferred.” In DEL-04-04, link this receiving contract without moving solver producer obligations. Keep `DEL-04-04:7` until both references and the receiving scope are verified. |
| `DEL-02-05/ScopeOfWork.md`, `REQ-02-05-021`/physical-package boundary | Add: “The `.opsproj` package compatibility-window size and versioning policy beyond `schema_version 1.0.0` remain to be selected under DEC-028 and the migration authority. Current document-version classification does not decide the portable-container reader window.” Preserve the existing `TBD` rather than inventing a numeric window, and link the DEL-08-01 source clause to this owning contract. |
| `DEL-10-04/ScopeOfWork.md`, release verification/limitations | Add: “DEC-089 selects only a future Apple Developer ID signing/notarization target. The historical PRD §22.6 R6-entry release-machinery residual under D-21/DEC-056 remains a separate future gate; DEC-057's current unsigned package posture, checksums, commit-bound sweep, release record and unsigned-install caveat continue until the applicable requirements are met and accepted. Neither a registered App ID nor this note establishes signing, notarization, issuance or release.” |
| `DEL-07-01/ScopeOfWork.md`, `DEL-07-02/ScopeOfWork.md`, `DEL-07-06/ScopeOfWork.md`, and `DEL-07-09/Palette_Operation_Routing.md` | Add the same bounded authority pointer to each applicable performance/acceptance section: “Redesigned-product performance acceptance, settled-frame behavior and owned-resource obligations remain open under D-72 and its final addendum. Apply the five inherited D-68 numeric limits, the accepted reference profile, geometry, workload and observation rules, and the S-1/S-2 run/pass rule; S-3 was not accepted. The decision records govern exact criteria. No present implementation or historical demonstration qualifies a later candidate. Select and run a successor demonstration before asserting acceptance.” Link both decision records and retain each source bullet until its carrier and receiving undertaking are verified. This adds no threshold or release gate. |
| `DEL-07-06/ScopeOfWork.md`, D-68 current bounded target/evidence section | “The accepted bounded UI foundation observed that short panels may require scrolling between a label and its fully visible control. Its transient inspector accessibility-tree omission remains unexplained even though fresh processes expose the controls. Preserve that observation for candidate-bound repair/review and independent practitioner usability; it is not a whole-product WCAG or security finding and creates no extra publication gate. Source: the final ROOT acceptance record for the 2026-09-15 production UI tranche.” Link the DEL-07-01, DEL-07-02 and DEL-07-09 source clauses to this shared carrier and their own functional contracts. |
| `DEL-07-09/Palette_Operation_Routing.md`, historical reviewed-source paragraph | “The formerly cited `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` was not available in the inspected project tree. Its asserted v11 revalidation remains unverified. Locate and bind that intake to the candidate it actually checked, or perform a bounded replacement verification before relying on the assertion. N7 V2 PASS and parent fan-in apply only to their frozen source; they do not accept a later UI candidate.” Preserve the existing source-bound review record and do not manufacture a historical intake. |
| `DEL-17-06/ScopeOfWork.md`, verification/limitations section | “For the accepted 830-row sparse fixture only, resolve or formally disposition unit preservation for `result:nonlinear-support:free-dof-work-residual` and `result:loadcase:load-L-200:nonlinear-support:free-dof-work-residual`. Retain every raw 0.2 row and keep other producers, schema conformance and row counts outside this fixture-specific conclusion.” |
| `DEL-17-06/_SEMANTIC.md` and `_SEMANTIC_LENSING.md` | Regenerate against the accepted D-67/CLM-042 selected-version schema/hash partition, or mark them explicitly historical and remove their current-evidence status. Do not rewrite their 2026-05-18 historical assertions as if they were originally current. |
| `DEL-17-06/_DEPENDENCIES.md` | Through the dependency workflow, replace the stale “DAG-007 is current” and mixed DAG-005/006 summary with a source-faithful DAG-011 summary after checking `Dependencies.csv`; do not change dependency rows or the DAG by this wording edit. |

For any other row whose exact surviving meaning is absent from its proposed
governing form, add a source-faithful, claim-specific amendment under its own
authority and keep that Remaining entry until the destination is verified.
The candidate CSV intentionally marks every destination as requiring that
backcheck; it does not claim file existence is adequate preservation.

## Grouped questions for the owner

The later human replies and their limits are in `OWNER_DIRECTION.md`; these
original candidate questions must not be read as still unanswered where that
record supplies an actual decision. The listed non-issued receiving-document
amendment authorization remains pending.

1. Approve the finite semantic disposition principle for the 20 markers,
   evidenced bounded-result clauses and true duplicates, subject to the
   source/destination backcheck and explicit exceptions above? This would
   retire list entries only, without accepting deliverables or underlying
   future requirements.
2. Authorize the specific receiving-document amendments above where their
   exact duties or current authority are absent, and direct the DEL-17-06 semantic derivatives to
   regeneration or explicit historical retirement? If neither route is
   selected, those source bullets remain live.
3. Confirm that owner-held legal, formal REVIEW, RF/PDU, D-68/D-72,
   CAEPIPE, scope/ownership and issued-baseline decisions remain at their
   named authorities, with no new Action Item promotion from this census?
   A separate formal-change act is needed for the ISSUED DEL-01-01 D-74 effect.
4. When the receiving documents and selected decisions are actually applied,
   authorize removal of only the verified source entries and a terse
   deliverable MEMORY pointer to this closed finite account. Future work
   remains in its governing form, never in MEMORY as a work list.

No answer is presumed. Rows lacking a selected destination or owning act stay
live; independent approved treatments can proceed without changing them.
