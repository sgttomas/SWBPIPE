# C3 helper handoff

Frozen helper checkpoint on base 88c6c7463199d1d69caebde04678fa74facca78f; the manager's dispatch supplies its commit ID. Shell may consume only the helper/type and tests from this internal checkpoint before its B3B freeze, then write the agreed bridge with its sole writer.

Export from `apps/desktop/src/features/viewport/viewportVisibility.ts`: `ViewportVisibilityProjection` and `deriveViewportVisibility(index, explicitHiddenKeys, isolationSelectionKeys)`. Output: readonly hiddenKeys, dimmedKeys, hiddenCount, isolationActive. Null isolation is inactive; empty set stays active. Dimming uses literal selected-key membership; Hide uses existing attachment expansion and wins. Hidden count is the intersection of expanded Hide and valid drawable keys. Inputs are not mutated. All geometry/picking and first-profile bytes stay unchanged.

Shell handoff remains exact: canonical `isolationSelectionKeys: ReadonlySet<EntityKey> | null`, the single memoized projection; actual hiddenKeys to tree+viewport, dimmedKeys/isolationActive/hiddenCount to viewport; `onIsolationSelectionChange` and existing `onClearVisibility`. Show All operative for active isolation even at zero count; prune deleted keys without turning active empty into null. Stage/view retain state, project replacement clears it.

Worker /root/i1_manager/c3_worker, TASK Astra/low, under WORKING_ITEMS Astra/high, delegated-harness-native. Six helper tests and TypeScript pass; initial path/fixture-typing failures preserved. Parent reviewed helper/test source and verified 83 protected hashes. No browser/native resource used and no performance timing. ROOT independent whole-batch review and full sweep remain pending. This internal checkpoint is not product acceptance or C3 completion.

Standard F-PIP-2/DEC-081 fence applies.
