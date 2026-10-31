//! The SI MBF writer (DEL-17-04 Design `desktop-mbf-export-import.md` §2, slice 2): a byte-faithful port of
//! `core/handoff/caepipe_mbf/si_writer.py`.
//!
//! [`render_si`] turns a SWBPIPE model document into MBF text in the SI unit set, or refuses it by name, with the
//! Python's codes, messages, maps, loss report and assumptions; [`regenerate_caepipe_sidecar`] is the importer's
//! regeneration identity. The units' exact normalization factors are rationals ([`Q`]), each sum is rounded once,
//! and the bend's trigonometry is the correctly rounded one the importer uses, so the text is the Python's on every
//! platform. A Python exception the writer raises is [`ImportError::Python`]; the writer's own preflight import is
//! the Rust importer.

mod read;
mod render;

use crate::pyfmt::{canonical_bytes, py_format_g, py_format_g_q, py_repr};
use crate::pystr::PyError;
use crate::rational::{bigint_to_f64, Q};
use crate::sidecar::{py_str, sidecar_self_digest, NotCanonical, SIDECAR_DIGEST_KEY, SIDECAR_FORMAT};
use crate::value::{Dict, OMap, Value};
use crate::{dict, ImportError, Reference, Regenerated, SidecarRegenerator};
use num_bigint::BigInt;
use std::collections::{BTreeMap, BTreeSet};

pub const MBF_SI_PROFILE_ID: &str = "caepipe-mbf-si-pilot-1";
pub(crate) const STANDARD_GRAVITY: f64 = 9.80665;
const MBF_VERSION: &str = "15.10";
const RETIRED_MODEL_VERSIONS: [&str; 7] = ["0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0", "0.7.0"];
const SUSTAINED: &str = "Sustained (W+P)";
const EXPANSION: &str = "Expansion (T1)";
pub(crate) const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
const ABSOLUTE_FAHRENHEIT: &str = "T(degC) = (T(degF) - 32) x 5/9";
const PRESSURE_CONTRACT_VERSION: (&str, &str) = ("3.0.0", "exact_pressure_v3");
const SHEAR_MODELS: [&str; 2] = ["euler_bernoulli", "timoshenko_annulus_energy_v1"];
const PRESSURE_BASIS: &str = "internal_differential_zero_external_v1";
const CLOSURE_TRANSFERS: [&str; 2] = ["transfers_to_wall", "separately_supported_or_compensated"];
const OPPRES_LINE: &str = "0,0,0,1,0";
const OPCODE_LINE: &str = "0,0,0";
const B31J_LOCKED_CODES: [&str; 1] = ["B311"];
const REGION_CHECK_CODES: [&str; 4] = [
    "MBF-IMPORT-PRESSURE-RUN-NONCOLLINEAR",
    "MBF-IMPORT-PRESSURE-BORE-MISMATCH",
    "MBF-IMPORT-PRESSURE-MITRE",
    "MBF-IMPORT-PRESSURE-GEOMETRY-INVALID",
];
const BEND_SOLVER_CONSUMPTION: &str = "curved_bend_macro_element";
const BEND_SIF_FIELDS: [&str; 2] = ["sif_in_plane_user_value", "sif_out_of_plane_user_value"];
const BEND_RELATIVE_TOLERANCE: f64 = 1e-9;
const PROJECT_UNITS: [(&str, &[&str]); 3] =
    [("length", &["m", "mm", "in"]), ("force", &["N", "lbf"]), ("temperature", &["degC", "degF"])];
const INVENTED_CERTIFICATION: &str = "invented non-engineering example";
const USER_SUPPLIED_CERTIFICATION: &str = "user-supplied own source";
const HANDLED_MEMBERS: [&str; 14] = [
    "schema_version", "document_kind", "pressure_contract", "project", "materials", "nodes", "pipe_segments",
    "supports", "components", "load_cases", "combinations", "reference_configurations", "model_settings",
    "shear_model",
];
const NOT_CARRIED: &str = "the MBF has no field for it and the sidecar does not carry it";
const IMPORTED_PROJECT_UNITS: [(&str, &str); 7] = [
    ("length", "mm"), ("force", "N"), ("moment", "N*m"), ("pressure", "MPa"), ("stress", "Pa"),
    ("temperature", "degC"), ("angle", "deg"),
];
const TITLE_MAX: usize = 70;
const DESCRIPTION_MAX: usize = 30;
const MAX_NODES: usize = 9999;

/// `_HANDLED_FIELDS[path]`.
fn handled_fields(path: &str) -> &'static [&'static str] {
    match path {
        "materials" => &[
            "id", "elastic_modulus", "shear_modulus", "poisson_ratio", "thermal_expansion_coefficient",
            "temperature_points", "expansion_laws",
        ],
        "nodes" => &["id", "position"],
        "pipe_segments" => &["id", "from", "to", "section", "material", "y_reference"],
        "pipe_segments.section" => &[
            "outside_diameter", "wall_thickness", "corrosion_allowance", "mill_tolerance", "insulation_thickness",
            "material_density", "contents_density", "insulation_density",
        ],
        "supports" => &["id", "node", "restraints", "family", "stiffness", "hanger", "nonlinear"],
        "components" => &["id", "kind", "node", "geometry", "modifiers", "mechanics_interface"],
        "components.geometry" => &["bend_pipe_ref", "bend_radius", "bend_angle"],
        "components.modifiers" => {
            &["flexibility_factor_user_value", "sif_in_plane_user_value", "sif_out_of_plane_user_value"]
        }
        "components.mechanics_interface" => &["solver_consumption"],
        "load_cases" => &[
            "id", "primitive_loads", "pressure_regions", "equivalent_static", "modulus_basis_ref",
            "modulus_basis_temperature", "analysis_state",
        ],
        "load_cases.primitive_loads" => &["id", "category", "target", "direction", "magnitude", "dimension"],
        "load_cases.pressure_regions" => {
            &["id", "member_pipe_ids", "member_connector_ids", "pressure_basis", "pressure", "terminals"]
        }
        "project" => &["units"],
        _ => &[],
    }
}

