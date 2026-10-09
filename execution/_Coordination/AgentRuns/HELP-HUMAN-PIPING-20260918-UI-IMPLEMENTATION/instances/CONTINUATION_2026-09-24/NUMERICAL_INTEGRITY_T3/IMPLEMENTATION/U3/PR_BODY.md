## The legacy pressure contract is retired product-wide

The owner retired the legacy pressure contract (`1.0.0/legacy_pressure_v1`) and the flawed computation behind it, product-wide. Before this PR the ordinary route already refused nonzero legacy pressure. The computation survived in a test-only historical scope, in a zero-pressure admission of labelled documents, and in the code itself. This PR removes it, refuses every legacy pressure input on every route, and replaces the bundled demo results that were computed with flawed premises.

**Refusals:**
- **`PRESSURE_MODEL_REAUTHOR_REQUIRED`** (blocking) refuses:
  - the `1.0.0/legacy_pressure_v1` label;
  - any pressure primitive in a non-exact (0.1.0 or 0.2.0) document, zero included.

  It applies on the ordinary route, the runner and the retained entry. One re-author text names `2.0.0/exact_straight_pressure_v2`.
- **`OP-PRESSURE-PRIMITIVE-RETIRED`:** the operation applier refuses authoring a new pressure primitive. Existing ones stay editable and deletable, so a document can be re-authored. The app no longer offers the pressure category.
- **G11:** a flexibility joint that the builder used to skip silently, while its review rows said "consumed", is now refused:
  - `JOINT_ELEMENT_STIFFNESS_INCOMPLETE` when a user stiffness value is missing;
  - `JOINT_ELEMENT_MAPPING_UNRESOLVED` when its pipe or node does not resolve.

  Each test fails before its fix.
- `primitive_loads` no longer computes a straight-pipe pressure thrust. Its only callers were its own tests and the mechanics benchmark.

**Removals:**
- the legacy thrust, bend radial thrust, joint thrust, hoop and longitudinal pressure rows, and the W2 pressure-thrust family (PP);
- the pressure membrane (`stress_recovery`) and the radial-pressure API (`curved_bend`);
- the test-only historical scope, its bypasses, and the oracles that pinned the flawed numbers;
- the validation cases that were oracles of the retired computation:
  - `STRESS-PRESSURE-MEMBRANE-ORIGINAL`;
  - `MECH-CURVED-BEND-PRESSURE-THRUST-ARC`;
  - MILLTOL's two membrane values;
  - MECH-TP-PHYS-008/009's pressure halves.
- 0.1.0 and 0.2.0 stay as the pressure-free namespace. The exact contract is unchanged.

**The B1 in-build memory profile is re-pinned.** The dead fields' removal shrinks two profile atoms, so every phase is exactly 800 B lower in both modes. No binding, form or phase changes.

**The demo.** The browser's bundled results are now the current product's output for a valid demo model, `invented_demo_model.json`: joint-free and pressure-free, and it solves. That model is also the app's default session model. The recipe regenerates the fixtures byte for byte on this head.

**Published text.** Texts that described a treatment the product no longer performs are corrected:
- the curved-bend and joint review rows;
- the two joint validation messages;
- the product-preview formulation limitation (T2).

T2 is carried by the source-block fixtures, whose two receipt digests bind it. ROOT's exact check holds for every re-pinned carrier: replacing only the string, then recomputing the digests by the product's rule, gives the head's own output byte for byte. One further carrier, the carrier-cases file's hash of one re-pinned fixture, is outside the ruled radius and is listed for confirmation. The retained-route limitation (T4) is true and unchanged; it waits for the next corpus generation.

**Evidence** (Mac; B = main `7eae707bb7`, candidate `fe657e3a68`):
- **The 40 manifests:** 2,755 ok, 3 FAILED, 80 ignored, against main's 2,776, 3 and 80.
  - The 57 changes are, name for name, the tests removed and added in source.
  - The 3 FAILED are the known Mac failures at both sides, which PR-N fixes.
- **src-tauri:** 118 (main 116).
- **pytest:** 4,428 passed, 32 skipped (main 4,426, 32).
- **vitest:** 4,250 tests, 4,249 passed and 1 todo.
- **e2e:** `r2-smoke`, `gui-workflow-validation`, `result-compatibility` and `ui-foundation`, 118/118, on Playwright's bundled headless shell.
- **Bytes against main:**
  - exact 96/96, B1 64/64 and W1 64/64 are equal;
  - of the pressure-free rows, 332 are equal and 6 differ only in a declared string;
  - 26 source-block rows carry T2. Their envelopes pass ROOT's extended check. Their export documents also pass with the document's own two source bindings recomputed, which is listed for ROOT.

**Merge with current main.** Main has moved to `ec5d397359` (PR-N). Merging it gives one conflict, in `PP/src/lib.rs`'s imports: main adds `correct_norm::{norm2, norm3}`, and this branch drops `exact_rounded_sum`, whose last use was the legacy pressure path. The resolution keeps both changes, and PP compiles with it, with the same warnings as main.

**Not in scope:**
- the joint element's code, which is deleted with T4's corrected joint;
- the five governance documents' pressure statements: a separate PR, authorized, cut after this one merges;
- the friction-reversal coverage re-author: a follow-up TASK;
- `buildPreviewComparison` on preview-physics-1 combination rows: routed to T6.

The evidence package is `IMPLEMENTATION/U3/` (CHANGE_RECORD.md). Implemented by I110 (T3), with the demo lane by I114 and M07's premise by I111.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
