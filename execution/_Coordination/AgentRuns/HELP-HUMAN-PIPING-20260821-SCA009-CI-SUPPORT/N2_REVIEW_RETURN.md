# N2 mandatory independent code review return

- Reviewer: fresh read-only `TASK + software-code-review`
- Reviewed: 100% of the N2 diff in `operation_applier/src/lib.rs`, `componentIntent.ts`, `PipeViewport.tsx`, `PropertyInspector.tsx`, and `App.test.tsx`; `apps/desktop/src/types.ts` confirmed unchanged.
- Verdict: **FAIL**
- Scope validation: PASS
- `git diff --check`: PASS
- `check_claimed_model_hash`: preserved before resolver execution
- Mutation route: structured-operation engine only

## Actionable findings

1. **P1 — validated references differ from persisted references.** The resolver trims component `id`, `node`, and `bend_pipe_ref` for validation but returns the original payload object. Whitespace-padded refs can therefore pass validation and be persisted in a form downstream consumers cannot resolve. The resolver must reject noncanonical strings or reconstruct a normalized record, with a regression test.
2. **P1 — viewport and Inspector can queue duplicate component targets.** Draft validity checks only accepted model components, not queued `insert_component_symbol` targets, and the two surfaces do not synchronize draft reservation when sibling-surface queued intents change. Include queued targets in validity/reservation or synchronize drafts, with a cross-surface regression test.

No remediation was started because the owner stop rule became active after the N4 registered-check failure.
