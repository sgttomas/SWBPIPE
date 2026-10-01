# Shared seam, launch binding and later validation plan

This is a design/plan only. ROOT selected VR -> H's existing library; no code, lock, runner, record, admission or product edit is made. A separate implementation grant and final-basis review are required.

## Exact module and Cargo seam

Proposed new H module:
  projects/chirality-piping/core/solver/performance_harness/src/k6/w1/envelope.rs
Export it from the existing src/k6/w1/mod.rs. This is the sole checked one-case kernel-envelope implementation. H/src/k6/w1/counts.rs becomes the H descriptor/compatibility adapter; VR/src/scale.rs becomes the VR descriptor/caller-composition adapter and deletes the copied kernel constants/phase loop. Neither copies the shared equations.

VR/Cargo.toml adds:
  open_pipe_stress_solver_performance_harness = { path = "../../../core/solver/performance_harness" }
The source-only manifest closure in DEPENDENCIES.json is acyclic with that edge. H already depends on FK, nonlinear_integration, diagnostics and sparse_direct; none of its local closure depends on VR. VR already shares FK/SD. Seven existing local0.1.0 packages enter VR's lock closure: performance_harness, nonlinear_integration, diagnostics, curved_bend, linear_supports, nonlinear_supports and primitive_loads. No new external package is proposed. Later Cargo.lock edits must be reviewed from actual offline resolution; VR/Cargo.lock changes, while H/Cargo.lock should remain unchanged unless an independently justified resolution difference appears. No lock was generated now.

Required later source/export/write locations:
- H/src/k6/w1/envelope.rs (new), mod.rs export, counts.rs adapter;
- H/src/k6/counts.rs and H/src/bin/k6_observe/main.rs for launch-aware composition/counts emission/backstop, if needed by the concrete adapter;
- H/src/bin/k6_observe/w1.rs stale KF3 comment and in-solve prefix regression support;
- VR/src/scale.rs, VR/examples/vk_scale.rs, VR/Cargo.toml/Cargo.lock;
- exact tests/runner/count records listed below.
The original implementation fence must be explicitly extended for the new module/export/main/counts/lock paths where necessary. No FK or retained-kernel edit is proposed. H/src/lib.rs already exports k6; its library receives no #[global_allocator], counter reset, cap, timing or observer hook. Existing binary-only K6Alloc/VkAlloc registrations remain.

## Proposed public fields and ownership boundaries

The following is an interface specification, not maintained Rust implementation. Inputs are pre-solve counts/lengths with named constructor/proof provenance, never observed heap/RSS, numerical run capacities or a free allowance.

KernelCounts fields (usize, checked):
  nodes,members,axis_springs,directional_springs,constraints,load_terms,stations,
  support_groups,bodies,free_blocks,dofs,free_dofs,pattern_entries,profile_entries,
  quantities,distinct_loaded_dofs,support_id_count,load_id_utf8_bytes,
  max_load_id_utf8_bytes.
Also retain raw_constraint_terms (before dedup) and raw_support_id_count where a constructor consumes a potentially larger input backing. H derives the former from its actual restrained-node input, not the deduplicated r alone. These are source populations, not byte allowances.
KernelDescriptor privately stores validated KernelCounts plus SourceConstruction.
SourceConstruction has the two existing contracts HAdapterV1 and VRModelV1.
It determines source-array/ID construction capacities internally from counts and input string lengths. It is not an arbitrary user-supplied capacity/bytes override.
Derivable values U,Pi,F,J upper, encoded lengths, aggregate neighbor/body/component uppers and queue high-waters are computed once in envelope.rs. Validate n=6N, f<=n, counts/ranges, q formula, nonnegative/safe widths, all checked products/ceilings/capacities. Existing validated counts or explicitly proved uppers may supply z/h/B/b; a scalar cannot silently claim a graph fact without the adapter's input/count provenance. Aggregate source09 uppers avoid allocating descriptor histograms.

MetricBytes { requested:u128, moving:u128 } distinguishes the two counters.
KernelPhase is the fixed K01..K20 enumeration.
PhaseBound { phase:KernelPhase, bytes:MetricBytes, precision:Option<u32> } is inline.
KernelEnvelope fields:
  all_precisions:MetricBytes;
  selected_at_128:MetricBytes;
  phase_maxima:[MetricBytes;20];
  shared:[MetricBytes;4], solved:[MetricBytes;4], verification:[MetricBytes;3];
  passes:[MetricBytes;3], reports:[MetricBytes;3], decision:MetricBytes;
  retained_base:MetricBytes;
  dominant_requested:KernelPhase, dominant_moving:KernelPhase;
  proof_basis:KernelProofId.
