# RV64 — full-truth correction backcheck

**CLEAR for I49's records-only correction and the single UZ conservative witness. I withdraw my original recommendation to pursue unchanged UX admission through tighter certification alone.** I missed an existing required source interpretation in that planning consequence. The original two point comparisons and center-independent inflation proof remain valid at their expressly narrower scope; they did not establish truth-pass over the full selected cover.

Reviewed subject: `I49/full_truth_addendum_02`, RETURN SHA-256 prefix `edfd6c25fa`, inventory SHA-256 prefix `1c7bf58603`. All 26 payloads match the inventory. The original I49 packet, new I49 addendum and original RV64 review were preserved byte-for-byte (89 files). Full identities are in the manifests. Code citations named `SOURCE_*.rs` refer to the preserved I49 original source copies under `I49/conservative_predicate_01/_run_records`; extra source locations are in SOURCE_CHECKS/ORIGINS. Source remains `d0daa18717`; fresh checks confirm the pinned copies and both NUM/CODE production files match.

## Existing cover and the corrected UX conclusion

I read the actual I36 warrants, not just the author's account:

- `I36/f2a_preview_truth_01/RETURN.md` §2, lines 78–87, requires both exact interpolation and actual rounded resolver values. Its §3 applies the selected ordinary E/G cover to source-annulus geometry.
- `I36/f2a_ordinary_coefficients_02/RETURN.md` §1 step 6, lines 104–111, constructs positive H_E/H_G containing both values. Section 2, lines 155–168, explicitly includes tuples with actual resolved E_hat/G_hat and source geometry and covers the positive coefficient rectangle.

Their SHA-256 prefixes are respectively `0f3ea0bed9` and `74670d4f43`. The source-annulus interpretation with E_hat is therefore already required. The represented K readout uses A_hat and cannot substitute for `π t(D−t)` at the same E_hat.

I re-extracted the actual I47 `interpolated-loaded` raw capture and independently decoded its complete K4SRC source encoding using the unchanged source format. Material/request bits, the fixed unit X-directed member, zero anchor prescriptions and unit tip UX/UY/RX loads are unchanged. UX remains final row 10/native row 6, raw bits `3ecd962166f5dd6e`, normalized/native SI bits `3e2e4be8dc1e94ec`, scale `3ed8d89883b941b9`.

Fresh exact arithmetic gives E_s=`12451840000000001/65536` Pa and E_hat=`190000000000` Pa. Therefore

`q_source(E_hat) = 1/(E_hat π t(D−t)) = q_source(E_s) · 12451840000000001/12451840000000000`.

The additional required readout uses the same geometry, frame, loads and sign convention. I bracketed π independently with `4(atan(1/2)+atan(1/3))`, using 700 rational alternating-series terms and the next-term remainder bounds. Directly evaluating the new expression and scaling the independently evaluated E_s expression produce identical rational endpoints. The normalized actual value is below the entire bracket.

| Check | Independently established value |
|---|---:|
| UX required-readout error lower bound | 9.6406357956604347570262993124…e−25 m |
| SharperExact allowance | 7.1270154216320683639135761790…e−25 m |
| SharperBinary64 allowance | 7.1270154216320689198522486007…e−25 m |
| Error lower / SharperExact | 1.3526890606128018921822637853… |

Both sharper predicates genuinely fail. `RESULTS.json` records exact rational bounds and separate RN64 allowance evaluation. Only after independent derivation, I checked the I49/ROOT fractions: scaling the preserved I49 source bracket gives their identical reported endpoints and error bounds. My independently justified, narrower π/response brackets lie inside those reported brackets; their quoted lower error still exceeds both allowances. Thus I verified both the underlying miss and the reported exact-fraction construction.

The prior geometric half-width lower bound is still correct, and the prior q_K and q_source(E_s) point checks still pass. Their coverage was incomplete for this planning question. Removing that enclosure excess cannot repair the resolved-E/source-annulus point miss while output and predicates stay unchanged. The unchanged native projection also leaves that UX miss in place. The earlier RV64 CLEAR must be read with this explicit correction; it is not full-cover clearance for seeking unchanged UX admission.

## UZ is conservative throughout the positive cover

Identity lookup in the fresh raw capture finds `result:disp:node-N-DEC092-TIP:uz` at final row 12, native ordinal 8 `Displacement(Dof { node: 1, component: Uz })`, body 0, translation kind. Its raw mm and normalized/native SI bits are all positive zero. The actual final verdict is Absolute, failed Absolute, predicates `[false,null,null,null]`. The recorded bound is `3ad8d89883b941b9`.

I derived its truth from the actual source equations. `SOURCE_source_residual.rs:275–338` supplies ex=X, ey=Y, ez=Z and length 1 for the captured frame; exact directed arithmetic preserves these representable values. Lines 341–376 supply B and D_PATTERN; lines 428–448 and 530–579 bind the zero constraints and actual per-term loads. I independently expanded the full six-free-coordinate BᵀDB as polynomials in four arbitrary positive coefficients `(C1,C2,C3,C4)=(EA,GJ,EI_z,EI_y)` at unit length. The full symbolic matrix is retained in RESULTS, so the independence of this subsystem is checked, not presumed from output zeros.

In free-coordinate order UX, UY, UZ, RX, RY, RZ, it splits into C1 on UX, C2 on RX, `C3[[12,−6],[−6,4]]` on UY/RZ and

`C4 [[12,6],[6,4]] [UZ,RY]^T = [0,0]^T`

on UZ/RY. Every cross entry between UZ/RY and the other four free coordinates is identically zero. All base prescriptions are zero. The actual forcing vector is `[1,1,0,1,0,0]`, confirmed from the decoded individual load ledger, so this subsystem's RHS is exactly zero.

