# RV38 — finite records-placement plan

**A concrete custody-preserving correction is available: 21 intact moves and one whole-packet archive, after the active T4 assignment has returned and ROOT releases placement.** No existing path was moved or edited by this task. No policy/checker exception, numerical change, archive-index amendment or new tool is proposed.

Basis: K6C `e67ae464618d271b4bda5dc98cb8a039f753af32`; saved `R/verification/k6c_record_placement_01/_run_records/BEFORE.json`, SHA256 `a61c7d8b9d1966bcec7800e406287d1edad3abda9a4f42b93ce57ce6304cf3ac`. Its 80 findings are 79 UNCLASSIFIED metadata records plus one CONTROL, across 23 immediate parent directories. They form 22 complete custody units. R means the enclosing `RESUME_2026-09-30` root.

[MAPPING.json](MAPPING.json) is the exact finite unit mapping. [_run_records/CUSTODY.json](_run_records/CUSTODY.json) records every file's source/destination, SHA256, frozen Git blob, Git mode and filesystem mode. [_run_records/FINDING_MAP.json](_run_records/FINDING_MAP.json) maps all 80 findings individually.

**Placement map.** Each ordinary move retains the complete packet, all nested paths, its original manifest and the same manifest-relative run base. All destinations were absent when checked.

| Existing unit under R | Destination under R | Files |
|---|---|---:|
| I21 root checkpoint only | I21/_run_records/source_checkpoint_00 | 8 |
| I21/layout_04 | I21/_run_records/layout_04 | 24 |
| I21/source_01 | I21/_run_records/source_01 | 8 |
| I21/source_02 | I21/_run_records/source_02 | 30 |
| I21/source_03 | I21/_run_records/source_03 | 8 |
| I21/source_10 | I21/_run_records/source_10 | 9 |
| metric_design_02 | _run_records/metric_design_02 | 9 |
| metric_design_03_exact | _run_records/metric_design_03_exact | 13 |
| metric_design_04_sparse | _run_records/metric_design_04_sparse | 11 |
| source_review_RV30/exact_11 | source_review_RV30/_run_records/exact_11 | 14 |
| source_review_RV30/finite_callers_12 | source_review_RV30/_run_records/finite_callers_12 | 9 |
| source_review_RV30/format_stream_18 | source_review_RV30/_run_records/format_stream_18 | 10 |
| source_review_RV30/h_request_bindings_10 | source_review_RV30/_run_records/h_request_bindings_10 | 15 |
| source_review_RV30/k0_assembly_16 | source_review_RV30/_run_records/k0_assembly_16 | 16 |
| source_review_RV30/public_layout_plan_21 | source_review_RV30/_run_records/public_layout_plan_21 | 9 |
| source_review_RV30/public_layout_result_22 | source_review_RV30/_run_records/public_layout_result_22 | 9 |
| source_review_RV30/serializer_15 | source_review_RV30/_run_records/serializer_15 | 10 |
| source_review_RV30/sparse_13 | source_review_RV30/_run_records/sparse_13 | 10 |
| source_review_RV30/sparse_correction_14 | source_review_RV30/_run_records/sparse_correction_14 | 7 |
| source_review_RV30/static_pools_20 | source_review_RV30/_run_records/static_pools_20 | 10 |
| source_review_RV30/vr_nodes_17 | source_review_RV30/_run_records/vr_nodes_17/packet.tar.gz | 25 |
| source_review_RV30/wrapped_errors_19 | source_review_RV30/_run_records/wrapped_errors_19 | 16 |

The I21 root unit is exactly the seven payloads listed by `OWNED_FILES.sha256` plus that manifest: `CHECKPOINT_0.md`, `RAW_COMMANDS.json`, `SOURCE_HASHES.txt`, `LEGACY_TERM_DELTAS.json`, `derive_deltas.js`, `BINARY_FIXED_TERMS.json`, `VERIFICATION.json`, and `OWNED_FILES.sha256`. Do not move I21 itself or any other child packet. This is an already sealed seven-payload checkpoint, not permission to collect its later subdirectories into a new packet.

**Why vr_nodes_17 needs different custody.** Existing `surface_roles.py:120–146` checks CONTROL names/tokens before evidence directories. Consequently `_run_records/BRIEF_BINDING.json` remains CONTROL after any directory move; nesting it in another `_run_records` cannot fix the finding. Its bytes bind a past review's supplied brief/path/hash/revision and say what was actually read. They are historical metadata, not a currently issued launch brief. Rewriting the path or renaming that file would break its original 24-payload seal.

Preserve the complete 25-file packet as an ordinary exact-byte evidence archive, with unchanged internal paths and `SHA256SUMS` at the archive root. The existing manifest SHA256 is `409c7f91e02a239570767775d97552761c2f9b08f80e25bfa8a3132d47b59fb6`. The uncompressed packet is 369,263 bytes; all 25 files are 100644/0644. A normal Git tree archive of the frozen packet subtree, followed by the existing gzip utility, can preserve that file set and modes without executing any historical script. Do not create the archive until the implementation grant.

For concrete recovery, retain both `e67ae464618d271b4bda5dc98cb8a039f753af32:<original repo-relative path>` and the new archive/member locator in the custody ledger. `git show` supplies exact bytes read-only. An archive inspection/extraction, if later needed, uses an isolated scratch destination and checks the original seal from its recovered packet root; it does not restore old controls into the active managed tree or execute the old brief/commands. Record the new container hash separately. Never replace the original member manifest with a manifest of the archive.

