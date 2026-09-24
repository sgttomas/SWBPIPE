# APT index acquisition: bounded hosted trial

Basis: `a957e0a23e27807bd6ade21c723675bd5e9d7cec`. ROOT owns Git integration,
source-gate refresh and the actual hosted trial. This candidate is not yet a
proven repair. The previous numerical suite's 78 commands, 38 crates and 1082
passing tests belong to the prior recorded candidate and are separate evidence.

## Observed failure and causal boundary

Two attempts of the same PR887 candidate failed during Playwright system
provisioning, before browser tests. The actual action already wrote HTTPS
Ubuntu sources with the archive keyring. Attempt 1 received 46392 bytes for
security Components where Release metadata expected 46384. Attempt 2 received
1698176 bytes for updates universe Packages where 1698296 were expected. PR886
independently encountered updates and backports Components mismatches. APT
correctly rejected these responses; this is not evidence of a product test
failure or of disabled integrity checking.

The earliest observed divergence is between a fetched Release generation and
an index served under its mutable filename. The logs do not expose the prior
effective by-hash settings or acquisition debug, so they cannot distinguish
an ambient disable from missing immutable objects followed by APT fallback.
Mirror synchronization, distributed origin inconsistency and intermediary
caching remain possible contributors. HTTPS alone does not bind two requests
to the same publication generation.

Raw excerpts and their original-log hashes are in `_run_records/`. They remain
unchanged. `custody.json` records their origin and the HTTP response-body map.

## Primary basis and bounded proof

Ubuntu Noble documents `yes`, `no` and `force` for source-level By-Hash. The
forced option requests hash-addressed indexes even when support is not
advertised. The default remains conditional on support. See the
[Noble sources manual](https://manpages.ubuntu.com/manpages/noble/man5/sources.list.5.html)
and [APT configuration manual](https://manpages.ubuntu.com/manpages/noble/man5/apt.conf.5.html).

Ubuntu's [AptByHash design](https://wiki.ubuntu.com/AptByHash) explains the
Release/index publication race and the remaining risk when requests reach
inconsistently synchronized mirrors. That page was consulted through web
retrieval; a subsequent direct request returned 404, recorded separately.

The [APT 2.8.3 acquisition source](https://salsa.debian.org/apt-team/apt/-/blob/2.8.3/apt-pkg/acquire-item.cc)
retains the stable URI as an alternative even when by-hash acquisition is
forced. This candidate must not be described as disabling that fallback.

The [HTTP transport manual](https://manpages.ubuntu.com/manpages/noble/man1/apt-transport-http.1.html)
describes request cache controls. The exact
[runner-image APT recipe](https://github.com/actions/runner-images/blob/ubuntu24/20260920.314/images/ubuntu/scripts/build/configure-apt.sh)
already sets HTTP and HTTPS No-Cache. The published recipe is not a dump of the
failed job's effective runtime configuration; adding duplicate cache settings
is not justified by the available evidence.

Read-only HTTPS probes reproduced an index-generation mismatch without
installing APT or changing the Mac host. In the paired updates observation,
Release metadata expected 1698176 bytes and SHA-256
`298d6aac769c3cb7cdaa5199052c12f2c44ded50f903ae1fedcf73c69f0adae9`.
The hash-addressed request returned exactly those bytes and digest. The stable
filename, also requested with no-cache headers, returned 1698296 bytes and
`5914f242dd289eb47e4e592db25970499a713b7624661ec76f67a8ea5f896ff5`.
The paired security hash-addressed response also matched its Release metadata.
An earlier request for the security hash from attempt 1 returned 404. These
observations support immutable addressing, while also demonstrating why it
cannot guarantee success when an object is unavailable.

`_run_records/immutable-index-proof.json` records the exact comparisons.
Response headers, timestamps, URLs and retained content-addressed index/Release
bodies are under `_run_records/http/`. Primary documentation URLs and observed
digests are recorded without duplicating whole manuals. This probe checked
payload identity; it did not independently verify InRelease signatures or
execute APT. The hosted APT trust checks remain required.

## Selected candidate and verification

The existing setup action now requests `By-Hash: force` in both Ubuntu stanzas.
It logs the rendered known source file, APT version, global and Packages/DEP-11
by-hash settings, and HTTP/HTTPS No-Cache values. Unset values are explicitly
reported as omitted; inherited defaults can still apply. No proxy credentials
or unrestricted configuration dump is emitted.

All suites, components, keyring binding, signature/hash/expiry checks, existing
Playwright cache branches, coverage and timeouts are preserved. The change
applies to the existing source workflow and Piping cache workflow that consume
this action; the separate Root/Runtime/App workflows and exporter are unchanged.
Three lightweight tests passed: rendering the actual expandable source heredoc,
checking shell syntax and diagnostic scope, and retaining both Playwright cache
paths. See `_run_records/apt-tests-final.log`. No local Cargo, npm, browser,
Docker, VM, APT install or hosted run was performed for this repair.

Fresh TASK implementation supplied the first action/test delta. When a followup
was refused by the global agent limit, the manager integrated the bounded
rendered-source, per-target and version diagnostics directly. Attribution and
instruction hashes are retained in `_run_records/manager-basis.json` and
`_run_records/apt-implementation-basis.json`.

## Acceptance boundary

This is the smallest supported configuration trial: it overrides an ambient
by-hash disable and makes the next job's state inspectable. If the prior
setting was already enabled and a required immutable object is missing, it may
not resolve the failure. ROOT explicitly authorized this trial rather than a
third unchanged rerun. Fresh independent review and the hosted outcome remain
required. No failed tests are retried and no APT authentication check is waived.
A stronger acquisition or bounded generation-refresh strategy requires the
next observed failure evidence and a separate justified change.
