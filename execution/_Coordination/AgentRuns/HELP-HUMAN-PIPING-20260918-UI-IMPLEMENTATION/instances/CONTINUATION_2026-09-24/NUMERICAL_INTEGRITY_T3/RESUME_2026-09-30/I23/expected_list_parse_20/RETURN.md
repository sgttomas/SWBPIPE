# I23 expected LIST parse scratch20

For the exact1357-byte expected_unresolved.json (SHA2562e5d0975a90c69e5d198d61760cc7734d42615461710da1fe226f60b4a46f2c9), the successful StrRead parse requires a finite scratch allowance: **80B retained during parsing;120B scratch construction old+new upper;40B maximum active-old increment;0B scratch after from_str returns**.

The only two backslashes are quote escapes in the final source value, at zero-based file offsets1300 and1351. Its73-byte encoded payload consists of20 ordinary bytes, escaped quote,49 ordinary bytes, escaped quote, empty suffix; decoded length71. Other keys/strings remain borrowed at the parser interface, so scratch is empty before this value.

Actual operations are extend20 -> push1 -> extend49 -> push1 -> extend0. The authenticated serde path and reviewed u8 Vec growth give capacities0 ->20 ->40 ->80 ->80 ->80. The last growth overlaps old40/new80; the final quote fits. No Unicode or number-token scratch path is reached by these fixed bytes.

The80B scratch overlaps the independent71B Value String copy. That71B remains part of the existing Value owner, not a second additive charge. from_trait drops its Deserializer/scratch before returning Value; the subsequent typed LIST collection overlaps raw input and Value but not scratch. Preserve the existing typed78B retained owner separately. File length1357 binds input content, not a newly inferred read-buffer capacity. A safe scratch addition to the established parse-construction union is120B, without also adding80B for the same scratch owner.

Exact bytes, growth trace, authenticated source hashes, numbered citations and phase overlaps are in `_run_records`. No input JSON parser, Rust/runtime/build/probe or solver ran. No code/tool changes, library survey, Git/index or delegation. No full Emax/admission claim. Start20:17:04 UTC; boundary20:22:04 UTC.

Completed:2026-10-01T20:21:04Z. Source/input hashes and capacity arithmetic rechecked; no parser execution.