The current GEN-8 implementation enumerates .md/.yaml/.yml/.json surfaces; it does not inspect archive members. This is a storage/custody transition of a historical packet, not reclassification of BRIEF_BINDING or a claim that the classifier accepts an extracted copy. Its role remains CONTROL whenever restored in the active tree. The ordinary archive plus portable navigation is explicit and recoverable; no filename-only camouflage of the single offending file is proposed.

Do **not** invoke or alter the rolling `tools/archive_agent_runs.py` mechanism for this correction. Its accepted policy selects top-level runs untouched for 14 days, or aged binary files after seven days. This current run is active and the tool does not select these nested metadata packets. Do not invent an ARCHIVE_INDEX entry, tag disposition, age exemption, historical control exception or portability override to make that policy apply. The proposed file container uses the repository's existing exact-byte archive/evidence convention, not the rolling archive's missing-path resolver.

**Apply and verify after the active lane.**

1. Wait for ROOT's explicit placement scope after I26 has completed/reaped T4 and any comparator, sealed its output and released its paths. Recheck the 280 source bytes/modes and destination absence against this mapping; if they changed, stop for a revised map. Preserve all audit records, I26 data/journals, I28 ordinary source/targets, source81c038 and prior placement ledgers.
2. Move the 21 units intact with existing filesystem/Git operations under ROOT's ownership. Preserve original payload/manifest bytes and modes. For vr_nodes_17, create the whole archive from the frozen tree, verify every member's SHA256/path/mode and the original manifest, and only then remove its extracted working-tree packet. This task authorizes none of those writes.
3. Leave portable navigation at each old root: `MOVED.md` and a `RETURN.md` pointer for normal packets; I21 root uses `MOVED.md` and a `CHECKPOINT_0.md` pointer. These are explicitly new navigation records, not the sealed original returns. The original bytes remain at their new location or in the archive. Each pointer links the canonical payload, original manifest hash, frozen commit locator and new relocation ledger. Do not leave old manifests next to pointer substitutes, old absolute-path CONTROL copies, or symlink aliases that recreate the old classified surface.
4. Write one additive relocation ledger under `R/verification/k6c_record_placement_01/_run_records` (a new file, not an edit of BEFORE.json), recording old/new prefix or archive/member, each blob/hash/mode, source commit, container hash and verification result. The existing `verification/record_placement_20261002_k6c/RELOCATION.json` and pointer convention are the precedent; preserve them unchanged.
5. Verify the 21 original manifests at the new roots and the archived original manifest in an isolated read-only recovery. The 22 original manifest byte hashes must remain identical. Preserve `AUXILIARY_FILES.sha256`, `SOURCE_HASHES.txt` and other external provenance lists verbatim: they describe historical source/scratch origins and are not new instructions to move or regenerate those external trees.
6. ROOT then runs the existing bounded changed-file preflight and the required final candidate gates, including GEN-8, in their authorized final-gate scope. This plan predicts resolution only of the supplied 80 findings; it does not claim a full gate pass or waive later findings.

**Consumers and navigation.** The inspected T4 recipe uses the original K6B wrapper/comparator, the fixed ordinary runner/source/binary, existing measurement seed/journal/dumps and R/I26 comparison records. None lies in this map. Bounded string searches found no affected packet references in current I26 T4/readiness/measurement-plan code/metadata or maintained tools/.github/loop configuration for vr_nodes_17. This is not process tracing and does not cancel the wait-before-placement rule.

There are real historical consumers: I28's sealed `ordinary_qualification_preflight_01/_run_records/metadata_check.py` names layout_04 and vr_nodes_17; `ordinary_artifacts_02/_run_records/{verify_artifact_evidence,compiler_identity,source_correspondence}.py` also read the original layout/review paths. Do not edit these sealed scripts or their bound origin records. Their reproducible replay remains on the frozen original tree or recovered original-relative layout, with the ledger explaining present-day custody. Do not rerun them blindly against old-location pointer substitutes.

The revisable `ROOT_CURRENT.md`, final D evidence index and I28 closeout draft should gain one placement-ledger/navigation note after relocation. Their inspected direct accepted-review links are outside the moved set, so no wholesale replacement is needed. The live work graph may add the placement-complete state through its ROOT owner; append a disposition to ROOT_RULINGS if needed, without rewriting historical rulings. All new prospective locators should use the mapped canonical paths. Sealed higher-level reports, accepted source warrants and old source strings retain their original meaning and hashes; a pointer alone does not make an old hash-based or raw-file consumer automatically relocatable.

**Checks and boundary.** Current existing `effective_role` and machine-path detection reproduce all 80 saved findings with no policy issue. All 280 files equal the frozen Git bytes and have 100644/0644 modes; all 258 manifest payloads verify. Twenty-one projected moves have no remaining machine-path-bearing CONTROL/UNCLASSIFIED surface, including the other CONTROL-named BRIEF.txt/LAUNCH_PLAN_RECHECK files, which are already portable. The only remaining extracted CONTROL issue is the explicitly archived historical BRIEF_BINDING. No unresolved custody-data issue was found. Actual archive creation/member verification, relocation, consumer navigation and final gates remain unexecuted.

No heavy check, full GEN-8, runtime/build/solver/model operation, prior packet copy/edit, Git/index write, relocation, audit/artifact-target change, checker/policy change or delegation occurred. Only this new plan/check packet was written. Receipt 2026-10-02 15:02:12 UTC; cutoff15:17:12; return deadline15:22:12. Native TASK `/root/rv38_composite_source` returns to ROOT `/root` and stops; this is planning support for final gates, not numerical acceptance.

