# RV62 — complete source-truth coverage backcheck

**CLEAR for the records-only correction. No false candidate PASS, new source implementation defect, or need for another runtime test is established.** This same-reviewer backcheck corrects my earlier numerical coverage claim; it does not reopen the independently accepted source/custody implementation merely because that evidence was incomplete.

TASK `/root/rv62_selected_material`, directly under ROOT `/root` HELP_HUMAN, delegated-harness-native, no descendants. First host clock: **2026-10-03 06:10:09 UTC**, Mac.lan. Checkpoint: **06:18:09**; analysis cutoff: **06:28:09**; return deadline: **06:35:09**. The instruction hashes are unchanged. Brief: `997e5e7992`; frozen captures/source: `d0daa18717`. Full pins, actual origins, hashes, byte counts and absolute locations are in `_run_records/INPUT_MANIFEST.json`. No runtime lane was taken.

## Correction and result

I36 preview-truth RETURN §2 and ordinary-coefficients RETURN §1 step 6 / §2 already require inclusion of actual resolved E/G with source-annulus geometry. My original RV62 checker compared exact interpolated E/G with source geometry and the represented K readout. K uses different, rounded section primitives and cannot substitute for the omitted resolved-material/source-geometry interpretation.

Accordingly, the original REVIEW's full-cover implication and interpolated-loaded **19 truth-miss / 47 conservative rows** classification are superseded. Those comparisons remain correct relative to the two readouts actually checked then. The previous no-false-PASS argument was likewise incomplete for the accepted cover; this backcheck supplies the missing coverage and still finds none. The source/custody review and its actual execution evidence remain distinct, unchanged records.

| Frozen case | Candidate numeric pass rows | Truth misses, old → complete | Conservative rows, old → complete |
|---|---:|---:|---:|
| Point zero |73/73|0 → 0|0 → 0|
| Point loaded |42/73|31 → 31|0 → 0|
| Interpolated zero |73/73|0 → 0|0 → 0|
| Interpolated loaded |7/73|19 → 20|47 → 46|

Exactly interpolated-loaded row 10, `result:disp:node-N-DEC092-TIP:ux`, changes. SharperExact and SharperBinary64 become genuine truth failures; both candidate predicates were already false. Decimal SI/raw still pass. The source-annulus response using resolved E gives an error lower bound about **9.640635795660434e-25 m**, exceeding the unchanged exact sharper allowance **7.127015421632068e-25 m**, by a factor of **1.352689060612802**. Direct annulus evaluation equals exact rescaling of the E_s response by E_s/E_hat and independently agrees with ROOT's bracket.

Across all four cases, **292 mechanical rows and 562 applicable predicates** are checked. All **390 candidate PASS predicates** have certified worst-error upper bounds within their allowances, including passing predicates on refused rows. No ambiguous interval is counted as a pass; none occurred. Conservative predicate refusals change from 73 to **71**. Actual candidate passing rows remain 195. Every other truth category and all candidate outcomes are unchanged.

## Why the finite cover is complete for these cases

The checker validates the frozen two-node, x-aligned, one-member cantilever, its y reference, anchor with all six base DOFs restrained, absent components/combinations, and three separate tip Fx/Fy/Mx terms, each zero or one. It derives selected point/bracket identities, temperatures, moduli, alpha and section operands from the actual captured request. The original row roster, native identities, raw/SI normalization, classes, scales, bounds, allowances, G5a and observable checks are replayed exactly before extending the truth set.

For interpolation, the normalized four-product numerator divided by positive h gives the exact material target. Every actual product, partial sum, denominator and quotient is dyadic with at most 62 significant bits and ordinary finite exponent. Thus the accepted 1024-bit directed construction is exact on these material operations, and its hulls are precisely:

- E ∈ [190000000000, 12451840000000001/65536] Pa;
- G ∈ [50000000000, 13107200000000001/262144] Pa.

The point hulls are singletons at E = 195000000000 and G = 55000000000 Pa. Source annular A, I, J, Z and c remain fixed by normalized D, effective t and real pi.

For the loaded L = 1 specimen, ux = 1/(E A), uy = 1/(3 E I), rx = 1/(G J), rz = 1/(2 E I), and uz = ry = 0. The nonzero components are positive and decreasing in their sole modulus. Translation magnitude is a positive constant divided by E. Hence their entire positive material-rectangle ranges lie between corner responses. Anchor motions are identically zero.

The cantilever is statically determinate: end/station actions and anchor reactions depend on the individual loads and cut location, not E/G. Their support norms and source section stresses are consequently material-independent. The normal-stress maximum is the static |N|/A + hypot(My,Mz)/Z maximum; torsional shear remains separate. With every individual load zero, positivity and the fixed anchor give unique zero motions/actions for the whole material rectangle. These arguments exhaust the 73 mechanical rows. They are not a corner theorem for arbitrary coupled or indeterminate frames.

The new script evaluates all four independent source-material corners and both represented K bending meanings, Z_hat and exact I_K/(D/2). It uses my earlier independent Fraction/Machin-pi/integer-sqrt algebra, not the author's arithmetic or product code. The two K meanings are recorded separately and their hull checked against the prior K cover. Positive raw-unit conversion preserves containment. For each predicate, the maximum constituent upper error certifies a pass; a constituent lower error beyond the allowance proves a genuine miss. Convexity of absolute error over the proved response ranges makes endpoint distances sufficient.

The coherent E/G material rectangle is separate from the implementation's larger independent geometry/coefficient enclosure. This backcheck does not declare every independently varied EA/EI or geometric interval endpoint to be a new physical truth. It neither reconstructs private recipe endpoints nor proves arbitrary interval tightening correct.

One remaining full-cover conservative witness is interpolated-loaded row 12, tip uz: its truth is identically zero for every covered law, the actual normalized value is +0, and the positive Absolute allowance has bits `3ad8d89883b941b9`; the candidate predicate still refuses. This identifies a row only and selects no subsequent method.

## Preservation and disposition

The independent calculations exactly reproduce the original reviewer result before extension, match every new author truth decision/allowance, and verify author captures against the original reviewer log. All 24 author-addendum payloads, 58 prior I47 payloads and 59 original RV62 payloads verify unchanged. The four maintained source hashes still agree in CODE and NUM. No source, capture, class, scale, bound, output, predicate, row order or old evidence changed.

Both zero cases retain G5a's negative-zero refusal at row 36; both loaded G5a checks pass; all four complete private cases remain refused. A narrower private interval cannot make the current UX pass the complete cover without addressing its genuine required-readout miss. This does not invalidate the separate earlier block-radius inflation result, which this task does not reassess.

The new packet contains only concise records, a small checker and manifests. The 6,503,115-byte exact calculation result is in authorized external scratch; `_run_records/BULK_MANIFEST.json` pins its hash, bytes and absolute location. Replay reads frozen inputs by manifest/hash and writes only this new packet and its scratch output. Both checker executions passed; the second only reduced committed-facing output bulk.

No source/Cargo/model/solver/native/Git/index/API write, host tooling, delegation or old-record edit occurred. No new runtime or acceptance gate is implied. ROOT alone integrates the correction and selects subsequent work; public availability, implementation of a tighter method, C2/readers/receipts, resource qualification and release remain outside this backcheck.
