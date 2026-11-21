//! The types and the trait a native caller of the DEL-12-02 export control's content rule
//! uses: what it asks the rule to decide ([`RouteRequest`] and [`Content`]), what the rule
//! returns ([`SummaryDecision`], bound to the decided bytes as a [`SummaryOutcome`]) or
//! refuses ([`RuleError`]), and the rule itself as a trait ([`ContentRule`]). It holds no
//! rule. The names are the texts of
//! `docs/security/redaction_export_controls.md` and
//! `schemas/redaction_export_controls.schema.yaml`.
//!
//! # One buffer
//!
//! A caller holds the route's content as one byte buffer and binds the decision to it:
//!
//! - The digest the decision is bound to ([`SummaryDecision::bound_to`]) is the SHA-256 of
//!   that buffer.
//! - The content given to the rule is read from that buffer by the route's reading. For
//!   JSON text the reading refuses, before any decision, text in which an object repeats a
//!   member name: a decoder keeps one of the two and the buffer holds both.
//! - Content the caller builds is serialised once, in the route's canonical form; that
//!   buffer is the one decided, digested and written.
//! - Where a route's content is text and the route decides it as one JSON string, the
//!   digest is of the text's UTF-8 bytes, not of a JSON encoding of it.
//! - On an admitted outcome the sink receives that buffer. An output derived from decided
//!   content (a print view, an excerpt, a document rendered from a decided input) is content
//!   of its own: it has its own buffer, its own digest and its own decision on its route,
//!   and is not admitted by the decision of what it was derived from.
//!
//! - Content too large to hold as one value is given as a source ([`Content::Object`]). Its
//!   buffer is the bytes its producer writes as the source is walked: the rule is shown each
//!   value in the call that writes that value's bytes, and the caller binds the decision to
//!   the digest of the bytes written, after the walk. Bytes staged during the walk reach
//!   their sink only on an outcome that may expose.
//!
//! The rule cannot check the digest; the binding is the caller's.
//!
//! # What a caller may rely on
//!
//! - `withheld` is false exactly when none of the three [`WithheldReason`]s applies: no
//!   decision blocks; on a request that requires lossless materialization, no decision
//!   redacts or omits, whether or not the controlled copy would come out equal; and the
//!   controlled copy equals the content as decoded JSON values. Then `withheld_reason` is
//!   `None`, `blocking_finding_total` is 0 and `blocking_findings` is empty. Otherwise
//!   `withheld` is true and `withheld_reason` is the first reason that applies, in the
//!   order of [`WithheldReason::ALL`].
//! - A caller exposes content only when the rule returned `Ok` for this content and this
//!   request and [`SummaryOutcome::may_expose`] is true of the decision bound to the bytes
//!   it exposes. A refusal ([`RuleError`]), or an outcome that contradicts itself, exposes
//!   nothing.
//! - Withheld: nothing is exposed. The caller reports `withheld_reason`, the counts and
//!   the bounded blocking findings (reason code and path, never a value).
//! - The decision's `route_id`, `export_context` and `explicit_local_private_intent` are the
//!   request's and `rule_revision` is the rule's, each unchanged; binding adds the digest
//!   and changes nothing else.
//! - The content is borrowed and never changed. Intent comes only from the request.
//! - The same rule revision, content, request and bound give the same decision.
//! - The route id is recorded, not admitted: the caller establishes that its route may run
//!   before it asks for a decision. The one exception is the user-release route, which the
//!   rule refuses with the `user_released_report` context
//!   ([`RuleError::UserReleaseRouteRequired`]).
//!
//! # What it does not offer
//!
//! No per-value decision or finding records and no controlled copy, so a route that must
//! expose a redacted copy is not served by it; no form in which a caller feeds the rule
//! pieces of a payload (the content is the route's whole payload, as a value or as a source
//! the rule walks from its root, because a value's decision depends on what encloses it);
//! no route catalogue, user-release projection, text reading, hashing or
//! write. No I/O, logging, clock or global state. The types serialise and do not
//! deserialise; a caller that hashes a serialised outcome canonicalises it first.

