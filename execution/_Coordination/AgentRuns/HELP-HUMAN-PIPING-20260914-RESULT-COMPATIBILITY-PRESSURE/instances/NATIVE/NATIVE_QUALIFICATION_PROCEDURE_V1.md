# Frozen native qualification procedure V1

Status: PREPARED only. Execute nothing in this file until HELP_HUMAN supplies the frozen source identity and the serialized native build/GUI lease. This procedure is derivative technical evidence under F-PIP-2; it cannot establish governed acceptance, release readiness, professional reliance, connected-provider identity, or support for private/user models.

## Release inputs and environment

Require one immutable candidate identity from root, the complete source diff/relevant-file manifest, and explicit confirmation that the compatibility writer has released `apps/desktop` build/WASM/dist/native surfaces. The runtime executor must exclusively own those surfaces until the final process is closed. Bind Root/project/TASK instructions, sealed brief, accepted plan and owner message; bind every file listed in `CURRENT_HOOKS_AND_FIXTURES_V1.json` against the final candidate, replacing preparation-time hook hashes rather than treating them as final.

Use an unlocked macOS graphical session with Codex computer control and Finder available, existing offline npm/Cargo dependencies, no network, and no normal user project store. Create an evidence-local Tauri overlay with a unique bundle identifier, product name and isolated app-data/store path. Confirm no prior witness process uses the candidate executable. Do not read or open user models. The only model inputs are the accepted invented synthetic fixture and values in `FROZEN_SYNTHETIC_EXPECTATIONS_V1.json`.

## Commands after source release

Run from `{WORKING_ROOT}` unless a command names another directory. Record exact argv, cwd, start/end time, exit, stdout/stderr hashes and output paths for every command, including failures.

1. Focused maintained checks, bound to the final source:

   ```text
   npm test --workspace apps/desktop -- --run src/features/result-export/resultExportAdapter.test.ts src/features/result-export/ResultExportPanel.test.tsx src/features/result-export/nativeResultSave.test.ts src/features/results/HistoricalRunContext.test.tsx src/features/stress-neutral/StressNeutralExportPanel.test.tsx
   ```

   ```text
   npm test --workspace apps/desktop -- --run src/App.test.tsx -t "suppresses solve proof unless the completed job, result, and exact model version remain bound|suppresses a real delayed backend completion when native open commits a new model|persists native immediate-cancel intent until the backend start receipt exists|dispatches detached pre-start cancellation once after native open invalidates the run|discards a delayed apply after opening a replacement model without creating a checkpoint|discards a pending batch when another model opens, and ignores delayed save metadata after a model replacement|opens saved results as readable history and preserves their exact fields on unchanged save|clears reopened computed evidence on model edit and exact undo/redo transitions"
   ```

   ```text
   cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml native_result_download
   ```

   ```text
   cargo test --manifest-path core/reporting/result_export/Cargo.toml
   ```

   ```text
   python -m pytest tests/test_analysis_run_schema.py tests/test_analysis_run_records.py tests/test_result_export_v0_2.py tests/test_stress_neutral_export_package.py
   ```

   These checks must cover all 60 signatures, legacy/new dispatch, unsafe numbers, duplicate keys, malformed Unicode/raw JSON, checksum scopes, source replacement, late replies, busy/cancel/stale generation guards, and native IPC errors. If the final implementation renames tests or adds the actual analysis-record/stress-neutral 0.2 test files, substitute the final registered focused command and record the mapping. Tests are supporting evidence and never replace the GUI witness.

2. Build one fresh isolated application from the frozen source:

   ```text
   CARGO_NET_OFFLINE=true npm run tauri -- build --bundles app --ci --no-sign --config <evidence-local absolute overlay path>
   ```

   Hash the exact executable, complete `.app` bundle tree and `apps/desktop/dist` tree. Repeat the complete relevant-source manifest after build and after GUI completion. Do not use a previously built binary.

3. Run the packaged file-backed supporting self-test:

   ```text
   <exact frozen candidate bundle executable> --self-test-saved-edited-load
   ```

   Require exit 0 and verify its attachment inventory, 350/500 analytical rows, persistence/reopen result, fresh-resolve equality and exact baseline/edit/undo/redo/equivalent-agent relations. Record its embedded fixture hashes separately from the actual GUI model hashes. Its currently declared missing native analysis builder cannot satisfy the new analysis-record 0.2 or GUI Current/Historical acceptance.

## Actual GUI/native sequence

1. Launch the exact bound executable against the isolated store and capture PID, start time, executable identity and empty/synthetic-only project inventory. Create a new blank local project and author, through existing typed GUI controls, the root and tip nodes, material, straight pipe, root anchor, sole empty load case, and sole +global-Y 350 N primitive. Explicitly observe the initially empty primitive selection; do not create a temporary case. Each accepted value must equal the frozen synthetic fixture. Save a store snapshot, extract only this synthetic record with the maintained snapshot tool, and freeze exact full `model_json`, model-content hash `H350`, attachment inventory and editor-intent evidence before solving. If the fixed retained project id is intentionally reused, `H350` must be `sha256:2932...1788`; otherwise derive and record the project-id-specific `H350` and do not compare it to the retained fixed hash.

2. Independently construct expected `M500` from exact frozen `M350` by changing only `load_cases[0].primitive_loads[0].magnitude.value` from numeric 350 to numeric 500. Canonicalize through the final checked canonicalizer and freeze `H500` before product apply. For the fixed retained project id, `H500` is `sha256:6c909e...8d0bf`. Retain the independently constructed bytes as an oracle; do not use the operation applier to construct the oracle.

