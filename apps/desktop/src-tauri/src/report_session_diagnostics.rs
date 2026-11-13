//! The session's Diagnostics entries in an exact-route or pressure-free
//! Timoshenko report, bound by the host (review of f3d02469c, finding 2).
//!
//! A bound report's `report_sections.diagnostics` opens with the projection
//! of the delivered result's diagnostics (`check_report_diagnostics`). The
//! entries after them are the session's, and no caller-supplied member (a
//! class, a reference type, a remediation text, an origin tag) can establish
//! that an entry is the session's: the host derives every entry the session
//! may append from what it holds, and admits an appended entry only when it
//! equals one of them on its code, class, severity, source, affected object,
//! message and remediation (the provenance is the session record, unbound).
//! An appended entry with a result diagnostic's code and affected object is
//! refused first, whatever its prose. The kinds, mirroring the desktop
//! builder (`apps/desktop/src/features/report/renderableReportInput.ts`
//! `buildRenderableReportInput`), each derived from the bound intake model:
//! - `COMPONENT_PROVENANCE_MISSING` for each component without provenance
//!   (`componentProvenanceDiagnostics`);
//! - `SPRING_HANGER_PROVENANCE_MISSING` for each spring-hanger or
//!   constant-effort support without `provenance` or `hanger.source_reference`
//!   (`springHangerDiagnostics`, `springHangerSupport.ts`);
//! - `MATERIAL_PROVENANCE_MISSING` and `SECTION_PROVENANCE_MISSING` for each
//!   material and section without provenance (`subjectProvenance`);
//! - a library store finding (`libraryFindingDiagnostics`):
//!   `MODEL-LIBRARY-REF-UNRESOLVED` or `MODEL-LIBRARY-REF-VERSION-CHANGED` on
//!   an entity that carries a `library_ref` in the bound model, at most one
//!   per reference site, among the store check's findings that reference's
//!   shape allows (`lib.rs` `library_ref_findings`; [`for_each_site`]).
//!   Whether the store holds the library is the one fact the host does not
//!   re-check: it reads no store here.
//!
//! Each site yields at most one entry, so a repeated entry is admitted only
//! as many times as the model's derivation yields it (counts, not
//! deduplication: two reference sites can yield identical findings), and a
//! site that may yield one of several findings is used once ([`place`]).
//!
//! Memory (review round 3 of c290410c6 and the provision of both routes): the
//! check's priced peak is at most [`session_check_provision_bytes`] of the
//! model text and the report's Diagnostics text, for any number of entries of
//! any size. No derived text is built: a site's entries are sequences of
//! borrowed pieces ([`Piece`]) compared and hashed byte by byte against the
//! report's own strings. The only storage that grows with the entries is an
//! index of the entries the check can match, and an entry is indexed only when
//! its own text pays for it: an entry not of a session entry's form is refused
//! at its index before anything is stored for it ([`place`]), and the copy
//! check stores only entries with a string code and affected object id
//! ([`first_copy_of_a_result_diagnostic`]). The arithmetic is at [`place`].
//!
//! Applied by the existing render command's admission
//! (`report_render_exact::bind_render_admission`, on both bound identities)
//! and by the local-private package save's admission
//! (`report_package_save::admit_both_routes`, on exact and pressure-free
//! Timoshenko documents).
//! Standard claim fence applies (claims taxonomy per DEC-081).

use open_pipe_stress_report_package::bindings::BINDING_MISMATCH;
use serde_json::{Map, Value};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

/// The library store check's diagnostic source (desktop
/// `features/library/libraryRefFindings.ts` `LIBRARY_REF_CHECK_SOURCE`).
pub(crate) const LIBRARY_REF_CHECK_SOURCE: &str = "apps/desktop/src-tauri/src/lib.rs check_model_library_refs";
/// The name of the one refusal that is a limit of this check, not a fault of
/// the report ([`place`]).
pub(crate) const SESSION_CHECK_LIMIT: &str = "SESSION-DIAGNOSTICS-CHECK-LIMIT";
/// The fixed part of [`session_check_provision_bytes`].
pub(crate) const SESSION_CHECK_FIXED_BYTES: u64 = 64 << 10;

/// The memory provided for [`check_session_diagnostics`] before it runs, on both routes: the
/// bound model's text, the text of the report's Diagnostics, and a fixed part. The render grows
/// its lease by exactly this before the check (`report_render_exact::bind_render_admission`);
/// the save, whose lease grows before its request is parsed, provides it with the request text
/// standing for the Diagnostics text it contains (`report_package_save::session_check_lease_bytes`,
/// within `ingress_lease_bytes`). The regressions assert the check's priced peak against
/// this same function, so the provision and the test are one number; the arithmetic that the
/// check stays within it for any entries is at [`place`].
pub(crate) fn session_check_provision_bytes(model_text_bytes: u64, diagnostics_text_bytes: u64) -> u64 {
    model_text_bytes.saturating_add(diagnostics_text_bytes).saturating_add(SESSION_CHECK_FIXED_BYTES)
}

