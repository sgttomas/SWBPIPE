//! Bounded private normalized member arithmetic. These scalar records carry no
//! source identity, eligibility, row certificate, admission policy, or tariff.
#![allow(dead_code)] // No product caller in this isolated implementation slice.

use super::adaptive::AttemptStop;
use super::directed::{self, Toward};
use super::wide::multi::{AttemptWork, WideContext};
use super::wide::Wide;
use super::wide_sum::{ExactWideSum, SumWork};
use super::work::{WorkFault, WorkStatus, WorkTotal};
use std::cmp::Ordering;

type Endpoint = Wide<16>;

#[derive(Debug, Clone, Copy)]
pub(super) struct Enclosure {
    lo: Endpoint,
    hi: Endpoint,
}
impl Enclosure {
    fn point(x: Endpoint) -> Self {
        Self { lo: x, hi: x }
    }
    fn positive(self) -> Result<Self, NumericError> {
        if positive(&self.lo) && self.lo.cmp_value(&self.hi) != Ordering::Greater {
            Ok(self)
        } else {
            Err(NumericError::NonpositiveSource)
        }
    }
    fn hull_point(self, x: Endpoint) -> Self {
        Self {
            lo: min(self.lo, x),
            hi: max(self.hi, x),
        }
    }
}

/// Numeric routes only. A future checked source adapter must prove selection,
/// adjacency, actual owner/ordinal association and all existing admission rules.
#[derive(Debug, Clone, Copy)]
pub(super) enum MaterialOperands {
    ExactENu {
        e: f64,
        nu: f64,
    },
    Ordinary {
        e: f64,
        g: f64,
    }, // base or exact point, independently selected E/G
    Interpolated {
        t_lo: f64,
        t: f64,
        t_hi: f64,
        e_lo: f64,
        e_hi: f64,
        g_lo: f64,
        g_hi: f64,
        e_hat: f64,
        g_hat: f64,
    },
}
#[derive(Debug, Clone, Copy)]
pub(super) struct AdmittedOperands {
    e: f64,
    g: f64,
    a: f64,
    j: f64,
    iz: f64,
    iy: f64,
    z_hat: f64,
}
#[derive(Debug, Clone, Copy)]
pub(super) struct MemberOperands {
    diameter: f64,
    effective_wall: f64,
    material: MaterialOperands,
    admitted: AdmittedOperands,
}
#[derive(Debug)]
pub(super) struct MemberEnclosures {
    effective_wall_bits: u64,
    c: Endpoint,
    ri: Enclosure,
    p: Enclosure,
    q: Enclosure,
    geometry_g: Enclosure,
    a: Enclosure,
    i: Enclosure,
    j: Enclosure,
    z: Enclosure,
    e: Enclosure,
    g: Enclosure,
    coefficients: [Enclosure; 4],
    admitted_products: [Endpoint; 4],
    coefficient_differences: [Endpoint; 4],
    represented_z: Option<Enclosure>,
}
#[derive(Debug, Clone, PartialEq)]
pub(super) enum NumericError {
    Arithmetic(AttemptStop),
    NonFinite,
    NonpositiveSource,
    InvalidGeometry,
    InvalidMaterial,
    MaterialBits,
    TemperatureOrder,
    NonpositiveDenominator,
    AxisBits,
    Binary64Range,
}
impl From<AttemptStop> for NumericError {
    fn from(e: AttemptStop) -> Self {
        Self::Arithmetic(e)
    }
}

