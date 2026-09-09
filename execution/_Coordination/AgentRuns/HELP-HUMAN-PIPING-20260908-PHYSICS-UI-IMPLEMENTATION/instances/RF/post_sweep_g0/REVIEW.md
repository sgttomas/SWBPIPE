# RF post-sweep G0 candidate review

Verdict: `CHANGES_REQUIRED`.

## Findings

1. **The proposed one-test candidate and its diagnosis omit active downstream retained-spring goldens.** The active preview model sets `mu=0.01`. The Python service test at `tests/product_preview/test_product_preview_service.py:217-219` and desktop service test at `apps/desktop/src/services/previewService.test.ts:141-152` assert friction `0.411514` and current normal `41.120255`, but the existing six-decimal publication rule gives `round6(0.01 * 41.120255) = 0.411203`. Their comments also say the friction-law relation remains held, which no longer describes the accepted same-iterate behavior. The generated active fixture `fixtures/product_preview/invented_mechanics_result.json` carries the same lagged pair and a second lagged pair, `0.205757` / `20.560128`, for which the published relation is `0.205601`. It also carries dependent force deltas, source/friction resultants, five pipe-P-130 shear rows for each load case, and `0.514392` combination reactions. Because the changed friction force feeds the same global solve, an authorized regeneration diff must inventory every changed displacement, reaction, force, stress, summary, hash-dependent, and combination row; the review cannot safely limit the fixture repair to the named literals.

2. **The clean failure did not prove that the neighboring `48.952652` normal remains current.** The Rust test fails at the friction assertion on line 10726 before reaching the normal assertion on line 10745. The proposed patch still places the exact friction assertion before it collects and asserts the normal. A passing future run would establish both, but the present diagnosis overstates current-run evidence when it calls the normal “separately asserted” and “valid.” The candidate should be finalized from the separate read-only diagnostic result, capture both public values before exact comparison, and preserve an explicit relation assertion.

## Confirmed numeric and scope assessment

The no-spring derivation itself is sound, conditional on the returned current normal. The fixture coefficient is `0.01`, production rounds published results once with `round6`, and the published normal `48.952652` implies a full-precision interval `[48.9526515, 48.9526525)`. Scaling gives `[0.489526515, 0.489526525)`, which always publishes as `0.489527`. The prior stable friction record reports the transition `49.010116 -> 48.952652`; `round6(0.01 * 49.010116) = 0.490101`, so the old literal encodes the superseded previous-normal behavior rather than the accepted current-normal law.

The patch SHA-256 `30c4b8203a43d7b9e2979730dd5b80fa0994a86346ad43e34859a0aabf5ba9f5` has two hunks, both inside the `#[cfg(test)] mod tests` block that begins at line 9730 of `core/product_physics/src/lib.rs`. It only reshapes the friction assertion and adds an equality using the existing `round6`; it changes no production bytes, threshold, public API, diagnostic, or solver behavior. It preserves the loop-count, convergence, state, sign, normal-kind/source, metadata, and diagnostic assertions. The candidate is valid for the isolated no-spring Rust test once the paired expected values are independently frozen, but it is not a complete correction package.

No production regression is identified by this review. The observed `0.489527` is consistent with the intended same-iterate repair and inconsistent with the old previous-normal oracle. That assessment does not establish the unexecuted current normal or the retained-spring successor values.

## Required bounded correction scope

A concrete successor candidate should contain only these mutable product surfaces:

- `core/product_physics/src/lib.rs`: the existing test block only; capture the current normal and friction, assert their independently frozen values, then assert `friction == round6(0.01 * current_normal)` with no tolerance or new threshold.
- `fixtures/product_preview/invented_mechanics_result.json`: deterministic generator-produced bytes only, regenerated from the same frozen source and model. Review the complete generated diff, including both load cases, deltas, resultants, pipe forces, combinations, summaries, and any hash-sensitive consumers.
- `tests/product_preview/test_product_preview_service.py`: update only stale generated-fixture numeric expectations/comments and assert the published coefficient/current-normal relation for the retained-spring pair.
- `apps/desktop/src/services/previewService.test.ts`: make the corresponding narrow fixture-expectation/comment update and relation check.

The Python result-schema and analysis-run tests, desktop fixture loader, and GUI workflow spec consume the regenerated fixture but contain no matching hard-coded friction literals; they require validation, not preemptive edits. Immutable execution, evaluation, reproduction, witness, and prior-run records remain historical evidence and must not be rewritten.

The post-authorization gate should add deterministic fixture regeneration to a temporary output, exact generated-diff review, JSON/schema checks, the narrow Python and desktop fixture tests, and relation checks for both retained-spring load cases. Then run the affected both-mode Rust test, formatting, the full offline locked product-physics crate, freeze exact production/test/fixture hashes, obtain fresh independent review, integrate through CHANGE, and run a clean five-surface DEC-025 sweep. The existing proposed checks are insufficient without the regeneration and downstream fixture gates.

A new bounded Owner act is still required for these four paths before any candidate is applied or generated. Production code remains read-only. This review makes no source-application, lifecycle, acceptance, integration, or release decision.

Runtime attribution: independent read-only Agent 2 reviewer, parent `/root` (`HELP_HUMAN`), requested `gpt-5.6-sol` with high reasoning. The actual model identifier was not exposed. Delegation was prohibited and not used. No build, test, source, fixture, Git, or accepted-evidence write was performed.
