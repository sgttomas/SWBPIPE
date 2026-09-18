# Implementation-ready handoff — SWBPIPE interface redesign

Status: ROOT record (HELP_HUMAN, successor), 2026-09-18, corrected the same day after independent review (REVIEW-03; dispositions in `{RUN}/instances/REVIEW/REVIEW-03_RETURN.md`), and refreshed the same day after the owner directed how the twenty-five contradictions between the design system and the specification are settled (design system V1.3, specification V1.2, the frames' fourth pass; §1's hashes and §7 changed, §2 to §6 and §8 did not; independent review REVIEW-04, dispositions in `{RUN}/instances/REVIEW/REVIEW-04_RETURN.md`). This record hands the accepted design to implementation. **It starts no implementation and authorizes none.** One owner decision is asked for, in §8. It replaces the [preparation record](IMPLEMENTATION_HANDOFF_PREPARATION_2026-09-18.md), which is kept as history; every prerequisite that record listed is now met except the owner's authorization.

Paths are relative to `{WORKING_ROOT}` (`{REPO_ROOT}/projects/chirality-piping`) unless they begin with `{RUN}` (`{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`).

## 1. What is handed over, by record and hash

| Record | Path | SHA-256 |
|---|---|---|
| Design system V1.3 | `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` | `5aef3bdf5746b0f93f9419b4c7ad459eb634a71db28a846f19133b225f277c81` |
| Tokens 1.2 | `{RUN}/instances/DESIGN-SYSTEM/tokens.json` | `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9` |
| Specimen V1.3 | `{RUN}/instances/DESIGN-SYSTEM/specimen.html` | `f9ce15a26331177bdf78cf6dfb3e5ff568ef0756c02b17333ebc5b6807273a99` |
| UX specification V1.2 | `{RUN}/instances/UX-SPEC/UX_SPEC_V1.md` | `66927ea0cbf062f562cbddbe80f512de504ac4d72f0657cb470a7decfa8dc262` |
| Operations map | `{RUN}/instances/UX-SPEC/OPERATIONS_MAP.md` | `47e2aa303a358b77fa89439aac198146dbc0a2a13cef9fb86521789888f111c2` |
| Specification return (uncertainties) | `{RUN}/instances/UX-SPEC/RETURN.md` | `6a8c7d166122e0effed1a5fc541e12f1dc17954b920a1d1861d592c59be077e9` |
| Frames record, fourth pass | `{RUN}/instances/MOCKS/MOCKS_V4.md` | `98da94b2dda5913eaec20ebc65e6de8ef981a460ad335a00e7c0f4d72dd80a10` |
| Owner's direction on the contradictions | `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_CONTRADICTIONS.md` | `7ac9d16e97b67cf25d5f6f2b04cc70782c248ff02c93a91c4a3e5b1d35180df0` |
| Frames return (per-frame hashes) | `{RUN}/instances/MOCKS/RETURN.md` | `1bfe236c1a71b41f7f2bc002244db8a817812c05353b98c951fcaab771a53779` |
| Frames index | `{RUN}/instances/MOCKS/frames/index.html` | `de2e25efc45dcf852e55869e63105d428afc23b2aed3991e5447eea7a64b2456` |
| Measured facts | `{RUN}/instances/MOCKS/shots/report.json` | `159ff9453145599887ef6460316869fc3f51e90ef95fde4a369389bc704ff8d2` |
| Rendering workload classification | `{RUN}/instances/RESEARCH/G_rendering_workload_classification.md` | `fcf6eb88bb43dbc28d32156b3766cf5a388dad0b96316ca8623c470dc0d2cc45` |
| Baseline reconciliation | `{RUN}/instances/ROOT/PIPING_HANDOFF_RECONCILIATION_2026-09-18.md` | `c0f6a6e182ec35d97683bf1be9444f5c01868e46e5150a344857863481d4ac41` |
| D-70 ruling | `execution/_Coordination/_DECISIONS/D-70_RULING_2026-09-17.md` | `b374b2307d6eb4049803e336edd6e624dbb6688274ceae8a184c361e8fb335f6` |
| D-71 ruling | `execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md` | `2efa514930ff41e47fb90ae8c4c8894f013e94f043ee5020d59b9f6fdab4c533` |
| D-71 addendum (item 7) | `execution/_Coordination/_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md` | `2e04bd4267b933f6f7731caf4d4430cb5711c7833c77bf2d83de4e086c53bc6b` |
| D-71 second addendum (maturity sentence) | `execution/_Coordination/_DECISIONS/D-71_RULING_ADDENDUM_2_2026-09-18.md` | `36dd13bf5c0b2efa6072c9fa1a3b4c48864d43dc2f83a15d21d82e9c6bb0833e` |
| D-72 ruling | `execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` | `76f2eff773d75b2ee1d9facc1f0dcf8196635e5f520d0dbf7abc66b09063e62b` |
| D-72 addendum | `execution/_Coordination/_DECISIONS/D-72_RULING_ADDENDUM_2026-09-18.md` | `e44fb50e244c614689c8ed98f0db87ac65f8fa899f0aee90d2a499d22e49e2cd` |

