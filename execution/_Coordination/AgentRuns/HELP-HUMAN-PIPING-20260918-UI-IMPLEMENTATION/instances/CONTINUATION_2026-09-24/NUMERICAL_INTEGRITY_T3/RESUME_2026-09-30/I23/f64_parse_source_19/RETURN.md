# I23 f64 parse source19

The pinned successful finite str::parse::<f64>() route has **zero incremental requested heap scratch and zero active-old realloc bytes**. This is a source warrant, covering the slow fallback rather than assuming all roster strings take a fast path.

str::parse calls f64 FromStr, then dec2flt. The initial scanner borrows bytes and uses scalars. Fast conversion uses primitives/constant tables; Eisel–Lemire uses u64/u128 and a static powers table. The fallback is a local DecimalSeq with an inline768-byte digit array plus scalar metadata, mutated in place. Excess digits set truncation state; no heap growth occurs. ByteSlice's callbacks are direct generic FnMut closures over those local values, not boxed callbacks or user-defined allocation hooks. The aarch64 precision guard is an empty unit-returning function. Final conversion is primitive bit assembly/transmute.

`_run_records/WARRANT.json` records the complete direct route, actual fixed fallback/table/callback storage and line citations. `_run_records/SOURCE_BINDINGS.json` binds14 installed pages to Rust1.97.1 metadata and the two immutable40129 caller sites. No allocation interface gap remains for this successful finite conversion subpath.

Existing reference/s_full String storage and caller maps remain separately charged. This is not an exact stack-frame claim, a new finiteness proof, a claim about caller expect/panic/error formatting, or a serde-parser storage bound. Final binary/source correspondence and full E_max/K0/admission remain separate.

No parser/executable/runtime, build/probe, source edit, library survey, tool installation/change, Git/index or delegation occurred. Only this evidence packet was written. Start20:06:41 UTC; boundary20:11:41 UTC.

Completed:2026-10-01T20:10:48Z. Final source hashes rechecked;14 pages and2 caller sites unchanged.
