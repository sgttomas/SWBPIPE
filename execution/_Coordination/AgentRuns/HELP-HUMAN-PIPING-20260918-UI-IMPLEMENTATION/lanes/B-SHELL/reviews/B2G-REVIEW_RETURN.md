# Retained return — B2G-REVIEW (independent review of B2G and the combined candidate), 2026-09-19

Retained by ROOT. What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `19a5f2bb5bff9aa3df2e92f6f23d61d03cbf3d35a4eaf89dd9884e475b3423e9` (129 lines), by this script.

---

I have everything I need. Writing the return.

# B2G-REVIEW return

**Verdict: FINDINGS — one finding, minor, not blocking the merge as a code change.** The repair is correct, minimal and exactly what addendum 4 specified; the tests are real and I reproduced red-then-green. One statement in the retained B2G return about the reachability of its own known edge is false, and I have a reproduction that shows it.

I am **Claude Opus 5** (`claude-opus-5[1m]`), TASK (Type 2), read-only, working alone, no delegation. I ran no state-changing git command and modified no file in the worktree.

## Instruments

| Instrument | Expected SHA-256 | Read |
|---|---|---|
| Sealed brief `B2G-REVIEW_code_review.md` | `992f3b4f…5e7f3d8a` | matches |
| Skill `.agents/skills/software-code-review/SKILL.md` | `06c27b1b…f95570a` | matches |

Worktree at `944f9d72f`, clean. Product tree is the candidate's (`74707b134`). Combined diff reviewed at 100 %: `64f86e17f...0fc5b065e` under the desktop app — exactly two files. B2G alone: `dfa7c47d5..0fc5b065e`, `workspaceSession.ts` +12/−4 and `App.projectHandlers.test.tsx` +114/−0.

I also verified, before reading the B2F return, that the two slices do not overlap: B2F's product hunks are at 1437/1462/1501/1520/1531/1677/1689; B2G's are at 1353/1384/1597/1629. B2G modifies no line B2F changed and adds only to the test file B2F created.

---

## Check 1 — The move, enumerated

`handleCreateProject` and `handleSaveProject` are structurally identical here; line numbers below are the candidate's for `handleSaveProject` (create is 1346–1394, same statements).

**The window** — every statement between the base's position (after `setProjectBusy(true)`, 1603) and the candidate's (1638):

| # | Line | Statement | Reads either integrity cell? |
|---|---|---|---|
| 1 | 1604 | `try {` | no |
| 2 | 1605 | `const actualRequestModelHash = await computeModelHash(requestModel);` **(await)** | no |
| 3 | 1606 | `const snapshotModelHash = requestHistoricalRun ? … : actualRequestModelHash;` | no |
| 4 | 1607 | `const snapshotResult = requestHistoricalRun ? … : result;` | no |
| 5 | 1608 | `const snapshotAnalysisRun = requestHistoricalRun ? … : analysisRun;` | no |
| 6 | 1609 | `if (!stillCurrent()) return;` | no |
| 7 | 1610–1618 | `const computedEnvelopeHash = await computeProjectEnvelopeHash({ model, editor_intents: combinedContext, proposal, selected_review_target, mechanics_result, analysis_run, model_hash });` **(await)** | no |
| 8 | 1619–1620 | `const envelopeHash = historicalRun && computedEnvelopeHash?.value === historicalRun.envelopePayloadHash ? historicalRun.envelopeHash : computedEnvelopeHash;` | no |
| 9 | 1621 | `if (!stillCurrent()) return;` | no |
| 10 | 1622–1632 | `const saved = await saveLocalProject(model, combinedContext, proposal, selectedReviewTarget, snapshotResult, snapshotAnalysisRun, snapshotModelHash, envelopeHash, modelDocumentMigration);` **(await)** | no |
| 11 | 1633 | `if (!stillCurrent()) return;` | no |
| 12 | 1638–1639 | `setModelHashIntegrity(null); setProjectEnvelopeHashIntegrity(null);` | writes |

No statement in the window reads either cell; neither cell is an argument to any call in it. `stillCurrent` reads only `projectRequest.current` and `requestEpochRef.current`. Create's window is the same list with `createLocalProject` (8 arguments, no `modelDocumentMigration`) at #10.