Use fixed arrays/scalars and borrowed input metadata; do not create an uncounted heap-heavy estimator graph. The four/three source-site slots are finite. Header minimum capacities remain correct even on shorter paths.

kernel_envelope(&KernelDescriptor,&VerifiedKernelProfile)
  -> Result<KernelEnvelope,EnvelopeError>.
VerifiedKernelProfile has private fields/factory and a named accepted artifact/source/compiler/target profile. It supplies actual kernel type/request facts; callers cannot invent leaf sizes or aggregate fixed bytes. Public sizeof expressions are evaluated for actual public types. Profile transfer is rejected/unavailable until final basis matches; compile-target checks alone do not establish correspondence to the historical artifact. A future implementation must decide the exact build-metadata carrier under ROOT's grant, not silently turn this proposal into an already verified factory.

EnvelopeError variants: ArithmeticOverflow, InvalidDescriptor, UnsupportedInvocation,
MissingProof(ProofCell), BuildBasisMismatch. ProofCell names the specific missing fact.
Do not encode a missing symbol as zero or accept a caller-chosen overhead. A partial diagnostic subtotal may be reported as such, but cannot become the admission estimate. Single-case scope matches the original H/VR calls; arbitrary selected clones/combinations require their own finite descriptor and are not claimed by this API.

H adapter:
  describe_h(&K6Model,&W1Counts,&HLaunchPlan) -> Result<KernelDescriptor,...>;
  h_envelope(&KernelEnvelope,&HCallerDescriptor,&VerifiedHProfile)
     -> Result<HEnvelope,...>.
HCallerDescriptor fields name actual model/label/frames construction, model ID bytes,
counts-path bytes, rows-path bytes, pass number, repeats=5, prefixes flag and J<=9.
HEnvelope carries source_stage,solve_stage,prefix_outer,all_windows,selected128
as MetricBytes, plus the separately named model/Args/runtime/saved/prefix terms.
No whole-process H sparse/JSON tail is added to these staged windows.

Bind counts/input identity through a sidecar manifest containing model/source/counts/launch-plan hashes; do not silently add non-estimate fields to immutable historical numerical records. A missing proof blocks checkpoint/implementation release of a numeric admission value; this is not a newly selected runtime verdict or waiver.

VR adapter:
  describe_vr(&Case,&Model,&Counts,&VRLaunchPlan) -> Result<KernelDescriptor,...>;
  vr_envelope(&KernelEnvelope,&VRCallerDescriptor,&VerifiedVRProfile)
     -> Result<VREnvelope,...>.
VRCallerDescriptor fields include exact Case/model raw identities and lengths,
embedded/external branch, retained model/Case children, row/control/key/string
descriptors, Nc, optional s_full, current fixed-source JSON shape, exact-operation
descriptor classes, and separate SD counts/upper policy (including stored-zero L
and hs<=f(f+1)/2 fallback). VREnvelope carries precut,source_controls,kernel_lane,
comparison_record,report_record3,rcm,sparse_parity,summary and global MetricBytes.
The seven private consumer request profiles and format/runtime leaves belong
to a separately qualified VR profile, not the kernel or a raw overhead number.
Their current missing proofs prevent a complete numeric VR return.

Compatibility mapping: use the complete composed moving upper for legacy max/sel128
admission fields, since moving dominates requested and existing comparisons/rho
include move-model heap. Keep requested and phase values as explicit diagnostics;
this strengthens the envelope without changing the numeric admission predicate.
Do not emit either legacy field from a subtotal while a required proof is missing.
This mapping is proposed for implementation review, not an accepted record change.

## Planned launch descriptors

PLANNED_LAUNCHES.json binds the33 model identities and66 W1 process/pass rows
from the committed K6b packet, including original run_id/order/tier. No runner,
model builder or graph was executed to create the plan. It gives concrete
repo-relative argument strings and their exact retained byte lengths:
- launch cwd=<CANDIDATE_REPO_ROOT>;
- counts argument=P/core/solver/performance_harness/observations/k6b/counts.jsonl;
- first-pass rows argument=T3/IMPLEMENTATION/K6C/_run_records/b3/<original_run_id>.rows;
- --model actual ID,--mode w1a,--heap-cap-bytes8053063680,
  --repeats5,--entry-repeats5,--first-repeat-limit-s600;
