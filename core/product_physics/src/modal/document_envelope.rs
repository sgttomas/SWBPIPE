//! The modal document envelope (DEL-04-09 REQ-007, "Before document
//! construction, canonicalization, reading or delivery"; review finding F4),
//! landing A1: the envelope's derivation from the document's own structure,
//! and a tally that computes it for an actual typed outcome and a borrowed
//! request **without allocating** and without constructing any
//! `serde_json::Value`.
//!
//! A1 enforces nothing. No builder, document byte, diagnostic or code
//! changes; no cap, limit or default exists here. A2 compares an envelope
//! with caller-supplied limits before any document value is built; A3 owns
//! headless publication and the exact delivery count.
//!
//! # What is counted and what is not
//!
//! Counted: the document `modal/document.rs` would build for the same inputs
//! (every branch of `modal_result_document`, `document.rs:505-540`, and
//! `modal_doc_solved`, `:553-898`), and the storage those builders hold
//! while they build and hash it, the checked re-parse inside every hash
//! included (REQ-007's ingress rule leaves that re-parse to the document
//! rule). Excluded, as bounded elsewhere: the typed outcome; the parsed
//! request; the ingress copy `request.clone()` (`:507`) and every ingress
//! operation on the encoded input (REQ-007's ingress rule, its own
//! allocator exception); the caller's borrowed request; the caller's
//! `ModalIntegrityParameters` and `Diagnostic`s. Also excluded: allocator
//! rounding and metadata, process resident size, stack depth and time. Every
//! byte figure is logical requested bytes. The derived peaks serve REQ-007's
//! document-envelope rule, one of its two allocator exceptions.
//!
//! # The tally ([`ModalDocumentTally`])
//!
//! For one JSON value: `wire`, its serde_json compact length; `float_bytes`
//! and `floats`, the part of `wire` written by binary64 numbers and their
//! count; `nodes` (every value); `objects` and `entries` (members);
//! `key_bytes` (raw UTF-8 bytes of member names); `arrays` and `elements`;
//! `strings` and `string_bytes` (string values and their raw UTF-8 bytes;
//! names are not strings here); `depth` (container nesting: a scalar 0, a
//! container one more than its deepest child); `longest_literal` (the
//! longest compact text of one scalar or one quoted name);
//! `unsafe_numbers` (numbers the checked canonical profile refuses);
//! `nonempty_containers`, `container_levels` and `slot_levels` (for the
//! pretty length, below). All are exact for the document the builders build;
//! the tests hold each to an independent walk of the built value.
//!
//! ## Leaves (exact)
//!
//! - `null` 4 bytes, `true` 4, `false` 5.
//! - Integers: serde_json writes u64/i64 through itoa (`serde_json` 1.0.149
//!   `number.rs:354-355`, the same writer the serializer uses): the decimal
//!   digits, with `-` for a negative i64.
//! - Binary64: `serde_json::to_value` of an f64 is `Value::from(f64)`, null
//!   when not finite (`value/from.rs:47`); a finite one is written by
//!   `zmij::Buffer::format_finite` (`ser.rs:1716-1723`). Its length is taken
//!   exactly by counting `Number`'s `Display` (`number.rs:356`, the same
//!   `format_finite`) through `modal_eigen::modal_quantities::formatted_length`,
//!   which stores nothing. At most 24 bytes: zmij writes into a 24-byte stack
//!   buffer (`zmij` 1.0.21 `lib.rs:90`, `BUFFER_SIZE`).
//! - Strings and names: `"` + content + `"`, the content escaped by
//!   serde_json's table (`ser.rs:2135-2165`, `format_escaped_str_contents`
//!   `:2079`): `"` and `\` 2 bytes; the controls 0x08, 0x09, 0x0A, 0x0C,
//!   0x0D 2 bytes; every other byte below 0x20 6 bytes (`\u00XX`); every other
//!   byte, 0x7F and multi-byte UTF-8 included, 1 byte. Escaping is per byte,
//!   so a concatenation's escaped length is the sum of its parts'
//!   ([`ModalEnvelopeText`]).
//!
//! ## Composition
//!
//! An object of m members, with escaped name lengths K_i and child tallies
//! V_i, and an array of n elements:
//!
//! ```text
//! wire(object) = 2 + max(m − 1, 0) + Σ (K_i + 2 + 1 + wire(V_i))     // quotes, colon
//! wire(array)  = 2 + max(n − 1, 0) + Σ wire(V_i)
//! nodes = 1 + Σ nodes(V_i);  depth = 1 + max depth(V_i) (1 when empty)
//! entries(object) = m + Σ entries(V_i);  elements(array) = n + Σ elements(V_i)
//! ```
//!
//! and every other count is the sum (longest literal the maximum) over the
//! children, the names included. Every sum and product is checked in u64;
//! overflow is [`ModalEnvelopeError::Overflow`], never wrapped or saturated.
//!
//! ## Whether a hash is a string or null
//!
//! `modal_doc_sha256` (`document.rs:218-222`) serializes, re-parses with the
//! checked profile and canonicalizes. Serializing a `Value` cannot fail. The
//! parse refuses exactly a container nested 128 deep (serde_json
//! `de.rs:63`, `remaining_depth: 128`, and `check_recursion!`
//! `:1372-1387`), i.e. `depth ≥ 128`; a `Map` holds each name once, so the
//! duplicate-member refusal (`canonical_json/src/lib.rs:78-81`) cannot
//! arise; serialization writes only finite numbers, so the non-finite one
//! cannot either. Validation (`lib.rs:94-116`) then refuses an i64 outside
//! ±(2⁵³ − 1), a u64 above it, or an integral f64 of magnitude above it. So a
//! hash is the 64-hex string (66 compact bytes) iff `depth < 128` and
//! `unsafe_numbers = 0`, and null otherwise. A row's checksum, a table's
//! digest (an array of strings and nulls: always a string) and every binding
//! hash follow this rule; the identity's run hash and the semantic table's
//! hash are always strings (their inputs hold strings or nulls only).
//!
//! # The mirror
//!
//! Each builder has its mirror, which walks the same inputs in the same
//! branches and returns the tally of what it would build: the quantity object
//! (`:235-270`, [`modal_env_quantity`], with the undefined variants
//! `not_formed`, `not_finite`, `outside_canonical_profile` and the caller's
//! reason); the row seal (`:273-283`) and table (`:286-289`); the diagnostic
//! row (`:352-359`); the parameters (`:362-383`); the binding
//! (`:387-416`); the subject census (`:423-436`); the identity (`:468-491`)
//! with the semantic table (`:204-214`); the refused document (`:493-499`);
//! the header and the fifteen tables of the solved document (`:553-898`).
//! The document's private tables (`MODAL_KINDS`, the option members, the
//! components, the directions and the profile limit) are restated here; the
//! tests hold them to the built documents and to `modal_semantic_table()`.
//! The two diagnostics the builder makes itself (parse `:511`, skew `:527`)
//! are taken whole from the caller, so no code or message is restated (and
//! the diagnostic catalog's scan finds no new producer here). Where a
//! builder would index out of range or panic (a node index past `node_ids`,
//! an axis past 2, a mode index 0, an unknown unit rule), the mirror returns
//! [`ModalEnvelopeError::Unprojectable`]: no document exists to measure.
//!
//! A borrowed request is walked whole (its hash is taken), and the walk
//! never descends into a container nested deeper than the caller's cap: it
//! returns [`ModalEnvelopeError::DepthCap`] instead.
//!
//! # Derived storage
//!
//! With V = `size_of::<Value>()` = 32 and N = 736 (a std `BTreeMap<String,
//! Value>` node, below):
//!
//! **H(T), the logical heap of a built tree** ([`modal_env_heap`]):
//!
//! ```text
//! H = 2(string_bytes + key_bytes) + 8(strings + entries)
//!   + V·(2·elements + 4·arrays) + N·(objects + ⌈entries/5⌉)
//! ```
//!
//! Strings: every `String` in these trees is either an exact copy (`to_owned`
//! in serde_json's value serializer, `value/ser.rs:168, 558`; `String::clone`)
//! with capacity = length, or a `format!` result (`document.rs:292-309, 487`,
//! row ids) with capacity ≤ max(2L, 8) (premise P3), so capacity ≤ 2L + 8.
//! Arrays: a `Vec<Value>` from `to_value` or `vec!` has capacity n
//! (`value/ser.rs:233-236`); one grown by `push` or `collect` from a filter
//! has capacity ≤ max(4, 2n) (premise P2): so ≤ 2n + 4 elements of V bytes.
//! Numbers, booleans and null live inside their 32-byte `Value`. Maps: a
//! non-empty map with e entries has at most ⌈e/5⌉ nodes, since its root has
//! at least one key and every other node at least 5 (premise P1), and
//! Σ⌈e_i/5⌉ ≤ objects + ⌈entries/5⌉; an empty map allocates nothing. A node
//! is at most 736 bytes: the leaf's parent pointer, index and length each
//! padded to 8, then eleven 24-byte names and eleven 32-byte values (640),
//! and an internal node's twelve 8-byte edges (+ 96), whatever the field
//! order (the formula of `retained_memory.rs` `btree_node_upper`, BUILD.md
//! §4, at K = `String`, V = `Value`; not called from here). H of a summed tally
//! bounds the summed heaps of the trees it sums.
//!
//! **HP(T), the peak while one value with tally T is hashed**, excluding the
//! value itself ([`modal_env_hash_peak`]); W_c = `wire`, W_k = the canonical
//! text bound below, LL = `longest_literal`, e = `entries`:
//!
//! ```text
//! HP = (128 + 3·W_c)              serialized text: Vec::with_capacity(128) then doubling,
//!                                 old + new at a reallocation (ser.rs:2217; P2)
//!    + (H(T) + V·elements)        the parse's rebuilt value, one array mid-reallocation
//!    + (key_bytes + 86e + 212·depth)  duplicate-name sets of the objects being parsed:
//!                                 cloned names, and per set of k names ≤ 86k + 212 (P4)
//!    + (3·LL + 8)                 the parser's escape scratch (P2)
//!    + (8 + 3·W_k)                the canonical text, String::new then doubling (P2)
//!    + 16e                        sorted name lists on the write path (8 per name) and
//!                                 one sort's scratch (≤ 8 per name, P5)
//!    + (128 + 3·LL)               write_string's to_string of one literal (lib.rs:335-337)
//!    + 256 + 512 + 384            number text (lib.rs:400-498), a refusal's message and
//!                                 boxed error, the hex digest's format!/collect (document.rs:221)
//! ```
//!
//! **The canonical text** ([`modal_env_canonical_bound`]) differs from the
//! compact text only in floats (names sorted, same escaping through
//! `serde_json::to_string`, integers by `to_string`): canonical =
//! `wire − float_bytes + Σ ecma(f)` ≤ `wire − float_bytes + 25·floats`.
//! 25 is the largest branch of `ecma_number_to_string` (`lib.rs:444-468`) for
//! at most 17 shortest digits and decimal exponents −324..308 (premise P6):
//! sign + `0.` + five zeros + 17 digits.
//!
//! **The pretty length** ([`modal_env_pretty`]): serde_json's pretty
//! formatter adds, to the compact text, for each non-empty container at
//! level l a newline after its opener and a newline and 2l spaces before its
//! closer, for each element or member at level l a newline and 2l spaces, and
//! one space after each name's colon. With `container_levels` and
//! `slot_levels` the summed relative levels, at root level 0:
//! `P = wire + nonempty + 2·container_levels + slots + 2·slot_levels +
//! entries`, slots = entries + elements. This is exact (tested); A3's
//! non-storing count stays the delivery authority.
//!
//! **C, the construction peak** ([`ModalDocumentEnvelope`]):
//!
//! ```text
//! C = D + H(binding) + H(document) + H(rows) + V·(3·rows + 4·tables) + 16·g
//!   + max(X_bind, X_census, X_identity, 2·H(header), X_row, X_table) + 2048
//! ```
//!
//! - D: a diagnostic the builder itself creates (parse `:511`, skew `:527`,
//!   A2's fallback): H of its row fields (which covers its strings and refs)
//!   + 2L + 8 for its `source` string (`lib.rs:16088`; L ≤ 64 for A2's).
//! - H(binding): `modal_result_document`'s own binding (`:510, 518, 522`),
//!   held while the document is built; the document holds a second copy
//!   (`binding.clone()`, `:489`), inside H(document).
//! - H(rows): every sealed row original, until its table copies it
//!   (`to_value(&rows)` in `json!`, `:288`): the mode, shape and participation
//!   originals are all live at once (`:728-778`), so all are counted. Their
//!   vectors: push-grown (`:728-730`) or collected, ≤ 3n + 4 elements each
//!   counting a reallocation (P2).
//! - 16·g: `free_index`, `vec![None; 6·nodes]` of `Option<usize>` (`:556`).
//! - X_bind (`:387-416`): the options map (clones of the option values) +
//!   the undeclared names' `Vec<&str>` (16·(3u + 4)) + the materials clone +
//!   the state object + 2·H(parameters) (`json!` copies the parameter
//!   vectors) + the largest HP of the request, model, materials, load case and
//!   parameters, all live while `json!` builds the binding.
//! - X_census (`:423-436`): the subjects `Vec<(&str, &str)>` (32 per subject)
//!   + the list value + HP(list) + 512.
//! - X_identity (`:468-491`): 2·H(semantic table) (its kinds array and
//!   `json!`'s copy) + HP(semantic table) + HP(run) + 1024 (the run object's
//!   copied hashes and the units' `collect` vectors).
//! - 2·H(header): a header member's locals and `json!`'s leaf copies, one
//!   member at a time (`:561-678`).
//! - X_row: per row, the `format!` temporaries, `json!`'s leaf copies and the
//!   row clone taken before its checksum (`:280`), each ≤ H(row), + HP(row) +
//!   512 (the checksum's own string); for the componentwise largest row.
//! - X_table: the checksum array and its hash (`:287-288`), for the
//!   componentwise largest checksum array.
//! - 2048: the options' `collect` vector (≤ 7 × 56 B × 3), the
//!   `binding["subject_census"]` name (`IndexMut` makes an owned key,
//!   serde_json `value/index.rs:101`) and the run id's `format!`.
//!
//! The transients are mutually exclusive in time; their maximum is added to
//! what stays live. Every term is ≥ what it bounds, so C ≥ the builders' peak
//! under the premises.
//!
//! # Premises
//!
//! Proved from source: everything above that cites a line. Asserted (a
//! toolchain or feature change fails a build or a test):
//! `size_of::<Value>() == 32`, `size_of::<String>() == 24`,
//! `size_of::<Number>() == 16` (no `arbitrary_precision`),
//! `size_of::<Option<usize>>() == 16` and a 64-bit `usize` (compile time,
//! below); `Map` iterates in sorted order (no `preserve_order`) and P2, P3
//! and P4's growth (tests). Named, not assertable without an allocation
//! observer:
//!
//! - **P1** std's `BTreeMap` has B = 6 (eleven keys a node, every non-root
//!   node at least five after any insertion, clone or bulk build) and its
//!   nodes are at most 736 bytes for 24-byte names and 32-byte values.
//! - **P2** `Vec` growth: capacity at least doubles when full, from 4 (8 for
//!   bytes), so a pushed vector of n has capacity ≤ max(4, 2n), and old and
//!   new buffers coexist only during one reallocation. Asserted for the
//!   capacities a test can see; the coexistence is the premise.
//! - **P3** `format!` (`alloc::fmt::format`) starts from
//!   `Arguments::estimated_capacity` (at most twice its literal pieces) and
//!   grows by P2, so capacity ≤ max(2L, 8). Asserted for this module's id
//!   formats.
//! - **P4** std's `HashSet<String>` (hashbrown): buckets a power of two with
//!   capacity 7/8 of them (3 below 8), growth to the next size when full,
//!   25 bytes a bucket plus ≤ 31 of control and alignment, old and new tables
//!   coexisting during a resize: a set of k names ≤ 1.5·(25·max(4, 16k/7))
//!   + 62 ≤ 86k + 212. The capacity growth is asserted; the layout is the
//!   premise.
//! - **P5** `slice::sort_by` allocates at most one scratch of the slice's
//!   length; `sort_unstable` allocates nothing.
//! - **P6** ryu (the canonical digit source) yields at most 17 digits.
//!
//! # Interface
//!
//! [`modal_env_parse_refused`], [`modal_env_run_refused`],
//! [`modal_env_skew_refused`] and [`modal_env_solved`] give the envelope of
//! the document `modal_result_document` builds on each branch;
//! [`modal_env_solved_with_binding`] that of `modal_doc_solved` for a given
//! binding; [`modal_env_fallback_refused`] that of a refused document with
//! one diagnostic within a [`ModalEnvelopeDiagnosticBound`], from the request
//! alone (A2 checks it before the solve, so that a named refusal after the
//! solve is known to fit). [`modal_env_value`] tallies any borrowed value.
use super::document::{
    ModalParameterStanding, MODAL_RESULT_CANONICAL, MODAL_RESULT_KIND, MODAL_RESULT_LIMITATIONS, MODAL_RESULT_PROFILE, MODAL_RESULT_SEMANTICS,
    MODAL_RESULT_VERSION, MODAL_SELECTED_PARAMETER_LABEL,
};
use super::eigen::{FiniteCensus, ReportedEndpoint};
use super::integrity::ModalIntegrityParameters;
use super::outcome::{ModalOutcome, ModalStanding};
use super::state::ModalSupportTreatment;
use crate::Diagnostic;
use open_pipe_stress_canonical_json::MAX_SAFE_INTEGER;
use open_pipe_stress_modal_eigen::clusters::{ExtractionBound, StoppingBound};
use open_pipe_stress_modal_eigen::modal_quantities::{formatted_length, frequency_from_eigenvalue, NormalizationCase};
use open_pipe_stress_modal_eigen::sturm::{Placement, ShiftDecision};
use open_pipe_stress_units::UnitId;
use serde_json::{Number, Value};

