# Provider configuration and credential custody

Standing: explicit, synchronous macOS core; commands, UI and native credential
qualification remain unwired/unrun. RT-E1 / exit criterion 17. This implements
the provider-config/credential part of embedded-agent-portability.md without
changing ScopeOfWork commitments, ACs or dependency conditions. No provider
selection, keychain lookup or persistence happens on module/global startup.
The future coordinator supplies an app-owned private directory and admitted
blocking execution; this core must not run on a UI/protocol polling thread.

## Configuration and secret boundary

`provider_store.rs` stores one selected provider: canonical API-base URL, explicit
model identifier, a fresh revision and either explicit no-key mode or an opaque
credential reference. It applies the relay's existing URL policy (HTTP only for
literal loopback; otherwise HTTPS; no URL credentials/query/fragment/escape) and
bounded control-free model input. There is no developer provider/model default,
provider-specific Keychain service, inherited home/config/authentication file or
implicit credential mode. Keeping a key requires the identical canonical API
base; it cannot retarget an existing credential to another origin/prefix.

Only native `LoadedProvider` carries a Secret, paired with its original provider
and model/revision. It has no Serialize/Clone or payload-bearing Debug. Metadata
responses never contain a password, account or ownership marker. The actual
credential value never enters SQLite, files, argv, environment or diagnostics.
Native errors are allowlisted classes, not Security.framework error messages.
A load failure, including missing item, lock/interaction, ACL/code-signature
mismatch, unavailable backend or invalid stored data, returns an explicit
`ReenterKey` state and a static instruction to re-enter the key. No failure
prompts, changes credential mode, retries another store or substitutes a key.
Safe heap/CF/header copies are not a zeroization or OS crash-output guarantee.

## Native Keychain and no-prompt policy

`keychain_native.rs` is the one new narrowly pinned unsafe boundary. It calls
SDK-public `SecItemAdd`, `SecItemCopyMatching` and `SecItemDelete`; uses the
user's default login Keychain, fixed generic service
`com.chirality.swbpipe.provider-api-key`, non-synchronizing attributes and exact
account/ownership-marker queries; and never upserts. A fresh independent
192-bit account and ownership marker are persisted as nonsecret custody handles.
Creation collision cannot authorize replacing or deleting an unowned item:
cleanup/get must also match the native generic ownership attribute. It selects
no access group, Data Protection store, custom keychain file or alternative
store, and does not unlock a keychain.

Agent 0 explicitly approved **checked
`SecKeychainSetUserInteractionAllowed(false)` before every legacy credential
operation, never restored**. The per-query `kSecUseAuthenticationUIFail` stays
present as well for modern API handling. Relay TLS initialization applies the
same monotonic process policy before creating the published OS verifier.
Failure to apply it refuses the operation/start; it cannot fall back to UI.
Data Protection Keychain was not adopted because signing entitlements and
provisioning would work against the authorized portability seam.

### Source scope and minimum-version basis

Pinned Apple Security source commit
`41b54a4894b8de0b7569a8695313e617c6f20f59`, tag
`Security-59754.120.12`, establishes client/process scope:

