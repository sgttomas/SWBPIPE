This is section 4.4 of the modal-analysis design, on the published piping-system benchmarks and their provenance, restored unchanged from lines 682 to 717 of `execution/PKG-04/DEL-04-09/Work/modal-analysis.md` at commit 0d2d37260.
The review of the rights to these sources remains the owner's.

### 4.4 Piping-system benchmarks and their provenance

- **[design]** The repository catalogues them as `NRC-DYNAMIC`: "NUREG/CR-1677 volumes 1/2, CR-6414, CR-6645; ANSYS descendants", readiness `catalogued_unadmitted` (`validation/qualification/capability_inventory.json:2523-2535`). NUREG/CR-1677 Vol. 1 (uniform support motion, with reference frequencies and participation factors) and Vol. 2 (independent support motion) are BNL problem sets for the NRC; NUREG/CR-6414 has three AP600 problems; NUREG/CR-6645 covers modal-response combination (an F2 concern).
- **[design]** NUREG/CR reports are prepared by contractors (for these, BNL) for the NRC. NRC sponsorship does not by itself make a contractor-authored report a US-government work. `docs/IP_AND_DATA_BOUNDARY.md` §1 requires human or legal review "when redistribution rights or protected-content status are uncertain", and §4 holds a record with `source_license = TBD` or `redistribution_status = unknown` until a maintainer records a disposition.
- **[decided]** (D-i) Agent 0, 2026-10-27, revising the earlier premise:
  > "Confirmed: the per-report condition. My premise was too broad. NRC sponsorship doesn't by itself make a contractor-authored NUREG/CR a US-government work. So:
  > - Transcribe from a report only with that report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture.
  > - Hold any report with uncertain rights for owner review, per IP_AND_DATA_BOUNDARY.
  > - Prefer sources whose rights are clear: original closed-form benchmarks we derive ourselves, and textbook formulas restated in our own words with citation.
  > - FR-SOL-016 stays partial until at least one published piping benchmark is compared with that evidence. Agreed."
- **Transcription is conditional on report-specific evidence.** Nothing is transcribed from a report until its own redistribution evidence is in hand: the report's rights statement (its front-matter notice, including any copyright or contractor-rights notice) and the NRC's distribution terms for it. A report whose rights are uncertain, or whose notice reserves rights, is **held for the owner's review** under `docs/IP_AND_DATA_BOUNDARY.md`, and nothing is transcribed from it in the meantime. Content inside a report that is marked as copyrighted or adapted from another source, such as figures adapted from journals, is never transcribed.
- **Per-fixture provenance record.** Each transcribed problem is committed with a `PROVENANCE.json` beside its fixture under `validation/references/f1_modal/published/<source>/<problem>/`, carrying the `docs/IP_AND_DATA_BOUNDARY.md` §4 fields:

  | Field | Content |
  |---|---|
  | `source_name` | Report number, volume, title and the problem's own identifier |
  | `source_location` | The report's NRC accession number (ADAMS) or NTIS/OSTI identifier, and the pages and tables transcribed |
  | `source_license` | The redistribution basis this report's own evidence supports, with its rights statement quoted from the front matter and the NRC distribution terms cited. `TBD` while the evidence is incomplete, in which case nothing is committed |
  | `contributor` | The transcribing role (TASK) and its PR; no personal identity |
  | `contributor_certification` | That only problem inputs and reference values were transcribed, and that no figure, table or text marked as copyrighted or adapted from another source was copied |
  | `redistribution_status` | `public_permissive` only when the report's own evidence supports it; otherwise the report is held and no fixture exists |
  | `review_status` | `pending`, the governing vocabulary's value (`docs/IP_AND_DATA_BOUNDARY.md` §4: pending, accepted, rejected, quarantined), until the maintainer records a disposition; agents never set another value |
  | `transcription_verification` | A separate field (D-u): an independent Type 2 reviewer verifies the transcription against the source and records agent verification, naming the reviewer role, the source pages and the date. It is evidence, not acceptance. If the rights are unclear, the fixture is not committed and the report holds for the owner |

  The record also lists what was **excluded** as copyrighted or adapted, with its location in the report, so a reviewer can see the line drawn.
- **Use.** NUREG/CR frequencies are numerical results of other programs, not exact answers ("NRC and ANSYS outputs retain shared lineage", `capability_inventory.json:2530-2533`). They are class-c comparison evidence at a stated tolerance. A report's participation factors may use another shape normalization, so they are compared through normalization-invariant quantities: effective masses, or Γ·φ.
- **Other sources:**
  - **Preferred, rights clear:** closed-form benchmarks derived in this repository (§4.2, §4.3), and textbook formulas restated in our own words with citation.
  - **Code_Aster** SDLX02 (Hovgaard modal) and SDLL14 (elbow frequencies) (VAL-REF-02 lines 94–95): GPL-3.0-or-later code; usable for independently implemented fixtures, not by vendoring, with the redistribution evidence for the transcribed data recorded per fixture as above.
  - **ASME PVP benchmark papers and NAFEMS natural-frequency benchmarks:** licensed or paywalled. Cite only; copy no data.
  - **CAEPIPE:** no outputs are copied. Any comparison is the owner's private exchange.
- **[decided] The published-benchmark obligation** (D-q). PRD §12.9 requires comparison with published piping benchmarks.
  - **Criterion.** Before FR-SOL-016 is reported complete, at least one published piping benchmark with recorded redistribution evidence is compared with the product, and the comparison is recorded as **comparison evidence, not exact verification**.
  - **Prerequisite and timing.** The per-report evidence above is the prerequisite. The comparison comes after M1b (the dense path).
  - **Until then**, FR-SOL-016 stays partial (`docs/COMPLETENESS_MAP.md`). M1–M3 rely on classes a and b. The first class-c target is one NUREG/CR-1677 Vol. 1 problem, whose modal part needs only frequencies and participation, if its report's evidence supports redistribution. Otherwise it is another published piping benchmark whose evidence does.
  - **Status of the first candidate (2026-10-31).** NUREG/CR-1677 Vol. 1 (BNL-NUREG-51267, August 1980; OSTI 6403318), Problem 1, is recorded with `source_license = TBD` and `redistribution_status = unknown` and held for the owner's review (O-3). Its notices mark distribution as unlimited and carry no copyright or contractor-rights notice, but they grant no licence, and the report is mostly contractor-authored. Its Problem 2 is adapted from the literature and is excluded regardless. Vol. 2, NUREG/CR-6414 and Code_Aster SDLX02 are held or unsuitable on their own evidence. Nothing is transcribed until the owner rules. A search for a problem by US government employees found one candidate, also held for the owner's ruling: D. H. Peters, "Shock Loads on Piping Systems" (Naval Postgraduate School thesis by a serving US Navy officer, 1972; DTIC ADA751643). It is marked "approved for public release; distribution unlimited" and carries no copyright notice; its §105 basis is inferred from authorship. Its 3-D, 12-element model has square corners and no bends, consistent mass, frequencies and participation factors. It is recorded with `source_license` TBD and `redistribution_status = unknown`.
