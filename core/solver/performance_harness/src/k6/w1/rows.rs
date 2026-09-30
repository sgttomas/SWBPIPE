//! K6b's published-row keys, the rows dump (`k6b-rows v1`) and R1's derived
//! member quantities (T3 K6b plan §3.4, §5.4).
//!
//! Keys: `u.<node>.<c>` (displacements and rotations, c = 0..5 in UX..RZ),
//! `mag.<node>`, `end.<member>.<i|j>.<c>`, `st.<station>.<c>`,
//! `spr.<spring>.<c>`, `dspr.<spring>.<c>`, `R.<node>.<c>`, `sf.<group>`,
//! `sm.<group>`. They are R1's numeric keys (`K4T/r1_large.txt`), since the
//! adapter numbers members and stations as R1 does.
//!
//! R1's member quantities are derived from the published rows, never
//! differenced: N = end.j.0, T = end.j.3, Mb = |(M_y, M_z)| at each end and at
//! the station, tw = T/k_t and ext = N/k_a with k_t = fl(fl(G·J)/L) and
//! k_a = fl(fl(E·A)/L), L the frame's binary64 length. The magnitude is
//! sqrt(a² + b²) with an exact power-of-two scaling, never `hypot`.

use super::super::models::K6Model;
use open_pipe_stress_frame_kernel::structural::retained_api::{
    Binary64Outcome, End, Kind, Publication, QuantityId, RowClass,
};
use std::collections::BTreeMap;
use std::io::Write;

/// The row's key.
pub fn key(id: &QuantityId) -> String {
    match *id {
        QuantityId::Displacement(d) => format!("u.{}.{}", d.node, d.component.index()),
        QuantityId::DisplacementMagnitude(n) => format!("mag.{n}"),
        QuantityId::EndAction {
            member,
            end,
            component,
        } => format!(
            "end.{member}.{}.{}",
            if end == End::I { "i" } else { "j" },
            component.index()
        ),
        QuantityId::StationAction { station, component } => {
            format!("st.{station}.{}", component.index())
        }
        QuantityId::SpringAction { spring, component } => {
            format!("spr.{spring}.{}", component.index())
        }
        QuantityId::DirectionalSpringAction { spring, component } => {
            format!("dspr.{spring}.{}", component.index())
        }
        QuantityId::Reaction(d) => format!("R.{}.{}", d.node, d.component.index()),
        QuantityId::SupportForceMagnitude(g) => format!("sf.{g}"),
        QuantityId::SupportMomentMagnitude(g) => format!("sm.{g}"),
    }
}

pub fn kind_str(kind: Kind) -> &'static str {
    match kind {
        Kind::Translation => "translation",
        Kind::Rotation => "rotation",
        Kind::Force => "force",
        Kind::Moment => "moment",
    }
}

pub fn class_str(class: RowClass) -> &'static str {
    match class {
        RowClass::RelativeVerified => "relative_verified",
        RowClass::AbsoluteVerified { .. } => "absolute_verified",
        RowClass::InputDerived => "input_derived",
        RowClass::Unpublishable => "unpublishable",
    }
}

/// The value token: the 16 hex digits of the binary64, or the range marker.
pub fn value_token(value: &Binary64Outcome) -> String {
    match *value {
        Binary64Outcome::Normal(v) | Binary64Outcome::Subnormal { value: v, .. } => {
            format!("{:016x}", v.to_bits())
        }
        Binary64Outcome::Underflow { negative } => {
            format!("underflow{}", if negative { '-' } else { '+' })
        }
        Binary64Outcome::Overflow { negative } => {
            format!("overflow{}", if negative { '-' } else { '+' })
        }
    }
}

/// Counts of rows by class: relative, absolute, input-derived, unpublishable.
pub fn class_counts(publication: &Publication) -> [usize; 4] {
    let mut out = [0; 4];
    for row in &publication.rows {
        out[match row.class {
            RowClass::RelativeVerified => 0,
            RowClass::AbsoluteVerified { .. } => 1,
            RowClass::InputDerived => 2,
            RowClass::Unpublishable => 3,
        }] += 1;
    }
    out
}

