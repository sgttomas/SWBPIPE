# Compact review repair manager return

WORKING_ITEMS /root/b4_manager returns SR-1/SR-2 repair to HELP_HUMAN /root for commit and independent backcheck. Retained TASK /root/b4_manager/compact_impl executed the bounded repair under the supplied disposition and full review. No source extension was required.

Frozen repair manifest: TASK/CANDIDATE.json, digest `e31a1920e479bd8c386025a4c189b81980d34349d61c7eb69eb2ae383eec686d`, relative to candidate1c8f04039b5f41812ebd879fc80d810392123072. Exactly three maintained files changed: ModelTree.tsx, b4-table-editing.spec.ts, ui-foundation-dist.spec.ts. Manager recomputed all three hashes (match), inspected the final source/test diff and ran scoped diff whitespace checking (PASS). MANAGER_CHECK.json records this integration check, not independent review.

| Finding | Repair and checked consequence |
|---|---|
| SR-1 | Confirmed pre-repair Tab/Shift+Tab escaped Details focus and collapsed narrow drawer. ModelTree bubble handler now owns otherwise-unhandled open-Details Escape after editor/enum handlers and before drawer. Focus exit dismisses without stealing focus. Two connected Model/narrowBoth browser journeys passed immediate/traversed Escape, trigger restoration, editor/enum priority, ordinary subsequent drawer Escape, retained drafts/input identity and compact/noncompact/Tree teardown. No global capture listener added. |
| SR-2 | Accessible name is Grid family; source/dist locators and explicit accessible-name assertions contain visible Family. Verified printable keyboard typeahead changes to Pipes with focus retained. |

Checks:65 component tests PASS; final TypeScript PASS;2 connected keyboard journeys PASS;2 representative minimum-drawer geometry/interaction backchecks PASS. No unaffected broad32-case rerun. Final product bytes were unchanged after the component run. First two browser-test attempts used unsupported arrow/menu assumptions on this headless host; a minimal HTML select reproduced them, and actual printable typeahead supplied the final meaningful keyboard witness. Failed attempts and diagnosis remain raw evidence. Native popup arrow behavior is not claimed.

Canonical evidence is enumerated in TASK/RETURN.md. Raw JSON reports/observations/commands reside beneath TASK/_run_records without sanitizing bytes, per ROOT portability direction. Initial candidate evidence remains historical; ROOT owns its separate relocation/prose repair. No new screenshot/trace payload is necessary for this repair.

All owned focused checks exited; TASK stopped Vite5174 (exit130) and reports no remaining listener. No native/CUA was used. Manager releases source and browser/build resource ownership to ROOT. No Git mutation, geometry/budget, operation/controller, benchmark criterion, instruction or other-project change was performed.

Independent complete-candidate backcheck is still required before native/broad sweep. Native long-feedback exposure, reachable Model/currentBoth pointer/wheel, protected native-minimum reachability and ROOT light/dark own-look remain pending under the existing native plan. Dist appearance was not rerun here. Separate wideBoth+Inspector Provenance pointer issue, broader grammar, practitioner/performance/product/release holds remain open. No owner question or new scope decision is required by these repairs.
