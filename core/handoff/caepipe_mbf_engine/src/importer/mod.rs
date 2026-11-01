//! The MBF importer (importer.py `_Importer`), stage for stage. Each method carries its Python name.

mod assemble;
mod build;
mod finish;
mod pressure;

use crate::dict;
use crate::pyfmt::py_repr;
use crate::pyjson::{parse_sidecar_json, SidecarJsonError};
use crate::pystr::{
    match_keyword_like, match_real, match_row_key, match_sha256, py_isupper, py_repr_str, py_strip, rstrip_chars,
    skip_chars, split_first, PyError,
};
use crate::rational::{fraction_is_zero, parse_fraction, Parsed, Q};
use crate::sidecar::{sidecar_self_digest, SidecarParser, DIGESTED_FORMATS, SHEAR_MODELS, SIDECAR_DIGEST_KEY};
use crate::value::{Dict, OMap, Value};
use crate::{ImportError, ImportOptions, ImportResult, SidecarInput};
use sha2::{Digest, Sha256};

pub(crate) const IMPORT_PROFILE_ID: &str = "caepipe-mbf-import-1";
pub(crate) const REPORT_KIND: &str = "swbpipe.caepipe_mbf_import_report";
pub(crate) const MODEL_KIND: &str = "openpipestress.product_preview.model";
pub(crate) const MBF_VERSION: &str = "15.10";
pub(crate) const ACCEPTED_VERSIONS: [&str; 1] = [MBF_VERSION];
pub(crate) const PD1_VERSIONS: [&str; 1] = [MBF_VERSION];

const SECTION_KEYWORDS: [(&str, &str); 37] = [
    ("VERSION", "version"), ("OPTIONS", "options"), ("OPCODE", "opcode"), ("OPTEMP", "optemp"),
    ("OPPRES", "oppres"), ("OPMISC", "opmisc"), ("MATERIAL", "material"), ("PIPE", "pipe"), ("LOADS", "loads"),
    ("LAYOUT", "layout"), ("SOILS", "refused"), ("BMATERIALS", "refused"), ("BSECTIONS", "refused"),
    ("BLOADS", "refused"), ("PUMPS", "refused"), ("COMPRESSORS", "refused"), ("TURBINES", "refused"),
    ("ACHXS", "refused"), ("FIHTR", "refused"), ("SEISMIC", "zero"), ("WIND", "zero"), ("ASCES", "zero"),
    ("ASCEW", "zero"), ("1991W", "zero"), ("WIND1", "zero"), ("WIND2", "zero"), ("WIND3", "zero"),
    ("WIND4", "zero"), ("SNOWICE", "zero"), ("WAVE1", "zero"), ("WAVE2", "zero"), ("WAVE3", "zero"),
    ("WAVE4", "zero"), ("FCURVE", "delegated"), ("FCYCLE", "delegated"), ("ALLOWABLES", "delegated"),
    ("SPCMS", "delegated"),
];
const SECTION_KEYWORDS_TAIL: [(&str, &str); 4] =
    [("SPLVL", "delegated"), ("PSDDT", "delegated"), ("PSDLS", "delegated"), ("PSDLD", "delegated")];

fn section_kind(name: &str) -> Option<&'static str> {
    SECTION_KEYWORDS.iter().chain(SECTION_KEYWORDS_TAIL.iter()).find(|(key, _)| *key == name).map(|(_, kind)| *kind)
}

fn section_names() -> impl Iterator<Item = &'static str> {
    SECTION_KEYWORDS.iter().chain(SECTION_KEYWORDS_TAIL.iter()).map(|(key, _)| *key)
}

pub(crate) const HANGER_OPTIONS: [&str; 11] =
    ["HGRA", "HGRB", "HGRC", "HGRD", "HGRE", "HGRF", "HGRG", "HGRH", "HGRI", "HGRJ", "HGRK"];
pub(crate) const PIPING_CODE_OPTIONS: [&str; 42] = [
    "B311", "B311-67", "B311-73", "B311-77", "B311-80", "B313", "B314", "B315", "B318", "B319", "B3112",
    "B3112-PL", "ASME-NM1", "ASME-NM2", "ASME", "ASME-86", "ASME-92", "ASME-2015", "ASME-2017", "ASME-2021",
    "ASME-2023", "ASME-ND-2017", "ASME-ND-2021", "ASME-ND-2023", "BS806", "DNV-ST", "NORWEIGIAN-83",
    "NORWEIGIAN-90", "IGEM", "RCC-M", "RCC-M-2018", "RCC-M-2020", "RCC-M-2022", "SNCT", "SWEDISH",
    "STOOMWEZEN", "Z183", "Z184", "Z662", "EUROPEAN", "EN13941", "ISO14692",
];
const OPMISC_EXPORT_FIELDS: usize = 13;
const GRAVITY_MIN_DIGITS: usize = 3;
const FIELD_OPTEMP: usize = 5;
const FIELD_OPMISC: usize = 10;
const FIELD_MATERIAL_FIRST: usize = 13;
const FIELD_MATERIAL_TEMPERATURE: usize = 7;
const FIELD_PIPE: usize = 10;
const FIELD_LOADS: usize = 35;
fn field_count(name: &str) -> usize {
    match name {
        "OPCODE" => 9,
        "OPTEMP" => FIELD_OPTEMP,
        "OPPRES" => 5,
        "OPMISC" => FIELD_OPMISC,
        _ => unreachable!("field count of {name}"),
    }
}
const MATERIAL_FRP_TYPES: [&str; 3] = ["FR", "F1", "F2"];
const MATERIAL_FIRST_PROTECTED: [(usize, &str); 6] = [
    (8, "yield strength"), (9, "tensile strength"), (10, "slope M"), (11, "constant"), (12, "endurance"),
    (13, "fatigue curve name"),
];
const MATERIAL_TEMPERATURE_PROTECTED: [(usize, &str); 3] =
    [(5, "allowable stress"), (6, "yield stress"), (7, "rupture stress")];
const PIPE_ZERO_FIELDS: [(usize, &str); 6] = [
    (4, "corrosion allowance"), (5, "mill tolerance"), (6, "insulation density"), (7, "insulation thickness"),
    (8, "lining density"), (9, "lining thickness"),
];
const OPPRES_DEFAULTS: [&str; 5] = ["0", "1", "1", "1.00", "0"];

fn loads_tail() -> Vec<String> {
    let mut tail: Vec<String> = ["T2", "P2", "T3", "P3", "additional weight", "peak temperature", "peak pressure"]
        .iter().map(|item| item.to_string()).collect();
    for n in 4..11 {
        tail.push(format!("T{n}"));
        tail.push(format!("P{n}"));
    }
    for item in ["Wind 1", "Wind 2", "Wind 3", "Wind 4", "Snow", "Ice", "Wave", "Wave 2", "Wave 3", "Wave 4"] {
        tail.push(item.to_string());
    }
    tail
}

/// A parsed MBF number: binary64 value, exact rational and the printed text.
#[derive(Clone, Debug)]
pub(crate) struct Num {
    pub value: f64,
    pub exact: Q,
    pub text: String,
}

impl Num {
    pub fn one() -> Num {
        Num { value: 1.0, exact: Q::from_int(1), text: "1".into() }
    }
}

/// `_negated(number)`.
pub(crate) fn negated(number: &Num) -> Num {
    let text = if let Some(rest) = number.text.strip_prefix('-') {
        rest.to_string()
    } else {
        format!("-{}", number.text.trim_start_matches('+'))
    };
    Num { value: 0.0 - number.value, exact: number.exact.neg(), text }
}

/// Why a stage stops: `_Refused`, or something the caller sees as an error.
pub(crate) enum Stop {
    Refused,
    Fail(ImportError),
}

impl From<PyError> for Stop {
    fn from(error: PyError) -> Stop {
        Stop::Fail(ImportError::Python(error))
    }
}

pub(crate) type R<T> = Result<T, Stop>;

pub(crate) fn index_error() -> Stop {
    Stop::Fail(ImportError::Python(PyError { kind: "IndexError", message: "list index out of range".into() }))
}

#[derive(Clone, Debug)]
pub(crate) struct Row {
    pub line: usize,
    pub kind: char,
    pub number: u32,
    pub absolute: bool,
    pub keys: OMap<char, String>,
    pub comments: Vec<(usize, String)>,
    pub started: bool,
}

pub(crate) enum LayoutEntry {
    Note(usize, String),
    Row(Row),
}

