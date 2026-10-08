# I90 B1-SR-RS, repair round 2: the three-reader alignment set in the Rust reader

TASK (Type 2), I90 (I-RS), for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-08 UTC.

## The basis

- **The brief:** `R/BRIEFS/B1_SR_RS_REPAIR_02.md`, sha256 `50b2ec7046142120cfda444d508c1c67be722c281bc6d139a7501d3922f30f94` (verified). I re-read `B1_COMMON.md` and `B1_SR_RS.md`.
- **The specification:** RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set ruled", items 1–4.
- **Read:**
  - RV113's SR-PY review, `R/REVIEW_RV113/rvr_sr_py_01/REVIEW.md` (`d8611e59…`, verified): §10, `evidence/fg/` and `evidence/repair_probes/`;
  - RV113's SR-TS review, `rvr_sr_ts_01/REVIEW.md` (`d44dec19…`, verified): S-1, N-1 and N-2;
  - TS's present code at I92's `7e47e51b5d`, as the reference forms: `coverage`, `ordinaryAttempts`, `validateRetainedPrecisionTransport` and `previewPhysicsEvidence.ts`'s `validatePreviewPhysicsTransportMetadata`;
  - PY's `preview_physics_evidence.validate_transport_metadata`.
- **For awareness, not acted on:** the coordinator's note about I98's `constructor_ordinal` producer defect. RS's G8 derivation is unchanged.

**State.** Done. One commit on `codex/piping-t3-b1-r-20261007` in `WT/b1-r`: **`6e3e4fe219`** over `b5cb7faaeb`. It touches two files in SR-RS's fence:
- RS, +390/−13;
- `RE/tests/retained_precision_contract.rs`, +1,007/−35.

`RE/src/source_blocks.rs` is unchanged. No 07m verdict changed, so no stop fired. Not pushed; ROOT pushes.

**Placeholders:** WT, NUM, P, PP, RE, RS, T, R, as in RETURN.md.

## 1. The changes and their evidence, item by item

### Item 1: (f) and its family at G3 COVERAGE; the ordinary attempt's basis at G5 ATTEMPT

**RS:**
- **`g3`:** the D29 source loop now also checks, for each source:
  - `index` equals its position;
  - `owner.kind == "case"`;
  - the case at `owner.case_index` exists and carries `owner.case_id`.

  A new loop over `material_bases` checks that each `index` equals its position and that `case_indices` is unique and names existing cases. These run bound and unbound.
- **`g8`** drops the moved checks: a source's `index` and owner kind, and a basis's `index`. It keeps the invocation-derived facts:
  - the owner id against the invocation's load case;
  - the source's basis by selector;
  - each basis's selector and exact case list;
  - the materials.
