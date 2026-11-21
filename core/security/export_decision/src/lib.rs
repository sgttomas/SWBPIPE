//! The types and the trait a native caller of the DEL-12-02 export control's content rule
//! uses: what it asks the rule to decide ([`RouteRequest`]), what the rule returns
//! ([`SummaryOutcome`]) or refuses ([`RuleError`]), and the rule itself as a trait
//! ([`ContentRule`]). It holds no rule. The names are the texts of
//! `docs/security/redaction_export_controls.md` and
//! `schemas/redaction_export_controls.schema.yaml`.
//!
//! # What a caller may rely on
//!
//! - `withheld == false` means that no decision blocks and that the controlled copy equals
//!   the content. Only then may the caller expose the content, and it exposes the bytes it
//!   had decided, bound by `text_sha256`.
//! - `withheld == true`: nothing is exposed. The caller reports `withheld_reason`, the
//!   counts and the bounded blocking findings (reason code and path, never a value).
//! - The content is borrowed and never changed. Intent comes only from the request.
//! - The same content, request and rule revision give the same outcome.
//! - The route id is recorded, not admitted: the caller establishes that its route may run
//!   before it asks for a decision.
//!
//! # What it does not offer
//!
//! No per-value decision or finding records and no controlled copy; no member-at-a-time
//! form (the content is the route's whole payload, because a value's decision depends on
//! its path from the payload's root); no route catalogue, user-release projection, text
//! reading, hashing or write. No I/O, logging, clock or global state.

use std::fmt;

use serde::{Serialize, Serializer};

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
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FindingCount {
    pub code: ReasonCode,
    pub class: FindingClass,
    pub count: u64,
}

/// A blocking finding as an outcome lists it: its reason code and its path, never a value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BlockingFinding {
    pub reason_code: ReasonCode,
    pub path: String,
}

/// What the rule returns for one decided content. Every member is always present when
/// serialised, `destination` and `withheld_reason` as `null` when absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SummaryOutcome {
    pub route_id: String,
    pub export_context: ExportContext,
    pub explicit_local_private_intent: bool,
    pub rule_revision: String,
    /// Of the bytes decided; supplied by the caller.
    pub text_sha256: Sha256Digest,
    /// None until the caller sets it ([`SummaryOutcome::with_destination`]).
    pub destination: Option<String>,
    pub withheld: bool,
    /// Some exactly when `withheld` is true.
    pub withheld_reason: Option<WithheldReason>,
    /// One entry per (code, class) pair with a count of at least one, ordered by code, then
    /// class.
    pub finding_counts: Vec<FindingCount>,
    /// The first `blocking_bound` blocking findings, in the rule's order.
    pub blocking_findings: Vec<BlockingFinding>,
    /// The true number of blocking findings.
    pub blocking_finding_total: u64,
}

impl SummaryOutcome {
    /// The same outcome, for the destination the caller chose.
    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }
}

/// What the rule refuses to decide.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleError {
    /// The `user_released_report` context or the user-release route, which only the
    /// user-release projection admits.
    UserReleaseRouteRequired { route_id: String, export_context: ExportContext },
    /// User-release intent with a context other than `user_released_report`.
    UserReleaseIntentOutsideContext { export_context: ExportContext },
}

impl RuleError {
    /// The contract's code for the refusal.
    pub fn code(&self) -> &'static str {
        "USER_RELEASE_ROUTE_REQUIRED"
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
        }
    }
}

impl std::error::Error for RuleError {}

/// The export control's content rule, as a native caller uses it.
pub trait ContentRule {
    /// The rule's identifier. It changes with any change that can alter the presence,
    /// contents or order of a finding.
    fn rule_revision(&self) -> &'static str;

    /// Decides `content`, the whole payload of the route as one decoded JSON value, on the
    /// route, context and intent of `request`. `content_sha256` is the caller's SHA-256 of
    /// the bytes the content was decoded from (or, for content the caller built, of its
    /// canonical text); it is recorded, not checked. Keeps counts and at most
    /// `blocking_bound` blocking findings, never one record per value.
    fn decide_summary(
        &self,
        content: &serde_json::Value,
        request: &RouteRequest<'_>,
        content_sha256: Sha256Digest,
        blocking_bound: usize,
    ) -> Result<SummaryOutcome, RuleError>;
}
