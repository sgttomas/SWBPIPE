# Sealed brief — DESIGN-SYSTEM-03: design system revision V1.2

Sealed by ROOT (HELP_HUMAN, successor) on 2026-09-18 before launch, after the owner ruled decision packet D-71 in session (items 1 to 6, 8 and 9; item 7 held for discussion) and accepted ROOT's recommendations on the frames' questions Q-15 to Q-22 with one amendment. Role: HELPS_HUMANS design manager, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1. Mechanism: Claude Code `Agent` tool, general-purpose type, background.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Never write an absolute machine path in anything you produce.

## Purpose

Revise the design system from V1.1 to V1.2 so that the specification, the token file and the specimen carry the owner's rulings, the answered screen questions and the closed gaps. The UX specification and the frames are revised from V1.2 after you; nothing in them should then contradict the system.

## Read, in this order

1. `{RUN}/instances/DESIGN-SYSTEM/` as it stands (V1.1): `DESIGN_SYSTEM_V1.md` with its §9 change log of 49 rows, `tokens.json` 1.1, `specimen.html`, `tools/`, `RETURN.md`.
2. `{RUN}/instances/ROOT/REVISION_PASS_PREPARATION_2026-09-18.md`: §1 inputs R-1 to R-9, §2 the eight questions with ROOT's recommendations, §3 the six gaps with ROOT's directions. Where this brief differs from that record, this brief governs (it was written before the owner answered).
3. `{RUN}/instances/MOCKS/MOCKS_V2.md` §3 to §6 (departures, frame decisions F-1 onward, Q-15 to Q-22, G-7 to G-12) and the frames under `{RUN}/instances/MOCKS/frames/` as evidence of how V1.1 draws.
4. `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/_DECISIONS/D-71_interface_program_governed_text_decisions.md` for what each item was about. The rulings below govern; the packet's recommendations do not where they differ.

## The owner's rulings, as design consequences

The owner's words are recorded by ROOT in the D-71 ruling record; you apply these consequences.

1. **The name is SWBPIPE, and only SWBPIPE.** The longer name "SWB Piping Designer" is dropped everywhere: window title, About, report identity block, toolbar, specimen, examples, this document's own title and prose. The product is never called a "Technical Preview" as part of its name. The old name appears nowhere.
2. **The maturity sentence** ("Technical preview — not a released product.") lives in the status bar's information popover and in About, and nowhere permanent. This is what V1.1 draws; state it as ruled, no longer conditional.
3. **The acceptance sentence is removed from the product.** No surface carries it or any short variant of it: not the results header or its disclosure, not the Review page, not the report, not the inspector, probe, canvas or agent panel. Remove its disclosure home (M-02) from §7.4 and wherever else it is placed, and close the space it held rather than leaving a hole. The owner's reason: it was there to restrain the product's first definition and no longer serves a purpose. This does not touch the control word "Accept" for a proposed edit, the Checked mark, or the report's required notice, which is governed text outside the design system.
4. **Status and evidence labels come from one table and nowhere else:** `MODEL_INCOMPLETE` "Model incomplete", `MECHANICS_SOLVED` "Mechanics solved", `RULE_INPUTS_INCOMPLETE` "Rule inputs incomplete", `USER_RULE_CHECKED` "User-rule checked", `USER_RULE_FAILED` "User-rule failed", `HUMAN_REVIEW_REQUIRED` "Human review required", `INTERNALLY_VERIFIED` "Internally verified", `PROVER_CORRELATED` "Prover correlated". Three rules: the raw token is reachable in place; the authority domain is shown with the label; no label exists outside the table. "Human review required" shows on the Review page and in the report. No chip when the model is complete but unsolved. The evidence chip appears only when a run record carries evidence. "User rules checked" and "User rule failed" become the hyphenated forms everywhere.
5. **Hanger tables** are a user-imported library class, never bundled; the hanger-selection surface carries the content boundary's short variant "no protected standards content; code-specific data is user-supplied".
6. **Historical run wording** is the text the product renders: band "Historical saved run · Run a fresh solve to establish current results."; the popover carries "Historical results cannot drive current overlays, rule checks, comparisons or report readiness." A Historical record never takes a current-model overlay or a readiness cue: specify the results header band, the legend and the canvas colour state for Current and for Historical separately.
7. **The Checked mark is unchanged in this revision.** Its classification is still with the owner. Keep V1.1's anatomy and words and mark the item open in §8; do not strengthen or weaken it.
8. **Agent cards** carry exactly one of five class words as their label: "Check", "Open issue", "Draft", "Proposal", "Evidence summary". The agent-authored "Note" becomes "Evidence summary". The words accepted, approved, verified, correct and compliant never appear in agent text.
9. **The export never names another vendor's product.** The string "CAEPIPE" appears in no product copy, example, tooltip, dialog or specimen text. The command is "Export model batch file (.mbf)…"; the dialog title is "Model batch file (.mbf)"; there is no grammar-attribution line and no compatibility flag. The design system's own explanatory prose may say where the row grammar came from only if it is clearly not product copy; prefer "the model batch file grammar".

