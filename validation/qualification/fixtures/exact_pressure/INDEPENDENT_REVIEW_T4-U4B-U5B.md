# T4-U4b/U5b: independent admission review of the exact-pressure qualification delta

Reviewer: independent TASK ADMIT, Type 2. I did not write the product,
package generator, transport or references. This document incorporates
round 1 and the bounded round-2 review.

**Verdict: ADMIT the T4-U4b/U5b qualification-package mapping delta for
development comparison only.** No blocking finding remains. F1 is closed
under the corrected reference-preservation condition; F2 and the requested
F3 corrections are verified. F4 is a non-blocking documentation finding.

Paths below are relative to `projects/chirality-piping`.

## Reviewed inputs

- Product source: `3dc41f7a2246298f8e9cc6d5cc0e9c437621e04d`, branch
  `claude/piping-t4-u4b-u5b-20261017`.
- Round-1 source: `b15755dd098abb771aecbcf82a74f97a0fe80a79`.
  The only subsequent changes are documentation:
  - `a01721cc1d`: `u4b-u5b-implementation.md` and `b7-plan.md`;
  - `3dc41f7a22`: the exact-pressure package README.
- `validation/qualification/fixtures/exact_pressure/generate_package.py`:
  SHA-256 `efbb4026281ab75f8533aba6746306d45e81eac7ff59e42fc04483ed2bf244fc`.
- Committed `validation/qualification/fixtures/exact_pressure/MANIFEST.json`:
  SHA-256 `a930d5ac073ebc638670813c52fbbed3da9dfa5c276e8cf8bf7e56aa13793826`.
- Pending MANIFEST, generated without `ADMISSION.json` from inputs identical
  to this head:
  SHA-256 `abdcfc659221a1a8b515766256cc5fbab9be878e5e080132dc94d135fc7eda6d`.
  This is unchanged from round 1.
- Governing design:
  `execution/PKG-04/DEL-04-07/Design/u4b-u5b-implementation.md`,
  SHA-256 `e4795d7e955a7527de7e91eb8b81024d9dcc058615a374db3e8247f100ce3eb6`.
- Frozen package inputs:
  - `validation/references/t4_i7/u2_reference_cases.json`:
    `4bb9d0fa38e60c8dbcc531f112f4dc7012906c60470bc63e0686ebbec71fab66`;
  - `validation/references/t4_i7/u2_document_sketches.json`:
    `9ccb690f91a9f243330075f75ca622075392efc42bf13cca7810e6419d52f1ef`;
  - `validation/references/t4_i8/rebuilt_reference_cases.json`:
    `6f305474d200427aea27edb17119dd2cee8db1433b895933c224f0dacd4021e0`.
- pressure-2 table:
  `fixtures/results/semantic_contract_v0_3_pressure_2.json`,
  SHA-256 `e661d9d5ba21332a49152eb6aaa0f34a76d3a210aa3647d4249887b8e436d788`.
- Reused prior warrants:
  - `INDEPENDENT_REVIEW_T4-U6.md`:
    `6c9cdf95115c0e08382e8899ce8e95adc87b2c04e6100095188edc26477d3b1b`;
  - `INDEPENDENT_REVIEW_T4-U4A.md`:
    `a1d1c9fc85dbd7c628ed8423fe6672fee54847c990aefe3b9e757a6960a2b554`;
  - `INDEPENDENT_REVIEW_T4-RV28.md`:
    `5f18862a9d831251744dc6c1402ca7dfc3d7bb2badc1ef7c726dbea3252e3ab2`.

The checked-in `ADMISSION.json` still binds the T4-U6 review. This review is
the warrant for the manager's subsequent T4-U4b/U5b admission binding; it
does not assert that the new binding has already been installed.

## Independent checks

### Checks carried forward from round 1

The round-2 diff changes no generator, reference input, product source,
transport, reader dependency, selector, criterion or MANIFEST. The
following round-1 results therefore remain applicable and were not repeated:

- The package's mapping, MANIFEST and reader-binding checks passed.
- All 172 selector files use pressure-2. Exactly 266 former arc-maximum
  absences were removed. No T4-I7 arc maximum is scored: the reference
  supplies no corresponding maximum.
- The newly published section, toroidal, directional and tie-rod kinds
  introduce no positive or negative selectors in this package.
- Per mode, the package contains 78,116 positive assertions, 672 negative
  assertions and 1,386 absences.
- The eight MILLTOL documents retain their frozen 0.5.0 documents and case
  labels. The transport submits in-memory 0.6.0 copies changed by version
  alone, preserving shear model, allowances and load/reference-state group.
- All 5,136 nodal selectors remain unchanged.
- Reader dependency pins equal their selected file bytes. The gate can
  load its closed binding; the default refusal of an unreviewed binding
  remains effective.
