//! K4 tests of `retained/wide_sum.rs` (the brief's B) and the N5 streams of
//! K3's arithmetic at K4's working precisions (the brief's D).
use super::super::wide::multi::{SupportedWidth, WideContext};
use super::super::wide::Wide;
use super::*;
use std::time::Instant;

#[allow(dead_code)]
#[path = "support.rs"]
mod support;
use support::*;

const TARGETED: &str = include_str!("sum_targeted.txt");
const DIFF_MANIFEST: &str = include_str!("sum_differential.txt");
const DIFF_SAMPLE: &str = include_str!("sum_differential_sample.txt");
const STREAMS: &str = include_str!("streams.txt");
const STREAMS_SAMPLE: &str = include_str!("streams_sample.txt");
const SHA256SUMS: &str = include_str!("SHA256SUMS");

#[test]
fn committed_vectors_match_their_recorded_sha256() {
    let files: &[(&str, &str)] = &[
        ("sum_targeted.txt", TARGETED),
        ("sum_differential.txt", DIFF_MANIFEST),
        ("sum_differential_sample.txt", DIFF_SAMPLE),
        ("streams.txt", STREAMS),
        ("streams_sample.txt", STREAMS_SAMPLE),
        ("ledger.txt", include_str!("ledger.txt")),
        ("formation.txt", include_str!("formation.txt")),
        ("models.txt", include_str!("models.txt")),
        ("r1_cases.txt", include_str!("r1_cases.txt")),
        ("classification.txt", include_str!("classification.txt")),
        ("o8_states.txt", include_str!("o8_states.txt")),
        ("encodings.txt", include_str!("encodings.txt")),
    ];
    for (name, text) in files {
        let line = SHA256SUMS
            .lines()
            .find(|l| l.ends_with(&format!("  {name}")))
            .unwrap_or_else(|| panic!("{name} not in SHA256SUMS"));
        assert_eq!(sha256_hex(text.as_bytes()), &line[..64], "{name}");
    }
}

/// Adds one vector term (`[~]w:`, `f:`, `x:`, `i:`, `s:`) to a sum.
fn add_term<const L: usize>(
    sum: &mut ExactWideSum,
    ctx: &mut WideContext<L>,
    token: &str,
) -> Result<(), SumRefusal>
where
    Wide<L>: SupportedWidth,
{
    let (negate, body) = match token.strip_prefix('~') {
        Some(rest) => (true, rest),
        None => (false, token),
    };
    let (kind, value) = body.split_at(2);
    match kind {
        "w:" => sum.add_wide(&parse::<L>(value), negate),
        "f:" => sum.add_binary64(f64_bits(value), negate),
        "x:" => {
            let (a, b) = value.split_once('*').unwrap();
            sum.add_product(ctx, &parse::<L>(a), &parse::<L>(b), negate)
        }
        "i:" => {
            let negative = value.starts_with('-');
            let (hex, e) = value[1..].split_once('@').unwrap();
            sum.add_integer(negative != negate, &hex_limbs(hex), e.parse().unwrap())
        }
        "s:" => {
            let (w, rest) = value.split_once('*').unwrap();
            let (factor, pow2) = rest.split_once('@').unwrap();
            sum.add_wide_scaled(
                &parse::<L>(w),
                negate,
                factor.parse().unwrap(),
                pow2.parse().unwrap(),
            )
        }
        other => panic!("unknown term kind {other}"),
    }
}

fn run_targeted<const L: usize>(p: u32, terms: &[&str], expect: &str) -> bool
where
    Wide<L>: SupportedWidth,
{
    let mut c = ctx::<L>(p);
    let mut sum = ExactWideSum::new();
    let mut refused = None;
    for t in terms {
        if let Err(r) = add_term(&mut sum, &mut c, t) {
            refused = Some(r);
            break;
        }
    }
    let got = match refused {
        Some(r) => r,
        None => match sum.round(&mut c) {
            Ok(v) => {
                assert_eq!(tok(&v), expect, "p {p} terms {terms:?}");
                if expect == "Z+" {
                    assert!(!v.is_sign_negative(), "exact zero must be +0");
                }
                return true;
            }
            Err(r) => r,
        },
    };
    let expected = match expect {
        "refuse:span" => SumRefusal::Span,
        "refuse:exponent" => SumRefusal::Exponent,
        other => panic!("p {p} terms {terms:?}: refused {got:?}, expected {other}"),
    };
    assert_eq!(got, expected, "p {p} terms {terms:?}");
    false
}