// Asserted premises (module docs, "Premises").
const _: () = assert!(std::mem::size_of::<Value>() == 32);
const _: () = assert!(std::mem::size_of::<String>() == 24);
const _: () = assert!(std::mem::size_of::<Number>() == 16);
const _: () = assert!(std::mem::size_of::<Option<usize>>() == 16);
const _: () = assert!(std::mem::size_of::<usize>() == 8);

/// V: one `Value`, inline in its parent's buffer or node.
pub(crate) const MODAL_ENV_VALUE_BYTES: u64 = 32;
/// N: one `BTreeMap<String, Value>` node, at most (P1).
pub(crate) const MODAL_ENV_MAP_NODE_BYTES: u64 = 736;
/// The fewest keys of a non-root map node (P1).
const MODAL_ENV_MAP_NODE_MIN_KEYS: u64 = 5;
/// serde_json's parse refuses a container nested this deep (`de.rs:63`).
const MODAL_ENV_PARSE_DEPTH: u64 = 128;
/// The checked profile's integer limit (`canonical_json/src/lib.rs:34`).
const MODAL_ENV_SAFE: u64 = MAX_SAFE_INTEGER as u64;
/// 2⁵³: `document.rs:77`, where a quantity becomes `outside_canonical_profile`.
const MODAL_ENV_PROFILE_LIMIT: f64 = 9_007_199_254_740_992.0;
/// The largest canonical text of a finite binary64 (module docs, P6).
pub(crate) const MODAL_ENV_CANONICAL_FLOAT_MAX: u64 = 25;
/// The largest compact text of a finite binary64 (zmij's `BUFFER_SIZE`).
pub(crate) const MODAL_ENV_WIRE_FLOAT_MAX: u64 = 24;
/// A SHA-256 in lowercase hex.
const MODAL_ENV_HEX: u64 = 64;

/// The counts of one JSON value (module docs, "The tally").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ModalDocumentTally {
    pub(crate) wire: u64,
    pub(crate) float_bytes: u64,
    pub(crate) floats: u64,
    pub(crate) nodes: u64,
    pub(crate) objects: u64,
    pub(crate) entries: u64,
    pub(crate) key_bytes: u64,
    pub(crate) arrays: u64,
    pub(crate) elements: u64,
    pub(crate) strings: u64,
    pub(crate) string_bytes: u64,
    pub(crate) depth: u64,
    pub(crate) longest_literal: u64,
    pub(crate) unsafe_numbers: u64,
    pub(crate) nonempty_containers: u64,
    pub(crate) container_levels: u64,
    pub(crate) slot_levels: u64,
}

const MODAL_ENV_ZERO: ModalDocumentTally = ModalDocumentTally {
    wire: 0,
    float_bytes: 0,
    floats: 0,
    nodes: 0,
    objects: 0,
    entries: 0,
    key_bytes: 0,
    arrays: 0,
    elements: 0,
    strings: 0,
    string_bytes: 0,
    depth: 0,
    longest_literal: 0,
    unsafe_numbers: 0,
    nonempty_containers: 0,
    container_levels: 0,
    slot_levels: 0,
};

/// Why no envelope is given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModalEnvelopeError {
    /// A checked sum or product of `term` left u64 (limit `u64::MAX`).
    Overflow { term: &'static str, left: u64, right: u64 },
    /// The borrowed value holds a container nested deeper than `cap`; the
    /// walk stopped there.
    DepthCap { cap: u64 },
    /// The builders would panic on this input (`what`): no document exists.
    Unprojectable { what: &'static str },
}

pub(crate) type ModalEnvelopeResult<T> = Result<T, ModalEnvelopeError>;

fn modal_env_add(term: &'static str, left: u64, right: u64) -> ModalEnvelopeResult<u64> {
    left.checked_add(right).ok_or(ModalEnvelopeError::Overflow { term, left, right })
}

fn modal_env_mul(term: &'static str, left: u64, right: u64) -> ModalEnvelopeResult<u64> {
    left.checked_mul(right).ok_or(ModalEnvelopeError::Overflow { term, left, right })
}

/// A length or index as u64 (lossless: `usize` is 64 bits, asserted above).
fn modal_env_count(n: usize) -> u64 {
    n as u64
}

/// The decimal digits of `v` (1 for 0).
fn modal_env_digits(mut v: u64) -> u64 {
    let mut digits = 1;
    while v >= 10 {
        v /= 10;
        digits += 1;
    }
    digits
}

fn modal_env_digits_wide(mut v: u128) -> u64 {
    let mut digits = 1;
    while v >= 10 {
        v /= 10;
        digits += 1;
    }
    digits
}

/// The componentwise maximum (a tally ≥ each, for monotone bounds).
pub(crate) fn modal_env_max(a: &ModalDocumentTally, b: &ModalDocumentTally) -> ModalDocumentTally {
    ModalDocumentTally {
        wire: a.wire.max(b.wire),
        float_bytes: a.float_bytes.max(b.float_bytes),
        floats: a.floats.max(b.floats),
        nodes: a.nodes.max(b.nodes),
        objects: a.objects.max(b.objects),
        entries: a.entries.max(b.entries),
        key_bytes: a.key_bytes.max(b.key_bytes),
        arrays: a.arrays.max(b.arrays),
        elements: a.elements.max(b.elements),
        strings: a.strings.max(b.strings),
        string_bytes: a.string_bytes.max(b.string_bytes),
        depth: a.depth.max(b.depth),
        longest_literal: a.longest_literal.max(b.longest_literal),
        unsafe_numbers: a.unsafe_numbers.max(b.unsafe_numbers),
        nonempty_containers: a.nonempty_containers.max(b.nonempty_containers),
        container_levels: a.container_levels.max(b.container_levels),
        slot_levels: a.slot_levels.max(b.slot_levels),
    }
}

/// The tally of a forest of two values (counts added; depth and longest
/// literal the maximum).
pub(crate) fn modal_env_sum(a: &ModalDocumentTally, b: &ModalDocumentTally) -> ModalEnvelopeResult<ModalDocumentTally> {
    let add = modal_env_add;
    Ok(ModalDocumentTally {
        wire: add("forest wire", a.wire, b.wire)?,
        float_bytes: add("forest float bytes", a.float_bytes, b.float_bytes)?,
        floats: add("forest floats", a.floats, b.floats)?,
        nodes: add("forest nodes", a.nodes, b.nodes)?,
        objects: add("forest objects", a.objects, b.objects)?,
        entries: add("forest entries", a.entries, b.entries)?,
        key_bytes: add("forest key bytes", a.key_bytes, b.key_bytes)?,
        arrays: add("forest arrays", a.arrays, b.arrays)?,
        elements: add("forest elements", a.elements, b.elements)?,
        strings: add("forest strings", a.strings, b.strings)?,
        string_bytes: add("forest string bytes", a.string_bytes, b.string_bytes)?,
        depth: a.depth.max(b.depth),
        longest_literal: a.longest_literal.max(b.longest_literal),
        unsafe_numbers: add("forest unsafe numbers", a.unsafe_numbers, b.unsafe_numbers)?,
        nonempty_containers: add("forest containers", a.nonempty_containers, b.nonempty_containers)?,
        container_levels: add("forest container levels", a.container_levels, b.container_levels)?,
        slot_levels: add("forest slot levels", a.slot_levels, b.slot_levels)?,
    })
}

// ---------------------------------------------------------------------------
// Leaves.

fn modal_env_scalar(wire: u64) -> ModalDocumentTally {
    ModalDocumentTally { wire, nodes: 1, longest_literal: wire, ..MODAL_ENV_ZERO }
}

const MODAL_ENV_NULL: ModalDocumentTally = ModalDocumentTally { wire: 4, nodes: 1, longest_literal: 4, ..MODAL_ENV_ZERO };

fn modal_env_bool(value: bool) -> ModalDocumentTally {
    modal_env_scalar(if value { 4 } else { 5 })
}

/// A u64 number (`json!` of a `usize` or `u64`).
fn modal_env_unsigned(v: u64) -> ModalDocumentTally {
    let mut t = modal_env_scalar(modal_env_digits(v));
    t.unsafe_numbers = u64::from(v > MODAL_ENV_SAFE);
    t
}

fn modal_env_index(v: usize) -> ModalDocumentTally {
    modal_env_unsigned(modal_env_count(v))
}

/// `json!(v)` of an f64: null when not finite, else a number whose compact
/// text is zmij's, measured without storing it (module docs, "Leaves").
fn modal_env_float(v: f64) -> ModalEnvelopeResult<ModalDocumentTally> {
    let Some(number) = Number::from_f64(v) else {
        return Ok(MODAL_ENV_NULL);
    };
    let length = formatted_length(format_args!("{number}")).ok_or(ModalEnvelopeError::Unprojectable { what: "number text" })?;
    let length = modal_env_count(length);
    let mut t = modal_env_scalar(length);
    t.floats = 1;
    t.float_bytes = length;
    t.unsafe_numbers = u64::from(v.fract() == 0.0 && v.abs() > MAX_SAFE_INTEGER as f64);
    Ok(t)
}

/// A borrowed number, in `validate_checked_value`'s order
/// (`canonical_json/src/lib.rs:96-109`).
fn modal_env_number(n: &Number) -> ModalEnvelopeResult<ModalDocumentTally> {
    if let Some(v) = n.as_i64() {
        let mut t = modal_env_scalar(modal_env_digits(v.unsigned_abs()) + u64::from(v < 0));
        t.unsafe_numbers = u64::from(!(-MAX_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&v));
        return Ok(t);
    }
    if let Some(v) = n.as_u64() {
        return Ok(modal_env_unsigned(v));
    }
    match n.as_f64() {
        Some(v) => modal_env_float(v),
        None => Err(ModalEnvelopeError::Unprojectable { what: "number" }),
    }
}

/// The raw and escaped content length of a string (module docs,
/// "Leaves"); concatenation adds both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ModalEnvelopeText {
    pub(crate) raw: u64,
    pub(crate) escaped: u64,
}

/// The exact escaped length of `text` under serde_json's table.
pub(crate) fn modal_env_text(text: &str) -> ModalEnvelopeResult<ModalEnvelopeText> {
    let mut escaped = 0u64;
    for &byte in text.as_bytes() {
        let width = match byte {
            0x08 | 0x09 | 0x0A | 0x0C | 0x0D | b'"' | b'\\' => 2,
            0x00..=0x1F => 6,
            _ => 1,
        };
        escaped = modal_env_add("escaped string", escaped, width)?;
    }
    Ok(ModalEnvelopeText { raw: modal_env_count(text.len()), escaped })
}

/// The text of `a` followed by `b`.
pub(crate) fn modal_env_text_join(a: ModalEnvelopeText, b: ModalEnvelopeText) -> ModalEnvelopeResult<ModalEnvelopeText> {
    Ok(ModalEnvelopeText { raw: modal_env_add("joined text", a.raw, b.raw)?, escaped: modal_env_add("joined text", a.escaped, b.escaped)? })
}

