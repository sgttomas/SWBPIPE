//! The native export disposition screens, shared by File > Save Model
//! Document… (W-6a, `model_document_save`), File > Save Handoff… (W-6b,
//! `handoff_package`) and File > Save Report Package… (W-18a slice (b1),
//! `report_package_save`). DEL-08-01 Design `report-package-current-results.md`
//! §2.6, "Sharing with W-6a"; DEL-15-01 Design `canonical-model-export.md`.
//!
//! Two screens, moved here unchanged from their first callers:
//! - the store-resolved screen of W-6a ([`screen_model_document_dispositions`],
//!   [`screen_dispositions_in`]): every `provenance_record`, every
//!   `rule_pack_refs[]` entry and every `library_ref`, with each referenced
//!   stored library and record judged by the library import's own per-kind
//!   rule ([`stored_item_blocking`], pinned by
//!   `disposition_verdicts_match_the_library_import`);
//! - the structural screen of W-6b ([`screen_value`], [`malformed_values`];
//!   for a stored rule pack, whose own top-level `classification` object is
//!   admitted and screened member by member, [`screen_document`] and
//!   [`malformed_dispositions`]): every object at every depth judged by its
//!   own [`SCREENED_MEMBERS`], and disposition members whose value is an array
//!   or an object refused as malformed.
//!
//! Each caller keeps its own codes, paths and messages. The store-resolved
//! screen's paths are dotted member paths (W-6a's) unless a caller asks for
//! JSON pointers ([`PathStyle::Pointer`], the report package's).

use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// The webview screener's code for a blocked disposition (`redactionExportControls.ts`).
pub const PROTECTED_CONTENT_BLOCKED: &str = "PROTECTED_CONTENT_BLOCKED";
/// The store check's code (`check_model_library_refs`, W-13 §3.5).
pub const LIBRARY_REF_UNRESOLVED: &str = crate::MODEL_LIBRARY_REF_UNRESOLVED;
/// The store-resolved screen's code for a screened record its verdict cannot
/// read (W-17 object form; DEL-15-01 Design `canonical-model-export.md`,
/// "Disposition screening"): a `provenance_record`, `rule_pack_refs[]` entry
/// or `library_ref` that is not an object (a positional, array-form record
/// among them), `rule_pack_refs` that is not a list, or a disposition member
/// the verdict reads that is an array or an object. The library import refuses
/// to store such a library with the same code, and library apply refuses to
/// apply such a record (`libraryApply.ts`); this screen still covers a library
/// stored before that rule or written to the store by other means.
pub const DISPOSITION_RECORD_MALFORMED: &str = open_pipe_stress_library_import_document::DISPOSITION_RECORD_MALFORMED;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct DispositionDiagnostic {
    pub code: String,
    /// JSON path of the offending member, or "" for the whole document.
    pub path: String,
    pub message: String,
}

pub(crate) fn diagnostic(code: &str, path: &str, message: impl Into<String>) -> DispositionDiagnostic {
    DispositionDiagnostic { code: code.to_string(), path: path.to_string(), message: message.into() }
}

/// Reads one stored library document: `(project_id, library_kind, library_id)`
/// to the stored `document_json`, `None` when the store holds no such library,
/// `Err` when the store cannot be read.
pub type LibraryLookup<'a> = dyn FnMut(&str, &str, &str) -> Result<Option<Value>, String> + 'a;

/// `truthy_str` of the library import: a non-empty string, else nothing.
pub(crate) fn truthy_str(value: Option<&Value>) -> Option<&str> {
    match value {
        Some(Value::String(text)) if !text.is_empty() => Some(text.as_str()),
        _ => None,
    }
}

/// The library import's status resolution: the item's own field wins when
/// usable, otherwise its `provenance`'s.
pub(crate) fn own_or_provenance<'a>(item: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    truthy_str(item.get(key)).or_else(|| truthy_str(item.get("provenance").and_then(Value::as_object).and_then(|provenance| provenance.get(key))))
}

/// The dispositions the library import quarantines (`protected_suspected`,
/// `quarantined`) or rejects (`rejected`, in either field), as `member value`.
fn blocking_dispositions(item: &Map<String, Value>) -> Vec<String> {
    let redistribution = own_or_provenance(item, "redistribution_status");
    let review = own_or_provenance(item, "review_status");
    let mut found = Vec::new();
    if let Some(status @ ("protected_suspected" | "rejected")) = redistribution {
        found.push(format!("redistribution_status {status}"));
    }
    if let Some(status @ ("quarantined" | "rejected")) = review {
        found.push(format!("review_status {status}"));
    }
    found
}

/// The members [`blocking_dispositions`] reads, as [`unreadable_members`]
/// names them: the item's own two disposition members, its `provenance`, and
/// that provenance's two.
const VERDICT_MEMBERS: [&str; 5] = ["redistribution_status", "review_status", "provenance", "provenance.redistribution_status", "provenance.review_status"];

/// The members [`blocking_dispositions`] reads from `item` that hold a value
/// it cannot read, as a bit per [`VERDICT_MEMBERS`] entry (W-17 object form):
/// a disposition member that is an array or an object (the structural
/// screen's [`malformed_values`] rule), or a `provenance` written as an array
/// (its positional form). The verdict would skip each of them, so a blocking
/// value could sit there unseen. Scalars other than text are not refused:
/// they state no disposition (as the structural screen treats them).
fn unreadable_members(item: &Map<String, Value>) -> u8 {
    let container = |value: Option<&Value>| matches!(value, Some(Value::Array(_) | Value::Object(_)));
    let mut found = 0u8;
    for (bit, key) in [(0, "redistribution_status"), (1, "review_status")] {
        if container(item.get(key)) {
            found |= 1 << bit;
        }
    }
    match item.get("provenance") {
        Some(Value::Array(_)) => found |= 1 << 2,
        Some(Value::Object(provenance)) => {
            for (bit, key) in [(3, "redistribution_status"), (4, "review_status")] {
                if container(provenance.get(key)) {
                    found |= 1 << bit;
                }
            }
        }
        _ => {}
    }
    found
}

/// The [`VERDICT_MEMBERS`] named by `found`, joined.
fn unreadable_names(found: u8) -> String {
    VERDICT_MEMBERS.iter().enumerate().filter(|(bit, _)| found & (1 << bit) != 0).map(|(_, name)| *name).collect::<Vec<_>>().join(", ")
}

/// The positional `provenance` of [`unreadable_members`] over a stored
/// library object or record of `library_kind`, at the objects its verdict
/// ([`stored_item_blocking`]) judges: the item itself, or, for a hanger
/// library, every object in it that carries a `provenance` or a disposition
/// member: [`StoredMembers::Structural`]'s stored check. (Its caller's
/// structural screen, [`malformed_values`], names a stored disposition member
/// that is an array or an object by its pointer in the stored library.)
fn stored_item_unreadable(library_kind: &str, item: &Map<String, Value>) -> u8 {
    const POSITIONAL_PROVENANCE: u8 = 1 << 2;
    if library_kind != "hanger" {
        return unreadable_members(item) & POSITIONAL_PROVENANCE;
    }
    let mut objects = Vec::new();
    collect_objects(item, "$", &mut objects);
    objects
        .iter()
        .filter(|(path, object)| {
            object.contains_key("provenance")
                || (!path.ends_with(".provenance") && ["redistribution_status", "review_status", "privacy_class"].iter().any(|key| object.contains_key(*key)))
        })
        .fold(0, |found, (_, object)| found | (unreadable_members(object) & POSITIONAL_PROVENANCE))
}

/// How the store-resolved screen treats a stored disposition member it cannot
/// read (W-17 object form, #1651 Codex r1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum StoredMembers {
    /// Refuse it by name ([`DISPOSITION_RECORD_MALFORMED`]): a
    /// `redistribution_status` or `review_status` (the members the verdict
    /// reads) holding an array or an object, or a `provenance` written as an
    /// array, at any depth of the referenced stored library object or record.
    /// Save Model Document, the stored-model recovery export and the report
    /// package's model, whose callers screen no stored library structurally.
    #[default]
    Refuse,
    /// Leave such members to the caller's own structural screen of the whole
    /// stored library ([`malformed_values`]; Save Handoff's
    /// `HANDOFF-DISPOSITION-MALFORMED`, named by its pointer in the library).
    /// A positional `provenance` the verdict judges is still refused here.
    Structural,
}

/// The stored members [`stored_item_malformed`] names, by bit.
const STORED_MEMBERS: [&str; 3] = ["redistribution_status", "review_status", "provenance"];

/// [`StoredMembers::Refuse`]'s finding over a stored library object or
/// record, at any depth, as a bit per [`STORED_MEMBERS`] entry.
/// Allocation-free; the depth is the parsed document's, which the JSON
/// parser bounds. Other disposition members (`classification` and the
/// privacy members) are not read by the verdict and follow it
/// (`model_document_save` `b1r1_classification_objects_follow_the_import_verdict`).
fn stored_item_malformed(item: &Map<String, Value>) -> u8 {
    let mut found = 0u8;
    for (key, value) in item {
        let bit = match (key.as_str(), value) {
            ("redistribution_status", Value::Array(_) | Value::Object(_)) => 1,
            ("review_status", Value::Array(_) | Value::Object(_)) => 1 << 1,
            ("provenance", Value::Array(_)) => 1 << 2,
            _ => 0,
        };
        found |= bit | stored_value_malformed(value);
    }
    found
}

fn stored_value_malformed(value: &Value) -> u8 {
    match value {
        Value::Object(object) => stored_item_malformed(object),
        Value::Array(items) => items.iter().fold(0, |found, item| found | stored_value_malformed(item)),
        _ => 0,
    }
}

/// The [`STORED_MEMBERS`] named by `found`, joined.
fn stored_malformed_names(found: u8) -> String {
    STORED_MEMBERS.iter().enumerate().filter(|(bit, _)| found & (1 << bit) != 0).map(|(_, name)| *name).collect::<Vec<_>>().join(", ")
}

