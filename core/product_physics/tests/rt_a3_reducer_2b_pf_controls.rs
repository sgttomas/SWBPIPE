//! RT-A3 reducer increment 2b: the W-15c helper extraction
//! (`rigid_component::judge_reducer`, `body_ends`, `member_mappings`) leaves
//! every existing offset judgment unchanged. Each control below is solved
//! through PP's public entry on the pressure-free route and on the exact
//! route with the body outside every pressure region (empty regions), and
//! its observable outcome (mechanics status, every diagnostic's ID, code,
//! severity, message and references in emission order, and every result
//! row's ID, kind and value bits) is digested.
//!
//! The expected digests were produced by this file against the selected
//! source 393a11ee75dcc1836d2fd586c842f90312a6023d (merged RT-A3 reducer 2a),
//! before the extraction, and are compared verbatim after it. A changed digest
//! is a behaviour change of the extracted W-15c method, never a value to
//! refresh. Set `RT_A3_2B_PRINT_DIGESTS=1` to print the observed table.
//!
//! Every model is invented: no catalog, vendor, material or code value.
use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const PROV: &str = "rt-a3 reducer 2b invented PF control; not library, component, material-catalog or code data";

#[derive(Clone, Copy, PartialEq, Debug)]
enum Route {
    Preview,
    ExactOutsideRegions,
}

fn qv(value: f64, unit: &str) -> Value {
    json!({"value": value, "unit": unit})
}

