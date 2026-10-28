---
doc_id: OPS-SECURITY-REDACTION-EXPORT-CONTROLS
doc_kind: security.redaction_export_controls
status: draft
created: 2026-05-03
deliverable_id: DEL-12-02
package_id: PKG-12
scope_items:
  - SOW-040
objectives:
  - OBJ-010
refs:
  - rel: governed_by
    to: OPS-CONTRACT
  - rel: informed_by
    to: OPS-SECURITY-LOCAL-FIRST-STORAGE-POLICY
  - rel: informed_by
    to: OPS-SECURITY-THREAT-MODEL
---

# Redaction And Export Controls

This document defines the `DEL-12-02` redaction and export-control behavior for
public reports, public examples, shared model exports, downstream-tool exports,
and local/private exports. The control is local-first and operates on an
export/report representation. It does not mutate source project data, move
quarantine material, transmit data to a cloud service, decide legal rights, or
provide professional approval.

## Classification Basis

Redaction decisions use explicit metadata only. The control does not infer
sensitivity from engineering-looking values, filenames, text patterns, or
hidden guesses.

A value-bearing export field should carry:

- `field_class`;
- `privacy_classification`;
- `redistribution_status`;
- `review_status`;
- `provenance` where the source is not self-evident public metadata.

The control also consumes explicit storage/privacy indicators when present:
`contains_payload`, `payload_present`, `secret_material_present`,
`direct_sql_access`, storage-bypass flags, `cloud_or_network_reference`,
`concrete_path_present`, symbolic path/locality metadata, and local/private
intent metadata. These indicators are treated as metadata gates only. They do
not authorize payload storage, direct SQLite access, raw SQL access, network or
cloud behavior, concrete user-path disclosure, secret handling, or source-data
mutation.

Absent, unknown, `TBD`, pending, rejected, or quarantined metadata does not
silently pass into public/shared exports. It produces warning, redaction, or
blocking behavior.

## Export Contexts

| Context | Default handling |
|---|---|
| `public_report` | Include public metadata and invented public examples; redact private or unresolved values; block suspected protected content and professional-boundary claims. |
| `public_example` | Same as `public_report`; examples must be invented or public-permissive reviewed content. |
| `shared_model` | Redact private project, material, component, rule-pack, owner-standard, design-basis, path, and secret-like data unless metadata supports sharing. |
| `downstream_tool` | Redact unresolved or private values unless explicit metadata supports the handoff. |
| `local_private` | Retain private values only when explicit local/private user intent is recorded; emit warning findings. |
| `user_released_report` | A report of the user's own session in which the user releases their own project data for sharing, for that one report, with their statement that they hold the redistribution right (`docs/IP_AND_DATA_BOUNDARY.md` §6). Admitted only on route `DREP-PACKAGE-USER-RELEASE-010` with the wrapper's user-release intent. After the professional-claim, network, SQL, storage-bypass, secret, payload, protected and quarantined/rejected checks, which apply as in every context, it decides in this order (and a concrete path then redacts only what (10)-(13) did not block; see Input Contract): (10) a private class other than `private_project_data` is redacted (`PRIVATE_DATA_REDACTED`) whatever its redistribution status, so a lossless route withholds it; (11) `private_project_data`, whatever its redistribution status, is retained with a `USER_RELEASED_PROJECT_DATA` warning only with the wrapper's user-release intent, else blocks (`USER_RELEASE_INTENT_REQUIRED`); (12) public metadata and invented public examples are included; (13) anything else, including missing, unknown, `TBD` or unrecognized metadata, blocks (`USER_RELEASE_VALUE_UNBOUND`), never warning-only. Payload-carried intent of any name is not read. |

