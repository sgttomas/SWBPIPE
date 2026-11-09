//! Model-input sections (RT-R4 slice C5; DEL-08-01 Design
//! `report-section-composer.md` §2.7).
//!
//! `model_input_tables` is the renderer input's projection `P(M, C)` of the
//! bound model: one member per model-input id the composition lists, named by
//! the id, each a list of rows in the model's array order whose cells are a
//! string or null. The form is closed at every depth (no other member, every
//! cell present). The renderer emits each listed table as a subsection of its
//! governed parent (Load Cases or Model Input Summary), in the composition's
//! list order within that parent, with its disclosure statements; a null cell
//! is shown as "—". The renderer does not hold the model: every boundary that
//! does re-derives `P(M, C)` and requires equality
//! (`report_package::model_input`). Here a listed id without its table, or a
//! table without a listing id, is refused with
//! [`MODEL_INPUT_UNBOUND`].
//!
//! Standard claim fence applies (claims taxonomy per DEC-081).

use crate::composer::{CatalogueEntry, ComposedSection, ReportComposition};
use crate::{meta_paragraph, paragraph, subheading, table, SectionBlock};
use serde::de::{self, Deserializer, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

/// A model-input table that is not the projection of the bound model and the
/// composition, or a listing id and its table that do not go together.
pub const MODEL_INPUT_UNBOUND: &str = "REPORT-COMPOSITION-MODEL-INPUT-UNBOUND";

pub const LOAD_COMBINATIONS: &str = "load_combinations";
pub const INPUT_GEOMETRY: &str = "input_geometry";
pub const INPUT_MATERIALS_SECTIONS: &str = "input_materials_sections";
pub const INPUT_SUPPORTS: &str = "input_supports";
pub const INPUT_LOADS: &str = "input_loads";

/// The model-input ids, in catalogue order.
pub const MODEL_INPUT_SECTIONS: [&str; 5] = [
    LOAD_COMBINATIONS,
    INPUT_GEOMETRY,
    INPUT_MATERIALS_SECTIONS,
    INPUT_SUPPORTS,
    INPUT_LOADS,
];

/// Whether `id` is a model-input id.
pub fn is_model_input(id: &str) -> bool {
    MODEL_INPUT_SECTIONS.contains(&id)
}

/// A cell: a stored string, a canonical number text, `"<number> <unit>"`, or
/// null for an absent member. Required: an absent cell does not decode (its
/// visitor takes a string or null only, so a missing member is a "missing
/// field"), and it serializes as the string or null.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell(pub Option<String>);

impl Serialize for Cell {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.0 {
            Some(text) => serializer.serialize_str(text),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for Cell {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CellVisitor;
        impl<'de> Visitor<'de> for CellVisitor {
            type Value = Cell;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a string or null")
            }
            fn visit_str<E: de::Error>(self, value: &str) -> Result<Cell, E> {
                Ok(Cell(Some(value.to_string())))
            }
            fn visit_string<E: de::Error>(self, value: String) -> Result<Cell, E> {
                Ok(Cell(Some(value)))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Cell, E> {
                Ok(Cell(None))
            }
            fn visit_none<E: de::Error>(self) -> Result<Cell, E> {
                Ok(Cell(None))
            }
        }
        deserializer.deserialize_any(CellVisitor)
    }
}

/// One combination: id, label, basis, operands, mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CombinationRow {
    pub id: Cell,
    pub label: Cell,
    pub basis: Cell,
    pub operands: Cell,
    pub mode: Cell,
}

/// One node: id, label and its stored coordinates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeRow {
    pub id: Cell,
    pub label: Cell,
    pub x: Cell,
    pub y: Cell,
    pub z: Cell,
}

/// One pipe segment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipeRow {
    pub id: Cell,
    pub label: Cell,
    pub from: Cell,
    pub to: Cell,
    pub material: Cell,
    pub outside_diameter: Cell,
    pub wall_thickness: Cell,
    pub section_ref: Cell,
}

/// One component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComponentRow {
    pub id: Cell,
    pub label: Cell,
    pub kind: Cell,
}

/// One material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialRow {
    pub id: Cell,
    pub label: Cell,
    pub elastic_modulus: Cell,
    pub shear_modulus: Cell,
    pub poisson_ratio: Cell,
    pub thermal_expansion_coefficient: Cell,
}

/// One shared section.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SectionRow {
    pub id: Cell,
    pub name: Cell,
    pub section_type: Cell,
    pub outside_diameter: Cell,
    pub wall_thickness: Cell,
}

