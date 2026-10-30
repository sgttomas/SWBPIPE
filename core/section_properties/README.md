# Pipe Section Property Calculator

`DEL-03-08` implements bounded pipe section and mass-property calculations from
user-entered dimensions and densities.

The calculator is intentionally narrow:

- it accepts already-parsed values, not external catalog formats;
- it does not provide pipe schedule, dimensional, material, or component
  defaults;
- it does not convert units until the project unit catalog and conversion
  constants are accepted;
- it records blocking diagnostics for missing, non-positive, or incompatible
  inputs;
- it marks calculated values as derived from user-entered data.

## Calculated Values

For circular pipe sections, the calculator can derive:

- effective inside diameter after corrosion allowance;
- metal area;
- second moment of area;
- elastic section modulus;
- polar/torsional constant;
- contents volume per length;
- metal, contents, insulation, and total mass per length when matching density
  inputs are provided.

Inputs remain user-supplied. Public fixtures and public schemas must not use
this module to smuggle protected dimensional tables, proprietary catalog values,
code-specific allowables, SIFs, flexibility factors, or private project data
into the repository.
