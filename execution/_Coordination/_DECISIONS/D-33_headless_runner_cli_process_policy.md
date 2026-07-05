# D-33 - Headless Runner Final CLI And Process Policy

**Date prepared:** 2026-07-05
**Prepared by:** piping loop agent during Step 0/selection for the PRD R5
Phase E E1 lead-up.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, implements a CLI,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met.

---

## 1. Decision Statement And Scope

Decide the final public/runtime policy for the DEL-10-05 headless runner CLI
needed by completion-plan Phase E row E1: command family, operation mapping,
structured input/output posture, process policy, filesystem policy, network
policy, and package-script posture.

**In scope:** the policy boundary for a later bounded implementation tranche:
final command names and subcommands, how the existing schema-first
`RunnerOperation` vocabulary maps to CLI verbs, whether input is JSON request
only or includes shorthand flags, stdout vs explicit local file output, exit
codes, process/network/filesystem constraints, and package-script exposure.

**Out of scope:** implementation; release packaging/signing/publication
(`D-06`/`D-06b`); hosted/public-export CI (`D-05b`); protected-content release
scan execution (`D-20`); final R5 exit acceptance; lifecycle issuance; public
repository creation; local FEA export disposition (`D-12`); and any live
agent binding or tier-0 profile change.

## 2. Verified Facts (Checked 2026-07-05)

| Fact | Source |
|---|---|
| The current target stage is PRD R5 / Phase E after `DEC-054`; R5 ordinary work includes engineering beta and release machinery. | `execution/_Coordination/_COORDINATION.md` current target-stage section; `execution/_Decomposition/SOFTWARE_DECOMP.md` `DEC-054`. |
| Phase E row E1 is the earliest current-stage dependency-spine item: add the headless runner CLI/process policy for schema-driven `solve`, `validate-input`, `export`, `run-benchmark`, and `run-regression` operations; resolve DEL-10-05 CLI-name/schema/process TBDs. | `plans/PLAN_2026-06-17_prd_completion.md` Phase E row E1. |
| `D-32` / `DEC-064` authorized and scoped only a PROVISIONAL thin entrypoint + validated-kernel run. Its ruling deliberately preserved DEL-10-05 Datasheet TBD rows and made no final CLI/API syntax, package-script, process policy, CI/release, lifecycle, release-readiness, professional, or code-compliance claim. | `execution/_Coordination/_DECISIONS/D-32_proven_l2_headless_entrypoint.md`; `execution/_Decomposition/SOFTWARE_DECOMP.md` `DEC-064`; DEL-10-05 run record `WORKING_ITEMS_RUN_2026-07-04_TP-RUNNER-014.md`. |
| The provisional entrypoint exists and is not final syntax: `core/runner/headless/src/bin/headless_preview_runner.rs` labels its command shape PROVISIONAL and reads one preview-model fixture path. | `core/runner/headless/src/bin/headless_preview_runner.rs`. |
| DEL-10-05 remains `IN_PROGRESS`; its setup records still reserve exact CLI command names, exact structured input schema fields, public API transport, package scripts, process invocation, network access, and filesystem mutation policy. | DEL-10-05 `_STATUS.md`, `Datasheet.md`, `Specification.md`, `MEMORY.md`; `docs/SPEC.md` runner section; `docs/TYPES.md` `HeadlessRunnerEnvelope` row. |
| The schema and Rust crate already name the operation categories: `solve`, `validate_input`, `export_results`, `run_benchmark`, `run_regression`, and `TBD`. | `schemas/headless_runner.schema.yaml` `$defs.RunnerOperation`; `core/runner/headless/src/lib.rs` `RunnerOperation`. |
| Current DEC-025 gate entrypoint exists as `tools/release/run_evidence_sweep.py`; dry-run lists five surfaces and exits 0. | Step 0 command `PYTHONDONTWRITEBYTECODE=1 python3 tools/release/run_evidence_sweep.py`. |
| Current lifecycle census is `CHECKING=8`, `IN_PROGRESS=92`, `ISSUED=1`; DEL-10-05 is present in `DAG-007` and remains `IN_PROGRESS`. | `python3 tools/coordination/list_deliverable_status.py --dag DAG-007 --format table --summary`. |

## 3. Why This Is Human-Gated