/// One support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportRow {
    pub id: Cell,
    pub label: Cell,
    pub node: Cell,
    pub family: Cell,
    pub restraints: Cell,
    pub stiffness: Cell,
    pub nonlinear_behavior: Cell,
}

/// One primitive load of a load case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoadRow {
    pub case_id: Cell,
    pub load_id: Cell,
    pub category: Cell,
    pub target: Cell,
    pub direction: Cell,
    pub magnitude: Cell,
}

/// `input_geometry`: nodes, pipes and components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeometryTables {
    pub nodes: Vec<NodeRow>,
    pub pipes: Vec<PipeRow>,
    pub components: Vec<ComponentRow>,
}

/// `input_materials_sections`: materials and sections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialsSectionsTables {
    pub materials: Vec<MaterialRow>,
    pub sections: Vec<SectionRow>,
}

/// `P(M, C)`: one member per listed model-input id.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelInputTables {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_combinations: Option<Vec<CombinationRow>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_geometry: Option<GeometryTables>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_materials_sections: Option<MaterialsSectionsTables>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_supports: Option<Vec<SupportRow>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_loads: Option<Vec<LoadRow>>,
}

impl ModelInputTables {
    /// Whether the table of `id` is present.
    pub fn has(&self, id: &str) -> bool {
        match id {
            LOAD_COMBINATIONS => self.load_combinations.is_some(),
            INPUT_GEOMETRY => self.input_geometry.is_some(),
            INPUT_MATERIALS_SECTIONS => self.input_materials_sections.is_some(),
            INPUT_SUPPORTS => self.input_supports.is_some(),
            INPUT_LOADS => self.input_loads.is_some(),
            _ => false,
        }
    }
}

/// The refusals of a composition's model-input listing against the tables: a
/// listed id without its table, a table whose id is not listed, and a tables
/// member (even an empty one) when no model-input id is listed. Named by id,
/// never by a value.
pub(crate) fn listing_refusals(composition: &ReportComposition, tables: Option<&ModelInputTables>) -> Vec<String> {
    let listed = |id: &str| composition.sections.iter().any(|section| section.section_id == id);
    if !MODEL_INPUT_SECTIONS.iter().any(|id| listed(id)) {
        return unlisted_tables_refusal(tables).into_iter().collect();
    }
    let mut refusals = Vec::new();
    for id in MODEL_INPUT_SECTIONS {
        let present = tables.is_some_and(|tables| tables.has(id));
        if listed(id) && !present {
            refusals.push(format!("{MODEL_INPUT_UNBOUND}: the composition lists {id}, but model_input_tables carries no {id} table"));
        } else if !listed(id) && present {
            refusals.push(format!("{MODEL_INPUT_UNBOUND}: model_input_tables carries a {id} table, but the composition does not list {id}"));
        }
    }
    refusals
}

/// The refusal of tables carried without a composition.
pub(crate) fn unlisted_tables_refusal(tables: Option<&ModelInputTables>) -> Option<String> {
    tables.map(|_| format!("{MODEL_INPUT_UNBOUND}: model_input_tables is carried without a composition that lists a model-input section"))
}

/// Every model-input table's statement.
pub const VALUES_AS_ENTERED_TEXT: &str = "Values as entered in the model, in their stored units, unconverted; a dash marks a member the model does not record. Provenance text and notes are not listed. This table carries no result, no check outcome and no code value.";
/// `input_geometry`'s statement on coordinates.
pub const COORDINATES_TEXT: &str = "Node coordinates x, y and z are stored numbers in the model's length unit (its project units, length).";
/// `input_supports`' statement.
pub const SUPPORTS_NOT_LISTED_TEXT: &str = "Hanger details and nonlinear details other than the behaviour are not listed.";
/// `input_loads`' statement.
pub const LOADS_NOT_LISTED_TEXT: &str = "Pressure regions, equivalent-static inputs and analysis states are not listed.";
/// A table with no row.
pub const NONE_ENTERED_TEXT: &str = "None entered in the model.";
/// The display of a null cell.
pub const ABSENT_CELL: &str = "\u{2014}";

fn shown(cell: &Cell) -> String {
    cell.0.clone().unwrap_or_else(|| ABSENT_CELL.to_string())
}

fn rows_table<R>(blocks: &mut Vec<SectionBlock>, header: &[&str], rows: &[R], cells: impl Fn(&R) -> Vec<&Cell>) {
    if rows.is_empty() {
        blocks.push(paragraph(NONE_ENTERED_TEXT));
    } else {
        blocks.push(table(header, rows.iter().map(|row| cells(row).into_iter().map(shown).collect()).collect()));
    }
}

