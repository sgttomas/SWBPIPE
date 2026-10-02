# RV30 wrapped_errors_19 — three private request backchecks

**No blocking defect found. The exact inspected release vk_records contains the claimed requests: StringError24/8, io::Custom24/8 and serde_json::ErrorImpl40/8.** These are allocator size/alignment arguments with checked source/type/data-flow attribution, not inferred layouts or measured peaks. Final ordinary-production vk_scale correspondence remains open.

TASK Type2 RV30 /root/rv30_k6c_kernel, direct native child of ROOT /root HELP_HUMAN Agent0. Actual start2026-10-01 19:47:37 UTC; deadline19:57:37 UTC. The preceding format_stream_18 topology review was sealed before this separate authorized leg. GRANT.json records the exact native grant and carried instruction hashes. No delegation.

All artifact18 payloads verify against sealc0fc7c94bc7b7fd7ce1d6958bda8811f952705efb5e809a1cc029a48fbd3b4c9. The2605200-byte executable independently matches7c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e. Current fingerprints and supplied build/source records match their hashes. The five exact bounded objdump commands were independently repeated on their existing ranges (1608 code bytes total); all exit0/empty-stderr results match the sealed output byte-for-byte. No binary/test, compiler, new symbol lookup or archive reader was run.

## Exact constructor attribution

In typed Error::new<&str> at0x10016fa64, the first allocation at0x10016fa90 uses incoming string length x19 and alignment1. It is the String backing and is not one of the private box requests. At0x10016faac/0x10016fab0 the code sets w0=24,w1=8, then calls __rust_alloc at0x10016fab4. The returned pointer becomes x22; the converted String capacity/pointer/length are stored into it. The pinned From<&str> -> From<String> source ends in Box::new(StringError(err)), identifying this first24-byte object. The empty-string constructor alternative has the same request, but no runtime occurrence is inferred.

The next independent call at0x10016fad4 again receives w0=24,w1=8. Its block stores the preceding StringError data pointer plus a vtable, then the ErrorKind; the returned pointer is tagged+1. Source Error::_new owns Box::new(Custom{kind,error}) in Repr::new_custom. This is a second live allocation, not reuse of the first24 bytes. The repr(align(4)) source attribute is only a lower representation constraint; the actual8 argument comes from this compiled call.

Typed serde_json::Error::io at0x1001a91c8 sets w0=40,w1=8 before __rust_alloc at0x1001a91f0. The constructor stores the Io error payload and zero line/column in that result. Its pinned body is Box::new(ErrorImpl{code:Io(error),line:0,column:0}), and the captured cleanup explicitly names ErrorImpl. No public Error size or guessed field sum is used.

The allocator entry's source fixes argument order __rust_alloc(layout.size(),layout.alignment()); AArch64 w0/w1 immediate writes zero-extend into x0/x1. The allocation-failure calls have a different lowered argument order and were not mistaken for the allocation sites. The three immediate Rust source pages and two serde pages independently hash/decode-match their supplied bindings.

## Direct caller and topology distinction

The supplied WriterFormatter::write_all capture shows the failed fmt write calling typed Error::new<&str> at0x10003c93c with string length9. That9 is the literal-message length, not a box-layout constant. The earlier w0=40 there is ErrorKind for that constructor, not ErrorImpl's40-byte request. The compact escaped helper forwards the io error; the typed Value serializer forwards it to Error::io, including the captured tail branch0x10003cd50. No additional caller/functions were inspected.

These facts bind the three numeric requests. The separate sealed format_stream_18 supplies their coexistence/move/drop theorem for the actual initialized normal-or-handled-I/O route. Combining that theorem with these artifact-specific values gives the conditional local substitution

    E_wrap = 9+24+24+40 = 97 registered bytes; active-old = 0.

This is an additional error-chain term above the still-live Value tree/caller/writer owners. It is not a measured97-byte process peak, an assertion the error path executes, a per-depth multiplier, or a full runtime envelope. The numeric sites do not substitute for the topology proof. Fatal allocation failure, unexpected backend/general panic and all broader remaining source/caller obligations retain their explicit interfaces.

## Scope of transfer

The exact recorded artifact is vk_records from source942572/numerical40129, Rust1.97.1/aarch64/LLVM22.1.6, release opt3/no debug assertions. It enables VR seeded-faults and FK mutation-controls. Normal-release wording must not be read as proving those features absent; it describes the recorded unmutated use of that feature-enabled artifact. There is no automatic equivalence to a future ordinary-production vk_scale binary.

Final source/type/compiler/library/target/cfg/profile/request-site and allocator correspondence is therefore still required. Current fingerprints, installed std source and authenticated serde bytes corroborate the scoped attribution, but do not supply an invented contemporaneous full linker-input list. The two static-message pool maxima remain source-routing gaps, and pre-cut/global union, checked integration and measurement/admission remain separate. The three private request gaps are now supplied for this exact artifact; they should not be reopened as unknown numeric facts while their transfer limits are preserved.

All earlier seals, including format_stream_18, remain unchanged. Only this additive review packet and its _run_records were written. No runtime/build/probe, library expansion, maintained source/API/estimator edit, new tool, Git/index, network/install or delegation occurred. No full E_max, K0, implementation or admission acceptance is granted.
