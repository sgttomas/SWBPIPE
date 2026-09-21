**One P2 finding; repair and backcheck before source fan-in.**

1. **P2 — Whole-batch projection refresh blocks unchanged direct cells.** At [ModelTree.tsx:1041](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/model-tree/ModelTree.tsx:1041), direct eligibility depends on presence in the sort projection. [quantitySortProjection.ts:48](/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-wt3/projects/chirality-piping/apps/desktop/src/features/workspace/table/quantitySortProjection.ts:48) empties that entire projection whenever **any** Material quantity changes.

   **Trigger/effect:** Apply an Elastic edit with Tab, then type into the unchanged Shear cell while reconversion remains pending. Shear becomes read-only despite its unchanged, previously verified basis. `EngineeringTable.tsx:140` refuses to start editing, while lines 241–242 consume the character/Enter event, silently dropping input. Likewise, an existing editor whose own before/unit/generation remain current receives “This cell is now read-only” at line 169 if another quantity changes during its edit. This affects ordinary consecutive entry and retained-editor recovery; it is separate from the disclosed restriction on genuinely converter-unavailable quantities.

   **Remedy:** Separate whole-projection sort readiness from per-cell direct eligibility. Retain conversion eligibility for an unchanged cell’s exact generation/identity/value/unit/dimension basis while other cells refresh; keep the required whole-sort pending/unavailable behavior. Add a delayed-conversion regression covering successful Apply+Tab followed by actual character input, and an unchanged open editor across another quantity’s update. This finding is established by source/caller tracing; I did not execute a reproduction.

No additional actionable source defects or weakened inherited test oracles were found. The persistent editor is bounded to Materials and preserves shared handlers while Node remains inline. The deferred-reorder alternative is candidly documented. Actual units, numeric grammar, positive E/G and signed alpha rules, raw review staging, filtered Queue, all-draft Clear, captured operation ownership, and existing result/history route were traced. The stricter unsupported-unit direct restriction is disclosed; the backend’s same-unit route does not justify claiming universal engine rejection.

Reviewed identities:

- Complete maintained diff: `c9f19195214b2e097980dde5e0d5fccbfba8e34c..6cd07a792a3e1efdd204f75b80f26f0a65e2de2e`.
- Canonical evidence head: `daf8d966cc46ebb11f92dd8415d1f427fd9b2e71`.
- Both desktop trees: `52670ab05ff303e17cf13947b0d0b1a372a73f5d`.
- Desktop `git diff --binary` SHA256: `59f3fbd725c528da4617687c8b04a4c2d30075939c8eefdb6fe978ac6aef3990`.
- Sealed launch SHA256 verified: `6881b98e7afee243acbbf1374900b054a0b0183ec9111d3221c8204083938070`.
- All launch-bound context hashes matched. All 11 maintained hashes matched `source-freeze-4.json`, SHA256 `22f6cb9e2f4f889026749f78e37b4061d10441d1e8f58acd7b96c506f1f5d3e7`.

Complete path inventory, relative to `projects/chirality-piping/apps/desktop/`:

```text
e2e/b4-table-editing.spec.ts
src/features/model-tree/MaterialTable.test.tsx
src/features/model-tree/ModelTree.tsx
src/features/workspace/table/EngineeringTable.test.tsx
src/features/workspace/table/EngineeringTable.tsx
src/features/workspace/table/modelTableAdapter.test.ts
src/features/workspace/table/modelTableAdapter.ts
src/features/workspace/table/quantitySortProjection.test.tsx
src/features/workspace/table/quantitySortProjection.ts
src/features/workspace/table/tableState.ts
src/styles.css
```

Evidence audit verified **69 payloads / 24,826,486 bytes / eight traces**, with zero manifest mismatches. Manifest SHA256: `0cf0d8c5bbd65afa8db4dbb32d31f7a94788d55d8654d31f5fad9ca827f4c578`; worker return SHA256: `5b6077889e77fd917f51140ee0878c91032d2a7296810492330a509e9c14a032`. Trace failures support the recorded Undo, initial-focus, inert-return, absent-Node-ID, and retained-disclosure diagnoses. The MPa/Pa fixture preserves opposing raw/physical order. Existing evidence supports the reported scoped passes, not a single clean final full-lane run. The unchanged App test remains one selected result-clear/re-solve/save witness, not a new table-to-Current browser journey.

Actual review: fresh TASK `/root/b4_3_code_review`, Codex harness-native, gpt-6-astra/xhigh, parent `/root` HELP_HUMAN. Actions were read-only source/diff/caller inspection, hashing, and log/ZIP inspection. No writes, tests, builds, UI, network, Git mutations, or descendants. Native verification, clean final sweep, combined-candidate review, actual-head CI, and product acceptance remain pending.
