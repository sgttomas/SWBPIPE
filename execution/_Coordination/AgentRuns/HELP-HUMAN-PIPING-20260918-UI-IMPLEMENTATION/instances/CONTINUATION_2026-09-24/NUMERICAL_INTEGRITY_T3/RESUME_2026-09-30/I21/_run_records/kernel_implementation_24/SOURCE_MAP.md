# I21 kernel implementation 24 — source mapping

This is a checked translation of accepted conditional arithmetic, not a new
source proof. Paths below use H=projects/chirality-piping/core/solver/performance_harness
and R=the enclosing RESUME_2026-09-30 directory. Exact origins/hashes are in
SOURCE_ORIGINS.json and BASIS_VERIFICATION.json. ROOT's b364c679 disposition is
preserved as ROOT_DISPOSITION.diff. source05 is read with the source06 corrections
and the later generic kernel22/independent kernel27 disposition; its historical
open-cell labels are not silently promoted into independent final qualification.

| Maintained implementation | Reviewed source arithmetic and preserved meaning |
|---|---|
| envelope.rs:17,25 — proof identity/profile | metric_design_12_profile_binding selects one immutable named reference basis. The only constructible profile selects the module's fixed source40129/Rust1.97.1/aarch64 fact bundle; no current-host sizes, build token, caller-supplied overhead, source reader or executable attestation. |
| :110 — KernelDescriptor::new | source09 DESCRIPTOR_MAP finite H/VR construction and population contracts; kernel22 POLICY/GENERIC_TERMS. Checks n=6N, f=n-r, generic q/X, z/h population limits, encoding/request widths, source-ID byte relationships and exact/upper B/b policy. Source validity and the supplied count/source identity remain external obligations. Supports/nonzero prescribed children require unavailable proof and fail explicitly. |
| :433–532 — checked scalar/capacity operators | source05 E/G/A/O/Sort/TI definitions and source09 input classes. Checked u128 propagates failure through all alternatives; final byte fields check the named 64-bit reference Layout maximum. The fixed source constants and bounded precision/index loops cannot overflow independently. No saturating arithmetic or float conversion. |
| :534–613 — pairs, trackers and R7 set | source05 K19 and kernel22/27 one-active-old interpretation. Pair addition takes max(M1+R2,R1+M2). Lazy storage 512/4352 and growth-old 256 slots retain table/tree and prune/rebuild alternatives; no duplicate active-old sum. |
| :654–669 — widths and strides | source05 actual public type/derived Arc roster, corrected BlockBound/Report values, and the accepted reference request facts carried by kernel22 and reviewed kernel27. No new private layout witness or mirror. |
| :688–892 — prepare | source05 K01–K09, source06 Base and source09 construction classes. Original/pushed H/VR source, exact prep clone, source/stiffness/ledger encodings, generic spring/directional contributions, separate geometry alternatives, ordering and free blocks, minimum attempt/state backings3328/64. Exact z/h are never reconstructed from B/b uppers. |
| :856–870 — Shared/Solved/VerifyShared | source05 persistent owner equations plus kernel22 generic directional additions. Success Arcs counted once, both directional widths retained, no duplicate residual directional result, non-ceiling widened temporary remains separate. |
| :926 — schedule | source06 finite Kpad roster: full four solves/three verifications and shortened S/U128,256+V256, both retaining minimum backings. Named phase arrays have fixed96 slots; actual full88 and short38 entries. |
| :970–1100 — solve/pass/shift/report phases | source05 K10–K18 plus source06 corrected RES/TOP/HATCHECK and moved-owner aliases, kernel22 generic s+3d recovery/formation helpers. PassRest cancels moved RES/r_hat/delta before B/b upper substitution. Shift retains start/result widths, current/next identities and one active old. Summary aliases its prepaid attempt slot. |
| :1102–1184 — R7/certificate/finish/return | source05 K19–K20 plus source06 and kernel22 stationary outcome policy. R7 is exposed by verification precision and maximum. Selected/refused/unresolved are padded stationary uppers; Report/Decision are not separately added after return. Complete certificate/finish alternatives remain even for a reference input that cannot select128. |
| mod.rs | Additive module export and narrow distinction between observations and conditional reference arithmetic; caller/admission behavior is untouched. |

The continuation changes only descriptor population consistency in the previous
implementation: for l>0 nonempty IDs require total>=maximum+(l-1); H k6:decimal
IDs require total>=maximum+4(l-1) and maximum>=4. These follow the already read
source.rs empty-ID rejection and source09 fixed-string grammar. They do not infer
the source's actual IDs or run source validation. CONTINUATION.diff isolates this
11-net-line implementation change and the added regressions from predecessor work.

Tests use maintained literal input/output fixtures, without an evidence-file
reader or duplicate full formula. Fixture origins are recorded in FIXTURE_BINDING.json:
axis spring RF-CHAIN-T-n03-r1e-04; directional RF-SKEW-T-PIN-AX-345-r1e-04;
retained cancellation terms RF-CANCEL-F-G1e5-GnG; five-node upper RF-ZERO-SYM;
and H/VR RF-LARGE-CHAIN-n00010-AX under their distinct original exact policies.
The new B=5 assertions bind TOP/HATCHECK and ReportBuild at256/512/1024. The
extended exact-B=1 assertions bind the no-old-first-allocation case at all widths.
Existing full/short, stationary subset, nonzero spring/directional, overflow,
missing descriptor/proof and request-width regressions remain.

The implementation uses scalars, fixed arrays, slices and static error strings.
Inspection found no estimator heap allocation, graph construction, source/encoder
execution, hidden formatting, runtime environment gate or execution-tree read.
This is a source inspection result, not a new allocator measurement. Test-harness
or caller allocations are not included in that statement.
