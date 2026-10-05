# D37's expected table, derived independently (RV79-N1)

The table pinned as `d37` in snapshot 07g's corpus comes from two sources only:
- the native sequence that leaves each stage record;
- C3's PublicError kinds.

It is not taken from any reader's code. I derived it before comparing it with I62's table (RETURN_07F §1) and the three readers. It matches all of them on the 25 records below.

## Source

**Stage transitions.** `PP/core/product_physics/src/retained_receipt.rs` (sha256 `2f48c0c5…`), lines 45–55:
- `enter` marks a stage entered;
- `completed` marks it completed;
- `checked(stage, i, passed)` marks it completed or failed;
- `fail_entered`, which runs on every error return, turns each entered stage into failed.

A published record therefore holds only three states: not entered (`-`), completed (`C`) or failed (`F`).

**Stage order:** preparation, native, proof_start, projection, maxima, values, aliases, certificate, observables, g5a.

**The sequence.** `PP/core/product_physics/src/retained_product.rs` at `844448112f` (sha256 `416b9600…`), the same bytes as `bee3dc07ca`.

**Preparation**, `prepare_owned_case`:
- it enters Preparation at :3313 and completes it at :3426;
- any error calls `fail_entered` at :3430, which leaves `F---------` (kind `preparation`).

**Native**, `solve_native`:
- it enters Native at :3453;
- a selected outcome completes it at :3466;
- any error calls `fail_entered` at :3467, which leaves `CF--------`. Which kind that record carries depends on whether a Run was recorded:
  - a nonselected outcome after the Run is recorded is `NativeUnavailable`, published as `native`;
  - any earlier error (a duplicate solve, a missing prepared source, an origin or capacity error) has no Run and is published as `capture`, case (a).

**`freeze_candidate`** (:3650–3760). On any error, `fail_entered` runs at :3749.

| Point in the sequence | Lines | Stage record | Kind |
|---|---|---|---|
| Before ProofStart: consumed attempt, missing owner, nonselected outcome, `bind_rows`, `prepared_specs`, MapWrite | :3655–3660 | `CC--------` | `capture` (b) |
| ProofStart fails | :3662–3664 | `CCF-------` | `proof` |
| Projection fails | :3666–3668 | `CCCF------` | `proof` |
| Maxima fails | :3668–3671 | `CCCCF-----` | `abandoned` |
| Values fails | :3671–3676 | `CCCCCF----` | `values` |
| Aliases fails | :3676–3681 | `CCCCCCF---` | `abandoned` |
| `bind_rows_view` fails, with Aliases completed and the certificate not entered | :3684–3685 | `CCCCCCC---` | `abandoned` |
| Certificate fails | :3686–3709 | see below | `proof` |
| Certificate passes, then the frozen owner or verdict copy fails | :3712–3719 | `CCCCCCCC--` | `capture` (c) |

**A failed certificate** marks the certificate failed. Then:
- if the verdict copy succeeds and covers every row (:3696–3703), Observables and G5a are each entered and checked, giving `CCCCCCCF` followed by one of `CC`, `CF`, `FC` or `FF`;
- otherwise neither is entered, giving `CCCCCCCF--`.

**After a passed certificate**, Observables and G5a are entered and checked (:3723–3728). If the case did not pass (:3734–3738), the kind follows the checks:
- observables failed gives `observable`, with record `CCCCCCCCFC` or `CCCCCCCCFF`;
- observables passed and G5a failed gives `g5a`, with record `CCCCCCCCCF`;
- both passed gives `numeric`, with record `CCCCCCCCCC`.

**The precharged commit** (:3740–3743) can still fail with every stage completed, giving `CCCCCCCCCC` as `capture` (d).

**Error kinds:** C3's PublicError, namely `preparation`, `native`, `capture`, `proof`, `values`, `abandoned`, `numeric`, `observable` and `g5a`.

## The universe of records (25)

The universe is every record the transitions above can shape:
- the eight pipeline stages form a completed prefix;
- at most one further pipeline stage is failed or not entered, and every later one is not entered;
- Observables and G5a are entered together, only after the certificate was entered, and each is completed or failed.

This gives 14 + 6 + 5 = 25 records:

| Group | Records | Count |
|---|---|---:|
| First open stage is one of the first seven | listed below | 14 |
| Certificate open | `CCCCCCCF` with `--`, `CC`, `CF`, `FC` or `FF`, plus `CCCCCCC---` | 6 |
| Certificate completed | `CCCCCCCC` with `--`, `CC`, `CF`, `FC` or `FF` | 5 |

The first group spelled out (records with the first open stage at position *k*, for *k* = 0 to 6):
- failed: `F---------`, `CF--------`, `CCF-------`, `CCCF------`, `CCCCF-----`, `CCCCCF----`, `CCCCCCF---`;
- not entered: `----------`, `C---------`, `CC--------`, `CCC-------`, `CCCC------`, `CCCCC-----`, `CCCCCC----`.

## The table

Every record in the universe that no kind lists is refused for every kind. Examples are `C---------`, `CCC-------` and `CCCCCC----`. An unknown kind (`storage`) is refused on every record.

| Kind | Records it can leave |
|---|---|
| `preparation` | `F---------` |
| `native` | `CF--------` |
| `capture` | `CF--------` (a); `CC--------` (b); `CCCCCCCC--` (c); `CCCCCCCCCC` (d) |
| `proof` | `CCF-------`; `CCCF------`; `CCCCCCCF--`; `CCCCCCCFCC`; `CCCCCCCFCF`; `CCCCCCCFFC`; `CCCCCCCFFF` |
| `values` | `CCCCCF----` |
| `abandoned` | `CCCCF-----`; `CCCCCCF---`; `CCCCCCC---` |
| `numeric` | `CCCCCCCCCC` |
| `observable` | `CCCCCCCCFC`; `CCCCCCCCFF` |
| `g5a` | `CCCCCCCCCF` |

Every reader's D37 test now checks its own predicate against this table: 10 kinds (nine plus `storage`) × 25 records, accept if and only if the record is listed:
- Python's `_g5_typed`;
- Rust's `error_stages`, in an in-crate test;
- TypeScript's new `@internal` `errorStageRecordAgrees`.

**Run presence (D4d) is a separate rule and is unchanged.** `native` needs a nonselected Run; `capture` (a) has none.
