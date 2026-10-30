# Provider configuration and credential custody

Standing: explicit, synchronous macOS core with current-host synthetic native
credential qualification; commands and UI remain unwired. RT-E1 / exit criterion 17. This implements
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
base. Independently of SQLite, the encrypted native password data contains a
versioned canonical-endpoint binding followed by the secret. Load compares that
protected binding before constructing or returning a Secret; changing both SQL
endpoint columns cannot retarget it. Unbound legacy values are refused. Deletion
still uses only the exact account/ownership handle, so endpoint corruption cannot
prevent an authorized stale-item cleanup.

Only native `LoadedProvider` carries a Secret, paired with its original provider
and model/revision. It has no Serialize/Clone or payload-bearing Debug. Metadata
responses never contain a password, account or ownership marker. The actual
credential value never enters SQLite, files, argv, environment or diagnostics.
Native errors are allowlisted classes, not Security.framework error messages.
Only missing native items and the two authorized no-prompt denial statuses
return `ReenterKey`, with distinct `Missing` and `Denied` reasons. Other native
statuses (including backend, invalid-data and signature-system errors) remain
explicit credential errors; they do not masquerade as recoverable denials. No failure
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
witness. The current host is macOS 26; the post-review native fixture qualifies the
no-prompt outcome there. Native minimum-version qualification remains
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
Schema admission requires the exact two table definitions and their one implicit
primary-key index, application/version identifiers, and no additional persistent
or temporary objects. Reopen and every operation/phase validate this before
native effects; matching version fields alone do not admit triggers, views,
indices or changed constraints.
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
reported as an unapplied save that invites replay. Save/clear capture the actual
metadata inside the publication transaction, before commit. Post-commit cleanup
or read failure returns that committed outcome with `cleanupPending: null`
(unknown) and an explicit cleanup error, rather than a plain operation error or
a false zero. A completed cleanup confirms zero. Explicit recovery deletes
only pending/retired owned items, never an active credential. Failures retain
journal entries; repeating exact deletion after an already-deleted item is safe.
Unpublished pending intents block loading. A committed active key remains
usable when deletion of retired stale items fails: the active native payload
still binds its recipient independently. Further saves remain blocked until
cleanup resolves, preventing repeated orphan creation. Re-entry after an
ACL/signature loss creates a fresh item for the current build, then attempts
stale deletion by the exact owned handle. Failure remains visible in the
snapshot and durable journal; it never prompts or removes the active key.
The regression for failed stale deletion asserts that the fresh key works,
the old handle remains recoverable, and later cleanup removes only that handle.

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
neither real keychain access nor native signature policy. Five targeted review
regressions cover changing both mutable endpoint columns, a malicious trigger,
reopening an unexpected schema, and post-commit read failures after save and
clear. All five reject the pre-repair flow; the repaired persistence suite passes
21 tests. The native payload unit also rejects a mismatched recipient and an
unbound legacy password value.

The opt-in post-review qualification uses an owned synthetic item and its
private SQLite journal, a fresh copy of the test executable re-signed to a
different identity, and an original owned Child with a timeout/cleanup guard.
Agent 0 amended the original single-status expectation: only raw
`errSecInteractionNotAllowed` (-25308) or `errSecAuthFailed` (-25293) qualify
as the expected denial, both mapping to `ReenterKey { reason: Denied }`.
The original expectation assumed modern-path semantics. Pinned
[Globals.cpp:68–71](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/Globals.cpp#L68)
selects `nullCred` instead of `promptCred` for legacy no-UI item operations;
authentication failure is a fail-closed result, not evidence of a prompt.
The requirement remains: no prompt, deny, map to re-entry and recover.
The first denied load must finish in less than two seconds; the original owned
child has a twenty-second deadline. The external
`tools/agent_keychain_ui_monitor.swift` harness (thirty-second command deadline,
one-second process-probe deadline and one-second owned-group kill/reap bound) observes NSWorkspace GUI launch
notifications and samples SecurityAgent/authorization UI process identities,
then drains notifications for one second. It fails on a detected launch,
incomplete observation, timeout or failing child. It records only static
standing and sample counts, never arbitrary process text. This is combined
source/return-bound and observed-launch evidence; sampling alone would not
prove that no short-lived daemon could exist between samples.
The regular re-sign fixture preserves the executable basename while changing
its signing identifier, and must accept re-entry and delete the stale item by
ownership handle. A separate renamed-app case expects native
`errSecInvalidOwnerEdit` (-25244), retains the stale handle, loads the freshly
committed key despite retired cleanup debt, and finally uses the two original
fixture identities to clean only their exact journal handles. Pinned
[SecItem.cpp:2020–2060](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/SecItem.cpp#L2020)
extracts executable basenames;
[2173–2287](https://github.com/apple-oss-distributions/Security/blob/41b54a4894b8de0b7569a8695313e617c6f20f59/OSX/libsecurity_keychain/lib/SecItem.cpp#L2173)
compares them for legacy safe deletion. Renaming the first fixture introduced
that additional cause, so it was corrected without changing the product API.
The renamed-case recovery helper never broadens service-wide deletion. It never reads a
real provider key, changes a user's default/search list, enables prompting or
runs a supplier/model. Native failures retain the journal for recovery; no
native success is claimed beyond the specific post-review fixture below.

Current bounded evidence: five pre-repair regression failures; repaired agent
suite 144 passed with eight explicitly gated ignores; release-reach suite
30 passed; routing 65 tests/87 subtests; public export 3752 files with nine
expected and zero unexpected findings. Independent credentials/persistence
review, repair confirmation and owner-disposition confirmation inspected the
changed surfaces. The last confirmation found a monitor timeout defect; the
repair uses only original unreaped POSIX children/groups, bounded nonblocking
process-probe reads and bounded kill/reap. Two injected regressions (a child
ignoring TERM and a hung process probe) timed out on the former helper and
returned explicit failure within 1.35 seconds on the repair. No required
expectation was weakened to silence a failure.

On committed implementation `6968d3f108` (2026-10-29, macOS 26), both native
cases passed in 3.01 seconds total. Each observed raw -25293, mapped Denied,
loaded the fresh key and ended with an empty provider/credential journal.
The same-basename case removed its stale item during re-entry. The renamed
case observed raw -25244, retained its stale handle while using the fresh key,
and then cleaned both synthetic items using their exact original ownership
handles. The external monitor reported complete observation, 72 process
samples and no detected authorization UI launch; source-derived no-UI policy,
short denied-call bound and GUI launch notifications supplement sampling.
The two earlier retained synthetic journals were also recovered by their exact
owned handles, verified empty, and removed along with the other cleaned
fixture localities. No real provider key, model, supplier or GUI session ran.

This qualifies the current-host synthetic credential/re-sign recovery seam,
not a live macOS 11 witness, integrated command/worker route, agent session,
criterion 17, acceptance or release. ScopeOfWork commitments/ACs and all needs
remain unchanged; only implementation and check pointers are reconciled here.
