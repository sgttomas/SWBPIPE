# D-04b - Coverage Tooling Selection And Numeric-Floor Promotion

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide the two matters `DEC-024` explicitly deferred to this named follow-up
(register row `D-04b`, `execution/_Coordination/_DECISIONS/_REGISTER.md:43`):

1. **Coverage tooling selection** — which per-lane tools (Rust, TS/Vitest,
   Python) measure numeric coverage, and how the resulting numbers enter the
   `DEC-025` local evidence-sweep discipline as **recorded-not-blocking
   telemetry**.
2. **Numeric-floor promotion** — whether and under what trigger any observed
   coverage baseline is later promoted to a blocking numeric floor, honoring
   `DEC-024`'s condition that promotion happens only "from observed
   baselines" and the tighten-only spirit of the `DEC-026` riders.

**In scope:** tool selection per language lane; the telemetry artifact shape
and where it slots relative to the five-surface sweep; the evidence
precondition for any future floor promotion; the dev-tooling dependency
posture.

**Out of scope:** the blocking coverage mechanism itself — `DEC-024` already
ruled that blocking coverage is the enforced surface-inventory gates
(contract-corpus floor precedent), and that is not reopened here; the
tolerance policy (`DEC-026`, not touched); CI residency of any coverage run
(`D-05`/`D-05b` territory — everything here is local-only per `DEC-025`);
performance thresholds (`docs/RELEASE_QUALITY_GATES.md:158`, separate TBD).

This packet decides nothing. No coverage number asserted anywhere below is a
gate, and no gate is created by preparation.

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| Register row | `execution/_Coordination/_DECISIONS/_REGISTER.md:43` — `D-04b` "Coverage tooling selection and any numeric-floor promotion from observed baselines (deferred from the D-04 C-C ruling, `DEC-024`)", state `NOT_PREPARED`, blocks "Coverage telemetry in evidence sweeps". |
| `DEC-024` deferral text | `execution/_Decomposition/SOFTWARE_DECOMP.md:602` — coverage policy is "blocking enforced surface-inventory gates (extending the contract-corpus coverage-floor precedent) plus numeric coverage measured and recorded as explicitly non-blocking telemetry **once tooling is selected (named follow-up D-04b)**, with **no numeric floor blocking until a later human ruling promotes one from observed baselines**." |
| `DEC-026` confirmation | `execution/_Decomposition/SOFTWARE_DECOMP.md:604` — the same-day Part 1 revision states "Part 2 of `DEC-024` (C-C coverage posture, D-04b follow-up) is unchanged", and carries the tighten-only rider pattern: "fixture-local `tolerance_policy` overrides may only tighten — any loosening of a governed value is itself a governance event". |
| Parent packet C-C definition | `execution/_Coordination/_DECISIONS/D-04_tolerance_coverage_thresholds.md:124-129` (Option C-C) and `:138` (recommendation): numeric coverage is "measured and recorded in local evidence sweeps once tooling is selected (a named follow-up, not this ruling) ... explicitly non-blocking until a later human ruling promotes any floor based on observed baselines." |
| Coverage tooling configured today | **None.** Repo-wide search 2026-07-04 for `llvm-cov`, `tarpaulin`, `grcov`, and coverage configuration across `*.toml`, `package.json` manifests, `tools/` scripts, and test configs finds no coverage tool anywhere; the only "coverage" hits are domain features (parser coverage, corpus inventory floors). Matches the parent packet's 2026-06-11 finding (`D-04_tolerance_coverage_thresholds.md:56`). |
| Rust lane shape | 34 `Cargo.toml` manifests (excluding `node_modules/`, `.archive/`), per-crate with **no workspace root**; the sweep's cargo surface discovers manifests under `core/` and `validation/benchmarks/` and runs `cargo test --manifest-path` per crate (`tools/release/check_release_readiness.py:27-30, 69-80, 145-150`). |
| TS lane shape | Desktop Vitest `^4.1.7` (`apps/desktop/package.json:36`), `"test": "vitest run"` (`:13`), test config inline in `apps/desktop/vite.config.ts:74-87`; **no coverage provider package installed and no `coverage` key configured**. |
| Python lane shape | Sweep runs `python -m pytest -q tests` (`tools/release/run_evidence_sweep.py:81-83`); `requirements-dev.txt` is a single line (`jsonschema>=4,<5`) — no `coverage`/`pytest-cov`; no `pytest.ini`/`pyproject.toml`/`setup.cfg` at repo root. |
| Five-surface sweep (`DEC-025`) | `tools/release/run_evidence_sweep.py:1-17, 57-110` — five surfaces in F-4-safe sequential order: `cargo_crate_sweep` (`:68`), `python_pytest` (`:81`), `desktop_vitest` (`:86`), `desktop_playwright_e2e` (`:94`), `desktop_production_build` (`:106`); fail-fast (`:201-204`); commit-bound summary artifact, `SCHEMA_VERSION = 2` (`:36`), written to `validation/evidence/sweeps/` (`:39`); `missing_tools()` pre-checks executables before execution (`:304-311`). |
| Observed coverage baselines | **None exist.** `validation/evidence/` contains only `sweeps/` (236 pass/fail sweep summaries; no coverage numbers in the artifact schema); no numeric coverage percentage appears in any `plans/VERIFICATION_*.md`. The "observed baselines" evidence base for floor promotion is **empty**. |
| Open TBD this closes | `docs/RELEASE_QUALITY_GATES.md:159-160` — "TBD: coverage thresholds for Rust, Python, GUI, validation, and protected-content gates" remains open. |
| Host runtime (latest accepted sweep) | `validation/evidence/sweeps/SWEEP_20260623T051552Z_16cca07f3b64.json:14-20` — macOS arm64 (`macOS-26.5.1-arm64`), Python 3.13.7, Node v24.5.0, cargo/rustc 1.92.0. No `rust-toolchain` pin file found in-repo. |
| Dependency-posture precedent | `DEC-023` (`execution/_Decomposition/SOFTWARE_DECOMP.md:601`): the solver ruling chose "zero new numerical dependencies" — a posture about the shipped computational core, not about host-installed dev tooling. |
| Residual-work row convention | `_REGISTER.md:17-24` (owner-ruled 2026-07-03): residue from a ruling gets its own new `D-XX` row citing the ruling as basis; ruled rows stay immutable. |

