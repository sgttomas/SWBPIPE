# I30 — F2a atomic with S-G1: checkpoint-0 proposal

Status: **revisable planning inventory; not design acceptance or implementation authority**.
TASK `/root/i30_f2a_checkpoint0`, directly under ROOT HELP_HUMAN `/root`.
Receipt 2026-10-02 18:05:16 UTC; new-analysis cutoff 18:55:16; return by 19:05:16.
P = `projects/chirality-piping`; FK = P/core/solver/frame_kernel; PP = P/core/product_physics.
T3 and R have the meanings in `_run_records/EXECUTION.json`; source anchors below are P-relative.

## 1. Decision package and authority

Recommend one atomic F2a/S-G1 change after the bounded prerequisites below are checked:
W1a source construction, invocation-level coexistence, final-row certificates, closed
receipts and all three readers. Keep F2b retirement, S-I implementation, W1b/W1c,
0.4.0 successor production, structural-zero exemptions and nonlinear changes out.
ROOT_SELECTION_DESIGNS fixes F2a/S-G1 → S-I1/S-I2 → F2b per domain → F3.
D2's older S-I2 grouping does not place interval evaluation in this write set.

The initial inspected NUM HEAD is `252e97404ba312b46fd85aef415729ba67cf652a`.
Maintained source equals merged main `49034a940f3f8cd3f3da4d4cbc839943b808063d`
across P/{core,apps,schemas,fixtures,tests,validation}: read-only Git diff is empty.
ROOT's concurrent records advanced HEAD to `a0a59f164ca87e22b6e6bc6af740c8d66f3c31a2`;
its planning-release addendum was read, and maintained diff/status remain empty.
D1 DESIGN r5a.2 is read with selected r5a.3/R7 and subsequent ROOT amendments;
D2 is r5b.2. A1's original proof is history, superseded by the selected correction.
The selected SI policy is `M03-INTEGRITY-MP-v2`; its method token remains
`contribution_preserving_multiprecision_v1`. Old v1 records are never relabelled.

This packet identifies real prerequisites, rather than treating planned APIs as
existing. It does not certify final-row mathematics, production allocation bounds,
availability, native behavior, or any unrun test. `SOURCE_MAP.md` bounds the proposed
maintained changes; ROOT must reconcile I29's memory plan before freezing a grant.

## 2. Actual routes and proposed transaction boundary

The captured entry is PP `lib.rs:2151`: actual Value → `CapturedInvocation::parse`
→ common case loop. Desktop reaches it at `apps/desktop/src-tauri/src/lib.rs:1562`;
headless captured production reaches it at `core/runner/headless/src/lib.rs:739`.
The typed entry (`PP:2141`) supplies `capture=None`; headless's compatibility
`run_preview_in_memory_mode` (`headless:804`) deliberately cannot mint custody.
D1 §4.3 requires “a captured invocation with `digest_ok()`”; source_receipt.rs:100
says “Captures the actual raw request before parsing. No parsed/hash constructor.”
Therefore the present typed API remains ordinary, with no W1 selection/proof.
No typed reserialization is an actual-request capture. A new custody-bearing API,
if desired, is a separate ROOT/owning decision; existing Value entry already serves it.

There is a second, distinct gap: `source_receipt.rs:109–119` currently makes
`checked(&raw)?` and digest hashing fatal before serde parsing; no `digest_ok` API
exists. S5-R's digest-unavailable ordinary fallback is thus a required explicit
prerequisite. Propose a capture state retaining actual raw Value/mode plus
`Available(digest)` or `Unavailable(reason)`, while parse/model errors remain errors.
Only Available enters either proof-bearing selection. Unavailable reaches ordinary
publication and the named `invocation_not_representable` W1 decline when relevant.
Representable existing exact-block-selected invocations must remain byte-identical.
Do not silently apply this fallback to unrelated old finalization behavior.
D2 §4.8 already says headless evidence hashing can fail, leaving canonical export
unavailable (T6). Actual headless:741–752 catches that failure and returns raw output;
its ordinary checksum at :1013 uses legacy canonical_json, not checked-profile custody.
Native `previewService.ts:102–130` likewise keeps raw IPC results but cannot register
unsafe request/result fingerprints. `workspaceSession.ts:847` then refuses before
setResult; AnalysisRun:107,125 also requires checked row/result hashes. Thus PP-only
S5-R is not end-to-end native display/Current fallback. Keep these established hashes
and freshness checks unchanged. ROOT must explicitly scope an inspection-only native
return with absent qualified evidence/AnalysisRun, or record that native availability
as held; D2's old caller claim supplies no current raw-inspection state. No placeholder
hash, changed canonicalization, manufactured capture or ordinary Current upgrade.

