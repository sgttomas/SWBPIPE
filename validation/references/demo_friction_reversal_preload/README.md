# Demo friction reversal with stop release, under the hanger preload law

This package supplements the independent expectations held inline in
`core/product_physics/tests/friction_reversal_stop_release_runtime.rs`, in
the table named `EXPECTED` (lines 61–66 at the staging commit). That table is frozen and unchanged, and
no generator for it is committed. The table was derived with the bundled demo
model's hanger `support:SH-140` as a bare 42000 N/m UZ spring, although the
record (`fixtures/product_preview/invented_demo_model.json`) carries an
entered `cold_load` of 430 N.

This package rebuilds the reference by the method the test's header states
and derives the same six scenarios under the law that
`execution/PKG-04/DEL-04-03/ScopeOfWork.md` REQ-014 and
`execution/PKG-04/DEL-04-11/ScopeOfWork.md` REQ-014 state:

- SH-140 is a spring of its rate on UZ, plus F_cold = 430 N as a constant
  force on the pipe along +Z at N-140. It acts in every load case and every
  variant: a variant negates the case's primitive loads only, and the cold
  load belongs to the support, not to a primitive load;
- SH-140's published Fz is F_cold − k·u_z;
- the cold load is also published separately as SH-140's applied force,
  430 N along +UZ (`notes[].sh140_applied_force_n`);
- `installed_load` and `hot_load` are review data and are not applied.

Status: these are candidate references. Their author TASK derived them
without seeing any product output. They have not yet been independently
refuted.

## Files

| File | Role |
|---|---|
| `generators/demo_friction_preload_reference.py` | the generator. It is a self-contained exact-rational 3D Euler–Bernoulli frame formed from the staged model's leaves, with every state enumerated. It writes the output and `SHA256SUMS` |
| `staged/invented_demo_model.json` | an input staged from `fixtures/product_preview/invented_demo_model.json` as committed at `3c8fc26a6c188f1613714f56d9807dc32433c46d`: the whole model document, byte for byte, because the derivation reads most of it |
| `staged/frozen_expected_table.json` | an input staged from `core/product_physics/tests/friction_reversal_stop_release_runtime.rs` at the same commit. It holds the table named `EXPECTED` (`const EXPECTED: [Expected; 6]`), the only table this package supplements even if the file holds others: the six rows verbatim and in order, and the author's transcription of them. `line_at_staging_commit` records where each row stood at that commit (lines 61–66); it is a record only. The generator checks the transcription against the staged rows number for number |
| `demo_friction_reversal_preload.json` | `rows` has the six scenarios under the law, each with exactly the fields of the test's `Expected` entries (case, variant, applied_y_n, stop_active, stop_uy_mm, stop_reaction_n, slip_mm, friction_n, normal_n, s130_fy_n, sh140_fz_n), in the test's units (N, mm, support actions on the structure). `notes` has, per scenario: the admissible state and its margins; every enumerated state with its standing and margins; the stiffness-only state; the applied force; each changed field next to its frozen (stiffness-only) value; and the exact rationals. `crosscheck` lists every check |
| `SHA256SUMS` | the hashes of every other file of this package. The generator writes it |

## Check

From this directory:

```sh
python3 -I generators/demo_friction_preload_reference.py --check
```

The check rebuilds the output into a temporary directory and compares it byte
for byte with the committed files. It changes no file and opens nothing
outside this package; in particular it opens no file under `core/`. It
establishes that the committed output follows from the staged inputs.
`--write` rewrites the outputs, for intentional changes only.

```sh
python3 -I generators/demo_friction_preload_reference.py --check-live
```

This separate check, which `--check` does not run, does two comparisons. It
compares the staged model with the live fixture on only the leaves that enter
the derivation (`DERIVATION_LEAVES` in the generator), so the fixture's
descriptive strings are ignored. It also finds the table named `EXPECTED` in the live test file by its
declaration, wherever it sits, and compares its rows, in order, with the
staged rows; line numbers play no part. It names every differing leaf or row,
including a missing or extra one, and exits nonzero. If it cannot find the
declaration exactly once, it says so and exits nonzero; it does not guess. It
establishes whether the staged inputs still match their sources. A difference
means a source has moved on and a decision is needed on re-staging; the
committed output still stands for its staged inputs.

## Method

**Axis and sign of the cold load.** The cold load acts along the positive axis
of the hanger's declared stiffness DOF (+Z here). This reading comes from
DEL-04-11 REQ-014 ("through the constant-effort seam") together with DEL-04-03
REQ-014's positive-axis convention for the constant-effort support; the
spring's own sentence in DEL-04-03 does not say "positive axis". If the sign
were decided otherwise, every value here that the cold load moves would
change, and the admissible states with them.

