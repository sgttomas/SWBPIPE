# CHECKPOINT_0 — independently frozen exact truth

Status: independent truth and comparison interface ready; actual-output comparison
awaits ROOT's release of the preserved output path. This is a ROOT checkpoint,
not an owner approval request. The packet makes no solver-publication, design
acceptance, product-reachability, host-admission, or general closure claim.

Agent: `/root/a1_oracle_fresh`, TASK Type 2, reporting directly to `/root` through
Codex native delegation. No delegation by this TASK. The initial context had no
old oracle results. See READ_ORDER.md for actual reads, including the initial
working-tree instruction read before the required pinned instruction read.

## Frozen basis and outputs

- Coordination instructions/briefs: `2eb85f37083f4f90c763f9c050212f09e522286f`.
- Mathematical/source contract: `d01ad98a754698631f927709d08284c272de85e8`.
- Input primitive definitions and probe grammar only: response
  `520d7dfb790bcedabc03e92b9692884ce295be54`, `src/cases.rs` and `src/main.rs`.
- `TRUTH.json`: **24 cases, 880 rows**, SHA256
  `1772d703e032a71587b922a2f6d825718f777c9dae78d1041fbd874210ee87ea`.
- `exact_oracle.py`: independent standard-library rational derivation, integer
  binary64 rounding, classification and comparison interface.
- `SOURCE_READS.json`: 22 consulted origins with exact revisions/hashes.
- `INPUT_BINDING.json`: all 24 ROOT-released primitive handoffs match source.
- `AUDIT_ARITHMETIC.json`, `DERIVATION_CHECKS.json`,
  `INTERFACE_SELF_CHECK.json`, `RAW_CHECK_OUTPUTS.txt`: executed checks.
- `FREEZE_MANIFEST.json`: checkpoint hashes and write inventory. This checkpoint
  and its frozen files are not to be silently overwritten after outputs are read.

The I22 provisional raw input JSON (SHA256
`fdc86cca269c660e88a9a977990bf52bf3beadd08b67dc8fe8ab30358fdcd091`)
was read only after independent source equations were established and before
numeric truth was written. It was not used as mathematical truth. Its nodes,
member properties, spring, constraints, separately identified load terms, and
empty auxiliary lists were all re-created from cases.rs/main.rs and matched.
The original matrix and oracle remain unread.

## Exact equations and complete row coverage

Main defines two nodes, (0,0,0) and (L,0,0), one straight member with E=G=Iy=Iz=1,
and y_reference=(0,1,0). Thus the local axes are exactly global axes. Accepted D1
DESIGN §4.1.2 defines axial/torsional basic stiffnesses a=EA/L=A/L and t=GJ/L=J/L.
The only permitted free coordinates are UX at either end and RX at the far end;
all bending deformation coordinates vanish. All loads are separate exact
binary64 rational terms, accumulated as exact per-DOF sums f_g.

For B cases, one DOF is free: u_6=f_6/a, or u_9=f_9/t. All other u are zero.
For C cases, free DOFs are 0,6,9, and one axial spring k grounds s=0 or s=6:

    k*u_s = f_0 + f_6
    if s=0: u_6 = u_0 + f_6/a
    if s=6: u_0 = u_6 + f_0/a
    u_9 = f_9/t
    N = a*(u_6-u_0); T = t*u_9

These follow by adding the two axial equilibrium equations, then solving the
ungrounded node's equation. They preserve every tiny input load term. Both free
axial equations and the free rotational equation are independently substituted
back exactly; every free equilibrium residual is zero.

D1 §4.1.5 and recover.rs fix signs: node-on-member local end actions are (-N,-T)
at I and (+N,+T) at J; spring action is -k*u_s; constrained reactions are K*u-f.
All other end-action and reaction components are zero. Node translation
magnitudes are |u_0| and |u_6|. The full source layout gives 12 displacements,
2 magnitudes, 12 end actions, and 11 constrained reactions for each B case
(37 rows); each C case has one spring row and 9 reactions (36 rows). Only
constrained displacement/rotation rows are InputDerived. Magnitudes are not.
The truth packet covers exactly these 16*37+8*36=880 rows, with no copied expected
row list and no source comparator used to derive values.

## Binary64 intervals and O9

