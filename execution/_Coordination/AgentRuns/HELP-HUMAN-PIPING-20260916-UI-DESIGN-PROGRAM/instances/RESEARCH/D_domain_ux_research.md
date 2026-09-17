# D — Domain UX research: what piping stress engineers expect, and what an award-winning OpenPipeStress could be

Date: 2026-09-16. Audience: lead designer, to sharpen questions for the product owner. Status: research memo, not a spec. No repository files were read or modified for this memo.

Sourcing legend: **[S]** = stated by an official doc, vendor page, or a named practitioner post (citation number in brackets). **[V]** = vendor or distributor marketing; treat claims as positioning. **[I]** = my inference or synthesis. Quotations are under 15 words each. No code text, allowable tables, or vendor data tables are reproduced.

Coverage caveats: Reddit blocks the crawler used here, so r/PipingStress and r/MechanicalEngineering could not be read; practitioner sentiment is drawn from Eng-Tips, LinkedIn, Bentley's ideas portal, and independent blogs. Hexagon's documentation portal is JavaScript-rendered and returned 404 to direct fetches; where I cite it, the content came from search snippets plus secondary sources and is marked accordingly.

---

## 1. The incumbents and their interaction models

One market fact first: CAESAR II became "Octave Aspect Pipe Stress" in 2026 when Hexagon spun its ALI software into Octave; models and workflows are stated to be unchanged [S: 1, 2, 3]. The incumbent is mid-rename, a rare window for a challenger to define "modern." [I]

| Tool | Input model | Load cases | Results review | Code-check reporting | The "classic" screen |
|---|---|---|---|---|---|
| CAESAR II / Aspect Pipe Stress (Octave) | One spreadsheet form per element: From/To node, DX/DY/DZ, section, temps/pressures; checkboxes open restraint, displacement, hanger, bend, and SIF panes [S: 4, 6, 7]. Values propagate down the element list and show red [S: 5]. Length-only entry, no coordinates or list paste [S: 48]. Numeric node numbers, default increment 10 [S: 27, 12] | Static Load Case Editor: engineer writes each case explicitly (W+T1+P1 etc.), tags stress type OPE/SUS/EXP/OCC, builds algebraic or scalar combination cases [S: 9, 17, 18] | Static Output Processor: pick load cases × report types; color-coded stress plots and animated displacements [S: 4, 10, 11] | Displacements, Restraints, Restraint Summary, element forces, Stresses, Stress Summary, Code Compliance, hanger table; Word export, Data Export Wizard, stress isos through ISOGEN [S: 10, 11, 21] | Spreadsheet form beside a 3D viewport; modal hops between Input, Error Check, Load Cases, and Output [S: 4, 6; I] |
| AutoPIPE (Bentley) | "Object-based GUI" with CAD-style point-and-click and undo/redo; input grids that "behave like Microsoft Excel" with two-way sync to the plot; Excel import; alphanumeric point names; 99 undo levels [S: 24, 25, 27] | Code combinations generated automatically in a Tools > Combinations grid; users add non-code combos [S: 29, 28] | Click the model for pop-up stresses/deflections/forces; results grid with filtering, sorting, grouping, conditional formatting; up to 1,000 combos [S: 25] | Stress isometrics to DXF/DWG/DGN; STAAD.Pro link; ML support optimizer [S: 24] | Viewport with docked input/results grids at the bottom; grids can get lost off-screen on multi-monitor setups [S: 30, 31] |
| ROHR2 (SIGMA) | ROHR2win graphical pre/post-processor; all input through it, with import/export formats [S: 32, 34] | Not detailed on public pages | Results "tabulated and graphically"; "all result details easy to reach by mouse click" [S: 32] | Template-driven calculation report; ROHR2iso draws isometrics from the same model so edits update the iso [S: 32, 33] | Windows MDI app; graphics plus tables |
| CAEPIPE (SST) | Text "Layout window", one row per element (node, type, DX/DY/DZ, data), synchronized with an OpenGL Graphics window [S: 35, 36] | Selected in results; not a builder in the CAESAR sense | Sorted stresses, code compliance, support loads, displacements, color-coded stress, animated deflection; claim that users "never see the hourglass" [S: 35, 36] | Standard tables; checkSTRESS lets designers pre-check lines inside CAD before handing to stress [S: 37] | Two-window pair: text layout beside 3D |
| PASS/START-PROF | Object model ("all pipes, fittings, and nodes are objects"); draw like a 3D modeler; on-the-fly unit conversion; project tree [V: 39, 38] | Operation Mode Editor: user states operating modes; load cases are generated from hidden code templates [V: 18, 39] | Interactive reports, overload cells highlighted red, warnings window [V: 39] | Export to Excel/Word/HTML [V: 39] | 3D canvas plus property panels |
| TRIFLEX (PipingSolutions) | Dialog-driven data entry, real-time geometry checking, up to four open models [V: 41] | "Unlimited" cases with automatic sub-cases [V: 41] | Gradient color output, deflection plots [V: 41] | PDF/Excel reports across 20+ codes [V: 41] | Oldest lineage: began as a preprocessor over punched-card programs [S: 42] |
| CloudCalc (browser) | Graphical or text input "via mouse, toolbars, hot keys, or touch commands"; unlimited undo; PCF and CAESAR II import; monthly pricing [V: 43] | Automated hanger design; B31.1/B31.3 | Animated modes, formatted reports | Cloud, mobile access [V: 43, 44] | Web page |
| Pipe Stress Infinity (open source) | Python scripts or REPL, no GUI; B31.1 [S: 45] | Scripted | Text | Text | It already owns openpipestress.com — a naming collision the product owner should know about [S: 45] |

