# Addendum 04 — accepted bare-b comparator frozen before repaired outputs

Status: complete bounded independent comparator; **no repaired implementation
output read or compared**. All prior oracle code, expectations and seals remain
unchanged. This is an additive selected-claim check, not a new mathematical truth
packet, expected-selection list, bound-construction proof or source repair.

The accepted contract is DESIGN_PROPOSAL.md SHA256
926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178 plus
CORRECTION.md SHA256
30c907589e80a60407acc139a5805d9941b5df1a5b9dd5a2f1dc845159bae80a,
as selected in BRIEFS/I22_IMPLEMENTATION_A.md and commissioned by
BRIEFS/A1_CHECKPOINT_B.md at coordination revision
bb6f9b75ea859560a45f2ff049a02e19098c1c2f. The proposal/correction's original
proposed-status wording is historical; ROOT's later selection is the authority.
Only their contract was used, not implementation source or generated results.

## Scope and immutable truth

Exactly **20 fixed sources / 744 rows**: B01–B16, C17, EXTRA-FM-01,
EXTRA-MF-01 and EXTRA-ZR-01. C18–C24 are outside this comparator's grant.
No source or expected value has been added or altered. The comparator verifies
these hashes at runtime:

- Original TRUTH.json: 1772d703e032a71587b922a2f6d825718f777c9dae78d1041fbd874210ee87ea.
- Three-control TRUTH.json: aac399bfb68bbb8b1cbcce08cc2901229535e88860dc5cc812cf08c9fe3ba0ee.
- Our exact bit/rational helper: fa8ea6f303148d9babb5d9fe6c53f64377b13cb130d03d076d4cec7d3f4c15e0.
- New SOURCE_IDENTITIES.json: 4f0865d132225612cd11f43ca715038d8b485646e6eb6f7bf8f51fc922e8d7f2.

B/C17 source identities were reconstructed from their frozen primitive bits and
the previously read pinned constructor/encoding. Extra-source identities retain
the exact independently frozen canonical encodings. A separate byte decoder
checked all 20 full K4SRC encodings against the prior independently bound input
handoff and extra truth. This catches source ID, separate load/spring, property,
constraint or coordinate differences. REGISTRY_CHECKS.json records every case.

## Exact predicates

Let x be the actual finite binary64 SI row publication and t its frozen exact
source truth. Let e=|x-t|, S the reported finite nonnegative SI scale, b the
reported finite nonnegative bound, epsilon=2^-64, u=2^-53, h=2^-1074.

- **AbsoluteVerified:** require e<=b, exactly, at p128, p256 and p512 alike.
  Equality passes. b=0 requires e=0. There is no historical 1+2^-22 or 1+2^-21
  multiplier, binary64 comparison rounding, fallback h, or private radius.
- **RelativeVerified:** require all three independently reported predicates:
  `10^9*e<=|x|`, `e<=A_exact`, and `e<=A_f64`.
- **InputDerived:** require the exact prescribed publication value. There is no
  solve-error tolerance for changing a prescription.
- **Unpublishable:** compare the reported Underflow/Overflow range outcome to
  the independently frozen exact range. No absent value is treated as zero.

The sharper exact allowance is

    A_exact = epsilon*max(|x|,S)*(1+2^-21) + u*|x| + h.

The other allowance is the exact decoded value of precisely these nearest-even
binary64 steps, in order and with no reassociation or fused multiply-add:

    a0 = RN64(epsilon*max(|x|,S))
    a1 = RN64(a0*(1+2^-21))
    u0 = RN64(u*|x|)
    u1 = RN64(u0+h)
    a2 = RN64(a1+u1)
    A_f64 = exact_decode(a2)

Every intermediate is required finite/nonnegative. The existing independently
written integer-bit rounder decides each RN64 operation. The public decimal
threshold is an exact integer cross multiplication by 10^9. The conventional
truth-denominator relative predicate is diagnostic only and cannot replace the
accepted published-value denominator. The result retains each step and each
predicate separately, so a disagreement is visible.

The historical qualified comparators and observations remain sealed historical
evidence. The new checker does not reinterpret them as certified under bare b.
It requires the corrected policy identity for new selected publications.

## Strict interface and source binding

The grammar is the existing tab-separated a1-public-tsv-v1, restricted to the
20 registered CASE IDs. FORMAT, CASE, SOURCE_COMMIT, LIMITS and STATUS must be
well formed and singleton. SOURCE_COMMIT must be a full lowercase SHA, and the
two limits must be positive canonical integers. Their values are recorded;
actual binary/candidate identity and runtime admission remain ROOT's evidence.

Every result must carry exact SOURCE_ENCODING and STIFFNESS_ENCODING matches.
A source refusal lacking an encodable source is therefore invalid/unbound for
this fixed-source comparator, not an accuracy pass. For Selected, also require:

