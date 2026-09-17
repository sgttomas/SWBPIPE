Here is where I've landed after reading the governance, the design history, the code, and driving the app myself in the browser pane (it's running at `http://localhost:5199` if you want to look alongside me).

## What I'm facing

**The UI is an honest map of the work breakdown structure, not of an engineer's day.** App.tsx composes roughly fifty feature panels, one per deliverable, and each panel was built to make its evidence visible to tests and receipts. The result is an app where the audit trail leaks into the product. On the Results, Analyze, and Issues surfaces the primary content is dozens of lines like `path=$.__route_key__schema_version; classification=unknown; action=warning_only; reason=REDISTRIBUTION_STATUS_UNKNOWN`. The Load Cases dock shows queued edits as `op:load-manager-combination:C-OPER-ALT-term-0-delete; before=load:L-100 x 1; after=not_present; unit_validation=not_required_dimensionless`. The Units card reads `model=angle=rad,force=N,length=m...; conversion=false`. Every card ends with the professional-judgment disclaimer, which also sits in the footer, the status bar, and the inspector. Repetition has turned a serious boundary into wallpaper.

**The model is still a postage stamp whenever real work happens.** All work surfaces (Loads, Analyze, Results, Review) open in a bottom dock, and the dock takes the canvas away.

| Window | Dock closed | Dock open (any work surface) |
|---|---|---|
| 1280x800 | ~940 x 510 px | ~1000 x 270 px, about a quarter of the window |
| 1024x768 | ~790 x 500 px | ~790 x 250 px |

Inside that strip, 21 entity chips are truncated to `C-1…`, `N-1…`, `P-1…`, pipes are fixed-radius dark lines, supports are colored cones, and there are only Front/Top/Iso presets. It doesn't read as an engineering instrument.

**There are still five or six command systems.** Menu bar (File/Edit/View/Insert/Analyze), ribbon (Model/Loads/Analyze/Results/Rules/Report), command bar (Node/Pipe/Support/Component/Load/Select), the Toolkit palette, Review, status-bar openers, and a title bar with five project buttons that say "local" four times. "Analyze" appears twice; a node can be created three ways. The June 2026 verdict was "three competing navigations"; the redesign that followed replaced them with a different set.

**The language is governance-speak.** The demo model's entities are named "Invented semi-rigid valve marker" and "Preview one-way terminal stop". The status bar reads "Mechanics ready for preview diagnostics · Rule check Inputs needed · Professional Review required". The node form explains "Click within 4 CSS pixels on the visible 3D canvas to capture on global XZ at Y=0m". Rule packs are loaded by pasting JSON. This is implementation narrating itself.

**What is genuinely excellent and must survive any redesign:** typed operation intents with atomic batches, one undo checkpoint per apply, and exact model-hash equivalence between human and agent edits (agent parity by construction); strict unit preservation with entered/SI/US display; Current vs Historical result truthfulness; provenance on every entity and explicit missing data; shared selection identity across tree, canvas, and inspector; the searchable capability catalogue with disabled reasons; and a large, real test base. The engine is not the problem. The instrument panel is.

## Where prior design work stands

- **June 2026 redesign** ([plan](projects/chirality-piping/plans/PLAN_2026-06-18_workspace_redesign_c5_7.md)) diagnosed "wall of strings, three navigations, postage stamp" and shipped the ribbon, command bar, status bar, and drawers. The shape changed; the substance recurred.
- **September 5 product-design report** ([report](projects/chirality-piping/plans/REPORT_2026-09-05_piping_modeling_and_analysis_product_design.md)) is good thinking: one engineering task as the design spine, docked inspector, single lower drawer, microcopy discipline, linked table/graphics selection. It explicitly says the owner must decide workspace preference, window sizes, and measurable usability targets. Those decisions are still open, which is why the DEL-07-06 usability holds have sat for months.
- **September 13 interaction contract and offline prototype** ([contract](projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/UI/INTERACTION_CONTRACT_V1.md), [prototype](projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/UI/successor-v3/prototype/index.html)) moved in the right direction (dominant canvas, persistent tool groups, task rail) and was witnessed for behavior by agents. It was never accepted as your visual direction, and its lower dock still shows JSON as UI.
- **September 15** a prior Agent 0 prepared the [production UI foundation tranche](projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/PRODUCTION_UI_FOUNDATION_CANDIDATE.md) to implement that prototype. Your message effectively puts a hold on that: the design bar is wrong, so implementing it would be premature. I'll treat it as held unless you say otherwise.

The pattern across all three: design has been done by agents optimizing for verifiable behavior. Nobody has yet supplied a point of view, taste, and observation of a real engineer. That is the gap this effort should fill.

## What I'd like to try

A design program rather than a tranche, with no implementation:

1. **Brief through Q&A** (this conversation), producing a one-page design brief you sign.
2. **Two or three divergent north-star concepts**, each a narrative plus one high-fidelity key screen built around the spine task: route a short system, give it engineering meaning, analyze, change a support, compare. Candidates: "the drafting table" (model-as-hero with a command line), "the spreadsheet that draws" (grid-first input in the CAEPIPE and AutoPIPE tradition with live 3D), and "the reviewed calculation" (a linear engineering document with embedded model, tables, and agents as reviewers). You pick or blend.
3. **A design system**: type scale, engineering palette where color never implies approval, a domain icon set, density modes, panel grammar, and a microcopy standard that retires machine strings to an audit drawer agents can still read.
4. **Mock drafts** for eight states (empty, routing, selected pipe, support, loads, results, issue and repair, agent review), with the results experience given the most care since that is where credibility is won.
5. **A UX spec** mapping every mock to the existing typed interfaces and to a governance constraints sheet, so the eventual implementation tranche is feasible and legal under CONTRACT.
6. **Validation** by task walkthrough with you, and ideally one or two stress engineers, before any code.

I'd stand up a HELPS_HUMANS design manager for steps 2 to 5 once the brief exists, using Fable subagents for all qualitative design work. Four background investigations are already running to feed this: a heuristic UX audit of the current app, a quantitative UI inventory, a governance constraints sheet, and domain research on what stress engineers expect from CAESAR II, AutoPIPE, ROHR2 and peers. I'll share those when they land.

To get to the heart of it, my opening questions:

1. **What was the session** that made you say "I don't like it"? Walk me through what you were trying to do and where it fell apart.
2. **Who is the one user** whose delight matters most: you running a real analysis, a stress engineer at a firm deciding whether to trust it, or a layout designer routing pipe?
3. **Model-first or table-first?** Should that person live in the 3D canvas, or in a grid with 3D as verification?
4. **What is the bar?** Name two or three products, in any domain, that feel like what this should feel like. And which piping tools do you respect versus want to beat?
5. **Where does the agent live in the UI?** A visible collaborator with its own panel, or an invisible actor whose proposals arrive as ordinary reviewed changes?
6. **Governance on the surface:** how much of the boundary, provenance, and hash apparatus must stay visible at all times versus on demand? Is "Technical preview" a permanent footer?
7. **Platform:** macOS native window first? Minimum size you care about? Dark mode? Keyboard-first?

Housekeeping: I installed the desktop dependencies in this worktree, borrowed the prebuilt WASM engine from your main checkout so the shell runs, and kicked off a proper WASM build so solving works in the browser next turn. Everything I created is gitignored and the working tree is clean.