/// The decimal text of `v` (ASCII: raw = escaped).
pub(crate) fn modal_env_text_digits(v: u64) -> ModalEnvelopeText {
    let digits = modal_env_digits(v);
    ModalEnvelopeText { raw: digits, escaped: digits }
}

/// An ASCII literal's text: every literal passed here is printable ASCII
/// with no quote or backslash, so no byte of it is escaped.
fn modal_env_ascii(text: &'static str) -> ModalEnvelopeText {
    let n = modal_env_count(text.len());
    ModalEnvelopeText { raw: n, escaped: n }
}

/// `prefix` followed by the digits of `v`.
fn modal_env_ref(prefix: &'static str, v: u64) -> ModalEnvelopeResult<ModalEnvelopeText> {
    modal_env_text_join(modal_env_ascii(prefix), modal_env_text_digits(v))
}

/// A string value of text `t`.
pub(crate) fn modal_env_string(t: ModalEnvelopeText) -> ModalEnvelopeResult<ModalDocumentTally> {
    let wire = modal_env_add("string literal", t.escaped, 2)?;
    Ok(ModalDocumentTally { wire, nodes: 1, strings: 1, string_bytes: t.raw, longest_literal: wire, ..MODAL_ENV_ZERO })
}

fn modal_env_str(text: &str) -> ModalEnvelopeResult<ModalDocumentTally> {
    modal_env_string(modal_env_text(text)?)
}

/// A hash of a value with tally `t`: the hex string, or null when the
/// checked profile refuses it (module docs, "Whether a hash is a string").
pub(crate) fn modal_env_digest(t: &ModalDocumentTally) -> ModalDocumentTally {
    if t.depth < MODAL_ENV_PARSE_DEPTH && t.unsafe_numbers == 0 {
        ModalDocumentTally { wire: MODAL_ENV_HEX + 2, nodes: 1, strings: 1, string_bytes: MODAL_ENV_HEX, longest_literal: MODAL_ENV_HEX + 2, ..MODAL_ENV_ZERO }
    } else {
        MODAL_ENV_NULL
    }
}

// ---------------------------------------------------------------------------
// Containers (module docs, "Composition").

#[derive(Debug, Clone, Copy, Default)]
struct ModalEnvelopeContainer {
    /// The children's sums; `depth` is the deepest child's until closed.
    inner: ModalDocumentTally,
    members: u64,
}

impl ModalEnvelopeContainer {
    /// One child, after `extra` bytes of its own (a member's name and colon).
    fn modal_env_slot(&mut self, extra: u64, child: &ModalDocumentTally) -> ModalEnvelopeResult<()> {
        let add = modal_env_add;
        let i = &mut self.inner;
        let separator = u64::from(self.members > 0);
        i.wire = add("container wire", add("container wire", add("container wire", i.wire, separator)?, extra)?, child.wire)?;
        i.float_bytes = add("container float bytes", i.float_bytes, child.float_bytes)?;
        i.floats = add("container floats", i.floats, child.floats)?;
        i.nodes = add("container nodes", i.nodes, child.nodes)?;
        i.objects = add("container objects", i.objects, child.objects)?;
        i.entries = add("container entries", i.entries, child.entries)?;
        i.key_bytes = add("container key bytes", i.key_bytes, child.key_bytes)?;
        i.arrays = add("container arrays", i.arrays, child.arrays)?;
        i.elements = add("container elements", i.elements, child.elements)?;
        i.strings = add("container strings", i.strings, child.strings)?;
        i.string_bytes = add("container string bytes", i.string_bytes, child.string_bytes)?;
        i.unsafe_numbers = add("container unsafe numbers", i.unsafe_numbers, child.unsafe_numbers)?;
        i.depth = i.depth.max(child.depth);
        i.longest_literal = i.longest_literal.max(child.longest_literal);
        i.nonempty_containers = add("container nonempty", i.nonempty_containers, child.nonempty_containers)?;
        // Every container and slot of the child is one level deeper here.
        i.container_levels =
            add("container levels", i.container_levels, add("container levels", child.container_levels, child.nonempty_containers)?)?;
        let child_slots = add("child slots", child.entries, child.elements)?;
        let child_levels = add("slot levels", add("slot levels", child.slot_levels, child_slots)?, 1)?;
        i.slot_levels = add("slot levels", i.slot_levels, child_levels)?;
        self.members = add("container members", self.members, 1)?;
        Ok(())
    }

    fn modal_env_close(&self, object: bool) -> ModalEnvelopeResult<ModalDocumentTally> {
        let mut t = self.inner;
        t.wire = modal_env_add("container wire", t.wire, 2)?;
        t.nodes = modal_env_add("container nodes", t.nodes, 1)?;
        if object {
            t.objects = modal_env_add("container objects", t.objects, 1)?;
        } else {
            t.arrays = modal_env_add("container arrays", t.arrays, 1)?;
        }
        t.depth = modal_env_add("container depth", t.depth, 1)?;
        t.nonempty_containers = modal_env_add("container nonempty", t.nonempty_containers, u64::from(self.members > 0))?;
        Ok(t)
    }
}

/// An object under construction, members in any order.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ModalEnvelopeObject(ModalEnvelopeContainer);

impl ModalEnvelopeObject {
    pub(crate) fn modal_env_member(&mut self, name: &str, child: ModalDocumentTally) -> ModalEnvelopeResult<()> {
        let name = modal_env_text(name)?;
        let quoted = modal_env_add("member name", name.escaped, 2)?;
        self.0.modal_env_slot(modal_env_add("member name", quoted, 1)?, &child)?;
        let i = &mut self.0.inner;
        i.entries = modal_env_add("object entries", i.entries, 1)?;
        i.key_bytes = modal_env_add("object key bytes", i.key_bytes, name.raw)?;
        i.longest_literal = i.longest_literal.max(quoted);
        Ok(())
    }

    pub(crate) fn modal_env_object(&self) -> ModalEnvelopeResult<ModalDocumentTally> {
        self.0.modal_env_close(true)
    }
}

/// An array under construction.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ModalEnvelopeArray(ModalEnvelopeContainer);

impl ModalEnvelopeArray {
    pub(crate) fn modal_env_element(&mut self, child: ModalDocumentTally) -> ModalEnvelopeResult<()> {
        self.0.modal_env_slot(0, &child)?;
        self.0.inner.elements = modal_env_add("array elements", self.0.inner.elements, 1)?;
        Ok(())
    }

    pub(crate) fn modal_env_array(&self) -> ModalEnvelopeResult<ModalDocumentTally> {
        self.0.modal_env_close(false)
    }
}

const MODAL_ENV_EMPTY_ARRAY: ModalDocumentTally =
    ModalDocumentTally { wire: 2, nodes: 1, arrays: 1, depth: 1, ..MODAL_ENV_ZERO };

/// The tally of a borrowed value, never descending into a container nested
/// deeper than `cap` (module docs, "The mirror").
pub(crate) fn modal_env_value(value: &Value, cap: u64) -> ModalEnvelopeResult<ModalDocumentTally> {
    modal_env_value_at(value, 0, cap)
}

fn modal_env_value_at(value: &Value, above: u64, cap: u64) -> ModalEnvelopeResult<ModalDocumentTally> {
    match value {
        Value::Null => Ok(MODAL_ENV_NULL),
        Value::Bool(b) => Ok(modal_env_bool(*b)),
        Value::Number(n) => modal_env_number(n),
        Value::String(s) => modal_env_str(s),
        Value::Array(items) => {
            if above >= cap {
                return Err(ModalEnvelopeError::DepthCap { cap });
            }
            let mut array = ModalEnvelopeArray::default();
            for item in items {
                array.modal_env_element(modal_env_value_at(item, above + 1, cap)?)?;
            }
            array.modal_env_array()
        }
        Value::Object(map) => {
            if above >= cap {
                return Err(ModalEnvelopeError::DepthCap { cap });
            }
            let mut object = ModalEnvelopeObject::default();
            for (name, item) in map {
                object.modal_env_member(name, modal_env_value_at(item, above + 1, cap)?)?;
            }
            object.modal_env_object()
        }
    }
}

// ---------------------------------------------------------------------------
// Derived storage (module docs, "Derived storage").

/// H(T): the logical heap of a built tree with tally `t`.
pub(crate) fn modal_env_heap(t: &ModalDocumentTally) -> ModalEnvelopeResult<u64> {
    let (add, mul) = (modal_env_add, modal_env_mul);
    let text = mul("heap strings", 2, add("heap strings", t.string_bytes, t.key_bytes)?)?;
    let headers = mul("heap strings", 8, add("heap strings", t.strings, t.entries)?)?;
    let slots = add("heap arrays", mul("heap arrays", 2, t.elements)?, mul("heap arrays", 4, t.arrays)?)?;
    let arrays = mul("heap arrays", MODAL_ENV_VALUE_BYTES, slots)?;
    let nodes = add("heap maps", t.objects, t.entries.div_ceil(MODAL_ENV_MAP_NODE_MIN_KEYS))?;
    let maps = mul("heap maps", MODAL_ENV_MAP_NODE_BYTES, nodes)?;
    add("heap", add("heap", text, headers)?, add("heap", arrays, maps)?)
}

/// The canonical text of a value with tally `t`, at most.
pub(crate) fn modal_env_canonical_bound(t: &ModalDocumentTally) -> ModalEnvelopeResult<u64> {
    let others = t.wire.checked_sub(t.float_bytes).ok_or(ModalEnvelopeError::Unprojectable { what: "float bytes beyond wire" })?;
    modal_env_add("canonical text", others, modal_env_mul("canonical text", MODAL_ENV_CANONICAL_FLOAT_MAX, t.floats)?)
}

/// The pretty text (two-space indentation, no trailing newline) of a value
/// with tally `t` at the root.
pub(crate) fn modal_env_pretty(t: &ModalDocumentTally) -> ModalEnvelopeResult<u64> {
    let (add, mul) = (modal_env_add, modal_env_mul);
    let slots = add("pretty slots", t.entries, t.elements)?;
    let containers = add("pretty containers", t.nonempty_containers, mul("pretty containers", 2, t.container_levels)?)?;
    let members = add("pretty slots", slots, mul("pretty slots", 2, t.slot_levels)?)?;
    add("pretty", add("pretty", t.wire, containers)?, add("pretty", members, t.entries)?)
}

/// HP(T): the peak while a value with tally `t` is hashed, the value
/// itself excluded.
pub(crate) fn modal_env_hash_peak(t: &ModalDocumentTally) -> ModalEnvelopeResult<u64> {
    let (add, mul) = (modal_env_add, modal_env_mul);
    let text = add("hash text", 128, mul("hash text", 3, t.wire)?)?;
    let parsed = add("hash parse", modal_env_heap(t)?, mul("hash parse", MODAL_ENV_VALUE_BYTES, t.elements)?)?;
    let sets = add("hash sets", add("hash sets", t.key_bytes, mul("hash sets", 86, t.entries)?)?, mul("hash sets", 212, t.depth)?)?;
    let literal = mul("hash literal", 3, t.longest_literal)?;
    let scratch = add("hash scratch", literal, 8)?;
    let canonical = add("hash canonical", 8, mul("hash canonical", 3, modal_env_canonical_bound(t)?)?)?;
    let sorting = mul("hash sorting", 16, t.entries)?;
    let quoting = add("hash quoting", 128, literal)?;
    let fixed = 256 + 512 + 384;
    let mut peak = 0;
    for term in [text, parsed, sets, scratch, canonical, sorting, quoting, fixed] {
        peak = add("hash peak", peak, term)?;
    }
    Ok(peak)
}

// ---------------------------------------------------------------------------
// The document's private tables, restated (held to the built documents and to
// `modal_semantic_table()` by the tests).

/// `document.rs:65-73`.
const MODAL_ENV_OPTION_MEMBERS: [&str; 7] = [
    "mode_count",
    "frequency_cutoff",
    "state_case",
    "modulus_basis_ref",
    "modulus_basis_temperature",
    "weight_to_mass_acceleration",
    "include_dynamic_only_restraints",
];
/// `document.rs:74-75, 98`: every component, direction and case name, by
/// its length (all ASCII).
const MODAL_ENV_COMPONENT: &str = "ux";
const MODAL_ENV_DIRECTION: &str = "x";
const MODAL_ENV_CASES: [&str; 2] = ["unit_max_translation", "unit_max_rotation"];
/// `document.rs:531`.