**Everything that can run while those awaits are pending.** Both cells are written only by the four project handlers (`workspaceSession.ts` 1389/1390, 1469/1470, 1530/1531, 1567, 1578, 1638/1639) — no effect and no service writes them. They are read only at render: `App.tsx` 220/222 → 892/902/904 → `ProjectValidationPanel` and `ProjectStorageAuditPanel`. I traced every reader in the panel (`modelHashEvidenceStatus` 457, `projectEnvelopeHashEvidenceStatus` 476, the preflight packet at 305/310 and 364/366, `validationDiagnostics` 789/792, `modelHashDiagnostic` 843, `projectEnvelopeHashDiagnostic` 864): every one produces a display string or a diagnostic severity. **No handler, gate, guard or service branches on either cell**, so nothing in the session can take a different code path because of the move. Nothing hash-bound or persisted captures them.

**No clobber is possible.** Line 1633 and lines 1638–1639 are in the same synchronous run (no await between), so a handler that starts in between cannot interleave. Any handler that starts during the window advances `projectRequest.current`, which makes this handler's `stillCurrent()` false at 1633 — and only the last-started request-numbered handler can satisfy `request === projectRequest.current`, so the null at 1638 can never land on top of an open's derived value at 1567/1578.

**Final state where the persistence landed and the response is accepted.** After 1639 the handler continues: `await computeModelHash(saved.model)` (1640), `if (!stillCurrent()) return` (1641), `await computeProjectEnvelopeHash({…})` (1642–1650), `if (!stillCurrent()) return` (1651), `responseModelChanged` (1652), `modelChanged` (1653–1659), the **PROJECT-PERSISTENCE-RESPONSE-INTEGRITY** refusal (1660–1664), `returnedHistory` (1665–1667), `if (!stillCurrent()) return` (1668), `adoptNormalizedPersistenceModel` or the guarded `setHistoricalRun` (1669–1676), then 1677–1684 and the guarded `finally`. **Neither cell is written again on any of those paths**, and `adoptNormalizedPersistenceModel` (1307–1344) does not write them either — I grepped the whole file: the only derived writes are the open handler's 1567/1578. So:

- success (1684): both cells null — **exactly the base's**;
- the refusal at 1660 (bytes written, response rejected): both cells null — **exactly the base's**, and this is the path the manager's placement choice was made for; it holds;
- supersession at 1641, 1651 or 1668 (after the bytes landed): both cells null — **exactly the base's**;
- the create handler is identical on all four.

**Only two classes differ from the base**, both intended: a request that throws before the bytes land (1685–1688), and supersession at 1609, 1621 or **1633**. The third is where the finding lives.

## Check 2 — Failure and supersession, my own reading

**A thrown request: right.** Nothing was persisted, the open-time verification still describes the store, and the base's nulling made a recorded `mismatch_review_required` vanish from a project that stays open — an under-report in the unsafe direction. The repair is correct and is the bulk of the slice's value.

**Supersession at 1609 or 1621: right.** No write was attempted.

**Supersession at 1633: this is the manager's edge, and the return understates it.** See the finding.

