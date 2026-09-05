# Compact R2 diagnosis return

RUN_STATUS: SUCCESS (diagnosis only; source remains held)
ControlSurface: MERGED
TaskProfile: NONE
TaskSkill: software-defect-diagnosis
ResolvedSkillPath: {REPO_ROOT}/skills/software-defect-diagnosis
ResolvedSkillVersion: 1
ResolvedTaskProfileRequirement: NONE
CompanionFiles: BRIEF_SCHEMA.md (found), TOOL_POLICY.md (found), QA_CHECKS.md (found)
AllowedTools: parent-authorized read-only source/archive/image inspection; no reproduction, builds, network, installation, Git mutation, or delegation.
ToolPolicyCompliance: PASS against explicit sealed brief inspection override; generic skill registered-check restriction does not describe these parent-authorized inspection tools.
WriteAuthorization: ALLOWED_WRITE_TARGETS, own diagnosis directory only.
ToolsUsed: shell source reads; python3 standard-library zipfile/json/hashlib trace extraction; view_image final captured frame.
AppliedChanges: own derivative evidence only.
MISSING: numerical boundingBox return values are not present in captured Playwright trace.
NEEDS_HUMAN_RULING: none for diagnosis; repair and validation are manager-owned next work.
DEPENDENCY_NOTES: none.

Accepted upstream source: 46f3aed341c07046bd0f1ccc1c64f704971bebf1. Source files read match this commit (scoped git diff empty). Upstream DEC025_ATTEMPT1_RETURN.json binds failing sweep SWEEP_20260905T061913Z_46f3aed341c0.json. This report is derivative diagnosis evidence, not authoritative decomposition truth, acceptance, or a sweep waiver. Model identifier unavailable; nondelegation instruction+config asserted. Role not mechanically enforced.

Scope: PKG-07 bounded compact-layout integration, DEL-07-05 results viewer (SOW-023, OBJ-006/007), with DEL-07-06 usability implications. Contract OPS-K-AGENT-1/3/4 and OPS-K-AUTH-1 apply: claims remain evidence-calibrated and output is a proposal.

## Symptom and evidence

Expected: after scrolling the axial force row into view, its rectangle does not intersect workspace status and the row remains selectable. Actual: chromium-compact 1280x800 source-mode preview, r2-smoke.spec.ts:547 → helper:88, received overlap=true. Trace context: Playwright1.60.0, Chromium148 user agent, darwin, deviceScaleFactor1, localhost5174. This is recorded reproduction evidence; no new check was run.

The original ignored trace at {WORKING_ROOT}/apps/desktop/test-results/r2-smoke-R2-desktop-previe-4ab18-report-and-viewport-overlay-chromium-compact/trace.zip was read and SHA256 verified as e5c604eef6a7f69bef5c63012b82b2461e467ae5e8e86b415fed2fa155eee3d8. N8 TRACE_RETENTION.json records raw retention; raw54MB was not duplicated. The final screencast resource resources/page@e3b89264c422e9fb1bce68c067106e82-1788589466859.jpeg is saved unchanged in _run_records/final-frame.jpeg. It visibly shows the results table passing underneath the status region. The extract failure-events.json retains the final tool event sequence. Trace scroll finishes before bounding-box requests; the target is visible, dock scrollTop is582. Bounding-box API results contain element handles rather than rectangle coordinates, so this report does not invent exact numeric boxes.

Screenshot-derived approximate CSS geometry (image rendered at800x500 for1280x800 viewport): titlebar height124, workspace top195, modeling area start203, results dock header top434, status top624, footer top766. These are visual estimates, not DOM measurements. Workspace has about429px height; compact child floors require534px (230 modeling +280 dock +8gap +16padding), leaving about105px overflow. Exact row intersection depth remains unmeasured.

## Causal diagnosis

High confidence: inconsistent compact layout minimum heights overflow the allocated grid workspace track. styles.css:61 app-shell reserves auto rows for chrome/status/footer and a remaining1fr workspace row; :479 workspace permits shrink via min-height0. At max-height840, :1816 onward gives modeling min210 and dock min300. Later max-width1320 rules (:2935 onward) override these with modeling min230 and dock min280. Their combined minimum534px exceeds the approximately429px workspace space in this solved state. Workspace overflow is visible by default, so the dock physically continues into the following status track. The dock body already scrolls, but scrolling to its own larger rectangle cannot respect the status row overlaid on that rectangle.

Contributors: wrapped titlebar display guidance/actions and solved proof/status content increase auto row heights. The captured status region is about142px, not its42px floor. There is no evidence of an animation race; the same geometry remains visible through the final captured frames. Arbitrary waits or weakened intersection assertions would not repair this physical budget conflict. No independent contribution magnitude is claimed for the new display selector without a before-state measurement.

## Proposed smallest repair and fence

Change only layout CSS: allow the compact expanded dock to shrink to the actual remaining workspace allocation by removing/overriding both responsive dock floors (300 and280) with min-height0 under the compact-height rule, placed after the width breakpoint so the cascade cannot restore280. Preserve the existing internal dock-body overflow-y:auto and visible dock header. At the captured state this leaves approximately175px for dock including its header, rather than demanding280. Exact selector fence: append `@media (max-height: 840px) { .workspace-dock { min-height: 0; } }` after the max-width1320 block. This is smaller than deleting the existing floors because it changes only compact-height behavior; the default210px and width-only280px floors survive on taller displays. This is a repair candidate, not verified working behavior. Do not merely add overflow:hidden to workspace; that could hide unreachable controls. Do not alter result data, status disclosures, or scroll helper to ignore the overlap. If smaller supported sizes leave insufficient header/body space, adjust modeling budget too based on measured container geometry; do not guess another arbitrary floor.

Keep the existing exact axial row no-overlap and subsequent click/details assertion. Add a compact solved-state structural fence that workspace-dock bottom is <=workspace-status-bar top and dock body has a positive usable clientHeight; include those measured rectangles in failure diagnostics. Exercise header close/reopen and late/bottom table or pagination controls after scrolling, to prove content remains reachable. Ensure both 1280x800 (height+width breakpoint intersection) and existing1440x920 lane retain current checks. The existing R2 test already supplies830 rows, solve proof and display selector; use it rather than fabricating a shallower state.

Verification next: manager-authorized focused compact R2 reproduction, then required fresh read-only review of frozen source diff; new source commit and clean complete DEC025 are still required. No test/build rerun, source edit, test edit, or Git mutation performed here.

## Handoff state

Diagnosis closure: complete, cause high-confidence, smallest repair proposed but unverified. Product/phase closure: OPEN. Accepted upstream source and failing sweep as above. Derivative package: this immutable return + status + hash manifest and minimal extracts. Remaining blockers: repair implementation, actual measured geometry backcheck, independent source review, fresh clean source-bound complete DEC025. No accepted upstream pointer updated.
