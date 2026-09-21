**Both P2 findings are closed at `db35d7582e61d69ad415df7cdf4132b91e710dd8`.** No new actionable findings remain in the six-path repair. The repaired source is suitable for integration and the remaining verification stages.

- **Popup clipping — closed.** `useEnumEditor.tsx:33–42` now checks the persistent input’s intersection with the body slot and grid client viewport on both axes. Empty intersection blocks opening/selection and passively closes an existing popup. Partial intersections retain existing placement. The change does not move the input, change text, apply an operation or restore focus; existing hidden/inert/source/options/detachment guards remain intact.

  The original real-scroll oracle is byte-identical to the fail-before test. Both browser profiles now show scrollTop `0→90`, intersection `35→0`, popup `null`, and option hitability `false`, while preserving `pi`, input focus, disabled Undo and zero edited markers. Cleanup verifies the unchanged model hash. I inspected both repaired screenshots. Horizontal partial/full clipping is supported by focused geometry tests, not a claimed horizontal browser/native witness.

- **Section unit status — closed.** `modelTableAdapter.ts:79` includes Section quantities in the existing pre-engine `not_run` policy. Dimensionless and other-family policies remain unchanged. Existing missing/malformed-unit Queue tests now assert that status while retaining their exact empty-unit and before/after assertions. Five retained fail-before cases demonstrate the former incorrect declaration; the affected suite passes after repair. No unit fabrication or engine bypass was introduced.

The new hook tests exercise visible partial overlap, complete clipping, preserved draft and focus. The targeted App fixture supplies the client dimensions needed by the new geometry guard and restores all three mocks in `finally`. No production changes occurred in `EngineeringTable`, `VirtualList`, engine or controller.

Verified evidence: **73 tests across four files passed; targeted App 1 passed/217 skipped; TypeScript passed; exact four clipping/enum browser executions passed in 13.6 seconds with zero retries, skips or flaky results.** These overlap earlier coverage and are not a fresh full-suite result. The metadata fail-before record’s limited command/exit metadata is explicitly disclosed.

Reviewed exact range in wt3:

`76de10606d2fdd4f8e1090883ab5d049567e975f..db35d7582e61d69ad415df7cdf4132b91e710dd8`

All six candidate SHA256 bindings match the sealed brief; changed-path validation passed. Final evidence commit `b028c82ba44d528fc169fec2e1b82e1bc58fa6dd` contains no subsequent desktop/core changes. I independently verified all **48 committed repair-manifest payloads**, with no hash mismatches.

Consulted origin/hash bindings:

| Origin | SHA256 |
|---|---|
| ROOT `_run_records/B4_4_P2_BACKCHECK_LAUNCH.md` | `72fef4026a2d5fec411af7b15b3c97475b015e14a3ca1455a1b7f749b4bb6402` |
| Original source-review return | `7a6319dba54889bcc73bfd582e9af536111583dbc2c1425ec4262901cd5a7b3b` |
| ROOT repair disposition | `7ada4ff31acb600fda73d4d2d4e3b025e069746d12386c6018f1bf70f83dbcf4` |
| `REVIEW_REPAIR_HANDOFF.md` | `3edf8171cd864392ae09141d2f27facfd63ad001ca81937a8829e3204157b440` |
| `REVIEW_REPAIR_MANIFEST.json` | `b4cfa87e8751a59159bd92569b5edee0e1c45a908a26f38a8cc03e73413f2211` |
| `worker/unit-status/RETURN.md` | `ae56482250d6fc5f1b0bd349cb383f5a503fa8b2d64afe82709631f99f13ef15` |
| `worker/unit-status/MANIFEST.json` | `7bf57d78e5eca7dfcc3819b72926d47ee6d1e51e7b40182f5f257fe6062a6dc8` |
| `worker/unit-status/browser-report.json` | `a0b2de2d21e300b72def09790e87c03623e0e926c9893d18450446940f2018d6` |

ROOT paths are under the original `CONTINUATION_2026-09-19_CODEX` origin; repair records are under wt3 `instances/B4-4-CODEX/_run_records`. The sealed brief binds the six exact source paths/hashes; the verified manifests bind the consulted raw outputs and screenshots.

Attribution: retained independent TASK `/root/b4_4_source_review`, actual **gpt-6-astra/xhigh**, resumed by ROOT. No descendants, writes, Git mutations, tests, builds, network or browser/native interaction were performed by this reviewer.

Native verification, final clean registered sweep, final combined-record review and actual-candidate hosted CI remain pending. This closes the two source findings only; it establishes no product acceptance, release or full B4 closure.

