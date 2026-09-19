# Retained return — B2F-REVIEW (independent code review of slice B2F), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`. One further kind of substitution is made and nothing else is changed: the agent quoted path patterns it searched for, and the repository's path-anchor validator reads such a literal as a machine path wherever it appears, so ROOT writes the pattern it quoted as the private temporary folder pattern; the pattern it quoted as the user home pattern; the pattern it quoted as the home pattern; the pattern it quoted as the var folders pattern. The message exactly as the host stored it has SHA-256 `5965979f26f6cf87f0d27c8675f5b6913b9e638a880e19eeff953abf1cf7323e` (190 lines), by this script.

---

# B2F-REVIEW return

**Verdict: PASS — no actionable finding.** Slice B2F is, on my reading, exactly the three named behaviour repairs plus one deliberate non-change, each held by a test, and nothing else.

I am **Claude Opus 5** (`claude-opus-5[1m]`), TASK (Type 2), read-only, working alone, no delegation. I ran no state-changing git command and modified no file.

## Instruments verified

| Instrument | Expected SHA-256 | Read |
|---|---|---|
| Sealed brief `B2F-REVIEW_code_review.md` | `28c09b8c…6b073d7` | matches |
| Skill `.agents/skills/software-code-review/SKILL.md` | `06c27b1b…f95570a` | matches |

Worktree detached at `a98eb24f7`; product tree is the candidate's. Diff range reviewed at 100 %: `64f86e17f...36e79602e`.

---

## Check 1 — Result integrity (item 3)

### (b) The window, enumerated

This is the audit trail for my PASS. "Window" = every statement between the base's position for the three statements and the candidate's.

**`handleOpenProject`** — base position: before line 1506. New position: line 1525.

| # | Statement | Reads gate or either integrity cell? |
|---|---|---|
| 1 | `const request = ++projectRequest.current;` | no |
| 2 | `let epoch = requestEpochRef.current;` | no |
| 3 | `const stillCurrent = () => request === projectRequest.current && epoch === requestEpochRef.current;` | no |
| 4 | `setProjectBusy(true);` | no |
| 5 | `const opened = await openLocalProject(projectId);` **(await)** | no — service module, cannot reach the hook-local ref |
| 6 | `if (!stillCurrent()) return;` | no |
| 7 | `if (!opened) { setProjectMessage(…); setProjectOperation("open_missing"); return; }` | no |
| 8 | `const restoredHistory = await buildHistoricalRunContext(opened);` **(await)** | no — same reason |
| 9 | `if (!stillCurrent()) return;` | no |

**`handleCreateBlankProject`** — base position: before line 1440. New position: line 1464.

| # | Statement | Reads gate or either cell? |
|---|---|---|
| 1 | `const blankModel = buildBlankLocalModelDocument();` | no |
| 2 | `const request = ++projectRequest.current;` | no |
| 3 | `let epoch = requestEpochRef.current;` | no |
| 4 | `const stillCurrent = () => …;` | no |
| 5 | `setProjectBusy(true);` | no |
| 6 | `const blankModelHash = await computeModelHash(blankModel);` **(await)** | no |
| 7 | `const envelopeHash = await computeProjectEnvelopeHash({…});` **(await)** | no |
| 8 | `const created = await createLocalProject(blankModel, …);` **(await)** | no |
| 9 | `if (!stillCurrent()) return;` | no |
| 10 | `const createdSummary = { …created.summary, message: … };` | no |

**What else can run while those awaits are pending.** I enumerated every reader in the tree, not only in these two functions.

- **`ruleRevisionGate`** has exactly six mentions in the product (`workspaceSession.ts` 545, 742, 754, 1288, 1464, 1525) plus its construction in `resultsSessionState.ts:64`. It is a `useRef` that is **not** placed on the session object and does not appear in `App.tsx`, so no component and no service can reach it. Three mentions are writes (`invalidate()` in `handleRun`, in `clearComputedModelState`, and the moved call). The **only reader** is `handleRuleCheckAggregate`'s `stillCurrent` at 754. If `handleRun` or `clearComputedModelState` (undo/redo) runs during the window, each invalidates the gate itself, so an in-flight revision is dropped exactly as at the base — the move changes nothing on those paths.
- **`modelHashIntegrity` / `projectEnvelopeHashIntegrity`** are written only by the four project handlers (open 1526/1563, blank create 1465/1466, create 1356/1357, save 1600/1601) — no effect writes them. They are read only at render, by `App.tsx` (892/902/904) feeding `ProjectValidationPanel` and `ProjectStorageAuditPanel`. **No handler branches on either cell**, so nothing in the session can take a different code path because of the move. If another project handler starts during the window it takes a new request number, so this open/create can no longer commit — the cells' final state is then that handler's, as at the base.

