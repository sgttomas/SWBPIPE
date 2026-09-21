Clarification against clean `c0a3314e2b141d8f93b90364250d2bffa89433f7` / product `ed8`; no writes or execution checks performed.

**Node provenance:** I found no supported editable route for structured Node provenance. `PreviewModel.nodes` declares a string; Property Inspector exposes the same scalar `set_field` route. Engine `FieldKind::Text` requires the existing value to be a string, checks the exact before-value, trims the replacement, rejects empty text and explicitly permits `TBD`. Geometry operations likewise reject object-valued provenance. The existing structured-provenance grid guard applies specifically to **Sections**.

Therefore B4.2 should edit string Node provenance and deliberately display any object-valued Node provenance read-only, preserving its contents without flattening. This is defensive handling of an unsupported edit shape, not a claim that another Node editor supplies the missing capability.

**Review semantics:** preserve raw text drafts, including blank/whitespace, while staging. Preserve current Queue behavior: dimensionless text becomes `value.trim() || "TBD"`; before-values come from the current model at Queue; only visible changed drafts are queued and cleared. Do not introduce direct-mode blank rejection into review staging or silently change that legacy conversion. Direct model Apply should instead reject blank text and require an explicitly entered `TBD`. Explain the review conversion visibly.

Use **“Keep draft”** for the cell-level review footer action, with feedback “Draft retained; model unchanged.” Keep the separate **“Queue changed cells”** action. Neither staging action should emit an applied-model outcome.

Decision-bearing source hashes actually consulted follow. Paths are relative to `projects/chirality-piping/`; `src/` below means `apps/desktop/src/`.

| Source | SHA-256 |
|---|---|
| `core/model_operations/operation_applier/src/lib.rs` | `22062ea42fd135a9dbefa0f7e09cf3a2feccc5193ff6028818fd1c6221a3f355` |
| `core/model_operations/operation_applier/src/geometry_operations.rs` | `cf787c0ae6d00c4179018f6ae47379ac34352c9de632a32e41543921b8da2bdf` |
| `src/types.ts` | `a2ccaf863646c48c3f1c3303ad31ae87132311c28f72b343824f47ac58a83314` |
| `src/features/model-tree/PropertyInspector.tsx` | `3120f29cb431fe649189a41048b95923487ff6140878dcb22bbad6b23a2d3f89` |
| `src/features/model-tree/ModelTree.tsx` | `5fb7efb59ccd97eeb547d21ac59f6de80fef8157136b011a9b5cbb49cf1cf919` |
| `src/features/workspace/table/modelTableAdapter.ts` | `cf0473c0cc4c201e63df4f754e38749b43860a5643c15b08eb26db81e9d04ce9` |
| `src/features/workspace/table/tableState.ts` | `ea98c7a719082509ac0732cf89b84bfea33fb7063fc2080ee27913bd240de87a` |
| `src/features/workspace/table/EngineeringTable.tsx` | `bedfc82601621155d030af0e8d21c302052c27aa03fd47821dbf3731c2b87530` |

These support the proposed bounded Node migration; they do not establish full B4 or structured-provenance editing completion.
