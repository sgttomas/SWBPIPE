# RV80 — confirmation round 05: Rust `g2()` and D34 (snapshot 07e)

RV80 is a TASK (Type 2) dispatched by ROOT (HELP_HUMAN) for the final scoped check before reader acceptance (workflow §3). ROOT is the return path. RV80 did not write the change and had no descendants.

- **Candidate:** READER `63355a91d24dce246f2b85ee4601825824fd1dac`, archived into `WT/rv80/`. NUM is at `5c5d2a15c4`.
- **Files:**
  - `retained_precision.rs`: `06204326ac4658f84bbb198a70d632951491134c10aa363aeddf0789d038760c`;
  - tests: `e895305d11e7…`;
  - `lib.rs`: unchanged;
  - corpus 07e: `bbca15d940` (15 cases, 263 mutations, 22 must-pass entries).
  
  These match I63's RETURN_07E.
- **Diff reviewed:** `abcb16fd27..63355a91d2`. The source changed +21/−2 and the tests +114/−7.
- **Run window:** 2026-10-03 20:20–20:28 MDT (WT/rv80 deleted at 20:28), inside the 30-minute box. Memory guard PID 5387 was running.
- **Host:** the default toolchain only. Every command ran under `env -u DEVELOPER_DIR`, and the mutant script strips the variable. `CARGO_TARGET_DIR=WT/targets/rv80`, one Cargo job at a time.
- **No** Git writes, installs, new tooling or native jobs.

## Verdict

**PASS — 0 BLOCKING, 0 SHOULD-FIX, 2 NOTE.**

D34 is implemented as ruled, in both `validate` and `validate_transport_metadata`. That includes the two enum/const fields, and also a JSON-text `-0`. The refactor neither drops nor weakens a check: both removed `encoding()` calls are now the first step of `g2()`. The harness rehash matches the 07e format rule. The baseline passes 56/56.

## 1. D34 in Rust (RV80's own probes)

**The code.** `g2()` (RS:455–467):
1. runs `encoding(retained_precision, schema)`;
2. then scans **every** number inside `retained_precision`. Any value equal to −0 gives G2 ENCODING_MISMATCH.

Both entry points call it right after G1 and before the D32 normalization (RS:4220–4221 and RS:4272–4273). The scan covers only the receipt, as D34 states.

**Probe results.** Each probe ran through `validate` with the invocation, `validate` without it, and `validate_transport_metadata` (`PROBES.json`).

