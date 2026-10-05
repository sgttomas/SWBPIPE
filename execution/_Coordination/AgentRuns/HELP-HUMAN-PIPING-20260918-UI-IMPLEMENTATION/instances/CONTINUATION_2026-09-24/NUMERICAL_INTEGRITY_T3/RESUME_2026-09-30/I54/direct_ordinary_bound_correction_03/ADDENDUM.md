# I54 corrective addendum — ordinary overlap owners

This corrects the sealed `direct_ordinary_bound_02` packet at `71615e34b8`; its bytes are unchanged. The old packet is not sufficient for reliance without this addendum. Source remains `24af17c47011608d187811024b51807395e4e526`. Container laws retain RV73's qualifications. No final layout, total, fit or M is supplied.

The changes below replace the affected completed-family claims, rather than merely adding qualifications to them. Use them in both `sparse_interactive` and `dense_scrutiny`. Existing source guards and conditional refusal paths still control execution. A sum of separately justified phase maxima is an upper, not a claim of simultaneous residence.

## Capacity and overlap conventions

Retain 02's `s(T)`, exact `X(T,h)`, append `A(T,h)`, push `V(T,h)`, `Hash(T,h)` and stable `Sort(T,h)` laws. Define the f64 push capacity `p(0)=0`, `p(h)=max(4,next_power_of_two(h))` otherwise, and `B(h)=8*p(h)`. These apply to each fresh `next` vector, not to a generic resident vector whose history is unknown. Let `G(h)=4*p(h)` if `p(h)>4`, else zero: an upper on the old next backing during its geometric reallocation.

For an Expansion with at most h scalar-add operations since empty construction (an add_product uses at most two), the active requested upper is

    Exp_req(h) = B(h) + B(max(h-1,0))
    Exp_mov(h) = Exp_req(h) + G(h).

The two requested terms are distinct owners: old self.terms and new next. A third backing belongs only to moving growth of next. If old-self storage is already included in an enclosing resident sum, add `B(h)` requested and `G(h)` moving; do not add self twice. For exact_scalar on a resident expansion of length at most t, the **additional** requested storage is `8t+B(t+1)`, moving adds `G(t+1)`, and the original self remains in its enclosing owner.

Proof: FK/structural.rs:729–752 creates `next=Vec::new`, loops over borrowed self.terms, pushes at most its length plus one, and replaces self only after completion. :755–775 calls add at most twice; :785–789 clones before subtracting the rounded value. Early arithmetic errors leave the same bounded prefix live; they do not release old self before the error.

## Corrected audit, residual and rigid terms

| Family | Corrected expression and lifetime | Pinned source |
|---|---|---|
| row_sources | While the active row owns `X(String,l_i)+sum(source-ID lengths)` and all previously produced LoadFidelityRows remain, add `Sort(String,l_i)`. Across the loop take `max_i Sort(String,l_i)`, not its sum. The force ledger, by_dof, contribution sums, row parts and residual remain enclosing owners. Sorting String compares borrowed contents without new String copies. Dedup can reduce length while retaining vector capacity. | FK/structural.rs:1096–1101,1126,1201; dense audit :1037–1094; sparse audit sparse.rs:1815–1867. |
| Contribution sums and differences | Keep 02's resident `2Z*s(Expansion)+8(Ps+Pd)` (dense replaces headers as before), rounding `V(ContributionRounding,Z)+8(2C+Z)`. Add **requested** `B(d+1)` for an active replacement next, and **moving** `G(d+1)` plus the old rounding-vector backing. The resident row includes current self. A fresh standalone sums construction similarly needs `Sums_resident+B(d)` requested and `G(d)` moving. | FK/structural.rs:820–918; sparse.rs:1183–1214,1515–1606. |
| Prescribed RHS delta | Add `Exp_req(h_delta)` / `Exp_mov(h_delta)`, where `h_delta=2*min(C+Z,k*(d+1))` bounds one row's products. The parent sums, differences and rounding Vec remain. For the named entry all prescribed values are constructed zero, so add_product returns before allocating and this delta child is zero. That source premise is not imported into arbitrary prescribed input. | FK/structural.rs:903–909 and add_product :761–762; sparse.rs:1578–1596; PP prescribed zeros remain as in 02. |
| Intended-action residual | Keep one fresh contribution-sums construction (including its replacement next), existing original residual rows, and the growing returned ResidualRow Vec. Replace the one-child/moving-pair description with `Exp_req(h_I)` / `Exp_mov(h_I)`, `h_I=1+2*max_i C_i`, or the looser `1+2C`. | FK/structural.rs:939–1009; completion :1720–1758. |
| Load-fidelity residual | Keep by_dof, restrained bitmap, fresh sums construction, parts Vec and previously returned row/source children. For an unrestrained row use `h_i<=2*C_i+2*l_i`; for a restrained row use `h_i<=1+2*l_i`. The old global `1+2C+2l` remains a safe loose h. Apply Exp_req/Exp_mov and the row_sources sort above. `row_sources` occurs after residual construction; summing their maxima is conservative. | FK/structural.rs:1137–1203; sparse.rs:1815–1867. |
| Rigid original-coordinate witness | The resident per-node children remain at most `3B(10)+3B(1)=480` bytes. Delta resident is `3B(2)=96`. While forming motion, add next at most `B(10)=128`, plus `G(10)=64` moving. While exact_scalar runs, add clone at most 80 and next 128, plus 64 moving; the original remains in the 480n resident sum. | FK/rigid_body.rs:195–250. |

