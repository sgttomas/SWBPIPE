---
name: swbpipe-ip-and-data-boundary
description: Set the public-data, private-input and intellectual-property boundaries.
metadata:
  kind: instruction
  surface: requirements
---

# IP and Data Boundary Policy

This document is governed by [CONTRACT.md](CONTRACT.md).

> **Registry:** This document is the policy depth behind the `BS-IP` boundary
> statement in `docs/claims_registry.md` (D-48 / `DEC-081`).

## 1. Purpose

This policy keeps the public SWBPIPE repository aligned with the project intent: public open mechanics, private/user-supplied code and proprietary data.

This policy is a project governance control, not legal advice, professional engineering approval, certification, sealing, standards-body endorsement, or a code-compliance claim. Human/legal review remains required when redistribution rights or protected-content status are uncertain.

## 1.1 Current authority basis

This draft policy is aligned to
the accepted `execution/_Decomposition/SOFTWARE_DECOMP.md`. The project license has been selected as
`MIT`; contribution-source license and redistribution
fields still record the source basis for submitted material and do not replace
the project license notice.

DEC-027 records a quorum of one: the sole human project authority is the sole
maintainer and release authority. External contribution intake remains closed.
DEC-079 adopts the D-07b sole-maintainer review and protected-content screening
skeleton and records “process defined; intake closed”; activation requires a
future owner decision. The final contributor legal instrument remains `TBD`
until that activation decision, with legal advice obtained by the owner. No
legal instrument or release is authorized by this current-basis correction.

Legal-review authority, security contact, release-label vocabulary,
human-acceptance workflow, and jurisdiction-specific professional-practice
wording retain their own unresolved decisions and qualified-review routes.
See DEC-027/DEC-079 in `execution/_Decomposition/SOFTWARE_DECOMP.md` and
`execution/_Coordination/_DECISIONS/D-07b_contributor_intake_mechanism.md` §10.
Blank per-contribution fields remain unfilled evidence slots.

## 2. Public repository may contain

- solver algorithms based on open mechanics;
- geometry, unit, load, stress-recovery, and report schemas;
- blank templates and private-library import mechanisms;
- invented example data with clear notices;
- public-domain or permissively licensed data with documented provenance;
- validation benchmarks from original, public-domain, or permissively licensed sources;
- review checklists, manifests, and empty schema slots that help users record private/code-specific data without bundling those values publicly.

## 3. Public repository must not contain

- ASME or other standards-body code text, tables, figures, examples, or commentary;
- material allowable tables copied from standards;
- B31J stress-intensification or flexibility-factor content copied from standards;
- B16/B36/MSS/ISO/EN/CSA dimensional or rating tables copied from standards;
- proprietary vendor catalogs without redistribution rights;
- commercial software examples, report templates, or benchmark files without permission;
- user private rule packs, owner standards, or company design bases;
- code-specific load combinations, allowables, SIFs, flexibility factors, acceptance criteria, or owner requirements unless they are invented examples or otherwise cleared for public redistribution.

## 4. Required provenance fields

Every public data record must include:

| Field | Meaning |
|---|---|
| `source_name` | Human-readable source. |
| `source_location` | URL/path/document identifier or `TBD`. |
| `source_license` | Redistribution basis or `TBD`. |
| `contributor` | Person/org submitting. |
| `contributor_certification` | Statement that data is not copied from protected source unless licensed. |
| `redistribution_status` | `public_permissive`, `private_only`, `unknown`, `protected_suspected`. |
| `review_status` | `pending`, `accepted`, `rejected`, `quarantined`. |

Records with `source_license = TBD`, `redistribution_status = unknown`, or missing contributor certification are not acceptable as public data. They remain pending, private-only, rejected, or quarantined until a maintainer records a review disposition.

## 5. Quarantine rule

If data appears to derive from a protected standard or proprietary source, agents and maintainers must:

1. stop ingestion;
2. mark the record `protected_suspected`;
3. move the artifact to `quarantine/protected-content/` or another maintainer-approved non-example quarantine path;
4. record the issue;
5. request human/legal review.

Quarantined material must not be used as public fixture data, copied into documentation, summarized into public tables, or converted into public defaults while its status is unresolved.

## 6. Private user data

Private material libraries, rule packs, component catalogs, and owner design bases belong in user-controlled private paths. The public project may provide schemas and importers, but must not assume those files are redistributable.

Importers, examples, reports, telemetry, and issue templates must default to excluding private project, material, component, rule-pack, owner-standard, and company design-basis data unless a user intentionally exports or contributes it with documented redistribution rights.

### 6.1 Runtime residency vs public redistribution (per `D-T0-04` / `DEC-051`)

