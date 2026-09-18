# Implementation-ready handoff — SWBPIPE interface redesign

Status: ROOT record (HELP_HUMAN, successor), 2026-09-18, corrected the same day after independent review (REVIEW-03; dispositions in `{RUN}/instances/REVIEW/REVIEW-03_RETURN.md`). This record hands the accepted design to implementation. **It starts no implementation and authorizes none.** One owner decision is asked for, in §8. It replaces the [preparation record](IMPLEMENTATION_HANDOFF_PREPARATION_2026-09-18.md), which is kept as history; every prerequisite that record listed is now met except the owner's authorization.

Paths are relative to `{WORKING_ROOT}` (`{REPO_ROOT}/projects/chirality-piping`) unless they begin with `{RUN}` (`{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`).

## 1. What is handed over, by record and hash

| Record | Path | SHA-256 |
|---|---|---|
| Design system V1.2 | `{RUN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` | `efa22d497772e717b4cc19bc2a703ed765d0ac158fa177efbf39f373e0acfc1f` |
| Tokens 1.2 | `{RUN}/instances/DESIGN-SYSTEM/tokens.json` | `00c1afe97365973e6c05a2a5d938a25c46022a55c3cbc8d04ff3a9d2d68cf1e9` |
| Specimen | `{RUN}/instances/DESIGN-SYSTEM/specimen.html` | `219fa8ee78c44507fcfc0b6a199298c8f4a4026abe798f4166eeda2eb607941b` |
| UX specification V1.1 | `{RUN}/instances/UX-SPEC/UX_SPEC_V1.md` | `ad91a456541b2486581f40fa600a0ceef564df432e9133726ae0f806d4c7139c` |
| Operations map | `{RUN}/instances/UX-SPEC/OPERATIONS_MAP.md` | `a64ba8bed5216de533ba804748102bf7bbaa18cd17473ce52179aa3f4bf599e4` |
| Specification return (contradictions, uncertainties) | `{RUN}/instances/UX-SPEC/RETURN.md` | `7a43a53f93cbb8f14eb254811e1f70db3d03bc0e2da1984fdf5f2e6140807a81` |
| Frames record, third pass | `{RUN}/instances/MOCKS/MOCKS_V3.md` | `805f78d7cae0190a8723e9110667cd7b2a31cba5d65016335cbadb9f574fbf1a` |
| Frames return (per-frame hashes) | `{RUN}/instances/MOCKS/RETURN.md` | `45807f8f8c786802ed6742f15db8b05fa43ff7751f822a05e53cc1bc17fcfc3d` |
| Frames index | `{RUN}/instances/MOCKS/frames/index.html` | `ee3b4a078cb55175e7952ced1e23c444a18146e2964030dd105a5331e0f10ee9` |
| Measured facts | `{RUN}/instances/MOCKS/shots/report.json` | `884e0136c3ffaebb7fd609ca0fec650434f65ab48857027f06d393f6e45cc740` |
| Rendering workload classification | `{RUN}/instances/RESEARCH/G_rendering_workload_classification.md` | `fcf6eb88bb43dbc28d32156b3766cf5a388dad0b96316ca8623c470dc0d2cc45` |
| Baseline reconciliation | `{RUN}/instances/ROOT/PIPING_HANDOFF_RECONCILIATION_2026-09-18.md` | `c0f6a6e182ec35d97683bf1be9444f5c01868e46e5150a344857863481d4ac41` |
| D-70 ruling | `execution/_Coordination/_DECISIONS/D-70_RULING_2026-09-17.md` | `b374b2307d6eb4049803e336edd6e624dbb6688274ceae8a184c361e8fb335f6` |
| D-71 ruling | `execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md` | `2efa514930ff41e47fb90ae8c4c8894f013e94f043ee5020d59b9f6fdab4c533` |
| D-71 addendum (item 7) | `execution/_Coordination/_DECISIONS/D-71_RULING_ADDENDUM_2026-09-18.md` | `2e04bd4267b933f6f7731caf4d4430cb5711c7833c77bf2d83de4e086c53bc6b` |
| D-71 second addendum (maturity sentence) | `execution/_Coordination/_DECISIONS/D-71_RULING_ADDENDUM_2_2026-09-18.md` | `36dd13bf5c0b2efa6072c9fa1a3b4c48864d43dc2f83a15d21d82e9c6bb0833e` |
| D-72 ruling | `execution/_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md` | `76f2eff773d75b2ee1d9facc1f0dcf8196635e5f520d0dbf7abc66b09063e62b` |
| D-72 addendum | `execution/_Coordination/_DECISIONS/D-72_RULING_ADDENDUM_2026-09-18.md` | `e44fb50e244c614689c8ed98f0db87ac65f8fa899f0aee90d2a499d22e49e2cd` |

