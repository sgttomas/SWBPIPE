# E1 stopped — PM03 compiler error, not a mutation kill

**STOP at PM03.** All14 exact baseline filters passed. PM01 and PM02 compiled and failed at their predeclared semantic assertions; each then passed its returned-to-untouched-baseline primary control. PM03 failed compilation before any test executed. No repair, alternate discriminator, PM03 control or later variant ran.

| Completed activity | Result |
|---|---|
|14 baseline filters |14 pass, exactly one test each |
| PM01 H-evidence bypass | Intended first p128 C17 D:0/Translation/AbsoluteBound rejection assertion failed |
| PM01 baseline control | Pass; original baseline binary unchanged |
| PM02 final-x discrepancy removal | First exact-sum equality failed with negative difference: required h/4 discrepancy became zero |
| PM02 baseline control | Pass; original baseline binary unchanged |
| PM03 epsilon*published-scale substitution | Compiler E0308; zero tests; NOT a kill |

There were19 Cargo invocations and18 executed tests. The remaining38 mutant-filter invocations and33 returned-baseline controls are UNRUN. All32 variants from PM04 through PM18b are UNRUN, individually listed in VERDICTS.json. PM16a/b/c have not run.

## Exact stop

PM03's frozen patch passes Wide::<4>::from_f64(scale) to add_wide_scaled inside generic certify_rows<const L>. Rust requires Wide<L> there:

    error[E0308]: ? operator has incompatible types
    adaptive.rs:3275:32
    expected Wide<L>, found Wide<4>

The PM03 filter was pc31_39_exact_relative_admission_positive_radii_and_partial_draft. It launched09:15:03.154Z (PTY65747), exited101 after4.21s real, max RSS706068480B. stdout is empty; no mutant test binary was produced. Full compiler/resource output is retained at mutant/PM03-00/stderr.txt and its original raw scratch hash is in EVIDENCE.json.

Frozen PM03 patch SHA256:180783dcddba7f73af5b12fe832cc704bf5ff1073c72dfffdaacc6239b89bfd8.
Its retained failed source SHA256:34dde1dfdebcd7cebaaa2a1965dad44d418dd49b66d126342f8c792a492e3bc8.
This is a prepared-patch type error, not a production/math finding or semantic survivor. ROOT must separately grant any exact correction/freeze and continuation. Nothing was corrected here.

## Basis, containment and runtime

Patch/source/test basis remains40129a225d73860ac2a53da9a2fa73869df668f3, numerical helpers dd1-identical. Records headb13a42ea3b56a492461e0c329ca75790b38818ba is not substituted for that basis. E's frozen manifest is93270b0f1fcd23e560bdf7ca2a829752fba7be601cd62d48aa0e54cdc03b4ced.

Actual E1 start09:06:30 UTC; boundary10:06:30 unchanged. ROOT grant9a8c6fe77f56f12916c440d52bc5351d47312b4a, expected SHA814f7f74375c141a7e2f490d0edc7cb0535c03a6e12d1b65071bada8fe1f7a25. Manager explicitly released containment09:08:42 before the first Rust command09:09:48.923Z.

CONTAINMENT_INVENTORY.json records36 independent roots,116 files each,4176 distinct inodes, no symlinks/hardlinks or maintained aliases. Manager independently verified them and36 empty exact target directories. The baseline is read-only; only PM01, PM02 and PM03 received their frozen patches. Mutated copies remain intact. Restoration meant returning to the untouched baseline, never reverting a mutant.

Every Rust command used time-l, tool-managed PTY, installed1.97.1, auto-install0, offline/locked/-j4, incremental0, two test threads and only <PM_TARGET>/base or its exact ID. Existing memguard5387 was observed before and after runs; one Cargo at a time. No five-minute command checkpoint was reached. No new supervisor, automatic deadline or process hard RSS cap is claimed. All sessions completed; no owned Rust/build/test/solver job is running. ROOT now owns the freed Cargo slot.

Baseline adaptive source4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9, testb04eb95536437274e400b85050bf082bf193587b5f0d2f78524fb78a0e699c0d and lock7e1d8a98a542f977cfcfe8b2b0bb27be24fb91fbcf8f0b39a2e9e235574a08b9 remained unchanged. Baseline binary SHA256:2c9ff111dd423d73fd6def78ceff60421b6570523c713dcfda7075811c02e970. Active features/rustflags were empty for executed test binaries. Per-run source/test/lock/binary/fingerprint hashes and full outputs remain at baseline/, mutant/ and control/.

COMMANDS.json preserves exact portable argv, launch/session/completion, guard observations and patch applications. VERDICTS.json is the concise machine-readable disposition; compiler failure has its own status. Source-copy inventory is the pre-application containment handoff, not a claim that the three mutated files still match baseline. Raw scratch keeps original machine-path logs/bindings; durable evidence uses placeholders.

No maintained source/test/lock/assertion/input/truth, old registry, observation, Git/index or prior seal was changed by I22. No broad suite, probe, new model, scale job or child. Old R7/A1/A2/V-K and broader qualification obligations remain separate. SHA256SUMS inventories this stopped packet and excludes itself.

