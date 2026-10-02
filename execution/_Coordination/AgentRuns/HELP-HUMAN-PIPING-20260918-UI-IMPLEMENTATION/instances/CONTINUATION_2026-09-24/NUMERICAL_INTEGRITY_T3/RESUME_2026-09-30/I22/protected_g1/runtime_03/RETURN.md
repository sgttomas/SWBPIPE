# G1 runtime_03 stopped — wrong G05 postimage and launch-gate breach

**G02, G03 and G04 qualified at their reviewed semantic assertions; all three untouched-baseline returns passed. G05 is not a kill.** I22 transcribed G05's frozen hunk without enough context to distinguish two identical locations. apply_patch inserted the extra context record at original line1424, not the intended solve_case_at location1873. G05_ACTUAL.diff preserves the actual change.

The displayed postimage was920d178e7dd604819fc2b7d4dd71f4b6cef6ed744268f9c63300045a8ff369c0, which did not match the frozen expecteda7a6f4060a1d1f8d06fe91cc002a5a26e8c20038b0f3612d359c77b2f8cb092a. I22 failed to make that check an advance gate and launched Cargo in the same tool cell. This is an executor provenance/ordering error, not a defect in the frozen patch or maintained production.

Cargo launched13:18:28.352Z, PTY51662. Upon recognizing the mismatch, I22 sent Ctrl-C through the supported tool. The process exited1 by13:18:36; time-l reported abnormal termination after7.47s during compilation. stdout is empty and no test executed. A subsequent process inspection found no Cargo/rustc/G05 process. No owned job remains.

No correction, retry, alternate discriminator or later group ran. The wrong G05 source and partial target are preserved; neither was reverted. Fifty mutant filters have no result (G05 plus49 later groups); fifty return controls are UNRUN. No G05 alias is credited.

## Qualified results

| Group / registration | Exact semantic observation | Return control |
|---|---|---|
| G02 / R7_K4::K4-M34 | bound_tests.rs100 F2-m110-p128 Uc tuple: U, t and Uc differ from upward emulation, N_L unchanged | PASS |
| G03 / R7_K4::K4-M35 | scale_tests.rs571 direct shifted-factor/factor entry parity: SKEW-K1E-28,p256,(8,3), -800…001p1 versus -8p1; not a pinned-digest-only failure | PASS |
| G04 / R7_K4::K4-M24 | method_tests.rs269 specifically CEIL5A3: required512 stop_rule:37 lost and replaced by publication_enclosure:37:AbsoluteBound;128 Pivot/256 verification_estimate:37 unchanged | PASS |

G04 credit is for the missing required named combination R7 verdict, as reviewed. It is not credit for a generic certificate failure or overall Ceiling. Three physical experiments credit three distinct registrations; no duplicate independent kills. ROOT's separate G01 result remains separately attributed and was not repeated.

## Log correction and preserved evidence

ROOT log-path addendumfd639cc62eddba9060a2eed1792f36cdc1ab7659 was read. A separate mkdir successfully created the writable sibling <G1_S>/runtime_03/logs; manager acknowledged continuation13:14:59. G02 was already patched and was not reapplied. Its launch13:15:26.986Z/PTy23616 used the correct source hash. All logs in this segment are outside GNN archives; no permissions/TMPDIR/tool/environment repairs occurred.

The original release acknowledgement13:10:19 and fixed14:10:19 deadline remained unchanged. Each actual Cargo/test command used installed1.97.1, offline/locked/-j4, incremental0, two test threads, cleared fault/Rust flags/wrappers, existing guard5387, exact owned target, tool-managed PTY and time-l. One process sequence ran at a time; no five-minute checkpoint was reached.

COMMANDS.json preserves every apply_patch input/result, command, launch/completion, raw-tool termination and semantic review. Per-run raw stdout/portable stderr, test/source/lock/binary/fingerprint identities are under mutant/ and control/. G05 raw stderr SHA256bed8c1d6cd0932fbcf7d2e08cdb02f7cf82e454c82d2ed9d2b15aeb8c4216212; stdout empty. Actual raw locations remain <G1_S>/runtime_03/logs/mutant/G05.stdout and G05.stderr.

The six-entry readiness,90-entry runtime_01 and four-entry runtime_02 seals verified unchanged. Maintained source/tests/criteria, frozen patches, inputs/truth, observations and Git/index were not edited by I22. No new framework, child, V-K, baseline batch or G01 repeat. ROOT must disposition this execution error before any new continuation. SHA256SUMS excludes itself.

