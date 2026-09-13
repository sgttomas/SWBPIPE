PLEASE IMPLEMENT THIS PLAN:
# Agent 0 implementation plan: workflow integrity and contact initialization

## Summary

Deliver a reliable native modelling cycle—author, solve, inspect, edit, undo/redo, save, reopen and re-solve—alongside a bounded repair for contact-supported models that currently fail before contact equilibrium can be found.

The planning basis is verified `origin/main@b2c133d7`. Implementation will begin with fresh state validation and two explicit integration briefs. Preserve the original handoff, previous accepted repairs, and assessment history.

CAEPIPE export, harness integration and UI redesign remain deferred. Semantic equivalence applies to every control touched.

## Delegation and model allocation

Role determines responsibility; model and reasoning effort determine the resources assigned to it.

| Assignment | Role and reporting | Model / reasoning |
|---|---|---|
| Priorities, boundaries, engineering decisions and combined acceptance | HELP_HUMAN, Agent 0 | `gpt-6-astra` / high |
| Physics implementation ownership | WORKING_ITEMS, Type 1 → Agent 0 | `gpt-5.6-sol` / high |
| Product workflow implementation ownership | WORKING_ITEMS, Type 1 → Agent 0 | `gpt-5.6-sol` / high |
| Bounded source changes and focused tests | TASK, Type 2 → owning manager | `gpt-5.6-sol` / high |
| Difficult analytical questions and independent physics refutation | TASK, Type 2 → Agent 0 | `gpt-6-astra` / xhigh |
| Fresh review of the complete integrated diff | TASK, Type 2 → Agent 0 | `gpt-6-astra` / high |
| Native workflow verification | TASK, Type 2 → Agent 0 | `gpt-5.6-sol` / high |
| Scoped Git closeout and evidence packaging | TASK under the change workflow | `gpt-5.6-sol` / medium |

