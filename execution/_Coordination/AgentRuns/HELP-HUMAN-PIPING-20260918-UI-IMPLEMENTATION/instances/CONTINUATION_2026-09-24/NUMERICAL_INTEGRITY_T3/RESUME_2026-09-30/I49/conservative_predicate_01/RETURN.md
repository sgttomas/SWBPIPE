# I49 — one conservative interpolation refusal

**The geometric/source residual enclosure alone forces both sharper refusals at actual interpolated-loaded row10. Direct projection of the captured native point into the actual units cannot remove this refusal.** A center-independent lower bound on this row's geometric half-width is **8.9200166690809899e-23 m**, about **125.1578 times** either sharper allowance. This is a source-grounded lower bound, not a reconstruction of uncaptured private endpoints.

TASK Type2 `/root/i49_conservative_predicate` directly under ROOT HELP_HUMAN `/root`, delegated-harness-native; no descendants. First actual clock/receipt: **2026-10-03 05:09:07 UTC**; checkpoint8 deadline05:17:07, new-analysis cutoff05:24:07, return deadline05:31:07. Numerical result supplied to ROOT at05:15:07. No compiler, model/solver/native execution, network/API, host tooling work, maintained-source change or Git/index write. All shell calls used explicit intended cwd; every Git read set `GIT_OPTIONAL_LOCKS=0`. Only the new NUM/R/I49/conservative_predicate_01 packet was written.

## Bound actual identity and independent point comparison

Inputs are frozen CODE/R/I47/selected_material_02 `_run_records/pp_debug.log` (SHA-256 `b443aabe00d24804e98dfe3f662db32d2a3277de976790998653354674f002c5`) and `oracle_final_debug_results.json` (`43e37d0c2085487c2f86087612400398706dbe4a21d6d8a12ec74420bb8c117f`). Source reads were copied from **d0daa18717f8243a7232e898c9ef9b4f4d18d9e4**, still an I47 candidate under independent review. The brief was checked byte-for-byte against **19703b23a89e816dcdda32faa8cd473f4214832f**. `_run_records/ORIGINS.json` records Root/TASK/Piping/brief, raw input, return and source origins/hashes. No additional role, workflow or skill body was selected.

The actual request is case:i45-loaded, one unit X-directed member, D and t as their captured binary64 values, fixed base, and three distinct unit tip loads UX/UY/RX. Material0 lower/upper point ordinals1/2 and their actual Pa/K request bits match the successful selection. Exact four-product interpolation of the captured lower293/selected303/upper313 K operands gives:

- E_s = 12451840000000001/65536 Pa; E_hat bits42461e70f6000000; E_s−E_hat=1/65536 Pa.
- G_s = 13107200000000001/262144 Pa; G_hat bits42274876e8000000; G_s−G_hat=1/262144 Pa.

No intended decimal values replace these represented inputs. Full four-product operands and exact fractions are in RESULTS.json.

Actual final row10 `result:disp:node-N-DEC092-TIP:ux` binds to native row6 `Displacement(Dof { node: 1, component: Ux })`. Its raw mm bits are **3ecd962166f5dd6e**; separately rounded division by1000 gives **3e2e4be8dc1e94ec**, exactly the native SI point. The coupled primary translation/rotation scale is **3ed8d89883b941b9**, supplied by tip RX; the body extent is exactly1. The row is RelativeVerified, and actual predicates are `[false,false,true,true]`.

An independent exact-rational checker derives q_K=1/(E_hat A_hat) and encloses q_source=1/(E_s π t(D−t)). Its π enclosure uses a fresh rational alternating-series Machin calculation, not the author's oracle. It independently checks all four applicable row predicates against both truths:

| Quantity | Value in SI |
|---|---:|
| Represented-truth point error | 6.1080658808120047e-26 |
| Source-truth point-error upper bound | 6.8081392744259433e-25 |
| SharperExact allowance | 7.1270154216320684e-25 |
| SharperBinary64 allowance | 7.1270154216320689e-25 |

Thus this is a conservative refusal relative to both independently derived truths. Author row10 claims were extracted only after that derivation and agree; author math/scripts were not used as proof or executed.

## Sufficient forced inequality

References below are lines in the pinned `SOURCE_*.rs` copies under `_run_records`.

