# Addendum 1: portable evaluation of two illustrative growth observations

15 November 2026.

This addendum corrects two illustrative `growth_g` fields in the earlier
`inertia_witnesses.json`, SHA-256
`63fe1628107f3f1a51938202bfe880c27bad253b3279df2ce2eea029a8c0028d`.
The README and original refutation are preserved unchanged. The exact
rational inertia counts remain the references; the binary64 observations
illustrate rounding hazards and are not product comparison targets.

The earlier generator evaluated the square in
`g = max(1, max|L|)² · max|d| / max|a_jj|` through floating-point power.
That operation uses the platform mathematics library and need not return
the correctly rounded square. The earlier package reproduced on the Mac
that froze it, but Linux x86-64 regeneration changed the first value below.
For the second value, both platforms' power results were one ULP below
the correctly rounded square. The original refutation's reproduction of
212 observations was made on one platform, not established across platforms.

Both square sites now use explicit multiplication. The remaining evaluation
order is unchanged: round the square, multiply by the maximum pivot and
round, then divide by the maximum diagonal and round. The correction is
grounded in exact rational arithmetic on the published binary64 operands,
not agreement with a product implementation.

Both changes are in `WIT-NEAR-PAIR`, under
`shifts[*].binary64_observation.unscaled.growth_g`:

| Shift (exact binary64 hex) | Earlier value | Corrected value | Change |
|---|---|---|---|
| `0x1.0000000000fffp+0` | `0.9999999470199651` | `0.9999999470199653` | +2 ULP, exactly 2⁻⁵² |
| `0x1.0000000001001p+0` | `0.9999999470199663` | `0.9999999470199664` | +1 ULP, exactly 2⁻⁵³ |

The exact squares of the respective published maxima of |L| are

- `45635421608382211596550441672729 / 5070602400912917605986812821504`,
  rounding to `0x1.20000000047f9p+3`;
- `45635421608382346704539263033369 / 5070602400912917605986812821504`,
  rounding to `0x1.2000000004808p+3`.

After the stated rounded multiplication, the exact quotients before final
rounding are respectively `1688849860266239 / 1688849949741568` and
`1688849860266241 / 1688849949741568`. They round to the corrected values
above. No exact count, pencil, shift, pivot, miscount observation, tolerance
or numerical threshold changes. Agent 0 authorized this frozen-reference
correction on these independent arithmetic grounds.