3. Solve 350 N through the actual GUI/native mechanics route. Require Current only when the exact live model, result, analysis record and in-session input manifest agree. Capture the 67 raw mechanics rows. Check tip Uy/Rz, direct support resultant, local end-i shear-z and bending-y against the frozen values and `5.001e-7` absolute allowance in each declared published unit. The signed global root Fy/Mz values are analytical transforms; do not claim direct component DTO rows that are absent.

4. Materialize the actual frontend-produced Current result 0.2 JSON through `ControlledExportLink` and the typed `save_local_result_json` native path. Capture the success receipt, exact requested basename/final collision-safe basename, byte count, and frontend Current binding. Verify the copied evidence bytes parse, dispatch as exactly 0.2, validate against the final strict schema, contain 67 one-to-one row-accounting entries, 66 exported quantities, one solver-mode disclosure and 66 preservation witnesses. Recompute carrier, row, target, derivative/analysis-record checksums independently using the final standalone checked canonicalizer and SHA-256. Confirm the derivative/record checksum projection excludes only its own carrier and that the raw received mechanics carrier remains unchanged. Preserve advertised legacy hashes as received.

5. Materialize the actual stress-neutral 0.2 JSON and CSV emitted by the frontend. Validate the actual delivered JSON, not a pre-hash construction object, against the final 0.2 schema. Require 67 received rows, 66 value/unit/semantic-dimension preservation witnesses, one explicitly withheld/disclosed solver-mode work witness, all declared package members including `unit_system_disclosure`, and no presentation-only fields. Independently recompute every JSON member hash, manifest seed, dedicated whole-package hash and normalized CSV-text hash under their declared scopes. Verify each member file exists and equals the corresponding declared payload. Mutate one covered field only in an evidence copy and require its proper member/package verification to fail.

6. Apply a human GUI edit from 350 to 500 through the typed load operation. Require immediate Current invalidation and no stale export, comparison, overlay, rule, report or query eligibility. Compare the full actual model to independently frozen `M500` and require exact `H500`. Undo must restore exact `M350`/`H350`; Redo must restore exact `M500`/`H500`. All attachments remain exact and the only semantic model delta is the force magnitude.

7. Return to exact `M350`. Submit the accepted synthetic agent-authored batch through the existing offline proposal review/apply route, updated only where the actual synthetic project/source ref requires it. Require explicit local review, structured operation application and exact full-model equality with human `M500`/`H500`. File-supplied `author_type=agent` and source metadata establish only equivalent synthetic agent-authored intent; do not claim a connected provider or authenticated agent identity. Preserve a missing/incorrect source-ref rejection through maintained tests or an isolated synthetic draft.

8. Solve exact 500 N fresh and repeat numerical, Current, result 0.2 and stress-neutral 0.2 checks. Save the project through the actual native persistence path. Snapshot all stored attachment fields, including model, raw mechanics result, analysis record(s), model hash, project-envelope hash, editor intents, proposal/review target and any new versioned record/checksum fields introduced by the final source.

9. Record PID/start/executable, invoke normal GUI Quit, and prove the exact executable is absent. Relaunch the same bound executable and same isolated file-backed store; require a new PID/start. The new 0.2 record opens Historical because no saved historical input-manifest payload is available. Its stored bytes and hashes remain verifiable/readable, while Current overlays, comparisons, rule checks, reports, queries and exports remain unavailable. An unchanged Save must preserve every captured attachment field byte for byte. A fresh 500 N solve must restore Current against a new exact input manifest and reproduce the 67 raw mechanics rows and both delivered 0.2 products.

10. In a separate isolated store copy, open the SHA-bound accepted synthetic legacy 0.1 fixture. Require Historical with `HISTORICAL_INPUT_MANIFEST_MISSING` only, no false result/envelope/model hash mismatch, no relabelling of advertised hashes, no JCS replacement of the desktop legacy `localeCompare` serializer, and unchanged-save preservation of all eight legacy raw fields. Treat missing, ambiguous or numeric-type-lost preimages as explicitly `unverifiable` rather than recomputed proof.

11. Close the final app normally and prove the exact executable is absent. Seal command records, source/build/post-GUI hashes, store/snapshot identities, raw output hashes, schema/hash verifier reports, screenshots/AX text, process lifecycle evidence and a failure ledger. Release the serialized native lease only with this sealed return.

## Downloads/Finder evidence limit

Prior direct terminal reads of `~/Downloads` stalled. Prefer the new typed native receipt plus visible Finder evidence. If a direct read is again unavailable, use Finder to copy the exact visible saved filename into this run's evidence directory, then hash and validate the evidence copy. Preserve the action sequence, UI observations and receipt. This is compositional evidence that the actual frontend/native command and Finder-visible artifact participated; it does not prove a direct terminal byte comparison between the original Downloads inode and the copied inode. Do not overstate the original file preimage or invent an original-producer checksum.

## Stop conditions

Stop and return the exact blocker if the source binding changes, another process owns build/WASM/dist/native surfaces, required final schemas/checksum projections are not frozen, the actual GUI route bypasses the typed operation/save path, a user/private model would be required, or output bytes cannot be captured truthfully. A failed attempt remains evidence and does not authorize a workaround that mutates source or bypasses product behavior.