The default-exclusion rule above governs *public redistribution and committed/exported artifacts* — it is unchanged. It is distinct from *runtime data residency*. Per the owner's tier-0 ruling `D-T0-04` (open residency), an embedded agent worker MAY read the owner's own private model and private Class-B inputs (allowables, SIFs, design basis) at runtime, and once the owner configures a model provider (local, Anthropic, or other) that data MAY be transmitted to it; the app does not enforce privacy/residency on this channel. This is owner-configured runtime handling of the owner's *own* data — it is NOT a public commit, NOT redistribution, and creates NO exception to §3 (public repository must not contain), §5 (quarantine), or the third-party protected-content prohibitions (`OPS-K-IP-1/2/3`). Third-party copyrighted standards content remains barred from the public repository regardless of this ruling; the owner cannot waive third parties' rights by configuration.

### 6.2 CAEPIPE evaluation (owner, 2026-10-26)

<!-- policy-text -->
<!-- owner-words -->"I want: **interoperate openly, evaluate privately**".<!-- /owner-words --> This rule governs CAEPIPE, the external prover the correlation work uses. Public text (the repository, PRs and commits) may name CAEPIPE and state the file-format and interface facts SWBPIPE's code needs to interoperate with it: record syntax, result-file layout, labels, sign and unit conventions, and how the importer and exporter map records. Each fact is cited to the vendor's public manual where it covers the fact, or otherwise stated plainly as observed format behaviour. Code may contain whatever it needs. Anything that evaluates CAEPIPE stays in the user's private exchange folder and is never committed: result values, differences, agreement counts and classifications, tolerances measured from results; its defects and crashes, trial limits, evaluation stamps and performance; screenshots and survey notes. No public text claims agreement with CAEPIPE. Survey proposals describe features in the project's own words, without attributing them to CAEPIPE. DEL-17-05 applies this to the correlation pilot.
<!-- /policy-text -->

#### 6.2.1 The correlation pilot's activation, target and invocation

The following are sections 1 to 3 of the correlation pilot's design, restored unchanged from lines 9 to 103 of `execution/PKG-17/DEL-17-05/Work/caepipe-correlation-pilot.md` at commit 0d2d37260, except that section 1's owner's words of 14 November 2026, and the paragraph after them from "The quotation contains" to "A full licence remains the owner's to buy.", are new on that date; that paragraph replaces the design's two sentences on the pilot's purpose and breadth.

## 1. Activation and purpose

PRD §22.5 makes activation "owner-gated on lawful access to the external tool". The
owner activated it on 2026-10-26:

<!-- owner-words -->
- Request: "Could we start making validation cases using a CAEPIPE solver in my
  Parallels Windows virtual desktop and have validation cases made as input files that
  can be given to CAEPIPE and solved and then take the outputs as the validation data
  for SWBPIPE to later use?"
- Rulings on Agent 0's questions: "1. Start that 5th lane now 2. Yes the license I just
  read allows such use and I take full responsibility. 3. Automate as much as possible,
  even the first time if that could be done right. 4. Make the shared folder in a place
  you recommend so that it's going to work well for our needs." And: "I have just
  installed an evaluation version of CAEPIPE and it is currently running through
  Parallels."
- Purpose of the pilot: "The pilot using the trial version of CAEPIPE is just to prove
  the basic concept for how I want to do validation will work. Once that's assured, I
  will pay for the full license allowing us to do more extensive validation work."
