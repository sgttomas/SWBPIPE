# BACKCHECK — T1_WAVE1_REVIEW_B repairs

- **Role:** the same fresh-context, non-author reviewer as `RETURN.md`. I wrote none of the repair bytes and did not delegate.
- **Requested by:** the T1 WORKING_ITEMS manager.
- **Git:** no writes.
- **Where I ran:** a shared, sparse scratch clone checked out at the committed revisions.
- **Candidates:**
  - WP3 at `bfb0fe3ce`;
  - F4 at `cad59d01e`;
  - WP5 at `ab5919133`, the branch head. Its operation_applier and package bytes equal `bfb0fe3ce` and `cad59d01e`.
- **Host hold:** ROOT paused heavy cargo work. I ran **no cargo build or test**, so the WP3 part below rests on code reading and the author's records (§4 lists what is pending). rustfmt is not installed for toolchain 1.97.1, so I could not check formatting.

## Verdict

- **WP5 (F3, F5 adapter, F8): CLEARED.**
- **F4: CLEARED.**
- **WP3 (F1, F2, F5 applier, F6, F7, F9): CLEARED ON READING, cargo confirmation pending.** I judge the F9 extension sound (§1).
- **New findings:** no blocking or medium finding. Three Info notes (§3).

## 1. WP3 at `bfb0fe3ce` (code reading)

File hashes equal the Addendum 2 table:

| File | sha256 |
|---|---|
| `pressure_authoring.rs` | `947e2961…` |
| `rich_authoring.rs` | `168ec01d…` |
| `load_state_authoring.rs` | `6f53188b…` |
| `lib.rs` (unchanged) | `4f088d67…` |
| `tests/load_state_authoring.rs` | `e32bf54f…` |
| `tests/load_state_delete_control.rs` | `35d12802…` |
| `load_reference_authoring_control.json` | `2e21291c…` |
| `load_reference_delete_control.json` (unchanged) | `1da7ef35…` |

**F1: resolved.**
- `pressure_authoring::resolve` refuses `Model/pressure_profile` on any 0.4.0 model, with `OP-PRESSURE-PROFILE-SCHEMA-VERSION-LOCKED`. The refusal comes after the owns and envelope checks and before the target and before-value checks.
- `pressure_authoring` was the only writer of `schema_version`. With this refusal, no operation can change the version of a 0.4.0 model.
- The 0.3.0 → 0.4.0 direction was already refused (`OP-PRESSURE-PAYLOAD-INVALID`, pinned in the new control).
- The test covers the downgrade, the unchanged profile, and an atomic batch refused as a whole, with a control batch that applies and stays 0.4.0.
- An unchanged (no-op) profile on 0.4.0 is now refused as well. That is deliberate and harmless (Info I-2).

**F2: resolved.**
- `refuse_point_orphans` is called in the rich `Material/temperature_points` path after `validate_temperature_points`.
- It follows the `refuse_orphans` rule:
  - only elements whose `material_selection.material_ref` is the target material count;
  - a reference is orphaned only if it resolves before the edit and not after;
  - the code is `OP-LOAD-STATE-INBOUND-REFERENCE`.
- A no-op replacement cannot orphan anything, because its after-value is the current points.
- No other path replaces `temperature_points`: `constitutive_properties` has a closed key list, and the shear-modulus-by-point-ID edit keeps IDs.
- The test covers:
  - refused: remove cold, rename cold, remove hot, with all four element paths named;
  - admitted controls: records stripped; a second material with the same point IDs; already-unresolved references.

**F5 (applier): resolved on reading.** The new tests target R01, R02 and R04 directly:
- **R01:** an integer 20 is respelled as `20.0`. The text is canonically equal, so nothing may be written, and the plain serialization must survive. The mutant's write would store `20.0`. The witness carries `"value":20}` in all three payloads, which the test asserts.
- **R02:** a `point_ref` that exists only on a second material is refused.
- **R04:** a support named only in the second case is refused, and the first case is not named.

The author reports 42 of 42 mutants killed, including my R01, R02 and R04 in their exact text. I have not re-run them (cargo hold).

**F6: resolved.**
- `displacement_mm` returns [sparse, dense], and every call site checks both against the prediction (`close_both`).
- `connected_middle_ux_mm` uses the general series form, and asserts that the far anchor has no entered motion.

**F7: resolved.** It is documented at `NOT_PRESENT`. The cited product code `LOAD_STATE_EXPLICIT_NULL_UNSUPPORTED` blocks explicit nulls.

**F9: resolved. The extension beyond my wording is sound.**
- A 0.4.0 model with the exact pressure contract is now the exact profile (E/ν pair).
- Its points no longer need `thermal_expansion_coefficient` to be complete.
- I checked this against the product:
  - `case_state/material.rs` states "Recorded G and alpha are never consumed as E/nu": 0.4.0 material selection reads E and ν only.
  - `case_state/resolve.rs` blocks a consumed legacy thermal primitive (`LOAD_STATE_LEGACY_THERMAL_PRIMITIVE_UNSUPPORTED`).
  - It also blocks case-wide `modulus_basis_ref`/`modulus_basis_temperature` on 0.4.0, so the loosened bracket requirement in `validate_temperature_points` cannot matter for a 0.4.0 model.
- A point's α is therefore not a 0.4.0 input, and demanding it only produced a spurious warning.
- The change is only a warning, so it cannot admit a value the product would consume.
- Pre-0.4 logic is unchanged on reading: `load_state` is false, so `point_thermal_field` stays `Some`. The new golden, blessed from `203396e4d`, pins 54 outcomes on 0.2.0 and 0.3.0 models.