/// `_OMITTED_REASONS.get(path, _NOT_CARRIED)`.
fn omitted_reason(path: &str) -> &'static str {
    match path {
        "data_boundary" => "the import writes its own data boundary for the local document",
        "analysis_status" => "the analysis state is not in the MBF; the import starts it afresh",
        "sections" => "each pipe segment's own section is written as a PIPE record; the section library is not",
        "diagnostics" => "stored diagnostics are not in the MBF; the import's validation forms its own",
        "materials.constitutive_basis" => "the MBF carries E and the Poisson ratio; the import forms its own basis",
        "materials.provenance_record" => "only its origin classification is carried, in the sidecar's material_map",
        "pipe_segments.section_ref" => "the pipe segment's own section is written; the section library is not",
        "supports.properties" => "display properties the solve ignores",
        "components.completeness" => "a derived finding; the import's validation forms its own",
        "components.modifiers.sif_user_value" => {
            "BSIF carries the directional pair; the scalar SIF is a different SWBPIPE field, which BSIF(s,s) would \
             import as the pair"
        }
        "components.modifiers.source_reference" => "the import names the MBF line as the source reference",
        "load_cases.label" => "the import labels each case by the target program's case (load_case_map)",
        "load_cases.kind" => "the import forms each case's kind from the target program's case (load_case_map)",
        "load_cases.status" => "the import sets each case's status",
        "load_cases.pressure_regions.provenance" => "the import names the MBF line as the region's provenance",
        _ => NOT_CARRIED,
    }
}

// ---------------------------------------------------------------------------
// units
// ---------------------------------------------------------------------------

/// How a unit scales to the MBF unit: an exact power of ten, an exact definitional normalization (by its
/// NORMALIZATION key), the spring stiffness's and the pressure's own psi-derived factors, or radians to degrees.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Scale {
    Pow10(i64),
    Norm(&'static str),
    StiffnessLbfIn,
    PressurePsi,
    Rad,
}

pub(crate) type Table = &'static [(&'static str, Scale)];

use Scale::{Norm, Pow10};

pub(crate) const LENGTH_TO_MM: Table = &[("m", Pow10(3)), ("mm", Pow10(0)), ("in", Norm("in"))];
pub(crate) const STRESS_TO_MPA: Table =
    &[("Pa", Pow10(-6)), ("kPa", Pow10(-3)), ("MPa", Pow10(0)), ("GPa", Pow10(3)), ("psi", Norm("psi"))];
pub(crate) const FORCE_TO_N: Table = &[("N", Pow10(0)), ("kN", Pow10(3)), ("lbf", Norm("lbf"))];
pub(crate) const MOMENT_TO_NM: Table =
    &[("N*m", Pow10(0)), ("kN*m", Pow10(3)), ("lbf*in", Norm("lbf*in")), ("lbf*ft", Norm("lbf*ft"))];
pub(crate) const FORCE_PER_LENGTH_TO_N_PER_M: Table =
    &[("N/m", Pow10(0)), ("N/mm", Pow10(3)), ("kN/m", Pow10(3)), ("kN/mm", Pow10(6)), ("lbf/in", Norm("lbf/in"))];
pub(crate) const EXPANSION_TO_PER_C: Table = &[("1/K", Pow10(0)), ("1/degC", Pow10(0)), ("1/degF", Norm("1/degF"))];
pub(crate) const TEMPERATURE_INTERVAL_TO_C: Table =
    &[("K", Pow10(0)), ("degC", Pow10(0)), ("C", Pow10(0)), ("degF", Norm("degF"))];
pub(crate) const ROTATION_TO_DEG: Table = &[("rad", Scale::Rad), ("deg", Pow10(0))];
pub(crate) const DIMENSIONLESS: Table = &[("1", Pow10(0)), ("none", Pow10(0))];
pub(crate) const STIFFNESS_TO_N_PER_MM: Table = &[
    ("N/m", Pow10(-3)), ("N/mm", Pow10(0)), ("kN/m", Pow10(0)), ("kN/mm", Pow10(3)), ("lbf/in", Scale::StiffnessLbfIn),
];
pub(crate) const PRESSURE_TO_BAR: Table =
    &[("Pa", Pow10(-5)), ("kPa", Pow10(-2)), ("MPa", Pow10(1)), ("bar", Pow10(0)), ("psi", Scale::PressurePsi)];

