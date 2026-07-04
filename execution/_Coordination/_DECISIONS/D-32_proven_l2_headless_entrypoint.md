# D-32 — Proven-L2 demonstration scoping: headless CLI entrypoint + validated-kernel run

**Status:** PROPOSAL — this packet confers no authority, implements nothing,
and rules nothing. Only the human project authority rules (K-AUTH-1;
D-GOV-04). Register row: `D-32` (`_REGISTER.md`), state `AWAITING_RULING`.

**Date prepared:** 2026-07-04 (bridge assessment decision slate item 3:
"direct scoping of a governed piping deliverable (or a fence-scoped brief)
for the headless CLI entrypoint + validated-kernel run"; owner in-session
blanket slate direction, verbatim in `_DomainEngines/bridge/LOOP_RECEIPTS.md`
Receipt 20).

**Basis rows:** `D-T0-08` (proven-L2 precondition), `D-22`/`DEC-041`
(sidecar staging), `D-31`/`DEC-063` (automation condition declared met —
proven L2 is now the sole engineering prerequisite on the live-binding path).

**Structural precedent:** the eight 2026-07-04 PROPOSAL packets in this
directory (D-06, D-11, D-12, D-10b, D-04b, D-05b, D-07b, D-20), themselves
on the D-28 packet skeleton — PROPOSAL disclaimer, verified-facts table,
options, non-binding recommendation, open human-ruling section, ruling
mechanism.

## 1. Decision

How and where the piping headless-runner CLI entrypoint and a
validated-kernel-run demonstration execute, establishing the "proven L2"
that `D-T0-08` requires before app-dev fence F3 can open. Three sub-questions:

