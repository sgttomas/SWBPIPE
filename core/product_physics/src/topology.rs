//! W-8 (DEL-04-06 `Design/model-topology-checks.md`): pre-solve topology
//! warnings for FR-KNOW-002 (disconnected routes) and PRD §12.8 (duplicate or
//! near-duplicate nodes), on both routes. Every finding is a warning: a model
//! whose parts are each restrained is solvable, and coincident nodes may be
//! intended, so nothing here refuses. An unrestrained part still refuses later
//! through the solve's mechanism check.
//!
//! The checks read the built model, after unit normalization, so coordinates
//! are in metres. Each code yields at most one diagnostic, naming at most
//! [`NAMED`] parts or pairs.

use super::{diag, BuiltModel, Diagnostic, PreviewModel};
use crate::formation_guard::{Bodies, NAMED};
use std::collections::{BTreeMap, HashMap, HashSet};

/// The near-duplicate tolerance relative to the model's extent (the diagonal
/// of its nodes' bounding box). The check is scale-free: the same model in
/// any length unit gives the same finding.
pub(crate) const NEAR_DUPLICATE_RELATIVE: f64 = 1e-6;

/// The element graph's edges: straight members other than connector-replaced
/// spans, curved spans and objective connectors (as `formation_bodies`).
fn element_edges(built: &BuiltModel) -> Vec<(usize, usize)> {
    let replaced = built
        .connector_records
        .iter()
        .map(|r| r.span_id.as_str())
        .collect::<HashSet<_>>();
    built
        .pipes
        .iter()
        .filter(|p| !replaced.contains(p.element_id.as_str()))
        .map(|p| (p.node_i.index, p.node_j.index))
        .chain(built.curved_bend_elements.iter().map(|b| (b.node_i, b.node_j)))
        .chain(
            built
                .connectors
                .iter()
                .map(|c| (c.node_i().index, c.node_j().index)),
        )
        .collect()
}

/// Appends the topology warnings for `built`, whose node indices follow
/// `model.nodes`.
pub(crate) fn append_topology_diagnostics(
    model: &PreviewModel,
    built: &BuiltModel,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let coordinates = built
        .nodes
        .iter()
        .map(|n| n.coordinates)
        .collect::<Vec<_>>();
    let edges = element_edges(built);
    let node_id = |index: usize| {
        model
            .nodes
            .get(index)
            .map(|n| n.id.clone())
            .unwrap_or_else(|| format!("node#{index}"))
    };
    disconnected_parts(built, &coordinates, &edges, &node_id, diagnostics);
    near_duplicate_nodes(&coordinates, &edges, &node_id, diagnostics);
}