- Stored-envelope checks passed all 1,077 positives, excluded all 96
  negatives and confirmed all 18 absences. These included 72 nodal
  assertions and six published arc-maximum rows. The newly published kinds
  were present without becoming qualification selectors.

### Checks performed in round 2

1. **Reference-preservation exception.**
   Compared with local `origin/main`
   (`a6fc4de3b600151d74598d7405cf0b2d11a105a2`), the complete
   `validation/references` diff contains exactly:
   - the RV-11 supersession paragraph in `t4_u5a/README.md`;
   - that README's digest on line 2 of `t4_u5a/SHA256SUMS`.

   Removing the added paragraph reproduces the baseline README exactly.
   Every other SHA256SUMS line is unchanged. No oracle, request, generator
   or numerical value differs. The complete t4_i7 and t4_i8 Git trees are
   identical to `origin/main`.

   `shasum -a 256 -c SHA256SUMS` passes all 22 entries in t4_u5a.
   An independent `python3 -I` SHA-256 check also passes all 22.

2. **README corrections.**
   The family absence counts are L-FREE 288, L-ANCH 278, U-ANCH 628,
   CBPT 96 and KINK 24. Including the unchanged I8 counts, they total
   1,386 per mode.

   The generator row now names pressure-2. The MILLTOL paragraph now
   distinguishes the frozen 0.5.0 documents and labels from the submitted
   in-memory 0.6.0 copies, changed by version alone.

3. **Reproducibility and pending identity.**
   At the reviewed head:

       python3 -I validation/qualification/fixtures/exact_pressure/generate_package.py --check

   reports:

       861 files, 0 differences, 860 case files not generated

   The missing case files are permitted by `--check`; it regenerates their
   bytes in memory and checks the committed MANIFEST.

   For the pending calculation, I verified that the scratch snapshot's
   generator, three reference inputs and pressure-2 table are byte-identical
   to this head and that it contains no `ADMISSION.json`. Running that
   generator with `--check` reports:

       861 files, 0 differences

   I captured the generated MANIFEST bytes in memory and independently
   recomputed their SHA-256 as
   `abdcfc659221a1a8b515766256cc5fbab9be878e5e080132dc94d135fc7eda6d`.

   The generator, committed MANIFEST and pending-MANIFEST hashes all equal
   round 1. The README is not a generator input or reader-binding
   dependency; its edit moves none of those identities.

4. **Scope and custody.**
   No repository file was modified. The working tree remained clean.
   No cargo, pytest, build or fresh solver execution was performed.

## Findings and dispositions

**F1 — CLOSED: the original preservation condition was incorrect.**

Round 1 treated any difference under `validation/references` as blocking.
The corrected condition permits the authorized t4_u5a RV-11 supersession
paragraph and its README checksum line.

Design §3.1 expressly specifies this note and its checksum update; the
Rulings record HELP_HUMAN's “no objection” to the supersession note. The
actual exception is exactly that change. All other reference bytes are
preserved, including every reference this package consumes. The checksum
checks pass. This is an authorized exception, not a reference defect.

**F2 — RESOLVED: stale absence counts.**

Commit `3dc41f7a22` corrects the five I7 family counts and the total to
1,386. The corrected text agrees with the round-1 package counts.

**F3 — RESOLVED for the requested corrections: generator table and
MILLTOL submission wording.**

Commit `3dc41f7a22` changes the generator row to pressure-2 and correctly
describes version-only in-memory re-authoring of the frozen MILLTOL
documents to 0.6.0.

**F4 — NEW, MINOR, OPEN: two other README passages still name pressure-1.**

`validation/qualification/fixtures/exact_pressure/README.md:136` says
dimensions and result families come from the pinned pressure-1 table.
Line 264 names pressure-1 among the reader-binding dependencies.

Both current implementations select
`fixtures/results/semantic_contract_v0_3_pressure_2.json`: the generator's
`TABLE` and the exact-pressure adapter's `TABLE` and `DEPENDENCIES`.
Following the binding guidance literally would select the wrong table and
fail the closed dependency check.

Replace these two prose references with pressure-2. This newly identified
finding was already present at round 1; it is not a runtime regression.
It changes no mapping, value, criterion or binding byte and does not block
development-comparison admission.

## Limits and admission scope

This is a delta admission. It reuses the unaffected T4-U6, T4-U4a and
T4-RV28 warrants and the unchanged round-1 checks. It does not repeat the
reference derivations or provide a fresh end-to-end solver or gate run.

The manager must bind this review by path and SHA-256 in `ADMISSION.json`
and regenerate the admitted package. Without a valid `ADMISSION.json`,
every generated reference remains `pending_independent_review` and every
criterion profile remains a draft.

Development comparison only. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).