1. `SOURCE_product_certificate.rs:418–453` computes interpolation and explicitly calls `hull_point(actual)` on E and G. Therefore the actual source coefficient interval includes **both** G_hat and G_s, despite only G_s being the exact selected source truth. Multiplying by its positive geometric J interval includes G_hat J* and G_s J* for any one fixed positive J* in that interval.
2. For this exact straight frame, the RX residual before scaling contains `1 − G J* c`, where c denotes any private corrected tip-RX center. The source B/D/transpose operations are at `SOURCE_source_residual.rs:340–414`, loads and residual scaling at529–580. No other load/component couples into this physical torsion residual.
3. Put a=G_hat J* and b=G_s J*. For every real c, the identity `b(1−ac)−a(1−bc)=b−a` implies `max(|1−ac|,|1−bc|) ≥ (b−a)/(b+a) = (G_s−G_hat)/(G_s+G_hat)`. This uses no actual correction center, midpoint or endpoint.
4. Every member contributes the full structural12×12 pattern (`SOURCE_assemble.rs:546–569`); free blocks follow that pattern including numerical zeros (`SOURCE_bound.rs:84–120`). All six free tip DOFs therefore share one block. The source bridge uses the verification factor's radix scales (`SOURCE_adaptive.rs:5492–5538`), with `s=−floor(diagonal_exponent/2)` (`SOURCE_factor.rs:523–535`). Exact axial and torsion diagonals are E_hat A_hat and G_hat J_hat (`SOURCE_assemble.rs:280–351`); their exponents are28 and17. Products fit exactly at verification precision256. Consequently s_UX=−14 and s_RX=−8.
5. Actual B bits **40139d85e14169c8** equal `690152861609273/140737488355328`; the capture is explicitly `ev.certified_bound` (`SOURCE_retained_product_tests.rs:903–907`) and `SourceBridgeView::bound` returns that field (`SOURCE_adaptive.rs:5323`). In `SOURCE_source_residual.rs:774–809`, beta=2B, omega is the maximum absolute scaled residual endpoint, and epsilon is rounded upward from beta·omega/(1−alpha). A completed result has 0≤alpha<1. Hence epsilon≥2B·2^s_RX·(G_s−G_hat)/(G_s+G_hat).
6. UX gathering expands its center by 2^s_UX epsilon (`SOURCE_source_residual.rs:424–448`). Therefore its geometric half-width is at least

   `L = 2 B · 2^(-14-8) · (G_s−G_hat)/(G_s+G_hat)`

   `  = 690152861609273/7737125245533627013267431579352825856`

   `  = 8.920016669080989931239326570e-23 m`.

7. The Native recipe takes this interval unchanged (`SOURCE_final_case.rs:478–479`), hulls it with the represented enclosure (`:1061–1079`), and tests the largest endpoint distance (`:548–553`, `:603–696`). For any tested value, that distance is at least the interval's half-width. Directed outward rounding can only preserve or enlarge the lower bound. **L exceeds both exact and separately rounded sharper allowances**, so the geometric branch alone forces both refusals for this actual row.

This isolates the interaction: interpolation's represented/source material hull has a nonzero width; the torsional residual from that width propagates through a common scaled block radius into UX. Real-π geometric rounding is not required to establish the bound. A more accurate correction center or additional solver precision alone cannot remove this center-independent obstruction while these operands, hull and radius recipe remain unchanged.

## Native enclosure, units and next dependency

The native publication radius itself is absent from the captured log. Its value and the geometric endpoints have **not** been reconstructed. The source nonetheless bounds the native radius by the native SharperBinary64 allowance (`SOURCE_adaptive.rs:3547–3570`, `:3796–3855`). Recomputing the native primary scale yields exactly the same scale as the final row. Since the final normalized value is also the same native point, the represented branch alone cannot force this row's SharperBinary64 refusal: its radius is no larger than that allowance, and the outer endpoints formed using the allowance are exactly representable at1024 bits. No claim is needed about a possible sub-ulp SharperExact effect from rounding the native radius upward; the independent geometric obstruction already forces both predicates.

The explicitly algebraic direct-unit projection uses each captured native primary value, rounds translation×1000 and normalization÷1000 separately, and recomputes primary maxima and coupling. For UX it yields the **same raw bits, same normalized bits and same scale** as the actual final row. The lower bound and both allowances therefore remain unchanged. This projection is not an executed complete product producer or a new accepted output.

**Next dependency:** a separately scoped, independently derived and reviewed tighter private final-row certification method under the unchanged truth and predicates is required before this specific refusal can disappear. Its warrant must address the proven material-hull/block-radius inflation. I49 selects no correction or alternate source, and grants no implementation. No additional controlled capture is needed for this sufficient inequality. Native radius and private endpoint reconstruction remain outside this result.

This does not infer causes for the other46 conservative rows, guarantee another route will pass, or disturb I46/RV61's distinct actual source-point miss. It supplies no I47 implementation clearance, protected availability, public routing, C2/reader/receipt, resource, acceptance or release qualification.

The recoverable rational script, actual-case extraction, complete exact results, algebraic projection records, source copies and command/provenance records are under `_run_records`. SEAL.json pins the completed packet.
