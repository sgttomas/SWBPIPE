# Demo friction reversal with stop release, under the hanger preload law

This package supplements the independent expectations held inline in
`core/product_physics/tests/friction_reversal_stop_release_runtime.rs`, in
the `EXPECTED` table at lines 61–66. That table is frozen and unchanged, and
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
| `generators/demo_friction_preload_reference.py` | the generator. It is a self-contained exact-rational 3D Euler–Bernoulli frame formed from the demo model's leaves, with every state enumerated. It holds a transcription of the frozen table, marked with its test-file line numbers and checked number by number against those lines. It writes the output and `SHA256SUMS` |
| `demo_friction_reversal_preload.json` | `rows` has the six scenarios under the law, each with exactly the fields of the test's `Expected` entries (case, variant, applied_y_n, stop_active, stop_uy_mm, stop_reaction_n, slip_mm, friction_n, normal_n, s130_fy_n, sh140_fz_n), in the test's units (N, mm, support actions on the structure). `notes` has, per scenario: the admissible state and its margins; every enumerated state with its standing and margins; the stiffness-only state; the applied force; each changed field next to its frozen (stiffness-only) value; and the exact rationals. `crosscheck` lists every check |
| `SHA256SUMS` | the hashes of every other file of this package. The generator writes it |

## Check

From this directory:

```sh
python3 -I generators/demo_friction_preload_reference.py --check
```

The check rebuilds the output into a temporary directory and compares it byte
for byte with the committed files. It changes no file. `--write` rewrites the
outputs, for intentional changes only.

## Method

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
and friction within 1e−9 N.

## Results under the law

Every scenario has exactly one strictly admissible state. The force margins are
all at least 4.70 N (the smallest is L-100 Original's normal, N = 4.70 N).
`notes[]` gives every margin.

| Scenario | NL-140 | NL-130-FRIC | S-130 action | Displacement margins |
|---|---|---|---|---|
| L-100 Original | active (as frozen) | **slide +UZ** (frozen: −UZ) | − (as frozen) | slip 0.579 mm |
| L-100 ReverseZ | active | slide +UZ | + | slip 13.87 mm |
| L-100 ReverseAll | released | slide +UZ | + | lift-off 4.50 mm, slip 13.33 mm |
| L-200 Original | active (as frozen) | **slide +UZ** (frozen: −UZ) | **+** (frozen: −) | slip 3.90 mm |
| L-200 ReverseZ | active | slide +UZ | + | slip 10.55 mm |
| L-200 ReverseAll | released | slide +UZ | + | lift-off 0.920 mm, slip 10.44 mm |

Under the law, SH-140's 430 N lifts N-130 and N-140 in Z in every scenario.
The shoe then slides +UZ in all six scenarios, and in both Original
scenarios it slides the opposite way from the frozen state. The Original and
ReverseZ variants therefore no longer slide in opposite senses. The stop
states, the released normals (= |applied Y|) and the frictions of the
released scenarios are unchanged. All other fields move; `notes[].changed`
lists each change.

## Independence

What is independent: the generator imports, reads and runs no product code
and no product output, and it copies no number into the derivation. Every
value comes from the demo model's leaves, in exact rationals; π is the
exception, as a 60-decimal rational. It reads the test file only to check its
transcription of the frozen table, which it uses for the reproduction check
and to stand beside each changed value. Its author saw no product value for
this model under the law.

What is not independent: the reference model is the test header's stated
basis. Its author read the test file, which also states which product rows
the values are compared with. The law as modelled is the author's reading of
the two REQ-014 texts.

All data are invented. Standard claim fence applies (F-PIP-2).
