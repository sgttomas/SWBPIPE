# T4-U1b references: the arc load certificate and the stable small-angle load vector

Consumed by FK `src/structural/arc_certificate_tests.rs` (T4-I9 DESIGN_R01
§6.1 V2, V2b and V2d) and CB `src/load_vector_tests.rs` (T4-RV8 C-1).
Produced by T4-I17 (T4-U1b's implementer), 2026-10-10.

## Sources, bytes unchanged

- `_run_records/rv129_ref.py`: T3's RV129 independent reference for the
  arc load vector (Decimal 120 digits, Gauss-Legendre quadrature of the 3D
  force method, its own trigonometry; no closed form shared with CB or the
  certificate). NUM `RESUME_2026-09-30/REVIEW_RV129/t4_i9_01/_run_records/tools/rv129_ref.py`,
  sha256 `30a46aedecd9464c80a7f6dc3d822c959879fa0ca56336f775e2b23042b92115`,
  the line in that review's `SHA256SUMS` (whose own sha256 is `c09e3fdb…`).
- `_run_records/arc_cert_probe_r01.py`: T4-I9 revision 01's executable
  specification of the certificate (DESIGN_R01 §3.2-§3.3), sha256
  `e7808c5dda059c292d4117e4c083e75140ea4e8bca2fadf41fe6cf7746b106e8`, the
  line in revision 01's `SHA256SUMS` (digest `a0a319ee…5e0375`), confirmed by
  T4-RV8.

## Generated here (standard library only)

- `arc_load_references.txt` from `t4_i17_arc_refs.py`
  (`python3 -I t4_i17_arc_refs.py _run_records/rv129_ref.py`): 79 arcs (RV129's
  39 + 16 extra cases, its 4 near-π skew chords, RV8's 0.3 m chord from 1e-8
  rad to 90° with an out-of-plane and an in-plane load, and T15's body at
  k = 2 and 1e10). Each component is stored as four binary64 terms summing
  exactly to the 120-digit value, with an upward bound e on the generator's
  error (1000 times the difference from a 140-digit / 64-node evaluation,
  plus the representation residual, plus 1e-100 of max|f|).
- `arc_certificate_pins.txt` from `t4_i17_bit_pins.py`
  (`python3 -I t4_i17_bit_pins.py _run_records/arc_cert_probe_r01.py`): the
  specification's midpoint splits, radii and ρ̂ for 25 arcs (17 certified,
  8 refused with their DESIGN_R01 §3.5 class) and 10 adversarial lemma
  inputs.
- `load_series_gen.py`: the exact rational Taylor series of the 15
  uniform-load integrals in CB `arc_integrals.rs` (`LOAD_*`); its output is
  pasted there. `check_series.py` checks each series against direct 45-digit
  quadrature at φ = 0.3, 0.8 and 1.04 (all within 2.4e-17 relative).

`SHA256SUMS` lists every file here; check with `shasum -a 256 -c SHA256SUMS`.