**One intermediate-state difference, which is the repair's point.** During the pending window the panels now show the still-open project's integrity status instead of `open_verification_not_run_this_session`. The still-open project is the one still displayed (`commitModel` has not run), so the panel now describes the project it is actually showing. A user-triggered preflight export during the window (`buildProjectValidationPreflight`, `ProjectValidationPanel.tsx` 305/310) likewise records the true status rather than "not run". Nothing hash-bound or persisted captures either cell, so no stored artefact can take a different value.

### (a) Final state on a successful open or create

- **Successful open:** gate invalidated exactly once (base: once); `setModelHashIntegrity(null)` then `setModelHashIntegrity(deriveModelHashIntegrity(…))` at 1563; `setProjectEnvelopeHashIntegrity(null)` then `deriveProjectEnvelopeHashIntegrity(…)` at 1574. Lines 1561–1580 are untouched by the diff, so **the verified-match path reads exactly as before**, byte for byte. Same final value for every reader.
- **Successful blank create:** both cells nulled once and never re-set, as at the base. Same final value.
- The commit block 1525→1560 contains **no await**, so the whole commit (invalidate, both nulls, `advanceProjectSession`, `commitModel`, and the ~25 setters after it) lands in one synchronous run and one React batch. There is no render in which some of it is applied and some is not.

### (c) Can the in-flight rule-check revision land against the wrong basis? No.

`handleRuleCheckAggregate`'s `stillCurrent` (751–760) requires seven things at once. During the open's pending window, each still describes the **still-open** project, because every symbol that would change is written only inside `commitModel` / `advanceProjectSession`, which run after the commit point:

| Condition | Written where | State during the window |
|---|---|---|
| `ruleRevisionGate.isCurrent(revision)` | moved call, 1525 | unchanged |
| `modelRevision.current === capturedModelRevision` | `commitModelAfterSolveInvalidation`, and the `[model]` effect at 401 | unchanged |
| `solveRunGate.current() === capturedSolveGeneration` | `handleRun` / `commitModel` | unchanged |
| `currentSolvedResultRef.current === capturedResult` | `setResult` at 1542 | unchanged |
| `currentInputManifestRef.current === capturedManifest` | `setInputManifest` at 1547 | unchanged |
| `currentModel.current?.project.id === capturedResult.model_ref` | `commitModel` 438 | still the open project |
| `renderedBasisIsCurrent()` (`projectSessionGenerationRef`, `uiModelRevisionRef`) | `advanceProjectSession` 481, `commitModel` 436 | unchanged |

So the landing is always against the still-open project's own model, result and manifest — never the incoming one. Three cases:

- **Open commits:** invalidate at 1525 precedes `setAnalysisRun(null)` at 1543 with no await between, so a revision resolving after the commit is dropped (twice over: gate, and `projectSessionGeneration`). A revision resolving *before* the commit sets `analysisRun`, which 1543 then nulls — last write wins by enqueue order, so the new project starts with no analysis run. Correct either way.
- **Open finds nothing / fails / is superseded:** the revision lands against the project that is still open, resolving the base's inconsistency (`ruleCheckAggregate` set while `analysisRun` unrevised). This is the repair.
- **Nothing depended on the base's extra invalidations.** A failed open changes no model, result or manifest, so the invalidation it performed protected nothing; it was pure loss. The candidate invalidates on strictly the committing paths and is, on every other path, at least as conservative as the base.

### (d) Superseded open

`stillCurrent()` false returns at 1512 or 1521, and the catch returns at 1582 — all before 1525. The cells and the gate are untouched, which is right: nothing about the still-open project changed, so nothing about its recorded verification should change.

**Untouched, confirmed by reading the diff:** no `stillCurrent` condition, neither gate class (`solveGates.ts` not in the diff), `commitModelAfterSolveInvalidation`, `clearComputedModelState`, the Current/Historical designation, the solve-input basis, undo and redo, persistence formats.

