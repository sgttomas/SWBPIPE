# B3 CI tooltip/Close repair return

WORKING_ITEMS Astra/high under ROOT HELP_HUMAN. Bounded resumed B3 repair only;
no B3A/B3B/I1 or Runtime work. Product candidate3861b614362d8da925e148d7506ecc9902c7f808,
base reviewed PR825 head002dff0f244976f98b36517d920b3761f6f88704. Clean c613 lane
fast-forwarded to that PR before implementation. ROOT owns independent review,
CI strategy, clean sweep, publication and merge. No product/engineering acceptance
or performance claim. Historical B3 packet/seals were not rewritten.

## Diagnosis and repair

Hosted pinned-Chromium run35496887667 failed2 cases after408pass/20existing skips:
Agent reason intercepted Close after covered-page keyboard traversal. Raw compressed
log and manifest hashes were verified at the supplied ROOT origin. New deterministic
regression keeps Agent focused and hovers its tooltip before ordinary Close click.
Both profiles failed at normal120s timeout; hit records show Close center owned by
agent-strip-reason. Focus-within/ancestor-hover persistence plus top/leftward placement
caused the obstruction, not a slow handler or model operation.

Agent reason now sits below its trigger, right aligned. A transparent gap bridge
preserves hoverability. Geometry-only6/6 passed, including unchanged original covered-
page traversal and tooltip clipping/topmost assertions, establishing causal repair.
No pointer-events:none, force-click, cursor bypass, timeout/skip/oracle change.

DisabledReason adds scoped Escape dismissal while keeping aria-describedby text
mounted. Document bubble handling follows child handlers and precedes window shell
handling; only a rendered visible reason can consume Escape. A WeakSet permits all
simultaneously hovered/focused reasons to dismiss for the same reason-owned event;
unrelated defaultPrevented child events are respected. Fresh anchor pointer/focus
entry restores reveal. Hidden reasons leave normal page/inspector Escape untouched.
No controller/shared event/model/native Rust/package/CI source changed.

Exact changed paths, relative to apps/desktop:
- src/styles.css: reason presentation rules only.
- src/features/workspace/shell/DisabledReason.tsx: local visual dismissal lifecycle.
- e2e/b3-accessibility.spec.ts:3 meaningful regressions; all existing assertions retained.

## Checked frozen bytes

| Check | Result |
|---|---|
| New Close regression before repair |2/2 failed real click at120s; traces/hit JSON retained |
| Geometry-only comparison |6/6 passed16.0s |
| Escape ownership before repair |4/4 failed; retained separately |
| Implementer final focused |10/10 passed20.8s; final accessible-description/new-regression pass6/6,9.2s |
| App.shell + workspaceSession.shell units |44/44 passed11.51s |
| Complete B3 accessibility/workspace-layout/gui workflow selection |60/60 passed2.5min, both configured profiles |
| Four adjacent ui-foundation interaction cases |8/8 passed18.5s, both profiles |
| Affected production-dist style/overlay selection |17/17 passed50.3s |
| TypeScript/Vite + actual app-only build |exit0; npm run tauri -- build --debug --bundles app |
| Practitioner self-check |exit0/no BLOCK; existing WARN/REVIEW facts retained |
| Actual Tauri affected interaction |focused reason/ordinary Close, first reason Escape/next page Escape and reentry passed |

Every browser run used installed Playwright Chromium1223, Chrome for Testing
148.0.7778.96, explicit executable override and one worker under existing run lock.
Arguments/version/log hashes and exact named selection are in _run_records/CHECKS.json
and per-run folders. No full source/dist suite repeated; ROOT owns required final
sweep and CI scope strategy. Linux hosted pass remains owed on integrated candidate.

Source selection: all b3-accessibility.spec.ts, workspace-layout.spec.ts,
gui-workflow-validation.spec.ts. Adjacent source grep in ui-foundation.spec.ts:
`keyboard splitters stay named|decorative viewport overlays|workspace Escape event ownership consumed (palette|drawer)`.
Dist grep in ui-foundation-dist.spec.ts:
`preflight|rail-clearance|outer overlays own actual|below 1280 px|production appearance`.

## Native witness and limitation

Uninstrumented3861 build binary SHA256
e9eda3cca4e626dd00cfee62279113d0d5161a751ba40cd58cea091cd23ca5f9.
FreshPID80466/start Sun Sep20 02:31:31 2026; explicit path/source/dist hashes in
NATIVE_BUILD.json/NATIVE_LAUNCH.json. CUA-only actions retained in NATIVE_ACTIONS.md,
raw AX and screenshots. Initial known synthetic project:invented-loop-01,27entities,
Both/Inspector closed/Select/empty history. No create/open/save/delete/apply/model
input or preferences mutation. Libraries open→8Tabs reached Agent; visible reason
was below trigger and clear of Close. Ordinary CUA Close click closed page and
returned focus Libraries, repeated twice. First Escape removed visual reason only,
Agent retained focus/page open/accessible help; second Escape closed page and focused
Select. ROOT independently repeated those affected routes successfully on same build.

Native CUA exposes no hover primitive. Attempts to click tooltip coordinate/text
dropped focus and hid it; those are retained unsuccessful hover observations, never
claimed as hover proof. Pinned browser tests establish actual hover/gap persistence,
topmost ownership and pointer Close; native establishes focus/Close/Escape behavior.
No screen resampling dimensions or native-class browser substitution claimed.

After ROOT own look, manager verified exact80466 path/start/binary, terminated only
that owned process and verified ps absence. Browser runs ended and ports5174/5175
were released. ROOT receives a quiescent candidate for clean sweep.

## Attribution and handoff

Fresh TASK ci_tooltip_repair ran Astra/low, no delegation/Git mutation. Its sealed
brief, actual source origins, context, verbatim return and manifest are retained;
manager verified child hashes. Written scope on unrestricted host is not sandbox
isolation. Manager performed integration, broader focused checks and native witness.
This is author/context independent from ROOT's review, not model diversity.

ROOT authorized ordinary git add -f for exactly6 manifest-bound ignored fail-before
trace.zip evidence files. No ignore/history force changes or extra artifacts are
included. Final evidence commit identity is supplied with the handoff. ROOT retains
independent review result, final sweep/actual-candidate CI and PR825 merge; this
assignment starts no next UI slice.
