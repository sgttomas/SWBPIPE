# NS Coverage

## Assessed surface

The review assessed the source at commit `533332349a4607eee561d4ef90fb05a62d86519e` under the authority amendment supplied with the brief.

| Surface | Assessment focus | Evidence used |
|---|---|---|
| `core/solver/nonlinear_supports/src/lib.rs` | one-way/gap/lift-off and Coulomb classification, validation, exact-contact behavior | full production implementation, focused tests, current file SHA `f6c6e629...` |
| `core/solver/nonlinear_integration/src/lib.rs` | active-set loop, convergence, sliding direction, current-normal coupling, constraints, dense/sparse routing | full production implementation and relevant focused tests, current file SHA `6e163a47...` |
| `core/solver/sparse_direct/src/lib.rs` | pivot policy, nonpositive-pivot evidence, solve semantics | factorization and solve paths, current file SHA `b480d297...` |
| `core/solver/frame_kernel/src/lib.rs` | dense pivot policy and fallback equivalence | dense solve paths, current file SHA `d657e650...` |
| `core/product_physics/src/lib.rs` | initial-state contract, spring transfer, result metadata and diagnostic propagation | relevant nonlinear adapter and output paths, current file SHA `f1ae3322...` |
| `core/solver/diagnostics/src/lib.rs` | factorization-report diagnostic conversion | helper definition and repository call-site search |
| Nonlinear benchmark and READMEs | declared support scope and benchmark coverage | source and documentation trace; benchmark SHA `ff3f318...` |
| Prior focused audits | executable evidence for preserved current branches and historical issue closure | M1-N, M1-L, I1 returns and raw observations |
| PR 760/source history | intent and scope of current-normal and related repairs | local git log/diff and project decision records |
| Evaluation-local NS-01 runner | current-SHA public nonlinear-integration execution for both initial friction states | exact command and stdout in `RUNTIME_PROBE.md` |

The review also checked the current source for closure of previously reported exact-contact chatter, zero-iteration acceptance, nonfinite trials, repeated support DOFs, spring transfer, and same-iterate current-normal coupling. These checks did not identify a new remaining defect beyond the four findings returned.

## Deterministic probes executed

Three small exact Python replays were executed without building the repository:

1. Coulomb seed divergence for `k=100`, `F=1`, `mu*N=3`, including the positive final `f*u` product.
2. Pivot scaling for `k=f=2^-40`, showing exact `x=1`, condition number one, and rejection by the absolute `1e-12` policy.
3. Contact stabilization for the determinant-zero inactive two-node system and the unique admissible active solution `u1=0.1`, `R0=-10`.

These probes are transparent algebraic oracles, not substitutes for focused Rust regression tests.

One separately authorized current-SHA Rust probe then compiled the evaluation-local runner and its current path dependencies in an isolated Cargo target. It reproduced NS-01 on its first and only `cargo run` attempt. The 61 MB target directory was removed immediately after capture; the small runner and evidence remain under `returns/NS/`.

## External reference check

Two primary technical references were consulted on 2026-09-09:

- [Abaqus friction theory](https://docs.software.vt.edu/abaqusv2025/English/SIMACAETHERefMap/simathe-c-coulombfric.htm), for a credible stick/slip transition comparison.
- [LAPACK DPOSVX](https://www.netlib.org/lapack/explore-html/d6/d44/group__posvx_gae23dac18e7ec69c36fd2a2648a470dc3.html), for a credible SPD equilibration, factorization, condition, refinement, and backward-error pattern.

They inform repair selection only. No implementation comparison or industry-solver validation was performed.

## Explicit exclusions

- No product, governed, or Git state was modified.
- No repository-wide build, blanket test, benchmark suite, or shared heavy command was run. The only compiled execution was the isolated NS-01 runner documented in `RUNTIME_PROBE.md`.
- No piping-design extracted equation or deliverable assertion was treated as authority.
- No material nonlinearity, geometric nonlinearity, transient dynamics, friction history, or general contact subset enumeration was assessed as an implemented capability.
- No load-combination, code-compliance, or domain-engine acceptance claim was made.
- No external solver result was generated or compared.

## Residual unknowns

| Unknown | Consequence | Required closure evidence |
|---|---|---|
| Current-SHA product execution of the NS-01 scalar fixture | Source and exact replay are decisive, but the newest product wrapper was not executed here | focused current-SHA Rust regression in direct integration and product adapter, both solve modes |
| General multi-contact feasible-set discovery | NS-02 repair is intentionally bounded to an admissible all-active recovery trial | a separately authorized algorithm/complexity decision and adversarial contact corpus |
| Practical conditioning warning thresholds | Arithmetic correctness can be repaired independently, but engineering warning thresholds depend on units/model scale | project-approved units policy plus scaled model corpus |
| Realistic indefinite-state reachability through validated product inputs | NS-04 contract gap is confirmed; reachability is narrower than the raw solver API | targeted product construction or explicit proof that the state is unreachable, plus solve-boundary invariant enforcement |
| External solver agreement | No such validation was run | separately scoped comparison matrix with declared versions, cases, norms, and tolerances |

## Closure statement

The assigned review surface is covered for source-visible remaining mechanics defects and decision-ready repair recommendations. Closure of the software defects requires the prioritized implementation and verification fence in `RETURN.md`; this return does not mark those future changes as complete.