use std::borrow::Cow;
use std::fmt;

use serde::{Serialize, Serializer};
use serde_json::Value;

macro_rules! named_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$vmeta:meta])* $variant:ident => $text:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($(#[$vmeta])* $variant),+
        }

        impl $name {
            /// Every value, in the order the contract or the schema lists them.
            pub const ALL: &'static [$name] = &[$($name::$variant),+];

            /// The contract's text for this value.
            pub fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $text),+
                }
            }

            /// The value whose text is `name`, if any.
            pub fn from_name(name: &str) -> Option<Self> {
                match name {
                    $($text => Some($name::$variant),)+
                    _ => None,
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
    };
}

named_enum! {
    /// The export context a route fixes.
    ExportContext {
        PublicReport => "public_report",
        PublicExample => "public_example",
        SharedModel => "shared_model",
        DownstreamTool => "downstream_tool",
        LocalPrivate => "local_private",
        UserReleasedReport => "user_released_report",
    }
}

named_enum! {
    /// Why a decision was made: the schema's reason codes.
    ReasonCode {
        SafePublicMetadata => "SAFE_PUBLIC_METADATA",
        SafeInventedPublicExample => "SAFE_INVENTED_PUBLIC_EXAMPLE",
        PrivateDataRedacted => "PRIVATE_DATA_REDACTED",
        PrivateLocalAllowedWithWarning => "PRIVATE_LOCAL_ALLOWED_WITH_WARNING",
        UnknownProvenanceWarning => "UNKNOWN_PROVENANCE_WARNING",
        RedistributionStatusUnknown => "REDISTRIBUTION_STATUS_UNKNOWN",
        ProtectedContentBlocked => "PROTECTED_CONTENT_BLOCKED",
        MissingMetadataRedacted => "MISSING_METADATA_REDACTED",
        LocalPrivateIntentRequired => "LOCAL_PRIVATE_INTENT_REQUIRED",
        ProfessionalBoundaryBlocked => "PROFESSIONAL_BOUNDARY_BLOCKED",
        PayloadMetadataOnlyRequired => "PAYLOAD_METADATA_ONLY_REQUIRED",
        SecretMaterialBlocked => "SECRET_MATERIAL_BLOCKED",
        CloudOrNetworkReferenceBlocked => "CLOUD_OR_NETWORK_REFERENCE_BLOCKED",
        DirectSqlAccessBlocked => "DIRECT_SQL_ACCESS_BLOCKED",
        StorageBypassBlocked => "STORAGE_BYPASS_BLOCKED",
        ConcretePathRedacted => "CONCRETE_PATH_REDACTED",
        UserReleasedProjectData => "USER_RELEASED_PROJECT_DATA",
        UserReleaseIntentRequired => "USER_RELEASE_INTENT_REQUIRED",
        UserReleaseValueUnbound => "USER_RELEASE_VALUE_UNBOUND",
        ExportInputNotPlainData => "EXPORT_INPUT_NOT_PLAIN_DATA",
        ExportInputReservedName => "EXPORT_INPUT_RESERVED_NAME",
    }
}

named_enum! {
    /// The class of a finding: the schema's finding classes.
    FindingClass {
        IpBoundaryWarning => "IP_BOUNDARY_WARNING",
        PrivateDataWarning => "PRIVATE_DATA_WARNING",
        ProvenanceWarning => "PROVENANCE_WARNING",
        ProfessionalBoundaryWarning => "PROFESSIONAL_BOUNDARY_WARNING",
        StorageBoundaryWarning => "STORAGE_BOUNDARY_WARNING",
        SafeMetadata => "SAFE_METADATA",
    }
}

