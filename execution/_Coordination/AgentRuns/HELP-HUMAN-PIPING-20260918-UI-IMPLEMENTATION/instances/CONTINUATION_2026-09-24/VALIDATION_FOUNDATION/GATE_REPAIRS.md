# Coherent gate-boundary repair

The independent full gate review found five actionable defects in the original
four-file candidate. Its FINDINGS, original hashes and synthetic false-pass
probes are retained in GATE_REVIEW. The original20 passing tests did not detect
these defects; they are not claimed as sufficient evidence for the repaired code.

- Actual controlled-output `BLOCKING` findings used uppercase severity. The gate
  now accepts only known severities, handles both actual producer casing forms,
  and checks wrapper blocking/finding/warning/decision counts for consistency.
- The original supervisor supplied a writable archive file as stdin and reported
  the original bytes even when that file was changed. It now delivers immutable
  bytes through a pipe while draining stdout/stderr concurrently, records the
  delivered prefix/count/hash, and separately checks retained archives. Input
  and output archive mutation is nonpassing. This proves delivery/capture, not
  the executable's internal use of input or its numerical correctness. Streaming
  bounded rehash also prevents a grown reference/input file from causing an
  unbounded read after execution.
- A nonzero lexical reference could underflow to zero and match an observed zero
  at zero tolerance. Strict numeric-token parsing now refuses this information
  loss as well as nonfinite values. This is new gate admission, not activation
  of the separate scientific JSON profile in any product producer.
- Extra raw-row fields could contradict the pinned semantic dimension. Current
  raw0.1 row fields and optional basis/metadata/source-reference shapes are closed;
  a caller cannot attach a conflicting dimension and have it ignored.
- The selector used an invented `load_combination` spelling. It now uses the
  actual producer vocabulary `load_case`/`combination`, with a valid combination
  control and rejection of the invented token.

The six added regression groups include these original triggers and simultaneous
large-input/output pipe delivery. Current actual check:26 unittest methods pass
with zero failures/skips in _run_records/gate_checks_04.log. These include
synthetic process execution and a preserved received legacy packet refusal;
no Piping solver, external deck, build, native or profile qualification occurred.
No criterion, legacy classifier, product schema or protected numerical threshold
was changed. GATE_FREEZE_02 binds the four repaired files and complete patch.
Independent repair backcheck remains its own evidence requirement.
