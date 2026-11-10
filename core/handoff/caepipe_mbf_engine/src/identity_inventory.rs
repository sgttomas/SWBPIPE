//! Per-class identity inventories of a model document, for the designated-source check (DEL-17-04 Design
//! `desktop-mbf-export-import.md` §5.4; A-3's identity set).
//!
//! Each of the nine classes is read from the document itself: nodes, pipes (`pipe_segments`), components, supports,
//! load cases, primitive loads, materials, pressure regions and sections. Primitive loads and pressure regions belong
//! to a load case, so their key is (case id, id); every other key is the id. A class's summary is its count, whether
//! any key occurs twice, and the SHA-256 of its keys in sorted order, each component length-prefixed, so a substituted,
//! missing, added or duplicated id changes the summary even when the count does not. An entry without a string id,
//! or a class member that is not a list, makes the inventory malformed, which never compares equal.
//!
//! The summaries are fixed size; building them borrows the id strings and sorts one vector of references per class,
//! which is working storage proportional to the class's size, released when the summary is formed.
use crate::value::Value;
use sha2::{Digest, Sha256};

/// The classes, in the order the summaries are kept.
pub const INVENTORY_CLASSES: [&str; 9] =
    ["node", "pipe", "component", "support", "load_case", "primitive_load", "material", "pressure_region", "section"];

/// One class's summary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClassInventory {
    pub count: u64,
    pub duplicates: bool,
    pub digest: [u8; 32],
}

/// A document's nine summaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdentityInventory {
    pub classes: [ClassInventory; 9],
    pub malformed: bool,
}

/// How two inventories compare, class by class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InventoryComparison {
    /// The classes whose summaries differ, or that hold a duplicate on either side, by index into
    /// [`INVENTORY_CLASSES`].
    pub differing: [bool; 9],
    /// Either side is malformed.
    pub malformed: bool,
}

impl InventoryComparison {
    /// The identity sets are the same, class by class, with no duplicate and nothing malformed.
    pub fn equal(&self) -> bool {
        !self.malformed && !self.differing.iter().any(|differs| *differs)
    }
}

fn list<'v>(value: &'v Value, key: &str, malformed: &mut bool) -> &'v [Value] {
    match value.get(key) {
        None | Some(Value::Null) => &[],
        Some(Value::List(items)) => items.as_slice(),
        Some(_) => {
            *malformed = true;
            &[]
        }
    }
}

fn id<'v>(entry: &'v Value, malformed: &mut bool) -> &'v str {
    match entry.get("id").and_then(Value::as_str) {
        Some(id) => id,
        None => {
            *malformed = true;
            ""
        }
    }
}

fn keyed<'v>(document: &'v Value, key: &str, malformed: &mut bool) -> Vec<(&'v str, &'v str)> {
    list(document, key, malformed).iter().map(|entry| ("", id(entry, malformed))).collect()
}

fn summarize(mut keys: Vec<(&str, &str)>) -> ClassInventory {
    keys.sort_unstable();
    let duplicates = keys.windows(2).any(|pair| pair[0] == pair[1]);
    let mut hasher = Sha256::new();
    for (scope, id) in &keys {
        for part in [scope, id] {
            hasher.update((part.len() as u64).to_le_bytes());
            hasher.update(part.as_bytes());
        }
    }
    ClassInventory { count: keys.len() as u64, duplicates, digest: hasher.finalize().into() }
}

/// The document's inventory. `document` is a model document's value (the source the user designated, or the
/// import's candidate); a value that is not an object is malformed.
pub fn identity_inventory(document: &Value) -> IdentityInventory {
    let mut malformed = !matches!(document, Value::Dict(_));
    let nodes = keyed(document, "nodes", &mut malformed);
    let pipes = keyed(document, "pipe_segments", &mut malformed);
    let components = keyed(document, "components", &mut malformed);
    let supports = keyed(document, "supports", &mut malformed);
    let materials = keyed(document, "materials", &mut malformed);
    let sections = keyed(document, "sections", &mut malformed);
    let cases = list(document, "load_cases", &mut malformed);
    let mut case_ids = Vec::with_capacity(cases.len());
    let mut loads = Vec::new();
    let mut regions = Vec::new();
    for case in cases {
        let case_id = id(case, &mut malformed);
        case_ids.push(("", case_id));
        for load in list(case, "primitive_loads", &mut malformed) {
            loads.push((case_id, id(load, &mut malformed)));
        }
        for region in list(case, "pressure_regions", &mut malformed) {
            regions.push((case_id, id(region, &mut malformed)));
        }
    }
    let classes = [
        summarize(nodes),
        summarize(pipes),
        summarize(components),
        summarize(supports),
        summarize(case_ids),
        summarize(loads),
        summarize(materials),
        summarize(regions),
        summarize(sections),
    ];
    IdentityInventory { classes, malformed }
}