The eighteen frames are under `{RUN}/instances/MOCKS/frames/`, each a self-contained HTML file, with 1:1 screenshots under `shots/`: the sixteen design frames regenerated, a slide-over frame and a Historical frame added, and the two D-71 decision-aid frames retired. The frames were drawn before the design system's correction 2 and the specification's correction 1, which changed attribution and wording in prose only and no appearance, string or behaviour a frame draws.

The design documents were produced by delegated children under sealed briefs and accepted by ROOT; the briefs, the models that ran and ROOT's checks are in `{RUN}/briefs/_INDEX.md`. ROOT's acceptance is a program-internal check against the brief. It is not the owner's review of the design, and the owner has said these details are subject to change once the product is in use.

Reading order for an implementer: specification §2 (the model of the interface, the one route, run standing, the pointer rule), then §12 (what changes in meaning), then the operations map's gap list, then the design system for appearance, then the frames as worked examples. Where the specification and the design system differ on one of the sixteen points the specification's return lists, the specification governs; those sixteen are candidates for a design system correction and are not blocking.

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

- The sixteen contradictions between the design system and the specification, read in the specification's favour; a design system correction is owed before the frames are used as a pixel reference for those points.
- The specification's thirteen uncertainties (its `RETURN.md`), among them: hanger selection against a Stale run's values; undo of an accepted proposal row; the multi-change stale band wording; existing product controls not yet searched for rows 278 to 313.
- "Evidence" as the domain word for the two evidence labels is a working word, the owner's to replace.
- The dataviz palette validator was not run on tokens 1.2. Every colour value of 1.1 is unchanged, and the validator ran on 1.1. The two values 1.2 adds, `canvas.edgeAlt` in light and dark, have never been assessed by it.
- All renders are Chromium only.
- The frames' return lists nine further contradictions between the two documents (C-17 to C-25), eight questions only a screen raises (Q-23 to Q-30) and eight component gaps (frames gaps G-13 to G-20, numbered in the frames' own series and not the operations map's). None blocks a first tranche. Three are worth the owner's eye when the product is first used: the slide-over inspector over a 470 px canvas leaves 170 px of canvas visible (Q-24); the Review header needs more width than the 1440 window gives, and the results header more than the 737 px Both-view pane gives (Q-25, Q-26); and under a Historical run the table's ratio bars keep their colour while the canvas is neutral (Q-29).
- Whether SCA-010 needs re-confirmation after ROOT narrowed it (line 15's path excepted) following review is the owner's call. ROOT has treated the acceptance as standing.

## 8. The one decision asked of the owner

Authorize the piping loop to implement from this handoff, under §3's constraints and §6's verification, in this order:

1. **Tranche A, the rulings.** G-34: the name, the two removed sentences, the label forms, the export's name, the registry and lint anchors, SCA-010's execution, with read-compatibility for existing documents. It is small and mechanical, and everything after it is built under the right names. It needs the owner's App ID act and the owner's direction for the root lint edit.
2. **Tranche B, the shell and the canvas over today's operations.** The interface and rendering classes of §4: the shell, the three views, the one table component, the docked inspector, the status bar, the results header, the pointer controls, the canvas palette, halos, dimming and Budget labels. Qualified under D-72.
3. **Later tranches** for typed-interface and engine gaps return to the owner per gap or per group, because each is a schema or engine change with its own compatibility obligation.

The alternatives are to authorize Tranche B alone and defer the rename, or to widen Tranche B to include G-11 and G-17 so that Stale runs and persisted view state arrive with the shell. ROOT recommends the order above. Until the owner decides, nothing is implemented and the piping loop stays on hold as the owner set it.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
