# RV30 source09_04 — finite caller-to-kernel descriptor review

**Disposition: no blocking finding in the reviewed source-side descriptor/capacity proposal. It can feed the conditional kernel formula.** This is source-derivation usability, not implementation, admission, full E_max or global caller closure. The outstanding prelaunch/data and caller-composition conditions below remain required.

Same independent TASK Type2 RV30, native child `/root/rv30_k6c_kernel`, reporting to ROOT `/root` HELP_HUMAN Agent0; no delegation. Actual start2026-10-01 13:56:28 UTC; deadline14:16:28 UTC. Completion after checks is recorded in VERIFICATION.json. Direct parent follow-up supplied this20-minute source-only scope, then explicitly asked for the smallest justified route for the two accessible caller-layout cells. Prior instruction/skill bindings remain at ../kernel_01/INSTRUCTION_BINDING.json.

Reviewed source_09 records at K6C commitd1179dd72a9a706cb44d31baea867c579baaf1bc:
- packet seal `6d2d3dae877a3cf756cd32697732aa72a3f9bca33de243d4dda7e9d8410a9195`;
- DESCRIPTOR_MAP.md `e56fb9767188f161c96aaeac0c221b004a8ad065719248669bbb9907716d4c33`;
- product basis40129a225d73860ac2a53da9a2fa73869df668f3.

Every source09 payload was hashed before reliance. All28 referenced product/data/review inputs have matching recorded frozen hashes. H/kernel reads use the already independently bound layout08 archive where available; VR current files match the frozen40129 recorded bytes. No Git command was invoked. INPUT_BINDING.json records the actual matched read locations.

## Checked source capacity boundary

DESCRIPTOR_MAP:99-136 correctly distinguishes H and VR construction classes.

H adapter.rs:39-85 uses borrowed exact maps for nodes, members and loads, an exact range map for stations, and fixed-template source IDs. restrained_dofs (models.rs:99-107) is a borrowed lower-zero flat-map/filter collect, safely G_8(r). Its following owned map expands actual usize8 to Constraint16. The pinned in-place eligibility condition fails; the exact-length nonfallible map destination can be16r, while the old G_8(r) input is a real transient. This is correctly separated from retained SRC0_H.

VR cases.rs:470-533 clones node coordinates exactly, then pushes separate member, axis-spring, directional-spring, constraint, load and station Vecs from empty. Thus the G_s classes in SRC0_VR are appropriate. Each load source-ID String is a separate exact-length clone, not a reused raw JSON String allocation. The axis/directional split is real and cannot be replaced by the old combined spring count.

PrimitiveSource moves its input arrays and retains their capacity. It creates exact constrained16n and body-index4N arrays. Validation sorting/dedup does not authorize shrinking, and cancellation does not delete load terms. PREP_source_clone is a separate deep clone with logical-length Vec/String children; its inline source header is already in ArcCasePrep. The explicit formulas do not add an invented source Box.

The source stage includes adapter construction, so retained SRC0 is not itself the phase maximum. The packet correctly retains the restrained-index transient and one active formatted-ID grow in H, plus constructor sorting/chord/union-find helpers from the existing kernel ledger. VR fresh-push growth likewise uses the existing one-active-old-buffer rule. I did not substitute phase slack for an omitted owner.

## Fixed input strings

The four installed1.97.1 source pages were independently hashed and decoded in memory; all full decoded hashes match STRING_CONTRACTS.json. Checked paths:
alloc::fmt::format and Arguments::estimated_capacity;
String Write/push_str/byte-Vec append;
String Clone;
str ToOwned and specialized ToString.

For the fixed formats in scope, initial format capacity is at most twice the literal bytes, hence at most2L. If any subsequent grow occurs, its old capacity is below the required final-length bound L and RawVec chooses at most max(8,2L). Therefore F(L)=max(8,2L) is a sound retained upper for these nonempty templates. One growing-old request is at most half its corresponding new request. This is a pinned implementation deduction, not a claim of exact capacity.

H's `k6:{global_dof}` and one-letter node/member prefixes have fixed decimal grammars; the n-based ID upper is conditioned on in-range DOFs in the accepted fixed inputs. Clone/str-to-owned paths use exact logical UTF-8 byte lengths. No arbitrary input String capacity or raw-parser capacity is being inferred from length. Source validation failure handling remains distinct, and the fixed33 scope is not silently extended to arbitrary invalid caller models.

## Structural and helper descriptor bounds

Checked against frozen assemble/source/factor/ledger and the H/VR count adapters:

- Validated distinct member endpoints produce78 upper-triangle contributions and144 symmetric raw row pushes per member. Axis/directional springs contribute1/6 to U and1/9 to raw row totals. Per-global-DOF p_g is12 per incident member, axis multiplicity plus3 per applicable node/kind directional spring.
- z<=min(n²,144m+s+9d) preserves duplicate contributions while bounding distinct matrix entries. No numerical coefficient or zero entry prunes the structural graph.
- n=6N and f=n-r require validated unique in-range constraints. Before validation, f<=n is a safe input upper; invalid sources use constructor/refusal accounting rather than successful-solve algebra.
- Bodies arise from member connectivity, not ground springs; B<=N includes isolated nodes. Free blocks satisfy b=0 at f=0 and1<=b<=f otherwise.
- Structural skyline h<=f(f+1)/2 and free adjacency sum<=min(f(f-1),sum p_g) are valid input-only bounds. H's fixed models have nodes incident to members, matching its node-block graph construction; the packet does not generalize that H exact-count implementation to arbitrary isolated-node models.
- Layout grammar q=7N+12m+6t+s+3d+r+2u and v<=min(l,n), J<=68v match their source. Zero and cancelled load DOFs still enter v. The exact source09 v metadata is a valid tightening of min(l,n), not a numerical cancellation result.
- P_s(k)<=mu_s+2k yields each displayed aggregate capacity inequality, including empty indexed lists as harmless padding. NotAssessed has at most2N_c tuples; the layout08 exact tuple stride8 justifies its instantiated inequality. Grounds and directional-list growth retain pre-dedup/source populations and the at-most3 appended axes.
- Optional body/block/row/neighbor histograms tighten the bounds. N_c<=N, f_c<=f and the displayed sums provide finite substitutes without needing future geometry success, tracker populations, runtime allocation addresses or solver outputs.

