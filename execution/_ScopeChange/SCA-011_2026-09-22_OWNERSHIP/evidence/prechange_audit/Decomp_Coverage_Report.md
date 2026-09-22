# SCA-011 independent prechange decomposition audit

**Evaluated basis:** `008ef6a4e370822c65eb26ddd792752b4267c7ab`, accepted SOFTWARE_DECOMP 0.12, active scope pointer SCA-010, accepted graph DAG-010. This evaluates accepted bytes, not the concurrent SCA-011 candidate. Method: `bundled:chirality-root/audit-decomp`; TASK execution by `/root/piping_scope_manager/piping_prechange_audit` through native Codex delegation. No canonical edits, lifecycle changes, product tests or subdelegation.

The materialized structure is intact: **80/80 deliverables in 14/14 scoped packages**, within a repository decomposition of 18 packages, 102 deliverables, 77 scope items and 18 objectives. All 80 identity/name/type/package/envelope comparisons match; markdown deliverable/ledger bindings agree with their companion registers. All 79 scoped SoW documents pass the registered format validator. DEL-07-09 is the remaining OPEN contract-only unit, with all three named contract artifacts present; its absence of a SoW or MEMORY is not evidence that its current accepted custom coverage work failed.

One inherited mapping issue and one historical-method compatibility observation matter when preparing SCA-011:

1. **OBJ-018 support summary is inconsistent.** SOFTWARE_DECOMP §5 includes DEL-12-02 and omits DEL-13-02. Its §7 and `docs/_Registers/Deliverables.csv` consistently assign OBJ-018 to DEL-13-02, while DEL-12-02 carries only OBJ-010. §9/SOW-067 supports the DEL-13-02 mapping. Regenerate the affected objective summary from accepted mappings when adding the new GUI owner. Do not silently expand DEL-12-02's scope. This is a record mismatch, not a newly discovered missing engineering obligation.
2. **SCA-010 follows its own accepted compact rename contract.** Its Brief and Acceptance Record identify Brief, Impact Assessment, Amendment Actions and Amendment Preview as the intended package, and those artifacts exist with the acceptance record. It differs from the full template selected for the new SCA-011 undertaking. That is an informational historical-method compatibility observation, not missing accepted obligations or failed closure. Check 10 passes against SCA-010’s own accepted contract. No retroactive gate, renewed acceptance or historical rewrite follows; SCA-011 must satisfy its presently selected contract.

| Check | Verdict | Evidence / boundary |
|---|---|---|
| 1 Forward packages | PASS | All 14 scoped package folders exist in accepted tracked state. |
| 2 Forward deliverables | PASS | All 80 scoped units have one materialized folder. |
| 3 Reverse folders | PASS | No scoped tracked unit lacks a declared identity. |
| 4 ID consistency | PASS | Folder, register and context identity/parent agreement. |
| 5 Context fidelity | PASS, bounded | Identity, name, type, parent and envelope match. Semantic intent was not re-audited across every description. |
| 6 Artifact presence | INCOMPLETE, bounded | 79 SoW formats PASS; DEL-07-09's three custom artifacts present. Full anticipated product-artifact realization was deliberately not re-censused. |
| 7 Objective mapping | WARNING | All 18 ledger objectives have materialized support; OBJ-018's summary names disagree with production-unit mappings. No scoped unit or IN ledger row lacks objective mapping. |
| 8 Ledger integrity | PASS for mappings; tool limits | All 77 markdown/CSV ledger mappings match and reference declared units. Local dependency mirrors have inherited issues described below. |
| 9 Derivative parity | SKIPPED | DOMAIN-only portion of the selected method. |
| 9b Package shape | WARNING | Working-surface role is labeled; explicit companion inventory is absent, while full deliverable and scope tables duplicate companion truth. Preserve parity through this amendment; wholesale package redesign is not required here. |
| 10 Active snapshot/handoff | PASS, own contract | SCA-010 pointer resolves once; accepted compact bundle is present. Current-template differences are informational, not retroactive requirements. |
| 11 Lifecycle distribution | PASS, informational | 79 IN_PROGRESS, DEL-07-09 OPEN. No state advanced. |

## Tool evidence and limitations

`register_validation.json` and `.txt` preserve the stock `validate_decomposition_registers.py` result. It scanned 94 local registers / 1,487 rows and reported 2,097 errors: 1,395 historical DAG-prefixed IDs contrary to its DEP-prefix convention, 701 unresolved former artifact paths, and one placeholder source locus. It **did not discover** the companion registers at Piping's `docs/_Registers`, and skipped XRG. It is not a passing cross-register audit. The independent mapping scan in `audit_baseline.py` explicitly uses the actual companion paths. The raw tool’s whole-project local-mirror scan is denominator context, not an expansion into product auditing or a claim that all mirror findings are amendment blockers. DAG-010 remains the accepted authority; a local-mirror diagnostic does not supersede it.

`sow_validation.json` records 79 actual validator exit-0 results and the disclosed DEL-07-09 custom-contract exception. Each validated SoW's current bytes matched the frozen source hash before validation. `_STATUS.md` and `MEMORY.md` were read together where both exist; the inventory records the absent DEL-07-09 MEMORY. `SOURCES.json` hashes actual consulted source bytes and origins. Source reads use the pinned commit so concurrent candidate edits cannot become prechange evidence. Filesystem coverage is the **tracked materialized baseline**, not an assertion about untracked candidate folders.

## Consequences for P-OWN-01 through P-OWN-06

The proposed new IDs DEL-04-07, DEL-07-11, DEL-07-12 and DEL-16-06, and SOW-078/079, are absent from accepted registers. Keep the proposal's DEL-07-10 / DEL-16-05 reservations distinct. The package must propagate scope and objective bindings through the actual companion registers, current contexts, coverage summaries and successor dependency evidence. The four new units are accountability assignments, not evidence of completed implementation.

Retain DEC-044's separate solve orchestration, DEC-020's sole operation engine, DEC-094's coverage/editor distinction and candidate-generator reservation, DEC-103's hanger schema/provenance split, DEL-07-09's OPEN lifecycle, PKG-13's staged obligations, native/practitioner/engineering evidence requirements, privacy/export boundaries, and immutable historical evidence. P-OWN-04 changes the accepted row-5 placement restriction explicitly; the old annex must remain historical, with a successor binding. P-OWN-06's schema drafts and P-OWN-05's per-panel source-owner bindings remain implementation/interface evidence work; this structural audit does not substitute for them. The existing comparison dependency direction must be evaluated by the successor graph author before adding a reverse prerequisite.

**Return:** usable prechange baseline with inherited findings and disclosed partial artifact audit. The scope manager may prepare the successor amendment; clean baseline closure, product correctness, engineering acceptance and postchange coverage are not claimed here.
