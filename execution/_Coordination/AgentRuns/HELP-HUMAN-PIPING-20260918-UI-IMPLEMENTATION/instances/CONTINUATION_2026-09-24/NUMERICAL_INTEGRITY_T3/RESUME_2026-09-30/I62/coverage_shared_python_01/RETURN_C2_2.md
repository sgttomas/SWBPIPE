# I62 return: checkpoint C2-2 (snapshot 06b, settlements, Python alignment)

**Basis:** the rulings at NUM 86c301bbbe, and I63/I64's `reader_audit_06a` RETURNs.

**Run window:** 2026-10-03T21:40:17Z to the 21:52:14Z freeze of SHARED_SNAPSHOT_06B, about 12 minutes of the 3-hour box. The memory guard (PID 5387) was running.

**Limits held:** no Git writes, no Cargo, no native job.

**Native citations** are FK/adaptive.rs and the other FK/PP files at CODE/NUM 652ad0cc1f, which has no diff from NUM.

## Changed READER files

| File | Before (06a) | Now |
|---|---|---|
| P/fixtures/results/retained_precision_cases.json | 58562c88dc… | e7983fc6415b3fe4df0b7a68e53a5cb9bb1a217b14d0c99dc1692b98733ce538 (4629135 B) |
| P/core/analysis_runs/retained_precision.py | 5bb6357a36… | ddf85962c3e5d7bca00b128e4fca7f6db7a4bd7ceb6958ae6e645baf89762593 (96916 B) |
| P/tests/test_retained_precision_contract.py | 514745e245… | 1c148103438bc4920ff823e57a35ef3cefa044354040677ed3dd6f4adc6c8463 (19768 B) |

**Final counts:** 15 cases, 163 mutations, 23 must-pass entries.

**Python tests:** `python_schema_H2`, **215 passed, 0 failed.** The brief's command ran with both BIN variables and a 1,200 s wall, and the input hashes match the final files.

## Python alignment to the native settlements

| ID | New Python rule | Native citation |
|---|---|---|
| N5 | A terminal stop ends on its exact `terminal()` translation, or on unresolved `work_accounting`. Escalating stops have no translation. | `terminal()` adaptive.rs:4349–4378; unreachable at 4377 |
| N9 | An escalating last stop with slots left ends only on WorkAccounting. Past the last slot it ends on the Ceiling or WorkAccounting. | the fault check precedes escalation, adaptive.rs:4545–4546 and 4581–4582; `finish_terminal` 4769–4800 |
| N10 | An idle run with group null is WorkAccounting (meter fault; takes precedence) or Budget(invocation) with `invocation_before` ≥ Li. After exhaustion, the idle run may also be WorkAccounting. An idle run in a ready group is refused `ledger_unavailable`. | adaptive.rs:4994–5016 (fault before exhausted); 5055–5075 (CasePrep `LedgerUnavailable`) |
| N11 | Unchanged: a refused group gives a refused terminal with that group's reason. | adaptive.rs:5030–5053 |
| N17 | Budget(case) requires case_charge > Lc. Budget(invocation) requires case_charge ≤ Lc and after > Li. | `test()` adaptive.rs:276–286 checks the case before the invocation |

## Settlements, with citations; Python aligned to each

1. **G7 code family: each language's own base code stands.**
   - The base contract fixes only the family: "A failure makes the source unsupported (Rust/Python error code prefix `SOURCE_PREVIEW_PHYSICS_`; TS equivalent)" (DEFAULT_ROUTE_DESIGN/IMPLEMENTATION/S1_INTERFACE.md:138, sha256 4288143bb363b1a5).
   - The existing base consumer tests match only that prefix (`tests/test_preview_physics_consumer_contract.py:394`: `match="SOURCE_PREVIEW_PHYSICS_"`).
   - The base validators differ:
     - Python `preview_physics_evidence.py:66` and TypeScript `previewPhysicsEvidence.ts:57` report `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: <detail>` for every check;
     - Rust `preview_physics_evidence.rs:85–91` reports `SOURCE_PREVIEW_PHYSICS_<CODE>`, here `EXTREMA_BOUNDS` (466–472).
   - C1:153 (G7 row) requires "existing base failure codes".
   - **Outcome:**
     - The G7 corpus expectation is per language: `expected_by_reader` = Python/TypeScript `SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID`, Rust `SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS`.
     - Each reader reports its own bare base code, with the detail carried separately.
     - Rust's provisional remap should be removed.
     - Python is unchanged (its base code is already `EVIDENCE_INVALID`).
