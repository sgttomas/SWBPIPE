# Generator recipe patch review

The proposed patch needs two write-safety repairs before application and controlled replay. No proposed Node/Cargo code was executed, and no maintained files or fixtures were changed by this review.

## R1 — P2: preserve rollback preimages through rollback failure

Proposed location: `projects/chirality-piping/tools/serialization/generate_product_preview_mechanics.mjs:145–155`, inside `replaceSet`.

The script keeps previous files only as in-memory buffers. If an installation rename fails after one or more successful replacements, rollback allocates and writes a new `restore` file at lines 149–151. That recovery write or rename can also fail—for example under the storage or filesystem error that caused the replacement failure. The second error aborts the restoration loop, then unconditional `rmSync(stage, ...)` deletes staging. Earlier destinations can remain replaced, while the only old bytes were in the now-exiting process. This contradicts the ordinary-error restoration claim and leaves no durable recovery copy. This is a source-level failure path, not an observed injected fault.

Stage durable preimage files before the first replacement, restore by renaming those staged backups where possible, and preserve the staging/recovery directory if any restoration fails. Continue attempting the other necessary restorations and report the original installation error plus incomplete rollback and its recovery location. Verify second/third replacement failure and rollback-failure behavior during the later authorized replay controls.

## R2 — P2: enforce actual destination-parent containment

Proposed location: `projects/chirality-piping/tools/serialization/generate_product_preview_mechanics.mjs:127–141`; related input reads at lines 22 and 66–70.

The destination allowlist compares relative strings, but `mkdtempSync`, input reads and replacement renames follow parent-directory symlinks. The regular-file check inspects only the final filename. If `fixtures` or `fixtures/product_preview` is linked to another checkout or outside directory, the script reads the linked model/legacy bytes and stages/replaces the linked precision outputs and generation record. `portable()` never checks this destination directory. Thus the allowlist does not enforce the promised physical project write boundary. The current checkout is not claimed to contain such a link; the proposed safety check is incomplete for that concrete path configuration.

Before staging or writing, resolve and validate the actual fixed destination directory against the intended project path, rejecting symlinked/redirected ancestor components. Apply equivalent real-path containment to fixed source inputs where they form the generation identity, and recheck the destination boundary at replacement as appropriate. Keep the regular-file destination check. A later controlled symlink-parent refusal check should confirm no staging or output mutation outside the intended directory.

## Other reviewed behavior

The package command removes the destructive redirection into the historical fixture. The proposed script uses built-in Node modules, passes explicit sparse/dense arguments to the existing real example, rejects command errors, captures stdout without reserialization and does not manufacture headers, passing statuses or numeric values. Producer/model/schema, finite rows, aggregate case quality and complete per-case mode evidence are checked before ordinary nonempty-output replacement. A genuine empty `MODEL_INCOMPLETE` response remains nonpassing and does not receive fabricated mode coverage.

Both outputs are captured and validated before replacement. The record is installed last and binds output hashes, commands, observed statuses, local source inventory, dependency inventory and tool versions. Source/input/lock/legacy hashes are checked after generation and again before replacement. The script inherits Cargo environment/resources; it does not claim complete compiler-environment authentication or process/power-loss atomicity. These limits are appropriate. Its write payload list contains only the two precision outputs and generation record; the safety findings concern filesystem enforcement and recovery around that list. No additional mathematical or physics changes are proposed.

The complete patch reconstructs from the exact proposed bytes. Patch SHA256 is `c45f8613271a67c4b6adc180da90771d33aab6a325a82c32cd29923aa2fc11be`; proposed script is `be74fec9edce8106edecee5529ad87c67b4e3fd396bfadd8fe6674b990fab706`, and proposed package is `ae2274d2078a378387a82d99af3fd49d58232d62aa4c383d2511cff02bff2a63`. The maintained script is absent, package remains the frozen preimage, and all declared maintained fixture/generator/input hashes still match. Static identity/scope checks do not clear R1/R2.

Evidence is in `_run_records/STATIC_PATCH_CHECKS.json`, `ORIGINS.json`, `backcheck.py` and the explicit-path scope output. Return the repaired patch for backcheck before application/replay. No App finding, native witness, runtime pass, physics qualification, whole-cut clearance or release claim is made.

Reviewer: TASK `/root/solver_manager/fixture_route_review`, parent `/root/solver_manager`, delegated-harness-native independent followup; no descendants and no source edits.
