# Checkpoint 7 — T1 wave 2 and WP7 qualification of the candidate

- **Author:** the T1 WORKING_ITEMS manager, parent HELP_HUMAN (ROOT).
- **Branch:** `codex/piping-load-states-20260925`.
- **Frozen candidate:** `f3270ea79`. The merge base with main is `82b43f9bd`.
- **Paths:** WORKING_ROOT-relative. Machine records are in `_run_records/session5/` to `session7/`.
- **Status:** completed execution and local qualification. This is not acceptance or release. M10, M16 and M29 do not close here: the native witnesses and hosted CI are still outstanding (see Closure below).

## What landed since checkpoint 6

| Commit | Work | By |
|---|---|---|
| `c5ec4dcba` | **WP2 native persistence.** `model_document_migration.rs` retains 0.4.0 as authored: current, no migration, nothing injected, 0.4.1 and later refused. `lib.rs` persistence tests show a 0.4.0 model with its result round-trips byte-exactly. | TASK T1_WP2_NATIVE_PERSISTENCE |
| `61fb7b219` | **WP4 headless and CLI** (tests only). For `load-reference-1`: export, document and proof. For `load-reference-source-1`: an invocation-bound receipt and no canonical export (§11). Also covered: refusals, blocked 0.4.0 routing, pre-0.4 routes, the SF-1 fallback through the runner, and actual-artifact lanes with Python consumers. | TASK T1_WP4_HEADLESS |
| `64711fd94` | **WP2 desktop readers.** Lossless 0.4.0 TS types. TS readers for both identities, check for check, including the invocation-free joined ledger. Dispatch and standing: joined results are always `needs_recompute`. Saved-result standing, the AnalysisRun mirror and the browser 0.4.0 mirror. The shared desktop output refusal, routed to T6 (§12, §14). | TASK T1_WP2_DESKTOP_READERS; §14 panel reclassification by the manager |
| `5cf56e3de` | **Repairs of review findings F1–F3.** A joined forged-proof check. Identifier-only invariance extended to booleans, counts and a proposal. `deriveResultDocument` now reaches its own refusal. | manager |
| `6f42b9be5` | **WP3 native inputs.** Plain fields for reference configurations, expansion laws and per-case `analysis_state`, submitted through the typed operations with engine validate-only refusals. A blank 0.4.0 model. A read-only resolved-state block labelled from WP2 standing. Tolerant display of absent labels and diagnostics, writing nothing (§15, §17). | TASK T1_WP3_NATIVE_INPUTS |
| `f3270ea79` | **Literal `include_str!` paths** in `load_reference_source_contract.rs`. The DEC-025 pytest surface found the `concat!` form. | manager |

Records and rulings are §11–§17 in `T1_WAVE1_RULINGS.md`, with the briefs in `TASK_BRIEFS/`.

## Independent review

- **T1_WAVE2_REVIEW:** FINDINGS, none blocking. Explicit items 1–4 are CLEAR. F1–F3 are repaired, and F4 (host load) was resolved by ROOT.
- **Accepted mutant survivors (§16).** Of the WP2 desktop mutants, 32 survive, all unreachable or equivalent. The reviewer checked all 8 reader survivors and sampled 10 of the 24 ledger-port survivors, covering every stated reason.
  - Reader survivors: `loadReferenceEvidence.ts` `:505`, `:343`, `:513`, `:531`, `:538`, `:368` (equivalent) and `:226`; `loadReferenceSourceEvidence.ts:67`.
  - Ledger-port survivors: listed in `T1_WP2_DESKTOP_READERS/RETURN.md` §4.1.
- **T1_WP7_FINAL_REVIEW:** see the addendum below.

## Qualification on `f3270ea79`

See `_run_records/session7/SWEEP_RECORD.md` for commands, windows, load averages and the failed attempts.

| Check | Result |
|---|---|
| DEC-025, sandboxed surfaces 1, 2, 3 and 5; clean tree; 1.97.1 | **PASS.** cargo: 39 crates, 1634 passed, 0 failed, 1 ignored. pytest: 3023 passed, 32 skipped. vitest: 2822/2822 with no timeout. Build: pass |
| src-tauri | 114/114 |
| Local Chromium e2e, both projects (not surface-4 evidence) | 481 passed, 20 skipped. The skips are the pre-existing owner-gated `full-cohort-controller` tests |
| VP-STATIC scratch rerun | 507/507 in both modes, with runner I/O byte-identical to the recorded run |
| D-GOV-45 leak check | PASS: 754 records, 0 credentials |

The 30 s timeouts seen earlier on a loaded host did not recur on the quiet host. No timeout was raised.

## Open-work routing (one table; T1 claims none of these)