/// The length of the compact JSON text of the report's Diagnostics
/// (`report_sections.diagnostics`), counted without building the text; 0 where there is none.
pub(crate) fn diagnostics_text_bytes(input: &Value) -> u64 {
    struct Count(u64);
    impl std::io::Write for Count {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self.0.saturating_add(bytes.len() as u64);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let Some(diagnostics) = input.pointer("/report_sections/diagnostics") else { return 0 };
    let mut count = Count(0);
    // Writing to a counter cannot fail, and a `Value` always serializes.
    let _ = serde_json::to_writer(&mut count, diagnostics);
    count.0
}

const PROVENANCE_WARNING: &str = "PROVENANCE_WARNING";
const WARNING: &str = "warning";
const UNRESOLVED: &str = "MODEL-LIBRARY-REF-UNRESOLVED";
const VERSION_CHANGED: &str = "MODEL-LIBRARY-REF-VERSION-CHANGED";
const LIBRARY_REMEDIATION: &str = "Compare the library reference with the local library store before relying on this report.";

/// JavaScript's `String.prototype.trim` (the builder's): WhiteSpace and
/// LineTerminator, so U+FEFF is removed and U+0085 kept, unlike Rust's.
fn js_trim(text: &str) -> &str {
    let space = |c: char| {
        matches!(c, '\u{9}' | '\u{a}' | '\u{b}' | '\u{c}' | '\u{d}' | ' ' | '\u{a0}' | '\u{1680}' | '\u{2000}'..='\u{200a}' | '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}' | '\u{feff}')
    };
    text.trim_matches(space)
}

/// The builder's `text`: a string non-empty after trimming, trimmed.
fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str).map(js_trim).filter(|text| !text.is_empty())
}

/// The builder's `hasProvenance`: `provenance_record.source_name`, or a
/// non-empty `provenance` note (a string after trimming, or an object with a
/// member).
fn has_provenance(entity: &Value) -> bool {
    if text(entity.get("provenance_record").filter(|record| record.is_object()).and_then(|record| record.get("source_name"))).is_some() {
        return true;
    }
    match entity.get("provenance") {
        Some(Value::String(note)) => !js_trim(note).is_empty(),
        Some(Value::Object(note)) => !note.is_empty(),
        _ => false,
    }
}

/// `springHangerSupport.ts` `isSpringHangerSupport` (Rust trimming, as there).
fn is_spring_hanger(support: &Value) -> bool {
    let explicit = support.pointer("/hanger/hanger_type").and_then(Value::as_str).map(str::trim).unwrap_or("");
    let kind = if explicit.is_empty() { support.get("family").and_then(Value::as_str).map(str::trim) } else { Some(explicit) };
    matches!(kind, Some("variable_spring_hanger" | "spring_hanger" | "constant_effort_support"))
}

/// The JavaScript `a || b` of the builder on optional strings.
fn or_id<'a>(label: Option<&'a Value>, id: &'a str) -> &'a str {
    label.and_then(Value::as_str).filter(|label| !label.is_empty()).unwrap_or(id)
}

fn items<'a>(model: &'a Value, member: &str) -> impl Iterator<Item = &'a Value> {
    model.get(member).and_then(Value::as_array).into_iter().flatten()
}

fn id(entity: &Value) -> &str {
    entity.get("id").and_then(Value::as_str).unwrap_or("")
}

// ---------------------------------------------------------------------------
// Texts compared without being built.
// ---------------------------------------------------------------------------

/// One step of a `library_ref` site's path in the builder's walk: a member
/// name or an array index.
#[derive(Clone, Copy)]
enum Segment<'a> {
    Key(&'a str),
    Index(usize),
}

/// A piece of a derived text: a borrowed string, or a site's path rendered as
/// the builder renders it (`a.b[0].library_ref`).
#[derive(Clone, Copy)]
enum Piece<'a> {
    Str(&'a str),
    Path(&'a [Segment<'a>]),
}

