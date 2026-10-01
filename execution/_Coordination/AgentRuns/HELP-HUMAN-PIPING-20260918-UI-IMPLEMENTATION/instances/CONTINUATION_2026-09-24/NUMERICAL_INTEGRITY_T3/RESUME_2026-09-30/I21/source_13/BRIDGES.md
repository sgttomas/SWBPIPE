# Source13 — two public synchronization bridges

TASK I21; actual start2026-10-01 15:32:55 UTC; deadline15:42:55 UTC. Additive correction to sealed source12; no source12 edit. Installed Rust1.97.1/aarch64-apple-darwin source only. Same RV30 backcheck remains required before dependent reliance.

The intermediate re-exports are explicit: sync/poison.rs:67-68 exports mutex::{Mutex,MutexGuard}; sys/sync/mod.rs:9-10 exports mutex::Mutex and once::{Once,OnceState}. These are the previously supplied target selector modules, so no same-named backend is silently substituted.

Public Mutex<()> resolves to std/sync/poison/mutex.rs through std/sync/mod.rs:231-240. Its fields:227-230 are inline sys::Mutex, poison::Flag and UnsafeCell<T>. With T=(), there is no owned user payload. new():350-351 embeds sys::Mutex::new() and Flag::new(); lock():490-494 calls that same inner backend before building the borrowed guard. Thus the already identified pthread OnceBox/pal::Mutex is the only allocation-bearing mutex child on this direct source path; no additional heap wrapper is implied by the public type.

MutexGuard:277-280 holds a borrowed Mutex and poison::Guard. MutexGuard::new:719-720 maps the inline poison result; Drop:741-747 calls poison.done then the same inner.unlock. Public Mutex has no additional custom Drop implementation in this page; when an owned Mutex is dropped its fields reach the previously bound sys destructor. The actual ThreadInfo LOCK is static and is not dropped per guard/sample. Dropping its lock Result only releases the guard/lock; it does not free/reallocate the cached OnceBox child.

Public Once is std/sync/once.rs (sync/mod.rs:190-193). Once:35-37 contains only inline sys::Once; new():83-84 directly constructs it. OnceState:48-50 contains inline sys::OnceState. call_once_force:216-226 first checks inner.is_completed(), otherwise stores f in a stack Option and passes a borrowed adapter closure to inner.call(true,...). There is no Box or owned heap callback wrapper. The true argument is the already bound queue backend's ignore_poisoning flag. Callback allocations, including stdout buffer/lock construction, remain their existing source12 owners; the facade is not claimed to make the callback allocation-free.

## Immediate poison helper and reached failure edges

std/sync/poison.rs:85-107 defines Flag as an inline Atomic<bool> for panic=unwind, otherwise no flag payload. new initializes false. guard():118-123 creates an inline Guard holding only a bool (:157-160), then returns it directly or in PoisonError. done():128-136 only checks panic status and stores the flag (or does nothing in non-unwind mode). No heap child, lazy allocation or capacity is present in these helper values.

The status read is direct: std/thread/functions.rs:220-221 calls panicking::panicking(); the already supplied source12 std/panicking.rs:615-616 reads panic_count::count_is_zero(). Its normal implementation (:382-384,463-485) uses a static atomic fast path and a const-initialized TLS Cell<(usize,bool)> on the slow path. The immediate-abort variant returns true (:358-359). The TLS value has no owned heap child; the pinned core/alloc/global.rs:126-133 infrastructure exclusion applies. This read does not create a Thread handle or an exception payload. No further panic machinery is reached merely by testing the flag.

LockResult<T> is Result<T,PoisonError<T>> (poison.rs:235); PoisonError:197-201 owns T inline. In unwind mode new():271-272 stores that field. map_result:380-388 maps either Ok guard or Err's owned guard into the same borrowed MutexGuard; it does not Box or format an error. The actual ThreadInfo code (source12 thread_info.rs:117,127) stores LOCK.lock() in _lock_guard without unwrap/format. Even if poisoned, the Err carries the held MutexGuard until Result drop; dropping it reaches the same unlock. The poison path therefore adds no registered error-string or wrapper request.

In non-unwind mode Flag::get:146-147 is always false, so this lock path cannot reach PoisonError::new's abort-mode panic at:284-285. This is a source branch proof, not an assumption that all arbitrary public calls to PoisonError::new are allocation-free.

OnceLock's supplied page imports public Once (:7), stores it inline (:109-113), and uses call_once_force in initialize (:533-555). Source12 queue.rs:189-227 checks ignore_poisoning, takes the initializer state, calls the borrowed callback exactly once then returns. The public true argument skips the backend's existing-poison panic branch (:195-197). Its Option callback therefore has a value when take().unwrap() is executed. The ordinary completed fast path executes no callback. Existing callback owners and any callback/backend fatal failure remain the corresponding already scoped source12 branch; no new facade-owned panic/error allocation is introduced or silently assigned a general bound. No claim about arbitrary user callbacks or recursive/concurrent Once use is needed for the actual main-thread path.

Public Once has no custom heap-bearing destructor. Its sole sys::Once field is the already supplied queue state; the OnceState and adapter closure/Option are stack values. Wrapper sizeof values are not extra requested-heap terms.

## Exact consequence for source12

| Checked bridge | Additional registered retained owner | Additional transient request | Additional active-old request | Lazy child/failure consequence |
|---|---:|---:|---:|---|
| Public Mutex<()> / guard / poison |0|0|0| Only the existing sys::Mutex OnceBox/pal::Mutex child M_mutex remains; poisoned Result wraps the same inline guard. |
| Public Once / call_once_force |0|0|0| No callback Box or new lazy child; callback allocations remain existing source12 owners, and the force flag skips poison propagation. |

Thus source12's conditional identity formula remains

    C_library =1024+M_stdout+beta*(M_stack+L_info+4), beta in{0,1},
    conservative upper =1028+2*M_mutex+L_info.

M_stdout and M_stack are distinct allocation identities with the same still-unbound request-size parameter. The public wrappers do not supply the missing numeric M_mutex or private ThreadInfo leaf L_info. They add no new term and do not alter H-R2/H-F0 conclusions. H-I0/C0, checked implementation, final-A1 and source/target reconciliation remain conditional. This is an additive forwarding proof, not a reopened synchronization/runtime audit or full E_max/admission acceptance.

No source12 file or prior seal changed. No runtime/Rust/build/probe/model/test/measurement, new tool, allocator/observer/guard, installation/network, maintained/Git/index mutation or delegation. The metric_design_02 witness proposal was not read or executed as authority. Same RV30 backcheck remains pending.
