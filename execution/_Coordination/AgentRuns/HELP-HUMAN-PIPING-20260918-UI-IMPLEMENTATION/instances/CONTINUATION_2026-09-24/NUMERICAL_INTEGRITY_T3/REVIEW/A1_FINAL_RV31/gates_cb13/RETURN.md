# RV31 A1 external-gate confirmation

**PASS for the external-gate bindings on cb13dcd9fcc8c252fa9610b1c6fef0c5b2c3c20c, against base546e05a159a58f5ceffe1d315373bc2f31982ea8, under the standing Mac-comparison plus Linux-CI rule.** No unresolved BLOCKING or SHOULD-FIX gate-binding finding remains. This completes the external-evidence check left pending by the sealed RV31 source/records confirmation. ROOT still owns the immediate main/head recheck and merge; no merge or product/K6c/F2a acceptance was performed by this review.

Fresh bounded grant: TASK RV31, native follow-up from ROOT HELP_HUMAN Agent0. First recorded clock/start **2026-10-02 01:35:30 UTC**; fifteen-minute end **01:50:30 UTC**, with a three-minute sealing reserve. The prior source/records review remains sealed under87c7f1373b0a23107e064e338c57933efb654c6a85b5146061e500c604b561ad; all fourteen payloads were rebound to NUMd9cf3a6e7d1b447019f10452a8a11e0447d54020. COMPLETION.json records this review's actual end.

## Committed packet and original streams

All **70 payloads** in T3/IMPLEMENTATION/A1_MERGE/dec025 match their manifest, NUMd9cf3a6 Git bytes and seal:

`1c99ff5472526192718de63699f4d4bb612b0747d092adcc4ba0bfd51b9735cb`.

All **66 raw/portable pairs** in ORIGINALS.json were independently checked against the original files. Their raw and portable hashes/sizes match. Portable bytes equal exactly ANSI removal plus the two documented literal VENV/T3 prefix substitutions. No numerical result, failure name, command, count or status was changed by that transformation. _run_records/PACKET_AND_RAW_BINDINGS.json retains the exact transformation and references without duplicating the large raw logs.

The Mac preflight records the exact candidate, base, clean candidate/sweep states, no old untracked sweep output, guard5387 and an absent fresh target. The generated fail-fast sweep independently records cb13dcd9, a clean working tree and the runtime versions. The existing eight-Cargo-job/four-test-thread driver matches hash9e34865b8a3fc49bbbb45bcbd457d1a4be76bd2aec7cac764662831e11afd133; its referenced no-fail-fast helper was read and hashed. The unchanged guard matches ee11ea1f43f0892dd0396bfe5c68e133084c90c87abbf757ce0be723e0ca7b79. These are preserved preflight observations, not reconstructed continuous monitoring or a hard resource-cap claim.

The driver uses the original installed1.97.1/auto-install0/incremental0 settings, invokes the sweep, then runs every discovered manifest with offline/locked/no-fail-fast tests and the remaining sweep commands. Its old comment says39manifests; the actual discovered roster and logs contain40. Nothing was excluded to make that old comment true. Compile/run paths bind the candidate sweep checkout and designated target.

## Mac DEC-025: comparison, not an all-green cargo claim

The original sweep summary remains **overall_status=fail**, cargo exit101; the wrapper records sweep exit1. Its later surfaces are honestly not_run in that initial summary. The separate no-fail-fast and later-surface records supplement it; they do not overwrite or relabel it.

The standing owner decision in T3/OWNER_DIRECTION.md explicitly accepts the Mac sweep with the three known platform failures matching the Mac baseline, together with passing Python/Vitest/build surfaces and clean hosted Linux numerical cargo. Its later-slice applicability and lapse condition remain unchanged.

I independently keyed every suite by manifest path, dropping only the positional filename index. Both candidate and preserved KF2 Mac baseline contain the same **40 keys**. Every aggregate passed/failed/ignored tuple and failing-test-name set matches except:

| Manifest | Baseline | Candidate | Source explanation |
|---|---|---|---|
| frame_kernel |432passed,0failed,1ignored|456passed,0failed,1ignored|Exactly24additional passing publication_tests names, each present in the already reviewed candidate source; no baseline test removed. |
| performance_harness |75passed,0failed,0ignored|76passed,0failed,0ignored|The added publication_failure_reasons_are_typed_at_the_public_boundary test; no baseline test removed. |

The exact three failing tests are identical on both sides:

- product_physics: s11g_tests::t13_committed_fallback_uz_is_byte_identical.
- runner_headless: cli_load_reference_one_both_modes_is_controlled_and_equals_the_library_route.
- runner_headless: load_reference_route_tests::load_reference_one_actual_solve_mints_bound_evidence_and_canonical_document_both_modes.

The two failing crates are byte-unchanged from the preserved baseline revision522167ac62f27ad999a4416d10b95f922ff8c665 to cb13dcd9. All forty original baseline log hashes/sizes match BASELINE_HASHES.json. The existing cmp_suites.py.txt was read and run only on the retained logs; its output is byte-identical to suites_vs_baseline.txt. No Cargo or test was rerun. The comparison is against this preserved, identified KF2 baseline, not an invented newly executed main sweep.

