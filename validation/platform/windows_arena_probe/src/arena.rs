//! Controlled arena experiment; never installed as a global allocator.
use crate::ledger::Failure;
use std::{ffi::c_void, mem::size_of, ptr};
const RESERVE: u32 = 0x2000;
const COMMIT: u32 = 0x1000;
const DECOMMIT: u32 = 0x4000;
const RELEASE: u32 = 0x8000;
const RW: u32 = 4;
const SLOTS: usize = 256;
const MAX_PAGES: usize = 2048;
#[repr(C)]
struct SystemInfo { arch: u16, reserved: u16, page: u32, min: *mut c_void, max: *mut c_void, mask: usize, processors: u32, kind: u32, granularity: u32, level: u16, revision: u16 }
#[link(name="kernel32")]
unsafe extern "system" {
    fn GetSystemInfo(info: *mut SystemInfo);
    fn VirtualAlloc(addr: *mut c_void, size: usize, kind: u32, protect: u32) -> *mut c_void;
    fn VirtualFree(addr: *mut c_void, size: usize, kind: u32) -> i32;
    fn GetLastError() -> u32;
}
fn align(n: usize, a: usize) -> Result<usize, Failure> {
    if !a.is_power_of_two() { return Err(Failure::Limit); }
    n.checked_add(a-1).map(|x| x & !(a-1)).ok_or(Failure::Limit)
}
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct Slot { offset: usize, size: usize, capacity: usize, live: usize }
fn sizing(page: usize, granularity: usize, slots: usize) -> Result<(usize, usize, usize), Failure> {
    if !page.is_power_of_two() || !granularity.is_power_of_two() || granularity < page || granularity % page != 0 { return Err(Failure::Limit); }
    let size = align(4 * 1024 * 1024, granularity)?;
    let metadata = align(slots.checked_mul(size_of::<Slot>()).ok_or(Failure::Limit)?, page)?;
    let control = align(size_of::<Arena>(), page)?;
    if metadata > size || size / page > MAX_PAGES { return Err(Failure::Limit); }
    Ok((size, metadata, control))
}
pub struct Arena {
    base: *mut u8, page: usize, granularity: usize, size: usize, metadata: usize, control: usize,
    used: usize, committed: [bool; MAX_PAGES], authorized: [bool; MAX_PAGES], calls: usize,
}
impl Arena {
    pub fn new(admit: &mut impl FnMut(usize) -> Result<(), Failure>, before_commit: &mut impl FnMut() -> Result<(), Failure>) -> Result<Self, Failure> {
        let mut info: SystemInfo = unsafe { std::mem::zeroed() };
        unsafe { GetSystemInfo(&mut info) };
        let page = info.page as usize; let granularity = info.granularity as usize;
        let (size, metadata, control) = sizing(page, granularity, SLOTS)?;
        // Fixed control arrays live on the stack; reserve their rounded size too.
        admit(control)?;
        let base = unsafe { VirtualAlloc(ptr::null_mut(), size, RESERVE, RW) }.cast::<u8>();
        if base.is_null() { return Err(Failure::Address); }
        let mut arena = Self { base, page, granularity, size, metadata, control, used: metadata, committed: [false; MAX_PAGES], authorized: [false; MAX_PAGES], calls: 1 };
        arena.commit(0, metadata, admit, before_commit)?;
        Ok(arena)
    }
    fn slots(&self) -> *mut Slot { self.base.cast() }
    fn commit(&mut self, start: usize, end: usize, admit: &mut impl FnMut(usize) -> Result<(), Failure>, before_commit: &mut impl FnMut() -> Result<(), Failure>) -> Result<(), Failure> {
        let first = start / self.page; let last = align(end, self.page)? / self.page;
        if last > self.size / self.page { return Err(Failure::Limit); }
        for i in first..last {
            if !self.committed[i] {
                if !self.authorized[i] { admit(self.page)?; self.authorized[i] = true; }
                // A failed commit retains the existing authority; retry never
                // asks for the same page twice. Guard also checks cancellation.
                before_commit()?;
                assert!(self.authorized[i]);
                self.calls += 1;
                let p = unsafe { VirtualAlloc(self.base.add(i*self.page).cast(), self.page, COMMIT, RW) };
                if p.is_null() { return Err(Failure::OsCommit(unsafe { GetLastError() })); }
                self.committed[i] = true;
            }
        }
        Ok(())
    }
    pub fn alloc(&mut self, size: usize, alignment: usize, admit: &mut impl FnMut(usize) -> Result<(), Failure>, before_commit: &mut impl FnMut() -> Result<(), Failure>) -> Result<usize, Failure> {
        if size == 0 || !alignment.is_power_of_two() { return Err(Failure::Limit); }
        let mut free_index = None;
        for i in 0..SLOTS {
            let s = unsafe { self.slots().add(i).read() };
            if s.live == 0 && s.capacity >= size && (self.base as usize + s.offset) % alignment == 0 {
                before_commit()?; // cancellation wins before reuse too
                unsafe { ptr::write_bytes(self.base.add(s.offset), 0, size); self.slots().add(i).write(Slot { size, live: 1, ..s }); }
                return Ok(i);
            }
            if s.live == 0 && s.capacity == 0 && free_index.is_none() { free_index = Some(i); }
        }
        let i = free_index.ok_or(Failure::Limit)?;
        let address = align((self.base as usize).checked_add(self.used).ok_or(Failure::Limit)?, alignment)?;
        let start = address.checked_sub(self.base as usize).ok_or(Failure::Limit)?;
        let end = start.checked_add(size).ok_or(Failure::Limit)?;
        if end > self.size { return Err(Failure::Limit); }
        self.commit(start, end, admit, before_commit)?;
        unsafe { ptr::write_bytes(self.base.add(start), 0, size); self.slots().add(i).write(Slot { offset: start, size, capacity: size, live: 1 }); }
        self.used = end; Ok(i)
    }
    pub fn free(&mut self, i: usize) { let mut s = unsafe { self.slots().add(i).read() }; assert_eq!(s.live, 1); s.live = 0; unsafe { self.slots().add(i).write(s); } }
    pub fn realloc(&mut self, old: usize, size: usize, alignment: usize, admit: &mut impl FnMut(usize) -> Result<(), Failure>, before_commit: &mut impl FnMut() -> Result<(), Failure>) -> Result<usize, Failure> {
        let s = unsafe { self.slots().add(old).read() }; assert_eq!(s.live, 1);
        let new = self.alloc(size, alignment, admit, before_commit)?;
        let n = unsafe { self.slots().add(new).read() };
        assert_ne!(new, old);
        assert!(s.offset+s.size <= n.offset || n.offset+n.size <= s.offset);
        assert!(self.stats().0 >= s.size.checked_add(n.size).expect("overlap overflow"));
        assert!(self.committed_bytes() <= self.authorized_bytes());
        println!("moving overlap requested,capacity,data,metadata,reserved_span_unused,syscalls={:?} retained_pages={} authorized={}", self.stats(), self.retained_pages(), self.authorized_bytes());
        unsafe { ptr::copy_nonoverlapping(self.base.add(s.offset), self.base.add(n.offset), s.size.min(size)); }
        self.free(old); Ok(new)
    }
    pub fn byte(&self, i: usize, n: usize) -> *mut u8 { let s = unsafe { self.slots().add(i).read() }; assert!(s.live == 1 && n < s.size); unsafe { self.base.add(s.offset+n) } }
    pub fn decommit_free(&mut self, inject: bool) -> Result<(), Failure> {
        for i in 0..SLOTS { if unsafe { self.slots().add(i).read() }.live != 0 { return Err(Failure::Limit); } }
        if inject { return Err(Failure::SyntheticDecommit); }
        self.calls += 1;
        if unsafe { VirtualFree(self.base.add(self.metadata).cast(), self.size - self.metadata, DECOMMIT) } == 0 { return Err(Failure::OsDecommit(unsafe { GetLastError() })); }
        for p in self.metadata/self.page..self.size/self.page { self.committed[p] = false; }
        // Authority is intentionally retained even after decommit until exit.
        unsafe { ptr::write_bytes(self.slots(), 0, SLOTS); }
        self.used = self.metadata; Ok(())
    }
    pub fn committed_bytes(&self) -> usize { self.committed.iter().filter(|&&x| x).count() * self.page }
    pub fn authorized_bytes(&self) -> usize { self.control + self.authorized.iter().filter(|&&x| x).count() * self.page }
    pub fn retained_pages(&self) -> usize {
        let mut retained = 0;
        for page in self.metadata/self.page..self.size/self.page {
            if !self.committed[page] { continue; }
            let start = page*self.page; let end = start+self.page;
            let occupied = (0..SLOTS).any(|i| { let s = unsafe { self.slots().add(i).read() }; s.live != 0 && s.offset < end && s.offset+s.size > start });
            if !occupied { retained += self.page; }
        }
        retained
    }
    pub fn stats(&self) -> (usize, usize, usize, usize, usize, usize) {
        let mut requested = 0; let mut live_capacity = 0;
        for i in 0..SLOTS { let s = unsafe { self.slots().add(i).read() }; if s.live != 0 { requested += s.size; live_capacity += s.capacity; } }
        (requested, live_capacity, self.committed_bytes() - self.metadata, self.metadata, self.used - self.metadata - live_capacity, self.calls)
    }
}
impl Drop for Arena { fn drop(&mut self) { if unsafe { VirtualFree(self.base.cast(), 0, RELEASE) } == 0 { std::process::abort(); } } }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    #[test]
    fn native_arena() {
        assert_eq!(align(usize::MAX, 4096), Err(Failure::Limit));
        assert_eq!(align(5, 3), Err(Failure::Limit));
        assert_eq!(sizing(4096, 65536, usize::MAX), Err(Failure::Limit));
        assert_eq!(sizing(65536, 4096, SLOTS), Err(Failure::Limit));
        let charged = Cell::new(0usize); let mut admit = |n| { charged.set(charged.get()+n); Ok(()) };
        let mut okay = || Ok(());
        let mut a = Arena::new(&mut admit, &mut okay).unwrap();
        println!("page={} granularity={} reserve={} metadata={} control={}", a.page,a.granularity,a.size,a.metadata,a.control);
        assert_eq!(a.alloc(usize::MAX, 8, &mut admit, &mut okay), Err(Failure::Limit));
        assert!(a.alloc(1, 1usize << (usize::BITS-1), &mut admit, &mut okay).is_err());
        assert_eq!(a.alloc(1, 3, &mut admit, &mut okay), Err(Failure::Limit));
        let mut ids = Vec::new();
        for _ in 0..128 { let id = a.alloc(24, 8, &mut admit, &mut okay).unwrap(); assert_eq!(unsafe { *a.byte(id,0) }, 0); ids.push(id); }
        let packed = a.stats();
        println!("packed committed_slack_bytes={} (distinct from reserved_span_unused_bytes={})", packed.2 - packed.1, packed.4);
        println!("packed requested,capacity,data,metadata,reserved_span_unused,syscalls={packed:?}; per-allocation-pages={}",128*a.page);
        assert!(packed.2 < 128*a.page);
        unsafe { *a.byte(ids[0], 0) = 77; } a.free(ids[0]);
        let reused = a.alloc(24, 8, &mut admit, &mut okay).unwrap(); assert_eq!(unsafe { *a.byte(reused,0) }, 0);
        for id in ids.into_iter().skip(1) { a.free(id); } a.free(reused);
        let big = a.alloc(256*1024, 256*1024, &mut admit, &mut okay).unwrap();
        assert_eq!(a.byte(big,0) as usize % (256*1024), 0);
        unsafe { *a.byte(big,0) = 91; }
        let before = a.stats(); let mut failed = false;
        let mut inject = || { if !failed { failed = true; Err(Failure::SyntheticCommitHeadroom) } else { Ok(()) } };
        assert_eq!(a.realloc(big, 512*1024, 256*1024, &mut admit, &mut inject), Err(Failure::SyntheticCommitHeadroom));
        assert_eq!(unsafe { *a.byte(big,0) }, 91);
        let held_after_fault = charged.get(); let authority_after_fault = a.authorized_bytes();
        let mut fail_again = || Err(Failure::SyntheticCommitHeadroom);
        assert_eq!(a.realloc(big, 512*1024, 256*1024, &mut admit, &mut fail_again), Err(Failure::SyntheticCommitHeadroom));
        assert_eq!(charged.get(), held_after_fault); assert_eq!(a.authorized_bytes(), authority_after_fault);
        assert_eq!(unsafe { *a.byte(big,0) }, 91);
        let moved = a.realloc(big, 512*1024, 256*1024, &mut admit, &mut inject).unwrap();
        assert_eq!(unsafe { *a.byte(moved,0) }, 91);
        assert_eq!(charged.get(), a.authorized_bytes()); assert!(charged.get() >= held_after_fault);
        assert!(a.committed_bytes() <= charged.get()); println!("churn before={before:?} moving={:?}",a.stats());
        a.free(moved); let retained = a.stats(); let authority = a.authorized_bytes();
        assert_eq!(a.retained_pages(), retained.2); assert_eq!(a.decommit_free(true), Err(Failure::SyntheticDecommit));
        assert_eq!(a.stats(), retained); assert_eq!(a.authorized_bytes(), authority);
        a.decommit_free(false).unwrap(); assert_eq!(a.stats().2, 0); assert_eq!(a.authorized_bytes(), authority);
        println!("retained={retained:?} after_decommit={:?} held={}",a.stats(),charged.get());
    }
}