named_enum! {
    /// Why the content is withheld: the first that applies.
    WithheldReason {
        /// A decision blocks the export.
        DecisionBlocked => "decision_blocked",
        /// The route requires the content unchanged and a decision would redact or omit.
        LosslessMaterializationWithheld => "lossless_materialization_withheld",
        /// The controlled copy is not equal to the content as decoded JSON values.
        ControlledCopyDiffers => "controlled_copy_differs",
    }
}

/// The route a content is decided on, its fixed context and the caller's intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RouteRequest<'a> {
    /// A catalogue route id; recorded, not admitted, by the rule.
    pub route_id: &'a str,
    pub export_context: ExportContext,
    /// Supplied by the native caller, never read from content.
    pub explicit_local_private_intent: bool,
    pub require_lossless_materialization: bool,
}

/// What a rule is asked to decide: the route's whole payload, as an ordinary value or as an
/// object whose members a source gives.
#[derive(Clone, Copy)]
pub enum Content<'a> {
    /// A decoded JSON value, held whole.
    Value(&'a Value),
    /// An object too large to hold as one value. See [`SourceObject`].
    Object(&'a dyn SourceObject),
}

impl<'a> From<&'a Value> for Content<'a> {
    fn from(value: &'a Value) -> Self {
        Content::Value(value)
    }
}

/// The greatest number of source objects and source lists nested inside one another that a
/// rule walks; a source nested deeper is refused
/// ([`RuleError::ContentSourceInconsistent`]). Ordinary values inside a source are not
/// counted and may be of any depth.
pub const MAX_SOURCE_NESTING: usize = 64;

/// A walk that did not reach its end: its visitor stopped it, or its source could not go
/// on. Nothing a stopped walk wrote is exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalkStopped;

/// An object whose members its source gives: small members by name, and every member in
/// order by a walk the source itself drives. The rule reads members by name and visits them
/// in order; it reads nothing else of a source.
///
/// A source promises:
///
/// - [`member_names`](SourceObject::member_names) gives every member name, each once.
/// - [`peek`](SourceObject::peek) answers for exactly those names, gives the same answer
///   each time, and writes nothing.
/// - [`walk`](SourceObject::walk) hands the visitor every member once, in ascending UTF-16
///   code unit order of its name (the order of canonical JSON), and returns `Ok` only when
///   it has handed over every member and the visitor stopped none. A member peeked as a
///   value is walked as an equal value; one peeked as an object or a list is walked as one.
/// - Every source object or list it hands the visitor has been walked to its end, once,
///   when the visitor returns `Ok`.
/// - Objects and lists it gives as sources nest to at most [`MAX_SOURCE_NESTING`].
///
/// A rule checks what it can see of these and refuses a source that breaks one.
pub trait SourceObject {
    /// Every member name, each once, in any order.
    fn member_names(&self) -> Vec<String>;

    /// The member so named, without writing anything: its value where the source holds it
    /// as a small value, its kind alone where the source gives it only by walk, and `None`
    /// exactly for a name that is not a member.
    fn peek(&self, name: &str) -> Option<Peek<'_>>;

