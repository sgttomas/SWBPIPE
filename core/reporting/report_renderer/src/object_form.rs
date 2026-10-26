//! Object form (DEL-08-01 Design `report-package-current-results.md` §2.6;
//! found in Codex review rounds 2 and 3 of #1648). serde reads a struct written as a
//! JSON array by field order (its positional form), while the native policy
//! checks (export context, exact detection, disposition screen, source
//! binding) read members by key. A struct in positional form therefore reaches
//! the decoder unseen by those checks. [`positional_struct_path`] finds the
//! first such struct at any depth: it serializes the value the decoder
//! produced and walks the original JSON beside it; wherever the decoded value
//! serializes a struct (or a struct variant, or a map) and the original holds
//! an array, the original is in positional form. Collections and tuples
//! serialize as sequences and match arrays, so legitimate arrays pass. The
//! walk builds no serialized copy: the serializer below compares in place and
//! keeps only the path, whose segments are field names, variant names and
//! indexes (a map entry is written `<key>`, so no input text is echoed).
//! Standard claim fence applies (claims taxonomy per DEC-081).

use serde::ser::{self, Serialize};
use serde_json::{Map, Value};
use std::fmt;

/// One path segment.
#[derive(Clone, Copy)]
enum Segment {
    Field(&'static str),
    Index(usize),
    Key,
}

/// Why the walk stopped.
#[derive(Debug)]
enum Stop {
    /// A struct in positional form at this path.
    Positional(String),
    /// The decoded value did not serialize (not expected for these types).
    Unserializable,
}

impl fmt::Display for Stop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stop::Positional(path) => write!(f, "positional at {path}"),
            Stop::Unserializable => f.write_str("unserializable"),
        }
    }
}

impl std::error::Error for Stop {}

impl ser::Error for Stop {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Stop::Unserializable
    }
}

fn path_text(path: &[Segment]) -> String {
    let mut text = String::new();
    for segment in path {
        match segment {
            Segment::Field(name) => {
                if !text.is_empty() {
                    text.push('.');
                }
                text.push_str(name);
            }
            Segment::Index(index) => {
                text.push('[');
                text.push_str(&index.to_string());
                text.push(']');
            }
            Segment::Key => text.push_str(".<key>"),
        }
    }
    if text.is_empty() {
        text.push_str("the input");
    }
    text
}

/// The comparing serializer: `original` is the JSON node the value being
/// serialized was decoded from (`None` where the decoder supplied a default).
struct Compare<'a, 'p> {
    original: Option<&'a Value>,
    path: &'p mut Vec<Segment>,
}

impl<'a> Compare<'a, '_> {
    /// A struct (or map) must have been decoded from an object.
    fn require_object(&self) -> Result<Option<&'a Map<String, Value>>, Stop> {
        match self.original {
            Some(Value::Array(_)) => Err(Stop::Positional(path_text(self.path))),
            Some(Value::Object(map)) => Ok(Some(map)),
            _ => Ok(None),
        }
    }

    /// The externally tagged content of `variant`.
    fn variant(&self, variant: &str) -> Option<&'a Value> {
        match self.original {
            Some(Value::Object(map)) => map.get(variant),
            _ => None,
        }
    }
}

fn child<'a>(original: Option<&'a Map<String, Value>>, key: &str) -> Option<&'a Value> {
    original.and_then(|map| map.get(key))
}

/// Serializes `value` against `original` with `segment` pushed on the path.
fn nested<T: Serialize + ?Sized>(value: &T, original: Option<&Value>, path: &mut Vec<Segment>, segment: Segment) -> Result<(), Stop> {
    path.push(segment);
    let outcome = value.serialize(Compare { original, path: &mut *path });
    path.pop();
    outcome
}

/// A struct's or struct variant's fields, by name (`pop` for a variant,
/// whose name is on the path).
struct Fields<'a, 'p> {
    original: Option<&'a Map<String, Value>>,
    path: &'p mut Vec<Segment>,
    pop: bool,
}

