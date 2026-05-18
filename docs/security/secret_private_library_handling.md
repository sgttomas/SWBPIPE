---
doc_id: OPS-SECURITY-SECRET-PRIVATE-LIBRARY-HANDLING
doc_kind: security.secret_private_library_handling
status: draft
created: 2026-05-09
deliverable_id: DEL-12-04
package_id: PKG-12
scope_items:
  - SOW-040
  - SOW-029
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: informed_by
    to: OPS-SECURITY-LOCAL-FIRST-STORAGE-POLICY
  - rel: informed_by
    to: OPS-SECURITY-REDACTION-EXPORT-CONTROLS
---

# Secret And Private-Library Handling

This document defines the bounded `DEL-12-04` reference-handling behavior for
private libraries, private path references, secret-like fields, and credential
placeholders. The control is local-first, metadata-only, and uses invented fixtures only.
It does not store usable credential material, private-library
payloads, private project payloads, protected standards content, or proprietary
source content.

## Records

Reference records carry explicit metadata:

- reference ID, record kind, label, and storage locality;
- privacy classification, redistribution status, and review disposition;
- source/provenance state and source note;
- checksum and checksum status where available;
- value descriptor for an opaque placeholder or symbolic path class;
- unresolved `TBD` entries that must remain visible to reviewers.

The implementation does not infer permission from filename patterns or
engineering-looking values. Absent, unknown, or `TBD` metadata remains an
explicit warning or blocking finding depending on release context.

## Guard Behavior

Public reports may retain metadata-only private-library references, such as an
invented name, version, checksum, and source note, while emitting warnings.
Public fixtures are stricter and block private references; fixture records
should use invented public-example metadata instead of private-kind references.

All release contexts block:

- secret-like material embedded in a reference record;
- private-library payload data embedded in a reference record;
- private path payload data embedded in a reference record;
- rejected, quarantined, or suspected protected source disposition.

Public and shared release contexts also block private data with unresolved
redistribution status or unresolved privacy classification. Local/private use
of private metadata requires explicit local/private intent and still emits a
warning.

## Boundaries

This control does not:

- select operating-system storage roots;
- integrate an external secret manager;
- finalize encryption or key-management policy;
- move quarantine material;
- transmit data to a cloud service;
- decide legal rights;
- assert professional approval or code-compliance status.

Diagnostics identify reference IDs and reason codes only. They must not print
secret-like values, private path payloads, private library payloads, private
project payloads, or protected source content.

## Verification Expectations

DEL-12-04 implementation changes should be checked for:

- traceability to `DEL-12-04`, `PKG-12`, `SOW-040`, `SOW-029`, and `OBJ-010`;
- deterministic classification for repeated invented inputs;
- metadata-only manifests that preserve checksum, source note, classification,
  warning severity, and unresolved `TBD` markers;
- blocking diagnostics for public fixture/export attempts that contain
  secret-like values, private path payloads, private-library payloads, or
  unknown-redistribution private data;
- absence of usable credential material, private payloads, protected standards
  content, cloud behavior, external secret-manager behavior, or professional
  authority assertions.