use UnitId as U;
const MODAL_ENV_T: &str = "unit_max_translation";
const MODAL_ENV_R: &str = "unit_max_rotation";
/// `document.rs:99-182` (`MODAL_KINDS`): kind, table, selector, unit rules.
pub(super) const MODAL_ENV_KINDS: &[(&str, &str, &str, &[(&str, UnitId)])] = &[
    ("node_coordinate", "nodes", "fixed", &[("fixed", U::Meter)]),
    (
        "mass_matrix_entry",
        "mass_matrix",
        "dof_pair_class",
        &[
            ("translation_translation", U::Kilogram),
            ("translation_rotation", U::KilogramMeterPerRadian),
            ("rotation_rotation", U::KilogramSquareMeterPerSquareRadian),
        ],
    ),
    ("modal_total_mass", "mass_totals", "fixed", &[("fixed", U::Kilogram)]),
    ("modal_free_dof_mass", "mass_totals", "fixed", &[("fixed", U::Kilogram)]),
    ("modal_circular_frequency", "modes", "fixed", &[("fixed", U::RadianPerSecond)]),
    ("modal_frequency", "modes", "fixed", &[("fixed", U::Hertz)]),
    ("modal_period", "modes", "fixed", &[("fixed", U::Second)]),
    ("modal_generalized_mass", "modes", "normalization_case", &[(MODAL_ENV_T, U::Kilogram), (MODAL_ENV_R, U::KilogramSquareMeterPerSquareRadian)]),
    ("modal_cluster_error_estimate", "modes", "fixed", &[("fixed", U::One)]),
    ("mode_shape_translation", "shapes", "normalization_case", &[(MODAL_ENV_T, U::One), (MODAL_ENV_R, U::MeterPerRadian)]),
    ("mode_shape_rotation", "shapes", "normalization_case", &[(MODAL_ENV_T, U::RadianPerMeter), (MODAL_ENV_R, U::One)]),
    ("modal_participation_factor", "participation", "normalization_case", &[(MODAL_ENV_T, U::One), (MODAL_ENV_R, U::RadianPerMeter)]),
    ("modal_effective_mass", "participation", "fixed", &[("fixed", U::Kilogram)]),
    ("modal_effective_mass_fraction", "participation", "fixed", &[("fixed", U::One)]),
    ("modal_cumulative_effective_mass_fraction", "participation", "fixed", &[("fixed", U::One)]),
    ("cluster_error_estimate", "clusters", "fixed", &[("fixed", U::One)]),
    ("computed_circular_frequency", "computed_modes", "fixed", &[("fixed", U::RadianPerSecond)]),
    ("computed_frequency", "computed_modes", "fixed", &[("fixed", U::Hertz)]),
    ("residual_backward_error", "computed_modes", "fixed", &[("fixed", U::One)]),
    ("eigenvalue_condition_estimate", "computed_modes", "fixed", &[("fixed", U::One)]),
    ("eigenvalue_error_estimate", "computed_modes", "fixed", &[("fixed", U::One)]),
    ("sturm_shift_circular_frequency", "sturm_candidates", "fixed", &[("fixed", U::RadianPerSecond)]),
    ("sturm_formation_term", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_formation_separation_ratio", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_factor_term", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_backward_error", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_inverse_norm_estimate", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_conditioning_product", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_separation_ratio", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_growth_factor", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_max_abs_lower_factor", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_max_abs_pivot", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("sturm_max_abs_scaled_diagonal", "sturm_candidates", "fixed", &[("fixed", U::One)]),
    ("identity_relative_discrepancy", "identity", "fixed", &[("fixed", U::One)]),
    ("member_length", "discretization", "fixed", &[("fixed", U::Meter)]),
    ("member_fixed_fixed_frequency", "discretization", "fixed", &[("fixed", U::Hertz)]),
    ("member_frequency_ratio", "discretization", "fixed", &[("fixed", U::One)]),
    ("basis_no_translation_threshold", "header", "fixed", &[("fixed", U::One)]),
    ("basis_bounding_box_diagonal", "header", "fixed", &[("fixed", U::Meter)]),
    ("basis_mass_center", "header", "fixed", &[("fixed", U::Meter)]),
    ("request_frequency_cutoff", "header", "fixed", &[("fixed", U::Hertz)]),
    ("request_weight_to_mass_acceleration", "header", "fixed", &[("fixed", U::MeterPerSecondSquared)]),
    ("identity_acceleration", "header", "fixed", &[("fixed", U::MeterPerSecondSquared)]),
    ("placement_circular_frequency", "header", "fixed", &[("fixed", U::RadianPerSecond)]),
    ("sturm_premise_parameter", "header", "fixed", &[("fixed", U::One)]),
    ("orthogonality_max_ratio", "header", "fixed", &[("fixed", U::One)]),
    ("stiffness_formation_difference", "header", "fixed", &[("fixed", U::One)]),
    ("mass_formation_difference", "header", "fixed", &[("fixed", U::SquareSecond)]),
    ("reformed_stiffness_norm", "header", "fixed", &[("fixed", U::One)]),
    ("reformed_mass_norm", "header", "fixed", &[("fixed", U::SquareSecond)]),
    ("section_derivation_difference", "header", "fixed", &[("fixed", U::One)]),
];

/// `modal_doc_rule_of` (`document.rs:224-231`): the catalog id of `kind`'s
/// unit under `key`.
fn modal_env_unit(kind: &str, key: &str) -> ModalEnvelopeResult<&'static str> {
    MODAL_ENV_KINDS
        .iter()
        .find(|(k, ..)| *k == kind)
        .and_then(|(_, _, _, rules)| rules.iter().find(|(k, _)| *k == key))
        .map(|(_, unit)| unit.catalog_id())
        .ok_or(ModalEnvelopeError::Unprojectable { what: "unit rule" })
}

/// `modal_semantic_table` (`document.rs:204-214`).
pub(crate) fn modal_env_semantic_table() -> ModalEnvelopeResult<ModalDocumentTally> {
    let mut kinds = ModalEnvelopeArray::default();
    for (kind, table, selector, rules) in MODAL_ENV_KINDS {
        let mut units = ModalEnvelopeObject::default();
        for (key, id) in *rules {
            let definition = id.definition();
            let mut rule = ModalEnvelopeObject::default();
            rule.modal_env_member("unit_status", modal_env_str("catalog")?)?;
            rule.modal_env_member("unit", modal_env_str(id.catalog_id())?)?;
            rule.modal_env_member("symbol", modal_env_str(definition.symbol)?)?;
            rule.modal_env_member("dimension", modal_env_str(definition.dimension.as_str())?)?;
            units.modal_env_member(key, rule.modal_env_object()?)?;
        }
        let mut entry = ModalEnvelopeObject::default();
        entry.modal_env_member("kind", modal_env_str(kind)?)?;
        entry.modal_env_member("table", modal_env_str(table)?)?;
        entry.modal_env_member("selector", modal_env_str(selector)?)?;
        entry.modal_env_member("units", units.modal_env_object()?)?;
        kinds.modal_env_element(entry.modal_env_object()?)?;
    }
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("id", modal_env_str(MODAL_RESULT_SEMANTICS)?)?;
    out.modal_env_member("document_kind", modal_env_str(MODAL_RESULT_KIND)?)?;
    out.modal_env_member("schema_version", modal_env_str(MODAL_RESULT_VERSION)?)?;
    out.modal_env_member("canonical_profile", modal_env_str(MODAL_RESULT_CANONICAL)?)?;
    out.modal_env_member("equilibrated_basis_kinds", MODAL_ENV_EMPTY_ARRAY)?;
    out.modal_env_member("kinds", kinds.modal_env_array()?)?;
    out.modal_env_object()
}

// ---------------------------------------------------------------------------
// Values (`document.rs:233-270`).

/// `modal_doc_quantity`.
pub(crate) fn modal_env_quantity(
    kind: &str,
    key: &str,
    value: Option<f64>,
    below_normal: bool,
    undefined: Option<&str>,
) -> ModalEnvelopeResult<ModalDocumentTally> {
    let (number, reason) = match (value, undefined) {
        (_, Some(reason)) => (MODAL_ENV_NULL, modal_env_str(reason)?),
        (None, None) => (MODAL_ENV_NULL, modal_env_str("not_formed")?),
        (Some(v), None) if !v.is_finite() => (MODAL_ENV_NULL, modal_env_str("not_finite")?),
        (Some(v), None) if v.abs() >= MODAL_ENV_PROFILE_LIMIT => (MODAL_ENV_NULL, modal_env_str("outside_canonical_profile")?),
        (Some(v), None) => (modal_env_float(v)?, MODAL_ENV_NULL),
    };
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("kind", modal_env_str(kind)?)?;
    out.modal_env_member("value", number)?;
    out.modal_env_member("unit", modal_env_str(modal_env_unit(kind, key)?)?)?;
    out.modal_env_member("unit_status", modal_env_str("catalog")?)?;
    out.modal_env_member("below_normal", modal_env_bool(below_normal))?;
    out.modal_env_member("undefined_reason", reason)?;
    out.modal_env_object()
}

/// `modal_doc_fixed`.
fn modal_env_fixed(kind: &str, value: f64) -> ModalEnvelopeResult<ModalDocumentTally> {
    modal_env_quantity(kind, "fixed", Some(value), false, None)
}

/// `modal_doc_optional`.
fn modal_env_optional(kind: &str, value: Option<f64>, undefined: &str) -> ModalEnvelopeResult<ModalDocumentTally> {
    match value {
        Some(v) => modal_env_fixed(kind, v),
        None => modal_env_quantity(kind, "fixed", None, false, Some(undefined)),
    }
}

/// `modal_doc_angular`.
fn modal_env_angular(value: f64) -> Option<f64> {
    (value.is_finite() && value > 0.0).then(|| value.sqrt())
}

/// `modal_doc_standing`.
fn modal_env_standing(standing: &ModalStanding) -> ModalEnvelopeResult<ModalDocumentTally> {
    let mut reasons = ModalEnvelopeArray::default();
    let token = match standing {
        ModalStanding::ChecksPassed => "checks_passed",
        ModalStanding::Sensitive(list) => {
            for reason in list {
                reasons.modal_env_element(modal_env_str(reason)?)?;
            }
            "sensitive"
        }
    };
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("token", modal_env_str(token)?)?;
    out.modal_env_member("reasons", reasons.modal_env_array()?)?;
    out.modal_env_object()
}

/// `modal_doc_dof_id`: `dof:{node id}:{component}`.
fn modal_env_dof(ids: &[String], dof: usize) -> ModalEnvelopeResult<ModalEnvelopeText> {
    let node = ids.get(dof / 6).ok_or(ModalEnvelopeError::Unprojectable { what: "degree of freedom past the nodes" })?;
    let text = modal_env_text_join(modal_env_ascii("dof:"), modal_env_text(node)?)?;
    modal_env_text_join(text, modal_env_ascii(":ux"))
}

/// `modal_doc_node_id`: `node:{node id}`.
fn modal_env_node(id: &str) -> ModalEnvelopeResult<ModalEnvelopeText> {
    modal_env_text_join(modal_env_ascii("node:"), modal_env_text(id)?)
}

/// `index + 1` (the builders' 1-based references), checked.
fn modal_env_next(index: usize) -> ModalEnvelopeResult<u64> {
    modal_env_add("one-based index", modal_env_count(index), 1)
}

// ---------------------------------------------------------------------------
// Rows, tables and what they leave live (module docs, "Derived storage").

/// A table under construction: its rows and their checksums.
#[derive(Debug, Clone, Copy, Default)]
struct ModalEnvelopeTable {
    rows: ModalEnvelopeArray,
    checksums: ModalEnvelopeArray,
    count: u64,
}

/// What the construction leaves live and its largest transients.
#[derive(Debug, Clone, Copy, Default)]
struct ModalEnvelopeLedger {
    /// Every sealed row, summed; the componentwise largest.
    rows_total: ModalDocumentTally,
    row_max: ModalDocumentTally,
    /// The componentwise largest checksum array.
    checksums_max: ModalDocumentTally,
    rows: u64,
    tables: u64,
    /// The header members, summed.
    header: ModalDocumentTally,
    /// H of a diagnostic the builder creates, with its `source` string.
    created_diagnostic: u64,
    global_dofs: u64,
    bind_transient: u64,
    census_transient: u64,
    identity_transient: u64,
    /// The largest HP of a value hashed outside rows and tables.
    hash_peak: u64,
}

impl ModalEnvelopeLedger {
    /// `modal_doc_row`: `fields` with `row_id`, `ordinal` and the checksum of
    /// the row without it, added to `table`.
    fn modal_env_seal(
        &mut self,
        table: &mut ModalEnvelopeTable,
        fields: ModalEnvelopeObject,
        row_id: ModalEnvelopeText,
        ordinal: u64,
    ) -> ModalEnvelopeResult<()> {
        let mut row = fields;
        row.modal_env_member("row_id", modal_env_string(row_id)?)?;
        row.modal_env_member("ordinal", modal_env_unsigned(ordinal))?;
        let checksum = modal_env_digest(&row.modal_env_object()?);
        row.modal_env_member("checksum", checksum)?;
        let sealed = row.modal_env_object()?;
        self.rows_total = modal_env_sum(&self.rows_total, &sealed)?;
        self.row_max = modal_env_max(&self.row_max, &sealed);
        self.rows = modal_env_add("rows", self.rows, 1)?;
        table.rows.modal_env_element(sealed)?;
        table.checksums.modal_env_element(checksum)?;
        table.count = modal_env_add("table rows", table.count, 1)?;
        Ok(())
    }

    /// `modal_doc_table`.
    fn modal_env_publish(&mut self, table: &ModalEnvelopeTable) -> ModalEnvelopeResult<ModalDocumentTally> {
        let checksums = table.checksums.modal_env_array()?;
        self.checksums_max = modal_env_max(&self.checksums_max, &checksums);
        self.tables = modal_env_add("tables", self.tables, 1)?;
        let mut out = ModalEnvelopeObject::default();
        out.modal_env_member("status", modal_env_str("published")?)?;
        out.modal_env_member("row_count", modal_env_unsigned(table.count))?;
        out.modal_env_member("digest", modal_env_digest(&checksums))?;
        out.modal_env_member("rows", table.rows.modal_env_array()?)?;
        out.modal_env_object()
    }

    /// A header member of the solved document.
    fn modal_env_header(&mut self, doc: &mut ModalEnvelopeObject, name: &str, member: ModalDocumentTally) -> ModalEnvelopeResult<()> {
        self.header = modal_env_sum(&self.header, &member)?;
        doc.modal_env_member(name, member)
    }
}

// ---------------------------------------------------------------------------
// Diagnostics.

/// Upper bounds on one diagnostic a caller will create (A2's named
/// refusal): each text's raw and escaped length, and its affected refs, each
/// within `affected_ref`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct ModalEnvelopeDiagnosticBound {
    pub(crate) id: ModalEnvelopeText,
    pub(crate) code: ModalEnvelopeText,
    pub(crate) severity: ModalEnvelopeText,
    pub(crate) message: ModalEnvelopeText,
    pub(crate) affected_refs: u64,
    pub(crate) affected_ref: ModalEnvelopeText,
}

/// A diagnostic row's fields (`document.rs:356-357`) before its seal.
fn modal_env_diagnostic_fields(
    id: ModalEnvelopeText,
    code: ModalEnvelopeText,
    severity: ModalEnvelopeText,
    message: ModalEnvelopeText,
    refs: ModalDocumentTally,
    source: &str,
) -> ModalEnvelopeResult<ModalEnvelopeObject> {
    let mut fields = ModalEnvelopeObject::default();
    fields.modal_env_member("diagnostic_id", modal_env_string(id)?)?;
    fields.modal_env_member("code", modal_env_string(code)?)?;
    fields.modal_env_member("severity", modal_env_string(severity)?)?;
    fields.modal_env_member("message", modal_env_string(message)?)?;
    fields.modal_env_member("affected_refs", refs)?;
    fields.modal_env_member("source", modal_env_str(source)?)?;
    Ok(fields)
}

fn modal_env_typed_fields(d: &Diagnostic, source: &str) -> ModalEnvelopeResult<ModalEnvelopeObject> {
    let mut refs = ModalEnvelopeArray::default();
    for r in &d.affected_refs {
        refs.modal_env_element(modal_env_str(r)?)?;
    }
    modal_env_diagnostic_fields(
        modal_env_text(&d.id)?,
        modal_env_text(&d.code)?,
        modal_env_text(&d.severity)?,
        modal_env_text(&d.message)?,
        refs.modal_env_array()?,
        source,
    )
}

