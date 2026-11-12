//! Bounded private-stdio frames. The raw buffer, borrowed-field inspection,
//! delivered-number guard and `Value` parse are reserved before allocation.
//! This uses general interactive admission; no solver job/channel policy.

use super::{catalog::Limits, numeric};
use crate::command_ingress::{lease_bytes, ArgsDecoding};
use crate::ingress::{self, allocation_envelope, IngressBound};
use crate::memory_pool::Lease;
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{value::RawValue, Value};
use std::fmt;
use std::io::{BufRead, BufReader, Read};

pub const MAX_FRAME_BYTES: usize = 128 * 1024;
const READ_BUFFER_BYTES: usize = 8192;
const INSPECTION_ALLOWANCE: u64 = 64 * 1024;

#[derive(Debug)]
pub enum Error {
    Invalid(&'static str),
    Admission(SolveJobError),
}
impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => out.write_str(message),
            Self::Admission(_) => out.write_str("agent frame resource admission refused"),
        }
    }
}
impl std::error::Error for Error {}

/// Hold the lease through interpretation and while any moved arguments or
/// derived outbound/event values remain live; dispatchers may retain it in Arc.
pub struct AdmittedFrame {
    pub value: Value,
    pub lease: Lease,
    /// Some object in the frame had two members with the same decoded name.
    /// No `Value` was built (`value` is null): the caller must end the
    /// session as a protocol violation, never interpret the frame.
    pub duplicate_member: bool,
}

/// Walks a whole frame without building a `Value` and fails at the first
/// object holding two members with the same decoded name, escaped spellings
/// included. Keys are kept per object only while it is open, within the frame
/// byte cap and the parser's recursion limit; numbers, strings and the rest
/// are skipped. The flag tells a duplicate from other invalid JSON.
struct Unique<'a>(&'a std::cell::Cell<bool>);
impl<'de> DeserializeSeed<'de> for Unique<'_> {
    type Value = ();
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Unique<'_> {
    type Value = ();
    fn expecting(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str("JSON without duplicate object members")
    }
    fn visit_bool<E>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_i128<E>(self, _: i128) -> Result<(), E> {
        Ok(())
    }
    fn visit_u128<E>(self, _: u128) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_none<E>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while seq.next_element_seed(Unique(self.0))?.is_some() {}
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
        let mut keys = std::collections::HashSet::new();
        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key) {
                self.0.set(true);
                return Err(de::Error::custom("duplicate object member"));
            }
            map.next_value_seed(Unique(self.0))?;
        }
        Ok(())
    }
}

/// B + M in the shared moved-value lease, plus 3B + M + allowance. The extra
/// covers borrowed-header string/scratch/error transients, one numeric-token
/// buffer and its parse/render/normalization transients. Buffers are bounded by
/// raw length; exponents never expand a coefficient. Headers and numeric
/// scratch are destroyed before the final Value parse.
pub fn reservation_bytes(bound: IngressBound) -> Option<u64> {
    lease_bytes(bound, ArgsDecoding::Moved)?
        .checked_add(allocation_envelope(bound.raw_len)?.checked_mul(3)?)?
        .checked_add(bound.materialization_bytes)?
        .checked_add(INSPECTION_ALLOWANCE)
}

struct Header<'a> {
    method: Option<String>,
    params: Option<&'a RawValue>,
}
impl<'de> Deserialize<'de> for Header<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct HeaderVisitor;
        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header<'de>;
            fn expecting(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
                out.write_str("one RPC object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let (mut method, mut params) = (None, None);
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "method" => {
                            if method.is_some() {
                                return Err(de::Error::duplicate_field("method"));
                            }
                            method = Some(map.next_value::<String>()?);
                        }
                        "params" => {
                            if params.is_some() {
                                return Err(de::Error::duplicate_field("params"));
                            }
                            params = Some(map.next_value::<&RawValue>()?);
                        }
                        _ => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }
                Ok(Header { method, params })
            }
        }
        deserializer.deserialize_map(HeaderVisitor)
    }
}

