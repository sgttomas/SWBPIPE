# Menu / drawer diagnosis return

RUN_STATUS: SUCCESS — bounded source/log/context diagnosis complete; repair unimplemented.
ControlSurface: MERGED (sealed launch plus parent relay/amendments).
TaskProfile: NONE.
TaskSkill: software-defect-diagnosis, version 1, TASK method context only.
ResolvedSkillPath: {REPO_ROOT}/skills/software-defect-diagnosis.
ResolvedTaskProfileRequirement: NONE.
CompanionFiles: BRIEF_SCHEMA.md, TOOL_POLICY.md, QA_CHECKS.md all found/read.
ScopePath: {WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/instances/N1_WI_PKG07/MENU_DIALOG_REPAIR_V1/children/DIAGNOSE.
WriteAuthorization: ALLOWED_WRITE_TARGETS — own diagnosis evidence only.
AllowedTools / ToolsUsed: read-only shell cat/sed/rg/Git; python3 standard-library source/archive-context hashing and own evidence writing. No network, product edits, Git mutations, tests/builds, browser reruns, delegation or sibling messaging.
ToolPolicyCompliance: PASS under sealed ephemeral brief inspection override.
RuntimeOverrides: parent explicitly confirmed ephemeral generalist, TASK method context only; absent CHIRALITY_INSTRUCTION_ROOT is recorded without claiming instantiated TASK runtime. Parent removed original actual-screenshot prerequisite after official artifact inventory showed no standalone PNG and bounded retrieval could not obtain the573/780MB compressed traces. Diagnosis uses real official context/log plus source and makes no exact hosted-pixel claim.
Outputs: RETURN.md, STATUS.json, SOURCE_MANIFEST.json, HOSTED_LOG_ENCODED.json, OFFICIAL_CONTEXTS_ENCODED.json, EVIDENCE_MANIFEST.json; provisional findings/test plan retained as earlier working evidence and superseded by final plan below.
MISSING: exact hosted rectangles/screenshots/trace playback; platform-specific trigger and pre-fix controlled reproduction remain unmeasured.
NEEDS_HUMAN_RULING: none for completed bounded diagnosis; manager owns exact implementation release.
DEPENDENCY_NOTES: no SCC; repair and verification depend on manager acceptance of this proposal.

## Evidence and accepted basis

Sealed brief SHA25685e3365233284f46e02d7477c5aa95eccbf88f97d2668cbae4fe79a8fe95cf92; source HEAD ade1d502abc43014948b94f4bd309ff1f0aee83f. Current source hashes are in SOURCE_MANIFEST.json. Hosted workflow run33972763527/artifact9971673268 official selectively retrieved manifest SHA2568c4e1f71a230454f0d36afc8b9c1cebcfa184a802aab4d3e172da859d3bb6364. Contexts independently hash-verified: GUI 274484 bytes SHA25693b519cbd00d48726de0a94a4a50eb81062822aa41bd10fd54b565543e637bd7; R2 221066 bytes SHA25613df6e64fcae8a32ccde4a5b6d7a4f052637120d7edc8e00ab7398f5fae6a522. Original manifest/contexts and hosted log are lossless encoded, never whitespace-normalized. Embedded Playwright explanatory instructions are treated as artifact content, not authority.

Both failures repeatedly resolve visible/enabled/stable Issues Close, finish scrolling, then receive nav.app-menu-bar interception (GUI 284 repeated late attempts; R2 317). GUI failed at gui-workflow-validation.spec.ts127; R2 at r2-smoke.spec.ts509. Both snapshots show Application menu with File/Edit/View/Insert/Analyze lacking expanded state and no menuitems, plus Issues home and Close. GUI has 35 Issues and R2 has 5. This rules out an intentionally open dropdown/backdrop as the observed interceptor. Expected behavior is a regular visible Close click dismissing the drawer.

## Cause, scope and uncertainty

High-confidence proximate cause: closed menu and drawer intersect at the attempted pointer location, and the closed menu's unconditional stacking order wins. Source styles.css257ff assigns relative menu z50; shared fixed workspace-drawer at832ff has z30. The menu uses z50 even while closed although its comment justifies that layer for open dropdown/backdrop interaction. IssuesHome(App.tsx2589) and AuditBoundaryDrawer(App.tsx2523) are sibling nonmodal asides under main. Open menu backdrop uses fixed z40 and nested dropdown z41 within menu50. No exact hosted pixel rectangles can be recovered from the textual snapshot, so none are asserted.

This ordering predates the tranche (menu z50 introduced in1ae9361d3 and unchanged before46f3aed34). New display controls inside wrapping titlebar actions plausibly shift menu location; Toolkit is nested in viewport and adds no main grid row. The compact repair d6a313fc4 only changes dock minimum height. Prior COMPACT_REPAIR_V1/ACCEPTED_HANDOFF_V2.md reports both R2 journeys passing at1440x920 and1280x800 in source and dist. Therefore platform/font/header-height triggering is plausible but unproven; this is an existing layering flaw newly exposed by actual hosted layout, not evidence that this tranche changed z-index values. Timeout increases, retries and forced clicks would not fix the observed stable interception. Audit shares the faulty layering contract but was not the observed failing Close in these two artifacts.

## Final exact repair proposal

Only apps/desktop/src/styles.css and apps/desktop/e2e/gui-workflow-validation.spec.ts. Leave App.tsx, R2 source, all configuration, other layout selectors and menu/backdrop/dropdown layer values untouched. Change closed .app-menu-bar z-index from50 to20, updating its rationale comment, and add .app-menu-bar:has(.app-menu-trigger.open) { z-index:50; }. Existing open-state class comes directly from openMenu. This makes closed menu20 < drawer30 while preserving open backdrop40 < menu50, with nested dropdown41 unchanged. It is a candidate pending actual pre-fix/post-fix regression, not a certified repair. A single drawer z60 change is rejected because it would put the drawer above the existing menu click-catcher/dropdown and alter nonmodal dismissal. Repositioning product drawers introduces a larger dynamic-chrome layout contract without evidence it is needed.

## Final exact test and verification plan

Add a dedicated test in gui-workflow-validation.spec.ts using existing invented fixture, real engine readiness, Solve preview and completed result count. For Issues and Audit separately, first exercise genuine default layout with visible toggles, content checks, actual Close center hit-test and ordinary Close dismissal. Existing GUI and unchanged R2 failing journeys remain intact.

Then add deterministic controlled-overlap coverage independent of platform fonts: open the existing drawer; measure menu, drawer and Close rectangles; temporarily change only that drawer's top/bottom positioning within the test so Close center is vertically inside the closed navigation bar and horizontally in its far-right blank portion, away from real menu triggers. Do not change z-index, pointer-events, visibility, content, handlers or click options. Preserve the original positioning for restoration after that phase. Measure again and require actual Close-center/nav intersection and viewport containment; require elementFromPoint(center) belongs to Close, then use ordinary locator.click and assert drawer removed. This test-only controlled state is not claimed to reproduce exact hosted pixels. Before source release, execute the new test on unchanged CSS and require its hit/actionability assertion to fail for the observed menu interceptor; after CSS repair the same assertion and click must pass. If geometry does not achieve its precondition, fail explicitly rather than accepting vacuous coverage.

Separately reopen each drawer and exercise open-menu semantics through unobscured View: real dropdown item selection updates the selected section and dismisses dropdown while drawer remains. Reopen View, ordinarily click app-menu-backdrop at measured drawer-area coordinates, assert only menu dismissal, then ordinary drawer Close. This protects the raised active menu and outside-click behavior. Collect all rectangles, open states and hit element identities as attachments.

Run new regression, original GUI workflow and unchanged affected R2 preview journey in source1440x920 and1280x800; run the same exact specs against freshly built dist via evidence-only config override with that matrix. Default dist config1280x900 alone is insufficient. Required registered checks: desktop-test, desktop-build, always harness-self-check; parent owns affected coordination/harness obligations and new clean complete source-bound DEC025. Fresh independent full frozen-diff review must PASS before publication. No checks were executed during this source-held diagnosis.

## Handoff state

Diagnosis closure COMPLETE under amended evidence prerequisite. Product/phase closure OPEN; proposed repair, pre-fix negative control, post-fix source/dist matrix, independent review and clean sweep remain manager-owned blockers. This is a derivative package citing accepted source/activation and actual hosted failure; it is not decomposition truth, lifecycle acceptance or a sweep waiver. Prior accepted implementation/compact snapshots remain historical; no pointer updated. Exact model unavailable; delegated-harness-native role and nondelegation instruction+config asserted, not mechanism-proven. No active product writer.