#[derive(Debug, Clone, Copy)]
enum Entry {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    R4 = 4,
    B64U = 5,
}
#[derive(Debug)]
pub(super) struct NumericWork {
    wide: AttemptWork,
    sums: SumWork,
    entries: [WorkTotal; 6],
    // No rounded binary64 arithmetic is performed by this component. Conversion
    // entries are counted separately; abs/successor are bit operations.
    f64_arithmetic: WorkTotal,
    status: WorkStatus,
}
impl NumericWork {
    fn new() -> Self {
        Self {
            wide: AttemptWork::default(),
            sums: SumWork::default(),
            entries: [WorkTotal::zero(); 6],
            f64_arithmetic: WorkTotal::zero(),
            status: WorkStatus::default(),
        }
    }
    fn status(&self) -> WorkStatus {
        self.entries.iter().fold(
            self.wide
                .checked_lme()
                .add(self.sums.checked_lme())
                .add(self.f64_arithmetic)
                .status()
                .join(self.status),
            |s, v| s.join(v.status()),
        )
    }
    fn begin(&mut self, entry: Entry) -> Result<(), NumericError> {
        if let Some(f) = self.status().fault() {
            return Err(AttemptStop::WorkAccounting(f).into());
        }
        let i = entry as usize;
        let next = self.entries[i].add(WorkTotal::exact_count(1));
        self.status = self.status.join(next.status());
        if let Some(f) = next.status().fault() {
            return Err(AttemptStop::WorkAccounting(f).into());
        }
        self.entries[i] = next;
        Ok(())
    }
    fn checked<T>(&mut self, result: Result<T, NumericError>) -> Result<T, NumericError> {
        if let Err(NumericError::Arithmetic(AttemptStop::WorkAccounting(f))) = &result {
            self.status = self.status.join(WorkStatus::from_fault(*f));
        }
        match result {
            Err(e) => Err(e), // retain original numeric cause, even with unavailable work
            Ok(v) => match self.status().fault() {
                Some(f) => Err(AttemptStop::WorkAccounting(f).into()),
                None => Ok(v),
            },
        }
    }
}
#[derive(Debug)]
pub(super) struct MemberSpent {
    result: Result<MemberEnclosures, NumericError>,
    work: NumericWork,
}
impl MemberSpent {
    pub(super) fn result(&self) -> Result<&MemberEnclosures, NumericError> {
        match &self.result {
            Err(e) => Err(e.clone()),
            Ok(v) => match self.work.status().fault() {
                Some(f) => Err(AttemptStop::WorkAccounting(f).into()),
                None => Ok(v),
            },
        }
    }
    pub(super) fn work(&self) -> &NumericWork {
        &self.work
    }
}

fn positive(x: &Endpoint) -> bool {
    !x.is_zero() && !x.is_sign_negative()
}
fn min(a: Endpoint, b: Endpoint) -> Endpoint {
    if a.cmp_value(&b) == Ordering::Greater {
        b
    } else {
        a
    }
}
fn max(a: Endpoint, b: Endpoint) -> Endpoint {
    if a.cmp_value(&b) == Ordering::Less {
        b
    } else {
        a
    }
}
fn lift(v: f64) -> Result<Endpoint, NumericError> {
    if !v.is_finite() {
        return Err(NumericError::NonFinite);
    }
    if v == 0.0 {
        return Ok(Endpoint::ZERO);
    }
    Endpoint::from_f64(v).map_err(|e| NumericError::Arithmetic(e.into()))
}
fn pos_lift(v: f64) -> Result<Endpoint, NumericError> {
    let x = lift(v)?;
    if positive(&x) {
        Ok(x)
    } else {
        Err(NumericError::NonpositiveSource)
    }
}
fn shift(x: &Endpoint, k: i64) -> Result<Endpoint, NumericError> {
    x.mul_pow2(k)
        .map_err(|e| NumericError::Arithmetic(e.into()))
}
fn shift_interval(x: Enclosure, k: i64) -> Result<Enclosure, NumericError> {
    Ok(Enclosure {
        lo: shift(&x.lo, k)?,
        hi: shift(&x.hi, k)?,
    })
}

