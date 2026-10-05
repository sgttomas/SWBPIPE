# I62 return: checkpoint B (Python reader coverage checks)

ROOT resumed I62 after accepting checkpoint A (ruling at NUM e02f02a93d). I62 had no descendants.

- **Run:** 2026-10-03T20:02:19Z to the 20:06:37Z freeze, well inside the 90-minute box. The memory guard (PID 5387) was running.
- **Limits held:** no Git writes or index operations; no Cargo, install, new tooling, solver or native job.
- **No shared file changed.** All five snapshot-04 files still hash to their SHARED_SNAPSHOT_04 values.
- **I63's concurrent edit was left alone.** It is in the Rust reader `core/reporting/result_export/src/retained_precision.rs` in READER.
- **Paths** use the brief's placeholders.

## Changed files (READER, inside the fence)

| File | Before (snapshot 03/04) | Now |
|---|---|---|
| P/core/analysis_runs/retained_precision.py | a9819e6a7a… | 94330e168f2a71a86a05102a091dd791e4266037680fd915c1e80e12be36dfc0 (71676 B) |
| P/tests/test_retained_precision_contract.py | ed910da858… | de1c401503f82d20ff6ffb2baf059ee4017aff456d6881f1ed628df4e658537a (14739 B) |

`test_retained_precision_schema.py` is unchanged (90bbd5660c). The public API stays disabled: `_IMPLEMENTATION_COMPLETE = False`, and `validate_retained_precision` still refuses at G0.

### Reader changes, by gate

Gate order and error codes are unchanged. Earlier original checks still win.

- **G1 and G2.** These come from the snapshot-04 schema through the existing pinned walkers: the closed nullable shape and the body U encoding. No code change was needed.
- **G3**, in the existing product-attempt loop. A non-null `summary_coverage` must list exactly the bodies of the attempt's source inventory, as `0..body_count-1` in ascending order. A null or invalid `source_ref` is left to G5.
- **G5**, new `_g5_coverage`. It runs in the C3 association pass after the existing native schedule, before the Ready block.
  - Ready, a completed certificate, a passed certificate, a completed G5a or a passed G5a each require non-null coverage. Null stays allowed on a failed certificate.
  - Non-null coverage requires all of the following:
    - the attempt's own `source_ref`/`run_ref`, equal to the case's source and the Run origin's source/owner;
    - a selected native Run;
    - lanes `[admitted_k, annular_source]`, both completed;
    - proof_start, projection, maxima, values and aliases completed;
    - a certificate entered, with stage completed or failed and check passed or failed.
- **G5a**, new `_g5a_coverage`. It replaces the draft's Cartesian body × kind roster and keeps the existing value ranges. It runs in the brief's order:
  1. Native p, the 2p verification record, and the existing floor presence and per-body rules.
  2. The canonical layout, rederived from the source maps. A row is input-derived if and only if it is the displacement row of a constrained DOF, and its kind matches the component. Every prescription must be exactly +0, so D=false.
  3. Extent through `_extent`.
  4. The 16-vector Boolean feasibility rule, with floor positivity ORed after the L≠0 coupling. This matches final_case.rs at NUM, sha256 3bc84bf113.
  5. The estimate/charge rederivation: L-coupled E hats; charge equals estimate below p512 and equals the force/moment stop bits at p512.
  6. The exact stop, estimate and charge lists.
  7. The certified-bound (B) list, with:
     - the record bound non-null if and only if `has_data`;
     - theta +0 on no-data bodies;
     - `data_blocks` 0 if and only if no body has data, and otherwise at least the true count.
  8. The direct data facts: no free DOF implies false; any individually nonzero original nodal term at a free DOF implies true.

  No final row feeds any coverage fact.

## Tests

The command is the brief's: from READER/P, both OPENPIPESTRESS_*_BIN variables set, `VENV python -m pytest -q -rA` on the two files, with a 1,200 s wall.

| Run | State | Result |
|---|---|---|
| B1 | reader changes only | 100 passed |
| B2 (final; `python_schema_B2.json` input hashes match the current files) | after adding the Python-only tests | 107 passed, 0 failed |

- **All 77 corpus mutations** produce their expected first failure, including the 30 earlier ones.
- **All three cases pass** with their expected classifications, including the no-data synthetic case.
- **Every earlier test passes.**

### Reason evidence (reasons_B2.json, the raising source line for each mutation)

| Mutations | Raising check |
|---|---|
| G1 controls | the schema shape check |
| G2 controls | the U encoding |
| G3 controls | the new inventory check (`attempt_owner_only`: the existing owner check) |
| `coverage_null_*` | `_g5_coverage` non-null requirement |
| `certified_bound_unbound_drop_existing_g5` | the existing G5 record binding |
| `certified_bound_duplicate` | the existing G5a uniqueness check, which comes first |
| `coverage_stop_forbidden_entry`, `coverage_stop_uncoupled_consistent_roster` | feasibility |
| stop, estimate and charge controls | their own list line (`charge_follows_stop_below_p512` at the charge line) |
| has_data/B controls | the B list |
| `coverage_no_data_claim_with_free_loads` | the direct free-load data constraint |

The six no-data mutations now fail at their I57 checks, not through the Cartesian check:
- the stop, estimate and charge controls at their own list lines;
- the two B controls at the B list;
- the coupling control at the estimate list.

### Python-only tests added (not shared corpus)

- **Four publicly consistent attestation rewrites must pass:** stop [T,T,F,F]; all-false stop; no-data all-true stop; no-data attested data block. This guards against over-rejection; producer custody or replay must catch such rewrites.
- **Three canonical-layout controls fail G5a at the layout or prescription line:** a force row marked input-derived; a constrained displacement not marked input-derived; a nonzero prescription.

## Deviations and open items

- **Not implemented: G5a direct checks for unavailable attempts that keep complete coverage.** Snapshot 04 has no unavailable or failed-certificate base, so they could not be tested. They should land with checkpoint C's bases.
- **Within-G5 ordering for dual defects is not pinned by the corpus.** The coverage association checks run before the Ready-block WORK check. I63/I64 parity on a coverage-plus-work dual defect is unverified.
- **Suggested for snapshot 05:** the three canonical-layout controls, as shared mutations. Their expected failure is G5a SCALE_MISMATCH.
- **No defect found in snapshot 04.**
- **Not claimed:** acceptance, eligibility, joint three-reader parity or independent review.

Bulk files are listed with sha256 and size in BULK_B.json. They are in WT/scratch/i62_coverage_shared_python_01/: the `python_schema_B1/B2` records and logs, `reasons_B*.py/json/log`, `reasons_layout_B.py`, `checkpoint_B_source.diff` and the `source_final_B/` copies.

## Files read in addition to the checkpoint A list (sha256)

| sha256 | File |
|---|---|
| 4a249ad480abab54bd8ea27cdb5e327aa466c69d39e0ede94623241f1606a538 | T3/ROOT_RULINGS_V1.md at e02f02a93d (the checkpoint A ruling) |
| 3bc84bf1138b227f759049d2f255a97e257978ff2cd1bbd95b22613515015634 | NUM/P/core/solver/frame_kernel/src/structural/retained/product_certificate/final_case.rs (coverage formula lines) |
