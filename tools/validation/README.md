# tools/validation

## Pressure elongation anchor

The regression anchor of the pressure-elongation switch (DEL-04-07
`Work/pressure-elongation.md` §7 item 1): with `model_settings.pressure_elongation`
naming every pressurised load case, each committed pressure input must solve to the
same physical numbers as today, bit for bit.

- `pressure_elongation.py`: the switch every maintained Python producer takes,
  `--pressure-elongation {absent,all,none}` (default `$SWBPIPE_PRESSURE_ELONGATION`, else
  `absent`). `absent` and `none` write today's bytes; `all` names each document's
  pressurised cases in document order.
- `pressure_elongation_inventory.json`: every tracked model input under a
  `pressure_contract` with a pressurised case, grouped by producer, with the tests that
  pin it and its anchor route. It is also the checklist for re-producing the fixtures.
  Regenerate with `python3 tools/validation/pressure_elongation_inventory.py`. A new
  pressure fixture must be classified in that script's `GROUPS`; the test fails until it is.
- `pressure_elongation_anchor.py`: the harness. It solves each document with the
  `core/product_physics` examples and compares the result with the committed envelope or
  with a captured baseline:

  ```
  python3 tools/validation/pressure_elongation_anchor.py check --switch absent    # self-test, green today
  python3 tools/validation/pressure_elongation_anchor.py capture --baseline DIR   # at a pre-change commit
  python3 tools/validation/pressure_elongation_anchor.py check --switch all --baseline DIR
  ```

  Each entry reports one of these statuses:
  - PASS;
  - FAIL, with the first differing JSON pointer;
  - REFUSED, when the engine does not accept the member yet (exit 3);
  - ERROR;
  - UNCOMPARED, when there is no committed envelope and no baseline;
  - EXCLUDED, for frozen persistence records.

  The comparison exempts only the bytes the design changes:
  - the key `pressure_elongation`, at any depth;
  - `pressure-3` read as `pressure-2` in strings;
  - the two §5 limitation statements in `limitations` lists.

  Every other number is compared as binary64 bits, and every other string exactly. Pass
  `--examples-dir` to use prebuilt examples; otherwise they are built with `$CARGO`
  (release).
