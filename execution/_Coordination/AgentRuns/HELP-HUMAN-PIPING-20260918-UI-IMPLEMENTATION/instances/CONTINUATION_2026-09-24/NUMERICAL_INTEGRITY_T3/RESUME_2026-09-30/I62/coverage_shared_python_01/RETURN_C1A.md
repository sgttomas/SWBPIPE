# I62 return: checkpoint C1a (snapshot 05a)

**Basis:** the ruling "Snapshot-05 plan approved in two parts; reader parity rules; F1a repair committed" (NUM 99e976e9a2).

**Run window:** 2026-10-03T20:18:06Z to the 2026-10-03T20:33:07Z freeze of SHARED_SNAPSHOT_05A, about 17 minutes of the 3-hour box. The memory guard (PID 5387) was running.

**Limits held:**
- No Git writes or index operations.
- No Cargo, solver, native job or install.
- I63/I64 files were not touched.
- C1b was not started.

Paths use the brief's placeholders.

## Changed READER files

| File | Before | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | 8e333e632c… (snapshot 04) | 159ef78c471875ec82878060b973b3c3a178248b821b4548e702c250d79f8ca6 (1657463 B) |
| P/core/analysis_runs/retained_precision.py | 94330e168f… | 27fc1797c2fecb9fc27dc1c99facdb8a260fa98ab725e6113caa616185cefc0f (76576 B) |
| P/tests/test_retained_precision_contract.py | de1c401503… | 7eab5b3793a504755313d3f3fb7236c4448f5a9f901112622f4734e90db3b5c9 (14149 B) |

- **Unchanged:** the schema (f943ebd351), preview table, results yaml, definition and schema test.
- **Snapshot 04 is preserved byte for byte:** all 77 of its mutations and all 3 of its cases are byte-identical. No existing expected first failure moved.

## Snapshot 05a contents (SHARED_SNAPSHOT_05A.json)

The snapshot JSON has the file hashes, the full tables with the expected gate and code (identical for all three readers), the Python observations and raising lines, the contract citations, the deferrals and the bulk listing.

**Format change.** There is a new top-level `must_pass` array, with entries `{id, base, edits, rehash:"all", expected:"pass"}`. A reader must accept each with the base case's classifications. `mutations` keep the snapshot-04 format.

**New cases (3), all labelled synthetic:**
- **Cancelled ±x loads** (`ordinary_prepared_cancelled_loads_synthetic`). It has a native witness at PP/retained_product_tests.rs:1460–1487. It carries has_data true, stop `[F;4]`, zero rows and one B.
- **Two-load-case unavailable templates.** A successor needs a selected case (C1 line 68; reader G3), so case 1 is the unavailable row next to snapshot 04's selected case 0. Case 1 repeats case 0's loads, so it reuses the stiffness group and cache. It uses the producer's later-case row-id convention and has no method token.
  - **F** (`two_case_facade_after_certificate_synthetic`): the certificate passed, then the adapter verdict copy was refused (PP:3522–3524). Synthetic accounting trigger; complete coverage.
  - **P** (`two_case_preparation_failure_synthetic`): preparation was refused before the first helper (PP:3165–3181). Synthetic allocation trigger; no proof, source or run.

**New mutations (27).** All produce their expected first failure in Python at the intended check:

| Group | Expected first failure |
|---|---|
| 6 promotions | the three layout controls at G5a SCALE; coverage-null plus product WORK at G5 PRODUCT_ATTEMPT; product WORK alone at G5 WORK; native WORK plus coverage null at G5 WORK |
| Parity rule 1: relabelled, dropped, reordered and foreign-body layout rows | G5a SCALE |
| Parity rule 2: empty roster on the no-data base | G3 COVERAGE |
| Parity rule 3: duplicate, missing and non-null-for-no-data record bound | G5a SCALE |
| Ladder: fresh-first p256 | G5 ATTEMPT |
| Cancelled base: no-data claim; B dropped | G5a SCALE |
| Unavailable rows: coverage null after a passed certificate | G5 PRODUCT_ATTEMPT |
| Unavailable rows: empty roster | G3 |
| Unavailable rows: free-load no-data claim, uncoupled stop, layout relabel, duplicate record bound | G5a |
| Unavailable rows: certificate failed before the summary, with G5a passed | G5 PRODUCT_ATTEMPT |
| Unavailable rows: K-lane failure with coverage; maxima abandoned with coverage | G5 PRODUCT_ATTEMPT |
| Unavailable rows: captured prefix with members | G3 |

