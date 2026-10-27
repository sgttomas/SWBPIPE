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
//! `Value` (every command), and D = (2 + vec_expansion) * M when a typed
//! request rebuilds `Value` fields and builds typed vectors. A command whose
//! argument is a canonical text it parses (`save_report_package`) grows its
//! lease by that text's priced parse before parsing it: [`text_parse_bytes`]
//! for its model and input-manifest texts, and
//! [`report_package_request_parse_bytes`] for its request text, which prices
//! the rebuild and the typed vectors by the request's regions
//! ([`REPORT_PACKAGE_REQUEST_REGIONS`]) and adds the command's working memory
//! ([`REPORT_PACKAGE_WORKING_MEMORY_FACTOR`]). Its route
//! `DREP-PACKAGE-USER-RELEASE-010` (W-18b) adds the run record text's priced
//! parse, its derivation's request and input manifest (one more priced parse
//! of each text) and its refusal formatting allowance
//! (`report_package_save::ingress_lease_bytes`), and grows the lease inside
//! admission by rule 3 (b)'s largest embedded candidate; every growth is
//! refused above the budget the command was admitted under. The
//! counting-allocator tests check the requested peak against these rules for
//! representative and adversarial inputs.
//!
//! Residual: the command's own working memory and its response serialization
//! run under the held lease but are not separately bounded, except the report
//! package save's working-memory term; wry/Tauri copies of the raw body made
//! before the command runs are not ours to price (as for the solve start). A
//! command's error is converted by Tauri after the command returns and its
//! lease is released; route `DREP-PACKAGE-USER-RELEASE-010`'s refusals are
//! bounded to one diagnostic of fixed size for that reason
//! (`report_package_save::bound_route_010_refusal`).

use crate::ingress::{self, allocation_envelope, FieldRegion, IngressBound, RegionField, RegionStruct};
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
    /// `vec_expansion` * M). No command decodes so now: the report-package
    /// request text, the one typed request, is priced by its regions
    /// ([`report_package_request_parse_bytes`]), which never exceed this rule
    /// with `REPORT_PACKAGE_VEC_EXPANSION` (checked by its tests).
    #[cfg_attr(not(test), allow(dead_code))]
    Rebuilt { vec_expansion: u64 },
}

/// The interactive ingress lease for a prescanned body: envelope(raw) + D +
/// [`ARGS_ERROR_ALLOWANCE_BYTES`] (see the module docs); `None` on overflow.
pub(crate) fn lease_bytes(bound: IngressBound, decoding: ArgsDecoding) -> Option<u64> {
    allocation_envelope(bound.raw_len)?.checked_add(decoded_bytes(bound.materialization_bytes, decoding)?)?.checked_add(ARGS_ERROR_ALLOWANCE_BYTES)
}

/// D of the lease rule for a materialization M: M when moved, (2 +
/// vec_expansion) * M when typed-decoded.
fn decoded_bytes(materialization: u64, decoding: ArgsDecoding) -> Option<u64> {
    match decoding {
        ArgsDecoding::Moved => Some(materialization),
        ArgsDecoding::Rebuilt { vec_expansion } => materialization.checked_mul(vec_expansion.checked_add(2)?),
    }
}