#[derive(Clone, Debug)]
pub(crate) struct Element {
    pub line: usize,
    pub start: u32,
    pub end: u32,
    pub material: String,
    pub section: String,
    pub load: Option<String>,
    pub ti: bool,
    pub radius: Option<Num>,
    pub factor: Option<Num>,
    /// BSIF(in-plane,out-plane)
    pub sifs: Option<(Num, Num)>,
    pub row: i64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct MaterialRec {
    pub line: usize,
    pub rows: Vec<(Num, Num, Num)>,
    pub refused: bool,
    pub density: Option<Num>,
    pub nu: Option<Num>,
    pub description: String,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct PipeRec {
    pub line: usize,
    pub refused: bool,
    pub od: Option<Num>,
    pub thk: Option<Num>,
    pub allowance_given: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct LoadRec {
    pub line: usize,
    pub refused: bool,
    pub t1: Option<Num>,
    pub p1: Option<Num>,
}

pub(crate) struct Importer<'a> {
    pub data: &'a [u8],
    pub options: &'a ImportOptions<'a>,
    pub derive_only: bool,
    pub derived: Option<Vec<((String, String), [f64; 3])>>,
    pub sidecar: Option<Value>,
    pub sidecar_current: bool,
    /// The parsed sidecar's format; None for a legacy sidecar or none.
    pub sidecar_format: Option<i64>,
    pub self_digest_state: Option<&'static str>,
    pub oppres: Option<(usize, Vec<String>, bool)>,
    pub pressure_notes: Vec<String>,
    pub diagnostics: Vec<Value>,
    pub loss: Vec<Value>,
    pub assumptions: Vec<String>,
    pub target_options: Dict,
    pub empty_comments: i64,
    pub delegated_sections: Dict,
    pub pressure_regions: Option<usize>,
    pub sha: String,
    pub version: Option<String>,
    pub si: bool,
    pub rad: bool,
    pub z_vertical: bool,
    pub lines: Vec<(usize, String)>,
    pub heading: Vec<(usize, String)>,
    pub sections: OMap<String, (usize, Vec<(usize, String)>)>,
    pub t_ref: Option<Num>,
    pub materials: OMap<String, MaterialRec>,
    pub pipes: OMap<String, PipeRec>,
    pub loads: OMap<String, LoadRec>,
    pub notes: Vec<String>,
    pub bound: bool,
    pub rebound: bool,
    pub maps: Value,
    pub node_ids: OMap<String, String>,
    pub used_conventions: Vec<(String, String)>,
    pub thermal_load_count: Option<Q>,
    pub unconfirmed: Vec<String>,
    pub build: build::BuildState,
}

impl<'a> Importer<'a> {
    pub fn new(data: &'a [u8], options: &'a ImportOptions<'a>, derive_only: bool) -> Self {
        Importer {
            data,
            options,
            derive_only,
            derived: None,
            sidecar: None,
            sidecar_current: false,
            sidecar_format: None,
            self_digest_state: None,
            oppres: None,
            pressure_notes: Vec::new(),
            diagnostics: Vec::new(),
            loss: Vec::new(),
            assumptions: Vec::new(),
            target_options: Dict::new(),
            empty_comments: 0,
            delegated_sections: Dict::new(),
            pressure_regions: None,
            sha: String::new(),
            version: None,
            si: false,
            rad: false,
            z_vertical: false,
            lines: Vec::new(),
            heading: Vec::new(),
            sections: OMap::new(),
            t_ref: None,
            materials: OMap::new(),
            pipes: OMap::new(),
            loads: OMap::new(),
            notes: Vec::new(),
            bound: false,
            rebound: false,
            maps: Value::Dict(Dict::new()),
            node_ids: OMap::new(),
            used_conventions: Vec::new(),
            thermal_load_count: None,
            unconfirmed: Vec::new(),
            build: build::BuildState::default(),
        }
    }

    // -- reporting -------------------------------------------------------
    pub fn refuse(&mut self, code: &str, line: usize, location: &str, message: &str) {
        self.diagnostics.push(dict![
            ("code", Value::str(code)),
            ("severity", Value::str("error")),
            ("line", Value::int(line as i64)),
            ("location", Value::str(location)),
            ("message", Value::str(message)),
        ]);
        self.loss.push(dict![
            ("location", Value::str(location)),
            ("line", Value::int(line as i64)),
            ("category", Value::str("unsupported")),
            ("code", Value::str(code)),
            ("reason", Value::str(message)),
        ]);
    }

    pub fn listed(&mut self, category: &str, code: &str, line: usize, location: &str, message: &str) {
        self.diagnostics.push(dict![
            ("code", Value::str(code)),
            ("severity", Value::str("warning")),
            ("line", Value::int(line as i64)),
            ("location", Value::str(location)),
            ("message", Value::str(message)),
        ]);
        self.loss.push(dict![
            ("location", Value::str(location)),
            ("line", Value::int(line as i64)),
            ("category", Value::str(category)),
            ("code", Value::str(code)),
            ("reason", Value::str(message)),
        ]);
    }

    pub fn info(&mut self, code: &str, line: usize, location: &str, message: &str) {
        self.diagnostics.push(dict![
            ("code", Value::str(code)),
            ("severity", Value::str("info")),
            ("line", Value::int(line as i64)),
            ("location", Value::str(location)),
            ("message", Value::str(message)),
        ]);
    }

    /// `incomplete(map_name, in_file, in_sidecar, what)` with items already as their `str()`.
    pub fn incomplete(&mut self, map_name: &str, in_file: &[String], in_sidecar: &[String], what: &str) {
        let missing: Vec<&String> = in_file.iter().filter(|item| !in_sidecar.contains(item)).collect();
        let extra: Vec<&String> = in_sidecar.iter().filter(|item| !in_file.contains(item)).collect();
        if missing.is_empty() && extra.is_empty() {
            return;
        }
        let join = |items: &[&String]| items.iter().map(|item| item.as_str()).collect::<Vec<_>>().join(", ");
        let mut parts = Vec::new();
        if !missing.is_empty() {
            parts.push(format!("no entry for {}", join(&missing)));
        }
        if !extra.is_empty() {
            parts.push(format!("entries for {}, not in the file", join(&extra)));
        }
        let message = format!("the bound sidecar's {map_name} does not match the file's {what}: {}", parts.join("; "));
        self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, map_name, &message);
    }

    pub fn loads_line(&self) -> usize {
        self.sections.get(&"LOADS".to_string()).map(|(line, _)| *line).unwrap_or(0)
    }

    pub fn has_errors(&self) -> bool {
        self.errors() > 0
    }

    pub fn errors(&self) -> usize {
        self.diagnostics.iter().filter(|item| item.get("severity").and_then(Value::as_str) == Some("error")).count()
    }

    fn result(&self, model: Option<Value>, id_map: Value) -> ImportResult {
        let refused = self.has_errors();
        let binding = if self.bound { "bound" } else if self.rebound { "rebound_unconfirmed" } else { "unbound" };
        let source = dict![
            ("mbf_sha256", Value::str(self.sha.clone())),
            ("mbf_version", self.version.clone().map(Value::Str).unwrap_or(Value::Null)),
            ("unit_mode", Value::str(if self.si { "SI" } else { "English" })),
            ("angles", Value::str(if self.rad { "rad" } else { "deg" })),
            ("vertical_axis", Value::str(if self.z_vertical { "Z" } else { "Y" })),
            ("line_count", Value::int(self.lines.len() as i64)),
            ("source_label", Value::str(self.options.source_label.clone())),
            ("binding", Value::str(binding)),
            ("sidecar_self_digest", self.self_digest_state.map(Value::str).unwrap_or(Value::Null)),
        ];
        let mut summary = Dict::new();
        summary.insert("empty_comments".into(), Value::int(self.empty_comments));
        summary.insert("delegated_sections".into(), Value::Dict(self.delegated_sections.clone()));
        if let Some(count) = self.pressure_regions {
            summary.insert("pressure_regions".into(), Value::int(count as i64));
        }
        ImportResult {
            status: if refused { "refused" } else { "imported" }.to_string(),
            model: if refused { None } else { model },
            diagnostics: self.diagnostics.clone(),
            loss_report: self.loss.clone(),
            assumptions: self.assumptions.clone(),
            target_options: self.target_options.clone(),
            id_map: if refused { Value::Dict(Dict::new()) } else { id_map },
            source,
            summary: Value::Dict(summary),
        }
    }

    // -- run -------------------------------------------------------------
    pub fn run(&mut self) -> Result<ImportResult, ImportError> {
        if py_strip(&self.options.source_label).is_empty() {
            return Err(ImportError::Python(PyError::value("source_label must be a non-empty string")));
        }
        self.sha = crate::sidecar::hex(&Sha256::digest(self.data));
        match self.stages() {
            Ok(Some((model, id_map))) => {
                if self.has_errors() {
                    return Ok(self.result(None, Value::Dict(Dict::new())));
                }
                self.add_assumptions().map_err(ImportError::Python)?;
                Ok(self.result(Some(model), id_map))
            }
            Ok(None) | Err(Stop::Refused) => Ok(self.result(None, Value::Dict(Dict::new()))),
            Err(Stop::Fail(error)) => Err(error),
        }
    }

    fn stages(&mut self) -> R<Option<(Value, Value)>> {
        self.decode()?;
        self.split_sections()?;
        self.read_version()?;
        self.read_options()?;
        self.bind_sidecar()?;
        if let Some(shear) = &self.options.shear_model {
            let valid = matches!(shear, Value::Str(text) if SHEAR_MODELS.contains(&text.as_str()));
            if !valid {
                let message = format!(
                    "the shear_model argument {} is not one of {}",
                    py_repr(shear)?,
                    SHEAR_MODELS.join(", ")
                );
                self.refuse("MBF-IMPORT-SHEAR-MODEL-INVALID", 0, "shear_model", &message);
            }
        }
        self.read_analysis_options()?;
        self.read_materials()?;
        self.read_pipes()?;
        self.read_loads()?;
        self.read_other_sections()?;
        let rows = self.read_layout_rows()?;
        if self.has_errors() {
            return Ok(None);
        }
        let (model, id_map) = self.build(rows)?;
        Ok(Some((model, id_map)))
    }

    // -- §4.1 bytes, lines and encoding ----------------------------------
    fn decode(&mut self) -> R<()> {
        let text = match std::str::from_utf8(self.data) {
            Ok(text) => text,
            Err(error) => {
                let start = error.valid_up_to();
                self.refuse(
                    "MBF-IMPORT-ENCODING-UNSUPPORTED",
                    0,
                    &format!("byte {start}"),
                    &format!("the MBF is not strict UTF-8: the byte at offset {start} cannot be decoded (TBD-17-04-006)"),
                );
                return Err(Stop::Refused);
            }
        };
        if text.starts_with('\u{feff}') {
            self.refuse(
                "MBF-IMPORT-ENCODING-UNSUPPORTED",
                1,
                "byte 0",
                "the MBF starts with a byte-order mark, which no admitted source shows (TBD-17-04-006)",
            );
            return Err(Stop::Refused);
        }
        let unified = text.replace("\r\n", "\n");
        if let Some(at) = unified.find('\r') {
            let line = unified[..at].matches('\n').count() + 1;
            self.refuse(
                "MBF-IMPORT-LINE-END-UNSUPPORTED",
                line,
                &format!("line {line}"),
                "a lone carriage return ends a line; only CRLF and LF line ends are read",
            );
            return Err(Stop::Refused);
        }
        let mut parts: Vec<&str> = unified.split('\n').collect();
        if parts.last() == Some(&"") {
            parts.pop();
        }
        self.lines =
            parts.iter().enumerate().map(|(index, part)| (index + 1, rstrip_chars(part, &[' ', '\t']).to_string())).collect();
        Ok(())
    }

    // -- §4.2 heading and sections -----------------------------------------
    fn keyword(&mut self, text: &str, line: usize) -> R<Option<String>> {
        if section_kind(text).is_some() {
            return Ok(Some(text.to_string()));
        }
        if text.chars().count() == 3 && py_isupper(text) && !text.contains(',') {
            let matches: Vec<&str> = section_names().filter(|name| name.starts_with(text)).collect();
            if matches.len() == 1 {
                return Ok(Some(matches[0].to_string()));
            }
            if matches.len() > 1 {
                let mut sorted = matches.clone();
                sorted.sort();
                let message = format!(
                    "the three-character keyword {} names {} documented sections ({}); write the full keyword",
                    py_repr_str(text),
                    matches.len(),
                    sorted.join(", ")
                );
                self.refuse("MBF-IMPORT-SECTION-AMBIGUOUS", line, &format!("line {line}"), &message);
                return Err(Stop::Refused);
            }
        }
        Ok(None)
    }

    fn split_sections(&mut self) -> R<()> {
        let mut current: Option<String> = None;
        let lines = self.lines.clone();
        for (line, text) in lines {
            let stripped = py_strip(&text).to_string();
            let name = if stripped.is_empty() { None } else { self.keyword(&stripped, line)? };
            if let Some(name) = name {
                if self.sections.contains_key(&name) {
                    self.refuse("MBF-IMPORT-SECTION-REPEATED", line, &name, &format!("section {name} appears a second time"));
                    return Err(Stop::Refused);
                }
                self.sections.insert(name.clone(), (line, Vec::new()));
                current = Some(name);
                continue;
            }
            let Some(section) = current.clone() else {
                self.heading.push((line, text));
                continue;
            };
            let kind = section_kind(&section).expect("section");
            let option_word = HANGER_OPTIONS.contains(&stripped.as_str())
                || PIPING_CODE_OPTIONS.contains(&stripped.as_str())
                || stripped == "SI"
                || stripped == "RAD";
            if !matches!(kind, "delegated" | "zero")
                && match_keyword_like(&stripped)
                && !(section == "LAYOUT" && match_row_key(&stripped).is_some())
                && !(section == "OPTIONS" && option_word)
            {
                let message = format!(
                    "{} is not a documented section keyword and does not parse as a {section} record",
                    py_repr_str(&stripped)
                );
                self.refuse("MBF-IMPORT-SECTION-UNKNOWN", line, &format!("line {line}"), &message);
                continue;
            }
            let free = section == "LAYOUT" && stripped.starts_with('*');
            if !free && section != "MATERIAL" {
                let allowed: &[char] = if section == "LAYOUT" { &['\u{2019}', '\u{201d}'] } else { &[] };
                self.check_characters(line, &stripped, allowed);
            }
            self.sections.get_mut(&section).expect("section").1.push((line, stripped));
        }
        Ok(())
    }

    pub fn check_characters(&mut self, line: usize, text: &str, allowed: &[char]) -> bool {
        for c in text.chars() {
            if (c as u32) > 127 && !allowed.contains(&c) {
                let message = format!(
                    "character U+{:04X} outside free text is not read; only ASCII and the typographic foot and inch \
                     marks in lengths are (TBD-17-04-006)",
                    c as u32
                );
                self.refuse("MBF-IMPORT-CHARACTER-UNSUPPORTED", line, &format!("line {line}"), &message);
                return false;
            }
        }
        true
    }

    pub fn data_lines(&self, name: &str) -> Vec<(usize, String)> {
        self.sections
            .get(&name.to_string())
            .map(|(_, entries)| entries.iter().filter(|(_, text)| !text.is_empty()).cloned().collect())
            .unwrap_or_default()
    }

    pub fn fields(&mut self, line: usize, text: &str, location: &str) -> Option<Vec<String>> {
        let result = split_fields(text);
        if result.is_none() {
            self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, location, "parentheses do not balance");
        }
        result
    }

