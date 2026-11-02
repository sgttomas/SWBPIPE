//! The desktop export dialog's settings as the writer's arguments (DEL-17-04 Design `desktop-mbf-export-import.md`
//! §3.1 and §3.6; slices 4a and its review round): one mapping, read from the settings' JSON text by the engine's own
//! reader ([`crate::parse_sidecar_json`]), so the webview's review (`caepipe_mbf_wasm`) and the native save
//! (`apps/desktop/src-tauri/src/mbf_export.rs`) cannot read a number differently. Both are given the same text: the
//! webview passes the text it reviewed with to the native command.
//!
//! The settings are exactly `{title, referenceTemperature: {value, unit}, maxLayoutRows: null}`:
//! - the title is a string of at most [`MAX_EXPORT_TITLE_BYTES`] bytes of UTF-8, passed as given (the writer trims
//!   it);
//! - `degC` is passed as `reference_temperature_c` and `degF` as `reference_temperature_f` (exact; the Celsius default
//!   is then unused); the value is a JSON number read as Python's `json.loads` reads it, an integer of any size as an
//!   `int` and any other number as a finite `float`;
//! - there is no row limit (`max_layout_rows=None`, Q-4).
//!
//! Anything else is refused by name ([`EXPORT_SETTINGS_INVALID`]). A text longer than
//! [`MAX_EXPORT_SETTINGS_JSON_BYTES`] is refused before it is read, and the reader keeps its own nesting bound and
//! fails closed as it does for a sidecar.

use crate::value::Value;
use crate::writer::WriterSettings;
use crate::SidecarJsonError;

/// The refusal code of settings that are not the dialog's.
pub const EXPORT_SETTINGS_INVALID: &str = "MBF-EXPORT-SETTINGS-INVALID";
/// The most bytes of UTF-8 a title may hold.
pub const MAX_EXPORT_TITLE_BYTES: usize = 4096;
/// The most bytes of settings text read: a title of [`MAX_EXPORT_TITLE_BYTES`] bytes escaped at JSON's worst (six
/// bytes for one, `\u0000`), and the rest of the object with room to spare.
pub const MAX_EXPORT_SETTINGS_JSON_BYTES: usize = 6 * MAX_EXPORT_TITLE_BYTES + 1024;

/// A refusal of the settings: [`EXPORT_SETTINGS_INVALID`] and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportSettingsError {
    pub message: String,
}

fn invalid(detail: impl std::fmt::Display) -> ExportSettingsError {
    ExportSettingsError {
        message: format!(
            "The export settings must be {{title, referenceTemperature: {{value, unit}}, maxLayoutRows: null}}: {detail}."
        ),
    }
}

fn number(value: &Value) -> bool {
    match value {
        Value::Int(_) => true,
        Value::Float(value) => value.is_finite(),
        _ => false,
    }
}

/// The writer's arguments for the dialog's settings text, or why they are refused.
pub fn export_settings_from_json(text: &[u8]) -> Result<WriterSettings, ExportSettingsError> {
    if text.len() > MAX_EXPORT_SETTINGS_JSON_BYTES {
        return Err(invalid(format_args!(
            "the settings text is {} bytes, more than the {MAX_EXPORT_SETTINGS_JSON_BYTES} a title of at most {MAX_EXPORT_TITLE_BYTES} bytes needs",
            text.len()
        )));
    }
    let settings = crate::parse_sidecar_json(text).map_err(|error| {
        invalid(match error {
            SidecarJsonError::Invalid(findings) => {
                format!("the text is not JSON ({})", findings.into_iter().map(|(_, reason)| reason).collect::<Vec<_>>().join("; "))
            }
            SidecarJsonError::LoneSurrogate => "the text holds an unpaired surrogate escape".to_string(),
            SidecarJsonError::TooDeep => format!("the text nests deeper than {} arrays and objects", crate::MAX_SIDECAR_NESTING),
        })
    })?;
    let mapped = map(&settings);
    crate::value::dispose(settings);
    mapped
}

