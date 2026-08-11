# Structured Return — A2-APPLICATION-VERIFY

## Terminal verdict

`FAILED / HOLD — SEMANTIC APPLICATION PASS; VERIFIER CONTAINMENT VIOLATION`

The complete ruled application is semantically consistent with C-B, V-D,
O-B, and MR-A after the owning MR-A child corrected its aggregate-manifest
formula. Terminal closeout is not releasable. This verifier's otherwise
passing practitioner-harness pytest command created three ignored Cargo
`target/` directories inside the repository because that command was not
given an external `CARGO_TARGET_DIR`. The directories are preserved
untouched. This verifier has no repair or deletion authority.

## Identity and parentage

| Field | Value |
| --- | --- |
| RunID | `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES` |
| ParentInstanceID | `WORKING_ITEMS-A1-APPLICATION` |
| ChildInstanceID | `A2-APPLICATION-VERIFY` |
| Agent form | fresh non-delegating evidence-only ephemeral Agent 2 |
| Branch | `codex/piping-del0904-owner-gates-20260810` |
| Accepted basis / HEAD | `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3` |
| Locally observed `origin/main` | `912e3a8c9c07e9b8359093f63feace1c7c9f4776` |
| Current-main caveat | Piping tree and relevant root instruction trees are identical between accepted basis and locally observed `origin/main` |

## Bounded-task shell report

RUN_STATUS: `FAILED`

ControlSurface: `FILE`

TaskProfile: `NONE`

TaskSkill: `NONE`

ScopePath:
`/private/tmp/chirality-piping-del0904-owner-gates-20260810/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES/instances/A2_APPLICATION_VERIFY`

ToolsUsed:

- shell `git`
- shell `shasum`
- shell `jq`
- shell `stat`
- shell `du`
- shell `diff`
- shell `pytest`
- shell `python3` standard-library validation
- runtime `apply_patch` for this return and status only

ToolPolicyCompliance: `VIOLATION` — the command
`PYTHONDONTWRITEBYTECODE=1 pytest -q -p no:cacheprovider tools/practitioner_harness`
passed 349/349 but was invoked without an external `CARGO_TARGET_DIR`; its
subprocesses created the three ignored target roots inventoried below.

WriteAuthorization: `ALLOWED_WRITE_TARGETS` — only this `RETURN.md` and this
instance's `STATUS.json` were authorized deliberate outputs. The three target
directories are unauthorized ignored side effects and make the run failed.

Outputs:

- this `RETURN.md`
- this instance's `STATUS.json`

MISSING:

- deletion authority and post-cleanup zero-ignored-drift backcheck for the
  exact three verifier-created target directories;
- commit-bound DEC-025 evidence sweep, which is lawfully post-commit and was
  not eligible in this uncommitted dirty worktree;
- full Piping pytest completion in the present interpreter environment:
  collection stopped on five modules because `jsonschema` is absent. The
  focused applicable nonlinear suite and independent witness structural/hash
  checks are reported below without inflating this to a full-suite pass.

NEEDS_HUMAN_RULING:

- authorize deletion of only the three exact ignored directory roots below,
  after rechecking each is a directory, not a symlink, and matches its
  fingerprint; then require terminal ignored-status, tracked/untracked
  containment, and hash backchecks. No broader clean or deletion is needed.

DEPENDENCY_NOTES:

- No semantic dependency cycle was found.
- Closeout is cleanup-gated on the verifier-created directories. The original
  V-D Cargo lock side effect was separately owner-authorized and removed by a
  different managed cleanup child; this verifier did not touch it.
- DEC-025 remains a sequential, clean-commit-bound pre-push gate under
  `docs/BUILD_AND_RELEASE.md` §5.1. Its dry-run plan passed, but execution now
  would be both premature and writing.

## Requirements matrix

