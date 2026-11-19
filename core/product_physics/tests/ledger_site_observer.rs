//! B1 (DEL-05-02 REQ-012): the ledger-site classification test.
//!
//! Built with the product feature `ledger-site-observer`, which turns on the
//! kernel's test-only observer (`load_ledger::site_observer`), this file runs
//! fixtures with the observer armed and compares what it recorded with rosters
//! written here from each fixture's closed form:
//! - every append to an applied-load ledger, with its caller (the site), its
//!   classification by file and enclosing function, the stored source
//!   identity, the index, DOF, kind and exact words, in order;
//! - every sealed ledger (`LoadLedger::finish`), joined to its appends, and
//!   every appended copy (`AssembledForce::with_appended`) and force scaling
//!   (`AssembledForce::force_scaled`);
//! - the right-hand side, free DOFs and row scale exponents each structural
//!   factor completion passed to its solve, against an exact dyadic
//!   expectation computed here, and against what the preparation formed.
//!
//! Without the feature this file compiles to one guard test, which the
//! product's ordinary test command runs: the kernel's own constant
//! `LEDGER_SITE_OBSERVER` is false there.
//!
//! What this file does not show:
//! - production behaviour: the observer is compiled only under its off
//!   feature, so code that differs between the two builds, or a defect only the
//!   ordinary build's code generation exposes, is not observed here;
//! - sites no fixture reaches (the static scan, `tests/test_ledger_site_census.py`,
//!   lists every call by name and argument count; it is not run-time evidence);
//! - calls outside the static scan's named syntax;
//! - the nonlinear base map, ground offsets, iteration inputs and a nonlinear
//!   iteration's internal copies; the prescribed list, ground offsets and
//!   spring base at their product consuming entries;
//! - two solves outside the ordinary structural solve: source recovery's exact
//!   solve (`source_recovery::solve_ordinary`, core/product_physics/src/source_recovery.rs:121),
//!   which consumes its own `ForceContribution` list (pushed at :1028, :1733,
//!   :1738) beside the `folded_force` ledger, and the sparse evidence
//!   observation solve (`observe_sparse_entries`,
//!   core/solver/nonlinear_integration/src/lib.rs:3144, solving at :3157). Their
//!   ledger appends are observed; what they consume is not.
//!
//! A zero term or right-hand-side entry whose closed form is 0 matches either
//! signed zero: the closed form does not fix the sign of a structural zero.
//! Every input is invented; nothing here is library, catalogue or code data.

#[cfg(not(feature = "ledger-site-observer"))]
#[test]
fn ledger_site_observer_is_off_in_the_ordinary_test_build() {
    assert!(!open_pipe_stress_frame_kernel::load_ledger::LEDGER_SITE_OBSERVER);
}

#[cfg(feature = "ledger-site-observer")]
mod observed {
    use open_pipe_stress_frame_kernel::load_ledger::site_observer::{
        self, Armed, Event, ObservedTerm, Preparation, Words,
    };
    use open_pipe_stress_frame_kernel::load_ledger::{Formation, LoadLedger, LEDGER_SITE_OBSERVER};
    use open_pipe_stress_frame_kernel::structural::{
        factor_structural_cholesky, finish_structural, prepare_assembled_sparse_structural,
        prepare_assembled_structural, SparseStiffness, SparseStructuralSystem, StructuralSystem,
    };
    use open_pipe_stress_frame_kernel::ForceScale;
    use open_pipe_stress_product_physics::{run_linear_static_preview_value_with_mode, PreviewSolverMode};
    use serde_json::{json, Value};
    use std::path::{Path, PathBuf};

    const MODES: [PreviewSolverMode; 2] = [PreviewSolverMode::SparseInteractive, PreviewSolverMode::DenseScrutiny];
    const THIS_FILE: &str = "core/product_physics/tests/ledger_site_observer.rs";
    const X0: &str = include_str!("fixtures/exact_pressure_connected_request.json");
    const Y0: &str = include_str!("../../../fixtures/product_preview/pressure/load_reference/pressure.request.json");
    const PROV: &str = "invented_b1_ledger_site_input_not_library_data";

    // ------------------------------------------------------------ classification