/// FR-KNOW-002: two or more parts that each contain an element. A node with
/// no element is not a part (an isolated anchored node carries nothing).
fn disconnected_parts(
    built: &BuiltModel,
    coordinates: &[[f64; 3]],
    edges: &[(usize, usize)],
    node_id: &dyn Fn(usize) -> String,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let bodies = Bodies::new(coordinates, edges);
    // Each part with an element, keyed by body, in order of its first node.
    let mut parts: BTreeMap<usize, (usize, usize)> = BTreeMap::new(); // first node -> (body, nodes)
    let mut first_of_body: HashMap<usize, usize> = HashMap::new();
    let mut with_element = HashSet::new();
    for &(a, b) in edges {
        for node in [a, b] {
            if let Some(body) = bodies.body_of_node(node) {
                with_element.insert(body);
            }
        }
    }
    for node in 0..coordinates.len() {
        let Some(body) = bodies.body_of_node(node) else {
            continue;
        };
        if !with_element.contains(&body) {
            continue;
        }
        let first = *first_of_body.entry(body).or_insert(node);
        parts.entry(first).or_insert((body, 0)).1 += 1;
    }
    if parts.len() < 2 {
        return;
    }
    let grounded = built
        .supports
        .iter()
        .map(|s| s.node_index)
        .chain(built.nonlinear_supports.iter().map(|s| s.node_index))
        .filter_map(|node| bodies.body_of_node(node))
        .collect::<HashSet<_>>();
    let described = parts
        .iter()
        .take(NAMED)
        .enumerate()
        .map(|(ordinal, (&first, &(body, nodes)))| {
            format!(
                "part {} (from {}; {} node{}; {})",
                ordinal + 1,
                node_id(first),
                nodes,
                if nodes == 1 { "" } else { "s" },
                if grounded.contains(&body) {
                    "supported"
                } else {
                    "no support"
                }
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let more = if parts.len() > NAMED {
        format!("; and {} more", parts.len() - NAMED)
    } else {
        String::new()
    };
    diagnostics.push(diag(
        "diagnostic:topology:disconnected",
        "MODEL_TOPOLOGY_DISCONNECTED",
        "warning",
        format!(
            "the model's elements form {} disconnected parts; each part is solved as a separate structure and no force or movement passes between them: {described}{more}",
            parts.len()
        ),
        parts.keys().take(NAMED).map(|&first| node_id(first)).collect(),
    ));
}

/// PRD §12.8: distinct nodes at the same position, or within the relative
/// tolerance of each other, that no element joins. Two nodes an element joins
/// are left to the element's own length checks.
fn near_duplicate_nodes(
    coordinates: &[[f64; 3]],
    edges: &[(usize, usize)],
    node_id: &dyn Fn(usize) -> String,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if coordinates.len() < 2 {
        return;
    }
    let mut low = [f64::INFINITY; 3];
    let mut high = [f64::NEG_INFINITY; 3];
    for point in coordinates {
        for axis in 0..3 {
            low[axis] = low[axis].min(point[axis]);
            high[axis] = high[axis].max(point[axis]);
        }
    }
    let d = [0, 1, 2].map(|axis| high[axis] - low[axis]);
    let extent = ((d[0] * d[0] + d[1] * d[1]) + d[2] * d[2]).sqrt();
    if !extent.is_finite() {
        return;
    }
    // At zero extent every element has zero length and the build refuses it.
    let tolerance = NEAR_DUPLICATE_RELATIVE * extent;
    let joined = edges
        .iter()
        .map(|&(a, b)| (a.min(b), a.max(b)))
        .collect::<HashSet<_>>();
    let mut coincident = Vec::new();
    let mut near = Vec::new();
    if tolerance > 0.0 {
        // Cells of the tolerance's size: a pair within the tolerance lies in
        // the same or a neighbouring cell. Indices stay below 2e6 per axis.
        let cell = |point: &[f64; 3]| {
            [0, 1, 2].map(|axis| ((point[axis] - low[axis]) / tolerance).floor() as i64)
        };
        let mut grid: HashMap<[i64; 3], Vec<usize>> = HashMap::new();
        for (node, point) in coordinates.iter().enumerate() {
            grid.entry(cell(point)).or_default().push(node);
        }
        for (node, point) in coordinates.iter().enumerate() {
            let c = cell(point);
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let Some(others) = grid.get(&[c[0] + dx, c[1] + dy, c[2] + dz]) else {
                            continue;
                        };
                        for &other in others {
                            if other <= node || joined.contains(&(node, other)) {
                                continue;
                            }
                            let q = coordinates[other];
                            let e = [0, 1, 2].map(|axis| q[axis] - point[axis]);
                            let distance = ((e[0] * e[0] + e[1] * e[1]) + e[2] * e[2]).sqrt();
                            if distance == 0.0 {
                                coincident.push((node, other));
                            } else if distance <= tolerance {
                                near.push((node, other, distance));
                            }
                        }
                    }
                }
            }
        }
    }
    coincident.sort_unstable();
    near.sort_unstable_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    let refs = |pairs: &mut dyn Iterator<Item = (usize, usize)>| {
        let mut refs = Vec::new();
        for (a, b) in pairs.take(NAMED) {
            for id in [node_id(a), node_id(b)] {
                if !refs.contains(&id) {
                    refs.push(id);
                }
            }
        }
        refs
    };
    let more = |count: usize| {
        if count > NAMED {
            format!("; and {} more", count - NAMED)
        } else {
            String::new()
        }
    };
    if !coincident.is_empty() {
        let named = coincident
            .iter()
            .take(NAMED)
            .map(|&(a, b)| format!("{} and {}", node_id(a), node_id(b)))
            .collect::<Vec<_>>()
            .join("; ");
        diagnostics.push(diag(
            "diagnostic:topology:coincident-nodes",
            "NODE_COINCIDENT",
            "warning",
            format!(
                "{} pair{} of distinct nodes share a position and no element joins them; elements meeting there are not connected unless they share one node: {named}{}",
                coincident.len(),
                if coincident.len() == 1 { "" } else { "s" },
                more(coincident.len())
            ),
            refs(&mut coincident.iter().copied()),
        ));
    }
    if !near.is_empty() {
        let named = near
            .iter()
            .take(NAMED)
            .map(|&(a, b, distance)| format!("{} and {} ({distance:e} m apart)", node_id(a), node_id(b)))
            .collect::<Vec<_>>()
            .join("; ");
        diagnostics.push(diag(
            "diagnostic:topology:near-duplicate-nodes",
            "NODE_NEAR_DUPLICATE",
            "warning",
            format!(
                "{} pair{} of distinct nodes lie within {tolerance:e} m of each other (1e-6 of the model's extent) and no element joins them; a pipe may end short of the node it should share: {named}{}",
                near.len(),
                if near.len() == 1 { "" } else { "s" },
                more(near.len())
            ),
            refs(&mut near.iter().map(|&(a, b, _)| (a, b))),
        ));
    }
}
