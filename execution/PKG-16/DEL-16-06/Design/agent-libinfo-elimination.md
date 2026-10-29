# Libinfo elimination investigation

**Standing: source/manual characterization, one refused synthetic-identity
diagnostic and accepted exact-libinfo config admission; no caller-capture run.**
The single allow is fixture-qualified and now emitted by the unwired host
profile source. There is no production activation, risk acceptance, or proof
of necessity. The focused hash/delta guard binds this source to the reviewed
and executed profile bytes.

## Source basis

The official `rust-v0.160.0` tag resolves through annotated tag
`79b1b666f2e8551f8abbbca34957227f67f3f553` to Codex source commit
`a956835d020762cb2b570053af06f643a11c0ecc`. The previously acquired official
binary remains pinned by its existing size/hash; reading release-tag source
does not establish a reproducible-build correspondence.

Apple Libinfo source is pinned to
`39b70c515baee5b609e7e91693edbd934b6845a1`. Published CoreFoundation source is
pinned to `dc54c6bb1c1e5e0b9486c1d26dd5bef110b20bf3`; that historical source
is explanatory evidence, not proof of the installed private implementation.
Installed `opendirectoryd(8)`, `dsconfigad(8)` and `dtrace(1)` were consulted.
No upstream dependency, model or supplier executable was installed or run.

## Callers, methods and elimination limits

