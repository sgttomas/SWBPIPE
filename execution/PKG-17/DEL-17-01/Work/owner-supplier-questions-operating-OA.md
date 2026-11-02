# Supplier questions for the operating case, tier O-A (DEL-17-01, CQ-17-01-016)

**Status: a draft for the owner. It has not been sent.** The owner decides whether, how and to
whom to send it. No agent sends it, and no supplier has been contacted. The questions ask how
CAEPIPE reads and uses version 15.10 MBF files. They ask for no internals, standards content,
example files or licensing. The packet follows Agent 0's decision (f) of 2026-11-02
(paraphrase), recorded in DEL-17-04 `Work/caepipe-mbf-import.md` §20.5.14, referred to below
as "the Design".

Only the quoted **Ask** text and its example go to the supplier; the bullets are notes for the
owner. The examples are **invented** and are written in the project writer's own form (Design
§7.1, §9.1); none is copied from a vendor example. Record any answer under the dossier's
**Closure Rule**, including whether it is public-citable. Asking a question closes nothing.

## Priority 1: indispensable for tier O-A

According to the Design's §20.5.12, O-A needs three supplier facts: the mapped E, the strain
behind T1, and an explicit thermal-load count. It also needs an internal persistence decision.
No fourth question is indispensable. Forces, moments and pressure extend O-A later
(§20.5.12, **What each answer would add**).

### Q1. Constant modulus

> **Ask.** In a version 15.10 MBF, suppose every temperature row of a MATERIAL gives the same
> elastic modulus E. Is exactly that E used, unchanged, for every load case (Sustained,
> Operating, and the thermal or expansion case)? Please cover all of these conditions: either
> Elastic Modulus option; OPTEMP field 5 ("Use Cold Modulus") set to 0, set to 1, or omitted;
> any piping code on OPTIONS, including one for which field 5 is ignored; no piping code at
> all; and an element temperature T1 that lies between rows. If the answer is not "yes, in
> every case", which conditions differ, and which E do they use?
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
> The same file also with OPTEMP `20,,1,,0` or `20,,1,,1`, and with a piping-code token after
> `HGRA`. *The values are invented.*

- **Fields.** MATERIAL rows (temperature, E, α), OPTEMP field 5, and the OPTIONS code token
  (`CAEPIPE-IMPORT-MBF`, *Format of .MBF*; Design §6, §7.1, §20.4.9 G-1 (a)). The hot and cold
  options come from `CAEPIPE-ANALYSIS`, *Elastic Modulus* and *Reference Temperature*.
- **Desired fact.** One statement: when all rows give one E, that E is the stiffness modulus
  for every case, under every option, field-5 state and code, null included. This is the
  source proof named in §20.5.4 (e). It settles the constant-E part of G-1 (c), (d), (e) and
  G-4. O-A does not need G-1 (b) (§20.5.11).
- **Unblocks.** The §20.5.12 rows "MATERIAL rows: E" and "OPTIONS code token", which is O-A
  condition 8 (§20.5.5).
- **Until answered.** The proposed design refuses every O-A export with
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
> Example: the Q1 file, with OPTEMP `20`, LOADS `L1,170,0,0` and α 1.2E-05 in both rows. Our
> reading is a strain of 1.2E-05 × 150 = 1.8E-03, or 5.4 mm of free growth on a 3000 mm
> straight. *The values are invented.*

- **Fields.** LOADS T1, OPTEMP field 1 and the MATERIAL row's α (`CAEPIPE-IMPORT-MBF`,
  *Format of .MBF*; Design §7.1, §9.1, §9.2). The units follow the importer's reading in
  Design §5.2. `CAEPIPE-MATERIAL` says α is a mean coefficient but gives no base and no
  interpolation.
- **Desired fact.** O-A needs (a): with constant α every base gives α·(T1 − Tref) (§7.1), so
  only the formula is asked. (b) is CQ-17-01-016's original question; it lifts the O-7 refusal
  of varying tables and is not needed for O-A. (c) confirms the units.
- **Unblocks.** The §20.5.12 row "LOADS T1 per element; MATERIAL α". No narrower refusal keeps
  an operating case without it.
- **Until answered.** The proposed design refuses with `MBF-SI-OPERATING-CLAIM-UNESTABLISHED`,
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
> ```text
> OPTEMP
> 20,,1
> ```
>
> *The values are invented.*

- **Fields.** OPTEMP fields 1 to 3; fields 2 to 5 are optional (`CAEPIPE-IMPORT-MBF`,
  *Format of .MBF*, OPTEMP; Design §6, §20.4.9 G-1 (a)). The count's meaning is from
  `CAEPIPE-ANALYSIS` and `CAEPIPE-LOAD-CASES` (Design §9.1, §20.5.3). The writer writes field 1
  only.
- **Desired fact.** An accepted encoding of field 3 = 1 that leaves the analysis unchanged.
  Failing that, the meaning of an omitted field 3 (G-8).
- **Unblocks.** The §20.5.12 row "OPTEMP field 3". It is avoided by writing field 3 = 1 once
  field 2's encoding is known; (c) would answer G-8 directly.
- **Until answered.** The proposed design refuses with `MBF-SI-OPERATING-CLAIM-UNESTABLISHED`,
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
