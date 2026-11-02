# Supplier questions for the operating case, tier O-A (DEL-17-01, CQ-17-01-016)

**Status: a draft for the owner. It has not been sent.** The owner decides whether, how and to
whom to send it. No agent sends it, and no supplier has been contacted. The questions ask how
CAEPIPE reads and uses version 15.10 MBF files. They ask for no internals, standards content,
example files or licensing. The packet follows Agent 0's decision (f) of 2026-11-02
(paraphrase), recorded in DEL-17-04 `Work/caepipe-mbf-import.md` §20.5.14, referred to below
as "the Design".

Only the quoted **Ask** text and its record excerpts go to the supplier; the bullets marked
*Owner note* are for the owner only. The excerpts are **invented**, written in the project
writer's own form (Design §7.1, §9.1), and none is copied from a vendor example. They are record
excerpts, not complete files: they stand inside an otherwise valid version 15.10 model (layout,
sections and supports omitted), and no executable example is offered. Record any answer under
the dossier's **Closure Rule**, including whether it is public-citable. Asking a question closes
nothing, and an answer admits only the scope it actually covers.

## Priority 1: indispensable for tier O-A

*Owner note.* Within the scope of the Design's §20.5.12 matrix, O-A needs three supplier facts:
the mapped E, the strain behind T1 (with its units), and an explicit thermal-load count. No
fourth supplier question is indispensable for that matrix scope. Answers progress O-A only for
the units, options, field-5 states, piping codes (null included), reference temperatures and
recorded encodings they actually cover, and only once authenticated and admitted under the
dossier's Closure Rule. Even then the Design's acceptance, an independent persistence review and
the implementation remain. Answers here do not cover the full reference system, pressure,
forces or combinations (§20.5.12, **What each answer would add**).

### Q1. Constant modulus

> **Ask.** In a version 15.10 MBF, suppose every temperature row of a MATERIAL gives the same
> elastic modulus E. For which of these conditions is exactly that E used, unchanged, for
> every load case (Sustained, Operating, and the thermal or expansion case): either Elastic
> Modulus option; OPTEMP field 5 ("Use Cold Modulus") set to 0, set to 1, or omitted; each
> piping code on OPTIONS; no piping code at all; and an element temperature T1 that lies
> between rows? Please name any conditions that are exceptions, and the E they use. We ask only
> about the analysis behaviour, not about any code's rules or rationale.
>
> Record excerpts (inside an otherwise valid model):
>
> ```text
> OPTIONS
> HGRA,SI
> OPTEMP
> 20
> MATERIAL
> EX1,7850,0.3,1,1,CS,invented steel
> EX1,20,200000,1.2E-05
> EX1,170,200000,1.2E-05
> LOADS
> L1,170,0,0
> ```
>
> Variants of the same excerpts: OPTEMP `20,,1,,0` or `20,,1,,1`, and a piping-code token
> after `HGRA`. *The values are invented.*

- *Owner note: fields.* MATERIAL rows (temperature, E, α), OPTEMP field 5, and the OPTIONS code token
  (`CAEPIPE-IMPORT-MBF`, *Format of .MBF*; Design §6, §7.1, §20.4.9 G-1 (a)). The hot and cold
  options come from `CAEPIPE-ANALYSIS`, *Elastic Modulus* and *Reference Temperature*.
- *Owner note: desired fact.* The supported subset of conditions under which a single-E table
  gives that E for every case, with its exceptions, for version 15.10 files. That is the source
  proof named in §20.5.4 (e) for the conditions it covers. An answer for one code or option
  admits only that code or option; it does not admit other codes or the null code. A complete
  answer would settle the constant-E part of G-1 (c), (d), (e) and G-4; O-A does not need
  G-1 (b) (§20.5.11). No standards rule or rationale is requested.
- *Owner note: unblocks.* The §20.5.12 rows "MATERIAL rows: E" and "OPTIONS code token" (O-A
  condition 8, §20.5.5), for the covered conditions only.
- *Owner note: until answered.* The proposed design refuses every O-A export with
  `MBF-SI-OPERATING-MODULUS-SELECTION-UNESTABLISHED`, naming the token or null. The
  implemented writer keeps its existing generic refusals (`MBF-SI-PRESSURE-CASE-UNSUPPORTED`,
  `MBF-SI-LOAD-CASE-MAPPING-UNSUPPORTED`), per decision (b).

### Q2. Thermal strain (G-3)

