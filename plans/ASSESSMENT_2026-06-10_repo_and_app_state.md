# OpenPipeStress — Repo & Application Assessment

## What this project is

OpenPipeStress is a source-available, noncommercial, **code-neutral piping flexibility and stress-analysis desktop application** built through the Chirality agentic process. The governing stance — "open the mechanics; protect the standards; empower the engineer" — is consistently carried from [INTENT.md](docs/INTENT.md) and the [PRD](docs/PRD.md) through [DIRECTIVE](docs/DIRECTIVE.md), [CONTRACT](docs/CONTRACT.md) (29 OPS-K-* invariants), [TYPES](docs/TYPES.md), [SPEC](docs/SPEC.md), and the IP/professional-boundary policies. The solver computes mechanics; users supply all code-specific data via private rule packs; the software never emits compliance/certification claims; human acceptance is an external, hash-bound record.

## The governance and decomposition layer: complete and internally coherent

[SOFTWARE_DECOMP.md](execution/_Decomposition/SOFTWARE_DECOMP.md) (rev 0.7) decomposes the product into **76 scope items → 18 packages → 101 deliverables**, with zero unassigned scope items and full register backing ([ScopeLedger.csv](docs/_Registers/ScopeLedger.csv), [Deliverables.csv](docs/_Registers/Deliverables.csv)). Four approved scope changes are layered cleanly: SCA-001 (architecture baseline: Rust core, Tauri 2, TS/React/Vite, Three.js, JSON Schema 2020-12, JCS-canonical hashing), SCA-002 (design-engine reframing, PKG-13–16), SCA-003 (local SQLite project store), SCA-004 (PKG-17 export interoperability, CAEPIPE MBF first target).

PKG-00 (the architecture runway) is 8 deliverables, each a complete packet (`_STATUS`, `_CONTEXT`, `_REFERENCES`, `_DEPENDENCIES`, four-document kit, semantic files, run records). The packets themselves are concise spec shells (~40–50 lines); their real force is the **AB-00-01…08 brief-injection rows** in the decomposition §8, which propagate binding constraints (layer boundaries, command/query/job envelopes, persistence determinism, diagnostics taxonomy, layered test strategy) into every downstream brief. That mechanism demonstrably worked — the implemented code matches these constraints closely.

**Lifecycle state:** 100 of 101 deliverables are in `CHECKING`; only DEL-01-01 (governance baseline) is `ISSUED`. Per [_COORDINATION.md](execution/_Coordination/_COORDINATION.md), the project has deliberately exited deliverable-by-deliverable buildout and entered the **application-integration phase**: ordinary work is now bounded app tranches against a "working desktop application" standard (technical preview), with formal issuance deferred to human gates. DAG-006 is the approved dependency authority.

## The code: substantial, real, and green

About **130K LOC** across three stacks, all test surfaces passing as of this run:

| Surface | Scale | Verified |
|---|---|---|
| Rust mechanics/contracts core | 25 crates, ~59K LOC | 422 `#[test]`s; frame_kernel 34/34 and product_physics 23/23 pass |
| Python contract/export layer | ~21 packages, ~24K LOC | **340/340 pytest pass** (13s) |
| Desktop app (Tauri 2 + React 19 + Three.js) | ~23K LOC TS + 2K Rust bridge | 13/13 Vitest pass; production build succeeds |

Highlights of what is genuinely implemented (not stubs):

- **Solver core** ([core/solver](core/solver)): 6-DOF two-node frame kernel with local/global transforms, dense assembly, boundary-condition reduction, deterministic dense solve, thermal strain; straight-pipe element adapter with local force recovery; linear supports; nonlinear support **active-set state classification**; deterministic solver diagnostics (singularity, conditioning, nonconvergence); performance harness.
- **Loads/stress** ([core/loads](core/loads)): primitive load categories, concentrated/distributed user loads, load-case algebra with dimension preservation, code-neutral stress recovery (axial/bending/torsion/pressure membrane).
- **Rule-pack engine** ([core/rules](core/rules)): sandboxed expression-tree evaluator (no eval, no host access), required-input completeness checker mapping to `RULE_INPUTS_INCOMPLETE`, lifecycle/checksum module.
- **Reporting** ([core/reporting](core/reporting)): audit manifest with canonical JSON hashing, report sections, result export envelopes, report generator, protected-content linter.
- **36 JSON schemas** ([schemas/](schemas)) — the canonical contract surface, heavily exercised by the Python tests.
- **Export/interop layer** (Python, [core/handoff](core/handoff)): native JSON package, CAEPIPE MBF writer, CAEPIPE external-run harness/CSV parser, stress-neutral CSV/JSON, conservative PCF, GLB/glTF review geometry, adapter SDK, target mapping — i.e., PKG-17 has real implementations with tests.
- **Desktop app** ([apps/desktop](apps/desktop)): 12 Tauri IPC commands; the preview mechanics path links the **real solver crates** through [product_physics](core/product_physics/Cargo.toml) (frame_kernel + straight_pipe + linear_supports + primitive_loads + load_case_algebra + stress_recovery) — not a mock; local SQLite project store with `user_version` migration ledger and project-envelope hash verification on open (the current uncommitted tranche, ~647 lines, continues this hash-evidence series); background solve jobs on real threads; ~40 feature panels surfacing diagnostics, rule-check, missing-data, telemetry, threat-model, export, comparison, and agent-proposal contracts; boundary notices everywhere.

