# Stock agent supplier build resources

Standing: RT-E1 source/metadata/tooling implementation, 2026-10-29. Offline check registration and independent packaging/permissions scrutiny are
complete for this source-preparation slice; its TLS-environment finding is
repaired. The bundled-loader/canonical-build increment below is implemented in
source and independently reviewed; actual artifact verification and
distribution qualification remain required. No
supplier was executed for this work. This document supplements
[embedded-agent-portability.md](embedded-agent-portability.md); it does not
change ScopeOfWork, needs, acceptance criteria, platform support or release
standing. The owner's “Switch it on. The app should be built so it will work on
other machines too.” authorizes implementation; isolation and cleanup still
precede launcher exposure.

## Resource contract for integration

Project-relative `apps/desktop/src-tauri/resources/agent-supplier-pins.json`
uses schema `swbpipe-agent-supplier-pins/1`. `supplier`, `version` and `source`
identify the stock upstream. `targets` is keyed by Rust target triple. Each
target contains independent `archive` and `binary` objects with integer byte
`size` and lowercase hexadecimal `sha256`. `archive.url`, `format` and `member`
describe the sole selected release member. `binary.resource_name`, `format`,
`architecture`, `minimum_macos`, `minimum_load_command` and `sdk` describe the
original thin Mach-O bytes. All paths in `licensing.files` are relative to the
project root, with upstream name, exact byte size and digest.

This is a reviewed build input, not a dynamically refreshed registry or an
architecture fallback list. Native architecture/runtime resource resolution is
manager-owned. The manager can bundle the selected executable under its
`resource_name` and use the same `binary` digest/size/architecture for runtime
verification before exec. Never select a PATH executable or a different target
as a fallback. Runtime verification must use the bytes that will actually be
executed; re-signing alters those bytes, so the original-stock digest cannot
silently be reused for a modified executable. A packaging/signing treatment
needs qualified evidence before exposure.

The new `tools/agent_supplier_resource.py` is explicit build-time tooling only.
From the project root, for a manager-selected target and a caller-owned existing
output directory:

```sh
python3 tools/agent_supplier_resource.py --target aarch64-apple-darwin \
  --archive /absolute/path/to/pinned-release.tar.gz \
  --output /absolute/path/to/build-resources/codex-aarch64-apple-darwin
python3 tools/agent_supplier_resource.py --target x86_64-apple-darwin \
  --download --output /absolute/path/to/build-resources/codex-x86_64-apple-darwin
python3 tools/agent_supplier_resource.py --target aarch64-apple-darwin \
  --verify-binary /absolute/path/to/build-resources/codex-aarch64-apple-darwin
```

`--download` fetches the exact official release archive at build time; it does
not implement or choose the portability design's conditional first-use route.
The bundled-sidecar preference remains. No actual bundling blocker has been
established, and no first-use download fallback is selected. Any such fallback
requires Agent 0 disposition under the governing design.

There is no import-time I/O/network, implicit cache discovery, supplier/model
execution, subprocess, provider setting, user home lookup or signing operation.
The manifest is the trust root; callers must review it rather than accept a
downloaded replacement. The tool accepts only the named official stock release
URLs. HTTPS redirects are restricted to GitHub and its release-asset host,
without proxy environment or auth configuration. An explicit verifying
`SSLContext(PROTOCOL_TLS_CLIENT)` supplies the HTTPS handler, requires TLS 1.2
or later, and retains hostname/certificate checking. It loads only Python's
compiled-in OpenSSL CA file/directory locations through explicit arguments;
it does not call default-cert loading or honor `SSL_CERT_FILE`/`SSL_CERT_DIR`.
Raw context construction does not enable `SSLKEYLOGFILE`, so download cannot
create that environment-selected secret output. Missing compiled trust refuses
before network/publication; the explicit verified local-archive route remains
available. This is build-time tool trust, separate from the product relay's
unchanged OS trust-store decision. Downloads have a 30-second
socket timeout, 600-second checked transfer deadline (one bounded read may
finish after the deadline), exact byte-size bound and streamed hashing. There
is no retry or fallback. Archive and executable hard caps are 256 and 512 MiB;
these exceed both current pins without admitting unbounded input.