- budget540 at10/100 and1740 at1000/10000; first W1 pass adds
  --dump-published <rows> --w1-prefixes, second does not.
These relative strings are a concrete proposed launch choice. ROOT may choose
other actual paths, but their byte descriptors and counts/launch binding then
must be recomputed before reliance. Candidate binary/archive path, compiled
manifest path and final source SHA remain build/launch facts, not invented here.

The counts artifact must describe the intended launch envelope, not just its
own counts-only process. The H adapter must use the maximum of the two planned
pass/path/prefix cases for a per-model counts row, or retain explicit per-plan
estimate fields with reviewed compatibility mapping. A counts-only run's shorter
argv cannot silently bound later first-pass row-path and saved/prefix owners.

VR original tiers remain V1/V2/V3: six CHAIN/TREE/CONT AX/ROT frames at100/1000/10000.
RF-LARGE10 is still a fixed comparison/count point; this plan does not invent a
new timed tier. --case and heap cap are unchanged. External --model-file paths
must be the exact12 I23-bound files (or byte-identical qualified copies); source14
records their hashes. Bind actual argument byte lengths and compiled VR manifest
directory before a global prelaunch claim. --counts-only remains a distinct
earlier branch with its512MiB count cap; it does not stand in for global history.

W1-T4 specifically preserves six10000 models, two W1 passes/model, five repeats
per process (early source/time stops still recorded), first-pass prefixes, and
the original ABAB/BABA rotated interleaving with sparse:12 W1+12 sparse processes.
It remains ROOT-conditional after W1-T3 prefixes. No n² mode at>=10000 is enabled,
no threshold changed, and no required measurement is waived by deferral.

## Historical comparison and admission replay

HISTORICAL_COMPARISON.json binds66 H W1 rows,18 original V-K B rows and18 KF3 B
rows with same-binary metric labels. Existing peaks are only comparison evidence.
H summary means staged source/solve; prefix uses its outer edge. VR summary is
global parse/output/RCM/parity history. Compare each with its own composed fixed
term and matching requested/move envelope.

The corrected KF3-B2 record demonstrates:
- TREE10000 AX/ROT principal shift delta+10799688 B;
- old H fixed term must not be compared directly with a vk_scale measured peak;
- original VR port was below the recorded global peaks, while final K6b kernel
  terms plus the VR fixed term left6970722/7511329 B slack. This is historical
  like-for-like slack, not proof of the missing phase term or a new allowance.
The historical code-derived H-shaped total3021565490 B (AX) exceeded its old
3010765802 B estimate because of the shift term; ROT differs by the same delta.

What is possible now: exact source/count/layout subterm deltas; existing-record
identity/metric inventory; the conditional coarse-profile deferral consequences.
What must wait: complete candidate H/VR totals, recorded-peak<=bound checks for
every row, and definitive chronological admission replay. The remaining symbols
prevent claiming that all old admissions are already rechecked.

Replay plan preserves the runner predicates:
1. Resolve same-model counts/estimate; retain named never/conditional rules.
2. Preserve ascent and successful smaller same-family/mode calibration population
   (>=100 for large cases). Recompute each historical rho denominator using the
   corrected same-binary estimate for that smaller row; do not leave stale
   estimate denominators while replacing only the target estimate.
3. Preserve max(net physical footprint,heap_move) ratio, RSS fallback/baseline,
   rss-to-footprint rule/default1.45 and projected RSS<=0.8C.
4. Preserve C=8GiB, heap cap=C-512MiB, binary E<=heap_cap/2 and ROOT tier approval.
5. Record old/new decision and exact reason for each row; immutable historical
   measurements/work/outcomes/rows remain untouched.

If the selected SD coarse hs upper is used, the factor-values subtotal alone
forces binary deferral for all six10000 VR models:4*f*(f+1) is14400240000 B for
CHAIN/TREE and8100180000 B for CONT, versus4026531840 B half-heap-cap. Both V-K B
and KF3 B historical rows are marked with that candidate-policy consequence.
This is not actual memory excess and does not force an H decision, whose staged
bound does not contain the sparse tail. A useful tighter SD bound is owner-held
work, not commissioned here. Other old/new decisions remain BLOCKED pending
complete totals; do not imply a list of unchanged admissions from partial sums.

