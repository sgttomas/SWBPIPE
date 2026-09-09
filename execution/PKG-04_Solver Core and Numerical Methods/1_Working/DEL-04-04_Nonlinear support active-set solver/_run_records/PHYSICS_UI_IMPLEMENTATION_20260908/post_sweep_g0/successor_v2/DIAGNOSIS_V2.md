# DEC-025 G0 complete successor diagnosis

Status: `COMPLETE_FOUR_PATH_CANDIDATE_HELD_FOR_FRESH_REVIEW_AND_OWNER_ACT`

This succeeds the preliminary V1 diagnosis and candidate without changing their bytes. RF V1 (`CHANGES_REQUIRED`, review SHA `17dde8ec770db7af923511a369d7ae167dad5cbbeef56b5779b791d4dd0a4e39`) correctly identified two gaps: the failing Rust assertion ran before the current-normal assertion, and the retained-spring runtime fixture plus Python/desktop consumers retained lagged values.

## Independent fixed-point evidence

A disposable harness used the existing public product API. At `mu=0`, explicit global-Z nodal probes of both signs measured the affine normal response `N=N0+Hq`; all probe cases retained friction state 3, one-way state 1, two iterations, convergence, and Dense/Sparse equality. Intersecting the six-decimal publication intervals and applying `q=0.01N` gives:

| Control | Case | Independently fixed friction | Independently bounded normal | Observed target normal |
| --- | --- | ---: | --- | ---: |
| retained spring | L-100 | `0.411203 N` | `41.120279 N` uniquely | `41.120279 N` |
| retained spring | L-200 | `0.205601 N` | `20.560139` or `20.560140 N` at the last publication digit | `20.560140 N` |
| historical no spring | L-100 | `0.489527 N` | `48.952719 N` uniquely | `48.952719 N` |
| historical no spring | L-200 | `0.244764 N` | `24.476359 N` uniquely | `24.476359 N` |

The retained L-200 normal is reported honestly as a public observation because the independent interval straddles one six-decimal boundary. Its friction result is independently unique. Target runs were observational backchecks and were not used to fit `N0` or `H`. Cross-coefficient runs are consistency observations only.

## Complete generated-fixture change

The authoritative command is `npm run generate:product-preview-mechanics`, registered in `projects/chirality-piping/package.json`; it runs the `core/product_physics/examples/preview_result.rs` producer. Redirecting that producer to a disposable candidate produced SHA `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` from current fixture SHA `f0a0ddcdc896df844e303b8278e0bbced295166dedf0f674a26014e5fe35bdee`.

Both files have 830 result rows and 31 diagnostics with identical row IDs, kinds, units, metadata structure, and diagnostics. Exactly 340 entries change: two summary values and the numeric `value` field of 338 result rows. These are the mechanically coupled displacements, reactions, element forces/moments/stresses, nonlinear deltas, both load cases, and combination rows expected from the changed sliding force. The structured inventory records every old/new field and category; no claim is made that all 830 values are unchanged.

The fixture is runtime data. `previewService.ts` dynamically imports it for browser runs with no model, the canonical fixture model, and invocation failure; sample-proposal fallback may also consume it. Python preview service reads the same file. The solver/public producer API and implementation bytes remain unchanged, while the bundled runtime asset identity changes.

## Minimum held candidate

The exact decoded patch SHA is `9210169b6968d7f319506cfb53b8d6bc71132b713fba8fa52736b2ee4bc8e8d7`; it passes `git apply --check` against the current lane and changes only:

1. `core/product_physics/src/lib.rs`: the existing Rust test block, with paired no-spring expected values and an exact publication relation assertion.
2. `fixtures/product_preview/invented_mechanics_result.json`: exact deterministic generator output, including all 340 changed entries.
3. `tests/product_preview/test_product_preview_service.py`: retained L-100 pair/comment and exact six-decimal relation.
4. `apps/desktop/src/services/previewService.test.ts`: matching runtime-fixture pair and relation.

Candidate post-image SHAs are Rust source `d24294d911ed584ee5907df35016901ab671779e4f7f375f4125ba3e6d462745`, runtime fixture `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`, Python test `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735`, and desktop test `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f`.

No live source, test, fixture, Git, lifecycle, or accepted-evidence state was changed. A new bounded Owner act covering all four paths is required before application.

## Proposed acceptance

Fresh review must verify the affine evidence, exact generator identity, complete 340-entry inventory, four-path patch, unchanged solver/public API, and runtime-fixture classification. After Owner authorization: apply the exact patch; reproduce the generator bytes; run the affected Rust both-mode test, full offline locked product crate, narrow Python fixture/service and schema/run-record consumers, narrow desktop service tests, formatting/type checks, and the production frontend build. Then freeze exact source/fixture/test hashes, obtain a post-application independent diff review, integrate through CHANGE, and rerun the clean DEC-025 five-surface sweep including the native/bundled artifact identity.
