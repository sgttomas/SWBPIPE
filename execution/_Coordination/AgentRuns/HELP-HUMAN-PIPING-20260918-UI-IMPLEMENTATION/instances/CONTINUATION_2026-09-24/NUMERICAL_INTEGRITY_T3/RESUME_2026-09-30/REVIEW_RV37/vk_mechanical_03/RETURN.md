# RV37 — full mechanical V-K matrix D review

**D's bounded mechanical matrix is complete and suitable for ROOT fan-in. No blocking finding remains.** This adds no semantic credit to the independently reviewed A/B/C witnesses and does not qualify ordinary artifacts, measurements, product/native behavior, K6c acceptance or release.

Reviewed committed K6C 8924af3cbe7a642b7d5938e65a1c75b1b6766d2c, I27/vk_mechanical_07, against maintained 81c03849033f3ce745668f581f446530789397b8 and its exact NUM grant. All 80 payloads match author seal e032d1a49db4f92458b12e18c9644cf388f891c9007e152d92211f720a4ed8dc. Canonical packet bytes were read directly; no earlier packet was copied.

The unchanged runner ran once with separate --timeout and 1800 tokens, no --only and no --from-logs. Raw stdout, 17 flushed events, final matrix and every complete row log agree on original NONE, fifteen-ID, UNKNOWN order. The same 60 source-declared test names appear in every row, with 12 completed summaries and 11 executed test targets. NONE passes all 60; every registered fault has at least one real failed test and exit 101. UNKNOWN has 23 failed tests, each with the exact quoted unknown-ID panic at seeded.rs:81:17. Its six envelope-related failures are explicitly mechanical only.

| Row | Failed tests | Exit | timed_out |
|---|---:|---:|---|
| NONE | 0 | 0 | false |
| F01 | 12 | 101 | false |
| F02 | 10 | 101 | false |
| F03 | 11 | 101 | false |
| F04 | 12 | 101 | false |
| F05 | 1 | 101 | false |
| F06 | 1 | 101 | false |
| F07 | 10 | 101 | false |
| F08 | 2 | 101 | false |
| F10 | 7 | 101 | false |
| F13 | 1 | 101 | false |
| F17 | 12 | 101 | false |
| R02 | 8 | 101 | false |
| R28 | 10 | 101 | false |
| S1 | 2 | 101 | false |
| S2 | 9 | 101 | false |
| UNKNOWN | 23 | 101 | false |

I checked the full test lists, summaries, panic sites and log tails, not only verdict samples. All summary totals reconcile with the per-test results; no ignored, measured or filtered tests were substituted. No abnormal-signal, compiler-failure or survivor evidence appears. The runner's source returns killed before checking timeout, but that precedence does not conceal a timeout here: every final timed_out is false, every actual exit is present, and all 60 results/12 summaries complete. Failed tests establish reach only to their reported assertion or panic; later assertions and case-loop iterations remain uncredited.

The archive contains exactly the independently reconstructed 639-file required scope: complete core and VR, all 41 tracked Piping manifests reachable by the unchanged feature guard's exclusions, and the release-readiness checker. Tar hash, file bytes, Git blobs, executable modes and current source inventory all bind to frozen Git; no overlay or missing guard input exists. The complete normal suite passes the original manifest feature guard and checker-source test. That is not a separate execution of the release-readiness gate.

All 11 executed binary hashes match their actual Running target paths in every row; all 34 retained fingerprints match current bytes. Raw fingerprints preserve exact integers and empty rustflags; FK mutation-controls and VR seeded-faults are explicit. Every row reports the unoptimized test profile with debuginfo. The recorded installed toolchain and sampled Cargo paths identify Rust/Cargo 1.97.1; command/source checks retain offline/locked, -j4, testthreads2, incremental0 and cleared compiler flags/wrappers.

**Build-binding limit:** the unchanged runner captures and discards successful initial --no-run output. No compiler-artifact JSON stream or live --no-run process sample is present. Initial build success is a control-flow inference from the unchanged runner reaching NONE, supported by subsequent complete test execution, exact source/archive bindings and actual binary/fingerprint identities. I have not invented a compiler event, independent initial-build duration or stronger per-artifact emission witness. No extra build or replay is required by this bounded original-runner review.

The 88 retained process-group snapshots contain one runner PID, 88835. All 17 event observations lie in the recorded 05:13:39.077–05:28:19.576 UTC outer interval; the runner exits 0 before its original 05:49:10 cutoff. The completion record reports wait/reap with no remaining members. An independent read-only current process-group check also finds no members. The original guard PID5387 is present in the initial guard record; snapshots are not a continuous monitoring or hard-RSS guarantee. The 880.499-second interval is operational provenance, not performance qualification. Author sealing at 05:31:32 also precedes the 05:54:10 deadline.

D supplies one initial NONE and no restoring NONE after each fault. The last row is UNKNOWN; no normal restoration is invented. Canonical semantic coverage remains RV37/vk_semantics_01, seal 4d195d574d100a98ebc06f10b97154480f28cbdf8aa80c81bdef48236354cc65, with wording closure vk_semantics_02, seal 9cda91957521bb68f8aee48401a48084cbaa144e9c512fa07171c7c46adc554c. Their separately bound A/B/C triplets, original F17-only scope, unavailable/unreached limits and zero-credit history are unchanged. Broad D failures and new envelope tests cannot replace those witnesses.

This review used only optional-locks0 Git reads, standard-library metadata/text/hash checks and one read-only owned-process-group inspection. No Rust/Cargo/binary/runner/model/count/solver runtime, source/test/criterion/tool change, Git/index mutation or delegation occurred. Only this compact additive packet was written. No automatic continuation follows.
