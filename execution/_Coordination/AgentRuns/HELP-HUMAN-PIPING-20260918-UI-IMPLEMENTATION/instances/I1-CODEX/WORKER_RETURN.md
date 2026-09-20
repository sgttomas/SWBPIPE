# I1-CODEX instrument worker return

TASK Type 2 under I1-CODEX WORKING_ITEMS, itself under ROOT HELP_HUMAN.
Actual engine/model: Codex / gpt-6-astra, low reasoning; delegated-harness-native.
No child delegation or Git mutations. The assigned worktree boundary was instruction-enforced
on an unrestricted host. Supplied HEAD: `81ab9de1ed3162c87b638a7811d557a579a5bc4a`;
existing I1 implementation: `240a246f34f4771fb91f760e065f9ef11f9910da`.

## Changes

- Historical geometry remains pinned to c0c09ebd at 8468a33c. Independently checked
  the preserved preimage against that commit, and confirmed current product fdf3eaa4.
  Replaced incidental prose assertions with a preserved-byte/hash control; existing
  source-pin admission coverage remains in the controller suite.
- Prepended only six comment lines to the historical 97b18c96 plan verifier. Original
  file bytes remain an exact suffix. Header names the attachment-centre transition,
  required old/new input directories and unavailable-input limitation.
- Kept the extracted D70 inventory validator and original 34-entry unique-path acceptance.
  Replaced dependence on future controller inventory contents with direct behavioral
  admission/refusal cases (34 accepts; 0/33/35/38, absent files and duplicate paths refuse).
  The manager independently checks nine recorded manifests; that result is not my review.
- Kept generator preflight, history pins, fallback and generation behavior unchanged.
  Every maintained generator invocation now executes in a scratch copy, including
  potentially writing calls. Tests cover no-history reproduction, malformed/unknown
  arguments, absent/invalid supplied history, invalid default history, and changed/missing
  frozen-byte negative controls. Content and mtimes are compared after each invocation.
- Named instrument reporting change: changed the newly introduced, unmerged missing-history
  token to `NOT_SUPPLIED_NINE_PINNED_FILES`; refusal now describes missing supplied/default
  history. Consumer search found only this I1 code/test/docs and historical evidence.
  Original return/raw evidence remains unchanged; README and appended completion account
  distinguish recovered canonical run evidence from the original unavailable-input account.

## Verification

- Full source `full-cohort-controller.spec.ts`, both projects, locked with one worker:
  92 passed, 20 skipped, exit 0. This run precedes the reporting-only token correction.
- TypeScript (`npx tsc --noEmit`) and verifier Node syntax check passed.
- All 13 protected fixture/sample/manifest files equal supplied HEAD byte-for-byte.
- Verifier original executable suffix unchanged; no product file, oracle, tolerance,
  target, policy, historical count, fixture or sample changed.
- Final focused reporting backcheck, both source projects: 6 passed, exit 0.
- Deliberate current-source override: 1 expected refusal at the geometry hash comparison
  (c0c09ebd expected, fdf3eaa4 observed), exit 1; trace and context retained separately.

Raw outputs and hashes: `_run_records/worker/` (full-cohort-controller.log,
static-*.log, frozen-equality.json, geometry-pins.json, header-byte-check.json,
source-hashes.json, diff-check.log and worker.diff). Successful temporary scratch trees
were inventoried by hash and removed; logs and Playwright result metadata retained.
Input origins/hashes are in `_run_records/WORKER_INPUTS.json`; current source hashes are
in the worker evidence. Browser work used the existing run lock and source port 5174;
all runs released their locks and final inspection found no listener on 5174.
Large synthetic raw trace-cap files are losslessly gzip-compressed with original hashes
and decompression equality in compressed-raw-inventory.json. No failure trace was discarded.

## Limits and handoff

No timed benchmark, full source/dist sweep, native/CUA or successful writing-mode generation.
The historical plan verifier was not executed because its old/new oracle directories are
not supplied. Environment-bound D70/product-profile tests remain skipped, not passed.
Manager owns recovered-history custody, explicit-directory verification, raw D70 compatibility,
final picking and independent checks. ROOT owns review, sweep and Git integration.
This is implementer verification, not independent review, product acceptance, performance,
usability, engineering validation or release. Standard claim fence F-PIP-2 applies.