/// The blocks of one listed model-input section.
fn section_blocks(entry: &CatalogueEntry, tables: &ModelInputTables) -> Vec<SectionBlock> {
    let mut blocks = vec![subheading(entry.title)];
    let mut statement = VALUES_AS_ENTERED_TEXT.to_string();
    match entry.id {
        INPUT_GEOMETRY => {
            statement.push(' ');
            statement.push_str(COORDINATES_TEXT);
        }
        INPUT_SUPPORTS => {
            statement.push(' ');
            statement.push_str(SUPPORTS_NOT_LISTED_TEXT);
        }
        INPUT_LOADS => {
            statement.push(' ');
            statement.push_str(LOADS_NOT_LISTED_TEXT);
        }
        _ => {}
    }
    blocks.push(meta_paragraph(&statement));
    match entry.id {
        LOAD_COMBINATIONS => {
            if let Some(rows) = &tables.load_combinations {
                rows_table(&mut blocks, &["Id", "Label", "Basis", "Operands", "Mode"], rows, |row| {
                    vec![&row.id, &row.label, &row.basis, &row.operands, &row.mode]
                });
            }
        }
        INPUT_GEOMETRY => {
            if let Some(geometry) = &tables.input_geometry {
                blocks.push(paragraph("Nodes"));
                rows_table(&mut blocks, &["Id", "Label", "x", "y", "z"], &geometry.nodes, |row| {
                    vec![&row.id, &row.label, &row.x, &row.y, &row.z]
                });
                blocks.push(paragraph("Pipes"));
                rows_table(
                    &mut blocks,
                    &["Id", "Label", "From", "To", "Material", "Outside diameter", "Wall thickness", "Section ref"],
                    &geometry.pipes,
                    |row| {
                        vec![
                            &row.id,
                            &row.label,
                            &row.from,
                            &row.to,
                            &row.material,
                            &row.outside_diameter,
                            &row.wall_thickness,
                            &row.section_ref,
                        ]
                    },
                );
                blocks.push(paragraph("Components"));
                rows_table(&mut blocks, &["Id", "Label", "Kind"], &geometry.components, |row| vec![&row.id, &row.label, &row.kind]);
            }
        }
        INPUT_MATERIALS_SECTIONS => {
            if let Some(tables) = &tables.input_materials_sections {
                blocks.push(paragraph("Materials"));
                rows_table(
                    &mut blocks,
                    &["Id", "Label", "Elastic modulus", "Shear modulus", "Poisson ratio", "Thermal expansion coefficient"],
                    &tables.materials,
                    |row| {
                        vec![
                            &row.id,
                            &row.label,
                            &row.elastic_modulus,
                            &row.shear_modulus,
                            &row.poisson_ratio,
                            &row.thermal_expansion_coefficient,
                        ]
                    },
                );
                blocks.push(paragraph("Sections"));
                rows_table(
                    &mut blocks,
                    &["Id", "Name", "Section type", "Outside diameter", "Wall thickness"],
                    &tables.sections,
                    |row| vec![&row.id, &row.name, &row.section_type, &row.outside_diameter, &row.wall_thickness],
                );
            }
        }
        INPUT_SUPPORTS => {
            if let Some(rows) = &tables.input_supports {
                rows_table(
                    &mut blocks,
                    &["Id", "Label", "Node", "Family", "Restraints", "Stiffness", "Nonlinear behaviour"],
                    rows,
                    |row| vec![&row.id, &row.label, &row.node, &row.family, &row.restraints, &row.stiffness, &row.nonlinear_behavior],
                );
            }
        }
        INPUT_LOADS => {
            if let Some(rows) = &tables.input_loads {
                rows_table(&mut blocks, &["Case", "Load", "Category", "Target", "Direction", "Magnitude"], rows, |row| {
                    vec![&row.case_id, &row.load_id, &row.category, &row.target, &row.direction, &row.magnitude]
                });
            }
        }
        _ => {}
    }
    blocks
}

/// The model-input subsections of governed section `parent`, in the
/// composition's list order (an admitted composition: every listed
/// model-input id has its table).
pub(crate) fn parent_blocks(parent: &str, sections: &[ComposedSection], tables: Option<&ModelInputTables>) -> Vec<SectionBlock> {
    let Some(tables) = tables else { return Vec::new() };
    sections
        .iter()
        .filter(|section| section.entry.parent == parent && is_model_input(section.entry.id))
        .flat_map(|section| section_blocks(section.entry, tables))
        .collect()
}