## Check 2 — Item 2, the busy state

`handleListProjects` adds one entry guard and one `finally` ownership test; it reads `projectRequest.current` and never advances it, so **a pending save's `stillCurrent` is untouched** (confirmed: the diff adds a read, not a write, and `requestEpochRef` is not mentioned).

Interleavings I worked through:

| Interleaving | Outcome |
|---|---|
| list → request-numbered handler H; list settles first | list's guard `requestAtStart ≠ projectRequest.current` → no release; H releases in its own `finally`. Busy correctly held. |
| list → H; H settles first | H releases; list then does not release (already false). Not stuck. |
| H → list | list returns at the entry guard; nothing touched. |
| two lists | both capture the same `requestAtStart`; both release; ends false. |
| list during a superseded handler chain | only the latest request-numbered handler satisfies `request === projectRequest.current`, and it always reaches its `finally`. |
| handler throws | `catch` then `finally` → released under the same guard. |

**`projectBusy` cannot be left stuck true.** Every request-numbered handler reaches its `finally`; the *last* one started always matches `projectRequest.current` and releases. A list with no handler after it releases itself. The two handlers with an early `if (!model) return;` (`handleCreateProject`, `handleSaveProject`) return *before* `setProjectBusy(true)`, so they never acquire it.

**`projectBusy` is never released while a request-numbered handler that owns it is pending** — the `finally` guard is conservative in the safe direction (it may decline to release when it could, and a pending handler always releases later).

**The same-tick edge: not reachable by a user, and not worse than the base.** The list is reached two ways. (i) The *List local* button is `disabled={projectBusy}` (`App.tsx:336`), so a click can only originate in a render where `projectBusy` is false — the guard's value is the same value that enabled the button. (ii) The native menu goes through `runMenuCommandRef.current`, reassigned on **every** render (1965–1966), dispatched from a `window` CustomEvent — a separate task, by which time React has flushed the render that set `projectBusy` true. To hit the stale read you would need the CustomEvent dispatched synchronously inside the same task as `setProjectBusy(true)`; no user input path does that. Were it hit, the list would capture the handler's own request number and release busy early — i.e. it would reproduce the base defect exactly, never something new. The in-DOM menu already carries `disabled: projectBusy` on `file.list-local` (`App.tsx:1076`), so the new guard makes the handler agree with both existing surfaces rather than introducing a third behaviour.

## Check 3 — Item 1

`setOperationOutcomes({})` and `setOperationMessage(null)` sit at 1539–1540, directly after `setBatchMessage(null)` — the same slot the blank handler uses (1477–1479) and the same order `adoptNormalizedPersistenceModel` uses (1330–1334). They are inside the commit block, after `if (!stillCurrent()) return;` at 1521 and after the `!opened` return at 1513–1519, with no await between 1525 and 1560. **There is no path on which they clear the diagnostics of a project that stays open.**

## Check 4 — Item 4 left alone

I confirm the manager's and the child's reading: the missing `ruleRevisionGate` invalidation in `handleApplyIntent` is **inert**, and I find nothing against the base here. `commitModel` (1196) synchronously bumps `uiModelRevisionRef.current` (436) and, via `commitModelAfterSolveInvalidation`, `modelRevision.current`. An in-flight rule-check revision therefore fails `modelRevision.current === capturedModelRevision` *and* `renderedBasisIsCurrent()`'s `uiModelRevisionRef` test — two independent guards, either sufficient. `setResult(null)` at 1209 breaks `currentSolvedResultRef` as a third. Lines 1159–1219 contain no await, so nothing can interleave between the outcome's publication and `commitModel`. No finding against the base.

## Check 5 — The tests

Seven tests, one file. I ran `npx vitest run src/App.projectHandlers.test.tsx` from the desktop app: **7 passed / 7**. I ran `npx tsc -p tsconfig.json --noEmit`: **exit 0**. I ran nothing else and changed nothing.

