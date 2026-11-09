//! The host-side working memory of reading a modal result document
//! (DEL-04-09 Design §5.3.8, "Readers"): [`modal_reader_working_bound`].
//!
//! **What is bounded.** The peak, over the sequence below, of the sum of
//! [`mr_envelope`] of every live requested heap allocation (a reallocation
//! counts its old and new blocks together, as the desktop's ingress bound
//! does, `apps/desktop/src-tauri/src/ingress.rs` `allocation_envelope`):
//! 1. the host parses the request text, if any, as
//!    `serde_json::from_str::<Value>` and holds the tree;
//! 2. it calls [`read_modal_result_text`](super::read_modal_result_text) with
//!    the document text and that request, and holds the verdict it returns.
//!
//! The texts themselves (the caller's), the stack, the verdict's JSON form
//! (`modal_verdict_json`) and the allocator's own metadata are outside it.
//! The envelope covers the allocator's size rounding of each block (see
//! `ingress.rs` for what it does and does not cover on this host).
//!
//! **Which inputs.** Any document text and any request text that is one JSON
//! value under serde_json's grammar (UTF-8, no control character in a
//! string, nesting at most 127 containers). For any other text the function
//! returns `None` and the host refuses it unread; a document text that is
//! JSON but not a modal document, or not in the checked profile, or refused,
//! is covered. `None` is also returned when the bound exceeds `u64`.
//!
//! **How.** One allocation-free scan of each text (a fixed stack of 127
//! frames) measures it: the serde_json tree it materializes, the checked
//! parse's key copies, the canonical and serialized lengths, and, by key path,
//! the row counts of each table and the largest row (and the request's model
//! arrays). Every reader allocation is then priced from those measures by the
//! terms below. Each per-item size is the source's, pinned by a `const`
//! assertion in this module. Every growable container is priced at its
//! growth envelope: a `Vec` grown by `push` from empty reaches capacity
//! max(4, next_power_of_two(n)) (one element of at most 1,024 B) and, while
//! growing, holds its previous buffer too ([`mr_vec`]); a hash table holds at
//! most next_power_of_two(max(16, ⌈8n/7⌉)) buckets and, while growing, its
//! previous table ([`mr_hash`]); a `BTreeMap` built by insertion holds at most
//! ⌊n/5⌋ + 1 nodes ([`mr_btree`]). Using the growth envelope everywhere is an
//! upper bound for the vectors allocated exactly.
//!
//! **The formula.** With T(x) the scan's tree bytes of text x:
//!
//! B = max(Q, T(request) + max(P1, P2, T(document) + READ))
//!
//! - Q, the request's parse (`read_modal_result_text` is not yet called):
//!   [`mr_parse`] of the request text: its tree, one array's previous buffer
//!   (`Vec::new` + `push`, serde_json `de.rs` `visit_seq`), the parser's
//!   scratch buffer, the float parser's transient (16 KiB, the ingress
//!   measurement, `lexical` bignums of at most 769 digits) and an error.
//! - P1, the checked profile of the document text (`lib.rs`
//!   `read_modal_result_text`, `canonical_json_checked_v1_text`):
//!   max([`mr_parse`] plus the checked parse's key copies, [`mr_write`]).
//!   The checked parse (`canonical_json/src/lib.rs` `checked_parse`) keeps a
//!   `HashSet<String>` of key copies per open object: the scan takes the
//!   peak over the parse of their sum (`seen_peak`). The canonical writer
//!   (`write_canonical`) holds the tree, its output `String` (at most
//!   max(8, 2C) for canonical length C, plus its previous buffer), one
//!   `sorted_keys` vector of 8·len per open object (the largest root-to-leaf
//!   sum, `keys_path`) and one escaped string (`serde_json::to_string` from
//!   capacity 128) or one number's digits ([`NUMBER_TRANSIENT`]). C is
//!   bounded token by token: a string re-escapes to at most its own text, an
//!   integer to its digits, any other number to at most 26 bytes.
//! - P2, the reader's own parse (`serde_json::from_str`): [`mr_parse`] without
//!   key copies, followed by the read:
//! - READ = PARSED + max(CLOSURE, BINDING, DERIVE, RELATIONS) + RESULT, the
//!   read of the parsed document (`read_modal_result`), solved or refused:
//!   - PARSED ([`parsed_bytes`]): every vector `mr_closure` keeps in `Parsed`,
//!     per table row count, at the pinned element sizes. No string of the
//!     document is copied: ids, kinds, reasons and paths are borrowed or
//!     static (`Vo`, `Num`, `Unitized`, `MrPath`).
//!   - CLOSURE ([`closure_bytes`]): the closure's maps (row ids, node and DOF
//!     indexes, the mass positions) and, in turn, one table's checksum list
//!     and either one row's checksum (`mr_row_checksum`: the members collected
//!     into a vector and sorted, a map of references, its serialization and
//!     [`mr_hash_text`] of it, priced from the largest row's own scan) or the
//!     table digest's (`mr_table`); plus a row's transient strings
//!     ([`transient_strings`]).
//!   - BINDING: the larger of [`mr_hash_text`] of the `binding` member and of
//!     the whole request (`mr_binding` hashes the request, its model,
//!     materials and state case, one at a time), plus the run id's own
//!     two-member object (2,048 B), a unit lookup's error copying one option
//!     string, and the transient strings.
//!   - DERIVE ([`derive_bytes`]): `model_mass::mm_form`, from the request's
//!     model arrays (nodes, pipe segments, components, supports, sections,
//!     load cases, primitive loads, restraint tokens); and FORMED, what of it
//!     `Formed` keeps.
//!   - RELATIONS ([`relations_bytes`]): FORMED, every relation's items
//!     (`RESULT`'s strings, kept in the verdict) and the largest single
//!     relation's transient, `mr_of`'s copies included; the census relation
//!     ([`census_bytes`]) and its subject digest within it.
//!   - RESULT: the verdict's fixed vectors.
//!
//! Each term's coefficients are at its function. **Source lines** priced
//! (as of this module's writing; the function names are the stable anchor):
//! - `lib.rs`: `read_modal_result_text` :543 (checked profile, then
//!   `serde_json::from_str`), `read_modal_result` :577, `mr_vo` :468 and
//!   `mr_unit_check` :520 (no copies), `mr_sha256_text` :710,
//!   `mr_row_checksum` :718, `mr_table` :725, `mr_fields` :767, `mr_join`
//!   :1080, `mr_closure` :1349, `mr_binding` :1972;
//! - `census.rs`: `cs_shape` :67, `cs_subjects` :113, `cs_digest` :261,
//!   `cs_relation` :273;
//! - `model_mass.rs`: `mm_section` :487, `mm_derive` :652, `mm_form` :667,
//!   `mm_states` :1189, `mm_add` :1228, `mm_straight` :1294, `mm_arc`
//!   :1419, `mm_body` :1494, `mm_check` :1567, `mm_mass_relation` :1628,
//!   `mm_participation_relation` :1745;
//! - `core/serialization/canonical_json/src/lib.rs`:
//!   `canonical_json_checked_v1_text` :39, `checked_parse` :45 (its
//!   `HashSet` per object, :78), `write_canonical` :339, `sorted_keys` :386.
use super::model_mass::MM_SIZES;
use super::{Candidate, Computed, IdentityRow, Mode, MrPath, Num, Participation, Reach, SupportRow, Unitized, Vo};
use std::mem::size_of;

type B = u128;

// ---------------------------------------------------------------------------
// Pins: every per-item size the formula uses is the source's.

