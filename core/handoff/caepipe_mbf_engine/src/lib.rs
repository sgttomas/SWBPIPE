//! CAEPIPE MBF engine: byte-faithful Rust ports
//! of the MBF importer `core/handoff/caepipe_mbf/importer.py` and the SI MBF writer
//! `core/handoff/caepipe_mbf/si_writer.py`, for the desktop's native code and its wasm engine.
//!
//! [`import`] reads the declared subset of an MBF into a model document of the current version, an import report
//! (`swbpipe.caepipe_mbf_import_report`) and an id map, with the Python's codes, messages, maps and JSON outputs:
//! [`ImportResult::model_bytes`], [`ImportResult::report_bytes`] and [`ImportResult::id_map_bytes`] are
//! byte-identical to `MbfImportResult.model_bytes()`, `report_bytes()` and `id_map_bytes()` (the committed parity
//! vectors check this both ways). [`render_si`] writes a model document as SI MBF text, or refuses it by name, and
//! its [`WriterResult`] carries the Python's text, diagnostics, maps, loss report and assumptions, its `to_dict()`
//! bytes and its format-6 sidecar ([`WriterResult::sidecar`]). Both twins take acos, asin, atan2, sin and tan
//! correctly rounded, by one algorithm (the Python's `correctly_rounded`, ported in `correctly_rounded.rs`), not from
//! the platform C library, whose last bit differs between platforms; so the bytes agree on every platform, natively
//! and in wasm. Where the port cannot reproduce the Python exactly it fails closed ([`ImportError::Unsupported`])
//! instead of approximating it. The crate does no file I/O and depends on no reporting crate.
//!
//! A bound import with a format 2, 3, 4 or 6 sidecar ends with the regeneration identity: the SI writer regenerates the
//! sidecar from the imported model and the given sidecar must equal it. The
//! crate's own writer ([`SiWriter`], [`regenerate_caepipe_sidecar`]) does this unless the caller supplies another
//! [`SidecarRegenerator`]; and the writer's own preflight (`MBF-SI-PREFLIGHT-REFUSED`) is this crate's importer.
//!
//! Standard claim fence applies (claims taxonomy per DEC-081): a successful import means the listed subset of the
//! file was read, and a written MBF means the listed subset of the model was written; neither is evidence that two
//! programs analyse the model alike.

#![forbid(unsafe_code)]
// The port keeps the Python's negated comparisons (`not x > 0`), which a NaN makes differ from `x <= 0`.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::nonminimal_bool)]

mod correctly_rounded;
mod decimal90;
mod designation;
mod export_review;
mod export_settings;
mod identity_inventory;
mod import_review;
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
mod writer;

pub use export_review::{
    export_review, Review, ReviewAllowanceTooSmall, ReviewFiles, ReviewList, ReviewSummary, EXPORT_OUTPUT_TOO_LARGE,
    MBF_EXPORT_OUTPUT_MAX_BYTES, MBF_REVIEW_MAX_BYTES, REVIEW_DOCUMENT_KIND, REVIEW_FIELD_MAX_BYTES, REVIEW_FORMAT,
    REVIEW_TRUNCATED,
};
pub use export_settings::{
    export_settings_from_json, ExportSettingsError, EXPORT_SETTINGS_INVALID, MAX_EXPORT_SETTINGS_JSON_BYTES, MAX_EXPORT_TITLE_BYTES,
};
pub use designation::{designation, evaluate_designated_source, ReviewDesignation, SourceEvaluation, DESIGNATION_REASONS};
pub use import_review::{
    import_review, import_review_designated, ImportReview, ImportReviewSource, ImportReviewSummary, ReviewDocumentCheck, ReviewModel, ReviewedIdentityFile, ASSUMPTION_UNAVAILABLE,
    IDENTITY_MAPS, IMPORT_REVIEW_DOCUMENT_KIND, IMPORT_REVIEW_FORMAT, IMPORT_REVIEW_TRUNCATED, MODEL_TOO_LARGE, NOT_ADOPTABLE,
};
pub use identity_inventory::{compare_inventories, identity_inventory, ClassInventory, IdentityInventory, InventoryComparison, INVENTORY_CLASSES};
pub use pyjson::{SidecarJsonError, MAX_NESTING as MAX_SIDECAR_NESTING};
pub use pystr::PyError;
pub use value::{py_eq, Dict, OMap, Value};
pub use writer::{regenerate_caepipe_sidecar, render_si, SiWriter, WriterResult, WriterSettings, MBF_SI_PROFILE_ID};

