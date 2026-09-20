**Recommendation:** define the first publishable SWBPIPE as a **local desktop workbench for creating, solving, comparing and handing off small piping concepts**, with a deliberately bounded linear-static model. “Finished” should mean one complete, dependable user journey—not completion of every planned component or interface feature.

This is advice, not an accepted scope change. Inventory used committed bytes at `e8f91e086a0ce65ca706bd63e0a3c6e4ef21f15a`. Actual executor: **TASK Type2, gpt-6-astra / low**, under ROOT. No writes, builds, tests, UI, delegation or network mutations.

### Existing product boundary

`projects/chirality-piping/docs/PRD.md` is adopted authority despite its “0.4 Draft” heading.

- **Purpose/audience:** early-design piping route/support iteration by qualified designers and engineers; transparent mechanical results and downstream handoff. It is not positioned as a replacement for professional stress-analysis software (§§1,3–4).
- **MVP mechanics:** six-DOF 3D nodes; straight pipe/beam and rigid elements; anchors/linear restraints; weight, thermal and imposed-displacement loading; pressure metadata; linear-static solving; forces, moments and fundamental stresses (§23, lines 1429–1462).
- **MVP workflow:** local storage, units, GUI tree/inspector, material/section entry, missing-data blockers, private rule-pack schema, named immutable states, state-bound runs, basic model/run comparison, canonical export and reports (§23).
- **Explicit exclusions:** comprehensive commercial-result ingestion, forced prover lifecycle, protected standards data, automatic professional acceptance, advanced nonlinear/dynamic modules unless separately prioritized (§23:1464–1470). Broader special-domain exclusions appear at §7:217–227.
- **External validation:** correlation with an external professional solver is the principal product-validation posture; activation requires lawful access and owner direction. Internal benchmarks remain development verification until activation (§22.5:1410–1425).
- **Platform/package:** v0.1 macOS Apple Silicon, Tauri `.app` zip plus SHA-256; other platforms require evidence-gated expansion (`SOFTWARE_DECOMP.md` DEC-057; `BUILD_AND_RELEASE.md` §6.1:193–230).

### What “MVP” currently means—and does not mean

There are several overlapping meanings:

1. **PRD §23 MVP** includes states, runs and comparison.
2. **R1 “Core Analytical Engine MVP”** is an engine milestone.
3. **R2 “GUI and Physical Model MVP”** exits when a user can create, solve, review and save a small model without raw-file editing (§24:1511–1527).
4. **R3** adds deterministic two-state/two-run comparison (§24:1529–1544).
5. **R6 “Design Knowledge and Handoff Beta”** adds richer design constraints/handoff and clean-checkout validation-manual reproduction (§24:1580–1595).

A technical preview or beta label is a maturity statement, separate from mechanical correctness, external correlation and human engineering acceptance. `VALIDATION_STRATEGY.md` §4:105–125 still reserves release-label vocabulary and final policy to the owner. `RELEASE_QUALITY_GATES.md` §8 refers to an “engineering-beta” floor, while that strategy now describes future release review more generally. **This wording should be reconciled before choosing the public label.**

### Completion and publication gates to preserve

- **End-to-end usability:** R2’s raw-file-free create → solve → review → save, plus §23/R3’s state/run comparison and canonical handoff.
- **Software evidence:** applicable mechanics, stress, schema/unit, rule completeness/sandbox, GUI and report tests; reproducibility, warnings, provenance and limitations retained (`PRD` §22.2–22.4; `RELEASE_QUALITY_GATES` §§3–7).
- **Human release decision:** sole maintainer is sole release authority; external contribution intake remains separately gated (`SOFTWARE_DECOMP` DEC-027/079).
- **Publication scan:** release artifact classes scanned with explicit applicability, private-data/secret/provenance checks and owner sign-off. Blocking findings stop publication; clean scan is not legal clearance (`SOFTWARE_DECOMP` DEC-058, line 649).
- **Public export:** staged sanitized export meeting G1–G7, green governed scan, and selected public repository identity precede public publication/CI activation (`SOFTWARE_DECOMP` DEC-059, line 650).
- **Packaging is not publication:** package, checksum and artifact record do not themselves authorize release (`BUILD_AND_RELEASE` §6.1:223–230).
- **Signing:** D-06b/DEC-089 adopts Developer ID signing/notarization as a future target, with implementation, credentials, agreements, evidence and publication gates still separate. Present unsigned posture persists until those conditions are met (`D-06b_RULING_2026-07-25.md`, “Selected effect”).
- **Current interface qualification:** D-72 and its addendum preserve performance criteria, independent-usability holds PDU-045/PDU-046 and WCAG 2.2 AA for touched controls. The addendum completes the previously open cohort rule; it does not establish qualification. `WORK_GRAPH.json` still plans closing visual review, owner-frozen second profile and D72 qualification.
- **Merge evidence remains separate:** clean-candidate DEC-025 obligations and independent review in `loop/LOOP_INIT.md`; DEC-093 provides a bound CI alternative for surface 4. Neither constitutes release acceptance.

### Current work versus the whole product journey

These are **committed coordination-record claims**, not independently reverified behavior:

