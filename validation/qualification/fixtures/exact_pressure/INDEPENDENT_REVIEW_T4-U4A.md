# T4-U4a: independent review of the VP-STATIC tangent mapping

Reviewer: independent TASK, Type 2, for T4's WORKING_ITEMS. I did not write
the implementation, package generator or frozen references.

**Verdict: ADMIT the U4a mapping delta.** No blocking finding remains. This
reviews the changed arc end mapping and its consumers; it reuses the valid
unchanged warrants in `INDEPENDENT_REVIEW_T4-RV28.md`. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

Paths below are relative to `projects/chirality-piping`.

## Reviewed inputs

- Product source: `d253b5aea29e5737552363d74d938cc229f0f34a`, including stage 1
  `51fa5e8ac4c9d347d5a52883febd22fb5bb19a8f`, against pre-U4a
  `bd3c773a8b002258693f6c0a73ae6e80e154da29`.
- `validation/qualification/fixtures/exact_pressure/generate_package.py`:
  SHA-256 `9a730af63af17affe669a505a973380181865c647e733a910ae4b875c09d6a89`.
- Unadmitted `validation/qualification/fixtures/exact_pressure/MANIFEST.json`:
  SHA-256 `90529efd4453334fd285331fd2090d406a797403df0369aad1467405a22ac673`.
- Governing convention: `execution/PKG-04/DEL-04-07/Design/arc-frames-and-signs.md`
  sections 2–4, 7 and 10.
- Frozen basis: `validation/references/t4_i7/u2_reference_cases.json` and
  `u2_document_sketches.json`; `validation/references/t4_i8/rebuilt_reference_cases.json`.
  Their bytes are unchanged from the reviewed baseline and remain bound by
  the package manifest.
- Reused review: `validation/qualification/fixtures/exact_pressure/INDEPENDENT_REVIEW_T4-RV28.md`,
  SHA-256 `5f18862a9d831251744dc6c1402ca7dfc3d7bb2badc1ef7c726dbea3252e3ab2`.

The candidate has no `ADMISSION.json`; reference readiness and criterion
profiles are pending. Admission may change readiness, review bindings and
review text only. The reviewed generator and numerical rules stay fixed.

## Independent checks

An independent read-only checker inspected every generated case file and
traced changed values directly to the frozen reference, without importing
the package generator.

- All 168 cases bind their five files by the manifest's SHA-256 values.
  The 336 runner-input and product-request files are byte-identical to the
  pre-U4a package. All 1,636 declared absences are unchanged. The ten T4-I8
  cases retain identical selectors, values and wrong values.
- Across all 158 T4-I7 document cases, 2,660 arc end assertions use the
  frozen `tangent_frame` block, node-on-element at each end. Each value is
  bit-identical to its frozen decimal rounded to binary64. Of those values,
  1,798 differ from the former chord mapping. Metadata equals the Design's
  tangent-frame sign text, `element_local` and the stiffness recovery basis.
- Force end rows use the frozen `shear_force` floor; moment end rows use
  `section_moment`. Every changed assertion's relative criterion is 1e-9,
  and its absolute floor equals the reference's 1e-9 times zero scale.
  All 2,196 criteria shared with the former package are unchanged apart
  from admission review text.
- Every arc endpoint section action equals minus its end-i row or plus its
  end-j row exactly in decimal. Its pointer is `implied_by_positive`, never
  `not_published`. Only the omitted elastic axial end row remains an end-row
  publication gap; arc Lamé and maximum absences remain intact.
- Twenty-six formerly unscored arc end-station negative controls now score
  the end row with the correct end-i negation. Their wrong values and
  unchanged rules were checked directly; each wrong value is excluded.
  Package totals per mode are 55,724 positive and 574 negative assertions.
- Three actual product-generated envelopes in
  `fixtures/results/pressure_v3_arc_reader_corpus.json` resolve all 777
  positive selectors and 96 negative selectors uniquely with their full
  metadata, unit, basis and row ID. Their values satisfy the corresponding
  positive rules and exclude the negatives; all 21 declared absences hold.
  These exercise 0.3.0 sparse, 0.4.0 dense and the dense kink control. The
  resolution was checked independently of the generator and agrees with
  `tools/validation/qualification_exact_pressure.py::resolve_row`.

## Product evidence and limits

The source review confirms that tangent axes enter the certified operator
and equivalent loads in ball arithmetic before row evaluation; binary64
sections use the same end-j frame as their published rows. The shared
recovery law also serves nonlinear confirmation. The forced pressure-1
demotion is test-only and removes certification before publication; its
test requires no certified records and checks the unchanged families.

Passing focused evidence was reused: the frozen T4-I7 cases in both modes
and document versions; U4a's twenty cases through both entries and modes;
the pressure binary64 controls; actual tangent coefficient enclosures and
end-group recomputation; and pressure arc reader checks. The reviewer also
independently checked all 162 stored axis decompositions, including the
exact-binary64-radius T0R supplements. Frozen oracle bytes were preserved.

This review does not repeat RV28's unchanged full CLI sweep, straight
mapping, exact evaluator or adapter refusal probes. Its three-envelope
selector check is a focused consumer check, not a new full 336-run gate
claim. B7a's remaining preview fixture regeneration, semantic digests,
pinning and registration remain required by its H-4 wave. K-D5 status
remeasurement and exact-head integration checks remain with the managers.
This admission concerns development comparison only.