Current ordinary construction is sparse per basis (`PP:2364`), including each
material/modulus basis in `basis_solve_states`; dense scrutiny materializes only
inside its existing guarded path. `solve_load_case` builds the exact source-term
load ledger before reduction (`PP:3425`), runs the ordinary dense/sparse solver,
then the existing exact-block attempt (`PP:3570–3668`). K2b/W2, formation findings,
load-fidelity findings and ordinary reports retain their existing meaning.
No automatic dense fallback or changed dense/observation ceiling is proposed.

**Arbitration before W1:** retain the original per-case ordinary attempt and
exact-block disposition in request order. Complete the invocation decision before
allocating a W1 source batch or spending W1 work. If any exact-block case selects,
return the legacy publication path, including identity, rows, diagnostics, receipt,
numerical_quality and ordering, with no new W1 diagnostics or method token.
This is D1 §4.4's literal no-attempt coexistence rule, not a mixed-method identity.

The current loop returns immediately on a case error/block (`PP:2476,2515`), so
this cannot be added merely after `source_selected` at PP:2524. CP1 must introduce
an owned deferred case-outcome representation: ordinary rows/render inputs or the
original terminal result, exact disposition, ordinary report and diagnostic order.
Keep the legacy terminal prefix and source budget; never repeat ordinary/exact
attempts to reconstruct it. Review the early-failed-case/later-exact-selected case
explicitly before authorizing this refactor. If collection cannot preserve legacy
semantics, hold that branch; do not remove the coexistence condition to make it work.

With no exact selection, W1 candidates are Sensitive (including D5 demotion),
the accepted unresolved/range triggers, and supported-family NegativeEnergy.
Mechanism, Asymmetric and InvalidInput do not trigger. Nonlinear invocations are
never selected and keep ordinary behavior without a refusal of their results.
Outside W1a, a would-be trigger records method-unavailable; preserve ordinary standing.
An unrecovered ordinary blocking case still blocks as today; T6 case-scoped standing
is not implemented here. F-P2 recovery inside an exact-selected invocation waits F2b.

## 3. W1a sources, cases and combinations

Build `SourceParts` from the normalized admitted model and each actual material
basis, not from rounded K, force-vector nets, member transforms or rounded solutions.
Use current node order and explicit bijections from product ids to kernel ids.
StraightMember receives coordinates, y_reference, E,G,A,Iy,Iz,J as its admitted
binary64 primitives. Derive no new physical range cutoff; keep SourceError rules.
Preserve every identified nodal force/moment term independently. Build positive
global-axis springs, zero rigid constraints, all required stations and SupportGroups.
No DirectionalSpring is built: source.rs expressly makes that kernel-only.
Respect exact-route explicitly empty pressure inventory; legacy route requires zero
pressure. Reject unsupported producers by source family, even when their net is zero.
Uniform/self-weight/thermal/thrust/generated/constant-effort/user-element/curved and
nonzero prescribed-motion product cases remain outside this F2a scope.
Do not promote a hanger's DOF to prescribed: use actual family/hanger rules.

Use `solve_cases(sources, CaseLimit(20_000_000_000), &mut meter)` for one admitted
batch in request order. Its stiffness groups/cache exist only inside that call
(`adaptive.rs:4342–4409`). Repeated solve_case calls share no cache merely because
source bytes match. Keep selected solves owned until their combinations/publication
finish; never clone them only to retain access to a row or private radius.

Actual `RetainedCombination::solve` is a fresh solve from the exact combined load
ledger and prescriptions, with its own certificate/schedule. It does **not** sum
published rows or retained u states. It borrows selected operand solves, merges
their actual cache snapshots, and requires identical stiffness, layout, stations
and support groups; nested combinations are refused (`combine.rs:1–115`).
Preserve T0R mechanics/subtraction/range gates, modulus gates and existing no-maxima
semantics. Retained combination classes/scales come from its own final rows.

**Open API choice:** a not_required ordinary case has no RetainedSolve. An all-selected
combination is implementable now; a mixed ordinary/selected combination cannot use
the current API. Do not force ordinary cases through W1 or silently combine rounded
rows. Prefer a separately reviewed source-based combination entry implementing the
same own-solve mathematics without solving operands on demand; it must define actual
cache ownership/work and meet I29's lifetime composition. Alternative: conservatively
withhold such combination outputs, with named reason and measured availability cost.
ROOT must select before CP1; a protected availability loss needs its owning ruling.

