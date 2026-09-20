# B3 continuation integration notes

Working record, not a frozen return. WORKING_ITEMS B3-CODEX, GPT-6 Astra/high,
parent ROOT; children use Sol/high and fresh TASK contexts. Actual dispatch and
capacity retry are retained in `_run_records/DISPATCH.json`. Root owns independent
review, full candidate sweep, PR and final merge. No B3A/B3B or canvas lane work started.

Entry was clean at `a9be4aaefa432e1eed56e87482391e90958cb087`. A clean merge of
`origin/main` `c459a0fa15e8d33613b6b95f529b5d369c707874` produced
`a63607e5c6d805f364c969eb6963f768baefb300`, preserving C2 without conflict.
The accepted source-qualified continuation package is at commit `c805bcc1a`,
`instances/ROOT/CONTINUATION_2026-09-19_CODEX/OWNER_APPROVAL.md` and its cited
`DECISION_AND_STRATEGY.md`. B3-FLOORS remains historical inventory; padding is rejected.

## Changes under integration

- D1 retains the collapsed strip overlay. One numeric presentation inset moves DOM
  triad/scale/readout and the painted orientation frame; renderer construction,
  main drawing dimensions, camera and picking are preserved. The named orientation
  witness changes placement only, retaining crop/visibility/passive-pointer checks.
- The existing routing panel portals into a stable inspector container. Its own
  state and handlers remain in PipeViewport. Arming borrows the inspector; disarming
  restores the earlier collapse state unless a deliberate inspector command has
  superseded that automatic restoration. Explicit destination focus wins.
- D2 borrows the inspector's width from the stored inspector-closed table split,
  down to 320 px, then from the canvas down to 220 px. Closing restores the split.
  The canvas moves horizontally when the inspector opens; it is not an invariant
  screen rectangle. No persisted model/schema change is involved.
- D3 restores drawn-canvas measurement, unconditional default-state 35% width/height,
  canvas-greater-than-inspector and overflow floors. Only wide Both-view dominance
  over the table becomes the ruled allocation check. Model/narrow dominance stays.
- The Both splitter's 24 px target ends at the canvas boundary, on the table side.
  Expanded narrow drawers enter flow, have a visible splitter, and retain requested
  180–600 px preferences. Actual drawer space is capped by the existing measured
  chrome reserve so the drawn-canvas 200 px floor survives short windows.
- Native menus receive stage/view/theme/density, enabled/checked state and project
  identity through one presentation-state command. Actions use the existing session
  sink. Native Cmd-1/2/3/I owns those accelerators; the webview shortcut path is disabled
  in Tauri. The native minimum becomes 1280×800 while browser 1024 cases remain.
  Busy-project File gating and unsaved title marker are still B3B/B3A.

## Inherited B3 changes in the complete diff

The full slice includes the existing shell, not just this continuation. The four
verbatim returns under `../B-SHELL/returns/B3*` retain controls, hooks and test-move
inventories. In particular: the four stages/three views replace the old dock;
mounted panels and session view memory persist; the Both inspector initially closes
and has fixed widths; Review has Table only; status chips follow the recorded run
policy and Historical lights none; Solve proof moved to Evidence, project controls
to Project, and boundaries to About; unregistered raw mechanics values leave the
bar (the adopted fallback is a separately scheduled follow-up). Appearance disclosure
and Select are temporary owner-adopted toolbar departures. The old 30/40 px collapsed
rails and resizable inspector have no equivalent in this design. Box16 retains its
existing 794×559 setup witness and all frozen gesture endpoints.

## Checks and limits

WASM rebuilt successfully. Focused manager shell/session tests passed 33/33.
Initial full unit run: 1381 passed, 3 failed. Two failures show automatic routing
restoration overriding explicit property navigation/focus, requiring repair. The
third expects the old narrow node-authoring inspector-closed state; the adopted
routing-home change requires an explicitly named assertion amendment, preserving
focus and mounted-control checks. The failing raw output remains `_run_records/full-unit.log`.

Harness self-check passes with the established mise Python 3.13 runtime; default
system Python lacked PyYAML and is recorded as an operational failure. Harness
pytest: 379 passed. Full source/dist Playwright, final unit/picking checks and final
native witness linkage remain pending while this record is working.

Contrast remains deferred for toolbar/view segments, stage rail/captions, tabs,
drawer chevron, splitters, inspector routing controls/close, agent strip, status
chips/popovers, project page controls, menu entries and palette view/group commands.
Other D-68 obligations remain in scope; no colour tuning was performed. B5's full
agent-column/user-split slide-over transitions remain deferred. Product acceptance,
D-72 qualification, release and practitioner usability are not claimed.