Two observations. The market has split into two input philosophies: spreadsheet-of-elements (CAESAR II, CAEPIPE) versus object/CAD model with grids as a view (AutoPIPE, START-PROF); reviews call CAD-style tools faster to learn and CAESAR's spreadsheet what veterans prefer for "precision and structure" [S: 51]. And every mature tool converges on the same outputs: displacements, restraint loads, code stress table, stress iso. That set is the credibility floor. [I]

## 2. Practitioner sentiment

**What they praise.** CAESAR's element input is repeatedly called good: "Piping input is very good" and results review "above average" (RVAmeche, Eng-Tips) [S: 46]; an older poster called it "much more user friendly" than AutoPIPE [S: 47]. AutoPIPE is credited for point-and-click editing, alphanumeric points, and deep undo [S: 27]. German calculation firms reportedly pick ROHR2 when the client allows, citing support and ease of change [S: 46]. CAEPIPE's pitch is responsiveness [S: 35].

**What they complain about.**
- *Geometry entry.* Lengths only; "you can't use coordinates or input through a list," which makes survey data tedious [S: 48].
- *Node numbering.* Numeric-only IDs; renumbering is block-limited [S: 27, 12]. Worse, propagation follows element-list order, not connectivity, so PCF-imported models with out-of-sequence nodes silently inherit wrong values [S: 5]: the category's clearest documented hidden state. [I]
- *Undo.* CAESAR loses undo on leaving input; competitors advertise 99 or unlimited levels, which tells you engineers notice [S: 27, 43].
- *Load cases.* The manual builder is powerful but error-prone; even a hostile vendor comparison frames it as "manual transmission" versus automatic [V: 18]. Independent commentary lands on the trade-off: automation reduces error but "provides less manual control" [S: 28].
- *Restraint semantics.* Threads recur on +Y meaning, gap sign, friction on gapped guides, and gap-caused non-convergence [S: 49, 50]. The vocabulary is standard; the mental model is where beginners fail. [I]
- *Units and configuration.* CAESAR keeps internal versus user units and per-machine setup files; guidance is to archive the setup file because "identical reruns" are not otherwise guaranteed [S: 15, 16]. START-PROF's on-the-fly unit switching is marketed precisely against this [V: 39].
- *Error messages.* The error checker is a hard gate with errors/warnings/notes; some messages are cryptic enough to need blog explainers [S: 8, 23].
- *Results navigation and grids.* AutoPIPE users objected when v24 removed per-column selection from the results grid; grid windows vanish on multi-monitor setups (a 10-vote idea) [S: 30, 31].
- *Reports.* Custom templates exist and Word export is the pipeline [S: 11]; designers read reports for support loads, movements, and nozzle loads, cross-checked against the 3D model [S: 22]. Wish-list items: auto-filled iso title blocks, model-level notes [S: 31].
- *Learning curve and price.* CAESAR is widely described as the costliest and steepest [S: 51, 52; low-trust 74]; one practitioner criticizes "cranking out yearly updates" and pricing changes and wants a free viewer [S: 46]. Lock-in is client-driven: only CAESAR II specified in 15 years of Middle East projects [S: 46].

