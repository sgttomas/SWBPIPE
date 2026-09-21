Recommend **B4.2: complete Node scalar editing and unify its direct/review rendering**, before migrating other families.

Add Label and string Provenance through the existing typed operation route alongside XYZ. Extend the common table core with explicit text/quantity behavior, then use it for both immediate editing and retained multi-change review. Retire the duplicate legacy Node renderer only after equivalent review, label and provenance coverage passes.

The bounded source set is:

- `EngineeringTable.tsx`, `tableState.ts`, `modelTableAdapter.ts`
- `ModelTree.tsx` and scoped table styles
- Their focused tests, the existing B4 browser journey and precisely affected legacy consumers

No engine, schema, VirtualList, controller, native-menu or CLI change appears necessary.

Two semantics need explicit preservation. Direct text editing must follow engine validation—trim text, reject blank and accept an explicitly entered `TBD`—without silently converting a blank into that marker. Legacy review currently captures before-values at Queue and queues only visible changed cells; preserve that behavior and its history granularity. A staged draft must not be reported as an applied operation.

Focused proof should cover Label/Provenance payloads, no-op and rejected input, text undo versus model undo, and an edited label filtering its own row out without losing a visible keyboard destination. Retain two review drafts across disclosure, Tree/family and unit transitions; verify filtered Queue preserves unqueued drafts, stale-generation protection and Undo/Redo truthfulness. The common renderer must retain the repaired finite body, stationary controls and aligned horizontal scrolling.

After that increment:

1. Migrate supported family scalars by field kind: text/quantity first, then validated enum/reference choices. Keep coupled or structured fields on their dedicated routes; structured provenance must remain unflattened.
2. Close the explicitly deferred Model/narrow-Both drawer layout gap before full B4 completion.
3. Address joined/expanded rows with a separately scoped variable-height contract, then common selection/copy/CSV and session-only Checked behavior against a stable row-content projection.

Numeric renumbering, downstream propagation and other missing operation semantics remain separate dependencies. B5 follows B4; live CLI binding remains separately activated.

This recommendation uses frozen `ed8` and worker return `f7aa4411…`. Preparation made no writes, tests, builds, UI actions or delegations; wt3 remains clean at `c0a3314e2b141d8f93b90364250d2bffa89433f7`.
