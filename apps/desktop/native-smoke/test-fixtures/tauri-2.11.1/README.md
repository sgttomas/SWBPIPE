# Tauri 2.11.1 webview IPC scripts (test fixture)

Byte-for-byte copies of the webview scripts of the `tauri` crate **2.11.1**, the version
`apps/desktop/src-tauri/Cargo.lock` resolves (crates.io checksum
`b93bd86d231f0a8138f11a02a584769fe4b703dc36ae133d783228dbc4801405`). They are copied from the
crate's `scripts/` directory, unmodified, under the crate's own licence (Apache-2.0 or MIT; both
texts are here).

`perf.test.mjs` runs them in a page so that the C1 driver's command observation is tested against
Tauri's actual IPC: `invoke` (`core.js`) → `__TAURI_INTERNALS__.ipc` (`ipc.js`, the brownfield
pattern of `pattern.js`) → `__TAURI_INTERNALS__.postMessage` (`ipc-protocol.js`), which sends one
`fetch` of `ipc://localhost/<command>` and, after any failure of that fetch or of reading its reply,
switches permanently to `window.ipc.postMessage` (wry). They are a test fixture only: no build,
bundle or App run reads them.

| File | SHA-256 |
| --- | --- |
| `scripts/core.js` | `71056442a732cca1f03572ccae96d68e16104c3a591c64de9618cd304e063070` |
| `scripts/ipc-protocol.js` | `68ae690606006f733bde75dd8bb6748464954a22c7e83914a93b1abc221d7072` |
| `scripts/ipc.js` | `3e3766fc90efa68003096cbf0a07c6a00cb31c5756d55b5a68ebbfd2930c4865` |
| `scripts/pattern.js` | `149570459687f1636c02e41ede8b40efe52378965eeddb6dbfebc0c525a3551b` |
| `scripts/process-ipc-message-fn.js` | `d51917f89e4d3d318d63f1f4f5a7ac7ea1399a5b85c81a840c4d606597f29132` |

The test checks these hashes and the lockfile's `tauri` version and checksum, and, where the cargo
registry holds the crate's source, that each file is identical to it. When the lockfile moves to
another `tauri`, the test fails: copy that version's scripts here and re-check the observation.
