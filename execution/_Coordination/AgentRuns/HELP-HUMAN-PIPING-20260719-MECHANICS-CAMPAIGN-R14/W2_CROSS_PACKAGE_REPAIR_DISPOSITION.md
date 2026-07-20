# Cross-Package Repair Disposition — R14-W2 sweep blocker (DEL-07-02 test mirror)

- **Disposition ID:** R14-W2-XREPAIR-01
- **Dispositioned:** 2026-07-20 (UTC), by HELP_HUMAN (Agent 0) at W2 fan-in
- **Class:** disposition-class exercise under DEC-082 / Shared-Block v1
  (recorded rationale artifact; cited from the wave receipt)

## Condition

R14-W2 T4 (DEL-05-01 sub-span wind exposure, landed `a854d43a1`) lawfully
changed `schemas/model.schema.yaml` `WindEquivalentStaticInput`: `required`
is now `[pressure, shape_factor, direction]` with `exposed_element_refs` and
the new `exposed_spans` as alternative exposure properties. The DEC-025
evidence sweep then truthfully FAILED on exactly one out-of-fence desktop
test — `apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`
(~line 381), the PKG-07/DEL-07-02 mirror of the same required-set assertion
class the T4 brief already cured in-fence. The W2 manager correctly reported
and routed rather than fixing cross-package. The branch is not pushable
(DEC-025 pre-push gate) until the mirror assertion reflects the landed
schema truth.

## Fast-reject screen (limits first)

- Not a scope/criteria change (class 4): the repair asserts the ALREADY
  LANDED schema truth equal-or-stronger (required triple plus existence of
  both exposure properties); it creates no new vocabulary or criterion.
- No lifecycle/stage/release act (class 5); no external effect (class 6);
  no merge-authority change (class 7) — merge occurs separately under the
  owner's transcribed session self-merge grant.
- Selection-principle basis: loop plan Step 1 (a)/(b) — repair failing
  validation for already-landed work; a gate prerequisite beats new scope.
  The owner lane directive (PKG-04/05/09) does not forbid a bounded repair
  whose sole purpose is landing in-lane work through its ruled gate.
- Bounded and reversible: one test file, one assertion cluster; ambiguity
  absent — the live schema is the single source of truth.

## Action

One bounded Agent 2 executor: update the mirror assertion to the landed
truth (equal-or-stronger: exact new required triple + both exposure
properties present), run the focused desktop vitest file, re-run the
registered `evidence-sweep` (expect PASS; exactly one new
`validation/evidence/sweeps/SWEEP_*.json`), append a DEL-07-02 MEMORY entry
and one run record citing this disposition, then HELP_HUMAN fan-in review,
commit, push, PR, merge (session grant), receipt.

The W2-reported sub-span GUI-emit follow-on (surfacing `exposed_spans` in
the desktop authoring UI) is OUT of this repair — recorded on the campaign
slate.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