DEL-10-05's own governed records explicitly reserve final CLI syntax and
process policy for later sealed scope or human approval. Choosing command
names, default output behavior, package-script exposure, or filesystem/network
capabilities creates a public/runtime surface and can affect reproducibility,
privacy, release packaging, and the R5 validation-manual path. Agents may
prepare this packet, but they must not settle those choices.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Minimal stable local CLI policy for E1.** Adopt one stable binary name and the existing operation vocabulary as subcommands; JSON request input remains the canonical structured input; stdout is the default result channel; explicit local output directories are allowed only when named by the operator; no network, daemon, telemetry, hidden filesystem mutation, repository-default private-data write, or direct SQL/SQLite bypass; package scripts are dev/test conveniences only, not release claims. | Unblocks a bounded DEL-10-05 implementation tranche that converts the provisional runner into an R5 external-reproducibility surface while preserving the schema-first and local-only boundary. |
| **O-B** | **Library/API only; keep CLI syntax provisional.** Continue using `headless_preview_runner` and library tests as evidence, but defer the final CLI/process policy. | Avoids choosing a public/runtime surface now, but leaves Phase E E1 and the E2 validation-manual reproduction path blocked on the same TBDs. |
| **O-C** | **Full release-grade CLI plus package scripts now.** Set final command names, package scripts, persistent project-container behavior, artifact directories, and release invocation shape in one ruling. | Broadest user-facing path, but couples E1 to release packaging/signing/publication, project-container, redaction, and protected-content scan surfaces better handled by D-06/D-06b/D-20/D-12 and later implementation evidence. |
| **O-D** | **New API/transport first, CLI later.** Route E1 through a local service or public transport layer, then expose CLI as a client later. | Conflicts with the local-first/no-daemon default and invents a transport decision where DEL-10-05 still records public transport as TBD. |

### 4.1 O-A Proposed Boundaries

If O-A is selected, the later implementation tranche should be bounded this
way unless the ruling edits it:

- Stable binary name: human-selected in the ruling; suggested
  `openpipestress-runner` because it is explicit and product-scoped.
- Stable verbs map one-to-one to the existing runner operation vocabulary:
  `solve`, `validate-input`, `export-results`, `run-benchmark`,
  `run-regression`.
- Canonical input is a schema-first JSON request file or stdin payload. No
  shorthand flags become authority for engineering data in the first tranche.
- Canonical output is structured JSON on stdout. Optional file output requires
  an explicit operator-provided path and never defaults to repository private
  data, user home scanning, hidden temp persistence, network egress, or daemon
  state.
- Exit codes: `0` for completed/no blocking runner diagnostics; `1` for
  completed-with-blocking-diagnostics or validation failure; `2` for usage,
  unreadable input, malformed JSON, or unsupported operation. The later
  implementation may refine this if the run record proves a better shape.
- Process policy: single foreground local process; no spawned solver sidecar,
  no background service, no network listener, no telemetry.
- Evidence: crate tests, schema tests, at least one invented solve run, at
  least one validation/blocking-diagnostic run, at least one export-result or
  benchmark/regression evidence stub if the operation is not fully implemented
  yet, DEL-10-05 run record, and DEC-025 sweep before closeout.

## 5. Recommended Disposition (PROPOSAL)

Recommend **O-A**.

Rationale: E1 needs a real reproducibility surface, not another provisional
fixture-only command, but the narrowest honest surface is still local,
schema-first, and operation-vocabulary-aligned. O-A settles only the public
runtime policy needed to write the next DEL-10-05 implementation tranche. It
does not drag in release packaging, public transport, signing/notarization,
publication, protected-content scan execution, local FEA export, or R5 exit
acceptance.

If the owner wants to keep all command naming open until more validation-manual
content exists, O-B is the coherent hold. O-C and O-D are not recommended
because they widen E1 into adjacent governed surfaces before the runner itself
has a stable, local, schema-first command.

## 6. On-Ruling Mechanism

If the owner selects O-A or edits it into another executable policy:

1. Append the ruling to `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as
   the next `DEC-*` entry citing this packet.
2. Update register row `D-33` to `RULED` with that `DEC-*` pointer.
3. Execute a bounded DEL-10-05 implementation tranche, suggested id
   `TP-RUNNER-015`, with write scope limited to the ruled runner/code/schema
   surfaces, DEL-10-05 evidence, and plan/log maintenance required by the
   tranche.
4. Preserve all boundary prohibitions: no protected standards content, private
   project data, repository-default private-data writes, network/daemon/
   telemetry feature, release-readiness claim, professional approval,
   certification, sealing, authentication, or code-compliance claim.

If the owner selects O-B, leave E1 parked and continue to the next unblocked
Phase E implementation item only if it does not depend on E1's reproducibility
surface. If no such item remains, stop at the pending-rulings slate.

## 7. Human Ruling And Disposition

Awaiting owner ruling.