    pub fn too_many(&mut self, fields: &[String], limit: usize, line: usize, location: &str) -> bool {
        if fields.len() > limit {
            let version = self.version.clone().unwrap_or_else(|| "None".into());
            let message = format!(
                "{location} has {} fields; MBF {version} documents {limit}, and field {} onward has no public description",
                fields.len(),
                limit + 1
            );
            self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", line, location, &message);
            return true;
        }
        false
    }

    pub fn real(&mut self, text: &str, line: usize, location: &str) -> R<Option<Num>> {
        if text.is_empty() {
            self.refuse("MBF-IMPORT-FIELD-MISSING", line, location, &format!("{location} is required and empty"));
            return Ok(None);
        }
        if text.chars().any(|c| (c as u32) > 127) {
            let message = format!("{location} contains a non-ASCII character; only lengths may carry foot and inch marks");
            self.refuse("MBF-IMPORT-CHARACTER-UNSUPPORTED", line, location, &message);
            return Ok(None);
        }
        if !match_real(text) {
            self.refuse("MBF-IMPORT-NUMBER-INVALID", line, location, &format!("{location} is not a real number"));
            return Ok(None);
        }
        let value: f64 = text.parse().map_err(|_| PyError::value(format!("could not convert string to float: {text:?}")))?;
        let refused = match parse_fraction(text)? {
            Parsed::Exact(exact) => {
                if value.is_finite() && !(!exact.is_zero() && value == 0.0) {
                    return Ok(Some(Num { value, exact, text: text.to_string() }));
                }
                true
            }
            Parsed::Huge => true,
        };
        if refused {
            let message = format!(
                "{location} is outside the binary64 range: it overflows, or a non-zero value underflows to 0"
            );
            self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, location, &message);
        }
        Ok(None)
    }

    /// `binary64(value, ...)` for an exact value.
    pub fn binary64_q(&mut self, value: &Q, line: usize, location: &str, what: &str, nonzero: bool) -> R<f64> {
        let result = value.to_f64().unwrap_or(f64::INFINITY);
        self.binary64_check(result, !value.is_zero(), line, location, what, nonzero)
    }

    /// `binary64(value, ...)` for a float.
    pub fn binary64_f(&mut self, value: f64, line: usize, location: &str, what: &str, nonzero: bool) -> R<f64> {
        self.binary64_check(value, value != 0.0, line, location, what, nonzero)
    }

    fn binary64_check(&mut self, result: f64, nonzero_value: bool, line: usize, location: &str, what: &str, nonzero: bool) -> R<f64> {
        if !result.is_finite() || ((nonzero_value || nonzero) && result == 0.0) {
            self.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, location, &format!("the derived {what} is outside the binary64 range"));
            return Err(Stop::Refused);
        }
        Ok(result)
    }

    pub fn is_zero(&mut self, text: &str, line: usize, location: &str) -> R<Option<bool>> {
        if text.is_empty() {
            return Ok(Some(true));
        }
        Ok(self.real(text, line, location)?.map(|number| number.exact.is_zero()))
    }

    // -- VERSION, OPTIONS ------------------------------------------------
    fn read_version(&mut self) -> R<()> {
        let Some((header, _)) = self.sections.get(&"VERSION".to_string()).cloned() else {
            self.refuse("MBF-IMPORT-VERSION-MISSING", 0, "VERSION", "the MBF has no VERSION section; the 6.xx variant without it is not read");
            return Err(Stop::Refused);
        };
        let lines = self.data_lines("VERSION");
        if lines.is_empty() {
            self.refuse("MBF-IMPORT-VERSION-MISSING", header, "VERSION", "the VERSION section gives no version");
            return Err(Stop::Refused);
        }
        if lines.len() > 1 {
            self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1].0, "VERSION", "VERSION documents one line");
        }
        let (line, text) = lines[0].clone();
        if !ACCEPTED_VERSIONS.contains(&text.as_str()) {
            let message = format!(
                "MBF version {} is not read; field layouts depend on the version (TBD-17-01-001) and this importer reads {}",
                py_repr_str(&text),
                ACCEPTED_VERSIONS.join(", ")
            );
            self.refuse("MBF-IMPORT-VERSION-UNSUPPORTED", line, "VERSION", &message);
            return Err(Stop::Refused);
        }
        self.version = Some(text);
        Ok(())
    }

    fn read_options(&mut self) -> R<()> {
        let mut hanger: Vec<Value> = Vec::new();
        let mut code: Vec<Value> = Vec::new();
        for (line, text) in self.data_lines("OPTIONS") {
            for token in self.fields(line, &text, "OPTIONS").unwrap_or_default() {
                match token.as_str() {
                    "SI" => self.si = true,
                    "RAD" => self.rad = true,
                    "Z" => self.z_vertical = true,
                    _ if HANGER_OPTIONS.contains(&token.as_str()) => hanger.push(Value::Str(token.clone())),
                    _ if PIPING_CODE_OPTIONS.contains(&token.as_str()) => code.push(Value::Str(token.clone())),
                    _ => {
                        let message = format!("OPTIONS token {} is not a documented option", py_repr_str(&token));
                        self.refuse("MBF-IMPORT-OPTION-UNSUPPORTED", line, "OPTIONS", &message);
                    }
                }
            }
        }
        for (key, items) in [("hanger_design", hanger), ("piping_code", code)] {
            if !items.is_empty() {
                self.target_options.insert(format!("OPTIONS.{key}"), Value::List(items));
                let header = self.sections.get(&"OPTIONS".to_string()).map(|(line, _)| *line).unwrap_or(0);
                let message = format!(
                    "OPTIONS {} is pass-through target configuration only; it does not become SWBPIPE rule or \
                     code-checking logic",
                    key.replace('_', " ")
                );
                self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", header, &format!("OPTIONS {key}"), &message);
            }
        }
        Ok(())
    }

    // -- sidecar (§15) -----------------------------------------------------
    fn bind_sidecar(&mut self) -> R<()> {
        let Some(given) = &self.options.sidecar else {
            if self.options.rebind_by_node_number {
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, "sidecar", "rebinding was requested without a sidecar");
            }
            return Ok(());
        };
        let sidecar = match given {
            SidecarInput::Parsed(value) => value.clone(),
            SidecarInput::Json(bytes) => match parse_sidecar_json(bytes) {
                Ok(value) => value,
                Err(SidecarJsonError::Invalid(findings)) => {
                    for (path, reason) in findings {
                        self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, &path, &format!("the sidecar JSON at {path} {reason}"));
                    }
                    return Ok(());
                }
                Err(SidecarJsonError::LoneSurrogate) => {
                    return Err(Stop::Fail(ImportError::Unsupported(
                        "the sidecar JSON escapes a lone surrogate (\\uD800-\\uDFFF without its pair), which Python holds \
                         in a str and this port does not represent"
                            .into(),
                    )))
                }
                Err(SidecarJsonError::TooDeep) => {
                    return Err(Stop::Fail(ImportError::Unsupported(format!(
                        "the sidecar JSON nests deeper than {}, where CPython's limit is not reproduced",
                        crate::pyjson::MAX_NESTING
                    ))))
                }
            },
        };
        self.sidecar = Some(sidecar.clone());
        if !self.self_digest(&sidecar)? {
            return Ok(());
        }
        let mut parser = SidecarParser::new();
        let parsed = parser.parse(&sidecar)?;
        let Some(mut parsed) = parsed else {
            for (path, reason) in parser.errors.clone() {
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, &path, &format!("the sidecar's {path} {reason}"));
            }
            return Ok(());
        };
        if !parser.format_number.is_some_and(|number| DIGESTED_FORMATS.contains(&number)) {
            self.self_digest_state = Some("absent");
            let head = if parser.current {
                "the sidecar is format 2, written before the self-digest"
            } else {
                "the sidecar is legacy, from main's tranche-1 writer"
            };
            let message = format!(
                "{head}, and carries no self-digest: an accidental or partial edit of it is caught only where the \
                 agreement rule, the parser or the regeneration identity reads the edited value"
            );
            self.info("MBF-IMPORT-SIDECAR-NO-SELF-DIGEST", 0, if parser.current { "sidecar_format" } else { "sidecar" }, &message);
        }
        let digest = parsed.as_dict_mut().and_then(|dict| dict.remove(&"mbf_sha256".to_string())).unwrap_or(Value::Null);
        self.maps = parsed;
        self.sidecar_current = parser.current;
        self.sidecar_format = parser.format_number;
        if self.options.rebind_by_node_number {
            self.rebound = true;
        } else if digest.as_str() == Some(self.sha.as_str()) {
            self.bound = true;
            if !parser.current && (!self.si || self.z_vertical) {
                let message = format!(
                    "the sidecar is legacy, from main's tranche-1 writer, which wrote only SI files with Y vertical; \
                     this file is {}",
                    if self.z_vertical { "Z-vertical" } else { "English" }
                );
                self.refuse("MBF-IMPORT-SIDECAR-CONFLICT", 0, "sidecar_format", &message);
            }
        } else {
            self.info(
                "MBF-IMPORT-SIDECAR-NOT-APPLIED",
                0,
                "sidecar",
                "the sidecar's mbf_sha256 does not equal the input's, so the import is unbound and the sidecar is not \
                 applied (no rebinding requested)",
            );
            self.maps = Value::Dict(Dict::new());
            return Ok(());
        }
        let mut node_ids = OMap::new();
        if let Some(nodes) = self.maps.get("node_map").and_then(Value::as_dict) {
            for (swb_id, name) in nodes.iter() {
                node_ids.insert(name.as_str().unwrap_or_default().to_string(), swb_id.clone());
            }
        }
        self.node_ids = node_ids;
        self.ambiguous();
        Ok(())
    }

    fn self_digest(&mut self, sidecar: &Value) -> R<bool> {
        let claimed = match sidecar.get("sidecar_format") {
            Some(Value::Int(number)) => crate::value::small_int(&Value::Int(number.clone())).filter(|number| DIGESTED_FORMATS.contains(number)),
            _ => None,
        };
        let (true, Some(claimed)) = (sidecar.as_dict().is_some(), claimed) else {
            return Ok(true);
        };
        let path = SIDECAR_DIGEST_KEY;
        let Some(given) = sidecar.get(path) else {
            let message = format!(
                "the sidecar's {path} is missing: a sidecar of format {claimed} carries its self-digest, and one \
                 without it is not read as an older format"
            );
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path, &message);
            return Ok(false);
        };
        let given = given.clone();
        if !matches!(&given, Value::Str(text) if match_sha256(text)) {
            self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, path, &format!("the sidecar's {path} must be a lowercase SHA-256 hex digest"));
            return Ok(false);
        }
        let actual = match sidecar_self_digest(sidecar) {
            Ok(actual) => actual,
            Err(Ok(error)) => {
                let message = format!(
                    "the sidecar's {} {}, so the sidecar has no canonical form for its self-digest",
                    error.path, error.reason
                );
                self.refuse("MBF-IMPORT-SIDECAR-INVALID", 0, &error.path, &message);
                return Ok(false);
            }
            Err(Err(error)) => return Err(error.into()),
        };
        if Some(actual.as_str()) != given.as_str() {
            let message = format!(
                "the sidecar's {path} is not the SHA-256 of its canonical content ({actual}): the sidecar was changed \
                 after it was written, or only part of it was kept. It is not applied on any route; restore the \
                 written sidecar"
            );
            self.refuse("MBF-IMPORT-SIDECAR-DIGEST-MISMATCH", 0, path, &message);
            return Ok(false);
        }
        self.self_digest_state = Some("verified");
        Ok(true)
    }

    fn ambiguous(&mut self) {
        let mut seen: Vec<((String, String), String)> = Vec::new();
        let elements: Vec<(String, Value)> = self
            .maps
            .get("element_map")
            .and_then(Value::as_dict)
            .map(|map| map.iter().map(|(key, value)| (key.clone(), value.clone())).collect())
            .unwrap_or_default();
        for (name, entry) in elements {
            let a = entry.get_or_null("from").as_str().unwrap_or_default().to_string();
            let b = entry.get_or_null("to").as_str().unwrap_or_default().to_string();
            let key = if a <= b { (a, b) } else { (b, a) };
            if let Some((_, first)) = seen.iter().find(|(existing, _)| *existing == key) {
                let message = format!(
                    "element_map[{name}] names the pair of CAEPIPE nodes that element_map[{first}] names, so the import \
                     cannot tell which entry is the file's"
                );
                self.refuse("MBF-IMPORT-SIDECAR-AMBIGUOUS", 0, &format!("element_map[{name}]"), &message);
            } else {
                seen.push((key, name));
            }
        }
        let mut labels: Vec<(String, String)> = Vec::new();
        let cases: Vec<(String, String)> = self
            .maps
            .get("load_case_map")
            .and_then(Value::as_dict)
            .map(|map| map.iter().map(|(key, value)| (key.clone(), value.as_str().unwrap_or_default().to_string())).collect())
            .unwrap_or_default();
        for (name, label) in cases {
            if let Some((_, first)) = labels.iter().find(|(existing, _)| *existing == label) {
                let message = format!(
                    "load_case_map[{name}] names the case label that load_case_map[{first}] names, so the import cannot \
                     tell which entry is the file's"
                );
                self.refuse("MBF-IMPORT-SIDECAR-AMBIGUOUS", 0, &format!("load_case_map[{name}]"), &message);
            } else {
                labels.push((label, name));
            }
        }
    }

    pub fn pd1(&self) -> bool {
        !self.options.pd1_disabled && self.version.as_deref().map(|version| PD1_VERSIONS.contains(&version)).unwrap_or(false)
    }

    pub fn convention(&mut self, what: &str, line: usize, location: &str) -> bool {
        if self.bound {
            self.add_convention("bound", what);
            return true;
        }
        if self.pd1() {
            self.add_convention("PD-1", what);
            return true;
        }
        let message = format!(
            "{what} rests on TBD-17-04-007 (CQ-17-01-013) and is refused in an unbound import of MBF version {}",
            self.version.clone().unwrap_or_else(|| "None".into())
        );
        self.refuse("MBF-IMPORT-TBD-17-04-007", line, location, &message);
        false
    }

    pub fn add_convention(&mut self, basis: &str, what: &str) {
        let item = (basis.to_string(), what.to_string());
        if !self.used_conventions.contains(&item) {
            self.used_conventions.push(item);
        }
    }

    // -- §6 analysis options -------------------------------------------------
    fn read_analysis_options(&mut self) -> R<()> {
        if self.sections.contains_key(&"OPTEMP".to_string()) {
            let lines = self.data_lines("OPTEMP");
            if lines.len() > 1 {
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1].0, "OPTEMP", "OPTEMP documents one line");
            }
            if let Some((line, text)) = lines.first().cloned() {
                let fields = self.fields(line, &text, "OPTEMP").unwrap_or_default();
                if !self.too_many(&fields, FIELD_OPTEMP, line, "OPTEMP") {
                    let first = fields.first().ok_or_else(index_error)?.clone();
                    self.t_ref = self.real(&first, line, "OPTEMP field 1 (reference temperature)")?;
                    if fields.len() > 2 && !fields[2].is_empty() {
                        let count = self.real(&fields[2].clone(), line, "OPTEMP field 3 (number of thermal loads)")?;
                        self.thermal_load_count = count.as_ref().map(|number| number.exact.clone());
                        if let Some(count) = &count {
                            if count.exact != Q::from_int(1) {
                                self.refuse(
                                    "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED",
                                    line,
                                    "OPTEMP field 3",
                                    "more than one thermal load is not read in tranche 1",
                                );
                            }
                        }
                    }
                    let delegated = fields.iter().enumerate().any(|(index, value)| matches!(index, 1 | 3 | 4) && !value.is_empty());
                    let rest: Vec<Value> = fields.iter().skip(1).map(|item| Value::Str(item.clone())).collect();
                    self.target_options.insert("OPTEMP".into(), dict![("fields", Value::List(rest))]);
                    if delegated {
                        self.listed(
                            "delegated",
                            "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                            line,
                            "OPTEMP",
                            "OPTEMP thermal cycles, solve-thermal and cold-modulus flags are pass-through target configuration",
                        );
                    }
                }
            }
        }
        for name in ["OPCODE", "OPPRES", "OPMISC"] {
            let Some((header, entries)) = self.sections.get(&name.to_string()).cloned() else { continue };
            if entries.first().map(|(_, text)| text.is_empty()).unwrap_or(false) && matches!(name, "OPPRES" | "OPMISC") {
                if name == "OPPRES" {
                    self.oppres = Some((header, OPPRES_DEFAULTS.iter().map(|item| item.to_string()).collect(), true));
                }
                self.target_options.insert(name.into(), Value::str("section present with CAEPIPE defaults"));
                self.listed(
                    "delegated",
                    "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                    header,
                    name,
                    &format!("{name} is present with a blank line (CAEPIPE defaults); pass-through only"),
                );
                let data = self.data_lines(name);
                if let Some((line, _)) = data.first() {
                    self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", *line, name, &format!("{name} gives data after the blank defaults line"));
                }
                continue;
            }
            let lines = self.data_lines(name);
            if lines.len() > 1 {
                self.refuse("MBF-IMPORT-FIELD-UNDOCUMENTED", lines[1].0, name, &format!("{name} documents one line"));
            }
            let Some((line, text)) = lines.first().cloned() else { continue };
            let fields = self.fields(line, &text, name).unwrap_or_default();
            if name == "OPMISC" {
                self.read_opmisc(line, &fields)?;
                continue;
            }
            if self.too_many(&fields, field_count(name), line, name) {
                continue;
            }
            if name == "OPPRES" {
                self.oppres = Some((line, fields.clone(), false));
            }
            self.target_options.insert(name.into(), dict![("fields", Value::List(fields.iter().map(|item| Value::Str(item.clone())).collect()))]);
            self.listed(
                "delegated",
                "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                line,
                name,
                &format!("{name} configures CAEPIPE's own analysis; pass-through target configuration only"),
            );
        }
        Ok(())
    }

    fn read_opmisc(&mut self, line: usize, fields: &[String]) -> R<()> {
        if fields.len() != OPMISC_EXPORT_FIELDS && self.too_many(fields, FIELD_OPMISC, line, "OPMISC") {
            return Ok(());
        }
        let head: Vec<Value> = fields.iter().take(8).map(|item| Value::Str(item.clone())).collect();
        self.target_options.insert("OPMISC".into(), dict![("fields", Value::List(head))]);
        self.listed(
            "delegated",
            "MBF-IMPORT-TARGET-OPTION-DELEGATED",
            line,
            "OPMISC fields 1-8",
            "OPMISC dynamics and hanger options configure CAEPIPE's own analysis; pass-through only",
        );
        let (acceleration, direction): (String, Vec<String>) = if fields.len() == OPMISC_EXPORT_FIELDS {
            if self.is_zero(&fields[8], line, "OPMISC field 9")? != Some(true) {
                self.refuse(
                    "MBF-IMPORT-FIELD-UNDOCUMENTED",
                    line,
                    "OPMISC field 9",
                    "OPMISC field 9 has no public description; only the 0 the export writes is read",
                );
                return Ok(());
            }
            self.listed(
                "delegated",
                "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                line,
                "OPMISC field 9",
                "OPMISC field 9 (no public description) is read only as 0 and passes through",
            );
            (fields[9].clone(), fields[10..13].to_vec())
        } else {
            if fields.len() == 10 && !fields[9].is_empty() {
                self.refuse(
                    "MBF-IMPORT-TBD-17-04-005",
                    line,
                    "OPMISC field 10 (gravitational direction)",
                    "a gravitational direction in one field has no documented encoding (TBD-17-04-005, CQ-17-01-011); \
                     weight direction is a mechanics input and is not assumed",
                );
                return Ok(());
            }
            (if fields.len() > 8 { fields[8].clone() } else { String::new() }, Vec::new())
        };
        if acceleration.is_empty() && !direction.iter().any(|item| !item.is_empty()) {
            return Ok(());
        }
        let g = self.real(&acceleration, line, "OPMISC gravitational acceleration")?;
        let mut components = Vec::new();
        for value in &direction {
            components.push(self.real(value, line, "OPMISC gravitational direction")?);
        }
        let Some(g) = g else { return Ok(()) };
        if components.iter().any(Option::is_none) {
            return Ok(());
        }
        let components: Vec<Num> = components.into_iter().map(Option::unwrap).collect();
        let earth = if fields.len() == OPMISC_EXPORT_FIELDS {
            let down = if self.z_vertical { [0, 0, -1] } else { [0, -1, 0] };
            let gravity = Q::new(3864.into(), 10.into());
            g.exact == gravity
                && components.len() == 3
                && components.iter().zip(down.iter()).all(|(component, &axis)| component.exact == Q::from_int(axis))
        } else {
            let standard = if self.si {
                Q::new(980665.into(), 100000.into())
            } else {
                Q::new(980665.into(), 100000.into()).div(&Q::new(254.into(), 10000.into()))
            };
            standard_gravity_as_printed(&g.text, &standard)?
        };
        if !earth {
            self.refuse(
                "MBF-IMPORT-GRAVITY-UNSUPPORTED",
                line,
                "OPMISC gravitational acceleration and direction",
                "only earth gravity is read (the export's default setting, or standard gravity in the documented \
                 layout's unit); another acceleration or direction scales or turns the weights, and SWBPIPE has no \
                 gravity setting (TBD-17-04-005)",
            );
            return Ok(());
        }
        let message = format!(
            "OPMISC gravitational acceleration and direction are the default earth setting; SWBPIPE has no gravity \
             setting and carries weight as explicit primitives along -Y{}",
            if self.z_vertical { " (the file's -Z, rotated)" } else { "" }
        );
        self.listed("delegated", "MBF-IMPORT-TARGET-OPTION-DELEGATED", line, "OPMISC gravity", &message);
        self.add_convention("gravity", "the earth gravity setting");
        Ok(())
    }

    // -- §7 MATERIAL -----------------------------------------------------------
    fn read_materials(&mut self) -> R<()> {
        let mut order: Vec<String> = Vec::new();
        let entries = self.sections.get(&"MATERIAL".to_string()).map(|(_, entries)| entries.clone()).unwrap_or_default();
        for (line, text) in entries {
            if text.is_empty() {
                continue;
            }
            let Some(fields) = self.fields(line, &text, "MATERIAL") else { continue };
            let name = fields[0].clone();
            if name.is_empty() {
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "MATERIAL field 1 (name)", "the material name is empty");
                continue;
            }
            if order.last() != Some(&name) {
                if self.materials.contains_key(&name) {
                    self.refuse("MBF-IMPORT-RECORD-REPEATED", line, &format!("MATERIAL {name}"), &format!("material {name} is defined a second time"));
                    continue;
                }
                order.push(name.clone());
                self.material_first(&name, fields, line)?;
            } else {
                self.material_temperature(&name, fields, line)?;
            }
        }
        for name in order {
            let Some(record) = self.materials.get(&name).cloned() else { continue };
            if record.refused {
                continue;
            }
            if record.rows.is_empty() {
                self.refuse(
                    "MBF-IMPORT-FIELD-MISSING",
                    record.line,
                    &format!("MATERIAL {name}"),
                    &format!("material {name} has no temperature line with E and the expansion coefficient"),
                );
                self.materials.get_mut(&name).expect("material").refused = true;
            } else if record.rows.iter().any(|row| (row.0.value, row.1.value) != (record.rows[0].0.value, record.rows[0].1.value)) {
                self.refuse(
                    "MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT",
                    record.line,
                    &format!("MATERIAL {name}"),
                    &format!(
                        "material {name} varies E or the expansion coefficient with temperature; temperature-dependent \
                         materials are tranche 2"
                    ),
                );
                self.materials.get_mut(&name).expect("material").refused = true;
            }
        }
        Ok(())
    }

    fn material_first(&mut self, name: &str, fields: Vec<String>, line: usize) -> R<()> {
        self.materials.insert(name.to_string(), MaterialRec { line, ..Default::default() });
        let location = format!("MATERIAL {name}");
        for (index, value) in fields.iter().enumerate() {
            if index != 6 && !self.check_characters(line, value, &[]) {
                self.materials.get_mut(&name.to_string()).expect("material").refused = true;
                return Ok(());
            }
        }
        if fields.len() >= 6 && MATERIAL_FRP_TYPES.contains(&fields[5].as_str()) {
            self.refuse("MBF-IMPORT-MATERIAL-FRP-UNSUPPORTED", line, &location, &format!("material {name} has an FRP type, whose line layout is not read"));
            self.materials.get_mut(&name.to_string()).expect("material").refused = true;
            return Ok(());
        }
        if self.too_many(&fields, FIELD_MATERIAL_FIRST, line, &location) {
            self.materials.get_mut(&name.to_string()).expect("material").refused = true;
            return Ok(());
        }
        let mut fields = fields;
        fields.resize(FIELD_MATERIAL_FIRST, String::new());
        for (position, column) in MATERIAL_FIRST_PROTECTED {
            if self.protected(&fields[position - 1].clone(), line, &location, column, position)? {
                self.materials.get_mut(&name.to_string()).expect("material").refused = true;
                return Ok(());
            }
        }
        let mut density = self.real(&fields[1].clone(), line, &format!("{location} field 2 (weight density)"))?;
        let mut nu = self.real(&fields[2].clone(), line, &format!("{location} field 3 (Poisson ratio)"))?;
        if density.as_ref().map(|value| value.exact.is_negative()).unwrap_or(false) {
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &format!("{location} field 2"), "a weight density must not be negative");
            density = None;
        }
        if let Some(value) = &nu {
            if !(0.0 < value.value && value.value < 0.5) {
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &format!("{location} field 3"), "the Poisson ratio must lie in (0, 0.5)");
                nu = None;
            }
        }
        let (Some(density), Some(nu)) = (density, nu) else {
            self.materials.get_mut(&name.to_string()).expect("material").refused = true;
            return Ok(());
        };
        let record = self.materials.get_mut(&name.to_string()).expect("material");
        record.density = Some(density);
        record.nu = Some(nu);
        record.description = fields[6].clone();
        if fields[3..6].iter().any(|value| !value.is_empty()) {
            self.listed(
                "delegated",
                "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                line,
                &location,
                &format!("material {name} joint factors and type are code-check settings; not imported"),
            );
        }
        Ok(())
    }

    fn protected(&mut self, value: &str, line: usize, location: &str, column: &str, position: usize) -> R<bool> {
        if value.is_empty() {
            return Ok(false);
        }
        if match_real(value) && fraction_is_zero(value)? {
            return Ok(false);
        }
        let message = format!(
            "{location} carries a value in its {column} column; strength, allowable and fatigue data never enter the \
             model and mark the material as suspected library or protected data"
        );
        self.refuse("MBF-IMPORT-MATERIAL-PROTECTED-SUSPECTED", line, &format!("{location} field {position} ({column})"), &message);
        Ok(true)
    }

    fn material_temperature(&mut self, name: &str, fields: Vec<String>, line: usize) -> R<()> {
        if self.materials.get(&name.to_string()).map(|record| record.refused).unwrap_or(false) {
            return Ok(());
        }
        let location = format!("MATERIAL {name} line {line}");
        for value in &fields {
            if !self.check_characters(line, value, &[]) {
                self.materials.get_mut(&name.to_string()).expect("material").refused = true;
                return Ok(());
            }
        }
        if self.too_many(&fields, FIELD_MATERIAL_TEMPERATURE, line, &location) {
            self.materials.get_mut(&name.to_string()).expect("material").refused = true;
            return Ok(());
        }
        let mut fields = fields;
        fields.resize(FIELD_MATERIAL_TEMPERATURE, String::new());
        for (position, column) in MATERIAL_TEMPERATURE_PROTECTED {
            if self.protected(&fields[position - 1].clone(), line, &location, column, position)? {
                self.materials.get_mut(&name.to_string()).expect("material").refused = true;
                return Ok(());
            }
        }
        let temperature = self.real(&fields[1].clone(), line, &format!("{location} field 2 (temperature)"))?;
        let mut modulus = self.real(&fields[2].clone(), line, &format!("{location} field 3 (E)"))?;
        let alpha = self.real(&fields[3].clone(), line, &format!("{location} field 4 (expansion coefficient)"))?;
        if let Some(value) = &modulus {
            if !(value.value > 0.0) {
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &format!("{location} field 3"), "E must be positive");
                modulus = None;
            }
        }
        let (Some(temperature), Some(modulus), Some(alpha)) = (temperature, modulus, alpha) else {
            self.materials.get_mut(&name.to_string()).expect("material").refused = true;
            return Ok(());
        };
        self.materials.get_mut(&name.to_string()).expect("material").rows.push((modulus, alpha, temperature));
        Ok(())
    }

    // -- §8 PIPE -------------------------------------------------------------
    fn read_pipes(&mut self) -> R<()> {
        for (line, text) in self.data_lines("PIPE") {
            let Some(fields) = self.fields(line, &text, "PIPE") else { continue };
            let name = fields[0].clone();
            let location = format!("PIPE {name}");
            if name.is_empty() {
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "PIPE field 1 (name)", "the section name is empty");
                continue;
            }
            if self.pipes.contains_key(&name) {
                self.refuse("MBF-IMPORT-RECORD-REPEATED", line, &location, &format!("section {name} is defined a second time"));
                continue;
            }
            self.pipes.insert(name.clone(), PipeRec { line, refused: true, ..Default::default() });
            if self.too_many(&fields, FIELD_PIPE, line, &location) {
                continue;
            }
            let allowance_given = fields.len() > 3 && !fields[3].is_empty();
            let mut fields = fields;
            fields.resize(FIELD_PIPE, String::new());
            let od = self.real(&fields[1].clone(), line, &format!("{location} field 2 (OD)"))?;
            let thk = self.real(&fields[2].clone(), line, &format!("{location} field 3 (thickness)"))?;
            let mut ok = od.is_some() && thk.is_some();
            if let (Some(od), Some(thk)) = (&od, &thk) {
                if !(0.0 < thk.value && thk.exact.mul_int(2) < od.exact) {
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &location, &format!("section {name} needs 0 < thickness < OD/2"));
                    ok = false;
                }
            }
            for (position, column) in PIPE_ZERO_FIELDS {
                let zero = self.is_zero(&fields[position - 1].clone(), line, &format!("{location} field {position} ({column})"))?;
                if zero == Some(false) {
                    self.refuse(
                        "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED",
                        line,
                        &format!("{location} field {position}"),
                        &format!("section {name} has a non-zero {column}; tranche 1 reads bare pipe only"),
                    );
                }
                ok = ok && zero == Some(true);
            }
            if !fields[9].is_empty() {
                self.refuse(
                    "MBF-IMPORT-SECTION-FIELD-UNSUPPORTED",
                    line,
                    &format!("{location} field 10"),
                    &format!("section {name} names a soil; buried pipe is tranche 2"),
                );
                ok = false;
            }
            if ok {
                let record = self.pipes.get_mut(&name).expect("pipe");
                record.refused = false;
                record.od = od;
                record.thk = thk;
                record.allowance_given = allowance_given;
            }
        }
        Ok(())
    }

    // -- §9 LOADS ------------------------------------------------------------
    fn read_loads(&mut self) -> R<()> {
        let tail_names = loads_tail();
        for (line, text) in self.data_lines("LOADS") {
            let Some(fields) = self.fields(line, &text, "LOADS") else { continue };
            let name = fields[0].clone();
            let location = format!("LOADS {name}");
            if name.is_empty() {
                self.refuse("MBF-IMPORT-FIELD-MISSING", line, "LOADS field 1 (name)", "the load name is empty");
                continue;
            }
            if self.loads.contains_key(&name) {
                self.refuse("MBF-IMPORT-RECORD-REPEATED", line, &location, &format!("load {name} is defined a second time"));
                continue;
            }
            self.loads.insert(name.clone(), LoadRec { line, refused: true, ..Default::default() });
            if self.too_many(&fields, FIELD_LOADS, line, &location) {
                continue;
            }
            let mut fields = fields;
            fields.resize(FIELD_LOADS, String::new());
            let t1 = self.real(&fields[1].clone(), line, &format!("{location} field 2 (temperature 1)"))?;
            let p1 = self.real(&fields[2].clone(), line, &format!("{location} field 3 (pressure 1)"))?;
            let sg = self.real(&fields[3].clone(), line, &format!("{location} field 4 (specific gravity)"))?;
            let mut ok = t1.is_some() && p1.is_some() && sg.is_some();
            if p1.as_ref().map(|value| value.exact.is_negative()).unwrap_or(false) {
                self.refuse(
                    "MBF-IMPORT-PRESSURE-UNSUPPORTED",
                    line,
                    &format!("{location} field 3"),
                    &format!(
                        "load {name} has a negative pressure 1; the manual gives no sign convention for external \
                         pressure, so it is not read"
                    ),
                );
                ok = false;
            }
            if sg.as_ref().map(|value| !value.exact.is_zero()).unwrap_or(false) {
                self.refuse(
                    "MBF-IMPORT-CONTENTS-UNSUPPORTED",
                    line,
                    &format!("{location} field 4"),
                    &format!("load {name} has a non-zero specific gravity; contents are tranche 2"),
                );
                ok = false;
            }
            let mut tail: OMap<String, String> = OMap::new();
            for (key, value) in tail_names.iter().zip(fields[4..].iter()) {
                tail.insert(key.clone(), value.clone());
            }
            let get = |tail: &OMap<String, String>, key: &str| tail.get(&key.to_string()).cloned().unwrap_or_default();
            for n in [2usize, 3, 4, 5, 6, 7, 8, 9, 10] {
                let t = get(&tail, &format!("T{n}"));
                let p = get(&tail, &format!("P{n}"));
                let beyond = match &self.thermal_load_count {
                    Some(count) => Q::from_int(n as i64) > *count,
                    None => false,
                };
                let good = if beyond {
                    self.inactive_pair(&name, &t, &p, n, line, &location)?
                } else {
                    self.thermal_pair(&name, &t, &p, n, line, &location)?
                };
                ok = good && ok;
            }
            for label in ["additional weight", "Wind 1", "Wind 2", "Wind 3", "Wind 4", "Snow", "Ice", "Wave", "Wave 2", "Wave 3", "Wave 4"] {
                let zero = self.is_zero(&get(&tail, label), line, &format!("{location} {label}"))?;
                if zero == Some(false) {
                    self.refuse(
                        "MBF-IMPORT-LOAD-FIELD-UNSUPPORTED",
                        line,
                        &format!("{location} {label}"),
                        &format!("load {name} sets {label}; tranche 1 reads it only as zero or empty"),
                    );
                }
                ok = ok && zero == Some(true);
            }
            if !get(&tail, "peak temperature").is_empty() || !get(&tail, "peak pressure").is_empty() {
                self.listed(
                    "delegated",
                    "MBF-IMPORT-TARGET-OPTION-DELEGATED",
                    line,
                    &format!("{location} peak values"),
                    &format!("load {name} peak temperature and pressure only feed CAEPIPE's peak case"),
                );
            }
            if ok {
                let record = self.loads.get_mut(&name).expect("load");
                record.refused = false;
                record.t1 = t1;
                record.p1 = p1;
            }
        }
        Ok(())
    }

    fn thermal_pair(&mut self, name: &str, t_text: &str, p_text: &str, n: usize, line: usize, location: &str) -> R<bool> {
        let Some(p_zero) = self.is_zero(p_text, line, &format!("{location} P{n}"))? else { return Ok(false) };
        if t_text.is_empty() && p_zero {
            return Ok(true);
        }
        let Some(temperature) = self.real(t_text, line, &format!("{location} T{n}"))? else { return Ok(false) };
        if let Some(t_ref) = &self.t_ref {
            if temperature.exact == t_ref.exact && p_zero {
                self.slot_inactive(name, n, line, location, &format!("T{n} equals the reference temperature and P{n} is 0"));
                return Ok(true);
            }
        }
        self.refuse(
            "MBF-IMPORT-THERMAL-LOADS-UNSUPPORTED",
            line,
            &format!("{location} T{n}"),
            &format!("load {name} defines thermal load {n}; multiple thermal cases are tranche 2"),
        );
        Ok(false)
    }

    fn inactive_pair(&mut self, name: &str, t_text: &str, p_text: &str, n: usize, line: usize, location: &str) -> R<bool> {
        if !t_text.is_empty() && self.real(t_text, line, &format!("{location} T{n}"))?.is_none() {
            return Ok(false);
        }
        if !p_text.is_empty() && self.real(p_text, line, &format!("{location} P{n}"))?.is_none() {
            return Ok(false);
        }
        if !t_text.is_empty() || !p_text.is_empty() {
            self.slot_inactive(name, n, line, location, "OPTEMP gives 1 thermal load, so no load case uses it");
        }
        Ok(true)
    }

    fn slot_inactive(&mut self, name: &str, n: usize, line: usize, location: &str, reason: &str) {
        let message = format!("load {name}: thermal load {n} (T{n}, P{n}) is inactive and dropped: {reason}");
        self.info("MBF-IMPORT-LOAD-SLOT-INACTIVE", line, &format!("{location} T{n}/P{n}"), &message);
    }

    // -- §14 other sections -------------------------------------------------
    fn read_other_sections(&mut self) -> R<()> {
        let sections: Vec<(String, (usize, Vec<(usize, String)>))> =
            self.sections.iter().map(|(name, value)| (name.clone(), value.clone())).collect();
        for (name, (header, entries)) in sections {
            match section_kind(&name).expect("section") {
                "refused" => self.refuse("MBF-IMPORT-SECTION-UNSUPPORTED", header, &name, &format!("section {name} is not read in tranche 1")),
                "delegated" => {
                    let count = entries.iter().filter(|(_, text)| !text.is_empty()).count();
                    self.delegated_sections.insert(name.clone(), Value::int(count as i64));
                    let message = format!(
                        "section {name} ({count} records) configures CAEPIPE's own checks or dynamics; its values are not copied"
                    );
                    self.listed("delegated", "MBF-IMPORT-SECTION-DELEGATED", header, &name, &message);
                }
                "zero" => {
                    let mut ok = true;
                    for (_, text) in &entries {
                        if text.is_empty() {
                            continue;
                        }
                        let values = split_fields(text).unwrap_or_else(|| vec!["?".into()]);
                        for value in values {
                            if !value.is_empty() && !(match_real(&value) && fraction_is_zero(&value)?) {
                                ok = false;
                            }
                        }
                    }
                    if ok {
                        self.listed("omitted", "MBF-IMPORT-SECTION-ZERO-OMITTED", header, &name, &format!("section {name} gives only zero load values and is omitted"));
                    } else {
                        self.refuse(
                            "MBF-IMPORT-SECTION-UNSUPPORTED",
                            header,
                            &name,
                            &format!("section {name} gives non-zero load values; static equivalents are tranche 2"),
                        );
                    }
                }
                _ => {}
            }
        }
        for required in ["MATERIAL", "PIPE", "LAYOUT"] {
            if !self.sections.contains_key(&required.to_string()) {
                self.refuse("MBF-IMPORT-SECTION-MISSING", 0, required, &format!("the MBF has no {required} section"));
            }
        }
        Ok(())
    }

    // -- §4.4 layout rows -------------------------------------------------------
    fn read_layout_rows(&mut self) -> R<Vec<LayoutEntry>> {
        let mut rows = Vec::new();
        let entries: Vec<(usize, String)> = self
            .sections
            .get(&"LAYOUT".to_string())
            .map(|(_, entries)| entries.iter().filter(|(_, text)| !text.is_empty()).cloned().collect())
            .unwrap_or_default();
        let mut index = 0;
        while index < entries.len() {
            let (line, text) = entries[index].clone();
            index += 1;
            if text.starts_with('*') {
                rows.push(LayoutEntry::Note(line, text));
                continue;
            }
            let Some(mut fields) = self.fields(line, &text, &format!("LAYOUT line {line}")) else { continue };
            let mut continued = fields.last().map(String::is_empty).unwrap_or(false) && fields.len() > 1;
            if continued {
                fields.pop();
            }
            let Some(mut row) = self.layout_row(line, &fields)? else { continue };
            if continued && !row.started {
                self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, &format!("LAYOUT line {line}"), "a row ends with a comma before any comment");
                continue;
            }
            while continued {
                if index >= entries.len() {
                    self.refuse(
                        "MBF-IMPORT-ROW-SYNTAX-INVALID",
                        line,
                        &format!("LAYOUT line {line}"),
                        "the last comment ends with a comma but no continuation line follows",
                    );
                    break;
                }
                let (next_line, next_text) = entries[index].clone();
                index += 1;
                let more = self.fields(next_line, &next_text, &format!("LAYOUT line {next_line}")).unwrap_or_default();
                continued = more.last().map(String::is_empty).unwrap_or(false) && more.len() > 1;
                let take = if continued { more.len() - 1 } else { more.len() };
                row.comments.extend(more.into_iter().take(take).map(|value| (next_line, value)));
            }
            rows.push(LayoutEntry::Row(row));
        }
        Ok(rows)
    }

    fn layout_row(&mut self, line: usize, fields: &[String]) -> R<Option<Row>> {
        let location = format!("LAYOUT line {line}");
        let first = fields[0].clone();
        let Some((kind, digits, marker)) = match_row_key(&first) else {
            let lead = first.chars().next();
            // `first[:1] in "FTL"`: the empty string is a substring of every string.
            let in_ftl = lead.map(|c| matches!(c, 'F' | 'T' | 'L')).unwrap_or(true);
            if lead == Some('H') {
                self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", line, &location, "row key H (hydrotest) is not read");
            } else if in_ftl && first.contains('*') {
                self.refuse(
                    "MBF-IMPORT-TBD-17-04-009",
                    line,
                    &location,
                    "the absolute-coordinate marker is read only trailing the node number (TBD-17-04-009, CQ-17-01-015); \
                     any other placement is refused",
                );
            } else if in_ftl {
                let message = format!(
                    "node {} is not a positive integer below 99999; generated suffixes (A, B, C, D, J) never appear on \
                     input rows",
                    py_repr_str(skip_chars(&first, 1))
                );
                self.refuse("MBF-IMPORT-NODE-NUMBER-INVALID", line, &location, &message);
            } else {
                self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", line, &location, "a layout row starts with F, T or L and a node number");
            }
            return Ok(None);
        };
        let number = crate::rational::py_int_digits(digits)?;
        let number = match num_traits::ToPrimitive::to_u32(&number) {
            Some(value) if value > 0 && value < 99999 => value,
            _ => {
                self.refuse("MBF-IMPORT-NODE-NUMBER-INVALID", line, &location, "a node number is a positive integer below 99999");
                return Ok(None);
            }
        };
        if marker && kind != 'T' {
            self.refuse(
                "MBF-IMPORT-ROW-KEY-UNSUPPORTED",
                line,
                &location,
                &format!("the absolute-coordinate marker is read on To rows only, not on a {kind} row"),
            );
            return Ok(None);
        }
        let mut row = Row { line, kind, number, absolute: marker, keys: OMap::new(), comments: Vec::new(), started: false };
        let mut started = false;
        for (offset, value) in fields[1..].iter().enumerate() {
            let position = offset + 2;
            if started {
                row.comments.push((line, value.clone()));
                continue;
            }
            let Some((letter, rest)) = split_first(value) else {
                self.refuse(
                    "MBF-IMPORT-ROW-SYNTAX-INVALID",
                    line,
                    &format!("{location} field {position}"),
                    "an empty key-letter field (a trailing comma continues comments only)",
                );
                return Ok(None);
            };
            if letter == 'C' {
                started = true;
                row.started = true;
                row.comments.push((line, rest.to_string()));
                continue;
            }
            if !"KMPJXYZBH".contains(letter) {
                let message = format!("key letter {} is not a documented layout key", py_repr_str(&letter.to_string()));
                self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", line, &format!("{location} field {position}"), &message);
                return Ok(None);
            }
            if row.keys.contains_key(&letter) {
                self.refuse(
                    "MBF-IMPORT-ROW-KEY-REPEATED",
                    line,
                    &format!("{location} field {position}"),
                    &format!("key letter {letter} appears twice in one row"),
                );
                return Ok(None);
            }
            row.keys.insert(letter, rest.to_string());
        }
        Ok(Some(row))
    }
}

