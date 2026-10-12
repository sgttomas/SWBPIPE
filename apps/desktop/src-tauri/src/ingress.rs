//! Allocation-free ingress prescan for a solve request's raw body (DEL-04-07
//! memory-budget decision: "An ingress byte limit/reservation must precede
//! native Value materialization").
//!
//! [`prescan`] validates the raw bytes as exactly one JSON value under the
//! grammar `serde_json::from_slice::<Value>` accepts (UTF-8, structure, the
//! parser's recursion limit, no `RawValue` token key) and computes a sound
//! upper bound on the heap bytes that materializing it allocates: the
//! `serde_json::Value` tree (serde_json 1.0.149 with `std`, `float_roundtrip`,
//! `raw_value`, `unbounded_depth`; not `preserve_order`, not
//! `arbitrary_precision`), the parser's scratch buffer, one reallocation
//! transient, the float parser's bignum transients, an error allocation and the
//! native solve request wrapper ([`native_request`]). Every allocation is
//! priced by its allocation envelope ([`allocation_envelope`]), which does not
//! depend on the allocator's size-class table.
//!
//! The prescan itself does not allocate: it walks the bytes once with a
//! fixed-size stack of per-container counters.

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
/// slow path while one float is parsed. Measured peak on adversarial
/// hard-to-round floats is recorded by `ingress_bound_covers_measured_materialization`.
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

/// The usable bytes the allocator may hand back for a request of `size` bytes.
///
/// Assumption: the system allocator rounds a request of s >= 1 bytes to at
/// most the next power of two, and never below 16 bytes. This holds for
/// every class of macOS 26 xzone malloc (measured: 257 -> 320, 728 -> 768,
/// 1008 -> 1024, 4097 -> 5120, 32769 -> 49152, 131073 -> 147456; large
/// allocations are 16 KiB-page rounded) and for the legacy libmalloc
/// tiny/small/large classes, and is checked against `malloc_size` of every
/// live allocation by `ingress_bound_covers_measured_materialization`. It is
/// deliberately independent of the class table, which differs between
/// allocator generations. Looseness: for s > 8 the envelope is < 2s, so it is
/// < 2x any real class (every class is >= its request). `None` when the next
/// power of two overflows (the caller refuses the input).
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

    fn add_tree(&mut self, bytes: u64) -> Result<(), IngressInvalid> {
        let priced = allocation_envelope(bytes).ok_or(invalid("input too large to price", self.index))?;
        self.tree = self.tree.checked_add(priced).ok_or(invalid("input too large to price", self.index))?;
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
    /// Returns whether it decodes to the `RawValue` token.
    fn string(&mut self) -> Result<bool, IngressInvalid> {
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
                    }
                    decoded += ch.len_utf8() as u64;
                }
                0x00..=0x1f => return Err(invalid("control character in string", self.index - 1)),
                other => {
                    feed(other, &mut token_match, &mut token_pos);
                    decoded += 1;
                }
            }
        }
        self.add_tree(decoded)?;
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
                self.tree = self.tree.checked_add(bytes).ok_or(too_large)?;
            }
        } else if frame.count > 0 {
            // `Vec::new()` + `push`: capacities 4, 8, 16, ...
            let capacity = frame.count.checked_next_power_of_two().ok_or(too_large)?.max(4);
            let buffer = capacity.checked_mul(VALUE_BYTES).ok_or(too_large)?;
            self.add_tree(buffer)?;
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
    if let Err(error) = std::str::from_utf8(input) {
        return Err(invalid("invalid UTF-8", error.valid_up_to()));
    }
    let mut scan = Scan {
        bytes: input,
        index: 0,
        tree: 0,
        largest_array_transient: 0,
        largest_scratch: 0,
        float_path_seen: false,
    };
    let mut stack = [Frame { object: false, count: 0 }; MAX_NESTING];
    let mut depth = 0usize;
    let mut expect = Expect::Value;
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
                            stack[depth] = Frame { object: byte == b'{', count: 0 };
                            depth += 1;
                            expect = if byte == b'{' { Expect::KeyOrObjectEnd } else { Expect::ValueOrArrayEnd };
                        }
                        b'"' => {
                            scan.string()?;
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
                    if scan.string()? {
                        return Err(invalid("reserved RawValue token key", scan.index));
                    }
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
    let materialization_bytes = [scratch, scan.largest_array_transient, float, ERROR_ALLOWANCE_BYTES, wrapper_bytes()]
        .into_iter()
        .try_fold(scan.tree, u64::checked_add)
        .ok_or(too_large)?;
    Ok(IngressBound { raw_len: input.len() as u64, materialization_bytes })
}