struct Model {
    length_unit: &'static str,
    nodes: Vec<(&'static str, [f64; 3])>,
    members: Vec<(&'static str, &'static str, &'static str)>,
    components: Vec<Value>,
}

fn request(m: &Model, route: Route) -> Value {
    let nodes: Vec<Value> = m.nodes.iter().map(|(id, p)| json!({"id": id, "position": {"x": p[0], "y": p[1], "z": p[2]}, "provenance": PROV})).collect();
    let pipes: Vec<Value> = m.members.iter().map(|(id, from, to)| {
        let mut sec = json!({"outside_diameter": qv(0.2, "m"), "wall_thickness": qv(0.01, "m")});
        if route == Route::ExactOutsideRegions {
            sec["corrosion_allowance"] = qv(0.0, "m");
        }
        json!({"id": id, "from": from, "to": to, "section": sec, "material": "material:pf2b",
            "y_reference": {"x": 0.0, "y": 0.0, "z": 1.0}, "provenance": PROV})
    }).collect();
    let units = json!({"length": m.length_unit, "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"});
    let status = json!({"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"});
    let tip = m.nodes.last().unwrap().0;
    let loads = json!([
        {"id": "load:fx", "category": "concentrated_force", "target": {"type": "node", "node": tip}, "direction": "global_x",
            "magnitude": qv(10000.0, "N"), "dimension": "force", "provenance": PROV},
        {"id": "load:fz", "category": "concentrated_force", "target": {"type": "node", "node": tip}, "direction": "global_z",
            "magnitude": qv(-3000.0, "N"), "dimension": "force", "provenance": PROV}]);
    let mut case = json!({"id": "case:pf2b", "primitive_loads": loads, "provenance": PROV});
    let mut model = json!({"schema_version": "0.9.0", "document_kind": "openpipestress.product_preview.model",
        "project": {"id": "project:pf2b", "units": units}, "analysis_status": status, "nodes": nodes, "pipe_segments": pipes,
        "supports": [{"id": "support:n0", "node": m.nodes[0].0, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV}],
        "components": m.components, "combinations": []});
    match route {
        Route::Preview => {
            model["materials"] = json!([{"id": "material:pf2b", "elastic_modulus": qv(200e9, "Pa"), "shear_modulus": qv(80e9, "Pa"),
                "thermal_expansion_coefficient": qv(1.2e-5, "1/degC"), "provenance": PROV}]);
        }
        Route::ExactOutsideRegions => {
            model["pressure_contract"] = json!({"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"});
            model["materials"] = json!([{"id": "material:pf2b", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                "elastic_modulus": qv(200e9, "Pa"), "poisson_ratio": qv(0.3, "1"), "provenance": PROV}]);
            case["pressure_regions"] = json!([]);
        }
    }
    model["load_cases"] = json!([case]);
    json!({"model": model, "materials": []})
}

/// A reducer at `node` mapping `member`, `offset` in mm (absent when None).
fn reducer(id: &str, node: &str, member: &str, offset: Option<[f64; 3]>, consumption: &str) -> Value {
    let mut geometry = json!({"rigid_pipe_ref": member});
    if let Some(e) = offset {
        geometry["eccentric_offset"] = json!({"x": e[0], "y": e[1], "z": e[2], "unit": "mm"});
    }
    json!({"id": id, "kind": "reducer", "node": node, "geometry": geometry,
        "modifiers": {"stiffness_scaling_user_value": qv(2.0, "1"), "source_reference": "invented RT-A3 2b PF control scale"},
        "mechanics_interface": {"solver_consumption": consumption}, "provenance": PROV})
}

/// Anchor n0, upstream n0 → a along X (length `up`), body a → b with chord
/// (0.4, 0, rise), downstream b → c along X (length `down`).
fn run(up: f64, rise: f64, down: f64, offset: Option<[f64; 3]>) -> Model {
    Model {
        length_unit: "m",
        nodes: vec![("node:n0", [0.0, 0.0, 0.0]), ("node:a", [up, 0.0, 0.0]), ("node:b", [up + 0.4, 0.0, rise]), ("node:c", [up + 0.4 + down, 0.0, rise])],
        members: vec![("pipe:up", "node:n0", "node:a"), ("pipe:body", "node:a", "node:b"), ("pipe:dn", "node:b", "node:c")],
        components: vec![reducer("component:r", "node:a", "pipe:body", offset, "mechanics_geometry_only")],
    }
}

/// The controls: (label, model).
fn controls() -> Vec<(&'static str, Model)> {
    let mut list = vec![
        ("collinear absent", run(2.6, 0.0, 2.0, None)),
        ("collinear zero", run(2.6, 0.0, 2.0, Some([0.0, 0.0, 0.0]))),
        ("collinear signed zero", run(2.6, 0.0, 2.0, Some([-0.0, 0.0, -0.0]))),
        ("eccentric realized", run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]))),
        ("eccentric absent", run(2.6, 0.025, 2.0, None)),
        ("eccentric zero mismatch", run(2.6, 0.025, 2.0, Some([0.0, 0.0, 0.0]))),
        ("eccentric wrong sign", run(2.6, 0.025, 2.0, Some([0.0, 0.0, -25.0]))),
        ("rule-B band", run(2.6, 0.025, 2.0, Some([0.0, 0.0, 19.8]))),
        ("beyond rule-B band", run(2.6, 0.025, 2.0, Some([0.0, 0.0, 19.7]))),
        ("below precision", run(2.6, 0.0, 2.0, Some([0.0, 0.0, 0.8]))),
        ("not transverse", run(2.6, 0.025, 2.0, Some([5.0, 0.0, 25.0]))),
        ("J3 tie", run(2.0, 0.025, 2.0, Some([0.0, 0.0, 25.0]))),
        ("unresolvable short neighbour", run(2.6, 0.025, 0.0015, Some([0.0, 0.0, 25.0]))),
        ("marker with offset", {
            let mut m = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
            m.components = vec![reducer("component:r", "node:a", "pipe:body", Some([0.0, 0.0, 25.0]), "not_solver_consumed")];
            m
        }),
        ("mapping invalid", {
            let mut m = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
            m.components = vec![reducer("component:r", "node:c", "pipe:body", Some([0.0, 0.0, 25.0]), "mechanics_geometry_only")];
            m
        }),
        ("origin at the other end", {
            let mut m = run(2.6, 0.025, 2.0, Some([0.0, 0.0, -25.0]));
            m.components = vec![reducer("component:r", "node:b", "pipe:body", Some([0.0, 0.0, -25.0]), "mechanics_geometry_only")];
            m
        }),
        ("body reversed", {
            let mut m = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
            m.members[1] = ("pipe:body", "node:b", "node:a");
            m
        }),
        ("millimetre model", {
            let mut m = run(2600.0, 25.0, 2000.0, Some([0.0, 0.0, 25.0]));
            m.length_unit = "mm";
            m.nodes[2].1 = [3000.0, 0.0, 25.0];
            m.nodes[3].1 = [5000.0, 0.0, 25.0];
            m
        }),
    ];
    // Run directions disagree: the downstream member tilts 0.01 rad.
    let mut disagree = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
    disagree.nodes[3].1 = [5.0, 0.0, 0.025 + 0.02];
    list.push(("directions disagree", disagree));
    // Ambiguous: a second member at b.
    let mut ambiguous = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
    ambiguous.nodes.insert(3, ("node:s", [3.0, 1.0, 0.025]));
    ambiguous.members.push(("pipe:side", "node:b", "node:s"));
    list.push(("ambiguous", ambiguous));
    // Unsupported: the downstream member is another reducer's body.
    let mut unsupported = run(2.6, 0.025, 2.0, Some([0.0, 0.0, 25.0]));
    unsupported.components.push(reducer("component:r2", "node:b", "pipe:dn", None, "mechanics_geometry_only"));
    list.push(("unsupported", unsupported));
    // Standalone: the body alone between the anchor and the tip.
    list.push(("standalone", Model {
        length_unit: "m",
        nodes: vec![("node:a", [0.0, 0.0, 0.0]), ("node:b", [0.4, 0.0, 0.025])],
        members: vec![("pipe:body", "node:a", "node:b")],
        components: vec![reducer("component:r", "node:a", "pipe:body", Some([0.0, 0.0, 25.0]), "mechanics_geometry_only")],
    }));
    // Not a reducer: a valve carrying the reducer slot.
    let mut valve = run(2.6, 0.0, 2.0, None);
    valve.components = vec![json!({"id": "component:v", "kind": "valve", "node": "node:a",
        "geometry": {"rigid_pipe_ref": "pipe:body", "eccentric_offset": {"x": 0.0, "y": 0.0, "z": 1.0, "unit": "mm"}},
        "modifiers": {"stiffness_scaling_user_value": qv(2.0, "1"), "source_reference": "invented"},
        "mechanics_interface": {"solver_consumption": "mechanics_geometry_only"}, "provenance": PROV})];
    list.push(("not a reducer", valve));
    list
}

