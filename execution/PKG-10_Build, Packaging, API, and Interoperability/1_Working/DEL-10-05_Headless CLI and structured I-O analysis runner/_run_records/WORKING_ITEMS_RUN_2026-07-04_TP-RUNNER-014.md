# WORKING_ITEMS Run Record - TP-RUNNER-014
Date: 2026-07-04
Persona: WORKING_ITEMS
Primary deliverable: DEL-10-05 - Headless CLI and structured I-O analysis runner
Package: PKG-10 - Build, Packaging, API, and Interoperability
Tranche: TP-RUNNER-014 (DEC-064 / D-32 O-A proven-L2 entrypoint)

## Scope

Bounded implementation tranche ruled by `DEC-064`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row `D-32`
RULED; owner rider-3 vehicle/write-scope grant recorded verbatim in
`_DomainEngines/bridge/LOOP_RECEIPTS.md` Receipt 22): add a thin
PROVISIONAL `[[bin]]` entrypoint wrapping the existing
`core/runner/headless` library surface and demonstrate at least one
validated-kernel fixture run through it on a clean head, with
inputs/outputs/exit status recorded commit-bound. The command shape is
PROVISIONAL — no final CLI syntax is claimed or settled (DEC-064 rider 1).

## Changes

- Added `core/runner/headless/src/bin/headless_preview_runner.rs` (cargo
  bin auto-discovery; `Cargo.toml` byte-untouched; zero new dependencies).
  The bin constructs the schema-first `RunnerRequest` in code (the headless
  request types are Serialize-only by design — no request-JSON parsing
  surface was added), loads the `LinearStaticPreviewRequest` model from the
  caller-named fixture path, executes `run_preview_in_memory` (the
  validated-kernel preview solve), emits the structured
  `PreviewRunnerOutput` JSON on stdout, and derives its exit status from
  the validation surface (0 clean; 1 blocking diagnostics; 2 usage/input
  errors). One bin-level unit test asserts the in-code request is
  schema-complete under `validate_request`.
- No library code, schema, fixture, or manifest changed.

## Evidence

Validated-kernel run (inputs / outputs / exit status, commit-bound to the
tranche code commit `035e25991`):

- Input: `fixtures/product_preview/invented_preview_model.json` (the
  invented, non-engineering preview model the library tests certify), via
  `cargo run --manifest-path core/runner/headless/Cargo.toml --bin
  headless_preview_runner -- fixtures/product_preview/invented_preview_model.json`
  (invocation PROVISIONAL).
- Exit status: `0`.
- Output (full captured stdout committed at
  `validation/witness/generated/tp_runner_014_headless_entrypoint_preview_run.json`):
  job `Completed`; mechanics status `MECHANICS_SOLVED` with rule_check
  `RULE_INPUTS_INCOMPLETE` (solve-only default) and
  professional_acceptance `NOT_PROVIDED`; `analysis_status`
  `[HumanReviewRequired, MechanicsSolved, RuleInputsIncomplete]`; 822
  result refs including `result:stress:pipe-P-120:end-j:torsional-shear`
  (the library test's golden signal); zero diagnostics; SHA-256 checksums
  `runner_request` `3eb2516d14a19a98c74d23a9b6912cdf32a96a5a11b49e6822c4128cbaa4a70b`
  and `result_envelope`
  `60cc21fcdab1311834a31aa4d5cd7bbe347dbca4ad81db4e5bf3d972b3ef905f`.
- Error-path behaviour: no-argument and unreadable-path invocations exit
  `2` with a usage/read message on stderr (no output emitted).

## Validation

Passed:

- headless crate `cargo test`: 17/17 (16 library + 1 bin), `cargo fmt
  --check` clean.
- entrypoint validated-kernel run: exit 0 with the Evidence values above.
- DEC-025 five-surface sweep: all five surfaces pass. A working-tree
  pre-run bound to code commit `035e25991` passed 5/5 (recorded `-dirty`
  by concurrent untracked staging files; not committed); the merge-gate
  sweep runs at this tranche's clean committed HEAD and its artifact is
  committed as the evidence-only closeout commit per
  `docs/BUILD_AND_RELEASE.md` §5.1.

## Boundary

No final CLI syntax, package scripts, process-invocation policy, network
policy, filesystem-mutation policy, CI/release decision, DAG change,
schema change, unit-conversion change, protected standards content,
private payload, lifecycle state (`_STATUS.md` untouched, stays
IN_PROGRESS), Datasheet TBD row, release-readiness status, professional
approval, certification, sealing, authentication, or code-compliance
status changed. The tier-0 profile is byte-untouched by this tranche: the
proven-L2 acknowledgment and any profile `status:` update are a separate
owner/tier-0 act (DEC-064 rider 4; staged at
`_DomainEngines/bridge/CHANGE_PREP_2026-07-04_proven_l2_acknowledgment.md`,
register row `D-T0-10`). `read_only` posture preserved: the entrypoint
reads the caller-named input and writes only stdout/stderr.
