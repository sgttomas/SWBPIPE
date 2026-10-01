# PM03 generic-type correction — source-only proposal

The full replacement PM03 patch against40129a225d73860ac2a53da9a2fa73869df668f3 changes exactly one type token from the frozen patch:

    Wide::<4>::from_f64(scale)
    Wide::<L>::from_f64(scale)

certify_rows<const L> already declares verification &[Wide<L>] and Wide<L>:SupportedWidth. The revised constructor uses that explicit generic width. The source contract of from_f64 (wide/multi.rs:834–857) exactly decodes every finite binary64, including subnormals/zero, into the top limb with zero lower limbs. Supported widths are4/8/16; changing the number of zero lower limbs preserves the represented scale. add_wide_scaled's exponent adjustment (wide_sum.rs:344–369) preserves the exact scale*2^-64 contribution.

The difference/absolute operation, factor1, shift−64, meter.collect call/order, predicate, radius conversion and fixed test are unchanged. Existing limb accounting follows the operand width L; no width-invariant or hypothetical mutant price is asserted. The targeted controlled fixture uses L=4. No expected output, assertion or oracle changed.

| Binding | SHA256 |
|---|---|
| Preimage |4e5618a8d809e6ffa4ddd73024c206cc45eac6d5404a2f2034674552e1a95db9|
| Original patch |180783dcddba7f73af5b12fe832cc704bf5ff1073c72dfffdaacc6239b89bfd8|
| Original failed postimage |34dde1dfdebcd7cebaaa2a1965dad44d418dd49b66d126342f8c792a492e3bc8|
| Revised full patch |b689954d60c7640ef2ddd5670a8a1e2949f28d44e85813889236e76c5219f38b|
| Revised postimage, memory only |7d1c6750677afa21a161c95bbbb987f138e983bccf3408f06658462223b19225|
| OVERRIDE.json |f5cb93bb3630448d4a773e0027c569eb6c016bfef21fa73cf5c5ed030cb5d741|

PATCH_DELTA.diff preserves the exact one-line change. OVERRIDE.json binds the old and revised PM03 entries, unchanged filter/semantic expectation/test hash, source-contract hashes and prior seals. Other34 variants and the original MANIFEST remain unchanged.

The frozen applicator checked this override against the immutable baseline in memory, without --apply: exit0, mode check-only, source_write=false, expected revised postimage hash. No source application, Rust/build/test or rerun occurred. Original E44-entry and stopped E1 61-entry seals verified unchanged; the [original stop](../RETURN.md), [original patch](../../mutations_e/patches/PM03.patch), failed source and raw logs are preserved.

Authority: ROOT native follow-up through manager. Actual start09:19:16 UTC; preparation boundary09:29:16. Any later E1 continuation retains10:06:30, with no automatic extension. Independent reviewer confirmation and ROOT runtime regrant remain pending. I22 is Rust-idle with no owned experiment.