Its first leading minor is 12C4>0 and determinant 12C4²>0. Thus UZ=RY=0 for every positive C4, independent of positive C1/C2/C3. This establishes UZ=0 over the entire required positive coefficient rectangle, including exact interpolation, resolver E/G with source geometry and every admitted mixed positive tuple; K's positive represented section also gives zero. No special material-corner theorem is needed. As a discriminator, changing just the UZ forcing to 1 would give `UZ=1/(3C4)`, so the proof explicitly depends on the actual zero forcing and cannot be replaced by an observed-zero shortcut.

The exact truth error is consequently zero and passes the unchanged actual Absolute bound across the full cover. This proves one genuinely conservative refusal, not a general zero recognizer or all-row result.

## Actual allowance, distinct scaling and forced radius

The positive primary scale is again `3ed8d89883b941b9`. With normalized UZ zero, the positive relative threshold forces the actual Absolute class (`SOURCE_final_case.rs:605–627`). This scale is in the ordinary range; division by 2^64 is exact and its scale-back equals the original value (`:306–321,639–652`). Hence the allowance is exactly

`A_abs = 6993548997640633/21778071482940061661655974875633165533184 m`

`      = 3.2112802105177482027541880587…e−25 m`.

It equals both the captured final bound bits and native row 8's recorded bound bits. I did not substitute a sharper relative allowance for this row.

The represented UZ diagonal is separately derived from the source BᵀDB and actual E_hat/I_hat operands:

`K_UZ,UZ = 12 E_hat I_hat = 2218741290985395146484375/576460752303423488`.

It needs 81 significant bits, fits exactly at verification precision 256, and has leading-bit exponent 21. The actual factor rule `−floor(exponent/2)` gives **s_UZ=−10** (`SOURCE_factor.rs:525–535`), distinct from UX's −14. The unchanged source bridge doubles selected precision 128 and supplies that verification factor's scales (`SOURCE_adaptive.rs:5357–5364,5492–5535`). RX remains −8.

The full structural member pattern still joins all free tip DOFs in the same data block (`SOURCE_assemble.rs:546–568`; `SOURCE_bound.rs:84–121,164–197`). The unit torsional load, G_s/G_hat pair, same arbitrary center, and positive geometry retain the reviewed center-independent residual inequality:

`max|r_RX| ≥ (G_s−G_hat)/(G_s+G_hat) = 1/26214400000000001`.

Captured body-0 B is still `690152861609273/140737488355328`, bound to `ev.certified_bound` by the capture and actual bridge accessor. The source radius computes β=2B, the maximum absolute scaled residual, and outward βω/(1−α), with 0≤α<1. Applying the actual UZ gather scale gives

`L_UZ = 2B · 2^(-10−8)/(26214400000000001)`

`     = 690152861609273/483570327845851688329214473709551616 m`

`     = 16 L_UX = 1.4272026670529583889982922512…e−21 m`.

This exceeds the actual absolute allowance by exactly

`814788685430050882101172033135050752/183331690843750616708748997640633`

or about **4444.341737536674**. The Native recipe preserves that geometric interval and the final gate hulls it with the represented one; maximum endpoint distance is at least the interval half-width (`SOURCE_final_case.rs:477–479,549–554,1061–1079`). Thus current refusal follows for any actual correction center even though the full-cover UZ truth is zero. No center, endpoint or actual native radius was reconstructed.

The inherited lower-bound steps are rechecked against the unchanged copied source and newly decoded capture; only the separately derived UZ scale changes the transfer. As before, the kernel uses a mixed canonical-coordinate norm. The UZ gather and normalization determine the metre interpretation; no standalone physical unit is assigned to B or the common epsilon.

## Projection, consequence and limits

The algebraic native-primary projection applies translation ×1000 then normalization ÷1000 with separate RN64 operations and recomputes all primary maxima. It preserves UZ at +0, scale `3ed8d89883b941b9` and bound `3ad8d89883b941b9`. The current shared-radius lower bound therefore still forces this UZ refusal. This is not a complete executed producer.

The corrected numerical distinction is: **UX has a required-readout point miss; UZ has a proved full-cover conservative refusal.** UZ supplies one sound witness for ROOT's later, separately scoped numerical design decision. This review does not lift the design hold, select a method, narrow a source interpretation, alter output/scale/predicates, or authorize implementation. It makes no inference about the other rows or I47's separate all-row survey, and no public/main availability, resource, acceptance or release claim.

This is the same TASK Type2 reviewer `/root/rv64_conservative_predicate`, continuing directly under ROOT HELP_HUMAN `/root` via delegated-harness-native execution, with no descendants. Receipt **2026-10-03 06:06:16 UTC**; checkpoint deadline **06:13:16**, analysis cutoff **06:20:16**, sealed return deadline **06:26:16**. Useful independent findings were sent at **06:08:37 UTC**, before checkpoint. The continuation brief and needed I36/challenge bytes match `4987f8290f`; continued Root/TASK/Piping origins remain hashed. No other role, workflow or skill was selected.

Only the new `REVIEW_RV64/full_truth_addendum_02` packet was written. A copy of my own prior independent arithmetic script ran only in this new owned directory; the new backchecker adds the full-truth and symbolic-matrix checks. No author checker, oracle, source, compiler, Cargo, model/solver/native runtime or host tooling was executed or changed. No Git/index/API writes or delegation occurred. Every shell call had explicit intended cwd; every Git read set GIT_OPTIONAL_LOCKS=0. Host filesystem permissions are unrestricted; these brief limits were observed by the agent, not falsely claimed as host enforcement.

Exact results, source-location/hash/size manifests, active origins, original-packet preservation, commands and execution limits are recoverable under `_run_records`; SEAL pins the complete packet. ROOT owns integration and consequence. The lane is relinquished at return.