The IP/professional-boundary discipline is pervasive and consistent in the code: invented fixtures only, provenance fields throughout, no compliance vocabulary in software-emitted statuses. I found no protected-content red flags.

## Completeness against the PRD's own milestones

- **R0 (architecture prototype)** — met in substance.
- **R1 (core solver MVP)** — substantially met for *linear static* analysis: solve, force/stress recovery, singularity detection, benchmark suites with hand-calc witnesses ([validation/benchmarks](validation)). Caveat: dense solve only.
- **R2 (GUI MVP: "create, solve, and report a small model without editing raw files")** — **not met**. This is the largest gap (detail below).
- **R3 (rule packs + private libraries)** — engine-side largely present; GUI authoring and final expression grammar not.
- **R4 (piping components + nonlinear supports)** — schema/data-model complete, solver-side **not implemented**.
- **R5 (engineering beta)** — distant; all release machinery is deliberately TBD.

## Significant development gaps

1. **No interactive model authoring (blocks R2).** The viewport ([PipeViewport.tsx](apps/desktop/src/features/viewport/PipeViewport.tsx), ~310 lines) renders centerline geometry but editing is demonstrated via buttons that enqueue *editor operation intents*; there is **no Tauri command that applies an operation and mutates a persisted model** — the 12 commands cover preview loading, mechanics jobs, and project envelope create/open/save/list only. The PKG-16 operation→validation→diff→accept loop exists as contracts and Python modules but is not wired end-to-end into the app. Until intents can become accepted model edits, the app remains a fixture-driven preview, not a modeler.

2. **No unit-conversion engine.** `core/units/` is empty by design: the unit catalog, conversion constants, and tolerance policy are gated on a human governance acceptance that hasn't happened (the SPEC keeps offset-temperature, gauge/absolute pressure, etc. as blocking TBDs). Everything carries unit metadata, but nothing converts. Real engineering use in mixed units is impossible until this is resolved — it is the most consequential *deliberate* TBD.

3. **Solver scope vs. piping reality.** Bends/elbows, branches, expansion joints, and spring hangers exist as **library schemas only** — there is no curved-pipe/bend macro-element, no flexibility-factor-modified stiffness, no expansion-joint element in the solver (the only "bend" in [frame_kernel](core/solver/frame_kernel/src/lib.rs) is straight-beam bending terms). The nonlinear active-set classifier is not driven by an assembled global iterative solve, so gaps/friction/lift-off aren't actually solvable. Sparse solver selection remains TBD (dense only → small models). For a *piping* tool, straight-pipe-only is the biggest mechanics gap.

4. **Headless runner is a contract, not a CLI.** [core/runner/headless](core/runner/headless) has no `main`/binary; CLI syntax, process policy, and package scripts are explicitly deferred (DEL-10-05 TBDs).

5. **Reports don't render.** Report generation assembles deterministic structured envelopes/sections; final document rendering (layout, styling, export to a reviewable calc report) remains TBD, as do redaction/export controls beyond contract level.

6. **Thin frontend test depth.** 13 Vitest tests for a ~23K-LOC frontend, manual [SMOKE.md](apps/desktop/SMOKE.md) checklist, and **no Playwright** despite DEC-011 naming it in the accepted test baseline. GUI behavior is the least-verified layer.

7. **Release machinery is all TBD** (deliberately): no CI provider or workflows, no release matrix/signing/packaging, no accepted numerical tolerance or coverage thresholds. The project's own [gap register](execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv) tracks these as release-claim blockers (RGAP-003/004 among 10), matching my reading.

8. **Governance closure pending:** 100 deliverables awaiting human issuance; maintainer roster/quorum, contributor legal mechanism, release-label vocabulary all open; model-document schema migrations and the native-package physical container are named as next seams in the untracked [init/init-prompt.md](init/init-prompt.md).

One stale-doc item to flag per the project's own "surface discrepancies" rule: [BUILD_AND_RELEASE.md](docs/BUILD_AND_RELEASE.md) §3 still states the repo has "no root Cargo workspace, root JavaScript package, or desktop shell package manifest" — the root `package.json` and `apps/desktop` now exist, so that section lags reality.

## Bottom line

This is a **disciplined, mid-stage project in excellent health for where it is**: the governance/decomposition layer is complete and coherent; the contract-and-mechanics foundation (~130K LOC, all tests green) is real and well-bounded; and the desktop technical preview genuinely exercises the real solver through the accepted app/core boundary with local persistence and hash evidence. What it is **not yet** is the product the PRD describes: an engineer cannot author a model in the GUI, convert units, model a bend, run a nonlinear case, or print a calculation report. The next phase's center of gravity is exactly where the coordination record points it — closing the intent→operation→persisted-model loop, the unit catalog decision, solver component/nonlinear depth, and GUI test evidence — followed by the human-gated issuance and release-authority decisions that everything else is queued behind.