## Subsequent verification and repairs

- Recoverable product/unit checkpoint `8f9c2d071` is explicitly incomplete.
- The restore-vs-explicit-navigation repair passed focused regression; the full
  unit suite then passed 87 files / 1385 tests. Raw output is
  `_run_records/full-unit-after-focus-repair.log`.
- Browser navigation subsequently found a separate timing defect: toolkit focus
  ran while the inspector was still hidden, before automatic opening committed.
  The routing toolkit handler now opens it in the same event as the focus request.
  Browser backcheck is required; jsdom does not enforce visibility when focusing.
- The new clearance scenario initially chose a label outside the fixture's
  visible label budget. Its setup now selects a known typed target; no clearance
  or protected assertion was weakened. Both projects then passed pane/drawing
  clearance with active measurement; pointer hit ownership is also being checked.
- Native AppKit check items flip themselves before their action callback. Native
  now restores the last React-owned checked state before dispatch, so reselecting
  the same value cannot leave a stale unchecked item. Actual final native reselect
  witness and affected Rust/build checks remain required.
- Preliminary native New Blank persisted a new disposable project row. Earlier
  draft wording calling it session-only was corrected; the exact created ID and
  truthful store use are in the native evidence. No destructive cleanup is planned.

## ROOT engineering disposition: history accelerator ownership

ROOT directed during integration: Cmd/Ctrl-Z and Shift-Cmd/Ctrl-Z belong to the
existing webview key handler in both runtimes, with editable-target and
`defaultPrevented` guards, and call the same `edit.undo` / `edit.redo` command sink.
Custom AppKit history accelerators remain unassigned so they do not preempt text
undo. Native menu pointer Undo/Redo remains synchronized; Cmd-1/2/3/I stays native
owned in Tauri. This is an explicit ownership exception, not an unimplemented
native-history gap. The local engineering decision adds the specification's
missing accelerators without changing the applier/history route. It is reversible
by removing the Z branch and its toolbar hint. Actual final Tauri checks must show
one model undo/redo with model focus and normal text undo without model/history
change with a text field focused; an observed host receipt failure returns to ROOT.

## Browser hit ownership backchecks

The expanded drawer's old resize hit target covered the measurement readout.
The target now occupies a reserved 24 px header area inside the same drawer allocation.
After that repair, collapsed-state readout geometry cleared the strip but the
canvas still owned its center because the frame had z-index 1. The positioned
readout now has z-index 2 inside the existing canvas-pane stacking context.
Both source projects passed all four readout/painted-frame clearance states with
actual topmost-owner assertions. These fixes preserve pane and drawing dimensions.
Raw before/after witnesses are under `_run_records/shell/clearance-*`.

## Independent review and native diagnosis

Independent review's three accessibility findings were repaired in `f5bc01f8b`:
reason clipping, covered-stage Tab/AX exposure, and wide/portalled inspector Escape.
Keyboard Close focus return was completed in `74e69a269`; actual browser follow-up
18/18 passes with no manual refocus. ROOT's independent backcheck closed the three
original findings. Full source on earlier `26a7478b5` passed 386 with 20 existing
skips; that is not final repaired-source coverage.

Actual f5 native testing then found missing text Undo and select-popup cancellation
ownership. Isolated f5 A/B probes show predefined native Undo/Redo responders restore
text history without diverting neutral model shortcuts, including populated text
undo/redo stacks. ROOT adopted the exact two-line responder addition; production
keeps explicit model actions and editable-aware shortcut guards. Native APIs measured
minimum 1280×800 logical, while DOM content height differs because of host chrome;
this is diagnostic measurement, not final uninstrumented acceptance or D-72 conformity.

An event-only observed-popup prototype C was rejected: same-value Return and pointer
commits delivered no DOM commit event and incorrectly consumed the next Escape.
Passing synthetic tests did not establish host correctness. Exact files/patches/logs
remain under `_run_records/select-popup-repair`; helper and consumer changes were
removed from production. A single public AppKit notification probe is authorized to
seek an actual distinguishing signal; no polling, timing guess, blanket suppression,
private API or production diagnostic surface has been introduced.


The public AppKit probe distinguished both same-value commit routes from cancellation:
end-tracking preceded will/didSendAction for Return and pointer commit; Escape ended
with key code 53 and no action; outside cancellation had pointer metadata and no
action. This is diagnostic feasibility only. The original post-popup Escape keydown
failure did not reproduce, and async frontend delivery ordering is not established.
No production guard is present. Probe PID 19891 was verified stopped and UI released.
The adopted predefined responders passed Rust 101/101; raw output is retained under
`_run_records/native/responders-production-cargo.log`. The f5 full unit run passed
87 files / 1391 tests; it does not qualify subsequent source bytes.