struct Span(&'static str);
impl<'de> DeserializeSeed<'de> for Span {
    type Value = &'de RawValue;
    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        struct SpanVisitor(&'static str);
        impl<'de> Visitor<'de> for SpanVisitor {
            type Value = &'de RawValue;
            fn expecting(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
                out.write_str("an object with the bounded payload")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut span = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == self.0 {
                        if span.is_some() {
                            return Err(de::Error::duplicate_field(self.0));
                        }
                        span = Some(map.next_value::<&RawValue>()?);
                    } else {
                        map.next_value::<IgnoredAny>()?;
                    }
                }
                span.ok_or_else(|| de::Error::missing_field(self.0))
            }
        }
        deserializer.deserialize_map(SpanVisitor(self.0))
    }
}

fn reserve(raw: &[u8], registry: &SolveJobRegistry) -> Result<Lease, Error> {
    // This cap precedes the prescan, borrowed deserialization and every Value.
    if raw.len() > MAX_FRAME_BYTES {
        return Err(Error::Invalid("agent RPC frame byte cap exceeded"));
    }
    let bound = ingress::prescan(raw).map_err(|_| Error::Invalid("invalid agent JSON frame"))?;
    let bytes = reservation_bytes(bound).ok_or(Error::Invalid("agent frame price overflow"))?;
    registry
        .acquire_interactive_ingress(bytes)
        .map_err(Error::Admission)
}

fn inspect_span(raw: &str, field: &'static str, cap: u64) -> Result<(), Error> {
    let mut deserializer = serde_json::Deserializer::from_str(raw);
    let span = Span(field)
        .deserialize(&mut deserializer)
        .map_err(|_| Error::Invalid("invalid agent payload envelope"))?;
    if span.get().len() as u64 > cap {
        return Err(Error::Invalid("agent payload byte cap exceeded"));
    }
    Ok(())
}

pub fn decode(
    raw: &[u8],
    registry: &SolveJobRegistry,
    limits: Limits,
) -> Result<AdmittedFrame, Error> {
    let lease = reserve(raw, registry)?;
    numeric::check_admitted_json(raw)
        .map_err(|_| Error::Invalid("agent JSON number loses decimal identity"))?;
    // Before every other inspection, so a duplicate anywhere, top-level
    // members included, takes one route.
    let duplicate = std::cell::Cell::new(false);
    let mut deserializer = serde_json::Deserializer::from_slice(raw);
    if Unique(&duplicate).deserialize(&mut deserializer).is_err() {
        if duplicate.get() {
            return Ok(AdmittedFrame {
                value: Value::Null,
                lease,
                duplicate_member: true,
            });
        }
        return Err(Error::Invalid("invalid agent RPC value"));
    }
    {
        let header: Header<'_> = serde_json::from_slice(raw)
            .map_err(|_| Error::Invalid("invalid agent RPC envelope"))?;
        if header.method.as_deref() == Some("item/tool/call") {
            let params = header
                .params
                .ok_or(Error::Invalid("missing agent tool parameters"))?;
            inspect_span(params.get(), "arguments", limits.argument_bytes)?;
        }
    }
    let value =
        serde_json::from_slice(raw).map_err(|_| Error::Invalid("invalid agent RPC value"))?;
    Ok(AdmittedFrame {
        value,
        lease,
        duplicate_member: false,
    })
}

/// A Tauri completion prepass. The caller first applies raw_args_body's header
/// discipline, then drops this inspection lease and uses normal command ingress
/// for the moved arguments. No Value is built by this function.
pub fn inspect_completion(
    raw: &[u8],
    registry: &SolveJobRegistry,
    limits: Limits,
) -> Result<(), Error> {
    let _lease = reserve(raw, registry)?;
    numeric::check_admitted_json(raw)
        .map_err(|_| Error::Invalid("agent JSON number loses decimal identity"))?;
    let raw = std::str::from_utf8(raw)
        .map_err(|_| Error::Invalid("invalid agent completion encoding"))?;
    inspect_span(raw, "outcome", limits.result_bytes)
}

