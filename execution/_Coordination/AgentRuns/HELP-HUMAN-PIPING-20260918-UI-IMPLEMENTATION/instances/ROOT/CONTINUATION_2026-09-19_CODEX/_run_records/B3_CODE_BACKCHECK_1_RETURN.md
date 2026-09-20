**Backcheck: original finding 1 remains partially unresolved. Findings 2 and 3 are resolved.**

Reviewed candidate `99c0bf0e404982c7de358af56d8292317af730ad` against `8c0f1593963e4cc8ee3f8ca750f14587afb8c2e0`. The clean wt2 checkout matches that candidate; its desktop bytes match manager checkpoint `f5bc01f8b`.

- **P2, residual of finding 1 — page Close still loses keyboard focus.** [App.tsx:805](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/src/App.tsx:805) calls [closeShellPage:2283](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/src/features/workspace/workspaceSession.ts:2283), which only changes navigation state. Keyboard activation of Close unmounts the focused button without selecting a visible focus destination. The regression at [b3-accessibility.spec.ts:79](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt2/projects/chirality-piping/apps/desktop/e2e/b3-accessibility.spec.ts:79) then explicitly focuses the retained draft, proving re-focusability rather than automatic restoration. Restore focus through the page-close operation and assert the destination immediately after real Close/Enter activation. ROOT has confirmed this remains within the original repair scope.

- **Finding 1’s covered-stage exposure is repaired.** The new `inert` boundary excludes retained stage controls from Tab and Chromium accessibility navigation without unmounting the canvas, inspector, or draft. The retained evidence covers Libraries, Project, and Analyze at both configured sizes.

- **Finding 2 is repaired.** Removing rail/strip clipping permits the reason tooltips to appear. All six retained tooltip witnesses have empty clipping lists, successful five-point hit ownership, and rectangles within their respective 1440×920 or 1280×800 viewport.

- **Finding 3 is repaired.** The global Escape handler checks the inspector’s actual DOM containment, including portalled routing controls, after consumed-event guards. Both-view closing uses the existing command sink, restores opener focus, and supersedes routing restoration. Model view remains docked. The narrow handler now respects consumed events as well. No additional regression found in these changes.

All five changed files were reviewed. The repaired log and return match the supplied hashes; all **52 evidence-manifest entries** and all **four tested-product/spec hashes** also match. The retained final browser run reports **12 passed**; the focused unit log reports **2 files / 40 tests passed**. `git diff --check` passes for this repair diff. I ran no additional browser/native session, test suite, server, benchmark, mutation, or delegation.

**Fan-in remains pending** the page-focus repair and independent backcheck. Repaired full source/dist lanes, the final rebuilt native witness—including page-close focus—structure review, ROOT inspection, clean-candidate sweep, and actual-candidate CI remain separate gates. The reported old-candidate source result does not cover these repairs. Native-select popup cancellation remains explicitly unwitnessed; the toolkit test establishes only its own consumed-Escape behavior.

For governed retention, the additional context manifest from the original review follows. Paths are repository-relative. `R` is `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `C` is `R/instances/ROOT/CONTINUATION_2026-09-19_CODEX`; `D` is the sibling run `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

```text
AGENTS.md
d151dad92a074abebf8e6225c92c4c6e88fd586f2283377b50b5051eff39be7b
agents/AGENT_TASK.md
1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7
projects/chirality-piping/AGENTS.md
eec1b9accc5dc4485fd0aab423678bed4c4c98396dc88b9089703965eb3d2005
projects/chirality-piping/loop/LOOP_INIT.md
f327d5c6fe4d964a15786813f6de72170bc5e94471753846f461f622281e5f94
.agents/skills/software-code-review/SKILL.md
06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a
C/B3_CODE_REVIEW_BRIEF.md
9381ac27b96b702ad244b6e3129a4824ed79d03bb476b908bcd4b26af14c6e7f
C/B3_REVIEW_CHECKLIST.md
da8d783c1c60fc3b5f61eb20cc10f27cd446fb075c36f250cc5dadeeca501755
C/DECISION_AND_STRATEGY.md
5dc9bfbaefc2fb2de35b6145f23762900556723c67dba15bb843bc026f0aac1a
C/OWNER_APPROVAL.md
c13f02ac0b19ea1e1fae86ff89f90f9472f3bfb8ad93074b13db4504670321d9
C/OWNER_MODEL_DIRECTION.md
29af1ded5d35ec10efd0820db164ea6d7dca2ae4330dfdc9eb2c9e1e1720cd14
C/B3_MANAGER_BRIEF.md
e494ccdc4763cca0ff53ae238a33c3d3a83299f66a701b75b107c1a5bdb5fb3f
C/B3_MANAGER_ADDENDUM_1.md
9deba418f41a372e4b7f3522ebe58ada2b5360b50fbefadd8bceb812cfd15b61
C/B3_MANAGER_ADDENDUM_2.md
f3363e5a2b365af8d0a95cfc1893d80f29ee21dd6aa8aaabd40a63ddb9147ae2
C/B3_UNDO_OWNERSHIP_DECISION.md
174fe12bd21785297f4d4d91e14cc217bbd36bf884dae262fff7ef6be31418dd
C/_run_records/CODE_REVIEW_INPUTS.json
4fad4532dc31905c2fa7107ea36bd24d2c226c9e7995f668185838cec3de6a5f
R/ORCHESTRATION_PLAN.md
4d93e0b5da354030061415bf1b96fba5819d09a059e79d4cb657d0aa289a5c9f
D/instances/UX-SPEC/UX_SPEC_V1.md
2141c1e844109d4869287048c695e79677f423acae965d5dd0436b7f6aeff682
D/instances/UX-SPEC/OPERATIONS_MAP.md
47e2aa303a358b77fa89439aac198146dbc0a2a13cef9fb86521789888f111c2
D/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md
0711b0e35f8d02f404dcae9a2b440bcf4506d5f0f9b76dc748195952bf7ab9a7
```

Same independent TASK reviewer, parent ROOT, requested Astra/xhigh allocation; no independent backend model/effort telemetry. Standard F-PIP-2 / DEC-081 claim fence applies.
