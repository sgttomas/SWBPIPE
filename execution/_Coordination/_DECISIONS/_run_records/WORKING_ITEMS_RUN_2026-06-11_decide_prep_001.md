# WORKING_ITEMS Run Record — Decision Recording + Packet Preparation (TP-DECIDE-PREP-001)

- Date: 2026-06-11
- Agent: WORKING_ITEMS (Type 1 persona) dispatching four parallel TASK
  (Type 2) decision-preparation workers with disjoint single-file write
  scopes. Human project authority directed this session toward ruling the
  pending human-gated decisions so parallel agent development branches can
  open.
- Authority: Application Integration And Issuance Loop step 4
  (decision escalation / decision-preparation tranches),
  `execution/_Coordination/_COORDINATION.md`.

## Part 1 — D-10 ruling recorded (human authority)

The human project authority ruled D-10 on 2026-06-11, selecting **Option
B** of `D-10_report_rendering_target.md` (deterministic hash-bound
single-file HTML canonical via a Rust renderer crate, plus webview
print-to-PDF explicitly labeled a derived non-hash-bound view; D-10b
PDF-emitter follow-up deferred to the R5 lead-up). Recorded as `DEC-021`
in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-10 →
RULED with pointer; packet status header updated; completion-plan §2 row
updated (**A7 unblocked**); new register row `D-10b` added NOT_PREPARED.

## Part 2 — Four packets prepared (agents; PROPOSAL only)

Workers were sealed to exactly one new file each; fan-in confirmed no
other writes and no register edits by workers. All four packets mirror
the D-10 exemplar structure, are pinned at HEAD `5079a8fa7`, carry
FACT/ASSUMPTION/PROPOSAL/TBD labels, contain zero NUL bytes, and decide
nothing.

| Packet | Lines | Recommendation (labeled PROPOSAL) |
|---|---|---|
| `D-02_rule_pack_expression_grammar.md` | 149 | Freeze the existing typed AST extended to the PRD §12.3 function set; text syntax deferred to a named D-02b at the C2 lead-up |
| `D-03_sparse_solver_model_scale.md` | 148 | Staged: bounded dense + measured limit + named diagnostic now; named D-03b sparse re-decision at Phase D lead-up with determinism/license bars fixed now |
| `D-04_tolerance_coverage_thresholds.md` | 167 | Class-tiered governed tolerances wired into the existing all-`None` `tolerance_policy` slots; blocking inventory gates + numeric coverage recorded-not-blocking (tooling to a suggested D-04b) |
| `D-05_ci_provider_workflow.md` | 151 | Formalize the five-surface local sweep as the commit-bound merge gate now (F-4 atomic wasm-build fix as rider); named D-05b public-export CI activation at Phase E; private-monorepo hosted CI only under an explicit BUILD_AND_RELEASE §7 authorization |

Register rows D-02/D-03/D-04/D-05 → AWAITING_RULING with packet pointers;
completion-plan §2 rows updated to match.

## Notable evidence surfaced by the workers (see packets for citations)

- The expression evaluator and the sparse-solver TBD are both already
  instrumented in code (`grammar_status: grammar_not_selected` in the
  rule-pack schema; `SolverDiagnosticCode::SparseSolverTbd` emitting at
  runtime), so rulings land in prepared seams rather than new surface.
- The benchmark crates carry structural `tolerance_policy` slots
  (all-`None`) with a live `tolerance_policy_tbd_diagnostic` — D-04's
  ruling fills slots that already refuse silent tolerance claims.
- Repo-topology FACT shaping D-05: `projects/chirality-piping` is not its
  own git repository; it rides the private monorepo remote, and the
  monorepo `.gitignore` does not exclude `projects/`. Two GitHub Actions
  workflows exist at the monorepo root but target only `frontend/`.
- Brief-pointer correction surfaced by two workers: the canonical
  RGAP-003/RGAP-004 wording lives in
  `execution/_Aggregation/TP-RELEASE-GAP-REGISTER-REFRESH-001_2026-05-31/Gap_Disposition_Register.csv`,
  not in `docs/RELEASE_QUALITY_GATES.md` (which carries the matching §10
  TBD list).

## Boundary review

Packets are PROPOSAL-only and confer no authority; invented/synthetic
example values only; no protected standards content; no network use; no
release-readiness, professional approval, certification, sealing,
authentication, or code-compliance claims. Rulings remain exclusively
human records per K-AUTH-1.

## Part 3 — Four rulings recorded (human authority, same day)

After reviewing the packets, the human project authority ruled all four on
2026-06-11:

| Decision | Ruling | DEC | Follow-up row created |
|---|---|---|---|
| D-02 | Option A — frozen typed AST extended to the PRD §12.3 function set | `DEC-022` | D-02b (text syntax, C2 lead-up) |
| D-03 | Option C — hand-rolled in-repo sparse skyline/profile direct solver | `DEC-023` | — |
| D-04 | T-B + C-C — governed global relative tolerance (measured 1.0e-9) with reasoned per-case overrides; blocking inventory gates + recorded-not-blocking numeric coverage | `DEC-024` | D-04b (coverage tooling / floor promotion) |
| D-05 | Option D — five-surface local sweep codified as the deterministic commit-bound merge gate for parallel agent branches; F-4 atomic-build rider | `DEC-025` | D-05b (public-export CI, with D-06) |

Two rulings diverged from the packet recommendations, which is the human's
prerogative and is recorded without reconciliation: D-03 selected Option C
over the staged Option D (accepting in-repo numerical implementation cost
for zero dependencies and full determinism control), and D-04 Part 1
selected T-B over T-C (the packet's recorded T-B trade-offs — override
scatter, silent-loosening risk, no absolute floor near zero, and the D-01e
analogue — stand as noted residuals for gate-record discipline; the
surviving exact-equality surfaces, e.g. the contract-corpus comparisons,
are outside D-04 scope and unchanged).

Recording surfaces updated: `SOFTWARE_DECOMP.md` §12 DEC-022..DEC-025;
register rows D-02..D-05 → RULED with pointers; packet status headers;
completion-plan §2 rows. No lifecycle state changed; no release,
professional, certification, or code-compliance claim is created by any of
this.