## The screen questions, as answered

Apply ROOT's recommendation for Q-15, Q-16, Q-17, Q-18, Q-19, Q-21 and Q-22 as written in the preparation record §2 (in Q-22 the fourth kind is "Evidence summaries"). The owner added that these details may change once the app is in use; specify them as defaults, not as doctrine.

**Q-20, amended by the owner:** "I don't want the engineer to have to press buttons. Keyboard inputs are acceptable but it needs a primary control via mouse click." Two consequences. First, keep the recommendation: if the camera has not moved since the last Fit, docking and undocking refit by themselves; otherwise the camera is kept. Second, state a general rule in §5 and §7 and check the whole system against it: **every action has a visible primary control operated by a mouse click; a keyboard shortcut is an accelerator for a control that exists, never the only way.** Fit stays visible in the canvas HUD at every canvas width, including the narrow case (G-10). List in RETURN every action in V1.1 that had only a key (for example ⌘I, ⌘⇧G, F, Escape, Tab in the routing compass) and the pointer control you gave it or found for it.

## The gaps

Apply the preparation record §3 for G-7, G-8, G-10, G-11 and G-12. G-9 closes unused: there is no caption band, because ruling 3 removed the sentence it would have carried.

## Also apply

R-4, R-5 and R-6 of the preparation record §1. Continue the §9 change log from row 50, one row per change with its source (ruling number, Q, G or R). Rename the section heading to cover V1.1 to V1.2 or add a §10; keep rows 1 to 49 as they are.

## Produce, in place under `{RUN}/instances/DESIGN-SYSTEM/`

`DESIGN_SYSTEM_V1.md` at V1.2 (keep the filename), `tokens.json` at version 1.2 if any token changes (say so if none does), `specimen.html` regenerated and self-contained (one file, styles inline, no network request, light and dark), `tools/` updated as needed with no absolute path, and `RETURN.md` replaced with this pass's return: what changed, what you verified and how, the key-only actions list, uncertainties, and anything in the rulings you could not apply cleanly. Write nothing outside that directory. Run no product build or test. Do not run git commands that change state.

## Copy rules

Canadian English ("colour", "Analyze"). "Accept", never "Approve", for product controls. Never certify, seal, approve, authenticate, comply, compliant or sign-off as a control; "Review/signoff block" remains the registered section name. End `DESIGN_SYSTEM_V1.md` and `RETURN.md` with: "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."

## Acceptance

ROOT accepts when: every ruling, question and gap above is applied and traceable to a numbered change-log row; a case-insensitive search of the specification, token file and specimen finds no "SWB Piping Designer", no old product name, no "CAEPIPE" in product copy, and no acceptance sentence or variant; the label table is the only source of status and evidence labels; the pointer-control rule is stated and the key-only list is complete; your agreement check reports no difference between document, token file and specimen; the specimen renders offline in both themes with no request and no console error; nothing outside the instance directory changed.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