2. **P5 covers every Conversion record**, including every PreparedMember's conversions, whether that member is prepared or refused. C3:182–195 states the kind/bits rule for Conversion outcomes generally ("Normal permits normal or exact ±0, Subnormal requires actual nonzero subnormal bits…"). Preparation endpoints are Conversions, and "an ambiguous-rounding failure preserves the distinct positive normal endpoint bits".
   - **Python:** `_conversion_kind_ok` now applies at G5 PRODUCT_ATTEMPT to all projection outcomes and all preparation conversions, as Rust and TypeScript already do.
3. **P7: bind every attempt, but only attached entries.** F1:101–106 says: "Public readers check old-input equality to PreparedMember.old_source wherever such an entry exists. … Old entries for not-yet-entered helpers … no dummy PreparedMember is created to bind them. Those unattached old operands/results remain producer attestations."
   - Together with C3:155–158 (old E/G = selected material; old-facts D/t = the request; old A/I/J = old-facts):
     - **every attempt**, with or without a source, binds each old tuple that has a PreparedMember (`inputs[6:10] == old_source[0,1,2,5]`, old E/G = the invocation's selected material, facts D/t = the request, and the internal old/facts equalities);
     - **unattached old entries are not bound.**
   - This narrows Python and Rust (which bound every sourceless old tuple, including positions) and widens Python's scope to sourced attempts (TypeScript's scope). Sourced attempts keep their existing source-path position binding.
   - **It moves one 06a entry,** listed below.
4. **O2: Rust's stricter rule is correct.** C2:166: "A per-case report_diagnostic_ref cannot be filled from an unrelated diagnostic. G5 validates typed initial/w2 evidence against this binding and exact diagnostic refs"; C1:100 ("exact existing diagnostics").
   - **Python:** every non-null typed reference (initial report or failure ref, w2 refs, formation refs, legacy_source ref) must be listed in the attempt's `diagnostic_refs` and name a diagnostic whose `affected_refs` contains the case.
5. **WorkAccounting `prior` is not on the wire.**
   - The closed schema Unresolved `work_accounting` is `{space, tag, fault}`.
   - C3:261–263 imports WorkAccounting with `{fault: Status excluding exact}` only.
   - C2's Reason map (C2:17–40) adds no prior.
   - Native `prior` (adaptive.rs:4352–4355, 4778–4789) is private.
   - Pinned by `work_accounting_prior_not_on_wire` (G1).

## Snapshot 06b (SHARED_SNAPSHOT_06B.json)

**Moved or changed 06a entries (by settlement only); everything else is byte-identical (asserted):**
- `prefix_old_inputs_unbound` was removed from mutations. Its unchanged edits are now must-pass `prefix_unattached_old_operand_attested` (P7: an unattached operand is attested).
- `g7_maximum_off_enclosure` gained `expected_by_reader` (G7). Its `expected` is unchanged.

**New bases (3, synthetic):**
- **`two_case_two_groups_synthetic`** (item 4): case 1 selects a named point with E,G ×2. It forms a separate stiffness group with its own cache in the same call. Rows, scales and classes are exactly derived by power-of-two scaling: displacements ×½, forces identical.
- **`ordinary_prepared_interpolated_material_synthetic`** (item 5): native interpolation is exact at 301 K.
- **`ordinary_prepared_mm_units_synthetic`** (item 5): mm normalization.

**New mutations (13), each at its intended Python check:**