All truth is Fraction arithmetic from exact decoded bits. The independent
rounder bisects nonnegative finite binary64 bit patterns to bracket a rational,
then compares exact distances and resolves ties with the significand's even bit.
Negative values use symmetry. The virtual successor of the largest finite value
is 2^1024; its midpoint rounds to infinity. The zero-cell midpoint is h/2, where
h=2^-1074; an exact zero is +0 and publishable, while a nonzero rounded to zero is
Underflow. TRUTH.json records direct bits, rounding-cell endpoints and endpoint
inclusion, outcome, class, bound, and O9 membership for every row.

Direct truth range losses are: B15 D:9; B16 D:6 and M:1; C22 D:0, D:6, M:0, M:1.
There are no direct-truth overflow rows. InputDerived and Unpublishable rows are
excluded from the raw maxima. Per accepted O9 (ROOT rulings 2011–2014 and source
adaptive.rs scales_at/classify_rows_floored), observed candidate publishability
also controls verification-scale membership. The checker can verify published
membership, but TSV does not expose retained verification row values; it cannot
reconstruct the actual verification scale or prove a universal transfer bound.

Truth direct-rounding/classification is a baseline, **not a prediction of solver
selection or selected-row bits**. For actual published values, the checker
recomputes scales and classes from those values. A direct-bit mismatch is
reported separately from claim failure. Range classification is checked against
exact truth when a value is withheld as Underflow or Overflow.

## Scale and bound contract

The extent is exactly L for these power-of-two axial geometries; squaring and
square root are exactly representable. From non-input-derived publishable raw
maxima (tr,ro,fo,mo), use fresh original operands in each coupling:

    S_tr=max(tr,RN(L*ro)); S_ro=max(ro,RN(tr/L))
    S_fo=max(fo,RN(mo/L)); S_mo=max(mo,RN(L*fo))

RN is exact nearest-even binary64. At selected precision 512, the separately
published Force/Moment FLOOR values are applied by max. Their independent
formation from resolution_scale is outside this checkpoint's interface; ROOT/I22
must retain that separate evidence. The direct-truth baseline applies no floor.

Classification: InputDerived first; no-value rows Unpublishable; otherwise
AbsoluteVerified iff S<2^-988 or |q_pub|<RN(2^-34*S). All other rows are
RelativeVerified. Ordinary b=RU(2^-64*S). For 0<S<2^-988, A1 specifies
b_row=RU(RU(2^-64*S)+RU(2^-53*|q_pub|)+h). At S=0 the bound remains zero.
RU is exact upward rounding, independently implemented with rationals.

**Authority-text conflict:** D2 DESIGN.md lines 589–593 still require the old
b=RU(2^-64*S) universally. ROOT_RULINGS_V1.md lines 2016–2024 and 2071 explicitly
adopt A1 into hash-pinned D1 and route the per-row rule to D2. The comparator uses
the later adopted A1 rule; the older D2 wording is recorded as a documentary
conflict, not silently treated as the live implementation contract.

## Separate named claim predicates

Let error=|q_pub-q_truth|. No truth is derived from any claim denominator.

1. **Public relative guarantee (D1 DESIGN §4.1.6):** for RelativeVerified rows,
   error <= 10^-9*|q_pub|. The floor derivation explicitly divides by the published
   |q| (lines 429,444–446). For clarity the checker also reports the conventional
   truth-denominator predicate error <= 10^-9*|q_truth| separately. A disagreement
   is not silently resolved by changing denominator. R1's reference-scale
   benchmark is not used as either predicate.
2. **Sharper scale-based check (R7 §5.2; ROOT ruling 1978–1982; corroborated by
   models.rs compare_honest lines 489–525):** for RelativeVerified rows,
   error <= 2^-64*max(|q_pub|,S_pub)*(1+2^-21)+2^-53*|q_pub|+h.
   This is a stricter internal check justified through the candidate bound and
   publication allowance; it is reported separately from the public 1e-9 promise.
   Its scale-transfer premise is precisely what AUD-T3-01 challenges. The test
   function corroborates how the stronger contract is exercised; it is not the
   sole authority replacing the public promise.
3. **Absolute publication claim (R7 §5.2 plus adopted A1):**
   error <= b_row*(1+2^-22) at 128/256, or b_row*(1+2^-21) at 512, using the row's
   published bound after independently verifying its bits. At b_row=0 the
   predicate requires exact truth agreement; no fallback tiny allowance is added.
4. **InputDerived:** verify exact prescribed zero. The inherited source-test
   allowance 2^-53*|q_pub|+h is also reported; it cannot excuse changed input.