/// `split_fields(text)`.
pub(crate) fn split_fields(text: &str) -> Option<Vec<String>> {
    let mut fields = Vec::new();
    let mut depth: i64 = 0;
    let mut current = String::new();
    for c in text.chars() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth < 0 {
                return None;
            }
        }
        if c == ',' && depth == 0 {
            fields.push(py_strip(&current).to_string());
            current.clear();
        } else {
            current.push(c);
        }
    }
    if depth != 0 {
        return None;
    }
    fields.push(py_strip(&current).to_string());
    Some(fields)
}

/// `half_unit(text)`.
pub(crate) fn half_unit(text: &str) -> R<Q> {
    let body = crate::pystr::lstrip_chars(text, &['+', '-']).to_lowercase();
    let (mantissa, exponent) = match body.find('e') {
        Some(at) => (body[..at].to_string(), body[at + 1..].to_string()),
        None => (body.clone(), String::new()),
    };
    let check = if mantissa.is_empty() { "0".to_string() } else { mantissa.clone() };
    if fraction_is_zero(&check)? {
        return Ok(Q::zero());
    }
    let decimals = match mantissa.find('.') {
        Some(at) => mantissa[at + 1..].chars().count() as i64,
        None => 0,
    };
    let exponent = if exponent.is_empty() { 0 } else { parse_signed_int(&exponent)? };
    Ok(Q::new(1.into(), 2.into()).mul(&Q::pow10(exponent - decimals)))
}