fn modal_env_bound_fields(bound: &ModalEnvelopeDiagnosticBound) -> ModalEnvelopeResult<ModalEnvelopeObject> {
    let mut refs = ModalEnvelopeArray::default();
    let each = modal_env_string(bound.affected_ref)?;
    for _ in 0..bound.affected_refs {
        refs.modal_env_element(each)?;
    }
    modal_env_diagnostic_fields(bound.id, bound.code, bound.severity, bound.message, refs.modal_env_array()?, "modal")
}

/// A diagnostic row (`document.rs:352-359`) sealed into `table`.
fn modal_env_diagnostic_row(
    ledger: &mut ModalEnvelopeLedger,
    table: &mut ModalEnvelopeTable,
    prefix: &'static str,
    ordinal: usize,
    fields: ModalEnvelopeObject,
) -> ModalEnvelopeResult<()> {
    let id = modal_env_text_join(modal_env_ascii(prefix), modal_env_ref(":", modal_env_count(ordinal))?)?;
    ledger.modal_env_seal(table, fields, id, modal_env_count(ordinal))
}

// ---------------------------------------------------------------------------
// The binding, the census and the identity.

/// `modal_doc_parameters` (`document.rs:362-383`).
pub(crate) fn modal_env_parameters(params: &ModalIntegrityParameters) -> ModalEnvelopeResult<ModalDocumentTally> {
    let f = modal_env_float;
    let list = |values: &[f64]| -> ModalEnvelopeResult<ModalDocumentTally> {
        let mut array = ModalEnvelopeArray::default();
        for &v in values {
            array.modal_env_element(modal_env_float(v)?)?;
        }
        array.modal_env_array()
    };
    let mut tolerance = ModalEnvelopeObject::default();
    tolerance.modal_env_member("relative", f(params.identity_tolerance.relative)?)?;
    tolerance.modal_env_member("absolute", f(params.identity_tolerance.absolute)?)?;
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("infinite_mu_relative_threshold", f(params.infinite_mu_relative_threshold)?)?;
    out.modal_env_member("mass_pivot_relative_threshold", f(params.mass_pivot_relative_threshold)?)?;
    out.modal_env_member("resolution_floor_constant", f(params.resolution_floor_constant)?)?;
    out.modal_env_member("resolution_floor_multiple", f(params.resolution_floor_multiple)?)?;
    out.modal_env_member("formed_backward_error_multiple", f(params.formed_backward_error_multiple)?)?;
    out.modal_env_member("screen_error_estimate_threshold", f(params.screen_error_estimate_threshold)?)?;
    out.modal_env_member("max_ql_iterations_per_eigenvalue", modal_env_index(params.max_ql_iterations_per_eigenvalue))?;
    out.modal_env_member("residual_threshold", f(params.residual_threshold)?)?;
    out.modal_env_member("orthogonality_threshold", f(params.orthogonality_threshold)?)?;
    out.modal_env_member("cluster_tolerance", f(params.cluster_tolerance)?)?;
    out.modal_env_member("no_translation_threshold", f(params.no_translation_threshold)?)?;
    out.modal_env_member("sturm_conditioning_multiple", f(params.sturm_conditioning_multiple)?)?;
    out.modal_env_member("sturm_separation_multiple", f(params.sturm_separation_multiple)?)?;
    out.modal_env_member("sturm_growth_limit", f(params.sturm_growth_limit)?)?;
    out.modal_env_member("sturm_max_candidates", modal_env_index(params.sturm_max_candidates))?;
    out.modal_env_member("sturm_gap_log_positions", list(&params.sturm_gap_log_positions)?)?;
    out.modal_env_member("sturm_whole_spectrum_offsets", list(&params.sturm_whole_spectrum_offsets)?)?;
    out.modal_env_member("identity_tolerance", tolerance.modal_env_object()?)?;
    out.modal_env_object()
}

