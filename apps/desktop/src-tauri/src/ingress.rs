//! Allocation-free ingress prescan for a raw request body: a solve's model
//! document, or an interactive command's arguments (`crate::command_ingress`)
//! (DEL-04-07 memory-budget decision: "An ingress byte limit/reservation must
//! precede native Value materialization").
//!
//! [`prescan`] validates the raw bytes as exactly one JSON value under the
//! grammar `serde_json::from_slice::<Value>` accepts (UTF-8, structure, the
//! parser's recursion limit, no `RawValue` token key) and computes a sound
//! upper bound on the heap bytes that materializing it allocates: the
//! `serde_json::Value` tree (serde_json 1.0.149 with `std`, `float_roundtrip`,
//! `raw_value`, `unbounded_depth`; not `preserve_order`, not
//! `arbitrary_precision`), the parser's scratch buffer, one reallocation
//! transient, the float parser's bignum transients, an error allocation and the
//! native solve request wrapper ([`native_request`]). The bound is over
//! *requested* allocations: it is at least the peak, over the parse, of the
//! sum of [`allocation_envelope`] of every live requested size (a
//! reallocation counts its old and new sizes together). It does not depend on
//! the allocator's size-class table, and it is not a bound on the allocator's
//! `malloc_size` of the blocks it hands back: see [`allocation_envelope`] for
//! what the envelope covers and the cached-block slack it does not.
//!
//! The prescan itself does not allocate: it walks the bytes once with a
//! fixed-size stack of per-container counters. [`prescan_regions`] is the same
//! walk, which also attributes the bound to the regions of a typed decoding
//! (the report-package request's, `crate::command_ingress`).

/// serde_json starts `remaining_depth` at 128 and refuses the container whose
/// entry brings it to 0, so at most 127 containers nest.
pub(crate) const MAX_NESTING: usize = 127;

/// `size_of::<serde_json::Value>()` for the configuration above.
const VALUE_BYTES: u64 = 32;
/// `BTreeMap<String, Value>` leaf node (632 B) and internal node (728 B); a
/// node of either kind is priced as the larger, `allocation_envelope(728)`.
const BTREE_LEAF_NODE_BYTES: u64 = 632;
const BTREE_INTERNAL_NODE_BYTES: u64 = 728;
/// Insert-only construction keeps every non-root node at >= 5 entries, so an
/// object with n >= 1 entries has at most floor(n / 5) + 1 nodes.
const BTREE_MIN_NON_ROOT_ENTRIES: u64 = 5;
/// The first key serde_json's `Value` visitor treats as an embedded `RawValue`
/// (and re-parses): refused at ingress.
const RAW_VALUE_TOKEN: &[u8] = b"$serde_json::private::RawValue";
/// Transient bignum allocations of serde_json's `float_roundtrip` (lexical)
/// slow path while one float is parsed; the priced peak on adversarial
/// hard-to-round floats is checked by `float_allowance_covers_hard_float_parsing`.
pub(crate) const FLOAT_PARSE_ALLOWANCE_BYTES: u64 = 16_384;
/// A serde_json error (`Box<ErrorImpl>`), allocated only if the parse fails.
const ERROR_ALLOWANCE_BYTES: u64 = 1_024;
/// The scratch buffer's minimum non-zero capacity (`Vec<u8>`).
const SCRATCH_MIN_CAPACITY: u64 = 8;
/// A string's scratch requirement can exceed its decoded length by the
/// `reserve(4)` of a non-ASCII `\u` escape.
const SCRATCH_ESCAPE_SLACK: u64 = 4;
/// A float-path number's scratch content: its digits plus at most a u64
/// significand (20 digits) and one leading zero.
const SCRATCH_NUMBER_SLACK: u64 = 21;

/// The price of one requested allocation of `size` bytes:
/// max(16, next_power_of_two(size)), 0 for 0, `None` when the next power of
/// two overflows (the caller refuses the input). For s > 8 it is < 2s.
///
/// What it covers, for a requested size s >= 1:
/// - A fresh allocation's usable size. The system allocator rounds a fresh
///   request to at most the next power of two and never below 16 bytes. On
///   macOS 26 xzone malloc, requests up to 32 KiB come from fixed size classes
///   (measured: 257 -> 320, 728 -> 768, 1008 -> 1024, 4097 -> 5120,
///   16385 -> 20480) and larger requests are page-granular (16 KiB pages:
///   32769 -> 49152, 131073 -> 147456); the legacy libmalloc tiny/small/large
///   classes satisfy it too. Checked by
///   `allocation_envelope_covers_this_hosts_malloc_size`.
/// - The pages any allocation can make resident. A size-class block (s <= 32
///   KiB here) never exceeds its class, whether fresh or reused. A
///   page-granular block is page-aligned, so the program touching its s
///   requested bytes (a reallocation copies at most that many) makes at most
///   round_up(s, P) bytes resident, P = 16 KiB. For every s >= P,
///   round_up(s, P) <= envelope(s): the envelope is a power of two not below
///   max(s, P), hence a multiple of P that is not below s, and round_up(s, P)
///   is the least such multiple. (A 4 KiB page only tightens this. For s < P
///   the inequality can fail, e.g. s = 100, but those requests are served from
///   size classes.) Checked by
///   `envelope_covers_the_resident_pages_of_page_granular_blocks`.
///
/// Not covered: `malloc_size` of a reused block. A page-granular request may
/// be served from an already-held cached free block larger than its envelope
/// (observed: a `Vec` reallocation to 4 MiB returned a 7,847,936 B block
/// after a ~7.8 MB buffer was freed). That slack is memory the process
/// already held, not new demand; only the touched pages above become
/// resident. The ingress bound therefore prices requested sizes, not
/// `malloc_size`.
pub(crate) fn allocation_envelope(size: u64) -> Option<u64> {
    match size {
        0 => Some(0),
        _ => size.checked_next_power_of_two().map(|p| p.max(16)),
    }
}

/// The native solve request `{"model": model, "materials": []}`, built by
/// moving the model in. (`json!` with an expression would deep-copy the model
/// through `to_value(&model)`, doubling it for the whole solve.)
pub(crate) fn native_request(model: serde_json::Value) -> serde_json::Value {
    let mut request = serde_json::Map::new();
    request.insert("model".to_string(), model);
    request.insert("materials".to_string(), serde_json::Value::Array(Vec::new()));
    serde_json::Value::Object(request)
}

