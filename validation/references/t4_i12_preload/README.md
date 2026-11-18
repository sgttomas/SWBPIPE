# T4-I12 preload addendum: U3 system cases under the hanger preload law

This package supplements the frozen package [`../t4_i12`](../t4_i12/README.md),
file `u3_reference_cases.json`. The frozen package and its files are
unchanged.

Its system cases solve the re-authored invented demo. In every one of its
nine system documents, the variable spring hanger `support:SH-140` (node
N-140, rate 42000 N/m on UZ) carries an entered `cold_load` of 430 N. Round
02 restored that value verbatim from the committed demo fixture
`fixtures/product_preview/invented_preview_model.json`. The hanger record's
mechanical leaves are staged in this package, and the generator checks the
nine documents' hanger records against that staged copy. The frozen generators modelled
the hanger as a bare spring, so the cold load was not applied. This package
derives the same quantities under the law that
`execution/PKG-04/DEL-04-03/ScopeOfWork.md` REQ-014 and
`execution/PKG-04/DEL-04-11/ScopeOfWork.md` REQ-014 state:

- the hanger is a spring of its rate on UZ, plus F_cold = 430 N as a
  constant force on the pipe along +Z at N-140, in every load case;
- its published support-on-pipe row is F_cold − k·u on UZ (no case moves its
  base);
- the cold load is also published separately as the support's applied force,
  430 N along +UZ in each admitted case;
- `installed_load` (460 N) and `hot_load` (390 N) are review data and are not
  applied.

Status: their author TASK derived these references without seeing any
product output, and an independent reader that saw no product output
examined the derivation and the values before the package was frozen.
They are not product evidence.

This package was frozen before the preload law (DEL-04-11 REQ-014) was
implemented, so no product test read it then; the product tests that read it
arrive with that law. Its own check and the tests of the reference-package
table (`tests/reference_packages.py`) maintain it.

## Files

| File | Role |
|---|---|
| `generators/u3_preload_reference.py` | the generator. It runs the frozen round-00/01 generator `../t4_i12/generators/u3_reference.py` in its own namespace, writing that generator's output only to a temporary directory, to use its frame, connector, document interpreter, document solver and formatting. It reads the request documents and frozen values from `../t4_i12/u3_reference_cases.json` and the hanger leaves from `staged/sh140_hanger_leaves.json`. It writes `u3_reference_cases_preload.json` and `SHA256SUMS` |
| `staged/sh140_hanger_leaves.json` | an input staged from `fixtures/product_preview/invented_preview_model.json` as committed at `3c8fc26a6c188f1613714f56d9807dc32433c46d`: the mechanical leaves of `support:SH-140` (id, node, family, the stiffness DOF and rate, and the installed, cold and hot loads and travel range), copied as they stand. The fixture's descriptive strings are not staged |
| `u3_reference_cases_preload.json` | `cases/<case id>/…` holds each derived subtree of the affected cases at the same JSON pointer and in the same string format as the frozen file, so that a test compares what it compared before. `inputs` gives each request document's pointer in the frozen file. `preload` gives the hanger, the published applied force, every changed leaf next to its frozen (stiffness-only) value, and the unaffected parts with the reason for each. `crosscheck` lists every check |
| `SHA256SUMS` | the hashes of every other file of this package. The generator writes it |

## Check

From this directory:

```sh
python3 -I generators/u3_preload_reference.py --check
```

The check rebuilds the output into a temporary directory and compares it byte
for byte with the committed files. It changes no file and opens nothing outside
this package and `../t4_i12`. It establishes that the committed output follows
from the frozen package and the staged hanger leaves. `--write` rewrites the
outputs, for intentional changes only.

```sh
python3 -I generators/u3_preload_reference.py --check-live
```

This separate check, which `--check` does not run, compares the live fixture's
SH-140 record with the staged leaves that enter the derivation: the id, node
and family, the stiffness DOF and rate, and the cold load. It names every leaf
that differs, including a member that is missing or extra among them, and
exits nonzero. A difference means the source has moved on and a decision is
needed on re-staging; the committed output still stands for its staged input.

The comparison is partial: it reads only those leaves. It does not see a
member added beside them that the product gives precedence (a stiffness on
the support record itself, for one), a changed member it does not list, or a
JSON form the product would reject. The maintained test that runs this check
(`tests/test_reference_packages_live.py`) therefore also holds the digest of
the whole fixture: any change to it fails that test until someone has
confirmed the change against this package.

