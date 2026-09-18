# Continuation candidate review — FAIL

Reviewed candidate `b97dfb1fe877eab042237107f6e4c0f381a9407d` against `fc21b00d0b4233349c7fa8df433d6a82e9f7678e`; instrument revision `5aba132e6176a55bf07ba116ccdf3c7b003d7c9e`. This verdict applies only to this frozen cut. Root reported a separate repair, which is not accepted by this review.

## Confirmed finding

**CR-1 / P2 — Preserve metadata rejected by the phase-label policy.** `projects/chirality-piping/apps/desktop/e2e/ui-foundation/full-cohort-controller.ts:536–537`: a structurally valid point-selection snapshot with labels enabled, or orbit snapshot with labels disabled, passes `captureBoundary` and its rejection-persistence wrapper, then throws before `persistBoundary`. The actual rejected snapshot and differences are lost. Route these unchanged checks through the rejection-persistence validator and cover both cases. Reported promptly to Root; Root acknowledged and routed correction. No other concrete defects found in this bounded review.

## Coverage and assessment

All 303 frozen paths have exact byte-size/SHA256 equality in the checkout and candidate Git blobs; the path set equals the complete base-to-candidate diff. 176 paths carry forward only through exact SHA256 equality with the prior independent review. All 127 new/changed paths were reviewed, including all eight maintained harness files, governance records, source archive, manifests, supplied logs and synthetic output. `RETURN.json` enumerates every path/hash and its coverage basis. The 34-member successor method manifest was rehashed; the compressed source patch was reconstructed in memory and its eight outputs matched integration source.

The content-height correction matches product structure: outer workspace columns scroll independently while inner panels grow with content; filtering to no matches substitutes a short empty-state paragraph. Positive inner height remains required and observed; selector, visibility and width remain invariant, alongside outer pane/canvas geometry, model/source/profile and display bindings. This is a demonstrated source-level invariant defect; the original rejected snapshot was not retained, so the exact historical differing field remains unknown. Frozen sample 18 is the same no-match string for both fixture sizes. The untimed smoke source covers immediate project-to-pipe and filter/clear transitions.

The per-slot driver path selects only the claimed slot. The original consumed attempt is hash-bound; nine remaining slots are ordered and claimed before launch. Claims survive absent results, canonical registry binds ledger/method, and prior cleanup/binding receipts are required. Interrupted attempts require external recovery. Process exit/signal/launch failure remains distinct from evidence validity/completeness, and only a successful actual exit plus valid complete evidence contributes to valid-complete counts. An individual failed slot does not fabricate cancellation of later slots. Strict/default semantics remain separate. These are offline source conclusions, not proof that a real continuation run succeeds.

Owner propagation in the decision, acceptance map, graph and three affected deliverable memory/status surfaces holds closeout and preserves historical evidence. It does not infer a homogeneous ten-run pass across method identities.

## Evidence limits and handoff

The 67 JSON files under writer test-output are synthetic Node-only regression artifacts, not timed browser evidence. Supplied focused logs report 39 tests; the final structured test-results file corresponds to the later single budget regression. I ran no tests, builds, servers, browsers or native tools. The original cohort remains one invalid attempt, zero valid complete and nine unattempted, with raw custody retained from exact prior coverage. Its incomplete report does not close the revised owner assignment.

Final runner policy, literal launch commands and precondition-generation scripts are not sealed in this candidate; drafts remain non-executable. Correct CR-1 and affected-backcheck its successor together with the final runner artifacts before Root considers runtime release. Real untimed witnesses, bounded continuation execution, final evidence reconciliation and the registered final sweep remain future work. No prospective acceptance is granted.

TASK Type 2 `/root/d70_final_review`, parent `/root`, configured gpt-6-astra / low as declared by dispatch; no independent runtime model introspection was available. Role, non-delegation and write boundaries are instruction/config asserted, not mechanism-proven. No delegation occurred. Only filesystem/Git reads, bounded offline checking and these two authorized output writes were used; no Git mutation or product/source edit occurred.