The original named rigid constant **2,992 bytes + 24*s(Expansion)** has sufficient source-proved slack for these internal Expansion buffers: its 96+208=304 scratch allowance exceeds both construction moving `96+128+64=288` and scalar moving `80+128+64=272` (and delta replacement `96+32=128`). This is a proof about that conservative sum, not a reclassification of next as a moving shadow. For whole rigid-family movement, add the old candidates backing (384 bytes) and old original-ground-rows backing (192 bytes), yielding **3,568 bytes + 24*s(Expansion)**. Adding these distinct phase maxima is intentionally conservative. Parent BodyEvidence vectors and a returned mechanism direction remain separately present as in 02. The straight qualified family uses assess_rigid_body; mixed W4 Expansion paths are not brought into this family's completed claim (SA:1290–1322).

For the named case the contribution histogram is 141 entries of multiplicity 1 and three of multiplicity 2. Requested audit children become **12,752 bytes**, replacing 12,720; the 32-byte next is requested. With d=2 there is no next reallocation after the initial four-slot allocation, so moving adds only the previously stated old rounding backing `128*s(ContributionRounding)` to this corrected request. Headers remain `288*s(Expansion)+256*s(ContributionRounding)` (plus 24 dense row headers).

The source connectivity gives row contribution totals 12 or 13; nodal terms occupy distinct DOFs 9,10,11. Thus `h_I<=27` and `h_L<=26`, each giving **512 requested / 640 moving bytes** for its active residual Expansion. The old global h values 295 and 301 give **8,192 requested / 10,240 moving**, not 8,192 moving. A fresh sums set has **4,640 requested / 4,640 moving child bytes** (4,608 resident plus 32 next). These are helper components with their parent owners still live.

For generic row_sources, RV73-accepted S1 gives zero heap for n<=20 in the non-size-optimized implementation; otherwise q=max(ceil(n/2),min(n,floor(8_000_000/s(String))),48), with heap zero if q<=floor(4096/s(String)), else s(String)*q. The shown 64-bit branches admit the configuration-independent upper s(String)*max(n,48); n<2 always returns without a heap buffer. Use the actual final configuration, or that conservative bound. Its stack buffer and helper frames stay in stack qualification. For row_sources the named maximum l_i is 1, so its stable-sort heap scratch is zero; its input source String and Vec remain live. Generic l_i retains the explicit sort law. No named zero excuses the generic term.

## Ordinary stress recovery and both extrema helpers

PP/lib.rs:4602–4612 retains two endpoint StressRecoveryResults and a `Vec<(&'static str,StressRecoveryResult)>` of at most three station results. Its exact slice-map collection has at most three tuple slots. Each result owns separate status and finding vectors; the inline endpoint result headers are stack, and station result headers are inside the tuple backing. The five results survive finite checks, diagnostic creation, output-row construction, summary_values, legacy extrema and component-modifier inspection (:4613–4877).