/// A sequence's, tuple's or tuple variant's elements, by position.
struct Elements<'a, 'p> {
    original: Option<&'a Vec<Value>>,
    next: usize,
    path: &'p mut Vec<Segment>,
    pop: bool,
}

/// A map's entries, by key (the key text is captured to look the entry up
/// and never written into the path).
struct Entries<'a, 'p> {
    original: Option<&'a Map<String, Value>>,
    key: Option<String>,
    path: &'p mut Vec<Segment>,
}

impl<'a, 'p> ser::Serializer for Compare<'a, 'p> {
    type Ok = ();
    type Error = Stop;
    type SerializeSeq = Elements<'a, 'p>;
    type SerializeTuple = Elements<'a, 'p>;
    type SerializeTupleStruct = Elements<'a, 'p>;
    type SerializeTupleVariant = Elements<'a, 'p>;
    type SerializeMap = Entries<'a, 'p>;
    type SerializeStruct = Fields<'a, 'p>;
    type SerializeStructVariant = Fields<'a, 'p>;

    fn serialize_bool(self, _: bool) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_i8(self, _: i8) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_i16(self, _: i16) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_i32(self, _: i32) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_i64(self, _: i64) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_i128(self, _: i128) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_u8(self, _: u8) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_u16(self, _: u16) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_u32(self, _: u32) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_u64(self, _: u64) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_u128(self, _: u128) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_f32(self, _: f32) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_f64(self, _: f64) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_char(self, _: char) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_str(self, _: &str) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_none(self) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_unit(self) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Stop> {
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), Stop> {
        value.serialize(self)
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<(), Stop> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _: &'static str, _: u32, variant: &'static str, value: &T) -> Result<(), Stop> {
        let content = self.variant(variant);
        nested(value, content, self.path, Segment::Field(variant))
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Elements<'a, 'p>, Stop> {
        let original = match self.original {
            Some(Value::Array(items)) => Some(items),
            _ => None,
        };
        Ok(Elements { original, next: 0, path: self.path, pop: false })
    }
    fn serialize_tuple(self, len: usize) -> Result<Elements<'a, 'p>, Stop> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_struct(self, _: &'static str, len: usize) -> Result<Elements<'a, 'p>, Stop> {
        self.serialize_seq(Some(len))
    }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, variant: &'static str, _: usize) -> Result<Elements<'a, 'p>, Stop> {
        let original = self.variant(variant).and_then(Value::as_array);
        self.path.push(Segment::Field(variant));
        Ok(Elements { original, next: 0, path: self.path, pop: true })
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Entries<'a, 'p>, Stop> {
        let original = self.require_object()?;
        Ok(Entries { original, key: None, path: self.path })
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Fields<'a, 'p>, Stop> {
        let original = self.require_object()?;
        Ok(Fields { original, path: self.path, pop: false })
    }
    fn serialize_struct_variant(self, _: &'static str, _: u32, variant: &'static str, _: usize) -> Result<Fields<'a, 'p>, Stop> {
        let content = self.variant(variant);
        self.path.push(Segment::Field(variant));
        let original = match content {
            Some(Value::Array(_)) => return Err(Stop::Positional(path_text(self.path))),
            Some(Value::Object(map)) => Some(map),
            _ => None,
        };
        Ok(Fields { original, path: self.path, pop: true })
    }
}

impl ser::SerializeStruct for Fields<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Stop> {
        nested(value, child(self.original, key), self.path, Segment::Field(key))
    }
    fn end(self) -> Result<(), Stop> {
        if self.pop {
            self.path.pop();
        }
        Ok(())
    }
}

impl ser::SerializeStructVariant for Fields<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Stop> {
        nested(value, child(self.original, key), self.path, Segment::Field(key))
    }
    fn end(self) -> Result<(), Stop> {
        if self.pop {
            self.path.pop();
        }
        Ok(())
    }
}