fn parse_signed_int(text: &str) -> R<i64> {
    let (negative, digits) = match text.chars().next() {
        Some('-') => (true, &text[1..]),
        Some('+') => (false, &text[1..]),
        _ => (false, text),
    };
    let value = crate::rational::py_int_digits(digits)?;
    let value = num_traits::ToPrimitive::to_i64(&value).ok_or_else(|| Stop::Fail(ImportError::Unsupported("an exponent beyond i64".into())))?;
    Ok(if negative { -value } else { value })
}

/// `standard_gravity_as_printed(text, standard)`.
fn standard_gravity_as_printed(text: &str, standard: &Q) -> R<bool> {
    let body = crate::pystr::lstrip_chars(text, &['+', '-']).to_lowercase();
    let mantissa = match body.find('e') {
        Some(at) => body[..at].to_string(),
        None => body.clone(),
    };
    let digits = mantissa.replace('.', "");
    let digits = digits.trim_start_matches('0');
    if digits.chars().count() < GRAVITY_MIN_DIGITS {
        return Ok(false);
    }
    let value = crate::rational::fraction_of(text)?;
    Ok(value.sub(standard).abs() <= half_unit(text)?)
}

/// `tenth_digit(value)`.
pub(crate) fn tenth_digit(value: &Q) -> R<Q> {
    if value.is_zero() {
        return Ok(Q::zero());
    }
    let size = value.abs();
    let float = size.to_f64()?;
    if float == 0.0 {
        return Err(PyError::value("math domain error").into());
    }
    let mut exponent = float.log10().floor() as i64;
    while Q::pow10(exponent) > size {
        exponent -= 1;
    }
    while Q::pow10(exponent + 1) <= size {
        exponent += 1;
    }
    Ok(Q::new(1.into(), 2.into()).mul(&Q::pow10(exponent - 9)))
}