Acquisition creates a fresh mode-0700 staging directory in the explicitly
chosen output filesystem. An explicit local archive is copied through the
same size/hash checks; final-component symlink/nonregular inputs are refused.
Extraction never calls extract/extractall. It copies only the exact regular
member, rejecting links, traversal, directories, devices, duplicates and other
members. Decompressed tar input is capped at executable size plus 64 KiB,
including a bounded drain to check gzip integrity. Before publication it
verifies executable size/hash and thin Mach-O executable CPU. It publishes
using an atomic no-replace hard link, refuses even a dangling preexisting output
symlink, and removes its private staging on success or failure. It cannot
overwrite unrelated existing output, including a destination created during
verification. Output parent directories must be caller-controlled; this is not
a defense against another principal who controls or can replace those parents.

## Bundled source and canonical app build

`tools/agent_supplier_bundle.py` prepares one fresh private resource directory
and a Tauri `--config` overlay from one explicit archive, cached binary or
build-time download choice. It reuses the checked source manifest and bounded
acquisition functions; copied licence/NOTICE, index and manifest bytes are also
verified. Source/metadata paths refuse links, preparation never overwrites an
existing/raced output, and failure removes only its newly claimed directory.
The overlay uses `bundle.resources`, not `externalBin`:

- `agent-supplier/codex-<target>` contains the original pinned executable;
- `agent-supplier/pins.json` contains the committed pin document;
- `THIRD_PARTY_NOTICES.md` and `third-party/codex-0.160.0-{LICENSE,NOTICE}.txt`
  retain the source index's relative links and exact upstream notice bytes.

