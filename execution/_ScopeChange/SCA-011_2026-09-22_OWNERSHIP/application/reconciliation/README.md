# SCA-011 applied ownership reconciliation

This is the targeted derivative required by the accepted Group-2 propagation plan. It binds current applied ownership to immutable discovery/R6 keys. It does not repeat the whole-corpus audit or establish implementation completion. The selected method is `bundled:chirality-root/bounded-reconciliation` (`workflows/bounded-reconciliation/WORKFLOW.md`); its document application is performed by the separate carrier TASK. This TASK owns only the derivative and this return. No reusable workflow was changed.

## Denominator and interpretation

- **598 original capability keys:** 120 have a specifically identified facet bound to an applied claim; all 478 others are explicitly `OUTSIDE_TARGETED_DERIVATIVE`. That label means not assessed by this slice, not unowned. The full key lists and preserved original classification, ownership and R5 disposition are in `CAPABILITY_FORWARD_CROSSWALK.csv`.
- **9,889 original claim keys:** 121 are relevant original ownership links or specifically affected ownership/envelope subjects. Each is preserved in `CLAIM_FORWARD_CROSSWALK.csv`, with original disposition, route, packet, outcome and residual reason. The other 9,768 are outside this derivative and retain their original R6 rows. New claim hooks are forward links, not replacements for original keys.
- **13 GUI panel families:** all appear individually in `GUI_PANEL_FORWARD_BINDINGS.csv`, with their source owners, contract artifacts, current source-file hashes, preserved semantics and activation holds. GUI `CLAIMED_BY`, producer `COVERS` and upstream `CONSTRAINS` are different relations; a GUI claim does not absorb an exporter or schema.
- `BOUNDARY_RELATIONS.csv` explicitly preserves vocabulary coverage versus editor/palette implementation, validation versus explicit acceptance/application, and the six result-interface producer/consumer homes. DEL-07-09 remains an OPEN coverage/organization contract with a context hook; this derivative invents no SoW for it.

The selection is repeatable in `build_crosswalk.py`: the exact H1 groups cited by P-OWN-01/02/03, explicit editor/palette/review/status subjects, the thirteen-panel inventory, the bounded internal transformation contract subjects, and the three source/consumer-checked subjects in RESIDUAL_SCOPE_CHECK.md. It does **not** reinterpret every capability in an area. Claim selection follows exact original `OwnerKeys` on those capabilities plus explicit affected legacy ownership/PDU envelope subjects. The code carries these finite mappings so review can challenge the mapping itself, not only its row counts.

A compound capability can span responsibilities. `AssignedFacet` limits the new link to product composition, shell composition, controlled application or GUI interaction; primitive mathematics, producer schemas, validation, user acceptance and persistence retain their owners. CAP-PHYS-022 stays DEL-04-01; CAP-PHYS-007's product adapter is DEL-04-07 against DEL-03-08's section/mass authority. CAP-WSUI-011/012 are common shell/session responsibility, while CAP-WSUI-027/028/029 stay with operation-review interaction in DEL-07-08. Acceptance/audit in DEL-16-03 is distinct from actual application in DEL-16-06.

A source/consumer follow-up linked CAP-PHYS-021 adapter/composition, CAP-SHELL-008 session bootstrap and CAP-FEATC-023 shared preparation to already accepted responsibilities. `RESIDUAL_SCOPE_CHECK.md` explains the source-grounded reasoning; `RESIDUAL_FACET_BINDINGS.csv` distinguishes their facets and consumers. `UNRESOLVED_BOUNDARIES.csv` preserves their prior dispositions and names remaining verification/engineering work. These are no longer generic delivery-home questions; no product residual is closed.

## Evidence and residuals

Original discovery source/test locators and notes are copied without promoting them to current results. The targeted code files (or files under the named hanger-fixture directory) are present and hashed. This is **source-locator evidence only**; function behavior, current defects, schema conformance, native behavior and test outcomes were not re-executed. Every targeted capability retains `UNKNOWN_CURRENT_CONFORMANCE`. No implementation remainder is closed, no original residual is recoded, and no claim is made that all 598 capabilities now have a single owner.

The three wire wrappers remain draft preparation. ValidationResult and TransformResult now have contract owners, but exact schema acceptance, application binding, wider scalar/runtime trace production and the DEL-13-04 internal-adapter exclusions remain. Comparison outputs have an interface owner; PDU-011 schema/conformance, operator acceptance, PDU-047 engineering suitability and protected tolerances remain independent. A runtime `actor_type: user` does not prove human acceptance. Rejected-history retention and actual outcome/persistence evidence remain outstanding. Licensed/external activation, privacy, source rights, professional acceptance, native qualification and lifecycle/release authority are unchanged.

The derivative is current for the hashed applied Group-2 carriers; final audited-poststate acceptance and active SCA/DAG pointer changes remain Group 3. It does not activate pending contracts or dependency stages.

## Replay and failure checks

From the repository root:

```text
python3 projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/reconciliation/build_crosswalk.py --check
```

`--check` is read-only: it reconstructs the mappings, checks actual SoW bytes against accepted application hashes, resolves every forward hook, compares complete derivative bytes and all recorded input hashes, and rejects missing, duplicate or invalid capability/claim keys. Six negative probes exercise those failures; `VALIDATION.json` contains the actual result. Running without `--check` deliberately regenerates derivative evidence and source bindings for review; it cannot constitute independent review or amend authority. Input hashes and execution attribution are in `../SOURCES_RECONCILIATION_APPLICATION.json`.

No product tests are claimed for this records-only slice. Independent applied-poststate review remains the parent manager's integration responsibility.