/// The one `schema_version` the model-document schema admits (`schemas/model.schema.yaml`); a test holds it to the
/// schema file, as si_writer `current_model_schema_version` reads it.
pub const MODEL_SCHEMA_VERSION: &str = "0.9.0";
pub const IMPORT_PROFILE_ID: &str = importer::IMPORT_PROFILE_ID;
pub const REPORT_KIND: &str = importer::REPORT_KIND;
pub const MBF_VERSION: &str = importer::MBF_VERSION;
/// The sidecar formats the importer reads (legacy has no `sidecar_format`).
pub const SIDECAR_FORMATS: [i64; 4] = sidecar::SIDECAR_FORMATS;
/// The format the writer records for every export (Format 6).
pub const SIDECAR_FORMAT: i64 = sidecar::SIDECAR_FORMAT;

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
    /// The sidecar the writer records for the model (format 6, with its self-digest, section_map and derivation).
    Sidecar(Value),
    /// The writer refuses the model: the codes of its error diagnostics.
    Refused(Vec<String>),
}

/// The SI writer's `regenerate_caepipe_sidecar(model, mbf_sha256=..., derived_y_reference=..., reference...)`.
/// [`SiWriter`] is the crate's own; a caller may supply another (a test's recorded regenerations, say).
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
    /// The writer for the regeneration identity of a bound format 2, 3, 4 or 6 import; None is the crate's own
    /// ([`SiWriter`]).
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

/// Why [`import`] or [`render_si`] returns no result.
#[derive(Clone, Debug, PartialEq)]
pub enum ImportError {
    /// The Python importer or writer raises this exception for the same input (type name and message).
    Python(PyError),
    /// An input the port does not represent exactly; it fails closed instead of approximating the Python.
    Unsupported(String),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportError::Python(error) => write!(f, "{}: {}", error.kind, error.message),
            ImportError::Unsupported(reason) => write!(f, "unsupported input: {reason}"),
        }
    }
}

impl std::error::Error for ImportError {}

/// One applied mapping assumption as a stable identifier and structured parameters (DEL-17-04 Design
/// `desktop-mbf-export-import.md` §4.3, REQ-015): the desktop review's source-safe counterpart of one entry of
/// [`ImportResult::assumptions`], recorded by the importer at the same emission and under the same condition, in the
/// same order. It is review support only: the assumption strings, the report, the model and the id map are unchanged.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssumptionFact {
    /// One of [`ASSUMPTION_IDS`], or [`ASSUMPTION_ID_UNAVAILABLE`] for an emission the importer has no structured
    /// form of (the review then blocks adoption, `MBF-IMPORT-ASSUMPTION-UNAVAILABLE`).
    pub id: &'static str,
    /// `(name, value)` in a fixed order per identifier; every value is engine data, never a sentence.
    pub parameters: Vec<(&'static str, String)>,
}

/// The identifier of an assumption the importer emitted without a structured form: it blocks adoption.
pub const ASSUMPTION_ID_UNAVAILABLE: &str = "unavailable";

/// Every structured assumption identifier the importer records ([`AssumptionFact::id`]).
pub const ASSUMPTION_IDS: [&str; 16] = importer::ASSUMPTION_IDS;

/// The importer's identity-recovery accounting, counted at
/// the emissions that report it: Python `IdentityRecoveryFacts`. Fixed-size producer facts for the bounded review,
/// not part of `to_dict()`, the report, the model or the id map. Neither count establishes that the original source
/// was completely represented; the exchange never establishes that.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IdentityRecoveryFacts {
    /// Original source ids the import names as not preserved (`MBF-IMPORT-ID-UNRECOVERABLE`), each counted once: today
    /// each bound `load_map` entry the MBF sums with others. The formed group (`MBF-IMPORT-IDENTITIES-MERGED`) is
    /// not an original id and is not counted.
    pub known_missing_ids: u64,
    /// Fields and references the import reports as not recovered, each counted once: a bend component's `.node`
    /// (`MBF-IMPORT-REFERENCE-UNRECOVERABLE`), a legacy support's `.family` (`MBF-IMPORT-FIELD-UNRECOVERABLE`) and a
    /// restored thermal entry's `.direction` (`MBF-IMPORT-DIRECTION-UNRECOVERABLE`).
    pub unrecovered_fields: u64,
}