| Group | Mutations | Expected first failure |
|---|---|---|
| N9/N5 over-acceptance (truncated skip base) | escalating end on Ceiling; escalating end on a terminal translation; wrong translation; wrong refusal kind | G5 ATTEMPT |
| N10 | idle budget below Li; idle ready group not `ledger_unavailable` | G5 ATTEMPT |
| `prior` | `prior` present on the wire | G1 |
| P5 | refused-member conversion kind/bits | G5 PRODUCT_ATTEMPT |
| P7 | attached old input unbound | G8 PREPARATION |
| O2 | report ref to an unrelated diagnostic | G5 ATTEMPT |
| C5 | distinct stiffness merged into one group | G5 ATTEMPT |
| G8 | named-point value mismatch; interpolation target mismatch | G8 PREPARATION |

**New must-pass entries (7), all using synthetic accounting triggers (standing rule):**
- the moved P7 entry;
- post-evaluator prefix (item 11, PP:3242–3245);
- observables failed after a passed certificate, and G5a failed after a passed certificate (item 14, PP:3529–3543);
- values failure with separate completion, aliases abandoned, and bind-rows abandoned (item 15, PP:3480–3491).

## Not built: stopped and reported

1. **N9/N10 positive must-pass entries and item 16 (a native WorkAccounting terminal row): contract conflict.**
   - C1 WIRE_CONTRACT.md:66 says "A checked inconsistency, max counter … prevents selected successor emission". C1:68 says "For an unencodable/saturation-ambiguous run, abandon successor finalization transactionally …".
   - A native WorkAccounting terminal exists only when some attempt's work status is not exact (adaptive.rs:4545, 4777, 4996). The receipt's native Work fields are exact U, with no unavailable variant.
   - So no emitted successor can carry such a run. These shapes are genuine natively but not representable in a receipt.
   - Python implements the ordered acceptance (N9/N10), with Python-only reader-logic tests.
   - **ROOT to rule:** keep acceptance, or treat a WorkAccounting terminal in a receipt as inconsistent per C1:66–68.
2. **Invocation-level G8 refusals:**
   - one un-normalized coordinate;
   - missing alpha;
   - duplicate temperature;
   - a strict bracket at a point.

   The shared format edits only the receipt source, so these remain Python-only tests. A shared control needs a format extension (invocation edits), which ROOT would have to rule on.

## Authoritative checklist status (Python, 06b)

These rows change from 06a; all others are as in RETURN_C2_1.

| ID | Python status | Evidence |
|---|---|---|
| N5 | checked | `terminal_stop_wrong_translation`, `terminal_refusal_wrong_kind`; unit test |
| N9 | checked (acceptance per native); positive base not representable (C1:66–68) | `escalating_end_requires_work_accounting`, `escalating_end_not_a_terminal_translation` |
| N10 | checked | `idle_budget_below_invocation_limit`, `idle_ready_group_not_ledger_refusal`; unit test (fault precedence, exhaustion) |
| N11 | implemented; no native-faithful base | — |
| N17 | checked in code (case-first budget rule); no shared base (≥20B/60B) | — |
| C5 | checked | `group_sources_out_of_order` (06a), `distinct_stiffness_merged_group`; base `two_case_two_groups_synthetic` |
| O2 | checked (strict) | `formation_d5_dangling_diagnostic`, `report_reference_unrelated_diagnostic` |
| P5 | checked (every Conversion) | `conversion_*` (06a), `refused_member_conversion_kind_bits` |
| P6 | checked | 06a, plus must-pass `values_failed_separate_completion`, `aliases_abandoned`, `bind_rows_abandoned` |
| P7 | checked (attached entries, every attempt) | `prefix_attached_old_input_unbound`; must-pass `prefix_unattached_old_operand_attested` |
| P9 | checked | 06a, plus must-pass `observables_failed_after_certificate`, `g5a_failed_after_certificate` |
| G7 | per-language base codes | `g7_maximum_off_enclosure` `expected_by_reader` |
| G8 material/units | checked | `named_point_value_mismatch`, `interpolation_target_mismatch`; bases I and M; invocation-level refusals Python-only |

## Open for ROOT

- The WorkAccounting representability conflict.
- The invocation-edit format extension.
- **Reader alignment:**
  - Rust: drop the G7 remap; narrow P7.
  - TypeScript: narrow P7 to attached entries.
  - Both: adopt `expected_by_reader` and the N10 ledger rule.
