# V-K runtime_01 — stopped at P09; no fault credit

Actual block: 2026-10-01 17:53:50 UTC; fixed end 18:38:50 UTC.
TASK I22, native child of WORKING_ITEMS recovery manager. ROOT grant and manager releases are bound in BINDINGS.json and RELEASES.json.

Both separately released builds passed exactly once: debug test harnesses (4.83 s real) and normal release vk_records (12.79 s real). Cargo profile, features, compiler, physical binary hashes and original raw fingerprints are preserved. The normal example is test=false/opt3 with seeded-faults.
Nine fresh direct processes ran in frozen order. P01–P08 each selected exactly one test and passed, including numerical/classification, stored-record, prescribed InputDerived-exception and sparse/dense parity assertions.

P09 was the first fault process: VK-F01, lane rf_chain, launched 18:11:33.951Z, PTY3859, exit101, 10.02 s real. All30 named RF-CHAIN cases reported `not selected: Unresolved Ceiling [Restrained]`; lane.rs75 failed with30 failures. P03 had selected all30. The failed lane path emits no per-attempt R7/certificate reason. The frozen F01 mapping permits named numerical predicate failure or typed numerical unavailability arising from stored assembly, while excluding generic certificate drift. This output alone cannot distinguish that required cause from the excluded path.
P09 is STOP_UNQUALIFIED, with NO KILL credit. No cause is inferred from the seeded fault name. Manager confirmed the stop; ROOT disposition is required before additional runtime.

P10–P53 are UNRUN, including P09's returned NONE control, all later faults, UNKNOWN and every example triplet. No repeat, repair, alternative command, source/test/criterion/observation edit, new collector/driver or broad run occurred.

Raw scratch streams remain at `<wt>/scratch/i23/vk_prep/logs/{TESTS_DEBUG,RECORDS_RELEASE_NORMAL,P01..P09}.{stdout,stderr}`. Portable full stream copies and exact raw hashes are in each evidence directory. VERDICTS.json records all53 planned IDs, actual commands, exits and semantic reviews; BUILD_SUMMARY.json contains both actual build command/tool records.
Tool-managed PTYs and /usr/bin/time -l supplied supervision/resource evidence. No command reached five minutes; no automatic deadline or per-process RSS cap is claimed.

Final check at18:12:54 UTC verified all189 source files/modes and four binaries/raw fingerprints unchanged, dependency fingerprints unchanged and guard5387 alive. No owned process remains. Writes comprise this additive evidence, the granted build target and eleven raw command stream pairs; no maintained source, Git/index, previous seal or observation was modified. No target-wide inventory was generated. SHA256SUMS covers this packet and excludes itself.
An evidence-construction JavaScript syntax error occurred before any host call; no file or runtime action resulted. The corrected evidence-only call followed, without repeating any build or test.

This stopped partial replay establishes no A1, E_max, W1 or F2a acceptance.