## 3. Canonical workflows: credibility signals versus legacy baggage

| Convention | Why it exists | Verdict [I unless noted] |
|---|---|---|
| Node numbers | Nodes are the shared address space between stress, layout, and support design; designers "mark node numbers on stress isometrics" and reports are keyed by node [S: 22, 21] | **Must-have as identity.** Keep stable, human-editable IDs (alphanumeric allowed). Legacy baggage: numeric-only, manual increments, order-dependent propagation. |
| From/To element with DX/DY/DZ | Universal spreadsheet fluency; every engineer can read it [S: 4, 36] | **Must-have as a view.** Add absolute coordinates, paste-from-Excel, and PCF/CAD import as first-class alternatives [S: 48, 24]. |
| Restraint vocabulary (anchor, +Y, guide, limit stop, gap, friction, connecting node) | Standard across tools and codes; consumed downstream [S: 13, 75] | **Must-have vocabulary; must-fix semantics.** Render meaning graphically (direction, gap, friction glyph) and warn on classic mistakes [S: 49, 50]. |
| Load case builder (OPE/SUS/EXP/OCC, combinations) | Engineers audit which loads went into which check [S: 17, 9] | **Generate, show, allow override.** AutoPIPE's visible combination grid is the right shape; START-PROF's hidden templates buy speed at the cost of auditability [S: 29; V: 18]. |
| Code stress summary and code compliance table | The deliverable reviewers sign against [S: 21, 22] | **Must-have.** For a code-neutral app, the table shape is generic: node, case, computed stress, allowable, ratio, plus the rule-pack rule ID and version so the reviewer sees provenance. |
| Restraint load summary, displacement table | Consumed by support, structural, and equipment teams [S: 21, 22] | **Must-have**, with per-case and enveloped views. |
| Stress isometric | Contractual artifact in EPC; produced by ISOGEN, DXF/DWG, or ROHR2iso from the same model [S: 10, 24, 33] | **Credibility signal but a generator, not an interaction model.** Design the markup (nodes, supports, movements, loads) before the drawing. |
| Hanger design | Two-pass ritual: rigid support to find load, then spring selection against vendor tables, output as hanger table [S: 19, 20] | **Expected in mature tools; scope-sensitive here** because vendor tables are private data. Treat as a plugin/rule-pack surface. |
| Error check before run | Model validation as an explicit gate [S: 8] | **Keep the gate, lose the mode.** A live issues panel (see 4) preserves the ritual without the modal screen. |
| Per-machine config affecting results | Historical | **Baggage.** Everything that changes a number belongs in the project and is hashed. |

## 4. Adjacent patterns worth borrowing

- **List linked to graphics, both ways.** Onshape cross-highlights features, parts, and faces between list and viewport, and its feature list filters with commands like `:errors` [S: 53, 54]. AutoPIPE's grid-to-plot sync and CAEPIPE's layout cursor are the domain's own versions [S: 27, 36]. The single most important pattern for a spreadsheet-and-3D app. [I]
- **Immutable versions and compare.** Onshape versions are immutable; Compare shows base and target in blue/red with a feature-list diff using ≠, <, > markers [S: 55, 56]. GitHub's compare view uses base/compare with a merge-base three-dot diff [S: 67]. Comparing two analysis runs (geometry, cases, rule pack, results) is a natural fit for a local-first, evidence-oriented tool. [I]
- **Direct distance entry and routing compass.** Plant 3D constrains routing to a plane with angle ticks and lets you "enter a distance (pipe length)" for the next point [S: 57]; Fusion lets you type a length, Tab, then an angle while drawing [S: 58]. This is DX/DY/DZ entry with the 3D view as the form. [I]
- **Command line and palette.** Rhino's prompt autocompletes with fuzzy match and exposes command options as clickable words [S: 59]; Blender's F3 search and N sidebar keep the viewport as hero [S: 62]; Cmd+K palettes in Linear, Figma, and Raycast let power users skip menus [S: 63, 64]. Linear's reputation rests on perceived speed and an opinionated, uncluttered UI [S: 65].
- **Few verbs.** SpaceClaim claims four tools cover most modeling [S: 61]. A stress model has roughly as few verbs: route, branch, restrain, load, run, review. [I]
- **Issues panel.** VS Code groups problems by file, shows a status-bar count, cycles with a key, and links each entry to its location with quick fixes [S: 66]. This maps directly onto errors/warnings/notes from the error checker and START-PROF's warnings window [S: 8; V: 39].
- **Result inspection.** ANSYS keeps results as tree objects with probes, min/max, legend control, and a tabular-data window [S: 68]; SimScale has inspect-point and legend range control [S: 69]; Fusion's legend carries the load-case selector [S: 70]. AutoPIPE's conditional-formatting grid is closer to what stress engineers already do in Excel than any contour plot. [S: 25; I]
- **Explicit affordances.** Shapr3D's Apple Design Award write-up credits visible handles: "Explicit is always better than implicit," and "Easy should be easy. Hard must not be impossible." [S: 71]. The domain analog is showing propagation, gaps, and restraint direction rather than encoding them in codes. [I]