/// `modal_doc_binding` (`document.rs:387-416`), with `census` in place of
/// its null when the solved branch forms one (`:536`).
fn modal_env_binding(
    ledger: &mut ModalEnvelopeLedger,
    request: &Value,
    params: &ModalIntegrityParameters,
    state: Option<(usize, &str)>,
    census: Option<ModalDocumentTally>,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentTally> {
    let whole = modal_env_value(request, cap)?;
    let mut options = ModalEnvelopeObject::default();
    for name in MODAL_ENV_OPTION_MEMBERS {
        if let Some(value) = request.get(name) {
            options.modal_env_member(name, modal_env_value(value, cap)?)?;
        }
    }
    let options = options.modal_env_object()?;
    let mut undeclared = ModalEnvelopeArray::default();
    let mut undeclared_count = 0u64;
    if let Some(members) = request.as_object() {
        for name in members.keys() {
            let name = name.as_str();
            if name != "model" && name != "materials" && !MODAL_ENV_OPTION_MEMBERS.contains(&name) {
                undeclared.modal_env_element(modal_env_str(name)?)?;
                undeclared_count = modal_env_add("undeclared members", undeclared_count, 1)?;
            }
        }
    }
    let materials = match request.get("materials") {
        Some(value) => modal_env_value(value, cap)?,
        None => MODAL_ENV_EMPTY_ARRAY,
    };
    let model = request.get("model").map(|model| modal_env_value(model, cap)).transpose()?;
    // The largest hash peak among the binding's five hashes.
    let mut peak = 0u64;
    let mut hash = |t: &ModalDocumentTally| -> ModalEnvelopeResult<ModalDocumentTally> {
        peak = peak.max(modal_env_hash_peak(t)?);
        Ok(modal_env_digest(t))
    };
    let state = match state {
        None => MODAL_ENV_NULL,
        Some((index, id)) => {
            // `request["model"]["load_cases"][index]`: null where absent.
            let case = request.get("model").and_then(|m| m.get("load_cases")).and_then(|c| c.get(index));
            let case = match case {
                Some(value) => modal_env_value(value, cap)?,
                None => MODAL_ENV_NULL,
            };
            let mut out = ModalEnvelopeObject::default();
            out.modal_env_member("case_id", modal_env_str(id)?)?;
            out.modal_env_member("case_index", modal_env_index(index))?;
            out.modal_env_member("case_sha256", hash(&case)?)?;
            out.modal_env_object()?
        }
    };
    let parameters = modal_env_parameters(params)?;
    let request_hash = hash(&whole)?;
    let model_hash = match &model {
        Some(model) => hash(model)?,
        None => MODAL_ENV_NULL,
    };
    let materials_hash = hash(&materials)?;
    let parameters_hash = hash(&parameters)?;
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("request_sha256", request_hash)?;
    out.modal_env_member("model_sha256", model_hash)?;
    out.modal_env_member("materials_sha256", materials_hash)?;
    out.modal_env_member("materials_member", modal_env_str(if request.get("materials").is_some() { "authored" } else { "absent" })?)?;
    out.modal_env_member("options", options)?;
    out.modal_env_member("undeclared_members", undeclared.modal_env_array()?)?;
    out.modal_env_member("state_case", state)?;
    out.modal_env_member("integrity_parameters_sha256", parameters_hash)?;
    out.modal_env_member("integrity_parameters", parameters)?;
    out.modal_env_member("subject_census", census.unwrap_or(MODAL_ENV_NULL))?;
    // X_bind: the locals live while `json!` builds the binding, and the
    // largest hash among them (module docs).
    let (add, mul, heap) = (modal_env_add, modal_env_mul, modal_env_heap);
    let names = mul("binding names", 16, add("binding names", mul("binding names", 3, undeclared_count)?, 4)?)?;
    let mut transient = add("binding", heap(&options)?, names)?;
    transient = add("binding", transient, heap(&materials)?)?;
    transient = add("binding", transient, heap(&state)?)?;
    transient = add("binding", transient, mul("binding", 2, heap(&parameters)?)?)?;
    transient = add("binding", transient, peak)?;
    ledger.hash_peak = ledger.hash_peak.max(peak);
    ledger.bind_transient = ledger.bind_transient.max(transient);
    out.modal_env_object()
}

/// `modal_doc_subject_census` (`document.rs:423-436`).
fn modal_env_subject_census(ledger: &mut ModalEnvelopeLedger, outcome: &ModalOutcome) -> ModalEnvelopeResult<ModalDocumentTally> {
    let mut counts = [0u64; 4];
    let mut list = ModalEnvelopeArray::default();
    for (kind, id) in &outcome.subjects {
        let slot = match *kind {
            "straight_member" => Some(0),
            "arc" => Some(1),
            "component_body" => Some(2),
            "support" => Some(3),
            _ => None,
        };
        if let Some(slot) = slot {
            counts[slot] = modal_env_add("subject count", counts[slot], 1)?;
        }
        let mut entry = ModalEnvelopeObject::default();
        entry.modal_env_member("kind", modal_env_str(kind)?)?;
        entry.modal_env_member("id", modal_env_str(id)?)?;
        list.modal_env_element(entry.modal_env_object()?)?;
    }
    let list = list.modal_env_array()?;
    let mut out = ModalEnvelopeObject::default();
    out.modal_env_member("straight_members", modal_env_unsigned(counts[0]))?;
    out.modal_env_member("arcs", modal_env_unsigned(counts[1]))?;
    out.modal_env_member("component_bodies", modal_env_unsigned(counts[2]))?;
    out.modal_env_member("supports", modal_env_unsigned(counts[3]))?;
    out.modal_env_member("nonlinear_supports", modal_env_index(outcome.nonlinear_state.as_ref().map_or(0, |s| s.supports.len())))?;
    let peak = modal_env_hash_peak(&list)?;
    ledger.hash_peak = ledger.hash_peak.max(peak);
    out.modal_env_member("subjects_sha256", modal_env_digest(&list))?;
    let subjects = modal_env_mul("census subjects", 32, modal_env_count(outcome.subjects.len()))?;
    let transient = modal_env_add("census", modal_env_add("census", subjects, modal_env_heap(&list)?)?, modal_env_add("census", peak, 512)?)?;
    ledger.census_transient = ledger.census_transient.max(transient);
    out.modal_env_object()
}

/// `modal_doc_identity` (`document.rs:468-491`).
fn modal_env_identity(
    ledger: &mut ModalEnvelopeLedger,
    binding: &ModalDocumentTally,
    standing: ModalParameterStanding<'_>,
) -> ModalEnvelopeResult<ModalEnvelopeObject> {
    let (parameter_standing, label) = match standing {
        ModalParameterStanding::Selected => ("selected", MODAL_SELECTED_PARAMETER_LABEL),
        ModalParameterStanding::TestLocal(label) => ("test_local", label),
    };
    let semantic = modal_env_semantic_table()?;
    // The run hash's input: two hashes, each at most a hex string.
    let hex = modal_env_digest(&MODAL_ENV_NULL);
    let mut run_input = ModalEnvelopeObject::default();
    run_input.modal_env_member("request_sha256", hex)?;
    run_input.modal_env_member("integrity_parameters_sha256", hex)?;
    let run_input = run_input.modal_env_object()?;
    let (semantic_peak, run_peak) = (modal_env_hash_peak(&semantic)?, modal_env_hash_peak(&run_input)?);
    ledger.hash_peak = ledger.hash_peak.max(semantic_peak).max(run_peak);
    let transient = modal_env_add(
        "identity",
        modal_env_add("identity", modal_env_mul("identity", 2, modal_env_heap(&semantic)?)?, semantic_peak)?,
        modal_env_add("identity", run_peak, 1024)?,
    )?;
    ledger.identity_transient = ledger.identity_transient.max(transient);

    let mut contract = ModalEnvelopeObject::default();
    contract.modal_env_member("id", modal_env_str(MODAL_RESULT_SEMANTICS)?)?;
    contract.modal_env_member("sha256", modal_env_digest(&semantic))?;
    let mut limitations = ModalEnvelopeArray::default();
    for text in MODAL_RESULT_LIMITATIONS {
        limitations.modal_env_element(modal_env_str(text)?)?;
    }
    let mut profile = ModalEnvelopeObject::default();
    profile.modal_env_member("profile_id", modal_env_str(MODAL_RESULT_PROFILE)?)?;
    profile.modal_env_member("limitations", limitations.modal_env_array()?)?;
    let mut admission = ModalEnvelopeObject::default();
    admission.modal_env_member("status", modal_env_str("experimental")?)?;
    admission.modal_env_member("public_engineering_result", modal_env_bool(false))?;
    admission.modal_env_member("parameter_standing", modal_env_str(parameter_standing)?)?;
    admission.modal_env_member("parameter_label", modal_env_str(label)?)?;
    // The run hash's input holds strings or nulls only, so it is a hex
    // string and `run_id` is `modal-run:` and its first 16 digits.
    let mut run = ModalEnvelopeObject::default();
    run.modal_env_member("run_id", modal_env_string(modal_env_text_join(modal_env_ascii("modal-run:"), modal_env_ascii("0123456789abcdef"))?)?)?;
    run.modal_env_member("mode_id_scope", modal_env_str("run_local")?)?;
    run.modal_env_member("solver_path", modal_env_str("dense_scrutiny")?)?;
    let mut doc = ModalEnvelopeObject::default();
    doc.modal_env_member("document_kind", modal_env_str(MODAL_RESULT_KIND)?)?;
    doc.modal_env_member("schema_version", modal_env_str(MODAL_RESULT_VERSION)?)?;
    doc.modal_env_member("semantic_contract", contract.modal_env_object()?)?;
    doc.modal_env_member("profile", profile.modal_env_object()?)?;
    doc.modal_env_member("canonical_profile", modal_env_str(MODAL_RESULT_CANONICAL)?)?;
    doc.modal_env_member("admission", admission.modal_env_object()?)?;
    doc.modal_env_member("run", run.modal_env_object()?)?;
    doc.modal_env_member("binding", *binding)?;
    Ok(doc)
}

/// The refusals a refused document carries.
enum ModalEnvelopeRefusals<'a> {
    /// The run's own, typed.
    Typed(&'a [Diagnostic]),
    /// One the builder creates (parse, skew), held while the document is
    /// built.
    Made(&'a Diagnostic),
    /// One A2 will create, within its bound.
    Bounded(&'a ModalEnvelopeDiagnosticBound),
}

/// `modal_doc_refused` (`document.rs:493-499`).
fn modal_env_refused(
    ledger: &mut ModalEnvelopeLedger,
    binding: &ModalDocumentTally,
    standing: ModalParameterStanding<'_>,
    refusals: ModalEnvelopeRefusals<'_>,
) -> ModalEnvelopeResult<ModalDocumentTally> {
    let mut doc = modal_env_identity(ledger, binding, standing)?;
    doc.modal_env_member("status", modal_env_str("refused")?)?;
    let mut table = ModalEnvelopeTable::default();
    match refusals {
        ModalEnvelopeRefusals::Typed(list) => {
            for (i, d) in list.iter().enumerate() {
                modal_env_diagnostic_row(ledger, &mut table, "refusal", i, modal_env_typed_fields(d, "modal")?)?;
            }
        }
        ModalEnvelopeRefusals::Made(d) => {
            let fields = modal_env_typed_fields(d, "modal")?;
            // D: its strings and refs (H of its row fields covers them) and
            // its own `source` (`lib.rs:16088`), ≤ 2L + 8.
            let source = d.source.as_deref().map_or(0, |s| 2 * modal_env_count(s.len()) + 8);
            ledger.created_diagnostic = modal_env_add("created diagnostic", modal_env_heap(&fields.modal_env_object()?)?, source)?;
            modal_env_diagnostic_row(ledger, &mut table, "refusal", 0, fields)?;
        }
        ModalEnvelopeRefusals::Bounded(bound) => {
            let fields = modal_env_bound_fields(bound)?;
            // D: as above, with a `source` of at most 64 bytes.
            ledger.created_diagnostic = modal_env_add("created diagnostic", modal_env_heap(&fields.modal_env_object()?)?, 2 * 64 + 8)?;
            modal_env_diagnostic_row(ledger, &mut table, "refusal", 0, fields)?;
        }
    }
    doc.modal_env_member("refusals", ledger.modal_env_publish(&table)?)?;
    doc.modal_env_object()
}

// ---------------------------------------------------------------------------
// The envelope.

/// The envelope of one document (module docs, "Derived storage").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModalDocumentEnvelope {
    /// The document's tally (exact).
    pub(crate) tally: ModalDocumentTally,
    /// H(document).
    pub(crate) heap_bytes: u64,
    /// The largest HP of any value hashed while it is built.
    pub(crate) hash_peak_bytes: u64,
    /// C: the construction peak, hashing included.
    pub(crate) construction_peak_bytes: u64,
    /// The pretty text (no trailing newline).
    pub(crate) pretty_bytes: u64,
    /// Its canonical text, at most.
    pub(crate) canonical_bound_bytes: u64,
}

fn modal_env_envelope(
    ledger: &ModalEnvelopeLedger,
    doc: &ModalDocumentTally,
    binding: &ModalDocumentTally,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let (add, mul, heap) = (modal_env_add, modal_env_mul, modal_env_heap);
    let row_peak = modal_env_hash_peak(&ledger.row_max)?;
    let table_peak = modal_env_hash_peak(&ledger.checksums_max)?;
    let hash_peak = ledger.hash_peak.max(row_peak).max(table_peak);
    let row_transient = add("row transient", add("row transient", mul("row transient", 3, heap(&ledger.row_max)?)?, row_peak)?, 512)?;
    let table_transient = add("table transient", heap(&ledger.checksums_max)?, table_peak)?;
    let header_transient = mul("header transient", 2, heap(&ledger.header)?)?;
    let transient = ledger
        .bind_transient
        .max(ledger.census_transient)
        .max(ledger.identity_transient)
        .max(header_transient)
        .max(row_transient)
        .max(table_transient);
    let row_vectors = mul(
        "row vectors",
        MODAL_ENV_VALUE_BYTES,
        add("row vectors", mul("row vectors", 3, ledger.rows)?, mul("row vectors", 4, ledger.tables)?)?,
    )?;
    let free_index = mul("free index", 16, ledger.global_dofs)?;
    let doc_heap = heap(doc)?;
    let mut peak = ledger.created_diagnostic;
    for term in [heap(binding)?, doc_heap, heap(&ledger.rows_total)?, row_vectors, free_index, transient, 2048] {
        peak = add("construction peak", peak, term)?;
    }
    Ok(ModalDocumentEnvelope {
        tally: *doc,
        heap_bytes: doc_heap,
        hash_peak_bytes: hash_peak,
        construction_peak_bytes: peak,
        pretty_bytes: modal_env_pretty(doc)?,
        canonical_bound_bytes: modal_env_canonical_bound(doc)?,
    })
}

/// The parse-refusal branch (`document.rs:507-513`): `diagnostic` is the
/// one the builder makes there from `parse_modal_request`'s message (taken
/// whole, so that no code or message of the builder is restated here).
pub(crate) fn modal_env_parse_refused(
    request: &Value,
    params: &ModalIntegrityParameters,
    standing: ModalParameterStanding<'_>,
    diagnostic: &Diagnostic,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let binding = modal_env_binding(&mut ledger, request, params, None, None, cap)?;
    let doc = modal_env_refused(&mut ledger, &binding, standing, ModalEnvelopeRefusals::Made(diagnostic))?;
    modal_env_envelope(&ledger, &doc, &binding)
}

/// The run-refusal branch (`document.rs:516-519`): `state` as the builder
/// finds it from the parsed request.
pub(crate) fn modal_env_run_refused(
    request: &Value,
    params: &ModalIntegrityParameters,
    standing: ModalParameterStanding<'_>,
    state: Option<(usize, &str)>,
    refusals: &[Diagnostic],
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let binding = modal_env_binding(&mut ledger, request, params, state, None, cap)?;
    let doc = modal_env_refused(&mut ledger, &binding, standing, ModalEnvelopeRefusals::Typed(refusals))?;
    modal_env_envelope(&ledger, &doc, &binding)
}

/// The skew-refusal branch (`document.rs:526-535`): `state` from the
/// outcome, `diagnostic` the one the builder makes there.
pub(crate) fn modal_env_skew_refused(
    request: &Value,
    params: &ModalIntegrityParameters,
    standing: ModalParameterStanding<'_>,
    state: Option<(usize, &str)>,
    diagnostic: &Diagnostic,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let binding = modal_env_binding(&mut ledger, request, params, state, None, cap)?;
    let doc = modal_env_refused(&mut ledger, &binding, standing, ModalEnvelopeRefusals::Made(diagnostic))?;
    modal_env_envelope(&ledger, &doc, &binding)
}

/// A refused document with one diagnostic within `bound`, from the request
/// alone (before any solve): the binding of `modal_doc_binding(request,
/// params, state)` (no subject census) and one refusal row, sourced
/// `modal`. Its construction peak assumes A2 builds it through
/// `modal_doc_binding` and `modal_doc_refused` with that one diagnostic.
pub(crate) fn modal_env_fallback_refused(
    request: &Value,
    params: &ModalIntegrityParameters,
    standing: ModalParameterStanding<'_>,
    state: Option<(usize, &str)>,
    bound: &ModalEnvelopeDiagnosticBound,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let binding = modal_env_binding(&mut ledger, request, params, state, None, cap)?;
    let doc = modal_env_refused(&mut ledger, &binding, standing, ModalEnvelopeRefusals::Bounded(bound))?;
    modal_env_envelope(&ledger, &doc, &binding)
}

/// The solved branch (`document.rs:520-538`).
pub(crate) fn modal_env_solved(
    outcome: &ModalOutcome,
    request: &Value,
    params: &ModalIntegrityParameters,
    standing: ModalParameterStanding<'_>,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let state = outcome.state_case.as_ref().map(|case| (case.case_index, case.case_id.as_str()));
    let census = modal_env_subject_census(&mut ledger, outcome)?;
    let binding = modal_env_binding(&mut ledger, request, params, state, Some(census), cap)?;
    let doc = modal_env_solved_document(&mut ledger, outcome, &binding, standing)?;
    modal_env_envelope(&ledger, &doc, &binding)
}

/// `modal_doc_solved(outcome, binding, standing)` for a given binding value.
pub(crate) fn modal_env_solved_with_binding(
    outcome: &ModalOutcome,
    binding: &Value,
    standing: ModalParameterStanding<'_>,
    cap: u64,
) -> ModalEnvelopeResult<ModalDocumentEnvelope> {
    let mut ledger = ModalEnvelopeLedger::default();
    let binding = modal_env_value(binding, cap)?;
    let doc = modal_env_solved_document(&mut ledger, outcome, &binding, standing)?;
    modal_env_envelope(&ledger, &doc, &binding)
}

// ---------------------------------------------------------------------------
// The solved document (`document.rs:553-898`).

fn modal_env_solved_document(
    ledger: &mut ModalEnvelopeLedger,
    outcome: &ModalOutcome,
    binding: &ModalDocumentTally,
    standing: ModalParameterStanding<'_>,
) -> ModalEnvelopeResult<ModalDocumentTally> {
    let ids = outcome.node_ids.as_slice();
    let dofs = ids.len().checked_mul(6).ok_or(ModalEnvelopeError::Overflow { term: "global dofs", left: modal_env_count(ids.len()), right: 6 })?;
    ledger.global_dofs = modal_env_count(dofs);
    let mut doc = modal_env_identity(ledger, binding, standing)?;
    doc.modal_env_member("status", modal_env_str("solved")?)?;
    let fixed = modal_env_fixed;
    let s = modal_env_str;

    // The basis and the request quantities (`:563-584`).
    let state = match &outcome.state_case {
        None => MODAL_ENV_NULL,
        Some(case) => {
            let linear = outcome.nonlinear_state.as_ref();
            let mut out = ModalEnvelopeObject::default();
            out.modal_env_member("case_id", s(&case.case_id)?)?;
            out.modal_env_member("case_index", modal_env_index(case.case_index))?;
            out.modal_env_member("linearized", modal_env_bool(linear.is_some()))?;
            out.modal_env_member("solver_mode", match linear {
                Some(state) => s(state.solver_mode.as_str())?,
                None => MODAL_ENV_NULL,
            })?;
            let refs = match linear {
                None => MODAL_ENV_NULL,
                Some(state) => {
                    let mut refs = ModalEnvelopeArray::default();
                    for &dof in &state.restrained_dofs {
                        refs.modal_env_element(modal_env_string(modal_env_dof(ids, dof)?)?)?;
                    }
                    refs.modal_env_array()?
                }
            };
            out.modal_env_member("restrained_dof_refs", refs)?;
            out.modal_env_object()?
        }
    };
    let center = match outcome.mass_summary.center_of_gravity {
        None => MODAL_ENV_NULL,
        Some(c) => {
            let mut out = ModalEnvelopeObject::default();
            out.modal_env_member("x", fixed("basis_mass_center", c[0])?)?;
            out.modal_env_member("y", fixed("basis_mass_center", c[1])?)?;
            out.modal_env_member("z", fixed("basis_mass_center", c[2])?)?;
            out.modal_env_object()?
        }
    };
    let mut basis = ModalEnvelopeObject::default();
    basis.modal_env_member("route", s(if outcome.exact_route { "exact_pressure" } else { "pressure_free" })?)?;
    basis.modal_env_member("no_translation_threshold", fixed("basis_no_translation_threshold", outcome.no_translation_threshold)?)?;
    basis.modal_env_member("bounding_box_diagonal", fixed("basis_bounding_box_diagonal", outcome.bounding_box_diagonal)?)?;
    basis.modal_env_member("weight_to_mass_acceleration", match outcome.weight_to_mass_acceleration {
        Some(a) => fixed("request_weight_to_mass_acceleration", a)?,
        None => MODAL_ENV_NULL,
    })?;
    basis.modal_env_member("identity_acceleration", fixed("identity_acceleration", outcome.identity.acceleration)?)?;
    basis.modal_env_member("mass_center", center)?;
    basis.modal_env_member("state_case", state)?;
    ledger.modal_env_header(&mut doc, "basis", basis.modal_env_object()?)?;

    // The extraction (`:585-612`).
    let (mode_count, cutoff) = match outcome.bound {
        ExtractionBound::ModeCount(n) => (Some(n), None),
        ExtractionBound::FrequencyCutoff(f) => (None, Some(f)),
        ExtractionBound::Both { mode_count, frequency_cutoff } => (Some(mode_count), Some(frequency_cutoff)),
    };
    let mut endpoint = ModalEnvelopeObject::default();
    match outcome.completed.endpoint {
        ReportedEndpoint::BelowComputedCluster { next_mode } => {
            endpoint.modal_env_member("kind", s("below_computed_cluster")?)?;
            endpoint.modal_env_member("next_computed_ref", modal_env_string(modal_env_ref("computed:", modal_env_next(next_mode)?)?)?)?;
            endpoint.modal_env_member("established_finite_mode_count", MODAL_ENV_NULL)?;
        }
        ReportedEndpoint::BelowLowestMode => {
            endpoint.modal_env_member("kind", s("below_lowest_mode")?)?;
            endpoint.modal_env_member("next_computed_ref", modal_env_string(modal_env_ref("computed:", 1)?)?)?;
            endpoint.modal_env_member("established_finite_mode_count", MODAL_ENV_NULL)?;
        }
        ReportedEndpoint::WholeFiniteSpectrum { established_finite_mode_count } => {
            endpoint.modal_env_member("kind", s("whole_finite_spectrum")?)?;
            endpoint.modal_env_member("next_computed_ref", MODAL_ENV_NULL)?;
            endpoint.modal_env_member("established_finite_mode_count", modal_env_index(established_finite_mode_count))?;
        }
    }
    let stopped_by = match outcome.reported.stopped_by {
        StoppingBound::ModeCount => "mode_count",
        StoppingBound::FrequencyCutoff => "frequency_cutoff",
        StoppingBound::ModeCountAndFrequencyCutoff => "mode_count_and_frequency_cutoff",
        StoppingBound::SpectrumExhausted => "spectrum_exhausted",
    };
    let mut extraction = ModalEnvelopeObject::default();
    extraction.modal_env_member("mode_count", mode_count.map_or(MODAL_ENV_NULL, modal_env_index))?;
    extraction.modal_env_member("frequency_cutoff", match cutoff {
        Some(f) => fixed("request_frequency_cutoff", f)?,
        None => MODAL_ENV_NULL,
    })?;
    extraction.modal_env_member("stopped_by", s(stopped_by)?)?;
    extraction.modal_env_member("bounded_count", modal_env_index(outcome.reported.bounded_count))?;
    extraction.modal_env_member("extension_count", modal_env_index(outcome.reported.extension_count))?;
    extraction.modal_env_member("count", modal_env_index(outcome.reported.count))?;
    extraction.modal_env_member("computed_count", modal_env_index(outcome.integrity.computed.len()))?;
    extraction.modal_env_member("endpoint", endpoint.modal_env_object()?)?;
    ledger.modal_env_header(&mut doc, "extraction", extraction.modal_env_object()?)?;

    // The census and the standing (`:613-624`).
    let mut census = ModalEnvelopeObject::default();
    match &outcome.integrity.census {
        FiniteCensus::Established { finite_mode_count, structurally_massless } => {
            census.modal_env_member("kind", s("established")?)?;
            census.modal_env_member("finite_mode_count", modal_env_index(*finite_mode_count))?;
            census.modal_env_member("structurally_massless", modal_env_index(*structurally_massless))?;
            census.modal_env_member("nonpositive_mu", MODAL_ENV_NULL)?;
            census.modal_env_member("rejected_mu", MODAL_ENV_NULL)?;
            census.modal_env_member("mass_block_established", MODAL_ENV_NULL)?;
        }
        FiniteCensus::Unresolved { structurally_massless, nonpositive_mu, rejected_mu, mass_block_established } => {
            census.modal_env_member("kind", s("unresolved")?)?;
            census.modal_env_member("finite_mode_count", MODAL_ENV_NULL)?;
            census.modal_env_member("structurally_massless", modal_env_index(*structurally_massless))?;
            census.modal_env_member("nonpositive_mu", modal_env_index(*nonpositive_mu))?;
            census.modal_env_member("rejected_mu", modal_env_index(*rejected_mu))?;
            census.modal_env_member("mass_block_established", modal_env_bool(*mass_block_established))?;
        }
    }
    ledger.modal_env_header(&mut doc, "census", census.modal_env_object()?)?;
    ledger.modal_env_header(&mut doc, "standing", modal_env_standing(&outcome.standing)?)?;

    // Check 4's header (`:626-645`).
    let sturm = &outcome.integrity.sturm;
    let angular = |v: f64| modal_env_quantity("placement_circular_frequency", "fixed", modal_env_angular(v), false, None);
    let mut placement = ModalEnvelopeObject::default();
    match sturm.evidence.placement {
        Placement::Given => {
            placement.modal_env_member("kind", s("given")?)?;
            placement.modal_env_member("lower", MODAL_ENV_NULL)?;
            placement.modal_env_member("upper", MODAL_ENV_NULL)?;
            placement.modal_env_member("established_finite_mode_count", MODAL_ENV_NULL)?;
        }
        Placement::Gap { lower, upper } => {
            placement.modal_env_member("kind", s("gap")?)?;
            placement.modal_env_member("lower", angular(lower)?)?;
            placement.modal_env_member("upper", angular(upper)?)?;
            placement.modal_env_member("established_finite_mode_count", MODAL_ENV_NULL)?;
        }
        Placement::AboveSpectrum { top_eigenvalue, established_finite_mode_count } => {
            placement.modal_env_member("kind", s("above_spectrum")?)?;
            placement.modal_env_member("lower", angular(top_eigenvalue)?)?;
            placement.modal_env_member("upper", MODAL_ENV_NULL)?;
            placement.modal_env_member("established_finite_mode_count", modal_env_index(established_finite_mode_count))?;
        }
    }
    let mut header = ModalEnvelopeObject::default();
    header.modal_env_member("count", modal_env_index(sturm.count))?;
    header.modal_env_member("reported", modal_env_index(sturm.reported))?;
    header.modal_env_member("placement", placement.modal_env_object()?)?;
    header.modal_env_member("conditioning_multiple", fixed("sturm_premise_parameter", sturm.evidence.conditioning_multiple)?)?;
    header.modal_env_member("conditioning_limit", fixed("sturm_premise_parameter", sturm.evidence.conditioning_limit)?)?;
    header.modal_env_member("separation_multiple", fixed("sturm_premise_parameter", sturm.evidence.separation_multiple)?)?;
    header.modal_env_member("growth_limit", fixed("sturm_premise_parameter", sturm.evidence.growth_limit)?)?;
    ledger.modal_env_header(&mut doc, "sturm", header.modal_env_object()?)?;

    // Orthogonality, formation and the guard (`:646-678`).
    let orthogonality = &outcome.integrity.orthogonality;
    let pair = match orthogonality.pair {
        None => MODAL_ENV_NULL,
        Some((a, b)) => {
            let mut pair = ModalEnvelopeArray::default();
            pair.modal_env_element(modal_env_string(modal_env_ref("mode:", modal_env_next(a)?)?)?)?;
            pair.modal_env_element(modal_env_string(modal_env_ref("mode:", modal_env_next(b)?)?)?)?;
            pair.modal_env_array()?
        }
    };
    let mut ortho = ModalEnvelopeObject::default();
    ortho.modal_env_member(
        "max_ratio",
        modal_env_quantity("orthogonality_max_ratio", "fixed", Some(orthogonality.max_ratio), orthogonality.max_ratio_below_normal_range, None)?,
    )?;
    ortho.modal_env_member("pair", pair)?;
    ledger.modal_env_header(&mut doc, "orthogonality", ortho.modal_env_object()?)?;
    let formation = &outcome.integrity.formation;
    let free_ref = |row: usize| -> ModalEnvelopeResult<ModalDocumentTally> {
        match outcome.free_dofs.get(row) {
            Some(&dof) => modal_env_string(modal_env_dof(ids, dof)?),
            None => Ok(MODAL_ENV_NULL),
        }
    };
    let mut form = ModalEnvelopeObject::default();
    form.modal_env_member("stiffness_difference", fixed("stiffness_formation_difference", formation.stiffness.norm)?)?;
    form.modal_env_member("stiffness_row_ref", free_ref(formation.stiffness.row)?)?;
    form.modal_env_member("stiffness_off_pattern", modal_env_index(formation.stiffness.off_pattern))?;
    form.modal_env_member("mass_difference", fixed("mass_formation_difference", formation.mass.norm)?)?;
    form.modal_env_member("mass_row_ref", free_ref(formation.mass.row)?)?;
    form.modal_env_member("mass_off_pattern", modal_env_index(formation.mass.off_pattern))?;
    form.modal_env_member("reformed_stiffness_norm", fixed("reformed_stiffness_norm", formation.scaling.reformed_stiffness_norm)?)?;
    form.modal_env_member("reformed_mass_norm", fixed("reformed_mass_norm", formation.scaling.reformed_mass_norm)?)?;
    form.modal_env_member("section", match formation.section {
        None => MODAL_ENV_NULL,
        Some(section) => {
            let mut out = ModalEnvelopeObject::default();
            out.modal_env_member("area", fixed("section_derivation_difference", section.area)?)?;
            out.modal_env_member("second_moment", fixed("section_derivation_difference", section.second_moment)?)?;
            out.modal_env_member("torsion_constant", fixed("section_derivation_difference", section.torsion_constant)?)?;
            out.modal_env_object()?
        }
    })?;
    ledger.modal_env_header(&mut doc, "formation", form.modal_env_object()?)?;
    let guard = &outcome.guard;
    // `u128::to_string` (`:675-676`): its decimal digits, as a string.
    let wide = |v: u128| {
        let digits = modal_env_digits_wide(v);
        modal_env_string(ModalEnvelopeText { raw: digits, escaped: digits })
    };
    let mut guard_out = ModalEnvelopeObject::default();
    guard_out.modal_env_member("global_dimension", modal_env_index(guard.global_dimension))?;
    guard_out.modal_env_member("free_dimension", modal_env_index(guard.free_dimension))?;
    guard_out.modal_env_member("static_estimate_bytes", wide(guard.static_estimate_bytes)?)?;
    guard_out.modal_env_member("ceiling_bytes", wide(guard.ceiling_bytes)?)?;
    guard_out.modal_env_member("eigen_core_bytes", wide(guard.eigen_core_bytes)?)?;
    guard_out.modal_env_member("operation_estimate", wide(guard.operation_estimate)?)?;
    guard_out.modal_env_member("statement", s("stated formulas, not measurements; no work guard is applied (O-4)")?)?;
    ledger.modal_env_header(&mut doc, "guard", guard_out.modal_env_object()?)?;

    let mut tables = ModalEnvelopeObject::default();
    modal_env_model_tables(ledger, &mut tables, outcome, dofs)?;
    modal_env_mode_tables(ledger, &mut tables, outcome, dofs)?;
    modal_env_evidence_tables(ledger, &mut tables, outcome)?;
    doc.modal_env_member("tables", tables.modal_env_object()?)?;
    doc.modal_env_object()
}

/// The free index of every global DOF in turn, as `free_index`
/// (`document.rs:556-559`) holds it: the last free position naming it.
struct ModalEnvelopeFreeIndex<'a> {
    free: &'a [usize],
    ascending: bool,
    cursor: usize,
}

impl<'a> ModalEnvelopeFreeIndex<'a> {
    fn modal_env_free_index(free: &'a [usize], dofs: usize) -> ModalEnvelopeResult<Self> {
        let mut ascending = true;
        for (i, &dof) in free.iter().enumerate() {
            if dof >= dofs {
                return Err(ModalEnvelopeError::Unprojectable { what: "free degree of freedom past the nodes" });
            }
            if i > 0 && free[i - 1] >= dof {
                ascending = false;
            }
        }
        Ok(Self { free, ascending, cursor: 0 })
    }

    /// The entry of `dof`, asked in ascending order from 0.
    fn modal_env_free_of(&mut self, dof: usize) -> Option<usize> {
        if self.ascending {
            if self.free.get(self.cursor) == Some(&dof) {
                self.cursor += 1;
                return Some(self.cursor - 1);
            }
            return None;
        }
        self.free.iter().rposition(|&d| d == dof)
    }
}

/// Nodes, DOFs, the formed mass and its totals (`document.rs:680-725`).
fn modal_env_model_tables(
    ledger: &mut ModalEnvelopeLedger,
    tables: &mut ModalEnvelopeObject,
    outcome: &ModalOutcome,
    dofs: usize,
) -> ModalEnvelopeResult<()> {
    let ids = outcome.node_ids.as_slice();
    let fixed = modal_env_fixed;
    let mut table = ModalEnvelopeTable::default();
    for (i, id) in ids.iter().enumerate() {
        let c = outcome.node_coordinates.get(i).ok_or(ModalEnvelopeError::Unprojectable { what: "node coordinates" })?;
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("node_id", modal_env_str(id)?)?;
        fields.modal_env_member("x", fixed("node_coordinate", c[0])?)?;
        fields.modal_env_member("y", fixed("node_coordinate", c[1])?)?;
        fields.modal_env_member("z", fixed("node_coordinate", c[2])?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_node(id)?, modal_env_count(i))?;
    }
    tables.modal_env_member("nodes", ledger.modal_env_publish(&table)?)?;

    let mut free = ModalEnvelopeFreeIndex::modal_env_free_index(&outcome.free_dofs, dofs)?;
    let mut table = ModalEnvelopeTable::default();
    for dof in 0..dofs {
        let node = ids.get(dof / 6).ok_or(ModalEnvelopeError::Unprojectable { what: "degree of freedom past the nodes" })?;
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("node_ref", modal_env_string(modal_env_node(node)?)?)?;
        fields.modal_env_member("component", modal_env_str(MODAL_ENV_COMPONENT)?)?;
        fields.modal_env_member("free_index", free.modal_env_free_of(dof).map_or(MODAL_ENV_NULL, modal_env_index))?;
        ledger.modal_env_seal(&mut table, fields, modal_env_dof(ids, dof)?, modal_env_count(dof))?;
    }
    tables.modal_env_member("dofs", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for (i, &(row, col, value)) in outcome.mass_upper_entries.iter().enumerate() {
        let pair = match (row % 6 < 3, col % 6 < 3) {
            (true, true) => "translation_translation",
            (false, false) => "rotation_rotation",
            _ => "translation_rotation",
        };
        let (a, b) = (modal_env_dof(ids, row)?, modal_env_dof(ids, col)?);
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("row_dof_ref", modal_env_string(a)?)?;
        fields.modal_env_member("col_dof_ref", modal_env_string(b)?)?;
        fields.modal_env_member("pair_class", modal_env_str(pair)?)?;
        fields.modal_env_member("value", modal_env_quantity("mass_matrix_entry", pair, Some(value), false, None)?)?;
        let id = modal_env_text_join(modal_env_text_join(modal_env_ascii("mass:"), a)?, modal_env_text_join(modal_env_ascii("|"), b)?)?;
        ledger.modal_env_seal(&mut table, fields, id, modal_env_count(i))?;
    }
    tables.modal_env_member("mass_matrix", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for k in 0..3 {
        let mut basis_ref = ModalEnvelopeObject::default();
        basis_ref.modal_env_member("ref_type", modal_env_str("model")?)?;
        basis_ref.modal_env_member("ref_id", modal_env_str("model")?)?;
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("direction", modal_env_str(MODAL_ENV_DIRECTION)?)?;
        fields.modal_env_member("basis_ref", basis_ref.modal_env_object()?)?;
        fields.modal_env_member("total_mass", fixed("modal_total_mass", outcome.mass_summary.total_by_direction[k])?)?;
        fields.modal_env_member("free_dof_mass", fixed("modal_free_dof_mass", outcome.free_dof_mass[k])?)?;
        let id = modal_env_text_join(modal_env_ascii("mass-total:"), modal_env_ascii(MODAL_ENV_DIRECTION))?;
        ledger.modal_env_seal(&mut table, fields, id, modal_env_count(k))?;
    }
    tables.modal_env_member("mass_totals", ledger.modal_env_publish(&table)?)
}

/// The reported modes, their shapes and participation (`document.rs:727-778`).
fn modal_env_mode_tables(
    ledger: &mut ModalEnvelopeLedger,
    tables: &mut ModalEnvelopeObject,
    outcome: &ModalOutcome,
    dofs: usize,
) -> ModalEnvelopeResult<()> {
    let ids = outcome.node_ids.as_slice();
    let (add, mul) = (modal_env_add, modal_env_mul);
    let (mut modes, mut shapes, mut participation) = (ModalEnvelopeTable::default(), ModalEnvelopeTable::default(), ModalEnvelopeTable::default());
    for mode in &outcome.modes {
        let case = match mode.normalization {
            NormalizationCase::UnitMaxTranslation => MODAL_ENV_CASES[0],
            NormalizationCase::UnitMaxRotation => MODAL_ENV_CASES[1],
        };
        let index = modal_env_count(mode.index);
        let ordinal = index.checked_sub(1).ok_or(ModalEnvelopeError::Unprojectable { what: "mode index 0" })?;
        let mode_ref = modal_env_ref("mode:", index)?;
        let quantity = |kind: &str, value: f64, below: bool| modal_env_quantity(kind, "fixed", Some(value), below, None);
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("mode_index", modal_env_unsigned(index))?;
        fields.modal_env_member("computed_ref", modal_env_string(modal_env_ref("computed:", index)?)?)?;
        fields.modal_env_member("cluster_ref", modal_env_string(modal_env_ref("cluster:", modal_env_next(mode.cluster)?)?)?)?;
        fields.modal_env_member("normalization_case", modal_env_str(case)?)?;
        fields.modal_env_member("reference_dof_ref", modal_env_string(modal_env_dof(ids, mode.reference_dof)?)?)?;
        fields.modal_env_member("standing", modal_env_standing(&mode.standing)?)?;
        fields.modal_env_member("circular_frequency", quantity("modal_circular_frequency", mode.circular_frequency, false)?)?;
        fields.modal_env_member("frequency", quantity("modal_frequency", mode.frequency, false)?)?;
        fields.modal_env_member("period", quantity("modal_period", mode.period, false)?)?;
        fields.modal_env_member(
            "generalized_mass",
            modal_env_quantity("modal_generalized_mass", case, Some(mode.generalized_mass), mode.below_normal_range.generalized_mass, None)?,
        )?;
        fields.modal_env_member("cluster_error_estimate", quantity("modal_cluster_error_estimate", mode.cluster_error_estimate, false)?)?;
        ledger.modal_env_seal(&mut modes, fields, mode_ref, ordinal)?;

        let first = mul("shape ordinal", ordinal, modal_env_count(dofs))?;
        let shape_prefix = modal_env_text_join(modal_env_text_join(modal_env_ascii("shape:"), mode_ref)?, modal_env_ascii(":"))?;
        for dof in 0..dofs {
            let kind = if dof % 6 < 3 { "mode_shape_translation" } else { "mode_shape_rotation" };
            let below = mode.below_normal_range.shape.contains(&dof);
            let value = *mode.shape.get(dof).ok_or(ModalEnvelopeError::Unprojectable { what: "shape shorter than the degrees of freedom" })?;
            let id = modal_env_dof(ids, dof)?;
            let mut fields = ModalEnvelopeObject::default();
            fields.modal_env_member("mode_ref", modal_env_string(mode_ref)?)?;
            fields.modal_env_member("dof_ref", modal_env_string(id)?)?;
            fields.modal_env_member("component", modal_env_str(MODAL_ENV_COMPONENT)?)?;
            fields.modal_env_member("value", modal_env_quantity(kind, case, Some(value), below, None)?)?;
            ledger.modal_env_seal(&mut shapes, fields, modal_env_text_join(shape_prefix, id)?, add("shape ordinal", first, modal_env_count(dof))?)?;
        }

        let below = &mode.below_normal_range;
        let participation_prefix = modal_env_text_join(modal_env_text_join(modal_env_ascii("participation:"), mode_ref)?, modal_env_ascii(":"))?;
        for k in 0..3 {
            let fraction = match mode.effective_mass_fraction[k] {
                Some(f) => modal_env_quantity("modal_effective_mass_fraction", "fixed", Some(f), below.effective_mass_fraction[k], None)?,
                None => modal_env_quantity("modal_effective_mass_fraction", "fixed", None, false, Some("total_mass_zero"))?,
            };
            let cumulative = match mode.cumulative_effective_mass_fraction[k] {
                Some(f) => modal_env_quantity("modal_cumulative_effective_mass_fraction", "fixed", Some(f), false, None)?,
                None => modal_env_quantity("modal_cumulative_effective_mass_fraction", "fixed", None, false, Some("total_mass_zero"))?,
            };
            let mut fields = ModalEnvelopeObject::default();
            fields.modal_env_member("mode_ref", modal_env_string(mode_ref)?)?;
            fields.modal_env_member("direction", modal_env_str(MODAL_ENV_DIRECTION)?)?;
            fields.modal_env_member(
                "participation_factor",
                modal_env_quantity("modal_participation_factor", case, Some(mode.participation[k]), below.participation[k], None)?,
            )?;
            fields.modal_env_member(
                "effective_mass",
                modal_env_quantity("modal_effective_mass", "fixed", Some(mode.effective_mass[k]), below.effective_mass[k], None)?,
            )?;
            fields.modal_env_member("effective_mass_fraction", fraction)?;
            fields.modal_env_member("cumulative_effective_mass_fraction", cumulative)?;
            let id = modal_env_text_join(participation_prefix, modal_env_ascii(MODAL_ENV_DIRECTION))?;
            ledger.modal_env_seal(&mut participation, fields, id, add("participation ordinal", mul("participation ordinal", ordinal, 3)?, modal_env_count(k))?)?;
        }
    }
    tables.modal_env_member("modes", ledger.modal_env_publish(&modes)?)?;
    tables.modal_env_member("shapes", ledger.modal_env_publish(&shapes)?)?;
    tables.modal_env_member("participation", ledger.modal_env_publish(&participation)?)
}

/// Clusters, computed modes, Sturm candidates, identity, supports,
/// discretization, disclosures and diagnostics (`document.rs:780-895`).
fn modal_env_evidence_tables(ledger: &mut ModalEnvelopeLedger, tables: &mut ModalEnvelopeObject, outcome: &ModalOutcome) -> ModalEnvelopeResult<()> {
    let ids = outcome.node_ids.as_slice();
    let (fixed, s) = (modal_env_fixed, modal_env_str);

    let mut table = ModalEnvelopeTable::default();
    for c in &outcome.clusters {
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("cluster_index", modal_env_unsigned(modal_env_next(c.id)?))?;
        fields.modal_env_member("first_computed_ref", modal_env_string(modal_env_ref("computed:", modal_env_next(c.modes.start)?)?)?)?;
        fields.modal_env_member("last_computed_ref", modal_env_string(modal_env_ref("computed:", modal_env_count(c.modes.end))?)?)?;
        fields.modal_env_member("reported", modal_env_bool(c.reported))?;
        fields.modal_env_member("error_estimate", fixed("cluster_error_estimate", c.error_estimate)?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_ref("cluster:", modal_env_next(c.id)?)?, modal_env_count(c.id))?;
    }
    tables.modal_env_member("clusters", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for c in &outcome.integrity.computed {
        let (f, omega, _) = frequency_from_eigenvalue(c.eigenvalue);
        let next = modal_env_next(c.index)?;
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("computed_index", modal_env_unsigned(next))?;
        fields.modal_env_member("cluster_ref", modal_env_string(modal_env_ref("cluster:", modal_env_next(c.cluster)?)?)?)?;
        fields.modal_env_member("reported", modal_env_bool(c.reported))?;
        fields.modal_env_member("mode_ref", if c.reported { modal_env_string(modal_env_ref("mode:", next)?)? } else { MODAL_ENV_NULL })?;
        fields.modal_env_member("band_recovered", modal_env_bool(c.band_recovered))?;
        fields.modal_env_member("circular_frequency", fixed("computed_circular_frequency", omega)?)?;
        fields.modal_env_member("frequency", fixed("computed_frequency", f)?)?;
        fields.modal_env_member("backward_error", fixed("residual_backward_error", c.residual.backward_error)?)?;
        fields.modal_env_member("condition", fixed("eigenvalue_condition_estimate", c.residual.condition)?)?;
        fields.modal_env_member("error_estimate", fixed("eigenvalue_error_estimate", c.residual.error_estimate)?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_ref("computed:", next)?, modal_env_count(c.index))?;
    }
    tables.modal_env_member("computed_modes", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for (j, c) in outcome.integrity.sturm.evidence.candidates.iter().enumerate() {
        let f = c.factored.as_ref();
        let factored = |kind: &str, value: Option<f64>| modal_env_optional(kind, value, "not_factored");
        let decision = match &c.decision {
            ShiftDecision::Accepted => "accepted",
            ShiftDecision::OutsideGap => "outside_gap",
            ShiftDecision::OutOfRange(_) => "out_of_range",
            ShiftDecision::SeparationOutOfRange { .. } => "separation_out_of_range",
            ShiftDecision::FactorizationRejected(_) => "factorization_rejected",
            ShiftDecision::Unseparated => "unseparated",
            ShiftDecision::GrowthLimit => "growth_limit",
            ShiftDecision::IllConditioned => "ill_conditioned",
        };
        let next = modal_env_next(j)?;
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("candidate_index", modal_env_unsigned(next))?;
        fields.modal_env_member("decision", s(decision)?)?;
        fields.modal_env_member("accepted", modal_env_bool(c.decision == ShiftDecision::Accepted))?;
        fields.modal_env_member("negative_pivot_count", f.map_or(MODAL_ENV_NULL, |f| modal_env_index(f.negative_pivot_count)))?;
        fields.modal_env_member("shift", modal_env_quantity("sturm_shift_circular_frequency", "fixed", modal_env_angular(c.sigma), false, None)?)?;
        fields.modal_env_member("formation_term", modal_env_optional("sturm_formation_term", c.formation_term, "not_formed")?)?;
        fields.modal_env_member(
            "formation_separation_ratio",
            modal_env_optional("sturm_formation_separation_ratio", c.formation_separation_ratio, "no_neighbour_or_not_formed")?,
        )?;
        fields.modal_env_member("factor_term", factored("sturm_factor_term", f.map(|f| f.factor_term))?)?;
        fields.modal_env_member("backward_error", factored("sturm_backward_error", f.map(|f| f.backward_error))?)?;
        fields.modal_env_member("inverse_norm_estimate", factored("sturm_inverse_norm_estimate", f.map(|f| f.inverse_norm_estimate))?)?;
        fields.modal_env_member("conditioning_product", factored("sturm_conditioning_product", f.map(|f| f.conditioning_product))?)?;
        fields.modal_env_member("separation_ratio", match f {
            None => modal_env_optional("sturm_separation_ratio", None, "not_factored")?,
            Some(f) => modal_env_optional("sturm_separation_ratio", f.separation_ratio, "no_neighbour")?,
        })?;
        fields.modal_env_member("growth", factored("sturm_growth_factor", f.map(|f| f.growth))?)?;
        fields.modal_env_member("max_abs_lower_factor", factored("sturm_max_abs_lower_factor", f.map(|f| f.max_abs_lower_factor))?)?;
        fields.modal_env_member("max_abs_pivot", factored("sturm_max_abs_pivot", f.map(|f| f.max_abs_pivot))?)?;
        fields.modal_env_member("max_abs_scaled_diagonal", factored("sturm_max_abs_scaled_diagonal", f.map(|f| f.max_abs_scaled_diagonal))?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_ref("sturm-candidate:", next)?, modal_env_count(j))?;
    }
    tables.modal_env_member("sturm_candidates", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for (i, d) in outcome.identity.discrepancies.iter().enumerate() {
        if d.axis >= 3 {
            return Err(ModalEnvelopeError::Unprojectable { what: "identity axis past z" });
        }
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("subject", s(&d.subject)?)?;
        fields.modal_env_member("subject_kind", s(d.subject_kind)?)?;
        fields.modal_env_member("axis", s(MODAL_ENV_DIRECTION)?)?;
        fields.modal_env_member("comparison", s(d.kind)?)?;
        fields.modal_env_member("relative", fixed("identity_relative_discrepancy", d.relative)?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_ref("identity:", modal_env_count(i))?, modal_env_count(i))?;
    }
    tables.modal_env_member("identity", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for (i, support) in outcome.nonlinear_state.iter().flat_map(|state| state.supports.iter()).enumerate() {
        let treatment = match support.treatment {
            ModalSupportTreatment::Restrained => "restrained",
            ModalSupportTreatment::AlreadyRestrained => "already_restrained",
            ModalSupportTreatment::Free => "free",
            ModalSupportTreatment::FreeFrozenFriction => "free_frozen_friction",
        };
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("support_id", s(&support.support_id)?)?;
        fields.modal_env_member("behavior", s(support.behavior)?)?;
        fields.modal_env_member("dof_ref", modal_env_string(modal_env_dof(ids, support.global_dof)?)?)?;
        fields.modal_env_member("state", s(support.state.as_str())?)?;
        fields.modal_env_member("treatment", s(treatment)?)?;
        let id = modal_env_text_join(modal_env_ascii("support:"), modal_env_text(&support.support_id)?)?;
        ledger.modal_env_seal(&mut table, fields, id, modal_env_count(i))?;
    }
    tables.modal_env_member("supports", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    for (i, m) in outcome.discretization.iter().enumerate() {
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("member_ref", s(&m.pipe)?)?;
        fields.modal_env_member("length", fixed("member_length", m.length)?)?;
        fields.modal_env_member("fixed_fixed_frequency", fixed("member_fixed_fixed_frequency", m.fixed_fixed_frequency)?)?;
        fields.modal_env_member("ratio_to_highest_reported", modal_env_optional("member_frequency_ratio", m.ratio_to_highest_reported, "no_reported_mode")?)?;
        let id = modal_env_text_join(modal_env_ascii("member:"), modal_env_text(&m.pipe)?)?;
        ledger.modal_env_seal(&mut table, fields, id, modal_env_count(i))?;
    }
    tables.modal_env_member("discretization", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    let disclosures = outcome.disclosures.iter().map(|t| (t, "modal")).chain(outcome.identity.disclosures.iter().map(|t| (t, "weight_basis")));
    for (i, (text, source)) in disclosures.enumerate() {
        let mut fields = ModalEnvelopeObject::default();
        fields.modal_env_member("text", s(text)?)?;
        fields.modal_env_member("source", s(source)?)?;
        ledger.modal_env_seal(&mut table, fields, modal_env_ref("disclosure:", modal_env_count(i))?, modal_env_count(i))?;
    }
    tables.modal_env_member("disclosures", ledger.modal_env_publish(&table)?)?;

    let mut table = ModalEnvelopeTable::default();
    let diagnostics = outcome
        .diagnostics
        .iter()
        .map(|d| (d, "modal"))
        .chain(outcome.nonlinear_state.iter().flat_map(|state| state.static_diagnostics.iter()).map(|d| (d, "state_case_solve")));
    for (i, (d, source)) in diagnostics.enumerate() {
        modal_env_diagnostic_row(ledger, &mut table, "diagnostic", i, modal_env_typed_fields(d, source)?)?;
    }
    tables.modal_env_member("diagnostics", ledger.modal_env_publish(&table)?)
}