The method follows the test header, and every input is read from the demo
model's leaves:

- **Section and material.** Annular A and I, with J = 2I. E and G come from
  the material record, and π is a 60-decimal rational (Machin's formula).
- **Loads.** Line loads are consistent; the thermal interval acts as the
  initial strain α·ΔT.
- **Supports.** S-100, S-120 (UX and UZ) and S-130 (UY) are rigid, and SH-140
  is a UZ spring at zero reference. The components and CE-120 are not
  consumed, as the test's stated basis says (its request declares C-130
  `not_solver_consumed`).
- **Nonlinear supports.** NL-140 is a one-way stop on UY that makes contact
  for a negative action. NL-130-FRIC is zero-reference Coulomb friction on UZ
  with μ = 0.01 and N = |S-130 UY action| of the same state, coupled exactly
  in the slide rows.

Every state is solved exactly: NL-140 {active, released} × friction {stick,
slide +UZ, slide −UZ} × sign of the S-130 action. A state is strict when every
margin is positive. The margins are:

- the stop's action (−R) when it is active, or N-140's −UY when it is
  released;
- μN − |F| when the shoe sticks, or dir·UZ when it slides;
- the S-130 action in the state's sign.

Each scenario must have exactly one strict state and no weak state. The
generator also checks exactly that the anchor carries no Y force, that Y
balances, that a released stop gives N = |applied Y|, and the Coulomb
relations.

**Reproduction.** With F_cold set to zero, all six scenarios give the frozen
states, and every field matches the table at the test's own criteria: frame
quantities within 1e−9 relative, an exact zero within 1e−9 of the applied Y,
and the friction force within 1e−9 times the scenario's normal force N. That
bound is one billionth of the normal force; it is not a fixed number of
newtons.

## Results under the law

Every scenario has exactly one strictly admissible state. The smallest force
margin is L-100 Original's normal, N = 4.697951333494526 N as the output gives
it (at least 4.697 N); every other force margin is larger. Margins quoted here
are truncated, never rounded up. `notes[]` gives every margin as an exact
rational (`exact`), which governs, with its nearest binary64 value (`value`).
That value can exceed the exact rational by up to half a unit in the last
place, so read lower bounds from `exact`.

| Scenario | NL-140 | NL-130-FRIC | S-130 action | Displacement margins |
|---|---|---|---|---|
| L-100 Original | active (as frozen) | **slide +UZ** (frozen: −UZ) | − (as frozen) | slip 0.579 mm |
| L-100 ReverseZ | active | slide +UZ | + | slip 13.86 mm |
| L-100 ReverseAll | released | slide +UZ | + | lift-off 4.49 mm, slip 13.32 mm |
| L-200 Original | active (as frozen) | **slide +UZ** (frozen: −UZ) | **+** (frozen: −) | slip 3.90 mm |
| L-200 ReverseZ | active | slide +UZ | + | slip 10.54 mm |
| L-200 ReverseAll | released | slide +UZ | + | lift-off 0.920 mm, slip 10.43 mm |

Under the law, SH-140's 430 N lifts N-130 and N-140 in Z in every scenario.
The shoe then slides +UZ in all six scenarios, and in both Original
scenarios it slides the opposite way from the frozen state. The Original and
ReverseZ variants therefore no longer slide in opposite senses.

From the output's own change list (`notes[].changed`, which gives each
changed field beside its frozen value):

- **Every scenario:** `applied_y_n` and `stop_active` are unchanged.
- **The four scenarios where the stop holds** (Original and ReverseZ of both
  cases):
  - `stop_uy_mm` (0) is unchanged;
  - `stop_reaction_n`, `slip_mm`, `friction_n`, `normal_n`, `s130_fy_n` and
    `sh140_fz_n` change.
- **The two released scenarios** (ReverseAll of both cases):
  - `stop_reaction_n` (0), `friction_n`, `normal_n` (= |applied Y|) and
    `s130_fy_n` are unchanged;
  - `stop_uy_mm`, `slip_mm` and `sh140_fz_n` change.

## Independence

This package is independent of the product. It is not independent of the
stated basis it builds on: the method stated in the test file's header.

What is independent: the generator imports, reads and runs no product code
and no product output, and it copies no number into the derivation. Every
value comes from the staged model's leaves, in exact rationals; π is the
exception, as a 60-decimal rational. The frozen table enters only through its
staged transcription, which the generator uses for the reproduction check and
to stand beside each changed value. Its author saw no product value for this
model under the law.

What is not independent: the reference model is the test header's stated
basis. Its author read the test file, which also states which product rows
the values are compared with. The law as modelled is the author's reading of
the two REQ-014 texts.

All data are invented. Standard claim fence applies (F-PIP-2).
