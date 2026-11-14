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
  re-producing the fixtures. A registered pin on a single file beyond its group's (`PINNED_BY`:
  a fixed table in a consumer that requires the file's current SHA-256, such as the O4 source
  twins' `PINS`) is listed on that file's entry or exemption record. For each table in its
  `PIN_TABLES`, the test extracts every target path by the table's own path-construction
  rules, however each digest is written and whether or not it still matches, refuses by name a
  path form it does not support, and requires every inventoried target to list the table. What the scan does not read (escaped or UTF-16 keys, source
  code, assembled blobs such as the MBF writer parity vectors' deltas) is stated in the script's
  docstring. Regenerate it with `python3 tools/validation/pressure_elongation_inventory.py`;
  `test_pressure_elongation_inventory.py` fails until it is current.

  The committed check establishes general structural properties: every scanned file is claimed
  exactly once, by a record or an exemption; no claim is stale; the declarations have their
  required shape; the generated copy equals what the producer writes; each inventoried file
  names its registered pin consumers (the tables in `PIN_TABLES`); and no code file naming the
  MBF parity vectors or their harvest modules, nor a Rust one's Cargo manifest, names a mechanics
  solve entry (a literal-name guard, not reachability). It also asserts particular data facts on
  the parsed fixtures: the K-D5 M3-T frozen subset's twelve pressurised documents, none naming
  the setting, and the locations, pressurised cases and naming of the five files whose documents
  are held only. It establishes nothing about source behaviour. The source classifications
  (`cases_named`, `expectations`, a group's producer) and the exemptions are curated assertions,
  supported by an independent reading at a6392aafb (its findings repaired in accdaf407), a second
  independent reading at 986096a03 that derived every in-code record and exempted code file from
  source, and a scratch probe at 4854700d0 that observed cases reaching pressure assembly (not
  successful solves) and whose observed sources are identical at 986096a03; the probe is not
  committed. Regenerating the copy does not renew that evidence, and a green check
  does not maintain their truth. Re-read an assertion when a change affects it: the case
  construction, the fixture inputs, the helpers, the callers, the naming or the dispatch of the
  file or of what it relies on, not only the setting's semantics. Before the inventory is used
  for a rebuild, reconcile the affected assertions with the source actually being used. These
  pass the check and are its limits: a flipped `cases_named`, after regeneration; a false
  exemption, after regeneration; an existing claimed source edited so that its classification
  becomes false; a pressurised solve added to an existing exempted file.
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
