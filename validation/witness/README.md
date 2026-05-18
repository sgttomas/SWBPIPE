# Formal Calculation Witnesses

This directory contains machine-interpretable hand-calculation witness
artifacts for validation evidence. The witness JSON is the authoritative
source. Human-readable Markdown and Strict Content MathML are deterministic
renderings from that source.

The pilot witness is:

- `fixtures/tp_phys_015_section_property_stress_witness.json`

The pilot uses an OpenMath-style abstract object binding in JSON, a
repo-local experimental content-dictionary record, explicit units and
dimensions, and a validation-local phrasebook. It does not call production
solver, section-property, or stress-recovery implementation code.

Boundary: these artifacts are software verification evidence only. They do
not contain protected standards content, allowables, SIF/flexibility factors,
fatigue criteria, code-compliance findings, certification, sealing,
authentication, approval, or professional reliance conclusions.