- Exactly one supported SELECTED pair: 128/256, 256/512 or 512/1024.
- Exactly the IDENTITY triple `contribution_preserving_multiprecision_v1`,
  `M03-INTEGRITY-MP-v2`, `3dd0000000000000`. A historical v1 identity is rejected.
- SOURCE_ENCODING_SELECTED equal to the independently encoded input source.
- Complete ROW and LAYOUT sets in the pinned canonical source order, with exact
  IDs/kinds/body 0/InputDerived membership. No duplicate, missing or extra row.
- Four unique finite nonnegative SCALE entries. At 512, precisely the Force
  and Moment FLOOR entries, finite/nonnegative; otherwise no FLOOR entries.
- Lowercase 16-hex finite value bits; positive canonical zero; proper value,
  outcome/class and bound/no-bound combinations. Negative/NaN/infinite bounds,
  negative zero and nonfinite/duplicate floors are invalid.

Source-bound Refused, Unresolved and SourceRefused records may not carry
selection or publication records. They produce `numeric_accuracy_pass: null`,
zero compared rows and exit **3**. This is explicitly no accuracy pass.
Unknown statuses or missing/wrong source binding produce exit **2**.

Other retained probe diagnostic records are preserved in the caller's raw TSV
and are not used to derive truth. This checker does not inspect new certificate
H/radius fields, reconstruct verification states, or certify old/new bound
construction. A reported bound passing the exact error test does not establish
that its formation is correct or authorized. The independent source/design and
accounting reviews retain those obligations.

## Exact CLI

From the A1 checkout, with R set to the resumed-run repository-relative path:

```sh
<VENV>/bin/python -B "$R/oracle_fresh/addendum_04_bare_b/bare_b_compare.py" <released-output.tsv> <new-owned-report.json>
```

The report path must not already exist. Exit codes:

| Code | Meaning |
|---|---|
| 0 | Selected, source-bound output; all applicable numerical/range checks pass |
| 1 | Selected output has a numerical/range finding; inspect per-row failures |
| 2 | Invalid interface, identity, source binding, frozen hash, or report target |
| 3 | Valid source-bound non-selection; no accuracy pass |

For a numerical failure in an unmutated output, stop the affected path and report
BLOCKING promptly under COMMON. A malformed output is not evidence that the
source's mathematics is dishonest. No source/guard admission is inferred from
a zero exit. Use an additive report path in the executing agent's own scope.

Boundary-check rerun:

```sh
<VENV>/bin/python -B "$R/oracle_fresh/addendum_04_bare_b/boundary_checks.py" <new-owned-boundary-report.json>
```

## Executed synthetic checks and actual ordering

27 checks passed. Exact predicate-only vectors cover zero and positive error at
b=0, equality at subnormal/normal b, just-outside values, and an error inside the
old multiplier but outside bare b. Relative vectors independently isolate
A_f64<A_exact and A_exact<A_f64, plus equality and just-outside the exact public
decimal boundary. None is an added primitive source case.

One synthetic direct-truth B01 TSV exercises the entire selected interface;
its metadata explicitly belongs to the synthetic test, not an observed solve.
Malformed variants check missing/wrong source bindings, old policy, duplicate
selection, unknown status, negative scale, nonfinite/negative-zero values and
missing/negative/duplicate floors. Three source-bound non-selection statuses
return no accuracy pass; an unbound source refusal is invalid. No repaired
output or implementation-generated expectation was used.

Actual reads/actions, in order:

1. 318d36: pinned A1_CHECKPOINT_B brief via read-only Git.
2. 1cf938 / 3f3134: filename-only discovery of brief/design pointers.
3. 83223c: I22_IMPLEMENTATION_A acceptance/identity contract, not its test plan.
4. ae97a2: accepted design and correction at their exact hashes; displayed full
   correction and relevant proposal contract, with middle output truncation.
5. 765af7: registry preparation accidentally launched in COORD; its first mkdir
   raised FileNotFoundError before any file write. ROOT was informed. No file or
   Git state was created/changed there. This failed operation is not a pass.
6. c89d71: corrected preparation in authorized A1; write source identity registry
   and read/provenance hashes from unchanged oracle inputs.
7. 960965: create additive comparator and initial 25 exact boundary checks; pass.
8. 40e85a: strengthen all-status source binding and add two checks; 27 pass.
9. 23acfb: independent decoder validates all 20 registered input identities.
10. Seal: verify all prior seals, record this checkpoint and additive inventory.

Tools used only normal file operations and read-only Git with
GIT_OPTIONAL_LOCKS=0. Numerical work used existing `<VENV>/bin/python -B`
(Python 3.13.14), standard library only. No Rust, solver, host tooling,
maintained-source edit, Git/index mutation, delegation, new case, old expectation
change, or repaired-output read occurred. The bounded contribution is ready for
ROOT's review and subsequent output release.
