# Human Decision Register — Preparation Tracking

**Created:** 2026-06-10. Non-governing tracking surface. This register tracks
decision-packet preparation and ruling pointers for the human-gated decisions
in `plans/PLAN_2026-06-10_prd_completion.md` §2. It confers no authority:
agents prepare packets labeled `PROPOSAL`; only the human project authority
rules. Record accepted rulings per existing decision practice (`DEC`/`SCA`
entries in `execution/_Decomposition/SOFTWARE_DECOMP.md` or a successor
register), then update the row here with a pointer. Newly discovered
human-gated `TBD`s get new `D-XX` rows appended here.

**Packet location:** `execution/_Coordination/_DECISIONS/D-XX_<slug>.md`

**Row states:** `NOT_PREPARED` → `AWAITING_RULING` (packet drafted) → `RULED`
(pointer to the human record).

| ID | Decision | Blocks | State | Packet | Ruling record |
|---|---|---|---|---|---|
| D-01 | Unit catalog acceptance: canonical unit set, conversion constants, offset-temperature and gauge/absolute-pressure semantics, tolerance policy | Phase B entirely; FR-002; full Phase A authoring value | RULED | [D-01_unit_catalog_acceptance.md](D-01_unit_catalog_acceptance.md) (2026-06-10) | `DEC-018` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (accepted as proposed, 2026-06-10) |
| D-02 | Rule-pack expression grammar freeze (operator/function set, conformance suite) | Phase C; FR-011 final form | AWAITING_RULING | [D-02_rule_pack_expression_grammar.md](D-02_rule_pack_expression_grammar.md) (2026-06-11) | — |
| D-03 | Sparse solver / model-scale strategy (sparse library vs bounded dense limit with diagnostics) | Phase D scale targets | AWAITING_RULING | [D-03_sparse_solver_model_scale.md](D-03_sparse_solver_model_scale.md) (2026-06-11) | — |
| D-04 | Numerical tolerance + coverage thresholds (RGAP-004) | R1/R4/R5 evidence claims; release-quality gates | AWAITING_RULING | [D-04_tolerance_coverage_thresholds.md](D-04_tolerance_coverage_thresholds.md) (2026-06-11) | — |
| D-05 | CI provider + hosted workflow location (RGAP-003) | Phase E; continuous evidence; merge-gate posture for parallel agent development branches | AWAITING_RULING | [D-05_ci_provider_workflow.md](D-05_ci_provider_workflow.md) (2026-06-11) | — |
| D-06 | Release matrix, installer formats, signing/notarization, publication targets (RGAP-003/006) | Phase E packaging | NOT_PREPARED | — | — |
| D-07 | Maintainer quorum + release authority; contributor legal mechanism | Any release claim; R5 IP contribution process | NOT_PREPARED | — | — |
| D-08 | Model-document schema migration policy (extending the store `user_version` ledger) | Phase A persistence (A2) | RULED | [D-08_model_document_schema_migration.md](D-08_model_document_schema_migration.md) (2026-06-10) | `DEC-019` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (accepted as proposed, 2026-06-10) |
| D-09 | Native package physical container format (single-file project container, public transport form) | Phase E distribution; FR-001 file-level semantics | NOT_PREPARED | — | — |
| D-10 | Report rendering target (deterministic HTML and/or PDF pipeline) | Phase A report rendering (A7); R5 full report package | RULED | [D-10_report_rendering_target.md](D-10_report_rendering_target.md) (2026-06-10) | `DEC-021` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (Option B selected, 2026-06-11; D-10b PDF follow-up deferred to the R5 lead-up) |
| D-11 | Issuance waves for the `CHECKING` deliverables | Phase F governance closure | NOT_PREPARED | — | — |
| D-12 | Disposition of FR-024 (dynamics) and FR-025 (local FEA export): implement post-beta or record explicit deferral | Final PRD-completeness claim | NOT_PREPARED | — | — |
| D-13 | Operation-seam engine unification: wasm32 build of `operation_applier` as the sole browser-mode engine; retire the TS mirror's validation/apply logic; stand up the DEL-00-01 ADR surface with this as its first accepted entry | Operation-seam plan T3/T4; the R2 from-scratch authoring set; Phase B unit-aware fields (B2) | RULED | [PLAN_2026-06-11_operation_seam_unification.md](../../../plans/PLAN_2026-06-11_operation_seam_unification.md) §2 (plan-as-packet, 2026-06-11) | `DEC-020` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (accepted as proposed, 2026-06-11) |
| D-10b | Hash-bound deterministic PDF emitter for the full report package (deferred from the D-10 Option B ruling, `DEC-021`) | R5 full report package gate | NOT_PREPARED | — | — |

**Recommended preparation order** (from the completion plan §2): `D-01` and
`D-08` first (longest poles, both touch Phase A/B foundations), `D-10` with
Phase A report work, `D-02` in the Phase C lead-up, `D-03`/`D-04` before
Phase D evidence, `D-05`/`D-06`/`D-07`/`D-09` mid-plan, `D-11` human-paced
throughout, `D-12` at the R5 gate.
