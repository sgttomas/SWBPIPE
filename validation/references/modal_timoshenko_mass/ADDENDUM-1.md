# Addendum 1 to the problem statement

This addendum extends the frozen problem statement, PROBLEM.md, SHA-256 `ab4a949c2533603d91d27cdeff561207d0339997dbe1e53bbc783ca8cc00784d`. It adds three cases and one output rule. Everything in PROBLEM.md continues to apply unchanged to the five original cases and to these three: the member, the axes, the degrees of freedom, the mass by definition, the self-checks and the output format. Only the rule below is added.

## Added output rule: a shear parameter outside binary64

Φ_y or Φ_z may lie outside the binary64 range: above the largest finite binary64 value, or below the smallest positive subnormal. In that case:
- give the decimal form as PROBLEM.md prescribes (forty significant digits in scientific notation);
- give no (hi, lo) pair for that value;
- give instead its exact rational value, as a numerator and a denominator written as decimal integers in lowest terms with a positive denominator.

The plane matrices and the global 12×12 are given exactly as PROBLEM.md prescribes.

## The added cases

The table gives each case's values for orientation. The bit patterns after it are the operands; where the two differ, the bit patterns govern.

| Id | x_i | x_j | v | E | G | A_s | I_y | I_z | m̄ | ī | j̄ |
|---|---|---|---|---|---|---|---|---|---|---|---|
| T-RANGE-SUBFLOOR | (0, 0, 0) | (1, 0, 0) | (0, 1, 0) | 1 | 1 | 12 | 1 | 1 | 3360·2⁻¹⁰⁰⁰ | 120·2⁻¹⁰⁰⁰ | 120·2⁻¹⁰⁰⁰ |
| T-RANGE-PHI-HUGE | (0, 0, 0) | (1, 0, 0) | (0, 1, 0) | 2¹⁰⁰⁰ | 1 | 12 | 2¹⁰⁰⁰ | 2¹⁰⁰⁰ | 1 | 1 | 1 |
| T-RANGE-PHI-TINY | (0, 0, 0) | (1, 0, 0) | (0, 1, 0) | 2⁻¹⁰⁰⁰ | 1 | 12 | 2⁻¹⁰⁰⁰ | 2⁻¹⁰⁰⁰ | 1 | 1 | 1 |

Operand bit patterns:

### T-RANGE-SUBFLOOR
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 3ff0000000000000, 0000000000000000, 0000000000000000
- v: 0000000000000000, 3ff0000000000000, 0000000000000000
- E: 3ff0000000000000
- G: 3ff0000000000000
- A_s: 4028000000000000
- I_y: 3ff0000000000000
- I_z: 3ff0000000000000
- mbar: 022a400000000000
- ibar: 01de000000000000
- jbar: 01de000000000000

### T-RANGE-PHI-HUGE
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 3ff0000000000000, 0000000000000000, 0000000000000000
- v: 0000000000000000, 3ff0000000000000, 0000000000000000
- E: 7e70000000000000
- G: 3ff0000000000000
- A_s: 4028000000000000
- I_y: 7e70000000000000
- I_z: 7e70000000000000
- mbar: 3ff0000000000000
- ibar: 3ff0000000000000
- jbar: 3ff0000000000000

### T-RANGE-PHI-TINY
- x_i: 0000000000000000, 0000000000000000, 0000000000000000
- x_j: 3ff0000000000000, 0000000000000000, 0000000000000000
- v: 0000000000000000, 3ff0000000000000, 0000000000000000
- E: 0170000000000000
- G: 3ff0000000000000
- A_s: 4028000000000000
- I_y: 0170000000000000
- I_z: 0170000000000000
- mbar: 3ff0000000000000
- ibar: 3ff0000000000000
- jbar: 3ff0000000000000