#[cfg(test)]
pub(crate) mod counting {
    //! Per-thread allocation accounting for the ingress bound tests: tracks the
    //! live requested bytes, the live bytes priced by `allocation_envelope`
    //! and (macOS) the live real usable sizes reported by `malloc_size`.
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::cell::Cell;

    pub struct CountingAllocator;

    thread_local! {
        static ACTIVE: Cell<bool> = const { Cell::new(false) };
        static LIVE: Cell<i64> = const { Cell::new(0) };
        static PEAK: Cell<i64> = const { Cell::new(0) };
        static LIVE_PRICED: Cell<i64> = const { Cell::new(0) };
        static PEAK_PRICED: Cell<i64> = const { Cell::new(0) };
        static LIVE_REAL: Cell<i64> = const { Cell::new(0) };
        static PEAK_REAL: Cell<i64> = const { Cell::new(0) };
        static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    }

    fn price(size: usize) -> i64 {
        super::allocation_envelope(size as u64).unwrap_or(u64::MAX) as i64
    }

    /// The allocator's real usable size of a live block (0 where unknown).
    fn usable(ptr: *mut u8) -> i64 {
        #[cfg(target_os = "macos")]
        {
            // SAFETY: `ptr` is a live block returned by the system allocator.
            (unsafe { libc::malloc_size(ptr as *const libc::c_void) }) as i64
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = ptr;
            0
        }
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

    fn record(added: usize, removed: usize, counts_as_allocation: bool) {
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

    fn record_real(added: i64, removed: i64) {
        if active() {
            bump(&LIVE_REAL, &PEAK_REAL, added, removed);
        }
    }

    unsafe impl GlobalAlloc for CountingAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let ptr = unsafe { System.alloc(layout) };
            if !ptr.is_null() {
                record(layout.size(), 0, true);
                record_real(usable(ptr), 0);
            }
            ptr
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            let ptr = unsafe { System.alloc_zeroed(layout) };
            if !ptr.is_null() {
                record(layout.size(), 0, true);
                record_real(usable(ptr), 0);
            }
            ptr
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            let real = usable(ptr);
            unsafe { System.dealloc(ptr, layout) };
            record(0, layout.size(), false);
            record_real(0, real);
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            let old_real = usable(ptr);
            let new = unsafe { System.realloc(ptr, layout, new_size) };
            if !new.is_null() {
                // Priced: old and new buffers are both counted live across the move.
                record(new_size, layout.size(), true);
                // Real: a moved block coexisted with the old one; an in-place
                // resize changes the live size by the difference.
                let new_real = usable(new);
                if new == ptr {
                    record_real(new_real.saturating_sub(old_real).max(0), old_real.saturating_sub(new_real).max(0));
                } else {
                    record_real(new_real, old_real);
                }
            }
            new
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Measured {
        pub peak_requested: u64,
        pub peak_priced: u64,
        /// Peak live `malloc_size` total (macOS; 0 elsewhere).
        pub peak_real: u64,
        pub allocations: u64,
    }

    /// Runs `f` with this thread's accounting active from zero.
    pub fn measure<T>(f: impl FnOnce() -> T) -> (T, Measured) {
        for cell in [&LIVE, &PEAK, &LIVE_PRICED, &PEAK_PRICED, &LIVE_REAL, &PEAK_REAL] {
            cell.with(|c| c.set(0));
        }
        ALLOCATIONS.with(|c| c.set(0));
        ACTIVE.with(|a| a.set(true));
        let out = f();
        ACTIVE.with(|a| a.set(false));
        let measured = Measured {
            peak_requested: PEAK.with(Cell::get).max(0) as u64,
            peak_priced: PEAK_PRICED.with(Cell::get).max(0) as u64,
            peak_real: PEAK_REAL.with(Cell::get).max(0) as u64,
            allocations: ALLOCATIONS.with(Cell::get),
        };
        (out, measured)
    }
}

#[cfg(test)]
#[global_allocator]
static COUNTING_ALLOCATOR: counting::CountingAllocator = counting::CountingAllocator;

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
        // At or above every measured macOS 26 xzone class.
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

    /// The allocator's real usable sizes, checked directly on this host.
    #[cfg(target_os = "macos")]
    #[test]
    fn allocation_envelope_covers_this_hosts_malloc_size() {
        let mut loosest = 0.0f64;
        let mut sizes: Vec<usize> = (1..=70_000).collect();
        sizes.extend((70_000..=4 << 20).step_by(4_093));
        for size in sizes {
            // SAFETY: plain malloc/free of a non-zero size; the block is unused.
            let real = unsafe {
                let ptr = libc::malloc(size);
                assert!(!ptr.is_null());
                let real = libc::malloc_size(ptr);
                libc::free(ptr);
                real as u64
            };
            let envelope = allocation_envelope(size as u64).unwrap();
            assert!(real <= envelope, "{size}: malloc_size {real} exceeds envelope {envelope}");
            loosest = loosest.max(envelope as f64 / real as f64);
        }
        assert!(loosest <= 2.0, "loosest envelope/malloc_size {loosest:.3}");
        eprintln!("loosest envelope / malloc_size over sampled requests: {loosest:.4}");
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

    #[test]
    fn ingress_bound_covers_measured_materialization() {
        // On macOS the bound is checked against the real live usable sizes
        // (`malloc_size`); elsewhere against the envelope-priced peak.
        let real = cfg!(target_os = "macos");
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
            if real {
                assert!(measured.peak_real > 0 || measured.allocations == 0, "{name}: malloc_size recorded");
                assert!(
                    measured.peak_real <= bound.materialization_bytes,
                    "{name}: real malloc_size peak {} exceeds bound {}",
                    measured.peak_real,
                    bound.materialization_bytes
                );
            }
            let peak = if real { measured.peak_real } else { measured.peak_priced };
            let ratio = bound.materialization_bytes as f64 / peak.max(1) as f64;
            report.push(format!(
                "{name}: bound {} real peak {} priced peak {} requested peak {} bound/{} {ratio:.3}",
                bound.materialization_bytes,
                measured.peak_real,
                measured.peak_priced,
                measured.peak_requested,
                if real { "real" } else { "priced" },
            ));
            if peak >= 64 * 1024 {
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
        eprintln!("content-dominated bound/peak: tightest {tightest:.3}, loosest {loosest:.3} ({loosest_name})");
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
            if cfg!(target_os = "macos") {
                assert!(measured.peak_real <= bound, "{text}: real {measured:?} > {bound}");
            }
            // How much of the float allowance this parse needed beyond everything else priced.
            let used = measured.peak_priced.saturating_sub(bound - FLOAT_PARSE_ALLOWANCE_BYTES);
            largest_used = largest_used.max(used);
            eprintln!("float of {} chars: priced peak {} real peak {} bound {}", text.len(), measured.peak_priced, measured.peak_real, bound);
        }
        eprintln!("largest share of the float allowance used: {largest_used} of {FLOAT_PARSE_ALLOWANCE_BYTES} bytes");
    }
}
