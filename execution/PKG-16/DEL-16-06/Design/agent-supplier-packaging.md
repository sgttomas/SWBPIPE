# Stock agent supplier build resources

Standing: RT-E1 source/metadata/tooling implementation, 2026-10-29. Offline check registration is integrated; independent packaging/permissions
scrutiny and actual bundle wiring remain required. No
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
without proxy environment or auth configuration. Downloads have a 30-second
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
timezone. This was signature **display**, not cryptographic validation,
Gatekeeper assessment, notarization verification or proof of the eventual
app's nested-resource signing. The manifest marks those unproved conditions.

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
15 offline invented-data tests pass, including committed dual-architecture
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
at runtime: zero findings. No private terms file was opened, and no commit/CI
scan is claimed for these unstaged files. Reproduced licence/notice sizes and
hashes were checked against the manifest.

Unproved: full build-time HTTPS path through this tool (the Intel acquisition
used a separate public HTTPS fetch), independent tool/permissions review,
Tauri resource wiring, nested signing/notarization,
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