/// A JSON value's kind, as a refusal names it.
fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "text",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

/// The library import's disposition verdict for one stored library object or
/// record of `library_kind`, as the blocking `member value` pairs it finds.
/// Material, section and component libraries judge the object itself
/// (`validate_object_disposition`). Hanger libraries
/// (`validate_hanger_import`, `library_import_document/src/lib.rs:797-829`)
/// judge every object in it that carries a `provenance` or a disposition member,
/// and, where an object's own `redistribution_status` or `review_status`
/// differs from its provenance's, the provenance alone as well. Own fields
/// also block where the import would first refuse a missing provenance.
/// Pinned against the import by `disposition_verdicts_match_the_library_import`.
pub(crate) fn stored_item_blocking(library_kind: &str, item: &Map<String, Value>) -> Vec<String> {
    if library_kind != "hanger" {
        return blocking_dispositions(item);
    }
    let mut objects = Vec::new();
    collect_objects(item, "$", &mut objects);
    let mut found = Vec::new();
    for (path, object) in objects {
        let judged = object.contains_key("provenance")
            || (!path.ends_with(".provenance")
                && ["redistribution_status", "review_status", "privacy_class"].iter().any(|key| object.contains_key(*key)));
        if !judged {
            continue;
        }
        found.extend(blocking_dispositions(object));
        if let Some(provenance) = object.get("provenance").and_then(Value::as_object) {
            let conflict = ["redistribution_status", "review_status"]
                .iter()
                .any(|key| object.contains_key(*key) && object.get(*key) != provenance.get(*key));
            if conflict {
                let mut alone = Map::new();
                alone.insert("provenance".to_string(), Value::Object(provenance.clone()));
                found.extend(blocking_dispositions(&alone).into_iter().map(|item| format!("provenance {item}")));
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

fn collect_objects<'a>(object: &'a Map<String, Value>, path: &str, out: &mut Vec<(String, &'a Map<String, Value>)>) {
    out.push((path.to_string(), object));
    for (key, value) in object {
        collect_values(value, &format!("{path}.{key}"), out);
    }
}

fn collect_values<'a>(value: &'a Value, path: &str, out: &mut Vec<(String, &'a Map<String, Value>)>) {
    match value {
        Value::Object(object) => collect_objects(object, path, out),
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                collect_values(item, &format!("{path}[{index}]"), out);
            }
        }
        _ => {}
    }
}

/// A `library_ref` identity member (`library_kind`, `library_id`,
/// `record_id`) as the store's open path normalizes it: Rust `trim`
/// (`open_local_library`), and nothing when blank. The one rule of the save
/// screen and the open-time store check (`check_model_library_refs`).
pub(crate) fn library_identity(value: &str) -> Option<&str> {
    Some(value.trim()).filter(|value| !value.is_empty())
}

/// Every record of a stored `kind` library document whose record id, trimmed,
/// equals `record_id` (already normalized by [`library_identity`]). All of them,
/// never the first: a clear first record never hides a blocked or changed
/// duplicate, and a padded id never escapes. The one rule of the save screen
/// and the open-time store check (`check_model_library_refs`).
pub(crate) fn matching_library_records<'a>(kind: &str, stored: &'a Value, record_id: &str) -> Vec<&'a Map<String, Value>> {
    // The store's kind table (`library_records_key`).
    let Some((records_key, id_key)) = crate::library_records_key(kind) else {
        return Vec::new();
    };
    stored
        .get(records_key)
        .and_then(Value::as_array)
        .map(|records| {
            records
                .iter()
                .filter_map(Value::as_object)
                .filter(|record| record.get(id_key).and_then(Value::as_str).map(str::trim) == Some(record_id))
                .collect()
        })
        .unwrap_or_default()
}

/// `parts`, concatenated into one buffer sized before it is written: a
/// message naming a long record or path holds that text once (route 010's
/// bounded screen, Codex round 1 at #1682, P1; the text is `format!`'s).
fn concat(parts: &[&str]) -> String {
    let mut text = String::with_capacity(parts.iter().map(|part| part.len()).sum());
    for part in parts {
        text.push_str(part);
    }
    text
}

fn join(path: &str, key: &str) -> String {
    if path.is_empty() { key.to_string() } else { format!("{path}.{key}") }
}

/// How the store-resolved screen names a member: W-6a's dotted member paths
/// (`materials[0].provenance_record`), or RFC 6901 JSON pointers
/// (`/materials/0/provenance_record`, the report package's).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum PathStyle {
    #[default]
    Dotted,
    Pointer,
}

impl PathStyle {
    fn member(self, path: &str, key: &str) -> String {
        match self {
            PathStyle::Dotted => join(path, key),
            PathStyle::Pointer => format!("{path}/{}", pointer_token(key)),
        }
    }

    fn index(self, path: &str, index: usize) -> String {
        match self {
            PathStyle::Dotted => format!("{path}[{index}]"),
            PathStyle::Pointer => format!("{path}/{index}"),
        }
    }
}

#[derive(Default)]
struct DispositionScreen {
    blocking: Vec<DispositionDiagnostic>,
    /// `library_ref`s by `(kind, id)`: each reference's path and record id.
    library_refs: BTreeMap<(String, String), Vec<(String, Option<String>)>>,
    style: PathStyle,
    stored_members: StoredMembers,
}

impl DispositionScreen {
    /// A record whose verdict members are unreadable is refused by name
    /// ([`DISPOSITION_RECORD_MALFORMED`]) instead of judged: one diagnostic
    /// per record either way.
    fn screen(&mut self, item: &Map<String, Value>, path: &str, record: &str, what: &str) {
        let unreadable = unreadable_members(item);
        if unreadable != 0 {
            return self.unreadable(&unreadable_names(unreadable), path, record, what);
        }
        let found = blocking_dispositions(item);
        self.block(found, path, record, what);
    }

    fn screen_stored(&mut self, library_kind: &str, item: &Map<String, Value>, path: &str, record: &str, what: &str) {
        let unreadable = match self.stored_members {
            StoredMembers::Refuse => stored_malformed_names(stored_item_malformed(item)),
            StoredMembers::Structural => unreadable_names(stored_item_unreadable(library_kind, item)),
        };
        if !unreadable.is_empty() {
            return self.unreadable_stored(&unreadable, path, record, what);
        }
        let found = stored_item_blocking(library_kind, item);
        self.block(found, path, record, what);
    }

    fn unreadable(&mut self, members: &str, path: &str, record: &str, what: &str) {
        self.blocking.push(diagnostic(
            DISPOSITION_RECORD_MALFORMED,
            path,
            concat(&[
                record,
                " (",
                what,
                ": ",
                members,
                ") holds an array or an object where its disposition is read, so the disposition cannot be judged; correct the record first.",
            ]),
        ));
    }

    /// [`Self::unreadable`] for a stored library or record (`kind:id`, or
    /// `kind:id#record`): the message says how the user repairs the store.
    fn unreadable_stored(&mut self, members: &str, path: &str, record: &str, what: &str) {
        self.blocking.push(diagnostic(
            DISPOSITION_RECORD_MALFORMED,
            path,
            format!(
                "{record} ({what}: {members}) holds an array or an object where its disposition is read, so the disposition cannot be judged. \
                 Open the library in the Library Manager, write each redistribution_status and review_status as text and each provenance as an object, \
                 and save it to the local store again (or delete the library, or remove the reference), then retry."
            ),
        ));
    }

    /// A screened record that is not an object (`member` names it): refused
    /// by name, never skipped.
    fn not_an_object(&mut self, path: &str, member: &str, kind: &str) {
        self.blocking.push(diagnostic(
            DISPOSITION_RECORD_MALFORMED,
            path,
            concat(&[
                "`",
                member,
                "` is ",
                kind,
                ", not an object (a record written as an array is in positional form), so its disposition cannot be judged; correct the record first.",
            ]),
        ));
    }

    fn block(&mut self, found: Vec<String>, path: &str, record: &str, what: &str) {
        if !found.is_empty() {
            let found = found.join(", ");
            self.blocking.push(diagnostic(
                PROTECTED_CONTENT_BLOCKED,
                path,
                concat(&[record, " (", what, ": ", &found, ") blocks the save; request human review of the record's disposition first."]),
            ));
        }
    }

    /// Every `provenance_record` (at any depth, including inside another),
    /// every `rule_pack_refs[]` entry and every `library_ref`: the one walk
    /// ([`walk_document`]). Each that is not an object is refused by name.
    fn walk(&mut self, value: &Value, path: &str) {
        let style = self.style;
        walk_document(value, path, style, &mut |site, at| match site {
            DocumentSite::Provenance { record, name } => self.screen(record, at, &name, "provenance_record"),
            DocumentSite::RulePackRef { entry, name } => self.screen(entry, at, &name, "rule_pack_ref"),
            DocumentSite::NotAnObject { member, value } => self.not_an_object(at, member, value_kind(value)),
            DocumentSite::LibraryRef(reference) => {
                if let Some((kind, id, record_id)) = library_ref_identity(reference) {
                    self.library_refs.entry((kind.to_string(), id.to_string())).or_default().push((at.to_string(), record_id.map(str::to_string)));
                }
            }
        });
    }
}