fn map(settings: &Value) -> Result<WriterSettings, ExportSettingsError> {
    let Some(dict) = settings.as_dict() else { return Err(invalid("not an object")) };
    if dict.len() != 3 || !["title", "referenceTemperature", "maxLayoutRows"].iter().all(|key| settings.has(key)) {
        return Err(invalid("the members differ"));
    }
    let Some(title) = settings.get_or_null("title").as_str() else { return Err(invalid("the title is not a string")) };
    if title.len() > MAX_EXPORT_TITLE_BYTES {
        return Err(invalid(format_args!("the title is {} bytes of UTF-8, more than {MAX_EXPORT_TITLE_BYTES}", title.len())));
    }
    if !settings.get_or_null("maxLayoutRows").is_null() {
        return Err(invalid("maxLayoutRows must be null (no row limit)"));
    }
    let reference = settings.get_or_null("referenceTemperature");
    let Some(reference_dict) = reference.as_dict() else { return Err(invalid("referenceTemperature is not an object")) };
    if reference_dict.len() != 2 || !reference.has("value") || !reference.has("unit") {
        return Err(invalid("referenceTemperature must hold exactly value and unit"));
    }
    let value = reference.get_or_null("value");
    if !number(value) {
        return Err(invalid("the reference temperature is not a finite number"));
    }
    let mut writer = WriterSettings::new(title.to_string());
    writer.max_layout_rows = None;
    match reference.get_or_null("unit").as_str() {
        Some("degC") => writer.reference_temperature_c = value.clone(),
        Some("degF") => writer.reference_temperature_f = value.clone(),
        _ => return Err(invalid("the reference temperature unit is not degC or degF")),
    }
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn settings(value: &str, unit: &str) -> String {
        format!(r#"{{"title":"T","referenceTemperature":{{"value":{value},"unit":"{unit}"}},"maxLayoutRows":null}}"#)
    }

    fn int(value: &Value) -> Option<BigInt> {
        match value {
            Value::Int(number) => Some(number.clone()),
            _ => None,
        }
    }

    fn float(value: &Value) -> Option<f64> {
        match value {
            Value::Float(number) => Some(*number),
            _ => None,
        }
    }

    #[test]
    fn the_dialogs_settings_map_to_the_writers_arguments() {
        let writer = export_settings_from_json(settings("20", "degC").as_bytes()).unwrap();
        assert_eq!(writer.title.as_str(), Some("T"));
        assert_eq!(int(&writer.reference_temperature_c), Some(BigInt::from(20)));
        assert!(writer.reference_temperature_f.is_null());
        assert_eq!(writer.max_layout_rows, None);
        let writer = export_settings_from_json(settings("68.5", "degF").as_bytes()).unwrap();
        assert_eq!(float(&writer.reference_temperature_f), Some(68.5));
        assert_eq!(float(&writer.reference_temperature_c), Some(20.0), "the Celsius default, unused");
    }

    /// Integers beyond i64 and u64 are Python ints, exactly, as the text writes them; a float stays a float.
    #[test]
    fn large_integers_are_read_as_the_engines_reader_reads_them() {
        for text in ["9223372036854775809", "18446744073709551615", "18446744073709551617", "100000000000000000000", "999999999999999999999"] {
            let writer = export_settings_from_json(settings(text, "degC").as_bytes()).unwrap();
            assert_eq!(int(&writer.reference_temperature_c), Some(text.parse::<BigInt>().unwrap()), "{text}");
        }
        let writer = export_settings_from_json(settings("1e20", "degC").as_bytes()).unwrap();
        assert_eq!(float(&writer.reference_temperature_c), Some(1e20));
    }

    #[test]
    fn anything_else_is_refused_by_name() {
        let title = |title: &str| format!(r#"{{"title":{title},"referenceTemperature":{{"value":20,"unit":"degC"}},"maxLayoutRows":null}}"#);
        for text in [
            "[]".to_string(),
            "{".to_string(),
            r#"{"title":"T","referenceTemperature":{"value":20,"unit":"degC"}}"#.to_string(),
            r#"{"title":"T","referenceTemperature":{"value":20,"unit":"degC"},"maxLayoutRows":20}"#.to_string(),
            settings("20", "K"),
            settings("\"20\"", "degC"),
            settings("NaN", "degC"),
            settings("Infinity", "degC"),
            settings("1e400", "degC"),
            title("1"),
            title(&format!("\"{}\"", "a".repeat(MAX_EXPORT_TITLE_BYTES + 1))),
            title(&format!("\"{}\"", "é".repeat(MAX_EXPORT_TITLE_BYTES / 2 + 1))),
            title(&format!("\"{}\"", "\\u0041".repeat(MAX_EXPORT_TITLE_BYTES + 1))),
            title("\"\\ud800\""),
            format!("{}{}", "[".repeat(100), "]".repeat(100)),
            format!("{}{}", " ".repeat(MAX_EXPORT_SETTINGS_JSON_BYTES), settings("20", "degC")),
        ] {
            assert!(export_settings_from_json(text.as_bytes()).is_err(), "{}", &text[..text.len().min(120)]);
        }
        // At the bound: a title of exactly 4 KiB, in two-byte characters and escaped.
        assert!(export_settings_from_json(title(&format!("\"{}\"", "é".repeat(MAX_EXPORT_TITLE_BYTES / 2))).as_bytes()).is_ok());
        assert!(export_settings_from_json(title(&format!("\"{}\"", "\\u0001".repeat(MAX_EXPORT_TITLE_BYTES))).as_bytes()).is_ok());
    }
}
