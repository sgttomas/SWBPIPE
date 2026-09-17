# UI design program activation

Status: owner-directed design program. Research opened 2026-09-16; this record was written 2026-09-17. The program covers design research, planning, ideation and mock drafts only. It authorizes no product implementation, no lifecycle, acceptance or release effect, and no writes outside this run record and the owner rulings that ROOT transcribes into `../../_DECISIONS/`.

ROOT is HELP_HUMAN (Agent 0), Claude Fable 5.1 running in Claude Code, on branch `claude/chirality-piping-ui-design-a31fd2`. The owner is Ryan Tufts. Sequencing with the piping session's baseline characterization tranche is ruled in [D-70](../../_DECISIONS/D-70_RULING_2026-09-17.md).

## Owner request

In-session transcription of the opening message, not original transport bytes:

> I want to do UI design research, planning and ideation, through to mock drafts, but not implementation in the `chirality/projects/chirality-piping/` project.  The project is building an application for piping stress analysis.  Currently the UI is quite useful for the agents operating it.  I think it's far, far away from what good UI should look like in this application, and what users ought to expect from something intended for serious work.  But all I'm telling you is I don't like it.  Let's engage in Q&A to get to the heart of the matter and produce an award winning design and user experience.  First get familiar with the current state of affairs and then report back with your impressions on what you're facing and what you'd like to try doing at that point.  Adopt the role of Agent 0.  If quality isn't a concern, because results will be quantitative or objective, use `opus-5` subagents.  If quality is a concern or results are subjective or qualitative, or require extensive analysis, then use `fable-5.1` subagents.

## Basis and scope

- **Source revisions.** The research children and the opening report ran against `28ad73cc3f936872f8de1d5116c617db1290331b`, the PR #788 merge, which is the pre-#789 shell. ROOT fast-forwarded the branch to `8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`, the PR #789 merge, on 2026-09-17 and re-screened the merged workspace foundation; the opening report and the rendering boundary answer say which findings changed and which stand. Later design work consumes the merged revision.
- **Accepted context consumed.** Decomposition 0.12, SCA-009, DAG-010; D-67, D-68, D-69 and D-70; DEC-081 claims taxonomy; DEC-051, DEC-037, DEC-094; the DEL-07-06 PDU-045 and PDU-046 usability holds; the professional boundary. The governance constraints sheet under `instances/RESEARCH/C_ui_constraints.md` enumerates the constraints the design must honour or bring back to the owner as decisions.
- **In scope.** Research; the owner Q&A and a design brief; north-star concept directions; a design system including the 3D presentation language; mock drafts; a UX specification mapped to the typed interfaces and the constraints sheet; a rendering brief for the piping session's overlay and deformation observations; a proposal of acceptance criteria for the redesigned product for a later owner ruling.
- **Out of scope.** Any change under `apps/desktop/src`, `core`, the e2e harness or the benchmark harness; any product claim; any promotion of DAG, stage, lifecycle or release state; the piping session's run record, which ROOT does not edit.
- **Model rule** (owner): Opus 5 where the result is quantitative or objective; Fable 5.1 where quality matters, the result is subjective or qualitative, or extensive analysis is required.

## Orchestration

ROOT dispatched four research children through the Claude Code `Agent` tool in the same host session. Their prompts, models, timestamps, hashes and returns are indexed in `briefs/_INDEX.md`. Sealed brief files were not written before those launches; the prompts were the briefs and are retained as extracted from the transcript, with two declared path substitutions. Write targets were instruction-asserted, not sandbox-enforced. No child delegated further.

From the design brief onward, ROOT intends to bring a HELPS_HUMANS design manager into the concept, design-system and mock phases, with Fable children for qualitative work and Opus children for measurement and extraction. Each later child gets a sealed brief under `briefs/` before launch, and its return is retained under `instances/`. Type 2 does not delegate. Consequential choices return to the owner with evidence at the checkpoints below.

## Phases and human checkpoints

| Phase | Content | State |
|---|---|---|
| 0 | Familiarization: governance, design history, code, driving the app; research A to D; opening report | Complete |
| 1 | Owner Q&A on the seven opening questions; design brief | Open; checkpoint: owner accepts the brief |
| 2 | Two or three north-star concept directions, each with a storyboard of the primary workflow | Checkpoint: owner chooses a direction |
| 3 | Design system: tokens, type, colour with dark mode, iconography, density, component states; 3D presentation language covering real outside diameter, fittings, supports, result overlays, deformed shape and annotations | Working |
| 4 | Mock drafts of the workflow states enumerated in the design brief | Checkpoint: owner reviews the mocks |
| 5 | UX specification mapped to typed interfaces and the constraints sheet; rendering brief to the piping session; proposed acceptance criteria for the redesigned product | Checkpoint: owner rules on the acceptance proposal |

## Evidence policy

Returns are retained verbatim except for declared path substitutions, and the hashes of the supplied bytes are recorded beside them. Research outputs are inputs to design, not accepted findings, not decomposition truth and not product claims. Where a research file names a constraint, decision or file, the governed source remains authoritative. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