/// A site of the model document's one walk.
enum DocumentSite<'a> {
    /// A `provenance_record` object, named by its owner's `id`, else the
    /// enclosing object's path (`$` at the root).
    Provenance { record: &'a Map<String, Value>, name: String },
    /// A `rule_pack_refs[]` object entry, named by its `rule_pack_id`, else its path.
    RulePackRef { entry: &'a Map<String, Value>, name: String },
    /// A `library_ref` object, at any depth and under any owner.
    LibraryRef(&'a Map<String, Value>),
    /// A `provenance_record`, `rule_pack_refs` or `rule_pack_refs[]` entry,
    /// or `library_ref` that is not an object (`rule_pack_refs`: not a list),
    /// a positional record among them (W-17 object form).
    NotAnObject { member: &'static str, value: &'a Value },
}

/// The one walk of a model document (the native save's screen, and the
/// recorded run revision's `library_refs`), pinned for `library_ref`s to
/// `fixtures/model_document/library_ref_sites.json` as the webview's
/// `libraryRefSites` is. Each site is visited with its path before the walk
/// descends into it.
fn walk_document<'a>(value: &'a Value, path: &str, style: PathStyle, visit: &mut dyn FnMut(DocumentSite<'a>, &str)) {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                walk_document(item, &style.index(path, index), style, visit);
            }
        }
        Value::Object(object) => {
            let owner = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty());
            for (key, child) in object {
                let at = style.member(path, key);
                match (key.as_str(), child) {
                    ("provenance_record", Value::Object(record)) => {
                        let name = owner.map(str::to_string).unwrap_or_else(|| if path.is_empty() { "$".to_string() } else { path.to_string() });
                        visit(DocumentSite::Provenance { record, name }, &at);
                    }
                    ("provenance_record", other) => visit(DocumentSite::NotAnObject { member: "provenance_record", value: other }, &at),
                    ("rule_pack_refs", Value::Array(refs)) => {
                        for (index, entry) in refs.iter().enumerate() {
                            let entry_at = style.index(&at, index);
                            match entry.as_object() {
                                Some(entry) => {
                                    let name = entry.get("rule_pack_id").and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or(&entry_at).to_string();
                                    visit(DocumentSite::RulePackRef { entry, name }, &entry_at);
                                }
                                None => visit(DocumentSite::NotAnObject { member: "rule_pack_refs[]", value: entry }, &entry_at),
                            }
                        }
                    }
                    ("rule_pack_refs", other) => visit(DocumentSite::NotAnObject { member: "rule_pack_refs", value: other }, &at),
                    ("library_ref", Value::Object(reference)) => visit(DocumentSite::LibraryRef(reference), &at),
                    ("library_ref", other) => visit(DocumentSite::NotAnObject { member: "library_ref", value: other }, &at),
                    _ => {}
                }
                walk_document(child, &at, style, visit);
            }
        }
        _ => {}
    }
}

/// A `library_ref`'s identity `(kind, id, record id)` as the store's open path
/// normalizes it (`open_local_library` trims; [`library_identity`]): `None`
/// when the kind or id is not text or is blank, and no record id when it is
/// not text or is blank. The site table's `identity`.
pub(crate) fn library_ref_identity(reference: &Map<String, Value>) -> Option<(&str, &str, Option<&str>)> {
    let text = |key: &str| truthy_str(reference.get(key)).and_then(library_identity);
    Some((text("library_kind")?, text("library_id")?, text("record_id")))
}

/// Every `library_ref` object of a model document, with its path, in the
/// walk's order (arrays in order, object keys sorted): the one walk's
/// `library_ref` sites.
pub(crate) fn library_ref_sites(document: &Value) -> Vec<(String, &Map<String, Value>)> {
    let mut sites = Vec::new();
    walk_document(document, "", PathStyle::Dotted, &mut |site, at| {
        if let DocumentSite::LibraryRef(reference) = site {
            sites.push((at.to_string(), reference));
        }
    });
    sites
}

/// One member named `library_ref` of a model document, whatever its value
/// ([`library_ref_member_sites`]).
pub(crate) struct LibraryRefMember<'a> {
    /// The member's JSON pointer.
    pub(crate) pointer: String,
    /// The reference when the member is an object; `None` for `null`, a
    /// string, a number, a boolean or an array (a positional record among them).
    pub(crate) reference: Option<&'a Map<String, Value>>,
}

/// Every member named `library_ref` of a model document, in the one walk's
/// order, whatever its value: each `DocumentSite::LibraryRef` and each
/// `DocumentSite::NotAnObject { member: "library_ref", .. }` (W-18b library
/// rule 1, DEL-08-01 Design `report-user-release.md` §2.7). It is its own
/// detector: [`library_ref_sites`] keeps the objects only, unchanged (Record
/// Run's revision and `fixtures/model_document/library_ref_sites.json` keep
/// their meaning). Pinned with the webview's `libraryRefMemberSites` to
/// `fixtures/model_document/library_ref_member_sites.json`. It collects every
/// site and builds each one's path, so admission does not call it: it is the
/// oracle of [`first_library_ref_member`] and [`holds_library_ref_member`],
/// which stop at the first site and build no path per level.
#[cfg(test)]
pub(crate) fn library_ref_member_sites(document: &Value) -> Vec<LibraryRefMember<'_>> {
    let mut sites = Vec::new();
    walk_document(document, "", PathStyle::Pointer, &mut |site, at| match site {
        DocumentSite::LibraryRef(reference) => sites.push(LibraryRefMember { pointer: at.to_string(), reference: Some(reference) }),
        DocumentSite::NotAnObject { member: "library_ref", .. } => sites.push(LibraryRefMember { pointer: at.to_string(), reference: None }),
        _ => {}
    });
    sites
}

/// The JSON pointer of every `rule_pack_refs[]` entry the one walk visits,
/// object or not, and of every `rule_pack_refs` member that is not a list
/// (W-18b's rule-pack rule, DEL-08-01 Design `report-user-release.md` §2.7):
/// the oracle of [`first_rule_pack_ref_member_site`], which admission calls.
#[cfg(test)]
pub(crate) fn rule_pack_ref_member_sites(document: &Value) -> Vec<String> {
    let mut sites = Vec::new();
    walk_document(document, "", PathStyle::Pointer, &mut |site, at| match site {
        DocumentSite::RulePackRef { .. } | DocumentSite::NotAnObject { member: "rule_pack_refs[]" | "rule_pack_refs", .. } => sites.push(at.to_string()),
        _ => {}
    });
    sites
}

/// One step of a path inside a value: an object member's key or an array
/// index, borrowed from the value.
#[derive(Clone, Copy, Debug)]
pub(crate) enum PathStep<'a> {
    Key(&'a str),
    Index(usize),
}

/// A path held on a walk's own stack frames: each level borrows its parent
/// and its key, so a walk allocates nothing per level, whatever the depth or
/// the keys' length (W-18b slice (r2b), Codex round 1 P1: a walk that built
/// each level's path text exceeded rule 3 (b)'s lease). The pointer text is
/// built only for a site that is reported ([`StackPath::pointer`]).
#[derive(Clone, Copy)]
pub(crate) struct StackPath<'p, 'a> {
    parent: Option<&'p StackPath<'p, 'a>>,
    step: PathStep<'a>,
}

impl<'p, 'a> StackPath<'p, 'a> {
    pub(crate) fn new(parent: Option<&'p StackPath<'p, 'a>>, step: PathStep<'a>) -> Self {
        Self { parent, step }
    }

    /// The path's JSON pointer (RFC 6901, [`pointer_token`]'s escapes), built
    /// in one allocation of its exact length.
    pub(crate) fn pointer(&self) -> String {
        self.pointer_under("")
    }
}

/// The JSON pointer of `path`; `""` at the root.
pub(crate) fn stack_pointer(path: Option<&StackPath<'_, '_>>) -> String {
    path.map_or_else(String::new, StackPath::pointer)
}

/// The first object member, in the one walk's order ([`walk_document`]'s:
/// arrays in order, object members in key order, each member before what it
/// holds), for which `hit` returns a value. It stops there and allocates
/// nothing itself: the path is a [`StackPath`].
pub(crate) fn first_member_hit<'a, T>(
    value: &'a Value,
    up: Option<&StackPath<'_, 'a>>,
    hit: &mut dyn FnMut(&'a str, &'a Value, &StackPath<'_, 'a>) -> Option<T>,
) -> Option<T> {
    match value {
        Value::Array(items) => items.iter().enumerate().find_map(|(index, item)| first_member_hit(item, Some(&StackPath::new(up, PathStep::Index(index))), hit)),
        Value::Object(object) => object.iter().find_map(|(key, child)| {
            let here = StackPath::new(up, PathStep::Key(key));
            hit(key, child, &here).or_else(|| first_member_hit(child, Some(&here), hit))
        }),
        _ => None,
    }
}

/// The first member named `library_ref`, whatever its value, in the one
/// walk's order: library rule 1's detector as admission runs it (the first of
/// [`library_ref_member_sites`], pinned by
/// `first_member_detectors_match_the_collecting_walks`). Only its pointer is
/// built.
pub(crate) fn first_library_ref_member(document: &Value) -> Option<LibraryRefMember<'_>> {
    first_member_hit(document, None, &mut |key, child, at| (key == "library_ref").then(|| LibraryRefMember { pointer: at.pointer(), reference: child.as_object() }))
}

/// Whether any member is named `library_ref`, whatever its value (rule 3
/// (b) through rule 1, inside a parsed embedded value): it allocates nothing.
pub(crate) fn holds_library_ref_member(document: &Value) -> bool {
    first_member_hit(document, None, &mut |key, _, _| (key == "library_ref").then_some(())).is_some()
}

/// The first of [`rule_pack_ref_member_sites`], found the same way as
/// [`first_library_ref_member`]: a non-empty `rule_pack_refs` list's first
/// entry, or a `rule_pack_refs` member that is not a list.
pub(crate) fn first_rule_pack_ref_member_site(document: &Value) -> Option<String> {
    first_member_hit(document, None, &mut |key, child, at| match (key, child) {
        ("rule_pack_refs", Value::Array(refs)) if refs.is_empty() => None,
        ("rule_pack_refs", Value::Array(_)) => Some(StackPath::new(Some(at), PathStep::Index(0)).pointer()),
        ("rule_pack_refs", _) => Some(at.pointer()),
        _ => None,
    })
}