## Records mapping, tests, mutants and original gates (not run)

Estimate fields only may change in existing numerical records:
- H counts: estimate_adm_bytes_w1a,estimate_w1_sel128_bytes,
  estimate_w1_fixed_bytes,estimate_w1_decide_bytes,
  estimate_w1_shared/state/solve_{128,256,512,1024},
  estimate_w1_verify/pass_{256,512,1024}.
  Preserve w1 counts/storage/model identities, source hashes, all non-estimate
  fields, and every original four-mode record. Counts JSONL regeneration remains
  bound to recomputation tests and its new explicit launch-plan provenance.
- VR counts/runner: estimate_max_bytes,estimate_sel128_bytes,estimate_fixed_bytes,
  estimate_model_bytes,estimate_decide_bytes and mirrored estimate_adm_bytes_w1a.
  Preserve record outcome, attempts, work, comparisons, row classifications,
  source/model hashes, summaries and existing availability exception criteria.
- VR observations/kernel_lane family records contain numerical case records,
  not the scale estimate; they must remain byte-for-byte unless an independently
  authorized A1 basis refresh is required. Keep that work separate.
- Historical K6b/V-K/KF3 raw records stay immutable. Candidate outputs go in the
  additive K6C run packet; an estimate-only derived replay table references them.

Planned regression/property checks:
- each corrected lifetime term, independent integer phase examples at all three
  verification widths and all33 fixtures; Option<Wide>=Wide with actual types;
- RES G16(B) vs exact summary16B at B=1 and5/non-power-of-two; TOP/HATCHECK
  alternatives, report identity moves, one current report, summary slot alias;
- fallible P(f), exact clones, queue q=max(1,c-1), c=0/1/5/6, consumed input and
  rebuilt old/new overlap; tracker high-water/prune with shrink-then-grow;
- Uc c+at/bt/ct=4fw, shifted work-drop versus3fw Nl, retry old-factor drop,
  publication P64(q), radius exact8q, canonical layouts sequential;
- certificate refusal/partial-drop and selected finish alias cases; descriptor
  checked overflow and invalid/incomplete proof return, no placeholder0;
- public H/VR adapters give identical kernel subterms for the same bound shape
  while caller terms differ; VR cannot silently retain its old estimate loop;
- source15 temporary-to_value vs move/clone, three records, late owners,
  Phase.fields19/33/45 and final-summary prefixes; sparse corrected n+16f arm,
  stored-zero L and Expansion replacement overlap;
- in-solve stop into_solve_128 if the existing staged test can expose it; if not,
  return its concrete missing interface before broadening product source.

Required mutant patches, each saved as a real .diff with input/binary/results:
drop at/bt/ct; restore dead work term; restore OPTION_EXTRA=8; drop Uc c overlap;
restore VR port; revert RES to16B; drop TOP/HATCHECK; double/miss moved summary;
drop current old buffer or old/new rebuild; drop sparse caller16f; conflate record
copies/drop late owners; omit refused-format argument phase/sample prefixes.
NONE must pass. Over-count mutants (dead work/Option padding) are killed by exact
identity/formula/layout tests, not falsely described as measured underbounds.
No mutant is run by this plan.

Later suites: H --all-targets including separate k6_alloc, H runner tests;
VR full suite (at least original47), feature guard and seeded-fault kill matrix;
committed counts recomputation; same-binary historical peak/bound checks outside
CI if heavy. Byte-for-byte numerical records move only for estimate fields.
ROOT then owns independent candidate review, full-SHA hosted CI, fresh DEC-025
sweep and GEN-8. Harness/validation-only scope does not run T9/both-entry as an
unrequested product gate. All suites are pending, not reported as passed.

Later W1-T4 archive run records each model/pass/repeat/prefix outcome, precision,
work by stage/precision, requested/move heap, footprint,RSS,time,load and candidate
bound; compare model-by-model with KF3 B. Stop on a recorded heap above the bound,
changed outcome/published row, admitted binary refusal/watchdog/heap-cap abort,
surviving mutant or ungranted edit. Same source/archive/full SHA and allocator/
normal production cfg must be bound. ROOT grants the actual host slot/guard and
command supervision; this source-only packet authorizes none of those executions.
