//! K5 (G): memory of `assess_constrained_bodies` on a tie chain of 10,000
//! sub-bodies (20,000 nodes), in its own test binary, under a capped counting
//! global allocator. The cap (256 MiB) makes a regression abort this binary
//! instead of exhausting the host; such an abort is a failure, not a kill.
//! The reduction keeps memory linear in the input: no 6S×6S (or n×n) map is
//! formed. The peak heap of one assessment must stay under 32 MiB, and the
//! peak must grow at most linearly between 1,000 and 10,000 sub-bodies.
use open_pipe_stress_frame_kernel::rigid_body::{
    assess_constrained_bodies, ConstrainedGround, RigidBodyStatus,
};
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

const CAP: usize = 256 << 20;
static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static LARGEST: AtomicUsize = AtomicUsize::new(0);

struct Capped;

unsafe impl GlobalAlloc for Capped {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let now = CURRENT.fetch_add(size, SeqCst) + size;
        if now > CAP {
            CURRENT.fetch_sub(size, SeqCst);
            return std::ptr::null_mut();
        }
        PEAK.fetch_max(now, SeqCst);
        LARGEST.fetch_max(size, SeqCst);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        CURRENT.fetch_sub(layout.size(), SeqCst);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old = layout.size();
        if new_size > old {
            let now = CURRENT.fetch_add(new_size - old, SeqCst) + (new_size - old);
            if now > CAP {
                CURRENT.fetch_sub(new_size - old, SeqCst);
                return std::ptr::null_mut();
            }
            PEAK.fetch_max(now, SeqCst);
        } else {
            CURRENT.fetch_sub(old - new_size, SeqCst);
        }
        LARGEST.fetch_max(new_size, SeqCst);
        System.realloc(ptr, layout, new_size)
    }
}

#[global_allocator]
static ALLOCATOR: Capped = Capped;

type Chain = (
    Vec<[f64; 3]>,
    Vec<Vec<usize>>,
    Vec<[usize; 2]>,
    Vec<ConstrainedGround>,
);

/// Sub-body i: nodes 2i at (2i, y_i, 0) and 2i+1 at (2i+1, y_i, 0), with
/// y_i = i mod 5; tie (2i+1, 2i+2). Every virtual position lies on the x axis
/// (v(2i) = (i, 0, 0)). `restrained`: all six DOFs grounded at both ends;
/// otherwise translation pins at both ends, so the rotation about the x axis
/// is free.
fn chain(sub_bodies: usize, restrained: bool) -> Chain {
    let mut coordinates = Vec::with_capacity(2 * sub_bodies);
    for i in 0..sub_bodies {
        let y = (i % 5) as f64;
        coordinates.push([(2 * i) as f64, y, 0.0]);
        coordinates.push([(2 * i + 1) as f64, y, 0.0]);
    }
    let bodies = (0..sub_bodies).map(|i| vec![2 * i, 2 * i + 1]).collect();
    let ties = (0..sub_bodies - 1)
        .map(|i| [2 * i + 1, 2 * i + 2])
        .collect();
    let last = 2 * sub_bodies - 1;
    let dofs = if restrained { 6 } else { 3 };
    let grounds = (0..dofs)
        .map(ConstrainedGround::Dof)
        .chain((0..dofs).map(|k| ConstrainedGround::Dof(6 * last + k)))
        .collect();
    (coordinates, bodies, ties, grounds)
}

/// Peak heap bytes (above the level before the call) and the largest single
/// allocation, of one assessment; checks its outcome.
fn measure(sub_bodies: usize, restrained: bool) -> (usize, usize) {
    let (coordinates, bodies, ties, grounds) = chain(sub_bodies, restrained);
    let base = CURRENT.load(SeqCst);
    PEAK.store(base, SeqCst);
    LARGEST.store(0, SeqCst);
    let start = std::time::Instant::now();
    let result = assess_constrained_bodies(&coordinates, &bodies, &ties, &grounds).unwrap();
    let seconds = start.elapsed().as_secs_f64();
    let peak = PEAK.load(SeqCst) - base;
    let largest = LARGEST.load(SeqCst);
    if restrained {
        assert_eq!(result.status, RigidBodyStatus::Restrained);
    } else {
        assert_eq!(result.status, RigidBodyStatus::MechanismWitnessed);
        let motion = result.node_motion.as_ref().unwrap();
        assert_eq!(motion.len(), 2 * sub_bodies);
        for m in motion {
            assert_eq!(*m, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        }
    }
    eprintln!(
        "k5 scale: sub_bodies={sub_bodies} restrained={restrained} peak_heap_bytes={peak} largest_allocation={largest} debug_seconds={seconds:.3}"
    );
    drop(result);
    (peak, largest)
}

/// G: one test function, so no other test allocates concurrently.
#[test]
fn k5_g_ten_thousand_sub_body_tie_chain_stays_linear_in_memory() {
    for restrained in [true, false] {
        let (small, _) = measure(1_000, restrained);
        let (large, largest) = measure(10_000, restrained);
        assert!(large <= 32 << 20, "peak {large} bytes");
        assert!(largest <= 64 << 20, "largest allocation {largest} bytes");
        assert!(
            large <= 12 * small,
            "peak {large} against {small}: not linear"
        );
    }
}