- On what the trial is used for (14 November 2026): "My philosophy here is use what we
  have available to do as much as we can. I also have given guidance that there's
  extensive model validation we can do even within the limitations of the [program's]
  trial. […] Perhaps that's what's motivated the 56-run benchmark and I have no reason
  to stop that from going forward. I see it as an important part of development if
  done well. It would provide you real data to compare SWBPIPE against, which is
  essential for effective hill-climbing. Thoughts?"
- On what an agent may read of the comparisons (14 November 2026), the owner's
  selections among wordings Agent 0 offered. For the comparison batch, "Everything,
  privately": "An agent reads the comparisons, values and differences included, inside
  the private folder and out of Git, and uses them to find and fix disagreements.
  Nothing is published before your licence check." For the captures sealed for the
  sitting of 21 to 24 November, "Same as the batch": "Whatever an agent may read of the
  batch, it may also read of the sealed sitting captures." The owner's words
  afterwards: "the old rulings seemed to treat those as secrets the agents had to be
  able to achieve parity with by their own deduction, but I want to open up those as
  targets for the agents to use as verification their chosen methods are working.
  Maybe I'm misconstruing things, but the direction is right."
- On the program's standing (14 November 2026): "CAEPIPE isn't "ground truth" but
  nothing except reality itself is. CAEPIPE is a trusted validator model. There are
  also trusted validation methods and outcomes that have been researched and planned
  for. I believe you will find that in the repo too."
<!-- /owner-words -->

The quotation contains a bracketed substitution and an omission to comply with §6.2.
The full wording is retained privately. The pilot began as a proof of concept
supporting that licence decision. Under the owner's direction of 14 November 2026,
validation may extend across the various modelling scenarios and certain
combinations, as far as they can be run now. Screening follows the benchmark method
in `docs/RELEASE_TARGET_v1.0.md`. A full licence remains the owner's to buy.
CAEPIPE results are a reference, not ground
truth: the hand-calculation witnesses remain the primary verification anchor, and
neither CAEPIPE agreement nor a completed run is professional acceptance, validation
of engineering use, or compatibility proof (DEL-17-05-REQ-008, F-PIP-3).

## 2. Target tool and environment

| Item | Pilot value |
|---|---|
| Target | CAEPIPE 15.10 (PRD §22.5 names the target tool and version), run under the owner's licence; MBF `VERSION 15.10` per the public import page |
| Environment | The owner's Windows virtual machine under Parallels on the development Mac; commands run as the logged-in Windows user (`prlctl exec <vm> --current-user`) |

The pilot cases are small. The writer reports the layout row count against a
configurable threshold and warns above it, but does not refuse.

## 3. Invocation profile (settles DEL-17-05-REQ-003 for the pilot)

The project selects the documented batch pattern (`CAEPIPE-BATCH`, located in DEL-17-01 `Source_Basis_Register.md`):

```text
<CAEPIPE executable> "<model>.mbf";batchmode=yes
```

It imports the MBF, writes `<model>.mod` and `<model>.res`, and is documented to write
`<model>.csv` beside the MBF. The harness always runs this route first and uses its CSV
whenever one appears.

The batch print's extension follows the print-file extension saved in the tool's preferences,
so the same print may be written as `<model>.txt`. The harness collects either and records the
name collected (`print_file`). `<model>.csv` is taken as named; `<model>.txt` is accepted only
when the print-CSV parser (§6) recognises its header, parses at least one section and reports no
error, and is otherwise refused as `print_file_unrecognised` and not read as results. When both
files are written, neither is taken (`print_file_ambiguous`) and no fallback runs. The saved
extension itself is never changed.

**Fallback when the batch route yields no result file.** The guest script then stops
the process it started, opens the solved results in a fresh CAEPIPE instance that it
starts itself, and prints the results to CSV through the documented print-to-file
command (`CAEPIPE-PRINT-TO-FILE`), finding menu items and buttons by their visible
captions. It reads no binary result file and stops only processes it started. The run
record states which route produced the CSV (`csv_from_batch` or `csv_from_gui_print`).

**MBF export of the solved model (two-way iteration, DEL-17-04 Design §18.3).** A
separate guest step writes the solved model back to MBF. The guest script, run with
`-ExportMod`, opens a guest copy of `caepipe_out/model.mod` in a CAEPIPE instance it
starts. A fresh instance starts either with the layout window of a start-up model or
with a frame window without one; both offer File > Open.... The script waits for the
layout window whose title names the opened model, then runs File > Export to MBF.... In
the "Export Model to Batch File" dialog it selects the type "Model batch file (*.mbf)",
sets the file name (control 1152) and commits (control 1). It waits for the file to
stop growing, then stops the instance it started. It reports the SHA-256 of the model it
opened and of the MBF it wrote. The step uses the stored solve rather than a fresh round
trip: the run record already binds `model.mod` to the current MBF, and the export only
reads that model. `run_pilot --export-mbf` runs the step after each case's live round
trip. `--export-only` runs it alone from each case's stored solve and needs no SWBPIPE
runner. It writes the case's `caepipe_out/` files (the export, its guest logs and the
run record's export entry) and the harness copy of the guest script and run lock in
`_harness/`, and creates missing layer folders. When the stored solve is no longer bound,
any earlier export is removed and the run record's export entry states why. The step's route is `mbf_exported` or the
reason none was written: `not_bound`, `prepare_failed`, `timeout`, `no_guest_json`,
`input_hash_mismatch`, `export_failed`, `mbf_not_written`, `guest_exit_nonzero` (the
guest reported the export but its step did not exit 0), `mbf_not_collected` or
`export_hash_mismatch`.

**Authored cases (DEL-17-04 Design §18.3).** `run_pilot --authored [--case ID …]` runs the
authored cases of `validation/caepipe_correlation/authored/` instead of the writer cases: invented,
hand-written MBF files listed in its `manifest.json`. Per case the live round trip above takes the
authored file as written in place of the writer's render, and the export step follows it.
`--authored --export-only` runs only the export step from the stored solve. No SWBPIPE solve, CSV
comparison or series step runs, and no runner is needed. A round trip whose guest did not report
importing the authored file (both hashes) stops the case before the export. No new harness or guest
mode is needed: the harness already takes the MBF text it is given, and the export reads the model
the batch route wrote.

## 7. Report boundary

Generated reports may reference a user rule-pack ID, version, checksum, and source note. Public report templates must not embed protected formulas or tables. Users are responsible for private report templates that quote licensed standards.

## 8. Contribution review checklist

Public contributions that add data, examples, report templates, schemas with example payloads, benchmark cases, import fixtures, or documentation excerpts must be reviewed before acceptance.

The review records:

- contribution identity and scope;
- source, provenance, license, redistribution, and contributor certification evidence;
- protected-content and private-data risk;
- test, validation, and documentation evidence;
- reviewer disposition;
- quarantine or escalation record when needed.

Maintainers may reject or defer contributions that do not provide enough evidence to satisfy this policy. Acceptance of repository content is project governance only; it is not professional approval of engineering use.