| Requirement | Result | Independent evidence |
| --- | --- | --- |
| Exact owner ruling | `PASS` | `OWNER_RULING_2026-08-11.md` SHA-256 `5f0f857e5b95284e88506c768c91547b3f02815dfd63747c966d2df7b8775a22`; exact four owner lines present. |
| Accepted preparation interface | `PASS` | `PACKAGE_RETURN.md` `01855ffbaee369e9321ae16972af56183fb6fada66a99a7aa9680c73a8bc906f`; DEC packet `1265e843c2c33eaa915f26cba5b75b72e811b7bf2bcc280bb90bcb02ecc5178c`; R14 packet `8218b4566c4e3d4476ee7f29db4b7e96ae8740c32273bc9b7c4102b86172335d`; MR packet `ee08a4af9cfb99ba624a2ab510f2c95d48484172a562f001bb60aa1eac9d8565`. |
| C-B ruled record | `PASS` | New record is 6,027 bytes, SHA-256 `1829a2a6a608dbbdd479248133f2c2a05406c0eabe7a03be57b998735fbe78b6`, calculated blob `468d6dd4a85525b64989ff520a5f4ff10e7c6e6f`; exact five-class order, zero-count fields/floors, cap 4, inclusive final zero, policy-not-release/non-comparison effects. |
| C-B historical identity | `PASS` | All 12 pre-existing direct nonlinear JSON records parse and equal their accepted-base Git blobs byte-for-byte. All C-B content-addressed pointers recompute. |
| C-B consumer/regression compatibility | `PASS` | Focused `test_nonlinear_support_regression.py`: 8 passed; external Cargo target used; original lock hash unchanged by that test. The new record has no falsely claimed runtime consumer. |
| V-D artifact integrity | `PASS` | Seven manifest hashes and all nine `SHA256SUMS.txt` rows recompute. Manifest SHA-256 `a08a738634155b01be83a04f2777bfcbbb131246b934ebda24eaf9ad4860f385`. |
| V-D raw/projection identity | `PASS` | Runner output SHA-256 `e41f8545cf6fc0603cb41a0bdc08d90726f59ce57e13ad6809036589db3ded8f`; embedded suite object equals the standalone suite projection exactly. |
| V-D current census | `PASS` | Independently recomputed 25 requested, 11 matched, 0 mismatched, 14 blocked; 206 rows, 91 observed, 115 unobserved. Every blocked row has null observation/delta/admission and a nonempty fail-closed reason. |
| V-D block classification | `PASS` | All 14 analysis rows map one-to-one to blocked fixture rows and are `implementation_binding`; counts/dimensions/reasons reconcile. DEC-092 is the new 6-value fourteenth member. |
| Historic 13-case census | `PASS` | Accepted `SUITE_RUN_MECHANICS.json` blob `c0fee1b6581169e4427686e5c1932ec6176f5ceb`; exact ID symmetric difference zero against the 13 `historic_july20_member` rows; 109 unobserved values. |
| V-D per-kind characterization | `PASS` | Recomputed from current mechanics rows plus immutable R14 stress output blob `2feb1b9cf0abc0f978457a6e1564db4596239a22`: 12 kinds, 217 rows, 102 observed, 115 unobserved; maxima, exact-zero floors, nearest-nonzero context, and all TBDs match. |
| V-D source identities/non-effect | `PASS` | Seven source blobs recompute; no public value/policy selected, no repair performed. Sparse design separates `m`, `rad`, `N`, `N-m` and truthfully refuses backfill from mixed-unit aggregate rows. |
| R14 immutable identity | `PASS` | Bundle tree `3d847390dfa74f8dced090164fb95f31eade83c7`; zero diff from base; checksum-index SHA-256 `581f039ffaca00097de75c151dcd13f9403c71076ab06671cd8e94c069f7ec7a`; all 74/74 entries pass. |
| O-B characterization | `PASS` | Acceptance record SHA-256 `c110ba0649ed91bf624622f6f2deed8ec7a9787a26fd79870c6741e3c6a81f2b`; source-pinned P1-P16 acceptance only, bundle remains `INTERNALLY_VERIFIED`, no current-head claim. |
| MR-A deterministic generation | `PASS` | Generator `--check` reports 64; fresh external generation compares byte-identically for mechanics/stress/nonlinear. Membership 21/15/28. |
| MR-A tier/non-promotion | `PASS` | All 64 pages have frontmatter `draft_evidence` and visible `DRAFT_EVIDENCE`; zero `MAINTAINER_REVIEWED`; exactly 63 generated pages differ and the DEC-092 page remains byte-identical. |
| MR-A review inventory | `PASS` | Final review instrument SHA-256 `0bc44d02ff8649cfc74275ddbde12e3a449019c4a8928d9aceb772af0eee49d8`; 64 unique rows, every page and source SHA/blob binding recomputes, 64 null `NOT_RECORDED`, zero promotions. |
| MR-A witness mapping | `PASS_WITH_ENVIRONMENT_CAVEAT` | Exactly one mapped formal chain and 63 `WITNESS_CHAIN_MISSING`; all referenced files exist and hashes match. Formal JSON has required structural keys, 11 formulas, 6 outputs, all three linked artifacts, canonical hash `1af3d494093179bb93435be0e252ba3b84d8d6959a49a3c4aee455b8153518dd`. Fresh `jsonschema` execution was unavailable in installed interpreters. |
| MR-A aggregate/manifests | `PASS` | After author attempt 3, declared repo-relative `path + NUL + lowercase SHA-256 + LF` recipe independently gives `e33877bef390f371a009c06e8247b56a2c410ad5fb499a8fc4fadb06165d8b45`. Final return `23bc35e9750479a19684828402350d3b6f1037e3799748c3015a84a236f697b9`, basis `c70e6096e73bf48523dd164c6764a085de872e5fe62feb24f229eaa944e51f04`, changed-path manifest `15e1426e837b05314ece932d4b72d3952b446d11f8e538a2672cd3f631364919`; all 68 changed-manifest and 20 basis rows recompute. |
| DEL-09-04 Remaining/state | `PASS` | Exactly two Remaining bullets before/after; bullet 1 byte-identical; only bullet 2 lawfully residualized to keep public comparison values open while recording C-B. Current State remains `IN_PROGRESS`; no tier/release/lifecycle promotion. |
| Protected/non-effects | `PASS` | Claims registry, PRD, Task Management registers, receipts, decomposition, DAG, GUI/app, reliance, export/CAEPIPE, release, and key protected trees/files have zero base diff. `TM-PIP-037` untouched. |
| Tracked/untracked containment | `PASS` before ignored-drift gate | Exactly 68 tracked modifications, all MR-A documentation/page plus DEL-09-04 memory/status paths. Exactly 68 authorized non-ignored untracked files before writing this return, all run-root, C-B, V-D, or DEL-09-04 run-record paths. Staged paths zero. No verifier-created non-ignored path. |
| Whitespace/conflicts | `PASS` | `git diff --check` exit 0; no conflict markers in scoped surfaces. The three MR-A surplus terminal blank lines were owner-child-remediated before final hashing. |
| Applicable tests | `PASS_WITH_ENVIRONMENT_CAVEAT` | nonlinear focused 8/8; practitioner harness 349/349; harness self-check exits 0 with pre-existing findings. Full Piping pytest stopped at collection on five `jsonschema` imports; no substantive test failure was observed, but no full-suite PASS is claimed. |
| DEC-025 trigger | `DEFERRED_TO_REQUIRED_POST_COMMIT_GATE` | Changed `validation/**` triggers `piping-pytest` and `evidence-sweep`; changed docs/execution trigger harness. DEC-025 §5.1 requires clean committed HEAD, then sweep, then evidence-only commit. Dry-run enumerated all five surfaces and exited 0. |
| Ignored drift | `FAIL` | At terminal return, exactly the three verifier-created target roots below remain ignored. Zero-ignored-drift is false. |

