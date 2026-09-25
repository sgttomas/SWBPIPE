# Exact pressure authoring — bounded TASK return

Status: implementation present; core connected tests passed; frontend tests and independent review pending in the parent's coordinated lane. No commit, native/browser run, release or acceptance claimed.

Execution: actual delegated-harness-native TASK `/root/physics_resume/exact_authoring`, parent `/root/physics_resume`; no descendants. Work stayed in the joined checkout selected by the brief. Starting Git HEAD `0a438a68672269e5750900792c9565ae9e5555b0` plus the parent's selected integration changes. Host filesystem permits the `/private/tmp` checkout; write scope follows the brief and explicit parent transfers. Parent transferred applier `lib.rs`/`rich_authoring.rs` after density repair and later granted the narrow App prop wiring and model/input portions of ProjectValidation/projectService. Parent retains product facade/types/native persistence and the reader manager retains result paths. Cargo lane was held until parent release, used with two jobs and an isolated target, then returned.

Implemented:

- New `pressure_authoring.rs` validates rich `Model/set_field/pressure_profile`, `Material/set_field/constitutive_properties`, and `Load/update_load/pressure_regions` replacements. These use the existing validation, diff, apply and backend-hash route, require complete current hash evidence, check canonical before-values, and never bypass the operation pipeline. Profile writes only schema version plus the explicit v2 contract. It rejects missing case arrays and any retained pressure primitive, including zero. Existing load/material history is not reinterpreted.
- Exact material creation admits explicit homogeneous E/nu without G; retained G stays authored and nonauthoritative in exact mode. Nu requires unit `1` and the open interval (-1, 0.5). Region quantities, ordered terminals, closure tokens and all provenance survive. Unknown exact-create payload fields are rejected rather than dropped. Structural reference checks precede solver topology/physical diagnostics.
- `PressureAuthoringPanel` provides explicit profile choice, E/nu creation, and per-case region/member/pressure/ordered-terminal/provenance controls. Empty arrays require their own queue action. The material inspector adds E/nu edits; temperature table retains nu. LoadCaseManager receives the existing busy, queue and preparation-epoch guards. Existing scalar G display is absence-safe.
- Model/input unit and provenance observers cover E/nu, point nu and pressure regions. No real engineering library or code-rule values were added. The maintained full UI fixture `fixtures/model_operations/exact_pressure_authoring_model.json` supplies synthetic physical inputs for connected tests; permanent tests do not load run evidence.

Verification completed (rustc 1.97.1 `8bab26f4f`, cargo 1.97.1 `c980f4866`, Node v24.18.0):

- Rust parser/rustfmt checks of the new module/tests and parser check of edited rich-authoring; JSON syntax check of the maintained fixture.
- `CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=/private/tmp/piping-create-section-inputs-target cargo test --manifest-path projects/chirality-piping/core/model_operations/operation_applier/Cargo.toml --test exact_authoring -- --nocapture`: **3 passed**. The connected test starts with a legacy profile, applies genuine material and region operations followed by profile selection, checks canonical model equality and JSON transport preservation, then solves through both actual sparse and dense public entrypoints. Closed-end extension agrees with the independent annular axial-strain expression after explicitly converting metres to the emitted millimetres. Other tests cover missing declarations/closure/hash, negative nu, no-G material creation and point-nu preservation/rejection.
- Two test-only corrections preceded that pass: canonical equality replaced serde integer-vs-float representation equality, and the analytical expected displacement was expressed in the verified emitted unit `mm`. These were test expectation errors, not implementation repairs. Product facade currently emits an unrelated unused-helper compiler warning.

Frontend launch: the reader's combined ten-file Vitest invocation stopped in global setup before any tests ran because this candidate lacks its built `public/wasm-engine` assets (`WASM-ENGINE-ASSET-ABSENT`). No stale WASM substitution or setup bypass was used. Current-source WASM build and rerun remain pending. Reader typecheck found no errors in the new authoring files; shared optional-G fallout was limited to known legacy `MaterialTable.test.tsx` assertions and the NativePackage model witness (plus a reader-owned test assertion). Parent explicitly granted the known-fixture non-null G test repair, now applied without changing values or expectations; parent owns NativePackage repair. Rust sources are frozen for the coordinated current-source WASM/TS batch. Scoped `git diff --check` passed.

Pending integration evidence: the five new frontend cases plus rich-form/load-case/project-service regressions need that candidate engine build and the single shared TS lane. Parent owns native model-0.3 save/reopen support and connected SQLite solve evidence, actual App undo/redo verification and fresh independent review. Plain JSON roundtrip in the core test is not represented as native persistence or App undo evidence.

Supplied instruction origins (SHA-256, all repository-relative):

- `AGENTS.md`: `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd`
- `projects/chirality-piping/AGENTS.md`: `d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792`
- `projects/chirality-piping/loop/LOOP_INIT.md`: `c712b6487faa3fab461e181aad1a6c16a2cb0eb734c510744e5037289105df1b`
- `.agents/skills/chirality-change/SKILL.md`: `2b490e172436417896c1cd25dbcd543c676e3473aa58b7663985d75785ff7dba`
- Sibling `BRIEF.md` (includes full TASK role): `c1e73539518b4c73c43c21d66dfdc93537e56d1244b7a17ab3e7b7e3aac37305`

No other role or workflow body was loaded. Additional scope/lane instructions are the actual parent messages in this run, not inferred authority from this return.