// An operation owns its fresh context and sum. No fallible exit can bypass
// collection, including denominator rejection and lower-pass exact sign checks.
fn scalar_owned(
    work: &mut NumericWork,
    entry: Entry,
    a: &Endpoint,
    b: &Endpoint,
    toward: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> Result<Endpoint, NumericError> {
    let result = (|| {
        if matches!(entry, Entry::Div) && !positive(b) {
            return Err(NumericError::NonpositiveDenominator);
        }
        let v = match entry {
            Entry::Add => directed::add_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Sub => directed::sub_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Mul => directed::mul_toward(&mut ctx, &mut sum, a, b, toward),
            Entry::Div => directed::div_toward(&mut ctx, &mut sum, a, b, toward),
            _ => return Err(AttemptStop::WorkAccounting(WorkFault::Inconsistent).into()),
        }?;
        Ok(if v.is_zero() { Endpoint::ZERO } else { v })
    })();
    work.wide.record(&ctx);
    work.sums.merge(&sum.work());
    work.checked(result)
}
impl NumericWork {
    fn scalar(
        &mut self,
        entry: Entry,
        a: &Endpoint,
        b: &Endpoint,
        t: Toward,
    ) -> Result<Endpoint, NumericError> {
        self.begin(entry)?;
        let ctx = WideContext::<16>::new(1024).map_err(|e| NumericError::Arithmetic(e.into()))?;
        scalar_owned(self, entry, a, b, t, ctx, ExactWideSum::new())
    }
    fn mul(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        a.positive()?;
        b.positive()?;
        Ok(Enclosure {
            lo: self.scalar(Entry::Mul, &a.lo, &b.lo, Toward::Down)?,
            hi: self.scalar(Entry::Mul, &a.hi, &b.hi, Toward::Up)?,
        })
        .and_then(Enclosure::positive)
    }
    fn div(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        a.positive()?;
        if !positive(&b.lo) || b.lo.cmp_value(&b.hi) == Ordering::Greater {
            return Err(NumericError::NonpositiveDenominator);
        }
        Ok(Enclosure {
            lo: self.scalar(Entry::Div, &a.lo, &b.hi, Toward::Down)?,
            hi: self.scalar(Entry::Div, &a.hi, &b.lo, Toward::Up)?,
        })
        .and_then(Enclosure::positive)
    }
    fn sub_pair(&mut self, a: &Endpoint, b: &Endpoint) -> Result<Enclosure, NumericError> {
        Ok(Enclosure {
            lo: self.scalar(Entry::Sub, a, b, Toward::Down)?,
            hi: self.scalar(Entry::Sub, a, b, Toward::Up)?,
        })
    }
    fn add(&mut self, a: Enclosure, b: Enclosure) -> Result<Enclosure, NumericError> {
        Ok(Enclosure {
            lo: self.scalar(Entry::Add, &a.lo, &b.lo, Toward::Down)?,
            hi: self.scalar(Entry::Add, &a.hi, &b.hi, Toward::Up)?,
        })
    }
    fn exact_product(&mut self, a: f64, b: f64) -> Result<Endpoint, NumericError> {
        // Separate lifts preserve up to 106 significant bits, including 2^-104.
        let (a, b) = (lift(a)?, lift(b)?);
        self.scalar(Entry::Mul, &a, &b, Toward::Up)
    }
    fn r4(&mut self, p: &[Endpoint; 4], t: Toward) -> Result<Endpoint, NumericError> {
        self.begin(Entry::R4)?;
        let ctx = WideContext::<16>::new(1024).map_err(|e| NumericError::Arithmetic(e.into()))?;
        r4_owned(self, p, t, ctx, ExactWideSum::new())
    }
    fn absdiff(&mut self, a: &Endpoint, b: &Endpoint) -> Result<Endpoint, NumericError> {
        let (large, small) = if a.cmp_value(b) == Ordering::Less {
            (b, a)
        } else {
            (a, b)
        };
        self.scalar(Entry::Sub, large, small, Toward::Up)
    }
    fn binary64_up(&mut self, x: &Endpoint) -> Result<f64, NumericError> {
        if x.is_sign_negative() && !x.is_zero() {
            return Err(NumericError::InvalidMaterial);
        }
        self.begin(Entry::B64U)?;
        let (result, sums) = directed::binary64_up_spent(x).into_parts();
        self.collect_binary64(result, sums)
    }
    fn collect_binary64(
        &mut self,
        result: Result<f64, AttemptStop>,
        sums: SumWork,
    ) -> Result<f64, NumericError> {
        self.sums.merge(&sums);
        let value = self.checked(result.map_err(NumericError::Arithmetic))?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(NumericError::Binary64Range)
        }
    }
}
fn r4_owned(
    work: &mut NumericWork,
    p: &[Endpoint; 4],
    t: Toward,
    mut ctx: WideContext<16>,
    mut sum: ExactWideSum,
) -> Result<Endpoint, NumericError> {
    let result = (|| {
        for (v, negate) in p.iter().zip([false, true, false, true]) {
            sum.add_wide(v, negate).map_err(AttemptStop::from)?;
        }
        if t == Toward::Down && sum.signum().map_err(AttemptStop::from)? <= 0 {
            return Err(NumericError::NonpositiveSource);
        }
        directed::round_toward(&mut ctx, &mut sum, t).map_err(NumericError::Arithmetic)
    })();
    work.wide.record(&ctx);
    work.sums.merge(&sum.work());
    work.checked(result)
}