    /// Hands `visit` every member once, in order.
    fn walk(
        &self,
        visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped>;
}

/// A list whose entries its source gives one at a time, by a walk the source drives. Each
/// entry lives for its call.
pub trait SourceList {
    /// Hands `visit` every entry once, in index order, and returns `Ok` only when it has
    /// handed over every entry and the visitor stopped none.
    fn walk(
        &self,
        visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped>;
}

/// What [`SourceObject::peek`] answers for a member.
#[derive(Debug, Clone, PartialEq)]
pub enum Peek<'a> {
    /// The member's value.
    Value(Cow<'a, Value>),
    /// An object the source gives only by walk.
    Object,
    /// A list the source gives only by walk.
    List,
}

/// A member or a list entry as a walk hands it over.
#[derive(Clone, Copy)]
pub enum Member<'a> {
    Value(&'a Value),
    Object(&'a dyn SourceObject),
    List(&'a dyn SourceList),
}

/// A SHA-256 digest, written `sha256:` and 64 lowercase hexadecimal digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Sha256Digest([u8; 32]);

impl Sha256Digest {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Sha256Digest(bytes)
    }

    /// The digest written `sha256:<64 lowercase hexadecimal digits>`, or none.
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix("sha256:")?.as_bytes();
        if hex.len() != 64 {
            return None;
        }
        let digit = |byte: u8| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let mut bytes = [0u8; 32];
        for (index, pair) in hex.chunks(2).enumerate() {
            bytes[index] = (digit(pair[0])? << 4) | digit(pair[1])?;
        }
        Some(Sha256Digest(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("sha256:")?;
        for byte in self.0 {
            write!(formatter, "{byte:02x}")?;
        }
        Ok(())
    }
}

impl Serialize for Sha256Digest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// The number of findings with one code and one class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct FindingCount {
    pub code: ReasonCode,
    pub class: FindingClass,
    pub count: u64,
}

impl FindingCount {
    /// The order of [`SummaryOutcome::finding_counts`]: by the code's text, then by the
    /// class's text, each compared as bytes. It is not the order of the schema's lists.
    pub fn summary_order(&self, other: &Self) -> std::cmp::Ordering {
        (self.code.as_str(), self.class.as_str()).cmp(&(other.code.as_str(), other.class.as_str()))
    }
}

/// A blocking finding as an outcome lists it: its reason code and its path, never a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BlockingFinding {
    pub reason_code: ReasonCode,
    pub path: String,
}

/// What the rule decides for one content, before it is bound to the bytes decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryDecision {
    pub route_id: String,
    pub export_context: ExportContext,
    pub explicit_local_private_intent: bool,
    pub rule_revision: String,
    pub withheld: bool,
    /// Some exactly when `withheld` is true.
    pub withheld_reason: Option<WithheldReason>,
    /// The number of decisions the rule made, inclusions counted.
    pub decision_total: u64,
    /// One entry per (code, class) pair with a count of at least one, in
    /// [`FindingCount::summary_order`].
    pub finding_counts: Vec<FindingCount>,
    /// The first `blocking_bound` blocking findings, in the rule's order.
    pub blocking_findings: Vec<BlockingFinding>,
    /// The true number of blocking findings.
    pub blocking_finding_total: u64,
}

impl SummaryDecision {
    /// The decision as the outcome for the bytes whose SHA-256 is `text_sha256`: the one
    /// buffer the content was read from and the sink will receive, or, for a source, the
    /// bytes written as it was walked.
    pub fn bound_to(self, text_sha256: Sha256Digest) -> SummaryOutcome {
        SummaryOutcome {
            route_id: self.route_id,
            export_context: self.export_context,
            explicit_local_private_intent: self.explicit_local_private_intent,
            rule_revision: self.rule_revision,
            text_sha256,
            destination: None,
            withheld: self.withheld,
            withheld_reason: self.withheld_reason,
            decision_total: self.decision_total,
            finding_counts: self.finding_counts,
            blocking_findings: self.blocking_findings,
            blocking_finding_total: self.blocking_finding_total,
        }
    }
}

/// A decision bound to the bytes decided. Every member is always present when serialised,
/// `destination` and `withheld_reason` as `null` when absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SummaryOutcome {
    pub route_id: String,
    pub export_context: ExportContext,
    pub explicit_local_private_intent: bool,
    pub rule_revision: String,
    /// Of the bytes decided; supplied by the caller ([`SummaryDecision::bound_to`]).
    pub text_sha256: Sha256Digest,
    /// None until the caller sets it ([`SummaryOutcome::with_destination`]).
    pub destination: Option<String>,
    pub withheld: bool,
    /// Some exactly when `withheld` is true.
    pub withheld_reason: Option<WithheldReason>,
    /// The number of decisions the rule made, inclusions counted. A decision that includes
    /// a value has no finding, so the finding counts do not give this number.
    pub decision_total: u64,
    /// One entry per (code, class) pair with a count of at least one, in
    /// [`FindingCount::summary_order`].
    pub finding_counts: Vec<FindingCount>,
    /// The first `blocking_bound` blocking findings, in the rule's order.
    pub blocking_findings: Vec<BlockingFinding>,
    /// The true number of blocking findings.
    pub blocking_finding_total: u64,
}

