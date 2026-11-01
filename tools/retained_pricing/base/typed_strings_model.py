"""T03's typed-request string census (retained-pricing-rules.md §8.36, S1-TS): the admitted request's heap strings that
the typed residence (`ordinary_caps.py` T_resident, `t25_g4.py` T25.1a) charges at one D1 text capacity (ID) each.

The chain's count was fixed at the basis (`6 + 2n + 12m + 6g + r + 2c + 7L`, plus the materials). Read against the
actual typed records and the census walk that caps every walked string (`retained_memory.rs` TypedWalk, D1.9's
TypedTextCapacity), three admitted owners are missing from it:
- a pipe segment holds 13 admitted strings, not 12: its id, from, to and material, its provenance, the units of its
  section's two required and six optional quantities (`section_ref` is refused at D1.3 and model document 0.9.0's
  `condition_set_ref` at D1.3 too; label, tags and notes are zero-size `IgnoredAny`);
- on route E the pressure contract's three strings (version, mode, shear_model), which the walk reads (D1.3 admits the
  contract there);
- `model_settings.node_proximity_tolerance`'s unit (D1 admits the setting; the walk does not read it, so its capacity is
  bounded by the raw text cap, 128 B, through the parse from the captured raw request).
The correction charges all three on both routes (route L's contract is absent: a conservative 3 strings).

Bound to the exact struct bodies and the walk's pipe and contract statements; a changed record or walk stops the run.
Model document 0.8.0's record (without condition_set_ref) reads the same 13. A snapshot before model_settings (model
document 0.7.0) keeps the chain's basis count for its registered trees."""
import os
import re

PP = "core/product_physics/src/"
BASIS_PIPE_STRINGS = 12
PIPE_FIELDS = ["id", "from", "to", "section", "section_ref", "condition_set_ref", "material", "y_reference", "provenance",
               "label", "tags", "notes"]
# Model document 0.8.0's record (main before S1): the same owners without condition_set_ref; the census is the same 13.
PIPE_FIELDS_0_8_0 = [f for f in PIPE_FIELDS if f != "condition_set_ref"]
SECTION_FIELDS = ["outside_diameter", "wall_thickness", "mill_tolerance", "corrosion_allowance", "material_density",
                  "contents_density", "insulation_thickness", "insulation_density"]
WALK_PIPE = ("for text in [&pipe.id, &pipe.from, &pipe.to, &pipe.material] {",
             "self.optional(&pipe.section_ref)?;",
             "self.optional(&pipe.provenance)?;",
             "self.quantity(&section.outside_diameter)?;",
             "self.quantity(&section.wall_thickness)?;",
             "for q in [&section.mill_tolerance, &section.corrosion_allowance, &section.material_density, &section.contents_density, &section.insulation_thickness, &section.insulation_density] {")
WALK_CONTRACT = ("self.optional(&contract.version)?;", "self.optional(&contract.mode)?;", "self.optional(&contract.shear_model)?;")


def _fields(text, struct):
    body = re.search(r"\npub struct " + struct + r" \{\n(.*?)\n\}\n", text, re.S)
    if not body:
        raise SystemExit(f"S1-TS: struct {struct} not found")
    return re.findall(r"^    pub ([a-z_]+): ", body.group(1), re.M)


def _types(text, struct):
    body = re.search(r"\npub struct " + struct + r" \{\n(.*?)\n\}\n", text, re.S)
    return dict(re.findall(r"^    pub ([a-z_]+): ([^,\n]+),", body.group(1), re.M))


def inspect(snapshot):
    """The S1-TS counts at this snapshot: {"pipe": strings per pipe, "model": strings beyond the basis' 6}. None only for
    a snapshot without model_settings (before model document 0.7.0, whose registered trees keep the basis count); any
    other record than this reading stops the run."""
    if not snapshot or not os.path.exists(os.path.join(snapshot, PP + "model_settings.rs")):
        return None
    read = lambda f: open(os.path.join(snapshot, PP + f), encoding="utf-8").read()
    lib, settings, runtime, memory = read("lib.rs"), read("model_settings.rs"), read("pressure_runtime.rs"), read("retained_memory.rs")
    fields = _fields(lib, "PreviewPipe")
    if fields not in (PIPE_FIELDS, PIPE_FIELDS_0_8_0):
        raise SystemExit(f"S1-TS: PreviewPipe's members changed; re-review the typed string count: {fields}")
    s1 = fields == PIPE_FIELDS
    pipe, section, quantity = _types(lib, "PreviewPipe"), _types(lib, "PipeSectionInput"), _types(lib, "Quantity")
    if (list(section) != SECTION_FIELDS or quantity != {"value": "f64", "unit": "String"}
            or [pipe[k] for k in ("id", "from", "to", "material")] != ["String"] * 4
            or [pipe[k] for k in ("section_ref", "provenance") + (("condition_set_ref",) if s1 else ())] != ["Option<String>"] * (3 if s1 else 2)
            or pipe["y_reference"] != "Option<Vec3>" or [pipe[k] for k in ("label", "tags", "notes")] != ["serde::de::IgnoredAny"] * 3
            or [section[k] for k in SECTION_FIELDS[:2]] != ["Quantity"] * 2 or [section[k] for k in SECTION_FIELDS[2:]] != ["Option<Quantity>"] * 6):
        raise SystemExit("S1-TS: PreviewPipe's or its section's owners changed; re-review the typed string count")
    if _types(runtime, "PressureContractInput") != {"version": "Option<String>", "mode": "Option<String>", "shear_model": "Option<String>"}:
        raise SystemExit("S1-TS: PressureContractInput changed; re-review the typed string count")
    if _types(settings, "ModelSettings") != {"node_proximity_tolerance": "Option<Quantity>", "shear_model": "Option<ShearModelSetting>"}:
        raise SystemExit("S1-TS: ModelSettings changed; re-review the typed string count")
    if "    Unknown,\n}" not in settings.split("pub enum ShearModelSetting {", 1)[1].split("\n}", 1)[0] + "\n}":
        raise SystemExit("S1-TS: ShearModelSetting changed; re-review the typed string count")
    walk = memory.split("    fn walk(&mut self, request: &LinearStaticPreviewRequest)", 1)[1].split("\npub(super) fn nested_typed_census", 1)[0]
    if any(walk.count(t) != 1 for t in WALK_PIPE + WALK_CONTRACT) or "condition_set_ref" in walk or "model_settings" in walk:
        raise SystemExit("S1-TS: the census walk's pipe or contract statements changed; re-review the typed string count")
    # D1.3 refuses a pipe's section_ref and condition_set_ref (their family clauses), so neither owns a string on D1.
    family = memory.split("fn family_clauses(", 1)[1].split("\n}\n", 1)[0]
    if ("if m.pipe_segments.iter().any(|pipe| pipe.section_ref.is_some()) {" not in family
            or (s1 and "if m.pipe_segments.iter().any(|pipe| pipe.condition_set_ref.is_some()) {" not in family)):
        raise SystemExit("S1-TS: D1.3's pipe reference clauses changed; re-review the typed string count")
    admitted = 4 + 1 + len(SECTION_FIELDS)        # id/from/to/material, provenance, eight quantity units
    return {"pipe": admitted, "model": 3 + 1}      # the contract's three strings and the tolerance's unit
