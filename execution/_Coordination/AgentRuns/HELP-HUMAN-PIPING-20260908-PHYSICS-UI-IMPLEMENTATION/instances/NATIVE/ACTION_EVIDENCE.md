# Native packaged U7 action evidence

**Terminal verdict:** `CHANGES_REQUIRED`
**Execution role:** fresh bounded ephemeral Agent 2, NATIVE
**Completed through:** `2026-09-08T17:27:02Z`

## Bound inputs

- `LAUNCH_BRIEF_V1.md`: `7d3aaeeade894dd662d3ca793984b1d25a34754a6ec329d52294d3367a510bb2`
- generated-project amendment V1: `afc30a71ec2d39eca7b5868f4e706fa761c622bf5973f2b0794d5fa5e0c1b61b`
- temporary-case amendment V2: `6d68a99e9f07be570854df84e30f5031ef2b5ffdaf4a96fd2566a9c79a3fafa4`
- accepted U7 implementation brief: `23a2ac4595c4b9ee2e18143b057a392f42c34c57051e5f328b45cbfc2ab1f709`
- CHANGE source binding: `3ce95c6ec51813bf5ff210ffbdb5a8b9456d396cac5fc9d76151998e87c1edc8`
- CHANGE native build result: `1e7412fa2ca67913a2006d4cfa2f965f77859336595e8feaf1f0cef7da7e64b6`
- CHANGE native build log: `fd3d048cf7f954d4089eaa32b1a8c6d6db3639293d7916aa571a8bc382403a98`
- CHANGE bundle inventory: `69e908c33b4ea3772037a151d10e55d64b98e52920fdb07dcc8dca1ec1ed34ab`
- canonical three-file bundle digest: `d54e06c6229163ad27bfb3d3ad27b1383b6cb50383d70316b0b6c83e71bd0b11`
- native load-case limitation triage: `5ded45062e8f52b6add0957dd4f4e461d132f0e220c15e14b127940ef450869b`

## Native action sequence and observations

1. The first CUA call was solely `cua.getApp()` with the verified unique full `.app` path. The returned native identity was `OpenPipeStress Technical Preview Implementation Witness 55df51ac 20260908`.
2. **New blank** and **Create local** exposed no project-name field. Native UI generated `project:blank-local-20260908t165725z`, name `Blank Local Model`, under the exact dedicated bundle store. This variance is bound by amendment V1.
3. The blank UI explicitly showed `Mechanics model incomplete`. The Add review for the first node displayed blank basis model hash `sha256:62f3f0b8bd40655f267c08ab544bbc235f4e166d1c7ed6b701d4d7e735165eb3`. This is recorded only as the observed blank basis hash; it is not claimed as the post-node hash.
4. Native Add then Apply created `node:UI-A-100`, label `Anchor`, position `(0,0,0) m`, entered provenance `synthetic_ui_acceptance_input`. Save/Open retained the node and the explicit incomplete state. A distinct full post-node-only hash was not exposed after later saves and is not inferred.
5. Native structured forms created and applied the exact material and section. One provenance-field paste attempt timed out and selected page text; the field was corrected with native typing before Add. The reviewed and applied payloads were exact.
6. Starting from selected `node:UI-A-100`, native Add showed the ordered route batch `op:viewport-create-node-node:UI-A-110-001` then `op:viewport-connect-pipe-pipe:UI-A-100-001`. Apply published both members together in `batch:viewport-straight-route-pipe:UI-A-100-001` with one receipt and one undo checkpoint. Observed batch before hash: `sha256:bd1024ecc0d987ea9299dbbdefa58c3a1ee74faf1b8f69dd6f04b0b44155c00d`; after hash: `sha256:10ead82b209d47605188e43fd08bf15e1263a1fe2d5baa235701921847a7433b`.
7. Native structured forms assigned `section:ui-phase-a-straight`, then added six-DOF `support:UI-A-100`, `load:UI-A`, and the exact `load:UI-A-FY = 350 N`. One unsupported `Space` key attempt occurred while checking support restraints; the exact six checkboxes were then clicked and verified in the review payload.
8. The first-case primitive editor visibly displayed sole case `load:UI-A` while Queue remained disabled. Bounded selection/category gestures did not reconcile the stale internal selection. Per amendment V2, native UI created `load:UI-TEMP` / `Temporary selector reset`, selected it, switched back to `load:UI-A`, added and applied the exact primitive, then deleted the temporary case through native structured review. Final model membership was eight entities and only one load case, `load:UI-A`.
9. The actual native menu **Analyze → Run Mechanics Preview** invoked the Tauri backend route. It returned a blocked mechanics result with zero rows and seven `PROVENANCE_INPUT_MISSING` diagnostics. The native Issues drawer showed ten total diagnostics: eight blocking, zero errors, one warning, and one info. `BLOCKED_DIAGNOSTICS.json` separates the one retained blank-project blocking diagnostic from the seven solve-result provenance blockers.
10. The entered and persisted provenance was preserved exactly for both nodes and every other authored entity. An earlier inference that node provenance had been dropped was withdrawn after the authoritative dedicated-store read. The accepted label `synthetic_ui_acceptance_input` does not contain the existing backend acceptance marker `invented` or `cleared`, so all seven physical/input record classes failed the backend provenance gate.
11. Per root instruction, no model change or solve retry followed the incompatible-label finding. The required successful initial solve, persistence-after-success, `500 N` inline revision, second successful solve, undo/redo, and final revised persistence were therefore not executed.
12. Native UI saved the exact blocked `350 N` fixture. The app window was resized to `1024 x 768`; CUA saved the final Results view and a separate diagnostics view plus accessibility evidence. The exact native close control was invoked and the exact bundle executable process was absent afterward.

## Acceptance blocker

The sealed fixture provenance value is incompatible with the packaged backend's existing accepted-provenance marker rule. The exact persisted fixture value is `synthetic_ui_acceptance_input`; native result diagnostics require provenance containing `invented` or `cleared`. Read-only source guidance confirmed the rule in `core/product_physics/src/validation.rs` (file SHA-256 `cc98b205467c9a965adc03179e91c43f174ba4b9bafb0f7aff47ed5ad6e936a7`). No source or store field was changed out of band.

The final model also retains the blank project's original `BLANK_PROJECT_AUTHORING_TARGET` diagnostic in the model document. Root's producer confirmation records this as a stale baseline display limitation: the backend ignores model diagnostics and input analysis status, recomputes validation, and the App run does not gate on merged Issues. The native solve result itself lists exactly the seven provenance diagnostics as its mechanics blockers.

## Visual evidence

- `FINAL_NATIVE_1024x768.png`: `1024 x 768`, SHA-256 `a9ed18de4576395b2986b0330993f84272f4072fba05b338a6b8a1713dae7ad8`
- `FINAL_NATIVE_1024x768_AX.txt`: SHA-256 `e371a49a41f8f55974cd3d57b69d345b445f1e861ff91d5c99e997627ec8ab45`
- `BLOCKED_NATIVE_DIAGNOSTICS_1024x768.png`: `1024 x 768`, SHA-256 `05b284f96ba45db8aa5fa4c98fd9db7cad7a4e9acd3817019b67f056cbd0d275`
- `BLOCKED_NATIVE_DIAGNOSTICS_1024x768_AX.txt`: SHA-256 `54197e9f12db168d3ea4d0f9ad7ab48d893971848b66687694ac799f8a0fbbb5`

The Results visual shows the persistent 3D canvas, compact palette, blocked state, and native solve proof. Because execution stopped at the first-solve blocker, it does not and cannot show the required final selected `500 N` editable field and inline action.