| Entry path | Source-established dispatch | What configuration changes can establish |
| --- | --- | --- |
| Codex shell discovery | [`get_user_shell_path`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/shell-command/src/shell_detect.rs#L66) calls `getpwuid_r(getuid())`; the default/fallback shell search can invoke it repeatedly. Libinfo `getpwuid_r` selects `si_user_byuid`, whose directory module intends procedure `getpwuid` with a numeric uid payload. | That function does not consult `HOME`, `USER`, `LOGNAME` or `SHELL` before its lookup. Disabling shell execution does not remove this discovery code. The session's internal shell override is not established as a supported external `thread/start` control. |
| Codex configuration load | [`thread_start_task`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/app-server/src/request_processors/thread_processor.rs#L1339) loads config before session creation. The [macOS requirements path](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/config/src/loader/application.rs#L58) calls `CFPreferencesAppSynchronize`; its false return becomes an error in [`synchronize_managed_preferences`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/config/src/loader/macos.rs#L136). | This is a concrete potential configuration-failure path, not an observed attribution. Managed settings must not be bypassed to silence it. Shell discovery occurs later in session construction, so its passwd call is not established as the cause of the prior configuration refusal. |
| CoreFoundation user/home helpers | Published [`CFPlatform.c`](https://github.com/apple-oss-distributions/CF/blob/dc54c6bb1c1e5e0b9486c1d26dd5bef110b20bf3/CFPlatform.c#L252) calls `getpwuid` before falling back to `USER`. Home helpers call `getpwuid`, or `getpwnam` for an explicit username, before the home-selection helper considers `CFFIXED_USER_HOME`/`HOME`. Intended directory procedures are `getpwuid` and `getpwnam`. | A synthetic `USER`/`LOGNAME` may provide fallback identity after a refused lookup; it does not prove that the syscall or broker dependency is eliminated. `CFFIXED_USER_HOME` alone does not eliminate the preceding lookup in this source. Current private CFPreferences call stacks remain unobserved. |
| Codex home selection | [`find_codex_home`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/utils/home-dir/src/lib.rs#L14) uses an explicit existing `CODEX_HOME` before the default home path. | The fixture already supplies private `CODEX_HOME` and `HOME`. It does not need an account lookup to choose that explicit Codex-home path. This does not remove unrelated library lookups. |
| Host/service resolution | Libinfo [`si_addrinfo`](https://github.com/apple-oss-distributions/Libinfo/blob/39b70c515baee5b609e7e91693edbd934b6845a1/lookup.subproj/si_getaddrinfo.c#L1554) recognizes numeric host/port and has a numeric-result branch. Named services can reach procedure `getservbyname` with name/protocol payload. The inspected directory module has no host-by-name/address handlers; do not label a hostname query as a proved `getaddrinfo` RPC to this service. | The fixture already uses `127.0.0.1:8000` with numeric port and no provider service name. Network resolution is not established as the source of its eight libinfo denials. No provider connection is needed to collect the already-failing no-tools config path. |

The eight observed events were denied service lookups, **not eight admitted
directory RPCs**. In the inspected Libinfo [directory module](https://github.com/apple-oss-distributions/Libinfo/blob/39b70c515baee5b609e7e91693edbd934b6845a1/lookup.subproj/ds_module.c#L1650),
`ds_user_byuid`/`ds_user_byname` check `_od_running` before building their query.
`_od_xpc_pipe` creates the named pipe; only a successful pipe can reach
`_od_rpc_call`, which submits the procedure name and version-2 payload.
An unsuccessful service lookup does not establish a completed method call.
Source does not assign all eight historical events to exact runtime callers,
nor establish which denial caused `configuration_load_error`.

The Apple source also contains a build-environment `XBS_DISABLE_LIBINFO=YES`
branch that disables its directory pipe. This is a private implementation
detail, not a selected or qualified product setting. It is not used here.

## Broker authority and current-host observations

The intended user methods can return account name, uid/gid, account metadata,
home and shell fields; a returned passwd field is not evidence of a disclosed
password. Named-service lookup returns service/protocol/port metadata. The
same named endpoint also has group, netgroup, alias, network, protocol, RPC and
filesystem procedures in the published module. A Seatbelt global-name allow
does not restrict a hostile supplier to its normal `getpwuid` use or qualify
server-side authorization. The daemon's installed manual explicitly includes
local databases, Active Directory, LDAP and NIS. Client socket denial is not
a no-brokered-network guarantee.

Read-only local configuration queries, with values reduced before output,
observed: no Active Directory configuration from `dsconfigad -show -xml`;
one local authentication search node and zero contact-search nodes from the
two `CSPSearchPath` queries; zero LDAP nodes from the local LDAP node listing.
The NIS-node query was unavailable. No directory names, binding identifiers,
account records or raw diagnostics were retained. These observations do not
certify absence of every dormant/custom binding, future configuration change,
or brokered remote operation. No binding was changed or remote node explicitly
addressed.

## Diagnostic capability and next decision

A bounded DTrace capability check using only a terminating BEGIN action failed
with insufficient privileges; its diagnostic also mentioned SIP. It launched
no supplier and captured no process data. No sudo retry, protection change,
injected library, profile widening or substitute snapshot attribution was used.

Consequently the optional single plain-A supplier diagnostic has **not** been
spent: a run without a functioning caller observer would not answer the
requested attribution question. No oMLX, model, keychain or stock process ran
in this investigation. The profile remains plain A and the one-service proposal
remains on hold.

The lowest-authority remaining hypothesis is fixture-only synthetic
`USER`/`LOGNAME` alongside the existing private home/numeric endpoint, keeping
the broker lookup denied. It might remove a fallback-identity failure while
leaving harmless denied attempts; that is untested and is not caller proof.
Agent 0 receives the source findings and unavailable tracing capability before
any altered diagnostic route. No risk acceptance, privileged tracing or product
configuration change follows from this report. Exact eight-event attribution,
necessity/sufficiency, native cleanup and activation remain unqualified.

Agent 0 subsequently authorized a separate single synthetic-identity admission
diagnostic, without caller proof. It keeps plain A and supplies only fixed
invented `USER`/`LOGNAME`, `SHELL=/bin/sh`, and private `CFFIXED_USER_HOME`
alongside the existing private HOME/CODEX_HOME and numeric endpoint. It uses
an invented no-auth token, never a keychain read, and starts no provider/model.
The fixture rejects combination with fork permission, system import, dynamic
tools or turn/EOF modes. No XBS knob, managed override, tracer or protection
workaround is introduced.

If admitted, the fixture checks the stock response's resolved provider alias,
model, private cwd, approval policy and read-only/network-denied snapshot.
The pinned source propagates managed-sync failure before producing the loaded
configuration snapshot; the fixture does not catch or bypass that step.
Acceptance plus these checks is bounded config-load evidence, not a trace of
CoreFoundation or proof that denied lookup attempts disappeared. The optional
caller-capture run remains distinct and unspent. No result is asserted here
until the diagnostic completes.

## Synthetic-identity result

The single authorized run at `91dc080b62d3852c0a094e987339e5a685dea7b8`
used all four settings above, an invented no-auth token and the same pinned
stock binary. The normalized plain-A profile hash remained
`80936c838f26fef039bbb9733f714fb68414baaec97d03709a17ba397b794f66`.
`thread/start` was refused with `-32600 / configuration_load_error`; no loaded
config snapshot or thread admission was observed. Thus these settings are
insufficient, and there is no claim that the libinfo allow became unnecessary
or necessary. No system import, other allow, managed-settings bypass or product
edit was introduced.

The safe denial report contained seven opendirectoryd.libinfo lookup refusals,
one logd and one notification_center refusal; `/private/etc` content and
system-ancestor/outside-home metadata denials remained. Counts are bounded
observations, not a causal attribution or proof that one setting removed a
particular call. The original supplier exited with raw status 0; its observer
remained valid, no fork was reported, and the supplier scope was confirmed
stopped. The fixture app's cleanup acknowledgment remained unconfirmed, so its
runtime/marker were retained and overall case verdict stayed false.

The check ran with no oMLX/server, model inference, keychain read, tools, turn,
EOF/crash experiment or caller tracing. The caller-capture run remains unspent.
Focused Rust verification of the identity/snapshot checks passed; all 29 offline
Python controller checks passed. The result returns to Agent 0 for the already
reserved broker decision; the one-service proposal remains unselected and
activation remains gated.

## Fixture-only broker disposition

After the refused identity diagnostic, Agent 0 selected exactly the single
`com.apple.system.opendirectoryd.libinfo` Mach lookup allow for fixture
qualification only. The supplied decision basis is:

1. Agent 0 treats this as Apple's normative App Sandbox baseline, rather than
   a bespoke permission. Inspection directly establishes that the installed
   `system.sb` contains the lookup; universal platform/support claims are the
   disposition rationale, not a new native observation from this fixture.
2. The authorized model can only use the existing pinned Codex implementation;
   no new broker tool is exposed, exec remains single-allow and fork denied.
   This decision trusts that implementation for its calls. The Seatbelt rule
   itself does not restrict libinfo message methods or prove what compromised
   supplier code could send.
3. Agent 0 interprets the no-AD, local-auth-search, zero-LDAP and unavailable-NIS
   observations as local-only directory disclosure/no remote broker path for
   today's fixture disposition. Those observations remain a configuration
   snapshot, not an exhaustive proof about dormant/custom nodes or daemon
   network behavior; NIS was unavailable rather than proved absent.
4. The no-other-network requirement is unchanged. No daemon-network exception
   is accepted for production by this fixture decision.

A later AD, LDAP or NIS binding reopens this allow for review. This is not
production risk acceptance: activation remains gated and the owner decides
whether to accept that residual at activation. The earlier broker-capability
limits remain explicit; a source grant and passing direct socket probes cannot
prove a broader no-brokered-network guarantee.

The authorized next check is one exact-candidate C/Rust preflight followed by
one no-tools `thread/start` with returned config-snapshot validation. The old
and new profile texts remain exactly those reviewed at `26c5b24e59`.
After replacing the private runtime root consistently by `<private-runtime>`,
the candidate bytes are pinned by SHA-256
`be2fe1d54fcc5c60a00ed7d304bf2e4c4885d662d84bfe5a9c7fe6ff9c6f4d7c`.
No system import, synthetic identity settings, XBS knob or other permission
is combined with that candidate. Preflight substitutes the admitted tiny
probe executable at the same exact-exec path and uses an owned inert loopback
listener for the positive network control, not oMLX. Stock admission uses an
invented token with no keychain read or model. A new denial class, unexpected
signal or config-load failure returns to Agent 0 before any further change.
The product source update follows this qualification; it introduces no start
command, frontend route or activation wiring.

## Exact-libinfo qualification result

At `fa9416939ed648636d4ef314b89fc090ea1dd93b`, the C preflight returned EPERM
for fork, self-spawn and forbidden exec, with exit 0 and no terminating signal.
All 14 Rust filesystem/network checks passed, also with exit 0/no signal.
Both preflight and stock admission used the reviewed normalized candidate hash
above. No system import or synthetic-identity settings were combined with it.

Stock no-tools `thread/start` was accepted. Its returned loaded configuration
snapshot matched the configured provider alias and model, private cwd,
never-approval and read-only/network-denied policy. As established in the
source analysis, the config loader propagates managed preference sync failure
before that snapshot; the fixture added no bypass. This qualifies the tested
permission/config-admission combination, not necessity of the allow or other
suppliers/hosts/configurations.

The safe report contained newly observed denied system-device reads at
`/dev/autofs_nowait` and `/dev/dtracehelper`. These were stock file accesses,
not host tracing or permission grants. Their operation/coarse classes already
existed; the device prefixes were new and were returned to Agent 0 before any
product edit. Agent 0 disposition deliberately leaves both denied: dyld's
`dtracehelper` open registers USDT probes, and `autofs_nowait` is used by system
file APIs to avoid triggering automounts. Admission and the loaded snapshot
succeeded despite both denials. No additional allow or qualification step was
selected for them. Shell metadata, outside-home reads/metadata, system-ancestor
metadata, logd and notification_center remained denied. No new allow was added
to silence them.

There was no oMLX/server, model inference, real credential/keychain read, tools,
turn, caller capture or EOF/crash experiment. Supplier raw status 9 followed
deliberate original-group anchor cleanup; its observer stayed valid, no fork
was reported and the scope was confirmed stopped. App cleanup acknowledgment
remained unconfirmed, so its runtime/marker were retained and the overall case
verdict stayed false. This is not prompt-exit or native-cleanup qualification.
The focused exact-profile hash/delta test and 30 offline Python checks passed.
The authorized one-candidate qualification is spent; activation stays gated.
