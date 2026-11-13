# tools/validation

## Pressure elongation anchor

The regression anchor of the pressure-elongation switch (DEL-04-07): with `model_settings.pressure_elongation`
naming every pressurised load case, each committed pressure input must solve to the
same physical numbers as today, bit for bit.

- `pressure_elongation.py`: the switch every maintained Python producer takes,
  `--pressure-elongation {absent,all,none}` (default `$SWBPIPE_PRESSURE_ELONGATION`, else
  `absent`). `absent` and `none` write today's bytes; `all` names each document's
  pressurised cases in document order.
- `pressure_elongation_inventory.json`: every tracked file under `core`, `validation`,
  `tests`, `tools`, `fixtures` and `apps` whose bytes name `pressure_regions`, each claimed
  exactly once in `pressure_elongation_inventory.py`. A data file (`.json`, or `.jsonl` read
  line by line) holding a pressurised model document is claimed by a group in `GROUPS`:
  its producer, the tests that pin it and its anchor route. Documents held as JSON text in
  a string or in the retained-precision DAG storage count too, and are listed as
  `held_documents`, which the harness does not read. A code file that builds a solved
  pressurised document is claimed by an `IN_CODE` entry. The entry states what its
  assertions rest on (a closed form, a reference file, the product's own output or no
  response value) and whether its pressurised cases are named. Every other file is
  claimed by an `EXEMPT` entry with a one-line reason. An unclaimed file, a file claimed
  twice, a stale claim, an exemption without a reason, and an exemption of a data file
  with a pressurised document each fail. The inventory is also the checklist for
  re-producing the fixtures. Regenerate it with
  `python3 tools/validation/pressure_elongation_inventory.py`;
  `test_pressure_elongation_inventory.py` fails until it is current.
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
  - EXCLUDED, for an entry whose anchor is excluded: frozen persistence records, and files
    whose documents are held in JSON Lines, JSON text or DAG storage.

  The comparison exempts only the bytes the design changes:
  - the key `pressure_elongation`, at any depth;
  - `pressure-3` read as `pressure-2` in strings;
  - the added pressure-elongation statements in `limitations` lists.

  Every other number is compared as binary64 bits, and every other string exactly. Pass
  `--examples-dir` to use prebuilt examples; otherwise they are built with `$CARGO`
  (release).