fn ratio(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

/// `INCH_MM = Fraction("25.4")`.
fn inch_mm() -> Q {
    ratio(254, 10)
}

/// `POUND_FORCE_N = Fraction("0.45359237") * Fraction("9.80665")`.
fn pound_force_n() -> Q {
    ratio(45_359_237, 100_000_000).mul(&ratio(980_665, 100_000))
}

/// `NORMALIZATION[unit]`: the exact factor and its stated definition.
pub(crate) fn normalization(unit: &str) -> (Q, &'static str) {
    let lbf = pound_force_n();
    let inch = inch_mm();
    match unit {
        "in" => (inch, "1 in = 25.4 mm"),
        "lbf" => (lbf, "1 lbf = 0.45359237 kg x 9.80665 m/s^2 = 4.4482216152605 N"),
        "psi" => (lbf.div(&inch.mul(&inch)), "1 psi = 1 lbf/in^2 = 4.4482216152605 N / 645.16 mm^2"),
        "lbf*in" => (lbf.mul(&inch).div(&Q::from_int(1000)), "1 lbf*in = 4.4482216152605 N x 0.0254 m"),
        "lbf*ft" => (lbf.mul_int(12).mul(&inch).div(&Q::from_int(1000)), "1 lbf*ft = 4.4482216152605 N x 0.3048 m"),
        "lbf/in" => (lbf.mul_int(1000).div(&inch), "1 lbf/in = 4.4482216152605 N / 0.0254 m"),
        "1/degF" => (ratio(9, 5), "1/degF = 9/5 per degC"),
        "degF" => (ratio(5, 9), "a degF interval = 5/9 degC"),
        _ => unreachable!("no normalization for {unit}"),
    }
}

/// The exact factor of a rational scaling and the NORMALIZATION key it records, or None for radians.
pub(crate) fn factor(scale: Scale) -> Option<(Q, Option<&'static str>)> {
    match scale {
        Scale::Pow10(exponent) => Some((Q::pow10(exponent), None)),
        Scale::Norm(unit) => Some((normalization(unit).0, Some(unit))),
        Scale::StiffnessLbfIn => Some((normalization("lbf/in").0.div(&Q::from_int(1000)), Some("lbf/in"))),
        Scale::PressurePsi => Some((normalization("psi").0.mul_int(10), Some("psi"))),
        Scale::Rad => None,
    }
}

/// A unit's exact factor as (table, unit, Some((numerator, denominator))), radians as None.
pub type UnitFactor = (&'static str, &'static str, Option<(String, String)>);

/// Every unit table's exact factors, then NORMALIZATION's, for the parity vectors.
#[doc(hidden)]
pub fn unit_factors() -> Vec<UnitFactor> {
    let tables: [(&str, Table); 11] = [
        ("length", LENGTH_TO_MM), ("stress", STRESS_TO_MPA), ("force", FORCE_TO_N), ("moment", MOMENT_TO_NM),
        ("force_per_length", FORCE_PER_LENGTH_TO_N_PER_M), ("expansion", EXPANSION_TO_PER_C),
        ("temperature_interval", TEMPERATURE_INTERVAL_TO_C), ("rotation", ROTATION_TO_DEG),
        ("dimensionless", DIMENSIONLESS), ("stiffness", STIFFNESS_TO_N_PER_MM), ("pressure", PRESSURE_TO_BAR),
    ];
    let mut out = Vec::new();
    for (name, table) in tables {
        for (unit, scale) in table {
            out.push((name, *unit, factor(*scale).map(|(q, _)| (q.numer_ref().to_string(), q.denom_ref().to_string()))));
        }
    }
    for unit in ["in", "lbf", "psi", "lbf*in", "lbf*ft", "lbf/in", "1/degF", "degF"] {
        let q = normalization(unit).0;
        out.push(("normalization", unit, Some((q.numer_ref().to_string(), q.denom_ref().to_string()))));
    }
    out
}

fn lookup(table: Table, unit: &str) -> Option<Scale> {
    table.iter().find(|(key, _)| *key == unit).map(|(_, scale)| *scale)
}

fn table_keys(table: Table) -> String {
    table.iter().map(|(key, _)| *key).collect::<Vec<_>>().join(", ")
}

// ---------------------------------------------------------------------------
// Python helpers
// ---------------------------------------------------------------------------

pub(crate) type R<T> = Result<T, ImportError>;

pub(crate) fn py(error: PyError) -> ImportError {
    ImportError::Python(error)
}

/// `repr(value)`.
pub(crate) fn repr(value: &Value) -> R<String> {
    py_repr(value).map_err(py)
}

/// `str(value)` as an f-string interpolates it.
pub(crate) fn text(value: &Value) -> R<String> {
    py_str(value).map_err(py)
}

/// `_num(value)`: "0" for zero, else `format(value, ".10g")`.
pub(crate) fn num(value: f64) -> String {
    if value == 0.0 {
        return "0".into();
    }
    py_format_g(value, 10)
}

/// `_num(value)` of an exact rational (Python's `Fraction.__format__`).
pub(crate) fn num_q(value: &Q) -> String {
    if value.is_zero() {
        return "0".into();
    }
    py_format_g_q(value, 10)
}

fn overflow_int() -> ImportError {
    py(PyError::overflow("int too large to convert to float"))
}

/// A model int or float as Python's `float(value)` (after `math.isfinite`, which raises for an int beyond binary64).
pub(crate) fn float_of(value: &Value) -> R<f64> {
    match value {
        Value::Int(number) => bigint_to_f64(number).ok_or_else(overflow_int),
        Value::Float(number) => Ok(*number),
        _ => unreachable!("a number"),
    }
}

/// `not isinstance(v, bool) and isinstance(v, (int, float)) and math.isfinite(v)`.
pub(crate) fn finite_number(value: &Value) -> R<bool> {
    match value {
        Value::Int(_) => float_of(value).map(|_| true),
        Value::Float(number) => Ok(number.is_finite()),
        _ => Ok(false),
    }
}

/// `Fraction(value)` of a model int or float.
pub(crate) fn fraction_of(value: &Value) -> Q {
    match value {
        Value::Int(number) => Q::from_bigint(number.clone()),
        Value::Float(number) => Q::from_f64(*number).expect("finite"),
        _ => unreachable!("a number"),
    }
}

/// Raises Python's TypeError where a set or dict lookup hashes an unhashable model value.
pub(crate) fn hashable(value: &Value) -> R<()> {
    match value {
        Value::List(_) => Err(py(PyError::type_error("unhashable type: 'list'"))),
        Value::Dict(_) => Err(py(PyError::type_error("unhashable type: 'dict'"))),
        _ => Ok(()),
    }
}

/// `value in {strings...}` (a set or dict of strings): hashes the value first.
pub(crate) fn in_str_set(value: &Value, set: &[&str]) -> R<bool> {
    hashable(value)?;
    Ok(matches!(value, Value::Str(text) if set.contains(&text.as_str())))
}

/// `_present(value)`: not None, "", [] or {}.
pub(crate) fn present(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Str(text) => !text.is_empty(),
        Value::List(items) => !items.is_empty(),
        Value::Dict(dict) => !dict.is_empty(),
        _ => true,
    }
}