The staged file also records the installed load, hot load and travel range.
The live check does not compare them, because they are review data the law
does not apply. The default check still requires the nine documents to carry
them as staged.

## Affected cases and derived subtrees

| Case | Subtree (same pointer as the frozen file) |
|---|---|
| `U3-SYS-DEMO-CONNECTOR-001` | `expected/<L-100, L-200, L-300>` and `wrong_result_discriminators/<case>`, which holds the raw-difference element solve and the retained-span parallel solve |
| `U3-SYS-DEMO-CONNECTOR-002-LR1` | `expected/<case>` |
| `U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL` | `boundary_zero_strain/expected_values/<case>` |
| `U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL` | `wrong_result_discriminators/<case>` |
| `U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL` | `expected/<case>` |

These parts are unaffected and are not repeated here:

- the structural refusals (`refusal_variants`, and the refusal case's
  `expected` a, b and c), which depend on what the replaced span owns, not on
  the hanger;
- the material control's `expected`, which is an equality statement and still
  holds under the law (checked);
- every non-system case, none of which has a hanger in its model (checked);
- `round_02.zero_floors`, which covers unit-level arrays only.

## Method and results

1. **Reproduction.** With F_cold set to zero, every derived subtree of the
   five cases equals the frozen file's string for string.
2. **The law.** The cold load enters the frozen document solve as a nodal
   force along +UZ at N-140, and +F_cold is added to the hanger's published
   row. The raw-difference discriminator uses the frozen generator's
   hard-coded demo solve, whose loads table gains the same nodal force. The
   generator checks that this demo solve equals the document solve under the
   law. It also checks:
   - the global balance of the document's own applied loads plus every
     published reaction (independently formed; the cold load is inside the
     hanger's row, not an applied load);
   - that the hanger's row is exactly F_cold − k·u;
   - that every string is stable under a second π truncation;
   - that the frozen equalities (002-LR1, zero-strain boundary, stored
     control and material control, all equal to 001) still hold.

   The models have no nonlinear supports, so there is no state enumeration.
3. **Effect, `load:L-100` as an example.**
   - SH-140's row rises from 253.55 N to 375.27 N.
   - N-140's UZ moves from −6.04 mm to +1.30 mm.
   - The guide S-120's Fz falls from 927.07 N to 735.92 N.
   - The anchor S-100's rows change.
   - The connector's axial end action follows the hanger's row.
   - The zero-scale floors change with the rows.

   The discriminating and moved-component lists are unchanged, and so are the
   raw-difference global imbalances (−770, −275 and 440 N·m): the cold load
   acts along the connector axis r = N-140 − N-130, so r × F_j does not
   change. The frozen text quoting those imbalances therefore still holds.

**Axis and sign of the cold load.** The cold load acts along the positive axis
of the hanger's declared stiffness DOF (+Z here). The positive axis is the
integrator's decision for both hanger families, read from DEL-04-03 REQ-014's
convention for the constant-effort support and DEL-04-11 REQ-014's "through
the constant-effort seam"; the words "positive axis" enter the spring's own
sentence in both scopes with the preload landing. If the sign
were decided otherwise, every value here that the cold load moves would
change.

**Sign convention.** The frozen generator publishes a spring's row as −k·u
(`solve_doc`: "a spring's is −k u"; conventions: "reactions: support-on-pipe").
This package adds +F_cold on the hanger's DOF, which is the law's
F = F_cold − k·(u − g) with g = 0.

## Independence

This package is independent of the product. It is not independent of the
frozen package it builds on: it uses the frozen generator's frame and solver.

What is independent: the generator imports, reads and runs no product code and
no product output. It reads the demo fixture only for the hanger's leaves. It
copies no number. Every value comes from the documents' leaves, in exact
rationals, with π as the frozen generator's 125-decimal rational. Its author
saw no product value for these models under the law.

What is not independent: it reuses the frozen generator's frame, connector,
document interpreter and solver, so it inherits their correctness. That
shared code is reference code, not product code. The law as modelled is the
author's reading of the two REQ-014 texts.

All data are invented. Standard claim fence applies (F-PIP-2).
