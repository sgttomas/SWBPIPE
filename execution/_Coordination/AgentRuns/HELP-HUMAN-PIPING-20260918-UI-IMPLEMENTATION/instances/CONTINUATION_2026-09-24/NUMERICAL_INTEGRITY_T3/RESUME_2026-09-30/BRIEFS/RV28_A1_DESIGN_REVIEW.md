# RV28 — independent A1 design re-derivation

Fresh TASK reporting directly to ROOT, separate from designer and I22.
Read Root AGENTS, TASK, Piping AGENTS, COMMON and this brief.
Use software-code-review skill for the bounded design/source review.
No delegation, Git/index mutation, Rust/solver experiment, source edit or host tool.
Standard-library exact arithmetic and pinned-source reading are permitted.

Review design_a1/DESIGN_PROPOSAL.md (SHA256
926dea73178b0ecde07203fdf7fc85e5ce752b1a5f6ead7cf31a30673249b178),
seal 080de38cd614012a7eba50b28b12e52ac122fb8246747e77f88fa9a82ab6c34c,
against source d01ad98a... / audit 3bddc2b... and accepted D1 R7/A1/A2/D2.
Diagnostic basis a6b40d2d036acac556e28f28f5b482a4adb39333 confirms C17.
Do not accept the designer's exact_checks.py as an independent oracle.

Independently re-derive:
- full pre-coupling rounding/candidate-verification error for all four directions,
  max switching, O9, L=0, zero/subnormal/nonfinite scales, threshold and p512 floor;
- actual certified error formulas for every row, tracing each proposed report
  field to its implementation and upstream R7 proof. In particular distinguish
  formation scale E_q from a certified error, keep all W/C/charge/magnitude terms,
  prescribed components, data/no-data blocks and unavailable-bound semantics;
- exact H=|actual final publication x - verification v| + certified error,
  its arithmetic representation/refusal/charge and stable escalation;
- bare b versus qualified interval semantics, relative public predicate and
  protected tighter predicate, G5a lower checks, D2 exact re-derivation, final
  unit/derived-row certification. Do not let the old scale-transfer premise
  re-enter through stress, receipt or conversion reasoning;
- producer/finalization/reader identity consistency, actual available lifetimes,
  required K6c re-accounting and testability/mutants.

Try to break the proposed guarantee with independently derived exact cases.
Name assumptions instead of proving by comments or old green tests. Assess
whether the smallest kernel slice is fully specified and what before-F2a work
remains. Separate mathematical sufficiency from executable design completeness.
No changed guarantee can rely on an unreviewed design amendment.

Report which selections stay within the owner's already accepted conservative-
refusal direction and which genuinely change published contracts/domain or
protected availability. Do not invent an extra owner checkpoint for routine
implementation, but do not select a product-semantic change for ROOT.
Bare versus qualified interval remains a proposal until ROOT disposes your review.

Write only <A1_WT>/<R>/design_review_RV28/** and <wt>/scratch/rv28-a1-design/**.
Return REVIEW.md: VERIFIED / NOT VERIFIED, precise blocking/should-fix/notes,
source locations, independent derivation/checks, concrete remedies and residual
limits. Preserve seals. The same reviewer backchecks fixes. No implementation
or design acceptance is granted by a review return alone.