I also worked the mixed case: an open superseded between its two derived writes (1567 and 1578) leaves the model cell derived and the envelope cell null until the superseding save lands. That is honest (the model verification completed, the envelope's did not) and is not worse than the base, which lost both. Not a finding.

## Finding 1 — The return's reachability claim is false; the edge is reachable by ordinary editing

- **Severity:** minor as a defect; **material** as a records correction.
- **Actionable before merge?** The *code* change is not required — the placement is precisely what addendum 4 prescribed, and the manager followed it. But the sentence is the basis of the manager's proposal to "leave it, and close it with the native-menu question", which ROOT would route to the owner. **That routing would be wrong**, so the sentence should be corrected before the owner acts on it.
- **Location:** `…/lanes/B-SHELL/returns/B2G_RETURN.md`, "What I bring back" item 2, the line "It is reachable only through the native menu during a busy request, which is item 1 already with the owner." The code it describes is `workspaceSession.ts` 1633 (save) and 1384 (create).
- **What is wrong:** `stillCurrent()` is `request === projectRequest.current && epoch === requestEpochRef.current`. Supersession by the **epoch** needs no project command at all. `requestEpochRef.current` is bumped in `commitModel` (line 433 — every applied intent, every undo, every redo, every model commit), in `handleClearReviewQueue` (825) and in `invalidateDirectDraftContext` (1758). Only the five project controls and the project-index buttons carry `disabled={projectBusy}` (`App.tsx` 324–358); model editing, undo, redo and the review queue are not gated while a save is pending. So a user editing the model while a save is in flight drops the save's response after its bytes were written — through the ordinary in-app UI, with no native menu involved.
- **Evidence.** I reproduced it, without touching the worktree, in a scratchpad copy of the desktop app (source copied, sibling folders and `node_modules` symlinked). A probe test opens a project with both mismatches recorded, starts a save on a deferred promise, applies a queued entity-grid intent while it is pending, then resolves the save successfully. On the candidate, the save's message never publishes (`"Opened invented handler project."` still stands) yet both lines read `integrity=mismatch_review_required`; on `9b7687da9`'s source the same probe reads `integrity=open_verification_not_run_this_session` on both lines. The stored bytes changed and the panel keeps an open-time record of the previous bytes.
- **Impact, honestly bounded.** The direction that matters is `verified_match`: the panel, and the exported preflight packet (`ProjectValidationPanel.tsx` 305/310), would carry `model_hash_integrity_status: verified_match` for bytes that were overwritten after that verification and never re-verified. The status strings and `verification_basis` are explicitly open-scoped (`model_hash_verified_on_open`, `recomputed_on_open_from_restored_envelope_payload`), which limits the misreading but does not remove it — and it contradicts the slice's own comment at 1385–1388 ("The persisted bytes have been rewritten, so the open-time verification no longer describes them"). The `mismatch_review_required` direction over-warns, which is safe. Against the base this is a trade, not a regression in the round: the base lost a recorded mismatch on *every* ordinary failure, which is far more common and in the unsafe direction.
- **Smallest correction (if ROOT wants the code closed too).** In both handlers, null under the request-number test *before* the epoch test, so a landed write clears the record even when a model edit dropped the response, while a later project handler still keeps ownership of the cells:

```ts
      );                                   // saveLocalProject / createLocalProject
      if (request === projectRequest.current) {
        setModelHashIntegrity(null);
        setProjectEnvelopeHashIntegrity(null);
      }
      if (!stillCurrent()) return;
```

  I applied exactly this in the scratchpad copy: the probe then reads `open_verification_not_run_this_session` on both lines, and **all 11 shipped tests still pass**, including the three B2G repair tests and the fourth success test. It leaves the remaining native-menu variant (supersession by another *project* request after the bytes landed) exactly as the manager describes, which is genuinely the owner's item 1.
- **Note for the same class in B2F:** `handleCreateBlankProject` (1466) has the identical shape — `createLocalProject` writes bytes, and supersession at its post-await check now leaves the previous project's record. If ROOT takes the correction, it belongs there too.

## Check 3 — Success is exactly today's

Confirmed from the code, not only from the fourth test. On the accepted-response path the cells are written once, to `null`, at 1638–1639 and never re-derived; the only re-derivation in the file is the open handler's 1567/1578; `adoptNormalizedPersistenceModel` does not touch them. The refusal path at 1660 also ends null, as at the base. The design question the addendum deferred — whether a landed save should re-derive rather than null — is correctly left open and listed in the return.

## Check 4 — The tests are real

I ran `npx vitest run src/App.projectHandlers.test.tsx` from the desktop app on the candidate: **11 passed / 11**. `npx tsc -p tsconfig.json --noEmit`: **exit 0**.

I then reproduced red-then-green in the scratchpad copy (worktree untouched) by substituting `9b7687da9`'s `workspaceSession.ts`: **3 failed, 8 passed**, and the three failures are exactly the three repair tests, each at the model-hash line, each receiving `integrity=open_verification_not_run_this_session` where `mismatch_review_required` is expected — the manager's stated reason, verified rather than inferred. The fourth (success) test passes on both trees, so it is a true pin and not a repair test.

- **Each asserts both lines.** All four assert `project-validation-model-hash` and `project-validation-envelope-hash`. The envelope assertion is load-bearing: `openProjectWithBothMismatchesRecorded` *waits* for `integrity=mismatch_review_required` on the envelope line before the test proceeds, so `mismatchedEnvelopeHash` really drives `deriveProjectEnvelopeHashIntegrity` (`projectPersistenceIntegrity.ts` 129–135) to a mismatch, and B2F's gap (model cell only) is closed.
- **No test pins appearance.** Assertions are `toHaveTextContent`, `toBeInTheDocument` and the helper's `not.toHaveClass("inactive")`, which pins that a section is open.
- **Mocks match the real bridge.** `save_local_project` is invoked as `{ request: {…} }` (`projectService.ts` 597) and the test reads `args!.request!` and echoes `saveRequest.model` back — the genuine unchanged-model save (`responseModelChanged` false, so `isSupportedChangedModelPersistenceResponse` is correctly not exercised). `open_local_project` returns `LocalProjectEnvelope | null` (552) and the test uses both a resolved `null` and an envelope. `create_local_project` and `save_local_project` rejections are ordinary bridge failures.
- One observation, not a finding: the third test's supersession is by *request number*; no shipped test covers supersession by **epoch**, which is the finding's path. That gap is what let the return's reachability sentence stand.

## Check 5 — Untouched

- `runMenuCommand` extracted from `64f86e17f` and from the candidate hashes **`b75f08dc135a384edda0ec1a80f1343d732b8163dc7f4cbb7728e0ea33375859`** on both — byte for byte the base's.
- Exactly two files under the desktop app in the whole combined range; no existing unit test and no Playwright spec appears anywhere in it. The test file's B2G change is +114/−0.
- Identifier counts identical base vs candidate for `const stillCurrent = () =>` (10), `commitModelAfterSolveInvalidation` (2), `clearComputedModelState` (6), `ruleRevisionGate` (7), `advanceProjectSession` (4). No guard condition, gate class, solve-input basis, designation, undo or redo is changed; `solveGates.ts` and the services are not in the diff.
- Grepping the B2G product diff for `aria-`, `role=`, `data-testid`, `className`, `getByText`, `.css`, `px`, `color`: **no match**. No tolerance, oracle or limit changed.
- Handoff §3: constraints 1, 2, 4, 5, 8, 9, 10 untouched; 6 untouched (no control changes); 7 satisfied (three semantic changes named in the return, nothing delivered as restyling, `App.tsx` untouched); 3 is improved on the common paths and carries the residual above.

## Check 6 — Records

No absolute machine path anywhere in `dfa7c47d5..0fc5b065e` (scanned for the user-home pattern, the home pattern, the Windows drive pattern, the private-temporary-folder pattern and the var-folders pattern). Record claims I recomputed and confirmed: addendum 4 `08b51c66…801c3088` matches the return's citation; the re-sealed `B3-SHELL.md` is `079876c3…0d9af29cec`, matching both the return and the struck-and-restated index row.

## Does B2G disturb what B2F established?

No. B2G touches neither the blank-create, open nor list handlers, changes no line B2F changed, and adds only to the test file. The B2F reviewer's check 9 item 2 is precisely what B2G implements, including its refinement that the failed/superseded case is the part worth repairing and that the re-derive question is separate — both honoured. The 2-of-4 asymmetry that reviewer recorded is now closed: all four project handlers preserve the open project's integrity record on failure. My finding is the one place its check 1 reasoning does not transfer, because an open persists nothing whereas a save and a create do.

## What I did not check

- I did not run the full unit suite, either Playwright lane, the build, the evidence sweep, the validators or a dev server; those are the manager's reported results and I did not reproduce them.
- I did not review `src-tauri/**`, where the native-menu enablement question lives, nor the Rust persistence layer — my finding's premise that the bridge can write bytes and still have its response dropped is the manager's own, and follows from the promise being dropped rather than cancelled.
- I did not audit the lane's commit graph, the B3 cut proposal or the re-sealed B3 brief's content beyond its hash and index row, and I did not review records outside the two hash checks and the path scan.
- I did not assess usability, accessibility conformance or performance, and I make no acceptance claim of any kind.
- My reproductions ran in a scratchpad copy of the app with sibling folders and `node_modules` symlinked; it is the candidate's source byte for byte, but it is not the worktree and it ran only this one test file.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
