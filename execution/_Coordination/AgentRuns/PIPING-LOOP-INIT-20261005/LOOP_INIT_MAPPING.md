# LOOP_INIT mapping: where each current Piping rule belongs

Prepared by LM, a Type 2 TASK, for HELP_HUMAN (ROOT), run
`PIPING-LOOP-INIT-20261005`. This is analysis and proposal only. No existing
file was edited. The draft is in `LOOP_INIT_PROPOSED.md`, and the edits needed
elsewhere are in `CONSISTENCY_EDITS.md`.

**Basis.**
- `RUN/OWNER_DECISIONS.md`: changing Piping's LOOP_INIT into the binding form,
  with the consequential edits elsewhere; the steer may be elaborate where
  justified.
- `V4RUN/OWNER_DECISIONS.md`, the sections "How LOOP_INIT steers agents
  through the guidance" and "What LOOP_INIT is, and what an agent reads at
  entry". The test: a sentence stays in LOOP_INIT only if it is specific to
  Piping, instructs, and is stated nowhere else. Also: "No routers."; "No
  don't pin to editions."; no new record type for current state; the entry
  reading is the Agent User Manual's headings to three levels and the Field
  Book in full, then the work graph.

**Subject.** `P/loop/LOOP_INIT.md` at NUM HEAD `6c2cb5a4eb`: 146 lines,
8,475 characters. Line numbers below refer to that file. A `diff` with
`NUM/projects/chirality-app-dev/loop/LOOP_INIT.md` shows four hunks only: the
title, the `WORKING_ROOT` value, the purpose/scope and dependency pointers
(lines 14–16), and the added line 19 ("Piping also has ArchitectureBasis and
bespoke contracts; use the actual form."). Every other line is App v3's text
word for word.

**Precedent.** `V4RUN/LOOP_INIT_MAPPING.md`, its review
`V4RUN/reviews/MR-LOOPINIT.md`, and the result
`NUM/projects/chirality-app-v4/loop/LOOP_INIT.md`. Rows below cite the matching
App v4 row as "v4 #n". No row was copied on trust: every destination quote was
re-verified at NUM HEAD. Two App v4 outcomes have changed since that mapping:
- v4 #87 (the receipt's content) was class (c). Its proposed addition A1 has
  landed in AUM §13, so the same Piping rule is now class (a) (row 69).
- v4's optional A5 (recording human decisions) has also landed in AUM §13.

**Method.**
- Every quotation in this file and in `CONSISTENCY_EDITS.md` was checked
  verbatim with `grep -F` against a copy of its source in which every run of
  whitespace, line breaks included, was collapsed to one space
  (`tr '\n' ' ' | tr -s ' \t' ' '`). That joins wrapped lines. 194 quotations
  from 25 source files were checked; all were found. A second pass covered
  every double-quoted passage of this file; see "Quotation check" at the end.
- Section headings (lines 12, 25, 39, 60, 76, 93, 111 and 132) are not rows.
  The numbered-step structure they carry is class (d): the methods carry the
  procedure in their own structure (Field Book §5, subsections 1–6;
  construct §§1–4; bounded-reconciliation §§1–5).
- Paths in the draft were checked with `test -e` relative to `P` or `NUM`.
  Workflow names were checked against `NUM/workflows/index.json`: all eight
  are `kind: workflow`, `source: bundled`, `sourceRootId: chirality-root`.

**Abbreviations.**
- AUM: `NUM/docs/alignment-manual/CHIRALITY_AGENT_USER_MANUAL_v3.md`, the
  current edition named by the alignment-manual README.
- FB: `NUM/docs/alignment-manual/Project_Management_for_Human_Agent_Teams_Field_Book_v1.md`.
- construct, ckw, BR, TM: `NUM/workflows/` `construct-local-work-graph`,
  `coordinated-knowledge-work`, `bounded-reconciliation` and `task-management`.
- Root AGENTS: `NUM/AGENTS.md`. P/AGENTS: `P/AGENTS.md`.
- COORD: `P/execution/_Coordination/_COORDINATION.md`.

**Classes.**
- **(a)** Carried elsewhere. The row gives the destination and a quote.
- **(b)** Piping-specific; it stays. The row names the draft section.
- **(c)** General and found nowhere else. The row gives a proposed home and
  text.
- **(d)** Obsolete or superseded. The row gives the reason.

## Mapping table

| # | Current text (lines) | Class | Destination, quote, or reason | v4 |
|---|---|---|---|---|
| 1 | Title, "# Piping development loop" (1) | b | Kept as the draft's title. | — |
| 2 | Resolve `REPO_ROOT`; set `WORKING_ROOT`; paths are relative to it (3–4) | b + a | Kept in the draft's opening, which adds "unless they begin with `{REPO_ROOT}`" as App v4 does. The anchoring rule itself is P/AGENTS: "Resolve `REPO_ROOT` with `git rev-parse --show-toplevel`. Set `WORKING_ROOT` to `{REPO_ROOT}/projects/chirality-piping`." | 1 |
| 3 | "Enter through `init/dev-loop-init-prompt.md` with the selected role and the human's steering." (5–6) | a | The init prompt is the entry: "Read `{WORKING_ROOT}/loop/LOOP_INIT.md` and follow it within the owner's steering and live authority." AUM §2: "Establish the project and active role, read the applicable entry instructions, and recover the current undertaking before creating replacement work." | 2 |
| 4 | "This file owns the recurring development-loop procedure;" (6) | d | Superseded by the owner's accepted reading (V4RUN): "LOOP_INIT is the single place that binds the project to the general layers." The procedure is carried by the workflows and manuals. Sentences elsewhere that repeat "owns" are in `CONSISTENCY_EDITS.md` §A. | 3 |
| 5 | "project `AGENTS.md` supplies standing responsibilities, boundaries and checks." (7) | a + b | P/AGENTS: "This file holds Piping-specific constraints." The b part: the draft's binding sentence names project `AGENTS.md`, and entry step 2 reads it (see N3). | 4 |
| 6 | "Keep this file evergreen: undertaking selection and graph references come from the init steering and subsequent human directions; execution state lives in the selected work graph." (8–10) | a | construct §4: "Keep undertaking-specific paths and state out of reusable loop instructions." Construct's introduction: "The human's steering selects the undertaking". P/AGENTS: "Keep LOOP_INIT evergreen, with no undertaking-specific graph pointer or execution state." Root SPEC: "LOOP_INIT remains evergreen and carries no undertaking-specific pointer or execution state." | 7 |
| 7 | "Purpose and scope: `docs/PRD.md`," (14) | b | Draft, "Basis". | 23 |
| 8 | "`execution/_Decomposition/SOFTWARE_DECOMP.md`, and the relevant adopted amendments or design specifications." (14–15) | b + d | The pointer is kept (draft, "Decomposition"), now with `execution/_Decomposition/_LATEST.md` ("Latest: execution/_Decomposition/SOFTWARE_DECOMP.md revision") and `execution/_ScopeChange/_LATEST.md` ("This pointer selects only the accepted SCA-011 amendment"). The decomposition records each adopted amendment in its revision notes. "design specifications" is dropped (d): it names no distinct record set. `P/docs/RECONCILIATION_PROFILE.md` groups them as "AgentRuns records, approved plans, design specifications and frames, owner-direction records \| Recorded direction and rationale", which the draft's run-records pointer reaches. | 25, 26 |
| 9 | "Project dependency basis: `execution/_DAG/_LATEST.md`." (16) | b | Draft, "DAG". The pointer names an accepted version ("Latest DAG artifact status: approved_active_graph_authority") and its currency rule: "departures go to a currency audit (`_Evaluation/DAGCurrency/`) and a human decision." `execution/_Evaluation/DAGCurrency/` does not exist yet, so unlike App v4 the draft has no currency pointer. | 27 |
| 10 | "Deliverables: relevant `execution/PKG-*/1_Working/DEL-*/` folders and their accepted production form, MEMORY.md, dependencies and lifecycle _STATUS.md." (17–18) | b | Draft, "Deliverables". Checked at HEAD: 106 deliverable folders; 97 have `ScopeOfWork.md`, the 8 in PKG-00 have `ArchitectureBasis.md`, and DEL-07-09 has bespoke palette contract files; 106 have `_DEPENDENCIES.md`, 98 `Dependencies.csv`, 101 `MEMORY.md`. | 29 |
| 11 | "Piping also has ArchitectureBasis and bespoke contracts;" (19) | b | Draft, "Deliverables": "an accepted bespoke form such as PKG-00's `ArchitectureBasis.md`". | — |
| 12 | "use the actual form." (19) | a | construct §2 table: "Affected Scope of Work or another accepted production form". BR §2: "`ScopeOfWork.md` or another accepted production form". AUM §15: "Preserve future requirements and accepted bespoke representations; do not convert formats incidentally." | — |
| 13 | "Decisions and boundaries: project `AGENTS.md`," (20) | b + a | Draft, entry step 2. Root AGENTS: "Ordinary selective context consists of this Root `AGENTS.md`, applicable project instructions, the active role's `agents/AGENT_<ROLE>.md`, and available skill names and descriptions." AUM §2's orientation block lists `cat "$WORKING_ROOT/AGENTS.md"`. See N3 for why it is an entry step. | 5 |
| 14 | "applicable entries in `execution/_Coordination/_DECISIONS/_REGISTER.md`, and relevant notices." (20–21) | b | Draft, "Decisions". The register's header says how rulings are recorded: "Record accepted rulings per existing decision practice (`DEC`/`SCA` entries in `execution/_Decomposition/SOFTWARE_DECOMP.md` or a successor register), then update the row here with a pointer." Notices: 74 files `execution/_Coordination/NOTICE_*.md`; Root AGENTS: "Notices communicate changes; each receiving loop decides its adoption and updates its own accepted basis." | 40 |
| 15 | "Checks: `software-workflow.json`" (22) | b + a | Draft, "Checks", as a pointer only. P/AGENTS: "Software work uses `software-workflow.json` under the root `docs/SOFTWARE_WORKFLOW_PROFILE.md` contract." | 36 |
| 16 | "and the applicable project verification rules." (22) | a | P/AGENTS, "Software checks", for example: "Exercise affected user workflows as they become operable alongside code tests". Read at entry step 2. | 36 |
| 17 | "Task Management register: `execution/_Coordination/_TaskManagement/REGISTER.csv`." (23) | b | Draft, "Task Management". | — |
| 18 | "The init steering and subsequent human directions establish the purpose, phase, priorities and limits." (27–28) | a | construct §1: "Read the init steering, subsequent human directions and relevant decisions. Establish the result, priorities, approach, limits and conditions for completion." | 41 |
| 19 | "Recover the graph for that undertaking from the supplied references and relevant project records." (28–29) | a | construct §1: "Recover a continuing undertaking through the current graph and compare its position with actual work before planning it again." construct §4: "Return its path so the continuing session can locate it through its steering or project records." | 41 |
| 20 | "Read it, relevant deliverable contracts, MEMORY run pointers, dependencies, implementation and evidence." (29–30) | a | construct §2 source table, rows "Affected Scope of Work or another accepted production form", "Deliverable dependency records and their cited evidence", "Implementation, tests and actual verification/validation results" and "Relevant decisions, scope changes, holds and MEMORY run pointers". | 42 |
| 21 | "Give a concise reading of the intended outcome and proceed where the direction is clear." (30–31) | a | construct §1: "State the reading of the undertaking briefly, distinguishing direction from interpretation. Proceed where intent is clear." | 42 |
| 22 | "Verify branch/worktree state, partial edits, active workers and prior integration before repeating work." (33–34) | a | AUM §18: "Inspect the selected cursor, branch and worktrees, staged and unstaged changes, outputs, still-running workers, and shared resources." FB §5, subsection 1: "Verify that earlier work was integrated before repeating it." | 43 |
| 23 | "Recover the actual graph and its accepted basis when continuing an undertaking." (34–35) | a | FB §5, subsection 1: "Read the human's steering and the accepted basis. Compare the current graph with actual revisions, local and unmerged work, evidence, open decisions, holds, and active operations." construct §1 (row 19). | — |
| 24 | "Preserve unrelated edits" (35) | a | AUM §11: "Preserve unrelated user work." P/AGENTS: "Preserve unrelated and unmerged work". | 44 |
| 25 | "and transfer shared-file or test-resource ownership explicitly." (35–36) | a | AUM §18: "Establish that prior workers have stopped or explicitly transfer ownership before reassigning their files or application state." FB §5, subsection 1: "Confirm that previous workers have stopped or transfer ownership before reassigning their files or resources." FB §5, subsection 3: "Control shared interfaces, applications, test fixtures, and other resources." | 44 |
| 26 | "Missing or stale pointers require recovery from their sources; they are not permission to restart completed work." (36–37) | a | AUM §18: "If a pointer is missing or contradictory, search the pertinent records and history for the intended target." construct §1: "A missing target requires recovery; a previous proposal or paused assignment is not new authorization." | 45 |
| 27 | "Use `chirality-root:bundled:workflow:construct-local-work-graph` when no graph exists or its route needs substantial revision." (41–42) | a + b | The method is the workflow. Its adoption by Piping is b (draft, "Methods"); construct §3 already names Piping: "For a loop that adopts this method (currently App, Piping and PEC), use the graph's stable run ID for one receipt at `execution/_Coordination/AgentRuns/<RunID>/RECEIPT.md`". | 46 |
| 28 | "Relate the intended outcome to the project DAG, deliverables and present work." (42–43) | a | construct §2 table: "Locate the intended contribution, required upstream inputs and affected downstream consumers." | 47 |
| 29 | "Before the first DAG exists, follow the phase steering and applicable dependency/cycle-resolution method." (43–44) | d | Obsolete: `_DAG/_LATEST.md` names an accepted version ("Latest DAG artifact status: approved_active_graph_authority"). The general no-DAG case is in construct §2: "A project without an accepted DAG uses its recorded registers (SPEC §5.3)." | 48 |
| 30 | "Create the Git-tracked graph at `execution/_Coordination/WorkGraphs/<undertaking>/WORK_GRAPH.md`." (46–47) | a + b | construct §4: "save the current graph exactly at `execution/_Coordination/WorkGraphs/<undertaking>/WORK_GRAPH.md`". The folder stays as a pointer (b, draft "Work graphs"). | 52 |
| 31 | "Return its path for continuation and commit it in the undertaking's PR sequence early enough for handoff." (47–49) | a | construct §4: "include it early in the undertaking's PR sequence for handoff" and "Return its path so the continuing session can locate it through its steering or project records." | 52 |
| 32 | "Keep the graph current across sessions" (49) | a | construct introduction: "One graph carries the undertaking across sessions." construct §4: "include it early in the undertaking's PR sequence for handoff, then update it in later PRs." | 52 |
| 33 | "and preserve historical graphs at their existing locations." (49–50) | a | construct §4: "Preserve historical graphs at their original paths." | 52 |
| 34 | "Plan substantive implementation and evidence through PRs, each carrying the documentation, reconciliation and conditional Task Management work its slice needs." (52–54) | a | construct §3: "Plan substantive PRs, each carrying the documentation, reconciliation and conditional Task Management consequences its slice needs. A PR boundary alone requires neither a formal reconciliation pass nor a Task Management intake." | 53 |
| 35 | "Then plan one final bounded documentation/governance closeout stage: reconciliation, conditional Task Management, one central loop receipt, terse MEMORY entries and the final PR." (54–56) | a | construct §3: "After the intended implementation/evidence PRs, plan one final bounded closeout stage. It includes documentation/governance reconciliation through `chirality-root:bundled:workflow:bounded-reconciliation`, conditional Task Management, the invoking loop's central receipt, terse MEMORY run entries, and the final PR." FB §5, subsection 2: "Prepare one planned final documentation and governance closeout after the intended implementation and evidence have been integrated." | 53 |
| 36 | "Steps 2–6 supply the mechanics;" (56) | d | The numbered steps are removed; the workflows supply the mechanics. | 14 |
| 37 | "no formal reconciliation pass is required after every node." (56–58) | a | construct §3 (row 34, second sentence). BR introduction: "Those consequences do not require a formal pass after each node." | 53 |
| 38 | "Use Agent 0/1/2 responsibilities to maintain alignment, manage connected work and execute bounded contributions." (62–63) | a | Root AGENTS roles table: HELP_HUMAN "Work with the human on alignment, continuity, and coordination"; WORKING_ITEMS "Organize implementation, assign bounded work, and integrate results"; TASK "Execute one bounded assignment, using a workflow when selected". | 66 |
| 39 | "Managers integrate their children's returns and advance independent ready work." (63–64) | a | Root AGENTS (row 38, WORKING_ITEMS). AUM §9: "advance ready authorized work without requesting approval for every routine node". | 69 |
| 40 | "Size concurrency to actual review and integration capacity," (64–65) | a | FB §5, subsection 3: "Size concurrency to the team's capacity to examine and integrate returns." | 69 |
| 41 | "with one owner for shared writes." (65) | a | FB §5, subsection 3: "Keep shared writes under one owner." P/AGENTS: "Give concurrent assignments disjoint write scopes or one integration owner." | 69 |
| 42 | "Implement, verify, validate where applicable, review, repair and integrate via PRs under the graph and project requirements." (67–68) | a | FB §5, subsection 4, "Produce, examine, repair, and integrate": "Verification examines conformity to specified requirements. Validation examines fitness for the intended use." and "Integrate against the actual receiving state and examine the combined behavior." AUM §11. | 70 |
| 43 | "Each PR carries the document, reconciliation and governance consequences needed for that slice, including a qualified Task Management transfer when needed under Step 4." (68–70) | a | construct §3 (row 34). P/AGENTS: "Each substantive PR includes the documentary, reconciliation and conditional Task Management consequences needed for that slice." The cross-reference "under Step 4" goes with the removed steps (row 36). | 71 |
| 44 | "Exercise meaningful connected behavior" (70–71) | a | AUM §11: "Exercise the actual consumer route as soon as it becomes operable." FB §5, subsection 4: "Exercise connected behavior and the relevant environment as well as component behavior." P/AGENTS (row 16). | 72 |
| 45 | "and record the actual candidate and evidence." (71) | a | AUM §11: "Use one canonical copy with references from the graph or deliverable." FB §5, subsection 4: "Preserve the actual inputs, candidate, relevant observations, and limitations." | 72 |
| 46 | "Prepare consequential decisions for the human while unaffected work proceeds." (71–72) | a | AUM §11: "Hold the affected acceptance or merge while independent work proceeds." construct §3: "Keep blockers and uncertainty truthful while independent authorized work proceeds." FB §2: "Bring decisions reserved to the human as concrete choices with evidence and consequences." | 73 |
| 47 | "Keep required production work in the graph until its conditions are satisfied; an intermediate merge does not finish the undertaking." (72–74) | a | AUM §9: "An intermediate merge or Task Management transfer cannot discharge it." AUM §18: "Earlier substantive merges remain intermediate results." | 74 |
| 48 | "After integrating the intended implementation and evidence work—normally the penultimate merge—perform one planned documentation/governance closeout stage." (78–79) | a | BR introduction: "The stage follows integration of the intended implementation and evidence work—normally the penultimate merge—and precedes the final PR." P/AGENTS: "Perform one final bounded documentation/governance closeout after the undertaking's intended implementation and evidence integration (normally the penultimate merge), before the final PR." | 75 |
| 49 | "Use `chirality-root:bundled:workflow:bounded-reconciliation` for bounded per-deliverable comparisons as needed within that stage." (80–81) | a + b | construct §3 (row 35) names the workflow for the closeout. BR introduction: "The manager may divide the affected deliverables into bounded assignments". Its selection for Piping is b (draft, "Methods"). | 75 |
| 50 | "The closeout precedes the final PR." (81–82) | a | BR introduction (row 48). | 75 |
| 51 | "Compare the delivered result and evidence with the actual Scope of Work, dependency and governance records;" (82–84) | a | BR §2: "Read current controlling decisions and the affected contents of these files", with its table of the production form, dependency records, MEMORY, references and status. | 76 |
| 52 | "apply warranted edits and accepted decisions." (84) | a | BR §3: "Correct outdated descriptions, resolved details, evidence references and permitted dependency statements. Apply accepted design decisions within their actual scope." | 76 |
| 53 | "Preserve future requirements and owning authority for scope, lifecycle, protected criteria and issued baselines." (84–85) | a | BR §3: "Future requirements remain requirements when implementation falls short" and "Changes to accepted scope, lifecycle, protected criteria or a pinned authority basis follow their owning decision path." BR §1: "If an affected deliverable is CHECKING or ISSUED, or lies inside an active concordance run's frozen scope, do not edit it." | 77 |
| 54 | "This closeout is bounded to the undertaking." (87) | a | BR introduction: "this does not introduce recurring passes during graph traversal or activate whole-corpus concordance." | 78 |
| 55 | "A supported no-change result is sufficient." (87–88) | a | BR §3: "a supported no-change result is sufficient when the affected record already represents the result accurately." | 78 |
| 56 | "If it finds missing required implementation or evidence, return that work to the graph and repair it before completing closeout." (88–90) | a | BR §5: "If missing implementation or evidence is found, repair that graph work and recheck only the affected comparisons before completing this closeout." | 78 |
| 57 | "Recheck affected comparisons after a change;" (90) | a | BR §5 (row 56), and: "If inputs changed, recheck the affected comparison or report stale claims." | 78 |
| 58 | "do not declare required work complete through a report or transfer." (90–91) | a | BR §4: "A transfer never fulfills required production or authorizes a foreign assignment." BR introduction: "A report alone does not complete authorized document edits." | 78 |
| 59 | "For a substantive PR or the final closeout, first resolve a concern through authorized graph work, a warranted document amendment, or the owning decision/scope-change route." (95–97) | a | BR §4: "First handle a consequence through the authorized graph, a warranted amendment, or the actual owning decision/scope-change route." AUM §13: "Apply the same eligibility during a substantive PR and the final closeout". | 79 |
| 60 | "Work already allocated to an identified successor stays there." (98) | a | FB §5, subsection 5: "Keep work already allocated to the current graph or an identified successor there." | 79 |
| 61 | "Ordinary future requirements remain in their governing scope." (98–99) | a | BR §4: "Preserve ordinary future requirements in their governing scope." | 79 |
| 62 | "Only a material, evidenced concern without a current or identified successor home qualifies for bounded Task Management intake." (101–102) | a + b | FB §5, subsection 5: "Consider Task Management intake only for a material, evidenced concern without such a home." The loop's permission is b (draft, "Methods"), because TM says "A loop may authorize bounded intake under its own stated conditions" and BR §4 says "when the calling loop permits it". | 80, 38 |
| 63 | "Give it to WORKING_ITEMS selecting `chirality-root:bundled:workflow:task-management`, with the source, significance, missing home and proposed treatment." (102–104) | a | AUM §13: "WORKING_ITEMS selects `chirality-root:bundled:workflow:task-management` directly with the invoking loop, purpose, authority and exact write boundary." BR §4: "Supply the concrete concern and its grounds". | 81 |
| 64 | "Perform that workflow's federation preflight; no general harvest is required." (104–105) | a | TM, step 2: "Perform the invocation-local federation preflight". TM, bounded intake: "Do not expand this into a general harvest". | 38 |
| 65 | "Promotion, disposition and external assignment remain actual human acts." (105–106) | a | AUM §13: "Promotion, disposition and external assignment remain actual human acts." | 82 |
| 66 | "Retain the outcome or pending intake in Task Management and link it from the originating PR/graph node and final closeout as applicable." (106–108) | a | AUM §13: "Pending intake remains in its Task Management home, linked from the originating PR/node and closeout as applicable." | 83 |
| 67 | "Routing does not satisfy an unmet requirement or permit graph completion that depends on it." (108–109) | a | TM: "An open intake does not discharge an unmet requirement in the originating undertaking." FB §5, subsection 5: "A transfer or deferral does not satisfy an unmet requirement." | 83 |
| 68 | "Near final PR preparation, write one receipt for the undertaking at `execution/_Coordination/AgentRuns/<RunID>/RECEIPT.md`, using the graph's stable run identity." (113–115) | a + b | construct §3 (quoted in row 27). Piping's adoption is b (draft, "Methods"). Piping already follows it: two `AgentRuns/<RunID>/RECEIPT.md` files exist. | 86 |
| 69 | "It is a concise, derivative account of what landed, affected deliverables, actual PRs, checks and evidence, decisions or Task Management transfers, and material limits." (115–117) | a | AUM §13 (v4's A1, landed): "It is a concise, derivative account of what landed: the affected deliverables, the actual PRs, the checks and evidence, any decisions or Task Management transfers, and material limits." | 87 (was c) |
| 70 | "Use its result/checks/limits account for the final PR description," (117–118) | a | AUM §13: "Its result, checks and limits supply the final PR description." BR §5: "The invoking loop's concise receipt may supply its final PR-sized result/checks/limits account". P/AGENTS: "The final PR description uses the receipt's result/checks/limits account". | 87 |
| 71 | "adjusting links for the PR surface." (118) | d | Editorial practice, not a rule that needs a home. Nothing general or Piping-specific depends on it. | — |
| 72 | "Keep detailed logs at their sources and link the graph;" (118–119) | a | AUM §13: "It is not a second execution graph, a future-work list or a decision record. Link the graph and the detailed sources rather than copying them." construct §4: "Keep detailed launch histories, child attribution, source hashes and old pauses in their owning run records". | 88 |
| 73 | "the receipt is neither a second execution graph, a future-work list nor decision authority." (119–121) | a | AUM §13 (row 72, first sentence). | 87 |
| 74 | "A graph node or Task Management invocation does not create another loop receipt." (121–122) | a | AUM §13: "A graph node or a Task Management invocation creates no further receipt." TM, step 5: "Do not create a separate Task Management receipt or a MEMORY work list." | 89 |
| 75 | "Then add a terse entry to each affected deliverable's `MEMORY.md`: run/date, what this run did there, and a link to the central receipt." (123–125) | a + b | AUM §13: "add a terse entry near final PR preparation in each affected deliverable's `MEMORY.md` Runs table. Use a stable run ID and date, state the work performed in that deliverable, and link the PR, central evidence, applicable rulings, scope changes, and Task Management transfers." The b part is the MEMORY convention and creation grant (draft, "Conventions"; see N11). | 90 |
| 76 | "Add the relevant PR, decision, scope-change or transfer pointer when useful." (125–126) | a | AUM §13 (row 75). | 90 |
| 77 | "MEMORY is a local run index, not a decision record or future assignment." (126) | a | AUM §13: "The table has no future-work queue." P/AGENTS: "Decision authority stays at its owning source; memory carries no future assignments." | 91 |
| 78 | "Preserve existing history." (126) | a | AUM §13: "Preserve existing headings and entries." | 91 |
| 79 | "Include the receipt and MEMORY changes in the final PR;" (127) | a | construct §3 (row 35): the closeout stage includes the central receipt, the MEMORY entries and the final PR. | 93 |
| 80 | "use a stable run/branch link until its PR URL exists, then bind that URL before final checks." (127–128) | a | AUM §13: "While a PR URL is unavailable, use a stable run/branch link and bind the actual URL before final candidate checks. Do not assert a future merge SHA or require a later commit solely to record that merge." | 93 |
| 81 | "Do not claim a pending merge as complete." (128–129) | a | AUM §13 (row 80, second sentence). construct §3: "later Git/PR evidence establishes the actual merge." | 93 |
| 82 | "Root SPEC §9.8 still governs required multi-agent execution provenance." (129–130) | a | Root AGENTS: "Record the actual mechanism, parentage, supplied basis, scopes, enforcement limits, and returns." construct §4: "Required execution provenance remains recoverable through those links." P/AGENTS: "Required execution provenance stays recoverable." SPEC §9.8 exists ("### 9.8 Multi-agent run record"). | 94 |
| 83 | "Complete the graph's promised work, bounded closeout, central receipt and memory entries, and prepare the final PR with the integrated result and evidence." (134–136) | a | AUM §18: "complete its promised work, the single planned documentation/governance closeout, and affected deliverable Runs entries; then complete final PR review, required checks and decisions, and merge." | 95 |
| 84 | "Review and required checks must cover the actual final candidate;" (136–137) | a | Root AGENTS: "with review and validation covering the actual candidate revision". P/AGENTS: "required CI and review must cover the actual merging revision". | 95 |
| 85 | "resolve blocking findings and obtain the decisions reserved to the human." (137–138) | a | Root AGENTS: "Merge when required CI passes and independent review has no unresolved blocking findings". AUM §18 (row 83). | 95 |
| 86 | "Record readiness for final merge and the PR URL in the candidate graph." (138–139) | a | construct §3: "Its candidate records readiness and the PR URL; later Git/PR evidence establishes the actual merge." | 96 |
| 87 | "Verify the actual merged state afterward through Git or the PR service;" (139–140) | a | AUM §18: "Verify the merged fact afterward through Git or the PR service without requiring another commit solely to record it." | 96 |
| 88 | "do not assert a future merge SHA in the candidate or require a later completion-record commit." (140) | a | AUM §13 (row 80, second sentence). construct §4: "Do not require a later commit solely to write the final merge result back into its own candidate." | 96 |
| 89 | "The loop ends when the completed work graph's final PR merges under standing Git authority." (142–143) | a | AUM §18: "The revised App/Piping loops make final PR merge their terminal condition; an open required node, review hold or unmerged final PR leaves the undertaking open." P/AGENTS: "The loop ends when its completed graph's final PR merges after required checks, review and human decisions." Root AGENTS' standing Git authorization "permits current and subsequent agents to commit, push, open/update PRs, and merge within authorized work". | 97 |
| 90 | "A review hold, unfinished required node or unmerged final PR means it remains open." (143–144) | a | AUM §18 (row 89). | 97 |
| 91 | "On interruption, retain the candidate, open checks, active operations and next safe action in the graph." (144–145) | a | construct §4: "Keep one current account of the ready work, holds and next safe action, bound to the checked candidate." FB §7: "Preserve actual state, active operations, ownership, blockers, and next action." | 98 |
| 92 | "Final integration does not itself issue a deliverable, accept a product or authorize release." (145–146) | a | AUM §9: "A completed graph remains selected and visible; it does not start another phase, release a product, or issue a deliverable." P/AGENTS: "Source integration, observed checks, engineering acceptance, lifecycle issuance and release remain distinct." | 99 |

## Class counts

92 rows, counted by first-listed class:
- (a) 77;
- (b) 11;
- (c) 0;
- (d) 4.

Eleven rows are mixed:
- seven are a + b: rows 5, 27, 30, 49, 62, 68 and 75;
- three are b + a: rows 2, 13 and 15;
- one is b + d: row 8.

Counting every row with a Piping-specific part, 18 rows feed a line in the
draft: rows 1, 2, 5, 7–11, 13–15, 17, 27, 30, 49, 62, 68 and 75.

No class (c) rule was found. The one rule App v4 found in class (c), the
receipt's content, now has its general home in AUM §13 (row 69).

## Draft lines not in the current file

| # | Draft line | Source |
|---|---|---|
| N1 | Opening: "This file binds Piping to Root `AGENTS.md`, the active role, project `AGENTS.md`, the bundled workflows and the manuals. It states only what is specific to Piping and stated in none of them." | The owner's accepted reading (V4RUN): "LOOP_INIT is the single place that binds the project to the general layers." The brief requires the binding sentence to name project `AGENTS.md`. "stated in none of them" applies the owner's test to a project that, unlike App v4, has a project `AGENTS.md`. |
| N2 | Entry step 1: the editions from the alignment-manual README; the AUM headings to three levels; the Field Book in full | The owner (V4RUN): "Your suggestion about presenting just the Agent User Manual's headings and the Field Book is excellent and accepted." and "No don't pin to editions." The same text as App v4's step 1. At HEAD the command gives 34 heading lines. |
| N3 | Entry step 2: "Read project `AGENTS.md` in full before you write anything." | Rows 5 and 13. AUM §2 lists it in the orientation, but the entry reading gives only the AUM's headings, and Piping's init prompt reads Root `AGENTS.md`, the role and LOOP_INIT, not project `AGENTS.md`. No `CLAUDE.md` exists in `P`. Project `AGENTS.md` holds the write fences (F-PIP-1 to F-PIP-4, the knowledge-source rule for DEC-043, the write-scope rule), which apply before the first write. App v4 kept its thesis fence in LOOP_INIT for the same reason (v4 #33). The init prompt must byte-match the Root launcher's §5 block (`NUM/tools/validation/validate_instruction_entrypoints.py`), so adding it there would change two files; LOOP_INIT changes one. |
| N4 | Entry step 3: the work graph the steering names, or construct one | The owner (V4RUN): the entry reading ends with the work graph. Same text as App v4's step 2, first two sentences. App v4's GC-8 sentence is not carried: Piping has no groups or GC-8 ruling. |
| N5 | "When to read further" | The owner (V4RUN), on the tables of contents: "the agent decides when to visit it (and here guidance can be given around this level of decision making if done right)". App v4's text, with "as Root `AGENTS.md` requires" added after "Record what you read in the run evidence" (MR NOTE-L3; Root AGENTS: "Record their actual origins and hashes in governed run evidence"). See open question Q3 on its last sentence. |
| N6 | Methods: `coordinated-knowledge-work`, applied in proportion | Parity with App v4, where the owner directed (V4RUN): "We should incorporate the `coordinated-knowledge-work` workflow into the LOOP_INIT instructions too." No Piping-specific owner direction names it. Open question Q2. |
| N7 | Methods: `scope-change`, `dependency-extract`, `audit-dep-closure` and `project-dag` | The brief. Piping uses all four surfaces: SCA snapshots under `execution/_ScopeChange/`, DAG versions under `execution/_DAG/`, and `_DEPENDENCIES.md`/`Dependencies.csv` registers. The current file names none of these workflows. |
| N8 | Methods: "`construct-local-work-graph`: Piping adopts its graph, closeout, receipt and MEMORY conventions" | Rows 27, 30, 49, 68 and 75. construct §3 names Piping among adopting loops; this line is the loop's own binding. |
| N9 | Records: `_Decomposition/_LATEST.md`, `_ScopeChange/_LATEST.md`; the decision register's header; the decomposition's §12; notices; run records | Rows 8 and 14. The register header (row 14). COORD: "Codified rulings: `execution/_Decomposition/SOFTWARE_DECOMP.md` §12." The decomposition's heading: "## 12. Decision log". |
| N10 | Records: "**Work graphs.** `execution/_Coordination/WorkGraphs/`." | The brief; row 30. |
| N11 | Conventions: "create the file the first time it is needed" (MEMORY) | AUM §6: "optional `MEMORY.md` requires a selected, authorized path". construct §3: "MEMORY and closeout writes remain subject to the project's write fences; where a needed grant is missing, record the entry in the graph or receipt and route the grant." At HEAD, 5 of 106 deliverables have no `MEMORY.md` (DEL-04-07, DEL-07-09, DEL-07-11, DEL-07-12, DEL-16-06). The current file grants no creation. This is a new grant; open question Q1. |
| N12 | Conventions: `loop/LOOP_RECEIPTS.md` is closed, ends at Receipt 162, is not appended, and is not a new undertaking's receipt or recovery cursor | Checked: the ledger's last entry is "Receipt-ID: `Receipt-162`"; Receipt 161 was never written ("Receipt-161 remains reserved to the unrelated unmerged continuation."). Only the 2026-09-23 notice states its status: "The old `loop/LOOP_RECEIPTS.md` ledger remains historical and is not appended by this method." Notices are not entry reading, and the ledger's own header still calls it "Append-only" with "Rules (fixed — part of the loop protocol)". AUM §18 explains historical chains and their validators ("the revised App/Piping loop does not impose them on every graph continuation"), and P/AGENTS says "Historical receipt validation protects existing records and does not require new entries." Neither forbids appending. Same reasoning as v4 #84. |
| N13 | Standing constraint: stage gates are the exit criteria of the PRD §24 release milestones; COORD records the target; only the owner's approved update advances it | AUM §5: "The project loop states who assesses the position; the App v4 loop, for example, reserves each stage-gate assessment to the human." AUM §5: "Stage gates remain human-managed milestones under the project's own records." For Piping those records are COORD, "## Current Target Stage (ruled record)": "Agents propose stage advancement with evidence; only a human-approved coordination update advances the target stage recorded here." The target is a PRD release milestone ("## 24. Release Milestones"; DEC-048 and DEC-054 advanced it from R3 to R4 to R5). Piping's records use no percentage gates, so the FB §1 60% checklist does not define Piping's gates. The line is a pointer plus the AUM §5 statement; the draft names no current stage. |

## App v4 lines not carried

- **Groups, GC-7 and GC-8 (entry step 2, "Groups").** Piping has no
  development groups or GC rulings.
- **Change control and `app/CONTRACT_ISSUES.md`.** App v4's ruling GC-9. Piping
  has no equivalent ruling; P/AGENTS already says "An explicit ruling, adopted
  requirement or protected criterion requires its owning decision before
  reversal."
- **SWBPIPE, PEC and Domains; App v3; Thesis; "Code".** App v4 facts. Piping's
  code surfaces are in AUM §15, whose heading is in the entry map.
- **Codex scratch home, sign-in, downloads, offline builds.** App v4's owner
  answers. I found no Piping owner record making them standing (I searched
  the run records named in this file only, not every Piping record). Adding
  them needs the owner.

## Additions needed elsewhere

**Class (c): none required.**

**Optional O1 (from v4's A6): the "When to read further" paragraph.** With this
draft, two loops carry the same general paragraph, which strains the "stated
nowhere else" test. A home in AUM §1, "Use the guide at the right scale",
would let each LOOP_INIT point to it. Manual revisions are outside this
tranche, and the 2026-09-23 notice records that "The owner retains authorship
of the manual/guide update." The draft keeps the paragraph, as App v4 does.

**Workflow text (needs `create-workflow`).** construct's introduction says
"The human's steering selects the undertaking; `LOOP_INIT.md` supplies the
evergreen procedure for recovering, constructing and following its graph."
That stops being true for Piping, as it already is for App v4 (v4's A4, still
unapplied). See `CONSISTENCY_EDITS.md` §C3.

## Gaps met

These were seen from LOOP_INIT's perspective only.

1. **Stale entry routes.** `P/README.md` (line 43), `P/docs/README.md`
   (line 58) and COORD's "Pointers" (lines 103–104) still route entry through
   "the newest `loop/WORKPLAN_*.md`" and `loop/LOOP_RECEIPTS.md`. These were
   stale before this change. Proposed edits are in `CONSISTENCY_EDITS.md` §B.
2. **`P/README.md` item 2** names `docs/_Decomposition/SOFTWARE_DECOMP.md`,
   which does not exist; the decomposition is at
   `execution/_Decomposition/SOFTWARE_DECOMP.md`. Not proposed here (unrelated
   to LOOP_INIT).
3. **Manual statements about the Piping loop** that become or already are
   stale are listed in `CONSISTENCY_EDITS.md` §C1–C2. AUM §2 and §15 also
   quote a header that the live file no longer has: "Both checked headers say
   “none selected for a successor undertaking.”" and "At this basis, the
   published Piping loop says “none selected for a successor undertaking.”"
   AUM §15 also says "Recover the graph selected by the live loop, then its
   linked resume procedure, phase events, handoff, and later owner
   directions.", which predates the evergreen rule.
4. **P/AGENTS duplicates general layers.** Its paragraph on PRs, closeout,
   receipt and MEMORY (lines 82–92) restates construct, BR and AUM §13. The
   brief excludes a wholesale AGENTS revision, so only the sentences that
   would become untrue are edited.
5. **The decision-register header** scopes the register to
   `plans/PLAN_2026-06-17_prd_completion.md` §2 but also says "Newly
   discovered human-gated `TBD`s get new `D-XX` rows appended here", and
   later rulings (for example D-77) use it. The draft points to the header
   rather than restating it.
6. **Validator compatibility.** `NUM/tools/validation/validate_instruction_entrypoints.py`
   checks Piping's `loop/LOOP_INIT.md` and newest `loop/WORKPLAN_*.md` for
   duplicated role routing and runtime mechanics. I imported its pure
   function `_structural_duplication_findings` and applied it to the draft and
   to the proposed WORKPLAN edit in a scratch copy: no findings for either.
   The draft does not contain the retired phrase it also checks. I did not
   run the validator or any test suite; ROOT should run the registered checks
   on the candidate.
7. **Entry cost.** AUM headings: 34 lines, about 360 tokens. Field Book:
   23,497 bytes, about 6,200 tokens. Project `AGENTS.md`: 9,016 bytes, about
   2,370 tokens. Draft: 3,875 characters, about 1,020 tokens (characters /
   3.8). Entry is about 9,950 tokens before the work graph. The current file
   alone is 8,475 characters, about 2,230 tokens, and its pointers include
   project `AGENTS.md`.
8. **Notices and manifest.** A repository-wide search found no other loop's
   live instructions citing `projects/chirality-piping/loop/LOOP_INIT.md`
   except the Root launcher catalog and the manuals (search excluded
   `execution/`, `plans/`, proposals, archives, receipts and evidence).
   `NUM/tools/software_workflow/test_hosted_ci.py` lists the path only to
   classify a changed file as an instruction change. Root AGENTS requires a
   tranche manifest for this instruction change.

## Open questions for ROOT

- **Q1. The MEMORY creation grant (N11).** It is new: the current file
  grants no creation, and 5 deliverables have no `MEMORY.md`. Keep it, or
  drop the clause and rely on construct §3's fallback ("record the entry in
  the graph or receipt and route the grant")?
- **Q2. `coordinated-knowledge-work` (N6).** The owner directed it for App
  v4's LOOP_INIT. Is parity enough for Piping, or does it need the owner?
- **Q3. "If you are unsure whether something matters, ask the human."** It is
  App v4's accepted text, kept for parity. It sits uneasily with Root AGENTS,
  "uncertainty alone does not require an extra prompt.", though asking does
  not weaken Root. An alternative that keeps the reading guidance
  self-contained (proposed wording, not a quotation): *If you are unsure
  whether a section matters, read it.*
- **Q4. Records outside instructions.** Two required edits touch records that
  are not instruction files: the owner-adopted navigation in
  `loop/WORKPLAN_2026-09-19_piping_loop.md`, and the banner of
  `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` (DEL-11-05, `_STATUS.md`
  IN_PROGRESS). See `CONSISTENCY_EDITS.md` A6 and A7.
- **Q5. The §B edits** were already stale before this change. Include them in
  this tranche?

## Quotation check

Two passes, both on whitespace-joined copies.
1. **Attribution.** 194 quotations, each searched with `grep -F` in the one
   file it is attributed to (25 source files). All were found.
2. **Coverage.** Every double-quoted passage of 20 or more characters in this
   file (276) and in `CONSISTENCY_EDITS.md` (14), outside the Before/After
   blocks, was extracted per table row or paragraph and searched in all
   sources read for this run, including `P/loop/LOOP_INIT.md`, both
   `OWNER_DECISIONS.md` files and the draft. All 290 were found, and no
   paragraph has an unpaired quotation mark. Thirteen are found only in the
   draft; each is a quotation of the draft itself (the N rows), not a
   destination quotation. The Before blocks of `CONSISTENCY_EDITS.md` were
   checked separately (each occurs exactly once).
