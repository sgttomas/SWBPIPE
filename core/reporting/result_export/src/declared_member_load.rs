//! Reader-owned D1–D5 arithmetic. No producer/solver arithmetic is linked here.
//!
//! All integers have at most 128 limbs (8192 bits). D1/D5 span at most 5220
//! bits and complete nodal/member nets at most 7325 bits in the admitted D1
//! domain. Every allocating operation reserves its final bounded length once;
//! no capacity-doubling push loop is used. Only the 128 six-entry declarations
//! are retained; each of the at most 192 DOF nets is built and hashed in turn.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;

const LIMBS: usize = 128;
const LOADS: usize = 128;
const NODES: usize = 32;
const MEMBERS: usize = 32;
const DOFS: [&str; 6] = ["UX", "UY", "UZ", "RX", "RY", "RZ"];
type R<T> = Result<T, ()>;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Nat(Vec<u64>);
impl Nat {
    fn words(n: usize) -> R<Vec<u64>> {
        if n > LIMBS {
            return Err(());
        }
        Ok(vec![0; n])
    }
    fn small(n: u64) -> Self {
        if n == 0 {
            Self::default()
        } else {
            Self(vec![n])
        }
    }
    fn trim(mut self) -> Self {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
        self
    }
    fn bits(&self) -> usize {
        self.0.last().map_or(0, |x| {
            (self.0.len() - 1) * 64 + 64 - x.leading_zeros() as usize
        })
    }
    fn cmp(&self, b: &Self) -> Ordering {
        self.0
            .len()
            .cmp(&b.0.len())
            .then_with(|| self.0.iter().rev().cmp(b.0.iter().rev()))
    }
    fn bit(&self, i: usize) -> bool {
        self.0
            .get(i / 64)
            .is_some_and(|w| w & (1u64 << (i % 64)) != 0)
    }
    fn low_nonzero(&self, n: usize) -> bool {
        (0..n.div_ceil(64).min(self.0.len())).any(|i| {
            let mask = if n - i * 64 >= 64 {
                u64::MAX
            } else {
                (1u64 << (n - i * 64)) - 1
            };
            self.0[i] & mask != 0
        })
    }
    fn shl(&self, n: usize) -> R<Self> {
        if self.bits() == 0 {
            return Ok(Self::default());
        }
        let len = (self.bits().checked_add(n).ok_or(())?).div_ceil(64);
        let mut out = Self::words(len)?;
        for (i, &x) in self.0.iter().enumerate() {
            out[i + n / 64] |= x << (n % 64);
            if n % 64 != 0 && i + n / 64 + 1 < len {
                out[i + n / 64 + 1] |= x >> (64 - n % 64);
            }
        }
        Ok(Self(out).trim())
    }
    fn shr(&self, n: usize) -> Self {
        if n >= self.bits() {
            return Self::default();
        }
        let len = (self.bits() - n).div_ceil(64);
        let mut out = vec![0; len];
        for (i, v) in out.iter_mut().enumerate() {
            let at = i + n / 64;
            *v = self.0[at] >> (n % 64);
            if n % 64 != 0 && at + 1 < self.0.len() {
                *v |= self.0[at + 1] << (64 - n % 64);
            }
        }
        Self(out).trim()
    }
    fn add(&self, b: &Self) -> R<Self> {
        let len = self.0.len().max(b.0.len());
        let mut out = Self::words((len + 1).min(LIMBS))?;
        let mut carry = 0u128;
        for (i, v) in out.iter_mut().enumerate().take(len) {
            let x = u128::from(*self.0.get(i).unwrap_or(&0))
                + u128::from(*b.0.get(i).unwrap_or(&0))
                + carry;
            *v = x as u64;
            carry = x >> 64;
        }
        if carry != 0 {
            if len == LIMBS {
                return Err(());
            }
            out[len] = carry as u64;
        }
        Ok(Self(out).trim())
    }
    fn sub(&self, b: &Self) -> R<Self> {
        if self.cmp(b) == Ordering::Less {
            return Err(());
        }
        let mut out = Self::words(self.0.len())?;
        let mut borrow = false;
        for (i, v) in out.iter_mut().enumerate() {
            let (x, b1) = self.0[i].overflowing_sub(*b.0.get(i).unwrap_or(&0));
            let (x, b2) = x.overflowing_sub(u64::from(borrow));
            *v = x;
            borrow = b1 || b2;
        }
        if borrow {
            return Err(());
        }
        Ok(Self(out).trim())
    }
    fn mul(&self, b: &Self) -> R<Self> {
        if self.bits() == 0 || b.bits() == 0 {
            return Ok(Self::default());
        }
        let len = (self.bits() + b.bits()).div_ceil(64);
        let mut out = Self::words(len)?;
        for (i, &a) in self.0.iter().enumerate() {
            let mut carry = 0u128;
            for (j, &z) in b.0.iter().enumerate() {
                let at = i + j;
                if at >= len {
                    return Err(());
                }
                let x = u128::from(a) * u128::from(z) + u128::from(out[at]) + carry;
                out[at] = x as u64;
                carry = x >> 64;
            }
            let at = i + b.0.len();
            if carry != 0 {
                if at >= len {
                    return Err(());
                }
                out[at] = carry as u64;
            }
        }
        Ok(Self(out).trim())
    }
    fn div_small(&self, d: u64) -> R<(Self, u64)> {
        if d == 0 {
            return Err(());
        }
        let mut out = Self::words(self.0.len())?;
        let mut rem = 0u128;
        for i in (0..self.0.len()).rev() {
            let x = (rem << 64) | u128::from(self.0[i]);
            out[i] = (x / u128::from(d)) as u64;
            rem = x % u128::from(d);
        }
        Ok((Self(out).trim(), rem as u64))
    }
    // Restoring square root, two input bits per step; no floating operations.
    fn isqrt(&self) -> R<Self> {
        let mut root = Self::default();
        let mut rem = Self::default();
        for k in (0..self.bits().div_ceil(2)).rev() {
            rem = rem.shl(2)?.add(&Self::small(
                u64::from(self.bit(2 * k)) + 2 * u64::from(self.bit(2 * k + 1)),
            ))?;
            let trial = root.shl(2)?.add(&Self::small(1))?;
            root = root.shl(1)?;
            if rem.cmp(&trial) != Ordering::Less {
                rem = rem.sub(&trial)?;
                root = root.add(&Self::small(1))?;
            }
        }
        Ok(root)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Dyad {
    negative: bool,
    n: Nat,
    exponent: i32,
}
impl Dyad {
    fn normalized(mut self) -> Self {
        if self.n.bits() == 0 {
            return Self::default();
        }
        let i = self.n.0.iter().position(|&x| x != 0).unwrap();
        let shift = i * 64 + self.n.0[i].trailing_zeros() as usize;
        self.n = self.n.shr(shift);
        self.exponent += shift as i32;
        self
    }
    fn word(s: &Value) -> R<Self> {
        let s = s.as_str().ok_or(())?;
        if s.len() != 16 {
            return Err(());
        }
        let bits = u64::from_str_radix(s, 16).map_err(|_| ())?;
        Self::binary64(bits)
    }
    fn binary64(bits: u64) -> R<Self> {
        let e = ((bits >> 52) & 2047) as i32;
        if e == 2047 {
            return Err(());
        }
        let frac = bits & ((1u64 << 52) - 1);
        Ok(Self {
            negative: bits >> 63 != 0,
            n: Nat::small(if e == 0 { frac } else { frac | (1u64 << 52) }),
            exponent: if e == 0 { -1074 } else { e - 1075 },
        }
        .normalized())
    }
    fn neg(mut self) -> Self {
        if self.n.bits() != 0 {
            self.negative = !self.negative;
        }
        self
    }
    fn pow2(mut self, k: i32) -> Self {
        if self.n.bits() != 0 {
            self.exponent += k;
        }
        self
    }
    fn add(&self, b: &Self) -> R<Self> {
        if self.n.bits() == 0 {
            return Ok(b.clone());
        }
        if b.n.bits() == 0 {
            return Ok(self.clone());
        }
        let e = self.exponent.min(b.exponent);
        let a = self.n.shl((self.exponent - e) as usize)?;
        let z = b.n.shl((b.exponent - e) as usize)?;
        let (negative, n) = if self.negative == b.negative {
            (self.negative, a.add(&z)?)
        } else if a.cmp(&z) != Ordering::Less {
            (self.negative, a.sub(&z)?)
        } else {
            (b.negative, z.sub(&a)?)
        };
        Ok(Self {
            negative,
            n,
            exponent: e,
        }
        .normalized())
    }
    fn mul(&self, b: &Self) -> R<Self> {
        Ok(Self {
            negative: self.negative != b.negative,
            n: self.n.mul(&b.n)?,
            exponent: self.exponent + b.exponent,
        }
        .normalized())
    }
    fn rn(&self) -> R<Self> {
        if self.n.bits() <= 1024 {
            return Ok(self.clone());
        }
        let k = self.n.bits() - 1024;
        let mut n = self.n.shr(k);
        if self.n.bit(k - 1) && (self.n.low_nonzero(k - 1) || n.bit(0)) {
            n = n.add(&Nat::small(1))?;
        }
        Ok(Self {
            negative: self.negative,
            n,
            exponent: self.exponent + k as i32,
        }
        .normalized())
    }
    fn sqrt(&self) -> R<Self> {
        if self.negative || self.n.bits() == 0 {
            return Err(());
        }
        let top = self.exponent + self.n.bits() as i32 - 1;
        let q = top.div_euclid(2) - 1023;
        let shift = self.exponent - 2 * q;
        if shift < 0 {
            return Err(());
        }
        let rad = self.n.shl(shift as usize)?;
        let mut n = rad.isqrt()?;
        let midpoint = n.shl(1)?.add(&Nat::small(1))?;
        let c = rad.shl(2)?.cmp(&midpoint.mul(&midpoint)?);
        if c == Ordering::Greater || (c == Ordering::Equal && n.bit(0)) {
            n = n.add(&Nat::small(1))?;
        }
        Ok(Self {
            negative: false,
            n,
            exponent: q,
        }
        .normalized())
    }
    fn div12(&self) -> R<Self> {
        if self.n.bits() == 0 {
            return Ok(Self::default());
        }
        // floor(log2(n/12)) is either bits(n)-4 or bits(n)-5.
        let mut top = self.n.bits() as i32 - 4;
        let ge = if top >= 0 {
            self.n.cmp(&Nat::small(12).shl(top as usize)?) != Ordering::Less
        } else {
            self.n.shl((-top) as usize)?.cmp(&Nat::small(12)) != Ordering::Less
        };
        if !ge {
            top -= 1;
        }
        let shift = 1023 - top;
        if shift < 0 {
            return Err(());
        }
        let (mut n, rem) = self.n.shl(shift as usize)?.div_small(12)?;
        if rem > 6 || (rem == 6 && n.bit(0)) {
            n = n.add(&Nat::small(1))?;
        }
        Ok(Self {
            negative: self.negative,
            n,
            exponent: self.exponent - shift,
        }
        .normalized())
    }
}

struct Declared {
    nodes: [usize; 2],
    entries: [Dyad; 6],
}
fn index(v: &Value, cap: usize) -> R<usize> {
    let f = v.as_f64().ok_or(())?;
    if !f.is_finite() || f < 0.0 || f.fract() != 0.0 || f >= cap as f64 {
        return Err(());
    }
    let n = f as usize;
    if n >= cap {
        return Err(());
    }
    Ok(n)
}
fn array(v: &Value) -> R<&[Value]> {
    v.as_array().map(Vec::as_slice).ok_or(())
}
fn declared(source: &Value) -> R<Vec<Declared>> {
    let nodes = array(&source["id_maps"]["nodes"])?;
    let members = array(&source["id_maps"]["members"])?;
    let loads = array(&source["member_load_terms"])?;
    if nodes.len() > NODES || members.len() > MEMBERS || loads.is_empty() || loads.len() > LOADS {
        return Err(());
    }
    let mut out = Vec::with_capacity(loads.len());
    for load in loads {
        let member = &members[index(&load["member"], members.len())?];
        let ni = index(&member["node_i"], nodes.len())?;
        let nj = index(&member["node_j"], nodes.len())?;
        let mut x: [Dyad; 3] = Default::default();
        let mut y: [Dyad; 3] = Default::default();
        let mut w: [Dyad; 3] = Default::default();
        if array(&nodes[ni]["coordinates"])?.len() != 3
            || array(&nodes[nj]["coordinates"])?.len() != 3
            || array(&load["intensity"])?.len() != 3
        {
            return Err(());
        }
        for a in 0..3 {
            x[a] = Dyad::word(&nodes[ni]["coordinates"][a])?;
            y[a] = Dyad::word(&nodes[nj]["coordinates"][a])?;
            w[a] = Dyad::word(&load["intensity"][a])?;
        }
        let mut s = Dyad::default();
        for a in 0..3 {
            s = s
                .add(&y[a].mul(&y[a])?)?
                .add(&x[a].mul(&y[a])?.pow2(1).neg())?
                .add(&x[a].mul(&x[a])?)?;
        }
        let length = s.rn()?.sqrt()?;
        let g = length.div12()?;
        let mut entries: [Dyad; 6] = Default::default();
        for a in 0..3 {
            entries[a] = length.mul(&w[a])?.rn()?.pow2(-1);
            let b = (a + 1) % 3;
            let c = (a + 2) % 3;
            let cross = y[b]
                .mul(&w[c])?
                .add(&x[b].mul(&w[c])?.neg())?
                .add(&y[c].mul(&w[b])?.neg())?
                .add(&x[c].mul(&w[b])?)?;
            entries[a + 3] = g.mul(&cross)?.rn()?;
        }
        out.push(Declared {
            nodes: [ni, nj],
            entries,
        });
    }
    Ok(out)
}

/// Layout atoms for the source-pinned native/wasm workspace proof.
pub(crate) const fn layouts() -> [usize; 3] {
    [std::mem::size_of::<Nat>(), std::mem::size_of::<Dyad>(), std::mem::size_of::<Declared>()]
}

pub(crate) const fn layout_pairs() -> [(usize,usize);3] {
    [(std::mem::size_of::<Nat>(),std::mem::align_of::<Nat>()),
     (std::mem::size_of::<Dyad>(),std::mem::align_of::<Dyad>()),
     (std::mem::size_of::<Declared>(),std::mem::align_of::<Declared>())]
}

pub(crate) struct Recomputed {
    pub(crate) digest: String,
    pub(crate) nonzero: [bool; 192],
}
/// Complete canonical ledger and individual nonzero flags. JSON is borrowed;
/// zero occurrences and cancelling nets remain present. SHA consumes each net
/// immediately, so no encoded ledger buffer or 192-net vector coexists.
pub(crate) fn recompute(source: &Value) -> R<Recomputed> {
    let loads = declared(source)?;
    let nodes = array(&source["id_maps"]["nodes"])?;
    let nodal = array(&source["nodal_terms"])?;
    if nodal.len() > LOADS {
        return Err(());
    }
    let mut present = [false; 192];
    let mut nonzero = [false; 192];
    for t in nodal {
        let n = index(&t["dof"]["node"], nodes.len())?;
        let d = DOFS
            .iter()
            .position(|d| t["dof"]["component"] == *d)
            .ok_or(())?;
        present[6 * n + d] = true;
    }
    for load in &loads {
        for &node in &load.nodes {
            for d in 0..6 {
                present[6 * node + d] = true;
            }
        }
    }
    let mut hash = Sha256::new();
    hash.update(b"K4LED\x01");
    hash.update((present.iter().filter(|&&p| p).count() as u32).to_le_bytes());
    for dof in 0..nodes.len() * 6 {
        if !present[dof] {
            continue;
        }
        let mut net = Dyad::default();
        for t in nodal {
            let n = index(&t["dof"]["node"], nodes.len())?;
            let d = DOFS
                .iter()
                .position(|d| t["dof"]["component"] == *d)
                .ok_or(())?;
            if 6 * n + d == dof {
                let v = Dyad::word(&t["value"])?;
                nonzero[dof] |= v.n.bits() != 0;
                net = net.add(&v)?;
            }
        }
        for load in &loads {
            for end in 0..2 {
                if load.nodes[end] == dof / 6 {
                    let mut v = load.entries[dof % 6].clone();
                    if end == 1 && dof % 6 >= 3 {
                        v = v.neg();
                    }
                    nonzero[dof] |= v.n.bits() != 0;
                    net = net.add(&v)?;
                }
            }
        }
        hash.update(((dof / 6) as u32).to_le_bytes());
        hash.update([(dof % 6) as u8, u8::from(net.negative)]);
        hash.update(i64::from(net.exponent).to_le_bytes());
        hash.update((net.n.0.len() as u32).to_le_bytes());
        for word in net.n.0 {
            hash.update(word.to_le_bytes());
        }
    }
    Ok(Recomputed {
        digest: format!("{:x}", hash.finalize()),
        nonzero,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_integer_and_rounding_boundaries() {
        for n in 0..1000u64 {
            let r = Nat::small(n).isqrt().unwrap();
            let v = *r.0.first().unwrap_or(&0);
            assert!(v * v <= n && (v + 1) * (v + 1) > n);
        }
        let n = Nat::small(1)
            .shl(1024)
            .unwrap()
            .add(&Nat::small(1))
            .unwrap();
        let d = Dyad {
            negative: false,
            n,
            exponent: 0,
        };
        assert_eq!(d.rn().unwrap().n, Nat::small(1));
        assert_eq!(d.rn().unwrap().exponent, 1024);
        assert!(Nat::small(1).shl(8192).is_err());
    }
    fn token(word: &str) -> Dyad {
        if word == "Z+" || word == "Z-" {
            return Dyad::default();
        }
        let (hex, exp) = word[1..].split_once('p').unwrap();
        let mut n = Nat::default();
        for digit in hex.chars() {
            n = n
                .shl(4)
                .unwrap()
                .add(&Nat::small(digit.to_digit(16).unwrap() as u64))
                .unwrap();
        }
        Dyad {
            negative: word.starts_with('-'),
            n,
            exponent: exp.parse::<i32>().unwrap() + 1 - (hex.len() * 4) as i32,
        }
        .normalized()
    }
    #[test]
    fn reader_declared_entries_and_complete_ledger_match_third_oracle() {
        use serde_json::json;
        let text = include_str!("../../../solver/frame_kernel/tests/retained_k4/declared.txt");
        let mut source = Value::Null;
        let mut expected = Vec::new();
        let mut cases = 0;
        for line in text.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.is_empty() {
                continue;
            }
            match w[0] {
                "case" => {
                    source = json!({"id_maps":{"nodes":[{"coordinates":&w[2..5]},{"coordinates":&w[5..8]}],"members":[{"node_i":0,"node_j":1}]},"nodal_terms":[],"member_load_terms":[]});
                    expected.clear();
                    cases += 1;
                }
                "load" => source["member_load_terms"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"member":0,"intensity":&w[1..4]})),
                "entries" => expected.push(std::array::from_fn::<Dyad, 6, _>(|i| token(w[i + 1]))),
                "nodal" => {
                    let d = w[1].parse::<usize>().unwrap();
                    source["nodal_terms"]
                        .as_array_mut()
                        .unwrap()
                        .push(json!({"dof":{"node":d/6,"component":DOFS[d%6]},"value":w[2]}));
                }
                "ledger" => {
                    let got = declared(&source).unwrap();
                    for (a, b) in got.iter().zip(&expected) {
                        assert_eq!(a.entries, *b, "case {cases}");
                    }
                    assert_eq!(recompute(&source).unwrap().digest, w[2], "case {cases}");
                }
                "end" => {}
                _ => panic!("unknown vector record"),
            }
        }
        assert_eq!(cases, 16);
    }
}