## 5. What "award-winning" would mean here

1. **One spine.** A single model with one selection state reflected in grid, viewport, issues, and results. Exemplars: Onshape cross-highlighting [S: 54]; ROHR2iso's "same model" isometrics [S: 33].
2. **Model as hero, calm density.** The viewport and the element grid are the page; everything else is a drawer. Exemplars: Shapr3D [S: 71], Linear [S: 65].
3. **Spreadsheet-and-3D duality.** Type DX/DY/DZ or draw with a compass and direct distance; both write the same element. Exemplars: AutoPIPE grids [S: 25], Plant 3D compass [S: 57], CAESAR's element form as the standard of fluency [S: 4].
4. **Instant, honest feedback.** Geometry checks while typing; issues panel always live; no modal error-check screen. Exemplars: VS Code Problems [S: 66], CAEPIPE's no-hourglass stance [S: 35], TRIFLEX's real-time geometry checking [V: 41].
5. **Honest state and provenance.** Immutable runs that record geometry, cases, units, rule-pack identity and hash; compare any two. Exemplars: Onshape versions and compare [S: 55, 56], GitHub compare [S: 67]. The negative exemplar is CAESAR's per-machine setup file [S: 15, 16].
6. **Keyboard fluency.** A palette plus a command prompt with autocomplete and clickable options; every grid cell reachable without the mouse. Exemplars: Rhino [S: 59], Blender [S: 62], Raycast/Linear [S: 63, 64].
7. **Beautiful, legible results.** Tables first (sortable, filterable, conditionally formatted, units in headers), then color on the model with probes and a real legend, then animation, then a generated stress iso. Exemplars: ANSYS probes and tabular data [S: 68], AutoPIPE results grid [S: 25].
8. **Explicit over implicit.** Restraints drawn as what they do; propagated values visibly marked, as CAESAR already does in red [S: 5, 71].

## 6. Risks and anti-patterns specific to this domain

- **Dashboard decoration.** Stephen Few's pitfalls (meaningless variety, misuse of color, clutter, inadequate context) apply directly to "results overview" screens [S: 72]. A stress engineer wants the max ratio, where, in which case, and why. [I]
- **Hiding the tables.** Engineers and their downstream consumers live in tables [S: 21, 22]; AutoPIPE's grid regression drew complaints [S: 30]. Any redesign that demotes the grid to a secondary tab will read as a toy. [I]
- **Color as approval.** Traffic-light ratios fail for deuteranopia [S: 73] and, worse, imply acceptance the software cannot give. In a code-neutral app the allowable comes from the user's private rule pack; the UI must show the ratio, the rule ID, and the pack version, never just green. [I]
- **Obscuring units.** The category's own history is the warning [S: 15, 16]. Units belong in every column header and every probe, with conversion explicit and logged. [I]
- **Automation without audit.** "You can't do a mistake with load cases" is a vendor promise [V: 18]; the independent view is that automation trades away control [S: 28]. Generate, then show the generated cases as editable, attributable rows. [I]
- **Breaking node addressability.** Object models that hide node IDs break the designer's iso markup and the client's review [S: 22]. [I]
- **Agents or AI as the hero.** The CloudCalc CEO frames engineers as "tech-wary" and AI as assistance around the model [S: 44, snippet only]; Bentley presents its ML optimizer as a feature, not a persona [S: 24]. The model, the run, and the evidence stay central; any assistant proposes and the human accepts. [I]
- **Code-neutral credibility gap.** Without bundled code names, the app must earn trust through visible rule-pack identity, versioning, and reproducible runs, or it will look like a hobby solver next to Aspect Pipe Stress. [I]
- **Name collision.** openpipestress.com is already Pipe Stress Infinity's site [S: 45].