- **`g5_native`** keeps only the Call position's case binding (`src.owner.case_index == owner_ref.index`). Its `owner.case_id` conjunct moved to G3, which makes it implied there.
- **`g5_ordinary`**, first in each case's checks: the ordinary attempt's `material_basis_ref` must resolve to a basis whose `case_indices` lists the case (G5 `ATTEMPT_MISMATCH`, TS's placement). G8 still binds it to the selector's basis.

**Evidence:**
- **RV113's (f) table:** all 7 probes now read G3 COVERAGE bound and unbound. At `b5cb7faaeb`, six were G8 PREPARATION bound and admitted unbound, and the owner probe was G5 ATTEMPT.
- **The ordinary basis:** `r_b_basis_ref_7`, `r_c_basis_omits_case_1` and `r_c_missing_sourceless_basis` now read G5 ATTEMPT bound and unbound (they were G8 PREPARATION bound and admitted unbound).
- **Invocation-derived controls are unchanged:** `r_c_cases_out_of_order`, `r_c_extra_empty_basis`, `r_b_basis_ref_1_second_basis` and the `r_e_*` probes still read G8 PREPARATION.
- **Tests:**
  - `b1_r2_f_family_at_g3_bound_and_unbound`: seven rows, bound and unbound;
  - `b1_r2_ordinary_basis_reference_at_g5_attempt`: the basis missing, or not listing the case, at G5 ATTEMPT bound and unbound; the cases out of the invocation's order at G8 PREPARATION bound and admitted unbound.
- **07m:** `integral_float_integers_and_references` stays a must-pass (the census).

### Item 2: (g) at G8 INVOCATION_MISMATCH

**RS.** In the model-scope check (before any PREPARATION check):
- `combinations` and `components` must each be absent or `[]`. Before, they had to be `list(…).is_empty()`, which let null, an object or a string through.
- The existing rules stay as they were: no `reference_configurations` member, null included; and `pressure_contract` absent or null.

**Evidence:**
- **RV113's (g) table:** `g_combinations_null`, `g_combinations_object` and `g_components_string` move from admitted (eligible) to G8 INVOCATION. The five others already read as ruled: `reference_configurations` null or `[]`, `pressure_contract` `{}` or `false` refused, and `pressure_contract` null admitted. Unbound, all 8 are admitted.
- **Test:** `b1_r2_g_model_scope_at_g8_invocation`.
  - Refused: `combinations` null, an object or a string; `components` null, a string or an object; `reference_configurations` null; `pressure_contract` false.
  - Admitted: both lists `[]`; both absent; `pressure_contract` null.
  - Unbound: admitted.

### Item 3: C2's cause table at G5 ATTEMPT_MISMATCH, in the ordinary class

**RS.** In `g5_ordinary`, after O5, for every `unavailable` case whose cause is not `prepared_product_failure`. TS's form, with the `precondition` keying:

| Cause | Rule |
|---|---|
| `source_error` | phase `preparation`; code `source_unavailable`; no Run; a `source_decline` whose `error` equals the cause's |
| `unavailable_precondition` | phase `routing` or `preparation`; no Run; the code keyed one-to-one by `precondition`: `caller` → `caller_not_qualified`, `resource_admission` → `resource_admission_not_available`, `upstream_no_wrap` → `upstream_no_wrap_not_established`, `capture` and `source_family` → `source_unavailable` |
| `receipt_failure` | phase `receipt`; code one of `receipt_encoding`, `publication_hash_range` or `invocation_not_representable` (C2 keys none to `check`, so the set form stands) |
| `facade_failure` | phase `facade`; code `facade_certificate`; the case's Run is `selected`; `owner_ref` is the case itself |
| any other (a kernel reason) | phase `kernel`; a Run is present; code `kernel_<the Run's terminal kind>`; the cause equals the Run's terminal reason |

**Evidence:**
- **RV113's six C2 probes and `x_reason_cause_receipt_failure`:** RS now reads as ruled and as TS does.
  - `c2_receipt_ok` admitted; `c2_facade_ok` G5 PRODUCT_ATTEMPT (the table passes, then D19);
  - `c2_receipt_phase_kernel`, `c2_receipt_code_facade`, `c2_receipt_phase_preparation`, `c2_facade_phase_kernel` and `x_reason_cause_receipt_failure`: G5 ATTEMPT (they were admitted, or G5 PRODUCT_ATTEMPT, at `b5cb7faaeb`).
- **Tests:**
  - **`b1_r2_c2_cause_table_receipt_and_facade`**, by `validate`, on 07j's two-case base, with case 1 unavailable beside its Ready attempt and selected Run, and rows without a method:
    - admitted: `receipt_failure` with each of the three codes;
    - G5 ATTEMPT: `receipt_failure` with phase kernel, with code `facade_certificate`, or with phase preparation; `facade_failure` with phase kernel, with code `receipt_encoding`, or naming another case;
    - G5 PRODUCT_ATTEMPT: a satisfied `facade_failure` (D19 next).
  - **`b1_r2_c2_cause_table_source_precondition_kernel`**, through `reader_logic::ordinary`:
    - `source_error`: satisfied; with no decline, with a decline whose error differs, with phase routing, and with code `caller_not_qualified`;
    - `unavailable_precondition`: each precondition with its keyed code in both phases (admitted), and with each of the other codes (G5 ATTEMPT: 16 rows); phase kernel;
    - the kernel reason on F_BASE's case 1 with its own refused Run: satisfied; with code `kernel_unresolved`; with phase facade; with another cause.
- **Existing tests, changed** (their probes now state a C2-consistent phase and code where the test is about something else):
  - `d19_converse_cause_binding`:
    - the two `receipt_failure` probes now carry reason (`receipt_encoding`, `receipt`), and still reach D19 (G5 PRODUCT_ATTEMPT);
    - "a precondition beside the case's Run" now expects C2's G5 ATTEMPT, since a precondition cause cannot sit beside a Run;
    - the Ready-attempt row's precondition cause became a `facade_failure` naming the case, and still reaches D19;
    - the `receipt_failure` row still reaches G6.
  - `b1_d38_capture_before_any_run_beside_a_selected_case`: its "cause not a prepared product failure" row (`receipt_failure` on the (4b) case) now expects G5 ATTEMPT (C2 first), as TS gives (I92's own variant, RR "I92's SR-TS verified; …" ruling 2).

### Item 4: transport, with the header at G2 and the preview-physics metadata check added at G7

**RS.** `validate_transport_metadata` keeps the header check (`semantic_contract::for_source_metadata` on the projection) at G2. After it, it runs a new private `preview_physics_transport_metadata` on the same projection. A failure is G7 with `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID` and its detail (as `base_error` splits it). The check follows TS's `validatePreviewPhysicsTransportMetadata` and PY's `validate_transport_metadata`, check for check:
- no `source_block_recovery` or `carrier_evidence`;
- the formulation basis is exactly `{profile_id, limitations}`, with the profile and the preview-physics-1 table's `supported_profile_limitations`;
- `contract_evidence` is exactly `{preview_cases, combination_gates}`, both arrays;
- each preview case:
  - its exact shape and a unique non-empty id;
  - its maximum coverage: the shape, distinct non-empty pipe lists, no overlap, and `complete` exactly when both lists are empty;
  - its support attribution: the shape, distinct attributed ids, withheld records with a known reason, and no support both attributed and withheld;
  - its extrema: the shape, ids, the three basis constants, fractions in [0, 1], safe integers `span_index` ≥ 0 and `subdivisions` in [0, 131072], and 0 ≤ lower ≤ upper;
  - pipes distinct and outside the coverage lists;
  - intensified measures: the shape, ids, location, factor role, finite inputs, and a positive `sif` and section modulus;
- the attribution sets are the same in every case;
- extrema and measure result ids are distinct across cases;
- each combination gate: the shape, a unique id, a boolean `withheld`, and a known code when withheld, else a null reason.

A serde_json number is always finite, so TS's finite-tree walk has nothing to refuse in Rust.

**Why in RS.** The Rust base reader has no metadata-only entry: `preview_physics_evidence::validate_preview_physics_evidence` joins rows throughout. Adding one there would touch a base reader, which PLAN_v2 §1 makes a stop. So RS carries the check itself, within SR-RS's fence (§3, item 1).

**Evidence:**
- No 07m entry's transport verdict changed (the census), and no probe's either. The header refuses each of their header defects first, at G2.
- **Test:** `b1_r2_transport_metadata_at_g7`.
  - Admitted: the base, and a valid combination gate.
  - G7: `carrier_evidence` present; limitations other than the table's; an extra `contract_evidence` member; `preview_cases` not a list; a case missing a member; two cases with one id; coverage inconsistent; an extrema constant changed; a fraction above 1; a negative span index; subdivisions above the maximum; inverted bounds; a withheld record with an unknown reason; a support attributed and withheld; attribution sets that differ between cases; an intensified measure at `midspan`; a withheld gate without a code; two gates with one id.
  - A header defect (`numerical_quality.status`) stays at G2.

## 2. The census over 07m (`_run_records/repair_02/out/`)

RV113's own harness was used unchanged:
- `harness/rv113_census.rs`, sha256 `c0dadef9545d78d96b742b625bdbff65250943917c380ced0f71d9eef30d5018`, copied from `R/REVIEW_RV113/rvr_sr_ts_01/evidence/harness/`;
- it was added only to my scratch archives of `b5cb7faaeb` and of the head, never committed;
- each side was one cargo job through `t3_cargo.sh`.

It records, for every 07m entry, the input's sha256 and four verdicts: bound, unbound, transport and standing.

**`CENSUS 339 entries: 0 changes (input, bound, unbound, transport, standing) b5cb7faaeb -> head; 0 misses against the corpus's Rust expectations`** (`compare_r2.out`).
- The 339 entries are 17 bases, 294 mutations and 28 must-pass.
- The 0 misses are on the corpus's `expected_by_reader.rust` (else `expected`), the must-pass eligibilities and the bases' eligibilities.

The contract test's own printed outcomes agree: `census.py` gives 294 mutations and 28 must-pass with **0 changes** against I1 (`suites/census_r2_contract.out`).

## 3. RV113's probes (`out/PROBE_TABLE_R2.json`)

The 135 probes are:
- RV113's 103 (`rvr_sr_py_01/evidence/probes/probes_v5.json`, which include the six C2 probes);
- its (f)/(g) table (15, `fg/probes_fg.json`);
- its repair-01 table (17, `repair_probes/probes_rp.json`).

`harness/build_probes.py` rebuilds their concatenation from those records (sha256 `19613cfd…8c7a`).

**Every probe reads as its item rules, or unchanged where no item rules one (135 of 135).** 18 changed from `b5cb7faaeb` to the head, all ruled:
- 7 (f);
- 3 (g);
- 3 for the ordinary basis;
- 5 C2: `x_reason_cause_receipt_failure`, `c2_receipt_phase_kernel`, `c2_receipt_code_facade`, `c2_receipt_phase_preparation` and `c2_facade_phase_kernel`.

**Against TS's present results** (RV113's TS columns at `7e47e51b5d`), RS now equals TS on all three verdicts (bound, unbound, transport) for **120 of 135**. The 15 differences are TS's side of the set, which I92's SR-TS repair 01 (items 2–4) addresses:
- **8 N-6 transport probes:** RS refuses them at G2 with the header code, and TS admits them, having no header check yet (item 4).
- **2 compound N-6 probes:**
  - `n6_carrier_evidence_with_case_defect`: bound and unbound, RS reads G7 `SOURCE_NUMERICAL_CASE_INVALID` against TS's G7 `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED`, each reader's own G7 base code; on transport, RS reads G2 `SOURCE_NUMERICAL_CASE_INVALID` against TS's G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`.
  - `n6_contract_evidence_null_and_source_block_recovery`: bound and unbound, RS reads G7 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` against TS's G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED`; on transport, RS reads G2 `SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN` against TS's G7 `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`. Neither changed from `b5cb7faaeb`.

  The G7 base codes are per-reader, as declared; the transport parts follow item 4.
- **5 (g) probes** that TS admits and RS refuses at G8 INVOCATION, as ruled: `reference_configurations` null or `[]`, `pressure_contract` false, and `combinations` null or an object (item 2).

On the C2 probes, (f), the ordinary basis and every B1 shape, RS equals TS.

## 4. Suites against `b5cb7faaeb` (`suites/`)

| Suite | `b5cb7faaeb` | Head | Per-test difference |
|---|---|---|---|
| RE (all targets) | 187 ok | **193 ok** | **+6 new (ok):** `b1_r2_f_family_at_g3_bound_and_unbound`, `b1_r2_ordinary_basis_reference_at_g5_attempt`, `b1_r2_g_model_scope_at_g8_invocation`, `b1_r2_c2_cause_table_receipt_and_facade`, `b1_r2_c2_cause_table_source_precondition_kernel`, `b1_r2_transport_metadata_at_g7`. **Changed, same names, still ok:** `d19_converse_cause_binding` and `b1_d38_capture_before_any_run_beside_a_selected_case` (§1, item 3). Nothing else |
| PP (all targets) | 712 ok, 1 failed (the known Mac `t13`), 11 ignored | the same | none. Compared with `cc81e78801`'s run; RS's production code is the same at `cc81e78801` and `b5cb7faaeb` |
| PP's c = 1 pins | 3 ok | 3 ok | the six successor documents are byte-identical to `b5cb7faaeb`'s (`pins/compare_pins_r2.out`): milestone `ac6986b0…` and `6cd1d249…`; L = 0 `93c6c865…` and `dbb3d477…` |

Compiler warnings are the same lines. For PP, only cargo's order of the "(6 duplicates)" annotation between its two summary lines differs, as in I85's record.

## 5. Mutants (`mutants/`)

**The method:**
- a mutant schema in a scratch copy of the head (`mutant_schema_RS.diff`, made by `make_schema.py`);
- each mutant is a guarded edit, active only when `I90_MUT` names it;
- one cargo build through `t3_cargo.sh`;
- then RE's contract-test and lib-test binaries for the control and each mutant, each run in a T3 lock slot through `WT/tools/t3_slot.sh`.

**The control passes 102 of 102.** **All 20 mutants are killed by assertions:**

| Mutant | Killed by |
|---|---|
| F1 G3 source-index conjunct dropped | `b1_r2_f_family_…` |
| F2 G3 source-owner conjunct dropped | `b1_r2_f_family_…`: G8 PREPARATION bound, admitted unbound |
| F3 G3 basis-index conjunct dropped | `b1_r2_f_family_…` |
| F4 G3 `case_indices` uniqueness dropped | `b1_r2_f_family_…` |
| F5 G3 `case_indices` range dropped | `b1_r2_f_family_…` |
| F6 G5 ordinary basis reference dropped | `b1_r2_ordinary_basis_…` |
| G1 `combinations` back to `list().is_empty()` | `b1_r2_g_model_scope_…` |
| G2 `components` back to `list().is_empty()` | `b1_r2_g_model_scope_…` |
| C1 C2's table removed | `b1_r2_c2_…` (both), `d19_converse_cause_binding`, `b1_d38_…` |
| C2 `source_error` branch always passes | `b1_r2_c2_cause_table_source_precondition_kernel` |
| C3 precondition keying coarsened to TS's set of four codes | the same (the 16 cross-code rows) |
| C4 `unavailable_precondition` branch always passes | the same, and `d19_converse_cause_binding` |
| C5 `receipt_failure` branch always passes | `b1_r2_c2_cause_table_receipt_and_facade`, `b1_d38_…` |
| C6 `facade_failure` branch always passes | `b1_r2_c2_cause_table_receipt_and_facade` |
| C7 kernel-reason branch always passes | `b1_r2_c2_cause_table_source_precondition_kernel` |
| T1 the G7 metadata check not called | `b1_r2_transport_metadata_at_g7` |
| T2 the formulation-limitations check dropped | the same |
| T3 the extrema constants dropped | the same |
| T4 the gate reason rule dropped | the same |
| T5 the cross-case attribution rule dropped | the same |

## 6. Format

**rustfmt (1.9.0, edition 2021) is clean on this round's hunks.** `fmt/fmt_hunks.py` counts the `rustfmt --check` blocks that would change a line this round added or changed: **0 in RS and 0 in the test file** (`fmt/fmt_touch_*.txt`).
- **RS:** the file's total is 31 blocks, the same as at `b5cb7faaeb`, so no pre-existing formatting changed.
- **Test file:** 108 blocks against 111 at `b5cb7faaeb`. The three fewer are whole statements this round edited (the D38 variants array and D19's probes and asserts), which rustfmt formats as units.

## 7. Host

- **Cargo:** every cargo ran through `WT/tools/t3_cargo.sh` (`--locked --offline`). The mutant runs of built test binaries went through `t3_slot.sh`, in a lock slot. My jobs are in `cargo_jobs_i90_r2.log`.
- **One slip of mine, corrected.** I started the mutant chain while my evidence chain (harness, suites, pins) was still queued. Its build had begun waiting on slot 1 under the old single-lock wrapper. When ROOT's three-slot note arrived, the two chains could have run at once. To keep one heavy job of mine at a time, I stopped my own waiting chain before it took a slot: pid 69553 has a `WAIT` line and no `START`. I re-ran it, through `t3_slot.sh`, only after the evidence chain's last job had gone. I killed no other job.
- **Waits:** each job had one wait, ending when its process had gone. None of my waits remain.
- **Scratch and targets:** `TMPDIR` was in scratch. My scratch archives (`repair_02/{base,head,mut}`) and targets (`WT/targets/i90-b1-sr-rs/{harness-base,harness-head,mut-r2}`) are deleted. The `WT/targets/i90-b1-sr-rs` head and base targets are kept.
- **Records:**
  - no symlink and no `build` folder;
  - no junit output;
  - placeholder paths only;
  - screened for machine paths and the machine's host names.

  The 6.4 MB probe file is not copied; `build_probes.py` rebuilds it from RV113's records, and its sha256 is recorded.

## 8. For ROOT

1. **Item 4's metadata check lives in RS.** The Rust base reader has no metadata-only preview-physics entry, and adding one is a base-reader change, a stop under PLAN_v2 §1. So RS carries its own copy of TS's and PY's check. It stays in step only by review.
   - If ROOT wants one Rust source of truth, a later ruled scope can move it into `preview_physics_evidence.rs` as a `validate_transport_metadata`, as PY's module has. RS would then call that.
   - The raw path is unaffected: RS's raw G7 still runs the full base reader, whose checks include these.
2. **RS's production code changed** (`6e3e4fe219`). It rides in B1's one RS re-qualification (SQ's G5, at I5).
   - **The new or changed loops on the D1 call graph, for SQ's TEXT loop rules:**
     - G3's new loops over `sources` (with an index into `cases`) and over `material_bases` and their `case_indices`;
     - `g5_ordinary`'s per-case basis lookup (a scan of one basis's `case_indices`) and the C2 table (no loop);
     - the transport metadata check, which is outside precommit, since precommit calls `validate`, not the transport entry.
3. **For SC (07n),** RS now gives each item's ruled gate. The C2 rows that RS and TS already agree on can be pinned now. The (g) and transport rows wait for I92's round.
