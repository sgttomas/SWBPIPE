//! Bounded on-disk PE32+ parser. No inference from Cargo dependencies.
//! https://learn.microsoft.com/en-us/windows/win32/debug/pe-format
use std::collections::BTreeSet;
type Result<T> = std::result::Result<T, &'static str>;
fn range(b: &[u8], at: usize, len: usize) -> Result<&[u8]> {
    b.get(at..at.checked_add(len).ok_or("offset overflow")?).ok_or("truncated PE")
}
fn u16at(b: &[u8], at: usize) -> Result<u16> { Ok(u16::from_le_bytes(range(b,at,2)?.try_into().unwrap())) }
fn u32at(b: &[u8], at: usize) -> Result<u32> { Ok(u32::from_le_bytes(range(b,at,4)?.try_into().unwrap())) }
fn u64at(b: &[u8], at: usize) -> Result<u64> { Ok(u64::from_le_bytes(range(b,at,8)?.try_into().unwrap())) }
#[derive(Clone)]
struct Section { va: u32, span: u32, raw: u32, size: u32 }
struct Image<'a> { bytes: &'a [u8], headers: u32, sections: Vec<Section> }
impl Image<'_> {
    fn rva(&self, rva: u32, len: usize) -> Result<usize> {
        let end = u64::from(rva).checked_add(len as u64).ok_or("RVA overflow")?;
        if end > u32::MAX as u64 { return Err("RVA overflow"); }
        if rva < self.headers && end <= self.headers as u64 { range(self.bytes,rva as usize,len)?; return Ok(rva as usize); }
        let mut found = None;
        for s in &self.sections {
            if rva >= s.va && end <= s.va as u64 + s.size as u64 {
                let off = s.raw as u64 + (rva-s.va) as u64;
                range(self.bytes,off as usize,len)?;
                if found.replace(off as usize).is_some() { return Err("ambiguous RVA"); }
            }
        }
        found.ok_or("unbacked RVA")
    }
    fn text(&self, rva: u32) -> Result<String> {
        let mut bytes = Vec::new();
        for i in 0..260u32 {
            let at = self.rva(rva.checked_add(i).ok_or("name RVA overflow")?,1)?;
            let b = self.bytes[at];
            if b == 0 { if bytes.is_empty() { return Err("empty import name"); } return String::from_utf8(bytes).map_err(|_| "import UTF8"); }
            if !b.is_ascii_alphanumeric() && !b"._-?@$".contains(&b) { return Err("unsafe import name"); }
            bytes.push(b);
        }
        Err("unterminated import name")
    }
    fn thunks(&self, rva: u32, dll: &str, kind: &str, out: &mut BTreeSet<String>) -> Result<()> {
        for i in 0..65536u32 {
            let at = self.rva(rva.checked_add(i.checked_mul(8).ok_or("thunk overflow")?).ok_or("thunk overflow")?,8)?;
            let value = u64at(self.bytes,at)?;
            if value == 0 { return Ok(()); }
            let name = if value & (1<<63) != 0 {
                if value & 0x7fff_ffff_ffff_0000 != 0 { return Err("invalid import ordinal"); }
                format!("ordinal_{}",value & 0xffff)
            } else {
                let ptr = u32::try_from(value).map_err(|_| "import RVA overflow")?;
                self.rva(ptr,2)?;
                self.text(ptr.checked_add(2).ok_or("import hint overflow")?)?
            };
            out.insert(format!("{kind}={dll}:{name}"));
            if out.len() > 65536 { return Err("total import cap"); }
        }
        Err("unterminated thunk table")
    }
}
#[derive(Debug)]
pub struct Facts { pub stack_reserve: u64, pub stack_commit: u64, pub image_size: u32, pub headers_size: u32, pub file_size: usize, pub imports: BTreeSet<String> }
pub fn parse(b: &[u8]) -> Result<Facts> {
    if b.len() > 256*1024*1024 || range(b,0,2)? != b"MZ" { return Err("not bounded DOS image"); }
    let pe = u32at(b,0x3c)? as usize;
    if pe < 64 || range(b,pe,4)? != b"PE\0\0" { return Err("invalid PE signature"); }
    if u16at(b,pe+4)? != 0x8664 { return Err("not AMD64"); }
    let count = u16at(b,pe+6)? as usize;
    if count == 0 || count > 96 { return Err("section count"); }
    let optional_size = u16at(b,pe+20)? as usize; let opt=pe+24;
    range(b,opt,optional_size)?;
    if optional_size < 112 || u16at(b,opt)? != 0x20b { return Err("not PE32+"); }
    let directories=u32at(b,opt+108)? as usize;
    if directories > 16 || optional_size < 112+8*directories { return Err("directory bounds"); }
    let image_size=u32at(b,opt+56)?; let headers=u32at(b,opt+60)?;
    let stack_reserve=u64at(b,opt+72)?; let stack_commit=u64at(b,opt+80)?;
    if image_size==0 || headers==0 || headers>image_size || stack_commit>stack_reserve { return Err("invalid image/stack sizes"); }
    range(b,0,headers as usize)?;
    let section_start=opt+optional_size;
    let table_end=section_start.checked_add(count*40).ok_or("section table overflow")?;
    if table_end>headers as usize { return Err("section table outside headers"); }
    let mut sections: Vec<Section>=Vec::new();
    for i in 0..count {
        let at=section_start+i*40;
        let size=u32at(b,at+16)?; let raw=u32at(b,at+20)?; let va=u32at(b,at+12)?;
        let span=u32at(b,at+8)?.max(size);
        if va as u64 + span as u64>image_size as u64 || va<headers { return Err("section image bounds"); }
        if size>0 { if raw<headers { return Err("raw section overlaps headers"); } range(b,raw as usize,size as usize)?; }
        for s in &sections {
            if (va as u64)<s.va as u64+s.span as u64 && (s.va as u64)<va as u64+span as u64 { return Err("overlapping virtual sections"); }
            if size>0 && s.size>0 && (raw as u64)<s.raw as u64+s.size as u64 && (s.raw as u64)<raw as u64+size as u64 { return Err("overlapping raw sections"); }
        }
        sections.push(Section {va,span,raw,size});
    }
    let image=Image {bytes:b,headers,sections}; let mut imports=BTreeSet::new();
    for (index,width,kind) in [(1,20,"import"),(13,32,"delayimport")] {
        if index>=directories { continue; }
        let d=opt+112+index*8; let start=u32at(b,d)?; let length=u32at(b,d+4)? as usize;
        if start==0 && length==0 { continue; }
        if start==0 || length<width || length>1024*1024 { return Err("import directory size"); }
        image.rva(start,length)?; let mut terminated=false;
        for offset in (0..length-width+1).step_by(width).take(4096) {
            let rva=start.checked_add(offset as u32).ok_or("directory RVA overflow")?; let at=image.rva(rva,width)?;
            if range(b,at,width)?.iter().all(|v| *v==0) { terminated=true; break; }
            let (name,thunk)=if index==1 { let lookup=u32at(b,at)?; (u32at(b,at+12)?,if lookup!=0 {lookup} else {u32at(b,at+16)?}) }
                else { if u32at(b,at)? != 1 { return Err("delay import must use RVAs"); } (u32at(b,at+4)?,u32at(b,at+16)?) };
            if thunk==0 { return Err("missing import lookup"); }
            let dll=image.text(name)?;
            imports.insert(format!("{kind}_dll={dll}"));
            image.thunks(thunk,&dll,kind,&mut imports)?;
        }
        if !terminated { return Err("unterminated import descriptors"); }
    }
    Ok(Facts { stack_reserve,stack_commit,image_size,headers_size:headers,file_size:b.len(),imports })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn put16(b:&mut[u8],at:usize,v:u16){b[at..at+2].copy_from_slice(&v.to_le_bytes());}
    fn put32(b:&mut[u8],at:usize,v:u32){b[at..at+4].copy_from_slice(&v.to_le_bytes());}
    fn valid()->Vec<u8>{let mut b=vec![0;1024]; b[..2].copy_from_slice(b"MZ");put32(&mut b,60,64);b[64..68].copy_from_slice(b"PE\0\0");put16(&mut b,68,0x8664);put16(&mut b,70,1);put16(&mut b,84,240);put16(&mut b,88,0x20b);put32(&mut b,144,8192);put32(&mut b,148,512);put32(&mut b,196,16);put32(&mut b,336,512);put32(&mut b,340,4096);put32(&mut b,344,512);put32(&mut b,348,512);b}
    #[test] fn minimal_pe(){assert!(parse(&valid()).is_ok());}
    #[test] fn empty(){assert!(parse(&[]).is_err());}
    #[test] fn every_truncated_prefix(){let b=valid();for n in 0..b.len(){assert!(parse(&b[..n]).is_err(),"prefix {n}");}}
    #[test] fn wrong_machine(){let mut b=valid();put16(&mut b,68,0x14c);assert!(parse(&b).is_err());}
    #[test] fn overflowing_header(){let mut b=valid();put32(&mut b,60,u32::MAX);assert!(parse(&b).is_err());}
    #[test] fn unbacked_import(){let mut b=valid();put32(&mut b,208,0xfffffff0);put32(&mut b,212,40);assert!(parse(&b).is_err());}
    #[test] fn unterminated_import(){let mut b=valid();put32(&mut b,208,4096);put32(&mut b,212,20);b[512..532].fill(1);assert!(parse(&b).is_err());}
    #[test] fn bad_directory_count(){let mut b=valid();put32(&mut b,196,u32::MAX);assert!(parse(&b).is_err());}
    #[test] fn raw_bounds(){let mut b=valid();put32(&mut b,348,u32::MAX);assert!(parse(&b).is_err());}
}
