# G1 stopped before first mutant — patch temporary-file I/O failure

**All28 exact baseline filters passed, one test each. No mutant test or return control ran.** G01/R7_K4::R7-M8 stopped at its required postimage check after the existing system patch command reported a temporary-file permission error. No registration receives a kill.

The command was the frozen /usr/bin/patch -p1 -F0 -t -i invocation, working in the independently verified G01 root. It returned exit0 and wrote this error to stdout (stderr empty):

    patching file 'projects/chirality-piping/core/solver/frame_kernel/src/structural/retained/verify.rs'
    Can't create '<HOST_TMP>/patchoqOhNQ13g6R', output is in '<HOST_TMP>/patchoqOhNQ13g6R': Permission denied

The mandatory postimage check caught that verify.rs was still the original:
31780a6a888decc64e2ebec2fcfbeff45c144a6f0d0169f0584d53fcccb89987,
rather than the frozen mutant:
053e9d2fe628707e29e38f04733736386fc1ad2490e96bc7b09ef16bc161bf3e.

The maintained file also retained its original hash. No .orig/.rej file was found in G01. PATCH_G01.stdout/stderr preserve portable output; original raw logs remain in owned scratch. Patch SHA7131afef127f31543b4fd7e7c88266e1205b20b3059245b9617e4280b6aa94fd and all preimage/test/lock checks passed before the command. This is an operational patch-tool failure, not a source/math finding, compiler failure, semantic kill or survivor.

No TMPDIR override, fallback tool, retry, patch repair, Rust launch for G01 or later experiment was attempted. All54 mutant filters and54 returned-baseline controls remain UNRUN. G02–G54 were not touched; all66 registrations remain uncredited. VERDICTS.json states these dispositions separately.

## Runtime and evidence

ROOT released at10:15:15 UTC, fixed end11:15:15. Actual runtime start10:17:03; first direct baseline launch10:18:01.444. The last baseline B28 passed before the first patch attempt. The stop was reported immediately and its concrete log inspected by10:30:17. No five-minute command checkpoint was reached; longest baseline was36.13s. No owned job is running.

All28 calls used the pinned E1 binary with the full cleared fault/Rust environment, two test threads, existing guard5387, tool-managed PTY and time-l. No Cargo build was started. Baseline source/test/lock bindings and binary/features were checked per filter; binary remains2c9ff111dd423d73fd6def78ceff60421b6570523c713dcfda7075811c02e970. Full per-filter output and identities are in baseline/. COMMANDS.json preserves actual commands/working directories, launch/completion and the single patch attempt.

Source basis remains40129a225d73860ac2a53da9a2fa73869df668f3. Readiness commit9ed9b020eb2d304d999c42902bfb9196b901ee62 is not a source rebind. Frozen manifestbd4dae325f30af11bb9ec1479f255f73912b96b7d4e50a066825aee725644e90 and readiness seal61c51e169bff21dd116a472e42f49be0468b77207b648b55c086d03830c04b30 remain unchanged; all six readiness entries were reverified.

All54 prepared copies, targets, logs and previous packets are retained. No maintained source/test/lock/assertion/input/truth, observation, Git/index or old registry change, new case, framework, child or unselected run. ROOT must disposition the patch-tool environment and separately authorize any continuation. The original semantic requirements and historical/reconstructed/equivalent/optional qualifications are unchanged. SHA256SUMS excludes itself.