The eighteen frames are under `{RUN}/instances/MOCKS/frames/`, each a self-contained HTML file, with 1:1 screenshots under `shots/`. The fourth pass redrew them for the owner's direction: `s1_both_light` replaces `s1_table_light`, because a new project opens in Both view; the edit chip's button reads "Apply"; a tooltip names its control and then the key in parentheses; the Review page shows three status chips. The third pass's record (`MOCKS_V3.md`) is kept as history.

The design documents were produced by delegated children under sealed briefs and accepted by ROOT; the briefs, the models that ran and ROOT's checks are in `{RUN}/briefs/_INDEX.md`. ROOT's acceptance is a program-internal check against the brief. It is not the owner's review of the design, and the owner has said these details are subject to change once the product is in use.

Reading order for an implementer: specification §2 (the model of the interface, the one route, run standing, the pointer rule), then §12 (what changes in meaning), then the operations map's gap list, then the design system for appearance, then the frames as worked examples. The twenty-five differences earlier found between the specification and the design system are settled by the owner's direction and applied in both documents; where a new difference is found, it is reported as one and neither document silently governs.

## 2. Decisions in force

| Decision | Record | Effect for implementation |
|---|---|---|
| D-70 | `execution/_Coordination/_DECISIONS/D-70_RULING_2026-09-17.md` | Sequencing with the piping loop; the baseline condition is satisfied ([reconciliation](PIPING_HANDOFF_RECONCILIATION_2026-09-18.md)) |
| D-71, nine items | the ruling record and two addenda; `DEC-099` to `DEC-105` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 | The name is SWBPIPE alone in both layers, with persistence read-compatibility (`DEC-101`); the maturity sentence and the acceptance sentence leave the product (`DEC-105`, `DEC-100`); labels from the one registered table with their domains (`DEC-102`); the export is a model batch file and names no other vendor's product (`DEC-103`); the Checked mark is a tag in interface state with its exact words (`DEC-104`) |
| SCA-010 | `execution/_ScopeChange/SCA-010_2026-09-18_1400/` | Accepted by the owner, not executed. It executes in the tranche that executes `DEC-101`: 23 replacements in the PRD, line 15's historical path excepted, three notice-template lines |
| D-72 | the ruling record and addendum | The performance criteria are frozen: the five D-68 limits; the built-in 120 Hz display gates; the external 60 Hz display, DPR 1 and Dark are observations that do not gate; canvases of 603 × 828 and 1000 × 828 at 1440 × 900; label caps of 138 and 230; one run per size per configuration, repeated twice only when a result is above 80 % of its limit; no fresh baseline cohort |
| Q-15 to Q-22 | the D-71 ruling record | ROOT's recommendations accepted as defaults the owner expects to revisit; on Q-20 the owner said: "I don't want the engineer to have to press buttons. Keyboard inputs are acceptable but it needs a primary control via mouse click." The owner's words are about Q-20. Applying them as a general rule, that every action has a primary control operated by a click and a key is an accelerator, is ROOT's reading, stated so it can be corrected |

Two acts in `DEC-101` and `DEC-105` are not the implementing loop's to take alone. A new Apple App ID is the owner's act. The claims lint at `{REPO_ROOT}/tools/validation/validate_claims_language.py` embeds the content-boundary text and the maturity sentence's anchor; changing a root tool needs the owner's explicit direction under the project's instruction-surface rule, in the same tranche as the text change so the lint never fails on correct text.

## 3. Constraints every implementation brief carries

The owner's ten implementation-handoff constraints, from the preparation record §2, restated as checks a reviewer applies. Constraint 3 separates the owner's obligation from ROOT's decisions. The test in constraint 1 and the absent-or-disabled rule in constraint 6 are ROOT's operational wording of the owner's obligations, not additions to them.