1. **Vehicle** — in-DEL-10-05 bounded tranche vs a new tranche/deliverable.
2. **Command shape** — provisional entrypoint syntax vs settling final CLI
   syntax (which DEL-10-05's records explicitly reserve).
3. **Evidence definition** — what artifact constitutes a demonstrated
   validated-kernel run for tier-0 consumption.

## 2. Verified facts (checked cold 2026-07-04)

| Fact | Source |
|---|---|
| DEL-10-05 "Headless CLI and structured I/O analysis runner" (PKG-10) owns the headless runner surface: SOW-054 → PKG-10 → DEL-10-05 | `execution/_Decomposition/SOFTWARE_DECOMP.md:511` (SOW map), `:215` (PKG-10 scope incl. "headless execution"), `:345` (DEL row) |
| The code is lib-only: `core/runner/headless/Cargo.toml` declares `[lib]` with no `[[bin]]` target; package `open_pipe_stress_headless_runner` | `core/runner/headless/Cargo.toml` |
| The tier-0 ADOPTED profile marks `headless_runner` as the L2 candidate, `mode: read_only`, `status: "TOOLMAKER handoff — entrypoint not yet built"`, impl note "needs a thin CLI entrypoint, lib.rs:655" | `_DomainEngines/profiles/open_pipe_stress.yaml:105-111` (repo root) |
| D-T0-08 ruled the F3 sequence "Sequential — source types first, then domain MCP tools; not before D-T0-01 (ruled) + a proven L2. Each step its own PROPOSAL packet." | `_DomainEngines/_DECISIONS/D-T0-08_fence3_sequence.md:16` (repo root) |
| DEL-10-05 is `IN_PROGRESS`; its Specification scopes the setup run only ("does not implement CLI/source code…") and its Datasheet holds "Exact CLI command names", "Exact structured input schema fields", and "Public API transport" as TBD | DEL-10-05 `_STATUS.md:3`, `Specification.md:5,7`, `Datasheet.md:29-31` |
| The 2026-06-07 DEL-10-05 REVIEW transition "does not authorize final CLI/API syntax, package scripts, CI/release decisions, DAG promotion, or professional/code-compliance claims" | DEL-10-05 `_STATUS.md` (2026-06-07 history entry) |
| No existing brief, plan, or backlog scopes the entrypoint as a governed unit of work; the gap is only noted descriptively | `plans/artifacts/bridge_piping_contribution_for_tier0_2026-06-21.md:219-220`; grep of `plans/` + `execution/` 2026-07-04 |
| Bounded in-DEL work records use the `WORKING_ITEMS_RUN_<date>_<TP-ID>.md` format (Scope / Evidence / Validation / Boundary) | `execution/PKG-10_Build, Packaging, API, and Interoperability/1_Working/DEL-10-05_Headless CLI and structured I-O analysis runner/_run_records/WORKING_ITEMS_RUN_2026-06-18_TP-UNITS-BTAIL-HEADLESSRUNNERLINTUNITS-001.md` |
| The bridge loop's standing constraints prohibit its agent writing `core/**`; execution of this scoping therefore needs an explicit owner write-scope grant or the piping lane's own vehicle | `_DomainEngines/bridge/WORKPLAN_2026-07-02_bridge_loop.md` standing constraints |

## 3. Why this is human-gated

DEL-10-05's own governed records reserve CLI syntax decisions to a human
gate; the work writes `core/**` (protected against the bridge agent's
standing scope); and the resulting "proven L2" claim is a tier-0 gate input
(`D-T0-08`) that only the owner should cause to exist. K-INVENT-1 forbids
inventing the evidence bar.

## 4. Options

- **O-A — bounded in-DEL-10-05 implementation tranche (thin entrypoint,
  provisional syntax, commit-bound run evidence).** Add a thin `[[bin]]`
  entrypoint to `core/runner/headless` wrapping the existing lib surface
  (the lib.rs:655 seam), with a command shape labeled PROVISIONAL — the
  Datasheet's "Exact CLI command names" TBD stays TBD and no final CLI
  syntax claim is made, honoring the 2026-06-07 boundary wording. The
  validated-kernel-run demonstration: run the existing validated kernel
  fixtures through the entrypoint with structured I/O captured, recorded as
  a commit-bound `WORKING_ITEMS_RUN` record in DEL-10-05 `_run_records/`
  plus the run artifacts, closing with the `DEC-025` five-surface sweep.
  The proven-L2 observation is then available to tier-0; the profile's
  `status:` line updates only via a subsequent tier-0 CHANGE (profile text
  is CHANGE-gated).
- **O-B — new deliverable/tranche outside DEL-10-05.** Rejected shape:
  splits ownership of a surface SOW-054 already assigns to DEL-10-05 and
  duplicates its governance records.
- **O-C — defer to R7 planning.** Keeps everything closed; the F3 sequence
  and live binding stay parked on the missing L2 proof indefinitely.

## 5. Recommended Disposition (PROPOSAL)

Recommend **O-A**, with these riders:

1. Provisional command syntax only; DEL-10-05 Datasheet TBD rows stay TBD;
   no final CLI/API syntax, package-script, CI/release, DAG, lifecycle,
   release-readiness, professional, or code-compliance claim.
2. Evidence bar (proposal values for the ruling to set): at least one
   validated-kernel fixture run through the entrypoint on a clean head,
   inputs/outputs/exit status recorded commit-bound; `read_only` posture
   preserved (no mutation surface added).
3. Execution vehicle: an owner-directed piping-lane tranche (or an
   explicitly granted write scope if the bridge loop executes), branch-first,
   closing with the `DEC-025` sweep and an adversarial review.
4. The tier-0 "proven L2" acknowledgment (and any profile `status:` update)
   is a separate tier-0 act after the evidence exists — not part of this
   tranche.

This recommendation is a PROPOSAL only. It confers no authority, writes no
code, and makes no claim about L2.

## 6. Human Ruling And Disposition

**Ruling recorded:** O-A approved as recommended by owner (Ryan Tufts), 2026-07-04 — in-session direction "Merge PR #42 and rule D-32 O-A as recommended." (verbatim in `_DomainEngines/bridge/LOOP_RECEIPTS.md` Receipt 21), adopting §5 as written including all four riders: provisional command syntax only (DEL-10-05 Datasheet TBD rows stay TBD; no final CLI/API syntax, package-script, CI/release, DAG, lifecycle, release-readiness, professional, or code-compliance claim); the §5.2 evidence bar (at least one validated-kernel fixture run through the entrypoint on a clean head, inputs/outputs/exit status recorded commit-bound; `read_only` posture preserved); execution as an owner-directed piping-lane tranche or under an explicitly granted write scope, branch-first, closing with the `DEC-025` sweep and an adversarial review; and the tier-0 "proven L2" acknowledgment as a separate tier-0 act after the evidence exists. Codified as `DEC-064` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-32 → RULED. The implementation tranche is now lawful under those riders; the vehicle/write-scope direction (rider 3) is the remaining owner act before code executes.

## 7. Ruling Mechanism

Per existing practice, the human project authority selects an option (with
any riders/edits) or rules directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; register row `D-32` then moves from `AWAITING_RULING` to
`RULED` with the decision pointer. The implementation tranche itself executes
only after and under that ruling, in the ruled vehicle, with the ruled write
scope. This packet does not edit the register, the decomposition, any
`core/**` path, or the tier-0 profile, and it does not resolve the decision.
