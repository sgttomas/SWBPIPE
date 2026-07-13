# R5 T2A Closeout — Canonicalization-Label Fidelity

Date: 2026-07-12
Decision basis: D-41 R4 ruling / DEC-074, engineering evidence requirement E1
Proposed-update scope: PDU-002 and PDU-003
Lifecycle effect: none

## Outcome

T2A is complete for the seven owning/consuming deliverables in PDU-002 and PDU-003: DEL-02-02, DEL-14-02, DEL-15-01, DEL-15-03, DEL-17-03, DEL-17-06, and DEL-17-09.

The current Python serializers are deterministic sorted-key, compact, ASCII-escaped JSON. They are not RFC 8785/JCS implementations. Producer labels, schema vocabulary, governed fixtures, tests, and active deliverable requirements now state that narrower fact:

- project persistence and analysis-run records use `SORTED_COMPACT_JSON` with canonical truth `sorted_compact_json_payload`;
- native JSON, stress-neutral, and adapter-SDK producers use `deterministic_sorted_compact_json_payload_hash` for hashes they compute;
- the shared handoff schema accepts the precise label while retaining legacy values for compatibility;
- DEL-15-03 carries producer-supplied checksum metadata without recomputing, validating, or relabeling it;
- DEL-17-03 preserves a governed caller-supplied source checksum mapping after structural validation and reserves its local label for hashes it computes; and
- stress-neutral CSV normalization remains separately labeled `normalized_ascii_lf_text`.

No serializer bytes were changed merely to satisfy a label. The persistence fixture's envelope hash was recomputed because the hash-bound metadata itself changed.

## Evidence

Focused evidence covers exact serialized bytes, a fixed SHA-256 vector, equivalent object-order stability, mutation sensitivity, schema/fixture validation, timestamp-free inputs where applicable, supplied-checksum pass-through, malformed supplied-checksum rejection, JSON/CSV label separation, and explicit absence of an unproved JCS claim.

The full cache-disabled project Python suite passed 469/469. The practitioner harness self-check exited 0 with its pre-existing INFO/WARN/REVIEW findings and no D-41 BLOCK; its cache-disabled pytest corpus passed 264/264. A copy-out Cargo sweep discovered and passed all 36 crate checks without writing lockfiles or targets into the governed worktree. The same copy-out tree passed desktop Vitest 471/471, the production build (with the existing chunk-size warning), dev-server Playwright 18/18, and production-dist Playwright 1/1.

An initial in-tree invocation of the generic evidence-sweep wrapper was stopped when it began violating the addendum-9 copy-out control. Only ignored artifacts created by that invocation were removed; the pre-existing allow-listed ignored sets were not altered. Revalidation then used the required copy-out method. The frozen evidence worktree was rechecked independently and still reports exactly the six addendum-9 allow-listed ignored paths and no seventh path.

## Independent fan-in

Independent GPT-5 fan-in initially returned FAIL with four findings:

1. DEL-17-03 incorrectly relabeled a supplied source checksum without recomputation.
2. The stress-neutral protected/private-content scan had lost list recursion.
3. Active kit clauses in DEL-02-02, DEL-15-03, and DEL-17-06 still required an unproved JCS basis.
4. DEL-14-02 retained a stale cross-owner DEL-02-02 residual after the same-diff repair existed.

Each finding was corrected through its owning implementation or deliverable surface. The supplied checksum is now preserved or rejected, list recursion is restored, active requirements and contexts are coherent with the narrow byte contract, and the satisfied cross-owner residual is closed while the D-41 bootstrap remains.

## Preserved boundaries and next gate

- All seven lifecycle states remain `IN_PROGRESS`.
- Exact D-41 bootstrap Remaining items remain for T7 corpus-currentness work.
- No scope expansion, target compatibility, release, code-compliance, solver-validation, professional-reliance, or human-review disposition is asserted.
- No dependency/DAG/register/decomposition or ISSUED-baseline change occurred.
- D-42 remains `AWAITING_RULING`; its two O3 surfaces remain untouched.
- T2 continues with the remaining PDU-011/013/014/015/023/024/025/029/030/031/032/033/034/035/044/047/048 subtranches.