/// `_is_vector(value)`.
pub(crate) fn is_vector(value: &Value) -> R<bool> {
    let Value::Dict(_) = value else { return Ok(false) };
    for axis in ["x", "y", "z"] {
        if !finite_number(value.get_or_null(axis))? {
            return Ok(false);
        }
    }
    Ok(true)
}

/// `_DOF_TOKENS.get(token)`.
pub(crate) fn dof_token(token: &str) -> Option<&'static str> {
    DOFS.iter().copied().find(|dof| {
        let lower = dof.to_lowercase();
        token == *dof || token == lower || token == format!("{}{}", &dof[..1], &lower[1..])
    })
}

/// `_DOF_TOKENS.get(value)` for a model value that `_known` admits (a string key), else None.
pub(crate) fn known_dof(value: &Value) -> Option<&'static str> {
    value.as_str().and_then(dof_token)
}

/// `_sanitize(text, limit)` of `str(text)`.
pub(crate) fn sanitize(text: &str, limit: usize) -> String {
    let cleaned: String = text.chars().filter(|&c| (' '..='~').contains(&c) && c != ',').collect();
    let stripped: String = cleaned.trim_matches(' ').chars().take(limit).collect();
    stripped.trim_matches(' ').to_string()
}

/// `_title(text)`.
pub(crate) fn title(text: &str) -> String {
    let cleaned = sanitize(text, TITLE_MAX);
    if cleaned.is_empty() {
        "SWBPIPE model".into()
    } else {
        cleaned
    }
}

/// `material_origin(material)`.
pub(crate) fn material_origin(material: &Value) -> Option<&'static str> {
    let record = material.get_or_null("provenance_record");
    if !matches!(record, Value::Dict(_)) || !record.get_or_null("library_ref").is_null() {
        return None;
    }
    let certification = record.get_or_null("contributor_certification").as_str();
    if certification == Some(INVENTED_CERTIFICATION)
        && record.get_or_null("redistribution_status").as_str() == Some("public_permissive")
    {
        return Some("project_invented");
    }
    if certification == Some(USER_SUPPLIED_CERTIFICATION) {
        return Some("user_supplied");
    }
    None
}

// ---------------------------------------------------------------------------
// public API
// ---------------------------------------------------------------------------

/// The keyword arguments of `render_caepipe_mbf_si(model, *, title, reference_temperature_c=20.0,
/// max_layout_rows=20, reference_temperature_f=None, piping_code=None)`. The Python takes any value for the title,
/// the reference temperatures and the piping code and refuses (or `str()`s) what it cannot use, so these are
/// [`Value`]s: [`Value::Null`] is Python's None.
#[derive(Clone, Debug)]
pub struct WriterSettings {
    pub title: Value,
    pub reference_temperature_c: Value,
    /// None writes any number of layout rows without the row-limit warning (the product's choice, Q-4).
    pub max_layout_rows: Option<i64>,
    pub reference_temperature_f: Value,
    pub piping_code: Value,
    /// Test knob mirroring a test that replaces `_Writer.survives_reimport` with `lambda self, result: True`.
    #[doc(hidden)]
    pub preflight_disabled: bool,
}

impl WriterSettings {
    /// The Python's defaults: 20 C, at most 20 layout rows, no piping code.
    pub fn new(title: impl Into<String>) -> Self {
        WriterSettings {
            title: Value::Str(title.into()),
            reference_temperature_c: Value::Float(20.0),
            max_layout_rows: Some(20),
            reference_temperature_f: Value::Null,
            piping_code: Value::Null,
            preflight_disabled: false,
        }
    }
}

/// `MbfSiResult`.
#[derive(Clone, Debug)]
pub struct WriterResult {
    pub status: String,
    pub text: Option<String>,
    pub diagnostics: Vec<Value>,
    pub node_map: Dict,
    pub element_map: Dict,
    pub support_map: Dict,
    pub load_case_map: Dict,
    pub material_map: Dict,
    pub section_map: Dict,
    pub load_map: Dict,
    pub pressure_map: Dict,
    pub layout_row_count: i64,
    pub loss_report: Vec<Value>,
    pub assumptions: Vec<String>,
}

pub(crate) fn key_error(key: &str) -> PyError {
    PyError { kind: "KeyError", message: crate::pystr::py_repr_str(key) }
}

fn field<'v>(entry: &'v Value, key: &str) -> Result<&'v Value, PyError> {
    entry.get(key).ok_or_else(|| key_error(key))
}

