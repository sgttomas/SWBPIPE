# Source and provenance basis

All numerical model values are original authored inputs in `fixtures/product_preview/invented_preview_model.json` (SHA-256 `986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c`). No protected standard, proprietary benchmark, commercial example, or `domains/piping-design` OCR/extracted equation was used.

Public mechanics references, retrieved 2026-09-08:

1. [CALFEM for Python, `beam3e`](https://calfem-python-manual.readthedocs.io/en/latest/element_functions.html) — Lund University-origin finite-element package documentation for a 3D beam global stiffness matrix, explicit local orientation and optional uniformly distributed-load vector. Used to cross-check formulation class and parameter meanings; no CALFEM numerical output is imported.
2. [Abaqus beam cross-section orientation](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEELMRefMap/simaelm-c-beamcrosssection.htm) — vendor-authored documentation for a right-handed beam tangent and section-axis convention. Used only to corroborate explicit local-axis reporting.
3. [Abaqus distributed loads](https://docs.software.vt.edu/abaqusv2025/English/SIMACAEPRCRefMap/simaprc-c-loaddistributed.htm) — vendor-authored documentation that beam line loads are force per unit length and may be supplied in global or local directions. Used only to corroborate load meaning; the consistent vector is independently integrated/stated in `METHOD_AND_SIGNS.md`.
4. [Zimmerman and Ateshian, “A Surface-to-Surface Finite Element Algorithm for Large Deformation Frictional Contact,” DOI 10.1115/1.4040497](https://pmc.ncbi.nlm.nih.gov/articles/PMC6056201/) — original research distinction between stick/slip branches and return mapping. Its surface/finite-deformation formulation is not transplanted into this frame-support calculation.
5. [COMSOL theory, Tangential Contact with Friction](https://doc.comsol.com/6.4/doc/com.comsol.help.sme/sme_ug_theory.06.088.html) — official theory documentation distinguishing incremental friction state/history from numerical iteration. Used to bound the unresolved history fork, not to select penalty parameters or product policy.

Local accepted evidence used only for scope and claim calibration: E1 `children/friction/DECISION_BRIEF.md`, Q1 `REPORT.md`, audit S2 `FINDING_DISPOSITIONS.csv`, and P9 preparation handoff. Those artifacts identify the missing independent mixed reference and known D01/D02 limits; their production observations do not derive any expected number here.
