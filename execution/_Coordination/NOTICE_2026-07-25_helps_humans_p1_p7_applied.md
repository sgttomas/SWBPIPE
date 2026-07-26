# NOTICE — `AGENT_SCOPE_CHANGE.md` / `AGENT_AUDIT_DECOMP.md` variant-section binding changed, 2026-07-25

Routed to this surface under `AGENTS.md`'s agent-index change-notice rule. This
loop references `agents/AGENT_SCOPE_CHANGE.md` in nine `_REFERENCES.md` files
under `PKG-17_Export Format Interoperability` (`DEL-17-01` … `DEL-17-09`, the
SCA-004 controlled-amendment workflow bullet) and in
`_Coordination/_DECISIONS/D-29_scope_change_propagation_v0_2.md`.

**This is coordination, not authority.** The chirality-piping loop adopts,
amends, or declines under its own instruments and cadence.

**No hash gate fires.** Those nine references are unhashed prose bullets outside
the `_REFERENCES.md` "Authoritative Source Corpus" table; no `ExpectedSHA256`
row in this loop targets any file changed by this tranche. That is precisely why
this notice exists — corpus-drift detection would not have surfaced the change.

## What changed that concerns this loop

Both agent files' **Variant Section Binding** blocks now bind decomposition
sections **by heading text, never by section number**:

- `agents/AGENT_AUDIT_DECOMP.md` carries the normative spec — collect `##`
  headings; normalize (strip `## `, strip a leading `N.`/`NA.` section number,
  trim, case-fold); match in rank order exact → prefix → substring; on multiple
  hits at a rank take the earliest heading and report the ambiguity; when no
  rank hits, **halt and report an unresolved binding — never resolve by
  position**.
- `agents/AGENT_SCOPE_CHANGE.md`'s table dropped every `§N` and now names
  heading text only. Its lead-in makes the rule normative and points at the
  `AGENT_AUDIT_DECOMP.md` block.
- The `Change Register` row for SOFTWARE_DECOMP now names `Decision Log`
  **and/or** `Revision History`, with the instruction to read both when both
  exist. The previous `Change Log §8` citation was a dangling reference.

**Why this matters here specifically.** The four prior SOFTWARE citations
(Packages §3, Deliverables §4, Scope Ledger §5, Change Log §8) were wrong
against every live SOFTWARE document, including this loop's
`execution/_Decomposition/SOFTWARE_DECOMP.md`. This document was one of the
three the normalization spec was verified against, and it exercises two clauses
no simpler rule survives: its `## 9. Scope ledger summary` needs both the prefix
rank and case-folding (lowercase `ledger`), and it has no `Revision History` or
`Change Log` heading, so the Change Register resolves through the alternation to
`## 12. Decision log` alone. Under the new rule all of its bindings resolve;
under the old numbers, Packages, Deliverables, Scope Ledger, and Change Register
all mis-resolved.

Also changed in the same tranche, listed for completeness because this loop does
not currently reference them: the `scope-of-work` skill contract (INIT/CONVERT
branching, upstream-ID citation rule, matrix row semantics), 
`tools/query/count_workspace_state.sh` (two counting defects fixed — deliverable
count and history-substring state matching), 
`tools/validation/validate_semantic_pipeline_scope.py` (`--step init`), and
`tools/scope_of_work/derive_review_checklist.py` (stderr warning only; derived
JSON byte-identical).

## Follow-on for this loop

1. **No corrective act is required or requested.** No prior chirality-piping
   artifact is reopened.
2. **Future SCOPE_CHANGE / AUDIT_DECOMP runs** against this loop's
   `SOFTWARE_DECOMP.md` will bind by heading text and should resolve correctly
   where they previously did not. If a run reports an unresolved binding, that
   is the new contract working as intended — the correct response is to report
   it, not to resolve by position.
3. **If this loop wants a hash gate on these contracts**, the `_REFERENCES.md`
   bullets under `PKG-17` would need promoting into the Authoritative Source
   Corpus table with an `ExpectedSHA256`. That is this loop's decision; nothing
   in this tranche makes it.

Full record: `plans/helps_humans_tooling_consolidation_2026-07-25/APPLIED_2026-07-25.md`.
Proposals: same directory, `P1_*.md` … `P7_*.md`, approved by the owner
2026-07-25.
