//! Ingress discipline for the model-sized interactive commands (DEL-04-07
//! memory-budget decision: "An ingress byte limit/reservation must precede
//! native Value materialization").
//!
//! Tauri parses a JSON invoke body into a `serde_json::Value` before any
//! command runs, unpriced. Every command whose arguments can carry a model,
//! result, report, rule pack or library document therefore takes the raw body
//! ([`tauri::ipc::Request`]) and goes through [`admit`]:
//!
//! 1. The body must be raw bytes sent with header
//!    `x-swbpipe-ingress: args-json-v1` ([`INGRESS_HEADER`],
//!    [`INGRESS_ARGS_JSON_V1`]): the UTF-8 JSON of one object holding the
//!    same camelCase argument keys a JSON invoke carried. A JSON body, or a
//!    missing or unknown header, is refused `INVALID_REQUEST` (Tauri has
//!    already materialized a JSON body unpriced).
//! 2. [`ingress::prescan`] validates the bytes without allocating and bounds
//!    their `Value` materialization M; it refuses the `RawValue` token key and
//!    nesting beyond serde_json's limit (`RESOURCE_UNAVAILABLE` /
//!    `ingress_invalid`). A top-level value other than an object is
//!    `INVALID_REQUEST`.
//! 3. An interactive ingress lease of [`lease_bytes`] is admitted before any
//!    deserialization ([`crate::memory_pool::AdmissionClass::Interactive`]:
//!    pool, fresh headroom, lease within the per-invocation budget; the
//!    pressure state does not gate it).
//! 4. The body is parsed into a `Value` (within M) and the arguments are
//!    moved out of it ([`Args`]). Typed arguments are decoded from their
//!    `Value` by [`ValueDecoder`], which moves strings, gives sequences no
//!    size hint and formats every error into at most
//!    [`MAX_ARGS_ERROR_MESSAGE_BYTES`] (serde_json's own errors would echo a
//!    whole string argument into the message).
//! 5. The command runs, and its response is serialized ([`respond`]), while
//!    the lease is held; the lease is released when the command returns
//!    (RAII, also while unwinding).
//!
//! Lease rule ([`ArgsDecoding`]): envelope(raw length) + D + the error
//! allowance, where D = M when the arguments are moved out of the parsed
//! `Value` (every command except `save_report_package`), and
//! D = (2 + vec_expansion) * M when a typed request rebuilds `Value` fields
//! and builds typed vectors (`save_report_package`, see
//! [`REPORT_PACKAGE_VEC_EXPANSION`]). The counting-allocator tests check the
//! requested peak against these rules for representative and adversarial
//! inputs.
//!
//! Residual: the command's own working memory and its response serialization
//! run under the held lease but are not separately bounded; wry/Tauri copies
//! of the raw body made before the command runs are not ours to price (as
//! for the solve start).

use crate::ingress::{self, allocation_envelope, IngressBound};
use crate::memory_pool::{Lease, ResourceRefusal};
use crate::solve_jobs::{SolveJobError, SolveJobRegistry};
use serde::de::{self, Deserialize, DeserializeOwned, DeserializeSeed, IntoDeserializer, Unexpected, Visitor};
use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt::{self, Write as _};
use tauri::http::HeaderMap;
use tauri::ipc::{InvokeBody, Response};

/// The request header naming the args encoding.
pub(crate) const INGRESS_HEADER: &str = "x-swbpipe-ingress";
/// The one current args encoding: raw UTF-8 JSON of the args object.
pub(crate) const INGRESS_ARGS_JSON_V1: &str = "args-json-v1";
/// The longest argument-decoding error message (one allocation of exactly
/// this capacity).
pub(crate) const MAX_ARGS_ERROR_MESSAGE_BYTES: usize = 240;
/// Allocations of a refused argument: the bounded decoding error, the
/// `INVALID_REQUEST` message built from it and their transients. Checked by
/// the counting-allocator tests.
pub(crate) const ARGS_ERROR_ALLOWANCE_BYTES: u64 = 4096;

/// How a command's arguments are decoded, which fixes its lease rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArgsDecoding {
    /// Arguments are `Value`s, strings, scalars or structs of strings and
    /// scalars, moved out of the parsed body: decoding allocates nothing
    /// beyond M except a refused argument's error.
    Moved,
    /// A typed request with `Value` fields and vectors of structs: decoding
    /// rebuilds the `Value` fields (at most M more: new map nodes and array
    /// buffers, strings moved) and builds typed vectors (at most
    /// `vec_expansion` * M).
    Rebuilt { vec_expansion: u64 },
}

/// The interactive ingress lease for a prescanned body: envelope(raw) + D +
/// [`ARGS_ERROR_ALLOWANCE_BYTES`] (see the module docs); `None` on overflow.
pub(crate) fn lease_bytes(bound: IngressBound, decoding: ArgsDecoding) -> Option<u64> {
    let materialization = bound.materialization_bytes;
    let decoded = match decoding {
        ArgsDecoding::Moved => materialization,
        ArgsDecoding::Rebuilt { vec_expansion } => materialization.checked_mul(vec_expansion.checked_add(2)?)?,
    };
    allocation_envelope(bound.raw_len)?.checked_add(decoded)?.checked_add(ARGS_ERROR_ALLOWANCE_BYTES)
}

/// A command's error: an ingress error (the typed object) or the command's
/// own error, serialized exactly as before (a plain string for the commands
/// that returned `Result<_, String>`).
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub(crate) enum CommandError<E = String> {
    Ingress(SolveJobError),
    Command(E),
    /// The response could not be serialized (Tauri reports this as a plain
    /// string as well).
    Response(String),
}

impl<E> From<SolveJobError> for CommandError<E> {
    fn from(error: SolveJobError) -> Self {
        Self::Ingress(error)
    }
}

/// The command error of a command that returned a bare value: only ingress
/// errors can occur.
#[derive(Debug, Serialize)]
pub(crate) enum NoCommandError {}

fn invalid_request(message: String) -> SolveJobError {
    SolveJobError::InvalidRequest { message }
}

/// Admits a command's raw invoke body (see the module docs) and returns its
/// arguments and the ingress lease, which the caller holds until it returns.
pub(crate) fn admit(
    registry: &SolveJobRegistry,
    request: &tauri::ipc::Request<'_>,
    command: &'static str,
    decoding: ArgsDecoding,
) -> Result<(Args, Lease), SolveJobError> {
    admit_body(registry, request.body(), request.headers(), command, decoding)
}

