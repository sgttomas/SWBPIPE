//! T4-I29: platform independence of the curved element's formation. No libm
//! call forms φ, K or the uniform-load vector (φ is FK's
//! `twice_atan2_nonnegative`, checked bitwise against correctly rounded
//! references in FK; every later step is a fixed sequence of IEEE 754 + − × ÷
//! √, which Rust neither contracts nor reorders). So the binary64 bits below
//! are the same on every IEEE 754 binary64 target, and a CI host compares
//! its own formation against them. Each digest is FNV-1a (64-bit) over the
//! little-endian bits of φ, then the global K row by row, then (where
//! listed) the global consistent load vector for w = (1, −2, 3) N/m.
use super::*;
use open_pipe_stress_frame_kernel::FrameNode;

struct Frozen {
    label: &'static str,
    xi: [f64; 3],
    xj: [f64; 3],
    radius: f64,
    y: [f64; 3],
    factors: (f64, f64),
    phi_bits: u64,
    k_digest: u64,
    /// Digest including the load vector (`None`: not frozen). Below the
    /// series switch it is T4-U1b's series path (`tip_deflection_by_series`).
    load_digest: Option<u64>,
}

const SECTION: (f64, f64, f64, f64, f64) = (
    200e9,
    80e9,
    0.005969026041820614,
    2.700984283923829e-05,
    5.401968567847658e-05,
);

const INTENSITY: [f64; 3] = [1.0, -2.0, 3.0];

const FROZEN: [Frozen; 4] = [
    // f1b_w2's T0R quarter arc (s = c to one ulp): the case whose libm φ
    // was one ulp below the correctly rounded φ on one platform. Each φ
    // below is the correctly rounded 2·atan2(s, c) of its s and c (T4-I29's
    // decimal reference).
    Frozen {
        label: "t0r_quarter",
        xi: [1.0, 0.0, 0.0],
        xj: [1.2, 0.2, 0.0],
        radius: 0.2,
        y: [0.0, 0.0, 1.0],
        factors: (1.0, 1.0),
        phi_bits: 0x3ff921fb54442d18,
        k_digest: 0x82e9fae73fe184ea,
        load_digest: Some(0xdd1b375e35908141),
    },
    // A short arc below the series switch (s < 1/2); its load digest is
    // T4-U1b's series path.
    Frozen {
        label: "short_series",
        xi: [3.0, -1.0, 2.0],
        xj: [3.05, -0.99, 1.98],
        radius: 0.3,
        y: [0.3, 1.0, 0.2],
        factors: (1.0, 1.0),
        phi_bits: 0x3fc766ee6f97bf88,
        k_digest: 0xa960942f1d683461,
        load_digest: Some(0x1a0c795a12ec77dd),
    },
    // s = 1/2 exactly (L = R): the first closed-form arc.
    Frozen {
        label: "switch_closed_form",
        xi: [0.0, 0.0, 0.0],
        xj: [0.3, 0.4, 0.0],
        radius: 0.5,
        y: [-1.0, 1.0, 3.0],
        factors: (2.5, 1.7),
        phi_bits: 0x3ff0c152382d7366,
        k_digest: 0x339995d1ca3ae0f1,
        load_digest: Some(0xd774f32b9183a6a5),
    },
    // A near-π arc on a skew chord.
    Frozen {
        label: "near_pi_skew",
        xi: [-2.0, 5.0, 1.0],
        xj: [-0.85, 6.15, 2.15],
        radius: 1.0,
        y: [0.0, 0.0, 1.0],
        factors: (1.0, 1.0),
        phi_bits: 0x4007b04580d8398c,
        k_digest: 0x739f4ee142ed53ac,
        load_digest: Some(0x021dd56871b62c49),
    },
];

fn fnv1a(bits: impl IntoIterator<Item = u64>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for value in bits {
        for byte in value.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

#[test]
fn formation_bits_are_frozen() {
    let (e, g, a, i, j) = SECTION;
    let mut failures = Vec::new();
    for case in &FROZEN {
        let element = CurvedBendMacroElement::new(
            FrameNode::new(0, case.xi).unwrap(),
            FrameNode::new(1, case.xj).unwrap(),
            case.radius,
            case.y,
            e,
            g,
            a,
            i,
            j,
            case.factors.0,
            case.factors.1,
        )
        .unwrap();
        let phi = element.included_angle().unwrap();
        let k = element.global_stiffness().unwrap();
        let mut bits: Vec<u64> = vec![phi.to_bits()];
        bits.extend(k.iter().flatten().map(|v| v.to_bits()));
        let k_digest = fnv1a(bits.iter().copied());
        let loads = element.consistent_uniform_nodal_loads(INTENSITY).unwrap();
        bits.extend(loads.iter().map(|v| v.to_bits()));
        let load_digest = fnv1a(bits.iter().copied());
        let got = (
            phi.to_bits(),
            k_digest,
            case.load_digest.map(|_| load_digest),
        );
        let want = (case.phi_bits, case.k_digest, case.load_digest);
        if got != want {
            failures.push(format!(
                "{}: phi_bits: 0x{:016x}, k_digest: 0x{:016x}, load_digest: 0x{:016x} (want {:x?})",
                case.label, got.0, got.1, load_digest, want
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
