# B4.2 Queue, Clear and raw-draft clarification

ROOT engineering clarification to the sealed B4_2_MANAGER_BRIEF.md SHA9d534b90967995ddf67dd81bef9afc6ff7e7a9bac0d711afa91272d7cf0c72f0. Original bytes remain unchanged. This resolves the independent records review's narrow P3 ambiguity; it does not authorize a product behavior change.

The phrase “visible-only Queue/clear semantics” refers to the cleanup performed by Queue. Precisely:

- **Queue changed cells** submits only currently visible changed drafts and clears only those submitted draft entries. Filtered-out drafts survive.
- The separate explicit **Clear grid edits** action retains its existing `setDrafts({})` behavior, clearing all lifted grid drafts, including filtered/other-family drafts. Do not silently change it into visible-only clearing.
- Active editor ordering must be explicit. Keeping a current review edit, then snapshotting Queue or clearing it, must not omit the last typed value, resurrect a cleared draft through a late blur/completion, or apply a model change. Include real click-from-active-editor paths in focused verification rather than depending on test helpers to blur first.
- Preserve legacy raw review text/quantity staging, including temporarily invalid numeric text; validation/rejection still belongs to the existing proposal route. Direct-mode validators must not silently restrict what the review path could previously retain/queue. Do not report these local draft actions as model application.

Source basis is merged15e8b7a72: ModelTree.tsx Queue handler clears only changedCells built from visibleRows, while the explicit Clear button resets the complete draft map. Other B4.2 requirements and scope are unchanged. Exact-before/trimmed replacement semantics must follow the engine; normalized no-op detection must compare the normalized proposed value with the exact stored canonical value, not silently pretend normalization of an unusual stored value was applied.