pub struct Reader<R: Read> {
    source: BufReader<R>,
    buffer: Vec<u8>,
    _buffer_lease: Lease,
    registry: SolveJobRegistry,
    limits: Limits,
    poisoned: bool,
}
impl<R: Read> Reader<R> {
    pub fn new(source: R, registry: SolveJobRegistry, limits: Limits) -> Result<Self, Error> {
        let bytes = allocation_envelope(MAX_FRAME_BYTES as u64)
            .and_then(|n| n.checked_add(allocation_envelope(READ_BUFFER_BYTES as u64)?))
            .and_then(|n| n.checked_add(4096))
            .ok_or(Error::Invalid("agent buffer price overflow"))?;
        let lease = registry
            .acquire_interactive_ingress(bytes)
            .map_err(Error::Admission)?;
        Ok(Self {
            source: BufReader::with_capacity(READ_BUFFER_BYTES, source),
            buffer: Vec::with_capacity(MAX_FRAME_BYTES),
            _buffer_lease: lease,
            registry,
            limits,
            poisoned: false,
        })
    }

    pub fn next(&mut self) -> Result<Option<AdmittedFrame>, Error> {
        if self.poisoned {
            return Err(Error::Invalid("agent frame reader closed"));
        }
        let result = self.read_inner();
        if result.is_err() {
            self.poisoned = true;
        }
        result
    }

