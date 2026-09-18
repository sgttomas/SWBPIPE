# RESEARCH-G — classify the rendering brief's workloads against the product as it exists

Sealed brief. Role: TASK (Type 2), bounded read-only research, working alone; you do not delegate. Parent: ROOT (HELP_HUMAN) of run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Model requested: Opus, because the result is objective: what the source does today.

## Paths

Resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`. `{WORKING_ROOT}` = `{REPO_ROOT}/projects/chirality-piping`. `{RUN}` = `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Never write an absolute machine path in your return.

## Purpose

The design program's rendering brief (`{RUN}/instances/ROOT/RENDERING_BRIEF_2026-09-18.md`) lists ten canvas behaviours (§2), five table-to-canvas needs (§3) and eight observations (§4). No overlay or deformation observation run has been performed. ROOT must now sort every requested behaviour and observation into three classes so that a bounded observation brief covers only what exists, and measurements of unimplemented features never become a circular prerequisite for implementing them:

- **Class 1 — implemented, observable now.** The product at the current revision does this, and the existing benchmark instrument can drive and measure it as it stands, or with configuration only.
- **Class 2 — existing behaviour needing a bounded harness adaptation.** The product does this (fully, or in a stated partial form), but the instrument has no named setup or query action, stimulus or probe for it. Name the smallest adaptation.
- **Class 3 — proposed behaviour requiring implementation before meaningful measurement.** The product does not do this. State the nearest existing behaviour, if any, without presenting it as the proposed one.

## Read

- The rendering brief and its notice (`{WORKING_ROOT}/execution/_Coordination/NOTICE_2026-09-18_RENDERING_BRIEF.md`).
- Product source at the current `HEAD`: `{WORKING_ROOT}/apps/desktop/src/features/viewport/`, `features/results/`, `features/model-tree/`, `features/agent-proposals/`, `features/diff-preview/`, `features/design-workspace/`, `App.tsx`, `styles.css`, and whatever they import for rendering, labels, selection, isolate or hide, result colour, deformation, camera presets and theming.
- The benchmark instrument: `{WORKING_ROOT}/apps/desktop/e2e/ui-foundation/` and `ui-foundation-workflows.ts`, `workspace-driver.ts`; and the method described in `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-PICKING-STABILITY/RUNTIME_REPORT.md` and its `instances/BENCHMARK_PREP/`.
- For what the successful demonstration measured and did not: `RUNTIME_REPORT.md` above and `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE/BASELINE_CHARACTERIZATION_REPORT.md`.

Read-only everywhere. Run no build, no test, no benchmark, no dev server: the piping session's timed work must stay clear of other runtime activity, and this is a source reading.

## Produce

One file: `{RUN}/instances/RESEARCH/G_rendering_workload_classification.md`. Nothing else is written or changed.

1. §0 Basis: the `HEAD` SHA you read, what you read, what you could not determine.
2. §1 One table row per behaviour of the rendering brief §2 (split a numbered item where its parts fall in different classes, for example item 6's halo, isolate and hide), each §3 need, and each §4 observation. Columns: item; class (1, 2 or 3); what exists today, with file and line citations; what is missing; for class 2 the smallest harness adaptation (named setup or query action, typed entity identity, real pointer or keyboard stimulus where interaction responsiveness is the subject); for class 3 the nearest existing behaviour and why measuring it would not characterise the proposed one.
3. §2 Semantic differences between what exists and what the brief proposes that an implementer must treat as a change and not as a restyle (for example dimming versus hiding during isolate; whether camera state persists; whether label mode persists across views). Cite both sides.
4. §3 What the existing instrument records per run (metrics, boundary metadata, trace extraction) and what it cannot record (list unavailable measurements explicitly, for example memory delta, time to first coloured paint, physical scanout), with citations.
5. §4 Answers, from the source only, to the rendering brief §4's two questions as far as a reading can answer them; mark anything that needs a runtime probe as TBD and say which probe.
6. §5 Uncertainties.

## Rules

- Every statement about the product or the instrument carries a file and line citation relative to `{WORKING_ROOT}`. An absence is reported as the searches you ran and their zero result.
- Do not characterise fittings geometry, annotation systems, proposal ghosts, deformation animation or any other feature as existing unless the source draws it. A type, a schema field or a test fixture is not a rendered behaviour; say which you found.
- Trace spans are not hardware GPU execution time; Chromium presentation feedback is not physical scanout; boundary snapshots are not continuous monitoring. Keep those distinctions in your wording.
- No performance claim, no speedup or scaling comparison between the 1,000-pipe and 10,000-pipe records (different display, product and method).
- Canadian English. Do not use the words certify, seal, approve, authenticate, comply, compliant or sign-off for any product control. End the file with the line: "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."

## Acceptance

ROOT accepts when: every §2, §3 and §4 item of the rendering brief appears in the table with exactly one class per row; every class 1 and class 2 row cites the rendering code and the instrument code that make it so; every class 3 row shows the search that found nothing; unavailable measurements are listed; no absolute path; no file other than the return is touched. ROOT will spot-check citations against the source.

## Return

The file above, and a final message of at most fifteen lines: counts per class, the three findings that most affect the observation brief, and your uncertainties.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