impl WriterResult {
    fn refused(diagnostics: Vec<Value>, loss_report: Vec<Value>, assumptions: Vec<String>) -> Self {
        WriterResult {
            status: "refused".into(),
            text: None,
            diagnostics,
            node_map: Dict::new(),
            element_map: Dict::new(),
            support_map: Dict::new(),
            load_case_map: Dict::new(),
            material_map: Dict::new(),
            section_map: Dict::new(),
            load_map: Dict::new(),
            pressure_map: Dict::new(),
            layout_row_count: 0,
            loss_report,
            assumptions,
        }
    }

    /// `to_dict()`.
    pub fn to_dict(&self) -> Value {
        dict![
            ("profile_id", Value::str(MBF_SI_PROFILE_ID)),
            ("status", Value::str(self.status.clone())),
            ("text", self.text.clone().map(Value::Str).unwrap_or(Value::Null)),
            ("diagnostics", Value::List(self.diagnostics.clone())),
            ("node_map", Value::Dict(self.node_map.clone())),
            ("element_map", Value::Dict(self.element_map.clone())),
            ("support_map", Value::Dict(self.support_map.clone())),
            ("load_case_map", Value::Dict(self.load_case_map.clone())),
            ("material_map", Value::Dict(self.material_map.clone())),
            ("section_map", Value::Dict(self.section_map.clone())),
            ("load_map", Value::Dict(self.load_map.clone())),
            ("pressure_map", Value::Dict(self.pressure_map.clone())),
            ("layout_row_count", Value::int(self.layout_row_count)),
            ("loss_report", Value::List(self.loss_report.clone())),
            ("assumptions", Value::List(self.assumptions.iter().map(|item| Value::str(item.clone())).collect())),
        ]
    }

    /// `canonical_bytes(to_dict())`: `json.dumps(sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False)` and
    /// a final newline.
    pub fn to_dict_bytes(&self) -> Result<Vec<u8>, PyError> {
        canonical_bytes(&self.to_dict())
    }

    /// `sidecar(mbf_sha256)`: the format-3 writer sidecar of the written text whose SHA-256 is `mbf_sha256`, with
    /// its self-digest `sidecar_sha256`. As in the Python, a refused result has none (KeyError), and a value
    /// without a canonical form raises `SidecarNotCanonical`.
    pub fn sidecar(&self, mbf_sha256: &str) -> Result<Value, PyError> {
        self.sidecar_checked(mbf_sha256).map_err(|error| match error {
            Ok(error) => PyError { kind: "SidecarNotCanonical", message: format!("{}: {}", error.path, error.reason) },
            Err(error) => error,
        })
    }

    /// The sidecar, or the path and reason of the value that has no canonical form (`SidecarNotCanonical`).
    pub(crate) fn sidecar_checked(&self, mbf_sha256: &str) -> Result<Value, Result<NotCanonical, PyError>> {
        let element = |entry: &Value| -> Result<Value, PyError> {
            let mut kept = Dict::new();
            for key in ["from", "to", "reversed"] {
                kept.insert(key.into(), field(entry, key)?.clone());
            }
            if let Some(bend) = entry.get("bend") {
                kept.insert("bend".into(), dict![("component", field(bend, "component")?.clone())]);
            } else {
                for key in ["y_reference", "y_reference_authored"] {
                    kept.insert(key.into(), field(entry, key)?.clone());
                }
            }
            Ok(Value::Dict(kept))
        };
        let subset = |map: &Dict, names: &[&str]| -> Result<Value, PyError> {
            let mut out = Dict::new();
            for (key, entry) in map.iter() {
                let mut kept = Dict::new();
                for name in names {
                    kept.insert((*name).into(), field(entry, name)?.clone());
                }
                out.insert(key.clone(), Value::Dict(kept));
            }
            Ok(Value::Dict(out))
        };
        let body = (|| -> Result<Value, PyError> {
            let mut elements = Dict::new();
            for (key, entry) in self.element_map.iter() {
                elements.insert(key.clone(), element(entry)?);
            }
            let pressure = Value::Dict(self.pressure_map.clone());
            let mut regions = Vec::new();
            let listed = field(&pressure, "contract").and(field(&pressure, "shear_model")).and(field(&pressure, "regions"));
            for region in listed?.as_list().into_iter().flatten() {
                let mut kept = Dict::new();
                for name in ["id", "members", "terminals"] {
                    kept.insert(name.into(), field(region, name)?.clone());
                }
                regions.push(Value::Dict(kept));
            }
            let pressure_map = dict![
                ("contract", field(&pressure, "contract")?.clone()),
                ("shear_model", field(&pressure, "shear_model")?.clone()),
                ("regions", Value::List(regions)),
            ];
            Ok(dict![
                ("sidecar_format", Value::int(SIDECAR_FORMAT)),
                ("node_map", Value::Dict(self.node_map.clone())),
                ("element_map", Value::Dict(elements)),
                ("support_map", subset(&self.support_map, &["node", "restrained", "family", "family_authored"])?),
                ("load_case_map", Value::Dict(self.load_case_map.clone())),
                ("material_map", subset(&self.material_map, &["swbpipe_material", "origin"])?),
                ("load_map", subset(&self.load_map, &["load_case", "category", "target", "direction"])?),
                ("pressure_map", pressure_map),
                ("mbf_sha256", Value::str(mbf_sha256)),
            ])
        })();
        let mut body = body.map_err(Err)?;
        let digest = sidecar_self_digest(&body)?;
        body.as_dict_mut().expect("dict").insert(SIDECAR_DIGEST_KEY.into(), Value::Str(digest));
        Ok(body)
    }