1. **One mutation route.** Every engineering action, by table cell, canvas gesture, paste, accepted proposal or future harness, is a typed operation through the existing Rust applier (`applyModelOperation`, `applyOperationBatch`). A human's and an agent's action are the same operation; that equivalence is a test.
2. **Projections, not rivals.** Tables and canvas are projections and editors of the canonical model. Neither holds state the model does not.
3. **Result integrity.** Current and Historical designation, the exact solve-input basis, stale-response guards, reviewed application, undo and redo, and persistence compatibility are preserved. A Historical record never acquires a current-model overlay or a readiness claim through a presentation change. ROOT's decisions for this pass, not owner rulings, extend that: after a model change the run stops being the current solve basis (as the product already behaves), a Stale run is held on a Historical record's terms, and only a Current run drives an overlay, a chip or a readiness cue (specification §2.6). The owner may reverse the Stale treatment; it is one column of one table.
4. **Picking repair preserved.** PR #794's shared closest-point computation and its regression tests stay. Tolerances and oracle expectations are never altered to obtain a benchmark result.
5. **Rendering foundation retained** unless separately justified and authorized: persistent renderer, instancing and chunking, invalidation scheduling, resource ownership, typed model index, selection and picking.
6. **Every control maps to an operation or a classed gap.** A control whose gap is outside the tranche is absent, or disabled with its reason. It is never faked.
7. **Semantic changes are named as changes** (§5), never delivered as restyling.
8. **The benchmark stays a reusable instrument** with named setup and query actions, typed entity identities, and real pointer and keyboard stimuli.
9. **The demonstration is a comparison basis**, not advance acceptance. Acceptance of performance is under D-72.
10. **Separately scoped work stays separate**: pressure runtime, connector mechanics, sparse execution, export implementation. Shared solver and result contracts are coordinated. Builds and tests stay clear of timed measurement.

## 4. Gaps, classes and write scopes

From the operations map's gap list: 313 rows (3 retired), 34 gap entries.