`recover_section_stress` (:11328–11356) constructs an exact one-slot input status Vec. SR/lib.rs:493–542 creates another output status Vec via collect_statuses (:1060–1095), containing MechanicsSolved and HumanReviewRequired. Input and output coexist during the call. They are not a shared vector. For this no-pressure family, three divide_optional channels and one torsional_shear channel can each return at most one finding (:880–927); statuses are valid, so no status finding is generated. The four-finding upper preserves non-finite/invalid recovery prefixes. Pressure is None from PP:13179–13204 under the admitted nodal-only family.

Let `StressMessageLen` be the maximum message byte length among the actual static subject/suffix templates in SR:968–980,1002–1044. Let J=17+max(8,2*StressMessageLen), for subject_id (at most 17 bytes) and the formatted message. The arithmetic file enumerates those source literals: StressMessageLen=57, message capacity upper=114, and J=131 bytes. A layout-independent safe output child upper is

    StressChildren = 8*s(AnalysisStatus) + 4*s(StressFinding) + 4J.

Eight status slots cover all minimum-capacity branches for two statuses; four finding slots cover the at-most-four findings since StressFinding is not byte-sized. No layout measurement is assumed. Only one current input status vector needs `s(AnalysisStatus)`.

The summary starts from a filtered two-value array, appends at most three station summaries, then at most one legacy extremum (:4714–4743). Its high-water six gives 64 requested / 96 moving bytes. The legacy `straight_summary_extrema` (:9805–9896) owns its **own** exact two-f64 boundaries (16 bytes); sorting two values uses no heap scratch. Its components closure calls straight_section_resultants and then recover_section_stress. One returned stress exists at a time, while the outer five remain; the three initial samples and at most eight stationary candidates are sequential, not eleven simultaneous results. StraightPipeError and its forwarded FrameKernelError contain only scalar/static-reference variants (straight_pipe/lib.rs:317–344; FK/lib.rs:319–374), hence no heap child; its later to_string and diagnostic copy remain explicit text owners. This is separate from the certified helper invoked earlier for preview_record (:4587–4594,9708–9789).