    /// Every append site the census knows, by file, enclosing function and
    /// callee, with its class. A site not listed here fails classification.
    const SITES: &[(&str, &str, &str, &str)] = &[
        ("core/product_physics/src/certified_loads.rs", "spring_base_motion", "push_formed", "case ledger"),
        ("core/product_physics/src/certified_loads.rs", "nodal", "push_formed", "case ledger"),
        ("core/product_physics/src/certified_loads.rs", "nodal", "push", "case ledger"),
        ("core/product_physics/src/lib.rs", "push_nodal_loads", "push", "retained replay ledger"),
        ("core/product_physics/src/lib.rs", "add_connector_reference_loads", "push_formed", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_tie_rod_reference_loads", "push_formed", "case ledger"),
        ("core/product_physics/src/lib.rs", "push_exact_pressure_operands", "push_formed", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_uniform_element_loads", "push_formed", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_thermal_equivalent_loads", "push_formed", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_curved_bend_thermal_equivalent_load", "push_formed_product", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_curved_bend_pressure_equivalent_load", "push_formed_product", "case ledger"),
        ("core/product_physics/src/lib.rs", "add_constant_effort_support_loads", "push", "case ledger"),
        ("core/product_physics/src/source_recovery.rs", "prepare_sources", "push", "source recovery fold"),
        ("core/product_physics/src/source_recovery.rs", "close_load_state", "push", "source recovery fold"),
        ("core/solver/nonlinear_integration/src/confirmation.rs", "appended_ledger", "push_formed", "confirmation extra"),
        ("core/solver/nonlinear_integration/src/confirmation.rs", "appended_ledger", "push", "confirmation extra"),
        ("core/solver/nonlinear_integration/src/confirmation.rs", "stage_a", "push", "confirmation influence unit"),
        ("core/solver/frame_kernel/src/restraint_basis.rs", "transform_force", "push", "skew basis (blocked on the exact route)"),
        ("core/solver/frame_kernel/src/restraint_basis.rs", "transform_force", "push_product", "skew basis (blocked on the exact route)"),
    ];
    const CALLEES: [&str; 4] = ["push_formed_product", "push_formed", "push_product", "push"];

    fn repository_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    /// A recorded file as a repository path (`Location::file` is relative to
    /// the package for the product itself and absolute for its path
    /// dependencies).
    fn repository_path(file: &str) -> String {
        let file = file.replace('\\', "/");
        for root in ["core/product_physics/", "core/solver/"] {
            if let Some(at) = file.rfind(root) {
                if root == "core/solver/" || !file[..at].contains("core/solver/") {
                    return file[at..].to_string();
                }
            }
        }
        format!("core/product_physics/{}", file.trim_start_matches("./"))
    }

    /// The text of `line` with `//` comments removed.
    fn code_line(line: &str) -> &str {
        line.find("//").map_or(line, |at| &line[..at])
    }

    fn source_lines(file: &str) -> std::sync::Arc<Vec<String>> {
        static CACHE: std::sync::Mutex<Vec<(String, std::sync::Arc<Vec<String>>)>> = std::sync::Mutex::new(Vec::new());
        let mut cache = CACHE.lock().unwrap_or_else(|p| p.into_inner());
        if let Some((_, lines)) = cache.iter().find(|(f, _)| f == file) {
            return lines.clone();
        }
        let path = repository_root().join(file);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let lines = std::sync::Arc::new(text.lines().map(String::from).collect::<Vec<_>>());
        cache.push((file.to_string(), lines.clone()));
        lines
    }

    /// The first of the four push names called at or after the recorded
    /// column (the location of a method call is its receiver or its name).
    fn callee_at(rest: &str) -> Option<&'static str> {
        (0..rest.len()).filter(|&i| rest.is_char_boundary(i)).find_map(|i| {
            let before = rest[..i].chars().last();
            if before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
                return None;
            }
            CALLEES.iter().copied().find(|c| rest[i..].starts_with(c) && rest[i + c.len()..].trim_start().starts_with('('))
        })
    }

    /// The callee at the recorded location and the nearest enclosing `fn`.
    fn enclosing(file: &str, line: u32, column: u32) -> (String, String) {
        let lines = source_lines(file);
        let at = &lines[line as usize - 1];
        let rest = at.get(column as usize - 1..).unwrap_or("");
        let callee = callee_at(code_line(rest))
            .unwrap_or_else(|| panic!("{file}:{line}:{column}: no push call at the recorded column: {at}"));
        let function = lines[..line as usize]
            .iter()
            .rev()
            .find_map(|l| {
                let code = code_line(l.as_str());
                let at = code.find("fn ")?;
                if at > 0 && (code.as_bytes()[at - 1].is_ascii_alphanumeric() || code.as_bytes()[at - 1] == b'_') {
                    return None;
                }
                let name: String = code[at + 3..].chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
                (!name.is_empty()).then_some(name)
            })
            .unwrap_or_else(|| panic!("{file}:{line}: no enclosing fn"));
        (callee.to_string(), function)
    }

    /// One classified append.
    #[derive(Debug, Clone, PartialEq)]
    struct Site {
        file: String,
        function: String,
        callee: String,
        class: &'static str,
        line: u32,
    }

    fn classify(file: &str, line: u32, column: u32) -> Result<Site, String> {
        let file = repository_path(file);
        let (callee, function) = enclosing(&file, line, column);
        if file == THIS_FILE {
            return Ok(Site { file, function, callee, class: "fixture", line });
        }
        SITES
            .iter()
            .find(|(f, n, c, _)| *f == file && *n == function && *c == callee)
            .map(|(_, _, _, class)| Site { file: file.clone(), function: function.clone(), callee: callee.clone(), class, line })
            .ok_or(format!("unclassified append site {file}:{line} in fn {function} ({callee})"))
    }

    // ------------------------------------------------------------ the join

    /// A sealed ledger with each term's append (or `None` for a copied term of
    /// an appended seal).
    #[derive(Debug, Clone)]
    struct Sealed {
        serial: usize,
        size: usize,
        appended: Option<(usize, usize)>,
        terms: Vec<(Option<Site>, ObservedTerm)>,
    }

    #[derive(Debug, Default)]
    struct Run {
        sealed: Vec<Sealed>,
        scaled: Vec<(i32, Vec<ObservedTerm>)>,
        formed: Vec<(Preparation, usize, Vec<u64>, Vec<i32>)>,
        consumed: Vec<Consumed>,
    }

    #[derive(Debug, Clone)]
    struct Consumed {
        preparation: Preparation,
        free_dofs: Vec<usize>,
        exponents: Vec<i32>,
        rhs: Vec<u64>,
        assembled: bool,
        binary64: bool,
        formation: bool,
        factorization: &'static str,
    }

    /// Joins every append to exactly one seal and every consumed right-hand
    /// side to the latest formed one at the same address. Fails on an
    /// unclassified site, an append that joins no seal or more than one, a
    /// sealed term with no append (other than an appended copy), and a
    /// consumed record with no formed record.
    fn join(events: Vec<Event>) -> Result<Run, String> {
        let mut pending: Vec<(Site, usize, ObservedTerm)> = Vec::new();
        let mut run = Run::default();
        let mut appended: Option<(usize, usize)> = None;
        for event in events {
            match event {
                Event::Append { file, line, column, index, term } => {
                    pending.push((classify(file, line, column)?, index, term));
                }
                Event::Appended { offset, extra } => {
                    if appended.replace((offset, extra)).is_some() {
                        return Err("two appended events before a seal".into());
                    }
                }
                Event::Seal { serial, size, terms } => {
                    let copied = appended.take();
                    let mut joined = Vec::with_capacity(terms.len());
                    for (index, term) in terms.into_iter().enumerate() {
                        let (offset, _) = copied.unwrap_or((0, 0));
                        if copied.is_some() && index < offset {
                            joined.push((None, term));
                            continue;
                        }
                        let own = if copied.is_some() { index - offset } else { index };
                        let matches: Vec<usize> = pending
                            .iter()
                            .enumerate()
                            .filter(|(_, (_, i, t))| *i == own && *t == term)
                            .map(|(k, _)| k)
                            .collect();
                        match matches.as_slice() {
                            [k] => {
                                let (site, _, term) = pending.remove(*k);
                                joined.push((Some(site), term));
                            }
                            [] => return Err(format!("seal {serial}: term {index} {term:?} has no append")),
                            _ => return Err(format!("seal {serial}: term {index} {term:?} joins {} appends", matches.len())),
                        }
                    }
                    if let Some((offset, extra)) = copied {
                        if joined.len() != offset + extra {
                            return Err(format!("appended seal {serial}: {} terms, offset {offset} + extra {extra}", joined.len()));
                        }
                    }
                    run.sealed.push(Sealed { serial, size, appended: copied, terms: joined });
                }
                Event::Scaled { exponent, terms } => run.scaled.push((exponent, terms)),
                Event::Formed { preparation, data, rhs, exponents } => run.formed.push((preparation, data, rhs, exponents)),
                Event::Consumed { data, free_dofs, exponents, rhs, binding, formation, factorization } => {
                    let (preparation, _, formed_rhs, formed_exponents) = run
                        .formed
                        .iter()
                        .rev()
                        .find(|(_, d, _, _)| *d == data)
                        .ok_or("a consumed right-hand side with no formed record")?;
                    if *formed_rhs != rhs || *formed_exponents != exponents {
                        return Err("the consumed right-hand side differs from the formed one".into());
                    }
                    run.consumed.push(Consumed {
                        preparation: *preparation,
                        free_dofs,
                        exponents,
                        rhs,
                        assembled: binding.assembled,
                        binary64: binding.binary64,
                        formation,
                        factorization,
                    });
                }
            }
        }
        if !pending.is_empty() {
            return Err(format!("{} append(s) join no seal: {:?}", pending.len(), pending));
        }
        if appended.is_some() {
            return Err("an appended event with no seal".into());
        }
        Ok(run)
    }

    fn observe<T>(work: impl FnOnce(&Armed) -> T) -> (T, Result<Run, String>) {
        let armed = site_observer::arm();
        let value = work(&armed);
        let events = armed.take();
        (value, events.and_then(join))
    }

    // ------------------------------------------------------------ exact dyadic arithmetic

    /// A nonnegative integer in 64-bit limbs, least significant first.
    #[derive(Clone, Default, Debug, PartialEq, Eq)]
    struct Big(Vec<u64>);

    impl Big {
        fn shifted(value: u128, shift: usize) -> Big {
            let mut limbs = vec![0u64; shift / 64 + 3];
            let (word, bit) = (shift / 64, shift % 64);
            let wide = [value as u64, (value >> 64) as u64];
            for (k, &w) in wide.iter().enumerate() {
                if bit == 0 {
                    limbs[word + k] |= w;
                } else {
                    limbs[word + k] |= w << bit;
                    limbs[word + k + 1] |= w >> (64 - bit);
                }
            }
            Big(limbs).trimmed()
        }
        fn trimmed(mut self) -> Big {
            while self.0.last() == Some(&0) {
                self.0.pop();
            }
            self
        }
        fn add(&self, other: &Big) -> Big {
            let n = self.0.len().max(other.0.len()) + 1;
            let mut out = vec![0u64; n];
            let mut carry = 0u128;
            for (k, slot) in out.iter_mut().enumerate() {
                let s = *self.0.get(k).unwrap_or(&0) as u128 + *other.0.get(k).unwrap_or(&0) as u128 + carry;
                *slot = s as u64;
                carry = s >> 64;
            }
            Big(out).trimmed()
        }
        fn cmp(&self, other: &Big) -> std::cmp::Ordering {
            self.0.len().cmp(&other.0.len()).then_with(|| self.0.iter().rev().cmp(other.0.iter().rev()))
        }
        /// self - other, for self >= other.
        fn sub(&self, other: &Big) -> Big {
            let mut out = self.0.clone();
            let mut borrow = 0i128;
            for (k, slot) in out.iter_mut().enumerate() {
                let d = *slot as i128 - *other.0.get(k).unwrap_or(&0) as i128 - borrow;
                borrow = if d < 0 { 1 } else { 0 };
                *slot = (d + (borrow << 64)) as u64;
            }
            Big(out).trimmed()
        }
        fn bits(&self) -> usize {
            self.0.last().map_or(0, |top| 64 * (self.0.len() - 1) + 64 - top.leading_zeros() as usize)
        }
        fn bit(&self, k: usize) -> bool {
            self.0.get(k / 64).is_some_and(|w| (w >> (k % 64)) & 1 == 1)
        }
        fn any_below(&self, k: usize) -> bool {
            (0..k).any(|b| self.bit(b))
        }
        fn field(&self, from: usize, count: usize) -> u64 {
            (0..count).fold(0u64, |acc, b| acc | ((self.bit(from + b) as u64) << b))
        }
    }

    /// Fixed-point offset: every binary64 value and every product of two is an
    /// integer multiple of 2^-OFFSET.
    const OFFSET: i32 = 2200;

    /// x = m·2^e with an integer m.
    fn split(x: f64) -> (bool, u64, i32) {
        let bits = x.to_bits();
        let negative = bits >> 63 == 1;
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & ((1u64 << 52) - 1);
        assert!(exponent != 0x7ff, "finite inputs only");
        if exponent == 0 {
            (negative, fraction, -1074)
        } else {
            (negative, fraction | (1u64 << 52), exponent - 1075)
        }
    }

    /// The exact signed sum of terms (a value, or the product of two values),
    /// as positive and negative parts.
    #[derive(Default)]
    struct Dyadic {
        positive: Big,
        negative: Big,
    }

    impl Dyadic {
        fn add_term(&mut self, x: f64) {
            let (negative, m, e) = split(x);
            self.add_raw(negative, m as u128, e);
        }
        fn add_product(&mut self, a: f64, b: f64) {
            let (na, ma, ea) = split(a);
            let (nb, mb, eb) = split(b);
            self.add_raw(na != nb, ma as u128 * mb as u128, ea + eb);
        }
        fn add_words(&mut self, words: Words, negate: bool) {
            let sign = if negate { -1.0 } else { 1.0 };
            match words {
                Words::Term(x) => self.add_term(sign * f64::from_bits(x)),
                Words::Product(a, b) => self.add_product(sign * f64::from_bits(a), f64::from_bits(b)),
            }
        }
        fn add_raw(&mut self, negative: bool, m: u128, e: i32) {
            if m == 0 {
                return;
            }
            let shift = usize::try_from(e + OFFSET).expect("within the fixed-point range");
            let value = Big::shifted(m, shift);
            if negative {
                self.negative = self.negative.add(&value);
            } else {
                self.positive = self.positive.add(&value);
            }
        }
        /// RN(2^scale · sum), rounded once, to nearest with ties to even; a
        /// zero sum is +0.0.
        fn round_scaled(&self, scale: i32) -> f64 {
            let (negative, magnitude) = match self.positive.cmp(&self.negative) {
                std::cmp::Ordering::Less => (true, self.negative.sub(&self.positive)),
                _ => (false, self.positive.sub(&self.negative)),
            };
            let bits = magnitude.bits();
            if bits == 0 {
                return 0.0;
            }
            // value = magnitude · 2^(scale - OFFSET); its leading bit has exponent top.
            let low = scale - OFFSET;
            let top = low + bits as i32 - 1;
            assert!(top <= 1023, "overflow is outside these fixtures");
            let quantum = (top - 52).max(-1074);
            let drop = (quantum - low).max(0) as usize;
            let mut mantissa = magnitude.field(drop, bits - drop) as u128;
            if drop > 0 {
                let half = magnitude.bit(drop - 1);
                let sticky = magnitude.any_below(drop - 1);
                if half && (sticky || mantissa & 1 == 1) {
                    mantissa += 1;
                }
            }
            let mut quantum = if drop > 0 { quantum } else { low };
            if mantissa == 1u128 << 53 {
                mantissa >>= 1;
                quantum += 1;
            }
            let value = mantissa as f64 * pow2_wide(quantum);
            if negative { -value } else { value }
        }
    }

    /// 2^e as an exact product of normal powers of two (e in [-1074, 1023]).
    fn pow2_wide(e: i32) -> f64 {
        if e >= -1022 {
            pow2(e)
        } else {
            pow2(e + 52) * pow2(-52)
        }
    }

    fn pow2(e: i32) -> f64 {
        assert!((-1022..=1023).contains(&e));
        f64::from_bits(((e + 1023) as u64) << 52)
    }

    /// The row scale exponent's stated rule: -floor(log2 d) div_euclid 2, with
    /// floor(log2 d) read from the diagonal (a normal value).
    fn row_exponent_exact(diagonal: f64) -> i32 {
        let (_, _, e) = split(diagonal);
        assert!(diagonal.is_normal() && diagonal > 0.0);
        -(e + 52).div_euclid(2)
    }

    /// The same rule on a closed-form diagonal evaluated in binary64, which
    /// must lie more than 1e-9 (relative) from a power of two, so that the
    /// product's own rounding of it cannot change floor(log2 d).
    fn row_exponent(diagonal: f64) -> i32 {
        let below = pow2(split(diagonal).2 + 52);
        assert!(diagonal / below - 1.0 > 1e-9 && 2.0 - diagonal / below > 2e-9, "diagonal {diagonal} is too close to a power of two");
        row_exponent_exact(diagonal)
    }

    /// Words equal; a closed-form zero matches either signed zero.
    fn same_words(observed: Words, expected: Words) -> bool {
        match (observed, expected) {
            (Words::Term(o), Words::Term(e)) if f64::from_bits(e) == 0.0 => f64::from_bits(o) == 0.0,
            _ => observed == expected,
        }
    }

    fn same_bits(observed: u64, expected: f64) -> bool {
        if expected == 0.0 {
            f64::from_bits(observed) == 0.0
        } else {
            observed == expected.to_bits()
        }
    }

    // ------------------------------------------------------------ rosters

    /// A roster term: the expected site (file and function), stored source
    /// identity, DOF and words.
    #[derive(Debug, Clone)]
    struct Expected {
        file: &'static str,
        function: &'static str,
        source: String,
        dof: usize,
        words: Words,
    }

    fn term(value: f64) -> Words {
        Words::Term(value.to_bits())
    }

    /// Compares a seal's joined terms with a roster, exactly and in order.
    fn check_seal(sealed: &Sealed, roster: &[Expected]) -> Result<(), String> {
        if sealed.terms.len() != roster.len() {
            return Err(format!("seal {}: {} terms, roster {}", sealed.serial, sealed.terms.len(), roster.len()));
        }
        for (k, ((site, observed), expected)) in sealed.terms.iter().zip(roster).enumerate() {
            let site = site.as_ref().ok_or(format!("seal {}: term {k} is a copied term", sealed.serial))?;
            if site.file != expected.file || site.function != expected.function {
                return Err(format!("seal {}: term {k} site {}::{} expected {}::{}", sealed.serial, site.file, site.function, expected.file, expected.function));
            }
            if observed.source != expected.source {
                return Err(format!("seal {}: term {k} source {:?} expected {:?}", sealed.serial, observed.source, expected.source));
            }
            if observed.dof != expected.dof || !same_words(observed.words, expected.words) {
                return Err(format!("seal {}: term {k} ({}, {:?}) expected ({}, {:?})", sealed.serial, observed.dof, observed.words, expected.dof, expected.words));
            }
        }
        Ok(())
    }

    /// The exact consumed right-hand side for free DOFs with closed-form
    /// diagonals, from roster terms and prescribed couplings (k_ij, g_j).
    fn expected_rhs(roster: &[Expected], free: &[usize], exponents: Vec<i32>, couplings: &[(usize, f64, f64)]) -> (Vec<i32>, Vec<f64>) {
        let rhs = free
            .iter()
            .zip(&exponents)
            .map(|(&dof, &e)| {
                let mut sum = Dyadic::default();
                for t in roster.iter().filter(|t| t.dof == dof) {
                    sum.add_words(t.words, false);
                }
                for &(row, k, g) in couplings {
                    if row == dof {
                        sum.add_product(-k, g);
                    }
                }
                sum.round_scaled(e)
            })
            .collect();
        (exponents, rhs)
    }

    fn check_consumed(consumed: &Consumed, free: &[usize], exponents: &[i32], rhs: &[f64]) -> Result<(), String> {
        if consumed.free_dofs != free {
            return Err(format!("free DOFs {:?} expected {free:?}", consumed.free_dofs));
        }
        if consumed.exponents != exponents {
            return Err(format!("row scale exponents {:?} expected {exponents:?}", consumed.exponents));
        }
        for (r, (&o, &e)) in consumed.rhs.iter().zip(rhs).enumerate() {
            if !same_bits(o, e) {
                return Err(format!("right-hand side entry {r}: {:e} ({o:#x}) expected {e:e} ({:#x})", f64::from_bits(o), e.to_bits()));
            }
        }
        if consumed.rhs.len() != rhs.len() {
            return Err("right-hand side length".into());
        }
        Ok(())
    }

    // ------------------------------------------------------------ the feature itself

    #[test]
    fn the_kernel_feature_is_on_in_this_build() {
        assert!(LEDGER_SITE_OBSERVER);
    }

    /// Every test here that makes an event arms first: an event on another
    /// thread while a scope is armed fails that scope.
    #[test]
    fn an_event_on_another_thread_fails_the_armed_scope() {
        let armed = site_observer::arm();
        assert_eq!(armed.take().unwrap(), Vec::new(), "a fresh scope starts empty");
        std::thread::scope(|scope| {
            scope.spawn(|| {
                let mut other = LoadLedger::new();
                other.push("fixture:other-thread", 0, 1.0);
            });
        });
        assert!(armed.take().unwrap_err().contains("another thread"));
    }

    // ------------------------------------------------------------ dispatch

    /// All four append functions, called directly, are attributed to their
    /// caller here (the wrappers are `track_caller`), with the stored source
    /// identity after `into()`.
    #[test]
    fn every_append_function_is_attributed_to_its_caller() {
        let (lines, run) = observe(|_| {
            let mut ledger = LoadLedger::new();
            let a = line!(); ledger.push(String::from("fixture:push"), 3, 1.5);
            let b = line!(); ledger.push_product("fixture:product", 1, 3.0, 0.25);
            let c = line!(); ledger.push_formed("fixture:formed", 2, 0.75, Formation::CannotBound, 0.0, false);
            let d = line!(); ledger.push_formed_product("fixture:formed-product", 0, -2.0, 0.5, Formation::CannotBound, 0.0, true);
            let _ = ledger.finish(4).unwrap();
            [a, b, c, d]
        });
        let run = run.unwrap();
        assert_eq!(run.sealed.len(), 1);
        let sealed = &run.sealed[0];
        let expected = [
            ("push", "fixture:push", 3, term(1.5)),
            ("push_product", "fixture:product", 1, Words::Product(3.0f64.to_bits(), 0.25f64.to_bits())),
            ("push_formed", "fixture:formed", 2, term(0.75)),
            ("push_formed_product", "fixture:formed-product", 0, Words::Product((-2.0f64).to_bits(), 0.5f64.to_bits())),
        ];
        assert_eq!(sealed.terms.len(), 4);
        for (k, ((site, observed), (callee, source, dof, words))) in sealed.terms.iter().zip(expected).enumerate() {
            let site = site.as_ref().unwrap();
            assert_eq!((site.file.as_str(), site.callee.as_str(), site.line), (THIS_FILE, callee, lines[k]), "term {k}");
            assert_eq!(site.function, "every_append_function_is_attributed_to_its_caller");
            assert_eq!((observed.source.as_str(), observed.dof, observed.words), (source, dof, words), "term {k}");
        }
        assert_eq!(sealed.size, 4);
    }

    // ------------------------------------------------------------ the kernel controls

    /// Free DOF 0 holds Term(1) and Term(2^-54); prescribed DOF 1 at g = 1
    /// couples with K01 = 1, contributing -1. The exact reduction is 2^-54;
    /// rounding the ledger net first gives fl(1 + 2^-54) = 1 and 0.
    fn rounding_control(armed: &Armed, dense: bool, flip: bool) -> Result<(), String> {
        let tiny = pow2(-54);
        let mut ledger = LoadLedger::new();
        ledger.push("fixture:one", 0, 1.0);
        ledger.push("fixture:tiny", 0, tiny);
        let force = ledger.finish(2).map_err(|e| e.to_string())?;
        let k = vec![vec![4.0, 1.0], vec![1.0, 4.0]];
        let (free, prescribed) = ([0usize], [(1usize, 1.0f64)]);
        if flip {
            armed.flip_next_formed(0);
        }
        if dense {
            let system = StructuralSystem::assembled(&k, &force, &free, &prescribed, None, None);
            let prepared = prepare_assembled_structural(&system).map_err(|e| format!("{e:?}"))?;
            let factor = factor_structural_cholesky(&prepared).map_err(|e| format!("{e:?}"))?;
            let _ = finish_structural(&factor);
        } else {
            let sparse = SparseStiffness::from_dense(&k).map_err(|e| format!("{e:?}"))?;
            let system = SparseStructuralSystem::assembled(&sparse, &force, &free, &prescribed, None, None);
            let prepared = prepare_assembled_sparse_structural(&system).map_err(|e| format!("{e:?}"))?;
            let _ = open_pipe_stress_sparse_direct::structural::solve_sparse_prepared(prepared);
        }
        Ok(())
    }

    fn rounding_roster() -> Vec<Expected> {
        let f = |source: &str, value| Expected { file: THIS_FILE, function: "rounding_control", source: source.into(), dof: 0, words: term(value) };
        vec![f("fixture:one", 1.0), f("fixture:tiny", pow2(-54))]
    }

    fn check_rounding(run: &Run, dense: bool) -> Result<(), String> {
        check_seal(&run.sealed[0], &rounding_roster())?;
        let [consumed] = run.consumed.as_slice() else { return Err(format!("{} consumed records", run.consumed.len())) };
        let preparation = if dense { Preparation::Dense } else { Preparation::Sparse };
        if consumed.preparation != preparation || !consumed.assembled || consumed.binary64 || consumed.formation {
            return Err(format!("path {consumed:?}"));
        }
        let (exponents, rhs) = expected_rhs(&rounding_roster(), &[0], vec![row_exponent_exact(4.0)], &[(0, 1.0, 1.0)]);
        // 2^-1 · (1 + 2^-54 - 1) = 2^-55; rounding the net first would give 0.
        assert_eq!((exponents.as_slice(), rhs[0]), (&[-1][..], pow2(-55)));
        check_consumed(consumed, &[0], &exponents, &rhs)
    }

    #[test]
    fn the_consumed_right_hand_side_is_the_exact_reduction_rounded_once() {
        for dense in [true, false] {
            let (formed, run) = observe(|armed| rounding_control(armed, dense, false));
            formed.unwrap();
            check_rounding(&run.unwrap(), dense).unwrap_or_else(|e| panic!("dense={dense}: {e}"));
        }
    }

    /// The armed hook flips the low bit of the formed entry; the consumed
    /// record carries the flipped word, so the roster comparison fails.
    #[test]
    fn a_right_hand_side_changed_between_formation_and_consumption_fails_the_roster() {
        for dense in [true, false] {
            let (formed, run) = observe(|armed| rounding_control(armed, dense, true));
            formed.unwrap();
            let run = run.unwrap();
            let error = check_rounding(&run, dense).unwrap_err();
            assert!(error.contains("right-hand side entry 0"), "dense={dense}: {error}");
            assert_eq!(run.consumed[0].rhs[0], pow2(-55).to_bits() ^ 1);
        }
    }

    /// Two normal factors with exact product 2^-1075: the net is +0 unscaled
    /// (a tie to even) and 2^-1074 at b = 1 through `force_scaled`, whose
    /// scaled event carries the exact scaling of the term.
    #[test]
    fn force_scaling_keeps_a_product_that_underflows_unscaled() {
        let (scaled, run) = observe(|_| {
            let mut ledger = LoadLedger::new();
            ledger.push_product("fixture:underflow", 0, pow2(-537), pow2(-538));
            let force = ledger.finish(1).unwrap();
            assert_eq!(force.values()[0].to_bits(), 0u64);
            assert_eq!(force.evidence().underflowed_dofs, vec![0]);
            force.force_scaled(ForceScale::new(1).unwrap()).unwrap().values()[0]
        });
        assert_eq!(scaled.to_bits(), 1u64);
        let run = run.unwrap();
        let [(exponent, terms)] = run.scaled.as_slice() else { panic!("{:?}", run.scaled) };
        assert_eq!(*exponent, 1);
        let [t] = terms.as_slice() else { panic!() };
        assert_eq!((t.source.as_str(), t.dof), ("fixture:underflow", 0));
        let mut observed = Dyadic::default();
        observed.add_words(t.words, false);
        let mut expected = Dyadic::default();
        expected.add_product(pow2(-537), pow2(-538));
        assert_eq!(observed.round_scaled(1100), expected.round_scaled(1101), "the scaled term is the exact scaling");
        assert_eq!(observed.round_scaled(0).to_bits(), 1u64);
    }

    /// `with_appended`: the appended seal holds the base terms, then the extra
    /// ledger's, which joined their own appends.
    #[test]
    fn an_appended_seal_holds_the_base_terms_then_the_extra_terms() {
        let (_, run) = observe(|_| {
            let mut base = LoadLedger::new();
            base.push("fixture:base-0", 0, 2.0);
            base.push("fixture:base-1", 1, -3.0);
            base.push("fixture:base-2", 0, 0.5);
            let force = base.finish(2).unwrap();
            let mut extra = LoadLedger::new();
            extra.push("fixture:extra-0", 1, 4.0);
            let _ = force.with_appended(extra).unwrap();
        });
        let run = run.unwrap();
        let [base, appended] = run.sealed.as_slice() else { panic!("{} seals", run.sealed.len()) };
        let roster: Vec<Expected> = [("fixture:base-0", 0, 2.0), ("fixture:base-1", 1, -3.0), ("fixture:base-2", 0, 0.5)]
            .iter()
            .map(|&(s, dof, v)| Expected { file: THIS_FILE, function: "an_appended_seal_holds_the_base_terms_then_the_extra_terms", source: s.into(), dof, words: term(v) })
            .collect();
        check_seal(base, &roster).unwrap();
        assert_eq!(appended.appended, Some((3, 1)));
        let copied: Vec<(String, usize, Words)> = appended.terms[..3].iter().map(|(_, t)| (t.source.clone(), t.dof, t.words)).collect();
        let expected: Vec<(String, usize, Words)> = roster.iter().map(|e| (e.source.clone(), e.dof, e.words)).collect();
        assert_eq!(copied, expected, "the copied base terms, in order");
        let (site, extra) = &appended.terms[3];
        assert_eq!((extra.source.as_str(), extra.dof, extra.words), ("fixture:extra-0", 1, term(4.0)));
        assert_eq!(site.as_ref().unwrap().class, "fixture");
    }

    // ------------------------------------------------------------ product fixtures

    /// X0's one six-component case on its cantilever: node:fixture-root (DOFs
    /// 0-5) anchored, node:fixture-tip (DOFs 6-11) free; pipe along +x, L = 1 m,
    /// OD 0.12 m, wall 0.01 m, E = 2e11 Pa, nu = 0.3, Euler-Bernoulli.
    fn cantilever(exact: bool, uniform: bool) -> Value {
        let mut request: Value = serde_json::from_str(X0).unwrap();
        let model = &mut request["model"];
        let cases = model["load_cases"].as_array_mut().unwrap();
        cases.retain(|case| case["id"] == "case:six-component-load");
        if uniform {
            cases[0]["primitive_loads"].as_array_mut().unwrap().push(json!({
                "id": "load:b1-uniform", "category": "distributed_force",
                "target": {"type": "element", "pipe": "pipe:fixture-span"},
                "direction": "global_y", "magnitude": {"value": 12.0, "unit": "N/m"},
                "dimension": "force_per_length", "provenance": PROV}));
        }
        let object = model.as_object_mut().unwrap();
        object.remove("model_settings");
        if !exact {
            object.remove("pressure_contract");
        }
        request
    }

    const NODAL: [(&str, usize, f64); 6] = [
        ("load:fixture-global_x", 6, 1000.0),
        ("load:fixture-global_y", 7, 2000.0),
        ("load:fixture-global_z", 8, -3000.0),
        ("load:fixture-rotation_x", 9, 400.0),
        ("load:fixture-rotation_y", 10, -500.0),
        ("load:fixture-rotation_z", 11, 600.0),
    ];

    /// The case ledger, in `case_force_ledger`'s producer order: the authored
    /// nodal loads (`certified_loads::nodal`, plain terms), then, with the
    /// uniform load w = 12 N/m along global y, its consistent equivalent at the
    /// element's 12 DOFs: F_y = wL/2 = 6 at both ends, M_z = +wL^2/12 = 1 at the
    /// root and -1 at the tip, every other component 0.
    fn cantilever_roster(uniform: bool) -> Vec<Expected> {
        let mut roster: Vec<Expected> = NODAL
            .iter()
            .map(|&(id, dof, value)| Expected { file: "core/product_physics/src/certified_loads.rs", function: "nodal", source: id.into(), dof, words: term(value) })
            .collect();
        if uniform {
            let (w, l) = (12.0f64, 1.0f64);
            let mut equivalent = [0.0f64; 12];
            equivalent[1] = w * l / 2.0;
            equivalent[5] = w * l * l / 12.0;
            equivalent[7] = w * l / 2.0;
            equivalent[11] = -w * l * l / 12.0;
            assert_eq!(equivalent[5], 1.0);
            for (dof, value) in equivalent.into_iter().enumerate() {
                roster.push(Expected { file: "core/product_physics/src/lib.rs", function: "add_uniform_element_loads", source: "load:b1-uniform".into(), dof, words: term(value) });
            }
        }
        roster
    }

    /// The free tip's diagonal stiffness: EA/L, 12EI/L^3 (twice), GJ/L, 4EI/L
    /// (twice), with A = pi(D^2 - d^2)/4, I = pi(D^4 - d^4)/64, J = 2I,
    /// G = E/(2(1 + nu)).
    fn cantilever_diagonals() -> [f64; 6] {
        let (d_out, d_in, l, e, nu) = (0.12f64, 0.10f64, 1.0f64, 2.0e11f64, 0.3f64);
        let pi = std::f64::consts::PI;
        let a = pi * (d_out * d_out - d_in * d_in) / 4.0;
        let i = pi * (d_out.powi(4) - d_in.powi(4)) / 64.0;
        let g = e / (2.0 * (1.0 + nu));
        [e * a / l, 12.0 * e * i / l.powi(3), 12.0 * e * i / l.powi(3), g * 2.0 * i / l, 4.0 * e * i / l, 4.0 * e * i / l]
    }

    fn run_product(request: &Value, mode: PreviewSolverMode) -> (Value, Result<Run, String>) {
        let (envelope, run) = observe(|_| run_linear_static_preview_value_with_mode(request.clone(), mode));
        (serde_json::to_value(envelope.expect("the request parses")).unwrap(), run)
    }

    fn codes(envelope: &Value) -> Vec<String> {
        envelope["diagnostics"].as_array().into_iter().flatten().filter_map(|d| d["code"].as_str().map(String::from)).collect()
    }

    fn blocking(envelope: &Value) -> Vec<String> {
        envelope["diagnostics"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|d| d["severity"] == "blocking")
            .filter_map(|d| d["code"].as_str().map(String::from))
            .collect()
    }

    /// One linear product case: one sealed case ledger, equal to its roster,
    /// and every consumed right-hand side (each from the preparation the mode
    /// selects, formation-checked, assembled binding) equal to the exact
    /// expectation.
    fn check_cantilever(run: &Run, mode: PreviewSolverMode, uniform: bool) -> Result<(), String> {
        let [sealed] = run.sealed.as_slice() else { return Err(format!("{} seals", run.sealed.len())) };
        if sealed.size != 12 || sealed.appended.is_some() {
            return Err(format!("seal shape {} {:?}", sealed.size, sealed.appended));
        }
        let roster = cantilever_roster(uniform);
        check_seal(sealed, &roster)?;
        if !run.scaled.is_empty() {
            return Err("an unexpected force scaling".into());
        }
        let free: Vec<usize> = (6..12).collect();
        let exponents = cantilever_diagonals().iter().map(|&d| row_exponent(d)).collect();
        let (exponents, rhs) = expected_rhs(&roster, &free, exponents, &[]);
        if run.consumed.is_empty() {
            return Err("no consumed right-hand side".into());
        }
        let (preparation, factorization) = match mode {
            PreviewSolverMode::DenseScrutiny => (Preparation::Dense, "positive dense Cholesky"),
            PreviewSolverMode::SparseInteractive => (Preparation::Sparse, "positive skyline LDL"),
        };
        for consumed in &run.consumed {
            if consumed.preparation != preparation || consumed.factorization != factorization || !consumed.assembled || consumed.binary64 || !consumed.formation {
                return Err(format!("path {consumed:?}"));
            }
            check_consumed(consumed, &free, &exponents, &rhs)?;
        }
        Ok(())
    }

    #[test]
    fn cantilever_nodal_and_uniform_loads_match_their_rosters_on_both_routes_in_both_modes() {
        for exact in [true, false] {
            for uniform in [false, true] {
                for mode in MODES {
                    let (envelope, run) = run_product(&cantilever(exact, uniform), mode);
                    assert!(blocking(&envelope).is_empty(), "exact={exact} uniform={uniform} {mode:?}: {:?}", codes(&envelope));
                    check_cantilever(&run.unwrap(), mode, uniform)
                        .unwrap_or_else(|e| panic!("exact={exact} uniform={uniform} {mode:?}: {e}"));
                }
            }
        }
    }

    // ------------------------------------------------------------ skew and pending paths

    /// An exact skew support is refused before any append.
    #[test]
    fn an_exact_skew_support_is_refused_before_any_append() {
        let mut request = cantilever(true, false);
        request["model"]["supports"].as_array_mut().unwrap().push(json!({
            "id": "support:b1-skew", "node": "node:fixture-tip", "family": "line_stop", "restraints": ["UY"],
            "direction": {"x": 1.0, "y": 1.0, "z": 0.0}, "provenance": PROV}));
        for mode in MODES {
            let (envelope, run) = run_product(&request, mode);
            assert!(blocking(&envelope).iter().any(|c| c == "SUPPORT_DIRECTION_SKEW_EXACT_UNSUPPORTED"), "{mode:?}: {:?}", codes(&envelope));
            let run = run.unwrap();
            assert!(run.sealed.is_empty() && run.consumed.is_empty(), "{mode:?}: {run:?}");
        }
    }

    /// Each pending path stays refused by its code; the test fails when one
    /// becomes reachable while it is still pending.
    #[test]
    fn pending_paths_stay_refused() {
        // The held-status subtrahend: before B2, the blanket exact-route code (a two-case probe).
        let mut held: Value = serde_json::from_str(X0).unwrap();
        held["model"]["combinations"] = json!([{"id": "combination:b1-difference", "basis": "result_state_subtraction",
            "minuend_id": "case:six-component-load", "subtrahend_id": "case:closed-pressure",
            "difference_basis": "operating_minus_sustained", "provenance": PROV}]);
        // Hydrotest: a test condition.
        let mut hydrotest = cantilever(true, false);
        hydrotest["model"]["load_cases"][0]["test_condition"] = json!({"kind": "hydrostatic", "held_supports": "none"});
        // Generation on a load and reference state document.
        let mut generated: Value = serde_json::from_str(Y0).unwrap();
        for case in generated["model"]["load_cases"].as_array_mut().unwrap() {
            if case["id"] == "case:cold-pressure" {
                case["equivalent_static"] = json!({"provenance": PROV});
            }
        }
        for (label, request, code) in [
            ("held-status subtrahend", &held, "LOAD_COMBINATION_DIFFERENCE_BASIS_EXACT_UNSUPPORTED"),
            ("hydrotest", &hydrotest, "LOAD_CASE_TEST_CONDITION_UNSUPPORTED"),
            ("generation on a load and reference state document", &generated, "EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED"),
        ] {
            for mode in MODES {
                let (envelope, run) = run_product(request, mode);
                assert!(blocking(&envelope).iter().any(|c| c == code), "{label} {mode:?}: {:?}", codes(&envelope));
                run.unwrap_or_else(|e| panic!("{label} {mode:?}: {e}"));
            }
        }
        // Hydrotest refuses before any solve.
        for mode in MODES {
            let (_, run) = run_product(&hydrotest, mode);
            assert!(run.unwrap().consumed.is_empty(), "hydrotest {mode:?}");
        }
    }
}