**Unresolved (explicitly not asserted as fact):** characterizations of the
candidate external tools in §4 (platform support, rustup integration,
profraw plumbing) are agent general knowledge, **not verifiable from any repo
file**, and are labeled `ASSUMPTION`. Whether per-crate coverage across 34
non-workspace manifests aggregates cleanly, and the exact Vitest-4-compatible
coverage-provider package version, are unverified until an implementation
spike runs. No claim in this packet depends on these resolving one way or the
other; the recommendation makes tool viability itself a recorded checkpoint.

## 3. What DEC-024 Ruled Versus What Is Open Here

The C-C ruling is two-layered and only the second layer is open:

- **Ruled, standing, not reopened:** blocking coverage = enforced surface
  inventories (corpus kind/block-class floor, benchmark-family presence,
  Must-FR evidence map) per `DEC-024` (`SOFTWARE_DECOMP.md:602`), reaffirmed
  untouched by `DEC-026` (`:604`). Numeric coverage is telemetry,
  non-blocking, until a **later human ruling** promotes a floor **from
  observed baselines**.
- **Deferred to D-04b (this packet):** which tools produce the telemetry,
  where the numbers are recorded relative to the `DEC-025` sweep, and what
  the promotion trigger is.

Because zero observed baselines exist (§2), the "numeric-floor promotion"
half of this decision **cannot be substantively ruled today without inventing
numbers** — which `DEC-024`'s own text forbids and K-INVENT-1 forbids. The
honest option space therefore includes ruling the tooling and the telemetry
seam now while ruling only the *promotion mechanism and trigger* (not any
value) for floors.