    /// The sidecar file's bytes as `json.dumps(sidecar, indent=2, sort_keys=True) + "\n"` writes them (the
    /// importer's `canonical_bytes` of the sidecar).
    pub fn sidecar_bytes(&self, mbf_sha256: &str) -> Result<Vec<u8>, PyError> {
        canonical_bytes(&self.sidecar(mbf_sha256)?)
    }
}

/// Fails closed on a value the port does not take: the sidecar parser's internal sentinel, which no Python value is,
/// and containers nested beyond [`crate::value::MAX_VALUE_NESTING`], where the Python may still write (an omitted
/// member is never walked) but the port's recursive walks are not followed that deep. The screen does not recurse.
fn representable(value: &Value, name: &str) -> R<()> {
    crate::value::screen(value).map_err(|problem| ImportError::Unsupported(crate::value::unrepresentable(problem, name)))
}

/// `render_caepipe_mbf_si(model, title=..., reference_temperature_c=..., max_layout_rows=...,
/// reference_temperature_f=..., piping_code=...)`.
pub fn render_si(model: &Value, settings: &WriterSettings) -> Result<WriterResult, ImportError> {
    representable(model, "model")?;
    for value in [&settings.title, &settings.reference_temperature_c, &settings.reference_temperature_f, &settings.piping_code] {
        representable(value, "writer settings")?;
    }
    let mut writer = Writer::new(model, settings.title.clone(), settings.reference_temperature_c.clone(), settings.max_layout_rows);
    writer.t_ref_f = settings.reference_temperature_f.clone();
    writer.piping_code = settings.piping_code.clone();
    writer.preflight_disabled = settings.preflight_disabled;
    writer.run()
}

/// `regenerate_caepipe_sidecar(model, mbf_sha256=..., derived_y_reference=..., reference_temperature_c=...,
/// reference_temperature_f=...)`: `(Some(sidecar), [])`, or `(None, the writer's error diagnostics)`.
pub fn regenerate_caepipe_sidecar(
    model: &Value,
    mbf_sha256: &str,
    derived_y_reference: &[String],
    reference: Reference,
) -> Result<(Option<Value>, Vec<Value>), ImportError> {
    representable(model, "model")?;
    let t_ref = match reference {
        Reference::Celsius(value) => Value::Float(value),
        _ => Value::Float(20.0),
    };
    let mut writer = Writer::new(model, Value::str("REGENERATED"), t_ref, None);
    if let Reference::Fahrenheit(value) = reference {
        writer.t_ref_f = Value::Float(value);
    }
    writer.regenerating = Some(derived_y_reference.iter().cloned().collect());
    let result = writer.run()?;
    if result.status != "written" {
        let errors = result
            .diagnostics
            .into_iter()
            .filter(|item| item.get("severity").and_then(Value::as_str) == Some("error"))
            .collect();
        return Ok((None, errors));
    }
    Ok((Some(result.sidecar(mbf_sha256).map_err(py)?), Vec::new()))
}

/// The crate's own SI writer as the importer's regenerator: what a bound format 2 or 3 import uses when
/// [`crate::ImportOptions::regenerator`] is None.
#[derive(Clone, Copy, Debug, Default)]
pub struct SiWriter;

impl SidecarRegenerator for SiWriter {
    fn regenerate(&self, model: &Value, mbf_sha256: &str, derived: &[String], reference: Reference) -> Result<Regenerated, ImportError> {
        let (sidecar, errors) = regenerate_caepipe_sidecar(model, mbf_sha256, derived, reference)?;
        Ok(match sidecar {
            Some(sidecar) => Regenerated::Sidecar(sidecar),
            None => Regenerated::Refused(
                errors.iter().filter_map(|item| item.get("code").and_then(Value::as_str).map(String::from)).collect(),
            ),
        })
    }
}

// ---------------------------------------------------------------------------
// the writer's state
// ---------------------------------------------------------------------------

pub(crate) type V3 = [f64; 3];

pub(crate) struct Material {
    pub e: f64,
    pub nu: f64,
    pub alpha: f64,
    pub has_alpha: bool,
    pub origin: Option<&'static str>,
}

pub(crate) struct Pipe {
    pub from: String,
    pub to: String,
    pub material: String,
    pub y_reference: Value,
    pub od: f64,
    pub thk: f64,
}

pub(crate) struct Support {
    pub node: String,
    pub dofs: BTreeSet<&'static str>,
    pub family: Option<String>,
}

pub(crate) struct Bend {
    pub component: Value,
    pub entity: String,
    pub radius: f64,
    pub k: f64,
    pub angle: Option<f64>,
    pub sifs: Option<[f64; 2]>,
    pub ti: Option<V3>,
    pub theta: Option<f64>,
}

/// A quantity in the MBF unit: rounded once, or the exact rational a sum rounds at its end.
pub(crate) enum Quantity {
    Rounded(f64),
    Exact(Q),
}