## 7. Sharper questions for the product owner

1. Is the element grid or the 3D view the primary authoring surface, and which must feel native to a CAESAR veteran on day one?
2. Node ID policy: stable alphanumeric labels with auto-increment, and what happens on insert, split, and PCF import?
3. Which output tables are contractual deliverables in target projects, and in what order do reviewers read them?
4. Load cases: generated from a rule pack and shown as editable rows, or authored by hand? Who owns provenance on override?
5. What does a code-neutral "code compliance" table show when the rule pack supplies names, and how is the pack's version surfaced?
6. Is a generated stress isometric in scope for v1, and if so which markup (nodes, supports, movements, loads) is mandatory?
7. Hanger design: in scope with user-supplied vendor tables, or out of scope?
8. Units: project-level with explicit conversion history, or per-view display units?
9. Run comparison: is "compare two runs" a v1 feature or a later evidence feature?
10. Where do agents or automation appear, if at all, and how are their proposals distinguished from user edits?

---

## Citations

Official docs and vendor pages
1. Octave / Hexagon — Aspect Pipe Stress product page (formerly CAESAR II): https://hexagon.com/products/caesar-ii (fetch blocked; title and positioning from search)
2. What Is Piping — What is Octave Aspect Pipe Stress? https://whatispiping.com/what-is-octave-aspect-pipe-stress/
3. Cortex Software — Hexagon to Octave, what customers need to know: https://www.cortexsoftware.com.au/blog/hexagons-software-spin-off-to-octave
4. wermac.org — CAESAR II overview (reproduces COADE-era positioning): https://www.wermac.org/misc/caesar2.html
5. ECE Design — CAESAR II piping input screen and propagated values: https://www.ecedesign.com/2015/03/13/caesar-ii-piping-input-screen-and-propagated-values/
6. Paul Howard (LinkedIn) — CAESAR II Classic Piping Input: https://www.linkedin.com/pulse/caesar-ii-classic-piping-input-paul-howard
7. Hexagon docs — Classic Piping Input Dialog (v14; JS-rendered, not fetched): https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-14/908054
8. Hexagon docs — Error Check (v15; snippet only): https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/15/343412
9. Hexagon docs — Editing Multiple Load Cases / Static Load Case Editor (snippet only): https://docs.hexagonppm.com/r/ru-RU/CAESAR-II-Users-Guide/Version-12/731599
10. Hexagon docs — CAESAR II 2016 changes (Static Output Processor report options, ISOGEN): https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-14/811442
11. Hexagon docs — Output Report Options: https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-14/425541
12. Hexagon docs — Node Numbers: https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-13/343300
13. Hexagon docs — Restraints: https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-14/335355
14. Hexagon docs — Change Model Units: https://docs.hexagonppm.com/r/en-US/CAESAR-II-Users-Guide/Version-12/908339
15. Cortex Software blog — CAESAR II working with units: https://cortexsoftwareblog.wordpress.com/2014/12/17/caesar-ii-working-with-units-02-changing-units-of-input-file/
16. ECE Design — CAESAR II network configuration for multiple users (setup-file archiving advice): https://www.ecedesign.com/2015/05/11/caesar-ii-network-configuration-for-multiple-users/
17. What Is Piping — Load cases for pipe stress analysis: https://whatispiping.com/load-cases/
18. What Is Piping — Load Case Editor vs Operation Mode Editor (START-PROF-affiliated author): https://whatispiping.com/load-case-editor-vs-operation-mode-editor/
19. What Is Piping — Spring hanger selection in CAESAR II: https://whatispiping.com/spring-hanger-selection/
20. Little P.Eng — CAESAR II hanger design control and options: https://www.littlepeng.com/single-post/2020/06/07/247-caesar-ii-hanger-design-control-and-options
21. The Piping Engineering World — Pipe stress analysis report contents: https://www.pipingengineer.org/pipe-stress-analysis-report/
22. Yasin Mohamed (LinkedIn) — How to read pipe stress reports for piping designers: https://www.linkedin.com/pulse/how-read-pipe-stress-reports-piping-designers-yasin-mohamed
23. What Is Piping — CAESAR II error "Material is outside the temperature range": https://whatispiping.com/caesar-ii-error-material-outside-temperature-range/
24. Bentley — AutoPIPE product page: https://www.bentley.com/software/autopipe/
25. Bentley — AutoPIPE product data sheet (snippet only): https://www.bentley.com/wp-content/uploads/pds-autopipe-ltr-en-lr.pdf
26. Bentley community wiki — AutoPIPE vs CAESAR user interface (JS-rendered, not fetched): https://bentleysystems.service-now.com/community?id=kb_article_view&sysparm_article=KB0024606
27. What Is Piping — AutoPIPE vs CAESAR II (reproduces Bentley comparison): https://whatispiping.com/autopipe-vs-caesar-ii/
28. EPCLand — AutoPIPE vs CAESAR II detailed differences: https://epcland.com/autopipe-vs-caesar-ii/
29. Bentley KB — Combination load cases, Tools > Combinations (snippet only): https://bentleysystems.service-now.com/community?id=kb_article_view&sysparm_article=KB0025345
30. Bentley KB — Input/Results grid missing, reset location; v24 grid complaint: https://bentleysystems.service-now.com/community?id=kb_article_view&sysparm_article=KB0038301
31. AutoPIPE Ideas Portal (user feature requests): https://autopipe.ideas.aha.io/ideas?category=7197792020196714774
32. SIGMA — ROHR2 input, results and documentation: https://www.rohr2.com/en/rohr2-working.html
33. SIGMA — ROHR2iso: https://www.rohr2.com/en/rohr2iso.html
34. SIGMA — ROHR2 overview: https://rohr2.com/en/rohr2.html
35. SST Systems — CAEPIPE graphics: https://www.sstcae.com/tech-feature/caepipe-graphics
36. SST Systems — CAEPIPE 3D+ modeling and results review tutorial: https://www.sstcae.com/caepipe-3d-plus/tutorials/tutorial-for-modeling-and-results-review-problem-1
37. SST Systems — Why checkSTRESS: https://www.sstcae.com/whycheckstress
38. PASS — START-PROF capabilities (SlideShare; product page returned 403): https://www.slideshare.net/slideshow/passstartprof-capabilities-for-pipe-stress-analysis-of-power-and-process-piping-systems/236834913 and https://passuite.com/start
39. What Is Piping — CAESAR II vs START-PROF (author heads START-PROF development): https://whatispiping.com/caesar-ii-vs-start-prof-piping-stress-analysis-software-differences/
40. Fluids & Co (distributor) — 10 reasons to replace CAESAR II with START-PROF: https://fluidsandco.com/2025/04/26/10-reasons-to-replace-your-actual-stress-analysis-tool-caesar-ii-with-pass-start-prof/
41. PipingSolutions — TRIFLEX features: https://pipingsolutions.com/features/
42. Dam Vo (LinkedIn) — A history story for pipe stress analysis programs: https://www.linkedin.com/pulse/history-story-pipe-stress-analysis-programs-dam-vo
43. CloudCalc — product site: https://www.cloudcalc.com/
44. Piping Technology & Products — CloudCalc CEO interview (403 on fetch; snippet only): https://pipingtech.com/ai-cloud-the-future-of-pipe-stress-cloudcalc-ceo-isnt-afraid-of-tech-wary-engineers/
45. Pipe Stress Infinity — https://openpipestress.com/ and https://github.com/denisgomes/psi