// --- Route 010's bounded screen (W-18b; Codex round 1 at #1682, P1) ---------
//
// Route `DREP-PACKAGE-USER-RELEASE-010` screens the same values with the same
// rules as the screens above, in their order, but stops at the first finding
// and builds only that finding's pointer: its walks allocate nothing per level
// and nothing per finding, so the refusal's memory is bounded by one pointer,
// which the route's ingress lease prices (DEL-08-01 Design
// `report-user-release.md` §2.5, "Refusals on route 010"). Every other caller
// keeps the collecting screens unchanged.

/// The first finding's text only, built at its exact length as
/// [`FindingTexts`] builds every one.
struct FirstFindingText(Option<String>);

impl WalkFindings<String> for FirstFindingText {
    fn add(&mut self, pointer: &String, member: Option<&str>, middle: &str, tail: &str) {
        if self.0.is_none() {
            self.0 = Some(finding_text(pointer, member, middle, tail));
        }
    }
}

/// Which structural screen found a value's first finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StructuralFinding {
    /// [`screen_value`]'s: `<pointer> <status>`.
    Blocking,
    /// [`malformed_values`]'s: `<pointer> is <kind>`.
    Malformed,
}

/// The first finding of [`screen_value`] over `value`, else the first of
/// [`malformed_values`], in their text: the first entry of each collecting
/// screen (pinned by `first_findings_match_the_collecting_screens`). The
/// walks' one pointer buffer is sized first, by the allocation-free count
/// ([`structural_bound`]), so it never grows; only the first finding is built.
pub(crate) fn first_structural_finding(value: &Value) -> Option<(StructuralFinding, String)> {
    let pointer_max = usize::try_from(structural_bound(value).pointer_max).unwrap_or(usize::MAX);
    for (kind, walk) in [
        (StructuralFinding::Blocking, blocking_walk::<String, FirstFindingText> as fn(&Value, &mut String, ClassificationExemption, &mut FirstFindingText)),
        (StructuralFinding::Malformed, malformed_walk::<String, FirstFindingText>),
    ] {
        let mut pointer = String::with_capacity(pointer_max);
        let mut found = FirstFindingText(None);
        walk(value, &mut pointer, ClassificationExemption::None, &mut found);
        drop(pointer);
        if let Some(text) = found.0 {
            return Some((kind, text));
        }
    }
    None
}

impl StackPath<'_, '_> {
    /// [`StackPath::pointer`] under `root`, in one allocation of its exact
    /// length.
    pub(crate) fn pointer_under(&self, root: &str) -> String {
        fn length(path: &StackPath<'_, '_>) -> usize {
            let own = 1 + match path.step {
                PathStep::Key(key) => pointer_token_len(key),
                PathStep::Index(index) => index.checked_ilog10().map_or(1, |digits| digits as usize + 1),
            };
            own + path.parent.map_or(0, length)
        }
        fn write(path: &StackPath<'_, '_>, out: &mut String) {
            if let Some(parent) = path.parent {
                write(parent, out);
            }
            out.push('/');
            match path.step {
                PathStep::Key(key) => push_pointer_token(out, key),
                PathStep::Index(index) => {
                    let _ = std::fmt::Write::write_fmt(out, format_args!("{index}"));
                }
            }
        }
        let mut out = String::with_capacity(root.len() + length(self));
        out.push_str(root);
        write(self, &mut out);
        out
    }
}

/// The first blocking finding of the store-resolved screen's own sites
/// ([`screen_dispositions_in`] over `value` under `root`, with JSON pointers,
/// before any library is resolved): each `provenance_record`, each
/// `rule_pack_refs[]` entry, and each of those or a `library_ref` that is not
/// an object, in the one walk's order ([`walk_document`]). The diagnostic is
/// the collecting screen's, word for word (pinned by
/// `first_findings_match_the_collecting_screens`). The walk holds its path on
/// its own stack frames; a site's pointer is built only when the site is
/// screened, and dropped before the next.
pub(crate) fn first_document_site_finding(value: &Value, root: &str) -> Option<DispositionDiagnostic> {
    fn walk<'a>(value: &'a Value, up: Option<&StackPath<'_, 'a>>, root: &str, screen: &mut DispositionScreen) -> bool {
        match value {
            Value::Array(items) => items.iter().enumerate().any(|(index, item)| walk(item, Some(&StackPath::new(up, PathStep::Index(index))), root, screen)),
            Value::Object(object) => {
                let owner = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty());
                for (key, child) in object {
                    let here = StackPath::new(up, PathStep::Key(key));
                    match (key.as_str(), child) {
                        ("provenance_record", Value::Object(record)) => {
                            let at = here.pointer_under(root);
                            // The enclosing object's path is `at` without its last token.
                            let enclosing = &at[..at.len() - "/provenance_record".len()];
                            let name = owner.unwrap_or(if enclosing.is_empty() { "$" } else { enclosing });
                            screen.screen(record, &at, name, "provenance_record");
                        }
                        ("provenance_record", other) => screen.not_an_object(&here.pointer_under(root), "provenance_record", value_kind(other)),
                        ("rule_pack_refs", Value::Array(refs)) => {
                            for (index, entry) in refs.iter().enumerate() {
                                let entry_at = StackPath::new(Some(&here), PathStep::Index(index)).pointer_under(root);
                                match entry.as_object() {
                                    Some(entry) => {
                                        let name = entry.get("rule_pack_id").and_then(Value::as_str).filter(|id| !id.is_empty()).unwrap_or(&entry_at);
                                        screen.screen(entry, &entry_at, name, "rule_pack_ref");
                                    }
                                    None => screen.not_an_object(&entry_at, "rule_pack_refs[]", value_kind(entry)),
                                }
                                if !screen.blocking.is_empty() {
                                    return true;
                                }
                            }
                        }
                        ("rule_pack_refs", other) => screen.not_an_object(&here.pointer_under(root), "rule_pack_refs", value_kind(other)),
                        // An object `library_ref` is resolved in the store by the collecting screen; route 010 refuses it instead ([`first_library_ref_object`]).
                        ("library_ref", Value::Object(_)) => {}
                        ("library_ref", other) => screen.not_an_object(&here.pointer_under(root), "library_ref", value_kind(other)),
                        _ => {}
                    }
                    if !screen.blocking.is_empty() || walk(child, Some(&here), root, screen) {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
    let mut screen = DispositionScreen { style: PathStyle::Pointer, stored_members: StoredMembers::Refuse, ..DispositionScreen::default() };
    walk(value, None, root, &mut screen);
    screen.blocking.into_iter().next()
}

/// The JSON pointer of the first `library_ref` member whose value is an
/// object, in the one walk's order: the sites the collecting screen resolves
/// in the local library store. Only its pointer is built.
pub(crate) fn first_library_ref_object(document: &Value) -> Option<String> {
    first_member_hit(document, None, &mut |key, child, at| (key == "library_ref" && child.is_object()).then(|| at.pointer()))
}

/// W-6a's store-resolved screen (`model_document_save`
/// `screen_model_document_dispositions`) over any parsed value (W-18 slice
/// (b1): the report package's request and model): paths start at `root` in
/// `style`, every `library_ref` is resolved in `project_id`'s store (trimmed),
/// and a store read error refuses with the caller's `store_unavailable` code.
///
/// The lookup returns each library owned or shared (`V`, for example an
/// `Rc<Value>` a caller also keeps); the screen only borrows it.
pub(crate) fn screen_dispositions_in<V: std::borrow::Borrow<Value>>(
    value: &Value,
    root: &str,
    style: PathStyle,
    project_id: &str,
    store_unavailable: &str,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<Vec<DispositionDiagnostic>, Vec<DispositionDiagnostic>> {
    screen_dispositions_with(value, root, style, project_id, store_unavailable, StoredMembers::Refuse, lookup)
}

/// [`screen_dispositions_in`] with the caller's [`StoredMembers`] policy.
pub(crate) fn screen_dispositions_with<V: std::borrow::Borrow<Value>>(
    value: &Value,
    root: &str,
    style: PathStyle,
    project_id: &str,
    store_unavailable: &str,
    stored_members: StoredMembers,
    lookup: &mut dyn FnMut(&str, &str, &str) -> Result<Option<V>, String>,
) -> Result<Vec<DispositionDiagnostic>, Vec<DispositionDiagnostic>> {
    let mut screen = DispositionScreen { style, stored_members, ..DispositionScreen::default() };
    screen.walk(value, root);
    let project_id = project_id.trim().to_string();
    let mut warnings = Vec::new();
    for ((kind, id), references) in std::mem::take(&mut screen.library_refs) {
        let named = format!("{kind}:{id}");
        let stored = match lookup(&project_id, &kind, &id) {
            Ok(stored) => stored,
            Err(error) => {
                return Err(vec![diagnostic(
                    store_unavailable,
                    &references[0].0,
                    format!("The local library store could not be read to resolve {named} ({error}); nothing was saved."),
                )]);
            }
        };
        let Some(stored) = stored else {
            for (path, _) in &references {
                warnings.push(diagnostic(LIBRARY_REF_UNRESOLVED, path, format!("The local library store holds no {kind} library `{id}`; its disposition is not known here.")));
            }
            continue;
        };
        let stored: &Value = stored.borrow();
        let library_member = format!("{kind}_library");
        match stored.get(&library_member) {
            Some(Value::Object(library)) => screen.screen_stored(&kind, library, &references[0].0, &named, "stored library"),
            Some(other) => screen.not_an_object(&references[0].0, &format!("{named} {library_member}"), value_kind(other)),
            None => {}
        }
        let mut screened = Vec::new();
        for (path, record_id) in &references {
            let Some(record_id) = record_id else { continue };
            if screened.contains(record_id) {
                continue;
            }
            screened.push(record_id.clone());
            // Every record with this id is screened (`matching_library_records`).
            let matching = matching_library_records(&kind, stored, record_id);
            for record in &matching {
                screen.screen_stored(&kind, record, path, &format!("{named}#{record_id}"), "stored library record");
            }
            match matching.first() {
                Some(_) => {}
                None => warnings.push(diagnostic(
                    LIBRARY_REF_UNRESOLVED,
                    path,
                    format!("The stored {kind} library `{id}` holds no record `{record_id}`; its disposition is not known here."),
                )),
            }
        }
    }
    if screen.blocking.is_empty() { Ok(warnings) } else { Err(screen.blocking) }
}

/// The disposition members the structural screen judges, with the values that
/// block: W-6a's policy (`redistribution_status` `protected_suspected` or
/// `rejected`, `review_status` `quarantined` or `rejected`), the privacy and
/// classification members' `protected_suspected` (the webview's rule), and a
/// rule pack's `lifecycle_status` `quarantined` or `rejected`.
pub(crate) const SCREENED_MEMBERS: [(&str, &[&str]); 6] = [
    ("redistribution_status", &["protected_suspected", "rejected"]),
    ("review_status", &["quarantined", "rejected"]),
    ("privacy_class", &["protected_suspected", "quarantined", "rejected"]),
    ("privacy_classification", &["protected_suspected", "quarantined", "rejected"]),
    ("classification", &["protected_suspected", "quarantined", "rejected"]),
    ("lifecycle_status", &["quarantined", "rejected"]),
];

/// One RFC 6901 reference token.
pub(crate) fn pointer_token(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

/// Where an object-valued `classification` is admitted. A schema defines
/// `classification` as an object in one place: a rule-pack document's own
/// top-level member (`schemas/rule_pack.schema.yaml` :13 required, :41 `$ref`
/// `#/$defs/RulePackClassification`, defined :1214 as an object of
/// `privacy_class`, `redistribution_status`, `public_repository_policy` and two
/// booleans). The exemption is granted by the caller, for the root of a
/// document it knows to be a stored rule pack; nothing in a document grants it
/// (no `rule_pack_kind` value does), and nothing nested is ever exempt
/// (W-18a (b1) review rounds 1-2).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ClassificationExemption {
    /// No object-valued `classification` anywhere (libraries, report packages).
    None,
    /// The document's own top-level `classification` object (a stored rule pack).
    RulePackRoot,
}

/// The blocking values of the `classification` member, applied to every
/// string member of an admitted classification object whatever its key.
fn classification_blocking() -> &'static [&'static str] {
    SCREENED_MEMBERS.iter().find(|(member, _)| *member == "classification").map(|(_, blocking)| *blocking).expect("classification is screened")
}

/// The admitted classification object at `pointer`/`key`, if any: only the
/// root's own `classification`, only under [`ClassificationExemption::RulePackRoot`].
fn admitted<'a>(exemption: ClassificationExemption, at_root: bool, key: &str, child: &'a Value) -> Option<&'a Map<String, Value>> {
    (exemption == ClassificationExemption::RulePackRoot && at_root && key == "classification").then(|| child.as_object()).flatten()
}

fn kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

/// `found` without repeats, in first-seen order. The repeats are found by
/// borrowing, so no finding is copied.
fn distinct(found: Vec<String>) -> Vec<String> {
    let keep: Vec<bool> = {
        let mut seen = std::collections::BTreeSet::new();
        found.iter().map(|item| seen.insert(item.as_str())).collect()
    };
    found.into_iter().zip(keep).filter_map(|(item, keep)| keep.then_some(item)).collect()
}

/// The JSON pointer of the value a structural walk is at, kept as it descends
/// (#1641 Codex r2 P2-a): one buffer, pushed and truncated, so no path is
/// retained per level; or only its length, to bound a walk without
/// allocating ([`structural_bound`]).
trait WalkPointer {
    fn len(&self) -> usize;
    fn push_key(&mut self, key: &str);
    fn push_index(&mut self, index: usize);
    fn truncate(&mut self, len: usize);
}

impl WalkPointer for String {
    fn len(&self) -> usize {
        str::len(self)
    }

    fn push_key(&mut self, key: &str) {
        self.push('/');
        push_pointer_token(self, key);
    }

    fn push_index(&mut self, index: usize) {
        use std::fmt::Write;
        let _ = write!(self, "/{index}");
    }

    fn truncate(&mut self, len: usize) {
        String::truncate(self, len);
    }
}

/// A pointer's length only, and the longest it reached.
#[derive(Default)]
struct PointerLength {
    len: usize,
    max: usize,
}

impl WalkPointer for PointerLength {
    fn len(&self) -> usize {
        self.len
    }

    fn push_key(&mut self, key: &str) {
        self.len += 1 + pointer_token_len(key);
        self.max = self.max.max(self.len);
    }

    fn push_index(&mut self, index: usize) {
        self.len += 1 + index.checked_ilog10().map_or(1, |digits| digits as usize + 1);
        self.max = self.max.max(self.len);
    }

    fn truncate(&mut self, len: usize) {
        self.len = len;
    }
}

/// `key` as one RFC 6901 reference token, appended ([`pointer_token`]'s text).
fn push_pointer_token(out: &mut String, key: &str) {
    for character in key.chars() {
        match character {
            '~' => out.push_str("~0"),
            '/' => out.push_str("~1"),
            other => out.push(other),
        }
    }
}

/// The length of [`pointer_token`]`(key)`.
fn pointer_token_len(key: &str) -> usize {
    key.len() + key.bytes().filter(|byte| matches!(byte, b'~' | b'/')).count()
}

/// Where a structural walk's findings go: each is the pointer, then `/` and
/// the token of `member` when there is one, then `middle` and `tail` (the
/// texts the walks always built: `<at> <status>`, `<at>/<member> <status>`,
/// `<at> is <kind>`, `<at>/<member> is <kind>`).
trait WalkFindings<P: WalkPointer> {
    fn add(&mut self, pointer: &P, member: Option<&str>, middle: &str, tail: &str);
}

/// The findings' texts, each built at its exact length.
struct FindingTexts(Vec<String>);

impl WalkFindings<String> for FindingTexts {
    fn add(&mut self, pointer: &String, member: Option<&str>, middle: &str, tail: &str) {
        self.0.push(finding_text(pointer, member, middle, tail));
    }
}

/// One finding's text at its exact length.
fn finding_text(pointer: &str, member: Option<&str>, middle: &str, tail: &str) -> String {
    let length = pointer.len() + member.map_or(0, |member| 1 + pointer_token_len(member)) + middle.len() + tail.len();
    let mut text = String::with_capacity(length);
    text.push_str(pointer);
    if let Some(member) = member {
        text.push('/');
        push_pointer_token(&mut text, member);
    }
    text.push_str(middle);
    text.push_str(tail);
    text
}

/// How many findings a walk makes and their texts' total length.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FindingCount {
    pub(crate) count: u64,
    pub(crate) text_bytes: u64,
}