## Exact ignored-side-effect chronology

1. At verifier start, ignored inventory contained only V-D's regular,
   non-symlink `projects/chirality-piping/core/runner/headless/Cargo.lock`,
   10,114 bytes, SHA-256
   `7a3bd7e0df41a07e5c503aa312734e95fa6625afcd8b12f1f7994bd7a75b2e66`.
2. The verifier command
   `PYTHONDONTWRITEBYTECODE=1 pytest -q -p no:cacheprovider tools/practitioner_harness`
   ran from the repository root and passed 349/349 in 28.89 seconds. It was
   not given `CARGO_TARGET_DIR`; its Cargo-using subprocesses created three
   ignored roots. Top-directory mtimes span
   `2026-08-11T10:08:11-0600` through `2026-08-11T10:08:26-0600`.
3. Ignored inventory then contained the original lock plus the three target
   roots. No fifth ignored root was present.
4. The owner separately authorized cleanup of the exact original lock. A
   different managed cleanup child removed it; terminal read-only backcheck
   finds it absent. This verifier neither touched nor deleted it.
5. Terminal ignored inventory is exactly the three roots below. Staging is
   empty and no new non-ignored path arose from verifier validation.

## Exact verifier-created target-root inventory

Fingerprint algorithm: recursively sort every descendant by root-relative
path; for each emit
`kind + NUL + relative_path + NUL + octal_lstat_mode + NUL + lstat_size + NUL + content_sha256_or_dash + LF`, where kind is `f`, `d`, `l`, or `o`; hash the concatenated bytes with SHA-256. Root itself is excluded from the row stream. `regular_bytes` is the exact sum of regular-file `lstat_size` values.