Practitioner forums and independent commentary
46. Eng-Tips — Piping Stress Analysis Software: https://www.eng-tips.com/threads/piping-stress-analysis-software.462376/
47. Eng-Tips — Comparaison with AutoPIPE: https://www.eng-tips.com/threads/comparaison-with-autopipe.48037/
48. Eng-Tips — Use of CAESAR II for pipeline stress analysis: https://www.eng-tips.com/threads/use-of-cesar-ii-for-pipeline-stress-analysis.451479/
49. Eng-Tips — Why isn't there friction in a guide restraint with a gap: https://www.eng-tips.com/threads/why-isn%E2%80%99t-there-friction-in-a-guide-restraint-with-a-gap-in-caesar-ii.582282/
50. Eng-Tips — Most common reasons CAESAR II diverges: https://www.eng-tips.com/threads/what-are-the-most-common-reasons-that-caesar-ii-diverges.386445/
51. Piping Technology & Products — CAESAR II vs AutoPIPE vs CAEPIPE: https://pipingtech.com/resources/ptp-blog/caesar-ii-autopipe-or-caepipe-your-engineers-guide-to-choosing-the-right-pipe-stress-analysis-software/
52. What Is Piping — Top pipe stress analysis software packages: https://whatispiping.com/top-piping-stress-analysis-software-packages/

