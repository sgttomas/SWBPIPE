//! CAEPIPE MBF engine (DEL-17-04 Design `desktop-mbf-export-import.md` §2, slice 3): a byte-faithful Rust port of
//! the MBF importer `core/handoff/caepipe_mbf/importer.py`, for the desktop's native code and its wasm engine.
//!
//! [`import`] reads the declared subset of an MBF into a model document of the current version, an import report
//! (`swbpipe.caepipe_mbf_import_report`) and an id map, with the Python's codes, messages, maps and JSON outputs:
//! [`ImportResult::model_bytes`], [`ImportResult::report_bytes`] and [`ImportResult::id_map_bytes`] are
//! byte-identical to `MbfImportResult.model_bytes()`, `report_bytes()` and `id_map_bytes()` (the committed parity
//! vectors check this both ways). CPython takes acos, asin, atan2, sin and tan from the platform C library, whose
//! last bit differs between platforms; the port evaluates them correctly rounded, the same natively and in wasm, so
//! it is the Python's bytes wherever that library rounds the import's inputs correctly. The crate does no file I/O
//! and depends on no reporting crate.
//!
//! A bound import with a format 2 or 3 sidecar ends with the regeneration identity: the SI writer regenerates the
//! sidecar from the imported model and the given sidecar must equal it (Design `caepipe-mbf-import.md` §14). The
//! writer is not part of this crate yet (slice 2), so the caller supplies it as a [`SidecarRegenerator`]; without
//! one such an import returns [`ImportError::RegeneratorRequired`] rather than a result.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081): a successful import means the listed subset of the
//! file was read; it is no evidence that two programs analyse the model alike.

#![forbid(unsafe_code)]
// The port keeps the Python's negated comparisons (`not x > 0`), which a NaN makes differ from `x <= 0`.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::nonminimal_bool)]

mod decimal90;
mod importer;
mod pyfmt;
mod pyjson;
mod pymath;
mod pystr;
mod rational;
mod runtime;
mod sidecar;
mod unicode_tables;
pub mod value;

pub use pyjson::SidecarJsonError;
pub use pystr::PyError;
pub use value::{py_eq, Dict, OMap, Value};

/// The one `schema_version` the model-document schema admits (`schemas/model.schema.yaml`); a test holds it to the
/// schema file, as si_writer `current_model_schema_version` reads it.
pub const MODEL_SCHEMA_VERSION: &str = "0.8.0";
pub const IMPORT_PROFILE_ID: &str = importer::IMPORT_PROFILE_ID;
pub const REPORT_KIND: &str = importer::REPORT_KIND;
pub const MBF_VERSION: &str = importer::MBF_VERSION;
/// The sidecar formats the importer reads (legacy has no `sidecar_format`).
pub const SIDECAR_FORMATS: [i64; 2] = sidecar::SIDECAR_FORMATS;

/// The writer sidecar as the caller has it.
#[derive(Clone, Debug)]
pub enum SidecarInput {
    /// The sidecar file's bytes, parsed strictly (`parse_sidecar_json`).
    Json(Vec<u8>),
    /// A sidecar the caller has already parsed.
    Parsed(Value),
}

/// The reference temperature the regeneration identity passes to the writer (OPTEMP field 1 in the file's unit).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reference {
    None,
    Celsius(f64),
    Fahrenheit(f64),
}

/// What the writer's `regenerate_caepipe_sidecar` returns.
#[derive(Clone, Debug)]
pub enum Regenerated {
    /// The sidecar the writer records for the model (format 3, with its self-digest).
    Sidecar(Value),
    /// The writer refuses the model: the codes of its error diagnostics.
    Refused(Vec<String>),
}

/// The SI writer's `regenerate_caepipe_sidecar(model, mbf_sha256=..., derived_y_reference=..., reference...)`.
pub trait SidecarRegenerator {
    fn regenerate(&self, model: &Value, mbf_sha256: &str, derived_y_reference: &[String], reference: Reference)
        -> Result<Regenerated, ImportError>;
}

/// The arguments of `import_caepipe_mbf`.
pub struct ImportOptions<'a> {
    pub sidecar: Option<SidecarInput>,
    /// `material_provenance`: as the caller parsed it (a dict to be read).
    pub material_provenance: Option<Value>,
    /// `pressure_closure`: as the caller parsed it.
    pub pressure_closure: Option<Value>,
    /// `shear_model`: any value; only `euler_bernoulli` or `timoshenko_annulus_energy_v1` is admitted.
    pub shear_model: Option<Value>,
    pub rebind_by_node_number: bool,
    pub source_label: String,
    /// The writer for the regeneration identity of a bound format 2 or 3 import.
    pub regenerator: Option<&'a dyn SidecarRegenerator>,
    /// Test knob mirroring `PD1_VERSIONS = ()` (profile decision PD-1 withdrawn: the base rule TBD-17-04-007).
    #[doc(hidden)]
    pub pd1_disabled: bool,
}

impl<'a> ImportOptions<'a> {
    pub fn new(source_label: impl Into<String>) -> Self {
        ImportOptions {
            sidecar: None,
            material_provenance: None,
            pressure_closure: None,
            shear_model: None,
            rebind_by_node_number: false,
            source_label: source_label.into(),
            regenerator: None,
            pd1_disabled: false,
        }
    }
}

