# V-K runtime_02 — stopped at P13

Actual restart: 2026-10-01 18:48:13 UTC. Granted hard end:19:23:13 UTC. Four fresh direct processes ran in fixed order; zero builds/repeats. ROOT grant ac8610bf6cc966f194e8a486269b718f5787623f and manager18:49:23 release are preserved.

| Process | Fault | Exit | Real seconds | Verdict |
|---|---|---:|---:|---|
| P10 | NONE | 0 | 3.58 | Original debug F01 returned control: exactly1PASS |
| P11 | VK-F02 | 101 | 3.69 | Registered named numerical predicate failures |
| P12 | NONE | 0 | 3.57 | F02 returned control: exactly1PASS |
| P13 | VK-F03 | 101 | 1.50 | STOP_UNQUALIFIED: cause not exposed |

P10 and P12 each restore30 selected RF-CHAIN cases,2,760 rows/2,700 passes/60 structural zeros/zero failures,82 discriminating/38 non-discriminating controls and the exact stored records.

P11 selects all30 cases but fails176 named numerical predicates. The required r1e-10/12 T/tw and N/ext rows are present, including RF-CHAIN-T-n03-r1e-10 T.M1/tw.M1 and RF-CHAIN-A-n03-r1e-10 N.M1/ext.M1. The full failure list and unchanged frozen reference strings are preserved. This is the registered value witness with a passing returned control, not generic availability, certificate or work/record drift.

P13 launched18:51:15.541Z (PTY14553) and failed exactly one rf_chain test at tests/lane.rs75. All30 named cases report only `Unresolved Ceiling [Restrained]`, without per-attempt reasons or named numerical row failures. The frozen F03 criterion requires attributable loss-of-soft numerical behavior. These top-level labels cannot distinguish that route from excluded certificate or unrelated causes. P13 receives NO KILL credit. P14–P53, including its returned control and second F03 filter, are UNRUN. No diagnostic, alternative command, criterion change or continuation was attempted.

Original runtime01 and debug P09 remain sealed/unqualified; P10 now has its own passing debug return evidence. The separate F01 release diagnostic remains distinct and subject to independent disposition.

READINESS.json binds the exact unchanged189-file source manifest, four physical artifacts, original raw fingerprints, frozen schedule and all88 initially unused stream paths. Every launch rechecked its binary/fingerprint, exact argv/fault, unused streams and live guard. Complete raw streams remain in `<wt>/scratch/i23/vk_prep/logs/P10..P13.{stdout,stderr}`, with full portable copies and raw SHA256 hashes in each process directory. Tool-managed PTY and /usr/bin/time -l records retain actual exits and resources; no command reached five minutes, and no automatic deadline/RSS cap is claimed.

FINAL_CHECK.json at18:51:58 UTC confirms unchanged189 source bytes/modes, four binaries/raw fingerprints, both prior seals/all76 payloads, guard5387 alive and no owned processes. No maintained source/test/corpus/oracle/observation, earlier evidence, Git/index or target bytes were edited. Writes are this additive packet and eight previously assigned raw streams. No helper/collector/driver, child, build, cleanup or extra model was introduced. Exact integers/fingerprint JSON were not reserialized through JavaScript.

VERDICTS.json enumerates all44 granted process IDs with completed/unrun disposition. SHA256SUMS covers this packet and excludes itself. A1 slot released; ROOT disposition is required before further runtime. No A1, E_max, W1 or F2a acceptance follows.