/// [`native_request`]'s own allocations: one map leaf node and the two key
/// strings ("model", "materials"); the empty array and the moved model
/// allocate nothing further.
pub(crate) fn wrapper_bytes() -> u64 {
    allocation_envelope(BTREE_LEAF_NODE_BYTES.max(BTREE_INTERNAL_NODE_BYTES)).unwrap()
        + allocation_envelope(5).unwrap()
        + allocation_envelope(9).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct IngressBound {
    /// The raw body's length, already buffered by the IPC layer before the command runs.
    pub raw_len: u64,
    /// Upper bound on the heap bytes of the native materialization.
    pub materialization_bytes: u64,
}

impl IngressBound {
    /// Bytes the ingress lease covers: the buffered body, priced by the
    /// allocation envelope of its length (its buffer's capacity is not visible
    /// to the command), plus its materialization. Residual outside this bound:
    /// wry/Tauri may hold more than one copy of the body before the command
    /// runs; those copies are not ours to price or release.
    pub(crate) fn lease_bytes(&self) -> Option<u64> {
        allocation_envelope(self.raw_len)?.checked_add(self.materialization_bytes)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct IngressInvalid {
    pub reason: &'static str,
    pub offset: usize,
}

#[derive(Clone, Copy)]
struct Frame {
    object: bool,
    count: u64,
    /// How the decoder treats this container's entries.
    ctx: Ctx,
    /// The region this container's own allocation (array buffer or map
    /// nodes) is priced in.
    own: Region,
}

/// One field of a struct a typed decoding builds, for [`prescan_regions`].
#[derive(Clone, Copy, Debug)]
pub(crate) struct RegionField {
    pub name: &'static str,
    pub region: FieldRegion,
}

/// A struct a typed decoding builds from an object (fields matched by name,
/// after the key's escapes are decoded) or an array (fields by position).
#[derive(Debug)]
pub(crate) struct RegionStruct {
    /// The wire type's name (its table is checked against the type).
    #[cfg_attr(not(test), allow(dead_code))]
    pub name: &'static str,
    /// In declaration order: a field's index is its position in array form.
    pub fields: &'static [RegionField],
}

/// What the decoder builds from a struct field's value.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FieldRegion {
    /// Nothing beyond moved strings and inline scalars and structs: the
    /// value's containers are in neither region.
    Neither,
    /// A `Value`, rebuilt by decoding: the value is in the opaque region.
    Opaque,
    /// A `Vec` of typed elements: the value is in the typed-vector region.
    TypedVector,
    /// A `Vec<Value>`: the value is in the typed-vector region and the
    /// elements of an array there are in the opaque region.
    TypedVectorOfOpaque,
    /// A struct of its own fields; the struct's container is in neither region.
    Struct(&'static RegionStruct),
}

/// How the decoder treats a container's entries.
#[derive(Clone, Copy)]
enum Ctx {
    Neither,
    Opaque,
    Typed,
    TypedOfOpaque,
    Struct(&'static RegionStruct),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Region {
    Neither,
    Opaque,
    Typed,
}

impl FieldRegion {
    fn entered(self) -> (Ctx, Region) {
        match self {
            FieldRegion::Neither => (Ctx::Neither, Region::Neither),
            FieldRegion::Opaque => (Ctx::Opaque, Region::Opaque),
            FieldRegion::TypedVector => (Ctx::Typed, Region::Typed),
            FieldRegion::TypedVectorOfOpaque => (Ctx::TypedOfOpaque, Region::Typed),
            FieldRegion::Struct(table) => (Ctx::Struct(table), Region::Neither),
        }
    }
}

impl Frame {
    /// The context and region of this container's next entry; `field` is the
    /// struct field its key matched (objects only).
    fn entry(&self, field: Option<FieldRegion>) -> (Ctx, Region) {
        match self.ctx {
            Ctx::Neither => (Ctx::Neither, Region::Neither),
            Ctx::Opaque => (Ctx::Opaque, Region::Opaque),
            Ctx::Typed => (Ctx::Typed, Region::Typed),
            Ctx::TypedOfOpaque if self.object => (Ctx::Typed, Region::Typed),
            Ctx::TypedOfOpaque => (Ctx::Opaque, Region::Opaque),
            Ctx::Struct(table) => {
                let field = if self.object {
                    field
                } else {
                    usize::try_from(self.count).ok().and_then(|position| table.fields.get(position)).map(|field| field.region)
                };
                field.unwrap_or(FieldRegion::Neither).entered()
            }
        }
    }
}

/// Matches a decoded key against a struct's field names without allocating.
struct FieldMatch {
    fields: &'static [RegionField],
    /// Bit i: field i still matches the bytes decoded so far.
    alive: u64,
    len: usize,
}

impl FieldMatch {
    fn new(table: &'static RegionStruct) -> Self {
        assert!(table.fields.len() <= 64, "a region struct has at most 64 fields");
        let alive = if table.fields.len() == 64 { u64::MAX } else { (1u64 << table.fields.len()) - 1 };
        Self { fields: table.fields, alive, len: 0 }
    }

    fn feed(&mut self, byte: u8) {
        let mut alive = self.alive;
        while alive != 0 {
            let index = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            if self.fields[index].name.as_bytes().get(self.len) != Some(&byte) {
                self.alive &= !(1u64 << index);
            }
        }
        self.len += 1;
    }

    fn matched(&self) -> Option<FieldRegion> {
        let mut alive = self.alive;
        while alive != 0 {
            let index = alive.trailing_zeros() as usize;
            alive &= alive - 1;
            if self.fields[index].name.len() == self.len {
                return Some(self.fields[index].region);
            }
        }
        None
    }
}

/// [`prescan_regions`]' attribution of the materialization bound M: every
/// priced allocation is in exactly one part, so the parts sum to M.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct RegionBytes {
    /// Strings and keys (decoding moves them).
    pub strings: u64,
    /// C_V: the containers (array buffers and map nodes) of the opaque regions.
    pub opaque: u64,
    /// C_T: the containers of the typed-vector regions.
    pub typed: u64,
    /// The containers in neither region.
    pub neither: u64,
    /// The fixed allowances: scratch, the array transient, the float
    /// allowance, the error allowance and the solve wrapper.
    pub allowances: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Expect {
    Value,
    ValueOrArrayEnd,
    KeyOrObjectEnd,
    Key,
    Colon,
    CommaOrEnd,
    Done,
}

struct Scan<'a> {
    bytes: &'a [u8],
    index: usize,
    /// Final-tree bytes: strings, keys, array buffers and map nodes.
    tree: u64,
    /// `tree`'s attribution (strings and the containers of each region).
    regions: RegionBytes,
    /// The largest previous-capacity buffer of any array (realloc transient).
    largest_array_transient: u64,
    /// The largest scratch content requirement.
    largest_scratch: u64,
    float_path_seen: bool,
}

/// Advances the `RawValue` token comparison by one decoded byte.
fn feed(byte: u8, token_match: &mut bool, token_pos: &mut usize) {
    if *token_match {
        if RAW_VALUE_TOKEN.get(*token_pos) == Some(&byte) {
            *token_pos += 1;
        } else {
            *token_match = false;
        }
    }
}

fn invalid(reason: &'static str, offset: usize) -> IngressInvalid {
    IngressInvalid { reason, offset }
}

impl Scan<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    fn next(&mut self, reason: &'static str) -> Result<u8, IngressInvalid> {
        let byte = self.peek().ok_or(invalid(reason, self.index))?;
        self.index += 1;
        Ok(byte)
    }

    fn skip_whitespace(&mut self) {
        while let Some(b' ' | b'\n' | b'\t' | b'\r') = self.peek() {
            self.index += 1;
        }
    }

    /// Prices one requested allocation of `bytes` in the tree: a string or key
    /// (`region` `None`) or a container of `region`.
    fn add_tree(&mut self, bytes: u64, region: Option<Region>) -> Result<(), IngressInvalid> {
        let priced = allocation_envelope(bytes).ok_or(invalid("input too large to price", self.index))?;
        self.add_priced(priced, region)
    }

    fn add_priced(&mut self, priced: u64, region: Option<Region>) -> Result<(), IngressInvalid> {
        self.tree = self.tree.checked_add(priced).ok_or(invalid("input too large to price", self.index))?;
        // Each part is at most `tree`, so it cannot overflow.
        let part = match region {
            None => &mut self.regions.strings,
            Some(Region::Opaque) => &mut self.regions.opaque,
            Some(Region::Typed) => &mut self.regions.typed,
            Some(Region::Neither) => &mut self.regions.neither,
        };
        *part += priced;
        Ok(())
    }

    fn hex4(&mut self) -> Result<u32, IngressInvalid> {
        let mut value = 0u32;
        for _ in 0..4 {
            let byte = self.next("unterminated unicode escape")?;
            let digit = (byte as char).to_digit(16).ok_or(invalid("invalid unicode escape", self.index - 1))?;
            value = value * 16 + digit;
        }
        Ok(value)
    }

    /// Scans a string whose opening quote was consumed; prices its owned copy.
    /// Returns whether it decodes to the `RawValue` token. A key of a struct
    /// the decoder builds is also matched against its field names (`field`).
    fn string(&mut self, mut field: Option<&mut FieldMatch>) -> Result<bool, IngressInvalid> {
        let mut decoded: u64 = 0;
        let mut escaped = false;
        let mut token_match = true;
        let mut token_pos = 0usize;
        loop {
            let byte = self.next("unterminated string")?;
            match byte {
                b'"' => break,
                b'\\' => {
                    escaped = true;
                    let escape = self.next("unterminated escape")?;
                    let simple = match escape {
                        b'"' => Some(b'"'),
                        b'\\' => Some(b'\\'),
                        b'/' => Some(b'/'),
                        b'b' => Some(0x08),
                        b'f' => Some(0x0c),
                        b'n' => Some(b'\n'),
                        b'r' => Some(b'\r'),
                        b't' => Some(b'\t'),
                        b'u' => None,
                        _ => return Err(invalid("invalid escape", self.index - 1)),
                    };
                    let code_point = match simple {
                        Some(decoded_byte) => {
                            feed(decoded_byte, &mut token_match, &mut token_pos);
                            if let Some(field) = field.as_deref_mut() {
                                field.feed(decoded_byte);
                            }
                            decoded += 1;
                            continue;
                        }
                        None => {
                            let first = self.hex4()?;
                            match first {
                                0xDC00..=0xDFFF => return Err(invalid("lone trailing surrogate", self.index)),
                                0xD800..=0xDBFF => {
                                    if self.next("lone leading surrogate")? != b'\\'
                                        || self.next("lone leading surrogate")? != b'u'
                                    {
                                        return Err(invalid("lone leading surrogate", self.index));
                                    }
                                    let second = self.hex4()?;
                                    if !(0xDC00..=0xDFFF).contains(&second) {
                                        return Err(invalid("lone leading surrogate", self.index));
                                    }
                                    (((first - 0xD800) << 10) | (second - 0xDC00)) + 0x1_0000
                                }
                                other => other,
                            }
                        }
                    };
                    let ch = char::from_u32(code_point).ok_or(invalid("invalid code point", self.index))?;
                    let mut buffer = [0u8; 4];
                    for byte in ch.encode_utf8(&mut buffer).bytes() {
                        feed(byte, &mut token_match, &mut token_pos);
                        if let Some(field) = field.as_deref_mut() {
                            field.feed(byte);
                        }
                    }
                    decoded += ch.len_utf8() as u64;
                }
                0x00..=0x1f => return Err(invalid("control character in string", self.index - 1)),
                other => {
                    feed(other, &mut token_match, &mut token_pos);
                    if let Some(field) = field.as_deref_mut() {
                        field.feed(other);
                    }
                    decoded += 1;
                }
            }
        }
        self.add_tree(decoded, None)?;
        if escaped {
            self.largest_scratch = self.largest_scratch.max(decoded.saturating_add(SCRATCH_ESCAPE_SLACK));
        }
        Ok(token_match && token_pos == RAW_VALUE_TOKEN.len())
    }

    fn digits(&mut self) -> u64 {
        let mut count = 0;
        while let Some(b'0'..=b'9') = self.peek() {
            self.index += 1;
            count += 1;
        }
        count
    }

    /// serde_json's number grammar; numbers allocate no tree bytes.
    fn number(&mut self) -> Result<(), IngressInvalid> {
        let start = self.index;
        if self.peek() == Some(b'-') {
            self.index += 1;
        }
        let integer_digits = match self.next("invalid number")? {
            b'0' => {
                if let Some(b'0'..=b'9') = self.peek() {
                    return Err(invalid("leading zero in number", self.index));
                }
                1
            }
            b'1'..=b'9' => 1 + self.digits(),
            _ => return Err(invalid("invalid number", self.index - 1)),
        };
        // An integer with 20+ digits may overflow u64 and take the float path.
        let mut float_path = integer_digits >= 20;
        if self.peek() == Some(b'.') {
            self.index += 1;
            if self.digits() == 0 {
                return Err(invalid("invalid number", self.index));
            }
            float_path = true;
        }
        if let Some(b'e' | b'E') = self.peek() {
            self.index += 1;
            if let Some(b'+' | b'-') = self.peek() {
                self.index += 1;
            }
            if self.digits() == 0 {
                return Err(invalid("invalid number", self.index));
            }
            float_path = true;
        }
        if float_path {
            self.float_path_seen = true;
            let len = (self.index - start) as u64;
            self.largest_scratch = self.largest_scratch.max(len.saturating_add(SCRATCH_NUMBER_SLACK));
        }
        Ok(())
    }

    fn literal(&mut self, rest: &'static [u8]) -> Result<(), IngressInvalid> {
        for expected in rest {
            if self.next("invalid literal")? != *expected {
                return Err(invalid("invalid literal", self.index - 1));
            }
        }
        Ok(())
    }

    fn close(&mut self, frame: Frame) -> Result<(), IngressInvalid> {
        let too_large = invalid("input too large to price", self.index);
        if frame.object {
            if frame.count > 0 {
                let nodes = frame.count / BTREE_MIN_NON_ROOT_ENTRIES + 1;
                let node = allocation_envelope(BTREE_INTERNAL_NODE_BYTES.max(BTREE_LEAF_NODE_BYTES)).unwrap();
                let bytes = nodes.checked_mul(node).ok_or(too_large)?;
                self.add_priced(bytes, Some(frame.own))?;
            }
        } else if frame.count > 0 {
            // `Vec::new()` + `push`: capacities 4, 8, 16, ...
            let capacity = frame.count.checked_next_power_of_two().ok_or(too_large)?.max(4);
            let buffer = capacity.checked_mul(VALUE_BYTES).ok_or(too_large)?;
            self.add_tree(buffer, Some(frame.own))?;
            if capacity > 4 {
                let previous = allocation_envelope(capacity / 2 * VALUE_BYTES).ok_or(too_large)?;
                self.largest_array_transient = self.largest_array_transient.max(previous);
            }
        }
        Ok(())
    }
}

/// Validates `input` as one JSON value and bounds its native materialization.
pub(crate) fn prescan(input: &[u8]) -> Result<IngressBound, IngressInvalid> {
    scan(input, None).map(|(bound, _)| bound)
}

/// [`prescan`], which also attributes each priced allocation of the bound M
/// to a region of the typed value `root` the decoder builds from it, by the
/// path the decoder follows (DEL-04-07 memory-budget decision, "Report-package
/// request pricing"). A struct's field is matched by name in an object, after
/// the key's escapes are decoded as the parser decodes them, or by position in
/// an array; an unmatched key or position, and everything under a
/// [`FieldRegion::Neither`] field, is in neither region. The validation, the
/// bound and the refusals are [`prescan`]'s, and it allocates nothing.
pub(crate) fn prescan_regions(input: &[u8], root: &'static RegionStruct) -> Result<(IngressBound, RegionBytes), IngressInvalid> {
    scan(input, Some(root))
}

fn scan(input: &[u8], root: Option<&'static RegionStruct>) -> Result<(IngressBound, RegionBytes), IngressInvalid> {
    if let Err(error) = std::str::from_utf8(input) {
        return Err(invalid("invalid UTF-8", error.valid_up_to()));
    }
    let mut scan = Scan {
        bytes: input,
        index: 0,
        tree: 0,
        regions: RegionBytes::default(),
        largest_array_transient: 0,
        largest_scratch: 0,
        float_path_seen: false,
    };
    let mut stack = [Frame { object: false, count: 0, ctx: Ctx::Neither, own: Region::Neither }; MAX_NESTING];
    let mut depth = 0usize;
    let mut expect = Expect::Value;
    // The struct field the last key matched, for the value that follows it.
    let mut key_field: Option<FieldRegion> = None;
    loop {
        scan.skip_whitespace();
        let Some(byte) = scan.peek() else {
            if expect == Expect::Done {
                break;
            }
            return Err(invalid("unexpected end of input", scan.index));
        };
        let mut value_done = false;
        match expect {
            Expect::Done => return Err(invalid("trailing characters", scan.index)),
            Expect::Value | Expect::ValueOrArrayEnd => {
                if expect == Expect::ValueOrArrayEnd && byte == b']' {
                    scan.index += 1;
                    depth -= 1;
                    scan.close(stack[depth])?;
                    value_done = true;
                } else {
                    scan.index += 1;
                    match byte {
                        b'{' | b'[' => {
                            if depth == MAX_NESTING {
                                return Err(invalid("nesting deeper than 127 containers", scan.index - 1));
                            }
                            let (ctx, own) = match depth {
                                0 => (root.map_or(Ctx::Neither, Ctx::Struct), Region::Neither),
                                _ => stack[depth - 1].entry(key_field),
                            };
                            stack[depth] = Frame { object: byte == b'{', count: 0, ctx, own };
                            depth += 1;
                            expect = if byte == b'{' { Expect::KeyOrObjectEnd } else { Expect::ValueOrArrayEnd };
                        }
                        b'"' => {
                            scan.string(None)?;
                            value_done = true;
                        }
                        b't' => {
                            scan.literal(b"rue")?;
                            value_done = true;
                        }
                        b'f' => {
                            scan.literal(b"alse")?;
                            value_done = true;
                        }
                        b'n' => {
                            scan.literal(b"ull")?;
                            value_done = true;
                        }
                        b'-' | b'0'..=b'9' => {
                            scan.index -= 1;
                            scan.number()?;
                            value_done = true;
                        }
                        _ => return Err(invalid("expected a value", scan.index - 1)),
                    }
                }
            }
            Expect::KeyOrObjectEnd | Expect::Key => {
                scan.index += 1;
                if expect == Expect::KeyOrObjectEnd && byte == b'}' {
                    depth -= 1;
                    scan.close(stack[depth])?;
                    value_done = true;
                } else if byte == b'"' {
                    let mut field = match stack[depth - 1].ctx {
                        Ctx::Struct(table) => Some(FieldMatch::new(table)),
                        _ => None,
                    };
                    if scan.string(field.as_mut())? {
                        return Err(invalid("reserved RawValue token key", scan.index));
                    }
                    key_field = field.and_then(|field| field.matched());
                    expect = Expect::Colon;
                } else {
                    return Err(invalid("expected an object key", scan.index - 1));
                }
            }
            Expect::Colon => {
                scan.index += 1;
                if byte != b':' {
                    return Err(invalid("expected ':'", scan.index - 1));
                }
                expect = Expect::Value;
            }
            Expect::CommaOrEnd => {
                scan.index += 1;
                let frame = stack[depth - 1];
                match (byte, frame.object) {
                    (b',', true) => expect = Expect::Key,
                    (b',', false) => expect = Expect::Value,
                    (b'}', true) | (b']', false) => {
                        depth -= 1;
                        scan.close(frame)?;
                        value_done = true;
                    }
                    _ => return Err(invalid("expected ',' or a closing bracket", scan.index - 1)),
                }
            }
        }
        if value_done {
            if depth == 0 {
                expect = Expect::Done;
            } else {
                stack[depth - 1].count += 1;
                expect = Expect::CommaOrEnd;
            }
        }
    }

    let too_large = invalid("input too large to price", input.len());
    // The scratch buffer persists across the parse at its largest capacity
    // C <= max(8, 2 L); its last growth also briefly held the previous
    // buffer (<= C / 2).
    let scratch = if scan.largest_scratch == 0 {
        0
    } else {
        let capacity = scan.largest_scratch.checked_mul(2).ok_or(too_large)?.max(SCRATCH_MIN_CAPACITY);
        allocation_envelope(capacity)
            .and_then(|c| c.checked_add(allocation_envelope(capacity / 2)?))
            .ok_or(too_large)?
    };
    let float = if scan.float_path_seen { FLOAT_PARSE_ALLOWANCE_BYTES } else { 0 };
    let allowances = [scratch, scan.largest_array_transient, float, ERROR_ALLOWANCE_BYTES, wrapper_bytes()]
        .into_iter()
        .try_fold(0u64, u64::checked_add)
        .ok_or(too_large)?;
    let materialization_bytes = scan.tree.checked_add(allowances).ok_or(too_large)?;
    let regions = RegionBytes { allowances, ..scan.regions };
    Ok((IngressBound { raw_len: input.len() as u64, materialization_bytes }, regions))
}

#[cfg(test)]
pub(crate) mod counting {
    //! Per-thread allocation accounting for the ingress bound tests: the live
    //! requested bytes and the live bytes priced by `allocation_envelope` of
    //! each requested size. A moving or in-place reallocation counts its old
    //! and new sizes as live together at that instant. Only requested sizes
    //! are counted, so the measure is deterministic: it does not depend on the
    //! heap's history (which blocks the allocator reuses) or on other threads.
    use std::cell::Cell;

    thread_local! {
        static ACTIVE: Cell<bool> = const { Cell::new(false) };
        static LIVE: Cell<i64> = const { Cell::new(0) };
        static PEAK: Cell<i64> = const { Cell::new(0) };
        static LIVE_PRICED: Cell<i64> = const { Cell::new(0) };
        static PEAK_PRICED: Cell<i64> = const { Cell::new(0) };
        static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    }

    fn price(size: usize) -> i64 {
        super::allocation_envelope(size as u64).unwrap_or(u64::MAX) as i64
    }

    fn active() -> bool {
        ACTIVE.try_with(Cell::get).unwrap_or(false)
    }

    /// Adds `added`, records the peak, then removes `removed` (both live across).
    fn bump(live: &'static std::thread::LocalKey<Cell<i64>>, peak: &'static std::thread::LocalKey<Cell<i64>>, added: i64, removed: i64) {
        live.with(|live| {
            live.set(live.get() + added);
            peak.with(|peak| peak.set(peak.get().max(live.get())));
            live.set(live.get() - removed);
        });
    }

    /// Called by the process allocator ([`crate::worker_alloc::CappedAllocator`]).
    pub(crate) fn record(added: usize, removed: usize, counts_as_allocation: bool) {
        if !active() {
            return;
        }
        if counts_as_allocation {
            ALLOCATIONS.with(|c| c.set(c.get() + 1));
        }
        bump(&LIVE, &PEAK, added as i64, removed as i64);
        let priced = |size: usize| if size > 0 { price(size) } else { 0 };
        bump(&LIVE_PRICED, &PEAK_PRICED, priced(added), priced(removed));
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Measured {
        pub peak_requested: u64,
        /// Peak of the live sum of `allocation_envelope(requested size)`.
        pub peak_priced: u64,
        pub allocations: u64,
    }

    /// The live priced bytes now (inside [`measure`]).
    pub fn live_priced() -> i64 {
        LIVE_PRICED.with(Cell::get)
    }

    /// Runs `f` with this thread's accounting active from zero.
    pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Measured) {
        for cell in [&LIVE, &PEAK, &LIVE_PRICED, &PEAK_PRICED] {
            cell.with(|c| c.set(0));
        }
        ALLOCATIONS.with(|c| c.set(0));
        ACTIVE.with(|a| a.set(true));
        let out = f();
        ACTIVE.with(|a| a.set(false));
        let measured = Measured {
            peak_requested: PEAK.with(Cell::get).max(0) as u64,
            peak_priced: PEAK_PRICED.with(Cell::get).max(0) as u64,
            allocations: ALLOCATIONS.with(Cell::get),
        };
        (out, measured)
    }
}

#[cfg(test)]
mod tests {
    use super::counting::measure;
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn native_request_moves_the_model_without_copying() {
        let model: Value = serde_json::from_str(r#"{"nodes":[1,2,3],"name":"n"}"#).unwrap();
        let expected = json!({"model": model.clone(), "materials": []});
        let (request, measured) = measure(|| native_request(model));
        assert_eq!(request, expected);
        assert_eq!(measured.peak_priced, wrapper_bytes(), "only the wrapper allocates");
    }

    fn nested(depth: usize, open: &str, close: &str, inner: &str) -> String {
        format!("{}{}{}", open.repeat(depth), inner, close.repeat(depth))
    }

    fn adversarial_inputs() -> Vec<(String, Vec<u8>)> {
        let tiny = format!("0.{}1e-300", "0".repeat(800));
        let long = format!("0.{}e-10", "7".repeat(900));
        let hard_floats = format!(
            "[2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936316e-324,{tiny},9007199254740993.0000000000000000000000000000001,1e-400,2.2250738585072011e-308,{long},1]"
        );
        let mut inputs: Vec<(String, String)> = vec![
            ("zeros_array".into(), format!("[{}0]", "0,".repeat(4_999))),
            ("tiny_objects".into(), format!("[{}{{\"\":0}}]", "{\"\":0},".repeat(2_999))),
            ("empty_containers".into(), format!("[{}[]]", "[],{},\"\",".repeat(1_000))),
            ("nested_arrays_127".into(), nested(127, "[", "]", "0")),
            ("nested_objects_127".into(), nested(127, "{\"a\":", "}", "1")),
            ("singleton_arrays".into(), format!("[{}[0]]", "[0],".repeat(2_000))),
            ("power_of_two_plus_one".into(), format!("[{}0]", "0,".repeat(4_096))),
            ("long_escaped_string".into(), format!("[\"{}\"]", "\\n".repeat(20_000))),
            ("long_plain_string".into(), format!("[\"{}\"]", "a".repeat(40_000))),
            ("unicode_escapes".into(), format!("[\"{}\",\"{}\"]", "\\u20ac".repeat(3_000), "\\ud83d\\ude00".repeat(2_000))),
            ("escaped_keys".into(), format!("{{{}\"z\":0}}", (0..2_000).map(|i| format!("\"k\\u00e9{i}\":0,")).collect::<String>())),
            ("duplicate_keys".into(), format!("{{{}\"a\":[1]}}", "\"a\":[1,2,3,4,5],".repeat(2_000))),
            ("many_keys_object".into(), format!("{{{}\"end\":0}}", (0..5_000).map(|i| format!("\"{i}\":{i},")).collect::<String>())),
            ("long_integers".into(), format!("[{}1]", "123456789012345678901234567890,".repeat(500))),
            ("hard_floats".into(), hard_floats),
            // Grammatical but out of f64 range: the parse fails after building a partial tree.
            ("out_of_range_float".into(), format!("[{}1,{}e-10]", "[0],".repeat(300), "7".repeat(900))),
            ("mixed_document".into(), format!("{{\"nodes\":[{}{{}}],\"name\":\"x\\ty\"}}", "{\"id\":\"n\\u0031\",\"xyz\":[1.5,-2e3,0.0],\"tags\":[]},".repeat(500))),
            ("empty_object".into(), "{}".into()),
            ("empty_array".into(), "[]".into()),
            ("scalar".into(), "  7  ".into()),
        ];
        // Strings just above size-class boundaries, repeated to ~4 MB: macOS
        // xzone counterexamples to a legacy-class price (32769 x 122 measured a
        // real peak of 6,001,312 B against the former 4,068,000 B bound).
        for len in [257usize, 513, 4_097, 16_385, 32_769, 65_537] {
            let n = (4_000_000 / len).max(1);
            let s = "a".repeat(len);
            inputs.push((format!("strings_{len}x{n}"), format!("[{}\"{s}\"]", format!("\"{s}\",").repeat(n - 1))));
        }
        // 20,000 keys of 129 B with values of 257 B.
        inputs.push((
            "object_k129_v257".into(),
            format!("{{{}}}", (0..20_000).map(|i| format!("\"{i:0>129}\":\"{}\"", "b".repeat(257))).collect::<Vec<_>>().join(",")),
        ));
        // Arrays at each capacity boundary.
        for n in [1usize, 3, 4, 5, 8, 9, 16, 17] {
            inputs.push((format!("array_{n}"), format!("[{}0]", "0,".repeat(n - 1))));
        }
        // Objects around B-tree split points.
        for n in [1usize, 5, 6, 11, 12, 23, 66, 67, 200] {
            inputs.push((format!("object_{n}"), format!("{{{}\"last\":0}}", (1..n).map(|i| format!("\"k{i:04}\":0,")).collect::<String>())));
            inputs.push((format!("object_desc_{n}"), format!("{{{}\"0000\":0}}", (1..n).rev().map(|i| format!("\"k{i:04}\":0,")).collect::<String>())));
        }
        inputs.into_iter().map(|(name, text)| (name, text.into_bytes())).collect()
    }

    #[test]
    fn allocation_envelope_is_the_checked_next_power_of_two_at_least_16() {
        assert_eq!(allocation_envelope(0), Some(0));
        assert_eq!(allocation_envelope(1), Some(16));
        assert_eq!(allocation_envelope(16), Some(16));
        assert_eq!(allocation_envelope(17), Some(32));
        // At or above every measured fresh macOS 26 xzone class.
        for (request, xzone_class) in [(257u64, 320u64), (728, 768), (1008, 1024), (4097, 5120), (32_769, 49_152), (131_073, 147_456)] {
            assert!(allocation_envelope(request).unwrap() >= xzone_class, "{request}");
        }
        assert_eq!(allocation_envelope(1 << 63), Some(1 << 63));
        assert_eq!(allocation_envelope((1 << 63) + 1), None);
        assert_eq!(allocation_envelope(u64::MAX), None);
        assert_eq!(wrapper_bytes(), 1024 + 16 + 16);
        assert_eq!(std::mem::size_of::<Value>() as u64, VALUE_BYTES);
        let bound = IngressBound { raw_len: 1025, materialization_bytes: 7 };
        assert_eq!(bound.lease_bytes(), Some(2048 + 7), "the raw body is priced by its envelope");
        assert_eq!(IngressBound { raw_len: u64::MAX, materialization_bytes: 0 }.lease_bytes(), None);
    }

    /// Fresh-allocation class table, checked directly on this host's
    /// allocator. Single-threaded within itself; it allocates and frees one
    /// block at a time.
    ///
    /// The `malloc_size <= envelope` check is limited to requests up to 32 KiB,
    /// which xzone serves from fixed size classes: a class block reports its
    /// class whether it is fresh or reused, so the check does not depend on the
    /// process's heap history or on concurrently running tests. Above 32 KiB
    /// blocks are page-granular and come from the large allocator, whose cache
    /// can hand back an already-held larger block that another test (or an
    /// earlier step of this one) freed; `malloc_size` of such a block can
    /// exceed the envelope (see [`allocation_envelope`]), so only fresh sizes
    /// would satisfy it and freshness cannot be arranged under parallel tests.
    /// Page-granular sizes are covered by
    /// `envelope_covers_the_resident_pages_of_page_granular_blocks` instead.
    #[cfg(target_os = "macos")]
    #[test]
    fn allocation_envelope_covers_this_hosts_malloc_size() {
        const SIZE_CLASS_LIMIT: usize = 32 * 1024;
        let mut loosest = 0.0f64;
        for size in 1..=SIZE_CLASS_LIMIT {
            // SAFETY: plain malloc/free of a non-zero size; the block is unused.
            let real = unsafe {
                let ptr = libc::malloc(size);
                assert!(!ptr.is_null());
                let real = libc::malloc_size(ptr);
                libc::free(ptr);
                real as u64
            };
            let envelope = allocation_envelope(size as u64).unwrap();
            assert!(real >= size as u64, "{size}: malloc_size {real}");
            assert!(real <= envelope, "{size}: malloc_size {real} exceeds envelope {envelope}");
            loosest = loosest.max(envelope as f64 / real as f64);
        }
        assert!(loosest <= 2.0, "loosest envelope/malloc_size {loosest:.3}");
        eprintln!("loosest envelope / malloc_size over size-class requests: {loosest:.4}");
    }

    /// Page-granular requests: every block (fresh or a reused cached one) is
    /// page-aligned and whole pages, so touching its s requested bytes makes at
    /// most round_up(s, page) bytes resident, and that is within the
    /// envelope for every s >= the page size. The alignment checks are
    /// properties of every block the large allocator returns, so they do not
    /// depend on heap history; the inequality is arithmetic.
    #[test]
    fn envelope_covers_the_resident_pages_of_page_granular_blocks() {
        const PAGE: u64 = 16 * 1024;
        let round_up = |s: u64| s.div_ceil(PAGE) * PAGE;
        // Exhaustive over [P, 2^24], then around every power of two up to 2^63.
        for s in PAGE..=(1 << 24) {
            assert!(round_up(s) <= allocation_envelope(s).unwrap(), "{s}");
        }
        for shift in 14..=63u32 {
            let p = 1u64 << shift;
            for s in [p - 1, p, p + 1] {
                if s >= PAGE {
                    if let Some(envelope) = allocation_envelope(s) {
                        assert!(s.div_ceil(PAGE) * PAGE <= envelope, "{s}");
                    }
                }
            }
        }
        // Below the page the inequality fails; those requests use size classes.
        assert!(round_up(100) > allocation_envelope(100).unwrap());
        #[cfg(target_os = "macos")]
        {
            // SAFETY: sysconf has no preconditions.
            let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64;
            assert!(page <= PAGE && PAGE.is_multiple_of(page), "page size {page}");
            let mut sizes: Vec<usize> = (32 * 1024 + 1..=96 * 1024).step_by(509).collect();
            sizes.extend((96 * 1024..=8 << 20).step_by(40_009));
            for size in sizes {
                // SAFETY: plain malloc/free of a non-zero size; the block is unused.
                let (address, real) = unsafe {
                    let ptr = libc::malloc(size);
                    assert!(!ptr.is_null());
                    let real = libc::malloc_size(ptr);
                    libc::free(ptr);
                    (ptr as usize as u64, real as u64)
                };
                assert_eq!(address % page, 0, "{size}: block not page-aligned");
                assert!(real >= size as u64 && real % page == 0, "{size}: malloc_size {real}");
            }
        }
    }

    #[test]
    fn prescan_never_allocates() {
        for (name, input) in adversarial_inputs() {
            let (outcome, measured) = measure(|| prescan(&input));
            assert!(outcome.is_ok(), "{name}: {outcome:?}");
            assert_eq!(measured.allocations, 0, "{name}: the prescan allocated");
        }
        let (outcome, measured) = measure(|| prescan(b"[1,2,"));
        assert!(outcome.is_err());
        assert_eq!(measured.allocations, 0);
    }

    /// The region prescan (the report-package request's regions) allocates
    /// nothing, has the prescan's bound and refusals, and attributes every
    /// priced allocation to exactly one part, so the parts and the fixed
    /// allowances sum to M.
    #[test]
    fn prescan_regions_allocates_nothing_and_its_parts_sum_to_the_bound() {
        use crate::command_ingress::REPORT_PACKAGE_REQUEST_REGIONS as REGIONS;
        let mut inputs = adversarial_inputs();
        inputs.push(("request_shaped".into(), format!(
            r#"{{"report":[{}0],"source_basis_refs":[{}[1]],"audit_manifest":["m",null,null,null,"SI",[[1]],[{{"a":[1]}}],null],"result_documents":[{}{{}}],"r\u0065port":{{"k":"\u00e9"}}}}"#,
            "[[0]],".repeat(2_000), "[1],".repeat(2_000), "{\"a\":[[1]]},".repeat(1_000)
        ).into_bytes()));
        inputs.push(("positional_request".into(), format!(r#"["p","e",[],[{}[]],{{}},[],[[]],[[1]],[[[2]]],null,"s"]"#, "[1],".repeat(3_000)).into_bytes()));
        for (name, input) in &inputs {
            let (outcome, measured) = measure(|| prescan_regions(input, &REGIONS));
            assert_eq!(measured.allocations, 0, "{name}: the region prescan allocated");
            let (bound, regions) = outcome.unwrap_or_else(|e| panic!("{name}: {e:?}"));
            assert_eq!(Ok(bound), prescan(input), "{name}");
            let parts = [regions.strings, regions.opaque, regions.typed, regions.neither, regions.allowances];
            assert_eq!(parts.iter().sum::<u64>(), bound.materialization_bytes, "{name}: {regions:?}");
            let (_, plain) = scan(input, None).unwrap();
            assert_eq!((plain.opaque, plain.typed), (0, 0), "{name}: no regions without a table");
            assert_eq!(plain.neither, regions.opaque + regions.typed + regions.neither, "{name}");
        }
        for invalid in [&b"[1,"[..], b"{\"report\":[1]", b"{\"\\x\":1}", br#"{"$serde_json::private::RawValue":"1"}"#] {
            let (outcome, measured) = measure(|| prescan_regions(invalid, &REGIONS));
            assert_eq!(measured.allocations, 0);
            assert_eq!(outcome.map(|(bound, _)| bound), prescan(invalid));
        }
    }

    #[test]
    fn prescan_agrees_with_serde_json_on_validity() {
        let cases: Vec<Vec<u8>> = vec![
            b"".to_vec(), b" ".to_vec(), b"[".to_vec(), b"]".to_vec(), b"[1,]".to_vec(), b"{\"a\":1,}".to_vec(),
            b"{\"a\" 1}".to_vec(), b"{1:1}".to_vec(), b"[01]".to_vec(), b"[-]".to_vec(), b"[1.]".to_vec(), b"[1e]".to_vec(),
            b"[.5]".to_vec(), b"[1.5e+7]".to_vec(), b"[-0]".to_vec(), b"[tru]".to_vec(), b"[truex]".to_vec(), b"nul".to_vec(),
            b"[\"a\nb\"]".to_vec(), b"[\"\\x\"]".to_vec(), b"[\"\\ud800\"]".to_vec(), b"[\"\\udc00\"]".to_vec(),
            b"[\"\\ud800\\u0041\"]".to_vec(), b"[\"\\ud83d\\ude00\"]".to_vec(), b"[\"\\u00G0\"]".to_vec(), b"1 2".to_vec(),
            b"{} ".to_vec(), b"\xef\xbb\xbf{}".to_vec(), b"[\"\xff\"]".to_vec(), b"[1]x".to_vec(), b"{\"a\":{}}".to_vec(),
            b"[[],[[]],{}]".to_vec(), b"\t\r\n[ 1 , 2 ]\n".to_vec(), b"[1e400]".to_vec(), b"[-1e400]".to_vec(),
            b"[123456789012345678901234567890]".to_vec(), b"[\"\\/\\b\\f\\r\\t\\\"\\\\\"]".to_vec(),
            nested(127, "[", "]", "").into_bytes(), nested(128, "[", "]", "").into_bytes(),
            nested(127, "{\"k\":", "}", "0").into_bytes(), nested(128, "{\"k\":", "}", "0").into_bytes(),
            nested(126, "[", "]", "{}").into_bytes(), nested(127, "[", "]", "{}").into_bytes(),
        ];
        for input in cases {
            let ours = prescan(&input).is_ok();
            let theirs = serde_json::from_slice::<Value>(&input).is_ok();
            // 1e400 is grammatical but out of f64 range: the prescan accepts and the
            // (covered) parse refuses.
            let out_of_range = input.windows(5).any(|w| w == b"1e400");
            if out_of_range {
                assert!(ours && !theirs);
            } else {
                assert_eq!(ours, theirs, "{}", String::from_utf8_lossy(&input));
            }
        }
    }

    #[test]
    fn prescan_refuses_raw_value_token_and_nesting_beyond_serde_json() {
        let token = br#"{"$serde_json::private::RawValue":"[1]"}"#;
        assert_eq!(prescan(token).unwrap_err().reason, "reserved RawValue token key");
        let escaped = br#"{"a":0,"\u0024serde_json::private::RawValue":"1"}"#;
        assert_eq!(prescan(escaped).unwrap_err().reason, "reserved RawValue token key");
        assert!(prescan(br#"{"$serde_json::private::RawValueX":"1"}"#).is_ok());
        assert!(prescan(br#"{"$serde_json::private::RawValu":"1"}"#).is_ok());
        assert!(prescan(nested(127, "[", "]", "").as_bytes()).is_ok());
        assert_eq!(prescan(nested(128, "[", "]", "").as_bytes()).unwrap_err().reason, "nesting deeper than 127 containers");
        assert!(serde_json::from_slice::<Value>(nested(128, "[", "]", "").as_bytes()).is_err(), "the parser's own limit is active");
    }

    /// The bound covers the peak live sum of `allocation_envelope` of every
    /// requested size during materialization (reallocations count old + new),
    /// measured by the counting allocator. The measure counts requested sizes
    /// only, so it is deterministic and independent of heap history.
    #[test]
    fn ingress_bound_covers_measured_materialization() {
        let mut tightest = f64::MAX;
        let mut loosest = 0.0f64;
        let mut loosest_name = String::new();
        let mut report = Vec::new();
        for (name, input) in adversarial_inputs() {
            let bound = prescan(&input).unwrap_or_else(|e| panic!("{name}: {e:?}"));
            let (parsed, measured) = measure(|| {
                // A grammatical but out-of-range float fails inside the parse; the
                // bound covers the partial tree and the error as well.
                match serde_json::from_slice::<Value>(&input) {
                    Ok(model) => drop(native_request(model)),
                    Err(error) => drop(error),
                }
            });
            let () = parsed;
            assert!(
                measured.peak_priced <= bound.materialization_bytes,
                "{name}: priced {measured:?} exceeds bound {}",
                bound.materialization_bytes
            );
            assert!(measured.peak_requested <= measured.peak_priced, "{name}");
            let ratio = bound.materialization_bytes as f64 / measured.peak_priced.max(1) as f64;
            report.push(format!(
                "{name}: bound {} priced peak {} requested peak {} bound/priced {ratio:.3}",
                bound.materialization_bytes, measured.peak_priced, measured.peak_requested,
            ));
            if measured.peak_priced >= 64 * 1024 {
                // Not absurdly loose where the content dominates the fixed allowances.
                assert!(ratio < 3.0, "{name}: ratio {ratio:.3}");
                tightest = tightest.min(ratio);
                if ratio > loosest {
                    loosest = ratio;
                    loosest_name = name.clone();
                }
            }
        }
        eprintln!("{}", report.join("\n"));
        eprintln!("content-dominated bound/priced peak: tightest {tightest:.3}, loosest {loosest:.3} ({loosest_name})");
    }

    #[test]
    fn float_allowance_covers_hard_float_parsing() {
        let floats = [
            format!("0.{}1e-300", "0".repeat(800)),
            format!("0.{}e-10", "7".repeat(900)),
            format!("{}e-10", "7".repeat(900)),
            format!("{}.{}e-500", "9".repeat(400), "9".repeat(400)),
            format!("{}e-700", "1".repeat(767)),
            format!("{}5e-1100", "4".repeat(780)),
            "2.4703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936316e-324".to_string(),
            "1.7976931348623157e308".to_string(),
            "123456789012345678901234567890".to_string(),
        ];
        let mut largest_used = 0;
        for text in floats {
            let bound = prescan(text.as_bytes()).unwrap().materialization_bytes;
            let (_, measured) = measure(|| serde_json::from_str::<Value>(&text).map(drop).map_err(drop));
            assert!(measured.peak_priced <= bound, "{text}: {measured:?} > {bound}");
            // How much of the float allowance this parse needed beyond everything else priced.
            let used = measured.peak_priced.saturating_sub(bound - FLOAT_PARSE_ALLOWANCE_BYTES);
            largest_used = largest_used.max(used);
            eprintln!("float of {} chars: priced peak {} bound {}", text.len(), measured.peak_priced, bound);
        }
        eprintln!("largest share of the float allowance used: {largest_used} of {FLOAT_PARSE_ALLOWANCE_BYTES} bytes");
    }
}