#[test]
fn targeted_sums_match_the_fraction_oracle() {
    let mut per_width = [0usize; 3];
    let mut refusals = 0;
    let mut zeros = 0;
    for line in TARGETED.lines() {
        let (lhs, expect) = line.split_once(" = ").unwrap();
        let f: Vec<&str> = lhs.split_whitespace().collect();
        let (l, p, n) = (
            f[1],
            f[2].parse::<u32>().unwrap(),
            f[3].parse::<usize>().unwrap(),
        );
        let terms = &f[4..];
        assert_eq!(terms.len(), n);
        let ok = match l {
            "4" => {
                per_width[0] += 1;
                run_targeted::<4>(p, terms, expect)
            }
            "8" => {
                per_width[1] += 1;
                run_targeted::<8>(p, terms, expect)
            }
            "16" => {
                per_width[2] += 1;
                run_targeted::<16>(p, terms, expect)
            }
            other => panic!("width {other}"),
        };
        refusals += usize::from(!ok);
        zeros += usize::from(expect == "Z+");
    }
    assert!(per_width.iter().all(|&n| n > 50), "{per_width:?}");
    assert_eq!(refusals, 4, "span and exponent refusals");
    assert!(zeros >= 30, "{zeros}");
}

#[test]
fn rv12_counterexample_is_rounded_once_and_a_fold_misrounds_it() {
    let mut c = ctx::<4>(128);
    let one_plus = w_value::<4>(false, 0, &[1, 1 << 63]); // 1 + 2^-127
    let half = w_value::<4>(false, -128, &[1]);
    let far = w_value::<4>(true, -400, &[1]);
    let mut sum = ExactWideSum::new();
    for t in [&one_plus, &half, &far] {
        sum.add_wide(t, false).unwrap();
    }
    let once = sum.round(&mut c).unwrap();
    assert_eq!(tok(&once), tok(&one_plus));
    // The fold at p (the mutant K4-M1's arithmetic) gives 1 + 2^-126.
    let a = c.add(&one_plus, &half).unwrap();
    let folded = c.add(&a, &far).unwrap();
    assert_ne!(tok(&folded), tok(&once));
}

#[test]
fn zero_negation_absolute_value_and_reuse() {
    let mut c = ctx::<8>(320);
    let mut sum = ExactWideSum::new();
    let x = lift::<8>(-3.5);
    sum.add_wide(&x, false).unwrap();
    sum.add_binary64(3.5, false).unwrap();
    assert!(sum.is_zero());
    let z = sum.round(&mut c).unwrap();
    assert!(z.is_zero() && !z.is_sign_negative());
    sum.clear();
    let empty = sum.round(&mut c).unwrap();
    assert!(empty.is_zero() && !empty.is_sign_negative());
    sum.add_binary64(-2.0, false).unwrap();
    assert_eq!(sum.signum(), -1);
    sum.make_absolute();
    assert_eq!(tok(&sum.round(&mut c).unwrap()), tok(&lift::<8>(2.0)));
    sum.negate();
    assert_eq!(tok(&sum.round(&mut c).unwrap()), tok(&lift::<8>(-2.0)));
    // add_scaled: 3·2^5·(−2) + 192 = 0 exactly.
    let mut other = ExactWideSum::new();
    other.add_scaled(&sum, false, 3, 5).unwrap();
    other.add_binary64(192.0, false).unwrap();
    assert!(other.is_zero());
    // Work is charged (term limbs, nettings, the from_integer length).
    let w = sum.work();
    assert!(w.term_limbs > 0 && w.net_limbs > 0 && w.rounded_limbs > 0);
    assert_eq!(
        w.limb_multiply_equivalents(),
        w.term_limbs + w.shift_limbs + w.net_limbs + w.rounded_limbs
    );
}

