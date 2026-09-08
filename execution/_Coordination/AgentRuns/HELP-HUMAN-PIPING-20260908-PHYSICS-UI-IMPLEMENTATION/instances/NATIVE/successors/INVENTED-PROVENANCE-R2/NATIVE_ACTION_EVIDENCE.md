# Native R2 action and evidence record

## Basis and identity

- Sealed brief: `instances/NATIVE/INVENTED_PROVENANCE_R2_LAUNCH_BRIEF_V1.md`, SHA-256 `e878999f3feff9e120d61d707304ff13b7c6e92d5014ef5094eb5cf3c0346dbf`.
- Frozen launch brief/amendments: `7d3aaeeade894dd662d3ca793984b1d25a34754a6ec329d52294d3367a510bb2`, `afc30a71ec2d39eca7b5868f4e706fa761c622bf5973f2b0794d5fa5e0c1b61b`, `6d68a99e9f07be570854df84e30f5031ef2b5ffdaf4a96fd2566a9c79a3fafa4`, `528357bf9cf786e9ed9b66ba659f4307ba2f5bdc56912b79c2b16589bb7b253b`.
- Corrected provenance and diagnostic triage: `cd521184a1443caff6cf15f96d212c906d2bbf26ed109be2279b334bdf28c441`; load-case limitation triage: `5ded45062e8f52b6add0957dd4f4e461d132f0e220c15e14b127940ef450869b`.
- Exact unchanged source aggregate: `1180436540fcb8025a5029b16f8d6e019928eeaa6fab74c434772ca6ec78cefe`; exact unchanged three-file bundle digest: `d54e06c6229163ad27bfb3d3ad27b1383b6cb50383d70316b0b6c83e71bd0b11`.
- Native packaged-app identity and prelaunch process/store absence are recorded in `PRELAUNCH.md`.

## Native walkthrough

1. Native CUA launched only the exact packaged `.app`. The product created `project:blank-local-20260908t174230z`, name `Blank Local Model`; no naming input was exposed. The blank state was visibly incomplete. Add review exposed initial blank basis hash `sha256:d5f9849f393e975acbe1bf71e1f68479b08190b1907c892b997f54bbe9a850de`.
2. Added and Applied `node:UI-A-100`, label `Anchor`, `(0,0,0) m`, with provenance `invented_synthetic_ui_acceptance_input`. Native Save/Open retained the node and incomplete state. The manual-lock pause is preserved in `WAITING_STATE.md`.
3. Created material `material:ui-phase-a-invented` (`E=200000000000 Pa`, `G=77000000000 Pa`) and section `section:ui-phase-a-straight` (`OD=0.168 m`, wall `0.007 m`) with the revised provenance.
4. Authored new-end route `node:UI-A-110` at `(3.2,0,0) m`, pipe `pipe:UI-A-100`, material/section references, and Y reference `(0,0,1)`. The reviewed ordered batch was `[create_node, connect_pipe_run]`, batch `batch:viewport-straight-route-pipe:UI-A-100-001`, basis `sha256:e9ea932944298eab2af0e0bee26c696b54bd85253fbaab47a6689011e977481c`, result `sha256:4be510964b3135a431dde78dd78936335297abedc8d430840fca73d859ee8e20`. Apply published it atomically with one receipt and undo checkpoint.
5. Assigned the shared section, created six-DOF support `support:UI-A-100` with UX/UY/UZ/RX/RY/RZ, created `load:UI-A`, and authored sole primitive `load:UI-A-FY` at `node:UI-A-110`, `global_y`, `350 N`. All eight provenance carriers (seven top-level entities plus the primitive) contain `invented_synthetic_ui_acceptance_input`.
6. The known sole-case draft limitation recurred. Native UI created temporary `load:UI-TEMP`, selected it, switched the primitive case back to `load:UI-A`, then deleted the temporary case before solve. Final membership was exactly 8/8 with only `load:UI-A`.
7. Original native 350 N success: job `backend-solve-job-1`, rows 67, generation 28. Native Save/Open retained the entities, quantity, provenance, model and solved result.
8. The product tree selects the load-case row `load:UI-A`; its `First primitive magnitude` editor maps to the sole primitive `load:UI-A-FY`. No separate primitive tree-row selection occurred. Through that real editor, changing only magnitude to `500 N` and clicking inline Apply once visibly cleared the old result. Original native 500 N success: job `backend-solve-job-2`, rows 67, generation 33.
9. One Undo restored the mapped sole primitive to `350 N` and cleared the result; one Redo restored it to `500 N`. `UNDO_350.png` and the later persisted/final 500 field prove the two states. Save/Open retained final 500 N.
10. The required history/save/reopen sequence left both persisted run/result JSON fields exactly `null`. Original success images remain historical evidence. With the reopened history reset, recovery used the same native mapped editor: Apply 350, solve, Save, immediate read-only store capture; then Apply 500, prove clearing, solve, Save, immediate capture. Complete accepted tuples are in `NATIVE_HASH_TUPLES.json` and the two `SUPPLEMENTAL_*_TUPLE.md` files.
11. Final native Save/Open retained 8/8 entities, 500 N, deformation normalized, and Mechanics solved. `FINAL_1024x768.png` is exactly 1024 by 768 and shows the persistent canvas, compact palette, selected `load:UI-A`, the real mapped `First primitive magnitude` field at 500 N, inline Apply, and visible solved/deformation provenance.
12. Native close was used. The exact executable has no remaining process. The exact dedicated store passed narrow member guards, was hashed, and only its three exact SQLite files plus its exact directory were removed. Post-check proves both exact process and exact store absent.

## Observed limitations and retries

- Native route Add stayed disabled while the Y-reference values appeared as pale default-looking text. Material and From selector resets, coordinate-unit roundtrip, and pipe-length-unit roundtrip did not enable it. Explicitly entering Y-reference X/Y/Z as `0,0,1` enabled Add. This is recorded as a placeholder-versus-entered-value correction; no source or unit-selector defect is claimed.
- The temporary-case recovery is an existing baseline authoring limitation documented by the sealed U7 triage. The temporary case was removed before solve.
- The retained `BLANK_PROJECT_AUTHORING_TARGET` diagnostic is a stale blank-project display note. It is kept separate from solve validity; both corrected native solves succeeded through backend validation.
- CUA paused during a locked Mac and resumed only after manual unlock. One physical-input-detection interruption delayed the first Undo click; the subsequent native Undo succeeded. One transient screenshot-unavailable response and one clipboard-read timeout occurred during property-menu recovery; re-observation showed the app and draft intact, and no duplicate Apply occurred.
- Saving/reopening after history checks does not retain historical run/result JSON. Supplemental solves were required to preserve complete tuples. No run IDs or clipped hashes were invented; the backend reused `run:preview-linear-static-001` exactly.

## Verdict

`PASS` for the sealed R2 walkthrough, with the observed baseline authoring and evidence-retention limitations recorded above.
