# RV30 independent conditional kernel review

**Disposition: repair required before treating source_05 as a closed conditional kernel ledger.** One confirmed, blocking owner-capacity finding; one non-blocking phase-accounting clarification. This is a source review at immutable A1 `40129a225d73860ac2a53da9a2fa73869df668f3`, not full E_max, W1, admission, A1 acceptance, or a measured-peak result.

RV30 is a fresh TASK Type 2 direct child of ROOT `/root` HELP_HUMAN Agent0, native task `/root/rv30_k6c_kernel`. I did not implement the reviewed packet and did not delegate. Mechanism: delegated-harness-native descendant. Start clock: 2026-10-01 09:40:33 UTC. Fixed deadline: 10:10:33 UTC. Completion/seal time is recorded separately after the checks and this return are complete.

Aliases: P=projects/chirality-piping; K=P/core/solver/frame_kernel/src/structural/retained; A=K/adaptive.rs; R=the governing RESUME_2026-09-30 directory. Formula references below mean R/I21/source_05/KERNEL_LEDGER.md. Source references mean the immutable A1 blobs, not the mutable A1 worktree.

## Findings

### RV30-F1 — P2, blocking: report resolution capacity is understated

**Formula:** KERNEL_LEDGER.md:116-121 (Report_L and body coefficients), :208-214 (Pass_L), and downstream uses of current Report at :311-352.

**Source:** K/verify.rs:289-317 constructs `resolution_scale` as a borrowed `top.iter().map(... Ok(...)).collect()`, returning `Result<Vec<[f64;2]>,AttemptStop>`. The result is bound at :777, moved into VerificationReport at :1193, and remains alive through R7, both canonical checks, certificate construction, and finish. The call that creates the report Arc is A:3892.

This is the same borrowed fallible collection class already correctly applied to factor.solve and publication rows. Under source_02's pinned implementation it is a GenericShunt with lower size hint zero, not a TrustedLen exact allocation and not an owned iterator that could reuse `top`. For element stride16, a nonempty result retains `G_16(B)=16*max(4,next_power_of_two(B))` requested bytes. The displayed report instead charges `16B` inside `B*(BodyReport_L+16)`.

For B=1, this owner requests64 bytes and the formula assigns16: a48-byte local deficit. More generally the retained-owner deficit is `16*(P_16(B)-B)`. B=5 gives128 versus80. These are direct integer/source deductions, not probes. No claim is made that this deficit exceeds the complete process estimate or a recorded peak: that full estimate is not available here. Padding elsewhere is not an identified replacement for this retained report owner after the pass's scratch drops.

**Repair:** change the report body term to `B*BodyReport_L + G_16(B)`, and replace the corresponding resolution owner inside Pass_L with the same identity/capacity. Keep one resolution identity where the pass transfers it into Report; do not add both as real owners. Reconcile every later Report-dependent phase. Preserve summary clones as exact `16B`: A:2408 clones the logical B entries, so those separate clones are correctly exact. Add a source-bound regression obligation for B=1 and a non-power-of-two body count when estimator implementation is authorized.

**Supporting library source:** source_02/decoded/core__result.rs:2155-2156; core__iter__adapters__mod.rs:170-206, especially :180-185; alloc__vec__spec_from_iter_nested.rs:18-42; alloc__vec__mod.rs:4022-4043; alloc__raw_vec__mod.rs:153-165. All are part of the verified source_02 seal.

### RV30-N1 — P3, non-blocking: identify or explicitly dominate resolution helper scratch

**Formula:** KERNEL_LEDGER.md:206-218 lists recovery, formation_scale and contribution-product helpers but does not name the following scale helper identities.

**Source:** K/verify.rs:297 allocates exact `2B*w_L` for `top`, alive alongside the growing resolution output at :314-316. After it drops, :789 calls `resolution_hats`; :340-360 constructs another borrowed fallible `Vec<[f64;2]>`, then the statement drops it. This temporary is distinct from the retained resolution vector.

The explicit source alternatives are:
- resolution_scale: `2B*w_L + G_16(B)`, plus only the active resolution grow's old request;
- resolution_hats: retained resolution `G_16(B)` plus temporary `G_16(B)`, plus only the temporary's active old request.

I have **not** established an additional phase-total deficit here. Early complete Pass padding includes future full-DOF vectors. At these calls delta_full and w_s do not yet exist. With valid-source `B<=N`, `n=6N`, and `w_L>=48`, one future `n*w_L` term dominates `2B*w_L`; another dominates the hats buffer because `G_16(B)<=64B<=n*w_L` for B>0. Unlike F1's retained report error, this may be repaired by naming that early padding instead of increasing the ultimate phase maximum. State that padding/inequality, or include the helper alternatives directly, before calling the identity inventory complete. Do not describe `top` or the hats buffer as stack-only.