| Probe | Edit | validate (bound / unbound) | transport |
|---|---|---|---|
| PR29 | F′ attempt-1 error `g5a{sanity, quantity_kind: -0.0}` (enum field) | **G2** / **G2** | **G2** |
| PR30 | same with `0` (control) | G5 PRODUCT_ATTEMPT / same | Ok |
| PR31 | `g5a{lower, quantity_kind: -0.0}` | **G2** / **G2** | **G2** |
| PR32 | `"quantity_kind":-0` **as JSON text**, re-parsed after the rehash (canonical hashing writes 0, so the hashes stay valid) | **G2** / **G2** | **G2** |
| PR33 | `source_decline.constructor_counts.directional_springs: -0.0` (const field), on the shared `unavailable_attempt_under_source_error_cause` edits | **G2** / **G2** | **G2** |
| PR34 | the same shared entry, with `0` (control) | G5 PRODUCT_ATTEMPT (its expectation) / same | Ok |
| PR35 | `case_charge: -0.0` (a U field) | **G2** / **G2** | **G2** |
| PR36 | a results row value `-0.0`, **outside** the receipt | G8 (the mode-basis record's own rule) / Ok | Ok, so not a D34 subject, as scoped |

**Earlier probes.** PR1–PR28 keep their round-04 outcomes:
- the D32 value probes PR19, PR22, PR24 and PR28 validate;
- PR20 (−0 `source_ref`) gives G2;
- PR21 (boolean) gives G1, and PR23 gives G0;
- PR25 and PR26 (out of range) give G1;
- PR27 (forged identity under `0.0`) gives G1.

## 2. Diff review (`abcb16fd27..63355a91d2`)

**Source:**
- The two removed lines are `encoding(&source["retained_precision"], schema())?` in `validate` and in `validate_transport_metadata`.
- Each is replaced by `g2(source)?`, whose first statement is that same `encoding(r, schema())?` call. The −0 scan is the only addition, so nothing is weakened.
- The order G0 → G1 → G2 → normalization is unchanged.

**Tests:**
- **Removed:** the old doc comment on `index()`, the rehash condition "has `source_identity_sha256`", and the 259/21 counts.
- **Added:** `d34_negative_zero_anywhere_in_receipt_fails_g2`, `rehash_index_rule_07e`, and the 07e slice (263/22).

**The 07e rehash rule, read against SHARED_SNAPSHOT_07E `format_rule`:**
- **Index:** `index()` is a strict integral value. It must be a JSON number (never a boolean), finite, integral, ≥ 0 and not −0, which matches `rehash_indexing`.
- **Source identities:** they are recomputed for each **selected** case whose `source_ref` resolves. This matches the rule's "for each selected case", and replaces the old "has the field" condition.
- **Preparation hashes:** recomputed only when the attempt resolves and all its members are prepared.
- **Order:** preparations, then identities, then the publication hash, then the receipt hash, then the `after_rehash` edits applied literally.
- **Outcomes:** I63 reports all 263 mutations unchanged under both rules. RV80's baseline run confirms the suite passes on 07e (56/56).

## 3. Mutants for D32 and D34

`MUTANTS.py` (WT and the mutant ids as arguments) ran M45–M50 from round 04, plus three new D34 mutants:
- **M53:** the −0 scan dropped;
- **M55:** the `g2()` encoding walk dropped;
- **M56:** the transport path back to the bare `encoding()` call.

Each starts from the pristine source (`06204326`), which is restored and re-hashed afterwards (02:22–02:26Z).

| Mutant | Result | Reason |
|---|---|---|
| M45 normalization disabled | killed | `d32_…` (2) and must-pass |
| M46 normalization converts −0 | survived | **Equivalent:** `g2()` now rejects −0 everywhere before normalization (PR29–PR35) |
| M47 range guard dropped | survived | **Equivalent:** out-of-range numbers fail at G1, where the canonical hash is refused (PR25, PR26) |
| M48 `receipt_version` host-type test restored | killed | `d32_…` and must-pass |
| M49 limits host-type test restored | killed | `d32_…` and must-pass |
| M50 normalization widened to the whole statement | survived | **Unpinned scope**, as noted in round 04 (N1) |
| M53 D34 scan dropped | killed | `d34_…` |
| M55 `g2()` encoding walk dropped | killed | six tests: the shared and slice G2 pins, `d32_…`, and `negative_zero_wire_scale_remains_g2` |
| M56 transport path without the D34 scan | survived | **Unpinned** (NOTE N1). RV80's probes discriminate it: under M56, PR29, PR31, PR32 and PR33 give **Ok** on the transport path (`PROBES_UNDER_M56.json`). Only the U-field −0 (PR35) still gives G2, through `uint()`. |

**Totals:** 5 killed, 4 survived. M46 and M47 are equivalent, and M50 and M56 are unpinned.

## Findings

| ID | Severity | Location | Finding | Remedy |
|---|---|---|---|---|
| RV80-N1 | NOTE | RS:4272; tests | **D34 on the transport-metadata path works but is untested.** M56 survives, while RV80's probes show the unmutated reader rejects enum/const −0 on that path and the mutant admits it. Transport is never eligible, so the effect is only on the G2 report for metadata-only carriers. | Optional: extend `d34_…` to call `validate_transport_metadata` on the `quantity_kind` −0 case. |
| RV80-N2 | NOTE | RS:283–289 | Carried from round 04: the receipt-only scope of `integral_receipt` stays unpinned (M50). It is correct by code reading. | Optional, as before. |

## Commands and evidence

| Run | Command (from WT/rv80, default toolchain) | Result |
|---|---|---|
| Baseline | `cargo test --locked --offline --manifest-path WT/rv80/P/core/reporting/result_export/Cargo.toml --test retained_precision_contract -- --test-threads=2` | 56 passed (`BASELINE_TESTS.txt`) |
| Probes | same, with `--test rv80_probes` | PR1–PR36 (`PROBES.json`, `PROBES_HARNESS.rs`) |
| Mutants | `python3 MUTANTS.py WT M45 M46 M47 M48 M49 M50 M53 M55 M56` | 5 killed, 4 survived (`MUTANTS.json`) |
| M56 discrimination | the probe test with M56 applied by hand; source restored to `06204326` | `PROBES_UNDER_M56.json` |

**Harness changes this round:**
- `#![recursion_limit = "512"]` for its larger `json!` table;
- `from_mutation`, which applies a shared mutation's edits first;
- `text_replace`, for the JSON-text `-0`;
- the transport path.

**Locations:** `rv80_probes.rs` exists only in RV80's archive copy. Bulk logs are in `WT/scratch/rv80_reader_confirm5/`. WT/rv80 is deleted at the end, and WT/targets/rv80 is kept.

## Files read

| Identity | File |
|---|---|
| NUM `5c5d2a15c4` | T3/ROOT_RULINGS_V1.md, from "RV79 confirmation 04" to the end (D34; 07e) |
| NUM `5c5d2a15c4` | R/I63/review_repair_07/RETURN_07E.md; R/I62/review_repair_07/SHARED_SNAPSHOT_07E.json (`format_rule`) |
| READER `63355a91d2` | the Rust source and test diff; schema (`G5aError` sanity/lower `quantity_kind` enum; `constructor_counts.directional_springs` const) |

## Open for ROOT

Nothing blocking. N1 and N2 are optional.
