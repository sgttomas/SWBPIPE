//! Directed binary1024 branch and bound. Joint Bernstein controls preserve
//! axial/bending correlation; the positive-rho tangent majorizes the norm.
use super::*;

pub(super) const LIVE:usize=256;
const DEPTH:u8=64;
const SPLITS:usize=1024;
#[derive(Clone,Copy)]
pub(crate) struct SearchNode {
    controls:[[Enclosure;3];3], // degree-two joint controls: N, My, Mz
    upper:Endpoint,
    depth:u8,
}
fn half(a:Enclosure)->Result<Enclosure,ProductFailure> {
    Ok(Enclosure{lo:shift(&a.lo,-1)?,hi:shift(&a.hi,-1)?})
}
fn midpoint(w:&mut NumericWork,a:Enclosure,b:Enclosure)->Result<Enclosure,ProductFailure>{half(w.add(a,b)?) }
fn objective(w:&mut NumericWork,p:[Enclosure;3],a:Enclosure,z:Enclosure)->Result<Enclosure,ProductFailure> {
    let axial=corners(w,abs_range(p[0]),a,Entry::Div)?;
    let bending=norm2(w,p[1],p[2])?;
    let bending=corners(w,bending,z,Entry::Div)?;
    Ok(w.add(axial,bending)?)
}
fn split(w:&mut NumericWork,p:[[Enclosure;3];3])->Result<([[Enclosure;3];3],[[Enclosure;3];3]),ProductFailure> {
    let mut left=p;let mut right=p;
    for c in 0..3 {
        let a=midpoint(w,p[0][c],p[1][c])?;
        let b=midpoint(w,p[1][c],p[2][c])?;
        let m=midpoint(w,a,b)?;
        left[1][c]=a;left[2][c]=m;right[0][c]=m;right[1][c]=b;
    }
    Ok((left,right))
}
fn point_max(w:&mut NumericWork,p:[[Enclosure;3];3],a:Enclosure,z:Enclosure)->Result<Endpoint,ProductFailure> {
    let (left,_)=split(w,p)?;
    let mut best=Endpoint::ZERO;
    for point in [p[0],left[2],p[2]] {best=max(best,objective(w,point,a,z)?.lo);}
    Ok(best)
}
fn upper(w:&mut NumericWork,p:[[Enclosure;3];3],a:Enclosure,z:Enclosure)->Result<Endpoint,ProductFailure> {
    let mut paired=Endpoint::ZERO;
    for point in p { paired=max(paired,objective(w,point,a,z)?.hi); }
    let (left,_)=split(w,p)?;
    let midpoint=norm2(w,left[2][1],left[2][2])?;
    let rho=corners(w,midpoint,z,Entry::Div)?.hi;
    if rho.is_zero() { return Ok(paired); }
    // Product of two degree-two Bernstein polynomials, exactly weighted by
    // C(2,i)C(2,j)/C(4,k). Signed corner products enclose each coefficient.
    let mut q=[Enclosure::point(Endpoint::ZERO);5];
    let c2=[1u32,2,1];let c4=[1u32,4,6,4,1];
    for k in 0..5 {
        let mut coefficient=Enclosure::point(Endpoint::ZERO);
        for i in 0..3 { if k<i || k-i>=3 {continue;}let j=k-i;
            for c in [1usize,2] {
                let product=corners(w,p[i][c],p[j][c],Entry::Mul)?;
                let numerator=Enclosure::point(lift(f64::from(c2[i]*c2[j]))?);
                let denominator=Enclosure::point(lift(f64::from(c4[k]))?);
                let weighted=corners(w,product,numerator,Entry::Mul)?;
                let weighted=corners(w,weighted,denominator,Entry::Div)?;
                coefficient=w.add(coefficient,weighted)?;
            }
        }
        q[k]=coefficient;
    }
    let z2=corners(w,z,z,Entry::Mul)?;
    let rho2=w.scalar(Entry::Mul,&rho,&rho,Toward::Up)?;
    let denominator=Enclosure::point(shift(&rho,1)?);
    let sign_positive=p.iter().all(|p|p[0].lo.cmp_value(&Endpoint::ZERO)!=Ordering::Less);
    let sign_negative=p.iter().all(|p|p[0].hi.cmp_value(&Endpoint::ZERO)!=Ordering::Greater);
    let mut tangent=None;
    // Taking both signs is sound when N crosses zero. N is elevated from
    // degree one to four jointly with Q, rather than using unmatched controls.
    for sign in [1i8,-1] {
        // Identically zero N satisfies both signs; retain the positive branch.
        if (sign_positive&&sign<0)||(!sign_positive&&sign_negative&&sign>0) {continue;}
        let mut bound=None;
        for k in 0..5 {
            let t=Enclosure::point(lift(k as f64/4.0)?);
            let omt=Enclosure::point(lift((4-k) as f64/4.0)?);
            let ni=corners(w,p[0][0],omt,Entry::Mul)?;
            let nj=corners(w,p[2][0],t,Entry::Mul)?;
            let n=w.add(ni,nj)?;
            let n=if sign<0{negate(n)}else{n};
            let axial=corners(w,n,a,Entry::Div)?;
            let norm_squared=corners(w,q[k],z2,Entry::Div)?;
            let numerator=w.add(norm_squared,Enclosure::point(rho2))?;
            let bending=corners(w,numerator,denominator,Entry::Div)?;
            let v=w.add(axial,bending)?.hi;
            bound=Some(bound.map_or(v,|b| max(b,v)));
        }
        let bound=bound.unwrap();tangent=Some(tangent.map_or(bound,|b|max(b,bound)));
    }
    Ok(min(paired,tangent.unwrap_or(paired)))
}
fn limits()->(usize,u8,usize) {
    #[cfg(any(test, feature = "loaded-maximum-hooks"))] if let Some(l)=hooks::get(){return l;}
    (LIVE,DEPTH,SPLITS)
}
/// The loaded-maximum branch and bound (W1 Design §4.4). `driven` (W1 coverage
/// repair §1.2) is set only for a lane whose Z is the represented hull
/// hull(Ẑ, Î/c) and not a point: the search then runs on the point problem at
/// z_lo, where the field is largest, and its convergence is decided there by the
/// unchanged stop rule and limits. Beside the driving lower it keeps
/// `lower_hull`, the best evaluated point's objective over the whole hull, whose
/// lower end is the value at z_hi. The lane returns [lower_hull, U]: U bounds the
/// maximum at z_lo and so over the hull; lower_hull is a lower bound of the
/// maximum at every Z of the hull. Undriven, this is today's search.
#[allow(clippy::too_many_arguments)]
pub(super) fn search(spent:&mut ProductCertificateSpent<'_>,row:usize,member:u32,
    ends:[[Enclosure;3];2],q:[Enclosure;2],a:Enclosure,z:Enclosure,driven:bool)->Result<Enclosure,ProductFailure> {
    let (live_limit,depth_limit,split_limit)=limits();
    #[cfg(feature = "census-probe")]
    let work_start=census_probe::work_counts(spent);
    let hull_z=z;
    let z=if driven {Enclosure::point(z.lo)} else {z};
    let mut controls=[ends[0];3];controls[2]=ends[1];
    for c in 0..3 {
        let sum=spent.numeric.add(ends[0][c],ends[1][c])?;
        let sum=if c==0{sum}else{spent.numeric.add(sum,q[c-1])?};
        controls[1][c]=half(sum)?;
    }
    let mut lower=point_max(&mut spent.numeric,controls,a,z)?;
    let mut lower_hull=if driven {point_max(&mut spent.numeric,controls,a,hull_z)?} else {lower};
    #[cfg(feature = "census-probe")]
    let mut probe=census_probe::SearchProbe::start(row,member,driven,hull_z,controls,lower,work_start);
    let first=SearchNode{controls,upper:upper(&mut spent.numeric,controls,a,z)?,depth:0};
    let mut live=reserve::<SearchNode>(live_limit)?;
    spent.loaded_search_bytes=spent.loaded_search_bytes.max(live.capacity()*std::mem::size_of::<SearchNode>());
    live.push(first);let mut splits=0usize;
    loop {
        spent.visit()?;
        let mut highest=0;
        for i in 1..live.len(){spent.visit()?;if live[i].upper.cmp_value(&live[highest].upper)==Ordering::Greater{highest=i;}}
        let upper=live[highest].upper;
        let gap=spent.numeric.scalar(Entry::Sub,&upper,&lower,Toward::Up)?;
        let allowance=shift(&lower,-64)?;
        if gap.cmp_value(&allowance)!=Ordering::Greater {
            #[cfg(feature = "census-probe")]
            probe.finish("converged",splits,lower,lower_hull,upper,a,census_probe::work_counts(spent));
            return Ok(Enclosure{lo:if driven {lower_hull} else {lower},hi:upper});
        }
        let parent=live[highest];
        if parent.depth>=depth_limit || splits>=split_limit || live.len()>=live_limit {
            #[cfg(feature = "census-probe")]
            probe.finish("exhausted",splits,lower,lower_hull,upper,a,census_probe::work_counts(spent));
            spent.loaded_diagnostic=Some(Box::new(LoadedMaximumDiagnostic{row,member,lower:(if driven {lower_hull} else {lower}).parts(),upper:upper.parts()}));
            return Err(bad("loaded maximum search exhausted"));
        }
        live.swap_remove(highest);
        let (left,right)=split(&mut spent.numeric,parent.controls)?;
        for controls in [left,right] {
            spent.visit()?;
            let point=point_max(&mut spent.numeric,controls,a,z)?;
            #[cfg(feature = "census-probe")]
            probe.point(controls,point,lower,parent.depth+1);
            lower=max(lower,point);
            if driven {lower_hull=max(lower_hull,point_max(&mut spent.numeric,controls,a,hull_z)?);}
            let bound=upper_bound(spent,controls,a,z)?;
            live.push(SearchNode{controls,upper:bound,depth:parent.depth+1});
        }
        splits+=1;
        #[cfg(feature = "census-probe")]
        probe.live(live.len());
        // Pruning keeps the lower bound's point live through its containing
        // node, and leaves at least one maximum-upper node for termination.
        let best=live.iter().map(|n|n.upper).max_by(|a,b|a.cmp_value(b)).unwrap();
        live.retain(|n|n.upper.cmp_value(&lower)==Ordering::Greater || n.upper.cmp_value(&best)==Ordering::Equal);
    }
}
#[cfg(feature = "census-probe")]
pub(super) fn probe_parts(p:[[Enclosure;3];3],a:Enclosure,z:Enclosure)->Option<(Endpoint,Endpoint)> {
    // The best control point's axial and bending lower ends, in a scratch owner.
    let mut w=NumericWork::new();
    let (left,_)=split(&mut w,p).ok()?;
    let mut best:Option<(Endpoint,Endpoint,Endpoint)>=None;
    for point in [p[0],left[2],p[2]] {
        let axial=corners(&mut w,abs_range(point[0]),a,Entry::Div).ok()?;
        let bending=norm2(&mut w,point[1],point[2]).ok()?;
        let bending=corners(&mut w,bending,z,Entry::Div).ok()?;
        let v=w.add(axial,bending).ok()?.lo;
        if best.as_ref().is_none_or(|b|v.cmp_value(&b.0)==Ordering::Greater) {best=Some((v,axial.lo,bending.lo));}
    }
    best.map(|b|(b.1,b.2))
}
fn upper_bound(spent:&mut ProductCertificateSpent<'_>,p:[[Enclosure;3];3],a:Enclosure,z:Enclosure)->Result<Endpoint,ProductFailure>{upper(&mut spent.numeric,p,a,z)}

/// Exact endpoint-grid construction. All expressions are unbounded-exponent
/// dyadics; directed1024 operations preserve every binary64 rounding boundary.
/// No common-denominator shift, binary64 overflow, or post-completion work.
pub(super) fn evidence(w:&mut NumericWork,n:f64,hull:Enclosure,pp_lower:f64,pp_upper:f64,visits:&mut WorkTotal)
    ->Result<ProductMaximumEvidence,ProductFailure> {
    values_visit(visits)?;
    if !n.is_finite()||n<0.0||!pp_lower.is_finite()||!pp_upper.is_finite()
        || pp_lower<0.0||pp_lower>pp_upper {return Err(bad("loaded maximum evidence unavailable"));}
    let lower=min(hull.lo,lift(pp_lower)?);let upper=max(hull.hi,lift(pp_upper)?);
    if n>0.0 {
        let nw=lift(n)?;let twice=shift(&nw,1)?;
        let reflected=w.scalar(Entry::Sub,&twice,&lower,Toward::Up)?;
        let ulp=Endpoint::ONE.mul_pow2((nw.exponent()-52).max(-1074)).map_err(AttemptStop::from).map_err(NumericError::from)?;
        let minimum=w.scalar(Entry::Add,&nw,&ulp,Toward::Up)?;
        let threshold=max(upper,max(reflected,minimum));
        let candidate=match w.binary64_up(&threshold) {
            Ok(hi)=>Some(hi),Err(NumericError::Binary64Range)=>None,Err(e)=>return Err(e.into()),
        };
        if let Some(hi)=candidate {
            if lift(hi)?.cmp_value(&twice)!=Ordering::Greater {
                let lo=w.scalar(Entry::Sub,&twice,&lift(hi)?,Toward::Down)?;
                values_visit(visits)?;
                let lo=lo.to_binary64().value().ok_or_else(||bad("loaded maximum evidence unavailable"))?;
                let gap=w.scalar(Entry::Sub,&lift(hi)?,&lift(lo)?,Toward::Up)?;
                values_visit(visits)?;
                let gap=gap.to_binary64().value().ok_or_else(||bad("loaded maximum evidence unavailable"))?;
                return Ok(ProductMaximumEvidence{value:n,lower:lo,upper:hi,gap});
            }
        }
    }
    let mut hi=match w.binary64_up(&upper) {Ok(hi)=>hi,Err(NumericError::Binary64Range)=>return Err(bad("loaded maximum evidence unavailable")),Err(e)=>return Err(e.into())};
    // Odd subnormals and odd low-normal significands lose a bit when halved.
    // A single successor reaches an exactly representable half.
    values_visit(visits)?;
    let bits=hi.to_bits();
    if bits&1!=0 && bits<0x0020_0000_0000_0000 {hi=f64::from_bits(bits+1);}
    if !hi.is_finite(){return Err(bad("loaded maximum evidence unavailable"));}
    let half=shift(&lift(hi)?,-1)?;
    values_visit(visits)?;
    let value=half.to_binary64().value().ok_or_else(||bad("loaded maximum evidence unavailable"))?;
    if lift(value)?.cmp_value(&half)!=Ordering::Equal {return Err(bad("loaded maximum evidence unavailable"));}
    Ok(ProductMaximumEvidence{value,lower:0.0,upper:hi,gap:hi})
}

/// The search's test capacity hook: `(live, depth, splits)` in place of the
/// production limits, on the calling thread only. Compiled in the kernel's own
/// tests and under the opt-in `loaded-maximum-hooks` feature (W1 coverage repair
/// §1.4, option 2), which no product build or `[dev-dependencies]` entry enables.
#[cfg(any(test, feature = "loaded-maximum-hooks"))]
pub mod hooks {
    use std::cell::Cell;
    thread_local! {static LIMIT:Cell<Option<(usize,u8,usize)>>=const{Cell::new(None)};}
    pub(super) fn get()->Option<(usize,u8,usize)>{LIMIT.with(Cell::get)}
    /// Runs `f` with the search limits `(live, depth, splits)` on this thread.
    pub fn with<T>(value:(usize,u8,usize),f:impl FnOnce()->T)->T {
        struct Restore(Option<(usize,u8,usize)>);impl Drop for Restore{fn drop(&mut self){LIMIT.with(|l|l.set(self.0));}}
        let old=LIMIT.with(|l|l.replace(Some(value)));let _restore=Restore(old);f()
    }
}

#[cfg(test)]
#[path = "../../../../tests/retained_k4/loaded_maximum_tests.rs"]
mod tests;