| Test | What it pins (user-visible) | Why it must fail unrepaired |
|---|---|---|
| item 1, open clears operation diagnostics | the Issues-drawer toggle's count and the drawer's "Operation diagnostics" section | without 1539–1540 the invented diagnostic survives the open; count stays baseline+1 |
| item 2, save pending → native list refused | the five project buttons' `disabled` state, the project message, and `listCalls === 0` | unrepaired, the list runs, re-enables the controls and publishes "Listed 0 …" |
| item 2, list first → native save → list settles | the same buttons after the list settles | unrepaired, `setProjectBusy(false)` in `finally` fires unconditionally |
| item 3 ×3 (open finds nothing / open fails / blank create fails) | `integrity=mismatch_review_required` on the validation panel's model-hash line, plus the tree row for the still-open project | unrepaired, the entry nulling makes it read `open_verification_not_run_this_session` |
| item 3, in-flight rule-check revision | `run-audit-status` text going from not-`USER_RULE_FAILED` to `USER_RULE_FAILED` after the open finds nothing | unrepaired, the entry `invalidate()` makes `isCurrent(revision)` false and `setAnalysisRun` never runs |

I verified two by reading in detail — the in-flight-revision test against `handleRuleCheckAggregate`'s guard (above), and the second item-2 test against the `finally` guard, which is the only line that can make it pass.

The first item-2 test does double duty worth naming: `expect(lastProjectOperation()).toBe("save")` is a live oracle against **the repair the addendum forbade** (advancing `projectRequest`), since that repair would make the save's `stillCurrent` false and drop its result.

**No test pins appearance.** Assertions are `toHaveTextContent`, `toBeDisabled` / `toBeEnabled`, `toBeInTheDocument`, and `toBe`. The single class assertion, `not.toHaveClass("inactive")`, is inside the `openWorkspaceSection` helper and pins *that a section is open* — user-visible state, not a pixel, colour or computed style.

**Mocks behave as the real bridge does for the cases used.** I checked each against `projectService.ts`: `open_local_project` returns `LocalProjectEnvelope | null` (552) — the test uses an envelope, `null`, and a rejection, all three real; `list_local_projects` returns an array (568) — `[]`; `save_local_project` takes `{ request: {…} }` (597) and the test reads `args?.request` and echoes `saveRequest.model` back, which is the genuine unchanged-model save path (`responseModelChanged` false, so the integrity branch is correctly not exercised); `create_local_project` rejects. The invented diagnostic is labelled `source: "invented_test"`.

**Gaps.** (i) The manager's own: no test covers the pending window of its semantic change 4 — the window in which the still-open project's integrity status is now retained. My reading of that window is in check 1(b)/(c) above; it is reachable only by observing a panel mid-flight, and it is the intended consequence of a move that *is* tested at both ends. (ii) The three mismatch tests pin `modelHashIntegrity` only; `projectEnvelopeHashIntegrity` is moved by the same three-line block in the same position, so it is covered by construction but not by assertion. (iii) The in-flight test mocks `get_unit_catalog` to reject, so it runs with a degraded unit catalog — honest, and orthogonal to what it pins. None of these is actionable.

## Check 6 — The safety net

`git diff --name-only 64f86e17f...36e79602e -- …/apps/desktop` returns exactly the two files. No existing test file and no Playwright spec appears anywhere in the range; `App.test.tsx` diffs empty. I extracted `runMenuCommand` from both `64f86e17f` and the candidate and hashed each: **both `b75f08dc135a384edda0ec1a80f1343d732b8163dc7f4cbb7728e0ea33375859`** — byte for byte the base's, as the manager states.

## Check 7 — Scope and copy

Only the two files under `apps/desktop`. I grepped the product diff for `aria-`, `role=`, `data-testid`, `className`, `getByText`, `.css`, `px` and `color`: **no match**. No tolerance, oracle or limit changed; `solveGates.ts` and the services are untouched.

## Check 8 — Records

