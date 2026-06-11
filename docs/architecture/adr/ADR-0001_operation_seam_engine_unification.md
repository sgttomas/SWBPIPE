# ADR-0001: Operation-seam engine unification — wasm32 `operation_applier` as the sole browser-mode engine

| Field | Value |
|---|---|
| Status | accepted |
| Date | 2026-06-11 |
| Human authority | Human project authority ruling, 2026-06-11, accepting `plans/PLAN_2026-06-11_operation_seam_unification.md` §2 as proposed (ruling record transcribed verbatim in that section; D-13 in `execution/_Coordination/_DECISIONS/_REGISTER.md`, state RULED) |
| Decision log cross-reference | `DEC-020` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |

## Context

The structured editor-operation seam — the enforcement point for
no-silent-defaults, unit/dimension checks, before-state integrity, and
professional-boundary receipts — was implemented twice
(plan §1, FACT basis):

- **Authoritative engine:** `core/model_operations/operation_applier/src/lib.rs`
  (Rust, ~4,885 lines, 34 tests), invoked by the `validate_model_operation` /
  `apply_model_operation` Tauri commands.
- **Browser mirror:** `apps/desktop/src/services/operationService.ts`
  (~2,280 lines, 17 tests), used whenever no Tauri backend exists — Vite dev
  mode, all jsdom/Vitest edit-loop tests, the Playwright e2e harness, and the
  agent in-browser smoke evidence that closed the 27 tranches of 2026-06-10/11.

The mirror was a deliberate, code-level choice — its file header names the
Rust crate as "the contract reference" — and it bought real value: the browser
evidence loop (Vite dev, jsdom/Vitest, Playwright) and agent verification
velocity. But no executable contract enforced alignment between the two
engines (each suite tests only its own engine), and no ADR recorded the
decision — the ADR surface DEL-00-01 anticipated did not exist in the worktree
until this entry stood it up.

Why this was resolved now rather than later: two scheduled work fronts
multiply the duplication — the R2 from-scratch authoring set (~4–6 new
operation kinds) and Phase B unit-aware fields (conversion/tolerance logic per
`DEC-018` — numerically sensitive code that must never be hand-maintained
twice). The failure mode if unresolved: environment-dependent validation
outcomes and environment-dependent canonical model hashes — an invisible
fracture of the project's central no-silent-defaults and hash-evidence
guarantees.

## Decision

Per the human ruling of 2026-06-11 (plan §2, accepted as proposed):

- Adopt the **wasm32 build of `core/model_operations/operation_applier`** as
  the **sole browser-mode operation engine** in every environment without a
  Tauri backend.
- **Retire the TypeScript mirror's validation/apply logic** at plan tranche T4
  — deletion by replacement, not amputation: the Rust crate compiled to
  WebAssembly becomes the engine everywhere before the mirror logic is
  removed.
- TypeScript retains only what must remain TypeScript: the **React UI, intent
  builders, and a thin routing adapter** (Tauri present → `invoke`
  authoritative path, unchanged; otherwise → the wasm module).
- The browser evidence loop (Vitest, Playwright, agent smoke) is preserved
  throughout the transition and afterward exercises the shipped engine rather
  than a copy.

## Alternatives Considered

All presented to and declined by the human project authority in the
2026-06-11 ruling (plan §1 and §2):

1. **Keep dual engines plus a permanent cross-engine contract test** — safe
   but inefficient as a steady state; Phase B unit-aware work compounds the
   dual-maintenance cost.
2. **Delete browser-mode editing outright** — guts the GUI test layer and the
   agent verification loop; forces all evidence onto packaged-native
   automation that does not exist and is weak on macOS.
3. **Local Rust dev-sidecar over HTTP** — adds a process and a network surface
   against the local-first/no-daemon posture.
4. **Deferral until T1 contract-corpus evidence** — declined; the scheduled R2
   and Phase B work fronts multiply the duplicated surface while the decision
   waits (plan §1, "why resolve now").

## Consequences

Traceable to plan §3 (T3/T4 scope) and §6:

- **Toolchain prerequisite:** the `wasm32-unknown-unknown` Rust target and
  wasm-bindgen tooling with **pinned versions** become build prerequisites.
  Prerequisites are documented; build scripts fail explicitly when tooling is
  absent — no silent fallback.
- **A wasm build step enters the frontend loop:** an npm script produces the
  wasm artifact for the desktop app; generated artifacts are not committed.
- **Debugging posture:** browser-mode engine issues are debugged through the
  engine's structured diagnostics rather than by stepping through native
  TypeScript engine logic.
- **Bundle size / startup impact:** mitigated by lazy engine init; measured
  against the existing ~577 kB index chunk and recorded in T4 evidence.
- **Loading shim obligation:** an explicit diagnostic when the wasm asset is
  absent; never a silent fallback.

## Affected Packages

Per the plan §3 write targets and run-record fan-out: PKG-16 (DEL-16-02,
DEL-16-03), PKG-00 (DEL-00-01 — this surface; DEL-00-02; DEL-00-08), PKG-07
(DEL-07-02), and PKG-10 (DEL-10-03, T1 rider only).

## Reconsideration Triggers

Per plan §6 risks and stop rules:

- **wasm loading proves intractable in the test environment** (Vitest/jsdom):
  stop and escalate to human ruling. Reintroducing a TypeScript engine as a
  workaround is not an authorized response — that would recreate the
  divergence this decision removes.
- **Material bundle-size or startup regression** beyond what lazy engine init
  mitigates.
- **Toolchain availability drift** — the wasm32 target or pinned bindgen
  tooling becoming unavailable or unpinnable.

## Source References

- `plans/PLAN_2026-06-11_operation_seam_unification.md` §1 (problem and
  alternatives), §2 (ruling record and transcription obligations), §3
  (tranche scopes), §6 (risks and stop rules).
- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, row `DEC-020`
  (transcribed ruling).
- `execution/_Coordination/_DECISIONS/_REGISTER.md`, row `D-13` (RULED).
- `core/model_operations/operation_applier/src/lib.rs` and
  `apps/desktop/src/services/operationService.ts` — the two engine
  implementations as of the ruling date.
- DEL-00-01 packet (`Datasheet.md`, `Specification.md`) — ADR surface
  obligations REQ-01-01..REQ-01-05.

## Boundary Note

This ADR records a software development decision only. It is not a
release-readiness, professional-approval, certification, sealing,
authentication, or engineering-code-compliance claim. It contains no
protected standards content and no proprietary data. Semantics of the
operation seam are preserved — this decision changes *where* validation runs,
never *what* it accepts (plan §5).