/// The observable outcome, canonically serialized and hashed.
fn digest(envelope: &Value) -> String {
    let diagnostics: Vec<Value> = envelope["diagnostics"].as_array().unwrap().iter()
        .map(|d| json!([d["id"], d["code"], d["severity"], d["message"], d["affected_refs"]])).collect();
    let rows: Vec<Value> = envelope["results"].as_array().unwrap().iter()
        .map(|r| json!([r["id"], r["kind"], r["value"].as_f64().map(|v| format!("{:016x}", v.to_bits()))])).collect();
    let summary = json!({"mechanics": envelope["status"]["mechanics"], "diagnostics": diagnostics, "rows": rows});
    let bytes = serde_json::to_vec(&summary).unwrap();
    Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// (label, route, digest) at 393a11ee7, before the extraction.
const BASELINE: &[(&str, &str, &str)] = &[
    ("collinear absent", "preview", "5fe45d8f976fd1b6a2d1829d50ee17217d65883d96fb0d1d762e66d5e35d34f4:c0fcddc58b8a1d4ebddf0a3cdada8761be41cabd7f16d6dc774a3689ea3206c8"),
    ("collinear absent", "exact_outside_regions", "765f6015bfd14771072ad3c08f09f43fe557cfd195dbd3ea3b55164562b70db7:8c0d1c5dcbf3aa7eaf9d606976445280050d0cae11d3969fd790645032310f34"),
    ("collinear zero", "preview", "6c3704f6f83cceb1a3dd65f13435f3cab9f2b53e53d14a4f45df417604bd5896:48702304053865044cf5053a6d7e3f607d299a6bba9d577e240ee1d1be8a77fe"),
    ("collinear zero", "exact_outside_regions", "efb4486610a9f7f0d49d04aa13e60809783da50bcddd58525a382644ec537f3d:4d433318391e41e04d4a3df55b19a46d8a38a3e232a4bc2805bb6609d5bf84d5"),
    ("collinear signed zero", "preview", "6c3704f6f83cceb1a3dd65f13435f3cab9f2b53e53d14a4f45df417604bd5896:48702304053865044cf5053a6d7e3f607d299a6bba9d577e240ee1d1be8a77fe"),
    ("collinear signed zero", "exact_outside_regions", "efb4486610a9f7f0d49d04aa13e60809783da50bcddd58525a382644ec537f3d:4d433318391e41e04d4a3df55b19a46d8a38a3e232a4bc2805bb6609d5bf84d5"),
    ("eccentric realized", "preview", "5efc49e8863fa034e6b66562cf73afa35ce907721279b0fb9874e05846256be6:e2e1a52a5408d2274208ae67fd6b52877610d627bce0ec125992b63e4b71f2a1"),
    ("eccentric realized", "exact_outside_regions", "84017bffeb152be4b2b5279a856b6fa391975988733ce238547eb8334c90bddd:54a493a7db3c92218f73a42add1762f735226761b588020ea01cf2c5621b5ad5"),
    ("eccentric absent", "preview", "2b4288945781e2633044acdd1117bd84b62bf51a7d285aa9300cd8f20f2fefff:2380b1ef1822301a107d5187232b3d0031a6a6716e60f77de477689d551a148b"),
    ("eccentric absent", "exact_outside_regions", "e1dbb56622031f0aeebf7d42004879f79e4e32444d72cdbf6028dddfce85f3a7:056bb2bd8a9f4d4c5df4de829a50fbbf1458b3c64f4045caf5f17744e2c8c282"),
    ("eccentric zero mismatch", "preview", "6bd4ae9ed971d595c32619184eb2246ee5c6ded69bb4aac0620d2efd3298bc70:6bd4ae9ed971d595c32619184eb2246ee5c6ded69bb4aac0620d2efd3298bc70"),
    ("eccentric zero mismatch", "exact_outside_regions", "6bd4ae9ed971d595c32619184eb2246ee5c6ded69bb4aac0620d2efd3298bc70:6bd4ae9ed971d595c32619184eb2246ee5c6ded69bb4aac0620d2efd3298bc70"),
    ("eccentric wrong sign", "preview", "6a6de8fd6cd388c222057b3e036c280f56f94b5601f4a7f440dfeaa91e1fa507:6a6de8fd6cd388c222057b3e036c280f56f94b5601f4a7f440dfeaa91e1fa507"),
    ("eccentric wrong sign", "exact_outside_regions", "6a6de8fd6cd388c222057b3e036c280f56f94b5601f4a7f440dfeaa91e1fa507:6a6de8fd6cd388c222057b3e036c280f56f94b5601f4a7f440dfeaa91e1fa507"),
    ("rule-B band", "preview", "a52270f27178460dbe2a5f4557fb26052d898e3ed7cf0c9752d14f87dec7fa05:39b992358446bedf2c417e9d938c92cdd90ea40bf9b9c1c6ae05c6990abb858f"),
    ("rule-B band", "exact_outside_regions", "d2ebd4f63b456fd0431d3ae76caaebb1e32a2bc2dd6847c754404bc660ef6a15:896a84737c197174876c93d07c4127e0b49472cc46b61b3712d331bd095ba120"),
    ("beyond rule-B band", "preview", "8e7c70e76cd8a3ee46e752c946138c021e73e25a341385502c6b2bedf43592fc:8e7c70e76cd8a3ee46e752c946138c021e73e25a341385502c6b2bedf43592fc"),
    ("beyond rule-B band", "exact_outside_regions", "8e7c70e76cd8a3ee46e752c946138c021e73e25a341385502c6b2bedf43592fc:8e7c70e76cd8a3ee46e752c946138c021e73e25a341385502c6b2bedf43592fc"),
    ("below precision", "preview", "17cc3cc9b42d2742582c1a50af9a5d12030a398c73f62f5289702704e0af2eff:17cc3cc9b42d2742582c1a50af9a5d12030a398c73f62f5289702704e0af2eff"),
    ("below precision", "exact_outside_regions", "17cc3cc9b42d2742582c1a50af9a5d12030a398c73f62f5289702704e0af2eff:17cc3cc9b42d2742582c1a50af9a5d12030a398c73f62f5289702704e0af2eff"),
    ("not transverse", "preview", "abad7b5ec926fa0ea277b70324d6fa0baf1abe676df616dc330c6840242db067:abad7b5ec926fa0ea277b70324d6fa0baf1abe676df616dc330c6840242db067"),
    ("not transverse", "exact_outside_regions", "abad7b5ec926fa0ea277b70324d6fa0baf1abe676df616dc330c6840242db067:abad7b5ec926fa0ea277b70324d6fa0baf1abe676df616dc330c6840242db067"),
    ("J3 tie", "preview", "68e08a8fb1b40e6e48d6eef4445d6a318f0ab4c37890f768c88b7bcdc62734e3:3048bbd1de7625934ead5dd5d61937278179a21dbe3f16baa092a027d70809ea"),
    ("J3 tie", "exact_outside_regions", "a74ea5b5655b7ee5e6abaa30d4c930f694bc7eebaea148c0d6981ec87f326e69:a0770544af184b34f861df5217a160d9224f6b9f3d04b1dec03b66fdb57a172a"),
    ("unresolvable short neighbour", "preview", "3288043d503e2c6cb68014762d5daa7a9d30e8c25370fdf10c7af36c0824b2f4:3288043d503e2c6cb68014762d5daa7a9d30e8c25370fdf10c7af36c0824b2f4"),
    ("unresolvable short neighbour", "exact_outside_regions", "3288043d503e2c6cb68014762d5daa7a9d30e8c25370fdf10c7af36c0824b2f4:3288043d503e2c6cb68014762d5daa7a9d30e8c25370fdf10c7af36c0824b2f4"),
    ("marker with offset", "preview", "73bd8b586324371c0be2e99a1b1f9068cd8cd28c56b2abd9fad73144476e1c37:23416d24e1063b433fcdae62898c97b667ea644b9f78f5b53a3a7e04659f00b3"),
    ("marker with offset", "exact_outside_regions", "455ba9e78d3a1fb263c56e9559eba11f24114ba11916af18633d146cb94ede87:4daebf1030b7480ea3f5ada620ec636a7f1be84101b59934fa7b7f6271c10061"),
    ("mapping invalid", "preview", "824550701da6f37e36eba44e55bbdaed84b3094e5ba0f30ca2234368f6b6a20d:824550701da6f37e36eba44e55bbdaed84b3094e5ba0f30ca2234368f6b6a20d"),
    ("mapping invalid", "exact_outside_regions", "824550701da6f37e36eba44e55bbdaed84b3094e5ba0f30ca2234368f6b6a20d:824550701da6f37e36eba44e55bbdaed84b3094e5ba0f30ca2234368f6b6a20d"),
    ("origin at the other end", "preview", "27bf712c942233b3f314101e0112a072388e0832738b7e3b2e8ecc9b5635200f:cdc235a1892a045ece65d33b8a35e2f701ba019f833cfbd4c1be9bee5ef53904"),
    ("origin at the other end", "exact_outside_regions", "62b7128bfac6681c7c8f20b34f9061d7c6be3106f96662bc8b520abf518882f9:1530d5b75a8e2698343754906203dfba8d64837a289ac4df2534b668801b2c4e"),
    ("body reversed", "preview", "c75f180864a06e2b5842ea1d8b26b2c837f95162e56547cdf8135216ddcc4399:4a4538e362dba13519e1bb8c8c6413de2d88dfdbdf46b9b44a998a4155e27c90"),
    ("body reversed", "exact_outside_regions", "6f057c60d3d1d9afdf262fe964b9d4673586db8d11405f985c7151f787743e3f:65dd03be0fe64b25508b8b4e9fced6a19f15309aba7be4637fcc84fbed37990f"),
    ("millimetre model", "preview", "5efc49e8863fa034e6b66562cf73afa35ce907721279b0fb9874e05846256be6:e2e1a52a5408d2274208ae67fd6b52877610d627bce0ec125992b63e4b71f2a1"),
    ("millimetre model", "exact_outside_regions", "84017bffeb152be4b2b5279a856b6fa391975988733ce238547eb8334c90bddd:54a493a7db3c92218f73a42add1762f735226761b588020ea01cf2c5621b5ad5"),
    ("directions disagree", "preview", "0b296ff960fc27693f8b0beb29028c3d724d607f37551000d8135ad94d03de9f:0b296ff960fc27693f8b0beb29028c3d724d607f37551000d8135ad94d03de9f"),
    ("directions disagree", "exact_outside_regions", "0b296ff960fc27693f8b0beb29028c3d724d607f37551000d8135ad94d03de9f:0b296ff960fc27693f8b0beb29028c3d724d607f37551000d8135ad94d03de9f"),
    ("ambiguous", "preview", "9fcce900ea13a32e99c8f1e53d04990df23bfe7ecb285d1c00ff11abf6eb1801:9fcce900ea13a32e99c8f1e53d04990df23bfe7ecb285d1c00ff11abf6eb1801"),
    ("ambiguous", "exact_outside_regions", "9fcce900ea13a32e99c8f1e53d04990df23bfe7ecb285d1c00ff11abf6eb1801:9fcce900ea13a32e99c8f1e53d04990df23bfe7ecb285d1c00ff11abf6eb1801"),
    ("unsupported", "preview", "e338623445f469a022f3c79dd919461b48118b601dac1d728e55068fc00ae725:e338623445f469a022f3c79dd919461b48118b601dac1d728e55068fc00ae725"),
    ("unsupported", "exact_outside_regions", "e338623445f469a022f3c79dd919461b48118b601dac1d728e55068fc00ae725:e338623445f469a022f3c79dd919461b48118b601dac1d728e55068fc00ae725"),
    ("standalone", "preview", "af9f078fd446b040847d00a74fc847695530b4b55a7e8de282209c12ae41726d:af9f078fd446b040847d00a74fc847695530b4b55a7e8de282209c12ae41726d"),
    ("standalone", "exact_outside_regions", "af9f078fd446b040847d00a74fc847695530b4b55a7e8de282209c12ae41726d:af9f078fd446b040847d00a74fc847695530b4b55a7e8de282209c12ae41726d"),
    ("not a reducer", "preview", "33d36abba97a5a421b2b66ade2b47c0627663f4b57bc78257746def35e349f26:33d36abba97a5a421b2b66ade2b47c0627663f4b57bc78257746def35e349f26"),
    ("not a reducer", "exact_outside_regions", "33d36abba97a5a421b2b66ade2b47c0627663f4b57bc78257746def35e349f26:33d36abba97a5a421b2b66ade2b47c0627663f4b57bc78257746def35e349f26"),
];

#[test]
fn every_w15c_pf_control_keeps_its_pre_extraction_outcome() {
    let print = std::env::var("RT_A3_2B_PRINT_DIGESTS").is_ok();
    let mut observed = Vec::new();
    for (label, model) in controls() {
        for route in [Route::Preview, Route::ExactOutsideRegions] {
            let route_name = match route { Route::Preview => "preview", Route::ExactOutsideRegions => "exact_outside_regions" };
            let mut digests = Vec::new();
            for mode in [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny] {
                let envelope = serde_json::to_value(run_linear_static_preview_value_with_mode(request(&model, route), mode)
                    .unwrap_or_else(|e| panic!("{label}: typed boundary rejected the request: {e}"))).unwrap();
                digests.push(digest(&envelope));
                if print {
                    let codes: Vec<&str> = envelope["diagnostics"].as_array().unwrap().iter()
                        .filter(|d| d["code"].as_str().unwrap().starts_with("RIGID_COMPONENT_ECCENTRIC")).map(|d| d["code"].as_str().unwrap()).collect();
                    eprintln!("# {label} {route_name} {mode:?}: {} {codes:?}", envelope["status"]["mechanics"]);
                }
            }
            observed.push((label, route_name, digests.join(":")));
        }
    }
    if print {
        for (label, route, digest) in &observed {
            println!("    (\"{label}\", \"{route}\", \"{digest}\"),");
        }
    }
    assert_eq!(observed.len(), BASELINE.len(), "control count");
    for ((label, route, digest), (b_label, b_route, b_digest)) in observed.iter().zip(BASELINE) {
        assert_eq!((label, route), (b_label, b_route));
        assert_eq!(digest, b_digest, "{label} {route}: the extracted W-15c method changed an observable outcome");
    }
}