impl WalkFindings<PointerLength> for FindingCount {
    fn add(&mut self, pointer: &PointerLength, member: Option<&str>, middle: &str, tail: &str) {
        let length = pointer.len + member.map_or(0, |member| 1 + pointer_token_len(member)) + middle.len() + tail.len();
        self.count += 1;
        self.text_bytes += length as u64;
    }
}

fn blocking_walk<P: WalkPointer, F: WalkFindings<P>>(value: &Value, pointer: &mut P, exemption: ClassificationExemption, found: &mut F) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let at_root = pointer.len() == 0;
                let parent = pointer.len();
                pointer.push_key(key);
                if let (Some((_, blocking)), Some(status)) = (SCREENED_MEMBERS.iter().find(|(member, _)| member == key), child.as_str()) {
                    if blocking.contains(&status) {
                        found.add(pointer, None, " ", status);
                    }
                }
                if let Some(members) = admitted(exemption, at_root, key, child) {
                    for (member, inner) in members {
                        let Some(status) = inner.as_str() else { continue };
                        // A member with its own policy that already blocks is reported once, by the walk below.
                        let own = SCREENED_MEMBERS.iter().find(|(screened, _)| screened == member).is_some_and(|(_, blocking)| blocking.contains(&status));
                        if !own && classification_blocking().contains(&status) {
                            found.add(pointer, Some(member), " ", status);
                        }
                    }
                }
                // Below the root nothing is exempt.
                blocking_walk(child, pointer, ClassificationExemption::None, found);
                pointer.truncate(parent);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                let parent = pointer.len();
                pointer.push_index(index);
                blocking_walk(item, pointer, ClassificationExemption::None, found);
                pointer.truncate(parent);
            }
        }
        _ => {}
    }
}

fn malformed_walk<P: WalkPointer, F: WalkFindings<P>>(value: &Value, pointer: &mut P, exemption: ClassificationExemption, found: &mut F) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let at_root = pointer.len() == 0;
                let parent = pointer.len();
                pointer.push_key(key);
                let screened = SCREENED_MEMBERS.iter().any(|(member, _)| member == key);
                let admitted = admitted(exemption, at_root, key, child);
                if screened && (child.is_array() || child.is_object()) && admitted.is_none() {
                    found.add(pointer, None, " is ", kind(child));
                }
                // A `provenance` in positional (array) form hides the members
                // this screen judges (W-17 object form, #1651 Codex r2).
                if key == "provenance" && child.is_array() {
                    found.add(pointer, None, " is ", kind(child));
                }
                // Inside the admitted object no structure is exempt, whatever its key.
                for (member, inner) in admitted.into_iter().flatten() {
                    if inner.is_array() || inner.is_object() {
                        found.add(pointer, Some(member), " is ", kind(inner));
                    }
                }
                malformed_walk(child, pointer, ClassificationExemption::None, found);
                pointer.truncate(parent);
            }
        }
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                let parent = pointer.len();
                pointer.push_index(index);
                malformed_walk(item, pointer, ClassificationExemption::None, found);
                pointer.truncate(parent);
            }
        }
        _ => {}
    }
}

/// What [`screen_value`] and [`malformed_values`] build over `value`, counted
/// without allocating: the longest pointer their shared buffer holds, and
/// each walk's findings (before repeats are dropped).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StructuralBound {
    pub(crate) pointer_max: u64,
    pub(crate) blocking: FindingCount,
    pub(crate) malformed: FindingCount,
}