> **Ask.** Take an element with LOADS temperature T1 and reference temperature Tref from
> OPTEMP field 1, both in the file's units. Is its free thermal strain α·(T1 − Tref), where α
> is the MATERIAL row's mean coefficient?
> (a) When every row gives the same α, is the strain exactly α·(T1 − Tref)?
> (b) From which temperature is the mean measured: Tref, or a fixed base of the table? Which
> row or interpolation applies when T1 or Tref falls between rows?
> (c) In an SI file (OPTIONS `SI`), are temperatures in °C and α in mm/mm/°C? In an English
> file, are they in °F and in/in/°F?
>
> Example, using the Q1 record excerpts: OPTEMP `20`, LOADS `L1,170,0,0` and α 1.2E-05 in both
> rows. Our reading would be a strain of 1.2E-05 × 150 = 1.8E-03, or 5.4 mm of free growth on a
> 3000 mm straight. Is that reading correct? *The values are invented.*

- *Owner note: fields.* LOADS T1, OPTEMP field 1 and the MATERIAL row's α (`CAEPIPE-IMPORT-MBF`,
  *Format of .MBF*; Design §7.1, §9.1, §9.2). The units follow the importer's reading in
  Design §5.2. `CAEPIPE-MATERIAL` says α is a mean coefficient but gives no base and no
  interpolation.
- *Owner note: desired fact.* O-A needs (a) and (c). The Design's proposed mathematical mapping
  is that, with constant α, any base of the mean gives α·(T1 − Tref) (§7.1); the target's use of
  it is unconfirmed, which is why (a) asks. (c), or an existing admitted unit proof, is a
  prerequisite too, not an optional part of the answer. (b) is CQ-17-01-016's original
  question and is not needed for O-A. An answer to (b) alone does not lift the O-7 refusal of
  varying tables: G-1, G-2 and G-4 conditions remain.
- *Owner note: unblocks.* The §20.5.12 row "LOADS T1 per element; MATERIAL α". No narrower refusal keeps
  an operating case without it.
- *Owner note: until answered.* The proposed design refuses with `MBF-SI-OPERATING-CLAIM-UNESTABLISHED`,
  naming T1 and G-3. Unbound imports of a varying table stay refused, as implemented
  (`MBF-IMPORT-MATERIAL-TEMPERATURE-DEPENDENT`).

### Q3. OPTEMP encoding of one thermal load (G-8)

> **Ask.** In OPTEMP, field 2 is the number of thermal cycles and field 3 the number of
> thermal loads.
> (a) Is `20,,1` accepted, with field 2 empty and field 3 = 1? Does it mean one thermal load
> with field 2 at its default, and what is that default?
> (b) If an empty field 2 is not accepted, which field 2 value leaves displacements, forces
> and moments unaffected? We use the analysis results, not the code-stress results.
> (c) When OPTEMP gives only field 1 (`20`), how many thermal loads are used?
>
> Record excerpt (inside an otherwise valid model):
>
> ```text
> OPTEMP
> 20,,1
> ```
>
> *The values are invented.*

- *Owner note: fields.* OPTEMP fields 1 to 3; fields 2 to 5 are optional (`CAEPIPE-IMPORT-MBF`,
  *Format of .MBF*, OPTEMP; Design §6, §20.4.9 G-1 (a)). The count's meaning is from
  `CAEPIPE-ANALYSIS` and `CAEPIPE-LOAD-CASES` (Design §9.1, §20.5.3). The writer writes field 1
  only.
- *Owner note: desired fact.* An accepted encoding of field 3 = 1 that leaves the analysis
  unchanged, recorded with the method selected. Failing that, the meaning of an omitted
  field 3 (G-8).
- *Owner note: unblocks.* The §20.5.12 row "OPTEMP field 3". It is avoided by writing field 3 = 1 once
  field 2's encoding is known; (c) would answer G-8 directly.
- *Owner note: until answered.* The proposed design refuses with `MBF-SI-OPERATING-CLAIM-UNESTABLISHED`,
  naming OPTEMP field 3 and G-8. Today's thermal export is unchanged.

## Priority 2: later, ranked

This ranking is proposed; Agent 0 or the owner may reorder it. Each question is drafted in full
where its line points.

4. **G-7, the pressure effect.** It adds pressure regions to O-A (§20.5.12, item 2). Draft:
   Design §20.5.11.
5. **G-1 (b), (c), (e), field 5's mapping.** It is needed for O-B, Route 1 and the reference
   system's varying modulus. Drafts: CQ-17-01-016 (1) and (G-1 (e)); (d) is (2).
6. **G-4, the exact row and interpolation.** It is needed for O-B and Route 1. Drafts:
   CQ-17-01-016 (5) and the original question.
7. **G-2, the hot sustained modulus.** It is needed for a sustained partner beside a varying
   modulus (RS-SUS). Draft: CQ-17-01-016 (3).
8. **G-5, the thermal-case method.** It bears only on RS-EXP's correspondence, and no admitted
   case depends on it. Draft: CQ-17-01-016 (4).
9. **G-10, support status.** **Not to be sent yet:** its boundary extension awaits IP/contract
   review and Agent 0's adoption (decision (e)). Draft: Design §20.5.11.

Also drafted, outside this ranking: G-11 and G-9 (Design §20.5.11), and CQ-17-01-013 for forces
without `FLC`.