impl Elements<'_, '_> {
    fn element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        let original = self.original.and_then(|items| items.get(self.next));
        let index = self.next;
        self.next += 1;
        nested(value, original, self.path, Segment::Index(index))
    }

    fn finish(self) -> Result<(), Stop> {
        if self.pop {
            self.path.pop();
        }
        Ok(())
    }
}

impl ser::SerializeSeq for Elements<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        self.element(value)
    }
    fn end(self) -> Result<(), Stop> {
        self.finish()
    }
}

impl ser::SerializeTuple for Elements<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        self.element(value)
    }
    fn end(self) -> Result<(), Stop> {
        self.finish()
    }
}

impl ser::SerializeTupleStruct for Elements<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        self.element(value)
    }
    fn end(self) -> Result<(), Stop> {
        self.finish()
    }
}

impl ser::SerializeTupleVariant for Elements<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        self.element(value)
    }
    fn end(self) -> Result<(), Stop> {
        self.finish()
    }
}

/// A map key's text, when the key is a string, a number or a unit variant
/// (the forms a JSON object key decodes from); otherwise none.
struct KeyText;

type NoKey = ser::Impossible<Option<String>, Stop>;

impl ser::Serializer for KeyText {
    type Ok = Option<String>;
    type Error = Stop;
    type SerializeSeq = NoKey;
    type SerializeTuple = NoKey;
    type SerializeTupleStruct = NoKey;
    type SerializeTupleVariant = NoKey;
    type SerializeMap = NoKey;
    type SerializeStruct = NoKey;
    type SerializeStructVariant = NoKey;

    fn serialize_str(self, value: &str) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_bool(self, value: bool) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_i8(self, value: i8) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_i16(self, value: i16) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_i32(self, value: i32) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_i64(self, value: i64) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_u8(self, value: u8) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_u16(self, value: u16) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_u32(self, value: u32) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_u64(self, value: u64) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_f32(self, _: f32) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_f64(self, _: f64) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_char(self, value: char) -> Result<Option<String>, Stop> {
        Ok(Some(value.to_string()))
    }
    fn serialize_bytes(self, _: &[u8]) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_none(self) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<Option<String>, Stop> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, variant: &'static str) -> Result<Option<String>, Stop> {
        Ok(Some(variant.to_string()))
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, value: &T) -> Result<Option<String>, Stop> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Option<String>, Stop> {
        Ok(None)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_tuple(self, _: usize) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
    fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<NoKey, Stop> {
        Err(Stop::Unserializable)
    }
}

impl ser::SerializeMap for Entries<'_, '_> {
    type Ok = ();
    type Error = Stop;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Stop> {
        self.key = key.serialize(KeyText)?;
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Stop> {
        let original = self.key.take().and_then(|key| child(self.original, &key));
        nested(value, original, self.path, Segment::Key)
    }
    fn end(self) -> Result<(), Stop> {
        Ok(())
    }
}

/// The path of the first struct, at any depth, that `original` writes in
/// positional (array) form, given `decoded`, the value the decoder produced
/// from `original`; `None` when every struct is an object. A decoded value
/// that does not serialize is reported at the root.
pub fn positional_struct_path<T: Serialize + ?Sized>(decoded: &T, original: &Value) -> Option<String> {
    let mut path = Vec::new();
    match decoded.serialize(Compare { original: Some(original), path: &mut path }) {
        Ok(()) => None,
        Err(Stop::Positional(path)) => Some(path),
        Err(Stop::Unserializable) => Some("the input".to_string()),
    }
}

/// [`positional_struct_path`] for a renderer input: decoded by reference (a
/// copy of the input's data, dropped before this returns), then compared. An
/// input that does not decode yields `None`: no output can be produced from
/// it, and the renderer's own decode refuses it later with its value-free
/// message.
pub fn report_positional_struct_path(original: &Value) -> Option<String> {
    use serde::Deserialize;
    let decoded = crate::RenderableReportInput::deserialize(original).ok()?;
    positional_struct_path(&decoded, original)
}