ROOT selected one bounded correlated native-popup prototype in isolation before
production adoption. It uses a main-thread native snapshot/barrier at ambiguous
Escape, genuinely observed popup generation, shared global/narrow cancellation
decision, and input/focus/view generation guards for deferred actions. Tests must
cover reordered/late responses, duplicate consumers, quick input and keyup-only
cancellation as well as silent commits. No fixed delays or async-push dependency.
If correlation fails, stop for a concrete interaction disposition. Fresh Astra/low
TASK owns that isolated work; no production bridge is authorized by this choice.

Post-probe practitioner self-check used explicit mise Python 3.13 and exited 0;
raw output is `_run_records/harness-self-check-post-probes.log` (no BLOCK findings;
INFO14, NOT_APPLICABLE1, REVIEW4, WARN112). This structural check does not close
native or product-verification gates.


The correlated prototype established a live Space-open binding, but stopped on the
first same-value Return → Escape history: menu generation/action matched while a
process-global NSWindowDidResignKey observer advanced its window epoch during popup
lifecycle. That overbroad epoch invalidated the deferred close and left the inspector
open. This is a specific observer-scoping defect, not proof that native menu generation
or action is unavailable. Pointer opening separately lacked a DOM :open observation;
its keyup-only cancellation pass is not the original keydown-failure regression.
Remaining native matrix scenarios are unexecuted, not passed. No production bridge
was adopted; further prototype correction needs ROOT disposition.


ROOT review candidate `5ef9de29179a9b814da81d17c2774663404ac93d` combines product
matching `2882acab9`, ROOT records `7d5603ece`, and current main `485051eac`.
ROOT dispatched independent two-line responder code backcheck. ROOT explicitly
coordinates a separate structure-only headless browser review on reserved5183/5184
through the run lock while isolated native prototype retains foreground/CUA.
Manager confirms no competing E2E/server. This is an explicit resource arrangement;
no foreground/native sharing or D-72 measurement is authorized by it.


ROOT rejected production adoption of the partially successful correlated popup
prototype and chose compact web-controlled selectors for the actual B3 inspector/
routing consumers. New Astra/low TASK supplied component and15 focused tests; manager
integrates exact select sites, optional shared-control opt-in, styles and test
navigation. A scope context cancels transient popups on project/stage/view changes
without remounting preserved drafts or canvas. Prototype native code remains absent.

Uninstrumented2882 actual native witness passed model keyboard/pointer once-only
history, populated text-stack separation, native same-value Both, New/Open/title and
keyboard page-close focus. First CmdZ immediately after typing was a no-op in two
child histories; subsequent text undo/redo/exhaustion worked. ROOT is independently
checking native text transaction/autocorrection layers. No new Undo code is authorized
from the no-op alone. The original Sol/high native TASK is complete; subsequent
new native assignments follow Astra/low policy. ROOT owns PID30560/UI and the bundle
must not be rebuilt during its inspection.

Independent fidelity reproduction found successful New Blank retains the previous
stage/view. ROOT authorized reset to Model/Both with coherent transient shell state,
preserving failed/superseded-create behavior and unrelated preferences. The repair
instruction is sealed before edits; no B3B save/busy or deferred HUD work is added.


Compact/reset product checkpoint81db7c62e was frozen after22/22 browser cases and
focused integration repairs. Upstream main21175b5d merged cleanly as9d0b717a; aggregate
fan-in includes Runtime and App dependency files, no changed Piping bytes or Piping
Runtime adoption. Full unit on9d0b passed88files/1410tests. App-only uninstrumented
build passed, binaryd84cd65a5364eadea449269a7b44b4e18436a6b60e445f14c4bab01d8d24801c,
manifest8e2db1660a181d9b25e5b915f85b2c311b93f20a1cd523c866038b496196ab4a.
Fresh Astra/low final native TASK explicitly covers text REDO availability during
neutral model shortcuts and custom native Edit model actions, not just toolbar.
It fresh-launched PID36554 but CUA reported host lock before any UI input; ROOT
handles owner unlock. This is not an approval-review rejection or completed witness.

