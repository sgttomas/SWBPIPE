//! The parity vectors' shared reading: the project root, digests, base64, binary64 bits, and the value store
//! (`blobs` and `values` with their edit ops) that `tools/fixtures/caepipe_mbf_import_vectors.py` writes and
//! `caepipe_mbf_writer_vectors.py` reuses.
#![allow(dead_code)]

use open_pipe_stress_caepipe_mbf::{self as mbf, Dict, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const NONFINITE: &str = "\u{0}float";

pub fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

pub fn b64(text: &str) -> Vec<u8> {
    let table = |c: u8| -> u32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => panic!("base64 {c}"),
        }
    };
    let clean: Vec<u8> = text.bytes().filter(|byte| *byte != b'=').collect();
    let mut out = Vec::new();
    for chunk in clean.chunks(4) {
        let mut acc = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            acc |= table(*byte) << (18 - 6 * index);
        }
        let bytes = [(acc >> 16) as u8, (acc >> 8) as u8, acc as u8];
        out.extend_from_slice(&bytes[..chunk.len() - 1]);
    }
    out
}

pub fn f64_bits(text: &str) -> f64 {
    let bytes: Vec<u8> = (0..8).map(|index| u8::from_str_radix(&text[2 * index..2 * index + 2], 16).expect("hex")).collect();
    f64::from_le_bytes(bytes.try_into().expect("8 bytes"))
}

pub fn f64_hex(value: f64) -> String {
    hex(&value.to_le_bytes())
}

pub struct Inputs<'d> {
    doc: &'d Value,
    blobs: HashMap<String, Vec<u8>>,
    values: HashMap<String, Value>,
}

impl<'d> Inputs<'d> {
    pub fn new(doc: &'d Value) -> Self {
        Inputs { doc, blobs: HashMap::new(), values: HashMap::new() }
    }

    pub fn blob(&mut self, key: &str) -> Vec<u8> {
        if let Some(found) = self.blobs.get(key) {
            return found.clone();
        }
        let entry = self.doc.get("blobs").and_then(|blobs| blobs.get(key)).expect("blob").clone();
        let bytes = if let Some(Value::Str(text)) = entry.get("b64") {
            b64(text)
        } else if let Some(Value::Str(text)) = entry.get("text") {
            text.as_bytes().to_vec()
        } else {
            let base: Vec<char> = String::from_utf8(self.blob(entry.get_or_null("base").as_str().expect("base"))).expect("utf-8").chars().collect();
            let prefix = mbf::value::small_int(entry.get_or_null("prefix")).expect("prefix") as usize;
            let suffix = mbf::value::small_int(entry.get_or_null("suffix")).expect("suffix") as usize;
            let middle = entry.get_or_null("middle").as_str().expect("middle");
            let mut text: String = base[..prefix].iter().collect();
            text.push_str(middle);
            text.extend(base[base.len() - suffix..].iter());
            text.into_bytes()
        };
        self.blobs.insert(key.to_string(), bytes.clone());
        bytes
    }

    pub fn value(&mut self, key: &Value) -> Option<Value> {
        let key = key.as_str()?;
        Some(restore(&self.value_plain(key)))
    }

    pub fn value_plain(&mut self, key: &str) -> Value {
        if let Some(found) = self.values.get(key) {
            return found.clone();
        }
        let entry = self.doc.get("values").and_then(|values| values.get(key)).expect("value").clone();
        let value = match entry.get("value") {
            Some(value) => value.clone(),
            None => {
                let mut value = self.value_plain(entry.get_or_null("base").as_str().expect("base"));
                for op in entry.get_or_null("ops").as_list().expect("ops") {
                    apply(&mut value, op.as_list().expect("op"));
                }
                value
            }
        };
        self.values.insert(key.to_string(), value.clone());
        value
    }
}

pub fn apply(value: &mut Value, op: &[Value]) {
    let kind = op[0].as_str().expect("kind");
    let path = op[1].as_list().expect("path");
    if path.is_empty() {
        *value = op[2].clone();
        return;
    }
    let mut parent = value;
    for step in &path[..path.len() - 1] {
        parent = step_into(parent, step);
    }
    let last = &path[path.len() - 1];
    match (parent, last) {
        (Value::Dict(dict), Value::Str(key)) => {
            if kind == "del" {
                dict.remove(key);
            } else {
                dict.insert(key.clone(), op[2].clone());
            }
        }
        (Value::List(items), Value::Int(_)) => {
            let index = mbf::value::small_int(last).expect("index") as usize;
            items[index] = op[2].clone();
        }
        _ => panic!("an op path that does not fit its value"),
    }
}

pub fn step_into<'v>(value: &'v mut Value, step: &Value) -> &'v mut Value {
    match (value, step) {
        (Value::Dict(dict), Value::Str(key)) => dict.get_mut(key).expect("key"),
        (Value::List(items), Value::Int(_)) => &mut items[mbf::value::small_int(step).expect("index") as usize],
        _ => panic!("an op path that does not fit its value"),
    }
}

pub fn restore(value: &Value) -> Value {
    match value {
        Value::Dict(dict) => {
            if dict.len() == 1 {
                if let Some(Value::Str(kind)) = dict.get(&NONFINITE.to_string()) {
                    return Value::Float(match kind.as_str() {
                        "nan" => f64::NAN,
                        "inf" => f64::INFINITY,
                        _ => f64::NEG_INFINITY,
                    });
                }
            }
            let mut out = Dict::new();
            for (key, item) in dict.iter() {
                out.insert(key.clone(), restore(item));
            }
            Value::Dict(out)
        }
        Value::List(items) => Value::List(items.iter().map(restore).collect()),
        other => other.clone(),
    }
}


/// A vectors file's bytes, parsed strictly (each test reads its own files, so their paths stay literals for the
/// hosted runtime-read declarations).
pub fn parse(bytes: &[u8]) -> Value {
    mbf::parse_sidecar_json(bytes).expect("the vectors are strict JSON")
}
