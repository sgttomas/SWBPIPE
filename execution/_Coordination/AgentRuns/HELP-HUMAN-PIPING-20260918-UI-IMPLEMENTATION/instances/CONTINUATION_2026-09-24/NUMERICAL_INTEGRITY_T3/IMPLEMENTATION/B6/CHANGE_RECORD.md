# T3-B6: the reader items: change record

T3 (numerical integrity), node T3-B1/B6, the B6 half. Records live on the integration branch `codex/piping-numerical-integrity-20260926` (NUM). This package is the PR's only execution content.

## 1. What the PR contains

Eleven maintained files under `projects/chirality-piping/`. Their maintained diff from main equals B6's branch at `a7de2a918f`, and NUM carries the same blobs (`source_equality.py`).

| File | Change | sha256 (prefix) |
|---|---|---|
| `apps/desktop/src/features/results/retainedPrecision.ts` | +33 / −1: `baseHeaderCode`; TS's G7 header refusal carries the base readers' code | `7f9b47a99e67d78f` |
| `apps/desktop/src/features/results/retainedPrecision.test.ts` | +34 / −8: the 07k and 07m slices; N-3's test | `9d6071c331214cf7` |
| `apps/desktop/src/features/results/retainedPrecisionIntegration.test.tsx` | +9 / −4: the declared-difference pins only (five entries; the N-3 sentence absent) | `1581166bbd8b5501` |
| `core/analysis_runs/retained_precision.py` | +30 / −4: `validate_retained_precision_transport` and `_validate_draft(raw=False)` | `9d1156ed6dacce33` |
| `core/analysis_runs/compatibility.py` | +16 / −4: `_retained_contract(check_receipt=False)` runs the transport validator | `3d9d1678d99ccce2` |
| `core/reporting/result_export/tests/retained_precision_carriers.rs` | +47 / −4: RV92's tampered set; the declared-difference pins | `a801c71e5847b956` |
| `core/reporting/result_export/tests/retained_precision_contract.rs` | +56 / −2: the 07k and 07m slices, with id checks | `ef8b67ffb35638e3` |
| `fixtures/results/retained_precision_cases.json` | corpus 07l → **07m**: 8 entries appended (286–293); 277's TS expectation aligned | `c21112fdbfad37dd` |
| `fixtures/results/retained_precision_carrier_cases.json` | `scope` and `declared_differences`: F-U6b-2 and the N-3 sentence removed | `98a7213ac6b7e070` |
| `tests/test_retained_precision_contract.py` | +68 / −8: Python reads its own expectation; the slices | `199570eb3ae14746` |
| `tests/test_retained_precision_carriers.py` | +80 / −4: the transport tests; RV92's tampered set | `5ce6c06630c979be` |

## 2. What it does

1. **Mutation 277 as a one-entry slice** (RR:12697). All three harnesses run `g7_not_required_quality_enum_invalid`. The eight new 07m entries get the same slice, and each slice checks its entries' ids.
2. **TS's G7 header codes match Python's and Rust's** (RV94 N-3; PLAN decision 11).
   - TS used one code for every header refusal. Its new `baseHeaderCode` returns the base readers' code in Python's order.
   - This aligns the N-3 class at its full width (any case status, every part of the case rule) and four sibling classes that also differed undeclared: the quality level, the formulation basis, a non-object `contract_evidence`, and a `source_block_recovery` member.
   - ROOT confirmed the widening (RR "I86's SW probe accepted; I83's B6 return verified and ruled; …", ruling 2).
   - The N-3 scope sentence is removed, and the pins in all three languages assert its absence.
3. **F-U6b-2: Python's transport validator for the retained successor.**
   - `validate_retained_precision_transport` is the twin of Rust's `validate_transport_metadata` and TS's `validateRetainedPrecisionTransport`.
   - Python's transport dispatch runs it instead of refusing every transported successor, and the declared difference is removed.
   - T6S decision 8 holds: Python still refuses successor packages.
   - RV92's ten tampered successors are refused with the same codes in all three languages.
4. **RV94 N-5.** Python reads `expected_by_reader.python` where it is present, as Rust and TS read their own fields.

## 3. What it does not do

- **No change to PP or any D1 crate's `src`,** including the Rust reader. No schema, other fixture, dependency or lock change.
- **Outside the F2a D1 milestone's call graph,** so there is no Pass B.
- **Reader eligibility stays closed.** Every input whose code changes is refused both before and after; only the code aligns.
- **No existing corpus entry moves.** The only in-place value is 277's TS expectation, which item 2 requires.
- **RV78-N1** (the preview table's policy bindings) is not done here. It goes to B3's new table (RR, ruling 1 of the same section).

## 4. Review

- **I83 implemented it** (`R/I83/b6_01/RETURN.md`). Its 26 mutants plus a control: 24 killed, 2 equivalent on the corpus.
- **RV108 reviewed it independently** (`R/REVIEW_RV108/b6_01/REVIEW.md`), with its own oracles:
  - 1,032 G7 probes on 7 bases, through five entry points per language, at base and head;
  - 98 tampered-transport probes, on which the three languages agree;
  - a raw-path differential;
  - 30 mutants of its own.
- **Verdict: PASS,** 0 BLOCKING, 0 SHOULD-FIX, 7 NOTE.
- **The notes** are pre-existing reader differences and wording. ROOT routed them to B1's SR and SC lanes (RR "Owner decision: SI1c is option D, a repair within grammar 1.0.0; RV108 passes B6; RV109 passes ST with SF-1").

## 5. Gates

| Gate | Result |
|---|---|
| Independent complete-diff review (RV108) | PASS (0/0/7) |
| Suites, base against head | Python 1,896 → 1,922 (+26 added); Rust `result_export` 177 → 180 (+3); vitest 3,612 → 3,620 (+13 added, 5 renamed or dropped with F-U6b-2's forms; no outcome changed); `tsc` clean |
| `source_equality.py`, `check_citations.py`, GEN-8 on the exact head, hosted CI with the full-SHA dispatch, and the exact-head Mac DEC-025 against a fresh main baseline | See the merge record |
| Pass B | Not applicable: no file in the D1 milestone's call graph |

## 6. Open obligations carried forward

- **To B1's SC (corpus 07n):** declarations for the pre-existing differences:
  - I83 §7 item 6, and RV108 N3;
  - mutant T9's two-defect order;
  - entries pinning each header branch's sub-conditions (RV108 N5);
  - Rust's slice-id checks (I83 §7 item 8).
- **To B1's SR-PY:** Python's type and key guards (RV108 N1, N2).
- **To B1's SR-TS:** TS's `null` raw row on transport (RV108 N4), and the doc comment's wording (RV108 N6).
- **To B3:** RV78-N1.