pub(crate) struct Writer<'m> {
    pub model: &'m Value,
    pub title: Value,
    pub t_ref_in: Value,
    pub t_ref: f64,
    pub max_layout_rows: Option<i64>,
    pub diagnostics: Vec<Value>,
    pub loss: Vec<Value>,
    pub assumptions: Vec<String>,
    pub length_exp: Scale,
    pub normalized: BTreeMap<String, String>,
    pub t_ref_f: Value,
    pub nodes: OMap<String, V3>,
    pub materials: OMap<String, Material>,
    pub pipes: OMap<String, Pipe>,
    pub node_restraints: BTreeMap<String, BTreeSet<&'static str>>,
    pub node_supports: BTreeMap<String, Vec<String>>,
    pub supports: OMap<String, Support>,
    pub node_springs: OMap<String, OMap<&'static str, f64>>,
    pub case_labels: OMap<String, String>,
    pub forces_exact: OMap<String, [Q; 3]>,
    pub moments_exact: OMap<String, [Q; 3]>,
    pub disp_exact: OMap<String, [Q; 3]>,
    pub rot_exact: OMap<String, [Q; 3]>,
    pub weight_exact: OMap<String, Q>,
    pub delta_t_exact: OMap<String, Q>,
    pub forces: OMap<String, V3>,
    pub moments: OMap<String, V3>,
    pub anchor_disp: OMap<String, V3>,
    pub anchor_rot: OMap<String, V3>,
    pub weight: OMap<String, f64>,
    pub delta_t: OMap<String, f64>,
    pub load_records: OMap<String, Dict>,
    pub bends: OMap<String, Bend>,
    pub tangent_points: OMap<String, String>,
    pub rotation_converted: bool,
    pub contract: Option<Value>,
    pub pipe_pressure: OMap<String, f64>,
    pub regions: Vec<Value>,
    pub regenerating: Option<BTreeSet<String>>,
    pub piping_code: Value,
    pub preflight_disabled: bool,
}

impl<'m> Writer<'m> {
    pub fn new(model: &'m Value, title: Value, t_ref: Value, max_layout_rows: Option<i64>) -> Self {
        Writer {
            model,
            title,
            t_ref_in: t_ref,
            t_ref: 0.0,
            max_layout_rows,
            diagnostics: Vec::new(),
            loss: Vec::new(),
            assumptions: Vec::new(),
            length_exp: Pow10(0),
            normalized: BTreeMap::new(),
            t_ref_f: Value::Null,
            nodes: OMap::new(),
            materials: OMap::new(),
            pipes: OMap::new(),
            node_restraints: BTreeMap::new(),
            node_supports: BTreeMap::new(),
            supports: OMap::new(),
            node_springs: OMap::new(),
            case_labels: OMap::new(),
            forces_exact: OMap::new(),
            moments_exact: OMap::new(),
            disp_exact: OMap::new(),
            rot_exact: OMap::new(),
            weight_exact: OMap::new(),
            delta_t_exact: OMap::new(),
            forces: OMap::new(),
            moments: OMap::new(),
            anchor_disp: OMap::new(),
            anchor_rot: OMap::new(),
            weight: OMap::new(),
            delta_t: OMap::new(),
            load_records: OMap::new(),
            bends: OMap::new(),
            tangent_points: OMap::new(),
            rotation_converted: false,
            contract: None,
            pipe_pressure: OMap::new(),
            regions: Vec::new(),
            regenerating: None,
            piping_code: Value::Null,
            preflight_disabled: false,
        }
    }

    // -- diagnostics -----------------------------------------------------
    pub fn diag(&mut self, code: &str, severity: &str, entity: &str, message: &str) {
        self.diagnostics.push(dict![
            ("code", Value::str(code)),
            ("severity", Value::str(severity)),
            ("entity", Value::str(entity)),
            ("message", Value::str(message)),
        ]);
    }

    pub fn refuse(&mut self, code: &str, entity: &str, message: &str) {
        self.diag(code, "error", entity, message);
        self.loss.push(dict![
            ("entity", Value::str(entity)),
            ("category", Value::str("unsupported")),
            ("code", Value::str(code)),
            ("reason", Value::str(message)),
        ]);
    }

    pub fn omit(&mut self, code: &str, entity: &str, message: &str) {
        self.diag(code, "warning", entity, message);
        self.loss.push(dict![
            ("entity", Value::str(entity)),
            ("category", Value::str("omitted")),
            ("code", Value::str(code)),
            ("reason", Value::str(message)),
        ]);
    }

