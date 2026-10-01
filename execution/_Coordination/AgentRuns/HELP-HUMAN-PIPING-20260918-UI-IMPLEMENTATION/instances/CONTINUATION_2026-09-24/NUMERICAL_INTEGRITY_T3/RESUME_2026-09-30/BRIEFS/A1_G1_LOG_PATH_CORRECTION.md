# G1 log-path correction — preserve source archive permissions

At13:12UTC G02's frozen patch applied and its expected postimage matched, but
the launch tried to create runtime_02_logs inside the deliberately read-only
G02 source archive. mkdir failed, and output redirection stopped before Cargo.
This is no numerical result. The failed native outputs/command are preserved
in runtime_02; its original evidence stays sealed.

ROOT inspected actual modes: G02 dr-xr-xr-x, owned parent protected_g1 and
existing runtime_01 evidence drwxr-xr-x. No host/tool configuration or privilege
repair is needed. Correct only the output root to
<wt>/scratch/i22/protected_g1/runtime_03/logs, outside all frozen GNN archives.
Create/check that evidence directory as a separate successful operation before
any dependent run. Do not combine mkdir with a later command that masks failure.
Do not change source/cwd/target/filter/patch or chmod any archive.

G02 is already patched; reverify exact hash and do not apply twice. Continue its
original semantic test/control, then G03–G54 under the existing source/guard/
compiler/stop rules and original14:10:19 boundary. No new tool/framework,
TMPDIR or permission changes, extra variants or automatic time extension.
This is a bounded correction of an evidence destination within the already
authorized owned scratch scope. Additive records are runtime_03 under the
existing I22/protected_g1 and manager/protected_g1 scopes.

