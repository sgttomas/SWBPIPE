//! importer.py §10-§13: the layout walk, comments, supports and bends, and the checks after assembly.

use super::{
    english_length_bound, half_unit, index_error, negated, parse_english_length, split_fields, Element, Importer,
    LayoutEntry, Num, Row, Stop, R,
};
use crate::pyfmt::py_float_repr;
use crate::pymath::{acos, py_max, py_min, sum, tan};
use crate::pystr::{match_real, py_strip, search_line_suffix, skip_chars};
use crate::rational::Q;
use crate::runtime::{local_y_reference, norm, sub, unit, unit_exact, V3};
use crate::sidecar::{sidecar_difference, summed_entries};
use crate::value::{OMap, Value};
use crate::{ImportError, Reference, Regenerated};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub(crate) struct NodeRec {
    pub exact: [Q; 3],
    pub bound: [Q; 3],
    pub line: usize,
    pub ti: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct SupportItem {
    pub kind: &'static str,
    pub line: usize,
    pub axis: Option<usize>,
    pub stiffness: Option<Num>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct NodeData {
    pub anchor: Vec<usize>,
    pub la: Vec<usize>,
    pub fixd: OMap<usize, usize>,
    pub fixr: OMap<usize, usize>,
    pub forces: OMap<usize, Vec<(Num, usize)>>,
    pub moments: OMap<usize, Vec<(Num, usize)>>,
    pub disp: OMap<usize, Vec<(Num, usize)>>,
    pub rot: OMap<usize, Vec<(Num, usize)>>,
    pub tag: Vec<(String, usize)>,
    pub comment_lines: Vec<usize>,
    pub load_lines: Vec<usize>,
    pub supports: Vec<SupportItem>,
    pub trak: Vec<(OMap<usize, Num>, usize)>,
    pub rotk: Vec<(OMap<usize, Num>, usize)>,
}

#[derive(Clone, Debug)]
pub(crate) struct Bend {
    pub theta: f64,
    pub t: f64,
    pub d_in: V3,
    pub d_out: V3,
    pub ti: V3,
    pub element: usize,
    pub start: u32,
    pub next: u32,
    pub near: V3,
    pub far: V3,
    pub y_reference: V3,
}

#[derive(Default)]
pub(crate) struct BuildState {
    pub nodes: OMap<u32, NodeRec>,
    pub node_order: Vec<u32>,
    pub elements: Vec<Element>,
    pub node_data: OMap<u32, NodeData>,
    pub run_end_after: HashSet<usize>,
    pub branch_starts: HashSet<u32>,
    pub location_rows: OMap<u32, Vec<usize>>,
    pub layout_present: bool,
    pub positions: OMap<u32, V3>,
    pub bend_geometry: OMap<u32, Bend>,
    pub carriers: OMap<(String, String), Vec<String>>,
    pub frame_x: HashMap<String, Option<V3>>,
    pub length_m: HashMap<String, f64>,
    pub arc_tangents: HashMap<String, Option<(V3, V3)>>,
    pub support_lines: HashMap<String, usize>,
    pub labels: HashMap<String, String>,
}

const AXES: [char; 3] = ['X', 'Y', 'Z'];

/// `_Z_TARGET`: file axis j goes to SWBPIPE axis target with sign.
fn z_target(axis: usize) -> (usize, i32) {
    match axis {
        0 => (0, 1),
        1 => (2, -1),
        _ => (1, 1),
    }
}

fn axis_suffix(name: &str, base: &str) -> Option<usize> {
    let rest = name.strip_prefix(base)?;
    match rest {
        "X" => Some(0),
        "Y" => Some(1),
        "Z" => Some(2),
        _ => None,
    }
}

/// A comment name's table entry: Some(None) for the three-value form, Some(Some(axis)) for an axis form.
fn force_axis(name: &str) -> Option<Option<usize>> {
    if name == "FOR" {
        return Some(None);
    }
    axis_suffix(name, "FOR").map(Some)
}
fn moment_axis(name: &str) -> Option<Option<usize>> {
    if name == "MOM" {
        return Some(None);
    }
    axis_suffix(name, "MOM").map(Some)
}
fn displacement_axis(name: &str) -> Option<Option<usize>> {
    if name == "D" || name == "DIS" {
        return Some(None);
    }
    axis_suffix(name, "DIS").or_else(|| axis_suffix(name, "D")).map(Some)
}
fn rotation_axis(name: &str) -> Option<Option<usize>> {
    if name == "ROT" {
        return Some(None);
    }
    axis_suffix(name, "ROT").map(Some)
}
fn fixd_axis(name: &str) -> Option<Option<usize>> {
    if name == "FIXD" || name == "F" {
        return Some(None);
    }
    axis_suffix(name, "FIXD").or_else(|| axis_suffix(name, "F")).map(Some)
}
fn fixr_axis(name: &str) -> Option<Option<usize>> {
    if name == "FIXR" {
        return Some(None);
    }
    axis_suffix(name, "FIXR").map(Some)
}

const SUPPORT_TYPES: [(&str, &str); 3] = [("G", "guide"), ("LS", "stop"), ("K", "skew")];
const SUPPORT_DATA: [(&str, &[&str]); 4] = [("GGAP", &["G"]), ("MU", &["G", "LS"]), ("STIFF", &["G", "LS"]), ("DV", &["LS", "K"])];
fn support_name(kind: &str) -> &'static str {
    match kind {
        "G" => "a guide (G)",
        "LS" => "a limit stop (LS)",
        _ => "a skewed restraint (K)",
    }
}
const TBD_010_COMMENTS: [(&str, &str); 4] = [
    ("AXIAL", "AXIAL sets a limit stop along its element's local x, but no public page says which element's axes a limit stop takes"),
    ("SHEARY", "SHEARY sets a limit stop along its element's local y, but no public page says which element's axes a limit stop takes"),
    ("SHEARZ", "SHEARZ sets a limit stop along its element's local z, but no public page says which element's axes a limit stop takes"),
    ("KR", "KR, a rotational stiffness, is in in-lb/deg or N-m/deg on the Format of .MBF page and in inch-lb/radian on the technical reference's Anchor page; its unit is open"),
];
fn tbd_010(name: &str) -> Option<&'static str> {
    TBD_010_COMMENTS.iter().find(|(key, _)| *key == name).map(|(_, text)| *text)
}

