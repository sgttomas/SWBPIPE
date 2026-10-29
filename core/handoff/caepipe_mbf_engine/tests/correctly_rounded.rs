//! The correctly rounded acos, asin, atan2, sin and tan (`src/correctly_rounded.rs`) at the inputs the Python twin's
//! `tests/test_caepipe_mbf_correctly_rounded.py` names: the hard-to-round cases it quotes from the CORE-MATH
//! worst-case files, its argument-reduction worst cases, a large reduction, and the node 20B repro of review r1. The
//! expected values are the Python module's, which that test holds to an independent decimal oracle; the sampled
//! cross-check is `numeric.correctly_rounded` in the parity vectors (`tests/parity.rs`).
//!
//! Standard claim fence applies (claims taxonomy per DEC-081): these tests check the floating-point evaluation only.

use open_pipe_stress_caepipe_mbf::{self as mbf, ports, ImportOptions, Value};

type Case = (&'static str, &'static str, &'static [u64], u64);

fn call(name: &str, args: &[u64]) -> u64 {
    let x = f64::from_bits(args[0]);
    let result = match name {
        "sin" => ports::sin(x),
        "tan" => ports::tan(x),
        "asin" => ports::asin(x),
        "acos" => ports::acos(x),
        "atan2" => ports::atan2(x, f64::from_bits(args[1])),
        _ => unreachable!("{name}"),
    };
    result.expect("a value").to_bits()
}

/// Each is quoted from the named section of the CORE-MATH file `src/binary64/<function>/<function>.wc`, as the Python
/// test quotes it; each lies within 2^-40 ulp of a rounding boundary.
const HARD_CASES: &[Case] = &[
    ("sin", "0x1.997d35866ce04p-14", &[0x3f1997d35866ce04], 0x3f1997d357b82f1b),
    ("sin", "0x1.4c377e1d8441fp-6", &[0x3f94c377e1d8441f], 0x3f94c31aa32e3bbe),
    ("sin", "0x1.0380e8e041344p+1023", &[0x7fe0380e8e041344], 0xbfbd4e10ebb56ad8),
    ("sin", "0x1.e3cf476542bd2p-25", &[0x3e6e3cf476542bd2], 0x3e6e3cf476542bcd),
    ("tan", "0x1.dffffffffff1fp-22", &[0x3e9dffffffffff1f], 0x3e9e000000000151),
    ("tan", "0x1.bf9f47347b021p-8", &[0x3f7bf9f47347b021], 0x3f7bfa10f640839b),
    ("tan", "0x1.da898d9a94d29p+4", &[0x403da898d9a94d29], 0x4015316231d4c189),
    ("tan", "0x1.4e1f2d2cbbca8p+383", &[0x57e4e1f2d2cbbca8], 0xbffe984e7ea3df1b),
    ("asin", "0x1.27225319ee943p-15", &[0x3f027225319ee943], 0x3f02722531af4168),
    ("asin", "0x1.06800e629d667p-23", &[0x3e806800e629d667], 0x3e806800e629d672),
    ("asin", "0x1.255e30ce85775p-4", &[0x3fb255e30ce85775], 0x3fb259e8cc869b98),
    ("acos", "0x1.d18f3b5c96858p-3", &[0x3fcd18f3b5c96858], 0x3ff576a69fcfcc3b),
    ("acos", "0x1.cf37d41c69889p-24", &[0x3e7cf37d41c69889], 0x3ff921fb3750afd6),
    ("acos", "0x1.d0bfb34fbe039p-4", &[0x3fbd0bfb34fbe039], 0x3ff7503adab13d54),
    ("acos", "-0x1.8b79e6cb1a3e8p-19", &[0xbec8b79e6cb1a3e8], 0x3ff921fe6b37faae),
    ("acos", "-0x1.39676733ae8fep-52", &[0xbcb39676733ae8fe], 0x3ff921fb54442d19),
    ("atan2", "0xb.cd9b43114a29p+76,0x1.47362dee55845p+52", &[0x44e79b3686229452, 0x43347362dee55845], 0x3ff921fb52889e27),
    ("atan2", "0x1.291d62491a95bp+52,0x1.9d70bcd22dbf4p+52", &[0x433291d62491a95b, 0x4339d70bcd22dbf4], 0x3fe3f0a78e8ec08b),
];

/// In their binades these inputs come closest to a multiple of 2 pi or of pi (CORE-MATH `sin.wc`), and
/// -0x1.ba2c250b601fcp+33 is the large argument the previous port left to the platform library.
const REDUCTION_CASES: &[Case] = &[
    ("sin", "0x1.0d09901565674p+614", &[0x6650d09901565674], 0xbcd09436b6278822),
    ("tan", "0x1.0d09901565674p+614", &[0x6650d09901565674], 0xbcd09436b6278822),
    ("sin", "0x1.140e5e9156e6cp+621", &[0x66c140e5e9156e6c], 0x3cd6b1d5cb97bc4e),
    ("tan", "0x1.140e5e9156e6cp+621", &[0x66c140e5e9156e6c], 0x3cd6b1d5cb97bc4e),
    ("sin", "0x1.a71df2723e34cp+750", &[0x6eda71df2723e34c], 0xbcb35e2e7b984dea),
    ("tan", "0x1.a71df2723e34cp+750", &[0x6eda71df2723e34c], 0x3cb35e2e7b984dea),
    ("sin", "0x1.54a28326cfedep+268", &[0x50b54a28326cfede], 0x3ca0618e0418c17a),
    ("tan", "0x1.54a28326cfedep+268", &[0x50b54a28326cfede], 0xbca0618e0418c17a),
    ("sin", "-0x1.ba2c250b601fcp+33", &[0xc20ba2c250b601fc], 0xbfec38c488b942bb),
    ("tan", "-0x1.ba2c250b601fcp+33", &[0xc20ba2c250b601fc], 0xbffdef67af1d2fea),
];

#[test]
fn hard_to_round_cases_from_the_literature() {
    let failures: Vec<String> = HARD_CASES
        .iter()
        .filter(|(name, _, args, want)| call(name, args) != *want)
        .map(|(name, text, args, want)| format!("{name}({text}) = {:#018x}, not {want:#018x}", call(name, args)))
        .collect();
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn argument_reduction_worst_cases_and_their_negations() {
    let mut failures = Vec::new();
    for (name, text, args, want) in REDUCTION_CASES {
        let negated = [(-f64::from_bits(args[0])).to_bits()];
        let want_negated = (-f64::from_bits(*want)).to_bits();
        if call(name, args) != *want || call(name, &negated) != want_negated {
            failures.push(format!("{name}({text})"));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

/// The two calls the importer makes for the repro's bend (theta = acos(cosine), tangent length = R tan(theta / 2)).
/// macOS's C library returns 1.793430522422298 and 1.2516923300464655 here.
const REPRO_CALLS: &[(&str, u64, u64)] = &[
    ("acos", 0xbfcc4329053dc30a, 0x3ffcb1e434173b40), // acos(-0x1.c4329053dc30ap-3) = 0x1.cb1e434173b40p+0
    ("tan", 0x3fecb1e434173b40, 0x3ff406ee896341fb), // tan(0x1.cb1e434173b40p-1) = 0x1.406ee896341fbp+0
];

#[test]
fn the_trig_values_that_feed_node_20b() {
    for (name, x, want) in REPRO_CALLS {
        assert_eq!(call(name, &[*x]), *want, "{name}");
    }
}

#[test]
fn node_20b_is_the_correctly_rounded_value() {
    // The Python test's REPRO_LAYOUT through its `mbf()` helper (an invented model), imported as its `run()` does.
    let data = "INVENTED IMPORTER TEST\r\nVERSION\r\n15.10\r\nOPTIONS\r\nHGRA,SI\r\nOPTEMP\r\n20\r\nMATERIAL\r\n\
M1,0,0.3,1,1,CS,invented steel\r\nM1,20,200000,1.2e-05\r\nPIPE\r\nP1,168.3,7.11,0,0,0,0,0,0\r\nLAYOUT\r\n\
F10,KA,MM1,PP1\r\nT20,KI,X10000,B100\r\nT30,X-1975,Y8724\r\n";
    let mut options = ImportOptions::new("invented-test");
    let declaration = br#"{"*": {"origin": "project_invented", "source_name": "invented importer test material"}}"#;
    options.material_provenance = Some(mbf::parse_sidecar_json(declaration).expect("declaration"));
    let result = mbf::import(data.as_bytes(), &options).expect("a result");
    assert_eq!(result.status, "imported", "{:?}", result.diagnostics);
    let model = result.model.as_ref().expect("model");
    let nodes = model.get("nodes").and_then(Value::as_list).expect("nodes");
    let node = nodes.iter().find(|node| node.get("id").and_then(Value::as_str) == Some("node:20B")).expect("node:20B");
    let y = node.get("position").and_then(|position| position.get("y")).and_then(Value::as_f64).expect("y");
    assert_eq!(y, 122.07995677558971);
}