## 4. Candidate Tooling Per Lane

All tool characterizations in this section are `ASSUMPTION` (external
knowledge, not repo-verifiable — see §2 Unresolved). Repo-side facts are
cited.

**Rust (34 per-crate manifests, no workspace; macOS arm64 host, stable rustc
1.92.0):**

- `cargo-llvm-cov` — host-installed cargo subcommand using rustc's built-in
  LLVM source-based instrumentation plus the `llvm-tools-preview` rustup
  component; works on stable, macOS arm64 included; supports `--manifest-path`
  invocation, so it can walk the same manifest list
  `discover_cargo_manifests()` already produces
  (`tools/release/check_release_readiness.py:69-80`). `ASSUMPTION` as to
  capability; the manifest-walk seam is `FACT`.
- `cargo-tarpaulin` — its primary ptrace engine is Linux-oriented; macOS
  support goes through its own LLVM engine, duplicating the mechanism above
  with an additional third-party wrapper. `ASSUMPTION`.
- `grcov` — Mozilla aggregator over `.profraw` files; requires manual
  RUSTFLAGS/profraw plumbing per crate that `cargo-llvm-cov` automates.
  `ASSUMPTION`.

**TS (desktop, Vitest ^4.1.7):** Vitest's built-in `--coverage` mode with the
`@vitest/coverage-v8` provider package as a `devDependencies` addition to
`apps/desktop/package.json` — the vendor-native path; no new test runner.
`ASSUMPTION` as to provider mechanics; the Vitest version and absence of any
current provider are `FACT` (§2).

**Python (pytest over `tests/`):** `coverage.py` (`coverage run -m pytest`,
`coverage json`) added to `requirements-dev.txt` — the standard-library-
adjacent default; `pytest-cov` is a thin wrapper over the same engine.
`ASSUMPTION` as to mechanics; the current one-line `requirements-dev.txt` is
`FACT` (§2).

**Dependency posture:** all three additions are host-installed or
dev-manifest-scoped **development tooling**. `DEC-023`'s "zero new numerical
dependencies" (`SOFTWARE_DECOMP.md:601`) governs the shipped computational
core — coverage tools instrument test runs and link nothing into shipped
artifacts. Adopting them does not conflict with `DEC-023`, but the packet
notes the distinction so the precedent is weighed, not silently skipped.
The determinism concern the parent packet recorded against numeric floors
(`D-04_tolerance_coverage_thresholds.md:122`) applies to *gating* on
coverage, not to recording it: telemetry that never blocks cannot make the
merge gate nondeterministic.

## 5. Telemetry Seam: Where Coverage Enters The Sweep

The `DEC-025` merge gate is codified as a **five-surface** sweep with
fail-fast semantics and a versioned summary schema
(`tools/release/run_evidence_sweep.py:1-17, 36, 201-204`;
`SOFTWARE_DECOMP.md:603`). Two structurally distinct entry points exist:

1. **Inside the gate** — add a sixth surface to `build_sweep_plan()`. This
   changes the ruled "five-surface local sweep" wording of `DEC-025`, and
   fail-fast semantics mean a coverage-tool crash would block merges on
   telemetry — inverting the recorded-not-blocking posture.
2. **Beside the gate** — a separate opt-in runner (e.g.
   `tools/release/run_coverage_telemetry.py`) reusing the same commit-binding
   and tool-precheck patterns (`run_evidence_sweep.py:153-177, 304-311`),
   writing its own commit-bound artifact under a sibling directory (e.g.
   `validation/evidence/coverage/`). The five-surface gate is untouched; a
   telemetry record can cite the sweep summary of the same commit; gate
   records (plan E7, `plans/PLAN_2026-06-10_prd_completion.md:126` via the
   parent packet `D-04_tolerance_coverage_thresholds.md:62`) file the numbers
   as telemetry.