The installed CLI is pinned at Tauri 2.11.1 in package-lock.json. Its
[app bundler](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.11.1/crates/tauri-bundler/src/bundle/macos/app.rs#L97)
copies ordinary resources separately from the external binaries added to its
signing targets. Its
[signer](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.11.1/crates/tauri-macos-sign/src/keychain.rs#L207)
does not pass `--deep`. This is the reason to use ordinary resources: preserve
upstream code/signature bytes, rather than re-signing a sidecar and invalidating
the original digest. Source inspection is not actual signed-bundle proof.

The canonical `tools/release/build_signed_app.py` now requires an explicit
`--supplier-archive`, `--supplier-binary` or `--supplier-download` when executed.
It fixes `--target aarch64-apple-darwin` to the existing release baseline,
prepares the private overlay, retains it through the original build's completion,
and verifies the actual app resource bytes before returning success. Identity
count/refusal, identity-kind signing selection, redaction and the separate
notarization route remain intact. Dry run or missing source has no Keychain or
build effect. Source paths and private overlay locations are masked in forwarded
output. Full redistribution and release checks are still separate duties.

At runtime `supplier.rs` embeds the committed pin document, rather than trusting
an editable bundled manifest. The native caller supplies its app resource
directory; neither model/webview input nor settings supplies an executable path.
Only the process's macOS architecture selects a pin. After the nominal 4 MiB
GENERAL startup admission, it opens the fixed named resource, refuses links,
nonregular/group-or-other-writable input, checks size and thin Mach-O CPU/type,
and retains that original input File through copying. The hash covers the actual
copied stream; changed/growing input refuses and removes the partial output.
An existing destination is preserved. The fresh private copy receives execute
permission only after exact digest/size and flush succeed. The source and
Runtime share the same admitted backing through copy and original group cleanup;
no worker or additional startup allowance is introduced. Parent directories are
native/caller-owned; malicious same-uid parent replacement is outside this seam.

The launcher now accepts `LaunchConfig.resource_dir` and refuses unavailable
resources before creating a runtime marker. Its profile, single runtime exec,
fork denial and process topology are unchanged. The ignored startup probe accepts
`SWBPIPE_CODEX_RESOURCES` pointing to a selected app's Resources directory; it no
longer selects a developer/cache binary. This does not expose a Tauri command,
start control, panel or unsandboxed fallback, and selection is not native Intel
qualification. Windows/Linux isolation and App's held GUI sessions are unchanged.

Focused tests cover both pin selections, invented source paths, admission/backing
lifetime, source/destination links, wrong size/CPU/type, source replacement or
mutation, partial cleanup, no overwrite, exact resource/license layout and
canonical-build failure standing. A preexisting stdio custody test's occasional
`Busy` was an observation race: EOF precedes final worker bookkeeping. It now
joins both finite workers before draining the frame and asserts actual Joined
standing; frame identity and lease-lifetime expectations remain unchanged.

Independent permissions/packaging review found one build-output failure path:
forwarding failure could stop pipe draining before an unconditional wait. The
repair discards remaining output before waiting, retaining private resources
through original child completion. A real self-bounded synthetic child writes
more than pipe capacity; the regression fails on the reviewed baseline and
passes after repair without signing, supplier execution or saved-PID signals.
Focused evidence is 130 agent passes/five native gates ignored and 55 Python
passes/19 subtests, plus current-source reach and structural checks. The one new
consumer condition adds no new strongly connected component to the existing
graph. No ScopeOfWork/AC or release-matrix change is made.

Actual bundle/copy/signature evidence is pending. The planned automated artifact check is an offline ARM debug bundle with the explicit
original cache input, no signing identity, supplier/model/GUI execution or
acquisition. It verifies all actual resource bytes and then the admitted Rust
copy path. Any ad-hoc seal is local development evidence, not owner Developer ID,
notarization, supported-platform acceptance or release.

DEL-10-04 now names DEL-16-06's verified supplier/notice inputs as a gating
consumer condition for this build route. It is not marked fulfilled by a merge
or source test. Existing needs and both ScopeOfWork commitments/ACs remain
unchanged; source/check pointers and BUILD_AND_RELEASE.md describe the new
explicit preparation and post-build check. Release and matrix decisions remain
owner acts.

## Exact upstream evidence

Official release API:
<https://api.github.com/repos/openai/codex/releases/tags/rust-v0.160.0>.
Observed tag `rust-v0.160.0`, target commit
`a956835d020762cb2b570053af06f643a11c0ecc`, published
`2026-10-01T20:19:13Z`. These are stock **codex** release assets, not the separate
app-server-only or provisioned-package assets. Version identity here comes from
release/tag/asset metadata and the earlier named ARM witness, not a new supplier
`--version` invocation.

| Rust target | Release archive bytes | Release archive SHA-256 |
| --- | ---: | --- |
| aarch64-apple-darwin | 95893648 | `07c3c7ca376a8f791115342f53138dda37e97cfa29b8125d0652d93784894b5d` |
| x86_64-apple-darwin | 104691669 | `a50c10606e4e81b8dd2f7b6bab635595aabc773c84cf2071773fbaf067825fbf` |

| Executable | Bytes | SHA-256 |
| --- | ---: | --- |
| codex-aarch64-apple-darwin | 241555024 | `112fae7a5a1223e673c8a1791d32338f37df8b527ff1159bb8adac6c4dbf1b4b` |
| codex-x86_64-apple-darwin | 260633072 | `5383ef71dd1bd8d2f3658c04a219e2cf165c7969aebd0cceced1bc9f0f68877f` |

Exact URLs are in the manifest. Both archive sizes/digests match the official
release API's published asset values and local streamed hashing. The ARM
archive and executable were reused read-only from the existing witness cache;
no ARM download occurred. Intel was downloaded once from its official public
HTTPS release URL into a new private scratch locality. Each archive lists only
its correspondingly named executable. Both were then successfully acquired
through the new tool using explicit local archives and published in scratch;
no product resource binary was added to source control.

Official npm metadata was also read directly over public HTTPS, without npm
CLI/auth configuration:

- <https://registry.npmjs.org/@openai/codex/0.160.0-darwin-arm64>:
  `@openai/codex`, `0.160.0-darwin-arm64`, OS `darwin`, CPU `arm64`, licence
  `Apache-2.0`, official registry tarball.
- <https://registry.npmjs.org/@openai/codex/0.160.0-darwin-x64>:
  `@openai/codex`, `0.160.0-darwin-x64`, OS `darwin`, CPU `x64`, licence
  `Apache-2.0`, official registry tarball.

The selected acquisition inputs are the smaller GitHub single-executable
release archives. npm archive bytes were not downloaded or independently
rehash-verified here. npm integrity signatures/attestations were not verified.

## Static minimum OS, signature and licensing limits

Installed `/usr/bin/vtool -show-build` and `file` inspected the original bytes:

| Architecture | Mach-O minimum command | Minimum | SDK |
| --- | --- | --- | --- |
| arm64 | LC_BUILD_VERSION, platform MACOS | 11.0 | 15.5 |
| x86_64 | LC_VERSION_MIN_MACOSX | 10.12 | 15.5 |

These are static binary load-command floors, not proved native compatibility,
an app minimum-OS setting or an expanded support promise. The desktop config
does not set `bundle.macOS.minimumSystemVersion`; its resolved packaging
minimum was not built/checked. Release target RT-V7 still records macOS Apple
Silicon as the release baseline. Pinning Intel does not change that baseline.
The manager/Agent 0 must reconcile any intended Intel support and app minimum
against product/runtime/dependency requirements; no support range was selected
or changed by this task.

`codesign -dvv` displayed both originals as thin `codex` executables with
hardened-runtime flag `0x10000`, team `2DC432GLL2`, authority
`Developer ID Application: OpenAI OpCo, LLC (2DC432GLL2)`, Apple certificate
chain and runtime version 15.5.0. Displayed timestamps were Oct 1, 2026,
11:49:46 AM (ARM) and 11:50:08 AM (Intel), in the inspection host's display
timezone. The initial task performed signature **display** only. Manager integration then
ran `codesign --verify --strict --verbose=2` read-only against both exact cached
original executables; both returned zero. This verifies the original code's
static signatures on this host. It is not Gatekeeper assessment, notarization
verification, distribution-chain/older-OS qualification or proof of eventual
nested-resource signing; those broader manifest conditions remain unproved.

The exact commit's root LICENSE and NOTICE were fetched from
`raw.githubusercontent.com/openai/codex/<commit>/LICENSE` and `/NOTICE`.
The licence is Apache-2.0 (10926 bytes, SHA-256
`d17f227e4df5da1600391338865ce0f3055211760a36688f816941d58232d8dc`).
NOTICE is 242 bytes, SHA-256
`9d71575ecfd9a843fc1677b0efb08053c6ba9fd686a0de1a6f5382fd3c220915`,
including upstream's OpenAI and Ratatui attributions. Both are reproduced
byte-for-byte under `docs/third-party/codex-0.160.0-*`, indexed by
`docs/THIRD_PARTY_NOTICES.md` and the manifest. No copyright wording was invented.
Their inclusion in the final app distribution and completeness of binary
dependency notices remain for independent redistribution scrutiny; this task
makes no legal sufficiency determination.

## Checks and reconciliation for manager

`PYTHONDONTWRITEBYTECODE=1 python3 tools/agent_supplier_resource_test.py`:
17 offline invented-data tests pass, including committed dual-architecture
manifest admission and exact licence/notice byte hashes. They exercise correct publication and
mode, private staging/cleanup, hash/size/truncation rejection, architecture and
file kind, missing/extra/duplicate/unsafe members, gzip corruption and expansion
bound, existing/raced output preservation, source/destination links, explicit
source choice, mocked transfer bounds/errors/deadline/proxy exclusion, redirect
rejection and manifest validation. No real supplier/model is used in tests.
The real ARM and Intel local-archive acquisition commands additionally passed
the exact size/hash and Mach-O CPU checks without supplier execution.
The seven new working files were screened using the repository credential
finding function and private-term patterns derived from the inspection host
at runtime: zero findings. No private terms file was opened. Manager integration's committed 13-path
private-term and credential/link scans pass with zero findings. The actual
hosted pytest entrypoint passes 31 tests and 19 subtests (17 acquisition tests
plus 14 Python-routing checks); each of the six source/manifest/notice input
paths selects the offline check, including hosted Python selection. Reproduced licence/notice sizes and
hashes were checked against the manifest. Independent review found inherited
TLS environment handling in the default HTTPS context. The repair's two
regressions exercise real TLS construction/opener wiring without network:
individual/combined key-log and CA-path overrides, enabled verification, and
fail-closed missing compiled trust. Both reject the reviewed baseline; no
session keys or supplier execution are needed for the test.

Unproved: full build-time HTTPS path through this tool (the Intel acquisition
used a separate public HTTPS fetch), Tauri resource wiring, nested signing/notarization,
distribution notices completeness, actual bundle verification on either Mac
architecture, Intel/older-OS native compatibility, runtime resource selection,
isolation/cleanup qualification and integrated session evidence. No GUI,
Cargo/native build, provider, keychain, model or service was exercised. Windows
and Linux remain unavailable for unqualified isolation; no VM route was opened.

ScopeOfWork OUT-005/CLM-005/AC-005/VER-005 and existing dependency conditions
are not discharged by this source-only slice. The manager registered the explicit offline check in software-workflow.json
and the hosted Python input selection, added the source/check/resource pointers
in deliverable.yaml, and reconciled portability text's previously missing Intel
metadata. ScopeOfWork commitments, ACs and dependency conditions are unchanged;
no Decisions file was changed. Launcher
exposure still requires the governing isolation/cleanup evidence; neither an
archive pin nor a passing offline test supplies it.