    fn read_inner(&mut self) -> Result<Option<AdmittedFrame>, Error> {
        self.buffer.clear();
        loop {
            let chunk = self
                .source
                .fill_buf()
                .map_err(|_| Error::Invalid("agent pipe read failed"))?;
            if chunk.is_empty() {
                return if self.buffer.is_empty() {
                    Ok(None)
                } else {
                    Err(Error::Invalid("agent pipe ended with a partial frame"))
                };
            }
            let newline = chunk.iter().position(|b| *b == b'\n');
            let take = newline.map_or(chunk.len(), |n| n + 1);
            if self
                .buffer
                .len()
                .checked_add(take)
                .filter(|n| *n <= MAX_FRAME_BYTES)
                .is_none()
            {
                return Err(Error::Invalid("agent RPC frame byte cap exceeded"));
            }
            self.buffer.extend_from_slice(&chunk[..take]);
            self.source.consume(take);
            if newline.is_some() {
                return decode(&self.buffer, &self.registry, self.limits).map(Some);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingress::counting::measure;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::io::Cursor;

    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn limits() -> Limits {
        super::super::catalog::Catalog::load().unwrap().limits()
    }

    #[test]
    fn a_duplicate_member_anywhere_is_flagged_before_any_value_is_built() {
        let registry = registry();
        let limits = limits();
        let duplicates: [&[u8]; 10] = [
            br#"{"method":"item/started","params":{"item":{"id":"i","type":"commandExecution","type":"userMessage"}}}"#,
            br#"{"method":"item/started","params":{"item":{"type":"commandExecution"},"item":{"type":"userMessage"}}}"#,
            br#"{"method":"turn/completed","params":{"turn":{"id":"t","status":"completed","items":[{"type":"webSearch"}],"items":[]}}}"#,
            br#"{"method":"turn/completed","params":{"turn":{"items":[{"type":"webSearch"}]},"turn":{"id":"t","items":[]}}}"#,
            br#"{"method":"thread/started","params":{"thread":{"id":"t","turns":[{"items":[{"type":"fileChange"}]}],"turns":[]}}}"#,
            br#"{"method":"item/started","params":{"item":{"type":"fileChange"}},"params":{"item":{"type":"userMessage"}}}"#,
            br#"{"id":4,"result":{"data":[{"items":[{"type":"commandExecution"}]}]},"result":{}}"#,
            br#"{"method":"item/started","params":{"item":{"id":"i","t\u0079pe":"commandExecution","type":"userMessage"}}}"#,
            br#"{"method":"item/completed","params":{"item":{"id":"i","type":"agentMessage","text":"x","deep":[{"a":{"b":1,"\u0062":2}}]}}}"#,
            br#"{"id":1,"result":{"codexHome":"a","codexHome":"b"}}"#,
        ];
        for raw in duplicates {
            let frame = decode(raw, &registry, limits).unwrap();
            assert!(frame.duplicate_member, "{}", String::from_utf8_lossy(raw));
            assert!(frame.value.is_null());
        }
        // The same name in sibling or nested objects is not a duplicate.
        let distinct = br#"{"method":"turn/completed","params":{"turn":{"id":"t","status":"completed","items":[{"id":"a","type":"userMessage"},{"id":"b","type":"reasoning"}]},"threadId":"x"}}"#;
        let frame = decode(distinct, &registry, limits).unwrap();
        assert!(!frame.duplicate_member);
        assert_eq!(frame.value["params"]["turn"]["items"][1]["type"], "reasoning");
        // Other invalid JSON stays a framing error, not a duplicate.
        assert!(decode(br#"{"id":1,"result":{"a":}}"#, &registry, limits).is_err());
    }

    #[test]
    fn frame_cap_precedes_admission_and_value_allocation() {
        let registry = registry();
        let limits = limits();
        let raw = vec![b' '; MAX_FRAME_BYTES + 1];
        let (result, measured) = measure(|| decode(&raw, &registry, limits));
        assert!(matches!(
            result,
            Err(Error::Invalid("agent RPC frame byte cap exceeded"))
        ));
        assert_eq!(registry.pool().reserved_total(), 0);
        assert_eq!(measured.allocations, 0);
    }

    #[test]
    fn arguments_are_capped_without_materializing_their_string() {
        let registry = registry();
        let limits = limits();
        let raw = format!(
            r#"{{"id":1,"method":"item/tool/call","params":{{"arguments":"{}"}}}}"#,
            "x".repeat(limits.argument_bytes as usize + 1)
        );
        let (result, measured) = measure(|| decode(raw.as_bytes(), &registry, limits));
        assert!(matches!(
            result,
            Err(Error::Invalid("agent payload byte cap exceeded"))
        ));
        assert!(measured.peak_priced < limits.argument_bytes);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn numeric_failure_releases_and_valid_value_retains_lease() {
        let registry = registry();
        let limits = limits();
        assert!(decode(br#"{"n":0.9999999999999999999}"#, &registry, limits).is_err());
        assert_eq!(registry.pool().reserved_total(), 0);
        let frame = decode(br#"{"id":1,"result":{}}"#, &registry, limits).unwrap();
        assert_eq!(frame.value["id"], 1);
        assert!(registry.pool().reserved_total() >= frame.lease.bytes());
        drop(frame);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn completion_inspection_has_no_value_and_releases_lease() {
        let registry = registry();
        let limits = limits();
        let raw = format!(
            r#"{{"requestRef":"ref","outcome":"{}"}}"#,
            "x".repeat(limits.result_bytes as usize + 1)
        );
        let (result, measured) = measure(|| inspect_completion(raw.as_bytes(), &registry, limits));
        assert!(result.is_err());
        assert!(measured.peak_priced < limits.result_bytes);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn reader_refuses_partial_or_oversize_and_cannot_resume_suffix() {
        let registry = registry();
        {
            let mut reader =
                Reader::new(Cursor::new(b"{\"id\":1}"), registry.clone(), limits()).unwrap();
            assert!(reader.next().is_err());
            assert!(reader.next().is_err());
        }
        let raw = vec![b' '; MAX_FRAME_BYTES + READ_BUFFER_BYTES * 2];
        let mut reader = Reader::new(Cursor::new(raw), registry.clone(), limits()).unwrap();
        assert!(reader.next().is_err());
        assert!(reader.source.get_ref().position() <= (MAX_FRAME_BYTES + READ_BUFFER_BYTES) as u64);
        assert!(reader.next().is_err());
        drop(reader);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn parse_and_inspection_peaks_fit_the_reservation() {
        let registry = registry();
        let limits = limits();
        let cases = [
            format!(
                r#"{{"method":"{}","params":{{"text":"{}"}}}}"#,
                "\\u006d".repeat(3000),
                "\\u0078".repeat(6000)
            ),
            format!(
                r#"{{"method":"notice","params":{{"values":[{}]}}}}"#,
                vec!["{}"; 7000].join(",")
            ),
            format!(
                r#"{{"method":"notice","params":{{"n":0.{}1}}}}"#,
                "0".repeat(16000)
            ),
        ];
        for raw in cases {
            let bound = ingress::prescan(raw.as_bytes()).unwrap();
            let budget = reservation_bytes(bound).unwrap();
            let (result, measured) = measure(|| decode(raw.as_bytes(), &registry, limits));
            assert!(
                measured.peak_priced <= budget,
                "{} > {budget}",
                measured.peak_priced
            );
            drop(result);
            assert_eq!(registry.pool().reserved_total(), 0);
        }
    }
}