| Ignored root | Top type | Regular files | Descendant dirs | Symlinks | Other | Regular bytes | Fingerprint SHA-256 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| `projects/chirality-piping/core/reporting/protected_content_linter/target/` | directory, non-symlink | 277 | 12 | 0 | 0 | 13,642,088 | `cfc574729e35b6b636944869cfd029b3067564736bcd76694e225abcd79406e7` |
| `projects/chirality-piping/core/runner/headless/target/` | directory, non-symlink | 4,283 | 155 | 0 | 0 | 547,137,823 | `4e5b9853191683ba1d2956579709bfa3b9fed4746a3e0c94fb3664c4c62d19a2` |
| `projects/chirality-piping/validation/benchmarks/nonlinear/target/` | directory, non-symlink | 1,189 | 36 | 0 | 0 | 101,489,154 | `173e930b12f32004c7aa32581f7d4e2cec6ae871d20e5e7343d467a44aa86aa0` |

The deterministic walk found only regular files and directories: zero
symlinks and zero other filesystem objects. No nested mount was observed in
the traversal; no deletion or traversal-boundary workaround was attempted.

## Smallest safe cleanup authority

Authorize a fresh managed cleanup child to delete recursively **only** these
three exact ignored directory roots after it independently verifies:

1. each exact path resolves under this exact worktree;
2. each is a directory and not a symlink;
3. descendant counts, regular bytes, zero-symlink/zero-other findings, and the
   exact fingerprint above still match;
4. none is a mount point;
5. terminal deletion makes all three paths absent while preserving their
   parent directories and every other path.

Then rerun only non-writing containment/hash/generator checks. Do not use
`git clean`, a wildcard, an unresolved variable, or a broader Cargo-cache
target. Receipt, staging, commit, DEC-025 execution, push, PR, and merge remain
separately governed.

## Attestation

- I did not repair any semantic artifact or delete any side effect.
- I did not delegate.
- I did not stage, commit, fetch, push, open a PR, merge, rebase, reset, clean,
  or perform a network action.
- After detecting the target directories, I stopped mutation-capable tests
  and limited remaining work to read-only inventory plus these two authorized
  runtime records.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
