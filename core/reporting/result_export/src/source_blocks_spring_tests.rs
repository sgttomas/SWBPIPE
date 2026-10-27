//! #1686's tests of the source-block support reader's solver mirror
//! (`source_blocks.rs`: `solver_spring_support`, `solver_support_hanger_type`,
//! `solver_constant_effort_support`, `solver_built_support`). They live here,
//! not in `source_blocks.rs`, because that file's test suffix is audited as
//! closed (`product_physics` `retained_memory_law_tests`).
use crate::source_blocks::{solver_constant_effort_support, solver_spring_support, supports, COMPONENTS};
use serde_json::{json, Value};
use std::collections::HashMap;

/// The support-reaction reader's terms for one authored support: six
/// components, the UZ slot carrying a ground spring when `spring` holds and an
/// ideal constraint otherwise, the producer's terms for each decision
/// (`core/product_physics/src/source_receipt/rows.rs`: ideal constraints only
/// for a non-spring support's restraints, a ground spring at the stiffness DOF).
/// The authored support restrains UZ and declares UZ stiffness, so only the
/// spring decision separates the two.
#[cfg(test)]
fn support_law_case(declaration: &Value, spring: bool) -> Result<(), String> {
    let mut authored = json!({"id":"support:s","node":"node:0","restraints":["UZ"],"stiffness":{"dof":"UZ"}});
    for (key, value) in declaration.as_object().unwrap() {
        authored[key] = value.clone();
    }
    let model = json!({"nodes":[{"id":"node:0"}],"supports":[authored]});
    let (mut rows, mut ps, mut components) = (Vec::new(), Vec::new(), Vec::new());
    for slot in 0..6 {
        let result_id = format!("result:{slot}");
        let kind = match (slot, spring) {
            (2, true) => "ground_spring",
            (2, false) => "ideal_constraint",
            _ => "structural_zero",
        };
        rows.push(json!({"kind":"support_reaction_component_v2","entity_ref":"support:s","basis_ref":"basis:b","unit":if slot < 3 { "N" } else { "N*m" },"value":if slot == 2 { 1.0 } else { 0.0 },"metadata":{"component":COMPONENTS[slot],"coordinate_system":"global","location":"node","basis":"recovered_from_assembled_support_law","sign_convention":crate::physics_source::SUPPORT_SIGN}}));
        ps.push(json!({"result_id":result_id,"functional_id":format!("functional:{slot}"),"quantity":"support_action_component","basis":if slot == 2 { "exact" } else { "exact_zero" }}));
        components.push(json!({"component":COMPONENTS[slot],"result_id":result_id,"functional_id":format!("functional:{slot}"),"action_terms":[{"kind":kind,"source_id":"support:s","global_dof":slot}]}));
    }
    let prescribed = if spring { json!([]) } else { json!([2]) };
    let case = json!({"basis_ref":"basis:b","supports":[{"support_id":"support:s","node_id":"node:0","components":components}],"source":{"support_ids":["support:s"],"dof_count":6,"prescribed_dofs":prescribed}});
    let raw = ps
        .iter()
        .zip(&rows)
        .map(|(p, r)| (p["result_id"].as_str().unwrap(), r))
        .collect::<HashMap<_, _>>();
    supports(&case, &raw, &ps, Some(&model))
}

/// Regressions: the reader decides a padded or conflicting hanger declaration
/// as the solver does. Each failed before the reader read declarations as the
/// solver does (at `eb9450806e`, ACTUAL_SUPPORT_LAW on the solver's terms).
#[cfg(test)]
mod support_law_regression_tests {
    use super::*;
    const LAW: &str = "SOURCE_BLOCKS_ACTUAL_SUPPORT_LAW";
    const CONSTANT_EFFORT: &str = "SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT";

    fn decides(declaration: Value, spring: bool) {
        assert_eq!(support_law_case(&declaration, spring), Ok(()), "{declaration}");
        assert_eq!(support_law_case(&declaration, !spring), Err(LAW.into()), "{declaration}");
    }

    /// Whatever terms are published for it, the support is refused by name.
    fn refuses(declaration: Value) {
        for spring in [true, false] {
            assert_eq!(
                support_law_case(&declaration, spring),
                Err(CONSTANT_EFFORT.into()),
                "{declaration}"
            );
        }
    }