fn refused_comment_code(name: &str) -> Option<&'static str> {
    let support: &[&str] = &[
        "CLD", "CNOD", "CS", "E", "FREE", "FREEX", "FREEY", "FREEZ", "GRA1", "GRA2", "GRB1", "GRB2", "GRB3", "GRC1",
        "GRC2", "HTYP", "MLV", "NOD", "NOZZLE", "NTHK", "SPIDER", "SR", "US", "VOD", "VTHK", "VS",
    ];
    let numbered = |prefix: &str| (2..11).any(|n| name == format!("{prefix}{n}"));
    let windy = |prefix: &str| (1..5).any(|n| name == format!("{prefix}{n}"));
    let displacement = numbered("DIS")
        || numbered("ROT")
        || windy("WIND")
        || windy("WINR")
        || ["LD", "DISD", "ROTD", "SETT", "STRO", "SEIS", "SEIS2", "SEIS3", "SERO", "SERO2", "SERO3"].contains(&name);
    let load: &[&str] = &["CWGT", "U", "UNIF", "T", "TEMP", "P", "PRES", "SG", "AMB", "HSG", "HPRES", "FRE", "PH"];
    let bend: &[&str] = &["IN1", "IN2", "BTHK", "BMAT", "WS"];
    let row_key: &[&str] = &[
        "WGT", "VWGT", "AWGT", "ES", "TA", "OD1", "OD2", "THK1", "THK2", "CONE", "FLANGE", "BDIA", "GDIA", "BK", "BETA",
        "FFOR", "FTOR", "FRCT", "GAP", "KTIE", "INSF", "LONG", "SHORT", "MAT", "SEC", "LOAD", "LXAX", "LYAX", "RLIM",
        "ROTL", "RK", "L1", "L2", "OFFSET", "RPAD", "THKF", "JCAP", "JMAT", "JSEC", "JLOAD", "JTHK", "JR",
    ];
    let comment: &[&str] = &[
        "SIF", "SIFA", "SIFT", "SIA", "SII", "SIO", "SIT", "SIFB", "IPS", "IBS", "ITS", "IQS", "ISS", "IPC", "IBC",
        "ITC", "IQC", "ISC", "R", "PAD", "CRTCH", "THK", "LEN", "MM", "WTYPE", "TJOINT",
    ];
    if support.contains(&name) {
        Some("MBF-IMPORT-SUPPORT-UNSUPPORTED")
    } else if displacement {
        Some("MBF-IMPORT-DISPLACEMENT-UNSUPPORTED")
    } else if load.contains(&name) {
        Some("MBF-IMPORT-LOAD-UNSUPPORTED")
    } else if bend.contains(&name) {
        Some("MBF-IMPORT-BEND-UNSUPPORTED")
    } else if row_key.contains(&name) {
        Some("MBF-IMPORT-ROW-KEY-UNSUPPORTED")
    } else if comment.contains(&name) {
        Some("MBF-IMPORT-COMMENT-UNSUPPORTED")
    } else {
        None
    }
}

/// `name in _DOCUMENTED_COMMENTS`.
fn documented(name: &str) -> bool {
    force_axis(name).is_some()
        || moment_axis(name).is_some()
        || displacement_axis(name).is_some()
        || rotation_axis(name).is_some()
        || fixd_axis(name).is_some()
        || fixr_axis(name).is_some()
        || ["FLC", "FF", "BSIF", "LA", "TAG", "LTAG", "TRAK", "ROTK"].contains(&name)
        || SUPPORT_TYPES.iter().any(|(key, _)| *key == name)
        || SUPPORT_DATA.iter().any(|(key, _)| *key == name)
        || tbd_010(name).is_some()
        || refused_comment_code(name).is_some()
}

type SupportEntry = (Option<String>, Option<String>, usize, String);

impl<'a> Importer<'a> {
    fn nd(&mut self, number: u32) -> &mut NodeData {
        self.build.node_data.entry_or_insert_with(number, NodeData::default)
    }

    // -- §5.4 lengths ------------------------------------------------------------
    fn length(&mut self, text: &str, line: usize, location: &str) -> R<Option<(Q, Q)>> {
        if self.si {
            let Some(number) = self.real(text, line, location)? else { return Ok(None) };
            return Ok(Some((number.exact, half_unit(text)?)));
        }
        let Some(value) = parse_english_length(text)? else {
            let message = format!(
                "{location} is not a documented English length form (feet, or feet-inch with whole inches 0..11 and \
                 an optional fraction)"
            );
            self.refuse("MBF-IMPORT-LENGTH-NOTATION-INVALID", line, location, &message);
            return Ok(None);
        };
        Ok(Some((value, english_length_bound(text)?)))
    }

    // -- §10-§13 the model -----------------------------------------------------------
    pub(crate) fn build(&mut self, rows: Vec<LayoutEntry>) -> R<(Value, Value)> {
        let mut carried: [Option<String>; 3] = [None, None, None];
        let mut current: Option<u32> = None;
        let mut first_from = true;
        let mut run_refused = false;
        self.build.layout_present = self.sections.contains_key(&"LAYOUT".to_string());
        let mut layout_index: i64 = -1;
        for entry in rows {
            let row = match entry {
                LayoutEntry::Note(line, text) => {
                    let body = py_strip(skip_chars(&text, 1));
                    self.notes.push(if body.is_empty() { format!("line {line}: *") } else { format!("line {line}: {body}") });
                    continue;
                }
                LayoutEntry::Row(row) => row,
            };
            if row.kind == 'L' {
                if !self.build.nodes.contains_key(&row.number) {
                    let message = format!("location row L{} names a node no earlier row defines", row.number);
                    self.refuse("MBF-IMPORT-NODE-UNKNOWN", row.line, &format!("LAYOUT line {}", row.line), &message);
                    continue;
                }
                if !row.keys.is_empty() {
                    self.refuse(
                        "MBF-IMPORT-ROW-KEY-UNSUPPORTED",
                        row.line,
                        &format!("LAYOUT line {}", row.line),
                        "a location row carries comments only (no offsets, M, P, K, J or B)",
                    );
                    continue;
                }
                self.build.location_rows.entry_or_insert_with(row.number, Vec::new).push(row.line);
                self.row_comments(&row, row.number, false)?;
                continue;
            }
            layout_index += 1;
            if !self.build.elements.is_empty() && row.kind == 'F' {
                let last = self.build.elements.len() - 1;
                self.build.run_end_after.insert(last);
            }
            self.row_properties(&row, &mut carried);
            if row.kind == 'F' {
                self.from_row(&row, first_from)?;
                first_from = false;
                current = if self.build.nodes.contains_key(&row.number) { Some(row.number) } else { None };
                run_refused = current.is_none();
            } else {
                if current.is_none() && run_refused {
                    continue;
                }
                let Some(start) = current else {
                    self.refuse("MBF-IMPORT-ROW-SYNTAX-INVALID", row.line, &format!("LAYOUT line {}", row.line), "a To row needs a preceding From row");
                    continue;
                };
                if !self.to_row(&row, start, &carried)? {
                    current = None;
                    run_refused = true;
                    continue;
                }
                self.build.elements.last_mut().expect("element").row = layout_index;
                current = Some(row.number);
            }
            self.row_comments(&row, row.number, true)?;
        }
        if !self.build.elements.is_empty() {
            let last = self.build.elements.len() - 1;
            self.build.run_end_after.insert(last);
        }
        if self.build.elements.is_empty() && self.build.layout_present {
            let header = self.sections.get(&"LAYOUT".to_string()).map(|(line, _)| *line).unwrap_or(0);
            self.refuse("MBF-IMPORT-TOPOLOGY-EMPTY", header, "LAYOUT", "the layout has no element");
        }
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        self.check_supports()?;
        self.resolve_bends()?;
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        if self.derive_only {
            let mut derived = Vec::new();
            for (a, b, start, end) in self.straight_ends() {
                let value = local_y_reference(&start, &end, self.z_vertical);
                if let Some(slot) = derived.iter_mut().find(|(key, _): &&mut ((String, String), V3)| key.0 == a && key.1 == b) {
                    slot.1 = value;
                } else {
                    derived.push(((a, b), value));
                }
            }
            self.derived = Some(derived);
            return Err(Stop::Refused);
        }
        let (model, id_map) = self.assemble()?;
        self.checked_json_domain(&model)?;
        if self.has_errors() {
            return Err(Stop::Refused);
        }
        if self.bound && self.sidecar_current {
            self.regeneration_identity(&model, &id_map)?;
        }
        Ok((model, id_map))
    }