/// Writes the rows dump: a header, then `key kind body class bound_bits value`
/// per row, in the layout's order.
pub fn write_rows<W: Write>(
    model_id: &str,
    publication: &Publication,
    out: &mut W,
) -> std::io::Result<()> {
    writeln!(out, "k6b-rows v1")?;
    writeln!(out, "model {model_id}")?;
    for row in &publication.rows {
        let bound = match row.class {
            RowClass::AbsoluteVerified { bound_bits } => format!("{bound_bits:016x}"),
            _ => "-".to_string(),
        };
        writeln!(
            out,
            "{} {} {} {} {} {}",
            key(&row.id),
            kind_str(row.kind),
            row.body,
            class_str(row.class),
            bound,
            value_token(&row.value)
        )?;
    }
    Ok(())
}

/// sqrt(a² + b²), with an exact power-of-two scaling when the squares would
/// overflow or underflow (not `hypot`, whose precision is unspecified).
pub fn magnitude(a: f64, b: f64) -> f64 {
    let m = a.abs().max(b.abs());
    let (down, up) = if m > f64::from_bits(0x5F30_0000_0000_0000) {
        (
            f64::from_bits(0x1A70_0000_0000_0000),
            f64::from_bits(0x6570_0000_0000_0000),
        )
    } else if m > 0.0 && m < f64::from_bits(0x20B0_0000_0000_0000) {
        (
            f64::from_bits(0x6570_0000_0000_0000),
            f64::from_bits(0x1A70_0000_0000_0000),
        )
    } else {
        (1.0, 1.0)
    };
    let (x, y) = (a * down, b * down);
    (x * x + y * y).sqrt() * up
}

/// The published values by key, and R1's derived member quantities (module
/// documentation). Rows without a binary64 value are left out.
pub fn r1_values(model: &K6Model, publication: &Publication) -> BTreeMap<String, f64> {
    let mut out: BTreeMap<String, f64> = publication
        .rows
        .iter()
        .filter_map(|r| r.value.value().map(|v| (key(&r.id), v)))
        .collect();
    let s = &model.section;
    let frames = model.frames().ok();
    for k in 0..model.member_count() {
        let id = k + 1;
        let get = |key: String| out.get(&key).copied();
        let n = get(format!("end.{id}.j.0"));
        let t = get(format!("end.{id}.j.3"));
        let mut extra = Vec::new();
        for end in ["i", "j"] {
            if let (Some(my), Some(mz)) = (
                get(format!("end.{id}.{end}.4")),
                get(format!("end.{id}.{end}.5")),
            ) {
                extra.push((format!("Mb.{id}.{end}"), magnitude(my, mz)));
            }
        }
        if let (Some(my), Some(mz)) = (get(format!("st.{id}.4")), get(format!("st.{id}.5"))) {
            extra.push((format!("Mbs.{id}"), magnitude(my, mz)));
        }
        if let Some(n) = n {
            extra.push((format!("N.{id}"), n));
        }
        if let Some(t) = t {
            extra.push((format!("T.{id}"), t));
        }
        if let Some(length) = frames.as_ref().and_then(|f| f[k].length().ok()) {
            let kt = (s.shear_modulus * s.torsion_constant) / length;
            let ka = (s.elastic_modulus * s.area) / length;
            if let Some(t) = t {
                extra.push((format!("tw.{id}"), t / kt));
            }
            if let Some(n) = n {
                extra.push((format!("ext.{id}"), n / ka));
            }
        }
        out.extend(extra);
    }
    out
}

/// R1's predicate, unchanged: |obs − exp| ≤ 1e-9·max(|exp|, scale).
pub fn r1_passes(obs: f64, exp: f64, scale: f64) -> bool {
    (obs - exp).abs() <= 1e-9 * exp.abs().max(scale)
}