/// `serde_json::Value` (serde_json 1.0.151 as locked, `float_roundtrip`, no
/// `preserve_order` or `arbitrary_precision`).
const VALUE_BYTES: B = 32;
const _: () = assert!(size_of::<serde_json::Value>() == VALUE_BYTES as usize);
/// A `BTreeMap<String, Value>` node: leaf 8 + 11·(24 + 32) + 4 = 628 → 632,
/// internal 632 + 12·8 = 728 (pinned by measurement in
/// `tests/working_bound.rs`, `btree_node_sizes_are_the_formula`). Every node
/// is priced as the larger.
const MAP_NODE_BYTES: B = 728;
const NUM_BYTES: B = 40;
const _: () = assert!(size_of::<Num<'static>>() == NUM_BYTES as usize);
const _: () = assert!(size_of::<Vo<'static>>() == 40);
const _: () = assert!(size_of::<MrPath>() == 48);
const UNITIZED_BYTES: B = 72;
const _: () = assert!(size_of::<Unitized<'static>>() == UNITIZED_BYTES as usize);
const MODE_BYTES: B = 168;
const _: () = assert!(size_of::<Mode<'static>>() == MODE_BYTES as usize);
const PARTICIPATION3_BYTES: B = 480;
const _: () = assert!(size_of::<[Participation<'static>; 3]>() == PARTICIPATION3_BYTES as usize);
const COMPUTED_BYTES: B = 184;
const _: () = assert!(size_of::<Computed<'static>>() == COMPUTED_BYTES as usize);
const CANDIDATE_BYTES: B = 512;
const _: () = assert!(size_of::<Candidate<'static>>() == CANDIDATE_BYTES as usize);
const IDENTITY_BYTES: B = 96;
const _: () = assert!(size_of::<IdentityRow<'static>>() == IDENTITY_BYTES as usize);
const SUPPORT_BYTES: B = 72;
const _: () = assert!(size_of::<SupportRow<'static>>() == SUPPORT_BYTES as usize);
const MASS_ENTRY_BYTES: B = 56;
const _: () = assert!(size_of::<(usize, usize, Num<'static>)>() == MASS_ENTRY_BYTES as usize);
const CLUSTER_BYTES: B = 64;
const _: () = assert!(size_of::<(usize, usize, bool, Num<'static>)>() == CLUSTER_BYTES as usize);
const REACH_BYTES: B = 32;
const _: () = assert!(size_of::<Reach>() == REACH_BYTES as usize);
const PART_BYTES: B = 24;
const _: () = assert!(size_of::<(f64, f64, bool)>() == PART_BYTES as usize);
const STR_BYTES: B = 16;
const _: () = assert!(size_of::<&str>() == STR_BYTES as usize);
const STRING_BYTES: B = 24;
const _: () = assert!(size_of::<String>() == STRING_BYTES as usize);
/// `model_mass` (`MM_SIZES`): `Member`, an `acc` entry, a `section_mass`
/// entry, a `bodies` entry, an `entries` entry, `Formed`, a `subjects` element.
const MM_MEMBER: B = 144;
const MM_ACC_ENTRY: B = 64;
const MM_SECTION_ENTRY: B = 40;
const MM_BODY_ENTRY: B = 88;
const MM_ENTRY: B = 32;
const MM_FORMED: B = 224;
const MM_SUBJECT: B = 40;
const _: () = assert!(MM_SIZES[0] == MM_MEMBER as usize);
const _: () = assert!(MM_SIZES[1] == MM_ACC_ENTRY as usize);
const _: () = assert!(MM_SIZES[2] == MM_SECTION_ENTRY as usize);
const _: () = assert!(MM_SIZES[3] == MM_BODY_ENTRY as usize);
const _: () = assert!(MM_SIZES[4] == MM_ENTRY as usize);
const _: () = assert!(MM_SIZES[5] == MM_FORMED as usize);
const _: () = assert!(MM_SIZES[6] == MM_SUBJECT as usize);
/// `ModalReadStanding` (status, difference, scope, unresolved).
const STANDING_BYTES: B = 80;
const _: () = assert!(size_of::<super::ModalReadStanding>() == STANDING_BYTES as usize);

// ---------------------------------------------------------------------------
// Allowances (each stated at its use).

/// serde_json's float parser (`float_roundtrip`, `lexical`): its bignum
/// transients on a hard-to-round float, at most 769 digits (`MAX_DIGITS`);
/// the ingress measurement, re-checked by `tests/working_bound.rs` on hard
/// floats.
const FLOAT_ALLOWANCE: B = 16_384;
/// A serde_json error: `Box<ErrorImpl>` and its message (`custom` errors carry
/// the duplicate key, priced separately as a document string).
const ERROR_ALLOWANCE: B = 1_024;
/// One canonical number's transients (`canonical_number`,
/// `ecma_number_to_string`, `shortest_digits`): ryu's digits (≤ 17), the
/// joined digit string, the rendering (≤ 26 B) and an exponent's digits.
const NUMBER_TRANSIENT: B = 256;
/// The hex digest (`mr_sha256_text`): 32 two-digit `format!`s collected into a
/// 64-byte `String` (8 → 16 → 32 → 64): 16 + 32 + 64.
const HEX_BYTES: B = 128;
/// The longest literal text (and numbers) of any message the reader formats:
/// a `format!` of at most this many bytes besides the document and request
/// strings it interpolates.
const MESSAGE_LITERAL: B = 1_024;
/// The literal text of a message formed once per table row or mode (an item
/// such as `unresolved: entry mass:{a}|{b}: undecidable against the
/// model-derived [{:e}, {:e}]`): at most this many bytes besides its node ids.
const ROW_MESSAGE_LITERAL: B = 400;
/// One `Reach` operation chain (`mr_join`, `mr_sum`, `mr_slack`,
/// `mr_quotient`): a join holds at most 8 × 8 = 64 parts (each operand has at
/// most 8 after `mr_new`), 64·24 = 1,536 B in a `Vec` of capacity 64, sorted
/// on the stack (≤ 4 KiB); an expression holds at most a few stored Reaches
/// (≤ 8 parts, ≤ 256 B each) beside it.
const REACH_OPS: B = 16_384;
/// A stored `Reach`'s parts: at most 8 after `mr_new`, in a buffer of at
/// most 8·24 = 192 B (a 9-or-more-part join is collapsed to one part).
const REACH_HEAP: B = 256;
/// A `Reach` from `mr_need` / `mr_reach` / a clone of one: at most two parts
/// (a signed span), 48 B.
const NEED_HEAP: B = 64;
/// Fixed small vectors: `mr_vo`'s and `mr_fields`' key lists, `vos`,
/// `row_set`, a candidate's nine sets, `formation`, `hits`, `relations`.
const SMALL_VECTORS: B = 8_192;
/// `mm_section`'s term vectors and one section's `format!`s besides its id.
const SECTION_SMALL: B = 4_096;

// ---------------------------------------------------------------------------
// Allocation shapes.

/// The allocation envelope of one requested size: max(16, next_power_of_two),
/// 0 for 0 (`ingress.rs` `allocation_envelope`).
fn mr_envelope(size: u128) -> u128 {
    match size {
        0 => 0,
        _ => size.checked_next_power_of_two().unwrap_or(u128::MAX).max(16),
    }
}

fn min_cap(size: B) -> B {
    if size == 1 {
        8
    } else if size <= 1024 {
        4
    } else {
        1
    }
}

/// A `Vec` of `n` elements of `size` bytes grown from empty (`push`,
/// `extend`, `collect` of an inexact iterator): its buffer at capacity
/// max(min, next_power_of_two(n)) and its previous buffer while growing. An
/// exactly allocated vector (`with_capacity`, an exact `collect`, a
/// `try_reserve` from empty) needs no more.
fn mr_vec(n: B, size: B) -> B {
    if n == 0 || size == 0 {
        return 0;
    }
    let cap = n.checked_next_power_of_two().unwrap_or(u128::MAX).max(min_cap(size));
    let previous = if cap > min_cap(size) { mr_envelope((cap / 2).saturating_mul(size)) } else { 0 };
    mr_envelope(cap.saturating_mul(size)).saturating_add(previous)
}

/// A hash table (`HashMap`, `HashSet`) of `n` entries of `size` bytes, by
/// inserts or `reserve(n)`: at most next_power_of_two(max(16, ⌈8n/7⌉))
/// buckets (hashbrown `capacity_to_buckets`), each `size` + 1 control byte,
/// plus 16 trailing control bytes and up to 16 of alignment; with its
/// previous table while it grows.
fn mr_hash(n: B, size: B) -> B {
    if n == 0 {
        return 0;
    }
    let buckets = (n.saturating_mul(8).div_ceil(7)).max(16).checked_next_power_of_two().unwrap_or(u128::MAX);
    let bytes = |b: B| b.saturating_mul(size + 1).saturating_add(32);
    mr_envelope(bytes(buckets)).saturating_add(mr_envelope(bytes(buckets / 2)))
}

/// A `BTreeMap` of `n` entries (key and value of `kv` bytes together) built
/// by insertion: [`btree_nodes`] nodes, each priced as an internal node,
/// 8 + 11·kv + 4 (rounded to 8) + 12·8.
fn mr_btree(n: B, kv: B) -> B {
    let internal = (8 + 11 * kv + 4).div_ceil(8) * 8 + 96;
    btree_nodes(n).saturating_mul(mr_envelope(internal))
}

/// The nodes of a `BTreeMap` of `n` entries built by insertion: none when
/// empty, one leaf up to its capacity of 11, else at most ⌊n/5⌋ + 1 (every
/// split leaves both halves with at least 5 keys, `splitpoint`, and inserts
/// never shrink a node).
fn btree_nodes(n: B) -> B {
    match n {
        0 => 0,
        1..=11 => 1,
        _ => n / 5 + 1,
    }
}

/// One `format!` (or `to_string`) of `len` bytes, kept: `format!`'s
/// capacity is at most twice its length (`estimated_capacity`, then doubling).
fn mr_message(len: B) -> B {
    mr_envelope(len.saturating_mul(2))
}

/// The strings a closure row, a relation step or a model-mass step holds at
/// once: at most eight `format!`s (paths, expected ids, labels, an eagerly
/// formed failure text), each quoting at most two strings of length
/// `quoted`, with their growth.
fn transient_strings(quoted: B) -> B {
    8 * mr_message(MESSAGE_LITERAL + 2 * quoted) + mr_envelope(MESSAGE_LITERAL + 2 * quoted)
}

/// One relation's difference: at most three quoted strings, one of them as
/// `Debug` (≤ 6 bytes per byte: `mr_unit_check`'s `{unit:?}`).
fn mr_difference(quoted: B) -> B {
    mr_message(MESSAGE_LITERAL + 8 * quoted)
}

/// The closure's difference: at most one quoted document string, shown
/// (`mr_one_of`'s `{text}`, an undeclared `{extra}`, a duplicate `{id}`), and
/// a refused document's `replace` copy of it.
fn closure_difference(any: B) -> B {
    2 * mr_message(MESSAGE_LITERAL + any)
}

// ---------------------------------------------------------------------------
// The scan.

/// One subtree's (or text's) measures, as if parsed alone.
#[derive(Clone, Copy, Default, Debug)]
struct Sub {
    /// The serde_json tree: every string and key `mr_envelope(len)`, every
    /// array buffer `mr_envelope(32·cap)`, every object (⌊n/5⌋ + 1) nodes.
    tree: B,
    /// The largest array's previous buffer (one array grows at a time).
    array_transient: B,
    /// The largest scratch requirement: an escaped string's decoded length
    /// + 4, a float-path number's length + 21 (`ingress.rs`).
    scratch: B,
    float: bool,
    /// The checked parse's live key-copy sets, at their peak.
    seen_peak: B,
    /// Bounds on the canonical and the compact serialized lengths.
    canonical: B,
    serialized: B,
    /// The largest root-to-leaf sum of `sorted_keys` buffers.
    keys_path: B,
    /// The longest string token (quotes included) and decoded string.
    longest_token: B,
    longest: B,
    /// The longest key (a parse error quotes at most a duplicate key).
    longest_key: B,
    /// The decoded bytes of every string and key.
    string_bytes: B,
}

impl Sub {
    fn absorb(&mut self, child: &Sub) {
        self.tree = self.tree.saturating_add(child.tree);
        self.array_transient = self.array_transient.max(child.array_transient);
        self.scratch = self.scratch.max(child.scratch);
        self.float |= child.float;
        self.canonical = self.canonical.saturating_add(child.canonical);
        self.serialized = self.serialized.saturating_add(child.serialized);
        self.longest_token = self.longest_token.max(child.longest_token);
        self.longest = self.longest.max(child.longest);
        self.longest_key = self.longest_key.max(child.longest_key);
        self.string_bytes = self.string_bytes.saturating_add(child.string_bytes);
    }
    /// The component-wise maximum (each priced term is monotone in each).
    fn widen(&mut self, other: &Sub) {
        self.tree = self.tree.max(other.tree);
        self.array_transient = self.array_transient.max(other.array_transient);
        self.scratch = self.scratch.max(other.scratch);
        self.float |= other.float;
        self.seen_peak = self.seen_peak.max(other.seen_peak);
        self.canonical = self.canonical.max(other.canonical);
        self.serialized = self.serialized.max(other.serialized);
        self.keys_path = self.keys_path.max(other.keys_path);
        self.longest_token = self.longest_token.max(other.longest_token);
        self.longest = self.longest.max(other.longest);
        self.longest_key = self.longest_key.max(other.longest_key);
        self.string_bytes = self.string_bytes.max(other.string_bytes);
    }
}

#[derive(Clone, Copy)]
enum Seg {
    Key(&'static str),
    Any,
}

use Seg::{Any, Key};

/// The document paths the bound reads: each table's rows (in `TABLES`
/// order), the refused document's rows, the `binding` member, the
/// linearized state's restrained DOF list and the headers whose value
/// objects the relations read.
static DOC_PATTERNS: [&[Seg]; 23] = [
    &[Key("tables"), Key("nodes"), Key("rows"), Any],
    &[Key("tables"), Key("dofs"), Key("rows"), Any],
    &[Key("tables"), Key("mass_matrix"), Key("rows"), Any],
    &[Key("tables"), Key("mass_totals"), Key("rows"), Any],
    &[Key("tables"), Key("modes"), Key("rows"), Any],
    &[Key("tables"), Key("shapes"), Key("rows"), Any],
    &[Key("tables"), Key("participation"), Key("rows"), Any],
    &[Key("tables"), Key("clusters"), Key("rows"), Any],
    &[Key("tables"), Key("computed_modes"), Key("rows"), Any],
    &[Key("tables"), Key("sturm_candidates"), Key("rows"), Any],
    &[Key("tables"), Key("identity"), Key("rows"), Any],
    &[Key("tables"), Key("supports"), Key("rows"), Any],
    &[Key("tables"), Key("discretization"), Key("rows"), Any],
    &[Key("tables"), Key("disclosures"), Key("rows"), Any],
    &[Key("tables"), Key("diagnostics"), Key("rows"), Any],
    &[Key("refusals"), Key("rows"), Any],
    &[Key("binding")],
    &[Key("basis"), Key("state_case"), Key("restrained_dof_refs"), Any],
    &[Key("basis")],
    &[Key("extraction")],
    &[Key("sturm")],
    &[Key("orthogonality")],
    &[Key("formation")],
];
const T_NODES: usize = 0;
const T_DOFS: usize = 1;
const T_MASS: usize = 2;
const T_TOTALS: usize = 3;
const T_MODES: usize = 4;
const T_SHAPES: usize = 5;
const T_PART: usize = 6;
const T_CLUSTERS: usize = 7;
const T_COMPUTED: usize = 8;
const T_CANDIDATES: usize = 9;
const T_IDENTITY: usize = 10;
const T_SUPPORTS: usize = 11;
const T_DISC: usize = 12;
const T_DISCLOSURES: usize = 13;
const T_REFUSALS: usize = 15;
const D_BINDING: usize = 16;
const D_RESTRAINED: usize = 17;
/// The tables (and headers) whose strings a relation's message may quote: a
/// value object's reason, unit or status, a node, member, support or
/// identity subject id. Not the disclosures, diagnostics or refusals.
const QUOTED: [usize; 18] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 18, 19, 20, 21, 22];

/// The request's model arrays `model_mass` and `census` read.
static REQUEST_PATTERNS: [&[Seg]; 8] = [
    &[Key("model"), Key("nodes"), Any],
    &[Key("model"), Key("pipe_segments"), Any],
    &[Key("model"), Key("components"), Any],
    &[Key("model"), Key("supports"), Any],
    &[Key("model"), Key("sections"), Any],
    &[Key("model"), Key("load_cases"), Any],
    &[Key("model"), Key("load_cases"), Any, Key("primitive_loads"), Any],
    &[Key("model"), Key("supports"), Any, Key("restraints"), Any],
];
const Q_NODES: usize = 0;
const Q_PIPES: usize = 1;
const Q_COMPONENTS: usize = 2;
const Q_SUPPORTS: usize = 3;
const Q_SECTIONS: usize = 4;
const Q_PRIMITIVE: usize = 6;
const Q_RESTRAINTS: usize = 7;

/// serde_json starts its recursion budget at 128 and refuses the container
/// that exhausts it: at most 127 containers nest.
const MAX_NESTING: usize = 127;
const MAX_PATTERNS: usize = 23;

/// A scanned text: its measures, and per pattern the number of values at
/// that path, the component-wise largest subtree among them and the largest
/// member count among them.
#[derive(Clone, Copy, Default)]
struct Scanned {
    whole: Sub,
    count: [B; MAX_PATTERNS],
    largest: [Sub; MAX_PATTERNS],
    members: [B; MAX_PATTERNS],
    /// The decoded string bytes of every value at that path.
    strings: [B; MAX_PATTERNS],
}

#[derive(Clone, Copy, Default)]
struct Frame {
    object: bool,
    count: B,
    sub: Sub,
    child_keys_path: B,
    own_seen: B,
    live_seen: B,
    /// Patterns whose prefix matches this container's path.
    mask: u32,
    /// Patterns this container fully matches.
    full: u32,
}

/// A decoded key, kept only up to 32 bytes (no pattern key is longer).
struct KeyBuf {
    bytes: [u8; 32],
    len: usize,
}

impl KeyBuf {
    fn push(&mut self, b: u8) {
        if self.len < 32 {
            self.bytes[self.len] = b;
        }
        self.len += 1;
    }
    fn is(&self, key: &str) -> bool {
        self.len == key.len() && self.len <= 32 && &self.bytes[..self.len] == key.as_bytes()
    }
}

struct Scan<'a> {
    bytes: &'a [u8],
    i: usize,
}

impl Scan<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.i).copied()
    }
    fn next(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.i += 1;
        Some(b)
    }
    fn skip_ws(&mut self) {
        while let Some(b' ' | b'\n' | b'\t' | b'\r') = self.peek() {
            self.i += 1;
        }
    }
    fn hex4(&mut self) -> Option<u32> {
        let mut v = 0u32;
        for _ in 0..4 {
            v = v * 16 + (self.next()? as char).to_digit(16)?;
        }
        Some(v)
    }
    /// A string after its opening quote: (token length with quotes, decoded
    /// length, escaped); `key` collects the decoded bytes.
    fn string(&mut self, mut key: Option<&mut KeyBuf>) -> Option<(B, B, bool)> {
        let start = self.i - 1;
        let (mut decoded, mut escaped) = (0u128, false);
        loop {
            match self.next()? {
                b'"' => break,
                b'\\' => {
                    escaped = true;
                    let simple = match self.next()? {
                        b'"' => Some(b'"'),
                        b'\\' => Some(b'\\'),
                        b'/' => Some(b'/'),
                        b'b' => Some(0x08),
                        b'f' => Some(0x0c),
                        b'n' => Some(b'\n'),
                        b'r' => Some(b'\r'),
                        b't' => Some(b'\t'),
                        b'u' => None,
                        _ => return None,
                    };
                    if let Some(b) = simple {
                        if let Some(k) = key.as_deref_mut() {
                            k.push(b);
                        }
                        decoded += 1;
                        continue;
                    }
                    let first = self.hex4()?;
                    let code = match first {
                        0xDC00..=0xDFFF => return None,
                        0xD800..=0xDBFF => {
                            if self.next()? != b'\\' || self.next()? != b'u' {
                                return None;
                            }
                            let second = self.hex4()?;
                            if !(0xDC00..=0xDFFF).contains(&second) {
                                return None;
                            }
                            (((first - 0xD800) << 10) | (second - 0xDC00)) + 0x1_0000
                        }
                        other => other,
                    };
                    let ch = char::from_u32(code)?;
                    let mut buf = [0u8; 4];
                    for b in ch.encode_utf8(&mut buf).bytes() {
                        if let Some(k) = key.as_deref_mut() {
                            k.push(b);
                        }
                    }
                    decoded += ch.len_utf8() as B;
                }
                0x00..=0x1f => return None,
                other => {
                    if let Some(k) = key.as_deref_mut() {
                        k.push(other);
                    }
                    decoded += 1;
                }
            }
        }
        Some(((self.i - start) as B, decoded, escaped))
    }
    fn digits(&mut self) -> B {
        let mut n = 0;
        while let Some(b'0'..=b'9') = self.peek() {
            self.i += 1;
            n += 1;
        }
        n
    }
    /// A number (serde_json's grammar): (token length, float path).
    fn number(&mut self) -> Option<(B, bool)> {
        let start = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        let integer = match self.next()? {
            b'0' => {
                if let Some(b'0'..=b'9') = self.peek() {
                    return None;
                }
                1
            }
            b'1'..=b'9' => 1 + self.digits(),
            _ => return None,
        };
        let mut float = integer >= 20;
        if self.peek() == Some(b'.') {
            self.i += 1;
            if self.digits() == 0 {
                return None;
            }
            float = true;
        }
        if let Some(b'e' | b'E') = self.peek() {
            self.i += 1;
            if let Some(b'+' | b'-') = self.peek() {
                self.i += 1;
            }
            if self.digits() == 0 {
                return None;
            }
            float = true;
        }
        Some(((self.i - start) as B, float))
    }
    fn literal(&mut self, rest: &[u8]) -> Option<()> {
        for b in rest {
            if self.next()? != *b {
                return None;
            }
        }
        Some(())
    }
}

/// The checked parse's live key set of an object with `keys` members so far:
/// the `HashSet<String>` table and the key copies.
fn seen_bytes(keys: B, copies: B) -> B {
    mr_hash(keys, STRING_BYTES).saturating_add(copies)
}

/// Scans one text (allocation-free): `None` unless it is one JSON value under
/// serde_json's grammar with at most 127 nested containers.
fn mr_scan(text: &str, patterns: &[&[Seg]]) -> Option<Scanned> {
    debug_assert!(patterns.len() <= MAX_PATTERNS);
    let all: u32 = if patterns.is_empty() { 0 } else { u32::MAX >> (32 - patterns.len()) };
    let mut out = Scanned::default();
    let mut s = Scan { bytes: text.as_bytes(), i: 0 };
    let mut stack = [Frame::default(); MAX_NESTING];
    let mut depth = 0usize;
    let mut top = Sub::default();
    // The patterns the next value continues (`mask`) and fully matches (`full`).
    let child_masks = |frame: &Frame, depth: usize, key: Option<&KeyBuf>| -> (u32, u32) {
        let (mut cont, mut full) = (0u32, 0u32);
        for (p, segs) in patterns.iter().enumerate() {
            if frame.mask & (1 << p) == 0 || segs.len() <= depth {
                continue;
            }
            let hit = match (segs[depth], key) {
                (Any, None) => true,
                (Key(k), Some(buf)) => buf.is(k),
                _ => false,
            };
            if hit {
                if segs.len() == depth + 1 {
                    full |= 1 << p;
                } else {
                    cont |= 1 << p;
                }
            }
        }
        (cont, full)
    };
    #[derive(PartialEq)]
    enum Expect {
        Value,
        ValueOrEnd,
        KeyOrEnd,
        Key,
        Colon,
        CommaOrEnd,
        Done,
    }
    let mut expect = Expect::Value;
    // The masks of the value about to start: (continuing, fully matching).
    let mut pending: (u32, u32) = (all, 0);
    loop {
        s.skip_ws();
        let Some(byte) = s.peek() else {
            return if expect == Expect::Done { Some(Scanned { whole: top, ..out }) } else { None };
        };
        // A completed value's measures, to add to its container (or the top).
        let mut done: Option<Sub> = None;
        match expect {
            Expect::Done => return None,
            Expect::Value | Expect::ValueOrEnd => {
                if expect == Expect::ValueOrEnd && byte == b']' {
                    s.i += 1;
                    depth -= 1;
                    done = Some(close(&mut stack, depth, &mut out));
                } else {
                    s.i += 1;
                    let (cont, full) = pending;
                    for p in 0..patterns.len() {
                        if full & (1 << p) != 0 {
                            out.count[p] = out.count[p].saturating_add(1);
                        }
                    }
                    let mut value = Sub::default();
                    match byte {
                        b'{' | b'[' => {
                            if depth == MAX_NESTING {
                                return None;
                            }
                            stack[depth] = Frame { object: byte == b'{', mask: cont, full, ..Frame::default() };
                            depth += 1;
                            expect = if byte == b'{' { Expect::KeyOrEnd } else { Expect::ValueOrEnd };
                            if byte == b'[' {
                                pending = child_masks(&stack[depth - 1], depth - 1, None);
                            }
                            continue;
                        }
                        b'"' => {
                            let (token, decoded, escaped) = s.string(None)?;
                            value.tree = mr_envelope(decoded);
                            value.canonical = token;
                            value.serialized = token;
                            value.longest_token = token;
                            value.longest = decoded;
                            value.string_bytes = decoded;
                            if escaped {
                                value.scratch = decoded + 4;
                            }
                        }
                        b't' | b'n' => {
                            s.literal(if byte == b't' { b"rue" } else { b"ull" })?;
                            value.canonical = 4;
                            value.serialized = 4;
                        }
                        b'f' => {
                            s.literal(b"alse")?;
                            value.canonical = 5;
                            value.serialized = 5;
                        }
                        b'-' | b'0'..=b'9' => {
                            s.i -= 1;
                            let (token, float) = s.number()?;
                            if float {
                                value.float = true;
                                value.scratch = token + 21;
                                value.canonical = 26;
                                value.serialized = 24;
                            } else {
                                value.canonical = token;
                                value.serialized = token;
                            }
                        }
                        _ => return None,
                    }
                    done = Some(value);
                }
            }
            Expect::KeyOrEnd | Expect::Key => {
                s.i += 1;
                if expect == Expect::KeyOrEnd && byte == b'}' {
                    depth -= 1;
                    done = Some(close(&mut stack, depth, &mut out));
                } else if byte == b'"' {
                    let mut key = KeyBuf { bytes: [0; 32], len: 0 };
                    let (token, decoded, escaped) = s.string(Some(&mut key))?;
                    let frame = &mut stack[depth - 1];
                    frame.sub.tree = frame.sub.tree.saturating_add(mr_envelope(decoded));
                    frame.sub.canonical = frame.sub.canonical.saturating_add(token);
                    frame.sub.serialized = frame.sub.serialized.saturating_add(token);
                    frame.sub.longest_token = frame.sub.longest_token.max(token);
                    frame.sub.longest = frame.sub.longest.max(decoded);
                    frame.sub.longest_key = frame.sub.longest_key.max(decoded);
                    frame.sub.string_bytes = frame.sub.string_bytes.saturating_add(decoded);
                    if escaped {
                        frame.sub.scratch = frame.sub.scratch.max(decoded + 4);
                    }
                    // The checked parse inserts a copy of the key before parsing its value.
                    frame.own_seen = frame.own_seen.saturating_add(mr_envelope(decoded));
                    frame.live_seen = seen_bytes(frame.count + 1, frame.own_seen);
                    frame.sub.seen_peak = frame.sub.seen_peak.max(frame.live_seen);
                    let frame = stack[depth - 1];
                    pending = child_masks(&frame, depth - 1, Some(&key));
                    expect = Expect::Colon;
                } else {
                    return None;
                }
            }
            Expect::Colon => {
                s.i += 1;
                if byte != b':' {
                    return None;
                }
                expect = Expect::Value;
            }
            Expect::CommaOrEnd => {
                s.i += 1;
                let object = stack[depth - 1].object;
                match (byte, object) {
                    (b',', true) => expect = Expect::Key,
                    (b',', false) => {
                        expect = Expect::Value;
                        pending = child_masks(&stack[depth - 1], depth - 1, None);
                    }
                    (b'}', true) | (b']', false) => {
                        depth -= 1;
                        done = Some(close(&mut stack, depth, &mut out));
                    }
                    _ => return None,
                }
            }
        }
        if let Some(value) = done {
            if depth == 0 {
                top.absorb(&value);
                top.seen_peak = top.seen_peak.max(value.seen_peak);
                top.keys_path = top.keys_path.max(value.keys_path);
                expect = Expect::Done;
            } else {
                let frame = &mut stack[depth - 1];
                frame.count += 1;
                frame.sub.absorb(&value);
                frame.sub.seen_peak = frame.sub.seen_peak.max(frame.live_seen.saturating_add(value.seen_peak));
                frame.child_keys_path = frame.child_keys_path.max(value.keys_path);
                expect = Expect::CommaOrEnd;
            }
        }
    }
}

/// Closes the container at `stack[depth]`: its own allocations, and the
/// patterns it fully matches.
fn close(stack: &mut [Frame; MAX_NESTING], depth: usize, out: &mut Scanned) -> Sub {
    let frame = stack[depth];
    let mut sub = frame.sub;
    let n = frame.count;
    let separators = 2 + n.saturating_sub(1);
    if frame.object {
        sub.tree = sub.tree.saturating_add(btree_nodes(n).saturating_mul(mr_envelope(MAP_NODE_BYTES)));
        sub.canonical = sub.canonical.saturating_add(separators + n);
        sub.serialized = sub.serialized.saturating_add(separators + n);
        sub.keys_path = mr_envelope(8 * n).saturating_add(frame.child_keys_path);
    } else {
        if n > 0 {
            let cap = n.checked_next_power_of_two().unwrap_or(u128::MAX).max(4);
            sub.tree = sub.tree.saturating_add(mr_envelope(cap.saturating_mul(VALUE_BYTES)));
            if cap > 4 {
                sub.array_transient = sub.array_transient.max(mr_envelope(cap / 2 * VALUE_BYTES));
            }
        }
        sub.canonical = sub.canonical.saturating_add(separators);
        sub.serialized = sub.serialized.saturating_add(separators);
        sub.keys_path = frame.child_keys_path;
    }
    for p in 0..MAX_PATTERNS {
        if frame.full & (1 << p) != 0 {
            out.largest[p].widen(&sub);
            out.strings[p] = out.strings[p].saturating_add(sub.string_bytes);
            if frame.object {
                out.members[p] = out.members[p].max(n);
            }
        }
    }
    sub
}

// ---------------------------------------------------------------------------
// The terms.

/// serde_json's scratch buffer at its largest capacity C ≤ max(8, 2L) for
/// the largest requirement L, plus its previous buffer (`ingress.rs`).
fn scratch_bytes(largest: B) -> B {
    if largest == 0 {
        return 0;
    }
    let cap = largest.saturating_mul(2).max(8);
    mr_envelope(cap).saturating_add(mr_envelope(cap / 2))
}

/// A parse of a text measured by `sub` (`serde_json::from_str`, or the
/// checked parse when `seen`): the tree, one array's previous buffer, the
/// scratch buffer, the float allowance, an error and, for the checked
/// parse, the live key copies. The error's messages (serde_json's, then
/// `checked_parse`'s `format!`s) quote at most a duplicate key.
fn mr_parse(sub: &Sub, seen: bool) -> B {
    let mut total = sub.tree.saturating_add(sub.array_transient).saturating_add(scratch_bytes(sub.scratch));
    if sub.float {
        total = total.saturating_add(FLOAT_ALLOWANCE);
    }
    if seen {
        total = total.saturating_add(sub.seen_peak);
    }
    total.saturating_add(ERROR_ALLOWANCE).saturating_add(4 * mr_message(MESSAGE_LITERAL + sub.longest_key))
}

/// `serde_json::to_string`'s buffer for `len` bytes: from capacity 128,
/// at most max(128, 2·len), plus its previous buffer.
fn mr_to_string(len: B) -> B {
    let cap = len.saturating_mul(2).max(128);
    mr_envelope(cap).saturating_add(mr_envelope(cap / 2))
}

/// The canonical writer over a tree measured by `sub` (`write_canonical`):
/// the tree, the output (capacity ≤ max(8, 2C), plus its previous buffer),
/// the open objects' sorted key lists and one escaped string or number.
fn mr_write(sub: &Sub) -> B {
    let cap = sub.canonical.saturating_mul(2).max(8);
    let output = mr_envelope(cap).saturating_add(mr_envelope(cap / 2));
    sub.tree
        .saturating_add(output)
        .saturating_add(sub.keys_path)
        .saturating_add(mr_to_string(sub.longest_token).max(NUMBER_TRANSIENT))
}

/// [`super::mr_sha256_text`] of the serialization of a value measured by
/// `sub` (`modal_reader_sha256`, `mr_row_checksum`): the serialized text,
/// the checked parse or the canonical writer over it, and the hex digest. A
/// re-serialized float may take serde_json's float path where its source
/// text did not (≤ 24 digits, so a scratch requirement ≤ 45), and its
/// canonical length is bounded as the source's (it depends only on values).
fn mr_hash_text(sub: &Sub) -> B {
    let mut parsed = *sub;
    parsed.scratch = parsed.scratch.max(45);
    parsed.float = true;
    mr_to_string(sub.serialized)
        .saturating_add(mr_parse(&parsed, true).max(mr_write(sub)))
        .saturating_add(HEX_BYTES)
}

/// The document's counts the formula reads.
struct Doc {
    rows: [B; MAX_PATTERNS],
    /// The largest row (any table's or the refusals'), and its member count.
    row: Sub,
    row_members: B,
    binding: Sub,
    restrained: B,
    /// The longest string in a refusal row (its code).
    largest_refusal: B,
    /// The longest string anywhere (a closure difference may quote it).
    longest: B,
    /// The longest a relation's message may quote ([`QUOTED`]), and a node id.
    quoted: B,
    node: B,
    /// The identity, discretization and supports rows' string bytes (the
    /// census's published ids).
    id_bytes: B,
}

/// The request's counts.
#[derive(Default)]
struct Req {
    nodes: B,
    pipes: B,
    components: B,
    supports: B,
    sections: B,
    primitive: B,
    restraints: B,
    largest_pipe: B,
    largest_section: B,
    hash: B,
    longest: B,
    /// The string bytes of the pipe segments, components and supports (the
    /// census's derived ids).
    subject_bytes: B,
}

/// The value objects `mr_closure` keeps in `unitized`: per row of each table,
/// and at most 22 in the headers (basis 7, extraction 1, sturm 6,
/// orthogonality 1, formation 7).
fn unitized_count(r: &[B; MAX_PATTERNS]) -> B {
    3 * r[T_NODES] + r[T_MASS] + 2 * r[T_TOTALS] + 5 * r[T_MODES] + r[T_SHAPES] + 4 * r[T_PART] + r[T_CLUSTERS] + 5 * r[T_COMPUTED]
        + 12 * r[T_CANDIDATES] + r[T_IDENTITY] + 3 * r[T_DISC] + 22
}

/// PARSED (`lib.rs` `Parsed`, `mr_closure`): per table, its vector at the
/// pinned element size.
fn parsed_bytes(d: &Doc) -> B {
    let r = &d.rows;
    // `shapes` is allocated only once its row count is modes × DOFs
    // (`vec![vec![Num; ndof]; n]`, n inner vectors).
    let shapes = if r[T_SHAPES] == r[T_MODES].saturating_mul(r[T_DOFS]) && r[T_SHAPES] > 0 {
        mr_vec(r[T_MODES], 24).saturating_add(r[T_MODES].saturating_mul(mr_envelope(r[T_DOFS].saturating_mul(NUM_BYTES))))
    } else {
        0
    };
    [
        mr_vec(r[T_NODES], 48),                       // nodes: [Option<f64>; 3]
        mr_vec(r[T_NODES], STR_BYTES),                // node_ids
        mr_vec(r[T_SUPPORTS], SUPPORT_BYTES),         // support_rows
        mr_vec(r[T_SUPPORTS], 24),                    // supports: (usize, &str)
        mr_vec(r[T_DOFS], 16),                        // free: Option<usize>
        mr_vec(r[T_MASS], MASS_ENTRY_BYTES),          // mass
        mr_vec(r[T_MODES], MODE_BYTES),               // modes
        shapes,                                       // shapes
        mr_vec(r[T_MODES], PARTICIPATION3_BYTES),     // participation
        mr_vec(r[T_CLUSTERS], CLUSTER_BYTES),         // clusters
        mr_vec(r[T_COMPUTED], COMPUTED_BYTES),        // computed
        mr_vec(r[T_CANDIDATES], CANDIDATE_BYTES),     // candidates
        mr_vec(r[T_IDENTITY], IDENTITY_BYTES),        // identity
        mr_vec(r[T_DISC], STR_BYTES),                 // members
        mr_vec(d.restrained, 8),                      // restrained_refs
        mr_vec(r[T_DISCLOSURES], 2 * STR_BYTES),      // disclosures
        mr_vec(unitized_count(r), UNITIZED_BYTES),    // unitized
        SMALL_VECTORS,                                // formation, vos, row_set
    ]
    .iter()
    .fold(0, |a: B, x| a.saturating_add(*x))
}

/// CLOSURE (`mr_closure`, `mr_table`, `mr_read_refused`): the maps it holds
/// (row ids over every table, node and DOF indexes keyed by the borrowed row
/// id, the mass positions), one table's checksum list (`&str`), and one
/// row's checksum (a `BTreeMap<&str, &Value>` of the row's members and
/// [`mr_hash_text`] of the largest row) or the table's digest (the
/// checksums' array: 66 B each with quotes and comma, a `Value` and a
/// 64-byte string each when parsed back); the row's transient strings and,
/// for a refused document, its codes.
fn closure_bytes(d: &Doc) -> B {
    let r = &d.rows;
    let all_rows = r.iter().take(16).fold(0, |a: B, x| a.saturating_add(*x));
    let largest_table = r.iter().take(16).copied().max().unwrap_or(0);
    let maps = mr_hash(all_rows, STR_BYTES)
        .saturating_add(mr_hash(r[T_NODES], 24))
        .saturating_add(mr_hash(r[T_DOFS], 24))
        .saturating_add(mr_hash(r[T_MASS], 16));
    // `BTreeMap::from_iter` collects the members into a vector (24 B each),
    // sorts it (a scratch of at most as many) and bulk-builds the nodes.
    let row_hash = mr_vec(d.row_members, 24)
        .saturating_add(mr_envelope(24 * d.row_members))
        .saturating_add(mr_btree(d.row_members, STR_BYTES + 8))
        .saturating_add(mr_hash_text(&d.row));
    let digest = {
        let n = largest_table;
        let cap = n.checked_next_power_of_two().unwrap_or(u128::MAX).max(4);
        let sub = Sub {
            tree: mr_envelope(cap.saturating_mul(VALUE_BYTES)).saturating_add(n.saturating_mul(64)),
            array_transient: mr_envelope(cap / 2 * VALUE_BYTES),
            canonical: n.saturating_mul(67).saturating_add(2),
            serialized: n.saturating_mul(67).saturating_add(2),
            longest_token: 66,
            longest: 64,
            ..Sub::default()
        };
        mr_hash_text(&sub)
    };
    // A refused document's codes (each a diagnostic code), and the
    // difference's `replace` copy.
    let codes = mr_vec(r[T_REFUSALS], STRING_BYTES).saturating_add(r[T_REFUSALS].saturating_mul(mr_envelope(d.largest_refusal)));
    maps.saturating_add(mr_vec(largest_table, STR_BYTES))
        .saturating_add(row_hash.max(digest))
        .saturating_add(transient_strings(d.quoted))
        // `mr_diagnostic_row`'s `{severity}\0{code}` of a diagnostics or
        // refusal row, which may quote any of its strings.
        .saturating_add(mr_message(MESSAGE_LITERAL + 2 * d.longest))
        .saturating_add(closure_difference(d.longest))
        .saturating_add(codes)
}

/// FORMED and DERIVE (`model_mass::mm_form`): what `Formed` keeps, and the
/// derivation's peak with everything it holds at once (summed).
fn derive_bytes(q: &Req, d: &Doc, quoted: B) -> (B, B) {
    // `capacity`: 78, 42 or 21 entries per member, one member per pipe; the
    // distinct upper positions are also at most ndof(ndof + 1)/2.
    let capacity = 78 * q.pipes;
    let ndof = 6 * q.nodes;
    let entries = capacity.min(ndof.saturating_mul(ndof + 1) / 2);
    let id = mr_envelope(q.longest);
    let formed = [
        mr_envelope(MM_FORMED),                                      // Box<Formed>
        mr_hash(entries, MM_ENTRY),                                  // entries (try_reserve(acc.len()))
        mr_vec(entries, 16),                                         // order
        mr_envelope(ndof),                                           // restrained
        mr_message(MESSAGE_LITERAL),                                 // conditional
        mr_vec(q.primitive + 1, STRING_BYTES) + mr_envelope(256),    // scope (one message after dedup)
        mr_vec(entries, 32),                                         // open
        mr_vec(q.nodes, 24),                                         // positions
        mr_vec(q.pipes, MM_SUBJECT) + q.pipes.saturating_mul(id),    // subjects and their ids
    ]
    .iter()
    .fold(0, |a: B, x| a.saturating_add(*x));
    let peak = [
        formed,
        q.primitive.saturating_mul(mr_envelope(256)),                // scope before dedup
        mr_hash(q.nodes, 24),                                        // node_index
        mr_hash(q.pipes, STR_BYTES),                                 // pipe_ids
        2 * mr_hash(q.components, 32),                               // replaced, arcs
        mr_btree(q.components, STR_BYTES + 24) + 112 * q.components, // mapped and its lists
        mr_hash(q.components, MM_BODY_ENTRY) + q.components.saturating_mul(id), // bodies
        mr_vec(q.sections, 8),                                       // shared
        mr_vec(q.pipes, MM_MEMBER) + 2 * q.pipes.saturating_mul(id), // members, ids, body components
        mr_hash(q.pipes, MM_SECTION_ENTRY),                          // section_mass
        q.largest_pipe + 2 * (q.largest_section + 2 * mr_envelope(MAP_NODE_BYTES)) + SECTION_SMALL, // one section's copy
        mr_vec(q.supports, 40),                                      // nonlinear
        2 * mr_vec(q.restraints + 1, 16) + 64,                       // tokens, dofs, components
        mr_envelope(ndof) + mr_vec(d.rows[T_SUPPORTS], 8) + mr_message(MESSAGE_LITERAL), // mm_states: base, rows, its reason
        mr_hash(capacity, MM_ACC_ENTRY),                             // acc (try_reserve(capacity))
        transient_strings(quoted),
    ]
    .iter()
    .fold(0, |a: B, x| a.saturating_add(*x));
    (formed, peak)
}

/// The census relation (`census.rs` `cs_subjects`, `cs_relation`,
/// `cs_digest`), with the bound request.
fn census_bytes(q: &Req, d: &Doc) -> B {
    let id = mr_envelope(q.longest);
    let subjects = q.pipes + q.supports;
    let r = &d.rows;
    let digest = {
        let cap = subjects.checked_next_power_of_two().unwrap_or(u128::MAX).max(4);
        // Each `{kind, id}` object: one map node, two keys, the kind and the id.
        let each = mr_envelope(MAP_NODE_BYTES) + 3 * 16 + id;
        let token = 6 * q.longest + 2;
        let sub = Sub {
            tree: mr_envelope(cap.saturating_mul(VALUE_BYTES)).saturating_add(subjects.saturating_mul(each)),
            array_transient: mr_envelope(cap / 2 * VALUE_BYTES),
            scratch: q.longest + 4,
            seen_peak: seen_bytes(2, 32),
            // `{"id":…,"kind":…},` is 16 B besides its id and its kind (≤ 17 B quoted).
            canonical: subjects.saturating_mul(token + 33).saturating_add(2),
            serialized: subjects.saturating_mul(token + 33).saturating_add(2),
            keys_path: 16,
            longest_token: token.max(17),
            longest: q.longest.max(15),
            ..Sub::default()
        };
        mr_vec(subjects, VALUE_BYTES) + subjects.saturating_mul(each) + mr_hash_text(&sub)
    };
    [
        mr_hash(q.pipes, STR_BYTES),                                   // pipe_ids
        2 * mr_hash(q.components, STR_BYTES),                          // replaced, arcs
        mr_btree(q.components, STR_BYTES + 24) + 112 * q.components,   // mapped
        mr_hash(q.components, 40) + q.components.saturating_mul(id),   // bodies
        mr_vec(subjects, MM_SUBJECT) + subjects.saturating_mul(id),    // list
        mr_vec(q.supports, STRING_BYTES) + q.supports.saturating_mul(id), // nonlinear
        mr_vec(q.pipes, MM_SUBJECT) + q.pipes.saturating_mul(id),      // formed (clone)
        mr_vec(subjects, MM_SUBJECT) + q.pipes.saturating_mul(id),     // census (clone)
        4 * mr_vec(subjects, STR_BYTES) + mr_vec(q.supports, STR_BYTES), // ids, nonlinear
        9 * mr_vec(r[T_IDENTITY], STR_BYTES) + mr_vec(r[T_DISC], STR_BYTES) + mr_vec(r[T_SUPPORTS], STR_BYTES), // along
        mr_vec(11, 72) + 11 * mr_message(MESSAGE_LITERAL),             // tables
        mr_vec(subjects, 8) + mr_vec(r[T_IDENTITY].max(r[T_DISC]).max(r[T_SUPPORTS]), 8), // missing, extra
        digest,
    ]
    .iter()
    .fold(0, |a: B, x| a.saturating_add(*x))
}

/// The two messages that list subjects (`census.rs`: the self-check's
/// `{formed:?}`/`{census:?}`, an item; the identities' `{missing:?}` /
/// `{extra:?}`, a difference): at most one is formed. A `Debug` string is at
/// most 6 bytes per decoded byte plus quotes and separators.
fn special_message(q: &Req, d: &Doc) -> B {
    let subjects = q.pipes + q.supports + q.components;
    // `("kind", "id"), ` per subject: ≤ 30 bytes besides the id's Debug.
    let listed = 2 * subjects.saturating_mul(30).saturating_add(12 * q.subject_bytes);
    let tables = d.rows[T_IDENTITY] + d.rows[T_DISC] + d.rows[T_SUPPORTS];
    let missing = subjects.saturating_add(tables).saturating_mul(6).saturating_add(6 * q.subject_bytes).saturating_add(6 * d.id_bytes);
    mr_message(MESSAGE_LITERAL + listed.max(missing))
}

/// RELATIONS (`read_modal_result` from `mr_mass` to `mr_units`, with
/// `model_mass` and `census`): the items every relation keeps (the verdict's
/// strings), and the largest relation's transient with `mr_of`'s copies.
fn relations_bytes(d: &Doc, q: &Req, with_request: bool) -> B {
    let r = &d.rows;
    let (e, n, ndof, nc, ncl, ni) = (r[T_MASS], r[T_MODES], r[T_DOFS], r[T_COMPUTED], r[T_CLUSTERS], r[T_IDENTITY]);
    let quoted = d.quoted.max(q.longest);
    let open = (78 * q.pipes).min((6 * q.nodes).saturating_mul(6 * q.nodes + 1) / 2);
    // Items, by message family. Per mass entry (and per open model entry): its
    // two DOF labels `dof:{node}:{c}`. Per mode, cluster, computed mode and
    // identity row: no string. At most 40 others; four of them (the
    // model-derived and census reasons) quote request strings; one
    // difference per standing (closure, binding and nine relations).
    let entry_items = e + open;
    let row_items = 27 * n + ncl + nc + ni;
    let entry_message = mr_message(ROW_MESSAGE_LITERAL + 2 * (d.node + 7));
    let row_message = mr_message(ROW_MESSAGE_LITERAL);
    let items = entry_items.saturating_mul(entry_message)
        .saturating_add(row_items.saturating_mul(row_message))
        .saturating_add(40 * mr_message(MESSAGE_LITERAL))
        .saturating_add(4 * mr_message(MESSAGE_LITERAL + 4 * q.longest))
        .saturating_add(11 * mr_difference(quoted))
        .saturating_add(if with_request { 2 * special_message(q, d) } else { 0 });
    let count = entry_items + row_items + 60;
    // Each relation's item and standing vectors (scope, unresolved, the
    // partition's two halves): Σ over relations of vec(k, 24) ≤ 144·(K + 18) + 288.
    let vectors = 144 * (count + 18) + 288 + mr_envelope(9 * STANDING_BYTES);
    // mr_of's copies of the unresolved items of the largest relation.
    let copies = entry_items.saturating_mul(mr_envelope(ROW_MESSAGE_LITERAL + 2 * (d.node + 7)))
        .max(row_items.saturating_mul(mr_envelope(ROW_MESSAGE_LITERAL)))
        .saturating_add(4 * mr_vec(count, STRING_BYTES));
    let strings = transient_strings(quoted);
    // Each relation's own transient (`lib.rs`), the largest of them.
    let mass = mr_vec(e, 48) + e * NEED_HEAP + mr_vec(e, 16) + mr_vec(e, 8) + mr_vec(n, REACH_BYTES) + n * NEED_HEAP + mr_vec(n, 8);
    let model_mass = mr_hash(e, 16) + 2 * mr_envelope(16 * ndof) + 2 * mr_envelope(8 * ndof) + mr_envelope(16 * ndof);
    let participation = mr_envelope(8 * ndof) + mr_envelope(ndof) + mr_vec(e, 16)
        + 3 * mr_vec(e, 24)
        + mr_vec(e, REACH_BYTES) + e * 128
        + 3 * (mr_vec(e, 48) + e * NEED_HEAP)
        + 4 * (mr_vec(e, REACH_BYTES) + e * NEED_HEAP)
        + mr_vec(e, 24) + mr_vec(e + 2, 8) + 128
        + mr_vec(n, REACH_BYTES) + n * REACH_HEAP + mr_vec(n, 8);
    let model_participation = 4 * mr_envelope(16 * ndof) + 2 * mr_envelope(8 * ndof) + mr_vec(ndof, 8) + mr_envelope(8 * ndof) + 64;
    let counts = 2 * mr_envelope(8 * nc);
    let clusters = mr_vec(nc, 8) + mr_vec(nc, REACH_BYTES) + nc * NEED_HEAP;
    let evidence = 3 * mr_vec(ni, 48) + 2 * mr_vec(ni, 32) + mr_hash(r[T_DISC], STR_BYTES) + mr_hash(ni, 32)
        + mr_hash(ndof, 8) + mr_hash(d.restrained, 8);
    let census = if with_request { census_bytes(q, d) } else { 0 };
    let transient = [mass, model_mass, participation, model_participation, counts, clusters, evidence, census]
        .into_iter()
        .max()
        .unwrap_or(0)
        .saturating_add(REACH_OPS)
        .saturating_add(SMALL_VECTORS)
        .saturating_add(strings);
    items.saturating_add(vectors).saturating_add(copies).saturating_add(transient)
}

fn doc_counts(s: &Scanned) -> Doc {
    let mut row = Sub::default();
    let mut row_members = 0;
    for p in 0..16 {
        row.widen(&s.largest[p]);
        row_members = row_members.max(s.members[p]);
    }
    Doc {
        rows: s.count,
        row,
        row_members,
        binding: s.largest[D_BINDING],
        restrained: s.count[D_RESTRAINED],
        largest_refusal: s.largest[T_REFUSALS].longest,
        longest: s.whole.longest,
        quoted: QUOTED.iter().map(|p| s.largest[*p].longest).max().unwrap_or(0),
        node: s.largest[T_NODES].longest,
        id_bytes: s.strings[T_IDENTITY].saturating_add(s.strings[T_DISC]).saturating_add(s.strings[T_SUPPORTS]),
    }
}

fn request_counts(s: &Scanned) -> Req {
    Req {
        nodes: s.count[Q_NODES],
        pipes: s.count[Q_PIPES],
        components: s.count[Q_COMPONENTS],
        supports: s.count[Q_SUPPORTS],
        sections: s.count[Q_SECTIONS],
        primitive: s.count[Q_PRIMITIVE],
        restraints: s.count[Q_RESTRAINTS],
        largest_pipe: s.largest[Q_PIPES].tree,
        largest_section: s.largest[Q_SECTIONS].tree,
        hash: mr_hash_text(&s.whole),
        longest: s.whole.longest,
        subject_bytes: s.strings[Q_PIPES].saturating_add(s.strings[Q_COMPONENTS]).saturating_add(s.strings[Q_SUPPORTS]),
    }
}

/// The working-memory bound of reading `document_text` with `request_text`
/// (module docs): requested heap bytes, each live block priced by
/// [`mr_envelope`]. `None` when either text is not one JSON value under
/// serde_json's grammar (the host then refuses it unread), or when the bound
/// exceeds `u64`.
pub fn modal_reader_working_bound(document_text: &str, request_text: Option<&str>) -> Option<u64> {
    let terms = modal_reader_working_terms(document_text, request_text)?;
    terms.iter().find(|(name, _)| *name == "bound").map(|(_, v)| *v)
}

/// [`modal_reader_working_bound`]'s named terms (module docs), the bound
/// last; for tests and reports.
#[doc(hidden)]
pub fn modal_reader_working_terms(document_text: &str, request_text: Option<&str>) -> Option<Vec<(&'static str, u64)>> {
    let doc_scan = mr_scan(document_text, &DOC_PATTERNS)?;
    let req_scan = match request_text {
        Some(text) => Some(mr_scan(text, &REQUEST_PATTERNS)?),
        None => None,
    };
    let d = doc_counts(&doc_scan);
    let q = req_scan.as_ref().map(request_counts).unwrap_or_default();
    let with_request = req_scan.is_some();
    let whole = &doc_scan.whole;

    // Q: the host's parse of the request.
    let request_parse = req_scan.as_ref().map_or(0, |s| mr_parse(&s.whole, false));
    // P1: the checked profile of the document text, and its difference.
    let closure_failed = closure_difference(whole.longest_key) + mr_envelope(9 * STANDING_BYTES);
    let p1 = mr_parse(whole, true).max(mr_write(whole)).saturating_add(closure_failed);
    // P2: the reader's parse.
    let p2 = mr_parse(whole, false).saturating_add(closure_failed);
    // READ.
    let parsed = parsed_bytes(&d);
    let closure = closure_bytes(&d);
    // BINDING: one hash at a time; the run id's object (one map node, two
    // keys, two 64-byte hashes: 1,200 B) beside its own hash; a unit
    // lookup's error copying an option's unit; the run id's `format!`.
    let binding = mr_hash_text(&d.binding)
        .max(if with_request { q.hash } else { 0 })
        .saturating_add(2_048)
        .saturating_add(mr_envelope(d.binding.longest))
        .saturating_add(transient_strings(0));
    let (formed, derive) = if with_request { derive_bytes(&q, &d, d.quoted.max(q.longest)) } else { (0, 0) };
    let relations = formed.saturating_add(relations_bytes(&d, &q, with_request));
    // The verdict's own vectors and the binding's difference and `not_checked`.
    let result = mr_envelope(9 * STANDING_BYTES) + mr_vec(2, STRING_BYTES) + 64 + mr_difference(0);
    let read = parsed
        .saturating_add(closure.max(binding).max(derive).max(relations))
        .saturating_add(result);
    let document = p1.max(p2).max(whole.tree.saturating_add(read));
    let request_tree = req_scan.as_ref().map_or(0, |s| s.whole.tree);
    let bound = request_parse.max(request_tree.saturating_add(document));
    let terms = [
        ("request_parse", request_parse),
        ("request_tree", request_tree),
        ("document_tree", whole.tree),
        ("p1_checked_profile", p1),
        ("p2_parse", p2),
        ("parsed", parsed),
        ("closure", closure),
        ("binding", binding),
        ("formed", formed),
        ("derive", derive),
        ("relations", relations),
        ("result", result),
        ("read", read),
        ("bound", bound),
    ];
    terms.iter().map(|(name, v)| u64::try_from(*v).ok().map(|v| (*name, v))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scan_refuses_what_serde_json_refuses() {
        for text in ["", " ", "{", "[1,]", "{\"a\":1,}", "01", "1.", "-", "\"\\x\"", "\"\u{1}\"", "\"\\udc00\"", "[1] 2", "nul", "\u{feff}1"] {
            assert!(mr_scan(text, &[]).is_none(), "{text:?}");
            assert!(serde_json::from_str::<serde_json::Value>(text).is_err(), "{text:?}");
        }
        let deep = format!("{}{}", "[".repeat(128), "]".repeat(128));
        assert!(mr_scan(&deep, &[]).is_none() && serde_json::from_str::<serde_json::Value>(&deep).is_err());
        let ok = format!("{}{}", "[".repeat(127), "]".repeat(127));
        assert!(mr_scan(&ok, &[]).is_some() && serde_json::from_str::<serde_json::Value>(&ok).is_ok());
    }

    #[test]
    fn patterns_count_rows_by_decoded_key() {
        let text = r#"{"tables": {"no\u0064es": {"rows": [{"a": 1}, {"b": 2, "c": 3}]}, "dofs": {"rows": []}}, "binding": {"x": "y"}}"#;
        let s = mr_scan(text, &DOC_PATTERNS).unwrap();
        assert_eq!(s.count[T_NODES], 2);
        assert_eq!(s.members[T_NODES], 2);
        assert_eq!(s.count[T_DOFS], 0);
        assert_eq!(s.count[D_BINDING], 1);
        assert!(s.largest[D_BINDING].tree > 0);
    }

    #[test]
    fn the_unitized_header_count_is_the_closures() {
        // basis: τ_t, ℓ, a, identity acceleration, mass centre ×3; extraction: cut-off;
        // sturm: lower, upper, κ_c, 1/κ_c, κ_s, g_max; orthogonality; formation: 4 + section 3.
        assert_eq!(7 + 1 + 6 + 1 + 7, 22);
        assert_eq!(super::super::TABLES.len(), 15);
    }
}
