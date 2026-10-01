# RV28 — expected-list parser scratch backcheck

**VERIFIED.** For the exact fixed expected_unresolved input and pinned successful
serde StrRead route, the additional parser scratch bounds are:

- Retained during parsing: **80 requested bytes**.
- Scratch construction old+new upper: **120 bytes**.
- Maximum growing-realloc active-old increment: **40 bytes**.
- Scratch after from_str returns: **0 bytes**.

No blocking or SHOULD-FIX finding. This is a component fact for conditional VR
assembly, not complete E_max, admission, runtime, final-build or error-path
acceptance.

TASK /root/rv28_a1_design, direct parent /root; native followup_task,
no delegation. Actual start **2026-10-01 20:22:08 UTC**; five-minute deadline
20:27:08 UTC. Completion is in TIMING.json. Same instruction/skill basis;
no new workflow or library programme. Raw provenance is under _run_records.

## Bound input and source

Reviewed I23 packet: R/I23/expected_list_parse_20, seal
`5d256c1dbbc107b2024eb9bd6e7a86a8649066905a2d42e59fc20bb1a0dfe6bf`.
Every payload is unchanged.

The actual input is VR/cases/expected_unresolved.json at immutable
`40129a225d73860ac2a53da9a2fa73869df668f3`, **1,357 bytes**, SHA256
`2e5d0975a90c69e5d198d61760cc7734d42615461710da1fe226f60b4a46f2c9`.
The supplied raw file matches that Git object byte-for-byte. Fixed-byte slicing,
not a JSON parser, independently confirms:

- The only backslashes are at zero-based offsets 1300 and 1351.
- Both are quote escapes in the final source string.
- The encoded payload is 73 bytes: segments 20,49,0 around two two-byte escapes.
- Replacing those two escapes with quotes gives 71 bytes.
- All actual scalar content is quoted; no JSON numeric-token scratch route is
  reached by these fixed bytes.

Eight bound source files were hash-checked: the three previously authenticated
serde pages, three reviewed Vec/RawVec pages, and the two caller files.
The caller archive bytes were also matched to immutable40129. Reuse of the
existing package/growth authentication is within its scope; no new package or
general library survey was performed.

## Actual scratch operations and growth

serde_json de.rs initializes scratch with Vec::new (:59–68) and clears its
length before each string/key (:1425–1430,2219–2223). Clearing does not shrink.
Before the final source value all strings are unescaped, so read.rs:513–519
returns borrowed slices and scratch has never allocated.

For that value, parse_str_bytes extends the preceding segment at each
backslash (:526–530); parse_escape's actual quote arm pushes one byte
(:879–883). At the closing quote, nonempty scratch gets the final suffix and
returns Reference::Copied (:520–523). The exact operations are therefore:

| Operation | Length after | Capacity after | Growing old |
|---|---:|---:|---:|
| Extend 20-byte prefix | 20 | 20 | 0 |
| Push decoded quote | 21 | 40 | 20 |
| Extend 49-byte middle | 70 | 80 | 40 |
| Push decoded quote | 71 | 80 | 0 |
| Extend empty suffix | 71 | 80 | 0 |

For u8, the reviewed RawVec minimum is 8 and a growth requests
max(2*old_capacity,required_length,8). extend_from_slice on the actual
TrivialClone byte slice reaches append_elements and reserve(count), not a
per-byte assumed push sequence. The first request is consequently 20 rather
than an invented next-power-of-two32. The subsequent requests are40 and80.
All integer arithmetic is small and checked independently.

Only one scratch buffer is growing at a time. The largest scratch old+new
upper is40+80=120; its retained request is80 and active-old increment40.
The final quote and empty suffix allocate nothing further.
Neither Unicode handling nor numeric-token scratch is reached.

These are requested-capacity/source-metric results, not allocator size classes,
observed addresses or a measured peak.

## Copy and lifetime accounting

de.rs:1430 calls ValueVisitor::visit_str with the scratch slice still borrowed.
value/de.rs:75–85 makes String::from(value), then owns that String in Value.
The separate71-byte backing therefore overlaps the80-byte scratch:
151 bytes for those two particular portions. The71 bytes already belong to
the Value-tree accounting and must not be added again on top of a complete
Jtree bound.

During the middle-segment growth, old40 and new80 scratch coexist under the
growing-move upper; the final71-byte Value String has not yet been copied.
Adding120 scratch to an already valid parse-construction union is conservative.
It does not justify adding another80 for the same scratch owner or claiming
every independently padded phase maximum occurs simultaneously.

serde_json from_trait owns the Deserializer locally (:2507–2518), checks
trailing input, and drops it before returning the owned Value. Its Vec scratch
cannot escape through Value's owned string. Thus when cases.rs:316 completes,
scratch is gone.

The subsequent cases.rs:317–322 typed collection borrows the completed Value.
It overlaps raw text and Value, but **not** parser scratch. On closure return,
raw text and Value drop; the typed LIST is retained by OnceLock. The carried
reviewed retained LIST amount remains2*24+15+15=78 bytes on the named type basis.
The two case-name byte lengths and arithmetic were rechecked; no new model or
typed constructor ran. The static OnceLock wrapper is not an extra heap child.

lane.rs:250 invokes expected_unresolved while the surrounding lane state and
outcome/caller owners can remain live. Nothing in this scratch calculation
removes those caller/kernel owners or changes the expected-outcome semantics.

## Composition and limits

Keep the raw read buffer, Jtree and its construction/node/string owners,
typed list construction, caller/kernel state, and other initialization helpers
separately accounted at their actual phases. **1,357 file bytes is content
length, not an inferred read-buffer capacity.** This result closes only the
specific parser scratch term.

The warrant requires the same exact input bytes and pinned serde features,
Vec/RawVec implementation and successful fixed parse. Malformed input,
allocation failure, panic/error formatting, arbitrary escaped JSON and changed
input remain outside it. Final source/feature/compiler/allocator/type/binary
correspondence and complete-envelope integration remain separate.

## Execution

One bounded source/hash/fixed-byte/integer check returned exit0, empty stderr
and **33 passing checks**. The command was:

    <VENV>/bin/python -B <OUT>/_run_records/check.py <K6C> <OUT>

Raw argv/source identities, byte offsets, segment lengths and full capacity
trace are retained under _run_records. JSON was read only as evidence metadata;
the actual expected-list input was not run through a parser.

An initial lookup used the nonexistent generic WARRANT.json name; the packet
inventory identified SCRATCH_WARRANT.json, which was then read in full.
That read-only filename mistake produced no numerical or source mutation.

No parser/runtime/build/probe/model/solver execution, library programme,
maintained edit, Git/index mutation or delegation occurred. Only this additive
review subtree was written; prior seals remain preserved.

