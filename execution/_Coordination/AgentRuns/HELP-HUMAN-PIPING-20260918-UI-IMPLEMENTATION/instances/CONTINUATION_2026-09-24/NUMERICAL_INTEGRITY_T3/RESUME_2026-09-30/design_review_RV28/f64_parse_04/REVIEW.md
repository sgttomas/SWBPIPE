# RV28 — successful f64 parse source review

**VERIFIED.** On the pinned successful finite f64 conversion route,
**incremental registered heap scratch = 0 and growing-realloc active-old = 0**,
including the slow fallback. No blocking or SHOULD-FIX finding.

This component fact may feed the conditional VR caller assembly. It does not
erase input/caller owners or close final-build, panic/error, complete E_max,
checkpoint0, admission or measurement obligations.

TASK /root/rv28_a1_design, parent /root, native followup_task; no delegation.
Actual start **2026-10-01 20:11:16 UTC**, deadline 20:16:16 UTC.
The same active instruction/skill basis applies; no new workflow or library
programme was selected. Completion is in TIMING.json.

Reviewed packet:
R/I23/f64_parse_source_19, seal
`5b5f5384465b3cddae63602816eb82652aab25b2eb29e3d120c1ec2f1d75270c`.
All sealed payloads match. Fourteen installed Rust1.97.1 source HTML pages
were independently hash-checked, decoded and matched to the supplied full
decoded hashes; every cited excerpt line matched. Both product caller sources
and cited lines matched immutable
`40129a225d73860ac2a53da9a2fa73869df668f3`.
Origins and raw checks are under _run_records.

## Actual call chain and storage

VR/src/floor.rs:130–139 consumes borrowed kind/expected string slices and
parses expected at :135. VR/examples/vk_scale.rs:313–325 borrows the existing
s_full map, parses its &String value at :321, and accumulates into an inline
[f64;4]. The explicit f64 destination selects primitive f64 FromStr; String
autoderef supplies &str without making a new String.

The source chain is str::parse (:2765–2766) -> f64 FromStr
(float_parse.rs:68–78) -> dec2flt (dec2flt/mod.rs:211–255).
This conclusion follows from the actual code and calls, **not** the fact that
they reside in core.

- **Scanner:** parse_number/parse_partial_number and scientific parsing work on
  borrowed byte slices, scalar mantissa/exponents and inline Decimal.
  ByteSlice::read_u64 uses a local [u8;8]. No copied input or dynamic collection
  is introduced.
- **Fast conversion:** Decimal::try_fast_path uses primitive arithmetic and
  fixed constant integer/f64 tables. On aarch64 the selected set_precision
  function is empty and returns unit; no precision-guard owner is allocated.
  Skipping the fast path under optimize_for_size still reaches the bounded
  routes below.
- **Eisel–Lemire:** compute_float and full_multiplication use u64/u128 and
  inline BiasedFp. POWER_OF_FIVE_128 is a static 651-entry table. Indexing it
  does not lazily construct a heap object.
- **Slow fallback:** dec2flt/mod.rs:247–248 calls parse_long_mantissa on the
  original borrowed bytes. slow.rs:40 creates a local DecimalSeq returned
  by parse_decimal_seq. DecimalSeq has scalar metadata and
  digits:[u8;768] (decimal_seq.rs:18–33,60), not Vec, Box or a dynamically
  expanding big integer.
- **Fallback digit/shift work:** try_add_digit writes only within MAX_DIGITS;
  excess input adjusts count/truncation metadata, then the retained digit
  count is capped. left_shift/right_shift/round/trim mutate the same fixed
  buffer and use scalar indices. Shift helpers use fixed const tables
  [u16;65], [u8;0x051c] and [u8;19]; iterations borrow table slices.
  Longer mantissas do not trigger a reserve/reallocation branch.
- **Callbacks:** the inspected ByteSlice generic FnMut is called directly.
  Its actual closures update local scalars or the local DecimalSeq; get_shift
  indexes its fixed table. None is boxed, stored in a heap owner, or supplied
  by a caller-defined hook through f64 FromStr.
- **Return:** biased_fp_to_float assembles integer bits. The f64 specialization
  calls f64::from_bits, whose body is primitive transmute. Result and
  ParseFloatError are inline values. No registered temporary is retained or
  freed by successful conversion.

Therefore there is no incremental registered allocation or reallocation in
the inspected successful route, regardless of whether a fixed string takes
the fast path, Eisel–Lemire, or fallback. No assumption that all roster strings
are fast was used. This says nothing about exact machine stack-frame size,
compiler-generated stack placement, OS memory or parser numerical correctness.

## Finite-input and caller limits

The zero-heap warrant is conditional on the successful finite route; it is not
a new finiteness proof. The separately reviewed
R/source_review_RV30/finite_callers_12 supplies fixed reference-decimal and
12 s_full finiteness/source binding. Its seal and payloads were checked
unchanged and its scope was retained. This review did not rerun any decimal
parser, source model, numerical predicate, floor or solver.

A syntactically accepted string can still parse to infinity; **Ok alone is not
the finite premise**. The earlier exact fixed-input magnitude/grammar proof,
unchanged raw input/source identity and final correspondence remain necessary
for the actual caller claim. This review does not generalize the roster to
arbitrary future strings.

floor.rs's expect failure would enter caller panic behavior; this warrant
does not price it. vk_scale's .ok()? error exit is also not a successful
finite value. The presence of an inline ParseFloatError does not establish
zero heap for arbitrary caller formatting, panicking or unwinding.

All original reference/s_full String backing, maps, Case/model structures,
caller iteration/result arrays and surrounding owner unions stay as previously
accounted. Parsing borrows the strings; it does not free their storage.
This primitive conversion fact is separate from serde parsing and wrapped
writer/error ownership.

Installed-source identities establish the source consequence for the named
Rust1.97.1/aarch64 route. Actual final VR source/std/compiler/target/features,
allocator and binary correspondence remain a transfer obligation; the review
does not authenticate future machine code merely from current documentation.

## Evidence and execution

One bounded metadata/source check returned exit0, empty stderr and
**1,209 passing hash/excerpt checks** over the 14 supplied source pages and
two actual caller sites. The count includes individual line-equality checks,
not parser runs or numerical experiments.

    <VENV>/bin/python -B <OUT>/_run_records/check.py <K6C> <OUT>

Raw stdout, Git argv/statuses, source origins/hashes and seal checks are in
_run_records. The one-off HTML decoding verified supplied source evidence;
it did not execute the f64 parser or open a new library research programme.

No parser/runtime/executable/build/probe/model/solver execution, maintained
edit, Git/index mutation, installation or delegation occurred. Only
R/design_review_RV28/f64_parse_04 was written. Previous seals remain unchanged.