/// [`admit`] on a body and headers (testable without Tauri).
pub(crate) fn admit_body(
    registry: &SolveJobRegistry,
    body: &InvokeBody,
    headers: &HeaderMap,
    command: &'static str,
    decoding: ArgsDecoding,
) -> Result<(Args, Lease), SolveJobError> {
    match headers.get(INGRESS_HEADER) {
        None => {
            return Err(invalid_request(format!(
                "INGRESS-HEADER-REQUIRED: `{command}` needs header `{INGRESS_HEADER}: {INGRESS_ARGS_JSON_V1}` and a raw body."
            )))
        }
        Some(value) if value.as_bytes() != INGRESS_ARGS_JSON_V1.as_bytes() => {
            return Err(invalid_request(format!(
                "INGRESS-HEADER-UNKNOWN: `{command}` accepts only `{INGRESS_HEADER}: {INGRESS_ARGS_JSON_V1}`."
            )))
        }
        Some(_) => {}
    }
    let InvokeBody::Raw(raw) = body else {
        return Err(invalid_request(format!(
            "INGRESS-RAW-BODY-REQUIRED: send `{command}`'s arguments as raw UTF-8 JSON bytes."
        )));
    };
    let bound = ingress::prescan(raw).map_err(|invalid| {
        registry.refuse(ResourceRefusal::IngressInvalid { reason: format!("{} at byte {}", invalid.reason, invalid.offset) })
    })?;
    if raw.iter().find(|byte| !matches!(byte, b' ' | b'\n' | b'\t' | b'\r')) != Some(&b'{') {
        return Err(invalid_request(format!("INGRESS-ARGS-NOT-OBJECT: `{command}`'s arguments must be one JSON object.")));
    }
    // The reservation precedes any deserialization.
    let lease = registry.acquire_interactive_ingress(lease_bytes(bound, decoding).unwrap_or(u64::MAX))?;
    match serde_json::from_slice::<Value>(raw) {
        Ok(Value::Object(map)) => Ok((Args { map, command }, lease)),
        Ok(_) => {
            drop(lease);
            Err(invalid_request(format!("INGRESS-ARGS-NOT-OBJECT: `{command}`'s arguments must be one JSON object.")))
        }
        // Only a grammatical number out of f64 range passes the prescan and fails here.
        Err(error) => {
            drop(lease);
            Err(registry.refuse(ResourceRefusal::IngressInvalid { reason: error.to_string() }))
        }
    }
}

/// [`admit`] for a command whose arguments are small by contract: a raw body
/// longer than `max_raw_bytes` is refused `INVALID_REQUEST`
/// (`INGRESS-ARGS-TOO-LARGE`) before the prescan and before any lease. The
/// model-state commands (W-12 S1) use it with their own budgets.
pub(crate) fn admit_within(
    registry: &SolveJobRegistry,
    request: &tauri::ipc::Request<'_>,
    command: &'static str,
    decoding: ArgsDecoding,
    max_raw_bytes: usize,
) -> Result<(Args, Lease), SolveJobError> {
    admit_body_within(registry, request.body(), request.headers(), command, decoding, max_raw_bytes)
}

/// [`admit_within`] on a body and headers (testable without Tauri).
pub(crate) fn admit_body_within(
    registry: &SolveJobRegistry,
    body: &InvokeBody,
    headers: &HeaderMap,
    command: &'static str,
    decoding: ArgsDecoding,
    max_raw_bytes: usize,
) -> Result<(Args, Lease), SolveJobError> {
    if let InvokeBody::Raw(raw) = body {
        if raw.len() > max_raw_bytes {
            return Err(invalid_request(format!(
                "INGRESS-ARGS-TOO-LARGE: `{command}` takes at most {max_raw_bytes} bytes of arguments; {} were sent.",
                raw.len()
            )));
        }
    }
    admit_body(registry, body, headers, command, decoding)
}

/// Serializes a command's outcome while its ingress lease is still held.
pub(crate) fn respond<T: Serialize, E>(outcome: Result<T, CommandError<E>>) -> Result<Response, CommandError<E>> {
    let value = outcome?;
    serde_json::to_string(&value).map(Response::new).map_err(|error| CommandError::Response(error.to_string()))
}

/// A command's arguments, moved out of the parsed body by key. Missing keys
/// and `null` behave as Tauri's JSON arguments did: a missing key is `None`
/// for an optional argument and an error otherwise; extra keys are ignored.
#[derive(Debug)]
pub(crate) struct Args {
    map: Map<String, Value>,
    command: &'static str,
}

impl Args {
    fn refused(&self, key: &str, error: ArgsError) -> SolveJobError {
        invalid_request(format!("INGRESS-ARGS-INVALID: invalid args `{key}` for command `{}`: {error}", self.command))
    }

    /// A required `Value` argument (`null` included), moved.
    pub(crate) fn value(&mut self, key: &'static str) -> Result<Value, SolveJobError> {
        match self.map.remove(key) {
            Some(value) => Ok(value),
            None => Err(self.refused(key, de::Error::custom(format_args!("missing required key {key}")))),
        }
    }

    /// An optional `Value` argument, moved: `None` when missing or `null`.
    pub(crate) fn opt_value(&mut self, key: &'static str) -> Option<Value> {
        self.map.remove(key).filter(|value| !value.is_null())
    }

    /// A typed argument decoded from its `Value` by [`ValueDecoder`]. A
    /// missing key decodes as `None` for an `Option` and is an error otherwise.
    pub(crate) fn decode<T: DeserializeOwned>(&mut self, key: &'static str) -> Result<T, SolveJobError> {
        let decoded = match self.map.remove(key) {
            Some(value) => T::deserialize(ValueDecoder(value)),
            None => T::deserialize(MissingKey(key)),
        };
        decoded.map_err(|error| self.refused(key, error))
    }

    /// A typed argument decoded by `decode` from its moved `Value`.
    pub(crate) fn decode_with<T>(
        &mut self,
        key: &'static str,
        decode: impl FnOnce(Value) -> Result<T, ArgsError>,
    ) -> Result<T, SolveJobError> {
        let value = self.value(key)?;
        decode(value).map_err(|error| self.refused(key, error))
    }
}

/// A struct's fields moved out of its `Value` without rebuilding the
/// documents they carry (for request structs with `Value` fields).
#[derive(Debug)]
pub(crate) struct Fields {
    map: Map<String, Value>,
}

impl Fields {
    pub(crate) fn of(value: Value, expecting: &'static str) -> Result<Self, ArgsError> {
        match value {
            Value::Object(map) => Ok(Self { map }),
            other => Err(de::Error::invalid_type(unexpected(&other), &expecting)),
        }
    }

    pub(crate) fn required_value(&mut self, field: &'static str) -> Result<Value, ArgsError> {
        self.map.remove(field).ok_or_else(|| de::Error::missing_field(field))
    }

    /// A `#[serde(default)] Value` field: `null` when missing.
    pub(crate) fn value_or_null(&mut self, field: &'static str) -> Value {
        self.map.remove(field).unwrap_or(Value::Null)
    }