## 4. From A1 SI rows to actual product publication

A1 proves a candidate's **final prescribed-replaced SI kernel rows** after unchanged
R7 gates: H=|x−v|+E; absolute H≤b, relative H≤min(A_exact,A_f64) and 10^9H≤|x|.
P is report.precision=2p, not twice that. The private RU64(H) slice is moved with the
same Publication into RetainedSolve; no report is retained or may be borrowed later.
The old scale-closeness argument proves neither conversion nor derived publication.

Gap: `publication_radius` and `SiRadius` are private (`adaptive.rs:3462–3553`),
and the frame_kernel/public facade is a different crate. Propose the smallest
internal-in-purpose Rust API: an immutable `CertifiedSiRow<'solve>` borrow obtained
only from `&RetainedSolve` using expected QuantityMeta, source identity and precision.
Expose it across the crate boundary via a narrow pub reexport, private constructors
and no Serialize; validate every existing index/layout/body/kind/class/source check.
Return explicit InputDerived/Unpublishable absence, never radius zero or infinity.
The lifetime prevents the borrow from outliving its owned solve; it does not distinguish
different cases by itself. Explicit identity checks and one view containing the actual
row/value/radius association prevent cross-case mixing. No report is retained. The bound
is producer integration data, not a new public row/receipt field or widened b.

Create a complete product-row binding map keyed by final qualified row id, basis ref,
source QuantityId or derived recipe, member/support identity, unit and sign convention.
Certification runs **after** preview render, support replacement, row qualification,
combination rendering and summary selection, immediately before receipt hashing.
`preview_physics.rs:528` rebuilds support/maximum rows; PP:2560 rewrites ids.
Certifying an earlier vector misses those real publication operations.

For each kernel identity row, x is SI, y is the emitted value in U, n=N_U(y).
Use exact a_U: SI→SI=1, mm→m=1/1000, kN/kN*m→SI=1000, MPa→Pa=1_000_000.
Pinned N_U is one RN64 operation: mm **divide by 1000**, not multiply by 0.001.
With certified R_x, H_U=(|a_U y−x|+R_x)/a_U and H_n=|n−x|+R_x are sound;
retain separate e_pub and e_norm checks even when the round trip equals x.
For relative kernel rows the exact minimum of r_x,A_exact,A_f64,|x|/10^9 can
tighten R_x; no nearest rounding may understate a non-dyadic allowance.

Recompute SI maxima from actual value-bearing, non-input-derived product rows,
original-operand coupling, selected p512 force/moment floor, per-member stress
scales and final class. Do not copy kernel scales/classes to a changed row set.
R=2^-34; below S=2^-988 all mapped rows are absolute. For 0<S<2^-988 use A1's
unchanged row-dependent upward b construction; S=0 gives b=0. At p512 use the
same E/ê/Φ bits and only the existing force/moment floor. No widened public radius.
Absolute final rows require H_n≤b_SI; relative final rows require both A1 sharper
allowances at n and the decimal predicate, plus 10^9H_U≤|y| for the raw claim.
Any missing operand, range/span/work failure, nonfinite bound or failed certificate
declines the selected case by name; never silently relabel it NotCovered to pass.
Current API cannot resume a selected kernel schedule after a facade refusal; default
to named ordinary fallback, not an invented p-escalation or second unmetered solve.
An unpublishable required row declines selection; it is not omitted to certify the rest.

The direct rows include displacement/rotation, **kernel-computed** displacement and
support magnitudes, end and station actions, reactions and individual spring actions.
Use recover.rs's station convention (minus i-end at fraction 0, j-end at 1), explicit
support-law attribution, and the existing ambiguity policy. Do not recompute a
magnitude from rounded components or a spring action from rounded displacement.
Zero-pressure wall/effective actions require exact identity/sign mapping to their
certified action. Any support component formed by several rows needs its own exact
sum enclosure; mapping by coincident numeric index is insufficient.

For derived rows, first enclose intended f(q*,operands) in I_SI using certified
source intervals; then H_n=sup|n−I_SI| and H_U=sup|y−I_SI/a_U|. Actual y still
comes from the existing product formula. Replaying that formula on rounded inputs
does not establish I_SI. Use a bounded private arithmetic helper, not S-I's rule engine.
ExactWideSum/WideContext are not exported; do not expand their public surface by
accident. Existing ExactAccumulator add/add_product/sign can support exact dyadic
comparisons after positive integer cross multiplication. Directed endpoint arithmetic
and sqrt adjustment need a checked finite-step/span limit and independent proof;
zero bounds need actual exactness, not a generic one-ulp interval asserted as exact.
CP0-proof must close these separate recipes with independent exact/directed oracles:
- axial/bending N/A, My/Z, Mz/Z; torsion's actual T·radius/J operation and section
  operands (do not assume rounded J/radius equals 2Z); zero-pressure membrane;