- [SecKeychain.cpp:861](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/SecKeychain.cpp#L861)
  calls only `globals().setUserInteractionAllowed(state)`; its getter reads the
  same object. The setter does not write preferences or change a server/session.
- [Globals.h:53](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/Globals.h#L53)
  stores that value in the client object's `mUI` bool;
  [Globals.cpp:36](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/Globals.cpp#L36)
  declares its ModuleNexus and selects nonprompting item/unlock credentials when
  false. This is client-local policy, not a user- or login-session preference.
- [globalizer.h:58](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_utilities/lib/globalizer.h#L58)
  holds the module's static pointer; `ModuleNexus::make` allocates the object
  locally with `new`. It is not shared service state or an IPC setter.

The installed public SDK `SecKeychain.h` declares the process-policy APIs from
macOS 10.2 (deprecated 10.10); `SecItem.h` declares UI-fail from 10.11 (deprecated
11.0). These availability contracts precede the supplier's ARM deployment
minimum 11.0. Availability alone is not proof of legacy behavior: pinned
[SecItem.cpp:4137](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/SecItem.cpp#L4137)
routes explicit Keychain/search-list queries to the legacy implementation,
while [SecItem.c:1342](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/sec/Security/SecItem.c#L1342)
translates UI-fail into noninteractive authentication on the modern path.
Therefore this core does not rely on the deprecated query flag alone.
The three locally declared SDK constants absent from the published sys crate
are pinned exactly; no private API/string or global OS setting is used.

This is SDK/source compatibility evidence for the minimum, not a live macOS 11
witness. The current host is macOS 26; the post-review native fixture must verify
the exact no-prompt outcome there. Native minimum-version qualification remains
unrun; no support-range change or universal native guarantee is inferred.

### Other Keychain uses affected in this SWBPIPE process

The source inventory identifies only the new provider credential operations and
OS certificate trust through published rustls-platform-verifier 0.7.1. The
latter uses Security.framework SecCertificate/SecPolicy/SecTrust and user/system
trust settings. Any legacy client-Keychain work in those paths also loses
optional prompting; all trust failures still refuse the provider exchange.
Ordinary OS OCSP/CRL/AIA validation traffic remains allowed as dispositioned in
embedded-agent-relay.md. No trust-network-fetch policy is disabled.

No other first-party password-store, Keychain API or `security` subprocess use
exists in the SWBPIPE product source. Tauri/WebKit/Foundation are linked OS
frameworks; their private internal accesses are not enumerated from repository
source, and any legacy accesses in this process inherit the same no-prompt
policy. Current webview assets are local and the app supplies no webview HTTP
credential handler. There is no re-enabling path. App v4, developer tools,
packaging `codesign`, and historical Python witnesses are separate processes;
this setting does not change their policy or a user's other applications.

## SQLite and crash recovery

The trusted caller supplies a canonical private directory. The dedicated
`provider-settings.sqlite3` and OS lock file are private regular files; symlinks,
unknown schemas/versions and oversized data refuse, with no default overwrite.
SQLite uses synchronous FULL, DELETE journaling, foreign keys, bounded pages and
cache. Only metadata/custody handles are bound as SQL parameters. At most 16
credential records exist. A standard-library owned file lock spans SQLite
transactions and native calls, preventing other instances recovering a live
operation. Its original descriptor closes on unwind/exit; no saved PID or
process-global database lock is signal authority.

Every settings edit/clear requires the exact captured revision. A replacement
first commits a pending ownership intent, then, under exclusive operation and
SQLite write gates, checks the revision again and creates the new native item.
Only a successful native create permits publishing the new profile and retiring
the old reference atomically. A crash or uncertain native/metadata result leaves
that durable pending handle; recovery never replays a save. Creation never
upserts an existing account. The independent ownership marker prevents cleanup
of a colliding unowned native item, even after a duplicate-create error.

After commit, old owned items are removed by handle. Cleanup failure preserves
the actual saved metadata plus explicit pending-count/error standing; it is not
reported as an unapplied save that invites replay. Explicit recovery deletes
only pending/retired owned items, never an active credential. Failures retain
journal entries; repeating exact deletion after an already-deleted item is safe.
Loading is blocked until cleanup is resolved and never returns a different key.
Re-entry after an ACL/signature loss creates a fresh item for the current build,
then removes the stale owned item; failure remains visible and does not prompt.

## Admission and verification

Before opening native/storage resources the core reserves 8 MiB GENERAL store
state and 1 MiB native wrapper allowance; each operation admits 16 MiB before
file/SQL/credential effects. These are nominal requested allowances, not a hard
allocator/RSS or retained-solver pricing claim. This synchronous core creates
no workers. The future native command owner must separately admit, own and
observe its blocking worker and hold command/response backing; it must derive
paths/origin from the real application, never a webview/model path or item name.

Mocked-Keychain tests exercise re-entry, exact endpoint/revision binding,
nonmutation on invalid/stale input, no-key choice, replacement/clear, ownership
collision, durable crash/failed-commit recovery, cleanup failure, cross-instance
exclusion, corrupt schema/binding and file-secret/symlink refusal. They establish
neither real keychain access nor native signature policy.

The opt-in post-review qualification uses an owned synthetic item and its
private SQLite journal, a fresh copy of the test executable re-signed to a
different identity, and an original owned Child with a timeout/cleanup guard.
It must return exact `errSecInteractionNotAllowed` (-25308), map to ReenterKey,
accept re-entry and remove the old item by ownership handle. It never reads a
real provider key, changes a user's default/search list, enables prompting or
runs a supplier/model. Native failures retain the journal for recovery; no
native success is claimed until this fixture has actually run after scrutiny.