fn material(
    work: &mut NumericWork,
    m: MaterialOperands,
    k: &AdmittedOperands,
) -> Result<(Enclosure, Enclosure, bool), NumericError> {
    match m {
        MaterialOperands::ExactENu { e, nu } => {
            let e = pos_lift(e)?;
            let nu = lift(nu)?;
            if nu.cmp_value(&Endpoint::ONE.neg()) != Ordering::Greater
                || nu.cmp_value(&shift(&Endpoint::ONE, -1)?) != Ordering::Less
            {
                return Err(NumericError::InvalidMaterial);
            }
            if e.cmp_value(&lift(k.e)?) != Ordering::Equal {
                return Err(NumericError::MaterialBits);
            }
            let denominator = work.add(Enclosure::point(Endpoint::ONE), Enclosure::point(nu))?;
            let denominator = shift_interval(denominator, 1)?;
            let e = Enclosure::point(e);
            let g = work.div(e, denominator)?;
            Ok((e, g, false))
        }
        MaterialOperands::Ordinary { e, g } => {
            let pair = (pos_lift(e)?, pos_lift(g)?);
            if e.to_bits() != k.e.to_bits() || g.to_bits() != k.g.to_bits() {
                return Err(NumericError::MaterialBits);
            }
            Ok((Enclosure::point(pair.0), Enclosure::point(pair.1), true))
        }
        MaterialOperands::Interpolated {
            t_lo,
            t,
            t_hi,
            e_lo,
            e_hi,
            g_lo,
            g_hi,
            e_hat,
            g_hat,
        } => {
            let (lo, at, hi) = (lift(t_lo)?, lift(t)?, lift(t_hi)?);
            if lo.cmp_value(&at) != Ordering::Less || at.cmp_value(&hi) != Ordering::Less {
                return Err(NumericError::TemperatureOrder);
            }
            let (ea, ga) = (pos_lift(e_hat)?, pos_lift(g_hat)?);
            pos_lift(g_lo)?;
            pos_lift(g_hi)?;
            if e_hat.to_bits() != k.e.to_bits() || g_hat.to_bits() != k.g.to_bits() {
                return Err(NumericError::MaterialBits);
            }
            let h = work.sub_pair(&hi, &lo)?.positive()?;
            let mut property = |xlo: f64, xhi: f64, actual: Endpoint| {
                let p = [
                    work.exact_product(t_hi, xlo)?,
                    work.exact_product(t, xlo)?,
                    work.exact_product(t, xhi)?,
                    work.exact_product(t_lo, xhi)?,
                ];
                let n = Enclosure {
                    lo: work.r4(&p, Toward::Down)?,
                    hi: work.r4(&p, Toward::Up)?,
                };
                Ok::<_, NumericError>(work.div(n, h)?.hull_point(actual))
            };
            let e = property(e_lo, e_hi, ea)?;
            let g = property(g_lo, g_hi, ga)?;
            Ok((e, g, true))
        }
    }
}