- user intensified i·hypot(My,Mz)/Z, resolving component→member from invocation;
- pressureless circular maximum |N|/A+hypot(My,Mz)/Z and open-formula |a|+|b|+|c|.
  W1a end/station actions are affine along an unloaded member; these functions are
  convex, so the intended span maximum is at an endpoint. Enclosing both endpoint
  functions and taking max of lower/upper endpoints encloses that maximum. Verify
  that each emitted recipe has exactly these premises; product span-statical rounding
  and its CertifiedStressMaximum of rounded coefficients remain separate errors.
Keep k=1, upward sqrt(2)·i, 2sqrt(2),4 class factors, not new error assumptions.
Bind A,Z,L,k_a,k_t to actual product/model sources. Any geometry/section rounding
not defined as an exact-bit operand needs a separate enclosure, including physical L.
The recipe-to-source truth/section boundary and bounded arithmetic proof are **open**;
this plan is not their verification. Unsupported recipes preserve existing NotCovered
semantics; a covered recipe's failed proof is a refusal, not a scope expansion.

## 5. Atomic producer, three readers and carriers

Proposed ids (collision results in SOURCE_MAP): `…/preview-physics-retained-1` and
`…/physics-retained-1`, each with its own inherited base-table hash and new table hash.
Proposed profiles: `product_preview_retained_w1a_v2`, `exact_straight_retained_w1a_v2`.
Use existing checked canonical profile `openpipestress_jcs_ijson_v1`; ROOT reserves
names/schema/digest domains before coding. Do not reserve an F3 successor here.
Pin corrected kernel policy and selected W1_RESOURCE_POLICY_V1 in each contract.

The closed retained_precision body binds actual invocation/mode, publication without
receipt, case order, source/ledger/state digests, ordinary attempt, actual p/2p and
R7 summaries, SI section/scales/classification and all required work/outcome evidence.
Only selected cases carry the method token. Numerical_quality remains ordinary.
No selected receipt may coexist with source-block selection/unavailable for that case.
Emit a successor only if at least one case survives final certificates and hashing.
Encoding failure is unavailable; publication_hash_range returns base ordinary
publication with spent W1 ledger preserved, never an invocation Err caused by W1.
If unavailable evidence itself exceeds checked integer range, decline successor
emission and retain exact spent counts in diagnostic/evidence text; freeze that
closed failure representation at CP0-wire, rather than truncating unsafe integers.

**Wire realization to freeze:** kernel attempts include role=Verification and
VerificationThenCandidate; verbatim copying would not satisfy D2's terminal rule.
D1 §5 specifies (p,outcome,reason,work), not a verbatim native record dump. Prefer
logical candidate entries ending Accepted, each with its verification outcome/p,
reason and lossless work breakdown. For a reused verification, charge its solve
and verification pass once to the earlier pair; its later candidate stop/certificate
work belongs to the later entry. Preserve shared/verification-shared built-versus-
reused distinctions and failed work; prove projection totals against the native
ledger. This is a faithful projection proposal, not a concluded contract amendment.
If that cannot preserve D1/D2's obligations, return the exact conflict before changing
public semantics. Also specify separate
combination records/coverage, since D2's one entry per load case cannot silently
double as combination coverage. No receipt may omit published retained combinations.

All three readers independently enforce ordered G0–G8 and shared failure codes:
G0 identity/profile/table/policy; G1 closed shape and hashes; G2 bits/safe integers;
G3 case/combination order/coverage; G4 diagnostic exclusivity; G5 actual schedule,
work threshold semantics and R7 summaries; G6 row token scope; G7 strict base
validator projection; G8 invocation/model/material/section/mode re-derivation.
Validate schemas as well as executable readers; a label/hash alone adds no standing.
Unknown id/profile/policy, malformed/mutated evidence and any G mismatch are
unsupported. Missing invocation, unavailable cases, ineligible ordinary cases or
scope findings are needs_recompute. Only all applicable checks allow numerical use.