/// `parse_english_length(text)`: exact inches, or None.
pub(crate) fn parse_english_length(text: &str) -> R<Option<Q>> {
    if let Some((sign, number)) = crate::pystr::match_feet_only(text) {
        let value = crate::rational::fraction_of(number)?.mul_int(12);
        return Ok(Some(if sign == "-" { value.neg() } else { value }));
    }
    let Some(parts) = crate::pystr::match_feet_inch(text) else { return Ok(None) };
    let inches = crate::rational::py_int_digits(parts.inches)?;
    if inches >= 12.into() {
        return Ok(None);
    }
    let feet = crate::rational::py_int_digits(parts.feet)?;
    let mut value = Q::from_bigint(feet * 12 + inches);
    if let Some((numerator, denominator)) = parts.fraction {
        // `int(denominator) == 0 or int(numerator) >= int(denominator)`, converted in that order
        let denominator = crate::rational::py_int_digits(denominator)?;
        if num_traits::Zero::is_zero(&denominator) {
            return Ok(None);
        }
        let numerator = crate::rational::py_int_digits(numerator)?;
        if numerator >= denominator {
            return Ok(None);
        }
        value = value.add(&Q::new(numerator, denominator));
    }
    Ok(Some(if parts.sign == "-" { value.neg() } else { value }))
}

/// `english_length_bound(text)`.
pub(crate) fn english_length_bound(text: &str) -> R<Q> {
    if let Some((_, number)) = crate::pystr::match_feet_only(text) {
        return Ok(half_unit(number)?.mul_int(12));
    }
    if let Some(parts) = crate::pystr::match_feet_inch(text) {
        if let Some((_, denominator)) = parts.fraction {
            let denominator = crate::rational::py_int_digits(denominator)?;
            return Ok(Q::new(1.into(), denominator * 2));
        }
    }
    Ok(Q::new(1.into(), 2.into()))
}

