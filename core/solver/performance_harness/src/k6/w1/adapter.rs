//! K6b's adapter: a K6 kernel model as K4's `SourceParts` (T3 K6b plan §3.2).
//!
//! - Nodes in the model's order (R1's order for RF-LARGE); node k is node k.
//! - Member k (0-based, in the model's order) has id k + 1, R1's 1-based member
//!   number, with the model's section and `y_reference` (P1's rule for
//!   RF-LARGE, ROOT's K6 ruling N4).
//! - Every restrained DOF is a constraint at 0.0.
//! - One nodal load per nonzero (DOF, value), with source id `k6:<global DOF>`.
//! - One station per member at fraction 0.5 (id = the member id), as K4's own
//!   adapter has, for R1's mid-span bending rows.
//! - No springs, directional springs or support groups: RF-LARGE and the
//!   DEC-053 nine have none.
//!
//! `K6Model` holds exact binary64 values already, so nothing is rounded here.

use super::super::models::K6Model;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    Constraint, Dof, NodalLoad, PrimitiveSource, SourceError, SourceParts, Station, StraightMember,
};

/// The fraction of every member's station.
pub const STATION_FRACTION: f64 = 0.5;

/// The ledger source id of the load at a global DOF.
pub fn load_source_id(global_dof: usize) -> String {
    format!("k6:{global_dof}")
}

/// A member's K4 id: its 1-based position in the model's member order.
pub fn member_id(index: usize) -> u32 {
    u32::try_from(index + 1).expect("fewer than 2^32 members")
}

fn dof(global: usize) -> Dof {
    Dof::from_global(global)
}

/// The model's source parts (module documentation).
pub fn source_parts(model: &K6Model) -> SourceParts {
    let s = &model.section;
    let node = |i: usize| u32::try_from(i).expect("fewer than 2^32 nodes");
    SourceParts {
        nodes: model.nodes.iter().map(|(_, p)| *p).collect(),
        members: model
            .members
            .iter()
            .enumerate()
            .map(|(k, (_, i, j, y))| StraightMember {
                id: member_id(k),
                node_i: node(*i),
                node_j: node(*j),
                elastic_modulus: s.elastic_modulus,
                shear_modulus: s.shear_modulus,
                area: s.area,
                second_moment_y: s.second_moment_y,
                second_moment_z: s.second_moment_z,
                torsion_constant: s.torsion_constant,
                y_reference: *y,
            })
            .collect(),
        constraints: model
            .restrained_dofs()
            .into_iter()
            .map(|g| Constraint {
                dof: dof(g),
                value: 0.0,
            })
            .collect(),
        loads: model
            .loads
            .iter()
            .map(|&(g, value)| NodalLoad {
                dof: dof(g),
                value,
                source_id: load_source_id(g),
            })
            .collect(),
        stations: (0..model.member_count())
            .map(|k| Station {
                id: member_id(k),
                member: member_id(k),
                fraction: STATION_FRACTION,
            })
            .collect(),
        ..SourceParts::default()
    }
}

/// The model's `PrimitiveSource`, or K4's validation refusal.
pub fn source(model: &K6Model) -> Result<PrimitiveSource, SourceError> {
    PrimitiveSource::new(source_parts(model))
}