pub(crate) fn structural_bound(value: &Value) -> StructuralBound {
    let mut pointer = PointerLength::default();
    let mut blocking = FindingCount::default();
    blocking_walk(value, &mut pointer, ClassificationExemption::None, &mut blocking);
    let mut malformed = FindingCount::default();
    malformed_walk(value, &mut pointer, ClassificationExemption::None, &mut malformed);
    StructuralBound { pointer_max: pointer.max as u64, blocking, malformed }
}

fn collect_walk(value: &Value, exemption: ClassificationExemption, walk: fn(&Value, &mut String, ClassificationExemption, &mut FindingTexts)) -> Vec<String> {
    let mut pointer = String::new();
    let mut found = FindingTexts(Vec::new());
    walk(value, &mut pointer, exemption, &mut found);
    drop(pointer);
    distinct(found.0)
}

/// The structural screen of one stored rule-pack document (review round 2,
/// finding 2): every object at every depth (a wrapper, its records, array
/// elements, every `provenance` and `provenance_record` beneath them) is
/// judged by its own [`SCREENED_MEMBERS`]; nothing is inherited and no field
/// hides another object's. In the document's own top-level `classification`
/// object (the one the rule-pack schema defines) every string member is also
/// judged by the `classification` values whatever its key. Returned as
/// `<JSON pointer> <value>`, in document order; empty when nothing blocks.
/// The whole document is screened before any of it is mapped into the
/// manifest. [`screen_value`] is the same screen with no exemption.
pub(crate) fn screen_document(document: &Value) -> Vec<String> {
    collect_walk(document, ClassificationExemption::RulePackRoot, blocking_walk)
}

/// The disposition members of a stored rule-pack document whose value is an
/// array or an object (review round 6 note 4, narrowed in round 6b by the
/// owner's decision): either could hide a blocking value, so the save refuses
/// them as malformed rather than guess. The document's own top-level
/// `classification` object is the one object admitted, and any array or
/// object inside it is malformed; an object-valued `classification` anywhere
/// else is malformed like any other. `null`, booleans and numbers are not
/// refused (the library import stores them); they count as no public
/// disposition and, for privacy members, as private. A `provenance` written
/// as an array (its positional form) is malformed too, at any depth: it hides
/// the members the structural screen judges (W-17, #1651 Codex r2). Returned
/// as `<JSON pointer> is <JSON value kind>`. [`malformed_values`] is the same
/// check with no exemption.
pub(crate) fn malformed_dispositions(document: &Value) -> Vec<String> {
    collect_walk(document, ClassificationExemption::RulePackRoot, malformed_walk)
}

/// [`screen_document`] with no `classification` exemption: a stored library
/// (W-6b) and a report package's request and model (W-18a (b1)).
pub(crate) fn screen_value(value: &Value) -> Vec<String> {
    collect_walk(value, ClassificationExemption::None, blocking_walk)
}