Adjacent UI patterns
53. Onshape help — Viewing, selecting, and shortcuts: https://cad.onshape.com/help/Content/Primer/viewing_and_selecting.htm
54. Onshape help — Feature and Part lists: https://cad.onshape.com/help/Content/PartStudio/features_and_parts_lists.htm
55. Onshape help — Versioning and branching: https://cad.onshape.com/help/Content/Document/versions_and_history.htm
56. Onshape help — Comparing: https://cad.onshape.com/help/Content/Document/compare.htm
57. Autodesk — Plant 3D compass and snaps: https://help.autodesk.com/cloudhelp/2026/ENU/Plant3D-UserGuide/files/GUID-80606EFD-7D5C-43CD-8F90-DBFEF07EE34D.htm
58. Product Design Online — dimensioning while sketching in Fusion: https://productdesignonline.com/fusion-360-tutorials/how-to-dimension-sketches-in-fusion-360/
59. McNeel — The Rhino window (command line, options): https://docs.mcneel.com/rhino/8/help/en-us/user_interface/rhino_window.htm
60. McNeel — Gumball guide: https://www.rhino3d.com/docs/guides/user-guide/gumball-basics/
61. Ansys — Discovery SpaceClaim four main tools: https://innovationspace.ansys.com/knowledge/forums/topic/discovery-spaceclaim-four-main-tools/
62. KatsBits — Blender basics (N sidebar, F3 search): https://www.katsbits.com/codex/blender-basics/
63. Philip Davis — Command palette interfaces: https://philipcdavis.com/writing/command-palette-interfaces ; Mobbin glossary: https://mobbin.com/glossary/command-palette
64. Hack Design — Raycast for designers: https://www.hackdesign.org/toolkit/raycast/
65. performance.dev — How is Linear so fast: https://performance.dev/how-is-linear-so-fast-a-technical-breakdown ; Tela — The elegant design of Linear: https://telablog.com/the-elegant-design-of-linear-app/
66. VS Code docs — Code navigation and the Problems panel: https://code.visualstudio.com/docs/editor/editingevolved
67. GitHub docs — Comparing commits: https://docs.github.com/en/pull-requests/committing-changes-to-your-project/viewing-and-comparing-commits/comparing-commits
68. Ansys help — Review results: https://ansyshelp.ansys.com/public//////Views/Secured/corp/v242/en/wb_sim/ds_Review_Results_step.html ; Graph and Tabular Data windows: https://ansyshelp.ansys.com/public/Views/Secured/corp/v251/en/wb_sim/ds_timeline_tab_data.html
69. SimScale docs — Integrated post-processor: https://www.simscale.com/docs/post-processing/new-integrated-post-processor/
70. Autodesk Fusion help — Legend options in simulation studies: https://help.autodesk.com/view/fusion360/ENU/?guid=SIM-LEGEND-OPTIONS-CONCEPT
71. Apple Developer — Behind the Design: Shapr3D: https://developer.apple.com/news/?id=i6qdbzn9 ; Shapr3D — Apple Design Award: https://www.shapr3d.com/blog/shapr3d-wins-the-apple-design-award
72. Stephen Few — Common pitfalls in dashboard design: https://www.perceptualedge.com/articles/Whitepapers/Common_Pitfalls.pdf
73. Dataslayer — dashboard design principles (traffic-light and color-vision note; low-authority source): https://www.dataslayer.ai/blog/dashboard-design-best-practices-15-principles-for-clear-reports
74. Industrial Monitor Direct — CAESAR II vs AutoPIPE (SEO content; used only for the learning-curve sentiment, low trust): https://industrialmonitordirect.com/blogs/knowledgebase/pipe-stress-analysis-software-comparison-caesar-ii-vs-autopipe
75. Hexagon — CAESAR II Applications Guide, restraints chapter: https://hexagonppm.fluidtopics.net/r/en-US/CAESAR-II-Applications-Guide/Version-13/329411