| Class | Gaps | Write scope | Beyond the tranche's own authorization |
|---|---|---|---|
| Interface | G-27, G-28, G-29, G-32, G-33 | `apps/desktop/src/**`, `apps/desktop/e2e/**` | Independent review is mandatory for product behaviour under `apps/desktop/src/**`. G-32 persists with G-17's carrier; until then it is session state |
| Product text and identity (ROOT's split; the operations map lists G-34 under Interface) | G-34 | product copy, the claims registry, schemas' namespace and document kinds, package and bundle names, the PRD by SCA-010, the root lint | The owner's App ID act; the owner's direction for the root lint edit; persistence read-compatibility tests for documents carrying the former kinds |
| Rendering | G-16, G-30, G-31 | `apps/desktop/src/features/viewport/**` and its tests | The foundation of constraint 5 is out of scope. Per-element edge colour (`canvas.edgeAlt`) is a feasibility question; the fallback is one edge colour |
| Typed interface | G-05, G-07 to G-11, G-14, G-15, G-17, G-18, G-20, G-21, G-25, G-26 | `schemas/**`, `apps/desktop/src/types.ts`, `apps/desktop/src-tauri/**`, persistence | Each new record or field is a schema change with a persistence-compatibility obligation, authorized per gap, never by the mock that needs it |
| Engine | G-01 to G-04, G-06, G-12, G-13, G-22 to G-24 | `core/**` | New change kinds and result fields; independent review mandatory; coordinated with the separately scoped solver work |
| Host | G-19 | none | Held by `DEC-042` and `DEC-091`. The design specifies the seam only |

## 5. Semantic changes

The specification's §12 names twenty, each with today's behaviour cited at `HEAD`, the proposed behaviour and the gap it depends on. The ones a first tranche would meet:

- Isolate dims where today it hides, and the hidden count counts everything hidden (G-30).
- Selection and hover are halos over the element's own colour where today selection replaces it (G-31).
- Every canvas colour is a token with light and dark values; real outside diameter is the resting state; labels follow Budget, not the cap of 80 (G-16). This changes the rendering workload, which is why D-72 fixes label caps and canvas sizes.
- Keys are accelerators only; every action gains a pointer control (G-33).
- The fitted-camera state (G-32).
- The removed sentences, the name and the label forms (G-34).

Keeping a Stale run readable after a model change (G-11) is a typed-interface change with a result-integrity review. Today the product clears the computed results when a model commit lands. Until G-11 is authorized, a first tranche shows the product's present behaviour: after a model change there are no results to show, and the Stale standing is not drawn.

## 6. Verification a tranche owes

The owning loop's registered checks (`software-workflow.json`); the DEC-025 sweep including the host-capability Playwright surfaces; the claims-language lint with its anchors changed in the same tranche as the text; the picking regression tests; operation-equivalence tests for constraint 1; a presentation test that a Historical record gains no overlay, chip or readiness cue; WCAG 2.2 AA criteria for touched controls (D-68); persistence read-compatibility tests where identity changes; a fresh read-only independent review over the complete frozen diff; and performance qualification under D-72 for any tranche that changes the canvas. The independent-usability holds PDU-045 and PDU-046 remain holds.

## 7. Open items carried

- The twenty-five contradictions between the design system and the specification are settled by the owner's direction (the direction record in §1) and applied: design system change-log rows 91 to 112, specification §13 rows 35 to 51. The owner ruled three directly (the tooltip form, the paste band's button faces, the Review page's three chips), confirmed the six functions the specification keeps and the design system gains, and adopted ROOT's approach for the rest. "Apply" on the edit chip is ROOT's word, chosen at the owner's direction to find one other than "Commit"; the owner may replace it.
- The design system child chose the copy for the six additions (the units selector's "As entered" and "Entered", the View tool's menu entries, the stress components' captions, "All kinds", the run menu's tooltip "Runs", the drawer's "Filter"). It is listed in the design system's `RETURN.md` for replacement. The child also added one behaviour: under "As entered" a column holding mixed units drops the unit from its header.
- One new difference, found by independent review (REVIEW-04, M-1) and open: the specification's §2.8 adds that a tooltip which also summarises content puts the summary first and the control with its key last. That clause is the specification child's, not ruled, and the design system's §7.6 does not carry it. The frames' marks tooltips follow it; two frame tooltips put the control first ("Agent (⌘⇧G) · 0 open proposals · opens the column", "Node labels (L): Budget"). All satisfy the ruled form. Whoever next touches the frames or the design system settles the order.
- A reading by the specification child, not ruled: with no rule pack loaded, the Review page shows two chips, Solver and Human.
- The specification's thirteen uncertainties (its `RETURN.md`), among them: hanger selection against a Stale run's values; undo of an accepted proposal row; the multi-change stale band wording; existing product controls not yet searched for rows 278 to 313.
- "Evidence" as the domain word for the two evidence labels is a working word, the owner's to replace.
- The dataviz palette validator was not run on tokens 1.2. Every colour value of 1.1 is unchanged, and the validator ran on 1.1. The two values 1.2 adds, `canvas.edgeAlt` in light and dark, have never been assessed by it.
- All renders are Chromium only.
- From the frames' fourth pass (`MOCKS_V4.md` §4 to §7; the frames' gap numbers are the frames' own series and not the operations map's): questions Q-23 to Q-25 and Q-27 to Q-33 are open, and frames gaps G-13 to G-17, G-19 and G-21. Design system V1.3 closed Q-26, G-18 and G-20. None blocks a first tranche. Worth the owner's eye when the product is first used: the slide-over inspector over a 470 px canvas leaves 170 px of canvas visible (Q-24); the Review header needs more width than the 1440 window gives (Q-25); under a Historical run the table's ratio bars keep their colour while the canvas is neutral (Q-29); the hanger table's state now lives in footer counts that give way when a row is selected (Q-30); the Review page's filter row does not hold in 320 px once a kind is chosen (Q-31); whether a reserved column is drawn empty or absent (Q-32); and what an empty model's canvas is fitted to (frames gap G-21). The frames child's four departures (D4-1 to D4-4) are drawn choices, not rules.
- No frame opens the units menu, the View tool's menu or the stress components' expansion; those three additions are drawn only in the design system's specimen.
- Whether SCA-010 needs re-confirmation after ROOT narrowed it (line 15's path excepted) following review is the owner's call. ROOT has treated the acceptance as standing.

## 8. The one decision asked of the owner

Authorize the piping loop to implement from this handoff, under §3's constraints and §6's verification, in this order:

1. **Tranche A, the rulings.** G-34: the name, the two removed sentences, the label forms, the export's name, the registry and lint anchors, SCA-010's execution, with read-compatibility for existing documents. It is small and mechanical, and everything after it is built under the right names. It needs the owner's App ID act and the owner's direction for the root lint edit.
2. **Tranche B, the shell and the canvas over today's operations.** The interface and rendering classes of §4: the shell, the three views, the one table component, the docked inspector, the status bar, the results header, the pointer controls, the canvas palette, halos, dimming and Budget labels. Qualified under D-72.
3. **Later tranches** for typed-interface and engine gaps return to the owner per gap or per group, because each is a schema or engine change with its own compatibility obligation.

The alternatives are to authorize Tranche B alone and defer the rename, or to widen Tranche B to include G-11 and G-17 so that Stale runs and persisted view state arrive with the shell. ROOT recommends the order above. Until the owner decides, nothing is implemented and the piping loop stays on hold as the owner set it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
