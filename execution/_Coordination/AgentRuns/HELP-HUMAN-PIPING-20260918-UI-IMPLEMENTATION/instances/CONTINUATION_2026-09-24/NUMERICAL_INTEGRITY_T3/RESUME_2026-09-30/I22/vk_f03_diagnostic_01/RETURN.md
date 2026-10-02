# F03 release diagnostic — complete observation, attribution remains open

Actual block: 2026-10-01 19:13:34–19:23:34 UTC. Exactly three fresh processes used the unchanged normal-release vk_records binary7c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e. No build or additional runtime occurred. Each process computed RF-CHAIN and RF-SKEW with all66 frozen show selectors, maximum10 members, no parity and no --write.

| Process | Fault | Launch UTC | Exit | Real seconds | Result |
|---|---|---|---:|---:|---|
| D01_NONE_BEFORE | NONE | 19:14:20.966 | 0 | 0.25 | All66 complete records and both fixed report sets match |
| D02_F03 | VK-F03 | 19:15:07.386 | 0 | 0.19 | Complete66-record observation; numerical causes exposed, contribution attribution open |
| D03_NONE_RETURN | NONE | 19:15:55.879 | 0 | 0.24 | All66 complete records restored exactly |

Ordinary Python standard JSON analysis traversed BOTH interleaved segments:30 CHAIN records/report then36 SKEW records/report. Both controls equal their complete frozen records, including schema, source IDs, work/storage and exact u64::MAX. They also equal each other. CHAIN pins:2,760 rows/2,700 passes/60 structural zeros/zero failures,82/38 controls. SKEW:1,080 rows/982 passes/96 structural zeros/2 not-covered/zero failures,105/33 controls. Both select all cases at128 and retain empty unexpected-control lists. No full record or fingerprint integer was reserialized through JavaScript.

Observed F03 sequences, with every case retained in CASE_LINKS.json and D02_F03/ATTEMPT_REASONS.json:
- All30 CHAIN cases fail Pivot at candidate128/256/512, with no verifier: globalDOF14 for10 n03 cases,26 for10 n05,59 for10 n10.
- SKEW's18 OFF cases fail Pivot at128/256/512: six T-PIN at3, six T-CANT at5, three A-CANT-345 at1, three A-CANT-122 at2.
- SKEW's18 AX cases reject each128/256/512 candidate with StopRule:12 torsion cases node0 Rx/body0/Rotation and6 axial cases node0 Ux/body0/Translation. Each ends with a solved1024 verification.
- All66 top-level outcomes are Ceiling. No publication-certificate, budget, source/geometry or terminal stop is emitted. SKEW additionally prints the two preserved FLOOR differences for T-CANT-AX-345/122-r1e-12 tw.M1 after loss of selection; these are not credited as separate value/class witnesses.

Source-linked facts: assemble.rs706–735 removes ONE minimum-absolute Wide contribution from EACH diagonal having multiple terms, then recomputes its exact sum. Contributions may come from members, global springs or directional springs; zero/tied minima are possible. source.rs29–76 maps each reported global DOF to node/component; factor.rs582–585 reports the failing ordered global pivot. CASE_LINKS.json binds all66 case/source identities, full NONE/F03 precision-role-reason sequences, actual reported DOF/body, original reference row where present, incident member IDs/nodes, springs at that node and historical NC-LOST-SOFT metadata. A Pivot location is not itself the identity or location of a deleted term.

Unresolved obligation: the existing records do not expose which contribution was omitted at each affected diagonal, its Wide value/sign, zero/tie status, or a case-specific causal path from that deletion to the original registered soft-root failure. The controlled process association and new numerical outcomes are established; the required contribution-to-soft-root attribution is not closed. No root-spring deletion is assumed, no private operand or prediction is invented, and F01's terminal exclusions are not imported. All66 cases remain visible; no favorable-case selection or enum-only credit. This packet therefore returns an ATTRIBUTION GAP for ROOT/RV29 disposition and claims no F03 kill.

D03 was expressly authorized after normal complete bound D02 even with attribution open. It restored every full record; no further process followed. Original debugP13 remains unqualified, and P14/P15/P16 plus all later original calls remain UNRUN. Release records do not retrospectively supply debug attempts or controls.

READINESS.json and FINAL_CHECK.json bind all189 source bytes/modes, original binary/profile/features/raw fingerprint/dependencies, exact proposal/review seals and live guard5387. Each launch rechecked complete source, artifact and guard. Final check19:18:07 verified prior runtime01/F01-diagnostic/runtime02 seals and all94 payloads unchanged; no owned process remains. Raw streams, exact commands/tool exits/resources, decoded records and both full reports are retained; raw SHA256 hashes point to the six files under `<wt>/scratch/i23/vk_prep/logs/f03_diagnostic_01`. No command approached five minutes; no automatic deadline or per-process RSS cap is claimed.

Writes are confined to this additive evidence and those six raw streams. No source/test/corpus/oracle/criterion/observation/Git/index/target modification, instrumentation, collector/driver/helper, child, cleanup or extra case occurred. SHA256SUMS excludes itself. A1 slot released; no A1, E_max, W1 or F2a acceptance is claimed.
