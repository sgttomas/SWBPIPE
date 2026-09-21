**Scoped PASS — P2 closed.** No new actionable defects found in the five-path repair.

`ModelTree` now uses separate per-cell eligibility and whole-projection sort values. Exact unchanged cells remain editable during unrelated reconversion. Changed value/type/unit/dimension, identity, generation, policy, availability, and removed rows immediately lose eligibility. Commit-time pruning prevents old proof revival; render does not mutate a cache. Owned negative/malformed/rejected responses revoke proof, stale completions cannot publish, and current-plan identity prevents stale physical sorting after an ABA basis change.

The evidence supports the repair:

- Before repair, actual Elastic Apply+Tab succeeded, but keyboard `8` left Shear focused as a read-only button with no editor. The separate ambiguous-status-locator failure remains clearly distinguished.
- After repair, both browser profiles recorded focused input `8`, then `87` with caret position 2, without focus/fill rescue. Pending sort retained input order; Cancel followed by Undo demonstrated exactly one Elastic checkpoint.
- The unchanged-open-Shear regression now reaches the apply route during Elastic reconversion. Changed-basis regressions remain blocked.
- Retained results: **53 tests in three files**, reported TypeScript pass, and **four browser cases**—the P2 scenario and connected Materials journey in both profiles—with zero skips/retries. Existing assertions were preserved. The corrected fail-before browser test has the same hash as the final passing test.

Exact identities:

- Reviewed repair: `6cd07a792a3e1efdd204f75b80f26f0a65e2de2e..75fa863ea5cfee9bfe8f5a6ebf0f31047663e4e5`.
- Canonical evidence: `2bbb8ce4f4d9a8230b710d15a278f2176f86d99b`.
- Identical repaired/evidence desktop tree: `d389887bcbd67f286d65c8d0534161a5aa161df9`.
- Desktop binary diff SHA256: `f8e64e4f0476c49fd197b20d9b4662d70a07ab8cfdf3ecc5499d18063cd09c93`.
- Launch SHA256: `de8db6399506d2fd40dbe9c5b4bdbc9f074dba9112579e0f46ef4a2a2646f2a2`; all 18 bound context/evidence hashes matched.
- All five maintained hashes matched `SOURCE_FREEZE_1.json`, SHA256 `3a8fc82e9bb64a89af77b24a2f949f6193c18cba9a7fd9b02b1889e66d764973`.
- Manifest SHA256 `fab2eee717a0dc778f616125fe3a33edfeaa6287d5d4d51aea556533d9e76b7a`: **42 payloads, 2,262,845 bytes, two traces**, zero mismatches. Both trace failures and raw before/after JSON attachments were inspected.

Complete maintained inventory, relative to `projects/chirality-piping/apps/desktop/`:

```text
e2e/b4-table-editing.spec.ts
src/features/model-tree/MaterialTable.test.tsx
src/features/model-tree/ModelTree.tsx
src/features/workspace/table/quantitySortProjection.test.tsx
src/features/workspace/table/quantitySortProjection.ts
```

Actual reviewer: retained independent TASK `/root/b4_3_code_review`, Codex harness-native, gpt-6-astra/xhigh, parent `/root`. Read-only source/diff/hash/log/JSON/trace inspection only; no tests, builds, UI, network, writes, Git mutations, or descendants.

This closes the source finding. Earlier evidence remains scoped carryforward. Native verification, clean combined sweep, combined-candidate review, actual-head CI, acceptance, and release remain separate gates.