/// A format-6 sidecar's `derivation.export_settings` (Format 6): the writer inputs
/// outside the model document that can change the MBF bytes, as the writer used them. Python `SidecarExportSettings`.
#[derive(Clone, Debug, PartialEq)]
pub struct SidecarExportSettings {
    /// The printed title line.
    pub title: String,
    pub reference_temperature: f64,
    /// `degC` or `degF`: the argument the writer used.
    pub reference_temperature_unit: &'static str,
    pub max_layout_rows: Option<i64>,
    /// Null, or one of the documented OPTIONS piping-code tokens.
    pub piping_code: Option<String>,
}

/// A bound format-6 sidecar's `derivation` (Format 6), as the sidecar declares it:
/// the SHA-256 of the canonical model document the writer read, and the export settings. Held at the sidecar's
/// source-free trust level for a later check against a designated source document; the import neither verifies nor
/// uses it. Python `SidecarDerivation`.
#[derive(Clone, Debug, PartialEq)]
pub struct SidecarDerivation {
    pub source_model_sha256: String,
    pub export_settings: SidecarExportSettings,
}

/// `MbfImportResult`.
#[derive(Clone, Debug)]
pub struct ImportResult {
    pub status: String,
    pub model: Option<Value>,
    pub diagnostics: Vec<Value>,
    pub loss_report: Vec<Value>,
    pub assumptions: Vec<String>,
    /// The structured counterpart of each entry of `assumptions`, in the same order (not part of `to_dict()`).
    pub assumption_facts: Vec<AssumptionFact>,
    /// Number of emitted materials whose origin was selected from a bound sidecar. Review-only metadata, not
    /// inferred from caller-controlled provenance wording and not part of the serialized model/report/id map.
    pub recorded_material_origins: u64,
    /// The importer's identity-recovery counts, carried on refused results too (not part of `to_dict()`).
    pub identity_recovery: IdentityRecoveryFacts,
    /// A bound format-6 import's sidecar derivation; None for formats 2 to 4, legacy, rebound, unbound and refused
    /// imports (not part of `to_dict()`).
    pub derivation: Option<SidecarDerivation>,
    pub target_options: Dict,
    pub id_map: Value,
    pub source: Value,
    pub summary: Value,
}

/// Fails closed ([`ImportError::Unsupported`]) when `value`, given as `name`, nests deeper than
/// [`value::MAX_VALUE_NESTING`]; checked without recursion, before any recursive walk.
pub(crate) fn bounded(name: &str, value: &Value) -> Result<(), ImportError> {
    if value::within_bound(value) {
        return Ok(());
    }
    Err(ImportError::Unsupported(value::unrepresentable(value::Unrepresentable::TooDeep, name)))
}

impl ImportResult {
    /// Every field a caller may have set is within [`value::MAX_VALUE_NESTING`] (the importer's own results are).
    fn bounded(&self) -> Result<(), ImportError> {
        if let Some(model) = &self.model {
            bounded("import result's model", model)?;
        }
        for item in self.diagnostics.iter().chain(&self.loss_report) {
            bounded("import result", item)?;
        }
        for (name, value) in [("source", &self.source), ("id_map", &self.id_map), ("summary", &self.summary)] {
            bounded(name, value)?;
        }
        self.target_options.values().try_for_each(|value| bounded("target_options", value))
    }

    /// `to_dict()`: the import report. A caller-set field beyond [`value::MAX_VALUE_NESTING`] fails closed.
    pub fn to_dict(&self) -> Result<Value, ImportError> {
        self.bounded()?;
        Ok(self.dict())
    }