The later surface records show:

| Surface | Actual result |
|---|---|
| Python pytest |3070passed,32skipped,130subtests passed, exit0; same recorded baseline counts. |
| Desktop wasm build |exit0. |
| Desktop Vitest |134files and2822tests passed, exit0. |
| Desktop production build |exit0. |

The32pytest skips and existing FK ignored test remain skips/ignored, not passes. Warnings and test stderr remain in the original logs. Driver chronology remains00:48:07 start, initial sweep stop00:49:13, all-manifest completion01:04:07 and ALL-DONE01:25:50 UTC.

## Exact-head GEN-8 and hosted checks

The cb13dcd9 GEN-8 log reports **one passed, ten deselected in34.54seconds**. Its head binding is the preserved ROOT candidate/preflight record, corroborated by the same unchanged sweep checkout and clean exact-head sweep record; the terse pytest output alone does not print a Git SHA.

The earlier7f5137e7 GEN-8 failure remains preserved: one failed, ten deselected and two ABS_PATH_IN_UNCLASSIFIED_SURFACE findings. The source/records review independently confirmed its byte-identical packet relocation at cb13dcd9. No checker/policy waiver, deleted failure or edited sealed payload supplies the later pass.

I queried GitHub read-only for all five runs and compared their exact run fields, job IDs, names, statuses, conclusions and step identities/conclusions with the committed records. All match and all runs are completed/success on the exact cb13dcd9 SHA:

| Run | Workflow/event | Binding |
|---|---|---|
|36947660030|Piping Desktop E2E, pull_request|Selection, numerical cargo, all four source partitions and source-mode aggregate succeed. |
|36947660080|Harness Pre-merge Validation, pull_request|Selection and Harness pre-merge succeed; App Runtime/instruction jobs remain skipped. |
|36947660041|pec-tests, pull_request|Selection and aggregate succeed; PEC workspace tests remain skipped. |
|36947660193|governance-harness, pull_request|Harness succeeds. |
|36947805251|Piping Desktop E2E, workflow_dispatch|Selection, numerical cargo, all four source partitions and source-mode aggregate succeed. |

The saved DISPATCH_36947805251.json is an early in_progress observation. It is preserved as such; completed status is established by the final ci_runs record and the independently refreshed authoritative GitHub response. Nothing is inferred from the early record's empty conclusion.

For the manual run, I independently downloaded selection artifact11203235288. Its ZIP SHA256 matches the authoritative artifact digest, its workflow_run identifies36947805251/cb13dcd9, and its plan bytes exactly equal the preserved plan. The plan states event=workflow_dispatch, mode=full, coverage_full=true, numerical_required=true, base=target_base=546e05a159a58f5ceffe1d315373bc2f31982ea8, head=cb13dcd9, all17inventory specs selected, and both chromium-desktop/chromium-compact projects.

The four full source partitions include accessibility. The separate partial-mode accessibility barrier is expected to be skipped under the inspected workflow condition; that skip is not counted as a pass or omitted full-mode coverage. App/PEC skipped jobs likewise remain skipped under their conditional selection. Final aggregate job success was checked, including its Require all planned coverage step. Fresh GitHub PR1070 data shows OPEN, ready, MERGEABLE and the same head; this is an observation, not a merge operation.

## Post-gate source and scope

A1 is still clean at cb13dcd9. The sweep checkout is also at cb13dcd9, with no tracked/staged delta. Its only untracked file is the newly generated SWEEP_20261002T004810Z_cb13dcd9fcc8.json; those bytes match the preserved original sweep artifact. It was not deleted. The complete maintained core/validation still equal942572, preserving the prior source/records verdict and its H/validation-only reach. T9/both-entry applicability does not change.

All relevant code/workflow bytes inspected here match their candidate Git objects. The closed source/records review, its historical limitations, owner F17 decision, K6c boundaries and previous failed/partial records remain intact. This gate confirmation establishes the required evidence on the named candidate under the standing Mac comparison policy; it does not claim zero Mac cargo failures, product/native qualification, E_max/W1/F2a completion or owner personal review.

## Execution limits and handback

Only this additive NUM review packet was written, with all machine-specific metadata under _run_records. All Git reads used GIT_OPTIONAL_LOCKS=0. No Git/index mutation, candidate/source/test edit, delegation, compiler/runtime/probe, new host tool or cleanup occurred. The only executed comparison was the existing read-only log comparator; other computations checked hashes, JSON, names and preserved results.

An initial driver lookup read an older four-job/two-thread copy. Its hash differed from the recorded driver, so it was not credited. The actual existing eight-job/four-thread driver was then found and exactly bound. No driver was edited or run.

**Handback:** the source/records confirmation and these external-gate bindings are clear for cb13dcd9. ROOT must still perform its immediate remote main/head check before the authorized merge. Any candidate or relevant base change requires reassessing affected bindings; this return creates no automatic follow-on work.

