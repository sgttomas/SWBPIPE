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
| D-01 | Unit catalog acceptance: canonical unit set, conversion constants, offset-temperature and gauge/absolute-pressure semantics, tolerance policy | Phase B entirely; FR-002; full Phase A authoring value | AWAITING_RULING | [D-01_unit_catalog_acceptance.md](D-01_unit_catalog_acceptance.md) (2026-06-10) | — |
| D-02 | Rule-pack expression grammar freeze (operator/function set, conformance suite) | Phase C; FR-011 final form | NOT_PREPARED | — | — |
| D-03 | Sparse solver / model-scale strategy (sparse library vs bounded dense limit with diagnostics) | Phase D scale targets | NOT_PREPARED | — | — |
| D-04 | Numerical tolerance + coverage thresholds (RGAP-004) | R1/R4/R5 evidence claims; release-quality gates | NOT_PREPARED | — | — |
| D-05 | CI provider + hosted workflow location (RGAP-003) | Phase E; continuous evidence | NOT_PREPARED | — | — |
| D-06 | Release matrix, installer formats, signing/notarization, publication targets (RGAP-003/006) | Phase E packaging | NOT_PREPARED | — | — |
| D-07 | Maintainer quorum + release authority; contributor legal mechanism | Any release claim; R5 IP contribution process | NOT_PREPARED | — | — |
| D-08 | Model-document schema migration policy (extending the store `user_version` ledger) | Phase A persistence (A2) | AWAITING_RULING | [D-08_model_document_schema_migration.md](D-08_model_document_schema_migration.md) (2026-06-10) | — |
| D-09 | Native package physical container format (single-file project container, public transport form) | Phase E distribution; FR-001 file-level semantics | NOT_PREPARED | — | — |
| D-10 | Report rendering target (deterministic HTML and/or PDF pipeline) | Phase A report rendering (A7); R5 full report package | NOT_PREPARED | — | — |
| D-11 | Issuance waves for the `CHECKING` deliverables | Phase F governance closure | NOT_PREPARED | — | — |
| D-12 | Disposition of FR-024 (dynamics) and FR-025 (local FEA export): implement post-beta or record explicit deferral | Final PRD-completeness claim | NOT_PREPARED | — | — |

**Recommended preparation order** (from the completion plan §2): `D-01` and
`D-08` first (longest poles, both touch Phase A/B foundations), `D-10` with
Phase A report work, `D-02` in the Phase C lead-up, `D-03`/`D-04` before
Phase D evidence, `D-05`/`D-06`/`D-07`/`D-09` mid-plan, `D-11` human-paced
throughout, `D-12` at the R5 gate.
