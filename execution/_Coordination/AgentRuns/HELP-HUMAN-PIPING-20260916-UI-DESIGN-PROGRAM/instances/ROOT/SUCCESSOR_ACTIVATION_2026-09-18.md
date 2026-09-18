# Successor ROOT activation — the owner's handoff message, 2026-09-18

Status: record of the owner act that handed this program to a successor ROOT. The previous ROOT's last entry in [`../../HANDOFF_STATE_WORKING.md`](../../HANDOFF_STATE_WORKING.md) ("handoff to a successor ROOT") announced the handoff; this message performed it. The successor is a Claude Code session running `claude-fable-5-1` as HELP_HUMAN (Agent 0), in the program's original design harness, on branch `codex/swb-ui-design-handoff-reconciliation` cut from `origin/main` at `451c5f595e0488a0d6064d9b0f972c5e8fc1d09d` (fetched and verified as the tip at start; the PR #792 merge, which contains PRs #793, #794 and #795).

## 1. Custody

The text in §3 was extracted from the successor session's stored transcript: transcript timestamp 2026-09-18T12:48:25.562Z, 18,684 bytes of UTF-8, SHA-256 `d1c985e0ad17a771fd42dd418f0844d5038acf3dbf56a8565d82695eed4ac808`. The extraction begins at the owner's first word; a host-inserted worktree reminder that preceded it in the same stored message, and that carried a machine-local path, is not part of the owner's words and is left out. This is custody of the host's stored transcript bytes, not of original transport bytes. The file reproduces the bytes inside a fenced block; the hash is of the extracted bytes, not of this file.

## 2. What the message does and does not supply

- It supplies the scope (design research, planning, ideation, specification and mock drafts only; no product implementation), the sequencing facts (D-70's baseline-report condition satisfied; PRs #793 to #795), the order of work, the implementation-handoff constraints and the working conventions.
- It supplies **no ruling on any D-71 item**. It says so itself: "Do not assume that silence means acceptance, and do not infer a ruling from this handoff prompt." At activation the successor searched the repository for a recorded ruling and found none: the packet is PROPOSAL / AWAITING_RULING and the register row reads AWAITING_RULING. The nine items remain with the owner.
- It supplies no answer to the frames' questions Q-15 to Q-22; a search of the run found none recorded.
- It states that the piping session has read the rendering brief and its notice, which closes the previous ROOT's unconfirmed relay item.

## 3. The owner's words

````text
You are taking over as Agent 0 (HELP_HUMAN, ROOT) of the SWB Piping Designer interface design program in the sgttomas/chirality repository. The owner is Ryan Tufts (ryan@chirality.ai).

Read the live root AGENTS.md first, then the applicable project instructions and Agent 0 instructions. Follow their actual current role and authority definitions.

SCOPE AND CURRENT SEQUENCING

This assignment is UI design research, planning, ideation, specification and mock drafts only. It does not authorize product implementation.

D-70’s baseline-report condition for handing over to redesign implementation is satisfied. The piping session has also landed a subsequent picking repair and completed a successful fresh 10,000-pipe demonstration. These facts remove the piping-session sequencing hold; they do not expand this design-only assignment into implementation. The design program retains its own readiness decisions, outstanding rulings and implementation authorization.

Do not restart historical baseline work, reopen settled measurement-policy decisions or treat the remaining physics backlog as a prerequisite for UI design.

WHERE THINGS ARE

The design preparation was merged through PR #792 at:
451c5f595e0488a0d6064d9b0f972c5e8fc1d09d

That merge already includes the piping session’s PRs #793, #794 and #795. Fetch and verify current origin/main rather than assuming this remains its tip. Record the actual starting SHA and create a fresh codex/ branch for new work. Preserve unrelated working changes.

Resolve {REPO_ROOT} with git rev-parse --show-toplevel.
Define {WORKING_ROOT} as {REPO_ROOT}/projects/chirality-piping.

Define these repository-relative paths:

{RUN} =
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM

{BASELINE} =
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE

{REPAIR} =
projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-PICKING-STABILITY

READ THESE FIRST, IN THIS ORDER

1. {RUN}/HANDOFF_STATE_WORKING.md, especially the top entry “handoff to a successor ROOT”. Treat older entries as historical where superseded.

2. {RUN}/instances/ROOT/DIRECTION_DECISION_2026-09-17.md:
   the owner’s direction, verbatim messages with hashes, §11’s fourteen mock-review decisions and §12.

3. {RUN}/instances/ROOT/DESIGN_BRIEF_V1.md:
   confirmed version V1.4.

4. projects/chirality-piping/execution/_Coordination/_DECISIONS/D-70_RULING_2026-09-17.md:
   the governing sequencing and performance-handoff ruling.

5. projects/chirality-piping/execution/_Coordination/_DECISIONS/D-71_interface_program_governed_text_decisions.md:
   the nine-item decision packet, currently PROPOSAL / AWAITING_RULING at this handoff. Verify its live status and corresponding _DECISIONS/_REGISTER.md row before acting.

6. {RUN}/WORK_GRAPH.json and {RUN}/briefs/_INDEX.md:
   sealed briefs, retained returns, SHA-256 hashes, acceptance notes and declared substitutions.

7. {BASELINE}/BASELINE_CHARACTERIZATION_REPORT.md and
   {BASELINE}/FINAL_ACCEPTANCE_AND_HANDOFF.md:
   the original incomplete characterization, published through PR #793.

8. {REPAIR}/RUNTIME_REPORT.md,
   {REPAIR}/_run_records/RUNTIME_EVIDENCE.json and
   {REPAIR}/_run_records/RUNTIME_CLOSEOUT.json:
   the successful post-repair demonstration, exact evidence custody, verification and comparison limits.

9. {RUN}/instances/ROOT/RENDERING_BRIEF_2026-09-18.md and
   projects/chirality-piping/execution/_Coordination/NOTICE_2026-09-18_RENDERING_BRIEF.md:
   the proposed rendering behaviours and separately scoped observation work.

Before selecting new work, reconcile these records with the current repository state. Preserve historical records; append or create successor records rather than rewriting sealed evidence.

ACCEPTED DESIGN DELIVERABLES

- Design system V1.1:
  {RUN}/instances/DESIGN-SYSTEM/
  Includes DESIGN_SYSTEM_V1.md with its 49-row §9 change log, tokens.json version 1.1, specimen.html and tools/.

- UX specification V1:
  {RUN}/instances/UX-SPEC/
  Includes UX_SPEC_V1.md and OPERATIONS_MAP.md with 277 rows and 29 classed gaps.

- Eighteen mock frames:
  {RUN}/instances/MOCKS/frames/index.html
  Includes two decision-aid frames for D-71 items 1 and 2.
  MOCKS_V2.md holds questions Q-15 to Q-22 and gaps G-7 to G-12.

- Research:
  {RUN}/instances/RESEARCH/
  Returns A through F. F_packet_bindings.md is the inventory behind D-71.

- Rendering brief:
  {RUN}/instances/ROOT/RENDERING_BRIEF_2026-09-18.md
  Its notice is at:
  projects/chirality-piping/execution/_Coordination/NOTICE_2026-09-18_RENDERING_BRIEF.md

PIPING SESSION’S COMPLETED HANDOFF

PR #793 published the original baseline characterization and its limitations.

PR #794 landed the bounded shared-endpoint picking repair at:
362dcffc0f66d52c58689a58f268891461db0346

PR #795 published the reviewed evidence from one fresh N10000 demonstration on that repaired product.

The fresh run:
- Completed all 243 measurement segments:
  one assignment, 200 point selections, 20 box selections, 20 filters and two orbit modes.
- Passed the formerly failing point 58 and the complete frozen point-selection population.
- Produced complete, valid evidence and met every unchanged original timing target.
- Observed zero owned pending viewport animation frames at the settled check.
- Passed cleanup and source/build/profile binding checks.

Recorded results:
- Assignment: 640.786 ms; target 2000 ms.
- Point-selection p95: 42.036 ms; target 100 ms.
- Box-selection p95: 44.296 ms; target 200 ms.
- Filter p95: 43.092 ms; target 200 ms.
- Centerline reported orbit interval p95: 8.335 ms; target 16.7 ms.
- Actual OD reported orbit interval p95: 8.335 ms; target 33.3 ms.

Reference conditions:
Apple M5 Max, 128 GiB, owner-approved internal 120 Hz display, pinned Chromium, 1440×920 browser viewport, 794×557 CSS-pixel canvas, 1588×1114 drawing buffer, DPR 2, Light appearance and Comfortable density.

Read the owning report for label populations, boundary snapshots, trace attribution and unavailable measurements. Do not infer continuous display monitoring from boundary observations.

This is one successful demonstration, not five-run qualification or acceptance of the redesigned product. The earlier successful N1000 baseline used the external LG 60 Hz display and an older product/method. Do not claim a same-profile scaling comparison or speedup.

Preserve the original D70 failures and the successful successor result with their separate attribution. Neither rewrites the other’s historical verdict.

YOUR FIRST JOB

Reconcile the completed piping handoff into this program’s working state and dependency graph. Remove any current assertion that the original baseline tranche is still blocking the redesign handoff. Preserve the previous assertions as history.

Then handle the owner’s D-71 rulings according to the packet’s on-ruling mechanism. Do not assume that silence means acceptance, and do not infer a ruling from this handoff prompt.

For rulings actually supplied:

(a) Transcribe the owner’s words verbatim with a SHA-256 hash. Append them to the D-71 packet as its Human Ruling section. Distinguish an in-session transcription from original transport-byte custody.

(b) Write:
    _DECISIONS/D-71_RULING_<date>.md
    following the form of D-70_RULING_2026-09-17.md.

(c) Update the register row to accurately reflect the ruling and point to its record. Preserve deferred or unresolved items explicitly; do not imply that all nine were decided if only some were ruled.

(d) Codify ruled registry acts and the item 7 classification as the next free DEC rows in:
    projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md §12.
    Follow D-48 §7’s on-ruling mechanism.

(e) If item 3(i) is ruled as recommended, prepare a scope-change bundle for the PRD §19.3 notice on the SCA-007 pattern, for the owner’s separate acceptance. Nothing in §19.3 changes before that acceptance.

(f) Prepare a coordination notice for the piping implementation loop, following the form of the D-70 notices. Route it through the owner unless direct delivery is explicitly authorized. Do not edit another session’s run record.

Respect the packet’s distinction between confirmations, registry acts, PRD-level acts, classification and items requiring no new act. Ask only for genuinely unresolved owner decisions; do not ask the owner to repeat recorded answers.

THEN

1. Run one revision pass from sealed briefs, applying the actual rulings:
   - Design system V1.2.
   - UX specification V1.1, reconciling §10.9’s floors to the 220 px canvas minimum and its V1 names to V1.1.
   - Regenerated frames.

   Cover MOCKS_V2 questions Q-15 to Q-22 and gaps G-7 to G-12. Check for existing owner answers first. Present remaining questions with recommendations before sealing dependent work, while continuing independent work.

2. Incorporate the original baseline report and successful post-repair report into the design program’s accepted context. Record that D-70’s performance handoff condition is satisfied, while redesigned-product performance and independent-usability obligations remain open.

3. The piping session has now read the rendering brief and its notice. No separate overlay/deformation observation run has been performed. Do not ask the owner again whether the notice was relayed.

4. Classify the rendering brief’s requested workloads into:
   - Implemented behaviour that can be observed now.
   - Existing behaviour needing a bounded harness adaptation.
   - Proposed behaviour requiring implementation before meaningful measurement.

   Do not characterize future fittings, annotation systems, proposal ghosts or deformation animation as though they already exist. Record unavailable measurements explicitly. Prepare a bounded observation brief for supported behaviour and identify future observations separately.

   This observation work blocks neither the completed baseline handoff nor redesign progress. Do not turn measurements of unimplemented features into a circular prerequisite for implementing them.

5. Prepare the redesigned-product acceptance-criteria proposal owed under D-70 effect 6:
   - Give a score-independent rationale.
   - Declare hardware, display, browser, canvas dimensions, DPR, label populations and workloads.
   - Freeze criteria prospectively.
   - Apply the same method and criteria to baseline and candidate.
   - Obtain the owner’s ruling before using new criteria for acceptance.
   - Identify which portions require later observation evidence.

   Keep main-frame work, interaction latency, available GPU trace evidence and Chromium-reported presentation distinct. Trace spans are not hardware GPU execution time; presentation feedback is not physical scanout. Do not substitute a CPU-only responsiveness gate or subtract presumed observer overhead.

6. Prepare an implementation-ready handoff when the design work is ready, without starting product implementation under this assignment. State accepted design decisions, unresolved items, operation/schema gaps, write scopes, verification requirements and the owner decision needed to authorize the implementation tranche.

IMPLEMENTATION-HANDOFF CONSTRAINTS

Preserve the completed engineering, compatibility and workflow repairs when preparing implementation briefs:

- Human and agent engineering actions use the same typed operations and Rust applier. Preserve semantic equivalence rather than creating a second mutation route for the UI or future harness.

- “The tables are the model” describes the table-first user experience. The tables and canvas must remain projections and editors of the canonical model, not independently mutable competing representations.

- Preserve Current/Historical result designation, exact solve-input basis, stale-response guards, reviewed application, undo/redo and persistence compatibility. Historical records must not acquire current-model overlays or readiness claims through presentation changes.

- Preserve the stable picking repair and its maintained regression tests. Do not alter pick tolerances or oracle expectations merely to obtain a benchmark pass.

- Retain the rendering foundation unless separately justified and authorized: persistent renderer, instancing/chunking, invalidation scheduling, resource ownership, typed model index and selection/picking.

- Map every proposed engineering control to an existing operation or an explicitly classified capability gap. A mock control does not establish an implemented operation or authorize a schema migration.

- Identify proposed semantic changes explicitly, including dimming versus hiding during isolate, persistent camera settings, new proposal/review behaviour and any new operation or saved-state requirement.

- Retain the benchmark as a reusable regression instrument, with named setup/query actions and typed entity identities. Use real pointer and keyboard stimuli when measuring interaction responsiveness.

- The redesigned canvas, geometry, labels and overlays change the workload. The current demonstration is a comparison basis, not advance acceptance of those changes.

- Pressure runtime, connector mechanics, sparse execution and export implementation remain separately scoped. Coordinate shared solver/result contracts and keep substantial builds, tests and other runtime activity clear of timed measurement.

WORKING CONVENTIONS

Delegation:
- Use the current root role definitions. Agent 0 owns alignment, authority application, integration decisions and acceptance of returns.
- Delegate bounded, independently checkable work from sealed briefs.
- Write the brief, hash it and record the launch UTC in briefs/_INDEX.md before launching.
- Record the actual executable delegation mechanism, parentage, supplied context, model/effort, tools, write scope and return requirements.
- Type 2 children never delegate.
- Use a Type 1 manager only when the assignment warrants coupled planning and integration; do not add a management layer merely because delegation is available.
- Use disjoint write scopes where possible. Serialize shared-file integration and runtime witnesses.

Model allocation:
- Preserve historical Opus/Fable model attribution exactly.
- If continuing in the original design harness, retain its established Opus allocation for objective or quantitative work and Fable allocation for design-quality and judgment work, subject to actual availability and owner direction.
- If continuing in Codex, use the owner’s standing allocations: gpt-6-astra/high for Type 1 managers and gpt-6-astra/low for new Type 2 instances, unless a task-specific owner-approved allocation applies.
- Never silently translate model names, claim an unavailable model ran or infer model configuration from a role label.

Return review:
- Review every return against its brief’s acceptance section.
- Verify citations and evidence, run relevant checks, and inspect frames visually.
- Do not repeat an expensive check merely to reproduce evidence when its bindings and result already establish the required fact.
- Retain returns with SHA-256 hashes in briefs/_INDEX.md and update WORK_GRAPH.json and HANDOFF_STATE_WORKING.md.
- Send corrections to the owning child rather than silently editing its output. Declare substitutions and any necessary transfer of ownership.
- Preserve failed evidence and distinguish a corrected result from a waived or unresolved finding.

Paths and artifacts:
- Do not add absolute /Users/ paths to authored design, governance or instruction files.
- Use {REPO_ROOT}, {WORKING_ROOT}, {RUN} or appropriate relative paths.
- Preserve existing historical evidence locators rather than rewriting them.
- Frames must be self-contained HTML with stylesheets inlined. The app’s browser pane renders local files as static snapshots.
- Use the host’s supported file-editing tool for governance Markdown. Do not assume a tool named Write exists.
- Keep one canonical copy of raw evidence with hash-bound references; avoid duplicating large evidence trees.

Validation:
- Run applicable path-anchor, claims-language and instruction-tranche checks:
  tools/validation/validate_path_anchors.py
  tools/validation/validate_claims_language.py
  tools/validation/validate_instruction_tranche_manifest.py
    --base <verified-base>
    --head <actual-candidate>
    --added-manifests-only
- Respect each tool’s actual interface and applicable scope.
- Validate the complete submitted candidate. A pre-commit --head HEAD check alone does not cover uncommitted changes.
- Run the owning loop’s required checks and affected CI entrypoints. Re-run affected checks after corrections; do not repeat unrelated product suites for evidence-only changes.
- harness.py coord-check REVIEW findings are report-only where the governing policy says so. Do not suppress real failures or turn report-only findings into invented approval gates.
- Obtain independent review where required, covering the actual candidate.

Copy:
- Product name: SWB Piping Designer.
- Wordmark: SWBPIPE.
- Use Accept, never Approve, for product controls.
- Do not use certify, seal, approve, authenticate, comply, compliant or sign-off as a control.
- “Review/signoff block” remains the registered section name.
- Use Canadian English, including “Analyze” and “colour”.
- Apply the actual D-71 rulings when they change governed wording; do not pre-empt them.
- End governed records with:
  “Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).”

Git and authority:
- The owner’s standing authorization of 2026-09-12 permits commits, pushes, PR creation/update and merges within authorized work without another per-operation or per-merge confirmation.
- Merge only after required actual-candidate CI passes and independent review has no unresolved blocking findings.
- Preserve explicit later holds and scope restrictions.
- Use merge commits so cited hashes survive.
- Use the owner’s configured authenticated Git/GitHub identity and preserve truthful agent attribution.
- Do not imply personal owner review.
- Git authority does not grant governed acceptance, implementation scope expansion, permission changes or product release.

Communicate material findings and decisions clearly. Continue routine authorized work without unnecessary escalation, including merging your PRs once the CI goes green. Return to the owner for actual reserved decisions, not because historical instructions contained an obsolete default approval step.
````

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