The five existing contexts keep every decision on every input except where the
Input Contract below makes it stricter (for example, private data with
`concrete_path_present: true` and no local intent now blocks where it was
redacted), and the release intent cannot enable export in them: the controls refuse
`explicit_user_release_intent` with any context other than
`user_released_report` (DEL-08-01 Design `report-user-release.md` §3), and
local-first route admission refuses it there with `block_storage` /
`USER_RELEASE_ROUTE_REQUIRED` on every route; an absent or false flag leaves
the route evidence unchanged. No route uses `user_released_report` yet.
The redaction walk (`redactExportPayload` / `redact_export_payload`) refuses
`user_released_report` outright, whatever the intent and the payload: it raises
`USER_RELEASE_ROUTE_REQUIRED` after the context and user-release intent checks
and before any walk. The walk decides a value-bearing object as a whole and
keeps an unlabelled value, so in that context it would let a labelled object
hide a restricted descendant (a nested marker, protected or quarantined
content, or a non-project private class) and would leave a private object's
other members unredacted. The classification of one item
(`classifyExportItem` / `classify_export_item`) still decides an item by the
rule above, and the five existing contexts' walk is unchanged.

## Protected Classes

The default protected classes are:

- private project data;
- private material data;
- private component data;
- private rule-pack data;
- owner-standard data;
- company design-basis data;
- path data;
- secret-like data;
- suspected protected content;
- unresolved provenance or redistribution status.

Safe metadata such as IDs, versions, checksums, source notes, warning summaries,
schema versions, and provenance summaries may remain visible where metadata
marks them public or invented public examples.

## Actions

| Action | Meaning |
|---|---|
| `include` | Export the field as-is because metadata permits it. |
| `warning_only` | Retain the value and emit a warning, limited to local/private exports with explicit intent or unresolved metadata in local/private context. |
| `redact_value` | Replace the field value or text with a redaction marker while keeping non-sensitive metadata. |
| `redact_field` | Keep only minimal field identity and classification metadata. |
| `omit_field` | Remove the field from the export representation. |
| `block_export` | Mark the export as blocked until metadata, rights, protected-content, or professional-boundary issues are resolved. |

## Diagnostics

Findings are machine-readable and include a reason code, finding class,
severity, affected path, action, message, and remediation. The expected finding
classes include `IP_BOUNDARY_WARNING`, `PRIVATE_DATA_WARNING`,
`PROVENANCE_WARNING`, `STORAGE_BOUNDARY_WARNING`, and
`PROFESSIONAL_BOUNDARY_WARNING`.

Unknown provenance, unknown redistribution status, missing metadata, rejected
review status, quarantined review status, suspected protected content, and
professional-boundary claims are explicit findings. A clean redaction run is
review evidence only, not a legal or professional clearance.

Storage/privacy hardening adds blocking findings for payload material, explicit
secret-material flags, cloud or network references, direct SQL or raw SQLite
access, and storage-bypass metadata. Concrete path indicators are reduced to a
redacted representation so concrete user paths are not emitted, unless the item
blocks for another reason (see Input Contract).

## Application Binding

The desktop technical preview binds this contract into the export workflow
(TP-E4-REDACTION-001):

- `apps/desktop/src/features/redaction-controls/redactionExportControls.ts` is
  a TypeScript mirror of `core/security/redaction/controls.py`. Semantic
  parity between the two implementations is pinned by the shared invented
  corpus `fixtures/redaction_export_controls/cases.json`, which both
  `tests/security/test_redaction_export_controls.py` and
  `apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`
  assert against; the corpus exercises every governed reason code.
- `apps/desktop/src/features/redaction-controls/RedactionExportControlsPanel.tsx`
  is the user-facing surface: the export context starts at `local_private`
  (private-by-default), every warning and blocking finding is rendered before
  any export artifact is offered, no download link exists while a blocking
  finding is present, private values are retained locally only after the user
  records explicit local-private intent, and the offered redacted manifest
  carries a canonical sha256 checksum computed through the same wasm hash seam
  the engines use.

The application binding classifies a metadata-only export representation of
the user-entered model; it never mutates the source model and never transmits
anything. Its warnings inform the local export decision only — they do not
certify redaction sufficiency and create no release or legal-clearance claim.

## Final-Sink Route Binding

The DEL-12-02 breadth tranche applies the contract at every frozen final
exposure route. Each binding returns an observable controlled envelope before
creating a download, preview, print action, stdout value, output file, or
downstream handoff:

```text
ControlledExport<T> = {
  payload: T | null,
  decisions: RedactionDecision[],
  findings: RedactionFinding[],
  blocked: boolean,
  summary: RedactionSummary
}
```

The route projector deep-copies the source, assigns explicit metadata to each
value-bearing leaf, removes top-level and nested `local_private_intent`,
`explicit_local_private_intent`, and `user_intent`, and supplies only the
route's fixed context and wrapper-owned intent. Source-carried intent therefore
cannot grant export authority. The projector materializes only the controlled
copy and preserves the source object byte/deep-equal.

User-release intent comes only from the wrapper's argument
(`explicitUserReleaseIntent` / `explicit_user_release_intent`), never from the
payload. The strip list above is unchanged: a payload member named
`user_release_intent` or `explicit_user_release_intent` grants nothing, so it is
not stripped, and in every existing context and route it is raw content,
preserved byte for byte. The user-release route `DREP-PACKAGE-USER-RELEASE-010`
has its own projector (`controlUserReleasedReportPackageRequest`), the only
payload entry for `user_released_report`; the redaction walk refuses the
context (above). The generic route exporter
(`controlRouteExport` / `control_route_export`), whose projector labels values
by path tokens, refuses that context on every route and route 010 in every
context, before any projection: it raises
`USER_RELEASE_ROUTE_REQUIRED`, as it raises for an unsupported context, after
the user-release intent check. The user-release projector first
decides every enclosing object of the request and of the model by the
professional-claim, network, SQL, storage-bypass, secret, payload, protected
and quarantined/rejected checks and by a concrete path, exactly as the
classification decides that object; a path-marked object that states no label
of its own is therefore refused by step 13 (`USER_RELEASE_VALUE_UNBOUND`). Only then does it label each leaf with the
privacy and redistribution of its nearest enclosing object that states them
(its own member, else its provenance's, as the classification reads them),
else `private_project_data` / `private_only`, with no path-token labelling.
Only an absent label inherits: an object's explicitly stated privacy or
redistribution that the classification does not recognize (for example `{}`,
`[]`, a number, an empty or unknown string, or a null or `unknown` privacy)
blocks at that object (`USER_RELEASE_VALUE_UNBOUND`), whatever its descendants
state. It
is lossless, and its local-first admission holds the route metadata under the
existing `private_report` storage context.

The frozen route set covers Python JSON/downstream/specialized-format writers,
both Rust headless runners, report DOM/JSON/render-IPC/HTML/save/print/lint,
and all other desktop download panels. Structured routes may emit a sanitized
payload. Lossless specialized formats are withheld if any destructive action
would corrupt their meaning. Local-private routes expose known private values
only after their own explicit-intent control is selected, with
`warning_only` evidence; unresolved local values remain unknown and
warning-only.

Desktop download links use `ControlledExportLink`. Reports are controlled
before DOM construction and before the Tauri render IPC call; blocked reports
have no raw DOM, iframe, JSON link, save link, print action, renderer call, or
browser fallback. Renderer and protected-content-lint diagnostics are composed
with the route findings when a sanitized report proceeds. The Python writers
and downstream workflow expose the same envelope and create no directory or
file on block. The Rust runners emit the same controlled shape, accept
wrapper-owned `--explicit-local-private-intent`, and write no output on block;
their mirror is pinned to the same invented parity corpus as Python and
TypeScript.

The bounded Rust mirror's departures in the five existing contexts were
resolved by the W-18b Rust-mirror alignment (2026-10-27). It now reads the
inputs described here as Python and TypeScript do: it screens `cache_payload` and
`sqlite_payload`, falls back to the provenance for a null privacy or
redistribution member and for an `unknown` or null review status, and counts a
scalar storage-privacy metadata key as metadata present. The alignment's
differential also found, and the mirror now follows, two further readings: any
safe privacy class with any public redistribution status is included (for
example `public_metadata` with `invented_non_engineering_example`), and only an
absent, null, `unknown` or `TBD` privacy or redistribution is unknown, so a
stated non-string label is decided as Python and TypeScript decide it.
Existing-context parity cases hold all three implementations to these inputs.
The mirror is test-only; the runners' fixed `control_local_private` projection
does not use it and is unchanged.

The protected-content lint CLI remains a separately governed DEC-058/DEC-059
diagnostic used only by the release scan and public-source exporter; it is not
a product report/model/result route and this tranche does not change it. No
plugin runtime/loader or bug/crash-report egress exists. Any future such route
must consume this controlled-export boundary before it can expose a payload.

## Input Contract

One input contract governs the classifier's reading of item metadata, and
Python (`core/security/redaction/controls.py`), TypeScript
(`redactionExportControls.ts`) and the test-only Rust mirror
(`core/runner/headless/src/redaction_binding.rs`) follow it identically
(HELP_HUMAN ruling under the owner's delegation, 2026-10-27). It governs
classifier input that no schema validates. The labels are already typed where
they are persisted: `schemas/model_state.schema.json` `$defs/Provenance`
(`additionalProperties: false`) types `privacy_classification`,
`redistribution_status` and `review_status` as string enums, and
`analysis_run.v0.3` fixes `professional_claim` to `const: false`. The flags
below are typed as item members in no persisted schema; `ExportRun` records
only the wrapper's `explicit_local_private_intent`, as a boolean.

**Decision order.** In every context the classifier decides, in order:

1. professional claim → `block_export` / `PROFESSIONAL_BOUNDARY_BLOCKED`;
2. cloud or network reference → `block_export`;
3. direct SQL access → `block_export`;
4. storage bypass → `block_export`;
5. secret material → `block_export`;
6. contained payload → `block_export` / `PAYLOAD_METADATA_ONLY_REQUIRED`;
7. privacy or redistribution `protected_suspected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`;
8. review status `quarantined` or `rejected` → `block_export` /
   `PROTECTED_CONTENT_BLOCKED`;
9. the context's own decision: steps 10-13 in `user_released_report`, else
   missing metadata, unknown redistribution, unknown privacy, the safe include,
   the private branch (`LOCAL_PRIVATE_INTENT_REQUIRED` without intent in
   `local_private`) and the fallback;
10. a concrete path → `redact_value` / `CONCRETE_PATH_REDACTED`, unless step 9
    blocked.

A concrete path never turns a block into a redaction. Before the contract it
was decided right after the payload check and so returned `redact_value` ahead
of the protected, quarantined/rejected, intent and user-release blocks; for
example `concrete_path_present: true` downgraded a `protected_suspected` block.

**Labels.** A `privacy_classification` or `redistribution_status` that is
neither a string nor null (an array, object, number or boolean) is a stated
label that matches no class: it is not unknown, safe, private or protected, and
it does not fall back to the provenance, so the item takes the fallback decision unless its other
label decides it. A `review_status` that is not a string reads as `unknown` and
so gives way to the provenance's. The same holds on the item and in its
provenance, and for the route projector's public basis: `["public"]` is not
public. Python no longer raises `TypeError` on an array or object label. A
null label is unchanged: it falls back to the provenance, as an absent one does,
so null privacy and redistribution under public, permissive provenance are
included (`SAFE_PUBLIC_METADATA`).

**Alias.** The route projector reads a record's privacy for its direct public
basis from `privacy_classification`, and from the `classification` alias only
when `privacy_classification` is absent. A present primary label, null
included, is the label and the alias is ignored, so a null primary label cannot
let the alias lift a sibling into a public basis (HELP_HUMAN decision under the
owner's delegation, 2026-10-27). TypeScript formerly read the alias with `??`,
so `{"privacy_classification": null, "classification": "public",
"redistribution_status": "public_permissive", "note": ...}` included `note` in
`public_report` on `DREP-JSON-002`; Python read it as now, and the test-only
Rust mirror carries the same reading for the sweep's projector cases. The
projector's public basis does not read the record's provenance, so a null
primary label gives the record no public basis: its siblings are labelled by
their paths, and an unknown leaf is redacted (`REDISTRIBUTION_STATUS_UNKNOWN`)
in the public and shared contexts and warning-only in `local_private`. The item
classifier reads no alias: a null `privacy_classification` falls back to the
provenance as above, never to `classification`. The user-release projector's
`privacy_class` alias is likewise read only when `privacy_classification` is
absent from the object and its provenance. A `__proto__` member is an own
member in all three: TypeScript copies input members by definition, not
assignment, so no JSON member becomes a prototype (HELP_HUMAN decision,
2026-10-27).

**`__proto__` member.** An object that carries an own `__proto__` member, on
itself or on its `provenance` object, fails closed on the key itself, in all
three implementations (HELP_HUMAN ruling, 2026-10-27: "option (b), fail closed
on the key itself, in all three implementations. A record carrying an own
`__proto__` member gets no direct public basis, and its siblings are labelled by
path. The item classifier treats the member as an unknown label."). The reason,
as ruled: "a `__proto__` key in export data is anomalous, and it may never lift
or restrict a record by accident". The rule applies in each place that reads labels:

- *Route projector.* A record that carries the member has no direct public
  basis, whatever its own `privacy_classification`, `classification` alias or
  redistribution state. Its siblings are labelled by their paths, as under a
  null primary label.
- *Item classifier.* The member reads as an unknown label only where the
  decision would otherwise be an inclusion. It is the last step of the decision
  order, after the concrete-path redaction:
  - in the five existing contexts a final `include` becomes the unknown-label
    decision (`redact_value`, `warning_only` in `local_private`,
    `UNKNOWN_PROVENANCE_WARNING`);
  - in `user_released_report` a final `include` becomes
    `USER_RELEASE_VALUE_UNBOUND`;
  - every other decision stands, the concrete-path redaction and every earlier
    block or redaction included.
  
  Replacing the item's stated labels with `unknown` was not chosen: it would
  turn a protected block into a redaction, and a private label's
  `LOCAL_PRIVATE_INTENT_REQUIRED` block into a warning.
- *User-release screen* (TypeScript only). The member reads as a stated
  unknown label, so the object blocks by step 13 and no descendant inherits a
  label past it.
- *Not affected.* `export_policy` carries no labels; only its intent keys are
  read. The runners' `control_local_private` reads no labels.

In TypeScript the plain-data gate (below) runs before the rule. It refuses a
non-enumerable member, so the member the rule reads is always an own,
enumerable data member, as JSON parsing makes it. Python and Rust can be
handed a programmatically built mapping (a dict, a serde_json map) as well as
parsed JSON, but neither has non-enumerable keys, so a `__proto__` key is
always visible there.

This is deliberately stricter than Python's previous reading, which took the
key as an ordinary member. The member itself is kept in the payload, and its
leaves are labelled like any other.

**Value records.** The route projector decides whether a `{value, ...}` record
with a scalar `value` is one item or a record whose members it labels by path.
Before, TypeScript made it an item if it carried any metadata key, and Python
only if it carried a scalar privacy, redistribution or review label, on itself
or its provenance. Neither was uniformly stricter: `{value, field_class, note}`
left `note` visible in TypeScript, and `{value, professional_claim: true}`
blocked in TypeScript but was only redacted in Python. All three
implementations now take the stricter decision per case (HELP_HUMAN decision,
2026-10-28: "take the stricter decision per case"). A record with a scalar
label is still an item. A record with only other metadata is a *value record*,
and the redaction walk decides it as follows:

- *Block.* It classifies the record as one item first. A block is the whole
  record's: one decision on the record, also where the projection would have
  blocked on a leaf.
- *Otherwise.* It decides the record as projected, and then:
  - every `value` and `text` leaf is raised to the item decision where that is
    stricter, keeping the item's own reason;
  - when the item decision redacts, the `privacy_classification` leaves are
    raised too, since the item's redaction replaces that member;
  - when the item decision redacts, every unsafe path key is dropped from the
    payload, key name included, as `omit_field` with the item's reason, unless
    its leaf blocks.
- *Whole subtree.* A raised member's restriction covers its whole subtree
  (Codex privacy review of #1722). Every decision beneath it is raised.
  Beneath a redacting floor the whole member is replaced by `[REDACTED]` once
  its decisions are taken: a nested item's extra metadata and a nested
  record's members go with it, and a block beneath still blocks.
- *Inherited floors.* A value record nested in a raised member, in an object,
  an array or a label member, keeps the enclosing floor and omission. The
  stricter of the enclosing floor and its own applies, and on a tie the
  enclosing one.
- *Ties.* Where both readings showed the same data, the decision records take
  the projection's form.

The walk knows the projector's value-record node by identity, a module-private
`WeakSet` in TypeScript and a private class in Python, never by a member an
input could carry. An input record carrying the old `_route_value_record`
marker with `item` and `projected` members is decided as itself. The
materializer still reads the `_route_projected_leaf` and `__route_key__`
markers by name, after every decision is taken. An input can carry them only
inside an item, where they can drop or rename that item's members but never
skip or weaken a decision.

The Rust test mirror now carries the projector and this walk.

Over a generated grid of 24,608 `{value, ...}` records in ten contexts (246,080
cases), the two readings diverged in 30,950 cases:

| Class | Cases | Decision taken |
|---|---|---|
| TypeScript blocked, Python did not | 20,880 | block |
| Python hid more | 6,200 | projection |
| Python blocked | 774 | projection |
| Both blocked | 2,320 | the item's block |
| Each hid something the other showed | 572 | the union |
| Same visible data | 204 | projection's form |

Over that grid the exported payload is identical in TypeScript and Python, and
less strict than neither reading: it exposes no key path or visible leaf that a
reading hid, and it does not unblock what a reading blocked.

The committed sweep holds a pinned subset of 7,730 cases
(`fixtures/redaction_export_controls/value_record_sweep.json` and
`value_record_sweep_decisions.json`): every record setting at most three axes,
plus extra records, in ten contexts. The extra records include nested value
records, arrays of them, raised items with extra metadata, the three repros of
the Codex review and records carrying the reserved key prefix. The suites check
the full exported payload, every key path and visible leaf, and the normalized
decisions:

- TypeScript and Python give the contract payload and decisions case for case.
  The Rust mirror, which does not materialize a payload, gives the contract
  decisions case for case and has no before column.
- No case is less strict than Python before. Exactly one case is less strict
  than TypeScript before, and it is pinned: `{value: "outer", field_class,
  "__route_key__value": "SECRET"}` in `local_private` with intent.
  TypeScript then exported only `value: "SECRET"`, because the input member's
  reserved prefix collided with the item's own `value`. The item's own value
  `"outer"`, which its decision kept, is now exported beside it, as Python
  exported both.
- The sweep is stricter than Python in 1,735 cases and than TypeScript in
  1,574.

No existing sweep case changes.

**Flags.** All three recognize the strings `true`, `yes`, `explicit`,
`present`, `local_private` and `explicit_local_private`, compared after
trimming and lowercasing. Python and JavaScript lowercase by Unicode and Rust by
ASCII; no non-ASCII code point lowercases into these strings in Python or
JavaScript (checked over every code point), so the three agree.

- *Blocking flags* (`contains_payload`, `payload_present`,
  `secret_material_present`, `cloud_or_network_reference`,
  `direct_sql_access`, `storage_bypass`, `storage_bypass_requested`,
  `direct_storage_bypass`, `application_service_bypass`,
  `concrete_path_present`) are set by `true`, by a non-zero number, or by a
  recognized string after trimming the union of what the three trims removed
  before the contract: U+0009-U+000D, U+001C-U+001F, U+0020, U+0085, U+00A0,
  U+1680, U+2000-U+200A, U+2028, U+2029, U+202F, U+205F, U+3000 and U+FEFF.
  No implementation reads a blocking flag as unset where it read it as set.
- *Intent flags* (`local_private_intent`, `explicit_local_private_intent`,
  `user_intent`, on the item, else in `export_policy`) are set only by `true` or
  by a recognized string after trimming ASCII whitespace (U+0009-U+000D,
  U+0020). A number, any other whitespace, an array or an object is not intent;
  a present member that is not intent still overrides the wrapper's intent, as
  before.

**Professional claim.** `professional_claim` blocks unless it is absent, null or
`false`. `"false"`, `"no"`, `""`, `0`, `[]` and `{}` all block.

**Proof.** `fixtures/redaction_export_controls/input_contract_sweep.json`
expands to 100,744 cases in each implementation's test
(`tests/security/test_redaction_input_contract_sweep.py`,
`redactionInputContractSweep.test.ts` and the Rust test module): the 74,760
cases of the contract; 7,840 alias cases, 5,040 classifier items and 2,800
records decided by the route projector on `DREP-JSON-002` in its five contexts ×
two intent settings; and 18,144 `__proto__` cases, 8,064 classifier items and
10,080 projector records carrying a `__proto__` member. `input_contract_sweep_decisions.json` freezes each implementation's
decision per case before the contract, captured from the real code, the Rust
from the real test-only mirror; the alias cases before the alias rule. The Rust
mirror then had no route projector, so its column holds no decision for the
2,800 projector cases, which are held to the Python and TypeScript columns. The
alias rule made 120 TypeScript projector cases stricter (a null primary label
with a safe alias and a public redistribution: `include` became `redact_value`
in 96 and `warning_only` in 24, in `local_private`) and changed no other
decision. Before the `__proto__` repair the TypeScript projector copied a record
by assignment, so a `__proto__` member became the copy's prototype or was
dropped: its frozen decisions on the 10,080 `__proto__` projector records are of
a different record than the input, and those cases are held to the Python
column instead (the Rust mirror then had none). Against that set-aside column the
own-member copy alone was stricter in 780 cases, where the member's public labels had lifted a
sibling into a public basis, and less strict in 120, where the member's
restrictive privacy had hidden the record's own safe `classification` alias and
public redistribution. Under the fail-closed rule the contract is stricter
than that set-aside column in 1,920 projector cases, the 780 and the 120
included, and less strict in none. It is also less strict than the Python
column in no case, and stricter in 1,932 `__proto__` cases (1,260 projector
records and 672 classifier items), as the rule intends. Each test fails if any case is decided less strictly than any
frozen decision it is held to, and requires the contract decision for every case, so the three
agree case for case. Strictness is the total order `include` < `warning_only` <
`redact_value` < `block_export`. `redact_value` sits above `warning_only` because
`warning_only` keeps the value and is the unknown-metadata action only in
`local_private`, while `redact_value` replaces `value` and `text` and is that
action in every public or shared context. The classifier produces no
`redact_field` or `omit_field`. The shared corpus's `input-contract-*` cases
pin one case per rule for all three suites.

"Less strictly" compares actions among decisions that existed before. It does
not cover inputs on which Python raised `TypeError` and so produced no output:
those now take the decision TypeScript and the Rust mirror already gave, which
can be `include` (192 sweep cases; for example public, permissive labels with
`review_status: []`). That follows the contract's label rule; it is not an
action loosening, but such an output is newly available from Python.

The runners' fixed `control_local_private` projection is untouched: it reads
none of these labels, flags or `professional_claim`. It strips the intent keys
at every depth and labels values by path.

**Plain-data gate (TypeScript).** Every TypeScript entry point checks its input
before reading it: `controlRouteExport`, `redactExportPayload`,
`classifyExportItem` and `controlUserReleasedReportPackageRequest` (its request
and its model). Anything other than a JSON-data graph refuses the whole export
as one `block_export` decision with reason `EXPORT_INPUT_NOT_PLAIN_DATA`, at the
first refused path, with a blocking finding that names the shape (HELP_HUMAN
ruling, 2026-10-28: "(a), the plain-data gate at the TS entry points. It
removes the class instead of modelling JS semantics one primitive at a
time."). A JSON-data graph has:

- *Single read.* The gate is a snapshot taken in one iterative pass. It uses
  an explicit stack, not recursion, and reads each own property's descriptor
  exactly once, validating and copying as it goes. Everything downstream
  classifies and exports only that snapshot, and the input is never read again
  (HELP_HUMAN ruling, 2026-10-28, (i), the single-read snapshot). Any
  exception during the pass, such as a Proxy trap that throws, is a refusal.
- *Containers.* Objects whose prototype is this realm's `Object.prototype` or
  null, and arrays (`Array.isArray`) whose prototype is `Array.prototype`.
  - An array's keys must be real array indices: canonical, below 2^32 − 1 and
    below `length`. Its elements must be contiguous data members from 0 to
    `length` − 1, so a hole is refused.
  - An object with an exotic internal slot is refused by a brand check,
    whatever its prototype: a boxed primitive, Map, Set, WeakMap, WeakSet,
    Date, RegExp, ArrayBuffer, a typed array or a DataView.
- *Values.* Strings, finite numbers, booleans and null. A non-finite number is
  refused. Parsed JSON is not always finite: `JSON.parse("1e400")` gives
  `Infinity`.
- *Members.* Own, enumerable data members only, and no symbol key. They are
  enumerated with `Reflect.ownKeys` and read through their descriptors, so no
  getter, setter or method of the input ever runs. A non-enumerable member is
  refused, `__proto__` included, because parsed JSON has none.

A class instance, a cross-realm object, any of the exotic objects above, an
accessor, a function, NaN or an infinity is refused. What no brand check can
detect, such as a Proxy that presents plain data, is exported as its
single-read plain copy. That copy holds only the own data members read once, so
it can only lose data, never add it.

- *`undefined`.* An object member whose value is `undefined` is absent:
  skipped and never classified (HELP_HUMAN ruling, 2026-10-28: "An undefined
  own data property on an object is treated as absent: skipped, never
  classified, exactly as JSON.stringify writes it. An undefined array element
  still fails closed."). This matches the serialized form, and absence is
  already the stricter path: an absent label reads as unknown.
- *Sharing and cycles.* An object reached twice in an acyclic graph is
  admitted, and the copy duplicates it per position, so redacting one position
  never touches another. A cycle is refused (HELP_HUMAN ruling, 2026-10-28:
  "Admit shared references in an acyclic graph and refuse only cycles."). The
  reason is that JSON writes DAG sharing as duplicates and cannot express a
  cycle. The gate tracks the current path, not every object seen.

After the gate, each entry point works on that per-position copy, never on the
input. Python and Rust receive parsed JSON, which is a JSON-data tree by
construction, so they need no gate.

*Known limits*, pre-existing and outside the ruling:

- Duplicating a shared object per position grows exponentially with the depth
  of nested sharing.
- Nesting deeper than the engine's call stack still overflows in the recursive
  steps downstream of the gate. That raises an exception and exports nothing.

## Source Data

The implementation copies the export/report representation before applying
redaction. Source project models, private libraries, rule packs, report inputs,
and local storage paths are not modified by this control.

## Verification Expectations

DEL-12-02 should be checked for:

- schema traceability to `DEL-12-02`, `PKG-12`, `SOW-040`, and `OBJ-010`;
- explicit metadata only classification;
- redaction of private project, material, component, rule-pack, owner-standard,
  company design-basis, path, and secret-like values in public/shared exports;
- warnings or redaction for missing provenance and unknown redistribution;
- blocking or redaction for payload material, secret-material flags,
  cloud/network references, direct SQL/raw SQLite access, storage bypass
  markers, and concrete path indicators;
- explicit local/private intent before retaining private values;
- source-carried intent ignored at every final route and wrapper intent tested;
- observable decisions, findings, blocked state, and summary before exposure;
- no output directory/file, DOM, IPC, iframe, save, print, stdout payload, or
  browser fallback when blocking findings exist;
- canonical Python, TypeScript, and bounded Rust parity-corpus agreement;
- continued absence of plugin and bug-report egress and release-tool-only use
  of the protected-content diagnostic CLI;
- no cloud transmission, secret handling, source mutation, protected standards
  content, non-invented private payloads, concrete user paths, direct
  SQL/storage bypass, or professional-authority assertions.