Option consequences below reference these as **seam 1** and **seam 2**.

## 6. Open Questions The Ruling Should Settle

1. **Baseline-count trigger values:** the §8 proposal names five clean-head
   telemetry artifacts across two distinct commits per lane before a
   promotion row may be prepared. These counts are proposal values with no
   measured basis (nothing to measure yet); the ruling may set different
   counts or delegate the counts to the promotion row itself.
2. **Cadence:** seam 2 telemetry is opt-in — the ruling may additionally name
   an expected cadence (e.g. once per verification-record preparation, or
   per fan-in) without making it merge-blocking. Silence leaves cadence to
   ordinary work selection.
3. **Rust aggregation form:** per-crate numbers are the natural output of the
   34-manifest walk (§2); whether a cross-crate aggregate percentage is also
   computed (and how crates are weighted) can be ruled here or left to the
   implementation tranche's recorded choice. An aggregate over non-workspace
   crates has no single canonical definition — the parent packet flagged
   exactly this awkwardness (`D-04_tolerance_coverage_thresholds.md:122`).
4. **Lane scope:** `DEC-024` names Rust, Python, and GUI lanes via the gate
   doc's TBD (`docs/RELEASE_QUALITY_GATES.md:159-160`). The Playwright e2e
   surface is exercised in-browser against a built bundle; instrumenting it
   is materially harder than the Vitest unit lane (`ASSUMPTION`), and this
   packet's O-A scopes TS telemetry to the Vitest lane only, leaving e2e
   coverage explicitly unmeasured. The ruling may confirm or extend that
   scope.

## 7. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Select the per-lane vendor-native toolset now (Rust: `cargo-llvm-cov`; TS: Vitest `--coverage` + `@vitest/coverage-v8`; Python: `coverage.py`); telemetry via seam 2 (separate opt-in commit-bound artifact, five-surface gate untouched); defer floor promotion until a named observed-baseline count exists, then a new residual `D-XX` row proposes tighten-only floors.** | Tooling unblocks immediately as bounded tranche work; `DEC-025`'s ruled gate text stays immutable; telemetry accumulates the evidence base `DEC-024` requires before any floor; no number is invented. First implementation tranche doubles as the feasibility spike for the §2 Unresolved items and records its findings. |
| **O-B** | Same toolset, but wired **inside** the sweep as a sixth surface (seam 1). | Coverage runs on every merge-gate invocation (maximal accumulation), but the ruled five-surface `DEC-025` wording must be superseded or forward-noted, and fail-fast semantics make tool breakage merge-blocking — telemetry acquires gate teeth `DEC-024` explicitly withheld. |
| **O-C** | Rust lane via `cargo-tarpaulin` or `grcov` instead of `cargo-llvm-cov`; TS/Python lanes as O-A. | Adds a third-party engine wrapper (tarpaulin) or manual profraw plumbing (grcov) on a macOS arm64 host where the `ASSUMPTION`-level platform fit is weakest (§4); no identified advantage over the rustc-native instrumentation path for a local-only posture. |
| **O-D** | Select tooling **and** promote initial numeric floors now (first measurement, or a conventional figure such as 80%). | Violates the standing ruling's own condition — zero observed baselines exist (§2), so any floor today is invented, not promoted "from observed baselines" (`SOFTWARE_DECOMP.md:602`); contradicts K-INVENT-1 and the parent packet's stated reason for rejecting C-B (`D-04_tolerance_coverage_thresholds.md:122`). |
| **O-E** | Defer tooling selection again. | Register row stays open; `docs/RELEASE_QUALITY_GATES.md:159-160` TBD persists; the telemetry evidence base for any future floor never starts accumulating — the deferral becomes self-perpetuating. |

## 8. Recommended Disposition (PROPOSAL)

Recommend **O-A**, with the following shape if accepted:

