# C1A-PALETTE, correction 1: pause status of 2026-09-19 (not the correction's return)

Retained by B-CANVAS. What follows the rule is the child's final message as the host stored it; machine paths, if any, are replaced by placeholders (0 substitution(s)) and nothing else is changed. The message exactly as stored has SHA-256 `3e9e76e63cff28e800056d760912612045464bcac1cf0815e9e3d0af5625367b`.

---

C1A-PALETTE status for B-CANVAS (model `claude-fable-5-1`): paused after F2, F3 not started. Nothing of mine is running, I hold no lock, and I have run no state-changing git command and launched no agent.

**Where the pause falls**
- F2 is complete and green, and the F2 snapshot is in `corr1_f2/`.
- No F3 test or code has been written, in the tree or in scratch.
- The three scoped files on disk are whole and byte-identical to the F2 snapshot, so nothing needed restoring.
- The lock is currently held by T1's `with_e2e_lock.sh node …/t1/exp_gpusync.mjs` (owner pid 71220, since 04:15:35Z).
- There are no stray temp files in the viewport folder.

**Hashes verified before starting**
- The correction: `f9bb4376…79ac9`.
- The review return: `5f87c173…3e08f`.
- ROOT's addendum 3: `a86cb318…db210`.
- My brief: `e2c8fffe…b77b`.

**F2 checks** (every vitest and tsc command run from `{DESKTOP}` through `sh {RUN}/tools/with_e2e_lock.sh`)
1. Picking first, `npx vitest run src/features/viewport/viewportSelection.test.ts`: exit 0, 1 file, 69 passed.
2. F2 red, `npx vitest run` on the two test files against the old material: exit 1, 6 failed and 37 passed of 43.
   - Four clone tests fail at `expect("color" in clone).toBe(true)` with "expected false to be true". This comes right after `isFigureMaterial(clone)` passes, which is the finding itself.
   - The plain `new THREE.ShaderMaterial().copy(figure)` test fails with "expected true to be false", because the marker survives the copy.
   - The cloned-material repaint test fails with "expected 16777215 to be 7187967": the plain clone's tint stays white while its shade ratio and the instance colours are painted.
   - The console-spy test was already green at red; its positive control showed the spy does see three's warnings.
3. An earlier red run is also kept. It failed first on my class-form assertion rather than on the finding, so I reordered the helper's assertions and re-ran.
4. `npx tsc -b` with the red tests in the tree: exit 0, so the tree compiled throughout.
5. F2 green on the two test files: exit 0, 43 passed.
6. F2 green, `npx vitest run src/features/viewport`: exit 0, 9 files, 180 passed (173 plus 7 of mine).
7. F2 green, `npx tsc -b`: exit 0.
8. Build-info: `tsconfig.tsbuildinfo` is ignored by `{WORKING_ROOT}/.gitignore`, so `git status --short` does not show it.

**The three files (F2 point, snapshot copies and on disk now are all identical)**
- `viewportFigureMaterial.ts`: `db9176557b060b344a81d51bd1502a1aac3b38eea8eee0111cfc16dc8145049e`
- `viewportFigureMaterial.test.ts`: `b0e64a700e81fa0c84c4d9a7b4d0606ba9737e2edd8d9360b20ff752952f380e`
- `viewportResource.test.ts`: `8b5162c3e8c31ca842e3a40d52e40c9a62010f4fb6b6b2c34071e359228915e7`; the change is one inserted test at the end of the repaint block, and a line diff confirmed it is insert-only.

**Other files and tree state**
- The byte-identical pair is unchanged: `viewportSelectionPresentation.ts` `00384d28…5931` and `viewportSelection.ts` `fdf3eaa4…46d5`.
- `git status --short` (read-only, head `2f4208976`) shows my three files modified. It also shows `briefs/_INDEX.md` and the untracked `drafts/` and `tools/` folders under `instances/B-CANVAS`; none of those are mine.

**What F2 is**
- `FigureMaterial` is now a class extending `THREE.ShaderMaterial`.
- `color` is an own getter, defined in the constructor with `declare` for its type (no emitted field), and reads `this.uniforms.tint.value` on every access.
- `isFigureMaterial` is `instanceof FigureMaterial`.
- The `userData` marker is dropped; nothing outside the material file read it.
- Nothing warning-prone passes through `super(parameters)`, and `type` and `isShaderMaterial` are left as three sets them.
- The only `as` in the file is the existing one in `setFigureShadeRatio`, and there is no suppression of any kind.

**Difference from trap 3, found with a scratch `tsc` 5.9.3 probe against `@types/three` 0.181.0**
- TS2611 does not fire for a prototype `opacity` accessor.
- TS2612 does not fire for a field redeclared without `declare`.
- The reason is that `opacity` is declared in an interface merged into the class.
- Probe controls:
  - An accessor override of `type`, which is declared in the class body, does raise TS2611.
  - A deliberately wrong assignment raises TS2322, showing the probe checks against three's real declarations.
- The emit probe shows a field without `declare` is emitted as a define, so the hazard is silent at compile time. The own-accessor form rests on the runtime facts and on the tests.

**Exact next step on resume**
F3, tests first. No change to `viewportResource.test.ts` is planned, and picking-last has not been run yet.
1. Add `expect(material.opacity).toBe(0.82)` to the existing overlay test, and add a new opacity describe covering:
   - constructed values 0.82 and 1;
   - `material.opacity` and the uniform writing through to each other;
   - `setValues({ opacity: 0.4 })`, with no console warning;
   - `clone()` and `copy()` giving the source's value in both places, with the source unchanged;
   - the own `opacity` key and the `in` check still holding;
   - `transparent` staying independent of opacity.
2. Red run through the lock, output kept.
3. Add the own enumerable, configurable `opacity` accessor over `this.uniforms.opacity.value`, with the one-sentence doc note that opacity and `transparent` stay independent, then go green.
4. Final checks: `npx vitest run src/features/viewport`, `npx tsc -b`, picking last, `validate_claims_language.py`, `git status --short`, end hashes, and the full return.

This is a pause status, not the correction's return. I claim no usability, conformance or performance acceptance.