/// The priced parse of one canonical-text argument, moved out of the args as a
/// string (`save_report_package`, DEL-08-01 Design
/// `report-package-current-results.md` §2.6 item 1): the text's own `Value`
/// materialization M (bounded by [`ingress::prescan`], which allocates
/// nothing), decoded as `decoding` says, plus the canonical re-serialization
/// the binding check builds (one `String` of the text's length and its last
/// growth transient, 2 * envelope(length)) and the error allowance. The
/// command grows its ingress lease by this before it parses the text. A text
/// the prescan refuses is returned as the prescan's finding, unparsed.
pub(crate) fn text_parse_bytes(text: &str, decoding: ArgsDecoding) -> Result<u64, ingress::IngressInvalid> {
    let bound = ingress::prescan(text.as_bytes())?;
    let overflow = ingress::IngressInvalid { reason: "the priced parse overflows", offset: 0 };
    decoded_bytes(bound.materialization_bytes, decoding)
        .and_then(|decoded| decoded.checked_add(allocation_envelope(bound.raw_len)?.checked_mul(2)?))
        .and_then(|bytes| bytes.checked_add(ARGS_ERROR_ALLOWANCE_BYTES))
        .ok_or(overflow)
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

/// The typed-vector factor for `save_report_package`'s typed request
/// (`open_pipe_stress_report_package::wire::ReportPackageRequest`), converted
/// from the parsed request text (W-18 slice (b1);
/// [`report_package_request_parse_bytes`] charges it on C_T): decoded
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
/// different vectors are disjoint parts of C_T, the typed-vector regions'
/// containers. Since request 2.0.0 the worst
/// types are `RulePackAuditRefDto` (S = 224 B) and `AssetManifestEntryDto`
/// (S = 216 B), both with c = 256 B: ratio 2.667; the constant keeps its
/// earlier margin.
pub(crate) const REPORT_PACKAGE_VEC_EXPANSION: u64 = 4;

/// W, the working-memory factor of the report-package request text's lease
/// growth (DEL-04-07 memory-budget decision, "Report-package request
/// pricing"): the growth carries W * M for the command's working memory after
/// the conversion, its screening walk over the parsed request and model and
/// the shared producer's assembly of the members and the container, which
/// DEL-04-07 otherwise lists as a residual that is not separately bounded.
/// It is not a computed bound: the whole-command counting-allocator tests
/// check it only for the committed witness request and the demo's request.
/// Two values were put to the owner:
/// - 1, the recommended value: the request text's growth is
///   M + M + C_V + 4 C_T, never above the former (2 + 4) M;
/// - 0, the region design without this term: M + C_V + 4 C_T, with the
///   working memory unpriced.
///
/// The owner chose 1 (2026-10-25: "Both decisions I approve your recommended
/// options."). The term covers the working memory in proportion to the
/// request's size; it does not prove it bounded in general.
///
/// For any W <= 1 the growth stays within the former rule
/// (`report_package_request_growth_never_exceeds_the_former_rule`).
pub(crate) const REPORT_PACKAGE_WORKING_MEMORY_FACTOR: u64 = 1;

/// The fields of `open_pipe_stress_report_package::wire::ReportPackageRequest`
/// (version 2.0.0) in declaration order, with what decoding builds from each
/// (checked against the wire types by
/// `report_package_region_table_matches_the_wire_types`). The opaque regions
/// are the value at `report` and the elements of `result_documents`,
/// `source_results` and `state_comparison_handoff_records`; the typed-vector
/// regions are the values at the six `Vec` fields without those opaque
/// elements.
pub(crate) static REPORT_PACKAGE_REQUEST_REGIONS: RegionStruct = RegionStruct {
    name: "ReportPackageRequest",
    fields: &[
        RegionField { name: "package_id", region: FieldRegion::Neither },
        RegionField { name: "export_profile_id", region: FieldRegion::Neither },
        RegionField { name: "source_model_ref", region: FieldRegion::Neither },
        RegionField { name: "source_basis_refs", region: FieldRegion::TypedVector },
        RegionField { name: "report", region: FieldRegion::Opaque },
        RegionField { name: "audit_manifest", region: FieldRegion::Struct(&AUDIT_MANIFEST_REGIONS) },
        RegionField { name: "result_documents", region: FieldRegion::TypedVectorOfOpaque },
        RegionField { name: "source_results", region: FieldRegion::TypedVectorOfOpaque },
        RegionField { name: "state_comparison_handoff_records", region: FieldRegion::TypedVectorOfOpaque },
        RegionField { name: "rule_check_aggregate", region: FieldRegion::Neither },
        RegionField { name: "solve_rule_check_status", region: FieldRegion::Neither },
    ],
};

/// `wire::AuditManifestDto`'s fields, as [`REPORT_PACKAGE_REQUEST_REGIONS`].
pub(crate) static AUDIT_MANIFEST_REGIONS: RegionStruct = RegionStruct {
    name: "AuditManifestDto",
    fields: &[
        RegionField { name: "manifest_id", region: FieldRegion::Neither },
        RegionField { name: "model_hash", region: FieldRegion::Neither },
        RegionField { name: "input_manifest_hash", region: FieldRegion::Neither },
        RegionField { name: "solver_version", region: FieldRegion::Neither },
        RegionField { name: "unit_system_ref", region: FieldRegion::Neither },
        RegionField { name: "rule_pack_refs", region: FieldRegion::TypedVector },
        RegionField { name: "assets", region: FieldRegion::TypedVector },
        RegionField { name: "professional_boundary", region: FieldRegion::Neither },
    ],
};

/// The priced parse of `save_report_package`'s request text (DEL-04-07
/// memory-budget decision, "Report-package request pricing"; DEL-08-01 Design
/// `report-package-current-results.md` §2.6 item 1): with M the text's
/// materialization bound and C_V and C_T the priced containers of its opaque
/// and typed-vector regions ([`ingress::prescan_regions`] over
/// [`REPORT_PACKAGE_REQUEST_REGIONS`], which allocates nothing),
/// M + W * M + C_V + [`REPORT_PACKAGE_VEC_EXPANSION`] * C_T, plus the
/// canonical re-serialization (2 * envelope(length), as [`text_parse_bytes`])
/// and the error allowance. W is [`REPORT_PACKAGE_WORKING_MEMORY_FACTOR`].
/// The parse and the binding check fit M and the re-serialization; decoding
/// moves every string and key, rebuilds the opaque values within C_V (each
/// rebuilt container has its source's entry count), fits its one growth
/// transient at a time in M's array-transient allowance (free once the parse
/// has returned), and builds each typed vector within
/// `REPORT_PACKAGE_VEC_EXPANSION` times the containers of its own region. A
/// text the prescan refuses is returned as the prescan's finding, unparsed.
pub(crate) fn report_package_request_parse_bytes(text: &str) -> Result<u64, ingress::IngressInvalid> {
    let (bound, regions) = ingress::prescan_regions(text.as_bytes(), &REPORT_PACKAGE_REQUEST_REGIONS)?;
    let overflow = ingress::IngressInvalid { reason: "the priced parse overflows", offset: 0 };
    let materialization = bound.materialization_bytes;
    [
        materialization.checked_mul(REPORT_PACKAGE_WORKING_MEMORY_FACTOR),
        Some(regions.opaque),
        regions.typed.checked_mul(REPORT_PACKAGE_VEC_EXPANSION),
        allocation_envelope(bound.raw_len).and_then(|envelope| envelope.checked_mul(2)),
        Some(ARGS_ERROR_ALLOWANCE_BYTES),
    ]
    .into_iter()
    .try_fold(materialization, |sum, term| sum.checked_add(term?))
    .ok_or(overflow)
}

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

    /// A growing site's normal path against the per-invocation budget
    /// (`memory_pool::Lease::grow_to`): `run` admits its body and grows the
    /// lease as the command does, returning the grown lease. It is admitted
    /// under `registry`'s budget, and again with the budget at exactly the
    /// grown lease; one byte below that, the growth is refused
    /// `ingress_exceeds_budget` (the ingress lease itself still fits).
    /// Returns (ingress lease, grown lease).
    pub(crate) fn grown_lease_meets_the_budget(
        registry: &SolveJobRegistry,
        label: &str,
        ingress: u64,
        run: &dyn Fn() -> Result<u64, SolveJobError>,
    ) -> (u64, u64) {
        let (pool_bytes, budget) = registry.pool().settings();
        let grown = run().unwrap_or_else(|error| panic!("{label}: refused under the budget {budget}: {error:?}"));
        assert!(ingress < grown && grown <= budget, "{label}: ingress {ingress} grown {grown} budget {budget}");
        registry.configure_pool(pool_bytes, grown, false).unwrap();
        assert_eq!(run().unwrap_or_else(|error| panic!("{label}: refused at a budget of its grown lease: {error:?}")), grown);
        registry.configure_pool(pool_bytes, grown - 1, false).unwrap();
        match run() {
            Err(SolveJobError::ResourceUnavailable { refusal, .. }) => assert_eq!(
                refusal,
                ResourceRefusal::IngressExceedsBudget { required_bytes: grown, budget_bytes: grown - 1 },
                "{label}"
            ),
            other => panic!("{label}: not refused at its growth one byte below: {other:?}"),
        }
        registry.configure_pool(pool_bytes, budget, false).unwrap();
        eprintln!("{label}: ingress lease {ingress} B, grown lease {grown} B, budget {budget} B");
        (ingress, grown)
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
            ("ProfessionalBoundaryDto", size_of::<wire::ProfessionalBoundaryDto>(), vec![Bool, Bool, Bool, Bool, Bool, Default]),
            ("RulePackAuditRefDto", size_of::<wire::RulePackAuditRefDto>(), vec![Str, Str, Str, Str, Opt, Bool]),
            ("AssetManifestEntryDto", size_of::<wire::AssetManifestEntryDto>(), vec![Str, Str, Str, Opt, Str]),
        ]
    }

    /// The vector element types of the request: the structs above that
    /// appear in a `Vec`, plus `Vec<String>` and `Vec<Value>` (the result
    /// documents, their source results and the handoff records).
    const VECTOR_STRUCTS: [&str; 3] = ["ReferenceDto", "RulePackAuditRefDto", "AssetManifestEntryDto"];

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
        check::<wire::ProfessionalBoundaryDto>("ProfessionalBoundaryDto");
        check::<wire::RulePackAuditRefDto>("RulePackAuditRefDto");
        check::<wire::AssetManifestEntryDto>("AssetManifestEntryDto");
    }

    fn report_request(source_basis_refs: Value, result_documents: Value) -> Value {
        json!({
            "package_id": "pkg",
            "export_profile_id": "profile",
            "source_model_ref": ["model", "m"],
            "source_basis_refs": source_basis_refs,
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
            "result_documents": result_documents.clone(),
            "source_results": result_documents,
            "state_comparison_handoff_records": [{"a": [1, {"b": "c"}]}, []],
            "rule_check_aggregate": null,
            "solve_rule_check_status": "s"
        })
    }

    /// The priced parse of `save_report_package`'s texts (DEL-08-01 Design
    /// §2.6 item 1, W-18 slice (b1); the input manifest since (b2) review
    /// round 1): parsing the request text, its canonical re-serialization and
    /// its typed conversion, then the model text's and the input-manifest
    /// text's parse and re-serialization, stay within the lease growth
    /// `report_package_save::parse_lease_bytes` prices, for the invented
    /// desktop request and adversarial shapes: many one-element struct
    /// vectors in array form, long vectors (growth transients), many result
    /// documents, and refused requests (a failing element after many decoded
    /// ones, long strings in refused positions). The arguments themselves are
    /// moved strings, priced by the ingress lease (`ArgsDecoding::Moved`).
    #[test]
    fn report_package_text_parse_stays_within_its_priced_growth() {
        use crate::report_package_save::{parse_canonical, parse_lease_bytes, solve_rule_packs, test_support, typed_request};
        use open_pipe_stress_operation_applier::canonical_json;
        let model = canonical_json(&test_support::model());
        // The input manifest embeds the model and repeats its load cases, as
        // the session's does (`inputManifestService.ts`).
        let mut manifest = test_support::input_manifest(&test_support::model(), Value::Array(
            (0..200).map(|index| test_support::active_pack(&format!("invented-pack-{index}"), "1.0.0", &"a".repeat(64))).collect(),
        ));
        manifest["load_basis"] = json!({"load_cases": test_support::model()["load_cases"].clone(), "combinations": []});
        let manifest = canonical_json(&manifest);
        let desktop = test_support::desktop_request(&test_support::model());
        let one = json!([["analysis_run", "r"]]);
        let mut duplicated = desktop.clone();
        let document = duplicated["result_documents"][0].clone();
        duplicated["result_documents"] = Value::Array(vec![document.clone(); 200]);
        duplicated["source_results"] = Value::Array(vec![document; 200]);
        let mut failing = report_request(Value::Array(vec![json!(["load_case", "LC1"]); 3_000]), json!([]));
        failing["source_basis_refs"].as_array_mut().unwrap().push(json!({}));
        let mut many_rule_packs = report_request(one.clone(), json!([]));
        many_rule_packs["audit_manifest"]["rule_pack_refs"] = Value::Array(vec![minimal("RulePackAuditRefDto"); 4_000]);
        let mut failing_rule_packs = many_rule_packs.clone();
        failing_rule_packs["audit_manifest"]["rule_pack_refs"].as_array_mut().unwrap().push(json!({}));
        let mut long_string_refused = report_request(one.clone(), json!([]));
        long_string_refused["audit_manifest"]["professional_boundary"][0] = json!("\u{1}\u{85}".repeat(40_000));
        let cases = [
            ("desktop", desktop, true),
            ("one_reference", report_request(one.clone(), json!([{}])), true),
            ("long_references", report_request(Value::Array(vec![json!(["load_case", "LC1"]); 5_000]), json!([])), true),
            ("many_documents", report_request(one.clone(), Value::Array(vec![json!({"a": [1, 2, {"b": "c"}]}); 5_000])), true),
            ("duplicated_desktop_documents", duplicated, true),
            ("many_rule_packs", many_rule_packs, true),
            ("failing_after_many", failing, false),
            ("failing_rule_packs", failing_rule_packs, false),
            ("long_string_refused", long_string_refused, false),
            ("empty_objects", report_request(Value::Array(vec![json!({}); 20_000]), json!([])), false),
        ];
        for (name, request, decodes) in cases {
            let text = canonical_json(&request);
            drop(request);
            let growth = parse_lease_bytes(&text, &model, &manifest).unwrap();
            let (decoded, measured) = ingress::counting::measure(|| {
                let request = parse_canonical("request", &text).expect("canonical");
                let model = parse_canonical("model", &model).expect("canonical");
                let manifest = parse_canonical("input_manifest", &manifest).expect("canonical");
                // Admission keeps the solve's rule-pack set past the manifest.
                let packs = solve_rule_packs(&manifest).expect("complete inventory");
                drop((model, manifest));
                assert_eq!(packs.len(), 200);
                typed_request(request).map(drop)
            });
            assert_eq!(decoded.is_ok(), decodes, "{name}: {:?}", decoded.err().map(|error| error.message));
            assert!(measured.peak_priced <= growth, "{name}: priced peak {} exceeds the priced growth {growth}", measured.peak_priced);
            eprintln!("{name}: text {} priced peak {} growth {growth}", text.len(), measured.peak_priced);
        }
    }

    // --- the report-package request's regions (DEL-04-07 "Report-package
    // request pricing") ----------------------------------------------------

    /// What a wire type asks its deserializer for, field by field.
    #[derive(Clone, Debug, PartialEq)]
    enum WireKind {
        /// `deserialize_any`: a `Value`.
        Any,
        /// `deserialize_seq`: a `Vec` of the element kind.
        Seq(Box<WireKind>),
        /// `deserialize_option`.
        Opt(Box<WireKind>),
        /// `deserialize_struct`: its name and its fields in declaration order.
        Struct(&'static str, Vec<(&'static str, WireKind)>),
        /// A request that builds no container: a string, a boolean, a
        /// number or a char.
        Scalar(&'static str),
    }

    /// The probe's leaf: the request made at the end of a path.
    #[derive(Clone, Debug)]
    enum Node {
        Any,
        Seq,
        Opt,
        Struct(&'static str, &'static [&'static str]),
        Scalar(&'static str),
        /// A request the probe does not follow (maps, tuples, enums,
        /// newtypes, bytes, units, identifiers, ignored values). The wire
        /// types make none of them, so one is drift: `kind_at` refuses it.
        Unsupported(&'static str),
    }

    #[derive(Clone, Copy, Debug)]
    enum Step {
        Field(&'static str),
        Elem,
        Some,
    }

    #[derive(Debug)]
    struct ProbeError(String);

    impl fmt::Display for ProbeError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str(&self.0)
        }
    }

    impl std::error::Error for ProbeError {}

    impl de::Error for ProbeError {
        fn custom<T: fmt::Display>(message: T) -> Self {
            Self(message.to_string())
        }
    }

    /// A deserializer that follows `path` into a wire type through the
    /// requests its derived `Deserialize` makes, records the request at the
    /// path's end and stops there.
    struct Probe<'a> {
        path: &'a [Step],
        found: &'a std::cell::RefCell<Option<Node>>,
    }

    impl Probe<'_> {
        fn leaf<T>(self, node: Node) -> Result<T, ProbeError> {
            assert!(self.path.is_empty(), "the path {:?} does not lead through {node:?}", self.path);
            *self.found.borrow_mut() = Some(node);
            Err(de::Error::custom("probe stop"))
        }
    }

    macro_rules! probe_scalars {
        ($($method:ident)*) => {$(
            fn $method<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, ProbeError> {
                self.leaf(Node::Scalar(stringify!($method)))
            }
        )*};
    }

    macro_rules! probe_unsupported {
        ($($method:ident)*) => {$(
            fn $method<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, ProbeError> {
                self.leaf(Node::Unsupported(stringify!($method)))
            }
        )*};
    }

    impl<'de> de::Deserializer<'de> for Probe<'_> {
        type Error = ProbeError;

        fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Any)
        }

        fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ProbeError> {
            match self.path.split_first() {
                Some((Step::Elem, rest)) => visitor.visit_seq(OneElement(Some(Probe { path: rest, found: self.found }))),
                _ => self.leaf(Node::Seq),
            }
        }

        fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, ProbeError> {
            match self.path.split_first() {
                Some((Step::Some, rest)) => visitor.visit_some(Probe { path: rest, found: self.found }),
                _ => self.leaf(Node::Opt),
            }
        }

        fn deserialize_struct<V: Visitor<'de>>(
            self,
            name: &'static str,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, ProbeError> {
            match self.path.split_first() {
                Some((Step::Field(field), rest)) => {
                    visitor.visit_map(OneEntry { key: Some(field), value: Some(Probe { path: rest, found: self.found }) })
                }
                _ => self.leaf(Node::Struct(name, fields)),
            }
        }

        probe_scalars! {
            deserialize_bool deserialize_i8 deserialize_i16 deserialize_i32 deserialize_i64 deserialize_i128
            deserialize_u8 deserialize_u16 deserialize_u32 deserialize_u64 deserialize_u128 deserialize_f32
            deserialize_f64 deserialize_char deserialize_str deserialize_string
        }

        probe_unsupported! {
            deserialize_bytes deserialize_byte_buf deserialize_unit deserialize_map deserialize_identifier
            deserialize_ignored_any
        }

        fn deserialize_unit_struct<V: Visitor<'de>>(self, _name: &'static str, _visitor: V) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Unsupported("deserialize_unit_struct"))
        }

        fn deserialize_newtype_struct<V: Visitor<'de>>(self, _name: &'static str, _visitor: V) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Unsupported("deserialize_newtype_struct"))
        }

        fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Unsupported("deserialize_tuple"))
        }

        fn deserialize_tuple_struct<V: Visitor<'de>>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Unsupported("deserialize_tuple_struct"))
        }

        fn deserialize_enum<V: Visitor<'de>>(
            self,
            _name: &'static str,
            _variants: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, ProbeError> {
            self.leaf(Node::Unsupported("deserialize_enum"))
        }
    }

    struct OneElement<'a>(Option<Probe<'a>>);

    impl<'de> de::SeqAccess<'de> for OneElement<'_> {
        type Error = ProbeError;

        fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, ProbeError> {
            match self.0.take() {
                Some(probe) => seed.deserialize(probe).map(Some),
                None => Ok(None),
            }
        }
    }

    struct OneEntry<'a> {
        key: Option<&'static str>,
        value: Option<Probe<'a>>,
    }

    impl<'de> de::MapAccess<'de> for OneEntry<'_> {
        type Error = ProbeError;

        fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, ProbeError> {
            match self.key.take() {
                Some(key) => seed.deserialize(de::value::StrDeserializer::<ProbeError>::new(key)).map(Some),
                None => Ok(None),
            }
        }

        fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, ProbeError> {
            seed.deserialize(self.value.take().expect("one value"))
        }
    }

    /// A path as a field path: `audit_manifest.model_hash?`, `assets[]`.
    fn path_name(path: &[Step]) -> String {
        let mut name = String::new();
        for step in path {
            match step {
                Step::Field(field) if name.is_empty() => name.push_str(field),
                Step::Field(field) => name.push_str(&format!(".{field}")),
                Step::Elem => name.push_str("[]"),
                Step::Some => name.push('?'),
            }
        }
        if name.is_empty() { "the request".into() } else { name }
    }

    /// The kind `T` requests at `path`, recursively. A request the probe
    /// does not follow fails, naming the field path.
    fn kind_at<T: DeserializeOwned>(path: &mut Vec<Step>) -> WireKind {
        let found = std::cell::RefCell::new(None);
        let outcome = T::deserialize(Probe { path, found: &found });
        assert!(outcome.is_err(), "the probe always stops");
        let node = found.into_inner().unwrap_or_else(|| panic!("no request at {path:?}"));
        let inner = |path: &mut Vec<Step>, step: Step| {
            path.push(step);
            let kind = kind_at::<T>(path);
            path.pop();
            kind
        };
        match node {
            Node::Any => WireKind::Any,
            Node::Scalar(method) => WireKind::Scalar(method),
            Node::Unsupported(method) => panic!(
                "{}: the wire type requests {method}, which the region probe does not follow; extend the probe and the region table",
                path_name(path)
            ),
            Node::Seq => WireKind::Seq(Box::new(inner(path, Step::Elem))),
            Node::Opt => WireKind::Opt(Box::new(inner(path, Step::Some))),
            Node::Struct(name, fields) => WireKind::Struct(name, fields.iter().map(|field| (*field, inner(path, Step::Field(field)))).collect()),
        }
    }

    /// Whether decoding `kind` builds a `Value` or a `Vec` anywhere.
    fn allocates_containers(kind: &WireKind) -> bool {
        match kind {
            WireKind::Any | WireKind::Seq(_) => true,
            WireKind::Opt(inner) => allocates_containers(inner),
            WireKind::Struct(_, fields) => fields.iter().any(|(_, kind)| allocates_containers(kind)),
            WireKind::Scalar(_) => false,
        }
    }

    /// The table names every field of the struct in declaration order, and
    /// its region says what decoding builds from the field's value.
    fn check_region_table(table: &RegionStruct, kind: &WireKind) {
        let WireKind::Struct(name, fields) = kind else { panic!("{} is not a struct: {kind:?}", table.name) };
        assert_eq!(*name, table.name);
        let names: Vec<&str> = table.fields.iter().map(|field| field.name).collect();
        let wire: Vec<&str> = fields.iter().map(|(field, _)| *field).collect();
        assert_eq!(names, wire, "{name}: field names and positions");
        for (field, (_, kind)) in table.fields.iter().zip(fields) {
            match (field.region, kind) {
                (FieldRegion::Opaque, WireKind::Any) => {}
                (FieldRegion::TypedVectorOfOpaque, WireKind::Seq(element)) if **element == WireKind::Any => {}
                (FieldRegion::TypedVector, WireKind::Seq(element)) if !allocates_containers(element) => {}
                (FieldRegion::Struct(inner), kind) => check_region_table(inner, kind),
                (FieldRegion::Neither, kind) if !allocates_containers(kind) => {}
                (region, kind) => panic!("{name}.{}: region {region:?} does not describe {kind:?}", field.name),
            }
        }
    }

    #[test]
    fn report_package_region_table_matches_the_wire_types() {
        let kind = kind_at::<wire::ReportPackageRequest>(&mut Vec::new());
        check_region_table(&REPORT_PACKAGE_REQUEST_REGIONS, &kind);
        // The probe reads the wire types themselves: a few kinds spelled out.
        let WireKind::Struct(_, fields) = &kind else { unreachable!() };
        let field = |name: &str| fields.iter().find(|(field, _)| *field == name).unwrap().1.clone();
        assert_eq!(field("report"), WireKind::Any);
        assert_eq!(field("result_documents"), WireKind::Seq(Box::new(WireKind::Any)));
        assert_eq!(
            field("source_basis_refs"),
            WireKind::Seq(Box::new(WireKind::Struct(
                "ReferenceDto",
                vec![("ref_type", WireKind::Scalar("deserialize_string")), ("ref_id", WireKind::Scalar("deserialize_string"))]
            )))
        );
        assert_eq!(field("rule_check_aggregate"), WireKind::Opt(Box::new(WireKind::Scalar("deserialize_string"))));
    }

    /// The regions of one request text: (C_V, C_T, containers in neither).
    fn regions_of(text: &str) -> (u64, u64, u64) {
        let (bound, regions) = ingress::prescan_regions(text.as_bytes(), &REPORT_PACKAGE_REQUEST_REGIONS).unwrap_or_else(|e| panic!("{text}: {e:?}"));
        let parts = regions.strings + regions.opaque + regions.typed + regions.neither + regions.allowances;
        assert_eq!(parts, bound.materialization_bytes, "{text}: the parts sum to M");
        assert_eq!(ingress::prescan(text.as_bytes()).unwrap(), bound, "{text}: the bound is the prescan's");
        (regions.opaque, regions.typed, regions.neither)
    }

    /// Region cases: the paths the decoder follows, by unescaped field name
    /// or by position, with duplicate and unknown keys and `null` options.
    #[test]
    fn report_package_regions_follow_the_decoders_paths() {
        let (a, o) = (array_price, object_price);
        let cases: Vec<(&str, String, (u64, u64, u64))> = vec![
            ("named fields", r#"{"report":{"a":[1,2]},"source_basis_refs":[["x","y"]]}"#.into(), (o(1) + a(2), a(1) + a(2), o(2))),
            ("escaped field names", r#"{"\u0072eport":[1],"source\u005fbasis_refs":[[1]],"\u0061udit_manifest":{"rule_pack\u005Frefs":[[2]]}}"#.into(), (a(1), 4 * a(1), o(3) + o(1))),
            ("escaped opaque vector names", r#"{"result\u005fdocuments":[{"a":[1]}],"\u0073ource_results":[[2]]}"#.into(), (o(1) + 2 * a(1), 2 * a(1), o(2))),
            ("near misses are neither", r#"{"reports":[1],"repor":[1],"Report":[1],"report ":[1]}"#.into(), (0, 0, o(4) + 4 * a(1))),
            (
                "positional request and audit manifest",
                r#"["p","e",["m","x"],[["a","b"]],{"k":[1]},["i",null,null,["s","v","b"],"SI",[[1]],[[2],[3]],[true,false,false,false,false]],[{"d":1}],[],[[]],null,"s"]"#.into(),
                (o(1) + a(1) + o(1), a(1) + a(2) + a(1) + a(1) + a(2) + 2 * a(1) + a(1) + a(1), a(11) + a(2) + a(8) + a(3) + a(5)),
            ),
            ("positions past the fields are neither", r#"["p","e",["m","x"],[],null,null,[],[],[],null,"s",[[1]]]"#.into(), (0, 0, a(12) + a(2) + 2 * a(1))),
            ("duplicate keys are each priced", r#"{"report":[1],"report":[1,2,3,4,5],"assets":[[1]]}"#.into(), (a(1) + a(5), 0, o(3) + 2 * a(1))),
            ("unknown keys are neither", r#"{"extra":{"a":[1]},"audit_manifest":{"extra":[1],"assets":[[1]]}}"#.into(), (0, 2 * a(1), o(2) + o(1) + a(1) + o(2) + a(1))),
            ("null options keep positions", r#"{"audit_manifest":["m",null,null,null,"SI",[[1]],[[2]],null]}"#.into(), (0, 4 * a(1), o(1) + a(8))),
            ("null vectors and values", r#"{"report":null,"source_basis_refs":null,"audit_manifest":{"model_hash":null,"rule_pack_refs":null}}"#.into(), (0, 0, o(3) + o(2))),
            ("an opaque vector given as an object is typed", r#"{"result_documents":{"a":[1]}}"#.into(), (0, o(1) + a(1), o(1))),
            ("an opaque vector's elements are opaque", r#"{"result_documents":[{"a":[1]},[2]],"state_comparison_handoff_records":[[[]]]}"#.into(), (o(1) + 2 * a(1) + a(1), a(2) + a(1), o(2))),
            ("field names inside an opaque value", r#"{"report":{"source_basis_refs":[1],"report":[[2]]}}"#.into(), (o(2) + 3 * a(1), 0, o(1))),
            ("field names inside a struct of neither", r#"{"source_model_ref":{"report":[1]},"audit_manifest":{"solver_version":{"assets":[1]}}}"#.into(), (0, 0, o(2) + o(1) + a(1) + o(1) + o(1) + a(1))),
            ("opaque-shaped content in a typed element", r#"{"source_basis_refs":[{"ref_type":"a","x":{"y":[1]}}]}"#.into(), (0, a(1) + o(2) + o(1) + a(1), o(1))),
            ("a scalar request", "7".into(), (0, 0, 0)),
            ("escaped key with a surrogate pair stays unmatched", r#"{"report\ud83d\ude00":[1]}"#.into(), (0, 0, o(1) + a(1))),
        ];
        for (name, text, expected) in cases {
            assert_eq!(regions_of(&text), expected, "{name}: {text}");
        }
        // Escaped keys are attributed as their decoded, plain-key equivalents.
        let escaped_and_plain = [
            (r#"{"\u0072eport":[1],"source\u005fbasis_refs":[[1]],"\u0061udit_manifest":{"rule_pack\u005Frefs":[[2]]}}"#, r#"{"report":[1],"source_basis_refs":[[1]],"audit_manifest":{"rule_pack_refs":[[2]]}}"#),
            (r#"{"result\u005fdocuments":[{"a":[1]}],"\u0073ource_results":[[2]]}"#, r#"{"result_documents":[{"a":[1]}],"source_results":[[2]]}"#),
            (r#"{"report\ud83d\ude00":[1]}"#, r#"{"report😀":[1]}"#),
            (r#"{"x\ud83d\ude00":"\uD83D\uDE00\n","\u0072eport":[1],"audit_manifest":["m",null,null,null,"SI",[],[[1]],null]}"#, "{\"x😀\":\"😀\\n\",\"report\":[1],\"audit_manifest\":[\"m\",null,null,null,\"SI\",[],[[1]],null]}"),
            (r#"{"\u0072\u0065\u0070\u006f\u0072\u0074":{"a":[1]},"\u0061ssets":[[1]]}"#, r#"{"report":{"a":[1]},"assets":[[1]]}"#),
        ];
        for (escaped, plain) in escaped_and_plain {
            assert_ne!(escaped, plain);
            assert_eq!(regions_of(escaped), regions_of(plain), "{escaped} against {plain}");
        }
        // The desktop request: every container is in exactly one part.
        let desktop = canonical_json_text(&crate::report_package_save::test_support::desktop_request(&crate::report_package_save::test_support::model()));
        let (opaque, typed, neither) = regions_of(&desktop);
        assert!(opaque > typed && typed > 0 && neither > 0, "{opaque} {typed} {neither}");
    }

    fn canonical_json_text(value: &Value) -> String {
        open_pipe_stress_operation_applier::canonical_json(value)
    }

    /// For W <= 1 the request text's growth never exceeds the former rule,
    /// envelope-free terms included: since C_V + C_T <= M (each priced
    /// container is in at most one region), M + W M + C_V + 4 C_T <=
    /// 2 M + 4 M, and the re-serialization and the error allowance are the
    /// former rule's. The growth is exactly the rule with the current W.
    #[test]
    fn report_package_request_growth_never_exceeds_the_former_rule() {
        const { assert!(REPORT_PACKAGE_WORKING_MEMORY_FACTOR <= 1) };
        use crate::report_package_save::test_support::{desktop_request, model};
        let mut texts: Vec<String> = vec![
            "{}".into(),
            "7".into(),
            r#"{"report":[1]}"#.into(),
            r#"{"source_basis_refs":[[1],[2]]}"#.into(),
            canonical_json_text(&desktop_request(&model())),
            format!(r#"{{"report":[{}0],"source_basis_refs":[{}[1]]}}"#, "[[0]],".repeat(3_000), "[1],".repeat(3_000)),
            format!(r#"{{"source_basis_refs":[{}{{}}]}}"#, "{\"a\":[[[[1]]]]},".repeat(2_000)),
        ];
        texts.push(format!(r#"{{"extra":[{}0]}}"#, "[[0]],".repeat(3_000)));
        for text in &texts {
            let (bound, regions) = ingress::prescan_regions(text.as_bytes(), &REPORT_PACKAGE_REQUEST_REGIONS).unwrap();
            let m = bound.materialization_bytes;
            assert!(regions.opaque + regions.typed <= m, "{text}");
            let former = text_parse_bytes(text, ArgsDecoding::Rebuilt { vec_expansion: REPORT_PACKAGE_VEC_EXPANSION }).unwrap();
            let fixed = 2 * allocation_envelope(text.len() as u64).unwrap() + ARGS_ERROR_ALLOWANCE_BYTES;
            assert_eq!(former, 6 * m + fixed, "the former rule");
            for w in 0..=1u64 {
                let growth = m + w * m + regions.opaque + REPORT_PACKAGE_VEC_EXPANSION * regions.typed + fixed;
                assert!(growth <= former, "W = {w}: {growth} > {former}");
                if w == REPORT_PACKAGE_WORKING_MEMORY_FACTOR {
                    assert_eq!(report_package_request_parse_bytes(text).unwrap(), growth, "the rule with the current W");
                }
            }
        }
        // A text the prescan refuses is its finding; overflow refuses.
        assert_eq!(report_package_request_parse_bytes("[1,").unwrap_err().reason, "unexpected end of input");
    }

    /// Counting-allocator cases for the region rule (the adversarial cases of
    /// its stage-1 measurement): vector-shaped content in the opaque regions,
    /// opaque-shaped content in typed-vector elements, positional structs,
    /// escaped field names and duplicate keys, and a long vector refused
    /// after many decoded elements. Escaped names, duplicates and some
    /// positional forms are not canonical, so the command refuses them before
    /// decoding; the measure decodes them anyway, after the parse and the
    /// binding check's re-serialization, so the rule is checked on the
    /// decoder's paths and not only on canonical texts.
    #[test]
    fn report_package_region_pricing_covers_adversarial_requests() {
        use crate::report_package_save::{parse_canonical, parse_lease_bytes, test_support, typed_request};
        use open_pipe_stress_operation_applier::canonical_json;
        let model = canonical_json(&test_support::model());
        let manifest = canonical_json(&test_support::input_manifest(&test_support::model(), json!([])));
        let one = json!([["analysis_run", "r"]]);
        let reference_arrays = Value::Array(vec![json!([["load_case", "LC1"], ["load_case", "LC2"]]); 3_000]);
        let mut vector_shaped = report_request(one.clone(), Value::Array(vec![reference_arrays.clone(); 2]));
        vector_shaped["report"] = json!({"sections": reference_arrays.clone(), "rows": Value::Array(vec![json!([[[1]]]); 4_000])});
        vector_shaped["state_comparison_handoff_records"] = Value::Array(vec![json!([[1, [2]], {"a": [[3]]}]); 3_000]);
        let opaque_element = json!({"x": [{"y": [1, 2, 3]}, [[4]]], "z": {"w": {}}});
        let mut opaque_shaped = report_request(
            Value::Array(vec![json!({"ref_type": "load_case", "ref_id": "LC1", "extra": opaque_element.clone()}); 3_000]),
            json!([]),
        );
        let pack = {
            let Value::Array(items) = minimal("RulePackAuditRefDto") else { unreachable!() };
            let names = ["rule_pack_id", "rule_pack_version", "source_notice", "redistribution_status", "checksum", "private_payload_redacted"];
            let mut object: serde_json::Map<String, Value> = names.iter().map(|n| n.to_string()).zip(items).collect();
            object.insert("extra".into(), opaque_element.clone());
            Value::Object(object)
        };
        opaque_shaped["audit_manifest"]["rule_pack_refs"] = Value::Array(vec![pack; 2_000]);
        let asset = json!({"asset_id": "a", "payload_kind": "k", "privacy_class": "p", "hash": null, "provenance": "v", "extra": opaque_element});
        opaque_shaped["audit_manifest"]["assets"] = Value::Array(vec![asset; 2_000]);
        // Positional forms: the request and its audit manifest as arrays.
        let positional_audit = json!(["m", null, null, ["s", "v", "b"], "SI",
            Value::Array(vec![minimal("RulePackAuditRefDto"); 4_000]), Value::Array(vec![minimal("AssetManifestEntryDto"); 4_000]),
            minimal("ProfessionalBoundaryDto")]);
        let positional = json!(["pkg", "profile", ["model", "m"], Value::Array(vec![json!(["load_case", "LC1"]); 5_000]),
            {"sections": [1]}, positional_audit, [{"a": [1]}], [], Value::Array(vec![json!([[1]]); 2_000]), null, "s"]);
        let mut positional_refused = positional.clone();
        positional_refused[3].as_array_mut().unwrap().push(json!({}));
        let canonical_cases = [
            ("vector_shaped_opaque_content", canonical_json(&vector_shaped), true),
            ("opaque_shaped_typed_elements", canonical_json(&opaque_shaped), true),
            ("positional_structs", canonical_json(&positional), true),
            ("positional_refused_after_many", canonical_json(&positional_refused), false),
        ];
        // Escaped field names and duplicate keys, written by hand.
        let refs = |n: usize| format!("[{}]", vec![r#"["load_case","LC1"]"#; n].join(","));
        let plain_names = format!(
            r#"{{"package_id":"pkg","export_profile_id":"profile","source_model_ref":["model","m"],"source_basis_refs":{},"report":{{"rows":[{}]}},"audit_manifest":{{"manifest_id":"m","model_hash":null,"input_manifest_hash":null,"solver_version":["s","v","b"],"unit_system_ref":"SI","rule_pack_refs":[{}],"assets":[],"professional_boundary":[true,false,false,false,false]}},"result_documents":[{{"a":[{}]}}],"source_results":[],"state_comparison_handoff_records":[],"rule_check_aggregate":null,"solve_rule_check_status":"s"}}"#,
            refs(5_000),
            vec!["[[1]]"; 3_000].join(","),
            vec![canonical_json(&minimal("RulePackAuditRefDto")); 3_000].join(","),
            vec!["[1]"; 3_000].join(","),
        );
        // The same request with escapes in its field names, attributed as the
        // plain-key text and measured below.
        let mut escaped = plain_names.clone();
        for (name, escaped_name) in [
            (r#""report":"#, r#""\u0072eport":"#),
            (r#""source_basis_refs":"#, r#""source\u005fbasis_refs":"#),
            (r#""audit_manifest":"#, r#""\u0061udit_manifest":"#),
            (r#""rule_pack_refs":"#, r#""rule\u005Fpack_refs":"#),
            (r#""result_documents":"#, r#""result\u005fdocuments":"#),
        ] {
            assert_eq!(escaped.matches(name).count(), 1, "{name}");
            escaped = escaped.replace(name, escaped_name);
        }
        assert_eq!(regions_of(&escaped), regions_of(&plain_names), "escaped field names against plain names");
        let duplicated = format!(
            r#"{{"source_basis_refs":{},"source_basis_refs":{},"report":[{}],"report":[{}],"result_documents":[],"result_documents":[[{}]]}}"#,
            refs(4_000),
            refs(4_000),
            vec!["[1]"; 3_000].join(","),
            vec!["[1]"; 3_000].join(","),
            vec!["[1]"; 3_000].join(","),
        );
        let mut cases: Vec<(&str, String, bool)> = canonical_cases.into_iter().map(|(name, text, decodes)| (name, text, decodes)).collect();
        cases.push(("escaped_field_names", escaped, true));
        cases.push(("duplicate_keys", duplicated, false));
        // Numbers whose canonical form is longer than their text (1e20 is
        // written as 21 digits): the binding check's re-serialization of a
        // non-canonical text outgrows the text's length.
        let mut expanding = report_request(one.clone(), json!([]));
        expanding["report"] = json!("numbers");
        let expanding = canonical_json(&expanding).replace(r#""numbers""#, &format!("[{}]", vec!["1e20"; 200_000].join(",")));
        cases.push(("number_expansion", expanding, true));
        for (name, text, decodes) in cases {
            let growth = parse_lease_bytes(&text, &model, &manifest).unwrap();
            let (regions_opaque, regions_typed, _) = regions_of(&text);
            let (decoded, measured) = ingress::counting::measure(|| {
                let request: Value = serde_json::from_str(&text).expect("JSON");
                // The binding check's re-serialization, built whether or not it matches.
                drop(canonical_json(&request));
                let model = parse_canonical("model", &model).expect("canonical");
                let manifest = parse_canonical("input_manifest", &manifest).expect("canonical");
                drop((model, manifest));
                typed_request(request).map(drop)
            });
            assert_eq!(decoded.is_ok(), decodes, "{name}: {:?}", decoded.err().map(|error| error.message));
            assert!(measured.peak_priced <= growth, "{name}: priced peak {} exceeds the priced growth {growth}", measured.peak_priced);
            eprintln!("{name}: text {} C_V {regions_opaque} C_T {regions_typed} priced peak {} growth {growth}", text.len(), measured.peak_priced);
        }
    }
}