1. **Tooling (ruled by this decision):** Rust `cargo-llvm-cov` (host-installed
   subcommand + `llvm-tools-preview` component, walking the existing
   `discover_cargo_manifests()` list per crate), TS Vitest `--coverage` with
   `@vitest/coverage-v8` as a desktop `devDependencies` entry, Python
   `coverage.py` in `requirements-dev.txt`. Dev-tooling only; no runtime or
   numerical dependency is added, preserving the `DEC-023` distinction (§4).
2. **Telemetry seam (ruled by this decision):** seam 2 — a separate opt-in
   `tools/release/run_coverage_telemetry.py` emitting a commit-bound,
   schema-versioned JSON artifact to `validation/evidence/coverage/`,
   reusing the sweep's git-binding and `missing_tools` precheck patterns.
   Numbers are per-lane (and per-crate for Rust); **recorded, never
   blocking**; a missing tool is recorded as `not_measured`, never a
   failure. The five-surface `DEC-025` gate is not modified.
3. **Floor promotion (mechanism ruled; values explicitly not ruled):** no
   numeric floor exists under this ruling. Once at least **five** clean-head
   telemetry artifacts spanning at least **two** distinct commits exist per
   lane (counts are proposal values for the ruling to set), a new
   residual-work `D-XX` row (per the 2026-07-03 register convention,
   `_REGISTER.md:17-24`) may propose promoting observed minima to floors —
   **tighten-only thereafter**, per the `DEC-026` rider spirit: loosening a
   promoted floor is a new governance event, never a config edit.
4. **Feasibility checkpoint:** the first implementation tranche verifies the
   §2 Unresolved `ASSUMPTION`s (macOS arm64 tool viability, 34-manifest
   walk, Vitest-4 provider compatibility) and records the outcome in its
   verification record; if a lane's selected tool fails the spike, that lane
   reverts to `not_measured` and the failure returns here as a follow-up,
   rather than being patched silently.

Rationale: O-A is the only option that discharges both halves of the register
row without either inventing numbers (O-D), acquiring gate teeth `DEC-024`
withheld (O-B), or perpetuating the empty evidence base (O-E). It selects the
per-lane vendor-native instrumentation paths, keeps the ruled `DEC-025` gate
text immutable, and converts the floor question from an opinion into a
scheduled consequence of accumulated measurement — the same
measure-first-then-govern pattern the owner already chose in `DEC-026`
(seed at the measured value, `TBD` until measured) and `DEC-046` (unmeasured
entries stay `TBD`).

## 9. Downstream Impact If Accepted

| Surface | Impact |
|---|---|
| `docs/RELEASE_QUALITY_GATES.md:159-160` | Coverage-thresholds TBD closes to "ruled: telemetry per D-04b; floors deferred to observed-baseline promotion" with a pointer. |
| `tools/release/` | New opt-in telemetry runner; `run_evidence_sweep.py` and `check_release_readiness.py` unchanged. |
| `apps/desktop/package.json`, `requirements-dev.txt` | One devDependency and one dev requirement added at implementation, not by this ruling. |
| E7 gate records / plan E2 | Gate records gain a telemetry citation slot (numbers as data per the `DEC-021` slot-data posture, `D-04_tolerance_coverage_thresholds.md:63`). |
| Register | `D-04b` → `AWAITING_RULING` at preparation, `RULED` at ruling; any future floor promotion arrives as its own new row. |

No lifecycle, release-readiness, professional, certification, or milestone
claim arises from acceptance; implementation remains bounded-tranche work
evidenced through the ordinary `DEC-025` sweep discipline.

## 10. Human Ruling And Disposition

**Ruling recorded:** _Awaiting owner ruling._

## 11. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; register row `D-04b` then moves from `AWAITING_RULING` to
`RULED` with the decision pointer. Any floor values remain nonexistent (not
`TBD`-numbered) until the separate observed-baseline promotion row is
prepared and ruled. No tool is installed, no config is edited, and no
telemetry artifact is emitted under this packet.
