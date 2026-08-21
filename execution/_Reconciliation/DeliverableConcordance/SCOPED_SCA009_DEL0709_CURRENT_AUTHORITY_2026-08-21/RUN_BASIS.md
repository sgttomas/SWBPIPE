# Scoped SCA-009 current-authority reconciliation — run basis

**RunID:** `SCOPED_SCA009_DEL0709_CURRENT_AUTHORITY_2026-08-21`

**State:** `FROZEN`

## Activation

| Field | Identity |
|---|---|
| Activation authority | Owner-directed 2026-08-21 Piping iteration steer, N1 mechanical SCA-009 obligations |
| Frozen source basis | branch `codex/piping-sca009-ci-support-20260821` at `b1876a5e0f0083e10c0c18255cd92ed0079b63a2` |
| Current decomposition | revision 0.12; SHA-256 `a1bf1148b8b96aed83c8a042437b9714d543cb068070662b97397dc700da48a3` |
| Current scope change | accepted `SCA-009`; Vocabulary Annex SHA-256 `f005abbc0d55b047ee7c34628e169aeb3646cd139a9c2c214a33760df4d63c7c` |
| Supersession binding | `Supersession_Delta.csv` SHA-256 `d0714e661b03c0236cc33cccf6a2732dd2ac49a240a3776f24c26a1fb2d8ed34` |
| Historical concordance | `DELIVERABLE_CONCORDANCE_2026-07-11_1305/CLAIM_CONCORDANCE.csv` SHA-256 `32095986662b4932d9b1bf403e1756addb87dac3142b488f56acd926178361e0` |

## Exact scope

- new current-authority bindings for `DEL-07-09`, `SOW-077`, `DEC-094`, and the accepted SCA-009 Vocabulary Annex;
- touched `DEL-07-03` claims and the `DEL-07-03-R-005` / `R-006` ownership-landing supersession;
- boundary-adjacent claim rows only for `DEL-07-01`, `DEL-07-02`, and `DEL-16-01`;
- closure ceiling: `CURRENT_AUTHORITY_REFRESHED / IMPLEMENTATION_OPEN`.

No other historical claim row receives a fresh current-effect disposition.

## Historical-package preservation

The 2026-07-11 concordance package is immutable upstream evidence. This run
does not edit its rows or replace their historical dispositions. The targeted
rows in `TARGETED_CLAIM_REFRESH.csv` add a current-effect overlay bound to the
accepted SCA-009 snapshot.

## Current-effect rules

- DEL-07-09 owns the coverage contract and palette organization only.
- Implementation lands in the deliverables named by the Vocabulary Annex.
- DEL-07-03 R-005/R-006 remain documented GUI absences; SCA-009 changes their
  accepted ownership landing but does not close them.
- Boundary-adjacent DEL-07-01/02 and DEL-16-01 rows gain the SCA-009 routing
  context only; their historical claim dispositions and lifecycle remain.

## Write and effect fence

This run writes only this new immutable scoped sibling derivative. It does not
edit the historical package, decomposition, scope-change snapshot, DAG,
dependencies, deliverable content, lifecycle, product, implementation,
coverage closure, release, estimate, schedule, or professional-reliance state.
