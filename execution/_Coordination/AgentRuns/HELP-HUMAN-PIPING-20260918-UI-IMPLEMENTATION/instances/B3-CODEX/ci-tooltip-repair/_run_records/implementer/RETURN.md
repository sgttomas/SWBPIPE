# B3 tooltip/Close bounded repair return

TASK Type2 /root/b3_manager/ci_tooltip_repair, delegated-harness-native child of
/root/b3_manager; supplied Astra/low. No delegation, Git mutation, native use,
controller/model/CI change or acceptance claim. Explicit workdir throughout:
`/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3` (desktop child for commands).
Host unrestricted; paths and resource limits were instruction boundaries.
Context origins/hashes: context.json. Basis:002dff0f244976f98b36517d920b3761f6f88704.

## Diagnosis and repair

High-confidence causal chain: Agent reason is an absolute positioned child of a
focused/hovered anchor, with z-index70, previously placed leftward at top0. This
puts it across the page header Close. In desktop reproduction Close was
x1316.140625,y82.5,w71.859375,h32 and elementFromPoint at its center was
#agent-strip-reason. Focus kept the reason open; moving to Close hovered the
reason itself and preserved the anchor hover, so ordinary pointer clicks could
not reach Close. Both pinned Chromium profiles reproduced this, independent of
the incidental70-Tab endpoint in the hosted test. Hosted raw archive and raw log
SHA256 were verified against ROOT PR825_CI_FAILURE/MANIFEST.json; interception
lines were inspected. No competing controller/state hypothesis was needed.

Minimal geometry fix first: Agent reason now appears below its button, right
aligned inside the viewport. A transparent space-2 bridge retains hover while
crossing the visual gap. All existing reason hit-ownership/clipping assertions
remain unchanged. Geometry-only6/6 pass demonstrates this alone repairs Close.

Dismissal semantics: DisabledReason keeps its describedby text mounted, and
hides only its visual tooltip on an unconsumed Escape when it has visible client
rects. Document bubble handlers run after child handlers and before window shell
handlers. An event-specific WeakSet lets all simultaneously visible reasons
dismiss for the same reason-consumed Escape; preexisting defaultPrevented from a
child remains untouched. New focus/pointer entry reopens a dismissed reason.
Hidden reasons do not consume Escape. Native title text is unchanged.

## Checks and evidence

All browser runs used existing with_e2e_lock.sh, PLAYWRIGHT_WORKERS=1 and explicitly
resolved @playwright/test chromium.executablePath (chromium-1223, Google Chrome for
Testing148.0.7778.96). No auto-selected installed Chrome, force clicks, pointer
bypass, timeout change, skip, or weakened existing assertion.

- before.log and before-artifacts: new actual focused+hovered Agent/Close test,
  production unchanged, both profiles FAILED at ordinary close.click120s timeout.
  Root-level chromium-*-agent-reason-close-hit.json records pointer ownership.
- geometry-only/result.log: new regression + original clipping/topmost + original
  covered-page70Tab test, both profiles6/6 PASS16.0s; geometry-only.diff and CSS hash.
- escape-before/result.log:4/4 FAILED before component change: Escape moved focus
  to shell/page and left the simultaneously hovered rail reason visible.
- final-focused/result.log: five selected scenarios, both profiles10/10 PASS20.8s.
- final-description/result.log: all3 new tests plus explicit accessible description
  after visual dismissal, both profiles6/6 PASS9.2s. Final source matches this run.
- git diff --check PASS. final-source.diff and manifest.json bind final files.

Replay from desktop directory: `sh ../../execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B3-CODEX/ci-tooltip-repair/_run_records/implementer/run-focused.sh <new-label> e2e/b3-accessibility.spec.ts --grep '<selection>'`.
Each recorded run's arguments.txt preserves its precise selection. Initial before
command used identical pinned-browser/lock/one-worker setup, selection
`--grep 'Agent reason preserves'`, output before-artifacts, stdout/stderr before.log,
and B3_A11Y_EVIDENCE_DIR=implementer. Native version invocation was recorded in
browser-version.txt; later runs record executable.txt and version.txt.

New test names:
1. Agent reason preserves hover and lets focused page Close receive the pointer
2. disabled reasons own only visible unconsumed Escape and reset on reentry
3. simultaneous hovered and focused reasons dismiss before the page

Affected existing named scenarios: disabled rail and Agent reasons escape clipping
under keyboard focus; covered pages exclude retained stage controls from Tab and
accessibility. Existing inspector/toolkit/page/compact Escape cases are retained
and left to manager's full B3 accessibility + affected source/dist selection.

## Limits and resource return

Manager owns TypeScript/build/unit, broader affected source/dist, actual Tauri
witness, review/integration and CI. No native equivalence claim here. Native witness
should open Libraries, focus Agent (tooltip readable below button), hover reason,
click Close, then verify visible reason Escape leaves page open and a second Escape
closes it. Both ports5174/5175 had no listeners after final run; lock released and
browser slot returned to manager. Only the three authorized production/test paths
changed; remaining writes are this evidence folder. No further source edits planned.
