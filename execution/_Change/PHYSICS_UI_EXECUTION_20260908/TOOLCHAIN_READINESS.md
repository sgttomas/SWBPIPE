# Physics/UI execution toolchain readiness

Status: **READY_WITH_SERIALIZED_NATIVE_BUILD_GATE** at `main@779dedb8670625b36af07b89fc5557470e47c50e`.

## Provisioned local tools

The lane-local Node workspace was absent and was provisioned from the committed lockfile with `PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1 npm ci` in `projects/chirality-piping/`. The command exited 0, added 206 packages and audited 208 packages. `node_modules/` is ignored. `package-lock.json` remained byte-identical to `HEAD` (Git blob `490bae945bd016ee6c43b879b36b71c184b8cb67`; SHA-256 `0dd1616e1ef3c596d5cdaa8e56994f26127c20943bfde066f26c9f924c65d732`), and no manifest or lockfile changed. NPM reported eight dependency audit findings (two low, one moderate, five high); no unrequested audit fix or version change was made.

Available versions are Node 24.18.0, npm 11.16.0, TypeScript 5.9.3, Vite 7.3.3, Vitest 4.1.10, Playwright 1.60.0 and Tauri CLI 2.11.1. `npm ls --depth=0` exited 0. The Playwright-managed Chromium executable exists, and the registered configs may also use installed system Chrome 152.0.7977.83.

The preparation Python environment remains reusable at its host-local temporary path: Python 3.13.14 with PyYAML 6.0.3, NumPy 2.5.3 and pytest 9.1.1. `pip check` returned “No broken requirements found.” This environment is a transient host dependency, not a durable project path; later executors must probe it again and recreate it from `requirements-dev.txt` plus the governance-harness NumPy dependency if it disappears.

Rust/native prerequisite discovery found cargo 1.97.1, rustc 1.97.1, targets `aarch64-apple-darwin` and `wasm32-unknown-unknown`, wasm-bindgen 0.2.123, Xcode 26.6 and the Xcode clang toolchain. F4 released the initial compile slot after this discovery; root/U7 still owns release of the serialized native build. This readiness pass ran no cargo fetch/build/test, Wasm build, Tauri build, application binary or full test surface. Cargo-cache and produced-artifact readiness therefore remain for the later serialized validation owner to prove.

## Registered validation routes

`projects/chirality-piping/software-workflow.json` currently registers:

- `apps/desktop/**`: desktop Vitest plus production build;
- `core/**`, `validation/**`, `tests/**`: Piping pytest plus the full evidence sweep;
- `execution/**`, `docs/**`, project `AGENTS.md` and `loop/**`: practitioner-harness pytest;
- repository self-check: always.

The profile is a check selector and explicitly does not replace project gates. The committed workplan requires the five-surface `DEC-025` sweep for every code-touching branch, while authoritative `DEC-025` states the local sweep is the pre-push/fan-in merge gate for every parallel-agent development branch.

An independent reference calculator written as `.py` inside a deliverable `_run_records/` directory is executable scientific/technical logic. It is therefore code-touching for the workplan gate, even though the profile's path rule selects only `harness-pytest` from the `execution/**` location. The full `DEC-025` sweep remains required for the resulting branch. A companion `.json` input/output or result record does not create an exemption. This classification does not broaden the change scope and does not claim the calculator is product runtime code.

## Native and packaged walkthrough posture

The exact established native bundle command is `CARGO_NET_OFFLINE=true npm run tauri -- build --bundles app` from `projects/chirality-piping/apps/desktop/`. Tauri then runs the configured `beforeBuildCommand`, `npm run build:wasm && npm run build`, and targets the macOS `.app` bundle. The expected binary is `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop`.

The native application identity is `OpenPipeStress Technical Preview`, bundle identifier `org.openpipestress.technical-preview`, executable `openpipestress-desktop`. The packaged binary exposes `--self-test-saved-edited-load`; source confirms that this mode creates `<system temporary directory>/openpipestress-packaged-edited-load-smoke-<pid>-<unix-nanoseconds>/openpipestress-projects.sqlite3`, runs its invented saved/edit/reopen/solve witness, and removes the whole temporary directory before returning. Its exact post-build invocation is `apps/desktop/src-tauri/target/release/bundle/macos/OpenPipeStress Technical Preview.app/Contents/MacOS/openpipestress-desktop --self-test-saved-edited-load`. It is a compiled-backend witness, not a packaged GUI walkthrough.

The registered Playwright source and production-dist browser routes can exercise visible UI controls with isolated browser context and both registered viewports, but they do not exercise the `tauri://` application shell. There is currently no registered automated packaged-Tauri GUI walkthrough.

Source inspection found no ordinary-GUI project-store path flag or environment variable. A subsequent conventional Tauri build-flavor check established a source-free isolation route: a `--config` overlay with a unique bundle identifier causes `app_local_data_dir()` to select a distinct macOS Application Support directory, while a unique product name gives the bundle a distinct application path. Exact source basis, overlay, build/launch command, store path, and guarded cleanup protocol are recorded in `NATIVE_WALKTHROUGH_ISOLATION.md`. Initial discovery made one metadata-only existence probe against the normal identifier directory, with no enumeration or content access; there was no database open, content read, hash, move, or write. This readiness pass did not build, open, quit, inspect or modify a running app or user model/data.

Generated `dist/`, `public/wasm-engine/` and Tauri bundle artifacts are absent in this fresh lane. Root/U7 must release the serialized native build before any command that creates them. The eventual implementation closeout still requires the relevant focused checks, fresh review where project rules require it, the full sequential `DEC-025` sweep for F/UI or calculator code, and exact-head hosted evidence as applicable.