#[test]
fn a_refused_span_adds_nothing() {
    let mut c = ctx::<4>(128);
    let mut sum = ExactWideSum::new();
    sum.add_binary64(1.0, false).unwrap();
    let far = w_value::<4>(false, -8128, &[1]);
    assert_eq!(sum.add_wide(&far, false), Err(SumRefusal::Span));
    // The value is unchanged by the refused term.
    assert_eq!(tok(&sum.round(&mut c).unwrap()), tok(&lift::<4>(1.0)));
    let near = w_value::<4>(false, -8127, &[1]);
    sum.add_wide(&near, false).unwrap();
    assert_eq!(sum.work().max_span_bits, 8128);
}

// ---------------------------------------------------------------- the differential

/// The generator's `rand_mantissa`, call for call (little-endian limbs).
fn rand_mantissa(rng: &mut SplitMix64, p: usize) -> Vec<u64> {
    let l = p.div_ceil(64);
    let mut m = shr(&rand_bits(rng, l), 64 * l - p);
    set_bit(&mut m, p - 1);
    let sel = rng.next();
    match sel % 6 {
        0 => {
            let nb = 1 + (sel >> 8) as usize % p;
            clear_below(&mut m, p - nb);
        }
        1 => {
            m = vec![0; l];
            for i in 0..p {
                set_bit(&mut m, i);
            }
        }
        2 => {
            m = vec![0; l];
            set_bit(&mut m, p - 1);
            set_bit(&mut m, (sel >> 8) as usize % (p - 1));
        }
        _ => {}
    }
    m
}

enum Term<const L: usize>
where
    Wide<L>: SupportedWidth,
{
    Value(Wide<L>, bool),
    Binary64(u64, bool),
    Product(Wide<L>, Wide<L>, bool),
    Integer(bool, Vec<u64>, i64, bool),
}

impl<const L: usize> Term<L>
where
    Wide<L>: SupportedWidth,
{
    fn negated(&self) -> Self {
        match self {
            Self::Value(w, n) => Self::Value(*w, !n),
            Self::Binary64(b, n) => Self::Binary64(*b, !n),
            Self::Product(a, b, n) => Self::Product(*a, *b, !n),
            Self::Integer(s, m, e, n) => Self::Integer(*s, m.clone(), *e, !n),
        }
    }

    fn token(&self) -> String {
        let (neg, body) = match self {
            Self::Value(w, n) => (*n, format!("w:{}", tok(w))),
            Self::Binary64(b, n) => (*n, format!("f:{b:016x}")),
            Self::Product(a, b, n) => (*n, format!("x:{}*{}", tok(a), tok(b))),
            Self::Integer(s, m, e, n) => {
                let mut hex = String::new();
                let top = m.iter().rposition(|&l| l != 0).unwrap_or(0);
                hex.push_str(&format!("{:x}", m[top]));
                for k in (0..top).rev() {
                    hex.push_str(&format!("{:016x}", m[k]));
                }
                (*n, format!("i:{}{hex}@{e}", if *s { '-' } else { '+' }))
            }
        };
        format!("{}{body}", if neg { "~" } else { "" })
    }

    fn add(&self, sum: &mut ExactWideSum, c: &mut WideContext<L>) -> Result<(), SumRefusal> {
        match self {
            Self::Value(w, n) => sum.add_wide(w, *n),
            Self::Binary64(b, n) => sum.add_binary64(f64::from_bits(*b), *n),
            Self::Product(a, b, n) => sum.add_product(c, a, b, *n),
            Self::Integer(s, m, e, n) => sum.add_integer(*s != *n, m, *e),
        }
    }
}

