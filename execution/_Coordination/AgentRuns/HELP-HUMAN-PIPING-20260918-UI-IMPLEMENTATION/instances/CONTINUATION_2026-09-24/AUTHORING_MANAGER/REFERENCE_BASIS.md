# M24 independent conversion basis

Checked 2026-09-24 against current primary publications, independently of the implementation catalog.

The [BIPM current SI Brochure landing page](https://www.bipm.org/en/publications/si-brochure) identifies the ninth edition with a 2026 update. Its [SI prefix table](https://www.bipm.org/en/measurement-units/si-prefixes) gives kilo = 10³, giga = 10⁹, and milli = 10⁻³. Thus 1 kN = 1000 N, 1 kN·m = 1000 N·m, 1 GPa = 10⁹ Pa, 1 N/mm = 1000 N/m, and 1 kN/mm = 10⁶ N/m. These compound factors follow by multiplication/division of the published prefix definitions.

[NIST SP 811 Appendix B.8](https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b8) states the bar conversion to pascals, yielding 1 bar = 100000 Pa. This unit identity conveys no absolute/gauge pressure kind. Changing kind needs an explicitly supplied reference pressure; a token must not supply one.

[NIST temperature guidance](https://www.nist.gov/pml/owm/si-units-temperature) states the Celsius/Fahrenheit interval ratio: 1 Celsius degree interval = 1.8 Fahrenheit degree intervals. Therefore 1 Fahrenheit degree interval = 5/9 K and its reciprocal is 1/(degree Fahrenheit interval) = 9/5 K⁻¹. A coefficient 10⁻⁵/degF represents 1.8×10⁻⁵/K. This is an interval inverse; absolute Fahrenheit's affine offset does not enter it. Absolute 0 degC is 273.15 K; a 10 degC interval is 10 K.

These exact unit definitions are applicable to M24; newer structural mechanics theory cannot replace dimensional definitions. Numeric test witnesses use independently stated magnitudes and negative dimension/offset controls, without deriving expected results from the implementation's factor metadata. The separately owned product seam uses free uniform expansion under its explicit linear small-strain assumptions, not a general proof of solver mechanics.
