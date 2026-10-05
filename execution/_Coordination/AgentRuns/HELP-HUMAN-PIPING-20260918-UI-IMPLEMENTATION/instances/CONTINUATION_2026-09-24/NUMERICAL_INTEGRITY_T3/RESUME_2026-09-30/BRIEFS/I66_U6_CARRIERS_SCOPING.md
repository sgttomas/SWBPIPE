# I66: U6 carriers and standing, scoping grant (records only)

TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is the return path; you do not delegate. **This grant is records only:** you write a plan, not code.

## Purpose

The first public F2a publication is one package: producer, readers and carriers together (FIRST_PUBLICATION_PATH §3; S-G1 is atomic with F2a). The producer (U1–U3) and the readers are under way or accepted. U6 makes the carriers accept the successor identity and carry its receipt, with standing derived only from the verified receipt. Standing stays `needs_recompute` until U7 switches eligibility on.

**Your job:** an accurate, bounded plan for U6, and for the reader-round items U7 needs, so that both can be granted without becoming the critical path. U4 is the critical path; its grants G4–G6 remain.

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- `R/FIRST_PUBLICATION_PATH.md` §3, and the source map it links: `R/I30/f2a_checkpoint0_01/SOURCE_MAP.md`.
- I61's step-4 plan `R/I61/step4_plan_01/PLAN.md` §2: U6, U7 and their conditions.
- The contract: C1 (`R/I32/f2a_wire_c1/WIRE_CONTRACT.md`), especially C1:162's successor branches in the AnalysisRun and stress-neutral carrier schemas; C2 and C3 as located by I61's plan §1.
- **The design references the plan names:**
  - D1 §5 item 3: standing only from the verified receipt;
  - D2 §4.9.7: `derive_document`'s receipt-copy list.
  
  Locate them through the plan and the source map.
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - "The F2a readers accepted and fanned in";
  - RV78's carried-artefact review and its routed S1, N1, N2 and N3 (search "RV78 on the carried artefacts");
  - "U3 grant 1 verified; R-1, R-2 and R-3 ruled" (the public carrier interface, Proposal A);
  - the D36-tracked items listed in "RV82 on U1 grant 2: PASS; U1 and U2 merged into NUM";
  - the D38 move to wider F2a ("U1 grant 2: I61's stop resolved").
- The current code at NUM: the result envelope and successor branch in `P/schemas`, `P/core/analysis_runs`, `P/core/reporting/result_export`, and `P/apps/desktop/src/features/results`. Also the derivative and export carriers the source map names.

## What the plan must contain (`NUM/R/I66/u6_scoping_01/PLAN.md`)

1. **The carrier inventory.** For every carrier that holds, copies, derives or exports a results document, give:
   - file and line;
   - its schema;
   - whether it accepts a successor identity today, which is expected to be "no, fails closed";
   - what U6 must change.
   
   Cover AnalysisRun, stress-neutral, `derive_document`, the desktop result admission and standing, and any export.
2. **Ownership and collisions.** For each file, give the owning track in the work graph (`P/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`): T3, or another track such as T6, which owns export. State where U6 would write into another track's files, and what coordination or reservation that needs. Run the fresh collision check that FIRST_PUBLICATION_PATH §3 requires for any new names.
3. **Standing.** Exactly where standing is computed today, and how U6 makes it depend only on the verified receipt, staying `needs_recompute` until U7.
4. **The receipt's survival.** Which document operations must copy, preserve or invalidate the receipt (D2 §4.9.7; "later receipt operations that alter covered rows or metadata invalidate the earlier certificate"), and where each lives.
5. **The reader-round items U7 needs.** List each with its owner reader and cost:
   - RV79-N1, the independent D37 expected table, which is a U7 condition;
   - RV80-N2;
   - RV78's N1 and N2;
   - U1's F5;
   - anything else D36-tracked that U7's conditions require.
   
   Say which can be bundled with U6.
6. **The units, write fences, order, costs and review plan** for U6 and the reader round. Make the first unit an end-to-end slice: the milestone successor carried through one real carrier and back out, verified.
7. **Decisions needed** from ROOT, or from the owner where a choice is reserved, each with a proposed answer and its citation.

## Host and method

- **Records only.** Git reads only, with `GIT_OPTIONAL_LOCKS=0`. Stdlib Python for any counting, under `NUM/R/I66/u6_scoping_01/_run_records/`. Scratch goes in `WT/scratch/i66_u6_scoping_01/`, never the system temp directory.
- **Never:** Cargo, npm, pytest runs, installs, Git writes, or native, solver or DEC-025 jobs.
- **The memory guard** must be running. Other TASKs are working: I61 in `WT/f2a-facade`, I65 on U4 G4 records, and RV85. Don't touch their files.

## Output and return

`PLAN.md`, `RETURN.md` and `SHA256SUMS` in `NUM/R/I66/u6_scoping_01/`, with placeholder paths only. **Budget: 3 hours.** End with a concise status: the plan's headline, its units and costs, the decisions needed, and the report's sha256.
