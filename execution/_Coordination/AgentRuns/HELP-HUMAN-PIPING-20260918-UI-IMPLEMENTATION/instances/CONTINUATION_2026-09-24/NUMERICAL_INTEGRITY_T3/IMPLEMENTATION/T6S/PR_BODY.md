**piping(T3 T6S): desktop result and stress-neutral export of a retained-precision successor behind an exhaustive output policy; the v0.3 results dispatcher; RV95 N-5's masking test**

This PR brings the T6 successor-output slice (T6S) to main. It closes public activation's checklist item 4 before B8.

- **An exhaustive output policy.** The shared refusal becomes a per-route, per-surface desktop policy, which `tsc` checks for exhaustiveness.
  - Only the Result Export and Stress-Neutral Export panels admit a retained-precision successor, and only at numerically eligible standing with the live native capture.
  - The other 18 gated surfaces and the report package keep refusing, with reworded text. A route without an entry fails closed.
- **Result export.** The desktop result JSON of a successor takes Rust `derive_document`'s form.
  - The receipt and `contract_evidence` are copied whole.
  - Each `absolute_verified` or `not_covered` row is disclosed, not valued, with Rust's exact message and bound text.
  - It is byte-identical to Rust goldens for both pinned successors.
- **Stress-neutral export.** A successor's package carries its receipt whole, including in the transport header.
  - Each classified row withholds its unit witness under one of two new info codes.
  - There is no schema change. The package reads `blocked` until the owner's choice at B8.
- **The v0.3 dispatcher.** `results.schema.yaml`'s 0.3.0 arm is now a `$ref` to `results.v0.3.schema.yaml`, so it admits exactly what the version file admits.
- **RV95 N-5.** A public-API test pins the two layers that mask `source_blocks::integer`'s 2^53−1 bound.

## What stays closed

- **No product caller.** The native commands call PP's ordinary wrapper, so the opened panels are reachable only by tests until B8. Activation, native Current evidence and the native panel witness (G10's moved half) are B8's.
- **No PP, D1-crate `src`, embedded static, build-identity, lock or dependency change.** `results.v0.3.schema.yaml`, the readers and `src-tauri` are unchanged. One carrier changes in a comment only.
- **No standing token, invocation or producer-origin claim** in either export.
- **Python keeps refusing successor packages,** a declared difference routed to B6.

## Source and packaging

- **19 files under `projects/chirality-piping`** (9 added, 10 modified; +2,062 / −1,903). They are byte-identical to the T6S head `fdcdb5e024` and to the integration branch with T6S merged; `source_equality.py` checks this before the merge.
  - `apps/desktop/src/features/{results,result-export,stress-neutral}/`: the policy, the derivative and validator, the stress-neutral packaging, and their tests.
  - `core/reporting/result_export/tests/`: the golden test and N-5's test.
  - `fixtures/results/retained_precision_successor_derivative_{sparse_interactive,dense_scrutiny}.json`: the Rust goldens.
  - `schemas/results.schema.yaml`, `tests/test_result_export_v0_2.py` and `tests/test_results_dispatcher_v0_3.py`: the dispatcher and its tests.
- **This branch is cut from main.** It carries none of the T6 branch's history, which stays on `codex/piping-t6-successor-outputs-20261005` and the integration branch.
- **Records committed:** 4 files in `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/T6S/`: the change record, this body, the citation index and `SHA256SUMS`.
  - The two verification scripts are main's, from #1082's package.
  - The slice's run and review records stay on the integration branch. Source citations resolve through `citations.json` to commit-pinned URLs.

## Reviews

- **RV101 reviewed the complete diff: PASS** (0 blocking, 1 should-fix, 10 notes).
  - Closure held in every probe.
  - The TypeScript and Rust derivatives are byte-identical over 63 statements.
  - 891 non-successor step outcomes are unchanged.
- **The should-fix, SF-1:** the bound's text differed from Rust's `{:e}` on exact decimal ties.
  - The implementer's repair covers both 16- and 17-digit ties.
  - RV101 **confirmed** it on 185,401 Rust-computed words, with no new findings.
- **RV101 confirms this head's package and equality** before the merge.

These are agent reviews, not personal review by the owner. Details: `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/T6S/CHANGE_RECORD.md`.

## Gates

| Gate | Result |
|---|---|
| Pass B | Not applicable (ruled): the slice touches no D1 crate source, embedded static or reviewed input (CHANGE_RECORD §5.1) |
| `check_citations.py` | 2 resolved, 0 ambiguous, 0 unresolved in the dry runs; rerun on this head before the merge |
| `source_equality.py`, GEN-8, hosted CI with the full-SHA dispatch, and the Mac DEC-025 against a fresh main baseline | Run on this head before the merge, and recorded on the integration branch |

**Acceptance runs on the slice:**
- desktop vitest 3,590 (base 3,552; only added and renamed tests), with `tsc` clean;
- `result_export` 176 (base 172);
- Python +23 passed, all in the dispatcher test;
- 186 non-successor builder inputs byte-identical to the base.

**Still open:** NT-1 (PR-B1), NT-9 (S-I2), NT-7, NT-10 and two unused test seams (T6's later slot), and the owner's B8 choice on successor package readiness (CHANGE_RECORD §7).

🤖 Generated with [Claude Code](https://claude.com/claude-code)