G5a uses resolution_scale, zero/sanity/lower checks on normalized SI rows, estimate
≤1/4, charge≤1, theta≤1/2 and finite positive B where applicable, with actual field
coverage from R7/A3 rulings; no invented B for a body without factor data. Run the
identical preflight in the producer. A1 removed the claimed automatic-pass proof;
an honest G5a refusal is conservative, not permission to bypass the reader. Producer
refusal uses the selected `receipt_encoding` reason with its exact predicate detail.
G5b/c derive bodies, L, O9 membership, E coupling/Φ, stress k and exact class/list/b
bits, including A1 small-scale b_row. Re-derive input-derived DOFs and pressure
inventory from invocation; cross-check section/material facts to the existing
base evidence and source-digest trust boundary. Rust replay audits are validation,
not a hidden prerequisite that Python/TS cannot meet. TS must run successor validation
at native direct/job registration, bind its async result to immutable source/capture
fingerprints, and require that validation in synchronous standing/freshness queries;
adding the identity to an admission set must not register an unchecked successor.

AnalysisRun and canonical derivatives copy the validated receipt without loss and
carry absolute/NotCovered row disclosures. Transport without raw rows checks only
its available metadata/hashes and remains ineligible. Before S-I, absolute and
NotCovered rows/headlines refuse binding with their two adopted codes; relative
and InputDerived retain point binding. No envelope-level downgrade solely for one
absolute row. Display b_SI around n in its SI unit, or derived outward endpoints
in raw U; never print raw y ± numeric b_SI labelled U. S-I later consumes (n,SI,b).
Keep stress-neutral packaging and desktop export availability at the existing T6
boundary; add explicit successor refusal tests, not accidental export admission.

## 6. Resources, bounded checkpoints and decisions

One InvocationMeter(60_000_000_000) spans actual cases, combinations and every W1
attempt/republication; CaseLimit=20_000_000_000 includes each combination. Charge
own/shared/verification-shared/failed work exactly under current semantics; invocation
pays actually built shared work once. Preserve actual overshoot and stop reason;
do not reject honest unavailable receipts solely because charged>threshold, clip
counts, reset per case, or claim time/memory/RSS maxima. Selected scheduling must
match real checkpoint precedence. A1 work is already metered. Source/graph/encoding,
ordinary/caller and new facade-certification work need explicit separate accounting
until ROOT adopts a priced extension; no invented LME prices or H-derived byte guard.

I29 must supply: admission API + checked count inputs + complete requested/moving
composition covering raw/normalized models, ordinary basis/results/diagnostics,
source batch/encodings, groups/cache, active/rejected attempts, retained operands,
private radii, derived arithmetic, drafts, receipt hashing and fallback lifetime.
It must check **before** every covered allocation/growth, bind source/build/target
premises, handle overflow/missing proof/excess with a named W1-only refusal, and
release reservations on success/failure/unwind. The 3.75-GiB number is a target only.
No production W1 merge until ROOT accepts the reviewed allowance/enforcement and
actual composition; ordinary results/standing must survive W1 admission refusal.

CP0-proof/wire: independent per-recipe mathematics, source truth, accessor and
attempt/combination schema review; resolve decisions below and integrate I29.
CP1: bounded source-only refactor/accessor/source-map draft; review early-failure
arbitration, exact byte preservation, capture fallback and mixed combinations.
CP2: authorized isolated tests and independent oracles, then final-row producer and
three readers/schemas/carriers; source review precedes production runtime reliance.
CP3: paired candidate/base and mutation checks, T9/both-entry, admitted native
witness and current-head gates described in SOURCE_MAP. Freeze/review the complete
atomic diff; fixes reopen affected checks. No partial producer-only merge.

ROOT dispositions: (a) place the already accepted S5-R captured digest-unavailable
fallback in the bounded implementation grant, and separately choose native raw-only
inspection representation/scope or keep that availability explicitly held;
(b) keep typed API ordinary (recommended), or separately authorize custody-bearing
entry migration with actual-Value tests; (c) adopt the internal accessor and freeze
per-recipe source truth/proofs; (d) faithful logical-candidate projection and combination wire
shape, work-overshoot/failure encoding; (e) source-based mixed-combination API versus
explicit withholding (measure protected availability before disposition);
(f) reserve collision-checked ids/profiles/domains and freeze exact write manifest;
(g) accept I29's reviewed byte policy/interface or hold W1 production admission.
Changing public b/class/scale, protected availability, physical domain or deployment
claims returns to its owning decision. Faithful bare-b correction needs no new
approval. These decisions prepare a reviewable implementation; none is made here.