/// Why [`import`] returns no result.
#[derive(Clone, Debug, PartialEq)]
pub enum ImportError {
    /// The Python importer raises this exception for the same input (type name and message).
    Python(PyError),
    /// An input the port does not represent exactly; it fails closed instead of approximating the Python.
    Unsupported(String),
    /// A bound import with a format 2 or 3 sidecar reached the regeneration identity without a writer.
    RegeneratorRequired,
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::Python(error) => write!(f, "{}: {}", error.kind, error.message),
            ImportError::Unsupported(reason) => write!(f, "unsupported input: {reason}"),
            ImportError::RegeneratorRequired => write!(f, "the regeneration identity needs the SI writer (SidecarRegenerator)"),
        }
    }
}

impl std::error::Error for ImportError {}

/// `MbfImportResult`.
#[derive(Clone, Debug)]
pub struct ImportResult {
    pub status: String,
    pub model: Option<Value>,
    pub diagnostics: Vec<Value>,
    pub loss_report: Vec<Value>,
    pub assumptions: Vec<String>,
    pub target_options: Dict,
    pub id_map: Value,
    pub source: Value,
    pub summary: Value,
}

impl ImportResult {
    /// `to_dict()`: the import report.
    pub fn to_dict(&self) -> Value {
        dict![
            ("document_kind", Value::str(REPORT_KIND)),
            ("profile_id", Value::str(IMPORT_PROFILE_ID)),
            ("status", Value::str(self.status.clone())),
            ("source", self.source.clone()),
            ("diagnostics", Value::List(self.diagnostics.clone())),
            ("loss_report", Value::List(self.loss_report.clone())),
            ("assumptions", Value::List(self.assumptions.iter().map(|item| Value::str(item.clone())).collect())),
            ("target_options", Value::Dict(self.target_options.clone())),
            ("summary", self.summary.clone()),
        ]
    }

    /// `model_bytes()`.
    pub fn model_bytes(&self) -> Result<Option<Vec<u8>>, PyError> {
        self.model.as_ref().map(canonical_bytes).transpose()
    }

    /// `report_bytes()`.
    pub fn report_bytes(&self) -> Result<Vec<u8>, PyError> {
        canonical_bytes(&self.to_dict())
    }

    /// `id_map_bytes()`.
    pub fn id_map_bytes(&self) -> Result<Vec<u8>, PyError> {
        canonical_bytes(&self.id_map)
    }
}

/// `import_caepipe_mbf(data, sidecar=..., material_provenance=..., pressure_closure=..., shear_model=...,
/// rebind_by_node_number=..., source_label=...)`.
pub fn import(data: &[u8], options: &ImportOptions) -> Result<ImportResult, ImportError> {
    importer::Importer::new(data, options, false).run()
}

/// `documented_orientations(data)`: CAEPIPE's documented local y of each straight, keyed by its (from, to) CAEPIPE
/// names in layout order, or None when the import refuses the file before its positions are read.
#[allow(clippy::type_complexity)]
pub fn documented_orientations(data: &[u8]) -> Result<Option<Vec<((String, String), [f64; 3])>>, ImportError> {
    let options = ImportOptions::new("documented orientations");
    let mut importer = importer::Importer::new(data, &options, true);
    importer.run()?;
    Ok(importer.derived.take())
}

/// `canonical_bytes(value)`: `json.dumps(value, sort_keys=True, indent=2, ensure_ascii=True, allow_nan=False)`
/// and a final newline.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, PyError> {
    pyfmt::canonical_bytes(value)
}

/// `parse_sidecar_json(bytes)`.
pub fn parse_sidecar_json(data: &[u8]) -> Result<Value, SidecarJsonError> {
    pyjson::parse_sidecar_json(data)
}

/// si_writer `sidecar_self_digest(sidecar)`, or the path and reason that leave it without a canonical form.
pub fn sidecar_self_digest(sidecar: &Value) -> Result<String, (String, String)> {
    match sidecar::sidecar_self_digest(sidecar) {
        Ok(digest) => Ok(digest),
        Err(Ok(error)) => Err((error.path, error.reason)),
        Err(Err(error)) => Err(("sidecar".into(), error.message)),
    }
}

/// Python's `repr(float)` (also json's number text); fails closed where the text would not read back.
pub fn python_float_repr(value: f64) -> Result<String, PyError> {
    pyfmt::py_float_repr(value)
}

/// Python's `format(value, ".{precision}g")`.
pub fn python_format_g(value: f64, precision: usize) -> String {
    pyfmt::py_format_g(value, precision)
}

/// The numerical ports the importer is built on, exposed for their parity vectors.
#[doc(hidden)]
pub mod ports {
    pub use crate::decimal90::twice_atan2_nonnegative;
    pub use crate::pymath::{correct_norm3, hypot, sum};
    pub use crate::runtime::{bend_tangents, collinear, frame_axes, kink, local_y_reference};
    /// `json.dumps(value, sort_keys=True)` (separators ", " and ": ").
    pub fn dumps_sorted(value: &crate::Value) -> Result<String, crate::PyError> {
        crate::pyfmt::dumps_sorted(value)
    }
    /// `math.atan2`, `math.acos`, `math.asin`, `math.sin`, `math.tan` as the importer calls them.
    pub use crate::pymath::{acos, asin, atan2, sin, tan};
    /// `float(Fraction(text))` for a decimal numeral (Python's correctly rounded conversion).
    pub fn fraction_to_f64(text: &str) -> Option<f64> {
        match crate::rational::parse_fraction(text).ok()? {
            crate::rational::Parsed::Exact(value) => value.to_f64().ok(),
            crate::rational::Parsed::Huge => None,
        }
    }
}