I scanned every authored file in `64f86e17f..36e79602e` for the user home pattern, the home pattern, `C:\`, the private temporary folder pattern and the var folders pattern: **no match**. The manager's brief index lists `B2F-HANDLERS.md` with SHA-256 `73952ad0…576f23b` and model requested `fable` (Claude Fable 5.1); I recomputed the hash and it matches, as does the retained launch brief's `2a2542c7…ccf7e33e`. The implementer's model is distinct from mine, as the brief intends.

## Check 9 — My reading of what the manager brings back

| # | Manager's item | My severity | Blocks this slice? |
|---|---|---|---|
| 1 | The native menu is not disabled while a project request is busy; the safety net pins that reachability | **major**, against the base | **No.** The fix is in `src-tauri/**`, outside the lane, and closing it requires changing a safety-net test. Correctly routed to the owner. |
| 2 | `handleCreateProject` / `handleSaveProject` null both integrity cells before their first `await` and do not restore them | **major**, against the base | **No.** Both functions are byte-identical to the base; this slice makes nothing worse. Out of addendum 2's scope. |
| 3 | A list settling during a later request still publishes its message, operation and index | **minor**, against the base | **No.** Strictly better than the base, which did this *and* released busy. |
| 4 | The same-tick edge | **trivial** | **No.** Not reachable by a user; if hit, identical to the base. See check 2. |
| 5 | Host note on the file-writing tools | not a code matter | No. Both product blobs are hash-attested and I reviewed the committed bytes. |

On **item 2**, which the brief singles out, one refinement I would add to the manager's wording. It frames the defect as failure-to-restore "when the request fails or is superseded". Reading 1590–1684, neither cell is restored on **any** save path, success included — `adoptNormalizedPersistenceModel` (1307–1344) does not set them either. I think that splits in two:

- On a **successful** save or create, nulling is defensible: the persisted bytes have just been rewritten, so the open-time comparison is genuinely stale and `open_verification_not_run_this_session` is an honest reading.
- On a **failed or superseded** save or create, nothing was persisted, so the open-time verification is still valid — and nulling it makes a recorded `mismatch_review_required` disappear from a project that stays open. This is the same class as item 3, in the same direction (it **under**-reports a mismatch), and it is the part worth repairing.

I would size the follow-up as the same move the child made here — the two nulls go to where the save or create lands — and note one thing this slice's shape does *not* transfer: for a save, "where it lands" must also decide whether to re-derive rather than simply null, since the persisted envelope has changed. That is a design question the open handler did not have to answer.

Also worth stating for the owner's benefit: after this slice, two of the four project handlers preserve the still-open project's integrity record on failure and two do not. That asymmetry is an argument for scheduling the follow-up, not for holding this slice, which repairs one half of it correctly and tests it.

## Observations that are not findings

Recorded so the PASS is not read as silence on them.

1. **A native `file.list-local` while busy is now silently refused.** Against constraint 6 ("a control whose gap is outside the tranche is absent, or disabled with its reason"), the native item is now enabled but inert. I do not raise it as a finding: the *button* and the in-DOM menu item are both already `disabled={projectBusy}` and give no feedback either, so the handler now matches them, which is what addendum 2 sanctioned ("a guard … that matches what the buttons already do"). The smallest honest correction — disabling the native item — is in `src-tauri/**`, outside this lane. A message inside the handler is *not* available as a correction: it would publish over a pending request and would break this slice's own first item-2 test.
2. **The new test finds four of five project controls by accessible name** (`Create local`, `New blank`, `Open local`, `List local`, `Save local`); only *Open local* carries a test id. That is the same practice as `App.test.tsx` and is not a copy change, but it couples the new file to that copy, which matters for B3's shell work.
3. **Handoff §3 constraints.** 1, 2, 4, 5, 8, 9, 10 are untouched by this diff. 3 (result integrity) is improved, not weakened, on my reading above. 6 is the observation at (1). 7 is satisfied: six semantic changes are named in both returns and none is delivered as restyling — the slice draws no surface and `App.tsx` is untouched.

## What I did not check

- I did not run the full unit suite, either Playwright lane, the build, the evidence sweep or a dev server. The 1,197-test suite, the two Playwright lanes, `npm run build:desktop` and the registered checks are the manager's and child's reported results; I did not reproduce them.
- I did not verify red-then-green by executing the tests against `ef81fc32e` or the unrepaired blob — doing so needs a state-changing git command or a file modification, both forbidden here. My failure reasoning is by reading, per the brief's allowance, and I identified for each test the specific assertion and the specific removed statement that makes it fail; the manager reports independently re-running all seven red.
- I did not review the records files' content beyond checks 8 and 9 (absolute paths, the brief index, the two returns), and I did not audit the lane's commit graph, the `b2_move_audit.mjs` / `b2_order_check.mjs` tools, or slice B2's own diff.
- I did not review `src-tauri/**`, which is where the native-menu enablement question lives.
- I did not assess usability, accessibility conformance or performance, and I make no acceptance claim of any kind.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
