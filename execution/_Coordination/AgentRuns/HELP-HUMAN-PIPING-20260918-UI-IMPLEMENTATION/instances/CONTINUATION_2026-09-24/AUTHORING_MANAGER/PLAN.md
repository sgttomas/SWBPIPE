# M24 authoring and units candidate

Base: `f7e8b467cb2db244f11fe49cede636140031b387`. Parent owns programme integration. This is bounded execution evidence, not a second central receipt or final acceptance.

## Contract selected

New blank projects and new thermal authoring use `degC`. Existing `C` is a narrowly scoped compatibility spelling for absolute temperature and temperature interval; its interpretation follows the explicit dimension. Saved project bytes and hashes are not silently migrated. Canonical catalog identity remains `degC`.

Add `kN`, `kN*m`, `GPa`, `bar`, `N/mm`, `kN/mm`, and `1/degF` in existing applicable dimensions, preserving existing entries. SI prefix definitions and the Fahrenheit interval ratio supply factors; no engineering table is needed. Numeric pressure unit alone conveys no gauge/absolute or reference basis. Dimension and offset errors remain blocking.

Remove operation application’s unknown-unit escape when a token matches the project preference. Preserve valid dimension-compatible inputs and explicit legacy compatibility.

## Verification and integration

The implementation TASK prepares meaningful maintained tests. Conversion checks cover independent numeric expectations, temperature scale/interval distinction, invalid dimensions, and pressure reference behavior. Desktop checks cover new default and save/reopen preservation. A separately owned product solver regression must apply authored operations and solve the resulting model, including degC and legacy C variants; browser fixtures alone do not satisfy this check. Solver manager owns that adapter.

All Cargo, npm, build, browser and native execution waits for the parent resource grant. Source read/edits and test preparation proceed. Fresh independent Astra/xhigh review follows a frozen diff; fixes need backcheck. Local commits follow checks/review, with no push, PR or integration-branch merge from this manager.

## Continuation

Library-to-model assignment (M23) and authoring throughput (M25) remain later planned work until this bounded unit handback. Parent/C4 retains tables, workspaceSession and graph; solver manager retains product_physics, solver and loads. No governance instruction, release or professional claim is changed.

Manager source amendment after implementation return: README wording now distinguishes legacy-C canonicalization from other valid temperature preferences, and cites current BIPM prefix table. Implementation-return hashes describe its own handback; final reviewed candidate hashes will bind manager amendment too.

ROOT authorized the bounded unitCatalogService.ts/test extension after manager/reviewer discovered false native C mismatch metadata. It preserves entered labels and persisted bytes, resolves only explicit temperature dimensions, and received expanded fresh review plus 19 focused tests and TypeScript check. Runtime work ran only under ROOT sequential grants; logs and source/artifact identities are in checks/.
