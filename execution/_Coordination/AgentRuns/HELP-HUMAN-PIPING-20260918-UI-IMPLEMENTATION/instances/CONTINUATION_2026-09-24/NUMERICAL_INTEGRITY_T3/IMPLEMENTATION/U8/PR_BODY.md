**piping(T3): U8: real-input fallback witnesses, the L = 0 producer-solved successor and corpus 07l (test-only)**

This PR brings U8, the witnesses deferred from the F2a D1 milestone (#1082), to main. It is test-only.

- **Real-input fallbacks.** RV93 N-5's Candidate and Preparation fallbacks, and W-C1's Native fallback (which ends `Unresolved(Ceiling)`), are reached from real D1 inputs on the actual Direct entry, with no fault hook.
  - Each publishes the plain bytes plus exactly one notice, from one ordinary run, in both solver modes.
- **The L = 0 base.** The milestone plus one memberless, fully restrained node publishes a successor in both modes, in the registered dev/test build.
  - Its document, receipt and published bytes are pinned.
  - Body 1 is 6 input-derived rows plus 9 exact zeros. Body 0 is bit-identical to the milestone.
  - The two successor documents are committed as fixtures, byte-identical to the live output (D-U6-5).
- **Corpus 07l.** The fixtures become the corpus's first producer-solved bases, with 8 mutations and 4 must-pass entries on the L = 0 rules.
  - The Python, Rust and TypeScript contract tests pin the new counts and entries.
  - All three readers agree on every 07l document.

## What stays closed

- **No production, reader `src`, schema, build-identity or CI change.** The registered entry and M are unchanged (Pass B).
- **No Ceiling receipt row.** That is W-C2, in B1.
- **F-1, a Rust reader rule stricter than the contract on dense, range-scaled inputs,** is routed to B0 and B1. U8 does not pin it.
- **No native Current evidence, and no product caller.** Hosted CI (the Stale build) exercises the plain-route branches; the registered-build witnesses run on the owner's Mac.

## Source and packaging

- **7 files under `projects/chirality-piping`** (2 added, 5 modified; +29,026 / −14). They are byte-identical to the U8 head `bd6b4be2c3` and to NUM with U8 merged; `source_equality.py` checks this before the merge.
  - `core/product_physics/src/retained_facade_tests.rs`: three tests.
  - `fixtures/results/retained_precision_l0_successor_{sparse_interactive,dense_scrutiny}.json`: the L = 0 fixtures.
  - `fixtures/results/retained_precision_cases.json`: corpus 07k → 07l.
  - `tests/test_retained_precision_contract.py`, `core/reporting/result_export/tests/retained_precision_contract.rs`, `apps/desktop/src/features/results/retainedPrecision.test.ts`: the reader tests.
- **This branch is cut from main.** It carries none of the U8 branch's history, which stays on `codex/piping-f2a-u8-20261005` and NUM.
- **Records committed:** 4 files in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/U8/`: the change record, this body, the citation index and `SHA256SUMS`.
  - The two verification scripts are main's, from #1082's package.
  - U8's run and review records stay on NUM. Source citations resolve through `citations.json` to commit-pinned URLs.

## Reviews

- **RV97 reviewed the complete diff in two rounds:**
  - round 1, the PP tests and the probe: **PASS** (0 blocking, 0 should-fix, 5 notes);
  - round 2, 07l and the reader tests: **PASS** (0 / 0 / 2), with 0 discrepancies across 348 documents in the three readers.
- **RV98 confirms Pass B:** **PASS** (0 / 0 / 2). The L = 0 fixture rows are test-only (`not-d1`).
- **RV97 confirms this head's package and equality** before the merge.

These are agent reviews, not personal review by the owner. Details: [CHANGE_RECORD.md](`…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/U8/CHANGE_RECORD.md`).

## Gates

| Gate | Result |
|---|---|
| Pass B, fresh and fail-closed | The same verdict as on #1082's frozen head F. The only new outcomes are U8's three tests, all `ok`; entry, M and maxima are unchanged |
| G5–G8 | Carried by ruling: no production byte changes |
| Full 40-manifest suite | **PASS:** 38 of 40 manifests are identical to #1082's; the other two differ only by the added tests |
| `check_citations.py` | 10 resolved, 0 ambiguous, 0 unresolved at the U8 head; rerun on this head before the merge |
| `source_equality.py`, GEN-8, hosted CI with the full-SHA dispatch, and the Mac DEC-025 against a fresh main baseline | Run on this head before the merge, and recorded on the integration branch |

**Acceptance runs on the U8 head:**
- PP 708 passed, with the known Mac `t13` failure and 10 ignored;
- result_export 173;
- Python retained 480;
- TS `retainedPrecision.test.ts` 474;
- the desktop suite 3,574;
- `tsc` clean.

**Still open:** W-C2 and F-1 (B1), RV97's R2-N-2 (B1), mutation 277's slice (B6) and erratum E-6 (CHANGE_RECORD §7).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