impl SummaryOutcome {
    /// Whether this outcome admits the bytes it is bound to: not withheld, with no withheld
    /// reason and no blocking finding listed or counted. A caller asks this, not `withheld`
    /// alone, so that an outcome that contradicts itself exposes nothing.
    pub fn may_expose(&self) -> bool {
        !self.withheld
            && self.withheld_reason.is_none()
            && self.blocking_finding_total == 0
            && self.blocking_findings.is_empty()
    }

    /// The same outcome, for the destination the caller chose.
    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }
}

/// What the rule refuses to decide. A caller treats every refusal, including one added
/// later, as content withheld.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RuleError {
    /// The `user_released_report` context or the user-release route, which only the
    /// user-release projection admits.
    UserReleaseRouteRequired { route_id: String, export_context: ExportContext },
    /// User-release intent with a context other than `user_released_report`.
    /// [`ContentRule::decide_summary`] never returns it: a [`RouteRequest`] carries no
    /// user-release intent, and intent is never read from content. It is the refusal of the
    /// rule's entries that take that intent from their caller.
    UserReleaseIntentOutsideContext { export_context: ExportContext },
    /// A content source that broke a promise of [`SourceObject`] or [`SourceList`], or whose
    /// walk stopped; `path` is where the rule saw it. It is a defect of the source, never a
    /// property of the content.
    ContentSourceInconsistent { path: String },
}

impl RuleError {
    /// The code of the refusal.
    pub fn code(&self) -> &'static str {
        match self {
            RuleError::UserReleaseRouteRequired { .. } | RuleError::UserReleaseIntentOutsideContext { .. } => {
                "USER_RELEASE_ROUTE_REQUIRED"
            }
            RuleError::ContentSourceInconsistent { .. } => "EXPORT_CONTENT_SOURCE_INCONSISTENT",
        }
    }
}

impl fmt::Display for RuleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleError::UserReleaseRouteRequired { route_id, export_context } => write!(
                formatter,
                "USER_RELEASE_ROUTE_REQUIRED: only the user-release projection admits the user_released_report context or the user-release route (given {export_context} on {route_id})"
            ),
            RuleError::UserReleaseIntentOutsideContext { export_context } => write!(
                formatter,
                "USER_RELEASE_ROUTE_REQUIRED: explicit user-release intent is admitted only in the user_released_report context, not in {export_context}"
            ),
            RuleError::ContentSourceInconsistent { path } => write!(
                formatter,
                "EXPORT_CONTENT_SOURCE_INCONSISTENT: the content source did not keep its promises at {path}; nothing was decided"
            ),
        }
    }
}

impl std::error::Error for RuleError {}

/// The export control's content rule, as a native caller uses it.
pub trait ContentRule {
    /// The rule's identifier. It changes with any change that can alter the presence,
    /// contents or order of a finding.
    fn rule_revision(&self) -> &'static str;

    /// Decides `content`, the whole payload of the route, on the route, context and intent
    /// of `request`. Keeps counts and at most `blocking_bound` blocking findings, never one
    /// record per value, and holds nothing that grows with the number of values: a source
    /// is decided as it is walked. The caller binds the decision to the bytes decided
    /// ([`SummaryDecision::bound_to`], the crate's "One buffer").
    fn decide_summary(
        &self,
        content: Content<'_>,
        request: &RouteRequest<'_>,
        blocking_bound: usize,
    ) -> Result<SummaryDecision, RuleError>;
}