## Independent checked derivation

### Finite schedule and allocation identities

A:3936-4165 supports a finite allocation-site interpretation, without knowing future heap addresses. The precision set is128,256,512,1024. A candidate failure advances c; a failed verification advances c by2; a successful rejected verification is moved into pending and reused as the next candidate. Thus no precision is solved twice in this one-case schedule: attempts and state handles are at most4. At most3 successful verification-shared payloads can enter GroupCache.

A:3603-3640 caches successful Shared even if its later own solve fails; A:3724-3731 creates Solved only on successful own solve. A:3783-3832 caches successful VerifyShared, and a non-budget failed build caches a deep clone of its refusal Vec. A:4014-4019 terminates on verification failure, justifying the packet's absence of previously failed VerifyShared children in a selected result from a fresh one-case cache. Generic preexisting caches/combination operands must retain their error-child parameters.

Cache references and state references are Arc clones, so cache/selected-owner handles do not create another Shared/Solved/VerifyShared allocation. A report is local to one loop iteration and drops before the next attempt. There is no four-report cache. A rejected certificate releases its own publication and radii before pending verification advances.

To instantiate a conservative source bound, name payloads by `(allocation site, precision, current attempt)`, not runtime addresses. A source-backed prefix upper may retain:
`source + group + prep + schedule geometry/scaffolding + sum_{p in {128,256,512,1024}} Shared_p + sum_p Solved_p + sum_{v in {256,512,1024}} VerifyShared_v + attempt children`.
For each construction, include its complete-result padding once and replace its ordinary child with its construction envelope. A tighter phase prefix restricts these sums to preceding/reached slots. A deliberately padded upper may retain all finite slots, while separately allowing stopped-build refusal clones. One current report and decision are then separate phase owners.

That is a finite roster, not permission to import unknown future allocation identities as input. The displayed `K_phi=actual union` is a bookkeeping definition; it must be expanded to such site/precision rows (and explicit copy counts at caller boundaries) for a prelaunch estimator. This expansion is feasible from the checked source, but the packet has not implemented it.

### Numeric construction and transient phases

- Shared members/directional arrays reserve exact source counts (assemble.rs:464-491). Residual-width members are transient; directional_q moves into Shared (A:1323-1347). Factor owns row headers, rows, order/first/scale/screens; `h*w + f*(48+Pivot)` matches factor.rs:520-602.
- Factor.solve's borrowed fallible scaled and returned buffers have capacity C=P_w(f). solve_scaled owns two exact f-vectors. Above caller RHS, retained helper upper `(2C+f)w` and growing-move upper `(2.5C+f)w` follow factor.rs:636-676. These are conservative maxima, not a claim all calls attain them.
- In solve_case_at, RHS, u_free, and evaluated clones remain during residual/correction/fallback (A:1734-1836). Evaluated contains at most4 exact-f clones plus a4-header capacity. Correction delta is a separate returned buffer; it does not replace u_free. Fallback row lists are sequential, but earlier eligible trackers survive (A:1613-1697). The chosen evaluated clone at :1811 overlaps the old u_free. Recovery retains evaluated/RHS/u_free after the residual rows/tracker drop.
- Recovery's returned q rows and6m generalized actions are distinct from12m end actions, s spring actions,3d directional actions and n reaction slots (recover.rs:270-475). formation_scale has18m member helper slots (verify.rs:102-283). One sequential contribution operand Vec, rather than z copies, is supported by verify.rs:660-670.
- Widened operator temporaries end before Uc (verify.rs:462-501). uc_bounds keeps returned c while nl_pass has at/bt/ct; later c/ct overlap two b-maxima and output (bound.rs:669-704). The ledger's4f-wide Nl overlap and later2f+2b alternative are consistent.
- Shift profile and shifted-factor buffers are distinct; rows are deep-cloned (bound.rs:753-787,844-846). The prior shifted factor drops before retry. Consumed current backing remains while next builds, with ct and n_l retained (bound.rs:1046-1128). Factor work and the3f Nl arrays are successive alternatives, not additive. E_result(k) is an exact borrowed map; current's first copy is exact k and later push-grown capacities are bounded by G_start(k).
- Pass/report transfer moves the five row arrays, r_hat and delta; helpers/controls die at the closure edge (verify.rs:1190-1225). Refusal output remains through caller extend; the original G_24(b) plus extension bounded by G_24(2b) is conservative here because the initial capacity is a fresh power-of-two/minimum class and one copied slice extends total length to at most2b (A:3875-3893).
- Summary theta and bound reserve B; resolution's separate clone copies B, producing40B child bytes (A:2394-2416). F1 concerns the original resolution buffer, not these clones.