/// [`malformed_dispositions`] with no `classification` exemption: every
/// array- or object-valued disposition member, `classification` included, and
/// every array-valued `provenance`, at any depth.
pub(crate) fn malformed_values(value: &Value) -> Vec<String> {
    collect_walk(value, ClassificationExemption::None, malformed_walk)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// The native save's walk finds the `library_ref`s, with the identities,
    /// that the webview's one walk (`libraryRefSites`, used by the save screen
    /// and the open-time store check) finds: the shared site table. The table
    /// holds known kinds only. For an unknown `library_kind` the surfaces differ,
    /// as on main: the webview save skips the lookup and reports nothing, while
    /// the native save and the open check report `MODEL-LIBRARY-REF-UNRESOLVED`.
    #[test]
    fn library_ref_walk_matches_the_shared_site_table() {
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/library_ref_sites.json")).unwrap();
        let mut screen = DispositionScreen::default();
        screen.walk(&table["document"], "");
        let by_path = |left: &Value, right: &Value| left["path"].as_str().cmp(&right["path"].as_str());
        let mut walked: Vec<Value> = std::mem::take(&mut screen.library_refs)
            .into_iter()
            .flat_map(|((kind, id), references)| {
                references.into_iter().map(move |(path, record_id)| json!({"path": path, "identity": [kind, id, record_id]}))
            })
            .collect();
        walked.sort_by(by_path);
        let mut expected: Vec<Value> = table["sites"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|site| !site["identity"].is_null())
            .map(|site| json!({"path": site["path"], "identity": site["identity"]}))
            .collect();
        expected.sort_by(by_path);
        assert_eq!(walked, expected);
    }

    /// W-18b library rule 1's detector alone, with no disposition screen
    /// (DEL-08-01 Design `report-user-release.md` §2.7, §6): every member
    /// named `library_ref`, whatever its value, with its pointer, in the walk's
    /// order, as the webview's `libraryRefMemberSites` finds it. The
    /// object-only walk still finds the objects alone.
    #[test]
    fn library_ref_member_walk_matches_the_shared_member_table() {
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/library_ref_member_sites.json")).unwrap();
        let kind = |member: &LibraryRefMember<'_>, document: &Value| match member.reference {
            Some(_) => "object",
            None => match document.pointer(&member.pointer).unwrap() {
                Value::Null => "null",
                Value::Bool(_) => "boolean",
                Value::Number(_) => "number",
                Value::String(_) => "string",
                Value::Array(_) => "array",
                Value::Object(_) => unreachable!("an object member is a reference"),
            },
        };
        let document = &table["document"];
        let found: Vec<Value> = library_ref_member_sites(document).iter().map(|member| json!({"pointer": member.pointer, "kind": kind(member, document)})).collect();
        assert_eq!(Value::Array(found), table["sites"]);
        for kind in ["object", "null", "string", "number", "boolean", "array"] {
            assert!(table["sites"].as_array().unwrap().iter().any(|site| site["kind"] == kind), "the table covers a {kind} member");
        }
        let objects: Vec<String> = library_ref_sites(document).into_iter().map(|(path, _)| path).collect();
        assert_eq!(objects.len(), table["sites"].as_array().unwrap().iter().filter(|site| site["kind"] == "object").count());
        // The disposition screen still refuses each non-object member as malformed, on its own.
        let mut screen = DispositionScreen { style: PathStyle::Pointer, ..DispositionScreen::default() };
        screen.walk(document, "");
        let malformed: Vec<&str> =
            screen.blocking.iter().filter(|item| item.code == DISPOSITION_RECORD_MALFORMED && item.path.ends_with("/library_ref")).map(|item| item.path.as_str()).collect();
        let non_objects: Vec<&str> =
            table["sites"].as_array().unwrap().iter().filter(|site| site["kind"] != "object").map(|site| site["pointer"].as_str().unwrap()).collect();
        assert_eq!(malformed, non_objects);
    }

    /// Admission's early-exit detectors (W-18b (r2b), Codex round 1 P1) find
    /// what the collecting walks find first, with the same pointer, through
    /// every site of the shared member table in turn (each found member is
    /// removed and the walk run again), and through escaped keys and indexes
    /// of more than one digit.
    #[test]
    fn first_member_detectors_match_the_collecting_walks() {
        fn remove(document: &mut Value, pointer: &str) {
            let (parent, token) = pointer.rsplit_once('/').unwrap();
            let key = token.replace("~1", "/").replace("~0", "~");
            match document.pointer_mut(parent).unwrap() {
                Value::Object(object) => assert!(object.remove(&key).is_some()),
                Value::Array(items) => {
                    items.remove(key.parse::<usize>().unwrap());
                }
                _ => unreachable!("a member's parent holds it"),
            }
        }
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/library_ref_member_sites.json")).unwrap();
        let escaped = json!({
            "a~b": [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {"c/d": {"library_ref": null}}],
            "z": {"rule_pack_refs": []},
            "w~/": {"x": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, {"rule_pack_refs": [{"rule_pack_id": "invented"}]}]},
            "y": [{"rule_pack_refs": "not a list"}, {"library_ref": {"library_kind": "hanger", "library_id": "invented"}}]
        });
        for mut document in [table["document"].clone(), escaped] {
            assert!(holds_library_ref_member(&document));
            let mut visited = 0;
            loop {
                let first = first_library_ref_member(&document).map(|member| (member.pointer, member.reference.is_some()));
                let oracle = library_ref_member_sites(&document).into_iter().next().map(|member| (member.pointer, member.reference.is_some()));
                assert_eq!(first, oracle);
                assert_eq!(holds_library_ref_member(&document), oracle.is_some());
                let Some((pointer, _)) = oracle else { break };
                remove(&mut document, &pointer);
                visited += 1;
            }
            assert!(visited > 1);
            loop {
                let oracle = rule_pack_ref_member_sites(&document).into_iter().next();
                assert_eq!(first_rule_pack_ref_member_site(&document), oracle);
                let Some(pointer) = oracle else { break };
                // An entry's site is removed with its whole list.
                let member = if pointer.ends_with("/rule_pack_refs") { pointer } else { pointer.rsplit_once('/').unwrap().0.to_string() };
                remove(&mut document, &member);
            }
        }
        let escaped = json!({"a~b": [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {"c/d": {"library_ref": null}}]});
        assert_eq!(first_library_ref_member(&escaped).unwrap().pointer, "/a~0b/11/c~1d/library_ref");
        assert_eq!(first_rule_pack_ref_member_site(&json!({"w~/": {"x": [0, {"rule_pack_refs": [1]}]}})).as_deref(), Some("/w~0~1/x/1/rule_pack_refs/0"));
        assert_eq!(first_library_ref_member(&json!({"library_ref": 1})).unwrap().pointer, "/library_ref");
        assert!(first_library_ref_member(&json!([{"a": "library_ref"}])).is_none());
    }

    #[test]
    fn rule_pack_ref_member_sites_names_every_entry_object_or_not() {
        let document = json!({
            "rule_pack_refs": [{"rule_pack_id": "invented-pack"}, "positional", null],
            "nested": {"rule_pack_refs": "not a list"},
            "empty": {"rule_pack_refs": []}
        });
        assert_eq!(rule_pack_ref_member_sites(&document), vec!["/nested/rule_pack_refs", "/rule_pack_refs/0", "/rule_pack_refs/1", "/rule_pack_refs/2"]);
        assert!(rule_pack_ref_member_sites(&json!({"rule_pack_refs": [], "project": {"id": "p"}})).is_empty());
    }

    // --- Route 010's bounded screen (W-18b; Codex round 1 at #1682, P1) ----

    /// Removes the member or element at `pointer`.
    fn remove_at(document: &mut Value, pointer: &str) {
        let (parent, token) = pointer.rsplit_once('/').unwrap();
        let key = token.replace("~1", "/").replace("~0", "~");
        match document.pointer_mut(parent).unwrap() {
            Value::Object(object) => assert!(object.remove(&key).is_some(), "{pointer}"),
            Value::Array(items) => {
                items.remove(key.parse::<usize>().unwrap());
            }
            _ => unreachable!("a member's parent holds it"),
        }
    }

    /// Documents with findings of every kind, at depth, under escaped keys.
    fn screened_corpus() -> Vec<Value> {
        let table: Value = serde_json::from_str(include_str!("../../../../fixtures/model_document/library_ref_member_sites.json")).unwrap();
        vec![
            table["document"].clone(),
            json!({
                "a~b": [{}, {"c/d": {"redistribution_status": "protected_suspected", "review_status": ["quarantined"]}}],
                "materials": [
                    {"id": "M-1", "provenance_record": {"redistribution_status": "public", "provenance": {"review_status": "rejected"}}},
                    {"provenance_record": {"redistribution_status": "rejected", "provenance_record": {"review_status": "quarantined"}}},
                    {"id": "M-3", "provenance_record": ["positional"]},
                    {"id": "M-4", "provenance_record": {"redistribution_status": {"nested": "protected_suspected"}}}
                ],
                "rule_pack_refs": [{"rule_pack_id": "invented", "redistribution_status": "rejected"}, {"review_status": "quarantined"}, "positional", null],
                "nested": {"rule_pack_refs": "not a list", "library_ref": 7, "privacy_class": "protected_suspected", "classification": {"x": 1}},
                "components": [{"library_ref": {"library_kind": "component", "library_id": "invented"}, "provenance": ["positional"]}],
                "provenance_record": {"review_status": "rejected"},
                "w~/": {"x": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, {"lifecycle_status": "quarantined", "privacy_classification": ["rejected"]}]}
            }),
        ]
    }

    /// Route 010's first findings are the collecting screens' first entries,
    /// finding after finding as each is removed: the structural screens'
    /// texts, the store-resolved screen's own-site diagnostics word for word
    /// (under a root, with no library resolved), and the first object
    /// `library_ref`.
    #[test]
    fn first_findings_match_the_collecting_screens() {
        let mut visited = 0;
        for mut document in screened_corpus() {
            loop {
                let oracle = screen_value(&document)
                    .into_iter()
                    .next()
                    .map(|text| (StructuralFinding::Blocking, text))
                    .or_else(|| malformed_values(&document).into_iter().next().map(|text| (StructuralFinding::Malformed, text)));
                assert_eq!(first_structural_finding(&document), oracle);
                let Some((kind, text)) = oracle else { break };
                let pointer = match kind {
                    StructuralFinding::Blocking => text.rsplit_once(' ').unwrap().0.to_string(),
                    StructuralFinding::Malformed => text.rsplit_once(" is ").unwrap().0.to_string(),
                };
                remove_at(&mut document, &pointer);
                visited += 1;
            }
        }
        assert!(visited > 10, "{visited}");
        let mut visited = 0;
        for mut document in screened_corpus() {
            loop {
                let collected = screen_dispositions_in(&document, "/model", PathStyle::Pointer, "p", "X", &mut |_: &str, _: &str, _: &str| Ok(None::<Value>));
                let oracle = collected.err().and_then(|blocking| blocking.into_iter().next());
                assert_eq!(first_document_site_finding(&document, "/model"), oracle);
                let Some(found) = oracle else { break };
                remove_at(&mut document, found.path.strip_prefix("/model").unwrap());
                visited += 1;
            }
            let oracle = library_ref_member_sites(&document).into_iter().find(|member| member.reference.is_some()).map(|member| member.pointer);
            assert_eq!(first_library_ref_object(&document), oracle);
        }
        assert!(visited > 8, "{visited}");
        assert_eq!(first_library_ref_object(&json!({"a": {"library_ref": null}, "b": [{"library_ref": {}}]})).as_deref(), Some("/b/0/library_ref"));
        let found = first_document_site_finding(&json!({"provenance_record": {"review_status": "rejected"}}), "").unwrap();
        assert!(found.message.starts_with("$ (provenance_record: review_status rejected)"), "{}", found.message);
    }

    /// The bounded walks allocate nothing per level and nothing per finding:
    /// over 100 levels of 4,000-byte keys holding 128 findings, each holds at
    /// most its one pointer buffer and the first finding's text and
    /// diagnostic, while the collecting screens hold every finding.
    #[test]
    fn first_findings_allocate_one_pointer() {
        let key = "x".repeat(3_997);
        let mut value = json!(vec![json!({"redistribution_status": "protected_suspected", "provenance_record": {"review_status": "rejected"}}); 128]);
        for level in 0..100 {
            value = json!({ format!("{level:03}{key}"): value });
        }
        let pointer = 100 * 4_001 + 64;
        let envelope = |bytes: usize| crate::ingress::allocation_envelope(bytes as u64).unwrap();
        let (found, structural) = crate::ingress::counting::measure(|| first_structural_finding(&value));
        assert_eq!(found.unwrap().0, StructuralFinding::Blocking);
        assert!(structural.peak_priced <= 2 * envelope(pointer), "{structural:?}");
        let (found, site) = crate::ingress::counting::measure(|| first_document_site_finding(&value, "/model"));
        assert!(found.unwrap().path.ends_with("/0/provenance_record"));
        assert!(site.peak_priced <= 4 * envelope(pointer + 512), "{site:?}");
        let (_, collected) = crate::ingress::counting::measure(|| screen_value(&value));
        assert!(collected.peak_priced > 64 * envelope(pointer), "{collected:?}");
    }

    // --- #1641 Codex r2 P2-a: the structural walks' paths -------------------

    /// The walks before #1641 r2 (each level kept its own path), the oracle.
    mod former {
        use super::super::*;

        fn distinct(found: Vec<String>) -> Vec<String> {
            let mut seen = std::collections::BTreeSet::new();
            found.into_iter().filter(|item| seen.insert(item.clone())).collect()
        }

        fn admitted<'a>(exemption: ClassificationExemption, pointer: &str, key: &str, child: &'a Value) -> Option<&'a Map<String, Value>> {
            (exemption == ClassificationExemption::RulePackRoot && pointer.is_empty() && key == "classification").then(|| child.as_object()).flatten()
        }

        pub(super) fn blocking_walk(value: &Value, pointer: &str, exemption: ClassificationExemption, found: &mut Vec<String>) {
            match value {
                Value::Object(object) => {
                    for (key, child) in object {
                        let at = format!("{pointer}/{}", pointer_token(key));
                        if let (Some((_, blocking)), Some(status)) = (SCREENED_MEMBERS.iter().find(|(member, _)| member == key), child.as_str()) {
                            if blocking.contains(&status) {
                                found.push(format!("{at} {status}"));
                            }
                        }
                        if let Some(members) = admitted(exemption, pointer, key, child) {
                            for (member, inner) in members {
                                let Some(status) = inner.as_str() else { continue };
                                let own = SCREENED_MEMBERS.iter().find(|(screened, _)| screened == member).is_some_and(|(_, blocking)| blocking.contains(&status));
                                if !own && classification_blocking().contains(&status) {
                                    found.push(format!("{at}/{} {status}", pointer_token(member)));
                                }
                            }
                        }
                        blocking_walk(child, &at, ClassificationExemption::None, found);
                    }
                }
                Value::Array(items) => {
                    for (index, item) in items.iter().enumerate() {
                        blocking_walk(item, &format!("{pointer}/{index}"), ClassificationExemption::None, found);
                    }
                }
                _ => {}
            }
        }

        pub(super) fn malformed_walk(value: &Value, pointer: &str, exemption: ClassificationExemption, found: &mut Vec<String>) {
            match value {
                Value::Object(object) => {
                    for (key, child) in object {
                        let at = format!("{pointer}/{}", pointer_token(key));
                        let screened = SCREENED_MEMBERS.iter().any(|(member, _)| member == key);
                        let admitted = admitted(exemption, pointer, key, child);
                        if screened && (child.is_array() || child.is_object()) && admitted.is_none() {
                            found.push(format!("{at} is {}", kind(child)));
                        }
                        if key == "provenance" && child.is_array() {
                            found.push(format!("{at} is {}", kind(child)));
                        }
                        for (member, inner) in admitted.into_iter().flatten() {
                            if inner.is_array() || inner.is_object() {
                                found.push(format!("{at}/{} is {}", pointer_token(member), kind(inner)));
                            }
                        }
                        malformed_walk(child, &at, ClassificationExemption::None, found);
                    }
                }
                Value::Array(items) => {
                    for (index, item) in items.iter().enumerate() {
                        malformed_walk(item, &format!("{pointer}/{index}"), ClassificationExemption::None, found);
                    }
                }
                _ => {}
            }
        }

        pub(super) fn all(value: &Value, exemption: ClassificationExemption) -> (Vec<String>, Vec<String>) {
            let (mut blocking, mut malformed) = (Vec::new(), Vec::new());
            blocking_walk(value, "", exemption, &mut blocking);
            malformed_walk(value, "", exemption, &mut malformed);
            (blocking, malformed)
        }

        pub(super) fn distinct_all(value: &Value, exemption: ClassificationExemption) -> (Vec<String>, Vec<String>) {
            let (blocking, malformed) = all(value, exemption);
            (distinct(blocking), distinct(malformed))
        }
    }

    /// Every committed JSON fixture that holds libraries, rule packs, models
    /// or handoff manifests (under 2 MB each).
    fn committed_documents() -> Vec<(String, Value)> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures");
        let mut found = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = ["material", "hanger", "component", "model_document", "handoff", "product_preview", "redaction_export_controls"].iter().map(|dir| root.join(dir)).collect();
        while let Some(path) = stack.pop() {
            if path.is_dir() {
                stack.extend(std::fs::read_dir(&path).unwrap().map(|entry| entry.unwrap().path()));
            } else if path.extension().is_some_and(|extension| extension == "json") && std::fs::metadata(&path).unwrap().len() < 2 << 20 {
                if let Ok(value) = serde_json::from_str::<Value>(&std::fs::read_to_string(&path).unwrap()) {
                    found.push((path.display().to_string(), value));
                }
            }
        }
        found.sort_by(|left, right| left.0.cmp(&right.0));
        found
    }

    /// Nested and long-key documents with findings at depth: blocking and
    /// malformed members, an admitted and a nested `classification`, keys
    /// holding `~` and `/`, arrays of objects.
    fn generated_documents() -> Vec<(String, Value)> {
        let mut documents = Vec::new();
        for (depth, key_length) in [(1, 1), (5, 3), (40, 64), (100, 32 * 1024)] {
            let mut tail = json!({
                "review_status": "quarantined",
                "redistribution_status": ["rejected"],
                "privacy_class": { "nested": "protected_suspected" },
                "classification": { "privacy_class": "quarantined", "other": "rejected", "deep": { "x": 1 } },
                "items": [{ "lifecycle_status": "rejected" }, { "review_status": "accepted" }, [{ "privacy_classification": "protected_suspected" }]]
            });
            for level in 0..depth {
                let key = format!("{}{}", if level % 3 == 0 { "~a/" } else { "k" }, "x".repeat(key_length));
                tail = if level % 4 == 1 { json!([tail, { "review_status": "rejected" }]) } else { json!({ key: tail, "review_status": if level % 2 == 0 { "rejected" } else { "accepted" } }) };
            }
            documents.push((format!("nested depth {depth}, keys {key_length}"), json!({ "classification": { "privacy_class": "rejected", "a/b": "quarantined", "list": [] }, "body": tail })));
        }
        documents
    }

    /// The walks keep one pointer buffer now; their findings, texts and order
    /// are the former walks' on every committed fixture and on nested and
    /// long-key documents, with and without the rule-pack exemption, and the
    /// allocation-free bound counts exactly the findings they build.
    #[test]
    fn the_structural_walks_find_what_the_former_walks_found() {
        let mut compared = 0;
        let mut with_findings = 0;
        for (name, document) in committed_documents().into_iter().chain(generated_documents()) {
            for exemption in [ClassificationExemption::None, ClassificationExemption::RulePackRoot] {
                let (blocking, malformed) = former::distinct_all(&document, exemption);
                let (now_blocking, now_malformed) = match exemption {
                    ClassificationExemption::None => (screen_value(&document), malformed_values(&document)),
                    ClassificationExemption::RulePackRoot => (screen_document(&document), malformed_dispositions(&document)),
                };
                assert_eq!(now_blocking, blocking, "{name}: blocking findings");
                assert_eq!(now_malformed, malformed, "{name}: malformed findings");
                with_findings += usize::from(!blocking.is_empty() || !malformed.is_empty());
                compared += 1;
            }
            let (blocking, malformed) = former::all(&document, ClassificationExemption::None);
            let bound = structural_bound(&document);
            let count = |found: &[String]| FindingCount { count: found.len() as u64, text_bytes: found.iter().map(|item| item.len() as u64).sum() };
            assert_eq!(bound.blocking, count(&blocking), "{name}: the blocking bound");
            assert_eq!(bound.malformed, count(&malformed), "{name}: the malformed bound");
        }
        eprintln!("STRUCTURAL_DIFFERENTIAL compared={compared} with_findings={with_findings}");
        assert!(compared > 60 && with_findings >= 8, "compared {compared}, {with_findings} with findings");
    }

}