    fn checked_json_domain(&mut self, model: &Value) -> R<()> {
        const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
        fn walk(importer: &mut Importer, value: &Value, path: &str, line: usize) -> R<()> {
            match value {
                Value::Dict(dict) => {
                    let mut line = line;
                    if let Some(Value::Str(provenance)) = dict.get(&"provenance".to_string()) {
                        if let Some(digits) = search_line_suffix(provenance) {
                            let number = crate::rational::py_int_digits(digits)?;
                            line = num_traits::ToPrimitive::to_usize(&number).unwrap_or(usize::MAX);
                        }
                    }
                    for (key, item) in dict.iter() {
                        let child = if path.is_empty() { key.clone() } else { format!("{path}.{key}") };
                        walk(importer, item, &child, line)?;
                    }
                }
                Value::List(items) => {
                    for (index, item) in items.iter().enumerate() {
                        let key = match item.get("id") {
                            Some(Value::Str(id)) => id.clone(),
                            _ => index.to_string(),
                        };
                        walk(importer, item, &format!("{path}[{key}]"), line)?;
                    }
                }
                Value::Int(number) => {
                    let bound = num_bigint::BigInt::from(9_007_199_254_740_991i64);
                    if num_traits::Signed::abs(number) > bound {
                        let message = format!(
                            "{path} is an integer beyond +-(2^53 - 1), which the solve's checked-JSON reader refuses \
                             (CHECKED-JSON-UNSAFE-INTEGER)"
                        );
                        importer.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path, &message);
                    }
                }
                Value::Float(number) => {
                    if !number.is_finite() {
                        let message = format!("{path} is not finite, which the solve's checked-JSON reader refuses (CHECKED-JSON-NON-FINITE)");
                        importer.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path, &message);
                    } else if number.fract() == 0.0 && number.abs() > MAX_SAFE_INTEGER {
                        let message = format!(
                            "{path} is the integral value {} beyond 2^53 - 1, which the solve's checked-JSON reader \
                             refuses (CHECKED-JSON-UNSAFE-INTEGRAL-FLOAT)",
                            py_float_repr(*number)?
                        );
                        importer.refuse("MBF-IMPORT-VALUE-UNREPRESENTABLE", line, path, &message);
                    }
                }
                _ => {}
            }
            Ok(())
        }
        walk(self, model, "", 0)
    }

    fn regeneration_identity(&mut self, model: &Value, id_map: &Value) -> R<()> {
        let regenerator: &dyn crate::SidecarRegenerator = self.options.regenerator.unwrap_or(&crate::SiWriter);
        let reference = match &self.t_ref {
            None => Reference::None,
            Some(number) if self.si => Reference::Celsius(number.value),
            Some(number) => Reference::Fahrenheit(number.value),
        };
        let derived: Vec<String> = id_map
            .get("y_reference_derived")
            .and_then(Value::as_list)
            .map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect())
            .unwrap_or_default();
        let regenerated = regenerator.regenerate(model, &self.sha, &derived, reference).map_err(Stop::Fail)?;
        let sidecar = self.sidecar.clone().expect("bound sidecar");
        let regenerated = match regenerated {
            Regenerated::Refused(codes) => {
                let mut codes = codes;
                codes.sort();
                codes.dedup();
                let message = format!(
                    "the bound sidecar is not the writer's record of the model it yields: the writer does not write \
                     the imported model ({})",
                    codes.join(", ")
                );
                self.refuse("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", 0, "sidecar", &message);
                return Ok(());
            }
            Regenerated::Sidecar(mut value) => {
                // A caller's regenerator is screened like any caller value, before anything clones, compares or
                // replaces part of it (review r3 of #32); a refused tree is dropped without recursion.
                if !crate::value::within_bound(&value) {
                    crate::value::dispose(value);
                    return Err(Stop::Fail(ImportError::Unsupported(crate::value::unrepresentable(
                        crate::value::Unrepresentable::TooDeep,
                        "regenerated sidecar",
                    ))));
                }
                // Compared in the given format: format 3 is 4 without section_map (its import binds no library
                // section, so the writer keys records by OD and wall as the writer of format 3 did), and format 2
                // is 3 without the self-digest.
                let format = sidecar.get_or_null("sidecar_format").clone();
                if let Some(dict) = value.as_dict_mut() {
                    dict.insert("sidecar_format".into(), format);
                    if self.sidecar_format != Some(crate::sidecar::SIDECAR_FORMAT) {
                        dict.remove(&"section_map".to_string());
                    }
                }
                value
            }
        };
        if let Some((path, given, again)) = sidecar_difference(&sidecar, &regenerated, &summed_entries(id_map))? {
            let message = format!(
                "the bound sidecar is not the writer's record of the model it yields: at {path} it has {given}, and \
                 the writer, regenerating the sidecar from the imported model, records {again}. A sidecar whose \
                 identities were edited is applied only on request, by CAEPIPE name, every restored identity \
                 unconfirmed (rebind_by_node_number, Design §15)"
            );
            self.refuse("MBF-IMPORT-SIDECAR-NOT-REPRODUCED", 0, &path, &message);
        }
        Ok(())
    }

    pub(crate) fn straight_ends(&self) -> Vec<(String, String, V3, V3)> {
        let end = |number: u32, near: bool| -> (String, V3) {
            if let Some(bend) = self.build.bend_geometry.get(&number) {
                if near {
                    (format!("{number}A"), bend.near)
                } else {
                    (format!("{number}B"), bend.far)
                }
            } else {
                (number.to_string(), *self.build.positions.get(&number).expect("position"))
            }
        };
        self.build
            .elements
            .iter()
            .map(|element| {
                let (a, start) = end(element.start, false);
                let (b, finish) = end(element.end, true);
                (a, b, start, finish)
            })
            .collect()
    }

    fn row_properties(&mut self, row: &Row, carried: &mut [Option<String>; 3]) {
        let location = format!("LAYOUT line {}", row.line);
        for (slot, letter, what) in [(0usize, 'M', "MATERIAL"), (1, 'P', "PIPE")] {
            if let Some(name) = row.keys.get(&letter).cloned() {
                let known = if letter == 'M' { self.materials.contains_key(&name) } else { self.pipes.contains_key(&name) };
                if !known {
                    self.refuse(
                        "MBF-IMPORT-REFERENCE-UNRESOLVED",
                        row.line,
                        &format!("{location} {letter}"),
                        &format!("{letter}{name} names no {what} record"),
                    );
                }
                carried[slot] = Some(name);
            }
        }
        for (comment_line, text) in &row.comments {
            if text.starts_with("L=") || text == "L" {
                let name = py_strip(skip_chars(text, 2)).to_string();
                self.convention("the load reference L=", *comment_line, &format!("{location} comment L="));
                if !self.loads.contains_key(&name) {
                    self.refuse("MBF-IMPORT-REFERENCE-UNRESOLVED", *comment_line, &format!("{location} comment L="), "L= names no LOADS record");
                }
                carried[2] = Some(name);
            }
        }
        if let Some(joint) = row.keys.get(&'J') {
            let message = format!("joint code J{joint} is not read in tranche 1");
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, &format!("{location} J"), &message);
        }
        if row.keys.contains_key(&'H') {
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, &format!("{location} H"), "hydrotest data are not read");
        }
        let code = row.keys.get(&'K').cloned();
        if let Some(code) = &code {
            if code != "A" && code != "I" {
                let message = format!("node code K{code} is not read in tranche 1 (branch codes set SIFs and flexibility)");
                self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, &format!("{location} K"), &message);
            }
        }
        if code.as_deref() == Some("I") && row.kind != 'T' {
            self.refuse("MBF-IMPORT-ROW-KEY-UNSUPPORTED", row.line, &format!("{location} K"), "KI (tangent intersection) is valid on To rows only");
        }
        if row.keys.contains_key(&'B') && code.as_deref() != Some("I") {
            self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", row.line, &format!("{location} B"), "a bend radius B is read only with KI");
        }
    }

    fn coordinates(&mut self, row: &Row) -> R<Option<OMap<usize, (Q, Q)>>> {
        let mut values = OMap::new();
        for (index, axis) in AXES.iter().enumerate() {
            if let Some(text) = row.keys.get(axis).cloned() {
                let Some(parsed) = self.length(&text, row.line, &format!("LAYOUT line {} {axis}", row.line))? else {
                    return Ok(None);
                };
                values.insert(index, parsed);
            }
        }
        Ok(Some(self.to_swb_pairs(values)))
    }

    fn to_swb_pairs(&self, values: OMap<usize, (Q, Q)>) -> OMap<usize, (Q, Q)> {
        if !self.z_vertical {
            return values;
        }
        let mut out = OMap::new();
        for (axis, (value, bound)) in values.iter() {
            let (target, sign) = z_target(*axis);
            out.insert(target, if sign < 0 { (value.neg(), bound.clone()) } else { (value.clone(), bound.clone()) });
        }
        out
    }

    pub(crate) fn to_swb(&self, values: &OMap<usize, Num>, signed: bool) -> OMap<usize, Num> {
        if !self.z_vertical {
            return values.clone();
        }
        let mut out = OMap::new();
        for (axis, value) in values.iter() {
            let (target, sign) = z_target(*axis);
            out.insert(target, if signed && sign < 0 { negated(value) } else { value.clone() });
        }
        out
    }

    fn file_axis(&self, index: usize) -> char {
        if self.z_vertical {
            ['X', 'Z', 'Y'][index]
        } else {
            AXES[index]
        }
    }

    fn from_row(&mut self, row: &Row, first: bool) -> R<()> {
        let location = format!("LAYOUT line {}", row.line);
        let Some(values) = self.coordinates(row)? else { return Ok(()) };
        if let Some(node) = self.build.nodes.get(&row.number).cloned() {
            self.build.branch_starts.insert(row.number);
            for (index, (value, bound)) in values.iter() {
                if value.sub(&node.exact[*index]).abs() > bound.add(&node.bound[*index]) {
                    let axis = self.file_axis(*index);
                    let message = format!(
                        "F{} {axis} does not equal the node's position within the half-unit bounds of the printed digits",
                        row.number
                    );
                    self.refuse("MBF-IMPORT-FROM-COORDINATES-CONFLICT", row.line, &format!("{location} {axis}"), &message);
                }
            }
            return Ok(());
        }
        let (exact, bound) = if !values.is_empty() {
            let get = |index: usize| values.get(&index).cloned().unwrap_or((Q::zero(), Q::zero()));
            ([get(0).0, get(1).0, get(2).0], [get(0).1, get(1).1, get(2).1])
        } else if first {
            ([Q::zero(), Q::zero(), Q::zero()], [Q::zero(), Q::zero(), Q::zero()])
        } else {
            let message = format!(
                "F{} starts a new run and needs its coordinates (only the first From row may omit them all)",
                row.number
            );
            self.refuse("MBF-IMPORT-NODE-POSITION-MISSING", row.line, &location, &message);
            return Ok(());
        };
        self.add_node(row.number, exact, bound, row.line);
        Ok(())
    }

    fn add_node(&mut self, number: u32, exact: [Q; 3], bound: [Q; 3], line: usize) {
        self.build.nodes.insert(number, NodeRec { exact, bound, line, ti: false });
        self.build.node_order.push(number);
    }

    fn to_row(&mut self, row: &Row, start: u32, carried: &[Option<String>; 3]) -> R<bool> {
        let location = format!("LAYOUT line {}", row.line);
        if self.build.nodes.contains_key(&row.number) {
            let message = format!("T{} names an existing node and would close a loop; loops are tranche 2", row.number);
            self.refuse("MBF-IMPORT-TOPOLOGY-LOOP", row.line, &location, &message);
            return Ok(false);
        }
        let Some(values) = self.coordinates(row)? else { return Ok(false) };
        let origin = self.build.nodes.get(&start).expect("origin").clone();
        let (exact, bound) = if row.absolute {
            if values.len() != 3 {
                self.refuse("MBF-IMPORT-NODE-POSITION-MISSING", row.line, &location, "a row with the absolute-coordinate marker needs X, Y and Z");
                return Ok(false);
            }
            let get = |index: usize| values.get(&index).cloned().expect("axis");
            ([get(0).0, get(1).0, get(2).0], [get(0).1, get(1).1, get(2).1])
        } else {
            let get = |index: usize| values.get(&index).cloned().unwrap_or((Q::zero(), Q::zero()));
            (
                [origin.exact[0].add(&get(0).0), origin.exact[1].add(&get(1).0), origin.exact[2].add(&get(2).0)],
                [origin.bound[0].add(&get(0).1), origin.bound[1].add(&get(1).1), origin.bound[2].add(&get(2).1)],
            )
        };
        if exact == origin.exact {
            let message = format!(
                "the element to T{} has zero length; zero-length joint elements are not read",
                row.number
            );
            self.refuse("MBF-IMPORT-GEOMETRY-ZERO-LENGTH", row.line, &location, &message);
            return Ok(false);
        }
        let (Some(material), Some(section)) = (carried[0].clone(), carried[1].clone()) else {
            let message = format!("the element to T{} has no material or section in effect", row.number);
            self.refuse("MBF-IMPORT-PROPERTY-MISSING", row.line, &location, &message);
            return Ok(false);
        };
        let load = carried[2].clone();
        if load.is_none() && self.sections.contains_key(&"LOADS".to_string()) {
            let message = format!("the element to T{} has no load reference in effect, and the file has LOADS", row.number);
            self.refuse("MBF-IMPORT-PROPERTY-MISSING", row.line, &location, &message);
            return Ok(false);
        }
        self.add_node(row.number, exact, bound, row.line);
        let mut element = Element {
            line: row.line,
            start,
            end: row.number,
            material,
            section,
            load,
            ti: false,
            radius: None,
            factor: None,
            sifs: None,
            row: -1,
        };
        if row.keys.get(&'K').map(String::as_str) == Some("I") {
            let Some(radius_text) = row.keys.get(&'B').cloned() else {
                self.refuse(
                    "MBF-IMPORT-BEND-RADIUS-MISSING",
                    row.line,
                    &location,
                    "KI needs a bend radius B; the default long radius depends on nominal size",
                );
                return Ok(false);
            };
            if !self.si && !match_real(&radius_text) {
                self.refuse("MBF-IMPORT-LENGTH-NOTATION-INVALID", row.line, &format!("{location} B"), "an English bend radius is a plain decimal in inches");
                return Ok(false);
            }
            let Some(radius) = self.real(&radius_text, row.line, &format!("{location} B"))? else { return Ok(false) };
            if !(radius.value > 0.0) {
                self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", row.line, &format!("{location} B"), "a bend radius must be positive");
                return Ok(false);
            }
            element.ti = true;
            element.radius = Some(radius);
            self.build.nodes.get_mut(&row.number).expect("node").ti = true;
        }
        self.build.elements.push(element);
        Ok(true)
    }

    #[allow(clippy::too_many_arguments)]
    fn comment_values(
        &mut self,
        name: &str,
        args: Option<&str>,
        value: Option<&str>,
        axis: Option<usize>,
        line: usize,
        location: &str,
        flag: bool,
    ) -> R<Option<OMap<usize, Num>>> {
        let Some(axis) = axis else {
            let Some(args) = args else {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, &format!("{name} needs three values in parentheses"));
                return Ok(None);
            };
            let parts = split_fields(args).unwrap_or_default();
            if parts.len() != 3 {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, &format!("{name} needs exactly three values"));
                return Ok(None);
            }
            let mut out = OMap::new();
            for (index, part) in parts.iter().enumerate() {
                let Some(number) = self.real(part, line, &format!("{location} value {}", index + 1))? else { return Ok(None) };
                out.insert(index, number);
            }
            return Ok(Some(out));
        };
        if args.is_some() {
            self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, &format!("{name} takes one value, not a list"));
            return Ok(None);
        }
        let Some(value) = value else {
            if flag {
                let mut out = OMap::new();
                out.insert(axis, Num::one());
                return Ok(Some(out));
            }
            self.refuse("MBF-IMPORT-COMMENT-INVALID", line, location, &format!("{name} needs a value"));
            return Ok(None);
        };
        let Some(number) = self.real(value, line, location)? else { return Ok(None) };
        let mut out = OMap::new();
        out.insert(axis, number);
        Ok(Some(out))
    }

    fn row_comments(&mut self, row: &Row, node: u32, allow_load: bool) -> R<()> {
        self.nd(node);
        let location = format!("LAYOUT line {}", row.line);
        if row.kind != 'L' && row.keys.get(&'K').map(String::as_str) == Some("A") {
            self.nd(node).anchor.push(row.line);
        }
        let mut flc: Option<(usize, Option<Num>)> = None;
        let mut loads: Vec<(&'static str, OMap<usize, Num>, usize)> = Vec::new();
        let mut support_data: OMap<String, SupportEntry> = OMap::new();
        for (line, text) in &row.comments {
            let line = *line;
            let head = text.split('=').next().unwrap_or("").split('(').next().unwrap_or("");
            let where_ = format!("{location} comment {}", if head.is_empty() { "?" } else { head });
            if text.starts_with("L=") || text == "L" {
                if !allow_load {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "L= is not read on a location row");
                }
                continue;
            }
            let Some((name, value, args)) = crate::pystr::match_comment(text) else {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "a comment is NAME, NAME=value or NAME(values)");
                continue;
            };
            let (name, value, args) = (name.to_string(), value.map(String::from), args.map(String::from));
            if documented(&name) && value.as_deref().map(|text| py_strip(text).is_empty()).unwrap_or(false) && args.is_none() {
                self.empty_comments += 1;
                continue;
            }
            self.nd(node).comment_lines.push(line);
            let force = force_axis(&name);
            let moment = moment_axis(&name);
            if force.is_some() || moment.is_some() {
                let (kind, axis) = if let Some(axis) = force { ("forces", axis) } else { ("moments", moment.expect("moment")) };
                if let Some(values) = self.comment_values(&name, args.as_deref(), value.as_deref(), axis, line, &where_, false)? {
                    let values = self.to_swb(&values, true);
                    loads.push((kind, values, line));
                }
            } else if name == "FLC" {
                let mut number = self.real(value.as_deref().unwrap_or(""), line, &where_)?;
                if let Some(found) = &number {
                    if found.exact != Q::from_int(1) {
                        self.refuse("MBF-IMPORT-FORCE-CASE-UNSUPPORTED", line, &where_, "only FLC=1 (sustained) is read in tranche 1");
                        number = None;
                    }
                }
                flc = Some((line, number));
            } else if displacement_axis(&name).is_some() || rotation_axis(&name).is_some() {
                let rotation = rotation_axis(&name).is_some();
                let axis = if rotation { rotation_axis(&name) } else { displacement_axis(&name) }.expect("axis");
                if let Some(values) = self.comment_values(&name, args.as_deref(), value.as_deref(), axis, line, &where_, false)? {
                    let rotated = self.to_swb(&values, true);
                    let record = self.nd(node);
                    let store = if rotation { &mut record.rot } else { &mut record.disp };
                    for (axis, number) in rotated.iter() {
                        if !number.exact.is_zero() {
                            store.entry_or_insert_with(*axis, Vec::new).push((number.clone(), line));
                        }
                    }
                }
            } else if fixd_axis(&name).is_some() || fixr_axis(&name).is_some() {
                let rotation = fixr_axis(&name).is_some();
                let axis = if rotation { fixr_axis(&name) } else { fixd_axis(&name) }.expect("axis");
                if name == "F" && args.is_none() {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "F needs three values in parentheses");
                    continue;
                }
                let Some(values) = self.comment_values(&name, args.as_deref(), value.as_deref(), axis, line, &where_, true)? else {
                    continue;
                };
                for (axis, number) in self.to_swb(&values, false).iter() {
                    if number.exact != Q::zero() && number.exact != Q::from_int(1) {
                        self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, &format!("{name} values must be 0 or 1"));
                        break;
                    }
                    if number.exact == Q::from_int(1) {
                        let record = self.nd(node);
                        if rotation {
                            record.fixr.insert(*axis, line);
                        } else {
                            record.fixd.insert(*axis, line);
                        }
                    }
                }
            } else if name == "FF" {
                if row.keys.get(&'K').map(String::as_str) != Some("I") {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "FF is read on a KI bend row only");
                    continue;
                }
                let mut number = self.real(value.as_deref().unwrap_or(""), line, &where_)?;
                if let Some(found) = &number {
                    if !(found.value > 0.0) {
                        self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &where_, "a flexibility factor must be positive");
                        number = None;
                    }
                }
                if let Some(number) = number {
                    let Some(last) = self.build.elements.last_mut() else { return Err(index_error()) };
                    last.factor = Some(number);
                }
                let lines = &mut self.nd(node).comment_lines;
                if let Some(at) = lines.iter().position(|&item| item == line) {
                    lines.remove(at);
                }
            } else if name == "BSIF" {
                // Format of .MBF, BSIF: a bend's SIFs, in-plane then out-of-plane (§11)
                if row.keys.get(&'K').map(String::as_str) != Some("I") {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "BSIF is read on a KI bend row only");
                    continue;
                }
                let parts = match args.as_deref() {
                    Some(text) => split_fields(text),
                    None => None,
                };
                let Some(parts) = parts.filter(|parts| parts.len() == 2) else {
                    self.refuse(
                        "MBF-IMPORT-COMMENT-INVALID",
                        line,
                        &where_,
                        "BSIF needs two values in parentheses, in-plane then out-of-plane",
                    );
                    continue;
                };
                let first = self.real(&parts[0], line, &format!("{where_} value 1"))?;
                let second = self.real(&parts[1], line, &format!("{where_} value 2"))?;
                let (Some(first), Some(second)) = (first, second) else { continue };
                if !(first.value > 0.0 && second.value > 0.0) {
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &where_, "a bend SIF must be positive");
                    continue;
                }
                let Some(last) = self.build.elements.last_mut() else { return Err(index_error()) };
                last.sifs = Some((first, second));
                let lines = &mut self.nd(node).comment_lines;
                if let Some(at) = lines.iter().position(|&item| item == line) {
                    lines.remove(at);
                }
            } else if name == "LA" {
                let number = self.real(value.as_deref().unwrap_or(""), line, &where_)?;
                match number {
                    Some(found) if !found.exact.is_zero() => {
                        self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &where_, "an anchor in local axes (LA=1) is not read")
                    }
                    Some(_) => self.nd(node).la.push(line),
                    None => {}
                }
            } else if SUPPORT_TYPES.iter().any(|(key, _)| *key == name)
                || SUPPORT_DATA.iter().any(|(key, _)| *key == name)
                || tbd_010(&name).is_some()
            {
                if support_data.contains_key(&name) {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, &format!("{name} is repeated on one row"));
                } else {
                    support_data.insert(name.clone(), (value.clone(), args.clone(), line, where_.clone()));
                }
            } else if name == "TRAK" || name == "ROTK" {
                if let Some(values) = self.comment_values(&name, args.as_deref(), value.as_deref(), None, line, &where_, false)? {
                    let moved = self.to_swb(&values, false);
                    let record = self.nd(node);
                    if name == "TRAK" {
                        record.trak.push((moved, line));
                    } else {
                        record.rotk.push((moved, line));
                    }
                }
            } else if name == "TAG" {
                let tag = py_strip(value.as_deref().unwrap_or("")).to_string();
                self.nd(node).tag.push((tag, line));
            } else if name == "LTAG" {
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &where_, "a non-empty LTAG (spectrum level) is not read");
            } else {
                match refused_comment_code(&name) {
                    None => self.refuse("MBF-IMPORT-COMMENT-UNKNOWN", line, &where_, &format!("comment {name} is not in the documented comment list")),
                    Some(code) => self.refuse(code, line, &where_, &format!("comment {name} is not read by this profile (Design §12 to §14)")),
                }
            }
        }
        let any_loads = !loads.is_empty();
        for (kind, values, line) in loads {
            match &flc {
                None => {
                    let what = format!("a {} without FLC", if kind == "forces" { "force" } else { "moment" });
                    let place = format!("{location} {}", if kind == "forces" { "FOR" } else { "MOM" });
                    if !self.convention(&what, line, &place) {
                        continue;
                    }
                }
                Some((_, None)) => continue,
                Some(_) => {}
            }
            let record = self.nd(node);
            let store = if kind == "forces" { &mut record.forces } else { &mut record.moments };
            for (axis, number) in values.iter() {
                store.entry_or_insert_with(*axis, Vec::new).push((number.clone(), line));
            }
            record.load_lines.push(line);
        }
        if let Some((flc_line, _)) = flc {
            if !any_loads {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", flc_line, &format!("{location} comment FLC"), "FLC without FOR or MOM");
            }
        }
        if !support_data.is_empty() {
            self.row_supports(node, &support_data, &location)?;
        }
        Ok(())
    }

    fn single_value(&mut self, name: &str, entry: &SupportEntry) -> R<Option<Num>> {
        let (value, args, line, where_) = entry;
        let (Some(value), None) = (value, args) else {
            self.refuse("MBF-IMPORT-COMMENT-INVALID", *line, where_, &format!("{name} takes one value, as {name}=value"));
            return Ok(None);
        };
        self.real(value, *line, where_)
    }

    fn row_supports(&mut self, node: u32, data: &OMap<String, SupportEntry>, location: &str) -> R<()> {
        let start = self.errors();
        let mut tbd: Vec<&String> = data.keys().filter(|name| tbd_010(name).is_some()).collect();
        tbd.sort();
        for name in tbd {
            let entry = data.get(name).expect("entry");
            self.refuse("MBF-IMPORT-TBD-17-04-010", entry.2, &entry.3, tbd_010(name).expect("tbd"));
        }
        let kinds: Vec<&str> = SUPPORT_TYPES.iter().map(|(key, _)| *key).filter(|key| data.contains_key(&key.to_string())).collect();
        if kinds.len() > 1 {
            let message = format!(
                "{} on one row: a row carries one guide (G), limit stop (LS) or skewed restraint (K), and its other \
                 comments are that support's data",
                kinds.join(" and ")
            );
            let line = data.get(&kinds[1].to_string()).expect("kind").2;
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, location, &message);
            return Ok(());
        }
        let kind = kinds.first().copied();
        for (name, readers) in SUPPORT_DATA {
            if let Some(entry) = data.get(&name.to_string()) {
                if !kind.map(|kind| readers.contains(&kind)).unwrap_or(false) {
                    let names: Vec<&str> = readers.iter().map(|reader| support_name(reader)).collect();
                    let message = format!("{name} is read only as the data of {} on its row", names.join(" or "));
                    self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", entry.2, &entry.3, &message);
                }
            }
        }
        let Some(kind) = kind else { return Ok(()) };
        if self.errors() > start {
            return Ok(());
        }
        let (value, args, line, where_) = data.get(&kind.to_string()).expect("kind").clone();
        let mut stiffness: Option<Num> = None;
        let mut axis: Option<usize> = None;
        if let Some(entry) = data.get(&"GGAP".to_string()) {
            if let Some(gap) = self.single_value("GGAP", entry)? {
                if !gap.exact.is_zero() {
                    self.refuse(
                        "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                        entry.2,
                        &entry.3,
                        "a guide gap (GGAP) is a clearance the pipe closes before the guide acts, symmetric about the \
                         guide axis: SWBPIPE has no support member for it; only GGAP=0 is read",
                    );
                }
            }
        }
        if let Some(entry) = data.get(&"MU".to_string()) {
            if let Some(mu) = self.single_value("MU", entry)? {
                if mu.exact.is_negative() {
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", entry.2, &entry.3, "a friction coefficient is not negative");
                } else if !mu.exact.is_zero() {
                    self.refuse(
                        "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                        entry.2,
                        &entry.3,
                        "a friction coefficient (MU) makes the support nonlinear, its friction opposing motion normal \
                         to the restraint; only MU=0 is read",
                    );
                }
            }
        }
        if let Some(entry) = data.get(&"STIFF".to_string()) {
            let rigid = entry.0.as_deref() == Some("Rigid") && entry.1.is_none();
            if !rigid {
                stiffness = self.single_value("STIFF", entry)?;
                if let Some(found) = &stiffness {
                    if !(found.exact > Q::zero()) {
                        self.refuse(
                            "MBF-IMPORT-VALUE-OUT-OF-RANGE",
                            entry.2,
                            &entry.3,
                            "a stiffness (STIFF) must be positive; without STIFF the support is rigid",
                        );
                    }
                }
            }
        }
        if let Some((dv_value, dv_args, dv_line, dv_where)) = data.get(&"DV".to_string()).cloned() {
            if let Some(components) = self.comment_values("DV", dv_args.as_deref(), dv_value.as_deref(), None, dv_line, &dv_where, false)? {
                let along: Vec<usize> =
                    self.to_swb(&components, true).iter().filter(|(_, number)| !number.exact.is_zero()).map(|(axis, _)| *axis).collect();
                if along.is_empty() {
                    self.refuse("MBF-IMPORT-COMMENT-INVALID", dv_line, &dv_where, "DV(0,0,0) names no direction");
                } else if along.len() > 1 {
                    self.refuse(
                        "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                        dv_line,
                        &dv_where,
                        "a direction vector off the global axes is a skewed restraint direction; SWBPIPE's supports \
                         restrain global DOFs only",
                    );
                } else {
                    axis = Some(along[0]);
                }
            }
        }
        if kind == "G" {
            if value.is_some() || args.is_some() {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "G takes no value");
            }
        } else if kind == "LS" {
            let parts = args.as_deref().and_then(split_fields);
            match parts {
                Some(parts) if parts.len() == 2 => {
                    for (label, part) in ["lower", "upper"].iter().zip(parts.iter()) {
                        if part.is_empty() || part == "None" {
                            let message = format!(
                                "a blank {label} limit (empty or None) leaves that side free: a one-way restraint, \
                                 which is nonlinear; only LS(0,0), stopped both ways at the undeflected position, is read"
                            );
                            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &format!("{where_} {label}"), &message);
                            continue;
                        }
                        if let Some(limit) = self.real(part, line, &format!("{where_} {label}"))? {
                            if !limit.exact.is_zero() {
                                let message = format!(
                                    "a non-zero {label} limit is a gap the node moves through freely before the stop \
                                     acts, which is nonlinear; only LS(0,0) is read"
                                );
                                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &format!("{where_} {label}"), &message);
                            }
                        }
                    }
                }
                _ => self.refuse("MBF-IMPORT-COMMENT-INVALID", line, &where_, "LS needs its two limits, LS(lower,upper)"),
            }
            if !data.contains_key(&"DV".to_string()) {
                self.refuse(
                    "MBF-IMPORT-TBD-17-04-010",
                    line,
                    &where_,
                    "a limit stop without DV: the Format of .MBF page documents no default direction",
                );
            }
        } else {
            let entry = data.get(&"K".to_string()).expect("K").clone();
            stiffness = self.single_value("K", &entry)?;
            if let Some(found) = &stiffness {
                if !(found.exact > Q::zero()) {
                    self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &where_, "a skewed restraint's stiffness K must be positive");
                }
            }
            if !data.contains_key(&"DV".to_string()) {
                self.refuse(
                    "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                    line,
                    &where_,
                    "a skewed restraint needs its direction DV (a connected node, CNOD, is not read)",
                );
            }
        }
        if self.errors() == start {
            let kind_name = SUPPORT_TYPES.iter().find(|(key, _)| *key == kind).map(|(_, name)| *name).expect("kind");
            self.nd(node).supports.push(SupportItem { kind: kind_name, line, axis, stiffness });
        }
        Ok(())
    }

    fn check_supports(&mut self) -> R<()> {
        let numbers: Vec<u32> = self.build.node_data.keys().copied().collect();
        for number in numbers {
            let location = format!("node {number}");
            let record = self.build.node_data.get(&number).expect("record").clone();
            if !record.la.is_empty() && record.anchor.is_empty() {
                self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", record.la[0], &location, "LA is read only with KA");
            }
            self.check_flexible_anchor(number, &record);
            for index in 0..record.supports.len() {
                if record.supports[index].kind == "guide" {
                    let axis = self.guide_axis(number, record.supports[index].line);
                    self.build.node_data.get_mut(&number).expect("record").supports[index].axis = axis;
                }
            }
            for (key, items) in [("disp", &record.disp), ("rot", &record.rot)] {
                let _ = key;
                if !items.is_empty() && record.anchor.is_empty() {
                    let line = items.values().flat_map(|list| list.iter().map(|(_, line)| *line)).min().expect("line");
                    self.refuse(
                        "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED",
                        line,
                        &location,
                        "specified displacements and rotations are read at anchors (KA) only",
                    );
                }
            }
            if self.build.nodes.get(&number).map(|node| node.ti).unwrap_or(false) {
                let mut lines: Vec<usize> = Vec::new();
                lines.extend(&record.comment_lines);
                lines.extend(&record.load_lines);
                lines.extend(&record.anchor);
                if let Some(rows) = self.build.location_rows.get(&number) {
                    lines.extend(rows);
                }
                if let Some(&line) = lines.iter().min() {
                    self.refuse(
                        "MBF-IMPORT-BEND-UNSUPPORTED",
                        line,
                        &location,
                        "a location row, support or load at a tangent-intersection node is not read",
                    );
                }
            }
        }
        Ok(())
    }

    fn check_flexible_anchor(&mut self, number: u32, record: &NodeData) {
        let location = format!("node {number}");
        for (name, items) in [("TRAK", &record.trak), ("ROTK", &record.rotk)] {
            if items.len() > 1 {
                self.refuse("MBF-IMPORT-COMMENT-INVALID", items[1].1, &format!("{location} {name}"), &format!("{name} is given twice at node {number}"));
            }
            if !items.is_empty() && record.anchor.is_empty() {
                self.refuse(
                    "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                    items[0].1,
                    &format!("{location} {name}"),
                    &format!("{name} is an anchor's stiffness and is read only with KA"),
                );
            }
        }
        if record.anchor.is_empty() || (record.trak.is_empty() && record.rotk.is_empty()) {
            return;
        }
        let line = record.anchor[0];
        if record.trak.is_empty() || record.rotk.is_empty() {
            self.refuse(
                "MBF-IMPORT-TBD-17-04-010",
                line,
                &location,
                "a flexible anchor needs both TRAK and ROTK: no public page says whether the DOFs of an absent one stay \
                 rigid or are free",
            );
            return;
        }
        let trak = &record.trak[0].0;
        let rotk = &record.rotk[0].0;
        if trak.values().chain(rotk.values()).any(|value| value.exact.is_negative()) {
            self.refuse("MBF-IMPORT-VALUE-OUT-OF-RANGE", line, &location, "an anchor stiffness is not negative");
        } else if rotk.values().any(|value| !value.exact.is_zero()) {
            self.refuse(
                "MBF-IMPORT-TBD-17-04-010",
                record.rotk[0].1,
                &format!("{location} ROTK"),
                "a non-zero ROTK: its unit is in-lb/deg or N-m/deg on the Format of .MBF page and inch-lb/radian on the \
                 technical reference's Anchor page, so it is open; ROTK(0,0,0), free rotations, is read",
            );
        } else if trak.values().all(|value| value.exact.is_zero()) {
            let message = format!("the flexible anchor at node {number} has no stiffness in any DOF and restrains nothing");
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &location, &message);
        }
        if !record.fixd.is_empty() || !record.fixr.is_empty() || !record.supports.is_empty() {
            self.refuse(
                "MBF-IMPORT-SUPPORT-UNSUPPORTED",
                line,
                &location,
                "a flexible anchor (KA with TRAK and ROTK) is read alone at its node: no public page says how it \
                 combines with another restraint there",
            );
        }
        for items in [&record.disp, &record.rot] {
            if !items.is_empty() {
                let moved = items.values().flat_map(|list| list.iter().map(|(_, line)| *line)).min().expect("line");
                self.refuse(
                    "MBF-IMPORT-DISPLACEMENT-UNSUPPORTED",
                    moved,
                    &location,
                    "a specified displacement at a flexible anchor moves the ground end of its springs; SWBPIPE's \
                     imposed displacement is a rigid prescribed DOF",
                );
            }
        }
    }

    fn guide_axis(&mut self, number: u32, line: usize) -> Option<usize> {
        let location = format!("node {number} G");
        let before: Vec<Element> = self.build.elements.iter().filter(|element| element.end == number).cloned().collect();
        let after: Vec<Element> = self.build.elements.iter().filter(|element| element.start == number).cloned().collect();
        if before.is_empty() && after.len() != 1 {
            let message = format!(
                "node {number} has no preceding element and {} following ones, so the guide's element is not determined",
                after.len()
            );
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &location, &message);
            return None;
        }
        let element = if before.is_empty() { after[0].clone() } else { before[0].clone() };
        let start = self.build.nodes.get(&element.start).expect("start").exact.clone();
        let end = self.build.nodes.get(&element.end).expect("end").exact.clone();
        let along: Vec<usize> = (0..3).filter(|&index| end[index] != start[index]).collect();
        if along.len() != 1 {
            let message = format!(
                "the guide's element, {} to {}, does not lie along a global axis, so its lateral directions are \
                 skewed; SWBPIPE's supports restrain global DOFs only",
                element.start, element.end
            );
            self.refuse("MBF-IMPORT-SUPPORT-UNSUPPORTED", line, &location, &message);
            return None;
        }
        Some(along[0])
    }

    // -- §11 bends ---------------------------------------------------------------
    fn resolve_bends(&mut self) -> R<()> {
        let nodes: Vec<(u32, NodeRec)> = self.build.nodes.iter().map(|(number, node)| (*number, node.clone())).collect();
        for (number, node) in &nodes {
            let mut point = [0.0f64; 3];
            for axis in 0..3 {
                point[axis] = self.binary64_q(&node.exact[axis], node.line, &format!("node {number}"), "node coordinate", false)?;
            }
            self.build.positions.insert(*number, point);
        }
        let elements = self.build.elements.clone();
        for element in &elements {
            let length = norm(&sub(&self.position(element.end), &self.position(element.start)));
            if !(0.0 < length && length < f64::INFINITY) {
                let code = if length == 0.0 { "MBF-IMPORT-GEOMETRY-ZERO-LENGTH" } else { "MBF-IMPORT-VALUE-UNREPRESENTABLE" };
                let message = format!(
                    "the element to node {} has a length of zero or beyond binary64 once its end positions are rounded",
                    element.end
                );
                self.refuse(code, element.line, &format!("LAYOUT line {}", element.line), &message);
                return Err(Stop::Refused);
            }
        }
        for (index, element) in elements.iter().enumerate() {
            if !element.ti {
                continue;
            }
            let location = format!("LAYOUT line {}", element.line);
            let n = element.end;
            let outgoing = elements.get(index + 1);
            if self.build.run_end_after.contains(&index) || outgoing.map(|item| item.start != n).unwrap_or(true) {
                let message = format!("the run ends at tangent intersection {n}; a bend needs an outgoing leg");
                self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", element.line, &location, &message);
                continue;
            }
            let outgoing = outgoing.expect("outgoing");
            if self.build.branch_starts.contains(&n) {
                self.refuse("MBF-IMPORT-BEND-UNSUPPORTED", element.line, &location, &format!("a branch starts at tangent intersection {n}"));
                continue;
            }
            let p = self.position(element.start);
            let ti = self.position(n);
            let q = self.position(outgoing.end);
            let d_in = unit(&sub(&ti, &p));
            let d_out = unit(&sub(&q, &ti));
            let cosine = py_max(-1.0, py_min(1.0, sum(&[d_in[0] * d_out[0], d_in[1] * d_out[1], d_in[2] * d_out[2]])));
            let theta = acos(cosine)?;
            if theta <= 1e-9 || theta >= std::f64::consts::PI - 1e-9 {
                let message = format!("the legs at tangent intersection {n} are parallel or reversed; there is no bend angle");
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, &location, &message);
                continue;
            }
            let radius = element.radius.as_ref().expect("radius").value;
            let t = self.binary64_f(radius * tan(theta / 2.0)?, element.line, &location, "tangent length", true)?;
            self.build.bend_geometry.insert(
                n,
                Bend { theta, t, d_in, d_out, ti, element: index, start: element.start, next: outgoing.end, near: [0.0; 3], far: [0.0; 3], y_reference: [0.0; 3] },
            );
        }
        let tolerance = 1e-9;
        let bends: Vec<(u32, Bend)> = self.build.bend_geometry.iter().map(|(n, bend)| (*n, bend.clone())).collect();
        for (n, bend) in &bends {
            let element = elements[bend.element].clone();
            for (leg, other) in [("incoming", bend.start), ("outgoing", bend.next)] {
                let span = norm(&sub(&bend.ti, &self.position(other)));
                let other_t = self.build.bend_geometry.get(&other).map(|item| item.t).unwrap_or(0.0);
                if span - bend.t - other_t <= tolerance * span {
                    let message = format!(
                        "the {leg} leg at tangent intersection {n} is not longer than the tangent length R \
                         tan(theta/2){}",
                        if other_t != 0.0 { " plus the adjacent bend's" } else { "" }
                    );
                    self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, &format!("LAYOUT line {}", element.line), &message);
                }
            }
        }
        for (n, bend) in &bends {
            let ti_exact: Vec<Q> = bend.ti.iter().map(|&value| Q::from_f64(value).expect("finite")).collect();
            let t_exact = Q::from_f64(bend.t).expect("finite");
            let line = elements[bend.element].line;
            let mut near = [0.0f64; 3];
            let mut far = [0.0f64; 3];
            let near_exact: Vec<Q> = (0..3).map(|axis| ti_exact[axis].sub(&t_exact.mul(&Q::from_f64(bend.d_in[axis]).expect("finite")))).collect();
            let far_exact: Vec<Q> = (0..3).map(|axis| ti_exact[axis].add(&t_exact.mul(&Q::from_f64(bend.d_out[axis]).expect("finite")))).collect();
            for axis in 0..3 {
                near[axis] = self.binary64_q(&near_exact[axis], line, &format!("bend {n}"), "tangent node", false)?;
            }
            for axis in 0..3 {
                far[axis] = self.binary64_q(&far_exact[axis], line, &format!("bend {n}"), "tangent node", false)?;
            }
            let two = Q::from_int(2);
            let toward: [Q; 3] = std::array::from_fn(|axis| {
                ti_exact[axis].sub(&Q::from_f64(near[axis]).expect("finite").add(&Q::from_f64(far[axis]).expect("finite")).div(&two))
            });
            let smallest = py_min(py_min(norm(&sub(&bend.ti, &near)), norm(&sub(&far, &bend.ti))), norm(&sub(&far, &near)));
            if smallest == 0.0 || toward.iter().all(Q::is_zero) {
                let message = format!(
                    "bend {n}'s tangent nodes collapse onto the tangent intersection or each other once rounded to binary64"
                );
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", line, &format!("bend {n}"), &message);
                return Err(Stop::Refused);
            }
            let y_reference = unit_exact(&toward)?;
            let stored = self.build.bend_geometry.get_mut(n).expect("bend");
            stored.near = near;
            stored.far = far;
            stored.y_reference = y_reference;
        }
        for element in &elements {
            let start = self.build.bend_geometry.get(&element.start).map(|bend| bend.far).unwrap_or_else(|| self.position(element.start));
            let end = self.build.bend_geometry.get(&element.end).map(|bend| bend.near).unwrap_or_else(|| self.position(element.end));
            let length = norm(&sub(&end, &start));
            if !(0.0 < length && length < f64::INFINITY) {
                let message = format!(
                    "the straight to node {} beside a bend has zero length once its generated tangent nodes are rounded \
                     to binary64",
                    element.end
                );
                self.refuse("MBF-IMPORT-BEND-GEOMETRY-INVALID", element.line, &format!("LAYOUT line {}", element.line), &message);
                return Err(Stop::Refused);
            }
        }
        Ok(())
    }

    pub(crate) fn position(&self, number: u32) -> V3 {
        *self.build.positions.get(&number).expect("position")
    }
}