    /// Rust's `str::trim` removes U+0085 and ASCII spaces, so the padded
    /// declaration is a hanger; it keeps U+FEFF and U+001C, so a type padded
    /// with either is an unknown type, which decides over the hanger family.
    #[test]
    fn a_padded_declaration_is_read_as_the_solver_trims_it() {
        decides(json!({"family":"guide","hanger":{"hanger_type":"\u{85}spring_hanger\u{85}"}}), true);
        decides(json!({"family":"guide","hanger":{"hanger_type":" variable_spring_hanger "}}), true);
        decides(json!({"family":"\u{85}spring_hanger\u{85}"}), true);
        decides(json!({"family":" variable_spring_hanger "}), true);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"\u{feff}spring_hanger\u{feff}"}}), false);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"\u{1c}spring_hanger\u{1c}"}}), false);
    }

    /// A non-empty explicit type decides over a hanger family. (A
    /// constant-effort type, which decided this way at `a58d893712`, is now
    /// refused: `a_constant_effort_support_is_refused`.)
    #[test]
    fn an_explicit_hanger_type_decides_over_the_family() {
        decides(json!({"family":"spring_hanger","hanger":{"hanger_type":"guide"}}), false);
        decides(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":"strut"}}), false);
    }

    /// The solver builds no support from a constant-effort declaration
    /// (`is_constant_effort_support`, before its spring decision), so a plan
    /// naming one is refused whatever terms it publishes. Each declaration
    /// failed at `a58d893712` and at `eb9450806e`: one of the two term sets was
    /// accepted (the first, Codex r1's reproduction, as a spring).
    #[test]
    fn a_constant_effort_support_is_refused() {
        refuses(json!({"family":"spring","hanger":{"hanger_type":"constant_effort_support"}}));
        // By family, the explicit type absent or empty.
        refuses(json!({"family":"constant_effort_support"}));
        refuses(json!({"family":"constant_effort_support","hanger":{"hanger_type":"  "}}));
        refuses(json!({"family":"constant_effort_support","hanger":{"hanger_type":"constant_effort_support"}}));
        // By explicit type, over any family.
        refuses(json!({"family":"guide","hanger":{"hanger_type":"constant_effort_support"}}));
        refuses(json!({"family":"spring_hanger","hanger":{"hanger_type":"constant_effort_support"}}));
        // Padded with what `str::trim` removes.
        refuses(json!({"family":" constant_effort_support "}));
        refuses(json!({"family":"\u{85}constant_effort_support\u{85}"}));
        refuses(json!({"family":"spring","hanger":{"hanger_type":"\u{85}constant_effort_support\u{85}"}}));
        refuses(json!({"family":"variable_spring_hanger","hanger":{"hanger_type":" constant_effort_support "}}));
    }
}

/// The reader decides every declaration as the solver does: refused where the
/// solver builds no support (constant effort), and otherwise spring or rigid as
/// the solver builds it. The grid is every schema `SupportFamily`, an absent,
/// null, empty, unknown and padded family, crossed with no hanger record, a
/// null record, a record without a type or with a null type, every schema
/// `SpringHanger.hanger_type`, and an empty, unknown, padded and padding-only
/// type. The solver's answers are an explicit table read from its text, which
/// the first test guards: product_physics depends on result_export, so it
/// cannot be a dev-dependency here, and its `support_hanger_type`,
/// `is_variable_spring_hanger` and `is_constant_effort_support` are
/// `pub(crate)` and its decisions are inline in its linear-support assembly,
/// so no public solver function computes them. Padding uses ASCII spaces,
/// U+0085 (which `str::trim` removes) and U+FEFF and U+001C (which it keeps).
#[cfg(test)]
mod solver_spring_rule_tests {
    use super::*;