/// The disposition parity table (review round 2): the library import's public
/// API and `stored_item_blocking` judge the same stored documents, across every
/// library kind, both levels (the library object and a record) and every
/// combination of own and provenance dispositions below; the verdicts must be
/// equal and equal to `fixtures/model_document/library_disposition_parity.json`,
/// which also pins the webview adapter (`modelDocumentSave.parity.test.ts`).
/// Regenerate the table after an intended import change with
/// `SWBPIPE_WRITE_DISPOSITION_PARITY=<path> cargo test disposition_verdicts`.
/// Every row carries a provenance object: where the import first refuses a
/// missing provenance, the port judges own fields alone (stricter), which this
/// table does not cover.
#[cfg(test)]
mod disposition_parity {
    use super::*;
    use open_pipe_stress_library_import_document::validate_library_import_tokens;
    use serde_json::json;

    const TABLE: &str = include_str!("../../../../fixtures/model_document/library_disposition_parity.json");
    const KINDS: [&str; 4] = ["material", "section", "component", "hanger"];
    const LEVELS: [&str; 2] = ["library", "record"];
    const OWN_REDISTRIBUTION: [Option<&str>; 5] = [None, Some(""), Some("private_only"), Some("protected_suspected"), Some("rejected")];
    const OWN_REVIEW: [Option<&str>; 4] = [None, Some("accepted"), Some("quarantined"), Some("rejected")];
    const PROVENANCE_REDISTRIBUTION: [&str; 3] = ["private_only", "protected_suspected", "rejected"];
    const PROVENANCE_REVIEW: [&str; 3] = ["accepted", "quarantined", "rejected"];

    /// The stored document of one row; the other level is clear.
    pub(super) fn document(kind: &str, level: &str, own: (Option<&str>, Option<&str>), provenance: (&str, &str)) -> Value {
        let clear = json!({"redistribution_status": "private_only", "review_status": "accepted"});
        let mut varied = json!({"provenance": {"redistribution_status": provenance.0, "review_status": provenance.1}});
        if let Some(status) = own.0 { varied["redistribution_status"] = json!(status); }
        if let Some(status) = own.1 { varied["review_status"] = json!(status); }
        let mut library = json!({"library_id": "invented.parity", "provenance": clear});
        let id_key = crate::library_records_key(kind).unwrap().1;
        let mut record = json!({id_key: "invented.record", "provenance": clear});
        let target = if level == "library" { &mut library } else { &mut record };
        for (key, value) in varied.as_object().unwrap() {
            target[key] = value.clone();
        }
        json!({"schema_version": "0.1.0", format!("{kind}_library"): library, format!("{kind}_records"): [record]})
    }

    fn import_blocks(kind: &str, document: &Value) -> bool {
        validate_library_import_tokens(document, kind, "private")
            .unwrap()
            .findings
            .iter()
            .any(|finding| matches!(finding.code.as_str(), "IMPORT_PROTECTED_CONTENT_SUSPECTED" | "IMPORT_REJECTED_SOURCE"))
    }

    fn port_blocks(kind: &str, document: &Value) -> bool {
        let library = document[format!("{kind}_library")].as_object().unwrap();
        let record = document[format!("{kind}_records")][0].as_object().unwrap();
        !stored_item_blocking(kind, library).is_empty() || !stored_item_blocking(kind, record).is_empty()
    }

    #[test]
    fn disposition_verdicts_match_the_library_import() {
        let mut rows = Vec::new();
        for kind in KINDS {
            for level in LEVELS {
                for own_redistribution in OWN_REDISTRIBUTION {
                    for own_review in OWN_REVIEW {
                        for provenance_redistribution in PROVENANCE_REDISTRIBUTION {
                            for provenance_review in PROVENANCE_REVIEW {
                                let stored = document(kind, level, (own_redistribution, own_review), (provenance_redistribution, provenance_review));
                                let imported = import_blocks(kind, &stored);
                                assert_eq!(port_blocks(kind, &stored), imported, "{kind}/{level}: own {own_redistribution:?}/{own_review:?}, provenance {provenance_redistribution}/{provenance_review}");
                                rows.push(json!([kind, level, own_redistribution, own_review, provenance_redistribution, provenance_review, imported]));
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(rows.len(), 1440);
        // The hanger conflict rule is exercised: an accepted own review over a quarantined provenance.
        assert!(rows.contains(&json!(["hanger", "record", null, "accepted", "private_only", "quarantined", true])));
        assert!(rows.contains(&json!(["material", "record", null, "accepted", "private_only", "quarantined", false])));
        let table = json!({
            "description": "Invented library-disposition parity table (DEL-15-01 W-6a review round 2). Columns: library_kind, level (library object or record), own redistribution_status, own review_status (null = absent), provenance redistribution_status, provenance review_status, blocked. blocked is the library import's verdict (IMPORT_PROTECTED_CONTENT_SUSPECTED or IMPORT_REJECTED_SOURCE); the native save and the webview adapter must agree. Generated by model_document_save.rs disposition_verdicts_match_the_library_import.",
            "columns": ["library_kind", "level", "own_redistribution_status", "own_review_status", "provenance_redistribution_status", "provenance_review_status", "blocked"],
            "rows": rows,
        });
        if let Ok(path) = std::env::var("SWBPIPE_WRITE_DISPOSITION_PARITY") {
            let lines: Vec<String> = table["rows"].as_array().unwrap().iter().map(|row| format!("    {row}")).collect();
            let text = format!(
                "{{\n  \"description\": {},\n  \"columns\": {},\n  \"rows\": [\n{}\n  ]\n}}\n",
                table["description"], table["columns"], lines.join(",\n")
            );
            std::fs::write(path, text).unwrap();
            return;
        }
        let pinned: Value = serde_json::from_str(TABLE).unwrap();
        assert_eq!(pinned["rows"], table["rows"], "the pinned table differs from the import's verdicts; regenerate it");
    }
}