These are task-specific allocations using reasoning levels supported by [Astra](https://developers.openai.com/api/docs/models/gpt-6-astra) and [Sol](https://developers.openai.com/api/docs/models/gpt-5.6-sol).

Use the two implementation managers because each undertaking couples source changes, tests, integration and repair cycles. Direct Type 2 dispatch suits independently checkable outputs such as analytical verification, final review and native witnesses. Agent 0 performs alignment, live-state inspection, dependency resolution and cross-manager acceptance directly.

HELPS_HUMANS is reserved for subsequent work requiring substantial design: pressure formulation, connector mechanics and the professional 3D interaction target. It is unnecessary as another management layer for this first tranche.

Operational defaults:

- At most six active instances, including Agent 0; one product writer per workstream.
- Separate worktrees and explicit file ownership. The physics manager owns all product-physics changes; the workflow manager owns desktop integration.
- Serialize native builds, browser/native witnesses and final Git integration.
- Record supplied context, hashes, parentage, configured model/effort, scopes and returns. Type 2 instances do not delegate.
- After two unsuccessful repair cycles without new evidence, stop the repeated approach and assign fresh Astra diagnosis. Escalate to the user only when the decision exceeds existing authority.

## Implementation

### Physics workstream

Anchor the bounded integration in DEL-04-04, with explicit product-caller accommodations and PKG09 verification ownership.

1. **Define the eligible contact class.** Validated translational OneWay, LiftOff and Gap supports; normalized Active/Inactive states; valid clearances; unique contact DOFs; no overlap with base restraints. Preserve supported positive ground springs.
2. **Repair product admission.** Count eligible potential contact DOFs in the necessary restraint preflight, while leaving actual solvability to the solver. Admit both active and inactive seeds. An eligible singular preliminary linear solve proceeds to nonlinear analysis.
3. **Add one recovery attempt.** Only an exact singular error on the first nonlinear iteration, with at least one inactive contact, permits an all-active trial. Preserve authored signed gap displacements, loads, stiffness and solve mode.
4. **Continue normal active-set iteration.** Classify against the actual recovery seed, then permit ordinary contact release/re-engagement. Every recovered-path acceptance requires exact state stability and existing admissibility checks. Later singularity remains failure; no second rescue, artificial stiffness or subset enumeration.
5. **Preserve truthful evidence.** Count the successful recovery trial as completed iteration one. Emit recovery context through existing warning diagnostics. Publish linear-success, fallback and parity evidence only when that linear solution actually exists. Preserve the classifier’s existing tolerance meaning and the assembled solver’s stricter acceptance guard.

Implementation stays in the nonlinear integration crate, its localized product caller, and focused verification. Input/result DTOs, numerical policy and friction formulation remain unchanged.

### Product workflow workstream

Anchor ownership in PKG07, consuming existing PKG14 analysis and PKG16 operation contracts.

1. **Propagate native solve failures.** Native job-start or solve invocation errors remain errors. Browser fixtures are selected only when the native host is absent.
2. **Separate historical and current results.** Reopened saved results enter a transient `HistoricalRunContext`. They remain readable with integrity findings, but cannot drive current-model overlays, comparisons, rule checks or report readiness. A fresh successful solve establishes current results against its exact input basis.
3. **Preserve save compatibility.** Keep the existing native format. An unchanged open/save preserves saved historical result data; model edits and history transitions clear computed context. Do not invent a missing historical input manifest.
4. **Unify history guards.** Enforce the synchronous operation-busy guard inside undo/redo handlers and reflect it in menus, shortcuts and panels. Project replacement continues to invalidate delayed responses.
5. **Preserve shared semantics.** Human gestures and forms continue through existing typed intents/batches and the same Rust applier. Queries expose result designation consistently. Blocked/nonconverged outcomes retain diagnostics without presenting solved-only behavior.

No new modelling operation, transport API, persistence migration or cross-session undo system is introduced. The only new state interface is transient historical-result context; existing native and operation wire contracts remain compatible.

## Verification and acceptance

Freeze independent expectations before implementation.

- **Contact cases:** the two-node axial model with stiffness `100 N/m`, positive gaps `0.05/0.20 m` and tip load `10 N`. Expect initial reactions `(-15,+5) N`, then tip release and equilibrium displacements `(0.05,0.15) m`, reactions `(-10,0) N`. Cover reversed load, insufficient restraint, active/inactive seeds, support ordering, both solver modes, zero-reaction conventions and invalid recovery classes.
- **Iteration limits:** a stable recovered trial may converge with a one-iteration cap. A changing contact boundary must remain nonconverged even when the configured tolerance is at least one.
- **Native lifecycle:** author a synthetic 3.2 m cantilever at Y=2.4 m, with OD 0.168 m, wall 0.007 m, E 200 GPa, G 77 GPa, an anchored root and a 350 N tip load. Edit to 500 N; verify analytical displacement/reactions, exact undo/redo model hashes, attachment preservation, historical designation after reopen, and matching results after re-solving.
- **Semantic equivalence:** an equivalent synthetic agent-authored load intent, reviewed through the existing operation route, must produce the same model hash as the human edit.
- **Failure cases:** native IPC failure, busy history, duplicate apply, stale preview, late solve completion, cancellation, invalid units/targets, partial-batch failure and inconsistent saved evidence must not produce partial mutations or current-result claims.

Use existing publication-precision allowances for published numerical quantities: `0.5e-6 + 1e-10` in their declared units. Hash comparisons require exact equality.

Run focused Rust and desktop tests, browser/distribution workflows, file-backed native checks and a fresh GUI quit/reopen witness. Then obtain fresh independent review of 100% of the integrated diff, run the complete registered DEC-025 sweep, practitioner harness, self-check and receipt validation.

Integrate through one scoped PR. Merge under standing authority after required CI and review pass on the actual candidate. Update deliverable Remaining entries, immutable evidence and handoff; do not infer whole-project completion from tranche closure.

## Defaults and subsequent work

This allocation replaces the earlier blanket Sol/high rule for assignments covered by this plan. Routine engineering choices and Git actions use existing authority.

After this tranche, prepare separate work for whole-result schema compatibility, pressure formulation, objective connector mechanics, numerical robustness and end-to-end sparse execution. Serialize changes sharing the product-physics implementation.

The future professional 3D interface remains the functionality target. Its design and the eventual harness connection will consume the shared operation semantics preserved here.
