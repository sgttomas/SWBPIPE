# Completed feedback crosses project generation

Bounded manager backcheck after freeze01. Reproduced by generation-feedback-repro-01.txt: accepted Apply leaves OLD_PROJECT_WARNING status with no active edit; rerendering a replacement generation leaves the old warning visible. Expected no previous-project diagnostic or inapplicable cancellation notice.

High-confidence cause: generation layout effect only calls setFeedback when editRef exists, so the no-active-edit branch preserves completed-operation feedback. Minimal repair: assign cancellation notice when there was an edit, otherwise clear feedback on every generation change. No controller/operation semantics change. Unchanged regression plus all core interaction tests and TypeScript are the repair checks. Existing software-defect-diagnosis skill applies with origin/hash retained in FOCUS_DEFECT_DIAGNOSIS.md. Manager explicitly authorized this repair; freeze01 and original WORKER_RETURN remain historical intermediate evidence.