    /// The solver's text the tables below are read from
    /// (`core/product_physics/src/lib.rs`), compared through `normalized`.
    const SOLVER_SOURCE: [&str; 3] = [
        "pub(crate) fn support_hanger_type(support: &PreviewSupport) -> Option<&str> {
    support
        .hanger
        .as_ref()
        .and_then(|hanger| hanger.hanger_type.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| support.family.as_deref().map(str::trim))
}",
        "pub(crate) fn is_variable_spring_hanger(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some(\"variable_spring_hanger\" | \"spring_hanger\")
    )
}",
        "pub(crate) fn is_constant_effort_support(support: &PreviewSupport) -> bool {
    matches!(
        support_hanger_type(support),
        Some(\"constant_effort_support\")
    )
}",
    ];
    /// The linear-support assembly's exclusion of a constant-effort support,
    /// which precedes its spring decision in the same closure.
    const EXCLUSION: &str = "            if is_constant_effort_support(support) {
                return None;
            }
            let Some(&node) = node_map.get(support.node.as_str()) else {";
    const DECISION: &str = "            if support.family.as_deref() == Some(\"spring\") || is_variable_spring_hanger(support) {
                let stiffness = support_stiffness_input(support).and_then(|input| {";

    /// `text` with comments dropped and whitespace outside string and char
    /// literals collapsed: one space is kept only between two word characters,
    /// two operator characters, or a word character and a quote (either way
    /// round), where dropping it could join two tokens: `'a 'b` is two
    /// lifetimes, `b "x"` an identifier and a string, `"x" y` a string and an
    /// identifier.
    /// Literals are kept verbatim, so a change inside one still counts.
    fn normalized(text: &str) -> String {
        let chars = text.chars().collect::<Vec<_>>();
        let word = |c: char| c.is_alphanumeric() || c == '_';
        let operator = |c: char| "!#$%&*+-./:<=>?@^|~".contains(c);
        let at = |i: usize| chars.get(i).copied();
        let (mut out, mut gap, mut i) = (String::new(), false, 0);
        while i < chars.len() {
            let c = chars[i];
            if c.is_whitespace() {
                (gap, i) = (true, i + 1);
                continue;
            }
            if c == '/' && at(i + 1) == Some('/') {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                gap = true;
                continue;
            }
            if c == '/' && at(i + 1) == Some('*') {
                let mut depth = 0;
                while i < chars.len() {
                    if c_at(&chars, i, "/*") {
                        (depth, i) = (depth + 1, i + 2);
                    } else if c_at(&chars, i, "*/") {
                        (depth, i) = (depth - 1, i + 2);
                        if depth == 0 {
                            break;
                        }
                    } else {
                        i += 1;
                    }
                }
                gap = true;
                continue;
            }
            let last = out.chars().last();
            let joined = last.is_some_and(word) && !gap;
            let quote = |q: char| q == '"' || q == '\'';
            if gap
                && last.is_some_and(|l| {
                    (word(l) && word(c))
                        || (operator(l) && operator(c))
                        || (word(l) && quote(c))
                        || (quote(l) && word(c))
                })
            {
                out.push(' ');
            }
            gap = false;
            // A literal: (its end, exclusive) or none.
            let mut prefix = i;
            if !joined && c == 'b' && matches!(at(i + 1), Some('"' | '\'' | 'r')) {
                prefix += 1;
            }
            let end = if !joined && at(prefix) == Some('r') && matches!(at(prefix + 1), Some('"' | '#')) {
                let hashes = chars[prefix + 1..].iter().take_while(|&&h| h == '#').count();
                let open = prefix + 1 + hashes;
                (at(open) == Some('"')).then(|| {
                    let close = format!("\"{}", "#".repeat(hashes));
                    (open + 1..chars.len())
                        .find(|&j| c_at(&chars, j, &close))
                        .map_or(chars.len(), |j| j + close.len())
                })
            } else if at(prefix) == Some('"') && (prefix == i || !joined) {
                let mut j = prefix + 1;
                while j < chars.len() && chars[j] != '"' {
                    j += if chars[j] == '\\' { 2 } else { 1 };
                }
                Some((j + 1).min(chars.len()))
            } else if at(prefix) == Some('\'') && at(prefix + 1) == Some('\\') {
                let mut j = prefix + 3;
                while j < chars.len() && chars[j] != '\'' {
                    j += 1;
                }
                Some((j + 1).min(chars.len()))
            } else if at(prefix) == Some('\'') && at(prefix + 2) == Some('\'') {
                Some(prefix + 3)
            } else {
                None
            };
            let end = end.unwrap_or(i + 1);
            out.extend(&chars[i..end]);
            i = end;
        }
        out
    }

    fn c_at(chars: &[char], i: usize, text: &str) -> bool {
        text.chars().enumerate().all(|(k, t)| chars.get(i + k) == Some(&t))
    }

    /// The pinned functions are present and the exclusion precedes the
    /// decision in the same closure, all read through `normalized`.
    fn pinned(solver: &str) -> Result<(), String> {
        let solver = normalized(solver);
        for text in SOLVER_SOURCE {
            if !solver.contains(&normalized(text)) {
                return Err(text.into());
            }
        }
        let exclusion = solver.find(&normalized(EXCLUSION)).ok_or(EXCLUSION)?;
        let decision = solver[exclusion..]
            .find(&normalized(DECISION))
            .ok_or(DECISION)?;
        if solver[exclusion..exclusion + decision].contains("fn ") {
            return Err("the exclusion and the decision are in different functions".into());
        }
        Ok(())
    }

    /// How a declaration reads, by the solver's `support_hanger_type`.
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Reading {
        /// `variable_spring_hanger` or `spring_hanger`.
        Hanger,
        /// `constant_effort_support`.
        ConstantEffort,
        Other,
    }
    use Reading::*;

    /// What the solver builds from a declaration.
    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Solver {
        Spring,
        Rigid,
        /// Nothing: `is_constant_effort_support` holds.
        NotBuilt,
    }

    const NEL: &str = "\u{85}";
    const BOM: &str = "\u{feff}";
    const FS: &str = "\u{1c}";

    /// How the hanger record decides: `None` when the trimmed explicit type is
    /// absent or empty and the family decides, else the type's reading.
    fn hanger_cases() -> Vec<(String, Option<Value>, Option<Reading>)> {
        let typed = |t: &str, decides| (format!("hanger_type {t:?}"), Some(json!({"hanger_type":t})), decides);
        let mut cases = vec![
            ("no hanger record".into(), None, None),
            ("null hanger record".into(), Some(Value::Null), None),
            ("no hanger_type".into(), Some(json!({})), None),
            ("null hanger_type".into(), Some(json!({"hanger_type":null})), None),
            typed("variable_spring_hanger", Some(Hanger)),
            typed("spring_hanger", Some(Hanger)),
            typed("constant_effort_support", Some(ConstantEffort)),
            typed("", None),
            typed("strut", Some(Other)),
            typed("spring", Some(Other)),
            typed(" spring_hanger ", Some(Hanger)),
            typed(&format!("{NEL}spring_hanger{NEL}"), Some(Hanger)),
            typed(&format!("{BOM}spring_hanger{BOM}"), Some(Other)),
            typed(&format!("{FS}spring_hanger{FS}"), Some(Other)),
            typed("  ", None),
            typed(NEL, None),
            typed(BOM, Some(Other)),
            typed(FS, Some(Other)),
        ];
        for pad in [" ", NEL, BOM, FS] {
            let trims = pad != BOM && pad != FS;
            cases.push(typed(
                &format!("{pad}constant_effort_support{pad}"),
                Some(if trims { ConstantEffort } else { Other }),
            ));
        }
        cases
    }

    /// Each family: whether it is exactly `spring`, and its trimmed reading
    /// (used when the family decides).
    fn family_cases() -> Vec<(String, Option<Value>, bool, Reading)> {
        let named = |f: &str, spring, reading| (format!("family {f:?}"), Some(json!(f)), spring, reading);
        let mut cases = vec![
            named("anchor", false, Other),
            named("guide", false, Other),
            named("line_stop", false, Other),
            named("vertical_support", false, Other),
            named("spring", true, Other),
            named("variable_spring_hanger", false, Hanger),
            named("spring_hanger", false, Hanger),
            named("constant_effort_support", false, ConstantEffort),
            named("nonlinear", false, Other),
            ("no family".into(), None, false, Other),
            ("null family".into(), Some(Value::Null), false, Other),
            named("", false, Other),
            named("strut", false, Other),
        ];
        for pad in [" ", NEL, BOM, FS] {
            let trims = pad != BOM && pad != FS;
            let reading = |r| if trims { r } else { Other };
            cases.push(named(&format!("{pad}spring_hanger{pad}"), false, reading(Hanger)));
            // `spring` is compared untrimmed: padded, it is no ground spring.
            cases.push(named(&format!("{pad}spring{pad}"), false, Other));
            cases.push(named(
                &format!("{pad}constant_effort_support{pad}"),
                false,
                reading(ConstantEffort),
            ));
        }
        cases
    }

    /// The solver's assembly: a constant-effort reading is excluded first;
    /// otherwise family `spring` or a hanger reading is a spring.
    fn grid() -> Vec<(String, Value, Solver)> {
        let mut grid = Vec::new();
        for (family_name, family, exactly_spring, family_reading) in family_cases() {
            for (hanger_name, hanger, decides) in hanger_cases() {
                let mut declaration = json!({});
                if let Some(family) = &family {
                    declaration["family"] = family.clone();
                }
                if let Some(hanger) = &hanger {
                    declaration["hanger"] = hanger.clone();
                }
                let reading = decides.unwrap_or(family_reading);
                let solver = if reading == ConstantEffort {
                    Solver::NotBuilt
                } else if exactly_spring || reading == Hanger {
                    Solver::Spring
                } else {
                    Solver::Rigid
                };
                grid.push((format!("{family_name} | {hanger_name}"), declaration, solver));
            }
        }
        grid
    }

    #[test]
    fn the_tables_are_read_from_the_solver_as_it_stands() {
        assert_eq!(pinned(include_str!("../../../product_physics/src/lib.rs")), Ok(()));
        let schema: Value = serde_json::from_str(include_str!("../../../../schemas/model.schema.yaml")).unwrap();
        let families = schema["$defs"]["SupportFamily"]["enum"].as_array().unwrap();
        let types = schema["$defs"]["SpringHanger"]["properties"]["hanger_type"]["enum"].as_array().unwrap();
        let named = |cases: Vec<Option<Value>>| cases.into_iter().flatten().collect::<Vec<_>>();
        let family_values = named(family_cases().into_iter().map(|c| c.1).collect());
        let type_values = named(hanger_cases().into_iter().map(|c| c.1.map(|h| h["hanger_type"].clone())).collect());
        assert_eq!(family_values[..families.len()], families[..]);
        for t in types {
            assert!(type_values.contains(t), "{t}");
        }
        assert_eq!(grid().len(), family_cases().len() * hanger_cases().len());
        for solver in [Solver::Spring, Solver::Rigid, Solver::NotBuilt] {
            assert!(grid().iter().any(|g| g.2 == solver), "{solver:?}");
        }
    }

    /// The guard reads through formatting, comments included, and fails on a
    /// change of meaning in the pinned text, inside a string literal included.
    #[test]
    fn the_guard_reads_through_formatting_and_not_through_a_change() {
        let solver = include_str!("../../../product_physics/src/lib.rs");
        let edit = |from: &str, to: &str| {
            assert_eq!(solver.matches(from).count(), 1, "{from}");
            solver.replacen(from, to, 1)
        };
        let trim = "        .map(str::trim)\n        .filter(|value| !value.is_empty())\n";
        let constant = "        Some(\"constant_effort_support\")\n    )\n}";
        let exclusion = "            if is_constant_effort_support(support) {\n                return None;\n            }\n            let Some(&node)";
        let decision = "== Some(\"spring\") || is_variable_spring_hanger(support) {\n                let stiffness";
        for reformatted in [
            edit(trim, "        .map(str::trim)\n\n            .filter( |value| ! value.is_empty() ) // reformatted\n"),
            edit(constant, "        Some(\n            \"constant_effort_support\" /* reformatted */\n        )\n    )\n}"),
            edit(exclusion, "            if is_constant_effort_support(support)\n            {\n                return None;\n            }\n\n            let Some(&node)"),
            edit(decision, "== Some(\"spring\")\n                || is_variable_spring_hanger(support)\n            {\n                let stiffness"),
        ] {
            assert_eq!(pinned(&reformatted), Ok(()));
        }
        for changed in [
            edit(trim, "        .map(str::trim_start)\n        .filter(|value| !value.is_empty())\n"),
            edit(trim, "        .map(str::trim)\n        .filter(|value| value.is_empty())\n"),
            edit(constant, "        Some(\"constant_effort_support \")\n    )\n}"),
            edit(exclusion, "            let Some(&node)"),
            edit(decision, "== Some(\"spring\") && is_variable_spring_hanger(support) {\n                let stiffness"),
            edit(decision, "== Some(\"spring \") || is_variable_spring_hanger(support) {\n                let stiffness"),
        ] {
            assert!(pinned(&changed).is_err());
        }
    }

    #[test]
    fn normalizing_keeps_token_boundaries_at_quotes() {
        // Code review r2 on #1686: each pair is two different token streams.
        for (spaced, joined) in [("m!('a 'b)", "m!('a'b)"), ("m!(b \"x\")", "m!(b\"x\")"), ("m!(\"x\" y)", "m!(\"x\"y)")] {
            assert_ne!(normalized(spaced), normalized(joined), "{spaced}");
        }
        assert_eq!(normalized("m!( 'a   'b )"), normalized("m!('a 'b)"));
    }

    #[test]
    fn the_reader_decides_every_declaration_as_the_solver_does() {
        for (name, declaration, solver) in grid() {
            assert_eq!(
                solver_constant_effort_support(&declaration),
                solver == Solver::NotBuilt,
                "{name}"
            );
            if solver == Solver::NotBuilt {
                for spring in [true, false] {
                    assert_eq!(
                        support_law_case(&declaration, spring),
                        Err("SOURCE_BLOCKS_CONSTANT_EFFORT_SUPPORT".into()),
                        "{name}"
                    );
                }
                continue;
            }
            let spring = solver == Solver::Spring;
            assert_eq!(solver_spring_support(&declaration), spring, "{name}");
            assert_eq!(support_law_case(&declaration, spring), Ok(()), "{name}");
            assert_eq!(
                support_law_case(&declaration, !spring),
                Err("SOURCE_BLOCKS_ACTUAL_SUPPORT_LAW".into()),
                "{name}"
            );
        }
    }
}