/// The generator's `gen_sum`, call for call.
fn gen_sum<const L: usize>(rng: &mut SplitMix64, p: usize) -> Vec<Term<L>>
where
    Wide<L>: SupportedWidth,
{
    let r = rng.next();
    let n = 2 + r % 63;
    let base = ((r >> 8) % 801) as i64 - 400;
    let mut terms: Vec<Term<L>> = Vec::new();
    for _ in 0..n {
        let s = rng.next();
        let kind = s % 8;
        let neg = (s >> 3) & 1 == 1;
        if kind == 4 && !terms.is_empty() {
            let k = ((s >> 4) % terms.len() as u64) as usize;
            let copy = terms[k].negated();
            terms.push(copy);
            continue;
        }
        if kind <= 4 {
            let mut e = base + ((s >> 4) % 257) as i64 - 128;
            if (s >> 16) % 16 == 0 {
                e = base + ((s >> 20) % 4001) as i64 - 2000;
            }
            let m = rand_mantissa(rng, p);
            terms.push(Term::Value(w_value::<L>(neg, e, &m), false));
        } else if kind == 5 {
            let biased = (base + 1023 + ((s >> 4) % 101) as i64 - 50).clamp(1, 2046) as u64;
            let frac = rng.next() & ((1u64 << 52) - 1);
            let bits = (u64::from(neg) << 63) | (biased << 52) | frac;
            terms.push(Term::Binary64(bits, false));
        } else if kind == 6 {
            let ea = base.div_euclid(2) + ((s >> 4) % 101) as i64 - 50;
            let eb = base - base.div_euclid(2) + ((s >> 12) % 101) as i64 - 50;
            let a = w_value::<L>(neg, ea, &rand_mantissa(rng, p));
            let b = w_value::<L>(false, eb, &rand_mantissa(rng, p));
            terms.push(Term::Product(a, b, false));
        } else {
            let limbs = 1 + ((s >> 4) % 8) as usize;
            let mut mag = rand_bits(rng, limbs);
            mag[0] |= 1;
            let e = base - 64 * limbs as i64 + ((s >> 12) % 129) as i64 - 64;
            terms.push(Term::Integer(neg, mag, e, false));
        }
    }
    terms
}

fn run_sum_differential<const L: usize>(name: &str, p: u32)
where
    Wide<L>: SupportedWidth,
{
    let started = Instant::now();
    let (seed, count, chunk_len, sha, chunks) = manifest_stream(DIFF_MANIFEST, "stream", name);
    assert!(count >= 100_000, "{name}: {count}");
    let samples: Vec<&str> = DIFF_SAMPLE
        .lines()
        .filter(|l| l.split_whitespace().next() == Some(name))
        .collect();
    assert_eq!(samples.len(), 1000, "{name}");
    let mut rng = SplitMix64(seed);
    let mut c = ctx::<L>(p);
    let mut sum = ExactWideSum::new();
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut record = Vec::new();
    let mut term_count = 0usize;
    for i in 0..count {
        let terms = gen_sum::<L>(&mut rng, p as usize);
        term_count += terms.len();
        sum.clear();
        for t in &terms {
            t.add(&mut sum, &mut c).unwrap();
        }
        let v = sum.round(&mut c).unwrap();
        if let Some(sample) = samples.get(i) {
            let (lhs, expect) = sample.split_once(" = ").unwrap();
            let f: Vec<&str> = lhs.split_whitespace().collect();
            assert_eq!(f[1].parse::<usize>().unwrap(), i);
            let tokens: Vec<String> = terms.iter().map(|t| t.token()).collect();
            assert_eq!(
                f[2].parse::<usize>().unwrap(),
                terms.len(),
                "{name} sum {i}"
            );
            assert_eq!(
                sha256_hex(tokens.join(" ").as_bytes()),
                f[3],
                "{name} sum {i} (generator port)"
            );
            assert_eq!(tok(&v), expect, "{name} sum {i}");
        }
        record.clear();
        record.push(0);
        enc(&v, &mut record);
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % chunk_len == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            assert_eq!(
                done.hex(),
                chunks[i / chunk_len],
                "{name} chunk {}",
                i / chunk_len
            );
        }
    }
    assert_eq!(total.hex(), sha, "{name}");
    println!(
        "{name}: {count} sums ({term_count} terms) at p = {p}, L = {L}, debug wall time {:.1} s",
        started.elapsed().as_secs_f64()
    );
}