    pub(crate) fn required<T: DeserializeOwned>(&mut self, field: &'static str) -> Result<T, ArgsError> {
        T::deserialize(ValueDecoder(self.required_value(field)?))
    }

    /// An optional field: `None` when missing or `null`.
    pub(crate) fn optional<T: DeserializeOwned>(&mut self, field: &'static str) -> Result<Option<T>, ArgsError> {
        match self.map.remove(field) {
            None => Ok(None),
            Some(value) => Option::<T>::deserialize(ValueDecoder(value)),
        }
    }
}

/// An argument-decoding error whose message is formatted into one buffer of
/// at most [`MAX_ARGS_ERROR_MESSAGE_BYTES`] (longer messages are truncated).
#[derive(Debug)]
pub(crate) struct ArgsError(String);

struct Truncating<'a>(&'a mut String);

impl fmt::Write for Truncating<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let room = MAX_ARGS_ERROR_MESSAGE_BYTES - self.0.len();
        if text.len() <= room {
            self.0.push_str(text);
            return Ok(());
        }
        let mut end = room;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        self.0.push_str(&text[..end]);
        // Stops the formatting: nothing more fits.
        Err(fmt::Error)
    }
}

impl de::Error for ArgsError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        let mut text = String::with_capacity(MAX_ARGS_ERROR_MESSAGE_BYTES);
        let _ = write!(Truncating(&mut text), "{message}");
        Self(text)
    }
}

impl fmt::Display for ArgsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ArgsError {}

fn unexpected(value: &Value) -> Unexpected<'_> {
    match value {
        Value::Null => Unexpected::Unit,
        Value::Bool(b) => Unexpected::Bool(*b),
        Value::Number(n) => match (n.as_u64(), n.as_i64()) {
            (Some(u), _) => Unexpected::Unsigned(u),
            (None, Some(i)) => Unexpected::Signed(i),
            _ => Unexpected::Float(n.as_f64().unwrap_or(f64::NAN)),
        },
        Value::String(s) => Unexpected::Str(s),
        Value::Array(_) => Unexpected::Seq,
        Value::Object(_) => Unexpected::Map,
    }
}

