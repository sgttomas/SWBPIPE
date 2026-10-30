//! K6's canonical model serialization, `k6-model v1` (ROOT's K6 ruling Q11):
//! ASCII, LF line ends, no trailing space, one final newline, every float as
//! the 16 lower-case hex digits of its bits. Its sha256 is computed in Python.
//!
//! ```text
//! k6-model v1
//! id <id>
//! source <source>
//! counts nodes <N> members <m> loads <L>
//! section E <hex> G <hex> A <hex> Iy <hex> Iz <hex> J <hex>
//! node <index> <label> <x> <y> <z>
//! member <index> <label> <i> <j> <yref_x> <yref_y> <yref_z>
//! restraint <node> <UX UY UZ RX RY RZ as a 0/1 string>
//! load <global dof> <value>
//! ```

use super::models::{Family, K6Model};
use open_pipe_stress_frame_kernel::FrameSection;
use std::fmt::Write;

pub const HEADER: &str = "k6-model v1";

fn hex(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

/// The model's canonical bytes.
pub fn serialize(model: &K6Model) -> String {
    let mut out = String::new();
    write_canonical(model, &mut out);
    out
}

/// Streams the canonical bytes into `out` (a hasher, for example), so no
/// model-sized string need be built.
pub fn write_canonical<W: Write>(model: &K6Model, out: &mut W) {
    let s = &model.section;
    let _ = writeln!(out, "{HEADER}");
    let _ = writeln!(out, "id {}", model.id);
    let _ = writeln!(out, "source {}", model.source);
    let _ = writeln!(
        out,
        "counts nodes {} members {} loads {}",
        model.nodes.len(),
        model.members.len(),
        model.loads.len()
    );
    let _ = writeln!(
        out,
        "section E {} G {} A {} Iy {} Iz {} J {}",
        hex(s.elastic_modulus),
        hex(s.shear_modulus),
        hex(s.area),
        hex(s.second_moment_y),
        hex(s.second_moment_z),
        hex(s.torsion_constant)
    );
    for (index, (label, p)) in model.nodes.iter().enumerate() {
        let _ = writeln!(
            out,
            "node {index} {label} {} {} {}",
            hex(p[0]),
            hex(p[1]),
            hex(p[2])
        );
    }
    for (index, (label, i, j, y)) in model.members.iter().enumerate() {
        let _ = writeln!(
            out,
            "member {index} {label} {i} {j} {} {} {}",
            hex(y[0]),
            hex(y[1]),
            hex(y[2])
        );
    }
    for (node, mask) in &model.restraints {
        let bits: String = mask.iter().map(|&r| if r { '1' } else { '0' }).collect();
        let _ = writeln!(out, "restraint {node} {bits}");
    }
    for (dof, value) in &model.loads {
        let _ = writeln!(out, "load {dof} {}", hex(*value));
    }
}

fn unhex(token: &str) -> Result<f64, String> {
    if token.len() != 16 {
        return Err(format!("not a 16-digit float: {token}"));
    }
    u64::from_str_radix(token, 16)
        .map(f64::from_bits)
        .map_err(|e| format!("{token}: {e}"))
}

fn number(token: &str) -> Result<usize, String> {
    token.parse().map_err(|e| format!("{token}: {e}"))
}

/// Parses canonical bytes back into a model. The family is taken from the id
/// (R1's family, DEC-053, or a Q5 grid), and serialization of the result must
/// give the same bytes (checked here).
pub fn parse(text: &str) -> Result<K6Model, String> {
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err("missing k6-model v1 header".to_string());
    }
    let mut take = |key: &str| -> Result<String, String> {
        let line = lines.next().ok_or(format!("missing {key} line"))?;
        line.strip_prefix(key)
            .and_then(|rest| rest.strip_prefix(' '))
            .map(str::to_string)
            .ok_or(format!("expected {key}: {line}"))
    };
    let id = take("id")?;
    let source = take("source")?;
    let counts: Vec<String> = take("counts")?.split(' ').map(str::to_string).collect();
    let section_tokens: Vec<String> = take("section")?.split(' ').map(str::to_string).collect();
    if counts.len() != 6 || section_tokens.len() != 12 {
        return Err("malformed counts or section line".to_string());
    }
    let (node_count, member_count, load_count) = (
        number(&counts[1])?,
        number(&counts[3])?,
        number(&counts[5])?,
    );
    let value = |k: usize| unhex(&section_tokens[2 * k + 1]);
    let section = FrameSection::new(
        value(0)?,
        value(1)?,
        value(2)?,
        value(3)?,
        value(4)?,
        value(5)?,
    )
    .map_err(|e| format!("{e:?}"))?;
    let mut model = K6Model {
        family: family_of(&id),
        id,
        source,
        section,
        nodes: Vec::with_capacity(node_count),
        members: Vec::with_capacity(member_count),
        restraints: Vec::new(),
        loads: Vec::with_capacity(load_count),
    };
    for line in lines {
        let t: Vec<&str> = line.split(' ').collect();
        match t.first().copied() {
            Some("node") if t.len() == 6 => model
                .nodes
                .push((t[2].to_string(), [unhex(t[3])?, unhex(t[4])?, unhex(t[5])?])),
            Some("member") if t.len() == 8 => model.members.push((
                t[2].to_string(),
                number(t[3])?,
                number(t[4])?,
                [unhex(t[5])?, unhex(t[6])?, unhex(t[7])?],
            )),
            Some("restraint") if t.len() == 3 && t[2].len() == 6 => {
                let mut mask = [false; 6];
                for (d, c) in t[2].chars().enumerate() {
                    mask[d] = c == '1';
                }
                model.restraints.push((number(t[1])?, mask));
            }
            Some("load") if t.len() == 3 => model.loads.push((number(t[1])?, unhex(t[2])?)),
            _ => return Err(format!("malformed line: {line}")),
        }
    }
    if model.nodes.len() != node_count
        || model.members.len() != member_count
        || model.loads.len() != load_count
    {
        return Err("counts line disagrees with the body".to_string());
    }
    if serialize(&model) != text {
        return Err("the text is not in canonical form".to_string());
    }
    Ok(model)
}

/// Same parse and drop edges, with inline construction provenance.
pub fn parse_described(text: &str) -> Result<(K6Model, super::models::ModelOrigin), String> {
    parse(text).map(|model| {
        (
            model,
            super::models::ModelOrigin(super::models::ModelRecipe::Canonical),
        )
    })
}

fn family_of(id: &str) -> Family {
    if id.starts_with("RF-LARGE-CHAIN") || id.starts_with("K6-CEIL-CHAIN") {
        Family::Chain
    } else if id.starts_with("RF-LARGE-TREE") {
        Family::Tree
    } else if id.starts_with("RF-LARGE-CONT") {
        Family::Cont
    } else if id.starts_with("K6-GRID") {
        Family::Grid
    } else {
        Family::Dec053
    }
}