#[test]
fn sum_differential_p128() {
    run_sum_differential::<4>("sum128", 128);
}
#[test]
fn sum_differential_p192() {
    run_sum_differential::<4>("sum192", 192);
}
#[test]
fn sum_differential_p256() {
    run_sum_differential::<4>("sum256", 256);
}
#[test]
fn sum_differential_p320() {
    run_sum_differential::<8>("sum320", 320);
}
#[test]
fn sum_differential_p512() {
    run_sum_differential::<8>("sum512", 512);
}
#[test]
fn sum_differential_p576() {
    run_sum_differential::<16>("sum576", 576);
}
#[test]
fn sum_differential_p1024() {
    run_sum_differential::<16>("sum1024", 1024);
}

// ---------------------------------------------------------------- D: the N5 streams

fn run_stream<const L: usize>(name: &str, p: u32)
where
    Wide<L>: SupportedWidth,
{
    let started = Instant::now();
    let (seed, count, chunk_len, sha, chunks) = manifest_stream(STREAMS, "stream", name);
    assert!(count >= 1_000_000, "{name}: {count}");
    let samples: Vec<Vec<&str>> = STREAMS_SAMPLE
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<_>>())
        .filter(|f| f[0] == name)
        .collect();
    assert_eq!(samples.len(), 1000, "{name}");
    let mut rng = SplitMix64(seed);
    let mut c = ctx::<L>(p);
    let mut total = Sha256::new();
    let mut chunk = Sha256::new();
    let mut record = Vec::new();
    let mut per_op = [0usize; 5];
    for i in 0..count {
        let (op, a, b) = k3_gen_operands::<L>(&mut rng);
        per_op[op as usize] += 1;
        let result = apply(&mut c, op, &a, &b);
        if let Some(sample) = samples.get(i) {
            assert_eq!(
                (
                    sample[2],
                    sample[3].parse::<u32>().unwrap(),
                    sample[4],
                    sample[5]
                ),
                (OP_NAMES[op as usize], p, tok(&a).as_str(), tok(&b).as_str()),
                "{name} operands {i} (generator port)"
            );
            let got = match &result {
                Ok(v) => tok(v),
                Err(_) => "E:div0".to_string(),
            };
            assert_eq!(got, sample[6], "{name} record {i}");
        }
        record.clear();
        record.push(op);
        match result {
            Ok(v) => {
                record.push(0);
                enc(&v, &mut record);
            }
            Err(super::super::wide::WideError::DivisionByZero) => {
                record.push(1);
                record.extend(std::iter::repeat_n(0u8, 9 + 8 * L));
            }
            Err(e) => panic!("{name} record {i}: unexpected {e:?}"),
        }
        total.update(&record);
        chunk.update(&record);
        if (i + 1) % chunk_len == 0 {
            let done = std::mem::replace(&mut chunk, Sha256::new());
            assert_eq!(
                done.hex(),
                chunks[i / chunk_len],
                "{name} chunk {}",
                i / chunk_len
            );
        }
    }
    assert_eq!(total.hex(), sha, "{name} stream");
    for (op, n) in per_op.iter().enumerate() {
        assert!(*n > count / 6, "{name}: {} {}", OP_NAMES[op], n);
    }
    println!(
        "{name}: {count} operations at p = {p}, L = {L}, debug wall time {:.1} s",
        started.elapsed().as_secs_f64()
    );
}

#[test]
fn n5_stream_p128_l4() {
    run_stream::<4>("p128", 128);
}
#[test]
fn n5_stream_p192_l4() {
    run_stream::<4>("p192", 192);
}
#[test]
fn n5_stream_p320_l8() {
    run_stream::<8>("p320", 320);
}
#[test]
fn n5_stream_p576_l16() {
    run_stream::<16>("p576", 576);
}