/// Compares the designated source's inventory with the import's.
pub fn compare_inventories(source: &IdentityInventory, imported: &IdentityInventory) -> InventoryComparison {
    let mut differing = [false; 9];
    for (index, differs) in differing.iter_mut().enumerate() {
        let (a, b) = (&source.classes[index], &imported.classes[index]);
        *differs = a != b || a.duplicates || b.duplicates;
    }
    InventoryComparison { differing, malformed: source.malformed || imported.malformed }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pyjson;

    fn doc(text: &str) -> Value {
        pyjson::parse_sidecar_json(text.as_bytes()).expect("test JSON")
    }

    const BASE: &str = r#"{"nodes":[{"id":"node:A"},{"id":"node:B"}],"pipe_segments":[{"id":"pipe:1"}],
        "components":[{"id":"component:bend-1"}],"supports":[{"id":"support:A"}],"materials":[{"id":"material:m"}],
        "sections":[{"id":"section:s"}],"load_cases":[{"id":"load_case:sus","primitive_loads":[{"id":"load:f"}],
        "pressure_regions":[{"id":"region:r"}]},{"id":"load_case:th","primitive_loads":[{"id":"load:t"}]}]}"#;

    fn compare(a: &str, b: &str) -> InventoryComparison {
        compare_inventories(&identity_inventory(&doc(a)), &identity_inventory(&doc(b)))
    }

    fn differing(comparison: InventoryComparison) -> Vec<&'static str> {
        INVENTORY_CLASSES.iter().zip(comparison.differing).filter(|(_, differs)| *differs).map(|(name, _)| *name).collect()
    }

    #[test]
    fn the_same_identities_compare_equal_whatever_their_order() {
        let reordered = BASE.replace(r#"{"id":"node:A"},{"id":"node:B"}"#, r#"{"id":"node:B"},{"id":"node:A"}"#);
        assert!(compare(BASE, &reordered).equal());
        let inventory = identity_inventory(&doc(BASE));
        assert_eq!(inventory.classes.map(|class| class.count), [2, 1, 1, 1, 2, 2, 1, 1, 1]);
    }

    #[test]
    fn a_substituted_id_with_the_same_count_differs_in_its_class_only() {
        let substituted = BASE.replace("node:B", "node:Z");
        assert_eq!(differing(compare(BASE, &substituted)), ["node"]);
    }

    #[test]
    fn a_duplicate_never_compares_equal_even_against_itself() {
        let duplicated = BASE.replace(r#"{"id":"node:B"}"#, r#"{"id":"node:A"}"#);
        assert!(identity_inventory(&doc(&duplicated)).classes[0].duplicates);
        assert_eq!(differing(compare(&duplicated, &duplicated)), ["node"]);
    }

    #[test]
    fn missing_and_added_ids_differ() {
        let missing = BASE.replace(r#",{"id":"section:s"}"#, "").replace(r#"[{"id":"section:s"}]"#, "[]");
        assert_eq!(differing(compare(BASE, &missing)), ["section"]);
        let added = BASE.replace(r#"[{"id":"material:m"}]"#, r#"[{"id":"material:m"},{"id":"material:n"}]"#);
        assert_eq!(differing(compare(BASE, &added)), ["material"]);
    }

    #[test]
    fn case_scoped_classes_key_by_their_case() {
        // The same load id moved to the other case keeps both counts but differs.
        let moved = BASE
            .replace(r#""primitive_loads":[{"id":"load:f"}]"#, r#""primitive_loads":[]"#)
            .replace(r#""primitive_loads":[{"id":"load:t"}]"#, r#""primitive_loads":[{"id":"load:t"},{"id":"load:f"}]"#);
        assert_eq!(differing(compare(BASE, &moved)), ["primitive_load"]);
        // Length-prefixing keeps ("ab", "c") apart from ("a", "bc").
        let a = identity_inventory(&doc(r#"{"load_cases":[{"id":"ab","primitive_loads":[{"id":"c"}]}]}"#));
        let b = identity_inventory(&doc(r#"{"load_cases":[{"id":"a","primitive_loads":[{"id":"bc"}]}]}"#));
        assert_ne!(a.classes[5].digest, b.classes[5].digest);
    }

    #[test]
    fn an_entry_without_a_string_id_or_a_non_list_member_is_malformed() {
        assert!(identity_inventory(&doc(r#"{"nodes":[{"label":"x"}]}"#)).malformed);
        assert!(identity_inventory(&doc(r#"{"nodes":{"id":"node:A"}}"#)).malformed);
        assert!(identity_inventory(&doc("[]")).malformed);
        assert!(!compare(BASE, r#"{"nodes":[{"id":1}]}"#).equal());
        // Absent members are empty classes, not malformed.
        assert!(!identity_inventory(&doc("{}")).malformed);
    }
}