A conservative **supplement** to 02's `256 + 4*s(StationResultants)` straight-recovery transient is

    StressSupplement_req = 3*s((&'static str,StressRecoveryResult))
                           + 6*StressChildren + s(AnalysisStatus)
                           + 64 + 16 + 160.

The extra 160 bounds the legacy closure's separate station-helper phase. It is deliberately summed with the outer helper maximum; no repeated-helper overlap is claimed. Numerically this supplement is **3,384 + 3*s((&str,StressRecoveryResult)) + 49*s(AnalysisStatus) + 24*s(StressFinding)** requested bytes. Together with the previous primitive/station transient, the constant is 3,640 and the four StationResultants slots remain. A safe moving addition is `32 + 8*s(AnalysisStatus) + 2*s(StressFinding) + max(8,2*StressMessageLen)`, covering summary growth, one active output status/finding growth and one active formatted finding message. Input-status exact allocation and station tuple exact allocation do not need reallocation shadows.

Stress findings remain inside the five outer results while their message is cloned into diagnostics. At most 20 findings per member reach these diagnostic loops (:4630–4658). Their retained Diagnostic children and one active ID-format/stable_suffix/location-replace temporary are listed separately in the arithmetic/residual roster; do not drop the original finding message when adding its diagnostic clone. The sixth legacy helper's findings are dropped inside components, with its MissingInput error then formatted separately.

The certified maximum's original Node/spans coefficients remain unchanged. Its returned success has only scalar fields (SR/elastic_extrema.rs:27–39); failure is formatted to an owned String and stored in preview_physics::MemberRecord.maximum (PP:4590; preview_physics.rs:156–164). Up to m such prior returned error Strings survive later helpers and suffix staging; retain `sum E_max_capacity`, not only the current helper error. Error String grammar for StraightPipeError Display and ExtremaError Debug is still a precisely named text obligation, not zero.

## Formation guard Bodies, records and later maps

This is PP/formation_guard.rs, separate from FK's 128-bit formation helper and from typed input capacities. `formation_bodies` is called after the source hook and force construction, before solver selection (PP:3533–3543), and the resulting two Vecs survive through recovery and its guard.

Let b<=n be connected bodies, q=m+n+g entity insertions and I_entity the sum of member/node/support ID byte lengths. PP:1219–1242 supplies an exact coordinate Vec and an edge chain Vec; a conservative A edge upper is sufficient. Bodies::new (formation_guard.rs:94–144) retains parent, ids HashMap, body_of_node, low, high and extent during construction:

    Bodies_resident = 8n + 8b
    Bodies_build_req = 40n + 56b + A((usize,usize),m) + Hash((usize,usize),b).

The 40n is caller coordinates 24n plus parent 8n plus body_of_node 8n; 56b is low/high 48b plus extent 8b. Add an old edge backing and old ids table for a conservative moving upper. Exact constructor vectors do not get duplicate reallocation shadows. The returned Bodies moves its two Vecs; parent/ids/low/high and caller coordinates/edges then drop. In the nodal-only family all formation slots are None, so load_row_finding returns before formation_rows, restrained set or row_scales allocate (FK/load_ledger.rs:307–309; formation_guard.rs:307–309); this does not omit Bodies, which was already built.

RecoveryRecord construction (:4411–4426) retains up to m member IDs and two error String clones per record. While constructing one record an original `bounds` error String also coexists with both clones. Add **one active original bound-error capacity** beyond 02's retained two error clones per member. Results, diagnostics, preview records and Bodies remain live.

After the member loop, if recovery_records is nonempty, `formation_entity_bodies` clones member/node/support IDs into a fresh map (PP:1193–1214,4881–4888). `moment_scales` builds a maxima map and collects a second output map while the consumed old table still exists (formation_guard.rs:380–420). A safe requested phase is

    Bodies_resident + Hash((String,usize),q) + I_entity
    + Hash((usize,(f64,f64)),b) + Hash((usize,f64),b)
    + existing RecoveryRecords/results/diagnostics/preview parents.

The temporary excluded curved-member set is empty in this straight family. For moving, add the old entity, maxima and result table uppers conservatively; no node count or capacity accessor is invented. Even duplicate entity IDs cannot exceed q insertion attempts or I_entity cloned bytes.

For the named geometry b=1, q=7 and I_entity=47 bytes. Bodies resident is **24 bytes**; construction is **136 + A((usize,usize),1) + Hash((usize,usize),1)**. The late maps contribute **47 + Hash((String,usize),7) + Hash((usize,(f64,f64)),1) + Hash((usize,f64),1)** beyond resident Bodies and existing parents.

RecoveryFinding (formation_guard.rs:433–473) has at most 2m fired Strings, a copied prefix of at most min(6,2m) Strings and Vec, a joined String, optional count suffix, and sentence. These coexist with records/scales and, during amend, the old diagnostic message and its replacement (:485–511). The residual roster spells out its finite text/owner expression, including the temporary integrity diagnostic ID, cloned prefix Strings, exact join allocation and old/new message. Its named String-owner polynomial is 1022+16*F_len+2*old_diagnostic_message_len+8*s(String), plus the separately open formatter helper/frame term. It is not covered by H_formation128 or the nested-input term.

## Effect on composition

Replace the common typed-core audit/residual/rigid components with the rows above and add the generic row_sources sort. `EvaluateW2` inherits that corrected Core_mode in each of its at-most-two evaluation maxima. W2's standalone return-tail constant 12,608 and direct unscaling replacement `4F+8(d+1)` remain unchanged: those replacements are iterator collections, not Expansion::add; Tail contains returned row/source clones and no row_sources invocation. A complete W2 claim is nevertheless conditional on the corrected common core.

Add Bodies construction/residency and late-map phases to the ordinary baseline/recovery composition. Add the stress supplement and returned finding/error owners to ordinary recovery, diagnostics and suffix parents as indicated. These are existing ordinary owners, not future I51 trace or receipt owners. Full totals remain withheld for the exact outstanding source/build facts in RESIDUALS.md.
