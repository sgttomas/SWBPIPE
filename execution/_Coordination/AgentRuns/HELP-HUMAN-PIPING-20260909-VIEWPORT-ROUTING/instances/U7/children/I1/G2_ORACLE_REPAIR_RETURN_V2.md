# U7/I1 G2 oracle repair return V2

- Sealed brief SHA-256: `df463b375e472564fd527695b803d9be9ed6a41735473be36bcc221270612c89`
- Changed path: `apps/desktop/src/App.test.tsx`
- Pre-change SHA-256: `46357cd1128c5f7d6818f950848b1af4fcf921318a9ab72022045cbc0d3c7232`
- Post-change SHA-256: `54fce6bcec3fd82bda27d5e022a4cfb14a4c7a1e457cdc89e1feedf3930e7b9b`
- Exact change: one expectation substring changed from `recovered_from_open_mechanics_stress_components` to `recovered_from_local_element_stiffness` at the torsional-shear end-j selected-result recovery-basis assertion.
- All surrounding assertions were preserved.

Focused command, run from `apps/desktop` with the existing prepared Wasm assets:

```text
../../node_modules/.bin/vitest run src/App.test.tsx -t "shows computed mechanics diagnostics in results, knowledge, and review-only proposal context"
```

Result:

```text
Test Files 1 passed (1)
Tests 1 passed | 162 skipped (163)
Duration 13.35s
Exit 0
```

No Git, build, product-source edit, broad test, delegation, or other assertion/test edit was performed.
