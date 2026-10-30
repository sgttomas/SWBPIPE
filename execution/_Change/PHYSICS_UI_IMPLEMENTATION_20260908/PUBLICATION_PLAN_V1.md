# Sealed CHANGE publication plan V1

Status: **PREPARED; EXECUTION HELD PENDING ROOT FINAL RELEASE**.

Role and authority: `CHANGE` Agent 1, actual task `/root/implementation_change`, running `gpt-5.6-sol` with high reasoning and no descendants. The Owner's standing Git authorization covers scoped commits, push, branch publication, PR opening, and PR merge. It does not authorize force, bypass, history rewrite, destructive cleanup, or merging without exact-head check review. Branch `codex/piping-physics-ui-implementation-20260908` is based on `55df51ac3201456e0f181823e3aefefef47a73bb`; last observed `origin/main` is `781b478176db46ca66ebd38991607612e0ce9338`, whose post-base changes have no `projects/chirality-piping/**` overlap. Refresh that fact at execution.

The accepted combined technical review is `instances/RI/final/REVIEW.md` SHA-256 `16f3faafc1c896a8863676cc84502b4ec9153611bd3672c72b00510f2cd47537` with `RETURN.md` SHA-256 `6ace2a3a236e02c94235da2a0267bcfef3c8d0f2adf83b50d3a9a19652cd007b` and reviewed inventory SHA-256 `ccc9c7b15caa2e79d3b0d8f2060ccd6f772d02caada109176fa8763dab8df090`. This plan does not itself accept evidence corrections, Step 5 metadata, or publication.

## Entry gates

Root must release CHANGE only after all of these are complete and all writers are stopped:

1. The sealed 40-file lossless portability/whitespace repair is complete, its original-to-successor mappings and decoded hashes pass, and the same RI reviewer issues a successor backcheck PASS accepted by root.
2. F4 and U7 Step 5 metadata fan-in is complete and root has verified every bound member and final status.
3. C0 has sealed the integration snapshot and publication dispatch, preserving the bounded Step 1 outcome and all non-closure limits.
4. Actual path-anchor and candidate-whitespace validators pass without exclusions, waivers, validator edits, or silent rewrites.
5. The receipt validator passes before CHANGE reads the current receipt cursor. Receipt 136 remains the expected input cursor unless the validated loop shows a newer accepted receipt.

Any hash mismatch, active writer, scope expansion, unbound correction, validator failure, or upstream piping overlap stops execution for a routed correction or review.

## Source container and current-main integration

1. Rehash the accepted F4 and U7 R3 source set, technical reviews, native packet, final RI packet, correction/backcheck, Step 5 fan-in, C0 integration snapshot, and the original local handoff. Require the original handoff SHA-256 `ebbed866266cc961344151519b2792c6cfc5889eb18143a7809be69700c56035`.
2. Run the receipt validator before reading the latest receipt, then run repository self-check, path-anchor validation, candidate-whitespace validation, JSON/LF/containment checks, and an exact allowlist inventory. Do not run the full DEC-025 sweep yet.
3. Stage only the authorized implementation source, current run/control/evidence records, approved D-66 addendum/register change, accepted correction successors and archives, and Step 5 artifacts. Exclude ignored build outputs, private temporary targets, app stores, and unrelated paths. Commit this clean pre-test state as the **source-freeze commit**.
4. Fetch `origin`, confirm the source-freeze branch and current `origin/main` ancestry, and inspect every upstream path since `55df51ac3201456e0f181823e3aefefef47a73bb`. If upstream touches piping source or an accepted control/receipt binding, stop for impact review. Otherwise integrate current `origin/main` with the repository-established non-fast-forward merge method. No rebase, force, or bypass.
5. Call the resulting post-integration commit the **tested source commit**. Require every accepted F4/U7 source hash to equal the pre-integration freeze, require no unexpected piping path change from integration, and require a clean working tree before tests.

The source-freeze commit and tested source commit may be the same only when `origin/main` has not advanced and no integration commit is needed.

## Unchanged full DEC-025 verification

From a clean tested source commit, run exactly:

```sh
python3 tools/release/run_evidence_sweep.py --execute
```

from `{WORKING_ROOT}`, using the established project Python environment. No capability omission or partial waiver is allowed. Record all five registered surfaces:

- G0: `cargo_crate_sweep`;
- G1: `python_pytest`;
- G2: `desktop_vitest`, including the registered desktop WASM build;
- G3: `desktop_playwright_e2e`, using the registered host execution unless a valid exact-commit DEC-093 CI binding is explicitly supplied;
- G4: `desktop_production_build`.

Require all five surfaces to return PASS and require the sweep artifact's `commit_hash` to equal the tested source commit. Do not repeat suites after a clean PASS unless source changes or a failure requires it. Then run the full practitioner harness required by the C0 release instrument and preserve its exact result.

## Receipt 137 and evidence-only container

Receipt 137 belongs **after** the tested source commit exists and its full DEC-025/practitioner evidence passes. C0 owns the append and must be explicitly released by root. Its `Examined-Through` value must equal the tested source commit, not the later evidence-only closeout commit and not the original base. It must name Receipt 136 or the validator-confirmed latest receipt as its parent and bind the accepted correction, same-RI backcheck, Step 5 fan-in, integration snapshot, source equality, five-surface sweep, practitioner result, and continuing non-closure limits.

After C0 writes the evidence container and Receipt 137:

1. Run the receipt validator before reading the new cursor and require Receipt 137 to validate.
2. Run final G0-G4 evidence-record checks, practitioner-result binding, path-anchor validation, candidate-whitespace validation, JSON/LF/source-containment checks, and exact allowlist/status review.
3. Commit only the new sweep, Receipt 137, final C0 handoff/snapshot, and CHANGE closeout as a separate **evidence-only commit**.
4. Require `git diff --exit-code {TESTED_SOURCE_COMMIT}..HEAD -- projects/chirality-piping/core projects/chirality-piping/apps` and the exact accepted producer/source fences to show no source change after testing.

## PR and merge

Push the branch and open one PR to `main`. Record its URL and exact head SHA. Read every required check result for that exact head; require all required CI checks green and mergeability current. Never merge behind an unresolved watch, stale head, skipped required check, failure, bypass, or force option. Under the standing Owner authorization, merge using the repository-established merge-commit method only after exact-head revalidation, then fetch `origin/main` and report the effective merge SHA. Preserve prior lanes and the original local handoff throughout.

This tranche closes only the bounded implementation and its evidence container. It does not mutate dependency rows, the active DAG pointer, lifecycle state, public physics/schema adoption, engineering compliance, professional reliance, or broader first-wave closure.