| Journey area | Recorded state at basis |
|---|---|
| Workspace shell, Table/Model/Both, toolbar/stage navigation | B3 merged |
| Unsaved-edit marker and not-solved fallback | B3A merged; source/head and later hosted pass recorded |
| Project request safety and verified-at-save integrity | B3B connected/native/review gates pending |
| Isolation/picking/hidden counts | C3 connected checks active |
| Unified table edits | B4 planned |
| Remaining inspector/slide-over work | B5 planned |
| Results header/run menu/Historical presentation/issues/search | B6 planned |
| Readable stale runs with integrity review | G-11 planned |
| Review page | B7 planned |
| Compatible persisted view state | G-17 authorized, not launched |
| Labels, outside diameter/HUD/camera | C4/C5 planned |
| Visual closure and performance qualification | Planned; second-profile freeze owner-held |

Source: run `WORK_GRAPH.json` nodes/current_frontier, under `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/`.

The work graph does **not** establish completeness of named state management, deterministic comparisons, private-library/rule authoring, installation/onboarding, sanitized public export or release packaging. Those are inventory gaps, not proof that their implementation is absent. Deliverable bindings are explicitly provisional; current UI integration cannot substitute for the later scope reconciliation.

### Decisions worth sharpening now

1. **First public promise:** narrowly useful engineering preview, or the whole §23 MVP? The former can be a staged publication target; calling it the latter while omitting comparison/state requirements would require an explicit amendment.
2. **Reference journey:** recommend “start blank; author a small linear piping route with supports/materials/loads; diagnose missing inputs; solve; inspect reactions/forces/stresses and warnings; save/reopen; change a support or route; compare two states/runs; export a reproducible package/report.”
3. **Bounded model envelope:** publish an explicit supported subset and reject unsupported inputs visibly. Do not make nonlinear supports, advanced components or agent-assisted generation prerequisites merely because implementation exists elsewhere.
4. **Meaning of handoff:** canonical schema package is the MVP requirement; a trustworthy direct exchange with a named external tool is a stronger separate promise. Decide which the first publication makes.
5. **Release label and validation:** choose a maturity label and define what evidence earns it; distinguish “internally verified” from “prover correlated.” Independent third-party reproduction is a publication-era objective, not an R6 exit prerequisite (`PRD` §22.1:1356).
6. **Install experience:** decide whether the adopted signing target is required for the first public download or whether an explicitly disclosed unsigned preview is acceptable.
7. **Update stale release prose selectively:** build/release docs still say hosted CI is deferred and D-06b awaits decision, despite later DEC-093 and DEC-089. `HANDOFF_STATE.md` is a dated log; even current graph parent summaries lag newer child statuses. Use dated rulings and explicit latest-node state, not those summaries, to define readiness.

**Recommended finish line:** a new user on the supported Mac can complete the reference journey from an installed package, without raw-file edits or developer assistance, recover their work, understand unsupported cases and warnings, and hand another person a reproducible model/result package. Retain the full existing criteria for the claimed scope; postpone optional breadth rather than dilute correctness.

### Consulted origins and hashes

All paths are committed at the stated basis. Prefix `P/` means `projects/chirality-piping/`; `R/` means its run directory named above; `D/` means `P/execution/_Coordination/_DECISIONS/`.

```text
d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b AGENTS.md
1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7 agents/AGENT_TASK.md
eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005 P/AGENTS.md
f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94 P/loop/LOOP_INIT.md
cd6297af2ed9588f6da24c0665d32c74ee33d3e7f9115e733b47244fd75b4611 P/docs/PRD.md
effc85b7ea52782c1d99c059fe2bb92d8a266087c061f021b647b4a80143d4ba P/docs/RELEASE_QUALITY_GATES.md
5ff4e8108bf0dbac6f3630f45914b6b3a422684b0fd2961eb705df1338dc0226 P/docs/BUILD_AND_RELEASE.md
a9bce923d5ca9656191a4abddad8f9464c2fde6b37c94b3d787e98c00949acda P/docs/VALIDATION_STRATEGY.md
0a3ee466dabfca57ba6845f110545d9afb6a5fbfc516ce4fb56e28da3a503c78 R/WORK_GRAPH.json
84aaa4dd728515cde6452e74fe8af537f69190b416329b917fbf80e3f69a475c R/HANDOFF_STATE.md
4b4442b911c8f6d686db4e4f55d975d9266720ff4f65a8803035f31458699c32 P/execution/_Decomposition/SOFTWARE_DECOMP.md
425da859ef768a2f5fdaa1532a6142228b1715dc53095a3d8b1f23b446e53410 D/D-06b_RULING_2026-07-25.md
45bcbb2bc2589ec40628fab22015cf5a893264af440f39f2e7791311f2e9c285 D/D-48_claims_language_taxonomy.md
76f2eff773d75b2ee1d9facc1f0dcf8196635e5f520d0dbf7abc66b09063e62b D/D-72_RULING_2026-09-18.md
e44fb50e244c614689c8ed98f0db87ac65f8fa899f0aee90d2a499d22e49e2cd D/D-72_RULING_ADDENDUM_2026-09-18.md
23bc60bf2b93d7b3231e2ec420e85cf49337584f5d7c0392d2dd8da4304b59b1 D/D-05b_public_export_ci_activation.md
```

Instruction bytes were confirmed identical to the previously read basis. Proposal packet headers were not treated as rulings; acceptance was traced through the DEC rows or explicit ruling/addendum. No practitioner acceptance, performance qualification or release readiness was established.

