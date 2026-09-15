# Source-freeze addendum V1 — interrupted refactor and numerical trigger

This addendum does not amend `SOURCE_FREEZE_V1.json`. Root's hash preflight stopped before independent refutation because an implementer refactor overlapped the root freeze message. The refactor was reversed byte-for-byte immediately. Current verified product hashes remain:

- `core/product_physics/src/lib.rs`: `aa91613c48346654dc7e8b110f22fc74cf9578fb16f21ae9c40cec863b910f15`
- `core/product_physics/src/pressure_exact.rs`: `855144bf880370b5e40b57ecfc5e08fe3dec5929fc6d4a0d824ef0c9487bcd32`

No further source effect is authorized unless root routes a concrete finding.

## Triggering numerical concern

The interrupted change arose from a valid positive-subnormal-area annulus beyond the frozen oracle's named cases:

```text
ri = 2^-537 = 2.2227587494850775e-162 m
ro = 1.125*ri = 2.500603593170712e-162 m
r  = ri + (ro-ri)/2 = 2.361681171327895e-162 m
p  = 3 Pa
```

Binary64 factored wall area `PI*(ro-ri)*(ro+ri)` is positive and representable at one minimum subnormal, `5e-324`, so the accepted constructor must admit this geometry under the frozen predicate. The unscaled squared-radius difference rounds to zero. At the stated interior radius, the current source's factored partial outside area `(PI*(ro-r))*(ro+r)` also rounds to zero before its division by `As`.

An independent 110-digit Decimal calculation from the exact binary64 inputs gives:

```text
radial stress = -1.3677997150417260329737431304701811520455933238347242010991247710156727050681864441278241400366374923671890902 Pa
f64 radial    = -1.367799715041726
trace         = 22.588235294117647058823529411764705882352941176470588235294117647058823529411764705882352941176470588235294119 Pa
f64 trace     = 22.58823529411765
hoop stress   = 23.956035009159373091797272542234887034398534500305312436393242418074496234479951150010177081213108080602483209 Pa
f64 hoop      = 23.956035009159372
```

Code-path inspection of the frozen implementation indicates the rounded-zero partial area makes interior radial stress zero. Its transverse trace also derives from separately rounded subnormal `Ai`, `P`, and `As`, yielding 18 Pa rather than the direct dimensionless-geometry value above. The same rounding can affect axial membrane stress even when the final stress is normal and representable. This is a concrete refutation input, not a request to enlarge tolerance or add a geometry cutoff.

The interrupted refactor replaced the partial-area quotient with dimensionless factored radius ratios. That would preserve the interior radial fraction, but it did not yet repair the transverse-trace or axial-stress path and therefore was not a complete correction. It is intentionally excluded from the restored freeze.

Requested root action: include this input in independent refutation and rule whether the contract requires all final representable stresses to survive positive-subnormal intermediate area rounding. The accepted statements that positive representable areas are valid, representable outputs must not be rejected, and no minimum wall/geometry threshold exists indicate that it should.