The first compact browser test incorrectly requested alternate unit mm, unavailable
in browser metadata-only mode by unitCatalogService contract. Retained initial run:
9pass/1failed/1interrupted/11notrun, exit130 after bounded manager stop. Repaired test
uses an actual multi-option property field for changed-value browser coverage; actual
Tauri will test alternate units. No catalog fallback or data injection. All22 cases
then passed. Initial full unit retained76 legacy control-navigation failures; adapters
now click rendered options and keep exact value/model assertions. One roundtrip test
also interacted behind a Project page; explicit close/edit/reopen navigation fixed it.
Only affected control interactions changed; unrelated native select/input routes stay.


Independent review found compact fallback commits could invent Anchor for missing/
null/unsupported Support family, and an index-based pending selection could retarget
under changing options. Two remaining Shared section e2e assertions also still assumed
HTMLSelectElement.value. The actual support consumer reproduced6/6 passive-dismissal
failures before repair. Component worker's added baseline reproduced11 failures,
then an additional manager event-time pointer regression reproduced Charlie→Delta.

Checkpoint24f5d9db0 repairs all three findings: opening fallback is provisional, commit
requires an explicit choice, pending identity is a value, and an opening controlled-
value/semantic-option basis is rechecked at commit. Pure reorder preserves identity;
semantic option or canonical-value changes cancel without commit or focus theft.
Rendered pointer handlers carry their option value, not a stale index. Component35
and representative consumer/engine/guard checks total75/75pass; typecheck passes.
Shared section keeps its empty-string oracle through data-value. Browser/native and
independent backchecks remain pending, source frozen for those checks.

9d0b native witness passed actual text REDO survival while neutral model shortcuts
changed3→2→3 and pointer custom NATIVE Edit model Undo/Redo also3→2→3. New Blank
reset/page-close/shortcut observations passed. This remains explicitly9d0b, with
compact qualification blocked by source findings above. ROOT currently uses unchanged
PID36554/d84cd65a for an Accessibility Inspector window-size witness; no rebuild
until release. Exact final window dimension claims await that evidence or its limit.


Independent code and short negative-case UI backchecks passed24f5; all3 review P2s
closed, no new issue reported. ROOT's prior full fidelity/F1 evidence carries for
unchanged surfaces. Manager75focused and1436full unit pass,8focused browser cases
pass. Native24f5 app-only build passes; binary55b95a087816c0340adae31758164b51c21d2a9910b46d9d8f0873817b2df1b7,
manifestbe76ef0b64d32bf76d09b2b48c4a05d9a22bc31c3709ddcb38c6b83c26905a65.
Old36554 verified terminated before rebuild; same Astra/low bounded assignment
fresh-launched40910 for affected native evidence, preserving prior N1 proof.
ROOT explicitly authorized full source/dist lanes concurrently with this scoped
native pass after its headless/browser lock release. Source lane now running on
frozen24f5; no source writes during execution. Dist follows sequentially.

ROOT's read-only Accessibility Inspector route did not acquire an element through
available CUA actions; tool was quit and absence verified. Its resampled1229×768
JPEG is not logical geometry. ROOT compared native host source/config from f5 to
current: only two predefined Undo/Redo additions, unchanged window initializer and
minimum1280×800. Exact prior public-Rust native measurement and32px DOM delta remain
separate from this unsuccessful final inspection route; no800CSS/D-72 claim.


Final source5045 passed410 with20 unchanged existing skips (exit0,11.7min).
Full dist initially passed51/failed2: readiness tests attempted focus on Review
behind the open inert Solve page. One dist-only keyboard Close action repaired the
user journey without product/helper/oracle changes; focused2/2 then finald375 full
53/53 pass (exit0,3.2min). Product remains24f5; the source suite's entire inputs
remain5045 because the only later path is an explicitly ignored dist spec. ROOT
backchecked the final four test-only lines; no product source changes remain.

ROOT affected native look independently passed missing-family provisional highlight,
Tab preservation and first/second Escape on40910/55b95a. Manager then identity-verified
and stopped that owned process at ROOT request; subsequent ps absent. Native/GPU is
quiescent for ROOT clean sweep. All raw native evidence remains immutable. No B3A,
B3B or canvas implementation started. Final handoff contains manager evidence and
complete inherited87-path product/test/historical inventory; ROOT owns final checks,
PR/CI/merge and shared records.


At final staging, repository ignore rules excluded17 trace.zip files already bound
in the packet manifest. ROOT explicitly clarified the original no-force wording:
ordinary git add -f for exactly those required in-scope evidence files is reversible
index staging, not history/ownership force. The sealed original brief is unchanged;
verbatim clarification and exact paths/hashes are retained in
_run_records/TRACE_STAGING_CLARIFICATION.json. No ignore rule, history, binary/cache
or oversized-original exception was made. Index byte identity is verified before
handoff commit.