Checked arithmetic/conversion is a future implementation obligation, not proven by nonwrapping Python integers. Keep per-field exact/upper provenance and consistent substitution when an owner is moved or a padded term is replaced. Descriptor unavailability must not silently become a smaller estimate, a product rejection or an altered admission decision.

The existing exact count paths allocate/validate source and graph data. The packet explicitly distinguishes their pre-solve use from an estimate needed before those helpers run: the latter must use input-only uppers and separately account count-stage construction. This avoids a future-allocation or observed-heap dependency.

## Census and actual-data checks

I read metadata directly from all ten hash-bound JSONL family files and the existing H counts file, without calling model builders, a generator, graph/RCM/profile code or a solver.

Independent results:
-33 H structural rows match the committed rows field-for-field; no heap, elapsed-time or estimate field enters their exported descriptor subset.
-213 unique VR identities match the census order and basic provenance.
-201 embedded model array/String/scalar metadata records match, including member endpoint and constraint/load index ranges checked from input arrays.
-12 external models remain present with declared model/source hashes; no synthetic arrays or file lengths were substituted.
-88 embedded axis-spring cases,22 directional cases,38 cases with multiple authored terms at a DOF, one zero-load model, and8 declared refusals match.
- The three N-m>1 forced-multibody identities match the packet.
-All33 H and201 VR arithmetic rows independently recompute. The VR ledger upper intentionally uses its input-derived exact v, giving68v even on multi-term cancellation cases.
-VR/tests/scale.rs:40-71 establishes193 as the separate factored-CI obligation; the201/213 census does not replace it or discard the8 geometry refusals.

CENSUS_BACKCHECK.json and STRING_BACKCHECK.json preserve these checks. An initial reviewer equality check used coarser68*min(l,n) rather than the packet's tighter68*v for38 cancellation rows and therefore failed; I corrected the review calculation to the declared exact v and all rows matched. No packet, source, criterion or runtime work was changed.

Exact VR B/b/z/h/histograms were not independently recomputed here. Their source algorithms and conservative substitutes were reviewed; the packet correctly leaves those per-case exact data absent. The external12 file hashes remain a future input-binding obligation before typed lengths are used. Current committed metadata is not a stand-in for those file bytes.

## Finite caller-model rows and smallest layout route

No blocking concrete error was found in the caller-model rows checked at DESCRIPTOR_MAP:220-245:
- RF-LARGE pushed48-byte nodes either retain owned input backing or have a generic exact destination no larger than G_48(N); both buffers may be padded during construction if reuse is not assumed. Node labels move; member labels are separate exact clones in the64m borrowed-map destination.
- H restraint tuples count restrained nodes, not constrained DOFs; loads retain their builder/filter capacity class.
- DEC053 node conversion is owned fallible Option collection. The byte envelope max(G_48(N),N*S_coord) safely permits inherited source backing or generic destination. The original-coordinate/destination overlap during model construction stays a caller-phase concern, not another permanent kernel input.
- H frames are a borrowed fallible collect, so G_136(m), not136m.
- VR parsed node names/coordinates are pushed Vecs; members/springs/constraints/loads/stations/omitted names are borrowed nonfallible exact collections. Verified layout04 strides support112-byte Member,88-byte SpringSpec and48-byte load tuples. Text-to-owned children are exact logical lengths. Cloned Models and SourceParts are distinct owners.

**No additional standalone layout measurement tranche is needed for the two accessible caller types.** At future authorized implementation, evaluate the exact expressions in the estimator's own compiled target/configuration:
`std::mem::size_of::<Option<[f64; 3]>>()` for S_coord and
`std::mem::size_of::<(u32, usize)>()` for VR constraint tuple stride.
Use checked byte multiplication and retain the source/type binding. Neither is an inaccessible std-private specialization. This is sufficient as a source-bound compile-time parameter; no guessed constant, copied representation or similarly shaped type is required. If a tighter in-place eligibility proof later needs alignment, `align_of` of those same accessible types is available in that implementation. The current two-branch conservative envelope does not require another probe. This route is advice under ROOT's explicit follow-up, not an implementation grant.

## Limits and return

The reviewed source-side descriptor contract is suitable for conditional kernel-formula integration. It does not implement an estimator or establish every concrete prelaunch descriptor. Maintain source validity/provenance, the external12 file/hash bindings, checked target/u32/Layout conversions and source final-A1 reconciliation.

Unclosed global/caller cells remain explicit: H persistent model/frame/args/saved-attempt/runtime and staged/prefix formatting; VR raw parser/Case/control/comparison/record/IO/runtime/RCM/parity ownership; complete byte/phase composition. No generic serde or process-format audit was attempted. Private BTree alignment remains unbound; the already backchecked request-byte facts do not establish it. A1 changes affecting types, constructors or lifetimes reopen the relevant bindings.

Only R/source_review_RV30/source09_04 was written. No Rust, model, generator, graph execution, probe, build, solver, expected numerical output, Git/index operation, network, installation or delegation occurred. Existing seals were preserved. This return confers no admission, numerical, product or full E_max acceptance.