fn pi() -> Result<Enclosure, NumericError> {
    // Reviewed 514-bit numerators / 2^512, normalized at exponent 1. Fixed
    // constants only; no binary64 pi and no runtime transcendental calculation.
    const LOWER: [u64; 16] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0xc000000000000000,
        0x4fe1356d6d51c245,
        0x302b0a6df25f1437,
        0xef9519b3cd3a431b,
        0x514a08798e3404dd,
        0x020bbea63b139b22,
        0x29024e088a67cc74,
        0xc4c6628b80dc1cd1,
        0xc90fdaa22168c234,
    ];
    const UPPER: [u64; 16] = [
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x4fe1356d6d51c246,
        0x302b0a6df25f1437,
        0xef9519b3cd3a431b,
        0x514a08798e3404dd,
        0x020bbea63b139b22,
        0x29024e088a67cc74,
        0xc4c6628b80dc1cd1,
        0xc90fdaa22168c234,
    ];
    let make = |limbs| {
        Endpoint::from_parts(false, 1, limbs).map_err(|e| NumericError::Arithmetic(e.into()))
    };
    Ok(Enclosure {
        lo: make(LOWER)?,
        hi: make(UPPER)?,
    })
}

pub(super) fn member_coefficients(input: &MemberOperands) -> MemberSpent {
    let mut work = NumericWork::new();
    let result = build_member(input, &mut work);
    let result = work.checked(result);
    MemberSpent { result, work }
}
fn build_member(
    input: &MemberOperands,
    work: &mut NumericWork,
) -> Result<MemberEnclosures, NumericError> {
    let k = &input.admitted;
    for v in [k.e, k.g, k.a, k.j, k.iz, k.iy] {
        pos_lift(v)?;
    }
    let (e, g, ordinary) = material(work, input.material, k)?;
    let d = pos_lift(input.diameter)?;
    let t = pos_lift(input.effective_wall)?;
    let c = shift(&d, -1)?;
    if t.cmp_value(&c) != Ordering::Less {
        return Err(NumericError::InvalidGeometry);
    }
    let ri = work.sub_pair(&c, &t)?.positive()?;
    let d_minus_t = work.sub_pair(&d, &t)?.positive()?;
    let p = work.mul(Enclosure::point(t), d_minus_t)?;
    // c is the exact binary64 D lift shifted by one: c*c has <=106 bits.
    let c_square = Enclosure::point(work.scalar(Entry::Mul, &c, &c, Toward::Up)?);
    let ri_square = work.mul(ri, ri)?;
    let q = work.add(c_square, ri_square)?.positive()?;
    let geometry_g = work.mul(p, q)?;
    let pi = pi()?;
    let a = work.mul(pi, p)?;
    let i = shift_interval(work.mul(pi, geometry_g)?, -2)?;
    let j = shift_interval(i, 1)?;
    let z = work.div(i, Enclosure::point(c))?;
    let coefficients = [
        work.mul(e, a)?,
        work.mul(g, j)?,
        work.mul(e, i)?,
        work.mul(e, i)?,
    ];
    let admitted_products = [
        work.exact_product(k.e, k.a)?,
        work.exact_product(k.g, k.j)?,
        work.exact_product(k.e, k.iz)?,
        work.exact_product(k.e, k.iy)?,
    ];
    let mut coefficient_differences = [Endpoint::ZERO; 4];
    for index in 0..4 {
        let lo = work.absdiff(&coefficients[index].lo, &admitted_products[index])?;
        let hi = work.absdiff(&coefficients[index].hi, &admitted_products[index])?;
        coefficient_differences[index] = max(lo, hi);
    }
    let represented_z = if ordinary {
        if k.iy.to_bits() != k.iz.to_bits() {
            return Err(NumericError::AxisBits);
        }
        let actual = pos_lift(k.z_hat)?;
        Some(
            work.div(Enclosure::point(pos_lift(k.iz)?), Enclosure::point(c))?
                .hull_point(actual),
        )
    } else {
        None
    };
    Ok(MemberEnclosures {
        effective_wall_bits: input.effective_wall.to_bits(),
        c,
        ri,
        p,
        q,
        geometry_g,
        a,
        i,
        j,
        z,
        e,
        g,
        coefficients,
        admitted_products,
        coefficient_differences,
        represented_z,
    })
}

#[cfg(test)]
#[path = "../../../tests/retained_k4/product_certificate_tests.rs"]
mod tests;