    fn dict(&self) -> Value {
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
    pub fn model_bytes(&self) -> Result<Option<Vec<u8>>, ImportError> {
        self.bounded()?;
        self.model.as_ref().map(pyfmt::canonical_bytes).transpose().map_err(ImportError::Python)
    }

    /// `report_bytes()`.
    pub fn report_bytes(&self) -> Result<Vec<u8>, ImportError> {
        self.bounded()?;
        pyfmt::canonical_bytes(&self.dict()).map_err(ImportError::Python)
    }

    /// `id_map_bytes()`.
    pub fn id_map_bytes(&self) -> Result<Vec<u8>, ImportError> {
        self.bounded()?;
        pyfmt::canonical_bytes(&self.id_map).map_err(ImportError::Python)
    }
}

/// `import_caepipe_mbf(data, sidecar=..., material_provenance=..., pressure_closure=..., shear_model=...,
/// rebind_by_node_number=..., source_label=...)`.
///
/// A parsed value in `options` beyond [`value::MAX_VALUE_NESTING`] fails closed
/// ([`ImportError::Unsupported`]) before anything walks it.
pub fn import(data: &[u8], options: &ImportOptions) -> Result<ImportResult, ImportError> {
    let parsed = [
        ("sidecar", match &options.sidecar {
            Some(SidecarInput::Parsed(value)) => Some(value),
            _ => None,
        }),
        ("material_provenance", options.material_provenance.as_ref()),
        ("pressure_closure", options.pressure_closure.as_ref()),
        ("shear_model", options.shear_model.as_ref()),
    ];
    for (name, value) in parsed {
        if let Some(value) = value {
            bounded(name, value)?;
        }
    }
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
/// and a final newline. A value beyond [`value::MAX_VALUE_NESTING`] fails closed ([`ImportError::Unsupported`]); the
/// Python's own exceptions are [`ImportError::Python`].
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>, ImportError> {
    bounded("value", value)?;
    pyfmt::canonical_bytes(value).map_err(ImportError::Python)
}

/// `parse_sidecar_json(bytes)`.
pub fn parse_sidecar_json(data: &[u8]) -> Result<Value, SidecarJsonError> {
    pyjson::parse_sidecar_json(data)
}

/// si_writer `sidecar_self_digest(sidecar)`, or the path and reason that leave it without a canonical form.
/// A sidecar beyond [`value::MAX_VALUE_NESTING`] has no digest here: `("sidecar", reason)`.
pub fn sidecar_self_digest(sidecar: &Value) -> Result<String, (String, String)> {
    if !value::within_bound(sidecar) {
        return Err(("sidecar".into(), value::unrepresentable(value::Unrepresentable::TooDeep, "sidecar")));
    }
    match sidecar::sidecar_self_digest(sidecar) {
        Ok(digest) => Ok(digest),
        Err(Ok(error)) => Err((error.path, error.reason)),
        Err(Err(error)) => Err(("sidecar".into(), error.message)),
    }
}

/// si_writer `model_canonical_bytes(model)` as a SHA-256: the digest a format-6 sidecar records as
/// `derivation.source_model_sha256` for the model document the writer read (Format 6), or the path and
/// reason that leave the document without a canonical form. A document beyond [`value::MAX_VALUE_NESTING`] has none
/// here: `("model", reason)`.
pub fn model_canonical_sha256(model: &Value) -> Result<String, (String, String)> {
    if !value::within_bound(model) {
        return Err(("model".into(), value::unrepresentable(value::Unrepresentable::TooDeep, "model")));
    }
    match sidecar::model_canonical_bytes(model) {
        Ok(bytes) => Ok(sidecar::hex(&<sha2::Sha256 as sha2::Digest>::digest(&bytes))),
        Err(Ok(error)) => Err((error.path, error.reason)),
        Err(Err(error)) => Err(("model".into(), error.message)),
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
    pub fn dumps_sorted(value: &crate::Value) -> Result<String, crate::ImportError> {
        crate::bounded("value", value)?;
        crate::pyfmt::dumps_sorted(value).map_err(crate::ImportError::Python)
    }
    /// `correctly_rounded.atan2`, `.acos`, `.asin`, `.sin`, `.tan` as the importer calls them.
    pub use crate::pymath::{acos, asin, atan2, sin, tan};
    /// si_writer `_num(value)`: "0" for zero, else `format(value, ".10g")`.
    pub fn writer_num(value: f64) -> String {
        crate::writer::num(value)
    }
    /// si_writer `_num(Fraction(numerator, denominator))` (Python's `Fraction.__format__`); None for a zero
    /// denominator.
    pub fn writer_num_fraction(numerator: &str, denominator: &str) -> Option<String> {
        let numerator: num_bigint::BigInt = numerator.parse().ok()?;
        let denominator: num_bigint::BigInt = denominator.parse().ok()?;
        if num_traits::Zero::is_zero(&denominator) {
            return None;
        }
        Some(crate::writer::num_q(&crate::rational::Q::new(numerator, denominator)))
    }
    /// The writer's unit tables and NORMALIZATION as exact factors `(table, unit, Some((numerator, denominator)))`,
    /// radians as None.
    pub use crate::writer::unit_factors as writer_unit_factors;
    /// `math.degrees` and `math.radians`.
    pub use crate::pymath::{degrees, radians};
    /// `float(Fraction(text))` for a decimal numeral (Python's correctly rounded conversion).
    pub fn fraction_to_f64(text: &str) -> Option<f64> {
        match crate::rational::parse_fraction(text).ok()? {
            crate::rational::Parsed::Exact(value) => value.to_f64().ok(),
            crate::rational::Parsed::Huge => None,
        }
    }
}
