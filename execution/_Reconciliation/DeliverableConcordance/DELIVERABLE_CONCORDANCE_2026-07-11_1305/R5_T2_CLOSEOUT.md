# R5 T2 Closeout — Canonical Schema, Units, and Mechanics

Date: 2026-07-12
Decision basis: D-41 R4 ruling / DEC-074, engineering evidence requirements E1, E2, E4, and E6
Proposed-update scope: PDU-002, PDU-003, PDU-011, PDU-013, PDU-014, PDU-015, PDU-023, PDU-024, PDU-025, PDU-029, PDU-030, PDU-031, PDU-032, PDU-033, PDU-034, PDU-035, PDU-044, PDU-047, and PDU-048
Lifecycle effect: none

## Outcome

T2 is complete as a governed repair-and-disposition tranche. PDU-002 and PDU-003 were closed in T2A. The remaining seventeen PDU groups were either repaired to the evidence level authorized by DEC-074 or given an explicit, smallest-action hold where the ruling did not select a policy value, review outcome, or independent validation witness.

| PDU | T2 disposition |
|---|---|
| PDU-002, PDU-003 | Complete in `R5_T2A_CLOSEOUT.md`: current deterministic JSON byte contracts are precisely labeled and do not claim RFC 8785/JCS. |
| PDU-011 | Bounded repair: the schema-derived dimension vocabulary is the single validation-preview vocabulary; DEL-00-07 records concrete adapter obligations. Analysis-run normalization and tolerance validation remain held because no accepted comparison-output authority exists. |
| PDU-013 | Bounded witness: the rights-safe pipe-section oracle is bound through the production calculation path with governed units, dimensions, and existing tolerances. COG convention/reference frame and broader three-dimensional mechanics validation remain held. |
| PDU-014 | Bounded repair: the named-mirror and unit-system architecture are coherent. Remaining B2/B3 wiring is retained as work, not asserted complete. |
| PDU-015 | Held: exact alias/parser acceptance policy was not selected by the ruling. |
| PDU-023 | Bounded repair: model/result trace schemas accept paired scalar field paths and the physical-to-analytical transform emits deterministic paths for valid geometry quantities. A runtime result-envelope producer and application-service home remain held. |
| PDU-024 | Complete within scope: desktop version classification uses the accepted 0.2.0 family and preserves distinct current, stale/migration-needed, failed, newer, and unsupported diagnostics. |
| PDU-025 | Held: exact unit-diagnostic taxonomy was not selected by the ruling. |
| PDU-029 | Complete within scope: written JSON glTF and sidecar stable IDs are checked one-to-one and corruption is blocking. |
| PDU-030 | Complete within scope: exact result-ID mappings are automatic only for unique compatible identity; all other mappings remain manual and use the governed mapping record. |
| PDU-031 | Held: observed timestamp-free/fixed-generator behavior is tested, but exact timestamp/generator policy was not selected. |
| PDU-032 | Complete within scope: model-state comparison tests prevent bare numeric deltas for missing, incompatible, and same-unit cases. |
| PDU-033 | Held at the actual boundary: result records expose unit strings but no authoritative explicit dimensions, so dimensions are not inferred. DEL-12-04 security-classification rows are carried to T3 under O7/E5 rather than misclassified as unit work. |
| PDU-034 | Held: exact protected-content quarantine/readiness and export-adapter readiness taxonomies, including destructive-workflow handling, were not selected. T3 O7/E5 is the next permitted security lane. |
| PDU-035 | Held: formal review disposition and dimensional/conversion validation remain review/evidence gated. |
| PDU-044 | Documented absence: DEL-06-04 has no numeric schema surface; numeric ownership remains with DEL-02-02 and DEL-06-02/DEL-06-03. |
| PDU-047 | Bounded witness: production pipe-section calculations are covered by the rights-safe oracle. Normalization/tolerance and broader mechanics validation claims remain held. |
| PDU-048 | Held: no independent numeric witness exists, so the surface remains verified-not-validated. |

## Evidence

The integrated cache-disabled project Python suite passed 481/481. Desktop Vitest passed 472/472. The copy-out production build passed after rebuilding the intentionally uncommitted WASM engine asset; only the pre-existing chunk-size warning remained. Focused owning-pilot suites additionally covered schema scalar paths, trace round trips, result-ID mapping, stable-ID corruption, production pipe-section witnesses, model-state unit boundaries, and version diagnostics. The practitioner-harness self-check exited 0 with its pre-existing INFO/WARN/REVIEW findings and no D-41 BLOCK; cache-disabled practitioner-harness pytest passed 263 with one skip.

Independent highest-available-capability GPT-5 fan-in first found two issues: PDU-024 collapsed distinct stale/failed conditions, and PDU-034/PDU-035/PDU-044 lacked durable hold records. Corrections were routed through the owning PDU-024 pilot and owning deliverable surfaces. Final independent read-only fan-in returned PASS. A subsequent production build found one TypeScript union-widening defect; the PDU-024 owning pilot corrected it, reran 65 focused tests, and passed a disposable production build.

The addendum-9 containment contract remains intact. T2-created bytecode and Cargo targets were removed after validation. The active worktree retains only its three pre-existing ignored paths, and the frozen evidence worktree retains exactly the six allow-listed ignored paths. No in-tree `py_compile` was used and lockless Cargo execution remained copy-out/external-target based.

## Preserved boundaries and next gate

- Every touched deliverable lifecycle state remains `IN_PROGRESS`.
- Exact D-41 bootstrap Remaining items remain for T7 corpus-currentness work.
- No policy value, review disposition, numeric validation, security validation, target readiness, compatibility, release, code-compliance, or professional-reliance conclusion is inferred from E1–E8.
- No dependency, DAG, register, decomposition, review-finding, or ISSUED-baseline change occurred.
- D-42 remains `AWAITING_RULING`; its two O3 authority-conflict surfaces remain untouched.
- T3 may proceed under the O7-selected private-by-default seam and E5 evidence requirement. PDU-033's security rows and PDU-034's security taxonomy are carried there only within the holds stated above.