/// The text's bytes in order, in chunks, to `f`; `false` as soon as `f`
/// returns `false`. Allocates nothing: an index's digits are written on the
/// stack.
fn chunks(text: &[Piece<'_>], f: &mut dyn FnMut(&[u8]) -> bool) -> bool {
    for piece in text {
        match *piece {
            Piece::Str(string) => {
                if !f(string.as_bytes()) {
                    return false;
                }
            }
            Piece::Path(segments) => {
                for (at, segment) in segments.iter().enumerate() {
                    match *segment {
                        Segment::Key(key) => {
                            if (at > 0 && !f(b".")) || !f(key.as_bytes()) {
                                return false;
                            }
                        }
                        Segment::Index(index) => {
                            let mut digits = [0u8; 20];
                            if !f(b"[") || !f(decimal(index, &mut digits)) || !f(b"]") {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
    true
}

/// `number` in decimal, written at the end of `digits`.
fn decimal(mut number: usize, digits: &mut [u8; 20]) -> &[u8] {
    let mut start = digits.len();
    loop {
        start -= 1;
        digits[start] = b'0' + (number % 10) as u8;
        number /= 10;
        if number == 0 {
            return &digits[start..];
        }
    }
}

/// Whether the pieced text equals `string`, byte for byte.
fn text_equals(text: &[Piece<'_>], string: &str) -> bool {
    let mut rest = string.as_bytes();
    let whole = chunks(text, &mut |chunk| match rest.strip_prefix(chunk) {
        Some(after) => {
            rest = after;
            true
        }
        None => false,
    });
    whole && rest.is_empty()
}

/// The text materialized (tests only).
#[cfg(test)]
fn text_string(text: &[Piece<'_>]) -> String {
    let mut out = Vec::new();
    chunks(text, &mut |chunk| {
        out.extend_from_slice(chunk);
        true
    });
    String::from_utf8(out).expect("pieces of UTF-8 strings")
}

/// Every byte one call at a time, then a separator no UTF-8 text contains: a
/// pieced text and an equal string feed the hasher the same calls, so they
/// hash alike whatever the hasher does with adjacent writes.
fn hash_bytes(hasher: &mut impl Hasher, bytes: &[u8]) {
    for &byte in bytes {
        hasher.write_u8(byte);
    }
}

fn hash_str(hasher: &mut impl Hasher, string: &str) {
    hash_bytes(hasher, string.as_bytes());
    hasher.write_u8(0xff);
}

fn hash_text(hasher: &mut impl Hasher, text: &[Piece<'_>]) {
    chunks(text, &mut |chunk| {
        hash_bytes(hasher, chunk);
        true
    });
    hasher.write_u8(0xff);
}

// ---------------------------------------------------------------------------
// The entries the session may append, and the report's entries.
// ---------------------------------------------------------------------------

/// A report entry's seven bound fields, borrowed: code, class, severity,
/// source and affected object (`{ref_type, ref_id}`, exactly those two
/// members), message and remediation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Fields<'a> {
    code: &'a str,
    class: &'a str,
    severity: &'a str,
    source: (&'a str, &'a str),
    affected: (&'a str, &'a str),
    message: &'a str,
    remediation: &'a str,
}

/// A `{ref_type, ref_id}` object with string members and no other member.
fn reference_of(value: Option<&Value>) -> Option<(&str, &str)> {
    let object = value?.as_object()?;
    if object.len() != 2 {
        return None;
    }
    Some((object.get("ref_type")?.as_str()?, object.get("ref_id")?.as_str()?))
}

impl<'a> Fields<'a> {
    /// `entry`'s bound fields, or `None` when one of them is not of a session
    /// entry's form, so that no derived entry can equal it.
    fn of(entry: &'a Value) -> Option<Self> {
        let string = |field: &str| entry.get(field).and_then(Value::as_str);
        Some(Fields {
            code: string("code")?,
            class: string("class")?,
            severity: string("severity")?,
            source: reference_of(entry.get("source"))?,
            affected: reference_of(entry.get("affected_object"))?,
            message: string("message")?,
            remediation: string("remediation")?,
        })
    }

    fn hash(&self, state: &RandomState) -> u64 {
        let mut hasher = state.build_hasher();
        for field in [self.code, self.class, self.severity, self.source.0, self.source.1, self.affected.0, self.affected.1, self.message, self.remediation] {
            hash_str(&mut hasher, field);
        }
        hasher.finish()
    }
}

/// An entry the session may append at a site, its texts in pieces: always a
/// provenance warning, so class and severity are fixed.
struct Derived<'a> {
    code: &'a str,
    source: (&'a str, &'a str),
    affected: (&'a str, &'a [Piece<'a>]),
    message: &'a [Piece<'a>],
    remediation: &'a [Piece<'a>],
}

impl Derived<'_> {
    /// The same calls [`Fields::hash`] makes for an equal entry.
    fn hash(&self, state: &RandomState) -> u64 {
        let mut hasher = state.build_hasher();
        for field in [self.code, PROVENANCE_WARNING, WARNING, self.source.0, self.source.1, self.affected.0] {
            hash_str(&mut hasher, field);
        }
        for text in [self.affected.1, self.message, self.remediation] {
            hash_text(&mut hasher, text);
        }
        hasher.finish()
    }

    fn equals(&self, entry: &Fields<'_>) -> bool {
        self.code == entry.code
            && entry.class == PROVENANCE_WARNING
            && entry.severity == WARNING
            && self.source == entry.source
            && self.affected.0 == entry.affected.0
            && text_equals(self.affected.1, entry.affected.1)
            && text_equals(self.message, entry.message)
            && text_equals(self.remediation, entry.remediation)
    }
}

/// The model's `library_ref` sites in the builder's walk
/// (`libraryImportService.ts` `libraryRefSites`): `emit(entity, reference)`
/// with the owning entity's id, or the site's path when no enclosing object
/// has one. `path` is a stack of borrowed segments, one per level of the walk;
/// the walk is as deep as the model, which the routes parse with serde_json's
/// nesting limit (128).
fn library_ref_sites<'m>(value: &'m Value, owner: Option<&'m str>, path: &mut Vec<Segment<'m>>, emit: &mut dyn FnMut(Piece<'_>, &'m Map<String, Value>)) {
    match value {
        Value::Array(entries) => {
            for (index, entry) in entries.iter().enumerate() {
                path.push(Segment::Index(index));
                library_ref_sites(entry, owner, path, emit);
                path.pop();
            }
        }
        Value::Object(object) => {
            let named = object.get("id").and_then(Value::as_str).filter(|id| !id.is_empty()).or(owner);
            for (key, child) in object {
                path.push(Segment::Key(key));
                if key == "library_ref" {
                    if let Value::Object(reference) = child {
                        match named {
                            Some(entity) => emit(Piece::Str(entity), reference),
                            None => emit(Piece::Path(path), reference),
                        }
                    }
                }
                library_ref_sites(child, named, path, emit);
                path.pop();
            }
        }
        _ => {}
    }
}

/// The library kinds the store holds (`lib.rs` `library_records_key`): a
/// reference of any other kind can only be unresolved.
const STORED_LIBRARY_KINDS: [&str; 4] = ["material", "section", "component", "hanger"];

/// Visits every site at which the session may append a Diagnostics entry for
/// `model`, with the entries it may yield there: at most one of them, once
/// (review of c2cc050ea, R3 and R4). A provenance kind's site yields exactly
/// its one entry. A library reference's site yields at most one store finding,
/// among those the store check (`lib.rs` `library_ref_findings`) can produce
/// for that reference's shape:
/// - the library not held (`MODEL-LIBRARY-REF-UNRESOLVED`), for any reference;
/// - the record not held (`MODEL-LIBRARY-REF-UNRESOLVED`), only when the store
///   can hold the library: a stored kind ([`STORED_LIBRARY_KINDS`], trimmed)
///   and a library id not blank after trimming;
/// - the stored record changed (`MODEL-LIBRARY-REF-VERSION-CHANGED`), only
///   when, in addition, the reference records a `record_id` not blank after
///   trimming (`library_identity`) and a `library_version`: the store check
///   returns the record-not-held finding before it compares any version.
///
/// The entries passed to `visit` are pieces on the stack, borrowing the model,
/// fixed prose and the walk's path: nothing is built or kept.
fn for_each_site(model: &Value, visit: &mut dyn FnMut(&[Derived<'_>])) {
    for component in items(model, "components").filter(|component| !has_provenance(component)) {
        let id = id(component);
        let message = [
            Piece::Str("Component "),
            Piece::Str(or_id(component.get("label"), id)),
            Piece::Str(" has neither a provenance_record nor a component.provenance note; report provenance is incomplete for this component."),
        ];
        let affected = [Piece::Str(id)];
        visit(&[Derived {
            code: "COMPONENT_PROVENANCE_MISSING",
            source: ("component", id),
            affected: ("component", &affected),
            message: &message,
            remediation: &[Piece::Str("Enter component source/provenance before relying on this report.")],
        }]);
    }
    for support in items(model, "supports").filter(|support| is_spring_hanger(support)) {
        let blank = |value: Option<&Value>| value.and_then(Value::as_str).is_none_or(|text| js_trim(text).is_empty());
        if blank(support.get("provenance")) || blank(support.pointer("/hanger/source_reference")) {
            let id = id(support);
            let message = [
                Piece::Str("Spring hanger "),
                Piece::Str(or_id(support.get("label"), id)),
                Piece::Str(" is missing support provenance or hanger.source_reference."),
            ];
            let affected = [Piece::Str(id)];
            visit(&[Derived {
                code: "SPRING_HANGER_PROVENANCE_MISSING",
                source: ("support", id),
                affected: ("support", &affected),
                message: &message,
                remediation: &[Piece::Str("Enter spring-hanger source/provenance before relying on this report.")],
            }]);
        }
    }
    for (kind, member, label, code, title) in
        [("material", "materials", "label", "MATERIAL_PROVENANCE_MISSING", "Material"), ("section", "sections", "name", "SECTION_PROVENANCE_MISSING", "Section")]
    {
        let remediation = [Piece::Str("Enter "), Piece::Str(kind), Piece::Str(" source/provenance before relying on this report.")];
        for subject in items(model, member).filter(|subject| !has_provenance(subject)) {
            let id = id(subject);
            let message = [
                Piece::Str(title),
                Piece::Str(" "),
                Piece::Str(text(subject.get(label)).unwrap_or(id)),
                Piece::Str(" has neither a provenance_record nor a provenance note; report provenance is incomplete for this "),
                Piece::Str(kind),
                Piece::Str("."),
            ];
            let affected = [Piece::Str(id)];
            visit(&[Derived { code, source: (kind, id), affected: (kind, &affected), message: &message, remediation: &remediation }]);
        }
    }
    let project = model.pointer("/project/id").and_then(Value::as_str).unwrap_or("").trim();
    let mut path = Vec::new();
    library_ref_sites(model, None, &mut path, &mut |entity, site| {
        let member = |key: &str| site.get(key).and_then(Value::as_str);
        let (kind, library) = (member("library_kind").unwrap_or(""), member("library_id").unwrap_or(""));
        let record = member("record_id");
        let not_held = [
            entity,
            Piece::Str(" names "),
            Piece::Str(kind),
            Piece::Str(" library `"),
            Piece::Str(library),
            Piece::Str("`, which the local store does not hold for project `"),
            Piece::Str(project),
            Piece::Str("`. The model's values stand. A project saved under a new id reads its references as unresolved until the library is imported into that project."),
        ];
        let record_not_held = [
            entity,
            Piece::Str(" names record `"),
            Piece::Str(record.unwrap_or("(none recorded)")),
            Piece::Str("` of "),
            Piece::Str(kind),
            Piece::Str(" library `"),
            Piece::Str(library),
            Piece::Str("`, which the stored library does not hold. The model's values stand."),
        ];
        let changed = [
            Piece::Str("The stored record `"),
            Piece::Str(record.unwrap_or("")),
            Piece::Str("` of "),
            Piece::Str(kind),
            Piece::Str(" library `"),
            Piece::Str(library),
            Piece::Str("` has changed since "),
            entity,
            Piece::Str(" recorded it. The model's values and recorded version stand; re-apply from the library to adopt the stored record."),
        ];
        let mut yielded = 1;
        if STORED_LIBRARY_KINDS.contains(&kind.trim()) && !library.trim().is_empty() {
            yielded = 2;
            let recorded = record.is_some_and(|record| !record.trim().is_empty());
            if recorded && member("library_version").is_some() {
                yielded = 3;
            }
        }
        let affected = [entity];
        let remediation = [Piece::Str(LIBRARY_REMEDIATION)];
        let alternatives = [
            library_finding(UNRESOLVED, &affected, &not_held, &remediation),
            library_finding(UNRESOLVED, &affected, &record_not_held, &remediation),
            library_finding(VERSION_CHANGED, &affected, &changed, &remediation),
        ];
        visit(&alternatives[..yielded]);
    });
}

/// A library store finding (`libraryFindingDiagnostics`).
fn library_finding<'a>(code: &'a str, affected: &'a [Piece<'a>], message: &'a [Piece<'a>], remediation: &'a [Piece<'a>]) -> Derived<'a> {
    Derived { code, source: ("diagnostic_source", LIBRARY_REF_CHECK_SOURCE), affected: ("model_entity", affected), message, remediation }
}

/// Every entry the session may append for `model`, each site's alternatives
/// in turn, materialized (tests: the module docs' kinds).
#[cfg(test)]
pub(crate) fn session_diagnostics(model: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for_each_site(model, &mut |alternatives| {
        for entry in alternatives {
            out.push(serde_json::json!({
                "code": entry.code, "class": PROVENANCE_WARNING, "severity": WARNING,
                "source": {"ref_type": entry.source.0, "ref_id": entry.source.1},
                "affected_object": {"ref_type": entry.affected.0, "ref_id": text_string(entry.affected.1)},
                "message": text_string(entry.message), "remediation": text_string(entry.remediation),
            }));
        }
    });
    out
}

#[cfg(test)]
thread_local! {
    static STEPS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// One unit of the check's work: an entry examined or looked up, a site's
/// entry looked up, a result diagnostic looked up, or a distinct key visited
/// (tests read the total, [`steps`]). A lookup's slot visits are not counted:
/// with a randomly keyed SipHash and a table at most half full their expected
/// number is a constant, as it was for the `HashMap` this replaced.
#[inline]
fn step() {
    #[cfg(test)]
    STEPS.with(|steps| steps.set(steps.get() + 1));
}

/// The units of work this thread's checks have taken (tests).
#[cfg(test)]
pub(crate) fn steps() -> u64 {
    STEPS.with(std::cell::Cell::get)
}

/// An open-addressing table of entry indices (linear probing), at least twice
/// as many slots as entries and a power of two: `4 * slots` bytes, at most 16
/// bytes per entry, already a power of two and so priced at its size.
struct Table {
    slots: Vec<u32>,
}

const EMPTY: u32 = u32::MAX;

impl Table {
    fn for_entries(entries: usize) -> Self {
        Table { slots: vec![EMPTY; entries.saturating_mul(2).max(2).next_power_of_two()] }
    }

    /// The stored index `same` accepts, or the empty slot where the probe ended.
    fn probe(&self, hash: u64, same: &mut dyn FnMut(u32) -> bool) -> Result<u32, usize> {
        let mask = self.slots.len() - 1;
        let mut at = hash as usize & mask;
        loop {
            match self.slots[at] {
                EMPTY => return Err(at),
                stored if same(stored) => return Ok(stored),
                _ => at = (at + 1) & mask,
            }
        }
    }
}

/// How a distinct appended entry sits among the sites that yield it: no site
/// ([`UNSEEN`]); every such site yields no other appended entry before it
/// ([`ROOT`]); sites yield it in different positions ([`CONFLICT`]); or every
/// such site yields this other key just before it (the key's number).
const UNSEEN: u32 = u32::MAX;
const ROOT: u32 = u32::MAX - 1;
const CONFLICT: u32 = u32::MAX - 2;

fn parent(link: u32) -> Option<u32> {
    (link < CONFLICT).then_some(link)
}

/// Why an appended entry is refused, by its index among the appended entries.
#[derive(Debug, PartialEq, Eq)]
enum Unplaced {
    /// No assignment of the appended entries to distinct sites that yield
    /// them places this entry and the entries before it.
    NotDerived(usize),
    /// [`SESSION_CHECK_LIMIT`]: the sites that yield this entry also yield
    /// other appended entries in different orders, which the check does not
    /// match.
    Limit(usize),
}

fn find(sets: &mut [u32], mut key: u32) -> u32 {
    while sets[key as usize] != key {
        sets[key as usize] = sets[sets[key as usize] as usize];
        key = sets[key as usize];
    }
    key
}

/// An assignment of each appended entry to a distinct site that may yield it
/// (each site yields at most one entry, so a repeated entry is admitted only
/// as many times as the model has sites yielding it, and a site that may
/// yield one of several entries is used once), without a general matching,
/// a sort or any derived text.
///
/// The entries before the first one not of a session entry's form are
/// indexed; that one, which no site can yield, is refused at its index unless
/// an entry before it is refused first, and nothing is stored for it or for
/// any entry after it. Each indexed entry gets a key (equal entries share
/// one) through [`Table`]. Each site is reduced to the keys it may yield, in
/// its order (library not held, record not held, record changed); within a
/// site those differ. When every site yielding a key yields the same key just
/// before it (or none), the keys form a forest of depth at most 2 and each
/// site may yield exactly the keys on the path from its last key to a root.
/// Then an assignment exists exactly when every key's subtree has at least as
/// many sites ending in it as entries claiming it (Hall's condition reduces to
/// subtrees: a key's claims can be met only by sites ending in its subtree,
/// and disjoint subtrees add). So one pass over the entries in order, taking
/// one from the slack of each key on the entry's path to its root, stops at
/// the first entry that the entries before it leave no site for.
///
/// Every genuine site structure is such a forest: a site's library findings
/// share its entity, a record-changed text fixes the record-not-held text, and
/// the three texts of a site differ. Distinct sites can share a text without
/// such a forest only where reference fields spell one finding's text through
/// different splits (a backtick in a kind, library or record id); an entry
/// there is refused by name as [`Unplaced::Limit`], a limit of this check.
///
/// Memory, against [`session_check_provision_bytes`]: with n indexed entries
/// and k <= n keys, the table holds at most 16 n bytes (priced at its size),
/// the entries' keys 4 n (priced at most 8 n), and the keys' links, counts and
/// sets 12 k (priced at most 24 n), so at most 48 bytes per indexed entry; the
/// conflict marks (k bytes, at most 2 n priced) are taken after the table is
/// freed. An indexed entry is an object with the seven bound fields, at least
/// 150 bytes of the Diagnostics' compact text with its separating comma (all
/// keys present, every string empty). The path stack is at most serde_json's
/// nesting limit of segments, about 3 KiB, within the fixed part. Nothing else
/// grows with the model or the entries.
///
/// Work: one examination and one lookup per entry, at most three lookups per
/// site, at most three visits per key, and one pass of at most three counter
/// visits per entry. No recursion in the matching.
fn place(appended: &[Value], model: &Value) -> Result<(), Unplaced> {
    let cutoff = appended
        .iter()
        .position(|entry| {
            step();
            Fields::of(entry).is_none()
        })
        .unwrap_or(appended.len());
    let indexed = &appended[..cutoff];
    let fields = |index: u32| Fields::of(&indexed[index as usize]).expect("an indexed entry is of a session entry's form");
    let state = RandomState::new();
    let mut table = Table::for_entries(indexed.len());
    let mut claims: Vec<u32> = Vec::with_capacity(indexed.len());
    let mut keys = 0u32;
    for (index, entry) in indexed.iter().enumerate() {
        step();
        let entry = Fields::of(entry).expect("an indexed entry is of a session entry's form");
        match table.probe(entry.hash(&state), &mut |stored| fields(stored) == entry) {
            Ok(stored) => {
                let key = claims[stored as usize];
                claims.push(key);
            }
            Err(slot) => {
                table.slots[slot] = index as u32;
                claims.push(keys);
                keys += 1;
            }
        }
    }
    let distinct = keys as usize;
    let mut links = vec![UNSEEN; distinct];
    let mut supply = vec![0u32; distinct];
    let mut sets: Vec<u32> = (0..keys).collect();
    for_each_site(model, &mut |alternatives| {
        let (mut path, mut length) = ([0u32; 3], 0);
        for alternative in alternatives {
            step();
            if let Ok(stored) = table.probe(alternative.hash(&state), &mut |stored| alternative.equals(&fields(stored))) {
                let key = claims[stored as usize];
                if !path[..length].contains(&key) {
                    path[length] = key;
                    length += 1;
                }
            }
        }
        for at in 0..length {
            let expected = if at == 0 { ROOT } else { path[at - 1] };
            let link = &mut links[path[at] as usize];
            *link = match *link {
                UNSEEN => expected,
                same if same == expected => same,
                _ => CONFLICT,
            };
            if at > 0 {
                let (above, below) = (find(&mut sets, path[at - 1]), find(&mut sets, path[at]));
                sets[below as usize] = above;
            }
        }
        if length > 0 {
            let last = path[length - 1] as usize;
            supply[last] = supply[last].saturating_add(1);
        }
    });
    drop(table);
    // A group of keys whose sites order them differently.
    let mut conflicted = vec![false; distinct];
    for key in 0..keys {
        step();
        if links[key as usize] == CONFLICT {
            let set = find(&mut sets, key);
            conflicted[set as usize] = true;
        }
    }
    // Each key's count becomes its subtree's: the deepest keys first.
    let depth = |key: usize| match parent(links[key]) {
        Some(above) if parent(links[above as usize]).is_some() => 2,
        Some(_) => 1,
        None => 0,
    };
    for level in [2, 1] {
        for key in 0..distinct {
            step();
            if depth(key) == level && !conflicted[find(&mut sets, key as u32) as usize] {
                let above = parent(links[key]).expect("a key below a root has a parent") as usize;
                supply[above] = supply[above].saturating_add(supply[key]);
            }
        }
    }
    for (index, &key) in claims.iter().enumerate() {
        step();
        if conflicted[find(&mut sets, key) as usize] {
            return Err(Unplaced::Limit(index));
        }
        let (mut chain, mut length, mut node) = ([0u32; 3], 0, Some(key));
        while let Some(at) = node {
            if supply[at as usize] == 0 {
                return Err(Unplaced::NotDerived(index));
            }
            chain[length] = at;
            length += 1;
            node = parent(links[at as usize]);
        }
        for &at in &chain[..length] {
            supply[at as usize] -= 1;
        }
    }
    if cutoff < appended.len() {
        return Err(Unplaced::NotDerived(cutoff));
    }
    Ok(())
}

/// A result diagnostic's affected object id as the projection writes it
/// (`bindings::expected_report_diagnostics`): its affected refs joined by
/// ", ", or, when that is empty, its id, or, when that is empty too,
/// `diagnostic-<index>`.
enum ResultId<'a> {
    Joined(&'a [Value]),
    Id(&'a str),
    Generated(usize),
}

impl ResultId<'_> {
    fn chunks(&self, f: &mut dyn FnMut(&[u8]) -> bool) -> bool {
        match *self {
            ResultId::Joined(refs) => {
                for (at, item) in refs.iter().enumerate() {
                    if (at > 0 && !f(b", ")) || !f(item.as_str().unwrap_or("").as_bytes()) {
                        return false;
                    }
                }
                true
            }
            ResultId::Id(id) => f(id.as_bytes()),
            ResultId::Generated(index) => {
                let mut digits = [0u8; 20];
                f(b"diagnostic-") && f(decimal(index, &mut digits))
            }
        }
    }

    fn equals(&self, string: &str) -> bool {
        let mut rest = string.as_bytes();
        let whole = self.chunks(&mut |chunk| match rest.strip_prefix(chunk) {
            Some(after) => {
                rest = after;
                true
            }
            None => false,
        });
        whole && rest.is_empty()
    }
}

/// Result diagnostic `index`'s code and affected id, refused as the
/// projection refuses a diagnostic whose fields are not of the producer's
/// form, in its order (code, severity, message, affected refs, id, source).
fn result_key(index: usize, diagnostic: &Value) -> Result<(&str, ResultId<'_>), String> {
    let unreadable = |field: &str| format!("{BINDING_MISMATCH}: report diagnostics: source diagnostic {index} has an unreadable {field}");
    let string = |field: &str| diagnostic[field].as_str().ok_or_else(|| unreadable(field));
    let code = string("code")?;
    string("severity")?;
    string("message")?;
    let refs: &[Value] = match &diagnostic["affected_refs"] {
        Value::Null => &[],
        Value::Array(refs) if refs.iter().all(Value::is_string) => refs,
        _ => return Err(unreadable("affected_refs")),
    };
    let id = match &diagnostic["id"] {
        Value::Null => None,
        Value::String(id) => Some(id.as_str()),
        _ => return Err(unreadable("id")),
    };
    if !matches!(&diagnostic["source"], Value::Null | Value::String(_)) {
        return Err(unreadable("source"));
    }
    let joined_empty = refs.is_empty() || (refs.len() == 1 && refs[0].as_str() == Some(""));
    let affected = match id.filter(|id| !id.is_empty()) {
        _ if !joined_empty => ResultId::Joined(refs),
        Some(id) => ResultId::Id(id),
        None => ResultId::Generated(index),
    };
    Ok((code, affected))
}

/// An appended entry's code and affected object id, when both are strings.
fn copy_key(entry: &Value) -> Option<(&str, &str)> {
    Some((entry.get("code")?.as_str()?, entry.get("affected_object")?.get("ref_id")?.as_str()?))
}

/// The index, among `appended`, of the first entry with a code and affected
/// object id of one of the result's diagnostics (as projected), or the
/// projection's refusal of a result diagnostic not of the producer's form.
/// Only appended entries with a string code and affected id are indexed (each
/// at least 44 bytes of compact text with its comma, against at most 16 bytes
/// of [`Table`]); each result diagnostic is looked up once, its affected id
/// compared in pieces.
fn first_copy_of_a_result_diagnostic(appended: &[Value], results: &[Value]) -> Result<Option<usize>, String> {
    let candidates = appended.iter().filter(|entry| copy_key(entry).is_some()).count();
    let state = RandomState::new();
    let mut table = Table::for_entries(candidates);
    let hash = |code: &str, id: &dyn Fn(&mut std::collections::hash_map::DefaultHasher)| {
        let mut hasher = state.build_hasher();
        hash_str(&mut hasher, code);
        id(&mut hasher);
        hasher.finish()
    };
    for (index, entry) in appended.iter().enumerate() {
        step();
        let Some((code, id)) = copy_key(entry) else { continue };
        let key = hash(code, &|hasher| hash_str(hasher, id));
        if let Err(slot) = table.probe(key, &mut |stored| copy_key(&appended[stored as usize]) == Some((code, id))) {
            table.slots[slot] = index as u32;
        }
    }
    let mut first: Option<usize> = None;
    for (index, diagnostic) in results.iter().enumerate() {
        step();
        let (code, affected) = result_key(index, diagnostic)?;
        let key = hash(code, &|hasher| {
            affected.chunks(&mut |chunk| {
                hash_bytes(hasher, chunk);
                true
            });
            hasher.write_u8(0xff);
        });
        let found = table.probe(key, &mut |stored| copy_key(&appended[stored as usize]).is_some_and(|(c, id)| c == code && affected.equals(id)));
        if let Ok(stored) = found {
            first = Some(first.map_or(stored as usize, |earlier| earlier.min(stored as usize)));
        }
    }
    Ok(first)
}

/// The binding (module docs): every entry of `report_sections.diagnostics`
/// after the result's projected entries is one the host derives from `model`,
/// each site yielding at most one ([`for_each_site`], [`place`]).
/// `Err` names the entry's index only. A report with no entry after the
/// result's derives nothing.
pub(crate) fn check_session_diagnostics(input: &Value, source: &Value, model: &Value) -> Result<(), String> {
    let found = input.pointer("/report_sections/diagnostics").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    let Some(results) = source.get("diagnostics").and_then(Value::as_array) else {
        return Err(format!("{BINDING_MISMATCH}: report diagnostics: the source has no diagnostics"));
    };
    if found.len() <= results.len() {
        return Ok(());
    }
    let count = results.len();
    let appended = &found[count..];
    if let Some(index) = first_copy_of_a_result_diagnostic(appended, results)? {
        return Err(format!("report diagnostic {} copies a result diagnostic's code and affected object", count + index));
    }
    place(appended, model).map_err(|unplaced| match unplaced {
        Unplaced::NotDerived(index) => format!("report diagnostic {} is not a session diagnostic the host derives from the bound model", count + index),
        Unplaced::Limit(index) => format!(
            "report diagnostic {}: {SESSION_CHECK_LIMIT}: the bound model's library references yield this entry's text at sites that order their findings differently, which this check does not match; this is a limit of the check, not a fault of the report",
            count + index
        ),
    })
}
