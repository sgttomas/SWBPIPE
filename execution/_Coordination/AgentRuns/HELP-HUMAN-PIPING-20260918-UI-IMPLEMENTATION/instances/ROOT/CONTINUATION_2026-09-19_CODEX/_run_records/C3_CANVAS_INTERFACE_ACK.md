Confirmed workable; draft SHA256 matches `34815f05fe750db1ae36af3dff30a117d3e74d3ffde6522d32f64fb60671cea9`. No conflict found.

Proposed canvas export from `viewportVisibility.ts`:

```ts
type ViewportVisibilityProjection = Readonly<{
  hiddenKeys: ReadonlySet<EntityKey>;
  dimmedKeys: ReadonlySet<EntityKey>;
  hiddenCount: number;
  isolationActive: boolean;
}>;

deriveViewportVisibility(
  index: ModelIndex,
  explicitHiddenKeys: ReadonlySet<EntityKey>,
  isolationSelectionKeys: ReadonlySet<EntityKey> | null,
): ViewportVisibilityProjection;
```

Use `onIsolationSelectionChange(keys: ReadonlySet<EntityKey> | null)` for the snapshot callback and retain `onClearVisibility` for Show All.

Semantics confirmed: an empty active snapshot dims all eligible geometry, including newly added entities; only `null` means inactive. Show All remains operable when `hiddenCount === 0` if isolation is active. Hide attachment expansion remains separate from literal isolation membership.

Order is workable: canvas freezes the pure helper/type first; shell freezes B3B, then solely implements the C3 state/controller/App/style/test bridge. Final integration removes obsolete live isolation state rather than maintaining parallel routes.

No implementation or timing activation inferred. Behavioral, resource and settling checks remain planned; D72 timing stays open until the owner-held profile freeze.

