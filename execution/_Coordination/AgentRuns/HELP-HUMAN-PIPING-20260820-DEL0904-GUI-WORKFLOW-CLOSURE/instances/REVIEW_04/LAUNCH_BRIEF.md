# Sealed TASK Brief — N1 Integrated Review 04

- ParentRole: WORKING_ITEMS Agent 1
- ChildForm: TASK + `software-code-review`
- RunID: `HELP-HUMAN-PIPING-20260820-DEL0904-GUI-WORKFLOW-CLOSURE`
- InstanceID: `WI-PKG09-DEL0904-GUI-CLOSURE-REVIEW-04`
- PackageID: `PKG-09`
- DeliverableID: `DEL-09-04`
- Base: `57803893d1eb161f395e0574c256dd27920bf1d4`
- DiffBasis: `FROZEN_DIFF_MANIFEST_V4.md`, exactly 28 SHA-256-bound pre-review paths
- Objective: independently review 100% of the complete V4 tranche. Verify all 28 identities twice; assess product-test behavior, fixture/evidence integrity, documentation and deliverable-state calibration, preservation of owner gates, the complete review/remediation history, and the stable conditional V4/Review 04 fan-in language in `HANDOFF_STATE.md` and manager `RETURN.md`.
- Required checks: independently run tracked and per-file untracked whitespace checks; validate the whole worktree against the N1 write fence; select affected checks using project-relative paths and confirm adequate PASS evidence; confirm no V4-frozen byte changes during review.
- Required verdict: `PASS` only if every V4 hash matches on both passes, 100% of V4 content is reviewed, semantic consistency and containment pass, and zero actionable findings remain. Otherwise return `FAIL` with exact evidence and remediation.
- WriteScope: only `instances/REVIEW_04/RETURN.md` and `instances/REVIEW_04/STATUS.json`; read-only elsewhere
- Forbidden: modifying any V4-bound path; implementation edits; commits; pushes; PR or receipt operations; lifecycle, release, publication, professional-reliance, benchmark-value, or owner-gated acts
- ExpectedReturn: terminal verdict; exact 28-path/two-pass coverage; semantic-consistency result; affected-check evidence assessment; whitespace and containment results; residual risk; CHANGE fan-in validity