| Route | Item | Basis |
|---|---|---|
| T3 | Numerical eligibility of joined `load-reference-source-1` results, and reader re-derivation | §7 |
| T3 | Headless joined binding and canonical-document route | §11 |
| T3 | **S11B-1**, a known limitation, not blocking (see the details below) | ROOT ruling; T3 `codex/piping-numerical-integrity-20260926` at `61b228543`, `T3/REVIEW/S11_BACKCHECK.md` |
| T4 | M07 joint-element moment coupling. The unqualified joint refusal applies on every route, the 0.4.0 route included | S6 §6; `_T1_COMMON` Wave 2 |
| T5 | Support semantics of pass-through fields: `base_motion`, `device_reference`, locked-equivalent components, predecessor position sources | §17; WP3 RETURN A4 |
| T6 | Desktop output of `load-reference-1` and `load-reference-source-1` on every result-data surface | §12, §14 |
| T6 | Solver-mode custody on receipt-less identities (shared with physics-1 and preview-physics-1) | §13 |
| T6 | Vacuous typed-caller assertion in T0R's `result_envelope_binding.rs:674` | §16 F1 |
| T6 | `unit_round_trip_summary` does not list the 0.4.0 quantities | WP2 native RETURN |
| T9 | Stale-artifact collision when crates with different lockfiles share one `CARGO_TARGET_DIR` (serde_json 1.0.150 vs 1.0.151) | `_run_records/session7/SWEEP_RECORD.md` |
| T9 | src-tauri is outside the DEC-025 cargo sweep and hosted CI | T0R S6 §6 (M27) |
| UI-SUCCESSOR | Browser mode opening 0.3.0 (divergence from native predates T1) | §14 |
| UI-SUCCESSOR | Desktop import tolerance for product-shaped model documents (crash on open in every version; writes nothing). List of unguarded reads: WP3 RETURN A5 | §17 |
| UI-SUCCESSOR | Editing of the pass-through fields: `analysis_basis_override`, `mass_state_ref`, `base_motion`, `device_reference`, locked components, predecessor or entered position sources | §17 |
| UI-SUCCESSOR | Layout and visual work for the 0.4.0 inputs | T1_PLAN (D2) |
| ROOT (decided) | Language gap: the Python stress-neutral packager and the 10 committed joined packages accept both identities, but desktop export does not. This is the validation-harness path, kept by ROOT | §12 ROOT addendum |
| ROOT (decided) | 126 historical run records (sessions 1–4) carry absolute machine paths as provenance, not credentials. 14 are hash-bound. All stay byte-unchanged | ROOT option (i); Records note below |

**S11B-1 details.**
- **What.** In the prescribed-motion RHS `f − ΣK_fc·g`, the sum is formed in binary64 (`FK/structural.rs:603-606`, `FK/lib.rs:870-877`).
- **Effect.** Inert before 0.4. On the 0.4.0 support-motion route, member moments can be off by about 1–3e-9 relative, and the guard does not see it; the error is bounded at about one rounding step of the gross `K·g` term.
- **T3's probe.** Two 3 m members with equal settlement and a small moment gave 1.2e-9 at g = 5 cm and 2.6e-9 at 20 cm.
- **Fix.** T3 S11-K will use an exact reduced RHS, in FK files that T1 does not touch. There is no T1 code change.
- **Relation to VP-STATIC.** The admitted VP-STATIC support-motion cases matched at their relative 1e-9 criterion on this candidate. Other geometries or larger motions can exceed 1e-9 relative on member moments until T3's fix lands.

## Records note

126 historical run records in this folder, from sessions 1–4, mention absolute machine paths. These are provenance, not credentials, and the leak check passes.
- 14 of them are hash-bound: the issued briefs, `OWNERSHIP.md`, `STARTUP.json`, `generate_references.py` and one `.diff`.
- By ROOT's decision, all 126 stay byte-unchanged. New records use placeholders.
- The DEC-025 summary JSONs are commit-bound tool output and are kept verbatim, as T0R's were. Their only machine path is the Python interpreter path in the recorded commands. All other session-7 records use placeholders.

## Closure

M10, M16 and M29 stay open. Closing them needs all of the following on the merged candidate:
- hosted CI, including the full dual-viewport dispatch (surface 4) with a 40-character `target_base`;
- the WP7 final review CLEAR, or FINDINGS with none blocking;
- the owner's-Mac native witnesses (`T1_PLAN.md` §6; witness 2 now reads "open a headlessly authored, desktop-shaped 0.4.0 model; inputs shown").

## WP7 final review (`T1_WP7_FINAL_REVIEW/RETURN.md`, at `f3270ea79`)

**Verdict:** FINDINGS, nothing blocking. The reviewer found no path that publishes a wrong result as Current, eligible or exported without a diagnostic. Its targeted mutants were killed 3 of 3: the SF-1 fallback, the prescribed write-back and the observation coupling. The machine paths in its record were replaced with placeholders when it was filed here (ROOT). The dispositions below are ROOT's.

| Finding | Disposition |
|---|---|
| R1 (should fix): no consolidated routing | Done: the single table above |
| N1: `/opt/pw-browsers` in the WP3 RETURN and e2e log | Replaced with `<PW_BROWSERS>` in both. These are T1 wave-2 records; no committed file references their hashes |
| N2: the checkpoint 6 counts have no retained logs | Recorded. The checkpoint 7 sweep on the candidate supersedes them, and its logs are retained (`_run_records/session7/logs/`, placeholders only) |
| N3: the F2 proposal case uses a fixed fixture | Note. It is still sufficient for §14 |
| N4: `toPayload` treats a value-only row as a quantity (misleading message; nothing sent, nothing defaulted) | Fixed locally after the freeze. The fix is `loadStateAuthoring.tsx:41`: a record is a quantity only when its `value` is scalar. It has a test in `LoadReferenceStateInputs.test.tsx`. ROOT is to have it narrowly reviewed |
| N5: the headless-artifact pytest is gated on environment variables | The DEC-025 sweep did **not** set `HEADLESS_LOAD_REFERENCE_OUTPUT_DIR` / `…_SOURCE_OUTPUT_DIR`, so those 15 tests are among its 32 skips. Separate run: full headless suite with both set (83 passed; 28 + 60 artifacts), then `tests/test_load_reference_headless_artifacts.py`: **15 passed** (`_run_records/session7/logs/n5_headless_artifacts.txt`) |