## 2. WP5 at `ab5919133` and F4 at `cad59d01e` (executed)

### Adapter suites (committed bytes)

- `test_qualification_load_reference`: **55 OK**.
- `test_qualification_gate`: **31 OK**.
- `test_qualification_physics*`: **42 OK**.

### F3: resolved

- Runner stdout is admitted up to the run's selected `output_limit_bytes`. That applies to the stdout read, the parse, the reader snapshot bound, the helper's `--source-limit-bytes`, and the retained helper stdin with its custody `byte_limit`. The helper timeout is `max(60, run limit)`. Bound input files keep 8 MiB. `capture` enforces the same limit on the runner's stdout, so nothing is truncated silently.
- `strict_json_limited` is the gate's `strict_json` body, token for token, with a caller-selected bound. At or below 8 MiB it calls the gate function itself.

### F5 (adapter): resolved

I re-ran my own `adapter_mutants.py` unchanged, plus six F3 mutants of mine (Y01–Y06), on the committed bytes. **All 18 were killed:**
- X01–X12;
- Y01: stdout capped at the gate limit again;
- Y02: large-path float hook dropped;
- Y03: large-path duplicate refusal dropped;
- Y04: parse bound dropped;
- Y05: snapshot bound dropped;
- Y06: helper not given the selected limit.

Log: `_run_records/backcheck/adapter_mutants_backcheck.log`.

### F8: resolved

- The pin comment is current.
- The test is renamed `test_pinned_identities_match_recorded_bytes`, and it checks the pin against `1ccca8b87`.
- The reader at head is `14e1750e…`, equal to `git show 1ccca8b87`. Its top-level imports are unchanged and inside the helper allowlist.
- **Re-pin provenance.** Replacing the re-pin lines in the committed adapter and test with their `bfef71b19` forms reproduces the author's §11 final hashes exactly (`ea79022f…`, `6613e9f0…`). So the committed bytes are the author's final bytes plus only the manager's re-pin (`_run_records/backcheck/repin_reproduction.log`).

### Admitted package in the adapter

The package at head still passes the adapter's own admission: 14 cases, 467 positive and 40 negative, in both modes and both purposes.

### F4: resolved

The comparison was `c1e130818..cad59d01e` (`_run_records/backcheck/f4_backcheck.*`):
- 18 package files changed: 14 criteria files, `MANIFEST.json`, `PROVENANCE.json`, `README.md` and the generator.
- In the criteria files, the rule `review` string is the only difference. It is one admitted text: "reviewed in the independent WP6 freeze named in ADMISSION.json … not a release criterion".
- References, selectors, runner inputs, preview requests and `ADMISSION.json` are unchanged.
- The generator reproduces all 28 `cad59d01e` files with `ADMISSION.json`, and all 28 `6824b6b6b` files without it. So values and rules stayed invariant through admission. `--check` shows 28 files, 0 differences.
- `MANIFEST.json`: all 84 bindings match.
- `PROVENANCE.json`:
  - status is `admitted_after_independent_freeze`;
  - the generator hash is current, and all 28 output hashes are current;
  - the new `admission` block binds `ADMISSION.json`, the freeze RETURN and `apply_R1.py`, all current;
  - it records `author_package.py` as superseded for R1;
  - the authoring records keep their original hashes, which are still valid.
- The README states the admission, how to reproduce it, and the supersession.

**Admitted-state checker** (`_run_records/session4/check_package_admitted.py`): **4535 passed, 0 failed**. Its only difference from the WP6 checker:
- the two pending/draft checks per case are replaced by three admitted checks: criteria `reviewed`, the reference naming the freeze by path#sha256, and admitted rule review text;
- a new group P, which checks PROVENANCE currency.

The count reconciles: 4489 − 28 + 42 + 32 = 4535. The original WP6 checker still gives 4461 passed, 28 failed, the pending invariants only, as expected.

## 3. New notes

| # | Severity | Location | Note |
|---|---|---|---|
| I-1 | Info | `LSI/T1_WP5_HARNESS_ADAPTER/RETURN.md` §11 (F8 paragraph and the "Final files" table) | The text says the pin is `bfef71b19` and unchanged, and gives `ea79022f…`/`6613e9f0…`. The committed bytes carry the manager's later re-pin to `1ccca8b87` (`ea40ad64…`/`f7040597…`). The commit message records the re-pin, and I reproduced the difference as exactly the re-pin, so this is record accuracy only. A one-line note in §11 would close it. |
| I-2 | Info | `pressure_authoring.rs` | A no-op `pressure_profile` on a 0.4.0 model is now refused, not accepted as a no-op. The addendum says this is deliberate. It is harmless unless a client sends no-op profile edits for 0.4.0 models; the desktop 0.4.0 work comes after T0R. |
| I-3 | Info | `qualification_load_reference.py` `strict_json_limited` | It duplicates the gate's parse rules because `qualification_gate.py` is outside WP5's write boundary. If the gate's rules change later, the two can drift. The equivalence test covers 12 documents. A future gate-owner change could add a `limit` parameter to `gate.strict_json`. |

## 4. Pending, and limits

**Pending under the host hold** (operation_applier cargo). I will run these when the manager lifts the hold:
- the crate suite (expected 193 passed);
- regenerating `load_reference_authoring_control.json` and `load_reference_delete_control.json` from `203396e4d` with the base code, and comparing bytes;
- re-running my R01, R02 and R04, plus mutants for F1 and F2.

**Not done:**
- rustfmt, which is not installed for 1.97.1;
- any product or VP-STATIC run;
- WP1 `1ccca8b87`, beyond its reader hash and imports (REVIEW_A owns it).

**Cleanup:** the scratch clone was deleted.