**Must-pass entries (15).** All pass in Python with the base classifications:
- **Undetectable attestations:** stop `[T,T,F,F]`; all-false stop; no-data all-true stop; no-data attested data block; cancelled all-true stop; null after a predicate failure with observables/G5a not entered.
- **Failure rows on F:**
  - certificate failed after the summary (complete coverage);
  - certificate failed before the summary (defensive-only, synthetic trigger; null);
  - K-lane failure and Source-lane failure (null);
  - maxima abandoned, with completion merged (null).
- **Failure prefixes on P:**
  - captured prefix;
  - helper refused (synthetic accounting SectionError);
  - unequal helper/new prefixes (defensive-only, synthetic).
- **Complete old-Err/new-Ready** on the selected case, promoted from Python-only.

## Python reader changes (public API still disabled)

- **G5:** the ladder opens with a fresh p128 in record 0. Attempts are capped at 3, and records are empty if and only if attempts are.
- **G5a:** full canonical-layout equality, rebuilt from the source maps. Constraints must be unique and exactly +0. This aligns with I63.
- **G5a for unavailable attempts that keep complete coverage:** the canonical layout, feasibility, record bound/theta/`data_blocks` and the direct data facts. At p512, floor positivity is derived from the Run record as Φ>0 if and only if ê>0. No Selection rosters are applied.
- **G5b:** at p512 the floor must equal `phi_512(e_hat(E, L))`.
- **Already aligned:** the G3 empty-roster rule and the bound rule.

## Floor and ladder: contract citations

- **Floor.** C1 WIRE_CONTRACT G5b (line 150) requires "same E/ê/Φ at p512" with exact scale bits, and line 116 says the floor is present only at p512. So the check is required at **G5b SCALE_MISMATCH**.
  - It is implemented in Python, with a Python-only rounding unit test.
  - The shared mutation needs the C1b p512 base.
  - Rust has no Φ check (I read `retained_precision.rs`); Rust and TypeScript must add it.
- **Ladder start.** C1 §1 items 1–5 (lines 25–31) with the G5 row (line 148, "actual logical/native schedule") require it **implicitly**: skips happen only through recorded failed solves. It is implemented at **G5 ATTEMPT_MISMATCH**, as Rust does at `retained_precision.rs:825–827`. **ROOT should confirm this reading.**

## Tests

The brief's command ran from READER/P with both BIN variables and a 1,200 s wall. Final run `python_schema_C3`: **146 passed, 0 failed.** The input hashes match the final files.

## Deferred, reported early

- **Source-construction failure (PP:3249).** I found no case-specific natural constructor refusal that leaves the ordinary case solvable, and the row is not defensive-only. Not built.
- **Post-native unavailable.** The natural load-dependent case is the Ceiling (C1 line 49), which needs the C1b four-record reuse chain.
- **Flag-swap must-pass entries and swap failure controls:** they need two bodies or two proofs (C1b).
- **The Φ shared mutation:** C1b.

## Memberless node (read-only)

PP lib.rs:6549–6640 builds every model node, with no member-reference requirement, and I found no refusal. End-to-end admission is not established without executing the producer; C1b must confirm it before building L=0.

## Open for ROOT

- Confirm the implicit ladder-start reading.
- Decide on the source-construction trigger.
- Note that Rust and TypeScript need the G5b Φ check and the unavailable G5a direct checks.