    pub fn invalid(&mut self, entity: &str, message: &str) {
        self.diag("MBF-SI-INPUT-INVALID", "error", entity, message);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|item| item.get("severity").and_then(Value::as_str) == Some("error"))
    }

    pub fn refused(&self) -> WriterResult {
        WriterResult::refused(self.diagnostics.clone(), self.loss.clone(), self.assumptions.clone())
    }

    // -- value helpers ---------------------------------------------------
    pub fn number(&mut self, value: &Value, entity: &str, name: &str) -> R<Option<f64>> {
        match value {
            Value::Int(_) => Ok(Some(float_of(value)?)),
            Value::Float(number) if number.is_finite() => Ok(Some(*number)),
            Value::Float(_) => {
                let message = format!("{entity} {name} is not finite ({})", repr(value)?);
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &message);
                Ok(None)
            }
            _ => {
                let message = format!("{entity} {name} must be a number (got {})", repr(value)?);
                self.invalid(entity, &message);
                Ok(None)
            }
        }
    }

    /// `quantity(record, table, target, entity, name, exact=...)`.
    pub fn quantity_of(&mut self, record: &Value, table: Table, target: &str, entity: &str, name: &str, exact: bool) -> R<Option<Quantity>> {
        if !matches!(record, Value::Dict(_)) || !record.has("value") || !record.has("unit") {
            self.invalid(entity, &format!("{entity} {name} must be a {{value, unit}} quantity"));
            return Ok(None);
        }
        let unit = record.get_or_null("unit");
        let Some(value) = self.number(record.get_or_null("value"), entity, name)? else { return Ok(None) };
        let scale = unit.as_str().and_then(|unit| lookup(table, unit));
        let Some(scale) = scale else {
            let message = format!(
                "{entity} {name} unit {} is refused: only units with an exact power-of-ten scaling or an exact \
                 definitional normalization to the MBF unit {target} are written ({})",
                repr(unit)?,
                table_keys(table)
            );
            self.refuse("MBF-SI-UNIT-UNSUPPORTED", entity, &message);
            return Ok(None);
        };
        let Some((factor, key)) = factor(scale) else {
            self.rotation_converted = true;
            let degrees = crate::pymath::degrees(value);
            if !degrees.is_finite() || (value != 0.0 && degrees == 0.0) {
                let message = format!("{entity} {name} is not representable in binary64 after conversion to degrees");
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &message);
                return Ok(None);
            }
            return Ok(Some(if exact { Quantity::Exact(Q::from_f64(degrees).expect("finite")) } else { Quantity::Rounded(degrees) }));
        };
        if let Some(key) = key {
            self.normalized.insert(key.into(), normalization(key).1.into());
        }
        let value_exact = Q::from_f64(value).expect("finite").mul(&factor);
        if exact {
            return Ok(Some(Quantity::Exact(value_exact)));
        }
        Ok(self.rounded(&value_exact, entity, name).map(Quantity::Rounded))
    }

    /// `quantity(...)` rounded once to binary64.
    pub fn quantity(&mut self, record: &Value, table: Table, target: &str, entity: &str, name: &str) -> R<Option<f64>> {
        Ok(match self.quantity_of(record, table, target, entity, name, false)? {
            Some(Quantity::Rounded(value)) => Some(value),
            Some(Quantity::Exact(_)) => unreachable!("rounded"),
            None => None,
        })
    }

    /// `quantity(..., exact=True)`.
    pub fn quantity_exact(&mut self, record: &Value, table: Table, target: &str, entity: &str, name: &str) -> R<Option<Q>> {
        Ok(match self.quantity_of(record, table, target, entity, name, true)? {
            Some(Quantity::Exact(value)) => Some(value),
            Some(Quantity::Rounded(_)) => unreachable!("exact"),
            None => None,
        })
    }

    /// `rounded(value, entity, name)`.
    pub fn rounded(&mut self, value: &Q, entity: &str, name: &str) -> Option<f64> {
        let result = value.to_f64().unwrap_or(f64::INFINITY);
        if !result.is_finite() || (!value.is_zero() && result == 0.0) {
            let message = format!("{entity} {name} is not representable in binary64 after conversion to the MBF unit");
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &message);
            return None;
        }
        Some(result)
    }

    /// `finite(values, entity, what)`.
    pub fn finite(&mut self, values: &[f64], entity: &str, what: &str) -> bool {
        if values.iter().all(|value| value.is_finite()) {
            return true;
        }
        self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", entity, &format!("{entity} {what} is not representable in binary64"));
        false
    }

    /// `convert(value, scaling, "in")` of a node coordinate.
    pub fn convert(&mut self, value: f64) -> Option<f64> {
        let (factor, key) = factor(self.length_exp).expect("a rational length scaling");
        if key.is_some() {
            self.normalized.insert("in".into(), normalization("in").1.into());
        }
        self.rounded(&Q::from_f64(value).expect("finite").mul(&factor), "model.nodes", "position")
    }

    /// `is_anchor(node)`.
    pub fn is_anchor(&self, node: &str) -> bool {
        self.node_restraints.get(node).map(|dofs| dofs.len() == 6).unwrap_or(false)
    }

    // -- run -------------------------------------------------------------
    pub fn run(mut self) -> R<WriterResult> {
        if !matches!(self.model, Value::Dict(_)) {
            self.invalid("model", "model must be an object");
            return Ok(self.refused());
        }
        if !self.t_ref_f.is_null() {
            if !finite_number(&self.t_ref_f)? {
                let message = format!("reference temperature must be a finite number (got {})", repr(&self.t_ref_f)?);
                self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", "reference_temperature_f", &message);
                return Ok(self.refused());
            }
            let celsius = fraction_of(&self.t_ref_f).sub(&Q::from_int(32)).mul(&ratio(5, 9));
            self.t_ref_in = Value::Float(celsius.to_f64().map_err(py)?);
            self.normalized.insert("degF absolute".into(), ABSOLUTE_FAHRENHEIT.into());
        }
        if !finite_number(&self.t_ref_in)? {
            let message = format!("reference temperature must be a finite number (got {})", repr(&self.t_ref_in)?);
            self.diag("MBF-SI-VALUE-OUT-OF-RANGE", "error", "reference_temperature_c", &message);
            return Ok(self.refused());
        }
        self.t_ref = float_of(&self.t_ref_in)?;
        self.check_piping_code()?;
        self.check_document()?;
        self.check_project_units()?;
        if self.has_errors() {
            return Ok(self.refused());
        }
        self.read_nodes()?;
        self.read_materials()?;
        self.read_pipes()?;
        self.read_components()?;
        self.read_supports()?;
        self.read_load_cases()?;
        self.read_other_members()?;
        if self.has_errors() {
            return Ok(self.refused());
        }
        self.resolve_bends()?;
        if self.has_errors() {
            return Ok(self.refused());
        }
        let rows = self.layout();
        if self.has_errors() {
            return Ok(self.refused());
        }
        self.render(rows)
    }
}