/// A missing argument key: `None` for an `Option`, an error otherwise.
struct MissingKey(&'static str);

impl<'de> de::Deserializer<'de> for MissingKey {
    type Error = ArgsError;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, ArgsError> {
        Err(de::Error::custom(format_args!("missing required key {}", self.0)))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        visitor.visit_none()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct
        newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}

/// Decodes a typed value from an owned `Value` with serde_json's `Value`
/// semantics, except that strings and map keys are moved (not copied),
/// sequences and maps report no size hint (a `Vec<T>` then allocates only as
/// elements decode), and errors are [`ArgsError`]s.
pub(crate) struct ValueDecoder(pub(crate) Value);

impl<'de> de::Deserializer<'de> for ValueDecoder {
    type Error = ArgsError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        match self.0 {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Number(n) => match (n.as_u64(), n.as_i64()) {
                (Some(u), _) => visitor.visit_u64(u),
                (None, Some(i)) => visitor.visit_i64(i),
                _ => visitor.visit_f64(n.as_f64().unwrap_or(f64::NAN)),
            },
            Value::String(s) => visitor.visit_string(s),
            Value::Array(items) => {
                let len = items.len();
                let mut seq = SeqDecoder(items.into_iter());
                let value = visitor.visit_seq(&mut seq)?;
                if seq.0.len() == 0 {
                    Ok(value)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in array"))
                }
            }
            Value::Object(map) => {
                let len = map.len();
                let mut entries = MapDecoder { iter: map.into_iter(), value: None };
                let value = visitor.visit_map(&mut entries)?;
                if entries.iter.len() == 0 {
                    Ok(value)
                } else {
                    Err(de::Error::invalid_length(len, &"fewer elements in map"))
                }
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        match self.0 {
            Value::Null => visitor.visit_none(),
            other => visitor.visit_some(ValueDecoder(other)),
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, ArgsError> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ArgsError> {
        match self.0 {
            Value::String(variant) => visitor.visit_enum(variant.into_deserializer()),
            Value::Object(map) => {
                let mut entries = map.into_iter();
                match (entries.next(), entries.next()) {
                    (Some((variant, value)), None) => visitor.visit_enum(EnumDecoder { variant, value }),
                    _ => Err(de::Error::invalid_value(Unexpected::Map, &"map with a single key")),
                }
            }
            other => Err(de::Error::invalid_type(unexpected(&other), &"string or map")),
        }
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        drop(self);
        visitor.visit_unit()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct
        seq tuple tuple_struct map struct identifier
    }
}

struct SeqDecoder(std::vec::IntoIter<Value>);

impl<'de> de::SeqAccess<'de> for SeqDecoder {
    type Error = ArgsError;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, ArgsError> {
        match self.0.next() {
            Some(value) => seed.deserialize(ValueDecoder(value)).map(Some),
            None => Ok(None),
        }
    }
    // No size hint: a typed vector grows only as its elements decode.
}

struct MapDecoder {
    iter: serde_json::map::IntoIter,
    value: Option<Value>,
}

impl<'de> de::MapAccess<'de> for MapDecoder {
    type Error = ArgsError;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, ArgsError> {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(KeyDecoder(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, ArgsError> {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDecoder(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

/// A map key, moved.
struct KeyDecoder(String);

impl<'de> de::Deserializer<'de> for KeyDecoder {
    type Error = ArgsError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        visitor.visit_string(self.0)
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ArgsError> {
        visitor.visit_some(self)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, ArgsError> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, ArgsError> {
        visitor.visit_enum(self.0.into_deserializer())
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct
        seq tuple tuple_struct map struct identifier ignored_any
    }
}

struct EnumDecoder {
    variant: String,
    value: Value,
}

impl<'de> de::EnumAccess<'de> for EnumDecoder {
    type Error = ArgsError;
    type Variant = VariantDecoder;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, VariantDecoder), ArgsError> {
        let variant = seed.deserialize(KeyDecoder(self.variant))?;
        Ok((variant, VariantDecoder(self.value)))
    }
}

struct VariantDecoder(Value);

impl<'de> de::VariantAccess<'de> for VariantDecoder {
    type Error = ArgsError;

    fn unit_variant(self) -> Result<(), ArgsError> {
        match self.0 {
            Value::Null => Ok(()),
            other => Err(de::Error::invalid_type(unexpected(&other), &"unit variant")),
        }
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, ArgsError> {
        seed.deserialize(ValueDecoder(self.0))
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value, ArgsError> {
        match self.0 {
            value @ Value::Array(_) => de::Deserializer::deserialize_any(ValueDecoder(value), visitor),
            other => Err(de::Error::invalid_type(unexpected(&other), &"tuple variant")),
        }
    }

    fn struct_variant<V: Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, ArgsError> {
        match self.0 {
            value @ (Value::Object(_) | Value::Array(_)) => de::Deserializer::deserialize_any(ValueDecoder(value), visitor),
            other => Err(de::Error::invalid_type(unexpected(&other), &"struct variant")),
        }
    }
}

/// `vec_expansion` for `save_report_package`'s typed request
/// (`open_pipe_stress_report_package::wire::ReportPackageRequest`): decoded
/// by [`ValueDecoder`] (no size hints), each `Vec<X>` of a struct X starts
/// allocating only when its first element has decoded and then grows by
/// doubling, so with k >= 1 decoded elements it holds a buffer of cap * S_X
/// (cap = max(4, next_power_of_two(k)); S_X = size_of X) and, while growing,
/// the previous buffer. Its k elements came from a JSON array priced by the
/// prescan at cap_v * 32 (cap_v >= cap) plus each element's own containers,
/// at least c_X (the cheapest container shape X decodes from: an array of its
/// positional fields or an object of its required fields, recursively over
/// its required inline struct fields; strings and nested vectors are priced
/// separately). The ratio of the vector's priced bytes to that priced share,
/// maximized over cap and the fewest elements cap admits, is at most this
/// constant for every vector type the request contains (checked by
/// `report_package_vec_expansion_bounds_every_vector_type`); the shares of
/// different vectors are disjoint parts of M. The worst type is
/// `QuantityResultDto` (S = 584 B, c = 1024 B: ratio 3.556 at k = 1).
pub(crate) const REPORT_PACKAGE_VEC_EXPANSION: u64 = 4;

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::memory_pool::test_host::ScriptedHost;
    use crate::memory_pool::MemoryPool;
    use std::sync::Arc;
    use std::time::Duration;

    pub(crate) fn registry(host: Arc<ScriptedHost>) -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::with_wait_tick(host, Duration::from_millis(5)))
    }

    pub(crate) fn headers(value: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        if let Some(value) = value {
            headers.insert(INGRESS_HEADER, value.parse().unwrap());
        }
        headers
    }

    /// A raw args body with the current header.
    pub(crate) fn admit_json(
        registry: &SolveJobRegistry,
        command: &'static str,
        decoding: ArgsDecoding,
        args: &Value,
    ) -> Result<(Args, Lease), SolveJobError> {
        let body = InvokeBody::Raw(serde_json::to_vec(args).unwrap());
        admit_body(registry, &body, &headers(Some(INGRESS_ARGS_JSON_V1)), command, decoding)
    }

    /// Checks the lease rule for one raw body: the requested peak (each live
    /// allocation priced by `allocation_envelope`, as the prescan bound
    /// prices them) of parsing the body and running `decode` on its args is
    /// within the lease less the buffered body's envelope. `decode` must
    /// drop what it decodes (or return it), as a command would.
    pub(crate) fn check_decode_bound<T>(
        raw: &[u8],
        decoding: ArgsDecoding,
        decode: impl FnOnce(&mut Args) -> T,
    ) -> (T, ingress::counting::Measured, u64) {
        let bound = ingress::prescan(raw).expect("prescan");
        let lease = lease_bytes(bound, decoding).unwrap();
        let available = lease - allocation_envelope(raw.len() as u64).unwrap();
        let (out, measured) = ingress::counting::measure(|| {
            let Value::Object(map) = serde_json::from_slice::<Value>(raw).unwrap() else { panic!("args object") };
            let mut args = Args { map, command: "test" };
            decode(&mut args)
        });
        assert!(
            measured.peak_priced <= available,
            "priced peak {} exceeds the lease's decoding share {available} (M = {})",
            measured.peak_priced,
            bound.materialization_bytes
        );
        (out, measured, available)
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::memory_pool::test_host::{ScriptedHost, GIB};
    use crate::memory_pool::{LeasePurpose, PressureLevel};
    use serde_json::json;

    const COMMAND: &str = "validate_model_operation";

    fn args_body() -> Value {
        json!({"model": {"nodes": [1, 2, 3], "name": "n"}, "intent": {"kind": "edit"}, "claimedModelHash": null})
    }

    fn refusal(error: &SolveJobError) -> &ResourceRefusal {
        error.refusal().unwrap_or_else(|| panic!("not a resource refusal: {error:?}"))
    }

    fn invalid_message(error: &SolveJobError) -> &str {
        match error {
            SolveJobError::InvalidRequest { message } => message,
            other => panic!("not INVALID_REQUEST: {other:?}"),
        }
    }

    #[test]
    fn raw_body_admits_moves_args_and_releases_the_lease_on_drop() {
        let registry = registry(ScriptedHost::normal());
        let body = serde_json::to_vec(&args_body()).unwrap();
        let bound = ingress::prescan(&body).unwrap();
        let (mut args, lease) = admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).unwrap();
        assert_eq!(lease.bytes(), lease_bytes(bound, ArgsDecoding::Moved).unwrap());
        assert_eq!(
            lease.bytes(),
            allocation_envelope(body.len() as u64).unwrap() + bound.materialization_bytes + ARGS_ERROR_ALLOWANCE_BYTES
        );
        assert_eq!(registry.pool().reserved(LeasePurpose::Ingress), lease.bytes());
        assert_eq!(args.value("model").unwrap(), json!({"nodes": [1, 2, 3], "name": "n"}));
        assert_eq!(args.opt_value("claimedModelHash"), None, "null is None");
        assert_eq!(args.opt_value("absent"), None);
        assert_eq!(args.decode::<Option<String>>("absent").unwrap(), None);
        let missing = args.value("model").unwrap_err();
        assert_eq!(
            invalid_message(&missing),
            "INGRESS-ARGS-INVALID: invalid args `model` for command `validate_model_operation`: missing required key model"
        );
        assert!(args.decode::<String>("absent").is_err());
        drop(lease);
        assert_eq!(registry.pool().reserved_total(), 0);
        // A rebuilt decoding leases (2 + e) M.
        let (_, lease) = admit_json(&registry, COMMAND, ArgsDecoding::Rebuilt { vec_expansion: 5 }, &args_body()).unwrap();
        assert_eq!(
            lease.bytes(),
            allocation_envelope(body.len() as u64).unwrap() + 7 * bound.materialization_bytes + ARGS_ERROR_ALLOWANCE_BYTES
        );
    }

    #[test]
    fn json_bodies_missing_or_unknown_headers_and_non_objects_are_invalid_requests() {
        let registry = registry(ScriptedHost::normal());
        let raw = InvokeBody::Raw(serde_json::to_vec(&args_body()).unwrap());
        let json_body = InvokeBody::Json(args_body());
        let current = headers(Some(INGRESS_ARGS_JSON_V1));
        for (body, headers, code) in [
            (&json_body, &current, "INGRESS-RAW-BODY-REQUIRED"),
            (&raw, &headers(None), "INGRESS-HEADER-REQUIRED"),
            (&raw, &headers(Some("args-json-v2")), "INGRESS-HEADER-UNKNOWN"),
            (&json_body, &headers(None), "INGRESS-HEADER-REQUIRED"),
        ] {
            let error = admit_body(&registry, body, headers, COMMAND, ArgsDecoding::Moved).unwrap_err();
            assert!(invalid_message(&error).starts_with(code), "{error:?}");
            assert_eq!(serde_json::to_value(&error).unwrap()["code"], "INVALID_REQUEST");
        }
        for body in [&b"[1,2]"[..], b" \"text\" ", b"7", b"null"] {
            let error = admit_body(&registry, &InvokeBody::Raw(body.to_vec()), &current, COMMAND, ArgsDecoding::Moved).unwrap_err();
            assert!(invalid_message(&error).starts_with("INGRESS-ARGS-NOT-OBJECT"), "{error:?}");
        }
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn invalid_json_and_the_raw_value_key_are_refused_ingress_invalid_before_any_lease() {
        let registry = registry(ScriptedHost::normal());
        let current = headers(Some(INGRESS_ARGS_JSON_V1));
        for body in [
            &b"{\"model\":"[..],
            b"{\"model\":{}} x",
            br#"{"model":{"$serde_json::private::RawValue":"[1]"}}"#,
            br#"{"$serde_json::private::RawValue":"1"}"#,
            b"{\"model\":\"\xff\"}",
        ] {
            let error = admit_body(&registry, &InvokeBody::Raw(body.to_vec()), &current, COMMAND, ArgsDecoding::Moved).unwrap_err();
            assert!(matches!(refusal(&error), ResourceRefusal::IngressInvalid { .. }), "{error:?}");
            let encoded = serde_json::to_value(&error).unwrap();
            assert_eq!(encoded["code"], "RESOURCE_UNAVAILABLE");
            assert_eq!(encoded["refusal"]["kind"], "ingress_invalid");
            assert!(encoded["pool"].is_object());
        }
        // Grammatical but out of f64 range: leased, then refused and released.
        let error = admit_body(&registry, &InvokeBody::Raw(b"{\"model\":1e400}".to_vec()), &current, COMMAND, ArgsDecoding::Moved)
            .unwrap_err();
        assert!(matches!(refusal(&error), ResourceRefusal::IngressInvalid { .. }));
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn a_lease_above_the_budget_is_refused_before_deserialization() {
        let registry = registry(ScriptedHost::normal());
        let body = serde_json::to_vec(&args_body()).unwrap();
        let lease = lease_bytes(ingress::prescan(&body).unwrap(), ArgsDecoding::Moved).unwrap();
        registry.configure_pool(12 * GIB, lease - 1, false).unwrap();
        let error = admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).unwrap_err();
        assert_eq!(refusal(&error), &ResourceRefusal::IngressExceedsBudget { required_bytes: lease, budget_bytes: lease - 1 });
        assert_eq!(serde_json::to_value(&error).unwrap()["refusal"]["kind"], "ingress_exceeds_budget");
        registry.configure_pool(12 * GIB, lease, false).unwrap();
        assert!(admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).is_ok());
        // Rebuilt decoding needs the larger lease.
        let error = admit_json(&registry, COMMAND, ArgsDecoding::Rebuilt { vec_expansion: 5 }, &args_body()).unwrap_err();
        assert!(matches!(refusal(&error), ResourceRefusal::IngressExceedsBudget { .. }));
        assert_eq!(registry.pool().reserved_total(), 0);
        // Overflowing lease arithmetic is refused, never wrapped.
        let huge = IngressBound { raw_len: 16, materialization_bytes: u64::MAX / 2 };
        assert_eq!(lease_bytes(huge, ArgsDecoding::Rebuilt { vec_expansion: 5 }), None);
        assert_eq!(lease_bytes(IngressBound { raw_len: u64::MAX, materialization_bytes: 0 }, ArgsDecoding::Moved), None);
    }

    #[test]
    fn headroom_refuses_but_a_pressure_warning_does_not_while_a_solve_start_is_refused() {
        let host = ScriptedHost::with(Some(16 * GIB), Some(GIB), PressureLevel::Warning);
        let registry = registry(host.clone());
        let (_, lease) = admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).expect("interactive at warning");
        let solve = registry.start(br#"{"nodes":[]}"#, None).unwrap_err();
        assert_eq!(refusal(&solve), &ResourceRefusal::PressureElevated { level: PressureLevel::Warning });
        drop(lease);
        host.set(|s| s.headroom_bytes = Some(1024));
        let error = admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).unwrap_err();
        assert!(matches!(refusal(&error), ResourceRefusal::HeadroomInsufficient { available_bytes: 1024, .. }));
        host.set(|s| s.headroom_bytes = None);
        let error = admit_json(&registry, COMMAND, ArgsDecoding::Moved, &args_body()).unwrap_err();
        assert_eq!(refusal(&error), &ResourceRefusal::HeadroomUnknown);
        assert_eq!(registry.pool().reserved_total(), 0);
    }

    #[test]
    fn respond_serializes_under_the_lease_and_errors_keep_their_shapes() {
        let response = respond::<_, String>(Ok(json!({"a": [1, 2]}))).unwrap();
        let _ = response;
        let command = serde_json::to_value(CommandError::<String>::Command("PROJECT-ID-EXISTS: x".into())).unwrap();
        assert_eq!(command, json!("PROJECT-ID-EXISTS: x"), "command errors serialize as before");
        let ingress = serde_json::to_value(CommandError::<String>::Ingress(invalid_request("m".into()))).unwrap();
        assert_eq!(ingress, json!({"code": "INVALID_REQUEST", "message": "m"}));
        #[derive(Serialize)]
        struct Typed {
            code: &'static str,
        }
        assert_eq!(serde_json::to_value(CommandError::Command(Typed { code: "SAVE_BUSY" })).unwrap(), json!({"code": "SAVE_BUSY"}));
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct Flat {
        name: String,
        flag: bool,
        count: usize,
        inner: Option<Inner>,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    struct Inner {
        id: String,
    }

    #[derive(Debug, serde::Deserialize, PartialEq)]
    enum Kind {
        Unit,
        Newtype(u8),
        Tuple(u8, u8),
        Struct { a: u8 },
    }

    /// The decoder agrees with serde_json's `Value` deserializer on accepted
    /// inputs and on which inputs it refuses.
    #[test]
    fn value_decoder_matches_serde_json_semantics() {
        let accepted = [
            json!({"name": "n", "flag": true, "count": 3, "inner": {"id": "x"}}),
            json!({"name": "n", "flag": false, "count": 0, "inner": null}),
            json!({"name": "n", "flag": false, "count": 0}),
            json!(["n", true, 7, ["x"]]),
            json!({"name": "n", "flag": true, "count": 1, "extra": [1, {"deep": "x"}]}),
        ];
        for value in accepted {
            let ours = Flat::deserialize(ValueDecoder(value.clone())).unwrap();
            assert_eq!(ours, serde_json::from_value::<Flat>(value).unwrap());
        }
        let refused = [
            json!("text"),
            json!({"name": 1, "flag": true, "count": 1}),
            json!({"name": "n", "flag": "yes", "count": 1}),
            json!({"name": "n", "flag": true, "count": -1}),
            json!({"name": "n", "flag": true, "count": 1.5}),
            json!({"name": "n", "flag": true, "count": 1, "inner": {"id": "x", "other": 1}}),
            json!({"name": "n", "flag": true}),
            json!(["n", true]),
            json!(["n", true, 7, null, "extra"]),
        ];
        for value in refused {
            assert!(Flat::deserialize(ValueDecoder(value.clone())).is_err(), "{value}");
            assert!(serde_json::from_value::<Flat>(value).is_err());
        }
        for (value, kind) in [
            (json!("Unit"), Kind::Unit),
            (json!({"Newtype": 4}), Kind::Newtype(4)),
            (json!({"Tuple": [1, 2]}), Kind::Tuple(1, 2)),
            (json!({"Struct": {"a": 9}}), Kind::Struct { a: 9 }),
        ] {
            assert_eq!(Kind::deserialize(ValueDecoder(value.clone())).unwrap(), kind);
            assert_eq!(serde_json::from_value::<Kind>(value).unwrap(), kind);
        }
        for value in [json!({"Unit": null, "Newtype": 1}), json!(3), json!("Missing"), json!({"Unit": 1})] {
            assert!(Kind::deserialize(ValueDecoder(value.clone())).is_err(), "{value}");
            assert!(serde_json::from_value::<Kind>(value).is_err());
        }
        let deep = json!({"a": [1, -2, 3.5, "s", null, true, {"b": []}], "c": {}});
        assert_eq!(Value::deserialize(ValueDecoder(deep.clone())).unwrap(), deep);
    }

    /// An error never echoes a long string: the message is one buffer of at
    /// most the bounded length, however long the offending string is.
    #[test]
    fn decoding_errors_are_bounded_whatever_the_offending_string() {
        let long = "\u{1}\u{85}x".repeat(200_000);
        for value in [json!(long.clone()), json!({"name": "n", "flag": long.clone(), "count": 1})] {
            let ((), measured) = ingress::counting::measure(move || {
                let error = Flat::deserialize(ValueDecoder(value)).unwrap_err();
                assert!(error.0.len() <= MAX_ARGS_ERROR_MESSAGE_BYTES && error.0.starts_with("invalid type: string"), "{error}");
                drop(error);
            });
            assert!(measured.peak_priced <= 256, "{measured:?}");
        }
        let unknown_key = format!("{{\"id\":\"x\",\"{}\":1}}", "k".repeat(100_000));
        let error = Inner::deserialize(ValueDecoder(serde_json::from_str(&unknown_key).unwrap())).unwrap_err();
        assert!(error.0.len() <= MAX_ARGS_ERROR_MESSAGE_BYTES && error.0.starts_with("unknown field"));
        // Truncation keeps UTF-8 boundaries.
        let error: ArgsError = de::Error::custom("é".repeat(1000));
        assert!(error.0.len() <= MAX_ARGS_ERROR_MESSAGE_BYTES && error.0.chars().all(|c| c == 'é'));
    }

    /// The moved decoding allocates nothing beyond the parse for moved
    /// `Value`s, strings, scalars and flat structs, and a refused argument's
    /// error fits the allowance, for representative and adversarial bodies.
    #[test]
    fn moved_decoding_stays_within_its_lease() {
        let long = "\\u0001".repeat(50_000);
        let bodies = [
            r#"{"model":{"nodes":[1,2,3]},"intent":{"k":"v"},"claimedModelHash":null}"#.to_string(),
            format!(r#"{{"model":{{"name":"{}"}},"intent":[{}0],"claimedModelHash":{{"value":"h"}}}}"#, "a".repeat(100_000), "1.5,".repeat(5_000)),
            format!(r#"{{"model":"{long}","intent":{{}}}}"#),
            format!(r#"{{"intent":{{}},"extra":[{}]}}"#, "[[[]]],".repeat(1_000) + "0"),
        ];
        for body in &bodies {
            check_decode_bound(body.as_bytes(), ArgsDecoding::Moved, |args| {
                let _ = (args.value("model"), args.value("intent"), args.opt_value("claimedModelHash"));
            });
        }
        // Typed arguments: moved strings and inline structs; refused ones with long strings.
        let typed = [
            r#"{"name":"n","flag":true,"count":2,"inner":{"id":"x"}}"#.to_string(),
            format!(r#"{{"name":"{}","flag":true,"count":2,"inner":{{"id":"{}"}}}}"#, "n".repeat(70_000), "i".repeat(40_000)),
            format!(r#"{{"name":"n","flag":"{long}","count":2}}"#),
            format!(r#"{{"name":"n","flag":true,"count":2,"inner":"{long}"}}"#),
            format!(r#"{{"name":"n","flag":true,"count":2,"inner":{{"id":"x","{}":1}}}}"#, "k".repeat(90_000)),
        ];
        for fields in &typed {
            let body = format!(r#"{{"value":{fields},"text":"{}","flag":"{long}"}}"#, "t".repeat(10_000));
            check_decode_bound(body.as_bytes(), ArgsDecoding::Moved, |args| {
                let _ = args.decode::<Flat>("value");
                let _ = args.decode::<String>("text");
                let _ = args.decode::<Option<bool>>("flag");
                let _ = args.decode::<Option<bool>>("absent");
                let _ = args.value("absent");
            });
        }
    }

    // --- save_report_package's typed request -------------------------------

    use open_pipe_stress_report_package::wire;
    use std::mem::size_of;

    /// One field of a wire struct, in declaration order.
    #[derive(Clone, Copy)]
    enum F {
        Str,
        Bool,
        Num,
        /// `Option<_>`: `null` (or omitted in an object).
        Opt,
        /// `Vec<_>`: `[]` (its own vector type, priced separately).
        Vec,
        /// `#[serde(default)]`: omittable when trailing.
        Default,
        /// A required inline struct.
        Struct(&'static str),
    }

    /// The vector element struct types of `ReportPackageRequest` and the
    /// inline structs they require, with `size_of` and field shapes mirroring
    /// `open_pipe_stress_report_package::wire` (checked against it by
    /// `report_package_shapes_match_the_wire_types`).
    fn shapes() -> Vec<(&'static str, usize, Vec<F>)> {
        use F::*;
        vec![
            ("ReferenceDto", size_of::<wire::ReferenceDto>(), vec![Str, Str]),
            ("ProvenanceDto", size_of::<wire::ProvenanceDto>(), vec![Str; 7]),
            ("ChecksumDto", size_of::<wire::ChecksumDto>(), vec![Str, Str, Struct("ReferenceDto"), Str]),
            (
                "DiagnosticDto",
                size_of::<wire::DiagnosticDto>(),
                vec![Str, Str, Str, Struct("ReferenceDto"), Struct("ReferenceDto"), Str, Str, Struct("ProvenanceDto")],
            ),
            (
                "ResultTraceLinkDto",
                size_of::<wire::ResultTraceLinkDto>(),
                vec![Str, Str, Struct("ReferenceDto"), Struct("ReferenceDto"), Struct("ProvenanceDto"), Vec],
            ),
            (
                "QuantityResultDto",
                size_of::<wire::QuantityResultDto>(),
                vec![
                    Str, Str, Struct("ReferenceDto"), Struct("ReferenceDto"), Opt, Num, Str, Str, Opt, Vec, Vec,
                    Struct("ProvenanceDto"),
                ],
            ),
            ("ResultSetDto", size_of::<wire::ResultSetDto>(), vec![Str, Str, Struct("ReferenceDto"), Vec]),
            (
                "ReproducibilityDto",
                size_of::<wire::ReproducibilityDto>(),
                vec![Struct("ChecksumDto"), Vec, Struct("ReferenceDto"), Bool],
            ),
            (
                "RulePackExportRefDto",
                size_of::<wire::RulePackExportRefDto>(),
                vec![Str, Str, Struct("ChecksumDto"), Str, Str, Str, Bool],
            ),
            ("ProfessionalBoundaryDto", size_of::<wire::ProfessionalBoundaryDto>(), vec![Bool, Bool, Bool, Bool, Bool, Default]),
            (
                "ResultEnvelopeDto",
                size_of::<wire::ResultEnvelopeDto>(),
                vec![
                    Str, Str, Struct("ReferenceDto"), Struct("ReferenceDto"), Str, Str, Str, Struct("ReferenceDto"), Vec, Vec,
                    Vec, Struct("ProvenanceDto"), Struct("ReproducibilityDto"), Vec, Vec, Struct("ProfessionalBoundaryDto"),
                ],
            ),
            ("RulePackAuditRefDto", size_of::<wire::RulePackAuditRefDto>(), vec![Str, Str, Str, Str, Opt, Bool]),
            ("AssetManifestEntryDto", size_of::<wire::AssetManifestEntryDto>(), vec![Str, Str, Str, Opt, Str]),
        ]
    }

    /// The vector element types of the request: the structs above that
    /// appear in a `Vec`, plus `Vec<String>` and `Vec<Value>`.
    const VECTOR_STRUCTS: [&str; 10] = [
        "ReferenceDto", "ChecksumDto", "DiagnosticDto", "ResultTraceLinkDto", "QuantityResultDto", "ResultSetDto",
        "RulePackExportRefDto", "ResultEnvelopeDto", "RulePackAuditRefDto", "AssetManifestEntryDto",
    ];

    fn shape(name: &str) -> Vec<F> {
        shapes().into_iter().find(|(n, _, _)| *n == name).unwrap_or_else(|| panic!("{name}")).2
    }

    /// The prescan's price of a container with `count` entries.
    fn array_price(count: u64) -> u64 {
        if count == 0 {
            0
        } else {
            allocation_envelope(count.next_power_of_two().max(4) * 32).unwrap()
        }
    }

    fn object_price(count: u64) -> u64 {
        if count == 0 {
            0
        } else {
            (count / 5 + 1) * 1024
        }
    }

    /// c_X: the least the prescan prices the containers of one element X
    /// decodes from (strings and nested vectors excluded).
    fn own_price(name: &str) -> u64 {
        let fields = shape(name);
        let positional = fields.iter().rposition(|f| !matches!(f, F::Default)).map_or(0, |i| i + 1) as u64;
        let required = fields.iter().filter(|f| !matches!(f, F::Opt | F::Default)).count() as u64;
        let nested: u64 = fields.iter().map(|f| if let F::Struct(inner) = f { own_price(inner) } else { 0 }).sum();
        array_price(positional).min(object_price(required)) + nested
    }

    /// The minimal array-form instance of a struct (the shape `own_price` prices).
    fn minimal(name: &str) -> Value {
        let fields = shape(name);
        let positional = fields.iter().rposition(|f| !matches!(f, F::Default)).map_or(0, |i| i + 1);
        Value::Array(
            fields[..positional]
                .iter()
                .map(|f| match f {
                    F::Str => json!(""),
                    F::Bool => json!(false),
                    F::Num => json!(0),
                    F::Opt => Value::Null,
                    F::Vec => json!([]),
                    F::Default => unreachable!(),
                    F::Struct(inner) => minimal(inner),
                })
                .collect(),
        )
    }

    /// The largest ratio of a `Vec<X>`'s priced allocations (its buffer and,
    /// while growing, its previous buffer) to the priced share of its
    /// elements (their slots in the source array and their own containers).
    fn vec_ratio(size: u64, own: u64) -> f64 {
        let min_cap = if size > 1024 { 1 } else { 4 };
        let mut worst = 0.0f64;
        let mut cap = min_cap;
        while cap <= 1u64 << 40 {
            // The fewest decoded elements that reach this capacity.
            let k = if cap == min_cap { 1 } else { cap / 2 + 1 };
            let typed = allocation_envelope(cap * size).unwrap()
                + if cap > min_cap { allocation_envelope(cap / 2 * size).unwrap() } else { 0 };
            let source = cap.max(4) * 32 + k * own;
            worst = worst.max(typed as f64 / source as f64);
            cap *= 2;
        }
        worst
    }

    #[test]
    fn report_package_vec_expansion_bounds_every_vector_type() {
        let mut report = Vec::new();
        let mut worst = 0.0f64;
        for name in VECTOR_STRUCTS {
            let size = shapes().into_iter().find(|(n, _, _)| *n == name).unwrap().1 as u64;
            let ratio = vec_ratio(size, own_price(name));
            report.push(format!("{name}: size {size}, c {}, ratio {ratio:.3}", own_price(name)));
            worst = worst.max(ratio);
        }
        for (name, size) in [("String", size_of::<String>() as u64), ("Value", size_of::<Value>() as u64)] {
            let ratio = vec_ratio(size, 0);
            report.push(format!("Vec<{name}>: size {size}, ratio {ratio:.3}"));
            worst = worst.max(ratio);
        }
        eprintln!("{}\nworst {worst:.3}", report.join("\n"));
        assert!(worst <= REPORT_PACKAGE_VEC_EXPANSION as f64, "{worst}");
    }

    /// The shape table mirrors the wire types: each minimal instance decodes,
    /// and one positional field fewer (or one more) does not.
    #[test]
    fn report_package_shapes_match_the_wire_types() {
        fn check<T: DeserializeOwned>(name: &str) {
            let instance = minimal(name);
            T::deserialize(ValueDecoder(instance.clone())).unwrap_or_else(|e| panic!("{name}: {e}"));
            let Value::Array(items) = instance else { unreachable!() };
            let mut short = items.clone();
            short.pop();
            assert!(T::deserialize(ValueDecoder(Value::Array(short))).is_err(), "{name}: fewer fields");
            let mut long = items;
            long.extend([json!(""), json!(""), json!("")]);
            assert!(T::deserialize(ValueDecoder(Value::Array(long))).is_err(), "{name}: more fields");
        }
        check::<wire::ReferenceDto>("ReferenceDto");
        check::<wire::ProvenanceDto>("ProvenanceDto");
        check::<wire::ChecksumDto>("ChecksumDto");
        check::<wire::DiagnosticDto>("DiagnosticDto");
        check::<wire::ResultTraceLinkDto>("ResultTraceLinkDto");
        check::<wire::QuantityResultDto>("QuantityResultDto");
        check::<wire::ResultSetDto>("ResultSetDto");
        check::<wire::ReproducibilityDto>("ReproducibilityDto");
        check::<wire::RulePackExportRefDto>("RulePackExportRefDto");
        check::<wire::ProfessionalBoundaryDto>("ProfessionalBoundaryDto");
        check::<wire::ResultEnvelopeDto>("ResultEnvelopeDto");
        check::<wire::RulePackAuditRefDto>("RulePackAuditRefDto");
        check::<wire::AssetManifestEntryDto>("AssetManifestEntryDto");
    }

    fn report_request(result_envelopes: Value) -> Value {
        json!({
            "package_id": "pkg",
            "export_profile_id": "profile",
            "source_model_ref": ["model", "m"],
            "source_basis_refs": [["load_case", "LC1"]],
            "report": {"sections": [{"title": "t", "rows": [1, 2, 3]}]},
            "audit_manifest": {
                "manifest_id": "m",
                "model_hash": null,
                "input_manifest_hash": null,
                "solver_version": ["s", "v", "b"],
                "unit_system_ref": "SI",
                "rule_pack_refs": [minimal("RulePackAuditRefDto")],
                "assets": [minimal("AssetManifestEntryDto")],
                "professional_boundary": minimal("ProfessionalBoundaryDto")
            },
            "result_envelopes": result_envelopes,
            "state_comparison_handoff_records": [{"a": [1, {"b": "c"}]}, []],
            "rule_check_aggregate": null,
            "solve_rule_check_status": "s"
        })
    }

    /// An array-form result envelope with exactly one element in each of its
    /// struct vectors, nested as deep as the request goes (the k = 1 case,
    /// the worst ratio).
    fn one_of_each_envelope() -> Value {
        let mut diagnostic = minimal("DiagnosticDto");
        diagnostic[0] = json!("d");
        let mut trace = minimal("ResultTraceLinkDto");
        trace[5] = json!([diagnostic.clone()]);
        let mut value = minimal("QuantityResultDto");
        value[9] = json!([diagnostic.clone()]);
        value[10] = json!([trace]);
        let mut set = minimal("ResultSetDto");
        set[3] = json!([value]);
        let mut envelope = minimal("ResultEnvelopeDto");
        envelope[8] = json!([["load_case", "LC1"]]);
        envelope[9] = json!([set]);
        envelope[10] = json!([diagnostic]);
        envelope[13] = json!(["ok"]);
        envelope[14] = json!([minimal("RulePackExportRefDto")]);
        envelope
    }

    fn decode_report(args: &mut Args) -> Result<wire::ReportPackageRequest, SolveJobError> {
        args.decode::<wire::ReportPackageRequest>("request")
    }

    /// The rebuilt decoding of the typed report-package request stays within
    /// envelope(raw) + (2 + REPORT_PACKAGE_VEC_EXPANSION) M + the allowance,
    /// for the invented witness and adversarial shapes: many one-element
    /// vectors in array form, long vectors (growth transients), duplicated
    /// object-form envelopes, and refused requests (a failing element after
    /// many decoded ones, long strings in refused positions).
    #[test]
    fn report_package_decoding_stays_within_its_lease() {
        let decoding = ArgsDecoding::Rebuilt { vec_expansion: REPORT_PACKAGE_VEC_EXPANSION };
        let witness: Value = serde_json::from_str(include_str!(
            "../../../../validation/witness/inputs/del1005_export_results_success_input.json"
        ))
        .unwrap();
        let witness_request = witness["export_results"].clone();
        let mut long_values = minimal("ResultSetDto");
        long_values[3] = Value::Array(vec![minimal("QuantityResultDto"); 5_000]);
        let mut long_envelope = minimal("ResultEnvelopeDto");
        long_envelope[9] = Value::Array(vec![long_values; 2]);
        long_envelope[8] = Value::Array(vec![json!(["", ""]); 3_000]);
        let mut duplicated = witness_request.clone();
        let envelope = duplicated["result_envelopes"][0].clone();
        duplicated["result_envelopes"] = Value::Array(vec![envelope; 200]);
        let mut failing = report_request(Value::Array(vec![one_of_each_envelope(); 300]));
        failing["result_envelopes"].as_array_mut().unwrap().push(json!({}));
        let mut long_string_refused = report_request(json!([]));
        long_string_refused["audit_manifest"]["professional_boundary"][0] = json!("\u{1}\u{85}".repeat(40_000));
        let cases = [
            ("witness", witness_request, true),
            ("one_of_each", report_request(Value::Array(vec![one_of_each_envelope(); 400])), true),
            ("one_of_each_single", report_request(json!([one_of_each_envelope()])), true),
            ("long_vectors", report_request(json!([long_envelope])), true),
            ("duplicated_witness_envelopes", duplicated, true),
            ("failing_after_many", failing, false),
            ("long_string_refused", long_string_refused, false),
            ("empty_objects", report_request(Value::Array(vec![json!({}); 20_000])), false),
        ];
        for (name, request, decodes) in cases {
            let body = serde_json::to_vec(&json!({ "request": request })).unwrap();
            let (decoded, measured, available) = check_decode_bound(&body, decoding, |args| decode_report(args).map(drop));
            assert_eq!(decoded.is_ok(), decodes, "{name}: {decoded:?}");
            eprintln!("{name}: raw {} priced peak {} lease share {available}", body.len(), measured.peak_priced);
        }
    }
}