The rational expressions above are evaluated exactly. The inherited Rust test
computes its allowance with binary64 operations, so the checker additionally
reports that operation-by-operation binary64 denominator and pass/fail. A tiny
rounding difference is exposed, not absorbed into a new tolerance. The h term in
the source-test half_ulp is preserved; it is not changed to the h/2 arithmetic
rounding-cell bound. No engineering criterion or accepted tolerance was modified.

## Independent audit checks

With r=5h/4 and L=2^100, RN(r)=h, S_v=L*r=(5/4)2^-974,
S_pub=RN(L*RN(r))=2^-974, and (S_v-S_pub)/S_v=1/5. The lost h/4 is amplified to
2^-976, exceeding h/2 and the claimed relative 2^-64+2^-52 allowance. The reciprocal
construction with raw translation 5h/4 and L=2^-100 gives the same exact loss.

With candidate translation zero and exact truth q*=9*2^-1038/8, the abstract
stop allowance is (5/4)2^-1038 while the ordinary published b is 2^-1038.
Thus the abstract stop inequality passes and the published absolute claim fails:
q*/b=9/8>1+2^-22. This independently confirms the proof counterexample only;
it does not establish an unmutated solver publication.

For r=5h/4 and L=2^-100, exact L*r>0 but RN(L*RN(r))=0, so the published zero
scale still gives bound zero. B05/B07 are just below the branch threshold after
coupling (2^-989), and B06/B08 are exactly at 2^-988. B11/B12 give exact-subnormal
controls; B13/B14 give normal-boundary controls; B15/B16 exercise O9 underflow.
The C cases preserve independent large cancelling loads and tiny tails; their
full rational truth, including spring actions, is frozen without assuming what
any finite-precision solver will retain.

## Input/output interface for ROOT and I22

The existing public probe TSV grammar from src/main.rs is sufficient for a
bounded row-claim comparison. Required records are:

    FORMAT  a1-public-tsv-v1
    CASE  <B01..B16 or C17..C24>
    STATUS  <Selected|Refused|Unresolved|SourceRefused>
    SELECTED  <128|256|512>  <2p>                  # Selected only
    LAYOUT  <key>  <Translation|Rotation|Force|Moment>  0  <true|false>
    ROW  <key>  <kind>  <Value|Underflow|Overflow>  <16hex|none>  <class>  <16hex|none>
    SCALE  <kind>  <16hex>                        # exactly four for Selected
    FLOOR  <Force|Moment>  <16hex>                # only when p=512

Fields are tab-separated; the displayed spacing above is explanatory. Keys are
D:g, M:node, E:1:I|J:component, S:1:0, R:g. No duplicate/missing/extra row keys are
accepted. Other probe evidence records are preserved by the producer but not
used as truth. Refused/Unresolved/SourceRefused reports carry no accuracy-pass
claim. Strict syntax errors stop comparison instead of skipping data.

ROOT must pair any output with its case primitive input hash, actual candidate
revision, command/limits, absent seeded-fault context, raw output hash, and host
admission record. The TSV alone cannot prove it came from an unmutated candidate
or valid admitted execution. No new Rust instrumentation is required for this
bounded comparison. Broader design review may need retained row/scale evidence;
that is a separate scope decision.

Run `python3 exact_oracle.py compare <oracle_fresh> <ROOT-released.tsv>
<owned-report.json>`. Keep reports additive under oracle_fresh. The returned JSON
lists per-row truth, exact error, named allowances/predicates, expected class,
range checks and direct-rounding equality. Any false unmutated publication is an
immediate BLOCKING return under COMMON, with no continued affected-path probing.

## Completed checks and limits

Executed: exact free-equilibrium substitution in all 24 cases; binary64 boundary
and tie checks; independent CPython Fraction-to-float agreement on all 880 rows;
all 24 provisional input schemas matched; truth regeneration byte-identical;
24 synthetic direct-truth TSV interface checks (880 rows), zero failures. The
synthetic checks are not solver observations. Raw check outputs are retained.

Unrun: Rust, solver, native product reachability, source receipt replay, G5a/unit
conversion, general design proof, output comparison, guard qualification and
any Git/CI/DEC-025 activity. This checkpoint does not alter or close their holds.
There was no unintended exposure to prohibited expected/output records. The old
worker's packet and suggestions were not read. No broad acceptance is requested.

Return to ROOT now for the preserved comparison output path; the independent
truth, derivation, rules and interface are hashed before that output is read.