### Tracker high-waters and drain

A:688-768 separates lazy growth, taken-lazy/table extension, sort, old-table/kept growth, and shrunk-kept assignment. Only the active growing realloc adds its old request; old table plus newly allocated kept is already two real requested-byte owners. Shrink does not allow replacing a historical table bound with P(current length).

In normal production, lazy threshold512 and set threshold4096 are source constants. With lazy stride4304, mu=1: capacities are powers of two up to512. A set offer starts at<=4096 held lazy slots; one noncollapsing grow adds<=256, giving4352 retained and4608 including that grow's old256. During a collapse, taken lazy is already part of the held union while table extension occurs. Standalone residual plus at most4 fallback trackers gives2560 retained /2816 at a lazy-grow sample. These are lazy-only terms; table and tree terms remain additional.

The per-site offer bounds (f for pivot/residual/fallback; total q+2F and<=8B keys for R7) follow the source loops. Table capacity <=max(4,2R_j) is valid with shrink/reuse and required-length high-water R_j. Full held trackers plus final Decision during drain is deliberate padding; it covers the local tracker removed from the map and the undrained map while summary Vecs grow (A:2271-2277). It does not require future runtime population measurements.

### Source, geometry and final owners

Source construction/clone, geometry-before-structure ordering, array children, consumed row/tagged backing, and preparation failure clones were traced through source.rs:352-590, factor.rs:130-219, sparse.rs:62-159, assemble.rs:548-646, bound.rs:84-121, A:908-977,4310-4409. The case/source clone and original source are separate. Pattern positions/tagged buffers survive as described. RCM's independent frontier buffers are real, and Qdeque remains unresolved rather than silently rounded as a Vec.

The geometry witness follows rigid_body.rs:45-247 and Expansion in structural.rs:707 onward. Three translation expansions need at most10 inserted scalar parts, rotation expansions at most1, and one next buffer can overlap the current terms. The candidate upper2N_c+5 follows one initial, six axis candidates, and at most two candidates per non-origin node. This is a count argument; it does not bind the explicit private tree/omitted tuple/queue cells.

Both canonical layouts are sequential temporaries (A:3119-3121,4027,3314). Provisional values are exact24q; publication rows use G_64(q). Raw/coupled binary64 scales coexist with row/body-scale construction but end before radius certification. At A:3342 values are explicitly dropped. Radii are one exact8q allocation; into_boxed_slice adds no second radius allocation for this exact-capacity source. Rejection, error and meter failure drop that draft (A:3223-3301).

finish_selected moves publication/radii and attempts/geometry, creates a fresh summary, selected-record child clone, encodings and evidence children, then the1976-byte outer Box (A:4184-4306). Report/Decision remain during this construction and end before the consumer sees the result. RetainedSolve deep Clone copies publication/radius and evidence/state-vector children but shares success Arc payloads. A Box clone adds its own outer1976. A bare value clone has no outer Box allocation. No report clone is retained by RetainedSolve.

## Conservative padding, limits, and repair route

Deliberate conservative terms are not omissions: complete Shared/Solved/VerifyShared padding during partial construction; exact-k current bounded by G_start(k); holding all earlier trackers during drain; and any explicitly retained Report/Pass duplicate identities. In particular Report and Pass both mention r_hat/delta/resolution during construction. Counting them twice is conservative, but an implementation that deduplicates must preserve the construction owners and F1's corrected capacity. Such temporary duplicate padding does not repair F1's later retained-report term.

This review does not bind the three std-private BTree leaf layouts, omitted nominal/tuple layouts, Qdeque or actual caller array/String capacities, concrete all-fixture descriptors, H staged/prefix envelopes, VR global consumer composition, runtime baselines, or final A1 revision changes. It does not rederive all library contracts, enumerate all finite fixtures, prove compiler allocation elision, or qualify arbitrary multi-case/combination/clone populations. Those remain the packet's explicit cells. No runtime/probe/build/solver/network operation or Git/index mutation was performed; Git reads used GIT_OPTIONAL_LOCKS=0.

Repair in an additive successor packet: fix F1 and state N1's direct terms or domination; make the cache/attempt/report site roster explicit enough to substitute finite descriptors; backcheck affected source phases; reconcile with the final A1 source before later estimator implementation and independent full-bound review. Do not alter sealed source_05. No additional human acceptance or scope expansion is implied by this review.

Checked source/brief hashes, seals and excerpts accompany this return. Source_02, source_03, layout_04 and source_05 seals independently verified with no mismatches. The full retained diff from layout_04 cb13fcf3 to immutable40129 is empty. This carries the retained actual-type evidence provisionally; it does not prove later A1 revisions unchanged.
